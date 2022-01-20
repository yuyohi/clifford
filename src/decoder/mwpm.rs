use crate::qubit_graph::ungraph::UnGraph;

use itertools::Itertools;
use petgraph::algo::matching;
use petgraph::graphmap::GraphMap;
use petgraph::graphmap::UnGraphMap;
use petgraph::Undirected;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Debug)]
struct State {
    distance: f32,
    coord: (i32, i32, i32),
    predecessor: (i32, i32, i32),
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .partial_cmp(&self.distance)
            .unwrap()
            .then_with(|| Ordering::Less)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn local_dijkstra(
    graph: &UnGraph,
    m: usize,
    s: &(i32, i32, i32),
) -> Vec<(Vec<(i32, i32, i32)>, f32)> {
    let mut state_list = graph
        .nodes()
        .map(|n| {
            (
                *n,
                State {
                    distance: f32::MAX,
                    coord: *n,
                    predecessor: *n,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    state_list.get_mut(&s).unwrap().distance = 0.0;
    let mut heap = BinaryHeap::new();
    heap.push(State {
        distance: 0.0,
        coord: *s,
        predecessor: *s,
    });

    let mut m_nearest_node = Vec::new();
    let mut visited = HashSet::new();

    while m_nearest_node.len() < m + 1 {
        if let Some(State {
            distance,
            coord,
            predecessor,
        }) = heap.pop()
        {
            if visited.contains(&coord) {
                continue;
            }
            if graph
                .classical_register(&coord)
                .unwrap_or_else(|| panic!("{:?} isn't exist", coord))
                .get()
                == 1
            {
                m_nearest_node.push(State {
                    distance,
                    coord,
                    predecessor,
                });
            }
            for v in graph.neighbors(&coord).unwrap() {
                let new_distance = graph.edge_weight(&(coord, *v)).unwrap() + distance;
                if new_distance < state_list.get(v).unwrap().distance {
                    let state = state_list.get_mut(v).unwrap();
                    state.distance = new_distance;
                    state.predecessor = coord;
                    heap.push(State {
                        distance: new_distance,
                        coord: *v,
                        predecessor: coord,
                    });
                }
            }
            visited.insert(coord);
        } else {
            break;
        }
    }

    // make m path
    let mut m_nearest_path = Vec::new();
    for n in m_nearest_node.into_iter().skip(1) {
        let mut path = vec![n.coord];
        let mut coord = n.coord;
        loop {
            let next = state_list.get(&coord).unwrap();
            if next.coord == next.predecessor {
                break;
            }

            path.push(next.predecessor);
            coord = next.predecessor;
        }
        m_nearest_path.push((path, n.distance));
    }

    m_nearest_path
}

fn construct_syndrome_graph(
    graph: &UnGraph,
    m: usize,
) -> (
    GraphMap<(i32, i32, i32), f32, Undirected>,
    HashMap<((i32, i32, i32), (i32, i32, i32)), Vec<(i32, i32, i32)>>,
) {
    let mut paths = Vec::new();
    let mut path_detail = HashMap::new();

    for (&coord, defect) in graph.iter_classical_register() {
        if defect.get() == 1 {
            let path = local_dijkstra(graph, m, &coord);

            for (p, d) in path.into_iter() {
                paths.push((coord, p[0], -d));
                path_detail.insert((coord, p[0]), p.clone());
                path_detail.insert((p[0], coord), p);
            }
        }
    }

    // construct local matching graph
    let local_graph = UnGraphMap::<(i32, i32, i32), f32>::from_edges(&paths);

    (local_graph, path_detail)
}

fn minimum_weight_perfect_matching(
    (local_graph, path_detail): (
        GraphMap<(i32, i32, i32), f32, Undirected>,
        HashMap<((i32, i32, i32), (i32, i32, i32)), Vec<(i32, i32, i32)>>,
    ),
) -> Vec<(i32, i32)> {
    let matching = matching::maximum_matching(&local_graph);

    let mut correction_qubit = Vec::new();

    // 空間方向にedgeが存在するものだけを抽出
    for (u, v) in matching.edges() {
        if (u.0 != v.0) || (u.1 != v.1) {
            let correction_path = path_detail
                .get(&(u, v))
                .unwrap_or_else(|| panic!("edge: {:?} is not exist", (u, v)));

            correction_path
                .into_iter()
                .tuple_windows()
                .filter(|(&u, &v)| (u.0 != v.0) || (u.1 != v.1))
                .for_each(|(&u, &v)| correction_qubit.push(UnGraph::edge_to_qubit((u, v))));
        }
    }
    correction_qubit
}

/// decode
pub fn decode(graph: &UnGraph, m: usize) -> Vec<(i32, i32)> {
    minimum_weight_perfect_matching(construct_syndrome_graph(graph, m))
}
