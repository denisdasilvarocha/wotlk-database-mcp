//! Compact rendering of SQLite values.

use rusqlite::types::ValueRef;

pub(crate) fn is_default(value: ValueRef<'_>) -> bool {
    match value {
        ValueRef::Null => true,
        ValueRef::Integer(n) => n == 0,
        ValueRef::Real(n) => n == 0.0,
        ValueRef::Text(text) => text.is_empty(),
        ValueRef::Blob(_) => false,
    }
}

pub(crate) fn escape(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(ch),
        }
    }
    result
}

pub(crate) fn value(value: ValueRef<'_>) -> String {
    match value {
        ValueRef::Null => "NULL".to_owned(),
        ValueRef::Integer(n) => n.to_string(),
        ValueRef::Real(n) => n.to_string(),
        ValueRef::Text(text) => escape(&String::from_utf8_lossy(text)),
        ValueRef::Blob(bytes) => format!("<{} bytes>", bytes.len()),
    }
}
