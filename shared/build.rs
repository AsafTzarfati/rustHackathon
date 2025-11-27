use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &["proto/operSystem_api_realtime.proto"],
        &["proto/"],
    )?;
    Ok(())
}
