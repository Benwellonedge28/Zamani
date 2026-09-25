/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/hardware.g4
 *
 * Grammar:
 *     Hardware
 *
 * Status:
 *     CANONICAL HARDWARE-DOMAIN COMPOSITION ROOT
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     Action-free ANTLR4 parser grammar.
 *     No embedded Rust.
 *     No semantic predicates.
 *     No filesystem access.
 *     No network access.
 *     No hardware discovery.
 *     No runtime execution.
 *     No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE HARDWARE-DOMAIN COMPOSITION ROOT.
 *
 * It does not attempt to implement every hardware concept itself.
 *
 * Instead it composes the independently owned hardware grammars:
 *
 *     resources.g4
 *     capabilities.g4
 *     constraints.g4
 *     targets.g4
 *     devices.g4
 *     topology.g4
 *     placement.g4
 *     accelerators.g4
 *     memory.g4
 *     interconnect.g4
 *     qpu.g4
 *     cpu.g4
 *     fpga.g4
 *     asic.g4
 *
 * The architectural direction is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     Hardware
 *       |
 *       +-------------------------------+
 *       |                               |
 *       v                               v
 *     hardware declarations        canonical expression
 *       |                               |
 *       +---------------+---------------+
 *                       |
 *                       v
 *                 domain-neutral AST
 *                       |
 *                       v
 *                semantic analysis
 *                       |
 *       +---------------+----------------+
 *       |               |                |
 *       v               v                v
 *    resources     capabilities      constraints
 *       |               |                |
 *       +---------------+----------------+
 *                       |
 *                       v
 *                target-independent
 *                hardware intent
 *                       |
 *                       v
 *                canonical semantic
 *                     model / IR
 *                       |
 *          +------------+-------------+
 *          |            |             |
 *          v            v             v
 *      classical     quantum::ir   HDL/hardware
 *          |            |             |
 *          +------------+-------------+
 *                       |
 *                       v
 *                 optimization
 *                       |
 *          +------------+-------------+
 *          |            |             |
 *          v            v             v
 *       routing     scheduling    resilience
 *                                    |
 *                                    v
 *                                   ZQN
 *                                    |
 *                                    v
 *                                   HAL
 *                                    |
 *                                    v
 *                            target realization
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - hardware-domain composition;
 *     - hardware declaration dispatch;
 *     - hardware statement dispatch;
 *     - hardware expression boundary;
 *     - hardware-domain integration;
 *     - cross-domain hardware composition;
 *     - the public Hardware grammar namespace;
 *     - integration of specialized hardware grammars.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - keywords;
 *     - identifiers;
 *     - literals;
 *     - expression precedence;
 *     - type syntax;
 *     - resource semantics;
 *     - capability semantics;
 *     - target semantics;
 *     - topology semantics;
 *     - placement algorithms;
 *     - device discovery;
 *     - physical allocation;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - HDL behavioral semantics;
 *     - compiler backend implementation;
 *     - runtime implementation.
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * All parser rules consume the canonical production lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * whose vocabulary is assembled by:
 *
 *     grammar/lexer/tokens.g4
 *
 * This file MUST therefore use canonical token names such as:
 *
 *     IDENTIFIER
 *     INTEGER
 *     FLOAT
 *     STRING
 *     CHAR
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     DOT
 *     SEMICOLON
 *     COLON
 *     ASSIGN
 *     EQ_EQ
 *     NOT_EQ
 *     LE
 *     GE
 *     AND_AND
 *     OR_OR
 *
 * It MUST NOT invent aliases such as:
 *
 *     K_HARDWARE
 *     K_RESOURCE
 *     K_CAPABILITY
 *     K_TARGET
 *     K_CPU
 *     K_GPU
 *     K_FPGA
 *     K_QPU
 *
 * unless those tokens have first been deliberately added to the canonical
 * lexer vocabulary.
 *
 * ============================================================================
 * IMPORTANT TOKEN POLICY
 * ============================================================================
 *
 * The current canonical lexer already provides source-level tokens such as:
 *
 *     RESOURCE
 *     CAPABILITY
 *     TARGET
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     REQUIRES
 *     QUANTUM
 *     GPU
 *
 * but does not currently provide a universal reserved HARDWARE/CPU/FPGA/ASIC
 * keyword family.
 *
 * This composition grammar therefore does NOT invent such tokens.
 *
 * Hardware-specific declaration keywords are owned by their respective leaf
 * grammars and their lexical contracts.
 *
 * This prevents hardware.g4 from becoming a second lexical authority.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * ANTLR parser grammars may import parser grammars and inherit their rules.
 *
 * The Hardware grammar deliberately imports specialized hardware parser
 * grammars rather than duplicating their rules here.
 *
 * The canonical composition is:
 *
 *     Hardware
 *       |
 *       +-- ZamaniHardwareResourcesParser
 *       +-- ZamaniHardwareCapabilitiesParser
 *       +-- ZamaniHardwareConstraintsParser
 *       +-- ZamaniHardwareTargetsParser
 *       +-- ZamaniHardwareDevicesParser
 *       +-- ZamaniHardwareTopologyParser
 *       +-- ZamaniHardwarePlacementParser
 *       +-- ZamaniHardwareAcceleratorParser
 *       +-- ZamaniHardwareMemoryParser
 *       +-- ZamaniHardwareInterconnectParser
 *       +-- ZamaniHardwareQpuParser
 *       +-- ZamaniHardwareCpuParser
 *       +-- HardwareFpga
 *       +-- HardwareAsic
 *
 * There is deliberately no second copy of those rules in this file.
 *
 * ============================================================================
 * SPECIALIZED FILE OWNERSHIP
 * ============================================================================
 *
 * resources.g4
 *     Owns hardware resource intent.
 *
 * capabilities.g4
 *     Owns hardware capability syntax.
 *
 * constraints.g4
 *     Owns hardware constraint syntax.
 *
 * targets.g4
 *     Owns abstract target syntax.
 *
 * devices.g4
 *     Owns logical device syntax.
 *
 * topology.g4
 *     Owns abstract topology syntax.
 *
 * placement.g4
 *     Owns placement intent.
 *
 * accelerators.g4
 *     Owns accelerator syntax.
 *
 * memory.g4
 *     Owns hardware memory contracts.
 *
 * interconnect.g4
 *     Owns hardware interconnect syntax.
 *
 * qpu.g4
 *     Owns QPU hardware contracts.
 *
 * cpu.g4
 *     Owns CPU hardware contracts.
 *
 * fpga.g4
 *     Owns FPGA hardware contracts.
 *
 * asic.g4
 *     Owns ASIC hardware contracts.
 *
 * hardware.g4
 *     ONLY composes and dispatches these domains.
 *
 * ============================================================================
 * NO DUPLICATE HARDWARE IR
 * ============================================================================
 *
 * This grammar produces syntax only.
 *
 * It does not define:
 *
 *     HardwareIR
 *     DeviceIR
 *     HardwareOperationIR
 *     HardwareResourceIR
 *     HardwareCapabilityIR
 *     QuantumHardwareIR
 *
 * or any other competing intermediate representation.
 *
 * Hardware semantics flow into the repository's canonical semantic/IR
 * architecture.
 *
 * Quantum computation continues through:
 *
 *     quantum::ir
 *
 * This file never replaces or duplicates that boundary.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Hardware syntax expresses portable computation intent.
 *
 * It does NOT require the source program to name:
 *
 *     a physical CPU;
 *     a physical core;
 *     a physical thread;
 *     a physical GPU;
 *     a physical FPGA;
 *     a physical ASIC;
 *     a physical QPU;
 *     a physical qubit;
 *     a physical node;
 *     a physical memory bank;
 *     a physical accelerator;
 *     a PCI address;
 *     an IP address;
 *     a serial number;
 *     a vendor device;
 *     a machine hostname;
 *     a calibration record.
 *
 * Concrete realization belongs downstream.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains NO universal hardware capacities.
 *
 * It MUST NOT define:
 *
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_ACCELERATORS
 *     MAX_PORTS
 *     MAX_CONNECTIONS
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_VECTOR_WIDTH
 *     MAX_TOPOLOGY_SIZE
 *
 * Repetition is represented with ANTLR repetition operators.
 *
 * Quantities are expressions.
 *
 * Therefore the language can describe:
 *
 *     tiny systems
 *     embedded systems
 *     single CPUs
 *     multicore CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     clusters
 *     distributed systems
 *     HPC systems
 *     heterogeneous systems
 *     future computational substrates
 *
 * without changing the hardware composition grammar.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY / CONSTRAINT SEPARATION
 * ============================================================================
 *
 * RESOURCE
 *     Something that may be available, consumed, shared, reserved, or
 *     otherwise relevant to execution.
 *
 * CAPABILITY
 *     What an implementation can do.
 *
 * REQUIREMENT
 *     What must exist for the program to be semantically valid.
 *
 * CONSTRAINT
 *     A mandatory restriction on legal realization.
 *
 * PREFERENCE
 *     Non-mandatory optimization guidance.
 *
 * HINT
 *     Advisory information.
 *
 * TARGET
 *     An abstract realization class or execution context.
 *
 * PLACEMENT
 *     Target-independent realization intent.
 *
 * TOPOLOGY
 *     An abstract relationship model among logical resources.
 *
 * DEVICE
 *     A logical device class or semantic device object, not a physical
 *     enumeration.
 *
 * These concepts MUST remain distinct.
 *
 * ============================================================================
 * TARGET-INDEPENDENCE
 * ============================================================================
 *
 * Hardware declarations must describe semantic requirements rather than
 * implementation decisions.
 *
 * Conceptually:
 *
 *     requires resource.qubits >= required_qubits;
 *
 *     requires capability.quantum.measurement;
 *
 *     requires capability.tensor.compute;
 *
 *     requires memory >= required_memory;
 *
 *     prefer target.gpu;
 *
 *     prefer accelerator.tensor;
 *
 *     constraint latency <= budget;
 *
 * are portable intent.
 *
 * By contrast:
 *
 *     use physical_gpu_0
 *     use physical_qubit_17
 *     map_to_pci_address(...)
 *     use_host_machine(...)
 *
 * are target-realization decisions and therefore do not belong in this
 * portable composition grammar.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file must remain free from:
 *
 *     fixed hardware counts;
 *     fixed physical IDs;
 *     fixed addresses;
 *     fixed topology sizes;
 *     fixed register widths;
 *     fixed memory capacities;
 *     fixed tensor dimensions;
 *     fixed accelerator counts;
 *     fixed qubit counts;
 *     fixed device counts;
 *     vendor-specific assumptions.
 *
 * Numeric literals remain valid program values.
 *
 * For example:
 *
 *     required_memory = 64GB
 *
 * may be valid program semantics if the canonical literal/type system supports
 * the size literal.
 *
 * What is prohibited is:
 *
 *     language maximum memory = 64GB
 *
 * or any equivalent universal compiler restriction encoded here.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Hardware syntax is intentionally reusable by:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     compile
 *     execution
 *
 * A hardware requirement may therefore originate from a quantum program,
 * classical computation, HDL co-design contract, accelerator computation,
 * distributed deployment, or future domain.
 *
 * The hardware grammar must not create domain-specific duplicate resource
 * systems.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * QPU declarations are delegated to:
 *
 *     qpu.g4
 *
 * Quantum program semantics remain outside this file.
 *
 * The pipeline is:
 *
 *     quantum source
 *         |
 *         v
 *     domain-neutral AST
 *         |
 *         v
 *     quantum semantic analysis
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     hardware capability/resource analysis
 *         |
 *         v
 *     routing
 *         |
 *         v
 *     scheduling
 *         |
 *         v
 *     QEC / resilience / ZQN
 *         |
 *         v
 *     HAL
 *         |
 *         v
 *     target realization
 *
 * hardware.g4 never creates a second quantum IR.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * HDL behavioral semantics remain owned by:
 *
 *     grammar/hdl/
 *
 * hardware.g4 may provide hardware intent consumed alongside HDL, but it must
 * not reproduce:
 *
 *     module behavior
 *     procedural blocks
 *     always blocks
 *     signal assignment semantics
 *     HDL process semantics
 *     HDL simulation semantics
 *
 * Hardware/software co-design is therefore composed rather than duplicated.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * hardware.g4 MUST NOT define a second expression language.
 *
 * The canonical public expression rule is:
 *
 *     expression
 *
 * from:
 *
 *     grammar/expressions/expressions.g4
 *
 * Therefore hardware-specific quantities, bounds, dimensions, predicates,
 * resource values, timing values, and properties use the canonical expression
 * model.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * hardware.g4 does not define a second type system.
 *
 * Hardware-specific semantic types are supplied by the canonical Zamani type
 * system and interpreted by semantic analysis.
 *
 * Examples include conceptual forms such as:
 *
 *     Qubit[n]
 *     Tensor<T, shape>
 *     Memory<T, size>
 *
 * where the dimensions are semantic expressions rather than grammar-level
 * limits.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every imported hardware construct must map to the domain-neutral frontend
 * AST.
 *
 * hardware.g4 does not prescribe vendor-specific AST nodes.
 *
 * The expected semantic information includes, as applicable:
 *
 *     declaration kind
 *     name
 *     qualified name
 *     parameters
 *     attributes
 *     requirements
 *     capabilities
 *     resources
 *     constraints
 *     preferences
 *     hints
 *     topology intent
 *     placement intent
 *     target intent
 *     source span
 *
 * Physical device identity is NOT part of the source-level portable contract.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis is responsible for:
 *
 *     name resolution;
 *     type validation;
 *     capability validation;
 *     resource validation;
 *     constraint validation;
 *     target compatibility;
 *     portability validation;
 *     conflict detection;
 *     requirement satisfaction;
 *     preference handling;
 *     topology validation;
 *     placement validation;
 *     device-class compatibility.
 *
 * A syntax error must not be used to represent an unavailable resource.
 *
 * Example:
 *
 *     requires qubits >= n;
 *
 * is syntactically valid even when the current target has insufficient
 * resources.
 *
 * Insufficient resources are a semantic/resource-resolution result.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * hardware.g4 has NO independent IR.
 *
 * The compiler may lower hardware intent into the repository's canonical
 * semantic/IR representations.
 *
 * Quantum constructs converge through:
 *
 *     quantum::ir
 *
 * Classical constructs converge through the canonical classical representation.
 *
 * HDL/hardware constructs converge through the repository's hardware/HDL
 * semantic representation.
 *
 * Hardware intent may remain metadata/contract information when no executable
 * operation is implied.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions;
 *     no predicates;
 *     no external state;
 *     no environment access;
 *     no hardware discovery;
 *     no randomness.
 *
 * The same token stream therefore receives the same syntactic interpretation.
 *
 * Any target-dependent result must be produced after parsing by the semantic,
 * resource, target, routing, scheduling, or runtime layers.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Stable hardware syntax must remain compatible with:
 *
 *     grammar/spec/hardware.md
 *     grammar/spec/resources.md
 *     grammar/spec/portability.md
 *     grammar/compatibility/
 *     grammar/grammar.md
 *
 * `Zamani-Grammar.md` may contain historical/proposed hardware concepts but
 * does not automatically add syntax to this grammar.
 *
 * ============================================================================
 * BUILD INTEGRATION
 * ============================================================================
 *
 * This grammar is imported by:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * through the grammar name:
 *
 *     Hardware
 *
 * The canonical parser therefore sees:
 *
 *     ZamaniParser
 *          |
 *          v
 *       Hardware
 *          |
 *          +--> specialized hardware parser rules
 *
 * The build system must make all imported grammars available to ANTLR's
 * grammar search path.
 *
 * ============================================================================
 * PRODUCTION COMPLETION CHECKLIST
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] Single hardware composition authority
 *     [x] No duplicate resource grammar
 *     [x] No duplicate capability grammar
 *     [x] No duplicate target grammar
 *     [x] No duplicate topology grammar
 *     [x] No duplicate placement grammar
 *     [x] No duplicate QPU grammar
 *     [x] No duplicate CPU grammar
 *     [x] No duplicate FPGA grammar
 *     [x] No duplicate ASIC grammar
 *     [x] No local expression grammar
 *     [x] No local type system
 *     [x] No physical device selection
 *     [x] No physical IDs
 *     [x] No universal capacity constants
 *     [x] No Rust actions
 *     [x] No unsafe requirements
 *     [x] Canonical lexer vocabulary
 *     [x] Canonical expression boundary
 *     [x] Canonical AST boundary
 *     [x] Canonical semantic boundary
 *     [x] quantum::ir preserved
 *     [x] POCO-REAF preserved
 *
 * Repository-wide completion additionally requires:
 *
 *     [ ] specialized imported grammars compile together
 *     [ ] duplicate grammar names removed
 *     [ ] GPU grammar converted to a parser grammar or composed by its
 *         canonical dispatcher
 *     [ ] all imported grammar token names match ZamaniLexer
 *     [ ] hardware tests cover positive/negative/boundary/scalability cases
 *
 * ============================================================================
 */

