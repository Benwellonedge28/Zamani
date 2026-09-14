/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/hardware-modules.g4
 *
 * Purpose:
 *     Canonical parser grammar for Zamani HDL/hardware-module declarations.
 *
 * Architectural position:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       +--> HardwareModules.g4
 *       |
 *       v
 *     frontend syntax/AST
 *       |
 *       v
 *     semantic hardware model
 *       |
 *       +--> classical IR
 *       +--> hardware/HDL IR
 *       +--> canonical resource model
 *       +--> quantum::ir where quantum semantics are actually present
 *       |
 *       v
 *     optimization / scheduling / routing / lowering
 *       |
 *       v
 *     target realization
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust.
 *     No Rust unsafe code is used or required.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - HDL module declaration syntax
 *     - HDL module identity/name syntax
 *     - HDL module generic parameters
 *     - HDL module parameters
 *     - HDL module requirements
 *     - HDL module capabilities declarations at syntax level
 *     - HDL module constraints at syntax level
 *     - HDL module body composition
 *     - HDL module member ordering
 *     - HDL module instantiation syntax
 *     - HDL module binding syntax
 *     - HDL module-level specialization syntax
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens
 *     - identifiers
 *     - generic expression syntax
 *     - general type syntax
 *     - ports
 *     - signals
 *     - wires
 *     - registers
 *     - clocks
 *     - timing
 *     - combinational logic
 *     - sequential logic
 *     - processes
 *     - state machines
 *     - memories
 *     - pipelines
 *     - hardware interfaces
 *     - target discovery
 *     - physical devices
 *     - placement
 *     - routing
 *     - scheduling
 *     - calibration
 *     - synthesis
 *     - simulation
 *     - optimization
 *     - vendor APIs
 *     - backend selection
 *     - resource discovery
 *     - runtime execution
 *     - canonical IR construction
 *
 * Those concepts belong to their owning grammar/compiler subsystem.
 *
 * ============================================================================
 * POCO-REAF PRINCIPLE
 * ============================================================================
 *
 * A module describes WHAT hardware computation exists.
 *
 * It does not inherently describe:
 *
 *     - how many physical devices exist;
 *     - how many CPUs exist;
 *     - how many FPGA resources exist;
 *     - how many ASIC cells exist;
 *     - how many quantum devices exist;
 *     - which physical board is selected;
 *     - which vendor is used;
 *     - which topology is available;
 *     - which physical address is used;
 *     - which machine is currently executing the program.
 *
 * Therefore this grammar deliberately contains:
 *
 *     no MAX_MODULES
 *     no MAX_PORTS
 *     no MAX_INSTANCES
 *     no MAX_PARAMETERS
 *     no MAX_WIDTH
 *     no MAX_DEPTH
 *     no MAX_DEVICES
 *     no MAX_CORES
 *     no MAX_LANES
 *     no MAX_MEMORY
 *     no MAX_FREQUENCY
 *     no MAX_CLOCKS
 *     no fixed topology
 *     no fixed device identifier
 *     no fixed vendor
 *
 * Actual limits are discovered or enforced by:
 *
 *     semantic analysis
 *     capability analysis
 *     resource analysis
 *     compilation
 *     scheduling
 *     hardware abstraction
 *     target lowering
 *     runtime
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * It consumes the canonical lexer vocabulary:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * The canonical parser imports this component.
 *
 * The importing parser supplies shared rules such as:
 *
 *     identifier
 *     expression
 *     typeExpr
 *     qualifiedName
 *     visibility
 *     attribute
 *
 * Domain-specific HDL rules are supplied by:
 *
 *     ports.g4
 *     signals.g4
 *     wires.g4
 *     registers.g4
 *     clocks.g4
 *     timing.g4
 *     combinational.g4
 *     sequential.g4
 *     processes.g4
 *     state-machines.g4
 *     memories.g4
 *     pipelines.g4
 *     hardware-generics.g4
 *     hardware-parameters.g4
 *     hardware-interfaces.g4
 *
 * This file must therefore remain a module-composition layer rather than
 * duplicating those grammars.
 *
 * ============================================================================
 * REQUIRED LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer must provide the following stable tokens:
 *
 *     HDL
 *     MODULE
 *     INSTANCE
 *     PARAMETER
 *     GENERIC
 *     REQUIRES
 *     ENSURES
 *     WHERE
 *     AS
 *     FROM
 *     IDENTIFIER
 *     INTEGER
 *     STRING
 *     ASSIGN
 *     COLON
 *     COMMA
 *     DOT
 *     DOUBLE_COLON
 *     SEMICOLON
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LT
 *     GT
 *
 * Existing canonical lexer tokens such as MODULE, REQUIRES, ENSURES,
 * WHERE, AS, IDENTIFIER and the punctuation tokens are reused rather than
 * redefined here.
 *
 * `HDL`, `INSTANCE`, `PARAMETER`, and `GENERIC` are required lexical
 * reservations for the canonical HDL language surface.
 *
 * These tokens must be added to the canonical lexer/keyword ownership before
 * this grammar is assembled into the production parser.
 *
 * No parser action or semantic predicate is permitted to compensate for a
 * missing keyword token.
 *
 * ============================================================================
 * MODULE DECLARATION
 * ============================================================================
 *
 * Canonical forms include:
 *
 *     hdl module Counter {
 *         ...
 *     }
 *
 *     hdl module ProcessingElement<T> {
 *         ...
 *     }
 *
 *     hdl module Accelerator<WIDTH, LANES> {
 *         ...
 *     }
 *
 *     hdl module namespace::ProcessingElement {
 *         ...
 *     }
 *
 * The module name is logical.
 *
 * It is NOT:
 *
 *     a filesystem path
 *     a device identifier
 *     a physical address
 *     a vendor name
 *     a board identifier
 *     a target identifier
 *
 * ============================================================================
 */

