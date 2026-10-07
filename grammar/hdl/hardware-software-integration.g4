/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/hdl/hardware-software-integration.g4
 *
 * GRAMMAR
 * -------
 * HdlHardwareSoftwareIntegration
 *
 * STATUS
 * ------
 * CANONICAL PRODUCTION HARDWARE/SOFTWARE INTEGRATION COMPOSITION GRAMMAR
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical HDL-facing composition boundary for
 * hardware/software integration.
 *
 * IMPORTANT:
 *
 * This file is intentionally a COMPOSITION GRAMMAR.
 *
 * It does NOT create a second hardware/software co-design language.
 *
 * Existing feature owners remain authoritative:
 *
 *     grammar/hdl/co-design.g4
 *         owns logical hardware/software co-design declarations,
 *         components, boundaries, flows, bindings and contracts.
 *
 *     grammar/hybrid/host-device.g4
 *         owns host/device execution-role boundaries, transfers,
 *         invocations, execution and synchronization.
 *
 *     grammar/hybrid/accelerator-interoperability.g4
 *         owns generic accelerator interoperability.
 *
 *     grammar/interoperability/hdl.g4
 *         owns HDL interoperability.
 *
 * This file provides ONE stable HDL-facing integration point through which
 * those existing semantic boundaries become reachable from the HDL
 * composition layer.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     grammar/hdl/hdl.g4
 *          |
 *          v
 *     THIS FILE
 *          |
 *          +----------------------------+
 *          |                            |
 *          v                            v
 *     HdlCoDesign                   HostDevice
 *          |                            |
 *          +-------------+--------------+
 *                        |
 *                        v
 *                 domain-neutral AST
 *                        |
 *                        v
 *                structural validation
 *                        |
 *             +----------+----------+
 *             |          |          |
 *             v          v          v
 *           types     effects   capabilities
 *                        |          |
 *                        +----+-----+
 *                             |
 *                             v
 *                         resources
 *                             |
 *                             v
 *                           policy
 *                             |
 *                             v
 *                         provenance
 *                             |
 *                             v
 *                    semantic integration
 *                             |
 *          +------------------+------------------+
 *          |                  |                  |
 *          v                  v                  v
 *      classical          quantum             HDL/
 *       semantic          semantic           hardware
 *          |                  |                  |
 *          |                  v                  |
 *          |             quantum::ir            |
 *          |                  |                  |
 *          +------------------+------------------+
 *                             |
 *                             v
 *                       canonical IR
 *                             |
 *                             v
 *                       optimization
 *                             |
 *                 +-----------+-----------+
 *                 |           |           |
 *                 v           v           v
 *              routing    scheduling   resilience
 *                                         |
 *                                      QEC/ZQN
 *                                         |
 *                                         v
 *                                        HAL
 *                                         |
 *                                         v
 *                                target realization
 *
 * ============================================================================
 * RUST BASELINE
 * ============================================================================
 *
 * Rust:
 *
 *     1.97+
 *
 * Edition:
 *
 *     2021
 *
 * Safety:
 *
 *     safe Rust only
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic actions;
 *     - no semantic predicates;
 *     - no unsafe implementation;
 *     - no runtime execution;
 *     - no hardware discovery;
 *     - no filesystem access;
 *     - no network access;
 *     - no target selection.
 *
 * The Rust frontend consuming this grammar MUST remain safe Rust.
 *
 * Recommended crate-level enforcement:
 *
 *     #![forbid(unsafe_code)]
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Normative architectural authority:
 *
 *     grammar/DESIGN.md
 *          |
 *          v
 *     grammar/specification/
 *          |
 *          v
 *     grammar/spec/hdl.md
 *          |
 *          v
 *     grammar/spec/hybrid.md
 *          |
 *          v
 *     grammar/hdl/hdl.g4
 *          |
 *          v
 *     THIS FILE
 *
 * This file MUST NOT supersede:
 *
 *     grammar/DESIGN.md
 *     grammar/specification/
 *     grammar/spec/
 *
 * It is an implementation composition boundary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     hdlHardwareSoftwareIntegrationConstruct
 *
 *     hdlHardwareSoftwareIntegrationItem
 *
 *     hdlHardwareSoftwareIntegrationDeclaration
 *
 *     hdlHardwareSoftwareIntegrationExpression
 *
 *     hdlHardwareSoftwareIntegrationStatement
 *
 *     hdlHardwareSoftwareIntegrationSource
 *
 * These rules provide stable integration names and do not introduce competing
 * syntax.
 *
 * THIS FILE DOES NOT OWN
 * ---------------------
 *
 *     - lexical definitions;
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - statements;
 *     - functions;
 *     - modules;
 *     - ports;
 *     - signals;
 *     - wires;
 *     - registers;
 *     - memories;
 *     - clocks;
 *     - timing;
 *     - protocols;
 *     - interfaces;
 *     - accelerator operations;
 *     - host/device semantics;
 *     - co-design semantics;
 *     - resource semantics;
 *     - capability semantics;
 *     - effect semantics;
 *     - contract semantics;
 *     - policy semantics;
 *     - provenance;
 *     - quantum operations;
 *     - quantum measurement;
 *     - quantum IR;
 *     - scheduling;
 *     - routing;
 *     - placement;
 *     - synthesis;
 *     - runtime execution;
 *     - physical hardware selection.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Hardware/software integration MUST NOT become a duplicate implementation of
 * any existing subsystem.
 *
 * The ownership boundaries are:
 *
 *     co-design
 *         -> grammar/hdl/co-design.g4
 *
 *     host/device
 *         -> grammar/hybrid/host-device.g4
 *
 *     accelerator interoperability
 *         -> grammar/hybrid/accelerator-interoperability.g4
 *
 *     HDL interoperability
 *         -> grammar/interoperability/hdl.g4
 *
 *     detailed HDL
 *         -> grammar/hdl/hdl.g4
 *
 *     resources
 *         -> grammar/resources/
 *
 *     capabilities
 *         -> grammar/core/capabilities.g4
 *            grammar/resources/capabilities.g4
 *
 *     effects
 *         -> grammar/effects/
 *
 *     contracts
 *         -> grammar/validation/
 *
 *     policies
 *         -> grammar/policies/
 *            grammar/security/
 *            grammar/execution/
 *
 *     quantum
 *         -> grammar/quantum/
 *
 * This file merely exposes the relevant integration boundary.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DIRECT PARSER DEPENDENCIES
 * --------------------------
 *
 *     HdlCoDesign
 *     HostDevice
 *
 * These are imported parser grammars.
 *
 * HdlCoDesign is the owner of logical hardware/software co-design.
 *
 * HostDevice is the owner of host/device execution-role integration.
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * Primary public rule:
 *
 *     hdlHardwareSoftwareIntegrationConstruct
 *
 * Stable source-level entry:
 *
 *     hdlHardwareSoftwareIntegrationSource
 *
 * Stable declaration adapter:
 *
 *     hdlHardwareSoftwareIntegrationDeclaration
 *
 * Stable expression adapter:
 *
 *     hdlHardwareSoftwareIntegrationExpression
 *
 * Stable statement adapter:
 *
 *     hdlHardwareSoftwareIntegrationStatement
 *
 * Consumers MUST prefer the primary public rule.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Hardware/software integration describes:
 *
 *     WHAT computational relationships exist.
 *
 * It may express:
 *
 *     - logical software/hardware components;
 *     - logical boundaries;
 *     - logical data flow;
 *     - logical bindings;
 *     - host/device roles;
 *     - transfers;
 *     - execution intent;
 *     - synchronization;
 *     - capability requirements;
 *     - resource requirements;
 *     - constraints;
 *     - preferences;
 *     - hints.
 *
 * It MUST NOT require the source program to encode unnecessary physical
 * realization details.
 *
 * Therefore this grammar must not require:
 *
 *     CPU identity;
 *     GPU identity;
 *     FPGA identity;
 *     ASIC identity;
 *     QPU identity;
 *     node identity;
 *     physical memory address;
 *     physical bus;
 *     physical pin;
 *     physical qubit;
 *     routing path;
 *     scheduler slot;
 *     vendor device identifier.
 *
 * ============================================================================
 * OPEN-WORLD TARGET MODEL
 * ============================================================================
 *
 * Integration MUST remain open-world.
 *
 * The grammar must support future realization classes without modification.
 *
 * Possible semantic realizations include:
 *
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     DSP
 *     tensor processor
 *     QPU
 *     quantum simulator
 *     embedded processor
 *     distributed processor
 *     cluster
 *     cloud execution
 *     future computational substrate
 *
 * These are semantic/runtime concerns.
 *
 * They MUST NOT become a closed grammar enumeration.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There is NO language-level maximum on:
 *
 *     integration declarations
 *     components
 *     boundaries
 *     flows
 *     bindings
 *     transfers
 *     invocations
 *     executions
 *     synchronizations
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     hints
 *     arguments
 *     results
 *     nested regions
 *     integration depth
 *     integration breadth
 *     hierarchy depth.
 *
 * ANTLR repetition is used through the imported feature grammars.
 *
 * "Unbounded" means:
 *
 *     no artificial language-level ceiling.
 *
 * It does NOT mean:
 *
 *     infinite physical resources;
 *     infinite memory;
 *     infinite compiler resources;
 *     infinite synthesis resources;
 *     infinite execution time.
 *
 * Practical limitations remain implementation/resource-policy concerns.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT introduce:
 *
 *     MAX_COMPONENTS
 *     MAX_BOUNDARIES
 *     MAX_FLOWS
 *     MAX_BINDINGS
 *     MAX_TRANSFERS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_QUBITS
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It MUST NOT encode equivalent hidden limits.
 *
 * It MUST NOT select physical resources.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements are semantic intent.
 *
 * This grammar does not allocate resources.
 *
 * Examples of downstream semantic intent include:
 *
 *     requires memory >= required_memory;
 *
 *     requires qubits >= required_qubits;
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("hardware.synthesis");
 *
 * The actual requirement grammar remains owned by:
 *
 *     grammar/resources/
 *
 * and the relevant feature grammars.
 *
 * This adapter does not reinterpret those expressions.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability identities remain open-world.
 *
 * New capabilities must not require changes to this file.
 *
 * Examples include:
 *
 *     compute::scalar
 *     compute::parallel
 *     tensor::compute
 *     accelerator::compute
 *     hardware::synthesis
 *     quantum::measurement
 *     quantum::dynamic_control
 *     distributed::communication
 *     network::rdma
 *     future::capability
 *
 * The capability system determines:
 *
 *     availability;
 *     compatibility;
 *     satisfiability;
 *     authorization;
 *     target realization.
 *
 * This grammar does not.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Hardware/software integration can participate in effects such as:
 *
 *     IO
 *     network
 *     mutation
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     quantum
 *     simulation
 *     adaptation
 *     reflection
 *     code generation
 *
 * Effects are NOT defined here.
 *
 * The canonical effect subsystem remains authoritative.
 *
 * The semantic analyzer determines effects produced or required by the
 * underlying integration construct.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Integration constructs may become subjects of:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax remains owned by:
 *
 *     grammar/validation/
 *
 * This file does not duplicate contract grammar.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Integration may be constrained by:
 *
 *     security policies;
 *     execution policies;
 *     resource policies;
 *     deployment policies;
 *     adaptation policies;
 *     sandbox policies;
 *     capability policies.
 *
 * Policy semantics remain outside this grammar.
 *
 * The parser records only the delegated syntax.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Every delegated integration construct MUST remain traceable to its source
 * span.
 *
 * Downstream provenance may record:
 *
 *     source construct;
 *     semantic normalization;
 *     capability decision;
 *     resource decision;
 *     policy decision;
 *     optimization;
 *     lowering;
 *     target realization.
 *
 * This grammar does not create provenance records.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The integration adapter MUST NOT create a second AST.
 *
 * The expected representation is:
 *
 *     hdlHardwareSoftwareIntegrationConstruct
 *                 |
 *                 v
 *          delegated parse tree
 *                 |
 *                 v
 *        existing domain-neutral AST
 *
 * Co-design constructs map through the existing co-design AST contract.
 *
 * Host/device constructs map through the existing host/device AST contract.
 *
 * No nodes such as:
 *
 *     HdlSoftwareIntegrationNode
 *     HdlHardwareIntegrationNode
 *     HardwareSoftwareIR
 *
 * are introduced merely because this adapter was the parser entry point.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic validation must determine:
 *
 *     - whether logical components exist;
 *     - whether bindings are legal;
 *     - whether boundaries are compatible;
 *     - whether transferred values are type-compatible;
 *     - whether ownership permits the transfer;
 *     - whether execution roles are valid;
 *     - whether synchronization is legal;
 *     - whether capabilities are satisfiable;
 *     - whether resources are sufficient;
 *     - whether effects are authorized;
 *     - whether policies permit the operation;
 *     - whether quantum/classical boundaries are valid;
 *     - whether HDL/hardware realization is feasible.
 *
 * A syntactically valid construct is NOT proof of target feasibility.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Hardware/software integration may connect:
 *
 *     classical
 *          |
 *          v
 *     hardware control
 *          |
 *          v
 *     quantum operation
 *          |
 *          v
 *     measurement
 *          |
 *          v
 *     classical result
 *
 * Quantum semantics remain owned by:
 *
 *     grammar/quantum/
 *
 * and eventually:
 *
 *     quantum::ir
 *
 * This adapter MUST NOT define:
 *
 *     quantum gates;
 *     qubit identities;
 *     physical qubit mappings;
 *     quantum circuits;
 *     QEC operations;
 *     quantum-specific IR.
 *
 * ============================================================================
 * HDL CONTRACT
 * ============================================================================
 *
 * Detailed HDL remains owned by:
 *
 *     grammar/hdl/hdl.g4
 *
 * This adapter must not define:
 *
 *     modules;
 *     ports;
 *     signals;
 *     registers;
 *     memories;
 *     clocks;
 *     timing;
 *     processes;
 *     pipelines;
 *     synthesis.
 *
 * It only provides the software/hardware integration boundary.
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * An integration boundary may ultimately cross:
 *
 *     one execution context;
 *     multiple processors;
 *     accelerators;
 *     machines;
 *     clusters;
 *     distributed systems;
 *     cloud resources.
 *
 * This file does not encode node counts or physical topology.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file creates NO permanent IR.
 *
 * Preferred path:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic integration model
 *       |
 *       +----------------+----------------+
 *       |                |                |
 *       v                v                v
 *   classical         quantum           HDL/
 *      IR            quantum::ir       hardware IR
 *       |                |                |
 *       +----------------+----------------+
 *                        |
 *                        v
 *                    optimization
 *                        |
 *                        v
 *                    lowering
 *                        |
 *                 routing/scheduling
 *                        |
 *                    resilience
 *                        |
 *                       ZQN
 *                        |
 *                       HAL
 *
 * No competing hardware/software integration IR is introduced.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source text;
 *     canonical token stream;
 *     grammar version;
 *     explicitly selected parser configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     CPU availability;
 *     GPU availability;
 *     FPGA availability;
 *     QPU availability;
 *     node count;
 *     memory availability;
 *     network state;
 *     filesystem state;
 *     environment variables;
 *     wall-clock time;
 *     randomness;
 *     runtime state.
 *
 * Identical source and parser configuration must produce equivalent parse
 * structures.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The adapter is intentionally shallow.
 *
 * It must not:
 *
 *     - perform graph analysis;
 *     - perform resource analysis;
 *     - resolve hardware;
 *     - perform scheduling;
 *     - perform routing;
 *     - perform type checking;
 *     - perform capability negotiation;
 *     - perform policy evaluation.
 *
 * Those operations belong downstream.
 *
 * This keeps parsing approximately proportional to the source structure
 * processed by the delegated grammar.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing this file must never:
 *
 *     - access hardware;
 *     - execute code;
 *     - invoke a device;
 *     - invoke a compiler backend;
 *     - invoke a synthesis tool;
 *     - invoke a simulator;
 *     - access filesystem state;
 *     - access network state;
 *     - access credentials;
 *     - inspect environment variables.
 *
 * External references remain inert source data until validated downstream.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics are produced by the delegated grammar.
 *
 * This adapter must preserve the delegated source context.
 *
 * Semantic diagnostics belong to the corresponding owner:
 *
 *     co-design errors
 *         -> co-design semantic analysis
 *
 *     host/device errors
 *         -> host/device semantic analysis
 *
 *     type errors
 *         -> type system
 *
 *     effect errors
 *         -> effect system
 *
 *     capability errors
 *         -> capability analysis
 *
 *     resource errors
 *         -> resource analysis
 *
 *     policy errors
 *         -> policy/security analysis
 *
 *     hardware realization errors
 *         -> hardware compiler/backend
 *
 *     quantum realization errors
 *         -> quantum compiler/backend
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This adapter introduces no new source spelling.
 *
 * Existing source forms retain their owning grammar.
 *
 * The adapter only gives those constructs an additional composition boundary.
 *
 * Consequently:
 *
 *     adding this file MUST NOT change the meaning of existing valid source.
 *
 * ============================================================================
 * PUBLIC GRAMMAR
 * ============================================================================
 */

