"""
Comprehensive Demo of 50+ Image Effects in imgrs
Showcases all advanced filters, kernels, and convolution operations
"""

import imgrs
import os

# Create output directory
output_dir = "examples/output/all_effects"
os.makedirs(output_dir, exist_ok=True)

# Load test image
print("Loading test image...")
img = imgrs.Image.open("examples/img/gradient.png")
print(f"Image loaded: {img.width}x{img.height}")

effects_count = 0

print("\n" + "="*70)
print("COMPREHENSIVE EFFECTS DEMO - 50+ Image Processing Operations")
print("="*70)

# ============================================================================
# BLUR EFFECTS (7 effects)
# ============================================================================
print("\n### BLUR EFFECTS ###")

print("1. Gaussian Blur...")
img.blur(3.0).save(f"{output_dir}/01_gaussian_blur.png")
effects_count += 1

print("2. Box Blur...")
img.box_blur(3).save(f"{output_dir}/02_box_blur.png")
effects_count += 1

print("3. Motion Blur (horizontal)...")
img.motion_blur(10, 0).save(f"{output_dir}/03_motion_blur_h.png")
effects_count += 1

print("4. Motion Blur (diagonal)...")
img.motion_blur(10, 45).save(f"{output_dir}/04_motion_blur_diag.png")
effects_count += 1

print("5. Median Blur...")
img.median_blur(2).save(f"{output_dir}/05_median_blur.png")
effects_count += 1

print("6. Bilateral Blur...")
img.bilateral_blur(3, 50.0, 50.0).save(f"{output_dir}/06_bilateral_blur.png")
effects_count += 1

print("7. Radial Blur...")
img.radial_blur(5.0).save(f"{output_dir}/07_radial_blur.png")
effects_count += 1

print("8. Zoom Blur...")
img.zoom_blur(20.0).save(f"{output_dir}/08_zoom_blur.png")
effects_count += 1

# ============================================================================
# EDGE DETECTION (7 effects)
# ============================================================================
print("\n### EDGE DETECTION ###")

print("9. Sobel Edge Detection...")
img.edge_detect().save(f"{output_dir}/09_sobel_edges.png")
effects_count += 1

print("10. Prewitt Edge Detection...")
img.prewitt_edge_detect().save(f"{output_dir}/10_prewitt_edges.png")
effects_count += 1

print("11. Scharr Edge Detection...")
img.scharr_edge_detect().save(f"{output_dir}/11_scharr_edges.png")
effects_count += 1

print("12. Roberts Cross Edge Detection...")
img.roberts_cross_edge_detect().save(f"{output_dir}/12_roberts_edges.png")
effects_count += 1

print("13. Laplacian Edge Detection...")
img.laplacian_edge_detect().save(f"{output_dir}/13_laplacian_edges.png")
effects_count += 1

print("14. Laplacian of Gaussian...")
img.laplacian_of_gaussian(1.5).save(f"{output_dir}/14_log_edges.png")
effects_count += 1

print("15. Canny Edge Detection...")
img.canny_edge_detect(50.0, 150.0).save(f"{output_dir}/15_canny_edges.png")
effects_count += 1

# ============================================================================
# SHARPENING (5 effects)
# ============================================================================
print("\n### SHARPENING EFFECTS ###")

print("16. Sharpen...")
img.sharpen(1.5).save(f"{output_dir}/16_sharpen.png")
effects_count += 1

print("17. Unsharp Mask...")
img.unsharp_mask(2.0, 1.5, 10).save(f"{output_dir}/17_unsharp_mask.png")
effects_count += 1

print("18. High Pass Filter...")
img.high_pass(3.0).save(f"{output_dir}/18_high_pass.png")
effects_count += 1

print("19. Edge Enhance...")
img.edge_enhance(0.5).save(f"{output_dir}/19_edge_enhance.png")
effects_count += 1

