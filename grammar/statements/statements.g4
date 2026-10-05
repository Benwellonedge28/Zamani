/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/statements.g4
 *
 * STATUS
 * ------
 * CANONICAL UNIVERSAL STATEMENT COMPOSITION ROOT
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 *
 * SAFETY
 * ------
 * This grammar contains:
 *
 *   - no embedded Rust actions;
 *   - no semantic predicates;
 *   - no unsafe Rust;
 *   - no I/O;
 *   - no filesystem access;
 *   - no networking;
 *   - no hardware discovery;
 *   - no runtime execution;
 *   - no mutable parser-global state;
 *   - no target-specific implementation;
 *   - no machine-capacity constants.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE AUTHORITATIVE STATEMENT COMPOSITION BOUNDARY
 * for Zamani.
 *
 * It owns the universal:
 *
 *     statement
 *
 * production and composes independently owned statement families.
 *
 * This file MUST NOT implement the concrete syntax of individual statement
 * families.
 *
 * Its responsibility is:
 *
 *     source
 *       ->
 *     statement category
 *       ->
 *     canonical statement grammar
 *
 * The concrete syntax remains owned by specialized grammars.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     Statements                  <-- THIS FILE
 *       |
 *       +--> declarations
 *       +--> assignments
 *       +--> assertions
 *       +--> control flow
 *       +--> concurrency
 *       +--> effects
 *       +--> resources
 *       +--> reasoning
 *       +--> domains
 *       +--> unsafe regions
 *       +--> blocks
 *       +--> expressions
 *       +--> empty statements
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +--> names
 *       +--> types
 *       +--> ownership
 *       +--> effects
 *       +--> capabilities
 *       +--> resources
 *       +--> contracts
 *       +--> policies
 *       +--> provenance
 *       |
 *       v
 *     semantic model
 *       |
 *       +--> classical
 *       +--> quantum
 *       +--> hybrid
 *       +--> HDL
 *       +--> hardware
 *       +--> AI/model
 *       +--> data
 *       +--> distributed
 *       +--> networking
 *       +--> accelerator
 *       +--> future domains
 *       |
 *       v
 *     canonical IR
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> other domain IR
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     routing / scheduling / resilience
 *       |
 *       v
 *     ZQN / HAL / target realization
 *
 * This file MUST NOT bypass that architecture.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/blocks.g4
 *     grammar/expressions/expressions.g4
 *     grammar/statements/declarations.g4
 *     grammar/statements/assignments.g4
 *     grammar/statements/assertions.g4
 *     grammar/statements/control-flow.g4
 *     grammar/statements/concurrency.g4
 *     grammar/statements/effects.g4
 *     grammar/statements/resource.g4
 *     grammar/statements/reason.g4
 *     grammar/statements/domains.g4
 *     grammar/statements/unsafe.g4
 *
 * EXPORTS:
 *
 *     statement
 *     emptyStatement
 *     expressionStatement
 *
 * CONSUMED_BY:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 *     and any higher-level parser composition that imports Statements.
 *
 * AST_OWNER:
 *
 *     repository frontend AST implementation.
 *
 * SEMANTIC_OWNER:
 *
 *     repository semantic-analysis layer.
 *
 * IR_OWNER:
 *
 *     canonical semantic/domain IR layers.
 *
 *     Quantum statements ultimately use:
 *
 *         quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/
 *
 *     parser/conformance tests
 *     frontend AST tests
 *     semantic integration tests
 *
 * SPEC_OWNER:
 *
 *     grammar/DESIGN.md
 *     grammar/specification/
 *     grammar/spec/
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     statement
 *     emptyStatement
 *     expressionStatement
 *
 * It also owns the ordering and composition of statement categories.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexer rules
 *     tokens
 *     identifiers
 *     names
 *     expressions
 *     expression precedence
 *     declarations
 *     assignments
 *     assertions
 *     control-flow syntax
 *     loops
 *     conditionals
 *     pattern matching
 *     break
 *     continue
 *     return
 *     exceptions
 *     concurrency
 *     effects
 *     resources
 *     reasoning
 *     knowledge
 *     learning
 *     adaptation
 *     contracts
 *     policies
 *     quantum operations
 *     HDL syntax
 *     hardware syntax
 *     AI syntax
 *     data syntax
 *     networking syntax
 *     distributed syntax
 *     security syntax
 *     FFI
 *     ABI
 *     metaprogramming
 *     simulation
 *     execution
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *     backend selection
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one effective universal `statement` rule in the
 * assembled production parser.
 *
 * That owner is this file.
 *
 * Imported grammars MUST NOT define another universal `statement` rule.
 *
 * Imported grammars may expose specialized public rules such as:
 *
 *     declarationStatement
 *     assignmentStatement
 *     assertionStatement
 *     controlFlowStatement
 *     concurrencyStatementAdapter
 *     effectStatement
 *     resourceStatement
 *     reasonStatement
 *     domainStatement
 *     unsafeStatement
 *     blockExpression
 *     expressionStatement
 *
 * but those rules MUST remain subordinate to this composition root.
 *
 * ============================================================================
 * COMPOSITION PRINCIPLE
 * ============================================================================
 *
 * The hierarchy is:
 *
 *     leaf grammar
 *          |
 *          v
 *     feature/domain composition
 *          |
 *          v
 *     statement composition
 *          |
 *          v
 *     ZamaniParser
 *
 * This file MUST NOT reverse that dependency direction.
 *
 * In particular, this file MUST NOT import the root:
 *
 *     ZamaniParser
 *
 * and it MUST NOT import a grammar that imports Statements.
 *
 * ============================================================================
 * CANONICAL IMPORTS
 * ============================================================================
 *
 * Each imported grammar is a composition owner.
 *
 * Important corrections relative to the previous composition:
 *
 * 1. Control-flow leaves are NOT imported individually here.
 *
 *    ConditionalsParser
 *    Loops
 *    PatternMatching
 *    BreakStatements
 *    ContinueStatements
 *    ReturnStatements
 *    ExceptionsParser
 *
 *    are already composed by:
 *
 *        grammar/statements/control-flow.g4
 *
 * 2. Reasoning is composed through:
 *
 *        ReasonStatements
 *
 *    rather than importing separate infer/deduce grammars here.
 *
 * 3. The canonical block grammar is:
 *
 *        ZamaniCoreBlocks
 *
 *    from:
 *
 *        grammar/core/blocks.g4
 *
 *    The file:
 *
 *        grammar/statements/blocks.g4
 *
 *    must not become a second competing owner of `blockExpression`.
 *
 * 4. Expressions remain composed through:
 *
 *        Expressions
 *
 * 5. Domains remain composed through:
 *
 *        Domains
 *
 * ============================================================================
 */

