use quizx::graph::VType::{WInput, WOutput, X, Z};
use quizx::graph::{EType, GraphLike};
use quizx::hash_graph::Graph;
use quizx::simplify::full_simp;

#[test]
fn test_w_spider_structure() {
    let mut g = Graph::new();

    let x_spider = g.add_vertex(X);
    let z_left = g.add_vertex(Z);
    let z_right = g.add_vertex(Z);
    let w_in = g.add_vertex(WInput);
    let w_out = g.add_vertex(WOutput);

    g.add_edge(x_spider, w_in);
    g.add_edge(w_out, z_left);
    g.add_edge(w_out, z_right);
    g.add_edge_with_type(w_in, w_out, EType::Wio);

    // Ensure simplification routines do not crash on diagrams with W spiders.
    full_simp(&mut g);

    assert_eq!(g.vertex_type(w_in), WInput);
    assert_eq!(g.vertex_type(w_out), WOutput);
    assert_eq!(g.degree(w_in), 2, "W input should connect to the X spider and W output");
    assert_eq!(g.degree(w_out), 3, "W output should fan out to two Z spiders and back to the input");
    assert!(g.neighbor_vec(w_in).contains(&x_spider));
    assert!(g.neighbor_vec(w_in).contains(&w_out));
    assert_eq!(g.edge_type(w_in, w_out), EType::Wio, "W spiders must be joined by a Wio edge");
}
