use clifford::qubit_graph::ungraph::UnGraph;

use petgraph::algo::matching;
use petgraph::graphmap::GraphMap;
use petgraph::graphmap::UnGraphMap;
use std::cell::Cell;
use std::rc::Rc;

#[test]
fn iterate_ungraph() {
    let edges = [((1, 1, 0), (1, 1, 1)), ((1, 1, 0), (2, 2, 0))];
    let round = 2;
    let seed = 0;
    let mut g = UnGraph::from_edges(&edges, round, seed);
    g.set_classical_register((1, 1, 0), Rc::new(Cell::new(0)));
    g.set_classical_register((1, 1, 1), Rc::new(Cell::new(0)));
    g.set_classical_register((2, 2, 0), Rc::new(Cell::new(0)));

    for i in g.iter_classical_register() {
        println!("{:?}, {:?}", i.0, i.1);
    }
}

#[test]
fn test_xor() {
    let edges = [
        ((1, 1, 0), (1, 1, 1)),
        ((1, 1, 1), (1, 1, 2)),
        ((1, 1, 2), (1, 1, 3)),
        ((1, 1, 3), (1, 1, 4)),
        ((1, 1, 4), (1, 1, 5)),
    ];

    let round = 6;
    let seed = 0;
    let mut g = UnGraph::from_edges(&edges, round, seed);

    for i in 0..6 {
        g.set_classical_register((1, 1, i), Rc::new(Cell::new(1)));
    }

    g.show_all_defect();
    g.xor_to_last_time();
    g.show_all_defect();
}

#[test]
fn test_matching() {
    let local_graph = UnGraphMap::<i32, i32>::from_edges(&[
        (0, 1, 0),
        (0, 2, 100),
        (3, 0, 2),
        (3, 1, 1),
        (1, 2, 2),
        (3, 2, 3),
    ]);

    let matching = matching::maximum_matching(&local_graph);
    for edge in matching.edges() {
        println!("{:?}", edge);
    }
}
