use std::fmt;
use std::sync::{Arc, Mutex};
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Level, Metadata, Subscriber};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScenarioCacheObservation {
    pub content: String,
    pub warning_operation: Option<String>,
    pub warning_message: Option<String>,
    pub warning_error: Option<String>,
}

#[derive(Debug, Default)]
struct CapturedWarning {
    operation: Option<String>,
    message: Option<String>,
    error: Option<String>,
}

pub(crate) fn capture_cache_write<F>(operation: F) -> ScenarioCacheObservation
where
    F: FnOnce() -> String,
{
    let events = Arc::new(Mutex::new(Vec::new()));
    let subscriber = CaptureSubscriber {
        events: Arc::clone(&events),
    };
    let content = tracing::subscriber::with_default(subscriber, operation);
    let warning = events.lock().ok().and_then(|mut events| events.pop());

    ScenarioCacheObservation {
        content,
        warning_operation: warning.as_ref().and_then(|event| event.operation.clone()),
        warning_message: warning.as_ref().and_then(|event| event.message.clone()),
        warning_error: warning.and_then(|event| event.error),
    }
}

struct CaptureSubscriber {
    events: Arc<Mutex<Vec<CapturedWarning>>>,
}

impl Subscriber for CaptureSubscriber {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= &Level::WARN
    }

    fn new_span(&self, _span: &Attributes<'_>) -> Id {
        Id::from_u64(1)
    }

    fn record(&self, _span: &Id, _values: &Record<'_>) {}

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {}

    fn event(&self, event: &Event<'_>) {
        if event.metadata().level() != &Level::WARN {
            return;
        }

        let mut visitor = WarningVisitor::default();
        event.record(&mut visitor);
        if let Ok(mut events) = self.events.lock() {
            events.push(visitor.warning);
        }
    }

    fn enter(&self, _span: &Id) {}

    fn exit(&self, _span: &Id) {}
}

#[derive(Default)]
struct WarningVisitor {
    warning: CapturedWarning,
}

impl WarningVisitor {
    fn record(&mut self, field: &Field, value: &str) {
        let value = normalize_debug_value(value);
        match field.name() {
            "operation" => self.warning.operation = Some(value),
            "message" => self.warning.message = Some(value),
            "error" => self.warning.error = Some(value),
            _ => {}
        }
    }
}

impl Visit for WarningVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        let value = format!("{value:?}");
        self.record(field, &value);
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.record(field, value);
    }
}

fn normalize_debug_value(value: &str) -> String {
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or(value)
        .to_string()
}
