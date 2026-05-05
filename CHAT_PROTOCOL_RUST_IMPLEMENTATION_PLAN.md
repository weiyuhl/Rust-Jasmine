# chat_protocol Rust 模块实现计划

## 目标

在 `rust-lib/jasmine-agent/src/chat_protocol/` 中实现聊天协议底层逻辑，替代 Kelivo 中对应的 Dart 代码。

**原则：** Dart 侧负责 HTTP 传输（dio 发请求、收流），Rust 侧负责协议解析和处理（请求构建、SSE 解析、工具调用提取）。

---

## 当前状态

```
✅ jasmine-agent 已重构完成（modal_provider + mcp_server + chat_protocol 骨架）
✅ FRB v2 绑定链路正常（flutter_rust_bridge_codegen generate 通过）
✅ cargo check + flutter analyze 零错误
❌ chat_protocol 仅有一个 greet demo 占位
❌ Kelivo 聊天协议层仍全部在 Dart 侧（7000+ 行）
```

---

## 对应 Kelivo 源码清单

| Dart 文件 | 行数 | 职责 | → Rust 对应 |
|----------|------|------|-----------|
| `providers/openai_common.dart` | 4096 | URL/Body/Headers 构建、SSE 解析、工具调用提取、图片处理 | `request/openai.rs` + `parse/openai_stream.rs` |
| `providers/claude_official.dart` | 917 | Claude URL/Body/Headers、SSE 解析、thinking 配置 | `request/claude.rs` + `parse/claude_stream.rs` |
| `chat_api_service.dart` | 1260 | `sendMessageStream()` 路由分发、`generateText()`、内容消毒 | 保留在 Dart（仅路由逻辑） |
| `chat_api_service_shims.dart` | 69 | 内部函数转发 shim | 无需移植 |
| `providers/openai_chat_completions.dart` | 109 | Chat Completions 流包装 | `request/openai.rs` |
| `providers/openai_responses.dart` | 70 | Responses API 包装 | `request/openai.rs` |
| `builtin_tools.dart` | 525 | 内建搜索工具（web_search 等） | `tool/builtin.rs` |
| `provider_request_headers.dart` | 19 | OpenRouter 默认头 | 已在 modal_provider 中实现 |

---

## Rust 模块结构

```
chat_protocol/
├── mod.rs                         # pub mod request; pub mod parse; pub mod message; pub mod tool;
├── request/                       # 请求构建
│   ├── mod.rs
│   ├── openai.rs                  # URL + Headers + Body (Chat Completions & Responses)
│   ├── openai_vendor.rs           # 供应商特化 (Azure/DeepSeek/SiliconFlow/Kimi/OpenRouter/Grok...)
│   ├── openai_images.rs           # DALL-E 图片生成 API
│   └── claude.rs                  # URL + Headers + Body + conversation loop (Messages API)
├── parse/                         # 响应解析
│   ├── mod.rs
│   ├── sse.rs                     # 通用 SSE 行解析: data: → JSON, [DONE] 检测
│   ├── openai_stream.rs           # OpenAI SSE 块解析: delta.content, tool_calls, reasoning, finish_reason
│   ├── claude_stream.rs           # Claude SSE 块解析: content_block_start/delta/stop, thinking, stop_reason
│   └── tool_calls.rs              # 工具调用聚合: 多个 delta → 完整 ToolCall
├── message/                       # 消息构建
│   ├── mod.rs
│   ├── builder.rs                 # messages 数组构建、system prompt 提取、拷贝/消毒
│   ├── image.rs                   # 图片/多模态处理: Markdown 解析、base64 编码、远程验证
│   └── sanitizer.rs               # Unicode 消毒、消息批量消毒
└── tool/                          # 工具处理
    ├── mod.rs
    ├── schema.rs                  # JSON Schema 规范化（const→enum, 去 $schema, anyOf 处理等）
    └── builtin.rs                 # 内建工具注入（web_search/code_interpreter/image_generation）
```

---

## 关键数据模型（必须实现）

来自 `chat_api_service.dart` 和 `lib/core/models/token_usage.dart`

| 模型 | 字段 | 说明 |
|------|------|------|
| `ChatStreamChunk` | content, reasoning?, isDone, totalTokens, usage?, toolCalls?, toolResults? | SSE 流输出的单个块 |
| `ToolCallInfo` | id, name, arguments, metadata? | 完整工具调用 |
| `ToolResultInfo` | id, name, arguments, content, metadata? | 工具调用结果 |
| `TokenUsage` | promptTokens, completionTokens, cachedTokens, totalTokens | 令牌用量（含 merge 逻辑） |

## 详细功能清单

### 1. request/openai.rs — OpenAI 兼容请求构建

