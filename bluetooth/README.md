# README: Bluetooth Toggle App (Linux, GTK + Rust)

A simple Linux desktop application built with **Rust** and **GTK4** that allows users to toggle Bluetooth ON/OFF via a user-friendly graphical interface. It uses **BlueZ D-Bus API** for system-level Bluetooth control and handles asynchronous logic using **Tokio**.

---

## Features

* Toggle Bluetooth ON/OFF with a single click
* Asynchronous D-Bus communication using `zbus`
* Custom GUI using `gtk4` with CSS styling
* Safe and modern Rust codebase

---

## Requirements

### System Dependencies

Install required libraries and tools on a Debian-based Linux system (like Ubuntu):

```bash
sudo apt update
sudo apt install -y build-essential libgtk-4-dev libglib2.0-dev libdbus-1-dev pkg-config
```

### Rust Toolchain

If you haven’t installed Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

---

## Rust Dependencies

Make sure your `Cargo.toml` includes:

```toml
[dependencies]
gtk = { version = "0.7", package = "gtk4" }
glib = "0.18"
zbus = "3.14"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

---

## How to Compile and Run

### 1. Clone the Repository

```bash
git clone https://github.com/abhinash000/LINUX-DESKTOP-APPLICATION.git
cd LINUX-DESKTOP-APPLICATION/bluetooth_toggle
```

### 2. Build the Project

```bash
cargo build --release
```

### 3. Run the App

```bash
cargo run
```

A GTK window will appear with a toggle button. Switching it ON/OFF updates the Bluetooth power state using D-Bus.

---

## Notes on D-Bus Permissions

* The app connects to the system D-Bus to communicate with `org.bluez`
* Make sure your user has permission to control Bluetooth (usually granted by default on Ubuntu)
* You can verify adapter state using:

```bash
bluetoothctl show
```

---

## Future Enhancements

* Device list and pairing
* Status indicator icon
* Packaged as `.deb` or `.AppImage`

---

## Author

**Abhinash** ([@abhinash000](https://github.com/abhinash000))
Made with ❤️ in Rust

---

## License

This project is licensed under the **MIT License**. Free to use, modify, and distribute.

---

*This document serves as the complete user and developer guide for the Bluetooth Toggle application inside the `LINUX-DESKTOP-APPLICATION` repository.*
