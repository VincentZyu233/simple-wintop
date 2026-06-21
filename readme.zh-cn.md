# 🖥️ htop-rust-for-win 🦀

[![Windows x64 | ARM64](https://img.shields.io/badge/Windows-x64_|_ARM64-0078D4?style=for-the-badge&logo=windows&logoColor=white)](https://github.com/VincentZyuApps/htop-rust-for-win/releases)

> 🌟 轻量级实时 **系统监控仪表盘**，专为 Windows 打造 — 灵感来自 [htop](https://github.com/htop-dev/htop) 🚀
> 完美复刻 **顶部仪表盘区域** — CPU 条形图 🧠、内存与交换空间使用率 🫧、任务计数器 🔢、负载均值 📈、开机时间 ⏱ — 全部呈现在多彩 TUI 中 🎨。

> **[📖 English](readme.md)**
> **[📖 简体中文](readme.zh-cn.md)**

## ✨ 功能特性

- 📊 **CPU 仪表盘** — 每个核心使用率条形图，绿色→红色平滑渐变表示低→高负载 🟢🔴
- 🧠 **内存与交换** — 实时使用率条形图，多色分段显示（绿 → 蓝 → 黄 → 红）🟦🟨
- 🔢 **任务计数** — 总进程数与运行中进程数一目了然 🔄
- 📈 **负载均值** — 平滑 CPU 负载近似值（1 / 5 / 15 分钟）📉
- ⏱ **开机时间** — 系统自上次启动以来的运行时长 🕒
- ⌨️ **键盘快捷键** — `q` / `Esc` 退出，即时响应 🎯
- 🎨 **多彩 TUI** — 基于 [ratatui](https://github.com/ratatui-org/ratatui) + [crossterm](https://github.com/crossterm-rs/crossterm) ✨
- 🪟 **仅限 Windows** — 原生构建 x86_64 和 ARM64 🏗️

## 🖼️ 屏幕截图

![screenshot](docs/screenshot.png)

## 🚀 使用方式

```bash
# 🎯 默认视图 — CPU、内存、交换、任务、负载、开机时间
htop-rust-for-win

# ⏱ 设置自定义刷新间隔（毫秒）
htop-rust-for-win -t 500

# 📖 查看帮助
htop-rust-for-win --help

# ℹ️ 查看版本
htop-rust-for-win --version
```

### ⌨️ 键盘快捷键

| 按键 | 动作 |
|------|------|
| `q` / `Q` / `Esc` | 🚪 退出 |

## 🔧 从源码构建

```bash
# 🦀 使用 Cargo 构建
cargo build --release

# 🏗️ 或使用 CI — 推送到 GitHub 时：
#   "build action"  → 仅构建 🛠️
#   "build release" → 构建 + 发布 GitHub Release 🚀
```

GitHub Actions 会在每次推送时自动构建 **Windows x86_64** 和 **Windows ARM64** 二进制文件。
