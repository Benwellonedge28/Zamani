/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/effects/effect-diagnostics.g4
 *
 * STATUS
 * ------
 * Canonical modular production grammar for EFFECT DIAGNOSTIC CONTRACTS.
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust edition 2021
 * Safe Rust only; no unsafe code is required or permitted by the consuming
 * implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX used to attach structured diagnostic
 * expectations and diagnostic policy metadata to effect-related constructs.
 *
 * It deliberately does NOT define compiler-generated diagnostics.
 *
 * Compiler-generated diagnostics are produced by:
 *
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     diagnostics subsystem
 *
 * This grammar only defines source constructs that allow a program author,
 * library author, dialect author, or conformance test to state diagnostic
 * expectations/policies in a portable and structured way.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         ZAMANI SOURCE
 *                               |
 *                               v
 *                            LEXER
 *                               |
 *                               v
 *                            PARSER
 *                               |
 *             +-----------------+------------------+
 *             |                                    |
 *             v                                    v
 *       effect grammar                    diagnostic metadata
 *             |                                    |
 *             +-----------------+------------------+
 *                               |
 *                               v
 *                         DOMAIN-NEUTRAL AST
 *                               |
 *                               v
 *                      STRUCTURAL VALIDATION
 *                               |
 *                               v
 *                       SEMANTIC ANALYSIS
 *                               |
 *                  +------------+------------+
 *                  |            |            |
 *                  v            v            v
 *                types       effects      resources
 *                  |            |            |
 *                  +------------+------------+
 *                               |
 *                               v
 *                     DIAGNOSTIC GENERATION
 *                               |
 *                               v
 *                       CANONICAL SEMANTICS
 *                               |
 *                               v
 *                           CANONICAL IR
 *
 * Effect diagnostics therefore belong on the SOURCE/FRONTEND side of the
 * compiler boundary. They must never contain backend implementation state.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     effectDiagnosticAnnotation
 *     effectDiagnosticArguments
 *     effectDiagnosticArgument
 *     effectDiagnosticKeyValue
 *     effectDiagnosticValue
 *     effectDiagnosticSelector
 *     effectDiagnosticCode
 *     effectDiagnosticSeverity
 *     effectDiagnosticAction
 *     effectDiagnosticExpectation
 *     effectDiagnosticPolicy
 *     effectDiagnosticMessage
 *     effectDiagnosticScope
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers
 *     qualified names
 *     paths
 *     attributes
 *     expressions
 *     types
 *     effect declarations
 *     effect sets
 *     effect handlers
 *     effect polymorphism
 *     capabilities
 *     requirements
 *     constraints
 *     resources
 *     targets
 *     hardware
 *     quantum IR
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     optimization
 *     runtime diagnostics implementation
 *     source-span representation
 *     compiler error storage
 *
 * Those responsibilities remain in their existing repository owners.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Effect analysis can fail for several fundamentally different reasons:
 *
 *     - malformed effect syntax;
 *     - unresolved effect identity;
 *     - invalid effect composition;
 *     - undeclared effect;
 *     - effect escaping its declared boundary;
 *     - incompatible effect;
 *     - unhandled effect;
 *     - invalid effect parameter;
 *     - capability/effect mismatch;
 *     - resource/effect mismatch;
 *     - forbidden effect in a context;
 *     - dialect-specific effect violation.
 *
 * These are NOT all parser errors.
 *
 * The source language therefore needs a way to express structured expectations
 * without making diagnostic generation itself part of the parser.
 *
 * Example conceptual usage:
 *
 *     @diagnostic(expect = "effect.unhandled", severity = "error")
 *     fn operation() effects { Quantum } {
 *         ...
 *     }
 *
 * or:
 *
 *     @diagnostic(
 *         code = "effect.capability_missing",
 *         action = "expect"
 *     )
 *     effect Quantum;
 *
 * The exact semantic interpretation belongs downstream.
 *
 * ============================================================================
 * IMPORTANT: DIAGNOSTIC VS EFFECT SEMANTICS
 * ============================================================================
 *
 * A diagnostic declaration does NOT create an effect.
 *
 * It does NOT modify effect identity.
 *
 * It does NOT satisfy an effect requirement.
 *
 * It does NOT grant a capability.
 *
 * It does NOT allocate a resource.
 *
 * It does NOT select hardware.
 *
 * It does NOT suppress a real semantic violation merely because source syntax
 * contains an expectation.
 *
 * For example:
 *
 *     @diagnostic(expect = "effect.unhandled")
 *
 * does not make an unhandled effect legal.
 *
 * Semantic analysis still determines whether the effect is valid.
 *
 * ============================================================================
 * OPEN-WORLD DIAGNOSTIC IDENTITIES
 * ============================================================================
 *
 * Diagnostic codes are OPEN-WORLD.
 *
 * This grammar MUST NOT enumerate a finite list such as:
 *
 *     EFFECT_UNHANDLED
 *     EFFECT_UNKNOWN
 *     EFFECT_CONFLICT
 *     EFFECT_CAPABILITY_MISSING
 *
 * as grammar alternatives.
 *
 * Codes are represented as values and resolved by the diagnostics registry.
 *
 * This allows:
 *
 *     effect.unhandled
 *     effect.capability_missing
 *     quantum.effect.invalid
 *     hardware.effect.unsatisfied
 *     future.domain.effect.invalid
 *
 * without modifying this grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Diagnostic syntax MUST remain independent of physical realization.
 *
 * A diagnostic contract MUST NOT encode:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     qubit count
 *     device ID
 *     node ID
 *     physical qubit
 *     register width
 *     memory capacity
 *     topology
 *     vendor
 *     backend
 *     calibration
 *     routing result
 *     schedule
 *
 * A diagnostic code may identify a semantic problem involving a resource, but
 * the diagnostic syntax does not select or hard-code the resource.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT introduce language-wide limits such as:
 *
 *     MAX_EFFECT_DIAGNOSTICS
 *     MAX_DIAGNOSTIC_ARGUMENTS
 *     MAX_DIAGNOSTIC_CODES
 *     MAX_DIAGNOSTIC_NESTING
 *     MAX_EFFECT_ERRORS
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * Any implementation/resource limit belongs to explicit compiler resource
 * policy and MUST NOT become a source-language semantic limit.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded target-language actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment inspection;
 *     - no hardware discovery;
 *     - no runtime calls;
 *     - no random behavior.
 *
 * Parsing depends only on:
 *
 *     source text
 *     selected grammar version
 *     lexical vocabulary
 *     parser configuration
 *     explicitly enabled dialect syntax
 *
 * ============================================================================
 * SAFE RUST
 * ============================================================================
 *
 * ANTLR generated code consumed by Zamani must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * This grammar contains no Rust actions.
 *
 * The Rust implementation MUST use safe Rust and must not require `unsafe`.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser should preserve:
 *
 *     - diagnostic action;
 *     - diagnostic code;
 *     - severity;
 *     - selector;
 *     - arguments;
 *     - optional message;
 *     - optional scope;
 *     - source span;
 *     - declaration/annotation context.
 *
 * The AST must remain target-neutral.
 *
 * Diagnostic metadata must not be converted directly into:
 *
 *     compiler error objects;
 *     runtime errors;
 *     backend errors;
 *     hardware faults;
 *     quantum IR nodes.
 *
 * Semantic analysis consumes the AST representation and produces diagnostics.
 *
 * ============================================================================
 * SOURCE SPAN CONTRACT
 * ============================================================================
 *
 * The parser is responsible only for preserving parse-tree positions.
 *
 * Source spans are resolved by the frontend according to the repository's
 * canonical source/span infrastructure.
 *
 * This grammar MUST NOT invent a second span representation.
 *
 * ============================================================================
 * INTEGRATION WITH CORE ATTRIBUTES
 * ============================================================================
 *
 * Diagnostic annotations are intentionally represented through the canonical
 * attribute mechanism where possible.
 *
 * The canonical attribute grammar remains responsible for:
 *
 *     attribute
 *     attribute arguments
 *     attribute names
 *
 * This file owns the diagnostic-specific argument vocabulary used when an
 * effect diagnostic is parsed through a dedicated diagnostic rule.
 *
 * A root parser MUST choose exactly one public path for the same source form.
 *
 * It MUST NOT expose both:
 *
 *     generic attribute parsing
 *
 * and:
 *
 *     effectDiagnosticAnnotation
 *
 * for the same syntactic position if that creates ambiguity.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECT GRAMMARS
 * ============================================================================
 *
 * Existing effect grammars remain responsible for their own domains:
 *
 *     effect-declarations.g4
 *     effect-sets.g4
 *     effect-types.g4
 *     effect-operations.g4
 *     effect-composition.g4
 *     effect-handling.g4
 *     effect-polymorphism.g4
 *     capabilities.g4
 *     custom-effects.g4
 *     quantum.g4
 *     hardware.g4
 *     distributed.g4
 *     network.g4
 *     security.g4
 *     io.g4
 *
 * This file does not redefine any of those rules.
 *
 * The aggregate effects grammar should import this grammar and expose its
 * public diagnostic rules where diagnostics are legal.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECT DECLARATIONS
 * ============================================================================
 *
 * An effect declaration may be preceded by diagnostic metadata if the
 * declaration grammar permits attributes at that boundary.
 *
 * Example:
 *
 *     @diagnostic(
 *         code = "effect.deprecated",
 *         action = "expect"
 *     )
 *     effect LegacyEffect;
 *
 * Semantic analysis decides whether the expectation is satisfied.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECT OPERATIONS
 * ============================================================================
 *
 * Effect operations may carry diagnostic expectations through the canonical
 * declaration-attribute mechanism.
 *
 * Example:
 *
 *     @diagnostic(
 *         code = "effect.unhandled",
 *         action = "expect"
 *     )
 *     fn perform() -> Unit;
 *
 * This file does not own function syntax.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECT HANDLERS
 * ============================================================================
 *
 * Effect handler diagnostics may identify:
 *
 *     handler resolution failures;
 *     unhandled effects;
 *     incompatible handlers;
 *     invalid propagation;
 *     invalid discharge.
 *
 * Handler implementation remains owned by effect-handling.g4 and semantic
 * analysis.
 *
 * ============================================================================
 * INTEGRATION WITH CAPABILITIES
 * ============================================================================
 *
 * Capability/effect mismatch diagnostics may use diagnostic codes such as:
 *
 *     effect.capability_missing
 *
 * The grammar treats this as opaque data.
 *
 * Capability resolution remains owned by:
 *
 *     grammar/core/capabilities.g4
 *     grammar/effects/capabilities.g4
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCES
 * ============================================================================
 *
 * A diagnostic may report a resource/effect conflict.
 *
 * The grammar MUST NOT encode the resource itself.
 *
 * Example:
 *
 *     code = "effect.resource_unsatisfied"
 *
 * is valid.
 *
 * Example:
 *
 *     code = "effect.requires_64_qubits"
 *
 * is not forbidden lexically, but it MUST NOT be interpreted as a universal
 * language rule. Semantic validation may reject target-specific hard-coded
 * diagnostic policies where repository policy prohibits them.
 *
 * ============================================================================
 * INTEGRATION WITH QUANTUM
 * ============================================================================
 *
 * Quantum effect diagnostics remain ordinary diagnostic identities.
 *
 * Examples:
 *
 *     quantum.effect.invalid
 *     quantum.effect.unhandled
 *     quantum.effect.incompatible
 *     quantum.effect.capability_missing
 *     quantum.effect.unsatisfied
 *
 * This grammar does not enumerate quantum operations or gates.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     src/quantum/ir/
 *
 * The downstream architecture remains:
 *
 *     Zamani source
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic effect analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target
 *
 * ============================================================================
 * INTEGRATION WITH HDL
 * ============================================================================
 *
 * HDL effect diagnostics may report semantic violations involving:
 *
 *     signals
 *     timing
 *     clocks
 *     interfaces
 *     synthesis constraints
 *     simulation semantics
 *     verification
 *
 * The grammar does not encode bus widths, device counts, FPGA capacities, or
 * ASIC topology as universal language limits.
 *
 * ============================================================================
 * INTEGRATION WITH CLASSICAL COMPUTING
 * ============================================================================
 *
 * Classical effect diagnostics remain target-neutral.
 *
 * For example:
 *
 *     effect.state
 *     effect.mutation
 *     effect.io
 *
 * may be diagnosed without selecting a CPU, register file, cache, or operating
 * system.
 *
 * ============================================================================
 * INTEGRATION WITH DISTRIBUTED COMPUTING
 * ============================================================================
 *
 * Diagnostic codes may identify:
 *
 *     distributed.effect.unhandled
 *     distributed.effect.incompatible
 *     distributed.effect.unsatisfied
 *
 * They must not encode a universal node count or topology.
 *
 * ============================================================================
 * INTEGRATION WITH AI / DATA / NETWORKING / SECURITY
 * ============================================================================
 *
 * All future and existing domains use the same open diagnostic identity model.
 *
 * Domain grammars may define semantic diagnostic code namespaces without
 * modifying this grammar.
 *
 * ============================================================================
 * DIAGNOSTIC ACTION MODEL
 * ============================================================================
 *
 * The action describes what the source-level diagnostic contract means.
 *
 * The stable conceptual actions are:
 *
 *     expect
 *     allow
 *     deny
 *     suppress
 *     note
 *     warn
 *
 * These are represented as identifiers rather than a closed token enumeration.
 *
 * Semantic validation determines which actions are legal in a particular
 * context.
 *
 * This preserves extensibility for future diagnostic systems.
 *
 * ============================================================================
 * IMPORTANT SUPPRESSION RULE
 * ============================================================================
 *
 * A source-level diagnostic policy MUST NOT silently erase a correctness or
 * safety violation.
 *
 * For example:
 *
 *     suppress effect.unhandled
 *
 * may suppress presentation of a diagnostic only if the semantic/diagnostic
 * policy explicitly permits suppression.
 *
 * It MUST NOT make an invalid program valid.
 *
 * This distinction is enforced downstream.
 *
 * ============================================================================
 * DIAGNOSTIC SEVERITY
 * ============================================================================
 *
 * Severity is represented as an open identifier.
 *
 * Common values include:
 *
 *     error
 *     warning
 *     note
 *     help
 *     info
 *
 * Future diagnostic systems may introduce additional severities.
 *
 * The parser therefore does not hard-code the vocabulary.
 *
 * ============================================================================
 * DIAGNOSTIC CODE
 * ============================================================================
 *
 * A diagnostic code is represented as a structured qualified name.
 *
 * Examples:
 *
 *     effect.unhandled
 *     effect.unknown
 *     effect.conflict
 *     quantum.effect.unhandled
 *     hardware.effect.unsatisfied
 *     future.domain.effect.invalid
 *
 * The grammar imposes no finite depth or domain count.
 *
 * ============================================================================
 * DIAGNOSTIC MESSAGE
 * ============================================================================
 *
 * Messages are optional source-level explanatory text.
 *
 * The message is metadata.
 *
 * It is not a replacement for the canonical compiler diagnostic generated by
 * semantic analysis.
 *
 * ============================================================================
 * DIAGNOSTIC SELECTOR
 * ============================================================================
 *
 * A selector identifies which diagnostic/effect condition the contract applies
 * to.
 *
 * It is intentionally represented as a qualified name.
 *
 * It must not contain:
 *
 *     device IDs
 *     physical resource IDs
 *     runtime addresses
 *     hardware topology
 *
 * ============================================================================
 * DIAGNOSTIC ARGUMENTS
 * ============================================================================
 *
 * Arguments use a key/value form:
 *
 *     code = "effect.unhandled"
 *     severity = "error"
 *     action = "expect"
 *     message = "..."
 *
 * Values are deliberately restricted to source-level scalar/qualified values
 * and expressions supported by the canonical expression grammar.
 *
 * The semantic layer decides which values are legal for a given diagnostic.
 *
 * ============================================================================
 * NO EMBEDDED SEMANTICS
 * ============================================================================
 *
 * This file must never use:
 *
 *     @members
 *     @parser::members
 *     @lexer::members
 *     semantic predicates
 *     target-language actions
 *     custom exception handlers
 *
 * for effect diagnostic semantics.
 *
 * Semantic behavior belongs in Rust semantic-analysis/diagnostics code.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * The intended public integration points are:
 *
 *     effectDiagnosticAnnotation
 *     effectDiagnosticExpectation
 *     effectDiagnosticPolicy
 *     effectDiagnosticCode
 *     effectDiagnosticSelector
 *
 * All subordinate rules are implementation details of this grammar module.
 *
 * ============================================================================
 */

