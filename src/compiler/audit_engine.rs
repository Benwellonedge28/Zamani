//! Zamani Compiler — Automated Static Audit Engine (ZAAE).
//!
//! The audit engine performs deterministic, local static analysis of a Zamani
//! source tree. It is intentionally conservative: it reports evidence found
//! in the scanned source instead of inventing findings.
//!
//! The engine is:
//! - deterministic;
//! - filesystem-local;
//! - dependency-free;
//! - non-mutating;
//! - suitable for CI;
//! - resilient to unreadable/binary files;
//! - explicit about scan errors;
//! - extensible through small independent rules.
//!
//! This module is a static audit engine. It does NOT claim to prove that a
//! program or compiler backend is secure. Formal verification, dynamic
//! analysis, fuzzing, sandbox testing, and target-specific security review
//! remain separate concerns.

use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};

/// Severity assigned to an audit finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuditSeverity {
    /// Informational observation; no security violation is implied.
    Info,
    /// Potentially unsafe pattern requiring review.
    Low,
    /// Significant security or correctness concern.
    Medium,
    /// High-impact security concern.
    High,
    /// Critical issue that should block a production security gate.
    Critical,
}

impl AuditSeverity {
    /// Returns the stable machine-readable representation of the severity.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "INFO",
            Self::Low => "LOW",
            Self::Medium => "MEDIUM",
            Self::High => "HIGH",
            Self::Critical => "CRITICAL",
        }
    }
}

/// Category assigned to an audit finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditCategory {
    /// Unsafe memory or pointer operations.
    MemorySafety,
    /// Dynamic execution or code-generation risk.
    CodeGeneration,
    /// Filesystem or process isolation concern.
    Isolation,
    /// Cryptographic/security-sensitive implementation concern.
    Cryptography,
    /// FFI/native boundary concern.
    Ffi,
    /// Concurrency/atomicity concern.
    Concurrency,
    /// Explicit unsafe Rust usage.
    UnsafeCode,
    /// Audit infrastructure or scanner failure.
    AuditInfrastructure,
}

impl AuditCategory {
    /// Returns the stable machine-readable representation of the category.
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditFinding {
    /// Stable category for programmatic consumers.
    pub category: AuditCategory,
    /// Severity of the finding.
    pub severity: AuditSeverity,
    /// Human-readable description.
    pub description: String,
    /// Repository-relative path where the evidence was found.
    pub target_module: String,
    /// One-based source line containing the evidence, when available.
    pub line: Option<usize>,
    /// The source pattern that caused the finding.
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
    /// Maximum size of an individual source file that will be scanned.
    ///
    /// Larger files are skipped rather than causing the audit to consume
    /// unbounded memory.
    pub max_file_size_bytes: u64,

    /// Whether hidden directories such as `.git` are ignored.
    pub ignore_hidden_directories: bool,

    /// Whether generated build directories are ignored.
    pub ignore_build_directories: bool,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            max_file_size_bytes: 4 * 1024 * 1024,
            ignore_hidden_directories: true,
            ignore_build_directories: true,
        }
    }
}

/// Summary returned by an audit.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AuditSummary {
    pub files_scanned: usize,
    pub files_skipped: usize,
    pub findings: usize,
    pub info: usize,
    pub low: usize,
    pub medium: usize,
    pub high: usize,
    pub critical: usize,
}

/// Complete result of an audit.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AuditReport {
    pub findings: Vec<AuditFinding>,
    pub summary: AuditSummary,
    pub scan_errors: Vec<String>,
}

impl AuditReport {
    /// Returns true when the audit encountered a critical or high-severity
    /// finding.
    pub fn has_blocking_findings(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.severity >= AuditSeverity::High)
    }

    /// Returns true when the scanner itself encountered an error.
    pub fn has_scan_errors(&self) -> bool {
        !self.scan_errors.is_empty()
    }

    /// Returns true only when the audit completed without scanner errors and
    /// found no high/critical findings.
    pub fn is_clean(&self) -> bool {
        !self.has_scan_errors() && !self.has_blocking_findings()
    }
}

/// Static audit engine for a Zamani source tree.
#[derive(Debug, Clone)]
pub struct ZamaniAuditEngine {
    pub codebase_root: PathBuf,
    pub config: AuditConfig,
}

impl ZamaniAuditEngine {
    /// Creates an audit engine using the default production configuration.
    pub fn new(root: &str) -> Self {
        Self {
            codebase_root: PathBuf::from(root),
            config: AuditConfig::default(),
        }
    }

    /// Creates an audit engine with explicit configuration.
    pub fn with_config(root: impl Into<PathBuf>, config: AuditConfig) -> Self {
        Self {
            codebase_root: root.into(),
            config,
        }
    }

    /// Runs the complete static audit.
    ///
    /// No findings are fabricated. Every finding originates from source
    /// evidence encountered during the scan.
    pub fn audit(&self) -> AuditReport {
        let mut report = AuditReport::default();

        if !self.codebase_root.exists() {
            report.scan_errors.push(format!(
                "audit root does not exist: {}",
                self.codebase_root.display()
            ));
            return report;
        }

        if !self.codebase_root.is_dir() {
            report.scan_errors.push(format!(
                "audit root is not a directory: {}",
                self.codebase_root.display()
            ));
            return report;
        }

        self.scan_directory(&self.codebase_root, &mut report);

        report
            .findings
            .sort_by(|left, right| match right.severity.cmp(&left.severity) {
                Ordering::Equal => match left.target_module.cmp(&right.target_module) {
                    Ordering::Equal => left
                        .line
                        .unwrap_or(0)
                        .cmp(&right.line.unwrap_or(0)),
                    ordering => ordering,
                },
                ordering => ordering,
            });

        report.summary.findings = report.findings.len();

        for finding in &report.findings {
            match finding.severity {
                AuditSeverity::Info => report.summary.info += 1,
                AuditSeverity::Low => report.summary.low += 1,
                AuditSeverity::Medium => report.summary.medium += 1,
                AuditSeverity::High => report.summary.high += 1,
                AuditSeverity::Critical => report.summary.critical += 1,
            }
        }

        report
    }

    /// Backwards-compatible entry point used by existing callers.
    ///
    /// Unlike the old implementation, this method performs a real static
    /// source scan and returns only evidence-backed findings.
    pub fn run_audit(&self) -> Vec<AuditFinding> {
        self.audit().findings
    }

    fn scan_directory(&self, directory: &Path, report: &mut AuditReport) {
        let entries = match fs::read_dir(directory) {
            Ok(entries) => entries,
            Err(error) => {
                report.scan_errors.push(format!(
                    "failed to read '{}': {}",
                    directory.display(),
                    error
                ));
                return;
            }
        };

        let mut paths = entries
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .collect::<Vec<_>>();

        paths.sort();

        for path in paths {
            if path.is_dir() {
                if self.should_skip_directory(&path) {
                    report.summary.files_skipped += 1;
                    continue;
                }

                self.scan_directory(&path, report);
                continue;
            }

            if !path.is_file() || !self.is_source_file(&path) {
                continue;
            }

            self.scan_file(&path, report);
        }
    }

    fn should_skip_directory(&self, path: &Path) -> bool {
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            return false;
        };

        if self.config.ignore_build_directories
            && matches!(name, "target" | "node_modules" | ".cache" | "dist")
        {
            return true;
        }

        self.config.ignore_hidden_directories && name.starts_with('.')
    }

    fn is_source_file(&self, path: &Path) -> bool {
        matches!(
            path.extension().and_then(|extension| extension.to_str()),
            Some("rs") | Some("snk") | Some("toml") | Some("g4")
        )
    }

    fn scan_file(&self, path: &Path, report: &mut AuditReport) {
        let metadata = match fs::metadata(path) {
            Ok(metadata) => metadata,
            Err(error) => {
                report.scan_errors.push(format!(
                    "failed to stat '{}': {}",
                    path.display(),
                    error
                ));
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
            .strip_prefix(&self.codebase_root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");

        for (line_number, line) in source.lines().enumerate() {
            let line_number = line_number + 1;

            self.check_unsafe_rust(
                &relative_path,
                line_number,
                line,
                &mut report.findings,
            );

            self.check_dynamic_execution(
                &relative_path,
                line_number,
                line,
                &mut report.findings,
            );

            self.check_raw_ffi(
                &relative_path,
                line_number,
                line,
                &mut report.findings,
            );

            self.check_shell_execution(
                &relative_path,
                line_number,
                line,
                &mut report.findings,
            );

            self.check_unencrypted_weak_crypto(
                &relative_path,
                line_number,
                line,
                &mut report.findings,
            );
        }
    }

    fn check_unsafe_rust(
        &self,
        path: &str,
        line: usize,
        source_line: &str,
        findings: &mut Vec<AuditFinding>,
    ) {
        if source_line.contains("unsafe {") || source_line.trim_start().starts_with("unsafe fn") {
            findings.push(AuditFinding::new(
                AuditCategory::UnsafeCode,
                AuditSeverity::Medium,
                "Unsafe Rust code requires explicit security review and must remain isolated to the smallest possible scope.",
                path.to_string(),
                Some(line),
                truncate_evidence(source_line),
                "ZAAE-RUST-UNSAFE-001",
            ));
        }
    }

    fn check_dynamic_execution(
        &self,
        path: &str,
        line: usize,
        source_line: &str,
        findings: &mut Vec<AuditFinding>,
    ) {
        let patterns = [
            ("std::process::Command::new", "external process execution"),
            ("Command::new", "external process execution"),
            ("std::fs::write", "runtime filesystem code generation"),
            ("fs::write", "runtime filesystem code generation"),
        ];

        for (pattern, description) in patterns {
            if source_line.contains(pattern) {
                findings.push(AuditFinding::new(
                    AuditCategory::CodeGeneration,
                    AuditSeverity::Medium,
                    format!(
                        "Detected {}. Production compiler paths must validate inputs, constrain destinations, and preserve isolation.",
                        description
                    ),
                    path.to_string(),
                    Some(line),
                    truncate_evidence(source_line),
                    "ZAAE-CODEGEN-001",
                ));
                break;
            }
        }
    }

    fn check_raw_ffi(
        &self,
        path: &str,
        line: usize,
        source_line: &str,
        findings: &mut Vec<AuditFinding>,
    ) {
        if source_line.contains("extern \"C\"") || source_line.contains("extern \"system\"") {
            findings.push(AuditFinding::new(
                AuditCategory::Ffi,
                AuditSeverity::High,
                "Detected a native FFI boundary. Production builds require explicit ABI, ownership, lifetime, and error-boundary validation.",
                path.to_string(),
                Some(line),
                truncate_evidence(source_line),
                "ZAAE-FFI-001",
            ));
        }
    }

    fn check_shell_execution(
        &self,
        path: &str,
        line: usize,
        source_line: &str,
        findings: &mut Vec<AuditFinding>,
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
            findings.push(AuditFinding::new(
                AuditCategory::Isolation,
                AuditSeverity::High,
                "Detected shell invocation. Production compiler code must avoid shell interpretation where possible and must never concatenate untrusted input into shell commands.",
                path.to_string(),
                Some(line),
                truncate_evidence(source_line),
                "ZAAE-ISOLATION-001",
            ));
        }
    }

    fn check_unencrypted_weak_crypto(
        &self,
        path: &str,
        line: usize,
        source_line: &str,
        findings: &mut Vec<AuditFinding>,
    ) {
        let weak_patterns = [
            "md5::",
            "sha1::",
            "DefaultHasher",
            "thread_rng()",
        ];

        if weak_patterns
            .iter()
            .any(|pattern| source_line.contains(pattern))
        {
            findings.push(AuditFinding::new(
                AuditCategory::Cryptography,
                AuditSeverity::Low,
                "Detected a potentially inappropriate primitive in security-sensitive compiler code. Review its purpose and threat model before production use.",
                path.to_string(),
                Some(line),
                truncate_evidence(source_line),
                "ZAAE-CRYPTO-001",
            ));
        }
    }
}

fn truncate_evidence(line: &str) -> String {
    const MAX_EVIDENCE_LENGTH: usize = 240;

    let trimmed = line.trim();

    if trimmed.chars().count() <= MAX_EVIDENCE_LENGTH {
        return trimmed.to_string();
    }

    trimmed.chars().take(MAX_EVIDENCE_LENGTH).collect::<String>() + "..."
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_directory() -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must be after UNIX epoch")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "zamani_audit_engine_{}_{}",
            std::process::id(),
            timestamp
        ))
    }

    #[test]
    fn missing_root_is_reported_without_panicking() {
        let root = temporary_directory();
        let engine = ZamaniAuditEngine::new(root.to_string_lossy().as_ref());

        let report = engine.audit();

        assert!(report.has_scan_errors());
        assert!(!report.is_clean());
    }

    #[test]
    fn clean_source_produces_no_findings() {
        let root = temporary_directory();
        fs::create_dir_all(&root).expect("temporary directory should be created");

        let source_path = root.join("clean.rs");
        fs::write(
            &source_path,
            "fn main() {\n    let value = 42;\n    println!(\"{}\", value);\n}\n",
        )
        .expect("source file should be written");

        let engine = ZamaniAuditEngine::new(root.to_string_lossy().as_ref());
        let report = engine.audit();

        assert_eq!(report.summary.files_scanned, 1);
        assert_eq!(report.summary.findings, 0);
        assert!(report.is_clean());

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn unsafe_code_is_reported_with_source_location() {
        let root = temporary_directory();
        fs::create_dir_all(&root).expect("temporary directory should be created");

        let source_path = root.join("unsafe.rs");
        fs::write(
            &source_path,
            "fn main() {\n    unsafe { dangerous(); }\n}\n",
        )
        .expect("source file should be written");

        let engine = ZamaniAuditEngine::new(root.to_string_lossy().as_ref());
        let report = engine.audit();

        assert_eq!(report.summary.files_scanned, 1);
        assert_eq!(report.summary.findings, 1);
        assert_eq!(report.findings[0].category, AuditCategory::UnsafeCode);
        assert_eq!(report.findings[0].severity, AuditSeverity::Medium);
        assert_eq!(report.findings[0].line, Some(2));
        assert_eq!(report.findings[0].rule_id, "ZAAE-RUST-UNSAFE-001");

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn shell_execution_is_blocking() {
        let root = temporary_directory();
        fs::create_dir_all(&root).expect("temporary directory should be created");

        let source_path = root.join("shell.rs");
        fs::write(
            &source_path,
            "use std::process::Command;\nfn run() { Command::new(\"sh\"); }\n",
        )
        .expect("source file should be written");

        let engine = ZamaniAuditEngine::new(root.to_string_lossy().as_ref());
        let report = engine.audit();

        assert!(report.has_blocking_findings());
        assert_eq!(report.summary.high, 1);

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn findings_are_sorted_by_severity_then_location() {
        let root = temporary_directory();
        fs::create_dir_all(&root).expect("temporary directory should be created");

        let source_path = root.join("mixed.rs");
        fs::write(
            &source_path,
            concat!(
                "unsafe fn native() {}\n",
                "extern \"C\" { fn call(); }\n",
                "unsafe { native(); }\n",
            ),
        )
        .expect("source file should be written");

        let engine = ZamaniAuditEngine::new(root.to_string_lossy().as_ref());
        let report = engine.audit();

        assert_eq!(report.findings.len(), 3);
        assert!(report.findings[0].severity >= report.findings[1].severity);
        assert!(report.findings[1].severity >= report.findings[2].severity);

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn oversized_files_are_skipped() {
        let root = temporary_directory();
        fs::create_dir_all(&root).expect("temporary directory should be created");

        let source_path = root.join("large.rs");
        fs::write(&source_path, vec![b'x'; 128]).expect("source file should be written");

        let config = AuditConfig {
            max_file_size_bytes: 64,
            ..AuditConfig::default()
        };

        let engine = ZamaniAuditEngine::with_config(&root, config);
        let report = engine.audit();

        assert_eq!(report.summary.files_scanned, 0);
        assert_eq!(report.summary.files_skipped, 1);
        assert!(report.is_clean());

        let _ = fs::remove_dir_all(&root);
    }
}