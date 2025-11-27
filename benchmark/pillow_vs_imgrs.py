#!/usr/bin/env python3
"""
Performance Benchmark: imgrs vs Pillow (PIL)
Compares execution time for common image operations
"""

import statistics
import time
from pathlib import Path

try:
    from PIL import Image as PILImage
    from PIL import ImageEnhance, ImageFilter

    HAS_PILLOW = True
except ImportError:
    HAS_PILLOW = False
    print("⚠️  Pillow not installed. Install with: pip install Pillow")

try:
    import imgrs

    HAS_IMGRS = True
except ImportError:
    HAS_IMGRS = False
    print("⚠️  imgrs not installed. Build with: maturin develop --release")

import numpy as np


def benchmark(func, iterations=10, warmup=2):
    """Run benchmark with warmup and multiple iterations"""
    # Warmup
    for _ in range(warmup):
        func()

    # Actual benchmark
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        func()
        end = time.perf_counter()
        times.append(end - start)

    return {
        "mean": statistics.mean(times),
        "median": statistics.median(times),
        "min": min(times),
        "max": max(times),
        "stdev": statistics.stdev(times) if len(times) > 1 else 0,
    }


def format_time(seconds):
    """Format time in appropriate unit"""
    if seconds < 0.001:
        return f"{seconds * 1_000_000:.2f} µs"
    elif seconds < 1:
        return f"{seconds * 1000:.2f} ms"
    else:
        return f"{seconds:.2f} s"


def print_results(name, pillow_result, imgrs_result):
    """Print comparison results"""
    speedup = pillow_result["mean"] / imgrs_result["mean"]

    print(f"\n{'='*60}")
    print(f"  {name}")
    print(f"{'='*60}")
    print(
        f"  Pillow:  {format_time(pillow_result['mean'])} ± {format_time(pillow_result['stdev'])}"
    )
    print(
        f"  imgrs:   {format_time(imgrs_result['mean'])} ± {format_time(imgrs_result['stdev'])}"
    )
    print(f"  Speedup: {speedup:.2f}x {'🚀' if speedup > 1 else '🐌'}")


def main():
    if not HAS_PILLOW or not HAS_IMGRS:
        print("\n❌ Both Pillow and imgrs are required for benchmarking")
        return 1

    # Setup test image
    img_path = Path("examples/img/geometric.png")
    if not img_path.exists():
        print(f"❌ Test image not found: {img_path}")
        return 1

    print("\n" + "=" * 60)
    print("  imgrs vs Pillow Performance Benchmark")
    print("=" * 60)
    print(f"  Test Image: {img_path}")
    print("  Iterations: 10 (with 2 warmup runs)")
    print("=" * 60)

    # ==================== LOAD IMAGE ====================
    def pillow_load():
        img = PILImage.open(img_path)
        img.load()
        return img

    def imgrs_load():
        return imgrs.open(str(img_path))

    print_results("Load Image", benchmark(pillow_load), benchmark(imgrs_load))

    # Prepare loaded images for subsequent tests
    pil_img = PILImage.open(img_path)
    pil_img.load()
    imgrs_img = imgrs.open(str(img_path))

    # ==================== RESIZE ====================
    def pillow_resize():
        return pil_img.resize((400, 300))

    def imgrs_resize():
        return imgrs_img.resize((400, 300))

    print_results("Resize (400x300)", benchmark(pillow_resize), benchmark(imgrs_resize))

    # ==================== BLUR ====================
    def pillow_blur():
        return pil_img.filter(ImageFilter.GaussianBlur(radius=5))

    def imgrs_blur():
        return imgrs_img.blur(5.0)

    print_results(
        "Gaussian Blur (radius=5)", benchmark(pillow_blur), benchmark(imgrs_blur)
    )

    # ==================== SHARPEN ====================
    def pillow_sharpen():
        return pil_img.filter(ImageFilter.SHARPEN)

    def imgrs_sharpen():
        return imgrs_img.sharpen(1.0)

    print_results("Sharpen", benchmark(pillow_sharpen), benchmark(imgrs_sharpen))

    # ==================== BRIGHTNESS ====================
    def pillow_brightness():
        enhancer = ImageEnhance.Brightness(pil_img)
        return enhancer.enhance(1.5)

    def imgrs_brightness():
        return imgrs_img.brightness(50)

    print_results(
        "Adjust Brightness", benchmark(pillow_brightness), benchmark(imgrs_brightness)
    )

    # ==================== CONTRAST ====================
    def pillow_contrast():
        enhancer = ImageEnhance.Contrast(pil_img)
        return enhancer.enhance(1.5)

    def imgrs_contrast():
        return imgrs_img.contrast(1.5)

    print_results(
        "Adjust Contrast", benchmark(pillow_contrast), benchmark(imgrs_contrast)
    )

    # ==================== ROTATE ====================
    def pillow_rotate():
        return pil_img.rotate(45, expand=True)

    def imgrs_rotate():
        return imgrs_img.rotate(45, expand=True)

    print_results("Rotate 45°", benchmark(pillow_rotate), benchmark(imgrs_rotate))

    # ==================== GRAYSCALE ====================
    def pillow_grayscale():
        return pil_img.convert("L")

    def imgrs_grayscale():
        return imgrs_img.convert("L")

    print_results(
        "Convert to Grayscale", benchmark(pillow_grayscale), benchmark(imgrs_grayscale)
    )

    # ==================== CROP ====================
    def pillow_crop():
        return pil_img.crop((50, 50, 250, 250))

    def imgrs_crop():
        return imgrs_img.crop((50, 50, 250, 250))

    print_results("Crop (200x200)", benchmark(pillow_crop), benchmark(imgrs_crop))

    # ==================== FLIP ====================
    # Skipping - flip methods not yet implemented in imgrs
    # def pillow_flip():
    #     return pil_img.transpose(PILImage.FLIP_LEFT_RIGHT)
    #
    # def imgrs_flip():
    #     return imgrs_img.flip_left_right()
    #
    # print_results(
    #     "Flip Horizontal",
    #     benchmark(pillow_flip),
    #     benchmark(imgrs_flip)
    # )

    # ==================== SAVE ====================
    output_dir = Path("examples/output")
    output_dir.mkdir(exist_ok=True)

    def pillow_save():
        pil_img.save(output_dir / "benchmark_pillow.png")

    def imgrs_save():
        imgrs_img.save(str(output_dir / "benchmark_imgrs.png"))

    print_results(
        "Save to PNG",
        benchmark(pillow_save, iterations=5),
        benchmark(imgrs_save, iterations=5),
    )

    # ==================== NUMPY ARRAY ====================
    def pillow_to_array():
        return np.array(pil_img)

    def imgrs_to_array():
        # imgrs doesn't have direct to_array, but has to_bytes
        return imgrs_img.to_bytes()

    print_results(
        "To Array/Bytes", benchmark(pillow_to_array), benchmark(imgrs_to_array)
    )

    # ==================== CHAIN OPERATIONS ====================
    def pillow_chain():
        return (
            pil_img.resize((400, 300)).filter(ImageFilter.GaussianBlur(3)).convert("L")
        )

    def imgrs_chain():
        return imgrs_img.resize((400, 300)).blur(3.0).convert("L")

    print_results(
        "Chain: Resize → Blur → Grayscale",
        benchmark(pillow_chain),
        benchmark(imgrs_chain),
    )

    print("\n" + "=" * 60)
    print("  Benchmark Complete!")
    print("=" * 60)
    print("\n")

    return 0


if __name__ == "__main__":
    exit(main())
