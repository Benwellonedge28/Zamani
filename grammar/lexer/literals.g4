/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/literals.g4
 *
 * Role:
 *     Canonical aggregation boundary for Zamani literal lexing.
 *
 * Status:
 *     Production lexical architecture.
 *
 * Language:
 *     Zamani
 *
 * Lexer technology:
 *     ANTLR
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no Rust code and introduces no unsafe behavior.
 *     The Zamani compiler/runtime implementation MUST use safe Rust only.
 *
 * ============================================================================
 *
 * ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * This file owns the lexical aggregation of literal families.
 *
 * It intentionally does NOT duplicate the implementation of individual
 * literal families.
 *
 * Specialized literal syntax belongs to:
 *
 *     numeric-literals.g4
 *     string-literals.g4
 *     character-literals.g4
 *     boolean-literals.g4
 *     quantum-literals.g4
 *     hardware-literals.g4
 *     duration-literals.g4
 *     size-literals.g4
 *
 * This file composes those families into the literal layer consumed by the
 * canonical Zamani lexer.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *
 *   - literal-family composition;
 *   - the dependency boundary between the canonical lexer and specialized
 *     literal grammars;
 *   - the guarantee that literal categories are assembled consistently;
 *   - documentation of literal-layer ownership;
 *   - literal-layer extensibility.
 *
 * DOES NOT OWN:
 *
 *   - keywords;
 *   - identifiers;
 *   - operators;
 *   - punctuation;
 *   - comments;
 *   - whitespace;
 *   - Unicode identifier policy;
 *   - semantic typing;
 *   - constant folding;
 *   - numeric overflow handling;
 *   - target machine widths;
 *   - memory capacity;
 *   - quantum resource limits;
 *   - physical qubit counts;
 *   - hardware capabilities;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - runtime execution;
 *   - canonical IR.
 *
 * ============================================================================
 *
 * LEXICAL PIPELINE
 * ============================================================================
 *
 *     source text
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *      keywords             literals
 *                               |
 *                    +----------+----------+
 *                    |          |          |
 *                    v          v          v
 *                 numeric    textual    domain
 *                    |          |          |
 *                    v          v          v
 *                 integer   string/char  quantum/etc.
 *
 * The parser consumes tokens produced by ZamaniLexer.
 *
 * No parser grammar should directly depend on this file.
 *
 * ============================================================================
 *
 * LITERAL PRINCIPLE
 * ============================================================================
 *
 * A literal describes a source-level value representation.
 *
 * A literal MUST NOT silently encode properties of the machine on which the
 * program happens to execute.
 *
 * Therefore this layer MUST NOT impose:
 *
 *     maximum integer width
 *     maximum floating-point precision
 *     maximum string length
 *     maximum array size
 *     maximum quantum-state size
 *     maximum resource quantity
 *     maximum hardware size
 *     maximum memory size
 *     maximum tensor dimension
 *
 * Such limits belong to semantic analysis, resource analysis, compilation,
 * runtime capabilities, or target-specific validation.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Literal syntax participates in:
 *
 *     Program_Once
 *       ->
 *     Compile_Once
 *       ->
 *     Run_Everywhere
 *       ->
 *     Run_Anywhere
 *       ->
 *     Run_Forever
 *
 * A source literal therefore represents source semantics rather than a
 * particular machine representation.
 *
 * For example:
 *
 *     42
 *
 * is a source integer literal.
 *
 * It MUST NOT mean:
 *
 *     32-bit integer
 *     64-bit integer
 *     native CPU integer
 *
 * unless a later semantic/type rule explicitly establishes that meaning.
 *
 * Likewise:
 *
 *     1.0
 *
 * MUST NOT lexically imply a particular hardware floating-point format.
 *
 * ============================================================================
 *
 * QUANTUM PORTABILITY
 * ============================================================================
 *
 * Quantum literals belong to the quantum literal grammar.
 *
 * This aggregation layer does not define:
 *
 *     - number of qubits;
 *     - physical qubits;
 *     - logical qubits;
 *     - topology;
 *     - gate set;
 *     - backend;
 *     - simulator;
 *     - QPU;
 *     - native instruction set;
 *     - numerical precision;
 *     - error model.
 *
 * Quantum syntax is later lowered through semantic analysis into the
 * repository's canonical quantum::ir boundary.
 *
 * This grammar MUST NOT create a second quantum IR.
 *
 * ============================================================================
 *
 * HARD-CODING POLICY
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_INTEGER_BITS
 *     MAX_FLOAT_BITS
 *     MAX_STRING_LENGTH
 *     MAX_QUANTUM_STATE_SIZE
 *     MAX_QUBITS
 *     MAX_REGISTER_SIZE
 *     MAX_TENSOR_RANK
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_RESOURCES
 *
 * No finite machine-dependent scalability ceiling belongs here.
 *
 * ============================================================================
 *
 * ANTLR INTEGRATION
 * ============================================================================
 *
 * This file is intentionally a lexer grammar.
 *
 * It is imported by the canonical lexer assembly:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The canonical lexer remains the only lexer exposed to parser grammars.
 *
 * Parser grammars continue to use:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * They MUST NOT use:
 *
 *     tokenVocab = ZamaniLiterals;
 *
 * ============================================================================
 *
 * SPECIALIZED LITERAL OWNERSHIP
 * ============================================================================
 *
 * Numeric literals:
 *
 *     numeric-literals.g4
 *
 * Owns integer and floating-point lexical forms.
 *
 * String literals:
 *
 *     string-literals.g4
 *
 * Owns string syntax and string escape representation.
 *
 * Character literals:
 *
 *     character-literals.g4
 *
 * Owns character literal syntax.
 *
 * Boolean literals:
 *
 *     boolean-literals.g4
 *
 * Owns true/false lexical forms if they are represented as literal tokens.
 *
 * Quantum literals:
 *
 *     quantum-literals.g4
 *
 * Owns source-level quantum-state literal syntax.
 *
 * Hardware literals:
 *
 *     hardware-literals.g4
 *
 * Owns hardware/resource literal notation where such notation is genuinely
 * lexical rather than semantic.
 *
 * Duration literals:
 *
 *     duration-literals.g4
 *
 * Owns duration/time literal syntax.
 *
 * Size literals:
 *
 *     size-literals.g4
 *
 * Owns source-level size/unit literal syntax.
 *
 * ============================================================================
 *
 * IMPORTANT: NO DUPLICATION
 * ============================================================================
 *
 * This file MUST NOT redefine tokens owned by those specialized grammars.
 *
 * For example, this file MUST NOT contain another:
 *
 *     INTEGER
 *     FLOAT
 *     STRING
 *     CHAR
 *     TRUE
 *     FALSE
 *     QUANTUM_LITERAL
 *
 * rule.
 *
 * Those rules have one owner each.
 *
 * Duplication would create:
 *
 *     ambiguous token ownership
 *     token-number instability
 *     maintenance divergence
 *     inconsistent lexical behavior
 *     difficult ANTLR import behavior
 *
 * ============================================================================
 *
 * IMPORT CONTRACT
 * ============================================================================
 */

lexer grammar ZamaniLiterals;

/*
 * ============================================================================
 * LITERAL FAMILY COMPOSITION
 * ============================================================================
 *
 * Each imported grammar owns one literal family.
 *
 * Keep this list synchronized with the canonical grammar tree.
 *
 * Do not place machine-specific literal grammars here.
 *
 * Hardware literal syntax remains source-level notation; hardware capability
 * validation belongs outside the lexer.
 * ============================================================================
 */

import
    ZamaniNumericLiterals,
    ZamaniStringLiterals,
    ZamaniCharacterLiterals,
    ZamaniBooleanLiterals,
    ZamaniQuantumLiterals,
    ZamaniHardwareLiterals,
    ZamaniDurationLiterals,
    ZamaniSizeLiterals
    ;

/*
 * ============================================================================
 * NO LOCAL TOKEN RULES
 * ============================================================================
 *
 * Literal token rules are intentionally not defined here.
 *
 * This file is an aggregation boundary.
 *
 * That makes each specialized grammar independently completable while
 * preserving one lexical owner for every token.
 * ============================================================================
 */