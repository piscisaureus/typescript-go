// Corresponds to internal/checker in the Go implementation
// This is the root module for type checking functionality

pub mod relater;
mod types;

pub use types::*;
