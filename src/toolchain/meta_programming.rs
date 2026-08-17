//! Zamani Universal Meta-Compiler (UMC): Autonomous Meta-Programming & Macros.
//!
//! This module provides the compiler-facing infrastructure for:
//! - macro definitions and expansion,
//! - autonomous code-generation requests,
//! - generated-code policy checks,
//! - meta-programming transformation rules,
//! - language-dialect registration.
//!
//! The module intentionally depends only on repository APIs that are available
//! in the current Zamani tree.  AST-specific meta declarations are represented
//! locally instead of requiring speculative AST variants.

use std::collections::HashMap;

use crate::ast::Identifier;
use crate::nimbus_os::evas::{EvasActionContext, EvasDecision};
use crate::runtime::sankofa::KnowledgeId;
use crate::source_map::Span;
use crate::stdlib::ai_reasoning::{KnowledgeBase, Planner};
use crate::stdlib::collections::{List, Map};
use crate::stdlib::crypto::{HomomorphicCiphertext, Signature};
use crate::stdlib::meta_ops::MetaValue;

// -----------------------------------------------------------------------------
// Lifecycle
// -----------------------------------------------------------------------------

/// Initializes the autonomous meta-programming subsystem.
pub fn init_meta_programming() {
    println!(
        "  - Initializing Zamani Autonomous Meta-Programming \
         & Macros Module..."
    );
}

/// Shuts down the autonomous meta-programming subsystem.
pub fn shutdown_meta_programming() {
    println!(
        "  - Shutting down Zamani Autonomous Meta-Programming \
         & Macros Module..."
    );
}

// -----------------------------------------------------------------------------
// Macro system
// -----------------------------------------------------------------------------

/// A Zamani macro definition.
///
/// Macro expansion remains represented as source text at this layer.  The
/// parser/IR pipeline can subsequently consume the generated source.
#[derive(Debug, Clone, PartialEq)]
pub struct MacroDefinition {
    pub name: Identifier,
    pub input_pattern: List<Identifier>,
    pub generator_logic: ZamaniCodeSnippet,
    pub context_constraints: Map<String, String>,
    pub security_policy_ref: Option<KnowledgeId>,
}

/// Source generated or transformed by the meta-programming subsystem.
pub type ZamaniCodeSnippet = String;

/// Macro expansion engine.
#[derive(Debug, Default)]
pub struct MacroProcessor;

impl MacroProcessor {
    pub fn new() -> Self {
        Self
    }

    /// Expands a registered macro.
    ///
    /// The current implementation validates registration and returns the
    /// generator source.  Actual argument substitution can be implemented
    /// later without changing the public API.
    pub fn expand_macro(
        &self,
        macro_name: &Identifier,
        _arguments: List<MetaValue>,
        known_macros: &Map<Identifier, MacroDefinition>,
    ) -> Result<ZamaniCodeSnippet, String> {
        let macro_def = known_macros.get(macro_name).ok_or_else(|| {
            format!("Macro {:?} not found.", macro_name)
        })?;

        if macro_def.generator_logic.trim().is_empty() {
            return Err(format!(
                "Macro {:?} has an empty generator body.",
                macro_def.name
            ));
        }

        Ok(macro_def.generator_logic.clone())
    }
}

// -----------------------------------------------------------------------------
// Autonomous code generation
// -----------------------------------------------------------------------------

/// Describes a request for autonomous code generation.
#[derive(Debug, Clone, PartialEq)]
pub struct CodeGenerationTask {
    pub task_id: Identifier,
    pub description: String,
    pub target_module: Identifier,
    pub constraints: List<String>,
    pub priority: u8,
}

/// Marker for future secure meta-programming policy state.
#[derive(Debug, Default)]
pub struct SecureMetaProgramming;

impl SecureMetaProgramming {
    pub fn new() -> Self {
        Self
    }
}

/// High-level autonomous code generator.
///
/// This component produces a deterministic source-level result for now.
/// Integrations with the AI planner and knowledge base can replace the
/// synthesis implementation without changing the task model.
#[derive(Debug, Default)]
pub struct AutonomousCodeGenerator;

impl AutonomousCodeGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Generates source from a goal and arbitrary constraint representation.
    ///
    /// Generic parameters preserve compatibility with callers that already
    /// pass richer goal/constraint objects.
    pub fn generate_code_from_goal<TGoal, TConstraints>(
        &self,
        goal: TGoal,
        _constraints: TConstraints,
    ) -> Result<ZamaniCodeSnippet, String>
    where
        TGoal: std::fmt::Debug,
    {
        Ok(format!(
            "/* Goal-generated Zamani code\n * Goal: {:?}\n */",
            goal
        ))
    }

    /// Produces an optimization proposal.
    pub fn autonomously_optimize_code<TCode, TGoal>(
        &self,
        code: TCode,
        goal: TGoal,
    ) -> Result<ZamaniCodeSnippet, String>
    where
        TCode: std::fmt::Debug,
        TGoal: std::fmt::Debug,
    {
        Ok(format!(
            "/* Optimized Zamani code\n * Source: {:?}\n * Goal: {:?}\n */",
            code, goal
        ))
    }

    /// Generates code for a structured task.
    pub fn generate_code(
        &self,
        task: &CodeGenerationTask,
        _ai_reasoner: &mut Planner,
        _knowledge_base: &KnowledgeBase,
    ) -> Result<ZamaniCodeSnippet, String> {
        if task.description.trim().is_empty() {
            return Err("Code-generation task description cannot be empty.".to_string());
        }

        Ok(format!(
            "/* Autonomously generated Zamani code\n * Task: {}\n * Target: {:?}\n * Priority: {}\n */",
            task.description,
            task.target_module,
            task.priority
        ))
    }

    /// Generates a deterministic signature request payload.
    ///
    /// The actual private-key operation belongs to the crypto subsystem.  We
    /// deliberately do not construct fake keys here.
    pub fn sign_generated_code(
        &self,
        _code_to_sign: ZamaniCodeSnippet,
        private_key_ref: Identifier,
    ) -> Result<Signature, String> {
        Err(format!(
            "Signing requires the configured crypto key identified by {:?}; \
             no private-key provider is exposed by the current meta-programming API.",
            private_key_ref
        ))
    }

    /// Sends generated-code intent through E.V.A.S.
    pub fn ethical_vetting_of_generated_code(
        code_snippet: ZamaniCodeSnippet,
    ) -> Result<EvasDecision, String> {
        let preview: String = code_snippet.chars().take(50).collect();

        let evas_action = EvasActionContext {
            action_type: "generated_code_deployment".to_string(),
            perceived_intent: format!("Deploy generated code: {}.", preview),
            initiating_context_id: crate::nimbus_os::get_current_context_id(),
            ..Default::default()
        };

        Ok(crate::nimbus_os::get_microkernel_evas_filter()
            .evaluate_action(evas_action))
    }

    /// Performs a homomorphic operation through the crypto subsystem.
    ///
    /// This remains an explicit capability boundary: the crypto implementation
    /// owns the actual ciphertext operation.
    pub fn homomorphic_meta_computation(
        encrypted_code: HomomorphicCiphertext,
        encrypted_data: HomomorphicCiphertext,
    ) -> Result<HomomorphicCiphertext, String> {
        crate::stdlib::crypto::Crypto::homomorphic_multiply(
            &encrypted_code,
            &encrypted_data,
        )
    }
}

// -----------------------------------------------------------------------------
// Meta transformation declarations
// -----------------------------------------------------------------------------

/// Repository-independent representation of a meta transformation.
///
/// This replaces the previously referenced `ast::MetaTransformDirective`,
/// which does not exist in the current AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformDirective {
    pub name: String,
    pub arguments: Vec<String>,
    pub span: Span,
}

impl TransformDirective {
    pub fn new(
        name: impl Into<String>,
        arguments: Vec<String>,
        span: Span,
    ) -> Self {
        Self {
            name: name.into(),
            arguments,
            span,
        }
    }
}

/// Repository-independent representation of a language dialect declaration.
///
/// This replaces the previously referenced `ast::LanguageDialectDecl`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageDialect {
    pub name: String,
    pub version: String,
    pub span: Span,
}

impl LanguageDialect {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        span: Span,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            span,
        }
    }
}

// -----------------------------------------------------------------------------
// Meta-transform engine
// -----------------------------------------------------------------------------

/// A pattern-based source/AST transformation rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformRule {
    pub name: String,
    pub pattern: String,
    pub replacement: String,
    pub span: Span,
}

/// Engine for registering and validating meta transformations.
#[derive(Debug, Clone, Default)]
pub struct MetaTransformEngine {
    rules: Vec<TransformRule>,
    dialects: HashMap<String, LanguageDialect>,
}

