use egui::TextStyle;

use super::WindowsTab;

pub struct LeftPanel {
    packages: Vec<String>,
    selected_index: usize,
    search_text: String,
}

impl LeftPanel {
    pub fn new() -> Self {
        Self {
            // packages: (1..=10000).map(|i| i.to_string()).collect(),
            packages: vec![],
            selected_index: 0,
            search_text: "".to_string(),
        }
    }

    pub fn set_packages(&mut self, packages: &Vec<String>) {
        self.packages.clone_from(packages);
    }

    pub fn show(&mut self, ui: &mut egui::Ui, current_tab: &WindowsTab) {
        ui.heading(format!("{} List", current_tab.to_string()));

        ui.separator();

        ui.add(egui::TextEdit::singleline(&mut self.search_text).hint_text("Search🔎"));

        ui.spacing();

        if self.packages.len() == 0 {
            ui.horizontal(|ui| {
                ui.label("Loading...");
                ui.spinner();
            });
        } else {
            let contents: Vec<String> = if self.search_text.len() > 0 {
                ui.label(format!("Searching for {}", self.search_text));
                self.packages
                    .clone()
                    .into_iter()
                    .filter(|x| x.contains(&self.search_text))
                    .collect()
            } else {
                self.packages.clone()
            };

            self.show_listview(ui, contents);
        }
    }

    fn show_listview(&mut self, ui: &mut egui::Ui, contents: Vec<String>) {
        let text_style = TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        let total_rows = contents.len();

        egui::ScrollArea::vertical().auto_shrink(false).show_rows(
            ui,
            row_height,
            total_rows,
            |ui, row_range| {
                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        for i in row_range {
                            let content = &contents[i];
                            let response = ui.selectable_label(self.selected_index == i, content);
                            if response.clicked() {
                                self.selected_index = i;
                            }
                        }
                    },
                );
            },
        );
    }
}
