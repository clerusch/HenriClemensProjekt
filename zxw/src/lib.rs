//! My ZX calculus library
//!
//! # Modules
//! - `graph`: graph data structures
//! - `zx_operations`: ZX-specific transformations
//! - `utils`: helper functions
//! - `prelude`: convenient re-exports

// Declare modules
pub mod create_error_diagram;
//more here

// Optional prelude
pub mod prelude {
    pub use crate::create_error_diagram::*;
    //more here
}

// Optional: top-level exports for convenience
// pub use quizx::Graph;
// pub use zx_operations::{apply_rule, simplify};
