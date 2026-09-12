/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/annotations.g4
 *
 * Role:
 *     Canonical lexical component for Zamani annotation/attribute markers.
 *
 * Grammar technology:
 *     ANTLR4 lexer grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no executable Rust code and requires no unsafe
 *     implementation. Generated Zamani compiler/runtime code MUST remain
 *     compatible with the repository-wide safe-Rust policy.
 *
 * ============================================================================
 *
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * This file owns the lexical representation of the annotation marker:
 *
 *     @
 *
 * It deliberately does NOT own the complete annotation construct.
 *
 * The complete annotation syntax belongs to the parser layer.
 *
 * Conceptually:
 *
 *     @
 *     |
 *     +--> identifier
 *     |
 *     +--> qualified name
 *     |
 *     +--> annotation arguments
 *     |
 *     +--> nested annotation values
 *     |
 *     +--> expressions
 *     |
 *     +--> future annotation forms
 *
 * are parser/semantic concerns.
 *
 * ============================================================================
 *
 * WHY `@identifier` IS NOT ONE TOKEN
 * ============================================================================
 *
 * An earlier/legacy grammar pattern represents annotations approximately as:
 *
 *     annotation
 *         : '@' IDENTIFIER ('(' annotationValue? ')')?
 *         ;
 *
 * The lexer must preserve that compositional architecture.
 *
 * Therefore this file MUST NOT define:
 *
 *     ANNOTATION : '@' IDENTIFIER ;
 *
 * or:
 *
 *     ANNOTATION : '@' ID_START ID_CONTINUE* ;
 *
 * Doing so would incorrectly make the annotation name inseparable from the
 * annotation marker and would make future grammar evolution harder.
 *
 * It would also prevent the parser from naturally supporting forms such as:
 *
 *     @name
 *     @name(...)
 *     @namespace.name
 *     @name(expr)
 *     @name(a = value)
 *     @name(other_annotation)
 *
 * and future extensible annotation forms.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the `AT` token;
 *     - lexical recognition of `@`.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - annotation names;
 *     - identifiers;
 *     - qualified names;
 *     - annotation arguments;
 *     - annotation expressions;
 *     - annotation value typing;
 *     - annotation semantics;
 *     - annotation validation;
 *     - attribute inheritance;
 *     - compiler directives;
 *     - target selection;
 *     - hardware selection;
 *     - quantum semantics;
 *     - QEC semantics;
 *     - ZQN semantics;
 *     - scheduling;
 *     - optimization;
 *     - runtime behavior;
 *     - AST construction;
 *     - IR construction.
 *
 * ============================================================================
 *
 * SINGLE SOURCE OF TRUTH
 * ============================================================================
 *
 * `AT` MUST have exactly one lexical owner in the canonical lexer assembly.
 *
 * Consequently:
 *
 *     grammar/lexer/tokens.g4
 *
 * MUST NOT independently define:
 *
 *     AT : '@' ;
 *
 * after this component is integrated.
 *
 * Likewise parser grammars MUST NOT declare a second lexer rule named `AT`.
 *
 * Parser grammars consume the token produced by this component.
 *
 * ============================================================================
 *
 * CANONICAL PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniAnnotations
 *       |
 *       | AT
 *       v
 *     canonical Zamani lexer
 *       |
 *       v
 *     Zamani parser
 *       |
 *       v
 *     annotation AST node
 *       |
 *       v
 *     semantic annotation validation
 *       |
 *       +--> compiler metadata
 *       +--> effect metadata
 *       +--> capability metadata
 *       +--> resource metadata
 *       +--> documentation metadata
 *       +--> quantum IR metadata where explicitly applicable
 *
 * The annotation lexer MUST NOT bypass the AST or semantic layers.
 *
 * ============================================================================
 *
 * PARSER CONTRACT
 * ============================================================================
 *
 * The parser may consume:
 *
 *     AT IDENTIFIER
 *
 * or, depending on the canonical language specification:
 *
 *     AT qualifiedName
 *
 * followed by an optional argument/value structure.
 *
 * This file intentionally does not choose between those semantic forms.
 *
 * That decision belongs to the canonical parser/specification layer.
 *
 * ============================================================================
 *
 * IDENTIFIER INTEGRATION
 * ============================================================================
 *
 * Annotation names use the canonical identifier system.
 *
 * The canonical identifier grammar is:
 *
 *     grammar/lexer/identifiers.g4
 *
 * Therefore this file MUST NOT redefine:
 *
 *     IDENTIFIER
 *     IDENTIFIER_START
 *     IDENTIFIER_CONTINUE
 *
 * Annotation names remain ordinary language identifiers unless a separate
 * specification explicitly declares a spelling reserved.
 *
 * This permits open-ended annotations such as:
 *
 *     @inline
 *     @deprecated
 *     @quantum
 *     @resource
 *     @vendor_specific
 *     @future_annotation
 *
 * without encoding an exhaustive annotation vocabulary into the lexer.
 *
 * ============================================================================
 *
 * DOMAIN INDEPENDENCE
 * ============================================================================
 *
 * This file does not reserve annotation names for:
 *
 *     - CPUs
 *     - GPUs
 *     - FPGAs
 *     - ASICs
 *     - QPUs
 *     - quantum gates
 *     - vendors
 *     - devices
 *     - topology
 *     - qubit counts
 *     - memory sizes
 *     - cluster sizes
 *     - accelerators
 *     - network endpoints
 *
 * For example, names such as:
 *
 *     @cuda
 *     @qpu
 *     @fpga
 *     @surface_code
 *     @vendor_operation
 *     @device_hint
 *
 * are not intrinsically lexer-level concepts.
 *
 * Their legality and meaning belong to semantic/domain layers.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Annotation marker syntax is independent of execution hardware.
 *
 * The same:
 *
 *     @
 *
 * token is valid regardless of whether the program is eventually realized on:
 *
 *     - an embedded processor;
 *     - a CPU;
 *     - a multicore CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a quantum simulator;
 *     - a heterogeneous accelerator;
 *     - a cluster;
 *     - a supercomputer;
 *     - a distributed environment;
 *     - a cloud environment;
 *     - a future machine.
 *
 * The annotation's semantic meaning may influence compilation, but the lexical
 * representation itself does not encode the target.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * This file contains NO finite resource limit.
 *
 * In particular, it does not impose:
 *
 *     - maximum annotations per declaration;
 *     - maximum annotation nesting depth;
 *     - maximum annotation-name length;
 *     - maximum argument count;
 *     - maximum value size;
 *     - maximum declaration count;
 *     - maximum program size;
 *     - maximum resource count;
 *     - maximum device count;
 *     - maximum qubit count;
 *     - maximum CPU count;
 *     - maximum accelerator count.
 *
 * Repetition and nesting are parser/implementation concerns and must be
 * bounded only by genuine implementation/resource limits rather than an
 * arbitrary language constant.
 *
 * ============================================================================
 *
 * NO SEMANTIC KEYWORD TABLE
 * ============================================================================
 *
 * Do NOT create rules such as:
 *
 *     INLINE_ANNOTATION
 *     QUANTUM_ANNOTATION
 *     GPU_ANNOTATION
 *     FPGA_ANNOTATION
 *     QEC_ANNOTATION
 *     ZQN_ANNOTATION
 *
 * merely because those annotations currently exist or may exist.
 *
 * Annotation names must remain extensible.
 *
 * Stable language keywords belong to the canonical keyword grammar.
 *
 * Domain-specific annotation names remain identifiers unless the language
 * specification explicitly reserves them.
 *
 * ============================================================================
 *
 * NO TARGET COUPLING
 * ============================================================================
 *
 * This file must never import or depend upon:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     scheduling
 *     routing
 *     optimization
 *     hardware HAL
 *     runtime
 *     simulator
 *
 * Those systems consume semantic information after parsing.
 *
 * ============================================================================
 *
 * LEXICAL DISAMBIGUATION
 * ============================================================================
 *
 * The annotation marker is intentionally a single-character token.
 *
 * This is important because `@` can participate in other language-level
 * constructs or future extensions.
 *
 * The lexer therefore emits:
 *
 *     @
 *
 * rather than consuming arbitrary following source text.
 *
 * For example:
 *
 *     @compile
 *
 * is tokenized conceptually as:
 *
 *     AT
 *     IDENTIFIER("compile")
 *
 * rather than:
 *
 *     ANNOTATION("compile")
 *
 * This preserves parser flexibility.
 *
 * ============================================================================
 *
 * HARDWARE ADDRESS INTERACTION
 * ============================================================================
 *
 * Zamani may support hardware/resource address syntax elsewhere.
 *
 * A hardware address such as:
 *
 *     @0x1234
 *
 * MUST NOT cause this annotation component to redefine hardware-address
 * semantics.
 *
 * If hardware address literals are supported, their lexical rule must be
 * deliberately ordered and owned by:
 *
 *     grammar/lexer/hardware-literals.g4
 *
 * or the canonical hardware-literal component.
 *
 * Annotation syntax remains:
 *
 *     @ + identifier/name
 *
 * at the parser level.
 *
 * ============================================================================
 *
 * COMMENTS AND WHITESPACE
 * ============================================================================
 *
 * This file does not own whitespace.
 *
 * This file does not own comments.
 *
 * This file does not own documentation comments.
 *
 * Those remain in their dedicated lexical components.
 *
 * The following are therefore intentionally NOT defined here:
 *
 *     WS
 *     LINE_COMMENT
 *     BLOCK_COMMENT
 *     DOC_COMMENT
 *
 * ============================================================================
 *
 * UNICODE
 * ============================================================================
 *
 * The `@` marker itself is ASCII and unambiguous.
 *
 * Annotation names inherit the canonical identifier policy from:
 *
 *     grammar/lexer/identifiers.g4
 *
 * This file must not duplicate or silently alter that policy.
 *
 * ============================================================================
 *
 * ERROR HANDLING
 * ============================================================================
 *
 * There is no special annotation-specific lexer error rule here.
 *
 * Invalid forms such as:
 *
 *     @
 *
 * are not necessarily lexical errors.
 *
 * `@` is a valid lexical token.
 *
 * Whether an annotation name is required, and what may follow `@`, is a
 * parser/semantic decision.
 *
 * This separation produces better diagnostics:
 *
 *     lexical layer:
 *         recognized `@`
 *
 *     parser layer:
 *         expected annotation name
 *
 *     semantic layer:
 *         unknown/invalid annotation
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * The rule is deterministic:
 *
 *     AT : '@' ;
 *
 * There is no dynamic behavior, semantic lookup, external state, or target
 * discovery involved in lexing.
 *
 * ============================================================================
 *
 * ANTLR INTEGRATION
 * ============================================================================
 *
 * Canonical lexer assembly should import this lexer component, for example:
 *
 *     lexer grammar ZamaniLexer;
 *
 *     import
 *         ZamaniAnnotations,
 *         ...
 *     ;
 *
 * The exact import list belongs to:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * and must contain this component exactly once.
 *
 * If the repository's canonical lexer assembly instead imports through
 * `grammar/lexer/tokens.g4`, that assembly layer must likewise import this
 * component exactly once.
 *
 * ============================================================================
 *
 * TOKEN STABILITY
 * ============================================================================
 *
 * `AT` is the public lexer token name.
 *
 * Renaming it to an implementation-specific name such as:
 *
 *     ANNOTATION_START
 *     ATTRIBUTE_PREFIX
 *
 * would unnecessarily break existing parser consumers.
 *
 * If a future language revision requires a different conceptual name, that
 * change must go through the grammar compatibility/versioning policy.
 *
 * ============================================================================
 *
 * AST INTEGRATION
 * ============================================================================
 *
 * This file produces no AST node.
 *
 * The parser should construct the annotation syntax node using:
 *
 *     AT
 *     +
 *     canonical identifier/name
 *     +
 *     optional annotation arguments
 *
 * The AST should preserve source spans so diagnostics and tooling can point
 * back to the exact `@` marker and annotation name.
 *
 * ============================================================================
 *
 * SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Annotation meaning is resolved after parsing.
 *
 * Possible semantic consumers include:
 *
 *     - compiler attributes;
 *     - optimization hints;
 *     - effect declarations;
 *     - resource requirements;
 *     - capability requirements;
 *     - diagnostics;
 *     - documentation;
 *     - interoperability metadata;
 *     - quantum metadata;
 *     - hardware intent;
 *     - deployment metadata.
 *
 * None of those meanings belong in this file.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * This component may lex annotation syntax used by quantum programs, for
 * example:
 *
 *     @logical
 *     @error_corrected
 *     @resource(...)
 *     @capability(...)
 *
 * but it does not define what those annotations mean.
 *
 * Semantic lowering may eventually attach selected annotation metadata to the
 * canonical quantum IR.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 *
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_ANNOTATIONS
 *     MAX_ANNOTATION_NAME_LENGTH
 *     MAX_ANNOTATION_ARGUMENTS
 *     MAX_NESTING
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_MEMORY
 *     MAX_NODES
 *     MAX_RESOURCES
 *
 * No numeric machine property is required by this lexical component.
 *
 * ============================================================================
 *
 * TEST CONTRACT
 * ============================================================================
 *
 * Direct lexical tests MUST verify:
 *
 *     @
 *
 * tokenizes as:
 *
 *     AT
 *
 * The following must tokenize with the marker separated from the name:
 *
 *     @inline
 *     @quantum
 *     @resource
 *     @custom_annotation
 *
 * Conceptually:
 *
 *     AT IDENTIFIER
 *
 * The lexer component must NOT emit a single combined annotation token.
 *
 * ============================================================================
 *
 * POSITIVE TESTS
 * ============================================================================
 *
 *     @inline
 *     @quantum
 *     @resource
 *     @custom_annotation
 *     @future_annotation
 *     @annotation123
 *     @_annotation
 *
 * ============================================================================
 *
 * NEGATIVE / PARSER-BOUNDARY TESTS
 * ============================================================================
 *
 * The lexer should still recognize the marker in:
 *
 *     @
 *
 * even though the parser may subsequently reject the incomplete annotation.
 *
 * This distinction is intentional.
 *
 * ============================================================================
 *
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Verify the same annotation marker can appear lexically in programs involving:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware
 *     distributed computation
 *     AI/data computation
 *
 * without changing the tokenization rule.
 *
 * ============================================================================
 *
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * A source containing:
 *
 *     @name
 *
 * must preserve the `@` marker and annotation-name spelling through:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> printer/serializer
 *
 * where the repository provides round-trip printing.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] `AT` is defined exactly once in the canonical lexer assembly.
 *
 * [ ] No combined `ANNOTATION` lexer token exists for `@identifier`.
 *
 * [ ] Annotation names continue to use canonical IDENTIFIER/name rules.
 *
 * [ ] Parser annotation rules consume AT.
 *
 * [ ] Annotation values remain parser/semantic constructs.
 *
 * [ ] No annotation-specific hardware limits exist.
 *
 * [ ] No quantum limits exist.
 *
 * [ ] No resource limits exist.
 *
 * [ ] No target-specific vocabulary is hard-coded here.
 *
 * [ ] Lexical behavior is deterministic.
 *
 * [ ] Positive lexer tests pass.
 *
 * [ ] Parser integration tests pass.
 *
 * [ ] Negative parser-boundary tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Rust 1.97 / 1.97.1 generated-code integration passes.
 *
 * [ ] Safe-Rust requirements remain satisfied.
 *
 * [ ] Existing annotation syntax remains compatible unless an explicit
 *     language-version migration says otherwise.
 *
 * ============================================================================
 */

lexer grammar ZamaniAnnotations;


/* ============================================================================
 * ANNOTATION MARKER
 * ============================================================================
 *
 * The marker is deliberately isolated from the annotation name.
 *
 *     @foo
 *
 * becomes:
 *
 *     AT
 *     IDENTIFIER
 *
 * rather than one combined token.
 *
 * This keeps annotation syntax compositional and extensible.
 */
AT
    : '@'
    ;