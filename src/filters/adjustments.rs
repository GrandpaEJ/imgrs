use image::{DynamicImage, ImageBuffer, Rgb, Rgba, Luma};
use crate::errors::ImgrsError;
use super::simd_ops::{fast_brightness, fast_contrast};

/// Apply brightness adjustment to an image
pub fn brightness(image: &DynamicImage, adjustment: i16) -> Result<DynamicImage, ImgrsError> {
    // Use optimized version for RGB/RGBA
    if matches!(image, DynamicImage::ImageRgb8(_) | DynamicImage::ImageRgba8(_)) {
        return fast_brightness(image, adjustment);
    }
    
    // Original implementation for other formats
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);

            for y in 0..height {
                for x in 0..width {
                    let pixel = rgb_img.get_pixel(x, y);
                    let r = (pixel[0] as i16 + adjustment).max(0).min(255) as u8;
                    let g = (pixel[1] as i16 + adjustment).max(0).min(255) as u8;
                    let b = (pixel[2] as i16 + adjustment).max(0).min(255) as u8;
                    result.put_pixel(x, y, Rgb([r, g, b]));
                }
            }

            Ok(DynamicImage::ImageRgb8(result))
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let (width, height) = rgba_img.dimensions();
            let mut result = ImageBuffer::new(width, height);

            for y in 0..height {
                for x in 0..width {
                    let pixel = rgba_img.get_pixel(x, y);
                    let r = (pixel[0] as i16 + adjustment).max(0).min(255) as u8;
                    let g = (pixel[1] as i16 + adjustment).max(0).min(255) as u8;
                    let b = (pixel[2] as i16 + adjustment).max(0).min(255) as u8;
                    let a = pixel[3]; // Keep alpha unchanged
                    result.put_pixel(x, y, Rgba([r, g, b, a]));
                }
            }

            Ok(DynamicImage::ImageRgba8(result))
        }
        DynamicImage::ImageLuma8(gray_img) => {
            let (width, height) = gray_img.dimensions();
            let mut result = ImageBuffer::new(width, height);

            for y in 0..height {
                for x in 0..width {
                    let pixel = gray_img.get_pixel(x, y);
                    let value = (pixel[0] as i16 + adjustment).max(0).min(255) as u8;
                    result.put_pixel(x, y, Luma([value]));
                }
            }

            Ok(DynamicImage::ImageLuma8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            brightness(&rgb_dynamic, adjustment)
        }
    }
}

/// Apply contrast adjustment to an image
pub fn contrast(image: &DynamicImage, factor: f32) -> Result<DynamicImage, ImgrsError> {
    // Use optimized version with lookup table for RGB/RGBA
    if matches!(image, DynamicImage::ImageRgb8(_)) {
        return fast_contrast(image, factor);
    }
    
    let factor = factor.max(0.0); // Ensure non-negative factor

    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);

            for y in 0..height {
                for x in 0..width {
                    let pixel = rgb_img.get_pixel(x, y);
                    let r = ((pixel[0] as f32 - 128.0) * factor + 128.0).max(0.0).min(255.0) as u8;
                    let g = ((pixel[1] as f32 - 128.0) * factor + 128.0).max(0.0).min(255.0) as u8;
                    let b = ((pixel[2] as f32 - 128.0) * factor + 128.0).max(0.0).min(255.0) as u8;
                    result.put_pixel(x, y, Rgb([r, g, b]));
                }
            }

            Ok(DynamicImage::ImageRgb8(result))
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let (width, height) = rgba_img.dimensions();
            let mut result = ImageBuffer::new(width, height);

            for y in 0..height {
                for x in 0..width {
                    let pixel = rgba_img.get_pixel(x, y);
                    let r = ((pixel[0] as f32 - 128.0) * factor + 128.0).max(0.0).min(255.0) as u8;
                    let g = ((pixel[1] as f32 - 128.0) * factor + 128.0).max(0.0).min(255.0) as u8;
                    let b = ((pixel[2] as f32 - 128.0) * factor + 128.0).max(0.0).min(255.0) as u8;
                    let a = pixel[3]; // Keep alpha unchanged
                    result.put_pixel(x, y, Rgba([r, g, b, a]));
                }
            }

            Ok(DynamicImage::ImageRgba8(result))
        }
        DynamicImage::ImageLuma8(gray_img) => {
            let (width, height) = gray_img.dimensions();
            let mut result = ImageBuffer::new(width, height);

            for y in 0..height {
                for x in 0..width {
                    let pixel = gray_img.get_pixel(x, y);
                    let value = ((pixel[0] as f32 - 128.0) * factor + 128.0).max(0.0).min(255.0) as u8;
                    result.put_pixel(x, y, Luma([value]));
                }
            }

            Ok(DynamicImage::ImageLuma8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            contrast(&rgb_dynamic, factor)
        }
    }
}

