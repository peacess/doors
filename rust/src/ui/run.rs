use crate::ui::{DchatUi, egui_entry};

pub fn run() {
    let app = DchatUi::new();
    egui_entry(app);
}


