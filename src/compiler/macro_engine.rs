//! Zamani Compiler — MacroEngine
//!
//! Production-oriented compile-time macro registration and expansion.
//!
//! Design principles:
//! - deterministic expansion
//! - explicit validation
//! - no silent macro replacement
//! - exact argument arity
//! - bounded expansion
//! - recursion protection
//! - no execution of generated code during expansion
//! - dependency-free core implementation
//!
//! Macro expansion produces source/IR text. It does not execute the generated
//! result. Execution belongs to later compiler/runtime stages.

use std::collections::{HashMap, HashSet};
use std::fmt;

const DEFAULT_MAX_EXPANSION_SIZE: usize = 1024 * 1024;
const DEFAULT_MAX_EXPANSION_DEPTH: usize = 64;

/// A compile-time macro definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroDefinition {
    /// Macro identifier.
    pub name: String,

    /// Positional parameter names.
    pub parameters: Vec<String>,

    /// Expansion template.
    ///
    /// Parameters are referenced using:
    ///
    /// `{parameter_name}`
    pub template: String,
}

impl MacroDefinition {
    /// Create a macro definition.
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

    /// Validate the complete macro definition.
    pub fn validate(&self) -> Result<(), MacroError> {
        validate_identifier(&self.name)?;

        if self.template.is_empty() {
            return Err(MacroError::EmptyTemplate {
                macro_name: self.name.clone(),
            });
        }

        let mut parameters = HashSet::new();

        for parameter in &self.parameters {
            validate_identifier(parameter)?;

            if !parameters.insert(parameter) {
                return Err(MacroError::DuplicateParameter {
                    macro_name: self.name.clone(),
                    parameter: parameter.clone(),
                });
            }
        }

        /*
         * Validate placeholders before registering the macro.
         *
         * This prevents malformed definitions from surviving until a later
         * compilation stage.
         */
        for placeholder in extract_placeholders(&self.template) {
            if !parameters.contains(&placeholder) {
                return Err(MacroError::UnknownParameter {
                    macro_name: self.name.clone(),
                    parameter: placeholder,
                });
            }
        }

        Ok(())
    }
}

/// Configuration controlling macro expansion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroEngineConfig {
    /// Maximum generated expansion size.
    pub max_expansion_size: usize,

    /// Maximum nested macro expansion depth.
    pub max_expansion_depth: usize,
}

impl Default for MacroEngineConfig {
    fn default() -> Self {
        Self {
            max_expansion_size: DEFAULT_MAX_EXPANSION_SIZE,
            max_expansion_depth: DEFAULT_MAX_EXPANSION_DEPTH,
        }
    }
}

/// Errors produced by the macro engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacroError {
    InvalidMacroName(String),
    InvalidParameterName(String),

    EmptyTemplate {
        macro_name: String,
    },

    DuplicateParameter {
        macro_name: String,
        parameter: String,
    },

    UnknownParameter {
        macro_name: String,
        parameter: String,
    },

    MacroAlreadyRegistered {
        name: String,
    },

    MacroNotFound {
        name: String,
    },

    ArgumentCountMismatch {
        macro_name: String,
        expected: usize,
        actual: usize,
    },

    UnresolvedPlaceholder {
        macro_name: String,
        placeholder: String,
    },

    ExpansionTooLarge {
        macro_name: String,
        size: usize,
        limit: usize,
    },

    ExpansionDepthExceeded {
        limit: usize,
    },

    RecursiveExpansion {
        name: String,
    },

    InvalidConfiguration(String),
}

