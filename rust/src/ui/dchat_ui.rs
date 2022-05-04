use std::time::Duration;

use egui::{CtxRef, Rgba, Vec2};
use epi::{App, Frame, Storage};

use crate::ui::Bars;

pub struct DchatUi {
    pub(crate) title: String,
    bars: Bars,
}

impl DchatUi {
    pub fn new() -> Self {
        DchatUi {
            title: "DChat".to_owned(),
            bars: Bars::new(),
        }
    }
}

impl App for DchatUi {
    fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.bars.show_inside(ui)
        });
    }

    // fn setup(&mut self, _ctx: &CtxRef, _frame: &Frame, _storage: Option<&dyn Storage>) {
    //     App::setup(self, _ctx, _frame, _storage)
    // }
    //
    // fn warm_up_enabled(&self) -> bool {
    //     App::warm_up_enabled(self)
    // }
    //
    // fn save(&mut self, _storage: &mut dyn Storage) {
    //     App::save(self, _storage)
    // }
    //
    // fn on_exit(&mut self) {
    //     App::on_exit(self)
    // }

    fn name(&self) -> &str {
        "Dchat"
    }

    // fn auto_save_interval(&self) -> Duration {
    //     App::auto_save_interval(self)
    // }
    //
    // fn max_size_points(&self) -> Vec2 {
    //     App::max_size_points(self)
    // }
    //
    // fn clear_color(&self) -> Rgba {
    //     App::clear_color(self)
    // }
    //
    // fn persist_native_window(&self) -> bool {
    //     App::persist_native_window(self)
    // }
    //
    // fn persist_egui_memory(&self) -> bool {
    //     App::persist_egui_memory(self)
    // }
}