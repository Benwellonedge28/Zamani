/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/placement.g4
 *
 * Grammar:
 *     ExecutionPlacement
 *
 * Status:
 *     Production-ready execution-placement intent parser grammar
 *
 * Target implementation:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     No embedded Rust.
 *     No semantic predicates.
 *     No semantic actions.
 *     No unsafe Rust requirement.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines SOURCE-LEVEL EXECUTION PLACEMENT INTENT.
 *
 * It answers:
 *
 *     "Where is this execution permitted, preferred, constrained, or
 *      associated to occur?"
 *
 * It does NOT answer:
 *
 *     "Which physical resource should actually be selected?"
 *
 * Physical realization is determined downstream from:
 *
 *     target capabilities
 *     resource availability
 *     hardware topology
 *     routing
 *     scheduling
 *     deployment policy
 *     runtime state
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
 *     ExecutionPlacement
 *          |
 *          v
 *     frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> placement validation
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed representation
 *          |
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> hardware HAL
 *          |
 *          v
 *     runtime / deployment
 *
 * ============================================================================
 * CRITICAL OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - execution placement intent;
 *     - placement association with an execution subject;
 *     - placement requirements;
 *     - placement constraints;
 *     - placement preferences;
 *     - placement hints;
 *     - placement selectors;
 *     - placement locality intent;
 *     - placement affinity intent;
 *     - placement anti-affinity intent;
 *     - placement scope intent;
 *     - symbolic placement references;
 *     - placement property expressions.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - hardware declarations;
 *     - hardware topology;
 *     - hardware discovery;
 *     - resource discovery;
 *     - resource allocation;
 *     - physical qubit mapping;
 *     - quantum routing;
 *     - classical scheduling;
 *     - quantum scheduling;
 *     - deployment implementation;
 *     - runtime dispatch;
 *     - device APIs;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - resilience algorithms;
 *     - optimization algorithms;
 *     - canonical IR;
 *     - quantum::ir.
 *
 * ============================================================================
 * RELATIONSHIP TO grammar/hardware/placement.g4
 * ============================================================================
 *
 * grammar/hardware/placement.g4 owns hardware-domain placement declarations.
 *
 * This file MUST NOT duplicate that grammar.
 *
 * Hardware placement may describe hardware-oriented placement structures.
 *
 * Execution placement describes placement INTENT attached to execution.
 *
 * Therefore:
 *
 *     hardware/placement.g4
 *         |
 *         v
 *     hardware placement semantics
 *
 * while:
 *
 *     execution/placement.g4
 *         |
 *         v
 *     execution placement intent
 *
 * The two may eventually lower into a common semantic placement model, but
 * neither grammar imports the other's implementation semantics.
 *
 * ============================================================================
 * RELATIONSHIP TO execution.g4
 * ============================================================================
 *
 * execution.g4 owns execution composition.
 *
 * It already expects:
 *
 *     executionPlacement
 *
 * Therefore this file provides that public rule.
 *
 * execution.g4 should compose this rule rather than redefine it.
 *
 * ============================================================================
 * RELATIONSHIP TO scheduling
 * ============================================================================
 *
 * Placement and scheduling are intentionally separate.
 *
 * Placement says WHERE execution may/preferably occurs.
 *
 * Scheduling says WHEN execution occurs and in what legal order.
 *
 * This grammar MUST NOT define:
 *
 *     ASAP
 *     ALAP
 *     list scheduling
 *     critical path
 *     RCPSP
 *     resource allocation
 *     timing calculation
 *     pulse scheduling
 *
 * Those belong downstream.
 *
 * ============================================================================
 * RELATIONSHIP TO ROUTING
 * ============================================================================
 *
 * Placement does not perform routing.
 *
 * For quantum computation:
 *
 *     logical qubits
 *          |
 *          v
 *     placement intent
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     physical realization
 *
 * Placement MUST NOT insert:
 *
 *     SWAP
 *     MOVE
 *     transport operations
 *     topology paths
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Placement syntax must preserve:
 *
 *     Program_Once
 *          |
 *          v
 *     Compile_Once
 *          |
 *          v
 *     Run_Everywhere
 *          |
 *          v
 *     Run_Anywhere
 *          |
 *          v
 *     Run_Forever
 *
 * Therefore placement syntax MUST NOT require:
 *
 *     - a fixed device;
 *     - a fixed CPU;
 *     - a fixed GPU;
 *     - a fixed FPGA;
 *     - a fixed ASIC;
 *     - a fixed QPU;
 *     - a fixed node;
 *     - a fixed core;
 *     - a fixed thread;
 *     - a fixed qubit;
 *     - a fixed memory address;
 *     - a fixed topology;
 *     - a fixed cluster;
 *     - a fixed deployment environment.
 *
 * A source program may express symbolic requirements such as:
 *
 *     placement {
 *         locality: local;
 *         capability: quantum;
 *     }
 *
 * The actual physical realization remains downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar deliberately contains NO:
 *
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_QUBITS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_RESOURCES
 *     MAX_PLACEMENTS
 *     MAX_REGIONS
 *     MAX_GROUPS
 *     MAX_LOCATIONS
 *
 * Lists use grammar repetition.
 *
 * Quantities and selectors are expressions.
 *
 * Consequently a placement expression may depend on:
 *
 *     compile-time information;
 *     semantic information;
 *     target capabilities;
 *     runtime resource discovery;
 *     deployment configuration;
 *     negotiated resources.
 *
 * No artificial finite hardware limit is introduced here.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar:
 *
 *     - contains no actions;
 *     - contains no predicates;
 *     - performs no discovery;
 *     - performs no allocation;
 *     - performs no runtime calls;
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - contains no random behavior;
 *     - contains no mutable parser state.
 *
 * Parsing therefore depends only on the supplied token stream.
 *
 * ============================================================================
 * CORE CONTRACT
 * ============================================================================
 *
 * This grammar consumes canonical core constructs:
 *
 *     expression
 *     blockExpression
 *     identifier
 *     qualifiedName
 *
 * It MUST NOT redefine those rules.
 *
 * ============================================================================
 * TOKEN CONTRACT
 * ============================================================================
 *
 * This grammar deliberately uses the small execution-placement vocabulary
 * already exposed by the execution grammar:
 *
 *     PLACEMENT
 *
 * and ordinary core punctuation/operators where available.
 *
 * Placement properties remain identifiers rather than requiring an enormous
 * closed keyword vocabulary.
 *
 * This permits future properties without modifying this grammar.
 *
 * If the canonical lexer later changes the spelling or token name of
 * PLACEMENT, the lexer/parser compatibility layer must update the token
 * contract centrally rather than introducing a second lexer here.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The parser produces syntax only.
 *
 * Semantic analysis must determine:
 *
 *     - whether the placement subject is valid;
 *     - whether a selector resolves;
 *     - whether a capability exists;
 *     - whether a requirement can be satisfied;
 *     - whether a constraint conflicts with another constraint;
 *     - whether a preference is satisfiable;
 *     - whether placement is compatible with the computation;
 *     - whether quantum placement is valid for the canonical quantum IR;
 *     - whether distributed placement is legal;
 *     - whether hardware placement is supported;
 *     - whether the requested placement is portable.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should represent this construct approximately as:
 *
 *     ExecutionPlacement {
 *         subject: Option<Expression>,
 *         clauses: Vec<PlacementClause>,
 *         span: SourceSpan
 *     }
 *
 * This grammar does NOT define Rust AST structures.
 *
 * The AST must preserve source spans and source ordering.
 *
 * Semantic normalization may later canonicalize clause ordering.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does not define an IR.
 *
 * Placement semantics should lower into the canonical semantic representation
 * consumed by the compiler.
 *
 * Quantum programs must eventually integrate with:
 *
 *     quantum::ir
 *
 * rather than creating an execution-specific quantum IR.
 *
 * ============================================================================
 * HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware capabilities and actual resources are discovered by hardware/resource
 * layers.
 *
 * This grammar may express:
 *
 *     capability
 *     resource
 *     locality
 *     affinity
 *     target
 *
 * but does not determine their actual values.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime consumes the resolved placement plan.
 *
 * Runtime may:
 *
 *     honor placement;
 *     negotiate placement;
 *     adapt placement;
 *     reject unsatisfied requirements.
 *
 * Runtime MUST NOT infer that a source placement expression means a particular
 * physical machine unless semantic resolution explicitly establishes that
 * meaning.
 *
 * ============================================================================
 * RESILIENCE CONTRACT
 * ============================================================================
 *
 * Placement may become invalid during execution because resources fail,
 * disappear, become unavailable, or change capability.
 *
 * Resilience owns recovery/adaptation.
 *
 * This grammar does not implement:
 *
 *     retry;
 *     failover;
 *     backend switching;
 *     migration algorithms;
 *     checkpoint restoration.
 *
 * ============================================================================
 */

