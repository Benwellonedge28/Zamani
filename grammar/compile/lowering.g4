/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/lowering.g4
 *
 * Role:
 *     Parser fragment for SOURCE-LEVEL LOWERING INTENT.
 *
 * Architectural position:
 *
 *     Source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     Zamani parser / frontend
 *       |
 *       v
 *     Frontend AST
 *       |
 *       v
 *     Semantic analysis
 *       |
 *       v
 *     Canonical semantic IR
 *       |
 *       +--> optimization
 *       +--> routing
 *       +--> scheduling
 *       +--> QEC
 *       +--> ZQN
 *       +--> hardware/resource analysis
 *       |
 *       v
 *     LOWERING
 *       |
 *       +--> target-independent lowering
 *       |
 *       v
 *     target-specific realization
 *       |
 *       v
 *     backend / executable / deployable representation
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This is an ANTLR grammar and contains no Rust implementation code.
 *     Compiler implementations consuming this grammar MUST use safe Rust.
 *     No Rust `unsafe` is required or permitted by the Zamani compiler
 *     contract.
 *
 * ============================================================================
 *
 * FUNDAMENTAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - source-level lowering declarations;
 *   - lowering intent;
 *   - lowering boundaries;
 *   - lowering stages;
 *   - lowering requests;
 *   - lowering relationships;
 *   - lowering policies;
 *   - lowering requirements;
 *   - lowering constraints;
 *   - lowering preferences;
 *   - lowering hints;
 *   - representation-selection intent;
 *   - source-level preservation requirements;
 *   - semantic-preservation intent;
 *   - lowering provenance intent;
 *   - lowering extensibility points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identifiers;
 *   - qualified names;
 *   - expressions;
 *   - types;
 *   - declarations;
 *   - functions;
 *   - modules;
 *   - resources;
 *   - hardware;
 *   - targets;
 *   - optimization;
 *   - scheduling;
 *   - routing;
 *   - quantum IR;
 *   - classical IR;
 *   - HDL IR;
 *   - QEC;
 *   - ZQN;
 *   - runtime execution;
 *   - backend implementation;
 *   - machine instructions;
 *   - binary formats;
 *   - linker behavior;
 *   - assembler behavior;
 *   - device selection;
 *   - hardware discovery.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Lowering syntax describes how a semantic program MAY be transformed while
 * preserving its defined meaning.
 *
 * It MUST NOT encode temporary characteristics of a particular machine.
 *
 * Therefore this grammar MUST NOT require:
 *
 *   - a fixed CPU architecture;
 *   - a fixed ISA;
 *   - a fixed number of registers;
 *   - a fixed vector width;
 *   - a fixed number of cores;
 *   - a fixed number of threads;
 *   - a fixed number of GPUs;
 *   - a fixed number of FPGAs;
 *   - a fixed number of accelerators;
 *   - a fixed memory capacity;
 *   - a fixed number of qubits;
 *   - a fixed quantum topology;
 *   - a fixed gate set;
 *   - a fixed network topology;
 *   - a fixed node count;
 *   - a fixed deployment size;
 *   - a physical device identifier;
 *   - a physical address.
 *
 * Those properties belong to downstream target, capability, resource,
 * scheduling, placement, hardware, runtime, or deployment models.
 *
 * ============================================================================
 *
 * IMPORTANT ARCHITECTURAL DISTINCTIONS
 * ============================================================================
 *
 * lowering intent != lowering implementation
 * lowering stage != backend
 * representation != physical device
 * target requirement != target selection
 * capability != device
 * resource requirement != resource allocation
 * preference != guarantee
 * hint != requirement
 * semantic preservation != implementation strategy
 *
 * ============================================================================
 *
 * CANONICAL SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The grammar MUST NOT create a second IR.
 *
 * In particular:
 *
 *   quantum::ir
 *
 * remains the canonical semantic representation for quantum computation.
 *
 * This grammar can describe an intent such as:
 *
 *     lower quantum semantics to <representation>
 *
 * but it MUST NOT define another quantum gate, qubit, circuit, operation,
 * topology, scheduling, QEC, or noise representation.
 *
 * The same principle applies to classical, HDL, hardware, AI, distributed,
 * networking, and future computational domains.
 *
 * ============================================================================
 */

parser grammar CompileLowering;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The canonical compilation/declaration grammar should reference:
 *
 *     loweringDeclaration
 *
 * This file intentionally does not redefine:
 *
 *     declaration
 *     expression
 *     identifier
 *     qualifiedIdentifier
 *     type
 *     block
 *
 * Those belong to their canonical grammar owners.
 *
 * ============================================================================
 */

loweringDeclaration
    : loweringDirective
    | loweringRequest
    | loweringStageDeclaration
    | loweringPipelineDeclaration
    | loweringPolicyDeclaration
    ;


/*
 * ============================================================================
 * LOWERING DIRECTIVE
 * ============================================================================
 *
 * Generic extensibility surface.
 *
 * The directive name is an identifier so that future lowering technologies
 * do not require modification of the grammar merely because a new lowering
 * concept appears.
 *
 * Semantic validation determines whether a directive is recognized.
 *
 * The parser does not execute directives.
 * ============================================================================
 */

loweringDirective
    : identifier loweringDirectiveBody?
    ;

loweringDirectiveBody
    : LPAREN loweringArgumentList? RPAREN
    | LBRACE loweringProperty* RBRACE
    | ASSIGN expression
    ;


/*
 * ============================================================================
 * LOWERING REQUEST
 * ============================================================================
 *
 * A lowering request expresses that a transformation boundary is desired.
 *
 * It does not perform the transformation.
 *
 * It does not select a concrete machine.
 *
 * It does not instantiate a backend.
 * ============================================================================
 */

loweringRequest
    : identifier loweringRequestSubject loweringRequestBody?
    ;

loweringRequestSubject
    : identifier
    | qualifiedIdentifier
    | STRING
    | expression
    ;

loweringRequestBody
    : LPAREN loweringArgumentList? RPAREN
    | LBRACE loweringProperty* RBRACE
    ;


/*
 * ============================================================================
 * LOWERING STAGE
 * ============================================================================
 *
 * A lowering stage names a semantic transformation boundary.
 *
 * Stage identity is intentionally extensible.
 *
 * Examples of semantic stage categories may include:
 *
 *     high-level
 *     canonical
 *     classical
 *     quantum
 *     HDL
 *     accelerator
 *     target-independent
 *     target-specific
 *
 * These names are not hard-coded here because the language must remain
 * extensible to future computational models.
 * ============================================================================
 */

loweringStageDeclaration
    : identifier identifier loweringStageBody?
    ;

loweringStageBody
    : LBRACE loweringStageEntry* RBRACE
    | LPAREN loweringArgumentList? RPAREN
    ;

loweringStageEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * LOWERING PIPELINE
 * ============================================================================
 *
 * A pipeline expresses an ordered semantic relationship between lowering
 * stages.
 *
 * The grammar does NOT impose:
 *
 *     a fixed number of stages;
 *     a fixed ordering for every program;
 *     a fixed backend pipeline;
 *     a fixed optimization sequence.
 *
 * Semantic validation determines whether a requested pipeline is legal.
 * ============================================================================
 */

loweringPipelineDeclaration
    : identifier identifier LBRACE loweringPipelineItem* RBRACE
    ;

loweringPipelineItem
    : loweringPipelineStage
    | loweringPipelineRelation
    | loweringProperty
    ;

loweringPipelineStage
    : identifier
    | qualifiedIdentifier
    ;

loweringPipelineRelation
    : loweringPipelineStage ARROW loweringPipelineStage
    ;


/*
 * ============================================================================
 * LOWERING POLICY
 * ============================================================================
 *
 * A policy specifies how lowering should behave without becoming an
 * implementation algorithm.
 *
 * Examples:
 *
 *     preserve-semantics
 *     preserve-observability
 *     deterministic
 *     portable
 *     reproducible
 *     capability-aware
 *     resource-aware
 *
 * The actual policy vocabulary is owned by semantic validation.
 * ============================================================================
 */

loweringPolicyDeclaration
    : identifier loweringPolicyBody
    ;

loweringPolicyBody
    : expression
    | LBRACE loweringPolicyEntry* RBRACE
    ;

loweringPolicyEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * LOWERING PROPERTY
 * ============================================================================
 *
 * Generic property syntax is deliberately based on the canonical expression
 * grammar.
 *
 * This prevents lowering.g4 from creating a second value system.
 * ============================================================================
 */

loweringProperty
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    | identifier expression SEMICOLON
    ;


/*
 * ============================================================================
 * LOWERING ARGUMENTS
 * ============================================================================
 */

