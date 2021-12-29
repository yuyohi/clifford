use clifford::qec_code::rotated_surface_code::RotatedSurfaceCode;

#[test]
fn gen_qec_code() {
    let distance = 3;
    let seed = 10;
    let mut code = RotatedSurfaceCode::new(distance, distance + 1, 0.01, seed);

    code.initialize();
    code.logical_measurement();
    code.run();

    let result = code.classical_register();

    let result_vec: Vec<Vec<u8>> = result
        .iter()
        .map(|row| row.iter().map(|value| value.get()).collect())
        .collect();

    let rowsum: Vec<u8> = result_vec.iter().map(|row| row.iter().sum()).collect();

    println!("rowsum: {:?}", rowsum);

    for row in result_vec.iter() {
        println!("{:?}", row);
    }

    for value in rowsum {
        assert!(value % 2 == 0);
    }
}
