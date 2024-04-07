use egui::TextStyle;

use crate::homebrew::{
    package_filter::PackageFilter,
    package_model::{PackageBrief, PackageState},
    packagelist::PackageList,
};

pub struct LeftPanel {
    pub packages: Option<PackageList>,
    pub selected_package: Option<PackageBrief>,
    pub is_pkglist_loaded: bool,
    filter: PackageFilter,
}

impl LeftPanel {
    pub fn new() -> Self {
        Self {
            packages: None,
            selected_package: None,
            is_pkglist_loaded: false,
            filter: PackageFilter::new(),
        }
    }

    // pub fn set_packages(&mut self, packages: PackageList) {
    //     self.packages = Some(packages);
    // }

    pub fn update_package_state(&mut self, package: String, state: PackageState) {
        if let Some(packages) = &mut self.packages {
            if let Some(index) = packages.list.iter().position(|p| p.name == package) {
                packages.list[index].package_state = state;
                if let Some(selected_package) = &self.selected_package {
                    if selected_package.name == package {
                        self.selected_package = Some(packages.list[index].clone());
                    }
                }
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Package List");

        ui.separator();

        ui.heading("Filter");
        self.show_filter(ui);
        ui.add(egui::TextEdit::singleline(&mut self.filter.name_filter).hint_text("Search🔎"));

        ui.separator();

        ui.spacing();

        if let Some(packages) = &self.packages {
            if !self.is_pkglist_loaded {
                ui.horizontal(|ui| {
                    ui.label("Still loading...");
                    ui.spinner();
                });
            }
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
                        let response = ui.selectable_label(
                            self.selected_package == Some(contents[i].clone()),
                            content,
                        );
                        if response.clicked() {
                            self.selected_package = Some(contents[i].clone());
                        }
                    }
                });
            },
        );
    }

    fn show_filter(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Type");
            ui.toggle_value(&mut self.filter.type_filter.is_formula, "Formula");
            ui.toggle_value(&mut self.filter.type_filter.is_cask, "Cask");
        });

        ui.horizontal(|ui| {
            ui.label("State");
            ui.toggle_value(&mut self.filter.state_filter.is_installable, "Installable");
            ui.toggle_value(&mut self.filter.state_filter.is_installed, "Installed");
            ui.toggle_value(&mut self.filter.state_filter.is_outdated, "Outdated");
        });
    }
}
