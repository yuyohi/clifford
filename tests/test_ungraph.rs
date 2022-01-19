use clifford::qubit_graph::ungraph::UnGraph;

use std::cell::Cell;
use std::rc::Rc;

#[test]
fn iterate_ungraph() {
    let edges = [((1, 1, 0), (1, 1, 1)), ((1, 1, 0), (2, 2, 0))];
    let round = 2;
    let mut g = UnGraph::from_edges(&edges, round);
    g.set_classical_register((1, 1, 0), Rc::new(Cell::new(0)));
    g.set_classical_register((1, 1, 1), Rc::new(Cell::new(0)));
    g.set_classical_register((2, 2, 0), Rc::new(Cell::new(0)));

    for i in g.iter_classical_register() {
        println!("{:?}, {:?}", i.0, i.1);
    }
}