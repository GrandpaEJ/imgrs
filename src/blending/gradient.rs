use image::{DynamicImage, ImageBuffer, Rgba, GenericImageView};
use crate::errors::ImgrsError;
use super::modes::BlendMode;
use super::composite::composite;

/// Gradient direction
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum GradientDirection {
    Horizontal,
    Vertical,
    Diagonal,
    Radial,
}

/// Create a gradient overlay
#[allow(dead_code)]
pub fn gradient_overlay(
    image: &DynamicImage,
    start_color: (u8, u8, u8, u8),
    end_color: (u8, u8, u8, u8),
    direction: GradientDirection,
    blend_mode: BlendMode,
    opacity: f32,
) -> Result<DynamicImage, ImgrsError> {
    let (width, height) = image.dimensions();
    let gradient = create_gradient(width, height, start_color, end_color, direction)?;
    composite(image, &gradient, blend_mode, opacity)
}

/// Create a gradient image
fn create_gradient(
    width: u32,
    height: u32,
    start_color: (u8, u8, u8, u8),
    end_color: (u8, u8, u8, u8),
    direction: GradientDirection,
) -> Result<DynamicImage, ImgrsError> {
    let mut gradient = ImageBuffer::new(width, height);
    
    let start_r = start_color.0 as f32;
    let start_g = start_color.1 as f32;
    let start_b = start_color.2 as f32;
    let start_a = start_color.3 as f32;
    
    let end_r = end_color.0 as f32;
    let end_g = end_color.1 as f32;
    let end_b = end_color.2 as f32;
    let end_a = end_color.3 as f32;
    
    for y in 0..height {
        for x in 0..width {
            let t = match direction {
                GradientDirection::Horizontal => x as f32 / (width - 1) as f32,
                GradientDirection::Vertical => y as f32 / (height - 1) as f32,
                GradientDirection::Diagonal => {
                    ((x + y) as f32) / ((width + height - 2) as f32)
                }
                GradientDirection::Radial => {
                    let center_x = width as f32 / 2.0;
                    let center_y = height as f32 / 2.0;
                    let max_distance = (center_x * center_x + center_y * center_y).sqrt();
                    let distance = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
                    (distance / max_distance).min(1.0)
                }
            };
            
            let r = (start_r * (1.0 - t) + end_r * t) as u8;
            let g = (start_g * (1.0 - t) + end_g * t) as u8;
            let b = (start_b * (1.0 - t) + end_b * t) as u8;
            let a = (start_a * (1.0 - t) + end_a * t) as u8;
            
            gradient.put_pixel(x, y, Rgba([r, g, b, a]));
        }
    }
    
    Ok(DynamicImage::ImageRgba8(gradient))
}

