//! Stable exit codes and machine-readable failures (REC-B02).
//!
//! A script has to know what went wrong without reading English prose. Every failure the command
//! line reports is one of a small, documented set of kinds, and each kind has its own exit code.
//! The numbers are part of the interface: they must only ever be added to, never reused, and a
//! change to one is a breaking change for the scripts that check it.
//!
//! | Code | Kind          | Meaning                                                            |
//! |------|---------------|--------------------------------------------------------------------|
//! | 0    | (success)     | The command did what was asked.                                    |
//! | 1    | `failed`      | Anything else: an I/O error, or a failure not classed below.       |
//! | 2    | `usage`       | The command line itself cannot be run (an option, a value, …).     |
//! | 3    | `unsupported` | This build or this tool table cannot do what was asked at all.     |
//! | 4    | `malformed`   | The input is not a document that can be read (damaged/not a PDF).  |
//! | 5    | `refused`     | The document forbids the operation (signature, permission, policy).|
//! | 6    | `partial`     | Part of the work happened; the rest was reported and skipped.      |
//!
//! `part` (6) is deliberately "some output exists": a script that sees it knows to inspect the
//! results rather than to retry the whole job. `refused` (5) is reserved for an operation the
//! document itself declines, so it is never reported for a broken invocation.

use std::io::Write as _;

/// Why a command failed, in the terms a script can act on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Failed,
    Usage,
    Unsupported,
    Malformed,
    Refused,
    Partial,
}

impl Kind {
    /// The name used on the command line and in the JSON envelope (stable).
    pub fn name(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::Usage => "usage",
            Self::Unsupported => "unsupported",
            Self::Malformed => "malformed",
            Self::Refused => "refused",
            Self::Partial => "partial",
        }
    }

    /// The process exit code for this kind (stable; add new kinds, never renumber).
    pub fn exit_code(self) -> u8 {
        match self {
            Self::Failed => 1,
            Self::Usage => 2,
            Self::Unsupported => 3,
            Self::Malformed => 4,
            Self::Refused => 5,
            Self::Partial => 6,
        }
    }
}

/// One failed command: its kind and the message a person reads.
#[derive(Clone, Debug, PartialEq)]
pub struct Failure {
    pub kind: Kind,
    pub message: String,
}

impl Failure {
    pub fn new(kind: Kind, message: impl Into<String>) -> Self {
        Self { kind, message: message.into() }
    }

    /// The `--error-format=json` line written to standard error, without a trailing newline.
    ///
    /// A single line, so a script can read it as it arrives, and no prose outside the JSON.
    pub fn json(&self) -> String {
        format!("{{\"error\":{{\"kind\":\"{}\",\"exit_code\":{},\"message\":{}}}}}", self.kind.name(), self.kind.exit_code(), quote(&self.message))
    }

    /// Report on standard error in the format asked for, and return the exit code.
    ///
    /// A closed or full standard error is not a reason to change the code: the failure already
    /// happened, and the exit code is the part a script reads.
    pub fn report(&self, json: bool) -> u8 {
        let line = if json { self.json() } else { format!("pdfcraft-cli: {}", self.message) };
        let _ = writeln!(std::io::stderr().lock(), "{line}");
        self.kind.exit_code()
    }
}

impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

/// `s` as a JSON string literal that always parses back to `s`.
///
/// Written here rather than through `serde_json` so this module has no dependencies and can be
/// compiled and tested on its own; the escaping is the JSON one (quote, backslash, the control
/// characters, and nothing else), which is also what the rest of the output uses.
fn quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The kinds as documented, with the code each one must keep.
    const DOCUMENTED: [(Kind, &str, u8); 6] = [
        (Kind::Failed, "failed", 1),
        (Kind::Usage, "usage", 2),
        (Kind::Unsupported, "unsupported", 3),
        (Kind::Malformed, "malformed", 4),
        (Kind::Refused, "refused", 5),
        (Kind::Partial, "partial", 6),
    ];

    #[test]
    fn every_kind_keeps_its_documented_code_and_name() {
        for (kind, name, code) in DOCUMENTED {
            assert_eq!(kind.exit_code(), code, "{name} changed its exit code");
            assert_eq!(kind.name(), name);
            assert_ne!(code, 0, "0 means success");
        }
        let mut codes: Vec<u8> = DOCUMENTED.iter().map(|(_, _, code)| *code).collect();
        codes.sort_unstable();
        codes.dedup();
        assert_eq!(codes.len(), DOCUMENTED.len(), "no two kinds may share a code");
    }

    #[test]
    fn the_envelope_carries_the_kind_code_and_message() {
        let failure = Failure::new(Kind::Malformed, "in.pdf: not a PDF");
        assert_eq!(failure.json(), r#"{"error":{"kind":"malformed","exit_code":4,"message":"in.pdf: not a PDF"}}"#);
    }

    #[test]
    fn a_message_that_looks_like_json_does_not_break_the_envelope() {
        // An input file's name reaches these messages; it must not be able to end the string.
        let nasty = "a\"b\\c\nd\re\tf\u{1}g\u{7f}h é 漢";
        let json = Failure::new(Kind::Failed, nasty).json();
        assert_eq!(json, "{\"error\":{\"kind\":\"failed\",\"exit_code\":1,\"message\":\"a\\\"b\\\\c\\nd\\re\\tf\\u0001g\u{7f}h é 漢\"}}");
        // One line only: the newline in the message is escaped, so the line count stays 1.
        assert_eq!(json.lines().count(), 1);
    }

    #[test]
    fn only_the_characters_json_requires_are_escaped() {
        // What must change, and nothing else: a file called `é 漢.pdf` stays readable as itself.
        let cases = [
            ("plain", "\"plain\""),
            ("", "\"\""),
            ("quote\"", "\"quote\\\"\""),
            ("back\\slash", "\"back\\\\slash\""),
            ("tab\tend", "\"tab\\tend\""),
            ("\u{0}\u{1f}", "\"\\u0000\\u001f\""),
            ("ünïcode ✓", "\"ünïcode ✓\""),
        ];
        for (message, expected) in cases {
            assert_eq!(quote(message), expected, "{message:?}");
        }
    }

    #[test]
    fn the_message_is_the_display_form() {
        let failure = Failure::new(Kind::Usage, "run: missing tool name");
        assert_eq!(failure.to_string(), "run: missing tool name");
    }
}
