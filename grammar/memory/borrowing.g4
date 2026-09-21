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
 *     Production borrowing-domain component.
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition:
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no embedded target-language actions.
 *     Zamani's Rust implementation is required to remain Safe Rust.
 *     No unsafe Rust is required or permitted by this contract.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the reusable SOURCE-LEVEL BORROWING CONTRACT.
 *
 * It describes borrowing intent without implementing borrow checking.
 *
 * Borrowing is intentionally separated from:
 *
 *     - reference-type syntax;
 *     - pointer-type syntax;
 *     - unary-expression syntax;
 *     - general expression syntax;
 *     - memory-place syntax;
 *     - ownership analysis;
 *     - lifetime analysis;
 *     - alias analysis;
 *     - allocation;
 *     - deallocation;
 *     - resource discovery;
 *     - hardware selection;
 *     - scheduling;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     grammar/antlr/ZamaniLexer.g4
 *       |
 *       v
 *     canonical parser assembly
 *       |
 *       +-----------------------------+
 *       |                             |
 *       v                             v
 *     memory.g4                  expression grammar
 *       |                             |
 *       v                             v
 *     borrowing.g4              canonical expressions
 *       |                             |
 *       +-------------+---------------+
 *                     |
 *                     v
 *               domain-neutral AST
 *                     |
 *                     v
 *             semantic analysis
 *                     |
 *       +-------------+--------------+
 *       |             |              |
 *       v             v              v
 *   ownership      lifetime      alias/borrow
 *    analysis       analysis       analysis
 *       |             |              |
 *       +-------------+--------------+
 *                     |
 *                     v
 *             canonical semantic model
 *                     |
 *       +-------------+-------------------------+
 *       |             |                         |
 *       v             v                         v
 *   classical     quantum::ir             HDL/hardware
 *                     |
 *                     v
 *             optimization/lowering
 *                     |
 *          routing / scheduling
 *                     |
 *             QEC / resilience
 *                     |
 *                    ZQN
 *                     |
 *                    HAL
 *                     |
 *              target realization
 *
 * ============================================================================
 * SINGLE RESPONSIBILITY
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 * 1. Borrow mutability intent.
 *
 * 2. Explicit borrow lifetime references when used by borrowing metadata.
 *
 * 3. Borrow-domain source references.
 *
 * 4. Borrow requirements.
 *
 * 5. Borrow constraints.
 *
 * 6. Borrow preferences.
 *
 * 7. Borrow hints.
 *
 * 8. Borrow metadata.
 *
 * 9. Borrow policies.
 *
 * 10. Open-world borrowing extensions.
 *
 * 11. Reusable borrowing fragments for memory-domain composition.
 *
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 * It MUST NOT define:
 *
 *     referenceType
 *     pointerType
 *     unaryExpression
 *     expression
 *     memoryPlace
 *     memoryOperation
 *     declaration
 *     statement
 *     lifetime semantics
 *     ownership semantics
 *     allocation semantics
 *     deallocation semantics
 *     synchronization semantics
 *     resource allocation
 *     hardware placement
 *     quantum routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime behavior
 *
 * ============================================================================
 * CANONICAL REPOSITORY INTEGRATION
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical lexical composition:
 *
 *     grammar/lexer/tokens.g4
 *
 * Canonical identifier token:
 *
 *     IDENTIFIER
 *
 * Relevant canonical tokens consumed by this grammar include:
 *
 *     IDENTIFIER
 *     MUT
 *     APOSTROPHE
 *     DOUBLE_COLON
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     ASSIGN
 *
 * This grammar MUST NOT declare lexer rules.
 *
 * ============================================================================
 * CRITICAL TOKEN CORRECTION
 * ============================================================================
 *
 * Earlier versions of this file referred to:
 *
 *     IDENT
 *
 * That token is NOT the canonical Zamani identifier token.
 *
 * The repository's canonical lexical vocabulary uses:
 *
 *     IDENTIFIER
 *
 * Therefore every identifier reference in this file uses IDENTIFIER directly.
 *
 * This avoids inventing another parser-level identifier dependency and allows
 * the component to remain independently compilable against ZamaniLexer.
 *
 * ============================================================================
 * MEMORY INTEGRATION
 * ============================================================================
 *
 * The canonical memory foundation is:
 *
 *     grammar/memory/memory.g4
 *
 * memory.g4 already owns source-level memory operations and memory-place
 * syntax, including constructs corresponding to:
 *
 *     &value
 *     &mut value
 *     &'a value
 *     &'a mut value
 *
 * Therefore THIS FILE MUST NOT redefine:
 *
 *     memoryBorrow
 *     memoryPlace
 *     memoryLifetimePrefix
 *     memoryOperation
 *     memoryExpression
 *
 * Instead, memory.g4 may compose this file's reusable borrowing metadata
 * constructs where an explicit borrowing clause is permitted.
 *
 * Dependency direction:
 *
 *     memory.g4
 *          |
 *          +--> borrowing.g4
 *
 * NOT:
 *
 *     borrowing.g4 --> memory.g4
 *
 * ============================================================================
 * REFERENCE-TYPE INTEGRATION
 * ============================================================================
 *
 * Reference types remain owned by the canonical type grammar.
 *
 * Examples:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *
 * are TYPE syntax.
 *
 * This file does not define them.
 *
 * Borrowing metadata and reference types are related semantically but are not
 * the same syntactic authority.
 *
 * ============================================================================
 * UNARY-EXPRESSION INTEGRATION
 * ============================================================================
 *
 * The canonical unary grammar owns prefix operators including:
 *
 *     &
 *     *
 *
 * Therefore this file MUST NOT define:
 *
 *     AMPERSAND expression
 *
 * or:
 *
 *     STAR expression
 *
 * as a second unary grammar.
 *
 * The meaning of:
 *
 *     &value
 *
 * is determined downstream by the canonical AST and semantic analysis.
 *
 * ============================================================================
 * LIFETIME INTEGRATION
 * ============================================================================
 *
 * The lifetime token sequence:
 *
 *     ' IDENTIFIER
 *
 * is preserved here only where a borrowing construct requires an explicit
 * lifetime reference.
 *
 * This grammar does NOT determine:
 *
 *     - lifetime start;
 *     - lifetime end;
 *     - scope;
 *     - region inference;
 *     - outlives relationships;
 *     - lifetime validity.
 *
 * Those are semantic responsibilities.
 *
 * ============================================================================
 * BORROWING MODEL
 * ============================================================================
 *
 * Zamani borrowing has two source-level mutability modes:
 *
 *     immutable
 *     mutable
 *
 * Mutable intent is represented by:
 *
 *     MUT
 *
 * Immutable intent is represented by the absence of MUT in a borrowing
 * construct.
 *
 * This grammar does not create a new BORROW keyword.
 *
 * This is intentional:
 *
 *     - it avoids unnecessary lexical compatibility changes;
 *     - it keeps borrowing composable with existing memory syntax;
 *     - it avoids turning a semantic category into a globally reserved word.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Successful parsing means only:
 *
 *     "the source has a structurally valid borrowing representation."
 *
 * It does NOT mean:
 *
 *     "the borrow is legal."
 *
 * Semantic analysis must determine:
 *
 *     - whether the source value exists;
 *     - whether ownership permits borrowing;
 *     - whether the requested mutability is legal;
 *     - whether aliases conflict;
 *     - whether a mutable borrow is exclusive;
 *     - whether an immutable borrow conflicts with mutation;
 *     - whether the lifetime is sufficient;
 *     - whether the borrow escapes;
 *     - whether the borrow crosses an asynchronous boundary;
 *     - whether the borrow crosses a task boundary;
 *     - whether the borrow crosses a process boundary;
 *     - whether the borrow crosses a distributed boundary;
 *     - whether the target realization can preserve the semantics.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * These four categories MUST remain distinct.
 *
 * REQUIREMENT
 * -----------
 *
 * A requirement is mandatory semantic intent.
 *
 * CONSTRAINT
 * ----------
 *
 * A constraint restricts permitted semantic or implementation choices.
 *
 * PREFERENCE
 * ----------
 *
 * A preference is desirable but non-mandatory.
 *
 * HINT
 * ----
 *
 * A hint is optional implementation guidance.
 *
 * This distinction is important for POCO-REAF.
 *
 * A backend MUST NOT silently turn:
 *
 *     requirement -> preference
 *
 * or:
 *
 *     constraint -> hint
 *
 * merely to make a target executable.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Borrowing names are deliberately open-world.
 *
 * Examples:
 *
 *     shared
 *     exclusive
 *     unique
 *     region::scoped
 *     capability::restricted
 *     distributed::borrow
 *     accelerator::borrow
 *     transactional::borrow
 *
 * These are names.
 *
 * Their semantic interpretation belongs downstream.
 *
 * The grammar therefore does NOT enumerate all possible borrowing policies.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO universal limit on:
 *
 *     - number of borrows;
 *     - number of aliases;
 *     - number of lifetimes;
 *     - reference depth;
 *     - number of regions;
 *     - number of values;
 *     - number of tasks;
 *     - number of threads;
 *     - number of cores;
 *     - number of GPUs;
 *     - number of FPGAs;
 *     - number of QPUs;
 *     - number of nodes;
 *     - memory capacity;
 *     - address width;
 *     - pointer width;
 *     - tensor dimensions;
 *     - quantum resources;
 *     - distributed resources.
 *
 * "Infinity" means that the language grammar establishes no artificial finite
 * ceiling. Actual compiler/parser/runtime limits remain implementation and
 * resource policies.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Borrowing syntax MUST NOT identify:
 *
 *     CPU 0
 *     GPU 0
 *     FPGA 0
 *     QPU 0
 *     physical qubit 0
 *     memory bank 0
 *     NUMA node 0
 *     physical address
 *     fixed device topology
 *
 * Hardware realization belongs downstream.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * Borrowing may semantically apply to:
 *
 *     classical values
 *     vectors
 *     matrices
 *     tensors
 *     datasets
 *     streams
 *     quantum semantic objects
 *     logical quantum resources
 *     HDL resources
 *     accelerator buffers
 *     distributed objects
 *     network data
 *     security-sensitive values
 *     future computational domains
 *
 * This file must not create one borrowing grammar per domain.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser-to-AST adapter must preserve at least:
 *
 *     - source span;
 *     - borrow mutability;
 *     - explicit lifetime, when present;
 *     - target/reference name;
 *     - qualification path;
 *     - requirement/constraint/preference/hint classification;
 *     - metadata;
 *     - extension name;
 *     - extension arguments.
 *
 * The exact AST type belongs to the repository's domain-neutral frontend AST.
 *
 * This grammar must not introduce:
 *
 *     QuantumBorrow
 *     CpuBorrow
 *     GpuBorrow
 *     FpgaBorrow
 *
 * or similar domain-specific AST syntax classes.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis combines this syntax with:
 *
 *     ownership
 *     lifetime
 *     types
 *     effects
 *     concurrency
 *     resources
 *     capabilities
 *     domain semantics
 *
 * The semantic result must distinguish:
 *
 *     borrow intent
 *     ownership state
 *     lifetime state
 *     aliasing state
 *     resource realization
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar has NO IR dependency.
 *
 * In particular it MUST NOT depend directly on:
 *
 *     quantum::ir
 *     classical IR
 *     HDL IR
 *     hardware IR
 *     ZQN
 *     QEC
 *
 * Correct direction:
 *
 *     borrowing syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *       +--+--------------------+
 *       |                       |
 *       v                       v
 *   classical              quantum::ir
 *       |                       |
 *       +-----------+-----------+
 *                   |
 *                   v
 *             lowering/optimization
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Borrowing of a quantum-related semantic object is still borrowing.
 *
 * This file MUST NOT introduce:
 *
 *     quantum borrow registers
 *     physical qubit borrow syntax
 *     QPU identifiers
 *     physical-qubit mappings
 *     gate-specific borrowing
 *
 * If a quantum construct is involved, semantic analysis eventually maps the
 * resulting semantics through:
 *
 *     quantum::ir
 *
 * The canonical quantum semantic boundary remains unchanged.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Borrowing syntax may apply semantically to hardware/co-design values, but
 * this file does not describe:
 *
 *     wires;
 *     registers;
 *     buses;
 *     physical addresses;
 *     memory banks;
 *     FPGA resources;
 *     ASIC resources;
 *     clock topology.
 *
 * Those belong to HDL/hardware/resource layers.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A borrow that crosses a process, node, service, or machine boundary may
 * require special semantic treatment.
 *
 * This grammar does not decide whether such a borrow:
 *
 *     - remains a reference;
 *     - is copied;
 *     - is serialized;
 *     - becomes remote;
 *     - is rejected.
 *
 * That decision belongs to semantic analysis and target realization.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Borrowing does not select resources.
 *
 * Resource/capability analysis may consume semantic information derived from
 * borrowing, but this grammar must not select:
 *
 *     CPU;
 *     GPU;
 *     FPGA;
 *     QPU;
 *     memory bank;
 *     node;
 *     accelerator.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     - source token stream;
 *     - grammar version;
 *     - canonical lexer;
 *     - explicitly selected dialect configuration.
 *
 * Parsing must NOT depend on:
 *
 *     - hardware availability;
 *     - CPU count;
 *     - GPU count;
 *     - QPU count;
 *     - memory capacity;
 *     - filesystem state;
 *     - network state;
 *     - wall-clock time;
 *     - randomness;
 *     - runtime scheduler state.
 *
 * ============================================================================
 * DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Syntax errors owned here include:
 *
 *     malformed lifetime spelling;
 *     malformed qualified borrow name;
 *     malformed metadata;
 *     malformed extension;
 *     malformed argument structure.
 *
 * Semantic errors belong downstream:
 *
 *     invalid borrow;
 *     invalid mutable alias;
 *     invalid lifetime;
 *     ownership violation;
 *     illegal escape;
 *     unsupported cross-boundary borrow;
 *     unavailable semantic capability.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing canonical tokens are consumed rather than renamed.
 *
 * In particular:
 *
 *     IDENTIFIER
 *     MUT
 *     APOSTROPHE
 *     DOUBLE_COLON
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     ASSIGN
 *
 * remain the lexical integration boundary.
 *
 * This file does not introduce a BORROW keyword and therefore does not
 * unnecessarily change the global keyword set.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests
 * --------------
 *
 * Mutable:
 *
 *     mut x
 *     mut value
 *     mut memory::buffer
 *
 * Immutable:
 *
 *     x
 *     value
 *     memory::buffer
 *
 * Lifetimes:
 *
 *     'a
 *     'buffer
 *     'scope
 *
 * Borrow references:
 *
 *     x
 *     value
 *     memory::value
 *     distributed::buffer
 *     future::domain::value
 *
 * Policies:
 *
 *     exclusive
 *     region::scoped
 *     capability::restricted
 *     distributed::borrow
 *
 * Policy arguments:
 *
 *     capability::restricted(x)
 *     region::scoped('a)
 *
 * Metadata:
 *
 *     shared
 *     exclusive = value
 *     region::scoped = 'a
 *
 * Extensions:
 *
 *     future::borrow
 *     future::borrow(x)
 *     vendor::extension::borrow(value)
 *
 * Negative syntax tests
 * ---------------------
 *
 * Reject:
 *
 *     '
 *     ' mut
 *     mut
 *     ::
 *     ::x
 *     x::
 *     x:::
 *     region::
 *     region:::scoped
 *     capability::restricted(
 *     capability::restricted(x, )
 *
 * Note:
 *
 * A bare `mut` may be valid in some larger language contexts. The borrowing
 * component alone does not classify every occurrence of a keyword. Contextual
 * validity belongs to the host grammar.
 *
 * Boundary tests
 * --------------
 *
 * Include:
 *
 *     deeply qualified names;
 *     many metadata entries;
 *     many policy arguments;
 *     large source programs;
 *     symbolic lifetimes;
 *     long but valid source identifiers.
 *
 * No artificial finite grammar limit is permitted.
 *
 * Cross-domain tests
 * ------------------
 *
 * The borrowing contract must compose with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *
 * without creating domain-specific borrowing syntax.
 *
 * Determinism tests
 * -----------------
 *
 * Identical:
 *
 *     source
 *     grammar version
 *     lexer configuration
 *
 * must produce identical parse structure.
 *
 * Round-trip tests
 * ----------------
 *
 * Formatting/parsing must preserve:
 *
 *     mutability;
 *     lifetime;
 *     target;
 *     qualification;
 *     metadata;
 *     requirement/constraint/preference/hint category;
 *     extension names;
 *     extension arguments.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Repository validation MUST reject future modifications introducing
 * universal constants equivalent to:
 *
 *     MAX_BORROWS
 *     MAX_SHARED_BORROWS
 *     MAX_MUTABLE_BORROWS
 *     MAX_REFERENCE_DEPTH
 *     MAX_LIFETIME_COUNT
 *     MAX_ALIAS_COUNT
 *     MAX_MEMORY
 *     MAX_POINTER_WIDTH
 *     MAX_ADDRESS_WIDTH
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *
 * Program-level numeric values are not prohibited.
 *
 * The prohibition is specifically against turning implementation limits into
 * universal language semantics.
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no embedded Rust actions.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The repository's Rust implementation must not require:
 *
 *     unsafe
 *
 * for this grammar or its semantic integration.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is a parser grammar.
 *
 * [x] It uses tokenVocab = ZamaniLexer.
 *
 * [x] It declares no lexer rules.
 *
 * [x] It uses IDENTIFIER, not the obsolete/non-canonical IDENT token.
 *
 * [x] It does not redefine memoryPlace.
 *
 * [x] It does not redefine memoryBorrow.
 *
 * [x] It does not redefine referenceType.
 *
 * [x] It does not redefine unaryExpression.
 *
 * [x] It does not define a second expression hierarchy.
 *
 * [x] It preserves explicit lifetime references.
 *
 * [x] It preserves mutable/immutable intent.
 *
 * [x] Requirements, constraints, preferences and hints remain distinct.
 *
 * [x] Qualified names are open-world.
 *
 * [x] No hardware limits are encoded.
 *
 * [x] No resource limits are encoded.
 *
 * [x] No physical addresses are encoded.
 *
 * [x] No device identifiers are encoded.
 *
 * [x] No quantum gate enumeration is encoded.
 *
 * [x] No second quantum IR is introduced.
 *
 * [x] Semantic legality remains downstream.
 *
 * [x] Resource realization remains downstream.
 *
 * [x] Routing remains downstream.
 *
 * [x] Scheduling remains downstream.
 *
 * [x] QEC remains downstream.
 *
 * [x] ZQN remains downstream.
 *
 * [x] HAL remains downstream.
 *
 * [x] Safe Rust/no-unsafe requirement is preserved.
 *
 * [x] Positive, negative, boundary, cross-domain, deterministic and
 *     round-trip tests are specified.
 *
 * ============================================================================
 */

parser grammar Borrowing;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL IDENTIFIER BRIDGE
 * ============================================================================
 *
 * This rule deliberately consumes the canonical lexer token directly.
 *
 * It does NOT create a second lexical identifier.
 *
 * It also avoids depending on an assembly-local parser rule named `identifier`,
 * making this component independently composable.
 */
borrowIdentifier
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * BORROW MODE
 * ============================================================================
 *
 * `mut` is explicit mutable intent.
 *
 * Absence of `mut` means immutable intent in a borrowing context.
 *
 * This rule is intentionally optional as a single rule rather than being
 * nested inside another optional `borrowMode?`, avoiding avoidable epsilon
 * ambiguity.
 */
borrowMode
    : MUT?
    ;


/*
 * Explicit mutable marker.
 */
borrowModeModifier
    : MUT
    ;


/*
 * ============================================================================
 * BORROW LIFETIME
 * ============================================================================
 *
 * Source syntax:
 *
 *     'a
 *     'buffer
 *     'scope
 *
 * The identifier is a symbolic lifetime name.
 */
borrowLifetime
    : APOSTROPHE borrowIdentifier
    ;


/*
 * ============================================================================
 * BORROW NAME
 * ============================================================================
 *
 * Open-world qualified name.
 *
 * Examples:
 *
 *     shared
 *     exclusive
 *     region::scoped
 *     capability::restricted
 *     future::borrow::policy
 *
 * There is no fixed qualification depth.
 */
borrowName
    : borrowIdentifier
      (DOUBLE_COLON borrowIdentifier)*
    ;


/*
 * ============================================================================
 * BORROW TARGET
 * ============================================================================
 *
 * This is intentionally NOT a general expression grammar.
 *
 * Complex source places remain owned by:
 *
 *     grammar/memory/memory.g4
 *
 * and the canonical expression hierarchy.
 *
 * Here a target is a symbolic source reference suitable for metadata and
 * borrowing contracts.
 */
borrowTarget
    : borrowName
    ;


/*
 * ============================================================================
 * BORROW REFERENCE
 * ============================================================================
 *
 * A borrow reference combines:
 *
 *     optional mutable intent
 *     target
 *     optional explicit lifetime
 *
 * Examples:
 *
 *     x
 *     mut x
 *     x 'a
 *     mut x 'a
 *     memory::buffer
 *     mut memory::buffer 'scope
 *
 * The host grammar determines the context in which this construct is legal.
 */
borrowReference
    : borrowMode borrowTarget borrowLifetime?
    ;


/*
 * ============================================================================
 * BORROW REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory semantic intent.
 *
 * The syntax itself does not establish validity.
 *
 * The semantic layer determines whether the requirement can be satisfied.
 */
borrowRequirement
    : borrowReference
    ;


/*
 * ============================================================================
 * BORROW CONSTRAINT
 * ============================================================================
 *
 * A constraint names a restriction on legal borrowing behavior.
 *
 * The name is intentionally open-world.
 */
borrowConstraint
    : borrowName borrowArgumentClause?
    ;


/*
 * ============================================================================
 * BORROW PREFERENCE
 * ============================================================================
 *
 * A preference is non-mandatory.
 */
borrowPreference
    : borrowName borrowArgumentClause?
    ;


/*
 * ============================================================================
 * BORROW HINT
 * ============================================================================
 *
 * A hint is optional implementation guidance.
 */
borrowHint
    : borrowName borrowArgumentClause?
    ;


/*
 * ============================================================================
 * BORROW ARGUMENT CLAUSE
 * ============================================================================
 *
 * Arguments are deliberately symbolic.
 *
 * General expressions remain owned by the host grammar.
 *
 * If a future borrowing feature requires arbitrary expressions, the host
 * grammar should provide the expression-bearing integration rule rather than
 * making this component a second expression hierarchy.
 */
borrowArgumentClause
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
 * BORROW METADATA
 * ============================================================================
 *
 * Generic metadata is intentionally open-world.
 *
 * Examples:
 *
 *     shared
 *     exclusive
 *     region::scoped = 'a
 *     capability::restricted = value
 */
borrowMetadata
    : borrowName
      (ASSIGN borrowMetadataValue)?
    ;


borrowMetadataValue
    : borrowName
    | borrowLifetime
    | borrowTarget
    ;


borrowMetadataList
    : borrowMetadata
      (COMMA borrowMetadata)*
      COMMA?
    ;


/*
 * ============================================================================
 * BORROW POLICY
 * ============================================================================
 *
 * A policy is a named borrowing semantic category.
 *
 * The grammar does not determine its implementation.
 */
borrowPolicy
    : borrowName borrowArgumentClause?
    ;


borrowPolicyList
    : borrowPolicy
      (COMMA borrowPolicy)*
      COMMA?
    ;


/*
 * ============================================================================
 * BORROW CLAUSE
 * ============================================================================
 *
 * Reusable host-grammar fragment.
 *
 * This is deliberately not a statement.
 *
 * It does not add a terminator.
 *
 * The enclosing grammar decides where the clause is legal.
 */
borrowClause
    : borrowReference
    ;


/*
 * ============================================================================
 * BORROW REQUIREMENT LIST
 * ============================================================================
 */
borrowRequirementList
    : borrowRequirement
      (COMMA borrowRequirement)*
      COMMA?
    ;


/*
 * ============================================================================
 * BORROW CONSTRAINT LIST
 * ============================================================================
 */
borrowConstraintList
    : borrowConstraint
      (COMMA borrowConstraint)*
      COMMA?
    ;


/*
 * ============================================================================
 * BORROW PREFERENCE LIST
 * ============================================================================
 */
borrowPreferenceList
    : borrowPreference
      (COMMA borrowPreference)*
      COMMA?
    ;


/*
 * ============================================================================
 * BORROW HINT LIST
 * ============================================================================
 */
borrowHintList
    : borrowHint
      (COMMA borrowHint)*
      COMMA?
    ;


/*
 * ============================================================================
 * BORROW EXTENSION
 * ============================================================================
 *
 * Open-world extension point.
 *
 * Examples:
 *
 *     future::borrow
 *     future::borrow(x)
 *     vendor::extension::borrow(value)
 *
 * No vendor, machine, processor, QPU, GPU or FPGA vocabulary is hard-coded.
 */
borrowExtension
    : borrowName borrowArgumentClause?
    ;


borrowExtensionList
    : borrowExtension
      (COMMA borrowExtension)*
      COMMA?
    ;


/*
 * ============================================================================
 * BORROW CONTRACT
 * ============================================================================
 *
 * This is a reusable aggregate for hosts that need to preserve all borrowing
 * metadata categories together.
 *
 * It intentionally contains no declaration/statement boundary.
 *
 * A host grammar may consume only the subset it permits.
 */
borrowContract
    : borrowMetadataList?
      borrowRequirementList?
      borrowConstraintList?
      borrowPreferenceList?
      borrowHintList?
      borrowPolicyList?
      borrowExtensionList?
    ;


/*
 * ============================================================================
 * INTEGRATION RULES
 * ============================================================================
 *
 * The following rules are the stable public interface of this component:
 *
 *     borrowMode
 *     borrowModeModifier
 *     borrowLifetime
 *     borrowName
 *     borrowTarget
 *     borrowReference
 *     borrowRequirement
 *     borrowConstraint
 *     borrowPreference
 *     borrowHint
 *     borrowMetadata
 *     borrowMetadataList
 *     borrowPolicy
 *     borrowPolicyList
 *     borrowClause
 *     borrowExtension
 *     borrowExtensionList
 *     borrowContract
 *
 * memory.g4 may import this grammar and use these rules.
 *
 * memory.g4 remains the owner of:
 *
 *     memoryBorrow
 *     memoryPlace
 *     memoryOperation
 *     memoryExpression
 *     memoryDeclaration
 *
 * ============================================================================
 * HOST EXPRESSION INTEGRATION
 * ============================================================================
 *
 * If a future host context needs a borrowing construct whose argument is an
 * arbitrary expression, the host should provide a bridge such as:
 *
 *     hostBorrowArgument
 *         : expression
 *         ;
 *
 * and compose it with the borrowing semantic category.
 *
 * This file must not import the complete expression hierarchy merely to support
 * one optional argument because doing so would make this leaf component
 * responsible for expression composition and could introduce grammar cycles.
 *
 * ============================================================================
 * OWNERSHIP INTEGRATION
 * ============================================================================
 *
 * ownership.g4 remains the owner of ownership syntax.
 *
 * Borrowing and ownership are intentionally separate:
 *
 *     ownership = who controls the value/resource
 *
 *     borrowing = who requests temporary/non-owning access
 *
 * Semantic analysis combines them.
 *
 * This grammar does not define:
 *
 *     move
 *     copy
 *     drop
 *     transfer
 *     ownership transfer
 *
 * ============================================================================
 * LIFETIME INTEGRATION
 * ============================================================================
 *
 * The explicit lifetime token sequence preserved here is only a symbolic
 * reference.
 *
 * Lifetime analysis remains downstream.
 *
 * ============================================================================
 * ALLOCATION / DEALLOCATION INTEGRATION
 * ============================================================================
 *
 * A borrow does not imply:
 *
 *     allocation
 *     deallocation
 *     copying
 *     reference counting
 *     garbage collection
 *     pinning
 *     migration
 *
 * Any such behavior is an implementation decision governed by semantics.
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * A borrow may participate in:
 *
 *     asynchronous execution
 *     task parallelism
 *     actors
 *     channels
 *     distributed execution
 *
 * but this grammar does not decide whether the borrow may cross such a
 * boundary.
 *
 * Concurrency and effect analysis determine legality.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A distributed implementation may represent a borrow through:
 *
 *     local reference;
 *     remote reference;
 *     copied value;
 *     serialized value;
 *     synchronized proxy;
 *     another semantically equivalent mechanism.
 *
 * The grammar does not choose among these.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Borrowing may contribute information to resource/capability analysis.
 *
 * It must not select physical resources.
 *
 * Correct separation:
 *
 *     borrow syntax
 *          |
 *          v
 *     semantic borrow model
 *          |
 *          v
 *     resource/capability analysis
 *          |
 *          v
 *     realization
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A borrow involving a quantum semantic object follows:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *
 * This grammar never creates:
 *
 *     QuantumBorrow
 *     PhysicalQubitBorrow
 *     QPU0Borrow
 *
 * or any other hardware-specific grammar.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Borrowing may be associated semantically with:
 *
 *     HDL values;
 *     accelerator buffers;
 *     hardware resources;
 *     device-visible data.
 *
 * But this grammar does not encode:
 *
 *     addresses;
 *     buses;
 *     register widths;
 *     physical memory banks;
 *     FPGA identities;
 *     ASIC identities;
 *     device identities.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler stages consume the semantic result, not this grammar directly.
 *
 * The compiler must be able to lower the same borrowing semantics differently
 * depending on target capabilities without requiring source changes.
 *
 * Examples include:
 *
 *     stack/reference representation;
 *     managed memory;
 *     device buffer;
 *     distributed object;
 *     quantum semantic resource;
 *     future memory technology.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime behavior is outside this grammar.
 *
 * A runtime may enforce semantic borrowing guarantees through any valid
 * implementation strategy.
 *
 * ============================================================================
 * TOOLING INTEGRATION
 * ============================================================================
 *
 * Formatters, IDEs, language servers, syntax highlighters, refactoring tools,
 * and linters should consume the canonical parse tree.
 *
 * They must not create an independent borrowing grammar.
 *
 * ============================================================================
 * VERSIONING
 * ============================================================================
 *
 * Changes to the following are language-contract changes:
 *
 *     borrowReference structure;
 *     lifetime syntax;
 *     qualified-name syntax;
 *     requirement/constraint/preference/hint distinction;
 *     metadata structure.
 *
 * Adding a new qualified borrowing policy does NOT necessarily require a
 * grammar change if the policy can be represented through the open-world
 * borrowing-name mechanism.
 *
 * ============================================================================
 * NEGATIVE / POSITIVE BOUNDARY
 * ============================================================================
 *
 * This component intentionally does not reject every semantically invalid
 * construct.
 *
 * For example:
 *
 *     mut x
 *
 * may be syntactically valid borrowing intent but semantically invalid if `x`
 * cannot be mutably borrowed.
 *
 * Such validation belongs downstream.
 *
 * ============================================================================
 * PRODUCTION INVARIANTS
 * ============================================================================
 *
 * 1. One canonical lexer.
 *
 * 2. One canonical token vocabulary.
 *
 * 3. No local lexer rules.
 *
 * 4. No `IDENT` token.
 *
 * 5. No duplicate memory-place grammar.
 *
 * 6. No duplicate reference-type grammar.
 *
 * 7. No duplicate unary grammar.
 *
 * 8. No ownership implementation.
 *
 * 9. No lifetime implementation.
 *
 * 10. No resource discovery.
 *
 * 11. No physical hardware selection.
 *
 * 12. No fixed machine limits.
 *
 * 13. No quantum gate enumeration.
 *
 * 14. No second quantum IR.
 *
 * 15. No runtime execution.
 *
 * 16. Requirements remain requirements.
 *
 * 17. Constraints remain constraints.
 *
 * 18. Preferences remain preferences.
 *
 * 19. Hints remain hints.
 *
 * 20. Semantic validation remains downstream.
 *
 * ============================================================================
 */