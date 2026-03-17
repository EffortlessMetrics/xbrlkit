//! Markdown rendering marker crate.

#[must_use]
pub fn render_summary(title: &str, body: &str) -> String {
    format!(
        "# {title}

{body}
"
    )
}
