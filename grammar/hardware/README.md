Zamani Hardware Grammar

Path: "grammar/hardware/"
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Canonical language root: "grammar/Zamani.g4"
Hardware composition root: "grammar/hardware/hardware.g4"
Grammar technology: ANTLR4
Implementation baseline: Rust 2021, Rust 1.97 or later
Safety requirement: Rust implementation MUST use safe Rust; Rust "unsafe" MUST NOT be required
Primary portability objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Scalability objective: From the smallest representable computation to arbitrarily large computations, subject only to program semantics, representation limits, declared requirements, policies, available compiler/runtime resources, and target feasibility
Status: Normative directory-level architecture and production-completion contract

---

1. Purpose

"grammar/hardware/" is the hardware-domain grammar subsystem of Zamani.

This directory provides the source-language representation of hardware intent, while keeping hardware discovery, physical allocation, routing, scheduling, optimization, device control, and runtime realization outside the grammar.

The directory MUST allow Zamani to describe hardware-related intent for:

- classical processors;
- multicore processors;
- accelerators;
- GPUs;
- FPGAs;
- ASICs;
- reconfigurable computing;
- quantum processors;
- quantum simulators;
- memory systems;
- storage;
- interconnects;
- networks;
- heterogeneous systems;
- embedded systems;
- HPC systems;
- clusters;
- distributed systems;
- future computing architectures;
- hardware/software co-design;
- hardware description integration;
- resource-aware computation;
- capability-aware computation;
- deployment;
- reliability;
- resilience;
- performance;
- power;
- thermal characteristics;
- timing;
- calibration intent;
- topology;
- placement;
- negotiation.

The directory MUST remain extensible so that a new hardware technology does not require redesigning the universal Zamani grammar.

---

2. Fundamental Rule

The hardware grammar describes:

«WHAT hardware-related properties, resources, capabilities, requirements, constraints, preferences, and deployment intent a program expresses.»

It does NOT decide:

«WHERE, WHEN, HOW, or ON WHICH PHYSICAL DEVICE the computation is realized.»

Therefore:

Hardware Intent
      |
      v
Semantic Analysis
      |
      v
Resource / Capability / Constraint Analysis
      |
      v
Target-independent planning
      |
      v
Target-specific lowering
      |
      v
Routing / Placement / Scheduling
      |
      v
ZQN / Backend / HAL
      |
      v
Physical Realization

The grammar MUST NOT perform the downstream decisions represented by the lower layers.

---

3. Directory Role

"grammar/hardware/" is a domain subsystem, not a second language.

The ownership hierarchy is:

grammar/Zamani.g4
        |
        v
grammar/antlr/ZamaniParser.g4
        |
        v
grammar/hardware/hardware.g4
        |
        +--> hardware leaf grammars
        |
        v
domain-neutral AST
        |
        v
hardware semantic model
        |
        +--> resources
        +--> capabilities
        +--> constraints
        +--> policies
        +--> effects
        +--> provenance
        |
        v
canonical semantic representation
        |
        +--> Classical IR
        +--> quantum::ir
        +--> HDL / hardware semantic IR
        |
        v
optimization
        |
        v
lowering
        |
        v
routing
        |
        v
placement
        |
        v
scheduling
        |
        v
resilience
        |
        v
ZQN
        |
        v
HAL
        |
        v
target realization

No hardware file may establish an independent pipeline.

---

4. Hardware Directory Composition Root

The file:

grammar/hardware/hardware.g4

is the sole composition root for this directory.

It is responsible for:

1. composing hardware grammar modules;
2. exposing the hardware-domain parser boundary;
3. dispatching declarations to their owning hardware grammars;
4. providing hardware-domain integration points;
5. preventing individual hardware grammars from becoming mutually coupled;
6. maintaining stable directory-level composition;
7. providing extensibility for future hardware domains.

It MUST NOT duplicate the implementation of its leaf grammars.

It MUST NOT become a monolithic hardware grammar.

It MUST NOT define physical hardware limits.

---

5. Complete Hardware File Ownership

The current hardware directory contains the following principal grammar modules:

hardware/
├── README.md
├── hardware.g4
├── accelerators.g4
├── asic.g4
├── calibration.g4
├── capabilities.g4
├── compute.g4
├── constraints.g4
├── cpu.g4
├── deployment.g4
├── devices.g4
├── fpga.g4
├── gpu.g4
├── hardware-constraints.g4
├── interconnect.g4
├── memory.g4
├── negotiation.g4
├── performance.g4
├── placement.g4
├── power.g4
├── qpu.g4
├── quantum-device.g4
├── resources.g4
├── reliability.g4
├── targets.g4
├── thermal.g4
├── timing.g4
└── ...

The exact directory contents MAY grow.

New hardware domains MUST be added as independently owned modules and then composed through "hardware.g4".

---

6. Required Ownership Matrix

The following ownership is normative.

File| Primary responsibility
"hardware.g4"| Directory composition and orchestration
"targets.g4"| Abstract target descriptions and target intent
"devices.g4"| Logical device descriptions
"resources.g4"| Hardware-domain resource declarations/references
"capabilities.g4"| Hardware-domain capability composition
"constraints.g4"| Hardware constraints
"hardware-constraints.g4"| Compatibility/migration candidate; MUST NOT remain a competing constraint authority
"compute.g4"| Generic hardware computation intent
"cpu.g4"| CPU-class specialization
"gpu.g4"| GPU-class specialization
"fpga.g4"| FPGA-class specialization
"asic.g4"| ASIC-class specialization
"accelerators.g4"| Accelerator-class specialization
"qpu.g4"| QPU-class specialization
"quantum-device.g4"| Quantum-device hardware properties
"memory.g4"| Memory hardware intent
"interconnect.g4"| Hardware interconnect intent
"topology.g4"| Topological relationships
"placement.g4"| Logical placement constraints/intent
"timing.g4"| Timing intent
"performance.g4"| Performance properties and requirements
"power.g4"| Power intent
"thermal.g4"| Thermal intent
"reliability.g4"| Reliability/resilience properties
"calibration.g4"| Calibration intent
"negotiation.g4"| Hardware/resource realization negotiation
"deployment.g4"| Deployment intent

If additional files exist or are added, each MUST declare an equivalent ownership contract.

---

7. The Orchestrator Rule

Every file in "hardware/" MUST have exactly one primary owner.

The dependency direction MUST be:

hardware.g4
    |
    +--> targets
    +--> devices
    +--> resources
    +--> capabilities
    +--> constraints
    +--> compute
    +--> placement
    +--> topology
    +--> deployment
    +--> ...

Leaf modules MAY consume shared language infrastructure.

