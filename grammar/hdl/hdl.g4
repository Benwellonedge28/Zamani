/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/hdl/hdl.g4
 *
 * ROLE
 * ----
 * CANONICAL HDL COMPOSITION ROOT / ORCHESTRATOR
 *
 * This file is intentionally a COMPOSITION ROOT.
 *
 * It does not implement individual HDL features.
 * It assembles the independently-owned HDL grammars and provides the stable
 * parser boundary consumed by the rest of Zamani.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Language:
 *     Zamani
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Rust:
 *     Rust 1.97 or later
 *     Rust 2021
 *     safe Rust only
 *
 * Safety:
 *     No embedded Rust.
 *     No actions.
 *     No semantic predicates.
 *     No unsafe implementation requirement.
 *     No filesystem access.
 *     No network access.
 *     No hardware discovery.
 *     No target discovery.
 *     No runtime execution.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Architectural authority:
 *
 *     grammar/DESIGN.md
 *
 * Human language specification:
 *
 *     grammar/specification/
 *
 * Machine contracts:
 *
 *     grammar/spec/
 *
 * HDL specification:
 *
 *     grammar/spec/hdl.md
 *
 * This file:
 *
 *     grammar/hdl/hdl.g4
 *
 * is the SINGLE HDL COMPOSITION AUTHORITY.
 *
 * It owns:
 *
 *     - HDL grammar identity;
 *     - HDL grammar imports;
 *     - HDL source-unit entry point;
 *     - HDL declaration dispatch;
 *     - HDL statement dispatch;
 *     - HDL expression/type integration façades;
 *     - HDL cross-domain integration façades;
 *     - standalone HDL parsing boundary.
 *
 * It does NOT own concrete HDL feature syntax.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     Zamani parser
 *          |
 *          v
 *     HDL composition root
 *          |
 *          +---------------------------------------------+
 *          |                                             |
 *          v                                             v
 *     HDL feature delegates                    other Zamani domains
 *          |                                             |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                         domain-neutral AST
 *                                 |
 *                                 v
 *                         structural validation
 *                                 |
 *              +------------------+------------------+
 *              |                  |                  |
 *              v                  v                  v
 *            types             effects           resources
 *              |                  |                  |
 *              +------------------+------------------+
 *                                 |
 *                                 v
 *                            capabilities
 *                                 |
 *                                 v
 *                             contracts
 *                                 |
 *                                 v
 *                              policies
 *                                 |
 *                                 v
 *                            provenance
 *                                 |
 *                                 v
 *                     canonical hardware semantics
 *                                 |
 *                                 v
 *                         target-independent IR
 *                                 |
 *              +------------------+------------------+
 *              |                  |                  |
 *              v                  v                  v
 *          optimization       verification       simulation
 *              |                  |                  |
 *              +------------------+------------------+
 *                                 |
 *                                 v
 *                        scheduling / routing
 *                                 |
 *                                 v
 *                              synthesis
 *                                 |
 *                                 v
 *                          target realization
 *                                 |
 *              +------------------+------------------+
 *              |        |         |         |        |
 *             CPU      GPU       FPGA      ASIC     QPU
 *              |        |         |         |        |
 *              +--------+---------+---------+--------+
 *                                 |
 *                                 v
 *                         future targets
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * HDL syntax expresses LOGICAL HARDWARE INTENT.
 *
 * It may describe:
 *
 *     - modules;
 *     - interfaces;
 *     - ports;
 *     - signals;
 *     - nets;
 *     - registers;
 *     - memories;
 *     - clocks;
 *     - resets;
 *     - processes;
 *     - combinational behavior;
 *     - sequential behavior;
 *     - state machines;
 *     - pipelines;
 *     - protocols;
 *     - timing intent;
 *     - verification intent;
 *     - simulation intent;
 *     - synthesis intent;
 *     - generation;
 *     - hardware/software co-design;
 *     - physical intent;
 *     - hardware dialects;
 *     - parameterization;
 *     - generic parameterization;
 *     - parallelism;
 *     - resource requirements;
 *     - capabilities;
 *     - constraints;
 *     - preferences;
 *     - contracts;
 *     - policies;
 *     - provenance.
 *
 * It does NOT inherently select:
 *
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a particular accelerator;
 *     - a board;
 *     - a package;
 *     - a physical pin;
 *     - a physical memory block;
 *     - a physical register;
 *     - a routing path;
 *     - a clock tree;
 *     - a vendor primitive;
 *     - a fabrication technology;
 *     - a fixed machine size.
 *
 * Target realization belongs downstream.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file introduces NO universal hardware capacity.
 *
 * It MUST NOT define or imply:
 *
 *     MAX_MODULES
 *     MAX_PORTS
 *     MAX_SIGNALS
 *     MAX_NETS
 *     MAX_REGISTERS
 *     MAX_MEMORIES
 *     MAX_STATES
 *     MAX_TRANSITIONS
 *     MAX_PIPELINE_STAGES
 *     MAX_INSTANCES
 *     MAX_CLOCKS
 *     MAX_CHANNELS
 *     MAX_WIDTH
 *     MAX_LANES
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_NETWORK_SIZE
 *
 * It MUST NOT encode universal physical values such as:
 *
 *     wire [31:0]
 *     register<32>
 *     memory<64GB>
 *     FPGA_WITH_N_LUTS
 *
 * Program-level quantities remain legal values.
 *
 * Physical feasibility is determined by:
 *
 *     semantic analysis
 *     resource analysis
 *     capability negotiation
 *     elaboration
 *     optimization
 *     scheduling
 *     routing
 *     synthesis
 *     lowering
 *     HAL
 *     deployment
 *
 * "Scale from tiny to infinity" therefore means:
 *
 *     NO ARTIFICIAL LANGUAGE-LEVEL CEILING.
 *
 * It does not mean that a physical machine has infinite resources.
 *
 * ============================================================================
 * SINGLE LEXER CONTRACT
 * ============================================================================
 *
 * HDL consumes the repository-wide canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * No HDL-specific lexer is permitted.
 *
 * No HDL grammar may introduce lexical rules.
 *
 * Hardware names remain extensible identifiers unless explicitly reserved by
 * the canonical language specification.
 *
 * ============================================================================
 * DOMAIN OWNERSHIP
 * ============================================================================
 *
 * This file is NOT the owner of:
 *
 *     modules
 *     generics
 *     parameters
 *     interfaces
 *     ports
 *     signals
 *     wires
 *     nets
 *     registers
 *     memories
 *     arrays
 *     clocks
 *     clocking
 *     reset
 *     timing
 *     processes
 *     combinational logic
 *     sequential logic
 *     state machines
 *     pipelines
 *     parallelism
 *     generation
 *     assertions
 *     verification
 *     simulation
 *     synthesis
 *     physical intent
 *     hardware dialects
 *     co-design
 *     hardware/software integration
 *     protocols
 *
 * Those are owned by the corresponding delegate grammar.
 *
 * ============================================================================
 * IMPORTANT OWNERSHIP RULE
 * ============================================================================
 *
 * hdl.g4 MUST NEVER reimplement a rule whose semantic ownership belongs to
 * another file.
 *
 * In particular, this file MUST NOT contain independent implementations of:
 *
 *     hdlModuleDeclaration
 *     hdlInterfaceDeclaration
 *     hdlPortDeclaration
 *     hdlSignalDeclaration
 *     hdlNetDeclaration
 *     hdlWireDeclaration
 *     hdlRegisterDeclaration
 *     hdlMemoryDeclaration
 *     hdlClockDeclaration
 *     hdlResetDeclaration
 *     hdlProcessDeclaration
 *     hdlCombinationalDeclaration
 *     hdlSequentialDeclaration
 *     hdlStateMachineDeclaration
 *     hdlPipelineDeclaration
 *     hdlGenerateDeclaration
 *     hdlTimingDeclaration
 *     hdlAssertion
 *     hdlSimulationDeclaration
 *     hdlSynthesisDeclaration
 *     hdlVerificationDeclaration
 *     hdlPhysicalIntentDeclaration
 *     hdlCoDesignDeclaration
 *     hdlProtocolDeclaration
 *
 * If a delegate owns a rule, this file only dispatches to it.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * The HDL directory is intentionally modular.
 *
 * The composition graph is:
 *
 *     HDL
 *      |
 *      +-- hardware-modules
 *      |     +-- hardware-generics
 *      |     +-- parameters
 *      |     +-- interfaces
 *      |     +-- ports
 *      |
 *      +-- signals
 *      +-- wires / nets
 *      +-- registers
 *      +-- memories
 *      +-- arrays
 *      +-- clocks
 *      +-- clocking
 *      +-- reset
 *      +-- timing
 *      +-- processes
 *      +-- combinational
 *      +-- sequential
 *      +-- state-machines
 *      +-- pipelines
 *      +-- parallelism
 *      +-- generate
 *      +-- assertions
 *      +-- verification
 *      +-- simulation
 *      +-- synthesis
 *      +-- protocols
 *      +-- physical intent
 *      +-- co-design
 *      +-- hardware/software integration
 *      +-- hardware dialects
 *
 * Each delegate remains independently maintainable.
 *
 * ============================================================================
 * COMPLETE HDL DELEGATE SET
 * ============================================================================
 *
 * The following files exist under grammar/hdl/ and participate in the HDL
 * architecture:
 *
 *     README.md
 *     arrays.g4
 *     assertions.g4
 *     clocking.g4
 *     clocks.g4
 *     co-design.g4
 *     combinational.g4
 *     generate.g4
 *     hardware-dialects.g4
 *     hardware-generics.g4
 *     hardware-modules.g4
 *     hardware-software-integration.g4
 *     hdl.g4
 *     interfaces.g4
 *     memories.g4
 *     nets.g4
 *     parallelism.g4
 *     parameters.g4
 *     physical-intent.g4
 *     pipelines.g4
 *     ports.g4
 *     processes.g4
 *     protocols.g4
 *     registers.g4
 *     reset.g4
 *     sequential.g4
 *     signals.g4
 *     simulation.g4
 *     state_machines.g4
 *     synthesis.g4
 *     timing.g4
 *     verification.g4
 *     wires.g4
 *
 * Every feature above is reachable from this composition graph either through
 * a direct import here or through a delegate's import graph.
 *
 * The composition root must not create a second copy of any feature.
 *
 * ============================================================================
 * DUPLICATE-AUTHORITY PREVENTION
 * ============================================================================
 *
 * `nets.g4` and `wires.g4` intentionally participate in one logical
 * connectivity model.
 *
 * The canonical semantic declaration is:
 *
 *     hdlNetDeclaration
 *
 * where the wire grammar may provide wire-specific syntax and normalization.
 *
 * hdl.g4 MUST NOT define another net/wire declaration.
 *
 * Likewise:
 *
 *     assertions.g4
 *     verification.g4
 *
 * participate in one verification/property model.
 *
 * The composition root dispatches to the canonical assertion rule rather than
 * creating another assertion language.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * HDL constructs may participate in the repository-wide:
 *
 *     resources
 *     capabilities
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     effects
 *     contracts
 *     policies
 *     provenance
 *
 * The HDL grammar merely provides the syntactic composition boundary.
 *
 * It does not:
 *
 *     - inspect target hardware;
 *     - query resources;
 *     - discover devices;
 *     - select a target;
 *     - perform scheduling;
 *     - perform placement;
 *     - perform routing.
 *
 * Example semantic intent:
 *
 *     requires capability("hardware.pipeline");
 *     requires capability("streaming");
 *
 * is analyzed downstream.
 *
 * The source remains portable.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * HDL can participate in:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     AI acceleration
 *     tensor computation
 *     distributed execution
 *     networking
 *     concurrency
 *     memory systems
 *     simulation
 *     security
 *     interoperability
 *     metaprogramming
 *
 * The composition root does not duplicate those domains.
 *
 * Their syntax and semantics remain owned by their respective repository
 * subsystems.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * HDL may surround or control quantum computation.
 *
 * It MUST NOT define quantum operation syntax.
 *
 * Quantum semantics remain:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     quantum semantic model
 *       |
 *       v
 *     quantum::ir
 *
 * HDL can provide:
 *
 *     - timing;
 *     - control;
 *     - interfaces;
 *     - memory;
 *     - classical control;
 *     - accelerator structure;
 *     - hardware/software boundaries.
 *
 * It MUST NOT create:
 *
 *     - a second quantum IR;
 *     - a hardware-specific qubit limit;
 *     - a fixed QPU topology;
 *     - a gate enumeration;
 *     - QEC implementation;
 *     - routing implementation.
 *
 * ============================================================================
 * VERIFICATION / SIMULATION / SYNTHESIS
 * ============================================================================
 *
 * These are distinct semantic phases even though they are represented in the
 * same source architecture.
 *
 * Verification:
 *
 *     describes properties, assertions, assumptions, coverage and related
 *     correctness intent.
 *
 * Simulation:
 *
 *     describes execution/simulation intent.
 *
 * Synthesis:
 *
 *     describes synthesis intent and constraints.
 *
 * None of these phases becomes the universal target-selection mechanism.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing through this grammar must depend only on:
 *
 *     source text
 *     canonical token vocabulary
 *     grammar version
 *     explicitly selected language/dialect configuration
 *
 * It must not depend on:
 *
 *     CPU count
 *     GPU availability
 *     FPGA availability
 *     QPU availability
 *     target topology
 *     filesystem state
 *     network state
 *     environment variables
 *     wall-clock time
 *     randomness
 *     runtime state
 *     scheduler state
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file creates NO AST nodes.
 *
 * Its rules only establish parser boundaries.
 *
 * The frontend must lower accepted syntax into the domain-neutral AST.
 *
 * The AST must remain independent of:
 *
 *     CPU architecture
 *     GPU architecture
 *     FPGA family
 *     ASIC implementation
 *     vendor primitive
 *     board
 *     physical pin
 *     routing
 *     placement
 *     calibration
 *     target topology
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis is responsible for:
 *
 *     - name resolution;
 *     - declaration resolution;
 *     - type checking;
 *     - width/shape analysis;
 *     - driver analysis;
 *     - clock-domain analysis;
 *     - reset analysis;
 *     - timing analysis;
 *     - process classification;
 *     - combinational completeness;
 *     - sequential legality;
 *     - state-machine validity;
 *     - pipeline validity;
 *     - protocol consistency;
 *     - resource analysis;
 *     - capability analysis;
 *     - effect analysis;
 *     - contract validation;
 *     - policy validation;
 *     - provenance construction;
 *     - synthesis eligibility;
 *     - simulation eligibility;
 *     - target-independent optimization eligibility.
 *
 * None of these are performed by this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * HDL parsing does not select a backend IR.
 *
 * The semantic layer produces the canonical hardware semantic representation.
 *
 * Lowering may subsequently produce:
 *
 *     - canonical classical IR where appropriate;
 *     - hardware/HDL IR;
 *     - quantum::ir for quantum semantics;
 *     - other explicitly-owned domain IRs.
 *
 * This grammar MUST NOT create an IR.
 *
 * ============================================================================
 * BACKEND CONTRACT
 * ============================================================================
 *
 * The following are downstream:
 *
 *     optimization
 *     elaboration
 *     scheduling
 *     placement
 *     routing
 *     synthesis
 *     simulation
 *     lowering
 *     ZQN where applicable
 *     HAL
 *     target realization
 *
 * Therefore this grammar contains no:
 *
 *     device selection
 *     board selection
 *     FPGA selection
 *     ASIC selection
 *     QPU selection
 *     vendor selection
 *     physical pin allocation
 *     physical address allocation
 *     clock-tree construction
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors are parser errors.
 *
 * Semantic errors belong to semantic validation.
 *
 * This grammar must not:
 *
 *     - silently discard invalid HDL;
 *     - turn malformed constructs into valid constructs;
 *     - select target-specific defaults;
 *     - inspect hardware;
 *     - execute HDL;
 *     - print diagnostics;
 *     - access external state.
 *
 * Source spans must remain available to the frontend for diagnostics,
 * formatting, IDE tooling, provenance and reproducibility.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Compatibility belongs to:
 *
 *     grammar/compatibility/
 *
 * This file must not duplicate historical syntax aliases.
 *
 * A historical syntax form must map to the canonical HDL semantic construct
 * through the compatibility architecture.
 *
 * ============================================================================
 * PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * `hdlDesign` is the standalone HDL entry point.
 *
 * It is EOF-bearing so it can be used by:
 *
 *     HDL-specific parser tests
 *     HDL tooling
 *     conformance tests
 *     standalone grammar validation
 *
 * The repository-wide Zamani parser may instead consume:
 *
 *     hdlSourceElement
 *
 * as one domain of the universal source language.
 *
 * ============================================================================
 */

