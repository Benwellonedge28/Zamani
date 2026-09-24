/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/linear-algebra.g4
 *
 * Status:
 *     Production-ready classical linear-algebra domain composition boundary.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No embedded Rust actions
 *     No semantic predicates
 *     No unsafe implementation
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the SOURCE-SYNTAX INTEGRATION BOUNDARY for linear algebra
 * in Zamani.
 *
 * Linear algebra is intentionally a semantic domain of the single Zamani
 * language. It is NOT a second language and does NOT introduce:
 *
 *     - a second expression grammar;
 *     - a second type system;
 *     - a second matrix grammar;
 *     - a second vector grammar;
 *     - a second tensor grammar;
 *     - a second operation-call grammar;
 *     - a linear-algebra-specific AST;
 *     - a linear-algebra-specific IR.
 *
 * The canonical pipeline remains:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     classical semantics          resource/capability
 *          |                      requirements
 *          |                              |
 *          +--------------+---------------+
 *                         |
 *                         v
 *                 canonical semantic model
 *                         |
 *                         v
 *                   classical IR
 *                         |
 *                         v
 *              optimization / lowering
 *                         |
 *              scheduling / placement
 *                         |
 *                 target realization
 *
 * Quantum computation remains integrated through the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This file MUST NOT introduce a second quantum IR merely because a linear
 * algebra operation happens to be used by a quantum program.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Linear algebra is important enough to have an explicit grammar-domain
 * contract, but most linear-algebra operations do NOT require special syntax.
 *
 * Examples:
 *
 *     transpose(A)
 *     inverse(A)
 *     solve(A, b)
 *     determinant(A)
 *     eig(A)
 *     svd(A)
 *     qr(A)
 *     cholesky(A)
 *     norm(A)
 *     dot(a, b)
 *     cross(a, b)
 *     matmul(A, B)
 *
 * are ordinary Zamani expressions.
 *
 * Their linear-algebra meaning is established by semantic analysis from:
 *
 *     - operation name;
 *     - namespace;
 *     - operand types;
 *     - result types;
 *     - shape information;
 *     - generic parameters;
 *     - effects;
 *     - capabilities;
 *     - resource requirements;
 *     - numerical semantics;
 *     - active libraries/dialects;
 *     - program constraints.
 *
 * This file therefore deliberately avoids a closed list such as:
 *
 *     linearAlgebraOperation
 *         : TRANSPOSE
 *         | INVERSE
 *         | SVD
 *         | QR
 *         | LU
 *         | ...
 *
 * Such an enumeration would make the language non-extensible and would force
 * every new mathematical algorithm, library operation, vendor operation, or
 * future computational abstraction to become a grammar modification.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - linear-algebra domain composition;
 *     - the public linear-algebra semantic boundary;
 *     - linear-algebra expression classification hooks;
 *     - linear-algebra type classification hooks;
 *     - linear-algebra region boundaries where explicitly required by the
 *       surrounding grammar;
 *     - linear-algebra resource/capability integration hooks;
 *     - linear-algebra portability/scalability contract;
 *     - linear-algebra conformance boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - keywords;
 *     - numeric literals;
 *     - operators;
 *     - expression precedence;
 *     - generic calls;
 *     - member access;
 *     - indexing;
 *     - slicing;
 *     - ranges;
 *     - assignment;
 *     - declarations;
 *     - statements;
 *     - general type syntax;
 *     - matrix literal syntax;
 *     - vector literal syntax;
 *     - tensor literal syntax;
 *     - matrix type syntax;
 *     - vector type syntax;
 *     - tensor type syntax;
 *     - numerical algorithms;
 *     - numerical precision;
 *     - numerical stability;
 *     - shape inference;
 *     - shape validation;
 *     - dimensional arithmetic;
 *     - storage layout;
 *     - sparse/dense representation;
 *     - memory allocation;
 *     - vectorization;
 *     - SIMD realization;
 *     - GPU realization;
 *     - FPGA realization;
 *     - accelerator selection;
 *     - distributed placement;
 *     - scheduling;
 *     - routing;
 *     - classical IR construction;
 *     - quantum::ir construction;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * General source expression syntax belongs to:
 *
 *     grammar/expressions/expressions.g4
 *
 * General type syntax belongs to:
 *
 *     grammar/types/types.g4
 *
 * Classical domain composition belongs to:
 *
 *     grammar/classical/classical.g4
 *
 * Matrix-specific source structure belongs to:
 *
 *     grammar/classical/matrix.g4
 *
 * Vector-specific source structure belongs to:
 *
 *     grammar/classical/vector.g4
 *
 * Tensor-specific source structure belongs to:
 *
 *     grammar/classical/tensor.g4
 *
 * Scalar/numeric syntax belongs to the corresponding canonical classical
 * grammars.
 *
 * The universal parser composition boundary remains:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * and the visible grammar composition root remains:
 *
 *     grammar/Zamani.g4
 *
 * This file MUST NOT redefine any of those authorities.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * It defines NO lexer rules.
 *
 * The parser consumes the canonical Zamani lexer vocabulary:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Linear-algebra names such as:
 *
 *     Matrix
 *     Vector
 *     Tensor
 *     transpose
 *     inverse
 *     solve
 *     svd
 *     qr
 *     lu
 *     eig
 *     norm
 *     matmul
 *
 * are NOT made special lexer tokens by this file.
 *
 * They remain ordinary identifiers unless the canonical language specification
 * explicitly establishes a separate lexical requirement elsewhere.
 *
 * This preserves open-world extensibility.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file introduces NO Rust AST type.
 *
 * Linear-algebra source syntax lowers into the existing domain-neutral
 * frontend AST.
 *
 * Conceptual mappings:
 *
 *     linearAlgebraExpression
 *         -> generic Expression
 *
 *     linearAlgebraType
 *         -> generic TypeExpr
 *
 *     linearAlgebraRegion
 *         -> existing block/statement representation
 *
 *     linearAlgebraRequirement
 *         -> existing resource/capability/constraint representation
 *
 * The exact Rust representation belongs to the existing frontend AST and
 * semantic-analysis layers.
 *
 * This grammar MUST NOT create:
 *
 *     LinearAlgebraExpr
 *     MatrixExpr
 *     VectorExpr
 *     LinearAlgebraOperation
 *
 * as competing AST representations merely because a value has linear-algebra
 * semantics.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "Is this structurally valid Zamani syntax at the
 *      linear-algebra domain boundary?"
 *
 * Semantic analysis answers:
 *
 *     "Does this expression represent linear algebra, and what
 *      mathematical operation/type/resource semantics does it have?"
 *
 * Semantic analysis owns:
 *
 *     - linear-algebra operation resolution;
 *     - overload resolution;
 *     - generic resolution;
 *     - operand compatibility;
 *     - result compatibility;
 *     - matrix/vector/tensor classification;
 *     - shape inference;
 *     - shape compatibility;
 *     - dimension arithmetic;
 *     - rank reasoning;
 *     - scalar promotion;
 *     - numerical precision;
 *     - numerical domain;
 *     - sparsity;
 *     - storage semantics;
 *     - layout;
 *     - algorithm selection;
 *     - numerical-stability requirements;
 *     - conditioning analysis;
 *     - capability requirements;
 *     - resource requirements;
 *     - effects;
 *     - target-independent lowering decisions.
 *
 * ============================================================================
 * OPEN-WORLD OPERATION MODEL
 * ============================================================================
 *
 * Linear algebra MUST remain open-world.
 *
 * The grammar MUST NOT enumerate a finite set of mathematical algorithms.
 *
 * Therefore all of the following can be represented without grammar changes:
 *
 *     transpose(A)
 *     inverse(A)
 *     determinant(A)
 *     solve(A, b)
 *     least_squares(A, b)
 *     lu(A)
 *     qr(A)
 *     svd(A)
 *     eig(A)
 *     eigensystem(A)
 *     cholesky(A)
 *     schur(A)
 *     hessenberg(A)
 *     polar(A)
 *     expm(A)
 *     logm(A)
 *     sqrtm(A)
 *     norm(A)
 *     dot(a, b)
 *     cross(a, b)
 *     outer(a, b)
 *     matmul(A, B)
 *     kron(A, B)
 *     reshape(A, shape)
 *     factorize(A)
 *     custom_algorithm(A, B)
 *
 * as ordinary expressions.
 *
 * Qualified operations are equally valid:
 *
 *     math::transpose(A)
 *     linear_algebra::solve(A, b)
 *     numerical::svd(A)
 *     scientific::eigensystem(A)
 *     vendor::domain::operation(A)
 *
 * Namespace depth is not bounded by this grammar.
 *
 * ============================================================================
 * IMPORTANT: NO SECOND CALL GRAMMAR
 * ============================================================================
 *
 * A previous architectural failure mode is defining:
 *
 *     linearAlgebraOperation
 *         : operationName '(' argumentList ')'
 *
 * when the universal expression grammar already defines exactly that syntax.
 *
 * This file deliberately does NOT reproduce generic call syntax.
 *
 * For example:
 *
 *     solve(A, b)
 *
 * is parsed by the canonical expression grammar.
 *
 * The semantic analyzer then determines whether:
 *
 *     solve
 *
 * denotes a linear-algebra operation.
 *
 * This avoids:
 *
 *     - duplicate call syntax;
 *     - parser ambiguity;
 *     - divergent argument-list behavior;
 *     - divergent source-span behavior;
 *     - divergent generic-call behavior;
 *     - divergent named-argument behavior;
 *     - divergent trailing-comma behavior;
 *     - competing AST nodes.
 *
 * ============================================================================
 * MATRIX INTEGRATION
 * ============================================================================
 *
 * Matrix source syntax remains owned by:
 *
 *     grammar/classical/matrix.g4
 *
 * This file does NOT redefine:
 *
 *     matrixLiteral
 *     matrixRow
 *     matrixElementList
 *     matrix indexing
 *
 * Linear-algebra semantics may operate on matrix values represented by the
 * canonical type/value system.
 *
 * Examples:
 *
 *     Matrix<f64>
 *     Matrix<f32, Rows, Cols>
 *     math::Matrix<f64, Rows, Cols>
 *
 * are interpreted by the canonical type system and semantic analysis.
 *
 * This file does not create a special Matrix token.
 *
 * ============================================================================
 * VECTOR INTEGRATION
 * ============================================================================
 *
 * Vector source syntax remains owned by:
 *
 *     grammar/classical/vector.g4
 *
 * This file does NOT redefine vector literals or comprehensions.
 *
 * Linear-algebra operations may consume vector values through ordinary
 * expressions:
 *
 *     dot(a, b)
 *     norm(v)
 *     normalize(v)
 *     solve(A, b)
 *
 * Whether a value is a vector is determined by type/semantic analysis.
 *
 * ============================================================================
 * TENSOR INTEGRATION
 * ============================================================================
 *
 * Tensor syntax remains owned by:
 *
 *     grammar/classical/tensor.g4
 *
 * Linear algebra may be generalized to tensor-valued computation where the
 * semantic model supports it.
 *
 * This file imposes no tensor-rank or dimension limit.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * Source type syntax belongs to:
 *
 *     grammar/types/types.g4
 *
 * Classical type specialization belongs to the existing classical type
 * grammar.
 *
 * This file therefore provides only a semantic classification boundary:
 *
 *     linearAlgebraType
 *         : typeExpression
 *         ;
 *
 * This is intentionally not a second type system.
 *
 * Semantic analysis may recognize types such as:
 *
 *     Matrix<T>
 *     Matrix<T, Rows, Cols>
 *     Vector<T>
 *     Tensor<T, Shape>
 *     SparseMatrix<T>
 *     DistributedMatrix<T>
 *     BlockMatrix<T>
 *     LinearOperator<T>
 *     quantum::Operator<T>
 *
 * without requiring every future type to become a grammar alternative.
 *
 * ============================================================================
 * SHAPE MODEL
 * ============================================================================
 *
 * Shape is semantic information.
 *
 * A shape may contain:
 *
 *     literals
 *     identifiers
 *     symbolic values
 *     compile-time expressions
 *     dependent values
 *     generic parameters
 *     runtime-known values where permitted
 *     values derived from other computations where permitted
 *
 * Examples:
 *
 *     Matrix<f64, N, M>
 *     Matrix<f64, Rows, Cols>
 *     Tensor<f64, [N, M, K]>
 *
 * This grammar imposes no maximum for:
 *
 *     rows
 *     columns
 *     dimensions
 *     rank
 *     elements
 *     operands
 *     operation count
 *     expression depth
 *
 * Any implementation/resource limits belong outside language semantics.
 *
 * ============================================================================
 * DIMENSION COMPATIBILITY
 * ============================================================================
 *
 * Dimension compatibility is NOT a parsing concern.
 *
 * Examples such as:
 *
 *     A + B
 *     A * B
 *     matmul(A, B)
 *     solve(A, b)
 *
 * may require semantic constraints.
 *
 * Examples:
 *
 *     A : Matrix<T, M, N>
 *     B : Matrix<T, N, K>
 *
 * permit semantic reasoning about:
 *
 *     A * B
 *
 * without the parser needing to understand what M, N, or K mean.
 *
 * The semantic layer owns:
 *
 *     M == N
 *     N == K
 *     shape compatibility
 *     broadcast compatibility
 *     rank compatibility
 *     result-shape inference
 *
 * ============================================================================
 * OPERATOR INTEGRATION
 * ============================================================================
 *
 * Linear algebra uses the canonical expression/operator grammar.
 *
 * Examples:
 *
 *     A + B
 *     A - B
 *     A * B
 *     A / B
 *     A @ B
 *
 * if those operators are part of the canonical expression vocabulary.
 *
 * This file MUST NOT redefine:
 *
 *     precedence
 *     associativity
 *     additive expressions
 *     multiplicative expressions
 *     bitwise expressions
 *     comparison expressions
 *     assignment
 *
 * The meaning of an operator is determined semantically from operand types
 * and applicable operator implementations.
 *
 * This is particularly important for:
 *
 *     *
 *     @
 *
 * because whether multiplication means scalar multiplication, matrix
 * multiplication, tensor contraction, or another operation is not a lexical
 * question.
 *
 * ============================================================================
 * INDEXING INTEGRATION
 * ============================================================================
 *
 * Indexing and slicing remain owned by the universal expression grammar.
 *
 * Examples:
 *
 *     A[i]
 *     A[i, j]
 *     A[i, j, k]
 *     A[start:end]
 *     A[start:end:step]
 *
 * are parsed by the canonical indexing grammar.
 *
 * This file does not introduce:
 *
 *     matrixIndex
 *     vectorIndex
 *     tensorIndex
 *
 * as competing syntax.
 *
 * Semantic analysis determines what the indexed value means.
 *
 * ============================================================================
 * LITERAL INTEGRATION
 * ============================================================================
 *
 * Linear-algebra values may originate from ordinary Zamani expressions.
 *
 * Examples:
 *
 *     [[1, 2], [3, 4]]
 *     [1, 2, 3]
 *     tensor(...)
 *     zeros(...)
 *     identity(...)
 *
 * Matrix/vector structural literals remain owned by their respective
 * classical grammar components.
 *
 * Function-style constructors such as:
 *
 *     zeros(shape)
 *     identity(n)
 *     diagonal(values)
 *
 * remain ordinary expressions.
 *
 * Their semantics are determined downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY MODEL
 * ============================================================================
 *
 * Linear algebra MUST NOT encode target capacities into syntax.
 *
 * The following are prohibited as universal language limits:
 *
 *     MAX_MATRIX_ROWS
 *     MAX_MATRIX_COLUMNS
 *     MAX_MATRIX_ELEMENTS
 *     MAX_VECTOR_LENGTH
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *
 * A program may express semantic requirements through the repository's
 * canonical resource/capability system.
 *
 * Examples:
 *
 *     requires capability("matrix.compute")
 *     requires capability("linear_algebra.solve")
 *     requires capability("tensor.compute")
 *     requires capability("distributed.linear_algebra")
 *
 * Resource requirements may describe actual program needs, for example:
 *
 *     requires memory >= required_memory
 *
 * or a domain-specific resource contract.
 *
 * These are NOT parser-level hardware limits.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE SEPARATION
 * ============================================================================
 *
 * The semantic architecture must distinguish:
 *
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *     implementation decision
 *
 * For example:
 *
 *     requires capability("linear_algebra.solve")
 *
 * means:
 *
 *     the execution environment must provide the requested capability.
 *
 * It does NOT mean:
 *
 *     use CPU 0
 *     use GPU 0
 *     use accelerator 3
 *     use SIMD width 16
 *     use memory bank 2
 *
 * Target realization belongs downstream.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The same linear-algebra source must be expressible independently of:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     accelerator count
 *     node count
 *     memory capacity
 *     SIMD width
 *     register width
 *     cache capacity
 *     NUMA topology
 *     network topology
 *     device identifier
 *     vendor library
 *     physical machine size
 *
 * A source program expresses WHAT mathematical computation is required.
 *
 * The compiler and runtime determine HOW and WHERE it is realized.
 *
 * Potential lowerings include:
 *
 *     scalar execution
 *     vector/SIMD execution
 *     multicore execution
 *     GPU execution
 *     FPGA execution
 *     ASIC execution
 *     accelerator execution
 *     distributed execution
 *     streaming execution
 *     heterogeneous execution
 *     future execution models
 *
 * subject to:
 *
 *     semantic correctness
 *     resource availability
 *     capability availability
 *     target constraints
 *     numerical requirements
 *     deployment policy.
 *
 * ============================================================================
 * "INFINITY" / SCALABILITY CONTRACT
 * ============================================================================
 *
 * "Scalable to infinity" is interpreted as:
 *
 *     no artificial language-level finite hardware cardinality.
 *
 * It does NOT claim:
 *
 *     infinite physical memory;
 *     infinite execution time;
 *     infinite bandwidth;
 *     infinite numerical precision;
 *     infinite hardware.
 *
 * This grammar therefore contains no finite machine-capacity constants.
 *
 * Structural repetition remains unbounded by the grammar:
 *
 *     *
 *     +
 *
 * and recursive composition are used where appropriate.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing through this grammar MUST depend only on:
 *
 *     - source token sequence;
 *     - canonical grammar composition;
 *     - selected language version.
 *
 * It MUST NOT depend on:
 *
 *     - CPU availability;
 *     - GPU availability;
 *     - QPU availability;
 *     - FPGA availability;
 *     - memory availability;
 *     - filesystem state;
 *     - network state;
 *     - environment variables;
 *     - wall-clock time;
 *     - randomness;
 *     - hardware discovery.
 *
 * Hardware/resource discovery is downstream.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar is side-effect free.
 *
 * It MUST NOT:
 *
 *     - execute code;
 *     - execute shell commands;
 *     - access files;
 *     - access networks;
 *     - inspect hardware;
 *     - access credentials;
 *     - invoke compiler backends;
 *     - invoke runtime services.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax diagnostics remain owned by the canonical parser/diagnostic system.
 *
 * Semantic diagnostics may later report:
 *
 *     incompatible shapes;
 *     invalid dimensions;
 *     unsupported operation;
 *     missing capability;
 *     insufficient resource;
 *     invalid numerical domain;
 *     unsupported precision;
 *     invalid operation overload;
 *
 * Those diagnostics MUST NOT be encoded as parser-only restrictions.
 *
 * In particular, the parser must not reject a mathematically valid operation
 * merely because the current compiler lacks an implementation for it.
 *
 * Unsupported implementation is distinct from invalid language syntax.
 *
 * ============================================================================
 * DOMAIN EXTENSIBILITY
 * ============================================================================
 *
 * New linear-algebra algorithms must NOT require changes to this file merely
 * because a new operation name is introduced.
 *
 * For example, a future implementation can introduce:
 *
 *     randomized_svd(A)
 *     quantum_linear_solver(A, b)
 *     tensor_network_solve(A, b)
 *     future_solver(A, b)
 *
 * without modifying the grammar provided the constructs are representable by
 * ordinary Zamani expressions and the semantic/operation registry supports
 * them.
 *
 * ============================================================================
 * DIALECT INTEGRATION
 * ============================================================================
 *
 * Vendor/framework-specific linear-algebra operations may be introduced by
 * dialects.
 *
 * Example:
 *
 *     vendor::operation(A, B)
 *
 * Dialects must NOT modify the base linear-algebra grammar merely to add
 * operation names.
 *
 * A dialect may provide:
 *
 *     operation declarations;
 *     semantic signatures;
 *     type constraints;
 *     capability requirements;
 *     resource requirements;
 *     lowering rules;
 *     compatibility metadata.
 *
 * The base grammar remains open.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Linear algebra is frequently used by quantum computation, but this file
 * does not own quantum syntax.
 *
 * Quantum source remains governed by:
 *
 *     grammar/quantum/
 *
 * and its canonical semantic boundary remains:
 *
 *     quantum::ir
 *
 * A linear-algebra expression may therefore appear in a hybrid program.
 *
 * Example:
 *
 *     classical matrix construction
 *          |
 *          v
 *     numerical computation
 *          |
 *          v
 *     quantum parameter generation
 *          |
 *          v
 *     quantum operation
 *
 * The semantic analyzer determines the domain transition.
 *
 * This file must not create a quantum-specific linear algebra IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Linear-algebra source may ultimately be lowered to:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *
 * or another hardware realization.
 *
 * This grammar does not select any of them.
 *
 * Hardware intent belongs to:
 *
 *     grammar/hardware/
 *     grammar/resources/
 *     grammar/hdl/
 *
 * Scheduling, placement, tiling, memory mapping, vectorization, and hardware
 * lowering are downstream responsibilities.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed linear algebra may use the same source semantics.
 *
 * The program need not specify:
 *
 *     node count;
 *     node identifiers;
 *     fixed partition count;
 *     fixed communication topology.
 *
 * The distributed compiler/runtime may derive a realization from:
 *
 *     data shape;
 *     communication requirements;
 *     capabilities;
 *     available resources;
 *     placement policy.
 *
 * ============================================================================
 * NUMERICAL SEMANTICS
 * ============================================================================
 *
 * Numerical meaning is deliberately outside parser ownership.
 *
 * Semantic/compiler layers may need to reason about:
 *
 *     precision;
 *     rounding;
 *     overflow;
 *     underflow;
 *     conditioning;
 *     stability;
 *     convergence;
 *     determinism;
 *     reproducibility;
 *     exact arithmetic;
 *     approximate arithmetic;
 *     symbolic arithmetic;
 *     interval arithmetic;
 *     automatic differentiation;
 *     probabilistic/numerical computation.
 *
 * This file only provides the syntax/domain boundary.
 *
 * ============================================================================
 * SPARSE / DENSE / STRUCTURED REPRESENTATION
 * ============================================================================
 *
 * The grammar must not assume that every matrix is dense.
 *
 * Semantic representations may distinguish:
 *
 *     dense;
 *     sparse;
 *     diagonal;
 *     triangular;
 *     banded;
 *     block;
 *     low-rank;
 *     distributed;
 *     streamed;
 *     lazy;
 *     symbolic;
 *     implicit;
 *     operator-valued.
 *
 * Such distinctions are semantic/type/optimization concerns.
 *
 * ============================================================================
 * ALGORITHM SELECTION
 * ============================================================================
 *
 * Source syntax does not mandate a particular algorithm.
 *
 * For example:
 *
 *     solve(A, b)
 *
 * does not necessarily mean:
 *
 *     LU
 *
 * or:
 *
 *     QR
 *
 * or:
 *
 *     iterative solve.
 *
 * The compiler may select an implementation according to:
 *
 *     correctness;
 *     numerical requirements;
 *     matrix properties;
 *     capabilities;
 *     available resources;
 *     target architecture;
 *     performance constraints;
 *     reproducibility requirements.
 *
 * ============================================================================
 * OPERATION REGISTRY CONTRACT
 * ============================================================================
 *
 * The semantic operation registry, not this grammar, is responsible for
 * answering:
 *
 *     Does this operation exist?
 *
 *     What signature does it have?
 *
 *     What types does it accept?
 *
 *     What shapes are required?
 *
 *     What type does it return?
 *
 *     What capabilities does it require?
 *
 *     What resources does it require?
 *
 *     What effects does it have?
 *
 *     What lowering options exist?
 *
 * The grammar must remain unchanged when an operation registry gains a new
 * operation whose syntax is already representable by ordinary Zamani syntax.
 *
 * ============================================================================
 * SOURCE SPAN CONTRACT
 * ============================================================================
 *
 * All syntax represented through this boundary must retain source locations
 * through the canonical AST.
 *
 * This includes:
 *
 *     operation names;
 *     namespace components;
 *     operands;
 *     type expressions;
 *     shape expressions;
 *     requirements;
 *     enclosing regions.
 *
 * This allows semantic diagnostics to identify the exact source construct.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file must remain compatible with:
 *
 *     grammar/DESIGN.md
 *     grammar/README.md
 *     grammar/specification/
 *     grammar/spec/
 *     grammar/classical/classical.g4
 *     grammar/classical/matrix.g4
 *     grammar/classical/vector.g4
 *     grammar/classical/tensor.g4
 *     grammar/expressions/expressions.g4
 *     grammar/types/types.g4
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/antlr/ZamaniParser.g4
 *
 * It must not require a new lexer token merely to add a new mathematical
 * operation.
 *
 * ============================================================================
 * VALIDATION CONTRACT
 * ============================================================================
 *
 * This file must be validated for:
 *
 *     - ANTLR grammar validity;
 *     - unreachable rules;
 *     - ambiguity;
 *     - token existence;
 *     - import correctness;
 *     - source-span preservation;
 *     - AST mapping;
 *     - semantic coverage;
 *     - portability;
 *     - scalability;
 *     - hard-coded capacity detection.
 *
 * The hard-coding audit must reject language-level limits such as:
 *
 *     MAX_MATRIX_ROWS
 *     MAX_MATRIX_COLUMNS
 *     MAX_MATRIX_ELEMENTS
 *     MAX_VECTOR_LENGTH
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * when they are introduced as universal language constraints.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The corresponding conformance tests belong under:
 *
 *     grammar/tests/classical/
 *     grammar/tests/classical/linear-algebra/
 *
 * Recommended categories:
 *
 *     positive/
 *     negative/
 *     boundary/
 *     scalability/
 *     portability/
 *     determinism/
 *     compatibility/
 *
 * Positive examples should include:
 *
 *     A + B
 *     A - B
 *     A * B
 *     transpose(A)
 *     solve(A, b)
 *     svd(A)
 *     qr(A)
 *     math::solve(A, b)
 *     linear_algebra::svd(A)
 *     vendor::domain::operation(A, B)
 *     Matrix<T>
 *     Matrix<T, N, M>
 *     Vector<T>
 *     Tensor<T, Shape>
 *
 * Boundary examples should include:
 *
 *     empty structures where the owning literal grammar permits them;
 *     symbolic dimensions;
 *     nested qualified names;
 *     generic operations;
 *     user-defined operations;
 *     mixed scalar/vector/matrix operands;
 *     expressions used as dimensions.
 *
 * Negative tests should test malformed syntax rather than implementation
 * availability.
 *
 * For example:
 *
 *     solve(A, )
 *
 * may be syntactically invalid depending on the canonical call grammar.
 *
 * But:
 *
 *     future_linear_solver(A, b)
 *
 * MUST NOT be rejected merely because no implementation currently exists.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The grammar must accept structurally equivalent programs regardless of
 * machine capacity.
 *
 * Tests must NOT establish artificial maxima such as:
 *
 *     1024 rows;
 *     4096 columns;
 *     32 dimensions;
 *     16 operands;
 *     8 matrices.
 *
 * Instead, tests should verify that repetition is structural and that the
 * grammar contains no machine-sized ceiling.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar is independent of the Rust implementation language except for
 * the repository's generated-parser integration contract.
 *
 * The compiler/frontend implementation must target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * This file contains no embedded Rust and therefore requires no unsafe code.
 *
 * ============================================================================
 * PUBLIC DOMAIN BOUNDARY
 * ============================================================================
 *
 * The public entry point is intentionally a DOMAIN BOUNDARY rather than a
 * second mathematical expression language.
 *
 * Consumers that specifically require a linear-algebra expression can use:
 *
 *     linearAlgebraExpression
 *
 * Consumers that require a linear-algebra type can use:
 *
 *     linearAlgebraType
 *
 * Consumers requiring a linear-algebra computation region can use:
 *
 *     linearAlgebraRegion
 *
 * The canonical whole-program parser should continue to parse ordinary
 * expressions through:
 *
 *     expression
 *
 * and classify them semantically.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This grammar imports only canonical expression/type infrastructure.
 *
 * It deliberately does NOT import matrix.g4, vector.g4, or tensor.g4 because
 * those files own their own source structures and importing them here would
 * encourage competing parse paths for ordinary literals and operations.
 *
 * Cross-domain integration is semantic rather than syntactic.
 * ============================================================================
 */

