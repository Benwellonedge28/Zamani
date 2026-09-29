/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 *     grammar/metaprogramming/capabilities.g4
 *
 * GRAMMAR
 *     MetaprogrammingCapabilities
 *
 * STATUS
 *     Production-oriented parser grammar component
 *
 * LANGUAGE
 *     Zamani
 *
 * COMPILER BASELINE
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *
 * SAFETY
 *     Safe Rust only.
 *     No unsafe Rust.
 *     No embedded Rust actions.
 *     No executable grammar predicates.
 *     No host-language execution.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX for declaring, requiring,
 * restricting, composing, and inspecting capabilities used by Zamani
 * metaprogramming.
 *
 * A metaprogramming capability describes an authorized semantic operation
 * that a compile-time transformation may request.
 *
 * Examples include:
 *
 *     capability("syntax.inspect")
 *     capability("syntax.generate")
 *     capability("type.inspect")
 *     capability("metadata.read")
 *     capability("compile.evaluate")
 *     capability("macro.expand")
 *     capability("quantum.operation.inspect")
 *     capability("hardware.capability.inspect")
 *
 * These names are OPEN-ENDED semantic identifiers.
 *
 * This file does not enumerate every capability in the language.
 *
 * New capabilities must not require new grammar alternatives merely because
 * a new computing domain, backend, library, or compiler facility is added.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> capability resolution
 *          +--> authorization
 *          +--> effect analysis
 *          +--> resource analysis
 *          +--> type checking
 *          +--> phase validation
 *          |
 *          v
 *     metaprogramming semantic model
 *          |
 *          +--> compile-time evaluation
 *          +--> reflection
 *          +--> quotation / unquotation
 *          +--> generation
 *          +--> specialization
 *          +--> macro integration
 *          |
 *          v
 *     validated canonical Zamani AST
 *          |
 *          v
 *     ordinary semantic pipeline
 *          |
 *          v
 *     canonical IR
 *
 * The grammar recognizes structure only.
 *
 * Parsing a capability declaration MUST NOT grant the declared capability.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - metaprogrammingCapabilityDeclaration
 *     - metaprogrammingCapabilityRequirement
 *     - metaprogrammingCapabilityRestriction
 *     - metaprogrammingCapabilityExpression
 *     - metaprogrammingCapabilityReference
 *     - metaprogrammingCapabilityPath
 *     - metaprogrammingCapabilityScope
 *     - metaprogrammingCapabilityCondition
 *     - metaprogrammingCapabilityMetadata
 *     - metaprogrammingCapabilityCore
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified-name syntax;
 *     - ordinary expressions;
 *     - ordinary types;
 *     - annotations;
 *     - declarations;
 *     - statements;
 *     - effects;
 *     - resource requirements;
 *     - hardware capabilities;
 *     - security policies;
 *     - authorization implementation;
 *     - compile-time evaluation;
 *     - macro expansion;
 *     - reflection implementation;
 *     - AST construction implementation;
 *     - semantic capability registries;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - runtime execution;
 *     - hardware discovery;
 *     - target selection;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL.
 *
 * ============================================================================
 * AUTHORITY AND INTEGRATION
 * ============================================================================
 *
 * Related authoritative components:
 *
 *     grammar/metaprogramming/README.md
 *     grammar/metaprogramming/metaprogramming.g4
 *     grammar/metaprogramming/compile-time.g4
 *     grammar/metaprogramming/compile-time-execution.g4
 *     grammar/metaprogramming/reflection.g4
 *     grammar/metaprogramming/generation.g4
 *     grammar/metaprogramming/specialization.g4
 *     grammar/metaprogramming/quotation.g4
 *     grammar/metaprogramming/unquotation.g4
 *
 *     grammar/macros/safety.g4
 *     grammar/macros/expansion.g4
 *
 *     grammar/resources/capabilities.g4
 *     grammar/resources/requirements.g4
 *     grammar/effects/
 *     grammar/security/
 *
 *     grammar/core/names.g4
 *     grammar/core/attributes.g4
 *     grammar/expressions/
 *
 *     grammar/Zamani.g4
 *     grammar/DESIGN.md
 *
 * The resource capability grammar owns general computational and hardware
 * capability requirements.
 *
 * This file owns metaprogramming-specific capability usage and boundaries.
 *
 * Neither component may silently redefine the other's semantic model.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This is a parser grammar component.
 *
 * It MUST NOT define lexer rules.
 *
 * Required token vocabulary:
 *
 *     IDENTIFIER
 *
 * Required punctuation:
 *
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     SEMICOLON
 *     DOT
 *     COLON
 *     COLON_COLON
 *
 * Required operators:
 *
 *     ASSIGN
 *     NOT
 *     AND
 *     OR
 *
 * Required keywords:
 *
 *     CAPABILITY
 *     REQUIRES
 *     FORBIDS
 *     ALLOWS
 *     WITH
 *     WHEN
 *     IN
 *     TRUE
 *     FALSE
 *
 * These symbolic names are integration contracts.
 *
 * They MUST be mapped to the existing canonical token vocabulary rather
 * than duplicated in this file.
 *
 * Where an existing canonical token uses another name, the composition
 * grammar or token registry must provide the mapping.
 *
 * This file MUST NOT create another lexer or keyword registry.
 *
 * ============================================================================
 * OPEN-WORLD CAPABILITY MODEL
 * ============================================================================
 *
 * Capability names are paths.
 *
 * Examples:
 *
 *     syntax.inspect
 *     syntax.generate
 *     type.inspect
 *     metadata.read
 *     compile.evaluate
 *     quantum.operation.inspect
 *     hardware.capability.inspect
 *     compiler.extension.custom_transform
 *
 * A capability path is not a hardware identifier.
 *
 * It does not identify a physical device, processor, accelerator, QPU,
 * memory bank, register, or vendor implementation.
 *
 * Capability identity is resolved semantically.
 *
 * ============================================================================
 * CAPABILITY DECLARATIONS
 * ============================================================================
 *
 * A declaration describes the capabilities associated with a metaprogram.
 *
 * Conceptual examples:
 *
 *     capability syntax_transform {
 *         allows syntax.inspect;
 *         allows syntax.generate;
 *     }
 *
 *     capability quantum_inspection {
 *         allows quantum.operation.inspect;
 *         allows quantum.type.inspect;
 *     }
 *
 * The grammar does not determine whether these capabilities are available.
 *
 * Semantic analysis verifies:
 *
 *     - declaration validity;
 *     - scope;
 *     - ownership;
 *     - capability registration;
 *     - dependency resolution;
 *     - authorization;
 *     - phase restrictions;
 *     - compatibility.
 *
 * ============================================================================
 * REQUIREMENTS AND RESTRICTIONS
 * ============================================================================
 *
 * A requirement expresses that an operation needs a capability.
 *
 * A restriction expresses that an operation must not request a capability.
 *
 * Neither is an authorization grant.
 *
 * Conceptual examples:
 *
 *     requires capability("syntax.inspect");
 *
 *     forbids capability("filesystem.write");
 *
 *     requires capability("type.inspect")
 *         and capability("metadata.read");
 *
 *     requires capability("syntax.generate")
 *         or capability("syntax.transform");
 *
 * Semantic policy decides whether a requirement can be satisfied.
 *
 * ============================================================================
 * SECURITY INVARIANT
 * ============================================================================
 *
 * A capability declaration MUST NOT grant authority.
 *
 * In particular, the following must never happen:
 *
 *     source annotation
 *          |
 *          v
 *     automatic privilege escalation
 *
 * The authorization pipeline must independently establish:
 *
 *     requested capability
 *     declared capability
 *     permitted capability
 *     effective capability
 *     execution phase
 *     security policy
 *     provenance
 *
 * A program cannot authorize itself merely by writing:
 *
 *     allows filesystem.write;
 *
 * or:
 *
 *     allows hardware.access;
 *
 * or:
 *
 *     allows compiler.mutate;
 *
 * These remain semantic requests subject to independent policy.
 *
 * ============================================================================
 * SCOPING
 * ============================================================================
 *
 * Capability scope describes where a declaration or requirement applies.
 *
 * Possible semantic scopes include:
 *
 *     module
 *     declaration
 *     function
 *     macro
 *     compile-time block
 *     transformation
 *     generated construct
 *
 * These scope categories are intentionally not represented as a fixed
 * enumeration of reserved words.
 *
 * The grammar uses an open-ended scope path.
 *
 * Semantic analysis determines which scopes are valid for each context.
 *
 * ============================================================================
 * PHASE SEPARATION
 * ============================================================================
 *
 * Capabilities are evaluated relative to an execution phase.
 *
 * Relevant phases may include:
 *
 *     source analysis
 *     parsing
 *     semantic analysis
 *     compile-time evaluation
 *     transformation
 *     generation
 *     specialization
 *     runtime
 *
 * This grammar does not define the compiler phase machine.
 *
 * It only permits phase-related capability metadata to be expressed.
 *
 * The phase model belongs to the metaprogramming semantic specification.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Capability syntax must remain independent of physical target size.
 *
 * It MUST NOT impose:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_CAPABILITIES
 *     MAX_CAPABILITY_DEPTH
 *
 * or equivalent universal limits.
 *
 * Capability expressions may be finite source expressions, but their
 * representational capacity must not be restricted by an artificial
 * language-level maximum.
 *
 * Implementation resource exhaustion must be reported separately from
 * semantic invalidity.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Capability parsing must be deterministic.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware discovery;
 *     current machine;
 *     current time;
 *     network availability;
 *     environment variables;
 *     filesystem contents;
 *     runtime state;
 *     random selection;
 *     backend availability.
 *
 * The same source and canonical token stream must produce equivalent
 * syntactic structure.
 *
 * Capability resolution occurs downstream.
 *
 * ============================================================================
 * GRAMMAR RULES
 * ============================================================================
 *
 * The rules below are deliberately structural.
 *
 * They do not duplicate general expressions, declarations, annotations,
 * names, or types.
 *
 * ============================================================================
 */


