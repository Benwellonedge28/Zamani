//! Zenith Universal Meta-Compiler (UMC) Source Map and Spans
//!
//! This module provides fundamental data structures for tracking source code
//! locations (spans) and managing source file information (source maps).
//! Accurate source mapping is crucial for precise error reporting, debugging,
//! and integrating with IDEs.

use std::collections::HashMap;
use std::sync::Arc; // For shared ownership of source code in SourceMap

/// Unique identifier for a source file within a compilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(pub usize);

impl FileId {
    pub fn new(id: usize) -> Self {
        FileId(id)
    }
}

/// A byte position within a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BytePos(pub u32);

impl BytePos {
    pub fn new(pos: u32) -> Self {
        BytePos(pos)
    }
}

/// Represents a contiguous region of text in a source file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    pub file_id: FileId,
    pub start: BytePos,
    pub end: BytePos,
    pub start_line: u32,
    pub start_column: u32,
}

impl Span {
    pub fn new(
        file_id: FileId,
        start: BytePos,
        end: BytePos,
        start_line: u32,
        start_column: u32,
    ) -> Self {
        Span {
            file_id,
            start,
            end,
            start_line,
            start_column,
        }
    }

    /// Creates a dummy span for cases where no real source location is available.
    pub fn dummy() -> Self {
        Span {
            file_id: FileId(0),
            start: BytePos(0),
            end: BytePos(0),
            start_line: 0,
            start_column: 0,
        }
    }

    /// Returns true if this span is a dummy span.
    pub fn is_dummy(&self) -> bool {
        self.file_id.0 == 0 && self.start.0 == 0 && self.end.0 == 0
    }

    /// Returns the length of the span in bytes.
    pub fn len(&self) -> u32 {
        self.end.0 - self.start.0
    }

    /// Returns true if the span is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Checks if this span contains another span.
    pub fn contains(&self, other: &Self) -> bool {
        self.file_id == other.file_id && self.start <= other.start && self.end >= other.end
    }

    /// Merges two spans into a single span that covers both.
    /// Panics if the spans are from different files.
    pub fn merge(&self, other: &Self) -> Self {
        assert_eq!(
            self.file_id, other.file_id,
            "Cannot merge spans from different files."
        );
        Span {
            file_id: self.file_id,
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            start_line: self.start_line.min(other.start_line), // This is an approximation
            start_column: self.start_column.min(other.start_column), // This is an approximation
        }
    }
}

/// Represents a single source file, including its name and content.
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub name: String,
    pub content: Arc<String>, // Use Arc for shared ownership without copying
    line_starts: Vec<BytePos>, // Precomputed byte offsets of the start of each line
}

impl SourceFile {
    pub fn new(name: String, content: String) -> Self {
        let mut line_starts = vec![BytePos(0)];
        for (i, c) in content.char_indices() {
            if c == '\n' {
                line_starts.push(BytePos(i as u32 + 1));
            }
        }
        SourceFile {
            name,
            content: Arc::new(content),
            line_starts,
        }
    }

    /// Get the line number and column number for a given BytePos.
    /// Returns (1-indexed line number, 1-indexed column number).
    pub fn get_line_info(&self, pos: BytePos) -> (u32, u32) {
        // Binary search to find the line number
        let line_num_idx = match self.line_starts.binary_search(&pos) {
            Ok(idx) => idx,
            Err(idx) => idx.saturating_sub(1),
        };
        let line_start_pos = self.line_starts[line_num_idx];
        let line_num = line_num_idx as u32 + 1; // 1-indexed line number
        let column_num = pos.0 - line_start_pos.0 + 1; // 1-indexed column
        (line_num, column_num)
    }

    /// Get a specific line of the source code.
    pub fn get_line(&self, line_num: u32) -> Option<&str> {
        self.content.lines().nth((line_num - 1) as usize)
    }
}

/// Manages all source files involved in a compilation.
#[derive(Debug, Clone)]
pub struct SourceMap {
    files: HashMap<FileId, Arc<SourceFile>>,
    next_file_id: usize,
}

impl SourceMap {
    pub fn new() -> Self {
        SourceMap {
            files: HashMap::new(),
            next_file_id: 1, // Start from 1, 0 is reserved for dummy
        }
    }

    /// Adds a new source file to the source map.
    pub fn add_file(&mut self, name: String, content: String) -> (FileId, Arc<SourceFile>) {
        let file_id = FileId(self.next_file_id);
        self.next_file_id += 1;
        let source_file = Arc::new(SourceFile::new(name, content));
        self.files.insert(file_id, Arc::clone(&source_file));
        (file_id, source_file)
    }

    /// Retrieves a source file by its ID.
    pub fn get_file(&self, file_id: FileId) -> Option<&Arc<SourceFile>> {
        self.files.get(&file_id)
    }

    /// Retrieves the content of a specific line from a file in the source map.
    pub fn get_source_line(&self, file_id: FileId, line_num: u32) -> Option<String> {
        self.get_file(file_id)
            .and_then(|file| file.get_line(line_num).map(String::from))
    }
}
