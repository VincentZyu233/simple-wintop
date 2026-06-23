![simple-wintop](https://socialify.git.ci/VincentZyuApps/simple-wintop/image?custom_description=%F0%9F%93%8A%F0%9F%A6%80+htop-inspired+Windows+system+monitor+%E2%80%94+CPU+bars%2C+Memory+%26+Swap%2C+Tasks%2C+Uptime+in+a+colorful+TUI+%28Rust+%2B+ratatui%29&description=1&forks=1&issues=1&language=1&logo=https%3A%2F%2Favatars.githubusercontent.com%2Fu%2F250448479%3Fs%3D200%26v%3D4&name=1&owner=1&pulls=1&stargazers=1&theme=Auto)

# 🖥️ simple-wintop 🦀

[![Windows x64 | ARM64](https://img.shields.io/static/v1?label=Windows&message=x64%20%7C%20ARM64&color=0078D4&style=for-the-badge&logo=data:image/svg%2bxml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNCAyNCI+PHBhdGggZD0iTTAgMGgxMS4zNzd2MTEuMzcySDB6TTEyLjYyMyAwSDI0djExLjM3MkgxMi42MjN6TTAgMTIuNjIzaDExLjM3N1YyNEgweiBNMTIuNjIzIDEyLjYyM0gyNFYyNEgxMi42MjN6IiBmaWxsPSIjZmZmIi8+PC9zdmc+)](https://github.com/VincentZyuApps/simple-wintop/releases)
[![Scoop.sh](https://img.shields.io/badge/Scoop.sh-7B4AE2?style=for-the-badge&logo=data:image/svg%2bxml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNCAyNCI+PGNpcmNsZSBjeD0iMTIiIGN5PSI4IiByPSI1IiBmaWxsPSIjRUM3MEExIi8+PGNpcmNsZSBjeD0iOCIgY3k9IjEyIiByPSI0LjUiIGZpbGw9IiNFQkYzQTEiLz48Y2lyY2xlIGN4PSIxNiIgY3k9IjEyIiByPSI0LjUiIGZpbGw9IiM4RTZFQzgiLz48cGF0aCBkPSJNMTYuNSA0bC0xLjUtMS41TDExLjUgNmwxLjUgMS41eiIgZmlsbD0iI2ZmZmZmZiIvPjxwYXRoIGQ9Ik0zIDEzaDE4YzAgNC40LTMuNiA4LTggOGgtNGMtNC40IDAtOC0zLjYtOC04eiIgZmlsbD0iIzRGNEI1MyIvPjwvc3ZnPg==)](https://scoop.sh/#/apps?q=%22https%3A%2F%2Fgithub.com%2FVincentZyuApps%2Fscoop-bucket%22&o=false)

> 🌟 轻量级实时 **系统监控仪表盘**，专为 Windows 打造 — 灵感来自 [htop](https://github.com/htop-dev/htop) 🚀
> 完美复刻 **顶部仪表盘区域** — CPU 条形图 🧠、内存与交换空间使用率 🫧、任务计数器 🔢、开机时间 ⏱ — 全部呈现在多彩 TUI 中 🎨。

> **[📖 English](readme.md)**
> **[📖 简体中文](readme.zh-cn.md)**

## ✨ 功能亮点

- 📊 **CPU 仪表盘** — 每个核心使用率条形图，绿→黄→红渐变表示低→高负载 🟢🟡🔴
- 🧠 **内存与交换** — 实时使用率条形图，多色分段显示（绿 → 蓝 → 黄 → 红）🟦🟨
- 🔢 **任务计数** — 总进程数与运行中进程数一目了然 🔄
- ⏳ **开机时间** — 系统自上次启动以来的运行时长 🕒
- ⌨️ **键盘快捷键** — `q` / `Esc` 退出，即时响应 🎯
- 🎨 **多彩 TUI** — 基于 [ratatui](https://github.com/ratatui-org/ratatui) + [crossterm](https://github.com/crossterm-rs/crossterm) ✨

## 🖼️ 屏幕截图

![screenshot](docs/images/screenshot.png)

## 🚀 快速开始

```bash
# 📖 查看帮助
simple-wintop --help
# 🎯 默认仪表盘 — CPU、内存、交换、任务、开机时间
simple-wintop
# ⏱️ 设置自定义刷新间隔（毫秒）
simple-wintop -t 500
# 📏 紧凑布局 — 零外边距，窄中间间距
simple-wintop --margin-top 0 --margin-bottom 0 --margin-left 0 --margin-right 0 --margin-center 2
# ℹ️ 查看版本
simple-wintop --version
```

## ⚙️ 常用选项

| 选项 | 说明 |
|------|------|
| `-t, --interval <毫秒>` | 刷新间隔（默认：1000） |
| `--margin-top <n>` | 顶部空白行数（默认：1） |
| `--margin-bottom <n>` | 底部空白行数（默认：0） |
| `--margin-left <n>` | 左侧空白列数（默认：1） |
| `--margin-right <n>` | 右侧空白列数（默认：1） |
| `--margin-center <n>` | 左右两栏中间间距（默认：5） |
| `--help` | 打印帮助信息 |
| `--version` | 打印版本信息 |

### ⌨️ 键盘快捷键

| 按键 | 动作 |
|------|------|
| `q` / `Q` / `Esc` | 🚪 退出 |

## 📦 安装

### Windows（Scoop）
> 📄 [Scoop Bucket](https://github.com/VincentZyuApps/scoop-bucket/blob/main/bucket/simple-wintop.json)
```powershell
scoop bucket add vincentzyu https://github.com/VincentZyuApps/scoop-bucket
scoop update
scoop install simple-wintop
```

### 📥 手动下载
从 [GitHub Releases](https://github.com/VincentZyuApps/simple-wintop/releases) 下载最新二进制文件 — 选择 `simple-wintop-x86_64-v0.x.x.exe`（64 位）或 `simple-wintop-arm64-v0.x.x.exe`（ARM64）。

## 🔧 从源码构建

```bash
cargo build --release
```

GitHub Actions 会自动构建 **Windows x86_64** 和 **Windows ARM64** 二进制文件。

## 📦 技术栈

| 包 | 版本 | 说明 |
|:---|:---|:---|
| [![Rust](https://img.shields.io/badge/Rust-stable-CE422B?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/) | stable | 编程语言 |
| [![ratatui](https://img.shields.io/badge/ratatui-0.29-000000?style=flat-square&logo=rust&logoColor=white)](https://github.com/ratatui-org/ratatui) | 0.29 | 终端 UI 框架 |
| [![crossterm](https://img.shields.io/badge/crossterm-0.28-000000?style=flat-square&logo=rust&logoColor=white)](https://github.com/crossterm-rs/crossterm) | 0.28 | 跨平台终端库 |
| [![sysinfo](https://img.shields.io/badge/sysinfo-0.32-000000?style=flat-square&logo=rust&logoColor=white)](https://github.com/GuillaumeGomez/sysinfo) | 0.32 | 系统信息库 |
| [![clap](https://img.shields.io/badge/clap-4-000000?style=flat-square&logo=rust&logoColor=white)](https://github.com/clap-rs/clap) | 4 | 命令行参数解析器 |
| [![unicode-width](https://img.shields.io/badge/unicode--width-0.2-000000?style=flat-square&logo=rust&logoColor=white)](https://github.com/unicode-rs/unicode-width) | 0.2 | Unicode 文本宽度 |

## 📄 许可证

📝 [MIT](LICENSE)
