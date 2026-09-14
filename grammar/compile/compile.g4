/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/compile.g4
 *
 * Grammar:
 *     Compile
 *
 * Status:
 *     Production compilation-intent parser grammar
 *
 * Purpose:
 *     Owns source-level compilation intent.
 *
 * This file describes WHAT compilation is requested, not HOW a compiler,
 * backend, scheduler, router, hardware device, QPU, CPU, GPU, FPGA, ASIC,
 * simulator, cluster, cloud service, or runtime must implement it.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser / Compile
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> name resolution
 *       +--> type analysis
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> target resolution
 *       +--> compilation planning
 *       |
 *       v
 *     canonical semantic IR
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL / hardware representation
 *       +--> distributed representation
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling / resilience / HAL
 *       |
 *       v
 *     target lowering
 *       |
 *       v
 *     runtime / deployment
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Source compilation intent MUST remain independent of:
 *
 *     - CPU count
 *     - GPU count
 *     - FPGA count
 *     - ASIC count
 *     - accelerator count
 *     - qubit count
 *     - register count
 *     - memory capacity
 *     - topology
 *     - physical addresses
 *     - device identifiers
 *     - vendor identifiers
 *     - queue size
 *     - deployment size
 *     - network size
 *
 * No finite machine inventory is encoded here.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the `compile` declaration boundary;
 *     - compilation-intent grouping;
 *     - compilation profiles;
 *     - compilation requirements;
 *     - compilation constraints;
 *     - compilation preferences;
 *     - compilation hints;
 *     - compilation feature requests;
 *     - compilation artifact requests;
 *     - compilation stage requests;
 *     - compilation plan composition;
 *     - target-independent compilation options.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - ordinary expressions;
 *     - types;
 *     - runtime control flow;
 *     - compile-time expressions;
 *     - compile-time functions;
 *     - targets;
 *     - hardware descriptions;
 *     - resources;
 *     - quantum syntax;
 *     - classical syntax;
 *     - HDL syntax;
 *     - optimization algorithms;
 *     - scheduling algorithms;
 *     - routing algorithms;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - canonical IR;
 *     - runtime execution.
 *
 * Those concepts remain owned by their respective grammar or semantic
 * subsystem.
 *
 * ============================================================================
 *
 * IMPORTANT COMPOSITION RULE
 * ============================================================================
 *
 * This grammar MUST NOT define:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     argumentList
 *     blockExpression
 *     pattern
 *     typeExpression
 *     attribute
 *
 * Those are canonical parser contracts.
 *
 * Redefining them here would create competing syntax authorities.
 *
 * ============================================================================
 *
 * IMPORTANT SEMANTIC RULE
 * ============================================================================
 *
 * A grammar node created by this file represents SOURCE INTENT.
 *
 * It MUST NOT directly become:
 *
 *     physical device configuration
 *     backend configuration
 *     hardware topology
 *     quantum operation
 *     quantum circuit
 *     machine instruction
 *     schedule
 *     route
 *     executable
 *
 * Semantic analysis and lowering perform those transformations.
 *
 * ============================================================================
 *
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no device discovery;
 *     - no host execution;
 *     - no unsafe code.
 *
 * Compiler/runtime implementation target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * Compiler implementation MUST use safe Rust only.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_TARGETS
 *     MAX_FEATURES
 *     MAX_RESOURCES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_NODES
 *     MAX_STAGES
 *     MAX_ARTIFACTS
 *     MAX_REQUIREMENTS
 *     MAX_CONSTRAINTS
 *
 * Repetition is represented by parser repetition.
 *
 * Actual resource limits belong to compiler policy, resource analysis,
 * execution infrastructure, or hardware capability discovery.
 *
 * ============================================================================
 */

