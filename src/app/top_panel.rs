
#[derive(Debug, PartialEq)]
pub enum WindowsTab {
    Browser,
    Installed,
    Update,
}

pub struct TopPanel {
    current_tab: WindowsTab,
}

impl TopPanel {
    pub fn new() -> Self {
        Self {
            current_tab: WindowsTab::Browser,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            ui.selectable_value(&mut self.current_tab, WindowsTab::Browser, "Browser");
            ui.selectable_value(&mut self.current_tab, WindowsTab::Installed, "Installed");
            ui.selectable_value(&mut self.current_tab, WindowsTab::Update, "Update");
        });
    }
}
