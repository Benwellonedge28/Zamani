
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Portable Paths
//!
//! Path: `src/stdlib/io/paths.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides portable logical path representation and manipulation.
//!
//! A `PortablePath` is independent of the host operating system's
//! native path representation. It can represent relative paths,
//! rooted paths, and provider-defined namespaces.
//!
//! Native operating-system paths are represented separately by
//! `HostPath`. This distinction is important because native paths
//! may contain non-UTF-8 data and platform-specific prefixes.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Portable logical path representation.
//! - Portable path parsing and formatting.
//! - Component-aware path manipulation.
//! - Lexical normalization.
//! - Relative path validation.
//! - Native path wrapper and explicit conversion.
//!
//! This module does not own:
//!
//! - Filesystem operations.
//! - Filesystem authorization.
//! - Symlink resolution.
//! - Physical storage.
//! - Mount tables.
//! - Provider discovery.
//! - Package resolution.
//! - Runtime resource allocation.
//! - Source-language syntax or semantic analysis.
//!
//! ## Portability
//!
//! Portable paths use `/` as their separator.
//!
//! A provider namespace has the form:
//!
//! `scheme://authority/path/components`
//!
//! Namespace schemes are extensible. This module does not enumerate
//! filesystem vendors, storage technologies, or hardware.
//!
//! A portable path is not automatically a valid path on every host.
//! Conversion to a native path is explicit and may fail.
//!
//! ## Security
//!
//! Lexical normalization is not filesystem canonicalization.
//!
//! It does not resolve symbolic links, mount points, aliases,
//! case folding, filesystem permissions, or race conditions.
//!
//! `join_relative` rejects rooted paths and parent-directory
//! components. This prevents lexical traversal through that API,
//! but does not replace provider-side authorization or protection
//! against symlink traversal.
//!
//! ## Scalability
//!
//! No arbitrary path-length, component-count, nesting-depth,
//! namespace-count, or provider-count ceiling is imposed.
//!
//! Memory use grows according to the supplied path and available
//! resources. Rust and the host platform still have representational
//! and allocation limits.
//!
//! ## Integration
//!
//! Shared errors:
//! `crate::stdlib::error`
//!
//! Filesystem compatibility:
//! `crate::stdlib::fs`
//!
//! I/O module:
//! `crate::stdlib::io`
//!
//! Storage providers:
//! `crate::stdlib::data::storage`
//!
//! Runtime and platform path interpretation remain outside this module.

use core::fmt;
use core::str::FromStr;

use std::path::{Path as StdPath, PathBuf};

use crate::stdlib::error::{Error, ErrorKind, Result};

/// Stable standard-library component identifier.
pub const COMPONENT: &str = "stdlib.io.paths";

/// Stable path parsing operation identifier.
pub const PARSE_OPERATION: &str = "io.path.parse";

/// Stable path conversion operation identifier.
pub const CONVERT_OPERATION: &str = "io.path.convert";

/// Stable path normalization operation identifier.
pub const NORMALIZE_OPERATION: &str = "io.path.normalize";

/// Stable path joining operation identifier.
pub const JOIN_OPERATION: &str = "io.path.join";

/// Root identity of a portable path.
///
/// Roots are logical identities, not physical devices or mount points.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathRoot {
    /// A path relative to an externally established base.
    Relative,

    /// A rooted path without a provider-specific namespace.
    Rooted,

    /// A provider-defined namespace.
    ///
    /// `scheme` identifies the namespace type and `authority`
    /// identifies the provider-defined namespace authority.
    ///
    /// Neither value establishes trust, authorization, availability,
    /// or physical location.
    Namespace {
        /// Namespace scheme.
        scheme: String,

        /// Namespace authority.
        authority: String,
    },
}

impl PathRoot {
    /// Returns whether this root is relative.
    #[must_use]
    pub const fn is_relative(&self) -> bool {
        matches!(self, Self::Relative)
    }

    /// Returns whether this root is rooted or namespaced.
    #[must_use]
    pub const fn is_absolute(&self) -> bool {
        !self.is_relative()
    }
}

