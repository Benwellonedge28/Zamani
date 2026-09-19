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
 * Status:
 *     Canonical closure-capture parser grammar.
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *
 * Safety:
 *     The Zamani implementation MUST use safe Rust only.
 *     No unsafe Rust is required or permitted.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE SYNTAX of explicit closure-capture clauses.
 *
 * Examples:
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
 * The grammar records source-level capture intent.
 *
 * It does NOT decide:
 *
 *     ownership
 *     borrowing
 *     lifetimes
 *     environment representation
 *     stack/heap placement
 *     closure conversion
 *     ABI
 *     calling convention
 *     execution placement
 *     scheduling
 *     hardware
 *     quantum mapping
 *     QEC
 *     ZQN
 *     routing
 *     runtime representation
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     closureCaptureClause
 *     closureCaptureList
 *     closureCapture
 *     closureCaptureMode
 *     closureCaptureTarget
 *     closureCaptureAlias
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lambdaExpression
 *     lambda parameters
 *     ordinary function declarations
 *     function parameters
 *     function return types
 *     function types
 *     expressions
 *     statements
 *     blocks
 *     identifiers
 *     qualified names
 *     types
 *     effects
 *     capabilities
 *     resources
 *     ownership semantics
 *     borrow checking
 *     lifetime checking
 *     closure conversion
 *     environment layout
 *     ABI
 *     calling conventions
 *     concurrency semantics
 *     distributed execution
 *     quantum semantics
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     optimization
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file is the sole owner of closure-capture syntax.
 *
 * In particular, these rules MUST NOT be redefined elsewhere:
 *
 *     closureCaptureClause
 *     closureCaptureList
 *     closureCapture
 *     closureCaptureMode
 *     closureCaptureTarget
 *     closureCaptureAlias
 *
 * grammar/expressions/closures.g4 is an integration adapter only.
 *
 * grammar/expressions/lambdas.g4 consumes closureCaptureClause.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
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
 *       v
 *     closureCaptureClause
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     name resolution
 *       |
 *       v
 *     type analysis
 *       |
 *       v
 *     ownership / borrowing / lifetime analysis
 *       |
 *       v
 *     effect / capability / resource analysis
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +-----------------------+
 *       |                       |
 *       v                       v
 *   classical IR           quantum::ir
 *       |                       |
 *       +-----------+-----------+
 *                   |
 *                   v
 *        optimization / lowering
 *                   |
 *          +--------+--------+
 *          |                 |
 *          v                 v
 *       routing          scheduling
 *          |                 |
 *          +--------+--------+
 *                   |
 *                   v
 *               resilience
 *                   |
 *                   v
 *                  ZQN
 *                   |
 *                   v
 *                  HAL
 *                   |
 *                   v
 *          target realization
 *
 * This grammar participates only in the source/parser portion.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Closure capture represents SOURCE-LEVEL DATA DEPENDENCIES.
 *
 * It does not represent physical execution resources.
 *
 * Therefore this grammar MUST NOT contain:
 *
 *     MAX_CAPTURES
 *     MAX_CLOSURES
 *     MAX_ENVIRONMENT_SIZE
 *     MAX_PARAMETERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *
 * Nor may an equivalent hidden restriction be introduced.
 *
 * The language therefore remains capable of representing programs ranging
 * from tiny embedded computations to arbitrarily large computations subject
 * only to actual implementation/resource availability.
 *
 * ============================================================================
 * CAPTURE CLAUSE
 * ============================================================================
 *
 * Canonical form:
 *
 *     capture { ... }
 *
 * The braces deliberately distinguish explicit capture syntax from lambda
 * parameter syntax.
 *
 * ============================================================================
 * CAPTURE LIST
 * ============================================================================
 *
 * The list is structurally unbounded by the language grammar.
 *
 * Examples:
 *
 *     capture { x }
 *     capture { x, y }
 *     capture { x, y, z }
 *     capture { x, y, z, }
 *
 * A trailing comma is accepted.
 *
 * ============================================================================
 * CAPTURE ENTRY
 * ============================================================================
 *
 * A capture entry consists of:
 *
 *     zero or more capture modes
 *     one capture target
 *     optional alias
 *
 * Examples:
 *
 *     x
 *     move x
 *     ref x
 *     mut x
 *     move x as y
 *     ref x as y
 *     mut x as y
 *     *
 *     move *
 *
 * The grammar intentionally permits syntactic combinations such as:
 *
 *     move ref x
 *
 * so that semantic analysis can issue the correct language-level diagnostic.
 *
 * The parser does not attempt to encode ownership rules.
 *
 * ============================================================================
 * WILDCARD CAPTURE
 * ============================================================================
 *
 * `*` means:
 *
 *     use the language-defined semantic default capture policy.
 *
 * It MUST NOT mean:
 *
 *     capture all machine resources
 *     capture all memory
 *     capture all devices
 *     capture all qubits
 *     capture every runtime object
 *
 * The actual capture set is determined by semantic analysis.
 *
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * An explicit capture target is an identifier.
 *
 * Example:
 *
 *     capture { value }
 *
 * Name resolution determines what the identifier denotes.
 *
 * It may resolve to:
 *
 *     local binding
 *     parameter
 *     outer binding
 *     pattern binding
 *     imported binding
 *     another language-defined lexical binding
 *
 * This grammar does not perform name resolution.
 *
 * ============================================================================
 * ALIAS
 * ============================================================================
 *
 * An optional alias may be supplied:
 *
 *     capture { value as captured }
 *
 * The alias changes source-level naming only.
 *
 * It does not prescribe:
 *
 *     storage location
 *     memory layout
 *     environment layout
 *     register allocation
 *     device placement
 *
 * ============================================================================
 * CAPTURE MODES
 * ============================================================================
 *
 * The currently standardized source-level modes are:
 *
 *     move
 *     ref
 *     mut
 *
 * These are requests/intent at the syntax level.
 *
 * Semantic analysis determines whether a particular combination is legal.
 *
 * In particular:
 *
 *     move
 *
 * may request ownership transfer semantics.
 *
 *     ref
 *
 * may request reference/borrow semantics.
 *
 *     mut
 *
 * may request mutable access semantics.
 *
 * Their actual implementation is NOT defined here.
 *
 * ============================================================================
 * DUPLICATES
 * ============================================================================
 *
 * Syntactic duplicate captures are accepted:
 *
 *     capture { x, x }
 *
 * The semantic phase decides whether this is:
 *
 *     redundant
 *     conflicting
 *     ambiguous
 *     invalid
 *
 * This prevents syntax from becoming coupled to name-resolution state.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It MUST NOT contain lexer rules.
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Required existing tokens:
 *
 *     AS
 *     MUT
 *     STAR
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     IDENTIFIER
 *
 * Required additions to the canonical lexer:
 *
 *     CAPTURE
 *     MOVE
 *     REF
 *
 * No alternate token names are introduced here.
 *
 * In particular, this grammar does NOT use:
 *
 *     K_CAPTURE
 *     K_MOVE
 *     K_REF
 *     ZamaniTokens
 *
 * The repository's canonical lexer vocabulary is `ZamaniLexer`.
 *
 * ============================================================================
 * LEXICAL OWNERSHIP
 * ============================================================================
 *
 * `CAPTURE`, `MOVE`, and `REF` are language keywords because their presence
 * changes the syntactic interpretation of a closure capture entry.
 *
 * They therefore belong in the canonical keyword section of:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * They MUST NOT be defined locally here.
 *
 * `MUT` and `AS` are already canonical lexer tokens and MUST be reused.
 *
 * ============================================================================
 * NAME OWNERSHIP
 * ============================================================================
 *
 * This grammar does not define:
 *
 *     IDENTIFIER
 *     qualifiedName
 *
 * Those remain owned by the canonical lexical/name system.
 *
 * ============================================================================
 * TYPE OWNERSHIP
 * ============================================================================
 *
 * Capture syntax contains no type syntax.
 *
 * For example:
 *
 *     capture { x }
 *
 * does not become:
 *
 *     capture { x: SomeType }
 *
 * unless a future language specification explicitly adds typed captures.
 *
 * Capture types are determined from the captured binding by semantic/type
 * analysis.
 *
 * This keeps closure syntax independent of:
 *
 *     classical types
 *     quantum types
 *     HDL types
 *     tensor types
 *     resource types
 *     capability types
 *     future domain types
 *
 * ============================================================================
 * EFFECT OWNERSHIP
 * ============================================================================
 *
 * Capture syntax does not define effects.
 *
 * Effects belong to:
 *
 *     grammar/effects/
 *
 * and the semantic effect system.
 *
 * A captured value may itself have effect-related semantics, but that is not
 * decided by this grammar.
 *
 * ============================================================================
 * RESOURCE OWNERSHIP
 * ============================================================================
 *
 * Captures are not resource declarations.
 *
 * This file therefore contains no:
 *
 *     memory sizes
 *     accelerator counts
 *     device IDs
 *     topology
 *     placement
 *     timing
 *     bandwidth
 *     power
 *     thermal constraints
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Captured bindings may eventually be used by:
 *
 *     async tasks
 *     futures
 *     actors
 *     parallel computations
 *     distributed computations
 *
 * Whether a particular capture is legal for those execution models is
 * determined by semantic ownership/borrowing/effect/resource analysis.
 *
 * No thread/core/device assumption appears in this grammar.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Capturing a value does not imply that the value is:
 *
 *     serializable
 *     transferable
 *     remotely accessible
 *     replicated
 *     migratable
 *
 * Distributed semantics determine those properties.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A closure may capture a quantum-semantic value if the type/semantic systems
 * allow it:
 *
 *     capture { q }
 *
 * This grammar MUST NOT introduce special quantum capture syntax such as:
 *
 *     capture qubit
 *     capture physical-qubit
 *     capture q[0]
 *
 * A quantum value remains a normal source binding at this syntactic layer.
 *
 * If the resulting computation is quantum-semantic, downstream lowering may
 * eventually reach:
 *
 *     quantum::ir
 *
 * The closure grammar itself MUST NOT create or modify quantum IR.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * CLASSICAL / HDL / AI / DATA INTEGRATION
 * ============================================================================
 *
 * The same capture syntax can capture:
 *
 *     classical values
 *     tensors
 *     datasets
 *     AI model values
 *     HDL elaboration values
 *     hardware intent values
 *     resource descriptions
 *     distributed values
 *     networking values
 *     security-related values
 *
 * No domain-specific closure grammar is necessary.
 *
 * ============================================================================
 * LAMBDA INTEGRATION
 * ============================================================================
 *
 * `grammar/expressions/lambdas.g4` owns:
 *
 *     lambdaExpression
 *     lambdaParameterClause
 *     lambdaReturnTypeClause
 *     lambdaBody
 *
 * This file owns:
 *
 *     closureCaptureClause
 *
 * Therefore lambda composition is:
 *
 *     closureCaptureClause?
 *     lambdaModifier*
 *     lambdaParameterClause
 *     lambdaReturnTypeClause?
 *     lambdaBody
 *
 * The existing `lambdas.g4` already consumes `closureCaptureClause?`.
 *
 * No capture rules are to be copied into `lambdas.g4`.
 *
 * ============================================================================
 * EXPRESSION-LAYER INTEGRATION
 * ============================================================================
 *
 * `grammar/expressions/closures.g4` is an adapter/integration point.
 *
 * It MUST NOT redefine this grammar.
 *
 * It should import or expose the canonical closure rule through the expression
 * composition layer.
 *
 * ============================================================================
 * FUNCTION INTEGRATION
 * ============================================================================
 *
 * `grammar/functions/functions.g4` owns function declarations.
 *
 * Closure capture syntax does not belong in:
 *
 *     functions.g4
 *
 * except where a future language construct explicitly makes a function
 * declaration capture an environment.
 *
 * Ordinary function declarations do not implicitly acquire closure capture
 * syntax.
 *
 * ============================================================================
 * FUNCTION RETURN INTEGRATION
 * ============================================================================
 *
 * `grammar/functions/returns.g4` owns:
 *
 *     functionReturnClause
 *
 * This file MUST NOT define or reference a second return-clause grammar.
 *
 * There is no conflict between:
 *
 *     closure capture syntax
 *
 * and:
 *
 *     function return type syntax
 *
 * because they belong to different language constructs.
 *
 * ============================================================================
 * FUNCTION TYPE INTEGRATION
 * ============================================================================
 *
 * `grammar/types/function.g4` owns function-type syntax.
 *
 * Closure capture does not modify function-type syntax.
 *
 * A closure's inferred callable type is determined semantically.
 *
 * ============================================================================
 * OWNERSHIP / BORROWING INTEGRATION
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     target exists
 *     target is in scope
 *     target can be captured
 *     capture mode is legal
 *     ownership transition is legal
 *     reference lifetime is valid
 *     mutable access is valid
 *     aliases do not conflict
 *
 * None of these are parser responsibilities.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve, without semantic loss:
 *
 *     capture clause span
 *     capture-entry order
 *     capture-mode order
 *     target identifier
 *     wildcard target
 *     alias identifier
 *
 * Conceptual frontend representation:
 *
 *     ClosureCaptureClause {
 *         captures: Vec<ClosureCapture>,
 *         source_span: Span,
 *     }
 *
 *     ClosureCapture {
 *         modes: Vec<CaptureMode>,
 *         target: CaptureTarget,
 *         alias: Option<Identifier>,
 *         source_span: Span,
 *     }
 *
 *     CaptureTarget:
 *         Identifier(Identifier)
 *         Wildcard
 *
 *     CaptureMode:
 *         Move
 *         Ref
 *         Mut
 *
 * The exact Rust representation remains owned by the existing frontend AST.
 *
 * This grammar MUST NOT introduce a second AST hierarchy.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST validate:
 *
 *     target existence
 *     lexical scope
 *     duplicate captures
 *     conflicting capture modes
 *     mutable capture legality
 *     reference legality
 *     move legality
 *     lifetime validity
 *     alias collisions
 *     type compatibility
 *     ownership compatibility
 *     borrow compatibility
 *     effect compatibility
 *     capability compatibility
 *     concurrency compatibility
 *     distributed execution compatibility
 *     quantum-value compatibility
 *
 * Wildcard capture resolution is semantic.
 *
 * The grammar does not expand `*`.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar emits no IR.
 *
 * Capture information first enters the domain-neutral semantic representation.
 *
 * It may then participate in:
 *
 *     classical IR
 *     quantum::ir
 *     hybrid representations
 *     hardware/co-design representations
 *     distributed representations
 *
 * according to the computation's actual semantics.
 *
 * No closure-specific quantum IR is permitted.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler transformations may include:
 *
 *     closure conversion
 *     lambda lifting
 *     environment specialization
 *     inlining
 *     specialization
 *     escape analysis
 *     allocation optimization
 *     parallelization
 *     distribution
 *     accelerator lowering
 *
 * Those transformations must not require changes to this grammar.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * The runtime may represent a closure using:
 *
 *     stack environments
 *     heap environments
 *     static environments
 *     function objects
 *     function pointers
 *     specialized environments
 *     distributed representations
 *     accelerator-specific representations
 *
 * The source grammar does not prescribe any of them.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given identical source tokens and grammar/version configuration, parsing
 * must produce identical:
 *
 *     parse-tree structure
 *     token ordering
 *     capture ordering
 *     source spans
 *
 * No:
 *
 *     time
 *     randomness
 *     filesystem state
 *     network state
 *     hardware state
 *     runtime state
 *
 * may influence parsing.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors include:
 *
 *     missing CAPTURE
 *     missing LBRACE
 *     missing RBRACE
 *     malformed capture entry
 *     missing capture target
 *     malformed alias
 *     missing alias identifier
 *     malformed comma separation
 *
 * Semantic errors belong downstream:
 *
 *     unknown binding
 *     invalid mode
 *     conflicting modes
 *     duplicate capture
 *     invalid mutable capture
 *     invalid reference
 *     invalid move
 *     invalid alias
 *     lifetime violation
 *     ownership violation
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The grammar must accept:
 *
 *     capture { }
 *
 *     capture { x }
 *
 *     capture { x, y }
 *
 *     capture { x, y, }
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
 *     capture { x, move y, ref z as alias }
 *
 *     capture {
 *         x,
 *         move y,
 *         ref z as alias,
 *     }
 *
 * The empty capture list is intentionally syntactically valid.
 *
 * Semantic analysis decides whether an empty explicit capture clause is useful
 * or redundant.
 *
 * ============================================================================
 * NEGATIVE SYNTAX TEST CONTRACT
 * ============================================================================
 *
 * The grammar must reject malformed source such as:
 *
 *     capture
 *
 *     capture {
 *
 *     capture }
 *
 *     capture { x
 *
 *     capture { x,, y }
 *
 *     capture { , x }
 *
 *     capture { x as }
 *
 *     capture { x as 123 }
 *
 *     capture { move }
 *
 *     capture { ref }
 *
 *     capture { mut }
 *
 *     capture { move as x }
 *
 *     capture { ref as x }
 *
 *     capture { mut as x }
 *
 * The semantic validity of:
 *
 *     capture { move ref x }
 *
 * is deliberately NOT a syntax concern.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     zero captures
 *     one capture
 *     many captures
 *     many aliases
 *     multiple modes
 *     wildcard capture
 *     nested lambdas
 *     nested closures
 *     capture inside generic functions
 *     capture of generic values
 *     capture of quantum values
 *     capture of tensors
 *     capture of hardware-intent values
 *     capture in async lambdas
 *     capture in distributed computations
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The grammar must not impose source-language limits on:
 *
 *     capture count
 *     closure count
 *     lambda count
 *     nesting depth
 *     parameter count
 *     source size
 *
 * Tests may exercise progressively larger valid inputs until an explicit
 * implementation/resource budget is reached.
 *
 * That budget is NOT a grammar-language limit.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_CAPTURES
 *     MAX_CLOSURES
 *     MAX_ENVIRONMENT_SIZE
 *     MAX_PARAMETERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * None appear in the grammar.
 *
 * No physical resource identifier is recognized specially by this grammar.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing closure syntax must never:
 *
 *     execute code
 *     inspect hardware
 *     access files
 *     access networks
 *     access credentials
 *     invoke a runtime
 *     invoke a QPU
 *     invoke an accelerator
 *     invoke an HDL simulator
 *
 * The parser is purely syntactic.
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no embedded Rust actions.
 *
 * Integration code MUST compile under:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * without:
 *
 *     unsafe
 *     unsafe blocks
 *     unsafe functions
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing lambda syntax:
 *
 *     |x| expression
 *
 * remains unchanged.
 *
 * Explicit capture syntax is an additive construct:
 *
 *     capture { x } |y| expression
 *
 * The closure grammar does not replace the existing lambda syntax.
 *
 * The exact placement of `closureCaptureClause` is owned by the lambda
 * composition contract in `expressions/lambdas.g4`.
 *
 * Existing parser users that do not use explicit captures continue to parse
 * exactly as before.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It is a parser grammar.
 *     [x] It uses tokenVocab = ZamaniLexer.
 *     [x] It contains no lexer rules.
 *     [x] It owns closure-capture syntax only.
 *     [x] It does not own lambda syntax.
 *     [x] It does not own function return syntax.
 *     [x] It does not own function declaration syntax.
 *     [x] It does not own type syntax.
 *     [x] It does not own expression syntax.
 *     [x] It does not own semantic ownership rules.
 *     [x] It does not own runtime representation.
 *     [x] It supports zero or more captures.
 *     [x] It supports explicit capture modes.
 *     [x] It supports wildcard capture.
 *     [x] It supports aliases.
 *     [x] It supports trailing commas.
 *     [x] It preserves source order.
 *     [x] It contains no hardware limits.
 *     [x] It contains no quantum limits.
 *     [x] It introduces no second quantum IR.
 *     [x] It has a predetermined AST contract.
 *     [x] It has a predetermined semantic contract.
 *     [x] It has a predetermined IR contract.
 *     [x] It has a predetermined compiler contract.
 *     [x] It has a predetermined runtime contract.
 *     [x] It has positive-test requirements.
 *     [x] It has negative-test requirements.
 *     [x] It has boundary-test requirements.
 *     [x] It has scalability-test requirements.
 *     [x] It has compatibility requirements.
 *     [x] It has a hard-coding audit.
 *     [x] It requires safe Rust.
 *
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
 * Public parser entry point.
 *
 * Canonical:
 *
 *     capture { ... }
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
 * The grammar deliberately has no finite capture count.
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
 * A capture entry contains:
 *
 *     zero or more modes
 *     one target
 *     optional alias
 */
closureCapture
    : closureCaptureMode*
      closureCaptureTarget
      closureCaptureAlias?
    ;


/*
 * ============================================================================
 * 4. CAPTURE MODE
 * ============================================================================
 *
 * Semantic analysis decides whether combinations such as:
 *
 *     move ref x
 *
 * are legal.
 *
 * The parser remains lossless and deterministic.
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
 * Explicit source binding or semantic-default wildcard.
 */
closureCaptureTarget
    : IDENTIFIER
    | STAR
    ;


/*
 * ============================================================================
 * 6. CAPTURE ALIAS
 * ============================================================================
 */
closureCaptureAlias
    : AS IDENTIFIER
    ;