来自 `openai_common.dart` + `openai_chat_completions.dart` + `openai_responses.dart`

| 函数 | 说明 |
|------|------|
| `build_openai_url(config, use_response_api)` → `String` | 拼接 API 端点 URL |
| `build_openai_headers(config, model_id)` → `HashMap` | Authorization Bearer + Content-Type + OpenRouter 头 |
| `build_openai_chat_body(model_id, messages, tools, stream, temp, max_tokens, thinking)` → `String` (JSON) | Chat Completions 请求体 |
| `build_openai_responses_body(model_id, messages, tools, stream, temp, thinking)` → `String` (JSON) | Responses API 请求体 |
| `build_openai_images_body(model_id, prompt, n, size, quality)` → `String` (JSON) | DALL-E 图片生成请求体 |

### 2. request/claude.rs — Claude API 请求构建

来自 `claude_official.dart`

| 函数 | 说明 |
|------|------|
| `build_claude_url(config)` → `String` | `{baseUrl}/messages` |
| `build_claude_headers(config)` → `HashMap` | x-api-key + anthropic-version + Content-Type |
| `build_claude_body(model_id, messages, tools, stream, temp, max_tokens, thinking_budget)` → `String` (JSON) | Messages API 请求体 |
| `claude_thinking_config(model_id, budget)` → `Option<Value>` | thinking: {type, budget_tokens} |
| `claude_output_config(model_id, budget)` → `Option<Value>` | output_config: {effort} |

### 3. parse/sse.rs — 通用 SSE 解析

来自 `openai_common.dart` 和 `claude_official.dart` 的 SSE 处理

| 函数 | 说明 |
|------|------|
| `parse_sse_line(line: &str)` → `Option<SseEvent>` | 解析一行 SSE 文本 → data/text/event 结构 |
| `is_sse_done(line: &str)` → `bool` | 检测 `data: [DONE]` |
| `extract_sse_json(data: &str)` → `Result<Value, String>` | 从 data 字段提取并解析 JSON |

### 4. parse/openai_stream.rs — OpenAI 流块解析

| 函数 | 说明 |
|------|------|
| `parse_openai_chunk(json: &Value)` → `OpenAIDelta` | 解析 choices[0].delta → {content, role, tool_calls, reasoning_content} |
| `extract_reasoning_content(delta: &Value)` → `Option<String>` | 提取 reasoning_content 文本（支持 Thinking 模型） |
| `extract_tool_call_delta(delta: &Value)` → `Option<ToolCallDelta>` | 提取 tool_calls[0].function.{name, arguments} |
| `is_openai_chunk_done(json: &Value)` → `bool` | 检测 finish_reason 不为 null |

### 5. parse/claude_stream.rs — Claude 流块解析

| 函数 | 说明 |
|------|------|
| `parse_claude_event(json: &Value)` → `ClaudeEvent` | 解析 event → content_block_start/delta/stop, message_start/delta/stop, ping |
| `extract_text_delta(event: &Value)` → `Option<String>` | content_block_delta.delta.text |
| `extract_thinking_delta(event: &Value)` → `Option<String>` | content_block_delta.delta.thinking |
| `extract_tool_use_delta(event: &Value)` → `Option<ToolUseDelta>` | content_block_start/delta → id, name, input |
| `is_claude_done(event: &Value)` → `bool` | 检测 message_delta.stop_reason != null |

### 6. parse/tool_calls.rs — 工具调用聚合

| 函数 | 说明 |
|------|------|
| `ToolCallAggregator::new()` | 创建聚合器 |
| `push_delta(&mut self, delta: ToolCallDelta)` | 添加一个 delta 块 |
| `current_calls(&self)` → `Vec<ToolCall>` | 获取当前完整的工具调用列表 |
| `reset(&mut self)` | 重置聚合器 |
| `has_incomplete_calls(&self)` → `bool` | 是否有未完成的调用 |

### 7. message/builder.rs — 消息构建

| 函数 | 说明 |
|------|------|
| `copy_chat_message(msg: &Value)` → `Value` | 复制并消毒单条消息（保留 role/content/tool_calls 等关键字段） |
| `extract_system_prompts(messages: &[Value])` → `(Option<String>, Vec<Value>)` | 分离 system prompt 和其余消息 |
| `build_user_message_with_images(text: &str, images: &[ImageRef])` → `Value` | 构建含图片的 user content（文本+base64 URL） |

### 8. tool/schema.rs — 工具 Schema 规范化

| 函数 | 说明 |
|------|------|
| `clean_tool_for_compatibility(tool: &Value)` → `Value` | 清理工具定义（const→enum, 去 $schema） |
| `to_responses_tools_format(tools: &[Value])` → `Vec<Value>` | OpenAI Responses API 扁平化工具格式 |
| `to_claude_tools_format(tools: &[Value])` → `Vec<Value>` | Claude 工具格式（input_schema 嵌套） |