parser grammar Hardware;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions,
    ZamaniHardwareResourcesParser,
    ZamaniHardwareCapabilitiesParser,
    ZamaniHardwareConstraintsParser,
    ZamaniHardwareTargetsParser,
    ZamaniHardwareDevicesParser,
    ZamaniHardwareTopologyParser,
    ZamaniHardwarePlacementParser,
    ZamaniHardwareAcceleratorParser,
    ZamaniHardwareMemoryParser,
    ZamaniHardwareInterconnectParser,
    ZamaniHardwareQpuParser,
    ZamaniHardwareCpuParser,
    HardwareFpga,
    HardwareAsic
;


/* ============================================================================
 * 1. HARDWARE DECLARATION DISPATCH
 * ============================================================================
 *
 * There is deliberately no giant monolithic hardware rule here.
 *
 * Each specialized declaration is owned by its own file.
 *
 * The alternatives below merely expose those declarations through the single
 * Hardware domain boundary consumed by ZamaniParser.
 *
 * ========================================================================== */

hardwareDeclaration
    : hardwareResourceDeclaration
    | hardwareCapabilityDeclaration
    | hardwareConstraintDeclaration
    | hardwareTargetDeclaration
    | deviceDeclaration
    | topologyDeclaration
    | placementDeclaration
    | hardwareAcceleratorDeclaration
    | hardwareMemoryContract
    | hardwareInterconnectDeclaration
    | qpuDeclaration
    | cpuDeclaration
    | hardwareFpgaDeclaration
    | hardwareAsicDeclaration
    ;