Leaf modules MUST NOT arbitrarily import one another merely because they need a convenient rule.

When two hardware domains need a common concept, the common concept MUST be moved to the appropriate shared owner:

core/
types/
expressions/
resources/
effects/
policies/

or to a properly owned hardware abstraction.

This prevents circular grammar dependencies.

---

8. No Duplicate Authorities

The following must each have one canonical owner:

identifier
qualified name
expression
type
literal
attribute
modifier
resource
capability
requirement
constraint
preference
policy
effect
provenance
target
hardware declaration

Hardware files MUST reuse these authorities.

They MUST NOT redefine them for convenience.

---

9. Relationship to "grammar/Zamani.g4"

"grammar/Zamani.g4" remains the complete-language composition root.

It MUST NOT directly duplicate the contents of "grammar/hardware/".

The intended hierarchy is:

grammar/Zamani.g4
        |
        v
ZamaniParser.g4
        |
        v
hardware.g4
        |
        +--> hardware modules

The hardware directory therefore enters the language through the canonical parser composition hierarchy.

No hardware file may become an alternate complete Zamani grammar.

---

10. Relationship to the Canonical Lexer

Hardware grammar MUST consume the canonical Zamani lexical vocabulary.

The canonical lexical authority remains outside this directory.

Hardware files MUST NOT define an independent:

HardwareLexer
HardwareToken
K_*
HARDWARE_*

vocabulary.

If a genuinely new hardware keyword is required:

hardware requirement
        |
        v
lexical specification
        |
        v
canonical token registry
        |
        v
canonical lexer
        |
        v
hardware parser

A leaf grammar must never silently invent a lexical authority.

---

11. Domain-Neutral Expression Rule

Hardware grammars MUST consume the canonical expression system.

They MUST NOT redefine:

- arithmetic precedence;
- boolean expressions;
- comparison;
- function invocation;
- indexing;
- member access;
- generic expressions;
- literals.

For example, a resource expression such as:

required_memory

or:

workload.size * element.size

must use the canonical expression infrastructure.

This ensures that hardware requirements can depend on ordinary Zamani computation.

---

12. Hardware Intent Model

Hardware intent is represented conceptually as:

HardwareIntent
    =
    target
    + device
    + resource
    + capability
    + requirement
    + constraint
    + preference
    + topology
    + placement
    + timing
    + performance
    + power
    + thermal
    + reliability
    + calibration
    + deployment
    + provenance
    + policy

Not every declaration contains every component.

The model MUST remain extensible.

A future hardware property MUST be representable without redesigning the entire hardware subsystem.

---

13. Requirement / Constraint / Preference / Hint Separation

Hardware semantics MUST distinguish:

Requirement

Must be satisfied.

requires capability("gpu.compute");

Constraint

Limits permitted realizations.

constrain ...

Preference

Desired but not mandatory.

prefer ...

Hint

Optimization guidance only.

hint ...

These MUST NOT be conflated.

In particular:

preference != requirement
hint != requirement

A backend MUST NOT turn an advisory preference into a semantic requirement without explicit policy.

---

14. Capability Model

Hardware capabilities are symbolic and extensible.

Examples:

gpu.compute
tensor.compute
quantum.measurement
quantum.dynamic_control
quantum.error_correction
fpga.reconfiguration
distributed.execution
high_precision.arithmetic

The grammar MUST NOT enumerate every future hardware capability.

Capability identity MUST remain data-driven.

The source expresses:

requires capability("quantum.measurement");

The compiler/runtime determines whether a realization provides that capability.

---

15. Resource Model

Hardware resources MUST integrate with the canonical resource subsystem.

Examples include:

compute
memory
storage
bandwidth
latency
throughput
energy
power
thermal_capacity
quantum
logical_qubits
physical_qubits
accelerator
interconnect

The list is extensible.

The hardware grammar MUST NOT create a competing resource model.

The canonical semantic relationship is:

hardware resource
       |
       v
resource intent
       |
       v
resource analysis
       |
       v
target capabilities
       |
       v
realization

---

16. Unbounded Hardware Scalability

Zamani hardware grammar MUST NOT establish arbitrary universal limits.

It MUST NOT contain or imply equivalents of:

MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_QPUS
MAX_ACCELERATORS
MAX_DEVICES
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_BUS_WIDTH
MAX_PIPELINE_DEPTH
MAX_QUANTUM_BITS
MAX_PHYSICAL_QUBITS
MAX_LOGICAL_QUBITS
MAX_NETWORK_SIZE

Renaming such constants does not make them acceptable.

There MUST be no disguised equivalent such as:

SUPPORTED_DEVICE_COUNT = 1024

or:

MAX_SUPPORTED_QUBITS = ...

in the language architecture.

---

17. Meaning of "Infinity"

POCO-REAF scalability uses unbounded language architecture, not a claim of physically infinite machines.

The language must not establish the upper bound.

Actual execution is limited by:

program semantics
+
representation
+
compiler resources
+
runtime resources
+
target resources
+
target capabilities
+
security policy
+
deployment policy

Therefore:

«"from atom to infinity" means the language architecture does not impose an arbitrary upper hardware bound.»

A finite target may still reject a program because its actual resources are insufficient.

---

18. Representation Limits

Implementation limits MUST be distinguished from language limits.

If an implementation uses a finite representation for a quantity, it MUST:

1. document the representation;
2. detect overflow;
3. reject unrepresentable values where required;
4. avoid silent truncation;
5. avoid silent wrapping when mathematically incorrect;
6. avoid silently changing semantic meaning.

The grammar itself MUST NOT convert an implementation detail into a language-wide capacity limit.

---

19. Symbolic Resource Expressions

Hardware resource quantities MUST be capable of depending on program semantics.

Examples:

required_memory

dataset.size * element.size

logical_qubits + ancilla_qubits

required_parallelism

workload.count

The grammar must not require every resource quantity to be a compile-time literal.

Static, symbolic, conditional, and runtime-evaluated resource semantics MAY be supported where defined by the resource semantic layer.

---

20. Target Abstraction

"targets.g4" MUST describe target intent rather than physical discovery.

A target can conceptually represent:

CPU-class
GPU-class
FPGA-class
ASIC-class
QPU-class
accelerator-class
embedded
HPC
cluster
distributed
simulator
heterogeneous
future

These are target categories, not a closed list of physical machines.

New target classes MUST be expressible without modifying the entire grammar architecture.

---

21. Device Abstraction

"devices.g4" owns logical device descriptions.

A logical device is not automatically a physical device.

The distinction is:

logical device
      !=
physical device

A source declaration MUST NOT silently bind a logical device to:

