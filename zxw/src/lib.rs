//! My ZX calculus library
//!
//! # Modules
//! - `graph`: graph data structures
//! - `zx_operations`: ZX-specific transformations
//! - `utils`: helper functions
//! - `prelude`: convenient re-exports
use pyo3::prelude::*;
use numpy::{PyReadonlyArray1, PyArray1, IntoPyArray};
use pyo3::BoundObject;
use pyo3::prelude::*;



// Declare modules
pub mod add_arrays;
pub mod bfs_graphs;
pub mod create_error_diagram;
pub mod zxw_rules;
pub mod zxw_simp_rules;
//more here
#[cfg(test)]
pub mod tests;

// Optional prelude
pub mod prelude {
    // pub use crate::create_error_diagram::*;
    // use quizx::hash_graph::{Graph, VType};
    // use quizx::graph::GraphLike;
    // use quizx::phase::Phase;
    //more here
}

// Optional: top-level exports for convenience
// pub use quizx::Graph;
// pub use zx_operations::{apply_rule, simplify};
#[pymodule]
fn hamiltonian_compiler(py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    // Submodule for add_arrays
    let add_mod = PyModule::new(py, "add_arrays")?;
    add_arrays::add_arrays_module(py, add_mod.clone())?;
    m.add_submodule(&add_mod)?;

    // Submodule for bfs_graphs
    let bfs_mod = PyModule::new(py, "bfs_graph")?;
    bfs_graphs::bfs_graph(py, bfs_mod.clone())?;
    m.add_submodule(&bfs_mod)?;
    Ok(())
}
