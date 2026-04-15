#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod core;
mod ui;
use eframe::NativeOptions;

#[tokio::main]
async fn main() -> eframe::Result<()> {
    let opts = NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([960.0, 620.0])
            .with_decorations(true)
            .with_transparent(false),
        ..Default::default()
    };
    eframe::run_native(
        "LightTorrent",
        opts,
        Box::new(|cc| Ok(Box::new(ui::app::App::new(cc, std::env::args().nth(1)))))
    )
}
