use egui::Grid;

use crate::homebrew::{
    package_info_model::PackageInfo,
    package_model::{PackageBrief, PackageState},
};

pub struct CentralPanel {
    current_package: Option<PackageBrief>,
    current_info: Option<PackageInfo>,
}

impl CentralPanel {
    pub fn new() -> Self {
        Self {
            current_package: None,
            current_info: None,
        }
    }

    pub fn set_package(&mut self, package: Option<PackageBrief>) {
        if self.current_package != package {
            self.current_package = package;
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Package Detail");
        ui.separator();
        if let Some(package) = &self.current_package {
            self.show_package(ui, package);
        } else {
            ui.horizontal(|ui| {
                ui.label("No package selected");
            });
        }
    }

    fn show_package(&self, ui: &mut egui::Ui, package: &PackageBrief) {
        ui.heading(package.name.clone());
        ui.horizontal(|ui| {
            ui.label(format!("Type: {:?}", package.package_type));
            ui.label(format!("State: {:?}", package.package_state));
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Description");
            if let Some(info) = &self.current_info {
                self.show_info_detail(ui, info);
            } else {
                ui.label("Detail Loading...");
                ui.spinner();
            }
        });
        ui.separator();
        ui.horizontal(|ui| match package.package_state {
            PackageState::Installed => {
                ui.button("Uninstall");
            }
            PackageState::Installable => {
                ui.button("Install");
            }
            PackageState::Outdated => {
                ui.button("Update");
                ui.button("Uninstall");
            }
        });
    }

    fn show_info_detail(&self, ui: &mut egui::Ui, info: &PackageInfo) {
        Grid::new("info_detail")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Name");
                ui.label(info.name.clone());
                ui.end_row();

                ui.label("Version");
                ui.label(info.versions.stable.clone());
                ui.end_row();

                ui.label("Description");
                ui.label(info.desc.clone());
                ui.end_row();

                ui.label("Homepage");
                ui.hyperlink(info.homepage.clone());
                ui.end_row();
            });
    }
}