parser grammar EffectDiagnostics;

options {
    /*
     * Canonical repository lexer.
     *
     * This intentionally follows the root architecture rather than introducing
     * a second token vocabulary.
     */
    tokenVocab = ZamaniLexer;
}

import Core, Expressions;


/*
 * ============================================================================
 * 1. EFFECT DIAGNOSTIC ANNOTATION
 * ============================================================================
 *
 * Canonical source form:
 *
 *     @diagnostic(...)
 *
 * The annotation itself is deliberately lightweight.
 *
 * Semantic meaning is resolved downstream.
 */
effectDiagnosticAnnotation
    : AT
      effectDiagnosticSelector
      effectDiagnosticArguments?
    ;


/*
 * ============================================================================
 * 2. DIAGNOSTIC ARGUMENTS
 * ============================================================================
 *
 * Parentheses are optional at the annotation level so that:
 *
 *     @diagnostic
 *
 * remains structurally distinguishable from:
 *
 *     @diagnostic(...)
 *
 * Semantic validation decides whether an argument-free diagnostic annotation
 * is meaningful.
 */
effectDiagnosticArguments
    : LPAREN
      effectDiagnosticArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 3. ARGUMENT LIST
 * ============================================================================
 *
 * There is no fixed argument count.
 *
 * A trailing comma is accepted consistently with other Zamani collection
 * grammars where the repository permits trailing commas.
 */