parser grammar Statements;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * The order is intentionally organized by architectural ownership rather than
 * target technology.
 *
 * No individual domain leaf is imported directly when a domain composition
 * grammar already exists.
 * ============================================================================
 */

import
    Declarations,
    Assignments,
    AssertionsParser,
    ControlFlow,
    ConcurrencyStatements,
    EffectStatements,
    ResourceStatements,
    ReasonStatements,
    Domains,
    UnsafeStatementsParser,
    Expressions,
    ZamaniCoreBlocks
    ;


/*
 * ============================================================================
 * UNIVERSAL STATEMENT
 * ============================================================================
 *
 * This is the ONE universal statement entry point.
 *
 * Every source-level statement accepted by the canonical parser must enter
 * through this rule.
 *
 * The rule deliberately delegates concrete syntax.
 *
 * It does not implement any specialized statement itself.
 * ============================================================================
 */

statement
    : declarationStatement
    | assignmentStatement
    | assertionStatement
    | controlFlowStatement
    | concurrencyStatementAdapter
    | effectStatement
    | resourceStatement
    | reasonStatement
    | domainStatement
    | unsafeStatement
    | blockExpression
    | emptyStatement
    | expressionStatement
    ;


/*
 * ============================================================================
 * EMPTY STATEMENT
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     ;
 *
 * An empty statement has no expression and no child statement.
 *
 * Semantic tools may later classify it as:
 *
 *     permitted
 *     redundant
 *     deprecated
 *     lint-warning
 *
 * but those decisions do not belong in this grammar.
 * ============================================================================
 */

emptyStatement
    : SEMICOLON
    ;


/*
 * ============================================================================
 * EXPRESSION STATEMENT
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     expression ;
 *
 * The expression grammar owns the entire expression hierarchy.
 *
 * This rule owns only the transition:
 *
 *     expression
 *         ->
 *     statement position
 *
 * followed by the canonical statement terminator.
 *
 * No domain-specific expression is enumerated here.
 *
 * Therefore all valid expression forms can naturally become statement-level
 * operations where the language semantics permit them.
 * ============================================================================
 */

expressionStatement
    : expression SEMICOLON
    ;


