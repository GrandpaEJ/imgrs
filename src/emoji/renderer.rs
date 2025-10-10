/// High-performance emoji renderer
/// Creates visual emoji representations with colors and shapes

use image::{DynamicImage, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_text_mut};
use ab_glyph::{FontVec, PxScale};
use crate::errors::ImgrsError;
use super::presets::EmojiType;

/// Embedded DejaVu Sans font for text labels
const FONT_DATA: &[u8] = include_bytes!("../../fonts/DejaVuSans.ttf");

/// Style options for emoji rendering
#[derive(Debug, Clone)]
pub struct EmojiStyle {
    pub size: u32,
    pub x: i32,
    pub y: i32,
    pub opacity: f32,
    #[allow(dead_code)]
    pub background: Option<(u8, u8, u8, u8)>,
    pub color: Option<(u8, u8, u8, u8)>,
}

impl Default for EmojiStyle {
    fn default() -> Self {
        EmojiStyle {
            size: 64,
            x: 0,
            y: 0,
            opacity: 1.0,
            background: None,
            color: None,
        }
    }
}

/// Add emoji to image using preset emoji type
pub fn add_emoji(
    image: &DynamicImage,
    emoji_type: EmojiType,
    style: EmojiStyle,
) -> Result<DynamicImage, ImgrsError> {
    add_emoji_text(image, emoji_type.as_str(), style)
}

/// Add emoji to image using raw emoji text (Unicode)
pub fn add_emoji_text(
    image: &DynamicImage,
    emoji: &str,
    style: EmojiStyle,
) -> Result<DynamicImage, ImgrsError> {
    let mut rgba_image = image.to_rgba8();
    
    // Get emoji color
    let emoji_color = match style.color {
        Some(c) => c,
        None => {
            let (r, g, b) = get_emoji_color(emoji);
            (r, g, b, (255.0 * style.opacity) as u8)
        }
    };
    
    // Render emoji as a colored circle with the emoji character
    render_emoji_visual(&mut rgba_image, emoji, emoji_color, &style)?;
    
    Ok(DynamicImage::ImageRgba8(rgba_image))
}

/// Render emoji as a visual element (circle + character)
fn render_emoji_visual(
    target: &mut RgbaImage,
    emoji: &str,
    color: (u8, u8, u8, u8),
    style: &EmojiStyle,
) -> Result<(), ImgrsError> {
    let (width, height) = target.dimensions();
    let center_x = style.x + (style.size as i32 / 2);
    let center_y = style.y + (style.size as i32 / 2);
    let radius = (style.size / 2) as i32;
    
    // Check if emoji is within bounds
    if center_x < 0 || center_y < 0 || 
       center_x >= width as i32 || center_y >= height as i32 {
        return Ok(());
    }
    
    // Draw background circle
    let bg_color = Rgba([color.0, color.1, color.2, color.3]);
    draw_filled_circle_mut(
        target,
        (center_x, center_y),
        radius,
        bg_color,
    );
    
    // Add highlight for 3D effect
    let highlight_radius = (radius as f32 * 0.4) as i32;
    let highlight_y = center_y - (radius as f32 * 0.25) as i32;
    let highlight_color = Rgba([
        color.0.saturating_add(50),
        color.1.saturating_add(50),
        color.2.saturating_add(50),
        (color.3 as f32 * 0.6) as u8,
    ]);
    draw_filled_circle_mut(
        target,
        (center_x, highlight_y),
        highlight_radius,
        highlight_color,
    );
    
    // Try to render the emoji character using font
    if let Ok(font) = FontVec::try_from_vec(FONT_DATA.to_vec()) {
        let scale = PxScale::from(style.size as f32 * 0.8);
        let text_color = Rgba([255, 255, 255, color.3]);
        
        // Calculate text position (centered)
        let text_x = style.x + (style.size as i32 / 8);
        let text_y = style.y + (style.size as i32 / 8);
        
        draw_text_mut(
            target,
            text_color,
            text_x,
            text_y,
            scale,
            &font,
            emoji,
        );
    }
    
    Ok(())
}

