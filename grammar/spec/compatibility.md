

Zamani Compatibility Specification

Path: "grammar/spec/compatibility.md"
Language: Zamani
Specification role: Normative compatibility, conformance, migration, and implementation-coherence contract
Compiler baseline: Rust 1.97 / Rust 1.97.1
Rust safety requirement: "unsafe" Rust is prohibited
Scalability requirement: No artificial language-level machine-size ceiling
Execution principle: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the compatibility contract between every layer that participates in the Zamani language:

Source
  │
  ▼
Lexical Specification
  │
  ├──────────────► Rust Lexer
  │
  └──────────────► ANTLR Lexer
  │
  ▼
Parser
  │
  ▼
Frontend AST
  │
  ▼
Name / Semantic Resolution
  │
  ▼
Type / Effect / Resource Validation
  │
  ▼
Canonical Semantic IR
  │
  ▼
IR Verification
  │
  ▼
Optimization / Lowering
  │
  ▼
Target-independent representation
  │
  ▼
Target-specific realization
  │
  ├── CPU
  ├── GPU
  ├── accelerator
  ├── simulator
  ├── emulator
  ├── QPU
  ├── photonic substrate
  ├── distributed machine
  └── future computational substrate

The purpose of this document is to prevent the following failure:

Zamani.g4       accepts A
lexer.rs        accepts B
parser.rs       accepts C
AST             represents D
semantic.rs     understands E
ir_gen.rs       lowers F
IR verifier     expects G
backend         executes H

Such divergence is prohibited for production Zamani.

There MUST be one language contract and one compatibility model.

Individual implementation files may have different responsibilities, but they MUST NOT silently define competing languages.

---

2. Normative Language Authority

Zamani uses a layered authority model.

The authorities are ordered as follows.

1. Versioned Zamani Language Specification
2. Canonical lexical and syntactic specifications
3. Reference Rust implementation
4. Canonical AST and semantic model
5. Canonical IR contract
6. ANTLR compatibility grammar
7. Generated/tooling representations
8. Historical or aspirational documentation

This ordering exists to separate:

- what the language means;
- what the current compiler implements;
- what tooling accepts;
- what future versions may introduce.

2.1 Canonical specifications

The canonical specification is distributed across:

grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/spec/semantics.md
grammar/spec/compatibility.md

These files form one specification, not four independent languages.

Their responsibilities are:

File| Responsibility
"lexical.md"| source characters, tokens, literals, comments, keywords
"syntax.md"| syntactically valid Zamani programs
"semantics.md"| meaning and semantic validity
"compatibility.md"| compatibility, versioning, implementation conformance, migration

---

3. Implementation Authority

The reference compiler implementation is authoritative for the syntax that is actually implemented.

The primary implementation boundaries are:

src/lexer.rs
src/parser.rs
src/ast/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs

The repository's implementation grammar explicitly identifies:

src/lexer.rs
src/parser.rs
src/ast/mod.rs
src/semantic.rs
src/ir_gen.rs

as the corresponding implementation boundaries.

Therefore:

lexer.rs
    └── lexical implementation

parser.rs
    └── syntactic implementation

src/ast/
    └── structural representation

semantic.rs
    └── semantic validation

ir_gen.rs
    └── AST → IR lowering

ir_verify.rs
    └── IR correctness validation

The implementation grammar in:

grammar/grammar.md

MUST describe the implemented language.

It MUST NOT silently advertise future syntax as implemented syntax.

---

4. Historical and Aspirational Specifications

The repository contains broader language-design material, including:

grammar/Zamani.g4
grammar/Zamani-Grammar.md

These files may contain syntax or capabilities that are broader than the currently implemented compiler.

They therefore MUST NOT independently establish compiler compatibility.

In particular:

Zamani-Grammar.md

MUST NOT cause a feature to be considered implemented merely because it is documented there.

Likewise:

grammar/Zamani.g4

MUST NOT silently become a second parser authority.

Its role is ANTLR compatibility and tooling unless explicitly promoted through the language-evolution process defined below.

---

5. Single-Language Rule

Zamani is one language.

The following are prohibited:

ANTLR Zamani
Rust-parser Zamani
AST Zamani
Quantum Zamani
Math Zamani
NIMBUS Zamani
Future Zamani

being treated as separate dialects without explicit language-version declarations.

Instead:

Zamani
 ├── core
 ├── quantum
 ├── mathematics
 ├── effects
 ├── temporal
 ├── nano
 ├── distributed
 ├── metaprogramming
 └── future domains

are semantic domains of the same language.

---

6. Compatibility Levels

Every Zamani construct MUST have one of the following statuses.

6.1 "STABLE"

The construct is:

- specified;
- implemented;
- tested;
- semantically defined;
- represented by the AST;
- representable in the canonical IR where applicable;
- compatible with the current language version.

Stable constructs MAY be relied upon by production programs.

---

6.2 "IMPLEMENTED"

The construct is accepted by the reference compiler but has not yet completed the stability process.

Implemented constructs MUST be explicitly documented.

They MUST NOT be presented as stable.

---

6.3 "SPECIFIED"

The construct has a normative specification but is not yet implemented.

A specified-only feature MUST NOT cause the compiler to accept source code.

---

6.4 "EXPERIMENTAL"

The construct is implemented for experimentation and is explicitly versioned or feature-gated.

Experimental syntax MUST NOT silently alter stable grammar.

---

6.5 "DEPRECATED"

The construct remains recognized for compatibility but has a documented replacement.

Deprecation MUST specify:

- first deprecated version;
- replacement;
- migration procedure;
- removal policy.

---

6.6 "REMOVED"

The construct is no longer accepted.

A removed construct MUST produce a deterministic diagnostic rather than being silently reinterpreted.

---

6.7 "RESERVED"

The spelling is reserved for future use but is not currently accepted as a language construct.

Reserved syntax MUST produce a clear diagnostic when encountered where an identifier or construct is expected.

---

7. Compatibility Matrix

Every language feature SHOULD be tracked using the following model.

Feature
├── specification status
├── lexer status
├── parser status
├── AST status
├── semantic status
├── IR status
├── verifier status
├── backend status
├── tests
├── language version
└── compatibility status

