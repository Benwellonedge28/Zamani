Zamani Portability Specification

Path: "grammar/specification/portability.md"
Status: Normative / Production
Scope: Language-wide portability, scalability, target independence, resource adaptation, and POCO-REAF
Language: Zamani
Implementation baseline: Rust 1.97 / Rust 1.97.1
Implementation safety requirement: "unsafe" Rust is prohibited

---

1. Purpose

This document defines the normative portability model of the Zamani programming language.

Zamani portability is based on the principle:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF).»

A Zamani program describes computation, behavior, correctness, resource requirements, capabilities, constraints, policies, and other semantic intent independently of any particular physical machine.

The same source program MUST NOT require source-level rewriting merely because it is executed on:

- a smaller machine;
- a larger machine;
- a CPU;
- a GPU;
- an FPGA;
- an ASIC;
- a QPU;
- a heterogeneous system;
- a workstation;
- an embedded system;
- a cluster;
- a distributed system;
- an edge system;
- a cloud system;
- a future computational architecture.

Portability is therefore a semantic property of the language, not merely a property of a compiler backend.

---

2. Authority

This document is authoritative for portability semantics.

The authority relationship is:

grammar/specification/portability.md
        │
        ├── defines portability semantics
        ├── defines POCO-REAF
        ├── defines scalability requirements
        ├── defines target independence
        └── defines resource/capability separation
                │
                ▼
grammar/specification/language.md
grammar/specification/semantics.md
grammar/spec/type-system.md
grammar/spec/resources.md
grammar/spec/determinism.md
grammar/spec/compatibility.md
                │
                ▼
grammar/Zamani.g4
                │
                ▼
lexer → parser → AST → semantic analysis
                │
                ▼
canonical semantic model / IR
                │
                ├── classical IR
                ├── quantum::ir
                └── HDL/hardware IR
                │
                ▼
optimization
routing
scheduling
QEC / resilience
ZQN
HAL
backend
runtime

This document MUST NOT create a competing grammar.

"grammar/Zamani.g4" defines syntactic composition.

"grammar/grammar.md" documents implementation conformance.

"grammar/Zamani-Grammar.md" is a design/history/reference source and MUST NOT silently introduce normative syntax.

"grammar/DESIGN.md" defines the overall grammar architecture.

This document defines the portability contract shared by those layers.

---

3. Definitions

3.1 Program

A program is the semantic computation expressed by Zamani source code.

A program includes:

- declarations;
- expressions;
- statements;
- types;
- control flow;
- effects;
- resource requirements;
- capabilities;
- correctness requirements;
- domain semantics;
- interoperability contracts.

A program does not inherently include a particular physical machine.

---

3.2 Target

A target is a computational environment capable of executing some or all of a program.

Examples include:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- CPU/GPU system;
- CPU/QPU system;
- FPGA/QPU system;
- distributed cluster;
- cloud environment;
- embedded system;
- future accelerator.

A target is an implementation concern unless the source program explicitly declares a target-dependent requirement.

---

3.3 Resource

A resource is something required or useful for executing a program.

Examples include:

- compute capacity;
- memory;
- storage;
- communication capacity;
- accelerator capacity;
- quantum resources;
- timing capacity;
- energy budget;
- reliability;
- bandwidth;
- concurrency capacity.

Resources are represented semantically rather than through fixed universal language limits.

---

3.4 Capability

A capability describes an operation or property that an execution environment can provide.

Examples:

quantum.measurement
quantum.mid_circuit_measurement
tensor.compute
parallel.execution
distributed.communication
hardware.reconfiguration
secure.execution

Capabilities MUST be resolved against the actual execution environment.

---

3.5 Requirement

A requirement states something necessary for correct execution.

Example:

requires capability("quantum.mid_circuit_measurement")

A requirement is not a physical placement instruction.

---

3.6 Constraint

A constraint limits the set of valid implementations or executions.

Example:

requires memory >= required_memory

A constraint MUST NOT be interpreted as a universal compiler limit.

---

3.7 Preference

A preference identifies a desirable implementation choice without making it mandatory.

Example:

prefer accelerator("quantum")

A preference MAY be ignored when necessary to preserve correctness.

---

3.8 Hint

A hint provides optimization information.

A hint MUST NOT change program semantics unless explicitly declared as a semantic constraint.

---

3.9 Implementation decision

An implementation decision is a choice made by the compiler, runtime, scheduler, router, backend, HAL, or deployment system.

Examples:

physical_qubit = 17
GPU = device_3
CPU_core = 12
memory_bank = 4
node = cluster_node_8

These MUST NOT become implicit source-level requirements.

---

4. Core Portability Principle

The fundamental Zamani rule is:

«Program semantics MUST be independent of implementation-specific physical resource identities unless the programmer explicitly requests a non-portable interoperability contract.»

The following MUST NOT be universal language assumptions:

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
MAX_TENSOR_DIMENSION
MAX_TENSOR_RANK
MAX_TIMELINES
MAX_PROCESSES
MAX_CHANNELS
MAX_DEVICES
MAX_NETWORK_LINKS

The absence of such limits is a semantic requirement.

An implementation MAY have practical limits imposed by:

- available memory;
- compiler resources;
- operating-system limits;
- backend limits;
- device capacity;
- execution quotas;
- timeouts;
- deployment policies.

Those limits MUST NOT redefine the Zamani language.

---

5. POCO-REAF

5.1 Meaning

POCO-REAF means:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever.»

The objective is that a programmer writes the computational intent once.

The implementation then adapts that intent to available environments.

Conceptually:

                    ┌── CPU
                    ├── GPU
                    ├── FPGA
Zamani Program ─────┼── QPU
                    ├── cluster
                    ├── cloud
                    ├── edge
                    └── future target

The source program is not rewritten for each target.

---

5.2 Semantic identity

A portable program MUST retain a stable semantic identity across derived artifacts.

The following may change:

- target;
- physical layout;
- scheduling;
- instruction selection;
- optimization;
- decomposition;
- device assignment;
- memory placement;
- communication topology;
- execution strategy.

The following MUST remain semantically equivalent unless explicitly permitted by the program:

- observable results;
- declared effects;
- correctness guarantees;
- resource semantics;
- ordering guarantees;
- type semantics;
- ownership semantics;
- domain semantics.

---

6. Compile-Once Model

"Compile once" refers to preservation and reuse of a target-independent compiled representation.

The architecture SHOULD support:

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
canonical semantic model
  ↓
canonical IR
  ↓
portable compilation artifact
  ↓
target-specific realization

A target-specific realization MAY include:

- machine code;
- accelerator code;
- quantum circuit;
- HDL;
- device program;
- distributed deployment plan;
- runtime schedule.

Such derived artifacts MUST NOT require modification of the original source program merely because the target changes.

---

7. Important Qualification of "Compile Once"

POCO-REAF does not require every physical target to execute identical machine instructions.

For example:

same Zamani semantic program
        │
        ├── CPU lowering
        ├── GPU lowering
        ├── FPGA lowering
        ├── QPU lowering
        └── distributed lowering

The compiler MAY perform target-specific lowering.

The distinction is:

source semantics ≠ target implementation

Therefore:

«Target-specific compilation MUST be a derived transformation, not a requirement to rewrite the Zamani source program.»

---

8. Target Independence

Portable Zamani source SHOULD describe:

- what must be computed;
- what values must be produced;
- what relationships must hold;
- what capabilities are required;
- what resources are required;
- what constraints apply;
- what correctness properties apply;
- what effects are permitted;
- what performance characteristics are preferred.

Portable source SHOULD NOT unnecessarily describe:

