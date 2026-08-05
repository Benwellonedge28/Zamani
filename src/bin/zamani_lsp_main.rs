//! Minimal real Zamani Language Server (stdio, LSP-framed JSON-RPC).
//!
//! Implements just enough of the protocol (`initialize`, `textDocument/didOpen`,
//! `textDocument/didChange`) to be a genuinely useful editor backend: every time
//! a document is opened/changed it is run through the REAL Zamani lexer +
//! parser (not a mock), and any parse errors are published as LSP diagnostics.
//! This is intentionally small in scope but not a stub — it reflects the
//! actual compiler frontend's behavior.

use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::sync::Arc;

use zamani_compiler::lexer::Lexer;
use zamani_compiler::parser::Parser;
use zamani_compiler::source_map::SourceFile;

fn read_message(stdin: &mut impl Read) -> Option<String> {
    let mut content_length: Option<usize> = None;
    let mut header = Vec::new();
    loop {
        let mut byte = [0u8; 1];
        if stdin.read_exact(&mut byte).is_err() {
            return None;
        }
        header.push(byte[0]);
        if header.ends_with(b"\r\n\r\n") {
            break;
        }
    }
    let header_str = String::from_utf8_lossy(&header);
    for line in header_str.lines() {
        if let Some(v) = line.strip_prefix("Content-Length:") {
            content_length = v.trim().parse().ok();
        }
    }
    let len = content_length?;
    let mut buf = vec![0u8; len];
    stdin.read_exact(&mut buf).ok()?;
    Some(String::from_utf8_lossy(&buf).to_string())
}

fn write_message(stdout: &mut impl Write, body: &str) {
    write!(stdout, "Content-Length: {}\r\n\r\n{}", body.len(), body).ok();
    stdout.flush().ok();
}

/// Runs the real lexer+parser pipeline on `source` and returns LSP-style
/// diagnostics (0-indexed line/character) for every parser error found.
fn diagnostics_for_source(uri: &str, source: &str) -> Vec<serde_json::Value> {
    let source_file = Arc::new(SourceFile::new(uri.to_string(), source.to_string()));
    let lexer = Lexer::new(zamani_compiler::source_map::FileId::new(0), source_file);
    let mut parser = Parser::new(lexer);
    let _program = parser.parse_program();

    parser
        .get_errors()
        .iter()
        .map(|e| {
            // Span line/column are 1-indexed in the compiler; LSP wants 0-indexed.
            let line = e.span.start_line.saturating_sub(1);
            let col = e.span.start_column.saturating_sub(1);
            let end_col = col + e.span.len().max(1);
            serde_json::json!({
                "range": {
                    "start": {"line": line, "character": col},
                    "end": {"line": line, "character": end_col}
                },
                "severity": 1,
                "source": "zamani-lsp",
                "message": e.message
            })
        })
        .collect()
}

fn publish_diagnostics(stdout: &mut impl Write, uri: &str, source: &str) {
    let diags = diagnostics_for_source(uri, source);
    let notif = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/publishDiagnostics",
        "params": {"uri": uri, "diagnostics": diags}
    });
    write_message(stdout, &notif.to_string());
}

fn main() {
    eprintln!("[zamani-lsp] starting stdio LSP server (real lexer/parser backend)");
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut open_docs: HashMap<String, String> = HashMap::new();

    while let Some(msg) = read_message(&mut stdin) {
        let value: serde_json::Value = match serde_json::from_str(&msg) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let method = value.get("method").and_then(|m| m.as_str()).unwrap_or("");

        match method {
            "initialize" => {
                let id = value.get("id").cloned().unwrap_or(serde_json::json!(null));
                let resp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "capabilities": {
                            "textDocumentSync": 1,
                            "diagnosticProvider": {"interFileDependencies": false, "workspaceDiagnostics": false}
                        },
                        "serverInfo": {"name": "zamani-lsp", "version": env!("CARGO_PKG_VERSION")}
                    }
                });
                write_message(&mut stdout, &resp.to_string());
            }
            "textDocument/didOpen" => {
                if let Some(doc) = value.pointer("/params/textDocument") {
                    let uri = doc
                        .get("uri")
                        .and_then(|u| u.as_str())
                        .unwrap_or("")
                        .to_string();
                    let text = doc
                        .get("text")
                        .and_then(|t| t.as_str())
                        .unwrap_or("")
                        .to_string();
                    open_docs.insert(uri.clone(), text.clone());
                    publish_diagnostics(&mut stdout, &uri, &text);
                }
            }
            "textDocument/didChange" => {
                if let Some(uri) = value
                    .pointer("/params/textDocument/uri")
                    .and_then(|u| u.as_str())
                {
                    if let Some(changes) = value
                        .pointer("/params/contentChanges")
                        .and_then(|c| c.as_array())
                    {
                        if let Some(text) = changes
                            .last()
                            .and_then(|c| c.get("text"))
                            .and_then(|t| t.as_str())
                        {
                            open_docs.insert(uri.to_string(), text.to_string());
                            publish_diagnostics(&mut stdout, uri, text);
                        }
                    }
                }
            }
            "shutdown" => {
                let id = value.get("id").cloned().unwrap_or(serde_json::json!(null));
                let resp = serde_json::json!({"jsonrpc": "2.0", "id": id, "result": null});
                write_message(&mut stdout, &resp.to_string());
            }
            "exit" => break,
            _ => {}
        }
    }
    eprintln!("[zamani-lsp] shutting down");
}
