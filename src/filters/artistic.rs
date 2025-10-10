use image::{DynamicImage, ImageBuffer, Rgb, Rgba};
use crate::errors::ImgrsError;

/// Apply vignette effect
pub fn vignette(image: &DynamicImage, strength: f32, radius: f32) -> Result<DynamicImage, ImgrsError> {
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            let center_x = width as f32 / 2.0;
            let center_y = height as f32 / 2.0;
            let max_dist = ((center_x * center_x + center_y * center_y).sqrt()) * radius;
            
            for y in 0..height {
                for x in 0..width {
                    let dx = x as f32 - center_x;
                    let dy = y as f32 - center_y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    
                    let vignette_factor = if distance < max_dist {
                        1.0 - (distance / max_dist).powf(2.0) * strength
                    } else {
                        1.0 - strength
                    };
                    
                    let pixel = rgb_img.get_pixel(x, y);
                    let r = (pixel[0] as f32 * vignette_factor).max(0.0).min(255.0) as u8;
                    let g = (pixel[1] as f32 * vignette_factor).max(0.0).min(255.0) as u8;
                    let b = (pixel[2] as f32 * vignette_factor).max(0.0).min(255.0) as u8;
                    
                    result.put_pixel(x, y, Rgb([r, g, b]));
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let (width, height) = rgba_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            let center_x = width as f32 / 2.0;
            let center_y = height as f32 / 2.0;
            let max_dist = ((center_x * center_x + center_y * center_y).sqrt()) * radius;
            
            for y in 0..height {
                for x in 0..width {
                    let dx = x as f32 - center_x;
                    let dy = y as f32 - center_y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    
                    let vignette_factor = if distance < max_dist {
                        1.0 - (distance / max_dist).powf(2.0) * strength
                    } else {
                        1.0 - strength
                    };
                    
                    let pixel = rgba_img.get_pixel(x, y);
                    let r = (pixel[0] as f32 * vignette_factor).max(0.0).min(255.0) as u8;
                    let g = (pixel[1] as f32 * vignette_factor).max(0.0).min(255.0) as u8;
                    let b = (pixel[2] as f32 * vignette_factor).max(0.0).min(255.0) as u8;
                    
                    result.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
                }
            }
            
            Ok(DynamicImage::ImageRgba8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            vignette(&rgb_dynamic, strength, radius)
        }
    }
}

/// Apply halftone effect
pub fn halftone(image: &DynamicImage, dot_size: u32) -> Result<DynamicImage, ImgrsError> {
    let gray_img = image.to_luma8();
    let (width, height) = gray_img.dimensions();
    let mut result = ImageBuffer::new(width, height);
    
    for block_y in (0..height).step_by(dot_size as usize) {
        for block_x in (0..width).step_by(dot_size as usize) {
            let mut brightness_sum = 0u32;
            let mut count = 0u32;
            
            for y in block_y..(block_y + dot_size).min(height) {
                for x in block_x..(block_x + dot_size).min(width) {
                    brightness_sum += gray_img.get_pixel(x, y)[0] as u32;
                    count += 1;
                }
            }
            
            let avg_brightness = brightness_sum / count;
            let dot_radius = (dot_size as f32 * (avg_brightness as f32 / 255.0)).sqrt();
            let center_x = block_x as f32 + dot_size as f32 / 2.0;
            let center_y = block_y as f32 + dot_size as f32 / 2.0;
            
            for y in block_y..(block_y + dot_size).min(height) {
                for x in block_x..(block_x + dot_size).min(width) {
                    let dx = x as f32 - center_x;
                    let dy = y as f32 - center_y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    
                    let value = if distance <= dot_radius { 0 } else { 255 };
                    result.put_pixel(x, y, image::Luma([value]));
                }
            }
        }
    }
    
    Ok(DynamicImage::ImageLuma8(result))
}

/// Apply crosshatch effect
    #[allow(dead_code)]
pub fn crosshatch(image: &DynamicImage, spacing: u32, angle: f32) -> Result<DynamicImage, ImgrsError> {
    let gray_img = image.to_luma8();
    let (width, height) = gray_img.dimensions();
    let mut result = ImageBuffer::new(width, height);
    
    let angle_rad = angle.to_radians();
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();
    
    for y in 0..height {
        for x in 0..width {
            let brightness = gray_img.get_pixel(x, y)[0];
            
            // Rotate coordinates
            let rx = x as f32 * cos_a - y as f32 * sin_a;
            let ry = x as f32 * sin_a + y as f32 * cos_a;
            
            // Create crosshatch pattern based on brightness
            let threshold = brightness as f32 / 255.0;
            let mut hatch_value = 255u8;
            
            if (rx % spacing as f32) < 1.0 && threshold < 0.75 {
                hatch_value = 0;
            }
            if (ry % spacing as f32) < 1.0 && threshold < 0.5 {
                hatch_value = 0;
            }
            if ((rx + ry) % spacing as f32) < 1.0 && threshold < 0.25 {
                hatch_value = 0;
            }
            
            result.put_pixel(x, y, image::Luma([hatch_value]));
        }
    }
    
    Ok(DynamicImage::ImageLuma8(result))
}

/// Apply pencil sketch effect
pub fn pencil_sketch(image: &DynamicImage, detail: f32) -> Result<DynamicImage, ImgrsError> {
    use super::blur::blur;
    use super::edges::edge_detect;
    
    // Blur the image
    let blurred = blur(image, detail)?;
    
    // Detect edges
    let edges = edge_detect(&blurred)?;
    
    // Invert edges for sketch effect
    if let DynamicImage::ImageLuma8(edge_img) = edges {
        let (width, height) = edge_img.dimensions();
        let mut result = ImageBuffer::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let val = edge_img.get_pixel(x, y)[0];
                result.put_pixel(x, y, image::Luma([255 - val]));
            }
        }
        
        Ok(DynamicImage::ImageLuma8(result))
    } else {
        Err(ImgrsError::InvalidOperation("Pencil sketch effect failed".to_string()))
    }
}

