/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/pipelines.g4
 *
 * Status:
 *     Production HDL pipeline parser grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust.
 *     No unsafe Rust is required.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX of hardware pipelines.
 *
 * A Zamani pipeline describes a semantically ordered collection of hardware
 * stages and their relationships.
 *
 * A pipeline may describe:
 *
 *     - streaming computation;
 *     - staged computation;
 *     - dataflow hardware;
 *     - instruction pipelines;
 *     - arithmetic pipelines;
 *     - accelerator pipelines;
 *     - processing pipelines;
 *     - quantum/classical interface pipelines;
 *     - heterogeneous hardware pipelines;
 *     - generated/repeated pipeline structures.
 *
 * The pipeline is an INTENT/SOURCE construct.
 *
 * This grammar does NOT determine:
 *
 *     - physical pipeline depth;
 *     - physical clock frequency;
 *     - FPGA resources;
 *     - ASIC cells;
 *     - DSP count;
 *     - LUT count;
 *     - register count;
 *     - physical placement;
 *     - routing;
 *     - target architecture;
 *     - scheduling decisions;
 *     - timing closure;
 *     - resource allocation;
 *     - machine topology;
 *     - vendor-specific implementation.
 *
 * Those decisions belong to downstream semantic, compilation, scheduling,
 * hardware-abstraction, synthesis, and runtime subsystems.
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
 *     HDL grammar
 *          |
 *          +--> pipelines.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> type analysis
 *          +--> capability analysis
 *          +--> resource requirements
 *          +--> timing semantics
 *          +--> dependency analysis
 *          |
 *          v
 *     canonical hardware/HDL semantic representation
 *          |
 *          +--> optimization
 *          +--> scheduling
 *          +--> synthesis
 *          +--> placement
 *          +--> routing
 *          +--> target lowering
 *          |
 *          v
 *     hardware / runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - pipeline declaration syntax;
 *     - pipeline identity syntax;
 *     - pipeline generic parameters;
 *     - pipeline attributes;
 *     - pipeline-level requirements;
 *     - pipeline-level constraints;
 *     - pipeline-level preferences;
 *     - pipeline ports/interfaces at pipeline syntax level;
 *     - stage declarations;
 *     - stage ordering;
 *     - stage inputs/outputs;
 *     - stage dependencies;
 *     - stage-local latency metadata;
 *     - stage-local initiation metadata;
 *     - stage-local buffering metadata;
 *     - stage-local control metadata;
 *     - stage-local resource intent;
 *     - pipeline connections;
 *     - pipeline boundaries;
 *     - pipeline annotations;
 *     - pipeline repetition/generation syntax;
 *     - pipeline-local semantic structure.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - general expressions;
 *     - general types;
 *     - module declarations;
 *     - ports globally;
 *     - signals globally;
 *     - registers globally;
 *     - memories globally;
 *     - clocks;
 *     - physical timing;
 *     - scheduling;
 *     - placement;
 *     - routing;
 *     - synthesis;
 *     - hardware discovery;
 *     - target selection;
 *     - FPGA resources;
 *     - ASIC resources;
 *     - CPU resources;
 *     - GPU resources;
 *     - quantum-device resources;
 *     - runtime dispatch;
 *     - canonical IR construction.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A pipeline describes a computation structure.
 *
 * It MUST NOT encode accidental machine limits.
 *
 * Therefore this grammar contains no:
 *
 *     MAX_STAGES
 *     MAX_PIPELINE_DEPTH
 *     MAX_LANES
 *     MAX_WIDTH
 *     MAX_INPUTS
 *     MAX_OUTPUTS
 *     MAX_BUFFERS
 *     MAX_RESOURCES
 *     MAX_INSTANCES
 *     MAX_CLOCKS
 *     MAX_FREQUENCY
 *
 * Any number of stages is represented structurally.
 *
 * Resource availability is evaluated later.
 *
 * Consequently the same source-level pipeline can potentially be lowered
 * to:
 *
 *     - a tiny implementation;
 *     - a CPU implementation;
 *     - a GPU implementation;
 *     - an FPGA implementation;
 *     - an ASIC implementation;
 *     - a heterogeneous accelerator;
 *     - a future hardware target.
 *
 * ============================================================================
 * SEMANTIC PRINCIPLE
 * ============================================================================
 *
 * The grammar distinguishes:
 *
 *     REQUIREMENT
 *         A property that must be satisfied.
 *
 *     CONSTRAINT
 *         A property that limits legal implementations.
 *
 *     PREFERENCE
 *         A desired property that may be relaxed.
 *
 *     HINT
 *         Information useful to implementation but not semantically binding.
 *
 *     RESOURCE
 *         A logical resource requirement.
 *
 *     TARGET
 *         A requested execution/implementation domain.
 *
 * Pipeline syntax must not silently convert one category into another.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * It consumes the canonical token vocabulary:
 *
 *     ZamaniTokens
 *
 * through:
 *
 *     options {
 *         tokenVocab=ZamaniTokens;
 *     }
 *
 * Shared HDL rules are expected to be supplied by the importing HDL parser,
 * including rules such as:
 *
 *     identifier
 *     hdlKeyword
 *     hdlExpression
 *     hdlTypeExpression
 *     hdlBlock
 *     hdlAttributes
 *     hdlAttribute
 *     hdlArgumentList
 *     hdlRangeExpression
 *     qualifiedHdlName
 *
 * This file deliberately does not redefine those rules.
 *
 * ============================================================================
 * INTEGRATION WITH hdl.g4
 * ============================================================================
 *
 * Existing hdl.g4 currently contains:
 *
 *     hdlPipelineDeclaration
 *     hdlPipelineStage
 *
 * Those rules are the old inline pipeline implementation.
 *
 * Production integration MUST make pipelines.g4 the sole owner of those
 * rules.
 *
 * hdl.g4 should import/delegate to this grammar and remove its duplicate
 * pipeline definitions.
 *
 * The public entry rule supplied by this file is:
 *
 *     hdlPipelineDeclaration
 *
 * Therefore downstream consumers do not need a second pipeline API.
 *
 * ============================================================================
 * INTEGRATION WITH hardware-modules.g4
 * ============================================================================
 *
 * hardware-modules.g4 may consume:
 *
 *     hdlPipelineDeclaration
 *
 * as a module member.
 *
 * It must not redefine pipeline stages.
 *
 * ============================================================================
 * INTEGRATION WITH OTHER HDL GRAMMARS
 * ============================================================================
 *
 * ports.g4
 *     Supplies port semantics when a pipeline explicitly declares ports.
 *
 * signals.g4
 *     Supplies signal semantics downstream.
 *
 * registers.g4
 *     Supplies register semantics downstream.
 *
 * clocks.g4
 *     Owns clock declarations and clock semantics.
 *
 * timing.g4
 *     Owns hardware timing semantics.
 *
 * processes.g4
 *     Owns process syntax/semantics.
 *
 * memories.g4
 *     Owns memory declarations.
 *
 * hardware-generics.g4
 *     Supplies generic hardware parameter structures.
 *
 * hardware-parameters.g4
 *     Supplies hardware parameter semantics.
 *
 * hardware-interfaces.g4
 *     Supplies interface semantics.
 *
 * None of those grammars should redefine pipeline syntax.
 *
 * ============================================================================
 * NO HARDWARE ASSUMPTIONS
 * ============================================================================
 *
 * These are intentionally NOT grammar concepts:
 *
 *     FPGA_A
 *     GPU_0
 *     QPU_7
 *     CPU_0
 *     DEVICE_1
 *     BRAM_COUNT
 *     DSP_COUNT
 *     LUT_COUNT
 *     REGISTER_COUNT
 *     FIXED_PIPELINE_DEPTH
 *     FIXED_LANE_COUNT
 *
 * Such information belongs to target descriptions, capabilities, resource
 * models, or deployment configuration.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PIPELINE DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     pipeline Compute {
 *         ...
 *     }
 *
 * Generic form:
 *
 *     pipeline Compute<T, Width> {
 *         ...
 *     }
 *
 * Attribute-bearing form:
 *
 *     @streaming
 *     pipeline Compute {
 *         ...
 *     }
 */
