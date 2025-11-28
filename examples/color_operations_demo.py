#!/usr/bin/env python3
"""
Color Operations Demo for Imgrs Image Processing Library

This example demonstrates the advanced color operations from ColorMixin:
- Transparency operations (set_alpha, add_transparency, remove_transparency)
- Advanced masking system (gradient masks, color masks, luminance masks)
- Color manipulation (extract_color, color_quantize, color_shift, selective_desaturate)
- Gradient and pattern overlays
- Alpha channel operations
- Color analysis
"""

import os
import sys

# Add the parent directory to the path to import imgrs
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "python"))

import imgrs  # noqa: E402

# Create output directory
OUTPUT_DIR = "examples/output/color_operations_demo"
os.makedirs(OUTPUT_DIR, exist_ok=True)

print("🎨 Imgrs Color Operations Demo")
print("=" * 60)

# Create a test image
print("Creating test image...")
base_img = imgrs.Image.new("RGB", (400, 400), (255, 100, 150))  # Pink background
base_img.save(f"{OUTPUT_DIR}/base_image.png")
print("✓ Base image created")

# ============================================================================
# 1. TRANSPARENCY OPERATIONS
# ============================================================================
print("\n1️⃣  Transparency Operations")
print("-" * 30)

# Set global alpha
alpha_img = base_img.set_alpha(0.7)
alpha_img.save(f"{OUTPUT_DIR}/global_alpha.png")
print(f"✓ Set global alpha: {base_img.get_alpha():.1f}")
# Add transparency to specific color
transparent_img = alpha_img.add_transparency((255, 100, 150), tolerance=20)
transparent_img.save(f"{OUTPUT_DIR}/color_transparency.png")
print("✓ Added transparency to pink areas")

# Remove transparency
opaque_img = transparent_img.remove_transparency((200, 200, 200))
opaque_img.save(f"{OUTPUT_DIR}/removed_transparency.png")
print("✓ Removed transparency with gray background")

# ============================================================================
# 2. ADVANCED MASKING SYSTEM
# ============================================================================
print("\n2️⃣  Advanced Masking System")
print("-" * 30)

# Create gradient mask
gradient_mask = base_img.create_gradient_mask("radial", 0.0, 1.0)
gradient_mask.save(f"{OUTPUT_DIR}/radial_gradient_mask.png")
print("✓ Created radial gradient mask")

# Create color-based mask
color_mask = base_img.create_color_mask((255, 100, 150), tolerance=30, feather=10)
color_mask.save(f"{OUTPUT_DIR}/color_mask.png")
print("✓ Created color-based mask")

# Create luminance mask
luminance_mask = base_img.create_luminance_mask(invert=True)
luminance_mask.save(f"{OUTPUT_DIR}/luminance_mask.png")
print("✓ Created inverted luminance mask")

# Apply mask
masked_img = base_img.apply_mask(gradient_mask, invert=False)
masked_img.save(f"{OUTPUT_DIR}/masked_image.png")
print("✓ Applied gradient mask to image")

# Combine masks
combined_mask = base_img.combine_masks([gradient_mask, color_mask], "multiply")
combined_mask.save(f"{OUTPUT_DIR}/combined_masks.png")
print("✓ Combined masks with multiply operation")

# ============================================================================
# 3. COLOR MANIPULATION
# ============================================================================
print("\n3️⃣  Color Manipulation")
print("-" * 30)

# Extract specific color
extracted = base_img.extract_color((255, 100, 150), tolerance=40)
extracted.save(f"{OUTPUT_DIR}/extracted_color.png")
print("✓ Extracted pink color regions")

# Color quantization
quantized = base_img.color_quantize(levels=8)
quantized.save(f"{OUTPUT_DIR}/color_quantized.png")
print("✓ Quantized colors to 8 levels")

# Color shift
shifted = base_img.color_shift(0.3)
shifted.save(f"{OUTPUT_DIR}/color_shifted.png")
print("✓ Applied color shift")

