/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/macros/invocations.g4
 *
 * Grammar:
 *     invocations
 *
 * Purpose:
 *     Canonical parser grammar for Zamani macro invocation syntax.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - macro invocation syntax;
 *     - macro invocation paths;
 *     - macro invocation expression integration.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - qualified names;
 *     - ordinary function calls;
 *     - argument-list syntax;
 *     - expressions;
 *     - macro declarations;
 *     - macro parameters;
 *     - macro defaults;
 *     - macro bodies;
 *     - macro expansion;
 *     - macro resolution;
 *     - hygiene;
 *     - reflection;
 *     - compile-time execution;
 *     - semantic analysis;
 *     - type checking;
 *     - capability checking;
 *     - resource checking;
 *     - target selection;
 *     - hardware discovery;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - classical IR;
 *     - HDL IR;
 *     - runtime execution.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * ZamaniLexer
 *   |
 *   v
 * Canonical Zamani Parser
 *   |
 *   +--> macro invocation syntax
 *   |
 *   v
 * Frontend AST
 *   |
 *   v
 * Name Resolution
 *   |
 *   v
 * Macro Resolution
 *   |
 *   v
 * Macro Expansion
 *   |
 *   v
 * Hygiene / Provenance
 *   |
 *   v
 * Semantic Analysis
 *   |
 *   v
 * Canonical Semantic IR
 *   |
 *   +--> Classical IR
 *   +--> quantum::ir
 *   +--> HDL / hardware representation
 *   +--> distributed representation
 *   +--> future domain representations
 *   |
 *   v
 * Optimization
 *   |
 *   v
 * Routing / Scheduling / Resilience / Lowering
 *   |
 *   v
 * Runtime
 *
 * Grammar syntax MUST NOT depend on the IR, runtime, hardware, scheduler,
 * optimizer, QEC, ZQN, or resilience implementation.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Macro invocation is a source-level construct.
 *
 * It MUST NOT inherently select:
 *
 *     - CPU;
 *     - GPU;
 *     - FPGA;
 *     - ASIC;
 *     - QPU;
 *     - simulator;
 *     - accelerator;
 *     - backend;
 *     - device;
 *     - topology;
 *     - memory capacity;
 *     - processor count;
 *     - qubit count;
 *     - register count;
 *     - deployment topology.
 *
 * The same invocation syntax must remain valid regardless of whether the
 * expanded program eventually executes on a tiny embedded target or a
 * heterogeneous distributed system.
 *
 * Physical requirements belong to the appropriate resource/capability/target
 * layers.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No language-level finite limit is imposed on:
 *
 *     - macro invocation count;
 *     - argument count;
 *     - qualified-name depth;
 *     - source size;
 *     - nesting;
 *     - expansion result size.
 *
 * This grammar deliberately contains no:
 *
 *     MAX_ARGUMENTS
 *     MAX_MACRO_DEPTH
 *     MAX_INVOCATIONS
 *     MAX_NAMES
 *     MAX_QUANTUM_SIZE
 *     MAX_DEVICE_COUNT
 *
 * or equivalent constants.
 *
 * Compiler implementations MAY impose configurable resource budgets for:
 *
 *     - source bytes;
 *     - token count;
 *     - parser memory;
 *     - AST nodes;
 *     - expansion depth;
 *     - expansion steps;
 *     - generated nodes;
 *     - compilation time;
 *     - diagnostics.
 *
 * Those are implementation/resource policies and are NOT grammar semantics.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * The Zamani compiler is implemented for Rust 1.97 / 1.97.1.
 *
 * This grammar contains no Rust executable code.
 *
 * The compiler implementation MUST use safe Rust.
 *
 * A Zamani source-language construct named `unsafe`, if supported elsewhere,
 * does not authorize Rust `unsafe`.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic with respect to the canonical token stream.
 *
 * This grammar does not perform macro resolution or expansion.
 *
 * Therefore deterministic expansion is a downstream compiler contract.
 *
 * Expansion must preserve:
 *
 *     - deterministic resolution;
 *     - deterministic expansion ordering;
 *     - source provenance;
 *     - diagnostic provenance;
 *     - hygiene;
 *     - reproducibility.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * The lexer remains the sole owner of lexical definitions.
 *
 * The canonical Zamani lexer must provide the token vocabulary required here,
 * including:
 *
 *     BANG
 *     LPAREN
 *     RPAREN
 *
 * No lexer rules are defined in this file.
 *
 * In particular, this file MUST NOT define:
 *
 *     BANG : '!' ;
 *
 * or another macro-specific lexer token.
 *
 * ============================================================================
 * SHARED PARSER CONTRACT
 * ============================================================================
 *
 * This grammar consumes canonical parser rules supplied by the composed
 * Zamani parser.
 *
 * Required shared rule:
 *
 *     qualifiedName
 *
 * Required shared rule:
 *
 *     argumentList
 *
 * The expression grammar integrates:
 *
 *     macroExpression
 *
 * into its canonical expression hierarchy.
 *
 * This file MUST NOT redefine those shared rules.
 *
 * ============================================================================
 * MACRO PATH CONTRACT
 * ============================================================================
 *
 * A macro path is structurally a canonical qualified name.
 *
 * Examples:
 *
 *     build
 *     math::build
 *     package::math::build
 *
 * The grammar recognizes the structure only.
 *
 * It does NOT decide whether the path:
 *
 *     - exists;
 *     - is imported;
 *     - is visible;
 *     - names a macro;
 *     - is ambiguous;
 *     - refers to a package;
 *     - refers to a dialect;
 *     - refers to another symbol.
 *
 * Name resolution owns those decisions.
 *
 * ============================================================================
 * INVOCATION CONTRACT
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     name!()
 *
 *     name!(argument)
 *
 *     name!(argument1, argument2)
 *
 * Qualified syntax:
 *
 *     module::name!(argument)
 *
 *     package::module::name!(argument)
 *
 * The `!` token distinguishes macro invocation syntax from ordinary function
 * invocation syntax.
 *
 * Therefore:
 *
 *     foo(x)
 *
 * and:
 *
 *     foo!(x)
 *
 * remain structurally distinct.
 *
 * ============================================================================
 * ARGUMENT CONTRACT
 * ============================================================================
 *
 * Macro invocation reuses the canonical `argumentList` rule.
 *
 * This is intentional.
 *
 * It prevents the macro system from creating a second incompatible argument
 * language.
 *
 * The canonical argument-list owner determines:
 *
 *     - argument ordering;
 *     - separators;
 *     - trailing comma policy;
 *     - supported argument forms.
 *
 * This file only determines that an optional canonical argument list occurs
 * between the macro invocation parentheses.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must provide sufficient structure for the frontend AST to create
 * the repository's canonical macro invocation node.
 *
 * A macro invocation must preserve, directly or indirectly:
 *
 *     - source span;
 *     - invocation path;
 *     - ordered argument references;
 *     - lexical/source identity;
 *     - provenance.
 *
 * The AST layer, not this grammar, determines whether these are represented
 * using NodeIds, source spans, interned names, syntax nodes, or another
 * canonical repository representation.
 *
 * This grammar MUST NOT create a second macro AST hierarchy.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Syntactic validity does not imply semantic validity.
 *
 * For example:
 *
 *     unknown!(x)
 *
 * may be syntactically valid.
 *
 * It may subsequently fail because:
 *
 *     - no macro named `unknown` exists;
 *     - the macro is not visible;
 *     - argument matching fails;
 *     - generic constraints fail;
 *     - capabilities are unavailable;
 *     - effects are incompatible;
 *     - expansion is forbidden;
 *     - expansion exceeds compiler policy.
 *
 * Such failures MUST NOT be encoded as parser productions.
 *
 * ============================================================================
 * DOMAIN-NEUTRAL CONTRACT
 * ============================================================================
 *
 * Macro invocation syntax is domain-neutral.
 *
 * Arguments may eventually describe or construct:
 *
 *     - classical computation;
 *     - quantum computation;
 *     - hybrid computation;
 *     - HDL;
 *     - hardware;
 *     - distributed computation;
 *     - AI/ML;
 *     - networking;
 *     - cryptography;
 *     - scientific computation;
 *     - future computing paradigms.
 *
 * This grammar does not determine the resulting domain.
 *
 * Semantic analysis determines the domain after macro resolution and
 * expansion.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Macro invocations may construct quantum syntax, but this grammar does not
 * define quantum semantics.
 *
 * There are deliberately no assumptions about:
 *
 *     - qubit count;
 *     - physical qubits;
 *     - logical qubits;
 *     - QPU size;
 *     - quantum topology;
 *     - gate set;
 *     - backend;
 *     - device.
 *
 * Quantum syntax produced after expansion must eventually lower through the
 * canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT create, modify, or duplicate quantum::ir.
 *
 * ============================================================================
 * HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware-specific implementation is outside this grammar.
 *
 * An invocation does not inherently mean:
 *
 *     use GPU X
 *     use FPGA Y
 *     use QPU Z
 *     use N cores
 *     use N devices
 *
 * If the program expresses a genuine hardware/resource requirement, that
 * requirement is handled by the canonical resource/capability/target
 * language and semantic layers.
 *
 * ============================================================================
 * EXPANSION CONTRACT
 * ============================================================================
 *
 * Parsing stops at the invocation structure.
 *
 * The parser MUST NOT:
 *
 *     - execute a macro;
 *     - expand a macro;
 *     - resolve a macro;
 *     - read files;
 *     - write files;
 *     - access the network;
 *     - execute processes;
 *     - inspect hardware;
 *     - access secrets;
 *     - select a backend.
 *
 * Expansion is a separate compiler phase.
 *
 * ============================================================================
 * HYGIENE CONTRACT
 * ============================================================================
 *
 * This grammar does not implement hygiene.
 *
 * The invocation must nevertheless preserve enough source identity/provenance
 * for the hygiene subsystem to associate generated syntax with its invocation
 * origin.
 *
 * Hygiene must be implemented after parsing and before the expanded syntax is
 * treated as ordinary semantic input.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Merely parsing:
 *
 *     macro!(...)
 *
 * must have no observable external side effects.
 *
 * Security-sensitive macro capabilities, if Zamani supports them, must be
 * checked by compiler policy and capability infrastructure rather than by
 * this parser grammar.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Parser diagnostics belong to this grammar.
 *
 * Semantic diagnostics do not.
 *
 * Parser errors include malformed invocation structure, for example:
 *
 *     foo!(
 *     foo!)
 *     foo!(,)
 *
 * when those structures violate the canonical argument grammar.
 *
 * Semantic errors include:
 *
 *     unknown macro;
 *     inaccessible macro;
 *     invalid argument type;
 *     invalid argument count;
 *     invalid generic arguments;
 *     unavailable compile-time capability;
 *     invalid expansion;
 *     expansion budget exceeded.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The existing Zamani macro invocation shape is preserved:
 *
 *     macroPath BANG LPAREN argumentList? RPAREN
 *
 * This avoids unnecessarily breaking existing source programs.
 *
 * Future syntax changes require:
 *
 *     - language-version policy;
 *     - migration documentation;
 *     - positive compatibility tests;
 *     - negative compatibility tests;
 *     - parser diagnostics.
 *
 * ============================================================================
 * LEGACY INTEGRATION CONTRACT
 * ============================================================================
 *
 * The repository currently has macro invocation logic in:
 *
 *     grammar/macros/macros.g4
 *
 * and macro-related invocation logic in:
 *
 *     grammar/antlr/Meta.g4
 *
 * Those files must NOT remain competing canonical owners.
 *
 * After migration:
 *
 *     grammar/macros/invocations.g4
 *
 * is the canonical owner of:
 *
 *     macroPath
 *     macroInvocation
 *     macroExpression
 *
 * `macros.g4` becomes the macro composition layer.
 *
 * `Meta.g4` must either:
 *
 *     - import/delegate to the canonical macro grammar;
 *     - or have its duplicate macro productions removed as part of the
 *       grammar-authority migration.
 *
 * No downstream file should need to edit this file merely because those
 * migrations occur.
 *
 * ============================================================================
 * NO CIRCULAR DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Valid direction:
 *
 *     lexer
 *       |
 *       v
 *     shared parser rules
 *       |
 *       v
 *     invocations.g4
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     IR
 *       |
 *       v
 *     optimization / routing / scheduling / runtime
 *
 * Invalid directions include:
 *
 *     invocation grammar -> quantum::ir
 *     invocation grammar -> runtime
 *     invocation grammar -> scheduler
 *     invocation grammar -> hardware
 *     invocation grammar -> ZQN
 *     invocation grammar -> QEC
 *     IR -> invocation grammar
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 *
 * The grammar name exactly matches the filename:
 *
 *     invocations.g4
 *
 * ============================================================================
 */

parser grammar invocations;


options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * MACRO PATH
 * ============================================================================
 *
 * Ownership:
 *
 *     This rule owns the macro-invocation use of a qualified name.
 *
 * Non-ownership:
 *
 *     qualifiedName itself remains owned by the canonical names/path grammar.
 *
 * Examples:
 *
 *     compute
 *     math::compute
 *     package::math::compute
 *
 * No finite namespace depth is imposed here.
 * The canonical qualifiedName grammar determines its syntax.
 *
 * ============================================================================
 */

macroPath
    : qualifiedName
    ;


/*
 * ============================================================================
 * MACRO INVOCATION
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     macroName!()
 *
 *     macroName!(argument)
 *
 *     macroName!(argument1, argument2)
 *
 *     module::macroName!(argument)
 *
 * The optional argument list is deliberately delegated to the canonical
 * argumentList rule.
 *
 * ============================================================================
 */

macroInvocation
    : macroPath
      BANG
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * MACRO EXPRESSION
 * ============================================================================
 *
 * This is the sole expression-level integration boundary owned by the macro
 * invocation grammar.
 *
 * The canonical expression grammar must include:
 *
 *     macroExpression
 *
 * in its expression hierarchy exactly once.
 *
 * This prevents the macro system from defining a second expression grammar.
 *
 * ============================================================================
 */

macroExpression
    : macroInvocation
    ;


/*
 * ============================================================================
 * INTEGRATION NOTES
 * ============================================================================
 *
 * The canonical expression grammar should conceptually provide an alternative
 * equivalent to:
 *
 *     primaryExpression
 *         : macroExpression
 *         | ...
 *         ;
 *
 * The exact expression hierarchy and precedence remain owned by the
 * expressions grammar.
 *
 * Do NOT add expression rules here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] Grammar name is exactly `invocations`.
 *
 * [ ] Filename is exactly `invocations.g4`.
 *
 * [ ] `tokenVocab = ZamaniLexer` resolves to the canonical Zamani lexer.
 *
 * [ ] `BANG` is owned by the lexer.
 *
 * [ ] `LPAREN` is owned by the lexer.
 *
 * [ ] `RPAREN` is owned by the lexer.
 *
 * [ ] `qualifiedName` has one canonical owner outside this file.
 *
 * [ ] `argumentList` has one canonical owner outside this file.
 *
 * [ ] `macroPath` has one canonical owner in the macro grammar.
 *
 * [ ] `macroInvocation` has one canonical owner in the macro grammar.
 *
 * [ ] `macroExpression` has one canonical owner in the macro grammar.
 *
 * [ ] `macroDeclaration` is NOT defined here.
 *
 * [ ] `macroParameter` is NOT defined here.
 *
 * [ ] `macroBody` is NOT defined here.
 *
 * [ ] macro expansion is NOT performed here.
 *
 * [ ] hygiene is NOT implemented here.
 *
 * [ ] reflection is NOT implemented here.
 *
 * [ ] compile-time execution is NOT implemented here.
 *
 * [ ] no filesystem access is possible from this grammar.
 *
 * [ ] no network access is possible from this grammar.
 *
 * [ ] no process execution is possible from this grammar.
 *
 * [ ] no hardware-specific assumptions exist.
 *
 * [ ] no quantum-machine assumptions exist.
 *
 * [ ] no fixed resource limits exist.
 *
 * [ ] no fixed qubit count exists.
 *
 * [ ] no fixed CPU/GPU/FPGA count exists.
 *
 * [ ] no topology is encoded.
 *
 * [ ] macro invocation remains domain-neutral.
 *
 * [ ] the canonical expression grammar integrates macroExpression once.
 *
 * [ ] `macros.g4` no longer duplicates macroInvocation ownership.
 *
 * [ ] `Meta.g4` no longer duplicates macroInvocation ownership.
 *
 * [ ] AST construction uses the repository's canonical AST.
 *
 * [ ] source spans/provenance are preserved.
 *
 * [ ] positive tests pass.
 *
 * [ ] negative syntax tests pass.
 *
 * [ ] boundary tests pass.
 *
 * [ ] scalability tests pass.
 *
 * [ ] deterministic parsing tests pass.
 *
 * [ ] cross-domain tests pass.
 *
 * [ ] compatibility tests pass.
 *
 * [ ] the complete composed ANTLR grammar generates successfully.
 *
 * ============================================================================
 */