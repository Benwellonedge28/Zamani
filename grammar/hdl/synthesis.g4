/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/synthesis.g4
 *
 * Grammar:
 *     HdlSynthesis
 *
 * Status:
 *     CANONICAL / PRODUCTION HDL SYNTHESIS-INTENT GRAMMAR
 *
 * Purpose:
 *     Define target-independent source syntax for expressing synthesis
 *     intent, synthesis requirements, constraints, preferences, hints,
 *     capabilities, resource requirements, profiles, and semantic properties.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     grammar/DESIGN.md
 *            |
 *            v
 *     grammar/spec/hdl.md
 *            |
 *            v
 *     grammar/hdl/synthesis.g4
 *            |
 *            v
 *     grammar/hdl/hdl.g4
 *            |
 *            v
 *     grammar/antlr/ZamaniParser.g4
 *            |
 *            v
 *     grammar/Zamani.g4
 *            |
 *            v
 *     domain-neutral frontend AST
 *            |
 *            v
 *     semantic analysis
 *            |
 *            v
 *     canonical hardware semantic representation / IR
 *            |
 *            +--> verification
 *            +--> optimization
 *            +--> synthesis
 *            +--> scheduling
 *            +--> placement
 *            +--> routing
 *            +--> target lowering
 *            |
 *            v
 *     target realization
 *
 * This file owns SOURCE-LEVEL SYNTHESIS INTENT SYNTAX ONLY.
 *
 * It does not synthesize anything.
 *
 * ============================================================================
 * RUST BASELINE
 * ============================================================================
 *
 * Rust:
 *     1.97 / 1.97.1
 *
 * Edition:
 *     Rust 2021
 *
 * Safety:
 *     Action-free ANTLR parser grammar.
 *     No embedded Rust.
 *     No unsafe code.
 *
 * ============================================================================
 * SINGLE AUTHORITY
 * ============================================================================
 *
 * This file is the sole owner of the general HDL synthesis-intent syntax.
 *
 * It owns:
 *
 *     hdlSynthesisDeclaration
 *     hdlSynthesisSubject
 *     hdlSynthesisBody
 *     hdlSynthesisClause
 *     hdlSynthesisRequiresClause
 *     hdlSynthesisConstraintClause
 *     hdlSynthesisPreferenceClause
 *     hdlSynthesisHintClause
 *     hdlSynthesisResourceClause
 *     hdlSynthesisCapabilityClause
 *     hdlSynthesisProfileClause
 *     hdlSynthesisPropertyClause
 *     hdlSynthesisPropertyValue
 *
 * It does NOT own:
 *
 *     identifiers
 *     qualified names
 *     expressions
 *     types
 *     modules
 *     ports
 *     signals
 *     nets
 *     registers
 *     memories
 *     generate
 *     pipelines
 *     timing implementation
 *     optimization algorithms
 *     synthesis algorithms
 *     technology mapping
 *     placement
 *     routing
 *     scheduling
 *     target discovery
 *     device selection
 *     vendor primitives
 *     physical resources
 *     netlists
 *     bitstreams
 *     ASIC layouts
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * Synthesis intent describes:
 *
 *     WHAT implementation properties matter.
 *
 * It does not describe:
 *
 *     HOW a particular synthesizer must implement them.
 *
 * Therefore this grammar does NOT contain constructs such as:
 *
 *     use_lut(...)
 *     use_bram(...)
 *     use_dsp(...)
 *     use_ff(...)
 *     use_vendor_primitive(...)
 *     map_to_fpga(...)
 *     place_on_fpga(...)
 *     route_through(...)
 *
 * Those belong to explicit target/dialect/deployment layers when genuinely
 * required.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Synthesis intent is target-independent.
 *
 * The same source program may be considered for:
 *
 *     embedded hardware
 *     CPU-associated hardware
 *     GPU-associated hardware
 *     FPGA
 *     ASIC
 *     accelerator
 *     heterogeneous system
 *     quantum-control hardware
 *     future hardware
 *
 * without changing source semantics merely because target capacity changes.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar contains no universal limits such as:
 *
 *     MAX_LUTS
 *     MAX_FFS
 *     MAX_BRAMS
 *     MAX_DSPS
 *     MAX_MODULES
 *     MAX_INSTANCES
 *     MAX_WIDTH
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_PIPELINE_STAGES
 *
 * A value appearing in source is program semantics.
 *
 * For example:
 *
 *     requires memory >= required_memory;
 *
 * is a semantic requirement.
 *
 * It is NOT a compiler-wide memory limit.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * The grammar preserves four distinct semantic categories:
 *
 *     REQUIREMENT
 *         MUST be satisfied by a valid realization.
 *
 *     CONSTRAINT
 *         Restricts valid realizations.
 *
 *     PREFERENCE
 *         Guides optimization but does not change semantic validity.
 *
 *     HINT
 *         Provides optional implementation guidance.
 *
 * A fifth category is supported:
 *
 *     PROPERTY
 *         Named semantic metadata whose meaning is defined by the owning
 *         specification/domain contract.
 *
 * ============================================================================
 * CAPABILITY / RESOURCE
 * ============================================================================
 *
 * Synthesis intent may reference:
 *
 *     capabilities
 *     resources
 *     performance
 *     latency
 *     throughput
 *     bandwidth
 *     energy
 *     power
 *     reliability
 *     resilience
 *     scalability
 *
 * The grammar does not evaluate any of them.
 *
 * Evaluation belongs to semantic analysis and downstream target realization.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * A synthesis declaration identifies a LOGICAL SOURCE SUBJECT.
 *
 * It does not identify:
 *
 *     FPGA0
 *     GPU0
 *     QPU0
 *     CPU4
 *     physical_pin17
 *     BRAM3
 *     DSP7
 *     LUT12
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * EXPRESSION REUSE
 * ============================================================================
 *
 * This grammar reuses the canonical:
 *
 *     expression
 *
 * rule.
 *
 * It deliberately does NOT define:
 *
 *     synthesisExpression
 *     synthesisArithmeticExpression
 *     synthesisCondition
 *     synthesisConstantExpression
 *
 * This prevents creation of a second expression language.
 *
 * ============================================================================
 * NAME REUSE
 * ============================================================================
 *
 * Logical synthesis subjects reuse:
 *
 *     qualifiedName
 *
 * or the existing HDL module reference when the composition layer supplies it.
 *
 * This grammar does not create a second identifier system.
 *
 * ============================================================================
 * SEMANTIC OPEN WORLD
 * ============================================================================
 *
 * Synthesis properties remain extensible through:
 *
 *     property <name> = <expression>;
 *
 * without requiring every future synthesis concern to become a new keyword.
 *
 * The semantic specification determines which properties are recognized,
 * deprecated, experimental, or dialect-specific.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic with respect to:
 *
 *     source text
 *     language version
 *     lexical configuration
 *     explicitly selected dialects
 *
 * This grammar performs no:
 *
 *     hardware discovery
 *     filesystem access
 *     network access
 *     runtime execution
 *     randomness
 *     environment inspection
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser produces normal ANTLR parse-tree structure.
 *
 * The frontend AST should normalize:
 *
 *     hdlSynthesisDeclaration
 *         -> synthesis-intent declaration
 *
 *     hdlSynthesisSubject
 *         -> logical synthesis subject reference
 *
 *     hdlSynthesisRequiresClause
 *         -> requirement
 *
 *     hdlSynthesisConstraintClause
 *         -> constraint
 *
 *     hdlSynthesisPreferenceClause
 *         -> preference
 *
 *     hdlSynthesisHintClause
 *         -> hint
 *
 *     hdlSynthesisResourceClause
 *         -> resource requirement/intent
 *
 *     hdlSynthesisCapabilityClause
 *         -> capability requirement/intent
 *
 *     hdlSynthesisProfileClause
 *         -> synthesis profile reference
 *
 *     hdlSynthesisPropertyClause
 *         -> named semantic property
 *
 * The exact AST type belongs to the domain-neutral frontend AST contract.
 *
 * This grammar must not introduce a vendor-specific AST.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether the synthesis subject exists;
 *     - whether the subject is synthesizable;
 *     - whether requirements are satisfiable;
 *     - whether constraints are legal;
 *     - whether preferences are valid;
 *     - whether hints are well formed;
 *     - whether capabilities exist;
 *     - whether resource expressions are valid;
 *     - whether profiles are compatible;
 *     - whether properties are recognized;
 *     - whether target realization is possible.
 *
 * The parser performs none of these checks.
 *
 * ============================================================================
 * SYNTHESIS BOUNDARY
 * ============================================================================
 *
 * This grammar represents intent BEFORE synthesis.
 *
 * The downstream flow is:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic validation
 *       |
 *       v
 *     hardware semantic representation
 *       |
 *       v
 *     synthesis planning
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     technology mapping
 *       |
 *       v
 *     placement / routing / scheduling
 *       |
 *       v
 *     target artifact
 *
 * This file does not select or invoke any synthesizer.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Synthesis intent may apply to hardware supporting quantum computation.
 *
 * It MUST NOT:
 *
 *     enumerate physical qubits;
 *     select a QPU;
 *     define quantum gates;
 *     create a quantum IR;
 *     perform routing;
 *     perform QEC;
 *     perform ZQN analysis.
 *
 * Quantum semantics continue through:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Synthesis intent may describe hardware supporting classical computation,
 * vector computation, tensor computation, signal processing, scientific
 * computing, or accelerator computation.
 *
 * The corresponding computation semantics remain owned by the classical/data
 * domains.
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * A synthesis declaration may apply to hardware participating in:
 *
 *     classical -> hardware -> quantum -> classical
 *
 * or:
 *
 *     software -> accelerator -> hardware -> software
 *
 * The grammar does not create a hybrid IR.
 *
 * ============================================================================
 * HARDWARE MODULE INTEGRATION
 * ============================================================================
 *
 * The logical subject may identify an existing hardware module or other
 * synthesizable source-level hardware construct.
 *
 * The module grammar remains responsible for declaring that module.
 *
 * This grammar only attaches synthesis intent to it.
 *
 * ============================================================================
 * GENERATE INTEGRATION
 * ============================================================================
 *
 * Generated hardware remains owned by:
 *
 *     grammar/hdl/generate.g4
 *
 * Synthesis intent may apply to the generated semantic result.
 *
 * This grammar does not duplicate generate syntax.
 *
 * ============================================================================
 * PIPELINE INTEGRATION
 * ============================================================================
 *
 * Pipeline declarations remain owned by:
 *
 *     grammar/hdl/pipelines.g4
 *
 * Synthesis properties such as throughput or latency may be expressed here
 * when they are source-level synthesis requirements or preferences.
 *
 * Pipeline construction itself remains outside this file.
 *
 * ============================================================================
 * MEMORY INTEGRATION
 * ============================================================================
 *
 * Memory declarations remain owned by:
 *
 *     grammar/hdl/memories.g4
 *
 * Synthesis intent may constrain logical memory behavior or performance.
 *
 * It does not select:
 *
 *     SRAM
 *     DRAM
 *     BRAM
 *     LUTRAM
 *     HBM
 *     vendor memory primitive
 *
 * unless an explicit target/dialect layer defines such a realization.
 *
 * ============================================================================
 * TIMING INTEGRATION
 * ============================================================================
 *
 * Timing semantics remain owned by the timing subsystem.
 *
 * Synthesis intent may reference timing requirements such as:
 *
 *     latency
 *     throughput
 *     frequency
 *     period
 *
 * but does not perform timing closure.
 *
 * ============================================================================
 * VERIFICATION INTEGRATION
 * ============================================================================
 *
 * Verification remains owned by the verification grammar and semantic system.
 *
 * Synthesis MUST preserve source semantics and remain traceable to verification
 * properties.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource expressions remain semantic expressions.
 *
 * Examples:
 *
 *     requires memory >= required_memory;
 *
 *     requires bandwidth >= required_bandwidth;
 *
 *     requires capability("parallel.compute");
 *
 * These do not impose fixed language-level resource ceilings.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capabilities describe what a realization must provide.
 *
 * Examples:
 *
 *     capability("streaming");
 *     capability("vector.compute");
 *     capability("tensor.compute");
 *     capability("reconfigurable");
 *
 * Capability satisfaction belongs downstream.
 *
 * ============================================================================
 * DIALECT INTEGRATION
 * ============================================================================
 *
 * Vendor- or technology-specific synthesis properties MUST remain in explicit
 * dialects.
 *
 * Core synthesis grammar remains vendor-neutral.
 *
 * A dialect may interpret a generic property only after explicit dialect
 * selection and compatibility validation.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler MUST treat this grammar as source intent.
 *
 * It should lower:
 *
 *     syntax
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     synthesis semantic model
 *       |
 *       v
 *     hardware IR
 *
 * The compiler may then use:
 *
 *     optimization
 *     verification
 *     scheduling
 *     synthesis
 *     placement
 *     routing
 *     target lowering
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Synthesis intent is normally compile/elaboration-time information.
 *
 * It does not execute at runtime.
 *
 * Runtime behavior belongs to the hardware artifact and execution subsystem.
 *
 * ============================================================================
 * SOURCE PROVENANCE
 * ============================================================================
 *
 * The semantic representation MUST preserve source provenance for synthesis
 * declarations and their clauses.
 *
 * Diagnostics should be able to identify:
 *
 *     synthesis subject
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     profile
 *     property
 *
 * back to source locations.
 *
 * ============================================================================
 * ERROR BOUNDARIES
 * ============================================================================
 *
 * Syntax errors:
 *     parser.
 *
 * Semantic errors:
 *     semantic analysis.
 *
 * Unsupported synthesis property:
 *     semantic/dialect validation.
 *
 * Unsatisfied capability:
 *     resource/capability analysis.
 *
 * Insufficient target resources:
 *     target/resource analysis.
 *
 * Synthesis failure:
 *     synthesis backend.
 *
 * Placement/routing failure:
 *     physical realization subsystem.
 *
 * These must not be collapsed into syntax errors.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar imposes no maximum on:
 *
 *     synthesis declarations
 *     synthesis clauses
 *     properties
 *     requirements
 *     constraints
 *     profiles
 *     modules
 *     generated structures
 *     design size
 *     hardware size
 *
 * Repetition is unbounded at language level.
 *
 * Compiler resource exhaustion is an implementation condition, not a language
 * semantic limit.
 *
 * ============================================================================
 * PUBLIC API
 * ============================================================================
 *
 * hdlSynthesisDeclaration
 *     is the primary integration rule.
 *
 * hdlSynthesisClause
 *     is the primary clause dispatcher.
 *
 * No EOF is consumed because this is a delegate grammar.
 *
 * ============================================================================
 */

