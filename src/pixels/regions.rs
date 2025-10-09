use image::{DynamicImage, GenericImageView};
use crate::errors::ImgrsError;
use super::access::{get_pixel, put_pixel};

/// Type alias for pixel color (RGBA)
pub type Color = (u8, u8, u8, u8);
/// Type alias for a 2D region of pixels
pub type PixelRegion = Vec<Vec<Color>>;

/// Get a region of pixels as a 2D array
#[allow(dead_code)]
pub fn get_region(
    image: &DynamicImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<PixelRegion, ImgrsError> {
    let (img_width, img_height) = image.dimensions();
    
    if x + width > img_width || y + height > img_height {
        return Err(ImgrsError::InvalidOperation(
            format!("Region ({}, {}, {}, {}) out of bounds for image size {}x{}", 
                   x, y, width, height, img_width, img_height)
        ));
    }
    
    let mut region = Vec::with_capacity(height as usize);
    
    for row in 0..height {
        let mut row_pixels = Vec::with_capacity(width as usize);
        for col in 0..width {
            let pixel = get_pixel(image, x + col, y + row)?;
            row_pixels.push(pixel);
        }
        region.push(row_pixels);
    }
    
    Ok(region)
}

/// Set a region of pixels from a 2D array
#[allow(dead_code)]
pub fn put_region(
    image: &DynamicImage,
    x: u32,
    y: u32,
    pixels: &[Vec<Color>],
) -> Result<DynamicImage, ImgrsError> {
    let (img_width, img_height) = image.dimensions();
    let height = pixels.len() as u32;
    
    if height == 0 {
        return Ok(image.clone());
    }
    
    let width = pixels[0].len() as u32;
    
    if x + width > img_width || y + height > img_height {
        return Err(ImgrsError::InvalidOperation(
            format!("Region ({}, {}, {}, {}) out of bounds for image size {}x{}", 
                   x, y, width, height, img_width, img_height)
        ));
    }
    
    let mut result = image.clone();
    
    for (row, pixel_row) in pixels.iter().enumerate() {
        for (col, &pixel) in pixel_row.iter().enumerate() {
            result = put_pixel(&result, x + col as u32, y + row as u32, pixel)?;
        }
    }
    
    Ok(result)
}

