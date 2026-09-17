/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/conditionals.g4
 *
 * Status:
 *     Canonical, production-ready modular grammar component for
 *     value-producing conditional expressions.
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only.
 *     No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE AUTHORITATIVE MODULAR SYNTAX OWNER for
 * value-producing conditional expressions.
 *
 * It owns:
 *
 *     conditionalExpression
 *     ifExpression
 *     elseIfExpressionBranch
 *     elseExpressionBranch
 *     ternaryConditionalExpression
 *
 * It supports:
 *
 *     - structured if expressions;
 *     - else-if chains;
 *     - optional final else branches;
 *     - ternary conditional expressions;
 *     - arbitrary syntactic nesting subject only to implementation resources;
 *     - use inside classical, quantum, hybrid, HDL, hardware, AI, data,
 *       distributed, networking, security, and future domains;
 *     - target-independent conditional computation.
 *
 * It does NOT own:
 *
 *     - the public expression entry point;
 *     - assignment;
 *     - range expressions;
 *     - logical operators;
 *     - arithmetic operators;
 *     - blocks;
 *     - statement-level conditionals;
 *     - lexer tokens;
 *     - AST structures;
 *     - semantic analysis;
 *     - type checking;
 *     - effect checking;
 *     - resource analysis;
 *     - capability resolution;
 *     - IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - HAL;
 *     - target selection;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY CONTRACT
 * ============================================================================
 *
 * There must be exactly one authoritative modular definition of:
 *
 *     conditionalExpression
 *
 * within grammar/expressions/.
 *
 * This file is that authority.
 *
 * The deleted/retired:
 *
 *     grammar/expressions/conditional-expressions.g4
 *
 * MUST NOT be recreated as a competing grammar.
 *
 * No other expression grammar may independently define:
 *
 *     conditionalExpression
 *     ifExpression
 *     ternaryConditionalExpression
 *
 * The canonical expression-composition grammar:
 *
 *     grammar/expressions/expressions.g4
 *
 * owns the precedence hierarchy and references conditionalExpression.
 *
 * This file owns the implementation of conditionalExpression itself.
 *
 * ============================================================================
 * EXISTING EXPRESSION HIERARCHY
 * ============================================================================
 *
 * The existing expressions.g4 establishes:
 *
 *     expression
 *         |
 *     assignmentExpression
 *         |
 *     conditionalExpression
 *         |
 *     rangeExpression
 *         |
 *     logicalOrExpression
 *         |
 *     logicalAndExpression
 *         |
 *     bitwiseOrExpression
 *         |
 *     bitwiseXorExpression
 *         |
 *     bitwiseAndExpression
 *         |
 *     equalityExpression
 *         |
 *     relationalExpression
 *         |
 *     shiftExpression
 *         |
 *     additiveExpression
 *         |
 *     multiplicativeExpression
 *         |
 *     prefixExpression
 *         |
 *     postfixExpression
 *         |
 *     primaryExpression
 *
 * This file MUST preserve that architecture.
 *
 * Therefore:
 *
 *     conditionalExpression
 *
 * consumes the next-tighter:
 *
 *     rangeExpression
 *
 * for the condition of a ternary expression.
 *
 * It MUST NOT use:
 *
 *     expression
 *
 * as the condition operand of the ternary operator.
 *
 * Otherwise the conditional layer would recursively consume its own
 * precedence boundary and make the grammar unnecessarily ambiguous and
 * difficult to reason about.
 *
 * ============================================================================
 * CANONICAL PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical parser composition
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic model / ZUIR
 *       |
 *       +-------------------+---------------------+
 *       |                   |                     |
 *       v                   v                     v
 *   classical          quantum::ir          HDL/hardware
 *       |                   |                     |
 *       +-------------------+---------------------+
 *                           |
 *                           v
 *                     optimization
 *                           |
 *                   routing/scheduling
 *                           |
 *                  resilience/QEC/ZQN
 *                           |
 *                          HAL
 *                           |
 *                   target realization
 *
 * This grammar creates no IR.
 *
 * quantum::ir remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Lexer ownership remains outside this file.
 *
 * The canonical lexer must provide the tokens consumed here, including:
 *
 *     IF
 *     ELSE
 *     QUESTION
 *     COLON
 *
 * and whatever token vocabulary is required by rangeExpression and
 * blockExpression.
 *
 * This file MUST NOT create aliases such as:
 *
 *     QUESTION_MARK
 *
 * when QUESTION is already canonical.
 *
 * Likewise, this file MUST NOT duplicate IF or ELSE lexer definitions.
 *
 * ============================================================================
 * BLOCK CONTRACT
 * ============================================================================
 *
 * Structured conditional branches consume:
 *
 *     blockExpression
 *
 * from the repository's canonical block grammar.
 *
 * This file MUST NOT redefine:
 *
 *     block
 *     blockExpression
 *     blockElement
 *
 * Branch blocks therefore inherit the complete universal Zamani block
 * capability, including constructs from:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     concurrency
 *     effects
 *     future domains
 *
 * ============================================================================
 * CONDITION CONTRACT
 * ============================================================================
 *
 * Conditions use:
 *
 *     rangeExpression
 *
 * rather than the complete:
 *
 *     expression
 *
 * This is deliberate.
 *
 * It means that:
 *
 *     a ? b : c
 *
 * parses as:
 *
 *     rangeExpression QUESTION expression COLON conditionalExpression
 *
 * and prevents the condition from recursively consuming another conditional
 * expression before QUESTION is reached.
 *
 * Semantic analysis remains responsible for deciding whether the resulting
 * expression is a valid condition.
 *
 * The grammar does not introduce separate:
 *
 *     booleanExpression
 *     classicalCondition
 *     quantumCondition
 *     hardwareCondition
 *     resourceCondition
 *
 * Conditional syntax is domain-neutral.
 *
 * ============================================================================
 * STRUCTURED IF EXPRESSIONS
 * ============================================================================
 *
 * Canonical forms:
 *
 *     if condition {
 *         value
 *     }
 *
 *     if condition {
 *         value_a
 *     } else {
 *         value_b
 *     }
 *
 *     if condition_a {
 *         value_a
 *     } else if condition_b {
 *         value_b
 *     } else {
 *         value_c
 *     }
 *
 * The final else branch is syntactically optional.
 *
 * Whether an omitted else is semantically legal depends on the expression
 * context and the language's type/flow rules.
 *
 * ============================================================================
 * ELSE-IF CHAINS
 * ============================================================================
 *
 * Else-if branches are repeated structurally:
 *
 *     (ELSE IF rangeExpression blockExpression)*
 *
 * There is deliberately no enumeration such as:
 *
 *     elseIf1
 *     elseIf2
 *     elseIf3
 *
 * Therefore the grammar imposes no artificial branch-count limit.
 *
 * Any practical limit comes from:
 *
 *     - source size;
 *     - parser memory;
 *     - compiler resources;
 *     - runtime resources;
 *     - deployment policy.
 *
 * None is a Zamani language limit.
 *
 * ============================================================================
 * TERNARY CONDITIONALS
 * ============================================================================
 *
 * Canonical form:
 *
 *     condition ? thenValue : elseValue
 *
 * The grammar is:
 *
 *     rangeExpression
 *         QUESTION
 *     expression
 *         COLON
 *     conditionalExpression
 *
 * The false/else branch recursively consumes conditionalExpression.
 *
 * This makes ternary conditionals RIGHT ASSOCIATIVE.
 *
 * Therefore:
 *
 *     a ? b : c ? d : e
 *
 * is parsed as:
 *
 *     a ? b : (c ? d : e)
 *
 * rather than:
 *
 *     (a ? b : c) ? d : e
 *
 * This association is part of the grammar contract and must not be
 * independently reimplemented downstream.
 *
 * ============================================================================
 * MIDDLE OPERAND CONTRACT
 * ============================================================================
 *
 * The true branch of a ternary consumes:
 *
 *     expression
 *
 * rather than:
 *
 *     rangeExpression
 *
 * This permits the true branch to contain a complete Zamani expression,
 * including assignments and nested conditional expressions where the
 * surrounding syntax makes them unambiguous.
 *
 * Example:
 *
 *     condition ? value_a + value_b : value_c
 *
 * and:
 *
 *     condition ? a ? b : c : d
 *
 * are structurally representable according to the canonical expression
 * hierarchy.
 *
 * Parentheses remain available wherever explicit grouping is desirable.
 *
 * ============================================================================
 * ELSE OPERAND CONTRACT
 * ============================================================================
 *
 * The false branch consumes:
 *
 *     conditionalExpression
 *
 * rather than the complete expression.
 *
 * This is the mechanism that gives the ternary operator its right
 * associativity while preserving its position below rangeExpression and
 * above assignmentExpression.
 *
 * ============================================================================
 * STRUCTURED IF ASSOCIATIVITY
 * ============================================================================
 *
 * Structured if expressions are represented as one conditional expression
 * containing:
 *
 *     - one initial condition;
 *     - one initial branch;
 *     - zero or more ordered else-if branches;
 *     - zero or one final else branch.
 *
 * Example:
 *
 *     if a {
 *         x
 *     } else if b {
 *         y
 *     } else if c {
 *         z
 *     } else {
 *         w
 *     }
 *
 * is one conditional-expression structure with three conditions and four
 * possible result branches.
 *
 * ============================================================================
 * EXPRESSION VS STATEMENT OWNERSHIP
 * ============================================================================
 *
 * This file owns VALUE-PRODUCING conditional expressions.
 *
 * Statement-level control flow belongs to the statement grammar.
 *
 * The repository MUST NOT use this file to redefine statement-level:
 *
 *     if
 *     else
 *     else-if
 *
 * constructs.
 *
 * The lexical tokens may be shared.
 *
 * The syntax ownership and AST semantics remain distinct.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar lowers into the existing domain-neutral frontend AST.
 *
 * It MUST NOT introduce a second AST hierarchy.
 *
 * The AST representation must preserve sufficient information for:
 *
 *     - source spans;
 *     - condition;
 *     - then branch;
 *     - ordered else-if branches;
 *     - optional else branch;
 *     - ternary source form when required;
 *     - child ordering;
 *     - nesting;
 *     - diagnostics;
 *     - provenance.
 *
 * Conceptually:
 *
 *     ConditionalExpression {
 *         condition,
 *         then_branch,
 *         else_if_branches,
 *         else_branch,
 *         source_span
 *     }
 *
 * The actual Rust type is owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT define Rust AST structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure.
 *
 * Semantic analysis establishes meaning.
 *
 * Semantic analysis owns:
 *
 *     - condition validity;
 *     - condition typing;
 *     - truth-value rules;
 *     - branch result typing;
 *     - branch compatibility;
 *     - contextual typing;
 *     - conversions;
 *     - effects;
 *     - ownership;
 *     - borrowing;
 *     - resource requirements;
 *     - capabilities;
 *     - domain legality;
 *     - control-flow guarantees.
 *
 * Precedence and associativity MUST NOT be confused with evaluation order.
 *
 * A parse tree establishes grouping.
 *
 * It does not by itself establish:
 *
 *     - eager evaluation;
 *     - lazy evaluation;
 *     - short-circuit behavior;
 *     - parallel evaluation;
 *     - speculative execution;
 *     - quantum execution;
 *     - hardware scheduling.
 *
 * Those are semantic/compiler/runtime concerns.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * For value-producing conditionals, semantic analysis determines the common
 * or otherwise valid result type of the branches.
 *
 * For:
 *
 *     if condition {
 *         a
 *     } else {
 *         b
 *     }
 *
 * the language type system determines whether the branch results can inhabit
 * the required surrounding type.
 *
 * The grammar MUST NOT encode a fixed type universe for conditional results.
 *
 * This permits:
 *
 *     scalar values
 *     vectors
 *     matrices
 *     tensors
 *     quantum values
 *     classical values
 *     hardware descriptions
 *     resources
 *     capabilities
 *     distributed values
 *     futures
 *     streams
 *     user-defined types
 *     future types
 *
 * without changing conditional grammar.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum conditions remain semantic constructs.
 *
 * This grammar must not enumerate quantum-specific conditional operators or
 * hardware-specific measurement forms merely to support conditional syntax.
 *
 * Quantum examples can use the same conditional structure:
 *
 *     if measurement_result {
 *         operation_a
 *     } else {
 *         operation_b
 *     }
 *
 * or:
 *
 *     measurement_result ? operation_a : operation_b
 *
 * Quantum legality, measurement semantics, classical feed-forward,
 * reversibility, dynamic control, and hardware constraints are resolved
 * downstream.
 *
 * Any resulting quantum computation lowers through the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This grammar does not create a quantum conditional IR.
 *
 * ============================================================================
 * CLASSICAL / HDL / HYBRID CONTRACT
 * ============================================================================
 *
 * The same conditional syntax applies to:
 *
 *     classical computation;
 *     quantum-classical hybrid computation;
 *     HDL expressions;
 *     hardware/software co-design;
 *     AI/ML;
 *     tensor/dataflow computation;
 *     distributed computation;
 *     networking;
 *     security;
 *     accelerator programming;
 *     future domains.
 *
 * Domain-specific behavior belongs to semantic analysis and downstream IRs.
 *
 * No domain is permitted to fork the universal conditional-expression
 * precedence model.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Conditional expressions may select computations involving arbitrary
 * resource and capability values.
 *
 * Examples:
 *
 *     capability_available ? use_capability() : fallback()
 *
 *     enough_memory ? allocate() : stream()
 *
 * The grammar does not inspect actual resources.
 *
 * Parsing MUST NOT query:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     memory
 *     topology
 *     device state
 *     network state
 *
 * Resource and capability decisions occur after parsing.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no artificial finite limit on:
 *
 *     - number of conditional branches;
 *     - number of nested conditionals;
 *     - expression size;
 *     - branch size;
 *     - number of conditions;
 *     - number of domain constructs inside branches;
 *     - number of resources referenced semantically;
 *     - number of operations selected by conditions.
 *
 * Repetition is structural.
 *
 * There are no:
 *
 *     MAX_CONDITIONAL_BRANCHES
 *     MAX_NESTED_CONDITIONALS
 *     MAX_TERNARY_DEPTH
 *     MAX_EXPRESSION_DEPTH
 *
 * language constants.
 *
 * "Infinity" means:
 *
 *     no artificial language-level finite limit is introduced.
 *
 * Actual implementation exhaustion must be treated as a resource/diagnostic
 * condition, never as a semantic reason to restrict the language.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Conditional syntax MUST NOT hard-code:
 *
 *     CPU identifiers;
 *     GPU identifiers;
 *     FPGA identifiers;
 *     physical qubit identifiers;
 *     memory-bank identifiers;
 *     network-node identifiers;
 *     accelerator identifiers;
 *     fixed topology;
 *     fixed device counts;
 *     fixed register widths;
 *     fixed machine sizes.
 *
 * A conditional selects semantic computation.
 *
 * Target realization remains downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this grammar depends only on:
 *
 *     - source token sequence;
 *     - active language version;
 *     - enabled, explicitly declared grammar features.
 *
 * Parsing MUST NOT depend on:
 *
 *     - wall-clock time;
 *     - randomness;
 *     - environment variables;
 *     - filesystem state;
 *     - network state;
 *     - hardware discovery;
 *     - runtime scheduler state;
 *     - device state;
 *     - backend availability.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing is non-executing.
 *
 * A branch containing:
 *
 *     system.run(...)
 *     network.send(...)
 *     device.invoke(...)
 *
 * remains syntax only during parsing.
 *
 * The parser MUST NOT:
 *
 *     - execute commands;
 *     - access files;
 *     - access credentials;
 *     - access networks;
 *     - inspect hardware;
 *     - invoke devices;
 *     - invoke compilers;
 *     - invoke simulators;
 *     - load arbitrary plugins.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * The parser/frontend should report useful source locations for:
 *
 *     - missing condition;
 *     - missing branch block;
 *     - missing colon in ternary;
 *     - malformed else-if;
 *     - malformed else branch;
 *     - unexpected QUESTION;
 *     - unexpected COLON;
 *     - unmatched delimiters.
 *
 * Semantic diagnostics own:
 *
 *     - invalid condition type;
 *     - incompatible branch types;
 *     - invalid effects;
 *     - illegal resource use;
 *     - illegal quantum control;
 *     - unavailable capability;
 *     - invalid target realization.
 *
 * Diagnostics must preserve source spans.
 *
 * ============================================================================
 * ERROR RECOVERY
 * ============================================================================
 *
 * Error recovery belongs to the parser/frontend infrastructure.
 *
 * This grammar must not embed parser actions or unsafe recovery mechanisms.
 *
 * Recovery must not execute semantic operations.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * The grammar is designed to avoid:
 *
 *     - full-expression recursion in the ternary condition;
 *     - duplicate conditional hierarchies;
 *     - enumerated branch counts;
 *     - semantic predicates;
 *     - embedded actions;
 *     - runtime-dependent parsing.
 *
 * Else-if chains use repetition.
 *
 * Ternary right-association is represented structurally rather than by
 * backtracking.
 *
 * The implementation should maintain deterministic parsing and avoid
 * exponential ambiguity.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages may transform conditional expressions only when the
 * transformation preserves language semantics.
 *
 * Optimization MUST preserve:
 *
 *     - branch meaning;
 *     - side-effect rules;
 *     - effect ordering;
 *     - ownership rules;
 *     - observable behavior;
 *     - floating-point semantics where required;
 *     - quantum semantics;
 *     - hardware/HDL semantics;
 *     - resource constraints;
 *     - determinism guarantees.
 *
 * The compiler may lower:
 *
 *     if-expression
 *
 * into:
 *
 *     control-flow IR
 *
 * or an equivalent canonical representation.
 *
 * This file does not prescribe the IR representation.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime behavior is downstream from parsing.
 *
 * The runtime decides how conditional computation is executed on available
 * resources according to the semantic and compiler contracts.
 *
 * This grammar must not encode:
 *
 *     thread count;
 *     core count;
 *     device count;
 *     QPU topology;
 *     memory size;
 *     scheduling policy;
 *     physical placement.
 *
 * ============================================================================
 * INTEROPERABILITY CONTRACT
 * ============================================================================
 *
 * Interoperability formats such as:
 *
 *     OpenQASM
 *     QIR
 *     HDL formats
 *     LLVM-related representations
 *     MLIR-related representations
 *     foreign languages
 *
 * may contain conditional constructs.
 *
 * Their syntax must be translated into the Zamani domain-neutral AST and
 * semantic model rather than creating a competing Zamani conditional grammar.
 *
 * ============================================================================
 * DIALECT CONTRACT
 * ============================================================================
 *
 * Dialects may extend conditional semantics only through declared extension
 * mechanisms.
 *
 * A dialect MUST NOT silently redefine the precedence or associativity of
 * core conditional syntax.
 *
 * Any dialect extension affecting conditional parsing must specify:
 *
 *     - dialect identifier;
 *     - version;
 *     - feature gate;
 *     - grammar extension;
 *     - precedence relationship;
 *     - associativity;
 *     - AST mapping;
 *     - semantic mapping;
 *     - IR mapping;
 *     - compatibility behavior;
 *     - positive tests;
 *     - negative tests;
 *     - boundary tests.
 *
 * ============================================================================
 * MACRO / METAPROGRAMMING CONTRACT
 * ============================================================================
 *
 * Macros may generate conditional syntax only through the canonical parser
 * and AST pipeline.
 *
 * Macro expansion must not bypass:
 *
 *     - syntax validation;
 *     - structural validation;
 *     - semantic analysis;
 *     - type checking;
 *     - effect checking;
 *     - capability checking.
 *
 * Generated conditionals remain ordinary Zamani conditional expressions.
 *
 * ============================================================================
 * SOURCE PROVENANCE
 * ============================================================================
 *
 * Conditional expressions must retain source provenance sufficient for:
 *
 *     - diagnostics;
 *     - formatting;
 *     - debugging;
 *     - source maps;
 *     - compiler provenance;
 *     - deterministic reproduction.
 *
 * The grammar itself does not define the Rust source-span representation.
 *
 * ============================================================================
 * CANONICAL RULES
 * ============================================================================
 */

