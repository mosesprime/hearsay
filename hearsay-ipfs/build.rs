fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(&["src/unixfs/pb/unixfs.proto"], &["src/"])?;
    Ok(())
}