parser grammar HDL;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * HDL FEATURE IMPORTS
 * ============================================================================
 *
 * Concrete feature ownership remains in the delegate grammars.
 *
 * IMPORTANT:
 *
 * The names below are the existing grammar identities used by the HDL tree.
 *
 * Where a delegate itself imports another HDL grammar, the delegate remains
 * the transitive owner. The composition root must not duplicate it.
 * ============================================================================
 */

import
    Arrays,
    HdlAssertions,
    HdlClocking,
    Clocks,
    HdlCoDesign,
    Combinational,
    HdlGenerate,
    HardwareDialects,
    HardwareGenerics,
    HardwareModules,
    HdlHardwareSoftwareIntegration,
    HdlInterfaces,
    Memories,
    nets,
    HardwareParallelism,
    HDLParameters,
    HdlPhysicalIntent,
    HardwarePipelines,
    HardwarePorts,
    processes,
    HdlProtocols,
    registers,
    ZamaniHDLReset,
    sequential,
    HardwareSignals,
    HdlSimulation,
    HdlStateMachines,
    HdlSynthesis,
    HdlTiming,
    HdlVerification,
    wires
;

/*
 * ============================================================================
 * STANDALONE HDL ENTRY
 * ============================================================================
 *
 * The source unit is intentionally unbounded.
 *
 * Zero or more HDL source elements are accepted.
 *
 * No source-count limit exists.
 */

