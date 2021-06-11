# Development

**Install Result**

https://www.rust-lang.org/tools/install

**Install bindgen**

`cargo install bindgen`

**Download c code**

`git submodule update --init --recursive`

# Updating to latest device-detection-cxx

**Point git submodule to target tag**

`cd device-detection-cxx`

`git checkout 4.2.3`

**Regenerate c bindings for rust**

`bindgen src/shim/wrapper.h -o src/shim/mod.rs`

**Run tests**

`cargo test`

# Performance Benchmarking

`cargo run --example benchmark --release`