print("20. Edge Enhance More...")
img.edge_enhance_more().save(f"{output_dir}/20_edge_enhance_more.png")
effects_count += 1

# ============================================================================
# STYLISTIC EFFECTS (8 effects)
# ============================================================================
print("\n### STYLISTIC EFFECTS ###")

print("21. Oil Painting...")
img.oil_painting(3, 20).save(f"{output_dir}/21_oil_painting.png")
effects_count += 1

print("22. Posterize...")
img.posterize(4).save(f"{output_dir}/22_posterize.png")
effects_count += 1

print("23. Pixelate...")
img.pixelate(8).save(f"{output_dir}/23_pixelate.png")
effects_count += 1

print("24. Mosaic...")
img.mosaic(10).save(f"{output_dir}/24_mosaic.png")
effects_count += 1

print("25. Cartoon...")
img.cartoon(6, 100.0).save(f"{output_dir}/25_cartoon.png")
effects_count += 1

print("26. Sketch...")
img.sketch(1.2).save(f"{output_dir}/26_sketch.png")
effects_count += 1

print("27. Solarize...")
img.solarize(128).save(f"{output_dir}/27_solarize.png")
effects_count += 1

print("28. Emboss...")
img.emboss().save(f"{output_dir}/28_emboss.png")
effects_count += 1

# ============================================================================
# NOISE EFFECTS (3 effects)
# ============================================================================
print("\n### NOISE EFFECTS ###")

print("29. Gaussian Noise...")
img.add_gaussian_noise(0.0, 10.0).save(f"{output_dir}/29_gaussian_noise.png")
effects_count += 1

print("30. Salt & Pepper Noise...")
img.add_salt_pepper_noise(0.02).save(f"{output_dir}/30_salt_pepper_noise.png")
effects_count += 1

print("31. Denoise (Median Filter)...")
noisy = img.add_gaussian_noise(0.0, 15.0)
noisy.denoise(2).save(f"{output_dir}/31_denoised.png")
effects_count += 1

# ============================================================================
# MORPHOLOGICAL OPERATIONS (5 effects)
# ============================================================================
print("\n### MORPHOLOGICAL OPERATIONS ###")

print("32. Dilate...")
img.convert("L").dilate(2).save(f"{output_dir}/32_dilate.png")
effects_count += 1

print("33. Erode...")
img.convert("L").erode(2).save(f"{output_dir}/33_erode.png")
effects_count += 1

print("34. Morphological Opening...")
img.convert("L").morphological_opening(2).save(f"{output_dir}/34_opening.png")
effects_count += 1

print("35. Morphological Closing...")
img.convert("L").morphological_closing(2).save(f"{output_dir}/35_closing.png")
effects_count += 1

print("36. Morphological Gradient...")
img.convert("L").morphological_gradient(2).save(f"{output_dir}/36_morpho_gradient.png")
effects_count += 1

# ============================================================================
# ARTISTIC EFFECTS (6 effects)
# ============================================================================
print("\n### ARTISTIC EFFECTS ###")

print("37. Vignette...")
img.vignette(0.5, 0.8).save(f"{output_dir}/37_vignette.png")
effects_count += 1

print("38. Halftone...")
img.convert("L").halftone(8).save(f"{output_dir}/38_halftone.png")
effects_count += 1

print("39. Pencil Sketch...")
img.pencil_sketch(2.0).save(f"{output_dir}/39_pencil_sketch.png")
effects_count += 1

print("40. Watercolor...")
img.watercolor(3).save(f"{output_dir}/40_watercolor.png")
effects_count += 1

print("41. Glitch Effect...")
img.glitch(5.0).save(f"{output_dir}/41_glitch.png")
effects_count += 1

# ============================================================================
# COLOR EFFECTS (3 effects)
# ============================================================================
print("\n### COLOR EFFECTS ###")

print("42. Duotone (Blue-Orange)...")
img.duotone((20, 20, 80), (255, 180, 100)).save(f"{output_dir}/42_duotone.png")
effects_count += 1

