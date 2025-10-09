// Pixel access operations
mod access;
mod regions;
mod analysis;
mod effects;

// Re-export public functions
pub use access::{get_pixel, put_pixel};
#[allow(unused_imports)]
pub use access::map_pixels;
#[allow(unused_imports)]
pub use regions::{get_region, put_region};
pub use analysis::{histogram, dominant_color, average_color};
pub use effects::{replace_color, threshold, posterize};

