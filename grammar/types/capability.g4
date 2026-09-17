/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/capability.g4
 *
 * Grammar:
 *     CapabilityTypes
 *
 * Status:
 *     Production-ready source-level capability type grammar.
 *
 * Purpose:
 *     Owns the complete source syntax of the canonical:
 *
 *         Capability<...>
 *
 *     type constructor.
 *
 * ============================================================================
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * This file owns ONLY source-level capability TYPE syntax.
 *
 * It owns:
 *
 *     - capabilityType;
 *     - capability identity syntax;
 *     - capability namespace qualification;
 *     - capability version constraints;
 *     - optional capability type parameters;
 *     - capability type argument structure;
 *     - capability type syntax integration with the generic type system;
 *     - source-level capability-type composition boundaries.
 *
 * It does NOT own:
 *
 *     - capability discovery;
 *     - capability registration;
 *     - capability authorization;
 *     - security credentials;
 *     - capability negotiation;
 *     - resource allocation;
 *     - hardware discovery;
 *     - target selection;
 *     - device selection;
 *     - physical resources;
 *     - CPU/GPU/FPGA/QPU counts;
 *     - topology;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime capability tokens;
 *     - backend capability implementations;
 *     - vendor-specific capability models.
 *
 * Those concerns belong to later semantic/compiler/runtime layers.
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
 *     parser
 *          |
 *          v
 *     capabilityType
 *          |
 *          v
 *     domain-neutral frontend TypeExpr
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic type resolution
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     capability semantics       resource/capability analysis
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                 canonical semantic model
 *                        |
 *              +---------+---------+
 *              |                   |
 *              v                   v
 *        classical semantics   quantum semantics
 *                                  |
 *                                  v
 *                             quantum::ir
 *                                  |
 *                        optimization/lowering
 *                                  |
 *                    routing/scheduling/resilience
 *                                  |
 *                                ZQN
 *                                  |
 *                                HAL
 *                                  |
 *                         target realization
 *
 * This grammar never depends on quantum::ir.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Capability syntax describes PORTABLE COMPUTATIONAL INTENT.
 *
 * It MUST NOT establish implementation limits such as:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_REGISTER_COUNT
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_RESOURCE_COUNT
 *     MAX_CAPABILITY_COUNT
 *     MAX_NAMESPACE_DEPTH
 *     MAX_CAPABILITY_PARAMETER_COUNT
 *
 * A capability identity is semantic data.
 *
 * For example:
 *
 *     Capability<"quantum.measurement">
 *
 * expresses a required semantic capability.
 *
 * It does NOT mean:
 *
 *     use QPU 0
 *     use device 0
 *     use exactly N qubits
 *     use a particular vendor
 *     use a particular backend
 *
 * The actual realization is resolved downstream from available resources and
 * capabilities.
 *
 * ============================================================================
 * OPEN-WORLD CAPABILITY MODEL
 * ============================================================================
 *
 * Capability identities are intentionally OPEN-WORLD.
 *
 * The grammar must NOT enumerate:
 *
 *     quantum.measurement
 *     quantum.dynamic_control
 *     quantum.logical_qubits
 *     gpu.compute
 *     cpu.vector
 *     hdl.synthesis
 *     tensor.compute
 *     distributed.collective
 *
 * as individual grammar alternatives.
 *
 * Those names are examples of semantic capabilities, not a closed language
 * vocabulary.
 *
 * A future capability must therefore be expressible without modifying this
 * grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates no AST.
 *
 * The parser maps the syntax into the existing domain-neutral frontend type
 * representation.
 *
 * The intended semantic structure is conceptually:
 *
 *     TypeExpr
 *       |
 *       +-- Capability
 *             |
 *             +-- identity
 *             +-- version constraint
 *             +-- type arguments
 *
 * The exact Rust representation remains owned by:
 *
 *     src/frontend/ast/
 *
 * The existing capability AST is explicitly source-level and intentionally
 * does not depend on quantum::ir, hardware, routing, scheduling, QEC, ZQN or
 * HAL. This grammar preserves that boundary.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "What capability type did the programmer write?"
 *
 * Semantic analysis answers:
 *
 *     "What does that capability mean?"
 *
 * Semantic analysis is responsible for:
 *
 *     - capability identity resolution;
 *     - namespace resolution;
 *     - extension/dialect lookup;
 *     - version satisfaction;
 *     - capability compatibility;
 *     - capability implication;
 *     - resource satisfaction;
 *     - target compatibility;
 *     - domain interpretation;
 *     - lowering eligibility.
 *
 * This grammar does none of those things.
 *
 * ============================================================================
 * CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * Capability and resource are different concepts.
 *
 * Capability:
 *
 *     Capability<"quantum.measurement">
 *
 * means:
 *
 *     "the computation requires the semantic ability to perform measurement."
 *
 * Resource:
 *
 *     Resource<Qubit>
 *
 * means:
 *
 *     "the computation has a resource type involving qubits."
 *
 * Neither construct selects a physical device.
 *
 * Similarly:
 *
 *     Capability<"gpu.compute">
 *
 * does not mean:
 *
 *     "use GPU 0".
 *
 * Resource/capability satisfaction remains downstream.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE SEPARATION
 * ============================================================================
 *
 * A capability TYPE is not itself a placement or scheduling directive.
 *
 * The following concepts remain distinct:
 *
 *     semantic requirement
 *     capability
 *     resource
 *     constraint
 *     preference
 *     hint
 *     implementation decision
 *
 * This file only defines the TYPE FORM of capability.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum capability types are ordinary open-world capability identities.
 *
 * Examples:
 *
 *     Capability<"quantum.measurement">
 *     Capability<"quantum.mid_circuit_measurement">
 *     Capability<"quantum.dynamic_control">
 *     Capability<"quantum.logical_qubits">
 *     Capability<"quantum.error_correction">
 *
 * These names are NOT enumerated here.
 *
 * The parser does not decide:
 *
 *     - physical qubit availability;
 *     - logical qubit count;
 *     - QEC code;
 *     - code distance;
 *     - decoder;
 *     - topology;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QPU selection.
 *
 * Semantic resolution may subsequently associate a quantum capability with
 * the canonical quantum::ir capability model.
 *
 * ============================================================================
 * QUANTUM::IR BOUNDARY
 * ============================================================================
 *
 * The transformation is:
 *
 *     CapabilityType syntax
 *          |
 *          v
 *     frontend TypeExpr
 *          |
 *          v
 *     semantic capability identity
 *          |
 *          v
 *     quantum::ir capability model
 *
 * This grammar MUST NOT:
 *
 *     - import quantum::ir;
 *     - define QuantumCapabilityIR;
 *     - define a second capability IR;
 *     - define physical-qubit capabilities;
 *     - define QEC implementation capabilities;
 *     - define ZQN capability semantics.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar is parser-only.
 *
 * It consumes the canonical composed vocabulary:
 *
 *     ZamaniTokens
 *
 * In particular it requires the canonical capability keyword token:
 *
 *     K_CAPABILITY
 *
 * The lexical spelling of that token belongs to:
 *
 *     grammar/lexer/
 *
 * This file MUST NOT define:
 *
 *     K_CAPABILITY
 *     IDENTIFIER
 *     STRING_LITERAL
 *     INTEGER_LITERAL
 *     punctuation
 *     operators
 *
 * locally.
 *
 * ============================================================================
 * COMPATIBILITY FORMS
 * ============================================================================
 *
 * The normative examples in the repository include:
 *
 *     Capability<"quantum.measurement">
 *
 * The grammar therefore treats a quoted capability identity as a supported
 * source spelling.
 *
 * An unquoted qualified identity is also supported:
 *
 *     Capability<quantum.measurement>
 *
 * and:
 *
 *     Capability<quantum::measurement>
 *
 * The semantic layer is responsible for canonicalizing these equivalent
 * source-level identity forms according to the language's capability naming
 * contract.
 *
 * The grammar does not infer meaning from the spelling.
 *
 * ============================================================================
 * TYPE PARAMETERS
 * ============================================================================
 *
 * Capability types may carry optional type/value parameters when a capability
 * contract requires them.
 *
 * Examples:
 *
 *     Capability<"tensor.compute", Tensor<float>>
 *
 *     Capability<"distributed.collective", Message>
 *
 *     Capability<"quantum.operation", Operation>
 *
 * Parameters are syntactic data.
 *
 * There is no finite language-level parameter-count limit.
 *
 * ============================================================================
 * VERSION CONSTRAINTS
 * ============================================================================
 *
 * Capability versions are source-level constraints.
 *
 * Examples:
 *
 *     Capability<"quantum.measurement", version >= 1.0.0>
 *
 *     Capability<"tensor.compute", version = 2.1.0>
 *
 *     Capability<"hdl.synthesis", version >= 1.0.0, version < 3.0.0>
 *
 * Version satisfaction belongs to semantic analysis.
 *
 * The grammar only preserves the requested constraint structure.
 *
 * No implementation version is selected by this file.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar intentionally uses repetition and recursive qualified names.
 *
 * There is no source-language maximum for:
 *
 *     - capability namespace depth;
 *     - capability identity length;
 *     - number of capability parameters;
 *     - number of capability types;
 *     - number of capability occurrences;
 *     - number of modules;
 *     - number of domains;
 *     - number of machines;
 *     - number of resources;
 *     - number of qubits;
 *     - number of CPUs;
 *     - number of accelerators.
 *
 * Compiler implementations may have configurable resource-safety policies for
 * hostile or pathological input, but those are NOT language semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The syntax is deterministic.
 *
 * The grammar must not depend on:
 *
 *     - hardware state;
 *     - runtime state;
 *     - current time;
 *     - random state;
 *     - process identity;
 *     - environment variables;
 *     - filesystem contents;
 *     - network state.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax errors should be reported against the smallest useful source span.
 *
 * Examples:
 *
 *     Capability<>
 *
 *     Capability<,>
 *
 *     Capability<"name",>
 *
 *     Capability<"name", version>
 *
 * are parser/semantic boundary cases.
 *
 * The grammar must not silently reinterpret malformed capability syntax as an
 * unrelated generic type.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation code.
 *
 * All Rust components integrating it MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and the repository-wide safe-Rust requirement:
 *
 *     no unsafe Rust
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. grammar/lexer/tokens.g4
 *
 *    Must expose K_CAPABILITY through the canonical ZamaniTokens vocabulary.
 *
 * 2. grammar/types/types.g4
 *
 *    Must import CapabilityTypes and delegate:
 *
 *        capabilityType
 *
 *    to this file.
 *
 *    `types.g4` MUST NOT contain another capabilityType implementation.
 *
 * 3. grammar/Zamani.g4
 *
 *    Must consume the canonical typeExpression from Types rather than defining
 *    another capability type.
 *
 * 4. src/frontend/ast/
 *
 *    The parser maps capability syntax into the existing domain-neutral
 *    capability/type AST.
 *
 * 5. semantic analysis
 *
 *    Resolves capability identity, namespace, versions, parameters,
 *    implications and target/resource compatibility.
 *
 * 6. quantum::ir
 *
 *    Quantum capability semantics are lowered through the existing canonical
 *    quantum::ir boundary.
 *
 * 7. compiler/runtime
 *
 *    Capability satisfaction and actual resource realization happen here or in
 *    the appropriate semantic/resource/runtime subsystem.
 *
 * 8. hardware/HAL
 *
 *    Determines whether a target can actually provide a capability.
 *
 * 9. routing/scheduling/QEC/ZQN
 *
 *    These systems may consume resolved capability information but must not be
 *    encoded into this grammar.
 *
 * ============================================================================
 */

