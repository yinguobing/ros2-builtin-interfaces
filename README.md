# ROS2 Builtin Interfaces For Rust

Message definitions for types in the OMG IDL Platform Specific Model, for Rust.

# Usage
Add the following to your `Cargo.toml`:
```toml
[dependencies]
ros2_builtin_interfaces = "0.1.0"
```

or using `cargo`:
```bash
cargo add ros2_builtin_interfaces
```

In your code:
```rust
use ros2_builtin_interfaces::msg::{Time, Duration};

let time = Time {
    sec: 0,
    nanosec: 0,
}

let duration = Duration {
    sec: 0,
    nanosec: 0,
}
```