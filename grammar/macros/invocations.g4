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
 *     Canonical parser component for Zamani macro invocation syntax.
 *
 * ============================================================================
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * This grammar owns SOURCE SYNTAX ONLY.
 *
 * It owns:
 *
 *   - macro invocation paths;
 *   - qualified macro invocation names;
 *   - macro invocation delimiters;
 *   - macro invocation argument structure;
 *   - the macro-expression integration boundary.
 *
 * It does NOT own:
 *
 *   - lexical token definitions;
 *   - macro declarations;
 *   - macro parameters;
 *   - macro parameter defaults;
 *   - macro bodies;
 *   - macro expansion;
 *   - macro resolution;
 *   - overload resolution;
 *   - macro hygiene;
 *   - token generation;
 *   - source generation;
 *   - compile-time execution;
 *   - reflection;
 *   - specialization;
 *   - filesystem access;
 *   - network access;
 *   - process execution;
 *   - package fetching;
 *   - target selection;
 *   - backend selection;
 *   - CPU/GPU/QPU selection;
 *   - hardware discovery;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - canonical IR construction.
 *
 * ============================================================================
 * COMPILER PIPELINE
 * ============================================================================
 *
 * UTF-8 source
 *     |
 *     v
 * ZamaniLexer
 *     |
 *     v
 * canonical Zamani parser
 *     |
 *     +--> invocations.g4
 *     |
 *     v
 * frontend AST
 *     |
 *     v
 * name / type / effect / capability / resource analysis
 *     |
 *     v
 * macro resolution
 *     |
 *     v
 * controlled macro expansion
 *     |
 *     v
 * hygiene / provenance preservation
 *     |
 *     v
 * semantic analysis
 *     |
 *     v
 * canonical semantic IR
 *     |
 *     +--> classical IR
 *     +--> quantum::ir
 *     +--> HDL / hardware representation
 *     +--> distributed representation
 *     +--> other domain representations
 *     |
 *     v
 * optimization
 *     |
 *     v
 * routing / scheduling / resilience / target lowering
 *     |
 *     v
 * runtime
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Macro invocation syntax is intentionally independent of the machine on
 * which the resulting program eventually executes.
 *
 * A macro invocation MUST NOT inherently encode:
 *
 *   - a fixed qubit count;
 *   - a fixed CPU count;
 *   - a fixed GPU count;
 *   - a fixed FPGA count;
 *   - a fixed device;
 *   - a fixed accelerator;
 *   - a fixed memory capacity;
 *   - a fixed topology;
 *   - a fixed quantum topology;
 *   - a fixed gate set;
 *   - a fixed register width;
 *   - a fixed deployment topology;
 *   - a fixed scheduler;
 *   - a fixed backend.
 *
 * Hardware and execution requirements belong to the appropriate semantic
 * capability/resource/target layers.
 *
 * Therefore:
 *
 *     macro invocation
 *
 * expresses source-level program structure, not physical machine selection.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No language-level finite limits are imposed here on:
 *
 *   - the number of macro invocations;
 *   - the number of arguments;
 *   - qualified-name depth;
 *   - invocation nesting;
 *   - expression complexity;
 *   - source size;
 *   - generated semantic size.
 *
 * Grammar repetition is intentionally unbounded.
 *
 * Compiler implementations MAY impose configurable resource budgets for:
 *
 *   - source bytes;
 *   - token count;
 *   - parser memory;
 *   - AST nodes;
 *   - expansion depth;
 *   - expansion steps;
 *   - generated nodes;
 *   - compilation time;
 *   - diagnostics;
 *
 * Such limits are implementation/resource policies, not language semantics.
 *
 * They MUST therefore be represented outside this grammar and MUST NOT be
 * encoded as arbitrary parser constants.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic with respect to the canonical Zamani token stream.
 *
 * Macro resolution and expansion are NOT parser responsibilities.
 *
 * Later compiler stages must define deterministic policies for:
 *
 *   - name resolution;
 *   - overload selection;
 *   - expansion ordering;
 *   - recursive expansion;
 *   - hygiene;
 *   - provenance;
 *   - diagnostics;
 *   - resource-budget enforcement.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer owns the lexical representation of:
 *
 *     BANG
 *     LPAREN
 *     RPAREN
 *     COMMA
 *
 * and all identifier/path tokens used by the canonical parser.
 *
 * This file MUST NOT define lexer rules.
 *
 * In particular, do not add lexer rules such as:
 *
 *     BANG : '!' ;
 *
 *     MACRO_INVOCATION : ... ;
 *
 * to this parser grammar.
 *
 * The lexer must remain the sole lexical authority.
 *
 * ============================================================================
 * SHARED PARSER CONTRACT
 * ============================================================================
 *
 * This grammar intentionally reuses canonical parser rules.
 *
 * Required shared rules:
 *
 *     qualifiedName
 *     argumentList
 *
 * The final composed parser may also expose:
 *
 *     expression
 *     primaryExpression
 *     postfixExpression
 *
 * depending on the expression architecture.
 *
 * This file MUST NOT redefine those rules.
 *
 * The repository must have exactly one canonical owner for each shared rule.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must provide enough structural information for the frontend AST
 * to represent a macro invocation without introducing a second macro AST.
 *
 * A macro invocation node must preserve, directly or indirectly:
 *
 *     - source span;
 *     - invocation path;
 *     - lexical/source identity;
 *     - ordered arguments;
 *     - child expression NodeIds where the AST architecture uses NodeIds;
 *     - provenance;
 *     - expansion origin information when later expansion occurs.
 *
 * This grammar does NOT decide the final AST representation.
 *
 * It only establishes the source structure from which the canonical AST is
 * constructed.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * A syntactically valid invocation does NOT imply that the invocation is
 * semantically valid.
 *
 * For example:
 *
 *     unknown_macro!(x)
 *
 * may be syntactically valid but semantically unresolved.
 *
 * Similarly:
 *
 *     macro_name!(x, y)
 *
 * may be syntactically valid while later failing:
 *
 *     - parameter matching;
 *     - type checking;
 *     - capability checking;
 *     - effect checking;
 *     - visibility checking;
 *     - module resolution;
 *     - generic constraint checking;
 *     - macro availability;
 *     - expansion policy.
 *
 * Those are downstream semantic responsibilities.
 *
 * ============================================================================
 * DOMAIN INDEPENDENCE
 * ============================================================================
 *
 * Macro invocation syntax is domain-neutral.
 *
 * A macro may ultimately generate syntax associated with:
 *
 *     - classical computing;
 *     - quantum computing;
 *     - hybrid computing;
 *     - HDL;
 *     - hardware;
 *     - distributed computing;
 *     - HPC;
 *     - AI/ML;
 *     - networking;
 *     - cryptography;
 *     - embedded systems;
 *     - future Zamani dialects.
 *
 * This grammar does not determine the domain of the generated construct.
 *
 * Semantic analysis determines the meaning after expansion.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Macro invocation syntax MUST remain independent of quantum hardware.
 *
 * This grammar therefore contains no:
 *
 *     MAX_QUBITS
 *     qubit_count
 *     physical_device
 *     topology
 *     gate_set
 *     backend
 *     QPU identifier
 *
 * and contains no special invocation syntax for a particular quantum machine.
 *
 * Quantum syntax generated by a macro must ultimately enter the canonical
 * quantum semantic pipeline.
 *
 * In particular:
 *
 *     grammar
 *         |
 *         v
 *     syntax / AST
 *         |
 *         v
 *     semantic lowering
 *         |
 *         v
 *     quantum::ir
 *
 * This file does NOT create or duplicate quantum::ir.
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * Macro invocation syntax MUST remain independent of:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     accelerator count
 *     memory capacity
 *     physical topology
 *     device address
 *     deployment topology
 *
 * If an invocation supplies an explicit resource requirement, that requirement
 * is parsed as an ordinary canonical Zamani expression or resource construct.
 *
 * Interpretation belongs to resource/capability/target analysis.
 *
 * ============================================================================
 * QUOTATION / TOKEN-TREE BOUNDARY
 * ============================================================================
 *
 * This file intentionally does not invent quote/splice syntax.
 *
 * In particular, this grammar does not introduce ad-hoc rules such as:
 *
 *     quote
 *     unquote
 *     splice
 *     tokenTree
 *
 * merely to make macro invocation syntax appear more expressive.
 *
 * Such facilities require a coordinated contract among:
 *
 *     lexer
 *     parser
 *     AST
 *     source provenance
 *     hygiene
 *     expansion
 *     diagnostics
 *     formatter
 *     tooling
 *
 * They belong in the appropriate future macro/metaprogramming grammar layer.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Parser errors belong here.
 *
 * Examples:
 *
 *     foo!
 *     foo!(
 *     foo!(x
 *     foo!(,)
 *     foo!(x,,y)
 *
 * depending on the canonical argument-list contract.
 *
 * Semantic errors do NOT belong here.
 *
 * Examples:
 *
 *     unknown macro
 *     inaccessible macro
 *     wrong argument type
 *     wrong generic arguments
 *     unavailable capability
 *     invalid expansion
 *     expansion budget exceeded
 *     recursive expansion forbidden
 *
 * Those are downstream diagnostics.
 *
 * ============================================================================
 * TRAILING COMMA POLICY
 * ============================================================================
 *
 * The macro invocation delegates argument-list structure to the canonical
 * argumentList rule.
 *
 * This is deliberate.
 *
 * The macro grammar must not define one argument-list syntax while ordinary
 * function calls use another.
 *
 * Therefore:
 *
 *     macro!(a, b)
 *
 * and the canonical call argument structure share the same list semantics
 * wherever the language specification defines argumentList.
 *
 * If trailing commas are legal in canonical argumentList, they are legal for
 * macro invocation.
 *
 * If the language specification later changes the canonical policy, the
 * argumentList owner changes once; this file does not duplicate the policy.
 *
 * ============================================================================
 * PATH POLICY
 * ============================================================================
 *
 * macroPath delegates to qualifiedName.
 *
 * This permits module/package/namespace integration without introducing a
 * second namespace language.
 *
 * Examples:
 *
 *     build!(x)
 *     math::build!(x)
 *     quantum::circuit::build!(x)
 *
 * The parser recognizes structure only.
 *
 * It does NOT determine whether a path:
 *
 *     exists;
 *     is visible;
 *     names a macro;
 *     refers to a package;
 *     refers to a dialect;
 *     is imported;
 *     is ambiguous.
 *
 * Those decisions belong to name resolution.
 *
 * ============================================================================
 * NO DUPLICATE CALL LANGUAGE
 * ============================================================================
 *
 * Macro invocation is deliberately distinct from an ordinary runtime call
 * through the explicit BANG marker.
 *
 * Therefore:
 *
 *     foo(x)
 *
 * is not implicitly converted into:
 *
 *     foo!(x)
 *
 * and:
 *
 *     foo!(x)
 *
 * must not silently become an ordinary runtime function call.
 *
 * The distinction is semantic and must be preserved through the AST.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * macroExpression is the integration point for the canonical expression
 * grammar.
 *
 * The canonical expression grammar should include:
 *
 *     macroExpression
 *
 * at the appropriate primary/postfix expression position.
 *
 * This file must NOT copy the entire expression grammar.
 *
 * That prevents:
 *
 *     expression -> macroExpression -> expression
 *
 * cycles and prevents multiple expression definitions.
 *
 * ============================================================================
 * STATEMENT INTEGRATION
 * ============================================================================
 *
 * Macro invocation is an expression-level construct.
 *
 * Therefore a statement such as:
 *
 *     build!(x);
 *
 * should be accepted through the canonical expression-statement mechanism,
 * rather than through a separate:
 *
 *     macroStatement
 *
 * rule.
 *
 * This avoids duplicate syntax ownership.
 *
 * ============================================================================
 * COMPILE-TIME EXECUTION BOUNDARY
 * ============================================================================
 *
 * The BANG token does NOT mean:
 *
 *     execute immediately;
 *     execute while parsing;
 *     execute arbitrary host code;
 *     execute Rust;
 *     perform filesystem access;
 *     perform network access;
 *     invoke a shell;
 *     inspect hardware;
 *     select a backend.
 *
 * The parser recognizes syntax only.
 *
 * Any compile-time execution facility must be independently specified,
 * capability controlled, deterministic, auditable, and implemented in the
 * compiler/runtime infrastructure rather than inside this grammar.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing an invocation MUST NOT trigger:
 *
 *     - file reads;
 *     - file writes;
 *     - network requests;
 *     - subprocesses;
 *     - dynamic library loading;
 *     - environment mutation;
 *     - hardware access;
 *     - secret access.
 *
 * The grammar is declarative and side-effect free.
 *
 * The Rust compiler implementation for this subsystem must remain compatible
 * with the repository requirement of safe Rust on Rust 1.97 / 1.97.1.
 *
 * No unsafe Rust requirement is introduced by this grammar.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This grammar preserves the established invocation shape:
 *
 *     macroPath ! ( argumentList? )
 *
 * Existing repository macro syntax using:
 *
 *     name!(...)
 *
 * and qualified forms should remain source-compatible.
 *
 * Any future change to invocation syntax requires:
 *
 *     - language-version policy;
 *     - migration guidance;
 *     - compatibility tests;
 *     - parser diagnostics;
 *     - documentation updates.
 *
 * ============================================================================
 * CANONICAL OWNERSHIP AFTER INTEGRATION
 * ============================================================================
 *
 * The final macro grammar architecture is:
 *
 *     macros/
 *       macros.g4
 *       declarations.g4
 *       invocations.g4
 *       hygiene.g4
 *       expansion.g4
 *
 * Ownership:
 *
 *     declarations.g4
 *         -> macro declaration syntax
 *
 *     invocations.g4
 *         -> macro invocation syntax
 *
 *     hygiene.g4
 *         -> syntax-level hygiene constructs, if required
 *
 *     expansion.g4
 *         -> syntax-level expansion constructs, if required
 *
 *     macros.g4
 *         -> macro grammar composition/integration
 *
 * No two of these files should define macroInvocation.
 *
 * ============================================================================
 * LEGACY META INTEGRATION
 * ============================================================================
 *
 * grammar/antlr/Meta.g4 currently contains a macroInvocation production.
 *
 * That production must ultimately be removed from Meta.g4 or transformed into
 * a delegation/import boundary.
 *
 * Meta.g4 MUST NOT remain a second canonical owner of macroInvocation.
 *
 * The same applies to the current invocation production in:
 *
 *     grammar/macros/macros.g4
 *
 * After migration:
 *
 *     invocations.g4
 *
 * is the sole canonical owner.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The final canonical parser must compose this grammar with the shared
 * grammar components that own:
 *
 *     qualifiedName
 *     argumentList
 *     expression
 *
 * The exact composition mechanism is owned by the top-level parser grammar.
 *
 * This file intentionally does not redefine those shared rules.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Required lexical vocabulary:
 *
 *     ZamaniLexer
 *
 * Required parser rules:
 *
 *     qualifiedName
 *     argumentList
 *
 * Required downstream expression integration:
 *
 *     macroExpression
 *
 * The canonical top-level parser must import/combine this grammar with the
 * grammar owning those shared rules.
 *
 * ============================================================================
 * DOWNSTREAM CONSUMERS
 * ============================================================================
 *
 * This grammar may be consumed by:
 *
 *     - Zamani parser;
 *     - frontend AST builder;
 *     - syntax diagnostics;
 *     - formatter;
 *     - syntax highlighter;
 *     - language server;
 *     - IDE tooling;
 *     - macro resolver;
 *     - macro expansion engine;
 *     - source-map/provenance system;
 *     - semantic analyzer.
 *
 * None of those consumers should infer hardware properties from this grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include:
 *
 *     foo!()
 *     foo!(x)
 *     foo!(x, y)
 *     foo!(expression)
 *     foo!(nested_call(x))
 *     module::foo!(x)
 *     module::nested::foo!(x)
 *
 * Where supported by canonical argumentList:
 *
 *     foo!(x,)
 *
 * Cross-domain syntax tests should include invocations whose arguments are:
 *
 *     - classical expressions;
 *     - quantum expressions;
 *     - resource expressions;
 *     - hardware-independent expressions;
 *     - distributed expressions;
 *     - AI/data expressions.
 *
 * Negative parser tests MUST include malformed delimiters and separators.
 *
 * Semantic negative tests belong to later compiler tests and MUST include:
 *
 *     - unknown macro;
 *     - inaccessible macro;
 *     - invalid argument count;
 *     - invalid argument type;
 *     - invalid generic arguments;
 *     - invalid capability requirements;
 *     - invalid expansion.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must verify that the grammar imposes no artificial source-level
 * maximum on:
 *
 *     - invocation count;
 *     - argument count;
 *     - qualified-name depth;
 *     - nesting;
 *     - source size.
 *
 * Test infrastructure may use finite test values because tests execute on
 * finite machines. Those finite values MUST NOT become grammar constants.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Parsing the same canonical token stream repeatedly must produce equivalent
 * parse structure.
 *
 * Macro resolution and expansion determinism must be tested separately.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     - maximum invocation count;
 *     - maximum argument count;
 *     - maximum namespace depth;
 *     - maximum nesting depth;
 *     - maximum source size;
 *     - hardware count;
 *     - qubit count;
 *     - device count;
 *     - topology size;
 *     - memory size.
 *
 * Any future finite limit introduced here must be rejected unless it is an
 * actual lexical/syntactic requirement of the language rather than an
 * implementation/resource limit.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when all of the following are true:
 *
 *   1. The grammar is named exactly "invocations".
 *
 *   2. The file is named:
 *
 *          grammar/macros/invocations.g4
 *
 *   3. ZamaniLexer supplies the required tokens.
 *
 *   4. qualifiedName has exactly one canonical owner elsewhere.
 *
 *   5. argumentList has exactly one canonical owner elsewhere.
 *
 *   6. macroInvocation has exactly one canonical owner:
 *
 *          this file.
 *
 *   7. macroExpression has exactly one canonical owner:
 *
 *          this file.
 *
 *   8. macro declarations remain owned by declarations.g4.
 *
 *   9. macro expansion remains outside the parser grammar.
 *
 *  10. Meta.g4 no longer independently defines macroInvocation.
 *
 *  11. macros.g4 no longer independently defines macroInvocation.
 *
 *  12. The canonical parser integrates macroExpression into the expression
 *      hierarchy exactly once.
 *
 *  13. Statement-level macro use is obtained through canonical expression
 *      statement handling rather than duplicate macro statement syntax.
 *
 *  14. No grammar-level machine-size limits exist.
 *
 *  15. No hardware-specific semantics exist here.
 *
 *  16. No quantum-specific hardware assumptions exist here.
 *
 *  17. No filesystem/network/process execution is introduced.
 *
 *  18. Parser diagnostics and semantic diagnostics remain separated.
 *
 *  19. Positive, negative, boundary, cross-domain, compatibility, and
 *      determinism tests exist.
 *
 *  20. The composed ANTLR grammar generates successfully with the repository's
 *      supported ANTLR toolchain.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 *
 * The grammar name intentionally matches:
 *
 *     grammar/macros/invocations.g4
 *
 * ANTLR grammar names and filenames must remain aligned.
 */
parser grammar invocations;


/*
 * ============================================================================
 * LEXER VOCABULARY
 * ============================================================================
 *
 * The lexer is the sole owner of lexical tokens.
 */
options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * MACRO PATH
 * ============================================================================
 *
 * A macro path reuses the canonical qualified-name grammar.
 *
 * Examples:
 *
 *     foo
 *     module::foo
 *     package::module::foo
 *
 * No namespace-depth limit is imposed here.
 */
macroPath
    : qualifiedName
    ;


/*
 * ============================================================================
 * MACRO INVOCATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     name!()
 *
 *     name!(argument)
 *
 *     name!(argument1, argument2)
 *
 * Qualified form:
 *
 *     module::name!(argument)
 *
 * The invocation marker is syntactic and carries no execution semantics.
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
 * This is the single expression-level integration boundary.
 *
 * The canonical expression grammar must include macroExpression in its
 * expression hierarchy.
 *
 * This file deliberately does not redefine expression precedence.
 */
macroExpression
    : macroInvocation
    ;