hdlPipelineDeclaration
    : hdlAttributes*
      hdlKeyword
      identifier?
      hdlPipelineGenericParameters?
      hdlPipelineHeaderClause*
      LBRACE
      hdlPipelineMember*
      RBRACE
    ;


/*
 * ============================================================================
 * PIPELINE GENERICS
 * ============================================================================
 */

hdlPipelineGenericParameters
    : LT
      hdlPipelineGenericParameterList
      GT
    ;

hdlPipelineGenericParameterList
    : hdlPipelineGenericParameter
      (
          COMMA
          hdlPipelineGenericParameter
      )*
      COMMA?
    ;

hdlPipelineGenericParameter
    : identifier
      (
          COLON
          hdlTypeExpression
      )?
      (
          ASSIGN
          hdlExpression
      )?
    ;


/*
 * ============================================================================
 * PIPELINE HEADER
 * ============================================================================
 *
 * Header clauses describe source-level intent.
 *
 * They do not select physical hardware.
 */

hdlPipelineHeaderClause
    : hdlPipelineRequirementClause
    | hdlPipelineConstraintClause
    | hdlPipelinePreferenceClause
    | hdlPipelineHintClause
    | hdlPipelineInputClause
    | hdlPipelineOutputClause
    | hdlPipelineInterfaceClause
    | hdlPipelinePropertyClause
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 */

