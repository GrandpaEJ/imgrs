use image::{DynamicImage, ImageBuffer, Rgb, Rgba};
use crate::errors::ImgrsError;

/// Apply sepia filter (CSS-like sepia effect)
pub fn sepia(image: &DynamicImage, amount: f32) -> Result<DynamicImage, ImgrsError> {
    let amount = amount.max(0.0).min(1.0); // Clamp between 0 and 1
    
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

                    // Sepia transformation matrix
                    let sepia_r = (r * 0.393 + g * 0.769 + b * 0.189).min(255.0);
                    let sepia_g = (r * 0.349 + g * 0.686 + b * 0.168).min(255.0);
                    let sepia_b = (r * 0.272 + g * 0.534 + b * 0.131).min(255.0);

                    // Blend with original based on amount
                    let final_r = (r * (1.0 - amount) + sepia_r * amount) as u8;
                    let final_g = (g * (1.0 - amount) + sepia_g * amount) as u8;
                    let final_b = (b * (1.0 - amount) + sepia_b * amount) as u8;

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

                    let sepia_r = (r * 0.393 + g * 0.769 + b * 0.189).min(255.0);
                    let sepia_g = (r * 0.349 + g * 0.686 + b * 0.168).min(255.0);
                    let sepia_b = (r * 0.272 + g * 0.534 + b * 0.131).min(255.0);

                    let final_r = (r * (1.0 - amount) + sepia_r * amount) as u8;
                    let final_g = (g * (1.0 - amount) + sepia_g * amount) as u8;
                    let final_b = (b * (1.0 - amount) + sepia_b * amount) as u8;

                    result.put_pixel(x, y, Rgba([final_r, final_g, final_b, a]));
                }
            }

            Ok(DynamicImage::ImageRgba8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            sepia(&rgb_dynamic, amount)
        }
    }
}
