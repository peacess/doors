use egui::{Direction, Layout, Stroke, Ui};

use crate::{Message, Partner};

#[derive(PartialEq)]
pub enum BarType {
    Message,
    Partner,
}

pub trait BarView {
    fn show_inside(&mut self, ui: &mut Ui);
    fn bar_type(&self) -> BarType;
    fn name(&self) -> String;
}

pub struct Bars {
    bar_list: Vec<Box<dyn BarView>>,
    selected: BarType,
}

impl Bars {
    pub fn new() -> Bars {
        let bar_list: Vec<Box<dyn BarView>> = vec![Box::new(Message {}), Box::new(Partner {})];
        let bars = Bars {
            bar_list: bar_list,
            selected: BarType::Message,
        };
        bars
    }

    pub fn show_inside(&mut self, ui: &mut Ui) {
        let frame = egui::Frame::default().stroke(Stroke::none());
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .frame(frame)
            .show_inside(ui, |ui| {
                ui.columns(self.bar_list.len(), |cols| {
                    for (i, col) in cols.iter_mut().enumerate() {
                        let bar = &self.bar_list[i];
                        let layout = Layout::centered_and_justified(Direction::LeftToRight);
                        col.with_layout(layout, |ui| {
                            ui.selectable_value(&mut self.selected, bar.bar_type(), bar.name())
                        });
                    }
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