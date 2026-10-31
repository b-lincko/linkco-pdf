//! Versioned tool schemas (REC-B02).
//!
//! A caller that drives these tools — a script, an agent, the command line — has to be able to
//! say which revision of the tool table it was written against, and to find out whether the build
//! in front of it speaks that revision. That is what [`TOOL_SCHEMA_VERSION`] is for: one number,
//! reported by `pdfcraft-cli schema`, by the MCP server's `initialize` and `tools/list`, and by
//! [`schema_document`], so the three cannot disagree.
//!
//! **What the version promises.** Within one major version, a change is additive: a new tool, a
//! new optional argument, a new field in a result, a new error kind. A caller that ignores what it
//! does not know keeps working. These are the changes that bump the major version, because they
//! break a caller that was working:
//!
//! * removing or renaming a tool, or making a previously optional argument required;
//! * removing a documented field from a result, or changing its meaning;
//! * changing what an existing tool does to a document (not how it describes itself — what it
//!   writes);
//! * reusing an error kind's name for a different meaning.
//!
//! The minor version goes up for additions, and for a change in behaviour a caller would notice
//! but that breaks nothing structural (a better message, a new hint). It is not a promise that
//! results are byte-identical between minor versions: the text of a message is not part of the
//! interface, and neither is the order of a list a tool did not promise to sort.
//!
//! **What is not versioned here.** The exit codes are their own contract (see
//! `pdfcraft-cli/src/exit.rs`: the numbers are never renumbered), and the MCP protocol version is
//! the protocol's, negotiated at `initialize`. A schema version says nothing about either.
//!
//! This module is data and policy, not behaviour: it reads the table [`crate::tools`] returns, so
//! the count it reports cannot drift from the tools that actually exist.

use serde_json::{Value, json};

use crate::tools::{ToolDef, tools};

/// The revision of the tool table, as `major.minor`.
///
/// Bump the minor for an addition or a harmless change, the major for anything that breaks a
/// caller. See the module documentation for the two lists.
///
/// 1.1 added the worked example a document-changing tool carries inside its schema
/// (`input_schema.examples`): additive, ignored by a caller that does not know it, and documented in
/// `docs/api-stability.md` as the kind of change that bumps the minor version rather than the major.
pub const TOOL_SCHEMA_VERSION: &str = "1.1";

/// The whole table as one document, with the version that names it.
///
/// `pdfcraft-cli schema` prints exactly this, and a test holds it to the same tools `tools/list`
/// offers (minus whatever the session's profile refuses).
pub fn schema_document() -> Value {
    let all = tools();
    json!({
        "schema_version": TOOL_SCHEMA_VERSION,
        "tool_count": all.len(),
        "tools": all.iter().map(tool_json).collect::<Vec<_>>(),
    })
}

/// One tool as the schema document lists it: the name it is called by, what it does, its arguments
/// and the two hints a caller needs to decide whether to run it unattended.
///
/// A tool that changes a document also carries a worked example inside its schema
/// (`examples`, one call that is known to work — see [`crate::examples`]), so a caller can copy a
/// call instead of guessing at one.
pub fn tool_json(t: &ToolDef) -> Value {
    json!({
        "name": t.name,
        "title": t.title,
        "description": t.description,
        "read_only": t.read_only,
        "destructive": t.destructive,
        "command": t.command,
        "input_schema": schema_with_example(t),
    })
}

/// The tool's schema with its example, when it has one. A copy: the table's own schema is never
/// changed, so two listings cannot disagree about what a tool takes.
pub fn schema_with_example(t: &ToolDef) -> Value {
    let mut schema = t.input_schema.clone();
    if let Some(example) = crate::examples::example(t.name) {
        schema["examples"] = Value::Array(vec![example]);
    }
    schema
}

/// `s` split into its major and minor numbers, if it is a version this module would accept.
///
/// Used by the tests and by anyone checking a caller's expectations against the build; a version
/// that does not parse is not a version, so it is `None` rather than a guess.
pub fn parse_version(s: &str) -> Option<(u32, u32)> {
    let (major, minor) = s.split_once('.')?;
    Some((major.parse().ok()?, minor.parse().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_version_is_a_major_and_a_minor() {
        assert_eq!(parse_version(TOOL_SCHEMA_VERSION), Some((1, 0)));
        assert_eq!(parse_version("1.10"), Some((1, 10)));
        assert_eq!(parse_version("1"), None);
        assert_eq!(parse_version("1.x"), None);
        assert_eq!(parse_version(""), None);
    }

    #[test]
    fn the_document_counts_the_tools_it_lists() {
        let document = schema_document();
        let listed = document["tools"].as_array().unwrap();
        assert_eq!(listed.len(), document["tool_count"].as_u64().unwrap() as usize);
        assert_eq!(listed.len(), tools().len());
        assert_eq!(document["schema_version"], TOOL_SCHEMA_VERSION);
    }
}