parser grammar Compile;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. CANONICAL ENTRY POINT
 * ============================================================================
 *
 * Canonical source form:
 *
 *     compile <compile-specification>
 *
 * `COMPILE` is the lexical boundary for this grammar.
 *
 * This is deliberately NOT represented as an arbitrary IDENTIFIER.
 *
 * That prevents:
 *
 *     compile(...)
 *
 * from becoming indistinguishable from an ordinary function call or user
 * identifier and gives the language a stable compilation-intent boundary.
 *
 * The COMPILE token is shared with:
 *
 *     grammar/compile/compile-time.g4
 *
 * and therefore MUST be defined once by the canonical lexer.
 */

compileDeclaration
    : COMPILE compileSpecification SEMI?
    ;


/* ============================================================================
 * 2. COMPILATION SPECIFICATION
 * ============================================================================
 *
 * A compilation specification can contain zero or more independent intent
 * clauses.
 *
 * The grammar imposes no fixed number of clauses.
 *
 * Empty compilation specifications are rejected because:
 *
 *     compile;
 *
 * carries no compilation intent and is therefore almost certainly a source
 * error.
 *
 * If the language later assigns a meaningful semantic to an empty compile
 * declaration, that is a semantic-version change rather than an implicit
 * parser behavior.
 */

compileSpecification
    : compileClause+
    ;


/* ============================================================================
 * 3. COMPILATION CLAUSES
 * ============================================================================
 *
 * Each clause has a distinct semantic category.
 *
 * The categories MUST remain distinct in the AST.
 *
 * In particular:
 *
 *     requirement != constraint
 *     preference != hint
 *     target != capability
 *     artifact != stage
 *     profile != target
 *
 * The compiler must not collapse them into a generic compiler-option map.
 */

compileClause
    : compileProfile
    | compileRequirement
    | compileConstraint
    | compilePreference
    | compileHint
    | compileFeature
    | compileArtifact
    | compileStage
    | compilePlan
    | compileOption
    ;


/* ============================================================================
 * 4. PROFILE
 * ============================================================================
 *
 * A profile is a named reusable compilation-intent grouping.
 *
 * Example:
 *
 *     compile profile portable_quantum {
 *         ...
 *     }
 *
 * A profile is NOT:
 *
 *     - a physical device;
 *     - a backend;
 *     - a topology;
 *     - a schedule;
 *     - an optimization implementation.
 */

compileProfile
    : PROFILE identifier compileProfileBody
    ;

compileProfileBody
    : LBRACE compileProfileEntry* RBRACE
    ;

compileProfileEntry
    : compileClause
    ;


/* ============================================================================
 * 5. REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory semantic intent.
 *
 * Example:
 *
 *     compile requires quantum;
 *
 *     compile requires {
 *         capability: quantum;
 *         precision: p;
 *     }
 *
 * A requirement says what must be true.
 *
 * It does NOT select a physical device.
 */

compileRequirement
    : REQUIRES compileRequirementValue
    ;

compileRequirementValue
    : expression
    | compilePropertyBlock
    ;


/* ============================================================================
 * 6. CONSTRAINT
 * ============================================================================
 *
 * A constraint limits acceptable implementations.
 *
 * It does not identify a particular implementation.
 *
 * Example:
 *
 *     compile constrain latency < bound;
 *
 * The comparison itself remains an ordinary Zamani expression.
 *
 * Semantic validity is downstream.
 */

compileConstraint
    : CONSTRAIN compileConstraintValue
    ;

compileConstraintValue
    : expression
    | compileConstraintBlock
    ;

compileConstraintBlock
    : LBRACE compileConstraintEntry+ RBRACE
    ;

compileConstraintEntry
    : identifier compileConstraintOperator expression SEMI?
    ;

compileConstraintOperator
    : EQUALS
    | NOT_EQUALS
    | LESS_THAN
    | LESS_THAN_EQUAL
    | GREATER_THAN
    | GREATER_THAN_EQUAL
    ;


