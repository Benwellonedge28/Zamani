/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/memories.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Status:
 *     Production HDL memory-declaration grammar.
 *
 * Language role:
 *     Defines SOURCE-LEVEL HDL MEMORY STRUCTURE and MEMORY INTERFACE INTENT.
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe Rust.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniTokens
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     HDL parser
 *          |
 *          v
 *     hdlMemoryDeclaration
 *          |
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *     frontend AST                 semantic analysis
 *                                          |
 *                                          +--> type analysis
 *                                          +--> resource analysis
 *                                          +--> capability analysis
 *                                          +--> memory legality
 *                                          +--> timing analysis
 *                                          +--> ownership/alias analysis
 *                                          +--> target-independent HDL model
 *                                          |
 *                                          v
 *                               canonical HDL/hardware IR
 *                                          |
 *                     +--------------------+--------------------+
 *                     |                    |                    |
 *                     v                    v                    v
 *                 optimization        scheduling            routing
 *                     |                    |                    |
 *                     +--------------------+--------------------+
 *                                          |
 *                                          v
 *                                   target lowering
 *                                          |
 *                     +--------------------+--------------------+
 *                     |                    |                    |
 *                     v                    v                    v
 *                   FPGA                 ASIC             other targets
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - HDL memory declarations;
 *     - logical memory identity;
 *     - memory element type reference;
 *     - symbolic memory dimensions;
 *     - memory declaration dimensions;
 *     - source-level memory interface declarations;
 *     - logical read/write access descriptions;
 *     - memory initialization intent;
 *     - memory access-property syntax;
 *     - memory layout-property syntax;
 *     - memory banking/partitioning intent syntax;
 *     - memory latency intent syntax;
 *     - memory persistence/coherence intent syntax;
 *     - memory resource/capability requirement syntax;
 *     - memory declaration attributes;
 *     - memory-local structural metadata.
 *
 * ============================================================================
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - general memory semantics;
 *     - general allocation;
 *     - ownership;
 *     - borrowing;
 *     - lifetimes;
 *     - virtual memory;
 *     - physical addresses;
 *     - cache implementation;
 *     - NUMA discovery;
 *     - DMA implementation;
 *     - allocator implementation;
 *     - garbage collection;
 *     - memory management runtime;
 *     - physical memory banks;
 *     - fixed BRAM count;
 *     - fixed SRAM count;
 *     - fixed DRAM count;
 *     - fixed memory capacity;
 *     - fixed address width;
 *     - physical placement;
 *     - physical routing;
 *     - synthesis;
 *     - timing closure;
 *     - target selection;
 *     - hardware discovery;
 *     - vendor APIs;
 *     - runtime allocation;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - resilience.
 *
 * Generic memory semantics remain owned by:
 *
 *     grammar/memory/memory.g4
 *
 * Specialized software-memory semantics remain owned by:
 *
 *     grammar/memory/ownership.g4
 *     grammar/memory/borrowing.g4
 *     grammar/memory/lifetimes.g4
 *     grammar/memory/allocation.g4
 *     grammar/memory/deallocation.g4
 *     grammar/memory/shared-memory.g4
 *     grammar/memory/distributed-memory.g4
 *     grammar/memory/memory-constraints.g4
 *
 * ============================================================================
 *
 * POCO-REAF PRINCIPLE
 * ============================================================================
 *
 * An HDL memory declaration describes WHAT memory structure and interface
 * semantics are required.
 *
 * It does not select:
 *
 *     - a particular FPGA;
 *     - a particular ASIC;
 *     - a particular SRAM;
 *     - a particular DRAM;
 *     - a particular BRAM;
 *     - a particular HBM stack;
 *     - a particular memory controller;
 *     - a particular physical address;
 *     - a particular bank number;
 *     - a particular vendor;
 *     - a particular board;
 *     - a particular machine.
 *
 * Therefore this grammar contains no:
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
 *     MAX_INSTANCES
 *
 * and no equivalent hidden parser limitation.
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * Symbolic dimensions and expressions are accepted without imposing a
 * language-level numerical maximum.
 *
 * Physical realizability is determined later by:
 *
 *     semantic analysis;
 *     resource analysis;
 *     capability analysis;
 *     target analysis;
 *     synthesis;
 *     scheduling;
 *     placement;
 *     routing;
 *     deployment;
 *     runtime.
 *
 * ============================================================================
 *
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file is a PARSER grammar.
 *
 * It consumes:
 *
 *     grammar/...
 *
 * through the canonical ZamaniTokens vocabulary.
 *
 * The HDL parser supplies/reuses:
 *
 *     identifier
 *     hdlKeyword
 *     hdlExpression
 *     hdlTypeExpression
 *     hdlRangeExpression
 *     hdlAttribute
 *     hdlArgumentList
 *     hdlConnectionList
 *
 * where those rules are available through the composed parser.
 *
 * IMPORTANT:
 *
 * This file MUST NOT create a second lexer.
 *
 * Existing canonical token:
 *
 *     K_MEMORY
 *
 * is used for the memory declaration boundary.
 *
 * This is important because the current hardware type grammar already treats
 * K_MEMORY as the canonical hardware-memory type constructor.
 *
 * ============================================================================
 *
 * INTEGRATION WITH EXISTING HDL GRAMMAR
 * ============================================================================
 *
 * grammar/hdl/hdl.g4 currently exposes:
 *
 *     hdlMemoryDeclaration
 *
 * as an HDL module member.
 *
 * That rule MUST be delegated to this grammar component.
 *
 * The old generic implementation:
 *
 *     hdlKeyword identifier COLON hdlTypeExpression
 *     hdlMemoryDimension+
 *     SEMICOLON
 *
 * is intentionally superseded by the more precise K_MEMORY-based rule here.
 *
 * ============================================================================
 *
 * INTEGRATION WITH HARDWARE MODULES
 * ============================================================================
 *
 * grammar/hdl/hardware-modules.g4 already composes:
 *
 *     hdlMemoryDeclaration
 *
 * as a module member.
 *
 * This file therefore provides the canonical implementation of that rule.
 *
 * ============================================================================
 *
 * INTEGRATION WITH HARDWARE TYPES
 * ============================================================================
 *
 * grammar/types/hardware-types.g4 owns hardware memory TYPE syntax:
 *
 *     K_MEMORY typeArgumentList
 *
 * This file does NOT redefine that type.
 *
 * A declaration such as:
 *
 *     memory storage: Memory<Word> [DEPTH];
 *
 * uses the type system for the element/storage semantic type while this file
 * owns the structural memory declaration.
 *
 * ============================================================================
 *
 * INTEGRATION WITH GENERAL MEMORY
 * ============================================================================
 *
 * grammar/memory/memory.g4 remains the canonical generic memory-domain
 * foundation.
 *
 * This file does not redefine:
 *
 *     memoryPlace
 *     memorySpace
 *     memoryLifetime
 *     memoryQualifiedName
 *     memoryOperation
 *     memoryOperationArgument
 *
 * HDL memory syntax describes hardware structure and interface intent.
 *
 * General memory semantics are resolved by semantic analysis against the
 * canonical memory model.
 *
 * ============================================================================
 *
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no Rust actions;
 *     - no Rust predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no process execution;
 *     - no mutable global state;
 *     - no unsafe code;
 *     - no target-specific code;
 *     - no hardware discovery.
 *
 * ============================================================================
 */

