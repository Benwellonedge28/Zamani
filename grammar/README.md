
grammar/README.md

Zamani Grammar

Status: Production grammar architecture and conformance guide
Language: Zamani
Compiler baseline: Rust 1.97 / Rust 1.97.1
Safety: Safe Rust only — "unsafe" Rust is prohibited
Primary objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: From the smallest supported computation to arbitrarily large computation, subject only to available and explicitly declared resources.

---

1. Purpose

The "grammar/" directory defines and documents the Zamani source-language boundary.

The grammar is responsible for describing what Zamani source code looks like.

It must not make today's hardware, compiler implementation, simulator, quantum processor, CPU, GPU, memory capacity, qubit count, topology, gate set, operating system, or other target-specific limitation part of the language.

The architectural rule is:

Source
  ↓
Lexing
  ↓
Parsing
  ↓
AST
  ↓
Name / Module Resolution
  ↓
Semantic + Type + Effect + Capability Analysis
  ↓
Canonical IR
  ↓
Optimization
  ↓
Target-Independent Lowering
  ↓
Target-Specific Lowering
  ↓
Simulator / Emulator / CPU / GPU / QPU / Future Target

The grammar therefore expresses source-level intent.

It does not dictate how that intent must ultimately be implemented.

---

2. Core Design Principles

Zamani grammar development MUST follow these principles.

2.1 One language

There must be one canonical Zamani language.

The repository may contain multiple representations of the grammar for tooling purposes, but they must describe the same language rather than independent dialects.

In particular:

Zamani.g4
        ↓
ANTLR representation

grammar.md
        ↓
implementation-conformance documentation

Zamani-Grammar.md
        ↓
language-design/specification material

src/lexer.rs
        ↓
lexical implementation

src/parser.rs
        ↓
syntax implementation

src/ast/
        ↓
structural representation

src/semantic.rs
        ↓
semantic validity

src/ir_gen.rs
        ↓
lowering

src/quantum/ir/
        ↓
canonical quantum meaning

These components must converge on one language definition.

---

2.2 Syntax is not semantics

The grammar answers:

«Is this source syntactically valid Zamani?»

The AST answers:

«What source construct did the programmer express?»

Semantic analysis answers:

«Is that construct valid under Zamani's type, effect, ownership, capability, resource, and domain rules?»

IR answers:

«What computation does the program represent?»

Backends answer:

«How can that computation be realized on this target?»

No layer should silently assume responsibilities belonging to another layer.

---

2.3 No hardware-defined language limits

The grammar MUST NOT encode fixed limits such as:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_MEMORY
MAX_TENSORS
MAX_TIMELINES
MAX_AGENTS
MAX_DEVICES
MAX_GATE_COUNT
MAX_REGISTER_SIZE
MAX_MATRIX_DIMENSION

A program may contain a resource-dependent value such as:

register qubits = requested_qubits;

but the compiler must not define a permanent language-level ceiling merely because a particular machine has limited resources.

The actual executable may be constrained by:

- available memory;
- available CPU/GPU/QPU resources;
- target capabilities;
- compilation budgets;
- execution budgets;
- user-defined resource policies;
- numerical limits;
- operating-system limits;
- backend constraints.

Those are resource and target concerns, not grammar limits.

---

3. POCO-REAF

Zamani is designed around:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever.»

POCO-REAF does not mean that every physical machine can execute every program with unlimited resources.

It means that source semantics should remain independent of a particular machine.

The compiler must distinguish:

Language validity
        ≠
Semantic validity
        ≠
Compilation feasibility
        ≠
Target compatibility
        ≠
Resource availability
        ≠
Runtime availability

For example, a quantum program should not become invalid merely because the selected QPU has fewer physical qubits than the program requests.

Instead, the compiler should be able to determine whether the computation can be:

- executed directly;
- decomposed;
- mapped;
- routed;
- scheduled;
- distributed;
- simulated;
- executed on logical qubits;
- executed on another target;
- compiled under a different resource policy;
- or rejected with a precise capability/resource diagnostic.

---

4. Repository Grammar Authorities

The repository currently contains several grammar-related artifacts.

They must have clearly separated responsibilities.

"grammar/Zamani.g4"

The ANTLR grammar representation.

It exists for:

- ANTLR-based tooling;
- alternative parsers;
- IDE tooling;
- syntax analysis;
- syntax highlighting;
- parser generation;
- external language tooling;
- conformance testing.

It must remain synchronized with the canonical Zamani syntax.

It must not independently invent language features.

---

"grammar/grammar.md"

The implementation-conformance grammar.

This document should describe the syntax actually accepted by the reference compiler.

When "src/lexer.rs" or "src/parser.rs" changes accepted syntax, this document must be updated in the same change.

It is therefore an implementation snapshot, not permission to diverge from the canonical specification.

---

"grammar/Zamani-Grammar.md"

The broader language-design/specification document.

It may contain:

- proposed language capabilities;
- advanced language domains;
- future syntax;
- design rationale;
- experimental features;
- universal-computation concepts.

However, material described there MUST NOT be presented as implemented unless the corresponding compiler pipeline exists.

Every feature should have an explicit status such as:

PROPOSED
DESIGNED
GRAMMARIAN
LEXER_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
BACKEND_IMPLEMENTED
TESTED
STABLE
DEPRECATED

---

"grammar/README.md"

This file.

It defines:

- the architecture of the grammar directory;
- authority rules;
- synchronization requirements;
- conformance requirements;
- scalability requirements;
- integration with the compiler;
- testing requirements;
- production readiness criteria.

---

5. Current Compiler Integration

The grammar must remain synchronized with the actual compiler pipeline.

The repository currently contains:

src/lexer.rs
src/parser.rs
src/ast/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs

The lexer already defines structured tokens, including identifiers, literals, quantum literals, nano annotations, MTS-related tokens, operators, and keywords.

The parser is a hand-written recursive-descent / Pratt parser with explicit operator precedence and parsing support for core and domain-specific constructs.

The AST represents source constructs structurally and attaches "Span" information to nodes for diagnostics.

The IR generator translates AST constructs into a typed, SSA-like intermediate representation consumed by optimization and backend infrastructure.

Therefore:

grammar
   ↕
lexer
   ↕
parser
   ↕
AST
   ↓
semantic analysis
   ↓
IR

must be treated as one integrated compiler contract.

---

6. Canonical Ownership Model

Component| Owns| Must not own
"grammar/"| language syntax specification| runtime behavior
"Zamani.g4"| ANTLR syntax representation| semantic rules
"grammar.md"| implementation grammar documentation| future promises
"Zamani-Grammar.md"| language design/specification material| claims of implementation
"src/lexer.rs"| tokenization| semantic interpretation
"src/parser.rs"| syntax recognition| target lowering
"src/ast/"| source structure| hardware realization
"src/semantic.rs"| semantic/type validation| textual parsing
"src/ir_gen.rs"| AST→IR lowering| hardware scheduling
"src/ir_verify.rs"| IR correctness validation| source parsing
"src/quantum/ir/"| canonical quantum semantics| parser syntax
optimization| semantics-preserving transformation| source grammar
scheduling| timing/resource scheduling| language syntax
ZQN| quantum-noise semantics/execution concerns| duplicate core IR
hardware| target capabilities and realization| language definition

---

7. Lexer Contract

The lexer is the first executable language boundary.

It must provide:

- deterministic tokenization;
- source spans;
- stable token identity;
- Unicode-aware source handling where supported;
- precise lexical diagnostics;
- deterministic handling of malformed input;
- no hidden global compiler state;
- no target-specific behavior.

The lexer must not silently create multiple meanings for the same lexical form.

Duplicate or overlapping token concepts must be reviewed and normalized.

For example, the current lexer contains closely related token categories such as:

BitAnd / Ampersand
BitOr  / Pipe
Question / QuestionMark

These require an explicit lexical ownership rule.

A token must have exactly one canonical meaning at a given syntactic position.