hdlPipelineRequirementClause
    : hdlKeyword
      hdlExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 */

hdlPipelineConstraintClause
    : hdlKeyword
      hdlExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 */

hdlPipelinePreferenceClause
    : hdlKeyword
      hdlExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * HINTS
 * ============================================================================
 */

hdlPipelineHintClause
    : hdlKeyword
      hdlExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * PIPELINE INPUTS
 * ============================================================================
 */

hdlPipelineInputClause
    : hdlKeyword
      hdlPipelinePortList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * PIPELINE OUTPUTS
 * ============================================================================
 */

hdlPipelineOutputClause
    : hdlKeyword
      hdlPipelinePortList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * PIPELINE INTERFACES
 * ============================================================================
 */

hdlPipelineInterfaceClause
    : hdlKeyword
      hdlPipelinePortList
      SEMICOLON?
    ;

hdlPipelinePortList
    : hdlPipelinePort
      (
          COMMA
          hdlPipelinePort
      )*
      COMMA?
    ;

hdlPipelinePort
    : identifier
      (
          COLON
          hdlTypeExpression
      )?
      hdlPipelinePortProperty*
    ;

hdlPipelinePortProperty
    : hdlAttribute
    | LBRACKET
      hdlRangeExpression?
      RBRACKET
    ;


/*
 * ============================================================================
 * PIPELINE PROPERTIES
 * ============================================================================
 *
 * These are deliberately generic.
 *
 * The semantic layer determines whether a property is meaningful for the
 * selected implementation.
 */

hdlPipelinePropertyClause
    : hdlKeyword
      (
          identifier
        | hdlExpression
      )
      (
          ASSIGN
          hdlExpression
      )?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * PIPELINE MEMBERS
 * ============================================================================
 */

hdlPipelineMember
    : hdlAttributes*
      (
          hdlPipelineDeclaration
        | hdlPipelineStageDeclaration
        | hdlPipelineConnectionDeclaration
        | hdlPipelineBoundaryDeclaration
        | hdlPipelineGenerateDeclaration
        | hdlPipelinePropertyClause
        | hdlPipelineRequirementClause
        | hdlPipelineConstraintClause
        | hdlPipelinePreferenceClause
        | hdlPipelineHintClause
        | hdlPipelineInputClause
        | hdlPipelineOutputClause
        | hdlPipelineInterfaceClause
        | hdlPipelineLocalDeclaration
        | hdlPipelineAssertion
        | hdlPipelineExpressionStatement
      )
    ;


