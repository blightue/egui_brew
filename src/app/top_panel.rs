use super::WindowsTab;

pub struct TopPanel {
    pub current_tab: WindowsTab,
}

impl TopPanel {
    pub fn new() -> Self {
        Self {
            current_tab: WindowsTab::Browser,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(
            egui::Layout::left_to_right(egui::Align::Center).with_main_justify(true),
            |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.current_tab, WindowsTab::Browser, "Browser");
                    ui.selectable_value(&mut self.current_tab, WindowsTab::Installed, "Installed");
                    ui.selectable_value(&mut self.current_tab, WindowsTab::Update, "Update");
                });
            },
        );
    }
}
