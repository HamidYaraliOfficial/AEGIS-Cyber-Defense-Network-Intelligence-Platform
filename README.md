# AEGIS — Cyber Defense & Network Intelligence Platform

<div align="center">

**A local-first Security Operations Center for your own network.**
Built with **Rust**, **Tauri 2**, **React**, and **TypeScript**.

English · [فارسی](#فارسی) · [中文](#中文)

</div>

---

## English

### Overview

AEGIS is a desktop Cyber Defense & Network Intelligence platform. It discovers and maps the devices on **your own authorized local network**, samples active connections, runs a lightweight detection engine (port-scan, connection-spike, DNS-anomaly, and authentication-failure heuristics), raises alerts you can escalate into incidents, monitors file integrity, and includes a fully **tool-based, read-only AI Security Analyst** that explains and correlates activity without ever taking action on its own.

The entire backend is written in Rust (Tokio, Rusqlite/SQLite, Rayon, Rustls, Argon2/AES-GCM) running inside a Tauri 2 shell, with a React + TypeScript frontend styled as a dark, glassmorphism SOC console.

### Features

- **Interactive Network Map** — live, animated topology graph of routers, computers, servers, mobile devices, IoT, and printers on your LAN
- **Device Inventory** — hostname/MAC/vendor detection, per-device risk scoring, on-demand port scanning
- **Security Timeline** — searchable, filterable event log with AI-assisted correlation
- **Packet / Flow Explorer** — live sampling of active TCP/UDP connections
- **Log Intelligence Center** — fast full-text search with saved searches
- **Incident Response Workspace** — convert alerts into incidents, track status, add investigation notes
- **Rules Studio** — visual detection rule builder (port scan, connection spike, DNS anomaly, auth failure, custom)
- **File Integrity Monitoring** — SHA-256 baseline hashing and change detection for files you choose to watch
- **AI Security Analyst** — strictly read-only, tool-based correlation and explanation engine; it never issues commands, blocks traffic, or takes autonomous action
- **Secure Vault** — AES-256-GCM encryption with Argon2id key derivation; nothing is ever stored in plain text
- **Six built-in themes** — Dark Premium, Light, Windows 11 Default, Crimson (Red), Cyan Blue, and AMOLED
- **Three languages** — English, فارسی (Persian, full RTL), 中文 (Chinese) — switchable at runtime
- **Performance Profiler** — live CPU, RAM, and network throughput on the dashboard

### Scope & safety

AEGIS is a **defensive** tool by design. It never performs exploitation, credential theft, malware deployment, persistence, or automated/offensive actions of any kind. Device discovery uses standard ICMP ping and OS ARP-table reads; port scanning is a simple TCP connect() probe limited to your own subnet and only runs when you trigger it.

### Prerequisites

| Requirement | Version |
|---|---|
| [Node.js](https://nodejs.org) | 18 or newer |
| [Rust](https://rustup.rs) | 1.77 or newer (stable toolchain) |
| [Tauri CLI](https://tauri.app) | v2 |

**Platform-specific system dependencies** (required by Tauri 2 to build the native shell):

- **Windows**: [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) and [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/) (preinstalled on Windows 11)
- **macOS**: Xcode Command Line Tools (`xcode-select --install`)
- **Linux**: `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `build-essential`, `curl`, `wget`, `file`

### Installation

1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Install Node.js dependencies:
   ```bash
   npm install
   ```
3. Install the Tauri CLI (if not already available):
   ```bash
   npm install -g @tauri-apps/cli
   ```
4. On Linux, install the system dependencies listed above, e.g. on Debian/Ubuntu:
   ```bash
   sudo apt update
   sudo apt install -y libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev build-essential curl wget file
   ```

### Running in development

```bash
npm run tauri dev
```

This starts the Vite dev server and launches the Tauri window with hot reload for the frontend.

### Building a production bundle

```bash
npm run tauri build
```

The installer/executable will be generated under `src-tauri/target/release/bundle/`.

### Project structure

```
aegis/
├── src/                     # React + TypeScript frontend
│   ├── components/          # Sidebar, Topbar, Command Palette, Network Graph, etc.
│   ├── pages/                # Dashboard, Network Map, Devices, Timeline, Flows, Logs,
│   │                          #   Incidents, Rules Studio, File Integrity, AI Analyst, Vault, Settings
│   ├── i18n/                 # en / fa / zh translation files
│   ├── store/                 # Zustand app state (theme, language, toasts)
│   ├── lib/                   # Typed Tauri invoke() wrapper
│   └── styles/                 # Theme CSS variables + global styles
└── src-tauri/                # Rust backend
    ├── src/
    │   ├── network/           # Device discovery, port scanning, flow sampling, topology
    │   ├── detection/         # Port-scan / spike / DNS / auth-failure analyzers
    │   ├── ai/                  # Tool-based AI Security Analyst
    │   ├── storage/             # SQLite schema, repository, encrypted vault
    │   ├── commands/            # Tauri command handlers
    │   ├── fim.rs                # File Integrity Monitoring
    │   └── main.rs                # App entry point & background tasks
    └── Cargo.toml
```

### License

This project is provided as-is for portfolio and personal/educational use. Review and adapt licensing terms before any commercial use.

---

## فارسی

### معرفی

AEGIS یک پلتفرم دسکتاپ دفاع سایبری و هوش شبکه است که به‌صورت **Local-First** کار می‌کند. این برنامه دستگاه‌های موجود در **شبکه مجاز شخصی شما** را شناسایی و نقشه‌برداری می‌کند، اتصالات فعال را نمونه‌برداری می‌کند، یک موتور Detection سبک (اسکن پورت، جهش اتصال، ناهنجاری DNS، شکست احراز هویت) اجرا می‌کند، هشدار تولید می‌کند که می‌توانید به Incident تبدیل کنید، یکپارچگی فایل‌ها را پایش می‌کند و شامل یک **تحلیل‌گر امنیتی هوش مصنوعی کاملاً Tool-Based و فقط-خواندنی** است که فعالیت‌ها را توضیح و همبسته می‌کند بدون آنکه هرگز خودش اقدامی انجام دهد.

تمام بخش Backend با Rust (Tokio، Rusqlite/SQLite، Rayon، Rustls، Argon2/AES-GCM) و در بستر Tauri 2 نوشته شده و رابط کاربری با React + TypeScript و ظاهری Dark Glassmorphism در سبک SOC طراحی شده است.

### ویژگی‌ها

- **نقشه تعاملی شبکه** — گراف زنده و انیمیشنی از روتر، کامپیوتر، سرور، موبایل، IoT و پرینتر در شبکه شما
- **فهرست دستگاه‌ها** — تشخیص Hostname/MAC/Vendor، امتیاز ریسک هر دستگاه، اسکن پورت درخواستی
- **جدول زمانی امنیتی** — لاگ رویدادهای قابل جستجو و فیلتر با همبستگی به کمک هوش مصنوعی
- **کاوشگر بسته/Flow** — نمونه‌برداری زنده از اتصالات فعال TCP/UDP
- **مرکز هوشمند لاگ** — جستجوی سریع متن کامل با قابلیت ذخیره جستجوها
- **فضای کاری پاسخ به Incident** — تبدیل هشدار به Incident، پیگیری وضعیت، افزودن یادداشت بررسی
- **استودیوی قوانین** — سازنده Visual برای قوانین Detection (اسکن پورت، جهش اتصال، ناهنجاری DNS، شکست احراز هویت، سفارشی)
- **نظارت بر یکپارچگی فایل** — Hash پایه SHA-256 و تشخیص تغییرات برای فایل‌های انتخابی شما
- **تحلیل‌گر امنیتی هوش مصنوعی** — کاملاً فقط‌خواندنی و Tool-Based؛ هرگز دستور صادر نمی‌کند، ترافیک را مسدود نمی‌کند و اقدام خودکار انجام نمی‌دهد
- **صندوق امن** — رمزنگاری AES-256-GCM با اشتقاق کلید Argon2id؛ هیچ Secretی به‌صورت Plain Text ذخیره نمی‌شود
- **شش تم آماده** — تیره پریمیوم، روشن، پیش‌فرض ویندوز ۱۱، قرمز، آبی و AMOLED
- **سه زبان** — English، فارسی (کاملاً راست‌چین)، 中文 — قابل تغییر در حین اجرا
- **پروفایلر عملکرد** — نمایش زنده CPU، RAM و توان عملیاتی شبکه در داشبورد

### محدوده و ایمنی

AEGIS به‌صورت طراحی یک ابزار **دفاعی** است. این برنامه هرگز Exploitation، سرقت Credential، استقرار Malware، Persistence یا هرگونه اقدام خودکار/تهاجمی انجام نمی‌دهد. شناسایی دستگاه‌ها از طریق Ping استاندارد ICMP و خواندن جدول ARP سیستم‌عامل انجام می‌شود؛ اسکن پورت یک تلاش ساده TCP connect() است که فقط در محدوده Subnet خودتان و فقط با اجرای دستی شما فعال می‌شود.

### پیش‌نیازها

| نیازمندی | نسخه |
|---|---|
| [Node.js](https://nodejs.org) | ۱۸ یا بالاتر |
| [Rust](https://rustup.rs) | ۱٫۷۷ یا بالاتر (Stable) |
| [Tauri CLI](https://tauri.app) | نسخه ۲ |

**وابستگی‌های سیستمی مخصوص هر پلتفرم** (برای Build کردن Tauri 2 لازم است):

- **ویندوز**: [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) و [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/) (در ویندوز ۱۱ از پیش نصب است)
- **macOS**: Xcode Command Line Tools (`xcode-select --install`)
- **لینوکس**: `libwebkit2gtk-4.1-dev`، `libgtk-3-dev`، `libayatana-appindicator3-dev`، `librsvg2-dev`، `build-essential`، `curl`، `wget`، `file`

### نصب

۱. نصب Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
۲. نصب وابستگی‌های Node.js:
   ```bash
   npm install
   ```
۳. نصب Tauri CLI (در صورت نبود):
   ```bash
   npm install -g @tauri-apps/cli
   ```
۴. در لینوکس، وابستگی‌های سیستمی بالا را نصب کنید؛ مثلاً در Debian/Ubuntu:
   ```bash
   sudo apt update
   sudo apt install -y libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev build-essential curl wget file
   ```

### اجرا در حالت توسعه

```bash
npm run tauri dev
```

این دستور سرور توسعه Vite را اجرا کرده و پنجره Tauri را با Hot Reload برای Frontend باز می‌کند.

### ساخت نسخه نهایی (Production)

```bash
npm run tauri build
```

فایل نصب/اجرایی در مسیر `src-tauri/target/release/bundle/` ساخته می‌شود.

### ساختار پروژه

```
aegis/
├── src/                     # رابط کاربری React + TypeScript
│   ├── components/          # Sidebar، Topbar، Command Palette، Network Graph و غیره
│   ├── pages/                # داشبورد، نقشه شبکه، دستگاه‌ها، جدول زمانی، Flowها، لاگ‌ها،
│   │                          #   Incidentها، استودیوی قوانین، یکپارچگی فایل، تحلیل‌گر AI، صندوق، تنظیمات
│   ├── i18n/                 # فایل‌های ترجمه en / fa / zh
│   ├── store/                 # وضعیت برنامه با Zustand (تم، زبان، Toastها)
│   ├── lib/                   # Wrapper تایپ‌شده برای invoke() تائوری
│   └── styles/                 # متغیرهای CSS تم‌ها و استایل‌های سراسری
└── src-tauri/                # بک‌اند Rust
    ├── src/
    │   ├── network/           # شناسایی دستگاه، اسکن پورت، نمونه‌برداری Flow، Topology
    │   ├── detection/         # تحلیل‌گرهای اسکن پورت / جهش / DNS / شکست احراز هویت
    │   ├── ai/                  # تحلیل‌گر امنیتی هوش مصنوعی Tool-Based
    │   ├── storage/             # اسکیمای SQLite، Repository، صندوق رمزنگاری‌شده
    │   ├── commands/            # هندلرهای Command تائوری
    │   ├── fim.rs                # نظارت بر یکپارچگی فایل
    │   └── main.rs                # نقطه ورود برنامه و Taskهای پس‌زمینه
    └── Cargo.toml
```

### مجوز استفاده

این پروژه به‌صورت "همان‌گونه که هست" برای استفاده Portfolio و شخصی/آموزشی ارائه شده است. پیش از هرگونه استفاده تجاری، شرایط مجوز را بررسی و تطبیق دهید.

---

## 中文

### 概述

AEGIS 是一款 **本地优先（Local-First）** 的桌面网络防御与情报分析平台。它可以发现并绘制 **您自己授权的本地网络** 中的设备拓扑图，采样活动连接，运行轻量级检测引擎（端口扫描、连接激增、DNS 异常、身份验证失败等启发式规则），生成可升级为事件的警报，监控文件完整性，并内置一个完全 **基于工具、只读** 的 AI 安全分析师，用于解释和关联活动，但绝不会自行采取任何行动。

整个后端使用 Rust 编写（Tokio、Rusqlite/SQLite、Rayon、Rustls、Argon2/AES-GCM），运行在 Tauri 2 容器中；前端使用 React + TypeScript，界面风格为深色玻璃拟态（Glassmorphism）SOC 控制台。

### 功能特性

- **交互式网络地图** — 实时动态拓扑图，展示局域网中的路由器、计算机、服务器、移动设备、物联网设备和打印机
- **设备清单** — 主机名/MAC/厂商识别、每台设备的风险评分、按需端口扫描
- **安全时间线** — 可搜索、可筛选的事件日志，并支持 AI 辅助关联分析
- **数据包 / 流量浏览器** — 实时采样活动的 TCP/UDP 连接
- **日志智能中心** — 快速全文搜索，支持保存搜索条件
- **事件响应工作区** — 将警报转换为事件、跟踪状态、添加调查备注
- **规则工作室** — 可视化检测规则构建器（端口扫描、连接激增、DNS 异常、身份验证失败、自定义）
- **文件完整性监控** — 对您选择监控的文件进行 SHA-256 基线哈希与变更检测
- **AI 安全分析师** — 严格只读、基于工具的关联与解释引擎；绝不下达指令、拦截流量或自主采取行动
- **安全保险库** — AES-256-GCM 加密配合 Argon2id 密钥派生；任何机密信息都不会以明文存储
- **六种内置主题** — 深色高级版、浅色、Windows 11 默认、深红、青蓝、纯黑 AMOLED
- **三种语言** — English、فارسی（波斯语，完整从右至左布局）、中文 — 可在运行时切换
- **性能分析器** — 仪表盘中实时显示 CPU、内存及网络吞吐量

### 使用范围与安全声明

AEGIS 在设计上是一款 **纯防御性** 工具。它绝不执行漏洞利用、凭据窃取、恶意软件部署、持久化驻留，或任何形式的自动化/攻击性操作。设备发现使用标准 ICMP Ping 和操作系统 ARP 表读取；端口扫描仅为简单的 TCP connect() 探测，范围限定在您自己的子网内，且仅在您手动触发时运行。

### 环境要求

| 依赖项 | 版本 |
|---|---|
| [Node.js](https://nodejs.org) | 18 或更高 |
| [Rust](https://rustup.rs) | 1.77 或更高（stable 工具链） |
| [Tauri CLI](https://tauri.app) | v2 |

**各平台所需的系统依赖**（Tauri 2 构建原生外壳所必需）：

- **Windows**：[Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) 与 [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/)（Windows 11 已预装）
- **macOS**：Xcode 命令行工具（`xcode-select --install`）
- **Linux**：`libwebkit2gtk-4.1-dev`、`libgtk-3-dev`、`libayatana-appindicator3-dev`、`librsvg2-dev`、`build-essential`、`curl`、`wget`、`file`

### 安装步骤

1. 安装 Rust：
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. 安装 Node.js 依赖：
   ```bash
   npm install
   ```
3. 安装 Tauri CLI（如尚未安装）：
   ```bash
   npm install -g @tauri-apps/cli
   ```
4. 在 Linux 上安装上述系统依赖，例如在 Debian/Ubuntu 上：
   ```bash
   sudo apt update
   sudo apt install -y libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev build-essential curl wget file
   ```

### 开发模式运行

```bash
npm run tauri dev
```

此命令将启动 Vite 开发服务器，并打开支持前端热重载的 Tauri 窗口。

### 构建生产版本

```bash
npm run tauri build
```

安装包/可执行文件将生成于 `src-tauri/target/release/bundle/` 目录下。

### 项目结构

```
aegis/
├── src/                     # React + TypeScript 前端
│   ├── components/          # 侧边栏、顶部栏、命令面板、网络图等
│   ├── pages/                # 仪表盘、网络地图、设备、时间线、流量、日志、
│   │                          #   事件、规则工作室、文件完整性、AI 分析师、保险库、设置
│   ├── i18n/                 # en / fa / zh 翻译文件
│   ├── store/                 # Zustand 应用状态（主题、语言、通知）
│   ├── lib/                   # 带类型的 Tauri invoke() 封装
│   └── styles/                 # 主题 CSS 变量与全局样式
└── src-tauri/                # Rust 后端
    ├── src/
    │   ├── network/           # 设备发现、端口扫描、流量采样、拓扑构建
    │   ├── detection/         # 端口扫描 / 激增 / DNS / 身份验证失败分析器
    │   ├── ai/                  # 基于工具的 AI 安全分析师
    │   ├── storage/             # SQLite 架构、数据仓库、加密保险库
    │   ├── commands/            # Tauri 命令处理程序
    │   ├── fim.rs                # 文件完整性监控
    │   └── main.rs                # 应用入口与后台任务
    └── Cargo.toml
```

### 许可

本项目按“原样”提供，供作品集展示及个人/教育用途使用。任何商业用途前，请审查并调整相应许可条款。
