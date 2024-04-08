use crate::homebrew::package_model::PackageState;
use crate::homebrew::pkg_cli_handle::PkgManageType;

use super::central_panel::CentralPanel;
use super::left_panel::LeftPanel;
use super::ConsoleOutput;
use eframe::egui;

pub struct MainApp {
    left_panel: LeftPanel,
    central_panel: CentralPanel,
}

impl MainApp {
    pub fn new() -> Self {
        Self {
            left_panel: LeftPanel::new(),
            central_panel: CentralPanel::new(),
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.left_panel.update_package_loader();
        if let Some(output) = self.left_panel.retrieve_output() {
            self.central_panel.push_console(output);
        }

        egui::SidePanel::left("Left_Panel")
            .resizable(true)
            .show(ctx, |ui| self.left_panel.show(ui));
        self.central_panel
            .set_package(self.left_panel.selected_package.clone());
        egui::CentralPanel::default().show(ctx, |ui| self.central_panel.show(ui, ctx));
        if let Some(pkg_handle) = &self.central_panel.successed_manage {
            let new_state = match pkg_handle.cli_type {
                PkgManageType::Install => PackageState::Installed,
                PkgManageType::Uninstall => PackageState::Installable,
                PkgManageType::Upgrade => PackageState::Installed,
            };
            self.left_panel
                .update_package_state(pkg_handle.package.clone(), new_state);
        }
    }
}
