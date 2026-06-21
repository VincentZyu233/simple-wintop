# 🖥️ htop-rust-for-win 🦀

[![Windows x64 | ARM64](https://img.shields.io/badge/Windows-x64_|_ARM64-0078D4?style=for-the-badge&logo=windows&logoColor=white)](https://github.com/VincentZyuApps/htop-rust-for-win/releases)

> 🌟 A lightweight, real-time **system monitor dashboard** for Windows — inspired by [htop](https://github.com/htop-dev/htop) 🚀
> Purely the **top meters section** — CPU bars 🧠, Memory & Swap usage 🫧, Tasks counter 🔢, Load average 📈, and Uptime ⏱ — all in a beautiful multi‑color TUI 🎨.

> **[📖 English](readme.md)**
> **[📖 简体中文](readme.zh-cn.md)**

## ✨ Features

- 📊 **CPU meters** — per‑core usage bars with smooth green→red gradient for low→high load 🟢🔴
- 🧠 **Memory & Swap** — real‑time usage bars with multi‑color segments (green → blue → yellow → red) 🟦🟨
- 🔢 **Tasks counter** — total processes and running thread count at a glance 🔄
- 📈 **Load average** — smoothed CPU load approximation (1 / 5 / 15 min) 📉
- ⏱ **Uptime** — system uptime since last boot 🕒
- ⌨️ **Keyboard shortcuts** — `q` / `Esc` to quit, instant and responsive 🎯
- 🎨 **Colorful TUI** — powered by [ratatui](https://github.com/ratatui-org/ratatui) + [crossterm](https://github.com/crossterm-rs/crossterm) ✨
- 🪟 **Windows only** — built natively for x86_64 and ARM64 🏗️

## 🖼️ Screenshot

![screenshot](docs/screenshot.png)

## 🚀 Usage

```bash
# 🎯 Default view — CPU, Memory, Swap, Tasks, Load, Uptime
htop-rust-for-win

# ⏱ Set custom refresh interval (milliseconds)
htop-rust-for-win -t 500

# 📖 Show help
htop-rust-for-win --help

# ℹ️ Show version
htop-rust-for-win --version
```

### ⌨️ Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `q` / `Q` / `Esc` | 🚪 Quit |

## 🔧 Build from Source

```bash
# 🦀 Build with Cargo
cargo build --release

# 🏗️ Or use CI — push to GitHub with:
#   "build action"  → just build 🛠️
#   "build release" → build + GitHub Release 🚀
```

GitHub Actions automatically builds **Windows x86_64** and **Windows ARM64** binaries on every push.
