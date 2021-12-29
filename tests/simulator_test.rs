use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::cell::Cell;
use std::rc::Rc;

use clifford::simulator::{chp_simulator::CHPSimulator, SimulatorInterface};

#[test]
fn make_bell_state() {
    let mut count_0 = 0;
    let loop_num = 100000;
    for seed in 0..loop_num {
        let rng = SmallRng::seed_from_u64(seed);
        let mut sim = CHPSimulator::new(3, rng);

        let result = vec![Rc::new(Cell::new(0)); 2];
        sim.add_h(0);
        sim.add_cx(0, 1);
        sim.add_measurement(0, Rc::clone(&result[0]));
        sim.add_measurement(1, Rc::clone(&result[1]));

        sim.run();

        assert_eq!(result[0], result[1]);
        if result[0].get() == 0 {
            count_0 += 1;
        }
    }

    println!("{}, {}", count_0 as f32, loop_num as f32);
}
