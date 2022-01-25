use retworkx_core::max_weight_matching::max_weight_matching;
use retworkx_core::petgraph;
use retworkx_core::Result;

use hashbrown::HashSet;

fn main() {
    // Create a path graph
    let g = petgraph::graph::UnGraph::<i32, i128>::from_edges(&[(1, 2, 5), (2, 3, 11), (3, 4, 5)]);

    // Run max weight matching with max cardinality set to false
    let res: Result<HashSet<(usize, usize)>> =
        max_weight_matching(&g, false, |e| Ok(*e.weight()), true);
    // Run max weight matching with max cardinality set to true
    let maxc_res: Result<HashSet<(usize, usize)>> =
        max_weight_matching(&g, true, |e| Ok(*e.weight()), true);

    let matching = res.unwrap();
    let maxc_matching = maxc_res.unwrap();
    // Check output
    assert_eq!(matching.len(), 1);
    assert!(matching.contains(&(2, 3)) || matching.contains(&(3, 2)));
    assert_eq!(maxc_matching.len(), 2);
    assert!(maxc_matching.contains(&(1, 2)) || maxc_matching.contains(&(2, 1)));
    assert!(maxc_matching.contains(&(3, 4)) || maxc_matching.contains(&(4, 3)));
}