impl fmt::Display for MacroError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMacroName(name) => {
                write!(formatter, "invalid macro name '{}'", name)
            }

            Self::InvalidParameterName(name) => {
                write!(formatter, "invalid macro parameter '{}'", name)
            }

            Self::EmptyTemplate { macro_name } => {
                write!(
                    formatter,
                    "macro '{}' has an empty template",
                    macro_name
                )
            }

            Self::DuplicateParameter {
                macro_name,
                parameter,
            } => {
                write!(
                    formatter,
                    "macro '{}' declares duplicate parameter '{}'",
                    macro_name,
                    parameter
                )
            }

            Self::UnknownParameter {
                macro_name,
                parameter,
            } => {
                write!(
                    formatter,
                    "macro '{}' references unknown parameter '{}'",
                    macro_name,
                    parameter
                )
            }

            Self::MacroAlreadyRegistered { name } => {
                write!(
                    formatter,
                    "macro '{}' is already registered",
                    name
                )
            }

            Self::MacroNotFound { name } => {
                write!(formatter, "macro not found: '{}'", name)
            }

            Self::ArgumentCountMismatch {
                macro_name,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "macro '{}' expects {} argument(s), received {}",
                    macro_name,
                    expected,
                    actual
                )
            }

            Self::UnresolvedPlaceholder {
                macro_name,
                placeholder,
            } => {
                write!(
                    formatter,
                    "macro '{}' contains unresolved placeholder '{}'",
                    macro_name,
                    placeholder
                )
            }

            Self::ExpansionTooLarge {
                macro_name,
                size,
                limit,
            } => {
                write!(
                    formatter,
                    "expansion of '{}' is {} bytes, exceeding limit of {} bytes",
                    macro_name,
                    size,
                    limit
                )
            }

            Self::ExpansionDepthExceeded { limit } => {
                write!(
                    formatter,
                    "macro expansion depth exceeded configured limit of {}",
                    limit
                )
            }

            Self::RecursiveExpansion { name } => {
                write!(
                    formatter,
                    "recursive macro expansion detected for '{}'",
                    name
                )
            }

            Self::InvalidConfiguration(message) => {
                write!(
                    formatter,
                    "invalid macro engine configuration: {}",
                    message
                )
            }
        }
    }
}

impl std::error::Error for MacroError {}

/// Production-oriented compile-time macro engine.
#[derive(Debug, Clone)]
pub struct MacroEngine {
    macros: HashMap<String, MacroDefinition>,
    config: MacroEngineConfig,
}

impl Default for MacroEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MacroEngine {
    /// Create an engine with production-safe defaults.
    pub fn new() -> Self {
        Self::with_config(MacroEngineConfig::default())
            .expect("default MacroEngine configuration must be valid")
    }

    /// Create an engine with explicit configuration.
    pub fn with_config(config: MacroEngineConfig) -> Result<Self, MacroError> {
        validate_config(&config)?;

        let mut engine = Self {
            macros: HashMap::new(),
            config,
        };

        /*
         * Built-in macros are registered through the same validation path as
         * user-defined macros.
         */
        engine.register(MacroDefinition {
            name: "assert_omni".into(),
            parameters: vec!["condition".into()],
            template:
                "if !({condition}) { panic(\"Omniversal Assertion Failed: {condition}\"); }"
                    .into(),
        })?;

        Ok(engine)
    }

    /// Register a macro.
    ///
    /// Registration is intentionally fail-closed: an existing macro cannot
    /// silently be replaced.
    pub fn register(&mut self, definition: MacroDefinition) -> Result<(), MacroError> {
        definition.validate()?;

        if self.macros.contains_key(&definition.name) {
            return Err(MacroError::MacroAlreadyRegistered {
                name: definition.name,
            });
        }

        self.macros
            .insert(definition.name.clone(), definition);

        Ok(())
    }

    /// Register or explicitly replace an existing macro.
    ///
    /// This operation is separate from `register()` so accidental replacement
    /// cannot occur.
    pub fn register_or_replace(
        &mut self,
        definition: MacroDefinition,
    ) -> Result<Option<MacroDefinition>, MacroError> {
        definition.validate()?;

        Ok(self
            .macros
            .insert(definition.name.clone(), definition))
    }

