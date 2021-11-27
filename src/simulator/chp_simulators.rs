use ndarray::*;
use rand::SeedableRng;

pub struct Simulator<T: SeedableRng> {
    qubit_num: usize,
    stabilizer_tableau: Array2<u8>,
    rng: T,
}

impl<T: SeedableRng> Simulator<T> {
    pub fn new(qubit_num: usize, rng: T) -> Self {
        let size = qubit_num * 2;
        let stabilizer_tableau: Array2<u8> =
            concatenate![Axis(1), Array::eye(size), Array::zeros((size, 1))];

        Simulator::<T> {
            qubit_num,
            stabilizer_tableau,
            rng,
        }
    }

    pub fn cx(&mut self, a: usize, b: usize) {
        self.stabilizer_tableau.slice_mut(s![.., -1]).assign(
            (self.stabilizer_tableau.slice(s![.., -1])
                ^ self.stabilizer_tableau.slice(s![.., a])
                    * self.stabilizer_tableau.slice(s![.., self.qubit_num + b])).to_owned()
        )
    }
}
