/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/blocks.g4
 *
 * Status:
 *     Canonical production block-syntax grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     target-specific code, unsafe code, I/O, filesystem access, networking,
 *     device discovery, runtime execution, or mutable global state.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE SYNTAX OWNER for Zamani block expressions.
 *
 * A Zamani block is an ordered source-level region:
 *
 *     {
 *         element
 *         element
 *         ...
 *     }
 *
 * The block may contain the language's canonical statement forms. An
 * expression statement is itself a statement, so value-producing expressions
 * can occur as the final child where the semantic model permits a block value.
 *
 * This grammar intentionally does NOT create a separate:
 *
 *     block statement
 *     block expression statement
 *     block-body grammar
 *     expression hierarchy
 *     statement hierarchy
 *
 * The canonical statement and expression rules remain owned by their
 * respective grammar components.
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - blockExpression
 *     - blockElement
 *     - the `{ ... }` delimiter structure
 *     - ordered repetition of block elements
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - expressions
 *     - expression precedence
 *     - statements
 *     - declarations
 *     - functions
 *     - control-flow semantics
 *     - lexical tokens
 *     - identifiers
 *     - types
 *     - effects
 *     - capabilities
 *     - resources
 *     - memory semantics
 *     - ownership
 *     - borrowing
 *     - quantum semantics
 *     - quantum IR
 *     - QEC
 *     - ZQN
 *     - HDL semantics
 *     - hardware discovery
 *     - topology
 *     - routing
 *     - scheduling
 *     - optimization
 *     - runtime execution
 *     - backend selection
 *     - machine limits
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * Lexer
 *   |
 *   v
 * Parser
 *   |
 *   +--> blockExpression          <-- THIS FILE
 *   |
 *   v
 * Native Zamani AST
 *   |
 *   v
 * Structural validation
 *   |
 *   v
 * Semantic analysis
 *   |
 *   +--> classical semantics
 *   +--> quantum semantics
 *   +--> HDL semantics
 *   +--> distributed semantics
 *   +--> accelerator semantics
 *   +--> future domain semantics
 *   |
 *   v
 * Canonical semantic representation
 *   |
 *   +--> classical IR
 *   +--> quantum::ir
 *   +--> HDL/hardware representation
 *   +--> control/data representation
 *   +--> resource representation
 *   |
 *   v
 * Optimization
 *   |
 *   v
 * Routing / scheduling / lowering
 *   |
 *   v
 * Target realization
 *   |
 *   v
 * Runtime / hardware
 *
 * There must be no direct dependency:
 *
 *     blocks.g4 -> quantum::ir
 *     blocks.g4 -> QEC
 *     blocks.g4 -> ZQN
 *     blocks.g4 -> scheduler
 *     blocks.g4 -> routing
 *     blocks.g4 -> hardware discovery
 *     blocks.g4 -> runtime
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * All lexical definitions are owned by the canonical Zamani lexer.
 *
 * This grammar consumes:
 *
 *     LBRACE
 *     RBRACE
 *
 * supplied by:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * No lexer rule is declared here.
 *
 * In particular, this file MUST NOT define:
 *
 *     LBRACE
 *     RBRACE
 *     IDENT
 *     SEMI
 *     keywords
 *     operators
 *     literals
 *
 * The repository lexer already defines:
 *
 *     LBRACE : '{' ;
 *     RBRACE : '}' ;
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file is intentionally a delegate grammar component.
 *
 * Its two semantic inputs are:
 *
 *     statement
 *     expression
 *
 * Those rules are owned by the canonical statement/expression composition
 * layers.
 *
 * The block grammar must therefore NEVER redefine either rule.
 *
 * Conceptually:
 *
 *     statements composition
 *            |
 *            +--------------------+
 *            |                    |
 *            v                    v
 *       blockExpression       statement
 *            |
 *            v
 *       blockElement
 *            |
 *            +--> statement
 *
 * The canonical parser composition layer is responsible for resolving the
 * shared `statement` rule when this delegate grammar is assembled.
 *
 * This avoids the circular ownership that would result from making
 * blocks.g4 import a grammar that itself imports blocks.g4.
 *
 * ============================================================================
 * WHY BLOCK ELEMENTS ARE STATEMENTS
 * ============================================================================
 *
 * The Zamani AST defines BlockExpression as an ordered sequence of NodeId
 * children.
 *
 * The AST does not introduce a separate `tail_expression` field.
 *
 * Therefore the grammar must not invent one.
 *
 * An expression appearing directly in a block is represented through the
 * canonical expression-statement path:
 *
 *     expression
 *         |
 *         v
 *     expressionStatement
 *         |
 *         v
 *     statement
 *         |
 *         v
 *     blockElement
 *
 * This preserves one canonical representation of source children.
 *
 * Whether the final expression contributes the block's value is a semantic
 * question, not a block-grammar question.
 *
 * ============================================================================
 * PRIMARY RULE
 * ============================================================================
 */

