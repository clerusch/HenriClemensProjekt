use quizx::hash_graph::{Graph, VType};
use quizx::graph::GraphLike;
use quizx::phase::Phase;

pub fn s1_simp(g: &mut impl GraphLike) -> bool {
    // das umschreiben
    //vertex_simp!(g, check_remove_id, remove_id_unchecked, false)
    true
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