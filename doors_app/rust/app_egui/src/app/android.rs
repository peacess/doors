//level = "error|warn|info|debug|trace"

// #[cfg_attr(target_os = "android", ndk_glue::main(logger(level = "info")))]
#[cfg_attr(target_os = "android", ndk_glue::main(ndk_glue = "::miniquad::sapp_android"))]
fn main() {
    doors::run();
}