parser grammar Memories;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The canonical HDL parser entry point for a memory declaration.
 *
 * Examples:
 *
 *     memory data: Word [DEPTH];
 *
 *     memory data: Word [ROWS][COLS];
 *
 *     memory data: Word [ROWS][COLS]
 *         read ...
 *         write ...;
 *
 *     memory data: Memory<Word> [DEPTH];
 *
 * Dimensions are symbolic expressions and therefore do not establish a fixed
 * machine capacity.
 */
hdlMemoryDeclaration
    : hdlMemoryHeader
      hdlMemoryBody?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 2. MEMORY HEADER
 * ============================================================================
 *
 * The header establishes logical memory identity, element/storage type and
 * dimensions.
 *
 * The declaration is deliberately target-independent.
 */
hdlMemoryHeader
    : K_MEMORY
      identifier
      hdlMemoryGenericArguments?
      COLON
      hdlTypeExpression
      hdlMemoryDimension+
      hdlMemoryModifier*
    ;


/*
 * ============================================================================
 * 3. MEMORY GENERIC SPECIALIZATION
 * ============================================================================
 *
 * Generic parameters are structural abstractions.
 *
 * No finite number of parameters is imposed.
 *
 * Example:
 *
 *     memory tile<T, DEPTH>: T [DEPTH];
 *
 * Concrete realizability is a semantic/compiler concern.
 */
hdlMemoryGenericArguments
    : LT
      hdlMemoryGenericArgumentList
      GT
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
    | identifier ASSIGN hdlExpression
    | hdlExpression
    ;


