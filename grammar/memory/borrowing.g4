/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/borrowing.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Status:
 *     Production borrowing-domain grammar component.
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file defines the SOURCE-LEVEL BORROWING SYNTAX CONTRACT for Zamani.
 *
 * Borrowing is intentionally separated from:
 *
 *     - reference-type syntax;
 *     - general unary-expression syntax;
 *     - ownership semantics;
 *     - lifetime semantics;
 *     - allocation;
 *     - deallocation;
 *     - physical memory;
 *     - hardware resources.
 *
 * This distinction is essential because the repository already has:
 *
 *     grammar/types/reference-types.g4
 *
 * as the owner of:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *
 * and:
 *
 *     grammar/expressions/unary.g4
 *
 * as the owner of:
 *
 *     &expression
 *
 * Consequently this file MUST NOT redefine those constructs.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     parser
 *       |
 *       +-------------------------------+
 *       |                               |
 *       v                               v
 * reference-types.g4                unary.g4
 *       |                               |
 *       +---------------+---------------+
 *                       |
 *                       v
 *                  borrowing.g4
 *                       |
 *                       v
 *                     AST
 *                       |
 *                       v
 *             structural validation
 *                       |
 *                       v
 *              semantic analysis
 *                       |
 *          +------------+-------------+
 *          |            |             |
 *          v            v             v
 *      ownership     lifetime      alias/borrow
 *       analysis      analysis       analysis
 *          |            |             |
 *          +------------+-------------+
 *                       |
 *                       v
 *               canonical semantic IR
 *                       |
 *          +------------+-------------+
 *          |            |             |
 *          v            v             v
 *      classical     quantum       hardware/
 *         IR           IR          resource IR
 *                       |
 *                       v
 *          optimization / scheduling /
 *          routing / lowering / runtime
 *
 * The dependency direction MUST remain downstream.
 *
 * This grammar does not depend on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     scheduling
 *     routing
 *     optimization
 *     hardware discovery
 *     runtime
 *     backend selection
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 * 1. Borrow-specific source-level annotations.
 *
 * 2. Borrow mode classification syntax.
 *
 * 3. Borrow requirements expressed as source metadata.
 *
 * 4. Borrow constraints expressed as source metadata.
 *
 * 5. Borrow preferences expressed as source metadata.
 *
 * 6. Borrow hints expressed as source metadata.
 *
 * 7. Borrow clauses that can be attached to declarations/contracts where the
 *    surrounding grammar explicitly permits memory-domain clauses.
 *
 * 8. Borrow-domain extension points.
 *
 * 9. The parser-level distinction between:
 *
 *        immutable borrow intent
 *        mutable borrow intent
 *
 *    without deciding whether such a borrow is semantically legal.
 *
 * ============================================================================
 * THIS FILE DOES NOT OWN
 * ============================================================================
 *
 * This file MUST NOT own:
 *
 *     - lexer rules;
 *     - identifier syntax;
 *     - Unicode rules;
 *     - name resolution;
 *     - reference-type syntax;
 *     - pointer-type syntax;
 *     - unary-expression syntax;
 *     - general expression syntax;
 *     - general statement syntax;
 *     - ownership checking;
 *     - move checking;
 *     - copy checking;
 *     - lifetime inference;
 *     - lifetime checking;
 *     - alias analysis;
 *     - data-race analysis;
 *     - allocation;
 *     - deallocation;
 *     - memory layout;
 *     - pointer width;
 *     - address representation;
 *     - physical memory;
 *     - cache hierarchy;
 *     - NUMA topology;
 *     - device memory;
 *     - GPU memory;
 *     - QPU memory;
 *     - distributed placement;
 *     - synchronization implementation;
 *     - resource allocation;
 *     - hardware selection;
 *     - backend selection;
 *     - quantum routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime execution;
 *     - classical IR;
 *     - quantum::ir;
 *     - hardware IR.
 *
 * ============================================================================
 * CRITICAL OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * The repository already establishes two distinct syntactic uses of '&'.
 *
 * TYPE SYNTAX
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *
 * is owned by:
 *
 *     grammar/types/reference-types.g4
 *
 * EXPRESSION SYNTAX
 *
 *     &value
 *     &mut_value
 *
 * where syntactically applicable is owned by:
 *
 *     grammar/expressions/unary.g4
 *
 * This file MUST NOT define:
 *
 *     referenceType
 *     unaryExpression
 *     AMPERSAND expression
 *
 * again.
 *
 * Doing so would create competing parse-tree owners and make semantic
 * ownership ambiguous.
 *
 * ============================================================================
 * BORROWING MODEL
 * ============================================================================
 *
 * Borrowing is a semantic relationship involving access to a value without
 * necessarily transferring ownership.
 *
 * The grammar preserves source intent only.
 *
 * It does NOT determine:
 *
 *     whether ownership was transferred;
 *     whether a borrow is valid;
 *     whether aliases overlap;
 *     whether a mutable borrow is exclusive;
 *     whether an immutable borrow conflicts with mutation;
 *     whether a lifetime is sufficient;
 *     whether a borrow crosses a task boundary;
 *     whether a borrow crosses a process boundary;
 *     whether a borrow crosses a distributed boundary;
 *     whether a borrow is representable on a target;
 *     whether a borrow requires copying;
 *     whether a borrow requires synchronization.
 *
 * Those decisions belong to semantic analysis.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Borrow syntax expresses portable source-level intent.
 *
 * It MUST NOT encode:
 *
 *     MAX_BORROWS
 *     MAX_SHARED_BORROWS
 *     MAX_MUTABLE_BORROWS
 *     MAX_REFERENCE_DEPTH
 *     MAX_LIFETIME_DEPTH
 *     MAX_ALIAS_COUNT
 *     MAX_MEMORY_SIZE
 *     MAX_POINTER_WIDTH
 *     MAX_ADDRESS_WIDTH
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *
 * Nor may it encode:
 *
 *     physical addresses;
 *     memory-bank identifiers;
 *     NUMA-node identifiers;
 *     GPU addresses;
 *     QPU identifiers;
 *     fixed machine topology;
 *     fixed memory capacity.
 *
 * Any implementation resource limit MUST be represented as an explicit
 * compiler, semantic-analysis, resource, or runtime policy.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * Borrowing may apply to values belonging to any supported computational
 * domain, including:
 *
 *     classical values
 *     vectors
 *     matrices
 *     tensors
 *     symbolic values
 *     quantum values
 *     logical quantum resources
 *     hardware resources
 *     accelerator resources
 *     HDL values
 *     distributed objects
 *     data objects
 *     future domain values
 *
 * This grammar does not enumerate those domains.
 *
 * A new computational domain MUST NOT require a new borrowing grammar merely
 * because its values can participate in the language's existing reference and
 * borrowing semantics.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes tokens from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The canonical lexer already provides the relevant lexical infrastructure,
 * including tokens such as:
 *
 *     IDENT
 *     MUT
 *     AMPERSAND
 *     APOSTROPHE
 *     AT
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     COLON
 *     SEMICOLON
 *     DOUBLE_COLON
 *
 * This file MUST NOT declare lexer rules.
 *
 * In particular it MUST NOT introduce local alternatives for:
 *
 *     identifier
 *     lifetime identifier
 *     &
 *     mut
 *     punctuation
 *
 * ============================================================================
 * KEYWORD POLICY
 * ============================================================================
 *
 * The canonical lexer does not currently establish a dedicated BORROW
 * keyword.
 *
 * This file therefore deliberately does NOT invent:
 *
 *     BORROW
 *     BORROW_MUT
 *     SHARED_BORROW
 *     EXCLUSIVE_BORROW
 *
 * lexer tokens.
 *
 * This is important for compatibility with the open-world language design.
 *
 * Borrowing remains representable through:
 *
 *     existing reference syntax
 *     existing unary syntax
 *     explicit attributes/annotations
 *     memory-domain clauses
 *
 * without forcing a new globally reserved keyword.
 *
 * ============================================================================
 * ANNOTATION MODEL
 * ============================================================================
 *
 * Borrow-specific metadata uses the existing attribute mechanism rather than
 * creating a second annotation syntax.
 *
 * Conceptually:
 *
 *     @borrow
 *     @borrow(...)
 *
 * The exact attribute spelling is intentionally resolved by the canonical
 * attribute grammar and semantic annotation registry.
 *
 * This grammar therefore defines only the reusable borrowing payload.
 *
 * ============================================================================
 * BORROW MODE
 * ============================================================================
 *
 * A borrow has one of the source-level modes:
 *
 *     immutable
 *     mutable
 *
 * This distinction is syntactic intent.
 *
 * It does NOT establish semantic validity.
 *
 * IMMUTABLE:
 *
 *     The borrower requests non-mutating access.
 *
 * MUTABLE:
 *
 *     The borrower requests mutating access.
 *
 * Whether a mutable borrow is legal depends on ownership, aliasing, lifetime,
 * concurrency and effect analysis.
 *
 * ============================================================================
 * PUBLIC BORROWING RULES
 * ============================================================================
 *
 * The rules exported by this component are deliberately narrow.
 *
 * The primary reusable rules are:
 *
 *     borrowMode
 *     borrowModeModifier
 *     borrowLifetime
 *     borrowRequirement
 *     borrowConstraint
 *     borrowPreference
 *     borrowHint
 *     borrowMetadata
 *     borrowClause
 *     borrowExtension
 *
 * These rules are designed to be imported by memory.g4 or by a higher-level
 * parser component that explicitly permits borrowing clauses.
 *
 * ============================================================================
 * BORROW MODE MODIFIER
 * ============================================================================
 *
 * The source-level distinction is represented with existing lexical tokens:
 *
 *     MUT
 *
 * for mutable intent.
 *
 * The absence of MUT represents immutable intent where the surrounding
 * borrowing construct establishes a borrow context.
 *
 * No new lexer token is necessary.
 *
 * ============================================================================
 * LIFETIME INTEGRATION
 * ============================================================================
 *
 * Lifetime syntax is owned by the canonical type/lifetime grammar.
 *
 * Where a borrowing clause needs to preserve an explicit lifetime, this file
 * uses:
 *
 *     APOSTROPHE identifier
 *
 * only as a syntactic bridge.
 *
 * It does NOT perform lifetime analysis.
 *
 * It does NOT determine:
 *
 *     lifetime start;
 *     lifetime end;
 *     outlives relationships;
 *     scope nesting;
 *     region inference.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * A borrowing annotation may identify a source-level target using a qualified
 * name or expression bridge supplied by the surrounding parser.
 *
 * This component deliberately does not redefine `expression`.
 *
 * The host parser is responsible for providing the canonical expression rule.
 *
 * Consequently this file can be imported by:
 *
 *     memory.g4
 *
 * without creating a second expression grammar.
 *
 * ============================================================================
 * PLACE INTEGRATION
 * ============================================================================
 *
 * A semantic borrow normally applies to a source-level place.
 *
 * Examples include:
 *
 *     x
 *     object.field
 *     values[index]
 *     object.field[index]
 *
 * The complete place-expression syntax belongs to the canonical expression
 * grammar.
 *
 * This file therefore uses a deliberately conservative `borrowTarget` bridge
 * based on canonical names.
 *
 * The richer expression target is attached by the host expression parser when
 * the borrowing construct occurs in expression position.
 *
 * This prevents this grammar from becoming a duplicate expression grammar.
 *
 * ============================================================================
 * BORROW REQUIREMENTS
 * ============================================================================
 *
 * A borrow requirement expresses a semantic requirement that downstream
 * analysis MUST satisfy.
 *
 * It does not identify a physical implementation.
 *
 * The grammar supports:
 *
 *     borrowRequirement
 *
 * with:
 *
 *     borrow mode
 *     optional lifetime
 *     source-level target
 *
 * Semantic analysis determines whether it is satisfiable.
 *
 * ============================================================================
 * BORROW CONSTRAINTS
 * ============================================================================
 *
 * A constraint restricts legal implementation or semantic choices.
 *
 * A constraint is not the same thing as a requirement.
 *
 * This distinction is important for POCO-REAF.
 *
 * Requirements:
 *
 *     must hold.
 *
 * Constraints:
 *
 *     restrict the permitted implementation/semantic space.
 *
 * Preferences:
 *
 *     express desirable behavior.
 *
 * Hints:
 *
 *     provide optional guidance.
 *
 * ============================================================================
 * BORROW PREFERENCES
 * ============================================================================
 *
 * Preferences MUST NOT be interpreted as semantic guarantees.
 *
 * They may guide downstream optimization or lowering.
 *
 * Example conceptual forms:
 *
 *     prefer immutable
 *     prefer local
 *
 * are not hard-coded by this grammar as machine policies.
 *
 * The actual vocabulary remains extensible through qualified names.
 *
 * ============================================================================
 * BORROW HINTS
 * ============================================================================
 *
 * Hints are non-binding implementation guidance.
 *
 * A backend is permitted to ignore them when necessary.
 *
 * The grammar does not define backend behavior.
 *
 * ============================================================================
 * BORROW EXTENSIONS
 * ============================================================================
 *
 * Future borrowing systems may require concepts such as:
 *
 *     capability-scoped borrowing
 *     region borrowing
 *     transactional borrowing
 *     distributed borrowing
 *     accelerator borrowing
 *     asynchronous borrowing
 *     persistent borrowing
 *     shared-resource borrowing
 *
 * The grammar must remain open to such extensions.
 *
 * They should be represented by qualified names and metadata rather than
 * modifying the foundational borrowing model for every future domain.
 *
 * ============================================================================
 * MEMORY INTEGRATION
 * ============================================================================
 *
 * `memory.g4` is the domain aggregator.
 *
 * It should import/use this component and expose borrowing syntax where the
 * memory domain is permitted.
 *
 * The dependency direction is:
 *
 *     memory.g4
 *          |
 *          +--> borrowing.g4
 *
 * not:
 *
 *     borrowing.g4 --> memory.g4
 *
 * This prevents a cycle.
 *
 * ============================================================================
 * OWNERSHIP INTEGRATION
 * ============================================================================
 *
 * Ownership and borrowing are related but distinct.
 *
 * ownership.g4 owns ownership intent.
 *
 * borrowing.g4 owns borrowing intent.
 *
 * Semantic analysis combines them.
 *
 * This grammar MUST NOT:
 *
 *     move ownership rules here;
 *     define ownership transfer;
 *     define copy semantics;
 *     define drop semantics.
 *
 * ============================================================================
 * LIFETIME INTEGRATION
 * ============================================================================
 *
 * lifetimes.g4 owns lifetime-domain syntax where applicable.
 *
 * borrowing.g4 may preserve an explicit lifetime reference but does not own
 * lifetime semantics.
 *
 * Semantic analysis combines:
 *
 *     borrow
 *     ownership
 *     lifetime
 *
 * into the canonical memory semantic model.
 *
 * ============================================================================
 * ALLOCATION INTEGRATION
 * ============================================================================
 *
 * Borrowing does not allocate memory.
 *
 * This file has no dependency on:
 *
 *     allocation.g4
 *     deallocation.g4
 *
 * A backend may implement a reference using a representation that requires
 * allocation, copying, pinning, registration, or another mechanism, but that
 * is a downstream implementation decision.
 *
 * ============================================================================
 * SHARED MEMORY INTEGRATION
 * ============================================================================
 *
 * Shared-memory semantics belong to:
 *
 *     shared-memory.g4
 *
 * A borrow may eventually participate in shared-memory semantics, but this
 * file does not define:
 *
 *     cache coherence;
 *     synchronization;
 *     atomicity;
 *     memory barriers;
 *     NUMA placement.
 *
 * ============================================================================
 * DISTRIBUTED MEMORY INTEGRATION
 * ============================================================================
 *
 * A source-level borrow may refer to a value whose eventual realization is
 * local, remote, replicated, migrated, or otherwise distributed.
 *
 * This grammar does not decide which.
 *
 * Distributed-memory semantics belong downstream.
 *
 * A distributed borrow may therefore be rejected semantically if the selected
 * execution model cannot preserve the required semantics, rather than being
 * made syntactically impossible here.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Borrowing syntax remains domain-neutral.
 *
 * A quantum type may participate in the same source-level reference/borrowing
 * model if the quantum semantic system permits it.
 *
 * This grammar MUST NOT introduce:
 *
 *     qubit ownership;
 *     physical-qubit borrowing;
 *     QPU borrowing;
 *     physical-qubit addresses;
 *     topology-specific references.
 *
 * Those concepts, where semantically meaningful, are resolved downstream.
 *
 * In particular, this grammar MUST NOT depend on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     hardware
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Borrowing may be meaningful for source-level hardware resources or HDL
 * values, but this grammar does not decide their physical realization.
 *
 * It MUST NOT encode:
 *
 *     bus widths;
 *     register counts;
 *     physical addresses;
 *     FPGA resources;
 *     ASIC cells;
 *     clock frequencies;
 *     hardware IDs.
 *
 * Hardware semantics remain downstream.
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * A borrow may cross a concurrency boundary only when the semantic model
 * permits it.
 *
 * This grammar does not decide:
 *
 *     Send/Sync-like properties;
 *     data-race freedom;
 *     synchronization;
 *     task ownership;
 *     actor ownership;
 *     channel ownership.
 *
 * Those belong to semantic/effect/concurrency analysis.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Borrowing does not directly allocate resources.
 *
 * If a borrow imposes resource requirements, those requirements are represented
 * downstream in the resource model.
 *
 * The grammar itself contains no:
 *
 *     resource count;
 *     memory capacity;
 *     device count;
 *     node count;
 *     thread count.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser does not define the AST implementation.
 *
 * The existing frontend AST should preserve enough information to represent:
 *
 *     borrow mode
 *     optional explicit lifetime
 *     optional source-level target
 *     source span
 *     attributes/metadata
 *     provenance
 *
 * Conceptually:
 *
 *     BorrowIntent {
 *         mode: Immutable | Mutable,
 *         lifetime: Optional<LifetimeName>,
 *         target: Optional<SourceTarget>,
 *         metadata: ...
 *     }
 *
 * This is illustrative only.
 *
 * This grammar MUST NOT introduce a competing Rust AST type.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     ownership validity
 *     alias validity
 *     mutable-alias validity
 *     lifetime validity
 *     borrow scope
 *     borrow escape analysis
 *     concurrency validity
 *     resource validity
 *     cross-domain validity
 *     target compatibility
 *
 * A syntactically valid borrow is therefore NOT necessarily semantically
 * valid.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * borrowing.g4 has NO direct IR dependency.
 *
 * It does not construct:
 *
 *     classical IR operations
 *     quantum::ir operations
 *     hardware IR
 *     scheduling IR
 *     routing IR
 *     runtime commands
 *
 * Semantic analysis lowers borrow information into the appropriate canonical
 * semantic representation.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler/frontend pipeline consuming this grammar must support:
 *
 *     lexing
 *     parsing
 *     AST construction
 *     name resolution
 *     type resolution
 *     ownership analysis
 *     borrow analysis
 *     lifetime analysis
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     canonical semantic lowering
 *
 * A new target/backend MUST NOT require this grammar to be rewritten merely
 * because the target uses a different representation for references.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * This grammar has no runtime dependency.
 *
 * Runtime realization may use:
 *
 *     native references
 *     handles
 *     descriptors
 *     indices
 *     capabilities
 *     managed references
 *     target-specific representations
 *
 * provided the semantic contract is preserved.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * IDEs, formatters, linters, language servers and refactoring tools should
 * consume the parser structure and canonical AST rather than independently
 * reparsing borrowing syntax.
 *
 * Tooling must preserve:
 *
 *     borrow mode
 *     lifetime metadata
 *     source span
 *     annotations
 *     provenance
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic predicates;
 *     no embedded Rust;
 *     no runtime calls;
 *     no hardware discovery;
 *     no environment inspection;
 *     no target selection.
 *
 * Parsing the same source with the same language version and canonical lexer
 * must produce the same syntactic structure.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing borrowing syntax MUST NOT:
 *
 *     allocate runtime memory;
 *     deallocate runtime memory;
 *     dereference addresses;
 *     access files;
 *     access hardware;
 *     access quantum hardware;
 *     inspect devices;
 *     contact networks;
 *     execute code.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing reference syntax remains owned by:
 *
 *     grammar/types/reference-types.g4
 *
 * Existing unary address-of syntax remains owned by:
 *
 *     grammar/expressions/unary.g4
 *
 * This file must not silently change their meaning.
 *
 * New borrowing metadata must be introduced through explicit versioned
 * language changes where required.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar deliberately uses recursive/open structures where appropriate.
 *
 * It contains no artificial limit on:
 *
 *     borrow count
 *     lifetime count
 *     target-name length
 *     metadata count
 *     nesting depth
 *     program size
 *
 * Practical parser/compiler resource limits remain implementation policies.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden language-level hard-coding includes:
 *
 *     MAX_BORROWS
 *     MAX_SHARED_BORROWS
 *     MAX_MUTABLE_BORROWS
 *     MAX_REFERENCE_DEPTH
 *     MAX_LIFETIMES
 *     MAX_MEMORY
 *     MAX_POINTER_WIDTH
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *
 * Any such value found in this file is an architectural defect unless it is
 * explicitly test-only documentation.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 */

