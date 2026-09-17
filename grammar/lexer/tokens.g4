/**
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/tokens.g4
 *
 * Role:
 *     CANONICAL LEXICAL COMPOSITION ROOT / TOKEN-VOCABULARY BOUNDARY
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
 * Edition:
 *     Rust 2021
 *
 * Safety:
 *     The Zamani compiler/runtime implementation MUST use safe Rust only.
 *     No `unsafe` Rust is required or permitted by the language architecture.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file is the single composition boundary for the modular lexical
 * components under:
 *
 *     grammar/lexer/
 *
 * It does NOT duplicate the lexical rules owned by those components.
 *
 * Instead, it assembles the complete Zamani lexical vocabulary into one
 * ANTLR lexer grammar that can be consumed by the canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The resulting token stream is consumed by the parser and subsequently
 * lowered through:
 *
 *     source
 *       |
 *       v
 *     lexical tokens
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> classical semantics / IR
 *       +--> quantum semantics / quantum::ir
 *       +--> HDL / hardware semantics
 *       +--> resource/capability semantics
 *       +--> distributed/data/AI/network/security semantics
 *       |
 *       v
 *     canonical IR / semantic representations
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
 *     HAL / target realization / runtime
 *
 * This file has NO dependency on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     HAL
 *     routing
 *     scheduling
 *     calibration
 *     optimization
 *     runtime
 *     deployment
 *     physical hardware
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - lexical composition;
 *     - lexical-component import order;
 *     - the single composed token vocabulary boundary;
 *     - prevention of independent competing lexer composition;
 *     - compatibility-oriented token assembly;
 *     - the integration boundary between modular lexical grammars and the
 *       canonical Zamani lexer.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - individual keyword spellings;
 *     - individual operator spellings;
 *     - individual punctuation spellings;
 *     - identifier syntax;
 *     - numeric literal syntax;
 *     - string syntax;
 *     - character syntax;
 *     - boolean literal syntax;
 *     - quantum literal syntax;
 *     - hardware literal syntax;
 *     - duration syntax;
 *     - size syntax;
 *     - comments;
 *     - whitespace;
 *     - Unicode character classes;
 *     - annotation semantics;
 *     - AST construction;
 *     - semantic analysis;
 *     - type checking;
 *     - resource discovery;
 *     - hardware discovery;
 *     - target selection;
 *     - quantum operation resolution;
 *     - quantum gate enumeration;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - runtime behavior.
 *
 * ============================================================================
 *
 * SINGLE LEXER AUTHORITY
 * ============================================================================
 *
 * There MUST be exactly one production lexer composition.
 *
 * The intended architecture is:
 *
 *     grammar/lexer/*.g4
 *             |
 *             v
 *     grammar/lexer/tokens.g4
 *             |
 *             v
 *     grammar/antlr/ZamaniLexer.g4
 *             |
 *             v
 *     parser grammar
 *
 * `tokens.g4` MUST NOT be compiled and selected at runtime as an alternative
 * lexer to `ZamaniLexer.g4`.
 *
 * Instead, `ZamaniLexer.g4` must import this composition boundary.
 *
 * The canonical integration form is:
 *
 *     lexer grammar ZamaniLexer;
 *
 *     import ZamaniTokens;
 *
 *     // Any remaining local rules must be limited to deliberately documented
 *     // canonical overrides. Duplicating imported lexical rules is forbidden.
 *
 * The existing monolithic rules currently present in
 * `grammar/antlr/ZamaniLexer.g4` must therefore eventually be removed or
 * replaced by this composition boundary as part of the canonical lexer
 * migration.
 *
 * ============================================================================
 *
 * WHY THIS FILE DOES NOT REDEFINE TOKENS
 * ============================================================================
 *
 * ANTLR lexer composition supports imported lexer grammars.
 *
 * Re-declaring a token here would create another lexical owner and would
 * eventually produce divergence between:
 *
 *     tokens.g4
 *     keywords.g4
 *     operators.g4
 *     punctuation.g4
 *     literals.g4
 *     identifiers.g4
 *     comments.g4
 *     annotations.g4
 *     ZamaniLexer.g4
 *     src/lexer.rs
 *
 * That is precisely the competing-lexer problem this architecture is intended
 * to eliminate.
 *
 * Therefore:
 *
 *     ONE TOKEN SPELLING
 *          ->
 *     ONE LEXICAL OWNER
 *          ->
 *     ONE CANONICAL COMPOSED LEXER
 *          ->
 *     ONE TOKEN STREAM
 *
 * ============================================================================
 *
 * IMPORT ORDER
 * ============================================================================
 *
 * Import order is deliberate.
 *
 * 1. Literals first
 *
 *    `ZamaniLiterals` imports:
 *
 *       ZamaniNumericLiterals
 *       ZamaniStringLiterals
 *       ZamaniCharacterLiterals
 *       ZamaniBooleanLiterals
 *       ZamaniQuantumLiterals
 *       ZamaniHardwareLiterals
 *       ZamaniDurationLiterals
 *       ZamaniSizeLiterals
 *
 *    This makes the specialized literal grammars the first lexical owners of
 *    literal forms such as TRUE and FALSE.
 *
 * 2. Annotations second
 *
 *    `ZamaniAnnotations` owns AT.
 *
 *    This deliberately resolves the existing duplicated AT ownership between
 *    annotations.g4 and punctuation.g4 in favor of the dedicated annotation
 *    component.
 *
 * 3. Keywords third
 *
 *    Keywords must precede the general identifier rule in the final composed
 *    lexer.
 *
 * 4. Operators fourth
 *
 *    Compound operators are assembled before structural punctuation.
 *
 * 5. Punctuation fifth
 *
 *    Punctuation owns only its documented structural spellings.
 *
 * 6. Identifiers sixth
 *
 *    The general IDENTIFIER rule must not steal reserved keywords.
 *
 * 7. Comments seventh
 *
 *    Comments are hidden lexical material and must not become ordinary parser
 *    syntax.
 *
 * 8. Lexical-error sentinels last
 *
 *    They classify malformed constructs that cannot be recognized as valid
 *    lexical forms.
 *
 * IMPORTANT:
 *
 * ANTLR's longest-match behavior remains the primary lexical rule.
 * Where equal-length imported rules overlap, import order provides the
 * deterministic tie-breaker.
 *
 * ============================================================================
 *
 * COMPONENT OWNERSHIP MATRIX
 * ============================================================================
 *
 * Component                         Owns
 * ---------------------------------------------------------------------------
 *
 * ZamaniLiterals                    Literal-family aggregation
 *
 * ZamaniNumericLiterals             Integer/floating numeric syntax
 *
 * ZamaniStringLiterals              String syntax
 *
 * ZamaniCharacterLiterals           Character syntax
 *
 * ZamaniBooleanLiterals             true / false
 *
 * ZamaniQuantumLiterals             |0⟩ / |1⟩ / |+⟩ / |-⟩ and approved
 *                                   quantum-state literal forms
 *
 * ZamaniHardwareLiterals            Source-level hardware/resource literals
 *
 * ZamaniDurationLiterals            Duration/time literals
 *
 * ZamaniSizeLiterals                Size/quantity literals
 *
 * ZamaniAnnotations                 @ marker
 *
 * ZamaniKeywords                    Reserved/contextual/compatibility words
 *
 * ZamaniOperators                   Operators
 *
 * ZamaniPunctuation                 Structural punctuation
 *
 * ZamaniIdentifiers                 IDENTIFIER and Unicode identifier classes
 *
 * ZamaniComments                    Comments, documentation comments,
 *                                   whitespace handling
 *
 * ZamaniLexerErrors                 Deterministic malformed-construct
 *                                   sentinels
 *
 * ============================================================================
 *
 * IMPORTANT LEXICAL CORRECTIONS
 * ============================================================================
 *
 * The following legacy concepts are deliberately NOT reintroduced here as
 * independent lexical owners:
 *
 *     Arrow
 *     ThinArrow
 *
 * if they represent the same spelling:
 *
 *     ->
 *
 * The canonical lexical spelling is owned by the operator grammar.
 *
 * Likewise:
 *
 *     Question
 *     QuestionMark
 *
 * must not become two independent representations of:
 *
 *     ?
 *
 * unless a future language version explicitly gives them distinct lexical
 * contexts.
 *
 * The same rule applies to every duplicated historical token spelling.
 *
 * The Rust implementation may retain compatibility enum variants temporarily,
 * but the canonical language grammar must have one token identity per
 * language-level lexical concept.
 *
 * ============================================================================
 *
 * LEGACY RUST TOKEN INTEGRATION
 * ============================================================================
 *
 * The current Rust implementation in:
 *
 *     src/lexer.rs
 *
 * contains a historical TokenType vocabulary including:
 *
 *     Identifier
 *     String
 *     Integer
 *     Float
 *     Char
 *     Boolean
 *     QuantumLiteral
 *     NanoAnnotation
 *     MTSLiteral
 *
 * plus Keyword* variants and operator variants.
 *
 * This file deliberately does NOT copy those Rust enum names into a second
 * ANTLR lexer.
 *
 * The canonical migration is:
 *
 *     source spelling
 *          |
 *          v
 *     canonical ANTLR token
 *          |
 *          v
 *     frontend token adapter
 *          |
 *          v
 *     canonical Rust TokenType
 *
 * The Rust adapter is responsible for implementation representation.
 *
 * The grammar remains the language authority.
 *
 * ============================================================================
 *
 * MTS COMPATIBILITY
 * ============================================================================
 *
 * The current Rust implementation contains MTSLiteral.
 *
 * The normative lexical architecture does NOT require an opaque MTSLiteral
 * token.
 *
 * The preferred source architecture is compositional:
 *
 *     mts
 *     [
 *         expression
 *     ]
 *
 * where:
 *
 *     mts       -> keyword/identifier according to the language policy
 *     [         -> LBRACKET
 *     expression -> parser-owned structure
 *     ]         -> RBRACKET
 *
 * This preserves extensibility and prevents timestamps or temporal values
 * from becoming an opaque lexical mini-language.
 *
 * Therefore this file intentionally does not add MTSLiteral as a new lexical
 * rule.
 *
 * Existing Rust compatibility handling belongs in the Rust lexer/token adapter
 * migration.
 *
 * ============================================================================
 *
 * NANO ANNOTATION COMPATIBILITY
 * ============================================================================
 *
 * The current Rust implementation contains NanoAnnotation.
 *
 * The production lexical model is compositional:
 *
 *     AT IDENTIFIER
 *
 * rather than:
 *
 *     NanoAnnotation
 *
 * This prevents:
 *
 *     @atom
 *     @molecule
 *     @qpu
 *     @vendor_specific
 *
 * from becoming an ever-growing list of lexical token types.
 *
 * The annotation component therefore owns only:
 *
 *     AT
 *
 * while the identifier component owns the name.
 *
 * ============================================================================
 *
 * QUANTUM GATE EXTENSIBILITY
 * ============================================================================
 *
 * This composition deliberately does NOT reserve:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     CX
 *     U
 *     RX
 *     RY
 *     RZ
 *     custom_gate
 *     vendor_gate
 *
 * as universal lexical keywords.
 *
 * Quantum operations remain extensible identifiers or parser-level constructs.
 *
 * This is essential for:
 *
 *     current quantum hardware
 *     future quantum hardware
 *     logical operations
 *     custom operations
 *     decomposed operations
 *     vendor operations
 *     calibrated operations
 *     simulator operations
 *
 * without making the lexer depend on a finite gate set.
 *
 * All quantum semantic lowering ultimately crosses the existing:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 *
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file contains NO machine-size limits.
 *
 * In particular, it contains no:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_REGISTER_SIZE
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_VECTOR_WIDTH
 *     MAX_PORTS
 *     MAX_TIMELINES
 *     MAX_PROGRAM_SIZE
 *     MAX_IDENTIFIER_LENGTH
 *     MAX_RESOURCE_COUNT
 *
 * Language validity is independent of target capacity.
 *
 * Therefore the same lexical source model can serve:
 *
 *     atom
 *     embedded
 *     microcontroller
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     cluster
 *     supercomputer
 *     distributed system
 *     cloud
 *     future computational substrates
 *
 * subject only to genuine implementation/resource availability.
 *
 * ============================================================================
 *
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * The lexer MUST NOT recognize physical topology as syntax.
 *
 * These remain ordinary source identifiers/data unless explicitly defined by
 * another lexical contract:
 *
 *     gpu0
 *     cpu0
 *     qpu0
 *     device0
 *     node0
 *     q[0]
 *     q[1]
 *
 * The lexer does not know whether such resources exist.
 *
 * Resource requirements, capabilities, placement, topology, scheduling,
 * routing, calibration and deployment are downstream semantic/runtime
 * concerns.
 *
 * ============================================================================
 *
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The composed lexer must preserve:
 *
 *     token kind
 *     source spelling
 *     source span
 *
 * so that:
 *
 *     diagnostics
 *     formatting
 *     IDE tooling
 *     source maps
 *     provenance
 *     semantic analysis
 *
 * can recover exact source information.
 *
 * No lexical component may silently normalize Unicode.
 *
 * ============================================================================
 *
 * UNICODE
 * ============================================================================
 *
 * `ZamaniUnicode` is intentionally NOT imported here as a token-producing
 * grammar.
 *
 * Its rules are reusable Unicode fragments.
 *
 * The current identifiers.g4 already owns the actual IDENTIFIER token and
 * provides Unicode-capable identifier classes.
 *
 * This avoids creating parser-visible Unicode category tokens.
 *
 * ============================================================================
 *
 * COMMENTS / WHITESPACE
 * ============================================================================
 *
 * Comments and whitespace remain owned by ZamaniComments.
 *
 * They MUST NOT be duplicated in this file.
 *
 * Documentation comments may be retained on a hidden channel for tooling.
 *
 * Ordinary comments and whitespace remain hidden/skipped according to the
 * canonical comment specification.
 *
 * ============================================================================
 *
 * ERROR RECOVERY
 * ============================================================================
 *
 * ZamaniLexerErrors provides specific malformed-construct sentinels such as:
 *
 *     UNTERMINATED_DOC_BLOCK_COMMENT
 *     UNTERMINATED_BLOCK_COMMENT
 *     UNTERMINATED_STRING
 *     UNTERMINATED_CHARACTER
 *
 * These are diagnostic tokens, not valid Zamani syntax.
 *
 * They must not be silently skipped.
 *
 * A generic:
 *
 *     ERROR : . ;
 *
 * rule is deliberately NOT introduced here because it can mask future syntax,
 * interfere with lexical diagnostics, and accidentally turn unsupported source
 * into apparently valid tokens.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * This file creates no AST.
 *
 * The parser maps canonical lexical tokens into the existing domain-neutral
 * AST architecture:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> frontend AST
 *       -> semantic model
 *       -> canonical IR
 *
 * The AST remains independent of:
 *
 *     LLVM
 *     QIR
 *     MLIR
 *     vendor backends
 *     physical topology
 *     QEC
 *     routing
 *     calibration
 *
 * ============================================================================
 *
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum lexical forms are syntax only.
 *
 * This file must never introduce:
 *
 *     QuantumGate enum
 *     PhysicalQubitId
 *     QubitId
 *     coupling-map tokens
 *     calibration tokens
 *     hardware gate sets
 *     QEC implementation tokens
 *
 * Quantum source syntax eventually lowers through:
 *
 *     quantum::ir
 *
 * and then through:
 *
 *     optimization
 *     decomposition
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     target realization
 *
 * ============================================================================
 *
 * CLASSICAL / HDL / HYBRID CONTRACT
 * ============================================================================
 *
 * The same lexical vocabulary is shared by:
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
 *     embedded
 *     systems
 *     accelerator
 *
 * domain grammars.
 *
 * Domain grammars MUST NOT create independent lexers.
 *
 * Domain-specific names should remain identifiers unless a genuine lexical
 * distinction is required by the language specification.
 *
 * ============================================================================
 *
 * RUST 1.97 / 1.97.1 CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust.
 *
 * Generated/compiler integration MUST:
 *
 *     - compile with Rust 1.97;
 *     - compile with Rust 1.97.1;
 *     - remain Rust 2021 compatible;
 *     - contain no unsafe Rust;
 *     - preserve source spans;
 *     - preserve deterministic token ordering;
 *     - avoid machine-dependent lexical behavior.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * For identical:
 *
 *     source
 *     language version
 *     lexical configuration
 *
 * the canonical lexer must emit the same token sequence.
 *
 * Tokenization must not depend upon:
 *
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     FPGA availability
 *     memory capacity
 *     network state
 *     filesystem state
 *     scheduler state
 *     calibration state
 *     runtime state
 *     random state
 *     hash iteration order
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] Every lexical component has exactly one composition path.
 *     [x] No token rule is duplicated here.
 *     [x] keywords.g4 remains the keyword owner.
 *     [x] identifiers.g4 remains the identifier owner.
 *     [x] literals.g4 remains the literal aggregation owner.
 *     [x] operators.g4 remains the operator owner.
 *     [x] punctuation.g4 remains the punctuation owner.
 *     [x] annotations.g4 remains the annotation-marker owner.
 *     [x] comments.g4 remains the comment/whitespace owner.
 *     [x] lexer-errors.g4 remains the malformed-lexeme owner.
 *     [x] No machine limits are encoded.
 *     [x] No quantum gate list is hard-coded.
 *     [x] No hardware topology is hard-coded.
 *     [x] No QEC/ZQN/HAL behavior is encoded.
 *     [x] No Rust or unsafe code is embedded.
 *     [x] Canonical lexer integration is deterministic.
 *     [x] Parser grammars consume the composed canonical lexer.
 *     [x] quantum::ir remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 *
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * The canonical lexer MUST import this grammar exactly once:
 *
 *     lexer grammar ZamaniLexer;
 *
 *     import ZamaniTokens;
 *
 * Parser grammars MUST continue to consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * They MUST NOT independently consume:
 *
 *     tokenVocab = ZamaniKeywords;
 *     tokenVocab = ZamaniOperators;
 *     tokenVocab = ZamaniLiterals;
 *     tokenVocab = ZamaniIdentifiers;
 *
 * Domain grammars MUST NOT define lexer rules for domain-specific constructs
 * when an existing universal lexical token is sufficient.
 *
 * ============================================================================
 */

lexer grammar ZamaniTokens;


/*
 * ============================================================================
 * MODULAR LEXER COMPOSITION
 * ============================================================================
 *
 * The import graph deliberately follows lexical ownership.
 *
 * IMPORTANT:
 *
 *     ZamaniLiterals
 *
 * is imported as an aggregation boundary and therefore already imports:
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
 * Those grammars MUST NOT also be imported directly here.
 *
 * Doing so would create duplicate import paths and potentially duplicate
 * generated delegates.
 *
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