Example:

Feature| Lex| Parse| AST| Semantic| IR| Verify| Status
"fn"| yes| yes| yes| yes| yes| yes| STABLE
"quantum circuit"| yes| yes| domain-dependent| domain-dependent| required| required| IMPLEMENTED/DOMAIN
"mts[...]"| planned/incomplete| planned| planned| planned| planned| planned| SPECIFIED
arbitrary hardware gate names| identifier| semantic| required| required| required| required| STABLE MODEL
future syntax| no| no| no| no| no| no| RESERVED

The exact status MUST be generated or manually verified against the repository before a release.

---

8. No Phantom Features

Documentation MUST NOT imply that a feature exists when the implementation does not support it.

For every documented syntax form, one of the following MUST be true:

implemented
specified-only
experimental
deprecated
reserved

There MUST NOT be an undocumented sixth state:

"documented but silently unsupported"

---

9. No Phantom Syntax

The compiler MUST NOT accept syntax that cannot be represented correctly by the AST and semantic model.

The following pipeline invariant is mandatory:

Accepted Source
      ↓
Parser
      ↓
AST

Every successfully parsed construct MUST have a corresponding structural representation.

Likewise:

AST
 ↓
Semantic Analysis

Every semantically valid AST construct MUST have a defined semantic interpretation.

And:

AST
 ↓
IR Generation

Every construct that reaches a lowering boundary MUST either:

1. lower correctly into the canonical IR; or
2. produce a structured unsupported-feature diagnostic before code generation.

It MUST NOT disappear.

---

10. No Silent Semantic Loss

The following behavior is prohibited:

source construct
    ↓
parser accepts
    ↓
AST stores partial information
    ↓
IR generator ignores part
    ↓
program changes meaning

Examples include:

- ignored quantum modifiers;
- dropped effect information;
- discarded resource requirements;
- lost type qualifiers;
- lost source locations;
- silently ignored attributes;
- unsupported gates emitted as comments;
- unknown operations converted into no-ops;
- hardware constraints silently discarded.

If information is required for semantic correctness, it MUST survive until the layer that owns its interpretation.

---

11. Parser/AST Compatibility

The parser MUST only construct AST nodes that correspond to valid AST invariants.

The AST MUST NOT depend on parser implementation details such as:

- token indices;
- parser stack state;
- source-buffer ownership;
- temporary lexer structures.

AST nodes SHOULD preserve:

- source span;
- semantic identity;
- syntactic structure;
- relevant modifiers;
- generic parameters;
- type expressions;
- effect information;
- resource requirements;
- quantum operation intent;
- domain-specific metadata where semantically meaningful.

---

12. AST Migration Boundary

The repository contains both:

src/ast/

and implementation references to:

src/ast/mod.rs

During AST decomposition, both representations MUST preserve one semantic model.

Migration MUST NOT create:

old AST semantics
new AST semantics

as competing meanings.

Instead:

legacy AST representation
        │
        ▼
canonical frontend AST
        │
        ▼
semantic analysis
        │
        ▼
IR

Any compatibility adapter MUST be lossless.

---

13. Syntax Compatibility

Syntax compatibility means that a source program retains the same syntactic interpretation across compatible Zamani versions.

A compatible release MUST NOT:

- change operator precedence;
- change associativity;
- reinterpret an existing keyword;
- change an existing literal's meaning;
- change the binding behavior of an existing construct;
- change a stable quantum operation's semantic intent;
- make a previously valid stable program invalid without a language-version transition.

---

14. Parser Precedence Compatibility

Operator precedence is part of the language contract.

The parser MUST maintain one canonical precedence table.

Conceptually:

assignment
range
logical-or
logical-and
bit-or
bit-xor
bit-and
equality
comparison
shift
sum
product
prefix
call
index
member

The actual implementation MUST be checked against the canonical syntax specification.

A precedence change is a language-level compatibility change.

It MUST NOT be introduced merely by modifying parser code.

---

15. Lexical Compatibility

The lexical contract is defined by:

grammar/spec/lexical.md

The Rust lexer and ANTLR lexer MUST agree on:

- token boundaries;
- identifiers;
- keywords;
- literals;
- punctuation;
- operators;
- comments;
- source spans;
- invalid input behavior.

The lexical specification already establishes "src/lexer.rs" as the reference executable lexer and "ZamaniLexer.g4" as its ANTLR representation.

Neither may silently define additional lexical behavior.

---

16. Keyword Compatibility

Zamani MUST maintain one canonical keyword registry.

The registry MUST distinguish:

reserved keyword
contextual keyword
ordinary identifier
reserved future spelling

New language functionality SHOULD prefer compositional syntax over creating a new keyword whenever practical.

This is especially important for:

- quantum operations;
- mathematics;
- hardware capabilities;
- AI operations;
- optimization;
- device names.

The language MUST NOT become dependent on an ever-growing keyword list.

---

17. Identifier Compatibility

The baseline identifier model remains:

IDENTIFIER_START =
    ASCII_LETTER
    | "_"

IDENTIFIER_CONTINUE =
    ASCII_LETTER
    | DIGIT
    | "_"

Unicode support MUST be versioned if expanded.

The compiler MUST NOT silently normalize identifiers in a way that changes their identity.

---

18. Literal Compatibility

Literal recognition belongs to lexical and syntactic compatibility.

Literal interpretation belongs to semantic processing.

For numeric literals, the lexer MUST NOT prematurely constrain values to:

u32
u64
i32
i64
usize

or another host-dependent representation.

For example:

999999999999999999999999999999999999999999

MUST remain lexically representable if it satisfies the language's integer literal syntax.

Whether it is semantically valid for a particular type is a later decision.

---

19. Source-Encoding Compatibility

The canonical source encoding is UTF-8.

Invalid UTF-8 MUST produce a deterministic source/lexical diagnostic.

Host-specific encodings MUST NOT change program interpretation.

---

20. Comment Compatibility

Comments MUST NOT alter program semantics.

The compiler MAY preserve comments for:

- documentation;
- IDE tooling;
- formatting;
- source transformation;
- macros.

Comment removal from the parser stream MUST NOT destroy source information needed by supported tooling.

---

21. Attribute Compatibility

Attributes such as:

#[attribute]

MUST have a defined lifecycle.

The pipeline is:

lexer
  ↓
parser
  ↓
AST attribute
  ↓
attribute validation
  ↓
semantic interpretation

An attribute MUST NOT be parsed and then silently discarded if it has semantic significance.

An attribute that is intentionally ignored MUST be explicitly documented as non-semantic.

---

22. Error Compatibility

Compiler errors are part of the developer-facing compatibility contract.

Diagnostics MUST be:

- deterministic;
- structured;
- source-located;
- actionable;
- stable enough for tooling;
- independent of hash iteration order;
- independent of machine topology.

The compiler MUST NOT use comments in generated output as a substitute for semantic errors.

For example:

unknown quantum operation

MUST produce a structured compiler diagnostic rather than:

// unknown quantum operation

followed by successful compilation.

---

23. Error Recovery

Error recovery MUST never manufacture a valid semantic program from invalid syntax.

Recovery MAY continue parsing to report additional diagnostics, but:

invalid source
    ≠
valid AST

unless the parser is explicitly operating in an IDE/error-tolerant mode.

Production compilation MUST fail when required correctness conditions are not met.

---

24. Language Versioning

Every Zamani source unit MUST be associated with a language version either explicitly or through the project's configured default.

For example:

language Zamani "1.0";

or an equivalent manifest/compiler configuration.

Language versions control:

- lexical rules;
- syntax;
- reserved words;
- semantic rules;
- compatibility behavior;
- feature availability.

A new compiler MUST NOT silently reinterpret old source using a newer incompatible language.

---

25. Compatibility Classes

Language changes are classified as:

Class A — Fully compatible

Examples:

- new tooling;
- diagnostics improvements;
- optimization;
- backend additions;
- new hardware targets;
- new simulators.

These MUST preserve source semantics.

Class B — Additive

Examples:

- new contextual syntax;
- new standard library facilities;
- new target capabilities.

Existing valid programs MUST retain their meaning.

Class C — Version-gated

Examples:

- new reserved words;
- changed precedence;
- new literal forms that conflict with old syntax;
- semantic changes.

These MUST require an explicit language version.

Class D — Breaking

Examples:

- removal of stable syntax;
- changed meaning of stable syntax;
- incompatible type-system changes.

These require a major language-version transition and migration documentation.

---

26. Backward Compatibility

For a compatible release:

Program P
compiled by version N

MUST retain its meaning when compiled by:

version N+1

provided that:

- P uses stable features;
- P does not depend on explicitly unstable behavior;
- the same language version is selected;
- external dependencies remain compatible.

---

27. Forward Compatibility

A newer compiler SHOULD provide useful diagnostics when encountering syntax from a future version.

It MUST NOT:

- reinterpret future syntax as an unrelated old construct;
- silently ignore future semantic information;
- compile a partially understood construct as a different program.

---

28. ANTLR Compatibility

"grammar/Zamani.g4" MUST be treated as an ANTLR compatibility representation of canonical Zamani syntax.

It MUST NOT become an independent language.

The required relationship is:

Canonical Specification
        │
        ├────────► Rust lexer/parser
        │
        └────────► ANTLR grammar

not:

Rust parser ──► one language

ANTLR grammar ──► another language

ANTLR MUST NOT introduce constructs that the canonical language does not define.

Likewise, stable canonical syntax MUST NOT be omitted from the ANTLR grammar without an explicit tooling limitation.

---

29. ANTLR Conformance

ANTLR grammar conformance MUST be tested using shared fixtures.

The repository SHOULD maintain:

grammar/tests/
├── valid/
├── invalid/
├── lexical/
├── syntax/
├── quantum/
├── mathematics/
├── types/
├── effects/
├── temporal/
├── modules/
├── compatibility/
└── regression/

For each fixture:

source
  ├── Rust lexer
  ├── Rust parser
  └── ANTLR parser

must produce compatible structural results.

Exact internal parse-tree shapes do not have to match, but accepted/rejected language and canonical semantic interpretation MUST match.

---

30. Quantum Compatibility

Quantum syntax MUST describe computational intent.

It MUST NOT encode assumptions about a particular machine.

The grammar MUST NOT define the language as:

H
X
Y
Z
S
T
CNOT
SWAP

alone.

Those may be recognized as standard operations, but they MUST NOT constitute the architectural ceiling of Zamani.

---

31. Quantum Operation Model

The preferred semantic model is:

source intent
     ↓
quantum operation
     ↓
canonical quantum IR
     ↓
capability analysis
     ↓
decomposition
     ↓
routing
     ↓
scheduling
     ↓
target realization

For example, source-level intent may represent:

apply H to q
apply controlled(X) from c to t
apply U(theta, phi, lambda) to q

The semantic system determines whether the operation is:

- directly supported;
- decomposable;
- synthesizable;
- simulatable;
- routable;
- schedulable;
- executable.

---

32. Quantum Hardware Independence

The source language MUST NOT require:

maximum qubits
fixed qubit topology
fixed gate set
fixed connectivity
fixed pulse duration
fixed clock
fixed memory size
fixed QPU vendor
fixed accelerator
fixed instruction width

as language-level constants.

Those belong to target capabilities and resource models.

---

33. Quantum Scaling

A quantum program MUST be expressible independently of machine size.

For example, the semantic intent may describe:

for q in register {
    ...
}

rather than forcing:

q0
q1
q2
...
q1023

into the source language.

Where a program genuinely requires a fixed resource count, that requirement MUST be represented explicitly as a program/resource constraint rather than hidden in compiler implementation constants.

---

34. Dynamic Quantum Resources

Quantum resources SHOULD be represented symbolically wherever possible.

Examples include:

Qubit
QubitRegister
QuantumState
QuantumResource
LogicalQubit
PhysicalQubit
Ancilla
MeasurementResource

The exact type names are governed by the canonical semantic/type specification.

The compiler MUST distinguish:

logical intent

from:

physical allocation

---

35. Quantum Gate Compatibility

A gate name is not automatically a hardware instruction.

The semantic model is:

OperationIdentity
OperationParameters
Operands
Preconditions
Effects
ResourceRequirements

A backend MAY implement an operation directly.

Otherwise it MAY use:

decomposition
synthesis
routing
approximation
emulation
simulation

subject to declared semantic guarantees.

---

36. No Unknown-Gate Comments

The following is prohibited:

unknown gate
↓
emit comment
↓
continue compilation

Instead:

unknown operation
↓
diagnostic
↓
compilation failure

unless the source explicitly requests an opaque/external operation whose semantics are defined by the language's foreign-operation mechanism.

---

37. Quantum IR Compatibility

The canonical quantum IR is the semantic boundary between language intent and target realization.

Optimization, routing, scheduling, noise modeling, hardware lowering, benchmarking, and execution MUST consume the canonical IR rather than defining independent incompatible quantum gate models.

The following architectural pattern is prohibited:

optimizer::QuantumGate
scheduler::QuantumGate
hardware::QuantumGate
benchmark::QuantumGate

when those types duplicate the canonical quantum operation model.

The preferred architecture is:

canonical quantum IR
        │
        ├── optimization
        ├── routing
        ├── scheduling
        ├── noise
        ├── hardware
        ├── simulation
        └── benchmarking

---

38. Resource Independence

All machine resource limits MUST be modeled as target/resource constraints.

Examples:

memory
qubits
logical qubits
physical qubits
registers
cores
threads
accelerators
bandwidth
latency
storage
stack depth
instruction count
energy
power

MUST NOT be hard-coded into the language grammar.

---

39. Scalability Contract

Zamani is designed to scale:

atom
   ↓
small device
   ↓
single machine
   ↓
accelerator
   ↓
QPU
   ↓
cluster
   ↓
distributed system
   ↓
large-scale computational substrate
   ↓
future architectures

subject only to actual resource availability and explicitly declared semantic constraints.

This means:

no artificial MAX_QUBITS
no artificial MAX_THREADS
no artificial MAX_MEMORY
no artificial MAX_DEVICES
no artificial MAX_DIMENSION
no artificial MAX_TENSOR_RANK

may be introduced into the language specification.

Implementation safety limits MAY exist, but they MUST be:

1. explicit;
2. configurable where appropriate;
3. reported diagnostically;
4. separate from language semantics;
5. not mistaken for language limits.

---

40. Resource Exhaustion

Resource exhaustion is not the same thing as invalid syntax.

For example:

program is semantically valid
machine cannot provide requested resources

MUST result in a resource/capability diagnostic rather than a grammar failure.

This distinction is essential:

SyntaxError
TypeError
EffectError
ResourceError
CapabilityError
BackendError
RuntimeError

must remain conceptually distinct.

---

41. POCO-REAF Compatibility

Zamani MUST support:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever.»

This does not mean every program must run identically on physically incompatible machines without transformation.

It means the source program expresses stable semantic intent rather than target-specific implementation details.

The compiler may perform:

specialization
lowering
optimization
decomposition
scheduling
routing
resource allocation
code generation

without requiring source modification.

---

42. Semantic Portability

A portable program MUST separate:

WHAT

from:

HOW

Source code specifies:

- computation;
- data;
- control;
- types;
- effects;
- resource requirements;
- quantum intent;
- correctness constraints.

Target layers determine:

- instruction selection;
- hardware mapping;
- physical layout;
- scheduling;
- memory allocation;
- gate decomposition;
- device-specific optimization.

---

43. Target Capability Model

Target capability is represented outside the grammar.

Conceptually:

Target
 ├── instruction capabilities
 ├── numeric capabilities
 ├── memory capabilities
 ├── concurrency capabilities
 ├── quantum capabilities
 ├── timing capabilities
 ├── communication capabilities
 └── reliability guarantees

A program is compiled against capabilities rather than against a hard-coded machine model.

---

44. Capability Failure

If a target cannot directly realize a construct, the compiler MUST attempt the declared lowering strategies before rejecting the program.

Conceptually:

direct implementation?
       │
       ├── yes → emit
       │
       └── no
            ↓
       decomposition?
            │
            ├── yes → lower
            │
            └── no
                 ↓
             synthesis?
                 │
                 ├── yes → lower
                 │
                 └── no
                      ↓
                  emulation?
                      │
                      ├── yes → lower
                      │
                      └── no → capability diagnostic

The exact policies belong to the compiler and target layers, not the grammar.

---

45. Mathematics Compatibility

Mathematical constructs MUST follow the same architecture.

The grammar may express mathematical intent such as:

vector
matrix
tensor
symbolic expression
differentiate
integrate
optimize
transform
solve

but the grammar MUST NOT assume:

fixed matrix dimension
fixed tensor rank
fixed floating-point width
fixed CPU
fixed SIMD width
fixed accelerator

unless explicitly part of the source program's type/resource contract.

---

46. Generic Mathematics

Mathematical dimensions SHOULD be represented symbolically or generically.

Instead of encoding:

matrix_1024x1024

as a language primitive, the semantic model SHOULD permit parameterized dimensions where appropriate.

Example:

Matrix<T, M, N>

where "M" and "N" are semantic dimensions rather than compiler hard-coded constants.

---

47. Effect Compatibility

Effects MUST be preserved across:

parser
→ AST
→ semantic analysis
→ IR
→ lowering

An effect MUST NOT disappear merely because a backend does not currently implement it.

Unsupported effects MUST produce an explicit diagnostic or be transformed through a formally defined lowering.

---

48. Temporal Compatibility

Temporal constructs such as:

zamani
sasa
MTS

MUST have clearly defined semantic meaning.

A temporal construct MUST NOT be implemented as an ordinary identifier merely because the lexer/parser does not yet support the intended literal.

If syntax is specified but not implemented, its status MUST remain:

SPECIFIED

until implementation is complete.

---

49. Memory and Sankofa Compatibility

Constructs associated with:

remember
recall
learn
infer
wisdom
zamani
sasa