parser grammar CapabilityTypes;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Canonical source-level capability type.
 *
 * Examples:
 *
 *     Capability<"quantum.measurement">
 *     Capability<"tensor.compute">
 *     Capability<"distributed.collective">
 *     Capability<"hdl.synthesis">
 *
 * Open-world qualified forms are also accepted:
 *
 *     Capability<quantum.measurement>
 *     Capability<quantum::measurement>
 *
 * Optional type/value parameters may follow the identity.
 */
capabilityType
    : K_CAPABILITY
      LESS_THAN
      capabilityTypeArgumentList
      GREATER_THAN
    ;


/* ============================================================================
 * CAPABILITY TYPE ARGUMENT LIST
 * ========================================================================== */

/**
 * Capability arguments consist of:
 *
 *     1. exactly one capability identity;
 *     2. zero or more capability parameters.
 *
 * The identity MUST be first so that:
 *
 *     Capability<identity, ...>
 *
 * remains structurally distinguishable from an arbitrary generic type.
 */
capabilityTypeArgumentList
    : capabilityIdentity
      (
          COMMA
          capabilityTypeArgument
      )*
      COMMA?
    ;


/**
 * Additional capability parameter.
 *
 * Parameters are intentionally open-ended.
 *
 * Semantic analysis determines whether the selected capability accepts a
 * particular parameter.
 */
