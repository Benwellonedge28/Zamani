Zamani Quantum Literals — Lexical Specification

Path: "grammar/lexer/quantum-literals.md"
Language: Zamani
Status: Normative production specification
Layer: Lexer / lexical contract
Grammar technology: ANTLR4
Compiler baseline: Rust 1.97 / Rust 1.97.1
Rust edition: Rust 2021
Safety: Safe Rust only; "unsafe" is prohibited
Primary objective: Target-independent, deterministic, scalable quantum literal syntax supporting POCO-REAF.

---

1. Purpose

This file defines the normative lexical contract for quantum-specific literal notation in Zamani.

It specifies:

- what source text constitutes a quantum literal;
- how compact quantum-state notation is tokenized;
- which Unicode symbols are part of that notation;
- how malformed quantum literal forms are handled;
- how quantum literals integrate with the general lexical system;
- how quantum literals integrate with parser-level quantum state expressions;
- how the resulting tokens map toward the frontend AST and canonical quantum semantic representation;
- how the lexical design remains independent of hardware, simulator, QPU, QEC, ZQN, routing, scheduling, and runtime implementation.

This file does not define the complete quantum language.

It does not define quantum operations, gates, qubits, circuits, measurements, observables, state evolution, QEC, noise, hardware, or quantum IR.

Its responsibility ends at the lexical boundary.

---

2. Production Status

This document is the authoritative design contract for:

grammar/lexer/quantum-literals.g4

and its integration with the repository's canonical lexer.

The repository currently contains an existing "QUANTUM_LITERAL" lexical concept for:

|0⟩
|1⟩
|+⟩
|-⟩

That existing capability is retained.

However, this document corrects an architectural problem:

«A finite set of compact quantum-state literals must not be confused with the complete quantum state-expression language.»

The parser-side quantum grammar already has a broader abstraction for:

- symbolic state references;
- state constructors;
- superpositions;
- mixtures;
- tensor/product composition;
- transformations;
- recursively composed state expressions.

Therefore:

grammar/lexer/quantum-literals.g4

owns only the lexical forms that genuinely need to be single lexical tokens.

The broader quantum state language belongs to:

grammar/quantum/

and its semantic integration.

---

3. Architectural Authority

The authority hierarchy for this file is:

grammar/DESIGN.md
        |
        v
grammar/specification/lexical.md
        |
        v
grammar/lexer/README.md
        |
        v
grammar/lexer/quantum-literals.md
        |
        v
grammar/lexer/quantum-literals.g4
        |
        v
canonical Zamani lexer
        |
        v
parser
        |
        v
frontend AST
        |
        v
semantic model
        |
        v
quantum::ir

This file must not override:

- "grammar/DESIGN.md";
- the language specification;
- the canonical source-span contract;
- the canonical token contract;
- the canonical identifier contract;
- the canonical AST architecture;
- the canonical quantum IR architecture.

If a conflict is discovered, the conflict must be resolved through the repository's authority hierarchy rather than silently creating a third behavior.

---

4. Core Principle

Zamani quantum literals describe source-level computational meaning.

They do not describe the machine that eventually realizes that meaning.

For example:

|0⟩

means an abstract quantum basis-state literal.

It does not mean:

physical qubit 0

It does not mean:

QPU 0

It does not mean:

simulator register 0

It does not specify:

- physical qubit placement;
- state-vector representation;
- density-matrix representation;
- tensor-network representation;
- numerical precision;
- simulator;
- QPU;
- gate decomposition;
- routing;
- scheduling;
- calibration;
- error correction;
- noise model.

Those decisions belong downstream.

---

5. POCO-REAF Contract

Quantum literals must support:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever»

The lexical system therefore MUST NOT contain artificial machine limits.

It must not define:

MAX_QUBITS
MAX_QUANTUM_LITERALS
MAX_STATE_SIZE
MAX_STATE_TERMS
MAX_AMPLITUDES
MAX_AMPLITUDE_PRECISION
MAX_REGISTER_SIZE
MAX_QUBIT_INDEX
MAX_CIRCUIT_SIZE
MAX_QPU_SIZE
MAX_SIMULATOR_SIZE
MAX_MEMORY
MAX_PROGRAM_SIZE

or equivalent constants as language rules.

A program can contain an arbitrary number of quantum literals subject to actual resource availability.

For example:

|0⟩
|1⟩
|0⟩ ⊗ |1⟩

and a program containing millions, billions, or more quantum-state expressions are all governed by the same language contract.

Any practical limit imposed by:

- memory;
- operating system;
- parser implementation;
- compiler resource budget;
- build environment;
- runtime;
- target hardware;

is an implementation/resource limit, not a grammar limit.

---

6. File Contract

File

grammar/lexer/quantum-literals.md

Purpose

Define the complete lexical contract for compact quantum-state literals.

Status

Normative.

Owns

This file owns:

- compact quantum-state literal policy;
- lexical boundaries of compact quantum literals;
- Unicode symbols required by compact quantum literals;
- tokenization requirements;
- malformed compact-literal classification;
- lexer/parser integration;
- literal preservation requirements;
- source-span requirements;
- compatibility requirements;
- scalability requirements;
- conformance requirements.

Does Not Own

This file does not own:

- quantum operation syntax;
- gate names;
- qubit references;
- quantum register declarations;
- circuit syntax;
- measurement semantics;
- observables;
- amplitudes as a semantic type;
- state normalization;
- state validity;
- state evolution;
- entanglement semantics;
- QEC;
- ZQN;
- routing;
- scheduling;
- calibration;
- HAL;
- physical qubit allocation;
- simulator selection;
- quantum backend selection;
- canonical quantum IR.

Inputs

UTF-8 Zamani source.

Outputs

Quantum-literal tokens consumed by the canonical parser.

Dependencies

- "grammar/DESIGN.md"
- "grammar/specification/lexical.md"
- "grammar/lexer/README.md"
- "grammar/lexer/tokens.g4"
- "grammar/lexer/unicode.md"
- canonical lexer
- quantum parser grammar
- source-span specification

Downstream Consumers

- canonical parser;
- frontend AST;
- semantic analysis;
- quantum semantic model;
- "quantum::ir";
- diagnostics;
- IDE tooling;
- formatter;
- syntax highlighting;
- source mapping;
- conformance tests.

---

7. Lexical Ownership Boundary

The following distinction is mandatory.

Lexical quantum literal

|0⟩
|1⟩
|+⟩
|-⟩

These may be represented as a dedicated lexical token.

Quantum state expression

Examples:

psi
initial_state
state(|0⟩)
superposition(...)
mixture(...)
tensor(...)
product(...)
transform(...)

These are not automatically lexical literals.

They belong to parser-level quantum syntax.

Quantum resource reference

Examples:

q
q[i]
register[index]
logical::q

These are not quantum literals.

They are quantum resource/reference syntax.

Quantum operation

Examples:

H
X
CNOT
U
custom_operation
vendor_operation

These are not quantum literals.

Their identity belongs to semantic operation resolution.

This separation is essential for extensibility.

---

8. Canonical Compact Quantum Literal Set

The stable compact quantum literal forms are:

|0⟩
|1⟩
|+⟩
|-⟩

The characters are:

Component| Unicode
`| `
"0"| U+0030
"1"| U+0031
"+"| U+002B
"-"| U+002D
"⟩"| U+27E9

The closing ket character MUST be the exact Unicode scalar value:

U+27E9

Alternative visually similar characters MUST NOT silently become equivalent.

For example:

|0>

is not the same lexical spelling as:

|0⟩

unless a future compatibility profile explicitly introduces such syntax.

No Unicode normalization or visual-glyph substitution may silently alter the source.

---

9. Canonical Lexical Production

The conceptual lexical production is:

quantumLiteral
    ::= "|" quantumCompactStateSymbol "⟩"

where:

quantumCompactStateSymbol
    ::= "0"
      | "1"
      | "+"
      | "-"

Therefore:

|0⟩
|1⟩
|+⟩
|-⟩

are valid compact quantum literals.

The lexer MUST emit the canonical quantum-literal token for these forms.

---

10. Token Contract

The canonical token is:

QUANTUM_LITERAL

The token MUST preserve its complete source spelling.

For example:

|0⟩

must not become three independent semantic tokens:

PIPE
INTEGER
KET

when it is recognized as a canonical quantum literal.

The parser must therefore receive a stable token representing the complete compact literal.

The token's source span covers the entire literal.

For:

|0⟩

the span begins at the byte offset of "|" and ends immediately after "⟩".

---

11. Why Compact Literals Are Tokens

The compact notation:

|0⟩

has a lexical identity that is useful to:

- parser grammar;
- AST construction;
- syntax highlighting;
- formatting;
- diagnostics;
- source mapping;
- documentation tooling.

Making it one lexical token prevents accidental ambiguity with ordinary bitwise/pipe syntax.

However, this does not imply that every quantum expression should become a lexer token.

The following should remain parser/semantic constructs:

superposition(...)
mixture(...)
tensor(...)
state(...)
psi
alpha
theta
custom_state

This keeps the lexer small, stable, and extensible.

---

12. General Quantum States Are Not Lexical Literals

The lexer MUST NOT attempt to create a universal grammar for arbitrary quantum state notation.

Do not turn forms such as:

|psi⟩
|alpha⟩
|state⟩
|00⟩
|01⟩
|010101⟩

into an ever-growing lexical family.

These forms have different semantic interpretations.

For example:

|00⟩

may represent a multi-qubit basis state in a future semantic grammar.

That does not mean the lexer should enumerate:

|00⟩
|01⟩
|10⟩
|11⟩
...

Doing so would create an artificial grammar boundary.

Instead, generalized state notation must be handled by the quantum parser grammar when and if the language specification defines it.

---

13. Scalable General State Representation

The production architecture must allow generalized quantum states without changing this lexer every time the state model expands.

The preferred pipeline is:

source
  |
  v
compact lexical literal
  |
  v
QUANTUM_LITERAL
  |
  v
parser
  |
  v
quantum state AST
  |
  v
semantic quantum state
  |
  v
quantum::ir

For more general forms:

source
  |
  v
ordinary Zamani tokens
  |
  v
quantum parser
  |
  v
quantum state AST
  |
  v
semantic quantum state
  |
  v
quantum::ir

This allows the language to grow without creating an ever-larger lexer.

---

14. Basis-State Meaning

The compact literals have source-level semantic meanings:

|0⟩

represents the computational-basis zero state.

|1⟩

represents the computational-basis one state.

|+⟩

represents the conventional plus state notation.

|-⟩

represents the conventional minus state notation.

This file does not define their mathematical implementation.

Semantic analysis owns:

- state dimensionality;
- normalization;
- state type;
- composition;
- context;
- validity;
- resource implications.

---

15. No Physical Qubit Meaning

A critical invariant is:

|0⟩ != physical_qubit(0)

The literal describes a state.

A physical qubit reference describes a resource.

Therefore:

|0⟩

must never implicitly allocate:

q[0]

or:

physical_qubit(0)

or:

device("qpu0")

Allocation belongs downstream.

---

16. No Fixed Qubit Count

The lexical grammar must never impose:

1 qubit
2 qubits
32 qubits
64 qubits
128 qubits
1024 qubits

as a language maximum.

A source program may contain:

|0⟩

or an arbitrarily large collection of quantum expressions.

The actual executable representation is determined by:

- semantic requirements;
- compiler strategy;
- available resources;
- target capabilities;
- runtime;
- execution environment.

---

17. Unicode Contract

Zamani source is UTF-8.

Quantum literal syntax uses Unicode:

⟩

The lexer MUST operate on valid Unicode scalar values.

The lexer MUST NOT:

- normalize source text;
- perform locale-dependent conversion;
- replace visually similar characters;
- perform Unicode case folding on literal syntax;
- interpret grapheme clusters as individual quantum symbols.

The exact source code points matter.

---

18. Unicode Normalization

The quantum literal lexer MUST NOT normalize:

NFC
NFD
NFKC
NFKD

or any other Unicode normalization form.

The compact quantum literal vocabulary does not require normalization.

For example, the canonical:

⟩

must remain the exact source code point.

If identifier normalization is introduced elsewhere, that does not apply to quantum literal source text.

---

19. Unicode Confusables

Visually similar Unicode characters must not be silently treated as quantum syntax.

For example, a character that visually resembles:

⟩

but has another Unicode code point is not automatically equivalent.

This is important for:

- deterministic compilation;
- source review;
- reproducible builds;
- security;
- source provenance.

Confusable detection may be provided by tooling.

It must not change lexical meaning.

---

20. Grapheme Clusters

Quantum literal recognition operates on Unicode scalar values.

It does not operate on user-perceived grapheme clusters.

The literal:

|0⟩

has an exact lexical sequence.

The lexer must not use display width or grapheme segmentation to decide whether a literal is valid.

Display concerns belong to:

- IDE tooling;
- formatting;
- diagnostics;
- source visualization.

---

21. Whitespace

No whitespace is permitted inside a compact quantum literal.

Valid:

|0⟩

Invalid as one quantum literal:

| 0⟩
|0 ⟩
|0⟩

where additional whitespace occurs within the literal's lexical boundary.

The final form of whitespace handling is inherited from the canonical lexical whitespace contract.

Quantum literal recognition must not silently skip whitespace internally.

---

22. Newlines

A quantum literal cannot cross a source line boundary.

