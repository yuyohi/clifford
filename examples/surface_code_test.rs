use clifford::qec_code::rotated_surface_code::RotatedSurfaceCode;
use colored::*;

fn main() {
    let loop_num = 10000;
    let distance = 7;
    let seed = 10;
    let mut code = RotatedSurfaceCode::new(distance, distance, 0.01, 0.01, seed);

    code.initialize();
    code.syndrome_measurement();

    let mut error_num = 0;
    let mut abnormal = 0;

    for i in 0..loop_num {
        code.reset();
        code.run();
        let graph = code.decode_mwpm(distance);

        let ans = code.logical_value();

        if ans == 1 {
            error_num += 1;
        }

        if ans == u8::MAX {
            abnormal += 1;
        }
        println!("ans = {}, loop {}", ans, i);
        if ans != 0 {
            println!("{}", "#########################################################################################\nerror\n#########################################################################################".red());
            // break;
        }
        println!("");
    }

    println!("abnormal: {}", abnormal);
    println!("error_num: {}", error_num);
    println!(
        "error rate: {}",
        (error_num + abnormal) as f32 / loop_num as f32
    );
}
