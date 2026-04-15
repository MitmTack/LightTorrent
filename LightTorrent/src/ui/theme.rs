use eframe::egui::{Color32, Visuals, Stroke};

pub const BLUE: Color32 = Color32::from_rgb(0, 120, 255);
pub const BG: Color32 = Color32::from_rgb(15, 15, 15);
pub const SURF: Color32 = Color32::from_rgb(25, 25, 25);
pub const TXT: Color32 = Color32::from_rgb(230, 230, 230);
pub const MUTED: Color32 = Color32::from_rgb(120, 120, 120);
pub const SELECTION_GRAY: Color32 = Color32::from_rgb(60, 60, 60);

pub fn visuals() -> Visuals {
    let mut v = Visuals::dark();
    v.widgets.noninteractive.bg_fill = BG;
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TXT);
    v.widgets.inactive.bg_fill = SURF;
    v.selection.bg_fill = SELECTION_GRAY;
    v.extreme_bg_color = Color32::from_rgb(10, 10, 10);
    v.window_fill = BG;
    v.panel_fill = BG;
    v
}