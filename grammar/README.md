# Zenith ANTLR4 Grammar

`Zenith.g4` is a standalone ANTLR4 grammar for the Zenith language, generated
and validated against the real ZUTC compiler (`src/lexer.rs`, `src/parser.rs`).
It's useful for building IDE plugins, syntax highlighters, or alternative
tooling without reimplementing the hand-written Rust recursive-descent parser.

## Validation

This grammar was checked by actually generating a parser (ANTLR 4.13.1,
Python3 target) and running it against real `.zn` source, then diffing its
accept/reject behavior against the actual `zenith` binary on the same input.
The only remaining known divergence from the reference compiler on tested
inputs is named-argument call syntax (`perform Effect(reason: "noise")`),
which neither implementation currently supports — confirmed by running both
in parallel on `GRAMMAR.md`'s own worked example.

## Generating a parser

```
antlr4 -Dlanguage=Python3 -visitor -o generated grammar/Zenith.g4
antlr4 -Dlanguage=Java    -visitor -o generated grammar/Zenith.g4
```

## Known limitations (match GRAMMAR.md §1.4 / §6)

- `MTS_LITERAL` is modeled but not yet produced by the reference lexer.
- `match` patterns are plain expressions — no structural destructuring yet.
- Postfix `T?` optional-type sugar isn't exposed; use `Optional<T>`/`Result<T,E>`.
- The reference parser treats several reserved keywords (`this`, `self`,
  primitive type names, Zenith-native domain keywords) as valid identifiers
  in identifier-expecting positions (path segments, member names, etc.) —
  modeled here via the `identLike` rule.
