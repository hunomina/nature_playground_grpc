fn main() {
    // generate protobuf code
    tonic_build::configure()
        .type_attribute(
            "nature.playground.Plant",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .compile(&["proto/main.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
