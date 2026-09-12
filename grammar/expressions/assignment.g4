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
 *     Own the complete source-level assignment-expression syntax.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar.
 *
 * Rust integration:
 *     Rust 1.97 / Rust 1.97.1.
 *
 * Safety:
 *     No embedded Rust actions.
 *     No semantic predicates requiring Rust code.
 *     No unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * Canonical Zamani lexer
 *   |
 *   v
 * Parser
 *   |
 *   v
 * Frontend AST
 *   |
 *   +--> name resolution
 *   +--> type checking
 *   +--> mutability / ownership checking
 *   +--> effect checking
 *   +--> capability checking
 *   +--> resource validation
 *   |
 *   v
 * Canonical semantic representation
 *   |
 *   +--> classical IR
 *   +--> quantum::ir
 *   +--> hardware/resource IR
 *   +--> control/data IR
 *   |
 *   v
 * optimization
 *   |
 *   v
 * lowering / routing / scheduling
 *   |
 *   v
 * target execution
 *
 * THIS FILE MUST NOT:
 *
 *   - define an AST implementation;
 *   - define classical IR;
 *   - define quantum::ir;
 *   - select a hardware target;
 *   - allocate a qubit;
 *   - select a physical qubit;
 *   - perform routing;
 *   - perform scheduling;
 *   - perform optimization;
 *   - perform QEC;
 *   - define ZQN semantics;
 *   - discover hardware;
 *   - impose machine limits.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *
 *   - assignmentExpression;
 *   - assignmentOperator;
 *   - assignment precedence;
 *   - assignment associativity;
 *   - syntactic assignment targets.
 *
 * DOES NOT OWN:
 *
 *   - identifier syntax;
 *   - member-access syntax;
 *   - indexing syntax;
 *   - dereference syntax;
 *   - literal syntax;
 *   - arithmetic syntax;
 *   - logical syntax;
 *   - comparison syntax;
 *   - conditional-expression syntax;
 *   - type checking;
 *   - mutability checking;
 *   - ownership checking;
 *   - borrow checking;
 *   - resource checking;
 *   - hardware mapping;
 *   - quantum mapping.
 *
 * ============================================================================
 * IMPORTANT SEMANTIC RULE
 * ============================================================================
 *
 * The grammar deliberately does NOT restrict the left side to a narrow
 * identifier-only form.
 *
 * Examples that may be syntactically represented include:
 *
 *     x = value
 *     object.field = value
 *     array[index] = value
 *     tensor[i, j] = value
 *     *pointer = value
 *     register.value = value
 *     quantum_classical_state = value
 *
 * Whether a particular expression is actually assignable is determined
 * downstream.
 *
 * This separation is essential for:
 *
 *   - overloaded/member assignment;
 *   - indexing abstractions;
 *   - references;
 *   - ownership-aware languages;
 *   - destructuring;
 *   - hardware registers;
 *   - memory-mapped abstractions;
 *   - future language extensions.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar contains no fixed machine limits.
 *
 * It MUST NOT encode:
 *
 *     MAX_VARIABLES
 *     MAX_ASSIGNMENTS
 *     MAX_TUPLE_ARITY
 *     MAX_ARRAY_SIZE
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_REGISTER_COUNT
 *     MAX_NODES
 *     MAX_ACCELERATORS
 *
 * Repetition is represented using ANTLR repetition operators rather than
 * arbitrary numeric ceilings.
 *
 * Any actual implementation limit belongs to:
 *
 *     compiler configuration
 *     parser/runtime resource policy
 *     semantic validation
 *     resource management
 *     scheduling
 *     deployment
 *     hardware capability negotiation
 *
 * and MUST NOT become part of source-language grammar semantics.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file consumes tokens from the canonical Zamani lexer.
 *
 * Required assignment tokens:
 *
 *     ASSIGN
 *     PLUS_ASSIGN
 *     MINUS_ASSIGN
 *     STAR_ASSIGN
 *     SLASH_ASSIGN
 *     MODULO_ASSIGN
 *     BIT_AND_ASSIGN
 *     BIT_OR_ASSIGN
 *     CARET_ASSIGN
 *     LEFT_SHIFT_ASSIGN
 *     RIGHT_SHIFT_ASSIGN
 *
 * The lexer owns their textual spelling.
 *
 * This parser file MUST NOT redefine those tokens.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION CONTRACT
 * ============================================================================
 *
 * Lower-level expression grammar must expose:
 *
 *     conditionalExpression
 *
 * as the expression immediately above assignment precedence.
 *
 * The canonical expression aggregator must expose:
 *
 *     assignmentExpression
 *
 * as the complete expression entry point.
 *
 * Conceptually:
 *
 *     expression
 *         -> assignmentExpression
 *             -> conditionalExpression
 *                 -> logical...
 *                     -> bitwise...
 *                         -> comparison...
 *                             -> arithmetic...
 *
 * Assignment therefore has the lowest precedence among ordinary expressions.
 *
 * ============================================================================
 * RIGHT ASSOCIATIVITY
 * ============================================================================
 *
 * Assignment is right-associative.
 *
 * Therefore:
 *
 *     a = b = c
 *
 * means structurally:
 *
 *     a = (b = c)
 *
 * and:
 *
 *     a += b += c
 *
 * means structurally:
 *
 *     a += (b += c)
 *
 * This grammar preserves that structure.
 *
 * ============================================================================
 * COMPOUND ASSIGNMENT
 * ============================================================================
 *
 * Compound assignment is syntactically represented as one operator.
 *
 * For example:
 *
 *     x += y
 *
 * is NOT parsed here as:
 *
 *     x + y
 *
 * followed by:
 *
 *     x =
 *
 * The semantic layer may subsequently lower compound assignment according
 * to language semantics.
 *
 * That lowering MUST preserve:
 *
 *     - evaluation order;
 *     - side effects;
 *     - single evaluation of the target where required;
 *     - ownership semantics;
 *     - overflow semantics;
 *     - numeric conversion rules;
 *     - quantum/classical boundary semantics;
 *     - hardware semantics where applicable.
 *
 * ============================================================================
 * NO SEMANTIC SHORTCUTS
 * ============================================================================
 *
 * This grammar does not assume that:
 *
 *     x += y
 *
 * is always equivalent to:
 *
 *     x = x + y
 *
 * because such equivalence can fail in the presence of:
 *
 *     overloaded operators
 *     side effects
 *     volatile/hardware state
 *     aliasing
 *     ownership rules
 *     saturating arithmetic
 *     checked arithmetic
 *     quantum/classical semantics
 *     custom numeric domains
 *
 * ============================================================================
 */

