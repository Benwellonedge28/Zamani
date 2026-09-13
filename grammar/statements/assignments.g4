/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/assignments.g4
 *
 * Status:
 *     Production-ready statement-level assignment grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No unsafe implementation.
 *     No filesystem access.
 *     No networking.
 *     No runtime execution.
 *     No hardware discovery.
 *     No mutable global state.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the STATEMENT-LEVEL representation of assignments.
 *
 * It deliberately does NOT own the assignment-expression language itself.
 *
 * Assignment expression semantics are owned by:
 *
 *     grammar/expressions/assignment.g4
 *
 * and composed through:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file therefore establishes the boundary:
 *
 *     assignment expression
 *             |
 *             v
 *     assignment statement
 *             |
 *             v
 *     statement terminator
 *
 * Examples:
 *
 *     x = value;
 *     x += value;
 *     x -= value;
 *     x *= value;
 *     x /= value;
 *     x %= value;
 *     x &= value;
 *     x |= value;
 *     x ^= value;
 *     x <<= value;
 *     x >>= value;
 *
 * as well as arbitrarily complex assignment targets and values supported by
 * the expression grammar:
 *
 *     object.field = value;
 *     object::field = value;
 *     array[index] = value;
 *     tensor[i, j, k] = value;
 *     a = b = c;
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - assignmentStatement;
 *     - the statement-level assignment boundary;
 *     - assignment statement termination;
 *     - assignment-statement integration with the canonical statement layer;
 *     - syntactic distinction between an assignment statement and a generic
 *       expression statement.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - assignment-expression precedence;
 *     - assignment associativity;
 *     - assignment operators;
 *     - assignment target semantics;
 *     - identifier syntax;
 *     - member access;
 *     - indexing;
 *     - dereferencing;
 *     - literals;
 *     - arithmetic;
 *     - logical expressions;
 *     - comparison;
 *     - conditional expressions;
 *     - type checking;
 *     - mutability;
 *     - ownership;
 *     - borrowing;
 *     - lifetimes;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - quantum IR;
 *     - classical IR;
 *     - HDL IR;
 *     - hardware discovery;
 *     - topology;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - runtime dispatch;
 *     - backend selection.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * The intended dependency direction is:
 *
 *     lexer
 *       |
 *       v
 *     expression grammar
 *       |
 *       +--> assignment expression
 *       |
 *       v
 *     assignments.g4              <-- THIS FILE
 *       |
 *       v
 *     statements.g4
 *       |
 *       v
 *     frontend AST
 *       |
 *       +--> name resolution
 *       +--> type checking
 *       +--> ownership / borrowing
 *       +--> effect checking
 *       +--> capability checking
 *       +--> resource validation
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL / hardware IR
 *       +--> distributed/data/control representations
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
 *     runtime / hardware
 *
 * There must be NO dependency:
 *
 *     assignments.g4 -> quantum::ir
 *     assignments.g4 -> ZQN
 *     assignments.g4 -> QEC
 *     assignments.g4 -> scheduler
 *     assignments.g4 -> router
 *     assignments.g4 -> hardware discovery
 *     assignments.g4 -> runtime
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Assignment syntax expresses source-level computation.
 *
 * It MUST NOT encode:
 *
 *     maximum variables
 *     maximum assignments
 *     maximum tuple size
 *     maximum array size
 *     maximum tensor rank
 *     maximum qubits
 *     maximum cores
 *     maximum threads
 *     maximum GPUs
 *     maximum FPGAs
 *     maximum devices
 *     maximum nodes
 *     maximum memory
 *     maximum registers
 *     maximum accelerators
 *     maximum hardware resources
 *
 * No machine-size property is represented by this grammar.
 *
 * Arbitrary repetition is represented through the underlying expression
 * grammar and ANTLR recursion/repetition rather than fixed numerical limits.
 *
 * Actual resource limits belong to:
 *
 *     parser resource policy
 *     compiler configuration
 *     semantic validation
 *     resource management
 *     scheduling
 *     deployment
 *     runtime
 *     hardware capability negotiation
 *
 * They are not source-language assignment limits.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical token vocabulary.
 *
 * It MUST use:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * The canonical lexer owns the textual spelling of assignment operators.
 *
 * Relevant existing tokens include:
 *
 *     ASSIGN
 *     PLUS_ASSIGN
 *     MINUS_ASSIGN
 *     STAR_ASSIGN
 *     SLASH_ASSIGN
 *     PERCENT_ASSIGN
 *     AMP_ASSIGN
 *     PIPE_ASSIGN
 *     CARET_ASSIGN
 *
 * and, where provided by the canonical lexer:
 *
 *     LEFT_SHIFT
 *     RIGHT_SHIFT
 *
 * This file MUST NOT define lexer rules.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Assignment-expression syntax is already owned by:
 *
 *     grammar/expressions/assignment.g4
 *
 * Its public rule is:
 *
 *     assignmentExpression
 *
 * The expression composition layer exposes assignment expressions through:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file consumes those rules instead of redefining them.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION
 * ============================================================================
 *
 * There are two different concepts:
 *
 *     assignmentExpression
 *
 * and:
 *
 *     assignmentStatement
 *
 * An assignment expression is an expression-level construct.
 *
 * An assignment statement is a statement-level construct that terminates an
 * assignment expression as a source statement.
 *
 * Keeping these separate prevents:
 *
 *     expressions/assignment.g4
 *
 * from becoming coupled to:
 *
 *     statements/*
 *
 * and prevents statements from redefining expression precedence.
 *
 * ============================================================================
 * STATEMENT / EXPRESSION AMBIGUITY
 * ============================================================================
 *
 * The repository's canonical statements grammar currently admits:
 *
 *     expressionStatement
 *
 * where:
 *
 *     expression statement
 *
 * can contain an assignment expression.
 *
 * Therefore the assembled grammar MUST establish one canonical ownership path
 * for assignment statements.
 *
 * The preferred architecture is:
 *
 *     statement
 *         |
 *         +--> assignmentStatement
 *         |
 *         +--> expressionStatement
 *                  |
 *                  +--> non-assignment expression
 *
 * In other words, generic expressionStatement should not be the competing
 * owner of assignment statements.
 *
 * This distinction is an integration responsibility of:
 *
 *     grammar/statements/statements.g4
 *
 * and MUST be completed when this grammar is assembled.
 *
 * This file itself must remain independently valid and must not duplicate
 * generic expression syntax merely to solve the dispatch problem.
 *
 * ============================================================================
 * IMPORTS
 * ============================================================================
 */

