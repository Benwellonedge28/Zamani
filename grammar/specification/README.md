

Zamani Language Specification

Path: "grammar/specification/README.md"
Status: Production specification architecture
Language: Zamani
Compiler: Zamani Compiler / ZUTC
Rust baseline: Rust 1.97.1
Safety: Safe Rust only; "unsafe" Rust is prohibited
Primary portability model: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability model: From the smallest representable computation to arbitrarily large computation, subject only to explicit program semantics, available resources, and target capabilities.

---

1. Purpose

This directory defines the normative architectural and language-level specification for Zamani.

The specification establishes:

- what the Zamani language means;
- how source syntax is organized;
- how syntax relates to the lexer and parser;
- how source constructs are represented by the AST;
- where semantic meaning is established;
- how classical, quantum, hardware, HDL, distributed, AI, mathematical, and future computing domains integrate;
- how resource-independent programming is represented;
- how POCO-REAF is preserved;
- how language versions remain compatible;
- how the grammar evolves without becoming coupled to temporary hardware;
- how every language feature receives a complete implementation and integration contract.

This document is architectural.

It does not replace the executable parser, lexer, AST, semantic analyzer, or canonical IR.

---

2. Normative Language Architecture

Zamani is one language.

Its implementation is a pipeline:

Zamani Source
     │
     ▼
Lexical Analysis
     │
     ▼
Parsing
     │
     ▼
AST
     │
     ▼
Name / Module Resolution
     │
     ▼
Type Analysis
     │
     ▼
Effect Analysis
     │
     ▼
Capability Analysis
     │
     ▼
Resource / Constraint Analysis
     │
     ▼
Domain Semantic Lowering
     │
     ├───────────────┐
     ▼               ▼
Classical IR     quantum::ir
     │               │
     └───────┬───────┘
             ▼
      Target-Independent
          Optimization
             │
             ▼
      Target-Aware Lowering
             │
             ▼
      Scheduling / Mapping
             │
             ▼
      Target Backend
             │
     ┌───────┼────────┬────────┐
     ▼       ▼        ▼        ▼
    CPU     GPU      FPGA     QPU
     │       │        │        │
     └───────┴────────┴────────┘
                     │
                     ▼
          Runtime / Deployment

The dependency direction is mandatory.

A downstream subsystem must not redefine an upstream language concept merely because it needs additional information.

---

3. Specification Authority

The repository currently contains several language-description surfaces.

They have different responsibilities.

3.1 "grammar/specification/"

This directory is the normative language-design and architectural specification.

It defines:

- language principles;
- language model;
- syntax contracts;
- semantic boundaries;
- type-system requirements;
- effect requirements;
- quantum language requirements;
- module requirements;
- compatibility rules;
- scalability rules;
- conformance requirements.

It may describe features that are not yet implemented, but every such feature MUST have an explicit lifecycle state.

A specification is not evidence that a feature is implemented.

---

3.2 "grammar/grammar.md"

"grammar/grammar.md" is the implementation-conformance grammar.

It documents what the current reference lexer/parser accepts.

The repository currently identifies the lexer, parser, AST, semantic analysis, and IR generation as the implementation chain behind this document.

When the accepted syntax changes:

src/lexer.rs
src/parser.rs
src/ast/

and the corresponding conformance grammar MUST change in the same logical change.

This prevents documentation from claiming syntax the reference compiler does not accept.

---

3.3 "grammar/Zamani.g4"

"grammar/Zamani.g4" is the ANTLR representation of Zamani syntax.

It MUST:

- represent the canonical language syntax;
- remain semantically equivalent to the canonical language specification;
- remain testable against conformance fixtures;
- avoid inventing target-specific semantics;
- avoid imposing arbitrary machine limits.

ANTLR is a representation/tooling mechanism.

It is not a second language.

---

3.4 "grammar/Zamani-Grammar.md"

"grammar/Zamani-Grammar.md" may contain broader language-design material.

Any feature described there MUST carry an explicit status.

Allowed statuses include:

PROPOSED
DESIGNED
SPECIFIED
LEXICALLY_SUPPORTED
PARSED
AST_SUPPORTED
SEMANTIC_SUPPORTED
IR_SUPPORTED
BACKEND_SUPPORTED
TESTED
STABLE
DEPRECATED
REMOVED

A feature MUST NOT be described as implemented solely because it appears in this document.

---

3.5 "grammar/README.md"

The parent README is the navigation and architecture overview for "grammar/".

It must point developers toward:

grammar/specification/
grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/tests/

It must not become another competing normative specification.

---

4. Single-Language Rule

Zamani MUST NOT fragment into independent grammars such as:

core grammar
quantum grammar
HDL grammar
AI grammar
hardware grammar

if those grammars define incompatible languages.

Instead:

                    Zamani Language
                          │
          ┌───────────────┼────────────────┐
          │               │                │
        Core           Domain          Extension
          │               │                │
    expressions       quantum            dialects
    statements        classical          macros
    types             HDL                metaprogramming
    modules           hardware           future domains
    effects           distributed

All domains share:

- lexical rules;
- identifiers;
- source locations;
- declarations;
- expressions;
- types;
- effects;
- capabilities;
- resources;
- versioning;
- diagnostics;
- semantic analysis.

Domain extensions add meaning.

They do not create another language.

---

5. Normative Layering

Every Zamani feature MUST belong to one or more explicitly defined layers.

5.1 Lexical layer

Owns:

- characters;
- tokens;
- identifiers;
- literals;
- operators;
- punctuation;
- comments;
- keywords.

Implementation:

src/lexer.rs

---

5.2 Syntactic layer

Owns:

- valid token sequences;
- declarations;
- expressions;
- statements;
- types;
- modules;
- domain syntax.

Implementation:

src/parser.rs
grammar/Zamani.g4

---

5.3 Structural layer

Owns the source representation.

Implementation:

src/ast/

Every AST node that requires diagnostics MUST retain sufficient source-location information.

The current AST already follows this principle through "Span"-carrying nodes.

---

5.4 Semantic layer

Owns:

- name resolution;
- type validity;
- ownership validity;
- effect validity;
- capability validity;
- resource requirements;
- domain validity;
- semantic diagnostics.

Implementation:

src/semantic.rs

---

5.5 IR layer

Owns canonical computation representation.

Implementation includes:

src/ir_gen.rs
src/ir_verify.rs

and domain-specific canonical IRs.

For quantum computation:

src/quantum/ir/

is the canonical semantic boundary.

