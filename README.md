# LightTorrent

LightTorrent keeps it simple. Fast downloads, clean interface, zero bloat. Built to be lightweight and transparent so you always know what is happening under the hood. Open source from day one because you deserve a client that just works without the nonsense. No tracking no clutter just torrents done right.

## What it does

Honestly most torrent clients feel heavy and packed with stuff I never touch. So I made this one for myself and figured others might want it too. It just downloads files and stays out of your way.

It handles magnets and torrent files like you would expect. Drop a file on the window or paste a link and it goes. There is a setting to limit peers per torrent so your router does not freak out. You can also tell it to remove finished torrents after seeding for a few hours which is nice if you wanna share back but not think about it.

The interface works in English or Russian depending on your system. A little popup shows up when downloads finish so you can ignore it until then.

## Getting it running

Just grab a release from the releases page and unzip it somewhere. Run the exe and you are good.

If you wanna build it yourself you need Rust. Clone the thing and run cargo build release. Takes maybe a minute.

git clone https://github.com/MitmTack/LightTorrent.git
cd LightTorrent
cargo build --release

## How to use it

Adding torrents is easy enough. Click the add button pick a file. Or paste a magnet link in that box on the left and hit enter. Dragging a torrent file onto the window also works which i use all the time.

Right click any torrent and you can open the folder where stuff went or delete it if you dont want it anymore. The sidebar lets you switch between seeing everything just downloads or just seeding stuff.

Settings are pretty basic. You pick where files go set some speed limits if you need to or change the peer count. Everything saves to a file next to the exe so your config travels with the folder if you move it.

## Why i made this

I got tired of clients that felt sketchy or did a million things in the background. This one is simple and the code is right here so you can look through it yourself. No telemetry no ads no calling home to some server i dont know about.

It uses librqbit underneath which handles the actual torrent protocol stuff. The ui is egui which is why it feels snappy and doesnt lag. Tokio keeps everything running smooth while downloads happen.

If you find a bug or whatever open an issue. I check when i have time and try to respond when i can.

## License

MIT so do whatever you want with it really. Use it change it share it i dont mind.

## Who made it

MitmTack

[GitHub](https://github.com/MitmTack)

[Telegram](https://t.me/mitmtack)
