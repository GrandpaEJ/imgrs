use image::DynamicImage;
use crate::errors::ImgrsError;
use super::access::map_pixels;

/// Replace all pixels of one color with another color
pub fn replace_color(
    image: &DynamicImage,
    target_color: (u8, u8, u8, u8),
    replacement_color: (u8, u8, u8, u8),
    tolerance: u8,
) -> Result<DynamicImage, ImgrsError> {
    map_pixels(image, |_x, _y, pixel| {
        let distance = color_distance(pixel, target_color);
        if distance <= tolerance as f32 {
            replacement_color
        } else {
            pixel
        }
    })
}

/// Calculate color distance between two pixels
fn color_distance(color1: (u8, u8, u8, u8), color2: (u8, u8, u8, u8)) -> f32 {
    let dr = color1.0 as f32 - color2.0 as f32;
    let dg = color1.1 as f32 - color2.1 as f32;
    let db = color1.2 as f32 - color2.2 as f32;
    let da = color1.3 as f32 - color2.3 as f32;
    
    (dr * dr + dg * dg + db * db + da * da).sqrt()
}

/// Apply threshold to create a binary image
pub fn threshold(image: &DynamicImage, threshold_value: u8) -> Result<DynamicImage, ImgrsError> {
    map_pixels(image, |_x, _y, pixel| {
        // Convert to grayscale first
        let gray = (pixel.0 as f32 * 0.2126 + pixel.1 as f32 * 0.7152 + pixel.2 as f32 * 0.0722) as u8;
        
        if gray >= threshold_value {
            (255, 255, 255, pixel.3) // White
        } else {
            (0, 0, 0, pixel.3) // Black
        }
    })
}

/// Apply posterization effect (reduce number of colors)
pub fn posterize(image: &DynamicImage, levels: u8) -> Result<DynamicImage, ImgrsError> {
    if levels == 0 {
        return Err(ImgrsError::InvalidOperation("Levels must be greater than 0".to_string()));
    }
    
    let step = 255.0 / (levels - 1) as f32;
    
    map_pixels(image, |_x, _y, pixel| {
        let r = ((pixel.0 as f32 / step).round() * step) as u8;
        let g = ((pixel.1 as f32 / step).round() * step) as u8;
        let b = ((pixel.2 as f32 / step).round() * step) as u8;
        
        (r, g, b, pixel.3)
    })
}

