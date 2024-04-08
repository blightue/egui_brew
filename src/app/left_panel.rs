use std::collections::VecDeque;

use egui::TextStyle;

use crate::homebrew::{
    package_brief_list::PackageBriefList,
    package_filter::PackageFilter,
    package_model::{PackageBrief, PackageState},
    package_name_list::TotalPackageNameListLoader,
};

use super::ConsoleOutput;

pub struct LeftPanel {
    pub packages: Option<PackageBriefList>,
    pub selected_package: Option<PackageBrief>,
    filter: PackageFilter,
    package_list_loader: TotalPackageNameListLoader,
    output_queue: VecDeque<String>,
}

impl ConsoleOutput for LeftPanel {
    fn retrieve_output(&mut self) -> Option<String> {
        self.output_queue.pop_front()
    }
}

impl LeftPanel {
    pub fn new() -> Self {
        let mut instance = Self {
            packages: None,
            selected_package: None,
            filter: PackageFilter::new(),
            package_list_loader: TotalPackageNameListLoader::new(),
            output_queue: VecDeque::new(),
        };
        instance
            .output_queue
            .push_back("Start loading package list...".to_string());
        instance
    }

    pub fn update_package_loader(&mut self) {
        if !self.package_list_loader.is_installable_retrieved {
            if let Some(package_list) = self
                .package_list_loader
                .get_by_state(PackageState::Installable)
            {
                self.output_queue.push_back(format!(
                    "Installable packages loaded: {} Formulae, {} Casks",
                    &package_list.formulae.len(),
                    &package_list.casks.len()
                ));
                self.packages = Some(PackageBriefList::from_all(
                    package_list.formulae,
                    package_list.casks,
                ));
            }
        } else if !self.package_list_loader.is_installed_retrieved {
            if let Some(package_list) = self
                .package_list_loader
                .get_by_state(PackageState::Installed)
            {
                self.output_queue.push_back(format!(
                    "Installed packages loaded: {} Formulae, {} Casks",
                    &package_list.formulae.len(),
                    &package_list.casks.len()
                ));
                if let Some(packagelist) = self.packages.as_mut() {
                    packagelist.push_installed(package_list.formulae, package_list.casks);
                }
            }
        } else if !self.package_list_loader.is_outdated_retrieved {
            if let Some(package_list) = self
                .package_list_loader
                .get_by_state(PackageState::Outdated)
            {
                self.output_queue.push_back(format!(
                    "Outdated packages loaded: {} Formulae, {} Casks",
                    &package_list.formulae.len(),
                    &package_list.casks.len()
                ));
                self.output_queue
                    .push_back("Total packages loaded".to_string());
                if let Some(packagelist) = self.packages.as_mut() {
                    packagelist.push_outdated(package_list.formulae, package_list.casks);
                }
            }
        }
    }

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

        if !self.package_list_loader.is_outdated_retrieved {
            ui.horizontal(|ui| {
                if !self.package_list_loader.is_installed_retrieved {
                    if !self.package_list_loader.is_installable_retrieved {
                        ui.label("Loading installable packages...");
                    } else {
                        ui.label("Loading installed packages...");
                    }
                } else {
                    ui.label("Loading outdated packages...");
                }
                ui.spinner();
            });
        }
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