- physical CPU identifiers;
- physical GPU identifiers;
- physical qubit identifiers;
- memory-bank identifiers;
- PCI addresses;
- machine-specific register numbers;
- cluster-node identifiers;
- vendor-specific execution units;
- fixed device topology.

---

9. Resource Abstraction

Resource quantities are semantic values.

A resource quantity MAY be:

- constant;
- variable;
- symbolic;
- parameterized;
- data-dependent;
- negotiated at compile time;
- negotiated at runtime.

For example:

let n = input_size();
requires qubits >= n;

does not create a language-level maximum for "n".

Likewise:

requires memory >= required_memory;

does not mean that Zamani has a fixed maximum memory size.

---

10. No Artificial Resource Limits

A grammar or semantic rule MUST NOT reject a program merely because its resource quantity exceeds an implementation-selected constant.

Invalid:

if qubit_count > 1024:
    reject_program()

when "1024" is merely an implementation limitation.

Correct behavior:

program requires N qubits
        ↓
discover available capability
        ↓
if sufficient:
    continue
else:
    report unsatisfied resource requirement

The compiler MAY reject execution because the selected target lacks the requested resource.

It MUST NOT claim that the language itself forbids the program.

---

11. Tiny-to-Infinite Scaling

Zamani defines no artificial upper semantic bound on computational scale.

"Infinity" in this specification means:

«Any finite program/resource requirement representable by the language and executable subject to actual available resources.»

A physical machine is always finite.

Therefore:

language scale
    >
implementation scale

is permitted.

The language MUST support programs ranging from:

one value

to:

large distributed workloads

without requiring a separate language mode.

---

12. Resource Availability

Execution feasibility is determined by actual resource availability.

Conceptually:

program requirements
        +
target capabilities
        +
runtime state
        +
deployment policy
        ↓
execution feasibility

If the required resources are unavailable, the implementation MUST distinguish:

1. statically impossible;
2. dynamically unavailable;
3. temporarily unavailable;
4. policy-prohibited;
5. unsupported capability;
6. insufficient capacity.

These conditions MUST NOT be silently converted into successful execution.

---

13. Dynamic Resource Negotiation

Resource requirements MAY depend on runtime information.

Example:

n = dataset.size();

requires memory >= memory_required(n);

The runtime MAY negotiate:

- memory;
- compute;
- accelerators;
- quantum resources;
- distributed placement;
- communication resources;
- concurrency.

The language MUST NOT require a fixed resource count merely because the runtime cannot know the count at parse time.

---

14. Requirements, Constraints, Preferences, Hints

These categories MUST remain distinct.

Category| Meaning| Can prevent execution?| Typical owner
Requirement| Necessary semantic condition| Yes| semantic analysis/runtime
Constraint| Valid implementation/execution bound| Yes| compiler/runtime
Preference| Desirable choice| Normally no| optimizer/scheduler
Hint| Optimization information| No| compiler/runtime
Capability| Available target property| N/A| HAL/environment
Implementation decision| Actual realization| N/A| compiler/runtime/backend

A preference MUST NOT accidentally become a requirement.

A hint MUST NOT silently become a constraint.

A hardware implementation decision MUST NOT leak backward into portable semantics.

---

15. Hardware Independence

Portable Zamani source MUST NOT assume a particular:

- CPU architecture;
- instruction-set architecture;
- register count;
- register width;
- cache hierarchy;
- GPU architecture;
- FPGA family;
- ASIC implementation;
- QPU architecture;
- memory technology;
- interconnect topology.

Hardware-specific information MAY be declared when explicitly required for interoperability or specialized deployment.

Such declarations MUST be clearly classified as target-dependent.

---

16. Physical Identity

Logical resources and physical resources MUST remain distinct.

For quantum computing:

logical qubit

is not inherently:

physical qubit 17

For distributed computing:

logical worker

is not inherently:

node-8

For GPU computing:

accelerator

is not inherently:

device 3

Physical mapping belongs downstream.

---

17. Quantum Portability

Quantum programs MUST be portable across QPUs with different:

- qubit counts;
- connectivity;
- gate sets;
- coherence characteristics;
- measurement capabilities;
- error rates;
- timing;
- calibration;
- control capabilities.

A quantum program SHOULD express:

logical qubits
logical operations
measurement requirements
resource requirements
capabilities
correctness constraints

rather than physical layout.

---

18. Quantum Operation Portability

Quantum syntax MUST NOT require a fixed enumeration of all possible gates.

The semantic representation MUST support:

- named operations;
- parameterized operations;
- controlled operations;
- adjoint operations;
- custom operations;
- composed operations;
- domain-specific operations;
- future operations.

The canonical semantic boundary remains the existing:

quantum::ir

The grammar MUST NOT create a competing quantum IR.

---

19. Quantum Mapping

The portable pipeline is:

logical quantum program
        ↓
quantum semantic analysis
        ↓
quantum::ir
        ↓
optimization
        ↓
decomposition
        ↓
routing
        ↓
scheduling
        ↓
QEC / resilience
        ↓
ZQN fault/noise semantics
        ↓
HAL capability/state
        ↓
physical target

Routing owns physical realization.

Scheduling owns timing/order/resource scheduling.

QEC owns error detection/correction.

ZQN owns fault/noise semantics.

HAL owns actual device capability/state.

Resilience owns recovery/orchestration.

Optimization owns implementation improvement.

Portability semantics MUST NOT duplicate those responsibilities.

---

20. No Implicit Quantum Hardware Assumptions

The language MUST NOT assume:

q[0]
q[1]

or any other fixed physical qubit identity.

The language MUST NOT automatically:

- map logical qubits to physical IDs;
- assume a fixed topology;
- assume all-to-all connectivity;
- automatically measure all remaining qubits;
- silently discard unsupported operations;
- convert unknown operations into comments.

Unsupported operations MUST produce a proper diagnostic or be handled through an explicitly declared capability/extension mechanism.

---

21. Classical Portability

Classical programs MUST remain independent of:

- CPU width;
- core count;
- thread count;
- cache size;
- SIMD width;
- register count;
- instruction-set extensions.

For example:

parallel compute(data)

MAY execute with:

1 worker
8 workers
1000 workers

depending on available resources.

The semantics of the computation MUST remain stable.

---

22. Numeric Portability

Numeric semantics MUST distinguish:

mathematical value

from:

machine representation

A type MUST NOT silently inherit implementation-specific limits.

Conversions MUST be explicit when information could be lost.

Overflow behavior MUST be defined by the type semantics.

The compiler MAY choose:

- scalar representation;
- vector representation;
- arbitrary precision;
- hardware floating point;
- software emulation;
- accelerator representation.

Such choices MUST preserve the specified semantics.

---

23. Size, Count, Index, and Resource Quantities

Semantic quantities such as:

- count;
- size;
- index;
- duration;
- capacity;
- memory size;
- tensor dimension;
- qubit count;

MUST NOT be defined solely in terms of an implementation-fixed integer type.

For example, the semantics of:

len(x)
sizeof(T)
shape(x)

MUST NOT be universally defined as "Rust "i64"" merely because a current implementation happens to use "i64".

Lowering MAY use an appropriate concrete representation.

Conversions MUST be checked where required.

---

24. Tensor and Array Portability

Tensor dimensions and ranks MUST NOT have language-level fixed maximums.

Valid conceptual forms include:

Tensor<T, shape>
Tensor<T, symbolic_shape>
Tensor<T, dynamic_shape>

The implementation MAY impose resource limits during actual compilation or execution.

Such limits are environmental constraints, not grammar restrictions.

---

25. HDL Portability

HDL semantics MUST distinguish:

hardware intent

from:

specific physical implementation

A hardware description MAY specify:

- interfaces;
- signals;
- clocks;
- timing requirements;
- state;
- combinational behavior;
- sequential behavior;
- memory semantics;
- pipelines;
- protocols;
- verification properties;
- resource requirements.

It SHOULD avoid assuming a specific FPGA/ASIC architecture unless the source explicitly requests target-specific behavior.

---

26. Parameterized Hardware

Hardware dimensions MAY be semantic parameters.

For example:

parameter width = W;

means the hardware description is parameterized by "W".

It MUST NOT mean:

W <= implementation_constant

unless such a constraint is explicitly declared.

---

27. Hardware/Software Co-Design

Zamani permits a program to express coordinated:

- software;
- hardware;
- accelerator;
- memory;
- communication;
- timing;
- verification;
- deployment intent.

The same semantic computation MAY therefore be lowered into different implementations.

Example:

algorithm
   ↓
semantic representation
   ├── CPU implementation
   ├── GPU implementation
   ├── FPGA implementation
   └── QPU implementation

The source algorithm remains one semantic program.

---

28. Distributed Portability

Distributed programs MUST NOT require a fixed number of nodes unless that number is itself part of program semantics.

The language MAY express:

replicate service according to workload

or:

requires capacity >= workload

without specifying a fixed physical cluster size.

The implementation MAY choose:

- one process;
- multiple processes;
- one node;
- multiple nodes;
- heterogeneous nodes;
- cloud resources;
- edge resources.

---

29. Distributed Topology

Logical communication topology and physical network topology MUST remain separate.

The program MAY require:

low-latency communication

without specifying:

node A → switch 4 → node B

Physical routing belongs to deployment/networking/runtime layers.

---

30. Concurrency Portability

Concurrency semantics MUST NOT depend on a fixed number of workers.

These are portable:

parallel
spawn
async
await
pipeline
data_parallel
task_parallel

These are target-specific unless explicitly requested:

run_on_core(3)
run_on_gpu(7)
run_on_thread(12)

Logical concurrency MAY exceed currently available physical concurrency.

The scheduler MAY serialize, batch, pipeline, or distribute work while preserving specified semantics.

---

31. Determinism

Portability MUST preserve the determinism guarantees specified by the program.

For a deterministic program:

same source
+
same semantic inputs
+
same specified environment
+
same specified seed/state

MUST produce equivalent observable results.

Parallel execution MUST NOT introduce observable nondeterminism where the language contract prohibits it.

If nondeterminism is permitted, it MUST be explicit through the appropriate effect/capability semantics.

See:

grammar/specification/semantics.md
grammar/spec/determinism.md

for detailed rules.

---

32. Floating-Point and Numerical Reproducibility

Exact bit-for-bit equality MUST NOT be assumed across targets unless the program explicitly requests an appropriate reproducibility contract.

The language MUST distinguish:

- mathematical equivalence;
- numerical tolerance;
- deterministic ordering;
- bitwise reproducibility.

A compiler MAY use a different numerical implementation if the resulting behavior satisfies the declared semantic contract.

---

33. Memory Portability

Portable programs MUST NOT assume a particular:

- RAM size;
- VRAM size;
- cache size;
- memory bank;
- address;
- NUMA topology;
- accelerator memory size.

The program MAY express memory requirements.

Example:

requires memory >= required_memory

The compiler/runtime MAY then:

- allocate;
- tile;
- stream;
- spill;
- distribute;
- recompute;
- use accelerator memory;

provided the semantic contract remains satisfied.

---

34. Storage Portability

Portable source MUST NOT require a particular:

- disk;
- SSD;
- storage device;
- filesystem layout;
- block size;
- physical path.

Storage requirements SHOULD be expressed through abstract capabilities and policies.

---

35. Networking Portability

Networking semantics SHOULD use abstract:

- endpoints;
- services;
- channels;
- protocols;
- capabilities;
- security policies.

Physical IP addresses, ports, interfaces, switches, and routes are deployment concerns unless explicitly required by an interoperability contract.

---

36. Security Portability

Security requirements MUST remain semantic.

For example:

requires capability("secure_execution")
requires confidentiality(data)
requires authenticated(channel)

is preferable to silently assuming a particular hardware security module.

Actual implementation MAY use:

- secure enclaves;
- hardware security modules;
- software cryptography;
- QKD;
- post-quantum cryptography;
- future mechanisms.

The semantic guarantee is authoritative.

---

37. AI/ML Portability

AI/ML programs MUST NOT depend on a particular framework merely because that framework is available on the current target.

The language MAY express:

- model;
- tensor;
- dataset;
- training;
- inference;
- optimization;
- differentiation;
- probabilistic behavior;
- agents;
- symbolic computation.

The implementation MAY lower those concepts to different frameworks/backends.

Tensor dimensions, accelerator counts, and training resources MUST remain dynamic or explicitly parameterized.

---

38. Future Computational Models

Portability MUST not be limited to currently known architectures.

The architecture MUST permit future targets such as:

- new accelerator classes;
- new quantum architectures;
- neuromorphic systems;
- photonic systems;
- molecular/nano computation;
- optical computation;
- biological computation;
- distributed future architectures;
- unknown future execution technologies.

A new target SHOULD require a new backend/capability implementation rather than a rewrite of the core language.

---

39. Domain-Neutral Core

Portability depends on a shared language foundation.

Classical, quantum, HDL, hybrid, AI, data, distributed, networking, security, and future domains MUST share:

- names;
- types;
- expressions;
- declarations;
- functions;
- modules;
- effects;
- ownership;
- resource semantics;
- capability semantics;
- diagnostics;
- versioning.

Domains MAY extend the language but MUST NOT redefine universal semantics inconsistently.

---

40. Interoperability and Non-Portable Code

Zamani MUST permit explicit interoperability with target-specific systems.

Examples include:

- foreign functions;
- vendor APIs;
- device-specific operations;
- physical addresses;
- ABI contracts;
- external HDL;
- external quantum formats.

However, such constructs MUST be explicitly marked as target-dependent or otherwise constrained.

For example:

portable computation
        ↓
explicit target-specific boundary
        ↓
vendor/device implementation

The target-specific boundary MUST NOT contaminate the semantics of unrelated portable code.

---

41. Portability Domains

A construct MAY be classified as:

portable
conditionally portable
target-constrained
target-specific
non-portable

Portable

No target-specific assumptions.

Conditionally portable

Portable when declared capabilities/resources are available.

Target-constrained

Requires a declared target property.

Target-specific

Explicitly tied to a target family or implementation.

Non-portable

Depends directly on physical implementation details.

Tooling SHOULD make this classification visible.

---

42. Portability Classification

Every feature specification under:

grammar/specification/
grammar/spec/

SHOULD identify its portability class.

Every feature manifest under:

grammar/specification/features/

MUST provide:

portability:
requirements:
capabilities:
constraints:
preferences:
target_dependencies:
hard_coding_policy:

This creates a machine-checkable portability contract.

---

43. Source-to-Target Contract

Every feature MUST define:

source syntax
    ↓
AST representation
    ↓
semantic representation
    ↓
canonical IR mapping
    ↓
compiler interpretation
    ↓
runtime interpretation

A feature is not portable merely because its syntax parses.

For each feature, the implementation MUST know:

- what it means;
- what resources it requires;
- what capabilities it requires;
- what effects it has;
- what IR represents it;
- what targets can implement it;
- what happens when requirements cannot be satisfied.

---

44. AST Independence

The frontend AST MUST remain target-independent.

The AST MUST NOT encode:

- physical CPU IDs;
- physical GPU IDs;
- physical qubit mappings;
- routing decisions;
- scheduling decisions;
- calibration data;
- vendor backend instructions.

Such information belongs downstream.

