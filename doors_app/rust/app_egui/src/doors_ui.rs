pub struct DoorsUi {
    pub title: String,
}

impl DoorsUi {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self { title: "Doors".to_owned() }
    }
}

impl eframe::App for DoorsUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Doors App");
        });
    }
}