MUST have explicit semantic ownership.

The grammar defines syntax.

The AST represents structure.

The semantic system defines meaning.

The runtime/IR defines execution.

No lexical rule may itself implement memory behavior.

---

50. Nano Compatibility

Nano-agent and nanoscale constructs MUST be target-independent.

Syntax such as:

nano
agent
@atom
@molecule(...)

MUST describe semantic intent.

It MUST NOT imply:

specific physical atom
specific fabrication process
specific molecular substrate
specific hardware vendor

unless represented through explicit target capabilities.

---

51. Generic Computational Domains

The same compatibility architecture applies to future domains:

AI
cognitive computing
biological computing
HDL
distributed computing
network computing
symbolic computing
formal verification
cryptography
metaprogramming

A domain MUST NOT create a second language pipeline.

The domain must integrate into:

lexer
parser
AST
semantic model
IR
verification
lowering

according to its semantic needs.

---

52. Unsafe Rust Prohibition

The Zamani compiler implementation MUST NOT require Rust "unsafe".

This includes:

- lexer;
- parser;
- AST;
- semantic analysis;
- IR;
- verification;
- optimization;
- scheduling;
- target lowering;
- test infrastructure used by production code.

The project baseline is:

Rust 1.97 / Rust 1.97.1

with safe Rust.

Any dependency requiring "unsafe" internally is a dependency-level concern, but Zamani source code itself MUST NOT introduce unsafe blocks or unsafe functions.

The compatibility specification MUST NOT require unsafe Rust to achieve scalability.

---

53. Safe Scalability

Scalability MUST be achieved through:

- streaming;
- incremental processing;
- bounded queues;
- arenas where appropriate;
- structural sharing;
- iterative algorithms where recursion depth is a concern;
- explicit resource accounting;
- lazy lowering where appropriate;
- chunked processing;
- deterministic parallelism;
- target-independent IR;
- configurable resource budgets.

Scalability MUST NOT depend on unsafe memory manipulation.

---

54. No Host-Width Semantics

The language MUST NOT derive semantic limits from:

usize
isize
pointer width
CPU register width
host address width

For example:

32-bit host

must not cause Zamani to become a 32-bit language.

Likewise:

64-bit host

must not establish the language's maximum semantic integer size.

---

55. No Fixed Compiler Machine Model

The compiler MUST NOT encode assumptions such as:

MAX_QUBITS = 32
MAX_REGISTERS = 256
MAX_THREADS = 1024
MAX_TENSOR_RANK = 16
MAX_MATRIX_SIZE = 4096

as language semantics.

If implementation safeguards are necessary, they belong to explicit resource-policy configuration.

---

56. Determinism

For the same:

source
language version
compiler version
dependency versions
target description
compiler configuration

the compiler MUST produce deterministic semantic results.

Determinism MUST NOT depend on:

- hash-map iteration;
- thread scheduling;
- CPU count;
- network timing;
- wall clock;
- random state.

Where nondeterminism is intentionally requested, it MUST be explicit in the semantic model.

---

57. Parallel Compilation

Parallelism MUST NOT change language semantics.

The compiler MAY parallelize:

- parsing independent modules;
- semantic analysis;
- IR verification;
- optimization;
- backend compilation;
- test execution.

But the result MUST be equivalent to a deterministic serial compilation.

---

58. Incremental Compilation

Incremental compilation MUST preserve the same semantic result as a clean compilation.

Cache keys SHOULD include every input that can affect semantics, including:

source content
language version
compiler version
feature configuration
dependency identity
relevant target description
relevant compiler options

A stale cache MUST never change program meaning.

---

59. Dependency Compatibility

Language syntax MUST remain independent of dependency implementation.

A package may introduce:

functions
types
traits
operations
quantum operations
effects

without modifying the core grammar.

Dependencies MUST NOT be able to silently redefine core syntax.

---

60. Modules and Imports

Module resolution belongs outside lexical analysis.

The parser may recognize:

module
import
use

but:

filesystem resolution
package resolution
registry lookup
network access

are not grammar responsibilities.

This separation is required for deterministic compilation and sandboxed tooling.

---

61. Security Compatibility

Parsing untrusted source MUST NOT require:

- arbitrary filesystem access;
- network access;
- process execution;
- environment mutation;
- secret access.

Grammar processing MUST be safe to execute in a sandbox.

---

62. Parser Resource Limits

The implementation MAY enforce configurable limits against denial-of-service inputs.

Examples:

maximum source bytes
maximum token count
maximum nesting depth
maximum diagnostic count
maximum AST nodes
maximum compilation time

Such limits are implementation/resource policies.

They MUST NOT be presented as inherent language limits.

---

63. Deep Nesting

The compiler SHOULD avoid unbounded native call-stack dependence for attacker-controlled or generated source.

Where practical, deep constructs SHOULD be processed using:

- explicit stacks;
- iterative traversal;
- bounded recursion;
- configurable depth checks.

A program must not crash the compiler merely because it contains a deeply nested valid structure.

---

64. AST Size

AST construction MUST account for potentially very large programs.

The implementation MUST NOT assume that:

program = small

The compiler MAY reject a program when actual resource budgets are exhausted, but MUST report resource exhaustion rather than incorrectly reporting syntax failure.

---

65. Source Span Compatibility

All parser-produced semantic nodes SHOULD retain source-location information sufficient for diagnostics.

A source span MUST be:

deterministic
non-negative
within source boundaries

where applicable.

Span representation MUST NOT depend on host pointer size.

---

66. Error Compatibility Across Layers

A lower layer MUST NOT erase higher-level error context.

For example:

lexer error

should retain:

source location
token context
error category
message

Parser errors should retain lexical/source context.

Semantic errors should retain AST/source context.

IR verification errors should be traceable back to source constructs where possible.

---

67. IR Compatibility

The canonical IR is the semantic boundary.

The IR MUST represent enough information to preserve the meaning of valid source programs.

The IR MUST NOT be a disguised target instruction set.

It MUST remain capable of representing:

CPU computation
GPU computation
quantum computation
distributed computation
mathematical computation
effects
resources
control flow
data flow
timing

where supported by the language.

---

68. IR Verification

Every lowering stage MUST preserve IR invariants.

Conceptually:

AST
 ↓
semantic validation
 ↓
IR generation
 ↓
IR verification
 ↓
optimization
 ↓
verification
 ↓
lowering
 ↓
verification

Optimization MUST NOT be permitted to bypass required verification boundaries.

---

69. Semantic Preservation

Every transformation:

IR₁ → IR₂

MUST preserve the semantics guaranteed by the IR contract.

For quantum transformations, this may mean preserving the declared equivalence relation rather than merely preserving textual gate sequences.

For mathematical transformations, this may mean preserving exact, approximate, symbolic, or numerical guarantees according to the operation's contract.

---

70. Hardware Lowering Compatibility

Hardware lowering MAY specialize a program.

It MUST NOT modify the source-level meaning silently.

Examples:

logical qubit
    ↓
physical qubit

abstract operation
    ↓
native gate sequence

abstract tensor
    ↓
accelerator kernel

parallel computation
    ↓
target-specific execution graph

These are valid transformations when their correctness is established.

---

71. Backend Addition Compatibility

Adding a backend MUST NOT require changing the source grammar merely to support the backend.

For example:

new QPU

should integrate through:

capability model
IR lowering
backend

rather than:

new QPU keyword
new QPU syntax
new parser branch

unless the backend exposes a genuinely language-level concept.

---

72. Standard Operation Compatibility

Standard operations such as quantum gates, mathematical functions, or target capabilities SHOULD be modeled as semantic entities rather than hard-coded grammar productions wherever possible.

For example:

operation identity

is preferable to:

one grammar production per hardware instruction

This keeps the language extensible.

---

73. External / Foreign Operations

If Zamani permits external operations, they MUST have explicit semantics.

An external operation SHOULD declare:

name
signature
effects
resource requirements
capabilities
determinism
side effects
availability

Unknown external operations MUST NOT be silently treated as no-ops.

---

74. Macro and Metaprogramming Compatibility

Metaprogramming MUST NOT bypass the semantic contract.

Generated source MUST pass through the appropriate validation pipeline.

Generated constructs MUST NOT gain privileges merely because they originated inside the compiler.

Macro expansion MUST be:

- deterministic unless explicitly specified otherwise;
- bounded;
- diagnosable;
- source-mappable;
- compatible with language-version rules.

---

75. Compatibility of Generated Code

Generated Zamani source MUST be interpreted according to the same language version and semantic rules as handwritten source.

A generated construct MUST NOT depend on undocumented parser behavior.

---

76. Formatting Compatibility

Formatting is not semantics.

A formatter MUST NOT:

- change AST meaning;
- change source semantics;
- introduce target-specific syntax;
- normalize identifiers incorrectly;
- alter literal values.

Formatting tests SHOULD round-trip:

source
 → AST
 → formatted source
 → AST

with semantic equivalence.

---

77. IDE/LSP Compatibility

IDE tooling SHOULD consume the same lexical and syntactic contract as the compiler.

The IDE MUST NOT accept syntax that production compilation rejects without clearly marking it as incomplete/error-tolerant parsing.

Tooling may use error recovery, but:

IDE recovery
≠
compiler acceptance

---

78. Syntax Highlighting Compatibility

Syntax highlighters SHOULD derive their keyword/token inventory from the canonical lexical specification.

A syntax highlighter MUST NOT become the authority for language syntax.

---

79. Test-First Compatibility Contract

Every stable grammar feature MUST have at least:

1 valid syntax test
1 invalid syntax test
1 semantic test

where applicable.

Compiler-critical constructs SHOULD additionally have:

AST test
IR test
verification test
round-trip or golden test

---

80. Differential Grammar Testing

The repository SHOULD test:

Rust lexer
vs
ANTLR lexer

and:

Rust parser
vs
ANTLR parser

using common fixtures.

Disagreements MUST be classified as:

Rust implementation bug
ANTLR implementation bug
specification bug
fixture bug
intentional compatibility difference

They MUST NOT remain unresolved.

---

81. Regression Testing

Every resolved compatibility defect SHOULD become a regression fixture.

Recommended structure:

grammar/tests/regression/
    lexical/
    parser/
    ast/
    semantic/
    quantum/
    types/
    effects/
    temporal/
    modules/
    ir/

A regression must remain permanently reproducible unless the corresponding language feature is intentionally removed.

---

82. Golden Tests

Golden tests SHOULD be used for:

- token streams;
- AST serialization;
- diagnostics;
- semantic representations;
- canonical IR;
- formatter output.

Golden tests MUST use stable representations.

They MUST NOT depend on:

- memory addresses;
- hash iteration order;
- thread order;
- host-specific paths where avoidable;
- absolute machine paths.

---

83. Property-Based Testing

The parser and lexer SHOULD support property-based testing where practical.

Important properties include:

lexer always progresses
parser never hangs
valid token streams do not crash parser
format(parse(format(source))) is stable
valid AST lowers without silent loss
IR verification rejects malformed IR

---

84. Fuzzing

The frontend SHOULD be fuzz-tested for:

- malformed UTF-8;
- random Unicode;
- malformed comments;
- malformed strings;
- deeply nested expressions;
- huge numeric literals;
- huge identifiers;
- enormous quantum programs;
- malformed attributes;
- malformed generic types;
- operator ambiguity;
- parser recovery.

The required result is:

diagnostic or successful parse

not:

panic
memory corruption
undefined behavior
hang

---

85. Compatibility Fixture Format

Each fixture SHOULD record:

language_version
feature_status
source
expected_lexical_result
expected_parse_result
expected_semantic_result
expected_ir_result
expected_diagnostics

This allows the same language example to validate multiple compiler layers.

---

86. Feature Promotion Process

A feature moves through:

IDEA
 ↓
DESIGNED
 ↓
SPECIFIED
 ↓
LEXED
 ↓
PARSED
 ↓
AST
 ↓
SEMANTIC
 ↓
IR
 ↓
VERIFIED
 ↓
TESTED
 ↓
IMPLEMENTED
 ↓
STABLE

No feature may jump directly from:

IDEA → STABLE

merely because documentation exists.

---

87. Feature Removal Process

Removal proceeds:

STABLE
 ↓
DEPRECATED
 ↓
MIGRATION AVAILABLE
 ↓
REMOVAL VERSION DECLARED
 ↓
REMOVED

The compiler SHOULD provide migration diagnostics during the deprecation period.

---

88. Breaking Changes

A breaking change MUST document:

old behavior
new behavior
reason
affected syntax
affected semantics
migration path
first affected version

Breaking changes MUST NOT be hidden in parser implementation changes.

---

89. Compatibility of "grammar/grammar.md"

"grammar/grammar.md" MUST remain the implementation-conformance document.

It MUST answer:

«What does the current reference compiler accept?»

It MUST NOT answer:

«What might Zamani eventually become?»

That second responsibility belongs to the canonical language specification and future design documents.

Whenever "src/lexer.rs" or "src/parser.rs" changes accepted syntax, "grammar/grammar.md" MUST be updated in the same change.

---

90. Compatibility of "grammar/Zamani.g4"

"grammar/Zamani.g4" MUST be reconciled with:

grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/spec/semantics.md
src/lexer.rs
src/parser.rs

before being treated as production-compatible.

Any construct appearing only in "Zamani.g4" is not automatically implemented.

Any construct required by stable canonical syntax but absent from "Zamani.g4" is an ANTLR compatibility defect.

---

91. Compatibility of "grammar/Zamani-Grammar.md"

"grammar/Zamani-Grammar.md" may retain broad historical and design material.

It MUST NOT silently promote aspirational features to implemented status.

If a feature from that document becomes part of the stable language:

feature design
 ↓
canonical specification
 ↓
implementation
 ↓
tests
 ↓
compatibility status

must be completed.

---

92. Compatibility of "grammar/README.md"

"grammar/README.md" MUST remain the navigation and governance entry point.

It SHOULD clearly identify:

canonical specification
implementation grammar
ANTLR compatibility grammar
historical/design grammar
test suites
compatibility rules

Users MUST be able to determine which file answers which question.

---

93. Repository Integration Contract

The grammar subsystem SHOULD be understood as:

grammar/
├── README.md
├── DESIGN.md
├── grammar.md
├── Zamani.g4
├── Zamani-Grammar.md
├── spec/
│   ├── lexical.md
│   ├── syntax.md
│   ├── semantics.md
│   └── compatibility.md
└── tests/
    ├── valid/
    ├── invalid/
    ├── lexical/
    ├── syntax/
    ├── semantic/
    ├── quantum/
    ├── mathematics/
    ├── effects/
    ├── temporal/
    ├── modules/
    ├── compatibility/
    └── regression/

No directory is required merely for aesthetics; directories SHOULD be introduced when they provide a real ownership boundary.

---

94. Required Integration Work

The compatibility specification requires the following repository-wide relationships.

"grammar/spec/lexical.md"

Owns:

characters
tokens
literals
operators
keywords
comments
lexical errors

"grammar/spec/syntax.md"

Owns:

program structure
declarations
statements
expressions
types
patterns
grammar composition

"grammar/spec/semantics.md"

Owns:

meaning
typing
effects
resource requirements
quantum semantics
temporal semantics
domain semantics

"grammar/spec/compatibility.md"

Owns:

versioning
implementation conformance
migration
ANTLR compatibility
feature status
backward compatibility
scalability contract

---

95. Required Compiler Integration

The compiler pipeline MUST conceptually remain:

lexer
  ↓
parser
  ↓
frontend AST
  ↓
name resolution
  ↓
semantic analysis
  ↓
type/effect/resource validation
  ↓
canonical IR generation
  ↓
IR verification
  ↓
optimization
  ↓
target-independent lowering
  ↓
target-specific lowering

No backend is permitted to redefine frontend semantics.

---

96. Required Quantum Integration

Quantum-related compiler components MUST converge on the canonical quantum semantic/IR model.

They MUST NOT independently define incompatible:

gate types
qubit identifiers
operation models
resource models
timing models

The grammar describes source intent.

The frontend AST preserves intent.

The canonical IR represents executable semantics.

Optimization, scheduling, routing, noise, hardware and benchmarking consume that representation.

---

97. Compatibility With Future Hardware

A future hardware architecture MUST be integrable without requiring a rewrite of the source language.

Examples include:

new CPU architecture
new GPU
new accelerator
new QPU
new photonic architecture
new memory architecture
new distributed substrate
new simulation engine

Integration SHOULD occur through:

capability description
IR lowering
backend implementation

rather than grammar modification.

---

98. Compatibility With Arbitrarily Large Logical Programs

The language MUST NOT impose arbitrary limits on:

program size
module count
function count
type count
generic depth
tensor dimensions
logical qubit count
logical resource count

The actual compiler implementation may impose configurable resource budgets.

Such budgets MUST be treated as implementation constraints, not language semantics.

---

99. "Infinity" and Physical Reality

"Infinity" in the Zamani scalability contract means:

«no artificial finite machine-size bound is encoded into the language model; execution remains bounded only by the resources, mathematical validity, declared constraints, and capabilities available to the execution environment.»

It does NOT require a compiler to allocate infinite physical memory or infinite hardware.

Therefore:

language scalability
≠
infinite physical resources

The language remains conceptually unbounded while each execution is resource-bounded.

---

100. Compatibility Principle

The fundamental rule is:

Language meaning
        ↓
must not depend on
        ↓
implementation accident

In particular, meaning MUST NOT depend on:

host CPU
host word size
host memory size
number of cores
QPU vendor
GPU vendor
machine topology
compiler thread count
hash-map order
filesystem layout
network availability

unless explicitly represented as a semantic/environmental dependency.

---

101. Compiler Correctness Invariant

For a valid source program:

Source
  ↓
Parse
  ↓
AST
  ↓
Semantic validation
  ↓
IR
  ↓
IR verification

must preserve semantic intent.

For any optimization:

IR_before
    ≡
IR_after

according to the applicable semantic equivalence relation.

For any backend:

IR
    ≡
Target realization

within the guarantees declared by the target/backend.

---

102. Compatibility Invariant

For every stable construct:

specification
     =
implementation
     =
AST meaning
     =
semantic meaning
     =
IR meaning

where equality means semantic agreement, not identical internal data structures.

---

103. No Special Case for Quantum

Quantum programs are not a separate language.

They are Zamani programs containing quantum-domain semantics.

Therefore the same compatibility rules apply to:

quantum
classical
hybrid
mathematical
temporal
nano
distributed
effectful

programs.

---

104. No Special Case for Mathematics

Mathematical programming is not a second grammar.

Mathematical constructs MUST integrate through the same:

AST
semantic model
IR
verification
lowering

architecture.

---

105. No Special Case for Future Domains

Future domains MUST integrate through the same architecture.

A new domain MUST answer:

What syntax does it require?
What AST representation does it require?
What semantic rules does it require?
What IR representation does it require?
What resource/capability model does it require?
How is it verified?
How is it lowered?

before being declared stable.

---

106. Compatibility Checklist

A feature is production-ready only when all applicable answers are YES.

[ ] Canonical syntax specified
[ ] Canonical semantics specified
[ ] Lexical behavior specified
[ ] Rust lexer implemented
[ ] Rust parser implemented
[ ] AST representation implemented
[ ] Semantic validation implemented
[ ] Canonical IR representation implemented
[ ] IR verification implemented
[ ] Error diagnostics implemented
[ ] ANTLR representation reconciled
[ ] Valid tests implemented
[ ] Invalid tests implemented
[ ] Regression tests implemented
[ ] Documentation updated
[ ] Version compatibility defined
[ ] Resource behavior defined
[ ] Target independence established
[ ] No hard-coded machine-size assumptions
[ ] No silent semantic loss
[ ] No unsafe Rust
[ ] Deterministic behavior verified
[ ] Large-input behavior tested
[ ] Deep-input behavior tested
[ ] Fuzzing/property testing considered

---

107. Release Gate

A Zamani release MUST NOT be declared grammar-compatible if:

Rust parser ≠ canonical syntax
ANTLR grammar ≠ canonical syntax
AST cannot represent accepted syntax
semantic layer cannot validate accepted syntax
IR cannot represent valid semantics
IR verifier rejects valid compiler output
unsupported constructs silently disappear
hardware assumptions leak into source grammar

Any one of these is a release-blocking compatibility defect.

---

108. Canonical Decision Tree

When deciding where a new concept belongs:

Is it about characters/tokens?
    └── lexical specification

Is it about source structure?
    └── syntax specification

Is it about meaning?
    └── semantic specification

Is it about source compatibility/versioning?
    └── compatibility specification

Is it about structural representation?
    └── AST

Is it about semantic executable representation?
    └── canonical IR

Is it about transforming semantics?
    └── optimizer/lowering

Is it about realizing semantics on hardware?
    └── backend/capability system

A concept MUST NOT be placed in the grammar merely because it exists somewhere else in the compiler.

---

109. Final Architectural Contract

The production Zamani language is therefore defined by:

                 CANONICAL ZAMANI
                       │
          ┌────────────┴────────────┐
          │                         │
      Lexical                     Syntax
          │                         │
          └────────────┬────────────┘
                       ▼
                     AST
                       │
                       ▼
              Semantic Resolution
                       │
          ┌────────────┼────────────┐
          │            │            │
        Types        Effects      Resources
          │            │            │
          └────────────┼────────────┘
                       ▼
                Canonical IR
                       │
                 IR Verification
                       │
          ┌────────────┼─────────────┐
          │            │             │
      Optimize       Route        Schedule
          │            │             │
          └────────────┼─────────────┘
                       ▼
              Target-independent
                    lowering
                       │
          ┌────────────┼─────────────┐
          │            │             │
         CPU          GPU           QPU
          │            │             │
          └────────────┼─────────────┘
                       ▼
                 Execution

The critical invariant is:

SOURCE INTENT
     ↓
MUST SURVIVE
     ↓
EVERY SEMANTIC BOUNDARY
     ↓
UNTIL REALIZATION

---

110. POCO-REAF Final Guarantee

Zamani's compatibility architecture is complete when a developer can write:

one program

whose semantics do not depend on:

machine size
CPU architecture
GPU architecture
QPU topology
native gate set
memory width
register count
device count
physical qubit count

and the compiler can specialize that program for the resources actually available.

Therefore:

Program Once
      ↓
Compile Once
      ↓
Canonical Semantic Representation
      ↓
Target-independent transformations
      ↓
Target-specific realization
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Remain Semantically Stable Forever

subject to:

language-version compatibility
program-declared requirements
resource availability
target capabilities
mathematical validity
semantic correctness

This is the normative compatibility contract for production Zamani.

---

111. Definition of Done

"grammar/spec/compatibility.md" is considered integrated when:

grammar/spec/lexical.md
        │
grammar/spec/syntax.md
        │
grammar/spec/semantics.md
        │
grammar/spec/compatibility.md
        │
        ▼
one canonical language contract
        │
        ▼
src/lexer.rs
src/parser.rs
src/ast/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs
        │
        ▼
one executable implementation
        │
        ▼
ANTLR compatibility
        │
        ▼
shared conformance tests

No document may silently override another.

No implementation layer may silently redefine the language.

No target backend may leak hardware assumptions into source semantics.

No accepted syntax may lack a semantic representation.

No semantic construct may silently disappear during lowering.

No scalability limit may be hard-coded into the language model.

No production Zamani compiler component may require Rust "unsafe".

The result is a single, versioned, compositional, implementation-verifiable language capable of scaling from the smallest supported computation to arbitrarily large programs and machines as resources permit.This is intentionally a compatibility contract, rather than another giant grammar. That keeps lexical.md, syntax.md, and semantics.md as the owners of their respective concerns while making compatibility.md the authority for reconciling grammar.md, Zamani.g4, Zamani-Grammar.md, the Rust frontend, AST, semantic layer, and IR. The existing repository documentation already points toward exactly those implementation boundaries.

 