parser grammar HardwareModules;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the canonical rule that replaces the old monolithic hdlModuleDecl
 * implementation in grammar/Zamani.g4.
 *
 * Example:
 *
 *     hdl module Counter {
 *         ...
 *     }
 */
hdlModuleDecl
    : hdlModuleHeader hdlModuleBody
    ;


/*
 * ============================================================================
 * 2. MODULE HEADER
 * ============================================================================
 *
 * Module identity is independent of implementation target.
 */
hdlModuleHeader
    : HDL MODULE hdlModuleName hdlModuleGenerics? hdlModuleParameters?
      hdlModuleRequirements?
      hdlModuleGuarantees?
    ;


/*
 * ============================================================================
 * 3. MODULE NAME
 * ============================================================================
 *
 * A module name may be a simple identifier or a logical qualified name.
 *
 * The grammar does not impose a depth limit.
 */
hdlModuleName
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/*
 * ============================================================================
 * 4. GENERIC PARAMETERS
 * ============================================================================
 *
 * Generics express compile-time structural abstraction.
 *
 * They are intentionally not tied to a physical machine size.
 *
 * Examples:
 *
 *     hdl module VectorUnit<WIDTH> { ... }
 *
 *     hdl module MatrixUnit<ROWS, COLS, ELEMENT> { ... }
 *
 *     hdl module QuantumInterface<QUBITS> { ... }
 *
 * The semantic layer determines whether a specialization is realizable.
 */
hdlModuleGenerics
    : LT hdlGenericParameterList GT
    ;


hdlGenericParameterList
    : hdlGenericParameter
      (COMMA hdlGenericParameter)*
      COMMA?
    ;


hdlGenericParameter
    : identifier
      (COLON typeExpr)?
      (ASSIGN expression)?
    ;


/*
 * ============================================================================
 * 5. MODULE PARAMETERS
 * ============================================================================
 *
 * Parameters describe module-level values/configuration.
 *
 * They do not automatically represent physical hardware properties.
 *
 * For example:
 *
 *     parameter WIDTH = 64
 *
 * means the module has a parameterized width.
 *
 * It does NOT mean:
 *
 *     every target has 64-bit hardware.
 *
 * The specialization/lowering layer decides how that parameter is realized.
 */
hdlModuleParameters
    : LPAREN hdlModuleParameterList? RPAREN
    ;


hdlModuleParameterList
    : hdlModuleParameter
      (COMMA hdlModuleParameter)*
      COMMA?
    ;