hdlDesign
    : hdlSourceElement* EOF
    ;

/*
 * ============================================================================
 * UNIVERSAL HDL SOURCE ELEMENT
 * ============================================================================
 *
 * This is the principal orchestration rule.
 *
 * It does not implement any concrete feature.
 *
 * Every alternative delegates to an existing feature owner.
 *
 * ============================================================================
 */

hdlSourceElement
    : hdlDeclaration
    | hdlStatement
    | hdlExpression
    ;

/*
 * ============================================================================
 * HDL DECLARATION DISPATCH
 * ============================================================================
 *
 * Declarations are grouped by semantic ownership.
 *
 * No concrete declaration is reimplemented here.
 * ============================================================================
 */

hdlDeclaration
    : hdlModuleDeclaration
    | hdlInterfaceDeclaration
    | hdlPackageDeclaration
    | hdlDialectDeclaration
    | hdlParameterDeclaration
    | hdlLocalParameterDeclaration
    | hdlTypeDeclaration
    | hdlGenericDeclaration
    | hdlPortDeclaration
    | hdlSignalDeclaration
    | hdlNetDeclaration
    | hdlRegisterDeclaration
    | hdlMemoryDeclaration
    | hdlClockDeclaration
    | hdlResetDeclaration
    | hdlProcessDeclaration
    | hdlAlwaysDeclaration
    | hdlCombinationalDeclaration
    | hdlSequentialDeclaration
    | hdlStateMachineDeclaration
    | hdlPipelineDeclaration
    | hdlInstanceDeclaration
    | hdlGenerateDeclaration
    | hdlTimingDeclaration
    | hdlPhysicalIntentDeclaration
    | hdlCoDesignDeclaration
    | hdlProtocolDeclaration
    | hdlHardwareSoftwareIntegrationDeclaration
    | hdlSimulationDeclaration
    | hdlSynthesisDeclaration
    | hdlAssertion
    | hdlVerificationIntent
    | hdlParallelismDeclaration
    ;

