use eframe::egui;
use brew_egui::app::mainapp::MainApp;

fn main() -> Result<(), eframe::Error>{

    let options = eframe::NativeOptions{
        initial_window_size: Some(egui::vec2(400.0, 400.0)),
        ..Default::default()
    };

    eframe::run_native("Brew_EGUI", options, Box::new(|_cc| Box::new(MainApp::new())))
}
