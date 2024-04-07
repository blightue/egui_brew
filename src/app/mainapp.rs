use crate::homebrew::package_model::PackageState;
use crate::homebrew::package_name_list::{PackageAllNameListLoader, PackageNameListLoader};
use crate::homebrew::packagelist::PackageList;
use crate::homebrew::pkg_cli_handle::PkgManageType;

use super::super::homebrew::packagelist::PackageListLoader;
use super::central_panel::CentralPanel;
use super::left_panel::LeftPanel;
use eframe::egui;

pub struct MainApp {
    left_panel: LeftPanel,
    central_panel: CentralPanel,
    package_list_loader: PackageAllNameListLoader,
    // is_loaded: bool,
}

impl MainApp {
    pub fn new() -> Self {
        Self {
            left_panel: LeftPanel::new(),
            central_panel: CentralPanel::new(),
            package_list_loader: PackageAllNameListLoader::new(),
            // package_list_loader: PackageListLoader::new(PackageList::new()),
            // is_loaded: false,
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // egui::TopBottomPanel::top("Top_Panel").show(ctx, |ui| self.top_panel.show(ui));
        if !self.package_list_loader.is_installable_retrieved {
            if let Some(package_list) = self
                .package_list_loader
                .get_by_state(PackageState::Installable)
            {
                self.central_panel.push_console(format!(
                    "Installable: {} Formulae, {} Casks",
                    &package_list.formulae.len(),
                    &package_list.casks.len()
                ));
                self.left_panel.packages = Some(PackageList::from_all(
                    package_list.formulae,
                    package_list.casks,
                ));
            }
        } else if !self.package_list_loader.is_installed_retrieved {
            if let Some(package_list) = self
                .package_list_loader
                .get_by_state(PackageState::Installed)
            {
                self.central_panel.push_console(format!(
                    "Installed: {} Formulae, {} Casks",
                    &package_list.formulae.len(),
                    &package_list.casks.len()
                ));
                if let Some(packagelist) = self.left_panel.packages.as_mut() {
                    packagelist.push_installed(package_list.formulae, package_list.casks);
                }
            }
        } else if !self.package_list_loader.is_outdated_retrieved {
            if let Some(package_list) = self
                .package_list_loader
                .get_by_state(PackageState::Outdated)
            {
                self.central_panel.push_console(format!(
                    "Outdated: {} Formulae, {} Casks",
                    &package_list.formulae.len(),
                    &package_list.casks.len()
                ));
                if let Some(packagelist) = self.left_panel.packages.as_mut() {
                    packagelist.push_outdated(package_list.formulae, package_list.casks);
                }
                self.left_panel.is_pkglist_loaded = true;
            }
        }
        egui::SidePanel::left("Left_Panel")
            // .exact_width(200.0)
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