capabilityTypeArgument
    : typeExpression
    | capabilityTypeValue
    | capabilityVersionConstraint
    ;


/* ============================================================================
 * CAPABILITY IDENTITY
 * ========================================================================== */

/**
 * Capability identity.
 *
 * Supported source forms include:
 *
 *     "quantum.measurement"
 *     "tensor.compute"
 *     quantum.measurement
 *     quantum::measurement
 *     future.domain.capability
 *     future::domain::capability
 *
 * The grammar does not maintain a registry of valid capability names.
 *
 * That is deliberate.
 */
capabilityIdentity
    : capabilityStringIdentity
    | capabilityQualifiedIdentity
    ;


/**
 * Quoted capability identity.
 *
 * The repository's normative type examples use this representation.
 *
 * The string contents remain source data. Semantic validation determines
 * whether the identity is well formed.
 */
capabilityStringIdentity
    : STRING_LITERAL
    ;


/**
 * Unquoted capability identity.
 *
 * Dot-separated and namespace-qualified forms are both supported.
 */
capabilityQualifiedIdentity
    : capabilityIdentitySegment
      (
          DOT capabilityIdentitySegment
      )*
      (
          DOUBLE_COLON capabilityIdentitySegment
          (
              DOT capabilityIdentitySegment
          )*
      )*
    ;


