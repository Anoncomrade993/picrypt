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
use image::{GenericImageView,RgbaImage};
use std::error;

#[derive(Debug)]
pub struct Image{
        src: Option<String>,
        pub save: Option<String>,
        pub dims: Option<(usize, usize)>,
        pub  pixels: Option<Vec<Vec<u8>>>,
       
}

impl Image {
    pub fn new(
        src: Option<String>,
        save: Option<String>,
        dims: Option<(usize, usize)>,
        pixels: Option<Vec<Vec<u8>>>,
    ) -> Self {
      Self {src,save,dims, pixels}
    }
fn flatten_vector(&self, buffer: Vec<Vec<u8>>) -> Result<Vec<u8>, Box<dyn error::Error>> {
        let flat_buffer: Vec<u8> = buffer.into_iter().flatten().collect();
        let mut bunk: Vec<u8> = Vec::new();
        for buff in flat_buffer.into_iter() {
            bunk.push(buff);
        }
        
        Ok(bunk)
    }

    pub fn read_image(&self) -> Result<Self, image::ImageError> {
        let image = image::open(self.src.clone().unwrap_or_default())?;
        let (width, height) = image.dimensions();
        println!("dims {} {}",width, height);
        
        let pixels: Vec<Vec<u8>> = image
            .pixels()
            .map(|(_, _, pixel)| vec![pixel[0], pixel[1], pixel[2], pixel[3]])
            .collect();
         println!("pixels {:?}",&pixels[0..10]);
        let _self = Self {
            src: Some(self.src.clone().unwrap_or_default()),
            save: Some(self.save.clone().unwrap_or_default()),
            dims: Some((width as usize, height as usize)),
            pixels: Some(pixels),
        };

        Ok(_self)
    }

    pub fn write_image(
        &self,
        pixels: &mut Vec<Vec<u8>>,
        width:usize, height:usize
    ) -> Result<(), image::ImageError> {
       
        let pixels = pixels.to_vec();
       // let (width, height) = self.dims.expect("dims not grabbed");
       
        let  flattened = self.flatten_vector(pixels).unwrap();
        
        RgbaImage::from_raw(width as u32, height as u32, flattened.clone())
            .ok_or(image::ImageError::Parameter(
                image::error::ParameterError::from_kind(
                    image::error::ParameterErrorKind::DimensionMismatch,
                ),
            ))?
            .save(&self.save.clone().unwrap_or_default())
            .expect("Error occurred writing image");

        Ok(())
    }
}
  