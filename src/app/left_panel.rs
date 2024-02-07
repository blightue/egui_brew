use egui::TextStyle;

use crate::homebrew::{
    package_model::{PackageBrief, PackageType},
    packagelist::PackageList,
};

use super::WindowsTab;

pub struct LeftPanel {
    packages: Option<PackageList>,
    selected_index: usize,
    search_text: String,
    package_type: PackageType,
}

impl LeftPanel {
    pub fn new() -> Self {
        Self {
            packages: None,
            selected_index: 0,
            search_text: "".to_string(),
            package_type: PackageType::Formula,
        }
    }

    pub fn set_packages(&mut self, packages: PackageList) {
        self.packages = Some(packages);
    }

    pub fn show(&mut self, ui: &mut egui::Ui, current_tab: &WindowsTab) {
        ui.heading(format!("{} List", current_tab.to_string()));

        ui.separator();

        ui.add(egui::TextEdit::singleline(&mut self.search_text).hint_text("Search🔎"));

        ui.spacing();

        if let Some(packages) = &self.packages {
            let contents: Vec<PackageBrief> = if self.search_text.len() > 0 {
                ui.label(format!("Searching for {}", self.search_text));
                packages
                    .list
                    .clone()
                    .into_iter()
                    .filter(|x| x.name.contains(&self.search_text))
                    .collect()
            } else {
                packages.list.clone()
            };

            self.show_listview(ui, contents);
        } else {
            ui.horizontal(|ui| {
                ui.label("Loading...");
                ui.spinner();
            });
        }
    }

    fn show_listview(&mut self, ui: &mut egui::Ui, contents: Vec<PackageBrief>) {
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
                            let content = &contents[i].name;
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
