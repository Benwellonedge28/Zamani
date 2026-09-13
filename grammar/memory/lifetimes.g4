/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/memory/lifetimes.g4
* 
* Grammar:
* ANTLR4 parser grammar
* 
* Role:
* Canonical source-level lifetime syntax for the Zamani memory model.
* 
* Implementation baseline:
* Rust 1.97 / Rust 1.97.1
* Edition 2021
* Safe Rust only
* No unsafe Rust
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file owns the SOURCE SYNTAX of lifetime constructs.
* 
* It provides a reusable lifetime grammar for:
* 
* - reference types;
* - borrowing;
* - ownership;
* - memory regions;
* - generic lifetime parameters;
* - lifetime bounds;
* - lifetime constraints;
* - lifetime annotations;
* - future memory-domain constructs.
* 
* This file is intentionally syntax-only.
* 
* It does NOT implement lifetime analysis.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* Zamani source
*      |
*      v
* ZamaniLexer
*      |
*      v
* Core/domain parser
*      |
*      +-----------------------------+
*      |                             |
*      v                             v
*    Types                       Memory
*      |                             |
*      +-------------+---------------+
*                    |
*                    v
*              Lifetimes.g4
*                    |
*                    v
*                 AST
*                    |
*                    v
*          structural validation
*                    |
*                    v
*            semantic analysis
*                    |
*          +---------+---------+
*          |                   |
*          v                   v
*   borrow analysis      ownership analysis
*          |                   |
*          +---------+---------+
*                    |
*                    v
*            canonical semantic IR
*                    |
*      +-------------+-------------+
*      |             |             |
*      v             v             v
*  classical      quantum         HDL
*      |             |             |
*      +-------------+-------------+
*                    |
*                    v
*         optimization / lowering /
*         routing / scheduling /
*         resource realization
* 
* The dependency direction MUST NOT be reversed.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - lifetime-name syntax;
* - lifetime annotations;
* - lifetime parameters;
* - lifetime parameter lists;
* - lifetime bounds;
* - lifetime-bound lists;
* - lifetime constraint syntax;
* - lifetime where-style constraints;
* - anonymous lifetime syntax;
* - lifetime references;
* - the reusable parser contract for lifetime syntax.
* 
* ============================================================================
* DOES NOT OWN
* ============================================================================
* 
* This file does NOT own:
* 
* - lexical identifier rules;
* - Unicode identifier rules;
* - Unicode normalization;
* - general expressions;
* - general types;
* - reference types;
* - pointer types;
* - borrowing semantics;
* - ownership semantics;
* - allocation;
* - deallocation;
* - memory layout;
* - physical addresses;
* - stack/heap selection;
* - garbage collection;
* - allocator implementation;
* - memory discovery;
* - NUMA topology;
* - device memory;
* - GPU memory;
* - QPU resources;
* - hardware topology;
* - scheduling;
* - routing;
* - optimization;
* - QEC;
* - ZQN;
* - resilience;
* - classical IR;
* - quantum::ir;
* - HDL IR;
* - runtime execution.
* 
* ============================================================================
* SINGLE SOURCE OF LIFETIME SYNTAX
* ============================================================================
* 
* This file is intended to become the canonical reusable lifetime grammar.
* 
* Existing lifetime rules elsewhere in the repository MUST NOT remain as
* independent competing definitions.
* 
* In particular, the following existing concepts:
* 
* lifetimeAnnotation
* memoryLifetime
* 
* should be migrated to consume the rules provided by this grammar.
* 
* The semantic name of an AST node is an AST concern and does not require the
* grammar rule name to be identical.
* 
* ============================================================================
* LEXER CONTRACT
* ============================================================================
* 
* This is a PARSER GRAMMAR.
* 
* It MUST use the canonical Zamani lexer:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* It MUST NOT declare lexer rules.
* 
* Lifetime syntax depends only on existing lexical primitives:
* 
* APOSTROPHE
* IDENT
* UNDERSCORE
* COLON
* COMMA
* LT
* GT
* WHERE
* 
* Exact token names are inherited from the canonical lexer/parser vocabulary.
* 
* The lexer remains responsible for:
* 
* - identifier spelling;
* - Unicode;
* - normalization policy;
* - reserved words;
* - token boundaries.
* 
* This file does not duplicate any of those rules.
* 
* ============================================================================
* LIFETIME SYNTAX MODEL
* ============================================================================
* 
* A lifetime is a SOURCE-LEVEL NAME.
* 
* Canonical named examples:
* 
* 'a
* 'buffer
* 'scope
* 'region
* 
* Anonymous lifetime:
* 
* '_
* 
* The leading apostrophe is syntactic decoration.
* 
* The AST should retain the logical lifetime identity without requiring the
* semantic layer to reinterpret source spelling.
* 
* ============================================================================
* LIFETIME NAME
* ============================================================================
* 
* A named lifetime consists of:
* 
* apostrophe + canonical identifier
* 
* Examples:
* 
* 'a
* 'buffer
* 'scope
* 
* The identifier remains owned by the canonical identifier system.
* 
* ============================================================================
* ANONYMOUS LIFETIME
* ============================================================================
* 
* The anonymous lifetime:
* 
* '_
* 
* is syntactically distinct from a named lifetime.
* 
* It represents source-level absence of an explicit lifetime identity.
* 
* The grammar does NOT decide what semantic lifetime inference means.
* 
* ============================================================================
* LIFETIME ANNOTATION
* ============================================================================
* 
* A lifetime annotation is the same source-level lifetime reference used by
* type and memory constructs.
* 
* Examples:
* 
* &'a T
* &'buffer T
* 
* This grammar provides the reusable:
* 
* lifetimeAnnotation
* 
* rule.
* 
* Reference types remain responsible for combining it with:
* 
* &
* mut
* typeExpression
* 
* ============================================================================
* LIFETIME PARAMETERS
* ============================================================================
* 
* Lifetime parameters may be used by generic declarations.
* 
* Conceptually:
* 
* <'a, 'b>
* 
* The grammar permits an arbitrary number of parameters.
* 
* No fixed lifetime-parameter count is encoded.
* 
* This is essential for:
* 
* generic APIs;
* higher-order abstractions;
* nested resource systems;
* distributed computations;
* asynchronous computations;
* future memory models.
* 
* ============================================================================
* LIFETIME BOUNDS
* ============================================================================
* 
* A lifetime bound expresses a source-level relationship between lifetime
* names.
* 
* Canonical form:
* 
* 'a: 'b
* 
* Semantically, this may later express an outlives relationship.
* 
* IMPORTANT:
* 
* The grammar does NOT decide that:
* 
* 'a: 'b
* 
* is semantically valid.
* 
* It only records that the source contains a lifetime-bound relation.
* 
* ============================================================================
* MULTIPLE LIFETIME BOUNDS
* ============================================================================
* 
* Multiple bounds are represented as an ordered list.
* 
* Example:
* 
* 'a: 'b + 'c
* 
* The plus operator is already part of the language's type-bound vocabulary.
* 
* The lifetime grammar therefore does not introduce a new relation operator.
* 
* Semantic analysis determines whether the resulting bound set is meaningful.
* 
* ============================================================================
* LIFETIME CONSTRAINTS
* ============================================================================
* 
* Lifetime constraints can appear in generic/constraint contexts.
* 
* The grammar supports:
* 
* lifetimeBound
* 
* and:
* 
* lifetimeBoundList
* 
* A downstream generic/type grammar can embed these rules in:
* 
* where clauses;
* generic constraints;
* function constraints;
* type constraints;
* reference constraints.
* 
* This file does not own the complete "where" grammar.
* 
* ============================================================================
* WHERE-STYLE LIFETIME CONSTRAINTS
* ============================================================================
* 
* Where-style syntax is represented using the existing WHERE token.
* 
* Example:
* 
* where 'a: 'b
* 
* Multiple constraints may be separated by commas.
* 
* Example:
* 
* where 'a: 'b, 'b: 'c
* 
* No fixed number of constraints is permitted.
* 
* ============================================================================
* ARBITRARY LIFETIME NAMES
* ============================================================================
* 
* Lifetime names are not restricted to:
* 
* 'a
* 'b
* 'c
* 
* Longer names are valid:
* 
* 'buffer
* 'transaction
* 'request_scope
* 'quantum_region
* 'distributed_resource
* 
* This is important because Zamani is not restricted to a Rust-like small
* lifetime alphabet.
* 
* ============================================================================
* NO MACHINE ASSUMPTIONS
* ============================================================================
* 
* Lifetime syntax MUST NOT encode:
* 
* - memory addresses;
* - pointer width;
* - address width;
* - stack size;
* - heap size;
* - cache size;
* - register count;
* - CPU count;
* - GPU count;
* - FPGA count;
* - QPU count;
* - qubit count;
* - node count;
* - memory-bank count;
* - fixed allocation capacity.
* 
* There is intentionally no:
* 
* MAX_LIFETIMES
* MAX_LIFETIME_DEPTH
* MAX_LIFETIME_PARAMETERS
* MAX_LIFETIME_BOUNDS
* MAX_REGIONS
* 
* Any compiler resource-protection limit belongs to an explicit compiler
* resource policy and MUST NOT become a language semantic limit.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Lifetime syntax expresses portable source semantics.
* 
* It must remain independent of the machine on which the program eventually
* executes.
* 
* The same source lifetime syntax must therefore be usable for:
* 
* embedded systems;
* CPUs;
* GPUs;
* FPGAs;
* ASICs;
* quantum systems;
* simulators;
* heterogeneous systems;
* distributed systems;
* cloud systems;
* future architectures.
* 
* A lifetime MUST NOT imply a physical storage location.
* 
* ============================================================================
* MEMORY-MODEL INTEGRATION
* ============================================================================
* 
* Lifetime syntax may participate in:
* 
* ownership;
* borrowing;
* allocation;
* deallocation;
* shared memory;
* distributed memory;
* resource handles.
* 
* It does not determine their implementation.
* 
* For example:
* 
* 'a
* 
* does not mean:
* 
* stack lifetime;
* heap lifetime;
* CPU lifetime;
* GPU lifetime;
* QPU lifetime;
* network lifetime.
* 
* Semantic analysis assigns the appropriate meaning.
* 
* ============================================================================
* REFERENCE-TYPE INTEGRATION
* ============================================================================
* 
* The canonical reference grammar should consume:
* 
* lifetimeAnnotation
* 
* from this grammar.
* 
* Conceptually:
* 
* referenceType
*     : AMPERSAND
*       lifetimeAnnotation?
*       MUT?
*       typeExpression
*     ;
* 
* This allows:
* 
* &T
* &mut T
* &'a T
* &'a mut T
* 
* without the reference grammar redefining lifetime syntax.
* 
* ============================================================================
* BORROWING INTEGRATION
* ============================================================================
* 
* Borrowing grammar may consume:
* 
* lifetimeAnnotation
* lifetimeName
* 
* but MUST NOT redefine them.
* 
* Borrowing semantics remain downstream.
* 
* The grammar does not determine whether:
* 
* &'a x
* 
* is valid.
* 
* It only recognizes lifetime syntax when embedded in a valid borrowing
* construct.
* 
* ============================================================================
* OWNERSHIP INTEGRATION
* ============================================================================
* 
* Ownership grammar may use lifetime names when expressing source-level
* ownership policies.
* 
* Ownership semantics remain owned by semantic analysis.
* 
* This file does not decide:
* 
* move;
* copy;
* drop;
* clone;
* aliasing;
* exclusivity.
* 
* ============================================================================
* MEMORY-REGION INTEGRATION
* ============================================================================
* 
* A memory-region abstraction may associate a semantic region with a lifetime.
* 
* Example:
* 
* 'region
* 
* The grammar records the lifetime name.
* 
* Region creation, nesting, inheritance and destruction remain semantic
* concerns.
* 
* ============================================================================
* CONCURRENCY INTEGRATION
* ============================================================================
* 
* Lifetime annotations may appear in concurrent APIs.
* 
* The grammar does not decide whether a lifetime may cross:
* 
* task;
* thread;
* actor;
* channel;
* future;
* asynchronous boundary;
* synchronization boundary.
* 
* Those are semantic safety decisions.
* 
* ============================================================================
* DISTRIBUTED INTEGRATION
* ============================================================================
* 
* A lifetime may syntactically occur in types representing distributed
* resources.
* 
* Example:
* 
* &'resource RemoteBuffer
* 
* The grammar does not decide whether the lifetime can cross a node boundary.
* 
* Distributed ownership, serialization, migration and consistency are
* downstream responsibilities.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* Lifetime syntax is domain-neutral.
* 
* A lifetime may be associated semantically with:
* 
* classical values;
* quantum values;
* logical quantum resources;
* hardware resources;
* accelerator resources;
* distributed resources;
* HDL values;
* future computational resources.
* 
* The lifetime grammar MUST NOT reference:
* 
* quantum::ir;
* QEC;
* ZQN;
* physical qubits;
* QPU topology;
* hardware routing.
* 
* A source lifetime does not identify a physical qubit.
* 
* ============================================================================
* HARDWARE INTEGRATION
* ============================================================================
* 
* Hardware-related semantic types may contain lifetime annotations.
* 
* The grammar does not determine:
* 
* device ownership;
* DMA lifetime;
* device allocation;
* accelerator residency;
* memory placement;
* physical resource availability.
* 
* Those belong to resource and hardware analysis.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* This grammar must lower into the existing frontend AST lifetime model.
* 
* The repository already defines:
* 
* LifetimeName
* 
* as a source-level lifetime representation.
* 
* The AST should preserve:
* 
* - lifetime spelling;
* - whether the lifetime is named or anonymous;
* - source span;
* - ordering in parameter/bound lists.
* 
* The grammar does NOT define the Rust AST type.
* 
* ============================================================================
* AST MAPPING
* ============================================================================
* 
* Conceptual mappings:
* 
* 'a
* 
*     -> LifetimeName("a")
* 
* 'buffer
* 
*     -> LifetimeName("buffer")
* 
* '_
* 
*     -> anonymous lifetime representation.
* 
* 'a: 'b
* 
*     -> lifetime-bound AST node containing:
*          left = 'a
*          right = 'b
* 
* The exact AST constructor remains owned by the frontend AST implementation.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis determines:
* 
* - whether a lifetime name is declared;
* - whether a lifetime is in scope;
* - whether duplicate declarations are legal;
* - whether anonymous lifetimes are legal in a context;
* - whether a lifetime bound is satisfiable;
* - whether one lifetime outlives another;
* - whether a reference outlives its referent;
* - whether borrowing is valid;
* - whether ownership constraints are satisfied;
* - whether a lifetime crosses a concurrency boundary;
* - whether a lifetime crosses a distributed boundary;
* - whether a resource remains valid;
* - whether an operation requires a longer lifetime;
* - whether lifetime constraints are contradictory.
* 
* None of these decisions are encoded in ANTLR.
* 
* ============================================================================
* TYPE SYSTEM CONTRACT
* ============================================================================
* 
* This file MUST NOT redefine:
* 
* typeExpression
* referenceType
* pointerType
* genericType
* 
* Those remain type-system responsibilities.
* 
* This grammar only provides reusable lifetime syntax.
* 
* ============================================================================
* EXPRESSION CONTRACT
* ============================================================================
* 
* This file does not define:
* 
* expression;
* identifierExpression;
* calls;
* indexing;
* arithmetic;
* assignment;
* control expressions.
* 
* Lifetime constraints are syntactic relations between lifetime names.
* 
* ============================================================================
* PARSER INTEGRATION
* ============================================================================
* 
* This file is a parser grammar intended to be imported by a parser that needs
* lifetime functionality.
* 
* Example integration:
* 
* parser grammar Types;
* 
* options {
*     tokenVocab = ZamaniLexer;
* }
* 
* import Lifetimes;
* 
* The exact ANTLR composition is determined by the repository's canonical
* parser-generation configuration.
* 
* ============================================================================
* EXISTING FILE MIGRATION CONTRACT
* ============================================================================
* 
* The following existing definitions must eventually be consolidated:
* 
* grammar/types/types.g4
*     lifetimeAnnotation
* 
* grammar/types/reference-types.g4
*     lifetimeAnnotation
* 
* grammar/memory/memory.g4
*     memoryLifetime
* 
* They MUST NOT remain semantically independent lifetime grammars.
* 
* Migration target:
* 
* Lifetimes.g4
*      |
*      +--> Types.g4
*      |
*      +--> ReferenceTypes.g4
*      |
*      +--> Borrowing.g4
*      |
*      +--> Ownership.g4
*      |
*      +--> Memory.g4
* 
* This prevents divergent lifetime syntax.
* 
* ============================================================================
* IMPORTANT ANTLR INTEGRATION RULE
* ============================================================================
* 
* This file deliberately does not use:
* 
* typeExpression
* expression
* pattern
* qualifiedName
* 
* because doing so would make this foundational grammar dependent on higher
* layers and could create grammar-import cycles.
* 
* Lifetime syntax must therefore remain independently parsable at the token
* level.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* This grammar contains:
* 
* - no semantic predicates;
* - no actions;
* - no embedded Rust;
* - no runtime calls;
* - no environment access;
* - no hardware discovery;
* - no target inspection.
* 
* Parsing is deterministic with respect to the canonical token stream.
* 
* ============================================================================
* SECURITY
* ============================================================================
* 
* Parsing a lifetime construct MUST NOT:
* 
* - allocate runtime memory;
* - inspect machine memory;
* - inspect hardware;
* - access files;
* - access a network;
* - execute user code;
* - invoke an allocator;
* - contact a QPU;
* - inspect runtime resources.
* 
* The grammar is a pure syntax layer.
* 
* ============================================================================
* COMPILER RESOURCE LIMITS
* ============================================================================
* 
* Very deeply nested or extremely large source programs may consume compiler
* resources.
* 
* That does NOT justify adding grammar-level artificial limits.
* 
* If protection is required, the compiler/frontend must expose explicit
* resource policies such as parser budgets or compilation budgets.
* 
* Such policies are:
* 
* implementation/resource policy
* 
* and NOT:
* 
* language semantic restrictions.
* 
* ============================================================================
* COMPATIBILITY
* ============================================================================
* 
* The lifetime grammar is intentionally open-ended with respect to lifetime
* names.
* 
* Existing forms:
* 
* 'a
* 'buffer
* 'scope
* 
* remain valid.
* 
* Future lifetime naming policies must be handled by the canonical lexer and
* identifier specification rather than by adding hard-coded names here.
* 
* ============================================================================
* PUBLIC RULES
* ============================================================================
* 
* The following rules constitute the public reusable API of this grammar:
* 
* lifetimeName
* anonymousLifetime
* lifetimeReference
* lifetimeAnnotation
* lifetimeParameter
* lifetimeParameterList
* lifetimeBound
* lifetimeBoundList
* lifetimeConstraint
* lifetimeConstraintList
* lifetimeWhereClause
* 
* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* POSITIVE TESTS
* 
* 'a
* 'buffer
* 'scope
* 'request_scope
* '_
* 
* 'a
* 'a: 'b
* 'buffer: 'scope
* 'a: 'b + 'c
* 
* <'a>
* <'a, 'b>
* <'request, 'resource, 'operation>
* 
* where 'a: 'b
* where 'a: 'b, 'b: 'c
* 
* ============================================================================
* NEGATIVE TESTS
* ============================================================================
* 
* The following must be rejected by the lifetime grammar:
* 
* '
* ' 
* 'a:
* : 'a
* 'a:
* <'a,
* <, 'a>
* where
* where 'a:
* where : 'a
* 
* ============================================================================
* BOUNDARY TESTS
* ============================================================================
* 
* Test:
* 
* - very long lifetime names;
* - many lifetime parameters;
* - many lifetime bounds;
* - deeply nested generic constructs supplied by the importing grammar;
* - large lifetime constraint lists.
* 
* Tests MUST NOT assume an arbitrary maximum lifetime count.
* 
* ============================================================================
* CROSS-DOMAIN TESTS
* ============================================================================
* 
* Lifetime syntax must be accepted when consumed by:
* 
* classical reference types;
* quantum resource types;
* hardware resource types;
* accelerator types;
* distributed handles;
* asynchronous APIs;
* concurrent APIs;
* memory-region abstractions.
* 
* Semantic validity is tested outside the grammar.
* 
* ============================================================================
* ROUND-TRIP TESTS
* ============================================================================
* 
* Where a canonical source printer exists:
* 
* source
*   -> lexer
*   -> parser
*   -> AST
*   -> printer
*   -> parser
* 
* must preserve the lifetime structure.
* 
* In particular:
* 
* 'a
* 'a: 'b
* <'a, 'b>
* 
* must not lose or reorder lifetime information.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* Forbidden:
* 
* MAX_LIFETIMES
* MAX_LIFETIME_PARAMETERS
* MAX_LIFETIME_BOUNDS
* MAX_LIFETIME_DEPTH
* MAX_REGIONS
* MAX_REFERENCES
* 
* Forbidden machine assumptions:
* 
* physical addresses;
* pointer widths;
* memory capacities;
* machine identifiers;
* device identifiers;
* CPU identifiers;
* GPU identifiers;
* QPU identifiers.
* 
* Lifetime names such as:
* 
* 'a
* 'b
* 
* are examples, not a finite vocabulary.
* 
* ============================================================================
* RUST CONTRACT
* ============================================================================
* 
* This file contains no Rust implementation.
* 
* Any generated Rust parser/frontend implementation must remain compatible
* with:
* 
* Rust 1.97
* Rust 1.97.1
* Edition 2021
* 
* and safe Rust only.
* 
* The repository-level Rust implementation should enforce:
* 
* #![forbid(unsafe_code)]
* 
* This grammar itself introduces no unsafe operation.
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when:
* 
* 1. It is a standalone ANTLR parser grammar.
* 
* 2. It consumes only canonical lexer tokens.
* 
* 3. It declares no lexer rules.
* 
* 4. It introduces no embedded Rust.
* 
* 5. Named lifetimes parse.
* 
* 6. Anonymous lifetimes parse.
* 
* 7. Lifetime annotations parse.
* 
* 8. Lifetime parameters parse.
* 
* 9. Arbitrary lifetime parameter counts are supported.
* 
* 10. Lifetime bounds parse.
* 
* 11. Multiple lifetime bounds parse.
* 
* 12. Where-style lifetime constraints parse.
* 
* 13. Arbitrary lifetime names are supported.
* 
* 14. No physical memory assumptions exist.
* 
* 15. No machine-size assumptions exist.
* 
* 16. No fixed lifetime-count limit exists.
* 
* 17. Lifetime semantics remain outside the parser.
* 
* 18. Reference types can consume the canonical lifetime annotation.
* 
* 19. Borrowing can consume the canonical lifetime syntax.
* 
* 20. Ownership can consume the canonical lifetime syntax.
* 
* 21. Memory grammar can consume the canonical lifetime syntax.
* 
* 22. The frontend AST can represent the resulting lifetime information.
* 
* 23. The grammar does not depend on quantum::ir.
* 
* 24. The grammar does not depend on QEC.
* 
* 25. The grammar does not depend on ZQN.
* 
* 26. The grammar does not depend on scheduling.
* 
* 27. The grammar does not depend on hardware discovery.
* 
* 28. The grammar does not depend on runtime execution.
* 
* 29. ANTLR generation succeeds using the canonical lexer.
* 
* 30. Positive, negative, boundary, cross-domain and round-trip tests exist.
* 
* ============================================================================
  */

