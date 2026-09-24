/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hybrid/host-device.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Status:
 *     Production-ready host/device interoperability composition boundary.
 *
 * Toolchain baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL HOST/DEVICE COMPUTATION BOUNDARY.
 *
 * It describes portable semantic relationships between:
 *
 *     host-side computation
 *
 * and:
 *
 *     device-side / accelerator-side computation.
 *
 * "Host" and "device" are semantic execution roles. They are NOT physical
 * machine identities.
 *
 * A device may ultimately be:
 *
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     DSP
 *     tensor accelerator
 *     QPU
 *     quantum simulator
 *     embedded accelerator
 *     distributed accelerator
 *     future computational substrate
 *
 * A host may likewise be any execution context capable of coordinating the
 * requested computation.
 *
 * This file therefore MUST NOT encode a particular:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     node
 *     device ID
 *     memory address
 *     bus
 *     topology
 *     vendor
 *     operating system
 *     deployment location.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     parser composition
 *          |
 *          +--> expressions
 *          +--> types
 *          +--> statements
 *          +--> memory
 *          +--> execution
 *          +--> resources
 *          +--> hardware
 *          +--> accelerator interoperability
 *          |
 *          v
 *     THIS FILE
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *     classical semantics             device semantics
 *          |                               |
 *          |                               +--> quantum::ir
 *          |                               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *                    canonical IR
 *                          |
 *                          v
 *                     optimization
 *                          |
 *              +-----------+-----------+
 *              |           |           |
 *              v           v           v
 *           routing   scheduling   resilience
 *                                      |
 *                                      +--> QEC
 *                                      +--> ZQN
 *                          |
 *                          v
 *                         HAL
 *                          |
 *                          v
 *                   target realization
 *
 * This grammar creates NO IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - host/device semantic boundary syntax;
 *     - host/device regions;
 *     - host/device role annotations;
 *     - host-to-device transfer intent;
 *     - device-to-host transfer intent;
 *     - bidirectional transfer intent;
 *     - host/device invocation composition;
 *     - host/device execution composition;
 *     - host/device synchronization intent;
 *     - host/device value bindings;
 *     - host/device capability and requirement composition;
 *     - host/device boundary metadata.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - keywords;
 *     - identifiers;
 *     - expression precedence;
 *     - general expressions;
 *     - types;
 *     - ordinary statements;
 *     - memory semantics;
 *     - ownership semantics;
 *     - accelerator operation definitions;
 *     - quantum operations;
 *     - quantum measurements;
 *     - quantum gates;
 *     - HDL syntax;
 *     - physical device selection;
 *     - hardware topology;
 *     - resource allocation;
 *     - scheduling;
 *     - routing;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - runtime implementation;
 *     - canonical classical IR;
 *     - quantum::ir.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * Existing repository ownership remains authoritative:
 *
 *     grammar/hybrid/accelerator-interoperability.g4
 *         generic accelerator interfaces, bindings, invocation, transfer and
 *         execution intent.
 *
 *     grammar/hybrid/quantum-classical-control.g4
 *         hybrid quantum/classical control composition.
 *
 *     grammar/hybrid/classical-quantum.g4
 *         existing classical/quantum source boundary.
 *
 *     grammar/quantum/*
 *         quantum-domain syntax.
 *
 *     grammar/memory/*
 *         memory semantics and memory-domain syntax.
 *
 *     grammar/execution/*
 *         execution semantics.
 *
 *     grammar/resources/*
 *         resource/capability requirements.
 *
 *     grammar/hardware/*
 *         target-independent hardware intent.
 *
 * This file MUST NOT copy those grammars' implementations.
 *
 * ============================================================================
 * IMPORTANT DESIGN DECISION
 * ============================================================================
 *
 * No HOST or DEVICE keyword is introduced here.
 *
 * The repository already treats lexical vocabulary as a centralized concern.
 * Introducing another pair of global keywords merely for this boundary would
 * unnecessarily increase lexical coupling.
 *
 * Instead, host/device roles are represented by:
 *
 *     @host
 *     @device
 *
 * where `host` and `device` are ordinary semantic identifiers.
 *
 * The parser accepts the role name structurally.
 *
 * Semantic analysis validates whether the annotation denotes one of the
 * reserved execution roles for this grammar.
 *
 * This keeps the lexical language open for future roles such as:
 *
 *     @controller
 *     @qpu
 *     @accelerator
 *     @cluster
 *     @remote
 *     @simulator
 *     @embedded
 *     @future
 *
 * without requiring a lexer change for every new execution class.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This grammar expresses:
 *
 *     semantic execution roles
 *     logical data movement
 *     computation boundaries
 *     dependency relationships
 *     synchronization intent
 *     capability requirements
 *     resource requirements
 *     implementation preferences
 *
 * It MUST NOT express:
 *
 *     physical device IDs
 *     CPU IDs
 *     GPU IDs
 *     FPGA IDs
 *     QPU IDs
 *     physical qubit IDs
 *     memory addresses
 *     PCIe paths
 *     NVLink paths
 *     bus identifiers
 *     fixed topology
 *     fixed device counts
 *     fixed memory capacities
 *     fixed register widths
 *     fixed thread counts
 *     fixed accelerator counts.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains NO artificial machine limits.
 *
 * It does not impose limits on:
 *
 *     host count
 *     device count
 *     transfers
 *     buffers
 *     values
 *     operations
 *     arguments
 *     results
 *     nested regions
 *     synchronization points
 *     concurrent operations
 *     nodes
 *     memory
 *     threads
 *     accelerators
 *     qubits
 *     tensor dimensions.
 *
 * Repetition is structural:
 *
 *     *
 *     +
 *
 * and not represented by finite enumerations.
 *
 * Actual resource limitations belong to:
 *
 *     compiler resources
 *     semantic resource validation
 *     deployment
 *     scheduler
 *     runtime
 *     HAL
 *     target capabilities.
 *
 * ============================================================================
 * NO HARD-CODED HARDWARE
 * ============================================================================
 *
 * Forbidden universal constructs include:
 *
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *
 * Also forbidden:
 *
 *     gpu0
 *     cpu0
 *     qpu0
 *     device0
 *     physical_qubit_0
 *
 * as implicit universal hardware identities.
 *
 * A source program may contain a user-defined symbolic name that happens to
 * contain such text, but the grammar must never interpret it as a physical
 * resource unless an explicitly downstream target-specific representation
 * says so.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE SEPARATION
 * ============================================================================
 *
 * Host/device syntax may participate in three distinct semantic categories:
 *
 * REQUIREMENT:
 *
 *     requires capability("tensor.compute");
 *
 * CAPABILITY:
 *
 *     capability("quantum.measurement");
 *
 * PREFERENCE:
 *
 *     prefer capability("accelerated.compute");
 *
 * The parser only recognizes structure.
 *
 * Semantic analysis determines:
 *
 *     mandatory vs advisory
 *     satisfiability
 *     compatibility
 *     portability
 *     lowering consequences.
 *
 * ============================================================================
 * DATA-MOVEMENT SEMANTICS
 * ============================================================================
 *
 * A transfer expresses logical value movement:
 *
 *     source
 *         |
 *         v
 *     destination
 *
 * It does NOT specify:
 *
 *     memcpy
 *     DMA
 *     PCIe
 *     NVLink
 *     shared-memory mapping
 *     cache operation
 *     network packet
 *     physical bus transaction.
 *
 * The selected implementation may use any mechanism that preserves program
 * semantics.
 *
 * ============================================================================
 * HOST-TO-DEVICE / DEVICE-TO-HOST
 * ============================================================================
 *
 * Direction is represented semantically by endpoint annotations:
 *
 *     transfer @host value to @device target;
 *
 *     transfer @device result to @host output;
 *
 *     transfer @host input to @device buffer;
 *
 *     transfer @device result to @host result;
 *
 * The grammar does not assign a physical implementation to either direction.
 *
 * ============================================================================
 * BIDIRECTIONAL DATA
 * ============================================================================
 *
 * A bidirectional boundary may be represented with:
 *
 *     transfer @host value to @device buffer;
 *     transfer @device buffer to @host value;
 *
 * or by a single semantic binding whose implementation is allowed to
 * synchronize both directions.
 *
 * This grammar does not introduce a fixed coherence model.
 *
 * ============================================================================
 * MEMORY / OWNERSHIP
 * ============================================================================
 *
 * This grammar does not redefine:
 *
 *     ownership
 *     borrowing
 *     lifetimes
 *     allocation
 *     shared memory
 *     distributed memory
 *     accelerator memory.
 *
 * Those remain owned by `grammar/memory/` and the semantic type/ownership
 * system.
 *
 * A host/device boundary may reference values whose memory semantics are
 * defined elsewhere.
 *
 * ============================================================================
 * SYNCHRONIZATION
 * ============================================================================
 *
 * Host/device synchronization expresses a dependency boundary.
 *
 * It does NOT encode:
 *
 *     clock frequency
 *     device latency
 *     pulse duration
 *     queue implementation
 *     barrier hardware
 *     host polling strategy
 *     interrupt mechanism.
 *
 * Example:
 *
 *     synchronize @host @device;
 *
 * means only that the semantic execution model requires synchronization
 * between the named roles.
 *
 * The scheduler determines how.
 *
 * ============================================================================
 * EXECUTION
 * ============================================================================
 *
 * Execution syntax identifies a logical execution subject and role.
 *
 * It does NOT select a physical target.
 *
 * Example:
 *
 *     execute computation on @device;
 *
 * means that semantic execution is associated with the device role.
 *
 * The compiler/HAL may realize that role on:
 *
 *     GPU
 *     FPGA
 *     QPU
 *     CPU
 *     simulator
 *     accelerator
 *     distributed service
 *     future target.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * If a device-side computation is quantum:
 *
 *     frontend AST
 *          |
 *          v
 *     semantic quantum operation
 *          |
 *          v
 *     quantum::ir
 *
 * This grammar MUST NOT introduce:
 *
 *     QuantumGate
 *     QubitId
 *     PhysicalQubitId
 *     QuantumCircuit
 *     QuantumInstruction
 *
 * or another quantum IR.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * A device role may ultimately represent an HDL/hardware realization.
 *
 * This file does not parse:
 *
 *     wires
 *     registers
 *     clocks
 *     RTL
 *     HDL modules
 *     synthesis directives.
 *
 * Those remain under `grammar/hdl/`.
 *
 * The host/device boundary may only express the semantic relationship.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Host/device roles may later be realized across:
 *
 *     one machine
 *     multiple machines
 *     a cluster
 *     an edge device
 *     a cloud deployment
 *     heterogeneous distributed hardware.
 *
 * This grammar therefore does not encode node IDs or node counts.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every host/device construct must map to domain-neutral frontend AST data
 * containing, as applicable:
 *
 *     source span
 *     source role
 *     destination role
 *     subject expression
 *     value expression
 *     optional type
 *     optional metadata
 *     optional capability requirements
 *     optional resource requirements
 *     optional synchronization intent
 *     optional execution intent.
 *
 * The AST MUST NOT contain:
 *
 *     physical device IDs
 *     physical addresses
 *     backend handles
 *     scheduling slots
 *     routing paths
 *     pulse identifiers
 *     calibration records.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must validate:
 *
 *     - endpoint role correctness;
 *     - value existence;
 *     - source/destination compatibility;
 *     - type compatibility;
 *     - ownership compatibility;
 *     - lifetime validity;
 *     - transfer legality;
 *     - synchronization dependencies;
 *     - execution legality;
 *     - capability requirements;
 *     - resource requirements;
 *     - domain crossings;
 *     - quantum/classical compatibility where applicable;
 *     - portability constraints.
 *
 * A syntactically valid host/device boundary is not automatically semantically
 * valid.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file creates no IR.
 *
 * Semantic lowering determines the appropriate canonical representation.
 *
 * Possible downstream destinations include:
 *
 *     canonical classical representation
 *     quantum::ir
 *     HDL/hardware representation
 *     execution representation
 *     memory representation
 *     distributed representation.
 *
 * The host/device relationship itself is a semantic dependency, not a second
 * intermediate representation.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Errors that belong to semantic analysis MUST NOT be encoded as parser
 * restrictions.
 *
 * Examples:
 *
 *     unsupported device capability
 *     insufficient memory
 *     unavailable accelerator
 *     unsupported transfer
 *     incompatible type
 *     invalid ownership transfer
 *     impossible synchronization
 *
 * These are semantic/resource/target errors, not syntax errors.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source
 *     token stream
 *     grammar
 *     language version.
 *
 * It must not depend on:
 *
 *     hardware availability
 *     device count
 *     runtime state
 *     network state
 *     current time
 *     randomness
 *     environment variables.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This file contains no embedded Rust actions.
 *
 * It therefore requires no unsafe Rust.
 *
 * Repository implementations consuming this grammar remain subject to:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes:
 *
 *     ZamaniLexer
 *
 * It imports only lower-level parser contracts needed for generic expressions,
 * types, statements, and accelerator interoperability.
 *
 * It MUST NOT import:
 *
 *     Zamani.g4
 *     ZamaniParser.g4
 *
 * The canonical parser composition layer owns reachability.
 *
 * ============================================================================
 */