parser grammar HdlSynthesis;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. SYNTHESIS DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     synthesize Counter;
 *
 *     synthesize compute::Counter;
 *
 *     synthesize Counter {
 *         requires capability("streaming");
 *         constraint latency <= required_latency;
 *         prefer throughput >= desired_throughput;
 *     }
 *
 * The subject is a logical source-level construct.
 */
hdlSynthesisDeclaration
    : SYNTHESIZE
      hdlSynthesisSubject
      hdlSynthesisBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. SYNTHESIS SUBJECT
 * ============================================================================
 *
 * A subject is a logical source-level name.
 *
 * Physical target selection is deliberately excluded.
 */
hdlSynthesisSubject
    : qualifiedName
    ;


/*
 * ============================================================================
 * 3. SYNTHESIS BODY
 * ============================================================================
 *
 * There is no fixed clause count.
 */
hdlSynthesisBody
    : LBRACE
      hdlSynthesisClause*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. CLAUSE DISPATCH
 * ============================================================================
 */

hdlSynthesisClause
    : hdlSynthesisRequiresClause
    | hdlSynthesisConstraintClause
    | hdlSynthesisPreferenceClause
    | hdlSynthesisHintClause
    | hdlSynthesisResourceClause
    | hdlSynthesisCapabilityClause
    | hdlSynthesisProfileClause
    | hdlSynthesisPropertyClause
    ;