parser grammar Lifetimes;

options {
tokenVocab = ZamaniLexer;
}

/* ============================================================================

* LIFETIME NAME
* ========================================================================== */

/**

* A named source-level lifetime.
* 
* Examples:
* 
* 'a
* 'buffer
* 'scope
* 'request_scope
* 
* The leading apostrophe is syntax. The identifier itself remains owned by
* the canonical lexer/name system.
  */
  lifetimeName
  : APOSTROPHE IDENT
  ;

/* ============================================================================

* ANONYMOUS LIFETIME
* ========================================================================== */

/**

* Anonymous lifetime.
* 
* Example:
* 
* '_
* 
* The semantic meaning of anonymity is determined downstream.
  */
  anonymousLifetime
  : APOSTROPHE UNDERSCORE
  ;

/* ============================================================================

* LIFETIME REFERENCE
* ========================================================================== */

/**

* Any source-level lifetime reference.
* 
* This is the preferred rule for consumers that accept either a named or
* anonymous lifetime.
  */
  lifetimeReference
  : lifetimeName
  | anonymousLifetime
  ;

/* ============================================================================

* LIFETIME ANNOTATION
* ========================================================================== */

/**

* Canonical lifetime annotation.
* 
* This rule exists as the stable integration point for reference types and
* borrowing grammars.
* 
* Examples:
* 
* &'a T
* &'buffer T
* 
* The surrounding '&' and referenced type are owned by the reference/type
* grammar, not this file.
  */
  lifetimeAnnotation
  : lifetimeName
  | anonymousLifetime
  ;

