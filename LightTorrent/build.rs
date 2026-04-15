#[cfg(windows)]
fn main() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("icon.ico");
    res.set("ProductName", "LightTorrent");
    res.set("FileDescription", "Lightweight Torrent Client");
    res.set("LegalCopyright", "Copyright © 2026");
    res.set_language(0x0419);
    res.set_version_info(winresource::VersionInfo::PRODUCTVERSION, 0x0001000000040000);
    res.set_version_info(winresource::VersionInfo::FILEVERSION,    0x0001000000040000);
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
