import 'dart:async';
import 'dart:convert';
import 'dart:io' show Platform, Process;

import 'package:flutter/foundation.dart';
import 'package:mcp_client/mcp_client.dart' as mcp;
import '../services/mcp/kelivo_fetch/kelivo_fetch_server.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:uuid/uuid.dart';
import '../../../src/rust/api/mcp_api.dart' as rust_mcp;

/// Transport type: SSE, Streamable HTTP, and STDIO (desktop-only).
enum McpTransportType { sse, http, stdio, inmemory }

/// Connection status for an MCP server.
enum McpStatus { idle, connecting, connected, error }

class McpParamSpec {
  final String name;
  final bool required;
  final String? type;
  final dynamic defaultValue;

  McpParamSpec({
    required this.name,
    required this.required,
    this.type,
    this.defaultValue,
  });

  Map<String, dynamic> toJson() => {
    'name': name,
    'required': required,
    'type': type,
    'default': defaultValue,
  };

  factory McpParamSpec.fromJson(Map<String, dynamic> json) => McpParamSpec(
    name: json['name'] as String? ?? '',
    required: json['required'] as bool? ?? false,
    type: json['type'] as String?,
    defaultValue: json['default'],
  );
}

class McpToolConfig {
  final bool enabled;
  final String name;
  final String? description;
  final List<McpParamSpec> params;
  // Raw JSON schema for parameters, if provided by the server
  final Map<String, dynamic>? schema;
  /// Whether this tool requires user approval before execution.
  final bool needsApproval;

  McpToolConfig({
    required this.enabled,
    required this.name,
    this.description,
    this.params = const [],
    this.schema,
    this.needsApproval = false,
  });

  McpToolConfig copyWith({
    bool? enabled,
    String? name,
    String? description,
    List<McpParamSpec>? params,
    Map<String, dynamic>? schema,
    bool? needsApproval,
  }) => McpToolConfig(
    enabled: enabled ?? this.enabled,
    name: name ?? this.name,
    description: description ?? this.description,
    params: params ?? this.params,
    schema: schema ?? this.schema,
    needsApproval: needsApproval ?? this.needsApproval,
  );

  Map<String, dynamic> toJson() => {
    'enabled': enabled,
    'name': name,
    'description': description,
    'params': params.map((e) => e.toJson()).toList(),
    if (schema != null) 'schema': schema,
    if (needsApproval) 'needsApproval': true,
  };

  factory McpToolConfig.fromJson(Map<String, dynamic> json) => McpToolConfig(
    enabled: json['enabled'] as bool? ?? true,
    name: json['name'] as String? ?? '',
    description: json['description'] as String?,
    params:
        (json['params'] as List?)
            ?.map(
              (e) => McpParamSpec.fromJson((e as Map).cast<String, dynamic>()),
            )
            .toList() ??
        const [],
    schema: (json['schema'] is Map)
        ? (json['schema'] as Map).cast<String, dynamic>()
        : null,
    needsApproval: json['needsApproval'] as bool? ?? false,
  );
}

class McpServerConfig {
  final String id; // stable id
  final bool enabled;
  final String name;
  final McpTransportType transport;
  // For SSE/HTTP
  final String url; // SSE endpoint or HTTP base URL
  final List<McpToolConfig> tools;
  final Map<String, String> headers; // custom HTTP headers
  // For STDIO (desktop-only)
  final String? command;
  final List<String> args;
  final Map<String, String> env;
  final String? workingDirectory;

  McpServerConfig({
    required this.id,
    required this.enabled,
    required this.name,
    required this.transport,
    this.url = '',
    this.tools = const [],
    this.headers = const {},
    this.command,
    this.args = const [],
    this.env = const {},
    this.workingDirectory,
  });

