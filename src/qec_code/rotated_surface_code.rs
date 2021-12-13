use ndarray;
use petgraph::graph::{Graph, UnGraph};
use rand::{self, Rng};
use std::collections::HashMap;

use crate::simulator;
use crate::qubit_network;

pub struct RotatedSurfaceCode<T>
where
    T: simulator::Simulator,
{
    circuit: qubit_network::QubitNetwork,
    measurement_qubit_z: Vec<(i32, i32)>,
    measurement_qubit_x: Vec<(i32, i32)>,
    data_qubit: Vec<(i32, i32)>,

    // シミュレーション用
    sim: T,
}

impl<T> RotatedSurfaceCode<T>
where
    T: simulator::Simulator,
{
    pub fn new(distance: usize, p: HashMap<(i32, i32), f32>) {
        if distance % 2 == 0 {
            panic!("distance must be odd number.");
        }

        // 測定bitの生成
        let (measurement_qubit_z, measurement_qubit_x) =
            RotatedSurfaceCode::<T>::gen_measurement_qubit(distance);
        // data bitの生成
        let mut data_qubit: Vec<(i32, i32)> = vec![];
        for x in (0..distance as i32 * 2).step_by(2) {
            for y in (0..distance as i32 * 2).step_by(2) {
                data_qubit.push((x, y));
            }
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
            for x in (-1..distance as i32 * 2).step_by(2) {
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

    fn gen_qubit_network(
        measurement_qubit_z: &Vec<(i32, i32)>,
        measurement_qubit_x: &Vec<(i32, i32)>,
        data_qubit: &Vec<(i32, i32)>,
        p: &HashMap<(i32, i32), f32>
    ) -> qubit_network::QubitNetwork {

        unimplemented!()
    }

    pub fn syndrome_measurement(&mut self, round: usize) {
        unimplemented!();
    }
}
