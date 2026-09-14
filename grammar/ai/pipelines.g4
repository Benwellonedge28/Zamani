/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/pipelines.g4
 *
 * Purpose:
 *     Production-ready source grammar for portable AI/ML computation
 *     pipelines.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4
 *
 * Runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021 edition
 *
 * Safety:
 *     This is parser grammar only.
 *     No embedded Rust actions.
 *     No unsafe code.
 *     No filesystem access.
 *     No network access.
 *     No runtime execution.
 *     No hardware discovery.
 *     No machine-specific assumptions.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * This grammar describes the semantic structure of an AI/ML pipeline.
 *
 * It does NOT decide:
 *
 *     - which CPU executes a stage;
 *     - which GPU executes a stage;
 *     - which QPU executes a stage;
 *     - which FPGA executes a stage;
 *     - which ASIC executes a stage;
 *     - how many workers exist;
 *     - how many machines exist;
 *     - how stages are scheduled;
 *     - how stages are physically placed;
 *     - how data is physically transported;
 *     - how models are trained;
 *     - how tensors are represented internally;
 *     - how quantum programs are represented internally;
 *     - how hardware is discovered;
 *     - how resources are allocated.
 *
 * Those responsibilities belong to downstream semantic/compiler/runtime
 * subsystems.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Pipeline source describes:
 *
 *     computation
 *     dependencies
 *     data flow
 *     control flow
 *     semantic requirements
 *     capabilities
 *     constraints
 *     preferences
 *     portability intent
 *
 * It does not permanently bind those semantics to a particular machine.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *
 *     pipeline declarations
 *     pipeline references
 *     pipeline invocation syntax
 *     pipeline stages
 *     pipeline dependencies
 *     pipeline inputs
 *     pipeline outputs
 *     pipeline parameters
 *     pipeline locals
 *     branches
 *     joins
 *     streams
 *     batches
 *     windows
 *     checkpoints
 *     provenance intent
 *     reproducibility intent
 *     pipeline requirements
 *     pipeline capabilities
 *     pipeline constraints
 *     pipeline preferences
 *     nested pipeline regions
 *     extensible pipeline operations
 *
 * DOES NOT OWN:
 *
 *     lexer definitions
 *     keywords
 *     identifiers
 *     primitive types
 *     general expressions
 *     general statements
 *     models
 *     datasets
 *     tensors
 *     training algorithms
 *     inference algorithms
 *     differentiation
 *     optimizers
 *     classical IR
 *     quantum::ir
 *     QEC
 *     ZQN
 *     hardware discovery
 *     topology
 *     routing
 *     scheduling
 *     runtime execution
 *     simulation
 *     resource allocation
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Canonical lexical layer:
 *
 *     ZamaniLexer
 *
 * Canonical type layer:
 *
 *     grammar/types/*
 *
 * Canonical expression layer:
 *
 *     grammar/expressions/*
 *
 * Canonical statement layer:
 *
 *     grammar/statements/*
 *
 * AI-domain consumers:
 *
 *     grammar/ai/ai.g4
 *     grammar/ai/models.g4
 *     grammar/ai/datasets.g4
 *     grammar/ai/training.g4
 *     grammar/ai/inference.g4
 *     grammar/ai/agents.g4
 *
 * The public boundary exported by this file is:
 *
 *     pipelineConstruct
 *
 * The semantic frontend converts the resulting parse tree into the Zamani
 * frontend AST. The compiler then performs semantic lowering.
 *
 * Quantum stages must ultimately cross the canonical quantum semantic boundary
 * and use quantum::ir. This grammar never defines a second quantum IR.
 *
 * Hardware stages must cross the hardware abstraction boundary.
 *
 * Scheduling belongs to the scheduling subsystem.
 *
 * Optimization belongs to the optimization subsystem.
 *
 * Resource discovery belongs to the resource/hardware layers.
 *
 * Runtime dispatch belongs to execution/runtime.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are intentionally NO grammar-level constants for:
 *
 *     maximum stages
 *     maximum workers
 *     maximum devices
 *     maximum nodes
 *     maximum tensors
 *     maximum dimensions
 *     maximum batches
 *     maximum streams
 *     maximum pipeline depth
 *     maximum inputs
 *     maximum outputs
 *     maximum edges
 *
 * Repetition is represented by grammar repetition operators.
 *
 * Practical limits are implementation/resource-policy concerns, not language
 * semantics.
 *
 * ============================================================================
 */

