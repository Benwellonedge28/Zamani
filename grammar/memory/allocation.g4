/*

* ============================================================================
* Zamani Universal Computing Language
* ============================================================================
* 
* FILE
* ---
* grammar/memory/allocation.g4
* 
* STATUS
* ---
* Canonical memory-allocation parser component.
* 
* GRAMMAR KIND
* ---
* ANTLR4 parser grammar.
* 
* IMPLEMENTATION BASELINE
* ---
* Rust 1.97 / Rust 1.97.1
* Rust 2021 Edition
* Safe Rust only.
* No unsafe implementation requirement.
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file owns the SOURCE-LEVEL SYNTAX for portable memory-allocation
* intent.
* 
* It describes what memory-related storage/resource behavior the program
* requests. It does not decide how that request is physically realized.
* 
* The semantic pipeline is:
* 
* Zamani source
*      |
*      v
* canonical lexer
*      |
*      v
* parser
*      |
*      v
* allocation syntax
*      |
*      v
* domain-neutral AST
*      |
*      v
* semantic analysis
*      |
*   +--+----------------------+
*   |                         |
*   v                         v
* ownership                 resource/capability
* /lifetime                 analysis
*   |                         |
*   +------------+------------+
*                |
*                v
*         canonical semantic model
*                |
*                v
*           canonical IR
*                |
*   +------------+-------------+
*   |            |             |
*   v            v             v
* classical    quantum::ir   HDL/hardware
*   |            |             |
*   +------------+-------------+
*                |
*                v
*   optimization / lowering
*                |
*   routing / scheduling
*                |
*          target realization
*                |
*             runtime
* 
* ============================================================================
* FILE OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - allocation operation syntax;
* - allocation invocation syntax;
* - allocation arguments;
* - allocation result-binding syntax where explicitly composed;
* - allocation resource-intent clauses;
* - allocation requirement clauses;
* - allocation constraint clauses;
* - allocation preference clauses;
* - allocation hint clauses;
* - allocation policy arguments as semantic data;
* - allocation extent/count arguments as expressions;
* - allocation initialization arguments as expressions;
* - allocation placement/memory-space references by composition;
* - reservation syntax when represented as allocation-domain syntax;
* - allocation-specific named arguments;
* - the allocation-domain composition boundary.
* 
* ============================================================================
* THIS FILE DOES NOT OWN
* ============================================================================
* 
* This file does NOT own:
* 
* - lexer definitions;
* - token spelling;
* - identifiers;
* - qualified-name definitions;
* - general expressions;
* - expression precedence;
* - types;
* - ownership semantics;
* - borrowing semantics;
* - lifetime inference;
* - memory places;
* - memory spaces;
* - memory regions;
* - deallocation;
* - allocator algorithms;
* - garbage collection;
* - reference counting;
* - physical addresses;
* - pointer representation;
* - machine word width;
* - memory topology;
* - NUMA topology;
* - GPU memory;
* - QPU memory;
* - FPGA memory;
* - hardware selection;
* - resource discovery;
* - scheduling;
* - routing;
* - optimization;
* - QEC;
* - ZQN;
* - HAL;
* - runtime execution;
* - classical IR;
* - quantum::ir;
* - HDL IR;
* - hardware IR.
* 
* ============================================================================
* CRITICAL OWNERSHIP RULE
* ============================================================================
* 
* "memory.g4" is the canonical memory-domain foundation.
* 
* Therefore this file MUST NOT redefine:
* 
* memoryPlace
* memoryQualifiedName
* memoryPath
* memoryOperation
* memoryArgument
* memoryArgumentList
* memoryNamedArgument
* memoryTypeAnnotation
* memorySpace
* memorySpaceClause
* memoryLifetime
* memoryLifetimeClause
* 
* Those rules belong to the appropriate canonical memory/type/expression
* components.
* 
* This file specializes them for allocation.
* 
* ============================================================================
* TOKEN POLICY
* ============================================================================
* 
* This file declares NO lexer rules and introduces NO tokens.
* 
* Existing canonical tokens are reused.
* 
* In particular, this file deliberately does NOT require tokens named:
* 
* ALLOCATE
* ALLOCATE_IN
* COUNT
* SIZE
* LENGTH
* LIFETIME
* POLICY
* CONSTRAINED_BY
* 
* because those are not part of the current canonical keyword vocabulary.
* 
* The spelling "allocate" remains an open-world identifier inside:
* 
* memory::allocate
* 
* Existing reserved words such as:
* 
* RESERVE
* RELEASE
* REQUIRES
* PREFER
* HINT
* WITH
* IN
* NEW
* 
* are reused where their existing lexical ownership permits it.
* 
* ============================================================================
* OPEN-WORLD ALLOCATION
* ============================================================================
* 
* Allocation operation identity is semantic data.
* 
* The grammar therefore accepts:
* 
* memory::allocate(...)
* 
* without requiring "allocate" to become a globally reserved keyword.
* 
* This is important for POCO-REAF because future allocation mechanisms can be
* introduced without continually expanding the global keyword set.
* 
* Examples:
* 
* memory::allocate(T)
* memory::allocate(T, n)
* memory::allocate(T, count = n)
* memory::allocate(T, place = memory::shared)
* memory::allocate(T, extent = n)
* 
* The semantic layer determines whether an operation is actually supported.
* 
* ============================================================================
* POCO-REAF / SCALABILITY
* ============================================================================
* 
* This grammar imposes NO universal limit on:
* 
* allocation size;
* allocation count;
* number of allocations;
* number of memory objects;
* number of memory spaces;
* number of regions;
* address width;
* pointer width;
* machine word width;
* memory capacity;
* memory bandwidth;
* device count;
* accelerator count;
* GPU count;
* QPU count;
* FPGA count;
* node count;
* process count;
* thread count.
* 
* No grammar-level constants such as:
* 
* MAX_MEMORY
* MAX_ALLOCATION
* MAX_ALLOCATIONS
* MAX_ADDRESS
* MAX_POINTER
* MAX_DEVICES
* 
* may be introduced here.
* 
* A value such as:
* 
* 1024
* 
* remains an ordinary program value.
* 
* It does NOT become a language-wide capacity limit.
* 
* ============================================================================
* SEMANTIC DISTINCTION
* ============================================================================
* 
* Allocation syntax represents:
* 
* WHAT storage/resource behavior is requested.
* 
* It does not represent:
* 
* HOW a particular machine implements it.
* 
* Therefore:
* 
* memory::allocate(...)
* 
* does not imply:
* 
* malloc
* calloc
* new/delete
* stack allocation
* heap allocation
* physical-page allocation
* GPU allocation
* QPU allocation
* DMA
* NUMA placement
* a particular allocator.
* 
* Those are downstream implementation decisions.
* 
* ============================================================================
* RESOURCE MODEL
* ============================================================================
* 
* Allocation supports four intentionally distinct classes of source intent:
* 
* requirement
* constraint
* preference
* hint
* 
* Requirement:
* 
* Must be satisfied for the requested program meaning.
* 
* Constraint:
* 
* Restricts legal realization.
* 
* Preference:
* 
* Desirable but non-semantic when unavailable.
* 
* Hint:
* 
* Advisory information that MUST NOT change program meaning when ignored.
* 
* Capability discovery and satisfiability remain downstream.
* 
* ============================================================================
* TYPE INTEGRATION
* ============================================================================
* 
* Allocation arguments may contain canonical type expressions.
* 
* This file does not define a second type grammar.
* 
* The canonical type composition supplies:
* 
* typeExpression
* 
* where the composed parser makes it available.
* 
* ============================================================================
* EXPRESSION INTEGRATION
* ============================================================================
* 
* Allocation quantities, predicates, initialization values and named argument
* values use the canonical expression grammar.
* 
* This allows:
* 
* compile-time values;
* runtime values;
* symbolic values;
* generic values;
* computed extents;
* dynamically discovered quantities;
* resource-dependent values.
* 
* No finite numeric grammar limit is imposed.
* 
* ============================================================================
* MEMORY FOUNDATION INTEGRATION
* ============================================================================
* 
* "memory.g4" owns:
* 
* memoryPlace
* memoryQualifiedName
* memoryOperation
* memoryArgument
* memoryArgumentList
* memoryNamedArgument
* memorySpace
* memoryLifetime
* 
* Allocation consumes those abstractions.
* 
* It MUST NOT reproduce them locally.
* 
* ============================================================================
* OWNERSHIP / LIFETIME INTEGRATION
* ============================================================================
* 
* Allocation can be associated semantically with:
* 
* ownership
* borrowing
* lifetime
* regions
* 
* but does not validate those relationships.
* 
* The corresponding grammar components remain:
* 
* grammar/memory/ownership.g4
* grammar/memory/borrowing.g4
* grammar/memory/lifetimes.g4
* 
* Semantic analysis determines:
* 
* whether the resulting object is owned;
* whether the object may move;
* whether it may be borrowed;
* whether its lifetime is valid;
* whether a region relationship is legal;
* whether allocation is compatible with its type.
* 
* ============================================================================
* DEALLOCATION INTEGRATION
* ============================================================================
* 
* Allocation and deallocation are separate semantic operations.
* 
* This file does NOT define deallocation syntax.
* 
* Deallocation remains owned by:
* 
* grammar/memory/deallocation.g4
* 
* This prevents allocation.g4 from becoming responsible for lifetime-end
* semantics.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* Allocation syntax MUST NOT allocate physical qubits.
* 
* Quantum semantic lowering remains:
* 
* allocation intent
*      |
*      v
* semantic analysis
*      |
*      v
* quantum semantic representation
*      |
*      v
* quantum::ir
*      |
*      v
* optimization
*      |
*      v
* routing
*      |
*      v
* scheduling
*      |
*      v
* QEC / resilience / ZQN
*      |
*      v
* HAL
*      |
*      v
* physical realization
* 
* This grammar MUST NOT contain physical-qubit identifiers, topology limits,
* device identifiers, calibration data, or QEC implementation rules.
* 
* ============================================================================
* HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* Allocation intent can describe semantic storage/resource requirements for
* hardware/software co-design.
* 
* It MUST NOT encode:
* 
* fixed bus width;
* fixed register count;
* fixed memory-bank count;
* fixed accelerator count;
* fixed FPGA resources;
* physical addresses;
* device IDs.
* 
* Those properties belong to hardware/resource analysis and downstream
* realization.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* This grammar contains:
* 
* no semantic predicates;
* no actions;
* no embedded Rust;
* no filesystem access;
* no network access;
* no hardware probing;
* no randomness;
* no environment inspection.
* 
* Given the same source and grammar version, parsing is deterministic.
* 
* ============================================================================
* SECURITY
* ============================================================================
* 
* Parsing an allocation construct MUST NOT:
* 
* allocate memory;
* access an address;
* contact hardware;
* contact a device;
* access credentials;
* execute source;
* execute an allocator;
* inspect runtime state.
* 
* This is syntax only.
* 
* ============================================================================
* PUBLIC ENTRY POINT
* ============================================================================
* 
* "allocationConstruct" is the single public entry point for this component.
* 
* The parser composition layer decides where this construct is accepted:
* 
* memoryConstruct
* statement
* expression
* declaration
* 
* This file does not create another program root.
* ============================================================================
  */