    /// Remove a macro.
    pub fn unregister(&mut self, name: &str) -> Option<MacroDefinition> {
        self.macros.remove(name)
    }

    /// Determine whether a macro exists.
    pub fn contains(&self, name: &str) -> bool {
        self.macros.contains_key(name)
    }

    /// Retrieve a macro definition.
    pub fn get(&self, name: &str) -> Option<&MacroDefinition> {
        self.macros.get(name)
    }

    /// Return the number of registered macros.
    pub fn len(&self) -> usize {
        self.macros.len()
    }

    /// Return whether no macros are registered.
    pub fn is_empty(&self) -> bool {
        self.macros.is_empty()
    }

    /// Return macro names in deterministic order.
    pub fn macro_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.macros.keys().cloned().collect();
        names.sort();
        names
    }

    /// Expand a macro.
    ///
    /// Arguments are substituted positionally.
    ///
    /// No generated code is executed here.
    pub fn expand(
        &self,
        macro_name: &str,
        args: &[String],
    ) -> Result<String, MacroError> {
        self.expand_internal(
            macro_name,
            args,
            0,
            &mut HashSet::new(),
        )
    }

    /// Expand a macro using borrowed argument strings.
    ///
    /// This avoids requiring callers to allocate `Vec<String>` when they
    /// already have string slices.
    pub fn expand_str(
        &self,
        macro_name: &str,
        args: &[&str],
    ) -> Result<String, MacroError> {
        let owned: Vec<String> = args.iter().map(|arg| (*arg).to_owned()).collect();
        self.expand(macro_name, &owned)
    }

    fn expand_internal(
        &self,
        macro_name: &str,
        args: &[String],
        depth: usize,
        active: &mut HashSet<String>,
    ) -> Result<String, MacroError> {
        if depth >= self.config.max_expansion_depth {
            return Err(MacroError::ExpansionDepthExceeded {
                limit: self.config.max_expansion_depth,
            });
        }

        let definition = self
            .macros
            .get(macro_name)
            .ok_or_else(|| MacroError::MacroNotFound {
                name: macro_name.to_string(),
            })?;

        if args.len() != definition.parameters.len() {
            return Err(MacroError::ArgumentCountMismatch {
                macro_name: macro_name.to_string(),
                expected: definition.parameters.len(),
                actual: args.len(),
            });
        }

        if !active.insert(macro_name.to_string()) {
            return Err(MacroError::RecursiveExpansion {
                name: macro_name.to_string(),
            });
        }

        let result = self.substitute(definition, args);

        active.remove(macro_name);

        result
    }

    /// Perform deterministic parameter substitution.
    fn substitute(
        &self,
        definition: &MacroDefinition,
        args: &[String],
    ) -> Result<String, MacroError> {
        let mut result = definition.template.clone();

        /*
         * Replace longer parameter names first.
         *
         * This avoids accidental interactions such as `{x}` and `{x_long}`
         * in simplistic replacement implementations.
         *
         * The placeholders themselves are exact, so ordinary text containing
         * a parameter's name is never replaced.
         */
        let mut parameters: Vec<(usize, &String, &String)> = definition
            .parameters
            .iter()
            .enumerate()
            .map(|(index, parameter)| {
                (index, parameter, &args[index])
            })
            .collect();

        parameters.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        for (_, parameter, argument) in parameters {
            let placeholder = format!("{{{}}}", parameter);

            result = result.replace(&placeholder, argument);

            if result.len() > self.config.max_expansion_size {
                return Err(MacroError::ExpansionTooLarge {
                    macro_name: definition.name.clone(),
                    size: result.len(),
                    limit: self.config.max_expansion_size,
                });
            }
        }

        /*
         * The definition was validated during registration, but verify again
         * before returning the expansion. This provides a defensive boundary
         * if future code mutates MacroDefinition internals.
         */
        for placeholder in extract_placeholders(&result) {
            return Err(MacroError::UnresolvedPlaceholder {
                macro_name: definition.name.clone(),
                placeholder,
            });
        }

        if result.len() > self.config.max_expansion_size {
            return Err(MacroError::ExpansionTooLarge {
                macro_name: definition.name.clone(),
                size: result.len(),
                limit: self.config.max_expansion_size,
            });
        }

        Ok(result)
    }

    /// Return the configured maximum expansion size.
    pub fn max_expansion_size(&self) -> usize {
        self.config.max_expansion_size
    }

    /// Return the configured maximum expansion depth.
    pub fn max_expansion_depth(&self) -> usize {
        self.config.max_expansion_depth
    }
}

