// Kernel operations
mod kernel;

// Filter implementations
mod blur;
mod sharpen;
mod edges;
mod adjustments;
pub mod simd_ops;

// Re-export public functions
pub use blur::blur;
pub use sharpen::sharpen;
pub use edges::{edge_detect, emboss};
pub use adjustments::{brightness, contrast};
pub use simd_ops::{fast_rgb_to_gray, fast_brightness, fast_contrast};

