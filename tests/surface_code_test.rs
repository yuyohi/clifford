use clifford::qec_code::rotated_surface_code::RotatedSurfaceCode;

#[test]
fn gen_qec_code() {
    let distance = 3;
    let seed = 10;
    let mut code = RotatedSurfaceCode::new(distance, distance + 1, 0.0,0.0, seed);

    for _ in 0..100{
        code.reset();
        code.initialize();
        code.syndrome_measurement();
        code.run();
        code.decode_mwpm(distance);

        let result = code.classical_register();

        let result_vec: Vec<Vec<u8>> = result
            .iter()
            .map(|row| row.iter().map(|value| value.get()).collect())
            .collect();

        let row_sum: Vec<u8> = result_vec
            .clone()
            .into_iter()
            .reduce(|row_a, row_b| row_a.iter().zip(row_b.iter()).map(|(&a, &b)| a + b).collect())
            .unwrap();

        //println!("row_sum: {:?}", row_sum);

        //for row in result_vec.iter() {
            //println!("{:?}", row);
        //}

        for value in row_sum {
            assert!(value % 2 == 0);
        }

        let ans = code.logical_value();
        assert!(ans == 0);
    }
}

#[test]
fn test_syndrome() {
    let distance = 17;
    let seed = 10;
    let mut code = RotatedSurfaceCode::new(distance, distance + 1, 0.01, 0.01, seed);

    code.initialize();

    code.syndrome_measurement();

    code.run()
}

#[test]
fn test_decode_scheme() {
    let distance = 3;
    let seed = 10;
    let mut code = RotatedSurfaceCode::new(distance, distance + 1, 0.01,0.01,  seed);

    code.initialize();
    code.run();
    
    code.decode_mwpm(distance);
    let ans = code.logical_value();

    println!("{}", ans);
}
