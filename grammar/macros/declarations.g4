/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/macros/declarations.g4
 *
 * Role:
 *     Canonical parser component for Zamani macro declarations.
 *
 * Architectural position:
 *
 *     UTF-8 source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical Zamani parser
 *          |
 *          +---- core syntax
 *          +---- declarations
 *          +---- expressions
 *          +---- modules
 *          +---- macros/declarations.g4
 *          |
 *          v
 *       Frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +---- name resolution
 *          +---- type analysis
 *          +---- effect analysis
 *          +---- capability analysis
 *          +---- resource analysis
 *          +---- macro resolution
 *          +---- controlled expansion
 *          |
 *          v
 *     canonical semantic IR
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - macro declaration syntax;
 *   - macro declaration visibility;
 *   - macro declaration name;
 *   - macro declaration generic parameters;
 *   - macro parameter list syntax;
 *   - macro parameter names;
 *   - optional macro parameter type syntax;
 *   - optional macro parameter defaults;
 *   - macro declaration body syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical token definitions;
 *   - the canonical identifier rule;
 *   - generic parameter semantics;
 *   - type semantics;
 *   - ordinary expression semantics;
 *   - ordinary function parameter semantics;
 *   - macro invocation syntax;
 *   - macro expression syntax;
 *   - macro path resolution;
 *   - macro expansion;
 *   - macro hygiene;
 *   - token generation;
 *   - source generation;
 *   - procedural execution;
 *   - filesystem access;
 *   - network access;
 *   - package downloading;
 *   - process execution;
 *   - compiler configuration;
 *   - target selection;
 *   - backend selection;
 *   - hardware selection;
 *   - QPU selection;
 *   - CPU/GPU selection;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - canonical IR construction.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Macro declarations describe reusable source-level transformations.
 *
 * They MUST NOT require a particular:
 *
 *   - processor;
 *   - processor count;
 *   - core count;
 *   - thread count;
 *   - GPU;
 *   - accelerator;
 *   - FPGA;
 *   - ASIC;
 *   - quantum processor;
 *   - qubit count;
 *   - register width;
 *   - quantum topology;
 *   - gate set;
 *   - memory size;
 *   - network topology;
 *   - machine topology;
 *   - backend;
 *   - scheduler;
 *   - routing strategy.
 *
 * Such information, when semantically required by a program, belongs to
 * capability/resource/target analysis outside this grammar.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are deliberately no grammar-level finite limits on:
 *
 *   - number of macro declarations;
 *   - number of macro parameters;
 *   - number of generic parameters;
 *   - parameter-name length;
 *   - macro-body size;
 *   - source size;
 *   - nesting depth;
 *   - namespace depth;
 *   - generated program size.
 *
 * Repetition is expressed using ANTLR repetition operators rather than
 * enumerating artificial capacities.
 *
 * Compiler implementations MAY impose configurable resource budgets for:
 *
 *   - source bytes;
 *   - token count;
 *   - AST nodes;
 *   - parser stack/resource consumption;
 *   - macro expansion depth;
 *   - expansion steps;
 *   - generated nodes;
 *   - generated source;
 *   - compilation memory;
 *   - compilation time.
 *
 * Those are implementation/resource policies and MUST NOT be encoded as
 * language-level grammar limits.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * Zamani's compiler implementation target is:
 *
 *   Rust 1.97
 *   Rust 1.97.1
 *
 * The compiler implementation MUST use safe Rust.
 *
 * This grammar contains no executable Rust and therefore cannot require Rust
 * `unsafe`.
 *
 * A Zamani source-level `unsafe` construct, if supported elsewhere in the
 * language, does not authorize unsafe Rust in the compiler.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar is structural syntax only.
 *
 * Parsing must depend only on the canonical token stream and parser state.
 *
 * This file MUST NOT:
 *
 *   - perform I/O;
 *   - inspect the filesystem;
 *   - inspect the network;
 *   - query hardware;
 *   - inspect runtime state;
 *   - select a backend;
 *   - execute a macro;
 *   - expand a macro;
 *   - mutate compiler-global state.
 *
 * Macro resolution and expansion determinism are semantic/compiler contracts
 * and are intentionally outside this grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser/frontend must map this syntax into the repository's canonical
 * source-level macro representation.
 *
 * A macro declaration must preserve, directly or indirectly:
 *
 *   - source span;
 *   - source identity/provenance;
 *   - declaration identity;
 *   - visibility;
 *   - macro name;
 *   - ordered generic parameters;
 *   - ordered macro parameters;
 *   - parameter names;
 *   - optional parameter type syntax;
 *   - optional default expressions;
 *   - body;
 *   - declaration provenance.
 *
 * This grammar MUST NOT introduce a second macro AST hierarchy.
 *
 * In particular, the macro AST MUST NOT acquire fields such as:
 *
 *   qubit_count
 *   cpu_count
 *   gpu_count
 *   backend
 *   topology
 *   gate_set
 *   scheduler
 *   qec
 *   zqn
 *   device
 *
 * ============================================================================
 * SHARED-RULE CONTRACT
 * ============================================================================
 *
 * This parser component deliberately reuses canonical parser rules.
 *
 * Expected shared rules include:
 *
 *   visibility
 *   identifier
 *   genericParameters
 *   typeExpression
 *   expression
 *   blockExpression
 *
 * The final composed parser MUST have exactly one canonical owner for each
 * shared rule.
 *
 * This file therefore MUST NOT redefine those rules.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer supplies the MACRO token.
 *
 * The identifier, punctuation, type, expression and block syntax are supplied
 * by the canonical lexer/parser composition.
 *
 * No identifier-based keyword matching is used here.
 *
 * In particular, this file MUST NOT introduce alternatives such as:
 *
 *   'macro'
 *
 * when the canonical lexer already owns MACRO.
 *
 * ============================================================================
 * TRAILING-COMMA POLICY
 * ============================================================================
 *
 * Macro parameter lists accept an optional trailing comma.
 *
 * Examples:
 *
 *   macro f() { }
 *
 *   macro f(a) { }
 *
 *   macro f(a, b) { }
 *
 *   macro f(a, b,) { }
 *
 * This policy is intentionally local to macro declarations.
 *
 * The ordinary runtime `parameterList` is not modified by this file.
 *
 * ============================================================================
 * PARAMETER SEMANTICS
 * ============================================================================
 *
 * Macro parameters are distinct from runtime function parameters at the
 * semantic level even though both use identifiers and may contain type
 * expressions.
 *
 * A macro parameter may syntactically contain:
 *
 *   name
 *   name: Type
 *   name = expression
 *   name: Type = expression
 *
 * The grammar does NOT decide whether a particular type represents:
 *
 *   expression syntax
 *   token syntax
 *   source syntax
 *   type syntax
 *   pattern syntax
 *   compile-time data
 *   another future meta representation
 *
 * Such distinctions belong to semantic macro analysis.
 *
 * ============================================================================
 * DEFAULT-VALUE CONTRACT
 * ============================================================================
 *
 * Defaults are ordinary Zamani expressions.
 *
 * This file does not evaluate defaults.
 *
 * Evaluation, validation, dependency checking, purity, determinism and
 * compile-time admissibility belong to semantic/compiler infrastructure.
 *
 * ============================================================================
 * BODY CONTRACT
 * ============================================================================
 *
 * The canonical macro body is a block expression.
 *
 * The body uses the canonical blockExpression rule so that macro bodies
 * participate in the same source structure, diagnostics and source mapping
 * infrastructure as the rest of Zamani.
 *
 * This file deliberately does not create a second macro-body language.
 *
 * ============================================================================
 * DOMAIN INDEPENDENCE
 * ============================================================================
 *
 * Macro declarations are domain-neutral.
 *
 * A macro may ultimately generate syntax participating in:
 *
 *   - classical computation;
 *   - quantum computation;
 *   - hybrid computation;
 *   - HDL;
 *   - hardware;
 *   - distributed computation;
 *   - AI/ML;
 *   - data processing;
 *   - networking;
 *   - security;
 *   - future Zamani domains.
 *
 * This file does not classify the generated program.
 *
 * Classification occurs after parsing and macro expansion through normal
 * semantic analysis.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Canonical parser integration:
 *
 *   item/declaration
 *       |
 *       +--> macroDeclaration
 *                  |
 *                  v
 *       macros/declarations.g4
 *
 * Expression integration is NOT owned here.
 *
 * Macro invocation belongs to:
 *
 *   grammar/macros/macros.g4
 *
 * or another explicitly designated canonical macro-invocation component.
 *
 * Therefore this file MUST NOT define:
 *
 *   macroInvocation
 *   macroExpression
 *   macroPath
 *
 * Doing so would create ownership overlap with macros.g4.
 *
 * ============================================================================
 * IMPORTANT EXISTING-REPOSITORY INTEGRATION
 * ============================================================================
 *
 * The repository currently contains:
 *
 *   grammar/macros/macros.g4
 *
 * which already contains macroDeclaration and supporting declaration rules.
 *
 * Once this file becomes canonical, those declaration productions in
 * macros.g4 MUST be removed from macros.g4 or converted into imports/composed
 * references.
 *
 * macros.g4 should retain invocation/composition responsibilities.
 *
 * The repository also contains:
 *
 *   grammar/antlr/Meta.g4
 *
 * Meta.g4 contains a broader meta-language design, including macro
 * declarations and invocations.
 *
 * The final grammar architecture MUST designate exactly one canonical source
 * grammar for each production. Meta.g4 MUST NOT become a third competing
 * definition of macroDeclaration.
 *
 * Migration from existing definitions must preserve source compatibility
 * according to the language-version policy.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This is a parser grammar component.
 *
 * The exact final composition mechanism is determined by the repository's
 * canonical ANTLR build architecture.
 *
 * Conceptually:
 *
 *   parser grammar ZamaniParser;
 *
 *   options {
 *       tokenVocab = ZamaniLexer;
 *   }
 *
 *   import macros/declarations;
 *
 * The generated parser must expose one macroDeclaration rule.
 *
 * If the canonical repository uses a different ANTLR import naming convention,
 * the build configuration may adapt the generated grammar name, but the
 * semantic ownership defined here remains unchanged.
 *
 * ============================================================================
 * ERROR-BOUNDARY CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Examples:
 *
 *   macro { }
 *   macro f( { }
 *   macro f(a: ) { }
 *   macro f(a = ) { }
 *   macro f(a,,b) { }
 *
 * are parser-invalid when the corresponding token sequence cannot satisfy the
 * grammar.
 *
 * The following are NOT syntax errors merely because of this file:
 *
 *   - unknown macro;
 *   - duplicate macro;
 *   - inaccessible macro;
 *   - invalid macro argument type;
 *   - invalid generic argument;
 *   - recursive macro expansion;
 *   - expansion budget exhaustion;
 *   - generated-resource exhaustion;
 *   - unavailable capability;
 *   - unavailable hardware;
 *   - unsupported target.
 *
 * Those belong to later compiler phases.
 *
 * ============================================================================
 * VERSIONING CONTRACT
 * ============================================================================
 *
 * Changes to this grammar are language-syntax changes.
 *
 * Backward-compatible additions should:
 *
 *   - avoid changing the interpretation of existing valid programs;
 *   - avoid introducing ambiguity with existing syntax;
 *   - avoid stealing existing identifiers unless intentionally versioned;
 *   - update grammar tests;
 *   - update grammar documentation;
 *   - update compatibility documentation;
 *   - update parser fixtures.
 *
 * Breaking changes require an explicit language-version/migration policy.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *   - qubit limits;
 *   - CPU limits;
 *   - GPU limits;
 *   - device counts;
 *   - memory limits;
 *   - topology definitions;
 *   - fixed register widths;
 *   - fixed namespace depth;
 *   - fixed parameter count;
 *   - fixed macro count;
 *   - fixed body size.
 *
 * Any implementation limit must be represented outside this grammar as an
 * explicit, configurable compiler/resource policy.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The corresponding test suite must cover at least:
 *
 * POSITIVE:
 *
 *   macro empty() { }
 *   macro one(a) { }
 *   macro many(a, b, c) { }
 *   macro trailing(a, b,) { }
 *   macro typed(a: T) { }
 *   macro defaulted(a = value) { }
 *   macro typedDefaulted(a: T = value) { }
 *   macro generic<T>(a: T) { }
 *   pub macro exported<T>(a: T) { }
 *
 * CROSS-DOMAIN:
 *
 *   macro wrapping classical syntax;
 *   macro wrapping quantum syntax;
 *   macro wrapping hybrid syntax;
 *   macro wrapping HDL syntax;
 *   macro wrapping resource/capability syntax;
 *   macro wrapping distributed syntax.
 *
 * The macro declaration grammar itself must remain domain-neutral.
 *
 * NEGATIVE:
 *
 *   missing macro name;
 *   missing parameter closing delimiter;
 *   malformed parameter;
 *   malformed type;
 *   malformed default;
 *   duplicate separators;
 *   missing body.
 *
 * SCALABILITY:
 *
 *   large parameter lists;
 *   large bodies;
 *   large generic parameter lists;
 *   deeply qualified names through invocation tests;
 *   large generated-source fixtures through macro-engine tests.
 *
 * No test may assume a finite language-level maximum unless the limit is
 * explicitly part of the language specification.
 *
 * DETERMINISM:
 *
 *   identical token streams produce identical parse trees;
 *   repeated parsing produces identical structural results.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *   1. It is the sole canonical grammar owner of macro declarations.
 *
 *   2. It composes with the canonical Zamani lexer.
 *
 *   3. It reuses canonical identifier, visibility, generic, type, expression
 *      and block rules.
 *
 *   4. It introduces no duplicate AST model.
 *
 *   5. It introduces no hardware/resource limits.
 *
 *   6. It introduces no execution or I/O behavior.
 *
 *   7. macros.g4 has an explicit integration path that removes duplicate
 *      declaration ownership.
 *
 *   8. grammar/antlr/Meta.g4 is reconciled so that it cannot create a second
 *      canonical macroDeclaration rule.
 *
 *   9. Positive, negative, boundary and deterministic parser tests pass.
 *
 *  10. Grammar documentation and language-version policy identify this file's
 *      ownership.
 *
 *  11. The generated parser compiles under the repository's supported ANTLR
 *      toolchain.
 *
 *  12. The Rust compiler/frontend implementation remains compatible with
 *      Rust 1.97 / Rust 1.97.1 and uses no unsafe Rust.
 *
 * ============================================================================
 */

parser grammar MacroDeclarations;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. MACRO DECLARATION
 * ========================================================================== */

/**
 * Canonical macro declaration:
 *
 *     macro name() {
 *         ...
 *     }
 *
 * Generic macro:
 *
 *     macro name<T>(value: T) {
 *         ...
 *     }
 *
 * Visibility is delegated to the canonical visibility rule.
 *
 * The declaration does not encode expansion behavior.
 */
macroDeclaration
    : visibility?
      MACRO
      identifier
      genericParameters?
      LPAREN
      macroParameterList?
      RPAREN
      blockExpression
    ;


/* ============================================================================
 * 2. MACRO PARAMETER LIST
 * ========================================================================== */

/**
 * Ordered macro parameters.
 *
 * Zero parameters are represented by omission of macroParameterList at the
 * call site:
 *
 *     macro name() { }
 *
 * One or more parameters are represented here.
 *
 * A trailing comma is intentionally accepted.
 */
macroParameterList
    : macroParameter
      (COMMA macroParameter)*
      COMMA?
    ;


/* ============================================================================
 * 3. MACRO PARAMETER
 * ========================================================================== */

/**
 * Macro parameter:
 *
 *     name
 *     name: Type
 *     name = expression
 *     name: Type = expression
 *
 * The parser records structure only.
 */
macroParameter
    : identifier
      (COLON typeExpression)?
      macroParameterDefault?
    ;


/* ============================================================================
 * 4. MACRO PARAMETER DEFAULT
 * ========================================================================== */

/**
 * Default values remain ordinary Zamani expressions.
 *
 * No compile-time evaluation occurs here.
 */
macroParameterDefault
    : ASSIGN expression
    ;