effectDiagnosticArgumentList
    : effectDiagnosticArgument
      (COMMA effectDiagnosticArgument)*
      COMMA?
    ;


/*
 * ============================================================================
 * 4. ARGUMENT
 * ============================================================================
 *
 * Diagnostic arguments are key/value metadata.
 *
 * The key is an identifier.
 *
 * The value may be:
 *
 *     - a diagnostic code;
 *     - severity;
 *     - action;
 *     - selector;
 *     - message;
 *     - scope;
 *     - a generic source expression.
 *
 * The semantic layer validates whether a particular key/value pair is legal.
 */
effectDiagnosticArgument
    : effectDiagnosticKeyValue
    ;


/*
 * ============================================================================
 * 5. KEY/VALUE
 * ============================================================================
 */
effectDiagnosticKeyValue
    : identifier
      EQUALS
      effectDiagnosticValue
    ;


/*
 * ============================================================================
 * 6. VALUE
 * ============================================================================
 *
 * Values are intentionally delegated to the canonical expression grammar.
 *
 * This prevents this file from inventing:
 *
 *     integer literals
 *     string literals
 *     boolean literals
 *     arrays
 *     maps
 *     qualified values
 *
 * a second time.
 *
 * Expressions remain source-level data only.
 */
effectDiagnosticValue
    : expression
    ;