parser grammar AIPipelines;

options {
    tokenVocab = ZamaniLexer;
}

import Types, Expressions;


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * Stable integration boundary.
 *
 * The canonical AI grammar should delegate pipeline syntax to this rule.
 */
pipelineConstruct
    : pipelineDeclaration
    | pipelineReference
    | pipelineInvocation
    | pipelineRegion
    ;


/* ============================================================================
 * PIPELINE DECLARATION
 * ========================================================================== */

/*
 * Example conceptual forms:
 *
 *     @pipeline inference {
 *         ...
 *     }
 *
 *     @pipeline inference: SomeType = value;
 *
 * Annotation semantics are validated downstream.
 */
pipelineDeclaration
    : pipelineAnnotation
      identifier
      pipelineGenericParameters?
      pipelineTypeClause?
      pipelineInitializer?
      pipelineBody?
      SEMICOLON?
    ;


pipelineAnnotation
    : NANO_ANNOTATION
    ;


pipelineGenericParameters
    : genericParameterList
    ;


pipelineTypeClause
    : COLON
      typeExpression
    ;


pipelineInitializer
    : ASSIGN
      expression
    ;


pipelineBody
    : LBRACE
      pipelineMember*
      RBRACE
    ;


/* ============================================================================
 * PIPELINE MEMBERS
 * ========================================================================== */

pipelineMember
    : pipelineInput
    | pipelineOutput
    | pipelineParameter
    | pipelineLocal
    | pipelineStage
    | pipelineEdge
    | pipelineBranch
    | pipelineJoin
    | pipelineStream
    | pipelineBatch
    | pipelineWindow
    | pipelineCheckpoint
    | pipelineRequirement
    | pipelineCapability
    | pipelineConstraint
    | pipelinePreference
    | pipelineProvenance
    | pipelineReproducibility
    | pipelineMetadata
    | pipelineOperation
    | pipelineNestedRegion
    | pipelineStatement
    ;


/* ============================================================================
 * INPUTS
 * ========================================================================== */

pipelineInput
    : roleAnnotation
      identifier
      pipelineValueType?
      pipelineValueInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * OUTPUTS
 * ========================================================================== */

pipelineOutput
    : roleAnnotation
      identifier
      pipelineValueType?
      pipelineValueInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * PARAMETERS
 * ========================================================================== */

pipelineParameter
    : roleAnnotation
      identifier
      pipelineValueType?
      pipelineValueInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * LOCAL VALUES
 * ========================================================================== */

pipelineLocal
    : roleAnnotation
      identifier
      pipelineValueType?
      pipelineValueInitializer?
      SEMICOLON
    ;


pipelineValueType
    : COLON
      typeExpression
    ;


pipelineValueInitializer
    : ASSIGN
      expression
    ;


/* ============================================================================
 * PIPELINE STAGES
 * ========================================================================== */

/*
 * A stage is a semantic computation boundary.
 *
 * A stage may eventually represent:
 *
 *     classical computation
 *     AI/ML model execution
 *     dataset transformation
 *     tensor computation
 *     quantum computation
 *     accelerator computation
 *     HDL/hardware computation
 *     distributed computation
 *     foreign computation
 *     future Zamani computation
 *
 * This grammar does not decide which one.
 */
pipelineStage
    : roleAnnotation
      identifier
      pipelineStageType?
      pipelineStageInitializer?
      pipelineStageBody?
      SEMICOLON?
    ;


pipelineStageType
    : COLON
      typeExpression
    ;


pipelineStageInitializer
    : ASSIGN
      expression
    ;


pipelineStageBody
    : LBRACE
      pipelineStageMember*
      RBRACE
    ;


pipelineStageMember
    : pipelineStageInput
    | pipelineStageOutput
    | pipelineStageParameter
    | pipelineStageRequirement
    | pipelineStageCapability
    | pipelineStageConstraint
    | pipelineStagePreference
    | pipelineStageOperation
    | pipelineStatement
    ;


pipelineStageInput
    : roleAnnotation
      identifier
      pipelineValueType?
      pipelineValueInitializer?
      SEMICOLON
    ;


pipelineStageOutput
    : roleAnnotation
      identifier
      pipelineValueType?
      pipelineValueInitializer?
      SEMICOLON
    ;