/*
 * ============================================================================
 * 5. REQUIREMENT
 * ============================================================================
 *
 * The expression must be semantically satisfied by a valid realization.
 *
 * Examples:
 *
 *     requires memory >= required_memory;
 *     requires latency <= maximum_latency;
 *     requires throughput >= required_throughput;
 */
hdlSynthesisRequiresClause
    : REQUIRES
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. CONSTRAINT
 * ============================================================================
 *
 * A constraint narrows valid implementations.
 */
hdlSynthesisConstraintClause
    : CONSTRAINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. PREFERENCE
 * ============================================================================
 *
 * A preference guides optimization but MUST NOT alter semantic validity.
 *
 * Examples:
 *
 *     prefer throughput >= desired_throughput;
 *     prefer latency <= preferred_latency;
 */
hdlSynthesisPreferenceClause
    : PREFER
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. HINT
 * ============================================================================
 *
 * A hint is non-binding implementation guidance.
 */
hdlSynthesisHintClause
    : HINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. RESOURCE INTENT
 * ============================================================================
 *
 * Resource semantics remain owned by the resource system.
 *
 * This rule only establishes the synthesis-context boundary.
 */
hdlSynthesisResourceClause
    : RESOURCE
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. CAPABILITY INTENT
 * ============================================================================
 *
 * Capability semantics remain owned by the capability/resource subsystem.
 */
