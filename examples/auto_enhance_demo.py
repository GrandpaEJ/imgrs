"""
Auto-Enhancement Demo - Automatic Image Optimization

Demonstrates automatic image enhancement features:
- Histogram Equalization (হিস্টোগ্রাম ইকুইলাইজেশন)
- Auto Contrast (স্বয়ংক্রিয় কন্ট্রাস্ট)
- Auto Brightness
- Exposure Adjustment
- Auto-Enhance (Combined optimization)
"""

import imgrs
import os

# Create output directory
output_dir = "examples/output/auto_enhance"
os.makedirs(output_dir, exist_ok=True)

print("="*70)
print("🎨 AUTO-ENHANCEMENT DEMO - Automatic Image Optimization")
print("="*70)
print()

# Load test image
print("Loading test image...")
img = imgrs.Image.open("examples/img/gradient.png")
print(f"✓ Image loaded: {img.width}x{img.height}")
print()

# Save original for comparison
img.save(f"{output_dir}/00_original.png")

# ========================================================================
# HISTOGRAM EQUALIZATION (হিস্টোগ্রাম ইকুইলাইজেশন)
# ========================================================================
print("### HISTOGRAM EQUALIZATION ###")
print()

print("1. Histogram Equalization...")
result = img.histogram_equalization()
result.save(f"{output_dir}/01_histogram_equalization.png")
print("   ✓ Saved: 01_histogram_equalization.png")
print("   → Enhances contrast by redistributing pixel intensities")

print()

# ========================================================================
# AUTO CONTRAST (স্বয়ংক্রিয় কন্ট্রাস্ট)
# ========================================================================
print("### AUTO CONTRAST ###")
print()

print("2. Auto Contrast...")
result = img.auto_contrast()
result.save(f"{output_dir}/02_auto_contrast.png")
print("   ✓ Saved: 02_auto_contrast.png")
print("   → Automatically stretches color range to 0-255")

print()

# ========================================================================
# AUTO BRIGHTNESS
# ========================================================================
print("### AUTO BRIGHTNESS ###")
print()

print("3. Auto Brightness...")
result = img.auto_brightness()
result.save(f"{output_dir}/03_auto_brightness.png")
print("   ✓ Saved: 03_auto_brightness.png")
print("   → Automatically adjusts to optimal brightness level")

print()

# ========================================================================
# EXPOSURE ADJUSTMENT
# ========================================================================
print("### EXPOSURE ADJUSTMENT ###")
print()

print("4. Exposure +1.0 (Brighten)...")
result = img.exposure_adjust(1.0)
result.save(f"{output_dir}/04_exposure_plus_1.png")
print("   ✓ Saved: 04_exposure_plus_1.png")

print("5. Exposure +0.5 (Slightly Brighten)...")
result = img.exposure_adjust(0.5)
result.save(f"{output_dir}/05_exposure_plus_0_5.png")
print("   ✓ Saved: 05_exposure_plus_0_5.png")

print("6. Exposure -0.5 (Slightly Darken)...")
result = img.exposure_adjust(-0.5)
result.save(f"{output_dir}/06_exposure_minus_0_5.png")
print("   ✓ Saved: 06_exposure_minus_0_5.png")

print("7. Exposure -1.0 (Darken)...")
result = img.exposure_adjust(-1.0)
result.save(f"{output_dir}/07_exposure_minus_1.png")
print("   ✓ Saved: 07_exposure_minus_1.png")

print()

# ========================================================================
# AUTO LEVEL
# ========================================================================
print("### AUTO LEVEL ###")
print()

print("8. Auto Level (1% clip)...")
result = img.auto_level(0.01, 0.01)
result.save(f"{output_dir}/08_auto_level.png")
print("   ✓ Saved: 08_auto_level.png")
print("   → Optimizes dynamic range with percentile clipping")

print()

# ========================================================================
# NORMALIZE
# ========================================================================
print("### NORMALIZE ###")
print()

