
## 开发原则

### ⚠️ 不要造锤子原则
- **写任何功能前，先搜索 GitHub 上是否有现成的 Rust crate 或开源实现**
- 优先使用社区成熟的库，不要自己从零实现
- 以下功能已有成熟方案，直接集成：
  - 键鼠输入监听 → `rdev` crate
  - 系统托盘 → `tauri-plugin-tray`
  - 透明窗口/点击穿透 → `window-vignette` 或 `tauri-plugin-window-state`
  - 配置持久化 → `tauri-plugin-store`
  - 开机自启 → `tauri-plugin-autostart`
  - 全局快捷键 → `tauri-plugin-global-shortcut`
  - 通知 → `tauri-plugin-notification`