/// Get representative color for emoji type
fn get_emoji_color(emoji: &str) -> (u8, u8, u8) {
    match emoji {
        // Smileys - Yellow
        "😊" | "😁" | "😂" | "🤣" | "😉" | "😎" | "😇" => (255, 220, 50),
        "😍" | "🤩" => (255, 180, 120),
        "🤔" => (255, 200, 80),
        
        // Hearts - Colors
        "❤️" | "💔" => (255, 60, 60),
        "💙" => (60, 120, 255),
        "💚" => (60, 220, 120),
        "💛" => (255, 220, 50),
        "💜" => (180, 80, 255),
        "🧡" => (255, 140, 50),
        "🖤" => (60, 60, 60),
        "🤍" => (240, 240, 240),
        "💖" => (255, 120, 180),
        
        // Gestures - Skin tones
        "👍" | "👌" | "✌️" | "🙌" | "👏" | "👋" => (255, 200, 150),
        "👎" => (255, 180, 140),
        "👉" | "👈" => (255, 190, 145),
        "🔥" => (255, 80, 30),
        
        // Nature
        "☀️" => (255, 200, 30),
        "🌙" => (220, 220, 240),
        "⭐" | "✨" => (255, 220, 50),
        "☁️" => (230, 230, 245),
        "🌈" => (180, 120, 255),
        "🌸" | "🌹" => (255, 140, 200),
        "🌲" | "🍃" => (80, 200, 100),
        
        // Food
        "🍕" => (255, 180, 80),
        "🍔" => (200, 140, 90),
        "🎂" => (255, 190, 210),
        "🍦" => (255, 230, 230),
        "☕" | "🍺" => (140, 90, 50),
        "🍎" => (220, 40, 40),
        "🍬" | "🍪" | "🍩" => (255, 180, 140),
        
        // Activities
        "⚽" | "🏀" => (255, 140, 40),
        "🎉" | "🎁" => (255, 80, 140),
        "🏆" | "🥇" => (255, 200, 0),
        "📷" => (80, 80, 80),
        "🎵" | "🎨" | "🎮" => (140, 80, 255),
        
        // Symbols
        "✅" => (80, 255, 80),
        "❌" => (255, 80, 80),
        "❓" | "❗" => (80, 140, 255),
        "⚠️" => (255, 180, 0),
        "🚫" => (255, 40, 40),
        "♻️" => (80, 200, 80),
        "⚛️" => (80, 140, 255),
        "➡️" => (80, 80, 255),
        
        _ => (255, 200, 80), // Default yellow
    }
}

/// Add multiple emojis to an image
pub fn add_emojis_batch(
    image: &DynamicImage,
    emojis: Vec<(EmojiType, EmojiStyle)>,
) -> Result<DynamicImage, ImgrsError> {
    let mut result = image.to_rgba8();
    
    for (emoji_type, style) in emojis {
        let emoji_str = emoji_type.as_str();
        let emoji_color = match style.color {
            Some(c) => c,
            None => {
                let (r, g, b) = get_emoji_color(emoji_str);
                (r, g, b, (255.0 * style.opacity) as u8)
            }
        };
        
        render_emoji_visual(&mut result, emoji_str, emoji_color, &style)?;
    }
    
    Ok(DynamicImage::ImageRgba8(result))
}

/// Quick emoji add with minimal parameters
    #[allow(dead_code)]
pub fn add_emoji_quick(
    image: &DynamicImage,
    emoji_type: EmojiType,
    x: i32,
    y: i32,
    size: u32,
) -> Result<DynamicImage, ImgrsError> {
    let style = EmojiStyle {
        size,
        x,
        y,
        ..Default::default()
    };
    
    add_emoji(image, emoji_type, style)
}
