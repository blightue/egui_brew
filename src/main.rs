use brew_egui::app::{app_icon::app_icon, mainapp::MainApp};
use eframe::egui;
use egui::ViewportBuilder;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    std::env::set_var("PATH", "/opt/homebrew/bin:/usr/local");
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder {
            title: Some("Brew_EGUI".to_owned()),
            inner_size: Some(egui::vec2(800.0, 600.0)),
            ..Default::default()
        }
        .with_icon(app_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "Brew_EGUI",
        options,
        Box::new(|_cc| Box::new(MainApp::new())),
    )
}