parser grammar LinearAlgebra;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions, Types;


/*
 * ============================================================================
 * 1. PUBLIC LINEAR-ALGEBRA DOMAIN ENTRY
 * ============================================================================
 *
 * This is a semantic-domain entry point.
 *
 * It is intentionally broad because the ordinary Zamani expression/type
 * systems already own the concrete syntax.
 *
 * The canonical root parser should not add this rule as a competing
 * alternative to `expression` for ordinary expressions.
 *
 * Instead:
 *
 *     expression
 *         |
 *         v
 *     semantic classification
 *         |
 *         v
 *     linear algebra
 *
 * Domain-specific parser consumers may use this entry point explicitly.
 * ============================================================================
 */

linearAlgebraConstruct
    : linearAlgebraExpression
    | linearAlgebraType
    | linearAlgebraRegion
    ;


/*
 * ============================================================================
 * 2. LINEAR-ALGEBRA EXPRESSION
 * ============================================================================
 *
 * Linear-algebra expressions are ordinary Zamani expressions.
 *
 * This rule deliberately does not introduce:
 *
 *     matrixOperationExpression
 *     vectorOperationExpression
 *     tensorOperationExpression
 *     linearAlgebraCall
 *
 * because those would duplicate the universal expression grammar.
 *
 * ============================================================================
 */