pipelineStageParameter
    : roleAnnotation
      identifier
      pipelineValueType?
      pipelineValueInitializer?
      SEMICOLON
    ;


pipelineStageRequirement
    : roleAnnotation
      pipelineClauseTarget?
      ASSIGN?
      expression
      SEMICOLON
    ;


pipelineStageCapability
    : roleAnnotation
      pipelineClauseTarget?
      ASSIGN?
      expression
      SEMICOLON
    ;


pipelineStageConstraint
    : roleAnnotation
      pipelineClauseTarget?
      ASSIGN?
      expression
      SEMICOLON
    ;


pipelineStagePreference
    : roleAnnotation
      pipelineClauseTarget?
      ASSIGN?
      expression
      SEMICOLON
    ;


pipelineStageOperation
    : roleAnnotation
      pipelineClauseTarget?
      pipelineOperationPayload?
      SEMICOLON
    ;


/* ============================================================================
 * DEPENDENCY EDGES
 * ========================================================================== */

/*
 * An edge expresses semantic dependency/data flow.
 *
 * Example:
 *
 *     @edge preprocess -> inference;
 *
 * This is NOT a physical network link.
 *
 * It does not represent:
 *
 *     CPU topology
 *     GPU topology
 *     QPU connectivity
 *     FPGA routing
 *     network topology
 *     scheduler reservations
 */
pipelineEdge
    : roleAnnotation
      pipelineEndpoint
      THIN_ARROW
      pipelineEndpoint
      pipelineEdgePayload?
      SEMICOLON
    ;


pipelineEndpoint
    : qualifiedPipelineReference
    ;


qualifiedPipelineReference
    : identifier
      (
          DOT identifier
        | DOUBLE_COLON identifier
      )*
    ;


pipelineEdgePayload
    : ASSIGN
      expression
    | pipelineClauseBody
    ;


/* ============================================================================
 * BRANCHING
 * ========================================================================== */

pipelineBranch
    : roleAnnotation
      pipelineClauseTarget?
      pipelineBranchCondition?
      pipelineClauseBody
    ;


pipelineBranchCondition
    : ASSIGN
      expression
    ;


/* ============================================================================
 * JOINING
 * ========================================================================== */

pipelineJoin
    : roleAnnotation
      pipelineClauseTarget?
      pipelineJoinInputs?
      pipelineClauseBody?
      SEMICOLON?
    ;


pipelineJoinInputs
    : LPAREN
      optionalExpressionList
      RPAREN
    ;


/* ============================================================================
 * STREAMING
 * ========================================================================== */

/*
 * Streaming describes semantic streaming intent.
 *
 * It does not select:
 *
 *     network transport
 *     queue implementation
 *     buffer size
 *     number of workers
 *     number of machines
 */
pipelineStream
    : roleAnnotation
      pipelineClauseTarget?
      pipelineStreamSource?
      pipelineClauseBody?
      SEMICOLON?
    ;


pipelineStreamSource
    : ASSIGN
      expression
    ;


/* ============================================================================
 * BATCHING
 * ========================================================================== */

/*
 * Batch configuration is expressed through normal expressions.
 *
 * Therefore:
 *
 *     @batch = dynamic_size;
 *
 * does not impose a compile-time machine limit.
 */
pipelineBatch
    : roleAnnotation
      pipelineClauseTarget?
      pipelineBatchExpression?
      pipelineClauseBody?
      SEMICOLON?
    ;


pipelineBatchExpression
    : ASSIGN
      expression
    ;


/* ============================================================================
 * WINDOWING
 * ========================================================================== */

/*
 * Window size, stride, time range, count, etc. are semantic expressions.
 *
 * They are not hard-coded resource limits.
 */
pipelineWindow
    : roleAnnotation
      pipelineClauseTarget?
      pipelineWindowExpression?
      pipelineClauseBody?
      SEMICOLON?
    ;


pipelineWindowExpression
    : ASSIGN
      expression
    ;


/* ============================================================================
 * CHECKPOINTS
 * ========================================================================== */

/*
 * Checkpoint syntax expresses intent.
 *
 * It does NOT guarantee that arbitrary runtime state can be serialized.
 *
 * The semantic/runtime layer decides whether the checkpoint is:
 *
 *     classical state
 *     compiled program state
 *     reconstructible state
 *     measurement boundary
 *     QEC-supported state
 *     provider-supported state
 *     another explicitly supported state
 */