parser grammar AssignmentExpressions;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 */

/**
 * Complete assignment expression.
 *
 * Lowest ordinary expression precedence.
 *
 * Right-associative assignment is represented by the recursive right-hand
 * assignmentExpression.
 *
 * Examples:
 *
 *     value
 *     x = value
 *     x += value
 *     object.field = value
 *     array[index] = value
 *     a = b = c
 */
assignmentExpression
    : conditionalExpression
    | assignmentTarget assignmentOperator assignmentExpression
    ;


/*
 * ============================================================================
 * ASSIGNMENT TARGET
 * ============================================================================
 *
 * Assignment target syntax intentionally delegates to the existing postfix
 * expression hierarchy.
 *
 * The semantic analyzer decides whether the resulting expression denotes an
 * assignable place/value.
 *
 * This prevents the grammar from becoming coupled to:
 *
 *     - one particular ownership model;
 *     - one reference model;
 *     - one object model;
 *     - one memory model;
 *     - one hardware model;
 *     - one quantum model.
 *
 * The canonical postfixExpression must therefore be supplied by the
 * expression grammar imported by the parser architecture.
 *
 * The preferred public contract is:
 *
 *     postfixExpression
 *
 * as the syntactic representation of a potentially assignable expression.
 */
assignmentTarget
    : postfixExpression
    ;


/*
 * ============================================================================
 * ASSIGNMENT OPERATORS
 * ============================================================================
 *
 * The operator set is deliberately centralized here.
 *
 * Arithmetic/bitwise expression grammars MUST NOT consume compound assignment
 * operators as ordinary binary operators.
 *
 * This preserves an unambiguous precedence boundary.
 */
assignmentOperator
    : ASSIGN
    | PLUS_ASSIGN
    | MINUS_ASSIGN
    | STAR_ASSIGN
    | SLASH_ASSIGN
    | MODULO_ASSIGN
    | BIT_AND_ASSIGN
    | BIT_OR_ASSIGN
    | CARET_ASSIGN
    | LEFT_SHIFT_ASSIGN
    | RIGHT_SHIFT_ASSIGN
    ;


