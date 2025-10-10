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

pub use renderer::{draw_text, draw_text_multiline, draw_text_styled, draw_text_centered, draw_text_quick};
pub use styles::{TextStyle, TextAlign, FontWeight};
pub use fonts::{load_font, get_default_font, FontManager};