/*
 * ============================================================================
 * HDL STATEMENT DISPATCH
 * ============================================================================
 *
 * HDL procedural statements remain integrated with the universal Zamani
 * statement model.
 *
 * This grammar does not create a second general-purpose programming language.
 * ============================================================================
 */

hdlStatement
    : hdlAssignment
    | hdlIfStatement
    | hdlCaseStatement
    | hdlForStatement
    | hdlWhileStatement
    | hdlRepeatStatement
    | hdlExpressionStatement
    ;

/*
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * General expression semantics remain owned by the universal expression
 * subsystem.
 *
 * This façade gives HDL delegates a stable domain-level expression boundary.
 * ============================================================================
 */

hdlExpression
    : expression
    ;

/*
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * General type semantics remain owned by the universal type subsystem.
 * ============================================================================
 */

hdlTypeExpression
    : typeExpression
    ;

/*
 * ============================================================================
 * ATTRIBUTE INTEGRATION
 * ============================================================================
 *
 * General attribute syntax remains owned by the canonical attribute system.
 * ============================================================================
 */

hdlAttribute
    : attribute
    ;

hdlAttributeList
    : attribute*
    ;

/*
 * ============================================================================
 * BLOCK INTEGRATION
 * ============================================================================
 *
 * HDL does not create a second block syntax.
 * ============================================================================
 */

