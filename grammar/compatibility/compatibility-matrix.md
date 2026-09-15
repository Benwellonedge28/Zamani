Zamani Compatibility Matrix

Path: "grammar/compatibility/compatibility-matrix.md"
Status: Normative compatibility contract
Scope: Zamani language grammar, parser/frontend, AST, semantic model, IR, compiler, runtime, hardware/resource abstraction, quantum stack, HDL, interoperability, tooling, tests, and future language evolution
Rust implementation baseline: Rust 1.97.1
Safety requirement: "unsafe" Rust is prohibited
Primary design objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

This document defines the compatibility contract for the Zamani language grammar and its integration with the rest of the Zamani repository.

It answers:

- Which syntax is stable?
- Which syntax is experimental?
- Which syntax is deprecated?
- Which changes are breaking?
- Which representations are authoritative?
- How do grammar changes propagate to the parser, AST, semantic model, IR, compiler, runtime, tooling, and documentation?
- How are quantum, classical, HDL, distributed, accelerator, AI, networking, security, and future constructs kept compatible?
- How does Zamani preserve POCO-REAF while hardware and execution environments evolve?
- How are scalability guarantees protected against accidental hard-coded limits?

This file is a compatibility policy, not a second grammar.

It must never become a second source of syntax truth.

---

2. Normative Authority

Zamani compatibility has several distinct layers of authority.

They MUST NOT be confused.

Layer| Authority| Responsibility
Lexical syntax| authoritative grammar source| Tokens and lexical boundaries
Surface grammar| authoritative ANTLR grammar| Accepted Zamani syntax
Parser implementation| conformance implementation| Converts syntax into AST
AST| structural representation| Represents source structure without target semantics
Semantic model| semantic authority| Meaning, typing, effects, capabilities, requirements
"quantum::ir"| canonical quantum semantic boundary| Quantum semantic representation
Domain IR| domain-specific semantic representation| Classical/HDL/other domain lowering
Compiler| transformation authority| Lowering, optimization, target realization
Runtime| execution authority| Runtime resource discovery and execution
Hardware/resource model| physical capability authority| Actual available machine resources
This document| compatibility authority| Versioning and evolution policy

2.1 Critical rule

No compatibility document, example, generated parser, README, or implementation convenience may silently become a competing syntax authority.

If two artifacts disagree, the disagreement MUST be resolved explicitly.

---

3. Current Repository Compatibility Baseline

The current repository contains multiple grammar representations with different historical purposes.

In particular:

- "grammar/Zamani.g4"
- "grammar/Zamani-Grammar.md"
- "grammar/grammar.md"
- Rust lexer/parser/AST implementation
- domain-specific grammar files
- grammar tests and examples

The parser-derived "grammar.md" describes the syntax accepted by the current Rust implementation, while "Zamani.g4" contains a substantially broader language specification.

Therefore the repository MUST NOT treat all existing representations as equally authoritative.

The production architecture establishes:

Normative Zamani language specification
             │
             ▼
      Authoritative grammar
             │
             ▼
       Lexer / Parser
             │
             ▼
            AST
             │
             ▼
   Structural validation
             │
             ▼
      Semantic model
             │
       ┌─────┴─────┐
       ▼           ▼
  Classical/     quantum::ir
  domain IR      canonical
       │           │
       └─────┬─────┘
             ▼
       Optimization
             │
       Routing/Scheduling
             │
       Target realization
             │
             ▼
          Runtime
             │
             ▼
      Available resources

The grammar MUST NOT depend on the runtime to determine whether syntax is valid.

The runtime MUST NOT be required to understand source-level grammar rules.

---

4. Compatibility Principles

Zamani compatibility is governed by the following principles.

4.1 Semantic compatibility over textual compatibility

A source-compatible change is not automatically semantically compatible.

Compatibility MUST consider:

1. lexical meaning;
2. syntactic meaning;
3. AST structure;
4. type meaning;
5. effect meaning;
6. capability meaning;
7. resource requirements;
8. IR meaning;
9. compiler meaning;
10. runtime meaning.

---

4.2 Additive evolution by default

New language capabilities SHOULD be introduced additively.

Examples:

new keyword
new optional clause
new attribute
new capability
new resource constraint
new dialect
new target

must not unnecessarily invalidate existing programs.

However, additive syntax is not allowed if it creates:

- ambiguity;
- incompatible parsing;
- semantic reinterpretation;
- hidden target dependence;
- impossible AST ownership;
- incompatible IR lowering.

---

4.3 No silent reinterpretation

A previously valid program MUST NOT silently acquire a different meaning merely because the compiler version changed.

If semantics must change:

- introduce a language-version boundary;
- provide diagnostics;
- provide migration guidance;
- preserve an old compatibility mode when feasible;
- explicitly document the migration.

---

4.4 No machine-size compatibility promises

Zamani source compatibility MUST NOT depend on fixed machine capacities.

The language MUST NOT define compatibility in terms of:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_MEMORY
MAX_NODES
MAX_DEVICES
MAX_TENSOR_RANK
MAX_CLUSTER_SIZE

unless such a value is an intrinsic representation boundary rather than a machine-capacity assumption.

---

5. POCO-REAF Compatibility Contract

POCO-REAF is a semantic portability requirement.

The source program describes:

- computation;
- data;
- control flow;
- effects;
- capabilities;
- requirements;
- constraints;
- preferences;
- correctness;
- resource-independent intent.

It MUST NOT unnecessarily describe:

- a particular processor;
- a particular quantum device;
- a particular GPU;
- a particular number of nodes;
- a particular physical topology;
- a fixed memory size;
- a fixed accelerator count.

The architecture is:

One source program
       │
       ▼
One stable semantic meaning
       │
       ▼
Architecture-independent representation
       │
       ├── CPU
       ├── GPU
       ├── FPGA
       ├── ASIC
       ├── quantum processor
       ├── simulator
       ├── cluster
       ├── cloud
       ├── embedded system
       └── future architecture

Target-specific decisions belong downstream.

---

6. Compatibility Levels

Every Zamani construct MUST have one compatibility level.

Level 0 — Internal

Implementation-only.

Not part of the public language contract.

May change without source compatibility guarantees.

Examples:

- internal parser node;
- temporary lowering representation;
- parser optimization;
- private compiler metadata.

---

Level 1 — Experimental

Publicly available but not stable.

Experimental constructs MUST be explicitly marked.

They MUST NOT silently become permanent semantics.

Experimental constructs SHOULD use:

- experimental namespace;
- explicit feature/dialect marker;
- documented version;
- compatibility diagnostics.

---

Level 2 — Provisional

Semantics are substantially defined but may receive controlled breaking changes before stabilization.

Provisional features MUST have:

- specification;
- tests;
- migration notes;
- ownership;
- compatibility status.

---

Level 3 — Stable

Stable syntax and semantics.

Breaking changes require:

- language-version change;
- migration policy;
- compatibility documentation;
- diagnostics;
- tests proving old/new behavior where relevant.

---

Level 4 — Long-Term Stable

Core language constructs intended to survive indefinitely.

Examples should include:

- identifiers;
- functions;
- basic types;
- expressions;
- declarations;
- control flow;
- modules;
- core effects;
- resource-independent semantics.

---

7. Language Versioning

Zamani MUST distinguish:

language version
grammar version
AST schema version
IR version
artifact version
runtime compatibility version
dialect version
target capability version

These MUST NOT be conflated.

For example:

language = 2.x
grammar = 2.x
AST schema = 4.x
quantum IR = 3.x
runtime = 5.x
target capability = device-defined

A runtime implementation version MUST NOT automatically redefine language syntax.

---

8. Compatibility Matrix

Component| Backward compatibility| Forward compatibility| Breaking change policy
Lexer| Required for stable tokens| Limited| Versioned token changes
Parser| Required for stable syntax| Reject unknown syntax deterministically| Language-versioned
AST| Stable semantic structure| Unknown extensions represented explicitly where possible| AST schema version
Type system| Stable meaning| New types additive| Language/semantic version
Expressions| Stable precedence/meaning| New operators require explicit grammar evolution| Versioned
Statements| Stable meaning| New forms additive| Versioned
Modules| Stable import/export meaning| Unknown module metadata ignored only when explicitly allowed| Versioned
Effects| Stable effect identity| New effects additive| Effect version
Resources| Semantic requirements stable| New capabilities additive| Capability schema
Classical IR| Semantic compatibility| New operations via versioned extension| IR version
"quantum::ir"| Canonical quantum semantics| Extension points required| Quantum IR version
QEC| Semantic compatibility| New codes/strategies additive| Domain version
ZQN| Fault/noise semantic compatibility| New fault models additive| Domain version
Scheduling| Semantic ordering preserved| New policies additive| Scheduler capability
Routing| Logical semantics preserved| New physical realizations additive| Target capability
HDL| Hardware semantics preserved| New target constructs additive| HDL dialect version
Hardware model| No source dependence on device size| Capability discovery| Hardware capability version
Runtime| Program semantics preserved| New execution environments| Runtime compatibility contract
Tooling| Diagnostics/schema compatibility| Unknown metadata tolerated when safe| Tool protocol version
Examples| Must remain valid unless intentionally versioned| N/A| CI enforced
Documentation| Must reflect normative grammar| N/A| Documentation validation

---

9. Lexical Compatibility

The lexer MUST provide deterministic tokenization.

Stable token spellings MUST NOT change silently.

A keyword can become reserved only through an explicit compatibility process.

Before introducing a new keyword:

1. Search existing source identifiers.
2. Determine collision impact.
3. Determine whether contextual keyword behavior is possible.
4. Determine parser ambiguity.
5. Provide migration guidance.
6. Add compatibility tests.

9.1 Identifier compatibility

If a previously legal identifier becomes a keyword, this is a source-breaking change.

Preferred migration:

old_identifier

to:

escaped_identifier

if the language provides identifier escaping.

If identifier escaping does not exist, the keyword SHOULD remain contextual where practical.

---

10. Operator Compatibility

Operators MUST have globally defined precedence and associativity.

Changing:

precedence
associativity
arity
overload meaning

is potentially breaking.

Such changes require:

- parser tests;
- AST comparison tests;
- semantic tests;
- migration documentation.

No new operator may create an unintended parse ambiguity.

---

11. Type Compatibility

Type syntax and type semantics MUST remain distinct.

For example:

Optional<T>
Result<T, E>
Array<T>
Vector<T>
Tensor<T>
Qubit
LogicalQubit
PhysicalQubit
Resource<T>
Capability<T>

