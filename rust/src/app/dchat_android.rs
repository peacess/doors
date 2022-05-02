//level = "error|warn|info|debug|trace"

#[cfg_attr(target_os = "android", ndk_glue::main(logger(level = "info")))]
fn main() {
    dchat::ui::run();
}