- PCI address;
- machine ID;
- physical qubit ID;
- driver handle;
- memory address;
- vendor-specific runtime object.

Physical binding belongs downstream unless explicitly expressed through a target-specific mechanism.

---

22. CPU / GPU / FPGA / ASIC / Accelerator Modules

The modules:

cpu.g4
gpu.g4
fpga.g4
asic.g4
accelerators.g4

are specializations of common hardware semantics.

They MUST NOT create separate resource, capability, expression, or policy languages.

They should primarily express domain-specific hardware intent that can be normalized into the common model.

For example:

GPU-class intent
        |
        +--> capability
        +--> compute resource
        +--> memory resource
        +--> topology
        +--> performance
        +--> power
        |
        v
common hardware semantic model

---

23. Quantum Hardware Integration

"qpu.g4" and "quantum-device.g4" integrate quantum hardware into the common hardware model.

They MUST distinguish:

logical quantum resource

from:

physical quantum resource

The hardware grammar MUST NOT perform physical qubit routing.

The semantic pipeline is:

quantum source
      |
      v
domain-neutral AST
      |
      v
quantum semantics
      |
      v
quantum::ir
      |
      v
resource/capability analysis
      |
      v
routing
      |
      v
scheduling
      |
      v
QEC / resilience
      |
      v
ZQN
      |
      v
HAL

The hardware grammar MUST NOT introduce another quantum IR.

---

24. HDL Integration

Hardware grammar and HDL grammar are related but distinct.

"grammar/hdl/" owns HDL language semantics.

"grammar/hardware/" owns hardware-resource and hardware-target intent.

The integration is:

HDL intent
      |
      +--> hardware resource intent
      +--> capability requirements
      +--> timing
      +--> power
      +--> thermal
      +--> reliability
      |
      v
HDL / hardware semantic analysis
      |
      v
synthesis / implementation

Neither subsystem should absorb the other's entire language.

---

25. Topology

Topology describes relationships between abstract resources.

Examples may include:

node
link
connectivity
locality
hierarchy
distance
bandwidth
latency

Topology syntax MUST remain abstract.

It MUST NOT encode a universal physical topology.

Physical topology discovery belongs downstream.

---

26. Placement

"placement.g4" describes placement intent and constraints.

It MUST NOT perform placement.

Correct separation:

placement intent
      |
      v
semantic constraints
      |
      v
placement analysis
      |
      v
target realization

A source program may express locality or affinity without specifying a physical address.

---

27. Interconnect

"interconnect.g4" owns abstract hardware communication relationships.

It may express semantic properties such as:

- bandwidth;
- latency;
- connectivity;
- reliability;
- topology;
- locality;
- communication capability.

It MUST NOT own network protocol syntax that belongs to "grammar/networking/".

---

28. Memory

"memory.g4" describes memory intent.

It MUST support abstract properties such as:

- capacity;
- access characteristics;
- bandwidth;
- latency;
- persistence;
- locality;
- coherence properties;
- capability;
- reliability.

It MUST NOT universally hard-code:

32 GB
64 GB
128 GB
24 GB VRAM

as language limits.

Literal capacities are permitted when they are actual program requirements or declarations.

---

29. Timing

"timing.g4" describes timing intent.

It MUST remain independent of any single clock architecture.

Timing semantics MAY represent:

- duration;
- latency;
- period;
- deadline;
- ordering;
- synchronization;
- temporal constraints.

The grammar MUST NOT establish a universal clock frequency.

---

30. Performance

"performance.g4" describes semantic performance requirements or properties.

Possible properties include:

latency
throughput
bandwidth
parallelism
utilization
efficiency

Performance preferences MUST NOT silently become correctness requirements.

Performance measurement and optimization remain downstream.

---

31. Power and Thermal Semantics

"power.g4" owns power intent.

"thermal.g4" owns thermal intent.

They MUST remain independent from physical power-management implementation.

They may express:

power budget
power preference
thermal constraint
thermal capability

They MUST NOT implement:

- fan control;
- voltage regulation;
- physical sensor access;
- device-driver behavior.

Those belong downstream.

---

32. Reliability

"reliability.g4" owns hardware reliability intent.

It must integrate with the repository-wide resilience model.

Existing resilience concepts such as:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

and outcomes such as:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

remain semantic/runtime concepts rather than grammar-local implementation algorithms.

---

33. Calibration

"calibration.g4" describes calibration requirements and intent.

It MUST NOT implement calibration algorithms.

It MUST NOT directly manipulate a physical device.

Correct boundary:

calibration intent
      |
      v
semantic validation
      |
      v
backend/runtime calibration service
      |
      v
physical target

---

34. Negotiation

"negotiation.g4" integrates hardware intent with resource/capability negotiation.

Negotiation can consider:

requirements
constraints
capabilities
preferences
hints
policies
availability
reliability
performance
power
thermal limits

The grammar describes negotiation intent.

The compiler/runtime performs the actual negotiation.

---

35. Deployment

"deployment.g4" owns hardware-oriented deployment intent.

It may describe:

- target classes;
- deployment constraints;
- resource requirements;
- placement intent;
- topology requirements;
- resilience requirements;
- capability requirements;
- deployment policy.

It MUST NOT become a cloud-provider-specific deployment language.

Provider-specific behavior belongs in dialects, libraries, tooling, or backend integrations.

---

36. Policies

Hardware realization MUST integrate with the repository-wide policy system.

Policies can govern:

resource selection
target selection
placement
power
thermal behavior
security
fallback
simulation
adaptation
deployment
availability
reliability

Hardware files MUST consume the canonical policy model.

They MUST NOT invent another policy syntax.

---

37. Effects

Hardware operations that cross execution boundaries must integrate with the canonical effect system.

Examples may include:

hardware access
measurement
native
foreign
IO
network
distributed
simulation

The grammar only represents effect-bearing constructs.

Effect validation belongs to semantic analysis.

---

38. Provenance

Hardware-related decisions MUST be capable of participating in the repository-wide provenance system.

Provenance may identify:

source intent
derived requirement
selected capability
transformation
optimization
placement decision
routing decision
scheduling decision
fallback
verification
target realization

The grammar MUST preserve source information needed by downstream provenance.

It MUST NOT implement provenance storage itself.

---

39. AI and Adaptive Hardware

Hardware grammar MUST remain domain-neutral while permitting integration with:

- adaptive execution;
- learning-driven optimization;
- hardware-aware inference;
- resource prediction;
- dynamic placement;
- adaptive scheduling.

Such behavior belongs to the semantic/compiler/runtime layers.

Hardware grammar should expose the required intent and metadata, not embed a machine-learning runtime.

