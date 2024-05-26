use rand::Rng;
use rand::distributions::Alphanumeric;

pub fn generate_watermark() -> String {
    let mut rng = rand::thread_rng();
    
    let watermark: String = (0..6)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    
    watermark
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
