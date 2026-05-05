use serde_json::Value;

/// Sanitize a string by removing or replacing unsafe Unicode characters.
/// Mirrors Kelivo's `UnicodeSanitizer.sanitize`.
pub fn sanitize_unicode(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            // Null, BOM, and other control characters → strip
            '\u{0000}' | '\u{FEFF}' | '\u{FFFE}' => '\0',
            // Soft hyphen → regular hyphen
            '\u{00AD}' => '-',
            // Zero-width characters → remove
            '\u{200B}' | '\u{200C}' | '\u{200D}' => '\0',
            _ => c,
        })
        .filter(|&c| c != '\0')
        .collect()
}

/// Batch sanitize all messages in a list.
pub fn sanitize_messages(messages: &[Value]) -> Vec<Value> {
    messages
        .iter()
        .map(|msg| {
            let mut out = msg.clone();
            if let Some(content) = msg.get("content").and_then(|v| v.as_str()) {
                out["content"] = Value::String(sanitize_unicode(content));
            }
            out
        })
        .collect()
}
