
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani compiler macro-template engine.
//!
//! File: src/compiler/macro_engine.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Architectural position
//!
//! This module implements a deterministic, non-executing textual template
//! expansion facility. It is one compiler utility, not the complete Zamani
//! source-language macro implementation.
//!
//! The canonical source-language macro pipeline is:
//!
//! ```text
//! Zamani source
//!     -> canonical lexer
//!     -> canonical parser
//!     -> canonical frontend AST
//!     -> name/module resolution
//!     -> macro resolution
//!     -> token-tree / syntax-tree expansion
//!     -> hygiene and provenance
//!     -> semantic validation
//!     -> canonical semantic model
//!     -> canonical IR
//! ```
//!
//! This module does not own or redefine the lexer, parser, AST, token-tree,
//! source-map, semantic model, or IR.
//!
//! # Responsibilities
//!
//! - Register validated textual macro templates.
//! - Reject accidental duplicate registration.
//! - Support explicit replacement and removal.
//! - Validate positional argument counts.
//! - Compile templates into reusable substitution segments.
//! - Expand templates deterministically in one pass.
//! - Prevent substituted arguments from being expanded recursively.
//! - Enforce optional, caller-configured operational budgets.
//! - Return structured errors.
//! - Never execute generated source or host-language code.
//!
//! # Non-responsibilities
//!
//! This module does not:
//!
//! - parse Zamani source;
//! - recognize macro declarations or invocations;
//! - resolve qualified names or imports;
//! - implement token-tree parsing;
//! - construct or mutate the canonical AST;
//! - implement AST macro expansion;
//! - implement identifier hygiene;
//! - allocate canonical AST NodeIds;
//! - create source-map entries;
//! - perform semantic validation;
//! - evaluate compile-time expressions;
//! - execute Rust, shell commands, or generated programs;
//! - access files, networks, processes, devices, or hardware;
//! - select targets or allocate resources;
//! - construct ZUIR or domain-specific IR.
//!
//! Those responsibilities remain with their canonical repository owners.
//!
//! # Template syntax
//!
//! The compatibility template API uses `{parameter}` placeholders.
//!
//! A placeholder is substituted only when its contents exactly match a
//! declared parameter name. Braces containing other text are preserved.
//! Substitution is single-pass: text supplied as an argument is never
//! interpreted as another placeholder.
//!
//! This syntax is a utility API, not the normative syntax of Zamani macros.
//! Source-language macro arguments must use the canonical parsed syntax and
//! token-tree contracts.
//!
//! # Resource policy
//!
//! No fixed expansion size, nesting depth, macro count, parameter count,
//! source size, or hardware capacity is imposed by default.
//!
//! Optional limits are operational admission policies. A configured limit
//! does not define a language or hardware capability.
//!
//! No finite implementation can process literally infinite input. Actual
//! processing remains bounded by available memory, address space, execution
//! time, and caller-configured budgets.
//!
//! # Concurrency
//!
//! An engine can be shared immutably when its contents are no longer being
//! changed. Registration and removal require exclusive mutable access.
//! There is no hidden global mutable state or nondeterministic registry order.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::ops::Range;

// =============================================================================
// Macro definitions
// =============================================================================

/// A textual macro template.
///
/// This type is retained for compatibility with existing compiler callers.
///
/// Source-language macro declarations must first be parsed into the
/// repository's canonical AST. An AST adapter may then deliberately lower a
/// supported, restricted declaration into this utility representation.
///
/// `name`, `parameters`, and `template` are public for compatibility.
/// Registration validates and compiles a snapshot of the definition, so
/// subsequent caller-side mutation cannot change an already registered macro.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroDefinition {
    /// Macro registry key.
    ///
    /// Qualified names such as `module::name` are accepted. Name resolution,
    /// visibility and import semantics belong to the frontend.
    pub name: String,

    /// Positional template parameters, in declaration order.
    pub parameters: Vec<String>,

    /// Template source containing `{parameter}` placeholders.
    pub template: String,
}

impl MacroDefinition {
    /// Construct and validate a textual macro definition.
    pub fn new(
        name: impl Into<String>,
        parameters: Vec<String>,
        template: impl Into<String>,
    ) -> Result<Self, MacroError> {
        let definition = Self {
            name: name.into(),
            parameters,
            template: template.into(),
        };

        definition.validate()?;
        Ok(definition)
    }

