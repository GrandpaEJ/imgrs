/// Comprehensive library of convolution kernels for various image processing effects

use crate::errors::ImgrsError;

/// Predefined kernel types
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum KernelType {
    // Edge Detection
    SobelX,
    SobelY,
    PrewittX,
    PrewittY,
    ScharrX,
    ScharrY,
    LaplacianSimple,
    LaplacianOfGaussian,
    RobertsCross1,
    RobertsCross2,
    
    // Sharpening
    Sharpen,
    SharpenIntense,
    SharpenSubtle,
    UnsharpMask,
    EdgeEnhance,
    EdgeEnhanceMore,
    
    // Blurring
    BoxBlur3,
    BoxBlur5,
    Gaussian3,
    Gaussian5,
    
    // Emboss and Relief
    Emboss,
    EmbossNorth,
    EmbossSouth,
    EmbossEast,
    EmbossWest,
    Relief,
    
    // Special Effects
    Ridge,
    Outline,
    FindEdges,
    Smooth,
    SmoothMore,
    Denoise,
    HighPass,
    LowPass,
    BandPass,
}

impl KernelType {
    /// Get the kernel matrix for the specified type
    #[allow(dead_code)]
    pub fn get_kernel(&self) -> Vec<Vec<f32>> {
        match self {
            // Sobel Operators
            KernelType::SobelX => vec![
                vec![-1.0, 0.0, 1.0],
                vec![-2.0, 0.0, 2.0],
                vec![-1.0, 0.0, 1.0],
            ],
            KernelType::SobelY => vec![
                vec![-1.0, -2.0, -1.0],
                vec![0.0, 0.0, 0.0],
                vec![1.0, 2.0, 1.0],
            ],
            
            // Prewitt Operators
            KernelType::PrewittX => vec![
                vec![-1.0, 0.0, 1.0],
                vec![-1.0, 0.0, 1.0],
                vec![-1.0, 0.0, 1.0],
            ],
            KernelType::PrewittY => vec![
                vec![-1.0, -1.0, -1.0],
                vec![0.0, 0.0, 0.0],
                vec![1.0, 1.0, 1.0],
            ],
            
            // Scharr Operators
            KernelType::ScharrX => vec![
                vec![-3.0, 0.0, 3.0],
                vec![-10.0, 0.0, 10.0],
                vec![-3.0, 0.0, 3.0],
            ],
            KernelType::ScharrY => vec![
                vec![-3.0, -10.0, -3.0],
                vec![0.0, 0.0, 0.0],
                vec![3.0, 10.0, 3.0],
            ],
            
            // Laplacian
            KernelType::LaplacianSimple => vec![
                vec![0.0, -1.0, 0.0],
                vec![-1.0, 4.0, -1.0],
                vec![0.0, -1.0, 0.0],
            ],
            KernelType::LaplacianOfGaussian => vec![
                vec![0.0, 0.0, -1.0, 0.0, 0.0],
                vec![0.0, -1.0, -2.0, -1.0, 0.0],
                vec![-1.0, -2.0, 16.0, -2.0, -1.0],
                vec![0.0, -1.0, -2.0, -1.0, 0.0],
                vec![0.0, 0.0, -1.0, 0.0, 0.0],
            ],
            
            // Roberts Cross
            KernelType::RobertsCross1 => vec![
                vec![1.0, 0.0],
                vec![0.0, -1.0],
            ],
            KernelType::RobertsCross2 => vec![
                vec![0.0, 1.0],
                vec![-1.0, 0.0],
            ],
            
            // Sharpening Kernels
            KernelType::Sharpen => vec![
                vec![0.0, -1.0, 0.0],
                vec![-1.0, 5.0, -1.0],
                vec![0.0, -1.0, 0.0],
            ],
            KernelType::SharpenIntense => vec![
                vec![-1.0, -1.0, -1.0],
                vec![-1.0, 9.0, -1.0],
                vec![-1.0, -1.0, -1.0],
            ],
            KernelType::SharpenSubtle => vec![
                vec![0.0, -0.5, 0.0],
                vec![-0.5, 3.0, -0.5],
                vec![0.0, -0.5, 0.0],
            ],
            KernelType::UnsharpMask => vec![
                vec![-1.0/256.0, -4.0/256.0, -6.0/256.0, -4.0/256.0, -1.0/256.0],
                vec![-4.0/256.0, -16.0/256.0, -24.0/256.0, -16.0/256.0, -4.0/256.0],
                vec![-6.0/256.0, -24.0/256.0, 476.0/256.0, -24.0/256.0, -6.0/256.0],
                vec![-4.0/256.0, -16.0/256.0, -24.0/256.0, -16.0/256.0, -4.0/256.0],
                vec![-1.0/256.0, -4.0/256.0, -6.0/256.0, -4.0/256.0, -1.0/256.0],
            ],
            KernelType::EdgeEnhance => vec![
                vec![0.0, -1.0, 0.0],
                vec![-1.0, 5.0, -1.0],
                vec![0.0, -1.0, 0.0],
            ],
            KernelType::EdgeEnhanceMore => vec![
                vec![-1.0, -1.0, -1.0],
                vec![-1.0, 9.0, -1.0],
                vec![-1.0, -1.0, -1.0],
            ],
            
            // Box Blur
            KernelType::BoxBlur3 => {
                let val = 1.0 / 9.0;
                vec![
                    vec![val, val, val],
                    vec![val, val, val],
                    vec![val, val, val],
                ]
            },
            KernelType::BoxBlur5 => {
                let val = 1.0 / 25.0;
                vec![
                    vec![val, val, val, val, val],
                    vec![val, val, val, val, val],
                    vec![val, val, val, val, val],
                    vec![val, val, val, val, val],
                    vec![val, val, val, val, val],
                ]
            },
            
            // Gaussian Blur
            KernelType::Gaussian3 => vec![
                vec![1.0/16.0, 2.0/16.0, 1.0/16.0],
                vec![2.0/16.0, 4.0/16.0, 2.0/16.0],
                vec![1.0/16.0, 2.0/16.0, 1.0/16.0],
            ],
            KernelType::Gaussian5 => vec![
                vec![1.0/273.0, 4.0/273.0, 7.0/273.0, 4.0/273.0, 1.0/273.0],
                vec![4.0/273.0, 16.0/273.0, 26.0/273.0, 16.0/273.0, 4.0/273.0],
                vec![7.0/273.0, 26.0/273.0, 41.0/273.0, 26.0/273.0, 7.0/273.0],
                vec![4.0/273.0, 16.0/273.0, 26.0/273.0, 16.0/273.0, 4.0/273.0],
                vec![1.0/273.0, 4.0/273.0, 7.0/273.0, 4.0/273.0, 1.0/273.0],
            ],
            
            // Emboss Kernels
            KernelType::Emboss => vec![
                vec![-2.0, -1.0, 0.0],
                vec![-1.0, 1.0, 1.0],
                vec![0.0, 1.0, 2.0],
            ],
            KernelType::EmbossNorth => vec![
                vec![1.0, 1.0, 1.0],
                vec![0.0, 0.0, 0.0],
                vec![-1.0, -1.0, -1.0],
            ],
            KernelType::EmbossSouth => vec![
                vec![-1.0, -1.0, -1.0],
                vec![0.0, 0.0, 0.0],
                vec![1.0, 1.0, 1.0],
            ],
            KernelType::EmbossEast => vec![
                vec![-1.0, 0.0, 1.0],
                vec![-1.0, 0.0, 1.0],
                vec![-1.0, 0.0, 1.0],
            ],
            KernelType::EmbossWest => vec![
                vec![1.0, 0.0, -1.0],
                vec![1.0, 0.0, -1.0],
                vec![1.0, 0.0, -1.0],
            ],
            KernelType::Relief => vec![
                vec![-2.0, -1.0, 0.0],
                vec![-1.0, 1.0, 1.0],
                vec![0.0, 1.0, 2.0],
            ],
            
            // Special Effects
            KernelType::Ridge => vec![
                vec![-1.0, -1.0, -1.0],
                vec![-1.0, 8.0, -1.0],
                vec![-1.0, -1.0, -1.0],
            ],
            KernelType::Outline => vec![
                vec![-1.0, -1.0, -1.0],
                vec![-1.0, 8.0, -1.0],
                vec![-1.0, -1.0, -1.0],
            ],
            KernelType::FindEdges => vec![
                vec![0.0, -1.0, 0.0],
                vec![-1.0, 4.0, -1.0],
                vec![0.0, -1.0, 0.0],
            ],
            KernelType::Smooth => vec![
                vec![1.0/13.0, 1.0/13.0, 1.0/13.0],
                vec![1.0/13.0, 5.0/13.0, 1.0/13.0],
                vec![1.0/13.0, 1.0/13.0, 1.0/13.0],
            ],
            KernelType::SmoothMore => vec![
                vec![1.0/25.0, 1.0/25.0, 1.0/25.0, 1.0/25.0, 1.0/25.0],
                vec![1.0/25.0, 2.0/25.0, 2.0/25.0, 2.0/25.0, 1.0/25.0],
                vec![1.0/25.0, 2.0/25.0, 4.0/25.0, 2.0/25.0, 1.0/25.0],
                vec![1.0/25.0, 2.0/25.0, 2.0/25.0, 2.0/25.0, 1.0/25.0],
                vec![1.0/25.0, 1.0/25.0, 1.0/25.0, 1.0/25.0, 1.0/25.0],
            ],
            KernelType::Denoise => vec![
                vec![0.0, 1.0/8.0, 0.0],
                vec![1.0/8.0, 1.0/2.0, 1.0/8.0],
                vec![0.0, 1.0/8.0, 0.0],
            ],
            KernelType::HighPass => vec![
                vec![-1.0/9.0, -1.0/9.0, -1.0/9.0],
                vec![-1.0/9.0, 8.0/9.0, -1.0/9.0],
                vec![-1.0/9.0, -1.0/9.0, -1.0/9.0],
            ],
            KernelType::LowPass => vec![
                vec![1.0/16.0, 2.0/16.0, 1.0/16.0],
                vec![2.0/16.0, 4.0/16.0, 2.0/16.0],
                vec![1.0/16.0, 2.0/16.0, 1.0/16.0],
            ],
            KernelType::BandPass => vec![
                vec![0.0, -1.0, 0.0],
                vec![-1.0, 4.0, -1.0],
                vec![0.0, -1.0, 0.0],
            ],
        }
    }
    