---

40. Simulation

Hardware simulation is an execution mode, not a second hardware language.

The intended pipeline is:

hardware intent
      |
      v
semantic model
      |
      v
simulation realization

The same hardware intent SHOULD be usable for:

simulation
emulation
synthesis
real hardware

when semantically valid.

---

41. Safe Rust Requirement

The ANTLR grammar MUST contain no embedded Rust implementation logic.

The Rust compiler implementation consuming the grammar MUST:

- target Rust 2021;
- support Rust 1.97 or later;
- use safe Rust;
- contain no "unsafe" implementation requirement;
- avoid unsafe FFI wrappers;
- avoid unsafe hardware access mechanisms in the core compiler.

Physical hardware interaction belongs behind safe abstractions and validated backend boundaries.

A hardware feature is not production-ready merely because its ".g4" file parses.

---

42. AST Contract

Every hardware grammar construct MUST have a predetermined domain-neutral AST representation.

The AST MUST preserve, as applicable:

- source span;
- declaration kind;
- symbolic identity;
- qualified name;
- resource identity;
- resource expression;
- capability identity;
- requirements;
- constraints;
- preferences;
- topology;
- placement intent;
- timing intent;
- performance intent;
- power intent;
- thermal intent;
- reliability intent;
- calibration intent;
- deployment intent;
- modifiers;
- attributes;
- policy references;
- provenance metadata.

The AST MUST NOT contain physical runtime objects.

It MUST NOT require:

- device handles;
- driver objects;
- physical addresses;
- scheduler state;
- routing state;
- calibration state;
- runtime secrets.

---

43. Semantic Contract

After parsing, hardware constructs enter semantic analysis.

Semantic analysis is responsible for:

- name resolution;
- type checking;
- resource validation;
- capability validation;
- requirement satisfaction;
- constraint checking;
- preference evaluation;
- policy evaluation;
- target compatibility;
- topology validation;
- placement validation;
- timing validation;
- performance validation;
- power validation;
- thermal validation;
- reliability validation;
- provenance;
- portability analysis.

Parsing MUST NOT perform these operations.

---

44. IR Contract

Hardware grammar MUST NOT create a competing universal hardware IR.

Hardware intent MUST normalize into the repository's canonical semantic/IR architecture.

Possible downstream representations include:

Classical IR
quantum::ir
HDL / hardware IR

depending on the semantic domain.

The hardware grammar itself owns no independent executable IR.

---

45. Quantum IR Boundary

Quantum hardware information MUST eventually integrate with:

quantum::ir

The hardware grammar MUST NOT define:

HardwareQuantumIR
QuantumHardwareIR
QHardwareIR

as competing canonical representations.

The intended boundary is:

source
  |
  v
hardware/quantum syntax
  |
  v
domain-neutral AST
  |
  v
quantum semantic analysis
  |
  v
quantum::ir

---

46. Backend Boundary

The hardware grammar MUST stop before:

instruction selection
register allocation
physical qubit mapping
routing
scheduling
device driver invocation
machine-code emission
bitstream generation
QPU command submission

Those belong to compiler/backend/HAL/runtime layers.

---

47. POCO-REAF Requirement

Hardware support is production-ready only when the hardware subsystem preserves:

Program Once
      |
Compile Once
      |
Run Everywhere
      |
Run Anywhere
      |
Forever

POCO-REAF does NOT mean:

«one physical binary is magically native on every architecture.»

It means the program's semantic intent remains portable, while the compiler can derive different valid realizations.

Therefore:

same source meaning
       |
       +--> CPU realization
       +--> GPU realization
       +--> FPGA realization
       +--> ASIC realization
       +--> accelerator realization
       +--> QPU realization
       +--> simulator realization
       +--> HPC realization
       +--> cluster realization
       +--> distributed realization
       +--> future realization

where permitted by semantics and target capabilities.

---

48. Target Failure Semantics

If a target cannot satisfy a mandatory requirement, the implementation MUST NOT silently alter the program.

It MUST report:

1. the unsatisfied requirement;
2. the affected semantic construct;
3. the relevant target/resource/capability;
4. whether an allowed alternative exists;
5. whether simulation, decomposition, distribution, or another realization is possible;
6. the reason a realization was accepted or rejected.

The distinction MUST remain:

valid program
!=
feasible on every target

---

49. Fallbacks

Fallbacks MUST be explicit.

Permitted fallback mechanisms can include:

simulation
decomposition
distribution
alternative accelerator
alternative implementation
recovery
retry

A fallback MUST preserve program semantics.

The compiler MUST NOT silently downgrade a mandatory semantic guarantee.

---

50. Determinism

Parsing hardware grammar MUST be deterministic.

Parsing MUST depend only on:

- source text;
- canonical lexical rules;
- selected grammar;
- explicitly supplied dialect configuration.

Parsing MUST NOT depend on:

- current hardware;
- available devices;
- wall-clock time;
- randomness;
- filesystem state;
- network state;
- environment variables;
- runtime state.

Hardware discovery occurs after parsing.

---

51. Hardware Discovery Boundary

Hardware discovery is NOT grammar responsibility.

The correct architecture is:

source intent
      |
      v
parser
      |
      v
semantic model
      |
      v
hardware discovery
      |
      v
capability/resource negotiation
      |
      v
realization

This is essential for reproducibility and POCO-REAF.

---

52. Vendor Neutrality

The universal hardware grammar MUST NOT be coupled to a vendor.

Vendor-specific hardware can be represented using:

- dialects;
- capability namespaces;
- target metadata;
- libraries;
- backend adapters;
- deployment configuration.

For example, vendor-specific features should be represented as extensible capability identities rather than forcing a new core grammar production for every vendor technology.

---

53. Future Hardware

A new hardware architecture SHOULD be able to enter Zamani through:

new capability namespace
        |
        v
new resource kinds
        |
        v
new target metadata
        |
        v
optional domain grammar
        |
        v
existing semantic model
        |
        v
existing compiler pipeline

A future hardware class MUST NOT require rewriting unrelated CPU, GPU, quantum, HDL, or classical grammar.

---

54. Hardware Extension Protocol

A new hardware subsystem is considered correctly integrated only when it supplies:

syntax
AST contract
semantic contract
resource contract
capability contract
effect contract
policy contract
provenance contract
IR boundary
backend boundary
diagnostics
positive tests
negative tests
boundary tests
scalability tests
portability tests
compatibility tests

The new subsystem then becomes a leaf under:

hardware.g4

rather than becoming another composition root.

---

55. Required Per-File Contract

Every ".g4" file under "grammar/hardware/" MUST document the following before it is considered complete:

PURPOSE
OWNS
DOES_NOT_OWN
DEPENDS_ON
IMPORTS
EXPORTS
CONSUMED_BY
LEXER_DEPENDENCIES
GRAMMAR_DEPENDENCIES
AST_OWNER
AST_CONTRACT
SEMANTIC_OWNER
SEMANTIC_CONTRACT
TYPE_CONTRACT
EFFECT_CONTRACT
CAPABILITY_CONTRACT
RESOURCE_CONTRACT
POLICY_CONTRACT
PROVENANCE_CONTRACT
IR_CONTRACT
QUANTUM_BOUNDARY
HDL_BOUNDARY
BACKEND_BOUNDARY
DIAGNOSTICS
POSITIVE_TESTS
NEGATIVE_TESTS
BOUNDARY_TESTS
SCALABILITY_TESTS
PORTABILITY_TESTS
DETERMINISM_TESTS
COMPATIBILITY
HARD-CODING_AUDIT
COMPLETION_CRITERIA

This contract is what allows a file to be completed independently without reopening it merely because another hardware file is later implemented.

---

56. Dependency Declaration

Each file MUST explicitly identify:

DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
IR_OWNER:
TEST_OWNER:
SPEC_OWNER:

For example:

grammar/hardware/memory.g4

DEPENDS_ON:
    grammar/core/*
    grammar/types/*
    grammar/expressions/*
    grammar/resources/*

EXPORTS:
    hardwareMemoryDeclaration
    hardwareMemoryRequirement
    hardwareMemoryConstraint

CONSUMED_BY:
    hardware.g4
    devices.g4
    targets.g4
    deployment.g4

AST_OWNER:
    domain-neutral frontend AST

SEMANTIC_OWNER:
    hardware/resource semantic analysis

IR_OWNER:
    canonical semantic representation

TEST_OWNER:
    grammar/tests/hardware/memory/

SPEC_OWNER:
    grammar/spec/resources.md
    hardware semantic specification

The exact rules must match the actual repository architecture.

---

57. "hardware.g4" Completion Contract

"hardware.g4" is DONE only when:

- every hardware leaf is explicitly composed;
- every imported rule has one owner;
- there are no duplicate parser authorities;
- there are no circular imports;
- there is no embedded Rust;
- there are no semantic predicates required for hardware discovery;
- all hardware declarations have deterministic dispatch;
- cross-domain boundaries are explicit;
- "ZamaniParser.g4" consumes it correctly;
- "grammar/Zamani.g4" reaches it through the canonical parser hierarchy;
- positive tests pass;
- negative tests pass;
- cross-domain tests pass;
- scalability tests pass;
- parser generation succeeds;
- Rust frontend integration succeeds.

---

58. "hardware-constraints.g4" Resolution

If both:

constraints.g4

and:

hardware-constraints.g4

contain overlapping constraint semantics, they MUST NOT remain competing authorities.

The repository must choose one of:

Option A — Merge

Move unique functionality into the canonical constraint module.

Option B — Adapter

Make "hardware-constraints.g4" a thin compatibility adapter over "constraints.g4".

Option C — Deprecate

Retain it temporarily for compatibility and explicitly mark it deprecated.

The final architecture MUST have one semantic owner for hardware constraints.

No duplicated constraint language is production-ready.

---

59. Cross-Domain Integration

Hardware MUST integrate with:

core
types
expressions
effects
resources
policies
security
classical
quantum
hybrid
hdl
concurrency
distributed
networking
AI
data
execution
compile
interoperability
metaprogramming
compatibility

Integration must use semantic contracts rather than copying grammar rules between directories.

---

60. Hardware + Classical

Classical computation can consume hardware intent:

classical computation
      |
      v
resource requirements
      |
      v
CPU/GPU/accelerator realization

No special hardware-only programming language is required.

---

61. Hardware + Quantum

Quantum computation can consume:

quantum capabilities
quantum resources
QPU properties
connectivity
timing
noise/reliability
calibration
power
thermal constraints

These become semantic inputs to quantum compilation and "quantum::ir".

---

62. Hardware + HDL

HDL can consume hardware intent for:

timing
resources
memory
interconnect
power
thermal
reliability
deployment

Hardware grammar does not replace HDL syntax.

---

63. Hardware + Distributed Computing

Hardware topology can integrate with distributed execution:

logical resources
      |
      v
topology
      |
      v
distributed placement
      |
      v
scheduling

Node count remains runtime/target data rather than a language maximum.

---

64. Hardware + AI

AI workloads can express:

tensor.compute
memory
accelerator
parallelism
throughput
latency
power

AI-specific semantics remain in the AI subsystem.

Hardware provides realization intent.

---

65. Hardware + Security

Hardware declarations may participate in:

capability policy
trust
authorization
sandbox
attestation
provenance
secure deployment

Hardware grammar does not become the security subsystem.

---

66. Hardware + Execution

Execution may consume:

target
device
resource
capability
placement
topology
timing
performance
power
thermal
reliability
deployment

Execution chooses a realization.

The grammar merely supplies the declarative input.

---

67. Hardware + Compilation

Compilation may transform:

hardware intent
        |
        v
specialization
        |
        v
lowering
        |
        v
routing
        |
        v
placement
        |
        v
scheduling

The transformation MUST preserve semantic meaning.

---

68. Hardware + Resilience

Hardware failures are runtime/semantic events.

The hardware grammar can express reliability requirements.

It must not implement recovery algorithms.

Correct boundary:

reliability requirement
      |
      v
semantic validation
      |
      v
execution plan
      |
      v
runtime resilience

---

69. Hardware + Provenance

Every major hardware realization decision SHOULD be traceable where provenance is enabled:

source requirement
      |
      v
candidate capabilities
      |
      v
selected realization
      |
      v
placement
      |
      v
routing
      |
      v
scheduling

This supports reproducibility, auditing, debugging, scientific workloads, and explainability.

---

70. Diagnostics

Hardware diagnostics MUST distinguish at least:

syntax error
unknown hardware construct
unknown resource
unknown capability
unsatisfied requirement
violated constraint
unsupported target
incompatible target
invalid placement
invalid topology
invalid timing
resource exhaustion
policy violation
security violation
unsupported realization

Diagnostics SHOULD identify:

- source location;
- semantic construct;
- requirement;
- target;
- relevant capability/resource;
- reason;
- permitted alternatives where known.

---

71. Negative-Semantics Rule

The compiler MUST never silently turn:

required

into:

preferred

or:

hint

because a target cannot satisfy it.

Similarly, it MUST NOT silently remove:

constraint
policy
security requirement
reliability guarantee

to make compilation succeed.

---

72. Positive Tests

Every hardware module requires valid tests.

Examples include:

minimal hardware declaration
abstract target
logical device
resource requirement
capability requirement
resource constraint
preference
topology
placement
memory
interconnect
timing
performance
power
thermal
reliability
calibration
deployment
quantum hardware
accelerator
heterogeneous target
distributed target

---

73. Negative Tests

Every hardware module requires invalid-input tests.

Examples:

unknown resource syntax
invalid quantity
invalid capability expression
invalid constraint
invalid topology
invalid placement
conflicting requirements
invalid target relationship
malformed deployment
invalid timing relationship

Tests must ensure invalid programs are rejected rather than partially accepted.

---

74. Boundary Tests

Hardware tests MUST cross subsystem boundaries.

Required examples include:

hardware + resources
hardware + capabilities
hardware + policies
hardware + effects
hardware + classical
hardware + quantum
hardware + HDL
hardware + distributed
hardware + execution
hardware + compile
hardware + security

---

75. POCO-REAF Tests

The repository MUST contain tests demonstrating that the same source intent can be analyzed for multiple realizations.

For example:

same source
    |
    +--> CPU
    +--> GPU
    +--> FPGA
    +--> accelerator
    +--> QPU
    +--> simulator
    +--> HPC
    +--> cluster
    +--> distributed

The test is not required to prove that every target is physically capable.

It must prove that the source semantic model is target-independent.

---

76. Scalability Tests

Scalability tests MUST avoid artificial maximum values.

Test dimensions SHOULD include:

small resource set
large resource set
deep resource hierarchy
large symbolic expressions
many capabilities
many constraints
many devices
many target candidates
large topology
large heterogeneous target
distributed realization

The test architecture must scale parametrically rather than encoding a fixed maximum.

---

77. Hard-Coding Audit

Every hardware file MUST be audited for:

MAX_*
LIMIT_*
CAPACITY_*
SUPPORTED_*
COUNT_*
WIDTH_*
SIZE_*

or equivalent disguised constants.

The audit MUST determine whether each number is:

1. actual program data;
2. semantic domain data;
3. implementation representation;
4. an accidental language limit.

Only the first three may be valid, and implementation limits must not be presented as universal language limits.

---

78. Numeric Literals Are Not Hardware Limits

This is valid:

requires memory >= 1024;

if "1024" is the program's actual requirement.

This is invalid architecture:

MAX_MEMORY = 1024;

if that is intended to define the maximum memory supported by Zamani.

The distinction is mandatory.

---

79. No Physical Resource Enumeration

The grammar MUST NOT establish a universal physical universe such as:

cpu0
cpu1
gpu0
gpu1
qpu0
qpu1
qubit0
qubit1

as the fundamental hardware model.

Physical enumeration belongs to target discovery or target-specific deployment.

---

80. No Universal Bus/Register Width

The grammar MUST NOT assume:

32-bit
64-bit
128-bit
256-bit
1024-bit

as universal hardware widths.

Widths that are actual program semantics remain valid.

Universal hardware assumptions do not.

---

81. No Universal Clock

The grammar MUST NOT assume a fixed clock frequency.

Timing must be expressed semantically.

A target may realize timing differently.

---

82. No Universal Memory Architecture

The hardware subsystem MUST NOT assume:

RAM
VRAM
cache hierarchy
NUMA
shared memory

as the only possible architecture.

These may be modeled as capabilities/resources/properties where relevant.

Future memory models must remain representable.

---

83. No Universal Processor Architecture

The subsystem MUST NOT make:

CPU
GPU
FPGA
ASIC
QPU

a closed physical universe.

These are known target categories.

The architecture must remain open to future target types.

---

84. No Vendor Lock-In

A vendor feature belongs in:

dialect
capability namespace
target metadata
backend
library
deployment layer

unless it represents a genuinely universal semantic concept.

---

85. Grammar Purity

Hardware ".g4" files MUST be:

- action-free;
- deterministic;
- parser-focused;
- target-independent;
- free of filesystem access;
- free of network access;
- free of hardware discovery;
- free of runtime execution;
- free of environment inspection;
- free of secrets.

---

86. Rust Integration

The Rust frontend must consume the resulting grammar/semantic architecture through safe interfaces.

The implementation baseline is:

Rust 2021
Rust >= 1.97

The hardware grammar must not require Rust-specific behavior.

No grammar feature is considered complete merely because it can be represented in ANTLR.

It must have a corresponding safe frontend/semantic integration path.

---

87. Generated Parser Compatibility

All grammar imports MUST be compatible with the project's selected ANTLR toolchain.

The hardware composition root MUST be tested independently before complete-language integration.

Required checks include:

ANTLR grammar generation
lexer compatibility
parser compatibility
import resolution
rule resolution
ambiguity analysis
complete-program parsing

---

88. Repository Authority

The hardware subsystem follows the repository authority hierarchy.

Conceptually:

grammar/DESIGN.md
        |
        v
grammar/specification/
        |
        v
grammar/spec/
        |
        v
hardware feature contracts
        |
        v
canonical lexer
        |
        v
ZamaniParser.g4
        |
        v
hardware/hardware.g4
        |
        v
hardware leaf grammars
        |
        v
Rust AST / semantic implementation
        |
        v
canonical IR

"README.md" orchestrates the directory but does not override higher-level normative specifications.

---

89. Relationship to Resource Specification

Hardware resources MUST conform to the repository's canonical resource semantics.

The primary semantic authority is:

grammar/spec/resources.md

Hardware-specific resource syntax is an adapter.

The relationship is:

hardware resource syntax
        |
        v
canonical resource semantics
        |
        v
resource analysis

Hardware MUST NOT redefine the meaning of:

requirement
constraint
capability
preference
hint
resource quantity
resource feasibility

---

90. Relationship to Effects

Hardware constructs that have effects MUST use the repository-wide effects system.

The hardware grammar MUST NOT create a second hardware-specific effect taxonomy.

---

91. Relationship to Policies

Hardware policies MUST use the repository-wide policy architecture.

The hardware directory may expose hardware-specific policy references, but policy semantics remain centralized.

---

92. Relationship to Provenance

Hardware source constructs MUST preserve source locations and metadata necessary for downstream provenance.

The hardware grammar does not own the provenance storage system.

---

93. Relationship to Compatibility

Hardware syntax changes MUST participate in the repository compatibility model.

Changes require:

status
version
migration
deprecation
compatibility tests

A removed hardware construct must not silently change meaning.

---

94. Extensibility Requirement

A new hardware technology should require approximately:

new leaf grammar
+
AST contract
+
semantic contract
+
capability/resource mappings
+
tests
+
hardware.g4 composition

It should NOT require:

rewriting Zamani.g4
rewriting the expression grammar
rewriting the type system
rewriting quantum grammar
rewriting classical grammar
rewriting the lexer architecture

unless the new feature genuinely introduces a universal language concept.

---

95. Independent-File-First Development

Hardware work MUST proceed independently before composition.

Recommended sequence:

1. specification/contract
2. AST contract
3. leaf grammar
4. semantic contract
5. tests
6. composition
7. cross-domain integration

Only after the leaf is independently complete should it be integrated through "hardware.g4".

---

96. File Completion Rule

A file is NOT DONE merely because:

ANTLR accepts it

A file is DONE only when:

Specification
      |
      v
Ownership
      |
      v
Dependencies
      |
      v
Grammar
      |
      v
AST
      |
      v
Semantics
      |
      v
Types
      |
      v
Effects
      |
      v
Capabilities
      |
      v
Resources
      |
      v
Policies
      |
      v
Provenance
      |
      v
IR
      |
      v
Backend boundary
      |
      v
Tests
      |
      v
Scalability
      |
      v
Compatibility
      |
      v
Hard-coding audit

has been satisfied.

---

97. Integration Details Must Be Written Before Completion

Every hardware file must state in advance:

what imports it;
what it imports;
what rules it exports;
what AST node it produces;
which semantic component consumes it;
which resource model it uses;
which capability model it uses;
which effect model it uses;
which policy model it uses;
which provenance model it uses;
which IR receives its meaning;
which tests prove it;
which downstream systems consume it.

This prevents completed files from requiring structural rework merely because another subsystem is subsequently implemented.

---

98. Required Hardware Test Tree

The hardware test architecture SHOULD converge toward:

grammar/tests/hardware/
├── lexical/
├── parser/
├── ast/
├── semantic/
├── resources/
├── capabilities/
├── constraints/
├── targets/
├── devices/
├── compute/
├── cpu/
├── gpu/
├── fpga/
├── asic/
├── accelerators/
├── qpu/
├── quantum-device/
├── memory/
├── interconnect/
├── topology/
├── placement/
├── timing/
├── performance/
├── power/
├── thermal/
├── reliability/
├── calibration/
├── negotiation/
├── deployment/
├── policies/
├── effects/
├── provenance/
├── classical/
├── quantum/
├── hdl/
├── distributed/
├── portability/
├── scalability/
├── compatibility/
├── determinism/
├── negative/
└── cross-domain/

The exact location may follow the repository's existing test organization, but equivalent coverage is mandatory.

---

99. Required Integration Programs

The hardware subsystem should be exercised through progressively larger programs:

hardware-minimal.zm
hardware-resource.zm
hardware-capability.zm
hardware-target.zm
hardware-device.zm
hardware-memory.zm
hardware-topology.zm
hardware-placement.zm
hardware-timing.zm
hardware-performance.zm
hardware-power.zm
hardware-thermal.zm
hardware-reliability.zm
hardware-deployment.zm
hardware-quantum.zm
hardware-hdl.zm
hardware-classical.zm
hardware-heterogeneous.zm
hardware-distributed.zm
hardware-poco-reaf.zm

---

100. Mandatory POCO-REAF Hardware Test

At least one conformance program MUST express hardware intent without naming a specific physical machine.

Conceptually:

requires capability("tensor.compute");
requires capability("quantum.measurement");
requires memory >= required_memory;
requires topology(required_topology);

prefer low_latency;

The semantic representation must remain target-independent.

The same program must be capable of being evaluated against different realizations.

---

101. Hardware-to-Everywhere Test

The test architecture must demonstrate:

one source
   |
   +--> tiny/embedded realization
   |
   +--> CPU realization
   |
   +--> multicore realization
   |
   +--> GPU realization
   |
   +--> FPGA realization
   |
   +--> ASIC realization
   |
   +--> accelerator realization
   |
   +--> QPU realization
   |
   +--> simulator realization
   |
   +--> HPC realization
   |
   +--> cluster realization
   |
   +--> distributed realization
   |
   +--> future-target representation

The test validates the semantic abstraction, not the existence of every physical backend.

---

102. Cross-Domain Master Test

The production integration test should eventually combine:

classical computation
+
tensor computation
+
AI computation
+
quantum computation
+
HDL intent
+
hardware intent
+
resources
+
capabilities
+
effects
+
contracts
+
policies
+
provenance
+
parallelism
+
distributed execution
+
simulation
+
adaptive execution

and verify the path:

Zamani source
      |
      v
lexer
      |
      v
parser
      |
      v
domain-neutral AST
      |
      v
structural validation
      |
      +--> types
      +--> effects
      +--> capabilities
      +--> resources
      +--> contracts
      +--> policies
      +--> provenance
      |
      v
semantic domains
      |
      +--> classical
      +--> quantum
      +--> HDL
      +--> AI
      +--> distributed
      +--> hardware
      |
      v
canonical representation
      |
      +--> Classical IR
      +--> quantum::ir
      +--> HDL/hardware representation
      |
      v
optimization
      |
      v
lowering
      |
      v
routing
      |
      v
placement
      |
      v
scheduling
      |
      v
resilience
      |
      v
ZQN
      |
      v
HAL

---

103. Production Readiness Gate

"grammar/hardware/" MUST NOT be declared production-ready until:

Architecture

- [ ] "hardware.g4" is the sole hardware composition root.
- [ ] No leaf grammar acts as another composition root.
- [ ] Ownership is unambiguous.
- [ ] No circular grammar dependencies exist.
- [ ] No duplicate semantic authorities exist.

Lexical

- [ ] Hardware grammar consumes canonical tokens.
- [ ] No private hardware token namespace exists.
- [ ] Required new tokens are registered centrally.

Syntax

- [ ] All hardware rules are deterministic.
- [ ] Canonical expressions/types/names are reused.
- [ ] ANTLR generation succeeds.
- [ ] Complete-program parsing succeeds.

AST

- [ ] Every exported rule has a predetermined AST contract.
- [ ] Source spans are preserved.
- [ ] No physical runtime objects leak into the AST.

Semantics

- [ ] Resources are validated semantically.
- [ ] Capabilities are validated semantically.
- [ ] Constraints are validated.
- [ ] Preferences remain advisory.
- [ ] Policies are integrated.
- [ ] Effects are integrated.
- [ ] Provenance is preserved.

IR

- [ ] No competing hardware IR exists.
- [ ] Quantum paths reach "quantum::ir".
- [ ] Classical paths reach canonical Classical IR.
- [ ] HDL paths reach the established HDL/hardware semantic boundary.

POCO-REAF

- [ ] Source intent is target-independent.
- [ ] No arbitrary hardware maximum exists.
- [ ] Resource requirements are symbolic where appropriate.
- [ ] Target feasibility is separated from language validity.
- [ ] Target-specific realization is downstream.

Safety

- [ ] Rust 2021.
- [ ] Rust 1.97 or later.
- [ ] Safe Rust only.
- [ ] No Rust "unsafe" requirement.
- [ ] No embedded executable grammar actions.
- [ ] No hardware discovery during parsing.

Testing

- [ ] Positive tests.
- [ ] Negative tests.
- [ ] Boundary tests.
- [ ] Cross-domain tests.
- [ ] Scalability tests.
- [ ] Portability tests.
- [ ] Determinism tests.
- [ ] Compatibility tests.
- [ ] Hard-coding audit.

Only when all gates pass is the directory production-ready.

---

104. Definition of DONE for the Directory

"grammar/hardware/" is DONE when all of the following are true:

Every file has one owner.
Every rule has one authority.
Every dependency is documented.
Every exported rule has an AST contract.
Every AST construct has semantic ownership.
Every semantic construct has an IR destination.
Every resource has canonical resource semantics.
Every capability has canonical capability semantics.
Every effect uses the universal effect model.
Every policy uses the universal policy model.
Every source construct preserves provenance.
Every target realization is downstream.
Every hardware domain composes through hardware.g4.
No arbitrary hardware ceiling exists.
No vendor is hard-coded into the universal model.
No physical device is implicitly selected.
No physical routing occurs in grammar.
No scheduling occurs in grammar.
No hardware discovery occurs during parsing.
No unsafe Rust is required.
All tests pass.

---

105. Final Architecture

The final hardware architecture is:

                         Zamani.g4
                             |
                             v
                     ZamaniParser.g4
                             |
                             v
                    hardware/hardware.g4
                             |
          +------------------+------------------+
          |                  |                  |
          v                  v                  v
       targets            devices            resources
          |                  |                  |
          +------------------+------------------+
                             |
          +------------------+------------------+
          |                  |                  |
          v                  v                  v
     capabilities        constraints        topology
          |                  |                  |
          +------------------+------------------+
                             |
          +------------------+------------------+
          |         |         |        |        |
          v         v         v        v        v
        CPU       GPU       FPGA     ASIC    Accelerator
          |         |         |        |        |
          +---------+---------+--------+--------+
                             |
                      Quantum hardware
                             |
                           QPU
                             |
          +------------------+------------------+
          |                  |                  |
          v                  v                  v
       memory          interconnect         placement
          |                  |                  |
          +------------------+------------------+
                             |
              timing / performance / power
                             |
                   thermal / reliability
                             |
                   calibration / deployment
                             |
                       negotiation
                             |
                             v
                   Domain-neutral AST
                             |
                             v
                    Semantic Validation
                             |
       +---------------------+----------------------+
       |                     |                      |
    Resources           Capabilities             Policies
       |                     |                      |
       +---------------------+----------------------+
                             |
                         Provenance
                             |
                             v
                  Canonical Semantic Model
                             |
              +--------------+--------------+
              |                             |
              v                             v
        Classical IR                   quantum::ir
              |                             |
              +--------------+--------------+
                             |
                         Optimization
                             |
                 +-----------+-----------+
                 |           |           |
                 v           v           v
              Lowering    Routing    Placement
                             |
                         Scheduling
                             |
                         Resilience
                             |
                            ZQN
                             |
                            HAL
                             |
              +--------------+--------------+
              |       |      |      |      |
             CPU     GPU    FPGA   ASIC    QPU
              |       |      |      |      |
              +-------+------+-----+------+
                             |
                    simulator / HPC /
                 cluster / distributed /
                       future targets

---

106. Core Principle

The hardware subsystem must ultimately implement one principle:

«Zamani source code describes computation and hardware intent; it does not describe a fixed physical universe.»

A programmer can therefore describe:

what computation is required
what resources are required
what capabilities are required
what constraints must hold
what policies apply
what outcomes are acceptable

without being forced to encode:

which machine
which processor number
which GPU number
which physical qubit
which memory address
which PCI device
which fixed topology
which fixed hardware capacity

The compiler, runtime, deployment system, routing system, scheduler, resilience layer, ZQN, and HAL determine the realization.

That separation is the foundation required for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.

---

107. Non-Negotiable Invariants

The following invariants apply to every present and future file under this directory:

1. One language.
2. One hardware composition root.
3. One lexical authority.
4. One canonical expression system.
5. One canonical type system.
6. One canonical resource model.
7. One canonical capability model.
8. One canonical effect model.
9. One canonical policy model.
10. One canonical provenance model.
11. One domain-neutral AST.
12. No competing hardware IR.
13. quantum::ir remains the quantum IR boundary.
14. Hardware grammar expresses intent, not realization.
15. No arbitrary universal hardware limits.
16. No implicit physical-device binding.
17. No hardware discovery during parsing.
18. No routing in grammar.
19. No scheduling in grammar.
20. No backend implementation in grammar.
21. No vendor lock-in in the universal grammar.
22. New hardware technologies remain extensible.
23. Rust implementation remains safe.
24. Rust unsafe is prohibited.
25. Every file has an explicit completion contract.
26. Every feature is testable independently.
27. Every integration boundary is defined before completion.
28. Semantic meaning must survive target specialization.
29. Implementation limits must never masquerade as language limits.
30. POCO-REAF is an architectural invariant, not a keyword.

---

108. Orchestrator Responsibility

Finally, this "README.md" is the orchestrator of "grammar/hardware/".

It does not replace the individual ".g4" files.

Instead, it establishes the contract they must collectively satisfy:

README.md
    |
    +--> defines ownership
    |
    +--> defines dependency direction
    |
    +--> defines composition
    |
    +--> defines integration
    |
    +--> defines AST boundaries
    |
    +--> defines semantic boundaries
    |
    +--> defines resource/capability contracts
    |
    +--> defines IR boundaries
    |
    +--> defines POCO-REAF invariants
    |
    +--> defines scalability rules
    |
    +--> defines safety rules
    |
    +--> defines testing requirements
    |
    +--> defines production gates
    |
    v
hardware.g4
    |
    v
all hardware leaf grammars
    |
    v
semantic/compiler/runtime architecture

No individual hardware file may contradict this directory contract.

If a future hardware feature cannot fit this architecture cleanly, the correct response is not to introduce another parallel hardware grammar. The feature must first identify the missing universal abstraction, establish its ownership and integration contract, and then be added as an independent extension under the existing composition root.