loweringArgumentList
    : loweringArgument
      (COMMA loweringArgument)*
    ;

loweringArgument
    : identifier ASSIGN expression
    | expression
    ;


/*
 * ============================================================================
 * REPRESENTATION INTENT
 * ============================================================================
 *
 * A representation identifies a semantic or compilation representation.
 *
 * It is NOT a machine architecture.
 *
 * It is NOT a physical device.
 *
 * It is NOT a hardware configuration.
 *
 * Representation names remain extensible.
 * ============================================================================
 */

loweringRepresentation
    : identifier
    | qualifiedIdentifier
    | STRING
    | expression
    ;

loweringRepresentationList
    : loweringRepresentation
      (COMMA loweringRepresentation)*
    ;


/*
 * ============================================================================
 * LOWERING SOURCE / INPUT
 * ============================================================================
 *
 * The source of a lowering operation may be identified symbolically.
 *
 * The grammar does not load files, access the network, or inspect external
 * resources.
 * ============================================================================
 */

loweringSource
    : identifier
    | qualifiedIdentifier
    | expression
    ;

loweringSourceList
    : loweringSource
      (COMMA loweringSource)*
    ;


/*
 * ============================================================================
 * LOWERING DESTINATION
 * ============================================================================
 *
 * A destination is a semantic representation or named compilation boundary.
 *
 * It is intentionally NOT a physical machine address or device identifier.
 * ============================================================================
 */

loweringDestination
    : identifier
    | qualifiedIdentifier
    | expression
    ;

loweringDestinationList
    : loweringDestination
      (COMMA loweringDestination)*
    ;


/*
 * ============================================================================
 * SEMANTIC-PRESERVATION INTENT
 * ============================================================================
 *
 * Lowering must not silently change program meaning.
 *
 * This grammar permits source-level declarations of preservation intent.
 *
 * The compiler's semantic verifier determines whether a requested lowering
 * actually satisfies the declared preservation contract.
 * ============================================================================
 */

loweringPreservation
    : identifier loweringPreservationBody?
    ;

loweringPreservationBody
    : expression
    | LBRACE loweringPreservationEntry* RBRACE
    ;

loweringPreservationEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * PROVENANCE INTENT
 * ============================================================================
 *
 * Lowering can change representations substantially.
 *
 * Provenance metadata permits tooling and verification systems to preserve
 * relationships between source semantics and generated representations.
 *
 * This grammar describes metadata intent only.
 *
 * It does not generate hashes, timestamps, signatures, certificates, or
 * external provenance records.
 * ============================================================================
 */

loweringProvenance
    : identifier loweringProvenanceBody?
    ;

loweringProvenanceBody
    : expression
    | LBRACE loweringProvenanceEntry* RBRACE
    ;

loweringProvenanceEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * PRESERVATION / VERIFICATION REQUIREMENTS
 * ============================================================================
 *
 * These declarations express properties that downstream semantic verification
 * must establish.
 *
 * The grammar itself does not perform verification.
 * ============================================================================
 */

loweringVerification
    : identifier loweringVerificationBody
    ;

loweringVerificationBody
    : expression
    | LBRACE loweringVerificationEntry* RBRACE
    ;

loweringVerificationEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * LOWERING RELATION
 * ============================================================================
 *
 * A relation describes a transformation relationship between representations.
 *
 * Example semantic model:
 *
 *     source_representation -> lowered_representation
 *
 * The grammar does not execute the transformation.
 * ============================================================================
 */

loweringRelation
    : loweringSource ARROW loweringDestination
    ;


/*
 * ============================================================================
 * LOWERING OPTIONS
 * ============================================================================
 *
 * Options are semantic intent, not implementation parameters.
 *
 * They may express:
 *
 *     preservation requirements
 *     portability preferences
 *     determinism requirements
 *     representation preferences
 *     capability requirements
 *     resource constraints
 *
 * Target-specific properties must be resolved through the target/resource
 * systems rather than embedded here.
 * ============================================================================
 */

loweringOptions
    : LBRACE loweringOption* RBRACE
    ;

loweringOption
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * LOWERING CHAIN
 * ============================================================================
 *
 * A chain provides an ordered sequence of symbolic lowering boundaries.
 *
 * There is deliberately no finite maximum.
 *
 * Program size and chain length are bounded only by implementation resources,
 * parser/runtime safety policies, and semantic limits defined outside this
 * grammar.
 * ============================================================================
 */

