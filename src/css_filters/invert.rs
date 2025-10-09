use image::{DynamicImage, ImageBuffer, Rgb, Rgba};
use crate::errors::ImgrsError;

/// Apply invert filter (CSS-like invert effect)
pub fn invert(image: &DynamicImage, amount: f32) -> Result<DynamicImage, ImgrsError> {
    let amount = amount.max(0.0).min(1.0);
    
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

                    let inv_r = 255.0 - r;
                    let inv_g = 255.0 - g;
                    let inv_b = 255.0 - b;

                    let final_r = (r * (1.0 - amount) + inv_r * amount) as u8;
                    let final_g = (g * (1.0 - amount) + inv_g * amount) as u8;
                    let final_b = (b * (1.0 - amount) + inv_b * amount) as u8;

                    result.put_pixel(x, y, Rgb([final_r, final_g, final_b]));
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

                    let inv_r = 255.0 - r;
                    let inv_g = 255.0 - g;
                    let inv_b = 255.0 - b;

                    let final_r = (r * (1.0 - amount) + inv_r * amount) as u8;
                    let final_g = (g * (1.0 - amount) + inv_g * amount) as u8;
                    let final_b = (b * (1.0 - amount) + inv_b * amount) as u8;

                    result.put_pixel(x, y, Rgba([final_r, final_g, final_b, a]));
                }
            }

            Ok(DynamicImage::ImageRgba8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            invert(&rgb_dynamic, amount)
        }
    }
}