  McpServerConfig copyWith({
    String? id,
    bool? enabled,
    String? name,
    McpTransportType? transport,
    String? url,
    List<McpToolConfig>? tools,
    Map<String, String>? headers,
    String? command,
    List<String>? args,
    Map<String, String>? env,
    String? workingDirectory,
    bool clearWorkingDirectory = false,
  }) => McpServerConfig(
    id: id ?? this.id,
    enabled: enabled ?? this.enabled,
    name: name ?? this.name,
    transport: transport ?? this.transport,
    url: url ?? this.url,
    tools: tools ?? this.tools,
    headers: headers ?? this.headers,
    command: command ?? this.command,
    args: args ?? this.args,
    env: env ?? this.env,
    workingDirectory: clearWorkingDirectory
        ? null
        : (workingDirectory ?? this.workingDirectory),
  );

  Map<String, dynamic> toJson() => {
    'id': id,
    'enabled': enabled,
    'name': name,
    'transport': transport.name,
    if (transport != McpTransportType.stdio &&
        transport != McpTransportType.inmemory)
      'url': url,
    'tools': tools.map((e) => e.toJson()).toList(),
    if (transport != McpTransportType.stdio &&
        transport != McpTransportType.inmemory)
      'headers': headers,
    if (transport == McpTransportType.stdio) 'command': command,
    if (transport == McpTransportType.stdio) 'args': args,
    if (transport == McpTransportType.stdio) 'env': env,
    if (transport == McpTransportType.stdio && workingDirectory != null)
      'workingDirectory': workingDirectory,
  };

  factory McpServerConfig.fromJson(Map<String, dynamic> json) {
    final tRaw = (json['transport'] as String?) ?? '';
    final t = tRaw == 'http'
        ? McpTransportType.http
        : (tRaw == 'stdio'
              ? McpTransportType.stdio
              : (tRaw == 'inmemory'
                    ? McpTransportType.inmemory
                    : McpTransportType.sse));
    final tools =
        (json['tools'] as List?)
            ?.map(
              (e) => McpToolConfig.fromJson((e as Map).cast<String, dynamic>()),
            )
            .toList() ??
        const <McpToolConfig>[];
    if (t == McpTransportType.stdio) {
      final argsAny = json['args'];
      final envAny = json['env'];
      return McpServerConfig(
        id: json['id'] as String? ?? const Uuid().v4(),
        enabled: json['enabled'] as bool? ?? true,
        name: json['name'] as String? ?? '',
        transport: McpTransportType.stdio,
        tools: tools,
        command: (json['command'] as String?)?.trim(),
        args: argsAny is List
            ? argsAny.map((e) => e.toString()).toList()
            : const <String>[],
        env: envAny is Map
            ? envAny.map((k, v) => MapEntry(k.toString(), v.toString()))
            : const <String, String>{},
        workingDirectory: (json['workingDirectory'] as String?)?.trim(),
      );
    } else if (t == McpTransportType.inmemory) {
      return McpServerConfig(
        id: json['id'] as String? ?? const Uuid().v4(),
        enabled: json['enabled'] as bool? ?? true,
        name: json['name'] as String? ?? '',
        transport: McpTransportType.inmemory,
        tools: tools,
      );
    } else {
      return McpServerConfig(
        id: json['id'] as String? ?? const Uuid().v4(),
        enabled: json['enabled'] as bool? ?? true,
        name: json['name'] as String? ?? '',
        transport: t,
        url: json['url'] as String? ?? '',
        tools: tools,
        headers:
            ((json['headers'] as Map?)?.map(
              (k, v) => MapEntry(k.toString(), v.toString()),
            )) ??
            const {},
      );
    }
  }
}

class McpProvider extends ChangeNotifier {
  static const String _prefsKey = 'mcp_servers_v1';
  static const String _prefsTimeoutKey = 'mcp_request_timeout_ms_v1';

