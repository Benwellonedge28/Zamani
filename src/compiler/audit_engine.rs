//! Zamani Compiler — Automated Static Audit Engine (ZAAE).
//!
//! ZAAE performs deterministic, local, non-mutating static analysis of a
//! Zamani source tree.
//!
//! Design goals:
//!
//! - deterministic traversal and finding ordering;
//! - no external dependencies;
//! - bounded file reads;
//! - explicit scan errors;
//! - no silent filesystem failures;
//! - no accidental symlink traversal;
//! - stable rule identifiers;
//! - CI-friendly blocking semantics;
//! - evidence-backed findings only;
//! - conservative security classifications;
//! - backwards-compatible `run_audit()` API.
//!
//! This module does NOT prove that the compiler or generated programs are
//! secure. It is a static analysis layer and must be complemented by:
//!
//! - compiler tests;
//! - fuzzing;
//! - dynamic analysis;
//! - dependency auditing;
//! - sandbox testing;
//! - formal verification where appropriate;
//! - target-specific security review.
//!
//! The scanner intentionally avoids treating generic filesystem writes,
//! ordinary randomness, comments, or arbitrary text as automatically
//! security vulnerabilities.

use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Severity assigned to an audit finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AuditSeverity {
    /// Informational observation.
    Info,

    /// Potential concern requiring review.
    Low,

    /// Significant correctness or security concern.
    Medium,

    /// High-impact concern that should normally block production.
    High,

    /// Critical issue that must block a production security gate.
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

    /// Whether this severity blocks a production audit gate.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::High | Self::Critical)
    }
}

/// Category assigned to an audit finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuditCategory {
    /// Unsafe memory or pointer operations.
    MemorySafety,

    /// Dynamic execution or generated-code concerns.
    CodeGeneration,

    /// Filesystem/process isolation concerns.
    Isolation,

    /// Cryptographic/security-sensitive implementation concerns.
    Cryptography,

    /// FFI/native ABI boundary concerns.
    Ffi,

    /// Concurrency/atomicity concerns.
    Concurrency,

    /// Explicit unsafe Rust usage.
    UnsafeCode,

    /// Audit infrastructure/scanner failure.
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
            Self::AuditInfrastructure => "audit-infrastructure",
        }
    }
}

/// A single statically detected audit finding.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuditFinding {
    /// Stable category.
    pub category: AuditCategory,

    /// Finding severity.
    pub severity: AuditSeverity,

    /// Human-readable description.
    pub description: String,

    /// Repository-relative target path.
    pub target_module: String,

    /// One-based source line.
    pub line: Option<usize>,

    /// Evidence that triggered the rule.
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

/// Configuration controlling an audit.
#[derive(Debug, Clone)]
pub struct AuditConfig {
    /// Maximum size of an individual file that can be scanned.
    pub max_file_size_bytes: u64,

    /// Ignore hidden directories.
    pub ignore_hidden_directories: bool,

    /// Ignore generated/build directories.
    pub ignore_build_directories: bool,

    /// Maximum directory traversal depth.
    ///
    /// `0` means only the root directory is scanned.
    pub max_directory_depth: usize,

    /// Maximum source line length retained as evidence.
    pub max_evidence_length: usize,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            max_file_size_bytes: 4 * 1024 * 1024,
            ignore_hidden_directories: true,
            ignore_build_directories: true,
            max_directory_depth: 128,
            max_evidence_length: 240,
        }
    }
}

impl AuditConfig {
    /// Validate configuration before scanning.
    pub fn validate(&self) -> Result<(), String> {
        if self.max_file_size_bytes == 0 {
            return Err(
                "max_file_size_bytes must be greater than zero".to_string(),
            );
        }

        if self.max_directory_depth == 0 {
            return Err(
                "max_directory_depth must be greater than zero".to_string(),
            );
        }

        if self.max_evidence_length == 0 {
            return Err(
                "max_evidence_length must be greater than zero".to_string(),
            );
        }

        Ok(())
    }
}

