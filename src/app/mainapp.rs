use eframe::egui;

use super::central_panel::CentralPanel;
use super::left_panel::LeftPanel;
use super::top_panel::TopPanel;

pub struct MainApp {
    top_panel: TopPanel,
    left_panel: LeftPanel,
    central_panel: CentralPanel,
}

impl MainApp {
    pub fn new() -> Self {
        Self {
            top_panel: TopPanel::new(),
            left_panel: LeftPanel::new(),
            central_panel: CentralPanel::new(),
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Top_Panel").show(ctx, |ui| self.top_panel.show(ui));
        egui::SidePanel::left("Left_Panel")
            // .exact_width(200.0)
            .resizable(true)
            .show(ctx, |ui| {
                self.left_panel.show(ui, &self.top_panel.current_tab)
            });
        egui::CentralPanel::default().show(ctx, |ui| self.central_panel.show(ui));
    }
}
