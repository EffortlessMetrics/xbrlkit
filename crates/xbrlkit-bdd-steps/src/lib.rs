//! Minimal step execution for the active BDD slices.
//!
//! This crate provides BDD step handlers for xbrlkit scenarios.
//! The implementation is organized into submodules:
//!
//! - `types`: Core types including `World`, `Step`, and context structs
//! - `given`: "Given" step handlers
//! - `when`: "When" step handlers
//! - `then`: "Then" step handlers and parameterized assertions
//! - `utils`: Helper functions

mod given;
mod then;
mod types;
mod utils;
mod when;

pub use types::{Step, World, run_scenario};
