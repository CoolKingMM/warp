# Warp OSS Slim Pure

> 本分支为 **Warp** 的纯净开源与极限瘦身构建版本（Warp OSS Slim Pure），基于 Rust 与自研 GPU 渲染引擎打造，专注于提供极速、纯净、无任何商业干扰与遥测绑定的现代化终端体验。

---

## 核心设计原则与约束规范（Project Constraints）

本项目严格遵循以下开发、构建与维护约束：

### 1. 严格禁止本地编译（No Local Builds）
- **CI 优先原则**：所有 Windows 正式构建产物必须全部交由 **GitHub Actions 云端自动化编译**（`Build Windows OSS` 工作流），禁止在本地机器执行 `cargo build` 全量编译，确保构建环境纯净与依赖一致性。
- **隔离验证原则**：云端编译完成后，产物下载至本地隔离目录（`.tmp/actions/<run-id>/`）执行冒烟测试与功能验证，严禁直接覆盖或污染本地生产环境中的现有安装。

### 2. 彻底物理剥离商业闭源模块（Zero Commercial Code）
- **物理删除而非条件跳过**：坚决摒弃 `if cfg!(...) { return; }` 式的软屏蔽，对商业闭源、计费充值、拉新推广等模块执行彻底的源码级物理移除（累计物理抹除 22,000+ 行非核心代码）。
- **已清空的商业组件**：
  - 彻底移除 `Billing and Usage` 商业账单与积分充值页面（包含超额扣款限额、按月自动续费设置等）；
  - 彻底移除 `Teams` 商业团队管理与 Stripe 扣费结算网关；
  - 彻底移除 `Referrals` 邀请拉新推广与返利体系；
  - 彻底移除启动广告幻灯片（`launch_modal`）、付费升级拦截弹窗（`free_tier_limit_hit_modal`）与模型升级提示。

### 3. 隐私保护与零遥测后门（Zero Telemetry & Sentry）
- **切断遥测外发**：清空 RudderStack 遥测外发模块与本地事件持久化落盘机制，网络上报通道清空为 No-op。
- **抹除 Sentry 远程监控**：彻底移除 Sentry 异常监控连接、Minidump 常驻守护子进程与系统环境标签上报。
- **消除后台心跳与退出等待**：彻底移除 30s/60s 周期性定时唤醒器，抹除客户端退出时强制挂起 5 秒等待发包的逻辑，实现秒关秒退。

### 4. 极致内存优化与自动工作集回收（Memory Optimization）
- **冷启动尖峰自动收缩**：应用启动 8 秒后（完成系统字体加载、DirectX 着色器构建与历史初始化），后台自动触发 `trim_working_set()` 将冷页与只读资源释放交还给操作系统。
- **窗口失焦/最小化即时修剪**：在 Windows 事件循环中监听 `WindowEvent::Focused(false)`，窗口最小化或切至后台时立即主动收缩工作集，使物理内存由 400+ MB 骤降至数十兆（20 ~ 30 MB 级别）。

### 5. 核心终端功能完整性（Zero Breakage）
- 保证终端仿真器内核（ANSI/VT100 状态机、Escape 序列解析、PTY 跨平台管道）；
- 保证 Block 分块交互模型、分屏组（Splits）、多标签页（Tabs）；
- 保证自研 WebGPU / DirectX GPU 加速渲染与字体栅格化；
- 保证命令智能补全、语法高亮与本地历史持久化。

### 6. 云端构建持续监控约束（CI Monitoring Policy）
- **40 分钟持续监控**：触发 GitHub Actions 云端编译后，必须保持持续 40 分钟的状态追踪与指标轮询。
- **每隔 5 分钟汇报一次**：以 5 分钟为一个阶段窗口，定时汇总并向用户汇报编译进度、当前步骤与耗时，严禁使用长时间无果或无响应的挂起命令。
- **构建完成即刻响应**：一旦检测到云端任务提前完成（`conclusion: success`），立即结束轮询，无缝进入产物下载、隔离环境 Smoke Test 验证与 Release 部署流程，严禁无效空转。

---

## 构建与发布规范（Build & Release）

### GitHub Actions 自动化流程
工作流定义位于 `.github/workflows/build-windows-oss.yml`：
- **Tag 触发发布**：推送以 `v*` 开头的 Tag（例如 `v0.0.0-oss-slim-pure`）即可自动触发完整编译、安装包制作与 Release 发布：
  - 编译主程序二进制：`warp-oss.exe`；
  - 打包 Inno Setup 安装包：`WarpOssSetup.exe`；
  - 组装免安装便携版压缩包：`WarpOssPortable-windows-x64.zip`；
  - 自动更新并发布到 GitHub Releases。
- **跨分支构建加速**：配置了 `rust-cache` 跨分支依赖持久化，全量构建在命中缓存后可在 15~18 分钟内全自动完成。

---

## 产物形态说明

| 产物名称 | 文件类型 | 适用场景 | 特性与说明 |
| :--- | :--- | :--- | :--- |
| **`WarpOssPortable-windows-x64.zip`** | 绿色便携免安装包 | **最推荐** | 解压即用，不写注册表，不冲突已有安装，支持随身携带。 |
| **`WarpOssSetup.exe`** | Windows 一键安装包 | 习惯桌面安装向导 | 自动创建开始菜单、桌面快捷方式与系统上下文菜单。 |
