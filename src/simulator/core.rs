use std::cell::Cell;
use std::rc::Rc;

use super::Operation;

pub struct Dispatcher {
    operations: Vec<Operation>,
    round: usize,
}

/// シミュレータの内部からのみアクセスできるオペレーション
pub trait SimulatorCore {
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
    fn measurement(&mut self, a: usize, register: &Rc<Cell<u8>>);

    // measure as once
    //fn measurement_at_once(&mut self, a: Vec<usize>, register: &mut Array3<u8>);

    ///reset
    fn reset(&mut self);
}

impl Dispatcher {
    pub fn new(operations: Vec<Operation>, round: usize) -> Self {
        Dispatcher { operations, round }
    }

    pub fn push(&mut self, operation: Operation) {
        self.operations.push(operation);
    }

    pub fn operations(&self) -> &Vec<Operation> {
        &self.operations
    }

    pub fn round(&self) -> usize {
        self.round
    }
}
