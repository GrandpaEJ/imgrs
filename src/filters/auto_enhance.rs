/// Automatic image enhancement features
/// Includes histogram equalization, auto-contrast, auto-brightness, and exposure adjustment

use image::{DynamicImage, ImageBuffer, Rgb, Rgba, Luma};
use crate::errors::ImgrsError;

/// Apply histogram equalization to enhance contrast
pub fn histogram_equalization(image: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    match image {
        DynamicImage::ImageLuma8(gray_img) => {
            let (width, height) = gray_img.dimensions();
            
            // Calculate histogram
            let mut histogram = vec![0u32; 256];
            for pixel in gray_img.pixels() {
                histogram[pixel[0] as usize] += 1;
            }
            
            // Calculate cumulative distribution function (CDF)
            let mut cdf = vec![0u32; 256];
            cdf[0] = histogram[0];
            for i in 1..256 {
                cdf[i] = cdf[i - 1] + histogram[i];
            }
            
            // Find minimum non-zero CDF value
            let cdf_min = *cdf.iter().find(|&&x| x > 0).unwrap_or(&0);
            let total_pixels = (width * height) as f32;
            
            // Create equalization lookup table
            let mut lut = vec![0u8; 256];
            for i in 0..256 {
                if cdf[i] > 0 {
                    let equalized = ((cdf[i] - cdf_min) as f32 / (total_pixels - cdf_min as f32) * 255.0).round();
                    lut[i] = equalized.max(0.0).min(255.0) as u8;
                }
            }
            
            // Apply equalization
            let mut result = ImageBuffer::new(width, height);
            for (x, y, pixel) in gray_img.enumerate_pixels() {
                let equalized = lut[pixel[0] as usize];
                result.put_pixel(x, y, Luma([equalized]));
            }
            
            Ok(DynamicImage::ImageLuma8(result))
        }
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            
            // Calculate histograms for each channel
            let mut hist_r = vec![0u32; 256];
            let mut hist_g = vec![0u32; 256];
            let mut hist_b = vec![0u32; 256];
            
            for pixel in rgb_img.pixels() {
                hist_r[pixel[0] as usize] += 1;
                hist_g[pixel[1] as usize] += 1;
                hist_b[pixel[2] as usize] += 1;
            }
            
            // Create equalization LUTs for each channel
            let lut_r = create_equalization_lut(&hist_r, width * height);
            let lut_g = create_equalization_lut(&hist_g, width * height);
            let lut_b = create_equalization_lut(&hist_b, width * height);
            
            // Apply equalization
            let mut result = ImageBuffer::new(width, height);
            for (x, y, pixel) in rgb_img.enumerate_pixels() {
                result.put_pixel(x, y, Rgb([
                    lut_r[pixel[0] as usize],
                    lut_g[pixel[1] as usize],
                    lut_b[pixel[2] as usize],
                ]));
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let (width, height) = rgba_img.dimensions();
            
            let mut hist_r = vec![0u32; 256];
            let mut hist_g = vec![0u32; 256];
            let mut hist_b = vec![0u32; 256];
            
            for pixel in rgba_img.pixels() {
                hist_r[pixel[0] as usize] += 1;
                hist_g[pixel[1] as usize] += 1;
                hist_b[pixel[2] as usize] += 1;
            }
            
            let lut_r = create_equalization_lut(&hist_r, width * height);
            let lut_g = create_equalization_lut(&hist_g, width * height);
            let lut_b = create_equalization_lut(&hist_b, width * height);
            
            let mut result = ImageBuffer::new(width, height);
            for (x, y, pixel) in rgba_img.enumerate_pixels() {
                result.put_pixel(x, y, Rgba([
                    lut_r[pixel[0] as usize],
                    lut_g[pixel[1] as usize],
                    lut_b[pixel[2] as usize],
                    pixel[3],
                ]));
            }
            
            Ok(DynamicImage::ImageRgba8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            histogram_equalization(&rgb_dynamic)
        }
    }
}

/// Helper function to create equalization lookup table
fn create_equalization_lut(histogram: &[u32], total_pixels: u32) -> Vec<u8> {
    let mut cdf = vec![0u32; 256];
    cdf[0] = histogram[0];
    for i in 1..256 {
        cdf[i] = cdf[i - 1] + histogram[i];
    }
    
    let cdf_min = *cdf.iter().find(|&&x| x > 0).unwrap_or(&0);
    let total = total_pixels as f32;
    
    let mut lut = vec![0u8; 256];
    for i in 0..256 {
        if cdf[i] > 0 {
            let equalized = ((cdf[i] - cdf_min) as f32 / (total - cdf_min as f32) * 255.0).round();
            lut[i] = equalized.max(0.0).min(255.0) as u8;
        }
    }
    
    lut
}

/// Apply adaptive histogram equalization (CLAHE - Contrast Limited Adaptive Histogram Equalization)
    #[allow(dead_code)]
pub fn adaptive_histogram_equalization(
    image: &DynamicImage,
    clip_limit: f32,
    tile_size: u32,
) -> Result<DynamicImage, ImgrsError> {
    let gray_img = image.to_luma8();
    let (width, height) = gray_img.dimensions();
    let mut result = ImageBuffer::new(width, height);
    
    let tiles_x = (width + tile_size - 1) / tile_size;
    let tiles_y = (height + tile_size - 1) / tile_size;
    
    // For each tile, calculate equalized histogram
    for tile_y in 0..tiles_y {
        for tile_x in 0..tiles_x {
            let start_x = tile_x * tile_size;
            let start_y = tile_y * tile_size;
            let end_x = ((tile_x + 1) * tile_size).min(width);
            let end_y = ((tile_y + 1) * tile_size).min(height);
            
            // Calculate histogram for this tile
            let mut hist = vec![0u32; 256];
            for y in start_y..end_y {
                for x in start_x..end_x {
                    let pixel = gray_img.get_pixel(x, y)[0];
                    hist[pixel as usize] += 1;
                }
            }
            
            // Apply contrast limiting
            let tile_pixels = ((end_x - start_x) * (end_y - start_y)) as f32;
            let clip_threshold = (clip_limit * tile_pixels / 256.0) as u32;
            
            let mut clipped = 0u32;
            for count in hist.iter_mut() {
                if *count > clip_threshold {
                    clipped += *count - clip_threshold;
                    *count = clip_threshold;
                }
            }
            
            // Redistribute clipped pixels
            let redistribution = clipped / 256;
            for count in hist.iter_mut() {
                *count += redistribution;
            }
            
            // Create LUT for this tile
            let tile_size_total = (end_x - start_x) * (end_y - start_y);
            let lut = create_equalization_lut(&hist, tile_size_total);
            
            // Apply to tile
            for y in start_y..end_y {
                for x in start_x..end_x {
                    let pixel = gray_img.get_pixel(x, y)[0];
                    result.put_pixel(x, y, Luma([lut[pixel as usize]]));
                }
            }
        }
    }
    
    Ok(DynamicImage::ImageLuma8(result))
}

/// Automatically adjust contrast to optimal levels
pub fn auto_contrast(image: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            
            // Find min and max for each channel
            let mut min_r = 255u8;
            let mut max_r = 0u8;
            let mut min_g = 255u8;
            let mut max_g = 0u8;
            let mut min_b = 255u8;
            let mut max_b = 0u8;
            
            for pixel in rgb_img.pixels() {
                min_r = min_r.min(pixel[0]);
                max_r = max_r.max(pixel[0]);
                min_g = min_g.min(pixel[1]);
                max_g = max_g.max(pixel[1]);
                min_b = min_b.min(pixel[2]);
                max_b = max_b.max(pixel[2]);
            }
            
            // Create stretch LUTs
            let lut_r = create_stretch_lut(min_r, max_r);
            let lut_g = create_stretch_lut(min_g, max_g);
            let lut_b = create_stretch_lut(min_b, max_b);
            
            // Apply stretching
            let mut result = ImageBuffer::new(width, height);
            for (x, y, pixel) in rgb_img.enumerate_pixels() {
                result.put_pixel(x, y, Rgb([
                    lut_r[pixel[0] as usize],
                    lut_g[pixel[1] as usize],
                    lut_b[pixel[2] as usize],
                ]));
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let (width, height) = rgba_img.dimensions();
            
            let mut min_r = 255u8;
            let mut max_r = 0u8;
            let mut min_g = 255u8;
            let mut max_g = 0u8;
            let mut min_b = 255u8;
            let mut max_b = 0u8;
            
            for pixel in rgba_img.pixels() {
                min_r = min_r.min(pixel[0]);
                max_r = max_r.max(pixel[0]);
                min_g = min_g.min(pixel[1]);
                max_g = max_g.max(pixel[1]);
                min_b = min_b.min(pixel[2]);
                max_b = max_b.max(pixel[2]);
            }
            
            let lut_r = create_stretch_lut(min_r, max_r);
            let lut_g = create_stretch_lut(min_g, max_g);
            let lut_b = create_stretch_lut(min_b, max_b);
            
            let mut result = ImageBuffer::new(width, height);
            for (x, y, pixel) in rgba_img.enumerate_pixels() {
                result.put_pixel(x, y, Rgba([
                    lut_r[pixel[0] as usize],
                    lut_g[pixel[1] as usize],
                    lut_b[pixel[2] as usize],
                    pixel[3],
                ]));
            }
            
            Ok(DynamicImage::ImageRgba8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            auto_contrast(&rgb_dynamic)
        }
    }
}

/// Create contrast stretch lookup table
fn create_stretch_lut(min: u8, max: u8) -> Vec<u8> {
    let mut lut = vec![0u8; 256];
    
    if min == max {
        return lut;
    }
    
    let range = (max - min) as f32;
    
    for i in 0..256 {
        if i < min as usize {
            lut[i] = 0;
        } else if i > max as usize {
            lut[i] = 255;
        } else {
            let normalized = (i as u8 - min) as f32 / range;
            lut[i] = (normalized * 255.0).round() as u8;
        }
    }
    
    lut
}

/// Automatically enhance image (auto-contrast + auto-brightness)
pub fn auto_enhance(image: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    // Apply auto-contrast first
    let contrasted = auto_contrast(image)?;
    
    // Then optimize brightness
    auto_brightness(&contrasted)
}

/// Automatically adjust brightness to optimal level
pub fn auto_brightness(image: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            
            // Calculate average brightness
            let mut total_brightness = 0u64;
            for pixel in rgb_img.pixels() {
                let brightness = (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3;
                total_brightness += brightness as u64;
            }
            
            let avg_brightness = (total_brightness / (width * height) as u64) as f32;
            let target_brightness = 128.0; // Target mid-range brightness
            let adjustment = target_brightness - avg_brightness;
            
            // Apply brightness adjustment
            let mut result = ImageBuffer::new(width, height);
            for (x, y, pixel) in rgb_img.enumerate_pixels() {
                let r = (pixel[0] as f32 + adjustment).max(0.0).min(255.0) as u8;
                let g = (pixel[1] as f32 + adjustment).max(0.0).min(255.0) as u8;
                let b = (pixel[2] as f32 + adjustment).max(0.0).min(255.0) as u8;
                result.put_pixel(x, y, Rgb([r, g, b]));
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            auto_brightness(&rgb_dynamic)
        }
    }
}

/// Adjust exposure (similar to camera exposure compensation)
pub fn exposure_adjust(image: &DynamicImage, exposure: f32) -> Result<DynamicImage, ImgrsError> {
    // Exposure adjustment using gamma correction
    // exposure > 0: increase exposure (brighten)
    // exposure < 0: decrease exposure (darken)
    // exposure = 0: no change
    
    let gamma = 2.0_f32.powf(-exposure);
    
    // Create lookup table
    let mut lut = vec![0u8; 256];
    for i in 0..256 {
        let normalized = i as f32 / 255.0;
        let adjusted = normalized.powf(gamma);
        lut[i] = (adjusted * 255.0).round().max(0.0).min(255.0) as u8;
    }
    
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for (x, y, pixel) in rgb_img.enumerate_pixels() {
                result.put_pixel(x, y, Rgb([
                    lut[pixel[0] as usize],
                    lut[pixel[1] as usize],
                    lut[pixel[2] as usize],
                ]));
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let (width, height) = rgba_img.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for (x, y, pixel) in rgba_img.enumerate_pixels() {
                result.put_pixel(x, y, Rgba([
                    lut[pixel[0] as usize],
                    lut[pixel[1] as usize],
                    lut[pixel[2] as usize],
                    pixel[3],
                ]));
            }
            
            Ok(DynamicImage::ImageRgba8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            exposure_adjust(&rgb_dynamic, exposure)
        }
    }
}

/// Auto-level: automatically adjust levels for optimal dynamic range
pub fn auto_level(image: &DynamicImage, black_clip: f32, white_clip: f32) -> Result<DynamicImage, ImgrsError> {
    // black_clip and white_clip are percentages (0.0-1.0) of pixels to clip
    
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let total_pixels = (width * height) as usize;
            
            // Collect all pixel values for each channel
            let mut r_values: Vec<u8> = Vec::with_capacity(total_pixels);
            let mut g_values: Vec<u8> = Vec::with_capacity(total_pixels);
            let mut b_values: Vec<u8> = Vec::with_capacity(total_pixels);
            
            for pixel in rgb_img.pixels() {
                r_values.push(pixel[0]);
                g_values.push(pixel[1]);
                b_values.push(pixel[2]);
            }
            
            // Sort to find percentile values
            r_values.sort_unstable();
            g_values.sort_unstable();
            b_values.sort_unstable();
            
            let black_idx = (total_pixels as f32 * black_clip) as usize;
            let white_idx = (total_pixels as f32 * (1.0 - white_clip)) as usize;
            
            let min_r = r_values[black_idx];
            let max_r = r_values[white_idx.min(total_pixels - 1)];
            let min_g = g_values[black_idx];
            let max_g = g_values[white_idx.min(total_pixels - 1)];
            let min_b = b_values[black_idx];
            let max_b = b_values[white_idx.min(total_pixels - 1)];
            
            // Create stretch LUTs with clipping
            let lut_r = create_stretch_lut(min_r, max_r);
            let lut_g = create_stretch_lut(min_g, max_g);
            let lut_b = create_stretch_lut(min_b, max_b);
            
            // Apply
            let mut result = ImageBuffer::new(width, height);
            for (x, y, pixel) in rgb_img.enumerate_pixels() {
                result.put_pixel(x, y, Rgb([
                    lut_r[pixel[0] as usize],
                    lut_g[pixel[1] as usize],
                    lut_b[pixel[2] as usize],
                ]));
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            auto_level(&rgb_dynamic, black_clip, white_clip)
        }
    }
}

