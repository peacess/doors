use egui::{CtxRef, Ui};
use epi::{App, Frame};

use crate::ui::{BarType, BarView};

pub struct Message {}

impl Message {}

impl BarView for Message {
    fn show_inside(&mut self, ui: &mut Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {});
    }

    fn bar_type(&self) -> BarType {
        BarType::Message
    }
}