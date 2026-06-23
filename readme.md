![simple-wintop](https://socialify.git.ci/VincentZyuApps/simple-wintop/image?custom_description=%F0%9F%93%8A%F0%9F%A6%80+htop-inspired+Windows+system+monitor+%E2%80%94+CPU+bars%2C+Memory+%26+Swap%2C+Tasks%2C+Uptime+in+a+colorful+TUI+%28Rust+%2B+ratatui%29&description=1&forks=1&issues=1&language=1&logo=https%3A%2F%2Favatars.githubusercontent.com%2Fu%2F250448479%3Fs%3D200%26v%3D4&name=1&owner=1&pulls=1&stargazers=1&theme=Auto)

# 🖥️ simple-wintop 🦀

[![Windows x64 | ARM64](https://img.shields.io/static/v1?label=Windows&message=x64%20%7C%20ARM64&color=0078D4&style=for-the-badge&logo=data:image/svg%2bxml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNCAyNCI+PHBhdGggZD0iTTAgMGgxMS4zNzd2MTEuMzcySDB6TTEyLjYyMyAwSDI0djExLjM3MkgxMi42MjN6TTAgMTIuNjIzaDExLjM3N1YyNEgweiBNMTIuNjIzIDEyLjYyM0gyNFYyNEgxMi42MjN6IiBmaWxsPSIjZmZmIi8+PC9zdmc+)](https://github.com/VincentZyuApps/simple-wintop/releases)
[![Scoop.sh](https://img.shields.io/badge/Scoop.sh-7B4AE2?style=for-the-badge&logo=data:image/svg%2bxml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNCAyNCI+PGNpcmNsZSBjeD0iMTIiIGN5PSI4IiByPSI1IiBmaWxsPSIjRUM3MEExIi8+PGNpcmNsZSBjeD0iOCIgY3k9IjEyIiByPSI0LjUiIGZpbGw9IiNFQkYzQTEiLz48Y2lyY2xlIGN4PSIxNiIgY3k9IjEyIiByPSI0LjUiIGZpbGw9IiM4RTZFQzgiLz48cGF0aCBkPSJNMTYuNSA0bC0xLjUtMS41TDExLjUgNmwxLjUgMS41eiIgZmlsbD0iI2ZmZmZmZiIvPjxwYXRoIGQ9Ik0zIDEzaDE4YzAgNC40LTMuNiA4LTggOGgtNGMtNC40IDAtOC0zLjYtOC04eiIgZmlsbD0iIzRGNEI1MyIvPjwvc3ZnPg==)](https://scoop.sh/#/apps?q=%22https%3A%2F%2Fgithub.com%2FVincentZyuApps%2Fscoop-bucket%22&o=false)

> 🌟 A lightweight, real-time **system monitor dashboard** for Windows — inspired by [htop](https://github.com/htop-dev/htop) 🚀
> Purely the **top meters section** — CPU bars 🧠, Memory & Swap usage 🫧, Tasks counter 🔢, and Uptime ⏱ — all in a beautiful multi‑color TUI 🎨.

> **[📖 English](readme.md)**
> **[📖 简体中文](readme.zh-cn.md)**

## ✨ Highlights

- 📊 **CPU meters** — per‑core usage bars with green→yellow→red gradient for low→high load 🟢🟡🔴
- 🧠 **Memory & Swap** — real‑time usage bars with multi‑color segments (green → blue → yellow → red) 🟦🟨
- 🔢 **Tasks counter** — total processes and running count at a glance 🔄
- ⏳ **Uptime** — system uptime since last boot 🕒
- ⌨️ **Keyboard shortcuts** — `q` / `Esc` to quit, instant and responsive 🎯
- 🎨 **Colorful TUI** — powered by [ratatui](https://github.com/ratatui-org/ratatui) + [crossterm](https://github.com/crossterm-rs/crossterm) ✨

## 🖼️ Screenshot

![screenshot](docs/images/screenshot.png)

## 🚀 Quick Start

```bash
# 📖 Show help
simple-wintop --help
# 🎯 Default dashboard — CPU, Memory, Swap, Tasks, Uptime
simple-wintop
# ⏱️ Set custom refresh interval (in milliseconds)
simple-wintop -t 500
# 📏 Compact layout — zero outer margins, narrow center gap
simple-wintop --margin-top 0 --margin-bottom 0 --margin-left 0 --margin-right 0 --margin-center 2
# ℹ️ Show version
simple-wintop --version
```

## ⚙️ Common Flags

| Flag | Description |
|------|-------------|
| `-t, --interval <ms>` | Refresh interval in milliseconds (default: 1000) |
| `--margin-top <n>` | Top margin rows (default: 1) |
| `--margin-bottom <n>` | Bottom margin rows (default: 0) |
| `--margin-left <n>` | Left margin columns (default: 1) |
| `--margin-right <n>` | Right margin columns (default: 1) |
| `--margin-center <n>` | Center gap between left/right columns (default: 5) |
| `--help` | Print help message |
| `--version` | Print version info |

### ⌨️ Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `q` / `Q` / `Esc` | 🚪 Quit |

## 📦 Installation

### Windows (Scoop)
> 📄 [Scoop Bucket](https://github.com/VincentZyuApps/scoop-bucket/blob/main/bucket/simple-wintop.json)
```powershell
scoop bucket add vincentzyu https://github.com/VincentZyuApps/scoop-bucket
scoop update
scoop install simple-wintop
```

### 📥 Manual Download
Download the latest binary from [GitHub Releases](https://github.com/VincentZyuApps/simple-wintop/releases) — choose `simple-wintop-x86_64-v0.x.x.exe` (64-bit) or `simple-wintop-arm64-v0.x.x.exe` (ARM64).

## 🔧 Build from Source

```bash
cargo build --release
```

GitHub Actions automatically builds **Windows x86_64** and **Windows ARM64** binaries.

## 📦 Tech Stack

| Package | Version | Description |
|:---|:---|:---|
| [![Rust](https://img.shields.io/badge/Rust-stable-CE422B?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/) | stable | Programming language |
| [![ratatui](https://img.shields.io/badge/ratatui-0.29-000000?style=flat-square&logo=rust&logoColor=white)](https://github.com/ratatui-org/ratatui) | 0.29 | Terminal UI framework |
| [![crossterm](https://img.shields.io/badge/crossterm-0.28-000000?style=flat-square&logo=rust&logoColor=white)](https://github.com/crossterm-rs/crossterm) | 0.28 | Cross-platform terminal library |
| [![sysinfo](https://img.shields.io/badge/sysinfo-0.32-000000?style=flat-square&logo=rust&logoColor=white)](https://github.com/GuillaumeGomez/sysinfo) | 0.32 | System information library |
| [![clap](https://img.shields.io/badge/clap-4-000000?style=flat-square&logo=rust&logoColor=white)](https://github.com/clap-rs/clap) | 4 | Command-line argument parser |
| [![unicode-width](https://img.shields.io/badge/unicode--width-0.2-000000?style=flat-square&logo=rust&logoColor=white)](https://github.com/unicode-rs/unicode-width) | 0.2 | Unicode text width |

## 📄 License

📝 [MIT](LICENSE)