  final Map<String, mcp.Client> _clients = {};
  final Map<String, McpStatus> _status = {}; // id -> status
  final Map<String, String> _errors = {}; // id -> last error
  List<McpServerConfig> _servers = [];
  // Reconnect bookkeeping to avoid duplicate concurrent retries
  final Set<String> _reconnecting = <String>{};
  // Heartbeat timers for live-connection health checks
  final Map<String, Timer> _heartbeats = <String, Timer>{};
  Duration _requestTimeout = const Duration(seconds: 30);
  String? _cachedSystemPath;

  McpProvider() {
    _load();
  }

  List<McpServerConfig> get servers => List.unmodifiable(_servers);
  McpStatus statusFor(String id) => _status[id] ?? McpStatus.idle;
  String? errorFor(String id) => _errors[id];
  bool get hasAnyEnabled => _servers.any((s) => s.enabled);
  bool isConnected(String id) =>
      _clients.containsKey(id) && statusFor(id) == McpStatus.connected;
  List<McpServerConfig> get connectedServers => _servers
      .where((s) => statusFor(s.id) == McpStatus.connected)
      .toList(growable: false);
  Duration get requestTimeout => _requestTimeout;
  int get requestTimeoutSeconds => _requestTimeout.inSeconds;

  Future<void> _load() async {
    final prefs = await SharedPreferences.getInstance();
    final timeoutMs = prefs.getInt(_prefsTimeoutKey);
    if (timeoutMs != null && timeoutMs > 0) {
      _requestTimeout = Duration(milliseconds: timeoutMs);
    }
    final raw = prefs.getString(_prefsKey);
    if (raw != null && raw.isNotEmpty) {
      try {
        final list = (jsonDecode(raw) as List)
            .map(
              (e) =>
                  McpServerConfig.fromJson((e as Map).cast<String, dynamic>()),
            )
            .toList();
        _servers = list;
      } catch (_) {}
    }
    // Ensure built-in @kelivo/fetch is present by default
    _ensureBuiltinFetchServerPresent();
    // initialize statuses
    for (final s in _servers) {
      _status[s.id] = McpStatus.idle;
      _errors.remove(s.id);
    }
    notifyListeners();

    // Auto-connect enabled servers
    for (final s in _servers.where((e) => e.enabled)) {
      // fire and forget
      unawaited(connect(s.id));
    }
  }

  void _ensureBuiltinFetchServerPresent() {
    final exists = _servers.any(
      (s) =>
          s.transport == McpTransportType.inmemory ||
          s.name == '@kelivo/fetch' ||
          s.id == 'kelivo_fetch',
    );
    if (exists) return;
    final cfg = McpServerConfig(
      id: 'kelivo_fetch',
      enabled: true,
      name: '@kelivo/fetch',
      transport: McpTransportType.inmemory,
      tools: const <McpToolConfig>[], // will refresh on connect
    );
    _servers = [..._servers, cfg];
  }

