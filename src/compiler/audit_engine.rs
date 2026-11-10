
// ============================================================================
// Zamani Universal Compiler
// Automated Static Audit Engine
//
// File:
//     src/compiler/audit_engine.rs
//
// Rust:
//     1.97+
//
// Edition:
//     2021
//
// Safety:
//     No unsafe Rust.
//
// Architectural ownership:
//     Compiler audit evidence and static source inspection.
//
// Canonical authorities:
//     src/frontend/
//     src/frontend/semantic/
//     src/zuir/
//     src/classical/
//     src/quantum/
//     src/hdl/
//     src/ai/
//     src/hybrid/
//     src/distributed/
//     src/backend/
//     src/runtime/
//     grammar/
//
// This module inspects and reports.
// It does not replace the responsibilities of those modules.
//
// ============================================================================

use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

// ============================================================================
// AUDIT SEVERITY
// ============================================================================

/// Severity assigned to an audit finding.
///
/// The ordering is intentional and is used when sorting reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AuditSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl AuditSeverity {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "INFO",
            Self::Low => "LOW",
            Self::Medium => "MEDIUM",
            Self::High => "HIGH",
            Self::Critical => "CRITICAL",
        }
    }

    /// Whether this severity blocks the production audit gate.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::High | Self::Critical)
    }
}

// ============================================================================
// AUDIT CATEGORY
// ============================================================================

/// Classification of an audit finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuditCategory {
    MemorySafety,
    CodeGeneration,
    Isolation,
    Cryptography,
    Ffi,
    Concurrency,
    UnsafeCode,
    Scalability,
    Grammar,
    Configuration,
    AuditInfrastructure,
}

impl AuditCategory {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MemorySafety => "memory-safety",
            Self::CodeGeneration => "code-generation",
            Self::Isolation => "isolation",
            Self::Cryptography => "cryptography",
            Self::Ffi => "ffi",
            Self::Concurrency => "concurrency",
            Self::UnsafeCode => "unsafe-code",
            Self::Scalability => "scalability",
            Self::Grammar => "grammar",
            Self::Configuration => "configuration",
            Self::AuditInfrastructure => "audit-infrastructure",
        }
    }
}

// ============================================================================
// AUDIT FINDING
// ============================================================================

/// A single evidence-backed audit finding.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuditFinding {
    /// Finding category.
    pub category: AuditCategory,

    /// Finding severity.
    pub severity: AuditSeverity,

    /// Human-readable explanation.
    pub description: String,

    /// Repository-relative path.
    pub target_module: String,

    /// One-based source line, when available.
    pub line: Option<usize>,

    /// Source evidence that triggered the rule.
    pub evidence: String,

    /// Stable rule identifier.
    pub rule_id: String,
}

impl AuditFinding {
    fn new(
        category: AuditCategory,
        severity: AuditSeverity,
        description: impl Into<String>,
        target_module: String,
        line: Option<usize>,
        evidence: impl Into<String>,
        rule_id: impl Into<String>,
    ) -> Self {
        Self {
            category,
            severity,
            description: description.into(),
            target_module,
            line,
            evidence: evidence.into(),
            rule_id: rule_id.into(),
        }
    }
}

// ============================================================================
// AUDIT CONFIGURATION
// ============================================================================

/// Configuration for a static audit.
///
/// Operational budgets are configurable.
///
/// They must not be confused with language-level or hardware
/// capability limits.
///
/// The default configuration does not impose an arbitrary
/// file-size or directory-depth ceiling.
#[derive(Debug, Clone)]
pub struct AuditConfig {
    /// Maximum permitted size of an individual scanned file.
    ///
    /// The default is u64::MAX.
    ///
    /// Applications can set an explicit operational budget.
    pub max_file_size_bytes: u64,

    /// Whether hidden directories should be excluded.
    pub ignore_hidden_directories: bool,

    /// Whether generated and build directories should be excluded.
    pub ignore_build_directories: bool,

    /// Maximum directory traversal depth.
    ///
    /// The default is usize::MAX.
    ///
    /// An explicitly configured depth remains an operational
    /// traversal budget, not a language limitation.
    pub max_directory_depth: usize,

    /// Maximum characters retained as source evidence.
    pub max_evidence_length: usize,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            max_file_size_bytes: u64::MAX,
            ignore_hidden_directories: true,
            ignore_build_directories: true,
            max_directory_depth: usize::MAX,
            max_evidence_length: 240,
        }
    }
}

impl AuditConfig {
    /// Validate configuration.
    pub fn validate(&self) -> Result<(), String> {
        if self.max_file_size_bytes == 0 {
            return Err(
                "max_file_size_bytes must be greater than zero".into(),
            );
        }

        if self.max_directory_depth == 0 {
            return Err(
                "max_directory_depth must be greater than zero".into(),
            );
        }

        if self.max_evidence_length == 0 {
            return Err(
                "max_evidence_length must be greater than zero".into(),
            );
        }

        Ok(())
    }

    /// Configure the maximum scanned file size.
    #[must_use]
    pub fn with_max_file_size(mut self, bytes: u64) -> Self {
        self.max_file_size_bytes = bytes;
        self
    }

    /// Configure the maximum directory traversal depth.
    #[must_use]
    pub fn with_max_directory_depth(mut self, depth: usize) -> Self {
        self.max_directory_depth = depth;
        self
    }

    /// Configure the maximum evidence length.
    #[must_use]
    pub fn with_max_evidence_length(mut self, length: usize) -> Self {
        self.max_evidence_length = length;
        self
    }

    /// Configure hidden-directory handling.
    #[must_use]
    pub fn with_hidden_directories(mut self, ignore: bool) -> Self {
        self.ignore_hidden_directories = ignore;
        self
    }

    /// Configure build-directory handling.
    #[must_use]
    pub fn with_build_directories(mut self, ignore: bool) -> Self {
        self.ignore_build_directories = ignore;
        self
    }
}

// ============================================================================
// AUDIT SUMMARY
// ============================================================================