/*
 * ============================================================================
 * STAGES
 * ============================================================================
 *
 * A stage is a semantic unit of pipeline computation.
 *
 * The grammar does not require stages to correspond one-to-one with physical
 * registers, clock cycles, execution units, or hardware components.
 */

hdlPipelineStageDeclaration
    : hdlKeyword
      identifier?
      hdlPipelineStageHeaderClause*
      LBRACE
      hdlPipelineStageMember*
      RBRACE
    ;


/*
 * ============================================================================
 * STAGE HEADER
 * ============================================================================
 */

hdlPipelineStageHeaderClause
    : hdlPipelineStageInputClause
    | hdlPipelineStageOutputClause
    | hdlPipelineDependencyClause
    | hdlPipelineLatencyClause
    | hdlPipelineInitiationClause
    | hdlPipelineThroughputClause
    | hdlPipelineBufferClause
    | hdlPipelineResourceClause
    | hdlPipelineRequirementClause
    | hdlPipelineConstraintClause
    | hdlPipelinePreferenceClause
    | hdlPipelineHintClause
    | hdlPipelinePropertyClause
    ;


/*
 * ============================================================================
 * STAGE INPUTS
 * ============================================================================
 */

hdlPipelineStageInputClause
    : hdlKeyword
      hdlPipelineReferenceList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * STAGE OUTPUTS
 * ============================================================================
 */

hdlPipelineStageOutputClause
    : hdlKeyword
      hdlPipelineReferenceList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Dependencies describe semantic ordering.
 *
 * They do not prescribe a particular scheduler algorithm.
 */

hdlPipelineDependencyClause
    : hdlKeyword
      hdlPipelineDependencyExpression
      SEMICOLON?
    ;

hdlPipelineDependencyExpression
    : hdlPipelineReference
    | hdlPipelineReference
      (
          COMMA
          hdlPipelineReference
      )*
    ;


/*
 * ============================================================================
 * LATENCY
 * ============================================================================
 *
 * Latency is represented as an expression rather than a fixed integer.
 *
 * This permits:
 *
 *     - compile-time symbolic values;
 *     - parameterized values;
 *     - target-dependent values;
 *     - capability-derived values;
 *     - implementation-selected values.
 *
 * The grammar does not decide whether latency is measured in:
 *
 *     cycles
 *     time
 *     events
 *     transactions
 *
 * That meaning belongs to semantic analysis and the timing model.
 */

hdlPipelineLatencyClause
    : hdlKeyword
      hdlExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * INITIATION INTERVAL
 * ============================================================================
 *
 * The initiation interval is an intent/property of a pipeline stage.
 *
 * It is NOT a promise that every target can satisfy it.
 */

hdlPipelineInitiationClause
    : hdlKeyword
      hdlExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * THROUGHPUT
 * ============================================================================
 */

hdlPipelineThroughputClause
    : hdlKeyword
      hdlExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * BUFFERING
 * ============================================================================
 *
 * Buffer requirements are expressed symbolically.
 *
 * No fixed buffer size is imposed by this grammar.
 */

hdlPipelineBufferClause
    : hdlKeyword
      (
          hdlExpression
        | identifier
      )
      (
          ASSIGN
          hdlExpression
      )?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * RESOURCE INTENT
 * ============================================================================
 *
 * This describes logical resource requirements/preferences.
 *
 * It does not identify physical resources.
 */

hdlPipelineResourceClause
    : hdlKeyword
      hdlExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * STAGE MEMBERS
 * ============================================================================
 */

hdlPipelineStageMember
    : hdlAttributes*
      (
          hdlPipelineStageDeclaration
        | hdlPipelineConnectionDeclaration
        | hdlPipelineBoundaryDeclaration
        | hdlPipelineGenerateDeclaration
        | hdlPipelineLocalDeclaration
        | hdlPipelineRequirementClause
        | hdlPipelineConstraintClause
        | hdlPipelinePreferenceClause
        | hdlPipelineHintClause
        | hdlPipelineAssertion
        | hdlPipelineExpressionStatement
        | hdlBlock
      )
    ;


