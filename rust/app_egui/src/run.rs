use crate::DchatUi;

pub fn run() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        // renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "DChat",
        options,
        Box::new(|cc| {
            Box::new(DchatUi::new(cc))
        }),
    );
}