parser grammar HostDevice;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions, Types, Statements, AcceleratorInteroperability;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The canonical hybrid composition layer should consume:
 *
 *     hostDeviceConstruct
 *
 * and nothing else from this grammar.
 */
hostDeviceConstruct
    : hostDeviceRegion
    | hostDeviceTransfer
    | hostDeviceInvocation
    | hostDeviceExecution
    | hostDeviceBinding
    | hostDeviceSynchronization
    | hostDeviceRequirement
    | hostDeviceCapability
    | hostDevicePreference
    | hostDeviceConstraint
    | hostDeviceHint
    ;


/*
 * ============================================================================
 * 2. HOST/DEVICE REGION
 * ============================================================================
 *
 * A region groups host/device interactions without implying:
 *
 *     thread
 *     process
 *     queue
 *     device
 *     hardware controller
 *     scheduling domain.
 *
 * Example:
 *
 *     @host @device {
 *         ...
 *     }
 *
 * The annotations identify semantic roles only.
 */
hostDeviceRegion
    : hostDeviceRoleAnnotation+
      LBRACE
      hostDeviceRegionItem*
      RBRACE
    ;


hostDeviceRegionItem
    : hostDeviceConstruct
    | statement
    ;


/*
 * ============================================================================
 * 3. ROLE ANNOTATION
 * ============================================================================
 *
 * No HOST or DEVICE lexer tokens are introduced.
 *
 * The role name is an identifier.
 *
 * Semantic validation recognizes:
 *
 *     host
 *     device
 *
 * as the standard roles for this grammar.
 *
 * The open identifier form permits future execution roles without changing
 * the lexical vocabulary.
 */
