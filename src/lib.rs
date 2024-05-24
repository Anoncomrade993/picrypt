mod core;
use wasm_bindgen::prelude::*;
use crate::core::aes_wrapper::{decrypt, encrypt};
use pixelate::lsb::{encode,decode};
use std::error;


///convert UINT8CLAMPEDARRAY to 2D vector 
fn D_2D(pixels:&mut Vec<u8>, rows: usize, cols: usize) -> Vec<Vec<u8>> {
    let cols:usize = 4;
    let rows:usize = pixels.len()/cols;
    
    let mut bunk = Vec::with_capacity(rows);

    for i in 0..rows {
        let start = i * cols;
        let end = start + cols;
        bunk.push(pixels[start..end].to_vec());
    }

    bunk
}

#[wasm_bindgen]
///wrapper function for the decryption function 
pub fn decrypt_pixels(pixels:&mut Vec<Vec<u8>>,key:&str) -> Vec<u8>{
  let res = decrypt(pixels,key).unwrap();
}

#[wasm_bindgen]
///wrapper function for the encryption function 
pub fn encrypt_pixels(pixels:&mut Vec<Vec<u8>>) -> Vec<u8>{
   let res = encrypt(pixels,key).unwrap();
   res
}

#[wasm_bindgen]
///wrapper function for the encode function 
pub fn encode_pixel(pixels:&mut Vec<u8>,data:&str,channel:u8) -> Vec<u8>{
  let res = encode(pixels,data,channel).unwrap();
  res
}

#[wasm_bindgen]
///wrapper function for the decode function 
pub fn decode_pixel(pixels:&mut Vec<u8>,channel:u8) -> String{
   let res =  decode(pixels,channel).unwrap();
    res
}