/*
 * --------------------------------------------------------------------------
 * Borrow mode
 * --------------------------------------------------------------------------
 *
 * The absence of MUT means immutable borrow intent.
 *
 * MUT means mutable borrow intent.
 *
 * This rule deliberately does not use a new BORROW keyword.
 */
borrowMode
    : MUT
    | /* empty: immutable borrow intent */
    ;


/*
 * --------------------------------------------------------------------------
 * Explicit borrow-mode modifier
 * --------------------------------------------------------------------------
 *
 * Useful when a host grammar wants to distinguish the presence of a mutable
 * marker from its absence without assigning semantic meaning.
 */
borrowModeModifier
    : MUT
    ;


/*
 * --------------------------------------------------------------------------
 * Borrow lifetime
 * --------------------------------------------------------------------------
 *
 * This is a source-level lifetime reference.
 *
 * It intentionally mirrors the canonical lifetime spelling:
 *
 *     'a
 *     'buffer
 *     'scope
 *
 * Lifetime semantics belong downstream.
 */
borrowLifetime
    : APOSTROPHE IDENT
    ;


/*
 * --------------------------------------------------------------------------
 * Borrow target
 * --------------------------------------------------------------------------
 *
 * This is deliberately a name-oriented bridge.
 *
 * Complete expression/place syntax remains owned by the expression grammar.
 *
 * The target may be:
 *
 *     x
 *     object
 *     namespace::object
 *
 * More complex expression targets are supplied by the host expression
 * grammar when borrowing appears in expression position.
 */
