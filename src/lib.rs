/**
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
pub fn decrypt_pixels(pixels:&mut Vec<Vec<u8>>,key:&str) -> &[u8]{
  let res = decrypt(&mut pixels,key).unwrap();
  res.to_slice()
}

#[wasm_bindgen]
///wrapper function for the encryption function 
pub fn encrypt_pixels(pixels:&mut Vec<u8>) -> Vec<u8>{
   let mut 1d = D_2D(&pixels).unwrap();
   let res = encrypt(&mut 1d,key).unwrap();
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
}***/
mod core;
use wasm_bindgen::prelude::*;
use crate::core::aes_wrapper::{decrypt, encrypt};
use pixelate::lsb::{encode, decode};
use std::error::Error;

/// Convert a 1D Vec<u8> into a 2D Vec<Vec<u8>>
fn d_2d(pixels: &Vec<u8>, rows: usize, cols: usize) -> Vec<Vec<u8>> {
    let mut bunk = Vec::with_capacity(rows);

    for i in 0..rows {
        let start = i * cols;
        let end = start + cols;
        bunk.push(pixels[start..end].to_vec());
    }

    bunk
}

#[wasm_bindgen]
/// Wrapper function for the decryption function 
pub fn decrypt_pixels(pixels: &mut [Vec<u8>], key: &str) -> Vec<u8> {
    match decrypt(pixels, key) {
        Ok(res) => res,
        Err(_) => vec![],  // Return an empty vector in case of an error
    }
}

#[wasm_bindgen]
/// Wrapper function for the encryption function 
pub fn encrypt_pixels(pixels: &mut [u8], key: &str) -> Vec<u8> {
    let rows = pixels.len() / 4;
    let mut two_d = d_2d(pixels, rows, 4);
    match encrypt(&mut two_d, key) {
        Ok(res) => res,
        Err(_) => vec![],  // Return an empty vector in case of an error
    }
}

#[wasm_bindgen]
/// Wrapper function for the encode function 
pub fn encode_pixel(pixels: &mut [u8], data: &str, channel: u8) -> Vec<u8> {
    match encode(pixels, data, channel) {
        Ok(res) => res,
        Err(_) => vec![],  // Return an empty vector in case of an error
    }
}

#[wasm_bindgen]
/// Wrapper function for the decode function 
pub fn decode_pixel(pixels: &mut [u8], channel: u8) -> String {
    match decode(pixels, channel) {
        Ok(res) => res,
        Err(_) => String::new(),  // Return an empty string in case of an error
    }
}