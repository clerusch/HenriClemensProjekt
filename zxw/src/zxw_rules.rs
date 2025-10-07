use quizx::hash_graph::{Graph, VType};
use quizx::graph::GraphLike;
use quizx::phase::Phase;

fn s1 (g:Graph) -> Graph {
    g
}
// etc. etc. 

#[cfg(test)]
mod tests {
    // Import everything from the outer scope
    use super::*;

    #[test]
    fn test_s1_rule() {
        let elements = [(0.5, "X")];
    }
}