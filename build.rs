fn main() -> Result<(), Box<dyn std::error::Error>> {

    let chap2_iface_files = &["api/v1/protos/product_info.proto"];
    let dirs = &["."];

    // Chapter 2: Unary gRPC Pattern
    tonic_build::configure()
        .compile(chap2_iface_files, dirs)
        .unwrap();
    
    // Chapter 3: gRPC Communication Patterns
    let chap3_iface_files = &["api/v2/protos/order_mgmt.proto"];
    tonic_build::configure()
        .compile(chap3_iface_files, dirs)
        .unwrap();
    Ok(())
}
