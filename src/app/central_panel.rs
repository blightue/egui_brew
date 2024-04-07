use egui::{Button, Grid, Label, ScrollArea};

use crate::homebrew::{
    package_info_model::{PackageInfo, PackageInfoHolder, PackageInfoLoader, PackageInfoQuery},
    package_model::{PackageBrief, PackageState},
    pkg_cli_handle::{PkgCliHandle, PkgManageType},
};

use crate::homebrew::brew_cli::BrewCli;

pub struct CentralPanel {
    current_package: Option<PackageBrief>,
    current_info: Option<PackageInfoQuery>,
    current_loader: Option<PackageInfoLoader>,
    current_clihandle: Option<PkgCliHandle>,
    command_output: Vec<String>,
    pub successed_manage: Option<PkgCliHandle>,
}

impl CentralPanel {
    pub fn new() -> Self {
        Self {
            current_package: None,
            current_info: None,
            current_loader: None,
            current_clihandle: None,
            command_output: Vec::new(),
            successed_manage: None,
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

    fn update_clihandle(&mut self) {
        if let Some(clihandle) = self.current_clihandle.take() {
            // let mut line = String::new();
            if let Ok(output) = clihandle.cli_handle.stdout.try_recv() {
                let line = output.trim_end_matches("\n").to_string();
                self.command_output.push(line);
            }
            let status = clihandle.cli_handle.child.lock().unwrap().try_wait();
            if let Ok(Some(status)) = status {
                if status.success() {
                    self.command_output.push(format!(
                        "==== {} {} success! ====",
                        clihandle.cli_type, clihandle.package
                    ));
                    self.update_pkg_state(clihandle.cli_type);
                    self.successed_manage = Some(clihandle); // successed, put it to successed_manage
                } else {
                    let code = status.code().unwrap();
                    self.command_output
                        .push(format!("Command Failed with code: {}", code));
                }
            } else {
                self.current_clihandle = Some(clihandle); // not finished, put it back
            }
        }
    }

    pub fn push_console(&mut self, line: String) {
        self.command_output.push(line);
    }

    fn update_pkg_state(&mut self, manage_type: PkgManageType) {
        if let Some(package) = &mut self.current_package {
            package.package_state = match manage_type {
                PkgManageType::Install => PackageState::Installed,
                PkgManageType::Uninstall => PackageState::Installable,
                PkgManageType::Upgrade => PackageState::Installed,
            };
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, ctx: &eframe::egui::Context) {
        self.check_loader();
        self.update_clihandle();
        if self.current_clihandle.is_some() {
            ctx.request_repaint();
        }
        ui.heading("Package Detail");
        ui.separator();
        if let Some(package) = &self.current_package.clone() {
            self.show_package(ui, package);
        } else {
            ui.horizontal(|ui| {
                ui.label("No package selected");
            });
        }
        ui.separator();
        self.show_console(ui);
    }

    fn show_package(&mut self, ui: &mut egui::Ui, package: &PackageBrief) {
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
        ui.horizontal(|ui| {
            let is_clirunning = self.current_clihandle.is_some();
            let installable = package.package_state == PackageState::Installable;
            let upgradeable = package.package_state == PackageState::Outdated;
            let uninstallable = package.package_state == PackageState::Installed
                || package.package_state == PackageState::Outdated;
            if ui
                .add_enabled(!is_clirunning & installable, Button::new("Install"))
                .clicked()
            {
                if let Some(package) = &self.current_package {
                    self.current_clihandle = Some(
                        BrewCli::manage_package(
                            &package.name,
                            package.package_type,
                            PkgManageType::Install,
                        )
                        .unwrap(),
                    );
                }
            }
            if ui
                .add_enabled(!is_clirunning & uninstallable, Button::new("Uninstall"))
                .clicked()
            {
                if let Some(package) = &self.current_package {
                    self.current_clihandle = Some(
                        BrewCli::manage_package(
                            &package.name,
                            package.package_type,
                            PkgManageType::Uninstall,
                        )
                        .unwrap(),
                    );
                }
            }
            if ui
                .add_enabled(!is_clirunning & upgradeable, Button::new("Upgrade"))
                .clicked()
            {
                if let Some(package) = &self.current_package {
                    self.current_clihandle = Some(
                        BrewCli::manage_package(
                            &package.name,
                            package.package_type,
                            PkgManageType::Upgrade,
                        )
                        .unwrap(),
                    );
                }
            }
            if is_clirunning {
                ui.spinner();
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

    fn show_console(&self, ui: &mut egui::Ui) {
        ui.heading("Console");
        ui.separator();
        ui.vertical(|ui| {
            ScrollArea::vertical()
                .auto_shrink(false)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        for line in &self.command_output {
                            ui.add(Label::new(line.clone()));
                        }
                    });
                });
        });
    }
}
