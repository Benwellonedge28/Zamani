/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/pipelines.g4
 *
 * Grammar:
 *     HardwarePipelines
 *
 * Status:
 *     Canonical production HDL pipeline parser delegate.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust, actions, predicates, I/O,
 *     hardware discovery, runtime execution, or unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE SOURCE-LEVEL SYNTAX OWNER for HDL pipelines.
 *
 * A pipeline describes target-independent staged computation and data/control
 * flow. It expresses semantic intent, not a particular physical realization.
 *
 * A pipeline may represent:
 *
 *     - streaming computation;
 *     - staged computation;
 *     - arithmetic pipelines;
 *     - dataflow hardware;
 *     - accelerator pipelines;
 *     - CPU-oriented pipelines;
 *     - GPU-oriented pipelines;
 *     - FPGA/ASIC intent;
 *     - heterogeneous computation;
 *     - classical/quantum boundary processing;
 *     - generated/repeated structures;
 *     - future hardware execution models.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Normative authority:
 *
 *     grammar/DESIGN.md
 *          |
 *          v
 *     grammar/spec/hdl.md
 *          |
 *          v
 *     grammar/hdl/pipelines.g4
 *          |
 *          v
 *     grammar/hdl/hdl.g4
 *          |
 *          v
 *     grammar/Zamani.g4
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
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
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - pipeline declarations;
 *     - pipeline parameters;
 *     - pipeline-level attributes;
 *     - pipeline inputs;
 *     - pipeline outputs;
 *     - pipeline requirements;
 *     - pipeline constraints;
 *     - pipeline preferences;
 *     - pipeline hints;
 *     - stage declarations;
 *     - stage parameters;
 *     - stage inputs;
 *     - stage outputs;
 *     - stage dependencies;
 *     - stage latency intent;
 *     - stage initiation interval intent;
 *     - stage throughput intent;
 *     - stage buffering intent;
 *     - stage resource intent;
 *     - semantic connections;
 *     - pipeline boundaries;
 *     - scalable generation constructs;
 *     - pipeline-local expressions;
 *     - pipeline-local assertions.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer definitions;
 *     - identifiers;
 *     - names;
 *     - general expressions;
 *     - general types;
 *     - attributes;
 *     - modules;
 *     - ports as a general HDL construct;
 *     - signals;
 *     - nets;
 *     - registers;
 *     - memories;
 *     - clocks;
 *     - resets;
 *     - timing analysis;
 *     - scheduling algorithms;
 *     - placement;
 *     - routing;
 *     - synthesis;
 *     - hardware discovery;
 *     - target selection;
 *     - resource allocation;
 *     - runtime execution;
 *     - canonical IR construction.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The grammar imposes NO language-level finite limit on:
 *
 *     pipeline count
 *     stage count
 *     parameter count
 *     input count
 *     output count
 *     connection count
 *     dependency count
 *     generated instances
 *     nesting depth
 *     logical resource quantities
 *     pipeline width
 *     symbolic dimensions
 *
 * There is intentionally no:
 *
 *     MAX_PIPELINE_STAGES
 *     MAX_PIPELINE_DEPTH
 *     MAX_LANES
 *     MAX_WIDTH
 *     MAX_INPUTS
 *     MAX_OUTPUTS
 *     MAX_BUFFERS
 *     MAX_CONNECTIONS
 *     MAX_INSTANCES
 *     MAX_RESOURCES
 *
 * "Infinity" means that the language introduces no artificial finite ceiling.
 * Actual limits come from:
 *
 *     - compiler resources;
 *     - available memory;
 *     - target capabilities;
 *     - implementation policy;
 *     - deployment resources;
 *     - synthesis/tool limits.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * These concepts are deliberately separate.
 *
 * Requirement:
 *     MUST be satisfied for a valid realization.
 *
 * Constraint:
 *     Restricts the legal realization space.
 *
 * Preference:
 *     Desired implementation property that may be relaxed where semantics
 *     permit.
 *
 * Hint:
 *     Non-binding implementation information.
 *
 * Resource:
 *     Logical resource requirement or property.
 *
 * None of these constructs selects a physical device.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     CPU_0
 *     GPU_0
 *     FPGA_0
 *     QPU_0
 *     physical_qubit_0
 *     LUT_0
 *     DSP_0
 *     BRAM_0
 *     physical_pin_0
 *     routing_channel_0
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * GENERIC / PARAMETER MODEL
 * ============================================================================
 *
 * Pipeline parameters use the canonical Zamani expression/type system.
 *
 * A parameter can represent:
 *
 *     - width;
 *     - depth;
 *     - stage count;
 *     - lane count;
 *     - data type;
 *     - latency;
 *     - throughput;
 *     - memory quantity;
 *     - resource quantity;
 *     - capability requirement;
 *     - implementation-independent configuration.
 *
 * A parameter is NOT a hidden hardware maximum.
 *
 * ============================================================================
 * SHARED GRAMMAR INTEGRATION
 * ============================================================================
 *
 * This grammar delegates:
 *
 *     Names
 *         -> grammar/core/names.g4
 *
 *     Types
 *         -> grammar/types/types.g4
 *
 *     Expressions
 *         -> grammar/expressions/expressions.g4
 *
 *     Attributes
 *         -> grammar/core/attributes.g4
 *
 * No local identifier, type, expression, literal, or attribute grammar is
 * duplicated here.
 *
 * ============================================================================
 */

