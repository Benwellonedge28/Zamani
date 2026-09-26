Zamani Resource Grammar

Path: "grammar/resources/README.md"
Language: Zamani
Grammar technology: ANTLR4
Compiler baseline: Rust 1.97 / Rust 1.97.1
Rust edition: Rust 2021
Safety: Safe Rust only; "unsafe" is prohibited
Primary objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)
Status: Normative directory-level architecture and integration contract

---

1. Purpose

The "grammar/resources/" directory defines the source-language resource-intent grammar layer of Zamani.

It provides the syntax and composition contracts required for programs to express computational resource intent independently of a particular machine, device, vendor, topology, deployment environment, or runtime allocation.

The resource subsystem covers:

- resource declarations;
- resource references;
- resource kinds;
- resource quantities;
- resource requirements;
- resource constraints;
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
- scalability;
- portability;
- cost;
- reservation;
- acquisition;
- release;
- derivation;
- resource groups;
- resource contracts;
- resource profiles;
- resource properties;
- resource metadata;
- resource negotiation intent.

The central rule is:

«Zamani source expresses computational intent and resource semantics. Downstream compilation and execution systems determine how that intent is realized using the resources and capabilities actually available.»

This directory therefore forms a boundary between source-language resource intent and:

- semantic analysis;
- type analysis;
- effect analysis;
- capability analysis;
- resource analysis;
- compilation;
- optimization;
- routing;
- scheduling;
- resilience;
- QEC;
- ZQN;
- hardware abstraction;
- runtime execution;
- deployment;
- physical realization.

---

2. Architectural Position

The resource grammar participates in the canonical Zamani pipeline:

Zamani Source
     |
     v
Canonical Lexer
     |
     v
Canonical Parser
     |
     v
grammar/Zamani.g4
     |
     v
grammar/resources/resources.g4
     |
     +-------------------------------+
     |                               |
     v                               v
resource components             shared language grammar
     |                               |
     +---------------+---------------+
                     |
                     v
              Frontend AST
                     |
                     v
          Structural Analysis
                     |
                     v
              Name Resolution
                     |
                     v
               Type Analysis
                     |
                     v
              Effect Analysis
                     |
                     v
          Resource / Capability
                 Analysis
                     |
                     v
          Canonical Semantic Model
                     |
          +----------+----------+
          |          |          |
          v          v          v
      Classical  quantum::ir   HDL/
                              Hardware
          |          |          |
          +----------+----------+
                     |
                     v
              Optimization
                     |
          +----------+----------+
          |          |          |
          v          v          v
       Routing   Scheduling  Resilience
          |          |          |
          +----------+----------+
                     |
                     v
                    ZQN
                     |
                     v
                    HAL
                     |
                     v
             Target Realization
                     |
          +----------+----------+----------+
          |          |          |          |
          v          v          v          v
         CPU        GPU       FPGA       QPU
          |          |          |          |
          +----------+----------+----------+
                     |
                     v
             Future Targets

The resource grammar is not the resource manager.

It is not the hardware abstraction layer.

It is not the scheduler.

It is not the router.

It is not the optimizer.

It is not QEC.

It is not ZQN.

It is not the runtime.

It is not the canonical IR.

It is the source-language representation of resource-related intent.

---

3. Authority Hierarchy

The resource directory participates in the following authority hierarchy.

3.1 Normative language architecture

grammar/DESIGN.md

Owns the global grammar architecture, language boundaries, AST/semantic/IR separation, portability principles, and POCO-REAF architecture.

3.2 Normative language specification

grammar/specification/

Owns normative language semantics and specification-level definitions.

3.3 Resource-specific contracts

grammar/spec/resources.md

Owns the focused resource and capability semantic contract.

3.4 Canonical resource grammar composition

grammar/resources/resources.g4

Owns the complete resource grammar composition boundary.

3.5 Resource grammar components

Files under:

grammar/resources/

own specialized resource syntax according to their documented ownership.

3.6 Canonical global composition

grammar/Zamani.g4

Composes the resource subsystem into the complete Zamani language.

3.7 Implementation conformance

grammar/grammar.md

Documents what the current compiler/frontend actually implements.

It is not a second grammar authority.

3.8 Historical/extended design

grammar/Zamani-Grammar.md

Retains historical, experimental, aspirational, and extended language design.

It cannot silently introduce legal syntax.

---

4. Ownership of This Directory

4.1 This directory owns

The resource grammar subsystem owns source syntax for:

Resource identity

- symbolic resource names;
- resource kinds;
- qualified resource kinds;
- resource references;
- resource property paths;
- logical resource groups.

Resource intent

- requirements;
- constraints;
- capabilities;
- preferences;
- hints;
- target intent.

Resource quantities

- quantities;
- capacities;
- availability;
- resource-derived expressions;
- workload-dependent quantities;
- symbolic quantities.

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

- groups;
- contracts;
- profiles;
- properties;
- derivations.

Extensibility

- open-world resource kinds;
- qualified resource namespaces;
- dialect-defined resource properties;
- future resource categories.

---

5. This Directory Does Not Own

The resource directory does not own:

- lexer definitions;
- token spelling;
- identifier lexical rules;
- Unicode identifier classification;
- numeric literal implementation;
- string literal implementation;
- general expression precedence;
- general arithmetic;
- general logical operators;
- general comparison operators;
- general type semantics;
- module resolution;
- symbol resolution;
- resource discovery;
- hardware discovery;
- physical allocation;
- physical placement;
- scheduling algorithms;
- routing algorithms;
- optimization algorithms;
- classical IR;
- "quantum::ir";
- QEC algorithms;
- ZQN implementation;
- HAL implementation;
- calibration;
- backend instruction selection;
- device enumeration;
- runtime resource management;
- cloud-provider integration;
- vendor-specific hardware implementation.

If another subsystem already owns a concept, resource grammars must reference or compose that concept rather than duplicate it.

---

6. Intent Is Not Realization

The fundamental resource distinction is:

Resource Intent != Resource Realization

For example:

requires memory >= required_memory

means:

«A valid realization must provide sufficient memory according to the semantic requirement.»

It does not mean:

use_memory_bank_0

Likewise:

requires capability("quantum.measurement")

does not mean:

use_qpu_0

And:

prefer capability("accelerator.tensor")

does not mean:

use_gpu_0

The source program describes requirements and permitted semantics.

The compiler/runtime chooses realization.

---

7. Mandatory Separation of Resource Concepts

The resource system must preserve these distinctions:

resource declaration
        !=
requirement
        !=
constraint
        !=
capability
        !=
preference
        !=
hint
        !=
target intent
        !=
physical allocation

In particular:

requirement != capability
capability != preference
preference != hint
requirement != implementation decision
logical resource != physical resource
resource intent != allocation

These distinctions are semantic contracts, not merely documentation conventions.

---

8. Requirements

A requirement is mandatory.

Conceptually:

requires qubits >= logical_qubits;

means that a valid realization must satisfy the requirement.

A failed requirement must not silently become:

- a preference;
- a hint;
- an optimization suggestion;
- an ignored annotation.

Resource analysis must report unsatisfied mandatory requirements explicitly.

A resource requirement failure is not a syntax error.

---

9. Constraints

A constraint describes a condition that a valid realization must respect.

Examples include:

constraint latency <= latency_budget;
constraint energy <= energy_budget;
constraint topology == required_topology;

The grammar represents the constraint.

It does not determine whether the current target satisfies it.

Feasibility belongs to semantic/resource/target analysis.

---

10. Capabilities

A capability describes an ability or property available from an execution environment.

Examples include:

quantum.measurement
quantum.mid_circuit_measurement
tensor.compute
accelerator.compute
hardware.reconfigurable
distributed.execution
network.high_bandwidth

Capabilities must remain open-ended.

The grammar must not require a permanent list of every future hardware capability.

Capability discovery belongs downstream.

The parser must not inspect hardware to determine whether a capability exists.

---

11. Preferences

A preference is advisory.

For example:

prefer low_latency;

A preference may influence:

- target selection;
- optimization;
- scheduling;
- placement;
- runtime realization.

A preference must not silently become a mandatory requirement.

---

12. Hints

A hint is weaker than a requirement, constraint, or preference.

An implementation may ignore a hint without changing the program's required semantics.

Examples include:

hint locality;
hint vectorization;
hint accelerator;

The exact meaning of a hint must be defined by its semantic contract rather than by the parser.