/// Normalize image to full dynamic range
pub fn normalize(image: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    auto_level(image, 0.0, 0.0)
}

/// Automatically optimize contrast and brightness
    #[allow(dead_code)]
pub fn auto_optimize(image: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    // Step 1: Normalize to use full range
    let normalized = auto_level(image, 0.01, 0.01)?;
    
    // Step 2: Apply histogram equalization for better contrast
    let equalized = histogram_equalization(&normalized)?;
    
    // Step 3: Slight brightness adjustment
    let final_img = auto_brightness(&equalized)?;
    
    Ok(final_img)
}

/// Smart enhance - combination of techniques for natural-looking results
pub fn smart_enhance(image: &DynamicImage, strength: f32) -> Result<DynamicImage, ImgrsError> {
    // strength: 0.0 to 1.0
    let strength = strength.max(0.0).min(1.0);
    
    // Apply auto-contrast
    let contrasted = auto_contrast(image)?;
    
    // Blend with original based on strength
    match (image, &contrasted) {
        (DynamicImage::ImageRgb8(orig), DynamicImage::ImageRgb8(enhanced)) => {
            let (width, height) = orig.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for (x, y, orig_pixel) in orig.enumerate_pixels() {
                let enh_pixel = enhanced.get_pixel(x, y);
                
                let r = (orig_pixel[0] as f32 * (1.0 - strength) + enh_pixel[0] as f32 * strength) as u8;
                let g = (orig_pixel[1] as f32 * (1.0 - strength) + enh_pixel[1] as f32 * strength) as u8;
                let b = (orig_pixel[2] as f32 * (1.0 - strength) + enh_pixel[2] as f32 * strength) as u8;
                
                result.put_pixel(x, y, Rgb([r, g, b]));
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            smart_enhance(&rgb_dynamic, strength)
        }
    }
}

/// Auto white balance - automatically correct color temperature
pub fn auto_white_balance(image: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            
            // Calculate average for each channel
            let mut avg_r = 0u64;
            let mut avg_g = 0u64;
            let mut avg_b = 0u64;
            
            for pixel in rgb_img.pixels() {
                avg_r += pixel[0] as u64;
                avg_g += pixel[1] as u64;
                avg_b += pixel[2] as u64;
            }
            
            let total = (width * height) as f64;
            let avg_r = (avg_r as f64 / total) as f32;
            let avg_g = (avg_g as f64 / total) as f32;
            let avg_b = (avg_b as f64 / total) as f32;
            
            // Calculate gray world assumption adjustment
            let avg_gray = (avg_r + avg_g + avg_b) / 3.0;
            
            let scale_r = avg_gray / avg_r.max(1.0);
            let scale_g = avg_gray / avg_g.max(1.0);
            let scale_b = avg_gray / avg_b.max(1.0);
            
            // Apply white balance
            let mut result = ImageBuffer::new(width, height);
            for (x, y, pixel) in rgb_img.enumerate_pixels() {
                let r = (pixel[0] as f32 * scale_r).min(255.0) as u8;
                let g = (pixel[1] as f32 * scale_g).min(255.0) as u8;
                let b = (pixel[2] as f32 * scale_b).min(255.0) as u8;
                result.put_pixel(x, y, Rgb([r, g, b]));
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            auto_white_balance(&rgb_dynamic)
        }
    }
}