The quantum IR architecture already explicitly separates the canonical "quantum::ir" namespace from downstream systems.

---

6. Ownership Rules

Component| Owns| Must not own
"grammar/specification/"| Language specification| Runtime implementation
"grammar/Zamani.g4"| ANTLR syntax| Semantic validity
"grammar/grammar.md"| Current parser conformance| Future promises
"grammar/Zamani-Grammar.md"| Broad design material| False implementation claims
"src/lexer.rs"| Tokenization| Semantic interpretation
"src/parser.rs"| Syntax recognition| Hardware realization
"src/ast/"| Source structure| Backend behavior
"src/semantic.rs"| Semantic validity| Tokenization
"src/ir_gen.rs"| AST-to-IR lowering| Target scheduling
"src/ir_verify.rs"| IR invariants| Source parsing
"src/quantum/ir/"| Canonical quantum semantics| Source grammar
Optimization| Semantics-preserving transformations| Language definition
Scheduling| Timing/resource scheduling| Language definition
ZQN| Quantum noise semantics/execution concerns| Duplicate quantum IR
Hardware| Target capabilities/realization| Language definition
Runtime| Execution| Source-language syntax
Backends| Target-specific realization| Portable source semantics

This table is normative.

---

7. Feature Ownership Contract

Every new Zamani language feature MUST answer these questions before implementation:

What syntax introduces it?
Which lexer tokens does it require?
Which parser rule consumes it?
Which AST node represents it?
Which semantic subsystem validates it?
Which IR represents its meaning?
Which optimizer consumes it?
Which scheduler consumes it?
Which runtime consumes it?
Which backend consumes it?
Which tests prove it?
Which compatibility rules govern it?

If any question has no answer, the feature is not production-ready.

---

8. Independent-File Completion Rule

Every file under this specification directory MUST be independently completable.

A file is considered complete only when its integration contract is already defined.

Every specification file MUST explicitly define:

Purpose
Scope
Normative status
Owns
Does not own
Inputs
Outputs
Dependencies
Upstream contracts
Downstream consumers
Syntax contract
AST contract
Semantic contract
IR contract
Compiler integration
Runtime integration
Tooling integration
Cross-domain integration
Compatibility requirements
Scalability requirements
Hard-coding requirements
Tests
Negative tests
Boundary tests
Completion criteria

The file MUST NOT depend on an undefined future decision.

---

9. No Re-Editing Principle

The implementation process MUST be dependency-first.

Before implementing a file, all contracts required by that file MUST already be fixed.

For example:

specification/language-principles.md
        ↓
specification/syntax-model.md
        ↓
lexer
        ↓
parser
        ↓
AST
        ↓
semantic model
        ↓
IR

A completed lower-level contract MUST NOT be invalidated merely because a higher-level feature is later introduced.

If a future feature genuinely requires changing an established language contract, it MUST go through language-versioning and compatibility procedures.

---

10. Language Evolution

Zamani MUST evolve additively whenever possible.

New features should prefer:

existing syntax
+
generic/composable extension

over:

new reserved keyword
+
new parser special case
+
new AST special case

A new keyword is justified only when ordinary identifiers and compositional syntax cannot express the required semantic distinction adequately.

This prevents the keyword namespace from becoming a permanent catalogue of every technology Zamani may ever support.

---

11. Compositional Language Principle

Zamani is intended to support technologies that do not yet exist.

Therefore syntax MUST favor composition.

Prefer:

operation(name, parameters, targets)

over permanently encoding every operation as a keyword.

Prefer:

resource(...)
capability(...)
constraint(...)
requirement(...)

over hard-coded target-specific constructs.

Prefer:

type constructors
generic parameters
traits
interfaces
effects
attributes
dialects

over unlimited keyword growth.

This is essential for long-term scalability.

---

12. POCO-REAF

Zamani's portability model is:

Program Once
     ↓
Compile Once
     ↓
Run Everywhere
     ↓
Run Anywhere
     ↓
Run Forever

This means source semantics are not defined by one machine.

It does not mean every machine can execute every program regardless of resources.

The following distinctions are mandatory:

Language validity
        ≠
Semantic validity
        ≠
Compilation validity
        ≠
Target compatibility
        ≠
Resource availability
        ≠
Runtime availability

A program can therefore be:

valid
+
semantically meaningful
+
not executable on the current target

without being an invalid Zamani program.

---

13. Portable Semantic Contract

The source program SHOULD describe:

- what computation is required;
- what results are required;
- what properties must be preserved;
- what capabilities are required;
- what constraints apply;
- what resources are preferred;
- what alternatives are acceptable.

The source program SHOULD NOT unnecessarily specify:

- today's processor model;
- today's QPU model;
- fixed device IDs;
- fixed topology;
- fixed memory capacity;
- fixed number of cores;
- fixed number of GPUs;
- fixed number of qubits;
- fixed accelerator count.

Target realization belongs downstream.

---

14. Scalability Model

Zamani uses resource-parametric computation.

There is no language-defined finite upper bound on:

- program size;
- data size;
- number of declarations;
- number of functions;
- number of modules;
- number of types;
- number of quantum resources;
- number of classical resources;
- number of hardware resources;
- number of devices;
- number of nodes;
- number of concurrent activities;
- number of dimensions;
- number of tensor elements;
- number of execution stages.

Actual implementation limits may arise from:

- address space;
- available memory;
- compiler resources;
- operating-system limits;
- backend limits;
- target capabilities;
- user-defined resource budgets;
- physical laws;
- numerical representation.

Such limits MUST NOT silently become permanent grammar limits.

---

15. "Infinity" and Resource Reality

"Scale to infinity" is interpreted as:

«No arbitrary finite limit is imposed by the language merely for convenience of a current implementation.»

It does not mean infinite physical storage or infinite computation exists.

For example:

N = program-defined or resource-defined

is valid.

This is not:

N <= 1024

unless "1024" is itself part of the program's explicit semantic requirement.

A target may reject:

N = 10^12

because of resource limitations.

That is a target/resource diagnostic, not a grammar failure.

---

16. Absolute No-Hard-Coding Rule

The grammar and language specification MUST NOT hard-code arbitrary machine limits.

Prohibited examples include:

MAX_QUBITS = 32
MAX_QUBITS = 64
MAX_CORES = 128
MAX_THREADS = 1024
MAX_DEVICES = 16
MAX_NODES = 1024
MAX_MATRIX_DIM = 4096
MAX_TENSOR_RANK = 32

Equivalent hidden restrictions are also prohibited.

The following are equally prohibited:

q[0]
q[1]
q[2]

as a compiler-defined complete universe of qubits.

Indexed resources MUST be represented as collections whose cardinality is determined by program semantics and/or available resources.

---

17. Resource Abstraction

Zamani distinguishes:

Requirement

Something the program needs for correctness.

Example:

requires quantum;

Capability

Something a target can provide.

Example:

supports quantum;

Constraint

Something that must not be violated.

Example:

constraint latency < T;

Preference

Something desirable but negotiable.

Example:

prefer low_energy;

Hint

Information supplied to optimization but not required for correctness.

Example:

hint locality;

Resource

An actual execution resource.

Example:

qubit
core
memory
device
node
accelerator

Target

A concrete execution environment.

These concepts MUST NOT be collapsed into one construct.

---

18. Quantum Language

Quantum computing is a first-class Zamani domain.

The grammar must support a complete abstract quantum programming model without coupling source syntax to one hardware generation.

The language must be capable of expressing:

- qubits;
- quantum registers;
- logical qubits;
- physical-qubit references;
- quantum states;
- operations;
- parameterized operations;
- controlled operations;
- inverse/adjoint operations;
- circuits;
- measurement;
- reset;
- observables;
- dynamic circuits;
- mid-circuit measurement;
- classical feed-forward;
- quantum/classical interaction;
- logical computation;
- error-correction intent;
- quantum resource requirements;
- quantum capability requirements;
- backend-independent execution intent.

---

19. Quantum Operation Model

Zamani MUST NOT require every possible quantum gate to become a reserved language keyword.

The language should support compositional operation descriptions.

Conceptually:

operation <name>
parameters <...>
targets <...>
controls <...>
modifiers <...>

The exact source syntax belongs to the canonical syntax specification.

The architectural contract is:

Source
  ↓
Quantum intent
  ↓
Semantic validation
  ↓
quantum::ir
  ↓
Optimization
  ↓
Routing
  ↓
Scheduling
  ↓
Noise/resilience analysis
  ↓
Hardware lowering

The current quantum IR architecture already establishes "quantum::ir" as a structured semantic namespace with subcomponents for quantum computation rather than a parser-level grammar.

---

20. Quantum Hardware Independence

The language MUST NOT require source code to know:

- QPU topology;
- coupling maps;
- calibration values;
- pulse implementations;
- native instruction encodings;
- physical qubit numbering;
- vendor-specific gate restrictions.

These belong to:

hardware
calibration
optimization
scheduling
ZQN
backend
runtime

A quantum program should remain semantically meaningful when moved between different quantum technologies.

---

21. Logical and Physical Quantum Resources

The language MUST distinguish:

logical qubit

from:

physical qubit

A logical program SHOULD NOT be forced to name physical qubits.

Mapping:

logical qubits
       ↓
resource analysis
       ↓
logical-to-physical mapping
       ↓
routing
       ↓
scheduling
       ↓
physical execution

is a compiler/backend responsibility.

---

22. Quantum Error Correction

The grammar may express error-correction intent.

It MUST NOT duplicate the complete implementation of QEC.

The architecture is:

Zamani source
     ↓
QEC intent
     ↓
semantic validation
     ↓
canonical quantum representation
     ↓
QEC subsystem

The QEC subsystem owns:

- code-specific semantics;
- syndrome processing;
- correction strategies;
- decoder integration;
- resource analysis;
- execution behavior.

The grammar owns only source syntax.

---

23. ZQN Boundary

Zamani's quantum-noise architecture MUST remain separate from grammar.

ZQN owns quantum-noise concerns such as:

- noise channels;
- faults;
- calibration-aware noise;
- noise-aware execution;
- resilience analysis.

The grammar may express source-level noise intent.

It must not become another noise implementation or another quantum IR.

---

24. Classical Computing

Zamani MUST support general classical computation.

The language model includes:

- scalars;
- structured values;
- arrays;
- vectors;
- matrices;
- tensors;
- functions;
- generics;
- traits;
- interfaces;
- control flow;
- pattern matching;
- memory;
- concurrency;
- parallelism;
- numerical computation;
- symbolic computation;
- accelerator-oriented computation.

Classical semantics MUST remain sufficiently general to interoperate with quantum and hardware domains.

---

25. Mathematical Computing

Mathematical capabilities SHOULD primarily be expressed through:

types
+
generic operations
+
functions
+
traits
+
libraries/intrinsics
+
semantic mathematical representations

The grammar MUST NOT become a permanent dictionary of algorithms.

For example, a new mathematical algorithm should not require a new reserved keyword unless the operation has fundamental language-level semantics that cannot be expressed compositionally.

---

26. HDL and Hardware

Zamani may describe hardware directly.

The language must be capable of expressing:

- hardware modules;
- ports;
- signals;
- wires;
- registers;
- clocks;
- timing;
- combinational logic;
- sequential logic;
- processes;
- state machines;
- memories;
- pipelines;
- hardware interfaces;
- parameters;
- generic hardware structures;
- hardware capabilities;
- hardware constraints.

The language MUST distinguish:

hardware intent

from:

physical implementation

For example:

pipeline depth = D

may be a program parameter.

It must not imply:

pipeline depth <= 32

unless such a bound is explicitly part of the program's semantics.

---

27. Hardware/Software Co-Design

Zamani must permit a single program to describe interacting:

software
hardware
accelerator
quantum
distributed

components.

The compiler determines legal lowering boundaries.

A hardware description must not force the rest of the program to become hardware-specific.

Likewise, software syntax must not prevent a compiler from lowering suitable computation into:

- CPU;
- GPU;
- FPGA;
- ASIC;
- accelerator;
- quantum resources;
- future computational substrates.

---

28. Distributed Computing

Distributed constructs must be resource-parametric.

The language must not assume:

node_count = N

as a permanent compiler limit.

It should be possible to express:

- nodes;
- services;
- messages;
- channels;
- remote execution;
- replication;
- consistency;
- placement;
- fault tolerance;
- distributed state.

The number of physical nodes belongs to deployment and resource management.

---

29. Concurrency

Concurrency syntax must express:

- tasks;
- asynchronous operations;
- futures;
- actors;
- channels;
- synchronization;
- parallel execution;
- cancellation;
- data parallelism;
- task parallelism.

The grammar MUST NOT define arbitrary concurrency ceilings.

Runtime resource availability determines actual concurrency.

---

30. AI and Data Computing

AI/data features must use the same semantic foundation.

Zamani may express:

- models;
- datasets;
- tensors;
- training;
- inference;
- agents;
- pipelines;
- differentiation;
- accelerator requirements.

The number of:

- model parameters;
- tensor elements;
- agents;
- pipeline stages;
- accelerator resources;

must not be limited by grammar-level constants.

---

31. Networking

Networking constructs may express:

- endpoints;
- services;
- messages;
- protocols;
- channels;
- network capabilities;
- communication requirements.

The grammar must not embed a fixed network topology.

---

32. Security

Security syntax may express:

- identities;
- permissions;
- capabilities;
- trust;
- cryptographic operations;
- privacy requirements;
- security constraints.

Security semantics belong to the semantic/compiler/runtime layers.

The grammar must not silently interpret a syntactic declaration as a security guarantee.

---

33. Effects

Effects represent observable or constrained behavior.

Potential effects include:

IO
HARDWARE
QUANTUM
NETWORK
DISTRIBUTED
SECURITY
MEMORY
TIME
RANDOMNESS

Effects MUST remain compositional.

A new backend must not require a new grammar keyword merely because it implements an existing effect differently.

---

34. Capabilities

Capabilities describe what an execution environment can provide.

Capabilities are not equivalent to resources.

For example:

capability quantum

does not mean:

use QPU #7

Likewise:

capability gpu

does not mean:

use exactly one NVIDIA device

Capability negotiation belongs downstream of parsing.

---

35. Compile-Time vs Run-Time

The language must clearly distinguish:

compile-time information

from:

runtime information

and:

target-discovered information

Compile-time constructs MUST NOT accidentally require runtime resources.

Runtime resource discovery MUST NOT alter the source grammar.

---

36. Target Selection

Target selection is not source semantics unless explicitly declared as a semantic requirement.

The compiler may use:

target descriptions
capabilities
resource constraints
optimization policies
deployment configuration

to determine realization.

Source code should remain portable unless the programmer explicitly chooses target-specific behavior.

---

37. Target-Specific Extensions

Target-specific functionality must be isolated through explicit mechanisms such as:

dialects
attributes
capabilities
target blocks
interoperability boundaries
backend extensions

Target-specific constructs MUST NOT leak into the universal core unintentionally.

A vendor-specific feature must not become a mandatory Zamani keyword.

---

38. Interoperability

Zamani may interoperate with:

- C;
- C++;
- Python;
- OpenQASM;
- Verilog;
- other HDLs;
- foreign functions;
- system interfaces;
- external ABIs.

Interoperability syntax describes boundaries.

It does not redefine Zamani's internal semantic model.

External representations must be lowered into appropriate Zamani semantic structures.

---

39. AST Contract

Every syntax construct must have a deterministic structural representation.

The AST must preserve enough information for:

- diagnostics;
- semantic analysis;
- tooling;
- source transformations;
- lowering;
- testing.

The AST must not encode target-specific implementation details merely because the parser happens to encounter them.

The current AST already contains explicit structures for classical declarations, quantum constructs, effects, patterns, expressions, types, and advanced constructs.

Future additions must extend this model deliberately rather than introducing parallel AST representations.

---

40. Parser Contract

The reference parser is a hand-written recursive-descent / Pratt parser.

Its responsibilities are:

- recognizing valid source syntax;
- constructing AST nodes;
- preserving spans;
- reporting syntax errors;
- recovering where safe;
- making forward progress.

The current parser explicitly implements precedence and dispatches both classical and Zamani-specific constructs, including quantum, noise, surface-code, effects, and advanced declarations.

The parser MUST NOT:

- schedule hardware;
- map physical qubits;
- perform optimization;
- select a QPU;
- perform runtime execution;
- embed arbitrary target limits.

---

41. Lexer Contract

The lexer must provide:

- deterministic tokenization;
- stable token identity;
- source spans;
- lexical diagnostics;
- deterministic malformed-input behavior;
- Unicode handling according to the language specification;
- no target-specific decisions.

The current lexer contains a large token vocabulary, including core language tokens, quantum/nano tokens, effects, advanced system constructs, and duplicate/overlapping operator categories that require normalization.

In particular, overlapping concepts such as:

BitAnd / Ampersand
BitOr / Pipe
Question / QuestionMark
Arrow / ThinArrow

must have an explicit canonical lexical rule.

---

42. Token Lifecycle

Every token MUST follow:

Specified
    ↓
Lexed
    ↓
Parser-consumable
    ↓
AST-representable
    ↓
Semantically meaningful
    ↓
Lowerable
    ↓
Tested

A token that exists only in an enum but is never emitted by the lexer MUST NOT be documented as implemented.

This applies particularly to token forms such as MTS-related syntax where the implementation and documentation must agree.

---

43. Error Model

Diagnostics must distinguish:

lexical error
syntax error
name-resolution error
type error
effect error
capability error
resource error
target compatibility error
lowering error
runtime error

The grammar layer must not report backend failures as syntax failures.

For example:

program requests 1000 logical qubits

is not a grammar error merely because:

current QPU has 127 usable physical qubits

The latter is a capability/resource/mapping problem.

---

44. Error Recovery

Parser recovery MUST:

1. identify the unexpected token;
2. preserve a structured diagnostic;
3. consume input;
4. make progress;
5. synchronize at a grammar boundary;
6. avoid fabricating executable semantics.

Recovery must never produce an apparently valid semantic program from malformed syntax without an explicit error state.

---

45. Deep-Program Scalability

Zamani must support extremely large programs subject to available resources.

Compiler implementation must avoid unnecessary recursion where deep input can cause stack exhaustion.

Where appropriate, compiler infrastructure should use:

- iterative traversal;
- explicit stacks;
- explicit work queues;
- streaming;
- incremental processing;
- lazy processing;
- resource-aware allocation;
- configurable diagnostic limits.

No arbitrary constant may be introduced merely to make implementation convenient.

If a limit is necessary for safety, it must be:

1. explicit;
2. configurable where appropriate;
3. documented;
4. reported diagnostically;
5. independent of language semantics.

---

46. Memory Safety

The Zamani compiler baseline is:

Rust 1.97.1

All compiler and grammar infrastructure covered by this specification MUST use safe Rust.

The following are prohibited:

unsafe { ... }
unsafe fn
unsafe impl
unsafe trait

unless the project's top-level safety policy is explicitly changed through a separate language/compiler architecture decision.

The grammar architecture must not require unsafe Rust.

---

47. Resource-Aware Compilation

A compiler may impose execution budgets to protect the compiler process.

Examples include:

memory budget
time budget
diagnostic budget
recursion/work budget
temporary-storage budget

Such budgets are implementation controls.

They MUST NOT become source-language semantics unless explicitly specified as such.

The distinction is:

Language limit

versus:

Compiler safety budget

versus:

Target resource limit

These must remain separate.

---

48. Determinism

Given the same:

source
language version
compiler configuration

lexing and parsing MUST be deterministic.

Semantic and lowering determinism must be defined independently where optimization is permitted.

Any nondeterministic optimization must not change program semantics.

---

49. Versioning

Every language feature belongs to a language version.

Versioning must distinguish:

syntax compatibility
semantic compatibility
AST compatibility
IR compatibility
backend compatibility
tooling compatibility

Removing or changing a stable feature requires:

- deprecation policy;
- migration guidance;
- compatibility classification;
- tests;
- version documentation.

---

50. Experimental Features

Experimental syntax must be explicitly marked.

Possible states:

experimental
unstable
proposed
deprecated
stable

Experimental constructs MUST NOT silently become permanent syntax.

They must remain isolated enough that their removal does not destabilize unrelated language features.

---

51. Reserved Space

Zamani must reserve extensibility mechanisms for future technologies.

Reserved space may include:

- dialect namespaces;
- attributes;
- annotations;
- generic operations;
- capability declarations;
- effect declarations;
- resource declarations;
- target extensions.

Reserved space MUST NOT become undocumented syntax.

---

52. Dialects

Dialects are extensions of Zamani's semantic ecosystem.

A dialect may introduce:

- domain-specific operations;
- attributes;
- types;
- declarations;
- interoperability constructs.

A dialect MUST declare:

name
version
namespace
required capabilities
syntax extensions
semantic extensions
AST representation
IR representation
compatibility policy

A dialect MUST NOT silently alter the meaning of existing core syntax.

---

53. Macros and Metaprogramming

Macros and metaprogramming must preserve the distinction between:

source transformation

and:

runtime execution

Macro expansion must be deterministic under identical inputs and configuration.

Generated source must remain subject to normal:

lexing
parsing
semantic analysis

unless a separately specified IR-level macro system is used.

---

54. Genericity

Zamani must use generic abstractions to express scalable computation.

Examples include:

generic type
generic function
generic collection
generic resource
generic dimension
generic device capability
generic quantum operation
generic hardware module

Genericity must not be implemented through a finite enumeration of supported sizes.

---

55. Parametric Dimensions

Dimensions should be representable as:

compile-time constants
symbolic expressions
runtime values
resource-derived values

where semantically valid.

This applies to:

- arrays;
- matrices;
- tensors;
- qubit registers;
- hardware resources;
- data structures;
- distributed collections.

The grammar MUST NOT force all dimensions into fixed compiler constants.

---

56. Quantum Register Scaling

A quantum register should be conceptually:

QubitCollection<N>

where "N" may be determined by:

- source semantics;
- generic parameters;
- compile-time information;
- runtime information;
- target resources.

The grammar MUST NOT assume a universal finite "N".

---

57. Hardware Scaling

A hardware module should be able to describe:

module
parameters
interfaces
resources
behavior
constraints

without embedding the exact number of:

- gates;
- registers;
- memory cells;
- processing elements;
- accelerators.

Instantiation determines scale.

---

58. Distributed Scaling

A distributed program should express logical deployment requirements rather than a fixed physical machine.

For example:

replicate service

does not imply:

replicas = 3

unless three is explicitly part of program semantics.

The deployment system may choose a feasible realization according to resource and policy constraints.

---

59. "Program Once"

A program should be portable when its semantics do not depend on target-specific details.

Portability does not require hiding genuine semantic requirements.

For example:

requires quantum

is a legitimate semantic requirement.

But:

requires IBM_X_device_17

is target-specific and must be isolated appropriately.

---

60. "Compile Once"

The architecture should permit compilation into stable, reusable representations.

The compiler may produce:

source AST
semantic model
canonical IR
portable executable representation
target-specific executable representation

The exact artifact boundaries belong to compiler architecture.

The grammar MUST NOT require source rewriting merely because the target changes.

---

61. "Run Everywhere"

The execution system should determine whether the program can be:

- directly executed;
- transformed;
- lowered;
- decomposed;
- mapped;
- routed;
- scheduled;
- simulated;
- emulated;
- distributed;
- executed on another compatible target.

The grammar remains unchanged.

---

62. "Run Anywhere"

The source language must not assume:

local machine
single machine
single CPU
single accelerator
single QPU
single operating system
single network

unless explicitly required by the program.

---

63. "Run Forever"

Forever means semantic longevity.

Zamani must preserve the meaning of programs as technology evolves through:

- stable semantics;
- versioned syntax;
- compatibility rules;
- explicit deprecation;
- dialects;
- capability negotiation;
- target-independent IR;
- migration tooling.

No language specification can guarantee that arbitrary future hardware will execute every historical program without adaptation.

The architectural objective is instead to ensure that the program's semantic meaning survives technological change.

---

64. Canonical Quantum IR Integration

The grammar MUST NOT create a competing quantum IR.

The required relationship is:

Zamani quantum syntax
        ↓
AST
        ↓
semantic quantum representation
        ↓
quantum::ir
        ↓
optimization
        ↓
routing
        ↓
scheduling
        ↓
ZQN / resilience
        ↓
hardware lowering

"quantum::ir" remains the canonical quantum semantic boundary.

Downstream quantum components must consume this canonical representation rather than defining independent temporary gate/program models.

---

65. Optimization Boundary

Optimization MUST preserve semantics.

Optimization may operate on:

- canonical IR;
- target-independent representations;
- target-aware representations where appropriate.

Optimization must not change source grammar.

An optimizer must never force a new source-language keyword merely because it discovers a new optimization strategy.

---

66. Scheduling Boundary

Scheduling owns:

- timing;
- resource conflicts;
- execution ordering;
- placement;
- scheduling constraints.

Scheduling does not own language syntax.

A scheduler must not introduce source-language limits such as:

MAX_QUBITS
MAX_OPS
MAX_THREADS

to compensate for a specific backend.

---

67. Hardware Boundary

Hardware owns:

- capabilities;
- topology;
- physical resources;
- calibration;
- native operations;
- target-specific constraints.

Hardware does not own universal language semantics.

---

68. Runtime Boundary

Runtime owns:

- resource discovery;
- execution;
- dispatch;
- runtime scheduling;
- environment interaction;
- failure reporting.

Runtime behavior must not alter the syntax accepted by the parser.

---

69. Repository Integration Matrix

The grammar specification integrates with the repository as follows:

Layer| Integration
Lexer| "src/lexer.rs"
Parser| "src/parser.rs"
AST| "src/ast/"
Semantic analysis| "src/semantic.rs"
IR generation| "src/ir_gen.rs"
IR verification| "src/ir_verify.rs"
Quantum IR| "src/quantum/ir/"
Quantum optimization| "src/quantum/optimization/"
Quantum scheduling| "src/quantum/scheduling/"
QEC| quantum error-correction subsystem
ZQN| quantum-noise subsystem
Hardware| quantum/classical hardware subsystem
Calibration| hardware/calibration subsystem
Benchmarking| benchmarking subsystem
Frontends| source-format frontends
Interoperability| FFI/foreign-language frontends
Tests| grammar/compiler/domain tests
Documentation| language and architecture documentation

The grammar remains upstream of these systems.

---

70. Dependency Direction

The required dependency direction is:

Specification
     ↓
Lexer
     ↓
Parser
     ↓
AST
     ↓
Semantic Analysis
     ↓
Canonical IR
     ↓
Optimization
     ↓
Scheduling
     ↓
Hardware / Runtime

Never:

Hardware
   ↓
Grammar

or:

Runtime
   ↓
Parser

or:

quantum::ir
   ↓
grammar syntax

The latter would invert the architecture.

---

71. Grammar-to-Repository Contract

For every new grammar construct:

grammar specification
        ↓
lexer token contract
        ↓
parser production
        ↓
AST representation
        ↓
semantic rule
        ↓
IR mapping
        ↓
tests

must be defined before the construct is declared complete.

---

72. Cross-Domain Integration

The grammar must permit composition of:

classical + quantum
classical + HDL
classical + hardware
quantum + classical
quantum + HDL
quantum + hardware
quantum + distributed
AI + classical
AI + quantum
AI + hardware
network + distributed
security + distributed

and combinations involving all of them.

No domain may create a syntax island that cannot participate in the rest of Zamani.

---

73. Cross-Domain Semantic Rule

Cross-domain syntax does not automatically imply cross-domain semantic validity.

For example:

quantum operation

may be syntactically valid.

Semantic analysis must determine:

- whether the referenced resource exists;
- whether the operation is defined;
- whether types are correct;
- whether capabilities are sufficient;
- whether resources are sufficient;
- whether the operation can be lowered.

---

74. Testing Contract

Every feature requires:

Positive tests

Valid syntax.

Negative tests

Invalid syntax.

Semantic tests

Invalid meaning despite valid syntax.

Boundary tests

Smallest and very large practical representations.

Cross-domain tests

Interactions with other language domains.

Compatibility tests

Old valid syntax remains valid where compatibility promises require it.

Determinism tests

Repeated parsing produces equivalent results.

Round-trip tests

Where a printer/serializer exists:

source
 ↓
AST
 ↓
printed source
 ↓
AST

must preserve semantics.

---

75. Scalability Tests

Tests MUST specifically detect accidental finite limits.

Examples include parameterized tests for:

many declarations
many functions
many modules
large nested structures
large arrays
large tensor dimensions
large qubit collections
large operation sequences
large distributed descriptions
large hardware descriptions

Tests must not encode an arbitrary maximum as proof of scalability.

The test should establish that the implementation scales with resources rather than an artificial language ceiling.

---

76. Hard-Coding Audit

Every language change must be audited for:

MAX_*
fixed counts
fixed device IDs
fixed topology
fixed qubit IDs
fixed core counts
fixed thread counts
fixed memory sizes
fixed tensor dimensions
fixed deployment counts
fixed accelerator counts

Every discovered constant must be classified:

LANGUAGE_SEMANTIC
RESOURCE_POLICY
TARGET_CONSTRAINT
IMPLEMENTATION_LIMIT
SAFETY_BUDGET
TEST_LIMIT
ACCIDENTAL_HARDCODING

"ACCIDENTAL_HARDCODING" MUST be removed.

---

77. Repository-Wide Hard-Coding Principle

The grammar specification alone cannot guarantee scalability if downstream code introduces fixed limits.

Therefore the audit must eventually include:

grammar/
src/lexer.rs
src/parser.rs
src/ast/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs
src/quantum/
src/hardware/
src/optimization/
src/scheduling/
src/zqn/

The language specification establishes the rule.

Compiler and backend implementations enforce it.

---

78. Generated Artifacts

Generated parser/lexer artifacts MUST NOT become independent authorities.

The relationship must be:

canonical specification
        ↓
canonical grammar representation
        ↓
generated artifacts

Generated files should be reproducible.

Generated artifacts should not be manually edited unless explicitly designated as source files.

---

79. ANTLR Integration

ANTLR is an interoperability/tooling representation.

If "Zamani.g4" is maintained as an ANTLR grammar, it MUST:

- compile successfully with the supported ANTLR toolchain;
- produce deterministic parsing;
- represent canonical syntax;
- have conformance tests;
- report unsupported constructs honestly;
- remain synchronized with the reference parser.

ANTLR-specific implementation details must not leak into Zamani language semantics.

---

80. Reference Parser Integration

The current reference parser is hand-written.

Therefore the canonical conformance relationship is:

Language specification
        ↓
reference parser
        ↓
grammar.md

and:

Language specification
        ↓
Zamani.g4

The two executable representations must converge on the same accepted language.

Differences must be classified as:

bug
unsupported feature
intentional version difference
experimental feature
obsolete syntax

Never silently ignored.

---

81. Language Status Model

Every language feature must have one status:

PROPOSED
DESIGNED
SPECIFIED
LEXER_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
BACKEND_IMPLEMENTED
TESTED
STABLE
DEPRECATED
REMOVED

A feature may only advance when all prerequisites are satisfied.

For example:

STABLE

requires:

LEXER_IMPLEMENTED
+
PARSER_IMPLEMENTED
+
AST_IMPLEMENTED
+
SEMANTIC_IMPLEMENTED
+
IR_IMPLEMENTED
+
TESTED

where those layers apply to that feature.

---

82. Feature Completion Definition

A language feature is complete only when:

- its syntax is specified;
- its lexical requirements are specified;
- its parser production is defined;
- its AST representation exists;
- its semantic meaning is defined;
- its ownership is defined;
- its IR mapping is defined;
- downstream consumers are identified;
- errors are defined;
- compatibility is defined;
- scalability is defined;
- hard-coding has been audited;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- cross-domain tests exist where applicable;
- documentation is synchronized.

