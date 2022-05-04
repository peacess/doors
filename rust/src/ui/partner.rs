use egui::Ui;

use crate::ui::{BarType, BarView};

pub struct Partner {}

impl Partner {}

impl BarView for Partner {
    fn show_inside(&mut self, ui: &mut Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {});
    }

    fn bar_type(&self) -> BarType {
        BarType::Partner
    }
}