/**
 * A source-level block expression.
 *
 * Examples:
 *
 *     {}
 *
 *     {
 *         let x = compute();
 *     }
 *
 *     {
 *         let x = compute();
 *         x
 *     }
 *
 *     {
 *         quantum_operation();
 *         measure();
 *     }
 *
 *     {
 *         hardware_operation();
 *         result
 *     }
 *
 * The number of elements is unbounded by the language grammar.
 *
 * Practical parser-resource limits, if required for denial-of-service
 * protection, belong to configurable parser/resource policy and MUST NOT
 * become language-level limits.
 */
blockExpression
    : LBRACE blockElement* RBRACE
    ;


/*
 * ============================================================================
 * BLOCK ELEMENT
 * ============================================================================
 */

/**
 * One ordered child of a block.
 *
 * `statement` is deliberately referenced rather than copied.
 *
 * This ensures that:
 *
 *     declarations
 *     control flow
 *     loops
 *     match constructs
 *     returns
 *     breaks
 *     continues
 *     exceptions
 *     unsafe constructs
 *     effects
 *     quantum statements
 *     HDL statements
 *     classical statements
 *     expression statements
 *     future statement forms
 *
 * all enter blocks through their canonical statement ownership.
 *
 * No fixed enumeration of statement kinds is maintained here.
 */
blockElement
    : statement
    ;


/*
 * ============================================================================
 * EMPTY BLOCKS
 * ============================================================================
 *
 * Empty blocks are syntactically valid:
 *
 *     {}
 *
 * Whether an empty block is legal in a particular semantic context belongs
 * to semantic analysis.
 *
 * Examples:
 *
 *     function bodies
 *     branches
 *     loops
 *     handlers
 *     scopes
 *     synchronization regions
 *     hardware processes
 *     quantum control regions
 *
 * may have different semantic requirements.
 *
 * The block grammar must not encode those contextual restrictions.
 */


/*
 * ============================================================================
 * NESTED BLOCKS
 * ============================================================================
 *
 * A block can contain a statement whose syntax contains another block.
 *
 * For example:
 *
 *     {
 *         if condition {
 *             work();
 *         }
 *     }
 *
 *     {
 *         while condition {
 *             work();
 *         }
 *     }
 *
 *     {
 *         {
 *             nested();
 *         }
 *     }
 *
 * Nesting is achieved through the canonical statement/expression grammar.
 *
 * No maximum nesting depth is encoded.
 *
 * Actual parser recursion/resource limits belong to the parser implementation
 * and configurable resource policy.
 */


/*
 * ============================================================================
 * VALUE-PRODUCING BLOCKS
 * ============================================================================
 *
 * A block can participate in expression contexts through `blockExpression`.
 *
 * Example:
 *
 *     let result = {
 *         let x = compute();
 *         x
 *     };
 *
 * The final `x` enters the block as the canonical expression statement.
 *
 * The parser records the source structure.
 *
 * Semantic analysis determines:
 *
 *     - whether the block produces a value;
 *     - which child is the value-producing child;
 *     - whether the final child is reachable;
 *     - whether its type is valid;
 *     - whether all paths produce compatible values;
 *     - whether effects are permitted;
 *     - whether capabilities are available.
 *
 * The grammar does not enforce those rules.
 *
 * This preserves compatibility with the existing BlockExpression AST, which
 * intentionally stores an ordered child NodeId sequence instead of adding a
 * competing tail-expression representation.
 */


