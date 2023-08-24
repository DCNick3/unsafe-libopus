//! Utilities for testing
//!
//! This module contains opus test utilities converted to functions
//! Binaries are also available

#![forbid(unsafe_code)]

mod compare;

pub use compare::{opus_compare, CompareParams, CompareResult};