linearAlgebraExpression
    : expression
    ;


/*
 * ============================================================================
 * 3. LINEAR-ALGEBRA TYPE
 * ============================================================================
 *
 * Type syntax is owned by the canonical type grammar.
 *
 * Semantic analysis determines whether a type is a linear-algebra type.
 * ============================================================================
 */

linearAlgebraType
    : typeExpression
    ;


/*
 * ============================================================================
 * 4. LINEAR-ALGEBRA COMPUTATION REGION
 * ============================================================================
 *
 * A domain-scoped region consists of canonical Zamani statements.
 *
 * This rule is an integration boundary, not a second statement grammar.
 *
 * It deliberately does not redefine:
 *
 *     if
 *     while
 *     for
 *     match
 *     return
 *     declaration
 *     assignment
 *     expression statement
 *
 * syntax.
 * ============================================================================
 */

linearAlgebraRegion
    : LBRACE
      linearAlgebraStatementSequence
      RBRACE
    ;


/*
 * ============================================================================
 * 5. LINEAR-ALGEBRA STATEMENT SEQUENCE
 * ============================================================================
 *
 * There is no fixed statement count.
 * ============================================================================
 */

linearAlgebraStatementSequence
    : statement*
    ;


/*
 * ============================================================================
 * 6. LINEAR-ALGEBRA VALUE BOUNDARY
 * ============================================================================
 *
 * A value consumed by a linear-algebra semantic operation is an ordinary
 * Zamani expression.
 *
 * Examples:
 *
 *     A
 *     B
 *     A[i, j]
 *     f(x)
 *     reshape(A, shape)
 *     classical_value
 * ============================================================================
 */