parser grammar HardwarePipelines;

options {
    /*
     * Parser grammars consume the canonical production lexer.
     *
     * ZamaniLexer imports ZamaniTokens and is the parser-facing lexical
     * authority.
     */
    tokenVocab = ZamaniLexer;
}

import Names, Types, Expressions, Attributes;


/*
 * ============================================================================
 * 1. PUBLIC PIPELINE ENTRY
 * ============================================================================
 *
 * This is the stable rule consumed by:
 *
 *     grammar/hdl/hdl.g4
 *     grammar/hdl/hardware-modules.g4
 *     grammar/Zamani.g4
 *
 * It deliberately does NOT contain EOF.
 *
 * The universal Zamani root owns EOF.
 *
 * Standalone HDL parsing may wrap the rule in its own EOF-bearing entry rule.
 */

hdlPipelineDeclaration
    : attribute*
      PIPELINE
      identifier
      hdlPipelineParameterBlock?
      hdlPipelineHeaderClause*
      hdlPipelineBody
    ;


/*
 * ============================================================================
 * 2. PIPELINE PARAMETERS
 * ============================================================================
 */

hdlPipelineParameterBlock
    : LPAREN
      hdlPipelineParameterList?
      RPAREN
    ;

hdlPipelineParameterList
    : hdlPipelineParameter
      (COMMA hdlPipelineParameter)*
      COMMA?
    ;

hdlPipelineParameter
    : identifier
      (
          COLON
          typeExpression
      )?
      (
          ASSIGN
          expression
      )?
      hdlPipelineParameterConstraint*
    ;

hdlPipelineParameterConstraint
    : REQUIRES
      expression
    | WHERE
      expression
    ;


/*
 * ============================================================================
 * 3. PIPELINE HEADER
 * ============================================================================
 */

hdlPipelineHeaderClause
    : hdlPipelineInputClause
    | hdlPipelineOutputClause
    | hdlPipelineRequirementClause
    | hdlPipelineConstraintClause
    | hdlPipelinePreferenceClause
    | hdlPipelineHintClause
    | hdlPipelinePropertyClause
    ;


/*
 * ============================================================================
 * 4. INPUTS / OUTPUTS
 * ============================================================================
 *
 * These are logical pipeline interfaces.
 *
 * They are NOT physical pins.
 */

hdlPipelineInputClause
    : INPUT
      hdlPipelinePortList
      SEMICOLON?
    ;

hdlPipelineOutputClause
    : OUTPUT
      hdlPipelinePortList
      SEMICOLON?
    ;

hdlPipelinePortList
    : hdlPipelinePort
      (COMMA hdlPipelinePort)*
      COMMA?
    ;

hdlPipelinePort
    : identifier
      (
          COLON
          typeExpression
      )?
      attribute*
    ;


/*
 * ============================================================================
 * 5. REQUIREMENTS
 * ============================================================================
 */

hdlPipelineRequirementClause
    : REQUIRES
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 6. CONSTRAINTS
 * ============================================================================
 */

hdlPipelineConstraintClause
    : CONSTRAINT
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 7. PREFERENCES
 * ============================================================================
 */

hdlPipelinePreferenceClause
    : PREFER
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 8. HINTS
 * ============================================================================
 */

hdlPipelineHintClause
    : HINT
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 9. GENERIC PIPELINE PROPERTY
 * ============================================================================
 *
 * PROPERTY is deliberately generic.
 *
 * Its semantic interpretation is downstream.
 *
 * This allows future pipeline properties without requiring this grammar to
 * enumerate every possible optimization or implementation property.
 */

