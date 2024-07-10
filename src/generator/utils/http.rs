use std::io::Cursor;
use image::{io::Reader as ImageReader, GenericImageView};
use docx_rs::Pic;
use log::error;

use crate::error::GeneratorError;

const PIXEL_PER_EMU: u32 = 9525;

pub async fn get_picture_by_url(url: &str) -> Result<Pic, GeneratorError> {
    let response = reqwest::get(url).await?;
    if response.status().is_success() {
        let bytes = response.bytes().await?;
        let buffer: Vec<u8> = bytes.to_vec();
        // Use the `image` crate to read the image dimensions
        let cursor = Cursor::new(buffer);
        let image_reader = ImageReader::new(cursor).with_guessed_format()?;
        let image = image_reader.decode();
        if image.is_err() {
            error!("Failed to decode image, error: {:?}", image.err().unwrap());
            return Err(GeneratorError::SystemError("Failed to decode image".to_string()));
        }
        let image = image.unwrap();
        
        // Get the dimensions
        let dimensions = image.dimensions();
        let width = dimensions.0;
        let height = dimensions.1;

        // Create a `Pic` object
        let pic = Pic::new(image.as_bytes()).size(width * PIXEL_PER_EMU, height * PIXEL_PER_EMU);
        Ok(pic)
    } else {
        return Err(GeneratorError::SystemError("Failed to get picture".to_string()));
    }
}

pub async fn get_file_by_url(url: String) -> Result<Vec<u8>, GeneratorError> {
    let response = reqwest::get(url).await?;
    if response.status().is_success() {
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    } else {
        return Err(GeneratorError::SystemError("Failed to get file".to_string()));
    }
}