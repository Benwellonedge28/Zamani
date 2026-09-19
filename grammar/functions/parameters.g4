/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/parameters.g4
 *
 * Role:
 *     Canonical reusable parser grammar for callable parameters.
 *
 * Grammar layer:
 *     Parser-only delegate grammar.
 *
 * Language:
 *     Zamani
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no Rust implementation code.
 *     Zamani compiler/frontend/runtime Rust MUST remain safe Rust.
 *     No unsafe Rust is required by this grammar.
 *
 * ============================================================================
 * 1. ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * Canonical lexical system
 *   |
 *   v
 * Canonical parser composition
 *   |
 *   +-------------------------------+
 *   |                               |
 *   v                               v
 * Parameters.g4                 Other grammar domains
 *   |                               |
 *   +---------------+---------------+
 *                   |
 *                   v
 *             Frontend AST
 *                   |
 *                   v
 *             Semantic analysis
 *                   |
 *       +-----------+-----------+
 *       |                       |
 *       v                       v
 * Classical IR             quantum::ir
 *       |                       |
 *       +-----------+-----------+
 *                   |
 *                   v
 *       optimization / lowering
 *                   |
 *       +-----------+-----------+
 *       |           |           |
 *       v           v           v
 *    routing    scheduling   resilience
 *                   |
 *                   v
 *                  ZQN
 *                   |
 *                   v
 *                  HAL
 *                   |
 *                   v
 *           target realization
 *
 * This grammar owns ONLY the concrete syntax of callable parameters.
 *
 * ============================================================================
 * 2. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     parameterList
 *     parameter
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifier
 *     typeExpression
 *     expression
 *     patterns
 *     generic parameter declarations
 *     generic substitution
 *     type inference
 *     name resolution
 *     ownership
 *     borrowing
 *     lifetimes
 *     effect checking
 *     capability checking
 *     resource allocation
 *     ABI selection
 *     calling conventions
 *     optimization
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *     hardware topology
 *     physical resource assignment
 *
 * Those responsibilities remain in their canonical subsystems.
 *
 * ============================================================================
 * 3. CANONICAL PARAMETER CONTRACT
 * ============================================================================
 *
 * The source-level parameter form owned by this grammar is:
 *
 *     ParameterList
 *         ::= Parameter ("," Parameter)*
 *
 *     Parameter
 *         ::= "mut"? Identifier
 *             (":" TypeExpression)?
 *             ("=" Expression)?
 *
 * Therefore these forms are syntactically supported:
 *
 *     fn f()
 *
 *     fn f(x)
 *
 *     fn f(x: Int)
 *
 *     fn f(mut x)
 *
 *     fn f(mut x: Int)
 *
 *     fn f(x = default_value)
 *
 *     fn f(x: Int = default_value)
 *
 *     fn f(mut x: Int = default_value)
 *
 * The surrounding function grammar owns the optionality of the complete
 * parameter list:
 *
 *     LPAREN parameterList? RPAREN
 *
 * This grammar therefore intentionally does not define an empty parameter
 * list.
 *
 * ============================================================================
 * 4. AST CONTRACT
 * ============================================================================
 *
 * The existing Zamani AST contains:
 *
 *     Parameter {
 *         name: Identifier,
 *         typ: Option<TypeExpr>,
 *         default: Option<Expression>,
 *         is_self: bool,
 *         is_mutable: bool,
 *     }
 *
 * This grammar supplies the syntax needed for:
 *
 *     name
 *     typ
 *     default
 *     is_mutable
 *
 * `is_self` is NOT syntactically created by this grammar.
 *
 * A self/receiver parameter, if supported by a surrounding declaration
 * grammar, must have its own explicit language contract and must not be
 * silently smuggled into ordinary parameter syntax.
 *
 * This grammar MUST NOT introduce another parameter AST structure.
 *
 * ============================================================================
 * 5. SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This grammar establishes syntax only.
 *
 * Semantic analysis determines:
 *
 *     - whether the parameter name is legal;
 *     - whether names are unique;
 *     - whether the parameter type is valid;
 *     - whether type inference is permitted;
 *     - whether a default value is type-compatible;
 *     - whether a default value is legal in that callable context;
 *     - whether mutation is legal for the parameter type;
 *     - ownership and borrowing semantics;
 *     - effect requirements;
 *     - capability requirements;
 *     - resource requirements;
 *     - ABI representation;
 *     - calling convention;
 *     - lowering strategy.
 *
 * None of those decisions belong here.
 *
 * ============================================================================
 * 6. POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * Parameter syntax MUST remain independent of target size.
 *
 * This grammar contains NO limits for:
 *
 *     MAX_PARAMETERS
 *     MAX_ARGUMENTS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_VECTOR_WIDTH
 *     MAX_NETWORK_SIZE
 *
 * A parameter can represent a value whose eventual semantic type describes
 * any supported computational domain:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI/ML
 *     data
 *     networking
 *     security
 *     accelerator
 *     future domains
 *
 * The parameter grammar does not need to know which domain a type belongs to.
 *
 * Repetition is represented structurally with:
 *
 *     (COMMA parameter)*
 *
 * rather than finite grammar expansion.
 *
 * Practical compiler/parser resource limits, if required, belong to explicit
 * implementation/resource policy and MUST NOT become language semantics.
 *
 * ============================================================================
 * 7. TARGET INDEPENDENCE
 * ============================================================================
 *
 * These are intentionally NOT parameter grammar constructs:
 *
 *     cpuParameter
 *     gpuParameter
 *     fpgaParameter
 *     qpuParameter
 *     physicalQubitParameter
 *     physicalRegisterParameter
 *     deviceParameter
 *     vendorParameter
 *     cudaParameter
 *     acceleratorParameter
 *
 * A programmer may name a source-level value:
 *
 *     device
 *     accelerator
 *     qpu
 *     memory
 *     topology
 *
 * but the parameter grammar does not determine what those names denote.
 *
 * Target realization is downstream.
 *
 * ============================================================================
 * 8. QUANTUM INTEGRATION
 * ============================================================================
 *
 * A quantum parameter is syntactically an ordinary parameter.
 *
 * For example:
 *
 *     fn execute(q: Qubit) { ... }
 *
 * or:
 *
 *     fn execute(register: QuantumRegister) { ... }
 *
 * The type grammar and semantic system determine the meaning.
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     LogicalQubitId
 *     MAX_QUBITS
 *     physical topology
 *     coupling maps
 *     gate sets
 *     calibration
 *     QEC
 *     noise models
 *     scheduling
 *
 * Quantum lowering remains:
 *
 *     source
 *       ->
 *     frontend AST
 *       ->
 *     semantic analysis
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     QEC / resilience
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * 9. CLASSICAL / HDL / HARDWARE / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * The same parameter syntax applies to all computational domains.
 *
 * Examples:
 *
 *     fn compute(x: Scalar) { ... }
 *
 *     fn transform(data: Tensor) { ... }
 *
 *     fn execute(q: Qubit) { ... }
 *
 *     fn synthesize(module: HardwareModule) { ... }
 *
 *     fn distribute(value: DistributedValue) { ... }
 *
 *     fn infer(model: Model) { ... }
 *
 * No domain-specific parameter grammar is required merely because the type
 * belongs to a different domain.
 *
 * ============================================================================
 * 10. GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic declaration syntax belongs to:
 *
 *     grammar/functions/generics.g4
 *
 * For example:
 *
 *     fn identity<T>(value: T) -> T
 *
 * This file owns only:
 *
 *     value: T
 *
 * It does NOT own:
 *
 *     <T>
 *
 * Generic type application remains owned by the type grammar.
 *
 * ============================================================================
 * 11. PATTERN / DESTRUCTURING POLICY
 * ============================================================================
 *
 * This grammar intentionally accepts an identifier binding, not an arbitrary
 * pattern:
 *
 *     mut? identifier
 *
 * It therefore does NOT independently introduce:
 *
 *     tuple destructuring
 *     array destructuring
 *     struct destructuring
 *     or-patterns
 *     range patterns
 *     arbitrary match patterns
 *
 * This is deliberate.
 *
 * If Zamani adopts parameter destructuring in a future language version, the
 * change MUST be coordinated across:
 *
 *     specification
 *     lexer, if new tokens are required
 *     parameter grammar
 *     pattern grammar
 *     AST
 *     semantic analysis
 *     diagnostics
 *     compatibility
 *     positive tests
 *     negative tests
 *     boundary tests
 *     scalability tests
 *
 * No future pattern feature should be inserted here merely for convenience.
 *
 * ============================================================================
 * 12. DEFAULT-VALUE CONTRACT
 * ============================================================================
 *
 * The default value is parsed as the canonical expression:
 *
 *     ASSIGN expression
 *
 * This grammar does not evaluate it.
 *
 * It does not decide whether a default expression is:
 *
 *     compile-time
 *     runtime
 *     constant
 *     pure
 *     effectful
 *     deterministic
 *     resource-consuming
 *
 * Those properties belong to semantic analysis and the execution model.
 *
 * ============================================================================
 * 13. DEFAULT-ORDERING CONTRACT
 * ============================================================================
 *
 * The canonical syntactic order is:
 *
 *     mut?
 *     identifier
 *     type?
 *     default?
 *
 * Therefore:
 *
 *     mut x: Int = value
 *
 * is syntactically valid.
 *
 * These forms are not accepted:
 *
 *     x = value: Int
 *     x: = value
 *     x = : Int
 *
 * The grammar must not reinterpret malformed syntax as another parameter
 * construct.
 *
 * ============================================================================
 * 14. TRAILING-COMMA POLICY
 * ============================================================================
 *
 * This file deliberately follows the canonical non-trailing-comma parameter
 * list contract:
 *
 *     parameter (COMMA parameter)*
 *
 * Therefore:
 *
 *     fn f(a, b)
 *
 * is valid, while:
 *
 *     fn f(a, b,)
 *
 * is not accepted by this grammar.
 *
 * This is intentional.
 *
 * Trailing-comma policy is a language-wide syntax decision and MUST NOT be
 * independently invented by individual grammar components.
 *
 * If Zamani adopts trailing commas for parameter lists in a future version,
 * the specification, grammar, AST/test expectations, and compatibility policy
 * must be changed together.
 *
 * ============================================================================
 * 15. LEXER INTEGRATION
 * ============================================================================
 *
 * The canonical lexical composition is:
 *
 *     grammar/lexer/tokens.g4
 *
 * which composes the modular lexical authorities, including:
 *
 *     ZamaniKeywords
 *     ZamaniOperators
 *     ZamaniPunctuation
 *     ZamaniIdentifiers
 *     ZamaniLiterals
 *     ZamaniComments
 *     ZamaniAnnotations
 *     ZamaniLexerErrors
 *
 * Parser grammars consume the resulting canonical token vocabulary.
 *
 * This file therefore uses:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * and does not define lexical rules itself.
 *
 * Relevant canonical tokens include:
 *
 *     MUT
 *     COMMA
 *     COLON
 *     ASSIGN
 *
 * `MUT` is owned by `grammar/lexer/keywords.g4`.
 *
 * `COMMA` and `COLON` are owned by `grammar/lexer/punctuation.g4`.
 *
 * `ASSIGN` is owned by `grammar/lexer/operators.g4`.
 *
 * No new lexer token is required for the parameter syntax currently owned by
 * this file.
 *
 * ============================================================================
 * 16. IDENTIFIER INTEGRATION
 * ============================================================================
 *
 * Identifier syntax belongs to the canonical identifier/name grammar.
 *
 * This file deliberately consumes:
 *
 *     identifier
 *
 * rather than creating another identifier rule.
 *
 * This prevents:
 *
 *     parameters.g4
 *
 * from becoming a competing identifier authority.
 *
 * ============================================================================
 * 17. TYPE INTEGRATION
 * ============================================================================
 *
 * Type syntax belongs to the canonical type grammar.
 *
 * This file consumes:
 *
 *     typeExpression
 *
 * rather than redefining:
 *
 *     primitive types
 *     generic types
 *     tuple types
 *     function types
 *     arrays
 *     slices
 *     references
 *     resource types
 *     quantum types
 *     hardware types
 *     tensor types
 *     future domain types
 *
 * This permits one parameter grammar to remain valid as the type system
 * expands.
 *
 * ============================================================================
 * 18. EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Default values consume:
 *
 *     expression
 *
 * from the canonical expression grammar.
 *
 * This means default expressions automatically gain whatever future
 * target-independent expression facilities the language adopts without
 * requiring parameters.g4 to be edited.
 *
 * ============================================================================
 * 19. ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser delegate grammar.
 *
 * Canonical function grammar composition is responsible for importing it:
 *
 *     import FunctionGenerics, Parameters, Types, Expressions, Statements;
 *
 * as already established by the current functions grammar architecture.
 *
 * The resulting parser exposes:
 *
 *     parameterList
 *     parameter
 *
 * to the parent grammar.
 *
 * The parent function grammar is responsible for:
 *
 *     LPAREN
 *     parameterList?
 *     RPAREN
 *
 * This file must not duplicate function declaration syntax.
 *
 * ============================================================================
 * 20. DEPENDENCY DIRECTION
 * ============================================================================
 *
 * Lexical layer
 *      |
 *      v
 * Parameters.g4
 *      |
 *      +--> identifier/name grammar
 *      +--> type grammar
 *      +--> expression grammar
 *      |
 *      v
 * Functions.g4
 *      |
 *      v
 * AST
 *      |
 *      v
 * Semantic analysis
 *      |
 *      v
 * IR
 *
 * This file MUST NOT depend on:
 *
 *     AST implementation
 *     compiler implementation
 *     runtime implementation
 *     quantum::ir
 *     QEC
 *     ZQN
 *     HAL
 *     routing
 *     scheduling
 *     hardware
 *
 * ============================================================================
 * 21. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * The grammar must reject structurally malformed parameters rather than
 * inventing missing information.
 *
 * Invalid examples:
 *
 *     fn f(, x)
 *     fn f(x:)
 *     fn f(: Int)
 *     fn f(x =)
 *     fn f(mut)
 *     fn f(x: Int =)
 *     fn f(x, , y)
 *     fn f(x, y,)
 *
 * The parser/frontend diagnostic subsystem owns the final diagnostic wording,
 * source span presentation, recovery strategy, and error aggregation.
 *
 * This grammar must not:
 *
 *     - invent names;
 *     - invent types;
 *     - invent defaults;
 *     - silently discard malformed parameters;
 *     - reinterpret malformed parameter syntax as another declaration.
 *
 * ============================================================================
 * 22. DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source text
 *     language version
 *     lexical configuration
 *
 * this grammar must produce the same parse structure.
 *
 * Parameter order is source order.
 *
 * Bound/default/type structure is preserved for downstream AST construction.
 *
 * No runtime, filesystem, network, hardware, scheduler, random, or
 * environment-dependent behavior is permitted.
 *
 * ============================================================================
 * 23. SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing parameters has no side effects.
 *
 * A default expression is syntax at this stage.
 *
 * The parser MUST NOT:
 *
 *     execute default expressions;
 *     invoke functions;
 *     access files;
 *     access networks;
 *     inspect credentials;
 *     discover hardware;
 *     allocate target resources;
 *     execute quantum operations;
 *     invoke external processes.
 *
 * ============================================================================
 * 24. COMPATIBILITY
 * ============================================================================
 *
 * Existing source syntax represented by the current Rust frontend:
 *
 *     fn name(x)
 *     fn name(x: Type)
 *     fn name(mut x)
 *     fn name(mut x: Type)
 *
 * remains compatible with this grammar.
 *
 * The existing AST representation:
 *
 *     Parameter
 *
 * remains the canonical downstream representation.
 *
 * No parameter token renaming is introduced here.
 *
 * ============================================================================
 * 25. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no machine limits
 *     no hardware IDs
 *     no physical addresses
 *     no qubit limits
 *     no CPU limits
 *     no GPU limits
 *     no FPGA limits
 *     no node limits
 *     no memory limits
 *     no tensor limits
 *     no register limits
 *     no vendor-specific ABI
 *     no backend-specific syntax
 *
 * The parameter count is represented with unbounded grammar repetition.
 *
 * ============================================================================
 * 26. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It owns only parameter-list and parameter syntax.
 *     [x] It uses the canonical token vocabulary.
 *     [x] It does not define lexer rules.
 *     [x] It does not duplicate identifier syntax.
 *     [x] It does not duplicate type syntax.
 *     [x] It does not duplicate expression syntax.
 *     [x] It matches the existing Parameter AST model.
 *     [x] It preserves mutable-parameter syntax.
 *     [x] It preserves optional type annotations.
 *     [x] It preserves optional default expressions.
 *     [x] It imposes no parameter-count limit.
 *     [x] It imposes no machine-resource limit.
 *     [x] It imposes no quantum-resource limit.
 *     [x] It does not create a second quantum IR.
 *     [x] It does not depend on hardware.
 *     [x] It does not depend on runtime behavior.
 *     [x] It contains no Rust unsafe requirement.
 *     [x] It remains deterministic.
 *     [x] It has a defined diagnostic boundary.
 *     [x] It has a defined compatibility boundary.
 *     [x] It has an explicit integration contract.
 *
 * ============================================================================
 * 27. CANONICAL GRAMMAR
 * ============================================================================
 */

parser grammar Parameters;

options {
    tokenVocab = ZamaniTokens;
};


/*
 * ============================================================================
 * PARAMETER LIST
 * ============================================================================
 *
 * ParameterList ::= Parameter ("," Parameter)*
 *
 * There is no finite cardinality encoded here.
 *
 * The enclosing function declaration decides whether this list is optional.
 */
parameterList
    : parameter
      (COMMA parameter)*
    ;


/*
 * ============================================================================
 * PARAMETER
 * ============================================================================
 *
 * Parameter ::=
 *     "mut"? Identifier
 *     (":" TypeExpression)?
 *     ("=" Expression)?
 *
 * The grammar intentionally delegates:
 *
 *     identifier
 *     typeExpression
 *     expression
 *
 * to their canonical grammar owners.
 */
parameter
    : MUT? identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
    ;