linearAlgebraValue
    : expression
    ;


/*
 * ============================================================================
 * 7. LINEAR-ALGEBRA OPERAND BOUNDARY
 * ============================================================================
 *
 * This named rule exists to provide a stable domain contract for semantic
 * consumers without introducing another argument grammar.
 * ============================================================================
 */

linearAlgebraOperand
    : expression
    ;


/*
 * ============================================================================
 * 8. LINEAR-ALGEBRA SCALAR BOUNDARY
 * ============================================================================
 *
 * Whether an expression is scalar is a semantic/type question.
 * ============================================================================
 */

linearAlgebraScalar
    : expression
    ;


/*
 * ============================================================================
 * 9. LINEAR-ALGEBRA VECTOR BOUNDARY
 * ============================================================================
 *
 * Vector syntax itself remains owned by vector.g4.
 *
 * This rule is a semantic classification hook.
 * ============================================================================
 */

linearAlgebraVector
    : expression
    ;


/*
 * ============================================================================
 * 10. LINEAR-ALGEBRA MATRIX BOUNDARY
 * ============================================================================
 *
 * Matrix syntax itself remains owned by matrix.g4.
 *
 * This rule deliberately accepts the canonical expression boundary rather than
 * duplicating matrix literals or matrix calls.
 * ============================================================================
 */

