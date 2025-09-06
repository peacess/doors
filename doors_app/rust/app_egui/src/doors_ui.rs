use miniquad::{window, EventHandler, PassAction, RenderingBackend};

use crate::Bars;

pub struct DoorsUi {
    pub title: String,
    bars: Bars,
    ctx: Box<dyn RenderingBackend>,
}

impl DoorsUi {
    pub fn new() -> Self {
        let ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();
        // egui_mq.egui_ctx().set_visuals(egui::Visuals::dark());
        DoorsUi {
            title: "doors".to_owned(),
            bars: Bars::new(),
            ctx,
        }
    }
}

impl EventHandler for DoorsUi {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.ctx.clear(Some((1., 1., 1., 1.)), None, None);
        self.ctx.begin_default_pass(PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        self.ctx.end_render_pass();

        self.ctx.end_render_pass();
        self.ctx.commit_frame();
    }
}
