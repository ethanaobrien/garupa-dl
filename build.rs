
fn main() -> std::io::Result<()> {
    println!("cargo::rerun-if-changed=proto");
    prost_build::compile_protos(
        &["proto/AppGetResponse.proto", "proto/AssetBundleInfo.proto"],
        &["proto/"]
    )?;
    Ok(())
}
