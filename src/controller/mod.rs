mod core;
mod utils;
mod models;

use axum::extract::Json;
use serde::{Deserialize,Serialize};
use crate::aes_wrapper::{encrypt,decrypt};
use pixelate::lsb::{encode,decode};
use crate::utils::{ 
d_2d,cleanser,splitter,generate_watermark,
joiner, hasher,flatten};


#[derive(Debug,Deserialize,Serialize)]
struct Data{
  pixels:Vec<u8>,
  secret:String,
  dimensions:[u32;2]
}

pub fn encryption_pixels(Json(body):Json<Data>)->Result<Json<Data>,io::Error>{
  let mut pixels = body.pixels;
  let secret = body.secret;
  let dims = body.dimensions;
  
  let mut 2d_vector = d_2d(pixels);
  
  match encrypt(&mut 2d_vector,&secret){
    Ok(pixels) => {
    
      let flattened = flatten(pixels);
      let watermark = generate_watermark();
      let encoded = match encode(&mut flattened,&secret,2u8){Ok(enc) => enc,Err(_) => return format!("Error encoding pixels")}
       
       if !(encoded && encoded.len() == 0){
          return Error("")
       }
       
       let data = Data { pixels:encoded, secret,dimensions};
       Ok(Json(data))
    }
  }
}



pub fn decryption_pixels(Json(body):Json<Data>)->Result<Json<Data>,io::Error>{
  let mut pixels = body.pixels;
  let secret = body.secret;
  let dims = body.dimensions;
  
  match decode(&mut pixels,2u8){
    Ok(wm) => {
      
      let watermark = wm.clone();
      //search for it in DB
      
      let decoded = match decrypt(&mut flattened,&watermark,2u8){Ok(enc) => enc,Err(_) => return format!("Error encoding pixels")}
       
       if !(encoded && encoded.len() == 0){
          return Error("")
       }
       
       let data = Data { pixels:encoded, secret,dimensions};
       Ok(Json(data))
    }
  }
}




/*****************
- extract request body
{pixels:Vec<u8>} {data:Option<String>} {dimensions:[u8;2]}
- encrypt pixels with key:data 
- watermark modified pixels 
- save generated watermark to database 
- respond with payload { pixels:Vec<u8>, data:String::new(), dimensions:Option<[u8;2]>
*****************/


