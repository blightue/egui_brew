use crate::homebrew::package_model::{PackageBrief, PackageState};

pub struct CentralPanel {
    current_package: Option<PackageBrief>,
}

impl CentralPanel {
    pub fn new() -> Self {
        Self {
            current_package: None,
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

    pub fn set_package(&mut self, package: Option<PackageBrief>) {
        self.current_package = package;
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
}
