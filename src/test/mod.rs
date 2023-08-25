//! Utilities for testing
//!
//! This module contains opus test utilities converted to functions
//! Binaries are also available

mod compare;
pub mod demo;

pub use compare::{opus_compare, CompareParams, CompareResult};