must not be interpreted as machine-specific resource guarantees merely because they have physical implementations.

Type compatibility MUST distinguish:

semantic type
resource requirement
hardware capability
physical representation

---

12. Resource Compatibility

Resource expressions MUST be semantic.

Correct:

requires quantum
requires capability quantum.entanglement
requires memory >= requirement
requires accelerator capability("tensor")
requires parallelism

Incorrect architecture:

requires 64 cpu
requires 32 qubits
requires gpu0
requires topology ring32

unless the programmer is deliberately expressing an actual semantic constraint rather than an accidental implementation assumption.

Even explicit resource quantities MUST remain runtime/compiler constraints rather than grammar-level maximums.

---

13. Capability Compatibility

Capabilities represent what an execution environment can provide.

Examples:

quantum
classical
vector
tensor
parallel
distributed
gpu
fpga
asic
simulation
network
secure
trusted
accelerated

Capabilities MUST NOT imply a particular vendor or device.

For example:

requires capability quantum

does not mean:

requires IBM device X

or:

requires N physical qubits

The compiler/runtime performs capability negotiation and realization.

---

14. Constraint Compatibility

Constraints describe requirements that MUST be satisfied.

Examples:

requires
constrain
ensure
where

A constraint MUST NOT silently become an implementation decision.

The distinction is:

Requirement
    ↓
Capability matching
    ↓
Candidate target selection
    ↓
Optimization
    ↓
Placement
    ↓
Scheduling
    ↓
Execution

---

15. Preference Compatibility

Preferences are not guarantees.

For example:

prefer gpu
prefer low_latency
prefer energy_efficiency

must not mean:

must use gpu
must achieve a fixed latency
must use a specific device

The semantic model MUST preserve this distinction.

---

16. Hardware Compatibility

Hardware syntax MUST describe hardware semantics without requiring a fixed hardware universe.

Supported conceptual categories include:

CPU
GPU
FPGA
ASIC
accelerator
memory
interconnect
device
controller
interface
clock
pipeline
resource
capability
topology

Hardware descriptions MUST be parameterized.

Bad:

gpu_count = 8
cores = 64
memory = 128GB

as universal language semantics.

Better:

requires capability gpu
requires memory >= requested
requires parallel execution

Physical values may appear in:

- hardware descriptions;
- deployment specifications;
- target configurations;
- test fixtures;
- benchmarking;
- explicit optimization constraints.

---

17. Quantum Compatibility

Quantum source syntax MUST remain hardware-independent unless physical realization is explicitly requested.

The grammar MAY express:

qubit
logical qubit
physical qubit
register
state
gate
operation
measurement
observable
reset
controlled operation
parameterized operation
dynamic circuit
mid-circuit measurement
classical control
quantum resource
error correction

But source syntax MUST NOT impose arbitrary maximums.

Invalid architectural assumptions include:

q[0]
q[1]

as universal semantic requirements.

Valid examples include dynamically resolved references and resource-derived dimensions.

---

18. "quantum::ir" Compatibility

"quantum::ir" remains the canonical quantum semantic boundary.

The grammar MUST NOT create a competing quantum semantic IR.

The compatibility flow is:

Quantum source
      │
      ▼
Quantum grammar
      │
      ▼
AST
      │
      ▼
Semantic validation
      │
      ▼
quantum::ir
      │
      ├── optimization
      ├── QEC
      ├── ZQN
      ├── routing
      ├── scheduling
      ├── calibration
      ├── HAL
      └── backend realization

Quantum grammar changes therefore require evaluation against:

- AST representation;
- "quantum::ir";
- QEC;
- ZQN;
- routing;
- scheduling;
- HAL;
- runtime.

---

19. Quantum Resource Compatibility

Quantum resource declarations MUST distinguish:

logical resource
physical resource
capability
requirement
constraint
implementation choice

For example:

requires logical qubits

does not determine:

physical qubit count

The compiler/runtime may map logical resources to physical resources through:

QEC
routing
hardware topology
calibration
scheduling
resource management

---

20. QEC Compatibility

The grammar may express QEC intent.

It MUST NOT duplicate QEC implementation policy.

The grammar MUST NOT define independent global resource limits that conflict with the repository's canonical QEC resource policy.

The existing QEC architecture establishes "QecLimits" as the declarative resource policy boundary.

Therefore:

grammar
  → semantic QEC requirement
  → QEC subsystem
  → QecLimits/resource policy
  → decoder/correction

The grammar MUST NOT invent a second "QecLimits".

QEC implementation details remain owned by the quantum error-correction subsystem.

---

21. ZQN Compatibility

ZQN/fault semantics belong to the quantum fault/noise semantic layer.

The grammar may express:

- fault model selection;
- resilience intent;
- acceptable fault characteristics;
- error-awareness;
- verification requirements.

The grammar MUST NOT encode physical fault behavior as parser constants.

For example:

fault_model = ...

may identify semantic behavior.

It must not imply:

maximum fixed number of faults

unless that is an explicit program-level semantic constraint.

---

22. Scheduling Compatibility

Scheduling syntax MUST express ordering/resource intent.

The grammar MUST NOT hard-code:

- number of scheduler lanes;
- number of devices;
- number of cores;
- number of qubits;
- fixed execution slots.

Scheduling remains downstream of semantic representation.

