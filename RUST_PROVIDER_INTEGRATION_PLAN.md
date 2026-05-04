# Jasmine-agent 模型供应商接入计划

## 目标

用 `rust-lib/jasmine-agent/` 中的 `modal_provider` 能力模块**替换** Kelivo 现有的 Dart 模型供应商业务底层。`jasmine-agent` 是唯一 Rust/FRB 主模块，模型供应商不是主模块。

**Rust 覆盖范围：** OpenAI / Claude / DeepSeek / SiliconFlow / OpenRouter + 自定义供应商（仅 OpenAI、Claude 类型）。

**Dart 保留的供应商：** Google/Gemini（Vertex AI 等）、Grok、ByteDance、Zhipu AI、Aliyun、AIhubmix、Tensdaq、KelivoIN。——这些不在 Rust 范围内，继续用原有 Dart 实现。

---

## 当前完成状态

```
✅ rust-lib/jasmine-agent/src/api/modal_provider_api.rs 全部 8 个 Dart 可见 API 已实现
✅ flutter_rust_bridge_codegen generate --no-dart-fix 已执行
✅ lib/src/rust/api/modal_provider_api.dart 生成完毕
✅ cargo check / flutter analyze → 零错误零警告
✅ Step 1: main.dart → RustLib.init() 已添加
✅ Step 2: model_provider.dart → OpenAIProvider/ClaudeProvider 已删除，ProviderManager 已路由到 Rust
✅ Step 2: model_types.dart → ModelInfo.fromJson() 已添加
✅ Step 5: Android native library 构建名已统一为 `jasmine_agent`
✅ 旧 `libmodel_provider_rust.so` 本地产物不再保留
❌ Step 3: provider_request_headers.dart → 可选，非紧急
❌ Step 4: chat_api_service.dart → _apiModelId 可替换为 Rust modalProviderResolveApiModelId
```

---

## Rust API 清单（`lib/src/rust/api/modal_provider_api.dart`）

| Dart 函数 | 返回值 | 说明 |
|-----------|--------|------|
| `modalProviderResolveApiModelId(configJson, modelId)` | `Future<String>` | 从 modelOverrides 解析上游模型 ID |
| `modalProviderClassifyProviderKind(providerId, explicitType)` | `Future<String>` | 分类为 `"openai"` 或 `"claude"` |
| `modalProviderDefaultBaseUrl(providerId)` | `Future<String>` | 已知供应商默认 Base URL |
| `modalProviderGetProviderDefaultHeaders(configJson)` | `Future<String>` | 默认 HTTP 头（JSON map） |
| `modalProviderListModels(configJson, siliconflowFallbackKey)` | `Future<String>` | **从 API 获取**模型列表 JSON（已包含模型能力推断） |
| `modalProviderTestConnection(configJson, modelId, useStream, siliconflowFallbackKey)` | `Future<void>` | 连通性测试 |
| `modalProviderValidateProviderConfig(configJson)` | `Future<void>` | 配置合法性校验 |
| `modalProviderCreateDefaultConfig(providerId, displayName)` | `Future<String>` | 创建默认 ProviderConfig JSON |

---

## Rust 源码结构

```
rust-lib/jasmine-agent/src/
├── lib.rs
├── frb_generated.rs                # ⚠️ 自动生成，不可手动编辑
├── api/
│   ├── mod.rs
│   ├── modal_provider_api.rs       # 模型供应商 FRB 暴露的 8 个函数
│   └── mcp_api.rs                  # MCP FRB 暴露入口
├── modal_provider/
│   ├── mod.rs
│   ├── service.rs                  # 模型供应商业务层
│   ├── models/
│   │   ├── mod.rs
│   │   ├── provider_kind.rs        # ProviderKind 枚举（OpenAI / Claude）
│   │   ├── provider_config.rs      # ProviderConfig 结构体
│   │   └── model_info.rs           # ModelInfo / ModelType / Modality / ModelAbility
│   ├── providers/
│   │   ├── mod.rs
│   │   ├── openai_compat.rs        # OpenAI / DeepSeek / SiliconFlow / OpenRouter
│   │   └── claude.rs               # Claude Messages API
│   └── utils/
│       ├── mod.rs
│       └── model_inference.rs      # 正则推断（对齐 ModelRegistry.infer）
└── mcp_server/
    ├── mod.rs
    ├── mcp_api.rs
    ├── models/
    ├── protocol/
    ├── server/
    └── transport/
```

---

## ⚠️ 关键规则：每次修改 Rust 必须重新生成绑定

```
修改 Rust 源码 → cargo check → flutter_rust_bridge_codegen generate --no-dart-fix → flutter analyze
```

配置文件 `flutter_rust_bridge.yaml`：
```yaml
rust_input: crate::api
rust_root: rust-lib/jasmine-agent/
dart_output: lib/src/rust
```