/* ============================================================================
 * 2. HARDWARE STATEMENT BOUNDARY
 * ============================================================================
 *
 * Hardware declarations are the primary source-level hardware contract form.
 *
 * Hardware-specific executable behavior must remain owned by the appropriate
 * domain grammar:
 *
 *     HDL -> grammar/hdl/
 *     quantum -> grammar/quantum/
 *     classical -> grammar/classical/
 *     distributed -> grammar/distributed/
 *
 * Assertions are the one universal hardware-domain statement form retained
 * here because they express a property of hardware intent rather than an
 * implementation algorithm.
 *
 * ========================================================================== */

hardwareStatement
    : hardwareAssertionStatement
    ;

hardwareAssertionStatement
    : ASSERT
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 3. HARDWARE EXPRESSION BOUNDARY
 * ============================================================================
 *
 * Hardware expressions are NOT a second expression language.
 *
 * This alias deliberately exposes the canonical expression rule to the
 * Hardware domain.
 *
 * ========================================================================== */

hardwareExpression
    : expression
    ;


/* ============================================================================
 * 4. HARDWARE CONTRACT EXPRESSION
 * ============================================================================
 *
 * A hardware contract expression is useful when a consumer needs a single
 * expression boundary for a requirement, constraint, preference, property,
 * topology predicate, placement predicate, or resource quantity.
 *
 * No additional precedence is introduced.
 *
 * ========================================================================== */

