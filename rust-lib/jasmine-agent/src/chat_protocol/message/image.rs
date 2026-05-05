/// Parse text with embedded image references (Markdown + custom markers).
/// Returns the cleaned text and a list of decoded image references.
#[derive(serde::Serialize)]
pub struct ParsedContent {
    pub text: String,
    pub images: Vec<ImageRef>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct ImageRef {
    pub kind: ImageKind,
    pub src: String,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub enum ImageKind {
    Data,
    Path,
    Url,
}

/// Parse raw text for Markdown image syntax `![...](url)` and
/// custom markers `[image:path]`. Returns text with images extracted.
pub fn parse_text_and_images(raw: &str) -> ParsedContent {
    let mut images = Vec::new();
    let mut text_buf = String::new();

    let chars: Vec<char> = raw.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Try Markdown image: ![...](...)
        if chars[i] == '!' && i + 1 < len && chars[i + 1] == '[' {
            if let Some(end_alt) = raw[i + 2..].find(']') {
                if end_alt + i + 3 < len && chars[i + end_alt + 3] == '(' {
                    if let Some(end_url) = raw[i + end_alt + 4..].find(')') {
                        let url_start = i + end_alt + 4;
                        let url = raw[url_start..url_start + end_url].trim().to_string();
                        if !url.is_empty() {
                            if is_data_url(&url) {
                                images.push(ImageRef { kind: ImageKind::Data, src: url });
                            } else if is_remote_url(&url) {
                                images.push(ImageRef { kind: ImageKind::Url, src: url });
                            } else {
                                images.push(ImageRef { kind: ImageKind::Path, src: url });
                            }
                        }
                        i = url_start + end_url + 1;
                        continue;
                    }
                }
            }
        }

        // Try custom marker: [image:...]
        if chars[i] == '[' && raw[i..].starts_with("[image:") {
            if let Some(end) = raw[i + 7..].find(']') {
                let url = raw[i + 7..i + 7 + end].trim().to_string();
                if !url.is_empty() {
                    if is_data_url(&url) {
                        images.push(ImageRef { kind: ImageKind::Data, src: url });
                    } else if is_remote_url(&url) {
                        images.push(ImageRef { kind: ImageKind::Url, src: url });
                    } else {
                        images.push(ImageRef { kind: ImageKind::Path, src: url });
                    }
                }
                i = i + 7 + end + 1;
                continue;
            }
        }

        text_buf.push(chars[i]);
        i += 1;
    }

    ParsedContent {
        text: text_buf.trim().to_string(),
        images,
    }
}

/// Guess MIME type from a file path extension.
pub fn mime_from_path(path: &str) -> String {
    let lower = path.to_lowercase();
    if lower.ends_with(".png") { "image/png".into() }
    else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") { "image/jpeg".into() }
    else if lower.ends_with(".gif") { "image/gif".into() }
    else if lower.ends_with(".webp") { "image/webp".into() }
    else if lower.ends_with(".svg") { "image/svg+xml".into() }
    else if lower.ends_with(".bmp") { "image/bmp".into() }
    else { "image/png".into() } // fallback
}

fn is_data_url(src: &str) -> bool {
    src.starts_with("data:")
}

fn is_remote_url(src: &str) -> bool {
    src.starts_with("http://") || src.starts_with("https://")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown_image() {
        let result = parse_text_and_images("Look ![img](https://example.com/a.png) here");
        assert_eq!(result.text, "Look  here");
        assert_eq!(result.images.len(), 1);
        assert_eq!(result.images[0].src, "https://example.com/a.png");
    }

    #[test]
    fn test_parse_custom_marker() {
        let result = parse_text_and_images("See [image:/path/to/img.png] now");
        assert_eq!(result.text, "See  now");
        assert_eq!(result.images.len(), 1);
        assert_eq!(result.images[0].kind, ImageKind::Path);
    }
}
