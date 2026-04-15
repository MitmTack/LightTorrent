use crate::core::torrent::{Torr, State};
use std::sync::{Arc, Mutex};
use librqbit::Session;
use librqbit::api::{TorrentIdOrHash, Api};
use std::path::PathBuf;
use std::fs;
use std::collections::HashSet;
use notify_rust::Notification;
use std::cell::RefCell;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Cfg {
    pub plim:  usize,
    pub dldir: String,
    pub maxdl: f64,
    pub maxul:  f64,
    pub autorm: bool,
    pub setrto: f64,
}

impl Default for Cfg {
    fn default() -> Self {
        Self { plim: 50, dldir: String::new(), maxdl: 0.0, maxul: 0.0, autorm: false, setrto: 2.0 }
    }
}

pub fn exedir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

pub fn cfgpath() -> PathBuf { exedir().join("Settings.cfg") }
pub fn plugdir() -> PathBuf { exedir().join("plugins") }

pub fn ldcfg() -> Cfg {
    fs::read_to_string(cfgpath())
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn svcfg(cfg: &Cfg) {
    if let Ok(s) = toml::to_string(cfg) { let _ = fs::write(cfgpath(), s); }
}

#[derive(Clone)]
pub struct Eng {
    pub sess:  Arc<Session>,
    pub list:  Arc<Mutex<Vec<Torr>>>,
    pub finsh: Arc<Mutex<HashSet<usize>>>,
    pub dldir: PathBuf,
    pub plim:  Arc<Mutex<usize>>,
    pub cfg:   Arc<Mutex<Cfg>>,
}

impl Eng {
    pub async fn new(cfg: Cfg) -> Self {
        let dldir = if cfg.dldir.is_empty() {
            dirs::download_dir().unwrap_or_else(|| PathBuf::from("./downloads"))
        } else {
            PathBuf::from(&cfg.dldir)
        };
        let _ = fs::create_dir_all(&dldir);

        let sess  = Session::new(dldir.clone()).await.expect("session fail");
        let list  = Arc::new(Mutex::new(Vec::new()));
        let finsh = Arc::new(Mutex::new(HashSet::new()));
        let plim  = Arc::new(Mutex::new(cfg.plim));

        let eng = Self { sess, list, finsh, dldir, plim, cfg: Arc::new(Mutex::new(cfg)) };
        eng.sync();
        eng
    }

    fn sync(&self) {
        let sess  = self.sess.clone();
        let list  = self.list.clone();
        let finsh = self.finsh.clone();
        let bdir  = self.dldir.clone();
        let plim  = self.plim.clone();
        let cfg   = self.cfg.clone();

        tokio::spawn(async move {
            let api = Api::new(sess.clone(), None);
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                let cur_plim = *plim.lock().unwrap();
                let autorm   = cfg.lock().unwrap().autorm;

                let upd    = RefCell::new(Vec::new());
                let topaus = RefCell::new(Vec::new());
                let toresm = RefCell::new(Vec::new());
                let torm   = RefCell::new(Vec::new());

                sess.with_torrents(|iter| {
                    for (id, h) in iter {
                        let stats = h.stats();
                        let mg    = h.metadata.load();
                        let name  = mg.as_ref()
                            .and_then(|md| md.info.name.as_ref())
                            .map(|b| String::from_utf8_lossy(b).into_owned())
                            .unwrap_or_else(|| if mg.is_some() { "Unknown".into() } else { "Loading...".into() });

                        let path  = bdir.join(&name);
                        let total = stats.total_bytes;
                        let prog  = if total > 0 {
                            (stats.progress_bytes as f32 / total as f32).clamp(0.0, 1.0)
                        } else { 0.0 };

                        let (dl, ul, peers) = stats.live.as_ref().map(|live| {
                            let d = live.download_speed.mbps * 125000.0;
                            let u = live.upload_speed.mbps * 125000.0;
                            let p = &live.snapshot.peer_stats;
                            (d, u, (p.live + p.queued + p.connecting) as u32)
                        }).unwrap_or((0.0, 0.0, 0));

                        if peers as usize > cur_plim && !h.is_paused() {
                            topaus.borrow_mut().push(id);
                        } else if h.is_paused() && (peers as usize) < cur_plim {
                            toresm.borrow_mut().push(id);
                        }

                        let state = if stats.finished { State::Seed }
                            else if h.is_paused() { State::Pause }
                            else { State::Down };

                        let eta = if dl > 0.0 && total > 0 {
                            let left = total.saturating_sub(stats.progress_bytes) as f64;
                            std::time::Duration::from_secs_f64(left / dl)
                        } else { std::time::Duration::ZERO };

                        let dsprog = if stats.finished { 0.99 } else { prog };
                        let dsdl   = if stats.finished { 0.0 } else { dl };
                        let dsul   = if stats.finished { 0.0 } else { ul };

                        if stats.finished {
                            let mut fl = finsh.lock().unwrap();
                            if !fl.contains(&id) {
                                fl.insert(id);
                                let n = name.clone();
                                tokio::spawn(async move {
                                    let body = if crate::ui::lang::is_ru() {
                                        format!("Загрузка завершена: {}", n)
                                    } else {
                                        format!("Download complete: {}", n)
                                    };
                                    let _ = Notification::new().summary("LightTorrent").body(&body).show();
                                });
                                if autorm { torm.borrow_mut().push(id); }
                            }
                        }

                        upd.borrow_mut().push(Torr {
                            id, name, size: total, prog: dsprog,
                            dl: dsdl, ul: dsul, state, peers, eta, path
                        });
                    }
                });

                for id in topaus.into_inner() {
                    let _ = api.api_torrent_action_pause(TorrentIdOrHash::Id(id)).await;
                }
                for id in toresm.into_inner() {
                    let _ = api.api_torrent_action_start(TorrentIdOrHash::Id(id)).await;
                }
                for id in torm.into_inner() {
                    let _ = sess.delete(TorrentIdOrHash::Id(id), false).await;
                }

                if let Ok(mut v) = list.lock() { *v = upd.into_inner(); }
            }
        });
    }

    pub async fn addfil(&self, path: PathBuf) -> anyhow::Result<()> {
        let data = fs::read(&path)?;
        self.sess.add_torrent(librqbit::AddTorrent::from_bytes(data), None).await?;
        Ok(())
    }

    pub async fn addmag(&self, link: String) -> anyhow::Result<()> {
        self.sess.add_torrent(librqbit::AddTorrent::from_url(link.as_str()), None).await?;
        Ok(())
    }

    pub async fn rmtrr(&self, id: usize) {
        let _ = self.sess.delete(TorrentIdOrHash::Id(id), false).await;
    }

    pub fn apcfg(&self, new: Cfg) {
        *self.plim.lock().unwrap() = new.plim;
        *self.cfg.lock().unwrap() = new.clone();
        svcfg(&new);
    }
}