/**
 * One capability identity component.
 *
 * The lexer owns identifier spelling.
 */
capabilityIdentitySegment
    : IDENTIFIER
    ;


/* ============================================================================
 * CAPABILITY TYPE PARAMETERS
 * ========================================================================== */

/**
 * Generic capability parameter.
 *
 * This rule deliberately accepts ordinary type expressions and type-level
 * values rather than introducing a capability-specific type language.
 */
capabilityTypeParameter
    : typeExpression
    | capabilityTypeValue
    ;


/**
 * Source-level capability type value.
 *
 * Examples:
 *
 *     N
 *     1024
 *     width
 *     2 * N
 *
 * The grammar preserves the expression and does not evaluate it.
 */
capabilityTypeValue
    : capabilityValueUnary*
      capabilityValuePrimary
      capabilityValueBinaryPart*
    ;


/**
 * Unary operators for capability type values.
 */
capabilityValueUnary
    : PLUS
    | MINUS
    ;


/**
 * Binary expression component for capability type values.
 */
capabilityValueBinaryPart
    : capabilityValueOperator
      capabilityValueUnary*
      capabilityValuePrimary
    ;


/**
 * Operators usable in source-level capability type values.
 *
 * Semantic analysis determines which operators are legal for a particular
 * capability contract.
 */
capabilityValueOperator
    : PLUS
    | MINUS
    | STAR
    | SLASH
    | MODULO
    | LEFT_SHIFT
    | RIGHT_SHIFT
    | BIT_AND
    | BIT_OR
    | CARET
    ;


/**
 * Primary capability type value.
 */
capabilityValuePrimary
    : INTEGER_LITERAL
    | IDENTIFIER
    | capabilityQualifiedValue
    | capabilityValueParenthesized
    ;


/**
 * Qualified symbolic capability value.
 */
