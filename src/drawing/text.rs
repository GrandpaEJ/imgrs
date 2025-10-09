use image::{DynamicImage, Rgb, Rgba};
use crate::errors::ImgrsError;
use std::collections::HashMap;

/// Simple text rendering using a basic bitmap font (8x8 pixels per character)
pub fn draw_text(
    image: &DynamicImage,
    text: &str,
    x: i32,
    y: i32,
    color: (u8, u8, u8, u8),
    scale: u32,
) -> Result<DynamicImage, ImgrsError> {
    let mut result = image.clone();
    let char_width = 8 * scale;
    let _char_height = 8 * scale;
    
    // Simple 8x8 bitmap font for basic ASCII characters (A-Z, 0-9)
    let font_data = get_basic_font_data();
    
    for (i, ch) in text.chars().enumerate() {
        let char_x = x + (i as i32 * char_width as i32);
        
        if let Some(char_bitmap) = font_data.get(&ch) {
            for row in 0..8 {
                for col in 0..8 {
                    if char_bitmap[row] & (1 << (7 - col)) != 0 {
                        // Draw scaled pixel
                        for sy in 0..scale {
                            for sx in 0..scale {
                                let px = char_x + col * scale as i32 + sx as i32;
                                let py = y + row as i32 * scale as i32 + sy as i32;
                                
                                result = draw_pixel(&result, px, py, color)?;
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(result)
}

/// Draw a single pixel with alpha blending
fn draw_pixel(
    image: &DynamicImage,
    x: i32,
    y: i32,
    color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    let mut result = image.clone();
    
    match &mut result {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (img_width, img_height) = rgb_img.dimensions();
            if x >= 0 && y >= 0 && (x as u32) < img_width && (y as u32) < img_height {
                rgb_img.put_pixel(x as u32, y as u32, Rgb([color.0, color.1, color.2]));
            }
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let (img_width, img_height) = rgba_img.dimensions();
            if x >= 0 && y >= 0 && (x as u32) < img_width && (y as u32) < img_height {
                let alpha = color.3 as f32 / 255.0;
                let existing = rgba_img.get_pixel(x as u32, y as u32);
                
                let blended_r = ((1.0 - alpha) * existing[0] as f32 + alpha * color.0 as f32) as u8;
                let blended_g = ((1.0 - alpha) * existing[1] as f32 + alpha * color.1 as f32) as u8;
                let blended_b = ((1.0 - alpha) * existing[2] as f32 + alpha * color.2 as f32) as u8;
                let blended_a = ((1.0 - alpha) * existing[3] as f32 + alpha * 255.0) as u8;
                
                rgba_img.put_pixel(x as u32, y as u32, Rgba([blended_r, blended_g, blended_b, blended_a]));
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

/// Get basic font data for simple text rendering
fn get_basic_font_data() -> HashMap<char, [u8; 8]> {
    let mut font = HashMap::new();
    
    // Basic 8x8 bitmap font data for some characters
    font.insert('A', [0x18, 0x3C, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00]);
    font.insert('B', [0x7C, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x7C, 0x00]);
    font.insert('C', [0x3C, 0x66, 0x60, 0x60, 0x60, 0x66, 0x3C, 0x00]);
    font.insert('D', [0x78, 0x6C, 0x66, 0x66, 0x66, 0x6C, 0x78, 0x00]);
    font.insert('E', [0x7E, 0x60, 0x60, 0x78, 0x60, 0x60, 0x7E, 0x00]);
    font.insert('F', [0x7E, 0x60, 0x60, 0x78, 0x60, 0x60, 0x60, 0x00]);
    font.insert('G', [0x3C, 0x66, 0x60, 0x6E, 0x66, 0x66, 0x3C, 0x00]);
    font.insert('H', [0x66, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00]);
    font.insert('I', [0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00]);
    font.insert('J', [0x1E, 0x0C, 0x0C, 0x0C, 0x0C, 0x6C, 0x38, 0x00]);
    font.insert('0', [0x3C, 0x66, 0x6E, 0x76, 0x66, 0x66, 0x3C, 0x00]);
    font.insert('1', [0x18, 0x18, 0x38, 0x18, 0x18, 0x18, 0x7E, 0x00]);
    font.insert('2', [0x3C, 0x66, 0x06, 0x0C, 0x30, 0x60, 0x7E, 0x00]);
    font.insert('3', [0x3C, 0x66, 0x06, 0x1C, 0x06, 0x66, 0x3C, 0x00]);
    font.insert('4', [0x06, 0x0E, 0x1E, 0x66, 0x7F, 0x06, 0x06, 0x00]);
    font.insert('5', [0x7E, 0x60, 0x7C, 0x06, 0x06, 0x66, 0x3C, 0x00]);
    font.insert(' ', [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    
    font
}