**约束：**
- Rust 暴露给 Flutter 的代码写在 `api/` 目录中，新文件必须在 `api/mod.rs` 注册 `pub mod xxx;`
- `lib/src/rust/` 和 `rust-lib/jasmine-agent/src/frb_generated.rs` 是自动生成的，**绝不可手动编辑**
- 生成的 Rust 桥接文件中函数编号（func_id）由代码生成器自动分配，手动修改会导致 FFI 调用错乱

---

## ⚠️ 关键规则：旧 Dart 文件不能整体删除

`model_provider.dart` 中除了 OpenAI/Claude 相关代码外，**还包含**：

| 保留内容 | 原因 |
|---------|------|
| `GoogleProvider` | Google/Gemini 不在 Rust 范围内 |
| `ProviderManager.forConfig()` | Google 分支仍需走 Dart 原实现 |
| `ProviderManager.testConnection()` | Google 分支仍需 Dart 原实现 |
| `_Http.clientFor()` | Google 分支使用 |
| `ModelRegistry.infer()` | Google 分支及 UI 中可能直接调用 |
| `ProviderConfig.classify()` → `ProviderKind.google` | Google 分类 |
| `ProviderConfig._defaultBase()` | Google/Grok/ByteDance 等的 base URL |

**改造方式：** 从这些文件中**仅删除**已被 Rust 接管的供应商代码，保留其他供应商的代码不动。

---

## 接入步骤

### Step 1: `main.dart` 初始化 RustLib

文件：`lib/main.dart`

在 `runZoned` 内部、`runApp()` 之前加入：

```dart
import 'src/rust/frb_generated.dart';

// ... 在 WidgetsFlutterBinding.ensureInitialized() 之后
await RustLib.init();
```

### Step 2: 改造 `model_provider.dart`（局部删除+修改）

文件：`lib/core/providers/model_provider.dart`

**完全删除的类：**
- `OpenAIProvider` 类 → 已由 Rust `OpenAICompatProvider` 接管
- `ClaudeProvider` 类 → 已由 Rust `ClaudeProvider` 接管

**修改的函数：**

| 原函数 | 改动 |
|--------|------|
| `ProviderManager.listModels(cfg)` | 加 if: `kind == openai/claude` → 调用 Rust `modalProviderListModels()`；`kind == google` → 保留原逻辑 |
| `ProviderManager.testConnection(cfg, modelId)` | 加 if: `kind == openai/claude` → 调用 Rust `modalProviderTestConnection()`；`kind == google` → 保留原逻辑 |
| `ProviderManager.forConfig(cfg)` | 删除 `case openai:` 和 `case claude:`，只保留 `case google:` 和 default |
| `ProviderConfig.classify(key, explicitType)` | 保留（Google 分支需要）；OpenAI/Claude 分支也可通过 Rust `modalProviderClassifyProviderKind` 调用 |
| `ProviderConfig._defaultBase(key)` | 保留（Google/Grok/ByteDance 等需要） |

**保留不动：**
- `GoogleProvider` 类
- `ModelRegistry` 所有静态方法
- `_Http.clientFor()`

### Step 3: 改造 `provider_request_headers.dart`（可选）

文件：`lib/core/services/api/provider_request_headers.dart`

OpenRouter 标识头已在 Rust 侧 `openai_compat.rs` 中内置处理。Dart 侧 `providerDefaultHeaders()` 仅剩 Google 聊天路径仍需使用，且逻辑极简（4 行静态字符串），无需替换为 Rust 调用。

**结论：保留不动。** 后续如需统一，可将 `modalProviderGetProviderDefaultHeaders()` 作为备用。

### Step 4: 改造 `chat_api_service.dart`（可选）

文件：`lib/core/services/api/chat_api_service.dart`

`_apiModelId()` 函数可替换为 Rust `modalProviderResolveApiModelId()`。但 `chat_api_service.dart` 是 1200+ 行的流式聊天核心，本次仅替换模型供应商层，不适合改动它。

**结论：保留不动。** 后续如需深层集成 Rust，可从这里切入。

> **说明：** Steps 3-4 是可选优化的，因为 Steps 1-2 已完成核心接入路径——UI → ProviderManager → Rust API。provider_detail_page 等页面对 ProviderManager 的调用已自动走 Rust。

### Step 5: Rust 交叉编译 → Android native library

Rust 代码修改后必须重新编译 native library。Cargokit 在 `flutter build` 时自动运行；也可以手动编译验证，但本地二进制产物不作为源码改动保留。

#### 5a: 一次性安装 Rust 交叉编译目标

```bash
# Android ARM（真机必须）
rustup target add aarch64-linux-android armv7-linux-androideabi
# Android x86（模拟器，本机不需要可跳过）
# rustup target add x86_64-linux-android i686-linux-android
```

