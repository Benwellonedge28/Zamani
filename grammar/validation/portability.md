Zamani Portability Validation Specification

Path: "grammar/validation/portability.md"
Language: Zamani
Validation domain: Portability, target independence, POCO-REAF, scalability, resource adaptation, realization independence
Status: Normative validation contract / Production
Specification role: Validation and conformance; this file does not define new source syntax
Grammar technology: ANTLR4 + Rust frontend
Rust baseline: Rust 1.97 / Rust 1.97.1, Rust 2021
Rust safety policy: Zamani-owned production Rust MUST use safe Rust; Rust "unsafe" is prohibited
Primary portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)
Scalability objective: From the smallest meaningful computation to arbitrarily large finite computations, bounded by actual semantics, declared requirements, implementation representation, policies, and resources actually available
Canonical grammar root: "grammar/Zamani.g4"
Canonical executable lexer: "src/lexer.rs"
Canonical frontend parser: "src/parser.rs"
Canonical AST boundary: "src/frontend/ast/" and the established domain-neutral AST model
Canonical quantum semantic boundary: "src/quantum/ir/" / "quantum::ir"
Canonical semantic analysis: "src/semantic.rs"
Canonical IR generation: "src/ir_gen.rs"
Canonical IR validation: "src/ir_verify.rs"

---

1. Purpose

This file defines the production validation contract for Zamani portability.

It answers:

«Does the language and compiler preserve program meaning when the realization changes?»

It validates the repository-wide property that a Zamani program can express computation independently of unnecessary physical-machine details and can subsequently be realized on different compatible computational environments.

This includes:

- classical processors;
- multicore processors;
- GPUs;
- FPGAs;
- ASICs;
- QPUs;
- quantum simulators;
- accelerators;
- embedded systems;
- heterogeneous systems;
- clusters;
- distributed systems;
- cloud environments;
- future computational architectures.

Portability validation is therefore not a test that every program can execute on every machine.

It is a test that:

1. target-independent semantics remain target-independent;
2. explicit target requirements remain explicit;
3. resource requirements are not confused with implementation limits;
4. capabilities are not confused with vendors;
5. logical resources are not confused with physical resources;
6. compiler realization does not silently alter source semantics;
7. scaling is not artificially limited by grammar or frontend implementation;
8. portable programs can be adapted to compatible realizations;
9. incompatible targets produce precise diagnostics rather than silent semantic degradation;
10. the complete source → lexer → parser → AST → semantics → IR → backend/runtime chain preserves the portability contract.

---

2. Authority

This file is a validation contract.

It does not replace or compete with the normative semantic specification.

The authority hierarchy is:

grammar/DESIGN.md
        │
        │ architecture
        ▼
grammar/specification/
        │
        │ human-readable normative language specification
        ▼
grammar/spec/
        │
        │ formal semantic contracts
        ▼
grammar/Zamani.g4 + modular grammar files
        │
        │ accepted syntax
        ▼
src/lexer.rs
src/parser.rs
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
        ├── classical semantics
        ├── quantum semantics
        ├── HDL/hardware semantics
        ├── hybrid semantics
        ├── distributed semantics
        └── other domain semantics
        │
        ▼
canonical/domain IR
        │
        ├── quantum::ir
        ├── classical IR
        └── HDL/hardware IR
        │
        ▼
optimization / lowering
        │
        ├── routing
        ├── scheduling
        ├── resilience
        ├── QEC
        └── ZQN
        │
        ▼
HAL / backend
        │
        ▼
runtime / deployment

The roles of the existing grammar documents are:

File| Authority
"grammar/DESIGN.md"| Normative architecture
"grammar/README.md"| Navigation and authority explanation
"grammar/specification/*.md"| Human-readable normative language specification
"grammar/spec/*.md"| Formal semantic/domain contracts
"grammar/Zamani.g4"| Canonical ANTLR composition root
modular "grammar/**/*.g4"| Domain/component grammar contracts
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Historical, proposed, experimental, or extended design material
"grammar/validation/*.md"| Validation contracts
"src/lexer.rs"| Executable lexical implementation
"src/parser.rs"| Executable parser
"src/frontend/ast/"| Canonical frontend AST architecture
"src/semantic.rs"| Executable semantic analysis
"src/ir_gen.rs"| AST/semantic lowering to IR
"src/ir_verify.rs"| IR verification
"src/quantum/ir/"| Canonical quantum semantic/IR boundary

A validation document MUST never promote a "PLANNED", "PROPOSED", "EXPERIMENTAL", or "HISTORICAL" feature into implemented language behavior.

---

3. Scope

This validation contract covers:

- semantic portability;
- source-level target independence;
- realization independence;
- resource portability;
- capability portability;
- scalability;
- logical/physical separation;
- target declarations;
- compilation targets;
- deployment intent;
- execution intent;
- classical portability;
- quantum portability;
- hybrid portability;
- HDL/hardware portability;
- accelerator portability;
- distributed portability;
- networking portability;
- AI/data portability;
- memory portability;
- interoperability;
- dialect boundaries;
- compatibility;
- reproducibility;
- deterministic semantics;
- portability diagnostics;
- source-span preservation;
- hard-coded capacity detection;
- frontend/IR portability;
- compiler/runtime integration.

This file does not own:

- lexical token definitions;
- expression precedence;
- general type semantics;
- resource syntax;
- capability syntax;
- scheduling algorithms;
- routing algorithms;
- QEC algorithms;
- ZQN implementation;
- HAL implementation;
- backend implementation;
- runtime implementation.

Those are validated through their own contracts.

---

4. Core Portability Invariant

For a Zamani program "P", environment "E", and realization strategy "R":

Realize(P, E, R) -> O

where "O" is observable behavior.

Two realizations are portable equivalents when:

Realize(P, E1, R1)
        ≡ semantic
Realize(P, E2, R2)

for every property that the program declares as semantically observable.

The implementation MAY change:

- instruction selection;
- machine instructions;
- parallel decomposition;
- task placement;
- scheduling;
- memory placement;
- communication topology;
- quantum physical mapping;
- gate decomposition;
- HDL synthesis;
- accelerator selection;
- optimization;
- representation;
- execution environment.

The implementation MUST NOT change:

- required results;
- declared type semantics;
- declared effects;
- ownership guarantees;
- resource requirements;
- correctness guarantees;
- explicit ordering requirements;
- observable side effects;
- security requirements;
- explicitly declared numerical guarantees;
- explicitly declared quantum semantics;
- explicitly declared hardware behavior.

---

5. POCO-REAF Validation

The validator MUST treat:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

as an architectural portability objective, not as a guarantee that impossible targets can execute arbitrary programs.

The following distinction is mandatory:

portable source
    ≠
universally executable source

A program requiring:

capability("quantum.measurement")

is portable across compatible quantum-capable environments.

It is not executable on a target that has no mechanism satisfying that capability.

Therefore a validator MUST distinguish:

PORTABLE_AND_SUPPORTED
PORTABLE_BUT_RESOURCE_UNAVAILABLE
PORTABLE_BUT_CAPABILITY_UNAVAILABLE
PORTABLE_BUT_POLICY_BLOCKED
EXPLICITLY_TARGET_CONSTRAINED
TARGET_SPECIFIC
NON_PORTABLE
SEMANTICALLY_INVALID

It MUST NOT classify a program as non-portable merely because the current machine cannot execute it.

---

6. Compile-Once Boundary

Portability validation MUST identify the highest semantic artifact that can be reused across target realizations.

The preferred pipeline is:

source
  ↓
lex
  ↓
parse
  ↓
AST
  ↓
semantic analysis
  ↓
canonical semantic representation
  ↓
portable canonical IR
  ↓
target specialization
  ↓
optimization/lowering
  ↓
routing/scheduling/resilience
  ↓
backend/HAL
  ↓
runtime

The reusable artifact MUST NOT contain accidental physical-machine assumptions.

A target-specialized artifact MAY contain physical information, but that artifact is downstream from the portable semantic representation.

Validation MUST distinguish:

portable artifact

from:

target-realized artifact

---

7. Portability Classification

Every target-related semantic dependency MUST be classifiable as one of:

Classification| Meaning
"portable"| No unnecessary target dependency
"resource-dependent"| Requires an abstract quantity of a resource
"capability-dependent"| Requires an abstract capability
"scale-dependent"| Resource quantity depends on workload/program size
"target-constrained"| Explicitly constrained to a target property/class
"target-specific"| Explicitly tied to a concrete target realization
"implementation-defined"| Chosen by implementation without changing semantics
"policy-dependent"| Depends on deployment or execution policy
"non-portable"| Cannot preserve semantics across the requested boundary
"invalid"| Violates the language semantic contract

A validator MUST never silently collapse these classifications.

---

8. Resource Versus Portability

"grammar/spec/resources.md" owns resource semantics.

This file validates their portability implications.

For example:

requires qubits >= n

means:

«the realization must provide sufficient logical quantum resources for the computation.»

It does not mean:

MAX_QUBITS = n

Similarly:

requires memory >= required_memory

does not establish a universal language memory limit.

Validation MUST ensure that:

program requirement

is never converted into:

compiler maximum

---

9. Capability Versus Vendor

Portable source SHOULD express capabilities rather than vendor identity.

Preferred:

requires capability("quantum.measurement");
requires capability("tensor.compute");
requires capability("distributed.communication");

rather than:

requires vendor_qpu_x;
requires gpu_device_3;
requires node_17;

Vendor-specific syntax is permitted only inside an explicitly target-specific boundary.

The validator MUST flag vendor identity when it escapes into a construct declared portable.

---

10. Requirement / Constraint / Preference / Hint

The validator MUST preserve these distinctions.

Requirement

Mandatory semantic condition.

requires qubits >= logical_qubits;

Constraint

A valid realization must satisfy the condition.

constraint latency <= latency_budget;

Preference

An optimization preference.

preference accelerator = quantum;

Hint

Non-binding guidance.

hint locality;

The validator MUST reject semantic models where:

preference -> requirement
hint -> requirement
implementation decision -> source requirement

occurs without an explicit semantic rule.

---

11. Hard-Coded Capacity Validation

The validator MUST work together with:

grammar/validation/hardcoding-audit.md
grammar/validation/scalability-rules.md

The following universal capacity constants are prohibited:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_ACCELERATORS
MAX_TIMELINES
MAX_PROCESSES
MAX_CHANNELS

The audit MUST also detect disguised equivalents, including:

if qubits > 1024 reject

for q in 0..1024

when used as the universal language representation.

enum PhysicalQubit {
    Q0,
    Q1,
    ...
}

when intended to define all quantum resources.

register_width <= 32

when used as a universal language restriction.

The validator MUST inspect:

- grammar files;
- lexer;
- parser;
- AST;
- semantic analyzer;
- IR generator;
- IR verifier;
- resource analysis;
- target selection;
- runtime-facing validation;
- tests;
- documentation that claims implementation limits.

---

12. Program Constants Are Permitted

The validator MUST NOT reject ordinary program values merely because they are large or finite.

Valid:

let n = 1024;

Valid:

let width = input_width;

Valid:

Tensor<float, 1024, 1024>

Valid:

requires qubits >= workload_size;

Invalid:

compiler_max_qubits = 1024

when this limits the language rather than the selected target.

The validator MUST therefore determine the role of a number rather than blindly reject numeric literals.

---

13. "Infinity" Validation

"Infinity" means:

«no artificial finite language-level capacity ceiling.»

It does not mean:

«physical resources are infinite.»

The validator MUST ensure that finite implementation resources are represented as:

resource availability
resource budget
resource quota
resource policy
target capability
compiler resource exhaustion
runtime resource exhaustion

rather than as permanent language limits.

A valid implementation MAY reject:

requires memory >= 1 TiB

on a target with insufficient memory.

It MUST report:

resource infeasible

rather than:

Zamani language supports at most N bytes

unless such a limit is explicitly part of the language's semantic representation.

---

14. Tiny-to-Large Scaling

The same language semantics MUST support:

single value
→
single operation
→
small program
→
embedded workload
→
single processor
→
multicore
→
accelerator
→
heterogeneous system
→
cluster
→
distributed system
→
large-scale system
→
future architecture

No separate "large-computing language" may be introduced.

Scaling MUST be represented through:

- data size;
- workload size;
- resource requirements;
- concurrency;
- parallelism;
- distribution;
- capability requirements;
- execution policies;
- target capabilities.

---

15. Logical Versus Physical Resources

The validator MUST enforce separation between logical and physical resources.

Examples:

logical_qubit

must not automatically become:

physical_qubit(17)

Likewise:

logical_worker

must not automatically become:

cpu_core(7)

and:

logical_buffer

must not automatically become:

memory_bank(3)

Physical realization belongs downstream.

For quantum computation:

logical quantum program
        ↓
quantum::ir
        ↓
mapping
        ↓
routing
        ↓
scheduling
        ↓
QEC
        ↓
ZQN
        ↓
HAL
        ↓
physical QPU

The portability validator MUST reject any source-to-IR path that requires physical resource identity before the canonical semantic boundary unless the source explicitly declares itself target-specific.

---

16. Quantum Portability

Quantum portability MUST be validated independently of physical QPU topology.

The validator MUST ensure that portable quantum source can express:

- logical qubits;
- symbolic qubit quantities;
- parameterized registers;
- arbitrary operation names;
- namespaced operations;
- custom operations;
- parameterized operations;
- controls;
- adjoints;
- measurement;
- reset;
- dynamic classical control;
- observables;
- channels;
- noise intent;
- resilience intent;
- logical operations.

The validator MUST reject a portable architecture based on a fixed universal gate list such as:

H
X
Y
Z
CNOT
...

when that list becomes the language's complete representation of quantum operations.

The preferred semantic form is:

operation name
+
namespace
+
operands
+
parameters
+
results
+
attributes
+
modifiers
+
effects
+
capabilities

Quantum source MUST lower toward:

quantum::ir

rather than a second competing frontend quantum IR.

---

17. Quantum Gate Portability

A gate name MAY be:

H
custom_gate
vendor.operation
operation(parameter)

provided semantic validation determines its meaning.

Portability validation MUST ensure that:

unknown operation

does not automatically mean:

syntax invalid

when the language architecture supports open-world operation resolution.

Instead it may mean:

semantic operation resolution failure

or:

capability unavailable

or:

dialect unavailable

depending on context.

---

18. Quantum Physical Mapping

Portable source MUST NOT require:

physical_qubit(0)
physical_qubit(1)
...

unless explicitly inside a target-specific realization boundary.

The validator MUST detect physical mappings that leak into:

- portable AST;
- portable semantic representation;
- canonical quantum source semantics.

Physical mapping MUST be permitted downstream in:

routing
scheduling
QEC
HAL
backend
deployment

---

19. Quantum QEC and ZQN

Portability validation MUST ensure that QEC and ZQN do not redefine source-level portability.

Quantum source may express:

requires fault_tolerance;
requires error_correction(...);
requires fidelity >= threshold;
requires capability("quantum.error_correction");

The implementation determines:

- code;
- physical qubits;
- syndrome extraction;
- correction strategy;
- routing;
- scheduling;
- calibration;
- device realization.

QEC MUST remain downstream from canonical quantum semantics.

ZQN MUST remain responsible for its defined fault/noise semantics.

The validator MUST reject duplicated quantum portability models that make:

grammar quantum model

and:

quantum::ir model

compete.

---

20. Classical Portability

Classical portability MUST distinguish semantic computation from instruction-set realization.

Portable source may express:

integer computation
vector computation
matrix computation
tensor computation
parallel computation

The compiler may lower them to:

scalar CPU
SIMD
multicore
GPU
accelerator
distributed execution

The validator MUST reject universal source semantics that depend on:

- a particular CPU register count;
- a particular instruction set;
- a particular cache size;
- a particular core count;
- a particular vector width;

unless explicitly target-specific.

---

21. Memory Portability

Portable memory intent may express:

requires memory >= required_memory;

or abstract memory properties.

It MUST NOT silently require:

RAM = 64 GB
VRAM = 24 GB
register = 32-bit

as universal language assumptions.

Validation MUST distinguish:

memory capacity requirement

from:

physical memory configuration

Memory placement belongs downstream.

---

22. Tensor Portability

Tensor semantics MUST permit dimensions determined by:

- constants;
- parameters;
- input data;
- symbolic expressions;
- runtime information;
- resource negotiation.

The validator MUST reject universal tensor rank/dimension ceilings.

A backend MAY have implementation limits.

Those belong to target feasibility.

---

23. HDL and Hardware Portability

HDL portability MUST validate hardware intent independently of a particular FPGA, ASIC, simulation environment, or synthesis vendor.

Portable HDL may express:

- parameterized widths;
- interfaces;
- pipelines;
- state machines;
- timing requirements;
- memory intent;
- protocols;
- verification properties;
- synthesis intent;
- simulation intent.

The validator MUST reject universal physical assumptions such as:

wire [31:0]

when "32" is being treated as a universal language width rather than a program-defined width.

It MUST also reject:

FPGA_RESOURCE_COUNT = N

as a language-wide maximum.

---

24. Hardware/Software Co-Design

A portable co-designed program may contain:

algorithm
+
parallelism
+
memory intent
+
communication requirements
+
latency constraints
+
accelerator capability
+
hardware implementation intent

The compiler may realize that intent as:

software

accelerator

FPGA

ASIC

hybrid implementation

The validator MUST ensure that target realization does not change the algorithm's required semantics.

---

25. Distributed Portability

Distributed programs MUST NOT encode a universal fixed node count.

Portable source may express:

distributed computation
replication
partitioning
consistency
collective communication
fault tolerance

The realization may use:

2 nodes
10 nodes
1000 nodes

or another available scale.

The validator MUST reject language-level assumptions equivalent to:

MAX_NODES = N

unless that number is explicitly semantic for a particular program.

---

26. Concurrency Portability

Portable concurrency MUST NOT depend on a fixed thread count.

For example:

parallel

is portable.

A program may explicitly request:

requires workers >= n

if that is part of its semantics.

The validator MUST distinguish this from:

compiler supports exactly N workers

The realization may use:

- CPU threads;
- GPU lanes;
- FPGA pipelines;
- distributed tasks;
- accelerator execution;
- another concurrency mechanism.

---

27. Networking Portability

Portable networking MUST separate:

communication semantics

from:

physical network topology

Source may require:

capability("reliable_stream");

or:

requires bandwidth >= required_bandwidth;

It must not silently depend on:

node_17
router_4
link_3

unless explicitly target-specific.

---

28. AI/Data Portability

AI/data programs MUST express:

- model semantics;
- tensors;
- datasets;
- training;
- inference;
- transformations;
- optimization;
- distribution;
- accelerator requirements.

They MUST NOT make a framework or vendor the semantic definition of the program.

For example:

requires capability("tensor.compute");

is portable.

A direct dependency on a particular accelerator API belongs in interoperability or target-specific code.

---

29. Interoperability

Interop is a portability boundary.

The validator MUST classify foreign dependencies as:

portable
conditional
target-constrained
target-specific
non-portable

Examples include:

- C;
- C++;
- Rust;
- Python;
- WebAssembly;
- OpenQASM;
- QIR;
- HDL;
- LLVM/MLIR-related representations;
- vendor APIs.

Foreign representations MUST NOT replace Zamani's canonical semantic model.

OpenQASM and similar quantum formats are interoperability boundaries, not replacements for "quantum::ir".

---

30. Dialect Portability

A dialect MUST explicitly declare:

- dialect identity;
- version;
- syntax extension;
- semantic extension;
- AST mapping;
- IR mapping;
- capability requirements;
- resource requirements;
- compatibility policy;
- portability classification.

A dialect MUST NOT silently redefine the core meaning of portable Zamani.

A dialect marked target-specific MUST remain isolated from portable source semantics.

---

31. Target Declarations

"grammar/compile/target.g4" provides target intent.

Portability validation MUST ensure that target declarations distinguish:

target preference

from:

mandatory target

and:

physical realization

A target preference MUST NOT automatically make the entire program non-portable.

A target-specific block MUST be explicit and bounded.

---

32. Deployment Portability

Deployment declarations MUST be treated as downstream realization intent.

Portable source may specify:

deployment availability
deployment policy
deployment portability
deployment recovery

without embedding a particular physical deployment identity.

Deployment-specific identifiers MUST be classified as target-specific.

---

33. Execution Portability

Execution intent may include:

- scheduling policy;
- resilience;
- checkpointing;
- recovery;
- observability;
- runtime policy.

The validator MUST distinguish:

semantic execution ordering

from:

scheduler implementation order

If ordering is not semantically observable, the compiler may reorder operations.

If ordering is observable, the compiler MUST preserve it.

