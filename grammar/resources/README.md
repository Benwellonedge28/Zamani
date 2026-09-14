Zamani Resource Grammar

Path: "grammar/resources/README.md"
Language: Zamani
Grammar technology: ANTLR4
Compiler/runtime baseline: Rust 1.97 / Rust 1.97.1, Rust 2021
Safety requirement: Safe Rust only; no "unsafe" Rust
Primary architectural objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Scalability objective: From the smallest supported computation to arbitrarily large computation, limited only by explicitly represented program semantics, compilation policy, available capabilities, and available resources.

---

1. Purpose

The "grammar/resources/" directory defines the source-language resource-intent grammar layer of Zamani.

It provides syntax through which a Zamani program can describe:

- resources;
- resource classes;
- resource quantities;
- resource relationships;
- requirements;
- constraints;
- capabilities;
- preferences;
- hints;
- targets;
- capacity;
- availability;
- performance;
- latency;
- throughput;
- bandwidth;
- energy;
- power;
- reliability;
- resilience;
- scalability;
- portability;
- cost;
- reservation;
- acquisition;
- release;
- derivation;
- grouping;
- resource properties;
- resource contracts;
- resource profiles.

The resource grammar exists to allow a program to express what computation needs, permits, prefers, or can use, without embedding the physical characteristics of a particular machine into the permanent source-language semantics.

The fundamental rule is:

«Zamani describes computational intent and resource semantics; downstream systems determine how that intent is realized on available hardware and execution environments.»

This directory therefore forms a boundary between source-language resource intent and:

- semantic analysis;
- capability negotiation;
- resource management;
- compilation;
- optimization;
- scheduling;
- routing;
- hardware abstraction;
- runtime execution;
- deployment.

---

2. Architectural Position

The resource grammar participates in the following pipeline:

Zamani Source
     |
     v
Canonical Lexer
     |
     v
Canonical Parser
     |
     v
Resource Grammar
     |
     v
Frontend AST / Syntax Representation
     |
     v
Name + Type + Effect + Capability + Resource Analysis
     |
     v
Canonical Semantic Representation
     |
     +--------------------+
     |                    |
     v                    v
Compilation Context   Runtime Context
     |                    |
     +---------+----------+
               |
               v
        Resource / Capability
          Negotiation
               |
               v
      Target Realization
               |
      +--------+--------+
      |        |        |
      v        v        v
     CPU      GPU      QPU
      |        |        |
      +--------+--------+
               |
               v
        Future Targets

The grammar is not the resource manager.

The grammar is not the hardware abstraction layer.

The grammar is not the scheduler.

The grammar is not the optimizer.

The grammar is not the runtime.

The grammar is not the canonical IR.

---

3. Resource Grammar Boundary

The resource grammar answers:

«What resource-related source construct did the programmer write?»

It does not answer:

«Can the current machine satisfy it?»

That second question belongs to downstream semantic/resource/capability analysis.

For example:

requires resource memory >= required_memory;

The grammar recognizes the structure.

It does not determine:

- how much memory exists;
- where that memory is;
- which device supplies it;
- whether it is local or distributed;
- whether it is CPU, GPU, accelerator, or other memory;
- whether the requirement can be satisfied;
- whether another implementation can satisfy the requirement.

Those decisions belong downstream.

---

4. Ownership

4.1 This directory owns

The "grammar/resources/" directory owns the source syntax for:

Resource identity

- resource references;
- resource names;
- resource classes;
- resource kinds;
- symbolic resource paths;
- resource property paths.

Resource intent

- requirements;
- constraints;
- preferences;
- hints;
- capabilities;
- targets.

Resource quantities

- quantity expressions;
- capacity expressions;
- availability expressions;
- derived quantities;
- resource comparisons;
- resource predicates.

Resource characteristics

- performance;
- latency;
- throughput;
- bandwidth;
- energy;
- power;
- reliability;
- resilience;
- scalability;
- portability;
- cost.

Resource lifecycle intent

- reservation;
- acquisition;
- release.

Resource composition

- resource groups;
- resource contracts;
- resource profiles;
- resource properties;
- resource derivations.

Extensibility

- symbolic resource kinds;
- namespaced resource kinds;
- dialect-defined resource properties;
- future resource categories.

---

5. This Directory Does Not Own

The resource grammar must not own or duplicate:

- lexer definitions;
- token spelling;
- identifier syntax;
- numeric literal syntax;
- string literal syntax;
- general expression precedence;
- general arithmetic;
- general logical operators;
- general comparison operators;
- general types;
- module resolution;
- symbol resolution;
- semantic evaluation;
- resource discovery;
- hardware discovery;
- physical allocation;
- physical placement;
- scheduling;
- routing;
- optimization;
- compilation;
- runtime dispatch;
- deployment;
- classical IR;
- canonical "quantum::ir";
- QEC algorithms;
- ZQN noise/fault semantics;
- resilience decision algorithms;
- simulation;
- calibration;
- backend-specific policies.

The resource grammar must never become a second implementation of one of these systems.

---

6. Core Design Principle

The central resource abstraction is:

Intent
  !=
Realization

A source program may state:

requires quantum capability

without stating:

use device X

Likewise:

requires memory >= workload_memory

does not mean:

use memory device Y

Likewise:

prefer low latency

does not mean:

latency must be below a fixed language-defined constant

Likewise:

requires scalable execution

does not mean:

maximum size = N

This separation is mandatory.

---

7. POCO-REAF

Resource syntax is a foundational part of POCO-REAF.

The intended model is:

PROGRAM ONCE
     |
     v
Portable source semantics
     |
     v
COMPILE ONCE
     |
     v
Stable semantic representation
     |
     v
Resource/capability negotiation
     |
     v
Target realization
     |
     +---- CPU
     +---- GPU
     +---- FPGA
     +---- ASIC
     +---- QPU
     +---- simulator
     +---- accelerator
     +---- cluster
     +---- distributed system
     +---- embedded system
     +---- future architecture

Resource declarations therefore describe the requirements and intent of the computation, not a permanent snapshot of one machine.

Important clarification

POCO-REAF does not mean:

«one immutable machine-specific binary must execute directly on every architecture that will ever exist.»

It means:

«the program's semantic meaning must remain stable and portable while target-specific realization can evolve.»

Therefore, resource semantics must be stable even when:

- processors change;
- accelerator architectures change;
- quantum hardware changes;
- memory systems change;
- network architectures change;
- scheduling strategies change;
- compiler implementations change;
- runtime systems change.

---

8. Absolute Scalability Rule

The resource grammar contains no source-language machine-size ceiling.

It must never define constants such as:

MAX_RESOURCES
MAX_DEVICES
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_QUBITS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_ACCELERATORS
MAX_RESOURCE_GROUPS
MAX_PROPERTIES
MAX_RESOURCE_DEPTH

The grammar must also not encode equivalent restrictions indirectly.

Examples of prohibited designs include:

resourceCount
    : ONE
    | TWO
    | THREE
    | FOUR
    ;

or:

qubitCount
    : INTEGER_LITERAL /* interpreted as <= 64 */
    ;

or:

device
    : DEVICE_0
    | DEVICE_1
    | DEVICE_2
    ;

or any equivalent fixed enumeration.

---

9. Quantities Are Expressions

Resource quantities must remain expressions.

For example:

quantity = problem_size;

quantity = workload_size * element_size;

quantity = input.count;

quantity = required_parallelism;

quantity = logical_qubits + ancilla_qubits;

quantity = available_memory - reserved_memory;

The grammar does not impose a finite resource range.

Semantic analysis determines:

- type validity;
- dimensional validity;
- unit validity;
- availability;
- feasibility;
- overflow behavior;
- target compatibility;
- execution policy.

---

10. Runtime Availability Is Not Grammar

The grammar must never inspect runtime resource availability.

For example, this source:

requires resource compute >= required_compute;

does not cause the grammar to:

- inspect CPUs;
- inspect GPUs;
- inspect QPUs;
- inspect memory;
- inspect cluster nodes;
- inspect network links;
- query a cloud provider.

Instead:

Parser
  |
  v
Syntax
  |
  v
AST
  |
  v
Semantic resource requirement
  |
  v
Resource/capability manager
  |
  v
Current environment

Runtime and compilation infrastructure perform resource discovery.

---

11. Resource Categories Are Open-Ended

The language must support both known and future resource categories.

Examples include:

compute
memory
storage
network
accelerator
quantum
quantum.logical_qubit
quantum.physical_qubit
cpu
gpu
fpga
asic
tensor
interconnect
bandwidth
energy

However, the grammar must not require a permanently closed enumeration of all possible resource kinds.

A namespaced resource such as:

future::resource

or:

vendor::accelerator::feature

can be represented syntactically.

Whether that resource is valid for a particular program is a semantic/dialect question.

---

12. Resource Names Are Symbolic

A resource name is not automatically a physical identifier.

For example:

resource compute;

does not mean:

- CPU 0;
- CPU 1;
- GPU 0;
- device "/dev/...";
- a physical address;
- a specific cloud instance.

Similarly:

resource quantum;

does not select:

- a particular QPU;
- a particular qubit;
- a particular topology;
- a particular vendor.

Physical selection belongs to downstream target realization.

---

13. Resource Requirement

A requirement is mandatory.

Conceptually:

requires resource memory >= required_memory;

means:

«a valid realization must satisfy the stated resource requirement.»

A failed requirement must not silently become:

- a preference;
- a hint;
- an optimization opportunity;
- an ignored annotation.

The grammar preserves the distinction.

Semantic/resource analysis determines feasibility.

---

14. Resource Constraint

A constraint defines a condition that the realization must respect.

Examples:

constraint resource latency <= latency_budget;

constraint resource energy <= energy_budget;

constraint resource topology == required_topology;

The grammar records the constraint.

It does not decide whether the constraint is satisfiable.

---

15. Resource Preference

A preference is advisory.

For example:

prefer resource low_latency;

A preference may influence:

- target selection;
- optimization;
- scheduling;
- placement;
- runtime selection.

It must not automatically become a hard requirement.

This distinction is semantically significant.

---

16. Resource Hint

A hint is weaker than a requirement or constraint.

An implementation may ignore a hint while preserving program semantics.

For example:

hint resource locality;

The grammar records that the programmer supplied a hint.

It does not guarantee that the hint will be honored.

---

17. Capability

A capability represents an ability or property of an execution environment.

Examples:

quantum
quantum.measurement
accelerator.tensor
hardware.reconfigurable
network.high_bandwidth
distributed.execution

Capability discovery belongs to the hardware/runtime/capability layer.

The grammar must not:

- discover capabilities;
- validate hardware;
- select a provider;
- query devices.

---

18. Target

A target is an abstraction describing the intended class of execution.

Examples may include:

cpu
gpu
quantum
accelerator
heterogeneous
distributed
embedded

A target category is not necessarily a physical device.

The grammar must distinguish:

target class

from:

concrete target instance

and from:

physical placement

---

19. Capacity

Capacity represents available or contextual resource capacity.

Examples include:

capacity = available_memory;

capacity = execution_context.capacity;

Capacity is contextual.

The grammar does not establish a permanent language-level maximum.

---

20. Availability

Availability may change during execution.

It may depend on:

- runtime conditions;
- distributed resources;
- scheduling;
- failures;
- resource contention;
- dynamic allocation;
- hardware state.

The grammar only represents the source expression.

Availability evaluation belongs downstream.

---

21. Performance

Performance is intentionally abstract.