Therefore forms such as:

|
0⟩

must not be interpreted as one compact quantum literal.

Likewise:

|0
⟩

must not be interpreted as one compact quantum literal.

Line-ending handling remains governed by the general Unicode/source-encoding contract.

---

23. Comments

Comments cannot occur inside a quantum literal.

Invalid:

|/* comment */0⟩

Invalid:

|0/* comment */⟩

The complete literal must be lexically contiguous.

Comments surrounding a literal remain ordinary comments:

/* initial state */
|0⟩

---

24. Adjacent Literals

Adjacent quantum literals remain separate tokens unless a parser-level production explicitly defines composition.

For example:

|0⟩|1⟩

produces two quantum literal tokens.

The lexer must not infer:

|01⟩

or a tensor product.

Composition is parser/semantic behavior.

This is crucial for preventing accidental semantic decisions in the lexer.

---

25. Tensor/Product Composition

The lexer does not define tensor products.

For example:

|0⟩ ⊗ |1⟩

contains quantum literals plus an operator.

The operator:

⊗

is governed by the general operator/quantum-expression contract.

The lexer must not transform:

|0⟩ ⊗ |1⟩

into a new combined quantum literal.

Semantic analysis determines the meaning of the tensor/product operation.

---

26. Bra Notation

This file does not define a dedicated bra literal.

For example:

⟨0|
⟨1|

must not automatically become a "QUANTUM_LITERAL".

General bra/ket algebra belongs to the quantum expression layer.

This avoids coupling lexical analysis to the complete mathematical notation system.

---

27. Measurement

Measurement is not a literal.

For example:

measure q

contains a quantum operation/statement.

The lexer must not create a quantum-literal token for measurement results.

Measurement semantics belong to the quantum semantic layer.

---

28. Quantum Operation Names

The quantum literal lexer MUST NOT enumerate gate names.

Do not add:

H
X
Y
Z
S
T
CNOT
CZ
SWAP
Toffoli
U
...

as quantum-literal rules.

Those are operations, not literals.

The language must support future operations without requiring modifications to this file.

---

29. Custom Operations

Custom quantum operations must not require lexer changes.

For example:

custom_operation
vendor_operation
logical_operation
calibrated_operation
future_operation

can remain ordinary identifiers or qualified names according to the quantum operation grammar.

This is essential for:

- extensibility;
- dialects;
- user-defined operations;
- future hardware;
- research operations;
- interoperability;
- vendor-independent source.

---

30. Quantum State References

These are not lexical literals:

psi
initial_state
ground_state
algorithm::state
domain::quantum::state

They are identifiers or qualified names.

The semantic layer determines what they refer to.

This allows a program to declare arbitrary state abstractions without changing the lexer.

---

31. Literal Versus State Constructor

These are intentionally different:

|0⟩

and:

state(|0⟩)

The first is a compact literal.

The second is a parser-level state constructor.

The constructor may eventually carry:

- metadata;
- annotations;
- parameters;
- type information;
- resource requirements;
- semantic constraints.

The lexer must not collapse both into one token.

---

32. Amplitudes

This file does not define amplitude syntax.

Expressions such as:

alpha
0.5
theta
complex_value
f(x)

remain ordinary Zamani expressions unless the quantum parser gives them quantum-state meaning.

This avoids imposing:

float64
float128
fixed precision

on the language.

Numerical representation belongs to type/semantic analysis.

---

33. Generalized Basis States

If Zamani later standardizes generalized basis-state notation such as:

|00⟩
|101⟩
|010101⟩

that feature must not be implemented by adding an unbounded collection of lexer alternatives.

Instead, it should receive a separate documented lexical/parser contract.

A future design may use a parameterized quantum-state grammar, but that change must:

1. preserve existing "|0⟩", "|1⟩", "|+⟩", "|-⟩";
2. preserve token compatibility;
3. define ambiguity resolution;
4. define source spans;
5. define AST mapping;
6. define semantic mapping;
7. define compatibility;
8. add positive/negative/boundary/scalability tests.

---

34. Invalid Compact Literals

The following are not valid "QUANTUM_LITERAL" forms:

|2⟩
|a⟩
|q⟩
|00⟩
|01⟩
|0>
|0
0⟩
||0⟩
|0⟩⟩

This does not mean every invalid form must produce a special quantum-literal error.

The final diagnostic depends on tokenization and parser context.

The important invariant is:

«Invalid compact quantum syntax must never silently become a different valid quantum literal.»

---

35. Malformed Literal Diagnostics

The lexer/parser diagnostic system must be able to distinguish at least:

- malformed quantum literal;
- incomplete quantum literal;
- invalid quantum literal symbol;
- wrong closing delimiter;
- unexpected Unicode delimiter;
- illegal whitespace within literal;
- illegal newline within literal;
- ambiguous token sequence.

Diagnostic ownership belongs to:

grammar/lexer/diagnostics.md

This file defines the conditions.

"diagnostics.md" owns stable diagnostic identifiers and user-facing diagnostic formatting.

---

36. Error Recovery

Malformed quantum literal input must not cause:

- panic;
- process termination;
- undefined behavior;
- unsafe memory access;
- silent token corruption.

The Rust implementation must use safe error propagation.

Malformed source is ordinary user input.

It must be handled deterministically.

---

37. Source Spans

Every recognized "QUANTUM_LITERAL" must have a source span.

The span must:

- begin at "|";
- end immediately after "⟩";
- use the repository's canonical source-span representation;
- never split a UTF-8 code point;
- preserve exact source location;
- support diagnostics and tooling.

If the repository's canonical source-span representation uses byte offsets, the quantum-literal token span uses UTF-8 byte offsets.

Line and column information may be derived from the canonical source map.

This file must not introduce a competing span representation.

---

38. Exact Source Preservation

The lexer must preserve the exact source spelling of the token.

For:

|0⟩

the token text remains:

|0⟩

It must not be rewritten as:

|0>

or normalized to another Unicode sequence.

This is necessary for:

- diagnostics;
- formatting;
- source maps;
- reproducible builds;
- IDE tooling;
- provenance.

---

39. AST Contract

This file produces lexical tokens only.

The frontend AST is responsible for converting:

QUANTUM_LITERAL

into the repository's canonical domain-neutral AST representation.

The AST must preserve:

- literal kind;
- exact source span;
- source representation where required;
- semantic literal payload;
- provenance.

The AST must not encode:

- physical qubit ID;
- QPU ID;
- simulator ID;
- hardware topology;
- native gate;
- calibration;
- scheduling;
- QEC configuration.

---

40. Semantic Contract

Semantic analysis determines the meaning of the parsed quantum literal.

For example:

|0⟩

may become a semantic representation corresponding to a computational-basis zero state.