parser grammar Allocation;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* PUBLIC COMPOSITION ENTRY
* ============================================================================
  */

allocationConstruct
: allocationStatement
| allocationExpression
;

/*

* ============================================================================
* ALLOCATION STATEMENT
* ============================================================================
* 
* Canonical function-style allocation:
* 
* memory::allocate(...)
* 
* The semicolon belongs to the statement boundary.
  */
  allocationStatement
  : allocationInvocation
  SEMICOLON
  ;

/*

* ============================================================================
* ALLOCATION EXPRESSION
* ============================================================================
* 
* Allocation may be expression-producing when semantic analysis establishes
* that the operation yields a memory/resource value.
  */
  allocationExpression
  : allocationInvocation
  ;

/*

* ============================================================================
* CORE ALLOCATION INVOCATION
* ============================================================================
* 
* The operation name is deliberately open-world.
* 
* The spelling "allocate" is an identifier and therefore does not require a
* new global lexer token.
* 
* The canonical semantic namespace is:
* 
* memory::allocate

*/
allocationInvocation
: allocationOperationName
LPAREN allocationArgumentList? RPAREN
;

/*

* ============================================================================
* ALLOCATION OPERATION NAME
* ============================================================================
* 
* "memory::allocate" is the canonical allocation operation.
* 
* The namespace is supplied as ordinary qualified-name syntax rather than
* introducing a MEMORY keyword.
* 
* This preserves the repository's open-world memory-operation model.
  */
  allocationOperationName
  : allocationMemoryNamespace DOUBLE_COLON allocationName
  ;