capabilityQualifiedValue
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )+
    ;


/**
 * Parenthesized capability type value.
 */
capabilityValueParenthesized
    : LPAREN
      capabilityTypeValue
      RPAREN
    ;


/* ============================================================================
 * CAPABILITY VERSION CONSTRAINTS
 * ========================================================================== */

/**
 * Source-level capability version requirement.
 *
 * Supported forms:
 *
 *     version = 1.0.0
 *     version >= 1.0.0
 *     version <= 2.0.0
 *     version > 1.0.0
 *     version < 3.0.0
 *     version >= 1.0.0, version < 3.0.0
 *
 * The grammar does not decide whether a version exists.
 */
capabilityVersionConstraint
    : capabilityVersionRequirement
    ;


/**
 * One version requirement.
 */
capabilityVersionRequirement
    : VERSION
      capabilityVersionOperator
      capabilityVersion
    ;


/**
 * Capability version comparison operator.
 *
 * These tokens must come from the canonical operator vocabulary.
 */
capabilityVersionOperator
    : EQUAL
    | NOT_EQUAL
    | LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    ;


/**
 * Capability semantic version.
 *
 * Version components are source data.
 *
 * No finite version width is imposed by the grammar.
 */
capabilityVersion
    : INTEGER_LITERAL
      DOT
      INTEGER_LITERAL
      (
          DOT
          INTEGER_LITERAL
      )?
      (
          capabilityVersionPrerelease
      )?
    ;


/**
 * Optional pre-release identifier.
 *
 * Examples:
 *
 *     1.0.0-alpha
 *     2.1.0-beta
 *     3.0.0-rc1
 *
 * The exact semantic interpretation belongs to version resolution.
 */
capabilityVersionPrerelease
    : MINUS
      capabilityVersionIdentifier
      (
          DOT
          capabilityVersionIdentifier
      )*
    ;


/**
 * Version identifier component.
 */
capabilityVersionIdentifier
    : IDENTIFIER
    | INTEGER_LITERAL
    ;


/* ============================================================================
 * CAPABILITY TYPE COMPOSITION
 * ========================================================================== */

/**
 * Capability type with nested capability parameters.
 *
 * Examples:
 *
 *     Capability<
 *         "quantum.operation",
 *         Capability<"quantum.measurement">
 *     >
 *
 * Nested capability types remain ordinary type expressions.
 */
nestedCapabilityType
    : capabilityType
    ;


/**
 * Capability type path.
 *
 * This rule is provided as a stable parser boundary for tooling and semantic
 * analysis that needs the identity path independently of the surrounding
 * type constructor.
 */
capabilityTypePath
    : capabilityQualifiedIdentity
    ;


/* ============================================================================
 * STRUCTURAL VALIDATION BOUNDARY
 * ========================================================================== */

/**
 * Structural capability type.
 *
 * This rule intentionally aliases the public entry point rather than creating
 * another semantic representation.
 *
 * It exists as an explicit integration boundary for validators and parser
 * tooling.
 */
capabilityTypeReference
    : capabilityType
    ;


/* ============================================================================
 * END OF CAPABILITY TYPE GRAMMAR
 * ============================================================================
 *
 * No lexer rules belong in this file.
 *
 * No capability registry belongs in this file.
 *
 * No hardware implementation belongs in this file.
 *
 * No quantum::ir representation belongs in this file.
 *
 * No QEC, ZQN, routing, scheduling, calibration, HAL or runtime behavior
 * belongs in this file.
 *
 * The complete architectural contract is:
 *
 *     capability syntax
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic capability identity
 *          |
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *     resource analysis      quantum semantics
 *                                  |
 *                                  v
 *                             quantum::ir
 *                                  |
 *                         target-independent
 *                           optimization
 *                                  |
 *                    routing / scheduling / QEC
 *                                  |
 *                                ZQN
 *                                  |
 *                                HAL
 *                                  |
 *                         target realization
 *
 * Therefore a new capability, domain, quantum technology, accelerator,
 * processor, machine size, deployment topology or future computational
 * substrate does not require changing this grammar.
 *
 * ============================================================================
 */