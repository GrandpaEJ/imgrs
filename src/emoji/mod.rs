pub mod presets;
/// Text and emoji rendering module
///
/// This module provides text and emoji rendering functionality with support for:
/// - Text with emojis
/// - Custom fonts
/// - Text boxes
/// - Emoji presets
pub mod renderer;

pub use presets::EmojiType;
pub use renderer::{
    add_emoji, add_emoji_text, add_emojis_batch, add_text, add_textbox, EmojiStyle,
};