/// Aggregate audit statistics.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AuditSummary {
    pub files_scanned: usize,
    pub files_skipped: usize,
    pub directories_scanned: usize,
    pub directories_skipped: usize,
    pub entries_failed: usize,

    pub lines_scanned: u64,
    pub bytes_scanned: u64,

    pub findings: usize,

    pub info: usize,
    pub low: usize,
    pub medium: usize,
    pub high: usize,
    pub critical: usize,
}

impl AuditSummary {
    fn increment_files_scanned(&mut self) {
        self.files_scanned = self.files_scanned.saturating_add(1);
    }

    fn increment_files_skipped(&mut self) {
        self.files_skipped = self.files_skipped.saturating_add(1);
    }

    fn increment_directories_scanned(&mut self) {
        self.directories_scanned =
            self.directories_scanned.saturating_add(1);
    }

    fn increment_directories_skipped(&mut self) {
        self.directories_skipped =
            self.directories_skipped.saturating_add(1);
    }

    fn increment_entries_failed(&mut self) {
        self.entries_failed = self.entries_failed.saturating_add(1);
    }
}

// ============================================================================
// AUDIT REPORT
// ============================================================================

/// Complete result of a repository audit.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AuditReport {
    /// All findings.
    pub findings: Vec<AuditFinding>,

    /// Aggregate statistics.
    pub summary: AuditSummary,

    /// Filesystem and scanner failures.
    pub scan_errors: Vec<String>,
}

impl AuditReport {
    /// Whether blocking findings exist.
    pub fn has_blocking_findings(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.severity.is_blocking())
    }

    /// Whether scanner failures occurred.
    pub fn has_scan_errors(&self) -> bool {
        !self.scan_errors.is_empty()
    }

    /// Whether the audit encountered skipped source files.
    ///
    /// Build and hidden directories deliberately excluded by
    /// configuration do not count as skipped source files.
    pub fn has_skipped_files(&self) -> bool {
        self.summary.files_skipped != 0
    }

    /// Whether the audit completed without scanner errors.
    pub fn is_complete(&self) -> bool {
        !self.has_scan_errors() && !self.has_skipped_files()
    }

    /// Whether the audit is complete and has no blocking findings.
    pub fn is_clean(&self) -> bool {
        self.is_complete() && !self.has_blocking_findings()
    }

    /// Count findings at or above a severity.
    pub fn count_at_or_above(
        &self,
        severity: AuditSeverity,
    ) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.severity >= severity)
            .count()
    }

    /// Produce a deterministic textual fingerprint.
    ///
    /// This is not a cryptographic digest.
    ///
    /// Length-prefixed fields avoid ambiguity caused by separators
    /// appearing in paths, evidence or descriptions.
    pub fn fingerprint(&self) -> String {
        let mut output = String::new();

        for finding in &self.findings {
            append_field(&mut output, finding.rule_id.as_str());
            append_field(&mut output, finding.category.as_str());
            append_field(&mut output, finding.severity.as_str());
            append_field(&mut output, finding.target_module.as_str());
            append_field(
                &mut output,
                &finding.line.map(|v| v.to_string()).unwrap_or_default(),
            );
            append_field(&mut output, finding.description.as_str());
            append_field(&mut output, finding.evidence.as_str());
            output.push('\n');
        }

        for error in &self.scan_errors {
            output.push_str("ERROR:");
            append_field(&mut output, error);
            output.push('\n');
        }

        output
    }

    /// Finalize the report's ordering and counters.
    fn finalize(&mut self) {
        self.findings.sort_by(|left, right| {
            right.severity.cmp(&left.severity)
                .then_with(|| {
                    left.target_module.cmp(&right.target_module)
                })
                .then_with(|| {
                    left.line.cmp(&right.line)
                })
                .then_with(|| {
                    left.rule_id.cmp(&right.rule_id)
                })
        });

        self.scan_errors.sort();
        self.scan_errors.dedup();

        self.summary.findings = self.findings.len();

        self.summary.info = 0;
        self.summary.low = 0;
        self.summary.medium = 0;
        self.summary.high = 0;
        self.summary.critical = 0;

        for finding in &self.findings {
            match finding.severity {
                AuditSeverity::Info => {
                    self.summary.info += 1;
                }
                AuditSeverity::Low => {
                    self.summary.low += 1;
                }
                AuditSeverity::Medium => {
                    self.summary.medium += 1;
                }
                AuditSeverity::High => {
                    self.summary.high += 1;
                }
                AuditSeverity::Critical => {
                    self.summary.critical += 1;
                }
            }
        }
    }
}

fn append_field(output: &mut String, value: &str) {
    output.push_str(&value.len().to_string());
    output.push(':');
    output.push_str(value);
    output.push('|');
}

// ============================================================================
// SOURCE LANGUAGE
// ============================================================================

/// Source types supported by the audit engine.
///
/// Domain-specific grammar files are inventoried separately from
/// Rust implementation files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SourceLanguage {
    Rust,
    Grammar,
    Toml,
    Zamani,
}

fn source_language(path: &Path) -> Option<SourceLanguage> {
    match path.extension().and_then(|value| value.to_str()) {
        Some("rs") => Some(SourceLanguage::Rust),
        Some("g4") => Some(SourceLanguage::Grammar),
        Some("toml") => Some(SourceLanguage::Toml),
        Some("snk") => Some(SourceLanguage::Zamani),
        _ => None,
    }
}

// ============================================================================
// LEXICAL COMMENT FILTER
// ============================================================================

/// Lexical state retained between source lines.
///
/// The scanner does not attempt to implement the Rust or ANTLR
/// language grammars. It only protects static rules from obvious
/// comment-related false positives.
#[derive(Debug, Default)]
struct LexicalState {
    block_comment_depth: usize,
    quote: Option<char>,
    escaped: bool,
    raw_string_hashes: Option<usize>,
}

