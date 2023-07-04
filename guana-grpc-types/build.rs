use std::{env, path::PathBuf};

fn main() -> std::io::Result<()> {
    // Note: OUT_DIR is guaranteed to be set by Cargo.
    // See https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    let fd_descriptor_set = {
        let artifacts_dir: PathBuf =
            env::var_os("OUT_DIR").unwrap().into();

        artifacts_dir.join("user_agent_analyzer_descriptor.bin")
    };

    tonic_build::configure()
        .file_descriptor_set_path(fd_descriptor_set)
        .emit_rerun_if_changed(true)
        .compile(
            &["../proto/user_agent_analyzer.proto"],
            &["../proto/"],
        )
}