The existing domain-neutral AST architecture therefore remains the portability boundary between syntax and target-independent semantic analysis.

---

45. Canonical IR

Portability requires a canonical semantic representation.

The canonical representation MUST preserve:

- logical identity;
- types;
- effects;
- resource requirements;
- capabilities;
- domain semantics;
- source provenance;
- correctness properties.

For quantum computation, the existing:

quantum::ir

remains canonical.

The grammar MUST NOT create a second quantum IR.

---

46. Compiler Responsibilities

The compiler is responsible for transforming portable intent into an executable realization.

It MAY perform:

- specialization;
- optimization;
- vectorization;
- parallelization;
- distribution;
- accelerator selection;
- quantum decomposition;
- quantum routing;
- scheduling;
- memory planning;
- hardware synthesis;
- code generation.

These decisions MUST be derived from semantic requirements and target capabilities.

---

47. Runtime Responsibilities

The runtime is responsible for conditions that cannot be completely resolved statically.

It MAY:

- discover resources;
- negotiate capabilities;
- allocate resources;
- schedule execution;
- select implementations;
- monitor health;
- recover from transient failures;
- adapt placement;
- manage distributed execution;
- perform dynamic dispatch.

Runtime adaptation MUST preserve the language's semantic contract.

---

48. HAL Responsibilities

The HAL is responsible for exposing actual target capabilities and state.

The HAL MAY report:

- available compute resources;
- memory;
- accelerators;
- quantum-device capabilities;
- topology;
- calibration;
- timing;
- reliability;
- device state.

The HAL MUST NOT redefine language semantics.

---

49. Routing Responsibilities

Routing determines physical realization where logical resources require physical placement.

For example:

logical quantum interaction
        ↓
physical connectivity
        ↓
routing

Routing MUST NOT require the source program to encode physical mappings merely to execute on a particular machine.

---

50. Scheduling Responsibilities

Scheduling determines:

- ordering;
- timing;
- resource sharing;
- concurrency;
- synchronization;
- placement timing.

The scheduler MUST operate over dynamic resource descriptions.

It MUST NOT impose fixed universal limits on:

- qubits;
- threads;
- nodes;
- accelerators;
- tasks.

---

51. QEC and Resilience

Portability does not mean ignoring target reliability.

A portable quantum program MAY express requirements such as:

requires fault_tolerance(...)
requires reliability(...)
requires error_correction(...)

The implementation may then use existing QEC/resilience mechanisms.

QEC MUST remain responsible for error detection/correction.

Resilience MUST remain responsible for orchestration/recovery.

Duplicated resource-limit definitions MUST NOT be introduced.

---

52. ZQN

ZQN represents fault/noise semantics.

Portability MAY require:

noise tolerance
fault model
reliability requirement
error budget

but MUST NOT encode a particular physical device unless explicitly target-specific.

The flow remains:

semantic requirement
        ↓
ZQN
        ↓
target noise/fault model

not:

grammar → physical device

---

53. Calibration

Calibration data MUST remain outside portable source semantics unless the programmer explicitly declares a calibration-related requirement.

A portable program may say:

requires calibrated capability(...)

but should not normally contain:

use calibration table device_7 revision_4

as its core computational meaning.

---

54. Resource Failure Semantics

If a required resource is unavailable:

Compile-time known failure

The compiler MUST reject the derived target realization with a diagnostic.

Runtime-discovered failure

The runtime MUST report a structured failure or invoke an explicitly declared fallback/recovery policy.

Preference unavailable

The implementation MAY choose another valid implementation.

Hint unavailable

The implementation MAY ignore the hint.

The implementation MUST NOT silently violate a requirement.

---

55. Fallbacks

Fallbacks MUST be explicit.

Example conceptual model:

requires capability("accelerator.compute")
fallback software_implementation

means fallback is part of the program's semantics.

Without an explicit fallback policy, an implementation MUST NOT silently substitute a semantically different execution strategy.

---

56. Graceful Scaling

Where semantics permit it, implementations SHOULD adapt to resource availability.

For example:

parallel workload

may execute as:

1 worker
→
many workers
→
distributed workers

without source changes.

However, automatic scaling MUST NOT alter specified observable semantics.

---

57. Resource Exhaustion

Resource exhaustion is an execution/compilation condition, not a grammar condition.

Examples:

out of memory
no available QPU
insufficient accelerator capacity
network capacity exhausted
scheduler capacity exhausted

MUST NOT be represented as:

invalid Zamani syntax

unless the source itself violates a language rule.

---

58. Compiler Resource Limits

The compiler MAY have operational limits.

Examples:

- memory available to the compiler;
- compilation time;
- backend capacity;
- recursion limits;
- cache limits.

These are implementation/environment constraints.

They MUST NOT become semantic limits such as:

Zamani supports only 1024 qubits

or:

Zamani supports only 64 tensor dimensions

unless deliberately introduced as a separately versioned language restriction, which is prohibited for the universal portability model.

---

59. Runtime Resource Limits

The runtime MAY enforce policy limits.

For example:

maximum execution time
maximum deployment cost
maximum memory allocation
maximum tenant quota

These are environment policies.

They MUST be reported as policy/resource failures rather than language incompatibilities.

---

60. Portability and Ownership

Ownership and borrowing semantics MUST remain target-independent.

A program's ownership behavior MUST NOT change because it is compiled for:

- CPU;
- GPU;
- FPGA;
- QPU;
- distributed execution.

Target-specific memory management is a lowering concern.

Resource handles representing unique resources MUST obey the same ownership semantics regardless of target.

---

61. Portability and Effects

Effects are part of the semantic portability contract.

Examples:

IO
network
randomness
device_access
quantum_measurement
persistent_storage
distributed_communication

MUST be explicit according to the language effect system.

A target cannot silently remove or add an observable effect merely because its implementation differs.

---

62. Portability and Macros

Macros MUST preserve portability classifications.

A macro MUST NOT secretly introduce target-specific behavior into otherwise portable code.

Macro expansion MUST undergo normal semantic analysis.

Therefore:

macro expansion
        ↓
AST
        ↓
semantic analysis
        ↓
portability analysis

A macro cannot bypass the portability rules.

---

63. Portability and Metaprogramming

Compile-time reflection and code generation MUST distinguish:

program semantics

from:

compiler/environment information

A program MAY inspect target capabilities when explicitly permitted.

However, target inspection MUST NOT automatically make the entire program non-portable.

The resulting specialization MUST remain traceable.

---

64. Portability and Dialects

A dialect MAY introduce target-specific syntax.

Every dialect MUST declare:

- portability classification;
- target dependencies;
- capabilities;
- semantic extensions;
- AST mapping;
- IR mapping;
- compatibility;
- version.

A dialect MUST NOT silently redefine core portability semantics.

---

65. Portability and Interoperability Formats

Formats such as:

- OpenQASM;
- QIR;
- LLVM;
- MLIR;
- HDL formats;
- C;
- C++;
- Rust;
- WebAssembly;

are interoperability or lowering formats.

They MUST NOT become the canonical semantic authority for Zamani portability.

Zamani semantics remain authoritative.

---

66. Source Compatibility Across Hardware Generations

A valid portable Zamani program SHOULD remain source-compatible when a target evolves.

For example:

QPU generation A
QPU generation B
QPU generation C

may expose different capabilities.

The source remains unchanged if all required capabilities remain satisfiable.

If a capability disappears, the implementation MUST report the capability mismatch.

---

67. Capability Evolution

New target capabilities MAY be added without breaking existing programs.

For example:

quantum.new_operation

may become available later.

A program that does not require it MUST remain valid.

A program that requires it MUST declare that dependency.

---

68. Capability Negotiation

Capability negotiation SHOULD be represented independently from resource quantities.

For example:

requires capability("quantum.mid_circuit_measurement")
requires qubits >= n

contains two different conditions:

capability

and:

resource capacity

Both must be evaluated.

---

69. Resource Scaling Functions

Resource requirements MAY be functions of program inputs.

For example:

memory_required(n)
qubits_required(n)
workers_required(n)
bandwidth_required(n)

The language MUST permit these quantities to scale with workload size.

The implementation MUST NOT replace them with fixed constants.

---

70. Complexity Is Not Capacity

A program's algorithmic complexity and target capacity are distinct.

For example:

O(n²)

does not mean:

n <= 1024

Likewise:

requires n qubits

does not imply a fixed maximum "n".

The runtime environment determines whether the actual instance is executable.

---

71. Topology Independence

Programs SHOULD express logical communication requirements rather than physical topology.

Examples:

requires connected(resources)
requires low_latency(channel)
requires bandwidth >= b

are portable.

Physical topology is resolved by:

- routing;
- deployment;
- scheduler;
- network runtime;
- HAL.

---

72. Timing Portability

Timing semantics MUST distinguish:

logical timing requirement

from:

physical clock period

A program MAY require:

deadline <= D
latency <= L
synchronization(...)

without requiring a specific physical clock frequency.

The backend determines how the requirement can be satisfied.

---

73. Power and Thermal Portability

A program MAY declare:

power_budget
thermal_constraint
energy_preference

These are resource/constraint semantics.

They MUST NOT assume a particular physical thermal-management implementation.

---

74. Reliability Portability

A program MAY specify reliability requirements.

For example:

requires reliability >= R

The target may satisfy that through:

- redundancy;
- QEC;
- replication;
- error correction;
- retry;
- hardware reliability;
- fault-tolerant scheduling.

The language requirement remains independent of the mechanism.

---

75. Observability

Tracing, profiling, logging, metrics, and provenance MUST NOT change the program's semantic result unless explicitly defined to do so.

Instrumentation SHOULD be portable at the semantic level.

Physical instrumentation details belong to tooling/runtime layers.

---

76. Provenance

Portable compilation MUST preserve provenance sufficient to relate:

source
→ AST
→ semantic model
→ IR
→ derived target artifact
→ execution

Provenance SHOULD identify:

- source version;
- language version;
- feature versions;
- dialects;
- compiler version;
- semantic configuration;
- target realization;
- resource/capability decisions.

Secrets MUST NOT be placed into ordinary provenance, diagnostics, or replay records.

---

77. Reproducibility

A portable build SHOULD be reproducible given the same declared:

- source;
- language version;
- dependencies;
- compiler configuration;
- semantic profile;
- relevant target-independent inputs.

Target-specific artifacts may differ while retaining semantic equivalence.

---

78. Portability Profiles

The implementation MAY provide profiles such as:

portable
reproducible
embedded
distributed
quantum
hdl
heterogeneous
secure
real-time

Profiles MUST refine the language contract.

They MUST NOT silently introduce arbitrary hardware limits.

---

79. Embedded Systems

Embedded execution MUST remain compatible with the same language semantics.

An embedded target MAY have fewer resources.

The program may fail due to:

insufficient resources

but the source language remains unchanged.

---

80. Edge and Cloud

The same program MAY be deployed across:

edge
local machine
cluster
cloud
hybrid edge/cloud

Deployment policy determines placement.

Portable semantics determine what the computation means.

---

81. Heterogeneous Execution

A single program MAY use multiple resource classes.

Example conceptual flow:

CPU
 ├── preprocessing
GPU
 ├── tensor computation
QPU
 ├── quantum kernel
CPU
 └── postprocessing

The semantic program remains unified.

The compiler/runtime may partition the workload.

---

82. Hybrid Quantum-Classical Portability

Hybrid programs MUST preserve explicit boundaries between:

classical computation
quantum computation
measurement
classical control
quantum re-entry

The physical host/device relationship is an implementation concern unless explicitly declared.

---

83. HDL/Software Portability

A co-designed program MAY describe:

algorithm
hardware intent
software implementation
communication
verification
deployment

The compiler MAY choose whether a component is:

- software;
- hardware;
- accelerator;
- hybrid.

Where semantics permit multiple implementations, the choice is an implementation decision.

---

84. Security Boundary

Target-specific security mechanisms MUST remain behind explicit capabilities.

For example:

requires capability("trusted_execution")

does not specify whether the implementation uses a particular vendor technology.

This allows future security technologies to satisfy the same semantic contract.

---

85. Prohibited Portability Violations

The following are prohibited in portable core semantics:

Fixed hardware maxima

MAX_QUBITS = ...
MAX_CORES = ...
MAX_THREADS = ...
MAX_GPUS = ...

Fixed topology

qubit 0 connects to qubit 1

as a universal assumption.

Fixed device identity

gpu = 0
qpu = 0

as implicit semantics.

Fixed memory

RAM = 64 GiB

as a universal assumption.

Fixed vector width

vector = 8 lanes

as a universal semantic limit.

Fixed tensor dimensions

tensor dimensions <= N

as a language limitation.

Fixed distributed scale

nodes <= N

as a language limitation.

Fixed concurrency

threads <= N

as a language limitation.

Fixed quantum gate universe

A closed gate enumeration MUST NOT be required to define all quantum computation.

---

86. Hard-Coding Audit

Every portability-sensitive file MUST be audited for:

MAX_
MIN_
DEFAULT_
FIXED_
LIMIT_
COUNT_
SIZE_
WIDTH_
DEPTH_
CAPACITY_
DEVICE_
CORE_
THREAD_
QUBIT_
NODE_
GPU_
FPGA_
QPU_

A match is not automatically an error.

The audit MUST determine whether the value is:

1. language semantics;
2. program data;
3. explicit user constraint;
4. implementation policy;
5. test fixture;
6. physical target property.

Only category 1 is prohibited when it represents an artificial universal hardware limitation.

---

87. Rust Implementation Requirements

The reference implementation MUST target:

Rust 1.97

or:

Rust 1.97.1

as the supported implementation baseline.

The implementation MUST use safe Rust.

"unsafe" MUST NOT be used in:

- lexer;
- parser;
- AST;
- semantic analyzer;
- grammar tooling;
- compiler;
- runtime;
- resource manager;
- scheduler;
- routing;
- quantum frontend;
- QEC orchestration;
- resilience;
- HAL integration.

Any dependency introducing unsafe implementation internally does not authorize Zamani source or first-party Zamani code to use "unsafe".

First-party source MUST NOT contain:

unsafe

blocks or unsafe escape mechanisms.

---

88. Rust Representation Versus Language Semantics

Rust implementation types are implementation details.

For example, the implementation may internally use:

Vec<T>
HashMap<K, V>
u64
u128
usize
BigInt-like representations

where appropriate.

Those choices MUST NOT silently become Zamani semantic limits.

In particular:

Rust usize

must not automatically define:

maximum Zamani program size

or:

maximum quantum register size

without an explicit semantic justification.

---

89. Lexer Integration

"src/lexer.rs" MUST implement the lexical contract established by:

grammar/specification/lexical.md
grammar/lexer/

Portability itself MUST NOT be encoded as arbitrary lexical restrictions.

Numeric and resource literals MUST be lexed without introducing artificial hardware limits.

---

90. Parser Integration

"src/parser.rs" MUST implement the syntax defined by:

grammar/Zamani.g4
grammar/specification/syntax.md

The parser MUST NOT make target-selection decisions.

For example, parsing:

requires qubits >= n

MUST produce syntax/AST information.

It MUST NOT inspect a physical QPU.

---

91. AST Integration

