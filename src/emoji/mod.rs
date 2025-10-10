/// High-performance emoji rendering module for images
/// 
/// This module provides fast emoji overlay functionality with support for:
/// - Multiple emoji rendering
/// - Custom positioning
/// - Size control
/// - Opacity control
/// - Common emoji presets

pub mod renderer;
pub mod presets;

pub use renderer::{add_emoji, add_emoji_text, add_emojis_batch, EmojiStyle};
pub use presets::EmojiType;

