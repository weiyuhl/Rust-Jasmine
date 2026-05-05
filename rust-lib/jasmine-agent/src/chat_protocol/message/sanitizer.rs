use serde_json::Value;

/// Replacement character for corrupted surrogates (U+FFFD).
const REPLACEMENT_CHAR: char = '\u{FFFD}';

/// Sanitize a string by removing or replacing unsafe Unicode characters.
/// Fully mirrors Kelivo's `UnicodeSanitizer.sanitize`, including:
/// - Surrogate pair validation and repair (UTF-16 surrogate handling)
/// - Damaged low surrogate repair (common PDF extraction corruption pattern)
/// - Control character stripping (null, BOM, reversed BOM, soft hyphen)
/// - Zero-width character stripping
pub fn sanitize_unicode(text: &str) -> String {
    if text.is_empty() {
        return text.to_string();
    }

    // Collect all code units as u16 for surrogate pair processing
    let code_units: Vec<u16> = text.encode_utf16().collect();
    let len = code_units.len();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;

    while i < len {
        let cu = code_units[i] as u32;

        // High surrogate: expect a following low surrogate
        if is_high_surrogate(cu) {
            if i + 1 < len {
                let next = code_units[i + 1] as u32;
                if is_low_surrogate(next) {
                    // Valid surrogate pair → decode and emit
                    out.push(safe_decode(code_point_from_surrogates(cu, next)));
                    i += 2;
                    continue;
                }
                // Damaged low surrogate repair (PDF corruption pattern)
                if looks_like_stripped_low_surrogate(next) {
                    let repaired = 0xD000 | next;
                    if is_low_surrogate(repaired) {
                        out.push(safe_decode(code_point_from_surrogates(cu, repaired)));
                        i += 2;
                        continue;
                    }
                }
            }
            // Unpaired high surrogate → replacement char
            out.push(REPLACEMENT_CHAR);
            i += 1;
            continue;
        }

        // Unpaired low surrogate → replacement char
        if is_low_surrogate(cu) {
            out.push(REPLACEMENT_CHAR);
            i += 1;
            continue;
        }

        // Regular character — sanitize control/zero-width chars
        if let Some(ch) = char::from_u32(cu) {
            match sanitize_single_char(ch) {
                Some(c) => out.push(c),
                None => {} // stripped
            }
        }
        i += 1;
    }

    out
}

/// Sanitize a single character. Returns None if the char should be stripped.
fn sanitize_single_char(c: char) -> Option<char> {
    match c {
        // Null → strip
        '\u{0000}' => None,
        // BOM and reversed BOM → strip
        '\u{FEFF}' | '\u{FFFE}' => None,
        // Soft hyphen → regular hyphen
        '\u{00AD}' => Some('-'),
        // Zero-width characters → strip
        '\u{200B}' | '\u{200C}' | '\u{200D}' => None,
        // Pass through
        _ => Some(c),
    }
}

/// Decode a code point, replacing invalid ones with the replacement character.
fn safe_decode(cp: u32) -> char {
    char::from_u32(cp).unwrap_or(REPLACEMENT_CHAR)
}

fn is_high_surrogate(cu: u32) -> bool {
    (0xD800..=0xDBFF).contains(&cu)
}

fn is_low_surrogate(cu: u32) -> bool {
    (0xDC00..=0xDFFF).contains(&cu)
}

fn looks_like_stripped_low_surrogate(cu: u32) -> bool {
    (0x0C00..=0x0FFF).contains(&cu)
}

fn code_point_from_surrogates(high: u32, low: u32) -> u32 {
    0x10000 + ((high - 0xD800) << 10) + (low - 0xDC00)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_null() {
        assert_eq!(sanitize_unicode("hello\u{0000}world"), "helloworld");
    }

    #[test]
    fn test_soft_hyphen_to_hyphen() {
        assert_eq!(sanitize_unicode("test\u{00AD}word"), "test-word");
    }

    #[test]
    fn test_strip_bom() {
        assert_eq!(sanitize_unicode("\u{FEFF}hello"), "hello");
    }

    #[test]
    fn test_valid_surrogate_pair() {
        // U+1F600 (grinning face) = 0xD83D 0xDE00
        let input = String::from_utf16(&[0xD83D, 0xDE00]).unwrap();
        let result = sanitize_unicode(&input);
        assert_eq!(result, "\u{1F600}");
    }

    #[test]
    fn test_unpaired_high_surrogate() {
        let mut buf = String::new();
        buf.push(char::from_u32(0xD83D).unwrap_or(REPLACEMENT_CHAR));
        let result = sanitize_unicode(&buf);
        assert!(result.contains(REPLACEMENT_CHAR));
    }
}
