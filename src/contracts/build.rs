use std::{io, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    compile_protos("./proto/payment_manager.proto")?;
    Ok(())
}

fn compile_protos(proto: impl AsRef<Path>) -> io::Result<()> {
    let proto_path: &Path = proto.as_ref();

    // directory the main .proto field resides in
    let proto_dir = proto_path
        .parent()
        .expect("proto file should reside in a directory");

    tonic_build::configure()
        .out_dir("./src/protos")
        .compile(&[proto_path], &[proto_dir])?;

    Ok(())
}
