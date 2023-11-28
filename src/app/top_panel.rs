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
        ui.style_mut().spacing.item_spacing = egui::vec2(10., 5.);
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.style_mut().text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::new(20.0, eframe::epaint::FontFamily::Proportional),
            );
            ui.style_mut().spacing.button_padding = egui::Vec2::new(5.0, 5.0);
            ui.selectable_value(
                &mut self.current_tab,
                WindowsTab::Browser,
                WindowsTab::Browser.to_string(),
            );
            ui.selectable_value(
                &mut self.current_tab,
                WindowsTab::Installed,
                WindowsTab::Installed.to_string(),
            );
            ui.selectable_value(
                &mut self.current_tab,
                WindowsTab::Update,
                WindowsTab::Update.to_string(),
            );
        });
    }
}
