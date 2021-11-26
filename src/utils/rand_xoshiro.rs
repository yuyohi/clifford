use rand::SeedableRng;

fn main() {
    let rng = rand_xoshiro::Xoshiro256StarStar::seed_from_u64(12);
}