/*
 * ============================================================================
 * DECLARATION INTEGRATION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/statements/declarations.g4
 *
 * Public rule consumed here:
 *
 *     declarationStatement
 *
 * This may ultimately cover:
 *
 *     bindings
 *     constants
 *     functions
 *     modules
 *     types
 *     traits
 *     implementations
 *     domain declarations
 *     hardware declarations
 *     quantum declarations
 *     distributed declarations
 *     data declarations
 *     AI/model declarations
 *
 * depending on the declaration composition architecture.
 *
 * This file does not duplicate any declaration syntax.
 *
 * Semantic validation remains downstream.
 * ============================================================================
 */


/*
 * ============================================================================
 * ASSIGNMENT INTEGRATION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/statements/assignments.g4
 *
 * Public rule:
 *
 *     assignmentStatement
 *
 * Assignment targets, operators and assignment expressions remain owned by
 * the expression/assignment subsystem.
 *
 * This file does not define:
 *
 *     =
 *     +=
 *     -=
 *     *=
 *     /=
 *     %=
 *     bitwise assignment
 *     target expressions
 *
 * as separate syntax.
 * ============================================================================
 */


/*
 * ============================================================================
 * ASSERTION INTEGRATION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/statements/assertions.g4
 *
 * Public rule:
 *
 *     assertionStatement
 *
 * Assertions are structural source constructs.
 *
 * Their semantic interpretation belongs to validation/contract analysis.
 *
 * This allows assertions to participate in:
 *
 *     classical verification
 *     quantum verification
 *     HDL verification
 *     hardware validation
 *     resource validation
 *     security validation
 *     AI/model validation
 *     distributed validation
 *
 * without making this file domain-specific.
 * ============================================================================
 */


/*
 * ============================================================================
 * CONTROL-FLOW INTEGRATION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/statements/control-flow.g4
 *
 * Public rule:
 *
 *     controlFlowStatement
 *
 * ControlFlow already composes:
 *
 *     conditionals
 *     loops
 *     pattern matching
 *     break
 *     continue
 *     return
 *     exceptions
 *
 * Therefore this file MUST NOT import those leaf grammars individually.
 *
 * This prevents multiple parser paths and duplicate ownership.
 * ============================================================================
 */


/*
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/statements/concurrency.g4
 *
 * Public adapter:
 *
 *     concurrencyStatementAdapter
 *
 * The concurrency subsystem ultimately composes constructs such as:
 *
 *     tasks
 *     parallelism
 *     actors
 *     channels
 *     synchronization
 *     cancellation
 *     asynchronous computation
 *     distributed concurrency
 *
 * This file does not define any of them.
 *
 * No fixed limit is imposed on:
 *
 *     tasks
 *     workers
 *     actors
 *     channels
 *     concurrent operations
 *     nodes
 *     devices
 *
 * Physical realization is downstream.
 * ============================================================================
 */


/*
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/statements/effects.g4
 *
 * Public rule:
 *
 *     effectStatement
 *
 * Effects are semantic capabilities of operations, not machine selections.
 *
 * The effect system may represent effects such as:
 *
 *     I/O
 *     network
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     code generation
 *     simulation
 *
 * This statement composition layer merely admits effect statements.
 * ============================================================================
 */


/*
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/statements/resource.g4
 *
 * Public rule:
 *
 *     resourceStatement
 *
 * Resource semantics remain target-neutral.
 *
 * Examples of semantic intent include:
 *
 *     requires capability("tensor.compute")
 *     requires capability("quantum.measurement")
 *     requires memory >= required_memory
 *     requires qubits >= required_qubits
 *     requires topology(required_topology)
 *
 * This grammar does not resolve those requirements.
 *
 * It MUST NOT turn logical requirements into:
 *
 *     CPU 0
 *     GPU 2
 *     QPU 1
 *     physical qubit 17
 *     FPGA region 4
 *     fixed node 8
 *
 * Resource negotiation belongs downstream.
 * ============================================================================
 */


/*
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/statements/reason.g4
 *
 * Grammar:
 *
 *     ReasonStatements
 *
 * Public rule:
 *
 *     reasonStatement
 *
 * Canonical reasoning forms include:
 *
 *     infer target;
 *     deduce target;
 *     reason target;
 *
 *     infer target from source;
 *     deduce target from source;
 *     reason target from source;
 *
 *     infer target from source with (context);
 *     deduce target from source with (context);
 *     reason target from source with (context);
 *
 * `infer`, `deduce` and `reason` are therefore admitted through ONE statement
 * family.
 *
 * This file MUST NOT import:
 *
 *     Infer
 *     Deduce
 *
 * separately when those grammars duplicate the same source forms.
 *
 * Reasoning remains generic and may be used for:
 *
 *     AI/model inference
 *     symbolic reasoning
 *     compiler decisions
 *     optimization decisions
 *     resource decisions
 *     security decisions
 *     hardware placement decisions
 *     quantum decisions
 *     scientific computation
 *     distributed decisions
 *
 * Reasoning algorithms, models, theorem provers and execution engines are
 * semantic/library/runtime concerns.
 * ============================================================================
 */