parser grammar HdlHardwareSoftwareIntegration;

options {
    tokenVocab = ZamaniLexer;
}

import
    HdlCoDesign,
    HostDevice
    ;


/*
 * ============================================================================
 * PRIMARY PUBLIC CONSTRUCT
 * ============================================================================
 *
 * Every hardware/software integration construct enters through this rule.
 *
 * The alternatives are deliberately delegated.
 *
 * ============================================================================
 */

hdlHardwareSoftwareIntegrationConstruct
    : hdlCoDesignDeclaration
    | hostDeviceConstruct
    ;


/*
 * ============================================================================
 * SOURCE ITEM
 * ============================================================================
 *
 * A source item is exactly one delegated integration construct.
 * ============================================================================
 */

hdlHardwareSoftwareIntegrationItem
    : hdlHardwareSoftwareIntegrationConstruct
    ;


/*
 * ============================================================================
 * DECLARATION ADAPTER
 * ============================================================================
 *
 * Stable semantic/parser-facing declaration boundary.
 *
 * Only declaration-form constructs belong here.
 * ============================================================================
 */

hdlHardwareSoftwareIntegrationDeclaration
    : hdlCoDesignDeclaration
    ;


/*
 * ============================================================================
 * EXPRESSION ADAPTER
 * ============================================================================
 *
 * Host/device constructs may contain expression-level execution and boundary
 * constructs.
 *
 * The underlying expression language remains owned by the canonical
 * expression grammar.
 *
 * This adapter deliberately delegates instead of recreating expression
 * precedence.
 * ============================================================================
 */