/// Summary returned by an audit.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AuditSummary {
    /// Number of successfully scanned files.
    pub files_scanned: usize,

    /// Number of files skipped because of configuration, size, or encoding.
    pub files_skipped: usize,

    /// Number of directories skipped.
    pub directories_skipped: usize,

    /// Number of filesystem entries that could not be inspected.
    pub entries_failed: usize,

    /// Total findings.
    pub findings: usize,

    /// Informational findings.
    pub info: usize,

    /// Low-severity findings.
    pub low: usize,

    /// Medium-severity findings.
    pub medium: usize,

    /// High-severity findings.
    pub high: usize,

    /// Critical findings.
    pub critical: usize,
}

/// Complete result of an audit.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AuditReport {
    /// All findings discovered by the scan.
    pub findings: Vec<AuditFinding>,

    /// Aggregate counts.
    pub summary: AuditSummary,

    /// Filesystem/scanner failures.
    pub scan_errors: Vec<String>,
}

impl AuditReport {
    /// Returns true when at least one High or Critical finding exists.
    pub fn has_blocking_findings(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.severity.is_blocking())
    }

    /// Returns true when scanner errors occurred.
    pub fn has_scan_errors(&self) -> bool {
        !self.scan_errors.is_empty()
    }

    /// Returns true only if the audit completed without scanner errors and
    /// without High/Critical findings.
    pub fn is_clean(&self) -> bool {
        !self.has_scan_errors() && !self.has_blocking_findings()
    }

    /// Returns the number of findings at or above the requested severity.
    pub fn count_at_or_above(&self, severity: AuditSeverity) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.severity >= severity)
            .count()
    }

    /// Returns a deterministic textual fingerprint of the findings.
    ///
    /// This is intentionally a simple standard-library-only fingerprint
    /// representation. It is suitable for comparing audit results but is not
    /// intended as a cryptographic hash.
    pub fn fingerprint(&self) -> String {
        let mut value = String::new();

        for finding in &self.findings {
            value.push_str(finding.rule_id.as_str());
            value.push('|');
            value.push_str(finding.target_module.as_str());
            value.push('|');

            if let Some(line) = finding.line {
                value.push_str(&line.to_string());
            }

            value.push('|');
            value.push_str(finding.evidence.as_str());
            value.push('\n');
        }

        value
    }
}

/// Static audit engine for a Zamani source tree.
#[derive(Debug, Clone)]
pub struct ZamaniAuditEngine {
    /// Root directory being scanned.
    pub codebase_root: PathBuf,

    /// Scanner configuration.
    pub config: AuditConfig,
}

impl ZamaniAuditEngine {
    /// Create an audit engine with production defaults.
    pub fn new(root: &str) -> Self {
        Self {
            codebase_root: PathBuf::from(root),
            config: AuditConfig::default(),
        }
    }

    /// Create an audit engine with explicit configuration.
    pub fn with_config(
        root: impl Into<PathBuf>,
        config: AuditConfig,
    ) -> Self {
        Self {
            codebase_root: root.into(),
            config,
        }
    }

    /// Run the complete static audit.
    pub fn audit(&self) -> AuditReport {
        let mut report = AuditReport::default();

        if let Err(error) = self.config.validate() {
            report.scan_errors.push(format!(
                "invalid audit configuration: {}",
                error
            ));
            return report;
        }

        let root = match fs::canonicalize(&self.codebase_root) {
            Ok(root) => root,
            Err(error) => {
                report.scan_errors.push(format!(
                    "failed to resolve audit root '{}': {}",
                    self.codebase_root.display(),
                    error
                ));
                return report;
            }
        };

        match fs::metadata(&root) {
            Ok(metadata) if metadata.is_dir() => {}
            Ok(_) => {
                report.scan_errors.push(format!(
                    "audit root is not a directory: {}",
                    root.display()
                ));
                return report;
            }
            Err(error) => {
                report.scan_errors.push(format!(
                    "failed to inspect audit root '{}': {}",
                    root.display(),
                    error
                ));
                return report;
            }
        }

        self.scan_directory(&root, 0, &root, &mut report);

        self.sort_and_finalize(&mut report);

        report
    }

