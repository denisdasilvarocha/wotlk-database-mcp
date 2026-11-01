use std::{fs, path::Path};

use rusqlite::{Connection, params};
use serde::Deserialize;

use crate::BuildError;

#[derive(Deserialize)]
struct Codex {
    functions: Vec<Function>,
    events: Vec<Event>,
    widgets: Vec<Widget>,
    cvars: Vec<Cvar>,
    types: Vec<DataType>,
    combatlog_prefixes: Vec<Combat>,
    combatlog_events: Vec<Combat>,
    secure_templates: Vec<Template>,
}
#[derive(Deserialize)]
struct Parameter {
    name: String,
    #[serde(rename = "type")]
    kind: String,
    description: String,
    #[serde(default)]
    optional: bool,
}
#[derive(Deserialize)]
struct Example {
    title: String,
    code: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Function {
    name: String,
    signature: String,
    description: String,
    parameters: Vec<Parameter>,
    returns: Vec<Parameter>,
    category: String,
    subcategory: String,
    tags: Vec<String>,
    protected: bool,
    hw_event: bool,
    memory_address: Option<String>,
    related: Vec<String>,
    patch: String,
    documentation_url: String,
    examples: Vec<Example>,
    book_page: Option<i64>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Event {
    name: String,
    description: String,
    parameters: Vec<Parameter>,
    category: String,
    related: Vec<String>,
    patch: String,
    documentation_url: String,
    book_page: Option<i64>,
}
#[derive(Deserialize)]
struct Signature {
    signature: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Widget {
    name: String,
    description: String,
    category: String,
    inherits: Vec<String>,
    methods: Vec<Signature>,
    inherited_methods: Vec<Signature>,
    script_handlers: Vec<Signature>,
    book_page: Option<i64>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Cvar {
    name: String,
    default_value: String,
    description: String,
    category: String,
}
#[derive(Deserialize)]
struct EnumValue {
    value: String,
    description: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DataType {
    name: String,
    description: String,
    #[serde(default)]
    values: Vec<EnumValue>,
    related: Vec<String>,
    book_page: Option<i64>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Combat {
    name: String,
    description: String,
    #[serde(rename = "type")]
    kind: Option<String>,
    #[serde(default)]
    prefixes: Vec<String>,
    arguments: Vec<Parameter>,
    book_page: Option<i64>,
}
#[derive(Deserialize)]
struct Attribute {
    name: String,
    values: Vec<String>,
    description: String,
}
#[derive(Deserialize)]
struct Template {
    name: String,
    description: String,
    attributes: Vec<Attribute>,
    examples: Vec<Example>,
}

fn nonempty(s: &str) -> Option<&str> {
    (!s.is_empty()).then_some(s)
}
fn joined(values: &[String], separator: &str) -> Option<String> {
    nonempty(&values.join(separator)).map(str::to_owned)
}
fn render_parameters(values: &[Parameter]) -> Option<String> {
    let values = values
        .iter()
        .map(|p| {
            let optional = if p.optional { ", optional" } else { "" };
            let description = if p.description.is_empty() {
                String::new()
            } else {
                format!(": {}", p.description)
            };
            format!("{} ({}{optional}){description}", p.name, p.kind)
        })
        .collect::<Vec<_>>()
        .join(" | ");
    nonempty(&values).map(str::to_owned)
}
fn examples(values: &[Example]) -> Option<String> {
    let result = values
        .iter()
        .map(|e| format!("{}: {}", e.title, e.code))
        .collect::<Vec<_>>()
        .join("\n\n");
    nonempty(&result).map(str::to_owned)
}
fn signatures(values: &[Signature], trim: bool) -> Option<String> {
    let result = values
        .iter()
        .map(|v| {
            if trim {
                v.signature.split_once(" - ").map_or(v.signature.as_str(), |(s, _)| s)
            } else {
                &v.signature
            }
        })
        .collect::<Vec<_>>()
        .join(" | ");
    nonempty(&result).map(str::to_owned)
}

pub(crate) fn import(conn: &Connection, dir: &Path) -> Result<String, BuildError> {
    let commit_path = dir.join("COMMIT");
    let commit = fs::read_to_string(&commit_path).map_err(|source| BuildError::Io {
        path: commit_path.clone(),
        source,
    })?;
    let commit = commit.trim();
    if commit.is_empty() {
        return Err(BuildError::Codex {
            path: commit_path,
            message: "empty COMMIT".into(),
        });
    }
    let path = dir.join("codex.json");
    let file = fs::File::open(&path).map_err(|source| BuildError::Io {
        path: path.clone(),
        source,
    })?;
    let data: Codex = serde_json::from_reader(std::io::BufReader::new(file)).map_err(|e| BuildError::Codex {
        path,
        message: e.to_string(),
    })?;
    insert(conn, &data)?;
    Ok(commit.to_owned())
}

#[expect(
    clippy::too_many_lines,
    reason = "Keeping the fixed codex-to-SQL mappings together makes them easier to audit"
)]
fn insert(conn: &Connection, data: &Codex) -> Result<(), BuildError> {
    {
        let mut stmt = conn.prepare("INSERT INTO lua_function (name, signature, description, parameters, returns, category, subcategory, tags, protected, hw_event, address, related, patch, url, examples, book_page) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16)")?;
        for f in &data.functions {
            stmt.execute(params![
                f.name,
                nonempty(&f.signature),
                nonempty(&f.description),
                render_parameters(&f.parameters),
                render_parameters(&f.returns),
                nonempty(&f.category),
                nonempty(&f.subcategory),
                joined(&f.tags, ", "),
                f.protected.then_some(1),
                f.hw_event.then_some(1),
                f.memory_address.as_deref().and_then(nonempty),
                joined(&f.related, ", "),
                nonempty(&f.patch),
                nonempty(&f.documentation_url),
                examples(&f.examples),
                f.book_page
            ])?;
        }
    }
    {
        let mut stmt = conn.prepare("INSERT INTO lua_event (name, description, parameters, category, related, patch, url, book_page) VALUES (?1,?2,?3,?4,?5,?6,?7,?8)")?;
        for e in &data.events {
            stmt.execute(params![
                e.name,
                nonempty(&e.description),
                render_parameters(&e.parameters),
                nonempty(&e.category),
                joined(&e.related, ", "),
                nonempty(&e.patch),
                nonempty(&e.documentation_url),
                e.book_page
            ])?;
        }
    }
    {
        let mut stmt = conn.prepare("INSERT INTO widget (name, description, category, inherits, methods, inherited_methods, script_handlers, book_page) VALUES (?1,?2,?3,?4,?5,?6,?7,?8)")?;
        for w in &data.widgets {
            stmt.execute(params![
                w.name,
                nonempty(&w.description),
                nonempty(&w.category),
                joined(&w.inherits, ", "),
                signatures(&w.methods, true),
                signatures(&w.inherited_methods, true),
                signatures(&w.script_handlers, false),
                w.book_page
            ])?;
        }
    }
    {
        let mut stmt =
            conn.prepare("INSERT INTO cvar (name, default_value, description, category) VALUES (?1,?2,?3,?4)")?;
        for c in &data.cvars {
            stmt.execute(params![
                c.name,
                nonempty(&c.default_value),
                nonempty(&c.description),
                nonempty(&c.category)
            ])?;
        }
    }
    {
        let mut stmt = conn.prepare(
            "INSERT INTO lua_type (name, description, enum_values, related, book_page) VALUES (?1,?2,?3,?4,?5)",
        )?;
        for t in &data.types {
            let values = t
                .values
                .iter()
                .map(|v| format!("{}: {}", v.value, v.description))
                .collect::<Vec<_>>()
                .join(" | ");
            stmt.execute(params![
                t.name,
                nonempty(&t.description),
                nonempty(&values),
                joined(&t.related, ", "),
                t.book_page
            ])?;
        }
    }
    {
        let mut stmt = conn.prepare("INSERT INTO combatlog_event (name, type, prefixes, description, arguments, book_page) VALUES (?1,?2,?3,?4,?5,?6)")?;
        for (prefix, entries) in [(true, &data.combatlog_prefixes), (false, &data.combatlog_events)] {
            for e in entries {
                stmt.execute(params![
                    e.name,
                    if prefix { Some("prefix") } else { e.kind.as_deref() },
                    joined(&e.prefixes, ", "),
                    nonempty(&e.description),
                    render_parameters(&e.arguments),
                    e.book_page
                ])?;
            }
        }
    }
    {
        let mut stmt =
            conn.prepare("INSERT INTO secure_template (name, description, attributes, examples) VALUES (?1,?2,?3,?4)")?;
        for t in &data.secure_templates {
            let attributes = t
                .attributes
                .iter()
                .map(|a| format!("{}={}: {}", a.name, a.values.join(","), a.description))
                .collect::<Vec<_>>()
                .join(" | ");
            stmt.execute(params![
                t.name,
                nonempty(&t.description),
                nonempty(&attributes),
                examples(&t.examples)
            ])?;
        }
    }
    Ok(())
}
