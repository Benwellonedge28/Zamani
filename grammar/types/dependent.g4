/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/dependent.g4
 *
 * Status:
 *     Canonical modular SOURCE-TYPE grammar for dependent/value-indexed types.
 *
 * Purpose:
 *     Owns the syntax required to express types whose meaning is parameterized
 *     by source-level values, symbolic dimensions, indices, lengths, shapes,
 *     capacities, predicates, or other type-level information.
 *
 * This file is intentionally syntax-only.
 *
 * It does NOT evaluate type-level expressions, prove propositions, resolve
 * names, infer types, allocate resources, select hardware, or construct IR.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 *     grammar/lexer/tokens.g4
 *                 |
 *                 v
 *     grammar/types/dependent.g4
 *                 |
 *                 v
 *     grammar/types/types.g4
 *                 |
 *                 v
 *     frontend AST / TypeExpr
 *                 |
 *                 v
 *     structural validation
 *                 |
 *                 v
 *     semantic type system
 *                 |
 *                 v
 *     canonical semantic model / IR
 *                 |
 *                 +----------------------+----------------------+
 *                 |                      |                      |
 *                 v                      v                      v
 *             classical              quantum::ir              HDL
 *                 |                      |                      |
 *                 +----------------------+----------------------+
 *                                        |
 *                                        v
 *                              optimization / lowering
 *                                        |
 *                              routing / scheduling
 *                                        |
 *                              QEC / resilience / ZQN
 *                                        |
 *                                        v
 *                                       HAL
 *                                        |
 *                                        v
 *                                target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * This file OWNS:
 *
 *   - dependent/value-indexed type syntax;
 *   - dependent argument lists;
 *   - symbolic type-level value references;
 *   - type-level value expressions when they are consumed by dependent types;
 *   - dependent function syntax;
 *   - dependent pair syntax;
 *   - identity/equality type syntax;
 *   - source-level binders used by dependent types;
 *   - syntax-level composition of dependent type constructs.
 *
 * This file DOES NOT OWN:
 *
 *   - lexer definitions;
 *   - token spelling;
 *   - identifier lexical rules;
 *   - ordinary generic type syntax;
 *   - ordinary arrays;
 *   - ordinary slices;
 *   - ordinary tuples;
 *   - ordinary functions;
 *   - type inference;
 *   - type unification;
 *   - dependent-type normalization;
 *   - compile-time evaluation;
 *   - proof checking;
 *   - theorem proving;
 *   - name resolution;
 *   - constant evaluation;
 *   - ownership;
 *   - borrowing;
 *   - resource allocation;
 *   - hardware discovery;
 *   - target selection;
 *   - topology;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime representation;
 *   - ABI layout.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Dependent types MUST describe program semantics rather than today's
 * implementation limits.
 *
 * The grammar MUST NOT contain:
 *
 *     MAX_TYPE_PARAMETERS
 *     MAX_TYPE_DEPTH
 *     MAX_DEPENDENT_ARGUMENTS
 *     MAX_ARRAY_LENGTH
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_QUANTUM_REGISTER_SIZE
 *     MAX_QUANTUM_DIMENSION
 *     MAX_MEMORY_SIZE
 *     MAX_RESOURCE_COUNT
 *
 * Examples such as:
 *
 *     Vector<int, N>
 *     Matrix<float, Rows, Cols>
 *     Tensor<T, N, M, K>
 *     QReg[N]
 *
 * remain source-level descriptions.
 *
 * Whether a particular target can realize those requirements is decided
 * downstream by semantic/resource/capability analysis and target realization.
 *
 * ============================================================================
 * DOMAIN-NEUTRALITY
 * ============================================================================
 *
 * Dependent types are deliberately not tied to:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     ASIC
 *     memory bank
 *     network node
 *     vendor backend
 *
 * The same dependent syntax can therefore describe:
 *
 *     classical arrays
 *     matrices
 *     tensors
 *     quantum registers
 *     HDL buses
 *     packet widths
 *     distributed collections
 *     accelerator tiles
 *     scientific dimensions
 *     AI tensor shapes
 *     symbolic resource quantities
 *
 * without embedding physical implementation decisions in the grammar.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * It consumes the canonical:
 *
 *     ZamaniTokens
 *
 * from:
 *
 *     grammar/lexer/tokens.g4
 *
 * It MUST NOT introduce lexer rules.
 *
 * Existing canonical lexical concepts used here include:
 *
 *     IDENTIFIER
 *     INTEGER_LITERAL
 *     FLOAT_LITERAL
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     COLON
 *     DOUBLE_COLON
 *     ARROW
 *     PLUS
 *     MINUS
 *     STAR
 *     SLASH
 *     MODULO
 *     LEFT_SHIFT
 *     RIGHT_SHIFT
 *     BIT_AND
 *     BIT_OR
 *     CARET
 *
 * The exact spelling of those tokens remains owned by tokens.g4.
 *
 * ============================================================================
 * IMPORTANT COMPOSITION RULE
 * ============================================================================
 *
 * `grammar/types/types.g4` is the public type-expression composition grammar.
 *
 * It MUST delegate:
 *
 *     dependentType
 *
 * to this grammar.
 *
 * It MUST NOT retain a second implementation of:
 *
 *     dependentType
 *     dependentArgumentList
 *     dependentArgument
 *     dependentBinder
 *     dependentFunctionType
 *     dependentPairType
 *     identityType
 *
 * after this file becomes canonical.
 *
 * This prevents grammar competition and prevents two subtly different
 * definitions of dependent types from entering the parser.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar does not define a new AST.
 *
 * Dependent syntax must lower into the repository's existing domain-neutral
 * frontend type representation.
 *
 * At minimum the frontend semantic mapping must preserve:
 *
 *     base type
 *     dependent arguments
 *     symbolic values
 *     binder names
 *     binder types
 *     body/result type
 *     source spans
 *
 * If the existing TypeExpr representation does not yet expose dedicated
 * dependent-type variants, that is an AST/semantic-model implementation task;
 * this grammar must not invent a second AST to compensate.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "What dependent type syntax was written?"
 *
 * Semantic analysis answers:
 *
 *     "What does the dependency mean?"
 *
 * Examples:
 *
 *     Vector<int, N>
 *
 * does NOT mean that the parser must determine N.
 *
 *     QReg[N]
 *
 * does NOT mean that the parser allocates N qubits.
 *
 *     Matrix<T, Rows, Cols>
 *
 * does NOT mean that the parser checks whether Rows * Cols fits in memory.
 *
 * Those are downstream semantic/resource questions.
 *
 * ============================================================================
 * DEPENDENT ARGUMENT MODEL
 * ============================================================================
 *
 * A dependent argument can be:
 *
 *     - a type;
 *     - a symbolic value;
 *     - a literal value;
 *     - a qualified symbolic value;
 *     - a structured type-level expression.
 *
 * The grammar deliberately does not force every domain to use the same
 * interpretation.
 *
 * For example:
 *
 *     Vector<T, N>
 *
 * may be interpreted as a length-indexed vector.
 *
 *     Tensor<T, Rows, Cols>
 *
 * may be interpreted as a shape-indexed tensor.
 *
 *     QReg[N]
 *
 * may be interpreted as a quantum register abstraction.
 *
 *     Bus[Width]
 *
 * may be interpreted as an HDL width parameter.
 *
 * ============================================================================
 * DEPENDENT TYPE SYNTAX
 * ============================================================================
 *
 * Canonical value-indexed form:
 *
 *     Type[arguments]
 *
 * Examples:
 *
 *     Vector[int]
 *     Vector[int, N]
 *     Matrix[float, Rows, Cols]
 *     Tensor[T, N, M, K]
 *     QReg[N]
 *
 * Generic constructors remain distinct:
 *
 *     Vector<int, N>
 *     Matrix<float, Rows, Cols>
 *
 * The semantic layer may recognize both representations where the language
 * specification declares them equivalent, but the parser does not silently
 * rewrite one form into another.
 *
 * ============================================================================
 * DEPENDENT FUNCTION / PI TYPE
 * ============================================================================
 *
 * A dependent function expresses a result type whose meaning depends on a
 * bound value/type.
 *
 * Canonical syntax:
 *
 *     (x: T) -> U
 *
 * where U may refer to x.
 *
 * Examples:
 *
 *     (n: Nat) -> Vector[int, n]
 *     (n: Nat) -> Matrix[float, n, n]
 *
 * The grammar records the binder.
 *
 * It does not prove that U legally depends on x.
 *
 * ============================================================================
 * DEPENDENT PAIR / SIGMA TYPE
 * ============================================================================
 *
 * A dependent pair binds a value/type and carries a body whose type depends
 * on that binding.
 *
 * Canonical source form:
 *
 *     Sigma(x: T, U)
 *
 * or an equivalent dedicated syntax established by the language specification.
 *
 * The grammar accepts the structured form without attempting semantic
 * interpretation.
 *
 * ============================================================================
 * IDENTITY / EQUALITY TYPE
 * ============================================================================
 *
 * The language type-system document defines:
 *
 *     Id(T, a, b)
 *
 * as an identity/equality type.
 *
 * The parser may represent this as a named dependent type constructor:
 *
 *     Id[T, a, b]
 *
 * where:
 *
 *     T = carrier type
 *     a = first term/value
 *     b = second term/value
 *
 * Proof/equality checking remains semantic.
 *
 * ============================================================================
 * TYPE-LEVEL VALUES
 * ============================================================================
 *
 * The grammar permits symbolic expressions such as:
 *
 *     N
 *     Rows
 *     Cols
 *     N + 1
 *     2 * N
 *     Rows * Cols
 *     Size / ElementSize
 *
 * No compile-time evaluator is embedded here.
 *
 * A compiler may later normalize or evaluate such expressions according to
 * the semantic/type-level evaluation rules.
 *
 * ============================================================================
 * SOURCE SPANS / DIAGNOSTICS
 * ============================================================================
 *
 * Every dependent construct must retain sufficient parser context for the
 * frontend to produce diagnostics identifying:
 *
 *     - the complete dependent type;
 *     - the dependent argument;
 *     - the binder;
 *     - the referenced symbolic value;
 *     - the source location.
 *
 * This grammar must not use target-dependent parser actions to diagnose
 * semantic validity.
 *
 * ============================================================================
 * RECURSION / SCALABILITY
 * ============================================================================
 *
 * Nested dependent types are legal:
 *
 *     Matrix<Vector<int, N>, Rows, Cols>
 *
 *     Vector<Tensor<float, N, M>, K>
 *
 *     QReg[Rows * Cols]
 *
 *     Outer[Inner[T, N], M]
 *
 * There is no language-defined nesting ceiling.
 *
 * Operational parser/resource limits, if required for denial-of-service
 * protection, must be configurable implementation policy outside the language
 * semantics.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Classical:
 *
 *     Vector<T, N>
 *     Matrix<T, R, C>
 *     Tensor<T, ...>
 *
 * Quantum:
 *
 *     QReg[N]
 *     State[N]
 *     LogicalQubitArray[N]
 *
 * HDL:
 *
 *     Bus[Width]
 *     Memory[Depth]
 *
 * AI/data:
 *
 *     Tensor<T, Batch, Sequence, Features>
 *
 * Distributed:
 *
 *     Shard<T, Nodes>
 *
 * None of these forms hard-code the eventual number of resources available on
 * a target.
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * A dependent type may express:
 *
 *     required logical width
 *     symbolic memory extent
 *     tensor shape
 *     quantum register cardinality
 *     protocol payload size
 *
 * It MUST NOT encode physical placement such as:
 *
 *     GPU 0
 *     QPU 3
 *     physical_qubit 17
 *     NUMA node 2
 *     memory_bank 4
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * `QReg[N]` is a source-level abstraction.
 *
 * The grammar does not:
 *
 *     allocate qubits;
 *     choose physical qubits;
 *     route operations;
 *     schedule operations;
 *     select a QPU;
 *     perform QEC;
 *     model calibration;
 *     implement ZQN.
 *
 * After semantic analysis, quantum constructs continue toward:
 *
 *     quantum::ir
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     routing / scheduling
 *         |
 *         v
 *     QEC / resilience / ZQN
 *         |
 *         v
 *     HAL
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * SECURITY / SAFETY
 * ============================================================================
 *
 * This grammar contains no Rust actions and no `unsafe` code.
 *
 * Generated Rust integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and must not require `unsafe`.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 */

