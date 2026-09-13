/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/conditionals.g4
 *
 * Status:
 *     Canonical production grammar for statement-level conditional control
 *     flow.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar component.
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1.
 *
 * Safety:
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No unsafe code.
 *     No I/O.
 *     No filesystem access.
 *     No networking.
 *     No device discovery.
 *     No runtime execution.
 *     No hardware inspection.
 *     No mutable global state.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns STATEMENT-LEVEL conditional control flow.
 *
 * It defines:
 *
 *     if <condition> <block>
 *     if <condition> <block> else <block>
 *     if <condition> <block> else if <condition> <block> ...
 *
 * It does NOT define value-producing conditional expressions.
 *
 * Those belong to:
 *
 *     grammar/expressions/conditionals.g4
 *
 * The two constructs intentionally remain separate:
 *
 *     statement conditional
 *         -> control flow
 *
 *     conditional expression
 *         -> value production
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - ifStatement
 *     - elseIfClause
 *     - elseClause
 *     - statement-level conditional composition
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - IF / ELSE lexer tokens
 *     - expression syntax
 *     - expression precedence
 *     - block syntax
 *     - declarations
 *     - assignments
 *     - loops
 *     - match syntax
 *     - exceptions
 *     - AST definitions
 *     - semantic analysis
 *     - classical IR
 *     - quantum::ir
 *     - QEC
 *     - ZQN
 *     - optimization
 *     - routing
 *     - scheduling
 *     - hardware discovery
 *     - target selection
 *     - runtime execution
 *     - resource limits
 *
 * ============================================================================
 * REPOSITORY INTEGRATION
 * ============================================================================
 *
 * grammar/statements/statements.g4 already provides the composition boundary:
 *
 *     controlFlowStatement
 *         -> conditionalStatement
 *             -> ifStatement
 *
 * This file supplies the canonical `ifStatement`.
 *
 * It MUST NOT redefine:
 *
 *     statement
 *     controlFlowStatement
 *     conditionalStatement
 *
 * The expression grammar separately owns:
 *
 *     conditionalExpression
 *
 * The block grammar separately owns:
 *
 *     blockExpression
 *
 * Therefore the architecture is:
 *
 *     statement
 *        |
 *        +--> controlFlowStatement
 *                |
 *                +--> conditionalStatement
 *                        |
 *                        +--> ifStatement
 *                                |
 *                                +--> expression
 *                                +--> blockExpression
 *                                +--> elseIfClause*
 *                                +--> elseClause?
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical Zamani lexer supplies:
 *
 *     IF
 *     ELSE
 *
 * along with the punctuation required by the referenced expression and block
 * grammars.
 *
 * No lexer rules are declared here.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * `expression` is supplied by the canonical expression grammar.
 *
 * This file does not impose a separate:
 *
 *     booleanExpression
 *     conditionExpression
 *     classicalCondition
 *     quantumCondition
 *
 * grammar.
 *
 * Whether an expression is semantically valid as a condition is determined by
 * semantic/type analysis.
 *
 * ============================================================================
 * BLOCK CONTRACT
 * ============================================================================
 *
 * `blockExpression` is supplied by:
 *
 *     grammar/statements/blocks.g4
 *
 * This file does not redefine `{ ... }`.
 *
 * Consequently, every canonical Zamani statement that is legal inside a block
 * remains available inside a conditional branch.
 *
 * ============================================================================
 */

parser grammar ConditionalsParser;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * STATEMENT-LEVEL IF
 * ============================================================================
 *
 * Canonical forms:
 *
 *     if condition {
 *         body
 *     }
 *
 *     if condition {
 *         then_body
 *     } else {
 *         else_body
 *     }
 *
 *     if condition_a {
 *         body_a
 *     } else if condition_b {
 *         body_b
 *     } else {
 *         body_c
 *     }
 *
 * A final `else` is optional because a statement-level conditional does not
 * itself need to produce a value.
 *
 * `elseIfClause*` deliberately uses repetition rather than a finite number of
 * alternatives. Therefore the grammar imposes no language-level branch-count
 * limit.
 */

ifStatement
    : IF expression blockExpression elseIfClause* elseClause?
    ;


/*
 * ============================================================================
 * ELSE-IF
 * ============================================================================
 *
 * An else-if clause consists of:
 *
 *     else
 *     if
 *     condition
 *     branch
 *
 * The repeated form in `ifStatement` permits arbitrary chaining.
 */

