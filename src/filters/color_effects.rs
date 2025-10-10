use image::{DynamicImage, ImageBuffer, Rgb};
use crate::errors::ImgrsError;

/// Apply duotone effect with two colors
pub fn duotone(image: &DynamicImage, shadow_color: (u8, u8, u8), highlight_color: (u8, u8, u8)) -> Result<DynamicImage, ImgrsError> {
    let gray_img = image.to_luma8();
    let (width, height) = gray_img.dimensions();
    let mut result = ImageBuffer::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let brightness = gray_img.get_pixel(x, y)[0] as f32 / 255.0;
            
            let r = (shadow_color.0 as f32 * (1.0 - brightness) + highlight_color.0 as f32 * brightness) as u8;
            let g = (shadow_color.1 as f32 * (1.0 - brightness) + highlight_color.1 as f32 * brightness) as u8;
            let b = (shadow_color.2 as f32 * (1.0 - brightness) + highlight_color.2 as f32 * brightness) as u8;
            
            result.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    
    Ok(DynamicImage::ImageRgb8(result))
}

/// Apply tritone effect with three colors
    #[allow(dead_code)]
pub fn tritone(image: &DynamicImage, shadow: (u8, u8, u8), midtone: (u8, u8, u8), highlight: (u8, u8, u8)) -> Result<DynamicImage, ImgrsError> {
    let gray_img = image.to_luma8();
    let (width, height) = gray_img.dimensions();
    let mut result = ImageBuffer::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let brightness = gray_img.get_pixel(x, y)[0] as f32 / 255.0;
            
            let (r, g, b) = if brightness < 0.5 {
                let t = brightness * 2.0;
                (
                    (shadow.0 as f32 * (1.0 - t) + midtone.0 as f32 * t) as u8,
                    (shadow.1 as f32 * (1.0 - t) + midtone.1 as f32 * t) as u8,
                    (shadow.2 as f32 * (1.0 - t) + midtone.2 as f32 * t) as u8,
                )
            } else {
                let t = (brightness - 0.5) * 2.0;
                (
                    (midtone.0 as f32 * (1.0 - t) + highlight.0 as f32 * t) as u8,
                    (midtone.1 as f32 * (1.0 - t) + highlight.1 as f32 * t) as u8,
                    (midtone.2 as f32 * (1.0 - t) + highlight.2 as f32 * t) as u8,
                )
            };
            
            result.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    
    Ok(DynamicImage::ImageRgb8(result))
}

/// Color splash - keep one color range, desaturate the rest
pub fn color_splash(image: &DynamicImage, target_hue: f32, tolerance: f32) -> Result<DynamicImage, ImgrsError> {
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for y in 0..height {
                for x in 0..width {
                    let pixel = rgb_img.get_pixel(x, y);
                    let (h, _s, _v) = rgb_to_hsv(pixel[0], pixel[1], pixel[2]);
                    
                    let hue_diff = (h - target_hue).abs();
                    let hue_diff = hue_diff.min(360.0 - hue_diff);
                    
                    if hue_diff <= tolerance {
                        result.put_pixel(x, y, *pixel);
                    } else {
                        let gray = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;
                        result.put_pixel(x, y, Rgb([gray, gray, gray]));
                    }
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            color_splash(&rgb_dynamic, target_hue, tolerance)
        }
    }
}

/// Channel shift effect
pub fn channel_shift(image: &DynamicImage, r_offset: (i32, i32), g_offset: (i32, i32), b_offset: (i32, i32)) -> Result<DynamicImage, ImgrsError> {
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for y in 0..height {
                for x in 0..width {
                    let r_x = (x as i32 + r_offset.0).max(0).min(width as i32 - 1) as u32;
                    let r_y = (y as i32 + r_offset.1).max(0).min(height as i32 - 1) as u32;
                    let g_x = (x as i32 + g_offset.0).max(0).min(width as i32 - 1) as u32;
                    let g_y = (y as i32 + g_offset.1).max(0).min(height as i32 - 1) as u32;
                    let b_x = (x as i32 + b_offset.0).max(0).min(width as i32 - 1) as u32;
                    let b_y = (y as i32 + b_offset.1).max(0).min(height as i32 - 1) as u32;
                    
                    let r = rgb_img.get_pixel(r_x, r_y)[0];
                    let g = rgb_img.get_pixel(g_x, g_y)[1];
                    let b = rgb_img.get_pixel(b_x, b_y)[2];
                    
                    result.put_pixel(x, y, Rgb([r, g, b]));
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            channel_shift(&rgb_dynamic, r_offset, g_offset, b_offset)
        }
    }
}

/// Apply false color effect
    #[allow(dead_code)]
pub fn false_color(image: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    let gray_img = image.to_luma8();
    let (width, height) = gray_img.dimensions();
    let mut result = ImageBuffer::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let intensity = gray_img.get_pixel(x, y)[0];
            
            // Map intensity to color gradient (e.g., thermal camera colors)
            let (r, g, b) = match intensity {
                0..=63 => (0, 0, intensity * 4),
                64..=127 => (0, (intensity - 64) * 4, 255),
                128..=191 => ((intensity - 128) * 4, 255, 255 - (intensity - 128) * 4),
                _ => (255, 255 - (intensity - 192) * 4, 0),
            };
            
            result.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    
    Ok(DynamicImage::ImageRgb8(result))
}

/// Apply color quantization
    #[allow(dead_code)]
pub fn color_quantize(image: &DynamicImage, levels: u8) -> Result<DynamicImage, ImgrsError> {
    if levels == 0 {
        return Err(ImgrsError::InvalidOperation("Quantization levels must be > 0".to_string()));
    }
    
    let step = 256.0 / levels as f32;
    
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for y in 0..height {
                for x in 0..width {
                    let pixel = rgb_img.get_pixel(x, y);
                    let r = ((pixel[0] as f32 / step).floor() * step).min(255.0) as u8;
                    let g = ((pixel[1] as f32 / step).floor() * step).min(255.0) as u8;
                    let b = ((pixel[2] as f32 / step).floor() * step).min(255.0) as u8;
                    result.put_pixel(x, y, Rgb([r, g, b]));
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            color_quantize(&rgb_dynamic, levels)
        }
    }
}

/// Apply chromatic aberration effect
pub fn chromatic_aberration(image: &DynamicImage, strength: f32) -> Result<DynamicImage, ImgrsError> {
    channel_shift(
        image,
        (-(strength as i32), 0),
        (0, 0),
        (strength as i32, 0)
    )
}

/// Helper function to convert RGB to HSV
fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    
    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };
    
    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;
    
    (if h < 0.0 { h + 360.0 } else { h }, s, v)
}