linearAlgebraMatrix
    : expression
    ;


/*
 * ============================================================================
 * 11. LINEAR-ALGEBRA TENSOR BOUNDARY
 * ============================================================================
 *
 * Tensor syntax remains owned by tensor.g4.
 * ============================================================================
 */

linearAlgebraTensor
    : expression
    ;


/*
 * ============================================================================
 * 12. LINEAR-ALGEBRA OPERATOR BOUNDARY
 * ============================================================================
 *
 * An operation is represented by the ordinary Zamani expression system.
 *
 * Examples:
 *
 *     transpose(A)
 *     solve(A, b)
 *     A * B
 *     A @ B
 *
 * The semantic layer determines whether the expression represents a
 * linear-algebra operation.
 * ============================================================================
 */

linearAlgebraOperation
    : expression
    ;


/*
 * ============================================================================
 * 13. LINEAR-ALGEBRA TRANSFORMATION BOUNDARY
 * ============================================================================
 *
 * Transformations such as:
 *
 *     transpose
 *     reshape
 *     permute
 *     factorize
 *     decompose
 *
 * remain ordinary operations.
 *
 * This rule exists only as a semantic-domain integration point.
 * ============================================================================
 */

linearAlgebraTransformation
    : expression
    ;


/*
 * ============================================================================
 * 14. LINEAR-ALGEBRA SOLVER BOUNDARY
 * ============================================================================
 *
 * Solver expressions such as:
 *
 *     solve(A, b)
 *     least_squares(A, b)
 *     iterative_solve(A, b)
 *
 * are ordinary expressions.
 *
 * The semantic layer chooses or validates the actual mathematical semantics.
 * ============================================================================
 */