hdlModuleParameter
    : PARAMETER identifier
      (COLON typeExpr)?
      (ASSIGN expression)?
    ;


/*
 * ============================================================================
 * 6. MODULE REQUIREMENTS
 * ============================================================================
 *
 * Requirements express semantic requirements.
 *
 * They are deliberately separated from target selection.
 *
 * Example:
 *
 *     requires expression
 *
 * means:
 *
 *     this module needs the stated property.
 *
 * It does NOT mean:
 *
 *     use device X
 *     use board Y
 *     use vendor Z
 *     use N resources
 *
 * Those are target/resource concerns.
 */
hdlModuleRequirements
    : REQUIRES hdlRequirementBody
    ;


hdlRequirementBody
    : LBRACE hdlRequirementEntry* RBRACE
    ;


hdlRequirementEntry
    : expression SEMICOLON
    ;


/*
 * ============================================================================
 * 7. MODULE GUARANTEES
 * ============================================================================
 *
 * Guarantees describe source-level semantic properties that the module
 * promises after successful elaboration/lowering.
 *
 * They are not performance promises unless explicitly represented by a
 * semantic resource/performance contract.
 */
hdlModuleGuarantees
    : ENSURES hdlGuaranteeBody
    ;


hdlGuaranteeBody
    : LBRACE hdlGuaranteeEntry* RBRACE
    ;


hdlGuaranteeEntry
    : expression SEMICOLON
    ;


/*
 * ============================================================================
 * 8. MODULE BODY
 * ============================================================================
 *
 * The body is intentionally composed from domain-owned HDL declarations.
 *
 * This file does not redefine ports, clocks, signals, memories, processes,
 * etc.
 *
 * This prevents duplicate hardware semantics and keeps ownership explicit.
 */
hdlModuleBody
    : LBRACE hdlModuleMember* RBRACE
    ;


/*
 * ============================================================================
 * 9. MODULE MEMBERS
 * ============================================================================
 *
 * The following rules are owned by other HDL grammar components.
 *
 * Their exact definitions are intentionally referenced rather than copied.
 *
 * Dependency direction:
 *
 *     HardwareModules
 *          |
 *          +--> Ports
 *          +--> Signals
 *          +--> Wires
 *          +--> Registers
 *          +--> Clocks
 *          +--> Timing
 *          +--> Combinational
 *          +--> Sequential
 *          +--> Processes
 *          +--> StateMachines
 *          +--> Memories
 *          +--> Pipelines
 *          +--> HardwareInterfaces
 *          +--> HardwareGenerics
 *          +--> HardwareParameters
 *
 * No child grammar is permitted to redefine hdlModuleDecl.
 */
hdlModuleMember
    : hdlModuleAttribute
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
    ;


/*
 * ============================================================================
 * 10. MODULE ATTRIBUTES
 * ============================================================================
 *
 * Module attributes remain syntax-level metadata.
 *
 * They must not become hidden target selection mechanisms.
 *
 * Examples of legitimate metadata:
 *
 *     @pure
 *     @synthesizable
 *     @simulation
 *     @formal
 *     @portable
 *
 * Whether an attribute is valid and what it means belongs to semantic
 * validation.
 *
 * Attribute syntax is deliberately generic.
 */
hdlModuleAttribute
    : AT identifier
      (
          LPAREN hdlAttributeArgumentList? RPAREN
      )?
    ;


hdlAttributeArgumentList
    : hdlAttributeArgument
      (COMMA hdlAttributeArgument)*
      COMMA?
    ;


hdlAttributeArgument
    : identifier ASSIGN expression
    | expression
    ;


/*
 * ============================================================================
 * 11. MODULE INSTANCE
 * ============================================================================
 *
 * Module instances compose reusable HDL modules.
 *
 * Example:
 *
 *     instance alu0: ArithmeticUnit;
 *
 *     instance alu0: ArithmeticUnit<WIDTH = 64>;
 *
 *     instance memory0: Memory<DEPTH = DEPTH>;
 *
 * The instance identifier is a source-level symbol.
 *
 * It is not a physical device identifier.
 */
