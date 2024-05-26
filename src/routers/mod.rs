mod core;
mod utils;
mod models;

use crate::models::Image;
use crate::core::aes_wrapper::{encrypt, decrypt};
use pixelate::lsb::{encode, decode};
use utils::{splitter, cleanser, joiner, d_2d, gen_watermark, connect_to_db};
use thiserror::Error;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
use rocket::response::status;

// Define the data structure for JSON input/output
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Data {
    pixels: Vec<u8>,
    dimension: [u32; 2],
    key: Option<String>,
}

// Define custom error types
#[derive(Error, Debug)]
enum RunError {
    #[error("Missing field error: {0}")]
    MissingFieldError(String),
    #[error("Invalid data type")]
    InvalidDataType,
    #[error("Encryption error")]
    EncryptionError,
    #[error("Server error")]
    ServerError,
}

// Endpoint to encrypt pixels
#[post("/encrypt_pixels", format = "json", data = "<data>")]
pub async fn encrypt_pixels_post(data: Json<Data>) -> Result<Json<Data>, status::Custom<Json<RunError>>> {
    let data = data.into_inner();
    let mut pixels = d_2d(data.pixels.clone());
    let dims = data.dimension;
    let key = data.key.clone();

    if pixels.is_empty() {
        return Err(status::Custom(Status::BadRequest, Json(RunError::MissingFieldError("pixels".to_string()))));
    }

    if let Some(ref key) = key {
        if !(key.len() == 16 || key.len() == 24 || key.len() == 32) {
            return Err(status::Custom(Status::BadRequest, Json(RunError::MissingFieldError("Key length is not acceptable".to_string()))));
        }
    } else {
        return Err(status::Custom(Status::BadRequest, Json(RunError::MissingFieldError("Key is missing".to_string()))));
    }

    let enc_proc = match encrypt(&mut pixels, key.unwrap().as_bytes()) {
        Ok(encrypted_data) => encrypted_data,
        Err(_) => return Err(status::Custom(Status::InternalServerError, Json(RunError::EncryptionError))),
    };

    let ranwaterMark: String = gen_watermark();
    let watermarked_pixels = match encode(&enc_proc, &ranwaterMark, 3u8) {
        Ok(res) => res,
        Err(_) => return Err(status::Custom(Status::InternalServerError, Json(RunError::ServerError))),
    };

    let new_image = Image::new(&ranwaterMark, &data.key, (dims[0], dims[1]));

    if let Err(_) = new_image.insert().await {
        return Err(status::Custom(Status::InternalServerError, Json(RunError::ServerError)));
    }

    let res = Data {
        pixels: watermarked_pixels,
        dimension: dims,
        key: None,
    };

    Ok(Json(res))
}

// Endpoint to decrypt pixels
#[post("/decrypt_pixels", format = "json", data = "<data>")]
pub async fn decrypt_pixels_post(data: Json<Data>) -> Result<Json<Data>, status::Custom<Json<RunError>>> {
    let data = data.into_inner();
    let pixels = data.pixels;
    let key = data.key.clone();
    let dims = data.dimension;

    let watermark = match decode(&pixels, 3u8) {
        Ok(decoded_data) => decoded_data,
        Err(_) => return Err(status::Custom(Status::InternalServerError, Json(RunError::ServerError))),
    };

    let find_watermark = Image::find_watermark(&watermark, key.as_deref(), None).await;

    if find_watermark {
        let mut pixels_clone = pixels.clone();
        match decrypt(&mut pixels_clone, key.unwrap().as_bytes()) {
            Ok(dec) => {
                let res = Data {
                    pixels: dec,
                    dimension: dims,
                    key: None,
                };
                Ok(Json(res))
            }
            Err(_) => Err(status::Custom(Status::InternalServerError, Json(RunError::ServerError))),
        }
    } else {
        Err(status::Custom(Status::InternalServerError, Json(RunError::ServerError)))
    }
}

// Endpoint to encode pixels with a message
#[post("/encode_pixels", format = "json", data = "<data>")]
pub async fn encode_pixels_post(data: Json<Data>) -> Result<Json<Data>, status::Custom<Json<RunError>>> {
    let data = data.into_inner();
    let pixels = data.pixels;
    let dims = data.dimension;
    let message = data.key.clone().unwrap_or_default();

    if pixels.is_empty() {
        return Err(status::Custom(Status::BadRequest, Json(RunError::MissingFieldError("pixels".to_string()))));
    }

    if message.is_empty() {
        return Err(status::Custom(Status::BadRequest, Json(RunError::MissingFieldError("Message length is not acceptable".to_string()))));
    }

    let watermark = gen_watermark();
    let new_image = Image::new(&watermark, None, (dims[0], dims[1]));

    if let Err(_) = new_image.insert().await {
        return Err(status::Custom(Status::InternalServerError, Json(RunError::ServerError)));
    }

    let joined = joiner(&watermark, &message);
    match encode(&pixels, &joined, 2u8) {
        Ok(enc_pixels) => {
            let res = Data {
                pixels: enc_pixels,
                dimension: dims,
                key: None,
            };
            Ok(Json(res))
        }
        Err(_) => Err(status::Custom(Status::InternalServerError, Json(RunError::ServerError))),
    }
}

// Endpoint to decode pixels and retrieve the hidden message
#[post("/decode_pixels", format = "json", data = "<data>")]
pub async fn decode_pixels_post(data: Json<Data>) -> Result<Json<Data>, status::Custom<Json<RunError>>> {
    let data = data.into_inner();
    let pixels = data.pixels;
    let dims = data.dimension;

    match decode(&pixels, 2u8) {
        Ok(dec_pixels) => {
            let (watermark, message) = splitter(&dec_pixels.to_string());
            let find_watermark = Image::find_watermark(&watermark, None, None).await;

            let lsb_cleanser = cleanser(&pixels);

            if find_watermark {
                let res = Data {
                    pixels: lsb_cleanser,
                    dimension: dims,
                    key: Some(message),
                };
                Ok(Json(res))
            } else {
                Err(status::Custom(Status::InternalServerError, Json(RunError::ServerError)))
            }
        }
        Err(_) => Err(status::Custom(Status::InternalServerError, Json(RunError::ServerError))),
    }
}