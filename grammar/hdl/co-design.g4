/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/co-design.g4
 *
 * Grammar:
 *     HdlCoDesign
 *
 * Status:
 *     PRODUCTION-READY HDL / SOFTWARE CO-DESIGN SYNTAX DELEGATE
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines portable source-level hardware/software co-design
 * intent.
 *
 * Co-design is the boundary where one Zamani program can describe cooperating
 * computational components whose realization may span:
 *
 *     - classical software;
 *     - CPUs;
 *     - GPUs;
 *     - programmable logic;
 *     - ASIC-oriented hardware;
 *     - accelerators;
 *     - quantum control;
 *     - heterogeneous computing;
 *     - distributed execution;
 *     - memory systems;
 *     - communication/interconnect;
 *     - future computational substrates.
 *
 * This file describes RELATIONSHIPS and INTENT.
 *
 * It does NOT select physical hardware.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Co-design source describes:
 *
 *     WHAT components exist;
 *     WHAT interfaces they expose;
 *     WHAT data/control relationships exist;
 *     WHAT capabilities are required;
 *     WHAT resources are required;
 *     WHAT constraints apply;
 *     WHAT implementation preferences exist;
 *     WHAT contracts must hold;
 *     WHAT data/control flows must be preserved.
 *
 * It MUST NOT require source rewriting merely because realization changes
 * between:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     embedded target
 *     distributed target
 *     future target.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * Normative hierarchy:
 *
 *     grammar/DESIGN.md
 *          |
 *          v
 *     grammar/spec/hdl.md
 *          |
 *          v
 *     grammar/hdl/co-design.g4
 *          |
 *          v
 *     grammar/hdl/hdl.g4
 *          |
 *          v
 *     grammar/antlr/ZamaniParser.g4
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic / hardware IR
 *          |
 *          +--> classical lowering
 *          +--> quantum::ir
 *          +--> HDL/hardware lowering
 *          +--> distributed lowering
 *          +--> accelerator lowering
 *          |
 *          v
 *     optimization
 *          |
 *     routing
 *          |
 *     scheduling
 *          |
 *     resilience / QEC / ZQN where applicable
 *          |
 *     HAL
 *          |
 *     target realization
 *
 * This file is a parser delegate.
 *
 * It MUST NOT become another Zamani root grammar.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - co-design declaration syntax;
 *     - co-design body composition;
 *     - logical co-design component references;
 *     - logical software/hardware boundary declarations;
 *     - logical component relationships;
 *     - logical data/control flow declarations;
 *     - co-design bindings;
 *     - co-design contracts;
 *     - co-design requirements;
 *     - co-design capability requirements;
 *     - co-design resource intent;
 *     - co-design constraints;
 *     - co-design preferences;
 *     - co-design hints;
 *     - co-design target intent;
 *     - co-design parameterization;
 *     - open-ended co-design extension blocks.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical definitions;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - general types;
 *     - functions;
 *     - modules generally;
 *     - ports generally;
 *     - interfaces generally;
 *     - protocols generally;
 *     - signals;
 *     - registers;
 *     - memories;
 *     - clocks;
 *     - timing implementation;
 *     - physical devices;
 *     - physical addresses;
 *     - physical pins;
 *     - physical qubits;
 *     - FPGA resources;
 *     - ASIC cells;
 *     - routing;
 *     - placement;
 *     - scheduling algorithms;
 *     - synthesis;
 *     - simulation;
 *     - calibration;
 *     - hardware discovery;
 *     - runtime allocation;
 *     - vendor APIs;
 *     - QEC implementation;
 *     - ZQN implementation;
 *     - HAL implementation;
 *     - quantum::ir.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * General HDL syntax remains owned by the existing HDL delegates:
 *
 *     hardware-modules.g4
 *     hardware-generics.g4
 *     hardware-parameters.g4
 *     ports.g4
 *     interfaces.g4
 *     protocols.g4
 *     signals.g4
 *     wires.g4
 *     registers.g4
 *     memories.g4
 *     clocks.g4
 *     timing.g4
 *     processes.g4
 *     combinational.g4
 *     sequential.g4
 *     state machines
 *     pipelines
 *     generate
 *     assertions
 *
 * This file only composes those concepts at the SOFTWARE/HARDWARE boundary.
 *
 * It MUST NOT copy their complete grammars.
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * This grammar consumes:
 *
 *     ZamaniLexer
 *
 * It MUST NOT define lexer rules.
 *
 * It MUST NOT introduce:
 *
 *     K_* aliases;
 *     parser-local token definitions;
 *     an HDL-specific lexer;
 *     a second keyword vocabulary.
 *
 * The repository's canonical lexer architecture is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     grammar/lexer/tokens.g4
 *          |
 *          +--> keywords
 *          +--> operators
 *          +--> punctuation
 *          +--> identifiers
 *          +--> literals
 *
 * ============================================================================
 * CO-DESIGN KEYWORD INTEGRATION
 * ============================================================================
 *
 * The current repository does not yet define a dedicated CO_DESIGN keyword.
 *
 * The canonical lexical addition, when this syntax is promoted to stable
 * language syntax, is:
 *
 *     CO_DESIGN : 'co_design' ;
 *
 * in:
 *
 *     grammar/lexer/keywords.g4
 *
 * and therefore through:
 *
 *     grammar/lexer/tokens.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * No `K_CO_DESIGN` token is to be invented.
 *
 * The parser rule below intentionally consumes:
 *
 *     CO_DESIGN
 *
 * rather than a second token spelling.
 *
 * Until the lexical addition is merged, this file is a complete parser
 * contract but the repository's lexer composition is incomplete for the
 * dedicated declaration spelling.
 *
 * This is an explicit integration dependency rather than hidden parser
 * behavior.
 *
 * ============================================================================
 * SHARED GRAMMAR DEPENDENCIES
 * ============================================================================
 *
 * This grammar consumes canonical shared rules:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     attribute
 *
 * It does NOT redefine them.
 *
 * The canonical parser composition layer is responsible for making those
 * rules available to this delegate.
 *
 * ============================================================================
 * POCO-REAF / OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * This grammar deliberately does NOT contain closed alternatives such as:
 *
 *     cpuComponent
 *     gpuComponent
 *     fpgaComponent
 *     asicComponent
 *     qpuComponent
 *     acceleratorComponent
 *
 * A component kind is represented by a source-level qualified name.
 *
 * Therefore future computational substrates do not require this grammar to
 * be rewritten.
 *
 * Examples:
 *
 *     software.host
 *     hardware.accelerator
 *     quantum.controller
 *     future.compute.substrate
 *
 * are semantic identities.
 *
 * The grammar does not decide what they mean.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO language-level limits on:
 *
 *     co-design declarations
 *     components
 *     boundaries
 *     flows
 *     bindings
 *     contracts
 *     requirements
 *     capabilities
 *     resources
 *     parameters
 *     interfaces
 *     protocol references
 *     nesting
 *     component relationships
 *
 * The grammar MUST NOT introduce:
 *
 *     MAX_COMPONENTS
 *     MAX_BOUNDARIES
 *     MAX_FLOWS
 *     MAX_BINDINGS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
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
 * Repetition is represented by ANTLR repetition operators.
 *
 * Any practical compiler/resource limitation remains an implementation
 * constraint and MUST NOT become a Zamani language limit.
 *
 * ============================================================================
 * RESOURCE SEMANTICS
 * ============================================================================
 *
 * Co-design distinguishes:
 *
 *     REQUIREMENT
 *         Must be satisfied for the requested semantic realization.
 *
 *     CAPABILITY
 *         A property that an implementation must provide.
 *
 *     RESOURCE
 *         Abstract computational capacity or resource requirement.
 *
 *     CONSTRAINT
 *         A condition restricting legal realization.
 *
 *     PREFERENCE
 *         Advisory optimization intent.
 *
 *     HINT
 *         Non-binding implementation information.
 *
 *     TARGET
 *         An abstract target class/context, not a physical device identity.
 *
 * None of these selects physical hardware.
 *
 * ============================================================================
 * SOFTWARE/HARDWARE BOUNDARY
 * ============================================================================
 *
 * A co-design component may represent:
 *
 *     software;
 *     hardware;
 *     accelerator;
 *     controller;
 *     memory-facing logic;
 *     communication logic;
 *     quantum-facing logic;
 *     distributed logic;
 *     another logical component.
 *
 * The component kind is open-ended.
 *
 * Physical realization is downstream.
 *
 * ============================================================================
 * DATA / CONTROL FLOW
 * ============================================================================
 *
 * Co-design flow syntax expresses logical relationships:
 *
 *     source -> destination;
 *
 * or:
 *
 *     source -> destination : expression;
 *
 * It does NOT define:
 *
 *     physical routing;
 *     network paths;
 *     wires;
 *     buses;
 *     physical channels;
 *     device links;
 *     memory addresses.
 *
 * Semantic analysis determines the meaning of the flow.
 *
 * ============================================================================
 * BINDING
 * ============================================================================
 *
 * A binding connects one logical co-design entity to another source-level
 * entity.
 *
 * Example:
 *
 *     bind host.compute = accelerator.kernel;
 *
 * This is a LOGICAL binding.
 *
 * It does not mean:
 *
 *     GPU 0;
 *     FPGA device 3;
 *     physical core 7;
 *     physical qubit 17.
 *
 * Target-specific binding belongs downstream.
 *
 * ============================================================================
 * CONTRACTS
 * ============================================================================
 *
 * A co-design contract may contain:
 *
 *     requirements;
 *     capabilities;
 *     resources;
 *     constraints;
 *     preferences;
 *     hints;
 *     expressions;
 *     nested contract clauses.
 *
 * Contract semantics remain outside this parser.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Co-design MAY connect:
 *
 *     classical computation
 *          |
 *          v
 *     quantum control
 *          |
 *          v
 *     quantum operation
 *          |
 *          v
 *     measurement
 *          |
 *          v
 *     classical decision
 *
 * This grammar does NOT define quantum operations.
 *
 * Quantum syntax remains owned by:
 *
 *     grammar/quantum/
 *
 * Quantum semantic lowering remains:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * No co-design IR and no second quantum IR are introduced.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * Co-design can reference existing HDL constructs:
 *
 *     modules;
 *     interfaces;
 *     protocols;
 *     ports;
 *     signals;
 *     memories;
 *     pipelines;
 *     timing contracts.
 *
 * This grammar references them symbolically.
 *
 * It does not duplicate their declarations.
 *
 * ============================================================================
 * HARDWARE / RESOURCE INTEGRATION
 * ============================================================================
 *
 * Hardware capability/resource analysis remains downstream.
 *
 * Examples of semantic intent:
 *
 *     requires capability quantum.measurement;
 *     requires capability accelerator.compute;
 *     requires resource memory >= required_memory;
 *     target = heterogeneous;
 *
 * The parser only records source structure.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Each rule is intended to map to domain-neutral frontend structures such as:
 *
 *     HdlCoDesign
 *     HdlCoDesignComponent
 *     HdlCoDesignBoundary
 *     HdlCoDesignFlow
 *     HdlCoDesignBinding
 *     HdlCoDesignContract
 *     HdlCoDesignRequirement
 *     HdlCoDesignCapability
 *     HdlCoDesignResource
 *     HdlCoDesignConstraint
 *     HdlCoDesignPreference
 *     HdlCoDesignHint
 *     HdlCoDesignTarget
 *     HdlCoDesignProperty
 *
 * These are semantic/AST contracts only.
 *
 * This grammar does NOT construct AST nodes.
 *
 * Source spans MUST remain available for every declaration/member.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST determine:
 *
 *     - whether referenced components exist;
 *     - whether component kinds are valid;
 *     - whether bindings are type-compatible;
 *     - whether flows are legal;
 *     - whether interfaces are compatible;
 *     - whether protocols are compatible;
 *     - whether requirements are satisfiable;
 *     - whether capabilities are available;
 *     - whether resources are sufficient;
 *     - whether constraints conflict;
 *     - whether preferences remain advisory;
 *     - whether target intent is realizable;
 *     - whether classical/quantum/HDL boundaries are valid;
 *     - whether quantum semantics can lower through quantum::ir;
 *     - whether a requested realization violates portability rules.
 *
 * The parser performs none of these semantic decisions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT create a co-design-specific permanent IR unless the
 * repository's canonical semantic architecture explicitly introduces one.
 *
 * Preferred path:
 *
 *     co-design syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic co-design model
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL/hardware IR
 *          +--> distributed representation
 *          |
 *          v
 *     canonical lowering
 *
 * Cross-domain relationships may be represented in the canonical semantic
 * model without creating competing domain IRs.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler stages consuming this construct may perform:
 *
 *     interface validation;
 *     capability matching;
 *     resource analysis;
 *     partitioning;
 *     specialization;
 *     lowering;
 *     optimization;
 *     communication planning;
 *     scheduling;
 *     routing;
 *     synthesis;
 *     target selection.
 *
 * These are NOT parser responsibilities.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime may consume semantic results such as:
 *
 *     executable boundaries;
 *     resource requirements;
 *     capability contracts;
 *     communication requirements;
 *     execution policies.
 *
 * Runtime MUST NOT reinterpret source syntax to discover physical topology.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Co-design grammar contains no:
 *
 *     filesystem access;
 *     network access;
 *     shell execution;
 *     compiler actions;
 *     target-language actions;
 *     embedded Rust;
 *     unsafe Rust;
 *     hardware discovery;
 *     environment inspection.
 *
 * Any external resource reference must remain data until validated by
 * semantic/compiler layers.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic predicates;
 *     no embedded actions;
 *     no randomness;
 *     no I/O;
 *     no runtime execution.
 *
 * Parsing is deterministic for a deterministic token stream.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * The grammar avoids:
 *
 *     unbounded lexer modes;
 *     semantic predicates;
 *     embedded actions;
 *     target-dependent parsing;
 *     fixed enumeration of hardware families.
 *
 * Repetition uses ordinary ANTLR constructs.
 *
 * Semantic complexity must be handled by semantic analysis rather than
 * forcing the parser to solve resource or graph problems.
 *
 * ============================================================================
 * PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * The canonical public entry point is:
 *
 *     hdlCoDesignDeclaration
 *
 * The enclosing HDL grammar should expose it through its HDL member/source
 * composition.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 1. CO-DESIGN DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     co_design accelerator_system {
 *         ...
 *     }
 *
 * Generic parameters are optional.
 *
 * The identifier remains the logical co-design unit name.
 *
 * ============================================================================
 */