/*
 * ============================================================================
 * 7. DIAGNOSTIC SELECTOR
 * ============================================================================
 *
 * Selectors use the repository's canonical qualified-name rule.
 *
 * Examples:
 *
 *     diagnostic
 *     effect
 *     effect.unhandled
 *     quantum.effect.unhandled
 *
 * If the canonical qualified-name grammar uses a different public rule in the
 * current parser composition, this rule MUST be wired to that existing rule
 * rather than introducing another name grammar.
 *
 * `identifier` is used here as the stable minimum dependency. Semantic
 * analysis may subsequently resolve richer diagnostic identity.
 */
effectDiagnosticSelector
    : identifier
    ;


/*
 * ============================================================================
 * 8. DIAGNOSTIC CODE
 * ============================================================================
 *
 * A diagnostic code is source-level metadata.
 *
 * The code is intentionally open-world.
 *
 * This rule does not enumerate diagnostic codes.
 */
effectDiagnosticCode
    : identifier
    ;


/*
 * ============================================================================
 * 9. DIAGNOSTIC SEVERITY
 * ============================================================================
 *
 * Severity is represented by an identifier.
 *
 * Examples:
 *
 *     error
 *     warning
 *     note
 *     info
 *     help
 *
 * The diagnostics subsystem owns the semantic registry.
 */
