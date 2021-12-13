use self::chp_simulator::CHPSimulator;

pub mod chp_simulator;

pub trait Simulator {
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
    fn measurement(&mut self, a: usize) -> u8;

    /// Reset stabilizer tableau
    fn reset(&mut self);
}

pub enum SimulatorWrapper {
    CHPSimulator(CHPSimulator),
}

impl Simulator for SimulatorWrapper {
    fn cx(&mut self, a: usize, b: usize) {
        match *self {
            SimulatorWrapper::CHPSimulator(ref mut sim) => sim.cx(a, b),
        };
    }

    fn h(&mut self, a: usize) {
        match *self {
            SimulatorWrapper::CHPSimulator(ref mut sim) => sim.h(a),
        };
    }

    fn s(&mut self, a: usize) {
        match *self {
            SimulatorWrapper::CHPSimulator(ref mut sim) => sim.s(a),
        };
    }

    fn x(&mut self, a: usize) {
        match *self {
            SimulatorWrapper::CHPSimulator(ref mut sim) => sim.x(a),
        };
    }

    fn z(&mut self, a: usize) {
        match *self {
            SimulatorWrapper::CHPSimulator(ref mut sim) => sim.z(a),
        };
    }

    fn measurement(&mut self, a: usize) -> u8 {
        match *self {
            SimulatorWrapper::CHPSimulator(ref mut sim) => sim.measurement(a),
        }
    }

    fn reset(&mut self) {
        match *self {
            SimulatorWrapper::CHPSimulator(ref mut sim) => sim.reset(),
        };
    }
}

pub enum Type {
    CHPSimulator,
}
