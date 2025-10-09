#!/usr/bin/env python3
"""
Fair benchmark comparing imgrs vs Pillow - only equivalent features.
"""

import time
import json
from pathlib import Path
from typing import Dict, Callable
import sys

sys.path.insert(0, str(Path(__file__).parent.parent))

try:
    from PIL import Image as PILImage, ImageFilter, ImageEnhance
    HAS_PILLOW = True
except ImportError:
    HAS_PILLOW = False
    print("⚠️ Pillow not installed")

try:
    from imgrs import Image as ImgrsImage
    HAS_IMGRS = True
except ImportError:
    HAS_IMGRS = False
    print("⚠️ imgrs not installed")

import numpy as np


class BenchmarkRunner:
    """Run fair benchmarks between imgrs and Pillow."""
    
    def __init__(self, test_image_path: str, iterations: int = 50):
        self.test_image_path = test_image_path
        self.iterations = iterations
        self.results = {}
        
    def time_operation(self, operation: Callable, name: str) -> Dict:
        """Time an operation."""
        times = []
        
        print(f"    {name}...", end=" ", flush=True)
        
        for i in range(self.iterations):
            start = time.perf_counter()
            try:
                operation()
                elapsed = time.perf_counter() - start
                times.append(elapsed)
            except Exception as e:
                print(f"ERROR: {e}")
                return {"name": name, "error": str(e), "avg_time": 0}
        
        avg_time = sum(times) / len(times)
        print(f"{avg_time*1000:.2f}ms")
        
        return {
            "name": name,
            "iterations": len(times),
            "total_time": sum(times),
            "avg_time": avg_time,
            "min_time": min(times),
            "max_time": max(times),
        }
    
    def create_test_image(self):
        """Create test image."""
        if Path(self.test_image_path).exists():
            return
            
        print("📸 Creating test image...")
        # Use numpy to create test image
        img_array = np.random.randint(0, 255, (1080, 1920, 3), dtype=np.uint8)
        
        if HAS_PILLOW:
            img = PILImage.fromarray(img_array)
            img.save(self.test_image_path, "JPEG")
            print(f"✅ Test image created: {self.test_image_path}\n")
    
    def run_all_benchmarks(self):
        """Run all benchmarks."""
        print("=" * 70)
        print("🏁 imgrs vs Pillow - Fair Performance Comparison")
        print("=" * 70)
        print(f"Test Image: 1920x1080 JPEG")
        print(f"Iterations: {self.iterations} per test")
        print("=" * 70)
        print()
        
        self.create_test_image()
        
        # Only test features that work identically in both
        self.benchmark_open()
        self.benchmark_save()
        self.benchmark_convert_grayscale()
        self.benchmark_rotate_90()
        self.benchmark_flip()
        self.benchmark_resize_simple()
        self.benchmark_crop()
        self.benchmark_split_channels()
        self.benchmark_composite_workflow()
        
        return self.results
    
    def benchmark_open(self):
        """Benchmark: Open image."""
        print("📖 Test 1: Open Image")
        
        if HAS_PILLOW:
            self.results["open_pillow"] = self.time_operation(
                lambda: PILImage.open(self.test_image_path),
                "Pillow"
            )
        
        if HAS_IMGRS:
            self.results["open_imgrs"] = self.time_operation(
                lambda: ImgrsImage.open(self.test_image_path),
                "imgrs"
            )
        
        self.print_speedup("open")
        print()
    
    def benchmark_save(self):
        """Benchmark: Save image."""
        print("💾 Test 2: Save to PNG")
        
        output_dir = Path(__file__).parent / "results"
        output_dir.mkdir(exist_ok=True)
        
        if HAS_PILLOW:
            img = PILImage.open(self.test_image_path)
            self.results["save_pillow"] = self.time_operation(
                lambda: img.save(output_dir / "test_pillow.png"),
                "Pillow"
            )
        
        if HAS_IMGRS:
            img = ImgrsImage.open(self.test_image_path)
            self.results["save_imgrs"] = self.time_operation(
                lambda: img.save(str(output_dir / "test_imgrs.png")),
                "imgrs"
            )
        
        self.print_speedup("save")
        print()
    
    def benchmark_convert_grayscale(self):
        """Benchmark: Convert to grayscale."""
        print("🎨 Test 3: Convert RGB → Grayscale")
        
        if HAS_PILLOW:
            img = PILImage.open(self.test_image_path)
            self.results["convert_pillow"] = self.time_operation(
                lambda: img.convert("L"),
                "Pillow"
            )
        
        if HAS_IMGRS:
            img = ImgrsImage.open(self.test_image_path)
            self.results["convert_imgrs"] = self.time_operation(
                lambda: img.convert("L"),
                "imgrs"
            )
        
        self.print_speedup("convert")
        print()
    
    def benchmark_rotate_90(self):
        """Benchmark: Rotate 90 degrees."""
        print("🔄 Test 4: Rotate 90°")
        
        if HAS_PILLOW:
            img = PILImage.open(self.test_image_path)
            self.results["rotate_pillow"] = self.time_operation(
                lambda: img.transpose(PILImage.Transpose.ROTATE_90),
                "Pillow"
            )
        
        if HAS_IMGRS:
            img = ImgrsImage.open(self.test_image_path)
            self.results["rotate_imgrs"] = self.time_operation(
                lambda: img.rotate(90),
                "imgrs"
            )
        
        self.print_speedup("rotate")
        print()
    
    def benchmark_flip(self):
        """Benchmark: Flip horizontal."""
        print("↔️  Test 5: Flip Horizontal")
        
        if HAS_PILLOW:
            img = PILImage.open(self.test_image_path)
            self.results["flip_pillow"] = self.time_operation(
                lambda: img.transpose(PILImage.Transpose.FLIP_LEFT_RIGHT),
                "Pillow"
            )
        
        if HAS_IMGRS:
            img = ImgrsImage.open(self.test_image_path)
            self.results["flip_imgrs"] = self.time_operation(
                lambda: img.transpose("FLIP_LEFT_RIGHT"),
                "imgrs"
            )
        
        self.print_speedup("flip")
        print()
    
    def benchmark_resize_simple(self):
        """Benchmark: Simple resize (bilinear)."""
        print("📏 Test 6: Resize 1920x1080 → 800x600 (BILINEAR)")
        
        if HAS_PILLOW:
            img = PILImage.open(self.test_image_path)
            self.results["resize_pillow"] = self.time_operation(
                lambda: img.resize((800, 600), PILImage.Resampling.BILINEAR),
                "Pillow"
            )
        
        if HAS_IMGRS:
            img = ImgrsImage.open(self.test_image_path)
            self.results["resize_imgrs"] = self.time_operation(
                lambda: img.resize((800, 600), resample="BILINEAR"),
                "imgrs"
            )
        
        self.print_speedup("resize")
        print()
    
    def benchmark_crop(self):
        """Benchmark: Crop image."""
        print("✂️  Test 7: Crop to 500x500")
        
        if HAS_PILLOW:
            img = PILImage.open(self.test_image_path)
            # Pillow: (left, top, right, bottom)
            self.results["crop_pillow"] = self.time_operation(
                lambda: img.crop((100, 100, 600, 600)),
                "Pillow"
            )
        
        if HAS_IMGRS:
            img = ImgrsImage.open(self.test_image_path)
            # imgrs: (x, y, width, height)
            self.results["crop_imgrs"] = self.time_operation(
                lambda: img.crop((100, 100, 500, 500)),
                "imgrs"
            )
        
        self.print_speedup("crop")
        print()
    
    def benchmark_split_channels(self):
        """Benchmark: Split into RGB channels."""
        print("🌈 Test 8: Split RGB Channels")
        
        if HAS_PILLOW:
            img = PILImage.open(self.test_image_path)
            self.results["split_pillow"] = self.time_operation(
                lambda: img.split(),
                "Pillow"
            )
        
        if HAS_IMGRS:
            img = ImgrsImage.open(self.test_image_path)
            self.results["split_imgrs"] = self.time_operation(
                lambda: img.split(),
                "imgrs"
            )
        
        self.print_speedup("split")
        print()
    
    def benchmark_composite_workflow(self):
        """Benchmark: Real-world workflow."""
        print("🎯 Test 9: Composite Workflow (open→resize→convert→save)")
        
        output_dir = Path(__file__).parent / "results"
        
        if HAS_PILLOW:
            def pillow_workflow():
                img = PILImage.open(self.test_image_path)
                img = img.resize((640, 480), PILImage.Resampling.BILINEAR)
                img = img.convert("L")
                img.save(output_dir / "workflow_pillow.jpg")
            
            self.results["workflow_pillow"] = self.time_operation(
                pillow_workflow,
                "Pillow"
            )
        
        if HAS_IMGRS:
            def imgrs_workflow():
                img = ImgrsImage.open(self.test_image_path)
                img = img.resize((640, 480), resample="BILINEAR")
                img = img.convert("L")
                img.save(str(output_dir / "workflow_imgrs.jpg"))
            
            self.results["workflow_imgrs"] = self.time_operation(
                imgrs_workflow,
                "imgrs"
            )
        
        self.print_speedup("workflow")
        print()
    
    def print_speedup(self, test_name: str):
        """Print speedup for a test."""
        pillow_key = f"{test_name}_pillow"
        imgrs_key = f"{test_name}_imgrs"
        
        if pillow_key in self.results and imgrs_key in self.results:
            p_time = self.results[pillow_key]["avg_time"]
            i_time = self.results[imgrs_key]["avg_time"]
            
            if "error" not in self.results[pillow_key] and "error" not in self.results[imgrs_key]:
                if i_time > 0:
                    speedup = p_time / i_time
                    if speedup > 1:
                        print(f"    ⚡ imgrs is {speedup:.2f}x FASTER")
                    else:
                        print(f"    ⚠️  Pillow is {1/speedup:.2f}x faster")
    
    def print_summary(self):
        """Print benchmark summary."""
        print("\n" + "=" * 70)
        print("📊 BENCHMARK RESULTS SUMMARY")
        print("=" * 70)
        
        tests = [
            ("open", "Open Image"),
            ("save", "Save PNG"),
            ("convert", "RGB → Grayscale"),
            ("rotate", "Rotate 90°"),
            ("flip", "Flip Horizontal"),
            ("resize", "Resize"),
            ("crop", "Crop"),
            ("split", "Split Channels"),
            ("workflow", "Composite Workflow"),
        ]
        
        print(f"\n{'Test':<25} {'Pillow':<12} {'imgrs':<12} {'Winner':<15}")
        print("-" * 70)
        
        imgrs_wins = 0
        pillow_wins = 0
        speedups = []
        
        for key, name in tests:
            p_key = f"{key}_pillow"
            i_key = f"{key}_imgrs"
            
            if p_key in self.results and i_key in self.results:
                if "error" not in self.results[p_key] and "error" not in self.results[i_key]:
                    p_time = self.results[p_key]["avg_time"] * 1000
                    i_time = self.results[i_key]["avg_time"] * 1000
                    speedup = p_time / i_time if i_time > 0 else 0
                    speedups.append(speedup)
                    
                    if speedup > 1:
                        winner = f"imgrs {speedup:.1f}x ⚡"
                        imgrs_wins += 1
                    elif speedup < 1:
                        winner = f"Pillow {1/speedup:.1f}x"
                        pillow_wins += 1
                    else:
                        winner = "Tie"
                    
                    print(f"{name:<25} {p_time:>10.2f}ms  {i_time:>10.2f}ms  {winner:<15}")
        
        print("-" * 70)
        
        if speedups:
            avg_speedup = sum(speedups) / len(speedups)
            print(f"\n{'AVERAGE SPEEDUP':<25} {'':<12} {'':<12} {avg_speedup:.2f}x")
            print(f"\n🏆 Results: imgrs wins {imgrs_wins}, Pillow wins {pillow_wins}")
            
            if avg_speedup > 1:
                print(f"🚀 imgrs is {avg_speedup:.1f}x faster overall!")
            elif avg_speedup < 1:
                print(f"⚠️  Pillow is {1/avg_speedup:.1f}x faster overall")
    
    def save_results(self):
        """Save detailed results."""
        output_file = Path(__file__).parent / "results" / "benchmark_results.json"
        output_file.parent.mkdir(exist_ok=True)
        
        with open(output_file, 'w') as f:
            json.dump(self.results, f, indent=2)
        
        print(f"\n💾 Detailed results: {output_file}")


def main():
    """Run benchmarks."""
    if not HAS_PILLOW or not HAS_IMGRS:
        print("\n❌ Both Pillow and imgrs must be installed!")
        print("Install with:")
        print("  pip install Pillow")
        print("  pip install -e .")
        return
    
    test_image = "benchmark/test_images/test_1920x1080.jpg"
    iterations = 50
    
    runner = BenchmarkRunner(test_image, iterations)
    runner.run_all_benchmarks()
    runner.print_summary()
    runner.save_results()
    
    print("\n" + "=" * 70)
    print("✅ Benchmark Complete!")
    print("=" * 70)


if __name__ == "__main__":
    main()

