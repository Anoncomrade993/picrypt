
///convert 1d to 2d 
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

///LSB cleanser 
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
pub fn flatten(v:Vec<Vec<u8>>)-> Vec<u8>{
   v.concat()
}
