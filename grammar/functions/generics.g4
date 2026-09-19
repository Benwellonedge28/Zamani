/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/generics.g4
 *
 * Status:
 *     Canonical function-generic declaration grammar.
 *
 * Purpose:
 *     Defines the source syntax for generic parameters declared by functions
 *     and other callable declarations that explicitly reuse this grammar.
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no Rust implementation code.
 *     Zamani compiler/frontend/runtime Rust MUST use safe Rust only.
 *     No unsafe Rust is required by this grammar.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Canonical pipeline:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     parser composition
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural + semantic analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *      classical IR          quantum::ir          HDL/hardware IR
 *          |                      |                      |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                      optimization / lowering
 *                                 |
 *                      routing / scheduling
 *                                 |
 *                       resilience / QEC / ZQN
 *                                 |
 *                                HAL
 *                                 |
 *                         target realization
 *
 * Generic parameter syntax belongs entirely to the source-language layer.
 *
 * Generic parameters MUST NOT encode:
 *
 *     - CPU counts;
 *     - GPU counts;
 *     - FPGA counts;
 *     - ASIC counts;
 *     - QPU counts;
 *     - qubit counts;
 *     - memory capacities;
 *     - register widths;
 *     - physical addresses;
 *     - physical device identifiers;
 *     - hardware topology;
 *     - deployment size;
 *     - scheduling decisions;
 *     - routing decisions;
 *     - calibration;
 *     - QEC implementation;
 *     - ZQN implementation;
 *     - HAL implementation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - functionGenericParameters
 *     - functionGenericParameter
 *     - functionGenericParameterName
 *     - functionGenericParameterBounds
 *     - functionGenericParameterBound
 *     - generic parameter source ordering
 *     - generic bound source ordering
 *     - generic parameter list delimiters
 *     - trailing-comma syntax for function generic declarations
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules
 *     - identifiers
 *     - type-expression implementation
 *     - generic type applications
 *     - type inference
 *     - generic substitution
 *     - monomorphization
 *     - specialization
 *     - overload resolution
 *     - trait/interface solving
 *     - constraint solving
 *     - semantic validation
 *     - ABI selection
 *     - calling conventions
 *     - hardware selection
 *     - resource allocation
 *     - quantum allocation
 *     - routing
 *     - scheduling
 *     - optimization
 *     - QEC
 *     - ZQN
 *     - HAL
 *     - runtime execution
 *
 * ============================================================================
 * TOKEN AUTHORITY
 * ============================================================================
 *
 * Parser-facing lexical vocabulary:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Modular lexical definitions that must ultimately feed the canonical lexer:
 *
 *     grammar/lexer/keywords.g4
 *     grammar/lexer/operators.g4
 *     grammar/lexer/punctuation.g4
 *     grammar/lexer/identifiers.g4
 *
 * This parser grammar does NOT define lexer rules.
 *
 * Required existing tokens:
 *
 *     IDENTIFIER
 *     LESS
 *     GREATER
 *     COMMA
 *     PLUS
 *     EXTENDS
 *
 * IMPORTANT:
 *
 *     LESS_THAN
 *     GREATER_THAN
 *     K_EXTENDS
 *
 * are NOT canonical token names in the current lexer.
 *
 * ============================================================================
 * TOKEN VOCABULARY CONTRACT
 * ============================================================================
 *
 * `functions.g4` is already composed against:
 *
 *     ZamaniLexer
 *
 * Therefore this grammar intentionally uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Do not independently switch this file to ZamaniTokens while its parent
 * parser composition remains ZamaniLexer.
 *
 * The repository's separate `grammar/lexer/tokens.g4` vocabulary must
 * eventually be integrated into the canonical ZamaniLexer assembly rather
 * than creating two parser-facing lexical authorities.
 *
 * ============================================================================
 * FUNCTION INTEGRATION
 * ============================================================================
 *
 * `grammar/functions/functions.g4` imports this grammar as:
 *
 *     FunctionGenerics
 *
 * and consumes:
 *
 *     functionGenericParameters?
 *
 * Therefore this file is the sole owner of those parser rules.
 *
 * `functions.g4` owns:
 *
 *     functionName
 *     functionGenericParameters?
 *     parameterList
 *     return type
 *     effects
 *     contracts
 *     implementation/body
 *
 * This file owns only the internal generic parameter syntax.
 *
 * ============================================================================
 * CANONICAL SYNTAX
 * ============================================================================
 *
 * Unbounded source-defined generic parameter lists:
 *
 *     <T>
 *     <T, U>
 *     <T, U, V>
 *
 * Bounded parameters:
 *
 *     <T extends Numeric>
 *
 * Multiple bounds:
 *
 *     <T extends Numeric + Ordered>
 *
 * Multiple parameters:
 *
 *     <T extends Numeric, U extends Serializable>
 *
 * Trailing comma:
 *
 *     <T,>
 *     <T, U,>
 *
 * Empty generic parameter lists are rejected:
 *
 *     <>
 *
 * ============================================================================
 * GENERIC DECLARATION VS GENERIC APPLICATION
 * ============================================================================
 *
 * Declaration:
 *
 *     fn identity<T>(value: T) -> T
 *
 * belongs to this grammar.
 *
 * Application:
 *
 *     Vec<T>
 *     Result<T, E>
 *     Matrix<T>
 *
 * belongs to:
 *
 *     grammar/types/generic.g4
 *
 * or the final type-composition grammar that owns `typeExpression`.
 *
 * This distinction is mandatory.
 *
 * This grammar MUST NOT define:
 *
 *     genericType
 *     genericTypeArguments
 *     genericArgumentList
 *
 * because those belong to generic type application.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The canonical frontend generic representation is the existing generic
 * parameter model.
 *
 * Conceptually:
 *
 *     TypeParameter
 *         name
 *         bounds
 *
 * The parser/AST lowering layer MUST preserve:
 *
 *     - parameter name;
 *     - source span;
 *     - ordered bounds;
 *     - bound source spans;
 *     - declaration ordering.
 *
 * Examples:
 *
 *     <T>
 *
 * becomes:
 *
 *     TypeParameter {
 *         name: T,
 *         bounds: []
 *     }
 *
 *     <T extends Numeric>
 *
 * becomes:
 *
 *     TypeParameter {
 *         name: T,
 *         bounds: [Numeric]
 *     }
 *
 *     <T extends Numeric + Ordered>
 *
 * becomes:
 *
 *     TypeParameter {
 *         name: T,
 *         bounds: [
 *             Numeric,
 *             Ordered
 *         ]
 *     }
 *
 * Bound ordering is source ordering.
 *
 * No second generic AST is permitted.
 *
 * ============================================================================
 * TYPE EXPRESSION CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical:
 *
 *     typeExpression
 *
 * rule.
 *
 * It does NOT redefine:
 *
 *     typeExpression
 *     named types
 *     qualified types
 *     generic type applications
 *     tuple types
 *     array types
 *     references
 *     pointers
 *     function types
 *     quantum types
 *     resource types
 *     hardware types
 *     dependent types
 *     linear types
 *     affine types
 *     effect types
 *
 * A bound is therefore syntactically represented by an ordinary canonical
 * type expression.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "What generic parameters and syntactic bounds did the programmer write?"
 *
 * Semantic analysis answers:
 *
 *     - Is the generic parameter name unique?
 *     - Does the bound exist?
 *     - Is the bound applicable to the parameter?
 *     - Is the bound satisfiable?
 *     - Does an implementation satisfy the bound?
 *     - Are generic substitutions valid?
 *     - Can specialization occur?
 *     - Is monomorphization legal?
 *     - Are all required constraints satisfied?
 *
 * Those questions MUST NOT be implemented in this grammar.
 *
 * ============================================================================
 * BOUND MODEL
 * ============================================================================
 *
 * Canonical inline bound syntax:
 *
 *     T extends Numeric
 *
 *     T extends Numeric + Ordered
 *
 * The `extends` keyword is the canonical function-generic bound introducer.
 *
 * The grammar does not decide whether the referenced type expression is:
 *
 *     - a trait;
 *     - an interface;
 *     - a capability;
 *     - a constraint;
 *     - a structural type;
 *     - a domain-specific type;
 *     - a future constraint abstraction.
 *
 * That interpretation belongs to semantic analysis.
 *
 * ============================================================================
 * WHERE-CLAUSE SEPARATION
 * ============================================================================
 *
 * The repository already contains `WHERE` vocabulary.
 *
 * However, function-level where-clause syntax is a separate concern.
 *
 * This file deliberately does NOT define:
 *
 *     whereClause
 *     wherePredicate
 *
 * because those belong to the function signature/constraint layer.
 *
 * Example future/companion syntax:
 *
 *     fn f<T>(value: T) -> T
 *     where T extends Numeric
 *
 * must be owned by the function constraint grammar, not duplicated here.
 *
 * Inline declaration bounds remain:
 *
 *     fn f<T extends Numeric>(value: T) -> T
 *
 * ============================================================================
 * DEFAULT GENERIC PARAMETERS
 * ============================================================================
 *
 * This grammar deliberately does NOT introduce generic parameter defaults.
 *
 * A source construct such as:
 *
 *     <T = DefaultType>
 *
 * requires an explicit language-wide semantic contract covering:
 *
 *     - generic declaration defaults;
 *     - generic argument omission;
 *     - type inference;
 *     - declaration compatibility;
 *     - overload resolution;
 *     - AST representation;
 *     - semantic substitution;
 *     - diagnostics.
 *
 * It must therefore be introduced as a coordinated feature rather than
 * silently added here.
 *
 * ============================================================================
 * CONST / VALUE GENERIC PARAMETERS
 * ============================================================================
 *
 * This grammar intentionally does NOT invent:
 *
 *     const N: usize
 *
 * generic parameters.
 *
 * The root `Zamani.g4` currently contains a broader genericParameter model
 * that permits value/type forms. The modular function-generic AST currently
 * models type parameters and type bounds.
 *
 * Until the frontend AST and semantic model provide a canonical first-class
 * generic value parameter representation, this file must not accept syntax
 * which would lose information during lowering.
 *
 * This is a correctness boundary, not a limitation on future Zamani.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Generic parameters are domain-neutral.
 *
 * Examples:
 *
 *     fn operate<Q extends QuantumResource>(q: Q) -> Q
 *
 *     fn transform<T extends QuantumState>(state: T) -> T
 *
 * are syntactically valid.
 *
 * This grammar does NOT determine:
 *
 *     - number of qubits;
 *     - logical/physical qubit mapping;
 *     - QPU selection;
 *     - gate set;
 *     - topology;
 *     - calibration;
 *     - noise;
 *     - QEC;
 *     - routing;
 *     - scheduling.
 *
 * After semantic analysis, quantum constructs continue through:
 *
 *     quantum::ir
 *
 * as the canonical quantum semantic boundary.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * The same generic syntax applies to all domains.
 *
 * Examples:
 *
 *     fn process<T extends Numeric>(value: T) -> T
 *
 *     fn execute<H extends HardwareResource>(resource: H) -> H
 *
 *     fn synthesize<M extends HardwareModule>(module: M) -> M
 *
 *     fn distribute<N extends NodeResource>(node: N) -> N
 *
 * No domain-specific generic grammar is permitted here.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar deliberately contains no finite language-level limits on:
 *
 *     - number of generic parameters;
 *     - number of bounds;
 *     - function count;
 *     - function parameter count;
 *     - type nesting;
 *     - quantum objects;
 *     - hardware resources;
 *     - distributed nodes;
 *     - processors;
 *     - accelerators;
 *     - memory;
 *     - topology;
 *     - deployment size.
 *
 * Repetition is expressed using ANTLR repetition operators.
 *
 * There is no:
 *
 *     MAX_GENERIC_PARAMETERS
 *     MAX_GENERIC_BOUNDS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * Any implementation resource budget must live outside the language grammar
 * and must be explicit, configurable, documented, diagnosable, and separate
 * from language semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For identical source and lexical configuration:
 *
 *     parameter ordering
 *     bound ordering
 *     source spans
 *
 * must be deterministic.
 *
 * The grammar introduces no unordered semantic structure.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The frontend parser/AST layer must preserve source spans for:
 *
 *     functionGenericParameters
 *     functionGenericParameter
 *     functionGenericParameterName
 *     functionGenericParameterBounds
 *     each functionGenericParameterBound
 *
 * This supports:
 *
 *     duplicate-name diagnostics;
 *     unknown-bound diagnostics;
 *     unsatisfied-constraint diagnostics;
 *     IDE navigation;
 *     formatting;
 *     source maps;
 *     provenance;
 *     compatibility tooling.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors handled by this grammar include:
 *
 *     <>
 *     <, T>
 *     <T, , U>
 *     <T extends>
 *     <T extends + Ordered>
 *     <T extends Numeric +>
 *     <T U>
 *
 * Semantic errors are NOT handled here:
 *
 *     <T, T>
 *     <T extends UnknownTrait>
 *     <T extends IncompatibleType>
 *     <T extends UnsatisfiedConstraint>
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing filename:
 *
 *     grammar/functions/generics.g4
 *
 * MUST NOT be renamed.
 *
 * Existing rule names:
 *
 *     functionGenericParameters
 *     functionGenericParameter
 *     functionGenericParameterName
 *     functionGenericParameterBounds
 *     functionGenericParameterBound
 *
 * are retained.
 *
 * The old token names:
 *
 *     LESS_THAN
 *     GREATER_THAN
 *     K_EXTENDS
 *
 * are corrected to the actual canonical lexical names:
 *
 *     LESS
 *     GREATER
 *     EXTENDS
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS:
 *
 *     no hardware count;
 *     no processor count;
 *     no accelerator count;
 *     no qubit count;
 *     no memory capacity;
 *     no topology;
 *     no physical device identifier;
 *     no fixed generic arity;
 *     no fixed bound count;
 *     no machine-width dependency;
 *     no target-specific parser branch.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar itself contains no Rust implementation.
 *
 * The generated Zamani compiler/frontend integration MUST:
 *
 *     - compile on Rust 1.97;
 *     - compile on Rust 1.97.1;
 *     - remain Rust 2021 compatible;
 *     - use safe Rust only;
 *     - contain no unsafe blocks;
 *     - contain no unsafe functions;
 *     - preserve deterministic AST construction;
 *     - preserve source spans;
 *     - remain target-independent at grammar/parser level.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] Canonical parser vocabulary is used.
 *     [x] Existing token names are used.
 *     [x] No lexer rules are duplicated.
 *     [x] Generic declaration is separated from generic application.
 *     [x] Generic parameter ordering is preserved.
 *     [x] Bound ordering is preserved.
 *     [x] Empty generic lists are rejected.
 *     [x] Trailing commas are supported.
 *     [x] Generic arity is not artificially bounded.
 *     [x] Generic bound count is not artificially bounded.
 *     [x] Type-expression syntax remains owned by the type grammar.
 *     [x] Semantic constraint solving remains downstream.
 *     [x] Quantum semantics remain downstream.
 *     [x] quantum::ir remains canonical.
 *     [x] No hardware limits are encoded.
 *     [x] No unsafe Rust is embedded.
 *     [x] Existing filename is preserved.
 *     [x] Existing rule family is preserved.
 *
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * Upstream:
 *
 *     canonical lexer
 *         -> ZamaniLexer
 *         -> this grammar
 *
 * Downstream:
 *
 *     this grammar
 *         -> functions.g4
 *         -> frontend AST
 *         -> semantic generic model
 *         -> type/constraint analysis
 *         -> canonical semantic model
 *         -> IR
 *
 * Cross-domain:
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
 *     future domains
 *
 * all consume the same generic declaration semantics.
 *
 * ============================================================================
 */

