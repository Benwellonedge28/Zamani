/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/hardware-modules.g4
 *
 * Grammar:
 *     HardwareModules
 *
 * Status:
 *     CANONICAL HDL MODULE DELEGATE
 *
 * Purpose:
 *     Own the source-level syntax for reusable, parameterized, logical HDL
 *     modules and logical module instantiation.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     Rust 2021
 *
 * Safety:
 *     Action-free ANTLR parser grammar.
 *     No embedded Rust.
 *     No semantic predicates.
 *     No unsafe code.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Normative architecture:
 *
 *     grammar/DESIGN.md
 *          |
 *          v
 *     grammar/specification/
 *          |
 *          v
 *     grammar/spec/
 *          |
 *          v
 *     grammar/hdl/hdl.g4
 *          |
 *          +--> HardwareModules
 *          |
 *          +--> Generate
 *          |
 *          +--> ports
 *          +--> signals
 *          +--> wires
 *          +--> registers
 *          +--> memories
 *          +--> clocks
 *          +--> timing
 *          +--> processes
 *          +--> combinational
 *          +--> sequential
 *          +--> pipelines
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical hardware semantic model / IR
 *          |
 *          +--> optimization
 *          +--> verification
 *          +--> scheduling
 *          +--> routing
 *          +--> synthesis
 *          +--> target lowering
 *          |
 *          v
 *     target realization
 *
 * This file is a parser delegate.
 *
 * It MUST NOT become another HDL parser root.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - HDL module declaration syntax;
 *     - logical module names;
 *     - module-level generic specialization attachment;
 *     - module body composition;
 *     - logical module instance syntax;
 *     - logical module references;
 *     - logical instance specialization;
 *     - logical instance connections;
 *     - logical instance connection lists.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names generally;
 *     - expressions;
 *     - types;
 *     - generic declaration syntax;
 *     - parameter declaration syntax;
 *     - ports;
 *     - signals;
 *     - wires/nets;
 *     - registers;
 *     - memories;
 *     - clocks;
 *     - timing;
 *     - processes;
 *     - combinational behavior;
 *     - sequential behavior;
 *     - state machines;
 *     - pipelines;
 *     - generate syntax;
 *     - resource requirements;
 *     - capabilities;
 *     - target selection;
 *     - physical placement;
 *     - routing;
 *     - synthesis;
 *     - scheduling;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file replaces the previous module implementation that also attempted
 * to own:
 *
 *     - generate syntax;
 *     - resource contracts;
 *     - verification syntax;
 *     - continuous assignments;
 *     - generic declaration syntax;
 *     - parameter declaration syntax.
 *
 * Those are separate ownership domains.
 *
 * In particular:
 *
 *     generate.g4
 *         owns general structural generation syntax.
 *
 *     hardware-generics.g4
 *         owns HDL generic declaration/specialization syntax.
 *
 *     signals.g4
 *         owns signal syntax.
 *
 *     wires.g4
 *         owns wire/net syntax.
 *
 *     registers.g4
 *         owns register syntax.
 *
 *     memories.g4
 *         owns memory syntax.
 *
 *     resources/
 *         owns resource/capability intent.
 *
 *     verification grammar
 *         owns verification constructs.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A module is a logical source-level abstraction.
 *
 * It does NOT identify:
 *
 *     a CPU;
 *     a GPU;
 *     an FPGA;
 *     an ASIC;
 *     a QPU;
 *     a physical accelerator;
 *     a physical board;
 *     a physical pin;
 *     a physical register;
 *     a physical memory block;
 *     a physical routing path;
 *     a physical placement;
 *     a vendor primitive.
 *
 * Therefore:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * remains possible at the language level.
 *
 * Target realization happens after semantic analysis and canonical IR
 * construction.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains NO universal machine/resource limits.
 *
 * It contains no:
 *
 *     MAX_MODULES
 *     MAX_INSTANCES
 *     MAX_PORTS
 *     MAX_SIGNALS
 *     MAX_REGISTERS
 *     MAX_MEMORIES
 *     MAX_PARAMETERS
 *     MAX_GENERICS
 *     MAX_WIDTH
 *     MAX_DEPTH
 *     MAX_LANES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Source repetition is represented by ANTLR repetition operators.
 *
 * Practical limits may exist in:
 *
 *     compiler resources;
 *     elaboration resources;
 *     target resources;
 *     synthesis;
 *     routing;
 *     scheduling;
 *     deployment.
 *
 * Those limits MUST NOT become language-level limits.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * The following are valid logical names:
 *
 *     cpu
 *     gpu
 *     fpga
 *     qpu
 *     accelerator
 *     node
 *
 * Their spelling does not select physical hardware.
 *
 * For example:
 *
 *     instance compute: ComputeUnit;
 *
 * means a logical instance.
 *
 * It does NOT mean:
 *
 *     physical device 0.
 *
 * ============================================================================
 * GENERATE INTEGRATION
 * ============================================================================
 *
 * Structural generation is NOT implemented in this file.
 *
 * The canonical generation grammar is:
 *
 *     grammar/hdl/generate.g4
 *
 * It owns:
 *
 *     hdlGenerateDeclaration
 *     hdlGenerateFor
 *     hdlGenerateIf
 *     hdlGenerateBlock
 *     hdlGenerateBody
 *
 * This module grammar merely permits the generation rule to participate in
 * module composition through the HDL member dispatcher.
 *
 * There must never be a second:
 *
 *     hdlModuleGenerate
 *     hdlModuleGenerateFor
 *     hdlModuleGenerateIf
 *     hdlModuleGenerateBlock
 *
 * implementation here.
 *
 * ============================================================================
 * GENERICS
 * ============================================================================
 *
 * Generic declarations and generic specialization are external contracts.
 *
 * The preferred ownership is:
 *
 *     grammar/hdl/hardware-generics.g4
 *
 * This file only consumes:
 *
 *     hardwareGenericParameters
 *     hardwareGenericSpecialization
 *
 * when those rules are provided by the HDL composition grammar.
 *
 * ============================================================================
 * EXPRESSIONS
 * ============================================================================
 *
 * This file MUST NOT define another expression grammar.
 *
 * Expressions are supplied by the canonical HDL/universal expression
 * composition.
 *
 * Module instance arguments therefore consume:
 *
 *     expression
 *
 * rather than defining:
 *
 *     moduleExpression
 *     instanceExpression
 *     hardwareExpression
 *
 * ============================================================================
 * NAMES
 * ============================================================================
 *
 * Simple identifiers and logical qualified module names are source-level
 * names.
 *
 * No physical resource identity is implied.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser produces syntax only.
 *
 * The frontend AST should be capable of representing:
 *
 *     HdlModuleDeclaration
 *     HdlModuleName
 *     HdlModuleBody
 *     HdlModuleInstance
 *     HdlModuleReference
 *     HdlModuleSpecialization
 *     HdlModuleConnection
 *
 * The AST remains domain-neutral.
 *
 * It MUST NOT create:
 *
 *     FpgaModuleNode
 *     GpuModuleNode
 *     CpuModuleNode
 *     QpuModuleNode
 *     VendorModuleNode
 *     PhysicalModuleNode
 *     PlacementModuleNode
 *     NetlistModuleNode
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - module name resolution;
 *     - namespace resolution;
 *     - duplicate module detection;
 *     - generic specialization;
 *     - parameter/type checking;
 *     - instance interface checking;
 *     - named connection validation;
 *     - connection type compatibility;
 *     - direction compatibility;
 *     - instance scope;
 *     - recursive dependency detection;
 *     - module elaboration;
 *     - capability analysis;
 *     - resource analysis;
 *     - target realizability.
 *
 * None of these checks belong in the parser.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Module syntax does not directly select hardware.
 *
 * Source-level requirements such as:
 *
 *     requires qubits >= n
 *
 *     requires capability("gpu.compute")
 *
 *     requires memory >= required_memory
 *
 * belong to the resource/capability subsystem.
 *
 * A module reference such as:
 *
 *     ComputeUnit
 *
 * is not a resource allocation.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A module may surround quantum-related hardware/control structures.
 *
 * This file does not define quantum operation semantics.
 *
 * Quantum source continues through:
 *
 *     generic AST
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *
 * This file MUST NOT create:
 *
 *     HardwareQuantumIR
 *     QuantumModuleIR
 *     HardwareGateIR
 *
 * or another quantum intermediate representation.
 *
 * QEC, ZQN, routing, scheduling, calibration and HAL remain downstream.
 *
 * ============================================================================
 * CLASSICAL / HYBRID INTEGRATION
 * ============================================================================
 *
 * A hardware module may contain or surround classical and hybrid computation.
 *
 * Classical semantics remain owned by the classical subsystem.
 *
 * Quantum/classical interaction remains owned by the hybrid and quantum
 * semantic subsystems.
 *
 * This grammar provides only the structural module boundary.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text;
 *     canonical lexer vocabulary;
 *     grammar version;
 *     selected language/dialect configuration.
 *
 * It MUST NOT inspect:
 *
 *     hardware;
 *     CPU count;
 *     GPU availability;
 *     QPU availability;
 *     filesystem state;
 *     network state;
 *     environment variables;
 *     wall-clock time;
 *     randomness;
 *     deployment topology.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - executes no code;
 *     - accesses no files;
 *     - accesses no network;
 *     - accesses no hardware;
 *     - accesses no credentials;
 *     - performs no target discovery;
 *     - contains no embedded Rust.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The stable module entry contract is:
 *
 *     hdlModuleDeclaration
 *
 * Existing inline module-generation rules are intentionally removed from this
 * file because generation now has a single owner in generate.g4.
 *
 * Existing module instance semantics are preserved as logical source-level
 * composition.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] one canonical module declaration rule exists;
 *     [x] module names are target-independent;
 *     [x] module bodies are unbounded;
 *     [x] module instances are logical;
 *     [x] instance specialization is supported;
 *     [x] named connections are supported;
 *     [x] positional connections are supported;
 *     [x] generic syntax is delegated;
 *     [x] expression syntax is delegated;
 *     [x] type syntax is delegated;
 *     [x] generate syntax is delegated;
 *     [x] signals are delegated;
 *     [x] ports are delegated;
 *     [x] memories are delegated;
 *     [x] registers are delegated;
 *     [x] no physical target is encoded;
 *     [x] no hardware maximum is encoded;
 *     [x] no vendor is encoded;
 *     [x] no competing quantum IR exists;
 *     [x] no embedded Rust exists;
 *     [x] no unsafe exists;
 *     [x] AST integration is predetermined;
 *     [x] semantic integration is predetermined;
 *     [x] IR integration is predetermined;
 *     [x] compiler integration is predetermined;
 *     [x] runtime integration is predetermined.
 *
 * ============================================================================
 */

parser grammar HardwareModules;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. MODULE DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     module Counter {
 *         ...
 *     }
 *
 *     module compute::Counter {
 *         ...
 *     }
 *
 * Generic specialization/declaration syntax is supplied by the canonical
 * generic grammar.
 */
hdlModuleDeclaration
    : hdlAttribute*
      K_MODULE
      hdlModuleName
      hardwareGenericParameters?
      hdlModuleBody
    ;


/*
 * ============================================================================
 * 2. MODULE NAME
 * ============================================================================
 *
 * A module name is a logical qualified source name.
 *
 * It is deliberately not a target/device identifier.
 */
hdlModuleName
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


/*
 * ============================================================================
 * 3. MODULE BODY
 * ============================================================================
 *
 * The module body is an unbounded ordered collection of HDL module members.
 *
 * Concrete member syntax remains owned by the corresponding HDL subsystem.
 *
 * The rule hdlModuleMemberCore is supplied by the HDL composition boundary.
 */
hdlModuleBody
    : LBRACE
      hdlModuleMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. MODULE MEMBER
 * ============================================================================
 *
 * Attribute ownership remains in the canonical HDL/core grammar.
 *
 * The actual member dispatcher is deliberately not duplicated here.
 *
 * `hdlModuleMemberCore` is the integration point supplied by the HDL
 * composition grammar.
 */
hdlModuleMember
    : hdlAttribute*
      hdlModuleMemberCore
    ;


/*
 * ============================================================================
 * 5. MODULE INSTANCE
 * ============================================================================
 *
 * Logical source-level module composition.
 *
 * Examples:
 *
 *     instance counter: Counter;
 *
 *     instance counter: Counter<WIDTH = width>;
 *
 *     instance arithmetic: compute::Arithmetic<WIDTH = width> {
 *         a = lhs,
 *         b = rhs,
 *         result = output
 *     };
 *
 * No physical device is selected.
 */
hdlInstanceDeclaration
    : K_INSTANCE
      identifier
      COLON
      hdlModuleReference
      hdlInstanceConnectionBlock?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. MODULE REFERENCE
 * ============================================================================
 *
 * A module reference may identify:
 *
 *     - a local module;
 *     - an imported module;
 *     - a package module;
 *     - a generated module;
 *     - a dialect-provided module;
 *     - an external logical module.
 *
 * Resolution is semantic.
 */
hdlModuleReference
    : hdlModuleName
      hardwareGenericSpecialization?
    ;


/*
 * ============================================================================
 * 7. INSTANCE CONNECTION BLOCK
 * ============================================================================
 *
 * Named and positional connections may coexist syntactically only where the
 * semantic model permits it.
 *
 * The semantic layer is responsible for rejecting invalid mixtures.
 *
 * No fixed connection count exists.
 */
hdlInstanceConnectionBlock
    : LBRACE
      hdlInstanceConnectionList?
      RBRACE
    ;


/*
 * ============================================================================
 * 8. CONNECTION LIST
 * ============================================================================
 */
hdlInstanceConnectionList
    : hdlInstanceConnection
      (
          COMMA
          hdlInstanceConnection
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 9. CONNECTION
 * ============================================================================
 *
 * Named:
 *
 *     input_a = a
 *
 * Positional:
 *
 *     a
 *
 * The semantic layer resolves the latter against the module interface.
 */
hdlInstanceConnection
    : identifier
      ASSIGN
      expression
    | expression
    ;


/*
 * ============================================================================
 * 10. GENERIC SPECIALIZATION ADAPTER
 * ============================================================================
 *
 * This grammar does not define generic specialization.
 *
 * `hardwareGenericSpecialization` is supplied by the generic subsystem.
 *
 * Conceptual forms include:
 *
 *     Counter<WIDTH = 32>
 *
 *     Counter<T = Data>
 *
 *     Counter<WIDTH = width, LANES = lanes>
 *
 * The actual generic grammar remains the sole authority.
 */


/*
 * ============================================================================
 * 11. MEMBER-DISPATCH INTEGRATION
 * ============================================================================
 *
 * The HDL composition root must expose `hdlModuleMemberCore`.
 *
 * It is responsible for composing:
 *
 *     parameters
 *     local parameters
 *     type declarations
 *     interfaces
 *     ports
 *     signals
 *     nets
 *     registers
 *     memories
 *     clocks
 *     resets
 *     assignments
 *     processes
 *     always constructs
 *     combinational constructs
 *     sequential constructs
 *     state machines
 *     pipelines
 *     generate constructs
 *     timing
 *     assertions
 *     nested/block declarations
 *     expression statements
 *
 * This file deliberately does not reproduce that list.
 *
 * This is important because otherwise:
 *
 *     hdl.g4
 *
 * and:
 *
 *     hardware-modules.g4
 *
 * would become competing module-member authorities.
 */


/*
 * ============================================================================
 * 12. GENERATE INTEGRATION
 * ============================================================================
 *
 * `generate.g4` is the sole general generation owner.
 *
 * The HDL composition layer must include:
 *
 *     hdlGenerateDeclaration
 *
 * in its canonical `hdlModuleMemberCore`.
 *
 * No module-specific generate grammar is defined here.
 *
 * Therefore the following old rules are intentionally absent:
 *
 *     hdlModuleGenerate
 *     hdlModuleGenerateBody
 *     hdlModuleGenerateItem
 *     hdlModuleGenerateFor
 *     hdlModuleGenerateBinding
 *     hdlModuleGenerateIf
 *     hdlModuleGenerateBlock
 *     hdlModuleGenerateMember
 *
 * This prevents structural generation from splitting into two grammars.
 */


/*
 * ============================================================================
 * 13. PORT INTEGRATION
 * ============================================================================
 *
 * Ports remain owned by:
 *
 *     grammar/hdl/ports.g4
 *
 * The module grammar merely permits the canonical port member rule through
 * `hdlModuleMemberCore`.
 *
 * Physical pins and board locations remain downstream.
 */


/*
 * ============================================================================
 * 14. SIGNAL / WIRE / REGISTER INTEGRATION
 * ============================================================================
 *
 * Ownership remains:
 *
 *     signals.g4
 *     wires.g4
 *     registers.g4
 *
 * No declaration syntax is duplicated here.
 */


/*
 * ============================================================================
 * 15. MEMORY INTEGRATION
 * ============================================================================
 *
 * Memory declarations remain owned by:
 *
 *     memories.g4
 *
 * Module composition does not imply:
 *
 *     RAM capacity;
 *     VRAM capacity;
 *     physical block RAM;
 *     SRAM technology;
 *     DRAM technology;
 *     device-local memory.
 */


/*
 * ============================================================================
 * 16. CLOCK / TIMING INTEGRATION
 * ============================================================================
 *
 * Clock and timing syntax remain owned by their respective HDL grammars.
 *
 * Module composition does not select:
 *
 *     physical oscillator;
 *     PLL;
 *     clock tree;
 *     physical clock pin.
 */


/*
 * ============================================================================
 * 17. PIPELINE INTEGRATION
 * ============================================================================
 *
 * Pipeline syntax remains owned by:
 *
 *     pipelines.g4
 *
 * No maximum stage count is encoded here.
 */


/*
 * ============================================================================
 * 18. RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Resource and capability expressions remain outside the module grammar.
 *
 * A module may eventually participate in:
 *
 *     requires ...
 *     provides ...
 *     prefer ...
 *     capability(...)
 *
 * but the lexical/syntactic ownership of those constructs belongs to the
 * resource/capability subsystem.
 *
 * This prevents unsupported K_REQUIRES/K_PROVIDES/K_CONSTRAINT/K_PREFER tokens
 * from being invented inside this grammar.
 */


/*
 * ============================================================================
 * 19. VERIFICATION INTEGRATION
 * ============================================================================
 *
 * Verification constructs remain owned by the HDL verification/assertion
 * grammar and enter the module through `hdlModuleMemberCore`.
 *
 * This file does not define:
 *
 *     assert
 *     assume
 *     cover
 *
 * again.
 */


/*
 * ============================================================================
 * 20. AST / SEMANTIC / IR PIPELINE
 * ============================================================================
 *
 * Module syntax:
 *
 *     hdlModuleDeclaration
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     name/type/generic/connection analysis
 *          |
 *          v
 *     semantic hardware module model
 *          |
 *          v
 *     canonical hardware semantic representation / IR
 *          |
 *          +--> optimization
 *          +--> verification
 *          +--> scheduling
 *          +--> synthesis
 *          +--> routing
 *          +--> placement
 *          +--> target lowering
 *
 * No module-specific backend IR is introduced here.
 */


/*
 * ============================================================================
 * 21. QUANTUM BOUNDARY
 * ============================================================================
 *
 * If a module participates in quantum/hybrid computation:
 *
 *     module syntax
 *          |
 *          v
 *     generic AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * This grammar does not create:
 *
 *     QuantumModuleIR
 *     HardwareQuantumIR
 *     HardwareGateIR
 *
 * quantum::ir remains the canonical quantum boundary.
 */


/*
 * ============================================================================
 * 22. COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler consumes module declarations after parsing.
 *
 * Required stages include:
 *
 *     parsing
 *     name resolution
 *     generic specialization
 *     type checking
 *     interface checking
 *     structural validation
 *     resource/capability analysis
 *     elaboration
 *     canonical IR lowering
 *     optimization
 *     scheduling
 *     routing
 *     synthesis
 *     target lowering
 *
 * Module parsing itself must not perform any of those operations.
 */


/*
 * ============================================================================
 * 23. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Module declarations describe source-level structure.
 *
 * They do not themselves execute.
 *
 * Runtime behavior belongs to the semantic constructs contained in a module
 * after compilation/elaboration.
 */


/*
 * ============================================================================
 * 24. ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Source locations must remain available for:
 *
 *     module declaration;
 *     module name;
 *     generic specialization;
 *     instance name;
 *     module reference;
 *     connection;
 *     module member.
 *
 * Semantic diagnostics should distinguish:
 *
 *     unknown module;
 *     duplicate module;
 *     invalid specialization;
 *     missing required connection;
 *     unknown connection;
 *     incompatible connection;
 *     invalid instance scope;
 *     recursive elaboration;
 *     unsatisfied capability;
 *     insufficient target resources.
 *
 * Syntax errors remain parser diagnostics.
 *
 * Resource/capability failures must not be reported as syntax errors merely
 * because the target cannot realize the program.
 */


/*
 * ============================================================================
 * 25. SECURITY CONTRACT
 * ============================================================================
 *
 * Module grammar performs no:
 *
 *     filesystem access;
 *     network access;
 *     process execution;
 *     hardware discovery;
 *     credential access;
 *     environment inspection;
 *     backend invocation.
 *
 * External module loading belongs to module/package infrastructure.
 *
 * ============================================================================
 * 26. HARD-CODING AUDIT
 * ============================================================================
 *
 * No universal resource limit appears in the grammar.
 *
 * Program constants such as:
 *
 *     32
 *     1024
 *     1000000
 *
 * remain ordinary expressions when supplied by the program.
 *
 * They must never be interpreted by this grammar as universal hardware
 * limits.
 *
 * ============================================================================
 * 27. SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust.
 *
 * Generated/parser/frontend implementations must remain:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust
 *
 * No `unsafe` is required or permitted by the Zamani compiler architecture.
 *
 * ============================================================================
 * 28. PRODUCTION TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests:
 *
 *     module Minimal {}
 *
 *     module Counter {
 *         ...
 *     }
 *
 *     module compute::Counter {}
 *
 *     instance counter: Counter;
 *
 *     instance counter: Counter<WIDTH = width>;
 *
 *     instance alu: compute::ALU {
 *         a = lhs,
 *         b = rhs,
 *         result = output
 *     };
 *
 *     instance pair: Pair {
 *         a,
 *         b
 *     };
 *
 * Required structural tests:
 *
 *     module containing ports
 *     module containing signals
 *     module containing wires
 *     module containing registers
 *     module containing memories
 *     module containing processes
 *     module containing pipelines
 *     module containing generate constructs
 *     nested generate constructs
 *     multiple module instances
 *     deeply qualified module names
 *     parameterized/generic modules
 *
 * Required negative tests:
 *
 *     missing module name
 *     missing module body
 *     malformed qualification
 *     malformed instance
 *     missing instance type
 *     malformed specialization
 *     malformed connection
 *     duplicate connection
 *     invalid member
 *
 * Required scalability tests:
 *
 *     many modules
 *     many instances
 *     large symbolic dimensions
 *     deeply nested logical modules
 *     large connection lists
 *     generated module families
 *
 * Tests MUST NOT establish artificial language maxima.
 *
 * ============================================================================
 * 29. DETERMINISM TEST
 * ============================================================================
 *
 * Identical source and identical grammar/lexer versions must produce
 * equivalent parse structures.
 *
 * Parsing must not depend on:
 *
 *     hardware availability;
 *     machine size;
 *     runtime state;
 *     network state;
 *     environment variables;
 *     randomness.
 *
 * ============================================================================
 * 30. FINAL OWNERSHIP CONTRACT
 * ============================================================================
 *
 * One responsibility per layer:
 *
 *     hardware-modules.g4
 *         -> module structure and logical instances
 *
 *     generate.g4
 *         -> structural generation
 *
 *     ports.g4
 *         -> ports
 *
 *     signals.g4
 *         -> signals
 *
 *     wires.g4
 *         -> wires/nets
 *
 *     registers.g4
 *         -> registers
 *
 *     memories.g4
 *         -> memories
 *
 *     pipelines.g4
 *         -> pipelines
 *
 *     hardware-generics.g4
 *         -> generics
 *
 *     resource subsystem
 *         -> requirements/capabilities/resources
 *
 *     hdl.g4
 *         -> composition
 *
 *     ZamaniParser.g4
 *         -> universal composition
 *
 *     frontend AST
 *         -> representation
 *
 *     semantic analysis
 *         -> meaning/validity
 *
 *     canonical IR
 *         -> executable semantic representation
 *
 *     backend
 *         -> target realization
 *
 * This is the required boundary for POCO-REAF.
 *
 * ============================================================================
 */