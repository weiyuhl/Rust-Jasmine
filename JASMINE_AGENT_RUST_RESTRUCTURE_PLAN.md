# Jasmine-agent Rust 结构调整计划

本文档只记录后续要做的事情。当前步骤不执行代码迁移、不运行生成、不改业务逻辑。

## 核心纠正

最外层 Rust/FRB 主模块不能再命名为模型供应商。

这个 Rust 部分承载的是一个完整 AI Agent：既包含模型供应商能力，也包含 MCP / 工具能力。因此主模块名称定为：

```text
Jasmine-agent
```

模型供应商不是主模块。MCP 服务也不是主模块。它们都是 `Jasmine-agent` 内部的能力模块。

如果后续落到 Rust/Cargo 技术标识，允许按工具链要求使用小写或下划线形式，例如 `jasmine-agent` / `jasmine_agent` / `jasmine_agent_rust`。但语义上必须保持：主模块是 `Jasmine-agent`，不是模型供应商主模块。

## 总目标

在不新增第二个 Rust crate、不新增第二个 flutter_rust_bridge 出口、不生成第二个 native library 的前提下，把当前 Rust bridge 从“模型供应商主模块”调整为“Jasmine-agent 主模块”。

后续目标结构语义：

```text
Jasmine-agent
  api/              # 唯一 FRB 暴露层
  modal_provider/   # 模型供应商能力模块
  mcp_server/       # MCP / 工具能力模块
```

## 第一阶段：模型供应商命名统一加 `modal` 前缀

用户明确要求模型供应商前缀使用 `modal`。后续实现时按这个前缀执行，不自动改成其他拼写。

需要完整修改，不能只改某一个 import 或某一个文件名。范围包括：

- Rust FRB 暴露模块文件：
  - `src/api/provider_api.rs` 调整为 `src/api/modal_provider_api.rs`
  - `src/api/mod.rs` 同步暴露 `modal_provider_api`
- Rust FRB 暴露函数：
  - `list_models` 改为 `modal_provider_list_models`
  - `test_connection` 改为 `modal_provider_test_connection`
  - `resolve_api_model_id` 改为 `modal_provider_resolve_api_model_id`
  - `classify_provider_kind` 改为 `modal_provider_classify_provider_kind`
  - `default_base_url` 改为 `modal_provider_default_base_url`
  - `get_provider_default_headers` 改为 `modal_provider_get_provider_default_headers`
  - `validate_provider_config` 改为 `modal_provider_validate_provider_config`
  - `create_default_config` 改为 `modal_provider_create_default_config`
- Dart 生成入口：
  - 生成文件应变成 `lib/src/rust/api/modal_provider_api.dart`
  - 旧的 `lib/src/rust/api/provider_api.dart` 不应保留
- Dart 调用侧：
  - `lib/core/providers/model_provider.dart` 不再 import `provider_api.dart`
  - 调用名同步为 `modalProviderListModels`、`modalProviderTestConnection` 等生成后的新 API
- 生成文件：
  - 重新执行 `flutter_rust_bridge_codegen generate --no-dart-fix`
  - 同步 `frb_generated.rs` 与 `lib/src/rust/frb_generated*.dart`
- 残留检查：
  - 清理 `provider_api`
  - 清理 `crateApiProviderApi`
  - 清理 `crate__api__provider_api`

## 第二阶段：主模块改为 Jasmine-agent

当前计划中的主模块名不能继续表达成模型供应商主模块。

后续实现时要把 FRB 主入口、Rust 主 crate / 目录、构建胶水中的语义统一到 `Jasmine-agent`：

- `flutter_rust_bridge.yaml` 的 `rust_root` 不应继续表达为模型供应商主模块
- `rust_builder` 平台构建脚本中的 `manifestDir` / `libname` 需要和新的主模块语义匹配
- `Cargo.toml` 的 package / library 命名需要和新的主模块语义匹配
- Dart 侧 `RustLib.init()` 保持只有一套初始化入口
- native library 仍然只能有一个

如果技术文件名必须使用 Rust 兼容写法，优先使用：

```text
jasmine-agent
jasmine_agent
jasmine_agent_rust
```

但文档和架构描述必须称其为 `Jasmine-agent`。

## 第三阶段：能力模块平级

在 `Jasmine-agent` 主模块内部，模型供应商和 MCP 服务是平级能力模块。

目标目录语义：

```text
Jasmine-agent/
  src/
    lib.rs
    frb_generated.rs

    api/
      mod.rs
      modal_provider_api.rs
      mcp_api.rs

    modal_provider/
      mod.rs
      models/
      providers/
      utils/

    mcp_server/
      mod.rs
      models/
      protocol/
      server/
      transport/
```

调整后，`src/` 根目录不应再直接放模型供应商内部业务目录：

- 不再保留 `src/models/`
- 不再保留 `src/providers/`
- 不再保留 `src/utils/`

`src/lib.rs` 应表达清楚：

```rust
pub mod api;
mod frb_generated;
pub mod modal_provider;
pub mod mcp_server;
```

## 不做的事情

- 不把模型供应商作为主模块
- 不把 MCP 服务作为主模块
- 不创建 `rust-lib/model-provider-core`
- 不创建独立 `rust-lib/mcp-server-rust`
- 不创建第二套 flutter_rust_bridge
- 不新增第二个 `rust_builder`
- 不新增第二个 `.so` / `.dll` / `.a`
- 不修改 Flutter UI
- 不手动编辑 `.dart_tool/**` 或 `build/**`
- 不把 MCP 业务塞进模型供应商目录
- 不把模型供应商业务继续直接摊在 `src/` 根目录

## 执行顺序

1. 确认 `git status --short`，确保工作区基线明确。
2. 先完成 `modal` 前缀命名调整。
3. 再把主模块语义从模型供应商改为 `Jasmine-agent`。
4. 再整理 `modal_provider/` 与 `mcp_server/` 平级能力模块。
5. 更新所有 Rust `use crate::...` 路径。
6. 更新 `flutter_rust_bridge.yaml` 和 `rust_builder` 中指向旧主模块名的配置。
7. 运行 `flutter_rust_bridge_codegen generate --no-dart-fix`。
8. 删除过期生成文件和旧 Rust API 文件。
9. 格式化改动文件。
10. 检索旧命名、旧目录、旧主模块名残留。
11. 执行验证。

## 验证要求

至少执行：

```powershell
cargo check
flutter_rust_bridge_codegen generate --no-dart-fix
dart format lib/core/providers/model_provider.dart lib/src/rust
flutter analyze
```

推荐补充执行：

```powershell
flutter test test\desktop_provider_grouping_compile_test.dart test\provider_grouping_logic_test.dart
```

若执行全量测试：

```powershell
flutter test
```

如果全量测试失败，需要明确记录失败用例、错误原因，以及是否与本次结构调整相关。

## 完成标准

- 主模块架构名称是 `Jasmine-agent`。
- 模型供应商只是 `Jasmine-agent` 内部的 `modal_provider` 能力模块。
- MCP 服务只是 `Jasmine-agent` 内部的 `mcp_server` 能力模块。
- `src/api/modal_provider_api.rs` 是模型供应商唯一 FRB 暴露文件。
- `src/api/mcp_api.rs` 是 MCP 唯一 FRB 暴露文件。
- 没有 `provider_api` 旧入口残留。
- 没有模型供应商作为主模块的语义残留。
- 没有额外 Rust crate。
- 没有第二个 native library 构建入口。
