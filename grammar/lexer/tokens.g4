
/*
 * Zamani Programming Language
 *
 * File: grammar/lexer/tokens.g4
 * Grammar: ZamaniTokens
 *
 * Status: CANONICAL LEXICAL COMPOSITION ROOT
 *
 * Normative authority:
 *   grammar/spec/lexical.md
 *   grammar/specification/
 *
 * Token documentation:
 *   grammar/lexer/tokens.md
 *
 * Implementation:
 *   src/lexer.rs
 *   src/parser.rs
 *
 * Public ANTLR boundary:
 *   grammar/antlr/ZamaniLexer.g4
 *
 * Rust baseline:
 *   Rust 1.97.1 or later
 *   Rust 2021
 *   Safe Rust only; unsafe Rust is prohibited.
 *
 * PURPOSE
 *
 * Assemble the independently owned lexical components into one vocabulary.
 *
 * This file owns:
 *   - the ZamaniTokens grammar identity;
 *   - lexical component composition;
 *   - the canonical import boundary;
 *   - the lexical dependency direction.
 *
 * This file does not own:
 *   - individual token spellings;
 *   - keyword definitions;
 *   - operator definitions;
 *   - punctuation definitions;
 *   - identifier rules;
 *   - literal rules;
 *   - comment rules;
 *   - whitespace rules;
 *   - error-sentinel rules;
 *   - AST or parser productions;
 *   - type or domain semantics;
 *   - hardware discovery or resource allocation;
 *   - IR definitions or target realization.
 *
 * TOKEN OWNERSHIP
 *
 * Every emitted token MUST have exactly one defining rule in the
 * composed lexer.
 *
 * Component grammars MUST NOT independently define the same token
 * name or claim the same source spelling with conflicting meanings.
 *
 * EXTENSIBILITY
 *
 * Quantum operation names, vendor names, device names, capability
 * names, and dialect-provided names remain extensible identifiers
 * unless a separately approved language specification explicitly
 * reserves a spelling.
 *
 * SCALABILITY
 *
 * This grammar defines no finite limit on:
 *   - source program size;
 *   - resource quantities;
 *   - qubit counts;
 *   - processor counts;
 *   - memory capacity;
 *   - tensor dimensions;
 *   - device counts;
 *   - distributed topology size.
 *
 * Actual implementation limits, resource feasibility, numeric
 * representation, and execution capacity are handled by their
 * appropriate downstream layers.
 *
 * SAFETY
 *
 * This file contains grammar source only. It embeds no Rust actions,
 * unsafe code, target-specific logic, or runtime side effects.
 *
 * INTEGRATION CONTRACT
 *
 *   Zamani source
 *       |
 *       v
 *   ZamaniLexer.g4
 *       |
 *       v
 *   lexer.g4
 *       |
 *       v
 *   tokens.g4
 *       |
 *       +--> keywords.g4
 *       +--> operators.g4
 *       +--> punctuation.g4
 *       +--> identifiers.g4
 *       +--> literals.g4
 *       |      |
 *       |      +--> numeric-literals.g4
 *       |      +--> string-literals.g4
 *       |      +--> character-literals.g4
 *       |      +--> boolean-literals.g4
 *       |      +--> quantum-literals.g4
 *       |      +--> hardware-literals.g4
 *       |      +--> duration-literals.g4
 *       |      +--> size-literals.g4
 *       +--> annotations.g4
 *       +--> comments.g4
 *       +--> whitespace.g4
 *       +--> lexer-errors.g4
 *
 * Parser grammars consume the generated ZamaniLexer vocabulary.
 *
 * The canonical AST, semantic model, and domain IRs remain downstream.
 */

lexer grammar ZamaniTokens;

import
    ZamaniKeywords,
    ZamaniOperators,
    ZamaniPunctuation,
    ZamaniIdentifiers,
    ZamaniLiterals,
    ZamaniAnnotations,
    ZamaniComments,
    ZamaniWhitespace,
    ZamaniLexerErrors
    ;