/// Apply color pencil effect
    #[allow(dead_code)]
pub fn color_pencil(image: &DynamicImage, detail: f32) -> Result<DynamicImage, ImgrsError> {
    use super::blur::blur;
    use super::edges::edge_detect;
    
    let blurred = blur(image, detail)?;
    let edges = edge_detect(image)?;
    
    match (&blurred, &edges) {
        (DynamicImage::ImageRgb8(color_img), DynamicImage::ImageLuma8(edge_img)) => {
            let (width, height) = color_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for y in 0..height {
                for x in 0..width {
                    let color = color_img.get_pixel(x, y);
                    let edge_val = 255 - edge_img.get_pixel(x, y)[0];
                    let blend = edge_val as f32 / 255.0;
                    
                    let r = (color[0] as f32 * blend).min(255.0) as u8;
                    let g = (color[1] as f32 * blend).min(255.0) as u8;
                    let b = (color[2] as f32 * blend).min(255.0) as u8;
                    
                    result.put_pixel(x, y, Rgb([r, g, b]));
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            let rgb_img = blurred.to_rgb8();
            Ok(DynamicImage::ImageRgb8(rgb_img))
        }
    }
}

/// Apply watercolor effect
pub fn watercolor(image: &DynamicImage, iterations: u32) -> Result<DynamicImage, ImgrsError> {
    use super::advanced_blur::median_blur;
    use super::stylistic::posterize;
    
    let mut result = image.clone();
    
    // Apply median blur multiple times for smoothing
    for _ in 0..iterations {
        result = median_blur(&result, 2)?;
    }
    
    // Posterize to create color regions
    result = posterize(&result, 8)?;
    
    Ok(result)
}

/// Apply glitch effect
pub fn glitch(image: &DynamicImage, intensity: f32) -> Result<DynamicImage, ImgrsError> {
    use rand::Rng;
    
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = rgb_img.clone();
            let mut rng = rand::thread_rng();
            
            let num_glitches = (height as f32 * intensity / 10.0) as u32;
            
            for _ in 0..num_glitches {
                let y = rng.gen_range(0..height);
                let offset = rng.gen_range(-20..20);
                let strip_height = rng.gen_range(1..10);
                
                for dy in 0..strip_height {
                    let source_y = (y + dy).min(height - 1);
                    for x in 0..width {
                        let source_x = ((x as i32 + offset).max(0).min(width as i32 - 1)) as u32;
                        let pixel = rgb_img.get_pixel(source_x, source_y);
                        result.put_pixel(x, source_y, *pixel);
                    }
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            glitch(&rgb_dynamic, intensity)
        }
    }
}

