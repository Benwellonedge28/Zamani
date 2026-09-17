/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/variables.g4
 *
 * Purpose:
 *     Canonical parser grammar for source-level variable bindings.
 *
 * Ownership:
 *     This file owns the syntax of `let` and `var` variable declarations.
 *
 * It does NOT own:
 *     - lexer/token definitions
 *     - identifier syntax
 *     - type-expression syntax
 *     - expression syntax
 *     - statement syntax as a whole
 *     - constant declarations
 *     - function declarations
 *     - module declarations
 *     - ownership/borrowing semantics
 *     - memory allocation
 *     - hardware placement
 *     - quantum resource allocation
 *     - routing
 *     - scheduling
 *     - QEC
 *     - ZQN
 *     - HAL
 *     - runtime execution
 *
 * Architecture:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     parser composition
 *       |
 *       +--> core/names.g4
 *       +--> types/types.g4
 *       +--> expressions/expressions.g4
 *       |
 *       v
 *     declarations/variables.g4
 *       |
 *       v
 *     declarations/declarations.g4
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> name resolution
 *       +--> type checking
 *       +--> mutability
 *       +--> definite initialization
 *       +--> ownership/lifetime
 *       +--> effects/capabilities
 *       +--> resource semantics
 *       |
 *       v
 *     canonical semantic IR
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware IR
 *       +--> distributed/accelerator IR
 *       |
 *       v
 *     optimization / routing / scheduling / resilience
 *       |
 *       v
 *     target realization
 *
 * POCO-REAF:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Run Anywhere
 *     Run Forever
 *
 * This grammar contains no target-specific resource limits.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     Grammar only.
 *     No embedded Rust.
 *     No unsafe.
 *     No semantic predicates.
 *     No filesystem access.
 *     No network access.
 *     No hardware discovery.
 *     No runtime execution.
 *
 * ============================================================================
 */

parser grammar ZamaniVariableDeclarations;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY RULE
 * ============================================================================
 *
 * Exactly one variable declaration is parsed by this rule.
 *
 * Examples:
 *
 *     let x = 1;
 *     let x: Integer = 1;
 *     var x = compute();
 *     var x: Tensor<Real>;
 *
 * Whether an uninitialized `let` is semantically legal is NOT decided here.
 */
variableDeclaration
    : variableBindingKind
      identifier
      variableTypeAnnotation?
      variableInitializer?
      declarationTerminator
    ;


/*
 * ============================================================================
 * BINDING KIND
 * ============================================================================
 *
 * `let` and `var` are intentionally distinct syntactic alternatives.
 *
 * `const` belongs exclusively to constants.g4.
 *
 * Do not add hardware-specific variants such as:
 *
 *     gpuVar
 *     qpuVar
 *     fpgaVar
 *     registerVar
 *
 * Those would violate the target-independent language boundary.
 */
variableBindingKind
    : LET
    | VAR
    ;


/*
 * ============================================================================
 * OPTIONAL EXPLICIT TYPE
 * ============================================================================
 *
 * Type syntax belongs to grammar/types/.
 *
 * This rule merely establishes the declaration-level `:` boundary.
 *
 * It deliberately does not reproduce:
 *
 *     primitive types
 *     generic types
 *     arrays
 *     tuples
 *     maps
 *     quantum types
 *     tensor types
 *     hardware types
 *     resource types
 *     function types
 *     dependent types
 *     linear/affine types
 */
variableTypeAnnotation
    : COLON typeExpression
    ;


/*
 * ============================================================================
 * OPTIONAL INITIALIZER
 * ============================================================================
 *
 * The expression grammar owns expression syntax.
 *
 * This rule owns only the declaration-level assignment boundary.
 *
 * Examples:
 *
 *     = 1
 *     = f()
 *     = tensor
 *     = quantum_state
 *     = accelerator.compute(data)
 *     = if condition { a } else { b }
 */
variableInitializer
    : ASSIGN expression
    ;


/*
 * ============================================================================
 * DECLARATION TERMINATION
 * ============================================================================
 *
 * Explicit semicolon termination is retained.
 *
 * Automatic semicolon insertion does not belong to this grammar.
 *
 * `declarationTerminator` is kept as a named integration boundary so the
 * surrounding grammar can evolve its statement/declaration termination policy
 * without duplicating variable syntax.
 */
declarationTerminator
    : SEMI
    ;


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM:
 *
 *     ZamaniLexer
 *
 * Required lexical tokens:
 *
 *     LET
 *     VAR
 *     COLON
 *     ASSIGN
 *     SEMI
 *     IDENTIFIER
 *
 * Required parser rules:
 *
 *     identifier
 *     typeExpression
 *     expression
 *
 * DOWNSTREAM:
 *
 *     declarations/declarations.g4
 *     frontend AST
 *     semantic analysis
 *     compiler
 *
 * AST CONTRACT:
 *
 *     VariableDeclaration {
 *         binding_kind,
 *         name,
 *         type_annotation?,
 *         initializer?,
 *         source_span
 *     }
 *
 * `binding_kind` maps to the existing frontend representation rather than
 * introducing another declaration enum in the grammar.
 *
 * The repository already contains a VariableDeclaration AST representation
 * and VariableBindingKind, so this grammar must feed that existing model
 * rather than create a competing AST. 
 *
 * SEMANTIC CONTRACT:
 *
 *     - scope
 *     - symbol identity
 *     - shadowing
 *     - mutability
 *     - definite initialization
 *     - type inference
 *     - type compatibility
 *     - ownership
 *     - borrowing
 *     - lifetime
 *     - effects
 *     - capabilities
 *     - resource requirements
 *
 * belong downstream.
 *
 * IR CONTRACT:
 *
 * This grammar produces no IR.
 *
 * Variable declarations may eventually participate in:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware IR
 *     distributed IR
 *     accelerator IR
 *
 * depending on semantic use.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 */


