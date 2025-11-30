#!/usr/bin/env python3
"""
Remove duplicate PNG files from examples/output/ root that already exist in subdirectories.
"""
from pathlib import Path


def remove_duplicates():
    """Remove PNG files from root that already exist in subdirectories."""
    output_dir = Path("examples/output")

    # Get all PNG files in the root output directory
    root_png_files = list(output_dir.glob("*.png"))

    print(f"Found {len(root_png_files)} PNG files in root")

    removed_count = 0

    for png_file in root_png_files:
        filename = png_file.name

        # Check if this file exists in any subdirectory
        found_in_subdir = False
        for subdir in output_dir.iterdir():
            if subdir.is_dir():
                potential_duplicate = subdir / filename
                if potential_duplicate.exists():
                    # File exists in subdirectory, remove from root
                    png_file.unlink()
                    print(f"Removed {filename} (exists in {subdir.name}/)")
                    removed_count += 1
                    found_in_subdir = True
                    break

        if not found_in_subdir:
            print(f"Kept {filename} (no duplicate found)")

    print(f"\nSummary: {removed_count} duplicate files removed")


if __name__ == "__main__":
    remove_duplicates()
