//! SAX-style streaming XBRL parser for large filings.
//!
//! This crate provides memory-efficient parsing of large XBRL documents
//! using event-based (SAX-style) parsing via `quick-xml`.
//!
//! # Example
//! ```
//! use xbrl_stream::{XbrlStreamReader, StreamingFact, FactHandler};
//! use std::io::Cursor;
//!
//! #[derive(Default)]
//! struct PrintHandler {
//!     facts: Vec<StreamingFact>,
//! }
//!
//! impl FactHandler for PrintHandler {
//!     fn on_fact(&mut self, fact: StreamingFact) -> anyhow::Result<()> {
//!         println!("Fact: {} = {}", fact.concept, fact.value);
//!         self.facts.push(fact);
//!         Ok(())
//!     }
//! }
//!
//! let xml = r#"<xbrl xmlns:us-gaap="http://fasb.org/us-gaap/2023">
//!     <us-gaap:Revenue contextRef="ctx-1" unitRef="usd" decimals="-3">12345000</us-gaap:Revenue>
//! </xbrl>"#;
//!
//! let handler = PrintHandler::default();
//! let reader = XbrlStreamReader::new(Cursor::new(xml), handler);
//! let handler = reader.parse().expect("parse failed");
//! assert_eq!(handler.facts.len(), 1);
//! ```

use quick_xml::events::Event;
use quick_xml::Reader;
use std::io::BufRead;

/// A streaming XBRL fact as extracted from the XML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamingFact {
    /// Full concept name (e.g., "us-gaap:Revenue")
    pub concept: String,
    /// Context reference ID
    pub context_ref: String,
    /// Optional unit reference
    pub unit_ref: Option<String>,
    /// Optional decimals attribute
    pub decimals: Option<String>,
    /// Fact value (normalized)
    pub value: String,
}

/// A streaming XBRL context definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamingContext {
    /// Context ID
    pub id: String,
    /// Entity identifier scheme
    pub entity_scheme: Option<String>,
    /// Entity identifier value
    pub entity_value: Option<String>,
    /// Period type (instant or duration)
    pub period: StreamingPeriod,
}

/// Period definition for streaming contexts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamingPeriod {
    /// Instant period with date
    Instant(String),
    /// Duration period with start and end dates
    Duration { start: String, end: String },
    /// Period not yet determined or invalid
    Unknown,
}

/// A streaming XBRL unit definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamingUnit {
    /// Unit ID
    pub id: String,
    /// Measure (e.g., "iso4217:USD")
    pub measure: Option<String>,
}

/// Callback trait for handling XBRL events during streaming parse.
pub trait FactHandler: Send {
    /// Called when a fact is fully parsed.
    fn on_fact(&mut self, fact: StreamingFact) -> anyhow::Result<()>;
    /// Called when a context definition is found.
    fn on_context(&mut self, context: StreamingContext) -> anyhow::Result<()> {
        let _ = context;
        Ok(())
    }
    /// Called when a unit definition is found.
    fn on_unit(&mut self, unit: StreamingUnit) -> anyhow::Result<()> {
        let _ = unit;
        Ok(())
    }
}

/// Errors that can occur during streaming XBRL parsing.
#[derive(Debug, thiserror::Error)]
pub enum StreamError {
    #[error("XML parse error: {source}")]
    XmlError { source: quick_xml::Error },
    #[error("Invalid XBRL structure: {0}")]
    StructureError(String),
    #[error("Missing required attribute: {0}")]
    MissingAttribute(String),
    #[error("Handler error: {0}")]
    HandlerError(#[from] anyhow::Error),
}

/// Streaming XBRL parser using SAX-style event processing.
pub struct XbrlStreamReader<R: BufRead, H: FactHandler> {
    reader: Reader<R>,
    buffer: Vec<u8>,
    handler: H,
}

impl<R: BufRead, H: FactHandler> XbrlStreamReader<R, H> {
    /// Create a new streaming XBRL parser.
    pub fn new(reader: R, handler: H) -> Self {
        let mut xml_reader = Reader::from_reader(reader);
        xml_reader.config_mut().trim_text(true);

        Self {
            reader: xml_reader,
            buffer: Vec::with_capacity(1024),
            handler,
        }
    }