The compatibility flow is:

source intent
   ↓
AST
   ↓
semantic model
   ↓
IR
   ↓
resource discovery
   ↓
routing
   ↓
scheduling
   ↓
execution

---

23. Routing Compatibility

Routing is a physical realization concern.

Logical program semantics MUST remain valid independent of physical topology whenever the program does not explicitly require topology semantics.

The grammar MUST NOT require:

line topology
ring topology
mesh topology
fixed qubit adjacency
fixed node adjacency

unless topology itself is part of the program's explicit semantics.

---

24. HDL Compatibility

HDL syntax MUST distinguish:

hardware behavior
hardware structure
hardware timing
hardware interfaces
hardware parameters
hardware resources
physical realization

A hardware module should remain portable across implementation scales where semantics permit.

For example:

pipeline

describes a semantic structure.

A specific number of pipeline stages is an implementation property unless explicitly required.

---

25. Classical Compatibility

Stable classical constructs include:

- expressions;
- statements;
- declarations;
- functions;
- types;
- control flow;
- modules;
- memory semantics;
- concurrency;
- generic abstractions.

Classical language evolution MUST NOT prevent future quantum/HDL/hybrid extensions.

The core grammar therefore MUST remain domain-neutral.

---

26. Hybrid Compatibility

Hybrid programs may combine:

classical
quantum
HDL
accelerator
distributed
AI
data

without requiring separate source languages.

The semantic boundary must remain explicit.

Example architecture:

classical source
      │
quantum source ──► common AST ──► semantic model
      │                              │
HDL source ──────────────────────────┤
                                     ▼
                              domain IRs

The AST MUST NOT become a hardware-specific or quantum-specific IR.

---

27. AI/Data Compatibility

AI/data constructs MUST avoid embedding fixed deployment dimensions.

Tensor dimensions may be explicit when they are semantic properties.

However:

device count
GPU count
memory capacity
cluster size

are not intrinsic tensor semantics.

The language MUST permit runtime/compiler selection of available acceleration resources.

---

28. Distributed Compatibility

Distributed syntax MUST distinguish:

logical node
service
communication
message
replication
consistency
placement intent
fault tolerance

from:

actual machine count
IP addresses
physical cluster topology
cloud provider

Logical distribution MUST remain portable.

---

29. Networking Compatibility

Network syntax MAY describe semantic endpoints, protocols, channels, messages, and services.

Physical addresses MUST NOT be embedded as universal grammar constraints.

Endpoint discovery and deployment configuration belong downstream.

---

30. Security Compatibility

Security syntax MUST preserve semantic intent.

Examples:

permission
capability
identity
trust
privacy
cryptographic requirement
security policy

must remain independent from a particular operating system or vendor implementation where possible.

Security-breaking language changes require explicit compatibility review.

---

31. Interoperability Compatibility

Foreign interfaces include:

C
C++
Python
OpenQASM
Verilog
SystemVerilog
VHDL
ABI
FFI
system interfaces

Interoperability syntax MUST be isolated from core language semantics.

A foreign dialect MUST NOT force the core grammar to inherit foreign implementation limitations.

---

32. Dialect Compatibility

Dialects MUST have:

name
namespace
version
capabilities
syntax ownership
semantic ownership
compatibility policy

Vendor-specific syntax MUST be isolated.

Vendor extensions MUST NOT silently become universal Zamani semantics.

Recommended structure:

dialect vendor.namespace version

with explicit capability registration.

---

33. Experimental Compatibility

Experimental constructs MUST be identifiable.

They SHOULD be placed under:

grammar/dialects/

or another explicit extension boundary.

Experimental syntax MUST NOT silently become stable syntax.

When stabilized:

experimental
    ↓
provisional
    ↓
stable

The migration MUST be documented.

---

34. Deprecated Syntax

Deprecated constructs MUST remain parseable for the documented deprecation window unless they introduce unacceptable ambiguity or security problems.

Deprecation requires:

1. documentation;
2. warning/diagnostic;
3. replacement syntax;
4. migration example;
5. compatibility test;
6. removal target version.

Deprecated syntax MUST NOT be removed merely because a cleaner syntax exists.

---

35. Reserved Syntax

Reserved words and syntax MUST be documented centrally.

Reserved syntax may be held for:

- future language constructs;
- future hardware models;
- future quantum models;
- future distributed semantics;
- future compiler features.

Reserved syntax MUST NOT accidentally become available as ordinary identifiers.

---

36. AST Compatibility

The AST MUST represent syntax structurally and generically.

The AST MUST NOT become:

- LLVM IR;
- QIR;
- MLIR;
- hardware topology;
- QEC representation;
- scheduler representation;
- vendor backend representation.

Quantum operations should use generic operation structures rather than forcing a closed enum of today's gates.

Conceptually:

Operation {
    name,
    namespace,
    operands,
    parameters,
    results,
    attributes,
    modifiers,
    effects,
    capabilities,
    source
}

This permits future operations without changing the fundamental AST architecture.

---

37. IR Compatibility

IR changes MUST preserve semantic meaning.

An IR extension MUST specify:

operation identity
operands
results
types
attributes
effects
capabilities
resource requirements
lowering behavior
version

An IR version MUST NOT be inferred solely from the source-language version.

---

38. Compiler Compatibility

Compiler passes MUST consume semantic representations.