linearAlgebraSolver
    : expression
    ;


/*
 * ============================================================================
 * 15. LINEAR-ALGEBRA DECOMPOSITION BOUNDARY
 * ============================================================================
 *
 * Decompositions such as:
 *
 *     LU
 *     QR
 *     SVD
 *     eigendecomposition
 *     Schur
 *     Cholesky
 *
 * are semantic operations rather than grammar keywords.
 * ============================================================================
 */

linearAlgebraDecomposition
    : expression
    ;


/*
 * ============================================================================
 * 16. LINEAR-ALGEBRA REDUCTION BOUNDARY
 * ============================================================================
 *
 * Examples:
 *
 *     norm(A)
 *     trace(A)
 *     sum(A)
 *     product(A)
 *     determinant(A)
 *
 * are ordinary expressions.
 * ============================================================================
 */

linearAlgebraReduction
    : expression
    ;


/*
 * ============================================================================
 * 17. LINEAR-ALGEBRA PRODUCT BOUNDARY
 * ============================================================================
 *
 * Examples:
 *
 *     dot(a, b)
 *     outer(a, b)
 *     matmul(A, B)
 *     kron(A, B)
 *
 * remain ordinary expressions.
 * ============================================================================
 */

linearAlgebraProduct
    : expression
    ;