parser grammar Dependent;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * 1. PUBLIC DEPENDENT TYPE ENTRY
 * ============================================================================
 *
 * A dependent type has a named/type-path constructor followed by a value/type
 * argument list enclosed in square brackets.
 *
 * Examples:
 *
 *     Vector[int, N]
 *     Matrix[float, Rows, Cols]
 *     Tensor[T, N, M, K]
 *     QReg[N]
 *
 * The base path is intentionally open.
 */
dependentType
    : dependentTypeConstructor
      LBRACKET
      dependentArgumentList
      RBRACKET
    ;


/*
 * ============================================================================
 * 2. DEPENDENT TYPE CONSTRUCTOR
 * ============================================================================
 *
 * The constructor is a source-level type path.
 *
 * Semantic analysis decides what the constructor denotes.
 */
dependentTypeConstructor
    : dependentTypePath
    ;


dependentTypePath
    : dependentTypePathSegment
      (
          DOUBLE_COLON
          dependentTypePathSegment
      )*
    ;


dependentTypePathSegment
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 3. DEPENDENT ARGUMENT LIST
 * ============================================================================
 *
 * No language-level arity limit is imposed.
 *
 * A trailing comma is accepted for consistency with the repository's generic
 * type argument conventions.
 */
dependentArgumentList
    : dependentArgument
      (
          COMMA
          dependentArgument
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 4. DEPENDENT ARGUMENT
 * ============================================================================
 *
 * The first alternative allows a dependent type to be parameterized by a
 * normal type.
 *
 * The second alternative allows symbolic/type-level values.
 *
 * Semantic analysis determines whether the constructor accepts that category.
 */
dependentArgument
    : typeExpression
    | dependentValueExpression
    ;


/*
 * ============================================================================
 * 5. DEPENDENT VALUE EXPRESSION
 * ============================================================================
 *
 * This is deliberately restricted to source-level symbolic/value syntax.
 *
 * It is NOT a general runtime expression grammar.
 *
 * Examples:
 *
 *     N
 *     Rows
 *     N + 1
 *     Rows * Cols
 *     2 * N
 */
dependentValueExpression
    : dependentValueUnary*
      dependentValuePrimary
      dependentValueBinaryPart*
    ;


dependentValueUnary
    : PLUS
    | MINUS
    ;


dependentValueBinaryPart
    : dependentValueOperator
      dependentValueUnary*
      dependentValuePrimary
    ;


dependentValueOperator
    : PLUS
    | MINUS
    | STAR
    | SLASH
    | MODULO
    | LEFT_SHIFT
    | RIGHT_SHIFT
    | BIT_AND
    | BIT_OR
    | CARET
    ;


dependentValuePrimary
    : INTEGER_LITERAL
    | FLOAT_LITERAL
    | IDENTIFIER
    | dependentQualifiedValue
    | dependentParenthesizedValue
    ;


dependentQualifiedValue
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )+
    ;


