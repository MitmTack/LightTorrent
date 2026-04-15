use std::time::Duration;

#[derive(Clone, PartialEq, Debug)]
pub enum State {
    Loading,
    Down,
    Seed,
    Pause,
}

#[derive(Clone, Debug)]
pub struct Torr {
    pub id: usize,
    pub name: String,
    pub size: u64,
    pub prog: f32,
    pub dl: f64,
    pub ul: f64,
    pub state: State,
    pub peers: u32,
    pub eta: Duration,
    pub path: std::path::PathBuf,
}