/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/wisdom.g4
 *
 * Grammar:
 *     Wisdom
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Status:
 *     CANONICAL WISDOM / SANKOFA MEMORY-DOMAIN LEAF GRAMMAR
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     2021
 *
 * Safety:
 *     Safe Rust only.
 *
 *     This grammar contains no embedded Rust actions and requires no unsafe
 *     Rust implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the canonical source syntax for Zamani's `wisdom`
 * construct.
 *
 * Wisdom is a Sankofa knowledge-domain construct. It provides source-level
 * intent for declaring or recording a named higher-level knowledge/wisdom
 * value.
 *
 * This grammar defines syntax only.
 *
 * It does NOT implement:
 *
 *     knowledge storage
 *     memory persistence
 *     learning
 *     inference
 *     reasoning
 *     confidence calculation
 *     trust calculation
 *     provenance verification
 *     consensus
 *     temporal execution
 *     AI models
 *     databases
 *     caches
 *     indexing
 *     replication
 *     scheduling
 *     routing
 *     hardware selection
 *     resource discovery
 *     runtime execution
 *
 * Those responsibilities belong to the appropriate semantic, compiler,
 * runtime, resource, capability, and target layers.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Wisdom is a domain feature of Zamani, not a separate language.
 *
 * The complete pipeline is:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     runtime / target realization
 *
 * This grammar MUST NOT introduce a second Sankofa AST or a second universal
 * IR.
 *
 * ============================================================================
 * FILE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     wisdomStatement
 *
 * THIS FILE MAY PROVIDE:
 *
 *     wisdomConstruct
 *
 * as a small public composition alias.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     attributes
 *     metadata
 *     provenance
 *     temporal semantics
 *     memory allocation
 *     memory ownership
 *     borrowing
 *     persistence
 *     history
 *     recall
 *     learning
 *     inference
 *     consensus
 *     resource requirements
 *     capability definitions
 *     hardware
 *     quantum computation
 *     HDL
 *     distributed execution
 *     runtime behavior
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * The authoritative relationships are:
 *
 *     grammar/DESIGN.md
 *             |
 *             v
 *     grammar/specification/syntax.md
 *             |
 *             v
 *     grammar/memory/wisdom.g4
 *             |
 *             v
 *     grammar/memory/sankofa.g4
 *             |
 *             v
 *     canonical parser composition
 *             |
 *             v
 *     src/ast/
 *             |
 *             v
 *     semantic analysis
 *             |
 *             v
 *     canonical IR
 *
 * `grammar/Zamani-Grammar.md` is not an independent syntax authority.
 *
 * `grammar/grammar.md` reports implementation conformance and MUST distinguish
 * specified syntax from syntax actually implemented by the Rust frontend.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It MUST consume the canonical Zamani lexer vocabulary:
 *
 *     WISDOM
 *     ASSIGN
 *     SEMICOLON
 *
 * and the canonical parser rules:
 *
 *     identifier
 *     expression
 *
 * It MUST NOT define lexer rules.
 *
 * It MUST NOT invent a second WISDOM token.
 *
 * The canonical lexical spelling is:
 *
 *     WISDOM : 'wisdom'
 *
 * ============================================================================
 * UNIVERSAL DEPENDENCIES
 * ============================================================================
 *
 * Names and Expressions are imported because this grammar is intentionally
 * independently completable.
 *
 * The grammar therefore does not depend on a later edit to Sankofa merely to
 * obtain the definitions of identifier or expression.
 *
 * ============================================================================
 * SOURCE SYNTAX CONTRACT
 * ============================================================================
 *
 * The canonical source form is:
 *
 *     wisdom identifier;
 *
 * or:
 *
 *     wisdom identifier = expression;
 *
 * The semicolon is optional for compatibility with the existing canonical
 * syntax specification and Rust frontend.
 *
 * Therefore all of the following are syntactically valid:
 *
 *     wisdom knowledge;
 *     wisdom knowledge = value;
 *     wisdom knowledge = value;
 *
 * The preferred formatted form remains:
 *
 *     wisdom knowledge;
 *     wisdom knowledge = value;
 *
 * No additional syntax is invented here.
 *
 * ============================================================================
 * IMPORTANT: NO EXPRESSION-ONLY WISDOM FORM
 * ============================================================================
 *
 * Wisdom is currently specified as a statement/declaration-like construct.
 *
 * Therefore this grammar intentionally does NOT define:
 *
 *     wisdom expression
 *
 * as a separate expression form.
 *
 * This prevents accidental ambiguity with ordinary expressions and preserves
 * the existing language contract.
 *
 * ============================================================================
 * NAME CONTRACT
 * ============================================================================
 *
 * The wisdom name is an ordinary canonical identifier.
 *
 * This grammar does not create a separate wisdom-name namespace.
 *
 * Name validity, reserved-word checking, scope, redeclaration, visibility,
 * ownership, and symbol resolution remain semantic/compiler responsibilities.
 *
 * The grammar MUST NOT encode:
 *
 *     a fixed number of wisdom values;
 *     a fixed name length;
 *     a fixed namespace depth;
 *     a fixed number of namespaces;
 *     a fixed knowledge capacity.
 *
 * ============================================================================
 * VALUE CONTRACT
 * ============================================================================
 *
 * The optional wisdom value is the canonical Zamani `expression`.
 *
 * This means wisdom can carry any expression already supported by the language
 * without creating a second Sankofa expression language.
 *
 * Examples include ordinary values, computed values, references, calls,
 * collections, temporal values, symbolic values, and future expression forms
 * admitted by the canonical expression grammar.
 *
 * This grammar does not decide what an expression means.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The existing Rust frontend contains:
 *
 *     Statement::Wisdom(Span, String, Expression)
 *
 * The existing parser represents:
 *
 *     wisdom name;
 *
 * by supplying:
 *
 *     Expression::Literal(Literal::Null(span))
 *
 * as the value.
 *
 * This grammar therefore preserves the existing compatibility contract:
 *
 *     wisdomStatement
 *          |
 *          +--> identifier
 *          +--> optional expression
 *          |
 *          v
 *     Statement::Wisdom(...)
 *
 * The grammar itself does not construct AST objects.
 *
 * Future AST evolution MAY replace the null-sentinel representation with an
 * explicit optional value, but such a change belongs to the AST/semantic
 * compatibility process and must not require a grammar redesign.
 *
 * Required semantic information to preserve:
 *
 *     source span
 *     wisdom name
 *     optional source value
 *
 * The semantic representation may additionally attach:
 *
 *     provenance
 *     temporal context
 *     confidence
 *     trust
 *     evidence
 *     policy
 *     resource requirements
 *     capability requirements
 *     metadata
 *
 * when those facilities are explicitly provided by other language constructs.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * A parsed wisdom statement expresses an intent to establish or record a
 * named knowledge/wisdom value.
 *
 * Semantic analysis is responsible for determining:
 *
 *     whether the name is valid;
 *     whether the name may be declared in the current scope;
 *     whether a previous binding conflicts;
 *     whether the value has a valid type;
 *     whether the value satisfies relevant contracts;
 *     whether provenance is valid;
 *     whether temporal relationships are valid;
 *     whether required capabilities exist;
 *     whether required resources can be satisfied;
 *     whether the construct is permitted by the active language/dialect
 *     version.
 *
 * None of those checks belong in this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Wisdom MUST lower through the repository's canonical semantic/IR pipeline.
 *
 * The grammar MUST NOT define:
 *
 *     WisdomIR
 *     SankofaIR
 *     KnowledgeIR
 *     MemoryWisdomIR
 *
 * as competing universal representations.
 *
 * Conceptually:
 *
 *     wisdomStatement
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic knowledge/wisdom model
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     canonical IR
 *
 * Any backend-specific realization is downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Wisdom may ultimately require resources or capabilities such as:
 *
 *     memory persistence
 *     knowledge storage
 *     inference
 *     learning
 *     provenance
 *     distributed communication
 *     temporal processing
 *
 * Those requirements MUST be represented through the canonical resource and
 * capability system.
 *
 * This grammar MUST NOT hard-code target resources.
 *
 * Valid semantic intent may include concepts such as:
 *
 *     requires capability("memory.persistence")
 *     requires capability("knowledge.reasoning")
 *     requires memory >= required_memory
 *
 * The exact resource/capability syntax is owned by the corresponding
 * resource/capability grammar.
 *
 * Wisdom does not duplicate it.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Wisdom may participate in provenance and lineage systems.
 *
 * This grammar does not define provenance syntax itself.
 *
 * Provenance belongs to:
 *
 *     grammar/memory/provenance.g4
 *
 * and its canonical composition.
 *
 * The semantic layer determines whether provenance is present, valid,
 * trustworthy, complete, or verifiable.
 *
 * A parsed wisdom declaration MUST NOT be treated as proof that its contents
 * are true.
 *
 * ============================================================================
 * TEMPORAL CONTRACT
 * ============================================================================
 *
 * Wisdom may be associated semantically with:
 *
 *     zamani
 *     sasa
 *     MTS
 *     history
 *     temporal state
 *     lineage
 *
 * This file does not redefine those constructs.
 *
 * In particular, this grammar does not interpret temporal values as:
 *
 *     CPU cycles
 *     wall-clock timestamps
 *     scheduler ticks
 *     fixed-width hardware counters
 *
 * Temporal meaning belongs to the canonical temporal specification.
 *
 * ============================================================================
 * LEARNING / INFERENCE SEPARATION
 * ============================================================================
 *
 * Wisdom is related to learning and inference but does not own them.
 *
 * `learn` expresses learning intent.
 *
 * `infer` expresses inference/reasoning intent.
 *
 * `wisdom` expresses a named knowledge/wisdom value.
 *
 * This grammar MUST NOT silently convert:
 *
 *     wisdom
 *
 * into:
 *
 *     learn
 *     infer
 *     recall
 *
 * nor may it implement any of those operations.
 *
 * ============================================================================
 * QUANTUM / CLASSICAL / HDL INTEGRATION
 * ============================================================================
 *
 * The value expression may ultimately contain computations from any Zamani
 * domain supported by the canonical expression system.
 *
 * Therefore wisdom remains compatible with:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL/co-design
 *     AI/ML
 *     distributed computation
 *     networking
 *     data/tensor computation
 *     temporal computation
 *     future domains
 *
 * This file does not need separate rules for each domain.
 *
 * Domain semantics remain downstream.
 *
 * Quantum constructs ultimately use the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * Wisdom does not create a quantum-specific IR.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Wisdom syntax is target-independent.
 *
 * A program containing wisdom may be compiled for:
 *
 *     tiny embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     accelerators
 *     FPGAs
 *     ASICs
 *     QPUs
 *     quantum simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational substrates
 *
 * subject to the actual semantic requirements and resources available to the
 * compiler/runtime.
 *
 * This grammar does not select any target.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is deliberately NO grammar-level maximum for:
 *
 *     number of wisdom declarations
 *     number of knowledge values
 *     number of expressions
 *     expression size
 *     identifier count
 *     namespace count
 *     namespace depth
 *     provenance depth
 *     history size
 *     timeline count
 *     branch count
 *     memory capacity
 *     storage capacity
 *     distributed node count
 *     processor count
 *     accelerator count
 *     device count
 *
 * The grammar MUST NOT introduce:
 *
 *     MAX_WISDOM
 *     MAX_KNOWLEDGE
 *     MAX_WISDOM_ITEMS
 *     MAX_HISTORY
 *     MAX_TIMELINES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *
 * or equivalent hidden ceilings.
 *
 * Numeric literals in the value expression remain ordinary program semantics.
 *
 * A program value such as:
 *
 *     wisdom threshold = 1024;
 *
 * does not establish a language-level limit of 1024.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is a pure function of:
 *
 *     source text
 *     canonical lexer vocabulary
 *     canonical parser grammar
 *     selected grammar/language version
 *     explicitly selected dialect configuration
 *
 * Identical source under identical grammar configuration MUST produce the
 * same parse structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware
 *     memory availability
 *     runtime state
 *     network state
 *     wall-clock time
 *     randomness
 *     environment variables
 *     filesystem state
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * The grammar should allow ANTLR to identify structural syntax errors at the
 * smallest practical source span.
 *
 * Structural errors include:
 *
 *     missing wisdom name
 *     missing expression after ASSIGN
 *     malformed expression
 *     malformed token sequence
 *
 * Examples that MUST be rejected structurally:
 *
 *     wisdom;
 *     wisdom = value;
 *     wisdom name = ;
 *     wisdom name == value;
 *
 * Examples that are syntactically valid but may fail semantic validation:
 *
 *     wisdom existing_name;
 *     wisdom existing_name = unknown_value;
 *
 * The latter questions belong to name/type/semantic analysis.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing accepted forms MUST remain representable:
 *
 *     wisdom name;
 *     wisdom name = expression;
 *     wisdom name = expression
 *
 * The grammar therefore intentionally keeps the optional semicolon behavior
 * already documented by the canonical syntax specification and implemented by
 * the existing Rust parser.
 *
 * Tightening the semicolon requirement would be a language compatibility
 * change and must not be performed silently in this leaf grammar.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar is intended to be imported by:
 *
 *     grammar/memory/sankofa.g4
 *
 * Sankofa MUST delegate its wisdom production to this grammar instead of
 * duplicating the same WISDOM syntax.
 *
 * Recommended composition:
 *
 *     parser grammar Sankofa;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 *     import Names, Expressions, Wisdom;
 *
 *     sankofaWisdom
 *         : wisdomStatement
 *         ;
 *
 * The existing Sankofa public rule:
 *
 *     sankofaStatement
 *         : ...
 *         | sankofaWisdom
 *         ;
 *
 * can therefore remain stable.
 *
 * This preserves the existing rule name used by the Sankofa grammar while
 * moving actual ownership of wisdom syntax into this independent file.
 *
 * ============================================================================
 * SANKOFA INTEGRATION
 * ============================================================================
 *
 * `grammar/memory/sankofa.g4` currently defines:
 *
 *     sankofaWisdom
 *         : WISDOM
 *           identifier
 *           (ASSIGN expression)?
 *           SEMICOLON?
 *         ;
 *
 * That duplicate syntax MUST be replaced with:
 *
 *     sankofaWisdom
 *         : wisdomStatement
 *         ;
 *
 * The following Sankofa rules MUST NOT be duplicated inside this file:
 *
 *     sankofaRecall
 *     sankofaLearn
 *     sankofaInfer
 *     sankofaFromClause
 *     sankofaWithClause
 *     sankofaTemporalQualifier
 *
 * They remain owned by their respective grammar components.
 *
 * ============================================================================
 * CANONICAL LEXICAL INTEGRATION
 * ============================================================================
 *
 * The WISDOM token already exists in:
 *
 *     grammar/lexer/keywords.g4
 *
 * Therefore no lexer change is required merely to add this parser grammar.
 *
 * The token flow remains:
 *
 *     ZamaniKeywords
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     Wisdom parser grammar
 *
 * ============================================================================
 * ROOT COMPOSITION INTEGRATION
 * ============================================================================
 *
 * `grammar/Zamani.g4` MUST NOT import this file directly merely because this
 * file exists.
 *
 * The root composition should remain responsible only for the top-level
 * language composition.
 *
 * Wisdom should enter the root through the existing canonical parser/memory/
 * Sankofa composition path.
 *
 * This avoids creating a second direct ownership path.
 *
 * ============================================================================
 * AST / FRONTEND INTEGRATION
 * ============================================================================
 *
 * Existing frontend compatibility:
 *
 *     src/ast/mod.rs
 *         Statement::Wisdom(Span, String, Expression)
 *
 * Existing parser compatibility:
 *
 *     src/parser.rs
 *         parse_wisdom()
 *
 * Existing semantic compatibility:
 *
 *     src/semantic.rs
 *         Statement::Wisdom(...)
 *
 * Existing compiler compatibility includes:
 *
 *     src/compiler/monomorphizer.rs
 *
 * The grammar therefore deliberately preserves the current source shape.
 *
 * The grammar file itself MUST NOT require changes to those Rust files merely
 * to represent the existing wisdom syntax.
 *
 * However, when the ANTLR frontend becomes authoritative for this construct,
 * its AST adapter MUST map the same source form to the existing domain-neutral
 * AST representation before any future AST redesign.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This grammar is complete only when conformance tests cover at least:
 *
 * POSITIVE
 *
 *     wisdom name;
 *     wisdom name
 *     wisdom name = value;
 *     wisdom name = value
 *     wisdom name = expression + expression;
 *     wisdom name = function_call(argument);
 *     wisdom name = nested_expression;
 *     wisdom name = symbolic_value;
 *
 * NEGATIVE
 *
 *     wisdom;
 *     wisdom = value;
 *     wisdom name = ;
 *     wisdom name == value;
 *     wisdom name = ,;
 *
 * BOUNDARY
 *
 *     minimal identifier
 *     longest implementation-supported identifier
 *     deeply nested expression
 *     large expression
 *     large source unit
 *     optional semicolon
 *
 * SCALABILITY
 *
 *     one wisdom value
 *     many wisdom values
 *     symbolic values
 *     large expressions
 *     many independent declarations
 *
 * DETERMINISM
 *
 *     identical input produces identical parse structure
 *     repeated parsing does not depend on target resources
 *
 * CROSS-DOMAIN
 *
 *     wisdom value containing classical computation
 *     wisdom value containing supported quantum computation
 *     wisdom value containing hybrid computation
 *     wisdom value containing supported data/tensor computation
 *
 * No test may establish a universal artificial capacity limit.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no physical resource constants and no universal capacity
 * constants.
 *
 * Validation MUST flag future modifications introducing concepts such as:
 *
 *     MAX_WISDOM
 *     MAX_KNOWLEDGE
 *     MAX_MEMORY
 *     MAX_HISTORY
 *     MAX_TIMELINES
 *     MAX_DEVICES
 *     MAX_NODES
 *     fixed register widths
 *     fixed memory capacities
 *     fixed processor counts
 *
 * A numeric literal occurring inside `expression` is not itself a violation.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * ANTLR generation and the consuming frontend MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021 edition
 *
 * The generated/handwritten Rust implementation MUST use safe Rust only.
 *
 * This grammar contains no Rust actions and introduces no `unsafe` requirement.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete as an independent grammar contract when:
 *
 * [x] Existing wisdom syntax is represented.
 * [x] Canonical WISDOM token is reused.
 * [x] Canonical identifier rule is reused.
 * [x] Canonical expression rule is reused.
 * [x] No lexer rules are duplicated.
 * [x] No second AST is introduced.
 * [x] No second IR is introduced.
 * [x] No runtime behavior is introduced.
 * [x] No learning implementation is introduced.
 * [x] No inference implementation is introduced.
 * [x] No provenance implementation is introduced.
 * [x] No temporal implementation is introduced.
 * [x] No hardware dependency is introduced.
 * [x] No resource ceiling is introduced.
 * [x] No target-specific syntax is introduced.
 * [x] Determinism is preserved.
 * [x] Existing optional-semicolon compatibility is preserved.
 * [x] Sankofa integration is explicitly defined.
 * [x] AST integration is explicitly defined.
 * [x] Semantic integration is explicitly defined.
 * [x] IR integration is explicitly defined.
 * [x] Compiler/runtime boundaries are explicit.
 * [x] Test obligations are explicit.
 * [x] Rust 1.97/1.97.1 compatibility is explicit.
 * [x] Safe-Rust requirement is explicit.
 *
 * ============================================================================
 * CANONICAL GRAMMAR
 * ============================================================================
 */

parser grammar Wisdom;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/* ============================================================================
 * PUBLIC COMPOSITION ENTRY
 * ========================================================================== */

wisdomConstruct
    : wisdomStatement
    ;


/* ============================================================================
 * WISDOM STATEMENT
 *
 * Canonical forms:
 *
 *     wisdom name;
 *     wisdom name = expression;
 *
 * Semicolon remains optional for compatibility with the existing language
 * specification and Rust frontend.
 * ========================================================================== */

wisdomStatement
    : WISDOM
      identifier
      (
          ASSIGN
          expression
      )?
      SEMICOLON?
    ;