/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/assignments.g4
 *
 * STATUS
 * ------
 * CANONICAL STATEMENT-LEVEL BINDING / ASSIGNMENT GRAMMAR
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * RUST IMPLEMENTATION BASELINE
 * ----------------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust edition 2021
 * Safe Rust only.
 * No unsafe Rust is required or permitted by the compiler implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the STATEMENT-LEVEL assignment/binding operation boundary.
 *
 * It deliberately does NOT own:
 *
 *     - lexical token definitions;
 *     - assignment-expression precedence;
 *     - assignment-expression associativity;
 *     - expression syntax;
 *     - type syntax;
 *     - declaration syntax;
 *     - ownership semantics;
 *     - mutability semantics;
 *     - borrowing semantics;
 *     - resource analysis;
 *     - capability analysis;
 *     - quantum semantics;
 *     - HDL semantics;
 *     - hardware realization;
 *     - compiler lowering;
 *     - runtime execution.
 *
 * The canonical expression grammar owns:
 *
 *     assignmentExpression
 *     assignmentTarget
 *     assignmentOperator
 *
 * through:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file converts an actual assignment expression into a statement-level
 * construct by adding the canonical statement terminator.
 *
 * ============================================================================
 * IMPORTANT ARCHITECTURAL DECISION
 * ============================================================================
 *
 * Zamani distinguishes:
 *
 *     binding/declaration
 *
 * from:
 *
 *     assignment.
 *
 * A binding introduces a name and is owned by:
 *
 *     grammar/statements/declarations.g4
 *
 * An assignment changes the value associated with an already-resolved target
 * and is owned here at statement level.
 *
 * Therefore:
 *
 *     let x = value;
 *
 * is a declaration/binding operation.
 *
 *     x = value;
 *
 * is an assignment operation.
 *
 * This file MUST NOT redefine `let`, `var`, or `const`.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *                         ZAMANI SOURCE
 *                              |
 *                              v
 *                       canonical lexer
 *                              |
 *                              v
 *                       canonical parser
 *                              |
 *                              v
 *                  Statements / Assignments
 *                              |
 *                              v
 *                     domain-neutral AST
 *                              |
 *                              v
 *                       name resolution
 *                              |
 *                              v
 *                     semantic validation
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *       ownership            types              effects
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                    capability/resource analysis
 *                              |
 *                              v
 *                    canonical semantic model
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
 *                    +---------+---------+
 *                    |                   |
 *                    v                   v
 *                 routing             scheduling
 *                    |                   |
 *                    +---------+---------+
 *                              |
 *                              v
 *                       resilience / QEC
 *                              |
 *                              v
 *                             ZQN
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                       target realization
 *
 * This grammar remains entirely upstream of those target-realization stages.
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     assignmentStatement
 *
 *     assignmentOperation
 *
 *     statement-level assignment termination
 *
 *     statement-level integration of assignment syntax
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     assignmentExpression
 *     assignmentTarget
 *     assignmentOperator
 *     conditionalExpression
 *     arithmeticExpression
 *     logicalExpression
 *     bitwiseExpression
 *     comparisonExpression
 *     postfixExpression
 *     primaryExpression
 *     identifier syntax
 *     member access
 *     indexing
 *     function calls
 *     type expressions
 *     declarations
 *     bindings
 *     ownership
 *     borrowing
 *     lifetimes
 *     mutability
 *     effects
 *     capabilities
 *     resource requirements
 *     quantum operations
 *     quantum IR
 *     HDL IR
 *     hardware topology
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly one authoritative owner for each concept.
 *
 * Assignment expression:
 *
 *     grammar/expressions/expressions.g4
 *
 * Statement-level assignment:
 *
 *     grammar/statements/assignments.g4
 *
 * Declaration/binding:
 *
 *     grammar/statements/declarations.g4
 *
 * Statement dispatch:
 *
 *     grammar/statements/statements.g4
 *
 * Lexical spelling:
 *
 *     grammar/lexer/*
 *     grammar/antlr/ZamaniLexer.g4
 *
 * AST:
 *
 *     src/ast/
 *
 * Semantic analysis:
 *
 *     semantic-analysis layer
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * No concept defined above may be duplicated here.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It defines NO lexer rules.
 *
 * The canonical parser composition uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Therefore this grammar consumes the production lexer vocabulary rather than
 * the intermediate lexical-composition vocabulary.
 *
 * Assignment operator spelling is owned by the canonical lexer.
 *
 * Currently relevant canonical assignment operators include:
 *
 *     ASSIGN
 *     PLUS_ASSIGN
 *     MINUS_ASSIGN
 *     STAR_ASSIGN
 *     SLASH_ASSIGN
 *     PERCENT_ASSIGN
 *     AMPERSAND_ASSIGN
 *     PIPE_ASSIGN
 *     CARET_ASSIGN
 *
 * If additional assignment operators are introduced, their lexer tokens must
 * first become canonical before being referenced here.
 *
 * This file must never create aliases merely to hide lexical inconsistencies.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * `assignmentExpression`, `assignmentTarget`, and `assignmentOperator` are
 * expression-layer concepts.
 *
 * They are consumed from the canonical expression composition grammar.
 *
 * The dependency is therefore:
 *
 *     expressions/expressions.g4
 *                |
 *                +--> assignmentExpression
 *                +--> assignmentTarget
 *                +--> assignmentOperator
 *                |
 *                v
 *     statements/assignments.g4
 *                |
 *                v
 *          assignmentStatement
 *
 * This file MUST NOT redefine any of those three rules.
 *
 * ============================================================================
 * STATEMENT CONTRACT
 * ============================================================================
 *
 * The canonical statement dispatcher is:
 *
 *     grammar/statements/statements.g4
 *
 * It already imports:
 *
 *     Assignments
 *
 * and dispatches:
 *
 *     statement
 *         : declarationStatement
 *         | assignmentStatement
 *         | assertionStatement
 *         | controlFlowStatement
 *         | unsafeStatement
 *         | blockExpression
 *         | emptyStatement
 *         | expressionStatement
 *         ;
 *
 * This file therefore exposes exactly:
 *
 *     assignmentStatement
 *
 * for statement-level assignment dispatch.
 *
 * ============================================================================
 * ASSIGNMENT / EXPRESSION DISAMBIGUATION
 * ============================================================================
 *
 * The grammar must distinguish:
 *
 *     x = y;
 *
 * from:
 *
 *     f(x);
 *
 * without embedding semantic predicates.
 *
 * The assignment statement starts with:
 *
 *     assignmentTarget
 *
 * followed by:
 *
 *     assignmentOperator
 *
 * followed by:
 *
 *     assignmentExpression
 *
 * and:
 *
 *     SEMICOLON
 *
 * Generic expression statements remain owned by:
 *
 *     statements.g4
 *
 * This means this file never claims ordinary expressions such as:
 *
 *     value;
 *     call();
 *     measure(q);
 *
 * as assignments.
 *
 * ============================================================================
 * RIGHT ASSOCIATIVITY
 * ============================================================================
 *
 * Assignment-expression associativity remains owned by the expression layer.
 *
 * Therefore:
 *
 *     a = b = c;
 *
 * is structurally:
 *
 *     a = (b = c);
 *
 * The statement grammar must not introduce another recursive assignment model.
 *
 * This file only wraps the assignment operation as a statement.
 *
 * ============================================================================
 * ASSIGNMENT TARGET
 * ============================================================================
 *
 * The target is supplied by:
 *
 *     assignmentTarget
 *
 * from the canonical expression grammar.
 *
 * This permits syntactically representable targets such as:
 *
 *     x
 *     object.field
 *     object::field
 *     array[index]
 *     tensor[i, j]
 *     object.field[index].value
 *
 * Whether a target is semantically assignable is NOT determined here.
 *
 * Examples that may be syntactically recognized and later rejected by
 * semantic analysis include:
 *
 *     literal = value;
 *     call() = value;
 *
 * depending on the expression grammar.
 *
 * Semantic analysis owns:
 *
 *     - lvalue/place validation;
 *     - mutability;
 *     - ownership;
 *     - borrowing;
 *     - aliasing;
 *     - lifetime;
 *     - initialization;
 *     - type compatibility;
 *     - conversion;
 *     - capability requirements;
 *     - resource requirements.
 *
 * ============================================================================
 * COMPOUND ASSIGNMENT
 * ============================================================================
 *
 * Compound operators are syntax.
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
 * This grammar does NOT rewrite:
 *
 *     x += y
 *
 * into:
 *
 *     x = x + y
 *
 * Such a transformation belongs to semantic lowering or canonical IR
 * construction.
 *
 * The downstream transformation must preserve:
 *
 *     - target evaluation;
 *     - evaluation order;
 *     - side effects;
 *     - aliasing;
 *     - ownership;
 *     - borrowing;
 *     - overflow semantics;
 *     - conversion semantics;
 *     - effect semantics;
 *     - volatile semantics;
 *     - hardware semantics;
 *     - domain-specific semantics.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Assignment syntax remains domain-neutral.
 *
 * Examples:
 *
 *     result = measure(q);
 *
 *     angle = parameter;
 *
 *     classical_bit = measurement;
 *
 *     state = quantum_value;
 *
 * This grammar does not determine whether the RHS represents:
 *
 *     classical data
 *     quantum-derived data
 *     measurement data
 *     symbolic data
 *     resource metadata
 *     capability metadata
 *     hardware state
 *
 * That determination belongs to semantic analysis.
 *
 * Quantum assignments eventually follow:
 *
 *     AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum semantic representation
 *       |
 *       v
 *     quantum::ir
 *
 * This grammar MUST NOT:
 *
 *     - enumerate gates;
 *     - allocate physical qubits;
 *     - identify physical devices;
 *     - encode QPU topology;
 *     - encode qubit limits;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - implement ZQN;
 *     - select a HAL backend.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Assignment syntax may be reused for:
 *
 *     software variables
 *     HDL signals
 *     registers
 *     ports
 *     state
 *     memory abstractions
 *     accelerator state
 *     co-designed software/hardware values
 *
 * For example:
 *
 *     signal = next_value;
 *
 * has the same statement-level assignment structure as:
 *
 *     variable = next_value;
 *
 * The semantic/HDL layers determine the meaning.
 *
 * This grammar contains no:
 *
 *     bus width
 *     register width
 *     physical address
 *     clock frequency
 *     FPGA resource count
 *     ASIC technology
 *     device identifier
 *     topology
 *     pipeline capacity
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * An assignment does not implicitly select a resource.
 *
 * For example:
 *
 *     x = value;
 *
 * does NOT mean:
 *
 *     CPU core N
 *     GPU N
 *     FPGA N
 *     QPU N
 *     physical qubit N
 *     memory bank N
 *     accelerator N
 *
 * Resource and capability decisions remain downstream.
 *
 * This is required for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Assignment syntax must remain valid regardless of the eventual realization:
 *
 *     tiny embedded target
 *     CPU
 *     multicore system
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     accelerator
 *     cluster
 *     HPC
 *     distributed system
 *     cloud
 *     future architecture
 *
 * The same source-level assignment can therefore participate in different
 * realizations without changing this grammar.
 *
 * No artificial finite language-level capacity is introduced here.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no maximum for:
 *
 *     assignments
 *     variables
 *     declarations
 *     assignment-chain length
 *     target-chain depth
 *     index count
 *     expression size
 *     program size
 *     qubits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     accelerators
 *     QPUs
 *     nodes
 *     memory
 *     storage
 *     registers
 *     tensor dimensions
 *     tensor rank
 *
 * Repetition and recursion are used instead of fixed enumerations.
 *
 * "Infinity" here means that this grammar introduces no artificial finite
 * language limit. Actual limits are governed by available compiler, runtime,
 * deployment and target resources.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source token sequence;
 *     - grammar version;
 *     - imported grammar versions;
 *     - explicit parser configuration.
 *
 * Parsing must not depend on:
 *
 *     - wall-clock time;
 *     - randomness;
 *     - environment variables;
 *     - filesystem state;
 *     - network state;
 *     - hardware discovery;
 *     - CPU availability;
 *     - GPU availability;
 *     - QPU availability;
 *     - scheduler state;
 *     - calibration state;
 *     - runtime state.
 *
 * ============================================================================
 * DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Syntax diagnostics belong here.
 *
 * Examples:
 *
 *     = value;
 *     x =;
 *     x +=;
 *     x +== value;
 *     x = = value;
 *     x += = value;
 *
 * Semantic diagnostics do NOT belong here.
 *
 * Examples:
 *
 *     assignment to immutable binding
 *     assignment to non-place expression
 *     type mismatch
 *     invalid conversion
 *     ownership violation
 *     borrow violation
 *     lifetime violation
 *     unavailable capability
 *     unavailable resource
 *     invalid quantum/classical boundary
 *     invalid HDL target
 *
 * ============================================================================
 * SOURCE SPAN CONTRACT
 * ============================================================================
 *
 * ANTLR parse contexts naturally retain token boundaries.
 *
 * The AST construction layer MUST preserve:
 *
 *     - complete assignment statement span;
 *     - target span;
 *     - operator span;
 *     - RHS span;
 *     - nested assignment spans;
 *     - source ordering.
 *
 * This grammar must not construct spans itself.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Conceptual AST representation:
 *
 *     AssignmentStatement {
 *         assignment: AssignmentExpression,
 *         span: SourceSpan
 *     }
 *
 * The repository's existing AST is the authoritative implementation.
 *
 * This grammar must not introduce:
 *
 *     QuantumAssignment
 *     GPUAssignment
 *     FPGAAssignment
 *     CPUAssignment
 *     QPUAssignment
 *     HardwareAssignment
 *
 * merely because the assignment later lowers to one of those domains.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parser responsibility:
 *
 *     "Does the source have assignment-statement syntax?"
 *
 * Semantic responsibility:
 *
 *     "Is the assignment legal, and what does it mean?"
 *
 * Semantic analysis must establish:
 *
 *     target resolution
 *     mutability
 *     assignability
 *     type compatibility
 *     conversion rules
 *     ownership
 *     borrowing
 *     lifetime
 *     effect legality
 *     capability requirements
 *     resource requirements
 *     domain legality
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Downstream lowering may produce:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware IR
 *     data/control IR
 *     distributed representations
 *     accelerator representations
 *
 * according to semantic meaning.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * No assignment grammar is allowed to create another quantum IR.
 *
 * ============================================================================
 * COMPILER / RUNTIME CONTRACT
 * ============================================================================
 *
 * This file has no direct dependency on:
 *
 *     compiler backends
 *     runtime implementations
 *     schedulers
 *     routers
 *     HAL implementations
 *     hardware drivers
 *     QPU drivers
 *     FPGA toolchains
 *     GPU toolchains
 *
 * Adding a new backend therefore must not require changing assignment syntax.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing stable forms remain:
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
 * The exact set of operators is determined by the canonical lexer and
 * expression grammar.
 *
 * Adding a new operator requires:
 *
 *     1. lexical specification;
 *     2. canonical lexer token;
 *     3. expression-layer operator integration;
 *     4. assignment-statement integration;
 *     5. AST representation;
 *     6. semantic rules;
 *     7. lowering/IR rules;
 *     8. positive tests;
 *     9. negative tests;
 *    10. boundary tests;
 *    11. compatibility documentation.
 *
 * No operator may silently change existing meaning.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
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
 * Complex targets:
 *
 *     object.field = value;
 *     object::field = value;
 *     array[index] = value;
 *     tensor[i, j] = value;
 *     object.field[index].value = expression;
 *
 * Assignment chains:
 *
 *     a = b = c;
 *     a += b += c;
 *     a -= b = c;
 *
 * RHS expressions:
 *
 *     x = a + b;
 *     x = a * b;
 *     x = a && b;
 *     x = a | b;
 *     x = condition ? a : b;
 *
 * Quantum/classical:
 *
 *     result = measure(q);
 *     bit = measurement;
 *
 * HDL/co-design:
 *
 *     signal = next_value;
 *     state = next_state;
 *
 * NEGATIVE SYNTAX
 * ---------------
 *
 *     = x;
 *     x =;
 *     x +=;
 *     x +== y;
 *     x = = y;
 *     x += = y;
 *
 * These must fail syntactically where the canonical expression grammar makes
 * the construct invalid.
 *
 * SEMANTIC NEGATIVES
 * ------------------
 *
 * These remain semantic rather than grammar errors where the expression
 * structure is syntactically valid:
 *
 *     immutable = value;
 *     literal = value;
 *     call() = value;
 *
 * depending on the semantic model.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *     - empty RHS;
 *     - empty target;
 *     - nested assignment;
 *     - long assignment chains;
 *     - deeply nested postfix targets;
 *     - long member chains;
 *     - long index chains;
 *     - symbolic expressions;
 *     - tensor expressions;
 *     - quantum-derived values;
 *     - HDL values;
 *     - distributed values.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Generated tests should exercise increasing:
 *
 *     assignment-chain length
 *     target-chain depth
 *     index arity
 *     RHS expression size
 *     program size
 *
 * without changing the grammar.
 *
 * No test may encode a universal machine-size ceiling.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Assignment syntax must remain reusable across:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware/software co-design
 *     embedded
 *     distributed
 *     HPC
 *     AI/ML
 *     data
 *     networking
 *     cryptography
 *     accelerator
 *     future domains
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar must contain none of the following as language limits:
 *
 *     MAX_ASSIGNMENTS
 *     MAX_VARIABLES
 *     MAX_TARGETS
 *     MAX_TENSOR_RANK
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ACCELERATORS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_DEVICES
 *
 * Numeric literals appearing in source programs remain program semantics.
 *
 * ============================================================================
 * SAFETY AUDIT
 * ============================================================================
 *
 * This grammar:
 *
 *     - contains no Rust;
 *     - contains no embedded actions;
 *     - contains no semantic predicates;
 *     - contains no unsafe code;
 *     - performs no I/O;
 *     - performs no hardware access;
 *     - performs no runtime execution.
 *
 * The Rust implementation consuming this grammar must remain:
 *
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *     safe Rust only.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] `Assignments` remains the grammar name.
 *     [ ] `assignmentStatement` is the only statement-level assignment owner.
 *     [ ] `assignmentExpression` is owned by the expression layer.
 *     [ ] `assignmentTarget` is owned by the expression layer.
 *     [ ] `assignmentOperator` is owned by the expression layer.
 *     [ ] canonical `ZamaniLexer` vocabulary is consumed.
 *     [ ] no lexer rules are duplicated.
 *     [ ] no declaration/binding syntax is duplicated.
 *     [ ] no generic expression grammar is duplicated.
 *     [ ] assignment chains remain right associative.
 *     [ ] statement termination is explicit.
 *     [ ] semantic assignability remains downstream.
 *     [ ] ownership remains downstream.
 *     [ ] type checking remains downstream.
 *     [ ] resource analysis remains downstream.
 *     [ ] capability analysis remains downstream.
 *     [ ] quantum::ir remains the canonical quantum boundary.
 *     [ ] QEC remains downstream.
 *     [ ] ZQN remains downstream.
 *     [ ] routing remains downstream.
 *     [ ] scheduling remains downstream.
 *     [ ] hardware realization remains downstream.
 *     [ ] no machine-size limit is encoded.
 *     [ ] no Rust unsafe is required.
 *     [ ] Rust 1.97 / 1.97.1 integration remains valid.
 *     [ ] positive tests exist.
 *     [ ] negative tests exist.
 *     [ ] boundary tests exist.
 *     [ ] scalability tests exist.
 *     [ ] cross-domain tests exist.
 *     [ ] deterministic parsing is preserved.
 *
 * ============================================================================
 * CANONICAL GRAMMAR
 * ============================================================================
 */