hdlPipelinePropertyClause
    : PROPERTY
      identifier
      (
          ASSIGN
          expression
      )?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 10. PIPELINE BODY
 * ============================================================================
 */

hdlPipelineBody
    : LBRACE
      hdlPipelineMember*
      RBRACE
    ;

hdlPipelineMember
    : attribute*
      (
          hdlPipelineStageDeclaration
        | hdlPipelineConnectionDeclaration
        | hdlPipelineBoundaryDeclaration
        | hdlPipelineGenerateDeclaration
        | hdlPipelineRequirementClause
        | hdlPipelineConstraintClause
        | hdlPipelinePreferenceClause
        | hdlPipelineHintClause
        | hdlPipelinePropertyClause
        | hdlPipelineAssertion
        | hdlPipelineExpressionStatement
      )
    ;


/*
 * ============================================================================
 * 11. STAGES
 * ============================================================================
 *
 * A stage is a semantic computation unit.
 *
 * A stage does NOT necessarily correspond to:
 *
 *     - one physical register;
 *     - one clock cycle;
 *     - one CPU instruction;
 *     - one FPGA region;
 *     - one ASIC cell;
 *     - one GPU kernel;
 *     - one physical accelerator.
 *
 * The implementation may choose an appropriate realization.
 */

hdlPipelineStageDeclaration
    : STAGE
      identifier
      hdlPipelineStageParameterBlock?
      hdlPipelineStageHeaderClause*
      hdlPipelineStageBody
    ;

hdlPipelineStageParameterBlock
    : LPAREN
      hdlPipelineStageParameterList?
      RPAREN
    ;

hdlPipelineStageParameterList
    : hdlPipelineStageParameter
      (COMMA hdlPipelineStageParameter)*
      COMMA?
    ;

hdlPipelineStageParameter
    : identifier
      (
          COLON
          typeExpression
      )?
      (
          ASSIGN
          expression
      )?
    ;

hdlPipelineStageBody
    : LBRACE
      hdlPipelineStageMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 12. STAGE HEADERS
 * ============================================================================
 */

hdlPipelineStageHeaderClause
    : hdlPipelineStageInputClause
    | hdlPipelineStageOutputClause
    | hdlPipelineStageDependencyClause
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
 * 13. STAGE INPUTS / OUTPUTS
 * ============================================================================
 */

hdlPipelineStageInputClause
    : INPUT
      hdlPipelineReferenceList
      SEMICOLON?
    ;

hdlPipelineStageOutputClause
    : OUTPUT
      hdlPipelineReferenceList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 14. DEPENDENCIES
 * ============================================================================
 *
 * Dependency syntax expresses semantic ordering.
 *
 * It does not select a scheduling algorithm.
 */

hdlPipelineStageDependencyClause
    : DEPENDS
      hdlPipelineReferenceList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 15. LATENCY
 * ============================================================================
 *
 * The value is an expression.
 *
 * Therefore it can be:
 *
 *     constant;
 *     parameterized;
 *     symbolic;
 *     capability-derived;
 *     target-specialized;
 *     implementation-selected.
 *
 * The grammar does not decide the physical unit.
 */

hdlPipelineLatencyClause
    : LATENCY
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 16. INITIATION INTERVAL
 * ============================================================================
 */

hdlPipelineInitiationClause
    : INITIATION
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 17. THROUGHPUT
 * ============================================================================
 */

hdlPipelineThroughputClause
    : THROUGHPUT
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 18. BUFFERING
 * ============================================================================
 *
 * Buffer quantities are expressions, never fixed grammar limits.
 */

hdlPipelineBufferClause
    : BUFFER
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 19. RESOURCE INTENT
 * ============================================================================
 */

hdlPipelineResourceClause
    : RESOURCE
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 20. STAGE MEMBERS
 * ============================================================================
 */

hdlPipelineStageMember
    : attribute*
      (
          hdlPipelineStageDeclaration
        | hdlPipelineConnectionDeclaration
        | hdlPipelineBoundaryDeclaration
        | hdlPipelineGenerateDeclaration
        | hdlPipelineRequirementClause
        | hdlPipelineConstraintClause
        | hdlPipelinePreferenceClause
        | hdlPipelineHintClause
        | hdlPipelinePropertyClause
        | hdlPipelineAssertion
        | hdlPipelineExpressionStatement
      )
    ;


