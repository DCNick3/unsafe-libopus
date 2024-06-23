//! Utilities for testing opus
//!
//! This crate contains opus test utilities converted to functions
//! Binaries are also available

mod compare;
pub mod demo;

pub use compare::{opus_compare, CompareParams, CompareResult};
