//! Replaces code spans and URLs with placeholders such as `⟦0⟧`, so that the model
//! cannot "correct" identifiers or links.

pub struct Masked {
    pub text: String,
    spans: Vec<String>,
}

impl Masked {
    pub fn new(text: &str) -> Self {
        // A text that already contains the placeholder brackets cannot be restored safely.
        if text.contains('⟦') {
            return Self {
                text: text.to_string(),
                spans: vec![],
            };
        }

        let mut masked = String::with_capacity(text.len());
        let mut spans = vec![];
        let (mut copied, mut i) = (0, 0);
        while i < text.len() {
            let rest = &text[i..];
            let len = protected_len(rest);
            if len > 0 {
                masked.push_str(&text[copied..i]);
                masked.push_str(&placeholder(spans.len()));
                spans.push(rest[..len].to_string());
                i += len;
                copied = i;
            } else {
                i += rest.chars().next().map_or(1, char::len_utf8);
            }
        }
        masked.push_str(&text[copied..]);

        Self {
            text: masked,
            spans,
        }
    }

    /// Puts the original spans back. Returns `None` if the model dropped, duplicated,
    /// or changed a placeholder.
    pub fn unmask(&self, corrected: &str) -> Option<String> {
        let mut text = corrected.to_string();
        for (i, span) in self.spans.iter().enumerate() {
            let placeholder = placeholder(i);
            if text.matches(&placeholder).count() != 1 {
                return None;
            }
            text = text.replacen(&placeholder, span, 1);
        }
        Some(text)
    }
}

fn placeholder(i: usize) -> String {
    format!("⟦{i}⟧")
}

/// Byte length of the code span or URL at the start of `s`, or 0.
fn protected_len(s: &str) -> usize {
    if let Some(body) = s.strip_prefix("```") {
        body.find("```").map_or(s.len(), |end| end + 6)
    } else if let Some(body) = s.strip_prefix('`') {
        body.find('`').map_or(0, |end| end + 2)
    } else if s.starts_with("http://") || s.starts_with("https://") {
        let end = s.find(char::is_whitespace).unwrap_or(s.len());
        s[..end]
            .trim_end_matches(['.', ',', ';', ':', '!', '?', ')', '"', '\''])
            .len()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masks_code_and_urls() {
        let masked =
            Masked::new("fix `parse_respone()` see https://exmaple.com/a, or\n```\nlet x\n```");
        assert_eq!(masked.text, "fix ⟦0⟧ see ⟦1⟧, or\n⟦2⟧");
        assert_eq!(
            masked.unmask("Fix ⟦0⟧; see ⟦1⟧, or\n⟦2⟧").unwrap(),
            "Fix `parse_respone()`; see https://exmaple.com/a, or\n```\nlet x\n```"
        );
    }

    #[test]
    fn leaves_unmatched_backtick_and_plain_text() {
        let masked = Masked::new("it`s ok, café");
        assert_eq!(masked.text, "it`s ok, café");
        assert_eq!(masked.unmask("It's ok, café").unwrap(), "It's ok, café");
    }

    #[test]
    fn rejects_altered_placeholders() {
        let masked = Masked::new("see `a` and `b`");
        assert!(masked.unmask("See ⟦0⟧ and ⟦0⟧").is_none());
        assert!(masked.unmask("See ⟦0⟧ and b").is_none());
    }

    #[test]
    fn skips_text_with_placeholder_brackets() {
        let masked = Masked::new("⟦0⟧ `code`");
        assert_eq!(masked.text, "⟦0⟧ `code`");
        assert_eq!(masked.unmask("⟦0⟧ `code`").unwrap(), "⟦0⟧ `code`");
    }
}
