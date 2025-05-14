#!/bin/bash

set -e

mkdir -p bin


echo "🪟 Building for Windows (x86_64)..."
docker run --rm -it \
  -v "$(pwd)":/usr/src/myapp \
  -w /usr/src/myapp \
  rust:latest \
  bash -c "
    rustup target add x86_64-pc-windows-gnu && \
    apt update && apt install -y mingw-w64 && \
    cargo build --release --target x86_64-pc-windows-gnu
  "
cp target/x86_64-pc-windows-gnu/release/kaspa-vanity.exe bin/kaspa-vanity-windows.exe


echo "🍏 Building for macOS (x86_64)..."
# Only works if you're on macOS and have the Rust toolchain locally
cargo build --release
cp target/release/kaspa-vanity bin/kaspa-vanity-macos


echo "✅ All builds completed. Binaries are in ./bin"