/// Difference blend mode implementation

use super::types::BlendMode;

pub fn apply_difference(base: (f32, f32, f32), overlay: (f32, f32, f32)) -> (f32, f32, f32) {
    (
        (base.0 - overlay.0).abs(),
        (base.1 - overlay.1).abs(),
        (base.2 - overlay.2).abs(),
    )
}

pub fn is_difference(mode: BlendMode) -> bool {
    matches!(mode, BlendMode::Difference)
}