---

13. Targets

A target describes an intended execution class or target family.

Examples may include:

cpu
gpu
fpga
asic
quantum
accelerator
heterogeneous
distributed
embedded

These are target categories, not necessarily physical devices.

The grammar must distinguish:

target class

from:

target instance

and:

physical placement

Physical placement is downstream.

---

14. Open-World Resource Kinds

Resource kinds must be open-ended.

The grammar must not permanently enumerate:

CPU
GPU
FPGA
ASIC
QPU
accelerator
memory
storage
network
node

as the complete universe of resource kinds.

These may exist as semantic names, but the grammar must permit future names.

Examples:

compute
memory
accelerator
quantum::logical_qubit
quantum::physical_qubit
tensor::compute
hardware::accelerator
future::resource
future::architecture::resource

Whether a resource kind is known, supported, imported, or meaningful is a semantic/dialect question.

---

15. Resource Names Are Symbolic

A resource name is not automatically a physical identifier.

For example:

resource compute;
resource quantum;
resource accelerator;

does not select:

CPU 0
GPU 0
QPU 0
FPGA 0

Nor does it imply a physical address, memory bank, device path, or cloud instance.

Physical realization belongs downstream.

---

16. No Hardware Limits

The resource grammar must never encode artificial hardware maxima.

The following must not become language-level limits:

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
MAX_TIMELINES

Nor may equivalent restrictions be encoded indirectly.

Forbidden designs include:

resourceCount
    : ONE
    | TWO
    | THREE
    | FOUR
    ;

or:

device
    : DEVICE_0
    | DEVICE_1
    | DEVICE_2
    ;

or grammar-level interpretations such as:

qubit count <= 64
memory <= 64 GB
register width <= 32

---

17. Program Values Are Not Compiler Limits

This distinction is mandatory.

This can be valid program semantics:

let n = 1024;
requires qubits >= n;

The value "1024" is program data.

It does not establish:

MAX_QUBITS = 1024

Likewise:

requires qubits >= 1000000;

must not be rejected merely because a historical machine has fewer qubits.

If a target cannot satisfy it, the appropriate result is a resource/capability/target failure.

---

18. Unbounded Grammar Scalability

The grammar must impose no artificial upper bound on:

- resource declarations;
- resource items;
- requirements;
- constraints;
- capabilities;
- preferences;
- hints;
- properties;
- groups;
- contracts;
- profiles;
- resource expressions;
- qualified resource names;
- nested resource structures;
- resource relationships.

Where repetition is semantically appropriate, grammar composition should use:

*
+

or recursive structures.

"Infinite scalability" means:

«No arbitrary language-level ceiling is embedded in the grammar.»

It does not claim that physical machines, operating systems, parser implementations, memory, compiler infrastructure, or networks are literally infinite.

Actual execution remains bounded by the resources available to the execution environment.

---

19. Quantities Are Expressions

Resource quantities must be expression-valued wherever the semantic model permits.

Examples:

required_memory

workload_size * element_size

input.count

logical_qubits + ancilla_qubits

required_parallelism

available_memory - reserved_memory

The resource grammar must not create a second arithmetic language.

Ordinary expression semantics belong to the canonical expression grammar.

The resource subsystem consumes those expressions.

---

20. No Resource-Specific Expression Language

"resource-expressions.g4" is the resource expression composition boundary.

It must integrate with the canonical expression system.

The intended architecture is:

canonical expression grammar
          |
          v
ordinary expressions
          |
          v
resource-expressions.g4
          |
          v
resource-specific semantic contexts

There must not be competing implementations of:

- arithmetic;
- boolean logic;
- comparison;
- function calls;
- indexing;
- member access;
- precedence;
- associativity.

Resource expressions are contextual uses of the canonical expression system.

---

21. Resource Grammar File Responsibilities

The current directory contains multiple specialized grammar files. Their responsibilities must remain distinct.

"resources.g4"

Role: canonical resource orchestrator.

Owns:

- resource subsystem entry point;
- resource-item dispatch;
- resource declaration composition;
- resource clause composition;
- integration of specialized resource grammars.

Does not own:

- lexer definitions;
- general expression semantics;
- resource discovery;
- hardware allocation;
- physical placement;
- scheduling;
- routing;
- optimization;
- QEC;
- ZQN;
- HAL.

No child grammar may import "resources.g4".

---

"resource.g4"

Role: leaf/component resource grammar.

It must remain composable by "resources.g4".

It must not become another resource orchestrator.

It must not redeclare:

resources
resourceItem
resourceDeclaration

or other universal resource rules owned by "resources.g4".

It must use shared:

- names;
- expressions;
- tokens;
- source-level structures.

It must not create a second resource IR.

---

"resource-expressions.g4"

Role: bridge between resource syntax and the canonical expression grammar.

Owns resource-context expression composition only.

Does not own a second expression precedence system.

---

"requirements.g4"

Role: mandatory resource requirement syntax.

Owns:

requirement

and its resource-specific structural forms.

Does not own:

- capability discovery;
- physical allocation;
- target scheduling.

---

"capabilities.g4"

Role: capability-intent syntax.

It must remain open-world.

It must not enumerate all hardware capabilities.

It must not discover capabilities.

It must not bind capabilities to a physical device.

---

"constraints.g4"

Role: resource-specific constraint syntax.

Generic boolean and comparison semantics belong to the canonical expression system.

---

"preferences.g4"

Role: advisory resource preference syntax.

Preferences must remain semantically weaker than requirements.

---

"hints.g4"

Role: advisory implementation hints.

Hints must never silently change program correctness.

---

"budgets.g4"

Role: resource budget intent.

Budgets must be represented as semantic expressions, not fixed machine constants.

---

"negotiation.g4"

Role: resource/capability negotiation intent.

Negotiation syntax describes conditions or policies.

It does not implement negotiation algorithms.

---

"placement.g4"

Role: explicit placement intent where Zamani semantics require it.

Portable resource semantics must remain separate from physical placement.

---

"scaling.g4"

Role: source-level scaling intent.

It must support workload-dependent and symbolic scaling without imposing finite limits.

---

"portability.g4"

Role: resource-related portability intent.

It must integrate with the global portability specification rather than creating a separate portability model.

---

22. Resource Declarations

A resource declaration introduces a symbolic semantic resource.

For example:

resource compute;
resource memory;
resource accelerator;
resource quantum;

A declaration does not allocate anything.

It does not imply:

- a physical device;
- a physical address;
- a specific node;
- a specific QPU;
- a specific CPU;
- a specific GPU;
- a specific FPGA.

Allocation belongs downstream.

---

23. Resource Properties

Resource properties must be extensible.

Examples include:

capacity
availability
performance
latency
throughput
bandwidth
energy
power
reliability
resilience
scalability
portability
cost

The language must not assume that this list is permanently complete.

Future properties may be introduced through:

- language evolution;
- namespaces;
- dialects;
- semantic extensions.

---

24. Capacity

Capacity represents contextual resource availability.

Examples:

available_memory
available_compute
available_qubits

The grammar represents expressions describing capacity.

It does not establish a permanent maximum.

Capacity is determined by the relevant semantic/environment layer.

---

25. Availability

Availability is contextual and potentially dynamic.

It can depend on:

- resource contention;
- failures;
- scheduling;
- dynamic allocation;
- distributed state;
- hardware state;
- runtime conditions.

The parser must never query the environment.

Availability evaluation belongs downstream.

---

26. Performance

Performance is intentionally abstract.

It may describe:

- required performance;
- preferred performance;
- estimated performance;
- observed performance;
- contextual performance.

The grammar must not define one universal hardware performance model.

---

27. Latency

Latency may be:

- required;
- constrained;
- preferred;
- measured;
- estimated.

The grammar must not impose a universal threshold.

For example:

latency <= latency_budget

is a program condition.

It is not a language-wide latency constant.

---

28. Throughput

Throughput is contextual.

The grammar must permit symbolic expressions such as:

throughput >= required_throughput

without imposing a universal minimum or maximum throughput.

---

29. Bandwidth

Bandwidth may refer to:

- memory bandwidth;
- network bandwidth;
- storage bandwidth;
- interconnect bandwidth;
- accelerator bandwidth;
- other resource-domain bandwidth.

The semantic layer determines the meaning from context.

---

30. Energy and Power

Energy and power are resource properties.

The grammar must not assume:

- one processor voltage;
- one clock frequency;
- one architecture;
- one physical implementation;
- one vendor;
- one fixed energy model.

Physical interpretation belongs downstream.

---

31. Reliability

Reliability expresses semantic intent.

For example:

reliability >= required_reliability

The resource grammar does not decide how reliability is measured.

Measurement, estimation, fault detection, and recovery belong to downstream systems.

---

32. Resilience

Resource syntax may express resilience intent.

However, resilience implementation remains outside the grammar.

The architectural boundary is:

Resource Grammar
       |
       v
Resilience Intent
       |
       v
Semantic Analysis
       |
       v
Resilience System
       |
       +--> detection
       +--> diagnosis
       +--> mitigation
       +--> recovery
       +--> policy
       +--> verification

The resilience vocabulary established elsewhere remains applicable:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

and:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

These are semantic/runtime states and outcomes, not resource grammar algorithms.

---

33. Scalability

Scalability must describe relationships between computation and resources without imposing a fixed upper bound.

Examples include:

workload_size
problem_size
parallelism
required_memory
required_qubits
required_nodes

The same semantic program may be instantiated for:

tiny workload
small workload
large workload
very large workload
distributed workload
heterogeneous workload
future workload

provided the execution environment satisfies the resulting semantic requirements.

---

34. Resource Groups

Resource groups may contain arbitrary numbers of resources.

The grammar must not assume:

group of 2
group of 4
group of 8
group of 16

or any other fixed cardinality.

Groups may be:

- homogeneous;
- heterogeneous;
- statically known;
- dynamically derived;
- workload-dependent.

---

35. Resource Contracts

A resource contract combines resource-related semantic obligations and/or properties.

A contract must remain distinct from physical allocation.

Contracts may participate in:

- compilation;
- target selection;
- deployment;
- runtime negotiation;
- verification.

The grammar only represents the source contract.

---

36. Resource Profiles

A resource profile describes a reusable resource semantic description.

A profile must not become a hidden machine configuration.

Profiles should remain portable unless explicitly declared target-specific by an appropriate downstream mechanism.

---

37. Resource Lifecycle

The grammar may represent:

reservation
acquisition
release

as source intent.

It does not perform allocation.

The architecture is:

source intent
     |
     v
semantic resource request
     |
     v
resource manager
     |
     v
actual allocation

---

38. Resource Derivation

Resource quantities may be derived from program values.

Examples:

required_memory = workload_size * element_size;

required_qubits = logical_qubits + ancilla_qubits;

required_parallelism = workload_size / grain;

The grammar parses the expression.

Semantic analysis determines:

- type validity;
- unit validity;
- dependency validity;
- evaluability;
- overflow behavior;
- feasibility.

---

39. Quantum Integration

The resource subsystem must support quantum resource intent without becoming a quantum implementation.

Examples:

requires qubits >= logical_qubits;

requires capability("quantum.measurement");

requires capability("quantum.mid_circuit_measurement");

requires capability("quantum.error_correction");

The resource grammar must not enumerate quantum gates.

It must not encode:

MAX_QUBITS

It must not select:

physical_qubit(0)

as a portable semantic requirement.

Quantum operations follow the established path:

Zamani source
     |
     v
domain-neutral AST
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
physical target

There must be no second competing quantum IR created by the resource grammar.

---

40. Classical Integration

Resource syntax must support classical computation without binding it to a particular processor.

Examples include requirements for:

- compute;
- memory;
- parallelism;
- vector capability;
- accelerator capability;
- latency;
- throughput;
- energy.

The grammar must not impose:

MAX_CPUS
MAX_THREADS
MAX_REGISTER_WIDTH

or equivalent limits.

---

41. GPU / Accelerator Integration

Portable source may express:

requires capability("gpu.compute");

or:

requires capability("tensor.compute");

without selecting:

GPU 0

or a vendor-specific device.

Vendor-specific realization belongs downstream.

---

42. FPGA / ASIC / HDL Integration

Resource intent may express requirements for:

- reconfigurable computation;
- hardware acceleration;
- memory;
- timing;
- bandwidth;
- interfaces;
- hardware capabilities.

The resource grammar must not assume:

wire [31:0]

as a universal hardware width.

Hardware widths and physical resources belong to the appropriate HDL/hardware semantic layer.

---

43. Distributed Integration

Distributed programs may express:

requires nodes >= required_nodes;

without encoding a fixed number of nodes into the grammar.

The source must not be restricted to:

node0
node1
node2

as the universal model.

Placement, topology, scheduling, replication, communication, and failure handling belong downstream.

---

44. AI and Data Integration

Resource syntax must support requirements arising from:

- tensor computation;
- model training;
- inference;
- datasets;
- memory;
- accelerators;
- distributed computation;
- data movement.

The grammar must not encode specific AI frameworks.

Framework-specific implementation belongs outside the core resource grammar.

---

45. Networking Integration

Resource requirements may refer to:

- bandwidth;
- latency;
- connectivity;
- communication capability;
- network capacity;
- distributed communication.

The grammar must remain independent of a specific network topology unless topology is explicitly part of program semantics.

---

46. Security Integration

Resource/capability requirements may participate in security semantics.

For example:

requires capability("secure.computation");

or a resource constraint requiring a security property.

The grammar does not implement:

- cryptographic algorithms;
- key storage;
- authentication;
- authorization;
- trust evaluation.

Those belong to "grammar/security/" and downstream security systems.

---

47. Interoperability

Resource syntax must remain interoperable with:

- classical compilation;
- quantum compilation;
- HDL synthesis;
- distributed execution;
- accelerator lowering;
- external execution environments.

External formats such as OpenQASM, QIR, HDL formats, LLVM-related representations, or other interoperability representations are not replacements for the canonical Zamani semantic resource model.

---

48. Dialect Integration

Dialects may introduce additional resource kinds or properties.

A dialect must declare:

name
version
owner
syntax extensions
semantic extensions
AST mapping
IR mapping
compatibility
feature gates

A dialect must not silently replace the core resource model.

Unknown dialect-defined resource kinds must not be confused with syntax errors when the dialect is intentionally available for semantic interpretation.

---

49. Source Spans

Every resource-related AST construct must preserve source location information.

At minimum, downstream representation must be capable of identifying:

- beginning of the construct;
- end of the construct;
- relevant resource name;
- relevant operator/keyword;
- nested expression spans.

Diagnostics must therefore be able to report the actual source location of:

- invalid resource syntax;
- invalid resource names;
- invalid resource expressions;
- unsatisfied requirements;
- unsupported capabilities;
- invalid constraints;
- portability failures.

The exact AST types belong to "src/frontend/ast/".

The grammar must not depend on the concrete AST implementation.

---

50. AST Contract

Every resource grammar construct must have a predetermined semantic mapping.

The intended flow is:

Grammar Rule
     |
     v
Frontend AST
     |
     v
Semantic Resource Model
     |
     v
Canonical IR / semantic lowering

Resource syntax must not be added with the assumption:

«"The AST can be figured out later."»

That creates cross-file rework.

The resource directory therefore requires each production to have an explicit AST contract before being considered complete.

---

51. Semantic Contract

Semantic analysis is responsible for:

- resolving resource names;
- resolving resource kinds;
- resolving capabilities;
- checking types;
- checking quantities;
- checking units where applicable;
- checking constraints;
- checking requirement feasibility;
- distinguishing requirements from preferences;
- checking target compatibility;
- checking portability;
- validating dialect-defined semantics;
- producing structured diagnostics.

The parser does not perform these operations.

---

52. Canonical IR Contract

The resource grammar must not create a resource-specific competing IR architecture.

Resource syntax lowers into the existing canonical semantic/IR architecture.

For quantum:

resource intent
      |
      v
semantic resource requirements
      |
      v
quantum semantic analysis
      |
      v
quantum::ir

For classical computation:

resource intent
      |
      v
semantic resource requirements
      |
      v
classical semantic/IR pipeline

For HDL/hardware:

resource intent
      |
      v
hardware semantic model
      |
      v
HDL/hardware IR pipeline

---

53. Compiler Integration

The compiler consumes resource semantics after parsing.

Compiler responsibilities include:

- checking resource requirements;
- determining target compatibility;
- negotiating supported capabilities;
- selecting implementation strategies;
- optimizing according to constraints/preferences;
- producing target-specific realization.

The compiler must not require source rewriting merely because target hardware changes.

