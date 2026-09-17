/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/never.g4
 *
 * Status:
 *     Canonical modular parser grammar for the source-level Never type.
 *
 * Purpose:
 *     Defines the syntax of Zamani's uninhabited / non-value-producing
 *     source-level type without introducing a second type system, AST,
 *     semantic model, runtime representation, or target-specific meaning.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Lexical authority:
 *
 *     grammar/lexer/tokens.g4
 *
 * This file MUST consume the canonical:
 *
 *     K_NEVER
 *
 * token.
 *
 * It MUST NOT define a lexer token for `never`.
 *
 * Type-composition authority:
 *
 *     grammar/types/types.g4
 *
 * AST authority:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * Canonical AST representation:
 *
 *     TypeExpr::Never
 *
 * Specialized AST façade:
 *
 *     src/frontend/ast/node/types/never.rs
 *
 * This grammar does not introduce another Never AST node.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * This file owns exactly one syntactic concern:
 *
 *     neverType
 *
 * It owns:
 *
 *   - recognition of the source-level Never type;
 *   - its canonical parser-rule boundary;
 *   - its lexical-token dependency;
 *   - its AST mapping contract;
 *   - its semantic-layer boundary;
 *   - its diagnostics boundary;
 *   - its scalability and portability contract.
 *
 * This file does NOT own:
 *
 *   - lexer definitions;
 *   - identifier syntax;
 *   - expression syntax;
 *   - function declarations;
 *   - return statements;
 *   - `throw`;
 *   - `panic`;
 *   - divergence analysis;
 *   - unreachable-code analysis;
 *   - control-flow analysis;
 *   - exhaustiveness checking;
 *   - type inference;
 *   - type unification;
 *   - type coercion;
 *   - generic substitution;
 *   - ownership;
 *   - borrowing;
 *   - resource allocation;
 *   - hardware selection;
 *   - quantum resources;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - runtime representation;
 *   - ABI layout.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/lexer/tokens.g4
 *          |
 *          | K_NEVER
 *          v
 *     grammar/types/never.g4
 *          |
 *          | neverType
 *          v
 *     grammar/types/types.g4
 *          |
 *          v
 *     frontend parser
 *          |
 *          v
 *     TypeExpr::Never
 *          |
 *          v
 *     structural AST validation
 *          |
 *          v
 *     semantic type analysis
 *          |
 *          v
 *     semantic Never / bottom / uninhabited representation
 *          |
 *          v
 *     canonical semantic model / ZUIR
 *          |
 *          v
 *     domain-specific lowering
 *
 * The exact semantic interpretation of Never belongs downstream.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Never is a semantic type, not a hardware capability.
 *
 * This grammar therefore introduces no machine-dependent limit.
 *
 * It MUST NOT contain or imply:
 *
 *     MAX_TYPE_DEPTH
 *     MAX_NEVER_TYPES
 *     MAX_FUNCTIONS
 *     MAX_EXPRESSIONS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_RESOURCE_COUNT
 *
 * A program may contain arbitrarily many syntactic occurrences of Never,
 * subject only to implementation resource policies outside the language
 * semantics.
 *
 * Examples:
 *
 *     fn terminate() -> never {
 *         ...
 *     }
 *
 *     fn impossible() -> never {
 *         ...
 *     }
 *
 *     Result<T, never>
 *
 *     Option<never>
 *
 *     List<never>
 *
 * The grammar does not impose a finite number of such declarations,
 * expressions, functions, modules, generic instantiations, or nested types.
 *
 * ============================================================================
 * PORTABILITY CONTRACT
 * ============================================================================
 *
 * `never` describes source-level program meaning.
 *
 * It does not select:
 *
 *   - CPU;
 *   - GPU;
 *   - FPGA;
 *   - ASIC;
 *   - QPU;
 *   - simulator;
 *   - physical qubit;
 *   - network node;
 *   - accelerator;
 *   - memory bank;
 *   - vendor runtime;
 *   - ABI;
 *   - operating system.
 *
 * A Never-typed program therefore remains target-independent.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/lexer/tokens.g4
 *
 * Required token:
 *
 *     K_NEVER
 *
 * Current canonical lexical spelling:
 *
 *     never
 *
 * This grammar MUST use:
 *
 *     K_NEVER
 *
 * rather than:
 *
 *     'never'
 *
 * and MUST NOT introduce:
 *
 *     NEVER
 *
 * as a competing token.
 *
 * The lexer remains the single authority for keyword recognition.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Parser recognition:
 *
 *     neverType
 *         |
 *         v
 *     TypeExpr::Never
 *
 * The grammar must not create:
 *
 *     NeverAst
 *     NeverNode
 *     BottomTypeNode
 *     UninhabitedTypeNode
 *
 * as alternative AST representations.
 *
 * The repository's canonical source-level type representation is `TypeExpr`.
 *
 * The specialized `NeverType` façade may wrap `TypeExpr::Never`, but the
 * parser does not need to construct that façade directly.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Syntax establishes only:
 *
 *     "the programmer wrote the Never type"
 *
 * Semantic analysis determines what that means in a particular language
 * context.
 *
 * Possible semantic uses may include:
 *
 *   - non-returning functions;
 *   - diverging computation;
 *   - unreachable values;
 *   - impossible states;
 *   - bottom-type relationships;
 *   - control-flow termination;
 *   - exhaustiveness reasoning;
 *   - type compatibility.
 *
 * This grammar MUST NOT decide any of those meanings.
 *
 * In particular:
 *
 *     never
 *
 * must NOT automatically mean:
 *
 *     throw
 *     panic
 *     abort
 *     cancellation
 *     hardware failure
 *     QEC failure
 *     runtime error
 *     process termination
 *
 * Those are distinct semantic/runtime concepts.
 *
 * ============================================================================
 * ERROR / FAILURE SEPARATION
 * ============================================================================
 *
 * Never is a TYPE.
 *
 * It is therefore distinct from:
 *
 *     Result<T, E>
 *     Option<T>
 *     Err(...)
 *     throw
 *     panic
 *     exception
 *     cancellation
 *     fault
 *
 * Examples:
 *
 *     Result<int, Error>
 *
 * describes a computation that can produce either an integer or an error.
 *
 *     never
 *
 * describes a type with no ordinary inhabitant according to the language's
 * semantic model.
 *
 * This grammar does not collapse those concepts.
 *
 * ============================================================================
 * RESULT INTEGRATION
 * ============================================================================
 *
 * Never may legally appear wherever `typeExpression` is permitted.
 *
 * Therefore a separate Result grammar does not need special handling for
 * Never.
 *
 * Conceptually:
 *
 *     Result<T, never>
 *
 * is parsed through:
 *
 *     resultType
 *         |
 *         +-- typeExpression
 *                 |
 *                 +-- neverType
 *
 * The Result grammar owns Result syntax.
 *
 * This file owns only Never syntax.
 *
 * Neither grammar may duplicate the other's rule.
 *
 * ============================================================================
 * OPTIONAL INTEGRATION
 * ============================================================================
 *
 * Never may participate in generic or postfix type composition where the
 * broader type grammar permits it.
 *
 * Examples may include:
 *
 *     never?
 *     Option<never>
 *
 * Whether such constructions are semantically useful or redundant is a
 * semantic/type-system decision.
 *
 * This grammar does not reject them merely because they may be semantically
 * unusual.
 *
 * ============================================================================
 * FUNCTION INTEGRATION
 * ============================================================================
 *
 * Function return types consume the broader `typeExpression` rule.
 *
 * Therefore:
 *
 *     fn terminate() -> never
 *
 * is composed by:
 *
 *     function grammar
 *          |
 *          v
 *     typeExpression
 *          |
 *          v
 *     neverType
 *
 * This file must not define function syntax.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Never is a normal type expression from the perspective of generic
 * composition.
 *
 * It may therefore occur wherever a generic type argument accepts a
 * type-expression:
 *
 *     Box<never>
 *     Result<T, never>
 *     Container<never>
 *
 * Generic arity, substitution, bounds and variance remain outside this file.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Never is domain-neutral.
 *
 * It may occur in a quantum program without acquiring quantum-specific
 * semantics.
 *
 * This grammar must not interpret:
 *
 *     never
 *
 * as:
 *
 *     missing qubit
 *     failed gate
 *     unavailable QPU
 *     QEC failure
 *     ZQN fault
 *     scheduler rejection
 *     routing failure
 *     calibration failure
 *
 * Those concepts belong to the corresponding downstream systems.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * Never must not create a second quantum representation.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Never is equally valid in programs containing:
 *
 *   - HDL;
 *   - hardware intent;
 *   - accelerator descriptions;
 *   - embedded computation;
 *   - distributed computation;
 *   - AI/ML;
 *   - networking;
 *   - scientific computation.
 *
 * No hardware-specific interpretation is permitted here.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Never does not allocate resources.
 *
 * It does not consume, reserve, discover, negotiate, or select:
 *
 *   - memory;
 *   - processors;
 *   - accelerators;
 *   - qubits;
 *   - nodes;
 *   - network links;
 *   - storage;
 *   - power;
 *   - thermal budget.
 *
 * Resource requirements and capability negotiation belong to the resource,
 * hardware, compiler, and execution layers.
 *
 * ============================================================================
 * SOURCE SPAN CONTRACT
 * ============================================================================
 *
 * The parser must preserve the source span corresponding to the K_NEVER token
 * when constructing the TypeExpr::Never node.
 *
 * This enables:
 *
 *   - diagnostics;
 *   - IDE navigation;
 *   - source formatting;
 *   - error reporting;
 *   - provenance;
 *   - tooling;
 *   - compatibility analysis.
 *
 * The grammar itself does not define source-span storage; that remains an AST
 * and parser responsibility.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Lexical failure:
 *
 *     handled by grammar/lexer/tokens.g4 and the lexer implementation.
 *
 * Syntactic failure:
 *
 *     handled by the parser when a type is expected but K_NEVER is not
 *     present where required.
 *
 * Semantic failure:
 *
 *     handled by semantic analysis.
 *
 * Examples of semantic—not grammar—diagnostics:
 *
 *   - invalid use of Never;
 *   - invalid coercion;
 *   - invalid generic bound;
 *   - invalid return-flow relationship;
 *   - invalid pattern/exhaustiveness relationship.
 *
 * No semantic diagnostic should be encoded as an alternative parser rule.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler consumes:
 *
 *     TypeExpr::Never
 *
 * after parsing and structural validation.
 *
 * Compiler stages may use Never information for:
 *
 *   - control-flow reasoning;
 *   - unreachable-path elimination;
 *   - type compatibility;
 *   - specialization;
 *   - optimization;
 *   - IR lowering.
 *
 * Those transformations must preserve language semantics.
 *
 * This grammar must not prescribe how those transformations are implemented.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * This grammar has no runtime representation requirement.
 *
 * A runtime may represent Never by:
 *
 *   - no materialized value;
 *   - a control-flow terminator;
 *   - another implementation-defined representation.
 *
 * The choice belongs downstream.
 *
 * The parser must never encode a runtime ABI decision.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * IDEs, formatters, documentation generators, syntax highlighters, linters,
 * refactoring tools and source analyzers should recognize `neverType` through
 * this canonical parser rule and the canonical K_NEVER token.
 *
 * No tooling-specific duplicate Never grammar is permitted.
 *
 * ============================================================================
 * VERSION / COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Adding or changing the syntax of Never is a language compatibility change.
 *
 * The canonical token remains:
 *
 *     K_NEVER
 *
 * The canonical parser rule remains:
 *
 *     neverType
 *
 * The canonical AST representation remains:
 *
 *     TypeExpr::Never
 *
 * A future spelling may be added only through an explicit compatibility or
 * language-version decision.
 *
 * It must not silently replace the canonical spelling.
 *
 * ============================================================================
 * CURRENT SPELLING INTEGRATION NOTE
 * ============================================================================
 *
 * The repository contains an existing documentation discrepancy:
 *
 *   - grammar/lexer/tokens.g4 defines K_NEVER for the spelling `never`;
 *   - existing type/specification material refers to Never;
 *   - the frontend NeverType documentation describes the canonical TypeExpr
 *     source spelling as `!`.
 *
 * This grammar deliberately follows the canonical lexer contract requested
 * for this modular grammar:
 *
 *     K_NEVER
 *
 * Therefore the source spelling accepted by this rule is currently:
 *
 *     never
 *
 * The `!` spelling must NOT be silently introduced here.
 *
 * If `!` is to become an additional or canonical spelling, that requires a
 * coordinated lexical/specification/AST-formatting compatibility change in
 * the appropriate files.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     no finite resource limits;
 *     no machine identifiers;
 *     no physical IDs;
 *     no fixed topology;
 *     no fixed hardware count;
 *     no fixed type nesting limit;
 *     no fixed generic arity;
 *     no fixed function count;
 *     no fixed program size.
 *
 * The only terminal accepted by this grammar is the canonical lexer token:
 *
 *     K_NEVER
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *   - performs no I/O;
 *   - performs no network access;
 *   - executes no source code;
 *   - allocates no runtime resources;
 *   - accesses no hardware;
 *   - contains no Rust;
 *   - contains no unsafe operation.
 *
 * Generated parser/compiler/runtime integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     no unsafe code
 *
 * ============================================================================
 * PERFORMANCE / SCALABILITY CONTRACT
 * ============================================================================
 *
 * Parsing a Never type is constant-size with respect to the Never construct
 * itself.
 *
 * The rule does not introduce recursion.
 *
 * It therefore adds no intrinsic complexity proportional to:
 *
 *   - machine size;
 *   - qubit count;
 *   - CPU count;
 *   - GPU count;
 *   - node count;
 *   - memory capacity.
 *
 * Any parser safety budget must be an implementation policy and must not
 * become a language semantic restriction.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax cases:
 *
 *     never
 *
 *     fn f() -> never { ... }
 *
 *     Result<int, never>
 *
 *     Option<never>
 *
 *     Box<never>
 *
 * Negative cases are owned by the broader parser/test system and include:
 *
 *     an incomplete type position where K_NEVER is required but absent;
 *     malformed surrounding generic syntax;
 *     malformed surrounding function syntax.
 *
 * The Never grammar itself should not reject syntactically valid composition
 * merely because a semantic checker may later determine that a construction
 * is meaningless.
 *
 * Boundary cases:
 *
 *     deeply nested generic contexts containing never;
 *     multiple Never occurrences;
 *     Never in classical, quantum, HDL, distributed and hybrid source;
 *     Never as a generic argument;
 *     Never as a function return type.
 *
 * Scalability cases:
 *
 *     arbitrarily many source-level occurrences of never;
 *     arbitrarily deep type composition, subject only to configurable parser
 *     resource policy rather than a language-level limit.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when all of the following are true:
 *
 *   [x] Owns only neverType syntax.
 *   [x] Uses the canonical K_NEVER token.
 *   [x] Defines no lexer rules.
 *   [x] Introduces no second AST.
 *   [x] Maps to TypeExpr::Never.
 *   [x] Composes through types.g4.
 *   [x] Remains independent of Result/Option/function/statement grammars.
 *   [x] Has no machine-specific limits.
 *   [x] Has no hardware-specific semantics.
 *   [x] Has no quantum-specific semantics.
 *   [x] Has no runtime representation.
 *   [x] Has explicit source-span integration.
 *   [x] Has explicit diagnostic boundaries.
 *   [x] Has explicit compiler/runtime/tooling contracts.
 *   [x] Has positive/negative/boundary/scalability test contracts.
 *   [x] Preserves POCO-REAF.
 *   [x] Does not require unsafe Rust.
 *
 * ============================================================================
 */

parser grammar Never;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 *
 * Canonical source-level Never type.
 *
 * The lexical spelling is owned by:
 *
 *     grammar/lexer/tokens.g4
 *
 * and is consumed through:
 *
 *     K_NEVER
 *
 * No literal keyword is duplicated here.
 */
neverType
    : K_NEVER
    ;