/// Remove comments while preserving string literal contents.
///
/// Supports:
/// - Single-line comments.
/// - Multiline block comments.
/// - Nested block comments.
/// - Quoted strings.
/// - Character literals.
/// - Rust raw strings.
///
/// The state is maintained across lines.
fn mask_comments(
    line: &str,
    state: &mut LexicalState,
) -> String {
    let characters: Vec<char> = line.chars().collect();
    let mut output = String::with_capacity(line.len());

    let mut index = 0;

    while index < characters.len() {
        let current = characters[index];

        // Raw string handling.
        if let Some(hashes) = state.raw_string_hashes {
            output.push(current);

            if current == '"' {
                let mut matched = true;

                for offset in 0..hashes {
                    if characters.get(index + 1 + offset) != Some(&'#') {
                        matched = false;
                        break;
                    }
                }

                if matched {
                    for _ in 0..hashes {
                        index += 1;
                        output.push('#');
                    }

                    state.raw_string_hashes = None;
                }
            }

            index += 1;
            continue;
        }

        // Normal string or character literal.
        if let Some(quote) = state.quote {
            output.push(current);

            if state.escaped {
                state.escaped = false;
            } else if current == '\\' {
                state.escaped = true;
            } else if current == quote {
                state.quote = None;
            }

            index += 1;
            continue;
        }

        // Nested block comments.
        if state.block_comment_depth != 0 {
            if current == '/'
                && characters.get(index + 1) == Some(&'*')
            {
                state.block_comment_depth =
                    state.block_comment_depth.saturating_add(1);

                output.push(' ');
                output.push(' ');
                index += 2;
                continue;
            }

            if current == '*'
                && characters.get(index + 1) == Some(&'/')
            {
                state.block_comment_depth -= 1;

                output.push(' ');
                output.push(' ');
                index += 2;
                continue;
            }

            output.push(if current == '\t' { '\t' } else { ' ' });
            index += 1;
            continue;
        }

        // Single-line comment.
        if current == '/'
            && characters.get(index + 1) == Some(&'/')
        {
            break;
        }

        // Start of a block comment.
        if current == '/'
            && characters.get(index + 1) == Some(&'*')
        {
            state.block_comment_depth = 1;
            output.push(' ');
            output.push(' ');
            index += 2;
            continue;
        }

        // Rust raw string prefix: r###"
        if current == 'r' {
            let mut cursor = index + 1;

            while characters.get(cursor) == Some(&'#') {
                cursor += 1;
            }

            if characters.get(cursor) == Some(&'"') {
                let hashes = cursor - index - 1;

                state.raw_string_hashes = Some(hashes);

                for character in &characters[index..=cursor] {
                    output.push(*character);
                }

                index = cursor + 1;
                continue;
            }
        }

        // Rust byte raw strings: br###"
        if current == 'b'
            && characters.get(index + 1) == Some(&'r')
        {
            let mut cursor = index + 2;

            while characters.get(cursor) == Some(&'#') {
                cursor += 1;
            }

            if characters.get(cursor) == Some(&'"') {
                let hashes = cursor - index - 2;

                state.raw_string_hashes = Some(hashes);

                for character in &characters[index..=cursor] {
                    output.push(*character);
                }

                index = cursor + 1;
                continue;
            }
        }

        if current == '"' {
            state.quote = Some('"');
            state.escaped = false;
            output.push(current);
            index += 1;
            continue;
        }

        // Avoid treating Rust lifetimes such as 'static as strings.
        if current == '\''
            && is_character_literal(&characters, index)
        {
            state.quote = Some('\'');
            state.escaped = false;
            output.push(current);
            index += 1;
            continue;
        }

        output.push(current);
        index += 1;
    }

    output
}

fn is_character_literal(
    characters: &[char],
    start: usize,
) -> bool {
    let mut index = start + 1;
    let mut escaped = false;

    while index < characters.len() {
        let current = characters[index];

        if escaped {
            escaped = false;
            index += 1;
            continue;
        }

        if current == '\\' {
            escaped = true;
            index += 1;
            continue;
        }

        if current == '\'' {
            return index > start + 1;
        }

        if current == '\n' || current == '\r' {
            return false;
        }

        index += 1;
    }

    false
}

// ============================================================================
// STATIC AUDIT ENGINE
// ============================================================================

/// Zamani automated static audit engine.
///
/// This engine:
///
/// - Does not modify the repository.
/// - Does not execute source code.
/// - Does not execute build scripts.
/// - Does not invoke external processes.
/// - Does not follow symbolic links.
/// - Does not create compiler artifacts.
/// - Does not replace semantic validation.
///
/// It only inspects source files and produces evidence-backed findings.
#[derive(Debug, Clone)]
pub struct ZamaniAuditEngine {
    /// Root directory being inspected.
    pub codebase_root: PathBuf,

    /// Audit configuration.
    pub config: AuditConfig,
}

impl ZamaniAuditEngine {
    /// Create an engine using default configuration.
    ///
    /// Retains the original public constructor.
    pub fn new(root: &str) -> Self {
        Self {
            codebase_root: PathBuf::from(root),
            config: AuditConfig::default(),
        }
    }

    /// Create an engine using an explicit configuration.
    pub fn with_config(
        root: impl Into<PathBuf>,
        config: AuditConfig,
    ) -> Self {
        Self {
            codebase_root: root.into(),
            config,
        }
    }

    /// Run the complete audit.
    pub fn audit(&self) -> AuditReport {
        let mut report = AuditReport::default();

        if let Err(error) = self.config.validate() {
            report.scan_errors.push(format!(
                "invalid audit configuration: {error}"
            ));

            report.finalize();
            return report;
        }

        let root = match fs::canonicalize(&self.codebase_root) {
            Ok(path) => path,
            Err(error) => {
                report.scan_errors.push(format!(
                    "failed to resolve audit root '{}': {error}",
                    self.codebase_root.display()
                ));

                report.finalize();
                return report;
            }
        };

        match fs::metadata(&root) {
            Ok(metadata) if metadata.is_dir() => {}
            Ok(_) => {
                report.scan_errors.push(format!(
                    "audit root is not a directory: '{}'",
                    root.display()
                ));

                report.finalize();
                return report;
            }
            Err(error) => {
                report.scan_errors.push(format!(
                    "failed to inspect audit root '{}': {error}",
                    root.display()
                ));

                report.finalize();
                return report;
            }
        }

        self.scan_tree(&root, &mut report);

        report.finalize();

        report
    }

    /// Backwards-compatible API.
    ///
    /// Prefer `audit()` because this method does not expose scan errors
    /// or completion status.
    pub fn run_audit(&self) -> Vec<AuditFinding> {
        self.audit().findings
    }

    // ------------------------------------------------------------------------
    // ITERATIVE TREE TRAVERSAL
    // ------------------------------------------------------------------------

    fn scan_tree(
        &self,
        root: &Path,
        report: &mut AuditReport,
    ) {
        let mut pending: Vec<(PathBuf, usize)> =
            vec![(root.to_path_buf(), 0)];

        let mut seen_findings: HashSet<AuditFinding> =
            HashSet::new();

        while let Some((directory, depth)) = pending.pop() {
            if depth > self.config.max_directory_depth {
                report.scan_errors.push(format!(
                    "maximum configured traversal depth exceeded at '{}'",
                    directory.display()
                ));

                report.summary.increment_entries_failed();
                continue;
            }

            report.summary.increment_directories_scanned();

            let entries = match fs::read_dir(&directory) {
                Ok(entries) => entries,
                Err(error) => {
                    report.scan_errors.push(format!(
                        "failed to read directory '{}': {error}",
                        directory.display()
                    ));

                    report.summary.increment_entries_failed();
                    continue;
                }
            };

            let mut paths = Vec::new();

            for entry in entries {
                match entry {
                    Ok(entry) => paths.push(entry.path()),
                    Err(error) => {
                        report.scan_errors.push(format!(
                            "failed to inspect directory entry in '{}': {error}",
                            directory.display()
                        ));

                        report.summary.increment_entries_failed();
                    }
                }
            }

            // Stable ordering.
            paths.sort();

            // Reverse push gives ascending processing order.
            for path in paths.into_iter().rev() {
                let metadata = match fs::symlink_metadata(&path) {
                    Ok(metadata) => metadata,
                    Err(error) => {
                        report.scan_errors.push(format!(
                            "failed to inspect '{}': {error}",
                            path.display()
                        ));

                        report.summary.increment_entries_failed();
                        continue;
                    }
                };

                // Never automatically follow symbolic links.
                if metadata.file_type().is_symlink() {
                    report.summary.increment_files_skipped();
                    continue;
                }

                if metadata.is_dir() {
                    if self.should_skip_directory(&path) {
                        report.summary.increment_directories_skipped();
                        continue;
                    }

                    if depth == usize::MAX {
                        report.scan_errors.push(format!(
                            "directory depth cannot be represented for '{}'",
                            path.display()
                        ));

                        report.summary.increment_entries_failed();
                        continue;
                    }

                    pending.push((path, depth + 1));
                    continue;
                }

                if !metadata.is_file() {
                    continue;
                }

                let Some(language) = source_language(&path) else {
                    continue;
                };

                self.scan_file(
                    &path,
                    root,
                    language,
                    report,
                    &mut seen_findings,
                );
            }
        }
    }

    // ------------------------------------------------------------------------
    // DIRECTORY FILTERING
    // ------------------------------------------------------------------------

    fn should_skip_directory(&self, path: &Path) -> bool {
        let Some(name) = path.file_name().and_then(|v| v.to_str()) else {
            return false;
        };

        if self.config.ignore_build_directories
            && matches!(
                name,
                "target"
                    | "node_modules"
                    | "dist"
                    | "build"
                    | ".cache"
                    | "coverage"
            )
        {
            return true;
        }

        self.config.ignore_hidden_directories
            && name.starts_with('.')
    }

    // ------------------------------------------------------------------------
    // FILE SCANNING
    // ------------------------------------------------------------------------

    fn scan_file(
        &self,
        path: &Path,
        root: &Path,
        language: SourceLanguage,
        report: &mut AuditReport,
        seen_findings: &mut HashSet<AuditFinding>,
    ) {
        let metadata = match fs::symlink_metadata(path) {
            Ok(metadata) => metadata,
            Err(error) => {
                self.record_read_error(path, error, report);
                return;
            }
        };

        if !metadata.is_file() || metadata.file_type().is_symlink() {
            report.summary.increment_files_skipped();
            return;
        }

        if metadata.len() > self.config.max_file_size_bytes {
            report.summary.increment_files_skipped();
            return;
        }

        let file = match File::open(path) {
            Ok(file) => file,
            Err(error) => {
                self.record_read_error(path, error, report);
                return;
            }
        };

        let relative_path = path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");

        let initial_findings = report.findings.len();

        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();

        let mut lexical_state = LexicalState::default();

        let mut total_bytes = 0u64;
        let mut line_number = 0usize;

        loop {
            buffer.clear();

            let bytes_read = match reader.read_until(b'\n', &mut buffer) {
                Ok(bytes) => bytes,
                Err(error) => {
                    report.findings.truncate(initial_findings);

                    self.record_read_error(path, error, report);
                    return;
                }
            };

            if bytes_read == 0 {
                break;
            }

            total_bytes = match total_bytes.checked_add(bytes_read as u64) {
                Some(value) => value,
                None => {
                    report.findings.truncate(initial_findings);

                    report.scan_errors.push(format!(
                        "byte count overflow while scanning '{}'",
                        relative_path
                    ));

                    report.summary.increment_entries_failed();
                    return;
                }
            };

            // Recheck the budget during reading. This also protects
            // against a file changing after metadata inspection.
            if total_bytes > self.config.max_file_size_bytes {
                report.findings.truncate(initial_findings);
                report.summary.increment_files_skipped();
                return;
            }

            let source_line = match std::str::from_utf8(&buffer) {
                Ok(value) => value,
                Err(_) => {
                    report.findings.truncate(initial_findings);
                    report.summary.increment_files_skipped();
                    return;
                }
            };

            line_number = line_number.saturating_add(1);

            report.summary.lines_scanned =
                report.summary.lines_scanned.saturating_add(1);

            let analysis_line = mask_comments(
                source_line,
                &mut lexical_state,
            );

            match language {
                SourceLanguage::Rust => {
                    self.check_rust(
                        &relative_path,
                        line_number,
                        &analysis_line,
                        report,
                        seen_findings,
                    );
                }

                SourceLanguage::Grammar => {
                    self.check_grammar(
                        &relative_path,
                        line_number,
                        &analysis_line,
                        report,
                        seen_findings,
                    );
                }

                SourceLanguage::Toml => {
                    self.check_toml(
                        &relative_path,
                        line_number,
                        &analysis_line,
                        report,
                        seen_findings,
                    );
                }

                SourceLanguage::Zamani => {
                    // Zamani source is inventoried and read.
                    // Language-specific semantic auditing must be
                    // supplied through a frontend-owned adapter.
                }
            }
        }

        report.summary.bytes_scanned =
            report.summary.bytes_scanned.saturating_add(total_bytes);

        report.summary.increment_files_scanned();
    }