parser grammar ExecutionPlacement;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the rule imported/composed by execution.g4.
 *
 * Canonical form:
 *
 *     placement { ... }
 *
 * Subject-associated form:
 *
 *     placement computation { ... }
 *
 * The subject is optional because execution.g4 may already establish the
 * execution subject externally.
 */
executionPlacement
    : PLACEMENT executionPlacementSubject? executionPlacementBody
    ;


/*
 * ============================================================================
 * PLACEMENT SUBJECT
 * ============================================================================
 *
 * A subject is intentionally an expression.
 *
 * This avoids a closed inventory such as:
 *
 *     task
 *     function
 *     circuit
 *     kernel
 *     device
 *
 * Any semantic entity representable by the language can therefore become a
 * placement subject.
 */
executionPlacementSubject
    : expression
    ;


/*
 * ============================================================================
 * PLACEMENT BODY
 * ============================================================================
 *
 * Empty bodies are legal.
 *
 * This is useful for:
 *
 *     - incremental compilation;
 *     - generated source;
 *     - macros;
 *     - tooling;
 *     - future extension.
 */
executionPlacementBody
    : LBRACE executionPlacementClause* RBRACE
    ;


/*
 * ============================================================================
 * CLAUSE COMPOSITION
 * ============================================================================
 *
 * Clauses are intentionally semantic categories.
 *
 * The grammar does not impose an execution order on them.
 */