hostDeviceRoleAnnotation
    : AT
      IDENTIFIER
    ;


/*
 * ============================================================================
 * 4. ENDPOINT
 * ============================================================================
 *
 * An endpoint consists of:
 *
 *     role annotation + value
 *
 * Example:
 *
 *     @host input
 *
 *     @device buffer
 *
 * The value remains a canonical Zamani expression.
 */
hostDeviceEndpoint
    : hostDeviceRoleAnnotation
      expression
    ;


/*
 * ============================================================================
 * 5. TRANSFER
 * ============================================================================
 *
 * This is the primary host/device data boundary.
 *
 * Examples:
 *
 *     transfer @host input to @device buffer;
 *
 *     transfer @device result to @host output;
 *
 * The physical transfer mechanism remains unspecified.
 *
 * The semantic layer determines whether the operation is:
 *
 *     copy
 *     move
 *     borrow
 *     shared
 *     streamed
 *     remote
 *     device-local
 *     zero-copy
 *     staged
 *     deferred
 *     otherwise realized.
 *
 * Existing accelerator interoperability owns generic transfer semantics;
 * this rule adds only the host/device role boundary.
 */
hostDeviceTransfer
    : 'transfer'
      hostDeviceEndpoint
      'to'
      hostDeviceEndpoint
      hostDeviceTransferClause*
      SEMICOLON
    ;


