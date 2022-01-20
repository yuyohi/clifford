use itertools::Itertools;
use ndarray::prelude::*;
use rand::{self, Rng};
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::decoder::mwpm;
use crate::noise::noise_model::NoiseType;
use crate::qubit_graph::ungraph::UnGraph;
use crate::qubit_network::QubitNetwork;
use crate::simulator::{frame::PauliFrame, Type};

pub struct RotatedSurfaceCode {
    distance: usize,
    round: usize,
    network: QubitNetwork,
    measurement_qubit_z: Vec<(i32, i32)>,
    measurement_qubit_x: Vec<(i32, i32)>,
    data_qubit: Vec<(i32, i32)>,
    classical_register: Vec<Vec<Rc<Cell<u8>>>>,
    measurement_graph_z: UnGraph,
    measurement_graph_x: UnGraph,
    pauli_frame: PauliFrame,
    error_rate: f32,
}

impl RotatedSurfaceCode {
    pub fn new(distance: usize, round: usize, p: f32, seed: u64) -> Self {
        if distance % 2 == 0 {
            panic!("distance must be odd number.");
        }

        // 測定bitの生成
        let (mut measurement_qubit_z, mut measurement_qubit_x) =
            Self::gen_measurement_qubit(distance);
        // 測定ビットを効率的に測定できるように並びかえ
        measurement_qubit_x.sort_by(|l, r| match l.1.cmp(&r.1) {
            std::cmp::Ordering::Equal => l.0.cmp(&r.0),
            other => other,
        });
        measurement_qubit_z.sort_by(|l, r| match l.0.cmp(&r.0) {
            std::cmp::Ordering::Equal => l.1.cmp(&r.1),
            other => other,
        });

        // data bitの生成
        let mut data_qubit = vec![];
        for x in (0..distance as i32 * 2).step_by(2) {
            for y in (0..distance as i32 * 2).step_by(2) {
                data_qubit.push((x, y));
            }
        }

        let network = QubitNetwork::new_rotated_planer_lattice_from_vec(
            data_qubit.clone(),
            measurement_qubit_z.clone(),
            measurement_qubit_x.clone(),
            p,
            Type::CHPSimulator,
            seed,
        );

        // data qubit の測定結果を格納する行列
        let classical_register = (0..distance)
            .map(|_| (0..distance).map(|_| Rc::new(Cell::new(0))).collect())
            .collect();

        // make syndrome graph
        let measurement_graph_z =
            Self::gen_measurement_graph(&measurement_qubit_z, round, distance, 'Z', p, seed);
        let measurement_graph_x =
            Self::gen_measurement_graph(&measurement_qubit_x, round, distance, 'X', p, seed);

        // make pauli frame
        let pauli_frame = PauliFrame::new_rotated_surface_code(distance);

        Self {
            distance,
            round,
            network,
            measurement_qubit_z,
            measurement_qubit_x,
            data_qubit,
            classical_register,
            measurement_graph_z,
            measurement_graph_x,
            pauli_frame,
            error_rate: p,
        }
    }

    /// generate measurement qubits
    fn gen_measurement_qubit(distance: usize) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
        let mut measurement_qubit_temp = Vec::new();

        for y in (1..distance as i32 * 2 - 1).step_by(4) {
            for x in (1..distance as i32 * 2).step_by(2) {
                measurement_qubit_temp.push((x, y));
            }
        }
        for y in (3..distance as i32 * 2 - 1).step_by(4) {
            for x in (-1..distance as i32 * 2 - 1).step_by(2) {
                measurement_qubit_temp.push((x, y));
            }
        }

        // 上下のqubitを追加
        for x in (1..distance as i32 * 2 - 2).step_by(4) {
            measurement_qubit_temp.push((x, -1));
        }
        for x in (3..distance as i32 * 2 - 2).step_by(4) {
            measurement_qubit_temp.push((x, distance as i32 * 2 - 1));
        }

        // Z, Xに振り分ける
        let mut measurement_qubit_z = Vec::new();
        let mut measurement_qubit_x = Vec::new();

        let mut qubit_is_z = true;
        for y in (-1..distance as i32 * 2).step_by(2) {
            for x in (-1..distance as i32 * 2).step_by(2) {
                if measurement_qubit_temp.contains(&(x, y)) {
                    match qubit_is_z {
                        true => measurement_qubit_z.push((x, y)),
                        false => measurement_qubit_x.push((x, y)),
                    }
                }
                // 交互に入れ替える
                qubit_is_z = !qubit_is_z;
            }
            qubit_is_z = !qubit_is_z;
        }

