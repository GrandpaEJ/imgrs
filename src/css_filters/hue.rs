use image::{DynamicImage, ImageBuffer, Rgb, Rgba};
use crate::errors::ImgrsError;

/// Apply hue rotation filter (CSS-like hue-rotate effect)
pub fn hue_rotate(image: &DynamicImage, degrees: f32) -> Result<DynamicImage, ImgrsError> {
    let radians = degrees.to_radians();
    let cos_val = radians.cos();
    let sin_val = radians.sin();
    
    // Hue rotation matrix coefficients
    let a = cos_val + (1.0 - cos_val) / 3.0;
    let b = (1.0 - cos_val) / 3.0 - (3.0_f32).sqrt() * sin_val / 3.0;
    let c = (1.0 - cos_val) / 3.0 + (3.0_f32).sqrt() * sin_val / 3.0;
    
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);

            for y in 0..height {
                for x in 0..width {
                    let pixel = rgb_img.get_pixel(x, y);
                    let r = pixel[0] as f32;
                    let g = pixel[1] as f32;
                    let b_val = pixel[2] as f32;

                    let new_r = (a * r + b * g + c * b_val).max(0.0).min(255.0) as u8;
                    let new_g = (c * r + a * g + b * b_val).max(0.0).min(255.0) as u8;
                    let new_b = (b * r + c * g + a * b_val).max(0.0).min(255.0) as u8;

                    result.put_pixel(x, y, Rgb([new_r, new_g, new_b]));
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
                    let r = pixel[0] as f32;
                    let g = pixel[1] as f32;
                    let b_val = pixel[2] as f32;
                    let alpha = pixel[3];

                    let new_r = (a * r + b * g + c * b_val).max(0.0).min(255.0) as u8;
                    let new_g = (c * r + a * g + b * b_val).max(0.0).min(255.0) as u8;
                    let new_b = (b * r + c * g + a * b_val).max(0.0).min(255.0) as u8;

                    result.put_pixel(x, y, Rgba([new_r, new_g, new_b, alpha]));
                }
            }

            Ok(DynamicImage::ImageRgba8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            hue_rotate(&rgb_dynamic, degrees)
        }
    }
}

