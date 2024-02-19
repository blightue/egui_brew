use egui::Grid;

use crate::homebrew::{
    package_info_model::{
        FormulaInfo, PackageInfo, PackageInfoHolder, PackageInfoLoader, PackageInfoQuery,
    },
    package_model::{PackageBrief, PackageState},
};

pub struct CentralPanel {
    current_package: Option<PackageBrief>,
    current_info: Option<PackageInfoQuery>,
    current_loader: Option<PackageInfoLoader>,
}

impl CentralPanel {
    pub fn new() -> Self {
        Self {
            current_package: None,
            current_info: None,
            current_loader: None,
        }
    }

    pub fn set_package(&mut self, package: Option<PackageBrief>) {
        if self.current_package != package {
            self.current_package = package;
            self.current_info = None;
            if let Some(package) = &self.current_package {
                let holder = PackageInfoHolder::new(package.clone());
                self.current_loader = Some(PackageInfoLoader::new(holder));
            }
        }
    }

    fn check_loader(&mut self) {
        if let Some(loader) = &mut self.current_loader {
            if let Some(info) = loader.get() {
                self.current_info = Some(info.package_info.unwrap());
                self.current_loader = None;
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.check_loader();
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
        ui.heading("Description");
        if let Some(info) = &self.current_info {
            let info: &dyn PackageInfo = match package.package_type {
                crate::homebrew::package_model::PackageType::Formula => &info.formulae[0],
                crate::homebrew::package_model::PackageType::Cask => &info.casks[0],
            };
            self.show_info_detail(ui, info);
        } else {
            ui.label("Detail Loading...");
            ui.spinner();
        }
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

    fn show_info_detail(&self, ui: &mut egui::Ui, info: &dyn PackageInfo) {
        Grid::new("info_detail")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Name");
                ui.label(info.get_name());
                ui.end_row();

                ui.label("Version");
                ui.label(info.get_version());
                ui.end_row();

                ui.label("Description");
                ui.label(info.get_desc());
                ui.end_row();

                ui.label("Homepage");
                ui.hyperlink(info.get_homepage());
                ui.end_row();
            });
    }
}