/*
 * ============================================================================
 * 18. LINEAR-ALGEBRA INDEXED VALUE
 * ============================================================================
 *
 * Indexing remains owned by expressions/indexing.g4.
 *
 * This boundary is intentionally represented by the canonical expression.
 * ============================================================================
 */

linearAlgebraIndexedValue
    : expression
    ;


/*
 * ============================================================================
 * 19. LINEAR-ALGEBRA SHAPE EXPRESSION
 * ============================================================================
 *
 * Shape is represented by an ordinary expression.
 *
 * Examples:
 *
 *     N
 *     M
 *     N * M
 *     rows
 *     columns
 *
 * Shape validity and dimension semantics are downstream concerns.
 * ============================================================================
 */

linearAlgebraShapeExpression
    : expression
    ;


/*
 * ============================================================================
 * 20. LINEAR-ALGEBRA DIMENSION EXPRESSION
 * ============================================================================
 *
 * Dimensions are semantic values, not grammar limits.
 * ============================================================================
 */

linearAlgebraDimensionExpression
    : expression
    ;


/*
 * ============================================================================
 * 21. LINEAR-ALGEBRA SYMBOLIC EXPRESSION
 * ============================================================================
 *
 * Symbolic linear algebra uses the ordinary expression system.
 * ============================================================================
 */

linearAlgebraSymbolicExpression
    : expression
    ;


/*
 * ============================================================================
 * 22. LINEAR-ALGEBRA NUMERICAL EXPRESSION
 * ============================================================================
 *
 * Numerical classification belongs to semantic/type analysis.
 * ============================================================================
 */

linearAlgebraNumericalExpression
    : expression
    ;


/*
 * ============================================================================
 * 23. LINEAR-ALGEBRA CONSTANT BOUNDARY
 * ============================================================================
 *
 * Constant-evaluability is a compiler/semantic property.
 * ============================================================================
 */

linearAlgebraConstantExpression
    : expression
    ;


/*
 * ============================================================================
 * 24. LINEAR-ALGEBRA COMPILE-TIME BOUNDARY
 * ============================================================================
 *
 * Compile-time evaluation is not established by parser syntax.
 * ============================================================================
 */

linearAlgebraCompileTimeExpression
    : expression
    ;


/*
 * ============================================================================
 * 25. LINEAR-ALGEBRA RESOURCE REQUIREMENT BOUNDARY
 * ============================================================================
 *
 * Resource syntax remains owned by grammar/resources/.
 *
 * This file does not define:
 *
 *     requires
 *     capability
 *     memory
 *     placement
 *     target
 *
 * syntax a second time.
 *
 * A semantic consumer can classify an existing canonical resource expression
 * as linear-algebra-related.
 * ============================================================================
 */

linearAlgebraResourceRequirement
    : expression
    ;


/*
 * ============================================================================
 * 26. LINEAR-ALGEBRA CAPABILITY BOUNDARY
 * ============================================================================
 *
 * Capability resolution is semantic.
 * ============================================================================
 */

