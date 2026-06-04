//! Zenith Standard Library: Editor Integration Interfaces
//!
//! This module defines common interfaces and data structures used for seamless
//! integration between Zenith applications/runtime and editor toolchains (LSP,
//! DAP). It ensures that complex, advanced Zenith features are visible and
//! interactable directly within the developer's environment.
//!
//! These interfaces facilitate:
//! - Custom diagnostics and warnings beyond standard language errors.
//! - Rich inline information for Zenith's unique types (const generics, linear types).
//! - Interactive previews of specialized backend DSLs (Video, Graphics, Quantum circuits).
//! - User commands for triggering toolchain actions (e.g., running property tests).

use crate::ast::Identifier;
use crate::source_map::Span;
use crate::stdlib::collections::{List, Map};
use crate::stdlib::meta_ops::MetaValue;

/// Common data structure for editor diagnostics.
#[derive(Debug, Clone, PartialEq)]
pub struct EditorDiagnostic {
    pub severity: u8, // 1: Error, 2: Warning, 3: Info, 4: Hint
    pub message: String,
    pub range: EditorRange,
    pub code: Option<String>,
    pub source: String,          // e.g., "zenithc", "zenith-test", "zenith-lsp"
    pub data: Option<MetaValue>, // Additional structured data for advanced diagnostics
}

/// Common structure for defining a region in source code.
#[derive(Debug, Clone, PartialEq)]
pub struct EditorRange {
    pub start: EditorPosition,
    pub end: EditorPosition,
}

/// Common structure for defining a position in source code.
#[derive(Debug, Clone, PartialEq)]
pub struct EditorPosition {
    pub line: u32,
    pub character: u32,
}

/// Represents a command that can be executed by the editor (e.g., from CodeLens).
#[derive(Debug, Clone, PartialEq)]
pub struct EditorCommand {
    pub title: String,
    pub command_id: String, // Unique ID for the command (e.g., "zenith.runProperty")
    pub arguments: List<MetaValue>, // Arguments for the command
}

/// Represents data associated with a CodeLens item.
#[derive(Debug, Clone, PartialEq)]
pub struct EditorCodeLensData {
    pub kind: String,      // e.g., "property", "backendDsl", "effectInfo"
    pub entity_id: String, // Fully qualified ID of the associated entity
    pub additional_info: Map<String, MetaValue>, // Any extra data specific to the kind
}

/// Interface for displaying custom interactive content in the editor (e.g., webviews).
pub struct CustomEditorDisplay {
    pub display_id: Identifier,
    pub content_type: String, // e.g., "webview/html", "image/png", "3d/model"
    pub payload: MetaValue,
    pub title: String,
}

impl CustomEditorDisplay {
    pub fn new(id: Identifier, content_type: String, payload: MetaValue, title: String) -> Self {
        CustomEditorDisplay {
            display_id: id,
            content_type,
            payload,
            title,
        }
    }
}

/// Initializes the Editor Integration module.
pub fn init_editor_integration() {
    println!("  - Initializing Zenith StdLib Editor Integration Interfaces...");
}

/// Shuts down the Editor Integration module.
pub fn shutdown_editor_integration() {
    println!("  - Shutting down Zenith StdLib Editor Integration Interfaces...");
}
