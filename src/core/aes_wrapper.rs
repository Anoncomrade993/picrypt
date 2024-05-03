use aes::*;

pub fn encrypt(pixels: &mut Vec<Vec<u8>>, key: &str) -> Result<Vec<Vec<u8>>, io::Error> {
    // Key Expansion
    let round_keys = key_expansion(key);

    // Determine the number of rounds based on the key length
    let rounds = get_rounds(key.len());

    // Initial AddRoundKey step
    add_round_key(pixels, round_keys[0]);

    // Main AES rounds (SubBytes, ShiftRows, MixColumns, AddRoundKey)
    for round in 1..rounds {
        sub_pixels(pixels);
        shift_rows(pixels);
        mix_column(pixels);
        add_round_key(pixels, round_keys[round]);
    }

    // Final round without MixColumns
    sub_pixels(pixels);
    shift_rows(pixels);
    add_round_key(pixels, round_keys[rounds]);

    Ok(pixels.clone()) // Return the encrypted pixels
}

pub fn decrypt(pixels: &mut Vec<Vec<u8>>, key: &str) -> Result<Vec<Vec<u8>>, io::Error> {
    // Key Expansion
    let round_keys = key_expansion(key);

    // Determine the number of rounds based on the key length
    let rounds = get_rounds(key.len());

    // Initial AddRoundKey step
    add_round_key(pixels, round_keys[rounds]);

    // Main AES rounds (InvShiftRows, InvSubBytes, AddRoundKey, InvMixColumns)
    for round in (1..rounds).rev() {
        inv_shift_rows(pixels);
        inv_sub_pixels(pixels);
        add_round_key(pixels, round_keys[round]);
        inv_mix_column(pixels);
    }

    // Final round without InvMixColumns
    inv_shift_rows(pixels);
    inv_sub_pixels(pixels);
    add_round_key(pixels, round_keys[0]);

    Ok(pixels.clone()) // Return the decrypted pixels
}