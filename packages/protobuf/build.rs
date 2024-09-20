use std::io::Result;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=proto/");
    prost_build::Config::new()
        .default_package_filename("pb")
        .compile_protos(&["protos/foo.proto"], &["protos/"])?;

    Ok(())
}
