//! # ros2-builtin-interfaces
//!
//! Message definitions for types in the OMG IDL Platform Specific Model
//!
pub mod msg {
    use serde::{Deserialize, Serialize};

    /// Time indicates a specific point in time, relative to a clock's 0 point.
    /// # Examples
    ///
    /// ```
    /// use ros2_builtin_interfaces::msg::Time;
    /// let time = Time { sec: 0, nanosec: 1 };
    /// assert_eq!(time.sec, 0);
    /// assert_eq!(time.nanosec, 1);
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
    pub struct Time {
        /// The seconds component, valid over all int32 values.
        pub sec: i32,

        /// The nanoseconds component, valid in the range [0, 10e9).
        pub nanosec: u32,
    }

    /// Duration defines a period between two time points. It is comprised of a seconds component and a nanoseconds component.
    /// # Examples
    ///
    /// ```
    /// use ros2_builtin_interfaces::msg::Duration;
    /// let duration = Duration { sec: 0, nanosec: 1 };
    /// assert_eq!(duration.sec, 0);
    /// assert_eq!(duration.nanosec, 1);
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
    pub struct Duration {
        /// Seconds component, range is valid over any possible int32 value.
        pub sec: i32,

        /// Nanoseconds component in the range of [0, 10e9).
        pub nanosec: u32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time() {
        let time = msg::Time { sec: 0, nanosec: 1 };
        assert_eq!(time.sec, 0);
        assert_eq!(time.nanosec, 1);
    }

    #[test]
    fn test_duration() {
        let duration = msg::Duration { sec: 2, nanosec: 3 };
        assert_eq!(duration.sec, 2);
        assert_eq!(duration.nanosec, 3);
    }
}
