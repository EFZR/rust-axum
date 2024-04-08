//! The utils module is designed to export independent sub-modules to the application code.
//!
//! Note: Even if the utils sub-module consists of a single file, it contains its own errors
//!       for improved compartmentalization.
//!

pub mod b64;
pub mod envs;
pub mod time;
