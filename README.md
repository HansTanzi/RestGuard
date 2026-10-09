# RestGuard

跨平台的强制休息提醒工具：每工作一段时间，全屏遮罩强制你休息，倒计时结束前关不掉。

A cross-platform break reminder that actually makes you rest. Built with Rust + Tauri 2 + Svelte.

> 前身是 Windows 专用的 C# 版 [ProtectEyes](./ProtectEyes)。

## 功能

- 每工作 60 分钟，强制休息 10 分钟（可配置）
- 多显示器：每块屏幕都会被遮挡
- 休息期间无法关闭遮罩（Alt+F4 无效），也无法从托盘退出
- 可推迟 5 分钟，最多 3 次；完整休息一次后次数重置
- 托盘菜单：查看剩余时间、提前休息、开机自启
- 单实例运行

## 配置

首次运行会生成 `config.toml`，修改后重启生效：

| 系统 | 路径 |
|---|---|
| Windows | `%APPDATA%\io.github.restguard\config.toml` |
| macOS | `~/Library/Application Support/io.github.restguard/config.toml` |
| Linux | `~/.config/io.github.restguard/config.toml` |

```toml
work_minutes = 60.0      # 连续工作多久后强制休息
rest_minutes = 10.0      # 每次休息多久
postpone_minutes = 5.0   # 每次推迟多久
max_postpones = 3        # 完整休息前最多推迟几次
overlay_coverage = 1.0   # 遮罩覆盖屏幕的比例 0.1 ~ 1.0
```

设置项刻意不放在界面里，否则就不算"强制"休息了。

## 平台限制

- **macOS**：`Cmd+Q`、调度中心等系统手势无法完全拦截
- **Linux Wayland**：协议不允许应用自行置顶和定位窗口，遮罩可能失效；X11 正常

## 开发

需要 [Rust](https://rustup.rs)、Node.js、pnpm，以及 [Tauri 的系统依赖](https://tauri.app/start/prerequisites/)。

```sh
pnpm install
pnpm tauri dev      # 开发运行
pnpm tauri build    # 打包安装包
cd src-tauri && cargo test
```

调试时可以把 `work_minutes` 设为 `0.1`（6 秒），快速触发休息。
