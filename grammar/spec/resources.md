Zamani Resource Semantic Specification

Path: "grammar/spec/resources.md"
Language: Zamani
Specification role: Normative resource semantic contract
Specification version: 3.0
Status: Production target / normative
Grammar technology: ANTLR4
Implementation baseline: Rust 1.97 / Rust 1.97.1, Rust 2021
Safety requirement: Safe Rust only; "unsafe" Rust is prohibited
Primary architecture: Target-independent, resource-parametric, capability-driven computation
Scalability principle: No artificial language-level resource ceiling
Portability principle: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

0. Purpose

This document defines the normative semantic contract for resource intent in Zamani.

It specifies how Zamani represents:

- resources;
- resource kinds;
- resource identities;
- resource quantities;
- requirements;
- constraints;
- capabilities;
- preferences;
- hints;
- targets;
- capacities;
- availability;
- performance;
- latency;
- throughput;
- bandwidth;
- energy;
- power;
- reliability;
- resilience;
- cost;
- reservation;
- acquisition;
- release;
- derivation;
- grouping;
- resource properties;
- resource contracts;
- resource profiles;
- scaling intent;
- portability intent;
- resource negotiation.

The purpose is to allow a Zamani program to express what computation requires or permits without embedding the implementation characteristics of a particular machine into the source-language semantics.

The fundamental rule is:

«Resource intent is semantic intent. Resource realization is an implementation decision.»

A Zamani source program therefore describes a computation and its resource requirements independently of whether it is ultimately realized by:

- one CPU;
- many CPUs;
- one GPU;
- many GPUs;
- an FPGA;
- an ASIC;
- a QPU;
- multiple QPUs;
- a simulator;
- an accelerator;
- an embedded system;
- a workstation;
- a cluster;
- a supercomputer;
- a distributed system;
- a cloud environment;
- a heterogeneous system;
- a future computing architecture.

The language must not require source rewriting merely because the available realization changes.

---

1. Document Authority

This document is authoritative for resource semantics.

It does not replace:

- "grammar/spec/lexical.md";
- "grammar/spec/syntax.md";
- "grammar/spec/type-system.md";
- "grammar/spec/semantics.md";
- "grammar/spec/effects.md";
- "grammar/spec/compatibility.md";
- "grammar/Zamani.g4";
- "grammar/resources/*.g4".

Instead, it defines the semantic meaning that those syntactic contracts must implement.

The authority relationship is:

grammar/specification/language.md
        |
        +-- language model
        |
grammar/spec/lexical.md
        |
        +-- lexical meaning
        |
grammar/spec/syntax.md
        |
        +-- syntactic structure
        |
