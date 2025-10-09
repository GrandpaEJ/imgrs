// CSS-like filter effects
mod sepia;
mod grayscale;
mod invert;
mod hue;
mod saturate;

// Re-export public functions
pub use sepia::sepia;
pub use grayscale::grayscale;
pub use invert::invert;
pub use hue::hue_rotate;
pub use saturate::saturate;

