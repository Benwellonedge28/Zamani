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
 *     Canonical modular HDL module grammar.
 *
 * Purpose:
 *     Define the source-level syntax for reusable, parameterized, portable
 *     hardware modules and their logical composition.
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
 *     No embedded target-language code.
 *     No unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
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
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     grammar/antlr/ZamaniParser.g4
 *          |
 *          +--> HardwareModules
 *                  |
 *                  +--> HardwareGenerics
 *                  +--> HardwareParameters
 *                  +--> HardwareInterfaces
 *                  +--> HardwarePorts
 *                  +--> HardwareSignals
 *                  +--> HardwareWires
 *                  +--> HardwareRegisters
 *                  +--> HardwareClocks
 *                  +--> HardwareTiming
 *                  +--> HardwareCombinational
 *                  +--> HardwareSequential
 *                  +--> HardwareProcesses
 *                  +--> HardwareStateMachines
 *                  +--> HardwareMemories
 *                  +--> HardwarePipelines
 *                  +--> HardwareDialects
 *                  |
 *                  v
 *              domain-neutral AST
 *                  |
 *                  v
 *              semantic analysis
 *                  |
 *                  v
 *              canonical hardware semantic representation / IR
 *                  |
 *                  +--> optimization
 *                  +--> verification
 *                  +--> scheduling
 *                  +--> routing
 *                  +--> synthesis
 *                  +--> target lowering
 *                  |
 *                  v
 *              target realization
 *
 * This file is a parser component.
 *
 * It is NOT:
 *
 *     - the canonical lexer;
 *     - the canonical root parser;
 *     - the hardware semantic model;
 *     - a netlist;
 *     - an HDL implementation backend;
 *     - a synthesis engine;
 *     - a physical placement engine;
 *     - a routing engine;
 *     - a scheduler;
 *     - a target selector;
 *     - a resource discovery mechanism;
 *     - a vendor API;
 *     - a runtime;
 *     - a quantum IR;
 *     - a QEC implementation;
 *     - a ZQN implementation;
 *     - a HAL implementation.
 *
 * ============================================================================
 * SINGLE AUTHORITY
 * ============================================================================
 *
 * This file owns ONLY:
 *
 *     - hardware module declaration syntax;
 *     - module names;
 *     - module-level generic composition;
 *     - module-level contract syntax;
 *     - module body composition;
 *     - logical module instantiation;
 *     - logical instance specialization;
 *     - logical instance connections;
 *     - module-local metadata;
 *
 * This file does NOT own:
 *
 *     - identifiers;
 *     - qualified-name lexical rules;
 *     - expressions;
 *     - types;
 *     - generic declaration syntax;
 *     - parameter declaration syntax;
 *     - ports;
 *     - signals;
 *     - wires;
 *     - registers;
 *     - memories;
 *     - clocks;
 *     - timing;
 *     - processes;
 *     - combinational behavior;
 *     - sequential behavior;
 *     - pipelines;
 *     - interfaces;
 *     - state machines;
 *     - dialect definitions;
 *     - resources;
 *     - capabilities;
 *     - physical targets;
 *     - physical devices;
 *     - placement;
 *     - routing;
 *     - scheduling;
 *     - synthesis;
 *     - optimization;
 *     - calibration;
 *     - runtime execution.
 *
 * ============================================================================
 * CRITICAL REPOSITORY INTEGRATION
 * ============================================================================
 *
 * The repository currently contains:
 *
 *     grammar/hdl/hardware-modules.g4
 *     grammar/hdl/hardware-generics.g4
 *     grammar/hdl/hardware-parameters.g4
 *     grammar/hdl/hardware-interfaces.g4
 *     grammar/hdl/ports.g4
 *     grammar/hdl/signals.g4
 *     grammar/hdl/wires.g4
 *     grammar/hdl/registers.g4
 *     grammar/hdl/clocks.g4
 *     grammar/hdl/timing.g4
 *     grammar/hdl/combinational.g4
 *     grammar/hdl/sequential.g4
 *     grammar/hdl/processes.g4
 *     grammar/hdl/memories.g4
 *     grammar/hdl/pipelines.g4
 *     grammar/hdl/hardware-dialects.g4
 *
 * This file is the module composition boundary.
 *
 * The canonical parser composition layer is responsible for importing the
 * required parser grammars. This file deliberately does not create a second
 * parser root or a second lexer.
 *
 * ============================================================================
 * CANONICAL LEXER CONTRACT
 * ============================================================================
 *
 * Parser grammars consume the canonical lexer vocabulary:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file MUST NOT define lexer rules.
 *
 * It MUST NOT introduce local tokens for:
 *
 *     MODULE
 *     HDL
 *     INSTANCE
 *     PARAMETER
 *     GENERIC
 *     PORT
 *     SIGNAL
 *     DEVICE
 *     FPGA
 *     GPU
 *     CPU
 *     QPU
 *     ASIC
 *
 * The parser consumes the canonical token names established by the lexer
 * authority.
 *
 * Existing parser architecture uses the K_* keyword vocabulary, therefore
 * this module grammar uses K_MODULE, K_INSTANCE and other canonical K_ tokens
 * where such tokens already form part of the repository's parser contract.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Hardware modules express portable hardware intent.
 *
 * A module may describe:
 *
 *     - behavior;
 *     - structure;
 *     - interfaces;
 *     - logical storage;
 *     - logical resources;
 *     - parameterization;
 *     - timing intent;
 *     - capabilities;
 *     - requirements;
 *     - composition;
 *     - verification intent.
 *
 * A module does NOT inherently identify:
 *
 *     - a particular FPGA;
 *     - a particular ASIC;
 *     - a particular CPU;
 *     - a particular GPU;
 *     - a particular QPU;
 *     - a particular accelerator;
 *     - a physical board;
 *     - a physical pin;
 *     - a physical address;
 *     - a physical memory block;
 *     - a physical register;
 *     - a vendor primitive;
 *     - a routing path;
 *     - a placement location;
 *     - a fabrication process.
 *
 * Therefore:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * remains a semantic/compiler/runtime responsibility rather than a parser
 * implementation detail.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO language-level maxima in this file.
 *
 * In particular, this grammar contains no:
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
 *     MAX_PIPELINE_STAGES
 *     MAX_CLOCKS
 *     MAX_DEVICES
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *
 * Repetition uses ANTLR repetition operators:
 *
 *     *
 *     +
 *     ?
 *
 * Quantities appearing in source are program-level expressions.
 *
 * Physical realizability is checked downstream using:
 *
 *     semantic analysis
 *     capability analysis
 *     resource analysis
 *     compilation
 *     scheduling
 *     routing
 *     hardware abstraction
 *     target lowering
 *     runtime/deployment
 *
 * ============================================================================
 * GENERIC / PARAMETER SEPARATION
 * ============================================================================
 *
 * Generic declaration syntax is owned by:
 *
 *     hardware-generics.g4
 *
 * Parameter declaration syntax is owned by:
 *
 *     hardware-parameters.g4
 *
 * This file only consumes those contracts.
 *
 * It MUST NOT redefine:
 *
 *     hardwareGenericParameters
 *     hardwareGenericParameter
 *     hardwareGenericArguments
 *     hardwareGenericSpecialization
 *     hdlParameterDeclaration
 *     hdlParameterBlock
 *     hdlParameterBinding
 *     hdlParameterSpecialization
 *
 * This prevents multiple incompatible representations of the same concept.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser produces syntax structure only.
 *
 * Conceptual mapping:
 *
 *     hdlModuleDecl
 *          |
 *          v
 *     domain-neutral module declaration AST
 *          |
 *          +--> generic parameter nodes
 *          +--> contract nodes
 *          +--> member nodes
 *          +--> instance nodes
 *          +--> connection nodes
 *          |
 *          v
 *     semantic hardware module model
 *          |
 *          v
 *     canonical hardware IR
 *
 * This grammar MUST NOT introduce backend AST nodes such as:
 *
 *     FpgaModuleNode
 *     CpuModuleNode
 *     GpuModuleNode
 *     QpuModuleNode
 *     VendorModuleNode
 *     PhysicalModuleNode
 *     NetlistModuleNode
 *     PlacementModuleNode
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A hardware module may surround or interact with quantum computation.
 *
 * This file does not own quantum operation semantics.
 *
 * If a module contains quantum constructs:
 *
 *     source
 *       |
 *       v
 *     generic/domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum semantics
 *       |
 *       v
 *     quantum::ir
 *
 * quantum::ir remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NOT create:
 *
 *     QuantumModuleIR
 *     HardwareQuantumIR
 *     HardwareGateIR
 *
 * as competing quantum representations.
 *
 * QEC, ZQN, routing, scheduling, calibration and HAL remain downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing a module is deterministic with respect to:
 *
 *     source;
 *     language version;
 *     canonical lexical vocabulary;
 *     canonical grammar composition.
 *
 * This grammar MUST NOT depend on:
 *
 *     hardware state;
 *     CPU count;
 *     GPU availability;
 *     QPU availability;
 *     filesystem state;
 *     network state;
 *     wall-clock time;
 *     environment variables;
 *     randomness;
 *     deployment state.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains no actions.
 *
 * It performs no:
 *
 *     filesystem access;
 *     network access;
 *     command execution;
 *     hardware probing;
 *     credential access;
 *     target discovery;
 *     backend invocation.
 *
 * ============================================================================
 * PUBLIC API
 * ============================================================================
 *
 * The public rule consumed by the HDL composition layer is:
 *
 *     hdlModuleDecl
 *
 * The canonical HDL parser should expose this rule through its normal
 * composition boundary.
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
 * Canonical source form:
 *
 *     module Counter {
 *         ...
 *     }
 *
 * Parameterized:
 *
 *     module Counter<WIDTH> {
 *         ...
 *     }
 *
 * With a logical namespace:
 *
 *     module compute::Counter<WIDTH> {
 *         ...
 *     }
 *
 * The module name remains a logical source identifier.
 */
