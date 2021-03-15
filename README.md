# gRPC Adventures with Rust

- Examples based on the Book __gRPC: Up and Running__

## Build / Usage

Build the ptotocol buffer stub using

    cargo build

Run Server using

    cargo run --bin ecomm-server

## grpcurl

Download the binary tarball from [grpcurl](https://github.com/fullstorydev/grpcurl/releases)

```bash

$ cd /tmp/ && wget https://github.com/fullstorydev/grpcurl/releases/download/v1.8.0/grpcurl_1.8.0_linux_x86_64.tar.gz

$ tar -xf grpcurl_1.8.0_linux_x86_64.tar.gz
$ sudo mv grpcurl /usr/local/bin/

```

### Testing Server

__addProduct Service__

```bash
grpcurl -plaintext -import-path ./api/v1/protos/ \
> -proto productinfo.proto \
> -d '{id: "1", "name": "ProductA", "description": "Amazing Product"}' \
> [::]:50051 ecommerce.ProductInfo/addProduct 
```

__getProduct Service__

```bash
grpcurl -plaintext -import-path ./api/v1/protos/ -proto productinfo.proto -d '{"value": "f69bf181-fb13-45ed-bdae-a6f9811d1a9a" }' [::]:50051 ecommerce.ProductInfo/getProduct
```

## Resources / Reference

- [Dev.to Blog by Anshul Goyal: "Beginners Guide to gRPC with Rust"](https://dev.to/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o)
- [SwiftDiaries: gRPC using Rust's Tonic](https://www.swiftdiaries.com/rust/tonic/)


## NOTES

- Generated Rust file in `target/debug/gprc-rs-<hash>/out/<your_grpc_package_name>.rs`