---

54. Runtime Integration

The runtime may resolve information that cannot be known statically, including:

- current availability;
- dynamic resource state;
- device health;
- placement;
- scheduling;
- runtime capabilities;
- resource contention;
- dynamic workload size.

Runtime adaptation must preserve declared program semantics.

---

55. HAL Integration

HAL is responsible for actual target facts such as:

- available resources;
- supported capabilities;
- physical topology;
- timing;
- calibration;
- device health;
- target-specific implementation details.

These facts are environment data.

They are not grammar constants.

---

56. Scheduling Integration

Scheduling consumes semantic resource information.

It may determine:

- temporal ordering;
- concurrency;
- resource sharing;
- placement;
- dynamic scheduling;
- quantum scheduling.

The resource grammar does not implement scheduling algorithms.

---

57. Routing Integration

Routing consumes target topology and semantic resource requirements.

For quantum systems, routing may translate:

logical qubits

into:

physical qubits

without changing the portable source program.

The resource grammar must not own this physical mapping.

---

58. QEC Integration

Resource syntax may express requirements such as:

requires capability("quantum.error_correction");

or fault-tolerance-related resource intent.

QEC determines:

- code selection;
- logical-to-physical overhead;
- correction strategy;
- syndrome processing;
- fault-tolerance realization.

The grammar does not implement QEC.

---

59. ZQN Integration

Resource semantics may carry fault/noise/reliability-related requirements into ZQN.

ZQN remains responsible for:

- noise models;
- fault semantics;
- reliability analysis;
- relevant quantum network/fault behavior.

The resource grammar must not duplicate ZQN.

---

60. Resilience Integration

The resource subsystem can provide resilience intent to the resilience subsystem.

The resilience subsystem remains responsible for:

- state observation;
- fault detection;
- recovery;
- retry;
- escalation;
- degradation policy.

The grammar must not encode recovery algorithms.

---

61. Runtime Resource Exhaustion

The following distinction is mandatory:

invalid syntax
        !=
invalid semantics
        !=
unsupported capability
        !=
unsatisfied resource requirement
        !=
runtime resource exhaustion

For example:

requires qubits >= n;

may be perfectly valid syntax and semantics even when a particular target cannot provide enough qubits.

That situation must produce an appropriate resource/capability/target diagnostic.

It must not be reported as a grammar failure.

---

62. Portability

The resource subsystem is a foundational component of POCO-REAF.

The intended model is:

PROGRAM ONCE
     |
     v
Portable Program Semantics
     |
     v
Resource + Capability Requirements
     |
     v
Canonical Semantic Model
     |
     v
Compile / Optimize
     |
     v
Target Realization

The source should express:

what is required
what is permitted
what is preferred
what capabilities are needed
what constraints must hold

rather than:

which CPU
which GPU
which QPU
which physical qubit
which FPGA
which memory bank
which node

unless explicit target-specific semantics are intentionally requested.

---

63. POCO-REAF Meaning

POCO-REAF does not mean that one immutable machine-specific binary must execute directly on every architecture that will ever exist.

It means that:

«The source program's semantic meaning remains stable while compilation and execution systems can choose different valid realizations for different environments.»

Therefore:

same source semantics
        +
different available resources
        +
different capabilities
        |
        v
different valid realizations

provided all required semantics remain satisfied.

---

64. Scaling From Atom to Everywhere

The resource model must be capable of describing computation across scales such as:

atom
molecule
material
embedded system
CPU
multicore
GPU
FPGA
ASIC
accelerator
QPU
workstation
server
HPC system
cluster
distributed system
cloud
heterogeneous system
future architecture

These are not a closed enumeration.

They are examples of possible realizations.

The grammar must remain valid when new architectures are introduced.

---

65. No Physical-Memory Assumptions

The resource grammar must not assume:

RAM = 64 GB
VRAM = 24 GB
register = 32 bits

as universal language properties.

A program may require a quantity:

required_memory

or express a program-specific quantity:

memory >= workload_memory

The target environment determines whether that requirement can be satisfied.

---

66. No Fixed Quantum Capacity

The resource grammar must not establish:

MAX_QUBITS

or equivalent.

These are all valid concepts when represented as program semantics:

logical_qubits
physical_qubits
ancilla_qubits
required_qubits

Their actual availability is a target/environment concern.

---

67. No Fixed CPU/GPU/FPGA Capacity

The same rule applies to:

CPUs
cores
threads
GPUs
FPGAs
ASIC resources
accelerators

The grammar must express semantic requirements rather than compiler-wide maxima.

---

68. No Fixed Distributed Capacity

The resource grammar must not define:

MAX_NODES
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

A program can express:

required_nodes
required_bandwidth
required_connectivity

without turning those values into grammar limits.

---

69. No Fixed Tensor Limits

The resource grammar must not define:

MAX_TENSOR_RANK

or equivalent.

Tensor dimensions and ranks are program semantics.

Actual execution feasibility is target-dependent.

---

70. No Fixed Register Width

The resource grammar must not assume:

MAX_REGISTER_WIDTH

or a universal 32-bit/64-bit machine model.

A program's logical data representation is separate from target register realization.

---

71. No Fixed Timeline Capacity

Where resource intent participates in multi-timeline or speculative execution, the grammar must not establish:

MAX_TIMELINES
MAX_BRANCHES

or equivalent.

Timeline count is determined by program semantics and execution resources.

---

72. Determinism

Resource grammar parsing must be deterministic.

It must depend only on the input token stream and grammar definition.

It must not:

- inspect hardware;
- inspect runtime state;
- inspect network state;
- query cloud providers;
- query device inventories;
- execute resource discovery;
- invoke external services.

The grammar must contain no:

- embedded Rust actions;
- semantic predicates for hardware state;
- runtime callbacks;
- random behavior;
- target-dependent parser branches.

---

73. Rust Safety Contract

The grammar files themselves contain no Rust implementation.

Generated parser integration must satisfy:

Rust 2021
Rust 1.97
Rust 1.97.1
Safe Rust only
No unsafe

No first-party implementation may require:

unsafe

for resource grammar operation.

Any resource discovery or hardware interaction belongs to downstream implementation layers.

---

74. Lexer Vocabulary Integration

The resource subsystem must have one canonical lexical vocabulary.

The current repository contains an important integration point that must be reconciled before the resource grammar stack is considered fully production-ready:

- "grammar/resources/resources.g4" currently declares "tokenVocab = ZamaniLexer";
- "grammar/core/names.g4" currently declares "tokenVocab = ZamaniTokens".

These cannot remain accidental parallel token authorities.

The final architecture must establish one canonical token source and make every imported parser grammar consume that same vocabulary, using the repository's actual ANTLR generation architecture.

This reconciliation belongs to grammar composition/lexer integration.

It must not be solved by inventing resource-local duplicate tokens.

The resource README therefore treats this as a production integration requirement, not as a resource-specific token definition.

---

75. Names Integration

Resource names must use the canonical name grammar.

The resource subsystem must not redefine:

identifier
qualifiedName
nameSegment

The current "grammar/core/names.g4" owns structural name syntax.

Resource meaning is resolved later.

A name such as:

quantum::logical_qubit

is structurally a name.

Its resource meaning is semantic.

---

76. Expressions Integration

Resource expressions must consume the canonical expression architecture.

Do not introduce:

resourceArithmetic
resourceBoolean
resourceComparison

as competing general expression systems unless a genuinely distinct semantic construct is required.

Resource expressions should reuse:

- canonical literals;
- identifiers;
- qualified names;
- calls;
- arithmetic;
- comparison;
- logical operations;
- indexing;
- member access;
- generic expression composition.

---

77. Type Integration

The resource grammar must integrate with "grammar/types/".

Resource types remain distinct from resource requirements.

For example:

Resource<T>

is a type-level concept.

Whereas:

requires memory >= required_memory

is resource intent.

They must not be collapsed into one grammar concept.

---

78. Declaration Integration

The resource grammar must integrate with the declaration system without creating competing declaration dispatch.

"resources.g4" owns resource-specific declaration composition.

"Zamani.g4" owns universal top-level declaration/statement dispatch.

The root grammar must not duplicate every resource production.

---

79. Effects Integration

Resource operations may interact with effects.

For example, resource acquisition or release may have effect semantics.

The resource grammar represents the construct.

Effect analysis determines its semantic effect.

