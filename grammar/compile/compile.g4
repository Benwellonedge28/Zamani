/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/compile.g4
 *
 * Role:
 *     Canonical parser fragment for source-level compilation intent.
 *
 * Architectural position:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     Core / domain parser
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> capability / requirement analysis
 *       +--> resource analysis
 *       +--> effect analysis
 *       +--> target-independent compilation planning
 *       |
 *       v
 *     canonical IR
 *       |
 *       +--> optimization
 *       +--> scheduling
 *       +--> routing
 *       +--> hardware lowering
 *       +--> backend
 *       |
 *       v
 *     executable / deployable artifact
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar introduces no Rust implementation code.
 *     All compiler implementations consuming this grammar MUST use safe Rust.
 *     Rust `unsafe` is neither required nor permitted by this grammar contract.
 *
 * ============================================================================
 *
 * FUNDAMENTAL CONTRACT
 * ============================================================================
 *
 * This grammar describes COMPILATION INTENT.
 *
 * It does NOT perform compilation.
 *
 * It does NOT:
 *
 *     - select a physical machine;
 *     - discover hardware;
 *     - define a hardware topology;
 *     - define a quantum topology;
 *     - define a qubit count;
 *     - define a CPU count;
 *     - define a GPU count;
 *     - define a memory limit;
 *     - define a register limit;
 *     - define a fixed vector width;
 *     - define a fixed tensor dimension;
 *     * define backend-specific gate sets;
 *     - perform optimization;
 *     - perform scheduling;
 *     - perform routing;
 *     - perform QEC;
 *     - define ZQN/noise semantics;
 *     - generate machine code;
 *     - execute code;
 *     - access the filesystem;
 *     - access the network;
 *     - execute arbitrary host-language code.
 *
 * Compilation intent is declarative.
 *
 * Hardware-specific realization is a downstream concern.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A Zamani source program should describe:
 *
 *     WHAT computation is required
 *     WHAT capabilities are required
 *     WHAT constraints must hold
 *     WHAT implementation freedom is permitted
 *
 * rather than:
 *
 *     WHICH CURRENT MACHINE MUST EXECUTE IT
 *
 * Consequently:
 *
 *     requirement != target
 *     capability != device
 *     preference != requirement
 *     constraint != topology
 *     hint != guarantee
 *     resource expression != fixed resource count
 *
 * ============================================================================
 *
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The canonical parser assembly is responsible for importing this grammar.
 *
 * This grammar intentionally uses the existing lexical vocabulary:
 *
 *     IDENTIFIER
 *     INTEGER
 *     FLOAT
 *     STRING
 *     TRUE
 *     FALSE
 *
 * together with punctuation/operators supplied by ZamaniLexer.
 *
 * Compilation keywords are represented through the existing language's
 * extensible identifier surface where dedicated lexical tokens do not yet
 * exist. This avoids silently modifying the canonical lexer from a parser
 * fragment and avoids creating a second lexical authority.
 *
 * A future lexer revision may promote frequently-used compilation words to
 * reserved tokens. Such a promotion is a compatibility change and MUST NOT
 * change the semantic model defined here.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - source-level compilation intent syntax;
 *     - target-independent compilation requests;
 *     - compilation profiles;
 *     - compilation requirements;
 *     - compilation constraints;
 *     - compilation preferences;
 *     - compilation hints;
 *     - feature-selection requests;
 *     - artifact-kind requests;
 *     - compilation-stage requests;
 *     - conditional compilation declarations;
 *     - compile-time target-independent configuration.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - paths;
 *     - expressions;
 *     - types;
 *     - modules;
 *     - functions;
 *     - resources;
 *     - hardware capabilities;
 *     - quantum IR;
 *     - classical IR;
 *     - HDL IR;
 *     - optimization algorithms;
 *     - scheduling algorithms;
 *     - routing algorithms;
 *     - QEC algorithms;
 *     - ZQN/noise models;
 *     - backend implementation;
 *     - executable generation;
 *     - runtime execution.
 *
 * ============================================================================
 *
 * IMPORTANT AST RULE
 * ============================================================================
 *
 * Every rule in this file should lower to an AST representation of SOURCE
 * INTENT.
 *
 * It must NOT lower directly to:
 *
 *     CompilationTarget
 *     backend configuration
 *     physical device identifiers
 *     hardware topology
 *     compiler implementation objects
 *     machine-code instructions
 *
 * Semantic analysis owns that interpretation.
 *
 * ============================================================================
 */

