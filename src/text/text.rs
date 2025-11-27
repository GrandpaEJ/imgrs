use crate::errors::ImgrsError;
use image::{DynamicImage, GenericImageView, Rgb, Rgba};
use std::collections::HashMap;

/// Text styling options for advanced text rendering
#[derive(Debug, Clone)]
pub struct TextStyle {
    pub outline: Option<(u8, u8, u8, u8, f32)>, // (r, g, b, a, width)
    pub shadow: Option<(i32, i32, u8, u8, u8, u8)>, // (offset_x, offset_y, r, g, b, a)
    pub background: Option<(u8, u8, u8, u8)>, // (r, g, b, a)
}

/// Simple text rendering using a basic bitmap font (8x8 pixels per character)
pub fn draw_text(
    image: &DynamicImage,
    text: &str,
    x: i32,
    y: i32,
    color: (u8, u8, u8, u8),
    scale: u32,
) -> Result<DynamicImage, ImgrsError> {
    let mut result = image.clone();
    let char_width = 8 * scale;

    // Simple 8x8 bitmap font for basic ASCII characters (A-Z, 0-9)
    let font_data = get_basic_font_data();

    // Pre-calculate image bounds for efficiency
    let (img_width, img_height) = result.dimensions();

    for (i, ch) in text.chars().enumerate() {
        let char_x = x + (i as i32 * char_width as i32);

        if let Some(char_bitmap) = font_data.get(&ch) {
            for row in 0..8 {
                let row_bits = char_bitmap[row];
                if row_bits == 0 {
                    continue; // Skip empty rows
                }

                for col in 0..8 {
                    if (row_bits & (1 << (7 - col))) != 0 {
                        // Draw scaled pixel block
                        let base_px = char_x + col as i32 * scale as i32;
                        let base_py = y + row as i32 * scale as i32;

                        for sy in 0..scale {
                            let py = base_py + sy as i32;
                            if py < 0 || py >= img_height as i32 {
                                continue;
                            }

                            for sx in 0..scale {
                                let px = base_px + sx as i32;
                                if px >= 0 && px < img_width as i32 {
                                    draw_pixel_fast(&mut result, px as u32, py as u32, color);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(result)
}

/// Flexible text rendering with position tuple or separate x,y parameters
pub fn add_text(
    image: &DynamicImage,
    text: &str,
    position: (i32, i32),
    size: u32,
    color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    let (x, y) = position;
    draw_text(image, text, x, y, color, size)
}

/// Styled text with outline, shadow, and background support
pub fn add_text_styled(
    image: &DynamicImage,
    text: &str,
    position: (i32, i32),
    size: u32,
    color: (u8, u8, u8, u8),
    style: TextStyle,
) -> Result<DynamicImage, ImgrsError> {
    let (x, y) = position;
    let mut result = image.clone();

    // Draw background if specified
    if let Some(bg_color) = style.background {
        let (width, height) = get_text_size(text, size);
        draw_filled_rect_fast(&mut result, x, y, width as u32, height as u32, bg_color);
    }

    // Draw shadow if specified
    if let Some((offset_x, offset_y, sr, sg, sb, sa)) = style.shadow {
        draw_text_fast(&mut result, text, x + offset_x, y + offset_y, (sr, sg, sb, sa), size);
    }

    // Draw outline if specified - optimized single pass
    if let Some((or, og, ob, oa, width)) = style.outline {
        draw_text_outline_fast(&mut result, text, x, y, (or, og, ob, oa), size, width);
    }

    // Draw main text
    draw_text_fast(&mut result, text, x, y, color, size);

    Ok(result)
}

/// Multi-line text rendering with line spacing
pub fn add_text_multiline(
    image: &DynamicImage,
    text: &str,
    position: (i32, i32),
    size: u32,
    color: (u8, u8, u8, u8),
    line_spacing: f32,
) -> Result<DynamicImage, ImgrsError> {
    let (x, y) = position;
    let lines: Vec<&str> = text.split('\n').collect();
    let mut result = image.clone();

    for (i, line) in lines.iter().enumerate() {
        let line_y = y + (i as f32 * size as f32 * line_spacing) as i32;
        draw_text_fast(&mut result, line, x, line_y, color, size);
    }

    Ok(result)
}

/// Text dimension calculation
pub fn get_text_size(text: &str, size: u32) -> (u32, u32) {
    let char_width = 8 * size;
    let char_height = 8 * size;
    let width = text.len() as u32 * char_width;
    let height = char_height;
    (width, height)
}

/// Complete text bounding box information
pub fn get_text_box(text: &str, x: i32, y: i32, size: u32) -> TextBox {
    let (width, height) = get_text_size(text, size);
    TextBox {
        x,
        y,
        width: width as i32,
        height: height as i32,
        baseline_y: y + height as i32,
    }
}

/// Text bounding box structure
#[derive(Debug, Clone)]
pub struct TextBox {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub baseline_y: i32,
}

/// Fast in-place pixel drawing
fn draw_pixel_fast(image: &mut DynamicImage, x: u32, y: u32, color: (u8, u8, u8, u8)) {
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (img_width, img_height) = rgb_img.dimensions();
            if x < img_width && y < img_height {
                rgb_img.put_pixel(x, y, Rgb([color.0, color.1, color.2]));
            }
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let (img_width, img_height) = rgba_img.dimensions();
            if x < img_width && y < img_height {
                let alpha = color.3 as f32 / 255.0;
                let existing = rgba_img.get_pixel(x, y);

                let blended_r = ((1.0 - alpha) * existing[0] as f32 + alpha * color.0 as f32) as u8;
                let blended_g = ((1.0 - alpha) * existing[1] as f32 + alpha * color.1 as f32) as u8;
                let blended_b = ((1.0 - alpha) * existing[2] as f32 + alpha * color.2 as f32) as u8;
                let blended_a = ((1.0 - alpha) * existing[3] as f32 + alpha * 255.0) as u8;

                rgba_img.put_pixel(x, y, Rgba([blended_r, blended_g, blended_b, blended_a]));
            }
        }
        _ => {} // Ignore unsupported formats
    }
}

/// Fast in-place text drawing
fn draw_text_fast(image: &mut DynamicImage, text: &str, x: i32, y: i32, color: (u8, u8, u8, u8), scale: u32) {
    let char_width = 8 * scale;
    let font_data = get_basic_font_data();
    let (img_width, img_height) = image.dimensions();

    for (i, ch) in text.chars().enumerate() {
        let char_x = x + (i as i32 * char_width as i32);

        if let Some(char_bitmap) = font_data.get(&ch) {
            for row in 0..8 {
                let row_bits = char_bitmap[row];
                if row_bits == 0 {
                    continue;
                }

                for col in 0..8 {
                    if (row_bits & (1 << (7 - col))) != 0 {
                        let base_px = char_x + col as i32 * scale as i32;
                        let base_py = y + row as i32 * scale as i32;

                        for sy in 0..scale {
                            let py = base_py + sy as i32;
                            if py < 0 || py >= img_height as i32 {
                                continue;
                            }

                            for sx in 0..scale {
                                let px = base_px + sx as i32;
                                if px >= 0 && px < img_width as i32 {
                                    draw_pixel_fast(image, px as u32, py as u32, color);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Fast filled rectangle drawing
fn draw_filled_rect_fast(image: &mut DynamicImage, x: i32, y: i32, width: u32, height: u32, color: (u8, u8, u8, u8)) {
    let (img_width, img_height) = image.dimensions();

    let start_x = x.max(0) as u32;
    let start_y = y.max(0) as u32;
    let end_x = (x + width as i32).min(img_width as i32) as u32;
    let end_y = (y + height as i32).min(img_height as i32) as u32;

    for py in start_y..end_y {
        for px in start_x..end_x {
            draw_pixel_fast(image, px, py, color);
        }
    }
}

/// Fast outline text drawing - single pass optimization
fn draw_text_outline_fast(image: &mut DynamicImage, text: &str, x: i32, y: i32, color: (u8, u8, u8, u8), scale: u32, width: f32) {
    let char_width = 8 * scale;
    let font_data = get_basic_font_data();
    let (img_width, img_height) = image.dimensions();
    let outline_radius = width.ceil() as i32;

    // Collect all text pixel positions
    let mut text_pixels = Vec::new();

    for (i, ch) in text.chars().enumerate() {
        let char_x = x + (i as i32 * char_width as i32);

        if let Some(char_bitmap) = font_data.get(&ch) {
            for row in 0..8 {
                let row_bits = char_bitmap[row];
                if row_bits == 0 {
                    continue;
                }

                for col in 0..8 {
                    if (row_bits & (1 << (7 - col))) != 0 {
                        let base_px = char_x + col as i32 * scale as i32;
                        let base_py = y + row as i32 * scale as i32;

                        for sy in 0..scale {
                            for sx in 0..scale {
                                let px = base_px + sx as i32;
                                let py = base_py + sy as i32;
                                if px >= 0 && px < img_width as i32 && py >= 0 && py < img_height as i32 {
                                    text_pixels.push((px as u32, py as u32));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Draw outline by expanding each text pixel
    for (px, py) in text_pixels {
        for dy in -outline_radius..=outline_radius {
            for dx in -outline_radius..=outline_radius {
                let distance = ((dx * dx + dy * dy) as f32).sqrt();
                if distance <= width {
                    let outline_x = px as i32 + dx;
                    let outline_y = py as i32 + dy;
                    if outline_x >= 0 && outline_x < img_width as i32 &&
                       outline_y >= 0 && outline_y < img_height as i32 {
                        draw_pixel_fast(image, outline_x as u32, outline_y as u32, color);
                    }
                }
            }
        }
    }
}

/// Get basic font data for simple text rendering
fn get_basic_font_data() -> HashMap<char, [u8; 8]> {
    let mut font = HashMap::new();

    // Basic 8x8 bitmap font data for some characters
    font.insert('A', [0x18, 0x3C, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00]);
    font.insert('B', [0x7C, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x7C, 0x00]);
    font.insert('C', [0x3C, 0x66, 0x60, 0x60, 0x60, 0x66, 0x3C, 0x00]);
    font.insert('D', [0x78, 0x6C, 0x66, 0x66, 0x66, 0x6C, 0x78, 0x00]);
    font.insert('E', [0x7E, 0x60, 0x60, 0x78, 0x60, 0x60, 0x7E, 0x00]);
    font.insert('F', [0x7E, 0x60, 0x60, 0x78, 0x60, 0x60, 0x60, 0x00]);
    font.insert('G', [0x3C, 0x66, 0x60, 0x6E, 0x66, 0x66, 0x3C, 0x00]);
    font.insert('H', [0x66, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00]);
    font.insert('I', [0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00]);
    font.insert('J', [0x1E, 0x0C, 0x0C, 0x0C, 0x0C, 0x6C, 0x38, 0x00]);
    font.insert('K', [0x66, 0x6C, 0x78, 0x70, 0x78, 0x6C, 0x66, 0x00]);
    font.insert('L', [0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x7E, 0x00]);
    font.insert('M', [0x63, 0x77, 0x7F, 0x6B, 0x63, 0x63, 0x63, 0x00]);
    font.insert('N', [0x66, 0x76, 0x7E, 0x7E, 0x6E, 0x66, 0x66, 0x00]);
    font.insert('O', [0x3C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00]);
    font.insert('P', [0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60, 0x60, 0x00]);
    font.insert('Q', [0x3C, 0x66, 0x66, 0x66, 0x6E, 0x66, 0x3E, 0x00]);
    font.insert('R', [0x7C, 0x66, 0x66, 0x7C, 0x78, 0x6C, 0x66, 0x00]);
    font.insert('S', [0x3C, 0x66, 0x60, 0x3C, 0x06, 0x66, 0x3C, 0x00]);
    font.insert('T', [0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00]);
    font.insert('U', [0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00]);
    font.insert('V', [0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00]);
    font.insert('W', [0x63, 0x63, 0x63, 0x6B, 0x7F, 0x77, 0x63, 0x00]);
    font.insert('X', [0x66, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x66, 0x00]);
    font.insert('Y', [0x66, 0x66, 0x66, 0x3C, 0x18, 0x18, 0x18, 0x00]);
    font.insert('Z', [0x7E, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x7E, 0x00]);
    font.insert('0', [0x3C, 0x66, 0x6E, 0x76, 0x66, 0x66, 0x3C, 0x00]);
    font.insert('1', [0x18, 0x18, 0x38, 0x18, 0x18, 0x18, 0x7E, 0x00]);
    font.insert('2', [0x3C, 0x66, 0x06, 0x0C, 0x30, 0x60, 0x7E, 0x00]);
    font.insert('3', [0x3C, 0x66, 0x06, 0x1C, 0x06, 0x66, 0x3C, 0x00]);
    font.insert('4', [0x06, 0x0E, 0x1E, 0x66, 0x7F, 0x06, 0x06, 0x00]);
    font.insert('5', [0x7E, 0x60, 0x7C, 0x06, 0x06, 0x66, 0x3C, 0x00]);
    font.insert('6', [0x3C, 0x66, 0x60, 0x7C, 0x66, 0x66, 0x3C, 0x00]);
    font.insert('7', [0x7E, 0x06, 0x0C, 0x18, 0x30, 0x30, 0x30, 0x00]);
    font.insert('8', [0x3C, 0x66, 0x66, 0x3C, 0x66, 0x66, 0x3C, 0x00]);
    font.insert('9', [0x3C, 0x66, 0x66, 0x3E, 0x06, 0x66, 0x3C, 0x00]);
    font.insert(' ', [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    font.insert('!', [0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x18, 0x00]);
    font.insert('?', [0x3C, 0x66, 0x06, 0x0C, 0x18, 0x00, 0x18, 0x00]);
    font.insert('.', [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x00]);
    font.insert(',', [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x30, 0x00]);

    font
}