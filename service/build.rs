use std::{env, path::PathBuf};  
use copy_to_output::copy_to_output;
  
fn main() {  
    // Way for re-runs script if any files under env folder have changed  
    // println!("cargo:rerun-if-changed=env/*");
    // Having re-runs unconditional
    let output_dir = format!("{}/{}", env::var("TARGET").unwrap(), env::var("PROFILE").unwrap());
    copy_to_output("env", &output_dir).expect("Could not copy");

    // Using custom Tonic build configuration for more flexibility and possible way to extend
    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("protos_descriptor.bin");
    tonic_build::configure()
        .file_descriptor_set_path(descriptor_path)
        // In general - could have multiple .proto files below
        .compile(&["../protos/voting.proto"], &["../protos/"]).expect("Could not compile protos");
}
