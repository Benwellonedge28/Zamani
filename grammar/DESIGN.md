
github.com/Benwellonedge28/Zamani/grammar/DESIGN.md

Zamani Grammar Design

Status: Normative production design
Language: Zamani
Compiler: ZUTC
Minimum Rust: Rust 1.97 / 1.97.1
Safety: Rust "unsafe" is forbidden
Primary goal: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

1. Purpose

This document defines the production architecture for the Zamani language grammar and its relationship to the lexer, parser, AST, semantic analysis, canonical quantum IR, optimization, scheduling, ZQN, and target backends.

The grammar is a language boundary, not a hardware description. Zamani source expresses computation, intent, types, effects, resource requirements, temporal constraints, quantum semantics, and other portable meaning.

Machine-specific facts such as qubit count, topology, native gate set, instruction width, CPU ISA, accelerator layout, pulse constraints, memory capacity, and device calibration are supplied by later compilation stages and execution contexts.

The architecture must support programs from the smallest representable computation to arbitrarily large computations bounded only by available resources, implementation limits, addressable representations, and explicit resource budgets.

No source-level construct may encode an artificial fixed machine size.

2. Repository Reality and Authority

The repository currently contains three important grammar/specification surfaces:

- "grammar/grammar.md": implementation-conformance reference derived from the current lexer/parser/AST implementation.
- "grammar/Zamani.g4": ANTLR4 grammar containing a broad language surface, including mathematics, quantum, nano, effects, OOP, modules, and other domains.
- "grammar/Zamani-Grammar.md": broad NIMBUS/Universal-Trinity language design document containing additional aspirational constructs.

They must not remain independent language authorities.

The production authority is:

1. Normative language specification — canonical language rules.
2. Lexer/parser implementation — conforms to those rules.
3. AST — represents every accepted semantic construct without lossy conversion.
4. Semantic analysis — defines typing, effects, ownership/resource rules, capability requirements, and semantic validity.
5. Canonical IR — defines target-independent executable meaning.
6. Backends — define target realization.

"grammar/grammar.md" remains useful as an implementation snapshot, but must be generated or validated against the canonical specification rather than maintained as a competing grammar.

"Zamani.g4" is the canonical ANTLR representation after reconciliation with the Rust frontend.

"Zamani-Grammar.md" is retained as historical/design material unless individual constructs are formally promoted through the language-evolution process.

The repository's implementation-oriented grammar already explicitly identifies "src/lexer.rs", "src/parser.rs", "src/ast/mod.rs", "src/semantic.rs", and "src/ir_gen.rs" as the relevant implementation boundaries.

3. Production Invariants

Every grammar change must preserve:

- One Zamani language.
- No accidental dialects caused by frontend differences.
- No syntax accepted by one canonical frontend and rejected by another without an explicit compatibility rule.
- No semantic information discarded between parse, AST, semantic analysis, and IR.
- No hard-coded maximum number of qubits, dimensions, tensor ranks, devices, cores, nodes, memory locations, or other machine quantities.
- No Rust "unsafe".
- Deterministic parsing for identical source and language version.
- Source spans on AST nodes needed for diagnostics.
- Structured diagnostics.
- Explicit resource/capability requirements.
- No arbitrary fixed parser recursion/depth ceilings.
- Versioned syntax and compatibility policy.
- Canonical quantum semantic identity.
- No duplicate frontend quantum IR concepts.

4. Compiler Pipeline Contract

Source
  ↓
Source Map / File Identity
  ↓
Lexer
  ↓
Token Stream
  ↓
Parser
  ↓
Frontend AST
  ↓
Name / Module / Import Resolution
  ↓
Semantic + Type + Effect + Capability Analysis
  ↓
Canonical Semantic IR
  ├── quantum::ir
  ├── classical/control/data IR
  └── effect/resource/temporal metadata
  ↓
Target-Independent Optimization
  ↓
Routing / Scheduling / Resilience / ZQN
  ↓
Target Lowering
  ↓
Simulator / Emulator / CPU / GPU / QPU / Future Target

The frontend must not lower directly to a particular quantum hardware gate set merely because a source operation has a familiar name.

Quantum optimization, routing, scheduling, resilience, ZQN and hardware integration consume the canonical "quantum::ir" boundary.

The repository already documents this separation and explicitly warns against duplicate quantum operation definitions during the structured IR transition.

5. Lexer Architecture

"src/lexer.rs" is the executable lexical implementation. It currently contains a broad token inventory covering core syntax, quantum/nano/Sankofa constructs, advanced system constructs, operators, source spans and lexical errors.

The production lexer must provide:

- UTF-8 source handling.
- Byte-accurate spans.
- Deterministic tokenization.
- Longest-valid-operator matching.
- Explicit whitespace/comment handling.
- Literal validation.
- Structured lexical errors.
- No panics on malformed source.
- No target-dependent behavior.
- One canonical keyword registry.
- No duplicate token meanings.
- Versioned keyword policy.

The current token inventory contains overlapping concepts such as "Question"/"QuestionMark" and "Ampersand"/"BitAnd". These must be normalized rather than allowed to become separate semantic concepts accidentally.

The lexer must not advertise a feature as implemented when it is only declared.

For example, the implementation-oriented grammar records that "MTSLiteral" is declared but not currently emitted by the lexer.

5.1 Canonical lexical classes

Identifier
IntegerLiteral
FloatLiteral
StringLiteral
CharLiteral
BooleanLiteral
NullLiteral
QuantumStateLiteral
AttributeMarker
Operator
Delimiter
Keyword
EOF
Illegal/Error

Domain concepts that do not require lexical distinction should remain identifiers and be resolved semantically.

In particular, quantum gate names must not be hard-coded into the lexer as a finite list.

5.2 Numeric literals

Numeric syntax must not impose machine-size assumptions.

Literal magnitude is checked when converted to a semantic type.

The grammar may support:

- decimal;
- hexadecimal;
- binary;
- octal;
- decimal floating-point;
- future explicitly versioned numeric forms.

Overflow must produce a diagnostic rather than truncation.

5.3 Quantum literals

Quantum state notation such as:

|0⟩
|1⟩
|+⟩
|-⟩

is syntax for semantic state values.

It does not imply a physical representation.

The grammar must never encode a maximum register width.

6. Parser Architecture

"src/parser.rs" currently uses recursive-descent parsing together with Pratt/precedence parsing. Its precedence hierarchy covers assignment, range, logical/bitwise operators, equality, comparison, shifts, arithmetic, prefix operations, calls, indexing and member access.

The production parser must:

- produce canonical AST nodes;
- report recoverable syntax errors;
- always make progress after recovery;
- avoid infinite loops;
- avoid silent acceptance of unsupported syntax;
- preserve useful source context;
- remain deterministic;
- avoid arbitrary fixed nesting limits;
- use explicit worklists where deep generated structures could exhaust the call stack;
- use only safe Rust.

The repository's AST/traversal architecture already emphasizes explicit worklists/stacks for deeply nested or generated structures.

7. Canonical Source Grammar

The canonical high-level grammar is:

Program          ::= Item* EOF

Item             ::= Attribute* Declaration
                   | Statement

Declaration      ::= ModuleDecl
                   | ImportDecl
                   | ExportDecl
                   | FunctionDecl
                   | StructDecl
                   | EnumDecl
                   | TraitDecl
                   | ImplDecl
                   | ClassDecl
                   | InterfaceDecl
                   | RecordDecl
                   | TypeAliasDecl
                   | ConstDecl
                   | QuantumDecl
                   | NanoDecl
                   | EffectDecl
                   | LanguageDecl
                   | MacroDecl
                   | PackageDecl
                   | DomainDecl

Statement        ::= LetStmt
                   | ConstStmt
                   | ReturnStmt
                   | IfStmt
                   | WhileStmt
                   | ForStmt
                   | MatchStmt
                   | BreakStmt
                   | ContinueStmt
                   | ThrowStmt
                   | TryStmt
                   | HandleStmt
                   | ExpressionStmt
                   | BlockStmt