---

83. Specification File Contract

Every file under:

grammar/specification/

must use the following conceptual template:

# Title

## Status

## Purpose

## Scope

## Normative Requirements

## Owns

## Does Not Own

## Inputs

## Outputs

## Dependencies

## Upstream Contracts

## Downstream Consumers

## Syntax Contract

## Lexer Contract

## Parser Contract

## AST Contract

## Semantic Contract

## IR Contract

## Compiler Integration

## Runtime Integration

## Tooling Integration

## Cross-Domain Integration

## Compatibility

## Scalability

## Hard-Coding Audit

## Tests

## Negative Tests

## Boundary Tests

## Completion Criteria

Not every section needs implementation details when irrelevant, but the ownership decision itself must be explicit.

---

84. Required Specification Files

The specification directory should contain only files with clearly defined responsibilities.

The planned production specification set is:

grammar/specification/
├── README.md
├── language-principles.md
├── language-scope.md
├── language-version.md
├── compatibility.md
├── grammar-authority.md
├── syntax-model.md
├── semantic-model.md
├── compilation-model.md
├── execution-model.md
├── scalability-model.md
├── poco-reaf.md
├── extensibility.md
└── reserved-space.md

These files are architectural specifications.

They must not duplicate the detailed grammar production rules unnecessarily.

---

85. "language-principles.md"

Owns:

- fundamental Zamani principles;
- semantic portability;
- compositionality;
- safety;
- determinism;
- extensibility;
- resource independence.

Does not own:

- concrete grammar productions;
- backend implementation.

Integration:

language-principles
    ↓
all specification files

Completion requires every principle to be referenced consistently by dependent specifications.

---

86. "language-scope.md"

Owns:

- domains supported by Zamani;
- classical computing;
- quantum computing;
- HDL;
- hardware;
- distributed computing;
- AI;
- data;
- networking;
- security;
- future domains.

Does not own:

- individual syntax productions.

Completion requires all domains to have defined integration boundaries.

---

87. "language-version.md"

Owns:

- language version;
- version identifiers;
- feature lifecycle;
- compatibility windows;
- experimental/stable status.

Does not own:

- semantic definitions of individual features.

---

88. "compatibility.md"

Owns:

- source compatibility;
- parser compatibility;
- semantic compatibility;
- AST compatibility;
- IR compatibility;
- migration policy;
- deprecation policy.

Does not own:

- target-specific compatibility.

---

89. "grammar-authority.md"

Owns:

- authority hierarchy;
- relationship between:
  - specification;
  - "Zamani.g4";
  - "grammar.md";
  - "Zamani-Grammar.md";
  - lexer;
  - parser;
  - AST.

Completion requires there to be no ambiguous grammar authority.

---

90. "syntax-model.md"

Owns:

- source syntax architecture;
- lexical/syntactic boundaries;
- compositional syntax;
- grammar modularity;
- declarations;
- expressions;
- statements;
- types.

Does not own:

- semantic validity.

---

91. "semantic-model.md"

Owns:

- source meaning;
- name resolution;
- type semantics;
- effects;
- capabilities;
- resource requirements;
- domain semantics.

Does not own:

- tokenization or concrete parsing.

---

92. "compilation-model.md"

Owns:

- AST-to-IR architecture;
- canonical representations;
- target-independent compilation;
- target-specific lowering;
- optimization boundaries.

Integration:

src/ast/
   ↓
src/semantic.rs
   ↓
src/ir_gen.rs
   ↓
canonical IR

---

93. "execution-model.md"

Owns:

- runtime semantics;
- execution contexts;
- resource discovery;
- scheduling;
- dispatch;
- deployment;
- target selection.

It must preserve source-level portability.

---

94. "scalability-model.md"

Owns the formal scalability contract.

It must explicitly prohibit arbitrary finite language limits.

It must define:

resource-parametric semantics
dynamic sizing
generic sizing
capability discovery
resource constraints
compiler budgets
target limits

as separate concepts.

---

95. "poco-reaf.md"

Owns the formal definition of:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

It must define exactly what is guaranteed and what is necessarily target/resource dependent.

---

96. "extensibility.md"

Owns:

- dialects;
- attributes;
- extension points;
- domain additions;
- future computing paradigms.

It must prevent extensions from modifying core semantics silently.

---

97. "reserved-space.md"

Owns:

- reserved keywords;
- reserved namespaces;
- extension namespaces;
- future syntax allocation;
- compatibility-safe extension points.

Reserved space must not become undocumented implementation behavior.

---

98. Relationship to Existing "grammar/" Files

The specification files MUST NOT duplicate entire existing grammar documents.

Instead:

specification/
    defines why and what

Zamani.g4
    defines ANTLR syntax

grammar.md
    records current parser conformance

Zamani-Grammar.md
    records broader language design

This eliminates competing documentation while retaining the existing repository assets.

---

99. Relationship to "src/lexer.rs"

The specification establishes lexical requirements.

"src/lexer.rs" implements them.

The lexer must not invent syntax that is absent from the specification without first updating the language contract.

The current lexer contains a broad token set and therefore requires continued normalization and conformance auditing rather than unlimited addition of new special cases.

---

100. Relationship to "src/parser.rs"

The parser implements syntax.

The current parser uses recursive descent plus Pratt/precedence parsing.

Future specification files must respect this existing architecture unless a deliberate parser architecture migration is approved.

The specification must never require parser behavior that cannot be represented deterministically.

---

101. Relationship to "src/ast/"

The AST is the structural contract.

The specification must define semantic categories independently of implementation-specific Rust enum layouts.

AST changes must preserve:

- source spans;
- structural meaning;
- deterministic representation;
- semantic lowering requirements.

---

102. Relationship to "src/semantic.rs"

Semantic analysis is the boundary where syntax becomes validated language meaning.

Examples:

valid syntax
+
invalid type

must be a semantic error.

Likewise:

valid quantum syntax
+
insufficient capability

must be a capability/resource diagnostic rather than a parser failure.

---

103. Relationship to "src/ir_gen.rs"

"src/ir_gen.rs" lowers validated source semantics toward canonical IR.

It must not become responsible for defining the language grammar.

The repository's documentation already identifies AST → IR as this boundary.

---

104. Relationship to "src/ir_verify.rs"

IR verification validates invariants after lowering.

It must not compensate for missing semantic analysis by redefining source-language validity.

---

105. Relationship to "quantum::ir"

The quantum IR is canonical.