/// Portable, provider-independent logical path.
///
/// The canonical textual representation uses `/` separators.
///
/// Path components are preserved during parsing. In particular,
/// `.` and `..` are not silently removed. Call [`PortablePath::normalize`]
/// when lexical normalization is explicitly desired.
///
/// The representation does not perform filesystem access.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PortablePath {
    root: PathRoot,
    components: Vec<String>,
    trailing_separator: bool,
    representation: String,
}

/// Compatibility-friendly name for the portable path type.
///
/// This alias is intentionally separate from `std::path::Path`.
pub type Path = PortablePath;

impl PortablePath {
    /// Parses a portable path.
    ///
    /// Portable paths use `/` as the only separator.
    ///
    /// Backslash is not treated as a separator because it is a valid
    /// filename character on some platforms.
    ///
    /// Empty components caused by repeated separators are ignored.
    ///
    /// The parser preserves `.` and `..` components.
    ///
    /// Namespace syntax is:
    ///
    /// `scheme://authority/path`
    ///
    /// # Errors
    ///
    /// Returns an error for:
    ///
    /// - embedded NUL characters;
    /// - invalid namespace schemes;
    /// - empty namespace authorities.
    pub fn parse(value: impl AsRef<str>) -> Result<Self> {
        let value = value.as_ref();

        if value.contains('\0') {
            return Err(path_error(
                ErrorKind::InvalidInput,
                "path contains a NUL character",
                PARSE_OPERATION,
            ));
        }

        let trailing_separator =
            !value.is_empty() && value.ends_with('/');

        let (root, remainder) = parse_root(value)?;

        let components = remainder
            .split('/')
            .filter(|component| !component.is_empty())
            .map(str::to_owned)
            .collect();

        Ok(Self::from_parts(
            root,
            components,
            trailing_separator,
        ))
    }

    /// Creates an empty relative path.
    #[must_use]
    pub fn empty() -> Self {
        Self::from_parts(
            PathRoot::Relative,
            Vec::new(),
            false,
        )
    }

    /// Creates a relative path from individual components.
    ///
    /// Components must not contain `/` or NUL.
    ///
    /// Components equal to `.` or `..` are permitted and preserved.
    ///
    /// # Errors
    ///
    /// Returns an error if a component contains an invalid separator
    /// or a NUL character.
    pub fn from_components<I, S>(components: I) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut values = Vec::new();

        for component in components {
            let component = component.into();

            if component.is_empty()
                || component.contains('/')
                || component.contains('\0')
            {
                return Err(path_error(
                    ErrorKind::InvalidInput,
                    "path component is empty or contains an invalid character",
                    PARSE_OPERATION,
                ));
            }

            values.push(component);
        }