hdlModuleInstance
    : INSTANCE identifier COLON hdlModuleReference
      hdlInstanceArguments?
      hdlInstanceConnections?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. MODULE REFERENCE
 * ============================================================================
 *
 * The referenced module may be:
 *
 *     local
 *     imported
 *     package-provided
 *     generated
 *     dialect-provided
 *     externally supplied
 *
 * Resolution belongs to the module/semantic resolver.
 */
hdlModuleReference
    : identifier
      (DOUBLE_COLON identifier)*
      hdlSpecializationArguments?
    ;


/*
 * ============================================================================
 * 13. SPECIALIZATION ARGUMENTS
 * ============================================================================
 *
 * This is intentionally distinct from generic declarations.
 *
 * Declaration:
 *
 *     <WIDTH, LANES>
 *
 * Specialization:
 *
 *     <WIDTH = 64, LANES = 8>
 *
 * No finite number of arguments is imposed.
 */
hdlSpecializationArguments
    : LT hdlSpecializationArgumentList GT
    ;


hdlSpecializationArgumentList
    : hdlSpecializationArgument
      (COMMA hdlSpecializationArgument)*
      COMMA?
    ;


hdlSpecializationArgument
    : identifier ASSIGN expression
    | expression
    ;


/*
 * ============================================================================
 * 14. INSTANCE ARGUMENTS
 * ============================================================================
 *
 * Parenthesized arguments are useful where a module exposes explicit
 * configuration values distinct from compile-time generic specialization.
 */
hdlInstanceArguments
    : LPAREN hdlInstanceArgumentList? RPAREN
    ;


hdlInstanceArgumentList
    : hdlInstanceArgument
      (COMMA hdlInstanceArgument)*
      COMMA?
    ;


hdlInstanceArgument
    : identifier ASSIGN expression
    | expression
    ;


/*
 * ============================================================================
 * 15. INSTANCE CONNECTIONS
 * ============================================================================
 *
 * Connections are source-level bindings.
 *
 * They do not imply physical routing.
 *
 * Physical routing belongs to the hardware/routing/scheduling layers.
 */
hdlInstanceConnections
    : LBRACE hdlConnectionList? RBRACE
    ;


hdlConnectionList
    : hdlConnection
      (COMMA hdlConnection)*
      COMMA?
    ;


hdlConnection
    : identifier ASSIGN expression
    ;


/*
 * ============================================================================
 * 16. MODULE BINDING
 * ============================================================================
 *
 * A binding associates a logical module interface/entity with another
 * language-level entity.
 *
 * Example:
 *
 *     bind implementation = implementationName;
 *
 * The grammar does not decide whether the target is:
 *
 *     RTL
 *     synthesized logic
 *     an FPGA implementation
 *     an ASIC implementation
 *     a simulator
 *     a quantum backend
 *     a software implementation
 *
 * Such interpretation belongs downstream.
 */
hdlModuleBinding
    : BIND identifier ASSIGN hdlBindingTarget SEMICOLON
    ;


hdlBindingTarget
    : qualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 17. GENERATION
 * ============================================================================
 *
 * Generate constructs express structural replication/selection without
 * imposing a fixed maximum.
 *
 * Examples:
 *
 *     generate i in range {
 *         ...
 *     }
 *
 *     generate lane in lanes {
 *         ...
 *     }
 *
 * The actual generated cardinality is semantic/compiler data.
 */
hdlModuleGenerate
    : GENERATE identifier IN expression
      hdlGenerateCondition?
      hdlModuleBody
    ;


hdlGenerateCondition
    : WHEN expression
    ;


/*
 * ============================================================================
 * 18. EMPTY MODULES
 * ============================================================================
 *
 * Empty module bodies are syntactically legal.
 *
 * Semantic validation determines whether an empty module is meaningful.
 *
 * This keeps syntax separate from design-policy decisions.
 */


/*
 * ============================================================================
 * 19. RECURSIVE MODULE COMPOSITION
 * ============================================================================
 *
 * The grammar permits modules to instantiate other modules without imposing
 * a depth limit.
 *
 * It does NOT permit unrestricted recursive semantic elaboration automatically.
 *
 * For example:
 *
 *     A -> B
 *     B -> A
 *
 * is syntactically representable.
 *
 * Detecting illegal elaboration cycles belongs to semantic/module analysis.
 *
 * This is intentional:
 *
 *     syntax != elaboration policy
 */


