part of '../chat_api_service.dart';

Map<String, dynamic> _copyChatCompletionMessage(Map<String, dynamic> m) {
  try {
    final resultJson = rust_chat.chatCopyMessage(msgJson: jsonEncode(m));
    final result = jsonDecode(resultJson) as Map<String, dynamic>;
    return result;
  } catch (_) {
    // Fallback for Rust panic — keep basic fields
    return {
      'role': m['role'] ?? 'user',
      'content': m['content'] ?? '',
    };
  }
}

List<Map<String, dynamic>> _cleanToolsForCompatibility(
  List<Map<String, dynamic>> tools,
) {
  try {
    final resultJson = rust_chat.chatCleanToolSchema(
      toolJson: jsonEncode(tools),
    );
    final result = (jsonDecode(resultJson) as List)
        .map((e) => e as Map<String, dynamic>)
        .toList();
    return result;
  } catch (_) {
    // Fallback: clean manually
    return tools.map((tool) {
      final result = Map<String, dynamic>.from(tool);
      final fn = result['function'];
      if (fn is Map) {
        final fnMap = Map<String, dynamic>.from(fn);
        final params = fnMap['parameters'];
        if (params is Map) {
          fnMap['parameters'] = _cleanSchemaForGemini(
            Map<String, dynamic>.from(params),
          );
        }
        result['function'] = fnMap;
      }
      return result;
    }).toList();
  }
}

Stream<ChatStreamChunk> _sendOpenAIChatCompletionsStream(
  http.Client client,
  ProviderConfig config,
  String modelId,
  List<Map<String, dynamic>> messages, {
  List<String>? userImagePaths,
  int? thinkingBudget,
  double? temperature,
  double? topP,
  int? maxTokens,
  List<Map<String, dynamic>>? tools,
  Future<String> Function(String, Map<String, dynamic>)? onToolCall,
  Map<String, String>? extraHeaders,
  Map<String, dynamic>? extraBody,
  bool stream = true,
}) {
  final cfg = config.copyWith(useResponseApi: false);
  return _sendOpenAIStream(
    client,
    cfg,
    modelId,
    messages,
    userImagePaths: userImagePaths,
    thinkingBudget: thinkingBudget,
    temperature: temperature,
    topP: topP,
    maxTokens: maxTokens,
    tools: tools,
    onToolCall: onToolCall,
    extraHeaders: extraHeaders,
    extraBody: extraBody,
    stream: stream,
  );
}