hardwareContractExpression
    : expression
    ;


/* ============================================================================
 * 5. HARDWARE VALUE
 * ============================================================================
 *
 * A hardware value is intentionally the canonical expression.
 *
 * This rule exists only as a semantic naming boundary for AST/visitor
 * integration.
 *
 * It does not alter expression semantics.
 *
 * ========================================================================== */

hardwareValue
    : expression
    ;


/* ============================================================================
 * 6. HARDWARE NAME REFERENCE
 * ============================================================================
 *
 * Hardware names remain ordinary canonical names.
 *
 * No physical-device ID syntax is introduced here.
 *
 * The canonical name grammar owns qualified-name semantics.
 *
 * ========================================================================== */

hardwareNameReference
    : IDENTIFIER
    ;


/* ============================================================================
 * 7. HARDWARE QUALIFIED REFERENCE
 * ============================================================================
 *
 * Qualified hardware references are intentionally composed from the canonical
 * expression/member-access model rather than inventing a second qualified
 * name syntax.
 *
 * The semantic layer determines whether the resulting name refers to:
 *
 *     resource
 *     capability
 *     target
 *     device class
 *     topology object
 *     placement object
 *     accelerator
 *     QPU class
 *     CPU class
 *     FPGA class
 *     ASIC class
 *     future hardware domain
 *
 * ========================================================================== */

