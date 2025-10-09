use image::{DynamicImage, ImageBuffer, Rgb, Rgba};
use crate::errors::ImgrsError;

/// Apply saturate filter (CSS-like saturate effect)
pub fn saturate(image: &DynamicImage, amount: f32) -> Result<DynamicImage, ImgrsError> {
    let amount = amount.max(0.0); // No upper limit for saturation
    
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);

            for y in 0..height {
                for x in 0..width {
                    let pixel = rgb_img.get_pixel(x, y);
                    let r = pixel[0] as f32;
                    let g = pixel[1] as f32;
                    let b = pixel[2] as f32;

                    // Calculate luminance
                    let luminance = r * 0.2126 + g * 0.7152 + b * 0.0722;

                    // Apply saturation
                    let new_r = (luminance + (r - luminance) * amount).max(0.0).min(255.0) as u8;
                    let new_g = (luminance + (g - luminance) * amount).max(0.0).min(255.0) as u8;
                    let new_b = (luminance + (b - luminance) * amount).max(0.0).min(255.0) as u8;

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
                    let b = pixel[2] as f32;
                    let a = pixel[3];

                    let luminance = r * 0.2126 + g * 0.7152 + b * 0.0722;

                    let new_r = (luminance + (r - luminance) * amount).max(0.0).min(255.0) as u8;
                    let new_g = (luminance + (g - luminance) * amount).max(0.0).min(255.0) as u8;
                    let new_b = (luminance + (b - luminance) * amount).max(0.0).min(255.0) as u8;

                    result.put_pixel(x, y, Rgba([new_r, new_g, new_b, a]));
                }
            }

            Ok(DynamicImage::ImageRgba8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            saturate(&rgb_dynamic, amount)
        }
    }
}

