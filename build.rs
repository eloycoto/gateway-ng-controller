fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().out_dir("src/protobuf").compile(
        &[
            "./protos/envoyproxy/data-plane-api/envoy/config/cluster/v3/cluster.proto",
            "./protos/envoyproxy/data-plane-api/envoy/service/cluster/v3/cds.proto",
        ],
        &[
            "./protos/envoyproxy/data-plane-api/",
            "./protos/googleapies/",
            "./protos/envoyproxy/protoc-gen-validate/",
            "./protos/cncf/udpa/",
        ],
    )?;

    Ok(())
}
