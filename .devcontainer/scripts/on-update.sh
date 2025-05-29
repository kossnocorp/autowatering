#!/usr/bin/env bash

set -e

eval "$(mise activate bash --shims)"

# Stack

mise install

# Dependencies

# Rust
# Cargo build to ensure dependencies are downloaded and built:
# See: https://github.com/rust-lang/cargo/issues/2644
cargo build || echo "Build failed, but that's ok"
