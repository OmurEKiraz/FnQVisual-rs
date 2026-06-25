# FnQVisual-rs 🚀

A zero-overhead, highly customizable On-Screen Display (OSD) overlay for Lenovo laptops running Linux. Written in Rust and GTK4, it brings the familiar visual feedback of the Windows Lenovo Fn+Q overlay natively to Linux (Wayland/X11) when switching performance modes.

This project was developed and explicitly tested on a **Lenovo IdeaPad Gaming 3 (15ACH6, Model 82K2)** running CachyOS with KDE Plasma 6 (Wayland).

## Features
* **Zero Overhead:** Consumes ~28MB of RAM and 0.0% CPU while idle.
* **Wayland Native:** Uses `gtk4-layer-shell` for true overlay positioning, click-through support, and focus-stealing prevention.
* **Highly Customizable:** Change colors, opacity, fonts, margins, text, and custom icons via a simple TOML configuration file.
* **Smart Polling:** Hardware debounce protection flushes the queue to prevent UI stuttering if you spam the shortcut keys.

---

## Installation & Deployment

### Method 1: Arch User Repository (AUR)
If you are on Arch Linux or an Arch-based distribution (like CachyOS, Manjaro, or EndeavourOS), you can install it using your favorite AUR helper:

paru -S fnq-visual

### Method 2: Building from Source
If you prefer to build it yourself or are using another distribution, follow these steps:

#### 1. Install Dependencies
Ensure you have `cargo`, `gtk4` (v4.10+ recommended), and `gtk4-layer-shell` installed via your package manager.

#### 2. Clone and Build
git clone https://github.com/omrkrz/FnQVisual-rs.git
cd FnQVisual-rs
cargo build --release --locked

The compiled binary will be located at `target/release/fnq-visual`.

---

## Setting Up the Background Service

To have the overlay start automatically with your graphical environment without needing manual terminal interaction, use systemd at the user level. **(DO NOT use sudo)**:

systemctl --user enable --now fnq-visual.service

---

## Configuration

Upon its very first launch, the application automatically creates a default configuration file for you at:
`~/.config/fnq-visual/config.toml`

You can change text labels, layout properties, or point `icon_path` to your own high-resolution PNG assets instead of using the embedded icons.

### Configuration Options

display_duration_ms = 2500  # OSD visible time in milliseconds

[window]
width = 200                 # Width of the OSD
height = 180                # Height of the OSD
icon_size = 72              # Size of the profile icon
font_size = 16000           # Font size in GTK Pango units (16000 = 16pt)
font_weight = "heavy"       # "heavy", "bold", "normal"
anchor_edge = "bottom"      # Edge alignment: "bottom", "top", or "center"
margin_offset = 100         # Pixels away from the screen edge
background_rgba = "rgba(25, 25, 25, 0.70)" # OSD window background color

[quiet]
text = "QUIET MODE"
icon_path = null            # Set a valid file path to override embedded asset

[balanced]
text = "BALANCED MODE"
icon_path = null

[performance]
text = "PERFORMANCE MODE"
icon_path = null

---

## Compatibility & Contributing

This application monitors the hardware ACPI state via `/sys/firmware/acpi/platform_profile`.

* **KDE Plasma 6 (Wayland) / wlroots (Hyprland, Sway):** Fully supported via native layer-shell implementations.
* **GNOME (Wayland) & X11 Environments:** Currently **untested**. 

### We need your help!
If you are running GNOME or an X11 environment, please help us test! If you encounter issues with focus-stealing, transparency, or window borders, feel free to **open an issue** or submit a **pull request** to help us perfect the support across all setups.

---

## License

This project is licensed under the **GNU General Public License v3.0 (GPL-3.0)**. See the `LICENSE` file for details.