parser grammar ConditionalsParser;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC CONDITIONAL EXPRESSION
 * ============================================================================
 *
 * This is the only public conditional-expression entry point.
 *
 * The expression composition layer in expressions.g4 places this rule between
 * assignmentExpression and rangeExpression.
 */
conditionalExpression
    : ifExpression
    | ternaryConditionalExpression
    ;


/*
 * ============================================================================
 * STRUCTURED IF EXPRESSION
 * ============================================================================
 *
 * One initial condition and branch, followed by zero or more else-if branches
 * and an optional final else branch.
 *
 * Repetition provides an unbounded structural form without artificial limits.
 */
ifExpression
    : IF rangeExpression blockExpression
      elseIfExpressionBranch*
      elseExpressionBranch?
    ;


/*
 * ============================================================================
 * ELSE-IF BRANCH
 * ============================================================================
 *
 * Each occurrence represents exactly one:
 *
 *     else if condition { ... }
 *
 * branch.
 *
 * The parent rule repeats this rule as necessary.
 */
elseIfExpressionBranch
    : ELSE IF rangeExpression blockExpression
    ;


/*
 * ============================================================================
 * FINAL ELSE BRANCH
 * ============================================================================
 */
elseExpressionBranch
    : ELSE blockExpression
    ;


/*
 * ============================================================================
 * TERNARY CONDITIONAL EXPRESSION
 * ============================================================================
 *
 * The condition uses rangeExpression, which is the next-tighter precedence
 * layer established by expressions.g4.
 *
 * The true branch uses the complete expression grammar.
 *
 * The false branch uses conditionalExpression to establish right
 * associativity:
 *
 *     a ? b : c ? d : e
 *
 * becomes:
 *
 *     a ? b : (c ? d : e)
 *
 * Parentheses may be used for explicit alternative grouping where permitted
 * by the surrounding expression grammar.
 */
ternaryConditionalExpression
    : rangeExpression QUESTION expression COLON conditionalExpression
    ;