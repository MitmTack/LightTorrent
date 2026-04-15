use eframe::egui::{self, CornerRadius};
use crate::core::{Eng, State, Cfg, ldcfg};
use crate::ui::lang::{is_ru, RU, EN, Tr, Lang};
use crate::ui::widgets::*;
use crate::ui::theme::MUTED;
use std::sync::{Arc, Mutex};

pub struct App {
    eng:     Arc<Mutex<Option<Eng>>>,
    cat:     String,
    mag:     String,
    shwset:  bool,
    shwabt:  bool,
    tr:      &'static Tr,
    curlng:  Lang,
    draft:   Cfg,
    settab:  Stab,
}

#[derive(PartialEq)]
enum Stab { Gen, Net }

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, init: Option<String>) -> Self {
        let cfg = ldcfg();
        let initlng = if is_ru() { Lang::RU } else { Lang::EN };
        let tr = if initlng == Lang::RU { &RU } else { &EN };

        let mut vis = egui::Visuals::dark();
        vis.window_corner_radius = CornerRadius::same(12);
        cc.egui_ctx.set_visuals(vis);

        let cont: Arc<Mutex<Option<Eng>>> = Arc::new(Mutex::new(None));
        let cl   = cont.clone();
        let cfg2 = cfg.clone();
        tokio::spawn(async move {
            let eng = Eng::new(cfg2).await;
            if let Some(m) = init { let _ = eng.addmag(m).await; }
            if let Ok(mut g) = cl.lock() { *g = Some(eng); }
        });

        Self {
            eng:    cont,
            cat:    tr.all.to_string(),
            mag:    String::new(),
            shwset: false,
            shwabt: false,
            tr,
            curlng: initlng,
            draft:  cfg,
            settab: Stab::Gen,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            for f in &i.raw.dropped_files {
                if let Some(path) = &f.path {
                    if path.extension().and_then(|e| e.to_str()) == Some("torrent") {
                        if let Ok(g) = self.eng.lock() {
                            if let Some(eng) = g.as_ref() {
                                let e = eng.clone();
                                let p = path.clone();
                                tokio::spawn(async move { let _ = e.addfil(p).await; });
                            }
                        }
                    }
                }
            }
        });

        let eng_opt = self.eng.lock().unwrap().clone();
        if let Some(eng) = eng_opt {
            egui::SidePanel::left("s")
                .frame(egui::Frame::NONE.fill(egui::Color32::from_rgb(20, 20, 20)).inner_margin(12.0))
                .show(ctx, |ui| {
                    ui.heading("LightTorrent");
                    ui.add_space(20.0);

                    if sidebtn(ui, self.tr.all,  self.cat == self.tr.all)  { self.cat = self.tr.all.into(); }
                    if sidebtn(ui, self.tr.down, self.cat == self.tr.down) { self.cat = self.tr.down.into(); }
                    if sidebtn(ui, self.tr.seed, self.cat == self.tr.seed) { self.cat = self.tr.seed.into(); }

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);
                    ui.label(self.tr.mag);
                    let ed = ui.add(egui::TextEdit::singleline(&mut self.mag).hint_text("magnet:?"));
                    if ed.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                        let e = eng.clone(); let m = self.mag.clone();
                        tokio::spawn(async move { let _ = e.addmag(m).await; });
                        self.mag.clear();
                    }

                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                        if ui.button(self.tr.addt).clicked() {
                            if let Some(p) = rfd::FileDialog::new().add_filter("torrent", &["torrent"]).pick_file() {
                                let e = eng.clone();
                                tokio::spawn(async move { let _ = e.addfil(p).await; });
                            }
                        }
                        if ui.button(self.tr.setts).clicked() {
                            self.draft  = eng.cfg.lock().unwrap().clone();
                            self.shwset = true;
                        }
                        if ui.button(self.tr.aboutb).clicked() { self.shwabt = true; }
                    });
                });

            egui::CentralPanel::default().show(ctx, |ui| {
                ctx.input(|i| {
                    if !i.raw.hovered_files.is_empty() {
                        ui.painter().rect_filled(
                            ui.max_rect(),
                            CornerRadius::same(0),
                            egui::Color32::from_rgba_unmultiplied(0, 120, 255, 30),
                        );
                    }
                });

                egui::ScrollArea::vertical().show(ui, |ui| {
                    let list = eng.list.lock().unwrap();
                    egui::Grid::new("g").num_columns(7).spacing([12.0, 8.0]).show(ui, |ui| {
                        for h in [self.tr.name, self.tr.size, self.tr.prog,
                                  self.tr.dl, self.tr.ul, self.tr.peers, self.tr.eta] {
                            ui.label(egui::RichText::new(h).color(egui::Color32::GRAY));
                        }
                        ui.end_row();
                        for t in list.iter() {
                            let skip = match self.cat.as_str() {
                                c if c == self.tr.down => t.state != State::Down,
                                c if c == self.tr.seed => t.state != State::Seed,
                                _ => false,
                            };
                            if skip { continue; }
                            let r = ui.label(&t.name);
                            r.context_menu(|ui| {
                                if ui.button(self.tr.open).clicked() { let _ = opener::open(&t.path); ui.close_menu(); }
                                if ui.button(self.tr.del).clicked() {
                                    let e = eng.clone(); let id = t.id;
                                    tokio::spawn(async move { e.rmtrr(id).await; });
                                    ui.close_menu();
                                }
                            });
                            ui.label(fmtsz(t.size));
                            pbar(ui, t.prog);
                            ui.label(fmtspd(t.dl, &t.state));
                            ui.label(fmtspd(t.ul, &t.state));
                            ui.label(if t.state == State::Seed { "---".to_string() } else { t.peers.to_string() });
                            ui.label(fmteta(t.eta, &t.state));
                            ui.end_row();
                        }
                    });
                });
            });

            if self.shwset {
                let mut is_open = self.shwset;
                egui::Window::new(self.tr.setts)
                    .open(&mut is_open)
                    .min_width(400.0)
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.settab, Stab::Gen, self.tr.tabgen);
                            ui.selectable_value(&mut self.settab, Stab::Net, self.tr.tabnet);
                        });
                        ui.separator();
                        ui.add_space(6.0);

                        match self.settab {
                            Stab::Gen => {
                                ui.label(self.tr.langset);
                                ui.horizontal(|ui| {
                                    if ui.selectable_label(self.curlng == Lang::RU, "Русский").clicked() {
                                        self.curlng = Lang::RU; self.tr = &RU;
                                        self.cat = self.tr.all.to_string();
                                    }
                                    if ui.selectable_label(self.curlng == Lang::EN, "English").clicked() {
                                        self.curlng = Lang::EN; self.tr = &EN;
                                        self.cat = self.tr.all.to_string();
                                    }
                                });

                                ui.add_space(8.0);
                                ui.label(self.tr.dldirlbl);
                                ui.horizontal(|ui| {
                                    ui.text_edit_singleline(&mut self.draft.dldir);
                                    if ui.small_button("…").clicked() {
                                        if let Some(p) = rfd::FileDialog::new().pick_folder() {
                                            self.draft.dldir = p.to_string_lossy().into_owned();
                                        }
                                    }
                                });

                                ui.add_space(8.0);
                                ui.checkbox(&mut self.draft.autorm, self.tr.autorm);
                                if self.draft.autorm {
                                    ui.horizontal(|ui| {
                                        ui.label(self.tr.setrto);
                                        ui.add(egui::DragValue::new(&mut self.draft.setrto)
                                            .range(0.1..=168.0).speed(0.1).suffix("h"));
                                    });
                                }
                            }

                            Stab::Net => {
                                ui.label(self.tr.plimset);
                                ui.add(egui::Slider::new(&mut self.draft.plim, 1..=500));

                                ui.add_space(8.0);
                                ui.label(self.tr.maxdllbl);
                                ui.horizontal(|ui| {
                                    ui.add(egui::DragValue::new(&mut self.draft.maxdl)
                                        .range(0.0..=1_000_000.0).speed(100.0));
                                    ui.label(if self.draft.maxdl == 0.0 {
                                        egui::RichText::new(self.tr.nolimt).color(MUTED)
                                    } else {
                                        egui::RichText::new("KB/s")
                                    });
                                });

                                ui.add_space(8.0);
                                ui.label(self.tr.maxullbl);
                                ui.horizontal(|ui| {
                                    ui.add(egui::DragValue::new(&mut self.draft.maxul)
                                        .range(0.0..=1_000_000.0).speed(100.0));
                                    ui.label(if self.draft.maxul == 0.0 {
                                        egui::RichText::new(self.tr.nolimt).color(MUTED)
                                    } else {
                                        egui::RichText::new("KB/s")
                                    });
                                });
                            }
                        }

                        ui.add_space(10.0);
                        ui.separator();
                        ui.horizontal(|ui| {
                            if ui.button(self.tr.savebtn).clicked() {
                                eng.apcfg(self.draft.clone());
                                self.shwset = false;
                            }
                            if ui.button("✕").clicked() { self.shwset = false; }
                        });
                    });
                if !is_open { self.shwset = false; }
            }

            if self.shwabt {
                let mut is_open = self.shwabt;
                let mut close   = false;
                egui::Window::new(self.tr.about)
                    .open(&mut is_open)
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("LightTorrent");
                            ui.label(egui::RichText::new("v1.0.4").color(MUTED));
                            ui.add_space(4.0);
                            ui.label(egui::RichText::new(self.tr.aboutdesc).color(MUTED).small());
                            ui.add_space(12.0);
                            ui.separator();
                            ui.add_space(8.0);
                            ui.label(egui::RichText::new(self.tr.authlbl).color(MUTED));
                            ui.label(egui::RichText::new("MitmTack").strong());
                            ui.add_space(6.0);
                            ui.hyperlink_to("GitHub: github.com/MitmTack", "https://github.com/MitmTack");
                            ui.hyperlink_to("Telegram: t.me/MitmTack",     "https://t.me/MitmTack");
                            ui.add_space(12.0);
                            if ui.button("Close").clicked() { close = true; }
                        });
                    });
                if close || !is_open { self.shwabt = false; }
            }
        }
        ctx.request_repaint();
    }
}
