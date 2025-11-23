use super::presets::EmojiType;
use crate::errors::ImgrsError;
/// Text and emoji rendering with emoji support
/// Uses Cairo/Pango for high-quality text and emoji rendering
use image::DynamicImage;

use cairo;
use pango;
use pangocairo;

/// Style options for emoji rendering
#[derive(Debug, Clone)]
#[allow(dead_code)]
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

pub fn add_text(
    image: &DynamicImage,
    text: &str,
    x: f64,
    y: f64,
    _font_family: &str,
    font_size: f64,
    color: (f64, f64, f64),
) -> Result<DynamicImage, ImgrsError> {
    let rgba_image = image.to_rgba8();
    let width = rgba_image.width() as i32;
    let height = rgba_image.height() as i32;
    let mut data = rgba_image.into_raw();

    let surface = unsafe {
        cairo::ImageSurface::create_for_data_unsafe(
            data.as_mut_ptr(),
            cairo::Format::ARgb32,
            width,
            height,
            width * 4,
        )
    }
    .map_err(|_| ImgrsError::InvalidOperation("Failed to create Cairo surface".to_string()))?;

    let cr = cairo::Context::new(&surface)
        .map_err(|_| ImgrsError::InvalidOperation("Failed to create Cairo context".to_string()))?;

    cr.set_source_rgb(color.0, color.1, color.2);

    let layout = pangocairo::create_layout(&cr);
    layout.set_text(text);

    let mut desc = pango::FontDescription::new();
    // Configure font with emoji support - prioritize emoji font for emoji rendering
    desc.set_family("Noto Color Emoji, DejaVu Sans");
    desc.set_size((font_size * pango::SCALE as f64) as i32);
    layout.set_font_description(Some(&desc));

    cr.move_to(x, y);
    pangocairo::show_layout(&cr, &layout);

    // Convert back to RGBA
    let mut rgba_data = Vec::with_capacity(data.len());
    for chunk in data.chunks_exact(4) {
        let b = chunk[0];
        let g = chunk[1];
        let r = chunk[2];
        let a = chunk[3];
        rgba_data.push(r);
        rgba_data.push(g);
        rgba_data.push(b);
        rgba_data.push(a);
    }

    let img = image::RgbaImage::from_raw(width as u32, height as u32, rgba_data).ok_or(
        ImgrsError::InvalidOperation("Failed to create image".to_string()),
    )?;

    Ok(DynamicImage::ImageRgba8(img))
}

pub fn add_textbox(
    image: &DynamicImage,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    fill_color: (f64, f64, f64),
    border_color: (f64, f64, f64),
    border_width: f64,
) -> Result<DynamicImage, ImgrsError> {
    let rgba_image = image.to_rgba8();
    let img_width = rgba_image.width() as i32;
    let img_height = rgba_image.height() as i32;
    let mut data = rgba_image.into_raw();

    let surface = unsafe {
        cairo::ImageSurface::create_for_data_unsafe(
            data.as_mut_ptr(),
            cairo::Format::ARgb32,
            img_width,
            img_height,
            img_width * 4,
        )
    }
    .map_err(|_| ImgrsError::InvalidOperation("Failed to create Cairo surface".to_string()))?;

    let cr = cairo::Context::new(&surface)
        .map_err(|_| ImgrsError::InvalidOperation("Failed to create Cairo context".to_string()))?;

    cr.set_source_rgb(fill_color.0, fill_color.1, fill_color.2);
    cr.rectangle(x, y, width, height);
    cr.fill()
        .map_err(|_| ImgrsError::InvalidOperation("Failed to fill".to_string()))?;

    cr.set_source_rgb(border_color.0, border_color.1, border_color.2);
    cr.set_line_width(border_width);
    cr.rectangle(x, y, width, height);
    cr.stroke()
        .map_err(|_| ImgrsError::InvalidOperation("Failed to stroke".to_string()))?;

    // Convert back
    let mut rgba_data = Vec::with_capacity(data.len());
    for chunk in data.chunks_exact(4) {
        let b = chunk[0];
        let g = chunk[1];
        let r = chunk[2];
        let a = chunk[3];
        rgba_data.push(r);
        rgba_data.push(g);
        rgba_data.push(b);
        rgba_data.push(a);
    }

    let img = image::RgbaImage::from_raw(img_width as u32, img_height as u32, rgba_data).ok_or(
        ImgrsError::InvalidOperation("Failed to create image".to_string()),
    )?;

    Ok(DynamicImage::ImageRgba8(img))
}

/// Add emoji to image using preset emoji type
pub fn add_emoji(
    image: &DynamicImage,
    emoji_type: EmojiType,
    style: EmojiStyle,
) -> Result<DynamicImage, ImgrsError> {
    let emoji = emoji_type.as_str();
    let color = style
        .color
        .map(|c| (c.0 as f64 / 255.0, c.1 as f64 / 255.0, c.2 as f64 / 255.0))
        .unwrap_or((0.0, 0.0, 0.0));
    add_text(
        image,
        emoji,
        style.x as f64,
        style.y as f64,
        "Sans",
        style.size as f64,
        color,
    )
}

/// Add emoji to image using raw emoji text (Unicode)
pub fn add_emoji_text(
    image: &DynamicImage,
    emoji: &str,
    style: EmojiStyle,
) -> Result<DynamicImage, ImgrsError> {
    let color = style
        .color
        .map(|c| (c.0 as f64 / 255.0, c.1 as f64 / 255.0, c.2 as f64 / 255.0))
        .unwrap_or((0.0, 0.0, 0.0));
    add_text(
        image,
        emoji,
        style.x as f64,
        style.y as f64,
        "Sans",
        style.size as f64,
        color,
    )
}

/// Add multiple emojis to an image
pub fn add_emojis_batch(
    image: &DynamicImage,
    emojis: Vec<(EmojiType, EmojiStyle)>,
) -> Result<DynamicImage, ImgrsError> {
    let mut result = image.clone();
    for (emoji_type, style) in emojis {
        result = add_emoji(&result, emoji_type, style)?;
    }
    Ok(result)
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
