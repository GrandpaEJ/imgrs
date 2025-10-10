use image::{DynamicImage, ImageBuffer, Rgb, Rgba};
use crate::errors::ImgrsError;
use super::kernel::apply_convolution;

/// Apply box blur to an image
pub fn box_blur(image: &DynamicImage, radius: u32) -> Result<DynamicImage, ImgrsError> {
    if radius == 0 {
        return Ok(image.clone());
    }

    let size = (radius * 2 + 1) as usize;
    let value = 1.0 / (size * size) as f32;
    let kernel = vec![vec![value; size]; size];
    
    apply_convolution(image, &kernel)
}

/// Apply motion blur to an image
pub fn motion_blur(image: &DynamicImage, size: u32, angle: f32) -> Result<DynamicImage, ImgrsError> {
    if size == 0 {
        return Ok(image.clone());
    }

    let kernel_size = (size * 2 + 1) as usize;
    let mut kernel = vec![vec![0.0; kernel_size]; kernel_size];
    let center = size as i32;
    
    let angle_rad = angle.to_radians();
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();
    
    let mut count = 0.0;
    for i in 0..=size * 2 {
        let offset = i as i32 - center;
        let x = center + (offset as f32 * cos_a).round() as i32;
        let y = center + (offset as f32 * sin_a).round() as i32;
        
        if x >= 0 && x < kernel_size as i32 && y >= 0 && y < kernel_size as i32 {
            kernel[y as usize][x as usize] = 1.0;
            count += 1.0;
        }
    }
    
    // Normalize kernel
    for row in kernel.iter_mut() {
        for val in row.iter_mut() {
            *val /= count;
        }
    }
    
    apply_convolution(image, &kernel)
}

/// Apply median blur to reduce noise while preserving edges
pub fn median_blur(image: &DynamicImage, radius: u32) -> Result<DynamicImage, ImgrsError> {
    if radius == 0 {
        return Ok(image.clone());
    }

    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for y in 0..height {
                for x in 0..width {
                    let mut r_vals = Vec::new();
                    let mut g_vals = Vec::new();
                    let mut b_vals = Vec::new();
                    
                    for dy in -(radius as i32)..=(radius as i32) {
                        for dx in -(radius as i32)..=(radius as i32) {
                            let nx = (x as i32 + dx).max(0).min(width as i32 - 1) as u32;
                            let ny = (y as i32 + dy).max(0).min(height as i32 - 1) as u32;
                            let pixel = rgb_img.get_pixel(nx, ny);
                            r_vals.push(pixel[0]);
                            g_vals.push(pixel[1]);
                            b_vals.push(pixel[2]);
                        }
                    }
                    
                    r_vals.sort_unstable();
                    g_vals.sort_unstable();
                    b_vals.sort_unstable();
                    
                    let mid = r_vals.len() / 2;
                    result.put_pixel(x, y, Rgb([r_vals[mid], g_vals[mid], b_vals[mid]]));
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let (width, height) = rgba_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for y in 0..height {
                for x in 0..width {
                    let mut r_vals = Vec::new();
                    let mut g_vals = Vec::new();
                    let mut b_vals = Vec::new();
                    let mut a_vals = Vec::new();
                    
                    for dy in -(radius as i32)..=(radius as i32) {
                        for dx in -(radius as i32)..=(radius as i32) {
                            let nx = (x as i32 + dx).max(0).min(width as i32 - 1) as u32;
                            let ny = (y as i32 + dy).max(0).min(height as i32 - 1) as u32;
                            let pixel = rgba_img.get_pixel(nx, ny);
                            r_vals.push(pixel[0]);
                            g_vals.push(pixel[1]);
                            b_vals.push(pixel[2]);
                            a_vals.push(pixel[3]);
                        }
                    }
                    
                    r_vals.sort_unstable();
                    g_vals.sort_unstable();
                    b_vals.sort_unstable();
                    a_vals.sort_unstable();
                    
                    let mid = r_vals.len() / 2;
                    result.put_pixel(x, y, Rgba([r_vals[mid], g_vals[mid], b_vals[mid], a_vals[mid]]));
                }
            }
            
            Ok(DynamicImage::ImageRgba8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            median_blur(&rgb_dynamic, radius)
        }
    }
}

/// Apply bilateral blur (edge-preserving blur)
pub fn bilateral_blur(image: &DynamicImage, radius: u32, sigma_color: f32, sigma_space: f32) -> Result<DynamicImage, ImgrsError> {
    if radius == 0 {
        return Ok(image.clone());
    }

    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for y in 0..height {
                for x in 0..width {
                    let center_pixel = rgb_img.get_pixel(x, y);
                    let mut r_sum = 0.0;
                    let mut g_sum = 0.0;
                    let mut b_sum = 0.0;
                    let mut weight_sum = 0.0;
                    
                    for dy in -(radius as i32)..=(radius as i32) {
                        for dx in -(radius as i32)..=(radius as i32) {
                            let nx = (x as i32 + dx).max(0).min(width as i32 - 1) as u32;
                            let ny = (y as i32 + dy).max(0).min(height as i32 - 1) as u32;
                            let pixel = rgb_img.get_pixel(nx, ny);
                            
                            // Spatial distance
                            let spatial_dist = (dx * dx + dy * dy) as f32;
                            let spatial_weight = (-spatial_dist / (2.0 * sigma_space * sigma_space)).exp();
                            
                            // Color distance
                            let color_dist = ((center_pixel[0] as f32 - pixel[0] as f32).powi(2) +
                                            (center_pixel[1] as f32 - pixel[1] as f32).powi(2) +
                                            (center_pixel[2] as f32 - pixel[2] as f32).powi(2)).sqrt();
                            let color_weight = (-color_dist / (2.0 * sigma_color * sigma_color)).exp();
                            
                            let weight = spatial_weight * color_weight;
                            
                            r_sum += pixel[0] as f32 * weight;
                            g_sum += pixel[1] as f32 * weight;
                            b_sum += pixel[2] as f32 * weight;
                            weight_sum += weight;
                        }
                    }
                    
                    let r = (r_sum / weight_sum).round().max(0.0).min(255.0) as u8;
                    let g = (g_sum / weight_sum).round().max(0.0).min(255.0) as u8;
                    let b = (b_sum / weight_sum).round().max(0.0).min(255.0) as u8;
                    
                    result.put_pixel(x, y, Rgb([r, g, b]));
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            bilateral_blur(&rgb_dynamic, radius, sigma_color, sigma_space)
        }
    }
}

/// Apply radial blur effect
pub fn radial_blur(image: &DynamicImage, strength: f32) -> Result<DynamicImage, ImgrsError> {
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            let center_x = width as f32 / 2.0;
            let center_y = height as f32 / 2.0;
            let samples = 8;
            
            for y in 0..height {
                for x in 0..width {
                    let dx = x as f32 - center_x;
                    let dy = y as f32 - center_y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    let blur_amount = (distance * strength / 100.0).min(10.0);
                    
                    let mut r_sum = 0.0;
                    let mut g_sum = 0.0;
                    let mut b_sum = 0.0;
                    
                    for i in 0..samples {
                        let offset = (i as f32 - samples as f32 / 2.0) * blur_amount / samples as f32;
                        let sample_x = (x as f32 + dx * offset / distance.max(1.0))
                            .max(0.0).min(width as f32 - 1.0) as u32;
                        let sample_y = (y as f32 + dy * offset / distance.max(1.0))
                            .max(0.0).min(height as f32 - 1.0) as u32;
                        
                        let pixel = rgb_img.get_pixel(sample_x, sample_y);
                        r_sum += pixel[0] as f32;
                        g_sum += pixel[1] as f32;
                        b_sum += pixel[2] as f32;
                    }
                    
                    let r = (r_sum / samples as f32) as u8;
                    let g = (g_sum / samples as f32) as u8;
                    let b = (b_sum / samples as f32) as u8;
                    
                    result.put_pixel(x, y, Rgb([r, g, b]));
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            radial_blur(&rgb_dynamic, strength)
        }
    }
}

/// Apply zoom blur effect
pub fn zoom_blur(image: &DynamicImage, strength: f32) -> Result<DynamicImage, ImgrsError> {
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            let center_x = width as f32 / 2.0;
            let center_y = height as f32 / 2.0;
            let samples = 10;
            
            for y in 0..height {
                for x in 0..width {
                    let mut r_sum = 0.0;
                    let mut g_sum = 0.0;
                    let mut b_sum = 0.0;
                    
                    for i in 0..samples {
                        let scale = 1.0 - (i as f32 / samples as f32) * strength / 100.0;
                        let offset_x = (x as f32 - center_x) * (1.0 - scale);
                        let offset_y = (y as f32 - center_y) * (1.0 - scale);
                        
                        let sample_x = (x as f32 + offset_x)
                            .max(0.0).min(width as f32 - 1.0) as u32;
                        let sample_y = (y as f32 + offset_y)
                            .max(0.0).min(height as f32 - 1.0) as u32;
                        
                        let pixel = rgb_img.get_pixel(sample_x, sample_y);
                        r_sum += pixel[0] as f32;
                        g_sum += pixel[1] as f32;
                        b_sum += pixel[2] as f32;
                    }
                    
                    let r = (r_sum / samples as f32) as u8;
                    let g = (g_sum / samples as f32) as u8;
                    let b = (b_sum / samples as f32) as u8;
                    
                    result.put_pixel(x, y, Rgb([r, g, b]));
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            zoom_blur(&rgb_dynamic, strength)
        }
    }
}

