//! Error types for taxonomy loading.

/// Errors that can occur during taxonomy loading.
#[derive(Debug, thiserror::Error)]
pub enum TaxonomyLoaderError {
    /// I/O error reading a file.
    #[error("io error reading {0}: {1}")]
    Io(String, #[source] std::io::Error),

    /// XML parsing error.
    #[error("xml parse error: {0}")]
    XmlParse(String),

    /// Unsupported URL scheme (only http/https/file supported).
    #[error("unsupported URL: {0}")]
    UnsupportedUrl(String),

    /// Missing required element or attribute.
    #[error("missing required element: {0}")]
    MissingElement(String),

    /// Invalid schema reference.
    #[error("invalid schema reference: {0}")]
    InvalidSchemaRef(String),

    /// Invalid linkbase reference.
    #[error("invalid linkbase reference: {0}")]
    InvalidLinkbaseRef(String),
}

impl From<roxmltree::Error> for TaxonomyLoaderError {
    fn from(err: roxmltree::Error) -> Self {
        Self::XmlParse(err.to_string())
    }
}