The semantic layer determines:

- valid quantum context;
- state type;
- dimensional compatibility;
- composition;
- ownership;
- resource implications;
- type constraints;
- semantic errors.

The lexer does none of these.

---

41. Canonical Quantum IR Contract

No IR is produced by this file.

The intended path is:

QUANTUM_LITERAL
      |
      v
frontend AST
      |
      v
semantic quantum representation
      |
      v
quantum::ir

"quantum::ir" remains the canonical quantum semantic boundary.

This file must never introduce:

QuantumLiteralIR
QuantumStateLexerIR
QuantumLiteralNode
PhysicalQuantumLiteralIR

as competing quantum representations.

---

42. QEC Integration

QEC is downstream.

This file does not encode:

surface code
color code
repetition code
LDPC
Steane
Shor
syndrome
decoder
logical error rate

A literal remains semantically independent of the QEC strategy.

The pipeline may later become:

quantum literal
    |
    v
quantum::ir
    |
    v
QEC analysis
    |
    v
logical/physical realization

but QEC does not modify lexical meaning.

---

43. ZQN Integration

ZQN owns quantum fault/noise semantics.

This file does not encode:

- noise probability;
- error channel;
- leakage;
- loss;
- erasure;
- correlated faults;
- calibration error;
- decoherence;
- fault location.

A literal:

|0⟩

has the same source-level meaning regardless of whether the eventual target is:

- ideal simulation;
- noisy simulation;
- superconducting QPU;
- trapped-ion QPU;
- photonic system;
- future quantum architecture.

---

44. Routing Integration

Routing is downstream.

This file must not encode:

physical_qubit(0)
physical_qubit(1)
coupling_map(...)

A quantum literal is not a routing instruction.

Routing determines physical realization after semantic lowering.

---

45. Scheduling Integration

Scheduling is downstream.

This file must not encode:

- gate duration;
- pulse duration;
- clock cycle;
- timing slot;
- hardware alignment;
- resource occupancy.

Quantum literals contain no scheduling semantics.

---

46. Optimization Integration

Optimization may transform the semantic representation derived from a quantum literal.

Examples include:

- simplification;
- canonicalization;
- state preparation optimization;
- decomposition;
- representation selection.

None of these transformations belong in lexical analysis.

---

47. Hardware/HAL Integration

Hardware discovery and capability negotiation belong downstream.

The literal does not identify:

QPU0
QPU1
device17
qubit42

The hardware abstraction layer determines available capabilities.

The resource layer determines whether the requested computation can be realized.

---

48. Runtime Integration

The runtime may choose:

- simulation;
- execution;
- remote execution;
- hybrid execution;
- distributed execution;
- accelerated execution.

The lexical representation must not change based on runtime target.

The same source must retain the same lexical meaning everywhere.

---

49. Interoperability

Quantum literals may eventually be lowered to:

- OpenQASM;
- QIR;
- backend-specific IR;
- simulator representations;
- other quantum formats.

Those are interoperability targets.

They are not lexical authorities for Zamani.

Zamani source syntax remains authoritative for Zamani source programs.

---

50. Dialects

A dialect may introduce additional quantum-state notation.

However, a dialect MUST NOT silently redefine stable core literals.

A dialect extension must declare:

dialect name
dialect version
syntax extension
lexer extension
AST mapping
semantic mapping
IR mapping
compatibility behavior
feature gate

A dialect must not create an incompatible second meaning for:

|0⟩
|1⟩
|+⟩
|-⟩

without an explicit language-version or dialect boundary.

---

51. Token Priority

"QUANTUM_LITERAL" must be integrated into the canonical lexer without unintended conflict with:

PIPE
PLUS
MINUS
INTEGER_LITERAL
IDENTIFIER
KET-related punctuation

The canonical composed lexer determines final token priority.

The rule must guarantee:

|0⟩

is recognized as one "QUANTUM_LITERAL" where the canonical lexical context permits it.

At the same time:

a | b

must remain ordinary operator syntax.

The lexer must not treat every "|" as the beginning of a quantum literal.

---

52. Context Sensitivity

The preferred design is to keep lexical recognition deterministic.

The lexer should recognize the compact literal based solely on its exact lexical sequence rather than requiring semantic knowledge such as:

is this identifier a qubit?
is this inside a quantum circuit?
is this target a QPU?

Semantic context must not be required to tokenize:

|0⟩

This ensures deterministic parsing.

---

53. No Semantic Lookahead

The lexer must not ask:

Does a qubit exist?
Does a quantum register exist?
Is a QPU available?
Is this state physically realizable?
Is the state normalized?
Can this state be simulated?

Those questions belong downstream.

---

54. Rust Implementation Contract

The implementation must target:

Rust 1.97

or:

Rust 1.97.1

with:

Rust 2021

and safe Rust only.

No "unsafe" is permitted.

Where lexical processing is implemented directly in Rust, the implementation should prefer:

&str
char
char_indices()
chars()
is_char_boundary()

and other safe UTF-8 APIs.

The implementation must never:

- slice a UTF-8 string at an arbitrary byte offset;
- construct invalid Unicode scalar values;
- assume one byte equals one character;
- use locale-dependent Unicode behavior;
- silently replace invalid source;
- panic on malformed user source.

---

55. UTF-8 Validation

If source enters the lexer as bytes, UTF-8 validation must occur before Unicode-sensitive lexical processing.

Invalid UTF-8 must result in a structured diagnostic.

It must not be:

- replaced;
- silently skipped;
- interpreted as another character;
- passed into semantic analysis as corrupted source.

If the public lexer API already receives "&str", UTF-8 validity is guaranteed by the Rust type system.

---

56. No Unsafe Rust

The quantum literal implementation requires no unsafe operations.

The project must maintain the repository-level requirement:

unsafe Rust is prohibited.

The lexical implementation must not introduce:

unsafe { ... }

or:

unsafe fn ...

for performance, Unicode processing, token construction, or source-span handling.

---

57. Resource Scalability

The lexer must not impose artificial limits on:

- number of quantum literals;
- number of source lines;
- number of quantum expressions;
- total source size;
- number of tokens;
- quantum state references;
- parser nesting;
- number of compilation units.

Practical resource limits may be imposed externally.

For example:

max_source_bytes
max_tokens
max_parser_memory
max_compilation_time

may exist as configurable resource policies.

They must not be hidden inside the lexical grammar.

---

58. "Infinity" Interpretation

Zamani's scalability requirement is interpreted as:

«No artificial finite language limit.»

It does not mean that a physical computer has infinite memory or infinite execution capacity.

Therefore:

unbounded by language design

is the required property.

Actual execution remains bounded by available resources.

This distinction applies to:

- quantum literals;
- qubits;
- state expressions;
- circuits;
- classical data;
- tensors;
- distributed nodes;
- memory;
- hardware resources.

---

59. Performance Requirements

Quantum literal recognition must be:

- deterministic;
- linear in source length;
- allocation-conscious;
- independent of target hardware;
- free from unnecessary Unicode normalization;
- free from repeated rescanning of the same source.

For a source of length "n", lexical processing should remain approximately:

O(n)

with respect to source size.

The grammar must not create pathological quadratic behavior merely because a source contains many quantum literals.

---

60. Memory Requirements

The lexer must not allocate a new semantic quantum-state object for every lexical token.

The lexical layer should represent:

token kind
source span
source text/reference

and defer semantic allocation to the frontend.

This keeps lexical processing scalable.

---

61. Determinism

For identical:

source bytes
+
language version
+
lexer configuration

the lexer must produce an identical token stream.

For example:

|0⟩

must always produce:

QUANTUM_LITERAL

with the same source span semantics.

No locale, host OS, hardware target, runtime state, or device availability may affect lexical classification.

---

62. Reproducibility

The lexical result must not depend on:

- CPU architecture;
- GPU;
- QPU;
- operating system;
- locale;
- installed quantum backend;
- hardware discovery;
- runtime configuration.

The same source must tokenize identically everywhere.

---

63. Compatibility

The following forms are stable:

|0⟩
|1⟩
|+⟩
|-⟩

A future language version must not silently reinterpret them.

If new quantum literal syntax is added, it must not make existing valid source tokenize differently unless the language version explicitly declares a breaking change.

---

64. Unicode Versioning

The language must not depend on host-specific Unicode behavior for the meaning of the canonical compact literal characters.

The characters used by the compact literal vocabulary have fixed Unicode scalar identities.

The implementation must use exact code-point matching.

No locale-specific behavior is permitted.

---

65. Security Requirements

The lexical implementation must protect against:

- Unicode confusable substitution;
- invisible-character confusion;
- malformed UTF-8;
- delimiter spoofing;
- source-span corruption;
- token-boundary confusion;
- accidental normalization;
- parser/lexer disagreement.

A visually similar character is not automatically equivalent to the canonical character.

Security tooling may additionally warn about confusables.

The lexer itself must preserve exact source semantics.

---

66. No Hidden Normalization

The following transformations are prohibited during lexing:

|0⟩ -> |0>
|０⟩ -> |0⟩

or any other visual/compatibility substitution.

Source text must remain exact.

---

67. Formatting Integration

The formatter may canonicalize source presentation only according to a separate formatting specification.

The lexer must never modify source.

For example, the formatter may eventually choose:

|0⟩

as canonical presentation.

That does not authorize the lexer to rewrite source.

---

68. IDE Integration

The lexer should expose enough information for IDE tooling to identify:

QUANTUM_LITERAL

as a distinct lexical category.

Tooling may provide:

- syntax highlighting;
- hover information;
- navigation;
- semantic information;
- state visualization.

Those features must not be embedded in the lexer.

---

69. Documentation Integration

Documentation generation may recognize:

QUANTUM_LITERAL

and render it as quantum notation.

Documentation must not infer hardware semantics from the token.

---

70. Testing Contract

The following test classes are mandatory.

Positive Tests

|0⟩
|1⟩
|+⟩
|-⟩

Each must produce:

QUANTUM_LITERAL

---

71. Positive Context Tests

Test literals in:

let state = |0⟩;
let state = |1⟩;
let state = |+⟩;
let state = |-⟩;

and in quantum state expressions.

---

72. Multiple-Literal Tests

Test:

|0⟩ |1⟩
|0⟩ ⊗ |1⟩
|+⟩ ⊗ |-⟩

The lexer must preserve the boundaries between individual literals.

---

73. Identifier Interaction Tests

Test:

state0
state1
q0
q1
zero
one
plus
minus

These must remain governed by identifier rules.

The lexer must not confuse them with compact quantum literals.

---

74. Operation Interaction Tests

Test:

H
X
CNOT
custom_operation
vendor_operation

None should become "QUANTUM_LITERAL".

---

75. Invalid Tests

Test:

|2⟩
|3⟩
|a⟩
|q⟩
|00⟩
|01⟩
|0>
|0
0⟩
||0⟩

The lexer must not incorrectly classify these as the stable compact quantum literal token.

---

76. Unicode Boundary Tests

Test:

- exact U+27E9;
- visually similar non-U+27E9 characters;
- malformed UTF-8;
- Unicode normalization variants;
- Unicode whitespace inserted into literals;
- Unicode newline characters;
- invisible characters.

Exact Unicode source semantics must be preserved.

---

77. Whitespace Tests

Reject:

| 0⟩
|0 ⟩

as compact quantum literals.

Do not silently remove whitespace.

---

78. Newline Tests

Reject cross-line compact literals:

|
0⟩

and:

|0
⟩

as one "QUANTUM_LITERAL".

---

79. Comment Tests

Reject/comment-separate:

|/*x*/0⟩
|0/*x*/⟩

as one quantum literal.

Verify that:

/*x*/
|0⟩

correctly produces a comment followed by a quantum literal.

---

80. Operator Boundary Tests

Test:

a | b
a || b
|0⟩
|0⟩ | 1⟩

to guarantee that ordinary pipe/logical operators remain distinct from compact quantum literals.

---

81. Source-Span Tests

For every valid compact literal, verify:

start == byte offset of '|'
end == byte offset immediately after '⟩'

The span must never split:

⟩

or any UTF-8 code point.

---

82. AST Tests

Verify:

|0⟩

produces the expected AST quantum-literal representation.

Verify that:

|0⟩ ⊗ |1⟩

produces two literal nodes plus the appropriate composition expression rather than one lexer-generated composite state.

---

83. Semantic Tests

Verify that:

|0⟩

can enter the quantum semantic pipeline.

Verify that the semantic layer—not the lexer—handles:

- state typing;
- dimensionality;
- contextual validity;
- composition;
- resource implications.

---

84. IR Integration Tests

Verify:

source
→ lexer
→ parser
→ AST
→ semantic model
→ quantum::ir

without creating a duplicate quantum IR.

The lexical layer must not bypass the frontend semantic boundary.

---

85. QEC Tests

Verify that adding or removing QEC configuration does not change lexical tokenization.

For example:

|0⟩

must tokenize identically regardless of:

surface-code configuration
logical-qubit configuration
decoder configuration
noise model

---

86. ZQN Tests

Verify that:

|0⟩

tokenization is independent of:

- noise;
- fault model;
- leakage;
- erasure;
- calibration;
- device reliability.

---

87. Hardware Tests

Verify that tokenization does not depend on:

- QPU availability;
- QPU count;
- physical qubit count;
- topology;
- native gate set;
- hardware generation;
- device identifier.

---

88. Scalability Tests

The test suite must include increasingly large source inputs containing:

|0⟩

and mixtures of all supported compact literals.

The purpose is to verify that no hidden constant exists such as:

MAX_QUANTUM_LITERALS

or:

MAX_SOURCE_SIZE

inside the grammar.

Tests should be resource-budgeted externally rather than embedding arbitrary language limits.

---

89. Stress Tests

Stress inputs should include:

- many consecutive quantum literals;
- large files;
- large numbers of quantum expressions;
- quantum literals mixed with classical expressions;
- quantum literals mixed with HDL expressions;
- quantum literals inside nested parser constructs;
- large module graphs;
- large generated sources.

The expected behavior is deterministic processing or explicit resource exhaustion diagnostics.

---

90. Fuzzing

The lexer must be fuzz-tested with:

- random Unicode;
- malformed UTF-8;
- random delimiters;
- random pipe characters;
- random Unicode lookalikes;
- truncated literals;
- oversized source;
- random quantum/classical mixtures.

Fuzzing must establish:

no panic
no unsafe behavior
no undefined behavior
no infinite loop
no silent source corruption

---

91. Property Tests

Useful invariants include:

valid compact literal
→ exactly one QUANTUM_LITERAL token

and:

different canonical literal spellings
→ deterministic distinct token text

and:

invalid compact form
→ never silently becomes a valid different compact literal

and:

source span
→ covers exactly the source literal

---

92. Round-Trip Tests

Where source-preserving tooling is supported:

source
→ lexer
→ parser
→ AST
→ formatter/source reconstruction

must preserve the semantic spelling of:

|0⟩
|1⟩
|+⟩
|-⟩

No ASCII replacement or Unicode normalization may occur implicitly.

---

93. Compatibility Matrix

The feature must be traceable through:

Layer| Contract
"unicode.md"| Unicode/scalar policy
"quantum-literals.md"| quantum literal policy
"quantum-literals.g4"| lexical implementation
canonical lexer| token composition
"tokens.g4"| token vocabulary
parser| syntactic consumption
frontend AST| literal representation
semantic layer| state meaning
"quantum::ir"| canonical quantum representation
optimization| optional transformation
routing| physical realization
scheduling| timing/resource ordering
QEC| error correction
ZQN| fault/noise semantics
HAL| hardware capabilities
runtime| execution

A feature is not complete if only the lexer rule exists.

---

94. Integration With "grammar/lexer/tokens.g4"

"tokens.g4" must contain exactly one canonical token contract for:

QUANTUM_LITERAL

This file defines its semantics and lexical meaning.

"tokens.g4" owns token declaration.

"quantum-literals.g4" owns lexical production.

No third file may redefine the same token independently.

---

95. Integration With "grammar/lexer/literals.md"

"literals.md" is the umbrella literal specification.

It must reference this file for:

QUANTUM_LITERAL

It must not redefine the quantum literal syntax independently.

The relationship is:

literals.md
    |
    +-- numeric literals
    +-- string literals
    +-- character literals
    +-- boolean literals
    +-- quantum literals
    +-- other formally defined literal classes

---

96. Integration With "grammar/lexer/unicode.md"

"unicode.md" owns:

- UTF-8;
- Unicode scalar values;
- Unicode normalization policy;
- source encoding;
- Unicode source boundaries;
- Unicode security rules.

This file references those rules.

It does not redefine them.

The quantum literal's "⟩" character is interpreted using the exact Unicode policy defined there.

---

97. Integration With "grammar/lexer/operators.md"

The pipe character:

|

may participate in ordinary operators or quantum notation.

"operators.md" owns ordinary operator meaning.

This file owns the compact literal combination:

|0⟩
|1⟩
|+⟩
|-⟩

The canonical lexer must resolve these without ambiguity.

---

98. Integration With "grammar/lexer/identifiers.md"

Identifiers remain independent.

This allows:

psi
state
zero_state
custom_state

to be ordinary identifiers.

The lexer must not convert arbitrary identifier names into quantum literals.

---

99. Integration With "grammar/lexer/diagnostics.md"

This file defines conditions requiring diagnostics.

"diagnostics.md" defines:

- diagnostic identifiers;
- severity;
- message format;
- source labels;
- related spans;
- recovery strategy.

The two documents must not duplicate diagnostic IDs.

---

100. Integration With "grammar/lexer/conformance.md"

"conformance.md" must include all quantum literal tests defined here.

At minimum:

|0⟩
|1⟩
|+⟩
|-⟩

plus malformed, Unicode, span, compatibility, determinism, and scalability cases.

---

101. Integration With "grammar/quantum/"

The lexical layer supplies:

QUANTUM_LITERAL

The parser-level quantum grammar consumes it.

The parser-level quantum grammar owns:

- state expressions;
- state references;
- state constructors;
- superposition;
- mixtures;
- tensor/product composition;
- transformations.

It must not recreate the lexical rule.

---

102. Integration With "grammar/quantum/quantum-states.g4"

"quantum-states.g4" must consume:

QUANTUM_LITERAL

rather than recreating:

'|' ...

for the stable compact forms.

Its state-expression grammar may build more general structures from ordinary tokens.

This preserves the distinction:

lexer:
    compact literal

parser:
    general state expression

semantic layer:
    quantum meaning

quantum::ir:
    canonical representation

---

103. Integration With "grammar/antlr/ZamaniLexer.g4"

The existing canonical lexer currently exposes "QUANTUM_LITERAL".

There must ultimately be exactly one authoritative implementation of that token.

If:

grammar/lexer/quantum-literals.g4

becomes the modular source of the rule, the canonical lexer must consume/generate that rule rather than independently maintaining another definition.

The migration must preserve existing source compatibility.

No duplicate independently maintained definitions are permitted.

---

104. Integration With "grammar/antlr/Quantum.g4"

Legacy or compatibility quantum grammar material must not independently redefine compact quantum literal semantics.

If it contains:

QUANTUM_LITERAL

or equivalent compact literal rules, they must be reconciled with the canonical lexer authority.

---

105. Integration With "grammar/grammar.md"

"grammar.md" must report the actual implementation status of:

QUANTUM_LITERAL

and distinguish:

specified
implemented
partial
planned
deprecated

It must not claim broader generalized quantum literal syntax merely because it is proposed here.

---

106. Integration With "Zamani-Grammar.md"

"Zamani-Grammar.md" may describe broader future quantum notation.

Such descriptions remain:

proposed
experimental
historical

until promoted through the normal feature lifecycle.

It cannot silently override this stable lexical contract.

---

107. Integration With "grammar/DESIGN.md"

"DESIGN.md" remains the highest-level architecture authority.

This file must obey its:

- one-language principle;
- no-hard-coded-limits rule;
- canonical IR rule;
- source-span rule;
- deterministic parsing rule;
- compatibility rule;
- dialect rule;
- generated-file rule.

---

108. Integration With Frontend AST

The frontend AST must remain domain-neutral where required by the repository architecture.

A quantum literal may be represented as a generic literal/operation/value node carrying quantum semantic information.

The lexer must not dictate a backend-specific AST.

Do not create a lexer-owned:

PhysicalQubitNode
QPUStateNode
SimulatorStateNode

---

109. Integration With Classical Computing

A quantum literal can occur in a hybrid program.

For example, a classical computation may prepare or consume quantum state information.

The lexer must remain independent of whether the surrounding program is:

classical
quantum
hybrid
distributed
AI
HDL

Lexical meaning must remain stable.

---

110. Integration With HDL

HDL syntax may coexist with quantum syntax in a hardware/software co-design program.

A quantum literal must not imply a physical HDL implementation.

For example:

|0⟩

does not specify:

wire
register
flip-flop
pulse
clock
physical qubit

Those belong to later domain-specific semantics.

---

111. Integration With Hardware Intent

Hardware requirements may be expressed elsewhere:

requires capability(...)
requires resource(...)
requires quantum(...)

A quantum literal does not itself constitute a hardware requirement.

This distinction is essential for POCO-REAF.

---

112. Integration With Resources

Resource analysis may determine that a program containing quantum state expressions requires:

- quantum compute;
- memory;
- communication;
- particular capabilities.

That analysis is downstream.

The lexer must not perform resource estimation.

---

113. Integration With Compile

The compiler may select:

- simulation;
- CPU;
- GPU;
- FPGA;
- QPU;
- distributed execution;
- future accelerators.

The quantum literal syntax remains unchanged.

---

114. Integration With Execution

Runtime execution may choose a realization appropriate to available resources.

The literal itself contains no runtime placement.

---

115. Integration With Interoperability

OpenQASM, QIR, simulator formats, or vendor representations may be generated downstream.

No interoperability format may redefine the meaning of Zamani's source lexical token.

---

116. Integration With Macros

Macros may generate quantum literals.

Generated source must pass through the same lexical/semantic contracts.

Macros must not bypass:

- Unicode validation;
- tokenization;
- AST validation;
- semantic analysis.

---

117. Integration With Metaprogramming

Metaprogramming may construct quantum-state syntax or AST fragments.

Generated constructs must still conform to the canonical quantum literal contract.

No metaprogramming feature may create an undocumented alternate spelling.

---

118. Integration With Dialects

A dialect may extend quantum syntax but must explicitly declare its lexical additions.

Dialect syntax must not silently change the core meaning of:

|0⟩
|1⟩
|+⟩
|-⟩

---

119. Integration With Compatibility

Stable compact quantum literals must remain source-compatible.

Future extensions must prefer:

new syntax

over silently changing:

existing syntax

when possible.

Breaking changes require an explicit language-version compatibility mechanism.

---

120. Integration With Validation

"grammar/validation/" must validate:

- duplicate "QUANTUM_LITERAL" definitions;
- token ambiguity;
- lexer/parser disagreement;
- unreachable quantum literal rules;
- Unicode mismatch;
- missing AST mapping;
- missing semantic mapping;
- missing IR mapping;
- missing tests;
- hidden fixed limits.

---

121. Hard-Coding Audit

This file passes the hardware hard-coding policy only if it contains no:

MAX_QUBITS
MAX_QPU
MAX_STATE_VECTOR
MAX_AMPLITUDES
MAX_REGISTER
MAX_QUBIT_INDEX
MAX_DEVICE
MAX_CIRCUIT
MAX_QUANTUM_MEMORY

as language limits.

The finite set:

0
1
+
-

is not a hardware limitation.

It is the deliberately finite vocabulary of the four compact state spellings.

---

122. Semantic Requirement Versus Implementation Decision

The language must distinguish:

Semantic requirement

state must represent |0⟩

from:

Resource requirement

requires quantum capability

from:

Preference

prefer quantum accelerator

from:

Implementation decision

map to physical resource X

The quantum literal belongs to the first category.

The others are handled downstream.

---

123. No Backend Vocabulary

This file must not add backend-specific literals such as:

ibm_qpu(...)
ionq_qpu(...)
rigetti_qpu(...)
cuda_qpu(...)
vendor_state(...)

unless a separately specified interoperability/dialect feature explicitly defines them.

Core Zamani syntax must remain target-independent.

---

124. No Gate Enumeration

Do not add gate alternatives to this file.

Never evolve it into:

quantumLiteral
    : H
    | X
    | Y
    | Z
    | CNOT
    | ...

Operations are not literals.

This would recreate the exact scalability problem the modular architecture is intended to eliminate.

---

125. No State-Vector Encoding

Do not encode:

[1,0]
[0,1]

as special quantum lexer forms.

These are ordinary collection/expression syntax unless a separate semantic feature gives them quantum-state meaning.

This prevents the lexer from deciding the internal mathematical representation of quantum states.

---

126. No Fixed Precision

The lexical form:

|0⟩

does not specify:

f32
f64
f128
arbitrary precision
fixed-point
symbolic

The semantic/compiler layers determine appropriate representation.

---

127. No Simulator Assumptions

The lexer must not know whether the program runs on:

state vector
density matrix
tensor network
stabilizer simulator
trajectory simulator
hardware

All are downstream realization choices.

---

128. No Physical State Allocation

Lexical recognition must not allocate quantum memory.

The lexer merely produces a token.

Memory allocation belongs to later layers.

---

129. Error Handling

Malformed input must produce structured diagnostics.

The implementation must not:

panic!

for ordinary malformed quantum source.

It must not use:

unwrap()
expect()
unreachable!()

as the normal mechanism for handling malformed user input.

Errors should propagate through the repository's established diagnostic/error model.

---

130. Performance and Allocation Discipline

The lexical implementation should avoid:

- per-character heap allocation;
- unnecessary string cloning;
- repeated normalization;
- repeated UTF-8 conversion;
- target discovery during lexing;
- semantic object construction.

The token should normally reference or span source text rather than copying large semantic representations.

---

131. Deterministic Tokenization Examples

These mappings are normative:

|0⟩  -> QUANTUM_LITERAL
|1⟩  -> QUANTUM_LITERAL
|+⟩  -> QUANTUM_LITERAL
|-⟩  -> QUANTUM_LITERAL

The exact token text and source span must be preserved.

---

132. Non-Examples

These are not "QUANTUM_LITERAL":

psi
|0>
|00⟩
|alpha⟩
q[0]
H
CNOT
superposition(...)
tensor(...)
measure q

They belong to other lexical/parser categories.

---

133. Boundary Examples

|0⟩x

must tokenize as:

QUANTUM_LITERAL
IDENTIFIER

if the general lexical rules permit that adjacency.

Similarly:

x|0⟩

must preserve the boundary between:

IDENTIFIER
QUANTUM_LITERAL

where the grammar permits the sequence.

No hidden whitespace insertion may occur.

---

134. Lexer/Parser Separation

The lexer answers:

«Is this exact source sequence a compact quantum literal?»

The parser answers:

«Where can this quantum literal occur syntactically?»

The semantic analyzer answers:

«What does this quantum state mean here?»

The compiler answers:

«How should that meaning be realized?»

The runtime answers:

«How should the realized computation execute?»

This separation must remain intact.

---

135. Feature Manifest Integration

A corresponding feature manifest should exist under:

grammar/specification/features/

with an identifier such as:

quantum-compact-literals

The manifest should record:

id
name
status
language_version
syntax
lexer
token
parser
ast
semantic_model
ir
compiler_consumers
runtime_consumers
domain
capabilities
negative_tests
boundary_tests
scalability_tests
compatibility
hard_coding_policy

The manifest is a traceability mechanism, not another grammar authority.

---

136. Completion Contract

This file is considered complete only when all of the following are true.

Specification

- [x] purpose defined;
- [x] authority defined;
- [x] ownership defined;
- [x] non-ownership defined;
- [x] compact literal syntax defined;
- [x] Unicode semantics defined;
- [x] malformed forms defined;
- [x] compatibility policy defined.

Lexer

- [x] token identified;
- [x] token boundary defined;
- [x] token priority defined;
- [x] source preservation defined;
- [x] no duplicate lexical authority permitted.

Parser

- [x] parser consumer defined;
- [x] generalized quantum state expressions separated from lexical literals;
- [x] composition behavior defined.

AST

- [x] AST responsibility defined;
- [x] source-span contract defined;
- [x] backend independence defined.

Semantic Layer

- [x] state meaning delegated downstream;
- [x] no semantic validation in lexer;
- [x] no resource allocation in lexer.

IR

- [x] canonical "quantum::ir" boundary preserved;
- [x] no duplicate quantum IR introduced.

QEC/ZQN

- [x] QEC ownership defined;
- [x] ZQN ownership defined;
- [x] no fault/noise semantics in lexer.

Hardware

- [x] no physical qubit allocation;
- [x] no QPU identifiers;
- [x] no topology;
- [x] no hardware limits.

Scalability

- [x] no artificial finite limits;
- [x] no fixed source-size limit;
- [x] no fixed quantum-resource limit;
- [x] resource limits delegated to external policies.

Rust

- [x] Rust 1.97 supported;
- [x] Rust 1.97.1 supported;
- [x] Rust 2021;
- [x] safe Rust only;
- [x] no "unsafe";
- [x] deterministic malformed-input handling.

Testing

- [x] positive tests;
- [x] negative tests;
- [x] Unicode tests;
- [x] source-span tests;
- [x] parser integration tests;
- [x] AST tests;
- [x] semantic tests;
- [x] IR integration tests;
- [x] scalability tests;
- [x] fuzzing requirements;
- [x] compatibility tests.

---

137. Final Normative Contract

The following rules are mandatory.

1. "QUANTUM_LITERAL" is the canonical token for stable compact quantum-state literals.

2. The stable compact forms are:
   
   |0⟩
|1⟩
|+⟩
|-⟩

3. Their exact Unicode source spelling is significant.

4. The lexer must not normalize or substitute visually similar characters.

5. General quantum states are parser/semantic constructs, not an ever-growing lexical token family.

6. Quantum operations are not quantum literals.

7. Qubit references are not quantum literals.

8. Physical qubit identifiers are not quantum literals.

9. QPU identifiers are not quantum literals.

10. Quantum literal syntax must not encode hardware limits.

11. Quantum literal syntax must not encode simulator limits.

12. Quantum literal syntax must not encode QEC semantics.

13. Quantum literal syntax must not encode ZQN semantics.

14. Quantum literal syntax must not encode routing.

15. Quantum literal syntax must not encode scheduling.

16. Quantum literal syntax must not encode calibration.

17. Quantum literal syntax must not encode backend-specific gate sets.

18. The lexer must remain deterministic.

19. The lexer must preserve source spans.

20. The lexer must preserve exact source spelling.

21. The parser owns broader quantum state-expression composition.

22. The semantic layer owns quantum meaning.

23. "quantum::ir" remains the canonical quantum semantic boundary.

24. No second quantum IR may be introduced by this feature.

25. Rust implementation must target Rust 1.97/1.97.1 and Rust 2021.

26. Rust "unsafe" is prohibited.

27. No artificial finite quantum-resource limit may be introduced.

28. Practical resource limits must remain configurable implementation/resource policies rather than language semantics.

29. Existing stable syntax must remain compatible.

30. Any future quantum literal extension must define its lexer, parser, AST, semantic, IR, compatibility, diagnostics, and scalability contracts before implementation.

---

138. Target End State

The completed architecture is:

                         Zamani Source
                              |
                              v
                   +----------------------+
                   | UTF-8 source layer   |
                   +----------------------+
                              |
                              v
                   +----------------------+
                   | Canonical Lexer      |
                   |                      |
                   | |0⟩                  |
                   | |1⟩                  |
                   | |+⟩                  |
                   | |-⟩                  |
                   +----------------------+
                              |
                              v
                    QUANTUM_LITERAL
                              |
                              v
                     Quantum Parser
                              |
             +----------------+----------------+
             |                                 |
             v                                 v
     compact literal                    general state
                                          expression
             |                                 |
             +----------------+----------------+
                              |
                              v
                        Frontend AST
                              |
                              v
                    Semantic Analysis
                              |
             +----------------+----------------+
             |                                 |
             v                                 v
      quantum semantics                 resource/capability
             |                                 |
             +----------------+----------------+
                              |
                              v
                         quantum::ir
                              |
             +----------------+----------------+
             |                |               |
             v                v               v
        Optimization       Routing       Scheduling
             |                |               |
             +----------------+----------------+
                              |
                              v
                    QEC / ZQN / Resilience
                              |
                              v
                             HAL
                              |
                              v
                    Target Realization
                              |
        +----------+----------+----------+----------+
        |          |          |          |          |
       CPU        GPU        FPGA       QPU      Future
        |          |          |          |        targets
        +----------+----------+----------+----------+
                              |
                              v
                           Runtime

The key architectural rule is therefore:

«A quantum literal is a portable description of quantum source meaning, not a description of a particular quantum machine.»

That keeps "grammar/lexer/quantum-literals.md" independently complete while allowing the quantum language to scale from a single abstract state to arbitrarily large quantum programs, subject only to actual resources rather than artificial grammar limits.