hostDeviceTransferClause
    : hostDeviceAsClause
    | hostDeviceUsingClause
    | hostDeviceRequirementClause
    | hostDeviceCapabilityClause
    | hostDevicePreferenceClause
    | hostDeviceConstraintClause
    | hostDeviceHintClause
    ;


hostDeviceAsClause
    : 'as'
      typeExpr
    ;


hostDeviceUsingClause
    : 'using'
      qualifiedName
    ;


hostDeviceRequirementClause
    : 'requires'
      expression
    ;


hostDeviceCapabilityClause
    : 'capability'
      LPAREN
      expression
      RPAREN
    ;


hostDevicePreferenceClause
    : 'prefer'
      expression
    ;


hostDeviceConstraintClause
    : 'constraint'
      expression
    ;


hostDeviceHintClause
    : 'hint'
      expression
    ;


/*
 * ============================================================================
 * 6. HOST/DEVICE INVOCATION
 * ============================================================================
 *
 * A logical operation may execute in a specified semantic role.
 *
 * Example:
 *
 *     invoke operation(arg) on @device;
 *
 * The operation remains a qualified semantic name.
 *
 * No finite operation inventory is created.
 */
hostDeviceInvocation
    : 'invoke'
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
      hostDeviceInvocationClause*
      SEMICOLON
    ;


