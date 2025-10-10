use image::{DynamicImage, ImageBuffer, Rgb, Rgba};
use crate::errors::ImgrsError;
use rand::Rng;

/// Add Gaussian noise to an image
pub fn add_gaussian_noise(image: &DynamicImage, mean: f32, stddev: f32) -> Result<DynamicImage, ImgrsError> {
    use rand_distr::{Distribution, Normal};
    let normal = Normal::new(mean, stddev).map_err(|e| 
        ImgrsError::InvalidOperation(format!("Invalid noise parameters: {}", e)))?;
    
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            let mut rng = rand::thread_rng();
            
            for y in 0..height {
                for x in 0..width {
                    let pixel = rgb_img.get_pixel(x, y);
                    let noise = normal.sample(&mut rng);
                    
                    let r = (pixel[0] as f32 + noise).max(0.0).min(255.0) as u8;
                    let g = (pixel[1] as f32 + noise).max(0.0).min(255.0) as u8;
                    let b = (pixel[2] as f32 + noise).max(0.0).min(255.0) as u8;
                    
                    result.put_pixel(x, y, Rgb([r, g, b]));
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let (width, height) = rgba_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            let mut rng = rand::thread_rng();
            
            for y in 0..height {
                for x in 0..width {
                    let pixel = rgba_img.get_pixel(x, y);
                    let noise = normal.sample(&mut rng);
                    
                    let r = (pixel[0] as f32 + noise).max(0.0).min(255.0) as u8;
                    let g = (pixel[1] as f32 + noise).max(0.0).min(255.0) as u8;
                    let b = (pixel[2] as f32 + noise).max(0.0).min(255.0) as u8;
                    
                    result.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
                }
            }
            
            Ok(DynamicImage::ImageRgba8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            add_gaussian_noise(&rgb_dynamic, mean, stddev)
        }
    }
}

/// Add salt and pepper noise to an image
pub fn add_salt_pepper_noise(image: &DynamicImage, amount: f32) -> Result<DynamicImage, ImgrsError> {
    if amount < 0.0 || amount > 1.0 {
        return Err(ImgrsError::InvalidOperation("Noise amount must be between 0 and 1".to_string()));
    }
    
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = rgb_img.clone();
            let mut rng = rand::thread_rng();
            
            let num_pixels = (width * height) as f32 * amount;
            
            for _ in 0..(num_pixels as u32) {
                let x = rng.gen_range(0..width);
                let y = rng.gen_range(0..height);
                let value = if rng.gen_bool(0.5) { 255 } else { 0 };
                result.put_pixel(x, y, Rgb([value, value, value]));
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let (width, height) = rgba_img.dimensions();
            let mut result = rgba_img.clone();
            let mut rng = rand::thread_rng();
            
            let num_pixels = (width * height) as f32 * amount;
            
            for _ in 0..(num_pixels as u32) {
                let x = rng.gen_range(0..width);
                let y = rng.gen_range(0..height);
                let value = if rng.gen_bool(0.5) { 255 } else { 0 };
                let alpha = result.get_pixel(x, y)[3];
                result.put_pixel(x, y, Rgba([value, value, value, alpha]));
            }
            
            Ok(DynamicImage::ImageRgba8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            add_salt_pepper_noise(&rgb_dynamic, amount)
        }
    }
}

/// Add uniform noise to an image
    #[allow(dead_code)]
pub fn add_uniform_noise(image: &DynamicImage, min: f32, max: f32) -> Result<DynamicImage, ImgrsError> {
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            let mut rng = rand::thread_rng();
            
            for y in 0..height {
                for x in 0..width {
                    let pixel = rgb_img.get_pixel(x, y);
                    let noise = rng.gen_range(min..max);
                    
                    let r = (pixel[0] as f32 + noise).max(0.0).min(255.0) as u8;
                    let g = (pixel[1] as f32 + noise).max(0.0).min(255.0) as u8;
                    let b = (pixel[2] as f32 + noise).max(0.0).min(255.0) as u8;
                    
                    result.put_pixel(x, y, Rgb([r, g, b]));
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            add_uniform_noise(&rgb_dynamic, min, max)
        }
    }
}

/// Simple denoise using median filter
pub fn denoise(image: &DynamicImage, radius: u32) -> Result<DynamicImage, ImgrsError> {
    super::advanced_blur::median_blur(image, radius)
}

/// Non-local means denoising (simplified version)
    #[allow(dead_code)]
pub fn nl_means_denoise(image: &DynamicImage, h: f32, template_window_size: u32, search_window_size: u32) -> Result<DynamicImage, ImgrsError> {
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            let h_sq = h * h;
            
            for y in 0..height {
                for x in 0..width {
                    let mut r_sum = 0.0;
                    let mut g_sum = 0.0;
                    let mut b_sum = 0.0;
                    let mut weight_sum = 0.0;
                    
                    // Search window
                    let search_start_x = (x as i32 - search_window_size as i32).max(0) as u32;
                    let search_end_x = (x + search_window_size).min(width - 1);
                    let search_start_y = (y as i32 - search_window_size as i32).max(0) as u32;
                    let search_end_y = (y + search_window_size).min(height - 1);
                    
                    for sy in search_start_y..=search_end_y {
                        for sx in search_start_x..=search_end_x {
                            let mut dist_sq = 0.0;
                            
                            // Template window comparison
                            for dy in -(template_window_size as i32)..=(template_window_size as i32) {
                                for dx in -(template_window_size as i32)..=(template_window_size as i32) {
                                    let px1 = ((x as i32 + dx).max(0).min(width as i32 - 1)) as u32;
                                    let py1 = ((y as i32 + dy).max(0).min(height as i32 - 1)) as u32;
                                    let px2 = ((sx as i32 + dx).max(0).min(width as i32 - 1)) as u32;
                                    let py2 = ((sy as i32 + dy).max(0).min(height as i32 - 1)) as u32;
                                    
                                    let p1 = rgb_img.get_pixel(px1, py1);
                                    let p2 = rgb_img.get_pixel(px2, py2);
                                    
                                    for i in 0..3 {
                                        let diff = p1[i] as f32 - p2[i] as f32;
                                        dist_sq += diff * diff;
                                    }
                                }
                            }
                            
                            let weight = (-dist_sq / h_sq).exp();
                            let pixel = rgb_img.get_pixel(sx, sy);
                            
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
            nl_means_denoise(&rgb_dynamic, h, template_window_size, search_window_size)
        }
    }
}