"src/ast/mod.rs" and "src/frontend/ast/" MUST represent portable source structure.

The AST MUST preserve:

- source identity;
- source spans;
- logical resources;
- requirements;
- capabilities;
- constraints;
- preferences;
- effects;
- domain constructs.

It MUST NOT prematurely resolve physical targets.

---

92. Semantic Analysis Integration

Semantic analysis MUST determine:

- what the program means;
- which resources it requires;
- which capabilities it requires;
- which constraints apply;
- whether ownership/type/effect rules are valid;
- whether portability rules are satisfied.

Semantic analysis MUST NOT depend on a particular target to determine ordinary program meaning.

---

93. Semantic Context

Semantic analysis MUST support nested contexts.

It MUST NOT rely on a single mutable global state such as:

current_return_type
in_loop

for the entire analysis.

Nested functions, loops, modules, effects, and domain contexts require properly scoped analysis environments.

This is required for correct portable semantics.

---

94. Symbol Scope

The symbol environment MUST be scoped by:

- module;
- lexical scope;
- function;
- block;
- generic context;
- pattern;
- domain context where applicable.

A global mutable symbol table MUST NOT leak state between independent programs or compilation units.

Portability requires compilation independence.

---

95. Unknown Types

An "Unknown" type MUST NOT allow production compilation to succeed silently.

An unknown type MAY exist temporarily in:

- IDE recovery;
- parser recovery;
- incomplete source analysis.

Production semantic compilation MUST resolve or reject it.

A target backend MUST NOT be asked to guess unresolved semantic meaning.

---

96. Builtins and Intrinsics

Builtins MUST have explicit semantic contracts.

Each builtin SHOULD define:

- name;
- parameters;
- result;
- effects;
- type behavior;
- resource requirements;
- capabilities;
- portability;
- diagnostics;
- IR mapping.

A builtin MUST NOT be implemented as an undocumented special case that bypasses portability semantics.

---

97. Expression Portability

Expression evaluation semantics MUST be target-independent.

This includes:

- arithmetic;
- comparisons;
- logical operators;
- indexing;
- calls;
- assignment;
- ranges;
- pattern matching;
- tensor operations;
- quantum expressions;
- effectful operations.

Optimization MAY change implementation but MUST preserve specified semantics.

---

98. Control-Flow Portability

Control-flow semantics MUST remain stable across targets.

The compiler MAY transform:

if
loop
match
async
parallel

into different implementation structures.

Such transformations MUST preserve observable semantics.

---

99. Parallel Portability

Parallelism MUST describe logical parallel work.

The runtime MAY execute logical parallelism using:

threads
processes
SIMD
GPU kernels
FPGA pipelines
distributed workers
QPU execution

The choice is target-dependent.

---

100. Scheduling Portability

Scheduling MUST be late-bound whenever possible.

The source program SHOULD describe:

ordering constraints
dependencies
deadlines
priorities
resource requirements

rather than physical schedule slots.

---

101. Optimization Portability

Optimization MUST be semantics-preserving.

Optimizations MAY include:

- constant folding;
- vectorization;
- fusion;
- decomposition;
- tiling;
- parallelization;
- caching;
- circuit optimization;
- gate cancellation;
- HDL optimization.

Optimization MUST NOT alter explicitly required behavior.

---

102. Resource-Aware Optimization

Optimization MAY use target information.

Example:

portable semantic program
        ↓
target capabilities
        ↓
optimized realization

This does not violate portability.

The target influences implementation, not the source program's meaning.

---

103. Target Specialization

Specialization MAY occur for:

- CPU features;
- GPU features;
- FPGA resources;
- QPU capabilities;
- distributed topology;
- memory capacity;
- available accelerators.

Specialization MUST preserve the source semantic identity.

---

104. Portable Artifact Identity

Derived artifacts SHOULD record:

source identity
semantic version
feature versions
compiler version
target information
capability assumptions
resource assumptions

This allows multiple target artifacts to be traced back to one source program.

---

105. Cross-Domain Portability

A program combining:

classical
quantum
HDL
AI
distributed
networking
security

MUST NOT become less portable merely because multiple domains are composed.

Each domain contributes:

- syntax;
- semantic rules;
- capabilities;
- resources;
- IR mappings.

The shared semantic model remains the integration boundary.

---

106. Domain Conflict Resolution

When domain rules conflict, resolution MUST occur through explicit semantic contracts.

Priority MUST NOT be determined by:

- grammar file order;
- parser rule order;
- backend preference;
- vendor preference.

The language specification defines semantic precedence.

---

107. Source-Level Target Requests

A programmer MAY explicitly request target behavior.

For example:

target family(...)
requires capability(...)
prefer accelerator(...)

Such declarations MUST be distinguished from the portable computation itself.

A target request that is impossible to satisfy MUST result in an explicit diagnostic.

---

108. Non-Portable Escape Hatches

Zamani MAY provide explicit non-portable mechanisms for:

- embedded systems;
- device programming;
- vendor APIs;
- low-level interoperability;
- physical hardware;
- special-purpose optimization.

They MUST be:

- explicit;
- typed;
- effect-aware;
- capability-aware;
- diagnostically visible;
- versioned.

They MUST NOT silently alter the meaning of portable code.

---

109. Portability Diagnostics

Portability diagnostics MUST include enough information to explain:

- what is non-portable;
- why it is non-portable;
- which capability/resource is required;
- whether the issue is compile-time or runtime;
- whether a portable alternative exists.

Example conceptual diagnostic:

PORTABILITY-E001

Target-specific physical qubit mapping was requested.

This construct requires a physical device mapping and is not
portable across QPU implementations.

Logical quantum operations remain portable.

Source: ...

Diagnostics MUST preserve source spans.

---

110. No Silent Degradation

An implementation MUST NOT silently:

- drop an operation;
- change numerical semantics;
- remove an effect;
- ignore a requirement;
- reduce a requested resource;
- change quantum measurement behavior;
- alter synchronization;
- remove a security property.

Any semantic degradation requires an explicit language-defined policy.

---

111. Portability and Error Handling

Portability failures MUST be represented distinctly from syntax errors.

At minimum, implementations SHOULD distinguish:

syntax error
type error
effect error
ownership error
resource error
capability error
portability error
target error
runtime resource exhaustion
interoperability error

---

112. Compatibility

Changing portability semantics is a language compatibility change.

Compatibility files MUST therefore track:

grammar/compatibility/
grammar/spec/compatibility.md
grammar/specification/

A syntax-compatible change can still be semantically breaking.

---

113. Versioning

Every portability-affecting semantic change MUST be versioned.

Versioning MUST cover:

- resource semantics;
- capability semantics;
- target classifications;
- deterministic behavior;
- fallback behavior;
- interoperability behavior;
- domain portability.

---

114. Deprecated Portability Behavior

Deprecated target-specific behavior MUST NOT remain silently supported forever.

Deprecation SHOULD provide:

- warning;
- migration guidance;
- replacement construct;
- compatibility period;
- removal version.

---

115. Testing Contract

Portability testing MUST cover the complete pipeline:

specification
↓
Zamani.g4
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

A grammar feature is not portable merely because the parser accepts it.

---

116. Required Portability Tests

Tests MUST include:

Positive

- portable programs;
- resource requirements;
- capability requirements;
- dynamic resource quantities;
- heterogeneous programs;
- quantum programs;
- HDL programs;
- distributed programs;
- AI programs.

Negative

- fixed physical IDs used as portable semantics;
- unsatisfied requirements;
- missing capabilities;
- illegal target assumptions;
- hidden effects;
- silent fallback attempts.

Boundary

- minimal resources;
- large resources;
- empty workloads;
- large symbolic dimensions;
- dynamic allocation;
- zero/one/many resource cases.

