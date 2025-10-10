/// Text rendering implementation with full styling support

use image::{DynamicImage, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, draw_filled_rect_mut};
use ab_glyph::{FontVec, PxScale, Font, ScaleFont};
use imageproc::rect::Rect;
use crate::errors::ImgrsError;
use super::styles::{TextStyle, TextAlign};
use super::fonts::{self};

/// Draw text on image with basic parameters
pub fn draw_text(
    image: &DynamicImage,
    text: &str,
    x: i32,
    y: i32,
    size: f32,
    color: (u8, u8, u8, u8),
    font_path: Option<&std::path::Path>,
) -> Result<DynamicImage, ImgrsError> {
    let mut rgba_image = image.to_rgba8();
    let font = fonts::load_font(font_path)?;
    
    let scale = PxScale::from(size);
    let rgba_color = Rgba([color.0, color.1, color.2, color.3]);
    
    draw_text_mut(&mut rgba_image, rgba_color, x, y, scale, &font, text);
    
    Ok(DynamicImage::ImageRgba8(rgba_image))
}

/// Draw multi-line text
pub fn draw_text_multiline(
    image: &DynamicImage,
    text: &str,
    x: i32,
    y: i32,
    style: &TextStyle,
    font_path: Option<&std::path::Path>,
) -> Result<DynamicImage, ImgrsError> {
    let mut rgba_image = image.to_rgba8();
    let font = fonts::load_font(font_path)?;
    
    let lines: Vec<&str> = text.lines().collect();
    let line_height = (style.size * style.line_spacing) as i32;
    
    for (i, line) in lines.iter().enumerate() {
        let line_y = y + (i as i32 * line_height);
        
        // Calculate x position based on alignment
        let line_x = match style.align {
            TextAlign::Left => x,
            TextAlign::Center => {
                let text_width = measure_text_width(line, style.size, &font);
                x - (text_width / 2)
            }
            TextAlign::Right => {
                let text_width = measure_text_width(line, style.size, &font);
                x - text_width
            }
        };
        
        render_text_with_effects(&mut rgba_image, line, line_x, line_y, style, &font)?;
    }
    
    Ok(DynamicImage::ImageRgba8(rgba_image))
}

/// Draw text with full styling support
pub fn draw_text_styled(
    image: &DynamicImage,
    text: &str,
    x: i32,
    y: i32,
    style: &TextStyle,
    font_path: Option<&std::path::Path>,
) -> Result<DynamicImage, ImgrsError> {
    // Handle multiline text
    if text.contains('\n') || style.max_width.is_some() {
        let wrapped_text = if let Some(max_width) = style.max_width {
            wrap_text(text, max_width, style.size, font_path)?
        } else {
            text.to_string()
        };
        return draw_text_multiline(image, &wrapped_text, x, y, style, font_path);
    }
    
    let mut rgba_image = image.to_rgba8();
    let font = fonts::load_font(font_path)?;
    
    render_text_with_effects(&mut rgba_image, text, x, y, style, &font)?;
    
    Ok(DynamicImage::ImageRgba8(rgba_image))
}

/// Render text with all effects (shadow, background, outline)
fn render_text_with_effects(
    target: &mut RgbaImage,
    text: &str,
    x: i32,
    y: i32,
    style: &TextStyle,
    font: &FontVec,
) -> Result<(), ImgrsError> {
    let scale = PxScale::from(style.size);
    
    // Draw background if specified
    if let Some((br, bg, bb, ba)) = style.background {
        let text_width = measure_text_width(text, style.size, font);
        let text_height = style.size as i32;
        
        if x >= 0 && y >= 0 {
            let rect = Rect::at(x, y).of_size(text_width as u32, text_height as u32);
            draw_filled_rect_mut(target, rect, Rgba([br, bg, bb, ba]));
        }
    }
    
    // Draw shadow if specified
    if let Some((sx, sy, sr, sg, sb, sa)) = style.shadow {
        let shadow_color = Rgba([sr, sg, sb, sa]);
        draw_text_mut(target, shadow_color, x + sx, y + sy, scale, font, text);
    }
    
    // Draw outline if specified
    if let Some((or, og, ob, oa, width)) = style.outline {
        let outline_color = Rgba([or, og, ob, oa]);
        // Draw text multiple times around the position for outline effect
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx != 0 || dy != 0 {
                    let offset = (width * 0.5) as i32;
                    draw_text_mut(
                        target,
                        outline_color,
                        x + dx * offset,
                        y + dy * offset,
                        scale,
                        font,
                        text,
                    );
                }
            }
        }
    }
    
    // Draw main text with opacity
    let final_alpha = (style.color.3 as f32 * style.opacity).min(255.0) as u8;
    let text_color = Rgba([style.color.0, style.color.1, style.color.2, final_alpha]);
    
    draw_text_mut(target, text_color, x, y, scale, font, text);
    
    Ok(())
}