    fn record_read_error(
        &self,
        path: &Path,
        error: io::Error,
        report: &mut AuditReport,
    ) {
        report.scan_errors.push(format!(
            "failed to read '{}': {error}",
            path.display()
        ));

        report.summary.increment_entries_failed();
    }

    // ------------------------------------------------------------------------
    // RUST AUDIT RULES
    // ------------------------------------------------------------------------

    fn check_rust(
        &self,
        path: &str,
        line: usize,
        source: &str,
        report: &mut AuditReport,
        seen: &mut HashSet<AuditFinding>,
    ) {
        self.check_unsafe_rust(path, line, source, report, seen);

        self.check_dynamic_execution(path, line, source, report, seen);

        self.check_shell_execution(path, line, source, report, seen);

        self.check_ffi(path, line, source, report, seen);

        self.check_weak_crypto(path, line, source, report, seen);

        self.check_hardcoded_resources(path, line, source, report, seen);

        self.check_unsafe_lint_suppression(
            path,
            line,
            source,
            report,
            seen,
        );
    }

    fn emit(
        &self,
        report: &mut AuditReport,
        seen: &mut HashSet<AuditFinding>,
        finding: AuditFinding,
    ) {
        if seen.insert(finding.clone()) {
            report.findings.push(finding);
        }
    }

    fn make_finding(
        &self,
        category: AuditCategory,
        severity: AuditSeverity,
        description: impl Into<String>,
        path: &str,
        line: usize,
        evidence: &str,
        rule: &'static str,
    ) -> AuditFinding {
        AuditFinding::new(
            category,
            severity,
            description,
            path.to_owned(),
            Some(line),
            truncate_evidence(
                evidence,
                self.config.max_evidence_length,
            ),
            rule,
        )
    }

    // ------------------------------------------------------------------------
    // RULE: UNSAFE RUST
    // ------------------------------------------------------------------------

    fn check_unsafe_rust(
        &self,
        path: &str,
        line: usize,
        source: &str,
        report: &mut AuditReport,
        seen: &mut HashSet<AuditFinding>,
    ) {
        if contains_identifier(source, "unsafe") {
            self.emit(
                report,
                seen,
                self.make_finding(
                    AuditCategory::UnsafeCode,
                    AuditSeverity::High,
                    "Unsafe Rust syntax was detected. Zamani forbids unsafe Rust; investigate the construct and enforce the crate safety policy.",
                    path,
                    line,
                    source,
                    "ZAAE-RUST-UNSAFE-001",
                ),
            );
        }
    }

    // ------------------------------------------------------------------------
    // RULE: UNSAFE LINT SUPPRESSION
    // ------------------------------------------------------------------------

    fn check_unsafe_lint_suppression(
        &self,
        path: &str,
        line: usize,
        source: &str,
        report: &mut AuditReport,
        seen: &mut HashSet<AuditFinding>,
    ) {
        let suppression = source.contains("allow(unsafe_code)")
            || source.contains("expect(unsafe_code)");

        if suppression {
            self.emit(
                report,
                seen,
                self.make_finding(
                    AuditCategory::UnsafeCode,
                    AuditSeverity::High,
                    "An unsafe-code lint suppression was detected. It conflicts with Zamani's crate-wide unsafe-code prohibition.",
                    path,
                    line,
                    source,
                    "ZAAE-RUST-UNSAFE-002",
                ),
            );
        }
    }

    // ------------------------------------------------------------------------
    // RULE: DYNAMIC PROCESS EXECUTION
    // ------------------------------------------------------------------------

    fn check_dynamic_execution(
        &self,
        path: &str,
        line: usize,
        source: &str,
        report: &mut AuditReport,
        seen: &mut HashSet<AuditFinding>,
    ) {
        if source.contains("Command::new")
            || source.contains("process::Command")
        {
            self.emit(
                report,
                seen,
                self.make_finding(
                    AuditCategory::CodeGeneration,
                    AuditSeverity::Medium,
                    "External process execution was detected. Validate executable selection, argument construction, environment inheritance, permissions and isolation.",
                    path,
                    line,
                    source,
                    "ZAAE-CODEGEN-001",
                ),
            );
        }
    }

    // ------------------------------------------------------------------------
    // RULE: SHELL EXECUTION
    // ------------------------------------------------------------------------

    fn check_shell_execution(
        &self,
        path: &str,
        line: usize,
        source: &str,
        report: &mut AuditReport,
        seen: &mut HashSet<AuditFinding>,
    ) {
        let patterns = [
            "Command::new(\"sh\")",
            "Command::new(\"bash\")",
            "Command::new(\"cmd\")",
            "Command::new(\"powershell\")",
            "Command::new(\"pwsh\")",
        ];

        if patterns.iter().any(|pattern| source.contains(pattern)) {
            self.emit(
                report,
                seen,
                self.make_finding(
                    AuditCategory::Isolation,
                    AuditSeverity::High,
                    "Direct shell invocation was detected. Review command construction, input validation and process isolation.",
                    path,
                    line,
                    source,
                    "ZAAE-ISOLATION-001",
                ),
            );
        }
    }

    // ------------------------------------------------------------------------
    // RULE: FFI
    // ------------------------------------------------------------------------

    fn check_ffi(
        &self,
        path: &str,
        line: usize,
        source: &str,
        report: &mut AuditReport,
        seen: &mut HashSet<AuditFinding>,
    ) {
        if source.contains("extern \"C\"")
            || source.contains("extern \"system\"")
            || source.contains("extern \"C-unwind\"")
        {
            self.emit(
                report,
                seen,
                self.make_finding(
                    AuditCategory::Ffi,
                    AuditSeverity::High,
                    "A native ABI boundary was detected. Validate ABI compatibility, ownership, lifetimes, error propagation, pointer validity and isolation.",
                    path,
                    line,
                    source,
                    "ZAAE-FFI-001",
                ),
            );
        }
    }

    // ------------------------------------------------------------------------
    // RULE: WEAK CRYPTOGRAPHIC PRIMITIVES
    // ------------------------------------------------------------------------

    fn check_weak_crypto(
        &self,
        path: &str,
        line: usize,
        source: &str,
        report: &mut AuditReport,
        seen: &mut HashSet<AuditFinding>,
    ) {
        let patterns = [
            ("md5::", "MD5"),
            ("sha1::", "SHA-1"),
        ];

        for (pattern, primitive) in patterns {
            if source.contains(pattern) {
                self.emit(
                    report,
                    seen,
                    self.make_finding(
                        AuditCategory::Cryptography,
                        AuditSeverity::Medium,
                        format!(
                            "{primitive} usage was detected. Verify that it is not used for security-sensitive authentication, integrity or cryptographic operations."
                        ),
                        path,
                        line,
                        source,
                        "ZAAE-CRYPTO-001",
                    ),
                );

                break;
            }
        }
    }

    // ------------------------------------------------------------------------
    // RULE: HARDCODED UNIVERSAL RESOURCE CEILINGS
    // ------------------------------------------------------------------------

    fn check_hardcoded_resources(
        &self,
        path: &str,
        line: usize,
        source: &str,
        report: &mut AuditReport,
        seen: &mut HashSet<AuditFinding>,
    ) {
        let trimmed = source.trim_start();

        let declaration = trimmed.starts_with("const ")
            || trimmed.starts_with("static ");

        if !declaration {
            return;
        }

        let Some(name) = declaration_name(trimmed) else {
            return;
        };

        if name.starts_with("MAX_") {
            self.emit(
                report,
                seen,
                self.make_finding(
                    AuditCategory::Scalability,
                    AuditSeverity::High,
                    "A MAX_* resource declaration was detected. Universal computing capacity must be capability-driven and resource-constrained, not governed by arbitrary language-wide ceilings.",
                    path,
                    line,
                    source,
                    "ZAAE-SCALE-001",
                ),
            );
        }
    }

    // ------------------------------------------------------------------------
    // GRAMMAR AUDIT
    // ------------------------------------------------------------------------

    fn check_grammar(
        &self,
        _path: &str,
        _line: usize,
        _source: &str,
        _report: &mut AuditReport,
        _seen: &mut HashSet<AuditFinding>,
    ) {
        // Grammar source is inventoried and checked for readability.
        //
        // It is deliberately not passed through Rust-specific rules.
        //
        // Grammar-specific semantic conformance belongs to ANTLR
        // validation and the canonical frontend grammar adapters.
        //
        // Future grammar rules must use the grammar authority model
        // documented under grammar/DESIGN.md.
    }

    // ------------------------------------------------------------------------
    // TOML AUDIT
    // ------------------------------------------------------------------------

    fn check_toml(
        &self,
        _path: &str,
        _line: usize,
        _source: &str,
        _report: &mut AuditReport,
        _seen: &mut HashSet<AuditFinding>,
    ) {
        // TOML is inventoried here.
        //
        // Dependency resolution and lockfile integrity belong to
        // Cargo and the security/supply-chain integration layer.
        //
        // Do not infer a vulnerability merely from a dependency name.
    }
}

// ============================================================================
// LEXICAL HELPERS
// ============================================================================

fn contains_identifier(source: &str, identifier: &str) -> bool {
    source.split(|character: char| {
        !character.is_ascii_alphanumeric() && character != '_'
    })
    .any(|token| token == identifier)
}

fn declaration_name(source: &str) -> Option<&str> {
    let remainder = source
        .strip_prefix("const ")
        .or_else(|| source.strip_prefix("static "))?;

    let name = remainder
        .split(|character: char| {
            !character.is_ascii_alphanumeric() && character != '_'
        })
        .next()?;

    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

/// Truncate evidence without splitting Unicode characters.
fn truncate_evidence(
    source: &str,
    maximum: usize,
) -> String {
    let trimmed = source.trim();

    let mut characters = trimmed.chars();

    let result: String = characters.by_ref().take(maximum).collect();

    if characters.next().is_some() {
        format!("{result}...")
    } else {
        result
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn new() -> Self {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos();

            let path = std::env::temp_dir().join(format!(
                "zamani-audit-{}-{timestamp}",
                std::process::id()
            ));

            fs::create_dir_all(&path)
                .expect("test directory must be created");

            Self { path }
        }

        fn write(&self, name: &str, contents: &str) {
            let path = self.path.join(name);

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .expect("parent directory must be created");
            }

            fs::write(path, contents)
                .expect("test source must be written");
        }

        fn engine(&self) -> ZamaniAuditEngine {
            ZamaniAuditEngine::with_config(
                &self.path,
                AuditConfig::default(),
            )
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    // ------------------------------------------------------------------------
    // CONFIGURATION
    // ------------------------------------------------------------------------

    #[test]
    fn default_configuration_is_valid() {
        assert!(AuditConfig::default().validate().is_ok());
    }

    #[test]
    fn default_configuration_has_no_arbitrary_depth_limit() {
        assert_eq!(
            AuditConfig::default().max_directory_depth,
            usize::MAX
        );
    }

    #[test]
    fn default_configuration_has_no_arbitrary_file_limit() {
        assert_eq!(
            AuditConfig::default().max_file_size_bytes,
            u64::MAX
        );
    }

    #[test]
    fn zero_file_budget_is_rejected() {
        let config = AuditConfig {
            max_file_size_bytes: 0,
            ..AuditConfig::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn zero_depth_budget_is_rejected() {
        let config = AuditConfig {
            max_directory_depth: 0,
            ..AuditConfig::default()
        };

        assert!(config.validate().is_err());
    }

    // ------------------------------------------------------------------------
    // SEVERITY
    // ------------------------------------------------------------------------

    #[test]
    fn severity_strings_are_stable() {
        assert_eq!(AuditSeverity::Info.as_str(), "INFO");
        assert_eq!(AuditSeverity::Low.as_str(), "LOW");
        assert_eq!(AuditSeverity::Medium.as_str(), "MEDIUM");
        assert_eq!(AuditSeverity::High.as_str(), "HIGH");
        assert_eq!(AuditSeverity::Critical.as_str(), "CRITICAL");
    }

    #[test]
    fn blocking_severities_are_correct() {
        assert!(!AuditSeverity::Info.is_blocking());
        assert!(!AuditSeverity::Low.is_blocking());
        assert!(!AuditSeverity::Medium.is_blocking());
        assert!(AuditSeverity::High.is_blocking());
        assert!(AuditSeverity::Critical.is_blocking());
    }

    // ------------------------------------------------------------------------
    // CLEAN AUDIT
    // ------------------------------------------------------------------------

    #[test]
    fn clean_repository_is_clean() {
        let directory = TestDirectory::new();

        directory.write(
            "src/main.rs",
            "fn main() { let value = 42; }\n",
        );

        let report = directory.engine().audit();

        assert_eq!(report.summary.files_scanned, 1);
        assert_eq!(report.summary.findings, 0);
        assert!(report.is_complete());
        assert!(report.is_clean());
    }

    // ------------------------------------------------------------------------
    // UNSAFE CODE
    // ------------------------------------------------------------------------

    #[test]
    fn unsafe_rust_is_detected() {
        let directory = TestDirectory::new();

        directory.write(
            "src/unsafe.rs",
            "fn dangerous() {\n    unsafe { call(); }\n}\n",
        );

        let report = directory.engine().audit();

        assert_eq!(report.summary.high, 1);

        assert_eq!(
            report.findings[0].rule_id,
            "ZAAE-RUST-UNSAFE-001"
        );

        assert_eq!(report.findings[0].line, Some(2));
    }

    #[test]
    fn unsafe_keyword_is_detected_in_declarations() {
        let directory = TestDirectory::new();

        directory.write(
            "src/unsafe.rs",
            "unsafe fn dangerous() {}\n",
        );

        let report = directory.engine().audit();

        assert_eq!(report.summary.high, 1);
    }

    #[test]
    fn unsafe_lint_suppression_is_detected() {
        let directory = TestDirectory::new();

        directory.write(
            "src/lib.rs",
            "#[allow(unsafe_code)]\n",
        );

        let report = directory.engine().audit();

        assert_eq!(report.summary.high, 1);

        assert_eq!(
            report.findings[0].rule_id,
            "ZAAE-RUST-UNSAFE-002"
        );
    }

    // ------------------------------------------------------------------------
    // PROCESS EXECUTION
    // ------------------------------------------------------------------------

    #[test]
    fn dynamic_process_execution_is_detected() {
        let directory = TestDirectory::new();

        directory.write(
            "src/process.rs",
            "Command::new(\"compiler\");\n",
        );

        let report = directory.engine().audit();

        assert_eq!(report.summary.medium, 1);
    }

    #[test]
    fn shell_execution_is_blocking() {
        let directory = TestDirectory::new();

        directory.write(
            "src/process.rs",
            "Command::new(\"sh\");\n",
        );

        let report = directory.engine().audit();

        assert!(report.has_blocking_findings());

        assert_eq!(report.summary.high, 1);
        assert_eq!(report.summary.medium, 1);
    }

    // ------------------------------------------------------------------------
    // FFI
    // ------------------------------------------------------------------------

    #[test]
    fn ffi_is_detected() {
        let directory = TestDirectory::new();

        directory.write(
            "src/native.rs",
            "unsafe extern \"C\" { fn native_call(); }\n",
        );

        let report = directory.engine().audit();

        assert!(report.has_blocking_findings());

        assert!(
            report.findings.iter().any(|finding| {
                finding.rule_id == "ZAAE-FFI-001"
            })
        );
    }

    // ------------------------------------------------------------------------
    // CRYPTOGRAPHY
    // ------------------------------------------------------------------------

    #[test]
    fn weak_crypto_is_detected() {
        let directory = TestDirectory::new();

        directory.write(
            "src/crypto.rs",
            "let digest = md5::compute(data);\n",
        );

        let report = directory.engine().audit();

        assert_eq!(report.summary.medium, 1);
    }

    // ------------------------------------------------------------------------
    // SCALABILITY
    // ------------------------------------------------------------------------

    #[test]
    fn hardcoded_resource_ceiling_is_detected() {
        let directory = TestDirectory::new();

        directory.write(
            "src/resources.rs",
            "const MAX_QUBITS: usize = 128;\n",
        );

        let report = directory.engine().audit();

        assert_eq!(report.summary.high, 1);

        assert_eq!(
            report.findings[0].rule_id,
            "ZAAE-SCALE-001"
        );
    }

    #[test]
    fn ordinary_constants_are_not_flagged() {
        let directory = TestDirectory::new();

        directory.write(
            "src/resources.rs",
            "const DEFAULT_BUFFER_SIZE: usize = 128;\n",
        );

        let report = directory.engine().audit();

        assert_eq!(report.summary.findings, 0);
    }

    // ------------------------------------------------------------------------
    // COMMENT FILTERING
    // ------------------------------------------------------------------------

    #[test]
    fn line_comments_do_not_create_findings() {
        let directory = TestDirectory::new();

        directory.write(
            "src/comments.rs",
            "// unsafe { dangerous(); }\n\
             // Command::new(\"sh\");\n\
             // extern \"C\" {}\n",
        );

        let report = directory.engine().audit();

        assert_eq!(report.summary.findings, 0);
    }

    #[test]
    fn multiline_comments_do_not_create_findings() {
        let directory = TestDirectory::new();

        directory.write(
            "src/comments.rs",
            "/*\n\
             unsafe {\n\
             Command::new(\"sh\");\n\
             }\n\
             */\n\
             fn safe() {}\n",
        );

        let report = directory.engine().audit();

        assert_eq!(report.summary.findings, 0);
    }

    #[test]
    fn nested_block_comments_are_supported() {
        let directory = TestDirectory::new();

        directory.write(
            "src/comments.rs",
            "/* outer /* unsafe { */ still comment */\n\
             fn safe() {}\n",
        );

        let report = directory.engine().audit();

        assert_eq!(report.summary.findings, 0);
    }

    #[test]
    fn raw_strings_preserve_comment_markers() {
        let directory = TestDirectory::new();

        directory.write(
            "src/raw.rs",
            "let text = r#\"// unsafe { dangerous(); }\"#;\n",
        );

        let report = directory.engine().audit();

        assert_eq!(report.summary.findings, 0);
    }

    #[test]
    fn string_literals_preserve_shell_detection() {
        let directory = TestDirectory::new();

        directory.write(
            "src/shell.rs",
            "Command::new(\"sh\");\n",
        );

        let report = directory.engine().audit();

        assert!(report.has_blocking_findings());
    }

    // ------------------------------------------------------------------------
    // GRAMMAR INVENTORY
    // ------------------------------------------------------------------------

    #[test]
    fn grammar_files_are_scanned() {
        let directory = TestDirectory::new();

        directory.write(
            "grammar/Zamani.g4",
            "grammar Zamani;\n",
        );

        let report = directory.engine().audit();

        assert_eq!(report.summary.files_scanned, 1);
        assert_eq!(report.summary.findings, 0);
    }

    #[test]
    fn grammar_text_is_not_misclassified_as_rust() {
        let directory = TestDirectory::new();

        directory.write(
            "grammar/quantum.g4",
            "// unsafe quantum operation\n\
             grammar Quantum;\n",
        );

        let report = directory.engine().audit();

        assert_eq!(report.summary.findings, 0);
    }

    // ------------------------------------------------------------------------
    // FILE BUDGETS
    // ------------------------------------------------------------------------

    #[test]
    fn configured_file_budget_is_enforced() {
        let directory = TestDirectory::new();

        directory.write(
            "src/large.rs",
            "unsafe { dangerous(); }\n",
        );

        let config = AuditConfig::default()
            .with_max_file_size(1);

        let engine = ZamaniAuditEngine::with_config(
            &directory.path,
            config,
        );

        let report = engine.audit();

        assert_eq!(report.summary.files_scanned, 0);
        assert_eq!(report.summary.files_skipped, 1);

        assert!(!report.is_complete());
        assert!(!report.is_clean());
    }

    // ------------------------------------------------------------------------
    // DIRECTORY FILTERS
    // ------------------------------------------------------------------------

    #[test]
    fn build_directories_are_skipped() {
        let directory = TestDirectory::new();

        directory.write(
            "target/generated.rs",
            "unsafe { dangerous(); }\n",
        );

        let report = directory.engine().audit();

        assert_eq!(report.summary.findings, 0);
        assert_eq!(report.summary.directories_skipped, 1);
    }

    #[test]
    fn hidden_directory_policy_is_configurable() {
        let directory = TestDirectory::new();

        directory.write(
            ".hidden/source.rs",
            "unsafe { dangerous(); }\n",
        );

        let config = AuditConfig::default()
            .with_hidden_directories(false);

        let engine = ZamaniAuditEngine::with_config(
            &directory.path,
            config,
        );

        let report = engine.audit();

        assert_eq!(report.summary.high, 1);
    }

    // ------------------------------------------------------------------------
    // INVALID ROOT
    // ------------------------------------------------------------------------

    #[test]
    fn missing_root_is_reported() {
        let root = std::env::temp_dir().join(format!(
            "zamani-audit-missing-{}",
            std::process::id()
        ));

        let engine = ZamaniAuditEngine::with_config(
            root,
            AuditConfig::default(),
        );

        let report = engine.audit();

        assert!(report.has_scan_errors());
        assert!(!report.is_clean());
    }

    // ------------------------------------------------------------------------
    // REPORT INTEGRITY
    // ------------------------------------------------------------------------

    #[test]
    fn findings_are_deduplicated() {
        let directory = TestDirectory::new();

        directory.write(
            "src/duplicate.rs",
            "unsafe {\n\
             }\n",
        );

        let report = directory.engine().audit();

        assert_eq!(report.summary.findings, 1);
    }

    #[test]
    fn fingerprint_is_deterministic() {
        let finding = AuditFinding::new(
            AuditCategory::UnsafeCode,
            AuditSeverity::High,
            "unsafe code",
            "src/test.rs".to_owned(),
            Some(10),
            "unsafe {}",
            "TEST-001",
        );

        let first = AuditReport {
            findings: vec![finding.clone()],
            ..AuditReport::default()
        };

        let second = AuditReport {
            findings: vec![finding],
            ..AuditReport::default()
        };

        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn fingerprint_distinguishes_field_boundaries() {
        let first = AuditFinding::new(
            AuditCategory::UnsafeCode,
            AuditSeverity::High,
            "a|b",
            "c",
            Some(1),
            "d",
            "RULE",
        );

        let second = AuditFinding::new(
            AuditCategory::UnsafeCode,
            AuditSeverity::High,
            "a",
            "b|c",
            Some(1),
            "d",
            "RULE",
        );

        let report_a = AuditReport {
            findings: vec![first],
            ..AuditReport::default()
        };

        let report_b = AuditReport {
            findings: vec![second],
            ..AuditReport::default()
        };

        assert_ne!(
            report_a.fingerprint(),
            report_b.fingerprint()
        );
    }

    #[test]
    fn findings_have_deterministic_order() {
        let directory = TestDirectory::new();

        directory.write(
            "src/b.rs",
            "unsafe { b(); }\n",
        );

        directory.write(
            "src/a.rs",
            "unsafe { a(); }\n",
        );

        let report = directory.engine().audit();

        assert_eq!(
            report.findings[0].target_module,
            "src/a.rs"
        );

        assert_eq!(
            report.findings[1].target_module,
            "src/b.rs"
        );
    }

    // ------------------------------------------------------------------------
    // EVIDENCE
    // ------------------------------------------------------------------------

    #[test]
    fn evidence_is_bounded() {
        let source = "a".repeat(1000);

        let evidence = truncate_evidence(&source, 100);

        assert!(evidence.chars().count() <= 103);
        assert!(evidence.ends_with("..."));
    }
}
