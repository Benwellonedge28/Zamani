/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/memories.g4
 *
 * Status:
 *     Canonical HDL memory-declaration parser component.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL HDL MEMORY STRUCTURE AND MEMORY INTERFACE INTENT.
 *
 * It describes logical memory declarations that may be lowered to:
 *
 *     - FPGA memories;
 *     - ASIC memories;
 *     - SRAM;
 *     - DRAM;
 *     - HBM;
 *     - distributed memory;
 *     - accelerator memory;
 *     - embedded memory;
 *     - future memory technologies;
 *     - heterogeneous memory systems.
 *
 * The grammar does NOT select a physical memory technology.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     HDL dispatcher
 *          |
 *          v
 *     hdlMemoryDeclaration
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     type analysis              resource/capability analysis
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *              semantic memory model
 *                        |
 *                        v
 *              canonical hardware/HDL IR
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *     optimization   scheduling     verification
 *          |             |             |
 *          +-------------+-------------+
 *                        |
 *                        v
 *                 target lowering
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *        FPGA          ASIC        other targets
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - HDL memory declaration syntax;
 *   - logical memory identity;
 *   - memory element/storage type syntax;
 *   - symbolic dimensions;
 *   - memory generic parameters;
 *   - logical memory ports;
 *   - logical access intent;
 *   - initialization intent;
 *   - memory-local properties;
 *   - memory-local requirements;
 *   - memory-local constraints;
 *   - memory-local preferences;
 *   - memory-local hints;
 *   - memory-local capability requirements;
 *   - nested logical memories where the HDL composition permits them;
 *   - source-level memory metadata.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical definitions;
 *   - identifiers;
 *   - general expressions;
 *   - general types;
 *   - general memory semantics;
 *   - ownership;
 *   - borrowing;
 *   - lifetimes;
 *   - allocation;
 *   - virtual memory;
 *   - physical addresses;
 *   - physical banks;
 *   - physical ports;
 *   - cache implementation;
 *   - DMA;
 *   - NUMA;
 *   - allocator implementation;
 *   - memory controllers;
 *   - synthesis;
 *   - placement;
 *   - routing;
 *   - timing closure;
 *   - target selection;
 *   - hardware discovery;
 *   - vendor APIs;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime allocation.
 *
 * ============================================================================
 * CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * This grammar is a parser component.
 *
 * It consumes the canonical lexer vocabulary:
 *
 *     ZamaniLexer
 *
 * It reuses parser rules supplied by the canonical HDL/parser composition:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     rangeExpression
 *     argumentList
 *     attribute
 *
 * No duplicate lexer or general-purpose expression/type grammar is created.
 *
 * ============================================================================
 * IMPORTANT TOKEN CORRECTION
 * ============================================================================
 *
 * The repository's canonical parser uses:
 *
 *     tokenVocab = ZamaniLexer
 *
 * Therefore this file MUST NOT use:
 *
 *     tokenVocab = ZamaniTokens
 *
 * `ZamaniTokens` is not the canonical parser-facing vocabulary.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A memory declaration expresses WHAT storage semantics are required.
 *
 * It does not encode:
 *
 *     - a particular FPGA;
 *     - a particular ASIC;
 *     - a particular SRAM;
 *     - a particular DRAM;
 *     - a particular HBM stack;
 *     - a particular bank;
 *     - a particular physical address;
 *     - a particular memory controller;
 *     - a particular board;
 *     - a particular vendor.
 *
 * The same source declaration may therefore be lowered differently on:
 *
 *     embedded hardware
 *     CPU systems
 *     multicore systems
 *     GPU systems
 *     FPGA systems
 *     ASIC systems
 *     accelerator systems
 *     quantum-classical systems
 *     HPC systems
 *     distributed systems
 *     cloud systems
 *     future architectures
 *
 * subject to the actual semantic requirements and available resources.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT encode universal limits such as:
 *
 *     MAX_MEMORY
 *     MAX_DEPTH
 *     MAX_WIDTH
 *     MAX_BANKS
 *     MAX_PORTS
 *     MAX_READ_PORTS
 *     MAX_WRITE_PORTS
 *     MAX_ENTRIES
 *     MAX_ADDRESS_BITS
 *     MAX_DIMENSIONS
 *     MAX_INSTANCES
 *
 * Nor may it encode assumptions such as:
 *
 *     memory<64GB>
 *     address<32>
 *     bank_count <= 8
 *     width <= 32
 *
 * unless such values occur explicitly as PROGRAM SEMANTICS.
 *
 * For example:
 *
 *     memory data: Word [DEPTH];
 *
 * is portable.
 *
 * Likewise:
 *
 *     memory data: Word [1024];
 *
 * is valid program data.
 *
 * But the language MUST NOT define:
 *
 *     MAX_DEPTH = 1024
 *
 * as a universal parser limitation.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * These concepts are intentionally distinct.
 *
 * REQUIREMENT:
 *     Must be satisfied.
 *
 * CONSTRAINT:
 *     Restricts the legal realization space.
 *
 * PREFERENCE:
 *     Non-binding optimization guidance.
 *
 * HINT:
 *     Non-binding implementation guidance.
 *
 * Capability requirements identify required capabilities rather than devices.
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * MEMORY MODEL
 * ============================================================================
 *
 * A logical HDL memory consists of:
 *
 *     identity
 *     optional generic arguments
 *     element/storage type
 *     one or more dimensions
 *     optional attributes
 *     optional properties
 *     optional interfaces
 *     optional initialization intent
 *     optional requirements
 *     optional constraints
 *     optional preferences
 *     optional hints
 *     optional capability requirements
 *
 * Dimensions are expressions.
 *
 * Dimensions may therefore be:
 *
 *     constants
 *     identifiers
 *     parameters
 *     generic parameters
 *     symbolic expressions
 *     derived expressions
 *     ranges
 *
 * The grammar does not evaluate them.
 *
 * ============================================================================
 * MEMORY PORT MODEL
 * ============================================================================
 *
 * A memory interface is LOGICAL.
 *
 * The source language describes:
 *
 *     direction
 *     logical name
 *     bindings
 *     address/data/control intent
 *     optional metadata
 *
 * It does not assert that a physical target has the same number of ports.
 *
 * Downstream lowering may:
 *
 *     preserve ports;
 *     merge ports;
 *     split ports;
 *     serialize accesses;
 *     replicate memory;
 *     partition memory;
 *     infer banking;
 *     introduce arbitration;
 *     introduce buffering;
 *     introduce caches;
 *     introduce controllers.
 *
 * Those are implementation decisions.
 *
 * ============================================================================
 * CONTEXTUAL MEMORY MEMBERS
 * ============================================================================
 *
 * The canonical lexer deliberately does not contain separate universal tokens
 * for every possible memory property such as:
 *
 *     read
 *     write
 *     init
 *     banking
 *     latency
 *     coherence
 *
 * Consequently this grammar does NOT invent duplicate lexer tokens.
 *
 * Contextual memory property names are represented by:
 *
 *     identifier
 *
 * and interpreted by semantic analysis.
 *
 * This keeps the language open-world while avoiding a second keyword registry.
 *
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 *
 *     hdlMemoryDeclaration
 *
 * is the only public memory-declaration integration boundary.
 *
 * The HDL composition root must delegate to this rule rather than maintain a
 * second implementation.
 *
 * ============================================================================
 */

