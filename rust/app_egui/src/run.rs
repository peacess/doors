use crate::DchatUi;

pub fn run() {
    let conf = miniquad::conf::Conf {
        high_dpi: true,
        ..Default::default()
    };
    miniquad::start(conf, |mut ctx| {
        miniquad::UserData::owning(DchatUi::new(&mut ctx), ctx)
    });
}


