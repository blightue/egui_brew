
pub struct LeftPanel {
    packages: Vec<String>,
}

impl LeftPanel {

    pub fn new() -> Self{
        Self{
            packages: vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
        }
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui){
        ui.heading("All packages");
    }
}