---

8. Lexer and Specification Synchronization

A token documented in the grammar must satisfy:

Specified
   ↓
Lexically recognized
   ↓
Parser-consumable
   ↓
AST-representable
   ↓
Semantically validated
   ↓
Lowerable
   ↓
Tested

A declared token that is not emitted by the lexer must never be documented as fully implemented.

For example, "MTS_LITERAL" is currently represented in the token model but has historically been documented as not yet emitted by the reference lexer.

Such states must be explicit.

---

9. Parser Contract

The reference parser is a hand-written recursive-descent / Pratt parser.

Its design must remain:

- deterministic;
- recoverable;
- span-aware;
- allocation-conscious;
- safe Rust;
- free of machine-size assumptions;
- independent of target hardware.

The parser must never use:

unsafe
unsafe fn
unsafe impl
unsafe { ... }

---

10. Error Recovery

Parser recovery must always make progress.

An invalid token sequence must not cause an infinite loop.

Recovery should:

1. identify the unexpected token;
2. record a structured diagnostic;
3. consume enough input to make progress;
4. synchronize at an appropriate grammar boundary;
5. continue when safe;
6. avoid manufacturing semantically valid constructs from invalid source.

Recovery nodes must never silently become executable semantics.

---

11. Deep-Program Scalability

Zamani must not assume that programs are shallow.

The implementation should avoid unnecessary recursive algorithms where input size could cause stack exhaustion.

Where practical, use:

- iterative traversal;
- explicit worklists;
- explicit stacks;
- streaming;
- incremental processing;
- lazy representations;
- bounded diagnostic accumulation;
- configurable resource budgets.

A program should fail because a declared or available resource budget has been exceeded—not because an arbitrary compiler constant was chosen.

---

12. Grammar Modularity

The grammar should be modular conceptually and, where practical, physically.

A scalable grammar organization is:

grammar/
├── README.md
├── DESIGN.md
├── Zamani.g4
├── Zamani-Grammar.md
├── grammar.md
│
├── spec/
│   ├── lexical.md
│   ├── syntax.md
│   ├── semantics.md
│   ├── type-system.md
│   ├── effects.md
│   ├── quantum.md
│   ├── modules.md
│   ├── compatibility.md
│   └── conformance.md
│
├── antlr/
│   ├── ZamaniLexer.g4
│   └── ZamaniParser.g4
│
├── reference/
│   └── ...
│
└── tests/
    ├── valid/
    ├── invalid/
    ├── quantum/
    ├── mathematics/
    ├── types/
    ├── effects/
    ├── modules/
    ├── scaling/
    └── compatibility/

This is an architectural target.

Directories and files should be introduced when their ownership is sufficiently defined rather than creating empty structure merely for appearance.

---

13. One Canonical Grammar Model

Modularity must not create multiple grammars.

The following are prohibited:

Core grammar
+
Quantum grammar
+
Math grammar
+
Nano grammar

where each independently defines a different language.

Instead:

Canonical Zamani Grammar
        │
        ├── Core syntax
        ├── Type syntax
        ├── Module syntax
        ├── Effect syntax
        ├── Quantum syntax
        ├── Mathematical syntax
        ├── Concurrency syntax
        ├── Meta syntax
        └── Domain extensions

All domains share:

- lexical rules;
- identifiers;
- literals;
- expressions;
- types;
- declarations;
- source spans;
- attributes;
- diagnostics;
- versioning;
- semantic resolution.

---

14. Quantum Grammar

Quantum computing is a first-class Zamani domain.

However, the grammar must not become a hard-coded list of today's hardware gates.

The language should express semantic intent.

Conceptually:

apply operation H to q;
apply operation controlled(X) from control to target;
apply operation U(theta, phi, lambda) to q;
measure q;
reset q;

The final concrete Zamani syntax is subject to the canonical language specification, but the architectural requirement is fixed:

«Source syntax must represent quantum intent rather than a particular hardware gate catalog.»

---

15. Quantum Scalability

The grammar must not contain:

Qubit0
Qubit1
...
QubitN

as a compiler-defined universe.

Nor may it define:

MAX_QUBITS = 1024

or an equivalent hidden limitation.

Quantum programs should be expressible using:

- symbolic sizes;
- runtime-known sizes where supported;
- logical resources;
- dynamically discovered target capabilities;
- abstract registers;
- parameterized circuits;
- generic operations;
- multi-qubit operations;
- controlled operations;
- adjoints/inverses;
- measurement;
- reset;
- classical feed-forward;
- logical qubits;
- physical mapping;
- error-correction constructs;
- noise-aware execution metadata.

---

16. Quantum Operation Vocabulary

The source language must not require every possible operation to become a new grammar keyword.

Avoid a grammar such as:

gate
    : 'hadamard'
    | 'cnot'
    | 'paulix'
    | 'pauliy'
    | 'pauliz'
    | 't'
    | 's'
    | 'swap'
    | ...

Such a grammar becomes a maintenance bottleneck and couples syntax to hardware evolution.

Instead, prefer a compositional operation model.

Conceptually:

operation
operation-name
operation-arguments
targets
controls
modifiers
parameters

Semantic validation determines whether the operation is defined.

Lowering determines how it can be realized.

---

17. Canonical Quantum IR Boundary

Quantum syntax must eventually lower into the repository's canonical quantum IR.

The intended flow is:

Zamani Quantum Source
        ↓
Frontend AST
        ↓
Semantic Quantum Model
        ↓
quantum::ir
        ↓
Optimization
        ↓
Routing
        ↓
Scheduling
        ↓
Noise / resilience analysis
        ↓
Hardware lowering

Optimization, scheduling, ZQN, hardware, calibration, and benchmarking must consume canonical IR rather than inventing incompatible temporary representations.

The grammar must therefore remain completely independent of:

- QPU topology;
- native gate sets;
- pulse implementations;
- calibration data;
- coupling maps;
- physical qubit identifiers;
- backend-specific instruction encodings.

---

18. Quantum Resource Semantics

A quantum program can require resources without embedding the resources into grammar.

For example:

required_qubits = symbolic_expression;
required_depth  = symbolic_expression;
required_memory = symbolic_expression;

Resource analysis belongs to semantic/compiler infrastructure.

The grammar merely parses the corresponding source constructs.

---

19. Mathematical Syntax

Zamani's mathematical ambitions are broad, including:

- vectors;
- matrices;
- tensors;
- symbolic mathematics;
- calculus;
- statistics;
- numerical methods;
- signal processing;
- optimization;
- quantum-enhanced mathematics.

However, the grammar should not become an ever-growing dictionary of mathematical keywords.

Prefer:

generic syntax
     ↓
typed mathematical representation
     ↓
mathematical semantic operation
     ↓
intrinsic/library implementation
     ↓
optimization
     ↓
backend

For example, mathematical operations should be expressible through compositional expressions and typed APIs rather than requiring every new algorithm to become a reserved keyword.

---

20. Types

The grammar must provide a compositional type syntax.

It should be capable of representing the type-system concepts actually implemented or formally specified, including where applicable:

- primitive types;
- named types;
- generic types;
- tuples;
- arrays;
- slices;
- functions;
- references;
- optional types;
- result types;
- never type;
- quantum types;
- linear types;
- affine types;
- effectful types;
- domain-specific types.

Advanced type concepts must not be added merely because they appear in a design document.

Each feature must have a complete pipeline:

Grammar
→ Parser
→ AST
→ Semantic Analysis
→ Type Representation
→ IR
→ Verification
→ Tests

---

21. Patterns

Pattern syntax should be compositional.

Supported patterns should be represented structurally rather than encoded as special-case parser behavior.

Potential categories include:

wildcard
identifier
literal
tuple
array
struct
enum
type
or
range
reference

Pattern semantics such as:

- exhaustiveness;
- unreachable cases;
- binding;
- type compatibility;
- ownership;

belong to semantic analysis.

---

