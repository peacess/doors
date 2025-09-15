use crate::DoorsUi;

pub fn run() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    let re = eframe::run_native("Doors", options, Box::new(|cc| Ok(Box::new(DoorsUi::new(cc)))));
    if let Err(e) = re {
        log::error!("Error running eframe: {}", e);
    }
}
