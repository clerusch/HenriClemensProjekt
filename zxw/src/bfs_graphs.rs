use pyo3::prelude::*;
use std::collections::VecDeque;

#[pyfunction]
pub fn bfs_count(n_nodes: usize, edges: Vec<(usize, usize)>, start: usize) -> PyResult<Vec<usize>> {
    let mut adj = vec![Vec::new(); n_nodes];
    for (u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut visited = vec![false; n_nodes];
    let mut distance = vec![usize::MAX; n_nodes];
    let mut queue = VecDeque::new();

    visited[start] = true;
    distance[start] = 0;
    queue.push_back(start);

    while let Some(u) = queue.pop_front() {
        for &v in &adj[u] {
            if !visited[v] {
                visited[v] = true;
                distance[v] = distance[u] + 1;
                queue.push_back(v);
            }
        }
    }
    Ok(distance)
}

/// Submodule initializer for bfs_graphs
#[pymodule]
pub fn bfs_graph(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bfs_count, &m)?)?;
    Ok(())
}