borrowTarget
    : IDENT
    | qualifiedBorrowTarget
    ;

qualifiedBorrowTarget
    : IDENT
      (DOUBLE_COLON IDENT)+
    ;


/*
 * --------------------------------------------------------------------------
 * Borrow requirement
 * --------------------------------------------------------------------------
 *
 * A requirement expresses a condition that semantic analysis must satisfy.
 *
 * Example conceptual structure:
 *
 *     borrow requirement
 *     mutable borrow requirement
 *     borrow requirement with lifetime
 *
 * The surrounding memory grammar supplies the syntactic context.
 */
borrowRequirement
    : borrowModeModifier?
      borrowTarget
      borrowLifetime?
    ;


/*
 * --------------------------------------------------------------------------
 * Borrow constraint
 * --------------------------------------------------------------------------
 *
 * A constraint is a source-level restriction.
 *
 * The constraint name remains open-world.
 */
borrowConstraint
    : borrowName
      borrowConstraintArguments?
    ;

borrowConstraintArguments
    : LPAREN borrowArgumentList? RPAREN
    ;

borrowArgumentList
    : borrowArgument
      (COMMA borrowArgument)*
      COMMA?
    ;

borrowArgument
    : borrowName
    | borrowLifetime
    | borrowTarget
    ;


/*
 * --------------------------------------------------------------------------
 * Borrow preference
 * --------------------------------------------------------------------------
 *
 * Preferences are non-binding.
 */
