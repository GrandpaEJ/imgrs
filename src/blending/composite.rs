use image::{DynamicImage, ImageBuffer, Rgba, GenericImageView};
use crate::errors::ImgrsError;
use super::modes::{BlendMode, blend_pixels};

/// Composite two images using the specified blend mode
#[allow(dead_code)]
pub fn composite(
    base: &DynamicImage,
    overlay: &DynamicImage,
    blend_mode: BlendMode,
    opacity: f32,
) -> Result<DynamicImage, ImgrsError> {
    let (base_width, base_height) = base.dimensions();
    let (overlay_width, overlay_height) = overlay.dimensions();
    
    // Use the smaller dimensions
    let width = base_width.min(overlay_width);
    let height = base_height.min(overlay_height);
    
    let base_rgba = base.to_rgba8();
    let overlay_rgba = overlay.to_rgba8();
    let mut result = ImageBuffer::new(width, height);
    
    let opacity = opacity.max(0.0).min(1.0);
    
    for y in 0..height {
        for x in 0..width {
            let base_pixel = base_rgba.get_pixel(x, y);
            let overlay_pixel = overlay_rgba.get_pixel(x, y);
            
            let blended = blend_pixels(
                (base_pixel[0], base_pixel[1], base_pixel[2], base_pixel[3]),
                (overlay_pixel[0], overlay_pixel[1], overlay_pixel[2], overlay_pixel[3]),
                blend_mode,
                opacity,
            );
            
            result.put_pixel(x, y, Rgba([blended.0, blended.1, blended.2, blended.3]));
        }
    }
    
    Ok(DynamicImage::ImageRgba8(result))
}

/// Alpha composite two images (standard alpha blending)
#[allow(dead_code)]
pub fn alpha_composite(base: &DynamicImage, overlay: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    composite(base, overlay, BlendMode::Normal, 1.0)
}

/// Apply a color overlay to an image
#[allow(dead_code)]
pub fn color_overlay(
    image: &DynamicImage,
    color: (u8, u8, u8, u8),
    blend_mode: BlendMode,
    opacity: f32,
) -> Result<DynamicImage, ImgrsError> {
    let (width, height) = image.dimensions();
    let color_layer = create_solid_color(width, height, color)?;
    composite(image, &color_layer, blend_mode, opacity)
}

/// Create a solid color image
fn create_solid_color(
    width: u32,
    height: u32,
    color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    let mut solid = ImageBuffer::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            solid.put_pixel(x, y, Rgba([color.0, color.1, color.2, color.3]));
        }
    }
    
    Ok(DynamicImage::ImageRgba8(solid))
}