> 本机已安装：`aarch64-linux-android` `armv7-linux-androideabi`

#### 5b: 手动编译验证 Android native library（可选）

```bash
cd rust-lib/jasmine-agent

cargo ndk \
  -t arm64-v8a \
  -t armeabi-v7a \
  -o ../../android/app/src/main/jniLibs \
  build --release
```

命令执行后会在本地输出 native library，预期结构：

```
android/app/src/main/jniLibs/
├── arm64-v8a/libjasmine_agent.so
└── armeabi-v7a/libjasmine_agent.so
```

#### 5c: Flutter 构建入口

Flutter 侧通过 `pubspec.yaml` 中的本地依赖接入唯一构建入口：

```yaml
jasmine_agent:
  path: rust_builder
```

`rust_builder/` 里的 Android / iOS / macOS / Linux / Windows 构建胶水统一指向 `rust-lib/jasmine-agent` 和 `jasmine_agent`。

### Step 6: 验证

```bash
# Rust 侧检查
cd rust-lib/jasmine-agent && cargo check

# 可选：交叉编译 Android native library（每次 Rust 改动后）
cargo ndk -t arm64-v8a -t armeabi-v7a -o ../../android/app/src/main/jniLibs build --release

# 代码生成（每次 Rust 改动后）
cd ../.. && flutter_rust_bridge_codegen generate --no-dart-fix

# Dart 格式化
dart format lib/

# 完整分析
flutter analyze

# 相关测试
flutter test
```

**手动 Android 编译验证的预期本地输出：**
```
android/app/src/main/jniLibs/
├── arm64-v8a/libjasmine_agent.so   4.0M ✅
└── armeabi-v7a/libjasmine_agent.so 2.6M ✅
```

---

## 数据流

```
Flutter UI (ProviderConfig Dart 对象)
  │
  ├─ toJson() → JSON 字符串
  │
  ▼
Rust API (modal_provider_api.rs)
  │
  ├─ 反序列化 JSON → ProviderConfig 结构体
  ├─ 按 providerType 路由 → OpenAICompatProvider / ClaudeProvider
  ├─ HTTP 请求 (ureq)
  ├─ 模型推断 (model_inference.rs)
  │
  ▼
返回 JSON 字符串
  │
  ▼
Flutter (解析 JSON → Dart 对象)
```

---

## 供应商覆盖矩阵

| 供应商 | 类型 | listModels | testConnection | 归属 |
|--------|------|------------|---------------|------|
| OpenAI | OpenAI | Rust | Rust | ✅ 已覆盖 |
| DeepSeek | OpenAI | Rust | Rust | ✅ 已覆盖 |
| SiliconFlow | OpenAI | Rust | Rust | ✅ 已覆盖 |
| OpenRouter | OpenAI | Rust | Rust | ✅ 已覆盖 |
| Claude | Claude | Rust | Rust | ✅ 已覆盖 |
| 自定义(OpenAI类) | OpenAI | Rust | Rust | ✅ 已覆盖 |
| 自定义(Claude类) | Claude | Rust | Rust | ✅ 已覆盖 |
| Gemini/Google | Google | Dart 保留 | Dart 保留 | ❌ 不覆盖 |
| Grok | OpenAI | **可切 Rust** | **可切 Rust** | 🔀 待决定 |
| ByteDance | OpenAI | **可切 Rust** | **可切 Rust** | 🔀 待决定 |
| Zhipu AI | OpenAI | **可切 Rust** | **可切 Rust** | 🔀 待决定 |
| Aliyun | OpenAI | **可切 Rust** | **可切 Rust** | 🔀 待决定 |
| AIhubmix | OpenAI | **可切 Rust** | **可切 Rust** | 🔀 待决定 |
| Tensdaq | OpenAI | **可切 Rust** | **可切 Rust** | 🔀 待决定 |
| KelivoIN | OpenAI | **可切 Rust** | **可切 Rust** | 🔀 待决定 |

> 🔀 标记的供应商虽然是 OpenAI 兼容协议，但先保留在 Dart 侧不动，后续可按需逐一切换到 Rust。

---

## 恢复上下文检查清单

如果对话中断，按以下顺序恢复：

1. 阅读本文件
2. 阅读 `FLUTTER_RUST_BRIDGE_V2_接入说明.md`
3. 阅读 `CLAUDE.md`
4. `git status --short` 确认工作区状态
5. 确认 `cargo check` + `flutter analyze` 当前状态
6. **确认绑定是最新的**：如果 Rust 源码有未提交的改动，重新执行 `flutter_rust_bridge_codegen generate --no-dart-fix`
7. 确认当前接入了哪一步，从未完成的 Step 继续
