/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/closures.g4
 *
 * Grammar:
 *     Closures
 *
 * Purpose:
 *     Canonical source-level grammar for closure-capture syntax.
 *
 * Language baseline:
 *     Zamani
 *
 * Compiler implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     The Rust compiler/frontend implementation integrating this grammar
 *     MUST use safe Rust only.
 *
 *     No unsafe Rust is required or permitted by this grammar contract.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * Zamani separates:
 *
 *     lambda syntax
 *     closure-capture syntax
 *     semantic capture analysis
 *     type checking
 *     ownership/borrowing
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     IR lowering
 *     runtime representation
 *
 * This file owns only the SOURCE-SYNTAX REPRESENTATION of explicit closure
 * capture specifications.
 *
 * It does NOT decide how a closure is represented or executed.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - closure capture clauses;
 *     - explicit capture entries;
 *     - capture modes;
 *     - capture aliases;
 *     - wildcard/default capture declarations;
 *     - capture ordering in source;
 *     - optional capture lists;
 *     - syntactic capture modifiers.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lambda expression syntax;
 *     - ordinary function declarations;
 *     - ordinary function parameters;
 *     - function types;
 *     - general expressions;
 *     - statements;
 *     - blocks;
 *     - identifiers;
 *     - qualified names;
 *     - types;
 *     - ownership semantics;
 *     - borrow checking;
 *     - lifetime checking;
 *     - closure conversion;
 *     - environment layout;
 *     - stack/heap allocation;
 *     - function-pointer representation;
 *     - ABI;
 *     - calling convention;
 *     - threading;
 *     - scheduling;
 *     - placement;
 *     - hardware selection;
 *     - CPU/GPU/FPGA/ASIC selection;
 *     - quantum execution;
 *     - physical qubits;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - optimization;
 *     - runtime execution.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Closure capture describes SOURCE-LEVEL DEPENDENCIES.
 *
 * It must never describe a physical execution resource.
 *
 * Therefore this grammar contains no:
 *
 *     MAX_CAPTURES
 *     MAX_ENVIRONMENT_SIZE
 *     MAX_CLOSURES
 *     MAX_PARAMETERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *
 * and no equivalent hidden limitation.
 *
 * A closure may capture one value, many values, or a dynamically determined
 * set of values where the language semantics permit it.
 *
 * The actual implementation is constrained only by the resources and
 * policies of the compiler/runtime, not by the source grammar.
 *
 * ============================================================================
 * SEMANTIC MODEL
 * ============================================================================
 *
 * A closure capture specification describes:
 *
 *     WHAT is captured
 *     HOW it is requested to be captured
 *
 * It does NOT decide:
 *
 *     WHERE the captured value is stored
 *     WHETHER storage is stack or heap
 *     WHETHER the closure is copied
 *     WHETHER the closure is moved
 *     WHETHER the closure is lowered to a function
 *     WHETHER an environment object is generated
 *     WHETHER the closure is inlined
 *     WHETHER it becomes a task
 *     WHETHER it becomes a GPU kernel
 *     WHETHER it becomes an accelerator operation
 *     WHETHER it participates in distributed execution
 *
 * Those decisions belong to semantic analysis, lowering, optimization,
 * scheduling, and runtime layers.
 *
 * ============================================================================
 * CAPTURE MODEL
 * ============================================================================
 *
 * The syntax supports explicit capture intent.
 *
 * Conceptually:
 *
 *     capture { x }
 *
 *     capture { x, y }
 *
 *     capture { move x }
 *
 *     capture { ref x }
 *
 *     capture { mut x }
 *
 *     capture { move x as y }
 *
 *     capture { ref x as y }
 *
 *     capture { mut x as y }
 *
 *     capture { * }
 *
 *     capture { move * }
 *
 * The exact semantic legality of these forms is NOT decided here.
 *
 * In particular:
 *
 *     ref
 *     mut
 *     move
 *
 * describe source-level capture intent only.
 *
 * The semantic layer determines whether the requested capture is compatible
 * with:
 *
 *     - the binding;
 *     - ownership;
 *     - borrowing;
 *     - lifetime;
 *     - mutability;
 *     - type;
 *     - effects;
 *     - capabilities;
 *     - concurrency;
 *     - execution domain.
 *
 * ============================================================================
 * DEFAULT CAPTURE
 * ============================================================================
 *
 * A wildcard capture:
 *
 *     *
 *
 * means:
 *
 *     "capture according to the language's semantic default policy."
 *
 * It MUST NOT mean:
 *
 *     "capture every machine resource."
 *
 * It MUST NOT encode a finite or machine-dependent environment size.
 *
 * The semantic analyzer resolves the actual captured bindings.
 *
 * ============================================================================
 * EXPLICIT CAPTURE
 * ============================================================================
 *
 * Explicit capture:
 *
 *     capture { value }
 *
 * identifies a source binding.
 *
 * The binding is resolved by name resolution after parsing.
 *
 * This grammar deliberately does not determine whether the name refers to:
 *
 *     local variable
 *     parameter
 *     module binding
 *     imported binding
 *     pattern binding
 *     captured outer closure
 *
 * Name resolution owns that decision.
 *
 * ============================================================================
 * ALIASING
 * ============================================================================
 *
 * A capture may optionally introduce a local alias:
 *
 *     capture { value as capturedValue }
 *
 * The alias is source-level naming information.
 *
 * It does not prescribe physical storage.
 *
 * ============================================================================
 * CAPTURE ORDER
 * ============================================================================
 *
 * Source order is preserved by the parser.
 *
 * Semantic analysis may canonicalize capture ordering for IR purposes, but
 * such canonicalization must preserve source semantics and diagnostics.
 *
 * ============================================================================
 * DUPLICATES
 * ============================================================================
 *
 * Duplicate captures are syntactically accepted:
 *
 *     capture { x, x }
 *
 * unless the surrounding grammar composition rejects them earlier.
 *
 * This is intentional.
 *
 * Whether duplicate captures are:
 *
 *     redundant
 *     conflicting
 *     aliases
 *     an error
 *
 * is a semantic question.
 *
 * The parser must not incorrectly reject syntactically valid source merely
 * because a later semantic phase has not yet resolved the bindings.
 *
 * ============================================================================
 * INTEGRATION WITH LAMBDAS
 * ============================================================================
 *
 * `grammar/expressions/lambdas.g4` owns lambda syntax.
 *
 * This file provides:
 *
 *     closureCaptureClause
 *
 * and related rules.
 *
 * The lambda grammar may consume:
 *
 *     closureCaptureClause?
 *
 * before its parameter clause.
 *
 * Conceptually:
 *
 *     closureCaptureClause?
 *     lambdaModifier*
 *     lambdaParameterClause
 *     lambdaReturnTypeClause?
 *     lambdaBody
 *
 * This prevents closure syntax from being duplicated inside lambdas.g4.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It MUST NOT define lexer rules.
 *
 * It consumes the canonical Zamani lexer vocabulary.
 *
 * Required lexical tokens:
 *
 *     CAPTURE
 *     MOVE
 *     REF
 *     MUT
 *     AS
 *     STAR
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     IDENTIFIER
 *
 * If the repository uses different canonical token names, the token names
 * MUST be adapted at the lexer ownership boundary rather than introducing
 * duplicate lexer tokens here.
 *
 * ============================================================================
 * IMPORTANT: KEYWORD OWNERSHIP
 * ============================================================================
 *
 * This file intentionally does not define:
 *
 *     CAPTURE
 *     REF
 *     MOVE
 *     MUT
 *     AS
 *
 * as lexer rules.
 *
 * The canonical lexer owns them.
 *
 * This prevents keyword duplication across grammar modules.
 *
 * ============================================================================
 * NAME OWNERSHIP
 * ============================================================================
 *
 * This file does not define identifier syntax.
 *
 * The canonical name grammar owns:
 *
 *     identifier
 *     qualifiedName
 *
 * Capture targets intentionally use `IDENTIFIER` at this syntactic boundary.
 *
 * Name resolution may later determine whether a richer source name form is
 * appropriate.
 *
 * ============================================================================
 * TYPE OWNERSHIP
 * ============================================================================
 *
 * Capture declarations contain no type grammar.
 *
 * Types belong to:
 *
 *     grammar/types/
 *
 * Capture semantics are derived from the type system after parsing.
 *
 * ============================================================================
 * EFFECT OWNERSHIP
 * ============================================================================
 *
 * Closure captures do not define effects.
 *
 * Effect declarations and checking belong to:
 *
 *     grammar/effects/
 *
 * and the semantic effect system.
 *
 * ============================================================================
 * RESOURCE OWNERSHIP
 * ============================================================================
 *
 * This file contains no:
 *
 *     resource counts
 *     memory sizes
 *     accelerator counts
 *     device IDs
 *     topology
 *     placement
 *     timing
 *
 * Closure captures are semantic dependencies, not physical resource
 * declarations.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A closure may semantically capture a quantum value if the type system and
 * quantum semantic model permit it.
 *
 * This file must not introduce special grammar such as:
 *
 *     capture qubit
 *     capture q[0]
 *     capture physical-qubit
 *
 * Quantum types remain owned by the quantum/type grammar.
 *
 * The resulting semantic representation may eventually lower into
 * `quantum::ir` when the captured computation is quantum-semantic.
 *
 * This file itself never creates or modifies quantum IR.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * A closure capture must not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     node
 *     device
 *     memory bank
 *     physical address
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Capture syntax is independent of execution parallelism.
 *
 * Semantic analysis determines whether a captured binding may safely
 * participate in:
 *
 *     tasks
 *     futures
 *     actors
 *     parallel execution
 *     distributed execution
 *     asynchronous execution
 *
 * No concurrency resource limit is encoded here.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Capturing a value does not imply that the value is serializable,
 * transferable, remotely accessible, replicated, or movable between nodes.
 *
 * Those properties belong to semantic/type/resource/distributed analysis.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * A closure capture does not automatically grant authority.
 *
 * Capabilities, permissions, identity, trust, and security policies remain
 * owned by the security/capability layers.
 *
 * ============================================================================
 * CANONICAL RULES
 * ============================================================================
 */

