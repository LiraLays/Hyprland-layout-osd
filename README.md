# Hyprland-layout-osd
Hyprland-layout-osd is a service written on Rust that displays the name of the switched keyboard layout via SwayOSD.
This is my first Rust project, so please go easy on me!

Currently, only two keyboard layouts are supported: English and Russian.

### Images
![image 1](assets/en_layout.png)
![image 2](assets/ru_layout.png)
### Dependencies
- On Arch: rustup, base-devel
- Hyprland
- SwayOSD (https://github.com/ErikReider/SwayOSD)
- swayosd-server in Hyprland autostart BEFORE script (```exec-once = swayosd-server```)

### Install
1. Download the script's source code.
2. Ensure SwayOSD (https://github.com/ErikReider/SwayOSD) is installed.
3. Install the `rustup` and `base-devel` packages (for Arch Linux). These are required to compile the Rust project.
4. Navigate to the project directory.
5. Run the following command in the project directory: `cargo build --release`.
6. Create the `~/.local/bin` directory if it does not exist: `mkdir -p ~/.local/bin`.
7. Copy the project binary to `~/.local/bin`: `cp target/release/hypr-layout-osd ~/.local/bin/`.
8. Add `exec-once = ~/.local/bin/hypr-layout-osd` to your Hyprland configuration to launch the script automatically at system startup.
