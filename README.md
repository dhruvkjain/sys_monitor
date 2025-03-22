# System Monitor TUI

A terminal-based system monitoring tool built in Rust using `sysinfo` and `ratatui`. It displays real-time system statistics, including CPU, memory, disk, and network usage, in a graphical terminal interface.

## Features
- Displays **per-core CPU usage**
- Shows **memory and swap usage**
- Lists **disk usage** per mounted device
- Monitors **network activity**
- Refreshes statistics in real-time

![image](https://github.com/user-attachments/assets/5cff49bf-0916-462c-9f87-3073b800b595)

## Installation
Ensure you have Rust installed. If not, install it using [Rustup](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then, clone this repository and build the project:

```sh
git clone <repository-url>
cd system-monitor-tui
cargo build --release
```

## Usage
Run the executable:

```sh
cargo run --release
```

Press `q` to exit the application.

## Dependencies
This project uses:
- [`sysinfo`](https://crates.io/crates/sysinfo) - For retrieving system statistics
- [`ratatui`](https://crates.io/crates/ratatui) - For rendering the TUI
- [`crossterm`](https://crates.io/crates/crossterm) - For handling terminal input/output