pipelineCheckpoint
    : roleAnnotation
      pipelineClauseTarget?
      pipelineInitializerClause?
      pipelineClauseBody?
      SEMICOLON?
    ;


/* ============================================================================
 * PROVENANCE
 * ========================================================================== */

pipelineProvenance
    : roleAnnotation
      pipelineClauseTarget?
      pipelineInitializerClause?
      pipelineClauseBody?
      SEMICOLON?
    ;


/* ============================================================================
 * REPRODUCIBILITY
 * ========================================================================== */

pipelineReproducibility
    : roleAnnotation
      pipelineClauseTarget?
      pipelineInitializerClause?
      pipelineClauseBody?
      SEMICOLON?
    ;


/* ============================================================================
 * METADATA
 * ========================================================================== */

pipelineMetadata
    : roleAnnotation
      pipelineClauseTarget?
      pipelineInitializerClause?
      pipelineClauseBody?
      SEMICOLON?
    ;


pipelineInitializerClause
    : ASSIGN
      expression
    ;


/* ============================================================================
 * REQUIREMENTS
 * ========================================================================== */

/*
 * Requirement:
 *
 *     "The execution environment must provide X."
 *
 * It does not mean:
 *
 *     "Use device X."
 */
pipelineRequirement
    : roleAnnotation
      pipelineClauseTarget?
      ASSIGN?
      expression
      SEMICOLON
    ;


/* ============================================================================
 * CAPABILITIES
 * ========================================================================== */

/*
 * Capability:
 *
 *     "This pipeline/stage can use or requires awareness of capability X."
 *
 * Actual capability discovery is downstream.
 */
pipelineCapability
    : roleAnnotation
      pipelineClauseTarget?
      ASSIGN?
      expression
      SEMICOLON
    ;


/* ============================================================================
 * CONSTRAINTS
 * ========================================================================== */

/*
 * Constraint:
 *
 *     a semantic condition that must remain satisfied.
 *
 * Constraint checking is semantic/compiler responsibility.
 */
pipelineConstraint
    : roleAnnotation
      pipelineClauseTarget?
      ASSIGN?
      expression
      SEMICOLON
    ;


/* ============================================================================
 * PREFERENCES
 * ========================================================================== */

/*
 * Preference:
 *
 *     an optimization preference rather than a semantic necessity.
 *
 * A preference must never silently become a hard requirement.
 */
pipelinePreference
    : roleAnnotation
      pipelineClauseTarget?
      ASSIGN?
      expression
      SEMICOLON
    ;


/* ============================================================================
 * EXTENSIBLE PIPELINE OPERATIONS
 * ========================================================================== */

/*
 * Generic operation boundary.
 *
 * This permits future AI dialects and repository extensions without requiring
 * pipelines.g4 to know every future operation.
 *
 * Examples of semantic operations could include:
 *
 *     preprocessing
 *     feature extraction
 *     inference
 *     evaluation
 *     transformation
 *     aggregation
 *     embedding
 *     serving
 *     orchestration
 *     quantum-assisted computation
 *     accelerator execution
 *
 * Their semantic meaning is registered and validated outside this grammar.
 */
pipelineOperation
    : roleAnnotation
      pipelineClauseTarget?
      pipelineOperationPayload?
      (
          pipelineClauseBody
        | SEMICOLON
      )
    ;


pipelineOperationPayload
    : ASSIGN
      expression
    | LPAREN
      optionalExpressionList
      RPAREN
    ;


/* ============================================================================
 * NESTED PIPELINES
 * ========================================================================== */

/*
 * Nested regions provide hierarchical composition.
 *
 * No maximum nesting depth is encoded here.
 */
pipelineNestedRegion
    : roleAnnotation
      pipelineClauseTarget?
      pipelineClauseBody
    ;


pipelineRegion
    : LBRACE
      pipelineMember*
      RBRACE
    ;


pipelineClauseBody
    : LBRACE
      pipelineMember*
      RBRACE
    ;


/* ============================================================================
 * REFERENCES
 * ========================================================================== */

pipelineReference
    : qualifiedPipelineReference
    ;


/* ============================================================================
 * INVOCATIONS
 * ========================================================================== */

pipelineInvocation
    : qualifiedPipelineReference
      LPAREN
      optionalExpressionList
      RPAREN
    ;


/* ============================================================================
 * GENERIC CLAUSE TARGET
 * ========================================================================== */

