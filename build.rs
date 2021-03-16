fn main() -> Result<(), Box<dyn std::error::Error>> {

    let chap2_iface_files = &["api/v1/protos/product_info.proto"];
    let dirs = &["."];

    // Chapter 2: Unary gRPC Pattern
    tonic_build::configure()
        .compile(chap2_iface_files, dirs)
        .unwrap();
    
    Ok(())
}
