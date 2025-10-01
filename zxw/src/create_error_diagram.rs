use quizx::hash_graph::{Graph, VType};
use quizx::graph::GraphLike;
use quizx::phase::Phase;
fn create_error_diagram (elements: &[(f64, Vec<VType>)]) -> Graph{
    // create new empty graph
    let mut g = Graph::new();

    // add basic loop with W spider
    let v1 = g.add_vertex(VType::Z);
    let v2 = g.add_vertex(VType::Z);
    let v3 = g.add_vertex(VType::Z);
    let v4 = g.add_vertex(VType::WInput);
    let v5 = g.add_vertex(VType::WOutput);
    let v6 = g.add_vertex_with_phase(VType::X, Phase::from_f64(std::f64::consts::PI));
    g.add_edge(v1, v3);
    g.add_edge(v3, v2);
    g.add_edge(v1, v2);

    // connect winput to red pi and winput to woutput?
    g.add_edge(v4, v5);
    g.add_edge(v4, v6);
    


    for elem in elements {
        let f = elem.0;
        let v7 = g.add_vertex_with_phase(VType::ZBox, Phase::from_f64(f));
        g.add_edge(v5, v7);
        g.add_edge(v7, v3);
        for ty in &elem.1 {
            // add some logic to put stuff on qubit lanes
            // possibly tad complicated
        }
    }
    g
}

#[cfg(test)]
mod tests {
    // Import everything from the outer scope
    use super::*;

    #[test]
    fn test_create_diagram() {
        elements = [(0.5, "X")]
        assert_eq!(add(2, 3), 5);
    }
}