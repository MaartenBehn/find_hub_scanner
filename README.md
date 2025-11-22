# Tauri BLE Example - 开发者快速开始指南

**中文** | **[English](README_EN.md)**

## 🎯 项目概述

这是一个基于 **Tauri + Vue 3 + TypeScript** 的跨平台蓝牙低能耗(BLE)应用示例。支持在 Windows、macOS、Linux 和 Android 等平台上进行蓝牙设备通信。

![Image](image.jpg)

## 📋 前置要求

| 组件 | 版本 | 说明 |
|------|------|------|
| Node.js | ≥ 18.0 | JavaScript 运行时 |
| Rust | Latest Stable | 后端开发语言 |
| Tauri CLI | ≥ 2.0 | Tauri 命令行工具 |
| Android SDK | API 21+ | 仅 Android 开发需要 |
| JDK | 11+ | 仅 Android 开发需要 |

---

## 🚀 快速开始（5 分钟）

### 1️⃣ 克隆项目

```bash
git clone https://github.com/your-repo/tauri-ble-example.git
cd tauri-ble-example
```

### 2️⃣ 配置 Node.js 环境

```powershell
# 检查 Node.js 版本
node --version
npm --version

# 如果没有安装，访问 https://nodejs.org/ 下载 LTS 版本
# 推荐使用 NVM 管理 Node.js 版本，访问 https://github.com/coreybutler/nvm-windows 下载 NVM
nvm install latest
nvm use latest
```

### 3️⃣ 配置 Rust 环境

```powershell
# 使用 rustup 安装 Rust（推荐）
# 访问 https://rustup.rs/ 或执行
irm https://rustup.rs | iex

# 验证安装
rustc --version
cargo --version

# 更新 Rust（推荐每周更新一次）
rustup update
```

### 4️⃣ 安装 Tauri CLI

```powershell
# 全局安装 Tauri CLI
cargo install tauri-cli

# 验证安装
cargo tauri --version
```

### 5️⃣ 安装项目依赖

```powershell
# 安装 npm 依赖
npm install
```

### 6️⃣ 开发调试

```powershell
# 开发模式运行 Tauri 应用
cargo tauri dev
```

> 应用将在开发模式下运行，支持热重载

---

## 🏗️ 编译与打包

### 本地开发编译

```powershell
# 仅编译前端
npm run build

# 编译桌面应用
cargo tauri build
```

编译产物位置：

- Windows: `src-tauri/target/release/`
- macOS: `src-tauri/target/release/`

### 🤖 Android 编译指南

#### 步骤 1：配置 Android 开发环境

```powershell
# 安装 JDK 11+（推荐）
# 访问 https://www.oracle.com/java/technologies/downloads/
# 或使用
choco install openjdk

# 验证
java -version
```

#### 步骤 2：安装 Android SDK

```powershell
# 下载 Android Studio
# https://developer.android.com/studio

# 或者使用命令行工具 (cmdline-tools)
# 设置 ANDROID_SDK_ROOT 环境变量
[Environment]::SetEnvironmentVariable("ANDROID_SDK_ROOT", "C:\Users\YourUsername\AppData\Local\Android\Sdk", [EnvironmentVariableTarget]::User)
[Environment]::SetEnvironmentVariable("ANDROID_HOME", "C:\Users\YourUsername\AppData\Local\Android\Sdk", [EnvironmentVariableTarget]::User)

# 刷新环境变量
$env:ANDROID_SDK_ROOT = [System.Environment]::GetEnvironmentVariable("ANDROID_SDK_ROOT","User")
$env:ANDROID_HOME = [System.Environment]::GetEnvironmentVariable("ANDROID_HOME","User")

# 验证
echo $env:ANDROID_SDK_ROOT
```

#### 步骤 3：安装 Android 工具链

```powershell
# 添加 Android 编译目标
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android

# 列出已安装目标
rustup target list | findstr "installed"
```

#### 步骤 4：生成应用签名密钥

> ⚠️ **安全提示**：应用签名密钥非常重要，请妥善保管！

**首次编译时创建密钥库（Keystore）：**

```powershell
# 使用 keytool 生成密钥库（需要 JDK）
keytool -genkey -v -keystore tauri-ble-example-keystore.jks -keyalg RSA -keysize 2048 -validity 10000 -alias upload

# 按照提示输入信息：
# - 密钥库密码（Keystore Password）
# - 密钥密码（Key Password）
# - 国家代码、城市等信息
```

**创建 `local.properties` 文件：**

在 `src-tauri/gen/android/` 目录下创建 `local.properties` 文件，内容如下：

```properties
# 签名配置（替换为你的实际值）
storePassword=your_keystore_password
keyPassword=your_key_password
keyAlias=upload # 注意，这里必须是upload，不能修改
storeFile=C:\\path\\to\\your\\tauri-ble-example-keystore.jks
```

**替换的值说明：**

- `your_keystore_password`: 创建密钥库时设置的密码
- `your_key_password`: 创建密钥时设置的密码
- `C:\\path\\to\\your\\tauri-ble-example-keystore.jks`: 密钥库文件的完整路径

#### 步骤 5：编译 Android APK

```powershell
# 编译 APK（带签名）
cargo tauri android build
```

APK 位置：`src-tauri/gen/android/app/build/outputs/apk/`

---

## 🔐 安全与密钥管理

### ⚠️ 重要警告

1. **永远不要泄露密钥信息**：
   - 🚫 不要将 `local.properties` 提交到 Git 仓库
   - 🚫 不要将密钥库文件（`.jks`）上传到公开仓库
   - 🚫 不要在代码注释或文档中暴露密钥
   - 🚫 不要分享你的签名密码给任何人

2. **密钥丢失处理**：
   - 如果密钥丢失或被盗，需要为应用重新生成新的签名密钥
   - 无法使用旧密钥重新签名已发布的应用
   - 建议在安全的位置保持备份

3. **环境变量方案**（推荐）：

```powershell
# 设置环境变量而不是硬编码到文件
[Environment]::SetEnvironmentVariable("KEYSTORE_PASSWORD", "your_password", [EnvironmentVariableTarget]::User)
[Environment]::SetEnvironmentVariable("KEY_PASSWORD", "your_password", [EnvironmentVariableTarget]::User)
[Environment]::SetEnvironmentVariable("KEYSTORE_PATH", "C:\path\to\keystore.jks", [EnvironmentVariableTarget]::User)

# 验证
$env:KEYSTORE_PASSWORD
```

### Git 安全配置

**在 `.gitignore` 中添加以下行（如果还未添加）：**

```gitignore
# Android 本地配置
src-tauri/gen/android/local.properties

# 密钥库文件
*.jks
*.keystore

# Gradle 文件
src-tauri/gen/android/.gradle/
src-tauri/gen/android/build/

# IDE 配置
.idea/
*.iml
```

### CI/CD 中的密钥保护

为保护 CI/CD 流程中的密钥，使用 GitHub Secrets 或类似的密钥管理服务

---

## 📦 项目结构

```bash
tauri-ble-example/
├── src/                          # Vue 前端代码
│   ├── App.vue                  # 主应用组件
│   ├── main.ts                  # 应用入口
│   └── index.css                # 样式文件
├── src-tauri/                   # Rust 后端代码
│   ├── gen/
│   │   ├── android/                # Android 相关文件
│   │   │   ├── local.properties    # 本地配置文件（不应提交到版本控制）
│   │   │   ├── ...
│   ├── src/
│   │   ├── lib.rs              # Rust 库文件
│   │   └── main.rs             # Rust 主文件
│   ├── Cargo.toml              # Rust 依赖配置
│   ├── tauri.conf.json         # Tauri 配置文件
│   └── capabilities/           # 权限配置
├── public/                      # 静态资源
├── package.json                 # npm 依赖
├── tsconfig.json               # TypeScript 配置
└── vite.config.ts              # Vite 配置
```

---

## 🔧 常见命令速查表

| 命令 | 说明 |
|------|------|
| `npm run dev` | 启动前端开发服务器 |
| `npm run build` | 编译前端资源 |
| `cargo tauri dev` | 开发模式运行桌面应用 |
| `cargo tauri build` | 编译打包桌面应用 |
| `cargo tauri android dev` | 开发模式运行安卓应用 |
| `cargo tauri android build` | 编译打包安卓应用 |

---

## 📱 蓝牙(BLE) 功能说明

本项目集成了 **tauri-plugin-blec** 插件，提供跨平台的蓝牙低能耗支持。

### 主要功能

- 📡 扫描蓝牙设备
- 🔗 连接/断开设备
- 📨 读写特征值
- 🔔 订阅通知

---

---

## 📚 相关资源

- 🦀 [Rust 官方文档](https://doc.rust-lang.org/)
- 🎨 [Vue 3 文档](https://vuejs.org/)
- 🚀 [Tauri 官方文档](https://tauri.app/)
- 📱 [Tauri 移动文档](https://tauri.app/v1/guides/getting-started/setup/mobile)
- 🔌 [tauri-plugin-blec](https://github.com/MnlPhlp/tauri-plugin-blec)
- 🤖 [Android 官方文档](https://developer.android.com/docs)

---

## 💡 开发建议

1. **使用 VS Code**：安装推荐扩展
   - Vue - Official
   - Tauri
   - rust-analyzer
   - TypeScript Vue Plugin

2. **调试技巧**：
   - 前端：使用 Vue DevTools
   - Rust：设置 `RUST_LOG=debug` 查看日志

3. **性能优化**：
   - 使用 `npm run build` 生成优化的生产版本
   - 利用 Vite 的代码分割功能

4. **版本管理**：
   - 定期运行 `rustup update`
   - 定期运行 `npm update`

---

## 📄 许可证

MIT License - 详见 LICENSE 文件
