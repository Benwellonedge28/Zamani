/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/metaprogramming/unquotation.g4
 *
 * Grammar:
 *     Unquotation / Splicing
 *
 * Status:
 *     Production parser-grammar component
 *
 * Compiler baseline:
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the INDEPENDENT SYNTACTIC CONTRACT for unquotation.
 *
 * Unquotation is the controlled boundary through which an ordinary Zamani
 * expression supplies a semantic value to an enclosing quotation context.
 *
 * Conceptually:
 *
 *     quote {
 *         ...
 *         unquote expression
 *         ...
 *     }
 *
 * means that `expression` is evaluated/validated according to the
 * metaprogramming phase rules and its resulting meta-value is incorporated
 * into the surrounding quoted source structure.
 *
 * IMPORTANT:
 *
 * This file defines SOURCE SYNTAX ONLY.
 *
 * It does not:
 *
 *     - evaluate expressions;
 *     - execute compiler code;
 *     - expand macros;
 *     - perform hygiene;
 *     - construct the canonical AST;
 *     - construct an IR;
 *     - construct quantum::ir;
 *     - perform reflection;
 *     - perform introspection;
 *     - perform specialization;
 *     - access files;
 *     - access networks;
 *     - access hardware;
 *     - access devices;
 *     - access credentials;
 *     - select targets;
 *     - allocate resources;
 *     - route operations;
 *     - schedule operations;
 *     - perform QEC;
 *     - perform ZQN processing;
 *     - invoke the HAL;
 *     - execute runtime code.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
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
 *     frontend AST
 *          |
 *          +---------------------------+
 *          |                           |
 *          v                           v
 *     ordinary semantics       metaprogramming semantics
 *                                      |
 *                                      v
 *                                  unquotation
 *                                      |
 *                                      v
 *                              quotation construction
 *                                      |
 *                                      v
 *                              canonical AST/source
 *                                      |
 *                                      v
 *                              ordinary validation
 *                                      |
 *                                      v
 *                           canonical semantic model
 *                                      |
 *                                      v
 *                               canonical IR
 *                                      |
 *                   +------------------+------------------+
 *                   |                  |                  |
 *                   v                  v                  v
 *              classical          quantum::ir       HDL/hardware
 *                   |                  |                  |
 *                   +------------------+------------------+
 *                                      |
 *                                      v
 *                              optimization/lowering
 *                                      |
 *                              routing/scheduling
 *                                      |
 *                              resilience/QEC/ZQN
 *                                      |
 *                                      v
 *                                     HAL
 *                                      |
 *                                      v
 *                              target realization
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Normative architectural authority:
 *
 *     grammar/DESIGN.md
 *
 * Metaprogramming composition authority:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * Quotation/unquotation syntax authority:
 *
 *     grammar/metaprogramming/quotation.g4
 *     grammar/metaprogramming/unquotation.g4
 *
 * Canonical syntax specification:
 *
 *     grammar/spec/syntax.md
 *
 * Metaprogramming documentation:
 *
 *     grammar/metaprogramming/README.md
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Frontend implementation:
 *
 *     src/lexer.rs
 *     src/parser.rs
 *     src/frontend/ast/
 *
 * Semantic implementation:
 *
 *     repository semantic-analysis subsystem
 *
 * Canonical IR:
 *
 *     repository canonical IR subsystem
 *
 * Quantum boundary:
 *
 *     quantum::ir
 *
 * This file MUST remain subordinate to those authorities.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the independent unquotation integration boundary;
 *     - the unquotation expression wrapper;
 *     - the unquotation/splicing semantic-category boundary;
 *     - explicit delegation to the canonical expression grammar;
 *     - the parser-visible distinction between ordinary expression syntax and
 *       an unquotation construct.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - the QUOTE token;
 *     - the UNQUOTE token;
 *     - identifier syntax;
 *     - path syntax;
 *     - expression syntax;
 *     - block syntax;
 *     - type syntax;
 *     - statement syntax;
 *     - declaration syntax;
 *     - quotation syntax as a whole;
 *     - macro declaration syntax;
 *     - macro invocation syntax;
 *     - macro expansion;
 *     - macro hygiene;
 *     - source provenance implementation;
 *     - reflection;
 *     - introspection;
 *     - specialization;
 *     - compile-time execution;
 *     - source generation;
 *     - AST implementation;
 *     - semantic analysis;
 *     - type checking;
 *     - effect checking;
 *     - capability checking;
 *     - resource checking;
 *     - canonical IR;
 *     - quantum::ir;
 *     - classical IR;
 *     - HDL/hardware representation;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * NON-DUPLICATION RULE
 * ============================================================================
 *
 * The repository already contains:
 *
 *     grammar/metaprogramming/quotation.g4
 *
 * That file historically contains the canonical core productions:
 *
 *     quoteExpressionCore
 *     unquoteExpressionCore
 *     metaQuoteCore
 *     metaSpliceCore
 *
 * This file MUST NOT redefine those symbols.
 *
 * Instead, this file supplies the independent named unquotation component
 * boundary:
 *
 *     unquotationExpression
 *
 * and:
 *
 *     unquotationSplice
 *
 * These rules delegate to the canonical quotation owner where necessary.
 *
 * This permits the repository to retain quotation.g4 without introducing
 * duplicate ANTLR rule ownership.
 *
 * If the composition architecture is subsequently migrated so that
 * unquotation.g4 becomes the sole owner of `unquoteExpressionCore`, that
 * migration must be performed as an explicit repository-wide grammar
 * refactoring with conformance tests. It MUST NOT be silently performed by
 * this file.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It MUST NOT define lexer rules.
 *
 * The canonical lexer owns:
 *
 *     UNQUOTE
 *
 * with the spelling:
 *
 *     unquote
 *
 * The token MUST come from the canonical Zamani lexer vocabulary.
 *
 * This file therefore does NOT contain:
 *
 *     UNQUOTE : 'unquote' ;
 *
 * The token is consumed through:
 *
 *     tokenVocab = ZamaniLexer
 *
 * ============================================================================
 * RESERVED-WORD CONTRACT
 * ============================================================================
 *
 * `unquote` is a language-level metaprogramming keyword.
 *
 * It is not redefined as an identifier by this grammar.
 *
 * The lexer is responsible for recognizing the canonical token.
 *
 * Name resolution remains downstream.
 *
 * ============================================================================
 * CANONICAL EXPRESSION CONTRACT
 * ============================================================================
 *
 * The expression following `unquote` MUST be the canonical Zamani expression.
 *
 * This file MUST NOT introduce:
 *
 *     unquoteExpression
 *     unquoteArithmeticExpression
 *     unquoteMetaExpression
 *     unquoteValueExpression
 *     unquoteSourceExpression
 *
 * as alternative miniature expression systems.
 *
 * The canonical expression rule remains authoritative.
 *
 * Conceptually:
 *
 *     UNQUOTE expression
 *
 * where `expression` is supplied by the canonical parser composition layer.
 *
 * ============================================================================
 * WHY THE EXPRESSION IS CANONICAL
 * ============================================================================
 *
 * Unquotation must be able to consume any ordinary Zamani expression that is
 * legal in the enclosing phase.
 *
 * Consequently, it automatically remains compatible with:
 *
 *     classical expressions
 *     quantum expressions
 *     hybrid expressions
 *     HDL expressions
 *     hardware-intent expressions
 *     tensor expressions
 *     AI expressions
 *     distributed expressions
 *     networking expressions
 *     security expressions
 *     dialect expressions
 *     future expression domains
 *
 * without this file having to be modified for every new domain.
 *
 * This is a core POCO-REAF property.
 *
 * ============================================================================
 * PRIMARY SYNTAX
 * ============================================================================
 *
 * Normative source form:
 *
 *     unquote expression
 *
 * The expression itself belongs to the canonical expression grammar.
 *
 * The parser therefore recognizes the boundary while semantic analysis
 * determines whether the boundary is legal in the current phase.
 *
 * ============================================================================
 * INDEPENDENT COMPONENT RULE
 * ============================================================================
 *
 * The uniquely owned public component rule is:
 *
 *     unquotationExpression
 *
 * It is intentionally an integration wrapper rather than another copy of
 * the unquotation implementation.
 *
 * Definition:
 *
 *     unquotationExpression
 *         : unquoteExpressionCore
 *         ;
 *
 * `unquoteExpressionCore` remains the canonical core supplied by
 * quotation.g4 under the current repository architecture.
 *
 * ============================================================================
 * SPLICE INTEGRATION
 * ============================================================================
 *
 * The repository currently uses:
 *
 *     metaSpliceCore
 *
 * as a metaprogramming composition category.
 *
 * The existing quotation contract defines that category as an integration
 * alias for canonical unquotation.
 *
 * Therefore this file exposes:
 *
 *     unquotationSplice
 *
 * as a uniquely named component boundary:
 *
 *     unquotationSplice
 *         : metaSpliceCore
 *         ;
 *
 * This does NOT introduce another keyword.
 *
 * It does NOT introduce `splice`.
 *
 * It does NOT introduce `quasiquote`.
 *
 * The only source spelling remains:
 *
 *     unquote
 *
 * ============================================================================
 * WHY THE WRAPPERS EXIST
 * ============================================================================
 *
 * The wrappers provide explicit ownership boundaries for grammar composition.
 *
 * They allow:
 *
 *     metaprogramming.g4
 *             |
 *             +--> unquotationExpression
 *             |
 *             +--> unquotationSplice
 *
 * without requiring the composition grammar to know which historical
 * component currently owns the canonical core.
 *
 * This also gives migration tooling a stable integration point.
 *
 * ============================================================================
 * PHASE SEMANTICS
 * ============================================================================
 *
 * The grammar does NOT enforce:
 *
 *     unquote is only legal inside quote
 *
 * because that is a contextual semantic rule.
 *
 * The semantic analyzer MUST track a quotation/metaprogramming phase context.
 *
 * At minimum, semantic analysis must distinguish:
 *
 *     ordinary source phase
 *     quotation construction phase
 *     unquotation evaluation phase
 *     generated-source validation phase
 *
 * An `unquote` occurring outside a valid quotation context MUST produce a
 * structured semantic diagnostic.
 *
 * It MUST NOT be accepted merely because the parser recognizes the syntax.
 *
 * ============================================================================
 * NESTING
 * ============================================================================
 *
 * Nested quotation and unquotation are legal at the syntactic level.
 *
 * For example, conceptually:
 *
 *     quote {
 *         unquote some_expression
 *     }
 *
 * and nested quotation structures may occur wherever the canonical expression
 * grammar permits them.
 *
 * The grammar imposes no artificial nesting limit.
 *
 * Semantic analysis and compiler resource management may impose operational
 * budgets, but those budgets are NOT language-level grammar limits.
 *
 * ============================================================================
 * RECURSION AND SCALABILITY
 * ============================================================================
 *
 * This grammar MUST NOT contain constants such as:
 *
 *     MAX_UNQUOTE_DEPTH
 *     MAX_QUOTATION_DEPTH
 *     MAX_UNQUOTED_EXPRESSIONS
 *     MAX_SPLICES
 *     MAX_GENERATED_NODES
 *     MAX_META_VALUES
 *     MAX_META_EXPRESSIONS
 *     MAX_SOURCE_SIZE
 *
 * The grammar imposes no finite language-level capacity on:
 *
 *     - quotation nesting;
 *     - unquotation nesting;
 *     - number of unquotations;
 *     - expression size;
 *     - quotation size;
 *     - generated source size;
 *     - number of generated constructs.
 *
 * Actual implementation resource limits are compiler policy.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Unquotation MUST preserve target-independent source meaning.
 *
 * It MUST NOT encode:
 *
 *     CPU count
 *     core count
 *     thread count
 *     GPU count
 *     FPGA count
 *     ASIC count
 *     QPU count
 *     qubit count
 *     memory capacity
 *     register width
 *     tensor rank
 *     network size
 *     node count
 *     accelerator count
 *     device identifiers
 *     vendor identifiers
 *     physical topology
 *
 * as grammar-level limitations.
 *
 * A generated expression may itself contain legitimate program-level
 * resource requirements.
 *
 * For example, a program may semantically construct:
 *
 *     requires qubits >= n
 *
 * or:
 *
 *     requires capability("quantum.measurement")
 *
 * but this file does not decide how such requirements are realized.
 *
 * ============================================================================
 * RESOURCE SEPARATION
 * ============================================================================
 *
 * Unquotation MUST NOT confuse:
 *
 *     source structure
 *
 * with:
 *
 *     target resource realization.
 *
 * Generated source containing resource requirements must pass through the
 * normal resource/capability analysis pipeline.
 *
 * A valid generated program can therefore be rejected downstream because:
 *
 *     - a capability is unavailable;
 *     - a resource requirement cannot be satisfied;
 *     - a target does not support an operation;
 *     - a compilation budget is exhausted;
 *     - a runtime resource is unavailable.
 *
 * Those are NOT syntax errors.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this construct is deterministic.
 *
 * Semantic determinism belongs to the compiler's metaprogramming model.
 *
 * Unquotation MUST NOT implicitly grant access to:
 *
 *     filesystem
 *     network
 *     environment
 *     process execution
 *     credentials
 *     hardware
 *     devices
 *     wall-clock time
 *     nondeterministic external state
 *     unrestricted randomness
 *
 * Such capabilities, if supported elsewhere by Zamani, must be explicitly
 * represented and authorized by the semantic/effect/capability system.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing an unquotation expression MUST NEVER execute the expression.
 *
 * Parsing is structural.
 *
 * Any compile-time evaluation or metaprogram execution occurs downstream
 * under the compiler's explicit phase, capability, effect, resource, and
 * security policies.
 *
 * Generated source MUST be treated as untrusted with respect to semantic
 * validity until it has passed the normal compiler pipeline.
 *
 * In particular:
 *
 *     generated source
 *          |
 *          v
 *     parse
 *          |
 *          v
 *     AST validation
 *          |
 *          v
 *     name resolution
 *          |
 *          v
 *     type validation
 *          |
 *          v
 *     effect validation
 *          |
 *          v
 *     capability validation
 *          |
 *          v
 *     resource validation
 *          |
 *          v
 *     semantic validation
 *          |
 *          v
 *     canonical IR
 *
 * ============================================================================
 * MACRO INTEGRATION
 * ============================================================================
 *
 * Unquotation is NOT itself a macro.
 *
 * Macro declarations and invocations remain owned by:
 *
 *     grammar/macros/
 *
 * A macro implementation may consume or produce quoted source.
 *
 * However:
 *
 *     unquote
 *
 * MUST NOT:
 *
 *     - invoke macro expansion directly;
 *     - bypass hygiene;
 *     - bypass name resolution;
 *     - bypass semantic validation;
 *     - construct compiler IR directly.
 *
 * If unquoted source is subsequently used by macro expansion, it enters the
 * normal macro expansion pipeline.
 *
 * ============================================================================
 * HYGIENE
 * ============================================================================
 *
 * This grammar has no authority over hygiene.
 *
 * The semantic/compiler layer must preserve, where applicable:
 *
 *     source spelling
 *     source location
 *     syntax context
 *     binding identity
 *     expansion provenance
 *     resolution environment
 *
 * Generated identifiers must receive the compiler's appropriate fresh
 * identities according to the macro/metaprogramming hygiene model.
 *
 * Unquotation MUST NOT provide an implicit unhygienic escape hatch.
 *
 * Any explicit capture mechanism must be separately specified, explicitly
 * authorized, and semantically validated.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * An unquoted value that contributes to generated source must retain enough
 * provenance for diagnostics and tooling.
 *
 * The compiler should be able to distinguish, where applicable:
 *
 *     handwritten source
 *     quoted source
 *     unquoted/generated source
 *     macro-generated source
 *     nested generated source
 *
 * Provenance is an AST/semantic/compiler concern, not a parser-side data
 * structure.
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * The parser must preserve source locations for:
 *
 *     `unquote`
 *
 * and:
 *
 *     the complete unquotation expression.
 *
 * The canonical AST builder is responsible for assigning the appropriate
 * source span to the resulting node.
 *
 * Diagnostics must be capable of pointing to:
 *
 *     - the unquote keyword;
 *     - the supplied expression;
 *     - the enclosing quotation;
 *     - the generated source location/provenance where applicable.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does NOT define AST structs.
 *
 * The frontend AST must represent the construct as a domain-neutral
 * metaprogramming operation.
 *
 * Conceptually:
 *
 *     UnquotationExpression
 *         {
 *             expression
 *             source_span
 *             provenance
 *             syntax_context
 *         }
 *
 * The exact Rust type and field names are owned by:
 *
 *     src/frontend/ast/
 *
 * No quantum-specific AST node is permitted merely because an unquoted
 * expression eventually produces quantum source.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     1. Is the unquotation syntactically well formed?
 *     2. Is it inside an active quotation context?
 *     3. What phase is the enclosing quotation in?
 *     4. What is the type of the unquoted expression?
 *     5. Is that type permitted in the quotation context?
 *     6. Is the value source structure, a token representation, a syntax
 *        representation, or another supported metaprogramming value?
 *     7. Are required effects authorized?
 *     8. Are required capabilities authorized?
 *     9. Is evaluation deterministic where determinism is required?
 *    10. Is the resulting generated structure valid canonical Zamani?
 *    11. Does generated source retain provenance?
 *    12. Does generated source pass ordinary semantic validation?
 *
 * This file deliberately does not answer any of those questions.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The unquoted expression uses the ordinary Zamani expression/type system.
 *
 * This file does not create:
 *
 *     MetaType
 *     MetaExpressionType
 *     UnquoteType
 *
 * merely to implement syntax.
 *
 * If Zamani defines source-structure types such as syntax/tree/token values,
 * those types belong to the canonical type-system and metaprogramming semantic
 * specifications.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Unquotation may depend semantically on compile-time effects.
 *
 * The grammar does not encode effect capabilities.
 *
 * Semantic analysis must reject unauthorized effects.
 *
 * In particular, merely writing:
 *
 *     unquote expression
 *
 * does not grant:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     hardware access
 *     secret access
 *     unrestricted environment access.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * An unquotation operation must not directly create:
 *
 *     ClassicalInstruction
 *     QuantumInstruction
 *     QuantumGate
 *     PhysicalQubit
 *     HardwareInstruction
 *     FPGA primitive
 *     ASIC cell
 *     GPU instruction
 *     CPU instruction
 *     schedule operation
 *     QEC operation
 *     ZQN operation
 *
 * If generated source contains executable quantum operations, the resulting
 * source follows the ordinary path:
 *
 *     generated source
 *          |
 *          v
 *     canonical AST
 *          |
 *          v
 *     semantic quantum representation
 *          |
 *          v
 *     quantum::ir
 *
 * There is no quotation-specific quantum IR.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Unquotation is domain-neutral.
 *
 * It may generate canonical quantum source such as:
 *
 *     apply H to q;
 *
 * or an extensible operation such as:
 *
 *     apply custom_gate to q;
 *
 * without this grammar enumerating any quantum operation.
 *
 * The quantum grammar remains authoritative for quantum syntax.
 *
 * Generated quantum source MUST subsequently use:
 *
 *     canonical AST
 *         ->
 *     quantum semantic analysis
 *         ->
 *     quantum::ir
 *
 * This preserves the repository's canonical quantum IR boundary.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Unquotation may generate canonical HDL or hardware-intent source.
 *
 * This file does not define:
 *
 *     physical wires
 *     fixed register widths
 *     device identifiers
 *     FPGA resources
 *     ASIC cell identifiers
 *     CPU identifiers
 *     GPU identifiers
 *     QPU identifiers
 *
 * Such constructs, where valid in Zamani, remain owned by their respective
 * canonical grammar components and semantic layers.
 *
 * ============================================================================
 * CLASSICAL / AI / DATA / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * No domain-specific unquotation syntax is required.
 *
 * Because the operand is canonical `expression`, this facility naturally
 * integrates with:
 *
 *     classical/
 *     quantum/
 *     hybrid/
 *     hdl/
 *     hardware/
 *     ai/
 *     data/
 *     distributed/
 *     networking/
 *     security/
 *     future domains
 *
 * Generated source must always re-enter the normal domain-specific semantic
 * pipeline.
 *
 * ============================================================================
 * QUOTATION INTEGRATION
 * ============================================================================
 *
 * Quotation remains the enclosing source-structure mechanism.
 *
 * The canonical conceptual relationship is:
 *
 *     quote {
 *         ...
 *         unquote expression
 *         ...
 *     }
 *
 * `unquote` therefore represents a phase boundary rather than an ordinary
 * runtime operator.
 *
 * Semantic validation determines whether the current context is a valid
 * quotation context.
 *
 * ============================================================================
 * REFLECTION INTEGRATION
 * ============================================================================
 *
 * Reflection and unquotation are distinct.
 *
 * Reflection inspects canonical language structures according to its own
 * semantic contract.
 *
 * Unquotation contributes an expression's semantic value to an enclosing
 * quotation.
 *
 * This file does not define reflection syntax and does not assume a reflection
 * implementation.
 *
 * ============================================================================
 * INTROSPECTION INTEGRATION
 * ============================================================================
 *
 * Introspection is distinct from unquotation.
 *
 * Introspection may semantically inspect target/resource/runtime/deployment
 * information when explicitly authorized.
 *
 * Unquotation itself does not grant target introspection.
 *
 * ============================================================================
 * SPECIALIZATION INTEGRATION
 * ============================================================================
 *
 * Specialization may consume source structures generated through quotation
 * and unquotation.
 *
 * Specialization remains downstream and must preserve program semantics.
 *
 * Unquotation does not select a target or specialization strategy.
 *
 * ============================================================================
 * COMPILE-TIME EXECUTION
 * ============================================================================
 *
 * Compile-time execution is a semantic/compiler concern.
 *
 * The parser only recognizes:
 *
 *     UNQUOTE expression
 *
 * It does not execute `expression`.
 *
 * Compiler implementation must maintain explicit phase boundaries so that
 * compile-time evaluation cannot accidentally become parser execution.
 *
 * ============================================================================
 * ERROR CATEGORIES
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Examples:
 *
 *     unquote
 *
 *     unquote )
 *
 *     unquote ,
 *
 * are parser errors when no canonical expression can follow.
 *
 * Contextual/semantic errors belong downstream.
 *
 * Examples:
 *
 *     unquote expression
 *
 * outside an active quotation;
 *
 * unquoted value of an invalid metaprogramming type;
 *
 * unauthorized compile-time effect;
 *
 * invalid generated syntax;
 *
 * invalid generated semantics;
 *
 * unavailable required capability.
 *
 * These MUST NOT be represented as grammar-level hardware/resource errors.
 *
 * ============================================================================
 * NEGATIVE SYNTAX CONTRACT
 * ============================================================================
 *
 * The following must NOT become valid merely because of this component:
 *
 *     splice expression
 *     quasiquote expression
 *     eval expression
 *     execute expression
 *     expand expression
 *
 * unless those keywords are independently introduced by the canonical
 * language specification.
 *
 * In particular:
 *
 *     splice
 *
 * is NOT a new keyword introduced by this file.
 *
 * ============================================================================
 * BOUNDARY CASES
 * ============================================================================
 *
 * The implementation must test:
 *
 *     unquote identifier
 *     unquote qualified.name
 *     unquote function_call(...)
 *     unquote expression + expression
 *     unquote conditional_expression
 *     unquote lambda_expression
 *     unquote quotation_expression
 *     unquote macro-related expression
 *     nested quotation/unquotation
 *     unquotation containing quantum expressions
 *     unquotation containing classical expressions
 *     unquotation containing HDL-related expressions
 *     unquotation containing generic expressions
 *
 * according to the canonical expression grammar.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must establish absence of artificial language limits.
 *
 * They should exercise increasingly large valid inputs generated by the test
 * harness rather than hard-coding a maximum in this grammar.
 *
 * Examples include:
 *
 *     many independent unquotations;
 *     deeply nested quotations;
 *     large canonical expressions;
 *     generated source containing many declarations;
 *     generated source containing many quantum operations;
 *     generated source containing large HDL structures;
 *     generated source containing large data/tensor expressions.
 *
 * The exact test workload is an implementation/resource concern.
 *
 * The grammar MUST NOT encode a maximum.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Identical source text must produce the same parse structure under identical
 * parser configuration.
 *
 * Tests must verify:
 *
 *     source
 *       |
 *       +--> ANTLR parser
 *
 * and:
 *
 *     source
 *       |
 *       +--> canonical Rust parser
 *
 * agree for stable syntax.
 *
 * Any intentional difference requires an explicit compatibility contract.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing source spelling remains:
 *
 *     unquote expression
 *
 * No alternate spelling is introduced here.
 *
 * Existing `quotation.g4` remains compatible because this file delegates to
 * its existing canonical core rules rather than redefining them.
 *
 * If the repository later moves `unquoteExpressionCore` from quotation.g4
 * into this file, the migration must update:
 *
 *     quotation.g4
 *     metaprogramming.g4
 *     canonical parser composition
 *     grammar.md
 *     specification/syntax.md
 *     AST conformance
 *     parser tests
 *     compatibility tests
 *
 * as one atomic grammar-contract migration.
 *
 * ============================================================================
 * RUST IMPLEMENTATION CONTRACT
 * ============================================================================
 *
 * This grammar contains no embedded target-language actions.
 *
 * Rust consumers MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and MUST use safe Rust.
 *
 * No `unsafe` implementation is required or permitted for this grammar
 * feature.
 *
 * This grammar itself contains no Rust implementation code.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file is a parser grammar.
 *
 * Canonical lexer vocabulary:
 *
 *     tokenVocab = ZamaniLexer
 *
 * The canonical parser composition layer must make the referenced canonical
 * rules available:
 *
 *     expression
 *     unquoteExpressionCore
 *     metaSpliceCore
 *
 * This file deliberately does not import or redefine the canonical expression
 * grammar.
 *
 * The repository's ANTLR build system is responsible for composing parser
 * components according to the established grammar architecture.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * Public integration rules:
 *
 *     unquotationExpression
 *     unquotationSplice
 *
 * Existing canonical rules consumed:
 *
 *     unquoteExpressionCore
 *     metaSpliceCore
 *
 * No duplicate canonical rule ownership is introduced.
 *
 * ============================================================================
 * INTEGRATION WITH metaprogramming.g4
 * ============================================================================
 *
 * The canonical metaprogramming composition should use:
 *
 *     unquotationExpression
 *
 * where an expression-level unquotation category is required.
 *
 * For generic splice composition it may use:
 *
 *     unquotationSplice
 *
 * The composition grammar MUST NOT create another independent implementation
 * of:
 *
 *     UNQUOTE expression
 *
 * ============================================================================
 * INTEGRATION WITH quotation.g4
 * ============================================================================
 *
 * Current ownership remains:
 *
 *     quotation.g4
 *         |
 *         +--> quoteExpressionCore
 *         +--> unquoteExpressionCore
 *         +--> metaQuoteCore
 *         +--> metaSpliceCore
 *
 * This file wraps the existing unquotation contracts without duplicating
 * their implementation.
 *
 * That is intentional because the existing repository file already provides
 * the canonical unquotation core.
 *
 * ============================================================================
 * INTEGRATION WITH Zamani.g4
 * ============================================================================
 *
 * Zamani.g4 remains the root composition grammar.
 *
 * It must expose metaprogramming syntax through the established
 * metaprogramming composition boundary.
 *
 * This file must never become a second root grammar.
 *
 * ============================================================================
 * INTEGRATION WITH src/frontend/ast/
 * ============================================================================
 *
 * The AST layer must provide a representation for unquotation that preserves:
 *
 *     expression
 *     source span
 *     phase/provenance metadata
 *     syntax context where required
 *
 * The exact AST type belongs to the frontend.
 *
 * No grammar-specific Rust type should be introduced merely to make this file
 * parse.
 *
 * ============================================================================
 * INTEGRATION WITH SEMANTIC ANALYSIS
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     quotation-context validation;
 *     phase validation;
 *     type validation;
 *     effect validation;
 *     capability validation;
 *     resource validation;
 *     provenance validation;
 *     hygiene integration;
 *     generated-source validation;
 *     determinism policy.
 *
 * This file supplies syntax only.
 *
 * ============================================================================
 * INTEGRATION WITH CANONICAL IR
 * ============================================================================
 *
 * Unquotation itself does not create a separate IR.
 *
 * After generated source is validated, executable constructs lower normally.
 *
 * Quantum constructs continue through:
 *
 *     semantic quantum model
 *            |
 *            v
 *        quantum::ir
 *
 * Classical constructs continue through the canonical classical/semantic IR.
 *
 * HDL/hardware constructs continue through their canonical semantic/IR
 * pipeline.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] Purpose is defined
 *     [x] Ownership is defined
 *     [x] Non-ownership is defined
 *     [x] Lexical dependency is defined
 *     [x] Canonical expression dependency is defined
 *     [x] Existing quotation ownership is respected
 *     [x] No duplicate core rule is introduced
 *     [x] Public integration boundary is defined
 *     [x] AST contract is defined
 *     [x] Semantic contract is defined
 *     [x] Phase contract is defined
 *     [x] Hygiene contract is defined
 *     [x] Provenance contract is defined
 *     [x] Diagnostics contract is defined
 *     [x] IR contract is defined
 *     [x] Quantum integration is defined
 *     [x] HDL integration is defined
 *     [x] POCO-REAF constraints are defined
 *     [x] No artificial scalability limit exists
 *     [x] Determinism contract is defined
 *     [x] Security boundary is defined
 *     [x] Rust 1.97/1.97.1 requirement is defined
 *     [x] Safe-Rust requirement is defined
 *     [x] Compatibility contract is defined
 *     [x] No lexer duplication exists
 *     [x] No expression grammar duplication exists
 *     [x] No second AST is introduced
 *     [x] No second IR is introduced
 *     [x] No hardware realization is introduced
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * Unquotation is a SOURCE-STRUCTURE PHASE BOUNDARY.
 *
 * It is not:
 *
 *     a runtime execution primitive;
 *     a macro engine;
 *     a reflection engine;
 *     a compiler escape hatch;
 *     a hardware interface;
 *     a resource allocator;
 *     a target selector;
 *     a quantum IR;
 *     an HDL backend;
 *     a second language.
 *
 * Its sole parser-level responsibility is to recognize the canonical
 * unquotation construct and provide a stable composition boundary.
 *
 * The canonical semantic pipeline remains:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     frontend AST
 *       ->
 *     semantic analysis
 *       ->
 *     generated/canonical source structure
 *       ->
 *     semantic validation
 *       ->
 *     canonical IR
 *       ->
 *     optimization
 *       ->
 *     routing/scheduling/resilience
 *       ->
 *     QEC/ZQN where applicable
 *       ->
 *     HAL
 *       ->
 *     target realization
 *
 * This preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * because unquotation modifies source structure rather than binding the
 * language to any particular machine, device, topology, capacity, vendor, or
 * hardware generation.
 *
 * ============================================================================
 */

parser grammar Unquotation;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * INDEPENDENT UNQUOTATION EXPRESSION BOUNDARY
 * ============================================================================
 *
 * This is the primary public rule owned by this file.
 *
 * The actual canonical source syntax remains owned by quotation.g4 through:
 *
 *     unquoteExpressionCore
 *
 * No duplicate `UNQUOTE expression` production is created here.
 */

unquotationExpression
    : unquoteExpressionCore
    ;


/*
 * ============================================================================
 * UNQUOTATION / SPLICE BOUNDARY
 * ============================================================================
 *
 * `metaSpliceCore` is the repository's existing structural metaprogramming
 * category for canonical unquotation.
 *
 * This wrapper provides an independently named integration point without
 * creating another source syntax.
 */

unquotationSplice
    : metaSpliceCore
    ;