linearAlgebraCapabilityRequirement
    : expression
    ;


/*
 * ============================================================================
 * 27. LINEAR-ALGEBRA CONSTRAINT BOUNDARY
 * ============================================================================
 *
 * Constraints are interpreted by the semantic/resource subsystem.
 * ============================================================================
 */

linearAlgebraConstraint
    : expression
    ;


/*
 * ============================================================================
 * 28. LINEAR-ALGEBRA PREFERENCE BOUNDARY
 * ============================================================================
 *
 * Preferences must never become target-selection syntax embedded in this
 * grammar.
 * ============================================================================
 */

linearAlgebraPreference
    : expression
    ;


/*
 * ============================================================================
 * 29. LINEAR-ALGEBRA PURE COMPUTATION BOUNDARY
 * ============================================================================
 *
 * Purity is an effect-system property.
 * ============================================================================
 */

linearAlgebraPureComputation
    : expression
    ;


/*
 * ============================================================================
 * 30. LINEAR-ALGEBRA EFFECTFUL COMPUTATION BOUNDARY
 * ============================================================================
 *
 * Effect classification belongs to the canonical effects subsystem.
 * ============================================================================
 */

linearAlgebraEffectfulComputation
    : expression
    ;


/*
 * ============================================================================
 * 31. LINEAR-ALGEBRA CLASSICAL/QUANTUM BOUNDARY
 * ============================================================================
 *
 * A linear-algebra expression may participate in a hybrid program.
 *
 * This grammar does not define quantum syntax.
 *
 * Quantum semantic lowering remains:
 *
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * ============================================================================
 */

linearAlgebraQuantumBoundary
    : expression
    ;


/*
 * ============================================================================
 * 32. LINEAR-ALGEBRA DATA BOUNDARY
 * ============================================================================
 *
 * Data, tensors, collections, streams, and distributed values remain ordinary
 * Zamani semantic values.
 * ============================================================================
 */

linearAlgebraDataBoundary
    : expression
    ;


/*
 * ============================================================================
 * 33. LINEAR-ALGEBRA DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Distributed realization is downstream.
 *
 * The grammar does not encode:
 *
 *     node0
 *     node1
 *     nodeCount
 *     shardCount
 *     fixed topology
 *
 * ============================================================================
 */

linearAlgebraDistributedBoundary
    : expression
    ;


/*
 * ============================================================================
 * 34. LINEAR-ALGEBRA ACCELERATOR BOUNDARY
 * ============================================================================
 *
 * Accelerator selection is downstream.
 * ============================================================================
 */

linearAlgebraAcceleratorBoundary
    : expression
    ;


/*
 * ============================================================================
 * 35. LINEAR-ALGEBRA HDL/HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware realization belongs to:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/resources/
 *
 * This rule is a semantic integration hook only.
 * ============================================================================
 */

linearAlgebraHardwareBoundary
    : expression
    ;


/*
 * ============================================================================
 * 36. LINEAR-ALGEBRA DOMAIN CLASSIFICATION
 * ============================================================================
 *
 * This rule provides one stable semantic entry point for downstream consumers
 * that need to classify a source construct as potentially linear algebra.
 *
 * It deliberately uses existing canonical syntax rather than creating new
 * syntax.
 * ============================================================================
 */

linearAlgebraSemanticBoundary
    : expression
    | typeExpression
    ;


/*
 * ============================================================================
 * 37. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when all of the following remain true:
 *
 *     [x] It composes with the canonical Zamani lexer.
 *
 *     [x] It composes with the canonical expression grammar.
 *
 *     [x] It composes with the canonical type grammar.
 *
 *     [x] It does not redefine identifiers.
 *
 *     [x] It does not redefine operators.
 *
 *     [x] It does not redefine expression precedence.
 *
 *     [x] It does not redefine function calls.
 *
 *     [x] It does not redefine indexing.
 *
 *     [x] It does not redefine matrix syntax.
 *
 *     [x] It does not redefine vector syntax.
 *
 *     [x] It does not redefine tensor syntax.
 *
 *     [x] It does not define a second AST.
 *
 *     [x] It does not define a second IR.
 *
 *     [x] It does not enumerate linear-algebra algorithms.
 *
 *     [x] It permits future operation names without grammar changes.
 *
 *     [x] It permits qualified operation names through canonical expressions.
 *
 *     [x] It imposes no matrix-size limit.
 *
 *     [x] It imposes no vector-size limit.
 *
 *     [x] It imposes no tensor-rank limit.
 *
 *     [x] It imposes no operand-count limit.
 *
 *     [x] It imposes no machine-size limit.
 *
 *     [x] It does not select hardware.
 *
 *     [x] It does not select a vendor library.
 *
 *     [x] It does not perform resource discovery.
 *
 *     [x] It does not perform scheduling.
 *
 *     [x] It does not perform placement.
 *
 *     [x] It does not perform numerical algorithm selection.
 *
 *     [x] It does not create a quantum IR.
 *
 *     [x] It remains compatible with POCO-REAF.
 *
 *     [x] It requires no unsafe Rust.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * The fundamental rule of this file is:
 *
 *     SYNTAX IS GENERIC.
 *     SEMANTICS ARE PRECISE.
 *     RESOURCES ARE NEGOTIATED.
 *     HARDWARE IS DISCOVERED DOWNSTREAM.
 *
 * Therefore:
 *
 *     linear_algebra::solve(A, b)
 *
 * describes an operation.
 *
 * It does not prescribe:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     memory bank
 *     SIMD width
 *     node
 *     device
 *     vendor
 *
 * The compiler is responsible for finding a valid realization from the
 * available target capabilities and resources.
 *
 * This is the linear-algebra portion of:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */