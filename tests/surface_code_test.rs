use clifford::qec_code::rotated_surface_code::RotatedSurfaceCode;


#[test]
fn gen_qec_code() {
    let distance = 5;
    let code = RotatedSurfaceCode::new(distance, 0.01, 0);
    assert_eq!(distance * distance, code.measurement_qubit_z.len())
}