/*
 * ============================================================================
 * 21. CONNECTIONS
 * ============================================================================
 *
 * Connections represent semantic flow.
 *
 * They do NOT represent:
 *
 *     physical wires;
 *     FPGA routing tracks;
 *     ASIC metal;
 *     network links;
 *     physical pins.
 */

hdlPipelineConnectionDeclaration
    : CONNECT
      hdlPipelineEndpoint
      THIN_ARROW
      hdlPipelineEndpoint
      hdlPipelineConnectionPropertyList?
      SEMICOLON?
    ;

hdlPipelineConnectionPropertyList
    : LPAREN
      hdlPipelineConnectionProperty
      (COMMA hdlPipelineConnectionProperty)*
      COMMA?
      RPAREN
    ;

hdlPipelineConnectionProperty
    : identifier
      (
          ASSIGN
          expression
      )?
    ;


/*
 * ============================================================================
 * 22. ENDPOINTS
 * ============================================================================
 */

hdlPipelineEndpoint
    : qualifiedName
      hdlPipelineIndexSuffix*
    ;

hdlPipelineIndexSuffix
    : LBRACKET
      expression
      RBRACKET
    ;


/*
 * ============================================================================
 * 23. BOUNDARIES
 * ============================================================================
 *
 * Boundaries describe logical entry/exit points.
 */

hdlPipelineBoundaryDeclaration
    : BOUNDARY
      hdlPipelineBoundaryKind
      hdlPipelineReferenceList
      SEMICOLON?
    ;

hdlPipelineBoundaryKind
    : INPUT
    | OUTPUT
    ;


/*
 * ============================================================================
 * 24. REFERENCE LISTS
 * ============================================================================
 */

hdlPipelineReferenceList
    : hdlPipelineReference
      (COMMA hdlPipelineReference)*
      COMMA?
    ;

hdlPipelineReference
    : qualifiedName
      hdlPipelineIndexSuffix*
    ;


/*
 * ============================================================================
 * 25. GENERATION
 * ============================================================================
 *
 * Generation is the scalable structural mechanism.
 *
 * Example:
 *
 *     generate for i in count {
 *         stage lane_i;
 *     }
 *
 * The grammar imposes no limit on the generated cardinality.
 *
 * The compiler/resource layer determines whether the requested specialization
 * can actually be materialized.
 */

hdlPipelineGenerateDeclaration
    : GENERATE
      FOR
      identifier
      IN
      expression
      hdlPipelineGenerateBody
    ;

hdlPipelineGenerateBody
    : LBRACE
      hdlPipelineMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 26. ASSERTIONS
 * ============================================================================
 *
 * Parsing recognizes assertion structure only.
 *
 * Verification semantics belong downstream.
 */

hdlPipelineAssertion
    : ASSERT
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 27. EXPRESSION STATEMENTS
 * ============================================================================
 *
 * Pipeline-local computations may use the canonical expression system.
 */

