use ndarray::prelude::*;
use ndarray::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::simulator::{self, SimulatorInterface, SimulatorWrapper, Type};

pub struct QubitNetwork {
    network: HashMap<(i32, i32), Vec<(i32, i32)>>,
    bit_error_map: HashMap<(i32, i32), f32>,
    connection_error_map: HashMap<((i32, i32), (i32, i32)), f32>,
    index_to_sim: HashMap<(i32, i32), usize>,
    sim: SimulatorWrapper,
    rng: rand::rngs::SmallRng,
}

impl QubitNetwork {
    /// 縦横の大きさからrotated surface codeに適したlatticeを作成する
    pub fn new_rotated_planer_lattice_from_rectangle(
        vertical: usize,
        horizontal: usize,
        p: f32,
        sim_type: simulator::Type,
        seed: u64,
    ) -> Self {
        // node一覧を作成
        // data_qubitを追加
        let mut qubit_index: Vec<(i32, i32)> = vec![];
        for x in (0..horizontal as i32 * 2).step_by(2) {
            for y in (0..vertical as i32 * 2).step_by(2) {
                qubit_index.push((x, y));
            }
        }
        // measurement qubitを追加 (dual lattice)
        for x in (-1..horizontal as i32 * 2 + 1).step_by(2) {
            for y in (-1..vertical as i32 * 2 + 1).step_by(2) {
                qubit_index.push((x, y));
            }
        }

        Self::new_rotated_planer_lattice(qubit_index, p, sim_type, seed)
    }

    /// 受け取ったvecを元にrotated surface codeに適したlatticeを作成する
    pub fn new_rotated_planer_lattice_from_vec(
        data_qubit: Vec<(i32, i32)>,
        measurement_qubit_z: Vec<(i32, i32)>,
        measurement_qubit_x: Vec<(i32, i32)>,
        p: f32,
        sim_type: simulator::Type,
        seed: u64,
    ) -> Self {
        let mut qubit_index = Vec::new();
        qubit_index.extend(data_qubit);
        qubit_index.extend(measurement_qubit_z);
        qubit_index.extend(measurement_qubit_x);

        Self::new_rotated_planer_lattice(qubit_index, p, sim_type, seed)
    }

    /// gen rotated_surface_lattice
    fn new_rotated_planer_lattice(
        qubit_index: Vec<(i32, i32)>,
        p: f32,
        sim_type: simulator::Type,
        seed: u64,
    ) -> Self {
        let mut network = HashMap::new();

        // 斜めにedgeを追加
        let direction = [(1, 1), (-1, 1), (-1, -1), (1, -1)];
        for &u in qubit_index.iter() {
            for &d in direction.iter() {
                match (u.0 + d.0, u.1 + d.1) {
                    v if qubit_index.contains(&v) => {
                        network.entry(u).or_insert_with(|| vec![]).push(v);
                    }
                    _ => (),
                }
            }
        }

        // qubitのerror rate dictを作成
        let mut bit_error_map = HashMap::new();
        for &qubit in qubit_index.iter() {
            bit_error_map.insert(qubit, p);
        }
        // connectionのerror rate dictを作成
        let mut connection_error_map = HashMap::new();
        for &u in qubit_index.iter() {
            for &v in qubit_index.iter() {
                connection_error_map.insert((u, v), p);
            }
        }

        // simulatorを生成
        // 乱数発生器は仮
        let mut rng = SmallRng::seed_from_u64(seed);
        let mut rng_sim = SmallRng::seed_from_u64(seed + 1);

        let sim = match sim_type {
            Type::CHPSimulator => simulator::SimulatorWrapper::CHPSimulator(
                simulator::chp_simulator::CHPSimulator::new(qubit_index.len(), rng_sim),
            ),
        };
        // (i32, i32)のindexからsimulatorのindexのusizeへのmap
        let mut index_to_sim = HashMap::new();
        for (i, &coord) in qubit_index.iter().enumerate() {
            index_to_sim.insert(coord, i);
        }
        QubitNetwork {
            network,
            bit_error_map,
            connection_error_map,
            index_to_sim,
            sim,
            rng,
        }
    }

    /// 指定したqubitのerror rateを返す
    pub fn qubit_error_rate(&self, index: (i32, i32)) -> f32 {
        let p = self
            .bit_error_map
            .get(&index)
            .expect("index does not exist");
        p.clone()
    }

    /// 指定したconnectionのerror rateを返す
    pub fn connection_error_rate(&self, index: ((i32, i32), (i32, i32))) -> f32 {
        let p = self
            .connection_error_map
            .get(&index)
            .expect("index does not exist");
        p.clone()
    }

    /// ゲート操作を追加する
    /// CNOT gate
    pub fn cx(&mut self, a: (i32, i32), b: (i32, i32)) {
        debug_assert!(self.connection_error_map.contains_key(&(a, b)));

        self.sim.add_cx(
            *self.index_to_sim.get(&a).expect("index does not exist"),
            *self.index_to_sim.get(&b).expect("index does not exist"),
        );
    }

    /// H gate
    pub fn h(&mut self, a: (i32, i32)) {
        self.sim
            .add_h(*self.index_to_sim.get(&a).expect("index does not exist"));
    }

    /// S gate
    pub fn s(&mut self, a: (i32, i32)) {
        self.sim
            .add_s(*self.index_to_sim.get(&a).expect("index does not exist"));
    }

    /// x gate
    pub fn x(&mut self, a: (i32, i32)) {
        self.sim
            .add_x(*self.index_to_sim.get(&a).expect("index does not exist"));
    }

    /// z gate
    pub fn z(&mut self, a: (i32, i32)) {
        self.sim
            .add_z(*self.index_to_sim.get(&a).expect("index does not exist"));
    }

    /// measurement
    pub fn measurement(&mut self, a: (i32, i32), register: Rc<Cell<u8>>) {
        self.sim.add_measurement(
            *self.index_to_sim.get(&a).expect("index does not exist"),
            register,
        );
    }

    /// arrayに測定結果を格納
    //pub fn measurement_at_once(&mut self, a: &Vec<(i32, i32)>, register: &mut Array3<u8>) {
    //    let a = a.iter().map(|coord| self.index_to_sim.get(&coord));
    //    self.sim
    //        .add_measurement_at_once(a, result);
    //}

    pub fn measurement_to_zero(&mut self, a: (i32, i32)) {
        self.sim
            .add_measurement_to_zero(*self.index_to_sim.get(&a).expect("index does not exist"));
    }

    /// 指定された座標がネットワークに存在するかを判定する
    pub fn check_contains(&self, a: (i32, i32)) -> bool {
        self.network.contains_key(&a)
    }

    /// 回路を実行する
    pub fn run(&mut self) {
        self.sim.run();
    }

    /// get index_to_sim
    pub fn index_to_sim(&self) -> &HashMap<(i32, i32), usize> {
        &self.index_to_sim
    }
}
