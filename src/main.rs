use eframe::egui;
use brew_egui::app::mainapp::MainApp;
use brew_egui::brew_cli::commands::Commands;
use tokio;

fn main() -> Result<(), eframe::Error>{
    let result = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(Commands::excute("list")).expect("msg");
    println!("result: {}", result);

    let options = eframe::NativeOptions{
        initial_window_size: Some(egui::vec2(400.0, 400.0)),
        ..Default::default()
    };

    eframe::run_native("Brew_EGUI", options, Box::new(|_cc| Box::new(MainApp::new())))
}
