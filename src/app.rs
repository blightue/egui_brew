pub mod mainapp;

mod central_panel;
mod left_panel;
mod top_panel;

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum WindowsTab {
    Browser,
    Installed,
    Update,
}

impl fmt::Display for WindowsTab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WindowsTab::Browser => write!(f, "Browser🌐"),
            WindowsTab::Installed => write!(f, "Installed💾"),
            WindowsTab::Update => write!(f, "Update🔁"),
        }
    }
}
