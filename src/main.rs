mod core;
use crate::core::aes_wrapper::{encrypt, decrypt};
use crate::core::image::Image;
#[allow(dead_code)]
#[allow(unused_imports)]
fn main() {
    let s16 = "src/assets/subject_16_by16.jpg";
    let _s32 = "src/assets/subject_32_by_32.jpg";
    let _s64 = "src/assets/subject64_by_64.jpg";

    let src: Option<String> = Some(String::from(s16));
    let save: Option<String> = Some(String::from("src/assets/test.png"));
    let dims: Option<(usize, usize)> = None;
    let pixel: Option<Vec<Vec<u8>>> = None;
    let key: &str = "ILoveRustSoMuch!";

    let image = Image::new(src, save, dims, pixel);
    let img = image.read_image().unwrap();

    let mut data: Vec<Vec<u8>> = img.pixels.clone().unwrap();
    let (width, height) = img.dims.clone().unwrap();

    let mut enc = encrypt(&mut data, key).unwrap();
    println!("{:?}", &enc[0..4]);
    let _= image.write_image(&mut enc, width, height);
    
}
