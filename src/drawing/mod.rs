// Drawing primitives
mod shapes;
mod shapes_extended;
mod text;

// Re-export public functions
pub use shapes::{draw_rectangle, draw_circle, draw_line};
pub use shapes_extended::{draw_star, draw_triangle, draw_polygon, draw_ellipse, draw_regular_polygon};
pub use text::draw_text;

