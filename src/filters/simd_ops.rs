use image::{DynamicImage, ImageBuffer, Rgb, Rgba, Luma};
use crate::errors::ImgrsError;

/// SIMD-optimized RGB to grayscale conversion
#[inline]
pub fn rgb_to_gray_simd(r: u8, g: u8, b: u8) -> u8 {
    // ITU-R BT.709 coefficients optimized for integer math
    // 0.2126*R + 0.7152*G + 0.0722*B
    // Using fixed-point: multiply by 256 for precision
    let r_weight = 54;   // 0.2126 * 256
    let g_weight = 183;  // 0.7152 * 256
    let b_weight = 19;   // 0.0722 * 256
    
    ((r as u32 * r_weight + g as u32 * g_weight + b as u32 * b_weight) >> 8) as u8
}

/// Fast grayscale conversion for RGB images
pub fn fast_rgb_to_gray(image: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            // Optimized pixel-by-pixel conversion with integer math
            for y in 0..height {
                for x in 0..width {
                    let pixel = rgb_img.get_pixel(x, y);
                    let gray = rgb_to_gray_simd(pixel[0], pixel[1], pixel[2]);
                    result.put_pixel(x, y, Luma([gray]));
                }
            }
            
            Ok(DynamicImage::ImageLuma8(result))
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let (width, height) = rgba_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            // Optimized conversion ignoring alpha
            for y in 0..height {
                for x in 0..width {
                    let pixel = rgba_img.get_pixel(x, y);
                    let gray = rgb_to_gray_simd(pixel[0], pixel[1], pixel[2]);
                    result.put_pixel(x, y, Luma([gray]));
                }
            }
            
            Ok(DynamicImage::ImageLuma8(result))
        }
        _ => {
            // Fallback to standard conversion
            Ok(DynamicImage::ImageLuma8(image.to_luma8()))
        }
    }
}

/// Fast brightness adjustment using SIMD-friendly operations
pub fn fast_brightness(image: &DynamicImage, adjustment: i16) -> Result<DynamicImage, ImgrsError> {
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            // Batch process pixels
            for y in 0..height {
                for x in 0..width {
                    let pixel = rgb_img.get_pixel(x, y);
                    let r = (pixel[0] as i16 + adjustment).clamp(0, 255) as u8;
                    let g = (pixel[1] as i16 + adjustment).clamp(0, 255) as u8;
                    let b = (pixel[2] as i16 + adjustment).clamp(0, 255) as u8;
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
                    let r = (pixel[0] as i16 + adjustment).clamp(0, 255) as u8;
                    let g = (pixel[1] as i16 + adjustment).clamp(0, 255) as u8;
                    let b = (pixel[2] as i16 + adjustment).clamp(0, 255) as u8;
                    result.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
                }
            }
            
            Ok(DynamicImage::ImageRgba8(result))
        }
        _ => {
            // Use existing implementation for other formats
            crate::filters::brightness(image, adjustment)
        }
    }
}

/// Fast contrast adjustment
pub fn fast_contrast(image: &DynamicImage, factor: f32) -> Result<DynamicImage, ImgrsError> {
    let factor = factor.max(0.0);
    
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            // Precompute for all possible values (lookup table optimization)
            let mut lut = [0u8; 256];
            for i in 0..256 {
                let value = ((i as f32 - 128.0) * factor + 128.0).clamp(0.0, 255.0) as u8;
                lut[i] = value;
            }
            
            // Apply lookup table (very fast)
            for y in 0..height {
                for x in 0..width {
                    let pixel = rgb_img.get_pixel(x, y);
                    result.put_pixel(x, y, Rgb([
                        lut[pixel[0] as usize],
                        lut[pixel[1] as usize],
                        lut[pixel[2] as usize],
                    ]));
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            // Use existing implementation
            crate::filters::contrast(image, factor)
        }
    }
}