/*

* ============================================================================
* MEMORY NAMESPACE
* ============================================================================
* 
* "memory" is intentionally an identifier.
* 
* This avoids adding a new reserved keyword while remaining compatible with
* memory.g4's qualified-name architecture.
  */
  allocationMemoryNamespace
  : identifier
  ;

/*

* ============================================================================
* ALLOCATION NAME
* ============================================================================
* 
* The primary allocation operation is "allocate".
* 
* It remains an identifier rather than a globally reserved keyword.
* 
* Additional allocation-domain operation names can be introduced through
* semantic/dialect extension without modifying the lexical core.
  */
  allocationName
  : identifier
  ;

/*

* ============================================================================
* ALLOCATION ARGUMENT LIST
* ============================================================================
* 
* Allocation arguments are unbounded by source-language policy.
* 
* They can carry:
* 
* type information;
* extent;
* initialization;
* memory-place intent;
* resource requirements;
* constraints;
* preferences;
* hints;
* policies;
* domain-specific semantic metadata.

/
allocationArgumentList
: allocationArgument
(
COMMA
allocationArgument
)
COMMA?
;

/*

* ============================================================================
* ALLOCATION ARGUMENT
* ============================================================================
* 
* An argument is either:
* 
* positional semantic data
* 
* or:
* 
* named semantic data.
* 
* Named arguments allow future allocation properties without continually
* changing the grammar.
  */
  allocationArgument
  : allocationNamedArgument
  | expression
  ;