/// Validate engine configuration.
fn validate_config(config: &MacroEngineConfig) -> Result<(), MacroError> {
    if config.max_expansion_size == 0 {
        return Err(MacroError::InvalidConfiguration(
            "max_expansion_size must be greater than zero".into(),
        ));
    }

    if config.max_expansion_depth == 0 {
        return Err(MacroError::InvalidConfiguration(
            "max_expansion_depth must be greater than zero".into(),
        ));
    }

    Ok(())
}

/// Validate a macro identifier.
fn validate_identifier(name: &str) -> Result<(), MacroError> {
    if name.is_empty() {
        return Err(MacroError::InvalidMacroName(name.to_string()));
    }

    let mut chars = name.chars();

    let first = chars
        .next()
        .ok_or_else(|| MacroError::InvalidMacroName(name.to_string()))?;

    if !(first == '_' || first.is_ascii_alphabetic()) {
        return Err(MacroError::InvalidMacroName(name.to_string()));
    }

    if !chars.all(|character| {
        character == '_'
            || character.is_ascii_alphanumeric()
    }) {
        return Err(MacroError::InvalidMacroName(name.to_string()));
    }

    Ok(())
}

/// Extract `{parameter}` placeholders from a macro template.
///
/// This intentionally implements only the macro engine's simple placeholder
/// syntax. More sophisticated token/AST macros should be implemented by a
/// future AST macro layer rather than making this textual engine ambiguous.
fn extract_placeholders(template: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut remaining = template;

    while let Some(start) = remaining.find('{') {
        let after_start = &remaining[start + 1..];

        let Some(end) = after_start.find('}') else {
            break;
        };

        let placeholder = &after_start[..end];

        if !placeholder.is_empty() {
            result.push(placeholder.to_string());
        }

        remaining = &after_start[end + 1..];
    }

    result
}

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
            parameters
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            template,
        )
        .unwrap()
    }

    #[test]
    fn builtin_macro_is_registered() {
        let engine = MacroEngine::new();

        assert!(engine.contains("assert_omni"));
        assert_eq!(engine.len(), 1);
    }

    #[test]
    fn basic_expansion_works() {
        let mut engine = MacroEngine::new();

        engine
            .register(definition(
                "hello",
                &["name"],
                "hello {name}",
            ))
            .unwrap();

        let result = engine
            .expand("hello", &["Zamani".to_string()])
            .unwrap();

        assert_eq!(result, "hello Zamani");
    }

    #[test]
    fn builtin_assert_macro_expands() {
        let engine = MacroEngine::new();

        let result = engine
            .expand("assert_omni", &["x > 0".to_string()])
            .unwrap();

        assert!(result.contains("x > 0"));
    }

    #[test]
    fn wrong_argument_count_is_rejected() {
        let mut engine = MacroEngine::new();

        engine
            .register(definition(
                "pair",
                &["a", "b"],
                "{a} + {b}",
            ))
            .unwrap();

        let result = engine.expand("pair", &["one".to_string()]);

        assert!(matches!(
            result,
            Err(MacroError::ArgumentCountMismatch {
                expected: 2,
                actual: 1,
                ..
            })
        ));
    }

    #[test]
    fn unknown_macro_is_rejected() {
        let engine = MacroEngine::new();

        let result = engine.expand("does_not_exist", &[]);

        assert!(matches!(
            result,
            Err(MacroError::MacroNotFound { .. })
        ));
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut engine = MacroEngine::new();

        let macro_definition =
            definition("hello", &[], "hello");

        engine
            .register(macro_definition.clone())
            .unwrap();

        let result = engine.register(macro_definition);

        assert!(matches!(
            result,
            Err(MacroError::MacroAlreadyRegistered { .. })
        ));
    }

    #[test]
    fn replacement_is_explicit() {
        let mut engine = MacroEngine::new();

        engine
            .register(definition(
                "hello",
                &[],
                "first",
            ))
            .unwrap();

        let old = engine
            .register_or_replace(definition(
                "hello",
                &[],
                "second",
            ))
            .unwrap();

        assert!(old.is_some());

        assert_eq!(
            engine.expand("hello", &[]).unwrap(),
            "second"
        );
    }

    #[test]
    fn duplicate_parameters_are_rejected() {
        let result = MacroDefinition::new(
            "invalid",
            vec!["x".into(), "x".into()],
            "{x}",
        );

        assert!(matches!(
            result,
            Err(MacroError::DuplicateParameter { .. })
        ));
    }

    #[test]
    fn unknown_template_parameter_is_rejected() {
        let result = MacroDefinition::new(
            "invalid",
            vec!["x".into()],
            "{unknown}",
        );

        assert!(matches!(
            result,
            Err(MacroError::UnknownParameter { .. })
        ));
    }

    #[test]
    fn invalid_macro_name_is_rejected() {
        let result = MacroDefinition::new(
            "123invalid",
            vec![],
            "test",
        );

        assert!(matches!(
            result,
            Err(MacroError::InvalidMacroName(_))
        ));
    }

    #[test]
    fn expansion_limit_is_enforced() {
        let config = MacroEngineConfig {
            max_expansion_size: 5,
            max_expansion_depth: 4,
        };

        let mut engine =
            MacroEngine::with_config(config).unwrap();

        engine
            .register(definition(
                "large",
                &["x"],
                "{x}",
            ))
            .unwrap();

        let result = engine.expand(
            "large",
            &["123456".to_string()],
        );

        assert!(matches!(
            result,
            Err(MacroError::ExpansionTooLarge { .. })
        ));
    }

    #[test]
    fn macro_names_are_deterministic() {
        let mut engine = MacroEngine::new();

        engine
            .register(definition("zeta", &[], "z"))
            .unwrap();

        engine
            .register(definition("alpha", &[], "a"))
            .unwrap();

        let names = engine.macro_names();

        assert_eq!(
            names,
            vec![
                "alpha".to_string(),
                "assert_omni".to_string(),
                "zeta".to_string()
            ]
        );
    }

    #[test]
    fn unregister_works() {
        let mut engine = MacroEngine::new();

        engine
            .register(definition(
                "temporary",
                &[],
                "value",
            ))
            .unwrap();

        assert!(engine.contains("temporary"));

        let removed = engine.unregister("temporary");

        assert!(removed.is_some());
        assert!(!engine.contains("temporary"));
    }

    #[test]
    fn string_slice_expansion_works() {
        let mut engine = MacroEngine::new();

        engine
            .register(definition(
                "concat",
                &["a", "b"],
                "{a}{b}",
            ))
            .unwrap();

        let result = engine
            .expand_str("concat", &["hello", "world"])
            .unwrap();

        assert_eq!(result, "helloworld");
    }

    #[test]
    fn placeholders_are_extracted() {
        let placeholders =
            extract_placeholders("foo {a} bar {b} baz {a}");

        assert_eq!(
            placeholders,
            vec![
                "a".to_string(),
                "b".to_string(),
                "a".to_string()
            ]
        );
    }
}