        (measurement_qubit_z, measurement_qubit_x)
    }

    fn gen_measurement_graph(
        measurement_qubit: &Vec<(i32, i32)>,
        round: usize,
        distance: usize,
        mode: char,
        p: f32,
        seed: u64,
    ) -> UnGraph {
        let mut network = UnGraph::new(round, seed);
        let direction = [(2, 2), (-2, 2)];
        let boundary_direction = [(2, 2), (-2, 2), (-2, -2), (2, -2)];

        for t in 0..round as i32 {
            let mut edges = Vec::new();

            // 通常のedgeを追加
            for u in measurement_qubit.iter() {
                for d in direction.iter() {
                    match (u.0 + d.0, u.1 + d.1) {
                        v if measurement_qubit.contains(&v) => {
                            let start = (u.0, u.1, t);
                            let end = (v.0, v.1, t);
                            edges.push((start, end));
                        }
                        _ => (),
                    }
                }
            }

            // 次のroundの同座標に対するedgeを追加
            let mut time_edge = Vec::new();
            for &(x, y) in measurement_qubit.iter() {
                time_edge.push(((x, y, t), (x, y, t + 1)));
            }
            // 最後のroundでは、次の時間はboundaryなので、weight0のedgeで繋ぐ
            if t == (round as i32 - 1) {
                let time_boundary_edge: Vec<_> = measurement_qubit
                    .iter()
                    .tuple_windows()
                    .map(|(&(u_x, u_y), &(v_x, v_y))| ((u_x, u_y, t + 1), (v_x, v_y, t + 1)))
                    .collect();
                network.add_edges_from(&time_boundary_edge);
                network.set_edges_weight(&time_boundary_edge, 0.0);
            }
            network.add_edges_from(&time_edge);
            network.set_edges_weight(&time_edge, p);

            // boundary nodeを追加
            let boundary_num = (distance / 2 + 1) as i32;
            let mut boundary_node = Vec::new();

            match mode {
                'Z' => {
                    let x_start_list = [-1, 1];
                    let height = (distance - 1) * 2;
                    let y_list = [-1, height as i32 + 1];
                    for (y, x_start) in y_list.iter().zip(x_start_list.iter()) {
                        for i in 0..boundary_num {
                            let x = x_start + 4 * i;
                            boundary_node.push((x, *y, t))
                        }
                    }
                }
                'X' => {
                    let y_start_list = [1, -1];
                    let width = (distance - 1) * 2;
                    let x_references = [-1, width as i32 + 1];
                    for (x, y_start) in x_references.iter().zip(y_start_list.iter()) {
                        for i in 0..boundary_num {
                            let y = y_start + 4 * i;
                            boundary_node.push((*x, y, t))
                        }
                    }
                }
                _ => panic!("Invalid mode"),
            }

            // boundary nodeと普通のnodeを結ぶ
            for u in boundary_node.iter() {
                for d in boundary_direction.iter() {
                    match (u.0 + d.0, u.1 + d.1) {
                        v if measurement_qubit.contains(&v) => {
                            let start = *u;
                            let end = (v.1, v.1, t);
                            edges.push((start, end));
                        }
                        _ => (),
                    }
                }
            }
            network.add_edges_from(&edges);
            network.set_edges_weight(&edges, p);

            // boundary node 同士をweight0のedgeで繋ぐ
            let boundary_edge: Vec<_> = boundary_node
                .iter()
                .tuple_windows()
                .map(|(&u, &v)| (u, v))
                .collect();
            network.add_edges_from(&boundary_edge);
            network.set_edges_weight(&boundary_edge, 0.0);

            // boundaryかどうかを設定
            for &(x, y) in measurement_qubit.iter() {
                network.set_is_boundary((x, y, t), false);
                if t == (round as i32 - 1) {
                    // 最後のroundでは、時間方向のboundaryを設定
                    network.set_is_boundary((x, y, t + 1), true);
                }
            }
            network.set_all_is_boundary(&boundary_node, true);

            // classical registerを設定 tが変わっても同じ順番で並べることが重要
            for &(x, y) in measurement_qubit.iter() {
                network.set_classical_register((x, y, t), Rc::new(Cell::new(0)));
            }
        }
        network
    }

    /// syndrome measurement
    pub fn syndrome_measurement(&mut self) {
        let Self {
            round,
            network,
            measurement_qubit_z,
            measurement_qubit_x,
            data_qubit,
            measurement_graph_z,
            measurement_graph_x,
            ..
        } = self;

        let noise_type = NoiseType::Depolarizing(network.error_rate());

        let x_order = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
        let z_order = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

        for t in 0..*round as i32 {
            // XスタビライザーにHゲートを作用させる
            for &x_stab in measurement_qubit_x.iter() {
                network.h(x_stab);
                network.insert_noise(x_stab, noise_type);
            }

            // CNOT
            for (x, z) in x_order.iter().zip(z_order.iter()) {
                for (x_stab, z_stab) in measurement_qubit_x.iter().zip(measurement_qubit_z.iter()) {
                    let coord_data_z = (z_stab.0 + z.0, z_stab.1 + z.1);
                    let coord_data_x = (x_stab.0 + x.0, x_stab.1 + x.1);

                    // data bitが存在するときのみCNOT
                    if data_qubit.contains(&coord_data_z) {
                        network.cx(coord_data_z, *z_stab);
                        network.insert_noise(*z_stab, noise_type);
                        network.insert_noise(coord_data_z, noise_type);
                    }
                    if data_qubit.contains(&coord_data_x) {
                        network.cx(*x_stab, coord_data_x);
                        network.insert_noise(*x_stab, noise_type);
                        network.insert_noise(coord_data_x, noise_type);
                    }
                }
            }

            // XスタビライザーにHゲートを作用させる
            for &x_stab in measurement_qubit_x.iter() {
                network.h(x_stab);
                network.insert_noise(x_stab, noise_type);
            }

            // measurement qubitの測定
            // Z
            for &(x, y) in measurement_qubit_z.iter() {
                network.measurement(
                    (x, y),
                    Rc::clone(measurement_graph_z.classical_register(&(x, y, t)).unwrap()),
                );
            }
            // X
            for &(x, y) in measurement_qubit_x.iter() {
                network.measurement(
                    (x, y),
                    Rc::clone(measurement_graph_x.classical_register(&(x, y, t)).unwrap()),
                );
            }
        }
    }

    /// encoding logical one
    pub fn initialize(&mut self) {
        let Self {
            network,
            measurement_qubit_x,
            data_qubit,
            ..
        } = self;
        let x_order = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

        // X syndrome の測定
        // XスタビライザーにHゲートを作用させる
        for &x_stab in measurement_qubit_x.iter() {
            network.h(x_stab);
        }

        // CNOT
        for x in x_order.iter() {
            for x_stab in measurement_qubit_x.iter() {
                let coord_data_x = (x_stab.0 + x.0, x_stab.1 + x.1);

                // data bitが存在するときのみCNOT
                if data_qubit.contains(&coord_data_x) {
                    network.cx(*x_stab, coord_data_x);
                }
            }
        }

        // XスタビライザーにHゲートを作用させる
        for &x_stab in measurement_qubit_x.iter() {
            network.h(x_stab);
        }

        // measurement qubit の測定 (強制的に固有値+1に射影する)
        for coord in measurement_qubit_x.iter() {
            network.measurement_to_zero(*coord);
        }
    }

    /// logical z measurement
    pub fn logical_measurement(&mut self) {
        let Self {
            network,
            data_qubit,
            classical_register,
            ..
        } = self;

        for &(x, y) in data_qubit.iter() {
            debug_assert!(x >= 0, "data coord must not be negative number");
            debug_assert!(y >= 0, "data coord must not be negative number");

            network.measurement(
                (x, y),
                Rc::clone(&classical_register[(x / 2) as usize][(y / 2) as usize]),
            );
        }
    }

    /// correct z error
    fn correct_z_error(&mut self) {
        let Self {
            classical_register,
            pauli_frame,
            ..
        } = self;

        pauli_frame
            .x_frame_mut()
            .iter()
            .zip(classical_register.iter().flatten())
            .for_each(|(&frame, register)| register.set(register.get() ^ frame));
    }

    /// return logical value
    pub fn logical_value(&mut self) -> u8 {
        self.correct_z_error();

        let result = self.classical_register();

        // Rc<Cell<u8>>をほどいて縦方向に足す
        let result_vec = result
            .iter()
            .map(|row| row.iter().map(|value| value.get()).collect::<Vec<u8>>())
            .reduce(|row_a, row_b| {
                row_a
                    .iter()
                    .zip(row_b.iter())
                    .map(|(&a, &b)| a + b)
                    .collect()
            })
            .unwrap();

        let logical_value: usize = result_vec.into_iter().map(|v| (v % 2) as usize).sum();

        match logical_value {
            n if n == self.distance => 1,
            0 => 0,
            _ => panic!("correction error"),
        }
    }

    /// decode by mwpm
    pub fn decode_mwpm(&mut self, m: usize) {
        Self::insert_measurement_error(&mut self.measurement_graph_z, self.error_rate);
        Self::insert_measurement_error(&mut self.measurement_graph_x, self.error_rate);

        Self::xor_syndrome(&mut self.measurement_graph_z);
        Self::xor_syndrome(&mut self.measurement_graph_x);

        Self::flip_defect(&mut self.measurement_graph_z);
        Self::flip_defect(&mut self.measurement_graph_x);

        let correction_qubit_z = mwpm::decode(&self.measurement_graph_x, m);
        let correction_qubit_x = mwpm::decode(&self.measurement_graph_z, m);

        // pauli frameに設定
        let mut z_frame = self.pauli_frame.z_frame_mut();
        for (x, y) in correction_qubit_z.into_iter() {
            debug_assert!(x % 2 == 0, "data coord must not be odd number");
            debug_assert!(y % 2 == 0, "data coord must not be odd number");
            z_frame[(x as usize, y as usize)] ^= 1;
        }
        let mut x_frame = self.pauli_frame.x_frame_mut();
        for (x, y) in correction_qubit_x.into_iter() {
            debug_assert!(x % 2 == 0, "data coord must not be odd number");
            debug_assert!(y % 2 == 0, "data coord must not be odd number");
            x_frame[(x as usize, y as usize)] ^= 1;
        }
    }

    /// defectが奇数のとき、boundaryを一つ反転させる
    fn flip_defect(measurement_graph: &mut UnGraph) {
        let defect_num = measurement_graph
            .iter_classical_register()
            .filter(|(_, defect)| defect.get() == 1)
            .count();
        if defect_num % 2 == 1 {
            for (coord, defect) in measurement_graph
                .iter_classical_register()
                .filter(|&(coord, _)| measurement_graph.is_boundary(coord).unwrap())
            {
                if defect.get() == 0 {
                    measurement_graph.flip_classical_register(coord, 1)
                }
            }
        }
    }

    /// 測定結果にxorを施してdecode用の値にする
    fn xor_syndrome(measurement_graph: &mut UnGraph) {
        // 毎回measurement qubitの初期化をしていないため、値が入れ替わったかどうかで判断する
        measurement_graph.xor_to_last_time();

        // measurement error を確認するために時間方向でもう一度xor
        measurement_graph.xor_to_last_time()
    }

    /// 測定結果にerrorを挿入
    fn insert_measurement_error(measurement_graph: &mut UnGraph, error_rate: f32) {
        measurement_graph.insert_measurement_error(error_rate);
    }

    /// run circuit
    pub fn run(&mut self) {
        self.network.run();
    }

    pub fn classical_register(&self) -> &Vec<Vec<Rc<Cell<u8>>>> {
        &self.classical_register
    }

    pub fn index_to_sim(&self) -> &HashMap<(i32, i32), usize> {
        &self.network.index_to_sim()
    }
}

mod test {

    #[test]
    fn gen_qec_code() {
        for distance in (3..27).step_by(2) {
            let code = super::RotatedSurfaceCode::new(distance, distance + 2, 0.01, 0);
            assert_eq!(
                distance * distance,
                code.measurement_qubit_z.len() + code.measurement_qubit_z.len() + 1
            )
        }
    }

    #[test]
    fn test_gen_measurement_graph() {
        let distance = 3;
        let code = super::RotatedSurfaceCode::new(distance, distance + 2, 0.01, 0);
        let seed = 0;

        super::RotatedSurfaceCode::gen_measurement_graph(
            &code.measurement_qubit_z,
            distance + 2,
            distance,
            'Z',
            0.01,
            seed,
        );
    }
}