parser grammar Closures;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. CLOSURE CAPTURE CLAUSE
 * ============================================================================
 *
 * Canonical form:
 *
 *     capture { ... }
 *
 * The braces make the capture region explicit and avoid ambiguity with the
 * lambda parameter pipes.
 */

closureCaptureClause
    : CAPTURE
      LBRACE
      closureCaptureList?
      RBRACE
    ;


/*
 * ============================================================================
 * 2. CAPTURE LIST
 * ============================================================================
 *
 * No fixed capture count is encoded.
 */

closureCaptureList
    : closureCapture
      (COMMA closureCapture)*
      COMMA?
    ;


/*
 * ============================================================================
 * 3. CAPTURE ENTRY
 * ============================================================================
 *
 * An entry may be:
 *
 *     explicit binding
 *     wildcard capture
 *     mode-qualified binding
 *     mode-qualified wildcard
 */

closureCapture
    : closureCaptureMode* closureCaptureTarget closureCaptureAlias?
    ;


/*
 * ============================================================================
 * 4. CAPTURE MODE
 * ============================================================================
 *
 * Multiple modifiers are syntactically permitted so that semantic analysis
 * can diagnose conflicting combinations.
 *
 * Examples:
 *
 *     move x
 *     ref x
 *     mut x
 *
 * The parser does not decide whether combinations are legal.
 */