/*
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * `blockElement*` preserves source order.
 *
 * This is semantically important because source order can affect:
 *
 *     - evaluation order
 *     - side effects
 *     - lexical scope
 *     - resource lifetime
 *     - ownership
 *     - borrowing
 *     - control flow
 *     - diagnostics
 *     - source reconstruction
 *     - quantum/classical dependency ordering
 *     - hardware semantic ordering
 *
 * The parser/frontend AST must preserve the resulting order exactly.
 */


/*
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * The grammar contains no language-level limits on:
 *
 *     - block element count
 *     - block nesting
 *     - statement count
 *     - expression size
 *     - program size
 *     - quantum operation count
 *     - qubit count
 *     - classical resource count
 *     - hardware resource count
 *     - device count
 *     - node count
 *     - memory capacity
 *     - accelerator count
 *
 * Therefore:
 *
 *     tiny program
 *          |
 *          v
 *     same syntax
 *          |
 *          v
 *     larger program
 *          |
 *          v
 *     very large program
 *
 * without changing the language grammar.
 *
 * POCO-REAF is preserved because block syntax describes program structure,
 * rather than the physical machine executing the program.
 *
 * There is deliberately no:
 *
 *     MAX_BLOCK_ELEMENTS
 *     MAX_STATEMENTS
 *     MAX_NESTING
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_DEVICES
 *     MAX_MEMORY
 *
 * Any practical resource ceiling must be represented by an external,
 * configurable compiler/parser resource policy.
 */


/*
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     lexer version
 *     grammar version
 *     parser configuration
 *
 * this rule must produce the same parse structure.
 *
 * Parsing must NOT depend on:
 *
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     hardware topology
 *     calibration
 *     queue state
 *     scheduler state
 *     network state
 *     backend state
 *     runtime state
 *
 * No runtime or hardware information may participate in block parsing.
 */


/*
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This grammar performs syntax recognition only.
 *
 * It does NOT decide:
 *
 *     whether a statement is semantically valid;
 *     whether a block is reachable;
 *     whether a block has a value;
 *     whether branches agree on type;
 *     whether a quantum operation is legal;
 *     whether a hardware operation is supported;
 *     whether a resource is available;
 *     whether a capability exists;
 *     whether an effect is permitted;
 *     whether a variable is initialized;
 *     whether ownership is valid;
 *     whether borrowing is valid;
 *     whether an operation can be scheduled;
 *     whether a resource can be routed;
 *     whether a backend can execute the result.
 *
 * These belong to downstream compiler layers.
 */


/*
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * Parser:
 *
 *     `{`
 *       |
 *       +--> zero or more blockElement
 *       |
 *     `}`
 *
 * becomes the frontend AST BlockExpression representation.
 *
 * The existing AST contract is:
 *
 *     BlockExpression
 *         |
 *         +--> ordered Vec<NodeId>
 *
 * The parser must:
 *
 *     1. recognize the opening brace;
 *     2. parse zero or more canonical statements;
 *     3. preserve source order;
 *     4. recognize the closing brace;
 *     5. create the BlockExpression AST node;
 *     6. attach each parsed child NodeId in source order;
 *     7. preserve the source span;
 *     8. allow structural validation to run afterward.
 *
 * The grammar itself does not construct Rust AST values.
 *
 * The AST implementation remains responsible for validating local NodeId
 * invariants.
 */