22. Modules and Imports

Grammar owns the syntax of modules and imports.

It must not own:

- filesystem traversal;
- network access;
- package downloads;
- registry authentication;
- dependency resolution;
- cryptographic verification;
- sandbox policy.

Those belong to compiler/package-management infrastructure.

Conceptually:

Source syntax
      ↓
Module declaration/import syntax
      ↓
AST
      ↓
Module resolver
      ↓
Package/dependency system

---

23. Effects and Capabilities

Effects should be represented explicitly where the language supports them.

Grammar may express:

effect declarations
effect annotations
effect handlers
effect-bearing functions

but semantic analysis must determine:

- whether an effect exists;
- whether it is permitted;
- whether it is handled;
- whether capabilities are available;
- whether a target supports the operation.

This allows source programs to describe intent without embedding environmental assumptions.

---

24. Attributes and Metadata

Attributes must use a structured representation.

They should not become an unrestricted escape hatch that allows arbitrary parser behavior.

Attributes may describe:

- optimization intent;
- compilation policy;
- diagnostics;
- ABI;
- target preferences;
- resource requirements;
- quantum properties;
- mathematical properties;
- experimental status.

Semantic ownership must be explicit.

---

25. Unsafe Policy

Zamani compiler implementation code must use safe Rust.

Target baseline:

Rust 1.97
or
Rust 1.97.1

The grammar documentation, grammar tooling, parser, AST, semantic analysis, and compiler integration must not require Rust "unsafe".

The following are prohibited:

unsafe { ... }
unsafe fn ...
unsafe impl ...

If a dependency internally uses unsafe implementation details, that does not authorize Zamani source/compiler code to introduce unsafe Rust.

---

26. Determinism

For identical:

source
language version
compiler version
configuration
dependency graph
target description
resource policy

the compiler should produce deterministic results wherever determinism is part of the relevant compilation contract.

Grammar parsing must not depend on:

- hash-map iteration order;
- machine size;
- host CPU;
- host OS;
- thread scheduling;
- wall-clock time;
- network availability.

Randomized or distributed compilation mechanisms must use explicit deterministic seeds or reproducibility metadata when reproducibility is required.

---

27. Diagnostics

Every lexical and syntactic diagnostic should carry enough source information to locate the problem.

The AST already uses "Span" information, which should remain the foundation for precise diagnostics.

Diagnostics should support:

error code
message
primary span
secondary spans
labels
notes
help
suggestions

Errors must never be communicated by:

- comments pretending to be executable output;
- stdout side effects;
- silently ignored syntax;
- fabricated AST nodes.

---

28. Versioning

The language grammar must be explicitly versioned.

A grammar change must be classified as:

additive
compatible
conditionally compatible
deprecated
breaking
experimental

Changes must be tracked across:

lexer
parser
AST
semantic analysis
IR
ANTLR grammar
conformance tests
documentation

A source program must not change meaning silently merely because a compiler upgrade changed an unrelated grammar rule.

---

29. Feature Status

Every significant language feature should have a status.

Recommended vocabulary:

Status| Meaning
"PROPOSED"| Design idea only
"SPECIFIED"| Formal language design exists
"LEXER"| Lexical implementation exists
"PARSER"| Parser implementation exists
"AST"| AST representation exists
"SEMANTIC"| Semantic validation exists
"IR"| Lowering exists
"BACKEND"| At least one backend realizes it
"TESTED"| Conformance tests exist
"STABLE"| Public supported language feature
"EXPERIMENTAL"| Implemented but not stable
"DEPRECATED"| Supported temporarily
"REMOVED"| No longer accepted

This prevents the broad design documents from being mistaken for compiler capabilities.

---

30. ANTLR Conformance

"Zamani.g4" must be treated as a generated/tooling representation of the canonical syntax.

ANTLR-generated parsers must be tested against the same corpus used by the reference Rust compiler.

The minimum conformance process is:

Source corpus
      ↓
Reference Rust lexer/parser
      ↓
Reference result

Source corpus
      ↓
