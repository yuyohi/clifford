use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::collections::HashMap;

use crate::simulator::{self, Simulator, SimulatorWrapper};

pub struct QubitNetwork {
    network: HashMap<(i32, i32), Vec<(i32, i32)>>,
    bit_error_map: HashMap<(i32, i32), f32>,
    connection_error_map: HashMap<((i32, i32), (i32, i32)), f32>,
    index_to_sim: HashMap<(i32, i32), usize>,
    sim: SimulatorWrapper,
    rng: rand::rngs::SmallRng,
}

impl QubitNetwork {
    /// rotated surface codeに適したlatticeを作成する
    pub fn new_rotated_planer_lattice(
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
        // ancilla qubitを追加 (dual lattice)
        for x in (-1..horizontal as i32 * 2 + 1).step_by(2) {
            for y in (-1..vertical as i32 * 2 + 1).step_by(2) {
                qubit_index.push((x, y));
            }
        }

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
            CHPSimulator => simulator::SimulatorWrapper::CHPSimulator(
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

    /// ゲート操作
    /// CNOT gate
    pub fn cx(&mut self, a: (i32, i32), b: (i32, i32)) {
        debug_assert!(self.connection_error_map.contains_key(&(a, b)));

        self.sim.cx(
            *self.index_to_sim.get(&a).expect("index does not exist"),
            *self.index_to_sim.get(&b).expect("index does not exist"),
        );
    }

    /// H gate
    pub fn h(&mut self, a: (i32, i32)) {
        self.sim
            .h(*self.index_to_sim.get(&a).expect("index does not exist"));
    }

    /// S gate
    pub fn s(&mut self, a: (i32, i32)) {
        self.sim
            .s(*self.index_to_sim.get(&a).expect("index does not exist"));
    }

    /// x gate
    pub fn x(&mut self, a: (i32, i32)) {
        self.sim
            .x(*self.index_to_sim.get(&a).expect("index does not exist"));
    }

    /// z gate
    pub fn z(&mut self, a: (i32, i32)) {
        self.sim
            .z(*self.index_to_sim.get(&a).expect("index does not exist"));
    }

    /// measurement
    pub fn measurement(&mut self, a: (i32, i32)) -> u8 {
        self.sim
            .measurement(*self.index_to_sim.get(&a).expect("index does not exist"))
    }
}