/* ============================================================================
 * 7. PREFERENCE
 * ============================================================================
 *
 * A preference is optional guidance.
 *
 * Ignoring a preference MUST NOT change program semantic correctness.
 */

compilePreference
    : PREFER compilePreferenceValue
    ;

compilePreferenceValue
    : expression
    | compilePropertyBlock
    ;


/* ============================================================================
 * 8. HINT
 * ============================================================================
 *
 * A hint provides optional implementation information.
 *
 * A compiler/backend is permitted to ignore a hint.
 *
 * A hint MUST NOT be required for semantic correctness.
 */

compileHint
    : HINT compileHintValue
    ;

compileHintValue
    : expression
    | compilePropertyBlock
    ;


/* ============================================================================
 * 9. FEATURE
 * ============================================================================
 *
 * Feature requests describe language/compiler/environment capabilities.
 *
 * They are intentionally open-ended.
 *
 * The grammar does not enumerate:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     ASIC
 *     vendor-specific features
 *
 * Such identities are semantic capability information.
 */

compileFeature
    : FEATURE compileFeatureValue
    ;

compileFeatureValue
    : expression
    | compileFeatureBlock
    ;

compileFeatureBlock
    : LBRACE compileFeatureEntry+ RBRACE
    ;

compileFeatureEntry
    : identifier
      (ASSIGN expression)?
      SEMI?
    ;


/* ============================================================================
 * 10. ARTIFACT
 * ============================================================================
 *
 * An artifact request describes a desired semantic representation.
 *
 * It does not prescribe its implementation.
 *
 * Examples include, semantically:
 *
 *     source
 *     canonical-ir
 *     quantum-ir
 *     classical-ir
 *     hdl
 *     object
 *     executable
 *     deployable
 *
 * The grammar does not enumerate those values.
 */

compileArtifact
    : ARTIFACT compileArtifactSpecification
    ;

compileArtifactSpecification
    : identifier
    | qualifiedName
    | STRING
    | compilePropertyBlock
    ;


/* ============================================================================
 * 11. STAGE
 * ============================================================================
 *
 * A stage identifies a semantic compilation boundary.
 *
 * It does not execute that stage.
 *
 * It does not prescribe its implementation.
 */

compileStage
    : STAGE compileStageSpecification
    ;

compileStageSpecification
    : identifier
    | qualifiedName
    | STRING
    | compilePropertyBlock
    ;


/* ============================================================================
 * 12. PLAN
 * ============================================================================
 *
 * A plan describes desired compilation composition.
 *
 * The compiler remains responsible for constructing the actual executable
 * compilation plan.
 *
 * This syntax is intent, not execution.
 */

compilePlan
    : PLAN compilePlanBody
    ;

compilePlanBody
    : LBRACE compilePlanEntry+ RBRACE
    ;

compilePlanEntry
    : compilePlanStage
    | compileClause
    ;

compilePlanStage
    : STAGE compileStageSpecification SEMI?
    ;


/* ============================================================================
 * 13. GENERIC COMPILATION OPTION
 * ============================================================================
 *
 * Options are intentionally namespaced/open-ended.
 *
 * They are not backend switches by definition.
 *
 * Semantic analysis determines whether an option is:
 *
 *     - recognized;
 *     - compatible;
 *     - deprecated;
 *     - unsupported;
 *     - ignored;
 *     - transformed.
 *
 * An option MUST NOT silently acquire semantic authority merely because a
 * backend recognizes its name.
 */

compileOption
    : OPTION identifier
      (ASSIGN expression)?
      SEMI?
    ;


/* ============================================================================
 * 14. PROPERTY BLOCK
 * ============================================================================
 *
 * Generic property blocks provide extensibility without adding a new grammar
 * rule every time a future computing model introduces a new semantic property.
 *
 * Properties remain data.
 *
 * Their names and values are interpreted downstream.
 */

compilePropertyBlock
    : LBRACE compilePropertyEntry+ RBRACE
    ;

compilePropertyEntry
    : compilePropertyName
      (COLON | ASSIGN)
      expression
      SEMI?
    ;

compilePropertyName
    : identifier
    | qualifiedName
    ;


/* ============================================================================
 * 15. TARGET INTEGRATION
 * ============================================================================
 *
 * Target semantics belong to:
 *
 *     grammar/compile/target.g4
 *
 * This file MUST NOT duplicate target grammar.
 *
 * The canonical parser composition layer should integrate:
 *
 *     compileDeclaration
 *         |
 *         +--> compile target intent
 *                 |
 *                 v
 *             CompileTarget
 *
 * Therefore target-specific grammar is intentionally NOT reproduced here.
 *
 * A compilation property may reference a target-related semantic expression,
 * but this file does not interpret it.
 */


/* ============================================================================
 * 16. COMPILE-TIME INTEGRATION
 * ============================================================================
 *
 * Compile-time control syntax belongs to:
 *
 *     grammar/compile/compile-time.g4
 *
 * This file does NOT redefine:
 *
 *     compile if
 *     compile select
 *     compile require
 *     compile specialize
 *     compile feature
 *     compile-time loops
 *     compile-time matching
 *
 * Those are control/evaluation constructs rather than compilation-intent
 * declaration structure.
 *
 * The canonical parser composition layer decides where
 * `compileTimeControlForm` may appear.
 */


/* ============================================================================
 * 17. EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Every value-bearing position consumes the canonical:
 *
 *     expression
 *
 * rule.
 *
 * This file therefore creates no second expression grammar.
 *
 * Consequences:
 *
 *     arithmetic
 *     logical operators
 *     comparison
 *     function calls
 *     indexing
 *     member access
 *     ranges
 *     quantum/classical expressions
 *
 * remain owned by the canonical expression parser.
 */


/* ============================================================================
 * 18. NAME INTEGRATION
 * ============================================================================
 *
 * `identifier` and `qualifiedName` are consumed from the canonical parser.
 *
 * They MUST NOT be redefined here.
 *
 * This prevents independent name-resolution domains from emerging.
 */


/* ============================================================================
 * 19. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource semantics belong to:
 *
 *     grammar/resources/
 *
 * This file can express resource-related intent through:
 *
 *     expression
 *     compilePropertyBlock
 *
 * but does not define:
 *
 *     memory models
 *     qubit models
 *     CPU models
 *     GPU models
 *     FPGA models
 *     node models
 *     resource allocation
 *     resource discovery
 *
 * Therefore:
 *
 *     compile requires memory > required
 *
 * is source intent.
 *
 * Whether that requirement is satisfiable belongs to semantic/resource
 * analysis.
 */


/* ============================================================================
 * 20. CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capability discovery and interpretation belong downstream.
 *
 * Examples:
 *
 *     quantum capability
 *     tensor capability
 *     parallel execution capability
 *     fault-tolerant capability
 *
 * are not machine definitions.
 *
 * This grammar does not know whether a capability is provided by:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     distributed cluster
 *     future architecture
 */


/* ============================================================================
 * 21. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum source syntax remains owned by grammar/quantum/.
 *
 * This grammar MUST NOT define:
 *
 *     Qubit
 *     PhysicalQubitId
 *     LogicalQubitId
 *     QuantumGate
 *     QuantumOperation
 *     Circuit
 *     Measurement
 *     QEC code
 *     noise model
 *
 * Compilation intent can refer to quantum requirements through expressions
 * and semantic properties.
 *
 * After semantic analysis, quantum computation MUST lower through the
 * repository's canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This file therefore never creates or duplicates a quantum IR.
 */


/* ============================================================================
 * 22. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical semantics remain owned by the classical grammar and semantic
 * layers.
 *
 * Compilation intent may constrain or describe classical execution
 * requirements but cannot redefine classical computation.


/* ============================================================================
 * 23. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL syntax belongs to:
 *
 *     grammar/hdl/
 *
 * Hardware-description syntax belongs to:
 *
 *     grammar/hardware/
 *
 * This file can express compilation intent surrounding such programs but
 * cannot define:
 *
 *     ports
 *     wires
 *     clocks
 *     registers
 *     topology
 *     physical placement
 *     chip inventory
 *
 * Those are downstream semantic/hardware concerns.
 */


/* ============================================================================
 * 24. OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * Optimization intent may be expressed as a preference or option.
 *
 * This file does NOT select or implement optimization algorithms.
 *
 * For example, a preference such as:
 *
 *     compile prefer optimization.level = level;
 *
 * is metadata.
 *
 * It does not directly invoke an optimization pass.
 */


/* ============================================================================
 * 25. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Scheduling is downstream.
 *
 * Compilation intent may express:
 *
 *     latency
 *     throughput
 *     ordering
 *     timing
 *     resource
 *
 * requirements or preferences.
 *
 * This grammar never creates timestamps or schedules.
 */


/* ============================================================================
 * 26. ROUTING / HARDWARE HAL
 * ============================================================================
 *
 * Routing and hardware realization remain downstream.
 *
 * No physical topology can be inferred merely from a compilation declaration.
 */


/* ============================================================================
 * 27. QEC / ZQN / RESILIENCE
 * ============================================================================
 *
 * QEC owns error-correction algorithms.
 *
 * ZQN owns noise/fault semantics.
 *
 * Resilience owns adaptive recovery/decision orchestration.
 *
 * Compilation intent may express semantic reliability/resilience requirements,
 * but this file does not define their implementation.
 */


/* ============================================================================
 * 28. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime execution is downstream.
 *
 * This grammar does not:
 *
 *     dispatch
 *     execute
 *     retry
 *     recover
 *     migrate
 *     switch backends
 *     access devices
 *
 * A compilation declaration may produce metadata consumed by those systems,
 * but it does not perform their work.
 */


/* ============================================================================
 * 29. SECURITY
 * ============================================================================
 *
 * A compile declaration is NOT an authorization mechanism.
 *
 * Syntax such as:
 *
 *     compile ...
 *
 * MUST NOT grant:
 *
 *     filesystem access
 *     network access
 *     credential access
 *     device access
 *     process execution
 *
 * Permissions and capabilities are established by the compiler/runtime
 * security model.
 */


/* ============================================================================
 * 30. DETERMINISM
 * ============================================================================
 *
 * Parsing must depend exclusively on the token stream.
 *
 * This grammar performs no:
 *
 *     - time inspection;
 *     - randomness;
 *     - environment inspection;
 *     - filesystem access;
 *     - network access;
 *     - hardware discovery.
 *
 * Therefore identical source/token streams have identical parse structure.
 */


/* ============================================================================
 * 31. SCALABILITY
 * ============================================================================
 *
 * Repetition is unbounded at the language level:
 *
 *     compileClause+
 *     compileProfileEntry*
 *     compilePropertyEntry+
 *     compilePlanEntry+
 *
 * There are no hardware-cardinality limits.
 *
 * A practical compiler may impose resource budgets for safety and denial-of-
 * service protection, but such budgets MUST be external to language semantics
 * and MUST NOT be encoded as grammar cardinality limits.
 */


/* ============================================================================
 * 32. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve the following semantic categories:
 *
 *     CompileDeclaration
 *       └── CompileSpecification
 *             ├── Profile
 *             ├── Requirement
 *             ├── Constraint
 *             ├── Preference
 *             ├── Hint
 *             ├── Feature
 *             ├── Artifact
 *             ├── Stage
 *             ├── Plan
 *             └── Option
 *
 * Each category must preserve source spans.
 *
 * Values must retain their canonical expression representation rather than
 * being prematurely converted into backend/compiler objects.
 */