hardwareQualifiedReference
    : expression
    ;


/* ============================================================================
 * 8. RESOURCE/CAPABILITY REQUIREMENT BRIDGE
 * ============================================================================
 *
 * These rules are semantic naming boundaries only.
 *
 * They do not redefine resource/capability syntax.
 *
 * ========================================================================== */

hardwareRequirementExpression
    : expression
    ;

hardwareConstraintExpression
    : expression
    ;

hardwarePreferenceExpression
    : expression
    ;

hardwareHintExpression
    : expression
    ;


/* ============================================================================
 * 9. HARDWARE CONTRACT LISTS
 * ============================================================================
 *
 * These generic list boundaries are intentionally unbounded.
 *
 * ========================================================================== */

hardwareDeclarationList
    : hardwareDeclaration*
    ;

hardwareStatementList
    : hardwareStatement*
    ;

hardwareExpressionList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/* ============================================================================
 * 10. HARDWARE PROPERTY VALUE
 * ============================================================================
 *
 * Property values remain canonical expressions.
 *
 * This supports future hardware technologies without adding a keyword for
 * every property.
 *
 * ========================================================================== */

hardwarePropertyValue
    : expression
    ;


/* ============================================================================
 * 11. HARDWARE DIMENSION
 * ============================================================================
 *
 * Dimensions are expressions.
 *
 * No fixed width, rank, count, or capacity is encoded.
 *
 * ========================================================================== */