    /// Validate the definition independently of registration.
    ///
    /// This performs structural checks required by this template facility.
    /// It intentionally does not replace canonical frontend identifier,
    /// visibility, type, or semantic validation.
    pub fn validate(&self) -> Result<(), MacroError> {
        validate_macro_key(&self.name)?;

        let mut parameter_indices = BTreeMap::new();

        for (index, parameter) in self.parameters.iter().enumerate() {
            validate_parameter_key(&self.name, parameter)?;

            if parameter_indices.insert(parameter.clone(), index).is_some() {
                return Err(MacroError::DuplicateParameter {
                    macro_name: self.name.clone(),
                    parameter: parameter.clone(),
                });
            }
        }

        compile_template(self, &parameter_indices)?;
        Ok(())
    }
}

// =============================================================================
// Configuration and operational budgets
// =============================================================================

/// Optional operational safeguards for template registration and expansion.
///
/// `None` means that this engine does not impose that particular budget.
/// The caller, compiler session, or service boundary remains responsible for
/// selecting policies appropriate to its available resources.
///
/// These are not language-level limits.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MacroEngineConfig {
    /// Optional maximum number of bytes in one expanded result.
    ///
    /// This counts UTF-8 bytes, not Unicode scalar values or display columns.
    pub max_expansion_size: Option<usize>,

    /// Optional maximum number of registered templates.
    pub max_registered_macros: Option<usize>,
}

impl MacroEngineConfig {
    /// Construct an unrestricted configuration.
    ///
    /// "Unrestricted" means that this module applies no configured numeric
    /// admission limit. It does not promise infinite physical resources.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_expansion_size: None,
            max_registered_macros: None,
        }
    }

    /// Construct a configuration with an output-byte budget.
    #[must_use]
    pub const fn with_max_expansion_size(mut self, limit: usize) -> Self {
        self.max_expansion_size = Some(limit);
        self
    }

    /// Construct a configuration with a registry-size budget.
    #[must_use]
    pub const fn with_max_registered_macros(mut self, limit: usize) -> Self {
        self.max_registered_macros = Some(limit);
        self
    }
}

// =============================================================================
// Compiled template representation
// =============================================================================

/// A prevalidated segment in a registered template.
///
/// Literal segments refer to byte ranges in the immutable registered
/// definition. Parameter segments refer to positional argument indices.
#[derive(Debug, Clone, PartialEq, Eq)]
enum TemplatePart {
    Literal(Range<usize>),
    Parameter(usize),
}

/// A registered macro and its precompiled substitution plan.
#[derive(Debug, Clone)]
struct RegisteredMacro {
    definition: MacroDefinition,
    parts: Vec<TemplatePart>,
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by this macro-template facility.
///
/// These errors describe template registration and expansion only.
/// Frontend parsing, name resolution, hygiene, semantic validation and
/// compiler-pipeline failures retain their owning error types.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MacroError {
    /// A registry key is empty or structurally invalid.
    InvalidMacroName(String),

    /// A parameter key is empty or structurally invalid.
    InvalidParameterName(String),

    /// The same parameter occurs more than once.
    DuplicateParameter {
        macro_name: String,
        parameter: String,
    },

    /// A template refers to a parameter not declared by the definition.
    UnknownParameter {
        macro_name: String,
        parameter: String,
    },

    /// The template contains an invalid placeholder form.
    InvalidPlaceholder {
        macro_name: String,
        offset: usize,
    },

    /// A macro with the same registry key already exists.
    MacroAlreadyRegistered {
        name: String,
    },

    /// No registered macro has the requested key.
    MacroNotFound {
        name: String,
    },

    /// The number of supplied arguments differs from the parameter count.
    ArgumentCountMismatch {
        macro_name: String,
        expected: usize,
        actual: usize,
    },

    /// Expansion would exceed the configured output-byte budget.
    ExpansionTooLarge {
        macro_name: String,
        attempted: usize,
        limit: usize,
    },

    /// The configured registry-size budget has been reached.
    RegistryLimitExceeded {
        limit: usize,
    },

    /// Checked size accounting could not represent the requested result.
    SizeOverflow {
        macro_name: String,
    },

