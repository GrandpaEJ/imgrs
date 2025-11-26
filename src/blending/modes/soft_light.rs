/// Soft light blend mode implementation

use super::helpers::soft_light_blend;
use super::types::BlendMode;

pub fn apply_soft_light(base: (f32, f32, f32), overlay: (f32, f32, f32)) -> (f32, f32, f32) {
    (
        soft_light_blend(base.0, overlay.0),
        soft_light_blend(base.1, overlay.1),
        soft_light_blend(base.2, overlay.2),
    )
}

pub fn is_soft_light(mode: BlendMode) -> bool {
    matches!(mode, BlendMode::SoftLight)
}