executionPlacementClause
    : executionPlacementRequirement
    | executionPlacementConstraint
    | executionPlacementPreference
    | executionPlacementHint
    | executionPlacementCapability
    | executionPlacementResource
    | executionPlacementTarget
    | executionPlacementLocality
    | executionPlacementAffinity
    | executionPlacementAntiAffinity
    | executionPlacementScope
    | executionPlacementProperty
    ;


/*
 * ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory.
 *
 * The semantic layer decides whether it can be satisfied.
 *
 * Example:
 *
 *     requires quantum;
 *
 * or:
 *
 *     requires capability::quantum;
 *
 * The grammar does not enumerate capabilities.
 */
executionPlacementRequirement
    : REQUIRES executionPlacementValue executionPlacementTerminator
    ;


/*
 * ============================================================================
 * CONSTRAINT
 * ============================================================================
 *
 * A constraint restricts acceptable realizations.
 *
 * It is not an allocation instruction.
 *
 * Example:
 *
 *     locality == local;
 */
executionPlacementConstraint
    : executionPlacementName executionPlacementComparison executionPlacementValue executionPlacementTerminator
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 *
 * Preferences are advisory.
 *
 * Failure to honor a preference does not automatically make execution
 * semantically invalid.
 */
executionPlacementPreference
    : executionPlacementName executionPlacementPreferenceOperator executionPlacementValue executionPlacementTerminator
    ;


/*
 * ============================================================================
 * HINT
 * ============================================================================
 *
 * Hints are advisory implementation guidance.
 *
 * They must never silently become mandatory requirements.
 */
executionPlacementHint
    : executionPlacementName executionPlacementHintOperator executionPlacementValue executionPlacementTerminator
    ;


/*
 * ============================================================================
 * CAPABILITY
 * ============================================================================
 *
 * Capability requirements remain open-ended.
 *
 * Examples:
 *
 *     capability: quantum;
 *     capability: accelerator;
 *     capability: distributed;
 *
 * The grammar does not maintain a finite capability catalogue.
 */
executionPlacementCapability
    : CAPABILITY executionPlacementValue executionPlacementTerminator
    ;


/*
 * ============================================================================
 * RESOURCE
 * ============================================================================
 *
 * Resource expressions are semantic references.
 *
 * This does not allocate resources.
 *
 * It also does not impose a maximum resource count.
 */
executionPlacementResource
    : RESOURCE executionPlacementValue executionPlacementTerminator
    ;


/*
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * Target remains abstract.
 *
 * A target expression may resolve to:
 *
 *     target class;
 *     target profile;
 *     deployment domain;
 *     capability set;
 *     runtime-selected target.
 *
 * A target expression does not inherently mean a physical device.
 */
executionPlacementTarget
    : TARGET executionPlacementValue executionPlacementTerminator
    ;


/*
 * ============================================================================
 * LOCALITY
 * ============================================================================
 *
 * Locality is semantic intent.
 *
 * Examples:
 *
 *     locality: local;
 *     locality: remote;
 *     locality: any;
 *
 * The actual meaning of local/remote is resolved by the execution environment.
 */
executionPlacementLocality
    : executionPlacementName executionPlacementLocalityOperator executionPlacementValue executionPlacementTerminator
    ;


/*
 * ============================================================================
 * AFFINITY
 * ============================================================================
 *
 * Affinity expresses a preference or constraint for co-location or related
 * placement.
 *
 * It does not choose a physical resource.
 */
executionPlacementAffinity
    : executionPlacementName executionPlacementRelationOperator executionPlacementName executionPlacementTerminator
    ;


