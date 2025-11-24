import os
import subprocess
import sys

examples_dir = "examples"
scripts = [
    f
    for f in os.listdir(examples_dir)
    if f.endswith(".py") and f != "run_all_examples.py"
]
scripts.sort()

print(f"Found {len(scripts)} example scripts.")
print("-" * 60)

passed = []
failed = []

for script in scripts:
    print(f"Running {script}...", end=" ", flush=True)
    try:
        # Run with timeout to prevent hanging
        result = subprocess.run(
            [sys.executable, os.path.join(examples_dir, script)],
            capture_output=True,
            text=True,
            timeout=30,
            cwd=os.getcwd(),
        )

        if result.returncode == 0:
            print("✅ PASS")
            passed.append(script)
        else:
            print("❌ FAIL")
            print(f"  Error: {result.stderr[:500]}...")  # Show first 500 chars of error
            failed.append(script)

    except subprocess.TimeoutExpired:
        print("⏰ TIMEOUT")
        failed.append(f"{script} (timeout)")
    except Exception as e:
        print(f"❌ ERROR: {e}")
        failed.append(f"{script} (error)")

print("-" * 60)
print(f"Summary: {len(passed)} passed, {len(failed)} failed")

if failed:
    print("\nFailed scripts:")
    for f in failed:
        print(f"- {f}")
    sys.exit(1)
else:
    print("\nAll examples passed successfully!")
    sys.exit(0)
