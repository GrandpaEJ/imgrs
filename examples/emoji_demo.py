"""
Emoji Feature Demo - Add emojis to photos!

Demonstrates the new high-performance emoji overlay feature.
"""

import imgrs
import os

# Create output directory
output_dir = "examples/output/emoji_demo"
os.makedirs(output_dir, exist_ok=True)

print("="*70)
print("🎨 EMOJI FEATURE DEMO - Add Emojis to Photos! 🎉")
print("="*70)
print()

# Load test image
print("Loading test image...")
img = imgrs.Image.open("examples/img/gradient.png")
print(f"✓ Image loaded: {img.width}x{img.height}")
print()

# ========================================================================
# BASIC EMOJI USAGE
# ========================================================================
print("### BASIC EMOJI USAGE ###")
print()

# 1. Single emoji with preset name
print("1. Adding a smile emoji...")
result = img.add_emoji('smile', x=50, y=50, size=80, opacity=1.0)
result.save(f"{output_dir}/01_single_smile.png")
print("   ✓ Saved: 01_single_smile.png")

# 2. Heart emoji
print("2. Adding a heart emoji...")
result = img.add_emoji('heart', x=150, y=50, size=80, opacity=1.0)
result.save(f"{output_dir}/02_heart.png")
print("   ✓ Saved: 02_heart.png")

# 3. Fire emoji
print("3. Adding a fire emoji...")
result = img.add_emoji('fire', x=50, y=150, size=80, opacity=1.0)
result.save(f"{output_dir}/03_fire.png")
print("   ✓ Saved: 03_fire.png")

# 4. Thumbs up
print("4. Adding a thumbs up...")
result = img.add_emoji('thumbsup', x=150, y=150, size=80, opacity=1.0)
result.save(f"{output_dir}/04_thumbsup.png")
print("   ✓ Saved: 04_thumbsup.png")

print()

# ========================================================================
# EMOJI SIZES AND OPACITY
# ========================================================================
print("### EMOJI SIZES AND OPACITY ###")
print()

# 5. Different sizes
print("5. Different emoji sizes...")
result = img.copy()
result = result.add_emoji('star', x=20, y=20, size=30, opacity=1.0)
result = result.add_emoji('star', x=70, y=20, size=50, opacity=1.0)
result = result.add_emoji('star', x=140, y=20, size=70, opacity=1.0)
result = result.add_emoji('star', x=230, y=20, size=90, opacity=1.0)
result.save(f"{output_dir}/05_size_variations.png")
print("   ✓ Saved: 05_size_variations.png")

# 6. Different opacities
print("6. Different emoji opacities...")
result = img.copy()
result = result.add_emoji('moon', x=20, y=60, size=60, opacity=1.0)
result = result.add_emoji('moon', x=90, y=60, size=60, opacity=0.75)
result = result.add_emoji('moon', x=160, y=60, size=60, opacity=0.5)
result = result.add_emoji('moon', x=230, y=60, size=60, opacity=0.25)
result.save(f"{output_dir}/06_opacity_variations.png")
print("   ✓ Saved: 06_opacity_variations.png")

print()

# ========================================================================
# MULTIPLE EMOJIS
# ========================================================================
print("### MULTIPLE EMOJIS ###")
print()

# 7. Multiple emojis one by one
print("7. Adding multiple emojis (chaining)...")
result = (img
    .add_emoji('smile', x=20, y=20, size=50)
    .add_emoji('heart', x=90, y=20, size=50)
    .add_emoji('fire', x=160, y=20, size=50)
    .add_emoji('star', x=230, y=20, size=50)
    .add_emoji('party', x=20, y=90, size=50)
    .add_emoji('gift', x=90, y=90, size=50)
    .add_emoji('trophy', x=160, y=90, size=50)
    .add_emoji('camera', x=230, y=90, size=50)
)
result.save(f"{output_dir}/07_multiple_chained.png")
print("   ✓ Saved: 07_multiple_chained.png")

# 8. Batch add emojis
print("8. Adding emojis in batch...")
emojis = [
    ('pizza', 30, 30, 60, 1.0),
    ('burger', 120, 30, 60, 1.0),
    ('cake', 210, 30, 60, 1.0),
    ('coffee', 30, 110, 60, 1.0),
    ('sun', 120, 110, 60, 1.0),
    ('rainbow', 210, 110, 60, 1.0),
]
result = img.add_emojis(emojis)
result.save(f"{output_dir}/08_batch_emojis.png")
print("   ✓ Saved: 08_batch_emojis.png")

print()

# ========================================================================
# EMOJI CATEGORIES
# ========================================================================
print("### EMOJI CATEGORIES ###")
print()

# 9. Smileys
print("9. Smiley emojis...")
result = img.copy()
smileys = [
    ('smile', 20, 20, 50, 1.0),
    ('grin', 80, 20, 50, 1.0),
    ('joy', 140, 20, 50, 1.0),
    ('cool', 200, 20, 50, 1.0),
    ('wink', 20, 80, 50, 1.0),
    ('thinking', 80, 80, 50, 1.0),
    ('laughing', 140, 80, 50, 1.0),
    ('hearteyes', 200, 80, 50, 1.0),
]
result = result.add_emojis(smileys)
result.save(f"{output_dir}/09_smileys.png")
print("   ✓ Saved: 09_smileys.png")

# 10. Hearts
print("10. Heart emojis...")
result = img.copy()
hearts = [
    ('heart', 30, 40, 60, 1.0),
    ('blueheart', 100, 40, 60, 1.0),
    ('greenheart', 170, 40, 60, 1.0),
    ('yellowheart', 240, 40, 60, 1.0),
    ('purpleheart', 30, 110, 60, 1.0),
    ('orangeheart', 100, 110, 60, 1.0),
]
result = result.add_emojis(hearts)
result.save(f"{output_dir}/10_hearts.png")
print("   ✓ Saved: 10_hearts.png")

