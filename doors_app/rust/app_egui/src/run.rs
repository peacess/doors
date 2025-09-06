use crate::DoorsUi;

pub fn run() {
    let conf = miniquad::conf::Conf {
        high_dpi: true,
        ..Default::default()
    };
    miniquad::start(conf, move || Box::new(DoorsUi::new()));
}