print("9. Normalize...")
result = img.normalize()
result.save(f"{output_dir}/09_normalize.png")
print("   ✓ Saved: 09_normalize.png")
print("   → Stretches to full 0-255 range")

print()

# ========================================================================
# AUTO ENHANCE (COMBINED)
# ========================================================================
print("### AUTO ENHANCE (COMBINED OPTIMIZATION) ###")
print()

print("10. Auto Enhance (Full Automatic Optimization)...")
result = img.auto_enhance()
result.save(f"{output_dir}/10_auto_enhance.png")
print("   ✓ Saved: 10_auto_enhance.png")
print("   → Combines auto-level + histogram equalization + auto-brightness")

print()

# ========================================================================
# SMART ENHANCE
# ========================================================================
print("### SMART ENHANCE (WITH STRENGTH CONTROL) ###")
print()

print("11. Smart Enhance (strength=0.5)...")
result = img.smart_enhance(0.5)
result.save(f"{output_dir}/11_smart_enhance_0_5.png")
print("   ✓ Saved: 11_smart_enhance_0_5.png")

print("12. Smart Enhance (strength=1.0)...")
result = img.smart_enhance(1.0)
result.save(f"{output_dir}/12_smart_enhance_1_0.png")
print("   ✓ Saved: 12_smart_enhance_1_0.png")

print()

# ========================================================================
# AUTO WHITE BALANCE
# ========================================================================
print("### AUTO WHITE BALANCE ###")
print()

print("13. Auto White Balance...")
result = img.auto_white_balance()
result.save(f"{output_dir}/13_auto_white_balance.png")
print("   ✓ Saved: 13_auto_white_balance.png")
print("   → Corrects color temperature automatically")

print()

# ========================================================================
# COMBINED WORKFLOWS
# ========================================================================
print("### COMBINED WORKFLOWS ###")
print()

print("14. Quick Fix (auto_enhance + slight sharpen)...")
result = img.auto_enhance().sharpen(1.2)
result.save(f"{output_dir}/14_quick_fix.png")
print("   ✓ Saved: 14_quick_fix.png")

print("15. Pro Enhancement (normalize + smart_enhance + white_balance)...")
result = img.normalize().smart_enhance(0.7).auto_white_balance()
result.save(f"{output_dir}/15_pro_enhancement.png")
print("   ✓ Saved: 15_pro_enhancement.png")

print("16. HDR-like (exposure + contrast)...")
result = img.exposure_adjust(0.3).auto_contrast().smart_enhance(0.6)
result.save(f"{output_dir}/16_hdr_like.png")
print("   ✓ Saved: 16_hdr_like.png")

print()

# ========================================================================
# SUMMARY
# ========================================================================
print("="*70)
print("✨ AUTO-ENHANCEMENT DEMO COMPLETE! ✨")
print("="*70)
print()
print(f"Created 17 demo images in: {output_dir}/")
print()
print("Features Demonstrated:")
print("  ✅ Histogram Equalization (হিস্টোগ্রাম ইকুইলাইজেশন)")
print("  ✅ Auto Contrast (স্বয়ংক্রিয় কন্ট্রাস্ট)")
print("  ✅ Auto Brightness")
print("  ✅ Exposure Adjustment")
print("  ✅ Auto Level")
print("  ✅ Normalize")
print("  ✅ Auto Enhance (Combined)")
print("  ✅ Smart Enhance (with strength)")
print("  ✅ Auto White Balance")
print()
print("Usage Examples:")
print("  # One-click enhancement")
print("  img.auto_enhance().save('enhanced.jpg')")
print()
print("  # Adjust exposure")
print("  img.exposure_adjust(1.0).save('brighter.jpg')")
print()
print("  # Smart enhancement")
print("  img.smart_enhance(0.7).save('smart.jpg')")
print()
print("🚀 All features use high-performance Rust implementation!")
print("="*70)

