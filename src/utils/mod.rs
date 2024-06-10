use rand::Rng;
use rand::distributions::Alphanumeric;

pub fn generate_watermark() -> &str {
    let mut rng = rand::thread_rng();
    
    let watermark: String = (0..6)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    
    watermark.as_bytes()
}


pub fn d_2d(pixels: Vec<u8>) -> Vec<Vec<u8>> {
    let rows: usize = 4;
    let cols = pixels.len() / rows;
    let mut result = Vec::with_capacity(rows);
  
    for i in 0..rows {
        let start = i * cols;
        let end = start + cols;
        result.push(pixels[start..end].to_vec());
    }

    result
}

pub fn joiner(mut data: String, watermark: String) -> String {
    data.push_str(&watermark); // append another String
    data
}

pub fn splitter(data: String) -> Result<(&str, &str), String> {
    let len = data.len();
    if len <= 6 {
        Err(data) // Return the data as an error if it's too short
    } else {
        Ok((&data[len-6..], &data[..len-6])) // Return the split data as a tuple of slices
    }
}

pub fn cleanser(data: &mut Vec<u8>, channel:u8) -> Vec<u8> {
    let len = data.len();
    
    for q in (0..len).step_by(4) {
        if let Some(pixel) = data.get_mut(q + channel as usize) { // Assume channel is q % 4
            *pixel &= 0xFE; // Clear the least significant bit
        } else {
            break; // Exit loop if pixel is None
        }
    }
    data.to_vec() // Return a clone of the modified data
}


pub fn hasher<'r>(key:'r &str)-> Result<'r &str>{
  
}