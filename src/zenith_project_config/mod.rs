//! Zenith Universal Meta-Compiler (UMC) Project Configuration
//!
//! This module defines the data structures for parsing and representing
//! the `Zenith.toml` project manifest file. This manifest is central
//! to how `zenith-pkg` and the Zenith compiler understand and manage
//! projects, their dependencies, build configurations, and deployment strategies.

use std::collections::HashMap;

/// Represents the parsed content of a `Zenith.toml` file.
#[derive(Debug, Clone, PartialEq)]
pub struct ZenithToml {
    pub package: PackageSection,
    pub dependencies: HashMap<String, DependencyConfig>,
    // Extend with other sections as needed:
    // pub build: BuildSection,
    // pub features: FeatureSection,
    // pub deploy: DeploySection,
    // pub nimbus_os: NimbusOsSection,
    // pub sankofa_memory: SankofaMemorySection,
}

/// Represents the `[package]` section in `Zenith.toml`.
#[derive(Debug, Clone, PartialEq)]
pub struct PackageSection {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub edition: String,
    pub description: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license: Option<String>,
    pub package_type: String, // Renamed 'type' to 'package_type' to avoid Rust keyword collision
}

/// Represents a dependency entry in the `[dependencies]` section.
#[derive(Debug, Clone, PartialEq)]
pub struct DependencyConfig {
    pub version: String,
    pub path: Option<String>,          // For local path dependencies
    pub features: Option<Vec<String>>, // Optional features to enable for this dependency
}

impl ZenithToml {
    /// Parses a real `Zenith.toml` manifest string into a `ZenithToml` struct.
    ///
    /// This performs an actual TOML parse (via the `toml` crate) of the
    /// `[package]` and `[dependencies]` tables rather than returning a fixed
    /// example — any valid Zenith.toml manifest works, not just the sample
    /// shipped at the repo root.
    pub fn parse_from_str(content: &str) -> Result<Self, String> {
        let doc: toml::Value =
            toml::from_str(content).map_err(|e| format!("Zenith.toml parse error: {e}"))?;

        let pkg = doc
            .get("package")
            .ok_or_else(|| "Zenith.toml missing [package] table".to_string())?;

        let as_str = |v: &toml::Value| v.as_str().unwrap_or_default().to_string();
        let opt_str = |v: Option<&toml::Value>| v.and_then(|v| v.as_str()).map(|s| s.to_string());

        let package = PackageSection {
            name: pkg.get("name").map(as_str).unwrap_or_default(),
            version: pkg.get("version").map(as_str).unwrap_or_default(),
            authors: pkg
                .get("authors")
                .and_then(|a| a.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            edition: pkg.get("edition").map(as_str).unwrap_or_default(),
            description: pkg.get("description").map(as_str).unwrap_or_default(),
            homepage: opt_str(pkg.get("homepage")),
            repository: opt_str(pkg.get("repository")),
            license: opt_str(pkg.get("license")),
            package_type: pkg
                .get("type")
                .map(as_str)
                .unwrap_or_else(|| "application".to_string()),
        };

        let mut dependencies = HashMap::new();
        if let Some(deps_table) = doc.get("dependencies").and_then(|d| d.as_table()) {
            for (name, spec) in deps_table {
                let dep = if let Some(s) = spec.as_str() {
                    DependencyConfig {
                        version: s.to_string(),
                        path: None,
                        features: None,
                    }
                } else {
                    DependencyConfig {
                        version: spec.get("version").map(as_str).unwrap_or_default(),
                        path: opt_str(spec.get("path")),
                        features: spec.get("features").and_then(|f| f.as_array()).map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect()
                        }),
                    }
                };
                dependencies.insert(name.clone(), dep);
            }
        }

        Ok(ZenithToml {
            package,
            dependencies,
        })
    }
}