The specification defines source-level quantum intent.

"quantum::ir" defines canonical quantum computation semantics.

Downstream systems consume it.

The grammar must never duplicate it.

The repository's quantum IR structure already contains separate namespaces for core, quantum, control, pulse, hashing, analysis, and validation concerns.

---

106. Relationship to Quantum Optimization

Quantum optimization consumes canonical quantum semantics.

It must not require grammar changes merely to add an optimization.

---

107. Relationship to Quantum Scheduling

Scheduling consumes semantic/IR representations and resource contexts.

It must not define source syntax.

Scheduling must remain capable of handling resource sizes determined at compile time, runtime, or target discovery.

---

108. Relationship to ZQN

ZQN consumes canonical quantum representations and noise-related information.

It must not become a duplicate grammar or quantum IR.

---

109. Relationship to Hardware

Hardware descriptions provide capabilities and constraints.

They must be discoverable or explicitly supplied.

They must not become permanent source-language limits.

---

110. Relationship to Benchmarking

Benchmarking may measure:

- circuit volume;
- depth;
- execution cost;
- latency;
- resource consumption;
- quality metrics.

Benchmarking must not define language semantics.

---

111. Repository Integration Rule

Any new language-domain subsystem must provide:

grammar contract
AST contract
semantic contract
IR contract
resource contract
capability contract
test contract

before it becomes part of the stable language.

This applies equally to future domains that do not yet exist.

---

112. No Domain Exception

The following are all subject to the same architecture:

classical
quantum
HDL
hardware
AI
distributed
networking
security
mathematics
future domains

No domain receives permission to bypass the common language architecture.

---

113. No Machine-Size Semantics

A language feature must never derive its meaning from an arbitrary machine-size constant.

Bad:

qubit register means 32 qubits

Good:

qubit register has program-defined cardinality

Bad:

parallel means 8 workers

Good:

parallel execution is mapped to available resources

Bad:

hardware array has 16 processing elements

Good:

hardware array has parameterized cardinality

---

114. Resource Failure Semantics

If a program is semantically valid but cannot be realized, the compiler must report the correct category.

Examples:

Insufficient quantum capability
Insufficient memory
Unsupported target operation
Unachievable timing constraint
Unsupported hardware feature
Insufficient deployment capacity

The compiler must not report:

invalid syntax

unless the source is actually syntactically invalid.

---

115. Graceful Scaling

Scaling should follow:

same source semantics
        +
different resource context
        ↓
different realization

Examples:

small machine
large machine
cluster
supercomputer
QPU
simulator
FPGA
future accelerator

must not require semantic rewriting when the program's requirements remain satisfiable.

---

116. Semantic Portability

Semantic portability is stronger than textual portability.

A program remains portable when its meaning can be preserved despite:

- different instruction sets;
- different memory hierarchies;
- different parallelism;
- different quantum technologies;
- different hardware topologies;
- different operating systems;
- different deployment models.

---

117. Target Independence

Target independence applies only to information that is not part of program meaning.

A program may legitimately require:

real-time behavior
quantum capability
specific precision
specific security property
specific hardware feature

when these are genuine semantic requirements.

The specification must distinguish those requirements from implementation choices.

---

118. Future-Proofing

Zamani must be able to incorporate future computing paradigms without redesigning the core grammar.

Future technologies should fit through:

types
operations
effects
capabilities
resources
constraints
dialects
IR extensions
interoperability

rather than forcing a new core grammar architecture every time a new technology appears.

---

119. Production Readiness

The grammar specification architecture is production-ready only when:

- authority is unambiguous;
- syntax/semantics boundaries are explicit;
- parser/lexer contracts are explicit;
- AST contracts are explicit;
- IR boundaries are explicit;
- quantum IR remains canonical;
- hardware is target-independent at source level;
- resource limits are not hard-coded into grammar;
- versioning is defined;
- extension mechanisms are defined;
- diagnostics are categorized;
- tests cover all domains;
- cross-domain composition is tested;
- scalability is tested;
- safe Rust requirements are enforced;
- generated grammar artifacts are reproducible;
- implementation documentation is synchronized.

---

120. Definition of Done for This File

"grammar/specification/README.md" is complete when:

1. It defines the authority hierarchy.
2. It defines the specification directory's ownership.
3. It establishes the syntax/semantic/IR boundaries.
4. It establishes the POCO-REAF model.
5. It establishes the resource-parametric scalability model.
6. It prohibits accidental machine-size hard-coding.
7. It distinguishes requirements, capabilities, constraints, preferences, hints, resources, and targets.
8. It establishes the quantum grammar boundary.
9. It establishes "quantum::ir" as the canonical quantum semantic boundary.
10. It establishes HDL/hardware boundaries.
11. It establishes classical/quantum/hardware interoperability.
12. It establishes lexer/parser/AST contracts.
13. It establishes semantic and IR integration.
14. It establishes compiler/runtime/backend ownership.
15. It establishes safe Rust 1.97.1 as the implementation baseline.
16. It establishes deterministic parsing.
17. It establishes scalable compiler implementation requirements.
18. It establishes feature lifecycle states.
19. It establishes compatibility rules.
20. It establishes testing requirements.
21. It establishes hard-coding audits.
22. It defines every planned specification file and its ownership.
23. It prevents circular dependencies.
24. It provides enough integration information for dependent specification files to be implemented without reopening this architectural contract.

---

121. Final Zamani Language Principle

The definitive principle of this specification is:

«Zamani describes computation, intent, semantics, capabilities, requirements, constraints, and portable abstractions—not arbitrary limitations of the machine currently available.»

Therefore:

                    ONE PROGRAM
                        │
                        ▼
                 ONE SEMANTIC MEANING
                        │
          ┌─────────────┼─────────────┐
          ▼             ▼             ▼
       CLASSICAL      QUANTUM       HARDWARE
          │             │             │
          └─────────────┼─────────────┘
                        ▼
                 CANONICAL IR
                        │
                        ▼
                MANY REALIZATIONS
                        │
        ┌───────────────┼────────────────┐
        ▼               ▼                ▼
      TINY            LARGE           HETEROGENEOUS
     SYSTEM          SYSTEM             SYSTEM
        │               │                │
        └───────────────┼────────────────┘
                        ▼
                 FUTURE SYSTEMS

The target is:

Zamani
From Atom to Everywhere

through:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

without turning today's hardware limitations into tomorrow's language limitations.

The language scales with the computation; the computation scales with the available resources; the grammar itself imposes no arbitrary finite machine ceiling.