Scalability

- tiny workloads;
- large workloads;
- dynamically growing workloads;
- distributed workloads;
- heterogeneous workloads.

---

117. No Maximum-Size Tests

The test suite MUST NOT accidentally define a maximum language size.

Bad:

assert max_qubits == 1024;

Good:

assert semantic_requirement_is("n");

or:

assert program remains semantically valid for larger finite n;

Tests SHOULD verify that scaling is constrained by resources rather than parser constants.

---

118. Property Testing

Property-based tests SHOULD verify:

increasing a symbolic workload does not create
a language-level rejection solely because of size.

For example:

n = 1
n = 2
n = 1024
n = larger finite value

should follow the same semantic model.

Actual execution may fail if resources are insufficient.

---

119. Differential Portability Testing

Equivalent programs SHOULD be tested across multiple execution strategies.

For example:

scalar
vectorized
parallel
distributed
accelerated

must satisfy the same semantic contract where their capabilities permit execution.

---

120. Quantum Differential Testing

Quantum portability testing SHOULD compare logical semantics across:

different qubit counts
different connectivity
different gate sets
different routing strategies
different schedules

The physical realization may differ.

The logical semantic result must satisfy the same contract.

---

121. HDL Differential Testing

HDL semantics SHOULD be validated across:

simulation
synthesis
different implementation parameters

provided the implementation satisfies the declared semantic hardware contract.

---

122. Resource Exhaustion Tests

Tests MUST distinguish:

language invalidity

from:

resource exhaustion

For example:

program requires N resources
target provides fewer than N

must produce a resource/capability failure, not a grammar failure.

---

123. Determinism Tests

Portability tests MUST verify that:

same semantics
+
same declared deterministic inputs

remain deterministic across valid execution strategies.

---

124. Compatibility Tests

The compatibility suite MUST test:

- previous stable syntax;
- previous semantics;
- deprecated constructs;
- feature gates;
- dialect compatibility;
- AST compatibility;
- IR compatibility;
- compiler compatibility.

---

125. Hard-Coding Validation

"grammar/validation/hard-coding.md" and related tooling MUST inspect portability-sensitive grammar/specification files.

The validation SHOULD identify:

- fixed hardware maxima;
- fixed topology;
- physical IDs;
- accidental fixed widths;
- fixed resource counts;
- vendor assumptions.

Every finding MUST be classified before rejection.

---

126. Integration With Existing Grammar Files

"grammar/Zamani.g4"

Owns syntax composition.

It MUST NOT encode target capacity.

---

"grammar/Zamani-Grammar.md"

Retains historical/aspirational language design.

It MUST NOT silently override portability semantics.

---

"grammar/grammar.md"

Documents actual implementation conformance.

It MUST expose portability-related implementation gaps.

---

"grammar/DESIGN.md"

Defines the architectural separation between syntax, semantics, resources, capabilities, IR, and targets.

This document provides the detailed portability contract.

---

"grammar/specification/language.md"

Defines the overall Zamani language model.

It MUST reference this document for POCO-REAF and portability.

---

"grammar/specification/semantics.md"

Defines evaluation and meaning.

It MUST defer resource portability rules to this document while preserving semantic consistency.

---

"grammar/specification/lexical.md"

Defines lexical form.

It MUST NOT introduce hardware-specific lexical restrictions.

---

"grammar/specification/syntax.md"

Defines syntax.

It MUST NOT encode implementation limits as grammar alternatives.

---

"grammar/spec/type-system.md"

Defines type semantics.

It MUST distinguish mathematical/logical types from target representations.

---

"grammar/spec/resources.md"

Defines resource and capability contracts.

It MUST be consistent with the requirement/preference/constraint distinction in this document.

---

"grammar/spec/determinism.md"

Defines reproducibility and deterministic execution.

It MUST remain consistent with target-independent observable semantics.

---

"grammar/spec/diagnostics.md"

Defines structured diagnostics.

Portability failures MUST use the common diagnostic framework.

---

"grammar/compatibility/"

Tracks changes to portability behavior.

---

"grammar/validation/"

Validates:

- grammar portability;
- AST coverage;
- semantic coverage;
- IR coverage;
- hard-coding;
- scalability.

---

"grammar/tests/"

Provides positive, negative, boundary, scalability, portability, compatibility, and determinism tests.

---

127. Integration With the Rust Frontend

The frontend pipeline is:

source
 ↓
src/lexer.rs
 ↓
src/parser.rs
 ↓
src/frontend/ast/
 ↓
structural validation
 ↓
name resolution
 ↓
type analysis
 ↓
ownership/resource analysis
 ↓
effect analysis
 ↓
portability analysis
 ↓
canonical semantic model
 ↓
IR

Portability analysis MUST occur before target-specific lowering.

---

128. Integration With Quantum Frontend

The existing OpenQASM frontend under:

src/quantum/frontend/formats/openqasm/

is an interoperability frontend.

It MUST lower into the canonical Zamani quantum semantic representation and ultimately:

quantum::ir

It MUST NOT introduce:

- fixed H/X/CNOT-only semantics;
- fixed "q[0], q[1]" assumptions;
- automatic measurement of all qubits;
- physical-qubit assumptions;
- comment-based unsupported-operation handling.

---

129. Integration With Quantum Scheduling

Quantum scheduling MUST consume:

logical operations
dependencies
resource requirements
capabilities
timing constraints

and a target hardware context.

It MUST determine physical scheduling downstream.

The portability specification does not define physical scheduling algorithms.

---

130. Integration With Routing

Routing consumes logical operations and target topology/capabilities.

The portable source remains independent of the selected physical mapping.

---

131. Integration With QEC

QEC consumes quantum requirements and logical program information.

The language may express fault-tolerance intent.

The QEC subsystem determines the implementation strategy.

---

132. Integration With ZQN

ZQN consumes fault/noise semantics.

Portable source can state required noise/reliability properties.

ZQN maps those requirements to actual fault/noise behavior.

---

133. Integration With HAL

HAL provides actual target facts.

Examples:

available qubits
supported operations
memory
accelerators
connectivity
timing
calibration
health

These are environment facts, not universal language constants.

---

134. Integration With Resilience

The resilience subsystem may adapt execution according to:

Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

and appropriate recovery policies.

Those states MUST NOT alter the source program's semantic identity.

---

135. Integration With Runtime

The runtime MUST resolve late-bound information such as:

- actual resources;
- actual capabilities;
- current device state;
- placement;
- scheduling;
- dynamic workload size.

Runtime adaptation MUST be observable only where the language semantics explicitly permit it.

---

136. Integration With Tooling

Tooling SHOULD expose:

- portability classification;
- required resources;
- required capabilities;
- target assumptions;
- non-portable boundaries;
- fallback policies;
- scalability characteristics.

IDE tooling MAY warn about portability risks before compilation.

---

137. Integration With Feature Manifests

Every production feature manifest under:

grammar/specification/features/

MUST specify:

portability_class
resource_requirements
capabilities
target_dependencies
fallback_policy
hard_coding_policy
scalability_model

This ensures every feature has a complete portability contract.

---

138. Independent File Completion Contract

A file implementing a portability-sensitive feature is complete only when it defines, directly or by stable reference:

File
Purpose
Status
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Public Grammar Contract
AST Contract
Semantic Contract
IR Integration
Compiler Integration
Runtime Integration
Tooling Integration
Cross-Domain Integration
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility Tests
Determinism Tests
Portability Tests
Diagnostics
Security
Performance
Hard-Coding Audit
Completion Criteria

This contract prevents later files from requiring undocumented semantic changes.

---

139. Feature Completion Rule

