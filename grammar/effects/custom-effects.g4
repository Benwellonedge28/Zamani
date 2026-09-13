/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/custom-effects.g4
 *
 * Status:
 *     Canonical modular production grammar for CUSTOM EFFECT EXTENSIONS.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No filesystem access.
 *     - No network access.
 *     - No runtime calls.
 *     - No hardware discovery.
 *     - No unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX for extending, composing, adapting,
 * aliasing, refining, and externally binding effects without introducing a
 * second effect declaration system.
 *
 * It exists because Zamani is an OPEN-WORLD language.
 *
 * A program may need to introduce domain-specific effects such as:
 *
 *     effect custom::telemetry;
 *     effect custom::robotics::motion;
 *     effect custom::quantum::calibration;
 *     effect custom::distributed::consensus;
 *     effect vendor::future::operation;
 *
 * However, ordinary effect declarations belong to:
 *
 *     grammar/effects/effect-declarations.g4
 *
 * Effect sets and references belong to:
 *
 *     grammar/effects/effect-sets.g4
 *
 * This file therefore MUST NOT redefine those constructs.
 *
 * Instead, this file provides syntax for relationships between already-defined
 * effects.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     custom effect aliases
 *     custom effect compositions
 *     custom effect refinements
 *     custom effect adapters
 *     custom effect wrappers
 *     external effect bindings
 *     effect extension metadata
 *     effect mapping clauses
 *     effect transformation declarations
 *     effect compatibility declarations
 *     effect parameterization at the extension boundary
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     effect declarations
 *     effect operations
 *     effect references
 *     effect sets
 *     functions
 *     expressions
 *     types
 *     capabilities
 *     resources
 *     hardware
 *     targets
 *     devices
 *     network endpoints
 *     quantum gates
 *     qubits
 *     physical qubits
 *     QEC
 *     ZQN
 *     scheduling
 *     routing
 *     optimization
 *     resilience
 *     runtime dispatch
 *     effect implementation
 *     IR
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Custom effects describe semantic relationships, not implementation targets.
 *
 * Therefore syntax such as:
 *
 *     custom effect Foo for device0;
 *
 * MUST NOT exist.
 *
 * Physical machine properties do not belong in this grammar.
 *
 * In particular, this file imposes no limits on:
 *
 *     effects
 *     effect domains
 *     effect aliases
 *     effect members
 *     composition depth
 *     generic arity
 *     parameter count
 *     implementation targets
 *     resources
 *     nodes
 *     devices
 *     qubits
 *     CPUs
 *     GPUs
 *     FPGAs
 *     memory
 *
 * Practical parser/compiler resource limits are implementation policy and
 * MUST NOT become language semantics.
 *
 * ============================================================================
 * OPEN-WORLD MODEL
 * ============================================================================
 *
 * Effect names remain ordinary qualified names.
 *
 * Examples:
 *
 *     IO
 *     Network
 *     quantum::Measurement
 *     distributed::Consensus
 *     vendor::future::Effect
 *
 * New effect domains therefore do not require modifying this grammar.
 *
 * ============================================================================
 * ARCHITECTURAL FLOW
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * ZamaniTokens
 *   |
 *   v
 * Core names / attributes
 *   |
 *   v
 * Types / Expressions
 *   |
 *   v
 * Effect declarations / effect sets
 *   |
 *   v
 * custom-effects.g4
 *   |
 *   v
 * Frontend AST
 *   |
 *   +--> name resolution
 *   +--> type analysis
 *   +--> effect analysis
 *   +--> capability analysis
 *   +--> resource analysis
 *   |
 *   v
 * Canonical semantic representation
 *   |
 *   +--> classical IR
 *   +--> quantum::ir
 *   +--> hardware / HDL representation
 *   +--> effect/resource metadata
 *   |
 *   v
 * optimization
 *   |
 *   v
 * routing / scheduling / resilience / ZQN
 *   |
 *   v
 * target lowering
 *   |
 *   v
 * runtime
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/lexer/tokens.g4
 *
 * Canonical token vocabulary:
 *
 *     ZamaniTokens
 *
 * Shared parser dependencies:
 *
 *     Core
 *     Types
 *     Expressions
 *     EffectSets
 *
 * This file MUST NOT recreate:
 *
 *     identifier
 *     qualifiedName
 *     attributes
 *     attribute
 *     typeExpression
 *     expression
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *
 * ============================================================================
 */

