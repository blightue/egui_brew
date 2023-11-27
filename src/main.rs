use eframe::egui;
use brew_egui::app::mainapp::MainApp;
use brew_egui::brew_cli::commands::Commands;
use egui::ViewportBuilder;
use tokio;

fn main() -> Result<(), eframe::Error>{
    // let result = tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(Commands::excute("list")).expect("msg");
    // println!("result: {}", result);

    let options = eframe::NativeOptions{
        viewport: ViewportBuilder{
            title: Some("Brew_eGUI".to_owned()),
            inner_size: Some(egui::vec2(800.0, 600.0)),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native("Brew_eGUI", options, Box::new(|_cc| Box::new(MainApp::new())))
}