A portability-related feature MUST NOT be considered production-ready until:

- syntax is defined;
- AST representation is defined;
- semantics are defined;
- resource semantics are defined;
- capability semantics are defined;
- portability class is defined;
- IR mapping is defined;
- compiler integration is defined;
- runtime integration is defined;
- diagnostics are defined;
- negative behavior is defined;
- boundary behavior is defined;
- scalability behavior is defined;
- compatibility behavior is defined;
- hard-coding audit passes.

---

140. Portability Invariants

The following invariants are mandatory.

Invariant 1

No artificial hardware maximum is part of core Zamani semantics.

Invariant 2

Logical resources are distinct from physical resources.

Invariant 3

Requirements are distinct from preferences.

Invariant 4

Capabilities are distinct from resource quantities.

Invariant 5

Implementation decisions are downstream from semantics.

Invariant 6

Target-specific lowering MUST NOT require source rewriting.

Invariant 7

Quantum semantics terminate at the existing "quantum::ir" boundary.

Invariant 8

Physical routing is not grammar semantics.

Invariant 9

Scheduling is not grammar semantics.

Invariant 10

QEC is not grammar semantics.

Invariant 11

ZQN is not grammar semantics.

Invariant 12

HAL is not grammar semantics.

Invariant 13

Runtime resource exhaustion is not syntax invalidity.

Invariant 14

Unknown semantic meaning cannot silently compile in production.

Invariant 15

First-party Zamani implementation code uses safe Rust only.

---

141. Formal Portability Model

For a program "P", environment "E", resource set "R", capability set "C", and implementation "I":

Semantics(P)

defines the program's meaning.

Execution is valid when:

Requirements(P) ⊆ Resources(E)

and:

Capabilities(P) ⊆ Capabilities(E)

and:

Constraints(P, E, I) are satisfied

and:

Effects(P)

are permitted.

The implementation then chooses:

Realization(P, E, I)

such that:

ObservableSemantics(Realization(P, E, I))
=
Semantics(P)

for all guarantees declared by "P".

This is the core mathematical basis of POCO-REAF.

---

142. Scaling Model

For workload parameter "n":

P(n)

defines a family of valid program instances.

Zamani MUST permit:

n₁ < n₂ < n₃ < ...

without imposing an artificial language maximum.

Execution succeeds when:

Resources(E) >= Requirements(P(n))

and fails explicitly otherwise.

Therefore:

language capacity

is not equivalent to:

current machine capacity

---

143. Semantic Portability Versus Performance Portability

A program is semantically portable when its meaning can be preserved across targets.

It is performance-portable when acceptable performance can also be achieved.

Zamani guarantees the former architecturally.

The latter depends on:

- target capabilities;
- optimization;
- scheduling;
- resource availability;
- algorithmic structure;
- implementation quality.

The language MUST NOT falsify performance portability by pretending all targets are equivalent.

---

144. Cost-Aware Portability

The implementation MAY expose target costs.

Examples:

latency
energy
memory
communication
financial cost
quantum error cost

These MAY influence preferences and optimization.

They MUST NOT alter required semantics.

---

145. Adaptation Without Semantic Drift

Adaptation is permitted:

more resources
→ more parallelism

fewer resources
→ less parallelism

different topology
→ different routing

different gate set
→ different decomposition

different accelerator
→ different lowering

provided:

program semantics remain satisfied

---

146. Portability and Optimization Failure

If no valid optimization can satisfy the target constraints, the implementation MUST report failure.

It MUST NOT:

- silently violate correctness;
- silently remove requirements;
- silently change effects;
- silently map unsupported operations.

---

147. Portability and Partial Compilation

A compiler MAY produce a partially lowered artifact for tooling or analysis.

Such an artifact MUST be marked incomplete.

Incomplete artifacts MUST NOT be presented as executable production artifacts.

---

148. Portability and Incremental Compilation

Incremental compilation MUST preserve semantic identity.

Changing an implementation detail of one target MUST NOT require reparsing or rewriting unrelated portable source unless semantic dependencies actually changed.

---

149. Portability and Caching

Cached compilation artifacts MUST include sufficient identity information to prevent reuse under incompatible semantic assumptions.

Caches MUST NOT confuse:

same source

with:

same target realization

unless target-independent artifact identity is explicitly defined.

---

150. Portability and Deployment

Deployment systems MUST consume:

- resource requirements;
- capability requirements;
- constraints;
- preferences;
- security requirements;
- provenance.

Deployment MUST determine physical placement.

Portable source SHOULD NOT encode deployment topology unnecessarily.

---

151. Portability and Verification

Verification properties MUST survive lowering.

For example:

assert invariant

must remain associated with the relevant semantic computation even after:

- optimization;
- vectorization;
- distribution;
- quantum decomposition;
- HDL synthesis.

---

152. Portability and Formal Verification

Where formal specifications are present, target-specific optimization MUST preserve the verified properties.

The compiler SHOULD retain proof/provenance relationships where supported.

---

153. Portability and Future Backends

A new backend SHOULD require implementation of:

capability discovery
resource discovery
IR lowering
target realization
runtime integration

rather than modification of portable source.

This is a primary extensibility requirement.

---

154. Production Acceptance Criteria

"grammar/specification/portability.md" and the corresponding implementation are production-ready only when:

- POCO-REAF is explicitly defined;
- target independence is defined;
- resource/capability semantics are defined;
- requirements/constraints/preferences/hints are separated;
- logical/physical resources are separated;
- no artificial hardware maxima exist;
- quantum portability is defined;
- classical portability is defined;
- HDL portability is defined;
- hybrid portability is defined;
- distributed portability is defined;
- AI/data portability is defined;
- networking/security portability is defined;
- interoperability boundaries are defined;
- compiler responsibilities are defined;
- runtime responsibilities are defined;
- HAL responsibilities are defined;
- routing responsibilities are defined;
- scheduling responsibilities are defined;
- QEC responsibilities are defined;
- ZQN responsibilities are defined;
- resilience responsibilities are defined;
- AST boundaries are defined;
- canonical IR integration is defined;
- "quantum::ir" remains canonical;
- diagnostics are defined;
- provenance is defined;
- determinism is defined;
- compatibility is defined;
- hard-coding audits exist;
- scalability tests exist;
- portability tests exist;
- Rust 1.97/1.97.1 compatibility is enforced;
- first-party Rust uses no "unsafe".

---

155. Final Normative Statement

Zamani portability is founded on one principle:

«The programmer describes computation and its semantic requirements once; the implementation determines how that computation is realized on the resources and capabilities actually available.»

Therefore:

Zamani Source
      ↓
Portable Semantics
      ↓
Canonical Semantic Model
      ↓
Canonical IR
      ↓
Target-Aware Compilation
      ↓
Resource/Capability Negotiation
      ↓
Optimization
      ↓
Routing
      ↓
Scheduling
      ↓
QEC / Resilience / ZQN
      ↓
HAL
      ↓
Actual Execution

The source program is the stable semantic artifact.

Hardware is not.

Physical topology is not.

Device identity is not.

Resource quantity is not.

Compiler strategy is not.

Scheduler strategy is not.

Routing is not.

Calibration is not.

Backend instruction selection is not.

Those are implementation realities resolved after portable semantics have been established.

Accordingly, Zamani MUST allow the same semantic program to scale from the smallest meaningful execution to the largest execution supported by the resources actually available, without introducing artificial language-level hardware limits.

The only universal limits are the limits required by the language's own semantic model and the mathematical/physical reality of a particular execution—not arbitrary constants embedded in the grammar or compiler.

This is the normative portability foundation for:

«Program Once → Compile Once → Run Everywhere → Anywhere → Forever.»