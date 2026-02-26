# NOTE: VIBEZZZZZ from claude who made this benchmark file

import time
import os
import glob

import numpy as np
import librosa
from pydub import AudioSegment

from decoding_backend.decoding_backend import get_streams, get_tracks

# ============================================================
# Configuration
# ============================================================
TRACKS_FOLDER = "../test_tracks"
TARGET_WIDTH = 800
AUDIO_EXTENSIONS = ("*.mp3", "*.wav", "*.flac", "*.ogg", "*.aac", "*.m4a")
BENCHMARK_RUNS = 5


# ============================================================
# 1. Get all audio files in a folder
# ============================================================
def get_audio_files(folder: str) -> list[str]:
    """Recursively find all audio files in the given folder."""
    files = []
    for ext in AUDIO_EXTENSIONS:
        files.extend(glob.glob(os.path.join(folder, "**", ext), recursive=True))
    return sorted(files)


# ============================================================
# 2a. Python backend — librosa
# ============================================================
def generate_waveform_librosa(file_path: str, target_width: int) -> np.ndarray:
    """Generate a waveform (peak envelope) using librosa."""
    y, sr = librosa.load(file_path, sr=None, mono=True)
    # Split the signal into `target_width` chunks and take the max absolute value
    chunk_size = max(1, len(y) // target_width)
    # Trim to an even multiple of chunk_size
    trimmed = y[: chunk_size * target_width]
    chunks = trimmed.reshape(target_width, chunk_size)
    peaks = np.max(np.abs(chunks), axis=1)
    return peaks


def run_librosa(audio_files: list[str], target_width: int) -> list[np.ndarray]:
    """Process all files with librosa."""
    results = []
    for f in audio_files:
        results.append(generate_waveform_librosa(f, target_width))
    return results


# ============================================================
# 2b. Python backend — pydub + numpy
# ============================================================
def generate_waveform_pydub(file_path: str, target_width: int) -> np.ndarray:
    """Generate a waveform (peak envelope) using pydub."""
    audio = AudioSegment.from_file(file_path)
    audio = audio.set_channels(1)  # mono
    samples = np.array(audio.get_array_of_samples(), dtype=np.float32)
    # Normalize to [-1, 1]
    samples = samples / (2**15)
    chunk_size = max(1, len(samples) // target_width)
    trimmed = samples[: chunk_size * target_width]
    chunks = trimmed.reshape(target_width, chunk_size)
    peaks = np.max(np.abs(chunks), axis=1)
    return peaks


def run_pydub(audio_files: list[str], target_width: int) -> list[np.ndarray]:
    """Process all files with pydub."""
    results = []
    for f in audio_files:
        results.append(generate_waveform_pydub(f, target_width))
    return results


# ============================================================
# 2c. Rust backend via PyO3
# ============================================================
def run_rust(crawler_objects: list, target_width: int) -> list:
    """Call the Rust-backed get_streams."""
    return get_streams(crawler_objects, target_width)


# ============================================================
# 3. Benchmark harness
# ============================================================
def benchmark(func, *args, runs: int = BENCHMARK_RUNS, label: str = ""):
    """Time a function over `runs` iterations and print stats."""
    times = []
    result = None
    for i in range(runs):
        start = time.perf_counter()
        result = func(*args)
        elapsed = time.perf_counter() - start
        times.append(elapsed)
        print(f"  [{label}] Run {i + 1}/{runs}: {elapsed:.4f}s")

    avg = sum(times) / len(times)
    best = min(times)
    worst = max(times)
    print(f"\n  [{label}] Summary ({runs} runs):")
    print(f"    Average : {avg:.4f}s")
    print(f"    Best    : {best:.4f}s")
    print(f"    Worst   : {worst:.4f}s\n")
    return result, avg


# ============================================================
# Main
# ============================================================
def main():
    print("=" * 60)
    print("  Waveform Generation Benchmark")
    print(f"  Runs per backend: {BENCHMARK_RUNS}")
    print("=" * 60)

    # --- Discover files ---
    print(f"\n[1] Scanning '{TRACKS_FOLDER}' for audio files...")
    audio_files = get_audio_files(TRACKS_FOLDER)
    print(f"    Found {len(audio_files)} file(s)")

    if not audio_files:
        print("    No audio files found — exiting.")
        return

    for f in audio_files:
        print(f"      • {f}")

    # --- Build Rust Crawler objects ---
    # ⚠️ Adjust this to match your Crawler constructor
    crawler_objects = get_tracks()

    # --- Benchmark: librosa ---
    print("\n" + "-" * 60)
    print("  Backend: librosa")
    print("-" * 60 + "\n")
    librosa_result, librosa_avg = benchmark(
        run_librosa, audio_files, TARGET_WIDTH, label="librosa"
    )

    # --- Benchmark: pydub ---
    print("-" * 60)
    print("  Backend: pydub + numpy")
    print("-" * 60 + "\n")
    pydub_result, pydub_avg = benchmark(
        run_pydub, audio_files, TARGET_WIDTH, label="pydub"
    )

    # --- Benchmark: Rust ---
    print("-" * 60)
    print("  Backend: Rust (PyO3)")
    print("-" * 60 + "\n")
    rust_result, rust_avg = benchmark(
        run_rust, crawler_objects, TARGET_WIDTH, label="rust"
    )

    # --- Comparison table ---
    print("=" * 60)
    print("  Comparison (average over {} runs)".format(BENCHMARK_RUNS))
    print("=" * 60)
    print(f"  {'Backend':<20} {'Avg Time (s)':<15} {'Speedup vs librosa'}")
    print(f"  {'-'*20} {'-'*15} {'-'*20}")

    results = [
        ("librosa", librosa_avg),
        ("pydub + numpy", pydub_avg),
        ("Rust (PyO3)", rust_avg),
    ]
    # Sort by time (fastest first)
    results.sort(key=lambda x: x[1])

    for name, avg_time in results:
        speedup = librosa_avg / avg_time if avg_time > 0 else float("inf")
        print(f"  {name:<20} {avg_time:<15.4f} {speedup:.2f}x")

    # --- Verify output shapes match ---
    print(f"\n{'=' * 60}")
    print("  Output Verification")
    print("=" * 60)
    for idx in range(len(audio_files)):
        l_len = len(librosa_result[idx])
        p_len = len(pydub_result[idx])
        r_len = len(rust_result[idx])
        match = "✓" if l_len == p_len == r_len else "✗ MISMATCH"
        print(f"  File {idx}: librosa={l_len}, pydub={p_len}, rust={r_len}  {match}")

    print()


if __name__ == "__main__":
    main()