hdlSynthesisCapabilityClause
    : CAPABILITY
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. SYNTHESIS PROFILE
 * ============================================================================
 *
 * A profile is a logical semantic selection.
 *
 * It is NOT a vendor tool invocation.
 *
 * Examples:
 *
 *     profile "performance";
 *     profile performance;
 *     profile profile_name;
 *
 * The semantic layer determines whether the selected profile exists.
 */
hdlSynthesisProfileClause
    : PROFILE
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. OPEN-WORLD PROPERTY
 * ============================================================================
 *
 * Properties provide an extensibility boundary without turning every future
 * synthesis concern into a reserved keyword.
 *
 * Example:
 *
 *     property optimization.level = level;
 *
 *     property implementation.strategy = strategy;
 *
 * The meaning of a property is defined by the relevant semantic specification
 * or explicitly selected dialect.
 */
hdlSynthesisPropertyClause
    : PROPERTY
      qualifiedName
      (
          ASSIGN
          hdlSynthesisPropertyValue
      )?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. PROPERTY VALUE
 * ============================================================================
 *
 * General Zamani expressions remain authoritative.
 *
 * This permits:
 *
 *     constants
 *     parameters
 *     generics
 *     symbolic values
 *     calls
 *     structured expressions
 *
 * without creating a second expression language.
 */
hdlSynthesisPropertyValue
    : expression
    ;