/*
 * ============================================================================
 * 20. HARDWARE/QUANTUM BOUNDARY
 * ============================================================================
 *
 * HDL modules may contain quantum-related declarations when the relevant
 * child grammar is imported.
 *
 * This file must never create a second quantum semantic representation.
 *
 * Example:
 *
 *     hdl module QuantumController {
 *         ...
 *     }
 *
 * If quantum operations occur inside the module, their semantic lowering
 * eventually crosses the canonical quantum boundary:
 *
 *     frontend syntax
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * The HDL grammar does not own:
 *
 *     QubitId
 *     PhysicalQubitId
 *     QuantumGate
 *     quantum topology
 *     calibration
 *     QEC algorithms
 *     noise models
 *
 * Those remain owned by their repository subsystems.
 */


/*
 * ============================================================================
 * 21. HARDWARE RESOURCE BOUNDARY
 * ============================================================================
 *
 * Module syntax may express requirements such as:
 *
 *     requires {
 *         capability::memory;
 *         capability::parallel_execution;
 *     }
 *
 * but it must not encode:
 *
 *     use gpu0
 *     use qpu0
 *     use exactly 32 cores
 *     use exactly 128 lanes
 *
 * unless such information is intentionally part of an explicitly
 * target-specific program.
 *
 * Portable source should normally express:
 *
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     hints
 *
 * rather than physical identities.
 */


/*
 * ============================================================================
 * 22. TARGET INDEPENDENCE
 * ============================================================================
 *
 * The following constructs are deliberately absent:
 *
 *     FPGA0
 *     GPU0
 *     CPU0
 *     QPU0
 *     ASIC0
 *     BOARD0
 *
 * Physical resource identities belong to target/resource/deployment layers.
 *
 * A module name such as:
 *
 *     GPUAccelerator
 *
 * is still legal as an ordinary language identifier if the language permits
 * it, but the grammar does not assign physical semantics to that spelling.
 */


/*
 * ============================================================================
 * 23. NO FIXED WIDTHS
 * ============================================================================
 *
 * This file never defines:
 *
 *     WIDTH <= 32
 *     WIDTH <= 64
 *     WIDTH <= 128
 *
 * or equivalent parser restrictions.
 *
 * A user may write:
 *
 *     <WIDTH>
 *
 * and semantic analysis determines whether a specialization is valid.
 *
 * This permits the same source program to be specialized for:
 *
 *     tiny hardware
 *     conventional hardware
 *     large accelerators
 *     future hardware
 *
 * subject to available resources.
 */


/*
 * ============================================================================
 * 24. NO FIXED INSTANCE COUNTS
 * ============================================================================
 *
 * This grammar intentionally uses:
 *
 *     hdlModuleMember*
 *
 * rather than:
 *
 *     hdlModuleMember{1,64}
 *
 * or another artificial bound.
 *
 * The parser therefore does not impose a module-instance ceiling.
 */


/*
 * ============================================================================
 * 25. NO FIXED MODULE DEPTH
 * ============================================================================
 *
 * Qualified names use:
 *
 *     (DOUBLE_COLON identifier)*
 *
 * rather than a finite sequence.
 *
 * Therefore:
 *
 *     a
 *     a::b
 *     a::b::c
 *     ...
 *
 * are syntactically governed by the same rule.
 */


/*
 * ============================================================================
 * 26. SEMANTIC OWNERSHIP
 * ============================================================================
 *
 * After parsing:
 *
 *     HDL module syntax
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic HDL model
 *          |
 *          +--> type checking
 *          +--> capability checking
 *          +--> resource analysis
 *          +--> elaboration
 *          +--> verification
 *          |
 *          v
 *     canonical hardware/HDL representation
 *
 * The grammar itself must not instantiate IR structures.
 */


/*
 * ============================================================================
 * 27. COMPILER INTEGRATION
 * ============================================================================
 *
 * Required compiler flow:
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
 *     hdlModuleDecl
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic HDL analysis
 *       |
 *       +--> names
 *       +--> types
 *       +--> effects
 *       +--> capabilities
 *       +--> resources
 *       +--> constraints
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> optimization
 *       +--> scheduling
 *       +--> routing
 *       +--> hardware lowering
 *       +--> runtime
 *
 * This file does not directly depend on any of those downstream stages.
 */


/*
 * ============================================================================
 * 28. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Module syntax must not contain a scheduler implementation.
 *
 * A module may contain timing/clock/process declarations supplied by the
 * appropriate child grammars.
 *
 * The scheduling subsystem later consumes semantic timing/resource
 * information.
 *
 * Therefore:
 *
 *     grammar/hdl/hardware-modules.g4
 *
 * does NOT depend on:
 *
 *     src/quantum/scheduling/
 *
 * and scheduling must not be imported into this grammar.
 */


/*
 * ============================================================================
 * 29. HARDWARE HAL INTEGRATION
 * ============================================================================
 *
 * The grammar describes a hardware abstraction.
 *
 * Hardware discovery and physical capabilities are supplied later by the
 * hardware subsystem.
 *
 * Therefore this grammar must not contain:
 *
 *     device discovery
 *     calibration queries
 *     topology queries
 *     backend selection
 *     provider API calls
 *     physical resource enumeration
 */


/*
 * ============================================================================
 * 30. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Module requirements become semantic resource requirements.
 *
 * Example:
 *
 *     requires {
 *         capability::parallel_execution;
 *     }
 *
 * can eventually become a resource/capability requirement.
 *
 * The resource manager determines whether the current target can satisfy it.
 *
 * The grammar does not decide that.
 */


/*
 * ============================================================================
 * 31. OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * Optimization consumes the semantic/IR representation produced after
 * parsing and analysis.
 *
 * This file must never:
 *
 *     perform constant folding
 *     choose an FPGA implementation
 *     select a gate decomposition
 *     choose a routing path
 *     choose a schedule
 *     rewrite hardware
 *
 * Those are compiler/backend responsibilities.
 */


/*
 * ============================================================================
 * 32. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic predicates
 *     no actions
 *     no Rust
 *     no filesystem access
 *     no network access
 *     no environment access
 *     no time access
 *     no randomness
 *     no target discovery
 *
 * Therefore parsing is independent of execution environment.
 */


/*
 * ============================================================================
 * 33. SECURITY
 * ============================================================================
 *
 * HDL module syntax does not grant:
 *
 *     hardware access
 *     MMIO access
 *     DMA access
 *     device access
 *     privileged execution
 *     network access
 *     filesystem access
 *
 * A declaration such as:
 *
 *     hdl module DeviceController
 *
 * is merely syntax.
 *
 * Capability authorization belongs to the compiler/runtime security model.
 */


/*
 * ============================================================================
 * 34. COMPATIBILITY
 * ============================================================================
 *
 * Existing legacy syntax:
 *
 *     hdl module NAME { ... }
 *
 * is preserved by hdlModuleDecl.
 *
 * The old monolithic rule in:
 *
 *     grammar/Zamani.g4
 *
 * must eventually be removed from the canonical grammar after this component
 * is imported into the authoritative parser.
 *
 * There must be exactly one authoritative definition of:
 *
 *     hdlModuleDecl
 *
 * after migration.
 *
 * Do NOT retain both definitions.
 */


/*
 * ============================================================================
 * 35. LEGACY MIGRATION
 * ============================================================================
 *
 * Old grammar:
 *
 *     hdlModuleDecl:
 *         'hdl' 'module' IDENTIFIER '{'
 *             hdlPort*
 *             hdlStatement*
 *         '}'
 *
 * is insufficient because it:
 *
 *     - mixes module structure with child HDL domains;
 *     - provides no generic specialization;
 *     - provides no parameter model;
 *     - provides no requirement model;
 *     - provides no module-instance model;
 *     - provides no scalable structural generation;
 *     - encourages hardware semantics to accumulate in one rule.
 *
 * This file separates those responsibilities.
 *
 * Migration order:
 *
 *     1. Establish canonical lexer tokens.
 *     2. Add/import HardwareModules.
 *     3. Import child HDL grammars.
 *     4. Update canonical parser composition.
 *     5. Remove the duplicate hdlModuleDecl from grammar/Zamani.g4.
 *     6. Update grammar documentation.
 *     7. Add parser tests.
 *     8. Add cross-domain semantic tests.
 */


