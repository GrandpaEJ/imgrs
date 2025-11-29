use image::{DynamicImage, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, draw_filled_rect_mut};
use ab_glyph::{FontVec, PxScale, Font, ScaleFont};
use imageproc::rect::Rect;
use crate::errors::ImgrsError;
use super::styles::{TextStyle, TextAlign, TextAnchor, TextBoxStyle};
use super::fonts::{self};

/// Calculate anchor offset for text positioning
fn calculate_anchor_offset(
    text: &str,
    anchor: TextAnchor,
    size: f32,
    font: &FontVec,
) -> (i32, i32) {
    let (width, height, ascent, _) = match get_text_metrics(text, size, font) {
        Ok(metrics) => metrics,
        Err(_) => return (0, 0), // Default to no offset if measurement fails
    };
    
    match anchor {
        TextAnchor::TopLeft => (0, 0),
        TextAnchor::TopCenter => (-(width as i32 / 2), 0),
        TextAnchor::TopRight => (-(width as i32), 0),
        TextAnchor::MiddleLeft => (0, -(height as i32 / 2)),
        TextAnchor::MiddleCenter => (-(width as i32 / 2), -(height as i32 / 2)),
        TextAnchor::MiddleRight => (-(width as i32), -(height as i32 / 2)),
        TextAnchor::BottomLeft => (0, -(height as i32)),
        TextAnchor::BottomCenter => (-(width as i32 / 2), -(height as i32)),
        TextAnchor::BottomRight => (-(width as i32), -(height as i32)),
        TextAnchor::BaselineLeft => (0, -(ascent)),
        TextAnchor::BaselineCenter => (-(width as i32 / 2), -(ascent)),
        TextAnchor::BaselineRight => (-(width as i32), -(ascent)),
    }
}

