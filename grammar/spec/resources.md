Zamani Resource Semantic Specification

Path: "grammar/spec/resources.md"
Specification role: Normative resource semantic contract
Specification version: 4.0
Status: Production-target normative specification
Language: Zamani
Grammar technology: ANTLR4
Implementation baseline: Rust 1.97 or later, Rust 2021
Safety requirement: Production Rust MUST use safe Rust; "unsafe" is prohibited
Primary architecture: Target-independent, resource-parametric, capability-driven computation
Scalability principle: No artificial language-level resource ceiling
Portability principle: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

This specification defines the normative semantic model for resources in Zamani.

It establishes how a Zamani program expresses:

- resource requirements;
- resource constraints;
- capabilities;
- resource quantities;
- resource properties;
- budgets;
- preferences;
- hints;
- resource groups;
- resource relationships;
- resource topology requirements;
- scalability intent;
- portability intent;
- target intent;
- resource profiles;
- resource negotiation;
- resource acquisition intent;
- resource reservation intent;
- resource release intent;
- resource derivation;
- resource availability;
- resource capacity;
- performance;
- latency;
- throughput;
- bandwidth;
- energy;
- power;
- reliability;
- resilience;
- cost;
- resource provenance.

The central rule is:

«Zamani source code expresses resource intent. Resource realization is determined by downstream compilation, execution, deployment, and target systems.»

A resource requirement MUST therefore describe what the computation needs, rather than silently selecting a particular physical machine.

The same semantic program MUST be capable of being considered for:

- a tiny embedded system;
- a single CPU;
- a multicore CPU;
- many CPUs;
- a GPU;
- multiple GPUs;
- an FPGA;
- an ASIC;
- an accelerator;
- a QPU;
- multiple QPUs;
- a simulator;
- a workstation;
- an HPC system;
- a cluster;
- a distributed system;
- a cloud environment;
- a heterogeneous system;
- a future computational substrate.

The resource model MUST remain valid as computing architectures evolve.

---

2. Normative Language

The terms:

- MUST
- MUST NOT
- REQUIRED
- SHALL
- SHALL NOT
- SHOULD
- SHOULD NOT
- MAY

are normative.

A requirement marked MUST is mandatory for language conformance.

A requirement marked SHOULD may be relaxed only with a documented implementation reason.

---

3. Authority

This file is the normative semantic authority for the resource model.

It does not replace the authority of:

- "grammar/specification/" for the normative language specification;
- "grammar/DESIGN.md" for repository architecture;
- "grammar/Zamani.g4" for root grammar composition;
- "grammar/antlr/ZamaniLexer.g4" for lexical authority;
- "grammar/resources/*.g4" for resource syntax;
- the AST implementation for structural representation;
- semantic-analysis code for implementation of semantic rules;
- canonical IR specifications for IR representation;
- HAL/runtime systems for physical realization.

The authority chain is:

grammar/specification/
        │
        ▼
language meaning
        │
        ▼
grammar/spec/resources.md
        │
        ▼