---

34. Determinism

Portability validation MUST classify nondeterminism explicitly.

A portable program may be nondeterministic by design.

The validator must distinguish:

intentional nondeterminism

from:

accidental target-dependent nondeterminism

A backend MUST NOT introduce observable nondeterminism where the source contract requires determinism.

Where the source permits nondeterminism, all permitted outcomes MUST satisfy the source semantic contract.

---

35. Numerical Portability

Numerical portability MUST account for representation differences.

The validator MUST determine whether a program specifies:

- exact arithmetic;
- bounded integer semantics;
- floating-point semantics;
- approximate numerical semantics;
- tolerance;
- reproducibility requirements.

A compiler MUST NOT silently substitute a different numerical model when doing so changes observable semantics.

If numerical approximation is permitted, it MUST be represented as an explicit semantic allowance.

---

36. Ordering and Memory Model

Portability validation MUST ensure that:

- memory ordering;
- synchronization;
- atomicity;
- visibility;
- race semantics;

are defined independently of the physical processor.

A backend may map the model to different memory architectures.

The mapping MUST preserve the source memory contract.

---

37. Security Portability

Security requirements MUST survive target changes.

For example:

requires capability("secure_execution");

must remain a semantic requirement.

The validator MUST reject a backend that silently drops a security requirement because the selected target lacks the required capability.

A portable source security property may require a target-specific implementation, but that implementation MUST be explicit in the realization layer.

---

38. Power and Thermal Portability

Power and thermal constraints may be:

requirement
constraint
preference
hint

The validator MUST preserve the distinction.

A power preference MUST NOT silently become a semantic requirement.

A hard target power limit MAY reject a realization without making the source language itself non-portable.

---

39. Calibration Portability

Calibration data is target/runtime information.

Portable source may express:

requires fidelity >= threshold

or equivalent semantic requirements.

The actual calibration data belongs downstream.

The validator MUST reject calibration identifiers leaking into portable semantics unless the program explicitly declares a target-specific dependency.

---

40. Topology Portability

Portable source MUST NOT assume:

- fixed QPU connectivity;
- fixed CPU topology;
- fixed GPU topology;
- fixed FPGA routing;
- fixed cluster topology;
- fixed network topology.

Topology MAY be consumed by:

routing
scheduling
placement
deployment
HAL

A program may require a topology property.

For example:

requires topology(...)

This is a requirement.

It is not a hard-coded topology realization.

---

41. Resource Negotiation

Resource negotiation is a downstream adaptation mechanism.

The validator MUST allow a program to state:

requires

and:

prefers

without assuming that the compiler has already acquired the resources.

Negotiation may result in:

satisfied
deferred
adapted
rejected

The program's semantic meaning MUST remain stable.

---

42. Adaptation

A realization MAY adapt:

- parallel decomposition;
- memory placement;
- operation decomposition;
- quantum mapping;
- routing;
- scheduling;
- data partitioning;
- accelerator selection;
- execution placement.

Adaptation MUST be semantics-preserving.

An adaptation MUST NOT:

- silently remove required work;
- silently weaken correctness;
- silently remove a required capability;
- silently change observable results;
- silently convert a requirement into a preference.

---

43. Graceful Scaling

A scalable implementation SHOULD be able to use more available resources when semantics permit.

For example:

parallel computation

may use:

1 worker

or:

many workers

without changing source semantics.

Scaling validation MUST test:

small resource realization

and:

larger resource realization

with the same semantic program.

---

44. Scaling Direction

The validator MUST test both:

scale down

and:

scale up

when the program permits.

Examples:

small tensor
large tensor

small quantum register
larger quantum register

single-node
multi-node

CPU
GPU
accelerator

The source semantics MUST remain unchanged.

---

45. Resource Exhaustion

Resource exhaustion is not automatically a portability violation.

For example:

requires memory >= 1 TiB

on a target with less memory is:

PORTABLE_BUT_RESOURCE_UNAVAILABLE

not:

NON_PORTABLE

unless the source itself explicitly requires that target.

The implementation MUST report the correct category.

---

46. Capability Exhaustion

Likewise:

requires capability("quantum.measurement")

on a classical-only target is:

PORTABLE_BUT_CAPABILITY_UNAVAILABLE

unless the source is explicitly tied to that target.

---

47. Target-Specific Source

Explicit target-specific code is allowed.

It MUST be represented by an explicit portability boundary.

Conceptually:

portable {
    ...
}

target_specific(...) {
    ...
}

The exact syntax is owned by the appropriate grammar/specification files.

This validation file only establishes the semantic requirement:

«target-specific semantics MUST be explicitly identifiable.»

---

48. Leakage Detection

The validator MUST detect leakage from target-specific information into portable semantics.

Examples:

portable program
    ↓
physical GPU ID

portable quantum program
    ↓
physical qubit index

portable distributed program
    ↓
fixed node identity

portable tensor
    ↓
fixed accelerator register width

Such leakage MUST fail validation unless the source explicitly declared the dependency.

---

49. AST Portability

Every stable portability-related syntax construct MUST map to a domain-neutral AST representation.

The validator MUST reject:

grammar construct
    ↓
vendor-specific AST node

when the grammar construct is declared portable.

The preferred model is:

syntax
 ↓
generic AST
 ↓
semantic classification
 ↓
domain semantic representation

The AST MUST preserve enough information for later portability analysis.

At minimum, relevant nodes MUST preserve:

- source span;
- identity/name;
- operands;
- parameters;
- resource expressions;
- capabilities;
- constraints;
- preferences;
- hints;
- modifiers;
- attributes;
- domain information;
- portability classification.

---

50. Source Spans

Every portability-related diagnostic MUST be traceable to source.

Validation MUST integrate with:

grammar/validation/source-spans.md
src/source_map.rs

A portability diagnostic MUST identify:

- file;
- start position;
- end position;
- line;
- column;
- relevant construct;
- portability classification;
- reason;
- remediation information where available.

A backend failure MUST NOT lose the originating source location when the failure corresponds to a source-level portability requirement.

---

51. Lexer Integration

The executable lexical implementation is:

src/lexer.rs

The portability validator MUST NOT assume that a token exists merely because a grammar document names it.

In particular, stale references to nonexistent lexer grammar artifacts MUST be detected.

The current repository contains:

grammar/antlr/

but the current directory is not the executable lexer authority.

The validator MUST therefore distinguish:

ANTLR lexical grammar artifact

from:

Rust executable lexer

and require an explicit, verified relationship between them.

If a grammar file references:

grammar/antlr/ZamaniLexer.g4

while that file is absent from the repository, validation MUST report:

PORTABILITY_VALIDATION_STALE_LEXER_REFERENCE

rather than treating the reference as authoritative.

---

52. Parser Integration

"src/parser.rs" is the executable parser.

The validator MUST verify that portability constructs accepted by the canonical grammar have corresponding parser behavior.

It MUST classify each construct as:

SPECIFIED
LEXER_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
BACKEND_SUPPORTED
TESTED

A construct MUST NOT be classified "production" solely because ANTLR accepts it.

---

53. Semantic Integration

"src/semantic.rs" is a principal semantic consumer.

Portability validation MUST ensure that semantic analysis receives enough information to distinguish:

- requirements;
- constraints;
- capabilities;
- preferences;
- hints;
- target-specific declarations;
- resource availability;
- portability classification.

Semantic analysis MUST NOT make hardware discovery part of parsing.

---

54. IR Integration

"src/ir_gen.rs" MUST preserve portability-relevant semantics when lowering.

The validator MUST inspect IR generation for:

- fixed resource counts;
- physical identifiers;
- vendor-specific assumptions;
- fixed register widths;
- fixed array limits;
- target-dependent defaults;
- silent semantic degradation.

"src/ir_verify.rs" MUST verify that the resulting IR satisfies the semantic portability contract.

---

55. Canonical Quantum IR Integration

The validator MUST enforce:

source quantum semantics
        ↓
canonical quantum::ir

and not:

source
        ↓
frontend quantum IR A
        ↓
frontend quantum IR B
        ↓
quantum::ir

A duplicate quantum frontend IR is a portability architecture violation if it becomes an independent semantic authority.

The canonical quantum semantic boundary remains:

src/quantum/ir/

---

56. Optimization Integration

Optimization MUST preserve portability semantics.

An optimization MAY:

- vectorize;
- parallelize;
- fuse operations;
- decompose operations;
- reorder independent operations;
- select accelerators;
- specialize implementation.

It MUST NOT:

- remove required computation;
- violate ordering;
- exceed declared constraints;
- violate resource requirements;
- change observable semantics.

---

57. Routing Integration

Routing is a realization concern.

Quantum routing may map:

logical qubit

to:

physical qubit

Distributed routing may map:

logical communication

to:

physical network path

Hardware routing may map:

logical datapath

to:

physical interconnect

The validator MUST ensure that these mappings occur downstream from portable semantics.

---

58. Scheduling Integration

Scheduling MAY change execution time and ordering where the semantic model permits it.

The validator MUST ensure:

scheduler choice

does not become:

source semantic requirement

unless explicitly declared.

---

59. Resilience Integration

The existing quantum resilience infrastructure may adapt execution under failure.

Portability validation MUST ensure that:

retry
recover
checkpoint
migrate
degrade
escalate
reject

do not silently change required semantics.

The source-level policy determines what degradation is permitted.

---

60. QEC Integration

QEC is a target realization mechanism.

The validator MUST verify:

logical quantum semantics

are preserved through:

QEC encoding
syndrome processing
correction
decoding

The source language MUST NOT be forced to express physical QEC details unless it explicitly chooses a target-specific boundary.

---

61. ZQN Integration

ZQN may describe or model:

- noise;
- faults;
- reliability;
- uncertainty;
- resilience.

Portability validation MUST ensure that ZQN metadata does not redefine the source language's semantic portability contract.

---

62. HAL Integration

The HAL provides actual target capabilities and state.

Portability validation MUST ensure that:

HAL capability

is consumed as environment information.

It MUST NOT become:

language grammar limit

For example:

HAL says 127 physical qubits

must not cause:

Zamani supports only 127 qubits

to become a language rule.

---

63. Backend Integration

Backend lowering may be target-specific.

The validator MUST require that every backend declare:

- supported capabilities;
- resource limits;
- unsupported semantics;
- adaptation strategies;
- unsupported portability dimensions;
- diagnostics.

A backend MUST fail explicitly when it cannot satisfy a required semantic condition.

It MUST NOT silently approximate unsupported semantics unless approximation is explicitly permitted.

---

64. Runtime Integration

Runtime resource changes MUST NOT alter source semantics unexpectedly.

The runtime may discover:

- device availability;
- load;
- faults;
- thermal state;
- power state;
- network state;
- calibration;
- available memory.

Dynamic adaptation is valid only when permitted by the program's semantic contract.

---

65. Reproducibility

A portable build SHOULD preserve:

- source identity;
- language version;
- compiler version;
- dependency versions;
- compilation configuration;
- semantic artifact identity;
- relevant target profile;
- relevant optimization policy.

Portability validation MUST distinguish:

same semantics

from:

byte-identical machine artifact

Different targets normally produce different binaries.

That does not imply non-portability.

---

66. Build Configuration

Build configuration MUST NOT silently redefine language semantics.

Target-specific compiler flags may affect:

- optimization;
- instruction selection;
- scheduling;
- backend;
- deployment.

They MUST NOT change the meaning of the portable source program.

---

67. Rust Baseline Validation

The repository declares a Rust 1.97 / 1.97.1 baseline.

The validator MUST require the actual Cargo manifest to use valid Cargo syntax.

"rust-version" MUST be exactly one valid bare version.

Valid examples:

rust-version = "1.97"

or:

rust-version = "1.97.1"

The following is invalid:

rust-version = "1.97" or "1.97.1"

The portability validator MUST report a repository/toolchain conformance error when such a manifest is encountered.

The portability specification MUST NOT silently compensate for invalid build metadata.

The repository's chosen policy MUST be made explicit elsewhere:

- minimum supported Rust version;
- exact CI versions;
- compiler compatibility policy.

This file validates consistency with that policy.

---

68. Rust Safety Validation

The Zamani compiler implementation MUST use safe Rust.

Portability validation MUST inspect production Rust for:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe {

and equivalent unsafe constructs.

The validator MUST distinguish:

source-language token named "unsafe"

from:

Rust unsafe implementation

A source-language "unsafe" construct does not automatically imply Rust "unsafe".

Conversely, Rust "unsafe" implementation code is a compiler safety violation regardless of the source language's syntax.

---

69. Dependency Portability

Dependencies used by the compiler MUST be checked against the declared Rust baseline.

A dependency that requires a newer Rust version than the declared baseline is a portability/build compatibility failure.

The validator SHOULD verify:

Cargo.toml
Cargo.lock
dependency MSRV
compiler CI

as a coherent set.

---

70. Feature Flags

Compiler feature flags MUST NOT silently change the language definition.

A feature flag may:

- enable an experimental backend;
- enable an integration;
- enable tooling;
- enable optional runtime support.

If a feature flag changes source syntax or semantics, it MUST be represented as an explicit language/dialect feature with version and compatibility metadata.

---

71. Compatibility

Portability validation integrates with:

grammar/spec/compatibility.md
grammar/compatibility/
grammar/validation/compatibility-rules.md

A language change MUST specify whether it affects:

- source compatibility;
- AST compatibility;
- semantic compatibility;
- IR compatibility;
- backend compatibility;
- runtime compatibility;
- serialized artifact compatibility.

---

72. Version Independence

A future backend MUST be able to consume stable semantic representations according to their declared version.

A new backend MUST NOT require source syntax changes solely because the backend has a different physical architecture.

---

73. Backward Compatibility

A stable portable program MUST remain semantically interpretable across compatible language versions according to the declared compatibility policy.

If a change breaks portability, the compatibility system MUST identify:

breaking

conditionally compatible

or:

fully compatible

rather than silently changing semantics.

---

74. Forward Compatibility

Unknown future target capabilities MUST NOT automatically invalidate portable source.

For example:

future::accelerator

may be represented as an open-world semantic identifier.

The validator MUST distinguish:

unknown capability

from:

invalid syntax

when the language's open-world capability model permits unknown names.

---

75. Open-World Principle

Portability domains SHOULD be extensible.

The validator MUST NOT require a closed enumeration of:

- processors;
- GPUs;
- FPGAs;
- QPUs;
- accelerators;
- future devices;
- network technologies;
- AI architectures;
- quantum operations.

New semantic capabilities may be added through registries/dialects/contracts without requiring universal grammar redesign.

---

76. Closed-World Exceptions

A closed enumeration is acceptable when the enumeration itself is part of language semantics.

Examples might include:

- boolean values;
- language-defined visibility categories;
- language-defined ownership modes.

It is not acceptable merely because the implementation currently knows only a finite set of hardware devices.

---

77. Target Discovery

Target discovery MUST happen after source parsing.

The parser MUST NOT:

- inspect hardware;
- enumerate devices;
- allocate memory;
- select a QPU;
- select a GPU;
- query network topology;
- perform scheduling;
- perform routing.

Portability validation MUST reject parser implementations that perform these actions as part of syntax recognition.

---

78. Resource Discovery

Resource discovery is downstream.

A resource expression such as:

requires memory >= required_memory;

must parse independently of the current machine.

Resource feasibility is determined later.

---

79. Compile-Time Versus Runtime Portability

A requirement MAY be checked:

- statically;
- at compile time;
- at deployment time;
- at runtime;

depending on when its necessary information becomes available.

The validator MUST ensure that deferred checks retain their semantic meaning.

A runtime check MUST NOT silently become a different source-level requirement.

---

80. Conditional Portability

Programs may have conditional realization paths.

For example:

if capability("quantum.measurement") {
    ...
} else {
    ...
}

The validator MUST determine whether both branches satisfy the declared semantic contract.

Capability-dependent behavior is portable when the language explicitly defines the conditional semantics.

---

81. Simulation Portability

A simulator MAY be used when a physical target is unavailable if the program permits simulation.

Simulation MUST NOT silently claim physical execution semantics when those semantics differ.

For example:

physical quantum execution

and:

classical quantum simulation

may have different performance/resource properties while sharing semantic results.

The validator must preserve that distinction.

---

82. Embedded Portability

Embedded targets may have severe physical limitations.

These limitations MUST remain target constraints.

A program may be rejected because an embedded target cannot satisfy its requirements.

That does not establish a language-wide limit.

---

83. Cloud Portability

Cloud-specific deployment information MUST be isolated from portable computation semantics.

Portable source may express:

requires distributed.communication;

Cloud provider identity belongs downstream unless explicitly requested.

---

84. Edge/Cloud Scaling

The same semantic computation may be realized:

locally

or:

edge

or:

cloud

or:

distributed

provided the semantic contract permits the adaptation.

---

85. Energy-Aware Portability

Energy constraints may be represented as:

requirement
constraint
preference

The validator MUST preserve the category.

An energy preference MUST NOT silently invalidate a target.

---

86. Time Portability

Time constraints must distinguish:

semantic deadline

from:

backend scheduling preference

If a program requires completion before a deadline, failure to satisfy that deadline is a constraint failure.

If a program merely prefers low latency, it is an optimization preference.

---

87. Fault Portability

A target may have different fault characteristics.

Portable semantics may require:

requires fault_tolerance;

or equivalent capability/resource intent.

The implementation may choose different resilience mechanisms.

A backend MUST NOT silently remove a required resilience property.

---

88. Graceful Degradation

Graceful degradation is permitted only when explicitly declared.

Valid conceptual outcomes include:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

The validator MUST ensure that "DEGRADED_ACCEPT" is permitted by the source semantics.

Otherwise degradation MUST be a failure.

---

89. Portability Diagnostics

Diagnostics MUST be precise.

Required categories include:

PORTABILITY_ERROR
PORTABILITY_REQUIREMENT_UNSATISFIED
PORTABILITY_CAPABILITY_UNAVAILABLE
PORTABILITY_RESOURCE_UNAVAILABLE
PORTABILITY_POLICY_BLOCKED
PORTABILITY_TARGET_INCOMPATIBLE
PORTABILITY_TARGET_SPECIFIC_LEAK
PORTABILITY_PHYSICAL_BINDING_LEAK
PORTABILITY_HARD_CODED_LIMIT
PORTABILITY_SEMANTIC_DRIFT
PORTABILITY_IR_DRIFT
PORTABILITY_BACKEND_DRIFT
PORTABILITY_STALE_CONTRACT
PORTABILITY_STALE_LEXER_REFERENCE
PORTABILITY_VERSION_MISMATCH
PORTABILITY_UNSAFE_IMPLEMENTATION
PORTABILITY_NONDETERMINISTIC_DRIFT

Diagnostics MUST include source spans where a source construct caused the condition.

---

90. Diagnostic Example

If:

requires qubits >= n;

and a selected target has insufficient capacity, the diagnostic SHOULD identify:

resource requirement:
    qubits >= n

target capability:
    insufficient

classification:
    PORTABLE_BUT_RESOURCE_UNAVAILABLE

It MUST NOT report:

Zamani supports at most N qubits

unless that is genuinely a language semantic limit.

---

91. Semantic Drift Detection

A portability validator MUST compare semantic behavior across realizations.

Given:

same source
+
different compatible target

it MUST verify equivalent observable semantics.

Potential drift includes:

- changed numerical result;
- changed ordering;
- lost side effect;
- changed ownership behavior;
- lost synchronization;
- lost quantum measurement semantics;
- lost hardware verification semantics;
- changed error behavior.

---

92. IR Drift Detection

The validator MUST ensure that target-specific lowering does not modify the canonical semantic contract.

The IR may differ structurally.

It must remain semantically equivalent.

Therefore validation MUST compare:

semantic properties

rather than requiring:

identical IR bytes

across all targets.

---

93. Backend Drift Detection

Each backend MUST document where it cannot preserve a semantic feature.

A backend MUST:

support

adapt

or:

reject

a required feature.

It MUST NOT silently ignore it.

---

94. Test Categories

Portability validation MUST include:

positive
negative
boundary
scalability
determinism
compatibility
resource
capability
target-independence
target-specific
cross-domain
IR-preservation
backend
runtime
diagnostic

tests.

---

95. Minimum Portability Test Matrix

The repository's portability tests SHOULD cover:

Dimension| Small| Medium| Large| Resource-limited| Alternate target
Classical| ✓| ✓| ✓| ✓| ✓
Quantum| ✓| ✓| ✓| ✓| ✓
Hybrid| ✓| ✓| ✓| ✓| ✓
HDL| ✓| ✓| ✓| ✓| ✓
Distributed| ✓| ✓| ✓| ✓| ✓
AI/data| ✓| ✓| ✓| ✓| ✓
Networking| ✓| ✓| ✓| ✓| ✓
Memory| ✓| ✓| ✓| ✓| ✓
Accelerator| ✓| ✓| ✓| ✓| ✓

No test suite may establish an artificial maximum simply because the test uses a particular size.

---

96. Positive Tests

Positive tests MUST demonstrate:

same program
+
different realization
=
same required semantics

Examples:

CPU realization
GPU realization

1-worker realization
many-worker realization

logical qubit mapping A
logical qubit mapping B

FPGA realization
ASIC realization

---

97. Negative Tests

Negative tests MUST include:

- missing required capability;
- insufficient resource;
- incompatible target;
- explicit target-specific construct used outside allowed boundary;
- physical binding leaked into portable semantics;
- hard-coded universal capacity;
- unsupported semantic adaptation;
- silent preference-to-requirement conversion;
- semantic drift;
- stale contract;
- stale lexer reference.

---

98. Boundary Tests

Boundary tests MUST cover:

- smallest valid resource quantity;
- zero where zero is semantically valid;
- empty collections;
- one element;
- symbolic quantities;
- large representable quantities;
- dynamic quantities;
- nested portability contracts;
- mixed domains;
- target-specific boundary crossing.

The tests MUST not define the boundary as a universal physical maximum.

---

99. Scalability Tests

Scalability tests MUST validate growth without changing language semantics.

Required patterns include:

n

where "n" is:

small
medium
large
symbolic
runtime-derived

The validator MUST test that the implementation does not contain accidental fixed thresholds.

---

100. Fuzzing

Portability validation SHOULD use property-based/fuzz testing for:

- resource quantities;
- capability identifiers;
- target descriptors;
- portability contracts;
- deeply nested constructs;
- large symbolic expressions;
- arbitrary qualified names;
- unknown future capabilities.

The primary property is:

valid portable intent
→
does not acquire accidental target dependence

---

101. Deep Input Testing

Deeply nested portability structures MUST not cause unbounded parser recursion when an iterative implementation is possible.

Examples include:

portability {
    ...
    portability {
        ...
    }
}

and deeply nested expressions.

The implementation MUST distinguish:

resource exhaustion

from:

language semantic maximum

---

102. Memory-Safety Validation

Portability validation MUST ensure that the compiler's handling of large portable programs does not introduce Rust "unsafe".

Large workloads may cause:

- allocation failure;
- compiler resource exhaustion;
- timeout;
- backend resource failure.

These are implementation/resource outcomes.

They MUST NOT become artificial language ceilings.

---

103. Source File Portability

Source file identity and spans MUST remain stable enough for diagnostics after:

- macro expansion;
- module resolution;
- lowering;
- optimization;
- target specialization.

The portability validator MUST integrate with "src/source_map.rs".

---

104. Macro Portability

Macros MUST NOT silently introduce target-specific semantics into portable code.

Macro expansion MUST preserve portability classification.

If a macro expands into target-specific constructs, the resulting semantic representation MUST reflect that fact.

---

105. Metaprogramming Portability

Compile-time code generation MUST be validated for target dependence.

A metaprogram may inspect:

target capabilities

only where the language explicitly permits target-aware specialization.

Generated source MUST retain an explicit portability classification.

---

106. Reflection Portability

Reflection over semantic program structure MUST not expose implementation details as if they were stable language semantics.

Physical target information must be classified separately from source semantics.

---

107. Temporal/Sankofa Portability

Temporal concepts such as:

- history;
- recall;
- learning;
- timelines;
- temporal state;

MUST remain semantic abstractions.

The validator MUST reject fixed universal limits such as:

MAX_TIMELINES
MAX_HISTORY_DEPTH
MAX_RECALL_ENTRIES

when those are implementation limits rather than program semantics.

---

108. Nano Portability

Nano/atom/material-oriented computation MUST remain target-independent.

A program may express:

atom
molecule
material
interaction

without embedding a specific physical instrument as the language definition.

The physical realization belongs downstream.

---

109. Future-Domain Portability

A future domain MUST be able to provide:

syntax
AST mapping
semantic model
resource model
capabilities
IR mapping
compiler integration
runtime integration
tests
compatibility

without changing the fundamental portability architecture.

---

110. Feature Manifest Integration

Every stable portability-sensitive feature SHOULD have a feature manifest containing:

feature_id
name
status
version
syntax
lexer_tokens
ast_nodes
semantic_rules
resource_requirements
capabilities
portability_class
ir_mapping
compiler_consumers
runtime_consumers
positive_tests
negative_tests
boundary_tests
scalability_tests
compatibility
hard_coding_policy

The validator MUST be able to trace a feature from specification to implementation.

---

111. Repository Traceability

For every stable portability-sensitive feature:

feature
 ↓
grammar/specification/
 ↓
grammar/spec/
 ↓
grammar/**/*.g4
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
optimizer/lowering
 ↓
backend/HAL
 ↓
runtime
 ↓
tests

A missing link MUST prevent the feature from being marked production-complete.

---

112. Current Repository Integration

The validator MUST integrate with these existing repository artifacts:

grammar/DESIGN.md
grammar/README.md
grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md

grammar/spec/portability.md
grammar/spec/resources.md
grammar/spec/compatibility.md
grammar/spec/semantics.md
grammar/spec/type-system.md

grammar/specification/portability.md
grammar/specification/semantics.md
grammar/specification/types.md

grammar/resources/portability.g4
grammar/resources/requirements.g4
grammar/resources/constraints.g4
grammar/resources/preferences.g4
grammar/resources/hints.g4
grammar/resources/scaling.g4
grammar/resources/negotiation.g4

grammar/compile/target.g4
grammar/execution/deployment.g4
grammar/execution/runtime.g4
grammar/declarations/resources.g4

grammar/validation/hardcoding-audit.md
grammar/validation/scalability-rules.md
grammar/validation/semantic-boundaries.md
grammar/validation/ast-coverage.md
grammar/validation/ir-coverage.md
grammar/validation/compatibility-rules.md
grammar/validation/source-spans.md

src/source_map.rs
src/lexer.rs
src/parser.rs
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs
src/frontend/ast/
src/quantum/ir/
src/quantum/routing/
src/quantum/scheduling/
src/quantum/resilience/
src/quantum/error_correction/
src/quantum/zqn/

The validator MUST NOT assume that every referenced file is already implemented.

Missing files or stale references MUST be reported.

---

113. Existing Lexer Authority Correction

The repository currently contains an inconsistency that portability validation MUST expose:

"grammar/resources/portability.g4" describes "grammar/antlr/ZamaniLexer.g4" as the canonical lexer grammar, while the current "grammar/antlr/" directory does not contain that file.

The executable lexical implementation currently exists at:

src/lexer.rs

Therefore the repository MUST eventually choose and document one of these models:

Model A

ANTLR lexer is authoritative and "src/lexer.rs" is a conforming implementation.

Model B

"src/lexer.rs" is authoritative and ANTLR lexical material is generated/derived.

Model C

Both are generated from one canonical lexical specification.

Until that relationship is explicit and verified, portability conformance MUST be:

PARTIAL

rather than:

PRODUCTION

This validation file MUST NOT invent a missing lexer file.

---

114. Grammar Root Validation

"grammar/Zamani.g4" MUST remain the canonical ANTLR composition root.

Validation MUST ensure:

- no second root grammar claims authority;
- modular grammars are composed consistently;
- imported rule names do not collide unintentionally;
- token vocabulary references resolve;
- root EOF ownership is unambiguous;
- domain grammars do not create incompatible semantic roots.

An empty or unused "grammar/antlr/" directory MUST NOT be treated as an alternative root.

---

115. "grammar.md" Validation

"grammar/grammar.md" MUST report actual implementation status.

For every portability-sensitive construct it MUST be possible to determine:

specified?
lexer?
parser?
AST?
semantic?
IR?
backend?
runtime?
tests?

A syntax appearing in "Zamani.g4" alone MUST NOT be reported as fully implemented.

---

116. "Zamani-Grammar.md" Validation

"Zamani-Grammar.md" may contain aspirational portability concepts.

The validator MUST ensure that such concepts are explicitly classified.

Allowed statuses include:

stable
proposed
experimental
deprecated
historical
not implemented

A historical/experimental feature MUST NOT enter the production portability test suite as stable syntax without promotion.

---

117. Portability and Grammar Syntax

This file does not define portability syntax.

The syntax authority remains:

grammar/resources/portability.g4

and its integration into:

grammar/Zamani.g4

Validation MUST compare actual grammar syntax against:

grammar/spec/portability.md
grammar/specification/portability.md

and report drift.

---

118. No Duplicate Portability Grammar

The validator MUST detect multiple incompatible definitions of:

portability
portabilityContract
portabilityRequirement
portabilityConstraint
portabilityPreference
portabilityHint

If another grammar file defines the same semantic construct, it MUST either:

1. delegate to the canonical portability grammar; or
2. be explicitly identified as a domain-specific extension.

Silent duplication is prohibited.

---

119. Expression Authority

Portability expressions MUST use the canonical expression/resource-expression model.

Portability validation MUST reject a portability grammar that creates an incompatible:

expression
arithmeticExpression
comparisonExpression
logicalExpression

model.

The portability validator validates meaning, not a second expression language.

---

120. Name Authority

Portability domains and capabilities SHOULD use the repository's canonical naming/qualified-name system.

Names such as:

quantum
hardware
classical
future::accelerator

must remain open-world semantic identifiers where appropriate.

The portability validator MUST reject closed enumerations that unnecessarily prevent future domain extension.

---

121. Physical Identity Validation

The following forms MUST be treated as target-specific when they identify concrete physical resources:

physical_cpu(...)
physical_gpu(...)
physical_qpu(...)
physical_qubit(...)
physical_fpga(...)
physical_node(...)
physical_memory_bank(...)

They may be legal in explicit realization/deployment code.

They MUST NOT be silently accepted as portable semantic requirements.

---

122. Resource Quantity Validation

Quantities must be interpreted according to the resource model.

The validator MUST allow quantities to be:

literal
symbolic
parameterized
derived
input-dependent
runtime-derived

It MUST NOT require quantities to be compile-time constants unless another semantic rule explicitly requires that.

---

123. Resource Dimension Validation

Resource dimensions may include:

count
capacity
bandwidth
latency
energy
power
precision
fidelity
reliability
storage
memory
compute
communication

The validator MUST preserve dimensional meaning.

A comparison between incompatible dimensions MUST be a semantic error, not a portability error.

---

124. Capability Namespace Validation

Capability names MUST be:

- stable;
- namespaced where appropriate;
- open to future extension;
- independently resolvable.

The validator SHOULD detect accidental collisions between:

vendor names
capability names
resource names
target names

---

125. Portability Boundary Validation

Every target-specific operation MUST have an identifiable boundary.

The validator SHOULD construct a conceptual graph:

portable nodes
        │
        ▼
target-specific boundary
        │
        ▼
realization nodes

If target-specific information flows backward into portable nodes without explicit permission, validation fails.

---

126. Information-Flow Rule

Portability is not only a syntax property.

It is an information-flow property.

The validator MUST track flows of:

hardware identity
resource identity
topology
calibration
vendor API
device state
physical addresses

and ensure they do not contaminate portable semantics accidentally.

---

127. Capability Discovery Flow

Valid:

source requirement
      ↓
semantic capability requirement
      ↓
target capability discovery
      ↓
feasibility
      ↓
realization

Invalid:

target discovery
      ↓
rewrite source semantics

unless explicit dynamic adaptation semantics permit it.

---

128. Resource Discovery Flow

Valid:

source
 ↓
abstract resource requirement
 ↓
resource discovery
 ↓
allocation/negotiation
 ↓
realization

Invalid:

hardware discovered
 ↓
language syntax changed

---

129. Compiler Choice

The compiler MAY choose:

- CPU;
- GPU;
- FPGA;
- QPU;
- simulator;
- distributed execution;
- accelerator;
- hybrid execution.

The choice MUST preserve semantic requirements.

A compiler choosing a different target does not make the source program non-portable.

---

130. Target Failure

If a target cannot satisfy the program:

do not rewrite source semantics
do not silently remove work
do not silently weaken requirements
do not invent a language maximum

Instead:

diagnose
adapt if explicitly permitted
or reject

---

131. Portability Across Hardware Generations

A program MUST be portable across hardware generations when:

- the target provides required capabilities;
- resource requirements are satisfiable;
- semantic guarantees can be preserved.

Generation-specific optimizations MAY differ.

Source semantics MUST remain stable.

---

132. Portability Across Architectures

The same semantic program may target:

von Neumann CPU
SIMD CPU
GPU
dataflow accelerator
FPGA
ASIC
QPU
distributed architecture
future architecture

The validator MUST focus on semantic preservation rather than instruction-level similarity.

---

133. Portability Across Physical Realizations

Physical realization may change:

placement
routing
memory
scheduling
parallelism
communication
quantum encoding

The validator MUST verify that the declared semantic contract survives.

---

134. Portability and Optimization

An optimization is valid if:

semantic(original) == semantic(optimized)

for the program's observable contract.

A faster implementation is not automatically portable if it violates semantics.

---

135. Portability and Specialization

Specialization is permitted.

The validator MUST require:

generic semantic representation
        ↓
specialized realization

rather than:

target discovery
        ↓
semantic reinterpretation

---

136. Portability and Caching

Cached compilation artifacts may be target-specific.

The cache key MUST include all semantic factors necessary to avoid incorrect reuse.

A target-specific cache artifact MUST NOT be treated as universally portable.

---

137. Portability and Incremental Compilation

Incremental compilation MUST preserve the same portability semantics as full compilation.

A partial recompilation MUST NOT accidentally use stale target assumptions.

---

138. Portability and LTO/PGO/JIT

LTO, PGO, JIT, AOT, and other compilation strategies may be target-aware.

They MUST preserve the portable semantic contract.

Profile information may influence optimization.

It MUST NOT redefine semantics.

---

139. Portability and Deterministic Builds

Where deterministic compilation is promised:

same source
+
same language version
+
same compiler
+
same configuration
+
same dependency graph

MUST produce semantically equivalent artifacts.

Byte identity is required only where separately specified.

---

140. Portability and Runtime Adaptation

Runtime adaptation may be used where permitted.

Examples:

resource migration
device failover
work redistribution
checkpoint recovery
dynamic placement

The validator MUST ensure that adaptation does not violate semantic requirements.

---

141. Portable Failure Semantics

Failure itself can be observable.

Therefore portability validation MUST verify:

error
exception
failure
retry
recovery
degradation

semantics across realizations where the source contract makes them observable.

---

142. Portable Concurrency Semantics

The validator MUST preserve:

- happens-before relations;
- synchronization;
- atomicity;
- ordering guarantees;
- race semantics.

A backend may use different concurrency mechanisms.

---

143. Portable Quantum Measurement Semantics

Measurement is observable.

A quantum backend MUST preserve:

- measurement location;
- measurement result semantics;
- classical feed-forward semantics;
- reset semantics;
- declared stochastic behavior.

Different physical implementations may produce different low-level operations while preserving the declared measurement semantics.

---

144. Portable HDL Semantics

HDL backends may differ in:

- synthesis;
- placement;
- routing;
- timing implementation;
- technology mapping.

The validator MUST preserve declared:

- logic;
- state;
- interfaces;
- timing constraints;
- verification properties.

---

145. Portable Data Semantics

Different storage systems may be used.

The validator MUST preserve:

- schema;
- ordering where observable;
- nullability;
- precision;
- consistency requirements;
- serialization semantics.

---

146. Portable Networking Semantics

Different network paths may be used.

The validator MUST preserve:

- reliability;
- ordering;
- message semantics;
- security requirements;
- delivery guarantees.

---

147. Portability of Effects

Effects may differ in implementation.

The validator MUST ensure that:

effect declaration

is preserved through target realization.

A backend cannot silently erase an effect that is semantically observable.

---

148. Portability of Ownership

Ownership/borrowing/linear/affine semantics MUST survive target changes.

Hardware-specific lowering MUST NOT violate source ownership guarantees.

---

149. Portability of Types

Type meaning MUST remain target-independent unless the type explicitly describes target-specific representation.

For example:

Integer

must not silently mean:

CPU-specific register

unless that is explicitly defined.

---

150. Portability of Representation

Representation may change.

For example:

integer

may become:

machine integer
vector representation
distributed representation
accelerator representation

provided the semantic contract is preserved.

---

151. Portability of Serialization

Serialized forms are interoperability concerns.

Changing serialization format may be portable when the semantic value remains equivalent and the program does not require a particular representation.

If exact byte representation is required, that requirement must be explicit.

---

152. Portability and ABI

ABI selection is downstream.

Portable source MUST NOT assume one physical ABI unless explicitly target-specific.

FFI contracts MUST declare their portability classification.

---

153. Portability and Vendor Extensions

Vendor extensions are permitted through:

dialects
interoperability
target-specific modules
backend contracts

They MUST NOT silently become core language semantics.

---

154. Portability and Future Hardware

Future hardware may expose capabilities unknown to the current compiler.

The language architecture MUST allow those capabilities to be represented without redesigning the core grammar.

Unknown capabilities may be:

deferred
unsupported
dialect-defined
target-specific

depending on context.

---

155. Portability and Unknown Operations

A future quantum operation or accelerator operation SHOULD be representable through open-world operation identifiers when the language supports dynamic operation resolution.

Unknown operation semantics remain a semantic resolution problem, not necessarily a syntax problem.

---

156. Portability and Experimental Features

Experimental features MUST carry an explicit status.

A portability validator MUST NOT treat experimental target integrations as proof of universal portability.

---

157. Portability and Historical Features

Historical constructs from:

- NIMBUS;
- Universal-Trinity;
- Sankofa;
- MTS;
- nano;
- older quantum models;

must be classified according to their current implementation status.

Historical documentation does not establish portability conformance.

---

158. Security of Portability Analysis

Portability analysis MUST be deterministic with respect to its declared inputs.

It MUST NOT depend on:

- hidden hardware state;
- network access;
- undocumented vendor services;
- filesystem side effects;
- arbitrary runtime execution.

Environment discovery belongs to explicit downstream phases.

---

159. Validator Purity

The grammar validation layer SHOULD be side-effect-free.

Validation MUST NOT:

- modify user source;
- allocate physical resources;
- access hardware;
- execute arbitrary program code;
- silently install dependencies;
- contact remote services.

The validator may consume declared metadata and externally supplied target descriptors.

---

160. Safe Rust

Any Rust implementation added for portability validation MUST satisfy:

Rust 2021
Rust 1.97-compatible
safe Rust

and MUST NOT use:

unsafe

or related unsafe implementation constructs.

---

161. Resource Descriptor Model

Target/resource descriptors used by validation SHOULD be data-only semantic descriptions.

Conceptually:

TargetDescriptor {
    capabilities
    resources
    constraints
    topology
    policies
}

The validator consumes this information.

It does not discover physical hardware itself.

---

162. No Global Hardware State

Portability validation MUST NOT depend on mutable global hardware state.

A validation result MUST be explainable from:

program
language version
compiler/specification version
declared target descriptor
declared policies

and other documented inputs.

---

163. Reproducibility of Validation

Given identical validation inputs:

same source
same language contract
same target descriptor
same policy
same compiler

the validator MUST produce the same semantic classification.

---

164. Portability Result Model

A production validator SHOULD produce a structured result equivalent to:

PortabilityResult {
    classification,
    requirements,
    capabilities,
    constraints,
    preferences,
    target_dependencies,
    resource_dependencies,
    adaptations,
    diagnostics,
    source_spans,
    semantic_hash_or_identity,
}

The exact Rust type belongs to the implementation layer.

This file defines the required information, not a mandatory Rust struct name.

---

165. Portability Result Classes

At minimum:

Portable
PortableWithRequirements
PortableWithAdaptation
PortableButResourceUnavailable
PortableButCapabilityUnavailable
PortableButPolicyBlocked
TargetConstrained
TargetSpecific
NonPortable
Invalid

---

166. No False Portability

A validator MUST NOT report a program as portable if:

- physical identity is semantically required;
- vendor API behavior is semantically required;
- target topology is hard-coded;
- resource limits are encoded as universal language rules;
- backend silently changes semantics;
- required capabilities are absent from the portable model;
- target-specific behavior leaks into portable semantics.

---

167. No False Non-Portability

A validator MUST NOT report a program as non-portable merely because:

- the current target is too small;
- a backend is unavailable;
- a capability is temporarily unavailable;
- a particular vendor is unsupported;
- a current machine cannot satisfy the resource requirement.

Those are realization/availability conditions unless the source explicitly requires that target.

---

168. Portability Proof Obligation

For a program claimed to be portable:

P

and compatible targets:

T1
T2
...
Tn

the implementation MUST demonstrate that each accepted realization preserves the declared semantic contract.

The validator need not prove arbitrary mathematical equivalence for all possible targets.

It MUST enforce the repository's defined conformance evidence.

---

169. Conformance Levels

Portability conformance SHOULD use:

Level 0 — Unspecified

No portability contract.

Level 1 — Syntax

Portable syntax parses.

Level 2 — AST

Portable syntax maps correctly to AST.

Level 3 — Semantic

Portability requirements are validated.

Level 4 — IR

Canonical semantics survive IR generation.

Level 5 — Backend

At least one compatible realization preserves semantics.

Level 6 — Cross-target

Multiple realization classes preserve semantics.

Level 7 — Production

Repository-wide traceability, scalability, compatibility, diagnostics, safety, and regression validation pass.

Only Level 7 is production portability conformance.

---

170. Production Gate

A feature MUST NOT be marked production-ready unless:

specification
✓
grammar
✓
lexer
✓
parser
✓
AST
✓
semantic analysis
✓
resource analysis
✓
capability analysis
✓
portability analysis
✓
IR
✓
IR verification
✓
backend integration
✓
runtime integration where applicable
✓
positive tests
✓
negative tests
✓
boundary tests
✓
scalability tests
✓
compatibility tests
✓
determinism tests where applicable
✓
hard-coding audit
✓
source-span validation
✓
safe-Rust validation

all pass.

---

171. Repository-Wide Portability Audit

The production audit MUST inspect:

grammar/
src/
tests/
Cargo.toml
Cargo.lock
build scripts
CI configuration
feature manifests
backend descriptors
runtime descriptors

The audit MUST search for:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

and semantically equivalent constructs.

---

172. Numeric Threshold Audit

The validator SHOULD identify suspicious numeric thresholds near:

- qubit counts;
- thread counts;
- node counts;
- register widths;
- tensor dimensions;
- memory capacities;
- accelerator counts.

A numeric value MUST only be rejected when its role is demonstrably an artificial implementation limit.

---

173. Enumeration Audit

The validator MUST detect closed enumerations representing physical resources.

Examples:

enum Qubit {
    Q0,
    Q1,
    Q2
}

enum Device {
    Cpu0,
    Cpu1
}

These are invalid as universal resource models.

An explicit program-specific enumeration remains legal when it is semantic data.

---

174. Index Audit

The validator MUST inspect index ranges such as:

0..N

and determine whether they represent:

program data

or:

universal implementation capacity

Only the latter is prohibited.

---

175. Array/Vector Audit

The validator MUST ensure that arrays/vectors/tensors are not constrained by accidental host-machine limits.

A source representation may be bounded by its declared type.

The language implementation MUST not impose an undocumented universal smaller bound.

---

176. Parser Collection Audit

Parser collections such as:

item*
argument*
parameter*
resource*
portabilityClause*

MUST NOT contain arbitrary small implementation limits.

A compiler may have configurable resource budgets.

Such budgets MUST be distinct from language semantics.

---

177. Recursion Audit

The validator MUST identify parser/compiler algorithms that can fail for deep valid programs because of accidental recursion.

Where feasible, implementations SHOULD use:

- iterative parsing;
- explicit stacks;
- work queues;
- streaming;
- bounded resource budgets.

---

178. Allocation Audit

The compiler MAY fail allocation due to actual host resource exhaustion.

The validator MUST distinguish:

allocation failure

from:

language capacity limit

---

179. Integer Representation Audit

The lexical layer MUST preserve integer literals without prematurely restricting them to a fixed machine integer type.

Semantic typing determines representability.

IR lowering determines target representation.

---

180. String/Data Audit

Large strings and data values must not be rejected by arbitrary small language limits.

Implementation limits may exist due to memory availability.

They must be classified as resource exhaustion.

---

181. Serialization Audit

Portable semantic artifacts MUST not depend on host-specific serialization.

Explicit binary layout requirements are allowed when part of the semantic contract.

---

182. Hash/Identity Audit

Semantic identity/hash values MUST not include accidental physical target information when the artifact is intended to be portable.

Target-specific artifacts may include target identity.

---

183. Cache Audit

Portable cache keys MUST not accidentally depend on:

CPU ID
GPU ID
QPU ID
physical address
node ID

unless the cache artifact is explicitly target-specific.

---

184. Diagnostic Stability

Portability diagnostics SHOULD remain stable enough for tooling.

Diagnostic identifiers SHOULD be machine-readable.

Human-readable messages MAY evolve.

---

185. IDE/LSP Integration

The LSP/editor layer MUST consume the same portability classifications.

It MUST NOT invent its own target rules.

An IDE may display:

portable
requires capability
target-specific
resource unavailable

using the compiler's semantic model.

---

186. Formatter Integration

Formatting MUST preserve portability semantics.

Formatting must not rewrite:

requirement

into:

preference

or otherwise alter semantic classification.

---

187. Documentation Integration

Documentation examples MUST state whether they are:

portable
target-constrained
target-specific
experimental

A target-specific example must not be presented as universal Zamani.

---

188. Compatibility With "grammar/validation/scalability-rules.md"

This file owns portability consequences.

"scalability-rules.md" owns the detailed scalability audit.

The two validators MUST share the same principle:

no artificial language-level capacity ceilings

Portability validation consumes scalability results.

Scalability validation consumes portability classification where needed.

Neither file may define a competing resource model.

---

189. Compatibility With "grammar/validation/hardcoding-audit.md"

"hardcoding-audit.md" owns the detailed detection of hard-coded limits.

This file requires portability validation to consume those findings.

A hard-coded universal physical capacity is both:

hard-coding violation

and:

portability violation

The same issue MUST have one canonical diagnostic identity where practical.

---

190. Compatibility With "grammar/validation/semantic-boundaries.md"

Semantic boundaries MUST ensure:

source semantics

remain distinct from:

target realization

Portability validation depends on this boundary.

---

191. Compatibility With "grammar/validation/ast-coverage.md"

Every stable portability construct MUST have an AST representation.

An AST omission is a portability completeness failure.

---

192. Compatibility With "grammar/validation/ir-coverage.md"

Every stable portability semantic MUST have a defined IR mapping or explicit reason why it is consumed before IR.

A semantic feature accepted by the parser but discarded before IR is incomplete.

---

193. Compatibility With "grammar/spec/resources.md"

Resource requirements remain resource semantics.

Portability determines whether those requirements survive target changes.

The validator MUST NOT duplicate resource definitions.

---

194. Compatibility With "grammar/spec/compatibility.md"

Language evolution MUST preserve portability guarantees unless the compatibility contract explicitly changes them.

---

195. Compatibility With "grammar/specification/portability.md"

"grammar/specification/portability.md" is the language-wide normative explanation.

This file is the validation contract.

If the two documents disagree, the conflict MUST be reported and resolved at the normative specification level.

This validation file MUST NOT silently override the normative specification.

---

196. Compatibility With "grammar/resources/portability.g4"

"grammar/resources/portability.g4" owns source syntax.

This file validates:

syntax
→
AST
→
semantic portability

The validator MUST ensure that portability grammar syntax does not create a duplicate expression/name/resource language.

---

197. Compatibility With "grammar/compile/target.g4"

Target syntax defines target intent.

This file validates whether that intent is:

preference
constraint
requirement
target-specific realization

---

198. Compatibility With "grammar/execution/deployment.g4"

Deployment syntax defines deployment intent.

Portability validation ensures deployment details do not silently contaminate portable computation semantics.

---

199. Compatibility With "grammar/declarations/resources.g4"

Resource declarations must preserve:

resource
requirement
capability
constraint
preference
negotiation

as distinct concepts.

---

200. Compatibility With "src/source_map.rs"

All portability diagnostics MUST retain source identity and span information.

---

201. Compatibility With "src/lexer.rs"

The lexer MUST provide deterministic tokenization and preserve literals and names needed by portability analysis.

The portability validator MUST NOT assume that a grammar token exists merely because documentation mentions it.

---

202. Compatibility With "src/parser.rs"

The parser MUST construct the portability-related AST without performing target discovery or resource allocation.

---

203. Compatibility With "src/semantic.rs"

Semantic analysis MUST classify portability dependencies.

---

204. Compatibility With "src/ir_gen.rs"

IR generation MUST preserve portable semantics.

Any target-specific lowering must occur after the semantic boundary.

---

205. Compatibility With "src/ir_verify.rs"

IR verification MUST reject semantic drift.

---

206. Compatibility With "src/quantum/ir/"

This remains the canonical quantum semantic/IR boundary.

Portability validation MUST prevent duplicate competing quantum semantic IRs.

---

207. Compatibility With Quantum Routing

Routing is a realization transformation.

It may change physical placement.

It must preserve logical quantum semantics.

---

208. Compatibility With Quantum Scheduling

Scheduling may change timing.

It must preserve required ordering and timing constraints.

---

209. Compatibility With Quantum Resilience

Resilience may alter execution strategy.

It must preserve declared semantic guarantees.

---

210. Compatibility With QEC

QEC may change physical resource requirements.

It must preserve logical semantics.

---

211. Compatibility With ZQN

ZQN may model physical noise/fault properties.

It must not redefine the portable source semantic model.

---

212. Completion Contract for This File

"grammar/validation/portability.md" is complete when:

- its ownership is clear;
- its non-ownership is clear;
- it does not define source syntax;
- it references the normative portability specifications;
- it references the canonical portability grammar;
- it integrates resource validation;
- it integrates hard-coding validation;
- it integrates scalability validation;
- it integrates AST validation;
- it integrates semantic validation;
- it integrates IR validation;
- it integrates source-span validation;
- it integrates compatibility validation;
- it integrates Rust safety validation;
- it defines target-independent semantics;
- it defines target-specific boundaries;
- it defines resource/capability distinctions;
- it defines logical/physical separation;
- it defines quantum portability;
- it defines HDL portability;
- it defines classical portability;
- it defines distributed portability;
- it defines AI/data portability;
- it defines networking portability;
- it defines future-target portability;
- it defines diagnostics;
- it defines positive tests;
- it defines negative tests;
- it defines boundary tests;
- it defines scalability tests;
- it defines deterministic validation;
- it defines production gates.

---

213. Production Readiness Checklist

Authority

- [ ] "grammar/DESIGN.md" is authoritative for architecture.
- [ ] "grammar/specification/portability.md" is authoritative for normative language semantics.
- [ ] "grammar/spec/portability.md" is authoritative for formal portability semantics.
- [ ] "grammar/Zamani.g4" is the only canonical ANTLR root.
- [ ] "grammar/grammar.md" reports implementation status.
- [ ] "grammar/Zamani-Grammar.md" cannot silently promote syntax.

Lexer/parser

- [ ] Lexer authority is explicitly identified.
- [ ] No stale lexer references remain.
- [ ] Parser does not perform target discovery.
- [ ] Portability syntax maps to the AST.
- [ ] Source spans are preserved.

Semantics

- [ ] Requirements are distinct from preferences.
- [ ] Capabilities are distinct from resources.
- [ ] Constraints are distinct from hints.
- [ ] Logical resources are distinct from physical resources.
- [ ] Target-specific semantics have explicit boundaries.
- [ ] Semantic drift is detected.

Scalability

- [ ] No universal resource ceilings exist.
- [ ] No fixed physical-resource enumerations exist.
- [ ] No hidden parser collection limits exist.
- [ ] No artificial tensor limits exist.
- [ ] No fixed thread/core/device limits exist.
- [ ] No fixed quantum gate universe is required.
- [ ] Large symbolic quantities are supported.
- [ ] Actual resource exhaustion is distinguished from language limits.

Quantum

- [ ] "quantum::ir" remains canonical.
- [ ] Logical qubits remain portable.
- [ ] Physical qubit mapping is downstream.
- [ ] Routing is downstream.
- [ ] Scheduling is downstream.
- [ ] QEC is downstream.
- [ ] ZQN is downstream.
- [ ] HAL is downstream.
- [ ] Open-world operations are supported where specified.

Classical

- [ ] CPU-specific details are downstream.
- [ ] SIMD/vector widths are not universal language limits.
- [ ] Core/thread counts are resource information.

HDL

- [ ] Hardware intent is target-independent by default.
- [ ] Widths are semantic/program-defined where appropriate.
- [ ] FPGA/ASIC resources are downstream.
- [ ] Synthesis and placement are downstream.

Distributed

- [ ] No fixed node count exists.
- [ ] Placement is downstream.
- [ ] Topology is downstream.
- [ ] Resource negotiation is explicit.

Build

- [ ] Rust baseline is valid Cargo syntax.
- [ ] Rust 1.97 compatibility is tested if declared supported.
- [ ] Rust 1.97.1 compatibility is tested if declared supported.
- [ ] Rust 2021 is used.
- [ ] Production compiler code contains no "unsafe".
- [ ] Dependencies respect the declared MSRV.

Testing

- [ ] Positive portability tests exist.
- [ ] Negative portability tests exist.
- [ ] Boundary tests exist.
- [ ] Scalability tests exist.
- [ ] Cross-target tests exist.
- [ ] Resource failure tests exist.
- [ ] Capability failure tests exist.
- [ ] Semantic drift tests exist.
- [ ] IR preservation tests exist.
- [ ] Diagnostic source-span tests exist.
- [ ] Compatibility tests exist.
- [ ] Determinism tests exist where applicable.

---

214. Required Test Families

The repository SHOULD maintain portability tests such as:

tests/portability/
├── positive/
├── negative/
├── boundary/
├── scalability/
├── deterministic/
├── resources/
├── capabilities/
├── classical/
├── quantum/
├── hybrid/
├── hdl/
├── hardware/
├── distributed/
├── ai/
├── data/
├── networking/
├── interoperability/
├── target-specific/
├── cross-target/
├── diagnostics/
└── compatibility/

If an existing test hierarchy already provides these categories elsewhere, it MUST be reused rather than duplicated.

---

215. Required Canonical Test Cases

At minimum, tests MUST demonstrate:

Classical

same computation
→ CPU
→ GPU

with equivalent semantics.

Quantum

same logical program
→ QPU A
→ QPU B

with different physical mapping but equivalent logical semantics.

HDL

same hardware intent
→ FPGA
→ ASIC/simulation

where the declared semantics permit such realization.

Distributed

same distributed computation
→ smaller deployment
→ larger deployment

where resource requirements are satisfied.

Resource failure

portable requirement
→ insufficient target resources
→ resource diagnostic

not a language-limit diagnostic.

Capability failure

required capability
→ absent capability
→ capability diagnostic

not a parser error.

---

216. Canonical Hard-Coding Tests

The validator MUST reject implementation-level constructs equivalent to:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

when they define universal language limits.

It MUST also reject semantic equivalents hidden behind different names.

---

217. Canonical Allowed Scaling Tests

The validator MUST accept concepts equivalent to:

requires qubits >= n;
requires memory >= required_memory;
requires capability("tensor.compute");
requires capability("gpu.compute");
requires capability("quantum.measurement");
requires topology(...);

provided the corresponding resource/capability semantics are valid.

These express requirements rather than implementation ceilings.

---

218. Production Invariants

The following invariants are non-negotiable:

1. One Zamani language.
2. One canonical ANTLR composition root.
3. One domain-neutral AST architecture.
4. One canonical quantum semantic boundary: "quantum::ir".
5. No artificial universal hardware limits.
6. Requirements, capabilities, constraints, preferences, hints, and realization remain distinct.
7. Target-specific information cannot silently leak into portable semantics.
8. Resource failure cannot be converted into a language limitation.
9. Capability failure cannot be converted into a syntax failure.
10. Backend optimization cannot change semantics.
11. QEC cannot redefine logical quantum semantics.
12. Routing cannot redefine logical topology.
13. Scheduling cannot redefine semantic ordering.
14. HAL cannot redefine language capabilities.
15. Historical documentation cannot silently become normative.
16. Experimental features cannot silently become stable.
17. Rust implementation remains safe.
18. Rust implementation remains compatible with the declared Rust baseline.
19. Every stable portability-sensitive feature has complete traceability.
20. No portability claim is made without corresponding implementation/conformance evidence.

---

219. Final Portability Model

The production model is:

                    ZAMANI SOURCE
                         │
                         ▼
                SOURCE SEMANTICS
                         │
             ┌───────────┼───────────┐
             ▼           ▼           ▼
          Types       Effects      Ownership
             │           │           │
             └───────────┼───────────┘
                         ▼
                  Resource Intent
                         │
                         ▼
                  Capability Intent
                         │
                         ▼
                 Portability Analysis
                         │
                         ▼
                 Canonical Semantics
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
      Classical      quantum::ir    HDL/Hardware
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                     Canonical IR
                         │
                         ▼
                    Optimization
                         │
              ┌──────────┼──────────┐
              ▼          ▼          ▼
           Routing   Scheduling   Resilience
              │          │          │
              └──────────┼──────────┘
                         ▼
                        QEC
                         │
                         ▼
                        ZQN
                         │
                         ▼
                        HAL
                         │
                         ▼
                TARGET REALIZATION
                         │
       ┌─────────┬───────┼───────┬─────────┐
       ▼         ▼       ▼       ▼         ▼
      CPU       GPU     FPGA     QPU     Future
       │         │       │       │       targets
       └─────────┴───────┴───────┴─────────┘
                         │
                         ▼
                      RUNTIME

---

220. Final POCO-REAF Rule

The portability validator MUST enforce the following principle:

«Zamani source describes computation and its semantic requirements. It does not unnecessarily describe the physical machine that will eventually realize that computation.»

Therefore:

PROGRAM ONCE
      ↓
DEFINE SEMANTICS ONCE
      ↓
COMPILE ONCE
      ↓
PRESERVE CANONICAL SEMANTICS
      ↓
DISCOVER CAPABILITIES
      ↓
NEGOTIATE RESOURCES
      ↓
SPECIALIZE
      ↓
OPTIMIZE
      ↓
ROUTE
      ↓
SCHEDULE
      ↓
APPLY RESILIENCE / QEC / ZQN
      ↓
REALIZE THROUGH HAL
      ↓
RUN

The same source semantics must be capable of scaling from:

atom

through:

single value
single operation
embedded system
CPU
multicore
GPU
FPGA
ASIC
QPU
accelerator
cluster
distributed system
cloud
future architecture

without turning today's physical limitations into permanent language limitations.

---

221. Final Definition of Portability

For Zamani:

«Portability is the preservation of the program's declared semantic contract while its implementation realization changes.»

For Zamani:

«Scalability is the ability of the same language semantics to represent workloads whose size and resource requirements vary without an artificial universal capacity ceiling.»

For Zamani:

«POCO-REAF means that developers should be able to express computation once and reuse its semantic definition across compatible computational realizations without rewriting the program merely because the underlying hardware, topology, resource quantity, or execution strategy changes.»

The compiler may adapt.

The optimizer may adapt.

The router may adapt.

The scheduler may adapt.

QEC may adapt.

ZQN may adapt.

The HAL may adapt.

The runtime may adapt.

The physical machine may change.

The program's declared semantics must remain the source of truth.

---

222. File Completion Statement

"grammar/validation/portability.md" is considered production-complete as a validation contract when the repository implements the checks defined here.

This file itself MUST NOT claim that the entire Zamani compiler is production-portable merely because the specification exists.

The correct state distinction is:

PORTABILITY CONTRACT
        ↓
DEFINED

PORTABILITY VALIDATOR
        ↓
IMPLEMENTED / NOT YET IMPLEMENTED

REPOSITORY PORTABILITY
        ↓
CONFORMANT / PARTIALLY CONFORMANT / NON-CONFORMANT

This distinction is mandatory.

A specification is not evidence of implementation.

---

223. Non-Negotiable Final Rule

Zamani MUST NOT solve today's hardware limitations by turning them into tomorrow's language limitations.

Do not encode:

today's maximum qubits
today's maximum CPUs
today's maximum GPUs
today's maximum FPGA resources
today's maximum tensor dimensions
today's maximum memory
today's maximum network size
today's maximum nodes
today's maximum accelerators

as permanent language restrictions.

Instead encode:

semantic intent
+
resource requirements
+
capability requirements
+
constraints
+
preferences
+
hints

and let:

compiler
+
optimizer
+
router
+
scheduler
+
resilience
+
QEC
+
ZQN
+
HAL
+
runtime
+
backend

determine the physical realization.

That is the portability contract required for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

with scalability limited by actual program semantics, declared requirements, representational capability, policies, and resources available to the realization, rather than by arbitrary language-level hardware ceilings.