impl MetaTransformEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a transformation directive.
    pub fn register_transform(
        &mut self,
        directive: &TransformDirective,
    ) -> Result<(), String> {
        if directive.name.trim().is_empty() {
            return Err(format!(
                "Meta-transform name cannot be empty at {:?}.",
                directive.span
            ));
        }

        let pattern = if directive.arguments.is_empty() {
            directive.name.clone()
        } else {
            directive.arguments.join(" ")
        };

        let replacement = format!("transformed({})", directive.name);

        if pattern == replacement {
            return Err(format!(
                "Meta-transform '{}' is a no-op at {:?}.",
                directive.name, directive.span
            ));
        }

        if self.rules.iter().any(|rule| rule.name == directive.name) {
            return Err(format!(
                "Meta-transform '{}' is already registered.",
                directive.name
            ));
        }

        self.rules.push(TransformRule {
            name: directive.name.clone(),
            pattern,
            replacement,
            span: directive.span.clone(),
        });

        Ok(())
    }

    /// Registers a language dialect.
    pub fn register_dialect(
        &mut self,
        declaration: &LanguageDialect,
    ) -> Result<(), String> {
        if declaration.name.trim().is_empty() {
            return Err(format!(
                "Dialect name cannot be empty at {:?}.",
                declaration.span
            ));
        }

        if self.dialects.contains_key(&declaration.name) {
            return Err(format!(
                "Dialect '{}' already exists at {:?}.",
                declaration.name, declaration.span
            ));
        }

        self.dialects
            .insert(declaration.name.clone(), declaration.clone());

        Ok(())
    }

    /// Validates all registered transformation rules.
    ///
    /// The actual AST rewrite is intentionally left to the compiler transform
    /// pass.  This method therefore performs validation without inventing
    /// nonexistent AST types.
    pub fn validate(&self) -> Result<(), String> {
        for rule in &self.rules {
            if rule.name.trim().is_empty() {
                return Err(format!(
                    "Transformation rule has an empty name at {:?}.",
                    rule.span
                ));
            }

            if rule.pattern == rule.replacement {
                return Err(format!(
                    "Meta-transform '{}' is a no-op at {:?}.",
                    rule.name, rule.span
                ));
            }
        }

        Ok(())
    }

    /// Applies the validated transformation set to a source string.
    ///
    /// This is intentionally conservative: the engine does not perform
    /// arbitrary textual replacement because doing so could silently alter
    /// program semantics.  Compiler AST passes should consume the validated
    /// rules.
    pub fn apply_to_source(
        &self,
        source: &str,
    ) -> Result<String, String> {
        self.validate()?;

        if source.is_empty() {
            return Err("Cannot transform an empty source program.".to_string());
        }

        Ok(source.to_string())
    }

    /// Validates transformations associated with a statement sequence.
    ///
    /// The current AST does not contain the former meta-transform node types,
    /// so this method operates on the ordinary statement count only.
    pub fn apply_transforms(
        &self,
        statements: &mut Vec<crate::ast::Statement>,
    ) -> Result<(), String> {
        self.validate()?;

        println!(
            "[Toolchain::MetaProg] Validated {} transform(s) \
             against {} statement(s).",
            self.rules.len(),
            statements.len()
        );

        Ok(())
    }

    pub fn transform_count(&self) -> usize {
        self.rules.len()
    }

    pub fn dialect_count(&self) -> usize {
        self.dialects.len()
    }

    pub fn has_dialect(&self, name: &str) -> bool {
        self.dialects.contains_key(name)
    }

    pub fn rules(&self) -> &[TransformRule] {
        &self.rules
    }

    pub fn dialects(&self) -> &HashMap<String, LanguageDialect> {
        &self.dialects
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn identifier(name: &str) -> Identifier {
        Identifier(name.to_string(), Span::default())
    }

    #[test]
    fn macro_processor_expands_registered_macro() {
        let processor = MacroProcessor::new();
        let mut macros = Map::new();

        let macro_id = identifier("test_macro");

        let macro_def = MacroDefinition {
            name: macro_id.clone(),
            input_pattern: List::new(),
            generator_logic: "generated_snippet".to_string(),
            context_constraints: Map::new(),
            security_policy_ref: None,
        };

        macros.insert(macro_id.clone(), macro_def);

        let result = processor.expand_macro(
            &macro_id,
            List::new(),
            &macros,
        );

        assert_eq!(
            result.expect("registered macro should expand"),
            "generated_snippet"
        );
    }

    #[test]
    fn macro_processor_rejects_unknown_macro() {
        let processor = MacroProcessor::new();
        let macros: Map<Identifier, MacroDefinition> = Map::new();

        let result = processor.expand_macro(
            &identifier("unknown"),
            List::new(),
            &macros,
        );

        assert!(result.is_err());
    }

    #[test]
    fn transform_engine_registers_directive() {
        let mut engine = MetaTransformEngine::new();

        let directive = TransformDirective::new(
            "inline_loops",
            vec!["loop".to_string()],
            Span::default(),
        );

        assert!(engine.register_transform(&directive).is_ok());
        assert_eq!(engine.transform_count(), 1);
    }

    #[test]
    fn transform_engine_rejects_duplicate_rule() {
        let mut engine = MetaTransformEngine::new();

        let directive = TransformDirective::new(
            "inline_loops",
            vec!["loop".to_string()],
            Span::default(),
        );

        assert!(engine.register_transform(&directive).is_ok());
        assert!(engine.register_transform(&directive).is_err());
    }

    #[test]
    fn dialect_registry_rejects_duplicates() {
        let mut engine = MetaTransformEngine::new();

        let dialect = LanguageDialect::new(
            "quantum_ext",
            "1.0",
            Span::default(),
        );

        assert!(engine.register_dialect(&dialect).is_ok());
        assert_eq!(engine.dialect_count(), 1);
        assert!(engine.register_dialect(&dialect).is_err());
    }

    #[test]
    fn transform_validation_succeeds() {
        let mut engine = MetaTransformEngine::new();

        engine
            .register_transform(&TransformDirective::new(
                "constant_fold",
                vec!["expression".to_string()],
                Span::default(),
            ))
            .expect("transform should register");

        assert!(engine.validate().is_ok());
    }
}