/// Measure text width in pixels
fn measure_text_width(text: &str, size: f32, font: &FontVec) -> i32 {
    let scale = PxScale::from(size);
    let scaled_font = font.as_scaled(scale);
    
    let mut width = 0.0;
    for c in text.chars() {
        let glyph = scaled_font.scaled_glyph(c);
        width += scaled_font.h_advance(glyph.id);
    }
    
    width as i32
}

/// Get text bounding box dimensions
/// Returns (width, height, ascent, descent)
pub fn get_text_size(
    text: &str,
    size: f32,
    font_path: Option<&std::path::Path>,
) -> Result<(u32, u32, i32, i32), ImgrsError> {
    let font = fonts::load_font(font_path)?;
    let scale = PxScale::from(size);
    let scaled_font = font.as_scaled(scale);
    
    // Measure width
    let mut width = 0.0_f32;
    let mut max_height = 0.0_f32;
    let mut min_y = 0.0_f32;
    let mut max_y = 0.0_f32;
    
    for c in text.chars() {
        let glyph = scaled_font.scaled_glyph(c);
        width += scaled_font.h_advance(glyph.id);
        
        // Get glyph bounds for height calculation
        if let Some(outlined) = scaled_font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            min_y = min_y.min(bounds.min.y);
            max_y = max_y.max(bounds.max.y);
            max_height = max_height.max(bounds.height());
        }
    }
    
    let height = (max_y - min_y).max(size);
    let ascent = (-min_y) as i32;
    let descent = max_y as i32;
    
    Ok((width as u32, height as u32, ascent, descent))
}

/// Get multiline text bounding box
/// Returns (width, height, line_count)
pub fn get_multiline_text_size(
    text: &str,
    size: f32,
    line_spacing: f32,
    font_path: Option<&std::path::Path>,
) -> Result<(u32, u32, usize), ImgrsError> {
    let font = fonts::load_font(font_path)?;
    let lines: Vec<&str> = text.lines().collect();
    let line_count = lines.len();
    
    if line_count == 0 {
        return Ok((0, 0, 0));
    }
    
    let mut max_width = 0;
    for line in &lines {
        let line_width = measure_text_width(line, size, &font);
        max_width = max_width.max(line_width);
    }
    
    let line_height = (size * line_spacing) as u32;
    let total_height = line_height * (line_count as u32);
    
    Ok((max_width as u32, total_height, line_count))
}

/// Get text bounding box with all details
/// Returns a TextBox struct with comprehensive information
pub fn get_text_box(
    text: &str,
    x: i32,
    y: i32,
    size: f32,
    font_path: Option<&std::path::Path>,
) -> Result<TextBox, ImgrsError> {
    let (width, height, ascent, descent) = get_text_size(text, size, font_path)?;
    
    Ok(TextBox {
        x,
        y,
        width,
        height,
        ascent,
        descent,
        baseline_y: y + ascent,
        bottom_y: y + height as i32,
        right_x: x + width as i32,
    })
}

/// Text bounding box information
#[derive(Debug, Clone)]
pub struct TextBox {
    /// X coordinate (left)
    pub x: i32,
    /// Y coordinate (top)
    pub y: i32,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Ascent (distance from baseline to top)
    pub ascent: i32,
    /// Descent (distance from baseline to bottom)
    pub descent: i32,
    /// Y coordinate of baseline
    pub baseline_y: i32,
    /// Y coordinate of bottom edge
    pub bottom_y: i32,
    /// X coordinate of right edge
    pub right_x: i32,
}

/// Wrap text to fit within max width
fn wrap_text(
    text: &str,
    max_width: u32,
    size: f32,
    font_path: Option<&std::path::Path>,
) -> Result<String, ImgrsError> {
    let font = fonts::load_font(font_path)?;
    let mut result = String::new();
    let mut current_line = String::new();
    
    for word in text.split_whitespace() {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };
        
        let width = measure_text_width(&test_line, size, &font);
        
        if width <= max_width as i32 {
            current_line = test_line;
        } else {
            if !current_line.is_empty() {
                result.push_str(&current_line);
                result.push('\n');
            }
            current_line = word.to_string();
        }
    }
    
    if !current_line.is_empty() {
        result.push_str(&current_line);
    }
    
    Ok(result)
}

/// Quick text rendering with minimal parameters
    #[allow(dead_code)]
pub fn draw_text_quick(
    image: &DynamicImage,
    text: &str,
    x: i32,
    y: i32,
    size: f32,
    color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    draw_text(image, text, x, y, size, color, None)
}

/// Draw text with automatic positioning (center)
pub fn draw_text_centered(
    image: &DynamicImage,
    text: &str,
    y: i32,
    style: &TextStyle,
    font_path: Option<&std::path::Path>,
) -> Result<DynamicImage, ImgrsError> {
    let font = fonts::load_font(font_path)?;
    let text_width = measure_text_width(text, style.size, &font);
    let x = (image.width() as i32 - text_width) / 2;
    
    draw_text_styled(image, text, x, y, style, font_path)
}