  Future<void> _persist() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(
      _prefsKey,
      jsonEncode(_servers.map((e) => e.toJson()).toList()),
    );
    await prefs.setInt(_prefsTimeoutKey, _requestTimeout.inMilliseconds);
  }

  Future<void> _persistTimeout() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setInt(_prefsTimeoutKey, _requestTimeout.inMilliseconds);
  }

  /// Export current MCP servers as a user-friendly JSON structure.
  ///
  /// Shape:
  /// {
  ///   "mcpServers": {
  ///     "serverId": {
  ///       "name": "...",
  ///       "type": "streamableHttp" | "sse",
  ///       "description": "",
  ///       "isActive": true/false,
  ///       "baseUrl": "...",
  ///       "headers": { ... }
  ///     },
  ///     ...
  ///   }
  /// }
  String exportServersAsUiJson() {
    final isDesktop = _isDesktopPlatform();
    try {
      final serversJson = jsonEncode(_servers.map((e) => e.toJson()).toList());
      return rust_mcp.exportMcpServersUiJson(
        serversJson: serversJson,
        isDesktop: isDesktop,
      );
    } catch (e, st) {
      _logError('exportServersAsUiJson', e, st);
      return '{}';
    }
  }

  Future<void> replaceAllFromJson(String rawJson) async {
    final isDesktop = _isDesktopPlatform();
    final parsedJson = rust_mcp.parseMcpImportJson(
      rawJson: rawJson,
      isDesktop: isDesktop,
    );
    final List<dynamic> list = jsonDecode(parsedJson);
    final next = list
        .map((e) => McpServerConfig.fromJson(e as Map<String, dynamic>))
        .toList();

    // Disconnect all current
    for (final s in _servers) {
      try {
        await disconnect(s.id);
      } catch (_) {}
    }

    // Replace and reset statuses
    _servers = next;
    _status.clear();
    _errors.clear();
    for (final s in _servers) {
      _status[s.id] = McpStatus.idle;
    }

    await _persist();
    notifyListeners();

    // Auto-connect enabled servers
    for (final s in _servers.where((e) => e.enabled)) {
      unawaited(connect(s.id));
    }
  }

  McpServerConfig? getById(String id) {
    for (final s in _servers) {
      if (s.id == id) return s;
    }
    return null;
  }

  Future<String> addServer({
    required bool enabled,
    required String name,
    required McpTransportType transport,
    String url = '',
    Map<String, String> headers = const {},
    String? command,
    List<String> args = const <String>[],
    Map<String, String> env = const <String, String>{},
    String? workingDirectory,
  }) async {
    final id = const Uuid().v4();
    final cfg = McpServerConfig(
      id: id,
      enabled: enabled,
      name: name.trim().isEmpty ? 'MCP' : name.trim(),
      transport: transport,
      url: url.trim(),
      headers: headers,
      command: command?.trim(),
      args: args,
      env: env,
      workingDirectory: (workingDirectory?.trim().isNotEmpty ?? false)
          ? workingDirectory!.trim()
          : null,
    );
    _servers = [..._servers, cfg];
    _status[id] = McpStatus.idle;
    await _persist();
    notifyListeners();
    if (enabled) {
      unawaited(connect(id));
    }
    return id;
  }

  Future<void> updateServer(McpServerConfig updated) async {
    final idx = _servers.indexWhere((e) => e.id == updated.id);
    if (idx < 0) return;
    _servers = List<McpServerConfig>.of(_servers)..[idx] = updated;
    await _persist();
    notifyListeners();
    if (!updated.enabled) {
      await disconnect(updated.id);
    } else {
      // Always reconnect after saving to apply changes (url/transport/name)
      await disconnect(updated.id);
      unawaited(connect(updated.id));
    }
  }

  Future<void> removeServer(String id) async {
    await disconnect(id);
    _servers = _servers.where((e) => e.id != id).toList(growable: false);
    _status.remove(id);
    await _persist();
    notifyListeners();
  }

  Future<void> reorderServers(int oldIndex, int newIndex) async {
    if (oldIndex == newIndex) return;
    if (oldIndex < 0 || oldIndex >= _servers.length) return;
    if (newIndex < 0 || newIndex >= _servers.length) return;
    final moved = _servers.removeAt(oldIndex);
    _servers.insert(newIndex, moved);
    notifyListeners();
    await _persist();
  }

  Future<void> setToolEnabled(
    String serverId,
    String toolName,
    bool enabled,
  ) async {
    final idx = _servers.indexWhere((e) => e.id == serverId);
    if (idx < 0) return;
    final server = _servers[idx];
    final tools = server.tools
        .map((t) => t.name == toolName ? t.copyWith(enabled: enabled) : t)
        .toList();
    _servers[idx] = server.copyWith(tools: tools);
    await _persist();
    notifyListeners();
  }

  /// Set whether a tool requires user approval before execution.
  Future<void> setToolNeedsApproval(
    String serverId,
    String toolName,
    bool needsApproval,
  ) async {
    final idx = _servers.indexWhere((e) => e.id == serverId);
    if (idx < 0) return;
    final server = _servers[idx];
    final tools = server.tools
        .map((t) =>
            t.name == toolName ? t.copyWith(needsApproval: needsApproval) : t)
        .toList();
    _servers[idx] = server.copyWith(tools: tools);
    await _persist();
    notifyListeners();
  }

  /// Check if a tool (by name) requires approval across all connected servers.
  /// Conservative: returns true if ANY connected server marks the tool as needing approval.
  bool toolNeedsApproval(String toolName) {
    for (final s in _servers) {
      if (statusFor(s.id) != McpStatus.connected) continue;
      if (!s.enabled) continue;
      for (final t in s.tools) {
        if (t.name == toolName && t.enabled && t.needsApproval) return true;
      }
    }
    return false;
  }

  Future<void> connect(String id) async {
    final server = _servers.firstWhere(
      (e) => e.id == id,
      orElse: () => throw StateError('Server not found'),
    );
    // If already connected, try a ping by listing tools quickly; else return
    if (_clients.containsKey(id)) {
      // Already connected; update status just in case
      _status[id] = McpStatus.connected;
      _errors.remove(id);
      notifyListeners();
      return;
    }
    _status[id] = McpStatus.connecting;
    _errors.remove(id);
    notifyListeners();

    try {
      // Log connect intent and parameters
      // debugPrint('[MCP/Connect] id=$id name=${server.name} transport=${server.transport.name}');
      // debugPrint('[MCP/Connect] url=${server.url}');
      // if (server.headers.isNotEmpty) {
      //   debugPrint('[MCP/Headers] ${server.headers.length} headers:');
      //   server.headers.forEach((k, v) {
      //     final masked = _maskIfSensitive(k, v);
      //     debugPrint('  - $k: $masked');
      //   });
      // } else {
      //   debugPrint('[MCP/Headers] (none)');
      // }

      final clientConfig = mcp.McpClient.simpleConfig(
        name: 'Kelivo MCP',
        version: '1.0.0',
        // Turn on library-internal verbose logs
        enableDebugLogging: false,
        requestTimeout: _requestTimeout,
      );

      // In-memory builtin server path
      if (server.transport == McpTransportType.inmemory) {
        final engine = KelivoFetchMcpServerEngine();
        final transport = KelivoInMemoryClientTransport(engine);
        final client = mcp.McpClient.createClient(clientConfig);
        await client.connect(transport);
        _clients[id] = client;
        _status[id] = McpStatus.connected;
        _errors.remove(id);
        notifyListeners();
        await refreshTools(id);
        _startHeartbeat(id);
        return;
      }

      final mergedHeaders = <String, String>{...server.headers};
      final transportConfig = await () async {
        if (server.transport == McpTransportType.sse) {
          return mcp.TransportConfig.sse(
            serverUrl: server.url,
            headers: mergedHeaders.isEmpty ? null : mergedHeaders,
          );
        } else if (server.transport == McpTransportType.http) {
          return mcp.TransportConfig.streamableHttp(
            baseUrl: server.url,
            headers: mergedHeaders.isEmpty ? null : mergedHeaders,
            timeout: _requestTimeout,
          );
        } else {
          // STDIO; only supported on desktop
          if (!_isDesktopPlatform()) {
            throw StateError('STDIO transport not supported on this platform');
          }
          final cmd = server.command;
          if (cmd == null || cmd.isEmpty) {
            throw StateError('STDIO command is empty');
          }
          final mergedEnv = await _resolveEnvironmentWithPath(server.env);
          final commandExists = await _validateCommand(cmd, mergedEnv);
          if (!commandExists) {
            throw StateError(
              'Command "$cmd" not found in PATH. '
              'Ensure the command is installed and accessible.',
            );
          }
          return mcp.TransportConfig.stdio(
            command: cmd,
            arguments: server.args,
            workingDirectory: server.workingDirectory,
            environment: mergedEnv.isEmpty ? null : mergedEnv,
          );
        }
      }();

      // debugPrint('[MCP/Connect] creating client (enableDebugLogging=true) ...');
      final clientResult = await mcp.McpClient.createAndConnect(
        config: clientConfig,
        transportConfig: transportConfig,
      );

      final client = clientResult.fold((c) => c, (err) => throw err);
      _clients[id] = client;
      _status[id] = McpStatus.connected;
      _errors.remove(id);
      // debugPrint('[MCP/Connected] id=$id (${server.name})');
      notifyListeners();

      // Try to refresh tools once connected
      // debugPrint('[MCP/Tools] refreshing tools for id=$id ...');
      await refreshTools(id);
      // debugPrint('[MCP/Tools] refresh done for id=$id');

      // Start/refresh heartbeat for this connection
      _startHeartbeat(id);
    } catch (e) {
      // debugPrint('[MCP/Error] connect failed for id=$id (${server.name})');
      // _logMcpException('connect', serverId: id, error: e, stack: st);
      _status[id] = McpStatus.error;
      _errors[id] = e.toString();
      notifyListeners();
    }
  }

  Future<void> updateRequestTimeout(
    Duration duration, {
    bool reconnectActive = true,
  }) async {
    if (duration.inMilliseconds <= 0) return;
    if (duration == _requestTimeout) return;
    _requestTimeout = duration;
    await _persistTimeout();
    notifyListeners();
    if (reconnectActive) {
      for (final id in _clients.keys.toList()) {
        if (_servers.any((s) => s.id == id && s.enabled)) {
          unawaited(reconnect(id));
        }
      }
    }
  }

  Future<void> disconnect(String id) async {
    final client = _clients.remove(id);
    try {
      // debugPrint('[MCP/Disconnect] id=$id ...');
      client?.disconnect();
      // debugPrint('[MCP/Disconnect] id=$id done');
    } catch (e) {
      // debugPrint('[MCP/Error] disconnect failed for id=$id');
      // _logMcpException('disconnect', serverId: id, error: e, stack: st);
    }
    _status[id] = McpStatus.idle;
    _errors.remove(id);
    _stopHeartbeat(id);
    notifyListeners();
  }

  Future<void> reconnect(String id) async {
    await disconnect(id);
    await connect(id);
  }

  Future<void> _reconnectWithBackoff(String id, {int maxAttempts = 3}) async {
    if (_reconnecting.contains(id)) return;
    _reconnecting.add(id);
    try {
      for (int attempt = 1; attempt <= maxAttempts; attempt++) {
        await reconnect(id);
        if (isConnected(id)) return;
        // progressive backoff: 600ms, 1200ms, 2400ms
        final delayMs = 600 * (1 << (attempt - 1));
        await Future.delayed(Duration(milliseconds: delayMs));
      }
    } finally {
      _reconnecting.remove(id);
    }
  }

  void _startHeartbeat(
    String id, {
    Duration interval = const Duration(seconds: 12),
  }) {
    _stopHeartbeat(id);
    _heartbeats[id] = Timer.periodic(interval, (t) async {
      // Heartbeat only when we think we're connected
      if (!isConnected(id)) return;
      final client = _clients[id];
      if (client == null) return;
      try {
        // A lightweight call to verify liveness
        // listTools is relatively cheap and available
        final fut = client.listTools();
        // Add a soft timeout to avoid piling up
        await fut.timeout(const Duration(seconds: 6));
      } catch (e) {
        // debugPrint('[MCP/Heartbeat] liveness check failed id=$id');
        // Consider connection lost; mark error and try auto-reconnect
        _status[id] = McpStatus.error;
        _errors[id] = e.toString();
        notifyListeners();
        await _reconnectWithBackoff(id, maxAttempts: 3);
        // If reconnected, restart heartbeat (connect() also starts it)
        if (!isConnected(id)) {
          // keep error state; next heartbeat tick will be a no-op
        }
      }
    });
  }

  void _stopHeartbeat(String id) {
    _heartbeats.remove(id)?.cancel();
  }

  McpToolConfig? _toolConfig(String serverId, String toolName) {
    final idx = _servers.indexWhere((e) => e.id == serverId);
    if (idx < 0) return null;
    final s = _servers[idx];
    for (final t in s.tools) {
      if (t.name == toolName) return t;
    }
    return null;
  }

  Map<String, dynamic> _normalizeArgsForTool(
    String serverId,
    String toolName,
    Map<String, dynamic> args,
  ) {
    try {
      final cfg = _toolConfig(serverId, toolName);
      final schema = cfg?.schema;
      if (schema == null || schema.isEmpty) return args;
      final schemaJson = jsonEncode(schema);
      final argsJson = jsonEncode(args);
      final resultJson = rust_mcp.normalizeToolArguments(
        schemaJson: schemaJson,
        argsJson: argsJson,
      );
      final result = jsonDecode(resultJson);
      if (result is Map<String, dynamic>) return result;
      return args;
    } catch (_) {
      return args;
    }
  }

  Future<void> refreshTools(String id) async {
    final client = _clients[id];
    if (client == null) return;
    try {
      // debugPrint('[MCP/Tools] listTools() ...');
      final tools = await client.listTools();
      // debugPrint('[MCP/Tools] listTools() returned ${tools.length} tools');
      // Preserve enabled state from existing config
      final idx = _servers.indexWhere((e) => e.id == id);
      if (idx < 0) return;
      final existing = _servers[idx].tools;
      final existingMap = {for (final t in existing) t.name: t};

      List<McpToolConfig> merged = [];
      for (final t in tools) {
        final prior = existingMap[t.name];
        // Extract params from inputSchema if present
        final params = <McpParamSpec>[];
        Map<String, dynamic>? schemaJson;
        try {
          final js = t.inputSchema;
          schemaJson = js;
          final props =
              (js['properties'] as Map?)?.cast<String, dynamic>() ??
              const <String, dynamic>{};
          final req =
              (js['required'] as List?)?.map((e) => e.toString()).toSet() ??
              const <String>{};
          props.forEach((key, val) {
            String? ty;
            dynamic defVal;
            try {
              final v = (val as Map).cast<String, dynamic>();
              final ttype = v['type'];
              if (ttype is String) {
                ty = ttype;
              } else if (ttype is List) {
                ty = ttype.map((e) => e.toString()).join('|');
              }
              defVal = v['default'];
            } catch (_) {}
            params.add(
              McpParamSpec(
                name: key,
                required: req.contains(key),
                type: ty,
                defaultValue: defVal,
              ),
            );
          });
        } catch (_) {}

        merged.add(
          McpToolConfig(
            enabled: prior?.enabled ?? true,
            name: t.name,
            description: t.description,
            params: params,
            schema: schemaJson,
            needsApproval: prior?.needsApproval ?? false,
          ),
        );
      }

      _servers[idx] = _servers[idx].copyWith(tools: merged);
      await _persist();
      notifyListeners();
    } catch (e) {
      // debugPrint('[MCP/Tools] listTools() failed for id=$id');
      // ignore tool refresh errors; status stays connected
    }
  }

  Future<void> ensureConnected(String id) async {
    // Do not attempt to connect if the server is disabled
    final cfg = getById(id);
    if (cfg == null || !cfg.enabled) return;
    if (isConnected(id)) return;
    // Try a few times with short backoff in case server blips
    await _reconnectWithBackoff(id, maxAttempts: 3);
  }

  Future<mcp.CallToolResult?> callTool(
    String serverId,
    String toolName,
    Map<String, dynamic> args,
  ) async {
    try {
      await ensureConnected(serverId);
      var client = _clients[serverId];
      if (client == null) return null;
      // Normalize arguments based on tool schema (best-effort)
      final normalized = _normalizeArgsForTool(serverId, toolName, args);
      // if (normalized != args) {
      //   debugPrint('[MCP/Call] serverId=$serverId tool=$toolName args(normalized)=${jsonEncode(normalized)}');
      // } else {
      //   debugPrint('[MCP/Call] serverId=$serverId tool=$toolName args=${jsonEncode(args)}');
      // }
      final result = await client.callTool(toolName, normalized);
      // Detailed call timing/content logging disabled
      return result;
    } catch (e) {
      // debugPrint('[MCP/Call/Error] serverId=$serverId tool=$toolName');

      // If this is a parameter validation error from the server, do NOT disconnect.
      try {
        if (e is mcp.McpError && (e.code == -32602)) {
          // Keep connection healthy status; surface error to caller via null
          _errors[serverId] = e.toString();
          // debugPrint('[MCP/Call] invalid arguments; skipping reconnect');
          return null;
        }
      } catch (_) {}

      _status[serverId] = McpStatus.error;
      _errors[serverId] = e.toString();
      notifyListeners();
      // Auto-reconnect a few times and try once more
      try {
        await _reconnectWithBackoff(serverId, maxAttempts: 3);
        if (!isConnected(serverId)) return null;
        final client = _clients[serverId];
        if (client == null) return null;
        // debugPrint('[MCP/Call] retry serverId=$serverId tool=$toolName');
        final normalized = _normalizeArgsForTool(serverId, toolName, args);
        final result = await client.callTool(toolName, normalized);
        // Detailed retry logging disabled
        // Mark healthy again
        _status[serverId] = McpStatus.connected;
        _errors.remove(serverId);
        notifyListeners();
        return result;
      } catch (e2) {
        // debugPrint('[MCP/Call/RetryError] serverId=$serverId tool=$toolName');
        // Keep error state; give up
        return null;
      }
    }
  }

  List<McpToolConfig> getEnabledToolsForServers(Set<String> serverIds) {
    // Only expose tools for servers that are both selected AND currently connected
    final tools = <McpToolConfig>[];
    for (final s in _servers.where((s) => serverIds.contains(s.id))) {
      if (statusFor(s.id) != McpStatus.connected) continue;
      if (!s.enabled) continue;
      tools.addAll(s.tools.where((t) => t.enabled));
    }
    return tools;
  }

  void _logError(String method, Object e, StackTrace st) {
    // Log to debug console for development; production builds strip these.
    debugPrint('[MCP/Error] $method failed: $e\n$st');
  }

  @override
  void dispose() {
    // Clean up timers
    for (final t in _heartbeats.values) {
      t.cancel();
    }
    _heartbeats.clear();
    super.dispose();
  }

  Future<String?> _getSystemPath() async {
    if (_cachedSystemPath != null) return _cachedSystemPath;
    if (!Platform.isMacOS) return null;
    try {
      final result = await Process.run('launchctl', ['getenv', 'PATH']);
      if (result.exitCode == 0) {
        _cachedSystemPath = (result.stdout as String).trim();
        return _cachedSystemPath;
      }
    } catch (_) {}
    return null;
  }

  Future<Map<String, String>> _resolveEnvironmentWithPath(
    Map<String, String> userEnv,
  ) async {
    final merged = Map<String, String>.from(userEnv);
    if (!merged.containsKey('PATH')) {
      final systemPath = await _getSystemPath();
      if (systemPath != null && systemPath.isNotEmpty) {
        merged['PATH'] = systemPath;
      }
    }
    return merged;
  }

  Future<bool> _validateCommand(
    String command,
    Map<String, String> environment,
  ) async {
    try {
      final whichCmd = Platform.isWindows ? 'where' : 'which';
      final result = await Process.run(
        whichCmd,
        [command],
        environment: environment,
        runInShell: true,
      );
      return result.exitCode == 0;
    } catch (_) {
      return false;
    }
  }

  bool _isDesktopPlatform() {
    if (kIsWeb) return false;
    return defaultTargetPlatform == TargetPlatform.windows ||
        defaultTargetPlatform == TargetPlatform.linux ||
        defaultTargetPlatform == TargetPlatform.macOS;
  }
}