/* ============================================================================
 * 33. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     compile source intent
 *          |
 *          v
 *     resolve names
 *          |
 *          v
 *     validate expressions/types
 *          |
 *          v
 *     resolve capabilities
 *          |
 *          v
 *     evaluate resource requirements
 *          |
 *          v
 *     resolve target intent
 *          |
 *          v
 *     construct compilation plan
 *
 * Semantic errors include:
 *
 *     unknown profile
 *     unknown capability
 *     contradictory requirements
 *     impossible constraints
 *     invalid option
 *     unsupported artifact
 *     incompatible stages
 *     unsatisfied resource requirement
 *
 * These are NOT parser errors.
 */


/* ============================================================================
 * 34. COMPILER / IR CONTRACT
 * ============================================================================
 *
 * This grammar does not lower directly to machine code.
 *
 * The expected direction is:
 *
 *     Compile AST
 *        |
 *        v
 *     semantic compilation intent
 *        |
 *        v
 *     target-independent compilation plan
 *        |
 *        v
 *     canonical semantic IR
 *        |
 *        +--> quantum::ir
 *        +--> classical IR
 *        +--> HDL/hardware representation
 *        +--> distributed representation
 *
 * The compiler may then invoke:
 *
 *     optimization
 *     routing
 *     scheduling
 *     resilience
 *     hardware HAL
 *     backend lowering
 *
 * according to the resolved plan.
 */


/* ============================================================================
 * 35. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The `compile` introducer is a stable language keyword/token.
 *
 * Clause names are deliberately semantic categories.
 *
 * Future implementation properties should preferably be introduced as:
 *
 *     qualified names
 *     properties
 *     options
 *     profiles
 *     capabilities
 *
 * rather than extending a finite machine-specific enum.
 *
 * Adding a new property should therefore not require changing the grammar
 * merely because a new processor, accelerator, quantum technology, topology,
 * or execution environment appears.
 */


/* ============================================================================
 * 36. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file intentionally contains no:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_TARGETS
 *     MAX_RESOURCES
 *     MAX_ARTIFACTS
 *     MAX_STAGES
 *
 * No physical device identifier is embedded in syntax.
 *
 * No hardware topology is embedded in syntax.
 *
 * No compiler backend is selected by grammar.
 */


/* ============================================================================
 * 37. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when ALL of the following are true:
 *
 * [ ] COMPILE is provided exactly once by the canonical lexer.
 *
 * [ ] This grammar does not redefine identifier.
 *
 * [ ] This grammar does not redefine qualifiedName.
 *
 * [ ] This grammar does not redefine expression.
 *
 * [ ] This grammar does not redefine typeExpression.
 *
 * [ ] This grammar does not redefine blockExpression.
 *
 * [ ] Target syntax remains owned by target.g4.
 *
 * [ ] Compile-time control remains owned by compile-time.g4.
 *
 * [ ] Expression-level compile-time syntax remains owned by
 *     expressions/compile-time.g4.
 *
 * [ ] Compilation-time functions remain owned by
 *     functions/compile-time-functions.g4.
 *
 * [ ] Resource semantics remain owned by resources/.
 *
 * [ ] Hardware semantics remain owned by hardware/.
 *
 * [ ] Quantum syntax remains owned by quantum/.
 *
 * [ ] quantum::ir remains the canonical quantum semantic boundary.
 *
 * [ ] No fixed machine/resource limits exist.
 *
 * [ ] Parser generation succeeds using ZamaniLexer.
 *
 * [ ] The canonical parser can invoke compileDeclaration.
 *
 * [ ] The generated Rust parser compiles under Rust 1.97 and 1.97.1.
 *
 * [ ] Compiler implementation contains no unsafe Rust.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] AST source spans are preserved.
 *
 * [ ] Semantic errors are not represented as parser actions.
 *
 * [ ] No filesystem/network/device side effects exist in parsing.
 *
 * ============================================================================
 */