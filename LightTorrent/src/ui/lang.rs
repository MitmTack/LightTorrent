#[cfg(windows)]
pub fn is_ru() -> bool {
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyboardLayoutList, HKL};
    let mut layouts = [HKL::default(); 32];
    let count = unsafe { GetKeyboardLayoutList(Some(&mut layouts)) };
    for i in 0..count as usize {
        if (layouts[i].0 as usize & 0xFFFF) == 0x0419 { return true; }
    }
    false
}

#[cfg(not(windows))]
pub fn is_ru() -> bool { false }

#[derive(PartialEq, Copy, Clone)]
pub enum Lang { RU, EN }

pub struct Tr {
    pub all:       &'static str,
    pub down:      &'static str,
    pub seed:      &'static str,
    pub setts:     &'static str,
    pub del:       &'static str,
    pub open:      &'static str,
    pub addt:      &'static str,
    pub mag:       &'static str,
    pub name:      &'static str,
    pub size:      &'static str,
    pub prog:      &'static str,
    pub dl:        &'static str,
    pub ul:        &'static str,
    pub peers:     &'static str,
    pub eta:       &'static str,
    pub langset:   &'static str,
    pub plimset:   &'static str,
    pub about:     &'static str,
    pub aboutb:    &'static str,
    pub aboutdesc: &'static str,
    pub authlbl:   &'static str,
    pub dldirlbl:  &'static str,
    pub maxdllbl:  &'static str,
    pub maxullbl:  &'static str,
    pub autorm:    &'static str,
    pub setrto:    &'static str,
    pub savebtn:   &'static str,
    pub nolimt:    &'static str,
    pub tabgen:    &'static str,
    pub tabnet:    &'static str,
}

pub const RU: Tr = Tr {
    all:       "Все",
    down:      "Загрузки",
    seed:      "Раздачи",
    setts:     "Настройки",
    del:       "Удалить",
    open:      "Открыть папку",
    addt:      "Добавить .torrent",
    mag:       "Магнет-ссылка:",
    name:      "Название",
    size:      "Размер",
    prog:      "Прогресс",
    dl:        "Загрузка",
    ul:        "Отдача",
    peers:     "Пиры",
    eta:       "Осталось",
    langset:   "Язык интерфейса",
    plimset:   "Лимит пиров на торрент",
    about:     "О программе",
    aboutb:    "Инфо",
    aboutdesc: "Легко. Быстро. Прозрачно.",
    authlbl:   "Автор:",
    dldirlbl:  "Папка загрузок",
    maxdllbl:  "Макс. скорость загрузки",
    maxullbl:  "Макс. скорость отдачи",
    autorm:    "Удалять торрент после завершения",
    setrto:    "Удалять через (часов раздачи):",
    savebtn:   "Сохранить",
    nolimt:    "без лимита",
    tabgen:    "Основное",
    tabnet:    "Сеть",
};

pub const EN: Tr = Tr {
    all:       "All",
    down:      "Downloads",
    seed:      "Seeding",
    setts:     "Settings",
    del:       "Delete",
    open:      "Open folder",
    addt:      "Add .torrent",
    mag:       "Magnet link:",
    name:      "Name",
    size:      "Size",
    prog:      "Progress",
    dl:        "Down",
    ul:        "Up",
    peers:     "Peers",
    eta:       "ETA",
    langset:   "Interface Language",
    plimset:   "Peer limit per torrent",
    about:     "About",
    aboutb:    "Info",
    aboutdesc: "Light. Fast. Transparent.",
    authlbl:   "Author:",
    dldirlbl:  "Download folder",
    maxdllbl:  "Max download speed",
    maxullbl:  "Max upload speed",
    autorm:    "Remove torrent when done",
    setrto:    "Remove after seeding (hours):",
    savebtn:   "Save",
    nolimt:    "unlimited",
    tabgen:    "General",
    tabnet:    "Network",
};