hostDeviceInvocationClause
    : hostDeviceRoleClause
    | hostDeviceInputClause
    | hostDeviceOutputClause
    | hostDeviceRequirementClause
    | hostDeviceCapabilityClause
    | hostDevicePreferenceClause
    | hostDeviceConstraintClause
    | hostDeviceHintClause
    ;


hostDeviceRoleClause
    : 'on'
      hostDeviceRoleAnnotation
    ;


hostDeviceInputClause
    : 'inputs'
      LPAREN
      expressionList?
      RPAREN
    ;


hostDeviceOutputClause
    : 'outputs'
      LPAREN
      expressionList?
      RPAREN
    ;


/*
 * ============================================================================
 * 7. EXECUTION
 * ============================================================================
 *
 * Execution intent is expressed independently from physical realization.
 *
 * Example:
 *
 *     execute computation on @device;
 *
 * The subject is a semantic expression.
 */
hostDeviceExecution
    : 'execute'
      expression
      hostDeviceExecutionClause*
      SEMICOLON
    ;


hostDeviceExecutionClause
    : hostDeviceRoleClause
    | hostDeviceInputClause
    | hostDeviceOutputClause
    | hostDeviceRequirementClause
    | hostDeviceCapabilityClause
    | hostDevicePreferenceClause
    | hostDeviceConstraintClause
    | hostDeviceHintClause
    | hostDeviceCompletionClause
    ;


hostDeviceCompletionClause
    : 'on'
      'completion'
      block
    ;


/*
 * ============================================================================
 * 8. CROSS-BOUNDARY VALUE BINDING
 * ============================================================================
 *
 * A binding associates a semantic value with a host/device boundary.
 *
 * Examples:
 *
 *     let result = @device computation;
 *
 *     let output = @host result;
 *
 * The exact value-domain legality is semantic.
 */
hostDeviceBinding
    : 'let'
      IDENTIFIER
      typeAnnotation?
      ASSIGN
      hostDeviceBoundValue
      SEMICOLON
    ;


hostDeviceBoundValue
    : hostDeviceAnnotatedValue
    | expression
    ;


hostDeviceAnnotatedValue
    : hostDeviceRoleAnnotation
      expression
    ;


/*
 * ============================================================================
 * 9. SYNCHRONIZATION
 * ============================================================================
 *
 * Examples:
 *
 *     synchronize @host @device;
 *
 *     synchronize @host @device (condition);
 *
 * Synchronization describes semantic dependency only.
 */
hostDeviceSynchronization
    : 'synchronize'
      hostDeviceRoleAnnotation+
      hostDeviceSynchronizationCondition?
      SEMICOLON
    ;