hardwareDimension
    : expression
    ;

hardwareDimensionList
    : LBRACKET
      expression
      RBRACKET
      (
          LBRACKET
          expression
          RBRACKET
      )*
    ;


/* ============================================================================
 * 12. HARDWARE ATTRIBUTE VALUE
 * ============================================================================
 *
 * Attributes are interpreted by their owning semantic subsystem.
 *
 * The hardware composition root only provides an expression boundary.
 *
 * ========================================================================== */

hardwareAttributeValue
    : expression
    ;


/* ============================================================================
 * 13. HARDWARE CONTRACT
 * ============================================================================
 *
 * This rule provides a generic aggregation boundary for consumers that need
 * to reason about a sequence of hardware declarations without creating a
 * second hardware program root.
 *
 * ========================================================================== */

hardwareContract
    : hardwareDeclaration+
    ;


/* ============================================================================
 * 14. HARDWARE DOMAIN
 * ============================================================================
 *
 * This is the domain dispatcher consumed by ZamaniParser.g4:
 *
 *     hardwareElement
 *         : hardwareDeclaration
 *         | hardwareStatement
 *         | hardwareExpression
 *         ;
 *
 * Keeping this dispatcher here means the universal root parser does not need
 * to know individual hardware grammar files.
 *
 * ========================================================================== */

hardwareDomain
    : hardwareDeclaration
    | hardwareStatement
    | hardwareExpression
    ;


/* ============================================================================
 * 15. PORTABILITY INVARIANT
 * ============================================================================
 *
 * Hardware syntax may describe:
 *
 *     requirement
 *     capability
 *     resource
 *     target
 *     constraint
 *     preference
 *     topology
 *     placement
 *     memory
 *     interconnect
 *     accelerator
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     logical device
 *
 * but physical realization remains outside the grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 16. FINAL ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * The complete hardware path is:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Hardware
 *          |
 *          +--> resources
 *          +--> capabilities
 *          +--> constraints
 *          +--> targets
 *          +--> devices
 *          +--> topology
 *          +--> placement
 *          +--> accelerators
 *          +--> memory
 *          +--> interconnect
 *          +--> CPU
 *          +--> GPU integration
 *          +--> FPGA
 *          +--> ASIC
 *          +--> QPU
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> portability analysis
 *          +--> target analysis
 *          +--> topology analysis
 *          +--> placement analysis
 *          |
 *          v
 *     canonical semantic model / IR
 *          |
 *          +--> classical
 *          +--> quantum::ir
 *          +--> HDL/hardware
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     ZQN / QEC where applicable
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * No stage above the target-realization boundary may depend on a particular
 * physical machine merely because that machine is available during
 * compilation.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */