use quizx::hash_graph::{Graph, VType, VType::{X, Z, WInput,WOutput,ZBox, B, H}, V};
use quizx::graph::{VData, GraphLike};
use quizx::phase::Phase;

fn create_error_diagram (elements: &[(f64, &'static str)]) -> Graph{
    // Takes a list of tuples, (float probability, paulistring) and 
    // applies that hamiltonian to a diagram of qubits empty lines
    let qubits = elements[0].1.chars().count() as u8;
    let lelements = elements.len() as f64;
    // create new empty graph
    let mut g = Graph::new();

    // add left and right boundary spiders
    for i in 0..qubits {
        let vdata_l = VData {
            ty: B,
            qubit: i as f64 + 3.0,
            row: 0.0,
            ..Default::default()
        };
        let vdata_r = VData {
            ty: B,
            qubit: i as f64 + 3.0,
            row: lelements*4.0,
            ..Default::default()
        };
        let left = g.add_vertex_with_data(vdata_l);
        let right = g.add_vertex_with_data(vdata_r);
        g.add_edge(left, right)

    }
    // At this point we have a skeleton of n lines, where n is the amount of qubits
    // Now next we are going to want a W spider and a paulistring gadget for each in list
    // This should give us a W spider exactly centered and above the rest of our diagram

    // OK so it looks like we need a special IO edge between WInput and Woutput spiders,
    // with an actual W spider consisting of one input connected to a bunch of outputs,
    // so we will have to rewrite this next part a bit
    let wdata = VData {
        ty: WInput,
        qubit: 0.0,
        row: lelements*2.0,
        ..Default::default()
    };
    let w = g.add_vertex_with_data(wdata);
    // now add the gadgets
    for i in 0..lelements as u8 {
        let zboxdata = VData {
            ty: ZBox,
            qubit: 1.0,
            row: i as f64*4.0,
            phase: Phase::from_f64(elements[i as usize].0),
            ..Default::default()
        };
        let zbox = g.add_vertex_with_data(zboxdata);
        
        let hdata = VData {
            ty: H,
            qubit: 0.5,
            row: i as f64*4.0,
            ..Default::default()
        };
        let h = g.add_vertex_with_data(hdata);
        let xdata = VData {
            ty: X,
            qubit: 0.2,
            row: i as f64*4.0,
            ..Default::default()
        };
        let x = g.add_vertex_with_data(xdata);
        g.add_edge(w, zbox);
        g.add_edge(zbox, h);
        g.add_edge(h, x);
        //TODO: Add the green spiders on lanes (remove/insert edges)
        // And add pauli conjugations

    }
    g
}
fn fidelity_loop (g:Graph) -> Graph {
    // Add the loops at left and right boundary of diagram to compute fidelity to identity
    for vertex in g.vertices() {
        //...
        // For nice loops will probably have to add a phaseless spider at the bottom to 
        // connect to
    }
    g
}

#[cfg(test)]
mod tests {
    // Import everything from the outer scope
    use super::*;

    #[test]
    fn test_create_diagram() {
        let elements = [(0.5, "X")];
    }
}