hostDeviceSynchronizationCondition
    : LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 10. REQUIREMENT
 * ============================================================================
 *
 * Requirements remain target-independent.
 *
 * Examples:
 *
 *     requires capability("accelerated.compute");
 *
 *     requires memory >= required_memory;
 *
 *     requires qubits >= n;
 *
 * No physical allocation follows from this grammar.
 */
hostDeviceRequirement
    : 'requires'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. CAPABILITY
 * ============================================================================
 */
hostDeviceCapability
    : 'capability'
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. PREFERENCE
 * ============================================================================
 *
 * Preferences are advisory.
 *
 * They must not silently become mandatory hardware selection.
 */
hostDevicePreference
    : 'prefer'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. CONSTRAINT
 * ============================================================================
 */
hostDeviceConstraint
    : 'constraint'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. HINT
 * ============================================================================
 *
 * A hint may influence downstream optimization but cannot change program
 * semantics.
 */
hostDeviceHint
    : 'hint'
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. HOST -> DEVICE
 * ============================================================================
 *
 * Named AST-facing boundary.
 *
 * This is an alias over the transfer model and does not create another
 * transfer implementation.
 */
hostToDevice
    : 'transfer'
      hostDeviceHostEndpoint
      'to'
      hostDeviceDeviceEndpoint
      hostDeviceTransferClause*
      SEMICOLON
    ;


hostDeviceHostEndpoint
    : hostDeviceRoleAnnotation
      expression
    ;


hostDeviceDeviceEndpoint
    : hostDeviceRoleAnnotation
      expression
    ;


/*
 * ============================================================================
 * 16. DEVICE -> HOST
 * ============================================================================
 *
 * This rule is intentionally structurally symmetric with hostToDevice.
 *
 * Semantic analysis validates the role names.
 */
deviceToHost
    : 'transfer'
      hostDeviceDeviceEndpoint
      'to'
      hostDeviceHostEndpoint
      hostDeviceTransferClause*
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. BIDIRECTIONAL BOUNDARY
 * ============================================================================
 *
 * The grammar represents bidirectional interaction as a pair of semantic
 * transfers.
 *
 * No special physical coherence model is implied.
 */
hostDeviceBidirectional
    : hostToDevice
      deviceToHost
    ;


/*
 * ============================================================================
 * 18. HOST/DEVICE SEQUENCE
 * ============================================================================
 *
 * Unbounded structural composition.
 */
hostDeviceSequence
    : hostDeviceSequenceItem*
    ;


hostDeviceSequenceItem
    : hostDeviceConstruct
    | statement
    ;


/*
 * ============================================================================
 * 19. HOST/DEVICE GROUP
 * ============================================================================
 *
 * Structural grouping only.
 */
hostDeviceGroup
    : LBRACE
      hostDeviceSequence
      RBRACE
    ;


/*
 * ============================================================================
 * 20. HOST/DEVICE ROLE PAIR
 * ============================================================================
 *
 * Useful to AST and tooling consumers that need explicit role relationships.
 */
hostDeviceRolePair
    : hostDeviceRoleAnnotation
      hostDeviceRoleAnnotation
    ;


/*
 * ============================================================================
 * 21. HOST/DEVICE EXPRESSION
 * ============================================================================
 *
 * Reuses the canonical expression language.
 *
 * No second expression precedence hierarchy is introduced.
 */
hostDeviceExpression
    : expression
    | hostDeviceAnnotatedValue
    ;


/*
 * ============================================================================
 * 22. HOST/DEVICE TYPE CONTEXT
 * ============================================================================
 *
 * Reuses the canonical type language.
 */
hostDeviceTypeContext
    : typeExpr
    ;


/*
 * ============================================================================
 * 23. ACCELERATOR INTEROPERABILITY ADAPTER
 * ============================================================================
 *
 * Generic accelerator interoperability already owns:
 *
 *     acceleratorTransferDeclaration
 *     acceleratorOperationDeclaration
 *     acceleratorExecutionDeclaration
 *
 * This adapter allows tooling to treat an existing generic accelerator
 * construct as a host/device semantic boundary when its endpoint metadata
 * establishes the corresponding roles.
 *
 * No duplicate accelerator grammar is created.
 */
