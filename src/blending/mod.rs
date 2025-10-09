// Blend modes and utilities
mod modes;
mod composite;
mod gradient;

// Re-export public types and functions (allow unused for future API expansion)
#[allow(unused_imports)]
pub use modes::BlendMode;
#[allow(unused_imports)]
pub use composite::{composite, alpha_composite, color_overlay};
#[allow(unused_imports)]
pub use gradient::{GradientDirection, gradient_overlay};