/*
 * --------------------------------------------------------------------------
 * Capability integration entry point
 * --------------------------------------------------------------------------
 *
 * This rule is the stable integration boundary for this component.
 *
 * The composition grammar may consume this rule from metaprogramming
 * declarations, statements, expressions, and compile-time blocks.
 */
metaprogrammingCapabilityCore
    : metaprogrammingCapabilityDeclaration
    | metaprogrammingCapabilityRequirement
    | metaprogrammingCapabilityRestriction
    ;


/*
 * --------------------------------------------------------------------------
 * Capability declaration
 * --------------------------------------------------------------------------
 *
 * Example:
 *
 *     capability syntax_transform {
 *         allows syntax.inspect;
 *         allows syntax.generate;
 *     }
 *
 * The identifier is a declaration name, not an authority grant.
 */
metaprogrammingCapabilityDeclaration
    : CAPABILITY IDENTIFIER
      LBRACE
          metaprogrammingCapabilityMember*
      RBRACE
    ;


/*
 * --------------------------------------------------------------------------
 * Declaration members
 * --------------------------------------------------------------------------
 *
 * Capability members are open-ended.
 *
 * Future capability categories do not require new parser alternatives.
 */
metaprogrammingCapabilityMember
    : metaprogrammingCapabilityAllowance
    | metaprogrammingCapabilityRequirement
    | metaprogrammingCapabilityRestriction
    ;


