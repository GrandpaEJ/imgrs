// Drawing primitives
mod shapes;
mod shapes_extended;
mod shapes_generation;
mod text;

// Re-export public functions
pub use shapes::{draw_rectangle, draw_circle, draw_line};
pub use shapes_extended::{draw_star, draw_triangle, draw_polygon, draw_ellipse, draw_regular_polygon};
pub use shapes_generation::{
    create_rectangle, create_circle, create_triangle, create_ellipse, create_star,
    create_square, create_diamond, create_hexagon, create_parallelogram,
    create_pentagon, create_octagon, create_heart, create_arrow, create_cross,
    create_quadrilateral
};
pub use text::draw_text;