    /// A caller supplied an invalid configuration.
    InvalidConfiguration(String),
}

impl fmt::Display for MacroError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMacroName(name) => {
                write!(formatter, "invalid macro registry key: {name:?}")
            }
            Self::InvalidParameterName(name) => {
                write!(formatter, "invalid macro parameter key: {name:?}")
            }
            Self::DuplicateParameter {
                macro_name,
                parameter,
            } => {
                write!(
                    formatter,
                    "macro {macro_name:?} declares parameter {parameter:?} more than once"
                )
            }
            Self::UnknownParameter {
                macro_name,
                parameter,
            } => {
                write!(
                    formatter,
                    "macro {macro_name:?} references undeclared parameter {parameter:?}"
                )
            }
            Self::InvalidPlaceholder {
                macro_name,
                offset,
            } => {
                write!(
                    formatter,
                    "macro {macro_name:?} contains an invalid placeholder at byte offset {offset}"
                )
            }
            Self::MacroAlreadyRegistered { name } => {
                write!(formatter, "macro {name:?} is already registered")
            }
            Self::MacroNotFound { name } => {
                write!(formatter, "macro {name:?} was not found")
            }
            Self::ArgumentCountMismatch {
                macro_name,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "macro {macro_name:?} expects {expected} argument(s), received {actual}"
                )
            }
            Self::ExpansionTooLarge {
                macro_name,
                attempted,
                limit,
            } => {
                write!(
                    formatter,
                    "expansion of {macro_name:?} requires {attempted} bytes; configured limit is {limit}"
                )
            }
            Self::RegistryLimitExceeded { limit } => {
                write!(
                    formatter,
                    "macro registry admission limit of {limit} entries has been reached"
                )
            }
            Self::SizeOverflow { macro_name } => {
                write!(
                    formatter,
                    "expanded size accounting overflowed for macro {macro_name:?}"
                )
            }
            Self::InvalidConfiguration(message) => {
                write!(formatter, "invalid macro engine configuration: {message}")
            }
        }
    }
}

impl Error for MacroError {}

// =============================================================================
// Engine
// =============================================================================

/// Deterministic registry and expansion engine for textual templates.
///
/// The registry uses `BTreeMap` so enumeration is deterministic and does not
/// depend on randomized hash-map iteration order.
///
/// The engine does not recursively parse generated text. In particular,
/// argument contents are appended as opaque text and are never re-expanded.
#[derive(Debug, Clone, Default)]
pub struct MacroEngine {
    macros: BTreeMap<String, RegisteredMacro>,
    config: MacroEngineConfig,
}