/*
 * ============================================================================
 * 4. MEMORY DIMENSIONS
 * ============================================================================
 *
 * A memory may have one or more dimensions.
 *
 * There is intentionally no maximum dimension count.
 *
 * Examples:
 *
 *     [DEPTH]
 *     [ROWS][COLS]
 *     [BATCH][ROWS][COLS]
 *     [symbolic_extent]
 *     [lower .. upper]
 *
 * The meaning of the range is determined semantically.
 */
hdlMemoryDimension
    : LBRACKET
      hdlRangeExpression
      RBRACKET
    ;


/*
 * ============================================================================
 * 5. MEMORY MODIFIERS
 * ============================================================================
 *
 * Modifiers provide declaration-level semantic intent.
 *
 * The grammar deliberately keeps modifiers extensible.
 *
 * The semantic layer determines which modifier names are valid.
 *
 * This prevents every new memory technology from requiring a grammar rewrite.
 */
hdlMemoryModifier
    : hdlMemoryProperty
    | hdlAttribute
    ;


/*
 * ============================================================================
 * 6. MEMORY BODY
 * ============================================================================
 *
 * A memory body contains optional structural/interface declarations.
 *
 * There is no fixed number of ports, declarations or properties.
 */
hdlMemoryBody
    : LBRACE
      hdlMemoryMember*
      RBRACE
    ;


hdlMemoryMember
    : hdlMemoryAttribute
    | hdlMemoryProperty
    | hdlMemoryReadPort
    | hdlMemoryWritePort
    | hdlMemoryReadWritePort
    | hdlMemoryInit
    | hdlMemoryAccess
    | hdlMemoryRequirement
    | hdlMemoryConstraint
    | hdlMemoryPreference
    | hdlMemoryCapabilityRequirement
    | hdlMemoryDeclaration
    ;


/*
 * ============================================================================
 * 7. MEMORY ATTRIBUTES
 * ============================================================================
 *
 * Attributes remain generic metadata.
 *
 * They must not become hidden physical-target selectors.
 */
hdlMemoryAttribute
    : hdlAttribute
    ;


/*
 * ============================================================================
 * 8. GENERIC MEMORY PROPERTY
 * ============================================================================
 *
 * Property syntax is intentionally extensible.
 *
 * Examples:
 *
 *     latency = expr;
 *     banking = expr;
 *     layout = expr;
 *     coherence = expr;
 *     persistence = expr;
 *
 * Semantic validation determines the vocabulary and meaning.
 */
hdlMemoryProperty
    : hdlKeyword
      (
          ASSIGN
          hdlExpression
        | LPAREN
          hdlArgumentList?
          RPAREN
        | hdlExpression
      )?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. READ PORT
 * ============================================================================
 *
 * A read port is a LOGICAL interface.
 *
 * It does not imply a physical memory port.
 *
 * Example:
 *
 *     read r:
 *         address = addr,
 *         data = value;
 *
 * The exact semantic names are validated downstream.
 */
hdlMemoryReadPort
    : hdlMemoryAccessKeyword
      hdlMemoryPortName?
      COLON
      hdlMemoryPortBindingList?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. WRITE PORT
 * ============================================================================
 */
hdlMemoryWritePort
    : hdlMemoryAccessKeyword
      hdlMemoryPortName?
      COLON
      hdlMemoryPortBindingList?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. READ/WRITE PORT
 * ============================================================================
 *
 * A bidirectional logical interface.
 */
hdlMemoryReadWritePort
    : hdlMemoryAccessKeyword
      hdlMemoryPortName?
      COLON
      hdlMemoryPortBindingList?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. ACCESS KEYWORD
 * ============================================================================
 *
 * Access-classification words remain contextual HDL vocabulary.
 *
 * The semantic layer distinguishes:
 *
 *     read
 *     write
 *     readwrite
 *     atomic
 *     streaming
 *     etc.
 *
 * This rule intentionally accepts the canonical HDL keyword abstraction rather
 * than introducing duplicate lexer tokens.
 */
hdlMemoryAccessKeyword
    : hdlKeyword
    ;


hdlMemoryPortName
    : identifier
    ;


hdlMemoryPortBindingList
    : hdlMemoryPortBinding
      (
          COMMA
          hdlMemoryPortBinding
      )*
      COMMA?
    ;


hdlMemoryPortBinding
    : identifier
      ASSIGN
      hdlExpression
    ;


/*
 * ============================================================================
 * 13. MEMORY INITIALIZATION
 * ============================================================================
 *
 * Initialization is SOURCE-LEVEL INITIALIZATION INTENT.
 *
 * It does not determine:
 *
 *     ROM technology;
 *     FPGA initialization mechanism;
 *     boot-loader format;
 *     physical image placement;
 *     physical memory address.
 *
 * Example:
 *
 *     init values = expression;
 *
 *     init expression;
 */