    /// Backwards-compatible audit API.
    pub fn run_audit(&self) -> Vec<AuditFinding> {
        self.audit().findings
    }

    fn scan_directory(
        &self,
        directory: &Path,
        depth: usize,
        root: &Path,
        report: &mut AuditReport,
    ) {
        if depth > self.config.max_directory_depth {
            report.scan_errors.push(format!(
                "maximum directory traversal depth exceeded at '{}'",
                directory.display()
            ));
            report.summary.entries_failed += 1;
            return;
        }

        let entries = match fs::read_dir(directory) {
            Ok(entries) => entries,
            Err(error) => {
                report.scan_errors.push(format!(
                    "failed to read directory '{}': {}",
                    directory.display(),
                    error
                ));
                report.summary.entries_failed += 1;
                return;
            }
        };

        let mut paths = Vec::new();

        for entry in entries {
            match entry {
                Ok(entry) => paths.push(entry.path()),
                Err(error) => {
                    report.scan_errors.push(format!(
                        "failed to inspect directory entry in '{}': {}",
                        directory.display(),
                        error
                    ));
                    report.summary.entries_failed += 1;
                }
            }
        }

        paths.sort();

        for path in paths {
            let metadata = match fs::symlink_metadata(&path) {
                Ok(metadata) => metadata,
                Err(error) => {
                    report.scan_errors.push(format!(
                        "failed to inspect '{}': {}",
                        path.display(),
                        error
                    ));
                    report.summary.entries_failed += 1;
                    continue;
                }
            };

            // Never follow symbolic links automatically. This prevents a
            // repository-local symlink from escaping the audit root.
            if metadata.file_type().is_symlink() {
                report.summary.files_skipped += 1;
                continue;
            }

            if metadata.is_dir() {
                if self.should_skip_directory(&path) {
                    report.summary.directories_skipped += 1;
                    continue;
                }

                self.scan_directory(
                    &path,
                    depth + 1,
                    root,
                    report,
                );

                continue;
            }

            if !metadata.is_file() {
                continue;
            }

            if !self.is_source_file(&path) {
                continue;
            }

            self.scan_file(&path, root, report);
        }
    }

    fn should_skip_directory(&self, path: &Path) -> bool {
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
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

    fn is_source_file(&self, path: &Path) -> bool {
        matches!(
            path.extension()
                .and_then(|extension| extension.to_str()),
            Some("rs")
                | Some("snk")
                | Some("g4")
                | Some("toml")
        )
    }

    fn scan_file(
        &self,
        path: &Path,
        root: &Path,
        report: &mut AuditReport,
    ) {
        let metadata = match fs::metadata(path) {
            Ok(metadata) => metadata,
            Err(error) => {
                report.scan_errors.push(format!(
                    "failed to stat '{}': {}",
                    path.display(),
                    error
                ));
                report.summary.entries_failed += 1;
                return;
            }
        };

        if metadata.len() > self.config.max_file_size_bytes {
            report.summary.files_skipped += 1;
            return;
        }

        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(error) => {
                report.scan_errors.push(format!(
                    "failed to read '{}': {}",
                    path.display(),
                    error
                ));
                report.summary.entries_failed += 1;
                return;
            }
        };

        let source = match String::from_utf8(bytes) {
            Ok(source) => source,
            Err(_) => {
                report.summary.files_skipped += 1;
                return;
            }
        };

        report.summary.files_scanned += 1;

        let relative_path = path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");

        for (index, line) in source.lines().enumerate() {
            let line_number = index + 1;

            // Static rules intentionally operate on source lines. We first
            // remove obvious comments to reduce false positives while keeping
            // string literals intact.
            let analysis_line = strip_line_comment(line);

            self.check_unsafe_rust(
                &relative_path,
                line_number,
                &analysis_line,
                report,
            );

            self.check_dynamic_execution(
                &relative_path,
                line_number,
                &analysis_line,
                report,
            );

            self.check_raw_ffi(
                &relative_path,
                line_number,
                &analysis_line,
                report,
            );

            self.check_shell_execution(
                &relative_path,
                line_number,
                &analysis_line,
                report,
            );

            self.check_weak_crypto(
                &relative_path,
                line_number,
                &analysis_line,
                report,
            );
        }
    }

