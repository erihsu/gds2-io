use gds2_io::parse_gds2;

#[test]
fn test_1() {
    let gds2_data = parse_gds2(&format!(
        "{}/testcases/sim_c1.gds",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
    ))
    .unwrap();
    println!(
        "header: {}",
        gds2_data
            .header
            .iter()
            .map(|(k, v)| format!("{}:{:?} \n", k, v))
            .collect::<String>()
    );
    println!("struct name: {:?}", gds2_data.structure_name);
    let _ = gds2_data.save_gds2(&format!(
        "{}/target/sim_c1_out.gds",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
    ));
    println!("write done");
    let gds2_data = parse_gds2(&format!(
        "{}/target/sim_c1_out.gds",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
    ))
    .unwrap();
    println!(
        "header: {}",
        gds2_data
            .header
            .iter()
            .map(|(k, v)| format!("{}:{:?} \n", k, v))
            .collect::<String>()
    );
    println!("struct name: {:?}", gds2_data.structure_name);
}

#[test]
fn test_2() {
    let gds2_data = parse_gds2(&format!(
        "{}/testcases/sim_c6.gds",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
    ))
    .unwrap();
    println!(
        "header: {}",
        gds2_data
            .header
            .iter()
            .map(|(k, v)| format!("{}:{:?} \n", k, v))
            .collect::<String>()
    );
    println!("struct name: {:?}", gds2_data.structure_name);
    let _ = gds2_data.save_gds2(&format!(
        "{}/target/sim_c6_out.gds",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
    ));
    println!("write done");
    let gds2_data = parse_gds2(&format!(
        "{}/target/sim_c6_out.gds",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
    ))
    .unwrap();
    println!(
        "header: {}",
        gds2_data
            .header
            .iter()
            .map(|(k, v)| format!("{}:{:?} \n", k, v))
            .collect::<String>()
    );
    println!("struct name: {:?}", gds2_data.structure_name);
}
