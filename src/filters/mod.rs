// Kernel operations
mod kernel;
pub mod kernels_library;

// Filter implementations
mod blur;
mod sharpen;
mod edges;
mod adjustments;
pub mod simd_ops;

// Advanced filters
mod advanced_blur;
mod advanced_edges;
mod advanced_sharpen;
mod stylistic;
mod noise;
mod morphological;
mod artistic;
mod color_effects;
mod auto_enhance;

// Re-export basic functions
pub use blur::blur;
pub use sharpen::sharpen;
pub use edges::{edge_detect, emboss};
pub use adjustments::{brightness, contrast};
// SIMD operations available but not yet exposed to Python
// pub use simd_ops::{fast_rgb_to_gray, fast_brightness, fast_contrast};

// Re-export advanced blur functions
pub use advanced_blur::{
    box_blur, motion_blur, median_blur, bilateral_blur, 
    radial_blur, zoom_blur
};

// Re-export advanced edge detection
pub use advanced_edges::{
    prewitt_edge_detect, scharr_edge_detect, roberts_cross_edge_detect,
    laplacian_edge_detect, laplacian_of_gaussian, canny_edge_detect
};

// Re-export advanced sharpening
pub use advanced_sharpen::{
    unsharp_mask, high_pass, edge_enhance, edge_enhance_more
};

// Re-export stylistic effects
pub use stylistic::{
    oil_painting, posterize, pixelate, mosaic, 
    cartoon, sketch, solarize
};

// Re-export noise filters
pub use noise::{
    add_gaussian_noise, add_salt_pepper_noise,
    denoise
};

// Re-export morphological operations
pub use morphological::{
    dilate, erode, opening, closing,
    morphological_gradient
};

// Re-export artistic effects
pub use artistic::{
    vignette, halftone, pencil_sketch,
    watercolor, glitch
};

// Re-export color effects
pub use color_effects::{
    duotone, color_splash,
    chromatic_aberration
};

// Re-export kernel library
// Kernel library available but not yet exposed to Python
// pub use kernels_library::{KernelType, apply_predefined_kernel};

// Re-export auto-enhancement functions
pub use auto_enhance::{
    histogram_equalization,
    auto_contrast, auto_brightness, auto_enhance,
    exposure_adjust, auto_level, normalize,
    smart_enhance, auto_white_balance
};