loweringChain
    : loweringChainItem
      (ARROW loweringChainItem)*
    ;

loweringChainItem
    : identifier
    | qualifiedIdentifier
    | STRING
    | expression
    ;


/*
 * ============================================================================
 * LOWERING CONDITION
 * ============================================================================
 *
 * Conditions are expressions.
 *
 * The semantic layer determines whether a condition is valid for the
 * requested compilation phase.
 *
 * This rule must not duplicate conditional-compilation semantics.
 * ============================================================================
 */

loweringCondition
    : expression
    ;


/*
 * ============================================================================
 * LOWERING CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * Capability references are symbolic.
 *
 * A capability is not a concrete device.
 *
 * Hardware discovery and capability resolution happen downstream.
 * ============================================================================
 */

loweringCapabilityRequirement
    : identifier
    | qualifiedIdentifier
    | expression
    ;

loweringCapabilityRequirements
    : loweringCapabilityRequirement
      (COMMA loweringCapabilityRequirement)*
    ;


/*
 * ============================================================================
 * LOWERING RESOURCE REQUIREMENT
 * ============================================================================
 *
 * Resource requirements are expressions.
 *
 * No resource count is hard-coded.
 *
 * Examples of semantic resource concepts may include:
 *
 *     memory
 *     compute
 *     parallelism
 *     quantum capacity
 *     accelerator capacity
 *     bandwidth
 *     latency
 *     energy
 *
 * Units and feasibility belong to the resource system.
 * ============================================================================
 */

loweringResourceRequirement
    : identifier ASSIGN expression
    | identifier COLON expression
    ;

loweringResourceRequirements
    : loweringResourceRequirement
      (COMMA loweringResourceRequirement)*
    ;


/*
 * ============================================================================
 * LOWERING CONSTRAINT
 * ============================================================================
 *
 * Constraints restrict legal realizations.
 *
 * They must not silently select a physical device.
 * ============================================================================
 */

loweringConstraint
    : identifier comparisonOperator expression
    | identifier COLON expression
    | identifier ASSIGN expression
    ;

loweringConstraints
    : loweringConstraint
      (COMMA loweringConstraint)*
    ;


/*
 * ============================================================================
 * LOWERING PREFERENCE
 * ============================================================================
 *
 * Preferences provide non-mandatory guidance.
 *
 * They may be ignored when satisfying them would violate stronger semantic
 * requirements or platform constraints.
 * ============================================================================
 */

loweringPreference
    : identifier ASSIGN expression
    | identifier COLON expression
    ;

loweringPreferences
    : loweringPreference
      (COMMA loweringPreference)*
    ;


/*
 * ============================================================================
 * LOWERING HINT
 * ============================================================================
 *
 * Hints are optional.
 *
 * A compiler/backend MUST NOT reject a semantically valid program merely
 * because an optional hint cannot be honored, unless the program explicitly
 * promoted that hint into a requirement through another semantic construct.
 * ============================================================================
 */

loweringHint
    : identifier ASSIGN expression
    | identifier COLON expression
    ;

loweringHints
    : loweringHint
      (COMMA loweringHint)*
    ;


/*
 * ============================================================================
 * LOWERING TRANSFORMATION
 * ============================================================================
 *
 * A transformation identifies a named semantic transformation.
 *
 * This does not implement the transformation.
 *
 * Examples may eventually include:
 *
 *     representation conversion
 *     domain lowering
 *     abstraction reduction
 *     dialect conversion
 *     interface adaptation
 *
 * The grammar does not enumerate those transformations.
 * ============================================================================
 */

loweringTransformation
    : identifier
    | qualifiedIdentifier
    | STRING
    ;

loweringTransformationList
    : loweringTransformation
      (COMMA loweringTransformation)*
    ;


/*
 * ============================================================================
 * LOWERING DECLARATION BODY
 * ============================================================================
 *
 * This reusable body permits future extensions without introducing another
 * parallel configuration language.
 * ============================================================================
 */

loweringDeclarationBody
    : LBRACE loweringDeclarationItem* RBRACE
    ;

loweringDeclarationItem
    : loweringProperty
    | loweringRelation SEMICOLON
    | loweringPreservation
    | loweringProvenance
    | loweringVerification
    | loweringOptions
    ;


