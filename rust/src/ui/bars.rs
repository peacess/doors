use egui::{CtxRef, Ui, Widget};
use egui::Key::S;
use egui::menu::bar;
use epi::{App, Frame};

use crate::ui::{Message, Partner};

#[derive(PartialEq)]
pub enum BarType {
    Message,
    Partner,
}

pub trait BarView {
    fn show_inside(&mut self, ui: &mut Ui);
    fn bar_type(&self) -> BarType;
}

pub struct Bars {
    bar_list: Vec<Box<dyn BarView>>,
    selected: BarType,
}

impl Bars {
    pub fn new() -> Bars {
        let bar_list: Vec<Box<dyn BarView>> = vec![Box::new(Message {}), Box::new(Partner {})];
        let mut bars = Bars {
            bar_list: bar_list,
            selected: BarType::Message,
        };
        bars
    }

    pub fn show_inside(&mut self, ui: &mut Ui) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Bottom Panel");
                });
            });
        for it in &mut self.bar_list {
            if it.bar_type() == self.selected {
                it.show_inside(ui);
                break;
            }
        }
    }
}