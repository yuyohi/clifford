use clifford::qec_code::rotated_surface_code::RotatedSurfaceCode;
use indicatif::ProgressBar;

fn main() {
    let loop_num = 10000;
    let distance = [3, 5, 7];
    let error_rate = [0.0001, 0.0005, 0.001, 0.003, 0.005, 0.007, 0.01];
    let seed = 1;

    let loop_time = distance.len() * error_rate.len();
    let mut count = 1;

    let mut result = vec![Vec::new(); distance.len()];

    for (&d, r) in distance.iter().zip(result.iter_mut()) {
        for &p in error_rate.iter() {
            println!("Progress {}/{}", count, loop_time);

            let mut code = RotatedSurfaceCode::new(d, d, p, p, seed);

            code.initialize();
            code.syndrome_measurement();

            let mut error_num = 0;

            let bar = ProgressBar::new(loop_num);

            for _ in 0..loop_num {
                code.reset();
                code.run();
                code.decode_mwpm(d);

                let ans = code.logical_value();

                if ans != 0 {
                    error_num += 1;
                }

                bar.inc(1)
            }

            r.push(error_num as f32 / loop_num as f32);

            bar.finish();

            count += 1;
        }
    }

    println!("{:?}", result);
}
