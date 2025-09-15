use egui::Ui;

use crate::{BarType, BarView};

pub struct Partner {}

impl Partner {}

impl BarView for Partner {
    fn show_inside(&mut self, ui: &mut Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label("coming soon...");
        });
    }

    fn bar_type(&self) -> BarType {
        BarType::Partner
    }

    fn name(&self) -> String {
        "🌐".to_owned()
        // "Partner".to_owned()
    }
}