They MUST NOT inspect source text to determine hardware-specific behavior where the semantic model already represents that information.

Correct:

AST
 → semantic model
 → IR
 → optimization
 → routing
 → scheduling
 → target

Incorrect:

AST
 → compiler guesses hardware
 → parser changes semantics

---

39. Runtime Compatibility

The runtime owns actual execution.

Runtime discovery may determine:

- available devices;
- resource capacity;
- capabilities;
- topology;
- calibration;
- current health;
- scheduling availability.

The runtime MUST NOT redefine the language semantics.

When resources are insufficient, the runtime SHOULD report a structured capability/resource failure rather than causing undefined semantic behavior.

---

40. Hardware Scaling Contract

Zamani MUST support scaling over the resource range actually available.

The language must not impose artificial ceilings.

The practical limits of an execution are determined by:

available resources
compiler resource policy
runtime resource policy
target capabilities
memory
time
storage
network capacity
hardware limits
algorithmic complexity

not by arbitrary grammar constants.

"Infinite" or "unbounded" language scalability therefore means:

«No artificial language-level machine-size ceiling is imposed where the semantic model can represent the computation.»

It does not mean physical hardware limitations disappear.

---

41. Integer and Representation Boundaries

A representation limit is different from a machine-capacity limit.

For example:

u64
usize
source-file byte offsets
token positions
AST node identifiers

may have implementation representation bounds.

These MUST be documented separately.

They MUST NOT be presented as:

maximum program size
maximum qubits
maximum nodes
maximum devices

unless they genuinely are such semantic restrictions.

Where repository architecture permits, resource quantities SHOULD use representations capable of expressing arbitrary valid resource requests without artificial grammar ceilings.

---

42. No "unsafe" Compatibility Policy

The Rust implementation of the grammar infrastructure MUST use:

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

where applicable.

No compatibility feature may require unsafe Rust.

Generated parser code MUST be audited for the selected ANTLR/Rust generation strategy.

If a dependency introduces unsafe implementation internally, that is a dependency concern; Zamani source code itself MUST NOT introduce unsafe blocks or unsafe functions.

---

43. Rust 1.97.1 Compatibility

The production implementation baseline is:

Rust 1.97.1
Edition: repository-defined compatible edition

The grammar infrastructure MUST compile under Rust 1.97.1.

Language/library features introduced after Rust 1.97.1 MUST NOT become mandatory implementation dependencies unless the repository explicitly raises its minimum Rust version.

The compatibility policy is about the Rust implementation toolchain; it does not limit the Zamani language's computational expressiveness.

---

44. Parser Compatibility

Parser behavior MUST be deterministic.

For identical:

source
grammar version
dialect set
language version

the parser MUST produce the same structural result.

Parser changes MUST include:

- valid syntax tests;
- invalid syntax tests;
- ambiguity tests;
- precedence tests;
- diagnostic tests;
- AST compatibility tests.

---

45. Grammar Change Classification

Every grammar change MUST be classified as one of:

Change| Classification
New unambiguous syntax| Additive
New optional attribute| Additive
New dialect| Additive
New capability| Additive
New resource expression| Additive if semantics are independent
New keyword| Potentially breaking
Removed keyword| Breaking
Changed precedence| Breaking
Changed associativity| Breaking
Changed literal meaning| Breaking
Changed type meaning| Breaking
Changed IR meaning| Semantic breaking
Changed diagnostics only| Usually non-breaking
New optimization| Non-breaking if semantics preserved
New backend| Non-breaking
New hardware target| Non-breaking
New runtime capability| Non-breaking
Removal of deprecated syntax| Versioned breaking
Vendor extension| Dialect-scoped

---

46. Compatibility Review Procedure

Every grammar change MUST pass this sequence.

1. Identify affected syntax
        ↓
2. Identify affected AST nodes
        ↓
3. Identify semantic contracts
        ↓
4. Identify affected IR
        ↓
5. Identify compiler consumers
        ↓
6. Identify runtime consumers
        ↓
7. Identify tooling
        ↓
8. Identify examples/tests
        ↓
9. Run hard-coding audit
        ↓
10. Classify compatibility
        ↓
11. Add migration if required
        ↓
12. Update matrix
        ↓
13. Validate full repository

No grammar file should be declared complete before this analysis is performed for changes affecting its public contract.

---

47. Cross-File Integration Contract

The compatibility matrix integrates with the following grammar areas.

File/area| Integration responsibility
"specification/language-version.md"| Language-version semantics
"specification/grammar-authority.md"| Authority hierarchy
"specification/poco-reaf.md"| Portability guarantees
"core/versioning.g4"| Surface version declarations
"core/capabilities.g4"| Capability syntax
"core/requirements.g4"| Requirements
"core/constraints.g4"| Constraints
"resources/*.g4"| Resource semantics
"dialects/*.g4"| Extension compatibility
"validation/compatibility-rules.md"| Validation rules
"compatibility/versions.md"| Version history
"compatibility/migrations.md"| Migration procedures
"compatibility/deprecated.md"| Deprecation policy
"compatibility/reserved.md"| Reserved syntax
"tests/compatibility/"| Automated compatibility validation
"tests/scalability/"| Scale-independence validation

This file owns the compatibility matrix and policy.

It does not own the grammar rules themselves.

---

48. Ownership Boundary

This file OWNS:

- compatibility classifications;
- compatibility levels;
- compatibility matrix;
- version compatibility policy;
- breaking-change policy;
- migration requirements;
- cross-domain compatibility rules;
- POCO-REAF compatibility guarantees;
- hardware-independence compatibility rules.

This file DOES NOT OWN:

- lexical token definitions;
- parser productions;
- AST implementation;
- semantic type checking;
- quantum IR;
- QEC implementation;
- ZQN implementation;
- scheduling implementation;
- routing implementation;
- runtime resource discovery;
- hardware drivers.

---

49. Compatibility With Existing Quantum Infrastructure

The existing repository's QEC architecture already separates declarative resource limits from actual runtime/resource accounting.

The compatibility model MUST preserve this separation:

language requirement
        │
        ▼
semantic resource requirement
        │
        ▼
QEC / resource policy
        │
        ▼
runtime resource observation
        │
        ▼
execution decision

A grammar change MUST NOT introduce a second resource-policy authority.

The same principle applies to:

- scheduling;
- routing;
- calibration;
- HAL;
- benchmarking;
- runtime resource management.

---

50. Compatibility With Existing "grammar.md"

"grammar/grammar.md" is currently described as the parser-derived description of what the Rust implementation accepts.

Therefore it MUST NOT silently remain a competing normative specification.

Production policy:

Authoritative grammar/specification
              │
              ├── generated/validated parser grammar
              │
              └── derived grammar documentation

If "grammar.md" remains, its status MUST be explicitly documented as:

derived implementation reference

or it must be regenerated from the authoritative grammar/specification.

It MUST NOT claim authority over syntax that the authoritative grammar intentionally defines differently.

---

51. Compatibility With "Zamani.g4"

"Zamani.g4" contains substantial existing language functionality.

No valid feature should be silently removed.

Before changing it:

1. inventory existing productions;
2. identify parser consumers;
3. identify AST consumers;
4. classify semantic ownership;
5. preserve valid constructs;
6. migrate misplaced constructs;
7. deprecate only with policy;
8. remove only with explicit compatibility justification.

The production grammar architecture may split "Zamani.g4" into maintainable grammar modules, but the resulting composed grammar MUST preserve the public language contract.

---

52. Generated Artifact Policy

Generated parser/lexer artifacts MUST NOT become authoritative source files.

Generated artifacts are:

derived
reproducible
versioned according to project policy
validated against source grammar

The repository MUST be able to determine:

source grammar
    ↓
generation process
    ↓
generated parser
    ↓
tests

A generated artifact modified manually without corresponding source-grammar changes MUST fail validation.

---

53. Documentation Compatibility

Documentation MUST identify whether a document is:

normative
explanatory
generated
historical
experimental
deprecated

A documentation file MUST NOT describe unsupported syntax as stable syntax.

Examples MUST be compiled or parser-tested where feasible.

---

54. Example Compatibility

Every canonical example MUST have:

- language version;
- required dialects;
- expected parse result;
- expected semantic result where practical;
- compatibility status.

Examples MUST NOT contain hidden fixed-resource assumptions unless the example is specifically a hardware/deployment example.

---

55. Negative Compatibility Tests

The compatibility suite MUST test:

- removed keywords;
- invalid versions;
- invalid dialect versions;
- invalid capability expressions;
- malformed resource constraints;
- ambiguous syntax;
- unsupported vendor syntax;
- invalid quantum constructs;
- invalid hybrid constructs;
- invalid HDL constructs;
- incompatible AST constructs;
- unsupported IR versions.

The parser MUST fail deterministically.

---

56. Forward Compatibility

Where safe, future-compatible structures SHOULD permit unknown metadata to survive without semantic corruption.

For example:

attributes
metadata
dialect extensions
capability descriptors
resource annotations

may support extension namespaces.

However, unknown semantic operations MUST NOT be silently accepted as known operations.

The rule is:

unknown metadata → potentially ignorable
unknown semantics → explicit error

unless a defined extension mechanism exists.

---

57. Source Migration Policy

Migration tools SHOULD support:

old source
   ↓
version-aware parser
   ↓
migration transformation
   ↓
new source

Migration MUST preserve semantics.

A migration that merely changes syntax while altering execution meaning is invalid.

---

58. Deprecation Lifecycle

The lifecycle is:

Stable
  ↓
Deprecated
  ↓
Migration-supported
  ↓
Removal candidate
  ↓
Removed in declared breaking version

Every deprecated construct MUST identify:

deprecated since
replacement
removal target
migration method
reason

---

59. Reserved-Keyword Lifecycle

Reserved syntax follows:

unreserved
  ↓
reserved
  ↓
experimental
  ↓
provisional
  ↓
stable

Reserved syntax MUST NOT be assigned casually.

A reserved identifier must have a documented reason.

---

60. Scalability Compatibility Matrix