/*
 * ============================================================================
 * DOMAIN INTEGRATION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/statements/domains.g4
 *
 * Public rule:
 *
 *     domainStatement
 *
 * Domains currently composed there include:
 *
 *     classical
 *     quantum
 *     HDL
 *     hybrid
 *     distributed
 *     AI/model
 *     data
 *     networking
 *     security
 *     resources
 *     compilation
 *     execution
 *     hardware
 *
 * This file deliberately does not enumerate individual domain syntax.
 *
 * A new computational domain should normally be added behind the Domains
 * composition boundary rather than rewriting this universal statement grammar.
 * ============================================================================
 */


/*
 * ============================================================================
 * UNSAFE REGION INTEGRATION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/statements/unsafe.g4
 *
 * Public rule:
 *
 *     unsafeStatement
 *
 * Important distinction:
 *
 *     Zamani source `unsafe`
 *
 * does NOT imply:
 *
 *     unsafe Rust.
 *
 * The compiler implementation remains safe Rust.
 *
 * The semantic layer determines the capabilities, effects, permissions and
 * proof obligations associated with an unsafe source region.
 * ============================================================================
 */


/*
 * ============================================================================
 * BLOCK INTEGRATION
 * ============================================================================
 *
 * Canonical owner:
 *
 *     grammar/core/blocks.g4
 *
 * Canonical parser grammar:
 *
 *     ZamaniCoreBlocks
 *
 * Public rule:
 *
 *     blockExpression
 *
 * The statement composition layer admits blocks but does not redefine:
 *
 *     {
 *     }
 *
 * or:
 *
 *     blockElement
 *
 * The separate `grammar/statements/blocks.g4` file must not become a competing
 * parser grammar authority for `blockExpression`.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Owner:
 *
 *     grammar/expressions/expressions.g4
 *
 * Public rule:
 *
 *     expression
 *
 * The expression subsystem already composes the repository's expression
 * families, including where supported:
 *
 *     assignment expressions
 *     conditional expressions
 *     ranges
 *     unary expressions
 *     postfix expressions
 *     literals
 *     arrays
 *     maps
 *     tuples
 *     lambdas
 *     identifiers
 *     comprehensions
 *     patterns
 *     guards
 *     match expressions
 *     quantum expressions
 *     reasoning expressions
 *     knowledge expressions
 *     uncertainty expressions
 *     query expressions
 *     policy expressions
 *     effect expressions
 *     compile-time expressions
 *     metaprogramming expressions
 *
 * This file MUST NOT duplicate expression precedence or associativity.
 * ============================================================================
 */


/*
 * ============================================================================
 * STATEMENT TERMINATION
 * ============================================================================
 *
 * Core statement syntax currently uses:
 *
 *     SEMICOLON
 *
 * for generic expression statements and empty statements.
 *
 * Specialized grammars may use the canonical punctuation/terminator rule where
 * their existing ownership contract requires it.
 *
 * This file MUST NOT invent alternative terminators.
 *
 * A future change to statement termination is compatibility-sensitive and must
 * be handled through:
 *
 *     specification
 *     lexer
 *     parser
 *     AST
 *     formatter
 *     compatibility
 *     tests
 *
 * together.
 * ============================================================================
 */


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The frontend AST owns the actual representation.
 *
 * Every accepted statement must preserve:
 *
 *     statement kind
 *     source span
 *     source ordering
 *     child relationships
 *     syntactic attributes
 *
 * The AST MUST remain domain-neutral at this layer.
 *
 * It MUST NOT introduce statement nodes whose identity exists only because of
 * a physical target.
 *
 * Forbidden examples:
 *
 *     CpuStatement
 *     GpuStatement
 *     FpgaStatement
 *     QpuStatement
 *     PhysicalQubitStatement
 *     FixedNodeStatement
 *
 * merely because the eventual semantic operation may be lowered to such a
 * target.
 *
 * The AST may contain semantic-domain constructs where the source language
 * actually defines them, but physical realization remains downstream.
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parser acceptance means:
 *
 *     "the source has valid structural statement syntax."
 *
 * It does NOT mean:
 *
 *     "the program is semantically valid."
 *
 * Semantic analysis must establish:
 *
 *     name resolution
 *     type validity
 *     ownership
 *     borrowing/lifetime rules
 *     effect legality
 *     capability requirements
 *     resource requirements
 *     contracts
 *     policies
 *     provenance
 *     control-flow legality
 *     domain compatibility
 *     quantum/classical boundaries
 *     hardware/software boundaries
 *     portability
 *
 * This separation is mandatory.
 * ============================================================================
 */