The resource grammar must not implement effect analysis.

---

80. Memory Integration

Resource memory intent must remain separate from the memory subsystem.

The resource grammar can express:

required_memory
memory capacity
memory bandwidth
memory locality

while "grammar/memory/" owns:

- ownership;
- borrowing;
- references;
- regions;
- address spaces;
- persistence;
- memory semantics.

---

81. Concurrency Integration

Resource requirements can describe desired parallelism.

The resource grammar must not impose:

MAX_THREADS

Concurrency syntax belongs to "grammar/concurrency/".

Resource analysis supplies semantic requirements to concurrency and scheduling.

---

82. Hardware Integration

Hardware-specific resource properties belong to "grammar/hardware/".

The resource subsystem may refer to hardware capabilities without becoming a hardware description language.

This separation permits:

software intent
+
resource requirements
+
hardware realization

without forcing portable software source to encode one machine.

---

83. HDL Integration

HDL syntax belongs to "grammar/hdl/".

Resource grammar may describe resource intent relevant to HDL/hardware co-design.

The HDL subsystem remains responsible for:

- ports;
- signals;
- nets;
- registers;
- timing;
- clocking;
- state machines;
- hardware generation;
- verification;
- synthesis intent.

Resource grammar must not duplicate HDL semantics.

Existing files such as:

grammar/hdl/memories.g4

must remain integrated rather than being unnecessarily duplicated under resources.

---

84. Distributed Integration

Distributed resource intent must remain independent from distributed execution semantics.

"grammar/distributed/" owns:

- nodes;
- processes;
- services;
- messages;
- communication;
- replication;
- partitioning;
- consistency;
- deployment.

"grammar/resources/" owns the resource requirement side.

---

85. AI/Data Integration

AI and data domains may generate resource requirements such as:

tensor.compute
memory
bandwidth
accelerator
distributed.execution

The resource subsystem represents these requirements.

AI framework behavior remains outside the resource grammar.

---

86. Security Integration

Resource capabilities may interact with security requirements.

Security implementation remains under:

grammar/security/

The resource grammar must not duplicate:

- identity;
- authorization;
- cryptographic implementation;
- key management;
- trust evaluation.

---

87. Interoperability Integration

Resource semantics may cross interoperability boundaries.

External frontends must lower into the canonical semantic model rather than creating independent resource semantics.

Examples include:

OpenQASM
QIR
HDL
LLVM-related formats
WASM
foreign-language interfaces

These are interoperability mechanisms, not replacements for the Zamani resource model.

---

88. Feature Manifest Integration

Every production resource feature should have a machine-readable feature contract under:

grammar/specification/features/

A resource feature manifest should identify at least:

id
name
status
version
syntax
grammar
lexer_tokens
ast_nodes
semantic_rules
ir_mapping
compiler_consumers
runtime_consumers
domain
capabilities
resource_requirements
portability_class
fallback_policy
hard_coding_policy
negative_tests
boundary_tests
scalability_tests
compatibility

This makes a resource feature independently completable.

---

89. Independent-File Completion Principle

A resource grammar file is not complete merely because ANTLR accepts it.

Before declaring any resource file complete, the following must already be defined:

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

This is mandatory for all production resource grammar components.

---

90. Positive Testing

The resource test suite must cover:

- simple resource declarations;
- qualified resource kinds;
- resource quantities;
- symbolic quantities;
- computed quantities;
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
- resource groups;
- resource contracts;
- resource profiles;
- resource derivation;
- lifecycle intent.

---

91. Negative Testing

Negative tests must include:

- malformed resource declarations;
- malformed resource names;
- malformed qualified names;
- malformed resource expressions;
- missing required values;
- invalid separators;
- invalid resource clauses;
- invalid requirement forms;
- invalid capability forms;
- invalid constraint forms;
- invalid preference forms;
- invalid lifecycle forms;
- conflicting resource semantics where the semantic layer owns the conflict;
- illegal physical assumptions where portability rules prohibit them.

---

92. Boundary Testing

Boundary tests must include:

- zero where semantically legal;
- one;
- large finite values;
- symbolic values;
- derived values;
- empty optional sections;
- single-resource groups;
- many-resource groups;
- deeply qualified names;
- large resource property sets;
- large requirement sets;
- large capability sets;
- large nested resource specifications.

The purpose is to prove that the grammar does not impose artificial ceilings.

---

93. Scalability Testing

Scalability tests must verify the same grammar architecture for:

n = 1
n = 2
n = 1024
n = larger finite values

where "n" represents a program value or resource requirement.

Tests must verify that increasing "n" does not cause rejection solely because of a hidden grammar constant.

For example, this principle is required:

requires qubits >= n;

must remain structurally valid for arbitrary representable program values.

Whether a target can execute it is a separate question.

---

94. No Maximum-Size Tests

The test suite must never accidentally establish an artificial language maximum.

Forbidden:

assert max_qubits == 1024;
assert max_nodes == 1024;
assert max_threads == 256;

Required instead:

assert resource_requirement_is_symbolic;
assert larger_finite_requirement_has_same_grammar_shape;

Tests should establish semantic scaling rather than a hardware ceiling.

---

95. Determinism Tests

The parser must produce the same structural interpretation for the same token sequence.

Tests must verify that parsing is independent of:

- CPU count;
- memory size;
- GPU availability;
- QPU availability;
- network availability;
- runtime state;
- current device health.

---

96. Portability Tests

Portability tests must verify:

same source
+
different valid target environments
=
same required semantics

where each environment satisfies the declared resource/capability contract.

Tests should cover:

- CPU;
- GPU;
- FPGA;
- accelerator;
- QPU;
- simulator;
- embedded;
- distributed;
- heterogeneous;
- future/unknown resource categories through extensible naming.

---

97. Resource Exhaustion Tests

The suite must distinguish:

grammar failure

from:

resource exhaustion

For example:

program requires N resources
target provides fewer

must remain a resource/target failure, not a parser failure.

---

98. Compatibility

Compatibility must be tracked across:

grammar/specification/
grammar/spec/
grammar/resources/
grammar/Zamani.g4
src/lexer.rs
src/parser.rs
src/frontend/ast/
semantic analysis
IR
compiler
runtime

A syntax-compatible change may still be semantically breaking.

Resource semantics therefore require explicit compatibility tracking.

---

99. Diagnostics

Resource diagnostics should distinguish at least:

syntax error
name error
type error
resource error
capability error
constraint error
portability error
target error
runtime resource exhaustion
interoperability error

Diagnostics must identify the source span responsible for the problem.

An unsatisfied resource requirement must not be presented as malformed syntax.

---

100. Security

Resource grammar implementation must remain deterministic and side-effect free.

Parsing must not:

- access files;
- access devices;
- access networks;
- invoke external processes;
- query cloud services;
- inspect credentials;
- inspect hardware state.

Resource discovery and deployment belong to controlled downstream systems.

---

101. Performance

The resource grammar should avoid unnecessary ambiguity and pathological parser behavior.

Production validation should check:

- ambiguity;
- unreachable rules;
- duplicate alternatives;
- excessive recursion;
- accidental exponential behavior;
- unnecessary token lookahead;
- redundant grammar layers.

Performance optimizations must not introduce semantic hardware limits.

---

102. Error Recovery

ANTLR error recovery must not silently turn invalid resource semantics into valid resource intent.

Parser recovery may recover enough structure to continue diagnostics, but production compilation must not silently accept malformed resource contracts.

---

103. Unknown Resource Kinds

Unknown resource kinds may be syntactically valid under the open-world model.

For example:

future::accelerator

may be syntactically valid.

Semantic analysis determines whether:

- it is known;
- it is imported;
- it is supplied by a dialect;
- it is supported by the target;
- it is invalid in the current semantic context.

This distinction is essential for future extensibility.

---

104. Unknown Capabilities

Capability names should similarly remain extensible.

The grammar must not require a closed list of capabilities.

Semantic analysis determines whether a capability is:

known
unknown
provided by a dialect
supported by a target
unsupported

Production behavior must not silently assume an unknown capability is available.

---

105. Vendor Independence

Vendor names may appear where the language explicitly permits namespaced semantic identifiers.

However, the core resource grammar must not encode a fixed vendor list.

For example:

vendor::accelerator::feature

may be structurally representable.

Vendor support is determined by semantic/dialect/backend infrastructure.

---

106. Physical Resource Boundaries