hdlPipelineExpressionStatement
    : expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 28. SCALABILITY INVARIANTS
 * ============================================================================
 *
 * This grammar intentionally uses:
 *
 *     *
 *     +
 *     recursive structure
 *     symbolic expressions
 *     generic parameters
 *     generated structures
 *
 * It does NOT enumerate finite capacities.
 *
 * Consequently the same syntax can represent:
 *
 *     tiny pipeline
 *     embedded pipeline
 *     CPU pipeline
 *     GPU pipeline
 *     FPGA pipeline
 *     ASIC pipeline
 *     accelerator pipeline
 *     heterogeneous pipeline
 *     future computational pipeline
 *
 * subject only to downstream resources and semantics.
 *
 * ============================================================================
 * 29. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST validate:
 *
 *     - unique pipeline identity;
 *     - parameter declarations;
 *     - parameter constraints;
 *     - stage identity;
 *     - reference resolution;
 *     - connection validity;
 *     - dependency validity;
 *     - cycle legality;
 *     - type compatibility;
 *     - input/output compatibility;
 *     - latency meaning;
 *     - initiation interval meaning;
 *     - throughput requirements;
 *     - buffering requirements;
 *     - resource requirements;
 *     - capability requirements;
 *     - generation legality;
 *     - generated specialization;
 *     - timing-domain correctness;
 *     - portability;
 *     - target realizability.
 *
 * The parser MUST NOT perform these checks.
 *
 * ============================================================================
 * 30. AST CONTRACT
 * ============================================================================
 *
 * Every construct in this file must map to the domain-neutral frontend AST.
 *
 * Required semantic information:
 *
 *     HdlPipelineDecl
 *         identity
 *         attributes
 *         parameters
 *         inputs
 *         outputs
 *         requirements
 *         constraints
 *         preferences
 *         hints
 *         properties
 *         members
 *         source span
 *
 *     HdlPipelineStage
 *         identity
 *         parameters
 *         inputs
 *         outputs
 *         dependencies
 *         latency
 *         initiation
 *         throughput
 *         buffering
 *         resources
 *         properties
 *         members
 *         source span
 *
 *     HdlPipelineConnection
 *         source
 *         target
 *         properties
 *         source span
 *
 *     HdlPipelineGenerate
 *         iterator
 *         domain expression
 *         body
 *         source span
 *
 * The grammar does not define Rust AST structures.
 *
 * ============================================================================
 * 31. IR CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT construct an IR.
 *
 * Required lowering:
 *
 *     source
 *       |
 *       v
 *     parse tree
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic validation
 *       |
 *       v
 *     canonical hardware/HDL semantic model
 *       |
 *       +--> optimization
 *       +--> scheduling
 *       +--> verification
 *       +--> synthesis
 *       +--> placement
 *       +--> routing
 *       +--> target lowering
 *
 * If a pipeline contains quantum computation, its quantum semantics MUST
 * eventually pass through the existing canonical `quantum::ir` boundary.
 *
 * This file MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * 32. SCHEDULING CONTRACT
 * ============================================================================
 *
 * This grammar does not choose:
 *
 *     ASAP scheduling
 *     ALAP scheduling
 *     list scheduling
 *     modulo scheduling
 *     resource-constrained scheduling
 *     retiming
 *     balancing
 *     pipeline insertion
 *
 * It only represents source-level intent and semantic ordering.
 *
 * ============================================================================
 * 33. HARDWARE CONTRACT
 * ============================================================================
 *
 * No grammar rule may:
 *
 *     - query hardware;
 *     - select a device;
 *     - allocate a physical resource;
 *     - perform placement;
 *     - perform routing;
 *     - perform timing closure;
 *     - invoke synthesis;
 *     - select a vendor backend.
 *
 * ============================================================================
 * 34. DETERMINISM
 * ============================================================================
 *
 * Parsing depends only upon:
 *
 *     - source token sequence;
 *     - active language version;
 *     - imported grammar contracts.
 *
 * It MUST NOT depend upon:
 *
 *     - system time;
 *     - randomness;
 *     - filesystem state;
 *     - network state;
 *     - hardware state;
 *     - runtime state;
 *     - target availability.
 *
 * ============================================================================
 * 35. SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     - filesystem access;
 *     - network access;
 *     - command execution;
 *     - environment inspection;
 *     - hardware discovery;
 *     - secret access;
 *     - dynamic execution.
 *
 * ============================================================================
 * 36. RUST CONTRACT
 * ============================================================================
 *
 * The grammar itself contains no Rust.
 *
 * Generated parser integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST require no unsafe Rust.
 *
 * ============================================================================
 * 37. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is a real ANTLR parser grammar.
 * [x] It has one canonical public pipeline declaration rule.
 * [x] It consumes ZamaniLexer.
 * [x] It delegates names.
 * [x] It delegates types.
 * [x] It delegates expressions.
 * [x] It delegates attributes.
 * [x] It has no duplicate identifier grammar.
 * [x] It has no duplicate type grammar.
 * [x] It has no duplicate expression grammar.
 * [x] It has no duplicate attribute grammar.
 * [x] It has no EOF in its compositional entry rule.
 * [x] Pipeline cardinalities are unbounded by syntax.
 * [x] Stage cardinalities are unbounded by syntax.
 * [x] Generated structures are parameterizable.
 * [x] Widths and quantities remain expressions.
 * [x] Requirements remain distinct from preferences.
 * [x] Physical placement is not represented as universal syntax.
 * [x] Routing is not represented as universal syntax.
 * [x] Scheduling algorithms are not represented as parser behavior.
 * [x] Hardware discovery is excluded.
 * [x] Quantum IR duplication is excluded.
 * [x] No fixed qubit limit exists.
 * [x] No fixed CPU/GPU/FPGA/device limit exists.
 * [x] No fixed pipeline-stage limit exists.
 * [x] No embedded Rust exists.
 * [x] No unsafe Rust is required.
 *
 * Remaining repository integration is specified below and does not require
 * changing this file once the shared lexical/parser contracts are established.
 *
 * ============================================================================
 */