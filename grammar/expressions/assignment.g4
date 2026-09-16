/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/assignment.g4
 *
 * Status:
 *     Production parser grammar component.
 *
 * Purpose:
 *     Canonical source-level assignment-expression grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar.
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only.
 *     No unsafe Rust is required or permitted.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This file owns ONLY assignment-expression syntax.
 *
 * Source pipeline:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     parser
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
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *     classical IR          quantum::ir          HDL/hardware IR
 *          |                      |                      |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                   optimization / lowering
 *                                 |
 *                   routing / scheduling
 *                                 |
 *                    resilience / QEC / ZQN
 *                                 |
 *                                HAL
 *                                 |
 *                         target realization
 *
 * This grammar does NOT:
 *
 *     - construct AST nodes;
 *     - perform name resolution;
 *     - perform type checking;
 *     - perform ownership checking;
 *     - perform borrow checking;
 *     - perform effect checking;
 *     - perform capability checking;
 *     - allocate resources;
 *     - select hardware;
 *     - select physical qubits;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform optimization;
 *     - perform QEC;
 *     - implement ZQN;
 *     - implement HAL;
 *     - execute programs;
 *     - impose hardware limits.
 *
 * quantum::ir remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *
 *     assignmentExpression
 *     assignmentTarget
 *     assignmentOperator
 *     assignment precedence
 *     assignment associativity
 *
 * DOES NOT OWN:
 *
 *     identifiers
 *     literals
 *     member access
 *     indexing
 *     calls
 *     postfix expressions
 *     unary expressions
 *     arithmetic expressions
 *     bitwise expressions
 *     comparisons
 *     logical expressions
 *     conditional expressions
 *     types
 *     semantic assignability
 *     ownership
 *     resources
 *     hardware
 *     quantum semantics
 *
 * ============================================================================
 * EXPRESSION HIERARCHY
 * ============================================================================
 *
 * The canonical hierarchy is:
 *
 *     expression
 *         |
 *         v
 *     assignmentExpression       <-- THIS FILE
 *         |
 *         v
 *     conditionalExpression
 *         |
 *         v
 *     range / logical / bitwise
 *         |
 *         v
 *     comparison
 *         |
 *         v
 *     shift
 *         |
 *         v
 *     additive
 *         |
 *         v
 *     multiplicative
 *         |
 *         v
 *     prefix
 *         |
 *         v
 *     postfix
 *         |
 *         v
 *     primary
 *
 * Assignment has the lowest precedence in this ordinary-expression hierarchy.
 *
 * ============================================================================
 * RIGHT ASSOCIATIVITY
 * ============================================================================
 *
 * Assignment is right associative.
 *
 * Therefore:
 *
 *     a = b = c
 *
 * has the structure:
 *
 *     a = (b = c)
 *
 * and:
 *
 *     a += b += c
 *
 * has the structure:
 *
 *     a += (b += c)
 *
 * This is achieved by recursive use of assignmentExpression on the RHS.
 *
 * No finite assignment-chain limit is encoded.
 *
 * ============================================================================
 * ASSIGNMENT TARGET MODEL
 * ============================================================================
 *
 * The grammar deliberately accepts a complete postfixExpression as the
 * syntactic assignment target.
 *
 * This permits syntax such as:
 *
 *     x = value
 *     object.field = value
 *     array[index] = value
 *     tensor[i, j] = value
 *     reference.field[index] = value
 *
 * and future target forms without requiring this grammar to know the complete
 * semantic storage model.
 *
 * Semantic analysis MUST subsequently determine whether the target is actually
 * assignable.
 *
 * Consequently syntax such as:
 *
 *     call() = value
 *
 * may be structurally recognized by the parser and then rejected by semantic
 * analysis.
 *
 * This separation is intentional.
 *
 * It supports:
 *
 *     - user-defined lvalues;
 *     - property assignment;
 *     - indexing abstractions;
 *     - references;
 *     - ownership-aware assignment;
 *     - hardware/HDL targets;
 *     - memory abstractions;
 *     - future language extensions.
 *
 * ============================================================================
 * COMPOUND ASSIGNMENT
 * ============================================================================
 *
 * The currently authoritative ZamaniLexer provides:
 *
 *     ASSIGN
 *     PLUS_ASSIGN
 *     MINUS_ASSIGN
 *     STAR_ASSIGN
 *     SLASH_ASSIGN
 *
 * This file therefore consumes exactly those tokens.
 *
 * IMPORTANT:
 *
 * The repository also contains modular lexical specifications describing
 * additional compound operators such as:
 *
 *     %=
 *     &=
 *     |=
 *     ^=
 *     <<=
 *     >>=
 *
 * Those additional spellings MUST NOT be referenced here until the canonical
 * lexer used by this parser exposes stable token names for them.
 *
 * This prevents this parser component from compiling against a vocabulary
 * different from the repository's current lexical authority.
 *
 * When those operators are promoted into the canonical lexer, they may be
 * added here without changing the assignment architecture.
 *
 * ============================================================================
 * COMPOUND ASSIGNMENT SEMANTICS
 * ============================================================================
 *
 * The parser represents:
 *
 *     x += y
 *
 * as a compound assignment operation.
 *
 * It MUST NOT silently rewrite it to:
 *
 *     x = x + y
 *
 * Grammar-level syntax does not establish that equivalence.
 *
 * Semantic lowering must preserve:
 *
 *     - evaluation order;
 *     - side effects;
 *     - target evaluation;
 *     - ownership;
 *     - borrowing;
 *     - aliasing;
 *     - overflow behavior;
 *     - conversion rules;
 *     - user-defined operator semantics;
 *     - volatile/hardware semantics;
 *     - domain-specific semantics.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar establishes no universal finite limits.
 *
 * It contains no:
 *
 *     MAX_ASSIGNMENTS
 *     MAX_VARIABLES
 *     MAX_TARGETS
 *     MAX_TUPLE_ARITY
 *     MAX_ARRAY_SIZE
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTER_COUNT
 *     MAX_DEVICES
 *     MAX_ACCELERATORS
 *
 * Assignment chains are represented recursively.
 *
 * Target complexity is inherited from postfix-expression syntax and therefore
 * has no artificial language-level finite ceiling here.
 *
 * "Infinity" means that the language grammar introduces no artificial finite
 * semantic limit. Actual parser/compiler/runtime resource budgets remain
 * implementation policy and available-resource concerns.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It defines NO lexer rules.
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Required tokens:
 *
 *     ASSIGN
 *     PLUS_ASSIGN
 *     MINUS_ASSIGN
 *     STAR_ASSIGN
 *     SLASH_ASSIGN
 *
 * The lexer owns:
 *
 *     - textual spelling;
 *     - maximal-munch behavior;
 *     - token identity;
 *     - source positions.
 *
 * This grammar owns:
 *
 *     - assignment syntax;
 *     - precedence;
 *     - associativity.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every assignment recognized here must lower to the domain-neutral frontend
 * AST as an assignment expression equivalent to:
 *
 *     AssignmentExpression {
 *         target: Expression,
 *         operator: AssignmentOperator,
 *         value: Expression,
 *         source: SourceSpan
 *     }
 *
 * The exact Rust AST type is owned by the frontend AST implementation.
 *
 * The grammar must preserve enough parse structure for the AST builder to
 * recover:
 *
 *     - target;
 *     - operator;
 *     - value;
 *     - source span;
 *     - child source spans;
 *     - source ordering;
 *     - nesting.
 *
 * The grammar does not introduce domain-specific AST variants merely because
 * an assignment occurs in:
 *
 *     classical code;
 *     quantum code;
 *     HDL;
 *     hardware code;
 *     distributed code;
 *     AI/data code;
 *     accelerator code.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "Is this syntactically an assignment expression?"
 *
 * Semantic analysis answers:
 *
 *     "Is this assignment valid and what does it mean?"
 *
 * Semantic analysis owns:
 *
 *     - name resolution;
 *     - target resolution;
 *     - mutability;
 *     - type compatibility;
 *     - implicit conversion;
 *     - ownership;
 *     - borrowing;
 *     - aliasing;
 *     - effects;
 *     - capability requirements;
 *     - resource requirements;
 *     - domain legality;
 *     - quantum/classical boundaries;
 *     - HDL/hardware legality;
 *     - compound-assignment lowering.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Assignment syntax remains domain-neutral.
 *
 * Examples may include:
 *
 *     result = measure(q)
 *     angle = parameter
 *     classical_bit = measurement
 *
 * The grammar does not decide whether a value is:
 *
 *     - classical;
 *     - quantum-derived;
 *     - symbolic;
 *     - logical;
 *     - resource metadata;
 *     - hardware state.
 *
 * Those decisions belong downstream.
 *
 * This file MUST NOT:
 *
 *     - create a quantum IR;
 *     - enumerate quantum gates;
 *     - select physical qubits;
 *     - encode qubit limits;
 *     - perform QEC;
 *     - perform routing;
 *     - perform scheduling;
 *     - implement ZQN.
 *
 * Any quantum-related assignment eventually reaches the existing canonical
 * quantum::ir boundary through semantic lowering.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Assignment syntax is reusable for:
 *
 *     software variables;
 *     HDL signals;
 *     registers;
 *     ports;
 *     state;
 *     memory abstractions;
 *     accelerator state;
 *     hardware/software co-design.
 *
 * The grammar does not determine which category a target belongs to.
 *
 * For example:
 *
 *     signal = next_value
 *
 * and:
 *
 *     variable = next_value
 *
 * have the same assignment-level syntax.
 *
 * Semantic/domain analysis distinguishes their meaning.
 *
 * No register width, bus width, clock frequency, device ID, physical address,
 * FPGA family, ASIC technology, or topology is encoded here.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Assignment syntax does not imply a particular machine resource.
 *
 * This:
 *
 *     x = value
 *
 * MUST NOT implicitly mean:
 *
 *     use CPU core N
 *     use GPU N
 *     use physical qubit N
 *     use FPGA resource N
 *     use memory bank N
 *     use device N
 *
 * Resource and capability decisions occur downstream through:
 *
 *     semantic analysis
 *     resource requirements
 *     capability analysis
 *     compilation
 *     placement
 *     routing
 *     scheduling
 *     deployment
 *
 * This preserves POCO-REAF.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For an identical token stream and language version, this grammar must produce
 * the same syntactic structure.
 *
 * Parsing must not depend on:
 *
 *     - wall-clock time;
 *     - randomness;
 *     - environment variables;
 *     - CPU count;
 *     - GPU count;
 *     - FPGA availability;
 *     - QPU availability;
 *     - device state;
 *     - calibration state;
 *     - network state;
 *     - scheduler state;
 *     - runtime state.
 *
 * ============================================================================
 * DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Parser diagnostics are limited to syntactic structure.
 *
 * Examples of parser-level errors:
 *
 *     = x
 *     x =
 *     x +=
 *     x +== y
 *     x = = y
 *
 * Semantic diagnostics belong downstream:
 *
 *     immutable target
 *     target not assignable
 *     type mismatch
 *     invalid conversion
 *     ownership violation
 *     borrow violation
 *     unavailable capability
 *     unavailable resource
 *     illegal quantum operation
 *     illegal hardware operation
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Assignment precedence and associativity are language compatibility
 * guarantees.
 *
 * Existing valid forms must retain their meaning:
 *
 *     x = y
 *     x += y
 *     x -= y
 *     x *= y
 *     x /= y
 *
 * Future assignment operators must be introduced through the normal language
 * versioning process:
 *
 *     1. canonical lexer token;
 *     2. assignmentOperator addition;
 *     3. AST operator representation;
 *     4. semantic lowering;
 *     5. specification update;
 *     6. positive tests;
 *     7. negative tests;
 *     8. ambiguity tests;
 *     9. compatibility documentation.
 *
 * No existing operator may silently change meaning.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file is designed to be imported by the canonical expression parser.
 *
 * The expression composition grammar must contain:
 *
 *     import AssignmentExpressions;
 *
 * and must own the public:
 *
 *     expression
 *
 * entry point.
 *
 * It must NOT redeclare:
 *
 *     assignmentExpression
 *     assignmentTarget
 *     assignmentOperator
 *
 * after importing this grammar.
 *
 * The importing expression grammar owns the lower expression hierarchy:
 *
 *     conditionalExpression
 *     rangeExpression
 *     logicalOrExpression
 *     logicalAndExpression
 *     bitwiseOrExpression
 *     bitwiseXorExpression
 *     bitwiseAndExpression
 *     equalityExpression
 *     relationalExpression
 *     shiftExpression
 *     additiveExpression
 *     multiplicativeExpression
 *     prefixExpression
 *     postfixExpression
 *     primaryExpression
 *
 * This file intentionally references those downstream rules without
 * duplicating them.
 *
 * ============================================================================
 * INTEGRATION WITH EXPRESSIONS/EXPRESSIONS.G4
 * ============================================================================
 *
 * The repository currently contains a broader expression grammar in:
 *
 *     grammar/expressions/expressions.g4
 *
 * That file currently contains its own assignmentExpression rule.
 *
 * To make this file the single authority, the composition grammar must
 * eventually:
 *
 *     import AssignmentExpressions;
 *
 * and remove its duplicate assignmentExpression, assignmentTarget and
 * assignmentOperator definitions.
 *
 * The lower expression rules remain owned by the composition grammar or their
 * respective imported grammar components.
 *
 * No second assignment implementation should remain.
 *
 * ============================================================================
 * INTEGRATION WITH STATEMENTS/ASSIGNMENTS.G4
 * ============================================================================
 *
 * Statement-level assignment syntax MUST NOT duplicate assignment-expression
 * operator definitions.
 *
 * A statement grammar may consume:
 *
 *     assignmentExpression
 *
 * as an expression statement or other statement-level construct.
 *
 * It must not create another:
 *
 *     assignmentOperator
 *
 * vocabulary.
 *
 * This guarantees that:
 *
 *     expression assignment
 *
 * and:
 *
 *     statement assignment
 *
 * have exactly one syntactic authority.
 *
 * ============================================================================
 * INTEGRATION WITH ARITHMETIC.G4
 * ============================================================================
 *
 * arithmetic.g4 owns:
 *
 *     additiveExpression
 *     multiplicativeExpression
 *     exponentExpression
 *
 * It MUST NOT consume:
 *
 *     PLUS_ASSIGN
 *     MINUS_ASSIGN
 *     STAR_ASSIGN
 *     SLASH_ASSIGN
 *
 * Those belong exclusively to this file.
 *
 * Therefore:
 *
 *     x += y
 *
 * cannot accidentally be interpreted as an arithmetic expression.
 *
 * ============================================================================
 * INTEGRATION WITH BITWISE / LOGICAL GRAMMARS
 * ============================================================================
 *
 * Bitwise and logical expression grammars own their binary operators.
 *
 * They MUST NOT consume assignment operators.
 *
 * Assignment remains the outer/lowest ordinary expression layer.
 *
 * ============================================================================
 * INTEGRATION WITH TYPES
 * ============================================================================
 *
 * This file consumes expressions only.
 *
 * It does not inspect:
 *
 *     typeExpression
 *
 * and does not decide:
 *
 *     assignability;
 *     coercion;
 *     mutability;
 *     ownership.
 *
 * Type and ownership analysis occurs after AST construction.
 *
 * ============================================================================
 * INTEGRATION WITH AST / FRONTEND
 * ============================================================================
 *
 * The frontend parser/AST builder must map:
 *
 *     assignmentExpression
 *
 * to the repository's generic/domain-neutral expression representation.
 *
 * The frontend AST must not introduce a second quantum assignment IR.
 *
 * If the RHS or target eventually participates in quantum computation, semantic
 * lowering continues toward the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 * INTEGRATION WITH COMPILER / RUNTIME
 * ============================================================================
 *
 * This grammar has no direct compiler or runtime dependency.
 *
 * Compiler stages consume semantic/IR representations produced downstream.
 *
 * Runtime stages consume lowered executable representations.
 *
 * Therefore this file remains unchanged when:
 *
 *     CPU backends are added;
 *     GPU backends are added;
 *     FPGA backends are added;
 *     QPU backends are added;
 *     distributed backends are added;
 *     future architectures are added.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * No source-level hardware or resource limit is present.
 *
 * No:
 *
 *     MAX_*
 *     fixed qubit count
 *     fixed CPU count
 *     fixed GPU count
 *     fixed FPGA count
 *     fixed node count
 *     fixed memory size
 *     fixed register count
 *     fixed tensor dimension
 *     fixed device identifier
 *     fixed topology
 *
 * occurs in the grammar rules.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The following MUST parse:
 *
 *     x = y
 *     x += y
 *     x -= y
 *     x *= y
 *     x /= y
 *
 * Right associativity:
 *
 *     a = b = c
 *     a += b += c
 *     a -= b = c
 *
 * Complex targets:
 *
 *     object.field = value
 *     object.field += value
 *     array[index] = value
 *     tensor[i, j] = value
 *     object.field[index].value = expression
 *
 * RHS precedence:
 *
 *     x = a + b
 *     x = a * b
 *     x = a && b
 *     x = a | b
 *     x = condition ? a : b
 *
 * Nested expressions:
 *
 *     x = (a = b)
 *     x = f(a = b)
 *     x = array[i = j]
 *
 * Syntax that MUST be rejected at the assignment-expression level:
 *
 *     = x
 *     x =
 *     x +=
 *     x +== y
 *     x = = y
 *     x += = y
 *
 * Semantic-only invalidity MUST remain semantic:
 *
 *     1 = x
 *     f() = x
 *
 * provided those forms are structurally accepted by postfixExpression.
 *
 * ============================================================================
 * BOUNDARY / SCALABILITY TESTS
 * ============================================================================
 *
 * Tests must include:
 *
 *     - long right-associative assignment chains;
 *     - deeply nested postfix targets;
 *     - deeply nested RHS expressions;
 *     - generated large expression trees;
 *     - long qualified/member/indexing chains;
 *     - assignments involving symbolic values;
 *     - assignments involving tensor values;
 *     - assignments involving quantum-derived values;
 *     - assignments in HDL contexts;
 *     - assignments in distributed contexts.
 *
 * No test may require an arbitrary grammar maximum merely to pass.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] Assignment syntax has one authoritative owner.
 *     [x] Assignment is right associative.
 *     [x] Assignment has lower precedence than conditional expressions.
 *     [x] Assignment targets use postfixExpression.
 *     [x] Assignment operators use the canonical lexer vocabulary.
 *     [x] No lexer rules are duplicated here.
 *     [x] No semantic logic is embedded here.
 *     [x] No AST implementation is embedded here.
 *     [x] No IR implementation is embedded here.
 *     [x] quantum::ir remains downstream and canonical.
 *     [x] QEC/ZQN/routing/scheduling remain downstream.
 *     [x] Hardware realization remains downstream.
 *     [x] No artificial machine limits exist.
 *     [x] No Rust actions exist.
 *     [x] No unsafe code is required.
 *     [x] Rust 1.97 / 1.97.1 compatibility is preserved downstream.
 *     [x] Integration ownership is explicitly defined.
 *     [x] Future assignment operators have a defined promotion path.
 *
 * ============================================================================
 */