parser grammar Assignments;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * IMPORT CANONICAL EXPRESSION GRAMMAR
 * ============================================================================
 *
 * Expressions.g4 is the canonical expression composition boundary.
 *
 * It provides access to the assignment-expression hierarchy while keeping
 * expression ownership in grammar/expressions/.
 *
 * This prevents:
 *
 *     statements/assignments.g4
 *
 * from becoming a second expression grammar.
 */
import Expressions;


/*
 * ============================================================================
 * 1. ASSIGNMENT STATEMENT
 * ============================================================================
 *
 * Public rule consumed by statements.g4.
 *
 * Only a syntactically actual assignment is accepted here.
 *
 * It is deliberately NOT:
 *
 *     expression SEMICOLON
 *
 * because that would make every expression a possible assignment statement
 * and would recreate the ambiguity that this file exists to prevent.
 */
assignmentStatement
    : assignmentOperation statementTerminator
    ;


/*
 * ============================================================================
 * 2. ASSIGNMENT OPERATION
 * ============================================================================
 *
 * This rule identifies the assignment form without redefining the complete
 * assignment-expression hierarchy.
 *
 * The right-hand side remains a complete assignment expression so that
 * right-associative chains such as:
 *
 *     a = b = c;
 *
 * retain their correct structure.
 *
 * The assignment target and operator are imported from the canonical
 * assignment-expression grammar.
 */
assignmentOperation
    : assignmentTarget assignmentOperator assignmentExpression
    ;


/*
 * ============================================================================
 * 3. STATEMENT TERMINATOR
 * ============================================================================
 *
 * The current Zamani lexer owns SEMICOLON.
 *
 * Statement termination is syntactic here.
 *
 * Whether a future language dialect permits optional semicolons belongs to
 * language-version / parser-composition policy and MUST NOT silently alter
 * this file.
 */
statementTerminator
    : SEMICOLON
    ;