borrowPreference
    : borrowName
      borrowPreferenceArguments?
    ;

borrowPreferenceArguments
    : LPAREN borrowArgumentList? RPAREN
    ;


/*
 * --------------------------------------------------------------------------
 * Borrow hint
 * --------------------------------------------------------------------------
 *
 * Hints are optional implementation guidance.
 */
borrowHint
    : borrowName
      borrowHintArguments?
    ;

borrowHintArguments
    : LPAREN borrowArgumentList? RPAREN
    ;


/*
 * --------------------------------------------------------------------------
 * Borrow metadata
 * --------------------------------------------------------------------------
 *
 * Generic metadata gives future borrowing systems an extension point without
 * requiring every new borrowing concept to become a globally reserved word.
 */
borrowMetadata
    : borrowName
      (ASSIGN borrowArgument)?
    ;


/*
 * --------------------------------------------------------------------------
 * Borrow name
 * --------------------------------------------------------------------------
 *
 * Open-world qualified naming.
 *
 * Examples:
 *
 *     shared
 *     exclusive
 *     region::scoped
 *     capability::restricted
 *     distributed::remote
 *
 * These are names only. Semantic interpretation belongs downstream.
 */
borrowName
    : IDENT
      (DOUBLE_COLON IDENT)*
    ;


