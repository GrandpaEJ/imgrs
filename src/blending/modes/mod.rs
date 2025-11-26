/// Blend modes module - split into individual mode files

pub mod types;
pub mod helpers;
pub mod blend;

// Individual blend mode modules
pub mod normal;
pub mod multiply;
pub mod screen;
pub mod overlay;
pub mod soft_light;
pub mod hard_light;
pub mod darken;
pub mod lighten;
pub mod difference;
pub mod exclusion;

// Re-export the main types and functions
pub use types::BlendMode;
pub use blend::blend_pixels;