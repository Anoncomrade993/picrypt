
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