elseIfClause
    : ELSE IF expression blockExpression
    ;


/*
 * ============================================================================
 * ELSE
 * ============================================================================
 */

elseClause
    : ELSE blockExpression
    ;


/*
 * ============================================================================
 * STATEMENT / EXPRESSION SEPARATION
 * ============================================================================
 *
 * DO NOT define `conditionalExpression` here.
 *
 * That construct belongs to:
 *
 *     grammar/expressions/conditionals.g4
 *
 * Statement context:
 *
 *     if condition {
 *         work();
 *     }
 *
 * Expression context:
 *
 *     let result =
 *         if condition {
 *             value_a
 *         } else {
 *             value_b
 *         };
 *
 * These are different language constructs even though they share IF/ELSE
 * lexical tokens.
 *
 * The expression form is value-producing and therefore requires its own
 * semantic contract.
 *
 * The statement form is control flow and does not require a resulting value.
 *
 * This separation prevents a generic `if` rule from becoming ambiguous or
 * forcing statement semantics into the expression grammar.
 */


/*
 * ============================================================================
 * CONDITION SEMANTICS
 * ============================================================================
 *
 * The parser only establishes:
 *
 *     IF expression blockExpression
 *
 * Semantic analysis determines:
 *
 *     - whether the condition is valid;
 *     - whether its type is acceptable;
 *     - whether required effects are available;
 *     - whether required capabilities are available;
 *     - whether the condition is reachable;
 *     - whether branch-local state is valid;
 *     - whether ownership/borrowing rules are satisfied;
 *     - whether resource usage is legal;
 *     - whether domain-specific operations are legal.
 *
 * None of these checks belong in this grammar.
 */


/*
 * ============================================================================
 * BLOCK SEMANTICS
 * ============================================================================
 *
 * Every branch uses the canonical blockExpression.
 *
 * Therefore branches can contain:
 *
 *     declarations
 *     assignments
 *     loops
 *     nested conditionals
 *     match statements
 *     returns
 *     breaks
 *     continues
 *     effects
 *     concurrency
 *     classical operations
 *     quantum operations
 *     hybrid operations
 *     HDL operations
 *     hardware operations
 *     distributed operations
 *     accelerator operations
 *     AI/data operations
 *     future domain statements
 *
 * The conditional grammar does not need to know which domain a statement
 * belongs to.
 *
 * This is essential for Zamani's universal-computing architecture.
 */


/*
 * ============================================================================
 * NESTED CONDITIONALS
 * ============================================================================
 *
 * Because branch bodies are canonical blocks, nested conditionals are naturally
 * supported:
 *
 *     if outer_condition {
 *         if inner_condition {
 *             work();
 *         } else {
 *             fallback();
 *         }
 *     }
 *
 * No fixed nesting depth is encoded.
 *
 * Any practical parser recursion/resource limit is an implementation/resource
 * policy and MUST NOT become a language semantic limit.
 */


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser/frontend must preserve:
 *
 *     condition
 *     then branch
 *     ordered else-if condition/branch pairs
 *     optional else branch
 *     source span
 *
 * Conceptual semantic shape:
 *
 *     ConditionalStatement {
 *         condition,
 *         then_branch,
 *         else_if_branches,
 *         else_branch,
 *         source_span
 *     }
 *
 * The exact Rust AST type is owned by the frontend AST subsystem.
 *
 * This grammar must never:
 *
 *     - define Rust structs;
 *     - define Rust enums;
 *     - allocate AST nodes;
 *     - execute semantic validation;
 *     - construct IR.
 *
 * Source order must be preserved exactly.
 */


/*
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * The intended pipeline is:
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
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic/control-flow representation
 *       |
 *       +--> classical representation
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed/accelerator representation
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling / lowering
 *       |
 *       v
 *     target realization
 *       |
 *       v
 *     runtime
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This file must never create a second quantum IR.
 */


/*
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A conditional may guard quantum operations:
 *
 *     if ready {
 *         quantum_operation();
 *         measurement();
 *     } else {
 *         fallback();
 *     }
 *
 * The grammar does not determine:
 *
 *     - qubit count;
 *     - logical/physical mapping;
 *     - backend;
 *     - topology;
 *     - calibration;
 *     - gate implementation;
 *     - QEC strategy;
 *     - ZQN noise model;
 *     - scheduling;
 *     - routing.
 *
 * Those are downstream semantic/compiler/runtime concerns.
 *
 * Therefore:
 *
 *     grammar/statements/conditionals.g4
 *
 * has NO direct dependency on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     hardware HAL
 *     calibration
 */


