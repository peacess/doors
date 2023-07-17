use crate::DoorsUi;

pub fn run() {
    let conf = miniquad::conf::Conf {
        high_dpi: true,
        ..Default::default()
    };
    miniquad::start(conf, |mut ctx| {
        Box::new(DoorsUi::new(&mut ctx))
    });
}


