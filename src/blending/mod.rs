// Blend modes and utilities
mod modes;
mod composite;
mod gradient;

// Re-export public types and functions
pub use modes::BlendMode;
pub use composite::{composite, alpha_composite, color_overlay};
pub use gradient::{GradientDirection, gradient_overlay};

