"""
Comprehensive demo of rich text rendering features
Demonstrates all text capabilities with various styles
"""

import imgrs
import os

# Create output directory
os.makedirs("examples/output/text_demo", exist_ok=True)

print("="*70)
print("🎨 RICH TEXT RENDERING DEMO")
print("="*70)
print()

# Create a clean canvas
img = imgrs.Image.new("RGBA", (800, 1200), (255, 255, 255, 255))

# =============================================================================
# 1. BASIC TEXT
# =============================================================================
print("1. Basic Text")
img = img.add_text("Basic Text", (50, 50), size=48, color=(0, 0, 0, 255))
img = img.add_text("Red Text", (50, 110), size=36, color=(255, 0, 0, 255))
img = img.add_text("Semi-transparent", (50, 160), size=32, color=(0, 128, 255, 128))
img.save("examples/output/text_demo/01_basic.png")
print("   ✅ Basic text with colors")

# =============================================================================
# 2. TEXT WITH OUTLINE
# =============================================================================
print("2. Text with Outline")
img2 = imgrs.Image.new("RGBA", (800, 400), (255, 255, 255, 255))
img2 = img2.add_text_styled(
    "OUTLINED",
    (100, 50),
    size=80,
    color=(255, 255, 255, 255),
    outline=(0, 0, 0, 255, 3.0)
)
img2 = img2.add_text_styled(
    "Colored Outline",
    (100, 180),
    size=64,
    color=(255, 255, 0, 255),
    outline=(255, 0, 0, 255, 2.0)
)
img2.save("examples/output/text_demo/02_outline.png")
print("   ✅ Text with outlines")

# =============================================================================
# 3. TEXT WITH SHADOW
# =============================================================================
print("3. Text with Shadow")
img3 = imgrs.Image.new("RGBA", (800, 400), (240, 240, 240, 255))
img3 = img3.add_text_styled(
    "Shadow Text",
    (100, 50),
    size=72,
    color=(0, 0, 0, 255),
    shadow=(3, 3, 100, 100, 100, 180)
)
img3 = img3.add_text_styled(
    "Colorful Shadow",
    (100, 180),
    size=60,
    color=(255, 100, 0, 255),
    shadow=(5, 5, 0, 100, 255, 200)
)
img3.save("examples/output/text_demo/03_shadow.png")
print("   ✅ Text with shadows")

# =============================================================================
# 4. TEXT WITH BACKGROUND
# =============================================================================
print("4. Text with Background")
img4 = imgrs.Image.open("examples/img/gradient.png")
img4 = img4.add_text_styled(
    "Background",
    (50, 50),
    size=48,
    color=(255, 255, 255, 255),
    background=(0, 0, 0, 180)
)
img4 = img4.add_text_styled(
    "Colored BG",
    (50, 120),
    size=40,
    color=(255, 255, 255, 255),
    background=(0, 128, 255, 200)
)
img4.save("examples/output/text_demo/04_background.png")
print("   ✅ Text with backgrounds")

# =============================================================================
# 5. TEXT ALIGNMENT
# =============================================================================
print("5. Text Alignment")
img5 = imgrs.Image.new("RGBA", (600, 400), (255, 255, 255, 255))

img5 = img5.add_text_styled("Left Aligned", (50, 50), size=36, color=(0, 0, 0, 255), align="left")
img5 = img5.add_text_styled("Center Aligned", (300, 120), size=36, color=(0, 0, 0, 255), align="center")
img5 = img5.add_text_styled("Right Aligned", (550, 190), size=36, color=(0, 0, 0, 255), align="right")
img5.save("examples/output/text_demo/05_alignment.png")
print("   ✅ Text alignment (left, center, right)")

# =============================================================================
# 6. CENTERED TEXT
# =============================================================================
print("6. Centered Text")
img6 = imgrs.Image.new("RGBA", (800, 300), (255, 255, 255, 255))
img6 = img6.add_text_centered("Centered Title", 50, size=56, color=(0, 0, 128, 255))
img6 = img6.add_text_centered("Subtitle", 130, size=36, color=(128, 128, 128, 255))
img6.save("examples/output/text_demo/06_centered.png")
print("   ✅ Horizontally centered text")

# =============================================================================
# 7. MULTILINE TEXT
# =============================================================================
print("7. Multiline Text")
img7 = imgrs.Image.new("RGBA", (800, 500), (255, 255, 255, 255))
multiline_text = """Line 1: First Line
Line 2: Second Line
Line 3: Third Line
Line 4: Fourth Line"""

