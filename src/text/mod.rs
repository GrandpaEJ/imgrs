/// Comprehensive text rendering module
/// 
/// Provides advanced text rendering with:
/// - TTF/OTF font support
/// - Text styling (size, color, weight)
/// - Text alignment (left, center, right)
/// - Multi-line text support
/// - Custom fonts or embedded defaults
/// - Anti-aliased rendering

pub mod renderer;
pub mod styles;
pub mod fonts;

pub use renderer::{
    draw_text, draw_text_multiline, draw_text_styled, draw_text_centered, draw_text_box,
    get_text_size, get_multiline_text_size, get_text_box
};
// Additional text functions available for future use
// pub use renderer::{draw_text_quick, TextBox};
// pub use styles::FontWeight;
// pub use fonts::{get_default_font, FontManager};