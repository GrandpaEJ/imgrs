// Color analysis and palette extraction
use crate::errors::ImgrsError;
use image::Rgba; // DynamicImage, ImageBuffer, GenericImageView};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

impl crate::image::core::PyImage {
    pub fn get_color_palette_impl(
        &mut self,
        max_colors: u32,
    ) -> Result<Vec<(u8, u8, u8, u8)>, ImgrsError> {
        let image = self.get_image()?;
        let rgba_image = image.to_rgba8();
        let (width, height) = rgba_image.dimensions();

        // Collect all colors with their frequency
        let mut color_counts: HashMap<(u8, u8, u8, u8), u32> = HashMap::new();

        for y in 0..height {
            for x in 0..width {
                let pixel = rgba_image.get_pixel(x, y);
                *color_counts
                    .entry((pixel[0], pixel[1], pixel[2], pixel[3]))
                    .or_insert(0) += 1;
            }
        }

        // Sort by frequency and take top colors
        let mut sorted_colors: Vec<((u8, u8, u8, u8), u32)> = color_counts
            .into_iter()
            .collect();

        sorted_colors.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count descending

        let palette = sorted_colors
            .into_iter()
            .take(max_colors as usize)
            .map(|(color, _)| color)
            .collect();

        Ok(palette)
    }

    pub fn analyze_color_distribution_impl(
        &mut self,
    ) -> Result<Py<pyo3::types::PyDict>, ImgrsError> {
        let image = self.get_image()?;
        let rgba_image = image.to_rgba8();
        let (width, height) = rgba_image.dimensions();

        let mut total_pixels = 0;
        let mut color_counts: HashMap<(u8, u8, u8, u8), u32> = HashMap::new();
        let mut hue_histogram: HashMap<u32, u32> = HashMap::new();
        let mut saturation_histogram: HashMap<u32, u32> = HashMap::new();
        let mut brightness_histogram: HashMap<u32, u32> = HashMap::new();

        // First pass: collect color data
        for y in 0..height {
            for x in 0..width {
                let pixel = rgba_image.get_pixel(x, y);
                *color_counts
                    .entry((pixel[0], pixel[1], pixel[2], pixel[3]))
                    .or_insert(0) += 1;
                total_pixels += 1;

                // Convert to HSB for histograms
                let (h, s, b) = rgb_to_hsb(pixel[0], pixel[1], pixel[2]);
                *hue_histogram.entry(h).or_insert(0) += 1;
                *saturation_histogram.entry(s).or_insert(0) += 1;
                *brightness_histogram.entry(b).or_insert(0) += 1;
            }
        }

        let unique_colors = color_counts.len();

        // Find dominant color
        let dominant_color = color_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(color, _)| color)
            .unwrap_or((0, 0, 0, 0));

        // Calculate average color
        let mut avg_r = 0.0;
        let mut avg_g = 0.0;
        let mut avg_b = 0.0;
        let mut avg_a = 0.0;

        for y in 0..height {
            for x in 0..width {
                let pixel = rgba_image.get_pixel(x, y);
                avg_r += pixel[0] as f32;
                avg_g += pixel[1] as f32;
                avg_b += pixel[2] as f32;
                avg_a += pixel[3] as f32;
            }
        }

        let avg_color = if total_pixels > 0 {
            (
                (avg_r / total_pixels as f32) as u8,
                (avg_g / total_pixels as f32) as u8,
                (avg_b / total_pixels as f32) as u8,
                (avg_a / total_pixels as f32) as u8,
            )
        } else {
            (0, 0, 0, 0)
        };