/*
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This file imposes no closed type universe.
 *
 * Statement operands ultimately use the canonical type/expression systems.
 *
 * Therefore statement syntax remains compatible with:
 *
 *     scalar types
 *     aggregates
 *     records
 *     variants
 *     functions
 *     tensors
 *     graphs
 *     probabilistic values
 *     uncertain values
 *     quantum-derived values
 *     distributed values
 *     hardware descriptions
 *     symbolic values
 *     future types
 *
 * Type validity is downstream.
 * ============================================================================
 */


/*
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * This file does not infer effects.
 *
 * A statement may semantically carry:
 *
 *     computation
 *     I/O
 *     network
 *     mutation
 *     randomness
 *     measurement
 *     learning
 *     adaptation
 *     distributed
 *     simulation
 *     foreign
 *     native
 *     reflection
 *     code generation
 *
 * depending on the concrete statement and its semantic operands.
 *
 * Effect checking belongs downstream.
 * ============================================================================
 */


/*
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capabilities are semantic requirements.
 *
 * This file does not resolve them.
 *
 * Examples:
 *
 *     capability("reasoning")
 *     capability("quantum.measurement")
 *     capability("tensor.compute")
 *     capability("distributed.compute")
 *     capability("network")
 *     capability("native.execute")
 *
 * Capability identifiers remain open-ended.
 *
 * No physical device catalog is embedded here.
 * ============================================================================
 */


/*
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This file introduces no resource limits.
 *
 * There is no grammar-level maximum for:
 *
 *     statements
 *     declarations
 *     expressions
 *     reasoning contexts
 *     resources
 *     tasks
 *     actors
 *     nodes
 *     devices
 *     qubits
 *     CPUs
 *     GPUs
 *     FPGAs
 *     accelerators
 *     tensor dimensions
 *     memory
 *     storage
 *
 * Repetition and recursion are represented structurally.
 *
 * Actual implementation limits are governed by available compiler/runtime
 * resources and explicit execution/resource policy, not by language constants.
 * ============================================================================
 */


/*
 * ============================================================================
 * CONTRACT / POLICY / PROVENANCE CONTRACT
 * ============================================================================
 *
 * Statement syntax can participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * and policies such as:
 *
 *     permissions
 *     prohibitions
 *     preferences
 *     fallbacks
 *     security policy
 *     execution policy
 *     resource policy
 *     adaptation policy
 *     determinism policy
 *
 * Provenance may record:
 *
 *     source
 *     transformation
 *     evidence
 *     decision
 *     derivation
 *     verification
 *
 * None of these semantic systems are implemented by this dispatcher.
 *
 * Their grammars remain independently owned.
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * This file does not enumerate quantum operations.
 *
 * It does not contain:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     SWAP
 *
 * as grammar-level statement alternatives.
 *
 * Quantum source syntax enters through:
 *
 *     domainStatement
 *
 * and is interpreted downstream.
 *
 * The canonical quantum pipeline remains:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     domain-neutral AST
 *       ->
 *     quantum semantic model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
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
 *     target realization
 *
 * This statement dispatcher MUST NOT create a second quantum IR.
 * ============================================================================
 */


/*
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * HDL and hardware statements enter through the domain composition layer.
 *
 * This file does not encode:
 *
 *     fixed bus widths
 *     fixed register widths
 *     fixed memory sizes
 *     fixed clock counts
 *     fixed pipeline depths
 *     fixed device counts
 *     physical addresses
 *     FPGA capacities
 *     ASIC capacities
 *     vendor topology
 *
 * Hardware intent is resolved downstream through:
 *
 *     capabilities
 *     resources
 *     constraints
 *     synthesis
 *     placement
 *     routing
 *     scheduling
 *     lowering
 *     target realization
 * ============================================================================
 */


