use std::io::BufRead;

use egui::{Grid, Label, ScrollArea};

use crate::homebrew::{
    cli_handle::CliHandle,
    package_info_model::{PackageInfo, PackageInfoHolder, PackageInfoLoader, PackageInfoQuery},
    package_model::{PackageBrief, PackageState},
};

use crate::homebrew::brew_cli::BrewCli;

pub struct CentralPanel {
    current_package: Option<PackageBrief>,
    current_info: Option<PackageInfoQuery>,
    current_loader: Option<PackageInfoLoader>,
    current_clihandle: Option<CliHandle>,
    command_output: Vec<String>,
}

impl CentralPanel {
    pub fn new() -> Self {
        Self {
            current_package: None,
            current_info: None,
            current_loader: None,
            current_clihandle: None,
            command_output: Vec::new(),
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

    fn update_command_output(&mut self) {
        if let Some(clihandle) = &mut self.current_clihandle {
            let mut line = String::new();
            if let Ok(size) = clihandle.stdout.read_line(&mut line) {
                if size > 0 {
                    let line = line.trim_end_matches("\n").to_string();
                    self.command_output.push(line);
                }
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.check_loader();
        self.update_command_output();
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
        ui.horizontal(|ui| match package.package_state {
            PackageState::Installed => {
                if ui.button("Uninstall").clicked() {
                    if let Some(package) = &self.current_package {
                        self.current_clihandle = Some(
                            BrewCli::uninstall_package(&package.name, package.package_type.clone())
                                .unwrap(),
                        );
                    }
                }
            }
            PackageState::Installable => {
                if ui.button("Install").clicked() {
                    if let Some(package) = &self.current_package {
                        self.current_clihandle = Some(
                            BrewCli::install_package(&package.name, package.package_type.clone())
                                .unwrap(),
                        );
                    }
                }
            }
            PackageState::Outdated => {
                ui.button("Update");
                if ui.button("Uninstall").clicked() {}
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
