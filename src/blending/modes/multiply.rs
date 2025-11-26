/// Multiply blend mode implementation

use super::types::BlendMode;

pub fn apply_multiply(base: (f32, f32, f32), overlay: (f32, f32, f32)) -> (f32, f32, f32) {
    (base.0 * overlay.0, base.1 * overlay.1, base.2 * overlay.2)
}

pub fn is_multiply(mode: BlendMode) -> bool {
    matches!(mode, BlendMode::Multiply)
}