#!/usr/bin/env python3
"""
Run all Pillow compatibility tests
"""

import os
import subprocess
import sys

# Change to project root
os.chdir("/home/grandpa/Downloads/imgrs")

tests = [
    ("Basic Operations", "test/scripts/test_basic_operations.py"),
    ("Filters", "test/scripts/test_filters.py"),
    ("Text Features", "test/scripts/test_text_features.py"),
    ("Advanced Features", "test/scripts/test_advanced_imgrs.py"),
    ("Pillow Compatibility", "test/scripts/test_pillow_compatible.py"),
]

print("=" * 70)
print("🧪 RUNNING ALL PILLOW COMPATIBILITY TESTS")
print("=" * 70)
print()

passed = 0
failed = 0

for name, script in tests:
    print(f"Running: {name}...")
    print("-" * 70)

    result = subprocess.run(["python", script], capture_output=False, text=True)

    if result.returncode == 0:
        passed += 1
        print(f"✅ {name} - PASSED")
    else:
        failed += 1
        print(f"❌ {name} - FAILED")

    print()

print("=" * 70)
print("📊 FINAL RESULTS")
print("=" * 70)
print(f"Total Tests: {len(tests)}")
print(f"✅ Passed: {passed}")
print(f"❌ Failed: {failed}")
print(f"Success Rate: {(passed/len(tests)*100):.1f}%")
print("=" * 70)

sys.exit(0 if failed == 0 else 1)