parser grammar Compile;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * COMPILATION UNIT INTEGRATION
 * ============================================================================
 *
 * `compileDeclaration` is the only top-level entry point owned by this file.
 *
 * The canonical program/declaration grammar imports or references this rule.
 *
 * Example integration:
 *
 *     declaration
 *         : ...
 *         | compileDeclaration
 *         ;
 *
 * The canonical declaration grammar remains responsible for declaration
 * ordering, attributes, visibility and module placement.
 *
 * This grammar does not redefine `declaration`.
 */
compileDeclaration
    : compileDirective
    | compilationProfile
    | compilationRequirement
    | compilationConstraint
    | compilationPreference
    | compilationHint
    | compilationFeature
    | compilationConditional
    | compilationArtifact
    | compilationStage
    ;


/*
 * ============================================================================
 * GENERIC COMPILATION DIRECTIVE
 * ============================================================================
 *
 * Generic directive form:
 *
 *     compile <name> ...
 *
 * The directive name remains an identifier rather than a finite enum.
 *
 * This is intentional.
 *
 * New compiler strategies can therefore be introduced without changing the
 * grammar merely because a new backend or compilation technology appears.
 *
 * Semantic validation determines which directive names are recognized.
 */
compileDirective
    : identifier compileDirectiveBody?
    ;


compileDirectiveBody
    : LPAREN compileArgumentList? RPAREN
    | LBRACE compileDirectiveEntry* RBRACE
    | ASSIGN expression
    ;


compileDirectiveEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier expression SEMICOLON
    ;


/*
 * ============================================================================
 * COMPILATION PROFILE
 * ============================================================================
 *
 * A profile describes a named set of compilation intent.
 *
 * A profile is NOT:
 *
 *     - a hardware target;
 *     - a device configuration;
 *     - a backend implementation;
 *     - an optimization pass.
 *
 * Profiles may be resolved by tooling, build configuration, or semantic
 * analysis.
 *
 * The grammar does not prescribe a finite set of profile names.
 */
compilationProfile
    : identifier identifier LBRACE compilationProfileEntry* RBRACE
    ;


compilationProfileEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier expression SEMICOLON
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Requirements are semantic obligations.
 *
 * A requirement means:
 *
 *     "The resulting execution must satisfy this property."
 *
 * It does NOT mean:
 *
 *     "Use this particular device."
 *
 * Examples at the semantic level include:
 *
 *     requires quantum
 *     requires capability
 *     requires precision
 *     requires memory
 *     requires latency
 *
 * The grammar remains generic so future computing paradigms do not require
 * parser redesign.
 */
compilationRequirement
    : identifier compilationRequirementBody
    ;


compilationRequirementBody
    : expression
    | LBRACE compilationRequirementEntry* RBRACE
    ;


compilationRequirementEntry
    : identifier COLON expression SEMICOLON
    | identifier ASSIGN expression SEMICOLON
    ;


/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 *
 * Constraints limit legal implementations without prescribing one particular
 * implementation.
 *
 * A constraint can be evaluated against:
 *
 *     - semantic properties;
 *     - capabilities;
 *     - resource models;
 *     - deployment environments;
 *     - compilation contexts.
 *
 * It must not become a hidden hardware-selection mechanism.
 */
compilationConstraint
    : identifier compilationConstraintBody
    ;


compilationConstraintBody
    : expression
    | LBRACE compilationConstraintEntry* RBRACE
    ;


compilationConstraintEntry
    : identifier comparisonOperator expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 *
 * Preferences are non-mandatory optimization or realization guidance.
 *
 * A preference MUST NOT be treated as a semantic requirement.
 *
 * This distinction is essential for portability.
 */
compilationPreference
    : identifier compilationPreferenceBody
    ;


compilationPreferenceBody
    : expression
    | LBRACE compilationPreferenceEntry* RBRACE
    ;


compilationPreferenceEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * HINTS
 * ============================================================================
 *
 * Hints provide optional information to downstream compilation stages.
 *
 * Hints must never be required for semantic correctness.
 *
 * A backend may ignore a hint.
 */
compilationHint
    : identifier compilationHintBody
    ;


compilationHintBody
    : expression
    | LBRACE compilationHintEntry* RBRACE
    ;


compilationHintEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * FEATURE SELECTION
 * ============================================================================
 *
 * Feature declarations describe source-level or compilation-level optional
 * capabilities.
 *
 * The grammar intentionally does not enumerate every future feature.
 *
 * Feature identity belongs to semantic capability resolution.
 */
compilationFeature
    : identifier compilationFeatureBody
    ;


compilationFeatureBody
    : identifier
    | expression
    | LBRACE compilationFeatureEntry* RBRACE
    ;


compilationFeatureEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * CONDITIONAL COMPILATION
 * ============================================================================
 *
 * Conditional compilation selects source-level structure based on a
 * compile-time semantic predicate.
 *
 * This is distinct from runtime `if`.
 *
 * The condition is still an ordinary Zamani expression.
 *
 * The semantic layer determines whether it is compile-time evaluable.
 *
 * This rule imposes no fixed number of branches.
 */
compilationConditional
    : identifier expression
      LBRACE compilationConditionalBody RBRACE
      (identifier LBRACE compilationConditionalBody RBRACE)*
      (identifier LBRACE compilationConditionalBody RBRACE)?
    ;


compilationConditionalBody
    : compilationConditionalItem*
    ;


compilationConditionalItem
    : compileDeclaration
    | expression SEMICOLON
    ;


/*
 * ============================================================================
 * ARTIFACT REQUEST
 * ============================================================================
 *
 * An artifact request describes WHAT representation is desired.
 *
 * It does not prescribe how the representation is generated.
 *
 * Examples of semantic artifact categories:
 *
 *     source
 *     canonical IR
 *     quantum IR
 *     classical IR
 *     HDL
 *     object
 *     executable
 *     deployable package
 *
 * Actual artifact identifiers remain extensible.
 */
compilationArtifact
    : identifier compilationArtifactBody
    ;


compilationArtifactBody
    : identifier
    | STRING
    | LBRACE compilationArtifactEntry* RBRACE
    ;


compilationArtifactEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * COMPILATION STAGE
 * ============================================================================
 *
 * A compilation stage identifies a semantic stage or requested pipeline
 * boundary.
 *
 * It does NOT execute that stage.
 *
 * It does NOT define its implementation.
 */
compilationStage
    : identifier compilationStageBody?
    ;


compilationStageBody
    : identifier
    | LBRACE compilationStageEntry* RBRACE
    ;


compilationStageEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * TARGET-NEUTRAL TARGET EXPRESSION
 * ============================================================================
 *
 * POCO-REAF requires the language to distinguish a semantic requirement from
 * a concrete target.
 *
 * This rule therefore represents target intent as an expression/name rather
 * than an enum containing today's architectures.
 *
 * Examples of semantic target categories:
 *
 *     classical
 *     quantum
 *     hybrid
 *     hardware
 *     distributed
 *     accelerator
 *
 * A concrete implementation such as:
 *
 *     x86_64
 *     arm64
 *     a particular QPU
 *     a particular FPGA
 *
 * is resolved outside this grammar.
 */
compilationTarget
    : identifier
    | qualifiedIdentifier
    | STRING
    | expression
    ;


compilationTargetSet
    : compilationTarget
    | LBRACKET compilationTargetList? RBRACKET
    ;


compilationTargetList
    : compilationTarget
      (COMMA compilationTarget)*
    ;


/*
 * ============================================================================
 * RESOURCE-NEUTRAL RESOURCE EXPRESSION
 * ============================================================================
 *
 * Resource quantities are expressions.
 *
 * No maximum cardinality is encoded.
 *
 * Examples:
 *
 *     number of qubits
 *     amount of memory
 *     parallelism
 *     latency
 *     energy
 *
 * The semantic resource model determines units and feasibility.
 */
compilationResource
    : identifier
    | qualifiedIdentifier
    | expression
    ;


compilationResourceBinding
    : identifier ASSIGN compilationResource
    ;


/*
 * ============================================================================
 * CAPABILITY REFERENCE
 * ============================================================================
 *
 * Capabilities describe what an execution environment can provide.
 *
 * A capability is not a device.
 *
 * Capability discovery belongs to the hardware/runtime/compiler context.
 */
compilationCapability
    : identifier
    | qualifiedIdentifier
    | STRING
    ;


compilationCapabilityList
    : compilationCapability
      (COMMA compilationCapability)*
    ;


/*
 * ============================================================================
 * COMPILATION ARGUMENTS
 * ============================================================================
 *
 * Arguments remain ordinary Zamani expressions.
 *
 * This prevents the grammar from creating a second expression/type system.
 */
compileArgumentList
    : compileArgument
      (COMMA compileArgument)*
    ;


compileArgument
    : identifier ASSIGN expression
    | expression
    ;


/*
 * ============================================================================
 * IDENTIFIER INTEGRATION
 * ============================================================================
 *
 * These rules are deliberately compatibility aliases.
 *
 * If the canonical parser already owns `identifier` and
 * `qualifiedIdentifier`, the canonical parser must replace these references
 * with those shared rules during grammar assembly.
 *
 * They are not intended to introduce a second identifier definition.
 */
identifier
    : IDENTIFIER
    ;


qualifiedIdentifier
    : identifier
      (DCOLON identifier)*
    ;


/*
 * ============================================================================
 * COMPARISON OPERATORS
 * ============================================================================
 *
 * These operators are intentionally limited to the operators already supplied
 * by the canonical lexical grammar.
 *
 * Semantic typing and comparison validity remain downstream.
 */
comparisonOperator
    : EQ
    | NEQ
    | LT
    | LE
    | GT
    | GE
    ;


/*
 * ============================================================================
 * INTEGRATION ADAPTERS
 * ============================================================================
 *
 * These aliases make the file easy to integrate into different parser
 * assembly arrangements without duplicating semantic rules.
 *
 * Canonical parser integration should reference:
 *
 *     compileDeclaration
 *
 * directly.
 *
 * These adapters are not alternate compilation systems.
 */
compileItem
    : compileDeclaration
    ;


compileItems
    : compileItem*
    ;