    fn check_unsafe_rust(
        &self,
        path: &str,
        line: usize,
        source_line: &str,
        report: &mut AuditReport,
    ) {
        let trimmed = source_line.trim_start();

        if source_line.contains("unsafe {")
            || trimmed.starts_with("unsafe fn ")
            || trimmed.starts_with("unsafe impl ")
        {
            self.push_unique(
                report,
                AuditFinding::new(
                    AuditCategory::UnsafeCode,
                    AuditSeverity::Medium,
                    "Unsafe Rust code requires explicit security review and should be isolated to the smallest possible scope.",
                    path.to_string(),
                    Some(line),
                    self.evidence(source_line),
                    "ZAAE-RUST-UNSAFE-001",
                ),
            );
        }
    }

    fn check_dynamic_execution(
        &self,
        path: &str,
        line: usize,
        source_line: &str,
        report: &mut AuditReport,
    ) {
        let process_patterns = [
            "std::process::Command::new",
            "std::process::Command::new",
            "Command::new",
        ];

        if process_patterns
            .iter()
            .any(|pattern| source_line.contains(pattern))
        {
            self.push_unique(
                report,
                AuditFinding::new(
                    AuditCategory::CodeGeneration,
                    AuditSeverity::Medium,
                    "External process execution was detected. Production compiler paths must validate arguments, constrain executable selection, and preserve process isolation.",
                    path.to_string(),
                    Some(line),
                    self.evidence(source_line),
                    "ZAAE-CODEGEN-001",
                ),
            );
        }
    }

    fn check_raw_ffi(
        &self,
        path: &str,
        line: usize,
        source_line: &str,
        report: &mut AuditReport,
    ) {
        if source_line.contains("extern \"C\"")
            || source_line.contains("extern \"system\"")
        {
            self.push_unique(
                report,
                AuditFinding::new(
                    AuditCategory::Ffi,
                    AuditSeverity::High,
                    "A native FFI boundary was detected. Production code requires explicit ABI, ownership, lifetime, and error-boundary validation.",
                    path.to_string(),
                    Some(line),
                    self.evidence(source_line),
                    "ZAAE-FFI-001",
                ),
            );
        }
    }

    fn check_shell_execution(
        &self,
        path: &str,
        line: usize,
        source_line: &str,
        report: &mut AuditReport,
    ) {
        let shell_patterns = [
            "Command::new(\"sh\")",
            "Command::new(\"bash\")",
            "Command::new(\"cmd\")",
            "Command::new(\"powershell\")",
        ];

        if shell_patterns
            .iter()
            .any(|pattern| source_line.contains(pattern))
        {
            self.push_unique(
                report,
                AuditFinding::new(
                    AuditCategory::Isolation,
                    AuditSeverity::High,
                    "Direct shell invocation was detected. Prefer direct executable invocation and never concatenate untrusted input into shell commands.",
                    path.to_string(),
                    Some(line),
                    self.evidence(source_line),
                    "ZAAE-ISOLATION-001",
                ),
            );
        }
    }

    fn check_weak_crypto(
        &self,
        path: &str,
        line: usize,
        source_line: &str,
        report: &mut AuditReport,
    ) {
        let patterns = [
            ("md5::", "MD5"),
            ("sha1::", "SHA-1"),
        ];

        for (pattern, primitive) in patterns {
            if source_line.contains(pattern) {
                self.push_unique(
                    report,
                    AuditFinding::new(
                        AuditCategory::Cryptography,
                        AuditSeverity::Medium,
                        format!(
                            "Detected {}. Verify that it is not being used for security-sensitive integrity, authentication, password hashing, or cryptographic purposes.",
                            primitive
                        ),
                        path.to_string(),
                        Some(line),
                        self.evidence(source_line),
                        "ZAAE-CRYPTO-001",
                    ),
                );

                break;
            }
        }
    }