/*
 * --------------------------------------------------------------------------
 * Borrow clause
 * --------------------------------------------------------------------------
 *
 * This is the primary reusable borrowing clause.
 *
 * It does not define the location in which the clause may appear.
 *
 * The host grammar owns that context.
 */
borrowClause
    : borrowModeModifier?
      borrowLifetime?
      borrowTarget
    ;


/*
 * --------------------------------------------------------------------------
 * Borrow extension
 * --------------------------------------------------------------------------
 *
 * Future borrowing-domain syntax can be represented through an extensible
 * qualified name plus arguments.
 */
borrowExtension
    : borrowName
      borrowExtensionArguments?
    ;

borrowExtensionArguments
    : LPAREN borrowArgumentList? RPAREN
    ;


/*
 * --------------------------------------------------------------------------
 * Borrow declaration metadata
 * --------------------------------------------------------------------------
 *
 * This rule groups reusable borrowing metadata without defining a declaration
 * grammar of its own.
 */
borrowMetadataList
    : borrowMetadata
      (COMMA borrowMetadata)*
      COMMA?
    ;


/*
 * --------------------------------------------------------------------------
 * Borrow policy fragment
 * --------------------------------------------------------------------------
 *
 * This is deliberately a fragment rather than a top-level statement.
 *
 * Policy interpretation belongs to semantic analysis.
 */
