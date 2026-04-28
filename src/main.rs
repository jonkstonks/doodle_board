#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    // println!("Hello, world!");
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Doodle Board")
            .with_min_inner_size(egui::vec2(600.0, 400.0))
            .with_resizable(true)
            .with_decorations(true),
        ..Default::default()
    };
    eframe::run_native(
        "doodle_board", 
        native_options,
        Box::new(|cc| Ok(Box::new(doodle_board::TemplateApp::new(cc)))),
    )
}