/*
 * ============================================================================
 * CONNECTIONS
 * ============================================================================
 *
 * Connections describe semantic data/control flow.
 *
 * They do not specify:
 *
 *     - physical wires;
 *     - routing tracks;
 *     - FPGA switch boxes;
 *     - ASIC routing layers;
 *     - network links.
 */

hdlPipelineConnectionDeclaration
    : hdlKeyword
      hdlPipelineEndpoint
      hdlPipelineConnectionOperator
      hdlPipelineEndpoint
      (
          COLON
          hdlPipelineConnectionPropertyList
      )?
      SEMICOLON?
    ;

hdlPipelineConnectionOperator
    : THIN_ARROW
    | FAT_ARROW
    ;

hdlPipelineEndpoint
    : hdlPipelineReference
    | qualifiedHdlName
      hdlIndexSuffix*
    ;

hdlPipelineConnectionPropertyList
    : hdlPipelineConnectionProperty
      (
          COMMA
          hdlPipelineConnectionProperty
      )*
      COMMA?
    ;

hdlPipelineConnectionProperty
    : identifier
      (
          ASSIGN
          hdlExpression
      )?
    ;


/*
 * ============================================================================
 * BOUNDARIES
 * ============================================================================
 *
 * Boundaries identify semantic pipeline entry/exit points.
 */

hdlPipelineBoundaryDeclaration
    : hdlKeyword
      hdlPipelineBoundaryKind
      hdlPipelineReferenceList
      SEMICOLON?
    ;

hdlPipelineBoundaryKind
    : identifier
    ;


/*
 * ============================================================================
 * REFERENCES
 * ============================================================================
 */

hdlPipelineReferenceList
    : hdlPipelineReference
      (
          COMMA
          hdlPipelineReference
      )*
      COMMA?
    ;

hdlPipelineReference
    : qualifiedHdlName
      hdlIndexSuffix*
    ;

hdlIndexSuffix
    : LBRACKET
      hdlExpression
      RBRACKET
    ;


/*
 * ============================================================================
 * GENERATION
 * ============================================================================
 *
 * Generation allows a source program to express scalable repeated pipeline
 * structures without baking a fixed number of stages into the grammar.
 *
 * The iteration domain is an expression.
 *
 * Therefore:
 *
 *     generate for i in N
 *
 * may be parameterized by N, a compile-time value, a generic, or another
 * legal semantic expression.
 *
 * Actual expansion limits belong to compiler/resource policy.
 */

hdlPipelineGenerateDeclaration
    : hdlKeyword
      hdlPipelineGenerateKind
      hdlPipelineGenerateClause
      (
          LBRACE
          hdlPipelineMember*
          RBRACE
      )
    ;

hdlPipelineGenerateKind
    : identifier
    ;

hdlPipelineGenerateClause
    : hdlExpression
    ;


/*
 * ============================================================================
 * LOCAL DECLARATIONS
 * ============================================================================
 */

hdlPipelineLocalDeclaration
    : hdlKeyword
      identifier
      (
          COLON
          hdlTypeExpression
      )?
      (
          ASSIGN
          hdlExpression
      )?
      SEMICOLON
    ;


/*
 * ============================================================================
 * ASSERTIONS
 * ============================================================================
 *
 * Syntax only.
 *
 * Verification semantics belong to the semantic/verification layer.
 */

hdlPipelineAssertion
    : hdlKeyword
      hdlExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * EXPRESSION STATEMENTS
 * ============================================================================
 */