pipelineClauseTarget
    : identifier
    ;


/* ============================================================================
 * ANNOTATION BOUNDARY
 * ========================================================================== */

/*
 * The canonical lexer owns annotation lexical structure.
 *
 * Semantic analysis determines whether a particular annotation is:
 *
 *     @pipeline
 *     @stage
 *     @input
 *     @output
 *     @parameter
 *     @edge
 *     @branch
 *     @join
 *     @stream
 *     @batch
 *     @window
 *     @checkpoint
 *     @requires
 *     @capability
 *     @constraint
 *     @preference
 *     @provenance
 *     @reproducible
 *     or another registered dialect annotation.
 *
 * This avoids creating a second keyword/lexer system inside the AI grammar.
 */
roleAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * PIPELINE STATEMENTS
 * ========================================================================== */

/*
 * Pipeline regions may contain canonical Zamani statements where the semantic
 * model permits statements.
 *
 * Statement ownership remains in grammar/statements.
 */
pipelineStatement
    : statement
    ;


/* ============================================================================
 * PUBLIC HELPER BOUNDARIES
 * ========================================================================== */

pipelineMembers
    : pipelineMember*
    ;


pipelineContents
    : pipelineBody
    ;


pipelineArguments
    : optionalExpressionList
    ;


pipelineType
    : typeExpression
    ;


/* ============================================================================
 * SEMANTIC CONTRACT
 * ========================================================================== */

/*
 * SEMANTIC ANALYSIS MUST:
 *
 *     - normalize annotation names;
 *     - validate annotation categories;
 *     - reject duplicate names where prohibited;
 *     - resolve stage references;
 *     - resolve pipeline references;
 *     - resolve input/output bindings;
 *     - validate type compatibility;
 *     - validate dependency relationships;
 *     - detect invalid cycles;
 *     - distinguish legal streaming feedback from illegal dependency cycles;
 *     - validate branch conditions;
 *     - validate joins;
 *     - validate requirements;
 *     - validate capabilities;
 *     - validate constraints;
 *     - validate preferences;
 *     - validate provenance;
 *     - validate reproducibility declarations;
 *     - validate checkpoint semantics;
 *     - validate cross-domain values;
 *     - preserve source provenance;
 *     - preserve deterministic semantic ordering;
 *     - reject unsupported target-specific assumptions;
 *     - reject accidental resource hard-coding.
 *
 *
 * RESOURCE MODEL:
 *
 * Requirements, capabilities, constraints and preferences are distinct.
 *
 * For example:
 *
 *     requires quantum
 *
 * must NOT automatically mean:
 *
 *     use QPU-X
 *
 * or:
 *
 *     allocate 32 qubits
 *
 * or:
 *
 *     use topology Y
 *
 * Those decisions belong downstream.
 *
 *
 * HARDWARE:
 *
 * A pipeline may semantically request hardware capabilities, but this grammar
 * must never discover or bind hardware.
 *
 *
 * QUANTUM:
 *
 * A pipeline may contain a quantum stage.
 *
 * The grammar does not define quantum operations.
 *
 * Quantum syntax is handled by the quantum grammar/domain and semantic lowering
 * ultimately reaches:
 *
 *     quantum::ir
 *
 * as the canonical quantum semantic boundary.
 *
 *
 * QEC:
 *
 * This grammar may express a semantic requirement or preference involving
 * error-correction capability.
 *
 * It does NOT implement QEC.
 *
 *
 * ZQN:
 *
 * This grammar may express a semantic requirement involving noise/fault
 * awareness.
 *
 * It does NOT define or model ZQN faults.
 *
 *
 * SCHEDULING:
 *
 * Pipeline dependencies can inform scheduling.
 *
 * This grammar does not produce schedules.
 *
 *
 * OPTIMIZATION:
 *
 * Preferences can inform optimization.
 *
 * This grammar does not optimize.
 *
 *
 * RUNTIME:
 *
 * The runtime consumes lowered semantic representations.
 *
 * This grammar never executes a pipeline.
 */


/* ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ========================================================================== */