### 9. tool/builtin.rs — 内建工具

| 函数 | 说明 |
|------|------|
| `is_builtin_search_model(model_id: &str)` → `bool` | 判断模型是否支持内建搜索 |
| `build_web_search_tool(config: &Value)` → `Option<Value>` | 构建 web_search / web_search_preview 工具定义 |

---

### 10. request/openai_images.rs — 图片生成请求

来自 `openai_common.dart` + `openai_images.dart`

| 函数 | 说明 |
|------|------|
| `is_openai_images_model(model_id: &str)` → `bool` | 判断模型是否应走 DALL-E 图片 API |
| `build_openai_images_url(config)` → `String` | `{baseUrl}/images/generations` |
| `build_openai_images_body(model_id, prompt, n, size, quality)` → `String` (JSON) | DALL-E 请求体 |

### 11. request/openai_vendor.rs — 供应商特化处理

来自 `openai_common.dart` 中 40+ 个供应商判断函数

| 函数 | 说明 |
|------|------|
| `vendor_kind(config, model_id)` → `OpenAIVendor` | 供应商分类枚举：Standard, Azure, DeepSeek, SiliconFlow, Mimo, LongCat, Kimi, OpenRouter, Grok, Zhipu, DashScope, ByteDance, XinLiu |
| `completion_tokens_key(vendor)` → `&str` | Azure/Mimo → `max_completion_tokens`, 其余 → `max_tokens` |
| `needs_reasoning_echo(vendor)` → `bool` | DeepSeek/Mimo/Kimi 需要回显 reasoning |
| `omit_sampling_params(vendor, model_id, is_reasoning)` → `bool` | 某些推理模型去掉 temperature/top_p |
| `sanitize_gpt5_sampling(model_id, body)` → `Value` | GPT-5 系列移除不支持的参数 |
| `normalize_kimi_body(model_id, body)` → `Value` | Kimi K2 系列 thinking 格式适配 |
| `apply_dashscope_search(body, overrides)` → `Value` | 阿里 DashScope 搜索注入 |
| `apply_grok_search(body, model_id)` → `Value` | Grok 搜索参数注入 |

### 12. parse/claude_stream.rs 补充 — Claude 完整事件处理

来自 `claude_official.dart` 后半部分（行 300-917）

| 函数 | 说明 |
|------|------|
| `parse_claude_content_block_start(json)` → `ClaudeBlockStart` | `content_block_start` 事件（含 tool_use id/name） |
| `parse_claude_content_block_delta(json)` → `ClaudeBlockDelta` | `content_block_delta.delta.{text, input_json, thinking, signature}` |
| `parse_claude_message_delta(json)` → `ClaudeMsgDelta` | `message_delta.delta.stop_reason`（流结束检测） |
| `extract_claude_usage(json)` → `Option<TokenUsage>` | `message_delta.usage` / `message_stop.usage` 提取令牌用量 |
| `claude_tool_use_to_openai_format(blocks)` → `Vec<ToolCallInfo>` | Claude `tool_use` → OpenAI `tool_calls` 格式转换 |

### 13. message/image.rs — 图片/多模态处理

来自 `openai_common.dart` 的图片处理函数

| 函数 | 说明 |
|------|------|
| `parse_text_and_images(raw, allow_remote, allow_local)` → `ParsedContent` | 解析 Markdown 图片标记 + `[image:]` 标记 |
| `encode_base64_file(path, with_prefix)` → `Result<String>` | 本地文件 → base64 data URL |
| `mime_from_path(path)` → `String` | 文件扩展名 → MIME 类型 |
| `is_valid_remote_image_url(url)` → `bool` | HEAD 请求验证远程图片可达性 |

### 14. message/sanitizer.rs — 内容消毒

来自 `chat_api_service.dart` 和 `unicode_sanitizer.dart`

| 函数 | 说明 |
|------|------|
| `sanitize_unicode(text)` → `String` | 移除/替换不安全的 Unicode 字符 |
| `sanitize_messages(messages)` → `Vec<Value>` | 批量消毒消息列表 |

### 15. utils/openai_model_compat.rs — OpenAI 模型兼容性

来自 `lib/core/utils/openai_model_compat.dart`

| 函数 | 说明 |
|------|------|
| `normalize_reasoning_effort(effort, model_id)` → `String` | 按模型版本规范化 reasoning effort 值（max/xhigh/high/medium/low） |
| `allows_sampling_params(model_id, effort)` → `bool` | GPT-5.2/5.4 特定 effort 下是否允许 temperature/top_p/logprobs |
| `supports_responses_api(model_id)` → `bool` | 模型是否支持 Responses API |
| `supports_chat_completions_api(model_id)` → `bool` | 模型是否支持 Chat Completions API |