parser grammar Memories;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. MEMORY DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     memory data: Word [DEPTH];
 *
 *     memory data: Word [ROWS][COLS];
 *
 *     memory data: Memory<Word> [DEPTH];
 *
 *     memory data<T, DEPTH>: T [DEPTH];
 *
 *     memory data: Word [DEPTH] {
 *         ...
 *     }
 *
 * The declaration is intentionally open with respect to:
 *
 *     dimensions
 *     members
 *     generic parameters
 *
 * ============================================================================
 */

hdlMemoryDeclaration
    : hdlMemoryHeader
      hdlMemoryBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. MEMORY HEADER
 * ============================================================================
 *
 * A header contains:
 *
 *     memory
 *     logical identity
 *     optional generic arguments
 *     element/storage type
 *     one or more dimensions
 *     optional declaration attributes
 *
 * The type remains delegated to the canonical type system.
 *
 * ============================================================================
 */

hdlMemoryHeader
    : K_MEMORY
      identifier
      hdlMemoryGenericArguments?
      COLON
      typeExpression
      hdlMemoryDimension+
      attribute*
    ;


/*
 * ============================================================================
 * 3. GENERIC MEMORY ARGUMENTS
 * ============================================================================
 *
 * Generic arguments are semantic parameters.
 *
 * There is no fixed parameter count.
 *
 * Examples:
 *
 *     memory tile<T, DEPTH>: T [DEPTH];
 *
 *     memory matrix<T, ROWS, COLS>: T [ROWS][COLS];
 *
 *     memory buffer<T, EXTENT = N>: T [EXTENT];
 *
 * The exact validity of a generic argument is semantic.
 *
 * ============================================================================
 */