impl MacroEngine {
    /// Create an empty engine with no configured numeric limits.
    ///
    /// Standard-library or application macros should be registered explicitly
    /// by the relevant library/provider. The engine does not hard-code a
    /// language-level built-in macro catalog.
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(MacroEngineConfig::unrestricted())
            .expect("the unrestricted macro configuration is valid")
    }

    /// Create an engine with explicit operational policy.
    pub fn with_config(config: MacroEngineConfig) -> Result<Self, MacroError> {
        Ok(Self {
            macros: BTreeMap::new(),
            config,
        })
    }

    /// Return the engine configuration.
    #[must_use]
    pub const fn config(&self) -> &MacroEngineConfig {
        &self.config
    }

    /// Register a definition without replacing an existing definition.
    ///
    /// Validation and template compilation happen before registry mutation.
    /// A failed registration therefore leaves the registry unchanged.
    pub fn register(
        &mut self,
        definition: MacroDefinition,
    ) -> Result<(), MacroError> {
        definition.validate()?;

        if self.macros.contains_key(&definition.name) {
            return Err(MacroError::MacroAlreadyRegistered {
                name: definition.name,
            });
        }

        self.check_registry_admission()?;

        let registered = compile_definition(definition)?;
        self.macros
            .insert(registered.definition.name.clone(), registered);

        Ok(())
    }

    /// Explicitly insert or replace a registered definition.
    ///
    /// The replacement is fully validated and compiled before the existing
    /// entry is changed. If validation fails, the old definition remains.
    ///
    /// Returns the previous definition, if one existed.
    pub fn register_or_replace(
        &mut self,
        definition: MacroDefinition,
    ) -> Result<Option<MacroDefinition>, MacroError> {
        definition.validate()?;

        let is_new = !self.macros.contains_key(&definition.name);
        if is_new {
            self.check_registry_admission()?;
        }

        let registered = compile_definition(definition)?;
        let previous = self
            .macros
            .insert(registered.definition.name.clone(), registered);

        Ok(previous.map(|entry| entry.definition))
    }

    /// Remove a registered definition.
    pub fn unregister(&mut self, name: &str) -> Option<MacroDefinition> {
        self.macros.remove(name).map(|entry| entry.definition)
    }

    /// Check whether a definition is registered.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.macros.contains_key(name)
    }

    /// Borrow a registered definition.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&MacroDefinition> {
        self.macros.get(name).map(|entry| &entry.definition)
    }

    /// Return the number of registered definitions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.macros.len()
    }

    /// Return whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.macros.is_empty()
    }

    /// Return registry keys in deterministic lexical order.
    #[must_use]
    pub fn macro_names(&self) -> Vec<String> {
        self.macros.keys().cloned().collect()
    }

    /// Expand a registered macro using owned string arguments.
    pub fn expand(
        &self,
        macro_name: &str,
        args: &[String],
    ) -> Result<String, MacroError> {
        let borrowed: Vec<&str> = args.iter().map(String::as_str).collect();
        self.expand_str(macro_name, &borrowed)
    }

    /// Expand a registered macro using borrowed string arguments.
    ///
    /// This method avoids cloning each argument before expansion.
    pub fn expand_str(
        &self,
        macro_name: &str,
        args: &[&str],
    ) -> Result<String, MacroError> {
        let registered = self
            .macros
            .get(macro_name)
            .ok_or_else(|| MacroError::MacroNotFound {
                name: macro_name.to_owned(),
            })?;

        if args.len() != registered.definition.parameters.len() {
            return Err(MacroError::ArgumentCountMismatch {
                macro_name: macro_name.to_owned(),
                expected: registered.definition.parameters.len(),
                actual: args.len(),
            });
        }

        self.expand_registered(registered, args)
    }

    /// Return the configured maximum output size, if one is configured.
    #[must_use]
    pub const fn max_expansion_size(&self) -> Option<usize> {
        self.config.max_expansion_size
    }

    /// Return the configured maximum number of registered macros, if any.
    #[must_use]
    pub const fn max_registered_macros(&self) -> Option<usize> {
        self.config.max_registered_macros
    }

    fn check_registry_admission(&self) -> Result<(), MacroError> {
        if let Some(limit) = self.config.max_registered_macros {
            if self.macros.len() >= limit {
                return Err(MacroError::RegistryLimitExceeded { limit });
            }
        }

        Ok(())
    }

    fn expand_registered(
        &self,
        registered: &RegisteredMacro,
        args: &[&str],
    ) -> Result<String, MacroError> {
        let definition = &registered.definition;

        // First calculate the exact output size with checked arithmetic.
        // This prevents allocating a partial output before detecting a
        // configured limit violation.
        let mut output_size = 0usize;

        for part in &registered.parts {
            let part_size = match part {
                TemplatePart::Literal(range) => range.len(),
                TemplatePart::Parameter(index) => args[*index].len(),
            };

            output_size = output_size
                .checked_add(part_size)
                .ok_or_else(|| MacroError::SizeOverflow {
                    macro_name: definition.name.clone(),
                })?;

            if let Some(limit) = self.config.max_expansion_size {
                if output_size > limit {
                    return Err(MacroError::ExpansionTooLarge {
                        macro_name: definition.name.clone(),
                        attempted: output_size,
                        limit,
                    });
                }
            }
        }

        let mut output = String::with_capacity(output_size);

        // The compiled plan contains only validated byte ranges and valid
        // parameter indices. No user input is parsed or executed here.
        for part in &registered.parts {
            match part {
                TemplatePart::Literal(range) => {
                    output.push_str(&definition.template[range.clone()]);
                }
                TemplatePart::Parameter(index) => {
                    output.push_str(args[*index]);
                }
            }
        }

        debug_assert_eq!(output.len(), output_size);
        Ok(output)
    }
}

// =============================================================================
// Validation and template compilation
// =============================================================================

