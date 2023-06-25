use std::{env, path::PathBuf};

use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional") // for older systems
        .build_client(true)
        .build_server(true)
        .out_dir("./proto/v1/")
        .file_descriptor_set_path(out_dir.join("scheduler_v1_descriptor.bin"))
        .compile(
            &[
                "./proto/v1/cmn-model.proto",
                "./proto/v1/scheduler-model.proto",
                "./proto/v1/scheduler-service.proto",
            ],
            &["proto/v1"],
        )?;

    Ok(())
}