ANTLR lexer/parser
      ↓
ANTLR result

Compare normalized results

The comparison should cover:

- acceptance;
- rejection;
- token boundaries;
- parse structure;
- source spans where comparable;
- error classification where defined.

A difference must be investigated rather than automatically declaring either implementation correct.

---

31. Existing ANTLR Validation

The existing grammar README documents validation of "Zamani.g4" using ANTLR 4.13.1 and comparison against the real compiler. It also records known limitations including MTS literal emission, structural match patterns, optional-type sugar, and identifier handling for certain reserved words.

Those limitations must remain tracked until the corresponding implementation/specification changes are completed.

Validation must not be treated as permanent: every grammar or parser change requires revalidation.

---

32. Conformance Corpus

Grammar tests should be organized as:

grammar/tests/
├── valid/
├── invalid/
├── lexical/
├── expressions/
├── declarations/
├── modules/
├── types/
├── patterns/
├── effects/
├── quantum/
├── mathematics/
├── concurrency/
├── temporal/
├── nano/
├── sankofa/
├── scaling/
└── compatibility/

Each test should have one clear purpose.

Tests should cover both:

smallest useful program

and:

large structurally generated program

---

33. Scaling Tests

Production grammar testing must include scale testing.

The test suite must verify that the implementation does not contain hidden fixed limits on:

- source file size;
- declaration count;
- expression count;
- nesting;
- quantum resource count;
- tensor dimensions;
- module count;
- function count;
- generic parameters;
- pattern count;
- timeline count;
- agent count.

The practical maximum is determined by:

available resources
+
explicit compiler limits
+
explicit user policy

rather than accidental constants in the grammar.

---

34. Resource Limits

Production systems still require resource protection.

A compiler may expose explicit budgets such as:

maximum compilation time
maximum memory
maximum diagnostics
maximum expansion
maximum generated artifact size
maximum recursion/work depth

These must be:

1. explicit;
2. configurable where appropriate;
3. diagnosable;
4. separate from language syntax;
5. documented as resource policies.

A resource budget is not a grammar limitation.

---

35. Infinite-Scale Principle

“Scale to infinity” means:

«No artificial language-level machine-size ceiling is embedded in Zamani.»

It does not mean that a physical computer has infinite memory or execution capacity.

Therefore:

Zamani source
     ↓
unbounded semantic model
     ↓
resource-aware compilation
     ↓
available target resources

The compiler should scale as far as its actual resources and configured policies permit.

---

36. AST Contract

Every grammar construct must have a clear relationship with the AST.

The contract is:

syntax construct
      ↓
AST node

or, where syntactic sugar is intentional:

syntax construct
      ↓
desugaring
      ↓
canonical AST representation

Every AST node should have:

- explicit ownership;
- deterministic structure;
- source-span information;
- no hidden hardware state;
- no implicit global state;
- a semantic interpretation.

The existing AST already covers a broad range of source constructs and uses spans as part of its node representation.

---

37. AST Must Not Become a Hardware Model

The AST should represent source meaning.

It must not contain target-specific structures such as:

IBMQQubit
NvidiaQuantumQubit
RigettiQubit
specific_device_gate
specific_chip_topology
physical_pulse
hardware_register_7

Those belong to later compiler stages.

---

38. IR Integration

The grammar is upstream of IR.

The repository's IR generator currently translates AST types and constructs into an SSA-like typed IR.

As the language grows, every new grammar feature must answer:

What AST represents it?
What semantic rules validate it?
What IR represents it?
How is it verified?
Which backend(s) can realize it?
What happens when the target lacks a required capability?

A syntax feature without a semantic and IR plan is not production-ready.

---

39. Quantum IR Integration

Quantum source must ultimately have one canonical semantic boundary.

The preferred architecture is:

Zamani Source
      ↓
Quantum AST
      ↓
Quantum Semantic Analysis
      ↓
quantum::ir
      ↓
Quantum Optimization
      ↓
Routing
      ↓
Scheduling
      ↓
ZQN / resilience / noise-aware processing
      ↓
Hardware abstraction
      ↓
Target implementation

No grammar rule should bypass this architecture.

---

40. Target Independence

The grammar must not contain syntax whose only purpose is to expose one machine's internal implementation.

Target-specific source constructs, where genuinely necessary, must be represented as explicit target/capability annotations or separate target configuration—not hidden grammar assumptions.

This enables the same semantic program to target:

CPU
GPU
TPU
FPGA
QPU
photonic system
simulator
emulator
distributed system
future computational substrate

without rewriting the language around each target.

---

41. Mathematical, Quantum, Nano, Temporal and Cognitive Domains

Zamani may eventually contain very broad computational domains.

These domains must follow the same architecture:

domain syntax
      ↓
AST
      ↓
semantic model
      ↓
canonical IR/domain representation
      ↓
optimization
      ↓
execution/lowering

This applies to:

- quantum computing;
- mathematical computing;
- nano computation;
- temporal/MTS computation;
- Sankofa memory;
- AI/cognitive computation;
- effects;
- distributed computation;
- HDL;
- metaprogramming;
- other future domains.

A new domain must not establish an independent compiler architecture.

---

42. Grammar Evolution Rules

Before adding a keyword, ask:

1. Can existing expression syntax represent the concept?
2. Can an existing type represent it?
3. Can an existing attribute represent it?
4. Can a library/intrinsic provide it?
5. Does it require a genuinely new semantic category?
6. Does the AST need a new node?
7. Does the IR need a new representation?
8. Is the feature target-independent?
9. How will it scale?
10. What is its compatibility policy?

Keywords should be introduced only when they materially improve the language model.

---

43. Avoid Grammar Keyword Explosion

The following pattern should be avoided:

keyword_for_every_algorithm
keyword_for_every_gate
keyword_for_every_hardware
keyword_for_every_math_operation
keyword_for_every_backend

Prefer compositional constructs.

This allows Zamani to evolve without repeatedly breaking the lexical grammar.

---

44. Security and Isolation

The grammar must never implicitly perform:

- network access;
- filesystem access;
- code execution;
- package downloads;
- external process execution;
- credential access.

Parsing source must remain a deterministic compiler operation.

Any capability requiring external effects must be represented explicitly and handled by the appropriate compiler/runtime subsystem.

---

45. Package and Dependency Syntax

Package declarations and dependency syntax may exist at the language/tooling boundary, but package resolution must remain outside the parser.

The parser should produce structured information.

The package manager/compiler tooling determines:

- registry;
- dependency versions;
- integrity;
- signatures;
- source locations;
- sandbox policy;
- build strategy;
- caching;
- reproducibility.

---

46. Backward Compatibility

When syntax changes:

old source
    ↓
compatibility analysis

must determine whether the source remains valid and whether its meaning changes.

For breaking changes, provide:

- migration documentation;
- diagnostics;
- replacement syntax;
- compatibility mode where justified;
- version gates where necessary.

---

47. Production Definition of Done

The grammar system is production-ready only when all of the following are true:

- one canonical language specification exists;
- lexer and parser conform to it;
- ANTLR grammar conforms to it;
- implementation documentation matches reality;
- aspirational features are clearly marked;
- every supported syntax construct has AST representation;
- every semantic feature has semantic validation;
- every lowerable feature has an IR representation;
- quantum syntax lowers through canonical quantum IR;
- no hardware size is hard-coded into grammar;
- no fixed qubit limit is encoded into syntax;
- no target-specific gate catalog defines the language;
- no Rust "unsafe" is required;
- parser errors are structured and span-aware;
- parser recovery always makes progress;
- deep programs do not depend on arbitrary stack depth;
- resource limits are explicit;
- conformance tests exist;
- negative tests exist;
- scalability tests exist;
- compatibility tests exist;
- ANTLR and Rust parser behavior is continuously compared;
- language versioning is explicit;
- documentation distinguishes implemented and proposed features;
- deterministic compilation is supported where required.

---

48. Recommended Development Order

Grammar work should proceed in this order:

1. Freeze the language/version contract
2. Establish canonical lexical rules
3. Reconcile lexer token definitions
4. Establish canonical syntax
5. Reconcile parser behavior
6. Reconcile AST representation
7. Establish semantic/type contracts
8. Establish IR contracts
9. Establish quantum semantic/IR contracts
10. Reconcile Zamani.g4
11. Reconcile grammar.md
12. Classify Zamani-Grammar.md features
13. Build conformance corpus
14. Add negative tests
15. Add scaling tests
16. Add ANTLR-vs-reference tests
17. Add compatibility tests
18. Add deterministic compilation tests
19. Add resource-budget tests
20. Only then expand the language

This order prevents new syntax from multiplying existing inconsistencies.

---

49. File Integration Matrix

File| Responsibility| Required relationship
"grammar/README.md"| Grammar architecture| Defines this contract
"grammar/DESIGN.md"| Detailed grammar design| Must conform to README
"grammar/Zamani.g4"| ANTLR syntax| Must conform to canonical syntax
"grammar/grammar.md"| Current implementation grammar| Must reflect compiler behavior
"grammar/Zamani-Grammar.md"| Broad language design| Must classify implemented/proposed features
"src/lexer.rs"| Lexing| Must conform to lexical specification
"src/parser.rs"| Parsing| Must conform to syntax specification
"src/ast/"| AST| Must represent canonical source structure
"src/semantic.rs"| Semantics| Must validate AST meaning
"src/ir_gen.rs"| AST→IR| Must lower supported constructs
"src/ir_verify.rs"| IR validation| Must verify generated IR
"src/quantum/ir/"| Quantum IR| Canonical quantum semantic boundary
"src/quantum/optimization/"| Optimization| Must consume canonical IR
"src/quantum/scheduling/"| Scheduling| Must consume semantic/quantum IR
"src/quantum/zqn/"| Noise-aware quantum processing| Must not duplicate core IR
"src/quantum/hardware/"| Hardware realization| Must remain downstream of language semantics

---

50. Final Architecture

The production Zamani grammar architecture is:

                         ZAMANI LANGUAGE
                               │
                               ▼
                    ┌────────────────────┐
                    │ Canonical Grammar  │
                    └─────────┬──────────┘
                              │
               ┌──────────────┴──────────────┐
               ▼                             ▼
        Rust Reference                  ANTLR Tooling
        Lexer + Parser                     Grammar
               │                             │
               └──────────────┬──────────────┘
                              ▼
                         Frontend AST
                              │
                              ▼
                 Name / Type / Effect /
                 Capability Analysis
                              │
                              ▼
                    Canonical Semantic IR
                       │              │
                       │              └── Classical/Data
                       │
                       └── Quantum IR
                              │
                 ┌────────────┼────────────┐
                 ▼            ▼            ▼
            Optimization   Routing     Scheduling
                 │            │            │
                 └────────────┼────────────┘
                              ▼
                         ZQN / Resilience
                              │
                              ▼
                       Hardware Abstraction
                              │
              ┌───────────────┼────────────────┐
              ▼               ▼                ▼
           Simulator        CPU/GPU            QPU
              │               │                │
              └───────────────┼────────────────┘
                              ▼
                     Future Targets

The grammar is therefore the beginning of the compiler contract, not the end of the computational model.

---

51. The Zamani Grammar Principle

The central rule is:

«Express intent once. Represent meaning canonically. Defer physical realization. Scale with available resources. Preserve semantics across targets. Version the language explicitly.»

Zamani source should describe what computation means, not what today's machine happens to look like.

That is the grammar architecture required for:

atom → everywhere

and:

Program Once → Compile Once → Run Everywhere → Anywhere → Forever.

---This version deliberately makes the README a governance and integration document, rather than duplicating the entire grammar. That is important because the current Zamani.g4 is already very large and mixes core syntax with mathematical and domain-specific constructs, while the actual Rust lexer/parser/AST/IR pipeline has its own concrete contracts.

