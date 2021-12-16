use self::chp_simulator::CHPSimulator;

pub mod chp_simulator;

/// シミュレータの外部からもアクセスできるオペレーション
pub trait SimulatorExternal<'a> {
    /// add CNOT gate
    fn add_cx(&mut self, a: usize, b: usize);

    /// add Hadamard gate
    fn add_h(&mut self, a: usize);

    /// add S gate (Phase gate)
    fn add_s(&mut self, a: usize);

    /// add X gate
    fn add_x(&mut self, a: usize);

    /// add Z gate
    fn add_z(&mut self, a: usize);

    /// add measurement
    fn add_measurement(&'a mut self, a: usize, result: &'a mut u8);

    // add measure as once
    // fn add_measurement_at_once(&mut self, a: Vec<usize>, register: &mut u8);

    /// add Reset stabilizer tableau
    fn reset(&mut self);

    /// add run circuit
    fn run(&mut self);
}

/// シミュレータの内部からのみアクセスできるオペレーション
pub trait SimulatorInternal {
    /// CNOT gate
    fn cx(&mut self, a: usize, b: usize);

    /// Hadamard gate
    fn h(&mut self, a: usize);

    /// S gate (Phase gate)
    fn s(&mut self, a: usize);

    ///X gate
    fn x(&mut self, a: usize);

    /// Z gate
    fn z(&mut self, a: usize);

    /// measurement
    fn measurement(&mut self, a: usize, result: &mut u8);

    // measure as once
    //fn measurement_at_once(&mut self, a: Vec<usize>, register: &mut u8);
}

pub enum SimulatorWrapper<'a> {
    CHPSimulator(CHPSimulator<'a>),
}

impl<'a> SimulatorExternal<'a> for SimulatorWrapper<'a> {
    fn add_cx(&mut self, a: usize, b: usize) {
        match *self {
            SimulatorWrapper::CHPSimulator(ref mut sim) => sim.add_cx(a, b),
        };
    }

    fn add_h(&mut self, a: usize) {
        match *self {
            SimulatorWrapper::CHPSimulator(ref mut sim) => sim.add_h(a),
        };
    }

    fn add_s(&mut self, a: usize) {
        match *self {
            SimulatorWrapper::CHPSimulator(ref mut sim) => sim.add_s(a),
        };
    }

    fn add_x(&mut self, a: usize) {
        match *self {
            SimulatorWrapper::CHPSimulator(ref mut sim) => sim.add_x(a),
        };
    }

    fn add_z(&mut self, a: usize) {
        match *self {
            SimulatorWrapper::CHPSimulator(ref mut sim) => sim.add_z(a),
        };
    }

    fn add_measurement(&'a mut self, a: usize, result: &'a mut u8) {
        match *self {
            SimulatorWrapper::<'a>::CHPSimulator(ref mut sim) => sim.add_measurement(a, result),
        }
    }

    fn reset(&mut self) {
        match *self {
            SimulatorWrapper::CHPSimulator(ref mut sim) => sim.reset(),
        };
    }

    fn run(&mut self) {
        match *self {
            SimulatorWrapper::CHPSimulator(ref mut sim) => sim.run(),
        }
    }
}

pub enum Operation<'a> {
    CX(usize, usize),
    H(usize),
    S(usize),
    X(usize),
    Z(usize),
    M(usize, &'a mut u8),
}

pub enum Type {
    CHPSimulator,
}