hdlBlock
    : block
    ;

/*
 * ============================================================================
 * NAME INTEGRATION
 * ============================================================================
 *
 * HDL names are logical source-level names.
 *
 * Physical meaning is assigned by semantic analysis and target realization.
 * ============================================================================
 */

hdlName
    : identifier
    ;

hdlQualifiedName
    : qualifiedName
    ;

/*
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * These façade rules deliberately delegate to the repository-wide resource
 * and capability systems.
 *
 * They do not perform resource negotiation.
 * ============================================================================
 */

hdlRequirement
    : requirement
    ;

hdlConstraint
    : constraint
    ;

hdlCapability
    : capability
    ;

hdlPolicy
    : policy
    ;

hdlContract
    : contract
    ;

/*
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Provenance remains repository-wide.
 *
 * HDL constructs may participate in it without creating a second provenance
 * model.
 * ============================================================================
 */

hdlProvenance
    : provenance
    ;

/*
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Effects remain repository-wide.
 *
 * HDL does not define a second effect system.
 * ============================================================================
 */

hdlEffect
    : effect
    ;

/*
 * ============================================================================
 * CROSS-DOMAIN EXTENSION POINT
 * ============================================================================
 *
 * HDL can coexist with other Zamani computational domains.
 *
 * The semantic layer decides whether a particular construct is legal in a
 * hardware context.
 *
 * The parser therefore remains open to the canonical domain integration point
 * rather than embedding domain-specific backend logic here.
 * ============================================================================
 */