hdlCoDesignDeclaration
    : attribute*
      CO_DESIGN
      identifier
      hdlCoDesignGenericParameters?
      hdlCoDesignBody
    ;


/*
 * ============================================================================
 * 2. GENERIC PARAMETERS
 * ============================================================================
 *
 * This adapter intentionally consumes the existing hardware generic contract.
 *
 * The actual generic grammar remains owned by:
 *
 *     grammar/hdl/hardware-generics.g4
 *
 * ============================================================================
 */

hdlCoDesignGenericParameters
    : hardwareGenericParameters
    ;


/*
 * ============================================================================
 * 3. BODY
 * ============================================================================
 */

hdlCoDesignBody
    : LBRACE
      hdlCoDesignMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. MEMBER DISPATCH
 * ============================================================================
 */

hdlCoDesignMember
    : hdlCoDesignComponent
    | hdlCoDesignBoundary
    | hdlCoDesignFlow
    | hdlCoDesignBinding
    | hdlCoDesignContract
    | hdlCoDesignRequirement
    | hdlCoDesignCapability
    | hdlCoDesignResource
    | hdlCoDesignConstraint
    | hdlCoDesignPreference
    | hdlCoDesignHint
    | hdlCoDesignTarget
    | hdlCoDesignProperty
    | hdlCoDesignExtension
    ;


