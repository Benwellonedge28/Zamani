/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/core/source-unit.g4
 *
 * GRAMMAR
 * -------
 * ZamaniSourceUnit
 *
 * STATUS
 * ------
 * Canonical production source-file/source-unit composition grammar.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the universal source boundary of Zamani.
 *
 * It defines:
 *
 *     sourceFile
 *     sourceUnit
 *     sourceItem
 *
 * The grammar is intentionally domain-neutral.
 *
 * It does not distinguish between:
 *
 *     classical programs
 *     quantum programs
 *     HDL programs
 *     AI programs
 *     distributed programs
 *     accelerator programs
 *     hardware programs
 *     hybrid programs
 *
 * All of those are Zamani programs and enter through the same source-unit
 * boundary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     sourceFile
 *     sourceUnit
 *     sourceItem
 *
 * It owns:
 *
 *     - complete source-file composition;
 *     - source-unit composition;
 *     - source-item ordering;
 *     - source-level EOF ownership;
 *     - the declaration/statement boundary.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     - lexical tokens;
 *     - keywords;
 *     - operators;
 *     - identifiers;
 *     - literals;
 *     - comments;
 *     - declarations;
 *     - statements;
 *     - expressions;
 *     - types;
 *     - functions;
 *     - modules;
 *     - effects;
 *     - resources;
 *     - capabilities;
 *     - contracts;
 *     - policies;
 *     - provenance;
 *     - AI semantics;
 *     - quantum operations;
 *     - quantum routing;
 *     - QEC;
 *     - HDL semantics;
 *     - hardware realization;
 *     - networking;
 *     - distributed execution;
 *     - target selection;
 *     - optimization;
 *     - scheduling;
 *     - lowering;
 *     - runtime execution;
 *     - AST construction;
 *     - semantic analysis;
 *     - IR construction.
 *
 * ============================================================================
 * SINGLE AUTHORITY CONTRACT
 * ============================================================================
 *
 * The production grammar must have exactly one effective owner for each of:
 *
 *     sourceFile
 *     sourceUnit
 *     sourceItem
 *
 * Therefore:
 *
 *     grammar/core/source-unit.g4
 *
 * is the canonical owner.
 *
 * Other grammars may IMPORT these rules but must not redefine them.
 *
 * In particular, canonical parser composition must not contain competing
 * implementations of:
 *
 *     sourceUnit
 *     sourceFile
 *     sourceItem
 *
 * under grammar/antlr/ or grammar/Zamani.g4.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                     canonical Zamani lexer
 *                              |
 *                              v
 *                         parser entry
 *                              |
 *                              v
 *                          sourceFile
 *                              |
 *                              v
 *                          sourceUnit
 *                              |
 *                              v
 *                          sourceItem*
 *                              |
 *                    +---------+---------+
 *                    |                   |
 *                    v                   v
 *               declaration          statement
 *                    |                   |
 *                    +---------+---------+
 *                              |
 *                              v
 *                    domain-neutral AST
 *                              |
 *                              v
 *                     structural validation
 *                              |
 *                              v
 *                      semantic analysis
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      classical          quantum::ir        HDL/hardware
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                         lowering
 *                              |
 *                   routing / scheduling
 *                              |
 *                    resilience / recovery
 *                              |
 *                           QEC / ZQN
 *                              |
 *                              v
 *                            HAL
 *                              |
 *                              v
 *                       target realization
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Zamani is designed for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 *     POCO-REAF
 *
 * The source-unit grammar therefore imposes no language-level limits on:
 *
 *     source items
 *     declarations
 *     statements
 *     modules
 *     functions
 *     types
 *     expressions
 *     namespaces
 *     imports
 *     nesting
 *     processes
 *     tasks
 *     actors
 *     channels
 *     nodes
 *     devices
 *     accelerators
 *     qubits
 *     quantum registers
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     memory
 *     storage
 *     tensor dimensions
 *     tensor rank
 *     network size
 *     topology size
 *     pipeline depth
 *     timeline length
 *
 * There are deliberately no universal constants defining those capacities.
 *
 * Physical and implementation limits belong to:
 *
 *     compiler resources
 *     operating-system resources
 *     runtime resources
 *     deployment policies
 *     target capabilities
 *     available hardware
 *
 * They must not become grammar ceilings.
 *
 * ============================================================================
 * SOURCE FILE VS SOURCE UNIT
 * ============================================================================
 *
 * sourceUnit
 * ----------
 *
 * A reusable sequence of source items.
 *
 * It MUST NOT consume EOF.
 *
 * sourceFile
 * ----------
 *
 * A complete externally parseable source file.
 *
 * It owns exactly one EOF boundary.
 *
 * Therefore:
 *
 *     sourceFile
 *         : sourceUnit EOF
 *         ;
 *
 * while:
 *
 *     sourceUnit
 *         : sourceItem*
 *         ;
 *
 * This separation is intentional.
 *
 * It allows sourceUnit to be composed by another grammar without causing:
 *
 *     EOF EOF
 *
 * or requiring callers to strip an EOF rule.
 *
 * ============================================================================
 * EMPTY SOURCE
 * ============================================================================
 *
 * An empty source file is syntactically valid:
 *
 *     sourceFile
 *         -> sourceUnit EOF
 *
 * with:
 *
 *     sourceUnit
 *         -> zero sourceItem
 *
 * Whether an empty program is semantically useful is NOT decided here.
 *
 * A compilation profile may impose additional semantic requirements.
 *
 * The universal grammar must not invent such requirements.
 *
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * Source items are ordered.
 *
 * The parser therefore preserves:
 *
 *     source item A
 *     source item B
 *     source item C
 *
 * in that order.
 *
 * This grammar does not reorder declarations or statements.
 *
 * Any later ordering for:
 *
 *     initialization
 *     dependencies
 *     optimization
 *     scheduling
 *     parallel execution
 *     distributed execution
 *
 * belongs to semantic analysis and compilation.
 *
 * ============================================================================
 * SOURCE ITEM
 * ============================================================================
 *
 * A source item is exactly one canonical:
 *
 *     declaration
 *
 * or:
 *
 *     statement
 *
 * This grammar does not enumerate domains.
 *
 * It therefore does NOT contain:
 *
 *     quantumItem
 *     classicalItem
 *     hdlItem
 *     gpuItem
 *     fpgaItem
 *     qpuItem
 *     aiItem
 *     networkItem
 *     distributedItem
 *     acceleratorItem
 *
 * Adding a new computational domain therefore does not require changing this
 * file.
 *
 * ============================================================================
 * DECLARATION INTEGRATION
 * ============================================================================
 *
 * Declaration syntax is owned by:
 *
 *     grammar/declarations/declarations.g4
 *
 * whose parser grammar is:
 *
 *     ZamaniDeclarations
 *
 * This file consumes the exported:
 *
 *     declaration
 *
 * rule.
 *
 * It must never copy individual declaration alternatives here.
 *
 * ============================================================================
 * STATEMENT INTEGRATION
 * ============================================================================
 *
 * Statement syntax is owned by:
 *
 *     grammar/statements/statements.g4
 *
 * whose parser grammar is:
 *
 *     Statements
 *
 * This file consumes the exported:
 *
 *     statement
 *
 * rule.
 *
 * It must never copy individual statement alternatives here.
 *
 * ============================================================================
 * IMPORT GRAPH
 * ============================================================================
 *
 * The intended dependency direction is:
 *
 *     source-unit
 *          |
 *          +--> declarations
 *          |
 *          +--> statements
 *
 * It must NOT be:
 *
 *     declarations
 *          |
 *          +--> source-unit
 *
 * or:
 *
 *     statements
 *          |
 *          +--> source-unit
 *
 * This keeps the source boundary above feature grammars and prevents cyclic
 * grammar ownership.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * All parser grammars in the production composition consume the canonical
 * Zamani lexer vocabulary:
 *
 *     ZamaniLexer
 *
 * The lexical authority remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         |
 *         v
 *     grammar/lexer/lexer.g4
 *         |
 *         v
 *     grammar/lexer/tokens.g4
 *
 * This file does not define lexer rules.
 *
 * ============================================================================
 * DOMAIN INTEGRATION
 * ============================================================================
 *
 * Classical computation enters through declarations/statements/expressions
 * owned by the corresponding classical grammar components.
 *
 * Quantum computation enters through the quantum grammar components.
 *
 * HDL and hardware/software co-design enter through their domain grammar
 * components.
 *
 * AI, knowledge, reasoning, learning, adaptation, uncertainty, agents,
 * provenance, policies and related capabilities enter through their owning
 * grammar components.
 *
 * Distributed and networking constructs enter through their respective
 * grammar components.
 *
 * None of those domains changes this source boundary.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * This file MUST NOT define:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     SWAP
 *
 * or any other finite gate catalogue.
 *
 * It MUST NOT define:
 *
 *     physical qubit identifiers
 *     physical topology
 *     routing
 *     scheduling
 *     calibration
 *     QEC distance
 *     error rates
 *     device-specific constraints
 *
 * Quantum processing remains:
 *
 *     source
 *       ->
 *     AST
 *       ->
 *     semantic quantum model
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
 *     target
 *
 * `quantum::ir` remains the canonical quantum IR boundary.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * This file MUST NOT encode universal physical hardware assumptions such as:
 *
 *     fixed register width
 *     fixed bus width
 *     fixed memory size
 *     fixed FPGA dimensions
 *     fixed ASIC dimensions
 *     fixed accelerator count
 *     fixed device count
 *     fixed clock count
 *     fixed pipeline depth
 *
 * Hardware intent and realization belong downstream.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Source programs may eventually contain constructs such as:
 *
 *     requires capability(...)
 *     requires resource(...)
 *     requires memory >= expression
 *     requires qubits >= expression
 *     requires topology(...)
 *     prefer ...
 *     constrain ...
 *     allow ...
 *     forbid ...
 *
 * This file does not interpret any of those constructs.
 *
 * Their grammar belongs to the appropriate resource, policy, validation, or
 * statement/expression owner.
 *
 * Their meaning belongs to semantic analysis.
 *
 * Their realization belongs to compilation/execution planning.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * The source-unit boundary is effect-neutral.
 *
 * Effects such as:
 *
 *     IO
 *     network
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     quantum
 *     learning
 *     adaptation
 *     reflection
 *     code generation
 *     simulation
 *
 * are handled downstream.
 *
 * ============================================================================
 * CONTRACT / POLICY / PROVENANCE
 * ============================================================================
 *
 * Constructs such as:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *     evidence
 *     provenance
 *     policy
 *
 * do not belong directly in this source-unit grammar.
 *
 * They are integrated through their canonical feature grammars.
 *
 * The source boundary simply permits them to occur wherever the owning
 * declaration or statement grammar permits them.
 *
 * ============================================================================
 * AI / KNOWLEDGE / REASONING INTEGRATION
 * ============================================================================
 *
 * Generic computational capabilities such as:
 *
 *     infer
 *     deduce
 *     reason
 *     assert
 *     retract
 *     query
 *     learn
 *     adapt
 *     explain
 *     evidence
 *     uncertainty
 *     provenance
 *
 * do not create a second program root.
 *
 * They enter through declarations, statements and expressions.
 *
 * This keeps the language universal rather than creating an AI-specific
 * source language.
 *
 * ============================================================================
 * MULTI-AGENT INTEGRATION
 * ============================================================================
 *
 * Agent syntax and actor semantics remain separate concerns.
 *
 * AI-agent semantics integrate with the existing concurrency model:
 *
 *     agent
 *       ->
 *     actor
 *       ->
 *     message
 *       ->
 *     channel
 *       ->
 *     scheduler/runtime
 *
 * This source-unit grammar does not create a second actor system.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * FFI, ABI, SQL, JSON, XML and other external representations do not alter
 * the universal source-file boundary.
 *
 * They enter through:
 *
 *     interoperability/
 *     dialects/
 *     data/
 *
 * as appropriate.
 *
 * External calls remain subject to downstream:
 *
 *     type checking
 *     effect checking
 *     capability checking
 *     security policy
 *     provenance
 *     ABI validation
 *
 * ============================================================================
 * METAPROGRAMMING
 * ============================================================================
 *
 * Reflection, introspection, compile-time execution, quotation, syntax-tree
 * manipulation and code generation do not alter this source boundary.
 *
 * They remain controlled by:
 *
 *     grammar/metaprogramming/
 *     grammar/macros/
 *
 * and their associated semantic/effect/capability rules.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces ANTLR parse-tree structure only.
 *
 * The frontend AST adapter maps:
 *
 *     sourceFile
 *         ->
 *     Program/source-file representation
 *
 *     sourceUnit
 *         ->
 *     ordered source-unit representation
 *
 *     sourceItem
 *         ->
 *     declaration AST
 *
 *     OR
 *
 *     statement AST
 *
 * The AST must remain domain-neutral.
 *
 * It must not require physical target information such as:
 *
 *     CPU count
 *     GPU count
 *     physical qubit map
 *     FPGA coordinates
 *     node allocation
 *     routing plan
 *     schedule
 *     calibration
 *
 * merely because downstream compilation may eventually create those artifacts.
 *
 * ============================================================================
 * SOURCE LOCATION CONTRACT
 * ============================================================================
 *
 * The frontend adapter should preserve:
 *
 *     source file identity
 *     source span
 *     start position
 *     end position
 *     source ordering
 *
 * for every source item.
 *
 * This is required for:
 *
 *     diagnostics
 *     IDE tooling
 *     provenance
 *     refactoring
 *     debugging
 *     deterministic builds
 *     source-to-IR mapping
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This file performs NO semantic validation.
 *
 * It does not determine:
 *
 *     whether names exist;
 *     whether types match;
 *     whether effects are permitted;
 *     whether capabilities are available;
 *     whether resource requirements can be satisfied;
 *     whether policies permit execution;
 *     whether a quantum operation is realizable;
 *     whether hardware can implement a program;
 *     whether a distributed topology is feasible.
 *
 * Those are downstream concerns.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file creates no IR.
 *
 * The required path is:
 *
 *     source
 *       ->
 *     parse tree
 *       ->
 *     domain-neutral AST
 *       ->
 *     structural validation
 *       ->
 *     semantic model
 *       ->
 *     canonical/domain IR
 *
 * Quantum specifically:
 *
 *     AST
 *       ->
 *     quantum semantic model
 *       ->
 *     quantum::ir
 *
 * This file must never become an alternate IR boundary.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     input token stream
 *     grammar version
 *     parser configuration
 *
 * It must not depend on:
 *
 *     hardware
 *     network availability
 *     filesystem state
 *     wall-clock time
 *     random state
 *     environment variables
 *     runtime state
 *     target discovery
 *
 * Therefore this grammar contains:
 *
 *     no actions
 *     no semantic predicates
 *     no embedded Rust
 *     no external calls
 *     no runtime callbacks
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing is non-executing.
 *
 * This grammar must never:
 *
 *     execute commands
 *     access files
 *     access network services
 *     inspect hardware
 *     execute foreign functions
 *     invoke native code
 *     mutate external state
 *
 * A parsed construct is source representation until explicitly interpreted
 * downstream.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to parser diagnostics.
 *
 * Examples:
 *
 *     unexpected source token
 *     malformed declaration
 *     malformed statement
 *     unexpected EOF
 *     invalid source-item dispatch
 *
 * Semantic errors are deliberately downstream.
 *
 * Examples:
 *
 *     unknown symbol
 *     invalid type
 *     unavailable capability
 *     unsatisfied resource requirement
 *     forbidden effect
 *     invalid policy
 *     invalid quantum operation
 *     impossible target realization
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The public filename remains:
 *
 *     grammar/core/source-unit.g4
 *
 * The public `sourceUnit` concept remains.
 *
 * The important production correction is:
 *
 *     sourceUnit
 *
 * does NOT consume EOF.
 *
 * Instead:
 *
 *     sourceFile
 *
 * consumes exactly one EOF.
 *
 * This permits:
 *
 *     program
 *         ->
 *     sourceFile
 *
 * while also allowing a reusable source-unit component where required.
 *
 * ============================================================================
 * BUILD CONTRACT
 * ============================================================================
 *
 * ANTLR grammar imports use grammar names rather than filesystem paths.
 *
 * The build system must therefore make the directories containing:
 *
 *     ZamaniDeclarations
 *     Statements
 *
 * available on the ANTLR grammar library path.
 *
 * This file must not embed filesystem paths in grammar syntax.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * The generated parser is consumed by the Zamani Rust frontend.
 *
 * Repository compatibility target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * with:
 *
 *     Rust 2021 edition
 *     safe Rust only
 *     no unsafe Rust
 *
 * This grammar contains no Rust implementation code.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     ZamaniLexer
 *     ZamaniDeclarations
 *     Statements
 *
 * EXPORTS:
 *
 *     sourceFile
 *     sourceUnit
 *     sourceItem
 *
 * CONSUMED_BY:
 *
 *     ZamaniProgram
 *     canonical Zamani parser composition
 *
 * AST_OWNER:
 *
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     frontend semantic-analysis layer
 *
 * IR_OWNER:
 *
 *     canonical semantic/domain IR layers
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/parser/
 *     grammar/tests/ast/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *     grammar/tests/compatibility/
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 * ============================================================================
 * INTEGRATION WITH grammar/core/program.g4
 * ============================================================================
 *
 * The program boundary must delegate to sourceFile:
 *
 *     program
 *         : sourceFile
 *         ;
 *
 * It must NOT write:
 *
 *     program
 *         : sourceUnit EOF
 *         ;
 *
 * because sourceFile is the canonical complete-file boundary.
 *
 * This keeps EOF ownership in exactly one place.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/antlr/ZamaniParser.g4
 * ============================================================================
 *
 * ZamaniParser must import the canonical program boundary:
 *
 *     ZamaniProgram
 *
 * and must not define another sourceFile/sourceUnit/sourceItem hierarchy.
 *
 * If legacy source rules remain in ZamaniParser.g4, they must be removed from
 * the canonical parser composition rather than being treated as a second
 * authority.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/Zamani.g4
 * ============================================================================
 *
 * grammar/Zamani.g4 remains the final combined grammar.
 *
 * It must not redefine:
 *
 *     sourceFile
 *     sourceUnit
 *     sourceItem
 *
 * It inherits the canonical definitions through the parser grammar imports.
 *
 * ============================================================================
 * INTEGRATION WITH DECLARATIONS
 * ============================================================================
 *
 * Required exported rule:
 *
 *     declaration
 *
 * from:
 *
 *     ZamaniDeclarations
 *
 * The declaration grammar may internally compose:
 *
 *     modules
 *     functions
 *     types
 *     variables
 *     constants
 *     quantum declarations
 *     HDL declarations
 *     resources
 *     capabilities
 *     contracts
 *     policies
 *     effects
 *     AI constructs
 *
 * without modifying this source-unit file.
 *
 * ============================================================================
 * INTEGRATION WITH STATEMENTS
 * ============================================================================
 *
 * Required exported rule:
 *
 *     statement
 *
 * from:
 *
 *     Statements
 *
 * The statement grammar may internally compose:
 *
 *     expressions
 *     control flow
 *     concurrency
 *     quantum operations
 *     measurement
 *     learning
 *     adaptation
 *     reasoning
 *     knowledge operations
 *     queries
 *     simulation
 *     policies
 *     contracts
 *     resource operations
 *
 * without modifying this source-unit file.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses:
 *
 *     sourceItem*
 *
 * rather than a finite list of source-item positions.
 *
 * There is no grammar-level cardinality ceiling.
 *
 * "Infinity" is interpreted as:
 *
 *     no artificial language-imposed upper bound;
 *
 * actual execution remains bounded only by available implementation resources
 * and explicitly selected resource policies.
 *
 * This distinction is fundamental to POCO-REAF.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
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
 * It must also avoid hidden fixed assumptions such as:
 *
 *     exactly N source items
 *     exactly N declarations
 *     exactly N statements
 *     exactly N modules
 *
 * No such limits are present.
 *
 * ============================================================================
 * TEST MATRIX
 * ============================================================================
 *
 * LEXICAL / PARSER
 * ----------------
 *
 *     - empty source file
 *     - one declaration
 *     - one statement
 *     - declaration sequence
 *     - statement sequence
 *     - mixed declaration/statement sequence
 *
 * DOMAIN
 * ------
 *
 *     - classical source
 *     - quantum source
 *     - hybrid source
 *     - HDL source
 *     - hardware/software co-design source
 *     - AI source
 *     - data source
 *     - distributed source
 *     - networking source
 *     - mixed-domain source
 *
 * NEGATIVE
 * --------
 *
 *     - malformed declaration
 *     - malformed statement
 *     - incomplete source item
 *     - unexpected token
 *     - unexpected EOF
 *
 * BOUNDARY
 * --------
 *
 *     - zero source items
 *     - one source item
 *     - large source-item sequence
 *     - deeply nested delegated construct
 *     - mixed domains in one source file
 *
 * SCALABILITY
 * -----------
 *
 *     - increasingly large source units
 *     - large declaration sequences
 *     - large statement sequences
 *     - mixed-domain source units
 *
 * DETERMINISM
 * -----------
 *
 *     identical token stream
 *         ->
 *     equivalent parse structure
 *
 * COMPATIBILITY
 * ------------
 *
 *     - existing source syntax remains parseable
 *     - EOF occurs exactly once
 *     - no legacy source-root rule becomes a competing authority
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] sourceFile is the canonical complete-file rule.
 *     [ ] sourceUnit is reusable and does not consume EOF.
 *     [ ] sourceItem is the canonical source-item dispatcher.
 *     [ ] declaration is imported from the declaration authority.
 *     [ ] statement is imported from the statement authority.
 *     [ ] no declaration alternatives are duplicated here.
 *     [ ] no statement alternatives are duplicated here.
 *     [ ] no domain alternatives are duplicated here.
 *     [ ] no expression grammar is duplicated here.
 *     [ ] no type grammar is duplicated here.
 *     [ ] no module grammar is duplicated here.
 *     [ ] no lexer rules are duplicated here.
 *     [ ] no EOF is consumed by sourceUnit.
 *     [ ] sourceFile consumes exactly one EOF.
 *     [ ] program delegates to sourceFile.
 *     [ ] canonical parser imports this component.
 *     [ ] canonical root does not redefine this component.
 *     [ ] AST mapping is predetermined.
 *     [ ] semantic mapping is predetermined.
 *     [ ] IR mapping is predetermined.
 *     [ ] quantum::ir remains downstream and canonical.
 *     [ ] no hardware capacity is hard-coded.
 *     [ ] no language capacity is hard-coded.
 *     [ ] parsing is deterministic.
 *     [ ] parsing has no external side effects.
 *     [ ] positive tests exist.
 *     [ ] negative tests exist.
 *     [ ] boundary tests exist.
 *     [ ] scalability tests exist.
 *     [ ] cross-domain tests exist.
 *     [ ] compatibility tests exist.
 *
 * ============================================================================
 */

parser grammar ZamaniSourceUnit;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * AUTHORITATIVE COMPOSITION IMPORTS
 * ============================================================================
 *
 * Declaration and statement syntax remain independently maintained.
 *
 * These grammar names are resolved through ANTLR's grammar library path.
 * They are not filesystem paths.
 */
import
    ZamaniDeclarations,
    Statements
;

/*
 * ============================================================================
 * COMPLETE SOURCE FILE
 * ============================================================================
 *
 * This is the complete-file boundary.
 *
 * EOF is owned here and nowhere else in the source-unit hierarchy.
 */
sourceFile
    : sourceUnit EOF
    ;

/*
 * ============================================================================
 * SOURCE UNIT
 * ============================================================================
 *
 * Ordered sequence of zero or more source items.
 *
 * No finite cardinality is encoded.
 */
sourceUnit
    : sourceItem*
    ;

/*
 * ============================================================================
 * SOURCE ITEM
 * ============================================================================
 *
 * One canonical declaration or statement.
 *
 * Domain-specific constructs enter through those canonical dispatchers.
 */
sourceItem
    : declaration
    | statement
    ;