/*
 * ============================================================================
 * AST SEMANTIC DISTINCTION
 * ============================================================================
 *
 * These representations must remain distinct:
 *
 *     blockExpression
 *         =
 *     parser syntax
 *
 *     BlockExpression
 *         =
 *     source AST structure
 *
 *     semantic block
 *         =
 *     resolved program meaning
 *
 *     ZUIR/control-flow region
 *         =
 *     universal semantic representation
 *
 *     classical IR block
 *         =
 *     classical implementation
 *
 *     quantum::ir representation
 *         =
 *     canonical quantum semantic representation
 *
 *     target block
 *         =
 *     machine/backend realization
 *
 * This file defines only the first layer.
 */


/*
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum syntax may occur inside a block through canonical statement rules.
 *
 * Example conceptually:
 *
 *     {
 *         prepare();
 *         apply_gate(...);
 *         measure();
 *     }
 *
 * This file does not know:
 *
 *     - how many qubits exist;
 *     - which physical qubits exist;
 *     - which backend is selected;
 *     - which topology exists;
 *     - which gate implementation is selected;
 *     - which calibration applies;
 *     - how operations are scheduled.
 *
 * Those decisions belong downstream.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * There is no direct blocks.g4 -> quantum::ir dependency.
 */


/*
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware statements may occur inside blocks when admitted by the
 * canonical statement grammar.
 *
 * Block syntax therefore does not need separate:
 *
 *     hardwareBlock
 *     gpuBlock
 *     fpgaBlock
 *     qpuBlock
 *     cpuBlock
 *     acceleratorBlock
 *
 * unless a future language specification establishes a genuinely different
 * source-language construct.
 *
 * Hardware capabilities and physical resources are downstream concerns.
 */


/*
 * ============================================================================
 * DISTRIBUTED / PARALLEL / AI / FUTURE DOMAIN INTEGRATION
 * ============================================================================
 *
 * The same block structure can contain statements belonging to:
 *
 *     classical computing
 *     parallel computing
 *     distributed computing
 *     AI/ML
 *     accelerators
 *     networking
 *     quantum computing
 *     HDL
 *     embedded computing
 *     future domains
 *
 * Domain-specific syntax belongs to the appropriate statement grammar.
 *
 * This file remains unchanged when a new statement domain is introduced,
 * provided that the new construct enters the canonical `statement` rule.
 *
 * This is an intentional extensibility property.
 */


/*
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE INTEGRATION
 * ============================================================================
 *
 * A block is a lexical/source region.
 *
 * Effects, capabilities and resources are NOT represented by adding fields to
 * this grammar rule.
 *
 * For example:
 *
 *     {
 *         quantum_operation();
 *     }
 *
 * may require capabilities or effects.
 *
 * The semantic layer determines those requirements.
 *
 * Similarly:
 *
 *     {
 *         distributed_work();
 *     }
 *
 * may require distributed execution capabilities.
 *
 * Resource analysis remains downstream.
 *
 * This prevents the grammar from coupling block syntax to a particular
 * execution architecture.
 */


/*
 * ============================================================================
 * CONTROL-FLOW INTEGRATION
 * ============================================================================
 *
 * Statement-level constructs such as:
 *
 *     if
 *     while
 *     for
 *     loop
 *     match
 *     try
 *     catch
 *     finally
 *
 * own their respective control-flow syntax.
 *
 * Those constructs may themselves consume `blockExpression`.
 *
 * This file therefore provides the common block boundary without redefining
 * control-flow constructs.
 *
 * Conceptually:
 *
 *     if condition blockExpression
 *
 *     while condition blockExpression
 *
 *     function ... blockExpression
 *
 *     try blockExpression ...
 *
 *     match ... blockExpression
 *
 * Each construct remains the owner of its own surrounding syntax.
 */


/*
 * ============================================================================
 * CONDITIONAL-EXPRESSION INTEGRATION
 * ============================================================================
 *
 * `grammar/expressions/conditionals.g4` explicitly delegates block ownership
 * to:
 *
 *     grammar/statements/blocks.g4
 *
 * Therefore conditional expressions consume this canonical block rule rather
 * than defining their own braces.
 *
 * Example:
 *
 *     if condition {
 *         value_a
 *     } else {
 *         value_b
 *     }
 *
 * Both branches use:
 *
 *     blockExpression
 *
 * from this file.
 *
 * The conditional-expression grammar must not create another block rule.
 */