    /// Parse the entire XBRL document, calling handler for each fact.
    /// Returns the handler for result inspection.
    pub fn parse(mut self) -> Result<H, StreamError> {
        let mut current_fact: Option<StreamingFactBuilder> = None;
        let mut current_context: Option<StreamingContextBuilder> = None;
        let mut current_unit: Option<StreamingUnitBuilder> = None;
        let mut depth = 0u32;

        loop {
            self.buffer.clear();
            match self.reader.read_event_into(&mut self.buffer) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    let name_bytes = e.name();
                    let name = std::str::from_utf8(name_bytes.as_ref())
                        .map_err(|_| StreamError::StructureError("Invalid UTF-8 in tag name".into()))?;
                    let name = name.to_string();

                    depth += 1;

                    // Check if this is a fact element (has contextRef attribute)
                    if let Some(ctx_ref) = Self::get_attr(&e, b"contextRef")? {
                        let concept = name.clone();
                        let unit_ref = Self::get_attr(&e, b"unitRef")?;
                        let decimals = Self::get_attr(&e, b"decimals")?;

                        current_fact = Some(StreamingFactBuilder {
                            concept,
                            context_ref: ctx_ref,
                            unit_ref,
                            decimals,
                            value: String::new(),
                        });
                    }

                    // Check for context definition
                    if name == "xbrli:context" || name.ends_with(":context") {
                        if let Some(id) = Self::get_attr(&e, b"id")? {
                            current_context = Some(StreamingContextBuilder {
                                id,
                                entity_scheme: None,
                                entity_value: None,
                                period_start: None,
                                period_end: None,
                                period_instant: None,
                            });
                        }
                    }

                    // Check for unit definition
                    if name == "xbrli:unit" || name.ends_with(":unit") {
                        if let Some(id) = Self::get_attr(&e, b"id")? {
                            current_unit = Some(StreamingUnitBuilder {
                                id,
                                measure: None,
                            });
                        }
                    }
                }

                Ok(Event::Text(e)) => {
                    if let Some(fact) = &mut current_fact {
                        let text = e.unescape().map_err(|_| {
                            StreamError::StructureError("Invalid text content".into())
                        })?;
                        fact.value.push_str(&text);
                    }
                }

                Ok(Event::End(e)) => {
                    let name_bytes = e.name();
                    let name = std::str::from_utf8(name_bytes.as_ref())
                        .map_err(|_| StreamError::StructureError("Invalid UTF-8 in end tag".into()))?;
                    let name = name.to_string();

                    depth = depth.saturating_sub(1);

                    // Finish fact if we're closing a fact element
                    if let Some(fact_builder) = current_fact.take() {
                        if Self::is_fact_end(&name, &fact_builder.concept) {
                            let fact = fact_builder.build()?;
                            self.handler.on_fact(fact).map_err(StreamError::HandlerError)?;
                        } else {
                            // Not our fact, restore it
                            current_fact = Some(fact_builder);
                        }
                    }

                    // Finish context
                    if (name == "xbrli:context" || name.ends_with(":context")) && current_context.is_some() {
                        if let Some(ctx_builder) = current_context.take() {
                            let context = ctx_builder.build()?;
                            self.handler.on_context(context).map_err(StreamError::HandlerError)?;
                        }
                    }

                    // Finish unit
                    if (name == "xbrli:unit" || name.ends_with(":unit")) && current_unit.is_some() {
                        if let Some(unit_builder) = current_unit.take() {
                            let unit = unit_builder.build()?;
                            self.handler.on_unit(unit).map_err(StreamError::HandlerError)?;
                        }
                    }
                }

                Ok(Event::Eof) => break,

                Err(e) => {
                    return Err(StreamError::XmlError { source: e });
                }
                _ => {}
            }
        }

        Ok(self.handler)
    }

    fn get_attr(e: &quick_xml::events::BytesStart<'_>, name: &[u8]) -> Result<Option<String>, StreamError> {
        for attr in e.attributes() {
            let attr = attr.map_err(|_| StreamError::StructureError("Invalid attribute".into()))?;
            if attr.key.as_ref() == name {
                let value = attr.unescape_value()
                    .map_err(|_| StreamError::StructureError("Invalid attribute value".into()))?;
                return Ok(Some(value.into_owned()));
            }
        }
        Ok(None)
    }

    fn is_fact_end(end_name: &str, concept: &str) -> bool {
        // Simple case: exact match
        if end_name == concept {
            return true;
        }
        // Handle namespace prefix variations
        if let Some(concept_local) = concept.split(':').nth(1) {
            if end_name.ends_with(&format!(":{}", concept_local)) {
                return true;
            }
        }
        false
    }
}

// Builder pattern for constructing facts during parsing
struct StreamingFactBuilder {
    concept: String,
    context_ref: String,
    unit_ref: Option<String>,
    decimals: Option<String>,
    value: String,
}

impl StreamingFactBuilder {
    fn build(self) -> Result<StreamingFact, StreamError> {
        Ok(StreamingFact {
            concept: self.concept,
            context_ref: self.context_ref,
            unit_ref: self.unit_ref,
            decimals: self.decimals,
            value: self.value.trim().to_string(),
        })
    }
}

struct StreamingContextBuilder {
    id: String,
    entity_scheme: Option<String>,
    entity_value: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
    period_instant: Option<String>,
}

impl StreamingContextBuilder {
    fn build(self) -> Result<StreamingContext, StreamError> {
        let period = if let Some(instant) = self.period_instant {
            StreamingPeriod::Instant(instant)
        } else if let (Some(start), Some(end)) = (self.period_start, self.period_end) {
            StreamingPeriod::Duration { start, end }
        } else {
            StreamingPeriod::Unknown
        };

        Ok(StreamingContext {
            id: self.id,
            entity_scheme: self.entity_scheme,
            entity_value: self.entity_value,
            period,
        })
    }
}