hdlMemoryInit
    : hdlMemoryInitKeyword
      (
          ASSIGN
          hdlExpression
        | hdlExpression
      )
      SEMICOLON
    ;


hdlMemoryInitKeyword
    : hdlKeyword
    ;


/*
 * ============================================================================
 * 14. MEMORY ACCESS
 * ============================================================================
 *
 * A memory access declaration describes logical access behavior.
 *
 * It is not a machine instruction and does not prescribe a specific controller.
 */
hdlMemoryAccess
    : hdlMemoryAccessKeyword
      LPAREN
      hdlMemoryAccessArgumentList?
      RPAREN
      SEMICOLON
    ;


hdlMemoryAccessArgumentList
    : hdlMemoryAccessArgument
      (
          COMMA
          hdlMemoryAccessArgument
      )*
      COMMA?
    ;


hdlMemoryAccessArgument
    : identifier
      ASSIGN
      hdlExpression
    | hdlExpression
    ;


/*
 * ============================================================================
 * 15. MEMORY REQUIREMENTS
 * ============================================================================
 *
 * Requirements express properties the implementation must satisfy.
 *
 * They do NOT select a device.
 *
 * Example:
 *
 *     requires property;
 *
 * The semantic resource system determines whether a target satisfies it.
 */
hdlMemoryRequirement
    : REQUIRES
      hdlExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. MEMORY CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict valid realizations without becoming physical-device
 * selection syntax.
 */
hdlMemoryConstraint
    : WHERE
      hdlExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. MEMORY PREFERENCES
 * ============================================================================
 *
 * Preferences are non-mandatory implementation guidance.
 *
 * They must never silently become semantic requirements.
 */
hdlMemoryPreference
    : hdlMemoryPreferenceKeyword
      hdlExpression
      SEMICOLON
    ;


hdlMemoryPreferenceKeyword
    : hdlKeyword
    ;


/*
 * ============================================================================
 * 18. MEMORY CAPABILITY REQUIREMENTS
 * ============================================================================
 *
 * Capability requirements express semantic capabilities.
 *
 * They do not identify a particular physical device.
 */
hdlMemoryCapabilityRequirement
    : hdlMemoryCapabilityKeyword
      hdlExpression
      SEMICOLON
    ;


hdlMemoryCapabilityKeyword
    : hdlKeyword
    ;


/*
 * ============================================================================
 * 19. MEMORY PROPERTY EXPRESSIONS
 * ============================================================================
 *
 * These rules are intentionally structural.
 *
 * The parser accepts symbolic values such as:
 *
 *     DEPTH
 *     WIDTH
 *     Rows * Columns
 *     parameterized expressions
 *     generic expressions
 *
 * No hard-coded integer maximum is present.
 */


/*
 * ============================================================================
 * 20. OPTIONAL MEMORY DECLARATION LIST
 * ============================================================================
 *
 * This helper allows future composed HDL grammars to consume multiple memory
 * declarations without introducing another memory root.
 */
hdlMemoryDeclarationList
    : hdlMemoryDeclaration+
    ;


/*
 * ============================================================================
 * 21. MEMORY PORT DECLARATION LIST
 * ============================================================================
 *
 * No finite number of ports is imposed.
 */
hdlMemoryPortDeclarationList
    : (
          hdlMemoryReadPort
        | hdlMemoryWritePort
        | hdlMemoryReadWritePort
      )+
    ;


/*
 * ============================================================================
 * 22. MEMORY DIMENSION LIST
 * ============================================================================
 *
 * Explicit helper for consumers that need dimensions independently.
 */
hdlMemoryDimensionList
    : hdlMemoryDimension+
    ;


/*
 * ============================================================================
 * 23. MEMORY PROPERTY LIST
 * ============================================================================
 */
hdlMemoryPropertyList
    : hdlMemoryProperty+
    ;


/*
 * ============================================================================
 * 24. MEMORY REQUIREMENT LIST
 * ============================================================================
 */
hdlMemoryRequirementList
    : hdlMemoryRequirement+
    ;


/*
 * ============================================================================
 * 25. MEMORY CONSTRAINT LIST
 * ============================================================================
 */
hdlMemoryConstraintList
    : hdlMemoryConstraint+
    ;


/*
 * ============================================================================
 * 26. MEMORY PREFERENCE LIST
 * ============================================================================
 */
