#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    // Log to android output
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info).with_tag("app_egui"));

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };
    eframe::run_native("Doors", options, Box::new(|cc| Ok(Box::new(DoorsUi::new(cc))))).unwrap()
}
fn main() {
    doors::run();
}
