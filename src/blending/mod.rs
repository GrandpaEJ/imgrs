// Blend modes and utilities
mod composite;
mod gradient;
mod modes;

// Re-export public types and functions (allow unused for future API expansion)
#[allow(unused_imports)]
pub use composite::{alpha_composite, color_overlay, composite, composite_with_mode};
#[allow(unused_imports)]
pub use gradient::{gradient_overlay, GradientDirection};
#[allow(unused_imports)]
pub use modes::BlendMode;