hdlMemoryPreferenceList
    : hdlMemoryPreference+
    ;


/*
 * ============================================================================
 * 27. MEMORY CAPABILITY LIST
 * ============================================================================
 */
hdlMemoryCapabilityRequirementList
    : hdlMemoryCapabilityRequirement+
    ;


/*
 * ============================================================================
 * 28. COMPOSITION CONTRACT
 * ============================================================================
 *
 * The following names are deliberately NOT redefined in this file:
 *
 *     identifier
 *     hdlKeyword
 *     hdlExpression
 *     hdlTypeExpression
 *     hdlRangeExpression
 *     hdlAttribute
 *     hdlArgumentList
 *
 * They are supplied by the composed HDL/core/type/expression grammars.
 *
 * This prevents:
 *
 *     duplicate identifier grammars;
 *     duplicate expression grammars;
 *     duplicate type grammars;
 *     duplicate attribute grammars;
 *     duplicate lexer definitions.
 *
 * ============================================================================
 *
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The parser only establishes syntactic structure.
 *
 * Semantic analysis must subsequently determine:
 *
 *     - whether the memory type is legal;
 *     - whether every dimension is valid;
 *     - whether dimensions are compatible;
 *     - whether symbolic dimensions can be resolved;
 *     - whether read/write interfaces are legal;
 *     - whether access modes are compatible;
 *     - whether initialization is legal;
 *     - whether latency constraints are satisfiable;
 *     - whether banking/partitioning intent is realizable;
 *     - whether coherence requirements are valid;
 *     - whether persistence requirements are valid;
 *     - whether resource requirements are satisfiable;
 *     - whether capabilities are available;
 *     - whether constraints can be met;
 *     - whether preferences can be honored;
 *     - whether the declaration can lower to canonical HDL/hardware IR.
 *
 * The parser MUST NOT answer those questions.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve at least:
 *
 *     - source span;
 *     - memory name;
 *     - generic arguments;
 *     - element/storage type;
 *     - dimensions;
 *     - modifiers;
 *     - attributes;
 *     - memory properties;
 *     - read ports;
 *     - write ports;
 *     - read/write ports;
 *     - initialization expressions;
 *     - access expressions;
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - capability requirements.
 *
 * No physical device information should be inferred merely from syntax.
 *
 * ============================================================================
 *
 * IR CONTRACT
 * ============================================================================
 *
 * Lowering must convert the AST into the repository's canonical HDL/hardware
 * semantic representation.
 *
 * This grammar must NEVER construct:
 *
 *     quantum::ir
 *     hardware placement
 *     physical memory addresses
 *     physical routing
 *     scheduling decisions
 *     target-specific allocation.
 *
 * If a memory is associated with quantum computation, the semantic/lowering
 * layer determines whether the operation participates in quantum semantics.
 *
 * quantum::ir remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 *
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar is open-world with respect to:
 *
 *     dimensions;
 *     ports;
 *     memory declarations;
 *     properties;
 *     requirements;
 *     constraints;
 *     capabilities;
 *     generic parameters;
 *     symbolic expressions;
 *     module instances.
 *
 * No parser-level numerical ceiling is encoded.
 *
 * A practical parser implementation may have external resource policies for:
 *
 *     memory;
 *     CPU time;
 *     recursion;
 *     token count;
 *     source size.
 *
 * Such policies are implementation/runtime safety controls and MUST NOT be
 * represented as source-language semantic limits.
 *
 * ============================================================================
 *
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     fixed memory capacity;
 *     fixed address width;
 *     fixed bank count;
 *     fixed port count;
 *     fixed dimension count;
 *     fixed element count;
 *     fixed device count;
 *     fixed memory technology;
 *     fixed FPGA resource count;
 *     fixed ASIC resource count;
 *     fixed topology;
 *     fixed physical address;
 *     fixed device ID;
 *     fixed vendor.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * The grammar contains:
 *
 *     - no semantic predicates;
 *     - no actions;
 *     - no external state;
 *     - no environment reads;
 *     - no hardware discovery.
 *
 * Therefore parsing is deterministic with respect to the token stream and
 * parser configuration.
 *
 * ============================================================================
 *
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing canonical HDL source:
 *
 *     memory name: type [dimension];
 *
 * remains representable.
 *
 * Existing module composition through:
 *
 *     hdlMemoryDeclaration
 *
 * remains the integration boundary.
 *
 * The canonical parser should import this grammar and stop maintaining a
 * second implementation of hdlMemoryDeclaration.
 *
 * ============================================================================
 */