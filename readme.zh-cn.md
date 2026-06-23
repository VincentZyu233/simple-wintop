# 🖥️ simple-wintop 🦀

[![Windows x64 | ARM64](https://img.shields.io/badge/Windows-x64_|_ARM64-0078D4?style=for-the-badge&logo=windows&logoColor=white)](https://github.com/VincentZyuApps/simple-wintop/releases)

> 🌟 轻量级实时 **系统监控仪表盘**，专为 Windows 打造 — 灵感来自 [htop](https://github.com/htop-dev/htop) 🚀
> 完美复刻 **顶部仪表盘区域** — CPU 条形图 🧠、内存与交换空间使用率 🫧、任务计数器 🔢、开机时间 ⏱ — 全部呈现在多彩 TUI 中 🎨。

> **[📖 English](readme.md)**
> **[📖 简体中文](readme.zh-cn.md)**

## ✨ 功能亮点

- 📊 **CPU 仪表盘** — 每个核心使用率条形图，绿→黄→红渐变表示低→高负载 🟢🟡🔴
- 🧠 **内存与交换** — 实时使用率条形图，多色分段显示（绿 → 蓝 → 黄 → 红）🟦🟨
- 🔢 **任务计数** — 总进程数与运行中进程数一目了然 🔄
- ⏱ **开机时间** — 系统自上次启动以来的运行时长 🕒
- ⌨️ **键盘快捷键** — `q` / `Esc` 退出，即时响应 🎯
- 🎨 **多彩 TUI** — 基于 [ratatui](https://github.com/ratatui-org/ratatui) + [crossterm](https://github.com/crossterm-rs/crossterm) ✨
- 🪟 **仅限 Windows** — 原生构建 x86_64 和 ARM64 🏗️

## 🖼️ 屏幕截图

![screenshot](docs/screenshot.png)

## 🚀 快速开始

```bash
# 📖 查看帮助
simple-wintop --help

# 🎯 默认仪表盘 — CPU、内存、交换、任务、开机时间
simple-wintop

# ⏱ 设置自定义刷新间隔（毫秒）
simple-wintop -t 500

# ℹ️ 查看版本
simple-wintop --version
```

## ⚙️ 常用选项

| 选项 | 说明 |
|------|------|
| `-t, --interval <毫秒>` | 刷新间隔（默认：1000） |
| `--help` | 打印帮助信息 |
| `--version` | 打印版本信息 |

### ⌨️ 键盘快捷键

| 按键 | 动作 |
|------|------|
| `q` / `Q` / `Esc` | 🚪 退出 |

## 🔧 从源码构建

```bash
# 🦀 使用 Cargo 构建
cargo build --release
```

GitHub Actions 会自动构建 **Windows x86_64** 和 **Windows ARM64** 二进制文件。

## 📄 许可证

📝 MIT