borrowPolicy
    : borrowName
      (LPAREN borrowArgumentList? RPAREN)?
    ;


/*
 * --------------------------------------------------------------------------
 * Borrow policy list
 * --------------------------------------------------------------------------
 */
borrowPolicyList
    : borrowPolicy
      (COMMA borrowPolicy)*
      COMMA?
    ;


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * memory.g4
 * ----------
 *
 * memory.g4 should import this grammar and expose the rules required by the
 * memory-domain aggregate.
 *
 * It remains responsible for deciding where borrowing clauses may occur.
 *
 *
 * types/reference-types.g4
 * ------------------------
 *
 * Owns:
 *
 *     referenceType
 *
 * including:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *
 * borrowing.g4 MUST NOT redefine referenceType.
 *
 *
 * expressions/unary.g4
 * --------------------
 *
 * Owns:
 *
 *     unaryExpression
 *
 * including the syntactic address-of/reference operator:
 *
 *     &expression
 *
 * borrowing.g4 MUST NOT redefine unaryExpression.
 *
 *
 * lifetimes.g4
 * ------------
 *
 * Owns lifetime-domain semantics and any canonical lifetime grammar that is
 * established there.
 *
 * borrowing.g4 only preserves an explicit lifetime reference where a borrow
 * clause requires one.
 *
 *
 * ownership.g4
 * ------------
 *
 * Owns ownership-specific syntax.
 *
 * Semantic analysis combines ownership and borrowing.
 *
 *
 * allocation.g4
 * -------------
 *
 * Owns allocation syntax.
 *
 * borrowing.g4 does not allocate.
 *
 *
 * deallocation.g4
 * ---------------
 *
 * Owns deallocation syntax.
 *
 * borrowing.g4 does not deallocate.
 *
 *
 * shared-memory.g4
 * ----------------
 *
 * Owns shared-memory-specific syntax.
 *
 * borrowing.g4 does not define cache coherence or synchronization.
 *
 *
 * distributed-memory.g4
 * ---------------------
 *
 * Owns distributed-memory-specific syntax.
 *
 * borrowing.g4 does not define remote references or placement.
 *
 * ============================================================================
 * ANTLR INTEGRATION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It requires:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It should be imported by the parser assembly that owns the memory domain.
 *
 * It MUST NOT become a standalone lexer.
 *
 * ============================================================================
 * IMPORT DIRECTION
 * ============================================================================
 *
 * Correct:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     borrowing.g4
 *          |
 *          v
 *     memory.g4 / parser assembly
 *          |
 *          v
 *         AST
 *
 * Incorrect:
 *
 *     borrowing.g4 -> runtime
 *     borrowing.g4 -> quantum::ir
 *     borrowing.g4 -> hardware
 *     borrowing.g4 -> scheduler
 *     borrowing.g4 -> ZQN
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * The grammar must allow the parser/frontend diagnostic layer to distinguish
 * structural failures such as:
 *
 *     missing borrow target
 *     malformed lifetime
 *     malformed qualified name
 *     malformed borrow arguments
 *
 * from semantic failures such as:
 *
 *     illegal mutable alias
 *     invalid lifetime
 *     ownership violation
 *     invalid cross-task borrow
 *     invalid distributed borrow
 *     unsupported target realization
 *
 * Semantic failures MUST NOT be encoded as parser failures merely because
 * they depend on target or program analysis.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * These tests apply to this component's rules independently of the host
 * expression/declaration grammar.
 *
 * --------------------------------------------------------------------------
 * Positive
 * --------------------------------------------------------------------------
 *
 * borrowMode:
 *
 *     <empty>
 *     mut
 *
 * borrowLifetime:
 *
 *     'a
 *     'buffer
 *     'scope
 *
 * borrowTarget:
 *
 *     x
 *     value
 *     memory::value
 *     distributed::buffer
 *
 * borrowClause:
 *
 *     x
 *     mut x
 *     'a x
 *     mut 'a x
 *
 * borrowRequirement:
 *
 *     x
 *     mut x
 *     x 'a
 *     mut x 'a
 *
 * borrowConstraint:
 *
 *     exclusive
 *     region::scoped
 *     capability::restricted(arg)
 *
 * borrowPreference:
 *
 *     immutable
 *     locality::preferred
 *
 * borrowHint:
 *
 *     optimizer::coalesce
 *     runtime::retain
 *
 * borrowExtension:
 *
 *     future::borrow
 *     future::borrow(argument)
 *
 * --------------------------------------------------------------------------
 * Negative
 * --------------------------------------------------------------------------
 *
 * Invalid:
 *
 *     &
 *     mut
 *     '
 *     ' mut
 *     ::x
 *     x::
 *     x:::
 *
 * malformed lifetime and malformed qualified names must be rejected by the
 * parser.
 *
 * --------------------------------------------------------------------------
 * Boundary
 * --------------------------------------------------------------------------
 *
 * Tests must include:
 *
 *     arbitrarily nested reference types;
 *     arbitrarily long qualified names;
 *     arbitrarily many metadata entries;
 *     arbitrarily many borrow clauses;
 *     deeply nested generic types used by the host type grammar;
 *     large source programs.
 *
 * Tests MUST NOT introduce artificial language-level maxima.
 *
 * --------------------------------------------------------------------------
 * Cross-domain
 * --------------------------------------------------------------------------
 *
 * Borrowing syntax must remain usable when the target eventually resolves to:
 *
 *     classical::Value
 *     quantum::State
 *     hardware::Resource
 *     accelerator::Buffer
 *     distributed::Object
 *     data::Record
 *     future::Value
 *
 * The borrowing grammar itself does not need domain-specific rules for these.
 *
 * --------------------------------------------------------------------------
 * Determinism
 * --------------------------------------------------------------------------
 *
 * Identical source + identical language version + identical lexer must produce
 * identical parse structure.
 *
 * --------------------------------------------------------------------------
 * Round-trip
 * --------------------------------------------------------------------------
 *
 * Parser/printer infrastructure should preserve:
 *
 *     borrow mode
 *     lifetime
 *     target
 *     qualified names
 *     metadata
 *     extension arguments
 *
 * without changing semantic intent.
 *
 * ============================================================================
 * HARD-CODING TESTS
 * ============================================================================
 *
 * Automated repository checks should fail if this grammar introduces:
 *
 *     MAX_BORROWS
 *     MAX_REFERENCE_DEPTH
 *     MAX_LIFETIME_COUNT
 *     MAX_ALIAS_COUNT
 *     MAX_MEMORY
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *
 * or equivalent machine-dependent constants.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when ALL of the following are true:
 *
 * 1. It is a valid ANTLR4 parser grammar.
 *
 * 2. It consumes the canonical ZamaniLexer vocabulary.
 *
 * 3. It contains no lexer rules.
 *
 * 4. It does not redefine `referenceType`.
 *
 * 5. It does not redefine `unaryExpression`.
 *
 * 6. It does not redefine `expression`.
 *
 * 7. It does not define a competing ownership model.
 *
 * 8. It does not define lifetime semantics.
 *
 * 9. It exposes reusable borrowing-domain rules.
 *
 * 10. Mutable and immutable borrow intent can be represented.
 *
 * 11. Explicit lifetime names can be preserved.
 *
 * 12. Qualified source-level targets can be represented.
 *
 * 13. Borrow requirements, constraints, preferences and hints remain distinct.
 *
 * 14. Future borrowing concepts can be represented without a closed
 *     enumeration of technologies.
 *
 * 15. No physical memory assumptions exist.
 *
 * 16. No hardware topology assumptions exist.
 *
 * 17. No quantum machine assumptions exist.
 *
 * 18. No fixed resource limits exist.
 *
 * 19. No runtime behavior is executed by parsing.
 *
 * 20. No embedded Rust exists.
 *
 * 21. No unsafe Rust requirement exists.
 *
 * 22. The grammar remains compatible with Rust 1.97 / 1.97.1 through the
 *     generated frontend/compiler integration.
 *
 * 23. Semantic borrow checking remains downstream.
 *
 * 24. Ownership, lifetime, concurrency and resource analysis remain separate
 *     semantic stages.
 *
 * 25. `memory.g4` can consume this component without creating a dependency
 *     cycle.
 *
 * 26. Existing `reference-types.g4` and `unary.g4` remain the sole syntactic
 *     owners of their respective reference/address-of forms.
 *
 * 27. Tests cover positive, negative, boundary, cross-domain, deterministic
 *     and round-trip behavior.
 *
 * ============================================================================
 */
 