parser grammar FunctionGenerics;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * FUNCTION GENERIC PARAMETER LIST
 * ============================================================================
 *
 * Canonical:
 *
 *     <T>
 *     <T, U>
 *     <T, U, V>
 *
 * Bounded:
 *
 *     <T extends Numeric>
 *     <T extends Numeric + Ordered>
 *
 * Trailing comma:
 *
 *     <T,>
 *     <T, U,>
 *
 * Empty:
 *
 *     <>
 *
 * is rejected.
 *
 * No language-level maximum generic arity is imposed.
 */

functionGenericParameters
    : LESS
      functionGenericParameter
      (
          COMMA
          functionGenericParameter
      )*
      COMMA?
      GREATER
    ;


/* ============================================================================
 * INDIVIDUAL GENERIC PARAMETER
 * ============================================================================
 */

functionGenericParameter
    : functionGenericParameterName
      functionGenericParameterBounds?
    ;


/* ============================================================================
 * GENERIC PARAMETER NAME
 * ============================================================================
 *
 * The canonical lexer supplies IDENTIFIER.
 *
 * Naming conventions such as:
 *
 *     T
 *     U
 *     V
 *
 * are conventions only and are not enforced by the grammar.
 */

functionGenericParameterName
    : IDENTIFIER
    ;


/* ============================================================================
 * GENERIC PARAMETER BOUNDS
 * ============================================================================
 *
 * Canonical:
 *
 *     T extends Numeric
 *
 *     T extends Numeric + Ordered
 *
 * `EXTENDS` is the actual canonical lexer token.
 */

functionGenericParameterBounds
    : EXTENDS
      functionGenericParameterBound
      (
          PLUS
          functionGenericParameterBound
      )*
    ;


/* ============================================================================
 * INDIVIDUAL BOUND
 * ============================================================================
 *
 * A bound is a canonical type expression.
 *
 * This file does not decide what the type expression means.
 */

functionGenericParameterBound
    : typeExpression
    ;