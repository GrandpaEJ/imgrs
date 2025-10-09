// Kernel operations
mod kernel;

// Filter implementations
mod blur;
mod sharpen;
mod edges;
mod adjustments;

// Re-export public functions
pub use blur::blur;
pub use sharpen::sharpen;
pub use edges::{edge_detect, emboss};
pub use adjustments::{brightness, contrast};