img7 = img7.add_text_multiline(
    multiline_text,
    (50, 50),
    size=32,
    color=(0, 0, 0, 255),
    line_spacing=1.5
)
img7.save("examples/output/text_demo/07_multiline.png")
print("   ✅ Multi-line text")

# =============================================================================
# 8. TEXT OPACITY
# =============================================================================
print("8. Text Opacity")
img8 = imgrs.Image.open("examples/img/gradient.png")
img8 = img8.add_text_styled("Opacity 1.0", (50, 30), size=48, color=(255, 255, 255, 255), opacity=1.0)
img8 = img8.add_text_styled("Opacity 0.7", (50, 90), size=48, color=(255, 255, 255, 255), opacity=0.7)
img8 = img8.add_text_styled("Opacity 0.4", (50, 150), size=48, color=(255, 255, 255, 255), opacity=0.4)
img8.save("examples/output/text_demo/08_opacity.png")
print("   ✅ Text with varying opacity")

# =============================================================================
# 9. COMBINED EFFECTS
# =============================================================================
print("9. Combined Effects")
img9 = imgrs.Image.new("RGBA", (800, 400), (30, 30, 50, 255))
img9 = img9.add_text_styled(
    "EPIC TEXT",
    (200, 100),
    size=80,
    color=(255, 215, 0, 255),  # Gold
    outline=(255, 140, 0, 255, 4.0),  # Dark orange outline
    shadow=(5, 5, 0, 0, 0, 200),  # Black shadow
    align="center"
)
img9.save("examples/output/text_demo/09_combined.png")
print("   ✅ Combined effects (outline + shadow)")

# =============================================================================
# 10. REAL-WORLD EXAMPLES
# =============================================================================
print("10. Real-World Examples")

# Meme style
meme = imgrs.Image.open("examples/img/gradient.png")
meme = meme.add_text_styled(
    "TOP TEXT",
    (meme.width // 2, 20),
    size=56,
    color=(255, 255, 255, 255),
    outline=(0, 0, 0, 255, 3.0),
    align="center"
)
meme = meme.add_text_styled(
    "BOTTOM TEXT",
    (meme.width // 2, meme.height - 70),
    size=56,
    color=(255, 255, 255, 255),
    outline=(0, 0, 0, 255, 3.0),
    align="center"
)
meme.save("examples/output/text_demo/10_meme_style.png")
print("   ✅ Meme-style text")

# Quote style
quote_bg = imgrs.Image.new("RGBA", (600, 400), (40, 44, 52, 255))
quote_text = "The only way to do\ngreat work is to\nlove what you do."
quote_bg = quote_bg.add_text_multiline(
    quote_text,
    (300, 150),
    size=36,
    color=(230, 230, 230, 255),
    line_spacing=1.4,
    align="center"
)
quote_bg = quote_bg.add_text_styled(
    "- Steve Jobs",
    (300, 320),
    size=24,
    color=(180, 180, 180, 255),
    align="center"
)
quote_bg.save("examples/output/text_demo/11_quote.png")
print("   ✅ Quote-style layout")

# Banner style
banner = imgrs.Image.new("RGBA", (800, 200), (0, 120, 215, 255))
banner = banner.add_text_styled(
    "SPECIAL OFFER",
    (400, 40),
    size=64,
    color=(255, 255, 255, 255),
    align="center",
    shadow=(3, 3, 0, 0, 0, 150)
)
banner = banner.add_text_styled(
    "50% OFF",
    (400, 120),
    size=48,
    color=(255, 255, 0, 255),
    align="center",
    outline=(255, 140, 0, 255, 2.0)
)
banner.save("examples/output/text_demo/12_banner.png")
print("   ✅ Banner-style design")

print()
print("="*70)
print("✨ TEXT RENDERING DEMO COMPLETE!")
print("="*70)
print()
print("Features Demonstrated:")
print("  ✅ Basic text rendering")
print("  ✅ Text colors (RGB + Alpha)")
print("  ✅ Text outlines")
print("  ✅ Text shadows")
print("  ✅ Background colors")
print("  ✅ Text alignment (left, center, right)")
print("  ✅ Centered text")
print("  ✅ Multi-line text")
print("  ✅ Text opacity")
print("  ✅ Combined effects")
print("  ✅ Real-world examples (memes, quotes, banners)")
print()
print(f"📁 Output: examples/output/text_demo/ ({12} files)")
print("="*70)

