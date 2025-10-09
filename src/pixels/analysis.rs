use image::{DynamicImage, GenericImageView};
use crate::errors::ImgrsError;
use super::access::get_pixel;

/// Type alias for RGBA histogram data
pub type HistogramData = ([u32; 256], [u32; 256], [u32; 256], [u32; 256]);

/// Create a histogram of pixel values
pub fn histogram(image: &DynamicImage) -> Result<HistogramData, ImgrsError> {
    let (width, height) = image.dimensions();
    let mut r_hist = [0u32; 256];
    let mut g_hist = [0u32; 256];
    let mut b_hist = [0u32; 256];
    let mut a_hist = [0u32; 256];
    
    for y in 0..height {
        for x in 0..width {
            let (r, g, b, a) = get_pixel(image, x, y)?;
            r_hist[r as usize] += 1;
            g_hist[g as usize] += 1;
            b_hist[b as usize] += 1;
            a_hist[a as usize] += 1;
        }
    }
    
    Ok((r_hist, g_hist, b_hist, a_hist))
}

/// Find the dominant color in the image
pub fn dominant_color(image: &DynamicImage) -> Result<(u8, u8, u8, u8), ImgrsError> {
    let (width, height) = image.dimensions();
    let mut color_counts = std::collections::HashMap::new();
    
    for y in 0..height {
        for x in 0..width {
            let pixel = get_pixel(image, x, y)?;
            *color_counts.entry(pixel).or_insert(0) += 1;
        }
    }
    
    color_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(color, _)| color)
        .ok_or_else(|| ImgrsError::InvalidOperation("No pixels found in image".to_string()))
}

/// Calculate average color of the image
pub fn average_color(image: &DynamicImage) -> Result<(u8, u8, u8, u8), ImgrsError> {
    let (width, height) = image.dimensions();
    let total_pixels = (width * height) as u64;
    
    if total_pixels == 0 {
        return Err(ImgrsError::InvalidOperation("Image has no pixels".to_string()));
    }
    
    let mut r_sum = 0u64;
    let mut g_sum = 0u64;
    let mut b_sum = 0u64;
    let mut a_sum = 0u64;
    
    for y in 0..height {
        for x in 0..width {
            let (r, g, b, a) = get_pixel(image, x, y)?;
            r_sum += r as u64;
            g_sum += g as u64;
            b_sum += b as u64;
            a_sum += a as u64;
        }
    }
    
    Ok((
        (r_sum / total_pixels) as u8,
        (g_sum / total_pixels) as u8,
        (b_sum / total_pixels) as u8,
        (a_sum / total_pixels) as u8,
    ))
}

