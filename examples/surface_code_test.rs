use clifford::qec_code::rotated_surface_code::RotatedSurfaceCode;

fn main() {
    let loop_num = 10000;
    let distance = 5;
    let seed = 1;
    let mut code = RotatedSurfaceCode::new(distance, distance, 0.0001, 0.0001, seed);

    code.initialize();
    code.syndrome_measurement();

    let mut error_num = 0;
    let mut abnormal = 0;

    for i in 0..loop_num {
        code.reset();
        code.run();
        code.decode_mwpm(distance);

        let ans = code.logical_value();

        if ans == 1 {
            error_num += 1;
        }

        if ans == u8::MAX {
            abnormal += 1;
        }
        //println!("ans = {}, loop {}", ans, i);
        //println!("");
    }

    println!("abnormal: {}", abnormal);
    println!("error_num: {}", error_num);
    println!(
        "error rate: {}",
        (error_num + abnormal) as f32 / loop_num as f32
    );
}