parser grammar Assignments;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Expressions is the canonical expression composition grammar.
 *
 * It already owns:
 *
 *     assignmentExpression
 *     assignmentTarget
 *     assignmentOperator
 *
 * and the complete lower expression hierarchy.
 *
 * This import therefore gives this grammar access to the existing canonical
 * expression rules without defining a second assignment expression language.
 */
import Expressions;


/*
 * ============================================================================
 * ASSIGNMENT STATEMENT
 * ============================================================================
 *
 * A statement-level assignment is:
 *
 *     assignment target
 *     assignment operator
 *     assignment expression
 *     statement terminator
 *
 * Examples:
 *
 *     x = y;
 *     x += y;
 *     object.field = value;
 *     array[index] = value;
 *     a = b = c;
 *
 * The RHS is the complete canonical assignmentExpression, allowing
 * right-associative assignment chains.
 *
 * No assignment operator is redefined here.
 */
assignmentStatement
    : assignmentOperation SEMICOLON
    ;


/*
 * ============================================================================
 * ASSIGNMENT OPERATION
 * ============================================================================
 *
 * This is intentionally separate from `assignmentStatement` so downstream
 * tooling can identify the assignment operation independently of its statement
 * terminator.
 *
 * It uses the canonical rules imported from Expressions.
 */
assignmentOperation
    : assignmentTarget assignmentOperator assignmentExpression
    ;