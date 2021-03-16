# gRPC Adventures with Rust

- Examples based on the Book __gRPC: Up and Running__

## Build / Usage

Build the ptotocol buffer stub using

    cargo build

Run Server using

    cargo run --bin chXX/<binary_name>

## grpcurl (gRPC Client)

### Installation

Download the binary tarball from [grpcurl](https://github.com/fullstorydev/grpcurl/releases)

```bash

$ cd /tmp/ && wget https://github.com/fullstorydev/grpcurl/releases/download/v1.8.0/grpcurl_1.8.0_linux_x86_64.tar.gz

$ tar -xf grpcurl_1.8.0_linux_x86_64.tar.gz
$ sudo mv grpcurl /usr/local/bin/

```

### Usage

General Syntax for `grpcurl`:

```bash
grpcurl -plaintext -import-path ./api/<version_chapter_number>/protos/ \
> -proto <proto_file_name>.proto \
> -d '<REQUEST_PAYLOAD_HERE>' \
> [::]:50051 <protobuf_package_name>.<ServiceName>/<ParticularService> 
```

## Resources / Reference

- [Dev.to Blog by Anshul Goyal: "Beginners Guide to gRPC with Rust"](https://dev.to/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o)
- [SwiftDiaries: gRPC using Rust's Tonic](https://www.swiftdiaries.com/rust/tonic/)


## NOTES

- Generated Rust file in `target/debug/gprc-rs-<hash>/out/<your_grpc_package_name>.rs`