#!/usr/bin/env python3
"""
Organize output images into subdirectories based on their names.
"""
import shutil
from pathlib import Path

# Define category mappings based on filename patterns
CATEGORIES = {
    "blending": ["blending_", "blend_"],
    "brightness_contrast": ["brightness_", "contrast_"],
    "blur": ["blur_"],
    "css_filters": ["css_"],
    "edges": ["edges_"],
    "emboss": ["emboss_"],
    "filters": ["filter_chain", "sharpen_", "posterize"],
    "geometry": [
        "flipped_",
        "rotated_",
        "cropped",
        "resized",
        "resampled_",
        "thumbnail",
    ],
    "masks": ["_mask", "circular_mask", "grayscale_mask"],
    "channels_color": ["channel_", "gray_channel", "rgba_channel"],
    "colors": ["replace_color", "transparency_levels"],
    "composition": ["paste_", "composition", "composite"],
    "drawing": ["base_", "drawing_"],
    "misc": ["new_", "functional_", "roundtrip_", "putpixel_", "converted_"],
    "text_rendering": ["text_", "woff2_test"],
    "chroma_key": ["chroma_key_"],
    "effects": ["combo_"],
}


def organize_images():
    """Move loose PNG files into appropriate subdirectories."""
    output_dir = Path("examples/output")

    # Get all PNG files in the root output directory
    png_files = list(output_dir.glob("*.png"))

    print(f"Found {len(png_files)} PNG files to organize")

    moved_count = 0
    skipped_count = 0

    for png_file in png_files:
        filename = png_file.name
        moved = False

        # Try to match filename to a category
        for category, patterns in CATEGORIES.items():
            if any(pattern in filename for pattern in patterns):
                # Create category directory if it doesn't exist
                category_dir = output_dir / category
                category_dir.mkdir(exist_ok=True)

                # Move the file
                dest = category_dir / filename
                if not dest.exists():
                    shutil.move(str(png_file), str(dest))
                    print(f"Moved {filename} -> {category}/")
                    moved_count += 1
                    moved = True
                    break
                else:
                    print(f"Skipped {filename} (already exists in {category}/)")
                    skipped_count += 1
                    moved = True
                    break

        if not moved:
            # If no category matched, move to misc
            misc_dir = output_dir / "misc"
            misc_dir.mkdir(exist_ok=True)
            dest = misc_dir / filename
            if not dest.exists():
                shutil.move(str(png_file), str(dest))
                print(f"Moved {filename} -> misc/")
                moved_count += 1
            else:
                print(f"Skipped {filename} (already exists in misc/)")
                skipped_count += 1

    print(f"\nSummary: {moved_count} files moved, {skipped_count} files skipped")


if __name__ == "__main__":
    organize_images()
