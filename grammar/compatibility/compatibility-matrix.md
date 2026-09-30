Zamani Compatibility Matrix

Path: "grammar/compatibility/compatibility-matrix.md"
Status: Normative
Scope: Repository-wide language, grammar, frontend, semantic, IR, compiler, runtime, resource, hardware, quantum, classical, HDL, hybrid, distributed, AI, networking, security, interoperability, dialect, tooling, and compatibility integration
Language: Zamani
Grammar technology: ANTLR4
Rust edition: 2021
Rust implementation baseline: Rust 1.97 / Rust 1.97.1
Rust safety policy: Zamani implementation code MUST NOT use Rust "unsafe"
Primary portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)

---

1. Purpose

This file defines the repository-wide compatibility matrix for Zamani.

It establishes how independently maintained language layers remain compatible without creating competing authorities.

This file answers:

- What does compatibility mean for Zamani?
- Which repository artifact owns each compatibility decision?
- Which changes are source-breaking?
- Which changes are AST-breaking?
- Which changes are semantic-breaking?
- Which changes are IR-breaking?
- Which changes are target-specific?
- How are language versions separated from compiler versions?
- How are dialects versioned?
- How are deprecated constructs migrated?
- How are future extensions handled?
- How are quantum, classical, HDL, hybrid, distributed, AI, networking, security, and future computing features integrated?
- How is "quantum::ir" protected as the canonical quantum semantic boundary?
- How are QEC and ZQN kept downstream of the grammar?
- How is POCO-REAF preserved?
- How is scalability protected from artificial hardware limits?
- How are Rust 1.97 / 1.97.1 and safe-Rust requirements enforced?
- How does a grammar change propagate through the complete repository?

This file is a compatibility authority only.

It is not:

- a second grammar;
- a second lexer;
- a second AST;
- a second semantic model;
- a second quantum IR;
- a runtime specification;
- a hardware specification;
- a QEC implementation specification;
- a ZQN implementation specification.

---

2. Authority Model

Zamani has multiple authoritative artifacts, but each artifact owns a different concern.

They MUST NOT be treated as interchangeable.

2.1 Authority hierarchy

The production relationship is:

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ├── language specification
        ├── grammar authority
        ├── language version
        └── semantic contracts
        │
        ▼
grammar/spec/
        │
        ├── formal contracts
        ├── type system
        ├── effects
        ├── resources
        ├── diagnostics
        ├── domains
        └── compatibility contracts
        │
        ▼
grammar/Zamani.g4
        │
        ▼
canonical lexical/parser implementation
        │
        ▼
frontend AST
        │
        ▼
semantic analysis
        │
        ▼
canonical semantic representation
        │
        ├───────────────┬────────────────┐
        ▼               ▼                ▼
   classical        quantum::ir        HDL/
   semantics         canonical       hardware semantics
        │               │                │
        └───────────────┼────────────────┘
                        ▼
                 optimization/lowering
                        │
          ┌─────────────┼──────────────┐
          ▼             ▼              ▼
       routing       scheduling      resilience
                                         │
                              ┌──────────┼──────────┐
                              ▼          ▼          ▼
                             QEC        ZQN        other
                              │          │
                              └────┬─────┘
                                   ▼
                                  HAL
                                   │
                                   ▼
                              target/runtime

2.2 Ownership matrix

Artifact| Owns| Does not own
"grammar/DESIGN.md"| Architecture and boundaries| Individual feature syntax
"grammar/specification/"| Normative language specification| Implementation internals
"grammar/spec/"| Formal contracts| Release migration execution
"grammar/Zamani.g4"| Canonical ANTLR composition| Runtime behavior
"grammar/lexer/"| Lexical contracts| Semantic interpretation
"grammar/grammar.md"| Implementation/conformance reference| Normative language authority
"grammar/Zamani-Grammar.md"| Historical/extended/proposed design material| Automatic stable syntax
"grammar/compatibility/versions.md"| Version policy| Cross-layer matrix details
"grammar/compatibility/migrations.md"| Migration procedures| Grammar ownership
"grammar/compatibility/deprecated.md"| Deprecation lifecycle| Migration implementation
"grammar/compatibility/reserved.md"| Reserved identifiers/syntax| General version policy
"grammar/compatibility/compatibility-matrix.md"| Cross-layer compatibility relationships| Grammar rules
"grammar/validation/"| Validation rules| Language semantics
"src/lexer.rs"| Lexer implementation| Language specification
"src/parser.rs"| Parser implementation| Language specification
"src/frontend/ast/"| Frontend structural representation| Target realization
semantic analysis| Meaning/type/effect/resource validation| Surface grammar
"src/quantum/ir/"| Canonical quantum semantic IR| Surface syntax
QEC subsystem| Error-correction policy/implementation| Surface grammar
ZQN subsystem| Fault/noise semantics| Surface grammar
routing| Physical/logical mapping| Source grammar
scheduling| Execution/implementation scheduling| Source grammar
HAL| Target/device realization| Language semantics
runtime| Execution| Source syntax

---

3. Fundamental Compatibility Invariant

The primary invariant is:

«A compatible implementation MUST NOT silently change the specified meaning of valid Zamani source code.»

A change MAY be breaking when explicitly classified and versioned.

A breaking change MUST:

1. be identified;
2. be classified;
3. have an owning specification;
4. have a version boundary;
5. have diagnostics;
6. have tests;
7. have migration guidance where practical;
8. be reflected in the compatibility matrix.

No implementation convenience may silently override the language contract.

---

4. Compatibility Dimensions

Compatibility is multidimensional.

A feature MUST NOT be described simply as "compatible" without identifying which dimension is meant.

Dimension| Question
Lexical| Does the same source tokenize the same way?
Syntactic| Does the source parse under the same rules?
AST| Does the source produce an equivalent frontend structure?
Name| Do identifiers/modules resolve equivalently?
Type| Do types retain equivalent meaning?
Effect| Do effects retain equivalent meaning?
Resource| Do resource requirements retain equivalent meaning?
Capability| Do capability requirements retain equivalent meaning?
Semantic| Does execution meaning remain equivalent?
Determinism| Does deterministic source produce equivalent deterministic results?
IR| Does the canonical representation preserve the same semantics?
Artifact| Can produced artifacts be consumed under the declared contract?
ABI| Are foreign/binary boundaries compatible where explicitly promised?
Runtime| Can the runtime execute the artifact without semantic change?
Target| Can the target satisfy the program requirements?
Tooling| Do supported tools understand the same language contract?
Documentation| Does documentation accurately describe the supported language?
Dialect| Does an extension retain its declared versioned meaning?

A feature can therefore be:

source-compatible
but
IR-incompatible

or:

source-compatible
but
target-infeasible

or:

syntax-compatible
but
semantic-incompatible

These cases MUST NOT be conflated.

---

5. Compatibility Status Versus Feature Lifecycle

Compatibility status and feature lifecycle are different concepts.

A feature MAY be:

- experimental and source-compatible;
- experimental and breaking;
- stable and source-compatible;
- deprecated but still source-compatible;
- removed and intentionally incompatible.

The feature lifecycle remains owned by the existing compatibility/deprecation policy:

PROPOSED
    ↓
DESIGNED
    ↓
EXPERIMENTAL
    ↓
IMPLEMENTED
    ↓
STABLE
    ↓
DEPRECATED
    ↓
REMOVED

This matrix records the compatibility consequences of those states.

It does not replace "deprecated.md".

---

6. Compatibility Levels

The repository MAY classify a construct using the following implementation-independent levels:

Level| Meaning
Internal| Not part of the public language contract
Experimental| Public but unstable
Provisional| Defined enough for controlled use but not permanently stable
Stable| Public stable contract
Long-term stable| Foundational contract intended for long-term preservation

A construct's lifecycle state MUST NOT be inferred merely from its existence in "Zamani.g4".

---

7. Canonical Repository Compatibility Matrix

Layer| Primary owner| Backward compatibility| Forward compatibility| Breaking-change trigger
Language specification| "grammar/specification/"| Required for stable features| Explicitly versioned| Meaning changes
Lexical rules| "grammar/lexer/"| Required for stable tokens| Unknown future syntax rejected unless extension is defined| Token meaning changes
Root grammar| "grammar/Zamani.g4"| Required for stable syntax| Version/feature-gated| Parse meaning changes
Domain grammar| Domain directory| Required for stable constructs| Explicit extension mechanism| Domain syntax changes
AST| "src/frontend/ast/"| Required for promised schema| Versioned extension| Structural meaning changes
Type system| "grammar/spec/type-system.md" + implementation| Required| New types additive where possible| Type meaning changes
Effects| "grammar/spec/effects.md"| Required| New effects versioned| Effect semantics change
Resources| "grammar/spec/resources.md" + resource subsystem| Semantic compatibility| New resource/capability forms additive| Requirement meaning changes
Capabilities| capability contracts| Semantic compatibility| New capabilities additive| Existing capability meaning changes
Classical semantics| classical semantic layer| Required| Extensible| Existing computation meaning changes
Quantum semantics| "quantum::ir"| Required| Versioned extension| Quantum meaning changes
QEC| QEC subsystem| Semantic contract| New strategies/codes additive| Existing correction semantics change
ZQN| ZQN subsystem| Semantic contract| New fault/noise models additive| Existing fault semantics change
HDL| "grammar/hdl/" + hardware semantics| Required| Extension/dialect mechanism| Hardware semantics change
Hardware capabilities| hardware subsystem| Requirement semantics stable| Capability discovery| Existing capability meaning changes
Routing| routing subsystem| Logical semantics preserved| New physical realization| Logical meaning changes
Scheduling| scheduling subsystem| Semantic ordering preserved| New scheduling strategy| Ordering semantics change
Compiler| compiler subsystem| Language semantics preserved| New backends additive| Compiler changes source meaning
Runtime| runtime subsystem| Artifact semantics preserved| New targets additive| Runtime changes execution semantics
Dialects| "grammar/dialects/"| Versioned per dialect| Explicit extension| Dialect contract changes
Interoperability| "grammar/interoperability/"| Translation contract| New formats additive| Translation semantics change
Tooling| tooling contracts| Supported-version compatibility| Explicit support declaration| Tool silently changes interpretation
Documentation| normative/derived docs| Must remain truthful| New material additive| Documentation contradicts authority

---

8. Language Version Compatibility

Language versioning is owned by:

grammar/compatibility/versions.md
grammar/specification/language-version.md

This file consumes that policy.

Zamani language versions MUST be independent from:

- Rust versions;
- compiler implementation versions;
- runtime versions;
- operating-system versions;
- CPU generations;
- GPU generations;
- FPGA generations;
- ASIC generations;
- QPU generations;
- target firmware;
- hardware topology.

A new target MUST NOT require a language major-version change merely because the target is new.

---

9. Language Version Matrix

Change| Patch| Minor| Major
Documentation correction| Yes| No| No
Diagnostic improvement preserving semantics| Yes| No| No
Parser bug fix preserving specified semantics| Yes| No| No
New additive capability| No| Yes| No
New optional syntax| No| Yes| No
New dialect| No| Yes/versioned| No
New target backend| No| Usually no language change| No
New stable type| No| Yes| No
Removal of stable syntax| No| No| Yes
Meaning change of stable syntax| No| No| Yes
Stable precedence change| No| No| Yes
Stable ownership semantics change| No| No| Yes
Stable quantum semantic change| No| No| Yes
Stable effect semantic change| No| No| Yes
Stable resource meaning change| No| No| Yes
Stable module resolution change| No| No| Yes

---

10. Compiler-Version Compatibility

Compiler implementation version and language version are independent.

A compiler MAY support multiple language versions:

Compiler C
 ├── Zamani 1.x
 ├── Zamani 2.x
 └── selected dialect versions

The compiler MUST:

- identify supported language versions;
- reject unsupported versions deterministically;
- never silently reinterpret an unsupported version as another version;
- preserve declared semantics.

Compiler upgrades MUST NOT silently redefine language semantics.

---

11. Rust Compatibility

The production implementation baseline is:

Rust 1.97
Rust 1.97.1
Rust edition 2021

The repository MUST treat the supported Rust baseline as an implementation constraint, not a Zamani language-version identifier.

Rust implementation code MUST NOT use:

unsafe

or require unsafe Rust for:

- parsing;
- lexing;
- AST construction;
- semantic analysis;
- compatibility validation;
- IR construction;
- migration;
- diagnostics;
- grammar tooling.

Generated parser code and dependencies MUST also be reviewed so that the repository does not accidentally introduce an unsafe implementation requirement.

The absence of Rust "unsafe" MUST NOT be confused with the existence of a Zamani-language file named:

grammar/statements/unsafe.g4

If that file remains part of Zamani syntax, it represents a language construct, not permission to use Rust "unsafe".

Its semantic status MUST be explicitly defined by the language specification and MUST NOT override the Rust implementation safety policy.

---

12. Lexical Compatibility

Lexical compatibility includes:

- token spelling;
- identifier rules;
- keyword rules;
- contextual keywords;
- numeric literals;
- string literals;
- character literals;
- Unicode;
- comments;
- operators;
- delimiters;
- quantum literals;
- interpolation.

The canonical lexical contract belongs under:

grammar/lexer/

The Rust implementation belongs to the repository frontend.

The two MUST remain conformant.

---

13. Keyword Compatibility

Before introducing a new reserved keyword, the implementation MUST evaluate:

1. existing identifiers;
2. contextual-keyword alternatives;
3. parser ambiguity;
4. dialect collisions;
5. source migration;
6. diagnostics;
7. compatibility tests.

A previously legal identifier becoming a reserved keyword is source-breaking unless a defined escape/contextual mechanism preserves compatibility.

---

14. Token Compatibility

The repository must not accidentally maintain multiple semantic token concepts.

Known areas requiring explicit audit include:

Question / QuestionMark
Ampersand / BitAnd

The correct outcome MUST be determined by lexical meaning rather than by whichever file was edited first.

Token aliases MAY exist for compatibility, but aliases MUST have:

- explicit owner;
- canonical token identity;
- migration status;
- parser behavior;
- tests.

---

15. Operator Compatibility

Operator compatibility covers:

- spelling;
- token identity;
- precedence;
- associativity;
- arity;
- AST shape;
- semantic meaning.

A precedence or associativity change is potentially breaking even when source tokens remain unchanged.

Such a change MUST include:

- parser tests;
- AST tests;
- semantic tests;
- compatibility tests;
- migration documentation where required.

---

16. AST Compatibility

The frontend AST is the structural contract between parsing and semantic analysis.

Every stable grammar construct MUST have a predetermined AST representation.

The required chain is:

Grammar
   ↓
Parse tree
   ↓
Frontend AST
   ↓
Semantic model
   ↓
Canonical IR

A grammar feature MUST NOT be considered complete merely because ANTLR accepts it.

AST compatibility includes:

- node identity;
- field identity;
- field meaning;
- field optionality;
- ordering;
- source spans;
- attributes;
- modifiers;
- semantic payload;
- version metadata where applicable.

---

17. Domain-Neutral Frontend AST

The frontend AST MUST remain domain-neutral where required by the architecture.

Hardware-specific realization objects MUST NOT leak into the frontend merely because source syntax mentions hardware.

Quantum source constructs MUST NOT require physical device objects in the frontend AST.

HDL source constructs MUST NOT require actual FPGA/ASIC objects.

Distributed source constructs MUST NOT require actual cluster nodes.

Target realization occurs later.

---

18. Semantic Compatibility

Semantic compatibility is stronger than syntactic compatibility.

Changes requiring semantic compatibility review include:

- name resolution;
- type inference;
- ownership;
- borrowing;
- effects;
- resource requirements;
- capability requirements;
- evaluation order;
- concurrency;
- determinism;
- numerical semantics;
- quantum measurement;
- quantum state semantics;
- module resolution;
- security semantics;
- hardware semantics.

A syntax-preserving semantic change may therefore require a major language-version transition.

---

19. Resource and Capability Compatibility

Zamani MUST distinguish:

Requirement
Constraint
Capability
Preference
Hint
Implementation decision

For example:

requires qubits >= n

is a resource requirement.

requires capability("quantum.measurement")

is a capability requirement.

prefer accelerator("quantum")

is a preference.

A physical mapping such as:

logical q0 -> physical resource X

is a target-realization decision.

These categories MUST NOT be collapsed.

---

20. No Universal Hardware-Capacity Limits

The language MUST NOT establish arbitrary universal limits such as:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_TIMELINES
MAX_CLUSTER_SIZE

These names may legitimately appear in negative examples, audits, or validation documentation when the text is explicitly describing what is prohibited.

Their presence as documentation does not itself constitute a language limit.

Their presence as a grammar rule, implementation ceiling, or semantic restriction requires correction unless the value is a genuine representation invariant or explicitly scoped implementation/resource policy.

---

21. Hard-Coding Classification

Every discovered fixed quantity MUST be classified as one of:

Classification| Allowed?
Program-level constant| Yes
Mathematical constant| Yes
Explicit user resource requirement| Yes
Target configuration| Yes
Deployment configuration| Yes
Test fixture| Yes
Serialization/representation invariant| Only when technically necessary and documented
Runtime resource limit| Yes when implementation-scoped and not presented as language capacity
Language-wide machine capacity| No
Hidden compiler ceiling| No
Hidden parser capacity presented as language semantics| No

Example:

let n = 1024;
allocate qubits[n];

is program semantics.

A compiler rule such as:

maximum supported qubits = 1024

is not a POCO-REAF language rule.

---

22. Repository Hard-Coding Audit

The compatibility audit MUST inspect at minimum:

grammar/**/*.g4
grammar/**/*.md
src/**/*.rs
tests/**
examples/**
generated artifacts
configuration files
build scripts

The audit MUST detect both explicit and disguised capacity limits.

Examples include:

MAX_*
fixed array sizes
fixed resource counts
fixed device IDs
fixed physical qubit ranges
fixed register widths
fixed topology sizes
fixed node counts
fixed tensor ranks
fixed pipeline depths
fixed timeline counts

The validator MUST distinguish prohibited language ceilings from legitimate examples or tests that intentionally exercise a particular finite target.

---

23. Scalability Compatibility

Zamani scalability means:

«The language architecture MUST NOT impose arbitrary domain ceilings merely because a current implementation or target has finite resources.»

Actual execution remains bounded by:

- the mathematical/semantic requirements of the program;
- representational requirements;
- declared constraints;
- compiler implementation resources;
- runtime resources;
- target capabilities.

Therefore:

language expressiveness
        ≠
implementation capacity
        ≠
test capacity
        ≠
target capacity

A test environment supporting a finite quantity MUST NOT establish that quantity as a language limit.

---

24. POCO-REAF Compatibility

POCO-REAF requires separation between:

WHAT

and:

HOW
WHERE
ON WHICH DEVICE
WITH WHICH PHYSICAL RESOURCE

The preferred flow is:

Zamani source
     ↓
portable semantic meaning
     ↓
requirements + capabilities + constraints + preferences
     ↓
canonical IR
     ↓
optimization/lowering
     ↓
resource discovery
     ↓
routing
     ↓
scheduling
     ↓
target realization
     ↓
runtime

A source program SHOULD remain usable across:

- tiny systems;
- embedded systems;
- CPUs;
- multicore CPUs;
- manycore systems;
- GPUs;
- FPGAs;
- ASICs;
- accelerators;
- quantum processors;
- simulators;
- hybrid systems;
- clusters;
- HPC systems;
- cloud environments;
- distributed systems;
- future computational substrates;

provided the target satisfies the program's semantic requirements.

---

25. Quantum Compatibility

Quantum syntax MUST remain target-independent unless physical realization is explicitly requested.

Supported semantic categories MAY include:

- qubits;
- logical qubits;
- registers;
- states;
- operations;
- parameterized operations;
- controls;
- adjoints;
- measurement;
- reset;
- observables;
- channels;
- noise;
- dynamic circuits;
- classical feed-forward;
- error-correction intent;
- resilience intent;
- resource requirements.

The grammar MUST NOT establish a fixed universal gate inventory as the semantic universe.

The preferred model is:

quantumOperation
    ↓
operation specification
    +
target list
    +
parameters
    +
modifiers

rather than a permanent enumeration of every gate known today.

This allows constructs such as:

apply H
apply custom_gate
apply vendor.operation
apply operation(parameter)

to share one semantic operation model.

---

26. "quantum::ir" Canonical Boundary

The repository's canonical quantum semantic boundary remains:

quantum::ir

The compatibility architecture MUST NOT introduce a competing frontend quantum IR.

Required flow:

Quantum source
      ↓
Quantum grammar
      ↓
Frontend AST
      ↓
Semantic validation
      ↓
quantum::ir
      ↓
optimization
      ↓
routing
      ↓
scheduling
      ↓
QEC / resilience / ZQN
      ↓
HAL
      ↓
target

Changes to quantum grammar MUST be reviewed against the actual repository "src/quantum/ir/" contracts.

The compatibility matrix therefore protects:

- qubit identity;
- operation identity;
- measurement semantics;
- classical control;
- resources;
- capabilities;
- mapping;
- validation;
- serialization;
- hashing;
- analysis;
- pulse semantics where applicable.

---

27. Quantum Resource Compatibility

The following MUST remain distinct:

logical qubit
physical qubit
qubit requirement
qubit capability
physical mapping
QEC overhead
target capacity

For example:

requires qubits >= n

does not itself determine:

physical qubit IDs
physical topology
device vendor
calibration
QEC code
routing strategy

Those decisions belong downstream.

---

28. QEC Compatibility

The grammar MAY express QEC intent.

It MUST NOT become the QEC implementation.

The repository's canonical QEC resource-policy ownership MUST remain authoritative.

Where the existing implementation defines "QecLimits", the compatibility architecture MUST NOT create another independent "QecLimits" type or policy authority.

Required relationship:

Zamani source
      ↓
semantic QEC requirement
      ↓
QEC subsystem
      ↓
canonical QecLimits/resource policy
      ↓
QEC analysis/correction

A compatibility change affecting QEC MUST review:

- resource requirements;
- QEC configuration;
- QEC limits/policy;
- logical/physical resource relationships;
- diagnostics;
- serialization;
- IR compatibility;
- runtime behavior.

---

29. ZQN Compatibility

ZQN remains a downstream fault/noise semantic layer.

The grammar MAY express:

- fault-model intent;
- resilience requirements;
- noise-awareness;
- reliability requirements;
- verification intent;
- acceptable error characteristics.

The grammar MUST NOT hard-code a physical noise universe.

A new ZQN model is compatible when it extends the defined semantic model without changing existing stable fault meanings.

---

30. Routing Compatibility

Routing is a physical realization concern.

Logical semantics MUST remain independent of physical topology unless topology itself is explicit program semantics.

The grammar MUST NOT silently assume:

- line topology;
- ring topology;
- fixed mesh;
- fixed qubit adjacency;
- fixed node adjacency;
- fixed network size.

A target may have any topology capable of satisfying the program's requirements.

---

31. Scheduling Compatibility

Scheduling is downstream from semantic intent.

The source may express:

- ordering;
- dependency;
- concurrency;
- latency requirement;
- timing requirement;
- synchronization;
- preference.

The grammar MUST NOT impose:

- fixed scheduler lanes;
- fixed device count;
- fixed execution slots;
- fixed thread count;
- fixed qubit count;
- fixed pipeline count.

Scheduling changes MUST preserve declared semantic ordering and constraints.

---

32. HDL Compatibility

HDL compatibility MUST distinguish:

- hardware behavior;
- hardware structure;
- timing;
- interfaces;
- parameters;
- resources;
- verification;
- synthesis intent;
- physical intent.

The grammar MUST NOT turn today's FPGA/ASIC characteristics into permanent language ceilings.

Examples such as:

32-bit register
64-stage pipeline
8 devices
128 ports

may be explicit program or target requirements.

They MUST NOT become universal parser limits.

---

33. Hardware Compatibility

Hardware descriptions SHOULD express:

capabilities
requirements
interfaces
resources
topology
timing
power
thermal constraints
reliability
deployment intent

rather than assuming a fixed machine universe.

Vendor-specific features MUST be isolated through explicit:

dialects
interoperability
target descriptions
backend contracts

and MUST NOT silently redefine core Zamani semantics.

---

34. Classical Compatibility

Classical computing remains part of the same language.

Compatibility covers:

- scalar computation;
- integer arithmetic;
- floating-point computation;
- vectors;
- matrices;
- tensors;
- symbolic computation;
- numerical computation;
- statistics;
- signal processing;
- optimization;
- linear algebra;
- control;
- scientific computing.

Mathematical library operations SHOULD generally be semantic operations or library/intrinsic capabilities rather than an ever-growing list of grammar keywords.

---

35. Hybrid Compatibility

Hybrid programs MUST preserve the boundary between:

classical semantics
quantum semantics
hardware semantics

without creating separate languages.

A hybrid program may therefore express:

classical computation
       ↓
quantum operation
       ↓
measurement
       ↓
classical decision
       ↓
quantum operation

The compatibility chain remains:

frontend AST
      ↓
semantic model
      ↓
quantum::ir + classical semantics
      ↓
hybrid lowering

No second hybrid quantum IR is permitted.

---

36. Distributed Compatibility

Distributed syntax MUST remain independent of actual machine count.

The language may express:

- logical processes;
- actors;
- services;
- messages;
- channels;
- replication;
- partitioning;
- consistency;
- transactions;
- collective operations;
- placement intent;
- fault tolerance.

It MUST NOT establish an arbitrary maximum node count.

A source program requiring a resource quantity larger than the target provides is a target/resource failure, not a syntax incompatibility.

---

37. AI Compatibility

AI language constructs MUST preserve semantic categories such as:

- models;
- tensors;
- datasets;
- training;
- inference;
- optimization;
- differentiability;
- probabilistic computation;
- symbolic reasoning;
- agents;
- pipelines;
- deployment.

Framework-specific behavior MUST NOT silently become core Zamani semantics.

A new AI backend MUST be additive where possible.

---

38. Data Compatibility

Data constructs MUST preserve:

- type;
- shape;
- schema;
- ordering where significant;
- nullability;
- provenance where specified;
- serialization semantics;
- transformation semantics.

Data movement between CPU, GPU, FPGA, QPU, distributed storage, or network environments MUST NOT change the source-level data meaning.

---

39. Networking Compatibility

Networking constructs MUST distinguish:

logical endpoint
protocol
communication semantics
physical address
physical route

A network implementation may change while preserving protocol semantics.

A physical address MUST NOT become a universal source-level requirement unless addressing is itself part of the program semantics.

---

40. Security Compatibility

Security semantics MUST preserve:

- identity;
- authorization;
- capabilities;
- policy;
- isolation;
- trust;
- cryptographic meaning;
- provenance;
- secure-computation intent.

Adding a stronger implementation MUST NOT silently weaken a stable security contract.

Vendor-specific security mechanisms belong downstream or in explicit dialects/interoperability layers.

---

41. Memory Compatibility

Memory syntax MUST distinguish:

semantic memory
ownership
borrowing
allocation
region
address space
persistence
shared memory
distributed memory
accelerator memory
quantum resource semantics

The grammar MUST NOT assume a fixed:

- RAM size;
- VRAM size;
- register width;
- address-space capacity;
- memory-bank count.

---

42. Concurrency Compatibility

Concurrency constructs MUST describe semantic concurrency.

Examples:

parallel
async
await
spawn
task
actor
channel
pipeline
data-parallel
task-parallel

A semantic "parallel" construct MUST NOT silently mean a fixed number of hardware threads.

Thread/core count is a resource realization property unless explicitly expressed as a program requirement.

---

43. Effects Compatibility

Effects are semantic contracts.

Compatibility MUST preserve:

- effect identity;
- operation meaning;
- handler behavior;
- effect composition;
- effect polymorphism.

Effect implementation limits MUST NOT be represented as universal language capacity limits.

---

44. Module Compatibility

Module compatibility covers:

- module identity;
- import paths;
- exports;
- visibility;
- aliases;
- package identity;
- dependencies;
- version constraints.

A module path change is breaking when existing source resolution changes.

Dependency resolution MUST remain deterministic under the declared compatibility policy.

---

45. Interoperability Compatibility

External representations such as:

- OpenQASM;
- QIR;
- HDL formats;
- LLVM-related formats;
- MLIR-related formats;
- C/C++;
- Python;
- Rust;
- WebAssembly;

are interoperability boundaries.

They MUST NOT silently become competing Zamani semantic authorities.

Required flow:

external format
      ↓
interoperability adapter
      ↓
Zamani semantic model
      ↓
canonical Zamani IR

not:

external format
      ↓
new competing Zamani IR

---

46. Dialect Compatibility

Every dialect MUST identify:

name
version
owner
syntax extensions
semantic extensions
AST mapping
IR mapping
capabilities
feature gates
compatibility rules
migration rules

A dialect MUST NOT silently modify core Zamani semantics.

Dialect changes MUST be classified independently from core-language changes.

---

47. Macro Compatibility

Macros MUST NOT bypass compatibility validation.

Macro expansion MUST remain subject to:

- version rules;
- lexical rules;
- syntax rules;
- AST validation;
- semantic validation;
- diagnostics.

A macro that expands differently across compiler versions in a semantic-breaking manner MUST be versioned or rejected.

---

48. Metaprogramming Compatibility

Compile-time and reflective constructs MUST preserve declared semantics.

Metaprogramming MUST NOT provide an unrestricted mechanism for bypassing:

- type checking;
- capability checking;
- resource validation;
- security boundaries;
- compatibility rules.

Generated source must be checked against the effective language version.

---

49. Diagnostics Compatibility

Diagnostics are part of the developer-facing compatibility contract.

The following properties SHOULD remain stable:

diagnostic code
severity
source span
category
structured metadata
migration guidance where applicable

Human-readable diagnostic wording MAY evolve.

A compatibility-breaking diagnostic MUST NOT hide a source semantic change.

---

50. Source Spans

Stable constructs MUST preserve useful source provenance.

Where a grammar or AST change affects source spans, the change MUST be tested.

Source-span compatibility is important for:

- diagnostics;
- IDE tooling;
- migrations;
- formatting;
- debugging;
- provenance;
- semantic analysis.

---

51. Determinism

Given identical:

source
language version
dialect set
grammar version
toolchain contract

the lexer/parser MUST produce equivalent results.

Deterministic parsing means:

source
  ↓
tokens
  ↓
parse tree
  ↓
AST

must not depend on:

- random state;
- hardware discovery;
- target order;
- nondeterministic iteration;
- runtime state.

Runtime nondeterminism is a separate semantic concern.

---

52. Round-Trip Compatibility

Where a canonical serializer/printer exists:

source
  ↓
AST
  ↓
canonical representation
  ↓
AST

must preserve defined semantics.

Where IR serialization exists:

semantic model
  ↓
IR
  ↓
serialize
  ↓
deserialize

must preserve the declared IR contract.

Version changes MUST be explicit.

---

53. Generated Artifact Compatibility

Generated parser/lexer artifacts are derived artifacts.

They MUST NOT become authority.

Required relationship:

authoritative grammar
      ↓
generation
      ↓
generated artifacts
      ↓
tests

Manual modification of generated artifacts without corresponding source changes MUST be detectable.

Generated artifacts MUST be reproducible.

---

54. "grammar.md" Compatibility

The current repository uses:

grammar/grammar.md

as an implementation/conformance reference derived from the actual frontend.

It MUST NOT become a competing normative grammar.

Its status MUST distinguish at least:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

Where "grammar.md" and "Zamani.g4" disagree, the repository MUST record the discrepancy and resolve it through the authority hierarchy.

---

55. "Zamani-Grammar.md" Compatibility

"grammar/Zamani-Grammar.md" remains valuable historical/extended design material.

It MUST distinguish:

stable
proposed
experimental
deprecated
historical
not implemented

Presence in "Zamani-Grammar.md" does not automatically make syntax legal.

Promotion remains:

design proposal
      ↓
semantic design
      ↓
AST contract
      ↓
canonical grammar
      ↓
implementation
      ↓
IR contract
      ↓
tests
      ↓
stable

---

56. "Zamani.g4" Compatibility

"grammar/Zamani.g4" remains the canonical ANTLR composition root.

It MUST:

- compose modular grammar components;
- preserve universal entry points;
- dispatch declarations/statements/expressions/types;
- provide EOF;
- use the canonical token vocabulary;
- avoid becoming a second semantic implementation.

It SHOULD NOT become a monolithic list of every hardware, quantum, AI, mathematical, or vendor operation.

Domain-specific constructs belong in their existing domain directories and are composed through the root.

---

57. Existing File Preservation

Existing major files MUST NOT be renamed merely to fit this compatibility architecture.

In particular:

grammar/Zamani.g4
grammar/Zamani-Grammar.md
grammar/grammar.md
grammar/DESIGN.md
grammar/compatibility/versions.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md
grammar/compatibility/reserved.md
grammar/compatibility/compatibility-matrix.md

retain their established roles.

New files/directories MAY be added where necessary.

Existing domain directories SHOULD be populated rather than duplicated under a new hierarchy.

---

58. Compatibility Ownership of Existing Compatibility Files

File| Compatibility responsibility
"compatibility/versions.md"| Version policy and release classification
"compatibility/migrations.md"| Source migration procedures
"compatibility/deprecated.md"| Deprecation lifecycle
"compatibility/reserved.md"| Reserved syntax/identifier policy
"compatibility/compatibility-matrix.md"| Cross-layer compatibility relationships
"spec/compatibility.md"| Formal semantic compatibility concepts
"grammar/grammar.md"| Implementation conformance
"specification/grammar-authority.md"| Grammar authority
"specification/language-version.md"| Language-version semantics

No file should silently absorb another file's authority.

---

59. Compatibility Change Procedure

Every public grammar change MUST follow:

1. Identify feature
        ↓
2. Identify owning specification
        ↓
3. Identify lexical impact
        ↓
4. Identify grammar impact
        ↓
5. Identify AST impact
        ↓
6. Identify semantic impact
        ↓
7. Identify resource/capability impact
        ↓
8. Identify IR impact
        ↓
9. Identify compiler impact
        ↓
10. Identify runtime/target impact
        ↓
11. Identify tooling impact
        ↓
12. Classify compatibility
        ↓
13. Update tests
        ↓
14. Update migration/deprecation data if needed
        ↓
15. Update this matrix
        ↓
16. Validate repository-wide

No public feature is complete merely because its ".g4" file parses.

---

60. Feature Completion Contract

Every production grammar feature MUST be traceable through:

Feature ID
    ↓
Specification
    ↓
Lexer
    ↓
Grammar
    ↓
AST
    ↓
Semantic model
    ↓
IR
    ↓
Compiler
    ↓
Runtime/target
    ↓
Tooling
    ↓
Tests
    ↓
Compatibility

The owning feature file MUST identify these relationships before it is declared complete.

This prevents later files from forcing completed files to be redesigned solely because integration contracts were omitted.

---

61. Cross-File Integration Matrix

Area| Must integrate with
Lexical contracts| "grammar/lexer/", "src/lexer.rs"
Root grammar| "grammar/Zamani.g4"
Syntax specification| "grammar/specification/", "grammar/spec/"
Parser| "src/parser.rs"
AST| "src/frontend/ast/"
Classical| "grammar/classical/" + semantic/compiler layers
Quantum| "grammar/quantum/" + "src/quantum/ir/"
QEC| quantum semantic model + QEC subsystem
ZQN| quantum/resilience/fault semantics
HDL| "grammar/hdl/"
Hardware| "grammar/hardware/"
Resources| "grammar/resources/"
Effects| "grammar/effects/"
Memory| "grammar/memory/"
Concurrency| "grammar/concurrency/"
Distributed| "grammar/distributed/"
AI| "grammar/ai/"
Data| "grammar/data/"
Networking| "grammar/networking/"
Security| "grammar/security/"
Compilation| "grammar/compile/"
Execution| "grammar/execution/"
Interoperability| "grammar/interoperability/"
Dialects| "grammar/dialects/"
Macros| "grammar/macros/"
Metaprogramming| "grammar/metaprogramming/"
Versioning| "compatibility/versions.md"
Migration| "compatibility/migrations.md"
Deprecation| "compatibility/deprecated.md"
Reserved syntax| "compatibility/reserved.md"
Compatibility matrix| this file
Validation| "grammar/validation/"
Conformance| "grammar/tests/"

---

62. Dependency Direction

The compatibility architecture MUST preserve this dependency direction:

lexical contracts
      ↓
core grammar
      ↓
expressions/types/declarations/statements
      ↓
domain grammar
      ↓
Zamani.g4
      ↓
lexer/parser
      ↓
frontend AST
      ↓
semantic analysis
      ↓
canonical semantic representation
      ↓
canonical IR
      ↓
optimization/lowering
      ↓
routing/scheduling/resilience
      ↓
HAL
      ↓
runtime/target

The grammar MUST NOT depend on runtime implementation.

The grammar MUST NOT depend on physical hardware discovery.

The grammar MUST NOT depend on target-specific routing.

The grammar MUST NOT depend on QEC implementation.

The grammar MUST NOT depend on ZQN implementation.

---

63. Resource Information Flow

Resource information may flow into compilation as an input:

source requirement
       ↓
semantic requirement
       ↓
capability/resource discovery
       ↓
candidate targets
       ↓
optimization
       ↓
routing
       ↓
scheduling
       ↓
execution

This does NOT mean:

runtime
   ↓
grammar

is a dependency.

Runtime observations inform realization, not source-language ownership.

---

64. Target Compatibility

A target is compatible when it can satisfy the source program's declared semantics.

A target MAY fail because it lacks:

- required capability;
- required memory;
- required quantum resources;
- required precision;
- required timing;
- required interconnect;
- required reliability;
- required protocol;
- required accelerator capability.

Such failure MUST be reported as a target/resource/capability failure.

The compiler MUST NOT silently change program semantics to make an incompatible target appear compatible.

---

65. Approximation Compatibility

Approximation is compatible only when explicitly permitted by source semantics.

For example:

prefer error <= bound

may permit implementation variation within that declared contract.

An exact operation MUST NOT silently become approximate.

A new approximation strategy requires semantic compatibility review.

---

66. Hardware Scaling Matrix

Source intent| Tiny target| CPU| GPU| FPGA| ASIC| QPU| Cluster| Future target
Classical computation| Realize if capable| Realize| Realize| Realize| Realize| Through hybrid boundary where applicable| Realize| Capability-dependent
Parallel computation| Limited by resources| Realize| Realize| Realize| Realize| Domain-dependent| Realize| Capability-dependent
Tensor computation| Realize if capable| Realize| Realize| Realize| Realize| Domain-dependent| Realize| Capability-dependent
Quantum computation| Simulator/compatible device| Simulator/device| Simulator/accelerator| Hybrid| Hybrid| Native QPU| Distributed quantum/hybrid| Capability-dependent
HDL intent| Limited| Co-design| Accelerator| Native| Native| Hybrid| Distributed hardware| Capability-dependent
Distributed computation| Limited| Single-node semantics| Multi-device| Deployment-dependent| Deployment-dependent| Hybrid| Native| Capability-dependent

The table describes realization possibilities, not mandatory target support.

---

67. Domain Compatibility Matrix

Domain| Source-level owner| Canonical downstream boundary| Target realization
Classical| "grammar/classical/"| Classical semantic/IR layer| CPU/GPU/FPGA/ASIC/etc.
Quantum| "grammar/quantum/"| "quantum::ir"| QPU/simulator/hybrid
Hybrid| "grammar/hybrid/"| Combined semantic representation| Heterogeneous target
HDL| "grammar/hdl/"| Hardware semantic/IR layer| FPGA/ASIC/accelerator
Hardware| "grammar/hardware/"| Hardware semantic model| Physical target
Distributed| "grammar/distributed/"| Distributed semantic model| Cluster/cloud/network
AI| "grammar/ai/"| AI/model semantic layer| CPU/GPU/accelerator/QPU
Data| "grammar/data/"| Data semantic layer| Local/distributed/cloud
Networking| "grammar/networking/"| Communication semantic layer| Network implementation
Security| "grammar/security/"| Security semantic layer| Platform/security backend
Nano/future domains| Explicit domain owner| Domain semantic layer| Capability-dependent

---

68. Quantum + HDL Compatibility

Hybrid quantum hardware MUST preserve:

HDL intent
   ↓
hardware semantic model
   ↓
hybrid semantic model
   ↓
quantum::ir
   ↓
optimization
   ↓
routing
   ↓
scheduling
   ↓
QEC/resilience
   ↓
ZQN
   ↓
HAL

HDL MUST NOT create another quantum IR.

Quantum grammar MUST NOT duplicate HDL implementation semantics.

---

69. QEC + ZQN Compatibility

The separation MUST remain:

quantum semantics
       ↓
quantum::ir
       ↓
QEC/resilience
       +
ZQN/fault/noise semantics
       ↓
physical realization

QEC owns correction strategy.

ZQN owns fault/noise semantics.

Routing owns physical mapping.

Scheduling owns ordering/time/resource scheduling.

HAL owns target interaction.

The compatibility matrix MUST preserve these ownership boundaries.

---

70. Compatibility of Physical IDs

Physical IDs MAY exist in explicit target/deployment contexts.

They MUST NOT become implicit universal source semantics.

Examples of target-specific constructs include:

physical_qubit(...)
target_device(...)
deployment_target(...)

when explicitly scoped to target realization.

They MUST NOT redefine portable logical constructs.

---

71. Compatibility of Resource Quantities

Explicit quantities remain legal when they are program semantics.

Examples:

requires qubits >= n
requires memory >= required_memory
requires bandwidth >= required_bandwidth
requires latency <= required_latency

These express requirements.

They do not establish compiler maximums.

---

72. Representation Limits

Zamani cannot claim that every implementation can represent literally infinite data.

Compatibility therefore means:

«No arbitrary language-level domain ceiling may be imposed merely because a particular implementation or present-day target is finite.»

A genuine representation limit MUST:

1. be technically necessary;
2. be documented;
3. be implementation-scoped where possible;
4. not masquerade as a language semantic limit;
5. produce a deterministic diagnostic;
6. not silently change program meaning.

---

73. Security Compatibility

Compatibility validation MUST consider:

- parser denial-of-service;
- pathological nesting;
- excessive macro expansion;
- compile-time computation exhaustion;
- resource-exhaustion attacks;
- malformed external formats;
- capability bypasses;
- foreign-function boundary changes;
- generated-code safety;
- dialect injection;
- ambiguous grammar behavior.

Resource controls are implementation security controls.

They MUST NOT silently become language semantics.

---

74. Safe Rust Compatibility

Production Zamani Rust code MUST remain safe Rust.

The compatibility matrix requires:

Rust 1.97 / Rust 1.97.1
Rust 2021
no unsafe Rust

This applies to Zamani-owned implementation code.

Dependencies MUST be reviewed according to repository security policy.

The language keyword "unsafe", if retained as a Zamani source construct, MUST be specified independently from Rust implementation safety.

If Zamani ultimately prohibits unsafe source semantics as well, that decision belongs to the language specification and MUST be reflected through a separate compatibility change rather than being silently inferred from the Rust safety rule.

---

75. Tooling Compatibility

Language tooling MUST consume the same authoritative version/grammar information.

Affected tools include:

- parser tooling;
- language servers;
- formatters;
- linters;
- documentation generators;
- migration tools;
- build tools;
- IDE integrations;
- test harnesses.

Tooling MUST NOT maintain an undocumented private grammar fork.

A tool supporting only a subset MUST identify:

supported language version
supported dialects
supported feature set

---

76. Example Compatibility

Canonical examples MUST identify, where applicable:

- language version;
- dialect;
- expected parse;
- expected semantic behavior;
- required capabilities;
- required resources;
- target assumptions;
- compatibility status.

Examples used only for a specific physical target MUST clearly identify that scope.

A target-specific example MUST NOT become evidence of a universal language limit.

---

77. Negative Compatibility Tests

The compatibility suite MUST include:

- invalid language versions;
- unsupported dialect versions;
- removed syntax;
- deprecated syntax;
- keyword collisions;
- malformed operators;
- invalid type constructs;
- invalid resource requirements;
- invalid capability expressions;
- unsupported quantum operations;
- malformed quantum targets;
- invalid hybrid constructs;
- invalid HDL constructs;
- incompatible IR versions;
- invalid external formats;
- invalid feature combinations.

Failures MUST be deterministic and machine-readable.

---

78. Positive Compatibility Tests

The suite MUST verify that stable constructs remain valid.

Coverage MUST include:

minimal.zm
classical.zm
generic.zm
quantum.zm
hybrid.zm
hdl.zm
poco-reaf.zm

and all applicable repository examples.

Each stable example MUST be associated with its declared compatibility contract.

---

79. Boundary Tests

Boundary tests MUST distinguish:

language invalidity
        ≠
implementation resource exhaustion
        ≠
target resource exhaustion

Tests SHOULD cover:

- smallest valid values;
- symbolic values;
- large values;
- parameterized values;
- empty values where legal;
- zero values where illegal;
- deeply nested programs;
- large module graphs;
- large tensors;
- large quantum resource requirements;
- large HDL structures;
- large distributed descriptions.

No boundary test may establish an arbitrary universal maximum.

---

80. Scalability Tests

Scalability tests MUST vary quantities such as:

- module count;
- declaration count;
- expression size;
- tensor dimensions;
- qubit requirements;
- operation count;
- HDL modules;
- ports;
- signals;
- nets;
- memories;
- pipeline stages;
- distributed processes;
- services;
- messages;
- AI tensors/models;
- network endpoints;
- timelines where supported.

Tests MUST verify that increasing a semantic quantity does not require a new language construct.

---

81. Determinism Tests

Repeated compilation of the same:

source
language version
dialect set
toolchain contract

MUST produce equivalent:

tokens
parse tree
AST
semantic representation
IR

where deterministic behavior is promised.

Target scheduling MAY vary when target resources differ, provided source semantics remain preserved.

---

82. Migration Compatibility

Migration ownership remains:

grammar/compatibility/migrations.md

The matrix records whether migration is required.

A migration MUST preserve semantics.

Required conceptual flow:

old source
    ↓
version-aware parser
    ↓
migration transformation
    ↓
new source
    ↓
new semantic model

A migration that only makes syntax compile while changing meaning is invalid.

---

83. Deprecation Compatibility

Deprecation ownership remains:

grammar/compatibility/deprecated.md

A deprecated feature MUST identify:

- deprecated version;
- current status;
- replacement;
- migration path;
- removal policy;
- compatibility consequences.

The matrix MUST record whether use remains source-compatible.

---

84. Reserved Syntax Compatibility

Reserved syntax ownership remains:

grammar/compatibility/reserved.md

A reserved identifier MUST NOT be promoted to stable syntax without:

- specification;
- lexical analysis;
- parser analysis;
- AST contract;
- semantic contract;
- compatibility review;
- tests.

Reserved words MUST NOT be allocated merely for speculative convenience.

---

85. Forward Compatibility

Future syntax MUST NOT be silently interpreted as existing semantics.

The rule is:

unknown metadata
    → may be preserved/ignored where explicitly permitted

unknown semantic construct
    → explicit error

known extension namespace
    → extension-specific interpretation

A compiler MUST NOT accept future syntax merely because it can parse it.

---

86. Compatibility of Unknown Attributes

Unknown attributes MAY be preserved where the attribute contract explicitly permits forward-compatible metadata.

However, an unknown attribute MUST NOT silently alter core semantics.

Attributes affecting:

- type;
- ownership;
- effects;
- resources;
- capabilities;
- quantum semantics;
- security;
- hardware semantics;

require explicit semantic ownership.

---

87. Artifact Compatibility

Artifacts SHOULD carry sufficient metadata to identify, where applicable:

language version
grammar/schema version
AST schema version
IR version
dialect versions
required capabilities
resource requirements
target constraints
compiler compatibility

An incompatible artifact MUST be rejected deterministically.

Runtime MUST NOT silently execute an artifact under incompatible semantics.

---

88. IR Compatibility

IR compatibility MUST be evaluated independently from source compatibility.

A source program may remain stable while the compiler changes its internal IR.

Therefore:

source compatibility
      ≠
internal IR compatibility

An IR becomes a public compatibility boundary only when explicitly designated as such.

Where an IR is public, it requires:

- schema version;
- migration policy;
- serialization policy;
- compatibility tests;
- semantic invariants.

---

89. "quantum::ir" IR Compatibility

"quantum::ir" is a canonical semantic boundary and therefore receives special protection.

Any change affecting it MUST identify:

- affected quantum nodes/types;
- qubit identity;
- operation identity;
- measurement;
- classical control;
- resources;
- capabilities;
- mapping;
- pulse semantics where applicable;
- serialization;
- validation;
- downstream consumers.

No frontend grammar file may introduce an alternative canonical quantum IR.

---

90. Classical/HDL IR Compatibility

Classical and HDL domain representations MAY evolve independently when they remain behind stable semantic contracts.

A domain IR MUST NOT leak target-specific assumptions backward into the source grammar.

Target-specific IR transformations belong downstream.

---

91. Compiler Compatibility

Compiler transformations MUST preserve semantic invariants.

Optimization MAY change:

- instruction order where permitted;
- representation;
- layout;
- implementation strategy;
- resource allocation;
- scheduling;
- routing.

Optimization MUST NOT change declared program meaning.

---

92. Runtime Compatibility

Runtime changes MUST preserve the semantics of compatible artifacts.

Runtime MAY change:

- resource allocation;
- scheduling strategy;
- caching;
- placement;
- device selection;
- execution strategy;

when these remain within the source semantic contract.

Runtime MUST NOT silently reinterpret source-level semantics.

---

93. Target Compatibility

Target compatibility is conditional:

program requirements
        ↓
target capabilities
        ↓
resource feasibility

A target is not "language-incompatible" merely because it lacks resources.

It is target-infeasible for that program.

This distinction is essential for POCO-REAF.

---

94. Compatibility and Future Computing

A future target SHOULD require only:

capability declaration
semantic lowering
backend realization

rather than a new core language.

Examples include future:

- processors;
- accelerators;
- quantum architectures;
- neuromorphic systems;
- photonic systems;
- molecular/nano systems;
- heterogeneous systems;
- distributed computational substrates.

The compatibility architecture must preserve extension without redesigning the core language for every new machine class.

---

95. Compatibility of Nano/Future Domains

If nano/future computing remains a first-class Zamani domain, its source constructs MUST integrate through:

domain grammar
      ↓
domain-neutral AST
      ↓
semantic model
      ↓
domain IR/canonical boundary
      ↓
target realization

The grammar MUST NOT encode a fixed physical universe.

---

96. Sankofa/Temporal Compatibility

Sankofa-related concepts such as:

- history;
- recall;
- learning;
- temporal state;
- provenance;
- consensus;
- memory;
- timelines;

MUST remain semantic constructs.

They MUST NOT impose fixed:

- history depth;
- timeline count;
- branch count;
- timestamp capacity;
- memory capacity.

If multi-timeline syntax is introduced, compatibility MUST be defined independently of any implementation's finite number of timelines.

---

97. Compatibility of Feature Manifests

Where the repository adopts feature manifests, each manifest SHOULD identify:

feature ID
name
status
language version
grammar location
lexer tokens
AST nodes
semantic rules
IR mapping
compiler consumers
runtime consumers
capabilities
resource requirements
positive tests
negative tests
boundary tests
scalability tests
compatibility policy
migration policy

The manifest does not replace this matrix.

It supplies feature-level evidence consumed by this matrix.

---

98. Compatibility Test Taxonomy

Every stable public feature SHOULD have:

positive/
negative/
boundary/
scalability/
determinism/
round-trip/
compatibility/
migration/
cross-domain/
security/

tests where applicable.

Quantum features additionally require:

logical-resource
physical-resource
dynamic-control
measurement
custom-operation
routing
QEC
ZQN

coverage where applicable.

---

99. Cross-Domain Compatibility Tests

The test suite MUST include combinations such as:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware
AI + quantum + distributed
data + networking + security

The purpose is to verify that domains compose rather than become separate languages.

---

100. Repository-Wide Compatibility Validation

A production compatibility release MUST validate:

specification
      ↓
grammar
      ↓
lexer
      ↓
parser
      ↓
AST
      ↓
semantic analysis
      ↓
IR
      ↓
compiler
      ↓
runtime
      ↓
tests

The repository MUST detect mismatches such as:

specified but not parsed
parsed but no AST
AST but no semantics
semantics but no IR
IR but no compiler consumer
compiler support but no tests
documentation claiming stable support when implementation is partial

---

101. Compatibility Status Vocabulary

Repository documentation MUST use these terms consistently:

Status| Meaning
"SPECIFIED"| Normatively defined
"IMPLEMENTED"| Implemented and validated
"PARTIALLY IMPLEMENTED"| Some contract exists but implementation is incomplete
"EXPERIMENTAL"| Public but unstable
"PROVISIONAL"| Controlled unstable feature
"PLANNED"| Intended but not implemented
"DEPRECATED"| Still recognized but scheduled for migration/removal
"REMOVED"| No longer accepted under that language version
"HISTORICAL"| Retained for design/history only
"NOT IMPLEMENTED"| Described but not available

No documentation may call a feature "stable" when the implementation is only "PLANNED" or "PARTIALLY IMPLEMENTED".

---

102. Compatibility of Documentation

Documentation MUST declare its role:

normative
derived
explanatory
experimental
historical
deprecated

The following must never happen:

historical design
       ↓
accidentally treated as
       ↓
stable syntax

Examples should be parser-tested wherever practical.

---

103. Compatibility of Generated Documentation

Generated documentation MUST identify its source.

For example:

authoritative specification
        ↓
generated reference

Generated output MUST NOT become the source of truth.

---

104. Compatibility and Repository Refactoring

Internal directory organization MAY evolve without changing language compatibility.

A file move is compatibility-neutral only when:

- public ownership remains unchanged;
- references are updated;
- generated paths are updated;
- tooling is updated;
- no public import/path contract changes;
- tests remain valid.

Existing filenames MUST NOT be renamed merely for stylistic consistency.

---

105. Compatibility and File Independence

Every production grammar file SHOULD be completable independently with its integration contract already specified.

Before marking a file complete, its contract must identify:

Purpose
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream contracts
Downstream consumers
Grammar contract
AST contract
Semantic contract
IR integration
Compiler integration
Runtime integration
Tooling integration
Cross-domain integration
Diagnostics
Versioning
Compatibility
Positive tests
Negative tests
Boundary tests
Scalability tests
Determinism tests
Hard-coding audit
Security/resource analysis
Completion criteria

This matrix is the cross-file compatibility consumer of those declarations.

---

106. Completion Rule

A compatibility-sensitive file is complete only when:

1. its authority is known;
2. its ownership is known;
3. its non-ownership is known;
4. all downstream consumers are identified;
5. compatibility impact is classified;
6. version impact is classified;
7. AST impact is known;
8. semantic impact is known;
9. IR impact is known;
10. compiler impact is known;
11. runtime impact is known;
12. tests exist;
13. scalability is considered;
14. hard-coding is audited;
15. diagnostics are defined;
16. migration/deprecation impact is known.

Later implementation of another file MUST NOT require architectural redesign merely to discover these relationships.

---

107. Compatibility CI

Production CI MUST validate, as applicable:

cargo fmt --check
cargo check
cargo test
cargo test --all-targets
ANTLR grammar generation
grammar validation
lexer conformance
parser conformance
AST conformance
semantic conformance
IR conformance
compatibility tests
negative tests
boundary tests
scalability tests
determinism tests
round-trip tests
hard-coding audit
documentation consistency

The supported implementation baseline is:

Rust 1.97 / Rust 1.97.1
Rust 2021
safe Rust only

No Rust "unsafe" may be introduced to satisfy compatibility requirements.

---

108. Compatibility Release Checklist

Before a production grammar release:

- [ ] Normative authority hierarchy is unchanged or explicitly updated.
- [ ] Language version is identified.
- [ ] Grammar version is identified where applicable.
- [ ] Lexer impact is reviewed.
- [ ] Parser impact is reviewed.
- [ ] AST impact is reviewed.
- [ ] Semantic impact is reviewed.
- [ ] Type impact is reviewed.
- [ ] Effect impact is reviewed.
- [ ] Resource impact is reviewed.
- [ ] Capability impact is reviewed.
- [ ] Classical impact is reviewed.
- [ ] Quantum impact is reviewed.
- [ ] "quantum::ir" compatibility is reviewed.
- [ ] QEC impact is reviewed.
- [ ] ZQN impact is reviewed.
- [ ] HDL impact is reviewed.
- [ ] Hardware impact is reviewed.
- [ ] Routing impact is reviewed.
- [ ] Scheduling impact is reviewed.
- [ ] Runtime impact is reviewed.
- [ ] Tooling impact is reviewed.
- [ ] Dialect impact is reviewed.
- [ ] Interoperability impact is reviewed.
- [ ] Deprecation impact is reviewed.
- [ ] Migration impact is reviewed.
- [ ] Reserved syntax impact is reviewed.
- [ ] Positive tests pass.
- [ ] Negative tests pass.
- [ ] Boundary tests pass.
- [ ] Scalability tests pass.
- [ ] Determinism tests pass.
- [ ] Round-trip tests pass where applicable.
- [ ] Cross-domain tests pass.
- [ ] Hard-coding audit passes.
- [ ] Rust 1.97 / 1.97.1 compatibility passes.
- [ ] No Rust "unsafe" is introduced.
- [ ] Generated artifacts are reproducible.
- [ ] No competing grammar authority was introduced.
- [ ] No competing quantum IR was introduced.
- [ ] Existing filenames were not unnecessarily renamed.
- [ ] "grammar.md" status is synchronized.
- [ ] "Zamani-Grammar.md" status is not accidentally promoted.
- [ ] Compatibility matrix is updated.

---

109. Production Compatibility Invariants

The following are permanent architecture invariants unless deliberately changed through an explicit major language/architecture decision.

Invariant 1 — One language authority

There MUST be one normative language specification.

Invariant 2 — One canonical grammar composition root

grammar/Zamani.g4

remains the canonical ANTLR composition root.

Invariant 3 — One frontend AST contract

Domain-specific syntax MUST map into the established frontend AST architecture.

Invariant 4 — One canonical quantum semantic boundary

quantum::ir

remains canonical.

Invariant 5 — No hardware-derived language ceilings

Current target capacity MUST NOT become a universal language limit.

Invariant 6 — Requirements are not realization

A resource requirement MUST NOT silently become a physical mapping.

Invariant 7 — Capabilities are not devices

A capability MUST NOT silently identify a vendor/device.

Invariant 8 — QEC remains downstream

Grammar expresses QEC intent; QEC implementation remains downstream.

Invariant 9 — ZQN remains downstream

Grammar expresses relevant fault/noise intent; ZQN remains downstream.

Invariant 10 — Routing remains downstream

Logical semantics are not physical placement.

Invariant 11 — Scheduling remains downstream

Semantic concurrency/order is not a fixed hardware schedule.

Invariant 12 — Runtime does not define syntax

Runtime capability cannot silently redefine source grammar.

Invariant 13 — Documentation does not override specification

Historical/proposed documentation cannot silently create stable syntax.

Invariant 14 — Rust implementation is safe

Zamani-owned Rust implementation does not use "unsafe".

Invariant 15 — Compatibility is explicit

No breaking change is introduced silently.

---

110. Final POCO-REAF Compatibility Model

The production architecture is:

                         Zamani Source
                               │
                               ▼
                    Language Version Resolution
                               │
                               ▼
                       Canonical Lexer
                               │
                               ▼
                    grammar/Zamani.g4
                               │
                               ▼
                       Canonical Parser
                               │
                               ▼
                    Frontend Domain-Neutral AST
                               │
                               ▼
                       Semantic Analysis
                               │
             ┌─────────────────┼─────────────────┐
             │                 │                 │
             ▼                 ▼                 ▼
           Types           Resources        Capabilities
             │                 │                 │
             └─────────────────┼─────────────────┘
                               │
                               ▼
                    Portable Semantic Model
                               │
             ┌─────────────────┼─────────────────┐
             │                 │                 │
             ▼                 ▼                 ▼
        Classical         quantum::ir          HDL/
        semantics         canonical          hardware
             │                 │              semantics
             └─────────────────┼─────────────────┘
                               │
                               ▼
                       Optimization/Lowering
                               │
               ┌───────────────┼────────────────┐
               │               │                │
               ▼               ▼                ▼
            Routing         Scheduling       Resilience
                                                │
                                      ┌─────────┼─────────┐
                                      ▼         ▼         ▼
                                     QEC       ZQN       other
                                      │         │
                                      └────┬────┘
                                           ▼
                                          HAL
                                           │
                                           ▼
                                  Target Realization
                                           │
             ┌─────────────┬───────────────┼──────────────┐
             ▼             ▼               ▼              ▼
            CPU           GPU             FPGA           ASIC
             │             │               │              │
             └─────────────┴──────┬────────┴──────────────┘
                                  │
                                  ▼
                                QPU
                                  │
                                  ▼
                           Distributed/HPC
                                  │
                                  ▼
                         Future substrates

The source program defines meaning.

The semantic model defines requirements and guarantees.

The compiler determines implementation strategy.

The runtime determines execution strategy.

The target provides actual resources and capabilities.

The compatibility system guarantees that those layers do not silently redefine one another.

---

111. Final Compatibility Principle

The fundamental Zamani compatibility rule is:

«SOURCE SEMANTICS MUST NOT BECOME CURRENT-MACHINE SEMANTICS.»

Therefore:

Program
   ↓
Portable Meaning
   ↓
Requirements
   +
Capabilities
   +
Constraints
   +
Preferences
   ↓
Canonical Representation
   ↓
Compilation
   ↓
Optimization
   ↓
Resource Discovery
   ↓
Routing / Scheduling / Resilience
   ↓
Target Realization
   ↓
Execution

A machine becoming:

- larger;
- smaller;
- faster;
- slower;
- multicore;
- manycore;
- GPU-based;
- FPGA-based;
- ASIC-based;
- quantum;
- hybrid;
- distributed;
- embedded;
- cloud-based;
- HPC-based;
- or based on a future computational substrate

MUST NOT by itself require a rewrite of the Zamani program.

The program may become infeasible on a particular target when its explicit requirements cannot be satisfied. That is a resource/capability/target failure, not a reason to redefine the language.

Likewise, a new hardware capability MUST NOT force a new core language syntax when the existing semantic extension mechanisms are sufficient.

---

112. Final Production Contract

"grammar/compatibility/compatibility-matrix.md" is production-ready when it remains the single cross-layer compatibility relationship authority while preserving the existing ownership model:

versions.md
    → version policy

migrations.md
    → migration procedure

deprecated.md
    → deprecation lifecycle

reserved.md
    → reserved syntax

compatibility-matrix.md
    → cross-layer compatibility relationships

specification/
    → normative language meaning

spec/
    → formal contracts

Zamani.g4
    → canonical grammar composition

grammar.md
    → implementation/conformance reference

Zamani-Grammar.md
    → historical/extended/proposed design

src/lexer.rs
    → lexer implementation

src/parser.rs
    → parser implementation

src/frontend/ast/
    → frontend structural representation

semantic analysis
    → meaning

quantum::ir
    → canonical quantum semantic boundary

QEC
    → error-correction implementation/policy

ZQN
    → fault/noise semantics

routing
    → physical mapping

scheduling
    → execution/implementation schedule

HAL
    → target realization

runtime
    → execution

No layer may silently assume ownership belonging to another layer.

No current machine capacity may become a permanent language ceiling.

No target-specific implementation may become a hidden grammar rule.

No external format may become a competing Zamani semantic authority.

No frontend quantum IR may replace "quantum::ir".

No Rust "unsafe" is required or permitted for the Zamani implementation.

And the governing portability invariant remains:

Program Once
      ↓
Compile Once
      ↓
Preserve One Semantic Meaning
      ↓
Adapt Through Capabilities + Resources
      ↓
Run Everywhere
      ↓
Anywhere
      ↓
Forever

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

From Atom to Everywhere.