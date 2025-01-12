# Rust Results

## Stats for rust_xlswriter
- cargo run --release  34.09s user 1.10s system 98% cpu 35.599 total
- cargo run --release  35.59s user 1.14s system 98% cpu 37.164 total
- cargo run --release  34.02s user 1.19s system 98% cpu 35.644 total
- cargo run --release  34.05s user 1.16s system 98% cpu 35.648 total
- cargo run --release  33.82s user 1.24s system 98% cpu 35.458 total
- cargo run --release  36.18s user 1.10s system 98% cpu 37.695 total
- cargo run --release  34.01s user 1.08s system 98% cpu 35.519 total
- cargo run --release  34.05s user 1.23s system 98% cpu 35.670 total
- cargo run --release  34.18s user 1.13s system 98% cpu 35.759 total

Average:
34.44333

## Stats for rust_xlswriter with zlib
- cargo run --release  28.33s user 1.61s system 97% cpu 30.852 total
- cargo run --release  28.26s user 1.63s system 97% cpu 30.787 total
- cargo run --release  28.23s user 1.71s system 97% cpu 30.797 total
- cargo run --release  28.45s user 1.58s system 96% cpu 31.026 total
- cargo run --release  29.57s user 1.55s system 97% cpu 31.904 total
- cargo run --release  28.47s user 1.59s system 96% cpu 31.023 total
- cargo run --release  28.46s user 1.53s system 96% cpu 30.936 total
- cargo run --release  28.20s user 1.55s system 96% cpu 30.678 total
- cargo run --release  28.41s user 1.64s system 96% cpu 31.008 total

Average:
28.4867

## Stats for rust_xlswriter with zlib, optimized with reduced str allocation using `Vec<String>`
- cargo run --release  26.16s user 1.49s system 98% cpu 28.183 total
- cargo run --release  26.65s user 1.33s system 98% cpu 28.500 total
- cargo run --release  26.50s user 1.37s system 98% cpu 28.354 total
- cargo run --release  26.09s user 1.44s system 98% cpu 28.069 total
- cargo run --release  26.02s user 1.43s system 97% cpu 28.046 total
- cargo run --release  26.82s user 1.37s system 97% cpu 28.802 total
- cargo run --release  26.23s user 1.39s system 97% cpu 28.256 total
- cargo run --release  26.26s user 1.50s system 98% cpu 28.279 total
- cargo run --release  26.29s user 1.40s system 98% cpu 28.129 total

Average:
26.335