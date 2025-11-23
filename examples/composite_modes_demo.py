"""
Demo of all CSS composite/blending modes in imgrs v0.3
Uses real images and saves outputs for visual comparison
"""

import os
from imgrs import Image

# Create output directory
output_dir = "examples/output/composite_modes"
os.makedirs(output_dir, exist_ok=True)

# Load test image
print("Loading test image...")
img_path = "examples/img/colorful_squares.png"
if not os.path.exists(img_path):
    # Fallback: create a test image
    print("Test image not found, creating one...")
    img = Image.new("RGBA", (400, 400), (255, 100, 100, 255))
else:
    img = Image.open(img_path)

print(f"Image loaded: {img.size}")

# Test all composite modes
modes = [
    # Source/Destination operations
    "source-over",
    "source-in",
    "source-out",
    "source-atop",
    "destination-over",
    "destination-in",
    "destination-out",
    "destination-atop",
    
    # Blending modes
    "normal",
    "multiply",
    "screen",
    "overlay",
    "darken",
    "lighten",
    "color-dodge",
    "color-burn",
    "hard-light",
    "soft-light",
    "difference",
    "exclusion",
    "lighter",
    "copy",
    "xor",
    "hue",
    "saturation",
    "color",
    "luminosity",
]

print(f"\nTesting {len(modes)} composite modes:")
print("=" * 60)

success_count = 0
for i, mode in enumerate(modes, 1):
    try:
        result = img.composite(mode)
        
        # Save output
        output_path = f"{output_dir}/{i:02d}_{mode.replace('-', '_')}.png"
        result.save(output_path)
        
        print(f"✓ {i:2d}. {mode:20s} - Saved to {output_path}")
        success_count += 1
    except Exception as e:
        print(f"✗ {i:2d}. {mode:20s} - Error: {e}")

print("\n" + "=" * 60)
print(f"Success: {success_count}/{len(modes)} modes")

# Test with opacity variations
print("\nTesting opacity variations...")
opacity_modes = ["multiply", "screen", "overlay"]
for mode in opacity_modes:
    for opacity in [0.3, 0.5, 0.7]:
        try:
            result = img.composite(mode, opacity=opacity)
            output_path = f"{output_dir}/{mode}_opacity_{int(opacity*100)}.png"
            result.save(output_path)
            print(f"✓ {mode} @ {opacity:.1f} opacity - Saved")
        except Exception as e:
            print(f"✗ {mode} @ {opacity:.1f} opacity - Error: {e}")

print(f"\n✅ Demo complete! Check outputs in: {output_dir}/")
