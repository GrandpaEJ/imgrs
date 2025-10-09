use crate::errors::ImgrsError;
use image::{DynamicImage, Rgb, Rgba};
pub fn draw_rectangle(
    image: &DynamicImage,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    let mut result = image.clone();
    
    match &mut result {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (img_width, img_height) = rgb_img.dimensions();
            
            for dy in 0..height {
                for dx in 0..width {
                    let px = x + dx as i32;
                    let py = y + dy as i32;
                    
                    if px >= 0 && py >= 0 && (px as u32) < img_width && (py as u32) < img_height {
                        rgb_img.put_pixel(px as u32, py as u32, Rgb([color.0, color.1, color.2]));
                    }
                }
            }
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let (img_width, img_height) = rgba_img.dimensions();
            
            for dy in 0..height {
                for dx in 0..width {
                    let px = x + dx as i32;
                    let py = y + dy as i32;
                    
                    if px >= 0 && py >= 0 && (px as u32) < img_width && (py as u32) < img_height {
                        let alpha = color.3 as f32 / 255.0;
                        let existing = rgba_img.get_pixel(px as u32, py as u32);
                        
                        // Alpha blending
                        let blended_r = ((1.0 - alpha) * existing[0] as f32 + alpha * color.0 as f32) as u8;
                        let blended_g = ((1.0 - alpha) * existing[1] as f32 + alpha * color.1 as f32) as u8;
                        let blended_b = ((1.0 - alpha) * existing[2] as f32 + alpha * color.2 as f32) as u8;
                        let blended_a = ((1.0 - alpha) * existing[3] as f32 + alpha * 255.0) as u8;
                        
                        rgba_img.put_pixel(px as u32, py as u32, Rgba([blended_r, blended_g, blended_b, blended_a]));
                    }
                }
            }
        }
        _ => {
            return Err(ImgrsError::InvalidOperation(
                "Unsupported image format for drawing".to_string()
            ));
        }
    }
    
    Ok(result)
}