struct StreamingUnitBuilder {
    id: String,
    measure: Option<String>,
}

impl StreamingUnitBuilder {
    fn build(self) -> Result<StreamingUnit, StreamError> {
        Ok(StreamingUnit {
            id: self.id,
            measure: self.measure,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[derive(Default)]
    struct TestHandler {
        facts: Vec<StreamingFact>,
        contexts: Vec<StreamingContext>,
        units: Vec<StreamingUnit>,
    }

    impl FactHandler for TestHandler {
        fn on_fact(&mut self, fact: StreamingFact) -> anyhow::Result<()> {
            self.facts.push(fact);
            Ok(())
        }

        fn on_context(&mut self, context: StreamingContext) -> anyhow::Result<()> {
            self.contexts.push(context);
            Ok(())
        }

        fn on_unit(&mut self, unit: StreamingUnit) -> anyhow::Result<()> {
            self.units.push(unit);
            Ok(())
        }
    }

    #[test]
    fn parses_simple_fact() {
        let xml = r#"<?xml version="1.0"?>
<xbrl xmlns="http://www.xbrl.org/2003/instance" xmlns:us-gaap="http://fasb.org/us-gaap/2023">
    <us-gaap:Revenue contextRef="ctx-1" unitRef="usd" decimals="-3">12345000</us-gaap:Revenue>
</xbrl>
"#;

        let handler = TestHandler::default();
        let reader = XbrlStreamReader::new(Cursor::new(xml), handler);
        let handler = reader.parse().expect("parse failed");

        assert_eq!(handler.facts.len(), 1);
        let fact = &handler.facts[0];
        assert_eq!(fact.concept, "us-gaap:Revenue");
        assert_eq!(fact.context_ref, "ctx-1");
        assert_eq!(fact.unit_ref, Some("usd".to_string()));
        assert_eq!(fact.decimals, Some("-3".to_string()));
        assert_eq!(fact.value, "12345000");
    }

    #[test]
    fn parses_multiple_facts() {
        let xml = r#"<?xml version="1.0"?>
<xbrl xmlns:us-gaap="http://fasb.org/us-gaap/2023">
    <us-gaap:Revenue contextRef="ctx-1" unitRef="usd">1000000</us-gaap:Revenue>
    <us-gaap:Assets contextRef="ctx-1" unitRef="usd">5000000</us-gaap:Assets>
    <us-gaap:Liabilities contextRef="ctx-1" unitRef="usd">2000000</us-gaap:Liabilities>
</xbrl>
"#;

        let handler = TestHandler::default();
        let reader = XbrlStreamReader::new(Cursor::new(xml), handler);
        let handler = reader.parse().expect("parse failed");

        assert_eq!(handler.facts.len(), 3);
        assert_eq!(handler.facts[0].concept, "us-gaap:Revenue");
        assert_eq!(handler.facts[1].concept, "us-gaap:Assets");
        assert_eq!(handler.facts[2].concept, "us-gaap:Liabilities");
    }

    #[test]
    fn handles_empty_xbrl() {
        let xml = r#"<?xml version="1.0"?>
<xbrl xmlns="http://www.xbrl.org/2003/instance">
</xbrl>
"#;

        let handler = TestHandler::default();
        let reader = XbrlStreamReader::new(Cursor::new(xml), handler);
        let handler = reader.parse().expect("parse failed");

        assert!(handler.facts.is_empty());
    }

    #[test]
    fn parses_context_definition() {
        let xml = r#"<?xml version="1.0"?>
<xbrl xmlns="http://www.xbrl.org/2003/instance" xmlns:xbrli="http://www.xbrl.org/2003/instance">
    <xbrli:context id="ctx-1">
        <xbrli:entity>
            <xbrli:identifier scheme="http://www.sec.gov/CIK">0000320193</xbrli:identifier>
        </xbrli:entity>
        <xbrli:period>
            <xbrli:instant>2023-09-30</xbrli:instant>
        </xbrli:period>
    </xbrli:context>
</xbrl>
"#;

        let handler = TestHandler::default();
        let reader = XbrlStreamReader::new(Cursor::new(xml), handler);
        let handler = reader.parse().expect("parse failed");

        assert_eq!(handler.contexts.len(), 1);
        assert_eq!(handler.contexts[0].id, "ctx-1");
    }

    #[test]
    fn parses_unit_definition() {
        let xml = r#"<?xml version="1.0"?>
<xbrl xmlns="http://www.xbrl.org/2003/instance" xmlns:xbrli="http://www.xbrl.org/2003/instance">
    <xbrli:unit id="usd">
        <xbrli:measure>iso4217:USD</xbrli:measure>
    </xbrli:unit>
</xbrl>
"#;

        let handler = TestHandler::default();
        let reader = XbrlStreamReader::new(Cursor::new(xml), handler);
        let handler = reader.parse().expect("parse failed");

        assert_eq!(handler.units.len(), 1);
        assert_eq!(handler.units[0].id, "usd");
    }
}
