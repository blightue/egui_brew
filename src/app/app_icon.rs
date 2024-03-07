use egui::IconData;

pub fn app_icon() -> IconData {
    let bytes = include_bytes!("../../assets/icons/beer-mug_1f37a.png");
    let image = image::load_from_memory(bytes).unwrap();
    let rgba_image = image.to_rgba8();
    let (width, height) = rgba_image.dimensions();
    let rgba = rgba_image.into_raw();
    egui::viewport::IconData {
        rgba,
        width: width,
        height: height,
    }
}