    /// Get the name of the kernel
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            KernelType::SobelX => "Sobel X",
            KernelType::SobelY => "Sobel Y",
            KernelType::PrewittX => "Prewitt X",
            KernelType::PrewittY => "Prewitt Y",
            KernelType::ScharrX => "Scharr X",
            KernelType::ScharrY => "Scharr Y",
            KernelType::LaplacianSimple => "Laplacian Simple",
            KernelType::LaplacianOfGaussian => "Laplacian of Gaussian",
            KernelType::RobertsCross1 => "Roberts Cross 1",
            KernelType::RobertsCross2 => "Roberts Cross 2",
            KernelType::Sharpen => "Sharpen",
            KernelType::SharpenIntense => "Sharpen Intense",
            KernelType::SharpenSubtle => "Sharpen Subtle",
            KernelType::UnsharpMask => "Unsharp Mask",
            KernelType::EdgeEnhance => "Edge Enhance",
            KernelType::EdgeEnhanceMore => "Edge Enhance More",
            KernelType::BoxBlur3 => "Box Blur 3x3",
            KernelType::BoxBlur5 => "Box Blur 5x5",
            KernelType::Gaussian3 => "Gaussian 3x3",
            KernelType::Gaussian5 => "Gaussian 5x5",
            KernelType::Emboss => "Emboss",
            KernelType::EmbossNorth => "Emboss North",
            KernelType::EmbossSouth => "Emboss South",
            KernelType::EmbossEast => "Emboss East",
            KernelType::EmbossWest => "Emboss West",
            KernelType::Relief => "Relief",
            KernelType::Ridge => "Ridge",
            KernelType::Outline => "Outline",
            KernelType::FindEdges => "Find Edges",
            KernelType::Smooth => "Smooth",
            KernelType::SmoothMore => "Smooth More",
            KernelType::Denoise => "Denoise",
            KernelType::HighPass => "High Pass",
            KernelType::LowPass => "Low Pass",
            KernelType::BandPass => "Band Pass",
        }
    }
    
    /// Get all available kernel types
    #[allow(dead_code)]
    pub fn all() -> Vec<KernelType> {
        vec![
            KernelType::SobelX,
            KernelType::SobelY,
            KernelType::PrewittX,
            KernelType::PrewittY,
            KernelType::ScharrX,
            KernelType::ScharrY,
            KernelType::LaplacianSimple,
            KernelType::LaplacianOfGaussian,
            KernelType::RobertsCross1,
            KernelType::RobertsCross2,
            KernelType::Sharpen,
            KernelType::SharpenIntense,
            KernelType::SharpenSubtle,
            KernelType::UnsharpMask,
            KernelType::EdgeEnhance,
            KernelType::EdgeEnhanceMore,
            KernelType::BoxBlur3,
            KernelType::BoxBlur5,
            KernelType::Gaussian3,
            KernelType::Gaussian5,
            KernelType::Emboss,
            KernelType::EmbossNorth,
            KernelType::EmbossSouth,
            KernelType::EmbossEast,
            KernelType::EmbossWest,
            KernelType::Relief,
            KernelType::Ridge,
            KernelType::Outline,
            KernelType::FindEdges,
            KernelType::Smooth,
            KernelType::SmoothMore,
            KernelType::Denoise,
            KernelType::HighPass,
            KernelType::LowPass,
            KernelType::BandPass,
        ]
    }
}

/// Apply a predefined kernel to an image
    #[allow(dead_code)]
pub fn apply_predefined_kernel(image: &image::DynamicImage, kernel_type: KernelType) -> Result<image::DynamicImage, ImgrsError> {
    let kernel = kernel_type.get_kernel();
    super::kernel::apply_convolution(image, &kernel)
}