hdlModuleDecl
    : hdlModulePrefix?
      K_MODULE
      hdlModuleName
      hardwareGenericParameters?
      hdlModuleContractClause*
      hdlModuleBody
    ;


/*
 * ============================================================================
 * 2. OPTIONAL MODULE PREFIX
 * ============================================================================
 *
 * This rule provides a controlled extension point for module metadata.
 *
 * It deliberately does not introduce a new target-specific module kind.
 *
 * Attributes are supplied by the canonical attribute grammar.
 */
hdlModulePrefix
    : attribute+
    ;


/*
 * ============================================================================
 * 3. MODULE NAME
 * ============================================================================
 *
 * Module names are logical names.
 *
 * No depth limit is imposed on qualification.
 *
 * Examples:
 *
 *     Counter
 *
 *     compute::Counter
 *
 *     compute::integer::Counter
 *
 * The canonical identifier rule remains authoritative.
 */
hdlModuleName
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/*
 * ============================================================================
 * 4. MODULE CONTRACT
 * ============================================================================
 *
 * Contracts express source-level module requirements and externally visible
 * guarantees without selecting a physical implementation.
 *
 * `requires` is a semantic requirement.
 *
 * `provides` describes a capability or property exposed by the module.
 *
 * `constraint` expresses a semantic restriction.
 *
 * `prefer` expresses a non-binding implementation preference.
 *
 * These meanings are validated downstream.
 */
hdlModuleContractClause
    : hdlModuleRequiresClause
    | hdlModuleProvidesClause
    | hdlModuleConstraintClause
    | hdlModulePreferenceClause
    ;


/*
 * ============================================================================
 * 5. REQUIREMENTS
 * ============================================================================
 *
 * Examples:
 *
 *     requires capability("streaming");
 *
 *     requires qubits >= required_qubits;
 *
 *     requires memory >= required_memory;
 *
 *     requires latency <= maximum_latency;
 *
 * The grammar does not evaluate the expression.
 */
hdlModuleRequiresClause
    : K_REQUIRES expression SEMICOLON
    ;


/*
 * ============================================================================
 * 6. PROVIDED CAPABILITIES
 * ============================================================================
 *
 * Examples:
 *
 *     provides capability("vector.compute");
 *
 *     provides capability("streaming");
 *
 *     provides interface::protocol;
 *
 * The meaning of the expression is semantic.
 */
hdlModuleProvidesClause
    : K_PROVIDES expression SEMICOLON
    ;


/*
 * ============================================================================
 * 7. CONSTRAINTS
 * ============================================================================
 *
 * Constraints are semantic restrictions.
 *
 * They are not target selections.
 */
hdlModuleConstraintClause
    : K_CONSTRAINT expression SEMICOLON
    ;


/*
 * ============================================================================
 * 8. PREFERENCES
 * ============================================================================
 *
 * Preferences are non-binding implementation guidance.
 *
 * A preference must never make a program semantically dependent on a
 * particular physical device unless the program separately expresses that
 * requirement.
 */
hdlModulePreferenceClause
    : K_PREFER expression SEMICOLON
    ;


/*
 * ============================================================================
 * 9. MODULE BODY
 * ============================================================================
 *
 * The module body is an ordered sequence of module members.
 *
 * There is intentionally no finite member count.
 */
hdlModuleBody
    : LBRACE
      hdlModuleMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 10. MODULE MEMBER
 * ============================================================================
 *
 * This is the critical integration boundary.
 *
 * Module-specific syntax is composed from the owning HDL grammar components.
 *
 * This rule MUST NOT copy their implementations.
 */
hdlModuleMember
    : hdlModuleMemberAttributes*
      hdlModuleMemberCore
    ;


/*
 * ============================================================================
 * 11. MODULE MEMBER ATTRIBUTES
 * ============================================================================
 *
 * Attributes are metadata.
 *
 * They are not physical placement directives.
 */
hdlModuleMemberAttributes
    : attribute
    ;


/*
 * ============================================================================
 * 12. MODULE MEMBER DISPATCH
 * ============================================================================
 *
 * The names below are integration points supplied by the existing HDL
 * component grammars.
 *
 * Ownership:
 *
 *     hdlParameterDeclaration
 *         hardware-parameters.g4
 *
 *     hardwareGenericParameters
 *         hardware-generics.g4
 *
 *     hdlPortDeclaration
 *         ports.g4
 *
 *     hdlSignalDeclaration
 *         signals.g4
 *
 *     hdlWireDeclaration
 *         wires.g4
 *
 *     hdlRegisterDeclaration
 *         registers.g4
 *
 *     hdlClockDeclaration
 *         clocks.g4
 *
 *     hdlTimingDeclaration
 *         timing.g4
 *
 *     hdlCombinationalDeclaration
 *         combinational.g4
 *
 *     hdlSequentialDeclaration
 *         sequential.g4
 *
 *     hdlProcessDeclaration
 *         processes.g4
 *
 *     hdlStateMachineDeclaration
 *         state-machines.g4
 *
 *     hdlMemoryDeclaration
 *         memories.g4
 *
 *     hdlPipelineDeclaration
 *         pipelines.g4
 *
 *     hdlInterfaceDeclaration
 *         hardware-interfaces.g4
 *
 * This module grammar owns only the dispatch relationship.
 */
hdlModuleMemberCore
    : hdlParameterDeclaration
    | hdlParameterAliasDeclaration
    | hdlPortDeclaration
    | hdlSignalDeclaration
    | hdlWireDeclaration
    | hdlRegisterDeclaration
    | hdlClockDeclaration
    | hdlTimingDeclaration
    | hdlCombinationalDeclaration
    | hdlSequentialDeclaration
    | hdlProcessDeclaration
    | hdlStateMachineDeclaration
    | hdlMemoryDeclaration
    | hdlPipelineDeclaration
    | hdlInterfaceDeclaration
    | hdlModuleInstance
    | hdlContinuousAssignment
    | hdlModuleGenerate
    | hdlModuleBinding
    | hdlModuleVerification
    ;


/*
 * ============================================================================
 * 13. MODULE INSTANCE
 * ============================================================================
 *
 * A module instance is a logical source-level composition relationship.
 *
 * Examples:
 *
 *     instance counter: Counter;
 *
 *     instance counter: Counter<WIDTH = width>;
 *
 *     instance counter: compute::Counter<WIDTH = width>;
 *
 * The instance name is NOT:
 *
 *     a device ID;
 *     a physical address;
 *     a board identifier;
 *     a CPU ID;
 *     a GPU ID;
 *     a QPU ID.
 */
hdlModuleInstance
    : K_INSTANCE
      identifier
      COLON
      hdlModuleReference
      hdlModuleInstanceConnections?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. MODULE REFERENCE
 * ============================================================================
 *
 * Module resolution belongs to module/semantic analysis.
 *
 * The reference can identify:
 *
 *     local modules;
 *     imported modules;
 *     package modules;
 *     generated modules;
 *     dialect-provided modules;
 *     external modules.
 */
hdlModuleReference
    : identifier
      (DOUBLE_COLON identifier)*
      hardwareGenericSpecialization?
    ;


/*
 * ============================================================================
 * 15. INSTANCE CONNECTIONS
 * ============================================================================
 *
 * Connections are logical bindings between the instance interface and
 * expressions/signals/ports in the containing module.
 *
 * Physical routing is NOT implied.
 *
 * Example:
 *
 *     instance alu: ALU {
 *         input_a = a,
 *         input_b = b,
 *         output = result
 *     };
 */
hdlModuleInstanceConnections
    : LBRACE
      hdlModuleInstanceConnectionList?
      RBRACE
    ;


/*
 * ============================================================================
 * 16. CONNECTION LIST
 * ============================================================================
 */
hdlModuleInstanceConnectionList
    : hdlModuleInstanceConnection
      (COMMA hdlModuleInstanceConnection)*
      COMMA?
    ;


/*
 * ============================================================================
 * 17. CONNECTION
 * ============================================================================
 *
 * Named connections are preferred because they remain stable when an
 * interface evolves.
 *
 * The expression on the right is interpreted by semantic analysis.
 */
hdlModuleInstanceConnection
    : identifier ASSIGN expression
    ;


/*
 * ============================================================================
 * 18. EXPLICIT POSITIONAL CONNECTIONS
 * ============================================================================
 *
 * Positional connection syntax is retained as a compatibility/expressive
 * extension.
 *
 * Example:
 *
 *     instance pair: Pair {
 *         a,
 *         b
 *     };
 *
 * Semantic analysis is responsible for validating interface ordering.
 */
hdlModulePositionalConnectionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 19. MODULE GENERATION
 * ============================================================================
 *
 * Generate constructs are owned by the module composition boundary only when
 * they represent module-level structural generation.
 *
 * The detailed generation semantics remain owned by the HDL generation
 * subsystem.
 */
hdlModuleGenerate
    : K_GENERATE
      hdlModuleGenerateBody
    ;


hdlModuleGenerateBody
    : LBRACE
      hdlModuleGenerateItem*
      RBRACE
    ;


hdlModuleGenerateItem
    : hdlModuleGenerateFor
    | hdlModuleGenerateIf
    | hdlModuleGenerateMember
    ;


hdlModuleGenerateFor
    : K_FOR
      LPAREN
      hdlModuleGenerateBinding
      SEMICOLON
      expression
      SEMICOLON
      expression
      RPAREN
      hdlModuleGenerateBlock
    ;


hdlModuleGenerateBinding
    : identifier
      ASSIGN
      expression
    ;


hdlModuleGenerateIf
    : K_IF
      LPAREN
      expression
      RPAREN
      hdlModuleGenerateBlock
      (
          K_ELSE
          hdlModuleGenerateBlock
      )?
    ;


hdlModuleGenerateBlock
    : LBRACE
      hdlModuleGenerateMember*
      RBRACE
    ;


hdlModuleGenerateMember
    : hdlModuleMember
    ;


/*
 * ============================================================================
 * 20. MODULE BINDING
 * ============================================================================
 *
 * A module binding establishes a source-level relationship between logical
 * module entities.
 *
 * It does not perform physical placement or routing.
 *
 * The semantic layer determines whether the binding is legal.
 */
hdlModuleBinding
    : K_BIND
      hdlModuleBindingSubject
      ASSIGN
      hdlModuleBindingTarget
      SEMICOLON
    ;


hdlModuleBindingSubject
    : hdlModuleReference
    | identifier
    ;


hdlModuleBindingTarget
    : hdlModuleReference
    | identifier
    | expression
    ;


/*
 * ============================================================================
 * 21. CONTINUOUS ASSIGNMENT
 * ============================================================================
 *
 * This rule is a composition boundary for the existing HDL assignment
 * subsystem.
 *
 * The implementation of assignment semantics belongs to that subsystem.
 */
hdlContinuousAssignment
    : K_ASSIGN
      expression
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. MODULE VERIFICATION
 * ============================================================================
 *
 * Module-local verification constructs remain source-level assertions.
 *
 * Verification engines are downstream.
 *
 * This rule intentionally keeps the assertion body as an expression.
 */
hdlModuleVerification
    : hdlModuleAssert
    | hdlModuleAssume
    | hdlModuleCover
    ;


hdlModuleAssert
    : K_ASSERT
      expression
      SEMICOLON
    ;


hdlModuleAssume
    : K_ASSUME
      expression
      SEMICOLON
    ;


hdlModuleCover
    : K_COVER
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 23. MODULE ATTRIBUTE / METADATA CONTRACT
 * ============================================================================
 *
 * Attribute semantics remain outside this grammar.
 *
 * Legitimate examples may include source-level metadata such as:
 *
 *     @synthesizable
 *     @simulation
 *     @formal
 *     @portable
 *
 * A module attribute MUST NOT silently select:
 *
 *     FPGA device
 *     GPU device
 *     CPU core
 *     QPU
 *     vendor
 *     board
 *     physical pin
 *     physical memory block.
 *
 * Those are separate semantic/target concerns.
 */


/*
 * ============================================================================
 * 24. GENERIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file consumes:
 *
 *     hardwareGenericParameters
 *
 * from:
 *
 *     grammar/hdl/hardware-generics.g4
 *
 * and:
 *
 *     hardwareGenericSpecialization
 *
 * from the same grammar.
 *
 * No generic syntax is duplicated here.
 *
 * Therefore a future change to generic semantics does not require this file
 * to redefine generic parameter lists.
 */


/*
 * ============================================================================
 * 25. PARAMETER INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file consumes:
 *
 *     hdlParameterDeclaration
 *     hdlParameterAliasDeclaration
 *
 * from:
 *
 *     grammar/hdl/hardware-parameters.g4
 *
 * Parameter types/defaults/domains/constraints remain owned there.
 *
 * Module syntax only decides where a parameter declaration is legal.
 */


/*
 * ============================================================================
 * 26. INTERFACE INTEGRATION CONTRACT
 * ============================================================================
 *
 * Hardware interface declarations remain owned by:
 *
 *     grammar/hdl/hardware-interfaces.g4
 *
 * Module instances connect to logical interface members.
 *
 * Physical pin mapping remains downstream.
 */


/*
 * ============================================================================
 * 27. PORT / SIGNAL / REGISTER / MEMORY INTEGRATION
 * ============================================================================
 *
 * The module grammar deliberately does not redefine these concepts.
 *
 * Ownership remains:
 *
 *     ports.g4
 *     signals.g4
 *     wires.g4
 *     registers.g4
 *     memories.g4
 *
 * A module can contain those constructs through hdlModuleMemberCore.
 */


/*
 * ============================================================================
 * 28. CLOCK / TIMING INTEGRATION
 * ============================================================================
 *
 * Clock and timing declarations remain owned by:
 *
 *     clocks.g4
 *     timing.g4
 *
 * This file merely establishes their legal location inside a module.
 *
 * Timing closure and physical clock-tree construction are downstream.
 */


/*
 * ============================================================================
 * 29. PROCESS / COMBINATIONAL / SEQUENTIAL INTEGRATION
 * ============================================================================
 *
 * Behavioral HDL constructs remain owned by:
 *
 *     processes.g4
 *     combinational.g4
 *     sequential.g4
 *
 * Semantic analysis determines:
 *
 *     - driver conflicts;
 *     - combinational completeness;
 *     - sequential legality;
 *     - reset behavior;
 *     - clock relationships;
 *     - CDC correctness;
 *     - inferred storage.
 */


/*
 * ============================================================================
 * 30. PIPELINE INTEGRATION
 * ============================================================================
 *
 * Pipeline declarations are delegated to:
 *
 *     pipelines.g4
 *
 * Pipeline depth is source/program data.
 *
 * There is no language maximum for:
 *
 *     pipeline stages;
 *     lanes;
 *     operations;
 *     generated instances.
 *
 * Physical retiming and placement are downstream.
 */


/*
 * ============================================================================
 * 31. STATE-MACHINE INTEGRATION
 * ============================================================================
 *
 * State-machine syntax is delegated to:
 *
 *     state-machines.g4
 *
 * There is no parser-level limit on:
 *
 *     states;
 *     transitions;
 *     guards;
 *     actions.
 */


/*
 * ============================================================================
 * 32. HARDWARE DIALECT INTEGRATION
 * ============================================================================
 *
 * Vendor/technology-specific syntax must use the dialect extension mechanism.
 *
 * This file MUST NOT enumerate:
 *
 *     vendor A;
 *     vendor B;
 *     FPGA family X;
 *     ASIC family Y;
 *     accelerator Z;
 *
 * as permanent core module forms.
 *
 * Dialect identity remains semantic/source metadata.
 *
 * Dialect compatibility is resolved downstream.
 */


/*
 * ============================================================================
 * 33. HARDWARE TARGET INTEGRATION
 * ============================================================================
 *
 * A module can express requirements/preferences through contract expressions.
 *
 * Example:
 *
 *     requires capability("streaming");
 *
 *     requires memory >= required_memory;
 *
 *     prefer accelerator("vector");
 *
 * None of these select a physical device.
 *
 * Target realization occurs later.
 */


/*
 * ============================================================================
 * 34. RESOURCE INTEGRATION
 * ============================================================================
 *
 * A module may use source expressions representing resource requirements.
 *
 * Examples:
 *
 *     requires qubits >= n;
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("gpu.compute");
 *
 *     requires capability("quantum.measurement");
 *
 * The grammar does not evaluate or resolve those requirements.
 */


/*
 * ============================================================================
 * 35. NO PHYSICAL HARD-CODING
 * ============================================================================
 *
 * This grammar MUST remain valid for:
 *
 *     tiny hardware designs;
 *     embedded systems;
 *     microcontrollers;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     QPUs;
 *     heterogeneous systems;
 *     distributed systems;
 *     HPC systems;
 *     future hardware.
 *
 * No syntax in this file assumes:
 *
 *     32-bit registers;
 *     64-bit registers;
 *     fixed FPGA resources;
 *     fixed GPU count;
 *     fixed CPU count;
 *     fixed QPU count;
 *     fixed memory size;
 *     fixed topology;
 *     fixed pipeline depth.
 */


/*
 * ============================================================================
 * 36. NO PHYSICAL INSTANCE SEMANTICS
 * ============================================================================
 *
 * This is valid source-level naming:
 *
 *     instance gpu: ComputeUnit;
 *
 * It does NOT mean:
 *
 *     physical GPU number 0.
 *
 * Likewise:
 *
 *     instance qpu: QuantumController;
 *
 * does not identify a particular physical QPU.
 *
 * Physical mapping is downstream.
 */


