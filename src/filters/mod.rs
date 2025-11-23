// Kernel operations
mod kernel;
pub mod kernels_library;

// Filter implementations
mod adjustments;
mod blur;
mod edges;
mod sharpen;
pub mod simd_ops;

// Advanced filters
mod advanced_blur;
mod advanced_edges;
mod advanced_sharpen;
mod artistic;
mod auto_enhance;
mod color_effects;
mod morphological;
mod noise;
mod stylistic;

// Re-export basic functions
pub use adjustments::{brightness, contrast};
pub use blur::blur;
pub use edges::{edge_detect, emboss};
pub use sharpen::sharpen;
// SIMD operations available but not yet exposed to Python
// pub use simd_ops::{fast_rgb_to_gray, fast_brightness, fast_contrast};

// Re-export advanced blur functions
pub use advanced_blur::{
    bilateral_blur, box_blur, median_blur, motion_blur, radial_blur, zoom_blur,
};

// Re-export advanced edge detection
pub use advanced_edges::{
    canny_edge_detect, laplacian_edge_detect, laplacian_of_gaussian, prewitt_edge_detect,
    roberts_cross_edge_detect, scharr_edge_detect,
};

// Re-export advanced sharpening
pub use advanced_sharpen::{edge_enhance, edge_enhance_more, high_pass, unsharp_mask};

// Re-export stylistic effects
pub use stylistic::{cartoon, mosaic, oil_painting, pixelate, posterize, sketch, solarize};

// Re-export noise filters
pub use noise::{add_gaussian_noise, add_salt_pepper_noise, denoise};

// Re-export morphological operations
pub use morphological::{closing, dilate, erode, morphological_gradient, opening};

// Re-export artistic effects
pub use artistic::{glitch, halftone, pencil_sketch, vignette, watercolor};

// Re-export color effects
pub use color_effects::{chroma_key, chromatic_aberration, color_splash, duotone};

// Re-export kernel library
// Kernel library available but not yet exposed to Python
// pub use kernels_library::{KernelType, apply_predefined_kernel};

// Re-export auto-enhancement functions
pub use auto_enhance::{
    auto_brightness, auto_contrast, auto_enhance, auto_level, auto_white_balance, exposure_adjust,
    histogram_equalization, normalize, smart_enhance,
};