hdlPipelineExpressionStatement
    : hdlExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses:
 *
 *     *
 *     +
 *     optional clauses
 *     symbolic expressions
 *     generic identifiers
 *     structural recursion
 *
 * rather than fixed cardinalities.
 *
 * Therefore it does not impose language-level limits on:
 *
 *     pipeline count
 *     stage count
 *     stage width
 *     connection count
 *     input count
 *     output count
 *     generated instances
 *     logical resources
 *     pipeline nesting
 *     parameter count
 *
 * Resource limits are external to syntax.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis MUST determine:
 *
 *     1. whether a pipeline declaration is legal;
 *     2. whether stage names are unique in the required scope;
 *     3. whether stage references resolve;
 *     4. whether dependencies are acyclic where required;
 *     5. whether connections are type-compatible;
 *     6. whether input/output boundaries are valid;
 *     7. whether latency expressions are semantically valid;
 *     8. whether initiation constraints are satisfiable;
 *     9. whether throughput requirements are satisfiable;
 *    10. whether resource requirements can be met;
 *    11. whether generated structures are finite/representable under the
 *        compilation resource policy;
 *    12. whether timing semantics are valid;
 *    13. whether hardware capabilities satisfy requirements;
 *    14. whether target realization is possible.
 *
 * None of these checks belong in the parser.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT construct a hardware IR directly.
 *
 * The frontend AST should preserve:
 *
 *     pipeline identity
 *     source spans
 *     generic parameters
 *     attributes
 *     stages
 *     dependencies
 *     connections
 *     boundaries
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     latency metadata
 *     initiation metadata
 *     throughput metadata
 *     resource intent
 *     generation expressions
 *
 * A later semantic lowering phase converts the validated AST into the
 * canonical hardware/HDL representation.
 *
 * If quantum semantics appear inside a pipeline, the quantum portion must
 * eventually lower through the repository's canonical `quantum::ir` boundary.
 *
 * This grammar must never become a second quantum IR.
 *
 * ============================================================================
 * SCHEDULING CONTRACT
 * ============================================================================
 *
 * Pipeline syntax expresses ordering and timing intent.
 *
 * It does NOT select:
 *
 *     ASAP
 *     ALAP
 *     list scheduling
 *     resource-constrained scheduling
 *     modulo scheduling
 *     pipeline balancing algorithm
 *     retiming algorithm
 *
 * Those decisions belong to the scheduling subsystem.
 *
 * ============================================================================
 * HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware abstraction determines whether a pipeline can be realized on the
 * available hardware.
 *
 * This grammar does not query hardware capabilities.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime execution does not depend directly on this grammar.
 *
 * Runtime consumes compiled/validated representations produced downstream.
 *
 * Therefore:
 *
 *     grammar -> AST -> semantics -> IR -> compilation -> runtime
 *
 * is permitted.
 *
 * The reverse dependency:
 *
 *     runtime -> grammar
 *
 * is prohibited.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - accesses no files;
 *     - accesses no network;
 *     - executes no external commands;
 *     - contains no embedded code;
 *     - contains no semantic predicates;
 *     - contains no mutable global state;
 *     - contains no unsafe Rust;
 *     - contains no target-specific behavior.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] Pipeline syntax has a single owner.
 *
 * [ ] hdl.g4 no longer contains a competing pipeline grammar.
 *
 * [ ] hardware-modules.g4 consumes this pipeline rule rather than redefining
 *     it.
 *
 * [ ] Shared identifiers/types/expressions are delegated to canonical rules.
 *
 * [ ] Stage count is not hard-coded.
 *
 * [ ] Pipeline width is not hard-coded.
 *
 * [ ] Connection count is not hard-coded.
 *
 * [ ] Generated pipeline cardinality is not hard-coded.
 *
 * [ ] No physical hardware identifiers are embedded in the grammar.
 *
 * [ ] Timing values remain expressions.
 *
 * [ ] Resource requirements remain semantic expressions.
 *
 * [ ] Requirements, constraints, preferences, and hints remain distinguishable.
 *
 * [ ] Parser semantics remain separate from scheduling semantics.
 *
 * [ ] Parser semantics remain separate from hardware discovery.
 *
 * [ ] Parser semantics remain separate from synthesis.
 *
 * [ ] Parser semantics remain separate from runtime execution.
 *
 * [ ] The grammar can represent tiny pipelines.
 *
 * [ ] The grammar can represent arbitrarily large pipelines subject only to
 *     parser/compiler/resource availability.
 *
 * [ ] No Rust action is present.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Cross-domain HDL tests exist.
 *
 * [ ] Round-trip tests exist where the frontend printer supports them.
 *
 * ============================================================================
 */