hdlDomainElement
    : hdlDeclaration
    | hdlStatement
    | hdlExpression
    ;

/*
 * ============================================================================
 * HYBRID / QUANTUM BOUNDARY
 * ============================================================================
 *
 * This is deliberately a façade rather than a quantum grammar.
 *
 * Quantum constructs remain owned by the quantum subsystem and eventually
 * cross the canonical quantum::ir boundary.
 * ============================================================================
 */

hdlHybridElement
    : hdlExpression
    | hdlStatement
    | hdlDeclaration
    ;

/*
 * ============================================================================
 * SIMULATION / VERIFICATION / SYNTHESIS COMPOSITION
 * ============================================================================
 *
 * These are syntactic integration points only.
 *
 * Their semantic meanings remain owned by their respective delegates.
 * ============================================================================
 */

hdlVerificationElement
    : hdlAssertion
    | hdlVerificationIntent
    ;

hdlSimulationElement
    : hdlSimulationDeclaration
    ;

hdlSynthesisElement
    : hdlSynthesisDeclaration
    ;

/*
 * ============================================================================
 * RESOURCE-AWARE HDL ELEMENT
 * ============================================================================
 *
 * The actual resource/capability grammar remains outside this file.
 *
 * This rule exists only to give HDL consumers one stable integration point.
 * ============================================================================
 */