# 11. Nature
print("11. Nature emojis...")
result = img.copy()
nature = [
    ('sun', 25, 25, 55, 1.0),
    ('moon', 90, 25, 55, 1.0),
    ('star', 155, 25, 55, 1.0),
    ('rainbow', 220, 25, 55, 1.0),
    ('flower', 25, 95, 55, 1.0),
    ('sparkles', 90, 95, 55, 1.0),
]
result = result.add_emojis(nature)
result.save(f"{output_dir}/11_nature.png")
print("   ✓ Saved: 11_nature.png")

# 12. Gestures
print("12. Gesture emojis...")
result = img.copy()
gestures = [
    ('thumbsup', 30, 30, 60, 1.0),
    ('thumbsdown', 100, 30, 60, 1.0),
    ('ok', 170, 30, 60, 1.0),
    ('victory', 240, 30, 60, 1.0),
    ('fire', 30, 100, 60, 1.0),
    ('wave', 100, 100, 60, 1.0),
]
result = result.add_emojis(gestures)
result.save(f"{output_dir}/12_gestures.png")
print("   ✓ Saved: 12_gestures.png")

print()

# ========================================================================
# CREATIVE EXAMPLES
# ========================================================================
print("### CREATIVE EXAMPLES ###")
print()

# 13. Rating/reaction strip
print("13. Rating strip...")
result = img.copy()
ratings = [
    ('smile', 20, 80, 70, 1.0),
    ('joy', 100, 80, 70, 1.0),
    ('heart', 180, 80, 70, 1.0),
]
result = result.add_emojis(ratings)
result.save(f"{output_dir}/13_rating_strip.png")
print("   ✓ Saved: 13_rating_strip.png")

# 14. Corner decorations
print("14. Corner decorations...")
result = img.copy()
corners = [
    ('star', 10, 10, 40, 0.8),           # Top-left
    ('star', 250, 10, 40, 0.8),          # Top-right
    ('star', 10, 160, 40, 0.8),          # Bottom-left
    ('star', 250, 160, 40, 0.8),         # Bottom-right
]
result = result.add_emojis(corners)
result.save(f"{output_dir}/14_corner_decorations.png")
print("   ✓ Saved: 14_corner_decorations.png")

# 15. Quick add (simplified API)
print("15. Quick add (simplified)...")
result = img.add_emoji_quick('party', x=100, y=80, size=100)
result.save(f"{output_dir}/15_quick_add.png")
print("   ✓ Saved: 15_quick_add.png")

print()

# ========================================================================
# EMOJI WITH OTHER EFFECTS
# ========================================================================
print("### EMOJI + FILTERS ###")
print()

# 16. Emoji on blurred background
print("16. Emoji on blurred background...")
result = img.blur(5.0).add_emoji('heart', x=120, y=80, size=100, opacity=1.0)
result.save(f"{output_dir}/16_emoji_on_blur.png")
print("   ✓ Saved: 16_emoji_on_blur.png")

# 17. Emoji with vignette
print("17. Emoji with vignette...")
result = img.add_emoji('sun', x=120, y=80, size=100).vignette(0.5, 0.8)
result.save(f"{output_dir}/17_emoji_vignette.png")
print("   ✓ Saved: 17_emoji_vignette.png")

# 18. Multiple emojis + sepia
print("18. Emojis + sepia tone...")
result = img.add_emojis([
    ('camera', 50, 50, 60, 1.0),
    ('heart', 180, 50, 60, 1.0),
    ('star', 50, 120, 60, 1.0),
    ('sparkles', 180, 120, 60, 1.0),
]).sepia(0.7)
result.save(f"{output_dir}/18_emojis_sepia.png")
print("   ✓ Saved: 18_emojis_sepia.png")

print()

# ========================================================================
# SUMMARY
# ========================================================================
print("="*70)
print("✨ EMOJI DEMO COMPLETE! ✨")
print("="*70)
print()
print(f"Created 18 demo images in: {output_dir}/")
print()
print("Features demonstrated:")
print("  ✓ Single emoji placement")
print("  ✓ Size control (30-100px)")
print("  ✓ Opacity control (0.0-1.0)")
print("  ✓ Multiple emojis (chaining)")
print("  ✓ Batch emoji addition")
print("  ✓ 70+ emoji presets available")
print("  ✓ Emoji categories (smileys, hearts, nature, etc.)")
print("  ✓ Creative layouts")
print("  ✓ Combination with other filters")
print()
print("Available Emoji Categories:")
print("  • Smileys (10): smile, grin, joy, laughing, hearteyes, etc.")
print("  • Hearts (10): heart, blueheart, greenheart, purpleheart, etc.")
print("  • Gestures (10): thumbsup, thumbsdown, fire, victory, etc.")
print("  • Nature (10): sun, moon, star, rainbow, flower, sparkles, etc.")
print("  • Food (10): pizza, burger, cake, coffee, etc.")
print("  • Activities (10): party, gift, trophy, camera, etc.")
print("  • Symbols (10): check, cross, warning, etc.")
print()
print("Usage:")
print("  img.add_emoji('heart', x=100, y=100, size=80, opacity=1.0)")
print("  img.add_emoji_quick('smile', x=50, y=50, size=64)")
print("  img.add_emojis([('fire', 10, 10, 60, 1.0), ...])")
print()
print("🎉 Performance-optimized Rust implementation!")
print("="*70)

