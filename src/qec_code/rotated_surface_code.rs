use ndarray::prelude::*;
use petgraph::graph::{Graph, UnGraph};
use rand::{self, Rng};
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::qubit_network::QubitNetwork;
use crate::simulator;
use crate::simulator::Type;

pub struct RotatedSurfaceCode {
    distance: usize,
    round: usize,
    network: QubitNetwork,
    measurement_qubit_z: Vec<(i32, i32)>,
    measurement_qubit_x: Vec<(i32, i32)>,
    data_qubit: Vec<(i32, i32)>,
    syndrome_result_z: Vec<Vec<Vec<Rc<Cell<u8>>>>>,
    syndrome_result_x: Vec<Vec<Vec<Rc<Cell<u8>>>>>,
    classical_register: Vec<Vec<Rc<Cell<u8>>>>,
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

        // syndrome resultを格納する行列
        let syndrome_result_z = (0..round)
            .map(|_| {
                (0..distance / 2)
                    .map(|_| (0..distance + 1).map(|_| Rc::new(Cell::new(0))).collect())
                    .collect()
            })
            .collect();
        let syndrome_result_x = (0..round)
        .map(|_| {
            (0..distance + 1)
                .map(|_| (0..distance / 2).map(|_| Rc::new(Cell::new(0))).collect())
                .collect()
        })
        .collect();

        // data qubit の測定結果を格納する行列
        let classical_register = (0..distance)
            .map(|_| (0..distance).map(|_| Rc::new(Cell::new(0))).collect())
            .collect();

        Self {
            distance,
            round,
            network,
            measurement_qubit_z,
            measurement_qubit_x,
            data_qubit,
            syndrome_result_z,
            syndrome_result_x,
            classical_register,
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

    /// syndrome measurement
    pub fn syndrome_measurement(&mut self) {
        let Self {
            network,
            measurement_qubit_z,
            measurement_qubit_x,
            data_qubit,
            syndrome_result_z,
            syndrome_result_x,
            ..
        } = self;

        let x_order = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
        let z_order = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

        for (matrix_z, matrix_x) in syndrome_result_z.iter().zip(syndrome_result_x.iter()) {
            // XスタビライザーにHゲートを作用させる
            for &x_stab in measurement_qubit_x.iter() {
                network.h(x_stab);
            }

            // CNOT
            for (x, z) in x_order.iter().zip(z_order.iter()) {
                for (x_stab, z_stab) in measurement_qubit_x.iter().zip(measurement_qubit_z.iter()) {
                    let coord_data_z = (z_stab.0 + z.0, z_stab.1 + z.1);
                    let coord_data_x = (x_stab.0 + x.0, x_stab.1 + x.1);

                    // data bitが存在するときのみCNOT
                    if data_qubit.contains(&coord_data_z) {
                        network.cx(coord_data_z, *z_stab);
                    }
                    if data_qubit.contains(&coord_data_x) {
                        network.cx(*x_stab, coord_data_x);
                    }
                }
            }

            // XスタビライザーにHゲートを作用させる
            for &x_stab in measurement_qubit_x.iter() {
                network.h(x_stab);
            }

            // measurement qubitの測定
            // Z
            for (coord, register) in measurement_qubit_z.iter().zip(matrix_z.iter().flatten()) {
                network.measurement(*coord, Rc::clone(register));
            }
            // X
            for (coord, register) in measurement_qubit_x.iter().zip(matrix_x.iter().flatten()) {
                network.measurement(*coord, Rc::clone(register));
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

        // mesurement qubit の測定 (強制的に固有値+1に射影する)
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

        for coord in data_qubit.iter() {
            debug_assert!(coord.0 >= 0, "data coord must not be negative number");
            debug_assert!(coord.1 >= 0, "data coord must not be negative number");

            network.measurement(
                *coord,
                Rc::clone(&classical_register[(coord.0 / 2) as usize][(coord.1 / 2) as usize]),
            );
        }
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
    use super::*;

    #[test]
    fn gen_qec_code() {
        for distance in (3..27).step_by(2) {
            let code = RotatedSurfaceCode::new(distance, distance + 2, 0.01, 0);
            assert_eq!(
                distance * distance,
                code.measurement_qubit_z.len() + code.measurement_qubit_z.len() + 1
            )
        }
    }
}
