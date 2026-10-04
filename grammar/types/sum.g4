/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/sum.g4
 *
 * Grammar:
 *     Sum
 *
 * Status:
 *     CANONICAL ANONYMOUS SUM-TYPE / UNION-TYPE PARSER DELEGATE
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the source-level syntax of an ANONYMOUS SUM TYPE.
 *
 * Canonical examples:
 *
 *     int | string
 *     A | B
 *     A | B | C
 *     Result<T, E> | Pending
 *     Qubit | ClassicalValue
 *     SomeType? | OtherType
 *
 * A sum type expresses that a value belongs to one of multiple alternative
 * source-level types.
 *
 * This grammar is deliberately domain-neutral.
 *
 * It can therefore participate in:
 *
 *     classical computing
 *     quantum computing
 *     hybrid computing
 *     HDL
 *     hardware abstraction
 *     AI/ML
 *     data processing
 *     networking
 *     distributed computing
 *     accelerator programming
 *     embedded computing
 *     future computational domains
 *
 * ============================================================================
 * IMPORTANT TERMINOLOGY
 * ============================================================================
 *
 * This file uses "sum type" for an ANONYMOUS TYPE-LEVEL ALTERNATIVE:
 *
 *     A | B
 *
 * It must not be confused with:
 *
 *     named union declarations
 *
 * such as:
 *
 *     union Result<T, E> = Ok(T) | Err(E);
 *
 * or algebraic constructor syntax:
 *
 *     Some(T) | None
 *
 * inside a declaration.
 *
 * Those are owned by other grammar components.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The intended parser architecture is:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Types
 *          |
 *          +----------------------+
 *          |                      |
 *          v                      v
 *     ordinary type          Sum.sumType
 *          |                      |
 *          +----------+-----------+
 *                     |
 *                     v
 *              canonical TypeExpr
 *                     |
 *                     v
 *             structural validation
 *                     |
 *                     v
 *              semantic type system
 *                     |
 *          +----------+----------+----------------+
 *          |                     |                |
 *          v                     v                v
 *      classical              quantum            HDL
 *      semantics              semantics          semantics
 *                                |
 *                                v
 *                           quantum::ir
 *                                |
 *                         optimization
 *                                |
 *                    lowering/routing/scheduling
 *                                |
 *                         resilience/QEC
 *                                |
 *                               ZQN
 *                                |
 *                               HAL
 *                                |
 *                         target realization
 *
 * This file terminates at source-level syntax.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - sumType
 *     - sumTypeMember
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - complete typeExpression
 *     - typeCore
 *     - typePostfix
 *     - primitive types
 *     - named types
 *     - generic application
 *     - tuple types
 *     - arrays
 *     - slices
 *     - function types
 *     - references
 *     - pointers
 *     - Result types
 *     - optional postfix semantics
 *     - dependent type semantics
 *     - type-level value semantics
 *     - named union declarations
 *     - enum declarations
 *     - algebraic constructor declarations
 *     - pattern matching
 *     - guards
 *     - exhaustiveness checking
 *     - type inference
 *     - type normalization
 *     - subtype checking
 *     - type equivalence
 *     - ownership checking
 *     - borrow checking
 *     - effects
 *     - resources
 *     - capabilities
 *     - policies
 *     - provenance
 *     - hardware selection
 *     - quantum allocation
 *     - quantum routing
 *     - scheduling
 *     - calibration
 *     - QEC
 *     - ZQN
 *     - HAL
 *     - runtime representation
 *     - ABI representation
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly ONE source-level anonymous sum-type syntax owner.
 *
 * After this file is integrated:
 *
 *     grammar/types/sum.g4
 *
 * is the canonical owner of:
 *
 *     A | B
 *
 * The existing:
 *
 *     grammar/types/union.g4
 *
 * MUST NOT remain a second independent implementation of the same anonymous
 * syntax.
 *
 * It should become a compatibility/deprecation forwarding component or have
 * its anonymous-union implementation removed after the migration.
 *
 * Named union declarations remain independently owned by:
 *
 *     grammar/declarations/unions.g4
 *
 * Algebraic constructor syntax remains independently owned by:
 *
 *     grammar/types/algebraic-types.g4
 *
 * ============================================================================
 * WHY A SEPARATE SUM COMPONENT EXISTS
 * ============================================================================
 *
 * An anonymous sum is structurally different from a named declaration.
 *
 * Anonymous sum:
 *
 *     A | B | C
 *
 * Named declaration:
 *
 *     union Value = A | B | C;
 *
 * Constructor-bearing algebraic declaration:
 *
 *     union Value =
 *         A(T)
 *       | B(U);
 *
 * Keeping these responsibilities separate prevents:
 *
 *     - declaration grammars from becoming type-expression grammars;
 *     - type-expression grammars from becoming declaration grammars;
 *     - constructor syntax from becoming ordinary type syntax;
 *     - semantic ownership from becoming ambiguous.
 *
 * ============================================================================
 * PARSER-DELEGATE CONTRACT
 * ============================================================================
 *
 * This file is a parser delegate.
 *
 * It deliberately consumes rules supplied by the canonical Types grammar:
 *
 *     typeCore
 *     typePostfix
 *
 * It MUST NOT redefine:
 *
 *     typeExpression
 *     typeCore
 *     typePostfix
 *
 * The intended dependency direction is:
 *
 *     Types
 *       |
 *       v
 *     Sum
 *
 * NOT:
 *
 *     Types <-> Sum
 *
 * and NOT:
 *
 *     Sum -> typeExpression -> Sum
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It declares NO lexer rules.
 *
 * The parser consumes the canonical:
 *
 *     ZamaniLexer
 *
 * vocabulary.
 *
 * The only operator token directly required by this grammar is:
 *
 *     PIPE
 *
 * The spelling:
 *
 *     |
 *
 * is owned by the canonical lexer/operator vocabulary.
 *
 * This file MUST NOT define:
 *
 *     UNION
 *     SUM
 *     SUM_TYPE
 *     UNION_TYPE
 *     BAR
 *
 * as replacement lexical tokens.
 *
 * ============================================================================
 * CANONICAL SYNTAX
 * ============================================================================
 *
 * A sum requires at least two members:
 *
 *     A | B
 *
 * and supports an arbitrary number of alternatives:
 *
 *     A | B | C
 *     A | B | C | D
 *     ...
 *
 * The canonical grammar is intentionally:
 *
 *     sumType
 *         : sumTypeMember (PIPE sumTypeMember)+
 *         ;
 *
 * The `+` after the repeated separator/member pair is important.
 *
 * It means:
 *
 *     A
 *
 * is NOT a sum.
 *
 * while:
 *
 *     A | B
 *
 * IS a sum.
 *
 * ============================================================================
 * SUM MEMBER
 * ============================================================================
 *
 * A sum member is:
 *
 *     typeCore typePostfix*
 *
 * This is intentional.
 *
 * It prevents recursive invocation of:
 *
 *     typeExpression
 *
 * and therefore avoids:
 *
 *     typeExpression
 *         -> sumType
 *             -> sumTypeMember
 *                 -> typeExpression
 *
 * which would create an unnecessary recursive ambiguity.
 *
 * Instead, the canonical Types grammar owns the surrounding type expression
 * composition.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * Public rule:
 *
 *     sumType
 *
 * Reusable member rule:
 *
 *     sumTypeMember
 *
 * No other rule in this file is intended to become a competing public
 * type-expression entry point.
 *
 * ============================================================================
 * TYPE PRECEDENCE / COMPOSITION
 * ============================================================================
 *
 * The surrounding Types grammar must establish the complete precedence and
 * composition relationship.
 *
 * The intended structure is conceptually:
 *
 *     typeExpression
 *         : typeQualifier*
 *           typeSumExpression
 *         ;
 *
 *     typeSumExpression
 *         : sumType
 *         | typeCore typePostfix*
 *         ;
 *
 *     sumType
 *         : sumTypeMember
 *           (PIPE sumTypeMember)+
 *         ;
 *
 *     sumTypeMember
 *         : typeCore typePostfix*
 *         ;
 *
 * The exact surrounding rule names may remain as they currently exist in
 * `types.g4`, provided:
 *
 *     1. `typeExpression` remains the sole public complete type-expression
 *        entry point;
 *
 *     2. `typeCore` remains the owner of atomic/composite non-sum type forms;
 *
 *     3. `typePostfix` remains the owner of postfix type operators;
 *
 *     4. `sumType` is composed above `typeCore`;
 *
 *     5. `sumTypeMember` never calls `typeExpression`;
 *
 *     6. no circular import is introduced.
 *
 * ============================================================================
 * PARENTHESIZED SUMS
 * ============================================================================
 *
 * Parenthesized sums are handled by the canonical parenthesized-type grammar.
 *
 * Examples:
 *
 *     (A | B)
 *
 *     Option<(A | B)>
 *
 *     Result<(A | B), Error>
 *
 * This file MUST NOT redefine:
 *
 *     parenthesizedType
 *
 * or any other parenthesis-based type construct.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic applications are owned by the canonical type grammar.
 *
 * Therefore:
 *
 *     Vec<A> | Vec<B>
 *
 *     Result<T, E> | Pending
 *
 *     QuantumState<T> | ClassicalState<T>
 *
 * are represented because `typeCore` can represent the generic application.
 *
 * This file does not define:
 *
 *     genericType
 *     typeArguments
 *     genericArgumentList
 *     generic parameters
 *     generic bounds
 *
 * Generic arity is determined by the applicable type constructor declaration
 * and semantic analysis.
 *
 * ============================================================================
 * OPTIONAL INTEGRATION
 * ============================================================================
 *
 * Optionality is a postfix type constructor owned by the canonical Types
 * grammar.
 *
 * Therefore:
 *
 *     A? | B
 *
 *     A | B?
 *
 * can be represented through:
 *
 *     sumTypeMember
 *         : typeCore typePostfix*
 *         ;
 *
 * The distinction between:
 *
 *     A? | B
 *
 * and:
 *
 *     (A | B)?
 *
 * is preserved structurally.
 *
 * The latter depends on the canonical parenthesized-type composition followed
 * by the optional postfix.
 *
 * This grammar must not redefine `?`.
 *
 * ============================================================================
 * FUNCTION TYPE INTEGRATION
 * ============================================================================
 *
 * Function types remain owned by the function-type grammar and canonical
 * Types composition.
 *
 * Sum members may therefore include function types where the surrounding type
 * grammar permits them.
 *
 * Examples:
 *
 *     fn(A) -> B | fn(C) -> D
 *
 *     fn(A | B) -> C
 *
 * Parenthesization rules for ambiguous function/sum combinations remain the
 * responsibility of the canonical type-expression composition.
 *
 * This file does not define function syntax.
 *
 * ============================================================================
 * REFERENCE / POINTER INTEGRATION
 * ============================================================================
 *
 * Reference and pointer types are consumed through `typeCore`.
 *
 * Examples where admitted by the canonical type grammar:
 *
 *     &A | &B
 *
 *     *A | *B
 *
 *     &mut A | B
 *
 * This file does not define:
 *
 *     &
 *     *
 *     mut
 *     lifetime
 *
 * ============================================================================
 * DEPENDENT / VALUE-PARAMETERIZED TYPES
 * ============================================================================
 *
 * Dependent/value-parameterized types may occur as sum members when admitted
 * by `typeCore`.
 *
 * Examples:
 *
 *     Vector<T>[N] | EmptyVector<T>
 *
 *     Matrix<T>[Rows, Cols] | SparseMatrix<T>
 *
 *     Tensor<T>[N, M, K] | SymbolicTensor<T>
 *
 * This grammar does not:
 *
 *     evaluate N;
 *     evaluate M;
 *     evaluate K;
 *     convert symbolic values to host integers;
 *     determine memory allocation;
 *     determine physical dimensions.
 *
 * All such decisions belong downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum source-level types may participate in sums:
 *
 *     Qubit | ClassicalValue
 *
 *     LogicalQubit | Measurement
 *
 *     QuantumState<T> | ClassicalState<T>
 *
 *     QRegister<N> | EmptyRegister
 *
 * The sum grammar does NOT:
 *
 *     - allocate qubits;
 *     - identify physical qubits;
 *     - select a QPU;
 *     - select a topology;
 *     - select a gate set;
 *     - perform decomposition;
 *     - perform routing;
 *     - perform scheduling;
 *     - select calibration;
 *     - perform QEC;
 *     - create ZQN;
 *     - invoke HAL.
 *
 * The source-level result remains a domain-neutral `TypeExpr::Union`.
 *
 * Quantum-specific semantic handling occurs later:
 *
 *     TypeExpr
 *         |
 *         v
 *     semantic type system
 *         |
 *         v
 *     quantum semantic model
 *         |
 *         v
 *     quantum::ir
 *
 * where the program actually contains executable quantum semantics.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical examples:
 *
 *     int | float
 *
 *     string | bytes
 *
 *     Vector<T> | Scalar<T>
 *
 *     Success | Failure
 *
 * No machine width is encoded here.
 *
 * For example:
 *
 *     i32
 *
 * remains governed by the repository's lexical/type policy rather than this
 * sum grammar.
 *
 * The grammar must never infer a host integer width from Rust's `usize`,
 * `u32`, `u64`, pointer width or compiler platform.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware and HDL types may participate in sums through `typeCore`.
 *
 * Examples:
 *
 *     Signal<T> | ControlSignal
 *
 *     Register<T> | MemoryReference
 *
 *     HardwareValue | SoftwareValue
 *
 * The sum grammar does not encode:
 *
 *     register width;
 *     bus width;
 *     pin count;
 *     FPGA capacity;
 *     ASIC capacity;
 *     accelerator count;
 *     memory capacity;
 *     device count;
 *     physical topology.
 *
 * Those are downstream semantic/resource/target concerns.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Domain types such as:
 *
 *     Tensor<T>
 *     Dataset<T>
 *     Model<I, O>
 *     Distribution<T>
 *     KnowledgeGraph<N, E>
 *     AgentState
 *
 * can participate in sums without changes to this grammar.
 *
 * The grammar deliberately does not enumerate AI algorithms or application
 * concepts.
 *
 * New domain types remain names, generic applications or registered
 * extensions at the type-system level.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORK INTEGRATION
 * ============================================================================
 *
 * Distributed and network types can participate:
 *
 *     Local<T> | Remote<T>
 *
 *     Connected | Disconnected
 *
 *     Message<T> | Error
 *
 *     Replica<T> | Unavailable
 *
 * The grammar does not determine:
 *
 *     node count;
 *     network topology;
 *     replication factor;
 *     placement;
 *     retry strategy;
 *     consistency protocol.
 *
 * Those belong to distributed/resource/execution semantics.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * A sum type describes a type-level alternative.
 *
 * It does not allocate resources.
 *
 * For example:
 *
 *     Qubit | ClassicalValue
 *
 * does NOT mean:
 *
 *     allocate a QPU
 *
 * or:
 *
 *     reserve one physical qubit.
 *
 * Resource requirements remain owned by:
 *
 *     grammar/resources/
 *
 * Capability semantics remain owned by:
 *
 *     grammar/core/capabilities.g4
 *     grammar/resources/
 *
 * Policy semantics remain owned by:
 *
 *     grammar/policies/
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * A sum type does not inherently perform an effect.
 *
 * For:
 *
 *     NetworkMessage | LocalMessage
 *
 * the appearance of the type does not itself perform network I/O.
 *
 * Likewise:
 *
 *     Qubit | Measurement
 *
 * does not itself perform quantum measurement.
 *
 * Effects remain attached to executable operations and semantic constructs.
 *
 * This grammar MUST NOT introduce:
 *
 *     sumEffect
 *     unionEffect
 *     quantumSumEffect
 *     aiSumEffect
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Contracts may refer to values whose type is a sum.
 *
 * Examples of semantic intent include:
 *
 *     requires value is A | B
 *
 *     ensures result satisfies condition
 *
 * But:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * remain owned by:
 *
 *     grammar/validation/
 *
 * This grammar does not define contract syntax.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Sum types may appear in declarations or operations governed by policies.
 *
 * Policy resolution is downstream.
 *
 * This grammar does not:
 *
 *     authorize;
 *     deny;
 *     sandbox;
 *     negotiate;
 *     deploy;
 *     allocate;
 *
 * ============================================================================
 * PATTERN MATCHING
 * ============================================================================
 *
 * Sum values are naturally consumed by pattern matching.
 *
 * Example conceptual source:
 *
 *     match value {
 *         ...
 *     }
 *
 * However, this grammar does not define:
 *
 *     match
 *     pattern
 *     guard
 *     arm
 *     exhaustiveness
 *
 * Those belong to the expression/statement/semantic validation layers.
 *
 * The type system must later expose enough structure for exhaustive pattern
 * checking.
 *
 * ============================================================================
 * ALGEBRAIC TYPE DISTINCTION
 * ============================================================================
 *
 * This file MUST NOT absorb algebraic constructor syntax.
 *
 * These concepts are different:
 *
 * Anonymous type-level sum:
 *
 *     A | B
 *
 * Algebraic constructors:
 *
 *     Some(T)
 *     None
 *
 * Named algebraic/union declaration:
 *
 *     union Option<T> =
 *         None
 *       | Some(T);
 *
 * The latter declarations are owned by:
 *
 *     grammar/declarations/unions.g4
 *
 * Constructor syntax is owned by:
 *
 *     grammar/types/algebraic-types.g4
 *
 * The semantic layer may unify their representation eventually, but the
 * parser ownership remains distinct.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces ANTLR parser contexts only.
 *
 * It does not define Rust AST structures.
 *
 * The canonical frontend type representation already contains:
 *
 *     TypeExpr::Union(Vec<TypeExpr>)
 *
 * Therefore the frontend AST builder should map:
 *
 *     sumType
 *
 * to:
 *
 *     TypeExpr::Union(
 *         ordered_members
 *     )
 *
 * where `ordered_members` contains the members in source order.
 *
 * Example:
 *
 *     A | B | C
 *
 * becomes conceptually:
 *
 *     TypeExpr::Union(vec![
 *         TypeExpr(A),
 *         TypeExpr(B),
 *         TypeExpr(C),
 *     ])
 *
 * This file MUST NOT introduce:
 *
 *     SumTypeAst
 *     SumTypeNode
 *     SumTypeIR
 *     AnonymousUnionAst
 *     UnionTypeNode
 *
 * as competing representations.
 *
 * ============================================================================
 * AST SOURCE-ORDER INVARIANT
 * ============================================================================
 *
 * Source order must be preserved.
 *
 * Given:
 *
 *     A | B | C
 *
 * the parser/AST integration must preserve:
 *
 *     A
 *     B
 *     C
 *
 * in exactly that order.
 *
 * A later semantic canonicalization pass may determine whether the semantic
 * union is commutative or can be normalized.
 *
 * Such normalization MUST NOT destroy the source-level ordering information
 * needed by:
 *
 *     diagnostics;
 *     IDE tooling;
 *     formatting;
 *     refactoring;
 *     source maps;
 *     provenance;
 *     compatibility tooling.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - resolving each member;
 *     - resolving aliases;
 *     - resolving generic parameters;
 *     - checking member validity;
 *     - checking duplicate/equivalent members;
 *     - normalization;
 *     - subtype relationships;
 *     - assignability;
 *     - narrowing;
 *     - pattern compatibility;
 *     - exhaustiveness;
 *     - recursive type validity;
 *     - ownership implications;
 *     - effect implications;
 *     - capability implications;
 *     - resource implications;
 *     - target representation.
 *
 * The grammar performs none of these.
 *
 * ============================================================================
 * DUPLICATE MEMBERS
 * ============================================================================
 *
 * The grammar accepts:
 *
 *     A | A
 *
 * because duplicate equivalence cannot reliably be determined syntactically.
 *
 * For example:
 *
 *     A | AliasOfA
 *
 * may only be known to be equivalent after name resolution.
 *
 * Therefore duplicate/equivalent-member handling belongs to semantic analysis.
 *
 * The semantic implementation may:
 *
 *     reject redundant members;
 *     canonicalize them;
 *     issue diagnostics;
 *     retain them according to the language specification.
 *
 * This grammar must not choose between those policies.
 *
 * ============================================================================
 * MINIMUM CARDINALITY
 * ============================================================================
 *
 * A source-level sum requires at least two members.
 *
 * Valid:
 *
 *     A | B
 *
 *     A | B | C
 *
 * Invalid as a sum:
 *
 *     A
 *
 * `A` remains an ordinary type.
 *
 * This does not mean that the language rejects `A`; it simply means that the
 * parser must not classify a single member as a sum.
 *
 * ============================================================================
 * NESTING
 * ============================================================================
 *
 * Sum members may contain nested type structures wherever the canonical
 * `typeCore` grammar admits them.
 *
 * Examples:
 *
 *     Option<A | B>
 *
 *     Result<A | B, Error>
 *
 *     Vec<Result<A | B, E>>
 *
 *     Resource<Capability<A | B>>
 *
 * Parenthesized forms may contain complete sums:
 *
 *     (A | B)
 *
 * This grammar does not impose a language-level nesting limit.
 *
 * ============================================================================
 * RECURSIVE TYPES
 * ============================================================================
 *
 * Recursive named types may participate in sums.
 *
 * Example:
 *
 *     Node | Empty
 *
 * or, inside a declaration:
 *
 *     union List<T> =
 *         Empty
 *       | Node(T, List<T>);
 *
 * The parser only sees source-level structure.
 *
 * Semantic analysis determines whether recursion is:
 *
 *     legal;
 *     guarded;
 *     representable;
 *     ownership-safe;
 *     resource-safe.
 *
 * ============================================================================
 * TYPE-LEVEL VALUES
 * ============================================================================
 *
 * This grammar never evaluates type-level values.
 *
 * A dependent type may contain symbolic values, for example:
 *
 *     Matrix<T>[Rows, Cols]
 *
 * and such a type may participate in a sum:
 *
 *     Matrix<T>[Rows, Cols] | SparseMatrix<T>
 *
 * Any symbolic value remains symbolic until the semantic/compiler layers
 * explicitly resolve it.
 *
 * No conversion to:
 *
 *     usize
 *     u32
 *     u64
 *     host pointer width
 *
 * is permitted merely because the parser consumed the syntax.
 *
 * ============================================================================
 * QUANTUM LINEARITY
 * ============================================================================
 *
 * If a sum contains quantum or linear resource types, the parser remains
 * unaware of resource ownership semantics.
 *
 * For example:
 *
 *     Qubit | ClassicalValue
 *
 * is syntactically a valid sum.
 *
 * Whether a value of that type can be:
 *
 *     copied;
 *     moved;
 *     borrowed;
 *     duplicated;
 *     discarded;
 *
 * is determined by the semantic ownership/linearity system.
 *
 * The grammar must not add special cases for quantum variants.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * The following are forbidden in this grammar:
 *
 *     physical device IDs;
 *     physical qubit IDs;
 *     CPU identifiers;
 *     GPU identifiers;
 *     FPGA identifiers;
 *     ASIC identifiers;
 *     register numbers;
 *     memory addresses;
 *     network node IDs;
 *     topology IDs;
 *     vendor-specific universal variants.
 *
 * Hardware-specific information may exist in explicit target/dialect
 * constructs elsewhere in the language, but must not alter the meaning of
 * anonymous sum syntax.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * The grammar contains NO universal resource limits.
 *
 * It does not define:
 *
 *     MAX_SUM_MEMBERS
 *     MAX_VARIANTS
 *     MAX_UNION_MEMBERS
 *     MAX_TYPE_DEPTH
 *     MAX_GENERIC_ARITY
 *     MAX_TUPLE_ARITY
 *     MAX_ARRAY_LENGTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Arbitrary source-level member sequences are represented by parser repetition.
 *
 * Conceptually:
 *
 *     T0 | T1
 *     T0 | T1 | T2
 *     T0 | T1 | T2 | T3
 *     ...
 *
 * The language does not change its grammar as computational scale increases.
 *
 * Practical implementation limits, if necessary, must be explicit compiler
 * resource policies and must not become source-language semantics.
 *
 * ============================================================================
 * "INFINITY" / RESOURCE MODEL
 * ============================================================================
 *
 * The grammar cannot and should not promise physically infinite memory,
 * compute, storage, qubits or devices.
 *
 * What it guarantees is the correct architectural property:
 *
 *     no language-defined finite ceiling is encoded here.
 *
 * Therefore the representable semantic scale is bounded by:
 *
 *     actual source size;
 *     implementation resources;
 *     compiler policy;
 *     runtime resources;
 *     target resources;
 *     target capabilities;
 *     explicitly declared program requirements.
 *
 * A target that cannot satisfy a program's requirements must be handled by
 * semantic/resource/deployment diagnostics rather than by changing the source
 * type grammar.
 *
 * ============================================================================
 * RESOURCE FEASIBILITY
 * ============================================================================
 *
 * A valid source sum does not imply that every member is executable on every
 * target.
 *
 * For example:
 *
 *     Qubit | ClassicalValue
 *
 * may be semantically valid even when a particular target has no quantum
 * capability.
 *
 * Target analysis must distinguish:
 *
 *     syntactically valid
 *
 * from:
 *
 *     semantically valid
 *
 * from:
 *
 *     resource-feasible
 *
 * from:
 *
 *     deployable.
 *
 * This separation is essential for POCO-REAF.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * Type syntax:
 *
 *     A | B
 *
 * is pure source-level type structure.
 *
 * It does not itself:
 *
 *     perform I/O;
 *     measure a quantum state;
 *     send a network packet;
 *     train a model;
 *     adapt an agent;
 *     allocate a device;
 *     reserve memory;
 *     access hardware.
 *
 * These behaviors are determined downstream.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parser/frontend must preserve source locations for:
 *
 *     complete sum;
 *     each member;
 *     each PIPE separator;
 *     nested member types.
 *
 * This permits downstream provenance to record:
 *
 *     source declaration;
 *     source member;
 *     transformations;
 *     normalization;
 *     semantic decisions;
 *     lowering decisions.
 *
 * This grammar itself does not create provenance records.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For a fixed:
 *
 *     token sequence;
 *     language version;
 *     grammar version;
 *     parser configuration;
 *
 * this grammar must produce deterministic structure.
 *
 * It must not depend on:
 *
 *     CPU model;
 *     GPU availability;
 *     QPU availability;
 *     filesystem state;
 *     network state;
 *     environment variables;
 *     current time;
 *     randomness;
 *     calibration;
 *     target selection.
 *
 * ============================================================================
 * PERFORMANCE / PARSER BEHAVIOR
 * ============================================================================
 *
 * The canonical implementation is a simple linear repetition:
 *
 *     sumTypeMember (PIPE sumTypeMember)+
 *
 * This preserves source order without constructing an unnecessarily recursive
 * binary tree at parse time.
 *
 * The grammar does not use:
 *
 *     semantic predicates;
 *     parser actions;
 *     embedded Rust;
 *     runtime calls;
 *     target inspection.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This file:
 *
 *     - performs no I/O;
 *     - performs no network access;
 *     - executes no user code;
 *     - queries no hardware;
 *     - allocates no target resources;
 *     - performs no filesystem access;
 *     - contains no embedded Rust;
 *     - contains no unsafe code.
 *
 * Hostile-input protection must be implemented through explicit parser/compiler
 * resource policy.
 *
 * Such policy must remain distinguishable from language semantics.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors owned by this grammar include:
 *
 *     |
 *     A |
 *     | B
 *     A || B
 *     A | | B
 *     A | | | B
 *
 * Examples with missing members must be rejected structurally by the parser.
 *
 * Semantic errors do NOT belong here:
 *
 *     unknown type;
 *     duplicate equivalent members;
 *     incompatible members;
 *     illegal recursive type;
 *     invalid ownership;
 *     invalid capability;
 *     unavailable resource;
 *     unsupported target.
 *
 * ============================================================================
 * ERROR RECOVERY
 * ============================================================================
 *
 * The grammar must remain compatible with normal ANTLR error recovery.
 *
 * It must not introduce custom parser actions to recover from:
 *
 *     missing member;
 *     missing PIPE;
 *     missing delimiter.
 *
 * Diagnostics should be enriched by the frontend diagnostic layer using the
 * parser context and preserved source spans.
 *
 * ============================================================================
 * COMPATIBILITY WITH EXISTING UNION GRAMMAR
 * ============================================================================
 *
 * The repository currently contains:
 *
 *     grammar/types/union.g4
 *
 * which defines anonymous union syntax substantially equivalent to this
 * component.
 *
 * That duplication must be eliminated.
 *
 * The canonical migration is:
 *
 *     BEFORE
 *
 *         Types
 *           |
 *           +--> Union
 *
 *     AFTER
 *
 *         Types
 *           |
 *           +--> Sum
 *
 * where:
 *
 *     Sum
 *       owns anonymous A | B syntax.
 *
 * The existing `union.g4` must not continue to define an independent
 * implementation of the same parser rules.
 *
 * For compatibility, `union.g4` may temporarily become a forwarding/deprecated
 * grammar component, provided it does not introduce another implementation.
 *
 * Named union declarations are unaffected:
 *
 *     grammar/declarations/unions.g4
 *
 * remains the declaration owner.
 *
 * ============================================================================
 * COMPATIBILITY WITH ALGEBRAIC TYPES
 * ============================================================================
 *
 * The repository also contains:
 *
 *     grammar/types/algebraic-types.g4
 *
 * That grammar describes constructor-oriented algebraic syntax.
 *
 * It must remain separate.
 *
 * Therefore:
 *
 *     A | B
 *
 * is owned here as an anonymous type-level sum.
 *
 * Constructor alternatives such as:
 *
 *     Some(T)
 *     None
 *
 * remain owned by algebraic/declaration grammar.
 *
 * The semantic type system may later normalize both concepts into a common
 * algebraic semantic representation where appropriate.
 *
 * ============================================================================
 * CANONICAL AST MAPPING
 * ============================================================================
 *
 * The frontend AST already contains:
 *
 *     TypeExpr::Union(Vec<TypeExpr>)
 *
 * Therefore the required mapping is:
 *
 *     sumType
 *         |
 *         v
 *     ordered member TypeExpr values
 *         |
 *         v
 *     TypeExpr::Union(Vec<TypeExpr>)
 *
 * No new AST node is required merely because this grammar has its own file.
 *
 * ============================================================================
 * SEMANTIC NORMALIZATION
 * ============================================================================
 *
 * Semantic normalization may eventually transform:
 *
 *     A | A
 *
 * into:
 *
 *     A
 *
 * or reject it, depending on the normative type-system specification.
 *
 * It may also canonicalize equivalent aliases:
 *
 *     A | AliasOfA
 *
 * after resolution.
 *
 * Such transformations must occur downstream and must preserve enough source
 * information for diagnostics and tooling.
 *
 * ============================================================================
 * SUBTYPING / ASSIGNABILITY
 * ============================================================================
 *
 * The grammar does not define subtype rules.
 *
 * Semantic analysis must determine:
 *
 *     whether A is assignable to A | B;
 *
 *     whether B is assignable to A | B;
 *
 *     whether A | B is assignable to another sum;
 *
 *     how generic variance affects sums;
 *
 *     how narrowing works after pattern tests.
 *
 * ============================================================================
 * PATTERN-MATCHING BOUNDARY
 * ============================================================================
 *
 * The semantic type representation must expose sum alternatives to the
 * pattern-matching subsystem.
 *
 * Conceptually:
 *
 *     TypeExpr::Union
 *           |
 *           v
 *     pattern analysis
 *           |
 *           +--> narrowing
 *           +--> exhaustiveness
 *           +--> unreachable-pattern detection
 *
 * This file does not own those algorithms.
 *
 * ============================================================================
 * QUANTUM SEMANTIC BOUNDARY
 * ============================================================================
 *
 * If a sum contains quantum types, the type grammar remains completely
 * independent of physical quantum implementation.
 *
 * Required downstream boundary:
 *
 *     source TypeExpr
 *          |
 *          v
 *     semantic quantum type
 *          |
 *          v
 *     quantum::ir
 *
 * No:
 *
 *     SumQuantumIR
 *     UnionQuantumIR
 *     PhysicalSumIR
 *
 * may be introduced by this grammar.
 *
 * ============================================================================
 * HDL / HARDWARE SEMANTIC BOUNDARY
 * ============================================================================
 *
 * HDL/hardware-specific representation must remain downstream.
 *
 * The sum grammar cannot determine:
 *
 *     signal encoding;
 *     bus implementation;
 *     register allocation;
 *     synthesis strategy;
 *     FPGA placement;
 *     ASIC layout;
 *     clock routing;
 *     physical pin mapping.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar has NO direct IR output.
 *
 * Intended flow:
 *
 *     source syntax
 *          |
 *          v
 *     TypeExpr::Union
 *          |
 *          v
 *     semantic type model
 *          |
 *          +------------------+
 *          |                  |
 *          v                  v
 *     classical semantic   domain semantic
 *                              |
 *                              v
 *                         quantum::ir
 *
 * Any representation such as:
 *
 *     tagged value;
 *     discriminated value;
 *     compact nullable form;
 *     target-specific variant layout;
 *
 * is selected downstream.
 *
 * ============================================================================
 * RESOURCE / TARGET ADAPTATION
 * ============================================================================
 *
 * The same source sum:
 *
 *     A | B
 *
 * may be represented differently on different targets.
 *
 * That does not change the source-level type identity.
 *
 * Target adaptation may change:
 *
 *     representation;
 *     layout;
 *     storage;
 *     instruction selection;
 *     calling convention;
 *     parallelization;
 *     distribution;
 *     accelerator mapping;
 *     quantum realization.
 *
 * It must not silently change:
 *
 *     A | B
 *
 * into a different source semantic type.
 *
 * ============================================================================
 * POCO-REAF GUARANTEE
 * ============================================================================
 *
 * This grammar supports the POCO-REAF architecture by ensuring that anonymous
 * sums describe semantic alternatives rather than machine realization.
 *
 * A program may therefore use:
 *
 *     Result<T, E> | Pending
 *
 * without encoding whether the eventual implementation uses:
 *
 *     one CPU;
 *     many CPUs;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU;
 *     simulator;
 *     cluster;
 *     distributed execution;
 *     cloud infrastructure;
 *     future hardware.
 *
 * Target feasibility remains a downstream concern.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * ============================================================================
 *
 * The following must parse as valid sum types:
 *
 *     int | string
 *
 *     int | float | string
 *
 *     User | Error
 *
 *     Result<T, E> | Pending
 *
 *     Option<T> | Error
 *
 *     Qubit | ClassicalValue
 *
 *     LogicalQubit | Measurement
 *
 *     Signal<T> | ControlSignal
 *
 *     Tensor<T> | Scalar<T>
 *
 *     Local<T> | Remote<T>
 *
 *     A? | B
 *
 *     A | B?
 *
 *     Vec<A> | Vec<B>
 *
 *     Result<Vec<T>, E> | Pending
 *
 *     Resource<Capability<T>> | Unavailable
 *
 *     Matrix<T>[Rows, Cols] | SparseMatrix<T>
 *
 * ============================================================================
 * NESTED / COMPOSITION TESTS
 * ============================================================================
 *
 *     Option<(A | B)>
 *
 *     Result<(A | B), Error>
 *
 *     Vec<Result<A | B, E>>
 *
 *     Resource<Option<A | B>>
 *
 *     QuantumState<Result<A | B, E>>
 *
 * Parenthesization must be used wherever the complete type grammar requires it.
 *
 * ============================================================================
 * QUANTUM TESTS
 * ============================================================================
 *
 *     Qubit | ClassicalValue
 *
 *     LogicalQubit | PhysicalRepresentation
 *
 *     QuantumState<T> | ClassicalState<T>
 *
 *     QRegister<N> | EmptyRegister
 *
 * No test may assume a fixed physical qubit count.
 *
 * ============================================================================
 * HDL / HARDWARE TESTS
 * ============================================================================
 *
 *     Signal<T> | ControlSignal
 *
 *     Register<T> | MemoryReference
 *
 *     HardwareValue | SoftwareValue
 *
 * Tests must not encode fixed:
 *
 *     bus width;
 *     register width;
 *     device count;
 *     memory size;
 *     FPGA capacity.
 *
 * ============================================================================
 * AI / DATA TESTS
 * ============================================================================
 *
 *     Tensor<T> | Scalar<T>
 *
 *     Model<Input, Output> | UntrainedModel
 *
 *     Dataset<T> | EmptyDataset
 *
 *     Distribution<T> | Deterministic<T>
 *
 * The grammar must remain independent of particular algorithms.
 *
 * ============================================================================
 * DISTRIBUTED TESTS
 * ============================================================================
 *
 *     Local<T> | Remote<T>
 *
 *     Replica<T> | Unavailable
 *
 *     Message<T> | Failure
 *
 * No fixed node or cluster count is permitted.
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 *     |
 *
 *     | A
 *
 *     A |
 *
 *     A || B
 *
 *     A | | B
 *
 *     A | | | B
 *
 *     A B
 *
 *     A, B
 *
 *     A ;
 *
 *     A <| B
 *
 * The parser must reject malformed alternatives rather than silently
 * interpreting them as another type construct.
 *
 * ============================================================================
 * SINGLE-MEMBER TEST
 * ============================================================================
 *
 *     A
 *
 * is NOT a sum.
 *
 * It must continue to parse as an ordinary type through `typeCore`.
 *
 * ============================================================================
 * DUPLICATE-MEMBER TEST
 * ============================================================================
 *
 *     A | A
 *
 * must be syntactically accepted.
 *
 * Whether it is:
 *
 *     rejected;
 *     normalized;
 *     warned;
 *     preserved;
 *
 * is determined by semantic analysis.
 *
 * ============================================================================
 * SOURCE-ORDER TEST
 * ============================================================================
 *
 * Input:
 *
 *     A | B | C
 *
 * Required source order:
 *
 *     A
 *     B
 *     C
 *
 * No parser transformation may reorder alternatives.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * The test generator should construct increasingly large source sums:
 *
 *     T0 | T1
 *     T0 | T1 | T2
 *     T0 | T1 | T2 | T3
 *     ...
 *
 * The test suite must NOT define a universal maximum member count.
 *
 * The purpose of scalability testing is to measure implementation behavior,
 * not to establish a language ceiling.
 *
 * If a compiler resource policy rejects an extremely large source file, the
 * diagnostic must identify the implementation/resource policy rather than
 * claiming that the language forbids the source form.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Parse the same source repeatedly:
 *
 *     A | B | C
 *
 * and verify identical:
 *
 *     member order;
 *     nesting;
 *     token association;
 *     source spans;
 *     resulting AST structure.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST
 * ============================================================================
 *
 * A mandatory integration fixture should combine:
 *
 *     Result<T, E>
 *     |
 *     Qubit
 *     |
 *     Tensor<T>
 *     |
 *     Resource<T>
 *     |
 *     distributed type
 *
 * into a legal source-level sum where the surrounding type grammar permits it.
 *
 * The test must verify that the type representation remains domain-neutral.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The frontend must preserve locations for:
 *
 *     sumType;
 *     each sumTypeMember;
 *     every PIPE;
 *     nested child type expressions.
 *
 * These spans are required for:
 *
 *     diagnostics;
 *     IDE/LSP;
 *     formatter;
 *     refactoring;
 *     source maps;
 *     provenance;
 *     compatibility tooling.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/types/types.g4
 * ============================================================================
 *
 * `types.g4` remains the sole public type-expression composition root.
 *
 * It must import/combine this `Sum` delegate according to the repository's
 * ANTLR composition mechanism.
 *
 * Required architectural relationship:
 *
 *     Types
 *       |
 *       +--> Sum
 *
 * The complete type-expression grammar must conceptually become:
 *
 *     typeExpression
 *         : typeQualifier*
 *           (
 *               sumType
 *             | typeCore typePostfix*
 *           )
 *         ;
 *
 * The precise factoring may differ if required by the existing ANTLR grammar
 * composition, but the following invariants are mandatory:
 *
 *     - typeExpression remains unique;
 *     - typeCore remains unique;
 *     - typePostfix remains unique;
 *     - sumType remains unique;
 *     - sumTypeMember never calls typeExpression;
 *     - no circular grammar dependency exists.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/types/union.g4
 * ============================================================================
 *
 * Existing anonymous-union syntax must converge on this file.
 *
 * The preferred final state is:
 *
 *     grammar/types/sum.g4
 *         |
 *         +-- canonical anonymous sum syntax
 *
 *     grammar/types/union.g4
 *         |
 *         +-- compatibility/deprecated forwarding surface
 *
 * `union.g4` must not independently define:
 *
 *     typeUnion
 *     unionTypeMember
 *
 * with a second implementation of:
 *
 *     A | B
 *
 * The migration must preserve existing consumers where practical while
 * establishing one canonical syntax owner.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/types/algebraic-types.g4
 * ============================================================================
 *
 * No direct dependency is required.
 *
 * `algebraic-types.g4` remains responsible for constructor-bearing algebraic
 * syntax.
 *
 * The semantic layer may eventually consume:
 *
 *     TypeExpr::Union
 *
 * together with algebraic declarations.
 *
 * The grammar layers must remain separate.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/declarations/unions.g4
 * ============================================================================
 *
 * Named union declarations continue to own:
 *
 *     unionDeclaration
 *     unionVariantList
 *     unionVariant
 *     unionVariantPayload
 *
 * This file must not import or duplicate those declaration rules.
 *
 * A declaration may use anonymous sums in payload types where the canonical
 * `typeExpression` permits them.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/types/result.g4
 * ============================================================================
 *
 * Result remains a specialized type constructor:
 *
 *     Result<T, E>
 *
 * and can participate as a sum member:
 *
 *     Result<T, E> | Pending
 *
 * This file does not duplicate Result syntax.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/types/generic.g4
 * ============================================================================
 *
 * Generic applications remain owned by the canonical generic grammar.
 *
 * This file consumes them indirectly through:
 *
 *     typeCore
 *
 * It must not redefine:
 *
 *     genericType
 *     genericTypeArguments
 *     genericArgumentList
 *
 * ============================================================================
 * INTEGRATION WITH grammar/types/tuple.g4
 * ============================================================================
 *
 * Tuple types remain independently owned.
 *
 * Parenthesized forms are particularly important because:
 *
 *     (A | B)
 *
 * is a complete type expression that may then participate in another type
 * constructor.
 *
 * This file must not redefine tuple syntax.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/types/function.g4
 * ============================================================================
 *
 * Function types remain independently owned.
 *
 * This file only consumes the composed function type through `typeCore`.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/types/dependent.g4
 * ============================================================================
 *
 * Dependent/value-parameterized type syntax remains independently owned.
 *
 * This file only consumes its resulting type structure through `typeCore`.
 *
 * ============================================================================
 * INTEGRATION WITH AST
 * ============================================================================
 *
 * Existing canonical frontend AST:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * already defines:
 *
 *     TypeExpr::Union(Vec<TypeExpr>)
 *
 * Therefore no Rust AST modification is required solely to create this grammar
 * file.
 *
 * The parser/frontend adapter must map:
 *
 *     Sum.sumType
 *
 * to:
 *
 *     TypeExpr::Union
 *
 * preserving member order and source locations.
 *
 * If the AST builder currently maps `grammar/types/union.g4` contexts to the
 * same `TypeExpr::Union`, the adapter should be migrated to recognize the
 * canonical `sumType` context without creating a second representation.
 *
 * ============================================================================
 * INTEGRATION WITH SEMANTIC TYPE ANALYSIS
 * ============================================================================
 *
 * Semantic analysis consumes:
 *
 *     TypeExpr::Union
 *
 * and is responsible for:
 *
 *     member resolution;
 *     normalization;
 *     duplicate detection;
 *     subtype relationships;
 *     assignability;
 *     narrowing;
 *     exhaustiveness;
 *     recursive-type validity;
 *     ownership/linearity;
 *     effects;
 *     capabilities;
 *     resources;
 *     target feasibility.
 *
 * No semantic operation belongs in this grammar.
 *
 * ============================================================================
 * INTEGRATION WITH QUANTUM SEMANTICS
 * ============================================================================
 *
 * When a union contains quantum types:
 *
 *     TypeExpr::Union
 *          |
 *          v
 *     semantic type analysis
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * This file has no dependency on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     calibration
 *     HAL.
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCE / CAPABILITY ANALYSIS
 * ============================================================================
 *
 * Resource and capability requirements remain separate from type syntax.
 *
 * Example:
 *
 *     requires capability("quantum.measurement");
 *
 * remains owned by the resource/capability subsystem.
 *
 * A sum type containing:
 *
 *     Qubit
 *
 * does not automatically create a physical-resource request.
 *
 * Semantic analysis may derive requirements where the language specification
 * explicitly defines such behavior, but that derivation is not parser syntax.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECTS
 * ============================================================================
 *
 * A sum type has no intrinsic effect.
 *
 * Executable operations involving a sum may have effects.
 *
 * The effect subsystem consumes semantic information after parsing.
 *
 * ============================================================================
 * INTEGRATION WITH CONTRACTS / POLICIES
 * ============================================================================
 *
 * Contract and policy systems may consume union/sum type information.
 *
 * They remain independent syntax owners.
 *
 * Required direction:
 *
 *     Sum
 *       |
 *       v
 *     TypeExpr::Union
 *       |
 *       +--> contracts
 *       +--> policies
 *       +--> validation
 *       +--> provenance
 *
 * ============================================================================
 * INTEGRATION WITH PATTERN MATCHING
 * ============================================================================
 *
 * Required semantic direction:
 *
 *     TypeExpr::Union
 *          |
 *          v
 *     match/pattern semantic analysis
 *          |
 *          +--> narrowing
 *          +--> exhaustiveness
 *          +--> unreachable pattern analysis
 *
 * Pattern grammar is not duplicated here.
 *
 * ============================================================================
 * INTEGRATION WITH COMPILER IR
 * ============================================================================
 *
 * This file has no direct IR dependency.
 *
 * Canonical direction:
 *
 *     source
 *       |
 *       v
 *     TypeExpr::Union
 *       |
 *       v
 *     semantic type
 *       |
 *       +------------------+
 *       |                  |
 *       v                  v
 *   classical           quantum
 *   semantic            semantic
 *                         |
 *                         v
 *                     quantum::ir
 *
 * The compiler may later choose:
 *
 *     tagged representation;
 *     discriminated representation;
 *     optimized variant representation;
 *     target-specific representation.
 *
 * That choice must not be encoded in this file.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime representation is downstream.
 *
 * The runtime may represent a sum using:
 *
 *     discriminant + payload;
 *     optimized nullable representation;
 *     target-native variant;
 *     another semantically equivalent representation.
 *
 * This grammar does not prescribe one.
 *
 * ============================================================================
 * ABI INTEGRATION
 * ============================================================================
 *
 * ABI layout remains downstream.
 *
 * The grammar does not determine:
 *
 *     alignment;
 *     padding;
 *     register placement;
 *     calling convention;
 *     object layout;
 *     pointer width.
 *
 * ============================================================================
 * RUST INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation.
 *
 * Rust components consuming this grammar must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * The repository's Rust implementation should retain:
 *
 *     #![forbid(unsafe_code)]
 *
 * where applicable.
 *
 * This grammar must never require:
 *
 *     unsafe;
 *     raw pointer manipulation;
 *     target-specific compiler intrinsics;
 *     nightly-only language features.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * FORBIDDEN:
 *
 *     MAX_SUM_MEMBERS
 *     MAX_UNION_MEMBERS
 *     MAX_VARIANTS
 *     MAX_TYPE_DEPTH
 *     MAX_GENERIC_ARITY
 *     MAX_TUPLE_ARITY
 *     MAX_ARRAY_LENGTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Also forbidden:
 *
 *     finite alternative enumerations;
 *     vendor-specific variants;
 *     physical hardware identifiers;
 *     fixed topology assumptions;
 *     target-specific parser behavior.
 *
 * ALLOWED:
 *
 *     PIPE as the language-level sum separator;
 *     recursive/repetitive parser rules;
 *     source-level type expressions;
 *     explicit semantic constraints;
 *     compiler resource policies outside grammar semantics.
 *
 * ============================================================================
 * NO LANGUAGE CEILING
 * ============================================================================
 *
 * The following must remain represented by grammar repetition rather than
 * finite alternatives:
 *
 *     member count;
 *     nesting;
 *     generic structure;
 *     domain composition;
 *     target-independent type complexity.
 *
 * The grammar therefore remains suitable for source programs ranging from
 * tiny embedded computations to very large distributed or heterogeneous
 * computational systems, subject to actual implementation resources.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] It exists at:
 *         grammar/types/sum.g4
 *
 * [ ] It is a parser grammar.
 *
 * [ ] It consumes the canonical ZamaniLexer vocabulary.
 *
 * [ ] It defines no lexer rules.
 *
 * [ ] It defines no Rust actions.
 *
 * [ ] It defines no semantic predicates.
 *
 * [ ] It defines `sumType`.
 *
 * [ ] It defines `sumTypeMember`.
 *
 * [ ] `sumType` requires at least two members.
 *
 * [ ] A single `A` remains an ordinary type.
 *
 * [ ] `A | B` parses as a sum.
 *
 * [ ] `A | B | C` parses as a sum.
 *
 * [ ] Member order is preserved.
 *
 * [ ] Generic members are supported through `typeCore`.
 *
 * [ ] Quantum members are supported through `typeCore`.
 *
 * [ ] HDL/hardware members are supported through `typeCore`.
 *
 * [ ] AI/data members are supported through `typeCore`.
 *
 * [ ] Distributed members are supported through `typeCore`.
 *
 * [ ] Dependent members are supported through `typeCore`.
 *
 * [ ] Postfix type syntax is consumed through `typePostfix`.
 *
 * [ ] `typeExpression` is not duplicated.
 *
 * [ ] `typeCore` is not duplicated.
 *
 * [ ] `typePostfix` is not duplicated.
 *
 * [ ] Generic grammar is not duplicated.
 *
 * [ ] Algebraic constructor syntax is not duplicated.
 *
 * [ ] Named union declarations are not duplicated.
 *
 * [ ] Result syntax is not duplicated.
 *
 * [ ] No semantic type checking occurs here.
 *
 * [ ] No resource allocation occurs here.
 *
 * [ ] No hardware selection occurs here.
 *
 * [ ] No quantum physical mapping occurs here.
 *
 * [ ] No QEC occurs here.
 *
 * [ ] No routing occurs here.
 *
 * [ ] No scheduling occurs here.
 *
 * [ ] No calibration occurs here.
 *
 * [ ] No HAL dependency exists.
 *
 * [ ] Existing `TypeExpr::Union(Vec<TypeExpr>)` remains the AST target.
 *
 * [ ] Existing anonymous-union grammar duplication is removed/forwarded.
 *
 * [ ] `types.g4` remains the sole public type-expression composition root.
 *
 * [ ] No language-level cardinality ceiling exists.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * The complete invariant is:
 *
 *     A | B
 *       |
 *       v
 *     Sum.sumType
 *       |
 *       v
 *     TypeExpr::Union
 *       |
 *       v
 *     semantic type
 *       |
 *       +-------------------------+
 *       |                         |
 *       v                         v
 *   classical                  quantum
 *   semantics                  semantics
 *                                 |
 *                                 v
 *                             quantum::ir
 *                                 |
 *                                 v
 *                        target-independent
 *                           optimization
 *                                 |
 *                                 v
 *                         target realization
 *
 * The source-level sum describes semantic alternatives.
 *
 * It does not describe:
 *
 *     physical storage;
 *     machine size;
 *     device count;
 *     qubit count;
 *     memory capacity;
 *     processor count;
 *     topology;
 *     vendor implementation.
 *
 * ============================================================================
 * FINAL RULES
 * ============================================================================
 *
 * 1. `sum.g4` owns anonymous A | B syntax.
 *
 * 2. `types.g4` remains the only complete type-expression composition root.
 *
 * 3. `sumTypeMember` consumes `typeCore`, never `typeExpression`.
 *
 * 4. `TypeExpr::Union(Vec<TypeExpr>)` is the canonical frontend AST target.
 *
 * 5. Named union declarations remain outside this file.
 *
 * 6. Algebraic constructors remain outside this file.
 *
 * 7. Semantic normalization remains outside this file.
 *
 * 8. Pattern matching remains outside this file.
 *
 * 9. Effects remain outside this file.
 *
 * 10. Resources and capabilities remain outside this file.
 *
 * 11. Hardware realization remains outside this file.
 *
 * 12. Quantum physical realization remains outside this file.
 *
 * 13. `quantum::ir` remains the canonical quantum IR boundary.
 *
 * 14. No fixed computational capacity is encoded.
 *
 * 15. No implementation-specific resource limit becomes language semantics.
 *
 * 16. No unsafe Rust is required.
 *
 * 17. Rust consumers remain compatible with Rust 1.97/1.97.1.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar Sum;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * ANONYMOUS SUM TYPE
 * ============================================================================
 *
 * At least two members are required.
 *
 *     A | B
 *     A | B | C
 *     A | B | C | D
 *
 * A single member is deliberately not classified as a sum.
 */
sumType
    : sumTypeMember
      (PIPE sumTypeMember)+
    ;


/*
 * ============================================================================
 * SUM MEMBER
 * ============================================================================
 *
 * IMPORTANT:
 *
 * Do not replace `typeCore` with `typeExpression`.
 *
 * Doing so would create recursive composition:
 *
 *     typeExpression
 *         -> sumType
 *             -> sumTypeMember
 *                 -> typeExpression
 *
 * The canonical Types grammar owns the complete expression composition.
 */
sumTypeMember
    : typeCore
      typePostfix*
    ;