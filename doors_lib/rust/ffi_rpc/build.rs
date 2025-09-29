use std::env;
use std::path::PathBuf;
fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_path = PathBuf::from("include");

    let mut builder = cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C);

    if let Ok(config) = cbindgen::Config::from_file("cbindgen.toml") {
        builder = builder.with_config(config);
    }
    
    builder.generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("ffi_rpc.h"));
}