effectDiagnosticSeverity
    : identifier
    ;


/*
 * ============================================================================
 * 10. DIAGNOSTIC ACTION
 * ============================================================================
 *
 * Examples:
 *
 *     expect
 *     allow
 *     deny
 *     suppress
 *     note
 *     warn
 *
 * These are deliberately open-world identifiers.
 */
effectDiagnosticAction
    : identifier
    ;


/*
 * ============================================================================
 * 11. DIAGNOSTIC EXPECTATION
 * ============================================================================
 *
 * This rule provides a semantic-tooling entry point for consumers that need
 * to identify an expectation without reimplementing the complete annotation
 * syntax.
 *
 * Canonical conceptual form:
 *
 *     @diagnostic(
 *         code = "effect.unhandled",
 *         action = "expect"
 *     )
 *
 * The rule intentionally delegates all interpretation to semantic analysis.
 */
effectDiagnosticExpectation
    : effectDiagnosticAnnotation
    ;


/*
 * ============================================================================
 * 12. DIAGNOSTIC POLICY
 * ============================================================================
 *
 * A policy is structurally the same annotation form.
 *
 * The semantic layer distinguishes:
 *
 *     expectation
 *     suppression
 *     allowance
 *     severity preference
 *     reporting policy
 *
 * from the source structure.
 */
effectDiagnosticPolicy
    : effectDiagnosticAnnotation
    ;


/*
 * ============================================================================
 * 13. EFFECT-SPECIFIC DIAGNOSTIC CODE CONVENIENCE
 * ============================================================================
 *
 * This rule exists for consumers that want an explicitly named effect
 * diagnostic code while preserving the open-world identity model.
 *
 * It is NOT a closed enumeration.
 */
effectDiagnosticEffectCode
    : effectDiagnosticCode
    ;


/*
 * ============================================================================
 * 14. DIAGNOSTIC MESSAGE
 * ============================================================================
 *
 * A diagnostic message is represented by a normal source expression.
 *
 * This allows string literals and future compile-time message forms to use
 * the canonical expression grammar.
 *
 * Semantic validation determines whether a message expression is legal and
 * whether it is required to be compile-time evaluable.
 */
effectDiagnosticMessage
    : expression
    ;


/*
 * ============================================================================
 * 15. DIAGNOSTIC SCOPE
 * ============================================================================
 *
 * Scope is source-level metadata.
 *
 * Examples may include:
 *
 *     declaration
 *     operation
 *     expression
 *     module
 *     block
 *
 * The semantic layer determines whether the selected scope is valid at the
 * location where the annotation appears.
 */
effectDiagnosticScope
    : identifier
    ;


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The intended semantic extraction is conceptually:
 *
 *     effectDiagnosticAnnotation
 *              |
 *              +--> selector
 *              |
 *              +--> arguments
 *                       |
 *                       +--> code
 *                       +--> severity
 *                       +--> action
 *                       +--> message
 *                       +--> scope
 *                       +--> extension arguments
 *
 * The parser preserves the structure.
 *
 * Semantic analysis validates:
 *
 *     - known/unknown diagnostic code;
 *     - supported action;
 *     - supported severity;
 *     - valid scope;
 *     - argument uniqueness;
 *     - argument compatibility;
 *     - effect-context applicability;
 *     - dialect applicability;
 *     - feature/version availability.
 *
 * ============================================================================
 * DUPLICATE-KEY POLICY
 * ============================================================================
 *
 * This grammar intentionally permits duplicate keys structurally:
 *
 *     @diagnostic(code = "a", code = "b")
 *
 * because rejecting duplicates requires semantic knowledge of the argument
 * schema.
 *
 * Semantic analysis MUST diagnose duplicate singleton keys.
 *
 * It MUST NOT silently select one value.
 *
 * If an extension explicitly defines a repeatable key, repetition may be
 * permitted according to that extension's semantic contract.
 *
 * ============================================================================
 * UNKNOWN-KEY POLICY
 * ============================================================================
 *
 * Unknown diagnostic keys remain syntactically valid.
 *
 * Semantic analysis decides whether:
 *
 *     - the key is standard;
 *     - the key is provided by a dialect;
 *     - the key is experimental;
 *     - the key is deprecated;
 *     - the key is invalid.
 *
 * This preserves forward compatibility without allowing unknown semantics to
 * silently alter program meaning.
 *
 * ============================================================================
 * DUPLICATE SEMANTIC AUTHORITIES
 * ============================================================================
 *
 * This file MUST NOT become a second diagnostic specification.
 *
 * Diagnostic code definitions belong to the diagnostics/validation/semantic
 * infrastructure.
 *
 * This grammar only establishes source syntax.
 *
 * ============================================================================
 * QUANTUM DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Quantum-specific diagnostic meaning belongs downstream.
 *
 * For example:
 *
 *     quantum.effect.unhandled
 *
 * may be resolved by quantum/effect semantic analysis.
 *
 * This file does not:
 *
 *     - inspect quantum operations;
 *     - enumerate gates;
 *     - identify qubits;
 *     - inspect physical topology;
 *     - access QPU state;
 *     - invoke QEC;
 *     - invoke ZQN;
 *     - perform routing;
 *     - perform scheduling.
 *
 * ============================================================================
 * HARDWARE DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * A diagnostic may report:
 *
 *     hardware.effect.unsatisfied
 *
 * but the grammar does not determine:
 *
 *     which hardware;
 *     which device;
 *     which accelerator;
 *     which topology;
 *     which physical resource.
 *
 * Hardware capability matching belongs downstream.
 *
 * ============================================================================
 * RESOURCE DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * A diagnostic may report that a resource requirement cannot be satisfied.
 *
 * It must not turn that report into a universal language limit.
 *
 * Example:
 *
 *     effect.resource_unsatisfied
 *
 * is a semantic diagnostic identity.
 *
 * It does not mean that Zamani has a fixed resource maximum.
 *
 * ============================================================================
 * POCO-REAF INVARIANT
 * ============================================================================
 *
 * Diagnostic metadata must survive target changes without requiring source
 * rewriting merely because:
 *
 *     CPU -> GPU
 *     GPU -> FPGA
 *     FPGA -> ASIC
 *     CPU -> QPU
 *     QPU -> simulator
 *     single machine -> cluster
 *     cluster -> cloud
 *     current hardware -> future hardware
 *
 * A source-level diagnostic contract concerns semantic behavior, not physical
 * realization.
 *
 * ============================================================================
 * EXTENSION / DIALECT INTEGRATION
 * ============================================================================
 *
 * Dialects may introduce additional diagnostic code namespaces and additional
 * semantic keys.
 *
 * They MUST NOT:
 *
 *     - redefine the meaning of standard keys;
 *     - change the lexical meaning of identifiers;
 *     - create a second diagnostic syntax for the same context;
 *     - bypass semantic validation;
 *     - introduce hardware limits into this grammar.
 *
 * Dialect compatibility belongs to:
 *
 *     grammar/dialects/
 *     grammar/compatibility/
 *
 * ============================================================================
 * VERSIONING
 * ============================================================================
 *
 * Diagnostic syntax evolves under the normal Zamani language compatibility
 * system.
 *
 * A semantic change to an existing diagnostic action/key must be versioned.
 *
 * Removing a diagnostic key requires:
 *
 *     deprecation
 *     compatibility policy
 *     migration guidance
 *
 * where the language version policy requires it.
 *
 * ============================================================================
 * ERROR RECOVERY
 * ============================================================================
 *
 * Error recovery is owned by the ANTLR/Rust parser integration.
 *
 * This grammar must not embed custom recovery actions.
 *
 * Malformed constructs such as:
 *
 *     @diagnostic(
 *     @diagnostic(code)
 *     @diagnostic(= "x")
 *     @diagnostic(code = )
 *
 * must be reported through the normal parser diagnostic pipeline.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Diagnostic metadata is untrusted source input.
 *
 * The parser must not:
 *
 *     - execute diagnostic messages;
 *     - access files;
 *     - access networks;
 *     - invoke commands;
 *     - access secrets;
 *     - inspect hardware.
 *
 * Message expressions are source syntax only.
 *
 * Any compile-time evaluation is a separate semantic/compiler concern and must
 * execute under the compiler's established safety rules.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar has no language-level limits on:
 *
 *     diagnostic annotations
 *     diagnostic arguments
 *     diagnostic codes
 *     namespaces
 *     nested source constructs
 *     domains
 *     effects
 *
 * Repetition is expressed using grammar repetition operators.
 *
 * Compiler resource exhaustion must be handled through explicit resource
 * policies and diagnostics rather than grammar-level semantic limits.
 *
 * ============================================================================
 * DETERMINISTIC NORMALIZATION
 * ============================================================================
 *
 * Semantic analysis should normalize diagnostic argument keys according to
 * the language's canonical identifier rules.
 *
 * Where diagnostic arguments are semantically unordered, normalization MUST
 * NOT depend on source hash-map ordering.
 *
 * Source order MUST nevertheless remain available through source spans for
 * diagnostics and tooling.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The AST/tooling pipeline should preserve:
 *
 *     source span
 *     original diagnostic code spelling
 *     original selector spelling
 *     original argument ordering
 *     original message expression
 *
 * Semantic normalization must not destroy source provenance.
 *
 * ============================================================================
 * TESTING CONTRACT
 * ============================================================================
 *
 * Production conformance must include:
 *
 * POSITIVE
 * --------
 *
 *     @diagnostic
 *     @diagnostic(code = "effect.unhandled")
 *     @diagnostic(code = "effect.unhandled", action = "expect")
 *     @diagnostic(code = "quantum.effect.unhandled", severity = "error")
 *     @diagnostic(message = "expected effect diagnostic")
 *     @diagnostic(scope = operation)
 *
 * NEGATIVE
 * --------
 *
 *     @diagnostic(
 *     @diagnostic(
 *         =
 *     @diagnostic(
 *         code =
 *     @diagnostic(
 *         ,
 *     )
 *
 * BOUNDARY
 * --------
 *
 *     deeply qualified diagnostic identities
 *     large argument sets
 *     large diagnostic messages
 *     repeated annotations
 *     nested effect declarations
 *     nested effect handlers
 *
 * SCALABILITY
 * ----------
 *
 *     arbitrarily many diagnostic annotations permitted by implementation
 *     resources
 *
 *     arbitrarily many diagnostic arguments permitted by implementation
 *     resources
 *
 *     arbitrarily many domains/effect namespaces
 *
 * DETERMINISM
 * -----------
 *
 *     identical source + grammar version => identical parse structure
 *
 * COMPATIBILITY
 * -------------
 *
 *     legacy diagnostic attributes
 *     dialect diagnostic attributes
 *     deprecated diagnostic keys
 *     version-gated diagnostic contracts
 *
 * HARD-CODING
 * -----------
 *
 *     no physical device identifiers
 *     no machine-count limits
 *     no universal resource capacities
 *     no fixed quantum limits
 *
 * ============================================================================
 * VALIDATION CONTRACT
 * ============================================================================
 *
 * grammar/validation/ must verify:
 *
 *     - grammar compiles;
 *     - imports resolve;
 *     - public rule names are unique;
 *     - no duplicate token definitions are introduced;
 *     - no embedded actions exist;
 *     - no semantic predicates exist;
 *     - no left-recursive diagnostic rules exist;
 *     - malformed annotations fail deterministically;
 *     - valid annotations parse deterministically;
 *     - diagnostic syntax does not introduce hardware limits;
 *     - AST mapping exists;
 *     - semantic mapping exists;
 *     - source spans remain recoverable.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] Existing repository naming conventions are preserved.
 * [x] The missing effect-diagnostics.g4 module is independently defined.
 * [x] The grammar is parser-only.
 * [x] Canonical ZamaniLexer vocabulary is consumed.
 * [x] No second lexer vocabulary is introduced.
 * [x] Core identifier syntax is reused.
 * [x] Expression syntax is reused.
 * [x] Effect declarations are not redefined.
 * [x] Effect sets are not redefined.
 * [x] Effect handlers are not redefined.
 * [x] Capabilities are not redefined.
 * [x] Requirements are not redefined.
 * [x] Resources are not redefined.
 * [x] Hardware is not redefined.
 * [x] Quantum IR is not redefined.
 * [x] Diagnostic codes are open-world.
 * [x] Diagnostic severities are open-world.
 * [x] Diagnostic actions are open-world.
 * [x] Diagnostic selectors are source-level identities.
 * [x] Diagnostic messages remain source-level expressions.
 * [x] No physical machine limit is encoded.
 * [x] No fixed quantum limit is encoded.
 * [x] No device ID is encoded.
 * [x] No topology is encoded.
 * [x] No runtime behavior is embedded.
 * [x] No filesystem/network access is embedded.
 * [x] No unsafe Rust is required.
 * [x] Rust 1.97 / 1.97.1 compatibility is preserved.
 * [x] Deterministic parsing is preserved.
 * [x] Forward-compatible diagnostic namespaces are supported.
 * [x] Dialect extension remains possible.
 * [x] Semantic validation remains downstream.
 * [x] POCO-REAF remains intact.
 *
 * ============================================================================
 * INTEGRATION SUMMARY
 * ============================================================================
 *
 * This file should be integrated as follows:
 *
 *     grammar/effects/effect-diagnostics.g4
 *             |
 *             +--> imported by the effects parser composition layer
 *             |
 *             +--> exposed at contexts where diagnostic annotations are legal
 *             |
 *             v
 *        frontend AST
 *             |
 *             v
 *        semantic diagnostics
 *             |
 *             +--> effect analysis
 *             +--> capability analysis
 *             +--> resource analysis
 *             +--> domain analysis
 *             |
 *             v
 *        canonical semantic model
 *             |
 *             v
 *        compiler / tooling / runtime diagnostics
 *
 * There is deliberately no reverse dependency:
 *
 *     runtime -> grammar
 *     backend -> grammar
 *     quantum::ir -> grammar
 *     QEC -> grammar
 *     ZQN -> grammar
 *     HAL -> grammar
 *
 * ============================================================================
 */