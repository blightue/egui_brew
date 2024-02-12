use egui::{Layout, TextStyle};

use crate::homebrew::{
    package_filter::PackageFilter,
    package_model::{PackageBrief, PackageType},
    packagelist::PackageList,
};

use super::WindowsTab;

pub struct LeftPanel {
    packages: Option<PackageList>,
    selected_index: usize,
    filter: PackageFilter,
}

impl LeftPanel {
    pub fn new() -> Self {
        Self {
            packages: None,
            selected_index: 0,
            filter: PackageFilter::new(),
        }
    }

    pub fn set_packages(&mut self, packages: PackageList) {
        self.packages = Some(packages);
    }

    pub fn show(&mut self, ui: &mut egui::Ui, current_tab: &WindowsTab) {
        ui.heading(format!("{} List", current_tab.to_string()));

        ui.separator();

        ui.heading("Filter");

        ui.separator();

        ui.add(egui::TextEdit::singleline(&mut self.filter.name_filter).hint_text("Search🔎"));

        ui.spacing();

        if let Some(packages) = &self.packages {
            if !self.filter.name_filter.is_empty() {
                ui.label(format!("Searching for {}", self.filter.name_filter));
            }
            let contents: Vec<PackageBrief> = packages
                .list
                .clone()
                .into_iter()
                .filter(|x| self.filter.is_filtered(x))
                .collect();

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
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::TOP), |ui| {
                    for i in row_range {
                        let content = format!("{}", contents[i]);
                        let response = ui.selectable_label(self.selected_index == i, content);
                        if response.clicked() {
                            self.selected_index = i;
                        }
                    }
                });
            },
        );
    }
}
