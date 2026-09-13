/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/constraints.g4
 *
 * Role:
 *     Canonical parser delegate for FUNCTION GENERIC CONSTRAINTS.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, runtime calls, or unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * This file owns the syntax used by function declarations to express
 * constraints on generic parameters.
 *
 * Examples:
 *
 *     fn add<T>(a: T, b: T) -> T
 *     where T: Numeric
 *     { ... }
 *
 *     fn measure<T>(value: T) -> Result
 *     where T: quantum::State
 *     { ... }
 *
 *     fn transform<T, U>(value: T) -> U
 *     where T: Numeric + Comparable,
 *           U: Serializable
 *     { ... }
 *
 * This file establishes syntax only.
 *
 * It does NOT decide:
 *
 *     - whether a bound is satisfiable;
 *     - whether a type implements a trait;
 *     - whether a type is a subtype;
 *     - whether a capability exists;
 *     - whether a hardware resource exists;
 *     - whether a quantum backend can satisfy a requirement;
 *     - whether a function can be specialized;
 *     - whether generic inference succeeds.
 *
 * Those decisions belong to semantic analysis.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - function-level generic constraint clauses;
 *     - lists of function generic constraints;
 *     - individual function generic constraints;
 *     - the syntactic relationship between a generic parameter and its
 *       type-bound expression;
 *     - ordering of function constraints.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - lexical tokens;
 *     - generic parameter declarations;
 *     - generic type applications;
 *     - canonical type expressions;
 *     - general expressions;
 *     - general source-level constraints;
 *     - type-system constraint syntax;
 *     - trait/interface declarations;
 *     - capability declarations;
 *     - resource declarations;
 *     - requirements;
 *     - hardware;
 *     - quantum semantics;
 *     - QEC;
 *     - ZQN;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - resilience;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - runtime behavior.
 *
 * ============================================================================
 * IMPORTANT OWNERSHIP BOUNDARIES
 * ============================================================================
 *
 * Generic parameter declaration:
 *
 *     grammar/functions/generics.g4
 *
 * Type constraint syntax:
 *
 *     grammar/types/type-constraints.g4
 *
 * General source-level constraints:
 *
 *     grammar/core/constraints.g4
 *
 * Type expressions:
 *
 *     grammar/types/types.g4
 *
 * Generic applications:
 *
 *     grammar/types/generic-types.g4
 *
 * Parameter declarations:
 *
 *     grammar/functions/parameters.g4
 *
 * Function declaration framing:
 *
 *     grammar/functions/functions.g4
 *
 * This file must consume those contracts rather than redefine them.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * canonical lexer
 *       |
 *       v
 * canonical names
 *       |
 *       +------------------------------+
 *       |                              |
 *       v                              v
 * generic parameter grammar       type grammar
 *       |                              |
 *       +---------------+--------------+
 *                       |
 *                       v
 *          functions/constraints.g4
 *                       |
 *                       v
 *                  frontend AST
 *                       |
 *                       v
 *                semantic analysis
 *                       |
 *        +--------------+---------------+
 *        |              |               |
 *        v              v               v
 *   type checking   constraint       capability /
 *                   solving          requirement analysis
 *        |              |               |
 *        +--------------+---------------+
 *                       |
 *                       v
 *                canonical semantics
 *                       |
 *        +--------------+---------------+
 *        |              |               |
 *        v              v               v
 *   classical IR   quantum::ir     HDL/hardware IR
 *
 * This file MUST NOT depend on:
 *
 *     IR
 *     runtime
 *     hardware discovery
 *     QEC
 *     ZQN
 *     scheduling
 *     routing
 *     optimization
 *     resilience
 *
 * ============================================================================
 * FUNCTION CONSTRAINT MODEL
 * ============================================================================
 *
 * A function generic constraint associates a generic parameter with a
 * type-level bound.
 *
 * Conceptually:
 *
 *     where T: Numeric
 *
 *     where T: Numeric + Comparable
 *
 *     where T: quantum::State
 *
 *     where T: quantum::State + quantum::Measurable
 *
 * The grammar preserves this source structure.
 *
 * Semantic analysis determines what the bound means.
 *
 * ============================================================================
 * WHY THIS FILE IS NOT A SECOND GENERAL CONSTRAINT LANGUAGE
 * ============================================================================
 *
 * The repository already has:
 *
 *     grammar/core/constraints.g4
 *
 * for general semantic constraints.
 *
 * This file must not duplicate rules such as:
 *
 *     booleanConstraint
 *     relationalConstraint
 *     comparisonConstraint
 *     resourceConstraint
 *     capabilityConstraint
 *
 * because function generic bounds are specifically type-level syntax.
 *
 * Likewise, this file must not duplicate the type-system constraint grammar
 * where that grammar already owns canonical type-bound representation.
 *
 * The function layer owns the function-specific attachment point:
 *
 *     WHERE genericParameterName COLON typeConstraint
 *
 * and delegates the actual bound structure to the canonical type-constraint
 * rule where possible.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Function constraints express semantic requirements on generic types.
 *
 * They MUST NOT encode physical machine characteristics.
 *
 * Forbidden grammar-level concepts include:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_DEVICES
 *     MAX_NODES
 *     fixed memory sizes
 *     fixed topology
 *     fixed register counts
 *     fixed accelerator counts
 *     fixed device identifiers
 *     physical addresses
 *
 * A constraint such as:
 *
 *     where T: quantum::State
 *
 * remains meaningful across:
 *
 *     quantum simulators
 *     quantum processors
 *     embedded systems
 *     distributed systems
 *     future computational substrates
 *
 * Target realization belongs downstream.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Generic constraints use canonical type expressions rather than an
 * enumerated list of domains.
 *
 * Therefore this file does NOT define rules such as:
 *
 *     numericConstraint
 *     quantumConstraint
 *     gpuConstraint
 *     fpgaConstraint
 *     tensorConstraint
 *     hardwareConstraint
 *
 * A future domain can introduce a new type:
 *
 *     future::SomeCapability
 *
 * without requiring this grammar to change.
 *
 * ============================================================================
 * SYNTAX OWNERSHIP
 * ============================================================================
 *
 * This file owns:
 *
 *     functionConstraintClause
 *     functionConstraintList
 *     functionConstraint
 *
 * It intentionally does NOT own the generic parameter itself.
 *
 * Generic parameter ownership remains in:
 *
 *     functions/generics.g4
 *
 * Example:
 *
 *     <T, U>
 *
 * is generic-parameter syntax.
 *
 * The following is function-constraint syntax:
 *
 *     where T: Numeric,
 *           U: Serializable
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser delegate grammar.
 *
 * The canonical/root parser should import it.
 *
 * Example:
 *
 *     parser grammar ZamaniParser;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 *     import FunctionConstraints;
 *
 * The surrounding function grammar may then use:
 *
 *     functionConstraintClause?
 *
 * without knowing the internal structure of an individual constraint.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar consumes canonical lexer tokens.
 *
 * It MUST NOT declare lexer rules.
 *
 * Expected canonical tokens include:
 *
 *     WHERE
 *     COLON
 *     COMMA
 *
 * and any punctuation/operator tokens required by canonical type expressions.
 *
 * The repository lexer already owns:
 *
 *     WHERE
 *     DOUBLE_COLON
 *     EQUAL_EQUAL
 *     NOT_EQUAL
 *     LESS
 *     LESS_EQUAL
 *     GREATER
 *     GREATER_EQUAL
 *
 * This grammar must not recreate any of them.
 *
 * ============================================================================
 * TYPE EXPRESSION CONTRACT
 * ============================================================================
 *
 * The actual bound is consumed through:
 *
 *     typeConstraint
 *
 * when the canonical type-constraint grammar exposes that rule.
 *
 * If the composed parser architecture exposes only the canonical
 * `typeExpression` rule, this file must consume that rule instead of
 * introducing a duplicate type grammar.
 *
 * The repository's type layer remains authoritative for:
 *
 *     primitive types
 *     references
 *     qualified types
 *     generic applications
 *     tuples
 *     arrays
 *     maps
 *     functions
 *     quantum types
 *     hardware types
 *     resource types
 *     future extensible types
 *
 * ============================================================================
 * PARAMETER NAME CONTRACT
 * ============================================================================
 *
 * A function constraint references an already-declared generic parameter.
 *
 * Example:
 *
 *     fn f<T>() where T: Numeric
 *
 * The parser records:
 *
 *     T
 *
 * but does not verify that T was actually declared.
 *
 * That validation belongs to semantic analysis.
 *
 * Therefore this grammar intentionally accepts:
 *
 *     where Unknown: Numeric
 *
 * syntactically.
 *
 * The semantic layer must subsequently report that `Unknown` is not a valid
 * generic parameter in the current function declaration.
 *
 * ============================================================================
 * ORDER PRESERVATION
 * ============================================================================
 *
 * Source ordering MUST be preserved.
 *
 * Example:
 *
 *     where T: A,
 *           U: B,
 *           V: C
 *
 * must remain ordered as:
 *
 *     T -> A
 *     U -> B
 *     V -> C
 *
 * The grammar must not:
 *
 *     sort;
 *     deduplicate;
 *     normalize;
 *     resolve;
 *     specialize;
 *     infer.
 *
 * Semantic normalization belongs downstream.
 *
 * ============================================================================
 * MULTIPLE BOUNDS
 * ============================================================================
 *
 * Multiple bounds on one parameter are represented by the canonical
 * type-constraint grammar.
 *
 * Example:
 *
 *     where T: Numeric + Comparable
 *
 * The function grammar therefore does not invent a second `+`-based type
 * system.
 *
 * This is important because:
 *
 *     T: A + B
 *
 * is a type-bound structure, not a general boolean expression.
 *
 * ============================================================================
 * MULTIPLE CONSTRAINTS
 * ============================================================================
 *
 * Multiple generic parameters may have constraints:
 *
 *     where T: Numeric,
 *           U: Serializable,
 *           V: quantum::State
 *
 * The grammar uses repetition rather than a fixed number of constraints.
 *
 * Therefore there is no grammar-level maximum for:
 *
 *     number of generic parameters
 *     number of constrained parameters
 *     number of bounds
 *     qualified-name depth
 *     generic nesting
 *     program size
 *
 * Practical parser/compiler limits are resource/security policy and are not
 * language semantics.
 *
 * ============================================================================
 * TRAILING COMMA POLICY
 * ============================================================================
 *
 * The canonical language policy must decide whether:
 *
 *     where T: Numeric,
 *
 * is valid.
 *
 * This file must not silently introduce a function-specific trailing-comma
 * convention.
 *
 * The production below intentionally follows the repository's existing
 * list convention:
 *
 *     item (COMMA item)*
 *
 * Therefore a trailing comma is not accepted unless the canonical language
 * specification is changed to make trailing commas generally valid.
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * The frontend AST must preserve source spans for:
 *
 *     - entire function constraint clause;
 *     - each constrained parameter;
 *     - each type bound;
 *     - each compound bound;
 *     - separators/operators where diagnostics require them.
 *
 * This supports diagnostics such as:
 *
 *     generic parameter `T` does not satisfy `Numeric`
 *
 * without requiring the grammar to perform semantic validation.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Conceptual AST representation:
 *
 *     FunctionConstraintClause {
 *         constraints: Vec<FunctionConstraint>,
 *         source_span
 *     }
 *
 *     FunctionConstraint {
 *         parameter: GenericParameterReference,
 *         bound: TypeConstraint,
 *         source_span
 *     }
 *
 * The exact Rust AST types belong to the frontend AST subsystem.
 *
 * This grammar must never embed Rust structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - generic parameter lookup;
 *     - duplicate constraint detection;
 *     - type resolution;
 *     - trait/interface resolution;
 *     - bound compatibility;
 *     - subtype checking;
 *     - associated-type validation;
 *     - capability interpretation;
 *     - generic inference;
 *     - constraint solving;
 *     - satisfiability;
 *     - specialization;
 *     - monomorphization.
 *
 * The grammar only establishes syntax.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum types can appear as ordinary type bounds:
 *
 *     where T: quantum::State
 *
 *     where T: quantum::Operation
 *
 *     where T: quantum::Observable
 *
 *     where T: quantum::LogicalQubit
 *
 * The grammar does not define these types.
 *
 * It also does not define:
 *
 *     physical qubit allocation
 *     coupling maps
 *     gate sets
 *     calibration
 *     backend selection
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * No dependency from this grammar to `quantum::ir` is permitted.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical bounds may include:
 *
 *     where T: Numeric
 *     where T: Comparable
 *     where T: Iterable
 *     where T: Serializable
 *
 * These are semantic types/traits/interfaces and are not hard-coded into
 * this grammar.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-oriented types may appear as qualified bounds:
 *
 *     where T: hardware::Signal
 *     where T: hardware::Memory
 *     where T: hardware::Accelerator
 *     where T: hdl::Module
 *
 * The grammar does not enumerate hardware classes.
 *
 * Therefore adding:
 *
 *     future::Accelerator
 *
 * does not require changing this file.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * A type constraint is not a resource requirement.
 *
 * For example:
 *
 *     where T: quantum::State
 *
 * does not mean:
 *
 *     allocate a quantum processor;
 *     allocate a fixed number of qubits;
 *     choose a particular backend;
 *     reserve a particular device.
 *
 * Resource requirements belong to the resource/requirement system.
 *
 * This distinction is essential for POCO-REAF.
 *
 * ============================================================================
 * EFFECT / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Function generic bounds may eventually reference types representing
 * capabilities or effects.
 *
 * Example:
 *
 *     where T: capability::Serializable
 *
 * or:
 *
 *     where T: effect::QuantumCompatible
 *
 * This grammar does not resolve whether those names represent:
 *
 *     traits
 *     interfaces
 *     capabilities
 *     effect carriers
 *     domain abstractions
 *
 * Semantic analysis owns that interpretation.
 *
 * ============================================================================
 * GENERAL CONSTRAINT INTEGRATION
 * ============================================================================
 *
 * General conditions such as:
 *
 *     where value >= minimum
 *
 * are NOT function generic type bounds merely because they appear after a
 * `where` clause.
 *
 * If Zamani supports both type bounds and arbitrary function predicates,
 * the language specification must define a distinct production and semantic
 * distinction.
 *
 * This file therefore must not silently absorb arbitrary expressions into
 * generic type constraints.
 *
 * This prevents:
 *
 *     functions/constraints.g4
 *
 * from becoming a duplicate of:
 *
 *     core/constraints.g4
 *
 * ============================================================================
 * DUPLICATE PARAMETER POLICY
 * ============================================================================
 *
 * The following is syntactically representable:
 *
 *     where T: A, T: B
 *
 * This grammar must preserve it rather than silently merge the bounds.
 *
 * Semantic analysis decides whether repeated constraints on the same generic
 * parameter are:
 *
 *     valid and conjunctive;
 *     redundant;
 *     conflicting;
 *     malformed.
 *
 * The parser must not make that semantic decision.
 *
 * ============================================================================
 * UNKNOWN TYPES
 * ============================================================================
 *
 * The following should remain syntactically valid:
 *
 *     where T: future::UnknownType
 *
 * if it satisfies the canonical type-expression grammar.
 *
 * Whether the type resolves is a semantic question.
 *
 * This open-world property is necessary for long-term language evolution and
 * POCO-REAF.
 *
 * ============================================================================
 * INVALID SYNTAX
 * ============================================================================
 *
 * The grammar must reject malformed function generic constraint syntax such
 * as:
 *
 *     where
 *
 *     where T
 *
 *     where : Numeric
 *
 *     where T:
 *
 *     where T: + Numeric
 *
 *     where T: Numeric +
 *
 *     where T: Numeric,
 *
 *     where T: , U: Numeric
 *
 * The parser must not invent missing:
 *
 *     parameter names;
 *     colons;
 *     type bounds;
 *     separators.
 *
 * ============================================================================
 * SEMANTICALLY INVALID BUT SYNTACTICALLY VALID
 * ============================================================================
 *
 * These may parse successfully and must be diagnosed downstream:
 *
 *     where Unknown: Numeric
 *
 *     where T: UnknownType
 *
 *     where T: IncompatibleBound
 *
 *     where T: SomeConcreteTypeThatCannotBeUsedAsABound
 *
 *     where T: A, T: ConflictingBound
 *
 * This separation provides stable parser behavior and high-quality semantic
 * diagnostics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded actions;
 *     no semantic predicates;
 *     no mutable external state;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no runtime calls;
 *     no random behavior.
 *
 * Identical source/token streams therefore produce identical parse trees.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar must not:
 *
 *     execute user code;
 *     invoke external processes;
 *     access files;
 *     access network resources;
 *     discover hardware;
 *     query runtime state;
 *     evaluate type bounds.
 *
 * Potentially expensive operations such as constraint solving, type
 * resolution, specialization, and inference belong downstream and must be
 * governed by compiler resource/security policies rather than grammar
 * cardinality restrictions.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are no language-level fixed limits for:
 *
 *     generic parameter count;
 *     constraint count;
 *     bound count;
 *     qualified-name depth;
 *     generic nesting;
 *     declaration count;
 *     function count;
 *     source size;
 *     quantum resource count;
 *     classical resource count;
 *     hardware resource count;
 *     distributed node count.
 *
 * The grammar uses repetition:
 *
 *     *
 *
 * rather than fixed-size alternatives.
 *
 * "Infinity" therefore means:
 *
 *     no artificial language maximum.
 *
 * Actual execution remains bounded by the resources available to the parser,
 * compiler, runtime, and target environment.
 *
 * ============================================================================
 * COMPILATION CONTRACT
 * ============================================================================
 *
 * Source:
 *
 *     function declaration
 *         + generic parameters
 *         + function constraint clause
 *
 *       |
 *       v
 *
 * ANTLR parse tree
 *
 *       |
 *       v
 *
 * frontend AST
 *
 *       |
 *       v
 *
 * generic/type semantic analysis
 *
 *       |
 *       v
 *
 * validated semantic representation
 *
 *       |
 *       +-------------------+-------------------+
 *       |                   |                   |
 *       v                   v                   v
 * classical lowering   quantum lowering     HDL/hardware
 *                                           lowering
 *
 *       |
 *       v
 *
 * canonical IR
 *
 *       |
 *       v
 *
 * optimization / routing / scheduling / resilience
 *
 *       |
 *       v
 *
 * target realization
 *
 * There is no reverse dependency from IR or runtime into this grammar.
 *
 * ============================================================================
 * FUNCTION SIGNATURE INTEGRATION
 * ============================================================================
 *
 * The surrounding function grammar should conceptually compose:
 *
 *     functionDeclaration
 *         : functionName
 *           genericParameterList?
 *           LPAREN parameterList? RPAREN
 *           returnClause?
 *           functionConstraintClause?
 *           functionBody
 *         ;
 *
 * The exact surrounding rule remains owned by:
 *
 *     functions/functions.g4
 *
 * This file supplies only:
 *
 *     functionConstraintClause
 *     functionConstraintList
 *     functionConstraint
 *
 * ============================================================================
 * GENERIC PARAMETER INTEGRATION
 * ============================================================================
 *
 * The generic parameter declaration:
 *
 *     <T, U>
 *
 * belongs to:
 *
 *     functions/generics.g4
 *
 * A constraint:
 *
 *     where T: Numeric
 *
 * belongs here.
 *
 * The generic parameter declaration and constraint clause are therefore
 * intentionally separated.
 *
 * This prevents a future change to generic parameter syntax from requiring
 * changes to the fundamental representation of type bounds.
 *
 * ============================================================================
 * TYPE-CONSTRAINT INTEGRATION
 * ============================================================================
 *
 * `typeConstraint` is expected to be the canonical reusable rule from:
 *
 *     grammar/types/type-constraints.g4
 *
 * This is important because the repository explicitly assigns type-level
 * bounds to that file.
 *
 * Function constraints therefore attach a generic parameter to the canonical
 * type constraint rather than inventing:
 *
 *     functionTypeConstraint
 *
 * with duplicate semantics.
 *
 * ============================================================================
 * NO HARD-CODED DOMAIN LIST
 * ============================================================================
 *
 * Do not add:
 *
 *     | QUANTUM_CONSTRAINT
 *     | CLASSICAL_CONSTRAINT
 *     | GPU_CONSTRAINT
 *     | FPGA_CONSTRAINT
 *     | ASIC_CONSTRAINT
 *     | AI_CONSTRAINT
 *     | NETWORK_CONSTRAINT
 *
 * Such rules would make the universal grammar evolve every time a new
 * computational domain appears.
 *
 * The correct abstraction is the canonical type-expression/type-constraint
 * system.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Syntax changes to this file must be coordinated with:
 *
 *     grammar/specification/language-version.md
 *     grammar/specification/compatibility.md
 *     grammar/compatibility/versions.md
 *     grammar/compatibility/migrations.md
 *     functions/generics.g4
 *     functions/functions.g4
 *     types/type-constraints.g4
 *     frontend AST
 *     semantic/type analysis
 *     parser tests
 *     round-trip tests
 *
 * Adding a new type name must NOT require modifying this file.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive cases:
 *
 *     where T: Numeric
 *
 *     where T: Numeric + Comparable
 *
 *     where T: quantum::State
 *
 *     where T: quantum::State + quantum::Measurable
 *
 *     where T: hdl::Module
 *
 *     where T: hardware::Accelerator
 *
 *     where T: future::SomeType
 *
 *     where T: A, U: B
 *
 *     where T: A + B, U: C + D
 *
 * Negative cases:
 *
 *     where
 *
 *     where T
 *
 *     where : Numeric
 *
 *     where T:
 *
 *     where T: + Numeric
 *
 *     where T: Numeric +
 *
 *     where T: Numeric, 
 *
 *     where T: , U: Numeric
 *
 * Boundary cases:
 *
 *     a single generic parameter;
 *     many generic parameters;
 *     many constraints;
 *     many bounds;
 *     deeply qualified types;
 *     deeply nested generic types;
 *     large source files;
 *     empty function parameter lists;
 *     functions without constraints;
 *     functions with constraints only;
 *     functions spanning multiple lines.
 *
 * Cross-domain cases:
 *
 *     classical generic + quantum bound;
 *     classical generic + hardware bound;
 *     quantum generic + classical bound;
 *     HDL generic + hardware bound;
 *     distributed generic + resource-independent type bound.
 *
 * Determinism tests:
 *
 *     identical source -> identical tokens -> identical parse tree.
 *
 * Round-trip tests:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter/serializer
 *       -> parser
 *
 * must preserve the semantic structure of the constraint clause.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file must contain:
 *
 *     no fixed generic-parameter maximum;
 *     no fixed constraint maximum;
 *     no fixed bound maximum;
 *     no fixed qualified-name depth;
 *     no fixed generic nesting limit;
 *     no CPU count;
 *     no GPU count;
 *     no FPGA count;
 *     no ASIC count;
 *     no qubit count;
 *     no node count;
 *     no memory size;
 *     no register size;
 *     no topology;
 *     no device ID;
 *     no hardware address;
 *     no vendor-specific resource requirement.
 *
 * Any future limit discovered here must be classified as:
 *
 *     semantic requirement;
 *     parser implementation/resource policy;
 *     security policy;
 *     test fixture limitation;
 *     accidental hard-coding.
 *
 * Accidental hard-coding must be removed.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     [ ] It is a parser grammar.
 *     [ ] It contains no lexer rules.
 *     [ ] It contains no embedded Rust.
 *     [ ] It requires no unsafe Rust.
 *     [ ] It targets Rust 1.97 / 1.97.1 through the repository's parser
 *         integration.
 *     [ ] It owns only function generic constraint syntax.
 *     [ ] It does not redefine identifiers.
 *     [ ] It does not redefine qualified names.
 *     [ ] It does not redefine generic parameter declarations.
 *     [ ] It does not redefine generic applications.
 *     [ ] It does not redefine expressions.
 *     [ ] It does not redefine general constraints.
 *     [ ] It does not redefine type constraints.
 *     [ ] It does not define quantum semantics.
 *     [ ] It does not depend on quantum::ir.
 *     [ ] It does not define hardware semantics.
 *     [ ] It does not define resource allocation.
 *     [ ] It does not define scheduling.
 *     [ ] It does not define routing.
 *     [ ] It does not define optimization.
 *     [ ] It does not define QEC.
 *     [ ] It does not define ZQN.
 *     [ ] It does not define resilience.
 *     [ ] It contains no machine-size limits.
 *     [ ] It contains no vendor-specific hardware assumptions.
 *     [ ] It preserves source ordering.
 *     [ ] It permits open-ended future type domains.
 *     [ ] It composes with functions/generics.g4.
 *     [ ] It composes with functions/functions.g4.
 *     [ ] It composes with types/type-constraints.g4.
 *     [ ] It composes with the canonical lexer.
 *     [ ] It has positive tests.
 *     [ ] It has negative tests.
 *     [ ] It has boundary tests.
 *     [ ] It has cross-domain tests.
 *     [ ] It has determinism tests.
 *     [ ] It has round-trip tests.
 *
 * ============================================================================
 */