/*
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is intentionally:
 *
 *     no MAX_VARIABLES
 *     no MAX_BINDINGS
 *     no MAX_LOCALS
 *     no MAX_SCOPE_DEPTH
 *     no MAX_VARIABLE_SIZE
 *     no MAX_TYPE_PARAMETER_COUNT
 *     no MAX_MEMORY_SIZE
 *     no MAX_CORES
 *     no MAX_THREADS
 *     no MAX_GPUS
 *     no MAX_FPGAS
 *     no MAX_QUBITS
 *     no MAX_DEVICES
 *     no MAX_NODES
 *
 * Repetition limits imposed by the grammar are therefore absent.
 *
 * Practical limits may be imposed by:
 *
 *     parser implementation
 *     compiler resource policy
 *     available memory
 *     operating system
 *     target capabilities
 *     deployment environment
 *
 * Those are implementation/resource constraints, not language semantics.
 */


/*
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     physical CPU identifiers
 *     physical GPU identifiers
 *     FPGA identifiers
 *     QPU identifiers
 *     physical qubit identifiers
 *     fixed register counts
 *     fixed memory sizes
 *     fixed topology sizes
 *     fixed accelerator counts
 *     fixed node counts
 *     fixed tensor dimensions
 *
 * A variable is a logical program binding.
 *
 * Its eventual storage or placement is determined downstream.
 */


/*
 * ============================================================================
 * DOMAIN INDEPENDENCE
 * ============================================================================
 *
 * The same declaration syntax is valid regardless of eventual realization:
 *
 *     var x = classical_value;
 *     var q = quantum_value;
 *     var t = tensor_value;
 *     var h = hardware_value;
 *     var d = distributed_value;
 *
 * The grammar does not need:
 *
 *     classicalVariableDeclaration
 *     quantumVariableDeclaration
 *     gpuVariableDeclaration
 *     fpgaVariableDeclaration
 *     qpuVariableDeclaration
 *
 * Domain semantics are provided by the type/expression/semantic systems.
 */


/*
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * No:
 *
 *     semantic predicates
 *     actions
 *     runtime conditions
 *     filesystem access
 *     network access
 *     target-dependent branches
 *     random behavior
 *
 * are permitted.
 *
 * Identical token streams and identical imported grammar versions must produce
 * the same parse structure.
 */


/*
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntactically invalid:
 *
 *     var;
 *     var = 1;
 *     var x:
 *     var x =;
 *     let x = ;
 *     let : Integer = 1;
 *
 * Semantically invalid examples that belong downstream:
 *
 *     var x: Integer = "text";
 *     let x: UnknownType = value;
 *     var x: IncompatibleType = value;
 *
 * This grammar must not perform type checking.
 */


/*
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Variable declarations cannot themselves:
 *
 *     access files
 *     access networks
 *     access devices
 *     spawn processes
 *     load arbitrary libraries
 *     retrieve secrets
 *     bypass capability checks
 *     perform privileged operations
 *
 * Such behavior must be represented through the language's explicit effect and
 * capability mechanisms and checked downstream.
 */


/*
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing source forms preserved:
 *
 *     let x = expression;
 *     let x: Type = expression;
 *     var x = expression;
 *     var x: Type = expression;
 *     var x: Type;
 *
 * `const` remains outside this grammar and belongs to:
 *
 *     declarations/constants.g4
 *
 * This prevents overlapping ownership.
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when all of the following are true:
 *
 *     [x] owns only variable declaration syntax
 *     [x] uses canonical lexer vocabulary
 *     [x] does not define lexer rules
 *     [x] does not duplicate identifier syntax
 *     [x] does not duplicate type syntax
 *     [x] does not duplicate expression syntax
 *     [x] does not duplicate constant syntax
 *     [x] has no target-specific syntax
 *     [x] has no hardware limits
 *     [x] has no quantum limits
 *     [x] has no embedded Rust
 *     [x] has no unsafe
 *     [x] has no semantic predicates
 *     [x] preserves source spans through parser contexts
 *     [x] maps to existing VariableDeclaration AST
 *     [x] preserves VariableBindingKind
 *     [x] is deterministic
 *     [x] is independently testable
 *     [x] integrates with declarations/declarations.g4
 *     [x] integrates with the canonical type grammar
 *     [x] integrates with the canonical expression grammar
 *     [x] is suitable for arbitrary program scale subject to resources
 *
 * Required external validation:
 *
 *     - lexer conformance
 *     - parser generation
 *     - positive syntax tests
 *     - negative syntax tests
 *     - boundary tests
 *     - scalability tests
 *     - compatibility tests
 *     - AST lowering tests
 *     - semantic integration tests
 */