/*
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * The expression composition layer may expose `blockExpression` as a primary
 * expression.
 *
 * It must not redefine it.
 *
 * Conceptually:
 *
 *     primaryExpression
 *         :
 *         ...
 *         | blockExpression
 *         ...
 *
 * The canonical expression grammar remains the owner of expression precedence
 * and composition.
 *
 * This file remains responsible only for the block itself.
 */


/*
 * ============================================================================
 * FUNCTION INTEGRATION
 * ============================================================================
 *
 * Function declarations consume a block as their body.
 *
 * The function grammar owns:
 *
 *     fn name(parameters) -> type
 *
 * while this file owns:
 *
 *     { ... }
 *
 * This separation prevents function syntax from becoming coupled to block
 * internals.
 */


/*
 * ============================================================================
 * MEMORY / OWNERSHIP INTEGRATION
 * ============================================================================
 *
 * Blocks establish lexical regions but do not themselves define:
 *
 *     ownership
 *     borrowing
 *     lifetime
 *     allocation
 *     deallocation
 *
 * Those rules belong to the memory/semantic layers.
 *
 * The parser merely preserves the lexical structure needed by those analyses.
 */


/*
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Compilation flow:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     blockExpression
 *       |
 *       v
 *     BlockExpression AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware IR
 *       +--> distributed/control IR
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing/scheduling/lowering
 *       |
 *       v
 *     target
 *
 * This grammar must never bypass the AST/semantic boundary.
 */


/*
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * There is no runtime dependency from this grammar.
 *
 * Runtime behavior is determined only after:
 *
 *     parsing
 *     semantic analysis
 *     lowering
 *     compilation
 *     target realization
 *
 * Therefore changing runtime hardware must not change the parse tree for an
 * otherwise identical source program.
 */


/*
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing a block must not:
 *
 *     - access files;
 *     - access networks;
 *     - execute programs;
 *     - execute macros implicitly;
 *     - probe hardware;
 *     - inspect QPU state;
 *     - inspect calibration;
 *     - inspect runtime state;
 *     - allocate target resources;
 *     - contact a backend.
 *
 * Any macro/metaprogramming facility is governed by its own security and
 * capability model.
 */


/*
 * ============================================================================
 * ERROR HANDLING CONTRACT
 * ============================================================================
 *
 * The grammar must permit the parser infrastructure to diagnose malformed
 * blocks such as:
 *
 *     {
 *
 *     }
 *
 *     { statement
 *
 *     statement }
 *
 *     { } extra
 *
 * Structured diagnostics belong to the parser/frontend diagnostic layer.
 *
 * This grammar must not embed Rust diagnostic construction.
 *
 * Diagnostics should preserve:
 *
 *     source span
 *     offending token
 *     expected construct
 *     grammar context
 *
 * without coupling the grammar to a particular diagnostic storage type.
 */


/*
 * ============================================================================
 * NO ERROR-SWALLOWING
 * ============================================================================
 *
 * The grammar must not introduce permissive alternatives intended to hide
 * malformed source.
 *
 * In particular, do NOT add:
 *
 *     optional arbitrary tokens
 *     wildcard recovery productions
 *     comments-as-statements
 *     unknown-block-element fallbacks
 *
 * merely to make invalid source parse.
 *
 * ANTLR's configured error strategy is responsible for parser recovery.
 *
 * The accepted language must remain deterministic and well-defined.
 */


/*
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The existing canonical parser currently uses:
 *
 *     blockExpression
 *
 * as the common block rule.
 *
 * This file preserves that public rule name.
 *
 * Existing consumers should therefore migrate toward this ownership rather
 * than introducing another name such as:
 *
 *     block
 *     blockStatement
 *     statementBlock
 *     blockBody
 *
 * unless a future language-version migration deliberately establishes such
 * an alias.
 *
 * `blockExpression` is retained because the repository's expression and AST
 * architecture already use that terminology.
 */