parser grammar CustomEffects;

options {
    tokenVocab = ZamaniTokens;
}

import Core, Types, Expressions, EffectSets;


/*
 * ============================================================================
 * 1. CUSTOM EFFECT DECLARATION
 * ============================================================================
 *
 * This is an extension declaration, NOT a second ordinary effect declaration.
 *
 * Example:
 *
 *     custom effect Logging = IO;
 *
 *     custom effect QuantumIO = IO, quantum::Measurement;
 *
 * The semantic analyzer determines whether the referenced effects exist and
 * whether the composition is valid.
 *
 * The keyword `custom` distinguishes this construct from the canonical
 * `effectDeclaration` owned by effect-declarations.g4.
 */

customEffectDeclaration
    : K_CUSTOM
      K_EFFECT
      identifier
      customEffectParameters?
      EQUALS
      customEffectDefinition
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. CUSTOM EFFECT PARAMETERS
 * ============================================================================
 *
 * Parameterization allows domain-specific extensions without hard-coding
 * machine-specific values.
 *
 * Example:
 *
 *     custom effect Telemetry<T>
 *         = observability::Emit<T>;
 *
 * No finite generic arity is imposed.
 */

customEffectParameters
    : LESS_THAN
      customEffectParameter
      (
          COMMA
          customEffectParameter
      )*
      COMMA?
      GREATER_THAN
    ;


customEffectParameter
    : identifier
      customEffectParameterType?
      customEffectParameterBound?
    ;


customEffectParameterType
    : COLON
      typeExpression
    ;


customEffectParameterBound
    : K_WHERE
      expression
    ;


/*
 * ============================================================================
 * 3. CUSTOM EFFECT DEFINITION
 * ============================================================================
 *
 * A custom effect can be:
 *
 *     a reference
 *     a composition
 *     an adapter
 *     a refinement
 *     an external binding
 *
 * The grammar remains open-world.
 */

customEffectDefinition
    : customEffectReferenceDefinition
    | customEffectCompositionDefinition
    | customEffectAdapterDefinition
    | customEffectRefinementDefinition
    | customEffectExternalDefinition
    ;


/*
 * ============================================================================
 * 4. CUSTOM EFFECT REFERENCE
 * ============================================================================
 *
 * Example:
 *
 *     custom effect Logging = IO;
 */

customEffectReferenceDefinition
    : effectReference
    ;


/*
 * ============================================================================
 * 5. CUSTOM EFFECT COMPOSITION
 * ============================================================================
 *
 * Example:
 *
 *     custom effect Service
 *         = Network, Storage, Authentication;
 *
 * Effect-set syntax remains owned by EffectSets.
 *
 * No semantic union/intersection behavior is defined here.
 */

customEffectCompositionDefinition
    : effectSet
    ;


/*
 * ============================================================================
 * 6. CUSTOM EFFECT ADAPTER
 * ============================================================================
 *
 * Adapters describe a semantic relationship between one effect and another.
 *
 * Example:
 *
 *     custom effect LegacyIO
 *         adapts IO
 *         to modern::IO;
 *
 * The adapter does NOT implement either effect.
 *
 * Runtime implementation belongs outside the grammar.
 */

customEffectAdapterDefinition
    : K_ADAPTS
      effectReference
      K_TO
      effectReference
      customEffectAdapterBody?
    ;


customEffectAdapterBody
    : LBRACE
      customEffectMapping*
      RBRACE
    ;


/*
 * ============================================================================
 * 7. EFFECT MAPPING
 * ============================================================================
 *
 * A mapping describes how named operations/effect members correspond.
 *
 * Example:
 *
 *     map read -> modern::read;
 *
 * The grammar treats both sides as names.
 *
 * It does not define operation semantics.
 */

customEffectMapping
    : K_MAP
      qualifiedName
      THIN_ARROW
      qualifiedName
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 8. CUSTOM EFFECT REFINEMENT
 * ============================================================================
 *
 * A refinement expresses that an effect has additional semantic requirements.
 *
 * Example:
 *
 *     custom effect SecureIO
 *         refines IO
 *         requires Security;
 *
 * `requires` here is an effect-level semantic relationship.
 *
 * It is NOT a hardware/resource requirement.
 */

customEffectRefinementDefinition
    : K_REFINES
      effectReference
      customEffectRefinementBody?
    ;


customEffectRefinementBody
    : LBRACE
      customEffectRefinementClause*
      RBRACE
    ;


customEffectRefinementClause
    : customEffectRequiresClause
    | customEffectProvidesClause
    | customEffectExcludesClause
    | customEffectMetadataClause
    ;


/*
 * ============================================================================
 * 9. EFFECT REQUIREMENT
 * ============================================================================
 *
 * A refinement may require other effects.
 *
 * Example:
 *
 *     requires { Security, Audit };
 *
 * These are effect references, not machine capabilities.
 */

customEffectRequiresClause
    : K_REQUIRES
      effectSet
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 10. PROVIDED EFFECTS
 * ============================================================================
 *
 * This describes effects exposed by the custom abstraction.
 *
 * Example:
 *
 *     provides { Logging, Metrics };
 */

customEffectProvidesClause
    : K_PROVIDES
      effectSet
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 11. EXCLUDED EFFECTS
 * ============================================================================
 *
 * This allows a custom effect contract to state that some effect must not
 * participate in the abstraction.
 *
 * Example:
 *
 *     excludes { UnsafeIO };
 *
 * Semantic compatibility is checked downstream.
 */

customEffectExcludesClause
    : K_EXCLUDES
      effectSet
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 12. CUSTOM EFFECT METADATA
 * ============================================================================
 *
 * Metadata is deliberately name/value based.
 *
 * It must not become a hidden machine configuration mechanism.
 *
 * Valid:
 *
 *     metadata { domain: "robotics" };
 *
 * Invalid semantic use:
 *
 *     metadata { gpu_count: 8 };
 *
 * The parser may accept metadata structurally; semantic policy determines
 * whether a particular metadata key is permitted.
 */

customEffectMetadataClause
    : K_METADATA
      LBRACE
      customEffectMetadataEntry*
      RBRACE
    ;


customEffectMetadataEntry
    : identifier
      COLON
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 13. EXTERNAL EFFECT
 * ============================================================================
 *
 * External effects provide a source-level declaration that an effect contract
 * is supplied outside the current source module.
 *
 * Example:
 *
 *     custom effect ForeignIO
 *         external "some.effect.contract";
 *
 * The string is an opaque semantic reference.
 *
 * The grammar does NOT perform filesystem/network loading.
 */

customEffectExternalDefinition
    : K_EXTERNAL
      STRING_LITERAL
    ;


/*
 * ============================================================================
 * 14. CUSTOM EFFECT EXTENSION
 * ============================================================================
 *
 * Allows an existing effect to be extended through a named custom abstraction.
 *
 * Example:
 *
 *     extend effect IO as LoggedIO {
 *         ...
 *     }
 *
 * This does not mutate the original effect declaration.
 *
 * The resulting semantic object is a distinct extension.
 */

customEffectExtensionDeclaration
    : K_EXTEND
      K_EFFECT
      effectReference
      K_AS
      identifier
      customEffectExtensionBody?
      SEMICOLON?
    ;


customEffectExtensionBody
    : LBRACE
      customEffectExtensionClause*
      RBRACE
    ;


customEffectExtensionClause
    : customEffectProvidesClause
    | customEffectRequiresClause
    | customEffectExcludesClause
    | customEffectMapping
    | customEffectMetadataClause
    ;


/*
 * ============================================================================
 * 15. CUSTOM EFFECT WRAPPER
 * ============================================================================
 *
 * A wrapper creates a new effect abstraction around an existing effect.
 *
 * Example:
 *
 *     wrap effect IO as AuditedIO;
 *
 * This is declaration-level syntax only.
 *
 * It does not define runtime interception or dispatch.
 */

customEffectWrapperDeclaration
    : K_WRAP
      K_EFFECT
      effectReference
      K_AS
      identifier
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 16. CUSTOM EFFECT COMPATIBILITY
 * ============================================================================
 *
 * A custom effect may state that it is compatible with another effect.
 *
 * Example:
 *
 *     compatible with IO;
 *
 * Compatibility is semantic and version-aware downstream.
 */

customEffectCompatibilityClause
    : K_COMPATIBLE
      K_WITH
      effectReference
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 17. CUSTOM EFFECT DECLARATION MEMBER
 * ============================================================================
 *
 * Shared composition rule for aggregate grammar consumers.
 */

customEffectDeclarationMember
    : customEffectRequiresClause
    | customEffectProvidesClause
    | customEffectExcludesClause
    | customEffectMapping
    | customEffectCompatibilityClause
    | customEffectMetadataClause
    ;


/*
 * ============================================================================
 * 18. CUSTOM EFFECT SPECIFICATION
 * ============================================================================
 *
 * Generic specification boundary for tools that need to consume a complete
 * custom-effect construct without knowing which extension form it contains.
 */

customEffectSpecification
    : customEffectDeclaration
    | customEffectExtensionDeclaration
    | customEffectWrapperDeclaration
    ;


/*
 * ============================================================================
 * 19. CUSTOM EFFECT REFERENCE EXPRESSION
 * ============================================================================
 *
 * This is intentionally a thin alias.
 *
 * It exists for downstream grammar composition and does not create a second
 * naming system.
 */

customEffectReference
    : effectReference
    ;


/*
 * ============================================================================
 * 20. CUSTOM EFFECT SET
 * ============================================================================
 *
 * Thin composition alias around the canonical effect-set grammar.
 */

customEffectSet
    : effectSet
    ;


/*
 * ============================================================================
 * 21. CUSTOM EFFECT REQUIREMENT SET
 * ============================================================================
 *
 * Explicitly named composition point for semantic tooling.
 */

customEffectRequirementSet
    : effectSet
    ;


/*
 * ============================================================================
 * 22. CUSTOM EFFECT PROVIDES SET
 * ============================================================================
 */

customEffectProvidesSet
    : effectSet
    ;


/*
 * ============================================================================
 * 23. CUSTOM EFFECT EXCLUDES SET
 * ============================================================================
 */

customEffectExcludesSet
    : effectSet
    ;


/*
 * ============================================================================
 * 24. CUSTOM EFFECT MAPPING LIST
 * ============================================================================
 *
 * No fixed number of mappings.
 */

customEffectMappingList
    : customEffectMapping+
    ;


/*
 * ============================================================================
 * 25. CUSTOM EFFECT DECLARATIONS
 * ============================================================================
 *
 * Zero or more custom effect constructs.
 *
 * This rule is useful to aggregate grammars.
 */

customEffectDeclarations
    : customEffectSpecification*
    ;


/*
 * ============================================================================
 * 26. CUSTOM EFFECT MODULE CONTENT
 * ============================================================================
 *
 * This rule intentionally contains only custom-effect syntax.
 *
 * Module/import/export ownership remains in the module grammar.
 */

customEffectModuleContent
    : customEffectSpecification*
    ;