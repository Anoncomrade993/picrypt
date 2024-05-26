
mod core;
mod utils;
mod models;

use crate::models::Image;
use crate::core::aes_wrapper::{encrypt, decrypt};
use pixelate::lsb::{encode, decode};
use utils::{d_2d,gen_watermark,connect_to_db};
use thiserror::Error;


use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::response::Redirect;
use rocket::http::Status;
use rocket::response::status;



let connection = Image::connect_to_db().await;



#[derive(Clone,Debug,Serialize, Deserialize)]
struct Data {
    pixels: Vec<u8>,
    dimension: (u32, u32),
    key: String,
}

#[derive(Error, Debug)]
enum RunError {
    #[error("Missing field error: {0}")]
    MissingFieldError(String),
    #[error("Invalid data type")]
    InvalidDataType,
    #[error("Encryption error")]
    EncryptionError,
    #[error("Server Error")]
    ServerError
}





#[post("/encrypt_pixels", format = "json", data = "<data>")]
pub fn encrypt_pixels_post(data: Json<Data>) -> Json<Result<Data, RunError>> {
    let data = data.into_inner();
    let mut pixels = d_2d(data.pixels.clone());
    let dims = data.dimension;
    let key:String = String::from(data.key.clone);

    if pixels.is_empty() {
        return Json(Err(RunError::MissingFieldError("pixels".to_string()))).status(Status::BadRequest);
    }

    if !(key.len() == 16 || key.len() == 24 || key.len() == 32) {
        return Json(Err(RunError::MissingFieldError("Key length is not acceptable".to_string()))).status(Status::BadRequest);
    }

    let enc_proc = match encrypt(&mut pixels, &key) {
        Ok(encrypted_data) => encrypted_data,
        Err(_) => return Json(Err(RunError::EncryptionError)).status(Status::InternalServerError),
    };
    
    let ranwaterMark: String = gen_watermark();
    let watermarked_pixels = encode(&enc_proc, ranwaterMark.clone(), 3usize);
    
    let new_image = Image::new(ranwaterMark, key.clone(), (dims.0, dims.1));
    if let Err(_) = new_image.insert(&collection).await {
        return Json(Err(RunError::ServerError)).status(Status::InternalServerError);
    }

    let res = Data {
        pixels: watermarked_pixels,
        dimension: dims,
        key: String::new(),
    };
    
    Json(Ok(res)).status(Status::Ok)
}

#[post("/decrypt_pixels",format="json",data="<data>")]
pub fn decrypt_pixels_post(data:Json<Data>) -> Json<Result<Data,RunError>> {
    let data = data.into_inner();
    let pixels = data.pixels;
    let key = data.key;
    let dims = data.dimension;
    
    let watermark = match decode(&pixels, 3u8) {
        Ok(decoded_data) => decoded_data,
        Err(_) => return Json(Err(RunError::ServerError("Server error occurred"))).status(Status::InternalServerError),
    };
    
    let find_watermark = Image::find_watermark(watermark.as_str(), key.as_str(), collection.clone()).await;
    
    if find_watermark {
        match decrypt(&mut pixels, key.clone()) {
            Ok(dec) => {
                Json(Ok(Data{
                    pixels: dec,
                    dimension: dims,
                    key: String::new()
                })).status(Status::Ok)
            },
            Err(_) => {
                Json(Err(RunError::ServerError)).status(Status::InternalServerError)
            }
        }
    } else {
        Json(Err(RunError::ServerError(""))).status(Status::InternalServerError)
    }
}