parser grammar Borrowing;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * BORROW MODE
 * ============================================================================
 *
 * Empty alternative = immutable borrow intent.
 *
 * MUT = mutable borrow intent.
 *
 * The semantic layer determines whether the resulting intent is legal.
 */
borrowMode
    : MUT
    | /* empty */
    ;


/*
 * Explicit mutable marker.
 */
borrowModeModifier
    : MUT
    ;


/*
 * ============================================================================
 * LIFETIME
 * ============================================================================
 */
borrowLifetime
    : APOSTROPHE IDENT
    ;


/*
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * This is intentionally narrower than a complete expression.
 *
 * Complex expression/place syntax belongs to the canonical expression parser.
 */
borrowTarget
    : IDENT
    | qualifiedBorrowTarget
    ;

qualifiedBorrowTarget
    : IDENT
      (DOUBLE_COLON IDENT)+
    ;


/*
 * ============================================================================
 * REQUIREMENT
 * ============================================================================
 */
borrowRequirement
    : borrowModeModifier?
      borrowTarget
      borrowLifetime?
    ;


/*
 * ============================================================================
 * CONSTRAINT
 * ============================================================================
 */
borrowConstraint
    : borrowName
      borrowConstraintArguments?
    ;

borrowConstraintArguments
    : LPAREN borrowArgumentList? RPAREN
    ;

borrowArgumentList
    : borrowArgument
      (COMMA borrowArgument)*
      COMMA?
    ;

borrowArgument
    : borrowName
    | borrowLifetime
    | borrowTarget
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 */
borrowPreference
    : borrowName
      borrowPreferenceArguments?
    ;

borrowPreferenceArguments
    : LPAREN borrowArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * HINT
 * ============================================================================
 */
borrowHint
    : borrowName
      borrowHintArguments?
    ;

borrowHintArguments
    : LPAREN borrowArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * METADATA
 * ============================================================================
 */
borrowMetadata
    : borrowName
      (ASSIGN borrowArgument)?
    ;

borrowMetadataList
    : borrowMetadata
      (COMMA borrowMetadata)*
      COMMA?
    ;


/*
 * ============================================================================
 * OPEN-WORLD BORROW NAME
 * ============================================================================
 */
borrowName
    : IDENT
      (DOUBLE_COLON IDENT)*
    ;


/*
 * ============================================================================
 * PRIMARY BORROW CLAUSE
 * ============================================================================
 */
borrowClause
    : borrowModeModifier?
      borrowLifetime?
      borrowTarget
    ;


/*
 * ============================================================================
 * EXTENSION
 * ============================================================================
 */
borrowExtension
    : borrowName
      borrowExtensionArguments?
    ;

borrowExtensionArguments
    : LPAREN borrowArgumentList? RPAREN
    ;


/*
 * ============================================================================
 * POLICY
 * ============================================================================
 */
borrowPolicy
    : borrowName
      (LPAREN borrowArgumentList? RPAREN)?
    ;

borrowPolicyList
    : borrowPolicy
      (COMMA borrowPolicy)*
      COMMA?
    ;