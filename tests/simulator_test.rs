use rand::{Rng, SeedableRng, rngs::SmallRng};

use clifford::simulator::{chp_simulator::CHPSimulator, Simulator};

#[test]
fn make_bell_state() {
    let seed = 0;
    let rng = SmallRng::seed_from_u64(seed);
    let mut sim = CHPSimulator::new(3, rng);

    sim.h(0);
    sim.cx(0, 1);
    
    let result = (sim.measurement(0), sim.measurement(1));

    println!("{:?}", result);
    assert_eq!(result.0, result.1);
}