/*
 * ============================================================================
 * CLASSICAL / HYBRID / DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * The same statement dispatcher must support source programs combining:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware/software co-design
 *     concurrency
 *     distributed computation
 *     AI/model computation
 *     data processing
 *     networking
 *     cryptography
 *     accelerators
 *     scientific computation
 *     embedded computation
 *     edge/cloud computation
 *     future computational domains
 *
 * No separate universal statement grammar is created for each target.
 * ============================================================================
 */


/*
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Statement syntax represents source-level intent.
 *
 * It does not encode a machine size.
 *
 * Therefore the same statement source can participate in realization on:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     heterogeneous systems
 *     future computational substrates
 *
 * subject to:
 *
 *     semantic correctness
 *     capability availability
 *     resource availability
 *     policy
 *     physical feasibility
 *
 * The grammar itself does not guarantee that every target can execute every
 * program.
 *
 * It guarantees that the source syntax does not artificially restrict the
 * realization space.
 * ============================================================================
 */


/*
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source token sequence
 *     grammar version
 *     parser configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     wall-clock time
 *     randomness
 *     filesystem state
 *     network state
 *     hardware state
 *     CPU availability
 *     GPU availability
 *     QPU availability
 *     scheduler state
 *     runtime state
 *     calibration state
 *
 * Identical source under identical parser configuration must produce
 * equivalent parse-tree structure.
 * ============================================================================
 */


/*
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level diagnostics include:
 *
 *     unexpected token
 *     missing statement terminator
 *     malformed declaration
 *     malformed assignment
 *     malformed assertion
 *     malformed control-flow construct
 *     malformed concurrency construct
 *     malformed effect statement
 *     malformed resource statement
 *     malformed reasoning statement
 *     malformed domain statement
 *     malformed block
 *     malformed expression statement
 *
 * Semantic diagnostics do NOT belong here.
 *
 * Examples:
 *
 *     unknown name
 *     invalid type
 *     invalid capability
 *     unavailable resource
 *     contract violation
 *     policy violation
 *     ownership violation
 *     invalid quantum operation
 *     invalid hardware requirement
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     model execution
 *     reasoning-engine execution
 *     knowledge-store access
 *     device discovery
 *     hardware inspection
 *     QPU access
 *     simulator invocation
 *     foreign-function invocation
 *     generated-code execution
 *
 * Parsing is purely structural.
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file preserves the existing universal rule name:
 *
 *     statement
 *
 * and the existing generic statement-position rules:
 *
 *     emptyStatement
 *     expressionStatement
 *
 * The following statement-family ownership is preserved:
 *
 *     declarations      -> Declarations
 *     assignments       -> Assignments
 *     assertions        -> AssertionsParser
 *     control flow      -> ControlFlow
 *     concurrency      -> ConcurrencyStatements
 *     effects           -> EffectStatements
 *     resources         -> ResourceStatements
 *     reasoning         -> ReasonStatements
 *     domains           -> Domains
 *     unsafe regions    -> UnsafeStatementsParser
 *     expressions       -> Expressions
 *     blocks            -> ZamaniCoreBlocks
 *
 * This is an architectural consolidation, not a source-language redesign.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * IMPORTANT COMPATIBILITY CLEANUP
 * ============================================================================
 *
 * The following imports MUST NOT remain in this file as independent imports:
 *
 *     ConditionalsParser
 *     Loops
 *     PatternMatching
 *     BreakStatements
 *     ContinueStatements
 *     ReturnStatements
 *     ExceptionsParser
 *
 * They are already composed by:
 *
 *     ControlFlow
 *
 * Importing them again here creates competing composition paths and makes the
 * ownership graph harder to reason about.
 *
 * Likewise:
 *
 *     Infer
 *     Deduce
 *
 * must not be imported alongside:
 *
 *     ReasonStatements
 *
 * when they parse the same source forms.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * LEGACY GRAMMAR INTEGRATION
 * ============================================================================
 *
 * The historical monolithic grammar may continue to exist as:
 *
 *     reference
 *     migration source
 *     compatibility documentation
 *     conformance reference
 *
 * but it MUST NOT create a second authoritative parser path.
 *
 * The production parser must converge on this modular ownership graph.
 * ============================================================================
 */


/*
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation.
 *
 * Generated parser/frontend integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST use safe Rust only.
 *
 * This grammar requires no:
 *
 *     unsafe blocks
 *     unsafe functions
 *     unsafe traits
 *     unsafe extern blocks
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden language-level limits include:
 *
 *     MAX_STATEMENTS
 *     MAX_DECLARATIONS
 *     MAX_EXPRESSIONS
 *     MAX_REASONING_STEPS
 *     MAX_CONTEXTS
 *     MAX_TASKS
 *     MAX_ACTORS
 *     MAX_CHANNELS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *
 * This file contains none of those limits.
 *
 * It also contains no:
 *
 *     physical device identifiers
 *     physical addresses
 *     fixed hardware topology
 *     vendor-specific hardware selection
 *     fixed quantum topology
 *     fixed accelerator count
 *
 * The grammar uses composition and repetition rather than finite machine
 * capacity.
 * ============================================================================
 */