/*
 * ============================================================================
 * 5. LOGICAL COMPONENT
 * ============================================================================
 *
 * The component kind is intentionally a qualified name.
 *
 * Examples:
 *
 *     software.host;
 *     hardware.accelerator;
 *     quantum.controller;
 *     distributed.worker;
 *     future.compute.substrate;
 *
 * No closed hardware taxonomy is encoded.
 * ============================================================================
 */

hdlCoDesignComponent
    : attribute*
      identifier
      COLON
      qualifiedName
      hdlCoDesignComponentArguments?
      SEMICOLON
    ;


hdlCoDesignComponentArguments
    : LPAREN
      hdlCoDesignArgumentList?
      RPAREN
    ;


hdlCoDesignArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 6. SOFTWARE/HARDWARE BOUNDARY
 * ============================================================================
 *
 * A boundary identifies a logical interface contract between components.
 *
 * It does not identify a physical pin, bus, memory bank, network endpoint,
 * device channel, or QPU connection.
 *
 * ============================================================================
 */

hdlCoDesignBoundary
    : attribute*
      INTERFACE
      identifier
      (
          COLON
          qualifiedName
      )?
      hdlCoDesignBoundaryBody?
      SEMICOLON?
    ;


hdlCoDesignBoundaryBody
    : LBRACE
      hdlCoDesignBoundaryMember*
      RBRACE
    ;