/*

* ============================================================================
* NAMED ALLOCATION ARGUMENT
* ============================================================================
* 
* Examples:
* 
* count = n
* extent = n
* place = memory::shared
* initialize = value
* policy = policy_name
* requirement = predicate
* constraint = predicate
* preference = predicate
* hint = predicate
* 
* The names are identifiers and therefore remain open-world.
  */
  allocationNamedArgument
  : identifier ASSIGN expression
  ;

/*

* ============================================================================
* OPTIONAL TYPE-DIRECTED ALLOCATION FORM
* ============================================================================
* 
* A canonical type may be represented through a dedicated semantic argument:
* 
* memory::allocate(type = T)
* 
* This is intentionally represented as a named argument rather than creating
* a second type grammar or a new allocation-specific type syntax.
* 
* The value after "type =" is parsed as an expression at this layer; semantic
* analysis determines whether the referenced value denotes a type according
* to the canonical type/metaprogramming system.
* 
* ============================================================================
  */

/*

* ============================================================================
* EXTENT / COUNT SEMANTICS
* ============================================================================
* 
* No dedicated COUNT/SIZE/LENGTH tokens are required.
* 
* Extents are represented by named arguments:
* 
* extent = n
* count = n
* elements = n
* 
* These names remain identifiers.
* 
* This means the grammar does not establish a finite vocabulary for future
* allocation dimensions.
* 
* Semantic validation determines which names are valid for a particular
* allocation operation.
* 
* ============================================================================
  */

/*

* ============================================================================
* MEMORY-SPACE / PLACEMENT SEMANTICS
* ============================================================================
* 
* Placement is represented as allocation data, for example:
* 
* place = memory::shared
* space = memory::device
* 
* The grammar does not define or enumerate:
* 
* local
* shared
* device
* persistent
* remote
* NUMA
* GPU
* QPU
* 
* Those are semantic memory-space names.
* 
* ============================================================================
  */

/*

* ============================================================================
* RESOURCE REQUIREMENTS
* ============================================================================
* 
* Requirements can be represented through the existing "REQUIRES" keyword:
* 
* memory::allocate(
*     type = T,
*     extent = n
* ) requires ...
* 
* However, the canonical memory-operation model already represents arbitrary
* semantic arguments. Therefore this component keeps requirement syntax
* compositional and does not redefine the repository-wide resource grammar.
* 
* The parser composition layer should use the canonical resource requirement
* grammar when that grammar is available at the same statement boundary.
* 
* ============================================================================
  */

/*

* ============================================================================
* RESOURCE CONSTRAINTS / PREFERENCES / HINTS
* ============================================================================
* 
* The repository already owns:
* 
* CONSTRAINT
* PREFER
* HINT
* 
* at the lexical level.
* 
* Allocation-specific properties should normally be represented as named
* arguments so that this file does not invent a second resource grammar.
* 
* Examples:
* 
* memory::allocate(
*     type = T,
*     extent = n,
*     constraint = memory::latency < bound,
*     preference = memory::bandwidth >= desired,
*     hint = memory::reuse
* )
* 
* The semantic resource system determines their interpretation.
* 
* ============================================================================
  */

