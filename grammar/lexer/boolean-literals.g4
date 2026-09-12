/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/boolean-literals.g4
 *
 * Role:
 *     Canonical lexical owner for boolean literals.
 *
 * Lexer technology:
 *     ANTLR
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This file contains grammar source only.
 *     It contains no Rust code and therefore introduces no unsafe Rust.
 *     The Zamani compiler/runtime MUST remain safe Rust only.
 *
 * ============================================================================
 *
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * This file owns the lexical representation of Zamani boolean literals.
 *
 * The canonical boolean values are:
 *
 *     true
 *     false
 *
 * This file owns HOW those source spellings become lexical tokens.
 *
 * It does NOT own:
 *
 *     - boolean types;
 *     - boolean operators;
 *     - truth tables;
 *     - constant folding;
 *     - type checking;
 *     - coercion;
 *     - control-flow semantics;
 *     - three-valued logic;
 *     - symbolic logic;
 *     - quantum measurement semantics;
 *     - classical/quantum conversion;
 *     - AST design;
 *     - IR;
 *     - target lowering;
 *     - hardware;
 *     - runtime representation;
 *     - machine word size;
 *     - memory representation;
 *     - optimization;
 *     - scheduling;
 *     - QEC;
 *     - ZQN.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Boolean literals describe language-level values.
 *
 * They MUST NOT encode:
 *
 *     - CPU register width;
 *     - machine word width;
 *     - SIMD width;
 *     - GPU lane count;
 *     - FPGA resource count;
 *     - quantum-device capacity;
 *     - hardware topology;
 *     - memory capacity;
 *     - target ABI;
 *     - runtime representation.
 *
 * Therefore:
 *
 *     true
 *
 * means the Zamani boolean value `true`.
 *
 * It does NOT mean:
 *
 *     1-bit hardware register
 *     8-bit integer
 *     32-bit integer
 *     64-bit integer
 *     machine-native boolean
 *
 * unless a later semantic/type/lowering stage explicitly establishes such
 * a representation for a particular target.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *
 *     - lexical spelling of `true`;
 *     - lexical spelling of `false`;
 *     - canonical boolean-literal token names;
 *     - lexical separation between boolean literals and identifiers;
 *     - lexical documentation of the boolean literal contract.
 *
 * DOES NOT OWN:
 *
 *     - `bool` type syntax;
 *     - `and`;
 *     - `or`;
 *     - `not`;
 *     - `&&`;
 *     - `||`;
 *     - `!`;
 *     - equality operators;
 *     - conditional expressions;
 *     - pattern matching;
 *     - boolean algebra;
 *     - type inference;
 *     - semantic validation.
 *
 * ============================================================================
 *
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Intended composition:
 *
 *     boolean-literals.g4
 *              |
 *              v
 *     literals.g4
 *              |
 *              v
 *     canonical lexer
 *              |
 *              v
 *     parser
 *              |
 *              v
 *     AST
 *              |
 *              v
 *     semantic/type analysis
 *              |
 *              v
 *     canonical IR
 *              |
 *              +--> classical IR
 *              +--> quantum IR
 *              +--> control/data IR
 *              |
 *              v
 *     optimization / scheduling / lowering / runtime
 *
 * `literals.g4` is the aggregation boundary for specialized literal
 * grammars. It already declares `ZamaniBooleanLiterals` as the owner of the
 * boolean literal family.
 *
 * The canonical parser MUST NOT import this grammar directly.
 *
 * The parser consumes the token vocabulary exposed by the canonical lexer.
 *
 * ============================================================================
 *
 * TOKEN OWNERSHIP
 * ============================================================================
 *
 * This grammar deliberately defines:
 *
 *     TRUE
 *     FALSE
 *
 * It MUST therefore be the sole owner of those lexical token rules in the
 * specialized literal architecture.
 *
 * The canonical lexer MUST NOT independently redefine:
 *
 *     TRUE
 *     FALSE
 *
 * after this grammar is integrated.
 *
 * There must be exactly one lexical owner for each spelling.
 *
 * ============================================================================
 *
 * IDENTIFIER BOUNDARY
 * ============================================================================
 *
 * `true` and `false` are reserved literal spellings.
 *
 * Because these rules occur before the general identifier rule in the
 * canonical lexical composition, the following must tokenize as boolean
 * literals:
 *
 *     true
 *     false
 *
 * while identifiers such as:
 *
 *     truthful
 *     falsehood
 *     true_value
 *     false_value
 *
 * remain identifiers, subject to the canonical identifier grammar.
 *
 * This file does not redefine identifier syntax.
 *
 * ============================================================================
 *
 * CASE SENSITIVITY
 * ============================================================================
 *
 * Boolean literal spellings are intentionally lowercase:
 *
 *     true
 *     false
 *
 * The grammar MUST NOT silently accept:
 *
 *     True
 *     TRUE
 *     False
 *     FALSE
 *
 * as boolean literals unless a future Zamani language version explicitly
 * changes the language specification.
 *
 * Case policy is therefore deterministic and versionable.
 *
 * ============================================================================
 *
 * NO BOOLEAN ALIASES
 * ============================================================================
 *
 * This grammar intentionally does not make these lexical aliases:
 *
 *     yes
 *     no
 *     on
 *     off
 *     enabled
 *     disabled
 *     1
 *     0
 *
 * equivalent to booleans.
 *
 * Numeric literals are owned by numeric-literals.g4.
 *
 * Domain-specific configuration words remain identifiers or keywords according
 * to their respective grammar ownership.
 *
 * Semantic conversion, when explicitly requested by the language, belongs to
 * semantic analysis rather than lexical aliasing.
 *
 * ============================================================================
 *
 * NULL / NIL SEPARATION
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     null
 *     nil
 *
 * Those are distinct language concepts and MUST remain owned by the
 * appropriate null/algebraic-value grammar.
 *
 * In particular:
 *
 *     true
 *     false
 *     null
 *     nil
 *
 * MUST NOT be collapsed into a single lexical family.
 *
 * ============================================================================
 *
 * ALGEBRAIC TYPES
 * ============================================================================
 *
 * Boolean literals are also distinct from constructors such as:
 *
 *     Some
 *     Ok
 *     Err
 *
 * if those constructs exist in the language.
 *
 * This grammar does not define algebraic-data-type constructors.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum measurement may eventually produce a classical boolean value.
 *
 * That does NOT make the boolean lexer responsible for quantum semantics.
 *
 * For example, semantic layers may represent:
 *
 *     measure(q) == true
 *
 * or another explicitly defined boolean-producing operation.
 *
 * The boolean literal remains an ordinary source-level boolean value.
 *
 * This file MUST NOT know about:
 *
 *     QubitId
 *     PhysicalQubitId
 *     quantum::ir
 *     QEC
 *     ZQN
 *     hardware topology
 *     measurement hardware
 *     readout fidelity
 *     physical measurement mechanisms.
 *
 * Those concerns belong to the corresponding repository subsystems.
 *
 * ============================================================================
 *
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * No hardware-specific information is permitted in this grammar.
 *
 * Forbidden examples include:
 *
 *     MAX_BOOLEAN_WIDTH
 *     MAX_BOOL_VALUES
 *     MAX_REGISTERS
 *     MAX_DEVICES
 *     DEVICE_BOOLEAN_WIDTH
 *     CPU_BOOLEAN_WIDTH
 *
 * Boolean lexical syntax has no finite machine-dependent scalability limit.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * A boolean literal has constant lexical structure regardless of the size of
 * the program in which it occurs.
 *
 * The grammar imposes no artificial limits on:
 *
 *     - number of boolean literals;
 *     - number of boolean expressions;
 *     - number of declarations;
 *     - number of functions;
 *     - number of modules;
 *     - number of devices;
 *     - number of qubits;
 *     - number of nodes;
 *     - program size.
 *
 * Resource limits, if any, are owned by compilation/runtime infrastructure.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * For a given character stream and lexer configuration:
 *
 *     true  -> TRUE
 *     false -> FALSE
 *
 * deterministically.
 *
 * The grammar does not depend on:
 *
 *     - machine size;
 *     - hardware availability;
 *     - runtime state;
 *     - current backend;
 *     - scheduling;
 *     - resource discovery;
 *     - random selection.
 *
 * ============================================================================
 *
 * ERROR HANDLING
 * ============================================================================
 *
 * This file deliberately does not embed target-language actions or custom
 * runtime code.
 *
 * Invalid spellings are handled by the canonical lexer and its configured
 * diagnostic/error strategy.
 *
 * Examples that must NOT silently become boolean literals:
 *
 *     True
 *     FALSE
 *     falsex
 *     truex
 *     tru
 *     fals
 *
 * The lexer must instead classify them according to the canonical lexical
 * rules, normally as identifiers or lexical errors depending on the source.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces lexical tokens only.
 *
 * It does NOT construct an AST.
 *
 * A parser/AST layer may map:
 *
 *     TRUE
 *         ->
 *     BooleanLiteral(true)
 *
 * and:
 *
 *     FALSE
 *         ->
 *     BooleanLiteral(false)
 *
 * The exact AST node name belongs to the frontend AST contract and MUST NOT
 * be duplicated here.
 *
 * ============================================================================
 *
 * TYPE SYSTEM CONTRACT
 * ============================================================================
 *
 * This grammar does not declare the Zamani boolean type.
 *
 * The type layer owns the semantic type.
 *
 * A later type-analysis stage may establish that:
 *
 *     true  : bool
 *     false : bool
 *
 * or the corresponding canonical Zamani boolean type representation.
 *
 * This file must remain unchanged if the internal representation of `bool`
 * changes from one target to another.
 *
 * ============================================================================
 *
 * IR CONTRACT
 * ============================================================================
 *
 * No IR is produced by this grammar.
 *
 * The frontend semantic layer may lower the parsed boolean literal into the
 * repository's canonical representation.
 *
 * This grammar must never create:
 *
 *     QuantumGate
 *     QubitId
 *     PhysicalQubitId
 *     quantum::ir
 *     hardware IR
 *     scheduling IR
 *     runtime instructions.
 *
 * ============================================================================
 *
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Rust 1.97 / Rust 1.97.1 compatibility is required for the compiler-side
 * implementation that consumes generated lexer/parser artifacts.
 *
 * This grammar itself contains no Rust-specific implementation.
 *
 * The Rust compiler crate MUST enforce safe Rust only.
 *
 * No unsafe code is required by this lexical rule.
 *
 * ============================================================================
 *
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * `true` and `false` are canonical language spellings.
 *
 * Changing these spellings would be a source-language compatibility change.
 *
 * Therefore any future change must update, as one coordinated language
 * evolution:
 *
 *     grammar/lexer/boolean-literals.g4
 *     grammar/spec/lexical.md
 *     grammar/spec/syntax.md
 *     grammar/lexer/literals.g4
 *     canonical lexer integration
 *     parser literal rules
 *     lexer tests
 *     parser tests
 *     compatibility tests
 *     language documentation.
 *
 * This file must not independently introduce alternate boolean spellings.
 *
 * ============================================================================
 *
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive lexical tests:
 *
 *     true
 *     false
 *
 * Required identifier-boundary tests:
 *
 *     truthful
 *     falsehood
 *     true_value
 *     false_value
 *
 * Required case-sensitivity tests:
 *
 *     True
 *     TRUE
 *     False
 *     FALSE
 *
 * Required prefix/suffix tests:
 *
 *     truex
 *     xtrue
 *     falsex
 *     xfalse
 *
 * Required adjacency tests:
 *
 *     truefalse
 *     false_true
 *     true.false
 *     false_value
 *
 * Required parser-integration tests:
 *
 *     let a = true;
 *     let b = false;
 *     if true { ... }
 *     if false { ... }
 *
 * Required expression tests belong to the parser/expression test layer and
 * MUST verify that the boolean token participates correctly in expressions.
 *
 * Required negative tests must verify that malformed boolean spellings do not
 * receive TRUE/FALSE tokens.
 *
 * Required scalability tests must verify that the number of occurrences of
 * boolean literals is not artificially bounded by this grammar.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when all of the following are true:
 *
 *     [x] Owns exactly the canonical boolean literal spellings.
 *     [x] Does not define bool type semantics.
 *     [x] Does not define boolean operators.
 *     [x] Does not define identifier syntax.
 *     [x] Does not define null/nil semantics.
 *     [x] Does not define quantum semantics.
 *     [x] Does not define hardware semantics.
 *     [x] Does not impose machine limits.
 *     [x] Does not contain Rust unsafe code.
 *     [x] Has a stable token contract.
 *     [x] Has deterministic lexical behavior.
 *     [x] Is composable through literals.g4.
 *     [x] Has no duplicate boolean-token owner.
 *     [x] Is consumable by the canonical lexer architecture.
 *     [x] Is compatible with Rust 1.97 / 1.97.1 compiler integration.
 *
 * ============================================================================
 *
 * CANONICAL RULES
 * ============================================================================
 *
 * The actual lexical rules are intentionally minimal.
 *
 * Do not add semantic predicates, target-language actions, machine-dependent
 * conditions, runtime calls, or resource limits to these rules.
 * ============================================================================
 */

lexer grammar ZamaniBooleanLiterals;


/*
 * ============================================================================
 * BOOLEAN LITERALS
 * ============================================================================
 *
 * These are the only canonical boolean literal spellings.
 *
 * Rule ordering inside this specialized lexer is deterministic.
 *
 * When composed into the canonical lexer, the resulting token ownership must
 * remain singular: TRUE and FALSE come from this grammar, not from another
 * lexer grammar.
 * ============================================================================
 */

TRUE
    : 'true'
    ;

FALSE
    : 'false'
    ;