hdlHardwareSoftwareIntegrationExpression
    : hostDeviceConstruct
    ;


/*
 * ============================================================================
 * STATEMENT ADAPTER
 * ============================================================================
 *
 * Stable statement-facing boundary.
 *
 * The delegated grammar remains authoritative for the concrete construct.
 * ============================================================================
 */

hdlHardwareSoftwareIntegrationStatement
    : hostDeviceConstruct
    | hdlCoDesignDeclaration
    ;


/*
 * ============================================================================
 * SOURCE ADAPTER
 * ============================================================================
 *
 * This rule is intentionally repetition-based and imposes no finite number of
 * integration constructs.
 *
 * It is useful for subsystem-level parser tests and tooling.
 *
 * It does NOT own the root EOF boundary.
 * ============================================================================
 */

hdlHardwareSoftwareIntegrationSource
    : hdlHardwareSoftwareIntegrationItem*
    ;


/*
 * ============================================================================
 * COMPOSITION ADAPTERS
 * ============================================================================
 *
 * These aliases make the boundary explicit for future parser composition
 * without adding another syntax vocabulary.
 * ============================================================================
 */

hdlHardwareSoftwareCoDesign
    : hdlCoDesignDeclaration
    ;


hdlHardwareSoftwareHostDevice
    : hostDeviceConstruct
    ;