/*
 * AI + CLASSICAL
 *
 * Classical expressions and values enter through the canonical expression and
 * type systems.
 *
 *
 * AI + QUANTUM
 *
 * A stage can refer to quantum computation semantically without importing the
 * quantum grammar here.
 *
 * This prevents:
 *
 *     pipelines.g4 -> quantum.g4 -> pipelines.g4
 *
 * cycles.
 *
 *
 * AI + HDL / HARDWARE
 *
 * Hardware computation is referenced semantically and lowered through the
 * hardware/HDL domain.
 *
 *
 * AI + DISTRIBUTED
 *
 * Distributed execution is represented as intent/requirements and resolved by
 * distributed execution/resource subsystems.
 *
 *
 * AI + ACCELERATORS
 *
 * Accelerator capability is expressed semantically.
 *
 * Device discovery and placement are downstream.
 *
 *
 * AI + NETWORKING
 *
 * Networking requirements can be expressed as semantic requirements.
 *
 * Network protocols and endpoints remain owned by networking grammar and
 * runtime layers.
 *
 *
 * AI + SECURITY
 *
 * Security requirements can be represented semantically.
 *
 * Identity, cryptography, permissions and trust semantics remain owned by the
 * security domain.
 */


/* ============================================================================
 * NO-CIRCULARITY CONTRACT
 * ========================================================================== */

/*
 * This file MUST NOT import:
 *
 *     AI.g4
 *     models.g4
 *     datasets.g4
 *     training.g4
 *     inference.g4
 *     tensors.g4
 *     quantum/*.g4
 *     hardware/*.g4
 *     distributed/*.g4
 *     networking/*.g4
 *     runtime grammars
 *
 * Instead it imports only canonical shared grammar foundations:
 *
 *     Types
 *     Expressions
 *
 * The canonical AI grammar delegates to:
 *
 *     pipelineConstruct
 *
 * rather than pipelines.g4 importing the canonical AI grammar.
 */


/* ============================================================================
 * DETERMINISM CONTRACT
 * ========================================================================== */

/*
 * The grammar:
 *
 *     does not generate random values;
 *     does not choose devices;
 *     does not choose workers;
 *     does not choose execution order;
 *     does not generate seeds;
 *     does not generate schedules;
 *     does not allocate resources.
 *
 * Identical source must produce equivalent parse structures.
 *
 * Any deterministic execution requirement must be represented explicitly and
 * validated downstream.
 */


/* ============================================================================
 * PROVENANCE CONTRACT
 * ========================================================================== */

/*
 * Source locations, source identity and parse provenance must be retained by
 * the frontend AST.
 *
 * pipelines.g4 itself does not create provenance identifiers.
 *
 * This prevents parser-level identifiers from becoming a second provenance
 * system.
 */


/* ============================================================================
 * VERSIONING CONTRACT
 * ========================================================================== */

/*
 * Pipeline syntax is governed by Zamani language versioning.
 *
 * New pipeline operations should preferably be introduced through:
 *
 *     registered annotations
 *     dialects
 *     versioned semantic contracts
 *
 * rather than by repeatedly introducing machine-specific grammar rules.
 *
 * Deprecated syntax must be handled through the repository's compatibility
 * and migration policy.
 */


/* ============================================================================
 * ERROR CONTRACT
 * ========================================================================== */

/*
 * Syntax errors are handled by ANTLR's parser diagnostics.
 *
 * Semantic errors belong to the frontend/semantic diagnostic system.
 *
 * This grammar MUST NOT encode semantic failures as comments.
 *
 * It MUST NOT silently recover by inventing:
 *
 *     stages
 *     devices
 *     workers
 *     resources
 *     types
 *     dependencies
 *     hardware
 *     values
 */


/* ============================================================================
 * TEST CONTRACT
 * ========================================================================== */

