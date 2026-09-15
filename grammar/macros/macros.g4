/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/macros/macros.g4
* 
* Role:
* Canonical parser component for Zamani macro declarations and
* macro invocation syntax.
* 
* ============================================================================
* ARCHITECTURAL CONTRACT
* ============================================================================
* 
* This grammar owns SOURCE SYNTAX ONLY.
* 
* It owns:
* 
* - macro declarations;
* - macro parameter declarations;
* - macro parameter defaults;
* - macro invocation syntax;
* - qualified macro invocation paths;
* - macro expression syntax;
* - macro argument structure.
* 
* It does NOT own:
* 
* - lexical tokens;
* - AST implementation;
* - macro name resolution;
* - overload resolution;
* - macro expansion;
* - hygiene;
* - token generation;
* - source generation;
* - filesystem access;
* - network access;
* - process execution;
* - compile-time arbitrary code execution;
* - package downloading;
* - target selection;
* - backend selection;
* - CPU/GPU/QPU selection;
* - hardware discovery;
* - quantum gate selection;
* - quantum resource allocation;
* - optimization;
* - scheduling;
* - routing;
* - QEC;
* - ZQN;
* - canonical IR construction.
* 
* ============================================================================
* COMPILER PIPELINE
* ============================================================================
* 
* UTF-8 source
*      |
*      v
* ZamaniLexer.g4
*      |
*      v
* canonical parser
*      |
*      +--> macros/macros.g4
*      |
*      v
* Frontend AST
*      |
*      v
* name / type / effect / capability / resource analysis
*      |
*      v
* macro resolution + controlled expansion
*      |
*      v
* canonical semantic IR
*      |
*      +--> classical IR
*      +--> quantum::ir
*      +--> HDL / hardware representation
*      +--> control / data / effect / resource representations
*      |
*      v
* optimization
*      |
*      v
* routing / scheduling / resilience / target lowering
*      |
*      v
* execution
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Macros must transform portable SOURCE-LEVEL PROGRAM STRUCTURE.
* 
* A macro must not make a program inherently dependent on:
* 
* - a fixed number of qubits;
* - a fixed number of CPUs;
* - a fixed number of GPUs;
* - a fixed number of devices;
* - a fixed machine topology;
* - a fixed quantum topology;
* - a fixed gate set;
* - a fixed register width;
* - a fixed memory capacity;
* - a fixed scheduler;
* - a fixed backend.
* 
* Such requirements belong to semantic/resource/capability analysis.
* 
* ============================================================================
* SCALABILITY
* ============================================================================
* 
* There are intentionally NO grammar-level finite limits on:
* 
* - number of macro declarations;
* - number of macro parameters;
* - number of macro arguments;
* - nesting depth;
* - qualified-name depth;
* - macro body size;
* - source size.
* 
* Compiler implementations MAY impose configurable resource budgets for:
* 
* - source bytes;
* - tokens;
* - AST nodes;
* - expansion depth;
* - expansion steps;
* - generated nodes;
* - memory;
* - compilation time.
* 
* Those are compiler resource/admission policies, not grammar semantics.
* 
* ============================================================================
* SAFETY
* ============================================================================
* 
* The compiler implementation target is:
* 
* Rust 1.97
* Rust 1.97.1
* 
* The compiler MUST use safe Rust.
* 
* This grammar contains no executable host-language code and therefore
* introduces no unsafe requirement.
* 
* A Zamani source-level "unsafe" construct, if supported elsewhere in the
* language, MUST NOT imply Rust "unsafe".
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* The grammar is deterministic with respect to the canonical token stream.
* 
* Macro expansion determinism is NOT a parser concern.
* 
* Expansion must later be controlled by compiler policy and must preserve:
* 
* - deterministic resolution;
* - deterministic expansion ordering;
* - source provenance;
* - diagnostics;
* - hygiene;
* - reproducibility.
* 
* ============================================================================
* ANTLR COMPOSITION
* ============================================================================
* 
* This is a parser grammar.
* 
* It deliberately uses the canonical Zamani lexer vocabulary:
* 
* MACRO
* BANG
* LPAREN
* RPAREN
* COMMA
* COLON
* ASSIGN
* 
* It deliberately reuses canonical parser rules supplied by the composed
* parser, including:
* 
* visibility
* identifier
* qualifiedName
* genericParameters
* typeExpression
* expression
* argumentList
* blockExpression
* 
* Those rules MUST have one canonical owner.
* 
* This file MUST NOT redefine them.
* 
* ============================================================================
* TOKEN / QUOTATION POLICY
* ============================================================================
* 
* Quote/splice/interpolation are intentionally NOT invented in this file.
* 
* The repository's current canonical lexer has "MACRO" and "BANG", but
* quotation/splicing requires a canonical lexical contract before it can be
* safely accepted by a parser-only grammar.
* 
* Consequently:
* 
* quote
* unquote
* splice
* interpolation
* 
* are not represented here using ad-hoc IDENTIFIER matching.
* 
* This prevents:
* 
* identifier-based keyword ambiguity;
* accidental reservation of ordinary identifiers;
* parser/lexer divergence;
* dialect conflicts.
* 
* A future quotation facility must first establish canonical lexer tokens and
* AST/semantic contracts, after which it can be added without changing the
* core macro declaration/invocation contract.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The parser must lower these productions into the repository's canonical
* frontend AST.
* 
* The macro declaration node must retain, directly or indirectly:
* 
* - source span;
* - declaration identity;
* - visibility;
* - macro name;
* - generic parameters;
* - ordered macro parameters;
* - parameter type syntax, when present;
* - default expressions, when present;
* - macro body;
* - provenance.
* 
* A macro invocation must retain:
* 
* - source span;
* - resolved/unresolved macro path;
* - ordered argument NodeIds;
* - invocation provenance.
* 
* The grammar MUST NOT require a second macro-specific AST hierarchy.
* 
* ============================================================================
* DOMAIN INDEPENDENCE
* ============================================================================
* 
* Macro syntax is domain-neutral.
* 
* The same macro mechanism may therefore construct syntax used by:
* 
* - classical computation;
* - quantum computation;
* - hybrid computation;
* - HDL;
* - hardware;
* - distributed computing;
* - AI/ML;
* - networking;
* - future dialects.
* 
* The macro layer does not decide which domain a generated construct belongs
* to. Semantic analysis determines that after expansion.
* 
* ============================================================================
  */

