//! Zenith Source Map
//!
//! This module provides structures for tracking source code locations (spans)
//! and managing file identifiers within the Zenith compiler.
//! Accurate source mapping is crucial for generating precise error messages
//! and debugging information.

use std::fmt;

/// Unique identifier for a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FileId(usize);

impl FileId {
    pub fn new(id: usize) -> Self {
        FileId(id)
    }

    pub fn value(self) -> usize {
        self.0
    }
}

/// A byte position within a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BytePos(pub u32);

impl BytePos {
    pub fn new(pos: u32) -> Self {
        BytePos(pos)
    }

    pub fn to_usize(self) -> usize {
        self.0 as usize
    }
}

/// Represents a contiguous region of text in a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub file: FileId,
    pub start: BytePos,
    pub end: BytePos,
    pub line: usize, // 1-indexed line number
    pub column: usize, // 1-indexed column number (of the start of the span)
}

impl Span {
    /// Creates a new Span.
    pub fn new(file: FileId, start: BytePos, end: BytePos, line: usize, column: usize) -> Self {
        Span { file, start, end, line, column }
    }

    /// Creates an empty span for error cases or placeholders.
    pub fn dummy() -> Self {
        Span {
            file: FileId::new(0),
            start: BytePos::new(0),
            end: BytePos::new(0),
            line: 0,
            column: 0,
        }
    }

    /// Returns true if the span is a dummy span.
    pub fn is_dummy(&self) -> bool {
        self.file.value() == 0 && self.start.0 == 0 && self.end.0 == 0 && self.line == 0 && self.column == 0
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileId:{}, {}:{}-{}:{}", self.file.0, self.line, self.column, self.end.0, self.end.0 - self.start.0)
    }
}

// Conceptual SourceMap for managing multiple files
#[derive(Debug, Default)]
pub struct SourceMap {
    files: Vec<String>, // Stores file content for easier lookup
    next_file_id: usize,
}

impl SourceMap {
    pub fn new() -> Self {
        SourceMap {
            files: Vec::new(),
            next_file_id: 1, // Start file IDs from 1
        }
    }

    pub fn add_file(&mut self, filename: String, content: String) -> FileId {
        let id = FileId::new(self.next_file_id);
        self.next_file_id += 1;
        self.files.push(content); // In a real compiler, we might store (filename, content)
        id
    }

    pub fn get_source(&self, file_id: FileId) -> Option<&str> {
        self.files.get(file_id.0 - 1).map(|s| s.as_str())
    }

    // A more advanced SourceMap would also store line/column offsets for faster lookup
}
