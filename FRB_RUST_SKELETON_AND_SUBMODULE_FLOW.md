# flutter_rust_bridge v2 Rust 骨架与子模块流程

## 目的

记录当前项目中使用 `flutter_rust_bridge v2` 官方命令生成 Rust crate 骨架，并把生成结果整理为 `jasmine-agent` 内部 Rust 子模块的通用流程。

以后新增 Rust 能力时按这个流程执行，不要手写目录来冒充官方生成结果，也不要手写 FRB 生成文件。

## 生成 Rust 骨架

在工作区根目录运行：

```powershell
flutter_rust_bridge_codegen integrate --rust-crate-name <rust_crate_name> --rust-crate-dir <rust_crate_dir> --no-integration-test --no-dart-fix --no-dart-format
```

参数规则：

```text
<rust_crate_name>  Rust crate 名称，使用 Cargo 可接受的下划线命名
<rust_crate_dir>   生成目录，通常先放在 rust-lib/ 下
```

不要使用：

```powershell
flutter_rust_bridge_codegen create ...
```

`create` 会生成完整 Flutter 工程，不适合当前项目内新增 Rust 骨架。

也不要给这一步加：

```powershell
--no-write-lib
```

在当前使用的 `flutter_rust_bridge_codegen 2.12.0` 中，`--no-write-lib` 会跳过 Rust crate 目录本身，导致需要的 Rust 骨架也不生成。

## 当前项目的 integrate 行为

当前项目已经存在：

```text
rust_builder/
```

并且 `rust_builder/pubspec.yaml` 的包名是：

```yaml
name: jasmine_agent
```

因此 `integrate` 最后自动执行：

```powershell
flutter pub add <rust_crate_name> --path=rust_builder
```

时可能会因为包名不匹配报错。这个报错发生在 pub 依赖添加阶段；Rust crate 骨架通常已经生成出来。

`integrate` 还可能写入官方 demo 入口和测试文件。只保留需要的 Rust 骨架，清理本次命令新增的 demo/template 产物，不删除原项目已有文件。

如果忘记加 `--no-integration-test`，可能会额外生成：

```text
integration_test/
test_driver/
```

这些属于官方 demo 测试产物，不是 Rust 子模块需要的源码。

## 生成出的独立 crate 形态

官方生成的 Rust crate 通常是：

```text
<rust_crate_dir>/
  Cargo.toml
  Cargo.lock
  src/
    lib.rs
    frb_generated.rs
    api/
      mod.rs
      simple.rs
```

这是独立 crate 形态。并入 `jasmine-agent` 后，不能继续把它当成第二个 FRB 主入口或第二个 native library。

## jasmine-agent 子模块结构

当前 `jasmine-agent` 已有子模块结构是：

```text
rust-lib/jasmine-agent/src/
  lib.rs
  api/
    mod.rs
    <module_name>_api.rs
  <module_name>/
    mod.rs
    ...
```

已有模块示例：

```text
modal_provider/
mcp_server/
```

业务逻辑放在：

```text
rust-lib/jasmine-agent/src/<module_name>/
```

FRB 暴露入口放在：

```text
rust-lib/jasmine-agent/src/api/<module_name>_api.rs
```

## 并入子模块流程

### 1. 移动官方生成目录

先把官方生成出的目录移动到 `jasmine-agent/src` 下作为中间落点：

```text
rust-lib/jasmine-agent/src/<generated_dir_name>/
```

这一步只是移动生成出来的目录，不写业务代码。随后必须继续执行下面的“改成 Rust 合法模块名”和“去掉独立 crate 文件”步骤。

### 2. 改成 Rust 合法模块名

Rust 模块名不能使用横杠。目录名如果是：

```text
some-module-name
```

作为 Rust 模块时改成：

```text
some_module_name
```

最终目录应是：

```text
rust-lib/jasmine-agent/src/<module_name>/
```

### 3. 去掉独立 crate 文件

并入 `jasmine-agent` 后，不再保留独立 crate 的这些文件：

```text
Cargo.toml
Cargo.lock
src/lib.rs
src/frb_generated.rs
```

把可用源码整理到：

```text
rust-lib/jasmine-agent/src/<module_name>/
```

例如官方默认示例源码可整理为：

```text
rust-lib/jasmine-agent/src/<module_name>/simple.rs
```

并添加：

```text
rust-lib/jasmine-agent/src/<module_name>/mod.rs
```

示例：

```rust
pub mod simple;
```

### 4. 注册业务子模块

修改：

```text
rust-lib/jasmine-agent/src/lib.rs
```

添加：

```rust
pub mod <module_name>;
```

### 5. 添加统一 FRB API 入口

新增：

```text
rust-lib/jasmine-agent/src/api/<module_name>_api.rs
```

这里写 `#[flutter_rust_bridge::frb]` 暴露函数，并委托到业务模块：

```rust
use crate::<module_name>::...;
```

函数命名要带模块前缀：

```rust
#[flutter_rust_bridge::frb(sync)]
pub fn <module_name>_<function_name>(...) -> ... {
    ...
}
```

业务模块里不要重复放 `#[flutter_rust_bridge::frb(init)]` 初始化入口；当前项目已有统一 Rust 初始化。

### 6. 注册 API 模块

修改：

```text
rust-lib/jasmine-agent/src/api/mod.rs
```

添加：

```rust
pub mod <module_name>_api;
```

### 7. 生成 FRB 绑定

在工作区根目录运行：

```powershell
flutter_rust_bridge_codegen generate --no-dart-fix
```

生成文件会更新到：

```text
lib/src/rust/
rust-lib/jasmine-agent/src/frb_generated.rs
```

这些生成文件不要手写。

## 验证命令

Rust 侧：

```powershell
cd rust-lib/jasmine-agent
cargo fmt
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
