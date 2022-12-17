# Usage

```rust
use fiftyonedegrees::properties::PropertyName;
use fiftyonedegrees::api::DeviceDetection;

fn main() {
    let properties = vec![PropertyName::IsMobile];

    let engine = DeviceDetection::new("path/to/file.hash", properties);

    let result = engine.lookup("my user agent");
    
    let boolean = result.getValueAsBoolean(&PropertyName::IsMobile).unwrap().unwrap();
    let string = result.getValueAsString(&PropertyName::BrowserName).unwrap().unwrap();
}
```

# Development

**Install Rust**

https://www.rust-lang.org/tools/install

**Install bindgen**

`cargo install bindgen-cli`

**Download c code**

`git submodule update --init --recursive`

# Updating to latest device-detection-cxx

**Point git submodule to target tag**

`cd device-detection-cxx`

`git checkout 4.2.3`

`cd src/common-cxx`

`git submodule update`

**Regenerate c bindings for rust**

`bindgen src/shim/wrapper.h -o src/shim/mod.rs`

**Run tests**

`cargo test`

# Performance Benchmarking

`cargo run --example benchmark --release`