hdlCoDesignBoundaryMember
    : hdlCoDesignRequirement
    | hdlCoDesignCapability
    | hdlCoDesignConstraint
    | hdlCoDesignPreference
    | hdlCoDesignHint
    | hdlCoDesignProperty
    ;


/*
 * ============================================================================
 * 7. LOGICAL FLOW
 * ============================================================================
 *
 * A flow represents a logical relationship.
 *
 * Examples:
 *
 *     host -> accelerator;
 *
 *     host.result -> accelerator.input : transfer_size;
 *
 * The expression after ':' is semantic metadata.
 * ============================================================================
 */

hdlCoDesignFlow
    : attribute*
      hdlCoDesignEndpoint
      ARROW
      hdlCoDesignEndpoint
      hdlCoDesignFlowMetadata?
      SEMICOLON
    ;


hdlCoDesignFlowMetadata
    : COLON
      expression
    ;


hdlCoDesignEndpoint
    : qualifiedName
    ;


/*
 * ============================================================================
 * 8. LOGICAL BINDING
 * ============================================================================
 *
 * A binding connects a co-design name to another logical entity.
 *
 * Example:
 *
 *     host.compute = software::compute;
 *
 * This is NOT physical placement.
 * ============================================================================
 */

hdlCoDesignBinding
    : attribute*
      K_BIND
      hdlCoDesignBindingTarget
      ASSIGN
      hdlCoDesignBindingSource
      SEMICOLON
    ;