/*
 * ============================================================================
 * 36. TEST CONTRACT
 * ============================================================================
 *
 * The implementation is incomplete until the following categories exist.
 *
 * --------------------------------------------------------------------------
 * Positive tests
 * --------------------------------------------------------------------------
 *
 *     hdl module Counter {
 *     }
 *
 *     hdl module Counter {
 *         ...
 *     }
 *
 *     hdl module ProcessingElement<WIDTH> {
 *         ...
 *     }
 *
 *     hdl module ProcessingElement<WIDTH, LANES> {
 *         ...
 *     }
 *
 *     hdl module ProcessingElement<WIDTH = 64> {
 *         ...
 *     }
 *
 *     hdl module org::zamani::ProcessingElement {
 *         ...
 *     }
 *
 * --------------------------------------------------------------------------
 * Parameter tests
 * --------------------------------------------------------------------------
 *
 *     hdl module M(PARAMETER WIDTH = 8) {
 *     }
 *
 *     hdl module M(PARAMETER WIDTH: int = 8) {
 *     }
 *
 *     hdl module M(PARAMETER WIDTH, PARAMETER LANES) {
 *     }
 *
 * --------------------------------------------------------------------------
 * Specialization tests
 * --------------------------------------------------------------------------
 *
 *     instance a: Module<WIDTH = 64>;
 *
 *     instance a: Module<WIDTH = 64, LANES = 8>;
 *
 *     instance a: Module<64, 8>;
 *
 * --------------------------------------------------------------------------
 * Connection tests
 * --------------------------------------------------------------------------
 *
 *     instance a: Module {
 *         input = source,
 *         output = destination
 *     };
 *
 * --------------------------------------------------------------------------
 * Requirement tests
 * --------------------------------------------------------------------------
 *
 *     hdl module M requires {
 *         capability::parallel_execution;
 *     } {
 *     }
 *
 * --------------------------------------------------------------------------
 * Guarantee tests
 * --------------------------------------------------------------------------
 *
 *     hdl module M ensures {
 *         property::deterministic;
 *     } {
 *     }
 *
 * --------------------------------------------------------------------------
 * Cross-domain tests
 * --------------------------------------------------------------------------
 *
 * HDL + classical
 * HDL + quantum
 * HDL + resource requirements
 * HDL + timing
 * HDL + scheduling metadata
 * HDL + distributed execution
 * HDL + accelerator declarations
 *
 * --------------------------------------------------------------------------
 * Negative tests
 * --------------------------------------------------------------------------
 *
 * Reject:
 *
 *     hdl
 *
 *     hdl module
 *
 *     hdl module 123
 *
 *     hdl module M<
 *
 *     hdl module M<>
 *         // if empty generic lists are prohibited by semantic policy
 *
 *     instance;
 *
 *     instance x;
 *
 *     instance x:
 *
 * --------------------------------------------------------------------------
 * Scalability tests
 * --------------------------------------------------------------------------
 *
 * Generate source with:
 *
 *     arbitrarily many module members
 *     arbitrarily many parameters
 *     arbitrarily many generic parameters
 *     arbitrarily deep qualified module names
 *     arbitrarily many instances
 *     arbitrarily many connection entries
 *
 * The grammar must contain no artificial cardinality limit.
 *
 * --------------------------------------------------------------------------
 * Determinism tests
 * --------------------------------------------------------------------------
 *
 * Identical source must produce identical parse-tree structure.
 *
 * --------------------------------------------------------------------------
 * Round-trip tests
 * --------------------------------------------------------------------------
 *
 * Source
 *   -> lexer
 *   -> parser
 *   -> AST
 *   -> canonical formatter/serializer
 *   -> parser
 *
 * must preserve module semantics.
 */


