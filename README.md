#ROS2 Builtin Interfaces For Rust

This crate provides ROS2 builtin interfaces for Rust.

# Usage
Add the following to your `Cargo.toml`:
```toml
[dependencies]
ros2_builtin_interfaces = "0.1.0"
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