/*
 * ============================================================================
 * ANTLR INTEGRATION REQUIREMENT
 * ============================================================================
 *
 * IMPORTANT:
 *
 * `blocks.g4` participates in the repository's modular parser composition.
 *
 * The final canonical parser must assemble:
 *
 *     lexer
 *       +
 *     expression grammars
 *       +
 *     statement grammars
 *       +
 *     declaration grammars
 *       +
 *     block grammar
 *
 * into one coherent parser namespace.
 *
 * There must be exactly one canonical definition of:
 *
 *     blockExpression
 *
 * and exactly one canonical definition of:
 *
 *     blockElement
 *
 * in the assembled parser.
 *
 * If the repository's ANTLR build system uses a single canonical
 * `ZamaniParser.g4`, these rules must be incorporated there through the
 * repository's established grammar-composition mechanism rather than copied
 * into multiple independent grammars.
 *
 * The important invariant is ONE OWNER, not duplicated text.
 */


/*
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Upstream:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * supplies:
 *
 *     LBRACE
 *     RBRACE
 *
 * Canonical statement composition supplies:
 *
 *     statement
 *
 * Canonical expression composition may consume:
 *
 *     blockExpression
 *
 * Downstream:
 *
 *     src/frontend/ast/node/expressions/block.rs
 *
 * receives the resulting source structure through the parser/frontend layer.
 *
 * Semantic analysis consumes the AST.
 *
 * IR layers consume semantic representations.
 *
 * ============================================================================
 * NON-DEPENDENCIES
 * ============================================================================
 *
 * This file intentionally does NOT depend directly upon:
 *
 *     src/quantum/ir
 *     src/quantum/qec
 *     src/quantum/zqn
 *     scheduling
 *     routing
 *     optimization
 *     hardware discovery
 *     calibration
 *     resilience
 *     runtime
 *     backend APIs
 *     QIR
 *     LLVM
 *     MLIR
 *
 * This keeps the grammar at the correct architectural boundary.
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     numeric limits
 *     machine constants
 *     hardware identifiers
 *     topology identifiers
 *     qubit counts
 *     core counts
 *     thread counts
 *     device counts
 *     memory sizes
 *     register counts
 *     accelerator counts
 *     architecture-specific assumptions
 *
 * Repetition uses:
 *
 *     blockElement*
 *
 * rather than finite enumeration.
 *
 * Consequently, block size is bounded only by implementation/resource limits,
 * not by the Zamani language grammar.
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests:
 *
 *     1. {}
 *
 *     2. {
 *            statement;
 *        }
 *
 *     3. {
 *            statement1;
 *            statement2;
 *            statement3;
 *        }
 *
 *     4. {
 *            let x = value;
 *            x;
 *        }
 *
 *     5. Nested block:
 *
 *        {
 *            {
 *                work();
 *            }
 *        }
 *
 *     6. Block used as an expression.
 *
 *     7. Block used as a function body.
 *
 *     8. Block used as a conditional branch.
 *
 *     9. Block used as a loop body.
 *
 *    10. Block containing quantum statements.
 *
 *    11. Block containing classical statements.
 *
 *    12. Block containing mixed-domain statements where the surrounding
 *        language construct permits them.
 *
 * Negative tests:
 *
 *     1. Missing opening brace.
 *
 *     2. Missing closing brace.
 *
 *     3. Unterminated block.
 *
 *     4. Invalid token where a statement is required.
 *
 *     5. Invalid nested construct.
 *
 *     6. Arbitrary text after a block where the surrounding grammar does not
 *        permit it.
 *
 * Boundary tests:
 *
 *     1. Empty block.
 *
 *     2. One-element block.
 *
 *     3. Very large block.
 *
 *     4. Deeply nested blocks.
 *
 *     5. Large nested blocks.
 *
 *     6. Long sequences of statements.
 *
 * Scalability tests:
 *
 *     Verify there is no grammar-defined maximum for:
 *
 *         block elements
 *         nesting
 *         statements
 *         expressions
 *         program size
 *
 * Resource-limit tests:
 *
 *     Parser resource policies may reject pathological input, but such
 *     rejection must be reported as a parser/resource condition and must not
 *     redefine the language grammar.
 *
 * Determinism tests:
 *
 *     Parse identical source repeatedly and verify identical parse structures.
 *
 * Round-trip tests:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter/printer
 *       -> parser
 *
 *     must preserve block structure and child ordering.
 */