grammar/resources/*.g4
        |
        +-- resource syntax
        |
grammar/Zamani.g4
        |
        +-- canonical grammar composition
        |
src/lexer.rs
        |
src/parser.rs
        |
src/frontend/ast/
        |
        +-- structural representation
        |
        v
semantic analysis
        |
        +-- name resolution
        +-- type analysis
        +-- effect analysis
        +-- resource analysis
        +-- capability analysis
        +-- constraint solving
        |
        v
canonical semantic representation
        |
        +-- classical semantics
        +-- quantum semantics
        +-- HDL semantics
        +-- hybrid semantics
        +-- AI/data semantics
        +-- distributed semantics
        +-- other domains
        |
        v
canonical IR/domain IR
        |
        +-- classical IR
        +-- quantum::ir
        +-- HDL/hardware IR
        |
        v
verification
        |
        v
optimization
        |
        +-- routing
        +-- scheduling
        +-- resilience
        +-- QEC
        +-- ZQN
        |
        v
HAL / target realization
        |
        v
runtime / deployment

No implementation layer may silently redefine the semantics specified here.

---

2. Ownership

2.1 This file owns

This file owns the semantic meaning of:

1. resource intent;
2. resource requirements;
3. resource constraints;
4. resource capabilities;
5. resource preferences;
6. resource hints;
7. resource quantities;
8. resource relationships;
9. resource feasibility;
10. resource satisfiability;
11. resource scalability;
12. resource portability;
13. resource negotiation;
14. resource realization boundaries;
15. resource lifecycle semantics;
16. resource property semantics;
17. resource profiles;
18. resource contracts;
19. resource composition;
20. resource-dependent diagnostics;
21. resource-related determinism;
22. resource-related failure semantics;
23. target-independent resource semantics.

2.2 This file does not own

This file does not own:

- token definitions;
- lexer implementation;
- parser implementation;
- general expression syntax;
- general type syntax;
- AST implementation;
- symbol resolution implementation;
- hardware discovery;
- physical device discovery;
- physical allocation algorithms;
- placement algorithms;
- routing algorithms;
- scheduling algorithms;
- optimization algorithms;
- QEC algorithms;
- ZQN implementation;
- HAL implementation;
- runtime implementation;
- vendor APIs;
- compiler backend implementation;
- device drivers;
- calibration;
- topology discovery;
- cloud-provider APIs;
- physical addresses;
- canonical quantum IR;
- classical IR;
- HDL IR.

The resource specification defines contracts for those systems to consume.

---

3. Integration Contract

The resource subsystem integrates with the repository as follows.

3.1 Grammar integration

The authoritative source grammar is:

grammar/Zamani.g4

Resource-specific grammar is distributed across:

grammar/resources/

including the existing:

resources.g4
requirements.g4
constraints.g4
preferences.g4
capabilities.g4
resource-expressions.g4
scalability.g4
portability.g4
performance.g4
latency.g4
energy.g4
reliability.g4

Those grammar files MUST implement the semantic categories defined here.

They MUST NOT introduce conflicting resource semantics.

3.2 Lexer integration

Resource keywords belong to the canonical lexer vocabulary.

Resource grammar files MUST NOT create independent lexical definitions.

Canonical lexical ownership remains with:

grammar/spec/lexical.md
src/lexer.rs

3.3 Parser integration

The parser produces syntax.

It does not:

- inspect hardware;
- query available resources;
- choose a device;
- allocate memory;
- select a QPU;
- perform scheduling;
- perform routing.

3.4 AST integration

Every resource construct MUST have a predetermined AST representation before its grammar is considered complete.

The AST representation MUST preserve:

- source span;
- resource intent kind;
- resource expression;
- resource identity;
- namespace;
- attributes;
- modifiers;
- ordering where semantically observable;
- nested resource expressions;
- domain information;
- requirement/constraint/preference/hint distinction.

Resource syntax MUST NOT require a hardware-specific AST.

3.5 Semantic integration

After parsing, resource constructs are analyzed alongside:

- names;
- types;
- effects;
- ownership;
- capabilities;
- domain semantics;
- compilation constraints.

3.6 IR integration

Resource intent MUST lower into semantic resource metadata/requirements associated with the canonical semantic representation.

Resource syntax MUST NOT become a competing IR.

For quantum computation:

resource intent
      |
      v
semantic resource requirements
      |
      v
quantum semantic model
      |
      v
quantum::ir

"quantum::ir" remains the canonical quantum semantic boundary.

3.7 Compiler integration

The compiler consumes resource requirements and capabilities when determining valid target realizations.

It MAY use:

- specialization;
- decomposition;
- parallelization;
- placement;
- routing;
- scheduling;
- memory planning;
- accelerator selection;
- distributed execution.

Those are implementation decisions.

3.8 Runtime integration

The runtime MAY re-evaluate dynamic resource conditions.

Examples include:

- dynamic availability;
- resource contention;
- failures;
- reservations;
- acquisition;
- release;
- resilience;
- dynamic scaling.

Runtime behavior MUST preserve the semantic distinction between requirements and preferences.

---

4. Fundamental Model

A Zamani resource declaration describes an abstract resource or resource property.

A useful conceptual model is:

ResourceIntent =
    identity
  + kind
  + quantity
  + properties
  + requirements
  + constraints
  + capabilities
  + preferences
  + hints
  + lifecycle
  + provenance

A realization is:

ResourceRealization =
    intent
  + environment
  + available_resources
  + available_capabilities
  + target_constraints
  + compiler_policy
  + runtime_policy

The realization MUST preserve the source program's semantic meaning.

Therefore:

resource intent != physical realization

and:

resource identity != physical device identity

unless the program explicitly enters a target-specific mechanism.

---

5. POCO-REAF

Zamani resource semantics are a foundational part of:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

POCO-REAF requires that resource intent remain stable while realization changes.

For example, a program may state:

requires capability("tensor.compute");

without specifying:

GPU 0
CUDA
vendor X
device Y

Likewise:

requires memory >= workload_memory;

does not specify:

RAM bank 3
VRAM device 0
address 0x...

Likewise:

requires capability("quantum.measurement");

does not specify:

QPU vendor X
physical qubit 17

The realization pipeline is:

portable source
      |
      v
resource intent
      |
      v
semantic analysis
      |
      v
capability/resource negotiation
      |
      v
target-independent optimization
      |
      v
target realization

A target that cannot satisfy a mandatory requirement MUST reject the realization or report a defined resource failure.

It MUST NOT silently change the program's meaning.

---

6. Semantic Scalability

6.1 No artificial language limit

Zamani resource semantics MUST NOT impose fixed limits on:

- number of resources;
- number of devices;
- number of CPUs;
- number of cores;
- number of threads;
- number of GPUs;
- number of FPGAs;
- number of ASICs;
- number of QPUs;
- number of qubits;
- number of nodes;
- amount of memory;
- amount of storage;
- number of accelerators;
- number of resource groups;
- resource hierarchy depth;
- number of properties;
- number of resource constraints;
- number of resource requirements;
- number of processes;
- number of tasks;
- number of timelines;
- tensor dimensions;
- vector widths;
- register counts.

No equivalent indirect limitation is permitted.

6.2 Infinity

"Infinity" in this specification means:

«unbounded by the language architecture.»

It does not claim physically infinite resources.

The actual execution scale is limited by:

available resources
+
available capabilities
+
target constraints
+
compiler resources
+
runtime resources

The language itself must not establish the upper bound.

6.3 Small-to-large scaling

The same semantic program model MUST be able to represent:

one operation
      |
      v
small workload
      |
      v
large workload
      |
      v
parallel workload
      |
      v
heterogeneous workload
      |
      v
distributed workload
      |
      v
very large workload

without requiring a new source language for each scale.

---

7. Resource Intent Categories

Zamani MUST distinguish six fundamental categories.

Category| Meaning| Mandatory?| May affect correctness?
Requirement| Must be satisfied| Yes| Yes
Constraint| Realization must obey condition| Yes| Yes
Capability| Ability/property required or available| Context-dependent| Yes when required
Preference| Desired realization property| No| No
Hint| Optimization guidance| No| No
Implementation decision| Downstream realization choice| N/A| Only after semantic preservation

These categories MUST NOT be collapsed.

---

8. Requirements

A requirement states a condition that a valid realization MUST satisfy.

Conceptually:

requires resource memory >= workload_memory;

means:

«A valid realization requires at least the semantically specified memory capacity.»

A failed requirement MUST NOT silently become:

- a preference;
- a hint;
- an ignored annotation;
- an optimization opportunity;
- a different semantic computation.

A requirement MAY be:

- static;
- symbolic;
- dependent on program input;
- dependent on another resource;
- dynamically evaluated;
- conditional;
- domain-specific.

---

9. Constraints

A constraint restricts valid realizations.

Examples:

constraint resource latency <= latency_budget;

constraint resource energy <= energy_budget;

constraint resource reliability >= required_reliability;

Constraints are semantic.

A backend MUST NOT violate a hard constraint merely because another realization is easier to implement.

If a constraint cannot be satisfied, the implementation MUST produce a defined failure/diagnostic.

---

10. Capabilities

A capability describes an ability of a realization.

Examples:

quantum.measurement
quantum.dynamic_control
quantum.error_correction
tensor.compute
distributed.execution
persistent.storage
reconfigurable.hardware
high_precision.arithmetic

Capability names are extensible.

The grammar MUST NOT permanently enumerate all future capabilities.

Capability discovery belongs downstream.

A source-level declaration such as:

requires capability("quantum.measurement");

does not perform capability discovery.

---

11. Preferences

Preferences are advisory.

Example:

prefer resource low_latency;

A backend SHOULD attempt to honor the preference.

It MAY ignore it when required to preserve:

- correctness;
- resource feasibility;
- portability;
- determinism;
- security;
- reliability;
- other mandatory constraints.

A preference MUST NOT become a hidden requirement.

---

12. Hints

Hints are weaker than preferences.

A hint may provide optimization guidance.

Example:

hint resource locality;

A backend MAY completely ignore a hint.

Ignoring a hint MUST NOT make an otherwise valid program semantically invalid.

---

13. Implementation Decisions

Implementation decisions belong downstream.

For example:

logical resource
      |
      v
compiler analysis
      |
      v
target selection
      |
      v
physical allocation

A source-level abstract quantum resource:

logical_qubit q;

must not implicitly mean:

physical_qubit(17)

Physical mapping belongs to routing/HAL/backend layers.

Likewise:

resource memory;

must not implicitly mean:

address = 0x80000000;

---

14. Resource Identity

A resource identity is normally symbolic.

Examples:

resource compute;
resource memory;
resource quantum;
resource accelerator;

A symbolic identity does not automatically identify a physical object.

Resource identity may contain:

- local names;
- qualified names;
- namespaces;
- dialect namespaces;
- domain namespaces;
- semantic property paths.

Examples:

quantum
quantum.logical_qubit
quantum.logical_qubit.capacity
hardware.accelerator
network.bandwidth
storage.capacity

The namespace mechanism MUST remain extensible.

---

15. Physical Identity

Physical identity MUST remain separate from abstract resource identity.

Examples of physical identity include:

- device IDs;
- physical qubit IDs;
- memory addresses;
- PCI identifiers;
- machine identifiers;
- node identifiers;
- vendor-specific resource identifiers.

Such identities MAY occur in explicitly target-specific source constructs or deployment specifications.

They MUST NOT become implicit requirements of the universal resource model.

---

16. Resource Kinds

Resource kinds identify semantic categories.

Common examples include:

compute
memory
storage
network
interconnect
accelerator
quantum
quantum.logical_qubit
quantum.physical_qubit
tensor
energy
power
bandwidth

The list is not closed.

Future kinds MUST be representable without modifying the fundamental resource model.

A new resource kind SHOULD be namespaced when necessary:

domain::resource

or:

vendor::domain::resource

A domain extension MUST normalize into the canonical resource semantic model.

---

17. Resource Quantities

Resource quantities are expressions.

They MUST NOT be restricted to a fixed set of literals.

Examples:

quantity = n;

quantity = workload.size * element.size;

quantity = logical_qubits + ancilla_qubits;

quantity = required_parallelism;

quantity = input.count;

quantity = available_memory - reserved_memory;

Resource quantity expressions MAY be:

- literal;
- symbolic;
- computed;
- generic;
- dependent;
- runtime-derived.

The grammar must not decide the maximum representable resource quantity.

---

18. Quantity Representation

The source language MUST distinguish semantic quantity from implementation representation.

For example:

memory >= required_memory

does not imply:

required_memory : u64

The implementation MAY use an integer, arbitrary-precision representation, symbolic representation, interval representation, or another safe representation.

The selected representation must preserve the language semantics.

If an implementation cannot represent a valid source quantity, it MUST report an explicit diagnostic.

It MUST NOT silently wrap, truncate, clamp, or reinterpret the quantity.

---

19. Units and Dimensions

Resource quantities MAY carry units or dimensional information.

Examples include:

- bytes;
- bits;
- operations;
- events;
- seconds;
- cycles;
- joules;
- watts;
- bytes/second;
- operations/second;
- bits/second.

Unit semantics belong to the type/semantic system.

Resource grammar MUST reuse canonical quantity/unit syntax rather than inventing incompatible numeric systems.

Unit conversion MUST preserve semantic correctness.

Implicit conversions that can lose correctness SHOULD be rejected.

---

20. Capacity

Capacity describes how much of a resource is available or potentially available.

Examples:

capacity = available_memory;

capacity = resource.capacity;

Capacity is contextual.

A source program MUST NOT assume that a capacity expression defines a permanent machine limit.

---

21. Availability

Availability describes whether a resource can currently be used.

Availability MAY vary because of:

- resource contention;
- scheduling;
- failures;
- maintenance;
- reservations;
- dynamic allocation;
- distributed state;
- power conditions;
- thermal conditions;
- hardware health;
- runtime policy.

Availability is therefore generally dynamic.

The grammar only represents the source expression.

The runtime/resource manager evaluates the actual environment.

---

22. Performance

Performance is contextual.

Resource semantics MAY represent:

- required performance;
- preferred performance;
- measured performance;
- estimated performance;
- performance bounds;
- performance objectives.

Performance must not be reduced to a single universal machine metric.

Examples:

requires performance >= required_performance;

prefer performance;

The interpretation belongs to the relevant resource/semantic domain.

---

23. Latency

Latency represents elapsed time between semantically relevant events.

Latency may apply to:

- computation;
- memory;
- communication;
- storage;
- accelerator invocation;
- quantum operations;
- distributed operations.

No universal maximum or minimum latency is defined by the language.

A program may specify its own semantic constraint.

---

24. Throughput

Throughput represents work or data processed per semantic unit of time.

Examples:

throughput >= required_throughput

The actual unit and measurement semantics are determined by the associated resource/property type.

No fixed universal throughput constant exists.

---

25. Bandwidth

Bandwidth may refer to:

- network bandwidth;
- memory bandwidth;
- interconnect bandwidth;
- storage bandwidth;
- accelerator bandwidth;
- communication bandwidth.

The resource property must identify the relevant semantic context.

---

26. Energy

Energy represents resource consumption or an energy-related constraint.

Examples:

energy <= energy_budget

Energy semantics must remain independent of:

- processor voltage;
- clock frequency;
- vendor;
- instruction set;
- physical implementation.

Physical measurement is downstream.

---

27. Power

Power represents an energy-rate property.

Examples:

power <= power_budget

Power constraints MAY influence:

- target selection;
- scheduling;
- placement;
- accelerator selection;
- runtime management.

They MUST NOT force a particular hardware implementation unless explicitly specified by a target-specific contract.

---

28. Reliability

Reliability represents a required or desired probability, guarantee, bound, or other formally defined reliability property.

Examples:

reliability >= required_reliability

The resource grammar does not determine how reliability is achieved.

It does not implement:

- fault detection;
- error correction;
- recovery;
- redundancy;
- retries;
- diagnosis.

Those belong to their respective systems.

---

29. Resilience

Resource-level resilience intent may describe the desired ability of a computation to continue under resource degradation or failure.

Examples of semantic intent include:

- tolerate resource failure;
- recover from resource loss;
- maintain required reliability;
- permit resource replacement;
- permit migration;
- preserve checkpoints.

The resource layer does not implement recovery.

The conceptual integration is:

resource intent
      |
      v
semantic resilience requirement
      |
      v
resilience subsystem
      |
      +-- detection
      +-- diagnosis
      +-- policy
      +-- recovery
      +-- migration
      +-- verification

The existing resilience architecture remains responsible for actual self-healing orchestration.

---

30. Quantum Resource Semantics

Quantum resource intent MUST remain target-independent.

Examples include:

quantum
quantum.logical_qubit
quantum.physical_qubit
quantum.measurement
quantum.dynamic_control
quantum.error_correction
quantum.coherence

The grammar may describe:

- logical qubit requirements;
- physical realization requirements;
- measurement capabilities;
- control capabilities;
- coherence requirements;
- reliability requirements;
- error-correction requirements;
- resource quantities.

It MUST NOT decide:

- physical qubit numbering;
- coupling topology;
- gate decomposition;
- pulse scheduling;
- calibration;
- physical routing.

The pipeline remains:

Zamani quantum source
        |
        v
resource intent
        |
        v
semantic analysis
        |
        v
quantum semantic model
        |
        v
quantum::ir
        |
        v
optimization
        |
        v
routing
        |
        v
scheduling
        |
        v
QEC / resilience / ZQN
        |
        v
HAL
        |
        v
physical realization

This prevents resource grammar from creating a second quantum IR.

---

31. Classical Resource Semantics

Classical resource intent may describe requirements involving:

- compute capacity;
- memory;
- storage;
- parallelism;
- numerical precision;
- vectorization;
- accelerator capabilities;
- communication;
- energy;
- latency;
- throughput.

It must not assume:

x86
ARM
RISC-V
fixed register count
fixed vector width
fixed core count
fixed cache size

unless explicitly expressed through a target-specific contract.

---

32. GPU and Accelerator Resources

GPU/accelerator requirements MUST be capability-based.

Prefer:

requires capability("tensor.compute");

over:

requires GPU0;

A program MAY express semantic requirements for:

- tensor computation;
- matrix acceleration;
- parallel execution;
- specialized arithmetic;
- accelerator memory;
- accelerator communication.

The compiler determines whether:

- CPU;
- GPU;
- FPGA;
- ASIC;
- custom accelerator;
- future accelerator

can provide a valid realization.

---

33. FPGA and Reconfigurable Resources

FPGA resource intent may describe:

- reconfigurability;
- logic capacity;
- memory requirements;
- pipeline requirements;
- interface capabilities;
- timing requirements;
- accelerator requirements.

The source language must not establish a universal:

LUT_COUNT
BRAM_COUNT
DSP_COUNT

or equivalent ceiling.

Actual FPGA capacity belongs to target discovery.

---

34. HDL Resource Integration

HDL resource declarations MUST integrate with the universal resource model.

Hardware intent may describe:

resource compute;
resource memory;
resource interconnect;
resource timing;

while HDL semantics describe actual hardware structure.

The distinction is:

resource requirement
        !=
hardware implementation

A hardware description MAY eventually become an implementation decision.

The universal resource model must remain usable outside HDL.

---

35. Distributed Resources

Distributed resource semantics must not assume a fixed number of nodes.

Valid source intent includes concepts such as:

- distributed execution;
- replication;
- partitioning;
- communication;
- locality;
- consistency;
- availability;
- fault tolerance;
- network bandwidth;
- collective computation.

The number of nodes is determined by realization.

A source program must not need to change merely because execution changes from:

1 node

to:

N nodes

provided its semantic requirements can be satisfied.

---

36. Concurrency Resources

Concurrency intent may describe:

- required parallelism;
- preferred parallelism;
- concurrency capacity;
- task resources;
- synchronization requirements.

The language must not hard-code:

MAX_THREADS
MAX_TASKS
MAX_CORES

Concurrency scaling is a resource realization problem.

---

37. Memory Resources

Memory resources include semantic categories such as:

- working memory;
- persistent memory;
- distributed memory;
- accelerator memory;
- quantum memory where supported;
- shared memory;
- private memory.

The source language must not hard-code:

64 GB
128 GB
24 GB

as universal limits.

A program may specify an actual semantic requirement:

memory >= required_memory

without the language deciding where the memory comes from.

---

38. Storage Resources

Storage intent may describe:

- required capacity;
- persistence;
- durability;
- bandwidth;
- latency;
- availability;
- consistency;
- locality.

The language must not assume a particular:

- disk;
- filesystem;
- storage vendor;
- cloud service;
- block size;
- device count.

---

39. Network Resources

Network resource intent may describe:

- bandwidth;
- latency;
- availability;
- communication capability;
- topology requirements;
- protocol capabilities;
- service reachability.

A source-level abstract network resource does not imply a physical interface or address.

Physical networking is downstream.

---

40. Resource Topology

Topology is target-dependent unless explicitly made part of program semantics.

A program may require an abstract topology property when its correctness genuinely depends on it.

Otherwise topology should remain an implementation freedom.

Examples:

prefer locality;

is different from:

requires topology == required_topology;

The first is advisory.

The second is semantic.

Physical topology discovery belongs downstream.

---

41. Resource Relationships

Resources may have relationships.

Examples:

memory associated_with compute

accelerator attached_to interconnect

quantum.logical_qubit supported_by quantum.device

Relationships must be represented semantically rather than through physical IDs.

---

42. Resource Groups

A resource group represents a logical collection of resources.

A group may contain an arbitrary number of members.

No fixed group cardinality is permitted.

Groups may represent:

- compute pools;
- memory pools;
- accelerator pools;
- quantum resources;
- distributed resources;
- heterogeneous resources.

Grouping must not force a physical allocation.

---

43. Resource Profiles

A resource profile is a reusable description of resource intent.

A profile may contain:

- requirements;
- constraints;
- capabilities;
- preferences;
- hints;
- properties;
- scaling information;
- portability information.

Profiles may be reused across:

- functions;
- modules;
- programs;
- domains;
- deployment contexts.

A profile must remain declarative.

---

44. Resource Contracts

A resource contract defines conditions under which a computation is valid.

A contract may specify:

requirements
constraints
capabilities
failure conditions
resource lifecycle

A contract is not a backend configuration file.

Contracts must be composable.

---

45. Resource Composition

Resource requirements from different scopes may be combined.

Conceptually:

program requirement
        +
module requirement
        +
function requirement
        +
operation requirement
        =
effective requirement

Composition MUST preserve category semantics.

For example:

requirement + preference

must not turn the preference into a requirement.

Likewise:

requirement + requirement

must preserve both requirements.

Conflicting requirements must produce a diagnostic rather than an arbitrary choice.

---

46. Conditional Requirements

Requirements MAY depend on semantic conditions.

For example:

if workload_is_quantum
    requires capability("quantum.measurement");

The condition is evaluated according to the normal semantic model.

The resource subsystem must not invent a second conditional-expression language.

---

47. Dynamic Requirements

Some resource requirements cannot be known until execution.

Examples:

- input-dependent memory;
- runtime-generated workloads;
- adaptive quantum workloads;
- distributed scaling;
- dynamic data;
- runtime accelerator availability.

Such requirements may be represented symbolically.

The runtime MUST evaluate them using the defined semantic mechanism.

---

48. Resource Negotiation

Resource negotiation determines whether an environment can satisfy source intent.

Conceptually:

program intent
      |
      v
requirements
      |
      v
available capabilities
      |
      v
available resources
      |
      v
constraints
      |
      v
candidate realizations
      |
      v
valid realization

Negotiation is not parser behavior.

The grammar only represents the input to negotiation.

---

49. Feasibility

A resource realization is feasible when all mandatory semantic requirements and constraints can be satisfied.

A realization is infeasible when at least one mandatory condition cannot be satisfied.

The compiler/runtime MUST distinguish:

syntactically invalid

from:

semantically invalid

from:

resource-unsatisfiable

from:

target-unrepresentable

---

50. Resource-Unsatisfiable Programs

A program may be syntactically and semantically valid but impossible to realize on a particular target.

For example:

requires capability("quantum.measurement");

is valid source semantics.

A CPU-only target may be unable to satisfy it.

The result is:

resource-unsatisfiable for selected target

not:

invalid Zamani syntax

and not:

silently remove quantum behavior

---

51. No Silent Degradation

An implementation MUST NOT silently:

- reduce resource quantity;
- reduce precision;
- reduce qubit count;
- remove tasks;
- drop distributed nodes;
- change algorithms;
- weaken reliability;
- ignore hard constraints;
- ignore mandatory capabilities;
- change memory semantics;
- change observable behavior.

If semantic degradation is permitted by an explicit program policy, it must be represented explicitly.

---

52. Graceful Scaling

An implementation MAY scale execution according to available resources when the program permits such scaling.

For example:

parallel(work)

may be realized using different amounts of parallel hardware.

The exact amount is an implementation decision unless the program explicitly constrains it.

This is one of the mechanisms supporting:

tiny -> large -> distributed -> future

execution from one semantic program.

---

53. Resource Elasticity

Resource elasticity describes the ability to acquire or release resources as workload changes.

The language may express:

- minimum requirements;
- desired capacity;
- scalable capacity;
- dynamic acquisition;
- release;
- adaptive execution.

The implementation determines how elasticity is realized.

---

54. Resource Lifecycle

Resource lifecycle operations include:

declare
derive
reserve
acquire
use
release
retire

The grammar represents intent.

The runtime/resource manager performs actual lifecycle operations.

Lifecycle operations must respect:

- ownership;
- effects;
- concurrency;
- failure;
- capability requirements.

---

55. Reservation

A reservation expresses intent to hold resource capacity for a defined semantic scope.

Reservation does not itself guarantee that the external environment will provide the resource.

If reservation is mandatory and cannot be established, the implementation must report failure.

---

56. Acquisition

Acquisition requests resource availability for execution.

Acquisition may be:

- static;
- compile-time;
- runtime;
- dynamic;
- scoped.

The actual acquisition mechanism is outside grammar ownership.

---

57. Release

Release expresses that a resource is no longer required by the current semantic scope.

Release must respect:

- ownership;
- aliases;
- dependencies;
- concurrent users;
- lifetime rules.

Physical deallocation is implementation-specific.

---

58. Resource Derivation

A resource may be derived from another resource.

Examples include:

derived_memory = memory_for(workload)

logical_qubits = algorithm_qubits + ancilla_qubits

Derivation is a semantic relationship.

The grammar does not perform the derivation.

Semantic analysis determines validity.

---

59. Resource Properties

Resource properties are extensible.

Examples include:

capacity
availability
latency
throughput
bandwidth
energy
power
reliability
precision
coherence
locality
persistence

New properties must be representable without modifying the universal resource architecture.

Properties may be:

- scalar;
- structured;
- symbolic;
- measured;
- estimated;
- dynamic.

---

60. Resource Property Paths

Properties may be nested.

Conceptually:

resource.memory.capacity

or:

quantum.device.coherence

The grammar must support qualified property paths through the canonical name/path mechanisms.

It must not hard-code a fixed property hierarchy.

---

61. Resource Expressions

Resource expressions reuse the general expression system wherever possible.

The resource grammar must not duplicate:

- arithmetic;
- logical operators;
- comparison;
- function calls;
- identifiers;
- indexing;
- member access.

Resource expressions should reference the canonical expression contract.

This prevents divergence between:

normal expression

and:

resource expression

---

62. Comparison Semantics

Resource comparisons may include:

==
!=
<
<=
>
>=

where valid for the associated resource property.

Semantic analysis determines whether comparison is meaningful.

For example:

memory >= 8 GiB

may be meaningful.

Comparing unrelated resource dimensions must be rejected.

---

63. Resource Arithmetic

Resource arithmetic must respect dimensions.

Examples:

memory_required = elements * element_size

throughput = work / time

Invalid dimensional combinations must be rejected during semantic analysis.

---

64. Resource Equality

Resource equality must distinguish:

semantic equivalence

from:

physical identity

Two different physical realizations may be semantically equivalent.

Therefore physical identity must not leak into semantic equality unless explicitly required.

---

65. Preferences and Optimization

Preferences may influence:

- optimization;
- scheduling;
- placement;
- target selection;
- resource allocation;
- runtime adaptation.

They MUST NOT override mandatory semantic requirements.

Optimization remains responsible for choosing an efficient realization.

---

66. Resource Hints and Compiler Freedom

Hints provide optimization information.

The compiler MAY:

- use;
- partially use;
- ignore;
- transform;

a hint.

A hint MUST NOT change the semantic result.

---

67. Target Intent

A target declaration may identify an abstract execution class.

Examples:

cpu
gpu
quantum
accelerator
heterogeneous
distributed
embedded

These are target classes, not necessarily physical devices.

Target-specific implementation details belong downstream.

---

68. Target-Specific Semantics

Zamani MAY support explicit target-specific constructs.

Such constructs must be clearly marked as target-dependent.

They must not silently become universal requirements.

The compatibility system must be able to identify target-specific constructs.

---

69. Portability Levels

Resource portability can be understood as:

unrestricted target-independent
domain-portable
capability-portable
profile-portable
target-class-specific
target-instance-specific

The implementation should preserve this distinction.

A program using a target-instance-specific construct is not equivalent to a fully target-independent program.

---

70. Portability Does Not Mean Universal Feasibility

POCO-REAF does not mean every target can execute every program.

A program requiring:

quantum.measurement

cannot necessarily execute on a target with no quantum capability.

Portability means:

«the program's source semantics remain stable while compatible realizations may vary.»

---

71. Resource Negotiation and Quantum Hardware

Quantum resource negotiation may consider:

- logical qubit count;
- physical resource capacity;
- measurement capability;
- dynamic control;
- coherence;
- error rates;
- error-correction capabilities;
- required reliability;
- required connectivity.

The grammar expresses the requirements.

The following remain downstream:

- physical mapping;
- gate decomposition;
- routing;
- scheduling;
- QEC;
- ZQN;
- calibration;
- HAL.

---

72. QEC Integration

Resource requirements related to quantum error correction may be represented semantically.

For example, a program may require a certain reliability or fault-tolerance property.

The resource specification does not define QEC algorithms.

The QEC subsystem remains responsible for:

- decoding;
- correction;
- logical protection;
- resource policy;
- runtime accounting.

The repository already contains QEC resource-policy/runtime-accounting concepts such as "QecLimits" and "ResourceManager"; these remain downstream implementation concerns rather than grammar semantics.

---

73. ZQN Integration

ZQN remains responsible for fault/noise semantics.

Resource syntax may state requirements such as:

reliability >= required_reliability

or equivalent quantum reliability intent.

It must not encode a particular ZQN model.

The flow is:

resource requirement
      |
      v
semantic representation
      |
      v
ZQN analysis
      |
      v
realization policy

---

74. Scheduling Integration

Resource intent may provide scheduling constraints.

The grammar does not schedule.

Scheduling consumes:

- resource requirements;
- timing constraints;
- capabilities;
- dependencies;
- preferences.

Scheduling determines:

- ordering;
- timing;
- resource sharing;
- parallel execution;
- placement where appropriate.

---

75. Routing Integration

Resource intent may constrain connectivity or locality.

The grammar does not route.

Routing determines physical realization where required.

For quantum computation:

logical operation
      |
      v
quantum::ir
      |
      v
routing
      |
      v
physical realization

---

76. HAL Integration

HAL translates abstract requirements into target capabilities.

HAL may expose:

- device capabilities;
- resource state;
- availability;
- calibration state;
- hardware properties.

The grammar MUST NOT contain HAL implementation.

---

77. Resource Manager Integration

The resource manager consumes semantic resource requirements.

Conceptually:

AST
 |
 v
resource semantics
 |
 v
ResourceManager
 |
 +-- discover
 +-- evaluate
 +-- reserve
 +-- acquire
 +-- account
 +-- release
 +-- report

The resource manager is not the grammar.

The resource grammar must remain valid even when the resource manager implementation changes.

---

78. Compiler Resource Accounting

Compilation may consume resources such as:

- memory;
- compute;
- temporary storage;
- compilation time;
- parallel workers.

Compiler resource limitations must not be confused with source-language limits.

If the compiler cannot compile a valid program because of implementation resource exhaustion, it must report an explicit compiler/resource failure.

It must not reinterpret the source program.

---

79. Runtime Resource Accounting

Runtime accounting may track:

- acquired resources;
- released resources;
- peak usage;
- current usage;
- failures;
- reservations;
- dynamic scaling.

Runtime accounting must not alter source semantics.

---

80. Resource Failures

Resource failures must be explicit.

Possible categories include:

resource-unavailable
resource-unsatisfiable
capability-unavailable
reservation-failed
acquisition-failed
resource-exhausted
resource-lost
resource-degraded
resource-invalid
resource-conflict
resource-timeout

Exact implementation error types belong outside this document.

The semantic distinction must remain stable.

---

81. Dynamic Resource Failure

If a resource becomes unavailable after successful negotiation, the runtime must follow the applicable resilience/failure policy.

It must not silently change mandatory semantics.

Possible defined outcomes include:

- recovery;
- migration;
- retry where semantically allowed;
- graceful failure;
- escalation;
- termination.

Those decisions belong to the resilience/runtime subsystem.

---

82. Determinism

Resource analysis must be deterministic when the source semantics require deterministic behavior.

It must not depend on:

- hash-map iteration;
- nondeterministic device discovery order;
- arbitrary backend ordering;
- machine enumeration order.

When multiple equivalent realizations exist, their selection may differ internally provided semantic observables remain unchanged.

---

83. Resource Preferences and Reproducibility

Preferences must not make deterministic programs nondeterministic merely because multiple resource realizations exist.

If the selected resource realization is externally observable, the relevant choice must be represented by the defined semantics or provenance system.

---

84. Provenance

Resource-related decisions should be traceable.

Provenance may record:

- requested resource intent;
- evaluated requirements;
- available capabilities;
- selected realization;
- rejected candidates;
- compiler decisions;
- runtime decisions.

Provenance MUST NOT change program semantics.

---

85. Security

Resource declarations must not provide an implicit security bypass.

A source program must not gain access to:

- privileged devices;
- protected memory;
- secret resources;
- restricted network endpoints;

merely by declaring a requirement.

Authorization remains a separate semantic/security concern.

---

86. Isolation

Resource realization MAY require isolation.

Examples:

- process isolation;
- memory isolation;
- accelerator isolation;
- tenant isolation;
- quantum-device isolation.

Isolation policy belongs to the execution/security environment.

---

87. Cost

Cost may represent:

- monetary cost;
- energy cost;
- execution cost;
- resource cost;
- opportunity cost.

Cost is normally a preference or constraint rather than an intrinsic computational meaning.

For example:

prefer lower_cost

must not become:

change algorithm semantics

---

88. Resource Scaling Functions

Resource requirements may depend on workload size.

Conceptually:

resource_requirement(workload_size)

This enables programs to express scalable resource behavior.

Examples:

memory = f(input.size)

parallelism = g(workload.size)

qubits = h(problem.size)

No fixed upper bound is encoded.

---

89. Monotonic Scaling

Where semantically appropriate, resource requirements SHOULD be expressible as monotonic functions of workload.

For example:

larger workload -> no fewer required elements

However, the language must not assume all algorithms scale monotonically.

The semantic model must support arbitrary valid resource relationships.

---

90. Resource Independence

A program should avoid unnecessary coupling between unrelated resource domains.

For example:

memory requirement

should not automatically constrain:

GPU count

unless an explicit semantic relationship exists.

This prevents hidden hardware coupling.

---

91. Heterogeneous Resources

A computation may use multiple resource classes simultaneously.

Example:

CPU + GPU + network + storage

or:

classical + quantum

or:

CPU + FPGA + accelerator

Each resource requirement remains semantically distinct.

The compiler may coordinate them.

---

92. Resource Substitution

A realization may substitute one physical implementation for another if the replacement provides equivalent required semantics.

For example:

tensor.compute

might be realized by:

- CPU;
- GPU;
- FPGA;
- ASIC;
- future accelerator.

Substitution is valid only when semantic requirements remain satisfied.

---

93. No Vendor Lock-In in Core Resource Semantics

The universal resource grammar must not require vendor-specific APIs.

Vendor-specific resource kinds MAY exist through explicit dialect mechanisms.

For example:

vendor::accelerator::feature

must remain distinguishable from:

accelerator.compute

---

94. Dialect Integration

A dialect may extend resource syntax.

A dialect MUST define:

- dialect name;
- version;
- namespace;
- resource kinds;
- properties;
- capabilities;
- semantic mapping;
- compatibility;
- AST mapping;
- IR mapping.

A dialect MUST NOT redefine universal meanings.

---

95. Resource Feature Lifecycle

A resource feature progresses through:

proposal
   |
experimental
   |
specified
   |
implemented
   |
conformance-tested
   |
stable

A feature appearing only in:

grammar/Zamani-Grammar.md

is not automatically normative.

The feature becomes normative only after integration into:

- specification;
- grammar;
- AST;
- semantics;
- IR;
- implementation;
- tests.

---

96. Compatibility

Resource syntax must follow the repository's compatibility policy.

Compatibility must account for:

specification
grammar
lexer
parser
AST
semantic model
IR
compiler
runtime

A syntax change must not silently change the meaning of an existing resource construct.

Breaking changes require an explicit compatibility mechanism.

---

97. Backward Compatibility

Existing valid resource constructs should remain valid unless deliberately deprecated.

When semantics must change:

1. identify the affected feature;
2. define the new semantic contract;
3. define migration behavior;
4. define diagnostics;
5. update conformance tests;
6. update compatibility documentation.

Do not silently repurpose existing resource keywords.

---

98. Grammar-to-AST Contract

Each resource grammar rule MUST map to an already-defined AST category.

At minimum the semantic representation must preserve:

ResourceIntent {
    kind
    identity
    resource_kind
    expression
    properties
    attributes
    source_span
}

and distinguish:

Requirement
Constraint
Capability
Preference
Hint

The exact Rust structure belongs to "src/frontend/ast/" and semantic implementation.

---

99. AST-to-Semantic Contract

The semantic layer converts syntactic resource constructs into canonical semantic resource requirements.

The semantic layer must perform:

- name resolution;
- type checking;
- unit checking;
- dimensional checking;
- scope checking;
- capability resolution;
- constraint validation;
- requirement composition;
- conflict detection;
- dependency analysis.

---

100. Semantic-to-IR Contract

Resource metadata must lower into the canonical semantic representation.

Resource information MUST NOT be duplicated into unrelated domain IRs when one shared semantic representation is sufficient.

Quantum resource information associated with quantum computation ultimately integrates with:

quantum::ir

without replacing it.

---

101. Compiler Contract

The compiler MAY use resource information to:

- select targets;
- specialize;
- optimize;
- parallelize;
- distribute;
- route;
- schedule;
- select memory;
- select accelerators;
- select execution policies.

All transformations MUST preserve semantic meaning.

---

102. Runtime Contract

The runtime MAY evaluate dynamic resource conditions.

Runtime evaluation MUST:

- preserve requirement/constraint semantics;
- preserve explicit failure semantics;
- preserve ownership;
- preserve effects;
- preserve determinism requirements.

---

103. Diagnostics

Resource diagnostics MUST distinguish:

syntax error
semantic error
type error
capability error
constraint conflict
resource unsatisfiable
resource unavailable
target unsupported
implementation resource exhaustion
runtime resource failure

Diagnostics SHOULD contain:

- diagnostic code;
- severity;
- source span;
- resource identity;
- requested quantity/property;
- relevant constraint;
- current context where available;
- target context where applicable;
- remediation information where meaningful.

---

104. No Silent Overflow

Resource quantity evaluation MUST NOT silently overflow.

If an implementation uses bounded integers internally and the semantic quantity exceeds that representation, it must:

1. detect the condition;
2. report a defined error;
3. avoid silent truncation/wrapping.

The language must not turn implementation integer limits into language-level resource limits.

---

105. No Silent Underflow

The same rule applies to resource subtraction and derived quantities.

For example:

available_memory - reserved_memory

must not silently wrap when the result is negative if the semantic domain disallows it.

---

106. No Fixed Resource IDs

The universal resource system must not encode:

CPU0
CPU1
GPU0
GPU1
QPU0
QPU1
QUBIT0
QUBIT1
NODE0
NODE1

as the universe of possible resources.

Such identifiers may exist in target-specific contexts.

They are not universal resource semantics.

---

107. No Fixed Topology

The resource grammar must not encode fixed:

- mesh dimensions;
- torus dimensions;
- network node counts;
- QPU connectivity;
- accelerator counts.

Topology constraints may be represented abstractly.

Physical topology is discovered downstream.

---

108. No Fixed Device Counts

Any grammar design equivalent to:

device : DEVICE0 | DEVICE1 | DEVICE2;

is prohibited.

Resource cardinality is unbounded by language design.

---

109. No Fixed Quantum Limits

The resource grammar must not impose:

MAX_QUBITS
MAX_SHOTS
MAX_REGISTERS
MAX_QUANTUM_DEVICES

or equivalent limits.

A quantum requirement is a semantic requirement.

Its feasibility is determined against available resources.

---

110. No Fixed AI/ML Hardware Limits

AI/data resource syntax must not encode:

- fixed tensor dimensions;
- fixed accelerator counts;
- fixed memory;
- fixed model sizes;
- fixed GPU counts.

Model/workload size is semantic data.

Execution capacity is a resource property.

---

111. No Fixed HDL Hardware Limits

HDL resource semantics must not establish universal:

MAX_LUTS
MAX_BRAMS
MAX_DSPS
MAX_PINS
MAX_CLOCKS

or equivalent constants.

Hardware-specific constraints belong to the selected hardware target.

---

112. Resource Introspection

Programs MAY inspect resources only through explicitly defined semantic mechanisms.

Resource introspection must not expose arbitrary implementation details unless the language contract says those details are observable.

This preserves portability.

---

113. Resource Discovery

Resource discovery belongs to:

- compiler;
- resource manager;
- HAL;
- runtime;
- deployment environment.

The parser must never perform discovery.

---

114. Resource Caching

Implementations MAY cache resource discovery information.

Caching must not change semantics when the underlying resource information is semantically dynamic.

Cache invalidation belongs to the resource/runtime subsystem.

---

115. Resource Reservations and Concurrency

Concurrent programs may compete for resources.

Reservation and acquisition must respect:

- ownership;
- synchronization;
- isolation;
- failure;
- cancellation.

The grammar expresses intent; the runtime implements arbitration.

---

116. Cancellation

Resource acquisition SHOULD integrate with the repository's cancellation model where applicable.

Cancellation must not leak resources.

The grammar does not implement cancellation.

---

117. Resource Lifetime

Resource lifetimes must integrate with:

- lexical scopes;
- ownership;
- borrowing;
- effects;
- asynchronous tasks;
- distributed execution.

A resource must not be released while a valid semantic consumer still depends on it.

---

118. Resource Ownership

Where resource ownership is represented by the type/effect system, resource intent must respect those contracts.

The resource grammar must not create a second ownership model.

---

119. Resource Sharing

Resources may be:

- exclusive;
- shared;
- replicated;
- partitioned;
- pooled.

The semantic model must distinguish these cases where correctness depends on them.

Actual sharing mechanisms are runtime/backend concerns.

---

120. Resource Aliasing

Two resource references may refer to the same semantic resource.

Aliasing rules belong to semantic analysis.

Physical aliasing must not be inferred merely because two names happen to resolve to the same implementation object.

---

121. Resource Partitioning

A resource may be partitioned into logical regions.

Examples:

- memory;
- compute;
- distributed datasets;
- accelerators.

Partitioning must remain semantic where relevant and implementation-defined otherwise.

---

122. Resource Replication

Replication may be used for:

- availability;
- performance;
- fault tolerance;
- distributed computation.

Replication policy belongs downstream unless explicitly required by source semantics.

---

123. Resource Migration

A logical resource may migrate between physical realizations when semantics permit.

Examples:

logical computation
        |
        +--> CPU
        +--> GPU
        +--> accelerator
        +--> another node

Migration must preserve observable semantics.

---

124. Resource Affinity

Affinity expresses preference or constraint regarding locality or association.

Affinity must be explicitly categorized as:

- requirement;
- constraint;
- preference;
- hint.

It must not be inferred from an ordinary resource reference.

---

125. Resource Locality

Locality may describe:

- memory locality;
- compute locality;
- network locality;
- quantum-device locality;
- data locality.

Locality should normally be advisory unless correctness explicitly depends upon it.

---

126. Resource Quality

Resource quality may include:

- reliability;
- precision;
- coherence;
- latency;
- bandwidth;
- availability;
- energy efficiency.

Quality is represented through properties and constraints.

---

127. Resource Profiles for Different Scales

A profile may remain valid across different scales.

For example:

profile scalable_compute {
    requires capability("parallel.compute");
    prefer locality;
}

The implementation may realize it with different resource quantities.

This is preferable to defining separate programs for:

small_machine
large_machine
cluster
supercomputer

---

128. Resource Requirements and Program Parameters

Requirements may depend on program parameters.

Example:

requires memory >= input.size * element_size;

The requirement therefore scales with the actual workload.

No source rewrite is required merely because the input grows.

---

129. Resource Requirements and Generic Programs

Generic programs may define resource relationships parameterized by generic values or types.

Semantic validation must ensure that all valid parameterizations satisfy the declared contract or produce an explicit specialization-time failure.

---

130. Resource Requirements and Compile-Time Evaluation

Compile-time resource expressions MAY be evaluated when all required information is available.

The compiler must not assume compile-time knowledge where the language permits runtime values.

---

131. Resource Requirements and Runtime Evaluation

Runtime resource expressions MAY depend on:

- input;
- dynamic state;
- availability;
- workload size;
- environment.

Such evaluation must be explicit in the semantic model.

---

132. Resource Expressions Must Remain Safe

Resource expressions must not require "unsafe" Rust.

The implementation baseline is:

Rust 1.97 / Rust 1.97.1
Rust 2021
Safe Rust only

No resource feature may require embedded "unsafe" Rust.

---

133. Resource Security Boundaries

Resource declarations must not bypass:

- authorization;
- isolation;
- sandboxing;
- capability security;
- secret handling.

A declaration of:

resource privileged_device;

does not grant permission to use it.

---

134. Capability Security

Where capabilities have security implications, capability semantics must integrate with the security subsystem.

A source capability requirement is not equivalent to an authorization grant.

---

135. Resource Provenance

Resource decisions should be attributable to:

- source requirement;
- semantic analysis;
- compiler policy;
- runtime policy;
- target capability information.

This allows reproducibility and diagnostics.

---

136. Reproducible Compilation

Resource-aware compilation SHOULD be reproducible given identical:

- source;
- compiler version;
- semantic configuration;
- target contract;
- capability information;
- resource context;
- deterministic policy.

Different physical realizations may produce different implementations while preserving semantics.

---

137. Resource-Aware Optimization

Optimization may use resource information.

Examples:

parallelization
vectorization
accelerator selection
memory tiling
distributed partitioning
quantum decomposition

Optimization must not mutate a mandatory resource requirement into a weaker one.

---

138. Resource-Aware Scheduling

Scheduling may use:

- capacity;
- latency;
- throughput;
- energy;
- availability;
- reliability;
- concurrency.

Scheduling must preserve semantic dependencies.

---

139. Resource-Aware Routing

Routing may use:

- topology;
- bandwidth;
- latency;
- connectivity;
- resource availability.

Routing remains target realization.

---

140. Resource-Aware Resilience

Resilience may use:

- resource redundancy;
- reliability;
- availability;
- fault state;
- degradation.

Resource syntax provides intent.

Resilience implements policy.

---

141. Resource Degradation

Resource quality may change over time.

Examples:

Healthy
Degraded
Unavailable
Recovering

The runtime/resilience system owns state transitions.

The grammar does not implement them.

---

142. Resource State

Resource state may be:

- unknown;
- available;
- unavailable;
- degraded;
- reserved;
- acquired;
- failed;
- retired.

State is contextual.

A source resource name does not itself encode the current state.

---

143. Resource Contracts Across Domains

The same resource contract model must work across:

classical
quantum
hybrid
HDL
hardware
distributed
AI
data
networking
security
embedded
scientific
accelerator
future domains

Domain-specific grammars may add properties but must normalize to the common resource model.

---

144. Cross-Domain Example

A hybrid computation might require:

classical compute
+
quantum capability
+
measurement
+
communication
+
memory

These remain separate resource requirements.

The compiler may coordinate them.

The grammar does not dictate the physical architecture.

---

145. Resource Requirements and Effects

Resource operations may interact with effects.

For example:

acquire
release
reserve
network use
device access

must integrate with the effect system.

The resource grammar must not create a second effect system.

---

146. Resource Requirements and Types

Resource quantities and properties must use canonical types.

For example:

memory_size
duration
bandwidth
energy

must be type-checked according to the type system.

---

147. Resource Requirements and Modules

Modules may declare resource contracts.

Imports must preserve the contracts of imported components.

A module requiring a capability imposes that requirement on any calling context unless explicitly abstracted by a valid semantic mechanism.

---

148. Resource Requirements and Functions

Functions may have resource contracts.

A function's resource contract must be visible to semantic analysis and compiler planning.

Generic functions may have parameterized resource requirements.

---

149. Resource Requirements and Macros

Macros may generate resource declarations.

Macro expansion must not bypass resource validation.

The expanded syntax must undergo normal resource semantic analysis.

---

150. Resource Requirements and Metaprogramming

Compile-time generated resource declarations must be validated exactly like handwritten declarations.

Metaprogramming must not create hidden resource requirements.

---

151. Resource Requirements and Interoperability

Foreign functions may have resource contracts.

For example, an FFI function may require:

capability("foreign.compute")

or another explicitly defined capability.

The resource contract must remain visible to semantic analysis.

---

152. Resource Requirements and Dialects

Dialect-defined resource requirements must declare their semantic mapping.

A dialect must not silently redefine:

requires
constraint
prefer
hint
capability

with incompatible semantics.

---

153. Resource Grammar Files

The existing resource grammar files remain individually responsible for syntax within their declared scope.

"grammar/resources/resources.g4"

Owns:

- universal resource declaration composition;
- resource item dispatch;
- resource names;
- resource types;
- resource specifications;
- resource clauses;
- resource expression entry points.

It must remain a composition grammar, not a semantic implementation.

"grammar/resources/requirements.g4"

Owns requirement syntax.

It must preserve mandatory semantics.

"grammar/resources/constraints.g4"

Owns constraint syntax.

It must preserve hard constraint semantics.

"grammar/resources/capabilities.g4"

Owns capability syntax.

Capability names remain extensible.

"grammar/resources/preferences.g4"

Owns advisory preference syntax.

"grammar/resources/resource-expressions.g4"

Owns resource-specific expression entry points while delegating ordinary expressions to the canonical expression grammar.

"grammar/resources/scalability.g4"

Owns scaling-intent syntax.

It must never establish maximum scale.

"grammar/resources/portability.g4"

Owns portability-intent syntax.

"grammar/resources/performance.g4"

Owns performance-related intent.

"grammar/resources/latency.g4"

Owns latency-related intent.

"grammar/resources/energy.g4"

Owns energy-related intent.

"grammar/resources/reliability.g4"

Owns reliability-related intent.

These files MUST implement this semantic contract rather than redefine it.

---

154. No Duplicate Resource Semantics

The repository must have one semantic definition for:

requirement
constraint
capability
preference
hint

Grammar files may distribute syntax.

They must not distribute conflicting meanings.

---

155. Resource Specification and "grammar/specification/semantics.md"

"grammar/specification/semantics.md" defines the broader semantic model.

This file specializes that model for resources.

When a general semantic rule and a resource rule interact:

general semantic rule
+
resource-specific rule

both apply.

This document must not contradict the general semantic contract.

---

156. Resource Specification and "grammar/spec/type-system.md"

Resource quantities, dimensions, units, and properties must use the canonical type system.

The resource grammar must not invent incompatible types.

---

157. Resource Specification and "grammar/spec/effects.md"

Resource acquisition, release, device interaction, networking, and similar operations may produce effects.

Effect semantics remain owned by the effect specification.

---

158. Resource Specification and "grammar/spec/compatibility.md"

Resource syntax evolution follows the repository compatibility contract.

Any incompatible resource change must be recorded there.

---

159. Resource Specification and "grammar/spec/syntax.md"

Syntax belongs to the syntax specification and grammar files.

This document describes semantic meaning.

Examples in this file are illustrative unless explicitly marked normative.

---

160. Resource Specification and "grammar/grammar.md"

"grammar/grammar.md" must describe implementation conformance.

It must not become a competing semantic authority.

Resource features should be classified as:

specified
implemented
partially implemented
experimental
deprecated
unsupported

---

161. Resource Specification and "grammar/Zamani-Grammar.md"

"Zamani-Grammar.md" may contain historical, experimental, or aspirational resource concepts.

Those concepts become normative only after promotion through the specification and conformance process.

---

162. Resource Specification and "grammar/Zamani.g4"

"Zamani.g4" remains the canonical grammar composition root.

Resource grammar must be integrated through that root.

No second root grammar is permitted.

---

163. Resource Specification and "src/lexer.rs"

All resource tokens must correspond to canonical lexer behavior.

A resource keyword appearing in a grammar but absent from the canonical lexer is an integration failure.

---

164. Resource Specification and "src/parser.rs"

The parser must produce the expected AST structure without performing resource discovery or allocation.

---

165. Resource Specification and "src/frontend/ast/"

Resource constructs must use the domain-neutral AST architecture.

Resource AST nodes must not depend on:

- LLVM;
- QIR;
- MLIR;
- CUDA;
- vendor APIs;
- physical topology;
- QEC implementation;
- routing implementation.

---

166. Resource Specification and Quantum IR

Resource semantics must integrate with:

quantum::ir

rather than create another quantum resource IR.

Resource metadata may accompany or annotate canonical quantum semantic constructs.

---

167. Resource Specification and HAL

HAL translates abstract capability/resource requirements into target-specific information.

The grammar remains independent of HAL.

---

168. Resource Specification and Scheduling

Scheduling receives resource requirements after semantic analysis.

Scheduling does not redefine source resource semantics.

---

169. Resource Specification and Routing

Routing receives connectivity/resource constraints after semantic analysis.

Routing does not redefine resource requirements.

---

170. Resource Specification and QEC

QEC receives applicable resource/reliability requirements.

The grammar does not implement QEC.

---

171. Resource Specification and ZQN

ZQN receives relevant fault/noise/reliability semantics.

The resource grammar does not encode ZQN implementation.

---

172. Resource Specification and Resilience

The resilience subsystem consumes resilience/resource intent and determines recovery policy.

The resource grammar only expresses the contract.

---

173. Resource Specification and Benchmarking

Benchmarking may measure resource properties such as:

- throughput;
- latency;
- energy;
- reliability.

Measurements must not retroactively change source semantics.

---

174. Resource Specification and Calibration

Calibration information may affect target feasibility.

Calibration does not change the meaning of source resource requirements.

---

175. Resource Specification and Provenance

Resource realization decisions should be traceable through the repository's provenance mechanisms.

This supports:

- reproducibility;
- diagnostics;
- verification;
- auditing.

---

176. Resource Constraint Conflicts

If two mandatory constraints conflict:

constraint A
constraint B

the compiler must reject the realization unless the semantic system can formally establish that both are satisfiable.

It must not choose one arbitrarily.

---

177. Requirement Conflicts

Conflicting requirements must produce a diagnostic.

Examples:

requires capability("A");
requires capability("not-A");

where the semantic model establishes incompatibility.

---

178. Preference Conflicts

Conflicting preferences do not necessarily make a program invalid.

The optimizer may rank them according to the defined preference policy.

Preference ordering must be deterministic where observable.

---

179. Hint Conflicts

Conflicting hints may be ignored or resolved by the implementation.

They cannot invalidate otherwise valid source semantics unless the hint is explicitly promoted to a requirement or constraint.

---

180. Resource Constraint Solving

Constraint solving belongs to semantic/compiler infrastructure.

The grammar only provides the syntactic representation.

The solver must be safe, deterministic where required, and explicit about unsatisfiable conditions.

---

181. Symbolic Resource Constraints

The semantic system should support symbolic constraints where values cannot be resolved statically.

Examples:

memory >= input.size * element_size

qubits >= problem.size

bandwidth >= workload / duration

These expressions enable scale-independent programs.

---

182. Resource Intervals

Where appropriate, resources may be described by intervals:

minimum <= resource <= maximum

The implementation must preserve whether a bound is:

- minimum requirement;
- maximum constraint;
- preferred target.

---

183. Resource Bounds

A source-level upper bound is not automatically a hardware limit.

For example:

memory <= memory_budget

is a program constraint.

It does not mean the language believes machines cannot provide more memory.

---

184. Resource Budgets

Budgets may constrain:

- memory;
- energy;
- time;
- communication;
- monetary cost;
- storage;
- compute.

Budgets are semantic constraints when explicitly declared.

---

185. Resource Objectives

A program may specify an optimization objective such as:

prefer lower latency

or:

prefer lower energy

Objectives must not override hard constraints.

---

186. Resource Negotiation Order

A conceptual negotiation order is:

1. validate syntax
2. validate types
3. resolve names
4. resolve resource identities
5. resolve capabilities
6. evaluate requirements
7. evaluate hard constraints
8. eliminate invalid realizations
9. rank preferences
10. apply hints
11. choose realization
12. verify semantic preservation

The implementation may use a different internal algorithm provided the observable semantics remain equivalent.

---

187. Resource Selection

Resource selection is an implementation decision.

The compiler/runtime MAY choose:

- one resource;
- several resources;
- replicated resources;
- pooled resources;
- heterogeneous resources.

The selection must satisfy the semantic contract.

---

188. Resource Pooling

Resource pools permit scalable execution.

A pool may dynamically provide resources.

The language must not assume a fixed pool size.

---

189. Resource Oversubscription

Oversubscription is an implementation/runtime policy unless explicitly represented in source semantics.

The runtime must not oversubscribe in a way that violates hard resource requirements.

---

190. Resource Underutilization

The implementation may use fewer resources than physically available.

A larger machine does not require the program to consume all available resources.

---

191. Resource Expansion

If more resources become available, a scalable program MAY use them when its semantic contract permits.

This is implementation freedom.

---

192. Resource Reduction

If fewer resources are available, a scalable program MAY use fewer resources only when doing so still satisfies its semantics.

The implementation must not silently violate mandatory requirements.

---

193. Resource-Aware Parallelism

Parallelism is a resource relationship, not a fixed machine count.

The source may express:

parallelism = workload.size

rather than:

use 16 threads

unless a fixed thread count is genuinely part of the program's semantics.

---

194. Resource-Aware Quantum Parallelism

Quantum computation must similarly avoid physical-count assumptions.

A source program may describe:

logical_qubits = f(problem_size)

and allow downstream realization to determine physical resources.

---

195. Resource-Aware AI Scaling

AI programs may describe:

model_size
batch_size
tensor_shape
parallelism
memory_requirement

without binding them to a fixed accelerator.

---

196. Resource-Aware Data Scaling

Data programs may express resource relationships based on:

dataset.size
record_count
tensor_shape
stream_rate
partition_count

without hard-coded infrastructure limits.

---

197. Resource-Aware Distributed Scaling

Distributed programs may scale from one node to many nodes without source rewriting when semantics permit.

The number of nodes remains a realization property unless explicitly constrained.

---

198. Resource-Aware Embedded Scaling

Embedded programs may use the same resource semantics while targeting smaller resource environments.

If requirements cannot be satisfied, the compiler must report that explicitly.

---

199. Resource-Aware Future Architectures

The resource model must support future resource kinds and capabilities without requiring a redesign of the core grammar.

Examples include future:

- compute architectures;
- memory technologies;
- quantum architectures;
- neuromorphic systems;
- optical systems;
- molecular systems;
- biological computing systems;
- hybrid systems.

Unknown future implementations must not invalidate the resource model.

---

200. Resource Model Extensibility

The resource model is intentionally open-ended.

New resource kinds should be introduced through:

resource kind
+
property definitions
+
capability definitions
+
semantic mapping
+
AST mapping
+
IR mapping
+
compatibility
+
tests

rather than through arbitrary parser special cases.

---

201. Hard-Coding Audit

Every resource-related grammar/specification change MUST pass a hard-coding audit.

The audit must reject universal semantic constructs equivalent to:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_ACCELERATORS
MAX_DEVICES
MAX_TIMELINES
MAX_TENSORS
MAX_VECTOR_WIDTH
MAX_REGISTER_WIDTH

It must also detect indirect fixed limits.

---

202. Valid Constants

The prohibition on hard-coded resource limits does not prohibit program constants.

This is valid:

let n = 1024;
allocate n;

if "1024" is part of program semantics.

This is invalid:

MAX_QUBITS = 1024

when it is presented as a universal language limit.

The distinction is:

program data
!=
language implementation limit

---

203. Physical Limits

Physical limits are real.

They belong to:

target
+
hardware
+
resource manager
+
runtime

not to universal grammar semantics.

---

204. Compiler Limits

Compiler implementation limits may exist because of finite host resources.

Such limits must be documented as implementation limits.

They must not become source-language semantics.

---

205. Runtime Limits

Runtime limits may occur because of actual resource exhaustion.

Such conditions must be reported explicitly.

---

206. No Silent Truncation

The implementation MUST NOT silently:

- truncate resource quantities;
- clamp resource counts;
- reduce dimensions;
- reduce work;
- reduce precision;
- reduce reliability.

---

207. No Silent Substitution

A compiler may choose a different implementation only if it preserves semantics.

For example:

tensor computation

may move from GPU to CPU if the semantic result remains valid.

But a mandatory requirement must not be removed merely to make the program execute.

---

208. Verification

Before execution, the implementation SHOULD verify:

requirements satisfied
constraints satisfied
capabilities available
types valid
resource quantities valid
resource relationships valid
target realization valid

Verification must be explicit.

---

209. Resource Conformance

A resource implementation is conformant only when:

1. syntax is recognized;
2. AST mapping exists;
3. semantic meaning is implemented;
4. requirements remain requirements;
5. constraints remain constraints;
6. preferences remain preferences;
7. hints remain hints;
8. quantities are scalable;
9. no artificial resource ceiling exists;
10. diagnostics are defined;
11. downstream integration exists;
12. tests exist.

---

210. Required Tests

The resource subsystem MUST include:

tests/
    resource/
        lexical/
        syntax/
        semantic/
        requirements/
        constraints/
        capabilities/
        preferences/
        hints/
        quantities/
        scalability/
        portability/
        quantum/
        classical/
        hdl/
        hybrid/
        distributed/
        ai/
        data/
        networking/
        security/
        negative/
        boundary/
        determinism/
        compatibility/

Exact directory naming must follow the repository's existing test organization rather than creating unnecessary duplicates.

---

211. Positive Tests

Positive tests must cover:

- resource declarations;
- symbolic quantities;
- requirements;
- constraints;
- capabilities;
- preferences;
- hints;
- properties;
- groups;
- profiles;
- contracts;
- lifecycle;
- dynamic quantities;
- domain-specific resources.

---

212. Negative Tests

Negative tests must cover:

- invalid resource kinds where required;
- malformed quantities;
- invalid units;
- incompatible dimensions;
- conflicting requirements;
- unsatisfiable constraints;
- invalid capability expressions;
- invalid resource relationships;
- unauthorized physical resources where applicable.

---

213. Boundary Tests

Boundary tests must include:

- zero where semantically valid;
- minimum valid quantities;
- very large representable quantities;
- symbolic quantities;
- nested resource expressions;
- empty resource sets where valid;
- large resource collections;
- deeply nested resource properties.

No test may define an artificial maximum merely because the test fixture is convenient.

---

214. Scalability Tests

Scalability tests must demonstrate that resource semantics work for:

tiny
small
medium
large
very large
symbolically sized
dynamically sized
distributed
heterogeneous

The tests should emphasize that no language-level ceiling exists.

---

215. Quantum Scalability Tests

Quantum resource tests must include:

one logical qubit
multiple logical qubits
symbolic logical-qubit count
large symbolic qubit count
dynamic resource requirement
logical/physical distinction
measurement capability
fault-tolerance requirement

No test may establish a universal maximum qubit count.

---

216. Distributed Scalability Tests

Distributed tests must cover:

single-node realization
multi-node realization
dynamic node availability
replication
partitioning
communication requirements

without fixing a universal node count.

---

217. Determinism Tests

Resource semantic analysis must be tested for deterministic output when the semantic context is identical.

The result must not depend on:

- map iteration order;
- discovery ordering;
- backend enumeration order.

---

218. Compatibility Tests

Compatibility tests must verify that existing valid resource constructs retain their defined meaning across supported language versions.

---

219. Cross-Domain Tests

At minimum, resource integration must be tested with:

classical
quantum
hybrid
hdl
hardware
distributed
ai
data
networking
security

---

220. Example: Portable Memory Requirement

Conceptual source:

requires resource memory >= workload_memory;

Meaning:

the realization must provide sufficient memory

Not:

use RAM
use VRAM
use address X
use device Y

---

221. Example: Portable Quantum Requirement

Conceptual source:

requires capability("quantum.measurement");

Meaning:

the realization must provide a compatible measurement capability

Not:

use QPU 0
use physical qubit 17

---

222. Example: Portable Accelerator Preference

Conceptual source:

prefer capability("tensor.compute");

Meaning:

prefer a realization capable of tensor computation

The implementation may select any compatible accelerator or computation resource.

---

223. Example: Scale-Dependent Requirement

Conceptual source:

requires memory >= input.size * element_size;

The same source can represent different workloads.

The required memory scales with the input.

No source rewrite is required merely because the input changes size.

---

224. Example: Requirement Versus Preference

These are different:

requires capability("quantum.measurement");

and:

prefer capability("quantum.measurement");

The first is mandatory.

The second is advisory.

The implementation MUST preserve this distinction.

---

225. Example: Requirement Versus Implementation

This is semantic:

requires memory >= workload_memory;

This is implementation:

allocate memory on device X;

The latter belongs downstream unless explicitly exposed through target-specific syntax.

---

226. Example: Heterogeneous Computation

A hybrid program may conceptually require:

requires capability("classical.compute");
requires capability("quantum.compute");
requires capability("quantum.measurement");
requires capability("communication");

The implementation determines how those capabilities are realized.

---

227. Example: Elastic Distributed Execution

A distributed program may express:

requires capability("distributed.execution");
requires bandwidth >= required_bandwidth;
prefer locality;

The implementation may select an appropriate distributed realization.

---

228. Example: Hardware Co-Design

An HDL/co-design program may specify:

requires capability("reconfigurable.hardware");
requires memory >= required_memory;
requires performance >= required_performance;

The implementation may target an FPGA, reconfigurable accelerator, or another compatible realization.

---

229. Example: Resource Failure

If:

requires capability("quantum.measurement");

and no compatible capability exists, the implementation must report a capability/resource failure.

It must not silently remove measurement.

---

230. Resource Semantics and "Forever"

"Forever" means semantic continuity.

It does not promise that:

- today's binary format never changes;
- today's hardware remains supported forever;
- every future architecture executes today's binary without lowering.

Instead:

stable source semantics
+
versioned contracts
+
compatible lowering
+
extensible capabilities
=
POCO-REAF

---

231. Production Invariants

The following are mandatory invariants.

Invariant 1

Resource syntax is target-independent by default.

Invariant 2

Requirements remain requirements.

Invariant 3

Constraints remain constraints.

Invariant 4

Preferences remain preferences.

Invariant 5

Hints remain hints.

Invariant 6

Capabilities describe abilities, not permissions.

Invariant 7

Resource identity does not imply physical identity.

Invariant 8

Quantities are expressions.

Invariant 9

No artificial resource ceiling exists in the language.

Invariant 10

Resource discovery is downstream.

Invariant 11

Physical placement is downstream.

Invariant 12

Scheduling is downstream.

Invariant 13

Routing is downstream.

Invariant 14

QEC is downstream.

Invariant 15

ZQN is downstream.

Invariant 16

HAL is downstream.

Invariant 17

Resource semantics do not create another quantum IR.

Invariant 18

"quantum::ir" remains the canonical quantum semantic boundary.

Invariant 19

All resource implementations use Safe Rust.

Invariant 20

No "unsafe" Rust is required.

Invariant 21

Valid resource quantities cannot silently overflow.

Invariant 22

Unsatisfied mandatory requirements cannot silently degrade.

Invariant 23

Domain-specific resource extensions normalize into the common model.

Invariant 24

Resource syntax cannot silently create vendor lock-in.

Invariant 25

Every resource feature has an AST/semantic/IR integration contract.

---

232. Completion Criteria

"grammar/spec/resources.md" is complete only when all of the following are true:

- [ ] resource intent semantics are defined;
- [ ] requirement semantics are defined;
- [ ] constraint semantics are defined;
- [ ] capability semantics are defined;
- [ ] preference semantics are defined;
- [ ] hint semantics are defined;
- [ ] resource identity semantics are defined;
- [ ] resource quantity semantics are defined;
- [ ] scalability semantics are defined;
- [ ] portability semantics are defined;
- [ ] capacity semantics are defined;
- [ ] availability semantics are defined;
- [ ] performance semantics are defined;
- [ ] latency semantics are defined;
- [ ] throughput semantics are defined;
- [ ] bandwidth semantics are defined;
- [ ] energy semantics are defined;
- [ ] power semantics are defined;
- [ ] reliability semantics are defined;
- [ ] resilience integration is defined;
- [ ] lifecycle semantics are defined;
- [ ] reservation semantics are defined;
- [ ] acquisition semantics are defined;
- [ ] release semantics are defined;
- [ ] grouping semantics are defined;
- [ ] profiles are defined;
- [ ] contracts are defined;
- [ ] dynamic requirements are defined;
- [ ] resource negotiation is defined;
- [ ] resource failure semantics are defined;
- [ ] quantum integration is defined;
- [ ] classical integration is defined;
- [ ] HDL integration is defined;
- [ ] hybrid integration is defined;
- [ ] distributed integration is defined;
- [ ] AI/data integration is defined;
- [ ] networking integration is defined;
- [ ] security boundaries are defined;
- [ ] AST integration is defined;
- [ ] IR integration is defined;
- [ ] "quantum::ir" integration is defined;
- [ ] compiler integration is defined;
- [ ] runtime integration is defined;
- [ ] HAL integration is defined;
- [ ] QEC integration is defined;
- [ ] ZQN integration is defined;
- [ ] routing integration is defined;
- [ ] scheduling integration is defined;
- [ ] resilience integration is defined;
- [ ] diagnostics are defined;
- [ ] compatibility requirements are defined;
- [ ] hard-coding rules are defined;
- [ ] scalability requirements are defined;
- [ ] Safe Rust/no-unsafe requirement is defined;
- [ ] positive tests are defined;
- [ ] negative tests are defined;
- [ ] boundary tests are defined;
- [ ] scalability tests are defined;
- [ ] determinism tests are defined;
- [ ] compatibility tests are defined;
- [ ] cross-domain tests are defined.

---

233. Final Normative Principle

Zamani resource semantics are governed by one fundamental rule:

«The program describes what it needs and what it means; the implementation determines how those needs are realized.»

Therefore:

Program
   |
   v
Semantic intent
   |
   +--> computation
   +--> types
   +--> effects
   +--> resources
   +--> capabilities
   +--> constraints
   |
   v
Canonical semantic representation
   |
   +--> classical IR
   +--> quantum::ir
   +--> HDL/hardware/domain IR
   |
   v
Optimization
   |
   +--> routing
   +--> scheduling
   +--> QEC
   +--> ZQN
   +--> resilience
   |
   v
HAL
   |
   v
Target realization

The language must therefore remain:

resource-parametric
capability-driven
target-independent by default
domain-extensible
hardware-agnostic by default
scalable
deterministic where required
explicit about failure
safe
free of artificial machine-size limits

The resource system must scale from:

atom

to:

single machine

to:

heterogeneous machine

to:

cluster

to:

distributed system

to:

future computing architecture

without making the resource model itself the bottleneck.

The definitive boundary is:

SOURCE RESOURCE INTENT
        !=
RESOURCE DISCOVERY
        !=
RESOURCE ALLOCATION
        !=
PHYSICAL PLACEMENT
        !=
SCHEDULING
        !=
ROUTING
        !=
QEC
        !=
ZQN
        !=
HAL
        !=
HARDWARE

This separation is mandatory for POCO-REAF.

Program Once. Compile Once. Run Everywhere. Anywhere. Forever.