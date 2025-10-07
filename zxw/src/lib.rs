//! My ZX calculus library
//!
//! # Modules
//! - `graph`: graph data structures
//! - `zx_operations`: ZX-specific transformations
//! - `utils`: helper functions
//! - `prelude`: convenient re-exports

// Declare modules
pub mod create_error_diagram;
pub mod zxw_rules;
pub mod zxw_simp_rules;
//more here

// Optional prelude
pub mod prelude {
    pub use crate::create_error_diagram::*;
    use quizx::hash_graph::{Graph, VType};
    use quizx::graph::GraphLike;
    use quizx::phase::Phase;
    //more here
}

// Optional: top-level exports for convenience
// pub use quizx::Graph;
// pub use zx_operations::{apply_rule, simplify};