hdlMemoryGenericArguments
    : LESS_THAN
      hdlMemoryGenericArgumentList
      GREATER_THAN
    ;


hdlMemoryGenericArgumentList
    : hdlMemoryGenericArgument
      (
          COMMA
          hdlMemoryGenericArgument
      )*
      COMMA?
    ;


hdlMemoryGenericArgument
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


/*
 * ============================================================================
 * 4. MEMORY DIMENSION
 * ============================================================================
 *
 * Every memory must declare at least one dimension.
 *
 * There is intentionally no fixed dimension count.
 *
 * Examples:
 *
 *     [DEPTH]
 *     [ROWS][COLS]
 *     [BATCH][ROWS][COLS]
 *
 * Range interpretation belongs to the canonical range-expression semantics.
 *
 * ============================================================================
 */

hdlMemoryDimension
    : LBRACKET
      rangeExpression
      RBRACKET
    ;


hdlMemoryDimensionList
    : hdlMemoryDimension+
    ;


/*
 * ============================================================================
 * 5. MEMORY BODY
 * ============================================================================
 *
 * The body is an unbounded sequence of memory-local members.
 *
 * No fixed number of:
 *
 *     ports
 *     properties
 *     accesses
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *
 * is imposed.
 *
 * ============================================================================
 */

hdlMemoryBody
    : LBRACE
      hdlMemoryMember*
      RBRACE
    ;


hdlMemoryMember
    : attribute*
      hdlMemoryMemberCore
    ;


hdlMemoryMemberCore
    : hdlMemoryPortDeclaration
    | hdlMemoryInitialization
    | hdlMemoryAccessDeclaration
    | hdlMemoryProperty
    | hdlMemoryRequirement
    | hdlMemoryConstraint
    | hdlMemoryPreference
    | hdlMemoryHint
    | hdlMemoryCapabilityRequirement
    | hdlMemoryNestedDeclaration
    ;


/*
 * ============================================================================
 * 6. LOGICAL MEMORY PORT
 * ============================================================================
 *
 * Port syntax is deliberately structural rather than tied to physical
 * memory-port technology.
 *
 * The canonical HDL direction tokens are reused:
 *
 *     K_INPUT
 *     K_OUTPUT
 *     K_INOUT
 *
 * A port therefore has the general form:
 *
 *     input  read_port  : AddressType;
 *     output write_port : DataType;
 *     inout  shared_port : SomeType;
 *
 * Optional bindings/properties remain expressions and metadata.
 *
 * The semantic layer determines whether a port is:
 *
 *     read
 *     write
 *     read/write
 *     address
 *     data
 *     enable
 *     mask
 *     control
 *     streaming
 *     atomic
 *     etc.
 *
 * ============================================================================
 */

hdlMemoryPortDeclaration
    : hdlMemoryPortDirection
      identifier
      (
          COLON
          typeExpression
      )?
      hdlMemoryPortRange*
      hdlMemoryPortClause*
      SEMICOLON
    ;


hdlMemoryPortDirection
    : K_INPUT
    | K_OUTPUT
    | K_INOUT
    ;


hdlMemoryPortRange
    : LBRACKET
      rangeExpression
      RBRACKET
    ;


hdlMemoryPortClause
    : attribute
    | hdlMemoryNamedBinding
    | hdlMemoryPropertyClause
    ;


hdlMemoryNamedBinding
    : identifier
      ASSIGN
      expression
    ;


hdlMemoryPropertyClause
    : identifier
      (
          ASSIGN
          expression
        | LPAREN
          argumentList?
          RPAREN
      )?
    ;


/*
 * ============================================================================
 * 7. INITIALIZATION
 * ============================================================================
 *
 * Initialization expresses source-level initialization intent.
 *
 * It does not select:
 *
 *     ROM
 *     BRAM initialization
 *     boot image format
 *     flash
 *     physical address
 *     vendor mechanism
 *
 * Canonical forms:
 *
 *     init = expression;
 *
 *     init expression;
 *
 * The contextual identifier "init" remains semantic data rather than a new
 * universal lexer token.
 *
 * ============================================================================
 */

hdlMemoryInitialization
    : identifier
      (
          ASSIGN
          expression
        | expression
      )
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. ACCESS DECLARATION
 * ============================================================================
 *
 * Generic access declarations remain open-world.
 *
 * Example:
 *
 *     access(read, address = addr, data = value);
 *
 *     access(write, address = addr, data = value);
 *
 *     access(readwrite, address = addr, data = value);
 *
 * The first expression identifies the logical operation.
 *
 * Semantic analysis determines whether the operation is legal.
 *
 * No finite operation enumeration is placed in the grammar.
 *
 * ============================================================================
 */

hdlMemoryAccessDeclaration
    : identifier
      LPAREN
      argumentList?
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. PROPERTY
 * ============================================================================
 *
 * A memory property is a named declaration-level semantic property.
 *
 * Examples:
 *
 *     latency = L;
 *
 *     throughput = T;
 *
 *     banking = banking_policy;
 *
 *     layout = layout_policy;
 *
 *     coherence = coherence_policy;
 *
 *     persistence = persistence_policy;
 *
 *     address_space = logical_space;
 *
 * The property name remains an identifier.
 *
 * The semantic registry determines:
 *
 *     - whether the property exists;
 *     - its expected value type;
 *     - its effect;
 *     - compatibility;
 *     - lowering behavior.
 *
 * This allows future memory technologies without modifying this grammar for
 * every new property.
 *
 * ============================================================================
 */

hdlMemoryProperty
    : identifier
      (
          ASSIGN
          expression
        | LPAREN
          argumentList?
          RPAREN
      )
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. REQUIREMENT
 * ============================================================================
 *
 * Requirements are mandatory semantic conditions.
 *
 * Examples:
 *
 *     requires capacity >= required_capacity;
 *
 *     requires latency <= required_latency;
 *
 *     requires capability("memory.streaming");
 *
 * The parser records structure only.
 *
 * Resource satisfaction belongs downstream.
 *
 * ============================================================================
 */

hdlMemoryRequirement
    : REQUIRES
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. CONSTRAINT
 * ============================================================================
 *
 * Constraints restrict the legal realization space.
 *
 * They do not identify a physical target.
 *
 * ============================================================================
 */

hdlMemoryConstraint
    : CONSTRAINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. PREFERENCE
 * ============================================================================
 *
 * Preferences are non-binding implementation guidance.
 *
 * A compiler may disregard a preference when necessary while preserving
 * program semantics.
 *
 * ============================================================================
 */

hdlMemoryPreference
    : PREFER
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. HINT
 * ============================================================================
 *
 * Hints are explicitly non-binding.
 *
 * ============================================================================
 */

hdlMemoryHint
    : HINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * Capability requirements describe semantic capabilities.
 *
 * Examples:
 *
 *     requires capability("memory.streaming");
 *
 *     requires capability("memory.atomic");
 *
 *     requires capability("memory.coherent");
 *
 * No device identifier is introduced.
 *
 * ============================================================================
 */

hdlMemoryCapabilityRequirement
    : REQUIRES
      hdlMemoryCapabilityExpression
      SEMICOLON
    ;


hdlMemoryCapabilityExpression
    : expression
    ;


/*
 * ============================================================================
 * 15. NESTED MEMORY DECLARATION
 * ============================================================================
 *
 * Nested logical memory declarations allow hierarchical hardware descriptions.
 *
 * This does not imply a physical nested memory technology.
 *
 * ============================================================================
 */

hdlMemoryNestedDeclaration
    : hdlMemoryDeclaration
    ;


/*
 * ============================================================================
 * 16. PORT DECLARATION LIST
 * ============================================================================
 *
 * Helper for consumers that need a complete logical memory-port list.
 *
 * ============================================================================
 */

hdlMemoryPortDeclarationList
    : hdlMemoryPortDeclaration+
    ;


/*
 * ============================================================================
 * 17. PROPERTY LIST
 * ============================================================================
 */

hdlMemoryPropertyList
    : hdlMemoryProperty+
    ;


/*
 * ============================================================================
 * 18. REQUIREMENT LIST
 * ============================================================================
 */

hdlMemoryRequirementList
    : hdlMemoryRequirement+
    ;


/*
 * ============================================================================
 * 19. CONSTRAINT LIST
 * ============================================================================
 */

hdlMemoryConstraintList
    : hdlMemoryConstraint+
    ;


/*
 * ============================================================================
 * 20. PREFERENCE LIST
 * ============================================================================
 */

hdlMemoryPreferenceList
    : hdlMemoryPreference+
    ;


/*
 * ============================================================================
 * 21. HINT LIST
 * ============================================================================
 */

hdlMemoryHintList
    : hdlMemoryHint+
    ;


/*
 * ============================================================================
 * 22. CAPABILITY REQUIREMENT LIST
 * ============================================================================
 */

hdlMemoryCapabilityRequirementList
    : hdlMemoryCapabilityRequirement+
    ;


/*
 * ============================================================================
 * 23. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing MUST NOT determine:
 *
 *     - whether dimensions are numerically valid;
 *     - whether dimensions fit available hardware;
 *     - whether a type is synthesizable;
 *     - whether a port can be physically implemented;
 *     - whether simultaneous accesses are legal;
 *     - whether banking is realizable;
 *     - whether timing requirements are satisfiable;
 *     - whether memory capacity exists;
 *     - whether a capability exists;
 *     - whether a target exists.
 *
 * Those questions belong to semantic analysis and downstream compilation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. AST CONTRACT
 * ============================================================================
 *
 * Every hdlMemoryDeclaration must preserve enough information for downstream
 * semantic analysis.
 *
 * At minimum:
 *
 *     source span
 *     memory name
 *     generic arguments
 *     storage type
 *     dimensions
 *     attributes
 *     ports
 *     initialization
 *     accesses
 *     properties
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     capability requirements
 *     nested declarations
 *
 * No physical realization may be inferred solely from parsing.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. IR CONTRACT
 * ============================================================================
 *
 * This grammar does not construct an IR.
 *
 * Semantic lowering must map the AST into the repository's canonical
 * hardware/HDL semantic representation.
 *
 * It MUST NOT create:
 *
 *     - a second memory IR;
 *     - a second hardware IR;
 *     - a quantum IR;
 *     - a physical placement graph;
 *     - a routing graph;
 *     - a scheduling representation.
 *
 * If the memory participates in quantum-classical computation, the quantum
 * semantic path remains:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * This grammar never constructs quantum::ir.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. RESOURCE CONTRACT
 * ============================================================================
 *
 * Memory resource analysis must distinguish:
 *
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *
 * Examples:
 *
 *     requires capacity >= required_capacity;
 *
 *     requires capability("memory.streaming");
 *
 *     constraint latency <= target_latency;
 *
 *     prefer memory_policy;
 *
 *     hint implementation_policy;
 *
 * Resource availability is determined downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar is open with respect to:
 *
 *     memory declarations
 *     dimensions
 *     ports
 *     properties
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     capability requirements
 *     generic arguments
 *     nested memories
 *
 * No parser-level numerical ceiling is imposed.
 *
 * ANTLR repetition constructs are used instead of fixed cardinalities.
 *
 * The compiler may still enforce operational safeguards for:
 *
 *     source size
 *     parser memory
 *     parser execution time
 *     generated design size
 *     semantic analysis cost
 *     synthesis cost
 *
 * Such safeguards are implementation policies and MUST NOT become language
 * semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions
 *     no semantic predicates
 *     no external state
 *     no filesystem access
 *     no network access
 *     no hardware discovery
 *     no randomness
 *     no runtime execution
 *
 * Therefore parsing depends only on:
 *
 *     source token stream
 *     grammar version
 *     parser configuration
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * The consuming Zamani implementation remains required to use:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * No unsafe Rust is required by this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The existing canonical declaration:
 *
 *     memory name: type [dimension];
 *
 * remains supported.
 *
 * Existing HDL composition continues to enter through:
 *
 *     hdlMemoryDeclaration
 *
 * The HDL composition root must therefore delegate that rule to this component
 * and MUST NOT maintain a competing implementation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. REQUIRED HDL INTEGRATION
 * ============================================================================
 *
 * grammar/hdl/hdl.g4
 * ------------------
 *
 * Its member dispatcher currently exposes:
 *
 *     hdlMemoryDeclaration
 *
 * That remains the stable integration name.
 *
 * The implementation must be composed from this grammar component.
 *
 * There MUST NOT be two independent implementations of:
 *
 *     hdlMemoryDeclaration
 *
 * within the same generated parser.
 *
 *
 * grammar/hdl/hardware-modules.g4
 * -------------------------------
 *
 * Module composition may continue to consume:
 *
 *     hdlMemoryDeclaration
 *
 * without introducing another memory grammar.
 *
 *
 * grammar/types/*
 * ---------------
 *
 * This file delegates storage/element type syntax to:
 *
 *     typeExpression
 *
 * No hardware-specific second type system is created here.
 *
 *
 * grammar/expressions/*
 * --------------------
 *
 * This file delegates values and symbolic dimensions to:
 *
 *     expression
 *     rangeExpression
 *     argumentList
 *
 * No expression precedence is duplicated here.
 *
 *
 * grammar/memory/*
 * ----------------
 *
 * General memory semantics remain owned by the generic memory subsystem.
 *
 * This file represents HDL-specific structure and interface intent.
 *
 *
 * grammar/resources/*
 * -------------------
 *
 * Requirements and capabilities must ultimately map into the universal
 * resource/capability model.
 *
 * This file does not create a second resource system.
 *
 *
 * grammar/hardware/*
 * ------------------
 *
 * Physical placement, topology, device identity, banking technology,
 * memory-controller selection and target realization remain downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. SOURCE-LEVEL EXAMPLES
 * ============================================================================
 *
 * Simple logical memory:
 *
 *     memory data: Word [DEPTH];
 *
 *
 * Symbolically sized memory:
 *
 *     memory data: Word [DEPTH];
 *
 *
 * Multidimensional memory:
 *
 *     memory matrix: Word [ROWS][COLS];
 *
 *
 * Generic memory:
 *
 *     memory tile<T, DEPTH>: T [DEPTH];
 *
 *
 * Logical ports:
 *
 *     memory data: Word [DEPTH] {
 *         input address: Address;
 *         output value: Word;
 *     };
 *
 *
 * Requirement:
 *
 *     memory data: Word [DEPTH] {
 *         requires capacity >= required_capacity;
 *     };
 *
 *
 * Capability:
 *
 *     memory data: Word [DEPTH] {
 *         requires capability("memory.streaming");
 *     };
 *
 *
 * Constraint:
 *
 *     memory data: Word [DEPTH] {
 *         constraint latency <= required_latency;
 *     };
 *
 *
 * Preference:
 *
 *     memory data: Word [DEPTH] {
 *         prefer memory_policy;
 *     };
 *
 *
 * Hint:
 *
 *     memory data: Word [DEPTH] {
 *         hint implementation_policy;
 *     };
 *
 *
 * Access intent:
 *
 *     memory data: Word [DEPTH] {
 *         access(read, address = addr, data = value);
 *     };
 *
 *
 * Initialization:
 *
 *     memory data: Word [DEPTH] {
 *         init = initialization_expression;
 *     };
 *
 *
 * None of these examples selects a physical memory technology.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. HARD-CODING AUDIT
 * ============================================================================
 *
 * PROHIBITED UNIVERSAL ASSUMPTIONS:
 *
 *     [x] No MAX_MEMORY
 *     [x] No MAX_DEPTH
 *     [x] No MAX_WIDTH
 *     [x] No MAX_BANKS
 *     [x] No MAX_PORTS
 *     [x] No MAX_READ_PORTS
 *     [x] No MAX_WRITE_PORTS
 *     [x] No MAX_ENTRIES
 *     [x] No MAX_ADDRESS_BITS
 *     [x] No MAX_DIMENSIONS
 *     [x] No MAX_INSTANCES
 *     [x] No fixed physical memory technology
 *     [x] No fixed physical address
 *     [x] No fixed bank identifier
 *     [x] No fixed device identifier
 *     [x] No fixed vendor
 *     [x] No fixed topology
 *     [x] No fixed FPGA resource count
 *     [x] No fixed ASIC resource count
 *
 * Program-supplied numeric values remain valid program semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] The existing filename is retained.
 *
 *     [x] The canonical parser-facing vocabulary is ZamaniLexer.
 *
 *     [x] No second lexer is introduced.
 *
 *     [x] No duplicate expression grammar is introduced.
 *
 *     [x] No duplicate type grammar is introduced.
 *
 *     [x] No duplicate identifier grammar is introduced.
 *
 *     [x] Symbolic dimensions are supported.
 *
 *     [x] Arbitrary numbers of dimensions are supported.
 *
 *     [x] Arbitrary numbers of logical ports are supported.
 *
 *     [x] Arbitrary numbers of memory properties are supported.
 *
 *     [x] Requirements are distinct from constraints.
 *
 *     [x] Preferences are distinct from requirements.
 *
 *     [x] Hints are distinct from preferences.
 *
 *     [x] Capability requirements remain target-independent.
 *
 *     [x] Initialization is source-level intent.
 *
 *     [x] Accesses remain open-world.
 *
 *     [x] No physical address is required.
 *
 *     [x] No physical bank is required.
 *
 *     [x] No physical memory technology is selected.
 *
 *     [x] No resource ceiling is encoded.
 *
 *     [x] No quantum IR is introduced.
 *
 *     [x] No QEC implementation is introduced.
 *
 *     [x] No ZQN implementation is introduced.
 *
 *     [x] No routing is introduced.
 *
 *     [x] No scheduling is introduced.
 *
 *     [x] No hardware discovery is introduced.
 *
 *     [x] No Rust actions are introduced.
 *
 *     [x] No unsafe Rust is required.
 *
 *     [x] Existing hdlMemoryDeclaration remains the integration boundary.
 *
 *     [x] Downstream AST/semantic/IR ownership is predetermined.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. FINAL INVARIANT
 * ============================================================================
 *
 * This grammar defines:
 *
 *     WHAT an HDL memory declaration means structurally.
 *
 * It does NOT define:
 *
 *     HOW that memory is physically realized.
 *
 * Consequently a Zamani program can describe portable memory intent once and
 * allow later compilation/lowering to determine an appropriate realization
 * from the available resources and target capabilities.
 *
 * ============================================================================
 */