/*
 * ============================================================================
 * 37. SOURCE-LEVEL VS TARGET-LEVEL SEPARATION
 * ============================================================================
 *
 * The following conceptual distinction is mandatory:
 *
 *     requirement
 *         = semantic condition
 *
 *     capability
 *         = property required/provided
 *
 *     preference
 *         = non-binding implementation guidance
 *
 *     hint
 *         = optional implementation guidance
 *
 *     realization
 *         = target-specific downstream decision
 *
 * This grammar represents only the source-level portions.
 */


/*
 * ============================================================================
 * 38. ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * The parser must preserve normal ANTLR token/source context.
 *
 * Semantic diagnostics should be able to identify:
 *
 *     - module name;
 *     - generic parameter;
 *     - contract;
 *     - module member;
 *     - instance;
 *     - instance specialization;
 *     - connection;
 *     - generated member.
 *
 * Diagnostics are implemented by the frontend/semantic infrastructure.
 *
 * This grammar contains no recovery actions or embedded Rust.
 */


/*
 * ============================================================================
 * 39. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing stable parser rule:
 *
 *     hdlModuleDecl
 *
 * is retained.
 *
 * Existing conceptual child-rule names are retained wherever they are already
 * established by the repository.
 *
 * The following older responsibilities are intentionally NOT retained here:
 *
 *     - duplicate generic grammar;
 *     - duplicate parameter grammar;
 *     - physical target selection;
 *     - physical placement;
 *     - fixed resource limits;
 *     - vendor-specific module enumeration.
 *
 * This keeps compatibility at the public module boundary while eliminating
 * duplicate internal authorities.
 */


/*
 * ============================================================================
 * 40. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden universal concepts:
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
 *     MAX_MODULES
 *     MAX_INSTANCES
 *     MAX_PORTS
 *
 * None are represented as grammar limits.
 *
 * Numeric values appearing inside expressions remain program data.
 */


/*
 * ============================================================================
 * 41. SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust.
 *
 * Consumers in the Zamani compiler are expected to target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The compiler implementation must use safe Rust only.
 *
 * This grammar requires no `unsafe`.
 */


/*
 * ============================================================================
 * 42. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when all of the following hold:
 *
 * [x] Module declaration has one canonical entry rule.
 * [x] Module naming is target-independent.
 * [x] Generic declarations are delegated.
 * [x] Generic specializations are delegated.
 * [x] Parameter declarations are delegated.
 * [x] Module contracts are source-level only.
 * [x] Module body composition is centralized here.
 * [x] Child HDL grammars remain owners of their constructs.
 * [x] Module instances are logical.
 * [x] Instance specialization is generic-aware.
 * [x] Instance connections are logical.
 * [x] Generation is parameterized.
 * [x] No fixed hardware capacity exists.
 * [x] No fixed topology exists.
 * [x] No physical device is selected.
 * [x] No vendor is hard-coded.
 * [x] No competing quantum IR exists.
 * [x] quantum::ir remains downstream.
 * [x] No embedded Rust exists.
 * [x] No unsafe exists.
 * [x] Source-level AST ownership is explicit.
 * [x] Semantic ownership is explicit.
 * [x] IR ownership is explicit.
 * [x] Resource/capability separation is preserved.
 * [x] POCO-REAF is preserved.
 *
 * The corresponding conformance tests belong under:
 *
 *     grammar/tests/hdl/
 *
 * and must cover:
 *
 *     - minimal module;
 *     - parameterized module;
 *     - generic module;
 *     - nested module;
 *     - qualified module;
 *     - module instance;
 *     - specialized instance;
 *     - named connections;
 *     - generated modules;
 *     - requirements;
 *     - capabilities;
 *     - preferences;
 *     - large symbolic dimensions;
 *     - quantum-aware hardware composition;
 *     - negative syntax;
 *     - ambiguity;
 *     - determinism;
 *     - scalability;
 *     - compatibility.
 *
 * ============================================================================
 */