        Ok(Self::from_parts(
            PathRoot::Relative,
            values,
            false,
        ))
    }

    /// Creates a provider-namespaced path.
    ///
    /// The namespace is logical and provider-defined.
    ///
    /// For example:
    ///
    /// `object://tenant-a/datasets/experiment`
    ///
    /// This constructor does not connect to or discover a provider.
    ///
    /// # Errors
    ///
    /// Returns an error when the scheme or authority is invalid.
    pub fn namespaced(
        scheme: impl Into<String>,
        authority: impl Into<String>,
        components: impl IntoIterator<Item = String>,
    ) -> Result<Self> {
        let scheme = scheme.into();
        let authority = authority.into();

        if !valid_scheme(&scheme) {
            return Err(path_error(
                ErrorKind::InvalidInput,
                "namespace scheme is invalid",
                PARSE_OPERATION,
            ));
        }

        if authority.is_empty()
            || authority.contains('/')
            || authority.contains('\0')
        {
            return Err(path_error(
                ErrorKind::InvalidInput,
                "namespace authority is invalid",
                PARSE_OPERATION,
            ));
        }

        let mut values = Vec::new();

        for component in components {
            if component.is_empty()
                || component.contains('/')
                || component.contains('\0')
            {
                return Err(path_error(
                    ErrorKind::InvalidInput,
                    "namespace path contains an invalid component",
                    PARSE_OPERATION,
                ));
            }

            values.push(component);
        }

        Ok(Self::from_parts(
            PathRoot::Namespace {
                scheme,
                authority,
            },
            values,
            false,
        ))
    }

    /// Returns the canonical portable representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.representation
    }

    /// Returns the path root.
    #[must_use]
    pub const fn root(&self) -> &PathRoot {
        &self.root
    }

    /// Returns the path components.
    ///
    /// Components are returned in their stored order.
    /// Dot components are not removed.
    #[must_use]
    pub fn components(&self) -> &[String] {
        &self.components
    }

    /// Returns whether the path is relative.
    #[must_use]
    pub const fn is_relative(&self) -> bool {
        self.root.is_relative()
    }

    /// Returns whether the path is rooted or namespaced.
    #[must_use]
    pub const fn is_absolute(&self) -> bool {
        self.root.is_absolute()
    }

    /// Returns whether the representation ends with `/`.
    #[must_use]
    pub const fn has_trailing_separator(&self) -> bool {
        self.trailing_separator
    }

    /// Returns whether the path has no components.
    ///
    /// A rooted path or namespace root can have no components
    /// while still having a meaningful root.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.root.is_relative() && self.components.is_empty()
    }

    /// Returns the number of components.
    #[must_use]
    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    /// Returns the final component, if present.
    ///
    /// A trailing separator does not remove the final component.
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.components.last().map(String::as_str)
    }

    /// Returns the extension of the final component, if present.
    ///
    /// A leading dot alone is not treated as an extension separator.
    #[must_use]
    pub fn extension(&self) -> Option<&str> {
        let name = self.file_name()?;
        let index = name.rfind('.')?;

        if index == 0 || index + 1 == name.len() {
            return None;
        }

        Some(&name[index + 1..])
    }

    /// Returns the final component without its extension.
    ///
    /// A leading dot alone is not treated as an extension separator.
    #[must_use]
    pub fn file_stem(&self) -> Option<&str> {
        let name = self.file_name()?;

        match name.rfind('.') {
            Some(index) if index > 0 => Some(&name[..index]),
            _ => Some(name),
        }
    }

    /// Returns the lexical parent path.
    ///
    /// Root paths have no parent.
    #[must_use]
    pub fn parent(&self) -> Option<Self> {
        if self.components.is_empty() {
            return None;
        }

        let mut components = self.components.clone();
        components.pop();

        Some(Self::from_parts(
            self.root.clone(),
            components,
            false,
        ))
    }

    /// Joins another path using conventional logical path semantics.
    ///
    /// If `other` is rooted or namespaced, it replaces the current path.
    ///
    /// Otherwise, its components are appended.
    ///
    /// This method does not reject `..`. Use [`Self::join_relative`]
    /// when untrusted relative input must not escape lexically.
    #[must_use]
    pub fn join(&self, other: &Self) -> Self {
        if other.is_absolute() {
            return other.clone();
        }

        let mut components = self.components.clone();
        components.extend(other.components.iter().cloned());

        Self::from_parts(
            self.root.clone(),
            components,
            other.trailing_separator,
        )
    }

    /// Joins a relative path while rejecting lexical traversal.
    ///
    /// The supplied path must:
    ///
    /// - be relative;
    /// - contain no `..` components.
    ///
    /// This is useful when constructing paths beneath a logical base.
    ///
    /// It is not a replacement for filesystem authorization or
    /// symlink-safe provider operations.
    ///
    /// # Errors
    ///
    /// Returns an error if the supplied path is rooted or contains
    /// a parent-directory component.
    pub fn join_relative(&self, other: &Self) -> Result<Self> {
        if !other.is_relative() {
            return Err(path_error(
                ErrorKind::InvalidInput,
                "join_relative requires a relative path",
                JOIN_OPERATION,
            ));
        }

        if other.components.iter().any(|part| part == "..") {
            return Err(path_error(
                ErrorKind::PermissionDenied,
                "relative path contains a parent-directory component",
                JOIN_OPERATION,
            ));
        }

        Ok(self.join(other))
    }

    /// Performs lexical normalization.
    ///
    /// The operation:
    ///
    /// - removes `.` components;
    /// - resolves a normal component followed by `..`;
    /// - preserves leading `..` in relative paths;
    /// - rejects traversal above rooted or namespaced roots.
    ///
    /// It does not access the filesystem or resolve symbolic links.
    ///
    /// # Errors
    ///
    /// Returns an error if a rooted or namespaced path attempts
    /// to traverse above its root.
    pub fn normalize(&self) -> Result<Self> {
        let mut normalized: Vec<String> = Vec::new();

        for component in &self.components {
            match component.as_str() {
                "" | "." => {}

                ".." => {
                    if normalized
                        .last()
                        .is_some_and(|last| last != "..")
                    {
                        normalized.pop();
                    } else if self.is_relative() {
                        normalized.push("..".to_owned());
                    } else {
                        return Err(path_error(
                            ErrorKind::InvalidInput,
                            "path attempts to traverse above its root",
                            NORMALIZE_OPERATION,
                        ));
                    }
                }

                _ => normalized.push(component.clone()),
            }
        }

        Ok(Self::from_parts(
            self.root.clone(),
            normalized,
            self.trailing_separator,
        ))
    }

    /// Returns whether this path begins with the supplied path.
    ///
    /// Comparison is component-aware and root-aware.
    ///
    /// It does not perform filesystem case folding or symlink resolution.
    #[must_use]
    pub fn starts_with(&self, base: &Self) -> bool {
        self.root == base.root
            && self.components.len() >= base.components.len()
            && self.components[..base.components.len()]
                == base.components[..]
    }

    /// Removes a component-aware prefix.
    ///
    /// The returned path is relative.
    ///
    /// # Errors
    ///
    /// Returns an error if `base` is not a prefix of this path.
    pub fn strip_prefix(&self, base: &Self) -> Result<Self> {
        if !self.starts_with(base) {
            return Err(path_error(
                ErrorKind::InvalidInput,
                "path does not start with the supplied prefix",
                JOIN_OPERATION,
            ));
        }

        let components = self.components[base.components.len()..]
            .to_vec();

        Ok(Self::from_parts(
            PathRoot::Relative,
            components,
            self.trailing_separator,
        ))
    }

    /// Computes a lexical relative path from `base` to this path.
    ///
    /// Both paths must have the same root identity.
    ///
    /// This operation does not inspect filesystem state.
    ///
    /// # Errors
    ///
    /// Returns an error if the roots differ.
    pub fn relative_to(&self, base: &Self) -> Result<Self> {
        if self.root != base.root {
            return Err(path_error(
                ErrorKind::InvalidInput,
                "cannot compute a relative path between different roots",
                JOIN_OPERATION,
            ));
        }

        let common = self
            .components
            .iter()
            .zip(base.components.iter())
            .take_while(|(left, right)| left == right)
            .count();

        let mut components = Vec::new();

        for _ in common..base.components.len() {
            components.push("..".to_owned());
        }

        components.extend(
            self.components[common..].iter().cloned(),
        );

        Ok(Self::from_parts(
            PathRoot::Relative,
            components,
            self.trailing_separator,
        ))
    }

    /// Converts this path into a native host path.
    ///
    /// Provider namespaces cannot be converted automatically because
    /// their meaning is provider-defined.
    ///
    /// A rooted portable path on Windows is rejected because `/name`
    /// does not identify a drive or volume unambiguously.
    ///
    /// Conversion does not canonicalize or access the filesystem.
    ///
    /// # Errors
    ///
    /// Returns an error if the path cannot be represented unambiguously
    /// as a native host path.
    pub fn to_host_path(&self) -> Result<HostPath> {
        if matches!(self.root, PathRoot::Namespace { .. }) {
            return Err(path_error(
                ErrorKind::Unsupported,
                "provider-namespaced paths require a provider-specific resolver",
                CONVERT_OPERATION,
            ));
        }

        #[cfg(windows)]
        if matches!(self.root, PathRoot::Rooted) {
            return Err(path_error(
                ErrorKind::Unsupported,
                "a rooted portable path does not identify a Windows volume",
                CONVERT_OPERATION,
            ));
        }

        Ok(HostPath {
            inner: PathBuf::from(self.as_str()),
        })
    }

    /// Creates a portable path from a native host path.
    ///
    /// Native paths that are not valid UTF-8 cannot be represented
    /// as `PortablePath`. Such paths remain available through `HostPath`.
    ///
    /// Windows native separators are converted to `/`.
    ///
    /// Platform-specific prefixes are not silently discarded.
    ///
    /// # Errors
    ///
    /// Returns an error for non-UTF-8 paths or native prefixes that
    /// cannot be represented by the portable path grammar.
    pub fn from_host_path(path: &StdPath) -> Result<Self> {
        let value = path.to_str().ok_or_else(|| {
            path_error(
                ErrorKind::InvalidData,
                "native path is not valid UTF-8",
                CONVERT_OPERATION,
            )
        })?;

        #[cfg(windows)]
        let portable = value.replace('\\', "/");

        #[cfg(not(windows))]
        let portable = value.to_owned();

        #[cfg(windows)]
        if portable.as_bytes().get(1) == Some(&b':') {
            return Err(path_error(
                ErrorKind::Unsupported,
                "drive-qualified native paths require a platform-specific resolver",
                CONVERT_OPERATION,
            ));
        }

        Self::parse(portable)
    }

    /// Constructs a path from its internal parts and canonicalizes
    /// only its textual separator representation.
    fn from_parts(
        root: PathRoot,
        components: Vec<String>,
        trailing_separator: bool,
    ) -> Self {
        let mut representation = match &root {
            PathRoot::Relative => String::new(),

            PathRoot::Rooted => "/".to_owned(),

            PathRoot::Namespace {
                scheme,
                authority,
            } => format!("{scheme}://{authority}"),
        };

        let has_root_prefix = !root.is_relative();

        if !components.is_empty() {
            if has_root_prefix
                && !representation.ends_with('/')
            {
                representation.push('/');
            }

            representation.push_str(&components.join("/"));
        }

        if trailing_separator
            && !representation.is_empty()
            && !representation.ends_with('/')
        {
            representation.push('/');
        }

        Self {
            root,
            components,
            trailing_separator,
            representation,
        }
    }
}

impl FromStr for PortablePath {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        Self::parse(value)
    }
}

impl fmt::Display for PortablePath {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.representation)
    }
}

/// Native operating-system path.
///
/// Unlike `PortablePath`, this type preserves the host's native
/// path representation, including non-UTF-8 data where supported.
///
/// It does not perform filesystem operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HostPath {
    inner: PathBuf,
}

impl HostPath {
    /// Creates a native path from a `PathBuf`.
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        Self { inner: path }
    }

    /// Creates a native path from a borrowed native path.
    #[must_use]
    pub fn from_path(path: &StdPath) -> Self {
        Self {
            inner: path.to_path_buf(),
        }
    }

    /// Returns the native path by reference.
    #[must_use]
    pub fn as_path(&self) -> &StdPath {
        &self.inner
    }

    /// Returns the native path as an owned `PathBuf`.
    #[must_use]
    pub fn into_path_buf(self) -> PathBuf {
        self.inner
    }

    /// Attempts to convert the native path to a portable UTF-8 path.
    ///
    /// This may fail when the native path contains non-UTF-8 data
    /// or platform-specific syntax not represented by `PortablePath`.
    pub fn to_portable(&self) -> Result<PortablePath> {
        PortablePath::from_host_path(&self.inner)
    }
}

impl From<PathBuf> for HostPath {
    fn from(path: PathBuf) -> Self {
        Self::new(path)
    }
}

impl AsRef<StdPath> for HostPath {
    fn as_ref(&self) -> &StdPath {
        self.as_path()
    }
}

impl fmt::Display for HostPath {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.inner.display().fmt(formatter)
    }
}

/// Parses the root portion of a portable path.
fn parse_root(value: &str) -> Result<(PathRoot, &str)> {
    if let Some(separator) = value.find("://") {
        let scheme = &value[..separator];

        if !valid_scheme(scheme) {
            return Err(path_error(
                ErrorKind::InvalidInput,
                "path namespace scheme is invalid",
                PARSE_OPERATION,
            ));
        }

        let remainder = &value[separator + 3..];

        let authority_end = remainder
            .find('/')
            .unwrap_or(remainder.len());

        let authority = &remainder[..authority_end];

        if authority.is_empty() {
            return Err(path_error(
                ErrorKind::InvalidInput,
                "path namespace authority must not be empty",
                PARSE_OPERATION,
            ));
        }

        let path_remainder = if authority_end == remainder.len() {
            ""
        } else {
            &remainder[authority_end + 1..]
        };

        return Ok((
            PathRoot::Namespace {
                scheme: scheme.to_owned(),
                authority: authority.to_owned(),
            },
            path_remainder,
        ));
    }

    if let Some(remainder) = value.strip_prefix('/') {
        return Ok((PathRoot::Rooted, remainder));
    }

    Ok((PathRoot::Relative, value))
}

