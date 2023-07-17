use egui_miniquad::EguiMq;
use miniquad::{Context, EventHandler, KeyCode, KeyMods, MouseButton, PassAction, TouchPhase};

use crate::Bars;

pub struct DoorsUi {
    pub title: String,
    bars: Bars,

    egui_mq: EguiMq,
}

impl DoorsUi {
    pub fn new(ctx: &mut Context) -> Self {
        let egui_mq = EguiMq::new(ctx);
        egui_mq.egui_ctx().set_visuals(egui::style::Visuals::dark());
        DoorsUi {
            title: "doors".to_owned(),
            bars: Bars::new(),

            egui_mq,
        }
    }
}

impl EventHandler for DoorsUi {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.clear(Some((1., 1., 1., 1.)), None, None);
        ctx.begin_default_pass(PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        ctx.end_render_pass();

        self.egui_mq.run(ctx, |_, egui_ctx| {
            egui::CentralPanel::default().show(egui_ctx, |ui| {
                #[cfg(debug_assertions)]
                ui.ctx().set_debug_on_hover(true);

                self.bars.show_inside(ui)
            });
        });

        // self.egui_mq.end_frame(ctx);

        // Draw things behind egui here

        self.egui_mq.draw(ctx);

        // Draw things in front of egui here

        ctx.commit_frame();
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) {
        EventHandler::resize_event(self, _ctx, _width, _height)
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {
        self.egui_mq.mouse_motion_event(_x, _y)
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {
        self.egui_mq.mouse_wheel_event(_x, _y)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        self.egui_mq.mouse_button_down_event(_ctx, _button, _x, _y)
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        self.egui_mq.mouse_button_up_event(_ctx, _button, _x, _y)
    }

    fn char_event(&mut self, _ctx: &mut Context, _character: char, _keymods: KeyMods, _repeat: bool) {
        self.egui_mq.char_event(_character)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.egui_mq.key_down_event(_ctx, _keycode, _keymods)
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {
        self.egui_mq.key_up_event(_keycode, _keymods)
    }

    fn touch_event(&mut self, ctx: &mut Context, phase: TouchPhase, _id: u64, x: f32, y: f32) {
        EventHandler::touch_event(self, ctx, phase, _id, x, y)
    }

    fn raw_mouse_motion(&mut self, _ctx: &mut Context, _dx: f32, _dy: f32) {
        EventHandler::raw_mouse_motion(self, _ctx, _dx, _dy)
    }

    fn window_minimized_event(&mut self, _ctx: &mut Context) {
        EventHandler::window_minimized_event(self, _ctx)
    }

    fn window_restored_event(&mut self, _ctx: &mut Context) {
        EventHandler::window_restored_event(self, _ctx)
    }

    fn quit_requested_event(&mut self, _ctx: &mut Context) {
        EventHandler::quit_requested_event(self, _ctx)
    }
}
