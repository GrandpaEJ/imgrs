#!/usr/bin/env python3
"""
Blending Modes Demo - Showcase advanced image compositing with imgrs

This example demonstrates the new blending functionality added to imgrs,
implementing various Porter-Duff and advanced blend modes for image compositing.
"""

from pathlib import Path

import imgrs


def create_base_images():
    """Create base images for blending demonstrations."""
    print("Creating base images...")

    # Create a gradient background
    bg = imgrs.Image.new("RGB", (400, 300), (50, 100, 200))

    # Create a circular overlay using shape creation
    circle = imgrs.Image.circle(160, (255, 100, 100, 180))

    # Create a rectangular overlay using shape creation
    rect = imgrs.Image.rectangle(150, 150, (100, 255, 100, 150))

    # Create a star overlay using shape creation
    star = imgrs.Image.star(120, (255, 255, 100, 200))

    return bg, circle, rect, star


def demonstrate_basic_blending(bg, circle, rect, star):
    """Demonstrate basic blending modes."""
    print("Demonstrating basic blending modes...")

    # Normal alpha blending (over)
    result_over = bg.composite(circle, mode="over")
    result_over.save("examples/output/blending_over.png")

    # Multiply
    result_multiply = bg.composite(circle, mode="multiply")
    result_multiply.save("examples/output/blending_multiply.png")

    # Screen
    result_screen = bg.composite(circle, mode="screen")
    result_screen.save("examples/output/blending_screen.png")

    # Overlay
    result_overlay = bg.composite(circle, mode="overlay")
    result_overlay.save("examples/output/blending_overlay.png")

    print("Basic blending results saved to examples/output/")


def demonstrate_advanced_blending(bg, circle, rect, star):
    """Demonstrate advanced blending modes."""
    print("Demonstrating advanced blending modes...")

    # Darken
    result_darken = bg.composite(rect, mode="darken")
    result_darken.save("examples/output/blending_darken.png")

    # Lighten
    result_lighten = bg.composite(rect, mode="lighten")
    result_lighten.save("examples/output/blending_lighten.png")

    # Difference
    result_difference = bg.composite(star, mode="difference")
    result_difference.save("examples/output/blending_difference.png")

    # Exclusion
    result_exclusion = bg.composite(star, mode="exclusion")
    result_exclusion.save("examples/output/blending_exclusion.png")

    # Hard Light
    result_hard_light = bg.composite(circle, mode="hard_light")
    result_hard_light.save("examples/output/blending_hard_light.png")

    # Soft Light
    result_soft_light = bg.composite(circle, mode="soft_light")
    result_soft_light.save("examples/output/blending_soft_light.png")

    print("Advanced blending results saved to examples/output/")


def demonstrate_porter_duff_modes(bg, circle):
    """Demonstrate Porter-Duff compositing modes."""
    print("Demonstrating Porter-Duff compositing modes...")

    modes = [
        "clear",
        "source",
        "dest",
        "dest_over",
        "in",
        "out",
        "atop",
        "dest_in",
        "dest_out",
        "dest_atop",
        "xor",
    ]

    for mode in modes:
        try:
            result = bg.composite(circle, mode=mode)
            result.save(f"examples/output/blending_{mode}.png")
            print(f"Saved {mode} mode")
        except Exception as e:
            print(f"Error with {mode}: {e}")

    print("Porter-Duff results saved to examples/output/")

    result = bg.blend("dest_atop", circle)
    result.save("examples/output/0_blending_with_mask.png")


def demonstrate_convenience_methods(bg, circle, rect):
    """Demonstrate convenience blending methods."""
    print("Demonstrating convenience blending methods...")

    # Using convenience methods
    result_over = bg.blend_over(circle)
    result_over.save("examples/output/blending_convenience_over.png")

    result_multiply = bg.blend_multiply(rect)
    result_multiply.save("examples/output/blending_convenience_multiply.png")

    result_screen = bg.blend_screen(circle)
    result_screen.save("examples/output/blending_convenience_screen.png")

    result_overlay = bg.blend_overlay(rect)
    result_overlay.save("examples/output/blending_convenience_overlay.png")

    result_difference = bg.blend_difference(circle)
    result_difference.save("examples/output/blending_convenience_difference.png")

    print("Convenience method results saved to examples/output/")


def create_comparison_grid(bg, circle):
    """Create a comparison grid showing different blend modes."""
    print("Creating comparison grid...")

    # Create a grid of blend mode results
    modes = [
        "over",
        "multiply",
        "screen",
        "overlay",
        "darken",
        "lighten",
        "difference",
        "exclusion",
        "hard_light",
        "soft_light",
        "color_dodge",
        "color_burn",
    ]

    # Create a larger canvas
    grid_width = 800
    grid_height = 600
    cell_width = grid_width // 4
    cell_height = grid_height // 3

    # Create white background for grid
    grid = imgrs.Image.new("RGB", (grid_width, grid_height), (255, 255, 255))

    for i, mode in enumerate(modes):
        try:
            result = bg.composite(circle, mode=mode)

            # Resize result to fit cell
            result = result.resize((cell_width - 10, cell_height - 30))

            # Calculate position
            x = (i % 4) * cell_width + 5
            y = (i // 4) * cell_height + 25

            # Paste onto grid
            grid = grid.paste(result, (x, y))

            # Add text label
            grid = grid.draw_text(mode, x + 5, y - 20, (0, 0, 0, 255), 1)

        except Exception as e:
            print(f"Error with {mode}: {e}")

    grid.save("examples/output/blending_grid.png")
    print("Comparison grid saved to examples/output/blending_grid.png")


def main():
    """Main demonstration function."""
    print("Imgrs Blending Modes Demo")
    print("=" * 40)

    # Ensure output directory exists
    Path("examples/output").mkdir(exist_ok=True)

    # Create base images
    bg, circle, rect, star = create_base_images()

    # Save base images for reference
    bg.save("examples/output/base_background.png")
    circle.save("examples/output/base_circle.png")
    rect.save("examples/output/base_rect.png")
    star.save("examples/output/base_star.png")

    # Demonstrate different blending categories
    demonstrate_basic_blending(bg, circle, rect, star)
    demonstrate_advanced_blending(bg, circle, rect, star)
    demonstrate_porter_duff_modes(bg, circle)
    demonstrate_convenience_methods(bg, circle, rect)
    create_comparison_grid(bg, circle)

    print("\nDemo completed!")
    print("Check examples/output/ for all generated images.")
    print("\nBlend modes demonstrated:")
    print("- Basic: over, multiply, screen, overlay")
    print("- Advanced: darken, lighten, difference, exclusion, hard_light, soft_light")
    print(
        "- Porter-Duff: clear, source, dest, dest_over, in, out, atop, dest_in, dest_out, dest_atop, xor"
    )
    print("- Arithmetic: add, saturate")
    print("- Special: color_dodge, color_burn")


if __name__ == "__main__":
    main()