/*
 * IMPORTANT:
 *
 * This is intentionally a parser delegate grammar.
 *
 * The repository's composed parser architecture should import it into the
 * canonical parser. It therefore does not declare a standalone lexer.
 */
parser grammar FunctionConstraints;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * FUNCTION CONSTRAINT CLAUSE
 * ========================================================================== */

/**
 * FunctionConstraintClause
 *
 * Associates one or more generic parameters with canonical type constraints.
 *
 * Example:
 *
 *     where T: Numeric
 *
 *     where T: Numeric, U: Serializable
 *
 * The clause itself is optional at the function-declaration level.
 */
functionConstraintClause
    : WHERE functionConstraintList
    ;


/* ============================================================================
 * FUNCTION CONSTRAINT LIST
 * ========================================================================== */

/**
 * FunctionConstraintList
 *
 * One or more function generic constraints separated by commas.
 *
 * No fixed cardinality is imposed.
 */
functionConstraintList
    : functionConstraint (COMMA functionConstraint)*
    ;


/* ============================================================================
 * INDIVIDUAL FUNCTION GENERIC CONSTRAINT
 * ========================================================================== */

/**
 * FunctionConstraint
 *
 * The constrained name is expected to resolve to a generic parameter declared
 * by the surrounding function.
 *
 * Example:
 *
 *     T: Numeric
 *
 * The parser deliberately does not verify that T was declared.
 *
 * That is a semantic-analysis responsibility.
 *
 * The bound is delegated to the canonical type-constraint grammar.
 */
functionConstraint
    : identifier COLON typeConstraint
    ;


/* ============================================================================
 * INTEGRATION BOUNDARY
 * ========================================================================== */

/*
 * The following rules are intentionally NOT defined here:
 *
 *     identifier
 *     qualifiedName
 *     typeExpression
 *     typeConstraint
 *     genericParameter
 *     genericParameterList
 *     expression
 *
 * Their canonical owners are responsible for those productions.
 *
 * This keeps FunctionConstraints independent and prevents grammar duplication.
 */