Property| Grammar may constrain?| Correct authority
Program semantics| Yes| Language
Type rules| Yes| Type system
Qubit count| No arbitrary maximum| Resource/compiler/runtime
CPU count| No arbitrary maximum| Resource/runtime
GPU count| No arbitrary maximum| Resource/runtime
FPGA count| No arbitrary maximum| Hardware/runtime
Memory capacity| No arbitrary maximum| Runtime/resource
Cluster size| No arbitrary maximum| Deployment/runtime
Network size| No arbitrary maximum| Deployment/runtime
Tensor dimension| Only when semantically intrinsic| Type/semantic model
Physical topology| Only when semantically required| Hardware model
Device ID| Only in explicit deployment/target contexts| Target/runtime
Address| Only where address is semantic| Platform/interop
QEC limits| No duplicate policy| QEC resource policy
Scheduling capacity| No grammar ceiling| Scheduler/resource manager
Routing topology| No hidden fixed topology| Routing/HAL
Execution parallelism| Intent only| Scheduler/runtime

---

61. Hard-Coding Compatibility Audit

Every grammar release MUST be audited for:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_DEVICES
MAX_NODES
MAX_MEMORY
MAX_TENSOR_DIM
MAX_CLUSTER_SIZE
MAX_TOPOLOGY_SIZE

and equivalent hidden forms.

Each discovered constant MUST be classified:

1. Semantic invariant
2. Representation invariant
3. Resource policy
4. Runtime limit
5. Test fixture
6. Serialization limit
7. Accidental hard-code

Only accidental hard-coding MUST be removed automatically.

A genuine representation invariant MUST be documented rather than disguised as unlimited capacity.

---

62. Compatibility and Determinism

The same source, language version, dialect set, and grammar version MUST produce deterministic parsing.

Compatibility tests MUST verify:

source
 → lexer tokens
 → parser tree
 → AST

is stable.

Semantic determinism must be separately tested at:

AST
 → semantic model
 → IR

Runtime nondeterminism MUST NOT be confused with parser nondeterminism.

---

63. Compatibility and Round Trips

Where a canonical printer/serializer exists:

source
 → AST
 → canonical form
 → AST

must preserve semantics.

For IR:

source
 → semantic model
 → IR
 → serialization
 → deserialization

must preserve the defined IR contract.

Version migration must be explicit.

---

64. Compatibility and Security

A grammar compatibility change MUST be reviewed for:

- parser denial-of-service;
- ambiguity explosions;
- excessive recursion;
- unbounded diagnostic memory;
- pathological token streams;
- unsafe generated code;
- foreign-function boundary changes;
- capability bypasses.

No compatibility feature may weaken security boundaries merely to preserve syntax.

---

65. Compatibility and Diagnostics

Diagnostics are part of the developer-facing compatibility contract, but exact wording SHOULD NOT normally be treated as a stable ABI.

Stable diagnostic properties SHOULD include:

error category
source span
machine-readable code
severity
migration guidance where applicable

Human-readable wording may evolve.

---

66. Compatibility and Tooling

Language servers, formatters, linters, documentation generators, IDEs, and build tools MUST consume the same authoritative grammar/version information.

Tooling MUST NOT maintain private grammar forks.

If a tool intentionally supports only a subset, it MUST declare the supported language version and feature set.

---

67. Compatibility and Build Artifacts

Compiled Zamani artifacts MUST carry sufficient version information to determine compatibility.

At minimum, where applicable:

language version
grammar/schema version
IR version
dialect versions
target requirements
capability requirements

Runtime MUST reject incompatible artifacts deterministically.

---

68. Compatibility and Reproducibility

A program compiled under a fixed:

language version
grammar version
dialect set
compiler version
IR version

SHOULD produce reproducible semantic artifacts where deterministic compilation is requested.

Hardware-specific scheduling and runtime decisions may vary when the target environment differs.

That variation MUST NOT alter source-level semantics.

---

69. Compatibility Across Machine Scale

The same semantic program MUST remain valid across:

atom-scale conceptual computation
tiny embedded systems
single-core machines
multicore systems
manycore systems
GPU systems
FPGA systems
ASIC systems
quantum systems
hybrid systems
clusters
supercomputers
cloud systems
distributed systems
future computational substrates

provided the target satisfies the program's semantic requirements.

The grammar MUST NOT need a new language syntax merely because resource quantity increases.

---

70. Compatibility Matrix for Major Domains

Domain| Source semantics| Target realization| Compatibility requirement
Classical| Algorithm/data/control| CPU/GPU/etc.| Preserve semantics
Quantum| Quantum operations| QPU/simulator| Preserve quantum meaning
Hybrid| Quantum/classical interaction| Heterogeneous system| Preserve boundary
HDL| Hardware behavior| FPGA/ASIC/RTL| Preserve hardware semantics
HPC| Parallel computation| Cluster/supercomputer| Preserve computation
AI| Models/tensors/training| CPU/GPU/accelerator| Preserve model semantics
Data| Data/schema/transforms| Local/distributed/cloud| Preserve data semantics
Networking| Communication intent| Network implementation| Preserve protocol semantics
Security| Security policy| Platform/security backend| Preserve policy
Embedded| Resource-aware semantics| MCU/SoC/etc.| Preserve semantics
Distributed| Logical distribution| Actual topology| Preserve logical behavior
Future| Extension/dialect| Future backend| Preserve extension contract

---

71. Compatibility Rules for New Grammar Files

Before adding a grammar file, the author MUST define:

File:
Purpose:
Owns:
Does Not Own:
Inputs:
Outputs:
Dependencies:
Upstream Contracts:
Downstream Consumers:
Public Grammar Contract:
AST Contract:
Semantic Contract:
IR Integration:
Compiler Integration:
Runtime Integration:
Tooling Integration:
Cross-Domain Integration:
Tests:
Negative Tests:
Boundary Tests:
Compatibility Requirements:
Scalability Requirements:
Hard-Coding Audit:
Completion Criteria:

The file MUST NOT be considered complete until all fields are resolved.

This ensures independent completion without requiring architectural redesign when later files are implemented.

---

72. Dependency Rule

Compatibility metadata may be consumed downstream, but grammar modules MUST NOT create circular dependencies.

Forbidden:

grammar → runtime → grammar
grammar → IR → grammar
quantum grammar → scheduler → quantum grammar

Required direction:

grammar
   ↓
AST
   ↓
semantic model
   ↓
IR
   ↓
compiler
   ↓
runtime

Capability/resource information may flow back as compilation inputs, but this MUST NOT make runtime implementation part of grammar ownership.

---

73. Compatibility Test Categories

The compatibility suite MUST include:

Positive

Previously valid programs remain valid.

Negative

Previously invalid programs remain invalid unless intentionally promoted.

Boundary

Smallest and largest practical representations.

Cross-domain

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

Scalability

No source-level artificial machine-size limits.

Version

Old versions parse under their declared rules.

Migration

Deprecated syntax migrates correctly.

Round-trip

Parser/printer preserves semantics.

Determinism

Repeated parsing gives equivalent results.

Security

Pathological syntax does not bypass parser/security boundaries.

---

74. Compatibility CI Requirements

CI SHOULD validate at minimum:

cargo fmt --check
cargo check
cargo test
cargo test --all-targets
grammar validation
ANTLR grammar generation
parser tests
negative grammar tests
compatibility tests
round-trip tests
hard-coding audit
documentation consistency

All must use Rust 1.97.1 for the supported baseline.

No unsafe Rust is permitted.

---

75. Release Compatibility Checklist

Before a grammar release:

- [ ] Authority hierarchy is unchanged or explicitly updated.
- [ ] Language version is identified.
- [ ] Grammar version is identified.
- [ ] AST compatibility reviewed.
- [ ] Semantic compatibility reviewed.
- [ ] IR compatibility reviewed.
- [ ] Quantum compatibility reviewed.
- [ ] QEC integration reviewed.
- [ ] ZQN integration reviewed.
- [ ] routing integration reviewed.
- [ ] scheduling integration reviewed.
- [ ] hardware integration reviewed.
- [ ] runtime integration reviewed.
- [ ] tooling integration reviewed.
- [ ] interoperability reviewed.
- [ ] deprecated syntax reviewed.
- [ ] reserved syntax reviewed.
- [ ] migration documentation updated.
- [ ] compatibility tests pass.
- [ ] scalability tests pass.
- [ ] hard-coding audit passes.
- [ ] no unsafe Rust introduced.
- [ ] Rust 1.97.1 compatibility verified.
- [ ] examples pass.
- [ ] generated artifacts are reproducible.
- [ ] no competing grammar authority was introduced.

---

76. Completion Criteria

"grammar/compatibility/compatibility-matrix.md" is complete only when:

1. Every public grammar feature has a compatibility classification.
2. Every language-version transition has defined rules.
3. Every stable construct has backward-compatibility expectations.
4. Breaking changes have a documented procedure.
5. Experimental constructs are identifiable.
6. Deprecated constructs have migration rules.
7. Reserved syntax is documented.
8. Quantum compatibility explicitly preserves "quantum::ir" as the canonical quantum semantic boundary.
9. QEC compatibility preserves the repository's canonical resource-policy ownership.
10. Hardware compatibility does not impose arbitrary machine-size limits.
11. POCO-REAF is explicitly protected.
12. Cross-domain compatibility is defined.
13. AST/IR/compiler/runtime ownership boundaries are explicit.
14. Generated artifacts cannot silently become grammar authority.
15. Compatibility tests are specified.
16. Scalability tests are specified.
17. Hard-coding audits are specified.
18. Rust 1.97.1 is supported.
19. Zamani implementation code remains free of "unsafe".
20. The document does not create a second grammar or semantic IR.

---

77. Final Compatibility Invariant

The fundamental invariant of Zamani is:

SOURCE SEMANTICS
       ≠
CURRENT MACHINE

Instead:

SOURCE SEMANTICS
       ↓
PORTABLE SEMANTIC MODEL
       ↓
CAPABILITIES + REQUIREMENTS + CONSTRAINTS + PREFERENCES
       ↓
COMPILATION / LOWERING
       ↓
RESOURCE DISCOVERY
       ↓
ROUTING / SCHEDULING / OPTIMIZATION
       ↓
TARGET REALIZATION
       ↓
RUNTIME

Therefore:

«A change in available hardware must not require rewriting a Zamani program merely because the hardware became larger, smaller, faster, slower, quantum, classical, heterogeneous, distributed, embedded, or otherwise different.»

And:

«A compatibility change must never turn an implementation detail into permanent language semantics merely because one current target happens to have that limitation.»

The long-term contract is:

One Program
     ↓
One Semantic Meaning
     ↓
One Portable Compilation Model
     ↓
Many Targets
     ↓
Many Architectures
     ↓
Many Resource Configurations
     ↓
Many Execution Environments
     ↓
Future Computing Paradigms

This is the compatibility foundation required for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

and for Zamani's governing principle:

From Atom to Everywhere.