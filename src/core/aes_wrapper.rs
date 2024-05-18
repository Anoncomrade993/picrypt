/*******************
* Name: Dess
* Email: anoncomrade993@gmail.com
* Copyright (c) 2024, Dess. All rights reserved.
* 
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
* 
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <https://www.gnu.org/licenses/>.
*******************/
use crate::core::aes;
use std::io;

pub fn encrypt(pixels: &mut Vec<Vec<u8>>, key: &str) -> Result<Vec<Vec<u8>>, io::Error> {
    // Key Expansion
    let mut round_keys = aes::key_expansion(key);

    // Determine the number of rounds based on the key length
    let rounds = aes::get_rounds(key.len());

    // Initial AddRoundKey step
    aes::add_round_key( pixels,&mut round_keys[0].to_vec());

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

    Ok(pixels.clone()) // Return the encrypted pixels
}

pub fn decrypt(pixels: &mut Vec<Vec<u8>>, key: &str) -> Result<Vec<Vec<u8>>, io::Error> {
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

    Ok(pixels.clone()) // Return the decrypted pixels
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