/*
 * --------------------------------------------------------------------------
 * Allowance declaration
 * --------------------------------------------------------------------------
 *
 * "allows" records a declared capability relationship.
 *
 * It does not grant permission.
 */
metaprogrammingCapabilityAllowance
    : ALLOWS metaprogrammingCapabilityPath SEMICOLON
    ;


/*
 * --------------------------------------------------------------------------
 * Capability requirement
 * --------------------------------------------------------------------------
 *
 * Examples:
 *
 *     requires capability("syntax.inspect");
 *
 *     requires capability("syntax.generate")
 *         and capability("type.inspect");
 *
 *     requires capability("syntax.transform")
 *         or capability("syntax.generate");
 *
 * The actual authorization decision is semantic.
 */
metaprogrammingCapabilityRequirement
    : REQUIRES metaprogrammingCapabilityExpression SEMICOLON
    ;


/*
 * --------------------------------------------------------------------------
 * Capability restriction
 * --------------------------------------------------------------------------
 *
 * Examples:
 *
 *     forbids capability("filesystem.write");
 *
 *     forbids capability("network.access");
 *
 * Restrictions are declarative constraints.
 */
metaprogrammingCapabilityRestriction
    : FORBIDS metaprogrammingCapabilityExpression SEMICOLON
    ;


/*
 * --------------------------------------------------------------------------
 * Capability expression
 * --------------------------------------------------------------------------
 *
 * Boolean composition is structural.
 *
 * Operator precedence:
 *
 *     NOT
 *     AND
 *     OR
 *
 * The semantic layer must validate the meaning of the resulting expression.
 */
metaprogrammingCapabilityExpression
    : metaprogrammingCapabilityOrExpression
    ;


metaprogrammingCapabilityOrExpression
    : metaprogrammingCapabilityAndExpression
      (OR metaprogrammingCapabilityAndExpression)*
    ;


metaprogrammingCapabilityAndExpression
    : metaprogrammingCapabilityUnaryExpression
      (AND metaprogrammingCapabilityUnaryExpression)*
    ;


metaprogrammingCapabilityUnaryExpression
    : NOT metaprogrammingCapabilityUnaryExpression
    | LPAREN metaprogrammingCapabilityExpression RPAREN
    | metaprogrammingCapabilityReference
    ;


/*
 * --------------------------------------------------------------------------
 * Capability reference
 * --------------------------------------------------------------------------
 *
 * A reference may be written using the canonical capability function form:
 *
 *     capability("syntax.inspect")
 *
 * Or using a capability path:
 *
 *     syntax.inspect
 *
 * The string form is intentionally accepted as an open semantic identifier.
 *
 * It is not a dynamically evaluated expression.
 *
 * No host-language code is executed.
 */
metaprogrammingCapabilityReference
    : CAPABILITY LPAREN metaprogrammingCapabilityString RPAREN
    | metaprogrammingCapabilityPath
    ;


/*
 * --------------------------------------------------------------------------
 * Capability path
 * --------------------------------------------------------------------------
 *
 * Examples:
 *
 *     syntax.inspect
 *     syntax.generate
 *     quantum.operation.inspect
 *     hardware.capability.inspect
 *     compiler.extension.custom_transform
 *
 * The path is open-world.
 *
 * It does not enumerate capabilities.
 */
metaprogrammingCapabilityPath
    : IDENTIFIER (DOT IDENTIFIER)*
    ;


/*
 * --------------------------------------------------------------------------
 * Capability scope
 * --------------------------------------------------------------------------
 *
 * Scope names are ordinary identifiers.
 *
 * Semantic analysis determines whether a particular scope is supported.
 */
metaprogrammingCapabilityScope
    : IN metaprogrammingCapabilityPath
    ;


/*
 * --------------------------------------------------------------------------
 * Conditional capability metadata
 * --------------------------------------------------------------------------
 *
 * A condition is metadata, not an execution request.
 *
 * It must not trigger compile-time execution during parsing.
 */
metaprogrammingCapabilityCondition
    : WHEN metaprogrammingCapabilityExpression
    ;


/*
 * --------------------------------------------------------------------------
 * Capability metadata
 * --------------------------------------------------------------------------
 *
 * This rule provides a stable extension point for scope and conditional
 * metadata without redefining general annotations.
 */
metaprogrammingCapabilityMetadata
    : metaprogrammingCapabilityScope?
      metaprogrammingCapabilityCondition?
    ;


/*
 * --------------------------------------------------------------------------
 * Capability string
 * --------------------------------------------------------------------------
 *
 * STRING is owned by the canonical lexer.
 *
 * The grammar does not define string escaping, Unicode normalization,
 * or literal decoding.
 */
metaprogrammingCapabilityString
    : STRING
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The canonical frontend AST remains domain-neutral.
 *
 * Suggested structural mapping:
 *
 *     metaprogrammingCapabilityDeclaration
 *         -> CapabilityDeclaration
 *
 *     metaprogrammingCapabilityAllowance
 *         -> CapabilityAllowance
 *
 *     metaprogrammingCapabilityRequirement
 *         -> CapabilityRequirement
 *
 *     metaprogrammingCapabilityRestriction
 *         -> CapabilityRestriction
 *
 *     metaprogrammingCapabilityExpression
 *         -> CapabilityExpression
 *
 *     metaprogrammingCapabilityReference
 *         -> CapabilityReference
 *
 *     metaprogrammingCapabilityPath
 *         -> QualifiedCapabilityName
 *
 *     metaprogrammingCapabilityMetadata
 *         -> CapabilityMetadata
 *
 * These are AST integration contracts, not permission to introduce a
 * second metaprogramming AST.
 *
 * Every node must preserve:
 *
 *     source span
 *     syntax identity
 *     declaration/reference distinction
 *     original capability path
 *     expression structure
 *     metadata
 *
 * The frontend AST must not perform authorization.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     1. Resolve capability identifiers.
 *     2. Validate declaration ownership.
 *     3. Validate capability availability.
 *     4. Validate scope.
 *     5. Validate phase restrictions.
 *     6. Validate effect compatibility.
 *     7. Validate authorization policy.
 *     8. Reject privilege escalation.
 *     9. Preserve provenance.
 *    10. Distinguish unknown from unavailable capabilities.
 *    11. Distinguish requirements from grants.
 *    12. Distinguish resource constraints from capabilities.
 *    13. Preserve deterministic diagnostics.
 *
 * Unknown capability names must not automatically become authorized
 * capabilities.
 *
 * An unknown name may be rejected, deferred, or resolved through a registered
 * extension according to the language specification.
 *
 * ============================================================================
 * INTEGRATION WITH REFLECTION
 * ============================================================================
 *
 * grammar/metaprogramming/reflection.g4 owns reflection syntax.
 *
 * This file does not redefine reflection queries.
 *
 * Reflection may inspect capability metadata through the canonical semantic
 * reflection model.
 *
 * Reflection must not reveal protected implementation details merely because
 * a capability name appears in source.
 *
 * ============================================================================
 * INTEGRATION WITH GENERATION
 * ============================================================================
 *
 * grammar/metaprogramming/generation.g4 owns source-generation syntax.
 *
 * Generated constructs must retain capability provenance.
 *
 * Generation cannot create authority by emitting an allowance declaration.
 *
 * Generated syntax must pass through ordinary semantic and authorization
 * analysis.
 *
 * ============================================================================
 * INTEGRATION WITH SPECIALIZATION
 * ============================================================================
 *
 * grammar/metaprogramming/specialization.g4 owns specialization syntax.
 *
 * Specialization may depend on semantic capability facts.
 *
 * It must not make source parsing hardware-dependent.
 *
 * Capability availability must be resolved through the canonical capability
 * model, not through hard-coded backend names.
 *
 * ============================================================================
 * INTEGRATION WITH COMPILE-TIME EXECUTION
 * ============================================================================
 *
 * grammar/metaprogramming/compile-time-execution.g4 owns compile-time
 * execution syntax.
 *
 * This file only defines capability requirements.
 *
 * The evaluator must independently enforce:
 *
 *     authorization
 *     effect restrictions
 *     resource budgets
 *     deterministic evaluation
 *     isolation
 *     provenance
 *
 * The grammar must never execute an operation.
 *
 * ============================================================================
 * INTEGRATION WITH MACROS
 * ============================================================================
 *
 * grammar/macros/safety.g4 owns macro safety syntax.
 *
 * Macro safety and metaprogramming capability syntax must share one semantic
 * authorization model.
 *
 * They must not implement independent privilege systems.
 *
 * Macro expansion must preserve:
 *
 *     capability provenance
 *     source provenance
 *     hygiene
 *     effect information
 *     authorization context
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCES AND HARDWARE
 * ============================================================================
 *
 * grammar/resources/capabilities.g4 owns general resource capability syntax.
 *
 * grammar/hardware/ owns hardware intent and target-independent hardware
 * capability descriptions.
 *
 * This file may refer to those capabilities by semantic identifier.
 *
 * It must not redefine:
 *
 *     hardware capability declarations
 *     resource requirements
 *     resource budgets
 *     topology
 *     target selection
 *     physical device identifiers
 *
 * For example:
 *
 *     hardware.gpu.compute
 *     quantum.measurement
 *     tensor.compute
 *
 * are identifiers, not hard-coded hardware selection instructions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Capability metadata is resolved before backend lowering.
 *
 * It must not create:
 *
 *     metaprogramming IR
 *     quantum IR
 *     hardware IR
 *     a second semantic IR
 *
 * The canonical semantic model carries validated capability requirements
 * and provenance into the relevant IR consumers.
 *
 * Quantum operations continue to lower through quantum::ir.
 *
 * HDL and hardware constructs continue through their authoritative semantic
 * representations.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax diagnostics:
 *
 *     malformed capability declaration
 *     missing capability path
 *     missing expression
 *     malformed Boolean composition
 *     malformed string literal
 *     missing delimiter
 *
 * Semantic diagnostics:
 *
 *     unknown capability
 *     undeclared capability
 *     unauthorized capability
 *     invalid scope
 *     invalid phase
 *     incompatible effects
 *     conflicting restriction
 *     invalid capability dependency
 *     privilege escalation attempt
 *
 * Resource diagnostics:
 *
 *     resource exhausted
 *     compilation budget exceeded
 *
 * These categories must remain distinct.
 *
 * A resource failure must not be reported as a syntax error.
 *
 * An authorization failure must not be reported as a lexer error.
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 * The composed parser and semantic pipeline must reject or diagnose:
 *
 *     capability { }
 *
 *     capability syntax_transform {
 *         allows ;
 *     }
 *
 *     requires ;
 *
 *     forbids ;
 *
 *     requires capability();
 *
 *     requires capability("syntax.inspect") and ;
 *
 *     requires unknown_capability_if_unregistered;
 *
 *     allows filesystem.write;
 *
 * where the semantic policy does not authorize that capability.
 *
 * Parser acceptance must never imply authorization.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Verify:
 *
 *     - one capability;
 *     - multiple capabilities;
 *     - deeply nested Boolean expressions;
 *     - qualified capability paths;
 *     - unknown extension capability;
 *     - empty declaration body;
 *     - repeated declarations;
 *     - duplicate requirements;
 *     - conflicting restrictions;
 *     - Unicode identifiers where permitted by the lexer;
 *     - escaped capability strings;
 *     - malformed strings;
 *     - source-span preservation.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Verify that capability syntax has no language-defined ceiling for:
 *
 *     declaration count;
 *     capability count;
 *     path length;
 *     Boolean expression length;
 *     nesting;
 *     metaprogram count;
 *     generated constructs;
 *     computing domains;
 *     hardware capabilities.
 *
 * Tests must use configurable implementation budgets.
 *
 * They must not establish a universal language maximum.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Identical source and canonical token input must produce equivalent AST
 * structure and diagnostics.
 *
 * Capability parsing must not depend on:
 *
 *     current hardware;
 *     compiler host;
 *     runtime environment;
 *     available network;
 *     current time;
 *     random state.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden universal restrictions include:
 *
 *     MAX_CAPABILITIES
 *     MAX_CAPABILITY_DEPTH
 *     MAX_METAPROGRAMS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *
 * Capability names must not be represented as a closed parser-level
 * enumeration.
 *
 * ============================================================================
 * RUST IMPLEMENTATION CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation.
 *
 * The downstream implementation must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * It must use safe Rust only.
 *
 * It must not require:
 *
 *     unsafe
 *     transmute
 *     unchecked indexing
 *     raw pointer manipulation
 *     embedded grammar actions
 *
 * Parser and semantic resource limits must be configurable implementation
 * policies, not universal language restrictions.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This component is complete when:
 *
 *     [ ] Canonical token mappings are verified.
 *     [ ] All parser rules compose without unresolved references.
 *     [ ] No duplicate capability authority exists.
 *     [ ] Open-world capability paths work.
 *     [ ] Boolean composition is unambiguous.
 *     [ ] Capability declarations preserve source spans.
 *     [ ] Requirements and allowances remain distinct.
 *     [ ] Parsing does not grant authority.
 *     [ ] Macro safety shares the canonical authorization model.
 *     [ ] Resource capabilities integrate without duplication.
 *     [ ] Reflection integration is defined.
 *     [ ] Generation preserves provenance.
 *     [ ] Specialization remains target-independent.
 *     [ ] Negative tests pass.
 *     [ ] Boundary tests pass.
 *     [ ] Scalability tests pass.
 *     [ ] Determinism tests pass.
 *     [ ] No universal resource ceiling is introduced.
 *     [ ] Rust implementation remains safe.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */