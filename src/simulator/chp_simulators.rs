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

    /// CNOT gate
    pub fn cx(&mut self, a: usize, b: usize) {
        let (mut r, x_a, mut x_b, mut z_a, z_b) = self.stabilizer_tableau.multi_slice_mut((
            s![.., -1],
            s![.., a],
            s![.., b],
            s![.., self.qubit_num + a],
            s![.., self.qubit_num + b],
        ));

        // rを計算
        let buf = &x_a & &z_b & (&x_b ^ &z_a ^ 1);
        r ^= &buf;

        // xを計算
        x_b ^= &x_a;

        // zを計算
        z_a ^= &z_b;
    }

    /// Hadamard generate
    pub fn h(&mut self, a: usize) {
        let (mut r, mut x_a, mut z_a) = self.stabilizer_tableau.multi_slice_mut((
            s![.., -1],
            s![.., a],
            s![.., self.qubit_num + a],
        ));

        let buf = &x_a & &z_a;
        r ^= &buf;

        // XとZを入れ替える
        Zip::from(x_a).and(z_a).for_each(::std::mem::swap);
    }

    /// S gate (Phase gate)
    pub fn s(&mut self, a: usize) {
        let (mut r, x_a, mut z_a) = self.stabilizer_tableau.multi_slice_mut((
            s![.., -1],
            s![.., a],
            s![.., self.qubit_num + a],
        ));

        let buf = &x_a & &z_a;
        r ^= &buf;

        z_a ^= &x_a;
    }

    ///X gate
    pub fn x(&mut self, a: usize) {
        let (mut r, z_a) = self
            .stabilizer_tableau
            .multi_slice_mut((s![.., -1], s![.., self.qubit_num + a]));

        r ^= &z_a;
    }

    /// Z gate
    pub fn z(&mut self, a: usize) {
        let (mut r, x_a) = self
            .stabilizer_tableau
            .multi_slice_mut((s![.., -1], s![.., a]));

        r ^= &x_a;
    }

    fn g(&self, x1: u8, z1: u8, x2: u8, z2: u8) -> u8 {
        match (x1, z1, x2, z2) {
            (0, 0, _, _) => 0,
            (1, 1, _, _) => z2 - x2,
            (1, 0, _, _) => z2 * (2 * x2 - 1),
            (0, 1, _, _) => x2 * (1 - 2 * z2),
            _ => panic!("Tableau parameters must be 0 or 1"),
        }
    }

    fn row_sum(&mut self, h: usize, i: usize) {
        let mut g_sum = 0;

        for j in 0..self.qubit_num {
            g_sum += self.g(
                self.stabilizer_tableau[[i, j]],
                self.stabilizer_tableau[[i, self.qubit_num + j]],
                self.stabilizer_tableau[[h, j]],
                self.stabilizer_tableau[[h, self.qubit_num + j]],
            );
        }

        let checker = 2 * self.stabilizer_tableau[[h, self.qubit_num * 2]]
            + 2 * self.stabilizer_tableau[[i, self.qubit_num * 2]]
            + g_sum;
        
        match checker % 4 {
            2 => self.stabilizer_tableau[[h, self.qubit_num * 2]] = 1,
            0 => self.stabilizer_tableau[[h, self.qubit_num * 2]] = 0,
            _ => panic!("Error at row_sum"),
        }

        let (mut row_h, row_i) = self.stabilizer_tableau.multi_slice_mut((s![h, ..self.qubit_num * 2], s![i, ..self.qubit_num * 2]));

        row_h ^= &row_i;
    }

    fn row_sum_temp(&self, i: usize, temp: &mut Array1<u8>) {
        let mut g_sum = 0;

        for j in 0..self.qubit_num {
            g_sum += self.g(
                self.stabilizer_tableau[[i, j]],
                self.stabilizer_tableau[[i, self.qubit_num + j]],
                temp[j],
                temp[self.qubit_num + j],
            );
        }

        let checker = 2 * temp[temp.len() - 1] + 2 * self.stabilizer_tableau[[i, self.qubit_num * 2]] + g_sum;
        
        match checker % 4 {
            2 => temp[temp.len() - 1] = 1,
            0 => temp[temp.len() - 1] = 0,
            _ => panic!("Error at row_sum"),
        }

        let row_i = self.stabilizer_tableau.slice(s![i, ..self.qubit_num * 2]);
        
        *temp += &row_i;
    }

    /// measutement
    pub fn measurement(self, a: usize) {
        //let outcome_is_random = false;
        //let mut p: Vec<u32> =  Vec::new();

        
    }
}