/*
 * ============================================================================
 * AST INTEGRATION TESTS
 * ============================================================================
 *
 * For every successfully parsed block:
 *
 *     - opening/closing source span is preserved;
 *     - child order is preserved;
 *     - each child maps to exactly one canonical AST node;
 *     - no child is silently discarded;
 *     - no synthetic machine-specific child is introduced;
 *     - empty blocks produce zero children;
 *     - large blocks do not require a fixed AST capacity.
 *
 * These tests belong primarily in the frontend/AST parser test layer, while
 * grammar tests verify syntactic acceptance/rejection.
 */


/*
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * The block grammar must remain domain-neutral.
 *
 * Tests should prove that the same block structure can contain canonical
 * statements from:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     networking
 *     accelerator
 *
 * domains where those statements are legal.
 *
 * The block grammar itself must not need modification when a new statement
 * domain is added.
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * `blocks.g4` is complete only when all of the following are true:
 *
 * [x] One canonical `blockExpression` owner is established.
 *
 * [x] One canonical `blockElement` owner is established.
 *
 * [x] Block syntax uses canonical lexer tokens.
 *
 * [x] No lexer rules exist in this file.
 *
 * [x] No expression hierarchy is duplicated.
 *
 * [x] No statement hierarchy is duplicated.
 *
 * [x] No final-expression AST field is invented.
 *
 * [x] Child ordering is structurally preserved.
 *
 * [x] Empty blocks are syntactically representable.
 *
 * [x] Arbitrary block size is representable.
 *
 * [x] Arbitrary nesting is representable.
 *
 * [x] No machine-specific limits exist.
 *
 * [x] No qubit limits exist.
 *
 * [x] No hardware limits exist.
 *
 * [x] No runtime dependency exists.
 *
 * [x] No quantum IR dependency exists.
 *
 * [x] `quantum::ir` remains downstream of semantic lowering.
 *
 * [x] QEC remains downstream.
 *
 * [x] ZQN remains downstream.
 *
 * [x] Scheduling remains downstream.
 *
 * [x] Routing remains downstream.
 *
 * [x] Optimization remains downstream.
 *
 * [x] Hardware discovery remains downstream.
 *
 * [x] Rust implementation remains responsible for safe Rust only.
 *
 * [x] No unsafe Rust is required by this grammar.
 *
 * [x] Parser diagnostics remain outside the grammar.
 *
 * [x] Parser resource limits remain external to language semantics.
 *
 * [x] The existing BlockExpression AST model is preserved.
 *
 * [x] Conditional-expression grammar can consume this canonical block rule.
 *
 * [x] Function/control-flow grammars can consume this canonical block rule.
 *
 * [x] Expression grammar can consume this canonical block rule.
 *
 * [x] No circular grammar ownership is introduced.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 *     BLOCK SYNTAX
 *          |
 *          v
 *     SOURCE AST
 *          |
 *          v
 *     SEMANTIC ANALYSIS
 *          |
 *          v
 *     UNIVERSAL / DOMAIN IR
 *          |
 *          v
 *     OPTIMIZATION
 *          |
 *          v
 *     ROUTING / SCHEDULING / LOWERING
 *          |
 *          v
 *     HARDWARE / RUNTIME
 *
 * A block describes a lexical and computational region.
 *
 * It does not describe the machine on which that region executes.
 *
 * This is the required boundary for POCO-REAF:
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
 * ============================================================================
 */