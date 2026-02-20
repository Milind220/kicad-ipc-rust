use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn collect_proto_files(root: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_proto_files(&path, out)?;
            continue;
        }

        if path.extension().and_then(|ext| ext.to_str()) == Some("proto") {
            out.push(path);
        }
    }

    Ok(())
}

fn main() {
    let proto_root = std::env::var_os("KICAD_PROTO_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("kicad/api/proto"));

    if !proto_root.exists() {
        panic!(
            "KiCad proto root not found at '{}'. Initialize submodule with `git submodule update --init --recursive` or set KICAD_PROTO_ROOT.",
            proto_root.display()
        );
    }

    println!("cargo:rerun-if-changed={}", proto_root.display());

    let mut proto_files = Vec::new();
    collect_proto_files(&proto_root, &mut proto_files).unwrap_or_else(|err| {
        panic!(
            "failed to enumerate proto files under {}: {err}",
            proto_root.display()
        )
    });

    proto_files.sort();

    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");

    config
        .compile_protos(&proto_files, &[proto_root])
        .expect("failed to compile KiCad protobuf schema");
}