/*
 * ============================================================================
 * LOWERING SPECIFICATION
 * ============================================================================
 *
 * A complete symbolic lowering specification.
 *
 * This is a syntax-level aggregate only.
 *
 * Semantic analysis determines:
 *
 *   - whether the source representation exists;
 *   - whether the destination exists;
 *   - whether the transformation is legal;
 *   - whether required capabilities exist;
 *   - whether resource requirements can be satisfied;
 *   - whether semantic preservation can be established;
 *   - whether the selected compilation path is compatible with the target.
 * ============================================================================
 */

loweringSpecification
    : loweringSource ARROW loweringDestination
      loweringSpecificationBody?
    ;

loweringSpecificationBody
    : LBRACE loweringSpecificationItem* RBRACE
    ;

loweringSpecificationItem
    : loweringProperty
    | loweringPreservation
    | loweringProvenance
    | loweringVerification
    | loweringOptions
    ;


/*
 * ============================================================================
 * LOWERING PATH
 * ============================================================================
 *
 * A lowering path can contain an arbitrary number of semantic boundaries.
 *
 * There is no fixed machine-dependent length.
 * ============================================================================
 */

loweringPath
    : loweringChain
    ;


/*
 * ============================================================================
 * LOWERING PLAN REFERENCE
 * ============================================================================
 *
 * A named plan can be referenced without embedding its implementation.
 * ============================================================================
 */

loweringPlanReference
    : identifier
    | qualifiedIdentifier
    | STRING
    ;


/*
 * ============================================================================
 * LOWERING PROFILE REFERENCE
 * ============================================================================
 *
 * Profiles belong to compilation semantics/configuration.
 *
 * This grammar only represents the reference.
 * ============================================================================
 */

loweringProfileReference
    : identifier
    | qualifiedIdentifier
    | STRING
    ;