### 16. tool/builtin.rs 补充 — 完整 BuiltInToolsHelper

来自 `builtin_tools.dart` 中的 `BuiltInToolsHelper` 类（100+ 静态方法）

| 函数 | 说明 |
|------|------|
| `is_openai_builtin_search_supported(model_id)` → `bool` | OpenAI Responses 内建搜索支持检测 |
| `is_dashscope_builtin_search_supported(model_id)` → `bool` | 阿里 DashScope 搜索支持 |
| `is_dashscope_provider(config)` → `bool` | 是否 DashScope 供应商 |
| `is_grok_model(model_id)` → `bool` | 是否 Grok 模型 |
| `is_claude_builtin_search_supported(model_id)` → `bool` | Claude 内建搜索支持 |
| `is_claude_dynamic_web_search(model_id)` → `bool` | Claude 动态 web search |
| `claude_builtin_search_tool_type(config, model_id)` → `String` | 返回 `web_search_20260209` 或 `web_search_20250305` |
| `dashscope_search_options_from_override(override)` → `Value` | 从 modelOverrides 提取搜索选项 |

## FRB API 入口（约 50 个函数）

```
api/chat_protocol_api.rs
```

| 模块 | 函数数 | 典型函数签名 |
|------|--------|-------------|
| request/openai | 5 | `chat_build_openai_url`, `chat_build_openai_body` |
| request/openai_vendor | 3 | `chat_vendor_kind`, `chat_completion_tokens_key` |
| request/openai_images | 3 | `chat_is_images_model`, `chat_build_images_body` |
| request/claude | 5 | `chat_build_claude_url`, `chat_build_claude_headers`, `chat_claude_thinking_config` |
| parse/sse | 3 | `chat_parse_sse_line`, `chat_is_sse_done` |
| parse/openai_stream | 6 | `chat_parse_openai_chunk`, `chat_extract_openai_tool_calls`, `chat_extract_token_usage` |
| parse/claude_stream | 6 | `chat_parse_claude_event`, `chat_claude_to_openai_tool_calls` |
| parse/tool_calls | 2 | `chat_aggregate_tool_call`, `chat_finalize_tool_calls` |
| message/builder | 4 | `chat_copy_message`, `chat_extract_system_prompts` |
| message/image | 4 | `chat_parse_text_and_images`, `chat_encode_base64` |
| message/sanitizer | 2 | `chat_sanitize_unicode`, `chat_sanitize_messages` |
| tool/schema | 3 | `chat_clean_tool_schema`, `chat_to_claude_tools_format` |
| tool/builtin | 2 | `chat_is_builtin_search_supported`, `chat_build_web_search_tool` |

```
api/chat_protocol_api.rs
```

按模块分组：

| 模块 | 函数数 | 示例 |
|------|--------|------|
| request/openai | 5 | `chat_build_openai_url`, `chat_build_openai_body` |
| request/claude | 5 | `chat_build_claude_url`, `chat_build_claude_headers` |
| parse/sse | 3 | `chat_parse_sse_line`, `chat_parse_openai_chunk` |
| parse/claude | 4 | `chat_parse_claude_event`, `chat_extract_text_delta` |
| parse/tool_calls | 2 | `chat_aggregate_tool_call`, `chat_finalize_tool_calls` |
| message/builder | 3 | `chat_copy_message`, `chat_build_user_with_images` |
| tool/schema | 3 | `chat_clean_tool_schema`, `chat_to_claude_tools` |
| tool/builtin | 1 | `chat_is_builtin_search_supported` |

---

## 接入流程（按 FRB_RUST_SKELETON_AND_SUBMODULE_FLOW.md）

1. 在 `rust-lib/jasmine-agent/src/chat_protocol/` 下创建上述子目录和 `.rs` 文件
2. 在 `chat_protocol/mod.rs` 中注册 `pub mod request; pub mod parse; ...`
3. 创建 `api/chat_protocol_api.rs`（替换现有 demo）写入 26 个 `#[frb]` 函数
4. `flutter_rust_bridge_codegen generate --no-dart-fix`
5. `cargo check` + `flutter analyze`
6. Dart 侧对接：`chat_api_service.dart` 中调用 Rust 替代原有的 Body/Headers/Parse 逻辑

---

## 验证

```bash
# Rust
cd rust-lib/jasmine-agent && cargo check && cargo test

# Dart
flutter analyze

# .so + APK
cargo ndk -t arm64-v8a -t armeabi-v7a -o ../../android/app/src/main/jniLibs build --release
flutter build apk --release --target-platform android-arm64
```
