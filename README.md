#  Picrypt

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Modules](#Modules)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)


## Installation 

- Method 1
```Bash
$ cargo install picrypt
```
- Method 2

```Toml
[dependencies]
 picrypt = {git="https:://github.com/anonComrade993/picrypt.giy"}
```
## Usage

picrypt has two modules with both having two public functions
 
```Rust
  use picrypt::{
        ImageIO::{read_image,write_image},
        Aes::{encrypt,decrypt}
  };
  ```
  
## Modules
  
  ```Rust
  //AES Module 
  pub fn encrypt (plain_pixels:&mut Vec<Vec<u8>>,key:&str) -> Vec<Vec<u8>>
  
  pub fn decrypt (enc_pixels:&mut Vec<Vec<u8>>,key:&str) -> Vec<Vec<u8>>
  
  ///ImageIO Module
  
  pub fn read_image(&self) -> Image 
  
  pub fn write_image(pixels:&mut Vec<Vec<u8>>, width:usize, height:usize) -> Result<(),image::ImageError>
  
  
  // Image Struct 
  
  Image {
    src : Option<String>, //source image path
    pub  save : Option<String>, //destination to be saved
    pub  dims: Option<(usize,usize)>, //from image read, dimensions (width, height )
    pub  pixels:Option<Vec<Vec<u8>>> //the pixels read from an image ,RGBA format 
    
  }  
  ```


## Examples

Start by creating a binary rust app 

```$ cargo new image_encryption --bin```

create a ```main.rs ``` the inside the ```src``` folder
create another folder to hold your images ```assets```

assumming you have an image  ```assets/cat.png``` 

in the ```main.rs```

```rust 
 use picrypt::Aes;
 use picrypt::ImageIO;
 
 
 fn main(){
 
 //key for encryption
 //length 16(AES128),24 (AES192) ,32 (AES256)
 
 const key:&str = "IWritePrototypesInJsToo!"; //24 
 
 ///Never try using an image with a wrong extension
 //every image format has a different architecture,this will cause errors
 
   let src:Option<String>= Some("src/assets/cat.png".to_string());
   
   let save:Option<String> = Some("src/assets/test.png".to_string());
   
   let dims:Option<(usize,usize)>= None;
   
   let pixels:Option<Vec<Vec<u8>>>= None;
   
      //create new image object 
   let img = imageIO::new(src,save,dims,pixels);
   
   let nimg = img.read_image();
   
   let pixels = nimg.pixels.clone().unwrap();
   let (width, height ) = nimg.dims.clone().unwrap();
   
   //encryption
    
    let enc_pixels = Aes::encrypt(&mut pixels,key)

   
 }


```
## Contributing

Provide guidelines for others who want to contribute to your project. Include information on how to report bugs, submit feature requests, and contribute code.

## License

Include information about the license under which your library is distributed. Specify any usage restrictions or permissions.