use crate::homebrew::package_model::PackageState;
use crate::homebrew::packagelist::PackageList;
use crate::homebrew::pkg_cli_handle::PkgManageType;

use super::super::homebrew::packagelist::PackageListLoader;
use super::central_panel::CentralPanel;
use super::left_panel::LeftPanel;
use super::top_panel::TopPanel;
use eframe::egui;

pub struct MainApp {
    top_panel: TopPanel,
    left_panel: LeftPanel,
    central_panel: CentralPanel,
    package_list_loader: PackageListLoader,
    is_loaded: bool,
}

impl MainApp {
    pub fn new() -> Self {
        Self {
            top_panel: TopPanel::new(),
            left_panel: LeftPanel::new(),
            central_panel: CentralPanel::new(),
            package_list_loader: PackageListLoader::new(PackageList::new()),
            is_loaded: false,
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // egui::TopBottomPanel::top("Top_Panel").show(ctx, |ui| self.top_panel.show(ui));
        if !self.is_loaded {
            if let Some(package_list) = self.package_list_loader.get() {
                self.left_panel.set_packages(package_list);
                self.is_loaded = true;
            }
        }
        egui::SidePanel::left("Left_Panel")
            // .exact_width(200.0)
            .resizable(true)
            .show(ctx, |ui| self.left_panel.show(ui));
        self.central_panel
            .set_package(self.left_panel.selected_package.clone());
        egui::CentralPanel::default().show(ctx, |ui| self.central_panel.show(ui));
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
