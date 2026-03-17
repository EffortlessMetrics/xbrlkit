//! Inline HTML scanning primitives.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InlineFragment {
    pub element_name: String,
    pub fact_name: Option<String>,
    pub context_ref: Option<String>,
    pub unit_ref: Option<String>,
    pub decimals: Option<String>,
    pub value: String,
    #[serde(default)]
    pub attributes: BTreeMap<String, String>,
}

#[must_use]
pub fn scan_inline_fragments(html: &str) -> Vec<InlineFragment> {
    let mut fragments = Vec::new();
    let mut offset = 0;
    while let Some(relative_start) = html[offset..].find("<ix:") {
        let start = offset + relative_start;
        let Some(tag_end) = find_tag_end(html, start) else {
            break;
        };
        let tag_body = &html[start + 1..tag_end];
        if tag_body.starts_with('/') {
            offset = tag_end + 1;
            continue;
        }
        let self_closing = tag_body.trim_end().ends_with('/');
        let trimmed_tag = tag_body.trim_end_matches('/').trim();
        let (element_name, attributes) = parse_start_tag(trimmed_tag);
        if !element_name.starts_with("ix:") {
            offset = tag_end + 1;
            continue;
        }
        let (value, next_offset) = if self_closing {
            (String::new(), tag_end + 1)
        } else {
            let close_tag = format!("</{element_name}>");
            if let Some(relative_close) = html[tag_end + 1..].find(&close_tag) {
                let value_end = tag_end + 1 + relative_close;
                (
                    strip_tags(&html[tag_end + 1..value_end]).trim().to_string(),
                    value_end + close_tag.len(),
                )
            } else {
                (String::new(), tag_end + 1)
            }
        };
        fragments.push(InlineFragment {
            element_name: element_name.to_string(),
            fact_name: attributes.get("name").cloned(),
            context_ref: attributes.get("contextRef").cloned(),
            unit_ref: attributes.get("unitRef").cloned(),
            decimals: attributes.get("decimals").cloned(),
            value,
            attributes,
        });
        offset = next_offset;
    }
    fragments
}

fn find_tag_end(html: &str, start: usize) -> Option<usize> {
    let mut in_quotes = false;
    for (relative_index, ch) in html[start..].char_indices() {
        match ch {
            '"' => in_quotes = !in_quotes,
            '>' if !in_quotes => return Some(start + relative_index),
            _ => {}
        }
    }
    None
}

fn parse_start_tag(tag_body: &str) -> (&str, BTreeMap<String, String>) {
    let mut split_index = tag_body.len();
    for (index, ch) in tag_body.char_indices() {
        if ch.is_whitespace() {
            split_index = index;
            break;
        }
    }
    let element_name = &tag_body[..split_index];
    let attributes = parse_attributes(&tag_body[split_index..]);
    (element_name, attributes)
}

fn parse_attributes(raw: &str) -> BTreeMap<String, String> {
    let bytes = raw.as_bytes();
    let mut attributes = BTreeMap::new();
    let mut index = 0;
    while index < bytes.len() {
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if index >= bytes.len() {
            break;
        }
        let key_start = index;
        while index < bytes.len() && !bytes[index].is_ascii_whitespace() && bytes[index] != b'=' {
            index += 1;
        }
        let key = raw[key_start..index].trim();
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if index < bytes.len() && bytes[index] == b'=' {
            index += 1;
            while index < bytes.len() && bytes[index].is_ascii_whitespace() {
                index += 1;
            }
            let value = if index < bytes.len() && bytes[index] == b'"' {
                index += 1;
                let value_start = index;
                while index < bytes.len() && bytes[index] != b'"' {
                    index += 1;
                }
                let value = raw[value_start..index].to_string();
                if index < bytes.len() {
                    index += 1;
                }
                value
            } else {
                let value_start = index;
                while index < bytes.len() && !bytes[index].is_ascii_whitespace() {
                    index += 1;
                }
                raw[value_start..index].to_string()
            };
            if !key.is_empty() {
                attributes.insert(key.to_string(), value);
            }
        } else if !key.is_empty() {
            attributes.insert(key.to_string(), String::new());
        }
    }
    attributes
}

fn strip_tags(value: &str) -> String {
    let mut text = String::new();
    let mut in_tag = false;
    for ch in value.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => text.push(ch),
            _ => {}
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::scan_inline_fragments;

    #[test]
    fn parses_inline_fact_attributes_and_value() {
        let html = r#"<html><body><ix:nonNumeric name="dei:DocumentType" contextRef="c1">10-K</ix:nonNumeric></body></html>"#;
        let fragments = scan_inline_fragments(html);

        assert_eq!(fragments.len(), 1);
        assert_eq!(fragments[0].element_name, "ix:nonNumeric");
        assert_eq!(fragments[0].fact_name.as_deref(), Some("dei:DocumentType"));
        assert_eq!(fragments[0].context_ref.as_deref(), Some("c1"));
        assert_eq!(fragments[0].value, "10-K");
    }

    #[test]
    fn captures_banned_attributes_for_rule_checks() {
        let html = r#"<html><body><ix:nonFraction name="us-gaap:Assets" contextRef="c1" xml:base="https://example.com">100</ix:nonFraction></body></html>"#;
        let fragments = scan_inline_fragments(html);

        assert_eq!(fragments.len(), 1);
        assert_eq!(
            fragments[0].attributes.get("xml:base").map(String::as_str),
            Some("https://example.com")
        );
    }
}