/*
 * Positive tests:
 *
 *     minimal pipeline
 *     typed pipeline
 *     untyped pipeline
 *     pipeline with inputs
 *     pipeline with outputs
 *     pipeline with parameters
 *     pipeline with locals
 *     pipeline with multiple stages
 *     pipeline with multiple edges
 *     pipeline with branch
 *     pipeline with join
 *     pipeline with stream
 *     pipeline with batch
 *     pipeline with window
 *     pipeline with checkpoint
 *     pipeline with provenance
 *     pipeline with reproducibility
 *     pipeline with requirements
 *     pipeline with capabilities
 *     pipeline with constraints
 *     pipeline with preferences
 *     nested pipelines
 *     pipeline invocation
 *     generic pipeline operation
 *
 *
 * Cross-domain positive tests:
 *
 *     classical + AI
 *     AI + quantum
 *     AI + hardware
 *     AI + HDL
 *     AI + distributed
 *     AI + accelerator
 *     AI + networking
 *     AI + security
 *     classical + quantum + AI
 *     quantum + hardware + AI
 *     classical + quantum + hardware + distributed + AI
 *
 *
 * Negative syntax tests:
 *
 *     missing pipeline name
 *     missing annotation
 *     malformed body
 *     malformed edge
 *     missing edge target
 *     malformed invocation
 *     malformed type
 *     malformed expression
 *     malformed branch
 *     malformed join
 *     malformed stream
 *     malformed batch
 *     malformed window
 *
 *
 * Semantic negative tests:
 *
 *     duplicate names
 *     unresolved stage
 *     unresolved pipeline
 *     invalid dependency
 *     invalid type connection
 *     illegal dependency cycle
 *     invalid branch
 *     invalid join
 *     contradictory requirements
 *     impossible constraints
 *     unsupported capabilities
 *     invalid checkpoint semantics
 *     invalid cross-domain value
 *     accidental hardware binding
 *
 *
 * Scalability tests:
 *
 *     arbitrary stage count
 *     arbitrary edge count
 *     arbitrary input count
 *     arbitrary output count
 *     arbitrary nested depth
 *     arbitrary batch expression
 *     arbitrary stream composition
 *     arbitrary window expression
 *
 * Tests must not mistake test-harness memory/time limits for language limits.
 *
 *
 * Determinism tests:
 *
 *     identical source -> identical token sequence
 *     identical source -> equivalent parse tree
 *     repeated parsing -> equivalent AST
 *
 *
 * Round-trip tests:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     canonical printer/serializer
 *       ->
 *     parser
 *
 * Semantic meaning must be preserved.
 */


/* ============================================================================
 * HARD-CODING AUDIT
 * ========================================================================== */

/*
 * Forbidden:
 *
 *     MAX_STAGES
 *     MAX_WORKERS
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_BATCH
 *     MAX_STREAMS
 *     MAX_TENSORS
 *     MAX_DIMENSIONS
 *
 * Forbidden:
 *
 *     device IDs
 *     machine IDs
 *     CPU counts
 *     GPU counts
 *     QPU counts
 *     FPGA counts
 *     fixed topology
 *     physical addresses
 *     network addresses
 *     fixed memory capacities
 *     fixed accelerator counts
 *
 * No such values are encoded by this grammar.
 */


/* ============================================================================
 * COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is complete when ALL of the following are true:
 *
 *     1. The grammar compiles with the canonical ZamaniLexer.
 *
 *     2. Types resolves to the canonical type grammar.
 *
 *     3. Expressions resolves to the canonical expression grammar.
 *
 *     4. pipelineConstruct is the stable public integration boundary.
 *
 *     5. AI.g4 delegates pipeline syntax to pipelineConstruct.
 *
 *     6. No duplicate pipeline grammar exists elsewhere.
 *
 *     7. No circular grammar dependency exists.
 *
 *     8. No machine-specific resource assumptions exist.
 *
 *     9. No fixed stage/device/worker/node limits exist.
 *
 *    10. No quantum IR is created here.
 *
 *    11. quantum::ir remains the canonical quantum semantic boundary.
 *
 *    12. No QEC implementation exists here.
 *
 *    13. No ZQN implementation exists here.
 *
 *    14. No scheduling implementation exists here.
 *
 *    15. No optimization implementation exists here.
 *
 *    16. No hardware discovery exists here.
 *
 *    17. No runtime execution exists here.
 *
 *    18. Cross-domain composition is possible through canonical types and
 *        expressions.
 *
 *    19. Positive tests exist.
 *
 *    20. Negative tests exist.
 *
 *    21. Boundary tests exist.
 *
 *    22. Scalability tests exist.
 *
 *    23. Determinism tests exist.
 *
 *    24. Round-trip tests exist where the repository's printer/serializer
 *        supports them.
 *
 *    25. Semantic diagnostics reject invalid pipeline constructs.
 *
 *    26. Rust-side integration remains compatible with Rust 1.97/1.97.1.
 *
 *    27. No unsafe Rust is required by this grammar.
 *
 *    28. Documentation identifies this file as the owner of pipeline syntax.
 *
 *    29. Future pipeline dialects can extend semantic operation registration
 *        without requiring a machine-specific rewrite of this grammar.
 *
 * ============================================================================
 */