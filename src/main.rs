use crate::core::aes_wrapper::*;
use crate::core::image::Image;




fn main(){
  let s16 = "src/assets/subject16_by_16.jpg";
  let s32 = "src/assets/subject32_by_32.jpg";
  let s64 = "src/assets/subject64_by_64.jpg";
  
  let src:Option<String> = Some(String::from(s16));
  let save:Option<String> = Some(String::from(""));
  let dims:Option<(usize,usize)> = None;
  let pixel:Option<Vec<Vec<u8>>> = None;
  let keys:&str = "ILoveRustSoMuch!";
  
  let img = Image::new(src,save,dims,pixel);
  let (dims,pixels) = img.read_image().unwrap();
  
  let data:Vec<Vec<u8>> = pixels.clone().unwrap();
  let (width, height ) = dims.clone().unwrap();
  
  let enc = encrypt(&mut data,key);
  println!("{:?}",&enc[0..4]);
  let nimage = write_image(&mut enc,width, height);
  println!("{}",nimage);
  
  
  
}