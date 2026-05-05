import 'dart:convert';

import 'package:flutter_test/flutter_test.dart';
import 'package:Kelivo/core/providers/settings_provider.dart';
import 'package:Kelivo/features/provider/widgets/share_provider_sheet.dart';

void main() {
  test('provider share code does not include API key', () {
    final code = encodeProviderConfig(
      ProviderConfig(
        id: 'OpenAI',
        enabled: true,
        name: 'OpenAI',
        apiKey: 'sk-secret',
        baseUrl: 'https://api.openai.com/v1',
        providerType: ProviderKind.openai,
      ),
    );

    final payload = code.substring('ai-provider:v1:'.length);
    final decoded =
        jsonDecode(utf8.decode(base64Decode(payload))) as Map<String, dynamic>;

    expect(decoded['type'], 'openai');
    expect(decoded['name'], 'OpenAI');
    expect(decoded['baseUrl'], 'https://api.openai.com/v1');
    expect(decoded.containsKey('apiKey'), isFalse);
    expect(code, isNot(contains('sk-secret')));
  });
}