FunctionDecl     ::= Modifiers? "fn" Identifier TypeParams?
                     "(" Parameters? ")" ReturnType? Effects? Block

ReturnType       ::= "->" TypeExpr

LetStmt          ::= "let" "mut"? Identifier
                     (":" TypeExpr)? "=" Expression ";"?

ConstStmt        ::= "const" Identifier
                     (":" TypeExpr)? "=" Expression ";"?

IfStmt           ::= "if" Expression Block
                     ("else" (IfStmt | Block))?

WhileStmt        ::= "while" Expression Block

ForStmt          ::= "for" Pattern "in" Expression Block

MatchStmt        ::= "match" Expression "{" MatchArm+ "}"

MatchArm         ::= Pattern Guard?
                     ("=>" Expression | "->" Block) ","?

Guard            ::= "when" Expression

QuantumDecl      ::= "quantum" "circuit" Identifier
                     GenericParams? ParameterList? Block
                   | "circuit" Identifier
                     GenericParams? ParameterList? Block

QuantumExpr      ::= QuantumStateLiteral
                   | QuantumOperation
                   | QuantumRegion
                   | QuantumMeasure
                   | QuantumReset
                   | QuantumBarrier
                   | QuantumControl
                   | QuantumObserve

QuantumOperation ::= "apply" QuantumOperationSpec
                     QuantumTargetList

QuantumOperationSpec
                 ::= Expression

QuantumTargetList
                 ::= "to" TargetList

TargetList       ::= Expression ("," Expression)*

NanoDecl         ::= "nano" "agent" Identifier Block
                   | "agent" Identifier Block

EffectDecl       ::= "effect" Identifier
                     TypeParams? EffectSignature? ";"

HandleStmt       ::= "handle" Expression Block

TypeExpr         ::= NamedType
                   | GenericType
                   | FunctionType
                   | TupleType
                   | ArrayType
                   | SliceType
                   | ReferenceType
                   | NullableType
                   | DependentType
                   | LinearType
                   | AffineType
                   | EffectfulType
                   | QuantumType
                   | TemporalType
                   | ResultType
                   | NeverType

Expression       ::= AssignmentExpr

AssignmentExpr   ::= RangeExpr
                     AssignmentOp AssignmentExpr
                   | RangeExpr

RangeExpr        ::= LogicalOrExpr
                     (RangeOp LogicalOrExpr)?

LogicalOrExpr    ::= LogicalAndExpr
                     (("||" | "or") LogicalAndExpr)*

LogicalAndExpr   ::= BitOrExpr
                     (("&&" | "and") BitOrExpr)*

BitOrExpr        ::= BitXorExpr ("|" BitXorExpr)*

BitXorExpr       ::= BitAndExpr ("^" BitAndExpr)*

BitAndExpr       ::= EqualityExpr ("&" EqualityExpr)*

EqualityExpr     ::= ComparisonExpr
                     (("==" | "!=") ComparisonExpr)*

ComparisonExpr   ::= ShiftExpr
                     (("<" | "<=" | ">" | ">=") ShiftExpr)*

ShiftExpr        ::= SumExpr
                     (("<<" | ">>") SumExpr)*

SumExpr          ::= ProductExpr
                     (("+" | "-") ProductExpr)*

ProductExpr      ::= PrefixExpr
                     (("*" | "/" | "%") PrefixExpr)*

PrefixExpr       ::= PrefixOp PrefixExpr
                   | PostfixExpr

PostfixExpr      ::= PrimaryExpr Postfix*

PrimaryExpr      ::= Identifier
                   | Literal
                   | Tuple
                   | Array
                   | Block
                   | Lambda
                   | IfExpr
                   | MatchExpr
                   | LoopExpr
                   | AsyncExpr
                   | AwaitExpr
                   | SpawnExpr
                   | NewExpr
                   | QuantumExpr
                   | NanoExpr
                   | SankofaExpr
                   | Parenthesized

This grammar deliberately describes semantic categories, not a closed catalog of every mathematical, quantum or hardware operation.

8. Quantum Language Design

Quantum computing is a first-class Zamani domain.

The grammar must express quantum intent while remaining independent of physical hardware.

8.1 Quantum identity

Quantum values must ultimately use canonical quantum semantic identities.

Frontend-local qubit/gate representations must not compete with "quantum::ir".

The repository's quantum architecture already establishes canonical IR identity as an important boundary.

8.2 No fixed qubit count

The language must never impose compiler-wide constants such as:

MAX_QUBITS = 1024
MAX_QUBITS = 4096
MAX_REGISTER_SIZE = 32

A program may explicitly request a size:

register[1024]

if that size is part of its algorithm.

That is program semantics.

It must not mean that the compiler can never represent:

register[1025]

Resource limitations belong to compilation/execution budgets.

8.3 Gate vocabulary

The current ANTLR grammar contains a fixed list including:

Hadamard
CNOT
PauliX
PauliY
PauliZ
T
S
Swap

That list must not be the complete quantum language.

The production model instead treats quantum operations as semantic operations.

A backend can have a native gate set such as:

{H, X, Y, Z, CX, RZ, ...}

and the compiler can synthesize a portable operation into that set.

Therefore:

Zamani source
    ↓
Portable quantum operation
    ↓
Canonical quantum IR
    ↓
Decomposition
    ↓
Routing
    ↓
Scheduling
    ↓
Calibration/noise-aware lowering
    ↓
Native hardware representation

The backend may reject an operation because the target lacks sufficient capability, but it must never silently replace the operation with a different computation.

8.4 Quantum control

The semantic model must support, where implemented:

- single-target operations;
- multi-target operations;
- controls;
- negative controls;
- parameterized operations;
- adjoints/inverses;
- measurement;
- mid-circuit measurement;
- reset;
- classical feed-forward;
- dynamic control flow;
- barriers/fences;
- logical-to-physical mapping;
- error-correction metadata;
- noise-aware execution;
- pulse-level lowering.

These should be represented as semantic structures rather than an uncontrolled list of keywords.

9. Mathematics

The current "Zamani.g4" contains many direct mathematical statements including vectorization, matrix operations, tensor operations, symbolic mathematics, calculus, statistics, FFT, signal processing and optimization.

These capabilities are valuable, but grammar-level keywords should not become an unbounded mathematical catalog.

The architecture is:

Language syntax
    ↓
Generic expressions/types/operators
    ↓
Mathematical semantic interfaces
    ↓
Intrinsics / standard library
    ↓
Optimization
    ↓
Backend

A mathematical operation becomes grammar syntax only when it has language-level semantics that cannot reasonably be represented as a typed operation, intrinsic or library capability.

This preserves expressiveness while preventing grammar explosion.

10. Nano, MTS and Sankofa

Nano agents, Multi-Timeline System constructs and Sankofa constructs follow the same architecture:

Syntax
  ↓
AST
  ↓
Semantic validation
  ↓
Canonical representation
  ↓
Runtime/backend

"remember", "recall", "learn", "infer", "zamani" and "sasa" must not cause parser-side hidden state.

Their semantics belong to semantic/runtime layers.

MTS must not encode a fixed number of timelines or slices.

11. Effects and Capabilities

Algebraic effects are language-level semantics.

The grammar can describe:

- effect declarations;
- effectful operations;
- handlers;
- effect lists;
- effectful types.

Semantic analysis determines whether an operation is valid.

Compiler implementation safety is separate from the source-language "unsafe" keyword.

The compiler itself must contain no Rust "unsafe".

12. Type System

The AST already contains type-oriented structures including generic parameters, bounds and advanced type forms.

The production language should support, as each feature becomes semantically implemented:

- named types;
- generic types;
- tuples;
- function types;
- arrays;
- slices;
- references;
- ownership/resource qualifiers;
- linear types;
- affine types;
- quantum types;
- effectful types;
- optional types;
- result types;
- never types;
- temporal types;
- dependent types;
- future versioned type extensions.

Surface sugar must lower to canonical type representations.

For example:

T?

may lower to:

Optional<T>

rather than creating a second unrelated semantic type.

13. Patterns

The AST already contains structured pattern concepts including:

- wildcard;
- identifier;
- literal;
- tuple;
- struct;
- enum;
- or-pattern;
- range;
- reference.

Therefore match patterns must be parsed as patterns, not as arbitrary expressions.

Semantic checking must cover:

- binding consistency;
- type compatibility;
- exhaustiveness;
- unreachable arms;
- guard validity;
- ownership/resource behavior.

14. Modules and Imports

Module syntax describes logical module identity.

Filesystem layout, package registries and remote dependencies belong to the package/build system.

The grammar must never perform filesystem or network operations.

Import resolution must occur after parsing.

This maintains deterministic and sandboxable frontend behavior.

15. Attributes and Annotations

Attributes are structured metadata.

They must preserve:

- name/path;
- arguments;
- source span;
- target node;
- semantic namespace.

Unknown attributes may be accepted only under an explicit extensibility policy.

Safety-critical attributes must never be silently ignored.

16. Diagnostics

Every lexer/parser diagnostic should contain:

error code
message
primary span
optional labels
optional notes
optional suggestion

Malformed input must never be turned into apparently valid AST nodes without explicit error state.

Recovery must make progress.

Diagnostics must remain deterministic.

17. AST Contract

"src/ast/mod.rs" is a major structural boundary and currently contains core statements, quantum constructs, nano/Sankofa constructs, effects, language declarations and advanced system constructs.

Production rules:

- Every grammar construct maps to one canonical AST representation or documented desugaring.
- Every AST node carries sufficient source information.
- AST nodes do not contain physical device state.
- AST nodes do not contain hidden global mutable state.
- AST traversal must support deep/generated programs.
- Semantic identity must use structured types rather than arbitrary strings where identity matters.
- Duplicate quantum semantic types are prohibited.

18. Canonical Quantum IR Integration

The intended ownership is:

Frontend AST
    ↓
Semantic quantum model
    ↓
quantum::ir
    ├── optimization
    ├── routing
    ├── scheduling
    ├── resilience/QEC
    ├── ZQN
    ├── calibration
    ├── hardware
    └── benchmarking

The repository already documents substantial boundaries for scheduling, ZQN and optimization.

The grammar must preserve those boundaries.

19. Scaling From Tiny to Infinity

“From atom to everywhere” means Zamani imposes no artificial finite machine ceiling.

Scaling must be achieved through:

- symbolic resource expressions;
- dynamically sized representations where semantically appropriate;
- checked/arbitrary-precision numeric representations where required;
- streaming;
- iterative traversal;
- explicit resource budgets;
- worklist-based analysis;
- lazy/chunked representations;
- target capability discovery;
- late physical-resource binding;
- deterministic compilation;
- distributed execution outside the grammar boundary.

A compiler may reject:

program requires 10^30 logical qubits

when a selected target/resource budget cannot satisfy that requirement.

That is a resource failure, not a language-size limitation.

20. POCO-REAF

Program Once

The programmer describes semantics rather than machine topology.

Compile Once

The compiler produces a canonical representation whose semantics do not depend on a specific target.

Run Everywhere

Any conforming target with adequate capabilities can execute or lower the artifact.

Anywhere

Target selection occurs independently of source syntax.

Forever

Language versioning, compatibility rules, semantic hashes, artifact metadata and explicit capability descriptions preserve portability and migration.

POCO-REAF does not mean every program runs on every physical machine.

It means target incompatibility is explicit and semantics-preserving.

The compiler distinguishes:

Language validity
Semantic validity
Artifact validity
Target compatibility
Resource availability
Runtime availability

21. Versioning

Grammar changes must be versioned.

Each language version defines:

- lexical changes;
- grammar changes;
- semantic changes;
- type changes;
- deprecated syntax;
- removed syntax;
- compatibility behavior;
- migration rules.

Breaking changes require explicit version boundaries.

The "language" declaration identifies source-language compatibility.

Compiler features and hardware capabilities remain separate.

22. Canonical Grammar Repository Layout

The intended structure is:

grammar/
├── DESIGN.md
├── README.md
├── spec/
│   ├── lexical.md
│   ├── syntax.md
│   ├── semantics.md
│   ├── types.md
│   ├── effects.md
│   ├── quantum.md
│   ├── compatibility.md
│   └── conformance.md
├── antlr/
│   ├── ZamaniLexer.g4
│   └── ZamaniParser.g4
├── reference/
├── tests/
│   ├── valid/
│   ├── invalid/
│   ├── quantum/
│   ├── types/
│   ├── effects/
│   └── compatibility/
├── Zamani.g4
├── grammar.md
└── Zamani-Grammar.md

"Zamani.g4" remains a compatibility entry point during migration.

"grammar.md" becomes an implementation-conformance snapshot.

"Zamani-Grammar.md" becomes historical/design material unless features are formally promoted.

23. ANTLR/Rust Synchronization

The same conformance corpus must be processed by:

1. Rust lexer/parser.
2. ANTLR grammar.

Valid source must be syntactically equivalent.

Invalid source must be rejected according to the same language version.

Parse-tree structure need not be identical.

Semantic normalization must be equivalent.

CI must detect drift whenever:

- tokens change;
- keywords change;
- precedence changes;
- declarations change;
- type syntax changes;
- quantum syntax changes;
- AST variants change.

24. Production Test Matrix

Lexical

Test:

- identifiers;
- Unicode identifiers according to the identifier policy;
- keywords;
- keyword boundaries;
- integer bases;
- floating-point literals;
- strings;
- escapes;
- character literals;
- comments;
- malformed literals;
- illegal characters;
- source spans.

Core Syntax

Test:

- declarations;
- functions;
- default arguments;
- blocks;
- control flow;
- patterns;
- modules;
- imports;
- classes;
- traits;
- impls;
- attributes;
- generics;
- type expressions;
- closures;
- precedence.

Quantum

Test:

- zero/small quantum programs;
- symbolic/dynamic resource sizes;
- parameterized operations;
- arbitrary semantic operation identifiers;
- controls;
- measurement;
- reset;
- classical feed-forward;
- logical/physical mapping;
- error correction;
- noise metadata;
- large generated circuits.

Scaling

Test:

- deep expressions;
- deep blocks;
- large declaration counts;
- large token streams;
- huge generated quantum programs;
- explicit resource exhaustion;
- deterministic repeated compilation.

Negative Tests

Test:

- malformed syntax;
- invalid patterns;
- invalid types;
- invalid effect usage;
- unsupported target capabilities;
- resource exhaustion;
- invalid version usage.

25. Rust 1.97 / 1.97.1

The repository declares Rust 1.97/1.97.1 as its compiler baseline.

The grammar/frontend implementation must therefore remain compatible with that baseline.

The implementation must contain no:

unsafe
unsafe fn
unsafe impl
unsafe { ... }

Performance must be achieved through safe Rust.

Preferred mechanisms include:

- ownership;
- borrowing;
- "Arc";
- explicit worklists;
- checked arithmetic;
- fallible operations;
- structured errors;
- deterministic serialization;
- safe concurrency.

No unsafe parser optimization is permitted.

26. Issues Resolved by This Design

Multiple grammar authorities

Resolved: one normative language specification.

Fixed quantum gate vocabulary

Resolved: operations are semantic entities.

Fixed machine sizes

Resolved: sizes are program/resource information, not compiler ceilings.

Lexer drift

Resolved: canonical lexical registry and conformance tests.

Parser drift

Resolved: canonical syntax and cross-frontend testing.

AST drift

Resolved: one-to-one mapping/desugaring contract.

Aspirational features presented as implemented

Resolved: explicit implementation/conformance status.

Duplicate quantum definitions

Resolved: canonical "quantum::ir".

Grammar explosion

Resolved: domain operations primarily belong to typed semantic APIs/intrinsics.

Deep-program failures

Resolved: iterative traversal/worklists and no arbitrary grammar-level depth limits.

Unsafe compiler implementation

Resolved: repository-wide safe-Rust requirement.

Hardware leakage into source

Resolved: late physical realization.

27. File Ownership and Integration

File/Area| Owns| Must Not Own| Integration
"grammar/DESIGN.md"| Grammar architecture/invariants| Runtime implementation| All grammar/frontend docs
"grammar/Zamani.g4"| ANTLR syntax| Runtime semantics| ANTLR validation
"grammar/grammar.md"| Implementation snapshot| Aspirational language| Lexer/parser/AST
"grammar/Zamani-Grammar.md"| Historical/universal design| Compiler authority| Language evolution
"src/lexer.rs"| Tokenization| Semantic interpretation| Parser/source map
"src/parser.rs"| Syntax| Backend decisions| AST
"src/ast/"| Source structure| Hardware execution| Semantic analysis
"src/semantic.rs"| Semantic/type validation| Hardware execution| AST/IR
"src/ir_gen.rs"| AST → IR| Hardware scheduling| Canonical IR
"src/quantum/ir/"| Quantum semantics| Frontend syntax| Optimizer/routing/scheduling/ZQN
"src/quantum/optimization/"| Semantics-preserving transforms| Source grammar| Quantum IR
"src/quantum/scheduling/"| Timing/resource scheduling| Source grammar| Quantum IR/hardware
"src/quantum/zqn/"| Noise-aware execution| Core AST grammar| IR/calibration/hardware

28. Definition of Done

The grammar subsystem is production-ready only when:

- one canonical specification exists;
- Rust lexer conforms;
- Rust parser conforms;
- ANTLR conforms;
- every syntax construct has an AST representation;
- every semantic construct has validation;
- every executable construct has a defined lowering path;
- quantum constructs use canonical "quantum::ir";
- quantum syntax does not impose a closed hardware gate set;
- no compiler-wide machine-size ceilings exist;
- resource limits are explicit;
- deep/generated programs are supported;
- diagnostics are structured and span-aware;
- valid/invalid conformance tests exist;
- Rust 1.97/1.97.1 is supported;
- Rust "unsafe" is absent;
- compilation is deterministic;
- language versioning is explicit;
- documentation does not claim unimplemented features are implemented.

29. Migration Order

Implement this design in dependency order:

1. Freeze language version.
2. Establish canonical token taxonomy.
3. Audit "src/lexer.rs".
4. Reconcile lexer with "grammar/Zamani.g4".
5. Reconcile lexer with "grammar/grammar.md".
6. Normalize duplicate tokens.
7. Audit "src/parser.rs".
8. Define canonical AST mappings.
9. Build conformance tests.
10. Reconcile ANTLR and Rust parsing.
11. Mark aspirational features in "Zamani-Grammar.md".
12. Remove fixed quantum gate enumeration as the semantic foundation.
13. Route quantum constructs through canonical "quantum::ir".
14. Remove duplicate quantum semantic representations.
15. Validate "src/ir_gen.rs" coverage.
16. Add deep/generated-program tests.
17. Add resource/capability diagnostics.
18. Add compatibility/version tests.
19. Only then add new language domains.

30. Final Architectural Principle

Zamani's grammar must describe what computation means, not merely what today's machines happen to support.

Zamani may grow across:

- classical computing;
- mathematics;
- quantum computing;
- nano computation;
- temporal systems;
- Sankofa memory;
- AI/cognitive computation;
- HDL;
- distributed systems;
- effects;
- dependent types;
- future computational substrates.

Growth must occur through composable semantic domains and canonical IR boundaries.

The grammar must therefore obey:

Express intent once.
Represent meaning canonically.
Defer physical realization.
Scale with available resources.
Preserve semantics across targets.
Version the language explicitly.
Never make today's machine the language's limit.

This is the grammar-level foundation for:

Program Once → Compile Once → Run Everywhere → Anywhere → Forever.I also verified that the current Cargo.toml declares the Rust 1.97/1.97.1 baseline and the existing ANTLR dependency, while src/parser.rs and src/ast/mod.rs confirm the current handwritten frontend/AST architecture.

