mod core;
mod utils;
mod models;

use serde_json::Value;
use serde::{Deserialize,Serialize};

use crate::aes_wrapper::{encrypt,decrypt};
use pixelate::lsb::{encode,decode};
use crate::utils::{ d_2d,
                  cleanser,
                  splitter,
                  generate_watermark,
                  joiner,
                  hasher
  
          };



#[derive(Cloneable,Serialize,Deserialize, Debug)]
struct Body<'r>{
   pixels:Option<Vec<u8>>, 
   data:Option<'r &str>,// key or message 
   dimension:Option<[u8;2]>
}


/*****************
- extract request body
{pixels:Vec<u8>} {data:Option<String>} {dimensions:[u8;2]}
- encrypt pixels with key:data 
- watermark modified pixels 
- save generated watermark to database 
- respond with payload { pixels:Vec<u8>, data:String::new(), dimensions:Option<[u8;2]>
*****************/


pub fn encryption(Json(req):Json<Body>) -> Result<Json<Body>>{
  
  let mut pixels:Option<Vec<u8>> = Some(req.pixels.clone());
  let key:Option<'r &str> = Some(req.data);
  let dimensions:Option<[u8;2]> = Some(req.dimensions);
  let mut 2d_pixels = d_2d(mut pixels.unwrap());
  
  let mut encrypted_pixels = match encrypt(&mut 2d_pixels,&data.unwrap()){
       Ok(pixels) => pixels,
       Err(err) => return format!("{err}")
  };
  
  const CHANNEL:u8 = 3u8;
  let watermark_tag:&str = generate_watermark(); //random generated watermark for images
  let hashed_key = hasher(&key);
  
  let pixel_encoded = match encode(&mut encrypted_pixels,&watermark_tag, CHANNEL){
    Ok(pixels) => pixels,
    Err(err) => return format!("{err}")
  }
   
     //save  to db ,if saved then move to inject into pixels
  /*
  doc!{
    Image{
      watermark:Some(watermark_tag),
      key:Some(hashed_key)
    }
  }
    */
     json!(Body{
         pixels:Some(pixel_encoded),
         data:Some(String::from("")),
         dimensions:Some(dimensions)
       })
}


