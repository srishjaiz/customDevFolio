/// Portfolio / account URL slug: lowercase letters, digits, single hyphens.
pub fn is_valid_slug(s: &str) -> bool {
    if s.is_empty() || s.len() > 128 {
        return false;
    }
    let mut prev_hyphen = false;
    for c in s.chars() {
        match c {
            'a'..='z' | '0'..='9' => prev_hyphen = false,
            '-' => {
                if prev_hyphen {
                    return false;
                }
                prev_hyphen = true;
            }
            _ => return false,
        }
    }
    !s.starts_with('-') && !s.ends_with('-')
}

/// Derive a slug from a display name (best-effort).
pub fn normalize_slug(raw: &str) -> String {
    let lower = raw.trim().to_ascii_lowercase();
    let mut out = String::with_capacity(lower.len());
    let mut prev_dash = false;
    for ch in lower.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            prev_dash = false;
        } else if !prev_dash && !out.is_empty() {
            out.push('-');
            prev_dash = true;
        }
    }
    let trimmed = out.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "portfolio".into()
    } else {
        trimmed.chars().take(128).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_slugs() {
        assert!(is_valid_slug("ada-lovelace"));
        assert!(is_valid_slug("a"));
        assert!(is_valid_slug("x1"));
        assert!(!is_valid_slug(""));
        assert!(!is_valid_slug("-a"));
        assert!(!is_valid_slug("a-"));
        assert!(!is_valid_slug("Ada"));
        assert!(!is_valid_slug("a--b"));
        assert!(!is_valid_slug("a b"));
    }

    #[test]
    fn normalize() {
        assert_eq!(normalize_slug("Ada Lovelace"), "ada-lovelace");
        assert_eq!(normalize_slug("  "), "portfolio");
        assert_eq!(normalize_slug("Foo_Bar"), "foo-bar");
    }
}
