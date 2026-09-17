/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/macros.g4
 *
 * Grammar:
 *     macros
 *
 * Status:
 *     Canonical expression-side macro integration contract.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe Rust required or permitted by the compiler implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the EXPRESSION-SIDE integration point for Zamani macros.
 *
 * It intentionally does NOT own macro declaration or macro invocation syntax.
 *
 * Those responsibilities belong to:
 *
 *     grammar/macros/declarations.g4
 *     grammar/macros/invocations.g4
 *
 * In particular:
 *
 *     macroPath
 *     macroInvocation
 *     macroExpression
 *
 * remain owned by:
 *
 *     grammar/macros/invocations.g4
 *
 * This file exists so that the canonical expression grammar has an explicit,
 * independently documented integration contract for macro expressions.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - expression-level macro integration;
 *     - the expression component representing a macro invocation;
 *     - the contract by which macro invocation becomes an ordinary expression
 *       alternative;
 *     - expression-level documentation and integration boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - qualified names;
 *     - macro declarations;
 *     - macro parameters;
 *     - macro defaults;
 *     - macro bodies;
 *     - macro paths;
 *     - macro invocation syntax;
 *     - macro expansion;
 *     - macro resolution;
 *     - hygiene;
 *     - provenance;
 *     - ordinary function calls;
 *     - ordinary expressions;
 *     - argument-list syntax;
 *     - types;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - quantum semantics;
 *     - classical semantics;
 *     - HDL semantics;
 *     - hardware selection;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - backend selection;
 *     - runtime execution;
 *     - canonical IR.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The intended pipeline is:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical Zamani parser
 *       |
 *       +-------------------------------+
 *       |                               |
 *       v                               v
 *     ordinary expressions        macroExpression
 *                                       |
 *                                       v
 *                                macro invocation
 *                                       |
 *                                       v
 *                                frontend AST
 *                                       |
 *                                       v
 *                                macro resolution
 *                                       |
 *                                       v
 *                                controlled expansion
 *                                       |
 *                                       v
 *                                hygiene/provenance
 *                                       |
 *                                       v
 *                                semantic analysis
 *                                       |
 *                                       v
 *                           canonical semantic representation
 *                                       |
 *                    +------------------+------------------+
 *                    |                  |                  |
 *                    v                  v                  v
 *               classical         quantum::ir        HDL/hardware
 *                    |                  |                  |
 *                    +------------------+------------------+
 *                                       |
 *                                       v
 *                             optimization / lowering
 *                                       |
 *                              routing / scheduling
 *                                       |
 *                              resilience / QEC / ZQN
 *                                       |
 *                                      HAL
 *                                       |
 *                               target realization
 *
 * This file is therefore a SOURCE-SYNTAX integration boundary only.
 *
 * ============================================================================
 * SINGLE AUTHORITY
 * ============================================================================
 *
 * Macro ownership is deliberately split by responsibility.
 *
 *     grammar/macros/declarations.g4
 *         owns macro declarations.
 *
 *     grammar/macros/invocations.g4
 *         owns:
 *
 *             macroPath
 *             macroInvocation
 *             macroExpression
 *
 *     grammar/macros/expansion.g4
 *         owns expansion-related syntax, where such syntax exists.
 *
 *     grammar/macros/hygiene.g4
 *         owns hygiene/provenance-related syntax, where such syntax exists.
 *
 *     grammar/macros/macros.g4
 *         owns macro grammar composition.
 *
 *     grammar/expressions/macros.g4
 *         owns expression-side integration only.
 *
 * No file may create another competing definition of:
 *
 *     macroPath
 *     macroInvocation
 *     macroExpression
 *
 * ============================================================================
 * WHY THIS FILE DOES NOT DEFINE macroInvocation
 * ============================================================================
 *
 * The repository already has a canonical invocation grammar.
 *
 * The canonical invocation shape is:
 *
 *     macroPath BANG LPAREN argumentList? RPAREN
 *
 * Examples:
 *
 *     build!()
 *     build!(value)
 *     build!(a, b)
 *     math::build!(value)
 *     package::math::build!(a, b)
 *
 * Repeating those productions here would create two owners for the same
 * syntax and could produce incompatible generated parser contexts.
 *
 * Therefore this file references the canonical:
 *
 *     macroExpression
 *
 * rather than reconstructing the invocation.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION CONTRACT
 * ============================================================================
 *
 * The canonical expression hierarchy must contain:
 *
 *     macroExpression
 *
 * as one of its primary-expression alternatives.
 *
 * Conceptually:
 *
 *     primaryExpression
 *         :
 *           ...
 *         | macroExpression
 *         | ...
 *         ;
 *
 * The actual primary-expression owner remains the canonical expression
 * composition grammar.
 *
 * This file MUST NOT redefine primaryExpression.
 *
 * It also MUST NOT redefine:
 *
 *     expression
 *     assignmentExpression
 *     conditionalExpression
 *     postfixExpression
 *     callExpression
 *     argumentList
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar component.
 *
 * The lexical vocabulary remains owned by the canonical lexer.
 *
 * Macro invocation tokens such as:
 *
 *     BANG
 *     LPAREN
 *     RPAREN
 *
 * are therefore NOT defined here.
 *
 * Likewise, this file does not define:
 *
 *     IDENTIFIER
 *     DOUBLE_COLON
 *     COMMA
 *     COLON
 *     or any other lexer token.
 *
 * Macro invocation syntax is consumed through the canonical parser rule:
 *
 *     macroExpression
 *
 * supplied by:
 *
 *     grammar/macros/invocations.g4
 *
 * ============================================================================
 * NO CIRCULAR DEPENDENCY CONTRACT
 * ============================================================================
 *
 * The dependency direction must remain:
 *
 *     lexer
 *       |
 *       v
 *     shared parser foundations
 *       |
 *       +--------------------------+
 *       |                          |
 *       v                          v
 *     macro invocation        expression hierarchy
 *       |                          |
 *       +-----------> macroExpression
 *                                  |
 *                                  v
 *                             primaryExpression
 *
 * This file MUST NOT import the complete expression grammar merely to obtain
 * the `expression` rule.
 *
 * Doing so can create:
 *
 *     expressions/macros.g4
 *          ->
 *     expressions/expression.g4
 *          ->
 *     expressions/macros.g4
 *
 * or an equivalent indirect cycle.
 *
 * The canonical composition root resolves the two components.
 *
 * ============================================================================
 * PUBLIC CONTRACT
 * ============================================================================
 *
 * The public expression-level macro construct is:
 *
 *     macroExpression
 *
 * It represents a syntactically valid macro invocation as an expression.
 *
 * The invocation itself remains owned by:
 *
 *     grammar/macros/invocations.g4
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file does not define AST structures.
 *
 * The frontend AST remains the sole source-level AST authority.
 *
 * A parsed macro expression must provide enough information for the existing
 * frontend macro-expression representation to preserve:
 *
 *     - source span;
 *     - invocation identity;
 *     - macro path;
 *     - ordered arguments;
 *     - source identity;
 *     - provenance information required downstream.
 *
 * The grammar must not introduce a competing:
 *
 *     MacroAst
 *     MacroInvocationAst
 *     MacroExpansionAst
 *
 * hierarchy.
 *
 * The repository already contains a dedicated frontend macro expression
 * representation. This grammar must lower into that existing architecture.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only syntactic structure.
 *
 * Semantic analysis determines:
 *
 *     - whether the macro exists;
 *     - whether the macro is visible;
 *     - whether imports are valid;
 *     - whether arguments match;
 *     - whether generic constraints are satisfied;
 *     - whether expansion is permitted;
 *     - whether required compile-time capabilities are available;
 *     - whether expansion satisfies resource policy;
 *     - whether the expansion is deterministic;
 *     - whether the expansion is hygienic;
 *     - whether generated syntax is semantically valid.
 *
 * None of these checks belong in this grammar.
 *
 * ============================================================================
 * MACRO EXPANSION BOUNDARY
 * ============================================================================
 *
 * Parsing MUST stop at the macro invocation syntax.
 *
 * ANTLR parser actions MUST NOT:
 *
 *     - execute macros;
 *     - expand macros;
 *     - resolve macros;
 *     - read files;
 *     - write files;
 *     - access networks;
 *     - execute subprocesses;
 *     - inspect hardware;
 *     - access secrets;
 *     - invoke external programs;
 *     - select a compiler backend;
 *     - select a QPU;
 *     - select a GPU;
 *     - perform code generation.
 *
 * Expansion is a separate controlled compiler phase.
 *
 * ============================================================================
 * HYGIENE AND PROVENANCE
 * ============================================================================
 *
 * Macro expansion must preserve source provenance.
 *
 * The parser must preserve the source structure necessary for the frontend
 * and macro subsystem to associate generated syntax with:
 *
 *     - the macro declaration;
 *     - the invocation;
 *     - the invocation source span;
 *     - the generated syntax;
 *     - nested expansion ancestry.
 *
 * The precise provenance representation belongs to the AST/compiler layers.
 *
 * This file does not implement hygiene.
 *
 * ============================================================================
 * DOMAIN-NEUTRALITY
 * ============================================================================
 *
 * A macro expression is domain-neutral.
 *
 * Its arguments may eventually represent:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL;
 *     hardware intent;
 *     distributed computation;
 *     AI/ML;
 *     data processing;
 *     networking;
 *     cryptography;
 *     scientific computation;
 *     embedded computation;
 *     future Zamani domains.
 *
 * This grammar does not need:
 *
 *     quantumMacroExpression
 *     classicalMacroExpression
 *     hdlMacroExpression
 *     gpuMacroExpression
 *     qpuMacroExpression
 *     fpgaMacroExpression
 *
 * Ordinary macro syntax is intentionally domain-neutral.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Macro expressions may expand into quantum source syntax.
 *
 * For example, a macro may eventually generate syntax representing:
 *
 *     quantum operations
 *     measurements
 *     state preparation
 *     controls
 *     observables
 *     error-correction intent
 *     resource requirements
 *
 * However, this grammar does not:
 *
 *     - define quantum operations;
 *     - define quantum types;
 *     - allocate qubits;
 *     - select physical qubits;
 *     - select a QPU;
 *     - select a gate set;
 *     - perform decomposition;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - implement ZQN;
 *     - select calibration data.
 *
 * After expansion and semantic validation, valid quantum constructs continue
 * through the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * No second macro-specific quantum IR is introduced.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * The same macro-expression mechanism may generate syntax consumed by:
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
 *     future domains
 *
 * Domain-specific semantics remain downstream.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Macro expressions must preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A macro expression must not inherently bind the program to:
 *
 *     a CPU;
 *     a processor count;
 *     a core count;
 *     a thread count;
 *     a GPU;
 *     an FPGA;
 *     an ASIC;
 *     a QPU;
 *     a simulator;
 *     a device identifier;
 *     a physical qubit;
 *     a memory bank;
 *     a network node;
 *     a topology;
 *     a scheduler;
 *     a routing implementation;
 *     a compiler backend.
 *
 * Portable resource requirements and capabilities are represented through the
 * appropriate resource/capability/target semantics.
 *
 * Macro expansion must not silently convert a portable requirement into a
 * physical placement decision.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This file imposes no language-level finite limit on:
 *
 *     macro invocation count;
 *     argument count;
 *     qualified-name depth;
 *     expression nesting;
 *     macro nesting;
 *     source size;
 *     expansion result size;
 *     generated operation count;
 *     generated quantum operation count;
 *     generated HDL structure;
 *     generated distributed structure.
 *
 * There are deliberately no constants such as:
 *
 *     MAX_MACROS
 *     MAX_ARGUMENTS
 *     MAX_MACRO_DEPTH
 *     MAX_EXPANSION_DEPTH
 *     MAX_QUANTUM_MACROS
 *     MAX_NODES
 *     MAX_GPUS
 *     MAX_QUBITS
 *     MAX_THREADS
 *
 * or equivalent universal grammar limits.
 *
 * "Infinity" means that the language does not introduce an artificial finite
 * semantic limit here.
 *
 * Actual implementation limits may exist as configurable resource policies
 * for:
 *
 *     source bytes;
 *     tokens;
 *     parser memory;
 *     AST nodes;
 *     macro expansion steps;
 *     expansion depth;
 *     generated nodes;
 *     compiler memory;
 *     compilation time.
 *
 * Such limits are implementation/resource policies rather than language
 * semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * The parse result must depend on:
 *
 *     - the canonical token sequence;
 *     - the grammar version.
 *
 * Parsing must not depend on:
 *
 *     system time;
 *     randomness;
 *     environment state;
 *     hardware discovery;
 *     network state;
 *     device state;
 *     runtime scheduling.
 *
 * Macro resolution and expansion have their own deterministic compiler
 * contract.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Merely parsing a macro expression must have no external side effects.
 *
 * Macro syntax must not implicitly grant:
 *
 *     filesystem access;
 *     network access;
 *     subprocess execution;
 *     secret access;
 *     hardware access;
 *     arbitrary host-language execution.
 *
 * Any future compile-time capability system must explicitly authorize such
 * capabilities outside the parser grammar.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * This file is responsible only for the expression-side syntactic boundary.
 *
 * Structural parser diagnostics include malformed macro-expression placement
 * when encountered by the canonical expression parser.
 *
 * Examples of semantic diagnostics that MUST remain downstream:
 *
 *     unknown macro;
 *     inaccessible macro;
 *     invalid argument count;
 *     invalid argument type;
 *     invalid generic argument;
 *     unavailable capability;
 *     forbidden expansion;
 *     expansion failure;
 *     expansion resource-policy violation.
 *
 * Diagnostics should retain source spans through the existing parser/AST
 * infrastructure.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing macro invocation syntax remains:
 *
 *     macroPath BANG LPAREN argumentList? RPAREN
 *
 * Examples:
 *
 *     build!()
 *     build!(value)
 *     build!(a, b)
 *     module::build!(value)
 *     package::module::build!(a, b)
 *
 * This file does not change that syntax.
 *
 * It only establishes how the already-canonical `macroExpression` enters the
 * expression hierarchy.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * REQUIRED EXISTING COMPONENTS
 * ----------------------------
 *
 *     grammar/macros/invocations.g4
 *
 * Must remain the owner of:
 *
 *     macroPath
 *     macroInvocation
 *     macroExpression
 *
 *     grammar/macros/declarations.g4
 *
 * Must remain the owner of macro declarations.
 *
 *     grammar/macros/macros.g4
 *
 * Must remain the macro composition boundary.
 *
 *     grammar/expressions/expression.g4
 *     or the repository's canonical expression composition owner
 *
 * must integrate:
 *
 *     macroExpression
 *
 * into:
 *
 *     primaryExpression
 *
 * or the equivalent canonical lowest-level expression production.
 *
 * REQUIRED AST DESTINATION
 * ------------------------
 *
 *     src/frontend/ast/node/expressions/macro.rs
 *
 * remains the frontend macro-expression representation.
 *
 * No grammar-local AST is introduced.
 *
 * REQUIRED SEMANTIC DESTINATION
 * -----------------------------
 *
 * Macro resolution, expansion, hygiene, provenance, and semantic validation
 * occur after parsing.
 *
 * REQUIRED IR DESTINATION
 * -----------------------
 *
 * Macro-generated source is semantically analyzed after expansion.
 *
 * Resulting domain constructs lower through the repository's canonical semantic
 * boundaries.
 *
 * Quantum constructs eventually cross:
 *
 *     quantum::ir
 *
 * rather than a macro-specific quantum IR.
 *
 * ============================================================================
 * INTEGRATION ORDER
 * ============================================================================
 *
 * This file can be completed independently because its contract depends only
 * on the already-established macro invocation and expression ownership model.
 *
 * Integration should then proceed in this order:
 *
 *     1. lexer vocabulary
 *            |
 *     2. grammar/macros/invocations.g4
 *            |
 *     3. grammar/expressions/macros.g4
 *            |
 *     4. canonical expression composition
 *            |
 *     5. canonical parser composition
 *            |
 *     6. frontend AST lowering
 *            |
 *     7. macro resolution
 *            |
 *     8. controlled expansion
 *            |
 *     9. hygiene/provenance
 *           |
 *    10. semantic analysis
 *           |
 *    11. canonical semantic IR
 *           |
 *    12. domain IR
 *           |
 *    13. compiler/lowering
 *           |
 *    14. runtime
 *
 * No later component should require this file to be rewritten merely because
 * the implementation of expansion, AST lowering, semantic analysis, quantum
 * lowering, HDL lowering, or runtime execution changes.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive expression integration:
 *
 *     build!()
 *     build!(value)
 *     build!(a, b)
 *     math::build!(value)
 *     package::math::build!(a, b)
 *
 * Macro expressions must be accepted anywhere the canonical expression
 * grammar accepts a primary expression, subject to the ordinary expression
 * composition rules.
 *
 * Nested expression coverage:
 *
 *     build!(x + y)
 *     build!(condition ? a : b)
 *     build!(call(x))
 *     build!(array[index])
 *     build!(quantum_value)
 *     build!(tensor[index])
 *
 * Nested macro coverage:
 *
 *     outer!(inner!(value))
 *
 * provided that the canonical argument grammar permits expressions of that
 * form.
 *
 * Composition coverage:
 *
 *     build!() + value
 *     value + build!()
 *     build!(value).member
 *     build!(value)[index]
 *     build!(value)(argument)
 *
 * where the canonical postfix-expression semantics permit the construct.
 *
 * Negative coverage must include malformed structures such as:
 *
 *     build!
 *     build!(
 *     build!()
 *     build!(,)
 *     build!(a,,b)
 *
 * where rejection is required by the canonical invocation/argument grammar.
 *
 * These tests belong in the canonical parser conformance suite; this file
 * must not duplicate the macro invocation test corpus.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Verify:
 *
 *     - empty macro argument list;
 *     - one argument;
 *     - many arguments;
 *     - trailing-comma policy;
 *     - deeply qualified macro names;
 *     - nested macro expressions;
 *     - nested ordinary expressions;
 *     - macro expression inside larger binary expressions;
 *     - macro expression inside conditional expressions;
 *     - macro expression inside indexing;
 *     - macro expression inside calls;
 *     - macro expression surrounding domain-specific syntax.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Test growth with available resources rather than fixed language limits:
 *
 *     - many macro invocations;
 *     - large argument lists;
 *     - deeply nested expressions;
 *     - deeply qualified paths;
 *     - large generated source;
 *     - large quantum-generated source;
 *     - large HDL-generated source;
 *     - large distributed/generated source.
 *
 * Tests must establish that no artificial grammar constant limits these cases.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file must contain:
 *
 *     no hardware identifiers;
 *     no device identifiers;
 *     no physical qubit identifiers;
 *     no processor counts;
 *     no thread counts;
 *     no memory capacities;
 *     no topology sizes;
 *     no accelerator counts;
 *     no fixed gate sets;
 *     no backend names as universal syntax;
 *     no MAX_* language limits.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when all of the following are true:
 *
 *     [x] Owns only expression-side macro integration.
 *     [x] Does not duplicate macro invocation syntax.
 *     [x] Does not duplicate macro declaration syntax.
 *     [x] Does not define lexer rules.
 *     [x] Does not introduce hardware limits.
 *     [x] Does not introduce domain-specific macro variants.
 *     [x] Preserves POCO-REAF.
 *     [x] Preserves domain neutrality.
 *     [x] Preserves the canonical frontend AST boundary.
 *     [x] Preserves quantum::ir as the quantum semantic boundary.
 *     [x] Does not execute macro expansion during parsing.
 *     [x] Does not require unsafe Rust.
 *     [x] Does not introduce circular grammar dependencies.
 *     [x] Documents the exact integration point.
 *     [x] Defines positive/negative/boundary/scalability coverage.
 *     [x] Defines downstream AST/semantic/IR/compiler/runtime ownership.
 *
 * Final integration requirement:
 *
 *     The canonical expression composition grammar must consume the existing
 *     `macroExpression` rule from `grammar/macros/invocations.g4`.
 *
 * This file itself must not become another owner of that rule.
 *
 * ============================================================================
 */

parser grammar macros;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * EXPRESSION-SIDE MACRO INTEGRATION
 * ============================================================================
 *
 * This rule intentionally delegates the actual invocation syntax to the
 * canonical macro grammar.
 *
 * Ownership:
 *
 *     macroExpression
 *         -> grammar/macros/invocations.g4
 *
 * This wrapper gives the expression subsystem a stable integration symbol
 * without duplicating the invocation grammar.
 *
 * ============================================================================
 */

macroExpressionInExpression
    : macroExpression
    ;