closureCaptureMode
    : MOVE
    | REF
    | MUT
    ;


/*
 * ============================================================================
 * 5. CAPTURE TARGET
 * ============================================================================
 *
 * A wildcard requests semantic default capture.
 *
 * An identifier refers to a source binding resolved later.
 */

closureCaptureTarget
    : IDENTIFIER
    | STAR
    ;


/*
 * ============================================================================
 * 6. CAPTURE ALIAS
 * ============================================================================
 *
 * Example:
 *
 *     capture { value as capturedValue }
 */

closureCaptureAlias
    : AS
      IDENTIFIER
    ;


/*
 * ============================================================================
 * 7. SEMANTIC INVARIANTS
 * ============================================================================
 *
 * The parser produces a lossless representation of:
 *
 *     capture clause
 *     capture order
 *     capture modes
 *     capture target
 *     optional alias
 *
 * Semantic analysis MUST subsequently validate:
 *
 *     - target existence;
 *     - target scope;
 *     - duplicate captures;
 *     - conflicting modes;
 *     - mutable capture legality;
 *     - reference lifetime;
 *     - move legality;
 *     - alias collisions;
 *     - type compatibility;
 *     - ownership constraints;
 *     - borrowing constraints;
 *     - effect constraints;
 *     - capability constraints;
 *     - concurrency constraints;
 *     - distributed-execution constraints;
 *     - quantum-value restrictions where applicable.
 *
 * None of those checks belong in this grammar.
 *
 * ============================================================================
 * 8. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify the syntactic location of:
 *
 *     CAPTURE
 *     capture target
 *     capture mode
 *     alias
 *     separators
 *     delimiters
 *
 * Semantic diagnostics should subsequently identify:
 *
 *     unknown capture target
 *     illegal capture mode
 *     conflicting capture modes
 *     invalid alias
 *     invalid lifetime
 *     ownership violation
 *     capability violation
 *
 * The parser must not manufacture semantic diagnostics.
 *
 * ============================================================================
 * 9. DETERMINISM
 * ============================================================================
 *
 * For identical source and identical lexer configuration:
 *
 *     parsing result
 *     parse-tree structure
 *     token ordering
 *     source spans
 *
 * must be deterministic.
 *
 * No runtime state, hardware discovery, resource discovery, or random value
 * may influence this grammar.
 *
 * ============================================================================
 * 10. SCALABILITY
 * ============================================================================
 *
 * The grammar places no fixed upper bound on:
 *
 *     number of captures
 *     number of closures
 *     number of functions
 *     closure nesting depth
 *     source-file size
 *
 * Actual implementation resource exhaustion is handled by parser/compiler
 * resource policies rather than source-language constants.
 *
 * ============================================================================
 * 11. ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * A formatter/AST serializer may preserve:
 *
 *     capture mode
 *     capture target
 *     capture alias
 *     capture order
 *
 * without changing semantic meaning.
 *
 * ============================================================================
 * 12. IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT emit IR.
 *
 * The frontend should lower the parsed capture information into the canonical
 * semantic representation.
 *
 * The resulting semantic representation may then be consumed by:
 *
 *     classical IR
 *     quantum::ir
 *     hybrid IR
 *     hardware/control IR
 *     distributed IR
 *
 * according to the actual computation.
 *
 * This file must never import or depend on those IR implementations.
 *
 * ============================================================================
 * 13. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime representation is deliberately unspecified.
 *
 * Valid implementations may use:
 *
 *     stack environments
 *     heap environments
 *     static environments
 *     function objects
 *     function pointers
 *     closures converted to functions
 *     specialized environments
 *     distributed closures
 *     accelerator-specific representations
 *
 * The source grammar remains unchanged.
 *
 * ============================================================================
 * 14. RUST CONTRACT
 * ============================================================================
 *
 * The grammar itself contains no Rust implementation code.
 *
 * Rust code generated around this grammar must target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and must compile without:
 *
 *     unsafe
 *     unsafe blocks
 *     unsafe functions
 *
 * Parser resource limits must be represented through explicit compiler/parser
 * policy rather than hard-coded machine assumptions.
 *
 * ============================================================================
 * 15. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     [ ] It compiles as part of the canonical ANTLR grammar composition.
 *
 *     [ ] It defines no lexer rules.
 *
 *     [ ] It defines no duplicate identifier rules.
 *
 *     [ ] It defines no duplicate type rules.
 *
 *     [ ] It defines no duplicate expression rules.
 *
 *     [ ] It defines no duplicate lambda rules.
 *
 *     [ ] It has no machine-size limits.
 *
 *     [ ] It has no quantum-resource limits.
 *
 *     [ ] It has no hardware-specific assumptions.
 *
 *     [ ] It preserves capture source order.
 *
 *     [ ] It supports zero or more captures.
 *
 *     [ ] It supports explicit capture modes.
 *
 *     [ ] It supports wildcard capture.
 *
 *     [ ] It supports capture aliases.
 *
 *     [ ] It leaves semantic legality to semantic analysis.
 *
 *     [ ] It integrates with lambdas.g4 through closureCaptureClause.
 *
 *     [ ] It does not create a second closure/lambda language.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative syntax tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Cross-domain tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Round-trip tests exist where the frontend supports them.
 *
 * ============================================================================
 */