dependentParenthesizedValue
    : LPAREN
      dependentValueExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 6. DEPENDENT BINDER
 * ============================================================================
 *
 * Binder syntax is intentionally independent of any particular semantic
 * representation.
 *
 * Examples:
 *
 *     x: Nat
 *     n: usize
 *     rows: Nat
 *
 * Name resolution and kind checking happen later.
 */
dependentBinder
    : IDENTIFIER
      COLON
      typeExpression
    ;


/*
 * ============================================================================
 * 7. DEPENDENT FUNCTION / PI TYPE
 * ============================================================================
 *
 * Canonical dependent-function syntax:
 *
 *     (x: T) -> U
 *
 * U may contain x in a dependent argument.
 *
 * Example:
 *
 *     (n: Nat) -> Vector[int, n]
 */
dependentFunctionType
    : LPAREN
      dependentBinder
      RPAREN
      ARROW
      typeExpression
    ;


/*
 * ============================================================================
 * 8. DEPENDENT PAIR / SIGMA TYPE
 * ============================================================================
 *
 * Canonical explicit constructor:
 *
 *     Sigma(x: T, U)
 *
 * The semantic layer determines whether Sigma is a dependent pair and whether
 * U is well-formed under the binder x.
 *
 * The constructor remains syntactic; this rule performs no proof checking.
 */
