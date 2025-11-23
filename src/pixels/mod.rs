// Pixel access operations
mod access;
mod analysis;
mod effects;
mod regions;

// Re-export public functions
#[allow(unused_imports)]
pub use access::map_pixels;
pub use access::{get_pixel, put_pixel};
pub use analysis::{average_color, dominant_color, histogram};
pub use effects::{posterize, replace_color, threshold};
#[allow(unused_imports)]
pub use regions::{get_region, put_region};