/*

* ============================================================================
* OPTIONAL EXISTING "new" INTEGRATION
* ============================================================================
* 
* The repository already has the "NEW" token.
* 
* "new" remains owned by the general expression/construction grammar.
* 
* This file MUST NOT redefine "new".
* 
* Therefore:
* 
* new T(...)
* 
* is not parsed through allocationInvocation unless the canonical expression
* composition explicitly delegates it here.
* 
* This prevents allocation.g4 from competing with the universal construction
* expression.
* 
* ============================================================================
  */

/*

* ============================================================================
* RESERVATION INTEGRATION
* ============================================================================
* 
* "RESERVE" already exists in the canonical keyword vocabulary.
* 
* Reservation is semantically related to allocation but is NOT the same
* operation.
* 
* It therefore remains outside the primary "allocationInvocation" rule.
* 
* The memory composition layer may route:
* 
* memory::reserve(...)
* 
* through the generic "memoryOperation" rule in memory.g4, or through the
* resource reservation grammar.
* 
* This file does not duplicate reservation semantics.
* 
* ============================================================================
  */

/*

* ============================================================================
* ALLOCATION TARGET / RESULT
* ============================================================================
* 
* Assignment to an allocation result remains owned by the canonical expression
* and statement grammar.
* 
* Examples:
* 
* buffer = memory::allocate(type = T, extent = n);
* 
* let buffer = memory::allocate(type = T, extent = n);
* 
* allocation.g4 MUST NOT duplicate:
* 
* assignmentExpression
* variableDeclaration
* let
* binding
* 
* because those belong elsewhere.
* 
* ============================================================================
  */

/*

* ============================================================================
* DYNAMIC / SYMBOLIC ALLOCATION
* ============================================================================
* 
* These are intentionally valid at the grammar level:
* 
* memory::allocate(type = T, extent = n)
* 
* memory::allocate(type = T, extent = compute_extent())
* 
* memory::allocate(type = T, extent = symbolic_extent)
* 
* memory::allocate(type = T, extent = available_extent)
* 
* memory::allocate(type = T, extent = resource::capacity(...))
* 
* Whether the value is:
* 
* compile-time known;
* runtime known;
* symbolic;
* negotiated;
* resource-dependent
* 
* is a semantic/compiler question.
* 
* ============================================================================
  */

/*

* ============================================================================
* RESIZABLE / GROWABLE ALLOCATION
* ============================================================================
* 
* Allocation itself does not define resize syntax.
* 
* A resizable allocation can be expressed through semantic metadata:
* 
* memory::allocate(
*     type = T,
*     extent = initial,
*     growth = policy
* )
* 
* A dedicated resize/grow operation belongs to the appropriate memory
* operation/deformation grammar and must not be duplicated here.
* 
* ============================================================================
  */

/*

* ============================================================================
* INITIALIZATION
* ============================================================================
* 
* Initialization is represented as ordinary named data:
* 
* initialize = expression
* 
* This file does not define initialization expressions.
* 
* The canonical expression grammar owns them.
* 
* ============================================================================
  */

/*

* ============================================================================
* POLICY
* ============================================================================
* 
* Allocation policies are semantic data:
* 
* policy = memory::policy
* 
* The grammar does not enumerate allocator strategies such as:
* 
* heap
* stack
* buddy
* slab
* arena
* pool
* garbage_collected
* reference_counted
* 
* Such names remain semantic/library/dialect identifiers.
* 
* ============================================================================
  */

/*

* ============================================================================
* FUTURE MEMORY DOMAINS
* ============================================================================
* 
* The following must remain representable without changing this file:
* 
* memory::persistent
* memory::distributed
* memory::remote
* memory::device
* memory::accelerator
* memory::quantum_associated
* memory::optical
* memory::neuromorphic
* memory::future_domain
* 
* These are examples of semantic names, not a closed grammar enumeration.
* 
* ============================================================================
  */

