#!/usr/bin/env python3
"""
Chroma Key (Green Screen) Demo

This example demonstrates how to use the chroma key functionality
to remove green backgrounds from images, similar to green screen effects
used in video production and photography.
"""

import imgrs


def create_green_screen_demo():
    """Create a demo showing chroma key functionality"""

    print("🎬 Chroma Key (Green Screen) Demo")
    print("=" * 40)

    # Create a green background (representing a green screen)
    green_screen = imgrs.new("RGB", (400, 300), (0, 255, 0))  # Pure green
    print("✓ Created green screen background")

    # Create a subject (red circle) to place on the green screen
    subject = imgrs.new("RGB", (150, 150), (255, 0, 0))  # Red background
    # Draw a blue circle on the red background
    subject = subject.draw_circle(75, 75, 60, (0, 0, 255, 255))  # Blue circle
    print("✓ Created subject (blue circle on red background)")

    # Composite subject onto green screen
    scene = imgrs.paste(green_screen, subject, (125, 75))
    scene.save("examples/output/chroma_key_original.png")
    print("✓ Saved original scene with green screen")

    # Apply chroma key to remove green background
    keyed_scene = imgrs.chroma_key(scene, (0, 255, 0), tolerance=0.2, feather=0.1)
    keyed_scene.save("examples/output/chroma_key_result.png")
    print("✓ Applied chroma key and saved transparent result")

    # Demonstrate compositing with new background
    beach_bg = imgrs.new("RGB", (400, 300), (135, 206, 235))  # Sky blue
    # Add some "sand" at the bottom
    beach_bg = imgrs.new("RGB", (400, 100), (238, 203, 173))  # Sandy color
    beach_bg = imgrs.paste(
        beach_bg, imgrs.new("RGB", (400, 200), (135, 206, 235)), (0, 0)
    )

    # Composite the keyed subject onto the beach background
    final_composite = imgrs.paste(beach_bg, keyed_scene, (0, 0))
    final_composite.save("examples/output/chroma_key_composite.png")
    print("✓ Created final composite with beach background")

    print("\n📁 Output files created:")
    print("   - examples/output/chroma_key_original.png (original scene)")
    print("   - examples/output/chroma_key_result.png (chroma keyed)")
    print("   - examples/output/chroma_key_composite.png (final composite)")

    print("\n🎨 Chroma Key Parameters:")
    print("   - Key Color: (0, 255, 0) - Pure green")
    print("   - Tolerance: 0.2 - Moderate color matching")
    print("   - Feather: 0.1 - Soft edges for natural results")


def demo_different_key_colors():
    """Demonstrate chroma key with different background colors"""

    print("\n🌈 Testing Different Key Colors")
    print("-" * 30)

    # Create base image with colored borders
    base = imgrs.new("RGB", (300, 200), (128, 128, 128))  # Gray background
    base = base.draw_rectangle(0, 0, 300, 50, (255, 0, 0, 255))  # Red border
    base = base.draw_rectangle(0, 150, 300, 200, (0, 255, 0, 255))  # Green border
    base = base.draw_rectangle(0, 0, 50, 200, (0, 0, 255, 255))  # Blue border
    base = base.draw_rectangle(250, 0, 300, 200, (255, 255, 0, 255))  # Yellow border

    # Add a white circle in the center
    base = base.draw_circle(150, 100, 40, (255, 255, 255, 255))

    base.save("examples/output/multi_color_base.png")
    print("✓ Created multi-colored test image")

    # Test different key colors
    key_colors = [
        ((255, 0, 0), "red"),
        ((0, 255, 0), "green"),
        ((0, 0, 255), "blue"),
        ((255, 255, 0), "yellow"),
    ]

    for color, name in key_colors:
        keyed = imgrs.chroma_key(base, color, tolerance=0.1, feather=0.05)
        keyed.save(f"examples/output/chroma_key_{name}.png")
        print(f"✓ Removed {name} background")

    print("\n📁 Additional output files:")
    print("   - examples/output/multi_color_base.png (original)")
    print("   - examples/output/chroma_key_red.png")
    print("   - examples/output/chroma_key_green.png")
    print("   - examples/output/chroma_key_blue.png")
    print("   - examples/output/chroma_key_yellow.png")


if __name__ == "__main__":
    create_green_screen_demo()
    demo_different_key_colors()

    print("\n✅ Chroma Key Demo Complete!")
    print("\n💡 Tips for best results:")
    print("   - Use evenly lit backgrounds")
    print("   - Avoid shadows and wrinkles in the background")
    print("   - Adjust tolerance based on lighting conditions")
    print("   - Use feather > 0 for natural-looking edges")
    print("   - For video, apply chroma key to each frame")