hostDeviceAcceleratorConstruct
    : acceleratorTransferDeclaration
    | acceleratorOperationDeclaration
    | acceleratorExecutionDeclaration
    ;


/*
 * ============================================================================
 * 24. HYBRID REGION ITEM
 * ============================================================================
 *
 * Stable composition point for hybrid.g4.
 */
hostDeviceRegionConstruct
    : hostDeviceConstruct
    | hostDeviceAcceleratorConstruct
    | statement
    ;


/*
 * ============================================================================
 * 25. DOMAIN-NEUTRAL BOUNDARY MARKER
 * ============================================================================
 *
 * Tooling may use this rule to identify a semantic host/device boundary.
 */
hostDeviceBoundary
    : hostDeviceRolePair
    ;


/*
 * ============================================================================
 * 26. SOURCE-ORDERED BOUNDARY SEQUENCE
 * ============================================================================
 *
 * Source ordering is preserved by the AST.
 *
 * No scheduling semantics are implied.
 */
hostDeviceBoundarySequence
    : hostDeviceBoundarySequenceItem*
    ;


hostDeviceBoundarySequenceItem
    : hostDeviceBoundary
    | hostDeviceConstruct
    ;


/*
 * ============================================================================
 * 27. OPTIONAL ROLE METADATA
 * ============================================================================
 *
 * Additional metadata remains generic.
 *
 * Example:
 *
 *     @host @device @stream(...)
 *
 * Semantic interpretation belongs elsewhere.
 */
hostDeviceMetadata
    : AT
      IDENTIFIER
      (
          LPAREN
          expressionList?
          RPAREN
      )?
    ;


/*
 * ============================================================================
 * 28. ANNOTATED ENDPOINT
 * ============================================================================
 *
 * Allows extensible metadata without hard-coding transport technologies.
 */
hostDeviceAnnotatedEndpoint
    : hostDeviceMetadata*
      hostDeviceEndpoint
    ;


/*
 * ============================================================================
 * 29. EXTENSIBLE TRANSFER
 * ============================================================================
 *
 * This is the preferred future-proof form when additional semantic metadata
 * is needed.
 */
hostDeviceExtensibleTransfer
    : 'transfer'
      hostDeviceAnnotatedEndpoint
      'to'
      hostDeviceAnnotatedEndpoint
      hostDeviceTransferClause*
      SEMICOLON
    ;


/*
 * ============================================================================
 * 30. COMPLETION / READINESS BOUNDARY
 * ============================================================================
 *
 * A completion observation is represented as a semantic expression.
 *
 * The grammar does not define a particular event/future/promise runtime type.
 */
hostDeviceCompletionExpression
    : hostDeviceRoleAnnotation
      expression
    ;


/*
 * ============================================================================
 * 31. SEMANTIC ROLE ASSERTION
 * ============================================================================
 *
 * This construct does not perform hardware discovery.
 *
 * It states that the source is written against a particular execution role.
 */
hostDeviceRoleRequirement
    : 'requires'
      hostDeviceRoleAnnotation
      SEMICOLON
    ;


/*
 * ============================================================================
 * 32. TARGET-INDEPENDENT CAPABILITY REQUIREMENT
 * ============================================================================
 */
hostDeviceCapabilityRequirement
    : 'requires'
      'capability'
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 33. HOST/DEVICE REGION WITH REQUIREMENTS
 * ============================================================================
 */
hostDeviceContractRegion
    : hostDeviceRoleAnnotation+
      LBRACE
      hostDeviceContractItem*
      RBRACE
    ;


hostDeviceContractItem
    : hostDeviceRequirement
    | hostDeviceCapabilityRequirement
    | hostDeviceConstruct
    | statement
    ;


/*
 * ============================================================================
 * 34. PUBLIC COMPLETION RULE
 * ============================================================================
 *
 * All externally composed host/device syntax should enter through:
 *
 *     hostDeviceConstruct
 *
 * The remaining rules are named integration/AST-facing productions.
 *
 * ============================================================================
 */