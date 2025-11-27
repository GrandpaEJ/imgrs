// Drawing primitives
mod shapes;
mod shapes_extended;
mod shapes_generation;

// Re-export public functions
pub use shapes::{draw_circle, draw_line, draw_rectangle};
pub use shapes_extended::{
    draw_ellipse, draw_polygon, draw_regular_polygon, draw_star, draw_triangle,
};
pub use shapes_generation::{
    create_arrow, create_circle, create_cross, create_diamond, create_ellipse, create_heart,
    create_hexagon, create_octagon, create_parallelogram, create_pentagon, create_quadrilateral,
    create_rectangle, create_square, create_star, create_triangle,
};
// Text functions are now in the text module