Portable resource semantics should prefer logical descriptions:

logical_qubit
compute
memory
accelerator
bandwidth
latency

Physical realization may later determine:

physical_qubit
device
core
memory_bank
network_link

The source language must not force physical identifiers into otherwise portable programs.

---

107. Explicit Target-Specific Semantics

Target-specific constructs may exist when deliberately requested.

However, they must be visibly distinguished from portable resource intent.

The architecture should therefore make the distinction explicit:

portable resource requirement

versus:

target-specific realization

Target-specific realization must not accidentally become the default interpretation of ordinary resource syntax.

---

108. Semantic Preservation During Adaptation

Target adaptation is permitted.

For example:

more resources
    -> more parallelism

fewer resources
    -> less parallelism

different topology
    -> different routing

different accelerator
    -> different lowering

different quantum gate set
    -> different decomposition

provided the declared program semantics remain satisfied.

---

109. No Silent Semantic Degradation

The implementation must never silently:

- drop a resource requirement;
- ignore a mandatory constraint;
- ignore a capability requirement;
- change resource semantics;
- reduce a requested semantic guarantee;
- silently select an incompatible target;
- silently remove correctness properties.

If a valid realization cannot be produced, compilation/execution must report the appropriate failure.

---

110. Resource Preferences Must Remain Preferences

An optimizer may use:

prefer ...

but must not reinterpret it as:

require ...

unless the source semantics explicitly establish such behavior.

Likewise, a hint may be ignored.

This hierarchy must remain stable:

requirement
    >
constraint
    >
preference
    >
hint

where ">" describes semantic obligation strength, not an optimization ranking.

---

111. Resource Requirements and Capabilities Are Complementary

A resource quantity answers:

«How much or what quantity is required?»

A capability answers:

«What can the environment do?»

Examples:

requires qubits >= n;

and:

requires capability("quantum.measurement");

may both be required.

The grammar must preserve both concepts separately.

---

112. Resource Semantics and Hardware Discovery

Hardware discovery must happen after parsing.

The intended flow is:

source
  |
  v
parser
  |
  v
resource intent
  |
  v
semantic analysis
  |
  v
resource/capability query
  |
  v
hardware environment

Not:

parser
  |
  +--> inspect GPU
  +--> inspect CPU
  +--> inspect QPU
  +--> inspect memory

---

113. Resource Semantics and Compilation

Compilation should consume stable semantic resource requirements.

A new backend should primarily require:

- capability discovery;
- resource discovery;
- canonical IR lowering;
- target realization;
- runtime integration.

It should not require rewriting portable source.

---

114. Future Backend Contract

A future backend must be able to consume the same resource semantics.

Examples:

CPU backend
GPU backend
FPGA backend
ASIC backend
QPU backend
simulator backend
HPC backend
distributed backend
future backend

The resource grammar should not require modification merely because a new target exists.

If a genuinely new language semantic concept is required, it must pass through the normal feature-promotion process.

---

115. Feature Promotion

Resource features originating in "Zamani-Grammar.md" or experimental designs must follow:

Zamani-Grammar.md
        |
        v
Feature Proposal
        |
        v
Semantic Design
        |
        v
AST Contract
        |
        v
Canonical Grammar
        |
        v
Semantic Implementation
        |
        v
IR Contract
        |
        v
Compiler Integration
        |
        v
Runtime Integration
        |
        v
Conformance Tests
        |
        v
Stable Feature

No historical or aspirational resource syntax becomes automatically stable.

---

116. Relationship to "grammar.md"

"grammar/grammar.md" is the implementation-conformance reference.

It should report resource feature status using:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

This directory README must not claim implementation completeness merely because a grammar rule exists.

---

117. Relationship to "Zamani-Grammar.md"

"Zamani-Grammar.md" may contain broader resource concepts, including future or experimental designs.

Those concepts remain historical/extended design until promoted.

This README is not required to duplicate every historical resource proposal.

---

118. Relationship to "DESIGN.md"

"DESIGN.md" remains the higher-level architectural authority.

If a resource grammar design conflicts with "DESIGN.md", the resource grammar must be corrected rather than creating a second architecture.

---

119. Relationship to "spec/resources.md"

"grammar/spec/resources.md" owns the detailed resource semantic specification.

This README explains:

- directory organization;
- grammar ownership;
- integration;
- completion criteria.

It must not create a competing semantic definition.

---

120. Relationship to "grammar/Zamani.g4"

"Zamani.g4" is the language composition root.

It should consume the complete resource subsystem rather than reproducing all resource rules itself.

Conceptually:

Zamani.g4
    |
    +--> Resources
              |
              +--> ResourceExpressions
              +--> Names
              +--> specialized resource components

No reverse import from a child resource grammar into "Zamani.g4" is permitted.

---

121. Import-Direction Rule

The dependency direction must remain acyclic.

Preferred architecture:

shared lexical foundation
        |
        v
shared names / expressions
        |
        v
resource components
        |
        v
Resources orchestrator
        |
        v
Zamani composition root

Never:

resources.g4
    <-->
resource-child.g4

and never:

Zamani.g4
    <-->
resources.g4

where such a cycle is introduced through imports.

---

122. No Parallel Resource Orchestrator

There must be exactly one canonical complete resource grammar composition:

grammar/resources/resources.g4

Do not create:

resource-root.g4
resource-language.g4
resource-main.g4
resources-v2.g4

as competing authorities.

Specialized files must remain components.

---

123. No Duplicate Resource IR

Do not create a second:

ResourceIR
QuantumResourceIR
HardwareResourceIR

architecture merely because the grammar contains resource constructs.

Resource information belongs in the established canonical semantic/IR architecture.

---

124. No Parser-Level Hardware Discovery

Parser generation and parsing must remain independent from:

- CPU inventory;
- GPU inventory;
- FPGA inventory;
- QPU inventory;
- memory inventory;
- network inventory;
- cloud inventory.

This is mandatory for deterministic parsing and portability.

---

125. No Parser-Level Allocation

A resource declaration must not allocate anything.

A requirement must not allocate anything.

A capability expression must not allocate anything.

A preference must not allocate anything.

Allocation belongs to resource-management/runtime infrastructure.

---

126. No Parser-Level Scheduling

Resource grammar must not choose:

- thread schedules;
- GPU streams;
- QPU time slots;
- FPGA placement;
- cluster nodes;
- network routes.

These are downstream implementation decisions.

---

127. No Parser-Level Routing

The grammar may represent topology-related intent.

It must not implement topology routing.

For quantum:

logical program
    |
    v
quantum::ir
    |
    v
routing
    |
    v
physical mapping

---

128. No Parser-Level QEC

The grammar may express fault-tolerance/resource requirements.

It must not implement:

- code selection;
- syndrome decoding;
- correction;
- logical-to-physical expansion.

---

129. No Parser-Level ZQN

The grammar may carry relevant semantic resource requirements.

It must not implement noise/fault modeling.

---

130. No Parser-Level HAL

The grammar must remain independent of the hardware abstraction layer.

HAL is downstream.

---

131. Source Stability

The goal of the resource architecture is that changing:

- CPU architecture;
- GPU architecture;
- FPGA family;
- ASIC implementation;
- QPU topology;
- memory architecture;
- network topology;
- accelerator design;

does not require rewriting portable resource intent.

---

132. Compilation Stability

Resource semantics should survive:

optimization
vectorization
parallelization
distribution
quantum decomposition
routing
scheduling
hardware synthesis

without changing their declared meaning.

---

133. Runtime Adaptation

Runtime systems may adapt realization according to:

- resource availability;
- resource health;
- capability availability;
- workload size;
- scheduling conditions.

Adaptation must preserve semantic requirements.

---

134. Cost

Cost may be represented as an abstract resource property.

The grammar must not hard-code:

- cloud vendors;
- pricing providers;
- currencies;
- machine prices;
- vendor-specific billing models.

Deployment infrastructure determines actual cost interpretation.

---

135. Availability and Failure

If a resource becomes unavailable after compilation, runtime infrastructure may:

- retry;
- recover;
- migrate;
- degrade where explicitly permitted;
- escalate;
- reject execution.

Such behavior must follow the declared semantic policy.

The grammar itself does not perform recovery.

---

136. Provenance

Resource decisions should be traceable downstream.

Where tooling supports provenance, it should be possible to determine:

source resource requirement
        |
        v
semantic resource contract
        |
        v