/*
 * ============================================================================
 * CROSS-DOMAIN ADAPTER
 * ============================================================================
 *
 * This rule intentionally exposes the two existing ownership domains as one
 * integration category.
 *
 * It does not flatten their semantic distinctions.
 * ============================================================================
 */

hdlHardwareSoftwareBoundary
    : hdlHardwareSoftwareCoDesign
    | hdlHardwareSoftwareHostDevice
    ;


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE only when:
 *
 * [ ] HdlHardwareSoftwareIntegration is the only grammar identity defined here.
 *
 * [ ] ZamaniLexer is the only lexer vocabulary consumed here.
 *
 * [ ] HdlCoDesign remains the owner of co-design syntax.
 *
 * [ ] HostDevice remains the owner of host/device syntax.
 *
 * [ ] No hardware/software syntax is duplicated here.
 *
 * [ ] No second resource grammar is created.
 *
 * [ ] No second capability grammar is created.
 *
 * [ ] No second effect grammar is created.
 *
 * [ ] No second contract grammar is created.
 *
 * [ ] No second policy grammar is created.
 *
 * [ ] No second quantum grammar is created.
 *
 * [ ] No second HDL grammar is created.
 *
 * [ ] No physical target selection is encoded.
 *
 * [ ] No physical resource identity is encoded.
 *
 * [ ] No fixed machine capacity is encoded.
 *
 * [ ] No universal hardware ceiling is encoded.
 *
 * [ ] Domain-neutral AST mapping is preserved.
 *
 * [ ] Source spans remain available.
 *
 * [ ] Semantic ownership is preserved.
 *
 * [ ] Resource analysis remains downstream.
 *
 * [ ] Capability analysis remains downstream.
 *
 * [ ] Effect analysis remains downstream.
 *
 * [ ] Policy analysis remains downstream.
 *
 * [ ] Quantum lowering remains through quantum::ir.
 *
 * [ ] Classical lowering remains through canonical classical IR.
 *
 * [ ] HDL/hardware lowering remains through the canonical hardware path.
 *
 * [ ] Routing remains downstream.
 *
 * [ ] Scheduling remains downstream.
 *
 * [ ] Resilience remains downstream.
 *
 * [ ] QEC remains downstream.
 *
 * [ ] ZQN remains downstream.
 *
 * [ ] HAL remains downstream.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Rust 1.97+ safe-Rust requirements are satisfied.
 *
 * [ ] No unsafe Rust is required.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */