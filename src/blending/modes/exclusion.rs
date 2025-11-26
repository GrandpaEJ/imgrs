/// Exclusion blend mode implementation

use super::types::BlendMode;

pub fn apply_exclusion(base: (f32, f32, f32), overlay: (f32, f32, f32)) -> (f32, f32, f32) {
    (
        base.0 + overlay.0 - 2.0 * base.0 * overlay.0,
        base.1 + overlay.1 - 2.0 * base.1 * overlay.1,
        base.2 + overlay.2 - 2.0 * base.2 * overlay.2,
    )
}

pub fn is_exclusion(mode: BlendMode) -> bool {
    matches!(mode, BlendMode::Exclusion)
}