compiler decision
        |
        v
target realization

This is especially important for:

- reproducibility;
- debugging;
- verification;
- deployment;
- compliance;
- scientific computing;
- quantum execution.

---

137. Reproducibility

Resource realization may differ across environments.

However, declared deterministic semantics must remain reproducible where the language guarantees determinism.

Compilation caches must distinguish:

source semantic identity

from:

target realization

unless the cache contract explicitly guarantees compatibility.

---

138. Incremental Compilation

Resource grammar changes should not force unrelated source recompilation.

Changing a backend-specific resource realization should not require changing portable source.

Changing resource semantics itself is a language compatibility change and must be versioned.

---

139. Versioning

Resource semantic changes must be tracked through:

grammar/compatibility/
grammar/spec/compatibility.md
grammar/specification/

Version changes must account for:

- resource semantics;
- capability semantics;
- requirement semantics;
- preference semantics;
- portability;
- fallback behavior;
- target interpretation.

---

140. Deprecation

Deprecated resource constructs must have:

- a defined warning;
- migration guidance;
- replacement syntax;
- compatibility period;
- removal version where applicable.

Deprecated syntax must not remain silently supported forever.

---

141. Production Validation

The resource subsystem is production-ready only after validation covers:

ANTLR grammar
    |
    v
lexer conformance
    |
    v
parser conformance
    |
    v
AST coverage
    |
    v
semantic coverage
    |
    v
resource analysis
    |
    v
capability analysis
    |
    v
IR integration
    |
    v
compiler integration
    |
    v
runtime integration
    |
    v
portability tests
    |
    v
scalability tests
    |
    v
compatibility tests

Parsing alone is insufficient.

---

142. Grammar Validation

Validation must check:

- ambiguous alternatives;
- unreachable rules;
- duplicate rules;
- duplicate tokens;
- token-vocabulary inconsistencies;
- accidental left recursion;
- precedence conflicts;
- import cycles;
- undefined references;
- duplicate ownership;
- hidden hardware limits;
- fixed resource enumerations.

---

143. Hard-Coding Audit

The resource directory must be continuously checked for prohibited constructs including:

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

and equivalent semantic encodings.

The audit should also identify suspicious fixed physical identifiers such as:

CPU_0
GPU_0
QPU_0
QUBIT_0
NODE_0
MEMORY_BANK_0

when they are being used as universal language semantics rather than explicit target-specific data.

---

144. Hard-Coding Audit Does Not Ban Constants

The audit must distinguish:

program value

from:

language implementation limit

For example:

let n = 1024;
requires qubits >= n;

is not prohibited.

What is prohibited is:

compiler accepts no more than 1024 qubits

as an artificial language restriction.

---

145. Resource Grammar Security

Grammar files must not contain embedded mechanisms that permit:

- arbitrary filesystem access;
- arbitrary network access;
- shell execution;
- hardware probing;
- secret retrieval;
- uncontrolled external calls.

The grammar is a declarative syntax layer.

---

146. Resource Grammar Maintainability

Every resource grammar file must have:

- one clear owner;
- one clear public entry point;
- documented dependencies;
- documented consumers;
- no duplicate general-purpose grammar;
- explicit AST contract;
- explicit semantic contract;
- explicit completion criteria.

Subdirectories should be introduced only where they materially improve maintainability.

Existing filenames must not be renamed merely for cosmetic consistency.

---

147. Recommended Resource Directory

The existing directory should evolve toward:

grammar/resources/
├── README.md
├── resources.g4
├── resource.g4
├── resource-expressions.g4
├── requirements.g4
├── constraints.g4
├── capabilities.g4
├── preferences.g4
├── hints.g4
├── budgets.g4
├── negotiation.g4
├── placement.g4
├── scaling.g4
└── portability.g4

Only create additional subdirectories when the number of independent components justifies them.

Do not create parallel copies of these files.

---

148. Resource Feature Completion Matrix

Every resource feature must be traceable through:

Specification
     |
     v
Grammar
     |
     v
Lexer
     |
     v
AST
     |
     v
Semantic Analysis
     |
     v
Resource Model
     |
     v
IR
     |
     v
Compiler
     |
     v
Runtime
     |
     v
Tests

A feature cannot be considered production-ready when one of these links is undefined.

---

149. Required Cross-Domain Coverage

The resource subsystem must be tested against:

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
memory
concurrency
effects
interoperability
dialects

This prevents resource syntax from becoming accidentally specialized to one computing model.

---

150. Quantum Resource Examples

The resource model must be capable of representing concepts equivalent to:

requires qubits >= logical_qubits;

requires capability("quantum.measurement");

requires capability("quantum.mid_circuit_measurement");

requires capability("quantum.error_correction");

requires capability("quantum.dynamic_control");

without requiring a fixed gate list or fixed QPU architecture.

---

151. Classical Resource Examples

The model must support intent equivalent to:

requires memory >= required_memory;

requires compute >= required_compute;

requires capability("parallel.compute");

requires capability("vector.compute");

without fixing a processor architecture.

---

152. Accelerator Resource Examples

The model must support intent equivalent to:

requires capability("tensor.compute");

requires capability("accelerator.compute");

without requiring:

gpu0
gpu1
gpu2

as universal source constructs.

---

153. Distributed Resource Examples

The model must support intent equivalent to:

requires nodes >= required_nodes;

requires capability("distributed.execution");

requires bandwidth >= required_bandwidth;

without imposing a fixed cluster size.

---

154. Hardware Resource Examples

The model must support semantic intent such as:

requires capability("hardware.reconfigurable");

requires capability("hardware.timing");

requires capability("hardware.acceleration");

without encoding one FPGA, ASIC, or board architecture.

---

155. Portability Classification

Resource features should be classified according to whether they are:

portable
target-aware
target-specific
non-portable

A feature's portability class must be explicit in its feature contract.

---

156. Semantic Portability vs Performance Portability

Zamani can define semantic portability.

Performance portability depends on:

- target capabilities;
- optimization;
- scheduling;
- resource availability;
- implementation quality;
- algorithmic structure.

The resource grammar must not falsely claim that all targets provide identical performance.

---

157. Graceful Target Variation

A valid implementation may choose different realizations:

scalar
vectorized
parallel
distributed
accelerated
quantum
hardware-synthesized

when their capabilities satisfy the program's semantic requirements.

The source resource contract remains stable.

---

158. Failure to Realize

If no valid realization satisfies the declared requirements, the compiler/runtime must explicitly report failure.

It must not:

- silently violate requirements;
- silently remove capabilities;
- silently change semantics;
- silently substitute an incompatible resource;
- silently claim portability.

---

159. Partial Compilation

Tooling may produce a partial artifact for analysis.

Such an artifact must be marked incomplete.

An incomplete artifact must not be presented as a production executable.

---

160. Resource-Aware Optimization

Resource information may guide:

- optimization;
- vectorization;
- parallelization;
- accelerator selection;
- scheduling;
- routing;
- memory placement.

Optimization must preserve semantic requirements.

---

161. Resource Negotiation

Negotiation should be modeled as:

program requirements
        |
        v
available capabilities
        |
        v
candidate realizations
        |
        v
constraint checking
        |
        v
valid realization

The grammar expresses the source-side contract.

Negotiation algorithms belong downstream.

---

162. Dynamic Resources

The architecture must support resources whose availability changes over time.

Examples include:

- cloud resources;
- shared accelerators;
- dynamic QPU availability;
- distributed nodes;
- transient memory;
- runtime-created resources.

The grammar remains deterministic because dynamic state is evaluated after parsing.

---

163. Resource Health

Resource health may be represented downstream using the established resilience state model.

The source program's semantic identity must not change simply because a target transitions from:

Healthy

to:

Degraded

or:

Unavailable

Runtime policy determines whether execution can continue.

---

164. Resource Availability and Semantic Guarantees

A system must distinguish:

resource unavailable

from:

program invalid

A portable program can be valid even when no currently available machine satisfies it.

---

165. Future Computing

The resource model must remain useful when new forms of computing appear.

A new resource category should generally be introducible through:

qualified semantic name
+
capability
+
resource properties
+
semantic contract
+
backend integration

rather than requiring a rewrite of the entire core grammar.

---

166. Nano and Emerging Computing

If nano computing becomes a first-class Zamani domain, resource syntax should be capable of expressing requirements involving:

- atomic-scale computation;
- molecular-scale computation;
- material resources;
- interaction capabilities;
- energy;
- precision;
- environmental conditions.

The resource grammar must not hard-code a periodic table or physical implementation.

---

167. Sankofa Integration

Sankofa-related concepts such as memory, history, recall, learning, temporal state, and provenance belong to their appropriate semantic domains.

Resource grammar may describe resources needed by such computations.

It must not implement Sankofa runtime memory semantics.

---

168. Multi-Timeline Integration

MTS-related execution may consume resource information.

The resource grammar must not impose:

MAX_TIMELINES

or a fixed number of branches.

Timeline semantics belong to the execution subsystem.

---

169. Tooling Integration

Tooling should be able to display:

- resource requirements;
- capabilities;
- constraints;
- preferences;
- hints;
- portability classification;
- target assumptions;
- scalability characteristics;
- unsatisfied requirements;
- target-specific dependencies.

IDE tooling should be able to identify resource portability risks without changing source semantics.

---

170. Documentation Integration

This README is the directory-level navigation and ownership contract.

It should not become:

- another complete language grammar;
- another semantic specification;
- another IR specification.

Detailed semantics belong in "grammar/spec/".

Canonical syntax belongs in ".g4".

Implementation status belongs in "grammar/grammar.md".

Architecture belongs in "DESIGN.md".

Historical design remains in "Zamani-Grammar.md".

---

171. Completion Contract for "grammar/resources/"

The resource directory is production-ready only when all of the following are true:

[ ] directory ownership is explicit
[ ] resources.g4 is the sole resource orchestrator
[ ] specialized grammars have unique ownership
[ ] no resource grammar duplicates general expressions
[ ] no resource grammar duplicates names
[ ] lexer vocabulary is canonical and consistent
[ ] import direction is acyclic
[ ] resource requirements are distinct from constraints
[ ] constraints are distinct from preferences
[ ] preferences are distinct from hints
[ ] capabilities are distinct from resources
[ ] logical resources are distinct from physical resources
[ ] resource intent is distinct from allocation
[ ] resource quantities are expression-based
[ ] resource kinds are open-world
[ ] resource properties are extensible
[ ] no hardware maximum is encoded
[ ] no physical device enumeration is encoded
[ ] no fixed qubit maximum exists
[ ] no fixed CPU maximum exists
[ ] no fixed GPU maximum exists
[ ] no fixed FPGA maximum exists
[ ] no fixed node maximum exists
[ ] no fixed memory maximum exists
[ ] no fixed thread maximum exists
[ ] no fixed tensor-rank maximum exists
[ ] no fixed register-width maximum exists
[ ] no fixed network-size maximum exists
[ ] no fixed device-count maximum exists
[ ] source spans are preserved
[ ] AST mappings are defined
[ ] semantic mappings are defined
[ ] IR integration is defined
[ ] compiler integration is defined
[ ] runtime integration is defined
[ ] HAL boundary is defined
[ ] routing boundary is defined
[ ] scheduling boundary is defined
[ ] QEC boundary is defined
[ ] ZQN boundary is defined
[ ] resilience boundary is defined
[ ] quantum::ir remains canonical
[ ] portability semantics are defined
[ ] deterministic parsing is verified
[ ] positive tests exist
[ ] negative tests exist
[ ] boundary tests exist
[ ] scalability tests exist
[ ] portability tests exist
[ ] compatibility tests exist
[ ] hard-coding audit passes
[ ] Rust 1.97 compatibility is verified
[ ] Rust 1.97.1 compatibility is verified
[ ] safe Rust is maintained
[ ] no unsafe implementation requirement exists

---

172. Definition of "Done" for an Individual Resource File

An individual resource ".g4" file is complete only when:

Purpose
    defined

Ownership
    defined

Non-ownership
    defined

Dependencies
    defined

Imports
    defined

Lexer vocabulary
    defined

Public entry rule
    defined

AST mapping
    defined

Semantic mapping
    defined

IR mapping
    defined

Compiler consumers
    defined

Runtime consumers
    defined

Tooling consumers
    defined

Cross-domain integration
    defined

Positive tests
    defined

Negative tests
    defined

Boundary tests
    defined

Scalability tests
    defined

Determinism tests
    defined

Compatibility tests
    defined

Portability tests
    defined

Hard-coding audit
    passed

Diagnostics
    defined

Security
    defined

Performance
    reviewed

Completion criteria
    satisfied

No later file should need to invent an undocumented AST, semantic meaning, or IR mapping for an already-declared production.

---

173. Production Integration Checklist

Before merging any resource grammar change, verify:

Architecture

[ ] correct owning file
[ ] no duplicate orchestrator
[ ] no circular imports
[ ] no competing specification

Syntax

[ ] ANTLR-valid
[ ] deterministic
[ ] unambiguous
[ ] canonical tokens
[ ] canonical names
[ ] canonical expressions

Semantics

[ ] requirement/constraint/preference/hint distinction preserved
[ ] capability distinction preserved
[ ] logical/physical distinction preserved
[ ] target distinction preserved

Scalability

[ ] no artificial maximum
[ ] no fixed hardware enumeration
[ ] symbolic quantities supported
[ ] arbitrary resource collections supported

Integration

[ ] AST contract
[ ] semantic contract
[ ] IR contract
[ ] compiler contract
[ ] runtime contract
[ ] HAL boundary
[ ] routing boundary
[ ] scheduling boundary
[ ] QEC boundary
[ ] ZQN boundary

Verification

[ ] positive tests
[ ] negative tests
[ ] boundary tests
[ ] scalability tests
[ ] determinism tests
[ ] portability tests
[ ] compatibility tests
[ ] hard-coding audit

---

174. Final Resource Architecture

The complete resource architecture is:

                         Zamani Source
                              |
                              v
                        Zamani.g4
                              |
                              v
                         Resources
                              |
          +-------------------+-------------------+
          |                   |                   |
          v                   v                   v
      Resources          ResourceExpressions    Names
          |
          +-------------------+-------------------+
                              |
             +----------------+----------------+
             |                |                |
             v                v                v
       Requirements      Capabilities      Constraints
             |                |                |
             +----------------+----------------+
                              |
                    Preferences / Hints
                              |
                              v
                     Resource Semantics
                              |
              +---------------+---------------+
              |               |               |
              v               v               v
          Classical       quantum::ir     HDL/Hardware
              |               |               |
              +---------------+---------------+
                              |
                              v
                        Optimization
                              |
                 +------------+------------+
                 |            |            |
                 v            v            v
              Routing     Scheduling   Resilience
                 |            |            |
                 +------------+------------+
                              |
                              v
                             ZQN
                              |
                              v
                             HAL
                              |
                              v
                      Target Realization
                              |
        +----------+----------+----------+----------+
        |          |          |          |          |
        v          v          v          v          v
       CPU        GPU        FPGA       QPU      Future
                                                   targets

---

175. Final Normative Principle

The resource subsystem exists to make the following model possible:

Program
  |
  | expresses computation
  | expresses semantic requirements
  | expresses capabilities
  | expresses constraints
  | expresses preferences
  | expresses portability intent
  v
Stable Zamani Semantics
  |
  v
Canonical Semantic Model
  |
  v
Canonical IR
  |
  v
Target-Aware Compilation
  |
  v
Resource / Capability Negotiation
  |
  v
Optimization
  |
  v
Routing / Scheduling / Resilience
  |
  v
QEC / ZQN where applicable
  |
  v
HAL
  |
  v
Actual Hardware / Runtime

The programmer should describe what the computation requires, not permanently encode which machine must provide it.

Therefore:

resource intent != hardware allocation
resource quantity != compiler maximum
capability != device identity
requirement != preference
logical resource != physical resource
portable semantics != target realization

The resource grammar must remain valid from the smallest meaningful computation to arbitrarily large finite computations supported by the execution environment.

The language must not introduce artificial limits merely because current machines have limits.

A new CPU, GPU, FPGA, ASIC, QPU, accelerator, memory architecture, network, cluster, or future computing architecture should be able to participate through semantic capabilities and target realization without requiring portable source programs to be rewritten.

That is the resource-layer foundation of:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

Production invariant:

«Zamani resource syntax describes portable computational intent. Resource discovery, capability discovery, allocation, placement, optimization, routing, scheduling, resilience, QEC, ZQN, HAL behavior, and physical realization remain downstream responsibilities.»