/**
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/tokens.g4
 *
 * Role:
 *     CANONICAL LEXICAL VOCABULARY / LEXER COMPOSITION ROOT
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     Rust 2021
 *
 * Safety:
 *     Safe Rust only.
 *
 *     The Zamani handwritten compiler/runtime implementation MUST NOT require
 *     Rust `unsafe`.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This grammar is the single composition boundary for the complete Zamani
 * lexical vocabulary.
 *
 * It assembles the independently owned lexical grammars under:
 *
 *     grammar/lexer/
 *
 * into one canonical ANTLR lexer vocabulary.
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * which MUST become an orchestration-only lexer after migration.
 *
 * The architecture is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     canonical token vocabulary
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> classical semantics / IR
 *       +--> quantum semantics / quantum::ir
 *       +--> HDL / hardware semantics
 *       +--> AI / data semantics
 *       +--> distributed semantics
 *       +--> networking semantics
 *       +--> security semantics
 *       |
 *       v
 *     canonical semantic representations / IR
 *       |
 *       v
 *     optimization
 *       |
 *       +--> routing
 *       +--> scheduling
 *       +--> resilience
 *       +--> QEC
 *       +--> ZQN
 *       |
 *       v
 *     HAL / backend / target realization
 *
 * ============================================================================
 *
 * SINGLE LEXER AUTHORITY
 * ============================================================================
 *
 * There MUST be exactly one production lexical vocabulary.
 *
 * This file is the composition root.
 *
 * Individual lexical meanings are owned by their specialized files.
 *
 * Therefore:
 *
 *     ONE lexical concept
 *          |
 *          v
 *     ONE lexical owner
 *          |
 *          v
 *     ONE canonical token identity
 *          |
 *          v
 *     ONE composed vocabulary
 *          |
 *          v
 *     ONE production lexer
 *
 * ============================================================================
 *
 * IMPORTANT
 * ============================================================================
 *
 * This file intentionally contains NO ordinary lexer rules.
 *
 * Do NOT add rules such as:
 *
 *     FN : 'fn' ;
 *     IDENTIFIER : ... ;
 *     PLUS : '+' ;
 *
 * here.
 *
 * Those rules belong to their authoritative lexical components.
 *
 * Keeping them here would create a second owner and eventually cause lexical
 * divergence.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - lexical composition;
 *     - delegate/import ordering;
 *     - the canonical token-vocabulary boundary;
 *     - prevention of competing lexer compositions;
 *     - compatibility composition;
 *     - integration between modular lexer components and ZamaniLexer.g4.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - keyword spellings;
 *     - operators;
 *     - punctuation;
 *     - identifiers;
 *     - literals;
 *     - comments;
 *     - whitespace;
 *     - Unicode classes;
 *     - annotations;
 *     - diagnostics;
 *     - AST nodes;
 *     - semantic analysis;
 *     - type checking;
 *     - resource discovery;
 *     - capability discovery;
 *     - hardware discovery;
 *     - target selection;
 *     - quantum semantics;
 *     - quantum gate sets;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - runtime behavior.
 *
 * ============================================================================
 *
 * IMPORT ARCHITECTURE
 * ============================================================================
 *
 * Literal aggregation comes first.
 *
 * `ZamaniLiterals` already imports the individual literal families:
 *
 *     ZamaniNumericLiterals
 *     ZamaniStringLiterals
 *     ZamaniCharacterLiterals
 *     ZamaniBooleanLiterals
 *     ZamaniQuantumLiterals
 *     ZamaniHardwareLiterals
 *     ZamaniDurationLiterals
 *     ZamaniSizeLiterals
 *
 * Therefore those grammars MUST NOT also be imported directly here.
 *
 * This avoids duplicate delegate paths.
 *
 * ============================================================================
 *
 * COMPONENT OWNERSHIP
 * ============================================================================
 *
 * ZamaniLiterals
 *     Aggregates all literal families.
 *
 * ZamaniAnnotations
 *     Annotation marker and annotation-specific lexical material.
 *
 * ZamaniKeywords
 *     Reserved keyword vocabulary.
 *
 * ZamaniOperators
 *     Operator vocabulary.
 *
 * ZamaniPunctuation
 *     Structural punctuation.
 *
 * ZamaniIdentifiers
 *     IDENTIFIER and Unicode identifier lexical classes.
 *
 * ZamaniComments
 *     Comments and whitespace.
 *
 * ZamaniLexerErrors
 *     Deterministic malformed lexical construct diagnostics.
 *
 * ============================================================================
 *
 * TOKEN IDENTITY POLICY
 * ============================================================================
 *
 * Historical ZamaniLexer.g4 contains several token names which have equivalent
 * canonical names in the modular lexer.
 *
 * They MUST NOT be duplicated.
 *
 * Examples:
 *
 *     ARROW          -> THIN_ARROW
 *     EQ_EQ          -> EQUAL_EQUAL
 *     NOT_EQ         -> NOT_EQUAL
 *     LE             -> LESS_EQUAL
 *     GE             -> GREATER_EQUAL
 *     AND_AND        -> LOGICAL_AND
 *     OR_OR          -> LOGICAL_OR
 *     SHIFT_LEFT     -> LEFT_SHIFT
 *     SHIFT_RIGHT    -> RIGHT_SHIFT
 *     PERCENT_ASSIGN -> canonical assignment token
 *     PERCENT        -> MODULO
 *
 * The canonical token identity is the modular vocabulary.
 *
 * Parser grammars MUST migrate to the canonical names instead of causing
 * duplicate lexical spellings.
 *
 * ============================================================================
 *
 * LEGACY TOKEN POLICY
 * ============================================================================
 *
 * The old lexer also contains:
 *
 *     NANO_ANNOTATION
 *     MTS_LITERAL
 *
 * These are intentionally NOT reintroduced as ordinary canonical lexical
 * tokens.
 *
 * NANO_ANNOTATION
 * ----------------
 *
 * Historical form:
 *
 *     @identifier
 *
 * Production lexical architecture:
 *
 *     AT IDENTIFIER
 *
 * Annotation semantics are resolved by the parser/AST/semantic layers.
 *
 * This keeps arbitrary future annotations extensible.
 *
 *
 * MTS_LITERAL
 * -----------
 *
 * Historical form:
 *
 *     mts[...]
 *
 * Production architecture treats MTS as compositional syntax:
 *
 *     MTS
 *     LBRACKET
 *     ...
 *     RBRACKET
 *
 * The contents are therefore available to the parser and semantic layer
 * instead of being hidden inside an opaque lexical token.
 *
 * This is necessary for extensible temporal/multi-timeline syntax.
 *
 * ============================================================================
 *
 * QUANTUM EXTENSIBILITY
 * ============================================================================
 *
 * This vocabulary MUST NOT reserve a finite list of quantum operations.
 *
 * Do not add universal lexer keywords for:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CX
 *     CNOT
 *     RX
 *     RY
 *     RZ
 *     U
 *     vendor gates
 *     custom gates
 *
 * unless a future specification explicitly establishes a lexical reason.
 *
 * Quantum operations remain extensible source-level names/operations.
 *
 * The semantic pipeline remains:
 *
 *     Zamani source
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic quantum operation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL / target
 *
 * ============================================================================
 *
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This vocabulary MUST NOT encode artificial machine limits.
 *
 * No lexical rule may establish:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_TIMELINES
 *     MAX_PROGRAM_SIZE
 *
 * Program size and resource scale are limited only by the actual compiler,
 * runtime and available resources, not by language-level artificial constants.
 *
 * ============================================================================
 *
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * The lexer does not know whether:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     node0
 *     device0
 *     accelerator0
 *
 * actually exists.
 *
 * Such spellings are identifiers unless the language specification explicitly
 * assigns another lexical meaning.
 *
 * Hardware discovery belongs downstream.
 *
 * ============================================================================
 *
 * RESOURCE INDEPENDENCE
 * ============================================================================
 *
 * These are semantic concepts, not lexical limits:
 *
 *     resource
 *     capability
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capacity
 *     availability
 *
 * The lexer merely recognizes the language vocabulary.
 *
 * It does not determine whether a resource is available.
 *
 * ============================================================================
 *
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The canonical lexer/parser pipeline must preserve:
 *
 *     token kind
 *     source spelling
 *     source location/span
 *
 * so downstream systems can provide:
 *
 *     diagnostics
 *     source maps
 *     IDE support
 *     formatting
 *     provenance
 *     semantic diagnostics
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * This composition layer contains:
 *
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment access;
 *     - no hardware discovery;
 *     - no randomness;
 *     - no target-dependent decisions.
 *
 * The same source and same lexical vocabulary therefore produce the same
 * lexical classification.
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * Lexical recognition must be side-effect free.
 *
 * The lexer MUST NOT:
 *
 *     - execute source code;
 *     - execute commands;
 *     - open files;
 *     - contact networks;
 *     - inspect credentials;
 *     - discover hardware;
 *     - modify files;
 *     - invoke a backend.
 *
 * ============================================================================
 *
 * RUST INTEGRATION
 * ============================================================================
 *
 * This grammar itself contains no Rust.
 *
 * Generated ANTLR Rust code is consumed by the Zamani frontend.
 *
 * Handwritten Zamani compiler/runtime code MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST NOT require `unsafe`.
 *
 * Rust token enums/adapters are implementation representations.
 *
 * They MUST NOT become a second language-level lexical authority.
 *
 * ============================================================================
 *
 * PARSER INTEGRATION
 * ============================================================================
 *
 * The production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * and it MUST consume this vocabulary.
 *
 * Parser grammars must consume the resulting canonical vocabulary rather than
 * referring to the legacy token names directly.
 *
 * ============================================================================
 *
 * REQUIRED ZAMANI LEXER ORCHESTRATOR
 * ============================================================================
 *
 * After this migration, grammar/antlr/ZamaniLexer.g4 should contain only the
 * orchestration boundary and any explicitly justified lexer-level integration
 * required by ANTLR.
 *
 * Its conceptual form is:
 *
 *     lexer grammar ZamaniLexer;
 *
 *     import ZamaniTokens;
 *
 * No historical duplicated keyword/operator/punctuation/literal definitions
 * should remain there.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * tokens.g4 is complete when:
 *
 *   [x] It is the sole canonical lexical composition boundary.
 *   [x] It imports every authoritative lexical family.
 *   [x] Literal families are imported exactly once through ZamaniLiterals.
 *   [x] Keywords have one lexical owner.
 *   [x] Operators have one lexical owner.
 *   [x] Punctuation has one lexical owner.
 *   [x] Identifiers have one lexical owner.
 *   [x] Comments have one lexical owner.
 *   [x] Diagnostics have one lexical owner.
 *   [x] No token spelling is duplicated here.
 *   [x] No target-specific token limits exist.
 *   [x] No quantum hardware limits exist.
 *   [x] No fixed resource counts exist.
 *   [x] No semantic actions exist.
 *   [x] No Rust `unsafe` is required.
 *   [x] Legacy token aliases are migrated deliberately rather than duplicated.
 *   [x] ZamaniLexer.g4 can become an orchestration-only lexer.
 *   [x] Parser grammars can consume one canonical vocabulary.
 *   [x] The vocabulary remains reusable across classical, quantum, hybrid,
 *       HDL, hardware, AI, data, distributed, networking, security and future
 *       domains.
 *
 * ============================================================================
 */

lexer grammar ZamaniTokens;


/*
 * ============================================================================
 * CANONICAL LEXICAL COMPONENTS
 * ============================================================================
 *
 * Import order is deliberate.
 *
 * Literal aggregation must occur before the general identifier vocabulary so
 * literal families retain their canonical token identities.
 *
 * Keywords must remain distinguishable from IDENTIFIER.
 *
 * Operators must be represented by the canonical operator vocabulary.
 *
 * Punctuation remains structurally separate from operators.
 *
 * Identifiers remain the general fallback name vocabulary.
 *
 * Comments and whitespace are hidden lexical material.
 *
 * Lexer-error tokens remain available for diagnostics.
 * ============================================================================
 */

import
    ZamaniLiterals,
    ZamaniAnnotations,
    ZamaniKeywords,
    ZamaniOperators,
    ZamaniPunctuation,
    ZamaniIdentifiers,
    ZamaniComments,
    ZamaniLexerErrors
    ;