/*

* ============================================================================
* HARDWARE INDEPENDENCE
* ============================================================================
* 
* Forbidden allocation grammar concepts include:
* 
* gpu0
* qpu0
* cpu0
* fpga0
* node0
* memory_bank0
* physical_address
* 
* as language-defined allocation mechanisms.
* 
* A source program may contain such names as ordinary user identifiers where
* the language permits them, but allocation.g4 must not give them intrinsic
* physical semantics.
* 
* ============================================================================
  */

/*

* ============================================================================
* RESOURCE SCALING
* ============================================================================
* 
* The following source forms must remain grammatically possible regardless of
* eventual resource scale:
* 
* extent = 0
* extent = 1
* extent = n
* extent = huge_value
* extent = runtime_extent
* extent = computed_extent
* extent = negotiated_extent
* 
* The semantic/resource layer determines whether an actual realization exists.
* 
* The parser does not reject a program merely because today's machine cannot
* satisfy its resource request.
* 
* ============================================================================
  */

/*

* ============================================================================
* ERROR BOUNDARY
* ============================================================================
* 
* Parser errors:
* 
* malformed parentheses;
* malformed commas;
* malformed assignment;
* malformed qualified operation syntax
* 
* belong to parsing.
* 
* Semantic errors:
* 
* unsupported allocation operation;
* invalid allocation type;
* invalid extent;
* unsatisfied capability;
* invalid memory place;
* illegal ownership transition;
* invalid lifetime;
* impossible resource requirement
* 
* belong downstream.
* 
* The grammar MUST NOT encode semantic hardware/resource decisions.
* 
* ============================================================================
  */

/*

* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* A successful allocation parse must provide sufficient source structure for
* the domain-neutral frontend AST to preserve:
* 
* source span;
* operation namespace;
* operation name;
* ordered positional arguments;
* named arguments;
* argument source spans;
* exact source spelling where required by provenance/tooling.
* 
* The AST layer decides whether this maps to a generic:
* 
* Operation
* 
* or another existing domain-neutral operation representation.
* 
* This grammar MUST NOT create:
* 
* QuantumGate
* PhysicalAllocation
* GpuAllocation
* QpuAllocation
* CpuAllocation
* 
* AST nodes.
* 
* ============================================================================
  */

/*

* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis must determine:
* 
* - whether `memory::allocate` exists;
* - whether the operation is legal in the current context;
* - whether its type arguments are valid;
* - whether its extent is valid;
* - whether its requirements are satisfiable;
* - whether its constraints are compatible;
* - whether preferences can be honored;
* - whether hints are valid;
* - whether ownership/lifetime rules permit the result;
* - what canonical semantic operation is produced.
* 
* None of these decisions belong to this parser grammar.
* 
* ============================================================================
  */

/*

* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* Allocation syntax does NOT directly select:
* 
* classical IR;
* quantum::ir;
* HDL IR;
* hardware IR;
* LLVM;
* MLIR;
* QIR;
* vendor IR.
* 
* Instead:
* 
* allocation syntax
*      |
*      v
* domain-neutral AST
*      |
*      v
* semantic analysis
*      |
*      v
* canonical semantic representation
*      |
*      +----------------+------------------+
*      |                |                  |
*      v                v                  v
* classical        quantum::ir       HDL/hardware
* 
* This preserves the repository's single semantic ownership model.
* 
* ============================================================================
  */

/*

* ============================================================================
* COMPILER INTEGRATION
* ============================================================================
* 
* The compiler may lower allocation intent into:
* 
* stack-like storage;
* heap-like storage;
* region storage;
* shared storage;
* distributed storage;
* accelerator storage;
* device-associated storage;
* another target representation.
* 
* The choice is target/resource dependent and MUST NOT be encoded here.
* 
* ============================================================================
  */

/*

* ============================================================================
* RUNTIME INTEGRATION
* ============================================================================
* 
* Runtime behavior may:
* 
* allocate;
* defer;
* pool;
* migrate;
* virtualize;
* distribute;
* reject;
* negotiate;
* 
* according to semantic validity and available resources.
* 
* This grammar does none of those things.
* 
* ============================================================================
  */

