
fn main() {
    println!("cargo:rerun-if-changed=../../idl");
    // flatc::Build::new()
    //     .schema("../../idl/base.fbs")
    //     .schema("../../idl/partner.fbs")
    //     .gen_mutable()
    //     .gen_onefile()
    //     .compile();
}