A performance expression may describe:

- desired performance;
- measured performance;
- estimated performance;
- required performance;
- preferred performance.

The grammar must not define one universal performance unit or machine model.

Interpretation belongs to semantic/resource analysis.

---

22. Latency

Latency may represent:

- requirement;
- constraint;
- preference;
- observation;
- runtime property.

The grammar must not impose a universal fixed latency threshold.

A threshold belongs to program semantics, policy, target context, or resource analysis.

---

23. Throughput

Throughput is similarly contextual.

The grammar must support symbolic expressions such as:

throughput >= required_throughput;

without imposing:

minimum throughput = X

at the language level.

---

24. Bandwidth

Bandwidth can describe:

- network bandwidth;
- memory bandwidth;
- interconnect bandwidth;
- accelerator bandwidth;
- storage bandwidth.

The specific interpretation belongs to the resource type/property model.

---

25. Energy and Power

Energy and power are resource properties.

The grammar must allow them to be expressed without assuming:

- a particular processor;
- a particular voltage;
- a particular clock;
- a particular hardware architecture;
- a particular energy unit implementation.

Unit checking and physical interpretation belong downstream.

---

26. Reliability

Reliability expresses desired or required execution characteristics.

Examples may include:

reliability >= required_reliability;

The grammar does not decide:

- how reliability is measured;
- how it is estimated;
- how faults are detected;
- how failures are recovered.

Those concerns belong to the relevant semantic and runtime subsystems.

---

27. Resilience

Resource-level resilience intent may be expressed through this grammar.

However, this is not the implementation of Zamani's resilience subsystem.

The separation is:

Resource grammar
    |
    | expresses resilience-related intent
    v
Semantic representation
    |
    v
Resilience subsystem
    |
    +--> detection
    +--> diagnosis
    +--> policy
    +--> recovery
    +--> mitigation
    +--> verification

The grammar must not implement recovery algorithms.

---

28. Scalability

Scalability expresses how resource requirements relate to workload or program scale.

Examples:

scalability = problem_size;

scalability = workload_size * parallelism;

scalability = input_dimension;

No finite upper bound is encoded.

The same source semantics may therefore describe:

tiny workload
        |
        v
small machine
        |
        v
large machine
        |
        v
cluster
        |
        v
supercomputer
        |
        v
future system

subject to actual resources and semantics.

---

29. Portability

Portability represents the degree or nature of intended target independence.

The grammar must not equate portability with a fixed list of devices.

Portability semantics are evaluated against:

- target capabilities;
- program requirements;
- dialect compatibility;
- compilation support;
- runtime support.

---

30. Cost

Cost remains abstract.

The grammar must not embed:

- a particular provider;
- a provider pricing model;
- a currency;
- a cloud vendor;
- a hardware vendor.

Cost expressions may be interpreted by deployment/resource-management layers.

---

31. Resource Lifecycle

The grammar may represent intent concerning:

reservation
acquisition
release

It does not implement resource allocation.

For example:

source intent
    |
    v
resource acquisition request
    |
    v
resource manager
    |
    v
actual allocation

The grammar must remain independent of the allocation mechanism.

---

32. Resource Derivation

Resource quantities may be derived from other expressions.

Examples:

quantity = workload_size * element_size;

quantity = logical_qubits + ancilla_qubits;

quantity = input.count * parallelism;

The grammar parses these expressions.

It does not evaluate them.

Evaluation belongs to semantic analysis and compilation/runtime contexts.

---

33. Resource Groups

Resource groups allow related resources to be represented collectively.

Groups must have arbitrary cardinality.

The grammar must not assume:

group of 2
group of 4
group of 8

or any other fixed machine-dependent size.

A resource group may contain:

- one resource;
- many resources;
- dynamically determined resources;
- heterogeneous resources.

---

34. Resource Properties

Resource properties must remain extensible.

Examples:

capacity
latency
bandwidth
energy
reliability
availability

Future properties may be introduced through:

- language evolution;
- dialects;
- namespaces;
- semantic extensions.

The grammar must not require a closed universe of physical properties.

---

35. Resource Expressions

"resource-expressions.g4" owns the composition boundary for resource expressions.

The current design intentionally delegates ordinary expression semantics to the canonical expression grammar rather than implementing a second arithmetic/logical grammar. This is the correct architectural direction.

Therefore:

grammar/resources/resource-expressions.g4
                 |
                 v
       canonical expression grammar
                 |
                 v
       arithmetic / logical /
       comparison / calls /
       indexing / member access

There must not be two competing expression-precedence systems.

---

36. "resources.g4" Ownership

"resources.g4" is the universal resource syntax composition layer.

It owns the composition of:

- resource declarations;
- contracts;
- profiles;
- requirements;
- constraints;
- preferences;
- hints;
- capabilities;
- targets;
- lifecycle intent;
- resource clauses.

It must consume the shared resource-expression boundary.

It must not redefine general expressions.

It must not become a resource semantic IR.

---

37. "requirements.g4"

"requirements.g4" owns the syntax of mandatory resource requirements.

It must preserve the distinction:

requirement

from:

constraint
preference
hint

Requirements must normalize downstream into the canonical resource semantic representation.

---

38. "constraints.g4"

"constraints.g4" owns resource-specific constraint syntax.

Generic boolean composition belongs to the canonical constraint/expression architecture rather than being independently reinvented in this file.

The existing design explicitly separates resource constraint syntax from generic boolean composition and resource discovery. That boundary must remain intact.

---

39. "capabilities.g4"

"capabilities.g4" owns syntax for resource-side capability intent.

It does not own:

- capability discovery;
- hardware inspection;
- runtime capability tokens;
- device enumeration;
- capability implementation.

Those belong to downstream systems.

---

40. "preferences.g4"

"preferences.g4" owns syntax for advisory resource preferences.

