use gds2_parser::parse_gds2;

#[test]
fn test_1() {
    let gds2_data = parse_gds2(&format!(
        "{}/testcases/sim_c2.gds",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
    ))
    .unwrap();
    println!("{:?}", gds2_data.structure_name);
}