/*
 * ============================================================================
 * CONDITIONAL EXPRESSION CONTRACT
 * ============================================================================
 *
 * `conditionalExpression` is owned by the conditional-expression layer.
 *
 * Assignment depends on it but does not redefine it.
 *
 * This dependency direction is intentional:
 *
 *     assignment
 *         |
 *         v
 *     conditional
 *         |
 *         v
 *     logical
 *         |
 *         v
 *     bitwise
 *         |
 *         v
 *     comparison
 *         |
 *         v
 *     arithmetic
 *         |
 *         v
 *     postfix / primary
 *
 * There must be no reverse dependency from conditionalExpression back into
 * assignmentExpression unless the language specification explicitly requires
 * assignment expressions inside conditional branches.
 *
 * If conditional expressions need assignment expressions in their branches,
 * that relationship must be provided through a higher-level expression
 * adapter rather than creating a grammar-import cycle.
 */


/*
 * ============================================================================
 * SEMANTIC AST CONTRACT
 * ============================================================================
 *
 * This grammar must lower into an AST representation equivalent to:
 *
 *     AssignmentExpression {
 *         target: Expression,
 *         operator: AssignmentOperator,
 *         value: Expression
 *     }
 *
 * where:
 *
 *     AssignmentOperator =
 *         Assign
 *         | AddAssign
 *         | SubtractAssign
 *         | MultiplyAssign
 *         | DivideAssign
 *         | ModuloAssign
 *         | BitAndAssign
 *         | BitOrAssign
 *         | BitXorAssign
 *         | LeftShiftAssign
 *         | RightShiftAssign
 *
 * The grammar does not define the Rust AST type.
 *
 * Source spans MUST be retained by the frontend AST implementation.
 *
 * The AST should preserve:
 *
 *     - complete target;
 *     - operator;
 *     - complete value;
 *     - source span;
 *     - child source spans;
 *     - syntactic ordering.
 *
 * No machine-specific information belongs in this AST node merely because
 * the assignment occurs in a quantum, hardware, embedded, GPU, or distributed
 * program.
 */


/*
 * ============================================================================
 * SEMANTIC ANALYSIS CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis must determine:
 *
 *     1. whether target is assignable;
 *     2. whether target is mutable;
 *     3. whether target is initialized appropriately;
 *     4. whether the value type is compatible;
 *     5. whether implicit conversion is permitted;
 *     6. whether ownership/borrowing permits the operation;
 *     7. whether effects permit the operation;
 *     8. whether capability requirements are satisfied;
 *     9. whether resource constraints permit it;
 *    10. whether a quantum/classical boundary is legal;
 *    11. whether hardware-specific semantics are required;
 *    12. whether compound-assignment lowering is valid.
 *
 * None of those decisions belong in this grammar.
 */


/*
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum assignments may eventually represent classical state associated
 * with quantum computation.
 *
 * Examples might include:
 *
 *     result = measure(q)
 *     bit = measurement
 *     parameter = theta
 *
 * The grammar does not decide whether an expression represents:
 *
 *     - a classical value;
 *     - a measurement result;
 *     - a symbolic parameter;
 *     - a logical-qubit property;
 *     - a resource value;
 *     - a hardware value.
 *
 * Semantic lowering determines the appropriate canonical representation.
 *
 * In particular:
 *
 *     assignment.g4
 *
 * MUST NOT create or manipulate:
 *
 *     quantum::ir
 *
 * directly.
 */


/*
 * ============================================================================
 * HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * Hardware-facing assignments may syntactically look identical to ordinary
 * assignments:
 *
 *     register_value = value
 *     signal = expression
 *     state = next_state
 *
 * The grammar intentionally does not decide whether a target is:
 *
 *     memory
 *     register
 *     signal
 *     wire
 *     port
 *     device property
 *     software variable
 *
 * That distinction belongs to semantic analysis and the hardware/HDL
 * lowering layers.
 *
 * Therefore assignment syntax remains portable across:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     quantum systems
 *     embedded systems
 *     distributed systems
 *     future targets
 */