        // Create Python dictionary with results
        Python::with_gil(|py| {
            let dict = PyDict::new(py);

            // Basic statistics
            dict.set_item("total_pixels", total_pixels)?;
            dict.set_item("width", width)?;
            dict.set_item("height", height)?;
            dict.set_item("dominant_color", dominant_color)?;
            dict.set_item("average_color", avg_color)?;

            // Color space distributions
            dict.set_item("unique_colors", unique_colors)?;
            dict.set_item("hue_distribution", hsb_histogram_to_list(&hue_histogram))?;
            dict.set_item(
                "saturation_distribution",
                hsb_histogram_to_list(&saturation_histogram),
            )?;
            dict.set_item(
                "brightness_distribution",
                hsb_histogram_to_list(&brightness_histogram),
            )?;

            Ok(dict.unbind())
        })
    }

    pub fn find_color_regions_impl(
        &mut self,
        target_color: (u8, u8, u8),
        tolerance: u8,
    ) -> Result<Vec<(u32, u32, u32, u32)>, ImgrsError> {
        let image = self.get_image()?;
        let rgba_image = image.to_rgba8();
        let (width, height) = rgba_image.dimensions();

        let mut visited = vec![vec![false; width as usize]; height as usize];
        let mut regions = Vec::new();

        for y in 0..height {
            for x in 0..width {
                if !visited[y as usize][x as usize] {
                    let pixel = rgba_image.get_pixel(x, y);
                    let distance = color_distance((pixel[0], pixel[1], pixel[2]), target_color);

                    if distance <= tolerance as f32 {
                        // Found a region, perform flood fill
                        let region = flood_fill_region(
                            &rgba_image,
                            &mut visited,
                            x,
                            y,
                            target_color,
                            tolerance,
                        );

                        if region.pixels.len() > 10 {
                            // Only include regions with more than 10 pixels
                            regions.push((
                                region.min_x,
                                region.min_y,
                                region.max_x - region.min_x,
                                region.max_y - region.min_y,
                            ));
                        }
                    }
                }
            }
        }

        Ok(regions)
    }
}

// Helper structures and functions
#[derive(Clone)]
struct Region {
    min_x: u32,
    min_y: u32,
    max_x: u32,
    max_y: u32,
    pixels: Vec<(u32, u32)>,
}

fn flood_fill_region(
    image: &image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    visited: &mut Vec<Vec<bool>>,
    start_x: u32,
    start_y: u32,
    target_color: (u8, u8, u8),
    tolerance: u8,
) -> Region {
    let mut region = Region {
        min_x: start_x,
        min_y: start_y,
        max_x: start_x,
        max_y: start_y,
        pixels: Vec::new(),
    };

    let width = image.width();
    let height = image.height();
    let mut stack = vec![(start_x, start_y)];

    while let Some((x, y)) = stack.pop() {
        if x >= width || y >= height || visited[y as usize][x as usize] {
            continue;
        }

        let pixel = image.get_pixel(x, y);
        let distance = color_distance((pixel[0], pixel[1], pixel[2]), target_color);

        if distance > tolerance as f32 {
            continue;
        }

        visited[y as usize][x as usize] = true;
        region.pixels.push((x, y));
        region.min_x = region.min_x.min(x);
        region.min_y = region.min_y.min(y);
        region.max_x = region.max_x.max(x);
        region.max_y = region.max_y.max(y);

        // Add adjacent pixels to stack
        if x > 0 {
            stack.push((x - 1, y));
        }
        if x < width - 1 {
            stack.push((x + 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if y < height - 1 {
            stack.push((x, y + 1));
        }
    }

    region
}

fn hsb_histogram_to_list(histogram: &HashMap<u32, u32>) -> Vec<(u32, u32)> {
    let mut result: Vec<_> = histogram.iter().map(|(&k, &v)| (k, v)).collect();
    result.sort_by_key(|&(key, _)| key);
    result
}

fn rgb_to_hsb(r: u8, g: u8, b: u8) -> (u32, u32, u32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let diff = max - min;

    // Brightness
    let brightness = (max * 255.0) as u32;

    // Saturation
    let saturation = if max == 0.0 {
        0
    } else {
        ((diff / max) * 255.0) as u32
    };

    // Hue
    let hue = if diff == 0.0 {
        0
    } else {
        match max {
            v if v == r => ((g - b) / diff * 60.0) as u32,
            v if v == g => ((b - r) / diff * 60.0 + 120.0) as u32,
            _ => ((r - g) / diff * 60.0 + 240.0) as u32,
        }
    };

    (hue % 360, saturation, brightness)
}

// Utility functions (already defined in color_ops.rs but need to be public)
pub fn color_distance(color1: (u8, u8, u8), color2: (u8, u8, u8)) -> f32 {
    let dr = color1.0 as f32 - color2.0 as f32;
    let dg = color1.1 as f32 - color2.1 as f32;
    let db = color1.2 as f32 - color2.2 as f32;
    (dr * dr + dg * dg + db * db).sqrt()
}