/* ============================================================================

* LIFETIME PARAMETER
* ========================================================================== */

/**

* A named lifetime parameter.
* 
* Lifetime parameters are intentionally represented independently from
* ordinary type parameters so semantic analysis can distinguish the two.
  */
  lifetimeParameter
  : lifetimeName
  ;

/* ============================================================================

* LIFETIME PARAMETER LIST
* ========================================================================== */

/**

* Generic lifetime parameter list.
* 
* Examples:
* 
* <'a>
* <'a, 'b>
* <'request, 'resource, 'operation>
* 
* There is intentionally no fixed parameter count.
  /
  lifetimeParameterList
  : LT lifetimeParameter (COMMA lifetimeParameter) COMMA? GT
  ;

/* ============================================================================

* LIFETIME BOUND
* ========================================================================== */

/**

* A source-level relationship between two lifetime names.
* 
* Canonical form:
* 
* 'a: 'b
* 
* The grammar records the relationship only.
* 
* Semantic analysis determines whether it represents a valid outlives
* relationship in the relevant language context.
* 
* Anonymous lifetimes are deliberately not accepted here because a bound
* requires named semantic identities on both sides.
  */
  lifetimeBound
  : lifetimeName
  COLON
  lifetimeName
  ;

/* ============================================================================

* LIFETIME BOUND LIST
* ========================================================================== */