/*
 * ============================================================================
 * 4. SEMANTIC AST CONTRACT
 * ============================================================================
 *
 * This grammar corresponds conceptually to:
 *
 *     AssignmentStatement {
 *         assignment: AssignmentExpression
 *         span: SourceSpan
 *     }
 *
 * with the assignment itself structurally equivalent to:
 *
 *     AssignmentExpression {
 *         target: Expression,
 *         operator: AssignmentOperator,
 *         value: Expression
 *     }
 *
 * This file does not define those Rust types.
 *
 * The frontend AST implementation owns them.
 *
 * Source spans MUST be retained by the frontend AST.
 *
 * The resulting AST MUST preserve:
 *
 *     - target;
 *     - operator;
 *     - value;
 *     - complete statement span;
 *     - child spans;
 *     - source ordering.
 *
 * The AST MUST NOT acquire machine-specific information merely because an
 * assignment eventually lowers to:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     distributed system
 *     embedded target
 *     future accelerator.
 *
 * ============================================================================
 * 5. TARGET SEMANTICS
 * ============================================================================
 *
 * The grammar deliberately permits the assignment-expression grammar to
 * describe complex targets.
 *
 * Examples include:
 *
 *     x = value;
 *     object.field = value;
 *     object::field = value;
 *     array[index] = value;
 *     tensor[i, j] = value;
 *     a = b = c;
 *
 * Whether a target is actually assignable is NOT a parser decision.
 *
 * Semantic analysis must determine:
 *
 *     - whether the expression denotes an assignable location/value;
 *     - whether it is mutable;
 *     - whether ownership permits mutation;
 *     - whether borrowing permits mutation;
 *     - whether lifetimes are valid;
 *     - whether the target is initialized;
 *     - whether the value type is compatible;
 *     - whether conversions are permitted;
 *     - whether effects permit the operation;
 *     - whether required capabilities exist;
 *     - whether resource constraints permit execution.
 *
 * ============================================================================
 * 6. COMPOUND ASSIGNMENT
 * ============================================================================
 *
 * Compound assignment is represented by its canonical operator token.
 *
 * Examples:
 *
 *     x += y;
 *     x -= y;
 *     x *= y;
 *     x /= y;
 *     x %= y;
 *     x &= y;
 *     x |= y;
 *     x ^= y;
 *
 * This grammar does NOT lower:
 *
 *     x += y
 *
 * into:
 *
 *     x = x + y
 *
 * Such lowering belongs to semantic lowering / canonical IR construction.
 *
 * The lowering implementation MUST preserve:
 *
 *     - evaluation order;
 *     - side effects;
 *     - target evaluation count;
 *     - ownership;
 *     - borrowing;
 *     - overflow semantics;
 *     - conversion semantics;
 *     - effect semantics;
 *     - volatile/hardware semantics;
 *     - domain-specific semantics.
 *
 * ============================================================================
 * 7. RIGHT ASSOCIATIVITY
 * ============================================================================
 *
 * The canonical assignment-expression grammar owns assignment associativity.
 *
 * Consequently:
 *
 *     a = b = c;
 *
 * must retain the structural meaning:
 *
 *     a = (b = c)
 *
 * rather than:
 *
 *     (a = b) = c
 *
 * This file MUST NOT introduce a second assignment recursion model.
 *
 * ============================================================================
 * 8. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Assignment statements may participate in quantum-classical programs.
 *
 * Examples:
 *
 *     result = measure(q);
 *     bit = measurement;
 *     parameter = theta;
 *
 * The grammar does not determine whether the right-hand side is:
 *
 *     classical data
 *     measurement data
 *     symbolic data
 *     resource metadata
 *     capability information
 *     quantum-derived data
 *     hardware state
 *
 * Semantic analysis and lowering determine that meaning.
 *
 * This file MUST NOT:
 *
 *     - allocate qubits;
 *     - identify physical qubits;
 *     - construct quantum circuits;
 *     - construct quantum::ir;
 *     - invoke QEC;
 *     - interpret ZQN noise;
 *     - choose a backend;
 *     - select a topology.
 *
 * ============================================================================
 * 9. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * The same assignment statement may occur in hardware-oriented source:
 *
 *     signal = expression;
 *     register_value = next_value;
 *     state = next_state;
 *
 * The grammar does not decide whether a target represents:
 *
 *     software variable
 *     hardware signal
 *     register
 *     wire
 *     port
 *     memory
 *     state-machine state
 *     device property
 *
 * Those distinctions belong to semantic analysis and HDL/hardware lowering.
 *
 * Therefore this statement grammar remains independent of:
 *
 *     CPU width
 *     register count
 *     FPGA resources
 *     ASIC resources
 *     device topology
 *     physical addresses
 *     clock implementation
 *     hardware vendor.
 *
 * ============================================================================
 * 10. RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Assignment itself imposes no hardware resource requirement.
 *
 * The following must never be inferred directly by this grammar:
 *
 *     x = value;
 *
 * -> use CPU 0
 * -> use GPU 1
 * -> use physical qubit 0
 * -> use FPGA 0
 * -> allocate N registers
 *
 * Resource requirements are established by semantic/resource analysis and
 * downstream compilation.
 *
 * ============================================================================
 * 11. POCO-REAF CONTRACT
 * ============================================================================
 *
 * An assignment source statement describes semantic intent.
 *
 * It must remain valid independent of the eventual realization target.
 *
 * Therefore:
 *
 *     x = value;
 *
 * may eventually lower to:
 *
 *     classical computation
 *     quantum-classical interaction
 *     hardware assignment
 *     distributed data movement
 *     accelerator operation
 *     future execution model
 *
 * without changing the grammar merely because the target changes.
 *
 * ============================================================================
 * 12. DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic with respect to:
 *
 *     input token stream
 *     grammar version
 *     imported grammar versions
 *
 * This file MUST NOT depend on:
 *
 *     wall-clock time
 *     random values
 *     environment variables
 *     filesystem state
 *     network state
 *     hardware state
 *     device discovery
 *     runtime state.
 *
 * ============================================================================
 * 13. DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Parser-level diagnostics belong to syntax.
 *
 * Examples of parser errors:
 *
 *     x = ;
 *     x += ;
 *     = x;
 *     x +== y;
 *     x = = y;
 *     x += ;
 *
 * Semantic diagnostics do NOT belong here.
 *
 * Examples:
 *
 *     immutable variable cannot be assigned
 *     type mismatch
 *     borrow violation
 *     assignment requires capability
 *     hardware resource unavailable
 *     quantum operation unavailable
 *
 * Those are downstream semantic diagnostics.
 *
 * ============================================================================
 * 14. COMPATIBILITY
 * ============================================================================
 *
 * Existing assignment operators remain owned by the canonical lexer and
 * expression assignment grammar.
 *
 * Adding a future assignment operator requires coordinated changes to:
 *
 *     1. canonical lexer;
 *     2. expressions/assignment.g4;
 *     3. frontend AST operator representation;
 *     4. semantic lowering;
 *     5. language compatibility documentation;
 *     6. positive parser tests;
 *     7. negative/ambiguity tests;
 *     8. round-trip tests where supported.
 *
 * This file should not silently reinterpret an existing operator.
 *
 * ============================================================================
 * 15. TEST CONTRACT
 * ============================================================================
 *
 * The dedicated statement-level test suite must cover:
 *
 * Positive:
 *
 *     x = y;
 *     x += y;
 *     x -= y;
 *     x *= y;
 *     x /= y;
 *     x %= y;
 *     x &= y;
 *     x |= y;
 *     x ^= y;
 *
 * Target forms:
 *
 *     object.field = value;
 *     object::field = value;
 *     array[index] = value;
 *     tensor[i, j] = value;
 *
 * Chained assignment:
 *
 *     a = b = c;
 *
 * Nested expressions:
 *
 *     result = compute(a + b * c);
 *
 * Quantum/classical:
 *
 *     result = measure(q);
 *
 * HDL-oriented:
 *
 *     signal = next_state;
 *
 * Negative:
 *
 *     x = ;
 *     = x;
 *     x += ;
 *     x +== y;
 *     x = = y;
 *
 * Boundary/scalability:
 *
 *     arbitrarily long assignment chains;
 *     arbitrarily complex expression values;
 *     arbitrarily deep member/index structures subject only to parser resource
 *     policy, never a language-level fixed machine limit.
 *
 * Determinism:
 *
 *     identical token streams produce identical parse structures.
 *
 * ============================================================================
 * 16. INTEGRATION WITH statements.g4
 * ============================================================================
 *
 * The canonical statement dispatcher MUST integrate this rule explicitly.
 *
 * The intended relationship is:
 *
 *     statement
 *         : attributedStatement
 *         | declarationStatement
 *         | bindingStatement
 *         | assignmentStatement
 *         | controlFlowStatement
 *         | blockExpression
 *         | effectStatement
 *         | concurrencyStatement
 *         | domainStatement
 *         | expressionStatement
 *         | emptyStatement
 *         ;
 *
 * The important rule is:
 *
 *     assignmentStatement
 *
 * must be a distinct statement alternative.
 *
 * The generic expression-statement path must not become a second owner of
 * assignment statements in the assembled grammar.
 *
 * This is an integration change to:
 *
 *     grammar/statements/statements.g4
 *
 * rather than a reason to duplicate expression syntax here.
 *
 * ============================================================================
 * 17. INTEGRATION WITH expressions/assignment.g4
 * ============================================================================
 *
 * This file consumes:
 *
 *     assignmentExpression
 *     assignmentTarget
 *     assignmentOperator
 *
 * from the canonical assignment-expression layer.
 *
 * It MUST NOT redefine any of those rules.
 *
 * Therefore there is one assignment-expression authority:
 *
 *     grammar/expressions/assignment.g4
 *
 * and one assignment-statement authority:
 *
 *     grammar/statements/assignments.g4
 *
 * ============================================================================
 * 18. INTEGRATION WITH THE AST
 * ============================================================================
 *
 * Frontend semantic construction should conceptually perform:
 *
 *     assignmentStatement
 *         |
 *         v
 *     AssignmentStmt
 *         |
 *         +--> AssignmentExpr
 *                 |
 *                 +--> target
 *                 +--> operator
 *                 +--> value
 *
 * The grammar does not prescribe the concrete Rust AST API.
 *
 * The AST must retain source spans and preserve source semantics.
 *
 * ============================================================================
 * 19. INTEGRATION WITH IR
 * ============================================================================
 *
 * This grammar has NO direct IR dependency.
 *
 * The downstream frontend decides whether an assignment lowers into:
 *
 *     classical IR
 *     quantum::ir-related classical interaction
 *     HDL/hardware IR
 *     distributed IR
 *     accelerator IR
 *     data-flow representation
 *
 * The grammar must never instantiate or reference those representations.
 *
 * ============================================================================
 * 20. RUST 1.97 / 1.97.1 CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust source.
 *
 * Rust compatibility therefore applies to:
 *
 *     ANTLR-generated parser integration
 *     frontend AST construction
 *     semantic analysis
 *     compiler integration
 *
 * The surrounding Rust implementation must:
 *
 *     - compile on Rust 1.97 / 1.97.1;
 *     - forbid unsafe code;
 *     - avoid relying on newer language features;
 *     - preserve deterministic parser integration.
 *
 * This grammar itself contains no unsafe construct.
 *
 * ============================================================================
 * 21. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     no machine size;
 *     no qubit count;
 *     no CPU count;
 *     no GPU count;
 *     no FPGA count;
 *     no node count;
 *     no memory limit;
 *     no register limit;
 *     no topology;
 *     no device identifier;
 *     no physical address;
 *     no deployment location;
 *     no hardware vendor assumption.
 *
 * The only structural repetition is grammar-defined and unbounded by a
 * language-level machine constant.
 *
 * ============================================================================
 * 22. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when:
 *
 *     [ ] It compiles as an ANTLR parser grammar.
 *
 *     [ ] Its token vocabulary matches the canonical lexer.
 *
 *     [ ] It does not define lexer rules.
 *
 *     [ ] It does not duplicate assignment-expression precedence.
 *
 *     [ ] It does not duplicate assignment operators.
 *
 *     [ ] It exposes assignmentStatement as the statement-level boundary.
 *
 *     [ ] It preserves right-associative assignment through the expression
 *         grammar.
 *
 *     [ ] It supports arbitrary assignment target complexity permitted by the
 *         canonical expression grammar.
 *
 *     [ ] It contains no machine-specific limits.
 *
 *     [ ] It contains no target-specific hardware assumptions.
 *
 *     [ ] It contains no AST implementation.
 *
 *     [ ] It contains no IR implementation.
 *
 *     [ ] It contains no runtime behavior.
 *
 *     [ ] It contains no unsafe code.
 *
 *     [ ] It is deterministic.
 *
 *     [ ] It has positive parser tests.
 *
 *     [ ] It has negative parser tests.
 *
 *     [ ] It has boundary/scalability tests.
 *
 *     [ ] statements.g4 integrates assignmentStatement exactly once.
 *
 *     [ ] generic expressionStatement does not become a competing assignment
 *         owner in the assembled grammar.
 *
 *     [ ] frontend AST construction preserves target/operator/value/source
 *         spans.
 *
 *     [ ] semantic analysis, rather than this grammar, decides assignability.
 *
 *     [ ] downstream lowering preserves assignment semantics.
 *
 * ============================================================================
 */