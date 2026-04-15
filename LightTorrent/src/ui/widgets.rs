use eframe::egui::{self, Ui, Color32, Sense, Vec2, CornerRadius};
use crate::core::State;
use crate::ui::theme::BLUE;
use std::time::Duration;

pub fn sidebtn(ui: &mut Ui, label: &str, sel: bool) -> bool {
    let w = ui.available_width();
    let (rect, res) = ui.allocate_at_least(Vec2::new(w, 32.0), Sense::click());
    let fill = if sel { Color32::from_rgb(45, 45, 45) }
        else if res.hovered() { Color32::from_rgb(35, 35, 35) }
        else { Color32::TRANSPARENT };
    ui.painter().rect_filled(rect, CornerRadius::same(6), fill);
    ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, label,
        egui::FontId::proportional(14.0), Color32::WHITE);
    res.clicked()
}

pub fn pbar(ui: &mut Ui, prog: f32) {
    let pct = (prog * 100.0) as u32;
    let txt = format!("{}%", pct);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(120.0, 16.0), Sense::hover());
    ui.painter().rect_filled(rect, CornerRadius::same(4), Color32::from_rgb(30, 30, 30));
    if prog > 0.0 {
        let mut fr = rect;
        fr.set_width(rect.width() * prog.clamp(0.0, 1.0));
        ui.painter().rect_filled(fr, CornerRadius::same(4), BLUE);
    }
    ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, txt,
        egui::FontId::proportional(10.0), Color32::WHITE);
}

pub fn fmtspd(bps: f64, state: &State) -> String {
    if *state == State::Pause || *state == State::Seed { return "0 KB/s".into(); }
    if bps >= 1_000_000.0 { format!("{:.1} MB/s", bps / 1_000_000.0) }
    else if bps >= 1_000.0 { format!("{:.1} KB/s", bps / 1_000.0) }
    else { format!("{:.0} B/s", bps) }
}

pub fn fmtsz(bytes: u64) -> String {
    if bytes >= 1_000_000_000 { format!("{:.1} GB", bytes as f64 / 1_000_000_000.0) }
    else if bytes >= 1_000_000 { format!("{:.1} MB", bytes as f64 / 1_000_000.0) }
    else { format!("{:.1} KB", bytes as f64 / 1_000.0) }
}

pub fn fmteta(dur: Duration, state: &State) -> String {
    if *state == State::Seed || *state == State::Pause || dur.as_secs() == 0 {
        return "---".into();
    }
    let s = dur.as_secs();
    if s >= 3600 { format!("{}h {}m", s / 3600, (s % 3600) / 60) }
    else if s >= 60 { format!("{}m {}s", s / 60, s % 60) }
    else { format!("{}s", s) }
}