hdlCoDesignBindingTarget
    : qualifiedName
    ;


hdlCoDesignBindingSource
    : qualifiedName
      hdlCoDesignBindingArguments?
    ;


hdlCoDesignBindingArguments
    : LPAREN
      hdlCoDesignArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 9. CONTRACT
 * ============================================================================
 */

hdlCoDesignContract
    : attribute*
      CONTRACT
      identifier
      hdlCoDesignContractBody
    ;


hdlCoDesignContractBody
    : LBRACE
      hdlCoDesignContractMember*
      RBRACE
    ;


hdlCoDesignContractMember
    : hdlCoDesignRequirement
    | hdlCoDesignCapability
    | hdlCoDesignResource
    | hdlCoDesignConstraint
    | hdlCoDesignPreference
    | hdlCoDesignHint
    | hdlCoDesignProperty
    | hdlCoDesignFlow
    ;


/*
 * ============================================================================
 * 10. REQUIREMENT
 * ============================================================================
 *
 * The expression remains open-ended.
 *
 * This prevents this grammar from creating another resource-expression
 * language.
 * ============================================================================
 */

hdlCoDesignRequirement
    : REQUIRES
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. CAPABILITY
 * ============================================================================
 *
 * `capability` is a semantic capability reference.
 *
 * The capability identity itself remains open-world.
 * ============================================================================
 */