/*
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * The same conditional syntax can control:
 *
 *     classical computation
 *     vector/matrix/tensor computation
 *     accelerator operations
 *     quantum operations
 *     HDL behavior
 *     hardware operations
 *     distributed operations
 *     networking operations
 *     AI/data operations
 *     future domains
 *
 * No separate:
 *
 *     cpuIfStatement
 *     gpuIfStatement
 *     fpgaIfStatement
 *     qpuIfStatement
 *     distributedIfStatement
 *
 * is permitted merely because the eventual target differs.
 *
 * Target-specific realization belongs downstream.
 */


/*
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Conditions may semantically depend on capabilities or resource state, but
 * this grammar does not interpret those concepts.
 *
 * It contains no:
 *
 *     MAX_BRANCHES
 *     MAX_NESTING
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_ACCELERATORS
 *     device IDs
 *     hardware addresses
 *     topology constants
 *
 * Resource requirements, constraints, preferences, capabilities, placement and
 * target information belong to their respective language/compiler layers.
 */


/*
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * This grammar is intentionally independent of physical machine scale.
 *
 * The same conditional syntax can describe:
 *
 *     a tiny embedded program
 *     a single CPU program
 *     a multicore program
 *     a GPU program
 *     an FPGA design
 *     an ASIC-oriented program
 *     a quantum program
 *     a hybrid quantum-classical program
 *     a distributed program
 *     an HPC program
 *     a heterogeneous program
 *     a future architecture
 *
 * Branch cardinality is unbounded by grammar constants.
 *
 * Nesting is unbounded by grammar constants.
 *
 * Program size is unbounded by grammar constants.
 *
 * Practical resource ceilings must be external parser/compiler resource
 * policies and must not change the language's semantic meaning.
 *
 * This preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * because the conditional describes program semantics rather than a particular
 * machine configuration.
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
 * parsing must produce the same structural result.
 *
 * Parsing must NOT depend on:
 *
 *     CPU availability
 *     GPU availability
 *     QPU availability
 *     FPGA availability
 *     hardware topology
 *     calibration
 *     queue state
 *     scheduler state
 *     network state
 *     backend state
 *     runtime state
 *
 * The grammar has no external side effects.
 */


/*
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser/frontend diagnostics should identify source spans for:
 *
 *     - missing condition;
 *     - missing then block;
 *     - missing opening brace;
 *     - missing closing brace;
 *     - malformed else-if condition;
 *     - malformed else-if branch;
 *     - malformed else branch;
 *     - unexpected tokens between clauses;
 *     - incomplete conditional at EOF.
 *
 * Diagnostic wording and recovery policy belong to the parser/frontend
 * diagnostic subsystem.
 *
 * No Rust actions or custom error code is embedded here.
 */


/*
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     device discovery
 *     hardware probing
 *     runtime dispatch
 *
 * Macro/metaprogramming facilities, if present elsewhere in Zamani, must be
 * governed by their own security/capability model.
 */