/// Validate a registry key without attempting to replace frontend name
/// resolution or canonical identifier validation.
///
/// Qualified names are accepted as registry keys. The frontend remains the
/// authority for whether a particular spelling is a valid Zamani name.
fn validate_macro_key(name: &str) -> Result<(), MacroError> {
    if name.is_empty()
        || name.trim() != name
        || name.chars().any(char::is_whitespace)
        || name.contains('\0')
        || name.starts_with("::")
        || name.ends_with("::")
        || name.contains("::::")
    {
        return Err(MacroError::InvalidMacroName(name.to_owned()));
    }

    Ok(())
}

/// Validate a parameter key supplied by a canonical caller or template user.
fn validate_parameter_key(
    macro_name: &str,
    parameter: &str,
) -> Result<(), MacroError> {
    if parameter.is_empty()
        || parameter.trim() != parameter
        || parameter.chars().any(char::is_whitespace)
        || parameter.contains('\0')
        || parameter.contains('{')
        || parameter.contains('}')
        || parameter.contains(':')
    {
        return Err(MacroError::InvalidParameterName(
            parameter.to_owned(),
        ));
    }

    let _ = macro_name;
    Ok(())
}

/// Compile a definition into literal and parameter segments.
fn compile_definition(
    definition: MacroDefinition,
) -> Result<RegisteredMacro, MacroError> {
    let mut indices = BTreeMap::new();

    for (index, parameter) in definition.parameters.iter().enumerate() {
        indices.insert(parameter.clone(), index);
    }

    let parts = compile_template(&definition, &indices)?;

    Ok(RegisteredMacro { definition, parts })
}

/// Parse the compatibility `{parameter}` placeholder notation.
///
/// Only a brace pair whose interior is a non-empty placeholder key is
/// interpreted as a placeholder. Other braces remain literal, allowing
/// ordinary source blocks to appear in templates.
///
/// An unknown identifier-shaped placeholder is an error. It is not silently
/// emitted as generated source.
fn compile_template(
    definition: &MacroDefinition,
    parameter_indices: &BTreeMap<String, usize>,
) -> Result<Vec<TemplatePart>, MacroError> {
    let template = definition.template.as_str();
    let bytes = template.as_bytes();

    let mut parts = Vec::new();
    let mut literal_start = 0usize;
    let mut cursor = 0usize;

    while cursor < bytes.len() {
        if bytes[cursor] != b'{' {
            cursor += 1;
            continue;
        }

        let Some(relative_end) = bytes[cursor + 1..]
            .iter()
            .position(|byte| *byte == b'}')
        else {
            // An unmatched brace is ordinary literal source text.
            cursor += 1;
            continue;
        };

        let closing = cursor + 1 + relative_end;
        let placeholder = &template[cursor + 1..closing];

        if !is_placeholder_key(placeholder) {
            // Braces containing block text or other non-placeholder source
            // remain literal. Continue scanning after the opening brace so
            // a later valid placeholder can still be found.
            cursor += 1;
            continue;
        }

        let Some(parameter_index) = parameter_indices.get(placeholder) else {
            return Err(MacroError::UnknownParameter {
                macro_name: definition.name.clone(),
                parameter: placeholder.to_owned(),
            });
        };

        if literal_start < cursor {
            parts.push(TemplatePart::Literal(literal_start..cursor));
        }

        parts.push(TemplatePart::Parameter(*parameter_index));

        cursor = closing + 1;
        literal_start = cursor;
    }

    if literal_start < template.len() {
        parts.push(TemplatePart::Literal(literal_start..template.len()));
    }

    Ok(parts)
}