hdlCoDesignCapability
    : CAPABILITY
      hdlCoDesignCapabilityReference
      SEMICOLON
    ;


hdlCoDesignCapabilityReference
    : qualifiedName
      hdlCoDesignArgumentList?
    ;


/*
 * ============================================================================
 * 12. RESOURCE
 * ============================================================================
 *
 * Resource intent remains expression-based.
 *
 * Examples:
 *
 *     resource memory >= required_memory;
 *     resource compute = workload;
 *
 * The expression is interpreted semantically.
 * ============================================================================
 */

hdlCoDesignResource
    : RESOURCE
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. CONSTRAINT
 * ============================================================================
 */

hdlCoDesignConstraint
    : CONSTRAINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. PREFERENCE
 * ============================================================================
 */

hdlCoDesignPreference
    : PREFER
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. HINT
 * ============================================================================
 */

hdlCoDesignHint
    : HINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. TARGET INTENT
 * ============================================================================
 *
 * A target is symbolic.
 *
 * It does not identify a physical device.
 * ============================================================================
 */

hdlCoDesignTarget
    : TARGET
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. PROPERTY
 * ============================================================================
 *
 * Generic properties permit forward-compatible metadata without turning every
 * future concept into a keyword.
 *
 * Examples:
 *
 *     latency = requirement;
 *     throughput = desired;
 *     locality = symbolic_property;
 * ============================================================================
 */

hdlCoDesignProperty
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. OPEN EXTENSION BLOCK
 * ============================================================================
 *
 * Future co-design domains may attach named semantic extension blocks without
 * modifying this grammar.
 *
 * Example:
 *
 *     timing {
 *         ...
 *     }
 *
 *     resilience {
 *         ...
 *     }
 *
 *     quantum_control {
 *         ...
 *     }
 *
 * The block name is data.
 *
 * Semantic ownership belongs to the appropriate subsystem.
 * ============================================================================
 */

hdlCoDesignExtension
    : identifier
      hdlCoDesignExtensionArguments?
      hdlCoDesignExtensionBody
    ;


hdlCoDesignExtensionArguments
    : LPAREN
      hdlCoDesignArgumentList?
      RPAREN
    ;


hdlCoDesignExtensionBody
    : LBRACE
      hdlCoDesignExtensionMember*
      RBRACE
    ;


hdlCoDesignExtensionMember
    : hdlCoDesignRequirement
    | hdlCoDesignCapability
    | hdlCoDesignResource
    | hdlCoDesignConstraint
    | hdlCoDesignPreference
    | hdlCoDesignHint
    | hdlCoDesignProperty
    | hdlCoDesignFlow
    | hdlCoDesignBinding
    ;