/**

* One or more lifetime bounds.
* 
* Example:
* 
* 'a: 'b + 'c
* 
* The PLUS token is reused from the canonical language vocabulary.
  /
  lifetimeBoundList
  : lifetimeName
  COLON
  lifetimeName
  (PLUS lifetimeName)
  ;

/* ============================================================================

* LIFETIME CONSTRAINT
* ========================================================================== */

/**

* A single lifetime constraint.
* 
* This rule is intentionally distinct from a lifetime bound list so importing
* grammars can choose whether they need one relation or a grouped relation.
  */
  lifetimeConstraint
  : lifetimeBound
  ;

/* ============================================================================

* LIFETIME CONSTRAINT LIST
* ========================================================================== */

/**

* Ordered lifetime constraint list.
* 
* Example:
* 
* 'a: 'b, 'b: 'c
* 
* No fixed number of constraints is imposed.
  /
  lifetimeConstraintList
  : lifetimeConstraint
  (COMMA lifetimeConstraint)
  COMMA?
  ;

/* ============================================================================

* WHERE-STYLE LIFETIME CONSTRAINTS
* ========================================================================== */

/**

* Lifetime constraints introduced by the canonical "where" keyword.
* 
* Examples:
* 
* where 'a: 'b
* 
* where 'a: 'b, 'b: 'c
* 
* The complete generic/type "where" system remains owned by the type/function
* grammar. This rule owns only the lifetime-specific portion.
  */
  lifetimeWhereClause
  : WHERE
  lifetimeConstraintList
  ;