grammar/resources/*.g4
        │
        ▼
ANTLR parser
        │
        ▼
domain-neutral AST
        │
        ▼
semantic analysis
        │
        ├── type analysis
        ├── effect analysis
        ├── resource analysis
        ├── capability analysis
        ├── contract analysis
        ├── policy analysis
        └── provenance
        │
        ▼
canonical semantic model
        │
        ├── classical
        ├── quantum
        ├── HDL
        ├── hybrid
        ├── AI/data
        ├── distributed
        ├── networking
        └── future domains
        │
        ▼
canonical IR
        │
        ├── Classical IR
        └── quantum::ir
        │
        ▼
optimization
        │
        ▼
lowering
        │
        ▼
routing / scheduling
        │
        ▼
resilience / recovery / QEC
        │
        ▼
ZQN
        │
        ▼
HAL
        │
        ▼
target realization

No lower layer may silently redefine resource semantics established here.

---

4. Ownership

4.1 This specification owns

This file owns the semantic meaning of:

1. resources;
2. resource identities;
3. resource kinds;
4. resource quantities;
5. resource properties;
6. resource requirements;
7. resource constraints;
8. capabilities as resource-realization predicates;
9. budgets;
10. preferences;
11. hints;
12. resource groups;
13. resource relationships;
14. resource topology intent;
15. resource scalability;
16. resource portability;
17. resource feasibility;
18. resource satisfiability;
19. resource negotiation;
20. resource availability semantics;
21. resource capacity semantics;
22. resource lifecycle intent;
23. resource profiles;
24. resource realization boundaries;
25. resource-related diagnostics;
26. resource-related determinism;
27. resource-related provenance;
28. resource-related failure semantics.

4.2 This specification does not own

It does not own:

- token spelling;
- lexical analysis;
- general expressions;
- general type syntax;
- AST implementation;
- hardware discovery;
- device drivers;
- physical allocation algorithms;
- physical placement;
- quantum routing;
- scheduling;
- compiler optimization;
- QEC implementation;
- ZQN implementation;
- HAL implementation;
- runtime implementation;
- vendor APIs;
- cloud-provider APIs;
- physical addresses;
- physical qubit identifiers;
- memory-bank allocation;
- instruction selection;
- ABI implementation.

These systems consume the resource model.

---

5. Core Resource Model

A Zamani resource intent is conceptually:

ResourceIntent =
    identity
  + kind
  + scope
  + quantity
  + properties
  + requirements
  + constraints
  + capabilities
  + budgets
  + preferences
  + hints
  + topology
  + lifecycle
  + provenance

A realization is conceptually:

ResourceRealization =
    resource intent
  + target environment
  + available resources
  + available capabilities
  + target constraints
  + execution policy
  + security policy
  + compiler decisions
  + runtime state

The realization MUST preserve the semantic meaning of the source program.

Therefore:

resource intent != physical realization

and:

resource identity != physical device identity

unless the program explicitly enters a target-specific mechanism.

---

6. Resource Intent Categories

Zamani MUST distinguish the following categories.

Category| Meaning| Mandatory
Requirement| A condition required for valid realization| Yes
Constraint| A condition that valid realization must obey| Yes
Capability requirement| An ability required from the realization| Yes
Budget| A bounded allocation or consumption objective| Depends on declaration
Preference| A desired realization property| No
Hint| Advisory optimization information| No
Target intent| Information about an intended realization domain| No unless explicitly required
Lifecycle intent| Requested resource-management behavior| Depends on operation
Observation| Information about an observed environment| No
Implementation decision| Downstream realization choice| Not source semantics

These categories MUST NOT be conflated.

In particular:

requirement != preference
preference != hint
constraint != preference
capability != resource
resource != physical device
budget != capacity
capacity != requirement

---

7. Resource Requirement

A requirement states a condition that MUST hold for a realization to be valid.

Examples:

requires qubits >= logical_qubits;
requires memory >= required_memory;
requires bandwidth >= required_bandwidth;
requires nodes >= required_nodes;
requires capability("quantum.measurement");
requires capability("tensor.compute");

A failed mandatory requirement MUST NOT silently become:

- a preference;
- a hint;
- a warning;
- an optimization opportunity;
- a different computation.

The compiler MAY discover an alternative realization, but only if the alternative satisfies the original semantics.

---

8. Resource Constraint

A constraint restricts valid realization.

Examples:

constrain latency <= latency_budget;
constrain energy <= energy_budget;
constrain power <= power_budget;
constrain reliability >= required_reliability;
constrain topology(required_topology);

A hard constraint MUST be satisfied.

If it cannot be satisfied, the realization MUST fail or enter an explicitly permitted failure path.

A backend MUST NOT silently violate a hard constraint.

---

9. Capability

A capability represents an ability or property that a realization may provide.

Examples:

quantum.measurement
quantum.dynamic_control
quantum.error_correction
tensor.compute
vector.compute
distributed.execution
persistent.storage
reconfigurable.hardware
high_precision.arithmetic

Capability identity is open-ended.

The universal grammar MUST NOT permanently enumerate every possible capability.

A new capability SHOULD normally be introduced through:

- a semantic capability registry;
- a dialect;
- target metadata;
- a standard library;
- a domain specification.

A capability requirement:

requires capability("quantum.measurement");

means:

«A valid realization must expose a capability semantically identified as "quantum.measurement".»

It does NOT mean:

- choose QPU 0;
- use a specific vendor;
- use a specific instruction set;
- use a specific physical qubit.

---

10. Resource and Capability Separation

A resource describes something that may be consumed, reserved, measured, or constrained.

A capability describes something that a realization can do or provide.

For example:

memory

is a resource.

tensor.compute

is a capability.

A target may have:

memory capacity = M

and:

capability = tensor.compute

These are different semantic facts.

A program may require both:

requires memory >= required_memory;
requires capability("tensor.compute");

Neither requirement identifies a physical device.

---

11. Open-World Resource Kinds

Resource kinds are open-world.

The language MUST NOT define a closed universal list of:

CPU
GPU
FPGA
ASIC
QPU
TPU
NPU

or any other finite machine taxonomy.

These may be represented as resource kinds or capabilities when useful, but the architecture MUST remain capable of representing future resource classes without redesigning the universal resource model.

Examples:

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

are semantic examples, not a closed enumeration.

Names SHOULD be qualified when ambiguity is possible:

quantum::logical_qubit
hardware::accelerator
network::bandwidth
storage::capacity

---

12. Resource Identity

A resource identity is normally symbolic.

Examples:

compute
memory
storage
quantum
accelerator

A resource identity MAY contain:

- a local identifier;
- a qualified identifier;
- a namespace;
- a domain;
- a dialect namespace;
- a property path.

Identity resolution is semantic, not lexical.

The parser MUST preserve identity structure but MUST NOT decide whether a resource exists on a target.

---

13. Physical Identity

Physical identity is separate from portable resource identity.

Examples include:

device identifiers
node identifiers
physical qubit identifiers
memory addresses
PCI identifiers
vendor-specific identifiers

Physical identities MAY exist in:

- deployment specifications;
- target-specific dialects;
- backend configurations;
- HAL interfaces;
- explicit hardware-control programs.

They MUST NOT become implicit requirements of the universal resource model.

A portable declaration such as:

resource memory;

MUST NOT mean:

memory bank 3

---

14. Resource Quantities

Resource quantities are semantic expressions.

They MAY be:

- literals;
- variables;
- constants;
- generic parameters;
- dependent expressions;
- input-derived expressions;
- compile-time expressions;
- runtime-derived expressions;
- symbolic expressions;
- expressions involving other resource quantities.

Examples:

requires qubits >= logical_qubits;

requires memory >= input.size * element_size;

requires nodes >= parallelism;

requires bandwidth >= workload.bandwidth;

requires storage >= dataset.size;

The grammar MUST NOT impose a maximum quantity.

---

15. Quantity Representation

A source-level quantity MUST be treated as a semantic quantity rather than being prematurely tied to an implementation integer type.

For example:

requires memory >= required_memory;

does NOT imply:

required_memory : u64

The compiler MAY represent a quantity using:

- a machine integer;
- arbitrary precision;
- a symbolic representation;
- a rational representation;
- an interval;
- a domain-specific numeric representation.

The representation MUST preserve language semantics.

Overflow, truncation, wrapping, clamping, or loss of precision MUST NOT silently change the meaning of a resource requirement.

---

16. Units and Dimensions

Resource quantities MAY carry units and dimensions.

Examples include:

bytes
bits
operations
events
seconds
cycles
joules
watts
bytes/second
bits/second
operations/second

Unit semantics belong to the canonical type/quantity system.

Resource grammar MUST reuse canonical quantity and expression semantics.

Resource grammar MUST NOT introduce an incompatible numeric/unit system.

Conversions MUST preserve semantic correctness.

A potentially lossy implicit conversion SHOULD be rejected.

---

17. Capacity

Capacity describes how much of a resource is available or potentially available in a particular realization context.

Examples:

capacity
available_capacity
memory_capacity
storage_capacity

Capacity is contextual.

A target with greater capacity MAY satisfy the same requirement as a smaller target.

A target with insufficient capacity MUST NOT satisfy a mandatory capacity requirement.

Capacity is not itself a source-level universal machine limit.

---

18. Availability

Availability describes whether a resource can currently participate in realization.

Availability MAY change because of:

- contention;
- scheduling;
- reservation;
- failure;
- maintenance;
- thermal state;
- power state;
- runtime policy;
- dynamic scaling;
- distributed failures;
- resource health.

Availability is therefore generally contextual and potentially dynamic.

The grammar represents source intent.

The resource-management system evaluates actual availability.

---

19. Resource Scope

Resource intent MUST have a semantic scope.

Possible scopes include:

program
module
function
block
task
actor
operation
transaction
execution
deployment
target
resource-group

The exact syntactic forms are owned by the relevant grammar.

Semantically:

resource requirement scope

determines where the requirement applies.

A function-local resource requirement MUST NOT automatically become a global machine requirement unless its semantics explicitly require propagation.

---

20. Requirement Propagation

Resource requirements MAY propagate through semantic dependencies.

For example:

function A()
    requires memory >= m;

If:

function B()
    calls A();

the compiler MAY infer that execution of "B" requires resources sufficient for "A".

The exact propagation mechanism belongs to semantic analysis.

Propagation MUST preserve:

- scope;
- conditionality;
- multiplicity;
- concurrency;
- resource lifetime;
- policy;
- provenance.

A requirement MUST NOT be duplicated merely because the same semantic resource is referenced multiple times unless the resource semantics require independent allocation.

---

21. Resource Multiplicity

A resource requirement may represent:

- one resource;
- a quantity of resources;
- a set of resources;
- a pool;
- a group;
- a distributed collection;
- a hierarchical resource.

The resource model MUST NOT require a fixed cardinality.

For example:

requires nodes >= required_nodes;

does not imply a maximum node count.

---

22. Resource Groups

A resource group represents a semantically related collection of resources.

A group MAY express:

- aggregate capacity;
- shared properties;
- common constraints;
- common capabilities;
- topology;
- locality;
- redundancy;
- placement relationships.

A group is not automatically a physical cluster.

A group may be realized by:

- one device;
- many devices;
- a virtual resource;
- a distributed collection;
- a heterogeneous collection.

---

23. Resource Composition

Resources MAY be composed hierarchically.

Conceptually:

system
 ├── compute
 ├── memory
 ├── network
 └── accelerator

or:

quantum_system
 ├── logical_qubits
 ├── physical_qubits
 ├── control
 └── measurement

The hierarchy is semantic.

It does not require a particular physical architecture.

---

24. Resource Relationships

Resource relationships may express:

- dependency;
- containment;
- adjacency;
- locality;
- affinity;
- anti-affinity;
- sharing;
- exclusivity;
- replication;
- redundancy;
- coupling;
- ordering;
- topology.

Relationship semantics MUST remain separate from physical realization.

---

25. Topology

A topology requirement expresses structural relationships among resources.

Examples:

requires topology(required_topology);

or equivalent structured resource expressions.

Topology MAY describe:

- connectivity;
- locality;
- hierarchy;
- communication relationships;
- quantum coupling;
- memory locality;
- accelerator interconnect;
- distributed placement relationships.

The topology model MUST be extensible.

The grammar MUST NOT enumerate fixed topology sizes.

---

26. Locality

Locality MAY be expressed as a resource property or constraint.

Examples include:

prefer locality;
constrain locality(required_locality);
requires capability("memory.local_access");

Locality semantics may apply to:

- memory;
- computation;
- network;
- quantum resources;
- distributed execution;
- accelerators.

The actual placement remains a compiler/runtime decision.

---

27. Affinity and Anti-Affinity

Resource intent MAY express that resources should:

- be close;
- share a resource domain;
- share memory;
- share an interconnect;
- avoid sharing a physical resource;
- avoid a failure domain.

These are semantic constraints or preferences depending on declaration.

A preference MAY be violated.

A hard constraint MUST NOT be violated.

---

28. Performance

Performance is a semantic property associated with a resource or resource realization.

It MAY include:

- throughput;
- latency;
- operations per unit time;
- communication rate;
- memory bandwidth;
- execution rate;
- response time;
- power efficiency.

Performance MUST NOT be reduced to a single universal machine metric.

A program may express:

requires performance >= required_performance;

or:

prefer performance;

The exact metric MUST be resolved by semantic typing and resource-property metadata.

---

29. Latency

Latency represents elapsed time between semantically relevant events.

It may apply to:

- computation;
- communication;
- storage;
- memory;
- accelerator invocation;
- quantum operations;
- distributed operations.

Example:

constrain latency <= latency_budget;

The resource model does not define a universal minimum or maximum latency.

---

30. Throughput

Throughput represents work, data, or events processed per semantic time unit.

Examples:

requires throughput >= required_throughput;
prefer throughput;

The relevant unit MUST be determined by the resource/property type.

---

31. Bandwidth

Bandwidth may apply to:

- memory;
- network;
- interconnect;
- storage;
- accelerator links;
- quantum control/data paths.

Example:

requires bandwidth >= required_bandwidth;

Bandwidth MUST remain symbolic and extensible.

---

32. Energy

Energy is a resource or resource property.

Example:

constrain energy <= energy_budget;

Energy semantics MUST NOT assume a particular hardware technology.

---

33. Power

Power describes energy consumption rate or another explicitly defined power metric.

Example:

constrain power <= power_budget;

Power constraints MAY influence target selection and scheduling.

They MUST NOT themselves select a physical device.

---

34. Reliability

Reliability describes an allowed or required level of successful operation according to a defined semantic metric.

Example:

constrain reliability >= required_reliability;

Reliability semantics MAY be domain-specific.

The common resource model provides the property boundary; the domain defines the precise metric.

---

35. Resilience

Resilience describes behavior under resource degradation or failure.

Resource semantics MAY express:

- redundancy;
- fallback requirements;
- recoverability;
- tolerated degradation;
- required availability;
- failure-domain separation.

Actual recovery algorithms belong to execution/resilience subsystems.

---

36. Cost

Cost may represent:

- computational cost;
- energy cost;
- monetary cost;
- communication cost;
- deployment cost;
- resource-allocation cost.

Cost MUST be explicitly typed or qualified.

A cost preference is advisory unless declared as a constraint.

Example:

prefer cost;
constrain cost <= budget;

---

37. Budgets

A budget defines a bounded resource consumption or objective.

A budget MUST be distinguished from available capacity.

For example:

budget energy <= energy_limit;

means:

«The computation's permitted energy expenditure is bounded.»

It does not mean:

«The target has exactly "energy_limit" energy available.»

Likewise:

memory capacity

and:

memory budget

are distinct concepts.

---

38. Budget Scope

Budgets MAY apply to:

- program;
- module;
- function;
- task;
- actor;
- operation;
- execution;
- deployment;
- resource group.

Budget scope MUST be explicit or deterministically inherited.

---

39. Budget Composition

When multiple budgets apply, semantic analysis MUST determine whether they are:

- nested;
- cumulative;
- independent;
- shared;
- exclusive.

The language MUST NOT silently assume that two references to a budget create two independent physical allocations.

---

40. Preferences

Preferences describe desired realization properties.

Examples:

prefer low_latency;
prefer energy_efficiency;
prefer locality;
prefer accelerator;

Preferences are advisory.

A compiler MAY violate a preference to satisfy:

- requirements;
- constraints;
- correctness;
- security;
- policy;
- determinism;
- reliability;
- availability.

A preference MUST NOT silently become a requirement.

---

41. Preference Ordering

Multiple preferences MAY be ordered or weighted where the language provides such semantics.

The ordering MUST be deterministic.

If two preferences conflict, the declared precedence mechanism determines the result.

If no precedence exists, the implementation MUST NOT claim that one preference has priority merely because it happened to be encountered first.

---

42. Hints

Hints are weaker than preferences.

A hint provides optimization information but carries no correctness requirement.

Example:

hint locality;
hint vectorization;
hint parallelism;

Ignoring a hint MUST NOT make a valid program invalid.

Hints MUST NOT change program semantics.

---

43. Requirement, Preference, and Hint Example

Consider:

requires capability("tensor.compute");

constrain memory >= required_memory;

prefer low_latency;

hint locality;

The meanings are:

capability("tensor.compute")
    mandatory

memory >= required_memory
    mandatory

low_latency
    desired

locality
    advisory

A backend MAY choose a slower realization if it still satisfies the mandatory conditions.

---

44. Capability Versioning

Capabilities MAY carry version information.

Example:

requires capability("quantum.dynamic_control") version >= required_version;

Capability version semantics belong to the capability registry.

The resource grammar MUST NOT hard-code a universal capability-version universe.

Version comparison MUST be deterministic.

---

45. Capability Parameters

Capabilities MAY have semantic parameters.

Examples:

capability("quantum.measurement", measurement_mode);
capability("tensor.compute", tensor_format);
capability("network.transport", protocol);

Parameters MUST be represented as semantic expressions.

The grammar MUST NOT interpret vendor-specific meaning.

---

46. Capability Provider

A capability MAY be supplied by:

- a CPU;
- a GPU;
- an accelerator;
- an FPGA;
- a QPU;
- a simulator;
- a distributed service;
- a runtime service;
- a library;
- a software implementation.

The provider is a realization detail unless explicitly requested by a target-specific program.

---

47. Capability Discovery

Capability discovery occurs after parsing.

The pipeline is:

source capability requirement
        ↓
semantic capability identity
        ↓
target capability registry
        ↓
available capability set
        ↓
feasibility analysis

The parser MUST NOT query:

- hardware;
- operating-system APIs;
- network services;
- device drivers;
- cloud APIs.

---

48. Resource Discovery

Resource discovery follows the same boundary.

The grammar describes:

what is required

The environment determines:

what exists

This separation is essential for deterministic parsing and POCO-REAF.

---

49. Resource Negotiation

Negotiation occurs when multiple valid realizations are possible.

Conceptually:

source intent
      ↓
requirements
      ↓
constraints
      ↓
capabilities
      ↓
available resources
      ↓
preferences
      ↓
policies
      ↓
candidate realizations
      ↓
selected realization

Negotiation MUST preserve mandatory semantics.

Preferences MAY influence candidate ordering.

Hints MAY influence optimization.

---

50. Negotiation Failure

If no realization satisfies all mandatory requirements and constraints, negotiation MUST report failure.

It MUST NOT silently:

- reduce resource quantities;
- remove capabilities;
- ignore constraints;
- change semantics;
- select an incompatible target.

A recovery path MAY exist if explicitly permitted by the program's semantics or policy.

---

51. Fallbacks

A resource policy MAY define fallback behavior.

A fallback MUST be explicit.

Examples include:

preferred accelerator
    ↓
alternative accelerator
    ↓
CPU implementation
    ↓
simulation

A fallback MUST preserve semantic correctness.

If the fallback changes semantics, it requires an explicit semantic mode permitting that change.

---

52. Target Intent

Target intent provides information about preferred or required realization domains.

Examples:

prefer target domain;
requires capability("quantum.execution");

Portable source should generally describe capabilities and resources rather than vendor/device identity.

Target-specific declarations belong to the appropriate target/deployment dialect.

---

53. Target Independence

The universal resource model MUST remain target-independent.

The source MUST NOT need to know:

- CPU model;
- GPU model;
- FPGA part number;
- ASIC revision;
- QPU vendor;
- physical qubit number;
- memory bank;
- PCI address;
- cloud provider;
- physical node identifier.

Those belong to target-specific layers.

---

54. Resource Lifecycle

The repository's resource grammar provides lifecycle-oriented constructs including:

- reservation;
- acquisition;
- release;
- derivation;
- grouping.

Their semantics are defined here.

These constructs express resource-management intent.

They do not implement allocation themselves.

---

55. Reservation

Reservation requests that a resource be held for a specified scope or execution period.

Reservation MAY be:

- mandatory;
- advisory;
- conditional;
- policy-controlled.

Reservation does not guarantee success.

The resource manager determines whether the requested reservation can be realized.

---

56. Acquisition

Acquisition requests access to a resource.

Acquisition MAY be:

- static;
- dynamic;
- lazy;
- conditional.

Acquisition MUST respect:

- requirements;
- capabilities;
- security policy;
- authorization;
- resource ownership;
- lifecycle rules.

---

57. Release

Release expresses that a resource is no longer required by the relevant semantic scope.

Release MUST NOT imply a specific operating-system primitive.

The runtime or resource manager determines physical release.

---

58. Resource Derivation

A resource MAY be derived from another resource.

Examples:

logical resource
    derived from
physical resource

or:

virtual accelerator
    derived from
hardware accelerator

Derivation MUST preserve provenance.

A derived resource MUST identify its semantic relationship to the source resource where required for correctness or auditability.

---

59. Resource Ownership

Ownership describes which semantic scope is responsible for a resource.

Ownership may belong to:

- a program;
- module;
- task;
- actor;
- execution;
- resource group;
- external environment.

Ownership MUST NOT be confused with physical device identity.

---

60. Resource Sharing

Resources MAY be shared.

Sharing semantics MUST distinguish:

- read sharing;
- exclusive use;
- time sharing;
- partitioning;
- replication;
- aliasing;
- virtualized access.

The exact behavior is determined by resource semantics and policies.

---

61. Resource Exclusivity

A constraint MAY require exclusive access.

Example semantic intent:

constrain exclusive(resource);

Exclusivity is a realization constraint.

It does not identify a physical resource unless explicitly target-specific.

---

62. Resource Partitioning

A resource MAY be partitioned into logical resources.

Examples:

memory
    ↓
logical memory regions

or:

accelerator
    ↓
logical execution partitions

Partitioning semantics MUST preserve total resource accounting where accounting is relevant.

---

63. Resource Accounting

Resource accounting MUST distinguish:

requested
allocated
reserved
consumed
available
released
failed

These states MUST NOT be conflated.

For example:

requested != allocated
allocated != consumed
capacity != availability

---

64. Dynamic Resources

Resource quantities MAY depend on runtime state.

Examples:

requires memory >= runtime_required_memory;
requires nodes >= current_parallelism;

A dynamic requirement MUST be evaluated at the appropriate execution boundary.

Dynamic evaluation MUST preserve deterministic semantics where the language contract requires determinism.

---

65. Runtime Resource Changes

A runtime MAY observe:

- resource loss;
- resource addition;
- resource degradation;
- resource recovery;
- resource contention.

Such changes MAY cause:

- retry;
- recovery;
- rescheduling;
- migration;
- fallback;
- failure.

The runtime MUST preserve declared semantics.

---

66. Resource State

A resource may have states such as:

unknown
available
reserved
allocated
busy
degraded
unavailable
recovering
released
retired

The precise runtime state machine belongs to resource management.

The language-level resource model MUST remain compatible with such state transitions.

---

67. Resource Failure

A resource failure occurs when an expected realization condition can no longer be maintained.

Failure MUST be classified sufficiently for downstream recovery.

Examples:

requirement_unsatisfied
capability_unavailable
capacity_exhausted
resource_lost
resource_degraded
policy_denied
reservation_failed
acquisition_failed
timeout
target_unavailable

---

68. Semantic Failure vs Implementation Failure

A compiler/runtime MUST distinguish:

program semantic invalidity

from:

current target inability to realize valid semantics

For example:

requires capability("quantum.measurement");

on a target without measurement capability does not make the source program syntactically invalid.

It means that the selected target cannot realize it.

---

69. Diagnostics

Resource diagnostics MUST identify, where applicable:

- requirement;
- resource identity;
- requested quantity;
- available quantity;
- capability;
- constraint;
- policy;
- target;
- scope;
- reason;
- source span.

A diagnostic SHOULD distinguish:

unsatisfied requirement

from:

unavailable realization

and:

invalid resource expression

---

70. No Silent Degradation

The compiler MUST NOT silently convert:

requirement

into:

preference

or:

constraint

into:

hint

or:

capability requirement

into:

best effort

or:

resource allocation failure

into:

different computation

unless an explicit semantic fallback permits it.

---

71. Classical Computing Integration

Classical computation may consume:

- compute resources;
- memory;
- storage;
- vector resources;
- tensor resources;
- parallelism;
- accelerators;
- network resources.

Examples:

requires memory >= working_set;
requires capability("vector.compute");
requires capability("tensor.compute");

The classical subsystem owns classical computation semantics.

The resource subsystem owns common resource semantics.

---

72. Quantum Integration

Quantum programs MAY express requirements involving:

- logical qubits;
- physical qubits;
- measurement;
- dynamic control;
- error correction;
- coherence-related properties;
- connectivity;
- control resources;
- quantum memory;
- quantum communication.

Examples:

requires qubits >= logical_qubits;

requires capability("quantum.measurement");

requires capability("quantum.dynamic_control");

requires capability("quantum.error_correction");

The resource system MUST NOT enumerate quantum gates.

---

73. Quantum IR Boundary

Resource requirements MUST NOT create a competing quantum IR.

The semantic path is:

quantum source
    ↓
domain-neutral AST
    ↓
quantum semantic analysis
    ↓
resource/capability analysis
    ↓
quantum::ir
    ↓
optimization
    ↓
routing
    ↓
scheduling
    ↓
resilience / QEC
    ↓
ZQN
    ↓
HAL

"quantum::ir" remains the canonical quantum semantic boundary.

---

74. Quantum Topology

Quantum topology requirements MAY express:

- connectivity;
- locality;
- coupling;
- logical-to-physical relationships;
- communication topology.

The resource grammar MUST NOT encode a fixed number of physical qubits or a fixed topology size.

Topology realization belongs to routing and target systems.

---

75. HDL Integration

HDL programs may express requirements involving:

- logic resources;
- memory;
- timing;
- bandwidth;
- I/O;
- reconfigurable resources;
- hardware capabilities;
- power;
- thermal constraints;
- reliability.

The resource model MUST remain independent of a specific HDL implementation.

HDL synthesis determines physical realization.

---

76. Hardware Integration

Hardware resource descriptions MAY expose:

- compute capacity;
- memory;
- storage;
- interconnect;
- accelerator capabilities;
- power;
- thermal properties;
- reliability;
- topology.

Hardware discovery remains outside the grammar.

---

77. Accelerator Integration

An accelerator is not required to be a particular hardware technology.

The semantic model SHOULD prefer:

requires capability("accelerator.compute");

over embedding:

requires GPU vendor X;

unless a target-specific dialect explicitly requires the latter.

---

78. Distributed Integration

Distributed programs may express:

requires nodes >= required_nodes;
requires capability("distributed.execution");
requires bandwidth >= required_bandwidth;

They may also constrain:

- topology;
- locality;
- replication;
- failure domains;
- consistency;
- communication.

No maximum node count is permitted in universal semantics.

---

79. Networking Integration

Network resources MAY include:

- bandwidth;
- latency;
- endpoints;
- connectivity;
- transport capabilities;
- reliability;
- security properties.

Network implementation remains outside the resource grammar.

---

80. AI and Data Integration

AI/data computations may require:

- memory;
- tensor capacity;
- accelerator capability;
- parallelism;
- storage;
- bandwidth;
- model-serving capabilities.

Examples:

requires capability("tensor.compute");
requires capability("model.inference");
requires memory >= model_memory;

Application-specific domains remain extensible through semantic identifiers rather than requiring universal resource keywords.

---

81. Effect Integration

Resource requirements may interact with effects.

Examples:

effect network
    → requires network capability

effect quantum
    → requires quantum capability

effect measurement
    → requires measurement capability

The effect subsystem owns effects.

The resource subsystem owns resource requirements.

Neither subsystem may silently become the authority of the other.

---

82. Policy Integration

Policies MAY constrain:

- which resources may be used;
- which capabilities may be consumed;
- resource budgets;
- target selection;
- geographic or administrative restrictions;
- security boundaries;
- acquisition;
- release;
- fallback;
- adaptation.

Policy semantics belong to the policy subsystem.

Resource analysis consumes policy decisions.

---

83. Contract Integration

Contracts MAY express resource invariants.

Examples include:

requires memory >= required_memory;
ensures resource_usage <= budget;
invariant resource_requirement_satisfied;

Contract semantics belong to validation/contracts.

Resource semantics define the resource predicates that contracts may reference.

---

84. Provenance

Resource decisions SHOULD preserve provenance where required.

Relevant provenance may include:

source requirement
resource identity
capability identity
target capability
constraint
policy
negotiation result
selected realization
transformation

For example:

source:
    requires capability("tensor.compute")

decision:
    selected target capability tensor.compute

realization:
    target-specific accelerator

reason:
    capability satisfied

The exact physical target identity belongs to downstream provenance.

---

85. Determinism

Resource semantic normalization MUST be deterministic for the same:

- source;
- language version;
- grammar version;
- dialect versions;
- semantic environment;
- capability registry;
- resource environment;
- policy configuration.

Equivalent semantic inputs MUST normalize equivalently.

Source ordering MAY be retained for diagnostics and tooling.

Canonical semantic ordering MUST NOT depend on nondeterministic iteration.

---

86. Reproducibility

Resource-aware compilation SHOULD record the resource information that affected a build when reproducibility requires it.

Relevant information may include:

- resource requirements;
- capabilities considered;
- selected policies;
- target characteristics;
- compiler version;
- language version;
- dialect versions;
- realization decisions.

Reproducibility metadata MUST NOT become part of the program's portable semantics unless explicitly declared.

---

87. Simulation

Simulation is a realization strategy.

A valid resource requirement may be satisfied by a simulator only when the simulator provides the required semantic capabilities and the program permits that realization.

For example:

requires capability("quantum.measurement");

does not automatically prohibit simulation.

But the simulator MUST actually provide the required semantic behavior.

---

88. Simulation Must Not Be Silent Semantic Substitution

If a program explicitly requires physical execution, simulation MUST NOT silently satisfy that requirement.

Likewise, if a program permits simulation, the simulator MAY be selected.

The distinction belongs to:

- resource requirements;
- capabilities;
- policies;
- execution mode.

---

89. Adaptive Execution

Resource-aware execution MAY adapt to changing resources.

A valid adaptive process is:

observe resource state
        ↓
evaluate requirements
        ↓
evaluate policy
        ↓
select permitted alternative
        ↓
verify semantic preservation
        ↓
continue execution

Adaptation MUST NOT silently change the program's declared meaning.

---

90. Scaling Semantics

The resource system is explicitly designed for scaling from tiny systems to arbitrarily large systems subject only to actual resource availability and implementation capacity.

The language itself MUST NOT define the upper bound.

For example:

requires qubits >= n;

means that "n" determines the requirement for that program instance.

The language does not define:

MAX_QUBITS

Likewise:

requires nodes >= n;

does not establish a maximum node count.

---

91. Meaning of "Infinity"

Within this specification:

«Unbounded means that the language architecture does not impose an artificial finite maximum.»

It does NOT mean that:

- physical hardware is infinite;
- memory is infinite;
- the compiler has infinite memory;
- the runtime has infinite execution time;
- the network has infinite bandwidth.

Actual execution is constrained by real resources.

The language does not hard-code those constraints.

---

92. Absolute Hard-Coding Prohibition

The universal resource model MUST NOT contain:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_QPUS
MAX_ACCELERATORS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_RESOURCE_GROUPS

Nor may equivalent indirect restrictions be introduced.

The prohibition includes:

- fixed parser alternatives;
- fixed array sizes;
- fixed enumeration counts;
- fixed AST fields representing a finite universe;
- fixed capability lists;
- fixed topology sizes.

---

93. Program Literals Are Not Language Limits

A source program MAY legitimately contain:

1024

or:

1_000_000

or any other valid program value.

For example:

requires qubits >= 1024;

means:

«This particular computation requires at least 1024 qubits under the stated resource semantics.»

It does NOT mean:

«Zamani supports only 1024 qubits.»

---

94. Implementation Limits

Implementations MAY have configurable protection limits for:

- parser memory;
- compiler memory;
- compilation time;
- AST size;
- diagnostic count;
- nesting depth;
- resource-expression complexity;
- solver complexity.

Such limits MUST be:

1. implementation limits;
2. documented;
3. distinguishable from language semantics;
4. configurable where appropriate;
5. diagnosed explicitly.

They MUST NOT be represented as universal Zamani resource limits.

---

95. Resource Solving

Resource satisfiability MAY require:

- symbolic reasoning;
- constraint solving;
- capability matching;
- unit analysis;
- topology matching;
- policy evaluation;
- scheduling analysis.

The resource grammar does not perform solving.

Semantic analysis performs the necessary reasoning.

---

96. Resource Expression Evaluation

Resource expressions MAY be:

static
symbolic
compile-time
runtime
target-dependent

The semantic layer MUST know which evaluation category applies.

The parser MUST remain independent of evaluation.

---

97. Resource Expression Safety

Resource expressions MUST NOT cause source parsing to:

- execute user code;
- access hardware;
- access the filesystem;
- access the network;
- inspect secrets;
- mutate runtime state.

Compile-time evaluation, where supported, belongs to the controlled metaprogramming/compile-time subsystem.

---

98. Resource Constraints and Types

Resource expressions MUST participate in normal type checking.

Examples:

memory >= duration

should be rejected if the semantic types are incompatible.

Unit-compatible expressions MAY be compared.

Unit-incompatible expressions MUST be rejected unless an explicit valid conversion exists.

---

99. Resource Quantity Overflow

The compiler MUST detect representational overflow when it occurs.

It MUST NOT silently:

- wrap;
- truncate;
- saturate;
- reinterpret.

Where symbolic or arbitrary-precision evaluation is possible, the compiler SHOULD preserve the semantic expression rather than prematurely reducing it to an insufficient representation.

---

100. Resource Contradictions

Semantic analysis MUST detect contradictions where they can be determined.

Examples:

requires memory >= 100;
constrain memory <= 10;

or:

requires capability("A");
forbid capability("A");

A contradiction that is statically provable MUST produce a diagnostic.

A contradiction that depends on runtime state MAY remain a runtime feasibility condition.

---

101. Conditional Requirements

Requirements MAY be conditional.

Conceptually:

requires condition -> resource_requirement;

The condition MUST be semantically typed.

A conditional requirement applies only when its condition is true.

This allows portable programs to express requirements dependent on:

- input size;
- execution mode;
- selected algorithm;
- data shape;
- runtime state;
- policy.

---

102. Alternative Requirements

A program MAY express alternatives where supported.

Conceptually:

requires one_of {
    capability("A");
    capability("B");
}

Alternative semantics MUST clearly distinguish:

any valid alternative

from:

all requirements

No ambiguity is permitted.

---

103. Resource Sets

A resource set represents a collection of semantically related resources.

Set semantics MAY be used for:

- pools;
- groups;
- distributed resources;
- alternative resources;
- replicated resources.

The number of members MUST be unbounded by language design.

---

104. Resource Aggregation

Resource quantities MAY aggregate over a set.

For example:

aggregate(memory) >= required_memory;

The exact aggregation function is semantic.

Possible aggregation forms include:

- sum;
- minimum;
- maximum;
- average;
- count;
- bottleneck;
- domain-specific aggregate.

The aggregation function MUST be explicit where ambiguity could affect correctness.

---

105. Resource Consumption

Consumption represents actual or planned use.

A resource requirement is not necessarily equivalent to consumption.

For example:

requires memory >= working_set;

describes minimum feasible capacity.

Actual consumption may be smaller, equal, or vary dynamically depending on execution.

---

106. Peak Resource Usage

A program MAY distinguish:

minimum required capacity

from:

peak consumption

This distinction is important for memory, storage, bandwidth, concurrency, and other resources.

---

107. Concurrency and Resources

Concurrent execution may require resources simultaneously.

Resource analysis MUST account for:

- task overlap;
- actor concurrency;
- parallel execution;
- synchronization;
- resource sharing;
- exclusivity.

A sequential requirement MUST NOT automatically be multiplied by concurrency unless semantic analysis determines that the resources are simultaneously needed.

---

108. Distributed Resources

Distributed resource requirements MAY account for:

- node count;
- aggregate memory;
- aggregate compute;
- communication;
- topology;
- locality;
- replication;
- failure domains.

A distributed requirement MUST NOT require a specific physical cluster.

---

109. Heterogeneous Resources

A program may require a heterogeneous combination.

Example:

requires capability("tensor.compute");
requires capability("quantum.measurement");
requires memory >= required_memory;

The realization may involve:

- CPU;
- GPU;
- accelerator;
- QPU;
- simulator;
- distributed components.

The resource model MUST permit heterogeneous realization.

---

110. Resource Co-Location

A program MAY require resources to be co-located.

This is a topology/locality constraint.

The constraint MUST be evaluated by the realization system.

The grammar MUST NOT encode physical addresses.

---

111. Resource Separation

A program MAY require resources to reside in separate failure or security domains.

This is useful for:

- redundancy;
- fault tolerance;
- security;
- distributed resilience.

Again, the program expresses intent; the target determines physical realization.

---

112. Resource Redundancy

A program may require redundant resources.

Examples:

requires redundancy >= required_redundancy;

Redundancy is semantic intent.

The actual devices or replicas are selected downstream.

---

113. Resource Resilience

Resource resilience MAY require the realization to tolerate:

- resource loss;
- degraded capability;
- node failure;
- accelerator failure;
- network failure;
- quantum resource degradation.

The resilience subsystem implements recovery.

Resource semantics describe required resilience properties.

---

114. Resource Health

A resource realization MAY expose health information.

Health information MAY affect:

- availability;
- reliability;
- resilience;
- scheduling;
- target selection.

Health discovery is runtime/target responsibility.

---

115. Resource Profiles

A resource profile groups related resource properties.

A profile MAY describe:

compute
memory
bandwidth
latency
energy
power
reliability
capabilities
topology

A profile is a semantic description, not necessarily a physical device record.

---

116. Profile Composition

Profiles MAY inherit or compose other profiles.

Composition MUST preserve deterministic property resolution.

Conflicting properties MUST have defined precedence.

No conflict may be resolved merely by parser import order.

---

117. Resource Metadata

Resource metadata MAY include:

- names;
- descriptions;
- provenance;
- versions;
- domains;
- annotations;
- policies;
- implementation hints.

Metadata MUST NOT silently alter mandatory resource semantics.

---

118. Resource Attributes

Attributes are owned syntactically by the resource grammar.

Semantically, an attribute is either:

- standard metadata;
- a semantic property;
- a dialect extension;
- an implementation annotation.

Unknown attributes MUST have defined behavior.

An unknown implementation annotation MAY be ignored where permitted.

An unknown mandatory semantic attribute MUST produce a diagnostic.

---

119. Dialect Extensions

Resource dialects MAY introduce:

- new resource kinds;
- properties;
- capabilities;
- metrics;
- topology models;
- target-specific resource metadata.

A dialect MUST declare:

dialect identity
version
resource extensions
semantic contract
compatibility
AST mapping
IR mapping
tests

A dialect MUST NOT silently redefine standard resource semantics.

---

120. Vendor Extensions

Vendor-specific resource information belongs in a vendor namespace or target-specific dialect.

Example conceptual form:

vendor::resource::property

Vendor extensions MUST NOT become mandatory dependencies of portable programs unless explicitly requested.

---

121. Compatibility

Resource semantics are versioned.

Changes MUST be classified as:

- additive;
- clarifying;
- compatible;
- deprecated;
- behavior-changing;
- breaking.

An additive resource kind or capability SHOULD NOT require modification of the universal resource model.

---

122. Deprecation

Deprecated resource constructs MUST remain semantically well-defined during their compatibility period.

The compiler SHOULD emit a migration diagnostic.

Deprecation MUST NOT silently reinterpret an old resource construct as a different resource.

---

123. Experimental Features

Experimental resource features MUST be explicitly marked through the language's feature-status mechanism.

Experimental status MUST NOT require a separate resource language.

---

124. Resource Provenance

Resource transformations SHOULD record:

source requirement
normalized requirement
derived requirement
capability match
constraint decision
negotiation decision
target realization
fallback
resource transformation

This is particularly important for:

- optimization;
- distributed execution;
- quantum routing;
- accelerator selection;
- adaptive execution.

---

125. Quantum Resource Provenance

For quantum realization, provenance MAY include:

logical resource
physical realization
topology
mapping
routing transformation
resource decision

The physical mapping remains downstream of "quantum::ir".

---

126. Hardware Provenance

Hardware realization provenance MAY record:

abstract resource
target capability
target resource
allocation
placement
backend
HAL

This information MUST NOT become part of portable source semantics unless explicitly requested.

---

127. Compilation Provenance

Resource-aware compilation SHOULD be capable of recording:

source language version
grammar version
resource requirements
capabilities
policies
target characteristics
compiler version
dialect versions
realization decisions

---

128. Resource Negotiation and Policies

Negotiation MUST evaluate policies before making decisions that policy restricts.

Conceptually:

requirements
      ↓
constraints
      ↓
capabilities
      ↓
policy
      ↓
candidate realization
      ↓
preferences
      ↓
selection

Policy MUST NOT modify the source program's semantics without an explicit language rule permitting such behavior.

---

129. Security

Resource operations may be security-sensitive.

Examples:

- acquisition;
- release;
- device access;
- native execution;
- network resources;
- privileged hardware;
- confidential resources.

Security policy MUST be evaluated separately from resource syntax.

The resource grammar MUST NOT grant authority merely by naming a resource.

---

130. Authorization

Resource acquisition MAY require authorization.

Authorization belongs to the security/policy subsystem.

A resource declaration MUST NOT itself constitute authorization.

---

131. Sandbox Integration

A sandbox MAY restrict:

- resources;
- capabilities;
- network;
- filesystem;
- native interfaces;
- devices;
- accelerators.

A resource requirement that cannot be satisfied inside the sandbox MUST fail according to the normal resource-feasibility rules.

---

132. Safe Rust Requirement

The Rust implementation of resource semantics MUST:

- use Rust 1.97 or later;
- use Rust 2021 or later as required by repository compatibility;
- contain no production "unsafe";
- avoid requiring unsafe FFI wrappers for ordinary resource analysis;
- avoid raw-pointer resource representations;
- use safe ownership and borrowing;
- use explicit error handling.

Where an external API requires unsafe internally, the Zamani resource subsystem MUST NOT make unsafe code part of its own semantic implementation contract.

---

133. Rust Implementation Boundary

This specification does not prescribe a particular Rust data structure.

The implementation MAY use safe structures such as:

Vec
VecDeque
HashMap
BTreeMap
BTreeSet
Option
Result
Arc
Mutex
RwLock

where appropriate.

The semantic model MUST remain independent of the chosen representation.

---

134. No Unsafe Grammar Dependency

ANTLR grammar files MUST contain no:

- embedded Rust actions;
- unsafe actions;
- filesystem operations;
- network operations;
- hardware discovery;
- runtime execution.

Resource discovery belongs to later compiler/runtime components.

---

135. Parser Determinism

Parsing MUST depend only on:

- source text;
- grammar version;
- lexical configuration;
- explicitly selected dialects.

Parsing MUST NOT depend on:

- hardware;
- current resource availability;
- wall-clock time;
- random numbers;
- filesystem state;
- network state;
- runtime state.

---

136. Semantic Determinism

Given the same semantic environment, resource normalization and validation MUST be deterministic.

Equivalent resource sets MUST normalize equivalently.

For example:

{memory, bandwidth, compute}

and:

{compute, memory, bandwidth}

MUST have equivalent semantic meaning when the constructs are sets.

---

137. Parallel Compilation

Resource analysis SHOULD support parallel analysis of independent semantic units.

The resource model MUST NOT require global mutable state.

Resource registries MUST have defined synchronization semantics where shared across compiler phases.

---

138. Compiler Resource Limits

Compiler implementations MAY impose operational limits to protect against denial-of-service conditions.

Examples:

- maximum parser work;
- maximum AST nodes;
- maximum constraint-solving work;
- maximum semantic-analysis memory;
- maximum diagnostic volume.

These are compiler policies, not Zamani language limits.

---

139. No Artificial Grammar Limits

Resource grammar MUST NOT encode:

exactly N resources
exactly N requirements
exactly N capabilities
exactly N nodes
exactly N devices
exactly N qubits

unless "N" is an actual program value or semantic constraint.

---

140. Resource Grammar Integration

The existing resource grammar hierarchy remains the syntactic owner.

The intended division is:

grammar/resources/resources.g4
    resource subsystem composition

grammar/resources/resource-expressions.g4
    resource expressions

grammar/resources/requirements.g4
    resource requirements

grammar/resources/capabilities.g4
    capability syntax

grammar/resources/constraints.g4
    constraints

grammar/resources/budgets.g4
    budgets

grammar/resources/preferences.g4
    preferences

grammar/resources/hints.g4
    hints

grammar/resources/negotiation.g4
    negotiation

grammar/resources/scalability.g4
    scalability intent

Other existing resource grammar files remain responsible for their declared specialized syntax.

No leaf grammar may redefine another subsystem's semantic authority.

---

141. "resources.g4" Integration Contract

"grammar/resources/resources.g4" is the resource grammar composition root.

It owns:

- resource dispatch;
- resource declarations;
- resource specification composition;
- resource body composition;
- open-world resource properties;
- integration of specialized resource grammar rules.

It does NOT own the semantic meaning defined in this specification.

"resources.g4" MUST consume the contracts defined here.

---

142. "requirements.g4" Integration Contract

"grammar/resources/requirements.g4" owns resource-requirement syntax.

It MUST preserve:

- requirement expression structure;
- capability requirement structure;
- source spans;
- grouping;
- ordering;
- symbolic expressions.

It MUST NOT perform:

- target discovery;
- allocation;
- routing;
- scheduling;
- hardware selection.

Its semantic owner is this specification plus the repository's common semantic layer.

---

143. Capability Grammar Integration

Capability syntax MUST converge on the repository's canonical capability model.

Both forms:

capability("quantum.measurement")

and:

capability quantum::measurement

where supported by the grammar, MUST normalize to the same semantic capability identity.

Capability identity MUST NOT be duplicated in multiple resource grammars.

---

144. Resource Expression Integration

Resource expressions MUST reuse the canonical expression semantics.

The resource subsystem MUST NOT create a parallel arithmetic or logical expression language.

Expressions such as:

n + 1
memory * element_size
qubits >= logical_qubits

must retain the same expression semantics used elsewhere in Zamani.

---

145. Core Requirement Integration

The repository also contains general requirement infrastructure.

The distinction MUST be maintained:

general requirement

versus:

resource-specific requirement

A generic requirement may refer to resource semantics.

The resource subsystem provides the resource-specific semantic model.

There MUST be no competing requirement semantics.

---

146. Effects Integration

Resource semantics consume effect information when effects imply resource needs.

For example:

network effect
    → network capability/resource analysis

quantum effect
    → quantum resource analysis

foreign effect
    → external-resource analysis

The effect system does not allocate resources.

---

147. Validation Integration

Validation consumes:

- resource requirements;
- constraints;
- budgets;
- capabilities;
- contracts.

Validation MUST be capable of distinguishing:

syntactically valid
semantically invalid
target-infeasible
policy-denied

---

148. Policy Integration

The resource system MUST consume policy decisions without becoming the policy authority.

Policy precedence MUST be defined by the policy subsystem.

Resource analysis MUST receive an unambiguous policy result.

---

149. Execution Integration

Execution consumes resource realization decisions.

It may perform:

- acquisition;
- reservation;
- scheduling;
- release;
- recovery;
- dynamic adaptation.

Execution MUST NOT redefine the source-level meaning of resource requirements.

---

150. Compile Integration

Compilation consumes resource intent during:

- feasibility analysis;
- specialization;
- lowering;
- placement;
- scheduling;
- backend selection.

Resource requirements may influence compilation.

They MUST NOT become target-specific source semantics.

---

151. Classical IR Integration

Resource metadata associated with classical computation MUST lower into the canonical classical semantic/IR representation without creating a competing resource IR.

Resource requirements may be attached as:

- metadata;
- constraints;
- execution requirements;
- scheduling information;
- realization annotations.

---

152. Quantum IR Integration

Quantum resource information MUST lower through the canonical "quantum::ir" path.

The resource subsystem MUST NOT define:

QuantumResourceIR

as a competing canonical IR.

Resource metadata MAY accompany quantum semantic operations.

---

153. HDL Integration

HDL resource requirements MUST remain separate from physical synthesis details.

The path is:

HDL source
    ↓
HDL semantic model
    ↓
resource analysis
    ↓
hardware/HDL IR
    ↓
synthesis
    ↓
target realization

---

154. Hardware Integration

Hardware discovery provides:

available resources
available capabilities
properties
topology
health

Resource analysis consumes this information.

The grammar does not perform discovery.

---

155. Runtime Integration

Runtime resource management MAY perform:

- dynamic acquisition;
- release;
- reallocation;
- recovery;
- migration;
- scaling.

Runtime behavior MUST preserve:

- requirements;
- constraints;
- policies;
- contracts;
- semantic correctness.

---

156. Backend Independence

The resource grammar MUST remain useful even if no backend is currently available.

A valid program may therefore reach:

source valid
semantic analysis valid
resource requirements valid
target realization unavailable

without becoming syntactically invalid.

---

157. Target Infeasibility

If no target can satisfy the requirements, diagnostics SHOULD include:

- unsatisfied requirement;
- missing capability;
- insufficient resource;
- violated constraint;
- applicable policy;
- target context;
- permitted alternatives.

The compiler MUST NOT silently rewrite the source.

---

158. Resource-Aware Specialization

Compilation MAY specialize a program based on available resources.

For example:

portable source
       ↓
resource-aware specialization
       ↓
target-specific implementation

Specialization MUST preserve source semantics.

Specialized artifacts SHOULD preserve provenance linking them to the source program.

---

159. Resource-Aware Optimization

Optimization MAY use:

- available memory;
- parallelism;
- accelerator capability;
- bandwidth;
- topology;
- latency;
- energy;
- power.

Optimization MUST NOT violate hard resource constraints.

---

160. Resource-Aware Routing

Routing may consume resource topology.

For quantum computation:

logical requirements
        ↓
quantum::ir
        ↓
target topology
        ↓
routing

Resource semantics provide the topology requirement.

Routing determines physical mapping.

---

161. Resource-Aware Scheduling

Scheduling may consume:

- capacity;
- latency;
- throughput;
- power;
- energy;
- concurrency;
- resource availability.

Scheduling remains downstream.

---

162. Resource-Aware Resilience

Resilience may consume:

- redundancy;
- reliability;
- availability;
- resource failure state;
- recovery capability.

Resource semantics describe requirements.

Resilience implements recovery.

---

163. Resource-Aware QEC

Quantum error correction may consume:

- logical-qubit requirements;
- physical-qubit capacity;
- error-correction capabilities;
- topology;
- reliability.

QEC remains outside this resource specification.

The resource model merely provides semantic inputs.

---

164. Resource/Capability Negotiation Model

A realization is feasible when, for the relevant scope:

all mandatory requirements are satisfied
AND
all hard constraints are satisfied
AND
all mandatory capabilities exist
AND
applicable policy permits realization
AND
required contracts remain valid

Preferences and hints do not determine basic feasibility.

---

165. Preference Selection Model

Among feasible realizations, selection MAY optimize:

preferences
cost
latency
energy
power
throughput
reliability
locality
other declared objectives

Selection MUST be deterministic when deterministic compilation/execution is required.

---

166. Negotiation Result

A successful negotiation SHOULD produce semantic information describing:

requirements satisfied
constraints satisfied
capabilities matched
policies applied
preferences considered
selected realization class
fallbacks used

Physical implementation details belong to later provenance.

---

167. Explicit Outcome Semantics

Resource feasibility SHOULD integrate with the repository's execution outcomes.

Possible outcomes include:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

A resource subsystem MUST NOT reinterpret these outcomes independently.

---

168. Degraded Acceptance

A degraded realization MAY be accepted only when:

1. the program explicitly permits degradation; or
2. a policy explicitly permits it; and
3. semantic contracts remain satisfied.

A mandatory requirement MUST NOT be violated merely to produce "DEGRADED_ACCEPT".

---

169. Retry

Retry MAY occur when a resource failure is transient.

Examples:

- temporary capacity exhaustion;
- temporary network unavailability;
- transient accelerator failure.

Retry MUST NOT change semantic requirements.

---

170. Recovery

Recovery MAY involve:

- resource replacement;
- target migration;
- rescheduling;
- replication;
- fallback.

Recovery MUST preserve the source-level contract.

---

171. Rejection

A realization MUST be rejected when a mandatory requirement cannot be satisfied and no permitted alternative exists.

The rejection MUST be diagnosable.

---

172. Resource State vs Resource Requirement

The distinction is mandatory:

requirement:
    what the program needs

state:
    what is currently available

A program MUST NOT encode the current environment as universal source semantics unless explicitly intended.

---

173. Resource Observation

Resource observation MAY be exposed to programs where the language permits dynamic resource introspection.

Observed values MUST be typed and explicitly identified as environmental observations.

An observation MUST NOT automatically become a compile-time constant.

---

174. Dynamic Adaptation

When dynamic resource observations are used to select execution paths:

observe
    ↓
evaluate
    ↓
select

the language MUST preserve the semantics of each permitted branch.

---

175. Resource Contracts

A resource contract MAY combine:

requirements
constraints
budgets
capabilities
guarantees
provenance

The contract subsystem owns contract syntax.

This specification defines the meaning of resource predicates within such contracts.

---

176. Resource Properties

Resource properties are open-world.

Examples:

memory.capacity
network.bandwidth
quantum.connectivity
compute.throughput
device.reliability

New properties MUST NOT require modifying the fundamental resource architecture.

Property identity SHOULD be namespaced.

---

177. Property Types

Resource properties MUST have semantic types.

Examples:

capacity → quantity
latency → duration
bandwidth → rate
reliability → defined reliability metric
power → energy/time

The canonical type/quantity system determines validity.

---

178. Property Comparison

Comparisons MUST obey type and unit rules.

Valid:

memory >= required_memory

when both operands represent compatible memory quantities.

Invalid:

memory >= latency

unless a domain explicitly defines such a comparison.

---

179. Resource Algebra

The resource semantic model SHOULD support algebraic operations appropriate to the resource:

sum
difference
minimum
maximum
ratio
scaling
aggregation
partition
composition

The semantics MUST be resource-aware.

For example, memory capacity may be additive across independent resources, while latency may behave according to a different aggregation rule.

---

180. Resource Accounting Semantics

Resource accounting MUST define whether a property is:

- additive;
- consumptive;
- shareable;
- exclusive;
- renewable;
- instantaneous;
- cumulative;
- bounded;
- derived.

This classification SHOULD be available to semantic analysis.

---

181. Temporal Resources

Some resource properties are temporal:

latency
duration
deadline
reservation period
availability window

Temporal resource semantics MUST integrate with the canonical time/duration model.

The resource specification MUST NOT create a second time system.

---

182. Reservation Windows

Reservation may apply to a time interval.

The resource manager determines whether the interval can be satisfied.

Source syntax MUST remain independent of wall-clock scheduling details.

---

183. Deadlines

A deadline is a constraint, not automatically a reservation.

For example:

constrain completion <= deadline;

does not guarantee that resources will be reserved.

If reservation is required, it MUST be separately expressed.

---

184. Resource Lease

A resource lease represents temporary authorization or reservation for a defined scope.

Lease semantics MUST remain distinct from ownership.

A lease may expire without changing the semantic identity of the resource.

---

185. Resource Lifetime

Resource lifetime MUST be associated with semantic scope.

Possible lifetime boundaries include:

expression
operation
block
function
task
actor
module
program
execution
deployment

Physical lifetime remains a runtime concern.

---

186. Resource Cleanup

When resource ownership or lease ends, the runtime MAY perform cleanup.

Cleanup MUST NOT alter program semantics.

Resource release failures MUST be reported according to the runtime/resource policy.

---

187. Resource Leakage

A semantic resource leak occurs when a program retains resource ownership beyond its declared lifetime without an allowed reason.

Detection MAY occur through:

- static analysis;
- runtime analysis;
- ownership analysis;
- policy enforcement.

---

188. Resource Security Boundaries

Resources may belong to security domains.

Resource policies may restrict crossing those boundaries.

The resource model MUST remain compatible with:

- sandboxing;
- authorization;
- trust domains;
- capability security.

---

189. Resource Confidentiality

Some resources may expose confidential properties.

For example:

- private hardware;
- confidential accelerators;
- protected memory;
- secure execution environments.

Resource metadata MUST respect security policy.

The universal resource model MUST NOT require exposing confidential target details to portable source programs.

---

190. Resource Identity Privacy

Physical identifiers SHOULD remain hidden unless explicitly required.

Portable programs should generally receive semantic identities such as:

quantum::measurement

rather than:

vendor_device_7

---

191. Resource Portability

A resource requirement is portable when it describes semantic needs rather than a specific physical implementation.

Portable:

requires memory >= required_memory;
requires capability("tensor.compute");

Less portable:

requires device("vendor-specific-model");

The latter MAY be valid in a target-specific dialect but MUST NOT be treated as universal portable resource semantics.

---

192. POCO-REAF

Resource semantics are a foundation of:

Program
    Once

Compile
    Once

Run
    Everywhere

Run
    Anywhere

Forever

The same source semantics MUST remain meaningful as:

- machine size changes;
- resource counts change;
- hardware types change;
- topology changes;
- accelerator types change;
- quantum architectures change;
- distributed scale changes.

---

193. What POCO-REAF Does Not Promise

POCO-REAF does not mean every program is executable on every machine.

A program may require capabilities unavailable on a target.

The guarantee is instead:

«The source program does not need to be rewritten merely because a different target is selected, provided that target can satisfy its declared semantics.»

---

194. Portable Failure

When a target cannot satisfy the source requirements, the compiler/runtime MUST report:

valid source
+
unsatisfied realization

rather than:

invalid source

unless the source itself violates the language.

---

195. Target Growth

A larger target MAY satisfy a program without source changes.

For example:

requires memory >= required_memory;

can be realized by:

small memory system
large memory system
distributed memory system
heterogeneous memory system

provided the semantic requirement is satisfied.

---

196. Target Reduction

A smaller target may fail to satisfy a requirement.

The compiler MAY use:

- decomposition;
- streaming;
- tiling;
- partitioning;
- scheduling;
- distribution;
- alternative implementation;

only when these preserve the program's semantics and satisfy the declared resource constraints.

---

197. Resource-Aware Decomposition

A compiler MAY transform:

large logical resource requirement

into:

multiple smaller physical resources

if the semantic model permits such decomposition.

The transformation MUST preserve provenance and correctness.

---

198. Resource-Aware Distribution

A computation MAY be distributed across multiple targets if:

- distributed execution is permitted;
- required capabilities exist;
- topology constraints are satisfied;
- effects remain valid;
- contracts remain valid;
- semantic meaning is preserved.

---

199. Resource-Aware Replication

Replication MAY be used for:

- resilience;
- throughput;
- availability;
- distributed execution.

Replication MUST NOT multiply semantic side effects incorrectly.

Effect analysis MUST participate where relevant.

---

200. Resource and Effects

Resource scaling can affect effects.

For example, distributing an operation may alter:

- network effects;
- ordering;
- communication;
- synchronization.

The compiler MUST account for these interactions.

The resource subsystem does not independently redefine effect semantics.

---

201. Resource and Memory

Memory requirements MUST be expressed semantically.

Examples:

requires memory >= working_set;
requires memory >= input.size * element_size;

No universal memory size may be encoded.

---

202. Resource and Storage

Storage requirements MAY distinguish:

- capacity;
- persistence;
- throughput;
- latency;
- durability;
- locality.

These properties MUST remain semantically distinct.

---

203. Resource and Compute

Compute requirements MAY express:

- capability;
- throughput;
- parallelism;
- precision;
- instruction features;
- accelerator capability.

They MUST NOT assume a fixed processor architecture.

---

204. Resource and Precision

A computation MAY require:

capability("high_precision.arithmetic");

or another semantic precision property.

Precision requirements belong to the type/numeric semantic system where possible.

---

205. Resource and Tensor Computation

Tensor programs MAY require:

requires capability("tensor.compute");
requires memory >= tensor_memory;

Tensor dimensions and rank are program semantics.

The resource system MUST NOT define a universal maximum tensor rank.

---

206. Resource and AI Model Execution

Model execution may require:

- memory;
- tensor compute;
- model storage;
- bandwidth;
- precision;
- accelerator capabilities.

These remain semantic requirements.

Specific model frameworks remain library/dialect concerns.

---

207. Resource and Learning

Learning workloads may require:

requires capability("learning.compute");
requires memory >= training_memory;
requires storage >= dataset_size;

The resource model does not enumerate learning algorithms.

---

208. Resource and Adaptation

Adaptation may require:

- additional compute;
- model storage;
- checkpoint storage;
- runtime authorization;
- additional capabilities.

The adaptation subsystem owns adaptation semantics.

Resource analysis evaluates its requirements.

---

209. Resource and Simulation

Simulation may require:

- simulator capability;
- additional memory;
- compute capacity;
- storage;
- parallelism.

Simulation remains an execution strategy.

---

210. Resource and FFI

Foreign interfaces may require:

capability("foreign.call");
capability("abi.compatibility");

The interoperability subsystem owns ABI semantics.

Resource analysis owns resource consequences.

---

211. Resource and Reflection

Reflection may require additional metadata or runtime capabilities.

Examples:

capability("reflection.runtime");

The metaprogramming subsystem owns reflection semantics.

Resource analysis consumes its declared requirements.

---

212. Resource and Networking

Networking may require:

capability("network.connect");
bandwidth >= required_bandwidth;
latency <= latency_budget;

Network effects and security policy remain separate.

---

213. Resource and Actors

Actor systems may require:

- tasks;
- memory;
- queues;
- channels;
- scheduling;
- network resources.

AI agents and other agents use the same actor/resource model.

No second resource system is permitted for agents.

---

214. Resource and Concurrency

Concurrency may increase simultaneous resource demand.

The semantic analyzer MUST distinguish:

sequential reuse

from:

simultaneous consumption

to avoid incorrect resource accounting.

---

215. Resource and Scheduling

Scheduling consumes resource constraints.

The resource specification does not prescribe a scheduling algorithm.

Schedulers MAY use:

- earliest deadline;
- throughput optimization;
- energy optimization;
- locality;
- topology;
- policy.

The selected algorithm must preserve semantics.

---

216. Resource and Routing

Routing consumes:

- topology;
- connectivity;
- bandwidth;
- locality;
- latency.

Routing remains a downstream transformation.

---

217. Resource and Optimization

Optimization MAY trade one resource property against another.

Example:

lower latency

may increase:

energy

The compiler MUST respect all hard constraints while optimizing preferences.

---

218. Resource Objective Functions

Where multiple preferences exist, the compiler MAY use an objective function.

The objective function MUST be derived from declared semantics/policies.

The compiler MUST NOT invent hidden mandatory objectives.

---

219. Resource Negotiation Priority

The semantic priority order is:

correctness
    >
mandatory requirements
    >
hard constraints
    >
mandatory capabilities
    >
policy restrictions
    >
contracts
    >
preferences
    >
hints

This ordering is conceptual.

A specific subsystem MAY define more precise precedence where necessary.

---

220. No Hidden Resource Semantics

A backend MUST NOT assume undocumented requirements.

For example, a backend MUST NOT infer:

GPU required

merely because:

tensor.compute

exists, unless the capability registry explicitly defines that capability as requiring such a realization.

---

221. Capability Emulation

A capability MAY be implemented by software rather than hardware.

For example:

tensor.compute

may be provided by:

- CPU;
- GPU;
- accelerator;
- simulator;
- distributed runtime.

The capability registry defines semantic equivalence.

---

222. Capability Refinement

A capability may refine another capability.

Example:

quantum.measurement
quantum.mid_circuit_measurement

A refinement relationship MUST be explicit.

The compiler MUST NOT assume that a specialized capability automatically satisfies every broader capability unless the registry says so.

---

223. Capability Composition

Multiple capabilities may combine to satisfy a higher-level requirement.

For example:

capability A
+
capability B
+
capability C

may satisfy:

capability D

only if the semantic capability registry explicitly defines the relationship.

---

224. Resource Aliases

Resource aliases MAY exist.

Aliases MUST resolve deterministically.

An alias MUST NOT change semantic identity.

Deprecated aliases MUST be tracked for compatibility.

---

225. Resource Namespaces

Resource namespaces SHOULD prevent collisions.

Examples:

quantum::measurement
network::bandwidth
hardware::accelerator
tensor::compute

Namespaces are semantic identities, not physical addresses.

---

226. Resource Registry

The semantic implementation SHOULD maintain an extensible registry for:

- resource kinds;
- properties;
- capabilities;
- units;
- aliases;
- dialect extensions;
- versions.

The registry MUST NOT require parser modification for every new resource kind.

---

227. Registry Versioning

Registry changes MUST be versioned where they affect semantic interpretation.

A program compiled under a different registry version MUST be reproducible or explicitly diagnosed if semantics changed.

---

228. Resource Contracts and AST

The domain-neutral AST MUST preserve sufficient information to distinguish:

requirement
constraint
capability
budget
preference
hint
negotiation
lifecycle

The AST MUST preserve source spans.

The AST MUST NOT contain physical target allocation unless a target-specific syntax explicitly requires it.

---

229. AST-to-Semantic Mapping

The semantic layer MUST normalize syntactically different but semantically equivalent resource expressions.

For example:

capability("quantum.measurement")

and a canonical capability reference form MAY normalize to:

CapabilityRequirement(
    identity = quantum::measurement
)

The exact Rust type is an implementation choice.

---

230. Canonical Semantic Representation

The semantic representation SHOULD contain concepts equivalent to:

ResourceIdentity
ResourceKind
ResourceScope
ResourceQuantity
ResourceProperty
ResourceRequirement
ResourceConstraint
CapabilityRequirement
ResourceBudget
ResourcePreference
ResourceHint
ResourceTopology
ResourceLifecycle
ResourceProfile
ResourceProvenance

The exact Rust structure belongs to implementation code.

---

231. IR Representation

Resource semantics MUST NOT require a universal standalone Resource IR.

Instead, resource information should be attached to the canonical semantic/IR structures consumed by:

- classical IR;
- "quantum::ir";
- HDL/hardware IR;
- execution plans.

A standalone resource planning representation MAY exist internally, but it MUST NOT become a competing language IR.

---

232. Resource Planning

The compiler MAY create an internal resource plan containing:

requirements
candidate targets
capability matches
allocation intent
scheduling information
placement constraints

This is an implementation artifact.

It does not redefine source semantics.

---

233. Resource Plan Provenance

Internal resource plans SHOULD retain links to source requirements.

This permits diagnostics such as:

source line X
    requires capability("...")
        ↓
resource planning decision
        ↓
target unavailable

---

234. Backend Independence

A resource plan MUST remain separate from backend-specific code generation.

Backend code generation consumes the plan.

---

235. HAL Boundary

HAL consumes target realization decisions.

The resource specification MUST NOT depend on HAL-specific data structures.

HAL may expose:

available resources
capabilities
properties
topology
health

to the compiler/runtime.

---

236. Runtime Boundary

Runtime resource management consumes resource realization plans.

The runtime MUST NOT parse Zamani source to rediscover resource semantics.

---

237. Tooling

Tooling SHOULD be able to display:

- resource requirements;
- constraints;
- capabilities;
- preferences;
- hints;
- budgets;
- unresolved requirements;
- target feasibility;
- provenance.

Tooling MUST distinguish mandatory from advisory intent.

---

238. IDE Diagnostics

An IDE SHOULD be able to report:

Requirement satisfied
Requirement unresolved
Capability unavailable
Constraint violated
Preference unavailable
Hint ignored
Target-specific restriction

without confusing these categories.

---

239. Documentation

Generated resource documentation MUST distinguish:

syntax
semantics
implementation
target realization

A documentation page MUST NOT imply that a semantic resource kind corresponds to one physical technology.

---

240. Testing Model

Resource conformance MUST be tested at multiple levels:

lexical
parser
AST
semantic
type
capability
resource
policy
contract
IR
compiler
runtime
integration
compatibility

Parsing alone is insufficient.

---

241. Positive Tests

At minimum, tests MUST cover:

requires qubits >= n;

requires memory >= required_memory;

requires nodes >= required_nodes;

requires capability("quantum.measurement");

requires capability("tensor.compute");

constrain latency <= latency_budget;

constrain energy <= energy_budget;

prefer low_latency;

hint locality;

Tests MUST also cover symbolic and nested expressions.

---

242. Negative Tests

Tests MUST cover:

- invalid resource expressions;
- incompatible units;
- invalid comparisons;
- contradictory requirements;
- contradictory constraints;
- unknown mandatory capabilities;
- invalid capability parameters;
- invalid budget expressions;
- invalid lifecycle operations;
- invalid resource scopes;
- invalid topology;
- invalid policy combinations.

---

243. Boundary Tests

Boundary tests MUST include:

- zero requirements;
- one requirement;
- many requirements;
- large requirement sets;
- deeply nested groups;
- large symbolic expressions;
- many capabilities;
- many properties;
- large resource groups;
- large topology descriptions.

No boundary test may establish a universal language maximum.

---

244. Scalability Tests

Scalability tests MUST progressively increase:

- resource count;
- requirement count;
- capability count;
- group count;
- expression size;
- topology size;
- symbolic quantity complexity;
- distributed resource count;
- quantum resource count.

The tests MUST distinguish:

implementation exhaustion

from:

language rejection

---

245. Determinism Tests

Equivalent resource specifications MUST normalize deterministically.

For example:

prefer A;
prefer B;

and an explicitly equivalent canonical representation MUST produce equivalent semantic metadata.

Repeated compilation under the same semantic environment MUST produce stable resource decisions where deterministic compilation is required.

---

246. Compatibility Tests

Every stable resource feature MUST be tested across:

lexer
parser
AST
semantic analysis
IR
compiler
runtime

A grammar feature is incomplete if downstream semantics do not understand it.

---

247. Cross-Domain Tests

Resource tests MUST cover combinations such as:

classical + resources
quantum + resources
HDL + resources
hybrid + resources
AI + resources
distributed + resources
networking + resources
simulation + resources
adaptive execution + resources

---

248. Quantum Resource Test

A mandatory integration example SHOULD resemble:

requires qubits >= logical_qubits;
requires capability("quantum.measurement");
requires capability("quantum.dynamic_control");

The test MUST verify that:

source
→ AST
→ resource semantics
→ quantum semantics
→ quantum::ir

works without embedding a physical QPU.

---

249. Classical Resource Test

A mandatory integration example SHOULD resemble:

requires memory >= working_set;
requires capability("tensor.compute");
prefer low_latency;

The test MUST verify that the same source semantics can be considered for different classical/accelerated realizations.

---

250. Distributed Resource Test

A mandatory integration example SHOULD resemble:

requires nodes >= required_nodes;
requires bandwidth >= required_bandwidth;
requires capability("distributed.execution");

The test MUST verify that no fixed node count exists in the grammar.

---

251. HDL Resource Test

A mandatory integration example SHOULD verify:

- timing constraints;
- resource properties;
- power;
- memory;
- hardware capabilities;
- synthesis boundary.

---

252. Heterogeneous Integration Test

At least one integration test SHOULD combine:

classical computation
+
tensor computation
+
quantum computation
+
resource requirements
+
capabilities
+
constraints
+
policies
+
effects
+
provenance

The complete path must remain coherent.

---

253. POCO-REAF Integration Test

A mandatory integration fixture SHOULD express a computation without naming a physical target:

requires memory >= required_memory;
requires capability("tensor.compute");
requires capability("quantum.measurement");
prefer low_latency;

The semantic representation MUST remain unchanged while different target descriptions are supplied.

Only realization decisions may change.

---

254. Hard-Coding Audit

CI MUST search resource specifications, grammar, and implementation for prohibited universal constants.

At minimum:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_QPUS
MAX_ACCELERATORS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

The audit MUST also detect equivalent hard-coded structural restrictions.

---

255. Safe-Rust Audit

CI MUST enforce that production Rust contains no prohibited "unsafe" implementation.

The project SHOULD use:

#![forbid(unsafe_code)]

where appropriate for the relevant crates.

Generated code MUST also be audited according to repository policy.

---

256. Grammar CI

CI SHOULD verify:

- grammar generation;
- parser generation;
- duplicate rule detection;
- duplicate token detection;
- import-cycle detection;
- grammar/parser parity;
- resource-rule ownership;
- deterministic generation.

---

257. Semantic CI

CI SHOULD verify:

- requirement normalization;
- capability normalization;
- unit correctness;
- contradiction detection;
- deterministic normalization;
- resource-property typing;
- policy integration;
- provenance preservation.

---

258. Production Gates

Resource semantics are production-ready only when:

1. specification exists;
2. grammar ownership is explicit;
3. lexical ownership is explicit;
4. AST mapping exists;
5. semantic mapping exists;
6. type integration exists;
7. effect integration exists;
8. capability integration exists;
9. policy integration exists;
10. contract integration exists;
11. provenance integration exists;
12. canonical IR integration exists;
13. compiler integration exists;
14. runtime integration exists where applicable;
15. diagnostics exist;
16. positive tests exist;
17. negative tests exist;
18. boundary tests exist;
19. scalability tests exist;
20. deterministic behavior is tested;
21. compatibility is tested;
22. hard-coding audit passes;
23. safe-Rust audit passes;
24. no artificial resource ceiling exists.

---

259. Independent-File Completion Contract

Every resource grammar/specification file MUST be completable against an already-published integration contract.

The contract MUST identify:

PURPOSE
OWNS
DOES_NOT_OWN
DEPENDS_ON
EXPORTS
CONSUMED_BY
LEXER_DEPENDENCIES
GRAMMAR_DEPENDENCIES
AST_OWNER
AST_MAPPING
SEMANTIC_OWNER
SEMANTIC_MAPPING
TYPE_CONTRACT
EFFECT_CONTRACT
CAPABILITY_CONTRACT
RESOURCE_CONTRACT
POLICY_CONTRACT
CONTRACT_INTEGRATION
PROVENANCE_CONTRACT
IR_DESTINATION
COMPILER_INTEGRATION
RUNTIME_INTEGRATION
HAL_INTEGRATION
TOOLING_INTEGRATION
POSITIVE_TESTS
NEGATIVE_TESTS
BOUNDARY_TESTS
SCALABILITY_TESTS
DETERMINISM_TESTS
COMPATIBILITY_TESTS
SECURITY
HARD_CODING_AUDIT
COMPLETION_CRITERIA

No file should depend on undocumented future semantics.

---

260. Resource File Dependency Contract

The intended dependency direction is:

lexer
  ↓
core identifiers / names
  ↓
canonical expressions
  ↓
resource expressions
  ↓
resource leaf grammars
  ↓
resources.g4
  ↓
ZamaniParser.g4
  ↓
Zamani.g4

Semantic direction:

AST
  ↓
resource semantic model
  ↓
type/effect/capability/policy/contract analysis
  ↓
canonical semantic representation
  ↓
Classical IR / quantum::ir / HDL IR
  ↓
optimization
  ↓
lowering
  ↓
routing
  ↓
scheduling
  ↓
ZQN
  ↓
HAL

No child resource grammar may import the resource composition root.

---

261. No Circular Ownership

The following must never occur:

resources.g4
    ↓
requirements.g4
    ↓
resources.g4

or:

resource specification
    ↓
hardware specification
    ↓
resource specification

The architecture must have one direction of semantic ownership.

---

262. Resource Specification vs Grammar

This file defines:

what resource constructs mean

The grammar defines:

how resource constructs are written

The parser defines:

how source is structurally recognized

The AST defines:

how source structure is represented

Semantic analysis defines:

whether resource intent is meaningful and satisfiable

The compiler defines:

how valid intent is realized

The runtime defines:

how realization executes

These responsibilities MUST remain separate.

---

263. Resource Specification vs Hardware

This file describes:

resource intent

Hardware systems describe:

actual resource inventory

They MUST NOT become the same authority.

---

264. Resource Specification vs Runtime

This file defines semantic rules.

Runtime code determines actual:

- availability;
- allocation;
- scheduling;
- failure;
- recovery;
- release.

Runtime behavior MUST implement the specification rather than redefine it.

---

265. Resource Specification vs Vendor APIs

Vendor APIs are implementation mechanisms.

A vendor API MUST NOT become the universal semantic definition of a Zamani resource.

Vendor-specific support belongs in:

- HAL;
- backend;
- interoperability;
- target dialect;
- driver;
- runtime adapter.

---

266. Resource Specification vs Application Domains

Application-specific concepts SHOULD be represented through:

- libraries;
- dialects;
- capabilities;
- policies;
- resource properties;
- semantic operations.

The universal resource model must remain application-neutral.

---

267. New Resource Kind Rule

A new resource kind SHOULD be introduced without modifying this specification's fundamental model.

It requires:

resource identity
semantic definition
properties
units where applicable
capabilities
constraints
tests
compatibility
provenance

It does not automatically require:

- new universal keyword;
- new root grammar;
- new IR;
- new compiler architecture.

---

268. New Capability Rule

A new capability SHOULD require:

capability identity
semantic meaning
version
provider semantics
requirements
compatibility
tests

It SHOULD NOT require a new universal parser rule if the existing capability-reference grammar can represent it.

---

269. New Resource Property Rule

A new property requires:

identity
type
unit/dimension where applicable
comparison semantics
aggregation semantics
provenance
tests

The property registry SHOULD permit extension without universal grammar modification.

---

270. New Resource Constraint Rule

A new constraint requires:

predicate
operand types
evaluation phase
hard/soft classification
diagnostics
policy interaction
tests

---

271. New Resource Lifecycle Rule

A new lifecycle operation requires:

scope
ownership
authorization
effect
resource state transition
failure behavior
rollback behavior
provenance
tests

---

272. New Resource Dialect Rule

A resource dialect MUST specify:

dialect name
version
resource extensions
capability extensions
property extensions
syntax
AST mapping
semantic mapping
IR mapping
compatibility
security
tests

---

273. New Hardware Rule

A new hardware technology SHOULD first be represented using:

resources
capabilities
properties
constraints
topology
policies
dialects

before introducing new universal syntax.

---

274. New Quantum Technology Rule

A new quantum technology SHOULD first use:

quantum operation metadata
resources
capabilities
topology
effects
policies
quantum::ir

rather than hard-coding every operation into the universal grammar.

---

275. New Accelerator Rule

A new accelerator SHOULD normally enter through:

capability
resource kind
resource properties
target metadata
HAL/backend

rather than a new universal accelerator keyword.

---

276. Resource Semantic Preservation

The following transformations MUST preserve declared resource semantics unless explicitly permitted:

optimization
lowering
specialization
parallelization
distribution
routing
scheduling
simulation
resource substitution
target migration
recovery

---

277. Resource Substitution

A resource may be substituted by another realization when:

1. required capabilities remain satisfied;
2. hard constraints remain satisfied;
3. contracts remain satisfied;
4. policy permits substitution;
5. effects remain semantically valid;
6. provenance is maintained where required.

---

278. Semantic Equivalence

Two resource realizations are semantically equivalent when they satisfy the same declared source-level requirements and preserve the program's observable semantics.

Physical implementations may differ completely.

---

279. Resource Transparency

The programmer should not need to know:

- which CPU executes an operation;
- which GPU executes a tensor operation;
- which physical qubit is used;
- which memory bank is selected;
- which cluster node is selected;

unless the program explicitly requests target-specific control.

---

280. Explicit Target-Specific Control

Target-specific resource control MAY exist through a target-specific dialect or deployment layer.

Such constructs MUST be clearly distinguished from portable resource semantics.

They MUST NOT silently contaminate universal POCO-REAF semantics.

---

281. Resource Constraints and Portability

A constraint may reduce the set of realizable targets.

That does not make the language non-portable.

For example:

requires capability("quantum.measurement");

is portable in semantic form even though only some targets can satisfy it.

Portability means the program retains its meaning across eligible targets.

---

282. Resource Requirements and Future Hardware

Future hardware MUST be able to expose new:

resource kinds
capabilities
properties
topologies
policies

without requiring the universal language to predict those technologies in advance.

---

283. Future Computational Models

The resource architecture MUST be usable by computational models not currently represented in the repository.

A future model should be able to describe:

what resources it needs
what capabilities it provides
what constraints apply
what properties matter

using the same semantic framework.

---

284. Resource Model Invariants

The following are mandatory invariants:

1. Resource intent is separate from realization.
2. Resource requirements are separate from preferences.
3. Preferences are separate from hints.
4. Capabilities are separate from resources.
5. Capacity is separate from requirement.
6. Availability is separate from capacity.
7. Physical identity is separate from semantic identity.
8. Policy is separate from resource semantics.
9. Effects are separate from resource semantics.
10. Contracts are separate from resource semantics.
11. Provenance is separate from resource identity.
12. Grammar does not discover resources.
13. Parser does not allocate resources.
14. Resource grammar does not create an IR.
15. Quantum resource semantics converge through "quantum::ir".
16. No artificial universal capacity exists.
17. New resource kinds remain extensible.
18. New capabilities remain extensible.
19. Resource decisions remain deterministic where required.
20. Resource failures remain explicit.
21. Target-specific information remains isolated.
22. Production Rust remains safe.
23. Resource transformations preserve program semantics.
24. Completed files have explicit integration contracts.

---

285. Production Resource Pipeline

The complete production pipeline is:

                    ZAMANI SOURCE
                          │
                          ▼
                       LEXER
                          │
                          ▼
                       PARSER
                          │
                          ▼
                RESOURCE AST NODES
                          │
                          ▼
                 STRUCTURAL VALIDATION
                          │
          ┌───────────────┼────────────────┐
          ▼               ▼                ▼
        TYPES           EFFECTS       CAPABILITIES
          │               │                │
          └───────────────┼────────────────┘
                          ▼
                   RESOURCE ANALYSIS
                          │
          ┌───────────────┼────────────────┐
          ▼               ▼                ▼
     REQUIREMENTS     CONSTRAINTS      POLICIES
          │               │                │
          └───────────────┼────────────────┘
                          ▼
                     CONTRACTS
                          │
                          ▼
                     PROVENANCE
                          │
                          ▼
               TARGET-INDEPENDENT
                 SEMANTIC MODEL
                          │
             ┌────────────┴────────────┐
             ▼                         ▼
       CLASSICAL IR                quantum::ir
             │                         │
             └────────────┬────────────┘
                          ▼
                     OPTIMIZATION
                          │
                       LOWERING
                          │
                ROUTING / SCHEDULING
                          │
                 RESILIENCE / QEC
                          │
                         ZQN
                          │
                         HAL
                          │
                  TARGET REALIZATION

---

286. Definition of Production Ready

The resource subsystem is production-ready only when:

Specification
      ✓
Grammar
      ✓
Lexer integration
      ✓
Parser integration
      ✓
AST integration
      ✓
Type integration
      ✓
Effect integration
      ✓
Capability integration
      ✓
Constraint validation
      ✓
Policy integration
      ✓
Contract integration
      ✓
Provenance integration
      ✓
Classical IR integration
      ✓
quantum::ir integration
      ✓
HDL/hardware integration
      ✓
Compiler integration
      ✓
Runtime integration
      ✓
Diagnostics
      ✓
Positive tests
      ✓
Negative tests
      ✓
Boundary tests
      ✓
Scalability tests
      ✓
Determinism tests
      ✓
Compatibility tests
      ✓
Hard-coding audit
      ✓
Safe-Rust audit
      ✓

A parser compiling successfully is therefore not sufficient to declare the resource subsystem complete.

---

287. Definition of File Completion

This specification itself is complete when:

- its ownership is unambiguous;
- resource semantics are defined;
- requirements are defined;
- constraints are defined;
- capabilities are defined;
- budgets are defined;
- preferences are defined;
- hints are defined;
- lifecycle intent is defined;
- scalability is defined;
- topology is defined;
- portability is defined;
- negotiation is defined;
- failure behavior is defined;
- provenance is defined;
- deterministic behavior is defined;
- quantum integration is defined;
- classical integration is defined;
- HDL/hardware integration is defined;
- compiler integration is defined;
- runtime integration is defined;
- AST integration is defined;
- IR integration is defined;
- testing requirements are defined;
- hard-coding prohibitions are defined;
- safe-Rust requirements are defined;
- integration ownership is defined.

No later resource grammar file should need to invent the meaning of any of these concepts.

---

288. Final Resource Principle

Zamani resource semantics are based on:

WHAT the computation requires
        +
WHAT capabilities it needs
        +
WHAT constraints must hold
        +
WHAT properties are preferred
        +
WHAT policies apply
        +
WHAT provenance must be retained
        ↓
TARGET-INDEPENDENT RESOURCE INTENT
        ↓
FEASIBILITY
        ↓
REALIZATION

The programmer describes intent.

The compiler determines feasible realization.

The runtime manages execution state.

The HAL manages target interaction.

The hardware provides physical resources.

---

289. Final Scalability Principle

The language MUST remain capable of expressing:

one operation
      ↓
one resource
      ↓
tiny computation
      ↓
large computation
      ↓
parallel computation
      ↓
heterogeneous computation
      ↓
distributed computation
      ↓
massively scaled computation
      ↓
future computational systems

without introducing a universal artificial resource ceiling.

The actual upper bound is determined by:

available resources
+
available capabilities
+
target constraints
+
compiler capacity
+
runtime capacity
+
deployment capacity

and not by arbitrary constants embedded in the Zamani language.

---

290. Final POCO-REAF Principle

The fundamental invariant is:

«A Zamani program describes portable computational intent; resource systems determine whether and how that intent can be realized on a particular target.»

Therefore:

same source
     │
     ├── tiny target
     │
     ├── CPU
     │
     ├── multicore
     │
     ├── GPU
     │
     ├── FPGA
     │
     ├── ASIC
     │
     ├── accelerator
     │
     ├── QPU
     │
     ├── simulator
     │
     ├── HPC
     │
     ├── cluster
     │
     ├── distributed system
     │
     └── future target

may produce different physical realizations while preserving the same source-level semantic intent.

---

291. Final Architectural Invariant

The resource subsystem MUST therefore obey:

RESOURCE INTENT
      +
CAPABILITIES
      +
CONSTRAINTS
      +
POLICIES
      +
CONTRACTS
      +
PROVENANCE
      ↓
CANONICAL SEMANTIC MEANING
      ↓
CANONICAL IR
      ↓
TARGET-INDEPENDENT OPTIMIZATION
      ↓
TARGET REALIZATION

It MUST NOT become:

resource grammar
      ↓
hardware enumeration
      ↓
device selection
      ↓
physical allocation

That would destroy the target independence required by POCO-REAF.

---

292. Final Non-Negotiable Invariants

The following are part of the normative resource contract:

1. One resource semantic model.
2. No competing resource authority.
3. No fixed universal machine capacity.
4. No fixed universal resource count.
5. No hard-coded quantum capacity.
6. No hard-coded classical capacity.
7. No hard-coded HDL capacity.
8. No hard-coded accelerator capacity.
9. No physical-device assumptions in portable resource semantics.
10. Requirements are mandatory.
11. Constraints are mandatory.
12. Capabilities are extensible.
13. Preferences are advisory.
14. Hints are advisory.
15. Budgets are distinct from capacity.
16. Availability is contextual.
17. Resource identity is distinct from physical identity.
18. Resource discovery is downstream.
19. Allocation is downstream.
20. Placement is downstream.
21. Routing is downstream.
22. Scheduling is downstream.
23. QEC is downstream.
24. ZQN is downstream.
25. HAL is downstream.
26. Resource semantics do not create a competing IR.
27. Quantum resource semantics converge through "quantum::ir".
28. Resource expressions reuse canonical expression semantics.
29. Resource quantities reuse canonical type/unit semantics.
30. Resource semantics participate in effect analysis.
31. Resource semantics participate in capability analysis.
32. Resource semantics participate in policy analysis.
33. Resource semantics participate in contracts.
34. Resource decisions can carry provenance.
35. Resource normalization is deterministic.
36. Resource failure is explicit.
37. Fallback is explicit.
38. Simulation is not a silent substitute for physical execution.
39. Target-specific behavior is isolated.
40. New resource kinds are open-world.
41. New capabilities are open-world.
42. New properties are open-world.
43. Future hardware does not require redesigning the resource foundation.
44. Rust 1.97 or later is supported.
45. Production Rust uses no "unsafe".
46. Grammar files contain no executable target-specific actions.
47. Resource grammar does not perform discovery.
48. Resource grammar does not perform allocation.
49. Resource grammar does not perform scheduling.
50. Resource grammar does not perform runtime execution.
51. Every production resource file has a complete integration contract.
52. Every stable resource feature has semantic and downstream tests.
53. No implementation limitation may masquerade as a language limitation.
54. No optimization may violate a mandatory resource condition.
55. No target substitution may silently change program meaning.
56. No backend may redefine the source resource semantics.
57. The source program remains target-independent unless target-specific behavior is explicitly requested.
58. Resource scale is bounded by actual realization resources, not by artificial language constants.

---

293. Final Statement

Zamani does not describe a fixed machine.

Zamani describes a computational intent whose realization may change according to:

resources
capabilities
constraints
policies
topology
availability
performance
reliability
execution environment

The resource architecture therefore establishes the foundation for:

Program Once
Compile Once
Run Everywhere
Run Anywhere
Forever

without requiring the language to predict the physical machines of the future.

The essential invariant is:

                     ZAMANI SOURCE
                           │
                           ▼
                    RESOURCE INTENT
                           │
             ┌─────────────┼─────────────┐
             ▼             ▼             ▼
        REQUIREMENTS   CAPABILITIES   CONSTRAINTS
             │             │             │
             └─────────────┼─────────────┘
                           ▼
                        POLICIES
                           │
                        CONTRACTS
                           │
                      PROVENANCE
                           │
                           ▼
                 SEMANTIC FEASIBILITY
                           │
                           ▼
                 CANONICAL SEMANTICS
                           │
              ┌────────────┴────────────┐
              ▼                         ▼
        Classical IR                quantum::ir
              │                         │
              └────────────┬────────────┘
                           ▼
                      OPTIMIZATION
                           │
                        LOWERING
                           │
                   ROUTING/SCHEDULING
                           │
                    RESILIENCE / QEC
                           │
                          ZQN
                           │
                          HAL
                           │
                           ▼
                  TARGET REALIZATION

The physical realization may change.

The resource intent remains semantic.

The source meaning remains stable.

The language imposes no artificial finite machine-size ceiling.

That is the normative resource foundation required for Zamani to scale from the smallest realizable computation to arbitrarily large realizations permitted by the available resources, while remaining compatible with classical, quantum, HDL, accelerator, distributed, heterogeneous, and future computational systems.