/// Get text metrics for anchor calculations
fn get_text_metrics(
    text: &str,
    size: f32,
    font: &FontVec,
) -> Result<(u32, u32, i32, i32), ImgrsError> {
    let scale = PxScale::from(size);
    let scaled_font = font.as_scaled(scale);
    
    // Calculate width
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

/// Draw text on image with basic parameters
pub fn draw_text(
    image: &DynamicImage,
    text: &str,
    x: i32,
    y: i32,
    size: f32,
    color: (u8, u8, u8, u8),
    font_path: Option<&std::path::Path>,
    anchor: Option<TextAnchor>,
) -> Result<DynamicImage, ImgrsError> {
    let mut rgba_image = image.to_rgba8();
    let font = fonts::load_font(font_path)?;
    
    let scale = PxScale::from(size);
    let rgba_color = Rgba([color.0, color.1, color.2, color.3]);
    
    let (dx, dy) = if let Some(anchor) = anchor {
        calculate_anchor_offset(text, anchor, size, &font)
    } else {
        (0, 0)
    };
    
    draw_text_mut(&mut rgba_image, rgba_color, x + dx, y + dy, scale, &font, text);
    
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
    anchor: Option<TextAnchor>,
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
    
    // Calculate anchor offset
    let (dx, dy) = if let Some(anchor) = anchor {
        calculate_anchor_offset(text, anchor, style.size, &font)
    } else {
        (0, 0)
    };
    
    // Apply offset to coordinates
    let x = x + dx;
    let y = y + dy;
    
    // Handle rotation
    if style.rotation != 0.0 {
        // Measure text size
        let (width, height, _, _) = get_text_size(text, style.size, font_path)?;
        
        // Create temporary image for text
        // Add padding for shadow/outline if needed
        let padding = style.size as u32 / 2; // Heuristic padding
        let temp_width = width + padding * 2;
        let temp_height = height + padding * 2;
        
        let mut temp_img = RgbaImage::new(temp_width, temp_height);
        
        // Draw text on temp image (centered in padding)
        render_text_with_effects(
            &mut temp_img, 
            text, 
            padding as i32, 
            padding as i32, 
            style, 
            &font
        )?;
        
        // Rotate temp image
        use imageproc::geometric_transformations::{rotate_about_center, Interpolation};
        
        let radians = style.rotation.to_radians();
        let w = temp_width as f64;
        let h = temp_height as f64;
        let cos_a = (radians as f64).cos();
        let sin_a = (radians as f64).sin();
        
        // Calculate new dimensions
        let corners = [(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)];
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        
        for &(cx, cy) in &corners {
            let rx = cx * cos_a - cy * sin_a;
            let ry = cx * sin_a + cy * cos_a;
            min_x = min_x.min(rx);
            max_x = max_x.max(rx);
            min_y = min_y.min(ry);
            max_y = max_y.max(ry);
        }
        
        let new_width = (max_x - min_x).ceil() as u32;
        let new_height = (max_y - min_y).ceil() as u32;
        
        // Create expanded canvas and rotate
        let mut large_rgba = RgbaImage::new(new_width, new_height);
        let offset_x = ((new_width as f64 - w) / 2.0).round() as i64;
        let offset_y = ((new_height as f64 - h) / 2.0).round() as i64;
        image::imageops::overlay(&mut large_rgba, &temp_img, offset_x, offset_y);
        
        let rotated_rgba = rotate_about_center(
            &large_rgba,
            radians,
            Interpolation::Bilinear,
            Rgba([0, 0, 0, 0]),
        );
        
        // Calculate paste position
        // We want to align the center of the rotated text with the center of the original text box
        // Original text box center (relative to image):
        // ox = x + width/2
        // oy = y + height/2
        //
        // Rotated text box center (relative to rotated image):
        // rx = new_width/2
        // ry = new_height/2
        //
        // Paste position (top-left):
        // px = ox - rx
        // py = oy - ry
        //
        // However, we added padding.
        // Original content was at (x, y) but we drew it at (padding, padding) in temp_img.
        // So the "visual" center of text in temp_img is (padding + width/2, padding + height/2).
        //
        // Let's simplify:
        // We want the text to appear at (x, y) but rotated around its center.
        // Center of text: cx = x + width/2, cy = y + height/2.
        //
        // The rotated image has the text centered in it (because we centered temp_img in large_rgba).
        // So the center of rotated_rgba corresponds to the center of the text.
        //
        // So we just need to place rotated_rgba such that its center is at (cx, cy).
        // px = cx - new_width/2
        // py = cy - new_height/2
        
        let cx = x + (width as i32) / 2;
        let cy = y + (height as i32) / 2;
        
        let px = cx - (new_width as i32) / 2;
        let py = cy - (new_height as i32) / 2;
        
        image::imageops::overlay(&mut rgba_image, &rotated_rgba, px as i64, py as i64);
    } else {
        render_text_with_effects(&mut rgba_image, text, x, y, style, &font)?;
    }
    
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
    draw_text(image, text, x, y, size, color, None, None)
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
    
    // Pass None for anchor since we manually centered it
    draw_text_styled(image, text, x, y, style, font_path, None)
}

/// Draw text within a bounding box
pub fn draw_text_box(
    image: &DynamicImage,
    text: &str,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    style: &TextBoxStyle,
    font_path: Option<&std::path::Path>,
) -> Result<DynamicImage, ImgrsError> {
    let mut rgba_image = image.to_rgba8();
    let font = fonts::load_font(font_path)?;
    
    // Wrap text to fit width
    let wrapped_text = wrap_text(text, width, style.text_style.size, font_path)?;
    let lines: Vec<&str> = wrapped_text.lines().collect();
    
    // Calculate total text height
    let line_height = (style.text_style.size * style.text_style.line_spacing) as i32;
    let total_text_height = line_height * lines.len() as i32;
    
    // Calculate starting Y based on vertical alignment
    let start_y = match style.vertical_align {
        TextAlign::Left => y, // Top
        TextAlign::Center => y + (height as i32 - total_text_height) / 2, // Middle
        TextAlign::Right => y + height as i32 - total_text_height, // Bottom
    };
    
    // Draw each line
    for (i, line) in lines.iter().enumerate() {
        let line_y = start_y + (i as i32 * line_height);
        
        // Skip lines outside the box if overflow is hidden
        if !style.overflow && (line_y < y || line_y + line_height > y + height as i32) {
            continue;
        }
        
        // Calculate X based on horizontal alignment
        let line_width = measure_text_width(line, style.text_style.size, &font);
        let line_x = match style.text_style.align {
            TextAlign::Left => x,
            TextAlign::Center => x + (width as i32 - line_width) / 2,
            TextAlign::Right => x + width as i32 - line_width,
        };
        
        // Render line
        render_text_with_effects(
            &mut rgba_image,
            line,
            line_x,
            line_y,
            &style.text_style,
            &font,
        )?;
    }
    
    Ok(DynamicImage::ImageRgba8(rgba_image))
}