/// Determine whether a brace-delimited string is a template parameter key.
///
/// This is deliberately a template-placeholder rule, not a replacement for
/// the Zamani frontend's canonical identifier grammar.
fn is_placeholder_key(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }

    let mut characters = value.chars();

    let Some(first) = characters.next() else {
        return false;
    };

    (first == '_' || first.is_alphabetic())
        && characters.all(|character| {
            character == '_' || character.is_alphanumeric()
        })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn definition(
        name: &str,
        parameters: &[&str],
        template: &str,
    ) -> MacroDefinition {
        MacroDefinition::new(
            name,
            parameters.iter().map(|value| (*value).to_owned()).collect(),
            template,
        )
        .expect("test definition should be valid")
    }

    #[test]
    fn new_engine_is_empty_and_unrestricted() {
        let engine = MacroEngine::new();

        assert!(engine.is_empty());
        assert_eq!(engine.len(), 0);
        assert_eq!(engine.max_expansion_size(), None);
        assert_eq!(engine.max_registered_macros(), None);
    }

    #[test]
    fn basic_expansion_works() {
        let mut engine = MacroEngine::new();

        engine
            .register(definition("hello", &["name"], "hello {name}"))
            .expect("registration");

        assert_eq!(
            engine.expand_str("hello", &["Zamani"]).expect("expansion"),
            "hello Zamani"
        );
    }

    #[test]
    fn owned_argument_api_remains_available() {
        let mut engine = MacroEngine::new();

        engine
            .register(definition("join", &["left", "right"], "{left}:{right}"))
            .expect("registration");

        let result = engine
            .expand("join", &["one".to_owned(), "two".to_owned()])
            .expect("expansion");

        assert_eq!(result, "one:two");
    }

    #[test]
    fn argument_text_is_not_recursively_substituted() {
        let mut engine = MacroEngine::new();

        engine
            .register(definition("inner", &[], "expanded"))
            .expect("registration");

        engine
            .register(definition("outer", &["value"], "{value}"))
            .expect("registration");

        let result = engine
            .expand_str("outer", &["inner!() {other}"])
            .expect("expansion");

        assert_eq!(result, "inner!() {other}");
    }

    #[test]
    fn source_block_braces_are_preserved() {
        let mut engine = MacroEngine::new();

        engine
            .register(definition(
                "block",
                &["value"],
                "fn example() { let x = {value}; }",
            ))
            .expect("registration");

        assert_eq!(
            engine.expand_str("block", &["42"]).expect("expansion"),
            "fn example() { let x = 42; }"
        );
    }

    #[test]
    fn longer_parameter_names_do_not_interfere() {
        let mut engine = MacroEngine::new();

        engine
            .register(definition(
                "pair",
                &["x", "x_long"],
                "{x_long}:{x}",
            ))
            .expect("registration");

        assert_eq!(
            engine.expand_str("pair", &["short", "long"]).expect("expansion"),
            "long:short"
        );
    }

    #[test]
    fn unknown_placeholder_is_rejected() {
        let result = MacroDefinition::new(
            "invalid",
            vec!["known".to_owned()],
            "{unknown}",
        );

        assert!(matches!(
            result,
            Err(MacroError::UnknownParameter { .. })
        ));
    }

    #[test]
    fn duplicate_parameters_are_rejected() {
        let result = MacroDefinition::new(
            "invalid",
            vec!["value".to_owned(), "value".to_owned()],
            "{value}",
        );

        assert!(matches!(
            result,
            Err(MacroError::DuplicateParameter { .. })
        ));
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut engine = MacroEngine::new();
        let item = definition("sample", &[], "value");

        engine.register(item.clone()).expect("first registration");

        assert!(matches!(
            engine.register(item),
            Err(MacroError::MacroAlreadyRegistered { .. })
        ));
    }

    #[test]
    fn replacement_is_explicit_and_returns_previous_definition() {
        let mut engine = MacroEngine::new();

        engine
            .register(definition("sample", &[], "first"))
            .expect("registration");

        let previous = engine
            .register_or_replace(definition("sample", &[], "second"))
            .expect("replacement")
            .expect("previous definition");

        assert_eq!(previous.template, "first");
        assert_eq!(engine.expand_str("sample", &[]).expect("expansion"), "second");
    }

    #[test]
    fn failed_replacement_preserves_previous_definition() {
        let mut engine = MacroEngine::new();

        engine
            .register(definition("sample", &[], "first"))
            .expect("registration");

        let result = engine.register_or_replace(
            MacroDefinition {
                name: "sample".to_owned(),
                parameters: vec!["declared".to_owned()],
                template: "{missing}".to_owned(),
            },
        );

        assert!(matches!(
            result,
            Err(MacroError::UnknownParameter { .. })
        ));

        assert_eq!(engine.expand_str("sample", &[]).expect("expansion"), "first");
    }

    #[test]
    fn exact_argument_arity_is_required() {
        let mut engine = MacroEngine::new();

        engine
            .register(definition("pair", &["left", "right"], "{left}{right}"))
            .expect("registration");

        assert!(matches!(
            engine.expand_str("pair", &["only_one"]),
            Err(MacroError::ArgumentCountMismatch {
                expected: 2,
                actual: 1,
                ..
            })
        ));
    }

    #[test]
    fn missing_macro_is_reported() {
        let engine = MacroEngine::new();

        assert!(matches!(
            engine.expand_str("missing", &[]),
            Err(MacroError::MacroNotFound { .. })
        ));
    }

    #[test]
    fn output_budget_is_enforced_before_output_is_built() {
        let config = MacroEngineConfig::unrestricted().with_max_expansion_size(4);
        let mut engine = MacroEngine::with_config(config).expect("configuration");

        engine
            .register(definition("sample", &["value"], "x={value}"))
            .expect("registration");

        assert!(matches!(
            engine.expand_str("sample", &["12345"]),
            Err(MacroError::ExpansionTooLarge { limit: 4, .. })
        ));
    }

    #[test]
    fn zero_output_budget_allows_only_empty_output() {
        let config = MacroEngineConfig::unrestricted().with_max_expansion_size(0);
        let mut engine = MacroEngine::with_config(config).expect("configuration");

        engine
            .register(definition("empty", &[], ""))
            .expect("registration");

        assert_eq!(engine.expand_str("empty", &[]).expect("empty expansion"), "");
    }

    #[test]
    fn registry_budget_is_enforced() {
        let config = MacroEngineConfig::unrestricted().with_max_registered_macros(1);
        let mut engine = MacroEngine::with_config(config).expect("configuration");

        engine
            .register(definition("first", &[], "one"))
            .expect("first registration");

        assert!(matches!(
            engine.register(definition("second", &[], "two")),
            Err(MacroError::RegistryLimitExceeded { limit: 1 })
        ));
    }

    #[test]
    fn replacement_does_not_consume_an_additional_registry_slot() {
        let config = MacroEngineConfig::unrestricted().with_max_registered_macros(1);
        let mut engine = MacroEngine::with_config(config).expect("configuration");

        engine
            .register(definition("sample", &[], "first"))
            .expect("registration");

        engine
            .register_or_replace(definition("sample", &[], "second"))
            .expect("replacement");

        assert_eq!(engine.len(), 1);
    }

    #[test]
    fn macro_names_are_returned_in_deterministic_order() {
        let mut engine = MacroEngine::new();

        engine.register(definition("zeta", &[], "z")).expect("registration");
        engine.register(definition("alpha", &[], "a")).expect("registration");
        engine.register(definition("middle", &[], "m")).expect("registration");

        assert_eq!(
            engine.macro_names(),
            vec![
                "alpha".to_owned(),
                "middle".to_owned(),
                "zeta".to_owned(),
            ]
        );
    }

    #[test]
    fn unregister_removes_a_definition() {
        let mut engine = MacroEngine::new();

        engine
            .register(definition("temporary", &[], "value"))
            .expect("registration");

        let removed = engine.unregister("temporary");

        assert!(removed.is_some());
        assert!(!engine.contains("temporary"));
    }

    #[test]
    fn empty_template_is_supported() {
        let mut engine = MacroEngine::new();

        engine
            .register(definition("empty", &[], ""))
            .expect("registration");

        assert_eq!(engine.expand_str("empty", &[]).expect("expansion"), "");
    }

    #[test]
    fn unicode_argument_text_is_preserved() {
        let mut engine = MacroEngine::new();

        engine
            .register(definition("identity", &["value"], "{value}"))
            .expect("registration");

        assert_eq!(
            engine.expand_str("identity", &["量子 вычисление"]).expect("expansion"),
            "量子 вычисление"
        );
    }

    #[test]
    fn registered_definition_is_a_snapshot() {
        let mut engine = MacroEngine::new();

        let mut definition = definition("snapshot", &["value"], "{value}");
        engine.register(definition.clone()).expect("registration");

        definition.template = "changed".to_owned();

        assert_eq!(
            engine.expand_str("snapshot", &["original"]).expect("expansion"),
            "original"
        );
    }
}