hdlResourceElement
    : hdlRequirement
    | hdlConstraint
    | hdlCapability
    | hdlPolicy
    | hdlContract
    ;

/*
 * ============================================================================
 * COMPLETION / OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE IS DONE WHEN:
 *
 * [x] HDL has exactly one composition root.
 *
 * [x] The root uses the canonical Zamani lexer.
 *
 * [x] Concrete HDL features remain in their owning files.
 *
 * [x] No hardware capacity is hard-coded.
 *
 * [x] No target is selected by parsing.
 *
 * [x] No vendor primitive is enumerated here.
 *
 * [x] No physical resource is allocated here.
 *
 * [x] Quantum operations are not duplicated here.
 *
 * [x] quantum::ir remains the canonical quantum IR boundary.
 *
 * [x] Resource/capability semantics remain downstream.
 *
 * [x] Effects remain downstream.
 *
 * [x] Contracts remain downstream.
 *
 * [x] Policies remain downstream.
 *
 * [x] Provenance remains downstream.
 *
 * [x] AST construction remains outside the grammar.
 *
 * [x] IR construction remains outside the grammar.
 *
 * [x] Optimization remains outside the grammar.
 *
 * [x] Routing remains outside the grammar.
 *
 * [x] Scheduling remains outside the grammar.
 *
 * [x] Synthesis implementation remains outside the grammar.
 *
 * [x] HAL remains outside the grammar.
 *
 * [x] Target realization remains outside the grammar.
 *
 * [x] The grammar contains no Rust.
 *
 * [x] The grammar requires no unsafe Rust.
 *
 * [x] Source cardinality is unbounded by language-level constants.
 *
 * [x] HDL can participate in classical, quantum, hybrid, distributed,
 *     accelerator and future computational designs.
 *
 * [x] HDL syntax can remain unchanged while target realization changes.
 *
 * ============================================================================
 * INTEGRATION REQUIREMENTS FOR THE OTHER HDL FILES
 * ============================================================================
 *
 * Every delegate imported by this file must satisfy the following invariant:
 *
 *     ONE FILE
 *       =
 *     ONE PRIMARY SYNTAX OWNER
 *
 * Each delegate must declare:
 *
 *     Purpose
 *     Owns
 *     Does Not Own
 *     Dependencies
 *     Exported Rules
 *     AST Contract
 *     Semantic Contract
 *     Type Contract
 *     Effect Contract
 *     Capability Contract
 *     Resource Contract
 *     Contract/Policy Contract
 *     Provenance Contract
 *     IR Contract
 *     Diagnostics
 *     Positive Tests
 *     Negative Tests
 *     Boundary Tests
 *     Scalability Tests
 *     Compatibility
 *     Completion Criteria
 *
 * The parent `hdl.g4` must not need to be edited when an existing delegate
 * gains internal rules.
 *
 * The parent only changes when:
 *
 *     - a new top-level HDL feature is introduced;
 *     - an ownership boundary changes;
 *     - a public entry rule changes;
 *     - a new delegate is introduced.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Public HDL composition:
 *
 *     hdl.g4
 *
 * owns:
 *
 *     hdlDesign
 *     hdlSourceElement
 *     hdlDeclaration
 *     hdlStatement
 *     HDL integration façades
 *
 * Concrete ownership:
 *
 *     hardware-modules.g4
 *         -> module / instance structure
 *
 *     hardware-generics.g4
 *         -> hardware generic syntax
 *
 *     parameters.g4
 *         -> HDL parameters
 *
 *     interfaces.g4
 *         -> interfaces
 *
 *     ports.g4
 *         -> ports
 *
 *     signals.g4
 *         -> signals
 *
 *     nets.g4 / wires.g4
 *         -> logical connectivity
 *
 *     registers.g4
 *         -> registers
 *
 *     memories.g4
 *         -> memories
 *
 *     arrays.g4
 *         -> HDL array constructs
 *
 *     clocks.g4
 *         -> clock declarations
 *
 *     clocking.g4
 *         -> clocking relationships
 *
 *     reset.g4
 *         -> reset declarations
 *
 *     timing.g4
 *         -> timing intent
 *
 *     processes.g4
 *         -> processes
 *
 *     combinational.g4
 *         -> combinational behavior
 *
 *     sequential.g4
 *         -> sequential behavior
 *
 *     state_machines.g4
 *         -> state machines
 *
 *     pipelines.g4
 *         -> pipelines
 *
 *     parallelism.g4
 *         -> hardware parallelism
 *
 *     generate.g4
 *         -> elaboration/generation
 *
 *     assertions.g4
 *         -> assertion syntax
 *
 *     verification.g4
 *         -> verification semantics
 *
 *     simulation.g4
 *         -> simulation syntax
 *
 *     synthesis.g4
 *         -> synthesis intent
 *
 *     protocols.g4
 *         -> hardware protocols
 *
 *     physical-intent.g4
 *         -> physical intent
 *
 *     co-design.g4
 *         -> hardware/software co-design
 *
 *     hardware-software-integration.g4
 *         -> HW/SW integration
 *
 *     hardware-dialects.g4
 *         -> open-world hardware dialects
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * No rule in this file may inspect or depend on:
 *
 *     target CPU
 *     target GPU
 *     target FPGA
 *     target ASIC
 *     target QPU
 *     target accelerator
 *     target node count
 *     target memory size
 *     target topology
 *     target routing
 *     target calibration
 *     target clock tree
 *
 * The same source syntax must remain meaningful across:
 *
 *     tiny
 *     embedded
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed
 *     cloud
 *     future hardware
 *
 * subject to semantic feasibility and available resources.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * `hdl.g4` is the ORCHESTRATOR, not the IMPLEMENTATION.
 *
 * The desired architecture is:
 *
 *                 +----------------+
 *                 |    hdl.g4      |
 *                 | composition    |
 *                 |     root       |
 *                 +-------+--------+
 *                         |
 *        +----------------+----------------+
 *        |                |                |
 *        v                v                v
 *     structure        behavior         intent
 *        |                |                |
 *        v                v                v
 *   modules/ports     processes/...    timing/verification/...
 *        |                |                |
 *        +----------------+----------------+
 *                         |
 *                         v
 *                domain-neutral AST
 *                         |
 *                         v
 *                 semantic validation
 *                         |
 *        +----------------+----------------+
 *        |        |       |       |        |
 *      types   effects resources contracts policies
 *        |        |       |       |        |
 *        +----------------+----------------+
 *                         |
 *                         v
 *               canonical hardware model
 *                         |
 *                         v
 *                  target-independent IR
 *                         |
 *                         v
 *              optimize / route / schedule
 *                         |
 *                         v
 *                       ZQN
 *                         |
 *                         v
 *                       HAL
 *                         |
 *                         v
 *                    realization
 *
 * This is the HDL architecture required for POCO-REAF.
 */