/*
 * ============================================================================
 * RESOURCE / POCO-REAF INTEGRATION
 * ============================================================================
 *
 * An assignment does not imply any particular resource.
 *
 * The grammar must never transform:
 *
 *     x = value
 *
 * into a machine-specific requirement such as:
 *
 *     use device X
 *     use core Y
 *     use GPU Z
 *     use physical qubit Q
 *
 * Such information belongs downstream in:
 *
 *     resource requirements
 *     capabilities
 *     target selection
 *     placement
 *     scheduling
 *     deployment
 *
 * This preserves:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Run Anywhere
 *     Run Forever
 */


/*
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify syntactic errors such as:
 *
 *     x =
 *     = x
 *     x +=
 *     x +== y
 *     x = = y
 *
 * The parser should NOT emit semantic diagnostics such as:
 *
 *     variable is immutable
 *     type mismatch
 *     cannot assign to quantum state
 *     hardware resource unavailable
 *     insufficient memory
 *     target lacks capability
 *
 * Those belong to downstream semantic/validation stages.
 */


/*
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For the same lexer token stream and grammar version, this grammar must
 * produce the same parse structure.
 *
 * No:
 *
 *     system time
 *     random number
 *     environment variable
 *     hardware state
 *     runtime device query
 *     network query
 *
 * may influence parsing.
 */


/*
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Assignment operator spelling is owned by the canonical lexer.
 *
 * Adding a future assignment operator requires:
 *
 *     1. lexer token definition;
 *     2. assignmentOperator addition;
 *     3. AST operator representation;
 *     4. semantic lowering;
 *     5. compatibility documentation;
 *     6. positive tests;
 *     7. negative/ambiguity tests;
 *     8. round-trip tests where applicable.
 *
 * Existing operators must not silently change meaning.
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     x = y
 *     x += y
 *     x -= y
 *     x *= y
 *     x /= y
 *     x %= y
 *     x &= y
 *     x |= y
 *     x ^= y
 *     x <<= y
 *     x >>= y
 *
 * Right associativity:
 *
 *     a = b = c
 *     a += b += c
 *
 * Complex targets:
 *
 *     object.field = value
 *     object.field += value
 *     array[index] = value
 *     tensor[i, j] = value
 *     *reference = value
 *
 * Precedence:
 *
 *     x = a + b
 *     x = a && b
 *     x = a | b
 *     x = condition ? a : b
 *
 * The right side must remain a complete assignment expression.
 *
 * Negative syntax:
 *
 *     = x
 *     x =
 *     x +== y
 *     x >>>= y       // unless explicitly supported by the lexer/specification
 *     x = = y
 *
 * Boundary:
 *
 *     extremely long assignment chains;
 *     extremely long compound-assignment chains;
 *     deeply nested syntactically valid targets;
 *     large generated expression trees.
 *
 * No artificial grammar maximum may be introduced to make these tests pass.
 */


/*
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * Before declaring this file complete, verify:
 *
 * [ ] Canonical lexer exports every assignment token used here.
 *
 * [ ] No assignment token is defined twice elsewhere.
 *
 * [ ] `conditionalExpression` is owned by the appropriate lower expression
 *     layer.
 *
 * [ ] `postfixExpression` is owned by the expression/postfix layer.
 *
 * [ ] `expressions.g4` exposes assignmentExpression as the top expression
 *     precedence layer.
 *
 * [ ] Arithmetic grammar does not consume compound assignment operators.
 *
 * [ ] Bitwise grammar does not consume compound assignment operators.
 *
 * [ ] Logical grammar does not consume assignment operators.
 *
 * [ ] Assignment remains right-associative.
 *
 * [ ] Semantic analysis, not grammar, determines assignability.
 *
 * [ ] Type checking, not grammar, determines type compatibility.
 *
 * [ ] Ownership checking, not grammar, determines mutability.
 *
 * [ ] Hardware mapping is downstream.
 *
 * [ ] Quantum lowering is downstream.
 *
 * [ ] quantum::ir remains canonical.
 *
 * [ ] No QEC/ZQN logic is embedded here.
 *
 * [ ] No scheduling/routing logic is embedded here.
 *
 * [ ] No machine-specific limits exist.
 *
 * [ ] No Rust code is embedded.
 *
 * [ ] Generated Rust remains compatible with Rust 1.97/1.97.1.
 *
 * [ ] Generated/runtime Rust remains free of unsafe code.
 *
 * [ ] Positive, negative, boundary, determinism and round-trip tests exist.
 *
 * ============================================================================
 */