/*
 * ============================================================================
 * 19. EMBEDDING CONTRACT
 * ============================================================================
 *
 * hdl.g4 MUST expose:
 *
 *     hdlCoDesignDeclaration
 *
 * through its source/member composition.
 *
 * It MUST NOT duplicate the co-design rules.
 *
 * The intended integration is:
 *
 *     hdlSourceUnit
 *         |
 *         +--> hdlCoDesignDeclaration
 *
 * and, where co-design declarations are legal inside HDL modules:
 *
 *     hdlModuleMemberCore
 *         |
 *         +--> hdlCoDesignDeclaration
 *
 * Whether co-design declarations are permitted at both levels is a semantic
 * language decision documented by grammar/spec/hdl.md.
 *
 * ============================================================================
 * 20. EXISTING HDL INTEGRATION
 * ============================================================================
 *
 * Existing grammar components remain authoritative for their own constructs:
 *
 *     hardware-modules.g4
 *         -> module structure
 *
 *     hardware-generics.g4
 *         -> hardware generic parameters
 *
 *     ports.g4
 *         -> ports
 *
 *     interfaces.g4
 *         -> interfaces
 *
 *     protocols.g4
 *         -> protocol contracts
 *
 *     memories.g4
 *         -> logical memories
 *
 *     timing.g4
 *         -> timing intent
 *
 *     assertions.g4
 *         -> verification properties
 *
 * Co-design references these concepts rather than copying their grammar.
 *
 * ============================================================================
 * 21. CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Classical:
 *
 *     component software.host: classical.compute(...);
 *
 * Quantum:
 *
 *     component quantum.controller: quantum.control(...);
 *
 * Hybrid:
 *
 *     classical -> quantum.controller;
 *
 * Hardware:
 *
 *     component accelerator: hardware.accelerator(...);
 *
 * Distributed:
 *
 *     component worker: distributed.worker(...);
 *
 * These are source-level semantic identities.
 *
 * No backend is selected by their names.
 *
 * ============================================================================
 * 22. QUANTUM INTEGRATION CONTRACT
 * ============================================================================
 *
 * A co-design declaration may reference quantum capabilities and logical
 * quantum components.
 *
 * Example:
 *
 *     requires capability quantum.mid_circuit_measurement;
 *
 * or:
 *
 *     quantum_controller -> classical_controller;
 *
 * Semantic analysis is responsible for connecting the relationship to:
 *
 *     quantum::ir
 *
 * No co-design quantum IR may be created.
 *
 * ============================================================================
 * 23. RESOURCE INTEGRATION CONTRACT
 * ============================================================================
 *
 * Resource semantics must be resolved through the existing resource and
 * hardware capability systems.
 *
 * Preferred semantic chain:
 *
 *     hdlCoDesignResource
 *          |
 *          v
 *     resource/capability semantic model
 *          |
 *          v
 *     hardware capability/resource analysis
 *          |
 *          v
 *     target realization
 *
 * This grammar must not import or duplicate resource implementation rules.
 *
 * ============================================================================
 * 24. AST / SEMANTIC / IR COMPLETION
 * ============================================================================
 *
 * A production implementation is complete only when every public rule has:
 *
 *     - AST mapping;
 *     - source-span preservation;
 *     - semantic ownership;
 *     - diagnostic ownership;
 *     - IR/lowering mapping;
 *     - positive tests;
 *     - negative tests;
 *     - boundary tests;
 *     - scalability tests;
 *     - determinism tests;
 *     - compatibility tests.
 *
 * ============================================================================
 * 25. REQUIRED POSITIVE TESTS
 * ============================================================================
 *
 * At minimum:
 *
 *     - empty co-design body;
 *     - one component;
 *     - many components;
 *     - parameterized co-design;
 *     - interface boundary;
 *     - nested boundary;
 *     - logical flow;
 *     - flow metadata;
 *     - logical binding;
 *     - contract;
 *     - requirement;
 *     - capability;
 *     - resource;
 *     - constraint;
 *     - preference;
 *     - hint;
 *     - target;
 *     - arbitrary property;
 *     - extension block;
 *     - classical/hardware composition;
 *     - classical/quantum composition;
 *     - hardware/quantum composition;
 *     - distributed/hardware composition;
 *     - deeply nested contracts.
 *
 * ============================================================================
 * 26. REQUIRED NEGATIVE TESTS
 * ============================================================================
 *
 * Reject semantically invalid or syntactically malformed forms including:
 *
 *     - missing co-design name;
 *     - missing body;
 *     - missing component name;
 *     - missing component kind;
 *     - missing flow endpoint;
 *     - missing binding assignment;
 *     - missing contract body;
 *     - missing requirement expression;
 *     - missing capability name;
 *     - missing resource expression;
 *     - missing target expression;
 *     - malformed delimiters;
 *     - malformed qualified names.
 *
 * Semantic tests must additionally reject:
 *
 *     - unknown component references;
 *     - incompatible bindings;
 *     - incompatible flows;
 *     - unsatisfied requirements;
 *     - unavailable capabilities;
 *     - contradictory constraints.
 *
 * ============================================================================
 * 27. BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *     one component;
 *     two components;
 *     many components;
 *     one flow;
 *     many flows;
 *     empty contracts;
 *     deeply nested contracts;
 *     large expressions;
 *     symbolic quantities;
 *     parameterized quantities;
 *     arbitrary qualified names;
 *     Unicode identifiers where supported.
 *
 * No boundary test may establish a universal maximum.
 *
 * ============================================================================
 * 28. SCALABILITY TESTS
 * ============================================================================
 *
 * The test suite MUST include generated source sizes that grow according to
 * available test infrastructure.
 *
 * Scaling dimensions include:
 *
 *     component count;
 *     boundary count;
 *     flow count;
 *     binding count;
 *     contract count;
 *     requirement count;
 *     capability count;
 *     nesting depth;
 *     expression size.
 *
 * The grammar MUST remain correct regardless of the selected test scale.
 *
 * ============================================================================
 * 29. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST pass a repository-wide hard-coding audit.
 *
 * Forbidden universal capacity concepts include:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Also forbidden:
 *
 *     physical_gpu(0)
 *     physical_qubit(0)
 *     cpu_core(0)
 *     fpga_bank(0)
 *     device_0
 *
 * when presented as universal language semantics.
 *
 * A numeric literal remains legal when it is explicitly part of source
 * semantics.
 *
 * ============================================================================
 * 30. DIAGNOSTICS
 * ============================================================================
 *
 * Diagnostics should identify:
 *
 *     co-design declaration;
 *     component;
 *     boundary;
 *     flow;
 *     binding;
 *     contract;
 *     requirement;
 *     capability;
 *     resource;
 *     constraint;
 *     preference;
 *     hint;
 *     target;
 *     property;
 *     extension.
 *
 * Diagnostics MUST retain source spans.
 *
 * ============================================================================
 * 31. SAFE RUST
 * ============================================================================
 *
 * The grammar contains no embedded Rust.
 *
 * Implementations consuming it MUST target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST NOT use:
 *
 *     unsafe
 *
 * ============================================================================
 * 32. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE only when:
 *
 *     [ ] canonical CO_DESIGN lexical token exists;
 *     [ ] canonical ZamaniLexer exposes it;
 *     [ ] HdlCoDesign is composed by the HDL dispatcher;
 *     [ ] ZamaniParser reaches it through HDL composition;
 *     [ ] shared identifiers/names are reused;
 *     [ ] shared expressions are reused;
 *     [ ] shared types are reused;
 *     [ ] hardware generics are reused;
 *     [ ] no duplicate module grammar exists here;
 *     [ ] no duplicate interface grammar exists here;
 *     [ ] no duplicate protocol grammar exists here;
 *     [ ] no duplicate resource grammar exists here;
 *     [ ] no duplicate capability grammar exists here;
 *     [ ] AST mappings are defined;
 *     [ ] semantic mappings are defined;
 *     [ ] IR mappings are defined;
 *     [ ] diagnostics are defined;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] determinism tests exist;
 *     [ ] compatibility tests exist;
 *     [ ] hard-coding audit passes;
 *     [ ] no unsafe Rust is required;
 *     [ ] no physical hardware assumptions are encoded;
 *     [ ] no competing co-design root exists.
 *
 * ============================================================================
 */