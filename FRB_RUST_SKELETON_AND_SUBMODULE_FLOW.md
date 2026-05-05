# flutter_rust_bridge v2 子模块接入流程

## 目的

记录当前项目 `jasmine-agent` 内部新增 Rust 子模块并通过 `flutter_rust_bridge v2` 暴露给 Flutter 的标准流程。

以后新增 Rust 能力时按这个流程执行，不要手写 FRB 生成文件。

## 核心原则

- `flutter_rust_bridge_codegen integrate` 是**一次性项目初始化命令**，不可在已集成项目中重复使用。
- 已集成项目新增模块 = 手动创建目录 → 写 Rust 源码 → 注册模块 → 跑 `generate`。
- 唯一需要 FRB 生成的文件只有两类：`lib/src/rust/` (Dart) 和 `src/frb_generated.rs` (Rust)，均由 `generate` 命令自动产出。

## 当前项目已有结构

```
rust-lib/jasmine-agent/src/
  lib.rs                           # 顶层: pub mod <各子模块>;
  frb_generated.rs                 # 自动生成, 不可手动编辑
  api/
    mod.rs                         # pub mod <module_name>_api;
    modal_provider_api.rs          # 模型供应商 FRB 入口
    mcp_api.rs                     # MCP FRB 入口
    chat_protocol_api.rs           # 聊天协议 FRB 入口 (骨架)
  modal_provider/                  # 模型供应商业务模块
    mod.rs
    models/ providers/ utils/
    service.rs
  mcp_server/                      # MCP 协议业务模块
    mod.rs
    models/ protocol/ server/ transport/
  chat_protocol/                   # 聊天协议 (骨架)
    mod.rs
    simple.rs
```

## 新增子模块标准流程

### Step 1: 创建业务模块目录

在 `rust-lib/jasmine-agent/src/` 下手动创建：

```
<module_name>/
  mod.rs
```

目录名使用 Rust 合法的下划线命名（如 `some_module`，不是 `some-module`）。

### Step 2: 写业务逻辑源码

在 `<module_name>/` 下按需创建子目录和文件。`mod.rs` 中用 `pub mod xxx;` 导出。

```rust
// <module_name>/mod.rs
pub mod sub_a;
pub mod sub_b;
```

### Step 3: 创建 FRB API 入口

```rust
// api/<module_name>_api.rs
use crate::<module_name>::...;

#[flutter_rust_bridge::frb(sync)]
pub fn <module_name>_do_something(param: String) -> Result<String, String> {
    // 委托到业务模块
    crate::<module_name>::do_something(&param)
}
```

**规则：**
- `#[flutter_rust_bridge::frb]` 只放在 `api/<module_name>_api.rs` 中，不要放在业务模块里
- 纯计算函数加 `#[frb(sync)]`，有 I/O 的不加（默认异步）
- 函数名加模块前缀避免冲突
- 不要重复放 `#[frb(init)]`；项目已有统一初始化入口

### Step 4: 注册子模块

修改 `lib.rs`：
```rust
pub mod <module_name>;
```

修改 `api/mod.rs`：
```rust
pub mod <module_name>_api;
```

### Step 5: 生成 FRB 绑定

在工作区根目录运行：

```powershell
flutter_rust_bridge_codegen generate --no-dart-fix
```

此命令自动更新：
- `lib/src/rust/` — Dart 侧绑定
- `rust-lib/jasmine-agent/src/frb_generated.rs` — Rust 侧胶水

**这两个路径的文件绝不可手动编辑。**

## 验证命令

Rust 侧：

```powershell
cd rust-lib/jasmine-agent
cargo check
```

Flutter 侧：

```powershell
flutter analyze
```

如果影响 Dart 调用链，再运行：

```powershell
flutter test
```

## 交叉编译 & APK

新增模块后重新编译 .so 和 APK：

```powershell
cd rust-lib/jasmine-agent
cargo ndk -t arm64-v8a -t armeabi-v7a -o ../../android/app/src/main/jniLibs build --release

cd ../..
flutter build apk --release --target-platform android-arm64
```


# 重新生成 Rust 绑定并构建 Android APK

该计划详细说明了如何使用 `flutter_rust_bridge_codegen` (v2) 重新生成 Rust 与 Dart 的绑定，修复两端由于接口变化产生的兼容性问题，并使用跨平台编译机制打包 Android APK。

## User Review Required

> [!WARNING]
> 在执行前请确认：
> 1. 生成新绑定后可能会破坏原有的 Dart API 调用（如果有不兼容更改），我将自动运行分析并修复所有破损的接口。
> 2. Android 编译由于带有 Rust 的 NDK 交叉编译（通过 `cargokit` 机制），可能会比较耗时，我会使用 `--split-per-abi` 参数编出针对各架构的瘦 APK。

## Proposed Changes

### 1. 重新生成绑定
将使用 `flutter_rust_bridge_codegen generate` 重新生成位于 `lib/src/rust` 以及 `rust-lib/jasmine-agent/src/` 中的相关胶水代码。

### 2. 检查与修复接口 (Dart / Rust)
- 运行 `flutter analyze` 检查新绑定生成后 Dart 层是否报错。
- 运行 `cargo check` 检查 Rust 层是否报错。
- 如果存在新旧 API 签名的不兼容问题，我将逐一修复它们。

### 3. 构建发布版 APK
- 调用 `flutter build apk --release --split-per-abi`。
- 因为项目使用了 `rust_builder` (内部集成 `cargokit`)，Flutter 构建工具链会自动调用 Rust/Cargo 进行交叉编译生成 `libjasmine_agent.so` 并打包进 APK。

## Verification Plan

### Automated Tests
- 执行 `flutter analyze` 确保无 Dart 警告。
- 观察 `flutter build apk` 的输出，确保 Rust `.so` 动态库被成功编译并打包。

### Manual Verification
- 测试生成的 APK，确保 Flutter UI 能正常调用 Rust 逻辑并且不再报 `Failed to load dynamic library` 错误。
