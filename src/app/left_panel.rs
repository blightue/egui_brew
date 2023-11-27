use egui::TextStyle;

use super::WindowsTab;

pub struct LeftPanel {
    packages: Vec<String>,
    selected_index: usize,
}

impl LeftPanel {
    pub fn new() -> Self {
        Self {
            packages: (1..=10000).map(|i| i.to_string()).collect(),
            selected_index: 0,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, current_tab: &WindowsTab) {
        ui.heading(format!("{} List", current_tab.to_string()));

        let text_style = TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        let num_rows = self.packages.len();

        egui::scroll_area::ScrollArea::vertical()
            .auto_shrink(false)
            .show_rows(ui, row_height, num_rows, |ui, row_range| {
                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        for i in row_range {
                            let package =
                                format!("This is row {} of {}", &self.packages[i], num_rows);
                            let response = ui.selectable_label(self.selected_index == i, package);
                            if response.clicked() {
                                self.selected_index = i;
                            }
                        }
                    },
                );
            });
    }
}