/*
 * ============================================================================
 * 37. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file must remain free from:
 *
 *     MAX_PORTS
 *     MAX_INSTANCES
 *     MAX_PARAMETERS
 *     MAX_GENERICS
 *     MAX_MODULE_DEPTH
 *     MAX_MODULES
 *     MAX_WIDTH
 *     MAX_LANES
 *     MAX_MEMORY
 *     MAX_CLOCKS
 *     MAX_PIPELINES
 *
 * It must also contain no:
 *
 *     vendor names
 *     board names
 *     device IDs
 *     CPU IDs
 *     GPU IDs
 *     FPGA IDs
 *     QPU IDs
 *     fixed physical addresses
 *     fixed topology
 *     fixed register counts
 *     fixed resource counts
 */


/*
 * ============================================================================
 * 38. NO PHYSICAL RESOURCE SEMANTICS
 * ============================================================================
 *
 * These identifiers remain syntactic names:
 *
 *     cpu
 *     gpu
 *     fpga
 *     qpu
 *     accelerator
 *     device
 *     memory
 *
 * The grammar does not assign physical identity merely from spelling.
 *
 * Example:
 *
 *     hdl module GPUController
 *
 * does not mean that a GPU exists.
 *
 * Semantic analysis may interpret the name according to the program's declared
 * meaning, but the grammar itself remains architecture-neutral.
 */


/*
 * ============================================================================
 * 39. POCO-REAF PROPERTY
 * ============================================================================
 *
 * A valid source-level module should be capable of surviving changes in:
 *
 *     machine size
 *     machine topology
 *     accelerator count
 *     memory capacity
 *     FPGA fabric
 *     ASIC implementation
 *     processor architecture
 *     quantum hardware
 *     simulator
 *     runtime environment
 *     deployment topology
 *
 * without requiring the source grammar to change.
 *
 * Physical realization is downstream.
 */


/*
 * ============================================================================
 * 40. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when:
 *
 * [ ] It is the sole owner of hdlModuleDecl.
 *
 * [ ] The canonical parser imports this grammar.
 *
 * [ ] ZamaniLexer provides the required HDL/module tokens.
 *
 * [ ] Identifier syntax comes from the canonical identifier grammar.
 *
 * [ ] Type syntax comes from the canonical type grammar.
 *
 * [ ] Expression syntax comes from the canonical expression grammar.
 *
 * [ ] Port syntax comes from ports.g4.
 *
 * [ ] Signal syntax comes from signals.g4.
 *
 * [ ] Wire syntax comes from wires.g4.
 *
 * [ ] Register syntax comes from registers.g4.
 *
 * [ ] Clock syntax comes from clocks.g4.
 *
 * [ ] Timing syntax comes from timing.g4.
 *
 * [ ] Combinational syntax comes from combinational.g4.
 *
 * [ ] Sequential syntax comes from sequential.g4.
 *
 * [ ] Process syntax comes from processes.g4.
 *
 * [ ] State-machine syntax comes from state-machines.g4.
 *
 * [ ] Memory syntax comes from memories.g4.
 *
 * [ ] Pipeline syntax comes from pipelines.g4.
 *
 * [ ] Hardware interface syntax comes from hardware-interfaces.g4.
 *
 * [ ] Generic/parameter semantics have a downstream owner.
 *
 * [ ] No hardware resource count is hard-coded.
 *
 * [ ] No machine topology is hard-coded.
 *
 * [ ] No physical device identity is hard-coded.
 *
 * [ ] No vendor-specific syntax is embedded in the core module grammar.
 *
 * [ ] No Rust code is embedded.
 *
 * [ ] No unsafe code is required.
 *
 * [ ] No semantic predicates are used.
 *
 * [ ] No runtime/hardware discovery occurs during parsing.
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
 * [ ] Cross-domain tests exist.
 *
 * [ ] Legacy hdlModuleDecl behavior is covered by compatibility tests.
 *
 * [ ] The duplicate legacy hdlModuleDecl has been removed from the
 *     authoritative monolithic grammar.
 *
 * [ ] Documentation identifies this file as the canonical HDL module syntax
 *     owner.
 *
 * [ ] The resulting AST/semantic model does not become a second IR.
 *
 * [ ] Quantum constructs ultimately lower through the repository's canonical
 *     quantum semantic boundary rather than through an HDL-specific quantum
 *     representation.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */