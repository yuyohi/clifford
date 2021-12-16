use ndarray::Array2;
use petgraph::graph::{Graph, UnGraph};
use rand::{self, Rng};
use std::collections::HashMap;

use crate::qubit_network::QubitNetwork;
use crate::simulator;
use crate::simulator::Type;

pub struct RotatedSurfaceCode<'a> {
    network: QubitNetwork<'a>,
    measurement_qubit_z: Vec<(i32, i32)>,
    measurement_qubit_x: Vec<(i32, i32)>,
    data_qubit: Vec<(i32, i32)>,
    Syndrome_result: Array2<u8>,
}

impl<'a> RotatedSurfaceCode<'a> {
    pub fn new(distance: usize, p: f32, seed: u64) -> Self {
        if distance % 2 == 0 {
            panic!("distance must be odd number.");
        }

        let network = QubitNetwork::new_rotated_planer_lattice(
            distance,
            distance,
            p,
            Type::CHPSimulator,
            seed,
        );

        // 測定bitの生成
        let (measurement_qubit_z, measurement_qubit_x) =
            RotatedSurfaceCode::gen_measurement_qubit(distance);
        // data bitの生成
        let mut data_qubit: Vec<(i32, i32)> = vec![];
        for x in (0..distance as i32 * 2).step_by(2) {
            for y in (0..distance as i32 * 2).step_by(2) {
                data_qubit.push((x, y));
            }
        }

        // 座標がネットワーク上に存在するかをチェック
        debug_assert!(measurement_qubit_z
            .iter()
            .all(|&coord| network.check_contains(coord)));
        debug_assert!(measurement_qubit_x
            .iter()
            .all(|&coord| network.check_contains(coord)));
        debug_assert!(data_qubit
            .iter()
            .all(|&coord| network.check_contains(coord)));

        Self {
            network,
            measurement_qubit_z,
            measurement_qubit_x,
            data_qubit,
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

    pub fn syndrome_measurement(&mut self, round: usize) {
        let x_order = ((1, 1), (1, -1), (-1, 1), (-1, -1));
        let z_order = ((1, 1), (-1, 1), (1, -1), (-1, -1));

        for i in 0..round {
            // XスタビライザーにHゲートを作用させる
            for x_stab in self.measurement_qubit_x {
                self.network.h(x_stab);
            }

            // CNOT 
            
                

            // XスタビライザーにHゲートを作用させる
            for x_stab in self.measurement_qubit_x {
                self.network.h(x_stab);
            } 
        }

        unimplemented!();
    }
}

mod test {
    use super::*;

    #[test]
    fn gen_qec_code() {
        for distance in (3..21).step_by(2) {
            let code = RotatedSurfaceCode::new(distance, 0.01, 0);
            assert_eq!(
                distance * distance,
                code.measurement_qubit_z.len() + code.measurement_qubit_z.len() + 1
            )
        }
    }
}