/*
 * ============================================================================
 * SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * The AST generated from this grammar should preserve at least these
 * distinctions:
 *
 *     Requirement
 *     Constraint
 *     Preference
 *     Hint
 *     Capability
 *     Resource
 *     TargetIntent
 *     ArtifactRequest
 *     StageRequest
 *     FeatureSelection
 *
 * They MUST NOT be collapsed into a generic "compiler option".
 *
 * Downstream semantic analysis is responsible for resolving:
 *
 *     source intent
 *         ->
 *     compilation context
 *         ->
 *     available capabilities
 *         ->
 *     feasible resource assignments
 *         ->
 *     compilation plan
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * This file does not define:
 *
 *     Qubit
 *     PhysicalQubitId
 *     LogicalQubitId
 *     QuantumGate
 *     QuantumOperation
 *     Circuit
 *     QEC code
 *     Noise model
 *
 * Quantum source syntax belongs to:
 *
 *     grammar/quantum/
 *
 * Quantum semantic lowering ultimately belongs at the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * Compilation intent may refer to quantum capabilities/resources through
 * generic expressions, but this file must never create a second quantum IR.
 *
 * ============================================================================
 *
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical computation remains governed by the canonical classical grammar
 * and semantic/type system.
 *
 * This file may express compilation requirements for classical computation
 * but does not redefine classical expressions, functions or types.
 *
 * ============================================================================
 *
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware grammars own:
 *
 *     hardware modules
 *     signals
 *     ports
 *     clocks
 *     timing
 *     hardware resources
 *     hardware semantics
 *
 * This grammar only expresses compilation intent around those constructs.
 *
 * A target declaration must never force a fixed chip, FPGA, ASIC, topology,
 * clock frequency or device count.
 *
 * ============================================================================
 *
 * OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * Optimization is downstream.
 *
 * A source preference may influence optimization, but this grammar does not
 * select or implement an optimization algorithm.
 *
 * Examples:
 *
 *     preference
 *     hint
 *     optimization intent
 *
 * remain metadata/intent until semantic analysis and optimization planning.
 *
 * ============================================================================
 *
 * SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Scheduling is downstream.
 *
 * Compilation intent may express constraints/preferences such as:
 *
 *     latency
 *     ordering
 *     throughput
 *     timing requirements
 *     resource requirements
 *
 * but must not create schedules or timestamps.
 *
 * ============================================================================
 *
 * ROUTING / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Placement, topology and routing are downstream.
 *
 * Source code must not encode an accidental fixed topology merely because
 * one backend currently has such a topology.
 *
 * ============================================================================
 *
 * ZQN / QEC INTEGRATION
 * ============================================================================
 *
 * Noise and fault semantics belong to ZQN.
 *
 * Error correction belongs to QEC.
 *
 * Compilation intent may express requirements concerning reliability or
 * resilience but must not define the algorithms implementing them.
 *
 * ============================================================================
 *
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime receives the semantic compilation result.
 *
 * This grammar does not define:
 *
 *     dispatch
 *     execution
 *     retry
 *     recovery
 *     backend switching
 *
 * Those belong to runtime/resilience/execution subsystems.
 *
 * ============================================================================
 *
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no machine-cardinality constants.
 *
 * It does not define:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_SIZE
 *
 * Any practical limit belongs to:
 *
 *     parser/runtime limits
 *     compiler resource policy
 *     semantic analysis
 *     hardware capability
 *     runtime availability
 *
 * and must be represented explicitly rather than silently embedded in syntax.
 *
 * ============================================================================
 *
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * For the same source token stream, this grammar must produce the same parse
 * structure.
 *
 * It must not:
 *
 *     - inspect hardware;
 *     - inspect wall-clock time;
 *     - access environment variables;
 *     - perform network access;
 *     - perform filesystem access;
 *     - perform random selection.
 *
 * ============================================================================
 *
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Semantic errors such as:
 *
 *     unknown capability
 *     impossible resource requirement
 *     unavailable target
 *     unsupported artifact
 *     contradictory constraints
 *
 * belong to semantic analysis/compiler diagnostics.
 *
 * The grammar must not encode those errors as parser actions.
 *
 * ============================================================================
 *
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Compilation syntax does not grant permissions.
 *
 * A source-level request such as:
 *
 *     compile
 *     deploy
 *     target
 *     hardware
 *     network
 *
 * must never itself authorize:
 *
 *     filesystem access
 *     network access
 *     device access
 *     credential access
 *     process execution
 *
 * Capability enforcement belongs downstream.
 *
 * ============================================================================
 *
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * New compilation concepts should preferably be represented as extensible
 * identifiers and semantic records rather than requiring a new finite enum
 * in this grammar.
 *
 * This permits future compilation technologies to be introduced without
 * breaking old source programs.
 *
 * If an existing identifier is promoted to a reserved lexer token, that
 * change requires a language-version compatibility review.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * 1. It is imported by the canonical parser assembly.
 *
 * 2. Its lexer vocabulary comes exclusively from ZamaniLexer.
 *
 * 3. No second lexer is introduced.
 *
 * 4. No machine-specific limits are encoded.
 *
 * 5. No backend-specific target enum is encoded.
 *
 * 6. Requirements, constraints, preferences and hints remain distinct.
 *
 * 7. Compilation syntax lowers to source intent rather than machine code.
 *
 * 8. Quantum syntax remains owned by grammar/quantum/.
 *
 * 9. Hardware syntax remains owned by grammar/hardware/ and grammar/hdl/.
 *
 * 10. Canonical IR remains the semantic boundary.
 *
 * 11. Optimization, scheduling and routing remain downstream.
 *
 * 12. Runtime and resilience remain downstream.
 *
 * 13. The grammar is deterministic.
 *
 * 14. The grammar introduces no filesystem/network side effects.
 *
 * 15. Positive, negative, boundary and cross-domain tests exist.
 *
 * 16. Parser generation succeeds with the canonical Zamani lexer.
 *
 * 17. Rust consumers remain compatible with Rust 1.97/1.97.1.
 *
 * 18. Rust implementations contain no `unsafe`.
 *
 * ============================================================================
 */