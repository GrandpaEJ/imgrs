#!/usr/bin/env python3
"""
Comprehensive benchmark comparing imgrs vs Pillow performance.
"""

import time
import json
from pathlib import Path
from typing import Dict, List, Callable
import sys

# Add parent directory to path
sys.path.insert(0, str(Path(__file__).parent.parent))

try:
    from PIL import Image as PILImage
    HAS_PILLOW = True
except ImportError:
    HAS_PILLOW = False
    print("⚠️ Pillow not installed. Install with: pip install Pillow")

try:
    from imgrs import Image as ImgrsImage
    HAS_IMGRS = True
except ImportError:
    HAS_IMGRS = False
    print("⚠️ imgrs not installed. Install with: pip install -e .")

import numpy as np


class BenchmarkRunner:
    """Run and record benchmarks."""
    
    def __init__(self, test_image_path: str, iterations: int = 100):
        self.test_image_path = test_image_path
        self.iterations = iterations
        self.results = {}
        
    def time_operation(self, operation: Callable, name: str) -> Dict:
        """Time an operation over multiple iterations."""
        times = []
        
        print(f"    Running {self.iterations} iterations...", end=" ", flush=True)
        for i in range(self.iterations):
            start = time.perf_counter()
            try:
                result = operation()
                elapsed = time.perf_counter() - start
                times.append(elapsed)
            except Exception as e:
                return {
                    "name": name,
                    "error": str(e),
                    "iterations": 0,
                    "avg_time": 0,
                    "total_time": 0
                }
        
        total_time = sum(times)
        avg_time = total_time / len(times)
        min_time = min(times)
        max_time = max(times)
        
        print("Done!")
        
        return {
            "name": name,
            "iterations": len(times),
            "total_time": total_time,
            "avg_time": avg_time,
            "min_time": min_time,
            "max_time": max_time,
        }
    
    def run_all_benchmarks(self):
        """Run all benchmarks."""
        print("=" * 60)
        print("🏁 Starting Benchmark Suite")
        print("=" * 60)
        print(f"Test Image: {self.test_image_path}")
        print(f"Iterations: {self.iterations}")
        print(f"Pillow Available: {HAS_PILLOW}")
        print(f"imgrs Available: {HAS_IMGRS}")
        print("=" * 60)
        print()
        
        # Create test image if it doesn't exist
        self.create_test_image()
        
        # Run benchmarks
        self.benchmark_open()
        self.benchmark_resize()
        self.benchmark_blur()
        self.benchmark_sharpen()
        self.benchmark_brightness()
        self.benchmark_rotate()
        self.benchmark_convert()
        self.benchmark_save()
        self.benchmark_composite_operations()
        
        return self.results
    
    def create_test_image(self):
        """Create a test image if it doesn't exist."""
        if not Path(self.test_image_path).exists():
            print("📸 Creating test image...")
            # Create with Pillow or imgrs
            if HAS_PILLOW:
                img = PILImage.new("RGB", (1920, 1080), color=(128, 128, 128))
                # Add some complexity
                from PIL import ImageDraw
                draw = ImageDraw.Draw(img)
                for i in range(0, 1920, 100):
                    draw.rectangle([i, 0, i+50, 1080], fill=(255, 0, 0))
                for i in range(0, 1080, 100):
                    draw.rectangle([0, i, 1920, i+50], fill=(0, 255, 0))
                img.save(self.test_image_path)
                print(f"✅ Created test image: {self.test_image_path}")
            elif HAS_IMGRS:
                img = ImgrsImage.new("RGB", (1920, 1080), color=(128, 128, 128, 255))
                # Add pattern
                for i in range(0, 1920, 100):
                    img = img.draw_rectangle(i, 0, 50, 1080, (255, 0, 0, 255))
                for i in range(0, 1080, 100):
                    img = img.draw_rectangle(0, i, 1920, 50, (0, 255, 0, 255))
                img.save(self.test_image_path)
                print(f"✅ Created test image with imgrs: {self.test_image_path}")
    
    def benchmark_open(self):
        """Benchmark image opening."""
        print("\n📖 Testing: Open Image")
        
        if HAS_PILLOW:
            result = self.time_operation(
                lambda: PILImage.open(self.test_image_path),
                "Pillow - Open"
            )
            self.results["open_pillow"] = result
            print(f"  Pillow: {result['avg_time']*1000:.2f}ms avg")
        
        if HAS_IMGRS:
            result = self.time_operation(
                lambda: ImgrsImage.open(self.test_image_path),
                "imgrs - Open"
            )
            self.results["open_imgrs"] = result
            print(f"  imgrs:  {result['avg_time']*1000:.2f}ms avg")
            
        if HAS_PILLOW and HAS_IMGRS:
            speedup = self.results["open_pillow"]["avg_time"] / self.results["open_imgrs"]["avg_time"]
            print(f"  ⚡ Speedup: {speedup:.2f}x")
    
    def benchmark_resize(self):
        """Benchmark resizing."""
        print("\n📏 Testing: Resize (1920x1080 → 800x600)")
        
        if HAS_PILLOW:
            img = PILImage.open(self.test_image_path)
            result = self.time_operation(
                lambda: img.resize((800, 600), PILImage.Resampling.LANCZOS),
                "Pillow - Resize"
            )
            self.results["resize_pillow"] = result
            print(f"  Pillow: {result['avg_time']*1000:.2f}ms avg")
        
        if HAS_IMGRS:
            img = ImgrsImage.open(self.test_image_path)
            result = self.time_operation(
                lambda: img.resize((800, 600), resample="LANCZOS"),
                "imgrs - Resize"
            )
            self.results["resize_imgrs"] = result
            print(f"  imgrs:  {result['avg_time']*1000:.2f}ms avg")
            
        if HAS_PILLOW and HAS_IMGRS:
            speedup = self.results["resize_pillow"]["avg_time"] / self.results["resize_imgrs"]["avg_time"]
            print(f"  ⚡ Speedup: {speedup:.2f}x")
    
    def benchmark_blur(self):
        """Benchmark blur filter."""
        print("\n🌫️  Testing: Blur (radius=5.0)")
        
        if HAS_PILLOW:
            from PIL import ImageFilter
            img = PILImage.open(self.test_image_path)
            result = self.time_operation(
                lambda: img.filter(ImageFilter.GaussianBlur(radius=5)),
                "Pillow - Blur"
            )
            self.results["blur_pillow"] = result
            print(f"  Pillow: {result['avg_time']*1000:.2f}ms avg")
        
        if HAS_IMGRS:
            img = ImgrsImage.open(self.test_image_path)
            result = self.time_operation(
                lambda: img.blur(5.0),
                "imgrs - Blur"
            )
            self.results["blur_imgrs"] = result
            print(f"  imgrs:  {result['avg_time']*1000:.2f}ms avg")
            
        if HAS_PILLOW and HAS_IMGRS:
            speedup = self.results["blur_pillow"]["avg_time"] / self.results["blur_imgrs"]["avg_time"]
            print(f"  ⚡ Speedup: {speedup:.2f}x")
    
    def benchmark_sharpen(self):
        """Benchmark sharpen filter."""
        print("\n🔪 Testing: Sharpen")
        
        if HAS_PILLOW:
            from PIL import ImageFilter
            img = PILImage.open(self.test_image_path)
            result = self.time_operation(
                lambda: img.filter(ImageFilter.SHARPEN),
                "Pillow - Sharpen"
            )
            self.results["sharpen_pillow"] = result
            print(f"  Pillow: {result['avg_time']*1000:.2f}ms avg")
        
        if HAS_IMGRS:
            img = ImgrsImage.open(self.test_image_path)
            result = self.time_operation(
                lambda: img.sharpen(2.0),
                "imgrs - Sharpen"
            )
            self.results["sharpen_imgrs"] = result
            print(f"  imgrs:  {result['avg_time']*1000:.2f}ms avg")
            
        if HAS_PILLOW and HAS_IMGRS:
            speedup = self.results["sharpen_pillow"]["avg_time"] / self.results["sharpen_imgrs"]["avg_time"]
            print(f"  ⚡ Speedup: {speedup:.2f}x")
    
    def benchmark_brightness(self):
        """Benchmark brightness adjustment."""
        print("\n💡 Testing: Brightness Adjustment")
        
        if HAS_PILLOW:
            from PIL import ImageEnhance
            img = PILImage.open(self.test_image_path)
            result = self.time_operation(
                lambda: ImageEnhance.Brightness(img).enhance(1.2),
                "Pillow - Brightness"
            )
            self.results["brightness_pillow"] = result
            print(f"  Pillow: {result['avg_time']*1000:.2f}ms avg")
        
        if HAS_IMGRS:
            img = ImgrsImage.open(self.test_image_path)
            result = self.time_operation(
                lambda: img.brightness(30),
                "imgrs - Brightness"
            )
            self.results["brightness_imgrs"] = result
            print(f"  imgrs:  {result['avg_time']*1000:.2f}ms avg")
            
        if HAS_PILLOW and HAS_IMGRS:
            speedup = self.results["brightness_pillow"]["avg_time"] / self.results["brightness_imgrs"]["avg_time"]
            print(f"  ⚡ Speedup: {speedup:.2f}x")
    
    def benchmark_rotate(self):
        """Benchmark rotation."""
        print("\n🔄 Testing: Rotate 90°")
        
        if HAS_PILLOW:
            img = PILImage.open(self.test_image_path)
            result = self.time_operation(
                lambda: img.rotate(-90, expand=True),
                "Pillow - Rotate"
            )
            self.results["rotate_pillow"] = result
            print(f"  Pillow: {result['avg_time']*1000:.2f}ms avg")
        
        if HAS_IMGRS:
            img = ImgrsImage.open(self.test_image_path)
            result = self.time_operation(
                lambda: img.rotate(90),
                "imgrs - Rotate"
            )
            self.results["rotate_imgrs"] = result
            print(f"  imgrs:  {result['avg_time']*1000:.2f}ms avg")
            
        if HAS_PILLOW and HAS_IMGRS:
            speedup = self.results["rotate_pillow"]["avg_time"] / self.results["rotate_imgrs"]["avg_time"]
            print(f"  ⚡ Speedup: {speedup:.2f}x")
    
    def benchmark_convert(self):
        """Benchmark color mode conversion."""
        print("\n🎨 Testing: Convert RGB → Grayscale")
        
        if HAS_PILLOW:
            img = PILImage.open(self.test_image_path)
            result = self.time_operation(
                lambda: img.convert("L"),
                "Pillow - Convert"
            )
            self.results["convert_pillow"] = result
            print(f"  Pillow: {result['avg_time']*1000:.2f}ms avg")
        
        if HAS_IMGRS:
            img = ImgrsImage.open(self.test_image_path)
            result = self.time_operation(
                lambda: img.convert("L"),
                "imgrs - Convert"
            )
            self.results["convert_imgrs"] = result
            print(f"  imgrs:  {result['avg_time']*1000:.2f}ms avg")
            
        if HAS_PILLOW and HAS_IMGRS:
            speedup = self.results["convert_pillow"]["avg_time"] / self.results["convert_imgrs"]["avg_time"]
            print(f"  ⚡ Speedup: {speedup:.2f}x")
    
    def benchmark_save(self):
        """Benchmark saving."""
        print("\n💾 Testing: Save to PNG")
        
        output_dir = Path(__file__).parent / "results"
        output_dir.mkdir(exist_ok=True)
        
        if HAS_PILLOW:
            img = PILImage.open(self.test_image_path)
            result = self.time_operation(
                lambda: img.save(output_dir / "test_pillow.png"),
                "Pillow - Save"
            )
            self.results["save_pillow"] = result
            print(f"  Pillow: {result['avg_time']*1000:.2f}ms avg")
        
        if HAS_IMGRS:
            img = ImgrsImage.open(self.test_image_path)
            result = self.time_operation(
                lambda: img.save(str(output_dir / "test_imgrs.png")),
                "imgrs - Save"
            )
            self.results["save_imgrs"] = result
            print(f"  imgrs:  {result['avg_time']*1000:.2f}ms avg")
            
        if HAS_PILLOW and HAS_IMGRS:
            speedup = self.results["save_pillow"]["avg_time"] / self.results["save_imgrs"]["avg_time"]
            print(f"  ⚡ Speedup: {speedup:.2f}x")
    
    def benchmark_composite_operations(self):
        """Benchmark complex composite operations."""
        print("\n🎯 Testing: Composite Pipeline (resize+blur+brightness+save)")
        
        output_dir = Path(__file__).parent / "results"
        
        if HAS_PILLOW:
            from PIL import ImageFilter, ImageEnhance
            
            def pillow_pipeline():
                img = PILImage.open(self.test_image_path)
                img = img.resize((800, 600), PILImage.Resampling.LANCZOS)
                img = img.filter(ImageFilter.GaussianBlur(radius=3))
                img = ImageEnhance.Brightness(img).enhance(1.1)
                img.save(output_dir / "pipeline_pillow.jpg")
                return img
            
            result = self.time_operation(pillow_pipeline, "Pillow - Pipeline")
            self.results["pipeline_pillow"] = result
            print(f"  Pillow: {result['avg_time']*1000:.2f}ms avg")
        
        if HAS_IMGRS:
            def imgrs_pipeline():
                img = ImgrsImage.open(self.test_image_path)
                img = (img
                    .resize((800, 600), resample="LANCZOS")
                    .blur(3.0)
                    .brightness(15))
                img.save(str(output_dir / "pipeline_imgrs.jpg"))
                return img
            
            result = self.time_operation(imgrs_pipeline, "imgrs - Pipeline")
            self.results["pipeline_imgrs"] = result
            print(f"  imgrs:  {result['avg_time']*1000:.2f}ms avg")
            
        if HAS_PILLOW and HAS_IMGRS:
            speedup = self.results["pipeline_pillow"]["avg_time"] / self.results["pipeline_imgrs"]["avg_time"]
            print(f"  ⚡ Speedup: {speedup:.2f}x")
    
    def save_results(self, output_file: str = "results/benchmark_results.json"):
        """Save results to JSON file."""
        output_path = Path(__file__).parent / output_file
        output_path.parent.mkdir(exist_ok=True)
        
        with open(output_path, 'w') as f:
            json.dump(self.results, f, indent=2)
        
        print(f"\n💾 Results saved to: {output_path}")
    
    def print_summary(self):
        """Print summary of all benchmarks."""
        print("\n" + "=" * 60)
        print("📊 BENCHMARK SUMMARY")
        print("=" * 60)
        
        operations = [
            ("open", "Open"),
            ("resize", "Resize"),
            ("blur", "Blur"),
            ("sharpen", "Sharpen"),
            ("brightness", "Brightness"),
            ("rotate", "Rotate"),
            ("convert", "Convert"),
            ("save", "Save"),
            ("pipeline", "Pipeline"),
        ]
        
        print(f"\n{'Operation':<15} {'Pillow (ms)':<15} {'imgrs (ms)':<15} {'Speedup':<10}")
        print("-" * 60)
        
        total_speedup = []
        
        for op, name in operations:
            pillow_key = f"{op}_pillow"
            imgrs_key = f"{op}_imgrs"
            
            if pillow_key in self.results and imgrs_key in self.results:
                pillow_time = self.results[pillow_key]["avg_time"] * 1000
                imgrs_time = self.results[imgrs_key]["avg_time"] * 1000
                speedup = pillow_time / imgrs_time
                total_speedup.append(speedup)
                
                print(f"{name:<15} {pillow_time:>12.2f}   {imgrs_time:>12.2f}   {speedup:>7.2f}x")
        
        if total_speedup:
            avg_speedup = sum(total_speedup) / len(total_speedup)
            print("-" * 60)
            print(f"{'AVERAGE':<15} {'':<15} {'':<15} {avg_speedup:>7.2f}x")
            print("\n🚀 imgrs is {:.1f}x faster on average!".format(avg_speedup))


def main():
    """Run benchmarks."""
    # Configuration
    test_image = "benchmark/test_images/test_1920x1080.jpg"
    iterations = 50  # Reduced for faster testing
    
    # Run benchmarks
    runner = BenchmarkRunner(test_image, iterations)
    results = runner.run_all_benchmarks()
    
    # Print summary
    runner.print_summary()
    
    # Save results
    runner.save_results()
    
    print("\n" + "=" * 60)
    print("✅ Benchmark Complete!")
    print("=" * 60)


if __name__ == "__main__":
    main()