/// Validates a portable namespace scheme.
fn valid_scheme(value: &str) -> bool {
    let mut characters = value.chars();

    let Some(first) = characters.next() else {
        return false;
    };

    if !first.is_ascii_alphabetic() {
        return false;
    }

    characters.all(|character| {
        character.is_ascii_alphanumeric()
            || matches!(character, '+' | '-' | '.')
    })
}

/// Constructs a structured path error.
fn path_error(
    kind: ErrorKind,
    message: impl Into<String>,
    operation: &'static str,
) -> Error {
    Error::new(kind, message)
        .in_component(COMPONENT)
        .during_operation(operation)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_relative_path() {
        let path = PortablePath::parse("src/stdlib/io")
            .expect("valid path");

        assert!(path.is_relative());
        assert_eq!(path.as_str(), "src/stdlib/io");
        assert_eq!(
            path.components(),
            &[
                "src".to_owned(),
                "stdlib".to_owned(),
                "io".to_owned()
            ]
        );
    }

    #[test]
    fn parses_rooted_path() {
        let path = PortablePath::parse("/var/data")
            .expect("valid path");

        assert!(path.is_absolute());
        assert_eq!(path.as_str(), "/var/data");
    }

    #[test]
    fn parses_provider_namespace() {
        let path = PortablePath::parse(
            "object://tenant-a/datasets/experiment",
        )
        .expect("valid namespace");

        assert_eq!(
            path.root(),
            &PathRoot::Namespace {
                scheme: "object".to_owned(),
                authority: "tenant-a".to_owned(),
            }
        );

        assert_eq!(
            path.as_str(),
            "object://tenant-a/datasets/experiment"
        );
    }

    #[test]
    fn preserves_dot_components_until_normalization() {
        let path = PortablePath::parse("a/./b/../c")
            .expect("valid path");

        assert_eq!(path.as_str(), "a/./b/../c");

        let normalized = path.normalize()
            .expect("normalization succeeds");

        assert_eq!(normalized.as_str(), "a/c");
    }

    #[test]
    fn preserves_leading_parent_components_in_relative_paths() {
        let path = PortablePath::parse("../../a")
            .expect("valid path");

        let normalized = path.normalize()
            .expect("relative normalization succeeds");

        assert_eq!(normalized.as_str(), "../../a");
    }

    #[test]
    fn rejects_traversal_above_root() {
        let path = PortablePath::parse("/../../a")
            .expect("parsing preserves components");

        assert!(path.normalize().is_err());
    }

    #[test]
    fn joins_relative_paths() {
        let base = PortablePath::parse("workspace/project")
            .expect("valid base");

        let child = PortablePath::parse("src/main.rs")
            .expect("valid child");

        assert_eq!(
            base.join(&child).as_str(),
            "workspace/project/src/main.rs"
        );
    }

    #[test]
    fn rooted_join_replaces_base() {
        let base = PortablePath::parse("workspace/project")
            .expect("valid base");

        let rooted = PortablePath::parse("/var/data")
            .expect("valid rooted path");

        assert_eq!(
            base.join(&rooted).as_str(),
            "/var/data"
        );
    }

    #[test]
    fn safe_relative_join_rejects_parent_traversal() {
        let base = PortablePath::parse("workspace")
            .expect("valid base");

        let child = PortablePath::parse("../outside")
            .expect("valid relative path");

        assert!(base.join_relative(&child).is_err());
    }

    #[test]
    fn safe_relative_join_accepts_normal_child() {
        let base = PortablePath::parse("workspace")
            .expect("valid base");

        let child = PortablePath::parse("src/main.rs")
            .expect("valid child");

        assert_eq!(
            base.join_relative(&child)
                .expect("safe join")
                .as_str(),
            "workspace/src/main.rs"
        );
    }

    #[test]
    fn parent_and_filename_are_component_aware() {
        let path = PortablePath::parse("src/stdlib/io/paths.rs")
            .expect("valid path");

        assert_eq!(path.file_name(), Some("paths.rs"));
        assert_eq!(path.extension(), Some("rs"));
        assert_eq!(path.file_stem(), Some("paths"));

        assert_eq!(
            path.parent().expect("parent").as_str(),
            "src/stdlib/io"
        );
    }

    #[test]
    fn hidden_file_has_no_extension() {
        let path = PortablePath::parse(".config")
            .expect("valid path");

        assert_eq!(path.extension(), None);
        assert_eq!(path.file_stem(), Some(".config"));
    }

    #[test]
    fn prefix_comparison_uses_components() {
        let path = PortablePath::parse("workspace/project-old")
            .expect("valid path");

        let prefix = PortablePath::parse("workspace/project")
            .expect("valid prefix");

        assert!(!path.starts_with(&prefix));
    }

    #[test]
    fn strips_component_prefix() {
        let path = PortablePath::parse(
            "workspace/project/src/main.rs",
        )
        .expect("valid path");

        let prefix = PortablePath::parse(
            "workspace/project",
        )
        .expect("valid prefix");

        assert_eq!(
            path.strip_prefix(&prefix)
                .expect("prefix matches")
                .as_str(),
            "src/main.rs"
        );
    }

    #[test]
    fn computes_relative_path() {
        let base = PortablePath::parse("a/b/c")
            .expect("valid base");

        let target = PortablePath::parse("a/d/e")
            .expect("valid target");

        assert_eq!(
            target.relative_to(&base)
                .expect("same root")
                .as_str(),
            "../../d/e"
        );
    }

    #[test]
    fn rejects_different_roots_for_relative_path() {
        let base = PortablePath::parse("/a/b")
            .expect("valid base");

        let target = PortablePath::parse("a/b")
            .expect("valid target");

        assert!(target.relative_to(&base).is_err());
    }

    #[test]
    fn rejects_nul_character() {
        assert!(PortablePath::parse("a\0b").is_err());
    }

    #[test]
    fn repeated_separators_are_canonicalized() {
        let path = PortablePath::parse("a//b///c")
            .expect("valid path");

        assert_eq!(path.as_str(), "a/b/c");
    }

    #[test]
    fn preserves_trailing_separator() {
        let path = PortablePath::parse("a/b/")
            .expect("valid path");

        assert!(path.has_trailing_separator());
        assert_eq!(path.as_str(), "a/b/");
    }

    #[test]
    fn namespace_cannot_be_converted_without_provider_resolution() {
        let path = PortablePath::parse("object://tenant/data")
            .expect("valid namespace");

        assert!(path.to_host_path().is_err());
    }

    #[test]
    fn host_path_round_trip_for_portable_relative_path() {
        let path = PortablePath::parse("workspace/src/main.rs")
            .expect("valid path");

        let host = path.to_host_path()
            .expect("relative host path");

        let restored = host.to_portable()
            .expect("portable host path");

        assert_eq!(restored.as_str(), path.as_str());
    }

    #[test]
    fn namespace_scheme_validation_is_extensible() {
        assert!(
            PortablePath::parse("future+storage://authority/object")
                .is_ok()
        );

        assert!(
            PortablePath::parse("1invalid://authority/object")
                .is_err()
        );
    }

    #[test]
    fn root_has_no_parent() {
        let root = PortablePath::parse("/")
            .expect("root");

        assert!(root.parent().is_none());
    }

    #[test]
    fn empty_relative_path_is_distinct_from_root() {
        let empty = PortablePath::empty();
        let root = PortablePath::parse("/")
            .expect("root");

        assert!(empty.is_empty());
        assert!(!root.is_empty());
        assert_ne!(empty, root);
    }
}