dependentPairType
    : IDENTIFIER
      LPAREN
      dependentBinder
      COMMA
      typeExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 9. IDENTITY / EQUALITY TYPE
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     Id[T, a, b]
 *
 * It is deliberately parsed through the dependent-type constructor rather than
 * hard-coding a finite list of identity domains.
 *
 * Semantic analysis determines:
 *
 *     T
 *     a
 *     b
 *
 * and establishes whether a valid identity/equality proposition exists.
 */
identityType
    : IDENTIFIER
      LBRACKET
      dependentIdentityArgumentList
      RBRACKET
    ;


dependentIdentityArgumentList
    : dependentArgument
      COMMA
      dependentValueExpression
      COMMA
      dependentValueExpression
      COMMA?
    ;


/*
 * ============================================================================
 * 10. DEPENDENT TYPE GROUPING
 * ============================================================================
 *
 * Grouping is kept local so dependent constructs can be nested without
 * introducing a second type grammar.
 */
dependentParenthesizedType
    : LPAREN
      typeExpression
      RPAREN
    ;


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * REQUIRED ONE-TIME INTEGRATION:
 *
 * 1. `grammar/types/types.g4`
 *
 *    Remove its existing local implementation of:
 *
 *        dependentType
 *        dependentArgumentList
 *        dependentArgument
 *
 *    and delegate the `dependentType` alternative to this grammar.
 *
 * 2. Root composition
 *
 *    The canonical root grammar must import/compose this parser grammar
 *    through the repository's existing modular ANTLR architecture.
 *
 * 3. `grammar/lexer/tokens.g4`
 *
 *    No lexer rules are added by this file.
 *
 *    All lexical symbols used above must continue to originate from
 *    ZamaniTokens.
 *
 * 4. Frontend AST
 *
 *    Existing `TypeExpr` remains authoritative.
 *
 *    If dedicated dependent variants are required by semantic implementation,
 *    they must be added to the existing AST type hierarchy rather than creating
 *    a grammar-specific AST.
 *
 * 5. Semantic type system
 *
 *    Dependent arguments must remain symbolic until the appropriate semantic
 *    phase.
 *
 * 6. Quantum
 *
 *    QReg[N] and equivalent quantum abstractions must eventually lower through
 *    the canonical quantum semantic boundary:
 *
 *        quantum::ir
 *
 *    No second quantum IR is introduced here.
 *
 * 7. Compiler/runtime
 *
 *    This grammar does not make compiler or runtime decisions about whether a
 *    target has enough resources.
 *
 * ============================================================================
 * NEGATIVE CONSTRAINTS
 * ============================================================================
 *
 * This grammar must reject nothing merely because a value is "too large".
 *
 * It must NOT reject:
 *
 *     Vector[int, 1_000_000_000]
 *     QReg[N]
 *     Tensor[T, A, B, C, D, E, ...]
 *
 * because of machine-specific assumptions.
 *
 * Semantic/resource validation may reject a program later when its declared
 * requirements cannot be satisfied under a selected execution context.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [ ] It is the sole owner of dependentType syntax.
 * [ ] types.g4 no longer duplicates dependentType.
 * [ ] It consumes ZamaniTokens only.
 * [ ] It defines no lexer rules.
 * [ ] It has no hardware-specific limits.
 * [ ] It permits symbolic dimensions.
 * [ ] It permits arbitrary nesting subject only to implementation budgets.
 * [ ] It permits type and value dependent arguments.
 * [ ] It supports dependent binders.
 * [ ] It supports dependent function syntax.
 * [ ] It provides a path for dependent pair/Sigma semantics.
 * [ ] It provides identity/equality-type syntax.
 * [ ] It preserves domain neutrality.
 * [ ] It maps to the existing frontend AST contract.
 * [ ] It does not introduce another IR.
 * [ ] It preserves quantum::ir as the quantum semantic boundary.
 * [ ] It has positive tests.
 * [ ] It has negative tests.
 * [ ] It has boundary tests.
 * [ ] It has scalability tests.
 * [ ] It has compatibility tests.
 * [ ] It contains no unsafe code.
 * [ ] It is compatible with Rust 1.97/1.97.1 through the generated frontend.
 *
 * ============================================================================
 */