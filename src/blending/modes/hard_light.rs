/// Hard light blend mode implementation

use super::helpers::hard_light_blend;
use super::types::BlendMode;

pub fn apply_hard_light(base: (f32, f32, f32), overlay: (f32, f32, f32)) -> (f32, f32, f32) {
    (
        hard_light_blend(base.0, overlay.0),
        hard_light_blend(base.1, overlay.1),
        hard_light_blend(base.2, overlay.2),
    )
}

pub fn is_hard_light(mode: BlendMode) -> bool {
    matches!(mode, BlendMode::HardLight)
}