use eframe::{
    egui::{Vec2, ViewportBuilder}
};
mod contacts;
mod ui;

use ui::App;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size(Vec2::new(350.0, 525.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Agenda",
        options,
        Box::new(|cc| {
            let app = App::new(cc);
            Box::new(app)
        }),
    )
}