Preferences must remain semantically weaker than requirements and constraints.

They must never force target selection at parse time.

---

41. Specialized Resource Grammars

Domain-specific grammars may specialize resource syntax.

Examples include:

grammar/quantum/quantum-resources.g4
grammar/hardware/resources.g4
grammar/hybrid/hybrid-resources.g4

The repository already contains such specialization points.

They must follow this rule:

«Specialize resource semantics; do not redefine universal resource semantics.»

For example, quantum grammar may introduce:

logical qubit resource
physical qubit capability
quantum measurement capability
quantum coherence property

but must ultimately integrate with the universal resource model.

---

42. Quantum Integration

The resource grammar must remain independent of "quantum::ir".

The correct architecture is:

Zamani quantum source
       |
       v
Quantum grammar
       |
       v
AST / semantic analysis
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
ZQN / hardware / runtime

Resource intent may influence quantum compilation, but:

resource grammar

must never become:

quantum::ir

The grammar must not define:

- quantum gate semantics;
- circuit DAGs;
- physical qubit mappings;
- QEC algorithms;
- noise models.

---

43. Hardware Integration

Hardware resource syntax describes intent.

Hardware-specific realization belongs downstream.

The architecture is:

resource intent
      |
      v
hardware capability model
      |
      v
target selection
      |
      v
placement
      |
      v
scheduling
      |
      v
runtime

The grammar must not encode:

GPU 0
GPU 1
CPU 0
QPU 0
FPGA 0

as permanent language concepts.

---

44. Classical Integration

Classical computation may express requirements involving:

- CPU resources;
- memory;
- vector resources;
- accelerator resources;
- parallelism;
- numerical workload;
- tensor workload.

These are source-level resource requirements.

The grammar must not assume a particular processor architecture.

---

45. HDL Integration

HDL constructs may consume resource intent for:

- hardware capacity;
- memory;
- timing;
- power;
- energy;
- accelerator resources;
- implementation targets.

The resource grammar does not own:

- clock semantics;
- signal semantics;
- hardware process semantics;
- RTL semantics.

Those belong to "grammar/hdl/".

---

46. Hybrid Computing

Hybrid programs may combine:

classical resources
+
quantum resources
+
accelerator resources
+
network resources
+
memory resources

The resource grammar must allow these to coexist without creating separate incompatible resource models.

---

47. Distributed Computing

Distributed execution may express:

- node requirements;
- network requirements;
- bandwidth;
- latency;
- storage;
- compute capacity;
- replication-related resource intent.

The grammar must not hard-code a node count.

For example, a requirement should be expressible in terms of a symbolic expression rather than a language-defined maximum.

---

48. AI / Accelerator Integration

AI workloads may consume:

- compute;
- memory;
- tensor resources;
- accelerator capabilities;
- bandwidth;
- energy;
- latency;
- scalability.

AI-specific syntax belongs to "grammar/ai/".

Universal resource requirements belong here.

---

49. No Circular Dependencies

The resource grammar must not create cycles such as:

resources
   -> hardware
      -> resources

or:

resources
   -> runtime
      -> resources

or:

resources
   -> quantum::ir
      -> resources

The dependency direction must remain:

Lexer
  |
  v
Core grammar
  |
  v
Expressions / Types
  |
  v
Resource grammar
  |
  v
AST / semantic analysis
  |
  v
Canonical IR
  |
  v
Compiler / scheduler / hardware / runtime

---

50. Grammar vs Semantic Analysis

The grammar recognizes syntax.

Semantic analysis determines:

- whether a resource exists;
- whether a quantity has the correct type;
- whether units are compatible;
- whether a requirement is satisfiable;
- whether a capability exists;
- whether a target is compatible;
- whether a constraint is contradictory;
- whether a preference is valid;
- whether resource expressions are evaluable;
- whether the program remains portable.

These must never be confused.

---

51. Grammar vs Resource Manager

The resource manager owns:

- discovery;
- allocation;
- reservation;
- release;
- accounting;
- availability;
- resource state.

The grammar only represents source intent.

---

52. Grammar vs Scheduler

Scheduling owns:

- temporal ordering;
- resource conflicts;
- scheduling policy;
- timing;
- alignment;
- execution order.

Resource grammar may provide requirements and constraints to scheduling.

It must not implement scheduling.

---

53. Grammar vs Routing

Routing owns physical realization.

For example:

logical requirement
        |
        v
routing
        |
        v
physical mapping

The resource grammar does not own the mapping.

---

54. Grammar vs Optimization

Optimization may use resource information to select better implementations.

The grammar does not perform optimization.

A preference such as:

prefer low latency

must remain an input to optimization/resource selection rather than becoming an optimization algorithm.

---

55. Grammar vs Runtime

Runtime evaluates resource availability and execution context.

The grammar does not:

- query runtime state;
- perform dispatch;
- allocate devices;
- recover resources;
- inspect hardware.

---

56. Resource Semantics Must Be Typed Downstream

The grammar should preserve syntax while downstream semantic analysis determines distinctions such as:

resource quantity
resource duration
resource capacity
resource rate
resource probability
resource ratio
resource boolean condition
resource identifier

The grammar must not solve all semantic typing through grammar-level duplication.

---

57. Units and Dimensions

The resource grammar may consume canonical quantity/unit expressions.

Dimensional correctness belongs to semantic analysis.

Examples of potentially meaningful dimensions include:

bytes
operations
seconds
joules
watts
bits/second
operations/second
probability

The grammar must not hard-code a finite universe of future resource dimensions unless the language specification explicitly makes a dimension part of the language.

---

58. Infinite-Scale Principle

“Infinity” in the POCO-REAF objective means:

«no artificial finite machine/resource limit is imposed by the grammar.»

It does not mean that physical execution is mathematically unlimited.

Actual execution remains bounded by:

- available memory;
- available hardware;
- execution time;
- energy;
- operating-system limits;
- compiler resources;
- runtime policies;
- target capabilities;
- physical laws.

Those constraints are external to the grammar.

---

59. Resource Expressions Must Remain Symbolic

The preferred architecture is:

source
  |
  v
symbolic resource expression
  |
  v
semantic analysis
  |
  v
context binding
  |
  v
resource realization

Rather than:

source
  |
  v
fixed machine number

This is essential for portability.

---

60. Forbidden Resource Hard-Coding

The following are prohibited in resource grammar semantics:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_DEVICES
MAX_NODES
MAX_GPUS
MAX_FPGAS
MAX_MEMORY
MAX_STORAGE
MAX_ACCELERATORS
MAX_NETWORKS
MAX_RESOURCE_GROUPS

Also prohibited:

- fixed device IDs;
- physical addresses;
- vendor-specific hardware assumptions in universal syntax;
- fixed topology sizes;
- fixed cluster sizes;
- fixed quantum processor sizes;
- fixed accelerator counts.

---

61. Allowed Constraints

A program may explicitly express a semantic requirement.

For example:

requires resource memory >= required_memory;

or:

constraint resource latency <= latency_budget;

These are valid because the values belong to program intent.

The grammar does not impose those values globally.

---

62. Target-Specific Constraints

Target-specific restrictions may exist downstream.

For example:

program requirement
       |
       v
target capability
       |
       v
feasible / infeasible

A target may have limited resources.

That does not justify encoding its limitation into the language grammar.

---

63. Determinism

Parsing must be deterministic.

The grammar must:

- avoid unnecessary ambiguity;
- avoid semantic predicates where possible;
- avoid embedded actions;
- avoid runtime-dependent parsing;
- avoid hardware-dependent parsing;
- avoid resource-discovery-dependent parsing.

The same source and grammar version must produce the same syntax structure.

---

64. No Embedded Rust

ANTLR grammar files in this directory must not contain embedded Rust implementation actions.

The Rust compiler/runtime integration occurs outside the grammar.

This keeps grammar behavior:

- deterministic;
- portable;
- testable;
- generator-independent.

---

65. Rust 1.97 / 1.97.1

The generated parser and surrounding implementation must remain compatible with:

Rust 1.97
Rust 1.97.1
Rust 2021

No grammar design may require a newer Rust language feature unless the repository explicitly changes its compiler baseline.

---

66. Safety

The resource grammar architecture requires:

unsafe Rust = prohibited

There must be no need for:

unsafe

in:

- parser integration;
- AST construction;
- resource semantic lowering;
- resource validation;
- resource tests;
- resource tooling.

Safe Rust abstractions must be used throughout the implementation.

---

67. Integration Contract: Lexer

The canonical lexer owns:

- keywords;
- identifiers;
- literals;
- operators;
- punctuation;
- comments.

The resource grammar consumes those tokens.

Resource grammar files must not silently introduce duplicate lexer definitions.

Before adding a resource keyword, verify:

1. whether it already exists;
2. whether it is globally reserved;
3. whether it conflicts with another domain;
4. whether a symbolic identifier would be better;
5. whether a namespace can avoid reserving a global keyword.

---

68. Integration Contract: Expressions

All resource expressions must ultimately use the canonical expression architecture.

Resource grammar must not implement a separate expression-precedence hierarchy.

The existing "resource-expressions.g4" explicitly establishes this non-duplication boundary.

---

69. Integration Contract: Types

Resource quantities and properties must integrate with the canonical type system.

The resource grammar must not define an incompatible second type system.

Resource types should be interpreted through:

grammar/types/

and downstream semantic type analysis.

---

70. Integration Contract: AST

The AST must represent source intent rather than physical allocation.

For example, a requirement should conceptually preserve:

Requirement {
    resource: ...
    condition: ...
}

rather than:

Requirement {
    gpu_id: 0
}

unless a physical identifier is explicitly part of a separate target/deployment language construct.

---

71. Integration Contract: Canonical IR

The resource grammar does not create IR.

The pipeline is:

Grammar
  |
  v
AST
  |
  v
Semantic resource model
  |
  v
Canonical IR / compilation representation

Resource semantics may become metadata, constraints, requirements, or scheduling/compiler inputs in the canonical semantic representation.

---

72. Quantum IR Contract

For quantum programs:

grammar/quantum/
        |
        v
AST
        |
        v
quantum::ir

Resource requirements may influence compilation but must not replace the canonical quantum IR.

---

73. QEC Contract

Resource syntax may describe resource requirements related to error correction.

It must not implement:

- QEC algorithms;
- stabilizer decoding;
- syndrome processing;
- code construction;
- decoder selection.

Those remain in the QEC subsystem.

---

74. ZQN Contract

Resource syntax may describe requirements related to execution environments affected by noise/fault characteristics.

It must not implement:

- noise models;
- fault channels;
- fault classification;
- calibration;
- noise simulation.

Those remain in ZQN.

---

75. Resilience Contract

Resource syntax may provide resilience requirements.

It must not implement:

- incident detection;
- diagnosis;
- recovery policies;
- retry strategies;
- rollback;
- backend switching;
- quarantine;
- mitigation algorithms.

Those belong to the resilience subsystem.

---

76. Scheduling Contract

Resource syntax supplies scheduling-relevant information such as:

- resource requirements;
- latency constraints;
- throughput requirements;
- energy preferences;
- availability conditions.

Scheduling determines actual execution order and timing.

---

77. Hardware HAL Contract

Hardware abstraction owns:

- physical device representation;
- hardware capabilities;
- topology;
- calibration;
- physical resources.

The resource grammar describes abstract requirements against those capabilities.

---

78. Compilation Contract

Compilation consumes resource intent to:

- validate target feasibility;
- select appropriate lowerings;
- construct target-independent representations;
- select target-specific implementations;
- communicate constraints to optimization and scheduling.

The grammar does not perform compilation.

---

79. Runtime Contract

Runtime may use resource semantics for:

- capability negotiation;
- dispatch;
- resource acquisition;
- availability;
- dynamic execution;
- deployment.

Runtime must not modify source semantics merely because a particular machine has different capacity.

---

80. Dialect Contract

Vendor, experimental, or future resource properties should be namespaced through the dialect system.

Universal resource syntax must remain stable.

For example, conceptually:

standard::resource
vendor::resource
experimental::resource
future::resource

The universal grammar should not become polluted with every vendor's current resource vocabulary.

---

81. Versioning

Resource syntax is versioned with the Zamani language.

Changes must be classified as:

- additive;
- compatible;
- deprecated;
- breaking.

A resource construct must not silently change meaning between language versions.

---

82. Compatibility

Backward compatibility must preserve the distinction between:

requirement
constraint
preference
hint
capability
target

A future grammar revision must not silently reinterpret one category as another.

---

83. Deprecation

Deprecated resource syntax must be documented through the language compatibility system.

Deprecation should include:

- original syntax;
- replacement syntax;
- first deprecated version;
- removal policy;
- migration guidance.

---

84. Diagnostics

Resource grammar diagnostics must clearly distinguish syntax errors from semantic errors.

Examples:

syntax error:
    malformed resource declaration

versus:

semantic error:
    resource expression has incompatible type

versus:

resource feasibility error:
    no current target satisfies the mandatory requirement

The parser must not pretend to know runtime feasibility.

---

85. Error Recovery

ANTLR error recovery must preserve useful diagnostics.

The resource grammar should avoid constructs that cause catastrophic parser recovery where a localized diagnostic is possible.

Tests must cover malformed:

- resource declarations;
- requirements;
- constraints;
- preferences;
- hints;
- capability expressions;
- target expressions;
- resource groups;
- property paths;
- resource contracts.

---

86. Testing Strategy

Every resource grammar rule must have tests.

Tests belong primarily under:

grammar/tests/resources/

with cross-domain tests under:

grammar/tests/cross-domain/

---

87. Positive Tests

Positive tests must cover:

- resource declarations;
- symbolic resource names;
- qualified resource names;
- resource expressions;
- quantities;
- requirements;
- constraints;
- preferences;
- hints;
- capabilities;
- targets;
- capacity;
- availability;
- scalability;
- portability;
- performance;
- latency;
- throughput;
- bandwidth;
- energy;
- power;
- reliability;
- resilience;
- cost;
- lifecycle expressions;
- groups;
- properties;
- derivations;
- nested resource specifications.

---

88. Negative Tests

Negative tests must cover:

- malformed resource declarations;
- missing expressions;
- missing semicolons;
- invalid resource paths;
- malformed qualified names;
- malformed attribute syntax;
- invalid grouping syntax;
- invalid clause composition;
- duplicate syntactic delimiters;
- invalid expression integration.

Semantic invalidity should be tested separately from parser invalidity.

---

89. Scalability Tests

Scalability tests must prove the grammar does not impose artificial finite resource limits.

Test source programs should exercise symbolic quantities such as:

n
problem_size
workload_size
qubit_count
parallelism
node_count
memory_required
device_count

without grammar-level maximums.

The test suite should deliberately search the grammar for forbidden patterns such as:

MAX_

and fixed resource enumerations.

---

90. Boundary Tests

Boundary tests must include:

Smallest meaningful resource expressions

resource compute;

Symbolic scale

quantity = workload_size;

Large symbolic scale

quantity = input_size * parallelism * replication;

The grammar must treat the latter as ordinary syntax without introducing a source-level scale limit.

---

91. Cross-Domain Tests

The resource grammar must be tested with:

classical + resource
quantum + resource
hybrid + resource
HDL + resource
hardware + resource
distributed + resource
AI + resource
networking + resource
security + resource

and combinations such as:

classical + quantum + distributed
classical + quantum + HDL + hardware
AI + quantum + accelerator
HDL + accelerator + distributed

---

92. Determinism Tests

The same source must produce identical parser results across repeated executions.

The test suite must not depend on:

- machine topology;
- runtime availability;
- random device selection;
- current time;
- external network state.

---

93. Round-Trip Tests

Where a Zamani formatter/printer exists:

source
  |
  v
lexer
  |
  v
parser
  |
  v
AST
  |
  v
printer
  |
  v
source
  |
  v
parser

must preserve resource semantics.

Formatting changes must not change:

- requirements;
- constraints;
- preferences;
- hints;
- capabilities;
- targets;
- quantities.

---

94. Hard-Coding Audit

Every change to this directory must undergo a hard-coding audit.

Search for:

MAX_

fixed resource counts;

fixed device IDs;

fixed topology;

fixed machine sizes;

fixed qubit counts;

fixed CPU counts;

fixed GPU counts;

fixed node counts;

fixed memory sizes;

fixed accelerator counts;

fixed physical addresses.

Each result must be classified as:

1. genuine language semantic requirement;
2. target-specific requirement;
3. resource constraint;
4. implementation limitation;
5. accidental hard-coding;
6. test-only limitation;
7. documentation-only example.

Accidental hard-coding must be removed.

---

95. Examples Must Not Become Limits

Examples may use values such as:

4
8
16
32
64

for demonstration.

Those numbers must never be interpreted as language maximums.

Documentation must clearly distinguish:

example value

from:

language constraint

---

96. Resource Vocabulary Strategy

Prefer symbolic and namespaced resource vocabulary over permanent enumeration.

Universal vocabulary should contain only concepts that are genuinely language-level concepts.

Future resource classes should be extensible without requiring a grammar rewrite merely because a new hardware technology appears.

---

97. Open-World Principle

The resource system is open-world.

New resources may arise from:

- future processors;
- new accelerator architectures;
- new quantum technologies;
- new memory technologies;
- new interconnects;
- new distributed systems;
- new scientific computing models;
- future computing paradigms.

The grammar should be capable of representing them without introducing a new fixed machine-size rule.

---

98. Security

Resource syntax must not become a mechanism for bypassing security policy.

For example:

requires resource device

does not grant permission to access that resource.

Security authorization belongs to:

grammar/security/

and the corresponding semantic/runtime security systems.

---

99. Resource Intent vs Permission

These concepts must remain separate:

requires resource X

means:

«the program needs X.»

It does not mean:

«the program is authorized to access X.»

Authorization is a separate semantic concern.

---

100. Resource Intent vs Ownership

A resource requirement does not automatically imply ownership.

For example:

requires memory

does not mean:

program owns physical memory

Ownership and borrowing semantics belong to the memory/type systems.

---

101. Resource Intent vs Placement

A requirement such as:

requires accelerator

does not imply a physical placement.

Placement belongs to hardware/resource/scheduling infrastructure.

---

102. Resource Intent vs Scheduling

A latency requirement does not directly specify a schedule.

Scheduling may use the requirement as an input.

The grammar must not encode a particular scheduling algorithm.

---

103. Resource Intent vs Optimization

A preference is an optimization input, not an optimization algorithm.

The grammar must not encode:

choose fastest implementation

as a compiler algorithm.

---

104. Resource Intent vs Runtime Failure

If runtime resources become unavailable, runtime/resilience systems determine what happens.

The grammar does not prescribe:

- retry;
- rollback;
- migration;
- backend switching;
- degradation;
- abort.

Those belong to the relevant runtime/resilience policies.

---

105. Repository Integration

The resource grammar integrates with:

grammar/core/
grammar/types/
grammar/expressions/
grammar/classical/
grammar/quantum/
grammar/hybrid/
grammar/hdl/
grammar/hardware/
grammar/distributed/
grammar/ai/
grammar/networking/
grammar/security/
grammar/compile/
grammar/execution/
grammar/dialects/
grammar/validation/

It must not create dependency cycles with those areas.

---

106. Repository Semantic Integration

Downstream integration should conceptually follow:

grammar/resources/
        |
        v
AST resource nodes
        |
        v
semantic/resource model
        |
        +--> capability analysis
        +--> resource analysis
        +--> target selection
        +--> compilation
        +--> optimization
        +--> scheduling
        +--> hardware realization
        +--> runtime

---

107. Canonical Ownership Matrix

Concern| Owner
Resource source syntax| "grammar/resources/"
General expressions| "grammar/expressions/"
Types| "grammar/types/"
Hardware syntax| "grammar/hardware/"
Quantum syntax| "grammar/quantum/"
Canonical quantum semantics| "src/quantum/ir/"
QEC| Quantum/QEC subsystem
Noise/fault semantics| ZQN
Scheduling| Scheduling subsystem
Routing| Routing subsystem
Optimization| Optimization subsystem
Hardware discovery| Hardware HAL
Resource allocation| Resource manager
Runtime dispatch| Runtime
Recovery policy| Resilience
Security authorization| Security subsystem

This matrix prevents ownership drift.

---

108. File-Level Integration Contract

Every file in "grammar/resources/" must document:

Purpose
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Grammar Contract
AST Contract
Semantic Contract
IR Integration
Compiler Integration
Runtime Integration
Tooling Integration
Cross-Domain Integration
Tests
Negative Tests
Boundary Tests
Compatibility Requirements
Scalability Requirements
Hard-Coding Audit
Completion Criteria

No resource grammar file is complete until these contracts are satisfied.

---

109. "resources.g4" Completion Contract

"resources.g4" is complete when:

- all universal resource constructs have a single composition point;
- it consumes canonical expressions;
- it does not duplicate domain-specific resource semantics;
- it contains no machine-size limits;
- it has deterministic parsing;
- all referenced rules/tokens have authoritative owners;
- all downstream AST mappings are defined;
- all resource categories have integration contracts;
- positive/negative/boundary tests exist;
- cross-domain integration is tested.

The current "resources.g4" already establishes the intended ownership of universal resource intent and explicitly rejects physical-machine ownership.

---

110. "resource-expressions.g4" Completion Contract

Complete when:

- canonical expressions are imported/consumed;
- no second precedence system exists;
- resource expression composition is defined;
- resource predicates are represented;
- resource quantities remain symbolic;
- no hardware discovery exists;
- no resource evaluation occurs in grammar;
- namespace depth remains unbounded;
- tests prove canonical expression integration.

The existing design already follows this architecture by making "resourceExpression" consume the canonical "expression" rule.

---

111. Domain Resource Grammar Completion Contract

Each domain-specific resource grammar is complete when:

- it uses the universal resource model;
- it introduces only domain-specific syntax;
- it does not redefine universal resource semantics;
- it does not create machine-size limits;
- it integrates into the domain AST;
- it has a semantic normalization path;
- it has cross-domain tests.

---

112. Generated Parser Contract

Generated parser artifacts are implementation outputs.

They are not independent language authorities.

The authoritative source remains the canonical grammar architecture.

Generated artifacts must be reproducible.

Generated files must not be manually edited unless the repository explicitly defines them as source artifacts.

---

113. Documentation Contract

Documentation must remain synchronized with grammar behavior.

If documentation says:

resource quantities are unbounded by grammar

the grammar must not introduce a hidden fixed maximum.

If syntax changes, corresponding documentation and compatibility information must be updated in the same architectural change.

---

114. Review Requirements

Any change to resource grammar should be reviewed for:

1. syntax correctness;
2. ambiguity;
3. ownership;
4. semantic boundary preservation;
5. AST compatibility;
6. IR integration;
7. scalability;
8. hardware independence;
9. cross-domain compatibility;
10. version compatibility;
11. deterministic behavior;
12. hard-coded limits;
13. security implications.

---

115. Production-Readiness Checklist

The "grammar/resources/" subsystem is production-ready only when all of the following are true.

Architecture

- [ ] Resource grammar has clear ownership.
- [ ] No circular dependency exists.
- [ ] Universal resource semantics have one owner.
- [ ] Domain-specific grammars specialize rather than duplicate.

Syntax

- [ ] Resource declarations parse.
- [ ] Requirements parse.
- [ ] Constraints parse.
- [ ] Preferences parse.
- [ ] Hints parse.
- [ ] Capabilities parse.
- [ ] Targets parse.
- [ ] Resource properties parse.
- [ ] Resource expressions use canonical expressions.
- [ ] Resource lifecycle syntax is defined.

Semantics

- [ ] Requirement/constraint/preference/hint distinctions are preserved.
- [ ] Resource intent is separated from realization.
- [ ] Capability intent is separated from capability discovery.
- [ ] Target intent is separated from target selection.
- [ ] Resource quantities are symbolic.
- [ ] Resource feasibility is evaluated downstream.

Scalability

- [ ] No fixed qubit limit exists.
- [ ] No fixed CPU limit exists.
- [ ] No fixed GPU limit exists.
- [ ] No fixed FPGA limit exists.
- [ ] No fixed node limit exists.
- [ ] No fixed memory limit exists.
- [ ] No fixed device limit exists.
- [ ] No fixed topology exists.
- [ ] No fixed resource-group cardinality exists.
- [ ] No hidden equivalent limits exist.

Integration

- [ ] Lexer integration is defined.
- [ ] Expression integration is defined.
- [ ] Type integration is defined.
- [ ] AST integration is defined.
- [ ] Semantic integration is defined.
- [ ] Classical integration is defined.
- [ ] Quantum integration is defined.
- [ ] "quantum::ir" boundary is preserved.
- [ ] QEC boundary is preserved.
- [ ] ZQN boundary is preserved.
- [ ] Scheduling boundary is preserved.
- [ ] Routing boundary is preserved.
- [ ] Optimization boundary is preserved.
- [ ] Hardware HAL boundary is preserved.
- [ ] Resource-manager boundary is preserved.
- [ ] Runtime boundary is preserved.
- [ ] Resilience boundary is preserved.

Safety

- [ ] No embedded unsafe Rust is required.
- [ ] Rust 1.97/1.97.1 compatibility is maintained.
- [ ] Grammar contains no runtime-dependent behavior.
- [ ] Grammar contains no hardware discovery.
- [ ] Grammar contains no external side effects.

Testing

- [ ] Positive tests exist.
- [ ] Negative tests exist.
- [ ] Boundary tests exist.
- [ ] Scalability tests exist.
- [ ] Determinism tests exist.
- [ ] Round-trip tests exist where supported.
- [ ] Cross-domain tests exist.
- [ ] Hard-coding audit exists.

Documentation

- [ ] Ownership is documented.
- [ ] Non-ownership is documented.
- [ ] Integration contracts are documented.
- [ ] Compatibility policy is documented.
- [ ] Extension policy is documented.
- [ ] Examples distinguish examples from limits.

---

116. Definition of Done

The "grammar/resources/" directory is considered complete only when a developer can answer all of the following without reopening the architecture:

What syntax does this directory own?
What syntax does it not own?
What tokens does it consume?
What expression system does it use?
What AST nodes does it produce?
What semantic model consumes those nodes?
How do classical programs use it?
How do quantum programs use it?
How does it reach quantum::ir without becoming quantum::ir?
How do HDL programs use it?
How do hardware targets use it?
How do distributed programs use it?
How do AI/accelerator programs use it?
How do scheduling and optimization consume it?
How does runtime consume it?
How are resource constraints evaluated?
Where is hardware discovered?
Where is physical placement decided?
Where are resource limits enforced?
Where are security permissions enforced?
How are future resource kinds added?
How is compatibility maintained?
How is scalability tested?
How is hard-coding prevented?

No subsequent domain grammar should require changing the fundamental resource model merely because a new machine architecture is introduced.

---

117. Final Principle

The resource grammar exists to express:

WHAT IS NEEDED
WHAT IS ALLOWED
WHAT IS PREFERRED
WHAT IS POSSIBLE
WHAT IS AVAILABLE
WHAT IS TARGETED
WHAT IS CONSTRAINED
WHAT IS SCALABLE
WHAT IS PORTABLE

It does not prescribe:

WHICH MACHINE
WHICH DEVICE
WHICH CORE
WHICH QUBIT
WHICH GPU
WHICH FPGA
WHICH NODE
WHICH ADDRESS
WHICH TOPOLOGY
WHICH SCHEDULER
WHICH ROUTING ALGORITHM
WHICH OPTIMIZATION

Those decisions belong to later compilation and execution layers.

Therefore:

                    ZAMANI RESOURCE MODEL

       Program intent
             |
             v
     Abstract resources
             |
       +-----+-----+
       |     |     |
       v     v     v
   require prefer hint
       |     |     |
       +-----+-----+
             |
             v
      capabilities
             |
             v
       target context
             |
             v
    resource negotiation
             |
             v
      target realization
             |
      +------+------+------+------+------+
      |      |      |      |      |      |
     CPU    GPU    FPGA   ASIC    QPU  Future
      |      |      |      |      |      |
      +------+------+------+------+------+
             |
             v
        execution

The permanent language meaning remains above the realization boundary.

That is the resource-layer foundation required for:

«Zamani — From Atom to Everywhere»

and for:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).»

The decisive rule is:

«Resource availability may constrain an execution, but resource availability must never become an accidental constraint on the Zamani language itself.»