# valorant-tap-fire-enforcer
My first Rust project!
Made for [my friend](https://www.youtube.com/channel/UCUA2BBixCfqjLVDj-yH6hGQ) who is addicted (but somehow still awful) to Valorant.

This is a command-line tool to help break the habit of crouch-spraying in Valorant by forcing the left mouse button up after a set amount of bullets are fired.

## Usage
### Building
*The executable will be in `target/release` by default.*
```sh
cargo build --release
```
Running:
```sh
valorant_tap_fire_enforcer.exe --help
valorant_tap_fire_enforcer.exe --weapon-name Vandal --bullet-count 2
```
