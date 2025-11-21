/// Text and emoji rendering module
///
/// This module provides text and emoji rendering functionality with support for:
/// - Text with emojis
/// - Custom fonts
/// - Text boxes
/// - Emoji presets

pub mod renderer;
pub mod presets;

pub use renderer::{add_text, add_textbox, add_emoji, add_emoji_text, add_emojis_batch, add_emoji_quick, EmojiStyle};
pub use presets::EmojiType;