# Selective desaturation
selective_desat = base_img.selective_desaturate(
    (255, 100, 150), tolerance=50, desaturate_factor=0.8
)
selective_desat.save(f"{OUTPUT_DIR}/selective_desaturate.png")
print("✓ Selectively desaturated pink areas")

# ============================================================================
# 4. GRADIENT AND PATTERN OVERLAYS
# ============================================================================
print("\n4️⃣  Gradient and Pattern Overlays")
print("-" * 30)

# Gradient overlay
gradient_overlay = base_img.apply_gradient_overlay(
    (0, 255, 100, 150), "horizontal", 0.8
)
gradient_overlay.save(f"{OUTPUT_DIR}/gradient_overlay.png")
print("✓ Applied horizontal green gradient overlay")

# Stripe pattern
stripes = base_img.create_stripe_pattern(
    (255, 255, 0, 120), width=15, spacing=10, angle=45.0
)
stripes.save(f"{OUTPUT_DIR}/stripe_pattern.png")
print("✓ Created diagonal yellow stripe pattern")

# Checker pattern
checker = base_img.create_checker_pattern((255, 0, 0, 100), (0, 0, 255, 100), size=20)
checker.save(f"{OUTPUT_DIR}/checker_pattern.png")
print("✓ Created red-blue checker pattern")

# ============================================================================
# 5. ALPHA CHANNEL OPERATIONS
# ============================================================================
print("\n5️⃣  Alpha Channel Operations")
print("-" * 30)

# Convert to RGBA for alpha operations
rgba_img = base_img.convert("RGBA")

# Split alpha
rgb_part, alpha_part = rgba_img.split_alpha()
rgb_part.save(f"{OUTPUT_DIR}/rgb_split.png")
alpha_part.save(f"{OUTPUT_DIR}/alpha_split.png")
print("✓ Split image into RGB and alpha channels")

# Merge alpha back
merged = rgb_part.merge_alpha(alpha_part)
merged.save(f"{OUTPUT_DIR}/alpha_merged.png")
print("✓ Merged alpha channel back")

# Convert alpha to color
alpha_to_color = rgba_img.alpha_to_color((100, 100, 100))
alpha_to_color.save(f"{OUTPUT_DIR}/alpha_to_color.png")
print("✓ Converted alpha channel to gray color")


# ============================================================================
# 7. COLOR ANALYSIS
# ============================================================================
print("\n7️⃣  Color Analysis")
print("-" * 30)

# Get color palette
palette = base_img.get_color_palette(max_colors=5)
print(f"✓ Extracted color palette: {len(palette)} colors")
for i, color in enumerate(palette):
    print(f"  Color {i+1}: RGB{color}")

# Analyze color distribution
distribution = base_img.analyze_color_distribution()
print("✓ Analyzed color distribution:")
print(f"  Total pixels: {distribution.get('total_pixels', 'N/A')}")
print(f"  Unique colors: {distribution.get('unique_colors', 'N/A')}")
print(f"  Dominant color: {distribution.get('dominant_color', 'N/A')}")

# Find color regions
regions = base_img.find_color_regions((255, 100, 150), tolerance=30)
print(f"✓ Found {len(regions)} color regions matching pink")

# ============================================================================
# SUMMARY
# ============================================================================
print("\n" + "=" * 60)
print("✅ COLOR OPERATIONS DEMO COMPLETED!")
print("=" * 60)
print(f"📁 Output saved to: {OUTPUT_DIR}/")
print("\n🎨 Demonstrated ColorMixin features:")
print("• Transparency operations (set_alpha, add_transparency, remove_transparency)")
print("• Advanced masking system (gradient, color, luminance masks)")
print("• Color manipulation (extract, quantize, shift, selective desaturate)")
print("• Gradient and pattern overlays")
print("• Alpha channel operations (split, merge, convert)")
print("• Color analysis (palette extraction, distribution, region finding)")
print("\n🚀 All color operations working perfectly!")
print("=" * 60)