/*
 * ============================================================================
 * COMPARISON OPERATORS
 * ============================================================================
 *
 * Uses the canonical lexer vocabulary.
 *
 * No new operator vocabulary is introduced here.
 * ============================================================================
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
 * SHARED NAME INTEGRATION
 * ============================================================================
 *
 * IMPORTANT:
 *
 * These are NOT local identifier definitions.
 *
 * The canonical parser assembly MUST provide:
 *
 *     identifier
 *     qualifiedIdentifier
 *     expression
 *
 * through the core/names/expressions grammar imports.
 *
 * This parser fragment deliberately references those shared rules instead of
 * defining duplicate aliases.
 *
 * If the grammar assembly uses ANTLR grammar imports, this file must be
 * imported after the grammar that owns those rules.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * NON-OWNERSHIP OF TARGETS
 * ============================================================================
 *
 * `target.g4` remains the owner of target declarations and target semantics.
 *
 * lowering.g4 may reference target-related expressions indirectly through:
 *
 *     expression
 *     identifier
 *     qualifiedIdentifier
 *
 * It must not reproduce target grammar.
 *
 * Therefore this file intentionally does NOT define:
 *
 *     target
 *     architecture
 *     cpu
 *     gpu
 *     qpu
 *     fpga
 *     asic
 *     device
 *     topology
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * NON-OWNERSHIP OF CODE GENERATION
 * ============================================================================
 *
 * `code-generation.g4` owns code-generation intent.
 *
 * lowering.g4 owns transformation/lowering intent.
 *
 * Neither file may absorb the other's semantic domain.
 *
 * A lowering request may reference a representation that code generation
 * eventually emits, but lowering does not define emission syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * NON-OWNERSHIP OF OPTIMIZATION
 * ============================================================================
 *
 * `optimization.g4` owns optimization intent.
 *
 * Lowering may occur after optimization or between semantic representations,
 * but lowering.g4 does not define optimization passes.
 *
 * This prevents:
 *
 *     lowering == optimization
 *
 * from becoming an architectural ambiguity.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * NON-OWNERSHIP OF CONDITIONAL COMPILATION
 * ============================================================================
 *
 * `conditional-compilation.g4` owns conditional compilation.
 *
 * loweringCondition is only an expression-level predicate reference.
 *
 * It does not create a second conditional-compilation mechanism.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * NON-OWNERSHIP OF FEATURE SELECTION
 * ============================================================================
 *
 * `feature-selection.g4` owns feature-selection semantics.
 *
 * Lowering may consume feature-derived semantic information downstream but
 * does not define feature selection itself.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum syntax belongs to grammar/quantum/.
 *
 * Quantum semantic lowering belongs downstream to the canonical quantum
 * representation.
 *
 * In particular:
 *
 *     Zamani quantum syntax
 *          |
 *          v
 *     quantum frontend
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization / routing / scheduling / QEC / ZQN / hardware lowering
 *
 * lowering.g4 MUST NOT introduce:
 *
 *     qubit
 *     gate
 *     circuit
 *     measurement
 *     physical qubit
 *     QEC code
 *     noise channel
 *
 * as duplicate grammar ownership.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware grammars describe their respective source-level semantics.
 *
 * lowering.g4 can express that an HDL/hardware representation participates in
 * a lowering path, but does not redefine ports, signals, clocks, devices,
 * topology, placement, or timing semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CLASSICAL / AI / DATA / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Lowering remains domain-neutral.
 *
 * The same lowering model can therefore describe transformations involving:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     future domains
 *
 * without creating one lowering language per domain.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST generated from these rules should preserve:
 *
 *     - source span;
 *     - declaration kind;
 *     - source representation;
 *     - destination representation;
 *     - transformation identity;
 *     - ordered lowering stages;
 *     - explicit properties;
 *     - expressions;
 *     - preservation requirements;
 *     - provenance intent;
 *     - verification intent;
 *     - explicit-vs-default information;
 *     - source ordering;
 *     - annotations/metadata where supplied by canonical grammar.
 *
 * The AST MUST NOT contain:
 *
 *     - physical device handles;
 *     - machine instructions;
 *     - runtime state;
 *     - scheduler state;
 *     - hardware addresses;
 *     - compiler backend objects;
 *     - mutable global compiler state.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must validate:
 *
 *     1. referenced representations;
 *     2. lowering-stage existence;
 *     3. transformation compatibility;
 *     4. source/destination compatibility;
 *     5. semantic preservation requirements;
 *     6. provenance requirements;
 *     7. verification requirements;
 *     8. capability requirements;
 *     9. resource requirements;
 *    10. target compatibility;
 *    11. policy compatibility;
 *    12. deterministic/reproducible requirements;
 *    13. conflicting lowering declarations;
 *    14. unsupported lowering paths.
 *
 * Semantic validation MUST reject impossible or contradictory requests rather
 * than silently reinterpret them.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * lowering.g4 does NOT define a LoweringIR.
 *
 * Source constructs should lower into the repository's canonical semantic
 * representation and associated compilation-plan metadata.
 *
 * If the compiler requires an internal lowering-plan data structure, that
 * structure belongs to compiler/semantic/IR infrastructure rather than this
 * grammar.
 *
 * Quantum semantics continue through `quantum::ir`.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages consuming this grammar must be able to:
 *
 *     parse
 *       ->
 *     build AST
 *       ->
 *     validate semantics
 *       ->
 *     construct canonical semantic representation
 *       ->
 *     construct lowering plan
 *       ->
 *     optimize / schedule / route / analyze resources
 *       ->
 *     perform target-independent lowering
 *       ->
 *     perform target-specific lowering
 *       ->
 *     generate artifacts
 *
 * The grammar must never invoke these stages.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime MUST NOT depend on the grammar to execute an already compiled
 * semantic representation.
 *
 * Runtime may consume artifacts and runtime metadata produced downstream.
 *
 * Runtime capability discovery MUST NOT be encoded into parser behavior.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * Formatter, syntax highlighter, IDE tooling, documentation generators,
 * language servers, and static analyzers may consume the parser AST.
 *
 * They must not need to instantiate compiler backends merely to understand
 * lowering syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing this grammar is deterministic with respect to:
 *
 *     - source text;
 *     - lexer version;
 *     - grammar version;
 *     - parser configuration.
 *
 * This grammar introduces no:
 *
 *     - network access;
 *     - filesystem discovery;
 *     - time;
 *     - randomness;
 *     - environment-variable lookup;
 *     - hardware probing.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is intentionally no grammar-level maximum for:
 *
 *     lowering stages;
 *     lowering paths;
 *     transformations;
 *     properties;
 *     arguments;
 *     representations;
 *     declarations;
 *     source size.
 *
 * Any implementation limit must be explicit and belong to the appropriate
 * parser/compiler resource-limit subsystem.
 *
 * Such limits MUST NOT become semantic language limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_LOWERING_STAGES
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTERS
 *     MAX_VECTOR_WIDTH
 *
 * Also forbidden:
 *
 *     fixed ISA names as grammar requirements;
 *     fixed physical addresses;
 *     fixed device identifiers;
 *     fixed hardware topology;
 *     fixed quantum topology;
 *     fixed deployment topology.
 *
 * If such a limitation is required by an implementation, it belongs in a
 * resource/limits subsystem and must not be confused with language semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Future grammar changes MUST preserve the semantic distinction between:
 *
 *     lowering
 *     optimization
 *     target selection
 *     code generation
 *     execution
 *
 * A new lowering technology should preferably be introduced as a semantic
 * identifier or qualified name rather than by adding a new reserved keyword.
 *
 * Promotion of an identifier to a reserved lexer token requires:
 *
 *     - language-version documentation;
 *     - compatibility analysis;
 *     - migration guidance;
 *     - lexer/parser regression tests.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests:
 *
 *     - minimal lowering declaration;
 *     - symbolic source/destination;
 *     - qualified representations;
 *     - arbitrary lowering chains;
 *     - named stages;
 *     - pipeline relationships;
 *     - preservation requirements;
 *     - provenance declarations;
 *     - verification declarations;
 *     - policies;
 *     - properties;
 *     - expressions;
 *     - capability requirements;
 *     - resource expressions;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - domain-neutral transformations.
 *
 * Required negative tests:
 *
 *     - malformed lowering declaration;
 *     - malformed stage;
 *     - malformed relation;
 *     - missing expression;
 *     - malformed property;
 *     - malformed argument list;
 *     - malformed constraint;
 *     - malformed pipeline;
 *     - invalid punctuation;
 *     - invalid operator.
 *
 * Required semantic-negative tests:
 *
 *     - unknown representation;
 *     - incompatible source/destination;
 *     - impossible preservation requirement;
 *     - contradictory requirements;
 *     - target conflict;
 *     - unsupported transformation;
 *     - unsatisfied capability;
 *     - unsatisfied resource requirement.
 *
 * Required scalability tests:
 *
 *     - many lowering stages;
 *     - long lowering chains;
 *     - deeply nested property structures;
 *     - large expressions;
 *     - large transformation lists;
 *     - large source programs.
 *
 * Required cross-domain tests:
 *
 *     classical -> classical
 *     quantum -> quantum
 *     classical -> quantum
 *     quantum -> classical
 *     quantum -> HDL
 *     HDL -> hardware
 *     classical -> accelerator
 *     AI -> accelerator
 *     distributed -> target-independent
 *     hybrid -> target-specific
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *   [ ] It compiles with the repository's ANTLR toolchain.
 *
 *   [ ] It uses the canonical Zamani lexer.
 *
 *   [ ] It imports/references canonical identifier and expression rules
 *       through the repository's parser assembly rather than redefining them.
 *
 *   [ ] It introduces no duplicate IR.
 *
 *   [ ] It does not duplicate target.g4.
 *
 *   [ ] It does not duplicate optimization.g4.
 *
 *   [ ] It does not duplicate feature-selection.g4.
 *
 *   [ ] It does not duplicate conditional-compilation.g4.
 *
 *   [ ] It does not duplicate code-generation.g4.
 *
 *   [ ] It does not duplicate quantum grammar ownership.
 *
 *   [ ] It contains no hardware-size assumptions.
 *
 *   [ ] It contains no fixed resource limits.
 *
 *   [ ] It contains no machine-specific identifiers.
 *
 *   [ ] It performs no semantic actions.
 *
 *   [ ] It performs no filesystem access.
 *
 *   [ ] It performs no network access.
 *
 *   [ ] It performs no runtime execution.
 *
 *   [ ] Positive parser tests pass.
 *
 *   [ ] Negative parser tests pass.
 *
 *   [ ] Boundary tests pass.
 *
 *   [ ] Cross-domain tests pass.
 *
 *   [ ] Scalability tests pass.
 *
 *   [ ] Determinism tests pass.
 *
 *   [ ] AST lowering preserves explicit source intent.
 *
 *   [ ] Semantic analysis can distinguish requirements, constraints,
 *       preferences and hints.
 *
 *   [ ] Quantum constructs remain represented by the canonical quantum IR.
 *
 *   [ ] Target-specific decisions remain outside this grammar.
 *
 *   [ ] The resulting lowering model can accommodate future computational
 *       paradigms without requiring a new grammar architecture.
 *
 * ============================================================================
 */