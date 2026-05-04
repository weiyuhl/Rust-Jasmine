# flutter_rust_bridge v2 接入说明

## 当前状态

本项目已接入 `flutter_rust_bridge` v2。

当前 Rust crate 已生成在：

```text
rust-lib/jasmine-agent/
```

当前桥接配置位于：

```text
flutter_rust_bridge.yaml
```

当前配置：

```yaml
rust_input: crate::api
rust_root: rust-lib/jasmine-agent/
dart_output: lib/src/rust
```

当前 Dart 侧生成代码位于：

```text
lib/src/rust/
```

当前 Flutter 侧本地依赖：

```yaml
jasmine_agent:
  path: rust_builder
flutter_rust_bridge: 2.12.0
```

`rust_builder/` 是 Flutter 构建 Rust 动态库/静态库所需的本地 FFI 插件胶水目录，通常不在这里写业务逻辑。

## 下一步接入方式

1. 在 `rust-lib/jasmine-agent/src/api/` 中编写需要暴露给 Flutter 的 Rust API。

2. 如果新增 Rust 暴露文件，需要在 `rust-lib/jasmine-agent/src/api/mod.rs` 中导出模块，例如：

```rust
pub mod modal_provider_api;
```

3. 修改 Rust API 后，在项目根目录运行代码生成：

```powershell
flutter_rust_bridge_codegen generate --no-dart-fix
```

开发时也可以使用官方推荐的 watch 模式：

```powershell
flutter_rust_bridge_codegen generate --watch
```

4. Dart 侧从 `lib/src/rust/api/` 下的生成文件调用 Rust API。生成文件不要手动修改。

5. 接入实际能力时，`Jasmine-agent` 作为唯一 Rust/FRB 主模块；模型供应商能力放在 `modal_provider/`，MCP / 工具能力放在 `mcp_server/`，Flutter 侧继续负责 UI、状态管理和本地化文本。

6. Rust 代码改动后至少确认：

```powershell
cargo check
```

运行目录：

```text
rust-lib/jasmine-agent/
```

涉及 Flutter 调用链后，再在项目根目录运行：

```powershell
flutter analyze
```

## 官方文档地址

- 官方 Quickstart: https://cjycode.com/flutter_rust_bridge/quickstart
- 官方 Guides: https://cjycode.com/flutter_rust_bridge/guides
- 官方目录结构说明: https://cjycode.com/flutter_rust_bridge/guides/miscellaneous/directory
- 官方 Dart 调用 Rust: https://cjycode.com/flutter_rust_bridge/guides/direction/dart-call-rust
- 官方函数映射说明: https://cjycode.com/flutter_rust_bridge/guides/functions/overview
- 官方并发与异步说明: https://cjycode.com/flutter_rust_bridge/guides/concurrency
- 官方集成概览: https://cjycode.com/flutter_rust_bridge/manual/integrate/overview
- 官方 create/integrate 命令说明: https://cjycode.com/flutter_rust_bridge/manual/integrate/builtin
- 官方 GitHub 仓库: https://github.com/fzyzcjy/flutter_rust_bridge
