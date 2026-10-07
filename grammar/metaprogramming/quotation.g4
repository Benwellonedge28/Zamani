/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/metaprogramming/quotation.g4
 *
 * Grammar:
 *     Quotation
 *
 * Status:
 *     CANONICAL QUOTATION / UNQUOTATION SYNTAX COMPONENT
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021+
 *
 * Safety:
 *     No embedded Rust.
 *     No actions.
 *     No semantic predicates.
 *     No unsafe implementation requirement.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the single syntax owner for:
 *
 *     quote
 *     unquote
 *
 * Quotation represents canonical Zamani source structure as a
 * metaprogramming value.
 *
 * Unquotation represents the controlled syntactic boundary through which an
 * ordinary Zamani expression supplies a value to an enclosing quotation.
 *
 *
 * OWNS
 * -----
 *
 *     quoteExpressionCore
 *     unquoteExpressionCore
 *     metaQuoteCore
 *     metaSpliceCore
 *
 * DOES NOT OWN
 * -------------
 *
 *     identifiers
 *     names
 *     paths
 *     expressions
 *     expression precedence
 *     statements
 *     declarations
 *     blocks
 *     types
 *     patterns
 *     macros
 *     macro hygiene
 *     reflection
 *     introspection
 *     code generation
 *     specialization
 *     compile-time evaluation
 *     AST implementation
 *     semantic analysis
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     provenance implementation
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     hardware realization
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       +-----------------------------+
 *       |                             |
 *       v                             v
 *     ordinary semantics       metaprogramming semantics
 *                                     |
 *                                     v
 *                                quotation
 *                                     |
 *                                unquotation
 *                                     |
 *                                     v
 *                              constructed source
 *                                     |
 *                                     v
 *                           canonical frontend pipeline
 *                                     |
 *                                     v
 *                              semantic validation
 *                                     |
 *                                     v
 *                              canonical IR
 *                                     |
 *              +----------------------+----------------------+
 *              |                      |                      |
 *              v                      v                      v
 *          classical              quantum::ir           HDL/hardware
 *              |                      |                      |
 *              +----------------------+----------------------+
 *                                     |
 *                                     v
 *                            optimization/lowering
 *                                     |
 *                              routing/scheduling
 *                                     |
 *                            resilience/QEC/ZQN
 *                                     |
 *                                     v
 *                                    HAL
 *                                     |
 *                                     v
 *                              target realization
 *
 * ============================================================================
 * CRITICAL DESIGN PRINCIPLE
 * ============================================================================
 *
 * Quotation MUST reuse canonical Zamani syntax.
 *
 * It MUST NOT create a second miniature language containing separate:
 *
 *     quoteExpression
 *     quoteStatement
 *     quoteDeclaration
 *     quoteType
 *     quoteIdentifier
 *     quotePattern
 *
 * implementations.
 *
 * The quotation body is therefore the canonical:
 *
 *     blockExpression
 *
 * Unquotation consumes the canonical:
 *
 *     expression
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It defines no lexer rules.
 *
 * Required canonical tokens:
 *
 *     QUOTE
 *     UNQUOTE
 *
 * Their source spellings are owned by:
 *
 *     grammar/lexer/keywords.g4
 *
 * The public lexer boundary is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * is mandatory.
 *
 * This grammar MUST NOT use:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * even though ZamaniTokens is an internal lexical composition grammar.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 * Lexer:
 *
 *     QUOTE
 *     UNQUOTE
 *
 * Canonical parser rules:
 *
 *     blockExpression
 *     expression
 *
 * These rules are supplied by the canonical parser composition:
 *
 *     Core
 *     Expressions
 *
 * The quotation grammar intentionally does not import the complete
 * Expressions grammar itself.
 *
 * Reason:
 *
 *     Expressions
 *         |
 *         v
 *     MetaprogrammingExpressions
 *         |
 *         v
 *     quotation
 *
 * is part of the final composition graph.
 *
 * Importing the complete expression grammar back into quotation would create
 * an unnecessary circular composition dependency.
 *
 * The canonical root parser resolves these shared rules after grammar
 * composition.
 *
 * ANTLR merges imported grammar rules into the root grammar before semantic
 * checking, so this is a deliberate composition-boundary design.
 *
 * ============================================================================
 * PUBLIC RULE CONTRACT
 * ============================================================================
 *
 * Primary rules:
 *
 *     quoteExpressionCore
 *     unquoteExpressionCore
 *
 * Compatibility/integration aliases:
 *
 *     metaQuoteCore
 *     metaSpliceCore
 *
 * `metaSpliceCore` is NOT a new source keyword.
 *
 * It is an integration name for the canonical `unquote` construct.
 *
 * ============================================================================
 * SOURCE SYNTAX
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     quote { ... }
 *
 *     unquote expression
 *
 * The language specification defines:
 *
 *     QuoteExpression ::= "quote" BlockExpression
 *
 *     UnquoteExpression ::= "unquote" Expression
 *
 * This grammar implements exactly those structural contracts.
 *
 * ============================================================================
 * QUOTATION
 * ============================================================================
 *
 * A quotation captures canonical source structure.
 *
 *     quote { ... }
 *
 * The body is parsed by:
 *
 *     blockExpression
 *
 * Consequently a quotation may contain any source construct that is legal
 * inside the canonical block grammar.
 *
 * This automatically permits future:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     data
 *     distributed
 *     networking
 *     security
 *     resource
 *     policy
 *     metaprogramming
 *     dialect
 *
 * constructs without modifying this file.
 *
 * ============================================================================
 * UNQUOTATION
 * ============================================================================
 *
 * An unquotation supplies an ordinary Zamani expression to the surrounding
 * quotation context.
 *
 *     unquote expression
 *
 * This grammar recognizes only the syntactic boundary.
 *
 * It does NOT determine whether the expression:
 *
 *     - is compile-time available;
 *     - has a quotation-compatible type;
 *     - is source structure;
 *     - is a syntax value;
 *     - is a token sequence;
 *     - is executable;
 *     - is deterministic;
 *     - has permitted effects;
 *     - has sufficient capabilities;
 *     - satisfies resource requirements.
 *
 * Those are semantic/compiler decisions.
 *
 * ============================================================================
 * PHASE CONTRACT
 * ============================================================================
 *
 * Quotation establishes a metaprogramming phase.
 *
 * Unquotation crosses from ordinary expression semantics into that quotation
 * context.
 *
 * The grammar deliberately does NOT enforce:
 *
 *     unquote outside quote
 *
 * through semantic predicates.
 *
 * The semantic layer must diagnose invalid phase usage.
 *
 * This keeps parsing deterministic and target-independent.
 *
 * ============================================================================
 * NESTING
 * ============================================================================
 *
 * Nested quotation is syntactically supported.
 *
 * Example:
 *
 *     quote {
 *         let inner = quote {
 *             let value = 1;
 *         };
 *     }
 *
 * There is no language-defined nesting limit.
 *
 * Compiler recursion/resource protection is implementation policy and MUST
 * NOT become a grammar constant.
 *
 * ============================================================================
 * HYGIENE
 * ============================================================================
 *
 * This grammar performs no identifier capture or renaming.
 *
 * Hygiene belongs to the semantic/macro subsystem.
 *
 * Semantic processing must preserve:
 *
 *     lexical scope
 *     binding identity
 *     quotation phase
 *     unquotation origin
 *     source span
 *     transformation provenance
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Quotation-related syntax must preserve:
 *
 *     quote span
 *     quotation body span
 *     unquote span
 *     unquoted expression span
 *     enclosing quotation identity
 *     generated/transformed source origin
 *
 * Provenance is consumed downstream by:
 *
 *     diagnostics
 *     macro expansion
 *     source generation
 *     deterministic builds
 *     auditing
 *     reproducibility
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates only ANTLR parse-tree contexts.
 *
 * The canonical frontend AST owns the actual semantic representation.
 *
 * The recommended AST representation is:
 *
 *     Expression::Quote(Span, Box<Expression>)
 *
 * where the child expression is the canonical Block expression.
 *
 *     Expression::Unquote(Span, Box<Expression>)
 *
 * where the child is the canonical expression.
 *
 * The exact Rust type names remain owned by:
 *
 *     src/ast/mod.rs
 *
 * No quotation-specific competing AST hierarchy is permitted.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes:
 *
 *     valid quotation syntax
 *     valid unquotation expression syntax
 *
 * Semantic analysis establishes:
 *
 *     quotation phase
 *     unquotation phase
 *     quotation value type
 *     scope/capture legality
 *     hygiene
 *     compile-time availability
 *     effect legality
 *     capability legality
 *     resource requirements
 *     policy legality
 *     deterministic-build compatibility
 *     provenance
 *     generated-source validity
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * `quote` itself is syntax for representing source structure.
 *
 * It does not automatically grant:
 *
 *     filesystem
 *     network
 *     native
 *     foreign
 *     reflection
 *     subprocess
 *     hardware
 *     device
 *     randomness
 *
 * access.
 *
 * `unquote` does not grant compile-time execution privileges.
 *
 * Any effectful expression used by unquotation must be checked by the
 * ordinary effect system.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Quotation syntax does not grant capabilities.
 *
 * Capability requirements are semantic information.
 *
 * For example, if a metaprogram needs a capability to transform source,
 * that capability is checked by the metaprogramming/compiler policy layer.
 *
 * This grammar must never encode:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     device
 *     vendor
 *     backend
 *
 * selection.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This grammar contains no resource limits.
 *
 * It MUST NOT define:
 *
 *     MAX_QUOTE_DEPTH
 *     MAX_UNQUOTE_DEPTH
 *     MAX_QUOTED_ITEMS
 *     MAX_GENERATED_NODES
 *     MAX_QUOTATION_SIZE
 *     MAX_EXPANSION_SIZE
 *
 * or any equivalent artificial ceiling.
 *
 * Practical limits belong to compiler resource policy.
 *
 * If compiler resources are exhausted, the compiler must report a structured
 * diagnostic rather than changing the language grammar.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Quotation preserves source-level intent.
 *
 * It must remain independent of:
 *
 *     CPU count
 *     core count
 *     thread count
 *     GPU count
 *     FPGA count
 *     accelerator count
 *     QPU count
 *     qubit count
 *     memory capacity
 *     storage capacity
 *     register width
 *     tensor rank
 *     node count
 *     device count
 *     network size
 *     physical topology
 *
 * A quoted program that is semantically valid remains the same source-level
 * program regardless of eventual target realization.
 *
 * Physical feasibility is evaluated downstream.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quotation can contain canonical quantum source.
 *
 * It does not enumerate quantum operations.
 *
 * It does not know:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     CX
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *
 * or any future operation.
 *
 * Quoted quantum source follows:
 *
 *     quotation
 *       ->
 *     canonical AST
 *       ->
 *     quantum semantic model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience/QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *
 * There is no quotation-specific quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Quoted HDL and hardware-intent source uses the ordinary HDL/hardware
 * grammar.
 *
 * Quotation must not introduce:
 *
 *     fixed bus widths
 *     fixed register widths
 *     physical device identifiers
 *     physical FPGA regions
 *     ASIC cell identifiers
 *     fixed clock frequencies
 *     fixed memory capacities
 *     fixed hardware topology
 *
 * ============================================================================
 * GENERATION CONTRACT
 * ============================================================================
 *
 * Quotation and generation are separate concepts.
 *
 * Quotation:
 *
 *     represents source structure.
 *
 * Generation:
 *
 *     creates/emits source structure.
 *
 * Generation is owned by:
 *
 *     grammar/metaprogramming/generation.g4
 *
 * Generated source MUST re-enter the canonical frontend/semantic pipeline.
 *
 * ============================================================================
 * MACRO CONTRACT
 * ============================================================================
 *
 * Macro declaration/invocation syntax remains owned by:
 *
 *     grammar/macros/
 *
 * Quotation may be consumed by macros.
 *
 * Quotation itself does not:
 *
 *     expand macros
 *     perform substitution
 *     perform hygiene
 *     invoke a macro engine
 *
 * ============================================================================
 * REFLECTION CONTRACT
 * ============================================================================
 *
 * Reflection remains owned by:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * Quoted source may contain reflection syntax because it is canonical Zamani
 * source.
 *
 * Reflection results may subsequently participate in quotation construction,
 * subject to semantic authorization.
 *
 * ============================================================================
 * COMPILE-TIME CONTRACT
 * ============================================================================
 *
 * Compile-time execution remains owned by:
 *
 *     grammar/metaprogramming/compile-time-execution.g4
 *
 * Unquotation does not automatically execute its operand.
 *
 * Compile-time execution requires the ordinary:
 *
 *     type
 *     effect
 *     capability
 *     resource
 *     policy
 *     provenance
 *
 * validation pipeline.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * It must never construct:
 *
 *     ClassicalIR
 *     QuantumIR
 *     HDLIR
 *     HardwareIR
 *     RoutingIR
 *     ScheduleIR
 *     QEC representation
 *     ZQN representation
 *     HAL representation
 *
 * Any constructed source ultimately follows the ordinary semantic-to-IR path.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing quotation MUST NOT:
 *
 *     execute code
 *     access files
 *     access networks
 *     access credentials
 *     inspect processes
 *     inspect hardware
 *     access devices
 *     invoke subprocesses
 *     bypass compiler policy
 *
 * Quoted source is syntax/data until an authorized downstream phase operates
 * on it.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source tokens
 *     grammar
 *     selected language/compatibility version
 *
 * It must not depend on:
 *
 *     hardware availability
 *     filesystem state
 *     network state
 *     wall-clock time
 *     randomness
 *     target selection
 *     deployment topology
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics are produced by the parser infrastructure.
 *
 * Required malformed forms include:
 *
 *     quote
 *     quote (
 *     quote ()
 *     unquote
 *     unquote (
 *     unquote ()
 *
 * Semantic diagnostics include:
 *
 *     unquote outside quotation
 *     invalid quotation phase
 *     invalid quotation value
 *     invalid capture
 *     hygiene violation
 *     unavailable compile-time value
 *     effect violation
 *     capability violation
 *     resource violation
 *     policy violation
 *     provenance failure
 *     invalid generated source
 *
 * This grammar itself emits no custom diagnostics.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing source spelling is preserved:
 *
 *     quote
 *     unquote
 *
 * Existing specification:
 *
 *     QuoteExpression ::= "quote" BlockExpression
 *     UnquoteExpression ::= "unquote" Expression
 *
 * remains authoritative.
 *
 * The change is structural:
 *
 *     quotation.g4
 *
 * becomes the actual parser implementation of the already specified feature.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses:
 *
 *     canonical block repetition
 *     canonical expression parsing
 *
 * It introduces no quotation-specific recursive data copying.
 *
 * There is no finite grammar-level limit on:
 *
 *     quotation count
 *     quotation nesting
 *     body size
 *     unquotation count
 *     expression complexity
 *     declaration complexity
 *     quantum operations inside quotations
 *     HDL constructs inside quotations
 *     generated source size
 *
 * Compiler memory/time limits remain implementation policy.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * FORBIDDEN:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *     MAX_QUOTE_DEPTH
 *     MAX_UNQUOTE_DEPTH
 *     MAX_GENERATED_NODES
 *
 * No such limits are present in this grammar.
 *
 * No physical target identifiers are present.
 *
 * No vendor gate catalogue is present.
 *
 * No hardware topology is present.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     quote {}
 *
 *     quote {
 *         let x = 1;
 *     }
 *
 *     quote {
 *         fn generated() {
 *             return 1;
 *         }
 *     }
 *
 *     quote {
 *         let value = unquote source;
 *     }
 *
 *     quote {
 *         let nested = quote {
 *             let x = 1;
 *         };
 *     }
 *
 * Negative:
 *
 *     quote
 *     quote (
 *     quote ()
 *     unquote
 *     unquote ()
 *
 * Boundary:
 *
 *     empty quotation
 *     nested quotation
 *     deeply nested quotation
 *     large quotation
 *     many unquotation sites
 *     Unicode identifiers
 *     comments
 *     strings containing "quote"
 *     strings containing "unquote"
 *     quoted quantum source
 *     quoted HDL source
 *     quoted hybrid source
 *     quoted distributed source
 *     quoted AI/data source
 *
 * Scalability:
 *
 *     increasing quotation body size
 *     increasing nesting
 *     increasing unquotation count
 *
 * No scalability test establishes a universal maximum.
 *
 * ============================================================================
 * INTEGRATION
 * ============================================================================
 *
 * This component is integrated by:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * which imports:
 *
 *     Quotation
 *
 * The expression-level metaprogramming adapter:
 *
 *     grammar/expressions/metaprogramming.g4
 *
 * exposes:
 *
 *     quoteExpressionCore
 *     unquoteExpressionCore
 *
 * The canonical expression composition:
 *
 *     grammar/expressions/expressions.g4
 *
 * already imports the expression-level metaprogramming component.
 *
 * The root parser:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * imports both:
 *
 *     Expressions
 *     Metaprogramming
 *
 * so all shared canonical rules are available in the final composed parser.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] parser grammar declaration is present;
 *     [ ] canonical lexer vocabulary is used;
 *     [ ] QUOTE exists exactly once lexically;
 *     [ ] UNQUOTE exists exactly once lexically;
 *     [ ] quoteExpressionCore exists exactly once;
 *     [ ] unquoteExpressionCore exists exactly once;
 *     [ ] metaQuoteCore is only a compatibility alias;
 *     [ ] metaSpliceCore is only a compatibility alias;
 *     [ ] blockExpression is reused;
 *     [ ] expression is reused;
 *     [ ] no quotation mini-language exists;
 *     [ ] no quotation-specific AST exists in grammar;
 *     [ ] no IR exists in grammar;
 *     [ ] no hardware selection exists;
 *     [ ] no resource limits exist;
 *     [ ] no semantic predicates exist;
 *     [ ] no embedded Rust exists;
 *     [ ] nested quotation parses;
 *     [ ] unquotation parses;
 *     [ ] malformed syntax fails;
 *     [ ] generated source re-enters the canonical pipeline;
 *     [ ] quantum source reaches quantum::ir normally;
 *     [ ] Rust implementation remains safe Rust;
 *     [ ] Rust 1.97+ remains supported.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar Quotation;


/*
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 */

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * QUOTATION
 * ============================================================================
 *
 * quote { ... }
 *
 * The body is the canonical blockExpression.
 *
 * ============================================================================
 */

quoteExpressionCore
    : QUOTE blockExpression
    ;


/*
 * ============================================================================
 * UNQUOTATION
 * ============================================================================
 *
 * unquote expression
 *
 * The operand is the canonical expression.
 *
 * ============================================================================
 */

unquoteExpressionCore
    : UNQUOTE expression
    ;


/*
 * ============================================================================
 * METAPROGRAMMING COMPATIBILITY ALIASES
 * ============================================================================
 *
 * These names exist because the metaprogramming composition layer already
 * exposes:
 *
 *     metaQuote
 *     metaSplice
 *
 * They introduce no additional source syntax.
 *
 * ============================================================================
 */

metaQuoteCore
    : quoteExpressionCore
    ;

metaSpliceCore
    : unquoteExpressionCore
    ;