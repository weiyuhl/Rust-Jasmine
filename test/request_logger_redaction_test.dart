import 'dart:convert';

import 'package:flutter_test/flutter_test.dart';
import 'package:Kelivo/core/services/network/request_logger.dart';

void main() {
  group('RequestLogger redaction', () {
    test('redacts sensitive headers', () {
      final headers = RequestLogger.redactHeaders({
        'Authorization': 'Bearer sk-secret',
        'x-api-key': 'sk-secret',
        'X-API-Key': 'sk-secret',
        'X-Api-Key': 'sk-secret',
        'Api-Key': 'sk-secret',
        'X-Goog-Api-Key': 'sk-secret',
        'X-Auth-Token': 'token-secret',
        'X-Client-Secret': 'client-secret',
        'Set-Cookie': 'session=secret',
        'Content-Type': 'application/json',
      });

      expect(headers['Authorization'], RequestLogger.redactedValue);
      expect(headers['x-api-key'], RequestLogger.redactedValue);
      expect(headers['X-API-Key'], RequestLogger.redactedValue);
      expect(headers['X-Api-Key'], RequestLogger.redactedValue);
      expect(headers['Api-Key'], RequestLogger.redactedValue);
      expect(headers['X-Goog-Api-Key'], RequestLogger.redactedValue);
      expect(headers['X-Auth-Token'], RequestLogger.redactedValue);
      expect(headers['X-Client-Secret'], RequestLogger.redactedValue);
      expect(headers['Set-Cookie'], RequestLogger.redactedValue);
      expect(headers['Content-Type'], 'application/json');
    });

    test('redacts request credentials and prompt content', () {
      final redacted = RequestLogger.redactBodyText(
        jsonEncode({
          'model': 'gpt-4.1',
          'apiKey': 'sk-secret',
          'messages': [
            {'role': 'user', 'content': 'private prompt'},
          ],
          'metadata': {'safe': 1, 'access_token': 'token-secret'},
        }),
      );
      final obj = jsonDecode(redacted) as Map<String, dynamic>;

      expect(obj['model'], 'gpt-4.1');
      expect(obj['apiKey'], RequestLogger.redactedValue);
      expect(obj['messages'], RequestLogger.redactedValue);
      expect((obj['metadata'] as Map<String, dynamic>)['safe'], 1);
      expect(
        (obj['metadata'] as Map<String, dynamic>)['access_token'],
        RequestLogger.redactedValue,
      );
      expect(redacted, isNot(contains('private prompt')));
      expect(redacted, isNot(contains('sk-secret')));
      expect(redacted, isNot(contains('token-secret')));
    });

    test('redacts streamed SSE payload content', () {
      final redacted = RequestLogger.redactBodyText(
        'data: ${jsonEncode({
          'choices': [
            {
              'delta': {'content': 'private output'},
            },
          ],
        })}',
      );

      expect(redacted, contains('data:'));
      expect(redacted, isNot(contains('private output')));
    });
  });
}
