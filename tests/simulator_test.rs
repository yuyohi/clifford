use rand::{Rng, SeedableRng, rngs::SmallRng};

use clifford::simulator::{chp_simulator::CHPSimulator, SimulatorExternal};

#[test]
fn make_bell_state() {
    let mut count_0 = 0;
    let loop_num = 1000;
    for seed in 0..loop_num {
        let rng = SmallRng::seed_from_u64(seed);
        let mut sim = CHPSimulator::new(3, rng);

        sim.h(0);
        sim.cx(0, 1);
        
        let result = (sim.measurement(0), sim.measurement(1));

        assert_eq!(result.0, result.1);
        if result.0 == 0 {
            count_0 += 1;
        }
    }

    println!("{}", count_0 as f32 / loop_num as f32);
}