    fn evidence(&self, line: &str) -> String {
        truncate_evidence(line, self.config.max_evidence_length)
    }

    fn push_unique(
        &self,
        report: &mut AuditReport,
        finding: AuditFinding,
    ) {
        if !report.findings.contains(&finding) {
            report.findings.push(finding);
        }
    }

    fn sort_and_finalize(&self, report: &mut AuditReport) {
        report.findings.sort_by(|left, right| {
            match right.severity.cmp(&left.severity) {
                Ordering::Equal => {
                    match left.target_module.cmp(&right.target_module) {
                        Ordering::Equal => {
                            match left.line.unwrap_or(0).cmp(
                                &right.line.unwrap_or(0),
                            ) {
                                Ordering::Equal => {
                                    left.rule_id.cmp(&right.rule_id)
                                }
                                ordering => ordering,
                            }
                        }
                        ordering => ordering,
                    }
                }
                ordering => ordering,
            }
        });

        report.summary.findings = report.findings.len();

        report.summary.info = 0;
        report.summary.low = 0;
        report.summary.medium = 0;
        report.summary.high = 0;
        report.summary.critical = 0;

        for finding in &report.findings {
            match finding.severity {
                AuditSeverity::Info => report.summary.info += 1,
                AuditSeverity::Low => report.summary.low += 1,
                AuditSeverity::Medium => report.summary.medium += 1,
                AuditSeverity::High => report.summary.high += 1,
                AuditSeverity::Critical => report.summary.critical += 1,
            }
        }
    }
}

/// Remove a conventional single-line comment.
///
/// This is deliberately conservative and only strips comments where doing so
/// is unambiguous enough for the simple rule engine. It does not attempt to be
/// a full Rust/Shona/ANTLR lexer.
fn strip_line_comment(line: &str) -> String {
    let mut in_string = false;
    let mut escaped = false;
    let mut previous = '\0';

    for (index, character) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }

        if character == '\\' && in_string {
            escaped = true;
            continue;
        }

        if character == '"' {
            in_string = !in_string;
            continue;
        }

        if character == '/'
            && previous == '/'
            && !in_string
        {
            return line[..index.saturating_sub(1)]
                .trim_end()
                .to_string();
        }

        previous = character;
    }

    line.to_string()
}