/*
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The statement grammar must remain structurally valid for increasingly large
 * source programs.
 *
 * Scaling dimensions include:
 *
 *     statement count
 *     declaration count
 *     expression size
 *     block size
 *     nesting
 *     reasoning context size
 *     resource requirement size
 *     concurrency structure
 *     domain composition
 *     quantum operation count
 *     hardware intent size
 *     distributed structure
 *
 * No universal finite language ceiling is encoded here.
 *
 * Practical parser/compiler limits may exist as implementation resource
 * controls. Such limits MUST NOT become language semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * -------------
 *
 * The parser test suite MUST cover at least:
 *
 *     ;
 *
 *     expression;
 *
 *     declaration;
 *
 *     assignment;
 *
 *     assertion;
 *
 *     if/else;
 *
 *     loops;
 *
 *     match;
 *
 *     break;
 *
 *     continue;
 *
 *     return;
 *
 *     throw/try/catch/finally;
 *
 *     concurrency;
 *
 *     effects;
 *
 *     resources;
 *
 *     infer;
 *
 *     deduce;
 *
 *     reason;
 *
 *     domain statements;
 *
 *     unsafe regions;
 *
 *     blocks;
 *
 *     mixed-domain statements.
 *
 * REASONING POSITIVES
 * -------------------
 *
 *     infer conclusion;
 *
 *     deduce result;
 *
 *     reason decision;
 *
 *     infer conclusion from evidence;
 *
 *     deduce result from premises;
 *
 *     reason decision from observation;
 *
 *     infer result from evidence with (confidence);
 *
 *     reason decision with (policy, provenance);
 *
 *     deduce result from quantum_result with (confidence, policy);
 *
 * CROSS-DOMAIN POSITIVES
 * ----------------------
 *
 * Test combinations including:
 *
 *     classical + quantum
 *     quantum + classical control
 *     quantum + reasoning
 *     reasoning + learning
 *     reasoning + resource requirements
 *     reasoning + provenance
 *     AI/model + quantum
 *     AI/model + accelerator
 *     HDL + resource requirements
 *     hardware + contracts
 *     distributed + networking
 *     simulation + quantum
 *     simulation + HDL
 *     FFI + effects
 *     metaprogramming + contracts
 *
 * NEGATIVE TESTS
 * -------------
 *
 * Test malformed:
 *
 *     declarations
 *     assignments
 *     assertions
 *     control flow
 *     concurrency
 *     effects
 *     resources
 *     reasoning
 *     domain statements
 *     blocks
 *     expressions
 *
 * Reasoning negatives include:
 *
 *     infer;
 *     deduce;
 *     reason;
 *     infer from source;
 *     deduce from source;
 *     reason from source;
 *     reason target with ();
 *     reason target with (a,);
 *     reason target with (a) from source;
 *     reason target from;
 *     reason target with;
 *
 * SEMANTIC NEGATIVES
 * ------------------
 *
 * These must be tested downstream rather than encoded into this grammar:
 *
 *     unknown name
 *     invalid type
 *     invalid capability
 *     unsatisfied resource requirement
 *     forbidden effect
 *     invalid ownership
 *     invalid contract
 *     policy violation
 *     invalid quantum/classical boundary
 *     impossible hardware requirement
 *
 * BOUNDARY TESTS
 * --------------
 *
 * Test:
 *
 *     empty programs
 *     one statement
 *     large statement sequences
 *     deeply nested blocks
 *     large expressions
 *     large reasoning contexts
 *     long control-flow chains
 *     many concurrent constructs
 *     large domain statements
 *     mixed quantum/classical programs
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Generated tests should progressively increase source size and structural
 * complexity without changing this grammar.
 *
 * They must demonstrate that no machine-capacity constant appears in the
 * parser architecture.
 *
 * DETERMINISM TESTS
 * ----------------
 *
 * Identical:
 *
 *     source
 *     lexer version
 *     grammar version
 *     parser configuration
 *
 * must produce equivalent parse-tree structure.
 *
 * ROUND-TRIP TESTS
 * ----------------
 *
 * Where formatter support exists:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     formatter
 *       ->
 *     parser
 *
 * must preserve statement structure and semantics.
 *
 * ============================================================================
 * AST / SEMANTIC INTEGRATION TEST
 * ============================================================================
 *
 * Every statement alternative must:
 *
 *     1. parse;
 *     2. map to exactly one canonical AST representation;
 *     3. preserve source spans;
 *     4. preserve source order;
 *     5. preserve child relationships;
 *     6. enter semantic analysis;
 *     7. participate in type/effect/capability/resource analysis as needed;
 *     8. participate in contracts/policies/provenance as needed;
 *     9. lower to the appropriate canonical semantic representation.
 *
 * No statement may jump directly from grammar to hardware.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] Statements is the sole universal `statement` owner.
 *
 * [ ] ZamaniParser imports Statements.
 *
 * [ ] Declarations are composed through Declarations.
 *
 * [ ] Assignments are composed through Assignments.
 *
 * [ ] Assertions are composed through AssertionsParser.
 *
 * [ ] Control flow is composed through ControlFlow.
 *
 * [ ] Concurrency is composed through ConcurrencyStatements.
 *
 * [ ] Effects are composed through EffectStatements.
 *
 * [ ] Resources are composed through ResourceStatements.
 *
 * [ ] Reasoning is composed through ReasonStatements.
 *
 * [ ] Domains are composed through Domains.
 *
 * [ ] Unsafe regions are composed through UnsafeStatementsParser.
 *
 * [ ] Expressions are composed through Expressions.
 *
 * [ ] Blocks are composed through ZamaniCoreBlocks.
 *
 * [ ] No individual control-flow leaf is imported here.
 *
 * [ ] No separate infer grammar path is imported here.
 *
 * [ ] No separate deduce grammar path is imported here.
 *
 * [ ] No concrete domain grammar is duplicated here.
 *
 * [ ] No lexer rules are defined here.
 *
 * [ ] No semantic actions exist.
 *
 * [ ] No semantic predicates exist.
 *
 * [ ] No runtime execution exists.
 *
 * [ ] No hardware discovery exists.
 *
 * [ ] No backend selection exists.
 *
 * [ ] No physical topology exists.
 *
 * [ ] No quantum gate catalog exists.
 *
 * [ ] No QEC implementation exists.
 *
 * [ ] No ZQN implementation exists.
 *
 * [ ] No routing implementation exists.
 *
 * [ ] No scheduling implementation exists.
 *
 * [ ] No machine-capacity constant exists.
 *
 * [ ] No artificial statement-count limit exists.
 *
 * [ ] AST mapping is defined.
 *
 * [ ] Semantic mapping is defined.
 *
 * [ ] Effect integration is defined.
 *
 * [ ] Capability integration is defined.
 *
 * [ ] Resource integration is defined.
 *
 * [ ] Contract integration is defined.
 *
 * [ ] Policy integration is defined.
 *
 * [ ] Provenance integration is defined.
 *
 * [ ] Quantum statements ultimately reach quantum::ir.
 *
 * [ ] Rust 1.97 integration succeeds.
 *
 * [ ] Rust 1.97.1 integration succeeds.
 *
 * [ ] Generated/consuming implementation remains safe Rust.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Round-trip tests pass.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This file describes neither:
 *
 *     machine size
 *     hardware topology
 *     physical device identity
 *     vendor backend
 *     quantum hardware capacity
 *     accelerator count
 *     distributed-node count
 *
 * It describes only how source-level statements enter the universal parser.
 *
 * Therefore:
 *
 *     SOURCE
 *       |
 *       v
 *     STATEMENT COMPOSITION
 *       |
 *       v
 *     DOMAIN-NEUTRAL AST
 *       |
 *       v
 *     SEMANTIC ANALYSIS
 *       |
 *       v
 *     CAPABILITY / RESOURCE / POLICY NEGOTIATION
 *       |
 *       v
 *     CANONICAL IR
 *       |
 *       +--> classical
 *       +--> quantum::ir
 *       +--> HDL/hardware
 *       +--> distributed
 *       +--> accelerator
 *       +--> future domains
 *       |
 *       v
 *     OPTIMIZATION / LOWERING
 *       |
 *       v
 *     ROUTING / SCHEDULING / RESILIENCE
 *       |
 *       v
 *     TARGET REALIZATION
 *
 * The same source-level statement grammar can therefore participate in:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * subject to actual semantic validity, implementation capabilities, available
 * resources, policy and physical feasibility.
 *
 * ============================================================================
 */