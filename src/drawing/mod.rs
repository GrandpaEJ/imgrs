// Drawing primitives
mod shapes;
mod text;

// Re-export public functions
pub use shapes::{draw_rectangle, draw_circle, draw_line};
pub use text::draw_text;

