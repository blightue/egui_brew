use brew_egui::app::mainapp::MainApp;
use eframe::egui;
use egui::ViewportBuilder;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder {
            title: Some("Brew_EGUI".to_owned()),
            inner_size: Some(egui::vec2(800.0, 600.0)),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "Brew_EGUI",
        options,
        Box::new(|_cc| Box::new(MainApp::new())),
    )
}
