use image::{DynamicImage, Rgb, Rgba, Luma, LumaA, GenericImageView};
use crate::errors::ImgrsError;

/// Get pixel value at specified coordinates
pub fn get_pixel(image: &DynamicImage, x: u32, y: u32) -> Result<(u8, u8, u8, u8), ImgrsError> {
    let (width, height) = image.dimensions();
    
    if x >= width || y >= height {
        return Err(ImgrsError::InvalidOperation(
            format!("Pixel coordinates ({}, {}) out of bounds for image size {}x{}", x, y, width, height)
        ));
    }
    
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let pixel = rgb_img.get_pixel(x, y);
            Ok((pixel[0], pixel[1], pixel[2], 255))
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let pixel = rgba_img.get_pixel(x, y);
            Ok((pixel[0], pixel[1], pixel[2], pixel[3]))
        }
        DynamicImage::ImageLuma8(gray_img) => {
            let pixel = gray_img.get_pixel(x, y);
            Ok((pixel[0], pixel[0], pixel[0], 255))
        }
        DynamicImage::ImageLumaA8(gray_alpha_img) => {
            let pixel = gray_alpha_img.get_pixel(x, y);
            Ok((pixel[0], pixel[0], pixel[0], pixel[1]))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let pixel = rgb_img.get_pixel(x, y);
            Ok((pixel[0], pixel[1], pixel[2], 255))
        }
    }
}

/// Set pixel value at specified coordinates
pub fn put_pixel(image: &DynamicImage, x: u32, y: u32, color: (u8, u8, u8, u8)) -> Result<DynamicImage, ImgrsError> {
    let (width, height) = image.dimensions();
    
    if x >= width || y >= height {
        return Err(ImgrsError::InvalidOperation(
            format!("Pixel coordinates ({}, {}) out of bounds for image size {}x{}", x, y, width, height)
        ));
    }
    
    let mut result = image.clone();
    
    match &mut result {
        DynamicImage::ImageRgb8(rgb_img) => {
            rgb_img.put_pixel(x, y, Rgb([color.0, color.1, color.2]));
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            rgba_img.put_pixel(x, y, Rgba([color.0, color.1, color.2, color.3]));
        }
        DynamicImage::ImageLuma8(gray_img) => {
            // Convert RGB to grayscale using luminance formula
            let gray = (color.0 as f32 * 0.2126 + color.1 as f32 * 0.7152 + color.2 as f32 * 0.0722) as u8;
            gray_img.put_pixel(x, y, Luma([gray]));
        }
        DynamicImage::ImageLumaA8(gray_alpha_img) => {
            let gray = (color.0 as f32 * 0.2126 + color.1 as f32 * 0.7152 + color.2 as f32 * 0.0722) as u8;
            gray_alpha_img.put_pixel(x, y, LumaA([gray, color.3]));
        }
        _ => {
            return Err(ImgrsError::InvalidOperation(
                "Unsupported image format for pixel manipulation".to_string()
            ));
        }
    }
    
    Ok(result)
}

/// Apply a function to each pixel in the image
#[allow(dead_code)]
pub fn map_pixels<F>(image: &DynamicImage, mut func: F) -> Result<DynamicImage, ImgrsError>
where
    F: FnMut(u32, u32, (u8, u8, u8, u8)) -> (u8, u8, u8, u8),
{
    let (width, height) = image.dimensions();
    let mut result = image.clone();
    
    for y in 0..height {
        for x in 0..width {
            let current_pixel = get_pixel(image, x, y)?;
            let new_pixel = func(x, y, current_pixel);
            result = put_pixel(&result, x, y, new_pixel)?;
        }
    }
    
    Ok(result)
}

