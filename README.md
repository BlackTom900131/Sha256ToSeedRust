# getSeed(sha256) - Seed Hash Finder

A high-performance Rust application that generates random seeds and searches for one that produces a specific SHA-256 hash value.

## Project Overview

This program generates 64-character random seeds composed of alphanumeric characters and computes their SHA-256 hashes in parallel. It continues searching until it finds a seed that matches a target hash value, then saves the matching seed to a file.

## Features

- **Parallel Processing**: Uses the `rayon` library to leverage multi-core processors for faster seed generation and hashing
- **SHA-256 Hashing**: Implements secure hashing using the `sha2` crate
- **Efficient Search**: Generates millions of seeds in parallel to find hash matches quickly
- **File Output**: Saves the matching seed to `matching_seed.txt` when found

## Dependencies

- `rand` (0.8) - Random number generation
- `sha2` (0.10) - SHA-256 hashing
- `hex` (0.4) - Hexadecimal encoding/decoding
- `rayon` (1.5) - Data-level parallelism

## Build & Run

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run --release
```

The `--release` flag is recommended for optimal performance during hash searching.

## How It Works

1. **Seed Generation**: Randomly generates 64-character seeds using lowercase letters, uppercase letters, and digits
2. **Hashing**: Computes the SHA-256 hash of each seed
3. **Matching**: Compares the generated hash against the target hash: `db7683d56ed28d85529167ebcd63b8708ad8ede9aa5b913ceb2e7d0801da0535`
4. **Output**: When a match is found, the seed is written to `matching_seed.txt` and the program exits

## Performance

The use of `rayon` for parallel processing enables:
- Multi-threaded seed generation across all CPU cores
- Concurrent hash computations
- Efficient thread pool management

Building with `--release` optimizations is essential for maximum throughput.

## Output

When a matching seed is found, the program creates:
- **matching_seed.txt** - Contains the hexadecimal-encoded matching seed

## Notes

- The seed search is probabilistic; finding a match may take considerable time depending on the target hash
- All computations are deterministic (given the same seed, the same hash is always produced)
- The program is designed to be CPU-bound, making multi-core systems essential for reasonable search times