/*

* ============================================================================
* GRAMMAR DECLARATION
* ============================================================================
  */

parser grammar macros;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* 1. MACRO DECLARATION
* ============================================================================
* 
* Canonical form:
* 
* macro name() {
*     ...
* }
* 
* Generic form:
* 
* macro name<T>(value: T) {
*     ...
* }
* 
* Parameters remain syntactic entities. Their semantic category is resolved
* later by the macro subsystem.
  */

macroDeclaration
: visibility?
MACRO
identifier
genericParameters?
LPAREN
macroParameterList?
RPAREN
macroBody
;

/*

* ============================================================================
* 2. MACRO PARAMETER LIST
* ============================================================================
* 
* The trailing comma is accepted intentionally.
* 
* This makes generated source easier to compose while remaining deterministic.
  */

macroParameterList
: macroParameter
(COMMA macroParameter)*
COMMA?
;

/*

* ============================================================================
* 3. MACRO PARAMETER
* ============================================================================
* 
* A macro parameter is deliberately not identical to a runtime function
* parameter.
* 
* Its type, when supplied, is syntax for later semantic validation.
* 
* This allows future macro parameter categories without forcing the parser
* to encode implementation-specific macro semantics.
  */

macroParameter
: identifier
(COLON typeExpression)?
macroParameterDefault?
;

/*

* ============================================================================
* 4. MACRO PARAMETER DEFAULT
* ============================================================================
* 
* Defaults remain ordinary Zamani expressions.
* 
* They are NOT evaluated by the parser.
  */

macroParameterDefault
: ASSIGN expression
;

/*

* ============================================================================
* 5. MACRO BODY
* ============================================================================
* 
* A macro declaration owns a source block.
* 
* Macro expansion is NOT performed here.
* 
* Keeping the body as the canonical block-expression syntax ensures that
* macro bodies participate in the same source/AST/provenance machinery as
* ordinary Zamani blocks.
  */

macroBody
: blockExpression
;

/*

* ============================================================================
* 6. MACRO PATH
* ============================================================================
* 
* Macro names may be qualified to support modules, packages, namespaces and
* dialects without imposing an artificial namespace depth.
* 
* Examples:
* 
* build!(x)
* math::build!(x)
* quantum::circuit::build!(x)
* 
* The parser does not resolve the path.
  */

macroPath
: qualifiedName
;

/*

* ============================================================================
* 7. MACRO INVOCATION
* ============================================================================
* 
* Canonical invocation syntax:
* 
* name!()
* name!(arg)
* name!(arg1, arg2)
* 
* Qualified invocation:
* 
* module::name!(arg)
* 
* The BANG is syntax only.
* 
* It does not mean:
* 
* execute immediately
* evaluate at parse time
* access the host system
* execute arbitrary Rust
* select a compiler backend
* 
* Those meanings are prohibited at this grammar boundary.
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
* 8. MACRO EXPRESSION
* ============================================================================
* 
* This rule provides the integration boundary for expression grammars.
* 
* The canonical expression grammar should include "macroExpression" in its
* primary/postfix expression alternatives.
* 
* It MUST NOT be duplicated inside this file.
  */

macroExpression
: macroInvocation
;

/*

* ============================================================================
* 9. MACRO INVOCATION ARGUMENTS
* ============================================================================
* 
* Macro invocation deliberately reuses the canonical runtime argument-list
* syntax.
* 
* This gives macros access to:
* 
* literals
* identifiers
* calls
* expressions
* quantum expressions
* classical expressions
* hardware-independent resource expressions
* future domain expressions
* 
* without creating a second expression language.
* 
* The macro engine later determines whether an argument is semantically valid
* for the macro's declared parameter contract.
* 
* No fixed argument count is encoded here.
  */

/*

* ============================================================================
* 10. OPTIONAL STATEMENT INTEGRATION BOUNDARY
* ============================================================================
* 
* A macro invocation is primarily an expression.
* 
* Therefore statement-level use is obtained through the canonical expression
* statement rule rather than by defining a second macro statement grammar.
* 
* For example:
* 
* build!(x);
* 
* is parsed through the canonical:
* 
* expression -> macroExpression -> macroInvocation
* 
* path.
* 
* This avoids duplicate syntax and prevents:
* 
* macroStatement
* macroExpression
* macroCall
* 
* from becoming three competing representations of the same construct.
  */

/*

* ============================================================================
* 11. SEMANTIC OWNERSHIP
* ============================================================================
* 
* The following stages own the following responsibilities:
* 
* LEXER
* Recognizes MACRO, BANG and ordinary source tokens.
* 
* PARSER
* Recognizes the structural syntax defined in this file.
* 
* AST
* Represents macro declarations/invocations and source provenance.
* 
* NAME RESOLUTION
* Resolves macroPath.
* 
* TYPE / CONTRACT ANALYSIS
* Validates macro parameters and arguments.
* 
* MACRO RESOLUTION
* Selects the applicable macro definition.
* 
* MACRO EXPANSION
* Expands the macro according to compiler policy.
* 
* HYGIENE / PROVENANCE
* Preserves source identity and prevents accidental capture.
* 
* RESOURCE CONTROL
* Applies configurable expansion/resource budgets.
* 
* SEMANTIC ANALYSIS
* Determines the meaning of generated constructs.
* 
* CANONICAL IR
* Receives semantic results after expansion/analysis.
* 
* QUANTUM IR
* Receives quantum semantics; this file does not create quantum IR.
* 
* HARDWARE / TARGET LAYERS
* Determine actual implementation against available capabilities.

*/

/*

* ============================================================================
* 12. NON-OWNERSHIP GUARANTEES
* ============================================================================
* 
* This grammar MUST NOT:
* 
* - contain fixed qubit counts;
* - contain fixed CPU counts;
* - contain fixed GPU counts;
* - contain fixed device IDs;
* - contain physical addresses;
* - contain topology descriptions;
* - contain gate-set restrictions;
* - contain scheduler policy;
* - contain optimization policy;
* - contain QEC policy;
* - contain ZQN semantics;
* - contain backend selection;
* - contain hardware discovery;
* - perform I/O;
* - perform network access;
* - execute source;
* - expand macros;
* - mutate compiler-global state.

*/

/*

* ============================================================================
* 13. SCALABILITY GUARANTEES
* ============================================================================
* 
* The grammar uses repetition rather than finite enumerations wherever the
* language model is conceptually unbounded.
* 
* In particular:
* 
* macroParameterList
*     -> zero or more parameters
* 
* argumentList
*     -> zero or more arguments
* 
* qualifiedName
*     -> arbitrary semantic namespace depth
* 
* blockExpression
*     -> canonical recursive source structure
* 
* No production contains:
* 
* [0..N]
* 
* or an equivalent artificial language-level capacity.
* 
* Actual implementation limits, if required for denial-of-service protection,
* belong to compiler configuration and must be:
* 
* - explicit;
* - configurable;
* - observable;
* - diagnosable;
* - independent of source semantics.

*/

/*

* ============================================================================
* 14. DETERMINISTIC PARSING CONTRACT
* ============================================================================
* 
* The canonical parser must integrate macroExpression at the same expression
* precedence level as other primary expressions.
* 
* Recommended conceptual order:
* 
* macroExpression
* literal
* identifier/path expression
* grouped expression
* ...
* 
* The exact expression-precedence implementation remains owned by the
* canonical expression grammar.
* 
* This file deliberately does not duplicate expression precedence.
  */

/*

* ============================================================================
* 15. DIAGNOSTIC CONTRACT
* ============================================================================
* 
* Syntax diagnostics are produced by the parser.
* 
* Semantic diagnostics belong to later phases.
* 
* Examples of syntax errors:
* 
* macro foo(
* macro foo(a: )
* foo!(
* foo!(a, )
* 
* depending on the canonical argument-list trailing-comma policy.
* 
* Examples that are NOT parser errors:
* 
* unknown macro name
* wrong macro argument type
* unavailable macro capability
* excessive expansion budget
* recursive expansion policy violation
* generated program exceeding compiler resource budget
* 
* Those belong to semantic/macro-engine diagnostics.
  */

/*

* ============================================================================
* 16. SECURITY CONTRACT
* ============================================================================
* 
* Parsing a macro MUST be side-effect free.
* 
* A macro invocation must not implicitly authorize:
* 
* filesystem reads/writes;
* network access;
* process creation;
* shell execution;
* environment inspection;
* secret access;
* credential access;
* hardware access.
* 
* Any such capability, if eventually supported by the language/toolchain,
* requires an explicit compiler/runtime capability boundary outside this
* grammar.
  */

/*

* ============================================================================
* 17. REPRODUCIBILITY CONTRACT
* ============================================================================
* 
* Macro expansion must eventually be reproducible from explicit inputs:
* 
* source
* macro definition
* compiler/language version
* macro environment
* explicitly declared capabilities
* explicit compilation policy
* 
* This grammar intentionally does not encode hidden environmental inputs.
  */

/*

* ============================================================================
* 18. FUTURE QUOTATION / SPLICE EXTENSION
* ============================================================================
* 
* Quotation and splice syntax are intentionally reserved for a separate
* canonical extension once the lexer and AST contracts are established.
* 
* The future design should use dedicated lexical tokens rather than treating
* words such as:
* 
* quote
* unquote
* splice
* 
* as ordinary identifiers.
* 
* When introduced, the extension must preserve this ownership boundary:
* 
* lexer
*     -> quote/splice tokens
* 
* parser
*     -> quoted syntax structure
* 
* AST
*     -> canonical syntax/meta nodes
* 
* macro engine
*     -> hygienic expansion
* 
* semantic analysis
*     -> meaning
* 
* canonical IR
*     -> computation
* 
* This file must not silently grow a second meta-language.
  */

/*

* ============================================================================
* 19. INTEGRATION CHECKLIST
* ============================================================================
* 
* The canonical parser composition MUST perform the following exactly once:
* 
* 1. Import this grammar.
* 
* 2. Keep "macroDeclaration" owned by this grammar.
* 
* 3. Remove/deprecate duplicate "macroDeclaration" definitions from Core.g4
* once this grammar becomes canonical.
* 
* 4. Add "macroExpression" to the canonical expression-primary/postfix
* alternatives.
* 
* 5. Keep "MACRO" and "BANG" owned by ZamaniLexer.g4.
* 
* 6. Do not add duplicate MACRO/BANG lexer tokens.
* 
* 7. Reuse canonical:
* 
*    visibility
*    identifier
*    qualifiedName
*    genericParameters
*    typeExpression
*    expression
*    argumentList
*    blockExpression
* 
* 8. Map parser contexts into the existing frontend AST.
* 
* 9. Route macro declarations/invocations through the existing macro semantic
* resolution infrastructure.
* 
* 10. Ensure expansion occurs before semantic IR construction whenever the
* macro produces source-level constructs.
* 
* 11. Ensure generated quantum constructs lower through the canonical
* quantum semantic boundary and ultimately `quantum::ir`.
* 
* 12. Do not let macros instantiate hardware-specific quantum representations.
* 
* 13. Ensure macro expansion preserves source provenance.
* 
* 14. Ensure macro expansion is governed by configurable compiler resource
* limits rather than grammar constants.
* 
* 15. Ensure macro expansion cannot bypass capability/resource/security
* validation.

*/

/*

* ============================================================================
* 20. COMPATIBILITY CONTRACT
* ============================================================================
* 
* Existing source form:
* 
* macro name(parameters) { ... }
* 
* remains supported.
* 
* Existing simple invocation form:
* 
* name!(arguments)
* 
* remains supported.
* 
* The canonical macro component therefore expands the existing feature rather
* than replacing it with an incompatible syntax.
* 
* Existing monolithic grammar syntax:
* 
* macroDecl
* 
* should be treated as legacy syntax ownership only. The canonical production
* name is:
* 
* macroDeclaration
* 
* Migration must be structural rather than semantic:
* 
* macroDecl
*     ->
* macroDeclaration
* 
* No user program should need to change merely because grammar ownership has
* moved.
  */

/*

* ============================================================================
* 21. TEST CONTRACT
* ============================================================================
* 
* Required positive tests:
* 
* macro empty() {}
* macro identity(value) {}
* macro identity(value: T) {}
* macro generic<T>(value: T) {}
* macro defaults(value = expression) {}
* macro qualified<T>(value: T) {}
* 
* Required invocation tests:
* 
* build!()
* build!(x)
* build!(x, y)
* namespace::build!(x)
* namespace::nested::build!(x)
* 
* Required integration tests:
* 
* macro-generated classical expression
* macro-generated function
* macro-generated type
* macro-generated quantum operation
* macro-generated hybrid computation
* macro-generated HDL structure
* macro-generated hardware-independent resource declaration
* 
* Required negative tests:
* 
* macro
* macro 123() {}
* macro name( {}
* macro name(value: ) {}
* name!(
* name!(,)
* 
* Required scalability tests:
* 
* many macro parameters;
* many invocation arguments;
* deeply qualified macro paths;
* deeply nested macro-containing expressions;
* large macro bodies;
* large source programs.
* 
* Resource stress limits must be supplied by compiler tests rather than
* encoded as grammar constants.
* 
* Required determinism tests:
* 
* identical source -> identical token structure
* identical source -> identical parse tree
* 
* Required provenance tests:
* 
* macro declaration span retained;
* invocation span retained;
* generated-node provenance retained by later compiler stages.

*/

/*

* ============================================================================
* 22. COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when:
* 
* [ ] It compiles as an ANTLR parser grammar against ZamaniLexer.
* 
* [ ] It has exactly one owner for every rule referenced from the canonical
* parser.
* 
* [ ] It introduces no duplicate lexer tokens.
* 
* [ ] It introduces no Rust code.
* 
* [ ] It requires no unsafe Rust.
* 
* [ ] It imposes no language-level machine/resource limits.
* 
* [ ] Macro declaration syntax is complete.
* 
* [ ] Macro parameter syntax is complete.
* 
* [ ] Macro defaults are represented.
* 
* [ ] Qualified macro paths are supported.
* 
* [ ] Macro invocation syntax is complete.
* 
* [ ] Macro invocations can participate in expressions.
* 
* [ ] Statement-level macro calls work through the canonical expression
* statement mechanism.
* 
* [ ] The existing AST can represent the resulting syntax.
* 
* [ ] Source spans/provenance are preserved by the frontend.
* 
* [ ] Macro expansion remains outside the grammar.
* 
* [ ] Macro expansion cannot bypass capability/resource/security analysis.
* 
* [ ] Quantum syntax generated by macros continues through the canonical
* semantic quantum boundary and `quantum::ir`.
* 
* [ ] Classical, quantum, HDL and hardware syntax can all be macro-generated
* without changing this grammar.
* 
* [ ] No hardware size, topology, backend, device or gate-set assumption is
* encoded here.
* 
* [ ] Positive, negative, boundary, cross-domain and deterministic tests pass.
* 
* ============================================================================
  */