print("43. Color Splash (Red)...")
img.color_splash(0.0, 30.0).save(f"{output_dir}/43_color_splash_red.png")
effects_count += 1

print("44. Chromatic Aberration...")
img.chromatic_aberration(3.0).save(f"{output_dir}/44_chromatic_aberration.png")
effects_count += 1

# ============================================================================
# CSS-LIKE FILTERS (5 effects)
# ============================================================================
print("\n### CSS-LIKE FILTERS ###")

print("45. Sepia...")
img.sepia(1.0).save(f"{output_dir}/45_sepia.png")
effects_count += 1

print("46. Grayscale...")
img.grayscale_filter(1.0).save(f"{output_dir}/46_grayscale.png")
effects_count += 1

print("47. Invert...")
img.invert(1.0).save(f"{output_dir}/47_invert.png")
effects_count += 1

print("48. Hue Rotate...")
img.hue_rotate(180.0).save(f"{output_dir}/48_hue_rotate.png")
effects_count += 1

print("49. Saturate...")
img.saturate(2.0).save(f"{output_dir}/49_saturate.png")
effects_count += 1

# ============================================================================
# ADJUSTMENT FILTERS (2 effects)
# ============================================================================
print("\n### ADJUSTMENT FILTERS ###")

print("50. Brightness Adjustment...")
img.brightness(30).save(f"{output_dir}/50_brightness.png")
effects_count += 1

print("51. Contrast Adjustment...")
img.contrast(1.5).save(f"{output_dir}/51_contrast.png")
effects_count += 1

# ============================================================================
# COMBINED EFFECTS (4 bonus effects)
# ============================================================================
print("\n### BONUS: COMBINED EFFECTS ###")

print("52. Vintage Effect (Sepia + Vignette)...")
img.sepia(0.8).vignette(0.4, 0.9).save(f"{output_dir}/52_vintage.png")
effects_count += 1

print("53. HDR Effect (Unsharp + Edge Enhance)...")
img.unsharp_mask(2.0, 2.0, 5).edge_enhance(0.3).save(f"{output_dir}/53_hdr.png")
effects_count += 1

print("54. Dream Effect (Blur + Saturate)...")
img.blur(2.0).saturate(1.5).brightness(10).save(f"{output_dir}/54_dream.png")
effects_count += 1

print("55. Dramatic B&W (Contrast + Edge Enhance)...")
img.convert("L").contrast(1.8).edge_enhance(0.5).save(f"{output_dir}/55_dramatic_bw.png")
effects_count += 1

# ============================================================================
# SUMMARY
# ============================================================================
print("\n" + "="*70)
print(f"✓ Successfully applied {effects_count} different effects!")
print(f"✓ All results saved to: {output_dir}/")
print("="*70)

print("\n### EFFECTS CATEGORIES ###")
print(f"  • Blur Effects: 8")
print(f"  • Edge Detection: 7")
print(f"  • Sharpening: 5")
print(f"  • Stylistic: 8")
print(f"  • Noise: 3")
print(f"  • Morphological: 5")
print(f"  • Artistic: 5")
print(f"  • Color Effects: 3")
print(f"  • CSS-like Filters: 5")
print(f"  • Adjustments: 2")
print(f"  • Combined Effects: 4")
print(f"\nTOTAL: {effects_count} effects demonstrated!")

print("\n### KERNEL LIBRARY ###")
print("The library also includes 35+ predefined convolution kernels:")
print("  • Sobel, Prewitt, Scharr, Roberts Cross operators")
print("  • Laplacian, LoG kernels")
print("  • Gaussian, Box blur kernels")
print("  • Various sharpening kernels")
print("  • Emboss kernels (N, S, E, W directions)")
print("  • High-pass, Low-pass, Band-pass filters")
print("  • Edge enhancement, Ridge detection, and more!")

print("\n" + "="*70)
print("Demo completed successfully!")
print("="*70 + "\n")

