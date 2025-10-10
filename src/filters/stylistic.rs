use image::{DynamicImage, ImageBuffer, Rgb, Rgba};
use crate::errors::ImgrsError;

/// Apply oil painting effect
pub fn oil_painting(image: &DynamicImage, radius: u32, intensity: u32) -> Result<DynamicImage, ImgrsError> {
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for y in 0..height {
                for x in 0..width {
                    let mut intensity_count = vec![0u32; 256];
                    let mut r_avg = vec![0u32; 256];
                    let mut g_avg = vec![0u32; 256];
                    let mut b_avg = vec![0u32; 256];
                    
                    for dy in -(radius as i32)..=(radius as i32) {
                        for dx in -(radius as i32)..=(radius as i32) {
                            let nx = (x as i32 + dx).max(0).min(width as i32 - 1) as u32;
                            let ny = (y as i32 + dy).max(0).min(height as i32 - 1) as u32;
                            let pixel = rgb_img.get_pixel(nx, ny);
                            
                            let intensity_val = ((pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3) as usize;
                            let bucket = (intensity_val * intensity as usize / 255).min(255);
                            
                            intensity_count[bucket] += 1;
                            r_avg[bucket] += pixel[0] as u32;
                            g_avg[bucket] += pixel[1] as u32;
                            b_avg[bucket] += pixel[2] as u32;
                        }
                    }
                    
                    let max_bucket = intensity_count.iter().enumerate()
                        .max_by_key(|(_, &count)| count)
                        .map(|(idx, _)| idx)
                        .unwrap_or(0);
                    
                    let count = intensity_count[max_bucket].max(1);
                    let r = (r_avg[max_bucket] / count) as u8;
                    let g = (g_avg[max_bucket] / count) as u8;
                    let b = (b_avg[max_bucket] / count) as u8;
                    
                    result.put_pixel(x, y, Rgb([r, g, b]));
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            oil_painting(&rgb_dynamic, radius, intensity)
        }
    }
}

/// Apply posterize effect
pub fn posterize(image: &DynamicImage, levels: u8) -> Result<DynamicImage, ImgrsError> {
    if levels == 0 {
        return Err(ImgrsError::InvalidOperation("Posterize levels must be greater than 0".to_string()));
    }
    
    let step = 255.0 / levels as f32;
    
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for y in 0..height {
                for x in 0..width {
                    let pixel = rgb_img.get_pixel(x, y);
                    let r = ((pixel[0] as f32 / step).floor() * step) as u8;
                    let g = ((pixel[1] as f32 / step).floor() * step) as u8;
                    let b = ((pixel[2] as f32 / step).floor() * step) as u8;
                    result.put_pixel(x, y, Rgb([r, g, b]));
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
                    let r = ((pixel[0] as f32 / step).floor() * step) as u8;
                    let g = ((pixel[1] as f32 / step).floor() * step) as u8;
                    let b = ((pixel[2] as f32 / step).floor() * step) as u8;
                    result.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
                }
            }
            
            Ok(DynamicImage::ImageRgba8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            posterize(&rgb_dynamic, levels)
        }
    }
}

/// Apply pixelate effect
pub fn pixelate(image: &DynamicImage, pixel_size: u32) -> Result<DynamicImage, ImgrsError> {
    if pixel_size == 0 {
        return Ok(image.clone());
    }
    
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for block_y in (0..height).step_by(pixel_size as usize) {
                for block_x in (0..width).step_by(pixel_size as usize) {
                    let mut r_sum = 0u32;
                    let mut g_sum = 0u32;
                    let mut b_sum = 0u32;
                    let mut count = 0u32;
                    
                    for y in block_y..((block_y + pixel_size).min(height)) {
                        for x in block_x..((block_x + pixel_size).min(width)) {
                            let pixel = rgb_img.get_pixel(x, y);
                            r_sum += pixel[0] as u32;
                            g_sum += pixel[1] as u32;
                            b_sum += pixel[2] as u32;
                            count += 1;
                        }
                    }
                    
                    let r_avg = (r_sum / count) as u8;
                    let g_avg = (g_sum / count) as u8;
                    let b_avg = (b_sum / count) as u8;
                    
                    for y in block_y..((block_y + pixel_size).min(height)) {
                        for x in block_x..((block_x + pixel_size).min(width)) {
                            result.put_pixel(x, y, Rgb([r_avg, g_avg, b_avg]));
                        }
                    }
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            pixelate(&rgb_dynamic, pixel_size)
        }
    }
}

/// Apply mosaic effect
pub fn mosaic(image: &DynamicImage, tile_size: u32) -> Result<DynamicImage, ImgrsError> {
    pixelate(image, tile_size)
}

/// Apply cartoon effect
pub fn cartoon(image: &DynamicImage, num_levels: u8, edge_threshold: f32) -> Result<DynamicImage, ImgrsError> {
    use super::edges::edge_detect;
    
    // Step 1: Posterize to reduce colors
    let posterized = posterize(image, num_levels)?;
    
    // Step 2: Detect edges
    let edges = edge_detect(image)?;
    
    // Step 3: Combine posterized image with edges
    match (&posterized, &edges) {
        (DynamicImage::ImageRgb8(color_img), DynamicImage::ImageLuma8(edge_img)) => {
            let (width, height) = color_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for y in 0..height {
                for x in 0..width {
                    let edge_val = edge_img.get_pixel(x, y)[0] as f32;
                    let color = color_img.get_pixel(x, y);
                    
                    if edge_val > edge_threshold {
                        result.put_pixel(x, y, Rgb([0, 0, 0])); // Black edges
                    } else {
                        result.put_pixel(x, y, *color);
                    }
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            let rgb_img = posterized.to_rgb8();
            Ok(DynamicImage::ImageRgb8(rgb_img))
        }
    }
}

/// Apply sketch effect
pub fn sketch(image: &DynamicImage, detail_level: f32) -> Result<DynamicImage, ImgrsError> {
    use super::edges::edge_detect;
    
    let edges = edge_detect(image)?;
    
    if let DynamicImage::ImageLuma8(edge_img) = edges {
        let (width, height) = edge_img.dimensions();
        let mut result = ImageBuffer::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let val = edge_img.get_pixel(x, y)[0];
                let inverted = 255 - val;
                let adjusted = (inverted as f32 * detail_level).min(255.0) as u8;
                result.put_pixel(x, y, image::Luma([adjusted]));
            }
        }
        
        Ok(DynamicImage::ImageLuma8(result))
    } else {
        Err(ImgrsError::InvalidOperation("Sketch effect failed".to_string()))
    }
}

/// Apply solarize effect
pub fn solarize(image: &DynamicImage, threshold: u8) -> Result<DynamicImage, ImgrsError> {
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for y in 0..height {
                for x in 0..width {
                    let pixel = rgb_img.get_pixel(x, y);
                    let r = if pixel[0] < threshold { 255 - pixel[0] } else { pixel[0] };
                    let g = if pixel[1] < threshold { 255 - pixel[1] } else { pixel[1] };
                    let b = if pixel[2] < threshold { 255 - pixel[2] } else { pixel[2] };
                    result.put_pixel(x, y, Rgb([r, g, b]));
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
                    let r = if pixel[0] < threshold { 255 - pixel[0] } else { pixel[0] };
                    let g = if pixel[1] < threshold { 255 - pixel[1] } else { pixel[1] };
                    let b = if pixel[2] < threshold { 255 - pixel[2] } else { pixel[2] };
                    result.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
                }
            }
            
            Ok(DynamicImage::ImageRgba8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            solarize(&rgb_dynamic, threshold)
        }
    }
}