parser grammar AssignmentExpressions;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * PUBLIC ASSIGNMENT EXPRESSION
 * ============================================================================
 *
 * The first alternative is the non-assignment case.
 *
 * The second alternative recursively consumes another assignmentExpression on
 * the RHS, making assignment right associative.
 *
 *     a = b = c
 *
 * therefore becomes:
 *
 *     a = (b = c)
 *
 * rather than:
 *
 *     (a = b) = c
 *
 * ============================================================================
 */

assignmentExpression
    : conditionalExpression
    | assignmentTarget assignmentOperator assignmentExpression
    ;


/* ============================================================================
 * ASSIGNMENT TARGET
 * ============================================================================
 *
 * `postfixExpression` is owned by the postfix-expression layer.
 *
 * Semantic analysis determines whether the resulting expression denotes a
 * valid assignable target.
 *
 * ============================================================================
 */

assignmentTarget
    : postfixExpression
    ;


/* ============================================================================
 * ASSIGNMENT OPERATOR
 * ============================================================================
 *
 * These are the compound-assignment tokens currently exposed by the
 * authoritative ZamaniLexer.
 *
 * Do not introduce token aliases here.
 *
 * ============================================================================
 */

assignmentOperator
    : ASSIGN
    | PLUS_ASSIGN
    | MINUS_ASSIGN
    | STAR_ASSIGN
    | SLASH_ASSIGN
    ;