/*
 * ============================================================================
 * ANTI-AFFINITY
 * ============================================================================
 *
 * Anti-affinity expresses separation intent.
 */
executionPlacementAntiAffinity
    : executionPlacementName executionPlacementAntiRelationOperator executionPlacementName executionPlacementTerminator
    ;


/*
 * ============================================================================
 * SCOPE
 * ============================================================================
 *
 * Scope identifies the semantic domain in which placement applies.
 *
 * Examples:
 *
 *     scope: local;
 *     scope: cluster;
 *     scope: region;
 *     scope: any;
 *
 * These names remain expressions/identifiers rather than physical topology
 * declarations.
 */
executionPlacementScope
    : SCOPE executionPlacementValue executionPlacementTerminator
    ;


/*
 * ============================================================================
 * GENERIC PROPERTY
 * ============================================================================
 *
 * Generic properties provide forward-compatible extension without requiring
 * a new grammar keyword for every future placement concept.
 *
 * Example:
 *
 *     locality: preferred;
 *
 *     topology: nearest;
 *
 *     energy: efficient;
 *
 *     vendor: neutral;
 *
 * The semantic layer determines whether a property is standard, dialect
 * specific, unsupported, or invalid.
 */
executionPlacementProperty
    : executionPlacementName executionPlacementPropertyOperator executionPlacementValue executionPlacementTerminator
    ;


/*
 * ============================================================================
 * VALUES
 * ============================================================================
 *
 * Values are expressions.
 *
 * This is critical for scalability.
 *
 * A value can therefore be:
 *
 *     literal
 *     symbolic
 *     generic
 *     compile-time
 *     target-derived
 *     resource-derived
 *     runtime-derived
 *
 * without changing this grammar.
 */
executionPlacementValue
    : expression
    | executionPlacementPropertyBlock
    ;


/*
 * ============================================================================
 * PROPERTY BLOCK
 * ============================================================================
 *
 * Nested properties permit structured placement metadata without creating a
 * new grammar for every domain.
 */
executionPlacementPropertyBlock
    : LBRACE executionPlacementPropertyEntry* RBRACE
    ;


executionPlacementPropertyEntry
    : executionPlacementName executionPlacementPropertyOperator executionPlacementValue executionPlacementTerminator
    ;


/*
 * ============================================================================
 * NAMES
 * ============================================================================
 *
 * Qualified names allow extensible namespaces:
 *
 *     capability::quantum
 *     target::generic
 *     locality::region
 *     policy::custom
 *
 * The grammar does not enumerate future domains.
 */
executionPlacementName
    : qualifiedName
    ;


/*
 * ============================================================================
 * OPERATORS
 * ============================================================================
 *
 * Equality/comparison operators are used only to express placement semantics.
 *
 * They do not cause allocation or selection.
 */
executionPlacementComparison
    : EQUALS
    | NOT_EQUALS
    | LESS_THAN
    | LESS_THAN_EQUAL
    | GREATER_THAN
    | GREATER_THAN_EQUAL
    ;


/*
 * ============================================================================
 * PREFERENCE OPERATOR
 * ============================================================================
 *
 * A preference may be written with assignment-like syntax.
 *
 * Semantic analysis determines whether the property represents:
 *
 *     preference
 *     default
 *     advisory value
 *     dialect extension
 */
executionPlacementPreferenceOperator
    : EQUALS
    | ASSIGN
    ;


/*
 * ============================================================================
 * HINT OPERATOR
 * ============================================================================
 *
 * Hints are explicitly advisory at the semantic level.
 */
executionPlacementHintOperator
    : EQUALS
    | ASSIGN
    ;


/*
 * ============================================================================
 * LOCALITY OPERATOR
 * ============================================================================
 */
executionPlacementLocalityOperator
    : EQUALS
    | ASSIGN
    ;


/*
 * ============================================================================
 * RELATION OPERATORS
 * ============================================================================
 *
 * Relations remain semantic.
 */
executionPlacementRelationOperator
    : TO
    | ON
    | IN
    | WITH
    | EQUALS
    ;


/*
 * ============================================================================
 * ANTI-RELATION OPERATORS
 * ============================================================================
 */
executionPlacementAntiRelationOperator
    : TO
    | ON
    | IN
    | WITH
    | NOT_EQUALS
    ;


/*
 * ============================================================================
 * PROPERTY OPERATOR
 * ============================================================================
 */
executionPlacementPropertyOperator
    : COLON
    | EQUALS
    | ASSIGN
    ;


/*
 * ============================================================================
 * TERMINATORS
 * ============================================================================
 *
 * Semicolons terminate clauses.
 *
 * This permits one property per clause while keeping parsing deterministic.
 */
executionPlacementTerminator
    : SEMI
    ;