/// Truncate evidence without splitting UTF-8 characters.
fn truncate_evidence(
    line: &str,
    max_length: usize,
) -> String {
    let trimmed = line.trim();

    if trimmed.chars().count() <= max_length {
        return trimmed.to_string();
    }

    let mut result = trimmed
        .chars()
        .take(max_length)
        .collect::<String>();

    result.push_str("...");

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn new() -> Self {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock must be after UNIX epoch")
                .as_nanos();

            let path = std::env::temp_dir().join(format!(
                "zamani-audit-test-{}-{}",
                std::process::id(),
                timestamp
            ));

            fs::create_dir_all(&path)
                .expect("test directory should be created");

            Self { path }
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn severity_strings_are_stable() {
        assert_eq!(AuditSeverity::Info.as_str(), "INFO");
        assert_eq!(AuditSeverity::Low.as_str(), "LOW");
        assert_eq!(AuditSeverity::Medium.as_str(), "MEDIUM");
        assert_eq!(AuditSeverity::High.as_str(), "HIGH");
        assert_eq!(
            AuditSeverity::Critical.as_str(),
            "CRITICAL"
        );
    }

    #[test]
    fn high_and_critical_are_blocking() {
        assert!(!AuditSeverity::Info.is_blocking());
        assert!(!AuditSeverity::Low.is_blocking());
        assert!(!AuditSeverity::Medium.is_blocking());
        assert!(AuditSeverity::High.is_blocking());
        assert!(AuditSeverity::Critical.is_blocking());
    }

    #[test]
    fn default_configuration_is_valid() {
        assert!(
            AuditConfig::default().validate().is_ok()
        );
    }

    #[test]
    fn zero_file_size_is_rejected() {
        let config = AuditConfig {
            max_file_size_bytes: 0,
            ..AuditConfig::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn zero_depth_is_rejected() {
        let config = AuditConfig {
            max_directory_depth: 0,
            ..AuditConfig::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn missing_root_is_reported() {
        let root = std::env::temp_dir().join(format!(
            "zamani-missing-audit-root-{}",
            std::process::id()
        ));

        let engine =
            ZamaniAuditEngine::new(root.to_string_lossy().as_ref());

        let report = engine.audit();

        assert!(report.has_scan_errors());
        assert!(!report.is_clean());
    }

    #[test]
    fn clean_source_is_clean() {
        let directory = TestDirectory::new();

        fs::write(
            directory.path.join("clean.rs"),
            "fn main() { let value = 42; }",
        )
        .expect("source should be written");

        let engine =
            ZamaniAuditEngine::new(
                directory.path.to_string_lossy().as_ref(),
            );

        let report = engine.audit();

        assert_eq!(report.summary.files_scanned, 1);
        assert_eq!(report.summary.findings, 0);
        assert!(report.is_clean());
    }

    #[test]
    fn unsafe_code_is_reported() {
        let directory = TestDirectory::new();

        fs::write(
            directory.path.join("unsafe.rs"),
            "fn main() {\n    unsafe { dangerous(); }\n}\n",
        )
        .expect("source should be written");

        let engine =
            ZamaniAuditEngine::new(
                directory.path.to_string_lossy().as_ref(),
            );

        let report = engine.audit();

        assert_eq!(report.summary.files_scanned, 1);
        assert_eq!(report.summary.medium, 1);
        assert_eq!(
            report.findings[0].category,
            AuditCategory::UnsafeCode
        );
        assert_eq!(
            report.findings[0].rule_id,
            "ZAAE-RUST-UNSAFE-001"
        );
        assert_eq!(report.findings[0].line, Some(2));
    }

    #[test]
    fn shell_execution_is_blocking() {
        let directory = TestDirectory::new();

        fs::write(
            directory.path.join("shell.rs"),
            "use std::process::Command;\n\
             fn run() { Command::new(\"sh\"); }\n",
        )
        .expect("source should be written");

        let engine =
            ZamaniAuditEngine::new(
                directory.path.to_string_lossy().as_ref(),
            );

        let report = engine.audit();

        assert!(report.has_blocking_findings());
        assert_eq!(report.summary.high, 1);
    }

    #[test]
    fn ffi_is_blocking() {
        let directory = TestDirectory::new();

        fs::write(
            directory.path.join("ffi.rs"),
            "extern \"C\" { fn native_call(); }\n",
        )
        .expect("source should be written");

        let engine =
            ZamaniAuditEngine::new(
                directory.path.to_string_lossy().as_ref(),
            );

        let report = engine.audit();

        assert!(report.has_blocking_findings());
        assert_eq!(report.summary.high, 1);
        assert_eq!(
            report.findings[0].category,
            AuditCategory::Ffi
        );
    }

    #[test]
    fn weak_crypto_is_reported() {
        let directory = TestDirectory::new();

        fs::write(
            directory.path.join("crypto.rs"),
            "let digest = md5::compute(data);\n",
        )
        .expect("source should be written");

        let engine =
            ZamaniAuditEngine::new(
                directory.path.to_string_lossy().as_ref(),
            );

        let report = engine.audit();

        assert_eq!(report.summary.medium, 1);
        assert_eq!(
            report.findings[0].category,
            AuditCategory::Cryptography
        );
    }

    #[test]
    fn comments_do_not_create_findings() {
        let directory = TestDirectory::new();

        fs::write(
            directory.path.join("comments.rs"),
            "// unsafe { dangerous(); }\n\
             // Command::new(\"sh\");\n\
             // extern \"C\" {}\n",
        )
        .expect("source should be written");

        let engine =
            ZamaniAuditEngine::new(
                directory.path.to_string_lossy().as_ref(),
            );

        let report = engine.audit();

        assert_eq!(report.summary.findings, 0);
        assert!(report.is_clean());
    }

    #[test]
    fn duplicate_findings_are_removed() {
        let directory = TestDirectory::new();

        fs::write(
            directory.path.join("duplicate.rs"),
            "unsafe {\n}\n",
        )
        .expect("source should be written");

        let engine =
            ZamaniAuditEngine::new(
                directory.path.to_string_lossy().as_ref(),
            );

        let report = engine.audit();

        assert_eq!(report.findings.len(), 1);
    }

    #[test]
    fn large_files_are_skipped() {
        let directory = TestDirectory::new();

        fs::write(
            directory.path.join("large.rs"),
            "1234567890",
        )
        .expect("source should be written");

        let config = AuditConfig {
            max_file_size_bytes: 1,
            ..AuditConfig::default()
        };

        let engine = ZamaniAuditEngine::with_config(
            &directory.path,
            config,
        );

        let report = engine.audit();

        assert_eq!(report.summary.files_scanned, 0);
        assert_eq!(report.summary.files_skipped, 1);
    }

    #[test]
    fn non_source_files_are_ignored() {
        let directory = TestDirectory::new();

        fs::write(
            directory.path.join("notes.txt"),
            "unsafe { dangerous(); }",
        )
        .expect("file should be written");

        let engine =
            ZamaniAuditEngine::new(
                directory.path.to_string_lossy().as_ref(),
            );

        let report = engine.audit();

        assert_eq!(report.summary.files_scanned, 0);
        assert_eq!(report.summary.findings, 0);
    }

    #[test]
    fn generated_directories_are_skipped() {
        let directory = TestDirectory::new();

        let target = directory.path.join("target");

        fs::create_dir_all(&target)
            .expect("target directory should be created");

        fs::write(
            target.join("generated.rs"),
            "unsafe { dangerous(); }",
        )
        .expect("generated source should be written");

        let engine =
            ZamaniAuditEngine::new(
                directory.path.to_string_lossy().as_ref(),
            );

        let report = engine.audit();

        assert_eq!(report.summary.findings, 0);
        assert_eq!(
            report.summary.directories_skipped,
            1
        );
    }

    #[test]
    fn evidence_is_bounded() {
        let line = "a".repeat(1000);

        let evidence = truncate_evidence(&line, 100);

        assert!(evidence.len() <= 103);
        assert!(evidence.ends_with("..."));
    }

    #[test]
    fn fingerprint_is_deterministic() {
        let finding = AuditFinding::new(
            AuditCategory::UnsafeCode,
            AuditSeverity::Medium,
            "description",
            "src/test.rs".to_string(),
            Some(10),
            "unsafe {}",
            "TEST-001",
        );

        let mut first = AuditReport::default();
        first.findings.push(finding.clone());

        let mut second = AuditReport::default();
        second.findings.push(finding);

        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn source_path_order_is_deterministic() {
        let directory = TestDirectory::new();

        fs::write(
            directory.path.join("b.rs"),
            "unsafe { b(); }",
        )
        .expect("b.rs should be written");

        fs::write(
            directory.path.join("a.rs"),
            "unsafe { a(); }",
        )
        .expect("a.rs should be written");

        let engine =
            ZamaniAuditEngine::new(
                directory.path.to_string_lossy().as_ref(),
            );

        let report = engine.audit();

        assert_eq!(
            report.findings[0].target_module,
            "a.rs"
        );

        assert_eq!(
            report.findings[1].target_module,
            "b.rs"
        );
    }
}