/*
 * ============================================================================
 * COMPATIBILITY WITH LEGACY ZAMANI GRAMMAR
 * ============================================================================
 *
 * The historical monolithic grammar contains the equivalent structure:
 *
 *     if expression block
 *     (else if expression block)*
 *     (else block)?
 *
 * This modular grammar preserves that source syntax.
 *
 * The migration requirement is:
 *
 *     legacy conditional syntax
 *             |
 *             v
 *     statements/conditionals.g4
 *             |
 *             v
 *     statements.g4
 *             |
 *             v
 *     canonical parser
 *
 * Once the modular grammar is authoritative, the legacy `Zamani.g4`
 * conditional production must not be compiled as a competing source of truth.
 *
 * It may remain temporarily for compatibility/migration tooling, but the
 * authoritative parser generation path must expose only one effective
 * statement-level conditional definition.
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     1. if condition {}
 *     2. if condition { work(); }
 *     3. if condition {} else {}
 *     4. if condition {} else if other {}
 *     5. if a {} else if b {} else {}
 *     6. multiple else-if clauses
 *     7. nested if statements
 *     8. if inside a loop
 *     9. if inside a function
 *    10. if inside a nested block
 *    11. declarations inside branches
 *    12. assignments inside branches
 *    13. expression statements inside branches
 *    14. quantum statements inside branches
 *    15. hardware/HDL statements where their grammar permits them
 *
 * NEGATIVE:
 *
 *     1. if {}
 *     2. if condition
 *     3. if condition else {}
 *     4. if condition { }
 *        else
 *        malformed branch
 *     5. else {}
 *     6. else if {}
 *     7. if condition {} else if {}
 *     8. if condition {} unexpected
 *     9. incomplete conditional at EOF
 *    10. malformed block delimiters
 *
 * BOUNDARY:
 *
 *     - long else-if chains;
 *     - deeply nested conditionals;
 *     - large branch bodies;
 *     - large condition expressions;
 *     - very large source units.
 *
 * CROSS-DOMAIN:
 *
 *     classical + quantum
 *     classical + HDL
 *     quantum + hardware
 *     quantum + distributed
 *     AI + quantum
 *     classical + quantum + distributed
 *     classical + quantum + HDL + hardware
 *
 * These tests verify syntactic composition only. Semantic legality remains
 * downstream.
 *
 * DETERMINISM:
 *
 *     Reparse identical source repeatedly and verify identical parse/AST shape.
 *
 * ROUND-TRIP:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> printer
 *       -> parser
 *
 * must preserve conditional structure and branch ordering.
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no language-level fixed limit for:
 *
 *     branch count
 *     nesting depth
 *     program size
 *     expression size
 *     qubits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     memory
 *     devices
 *     nodes
 *     topology
 *     accelerators
 *
 * Any future discovered restriction must be classified as:
 *
 *     1. genuine language semantic requirement
 *     2. target-specific requirement
 *     3. resource constraint
 *     4. implementation limitation
 *     5. accidental hard-coding
 *     6. test-only limitation
 *     7. documentation-only limitation
 *
 * Accidental language-level hard-coding must be removed.
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] `ifStatement` is the sole owner of statement-level `if` syntax.
 *     [ ] `elseIfClause` is owned here.
 *     [ ] `elseClause` is owned here.
 *     [ ] `conditionalExpression` remains owned by expressions/conditionals.g4.
 *     [ ] `expression` comes from the canonical expression grammar.
 *     [ ] `blockExpression` comes from statements/blocks.g4.
 *     [ ] IF/ELSE come only from the canonical lexer.
 *     [ ] No lexer rules are duplicated.
 *     [ ] No statement dispatcher is duplicated.
 *     [ ] No block grammar is duplicated.
 *     [ ] No AST types are defined here.
 *     [ ] No semantic validation is embedded here.
 *     [ ] No IR is defined here.
 *     [ ] No quantum IR is defined here.
 *     [ ] No QEC/ZQN dependency exists.
 *     [ ] No routing/scheduling/hardware dependency exists.
 *     [ ] No machine/resource limit is encoded.
 *     [ ] Nested conditionals work without a fixed nesting limit.
 *     [ ] Else-if chains work without a fixed branch limit.
 *     [ ] Parsing is deterministic.
 *     [ ] Positive tests pass.
 *     [ ] Negative tests pass.
 *     [ ] Boundary/scalability tests pass.
 *     [ ] Cross-domain composition tests pass.
 *     [ ] Determinism tests pass.
 *     [ ] Round-trip tests pass.
 *     [ ] `statements.g4` resolves its conditional path to this rule.
 *     [ ] The legacy root grammar is not a competing authoritative parser path.
 *     [ ] Generated Rust integration remains compatible with Rust 1.97/1.97.1.
 *     [ ] No unsafe Rust is required.
 *
 * ============================================================================
 * FINAL PRINCIPLE
 * ============================================================================
 *
 * Zamani conditional syntax describes program control flow.
 *
 * It does not describe:
 *
 *     a particular CPU
 *     a particular GPU
 *     a particular FPGA
 *     a particular ASIC
 *     a particular QPU
 *     a particular topology
 *     a particular device
 *     a particular deployment
 *
 * Therefore:
 *
 *     one source conditional
 *          |
 *          v
 *     one semantic meaning
 *          |
 *          +--> classical realization
 *          +--> quantum realization
 *          +--> hybrid realization
 *          +--> HDL/hardware realization
 *          +--> distributed realization
 *          +--> accelerator realization
 *          +--> future realization
 *
 * without changing the source construct merely because the execution machine
 * changes.
 *
 * This is the required POCO-REAF boundary.
 */