mod core;
mod utils;
use std::io;
use wasm_bindgen::prelude::*;
use crate::core::aes;
use crate::utils;


/*******************
* Name: Dessxvii 
* Email: anoncomrade993@gmail.com
* 
* GNU GENERAL PUBLIC LICENSE
* Version 3, 29 June 2007
*
* Copyright (C) 2007 Free Software Foundation, Inc. <https://fsf.org/>
* Everyone is permitted to copy and distribute verbatim copies of this license document, but changing it is not allowed.
******************/

#[wasm_bindgen]
pub fn encrypt(mut buffers:&[u8], key: &str) -> Result<&[u8], io::Error> {
   
    let mut pixels = utils::d_2d(&mut buffers.to_vec()); //transpose pixels to 2D 

    let mut round_keys = aes::key_expansion(key);     // Key Expansion
    let rounds = aes::get_rounds(key.len());     // Determine the number of rounds based on the key length
 

    // Initial AddRoundKey step
    aes::add_round_key(pixels,&mut round_keys[0].to_vec());

    // Main AES rounds (SubBytes, ShiftRows, MixColumns, AddRoundKey)
    for round in 1..rounds {
        aes::sub_pixels( pixels);
        aes::shift_rows(pixels);
        aes::mix_columns( pixels);
        aes::add_round_key(pixels,&mut round_keys[round].to_vec());
    }

    // Final round without MixColumns
    aes::sub_pixels(pixels);
    aes::shift_rows(pixels);
    aes::add_round_key(pixels,&mut round_keys[rounds].to_vec());
   
    let result = utils::vec_to_slice(&mut pixels.clone());
    Ok(result) // Return the encrypted pixels
}

#[wasm_bindgen]
pub fn decrypt(mut buffers:&[u8], key: &str) -> Result<&[u8], io::Error> {
    
    let mut pixels = utils::d_2d(&mut buffers.to_vec()); //transpose pixels to 2D 
    // Key Expansion
    let round_keys = aes::key_expansion(key);

    // Determine the number of rounds based on the key length
    let rounds = aes::get_rounds(key.len());

    // Initial AddRoundKey step
    aes::add_round_key(pixels,&mut round_keys[rounds].to_vec());

    // Main AES rounds (InvShiftRows, InvSubBytes, AddRoundKey, InvMixColumns)
    for round in (1..rounds).rev() {
        aes::inv_shift_rows(pixels);
        aes::inv_sub_pixels(pixels);
        aes::add_round_key(pixels,&mut round_keys[round].to_vec());
        aes::inv_mix_columns(pixels);
    }

    // Final round without InvMixColumns
    aes::inv_shift_rows(pixels);
    aes::inv_sub_pixels(pixels);
    aes::add_round_key(pixels,&mut round_keys[0].to_vec());

    let result = utils::vec_to_slice(&mut pixels.clone());
    Ok(result) // Return the decrypted pixels
}

/*
pub fn test(pixels:&mut Vec<Vec<u8>>){
    println!("init {:?}",pixels);
    
    //AddRoundKey
    //subBytes
    //shift_rows
    //MixColumn
    let fess = pixels.clone();
    let mut key =b"";
    aes::add_round_key(pixels,&mut key);
    println!("add_round_key {:?}",pixels);
    aes::sub_pixels(pixels);
    println!("sub_pixelss {:?}",pixels);
    aes::shift_rows(pixels);
    println!("shift_rows {:?}",pixels);
    aes::mix_columns(pixels);
    println!("mix_columns {:?}",pixels);
    
    //inverse MixColumn
    //inverse shift_rows
    //inverse subBytes
    //AddRoundKey 
    aes::inv_mix_columns(pixels);
    println!("inv_mix_columns {:?}",pixels);
    aes::inv_shift_rows(pixels);
    println!("inv_shift_rows {:?}",pixels);
    aes::inv_sub_pixels(pixels);
    println!("inv_sub_pixels {:?}",pixels);
    aes::add_round_key(pixels,&mut key);
    println!("add_round_key {:?}",pixels);
    let lass = pixels.clone();
    assert_eq!(fess,lass);
}*/