/*

* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* Required positive tests include:
* 
* memory::allocate()
* memory::allocate(type = T)
* memory::allocate(type = T, extent = n)
* memory::allocate(T, n)
* memory::allocate(type = T, place = memory::shared)
* memory::allocate(type = T, extent = compute_extent())
* memory::allocate(type = T, policy = memory::policy)
* memory::allocate(type = T, requirement = requirement)
* memory::allocate(type = T, constraint = constraint)
* memory::allocate(type = T, preference = preference)
* memory::allocate(type = T, hint = hint)
* 
* Required statement tests include:
* 
* memory::allocate(type = T);
* buffer = memory::allocate(type = T, extent = n);
* 
* Required negative tests include:
* 
* memory::allocate(
* memory::allocate(type = )
* memory::allocate(type = T, )
*   [only reject if trailing comma is not accepted by the canonical
*    expression/list contract]
* memory:allocate(type = T)
* memory::allocate(type T)
* 
* Required scalability tests include:
* 
* symbolic extent;
* runtime extent;
* arbitrarily long argument lists within implementation resources;
* arbitrarily deep qualified memory names where supported by the canonical
* name grammar;
* allocation of quantum-associated classical storage;
* distributed storage intent;
* accelerator-associated storage;
* future memory-domain names.
* 
* Required cross-domain tests include:
* 
* classical + allocation;
* quantum + allocation;
* hybrid + allocation;
* HDL/hardware + allocation;
* distributed + allocation;
* AI/data + allocation.
* 
* ============================================================================
* COMPATIBILITY
* ============================================================================
* 
* Existing lexical tokens remain unchanged.
* 
* No new keyword is required to parse:
* 
* memory::allocate(...)
* 
* Therefore this file does not force a global lexical compatibility break.
* 
* Existing "NEW" remains owned by the canonical construction grammar.
* 
* Existing "RESERVE" and "RELEASE" remain owned by the canonical keyword
* grammar and are not redefined here.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This file contains:
* 
* no maximum resource constant;
* no fixed allocation count;
* no fixed memory size;
* no fixed address width;
* no fixed pointer width;
* no fixed machine width;
* no device identifier;
* no physical address;
* no topology;
* no fixed quantum size;
* no fixed CPU/GPU/FPGA/QPU count;
* no target-specific allocation algorithm.
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when:
* 
* [x] The file has one public allocation entry point.
* [x] Existing tokens are reused.
* [x] No new lexer token is required.
* [x] "memory::allocate" remains open-world.
* [x] "memory.g4" owns generic memory names/places/operations.
* [x] General expressions remain externally owned.
* [x] General types remain externally owned.
* [x] Ownership remains externally owned.
* [x] Borrowing remains externally owned.
* [x] Lifetimes remain externally owned.
* [x] Deallocation remains externally owned.
* [x] Resource discovery remains downstream.
* [x] Hardware realization remains downstream.
* [x] Quantum physical allocation remains downstream.
* [x] quantum::ir remains canonical.
* [x] No finite hardware/resource limit exists.
* [x] No physical address is encoded.
* [x] No allocator implementation is encoded.
* [x] No ANTLR actions or semantic predicates exist.
* [x] No unsafe Rust is required.
* 
* Integration acceptance additionally requires:
* 
* [ ] ZamaniParser.g4 imports Allocation exactly once.
* [ ] Memory composition exposes allocationConstruct exactly once.
* [ ] The canonical "identifier" rule is available to this grammar.
* [ ] The canonical "expression" rule is available to this grammar.
* [ ] The AST has a generic operation representation for allocation intent.
* [ ] Semantic analysis validates allocation-specific named arguments.
* [ ] Resource analysis consumes requirement/constraint/preference/hint data.
* [ ] Ownership/lifetime analysis consumes allocation result information.
* [ ] Canonical IR lowering exists.
* [ ] Positive tests exist.
* [ ] Negative tests exist.
* [ ] Boundary tests exist.
* [ ] Scalability tests exist.
* [ ] Determinism tests exist.
* [ ] Cross-domain tests exist.
* 
* ============================================================================
* FINAL ARCHITECTURAL INVARIANT
* ============================================================================
* 
* allocation.g4
*      |
*      v
* portable allocation intent
*      |
*      v
* domain-neutral AST
*      |
*      v
* semantic/resource analysis
*      |
*      v
* canonical semantic representation
*      |
*      +-----------------------------+
*      |                             |
*      v                             v
* target-independent             target-specific
* meaning                        realization
* 
* Never:
* 
* allocation.g4
*      |
*      v
* physical hardware
* 
* The governing principle is:
* 
* MEMORY ALLOCATION IS SEMANTIC RESOURCE INTENT,
* NOT A FIXED MACHINE SHAPE.
* 
* ============================================================================
  */