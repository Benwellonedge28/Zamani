Zamani Target Compatibility Version Contract

Path: "grammar/compatibility/target-compatibility-version.md"
Status: Normative
Scope: Target compatibility-contract versioning, target realization compatibility, capability/resource negotiation, target-independent portability, migration, deterministic resolution, provenance, and integration with the Zamani compilation and execution architecture
Language: Zamani
Grammar technology: ANTLR4-compatible
Rust baseline: Rust 1.97 or later
Rust edition: 2021
Rust safety: Safe Rust only; "unsafe" MUST NOT be required or used by the Zamani implementation
Primary portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)

---

0. Document Contract

0.1 Purpose

This document defines the normative versioning model for target compatibility contracts in Zamani.

It specifies how Zamani determines whether a compiled semantic artifact, canonical IR, execution plan, or other portable compilation artifact can be realized against a target contract.

This document governs:

- target compatibility-contract identity;
- target compatibility-contract versions;
- compatibility between artifact requirements and target contracts;
- compatibility between target contracts and compiler/runtime capabilities;
- target capability negotiation;
- resource feasibility;
- target feature availability;
- target migration;
- target compatibility deprecation;
- target compatibility provenance;
- deterministic target resolution;
- forward and backward compatibility;
- target-independent portability;
- future computational targets;
- quantum target compatibility;
- HDL/hardware target compatibility;
- distributed/HPC target compatibility;
- simulation target compatibility;
- heterogeneous target compatibility.

It does not define a particular physical machine.

It does not define a universal hardware inventory.

It does not define the target version itself.

It does not define the language version.

It does not define the grammar version.

It does not define the AST version.

It does not define the semantic-contract version.

It does not define the canonical IR version.

It does not define ABI or runtime versioning.

Those are separate compatibility dimensions.

---

0.2 Fundamental distinction

Zamani MUST distinguish:

Target Identity
        +
Target Version
        +
Target Compatibility Contract Version
        +
Target Capabilities
        +
Target Resources
        +
Target Policies

These concepts MUST NOT be collapsed.

A target may change internally while preserving its target compatibility contract.

Conversely, a target compatibility contract may change without requiring a new physical target.

Therefore:

target-version

and

target-compatibility-version

are different version domains.

---

0.3 Core invariant

The fundamental invariant is:

«A target compatibility contract describes whether a target realization can satisfy the semantics and declared requirements of a Zamani artifact; it MUST NOT redefine the meaning of the Zamani program.»

Therefore target compatibility MUST remain downstream from language semantics.

The required architecture is:

Zamani Source
     │
     ▼
Language Version
     │
     ▼
Lexer / Parser
     │
     ▼
Domain-Neutral AST
     │
     ▼
Semantic Model
     │
     ├── Types
     ├── Effects
     ├── Contracts
     ├── Policies
     ├── Provenance
     ├── Capabilities
     └── Resources
     │
     ▼
Canonical IR
     │
     ├── Classical IR
     └── quantum::ir
     │
     ▼
Optimization / Lowering
     │
     ▼
Target Compatibility Analysis
     │
     ├── Target Contract
     ├── Capabilities
     ├── Resources
     ├── Policies
     └── Constraints
     │
     ▼
Routing / Scheduling / Resilience
     │
     ▼
HAL
     │
     ▼
Target Realization

Target compatibility MUST NOT move above semantic analysis.

---

1. Authority and Ownership

1.1 Repository authority

Target compatibility MUST follow the existing Zamani authority hierarchy.

The conceptual hierarchy is:

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ▼
grammar/spec/
        │
        ▼
grammar/compatibility/
        │
        ▼
grammar/Zamani.g4
        │
        ▼
Lexer / Parser
        │
        ▼
Frontend AST
        │
        ▼
Semantic Analysis
        │
        ▼
Canonical IR
        │
        ▼
Optimization / Lowering
        │
        ▼
Routing / Scheduling / Resilience / QEC / ZQN
        │
        ▼
HAL
        │
        ▼
Runtime / Target

This document MUST NOT create a competing language authority.

---

1.2 Ownership matrix

File / subsystem| Owns| Does not own
"grammar/DESIGN.md"| architecture and boundaries| concrete target compatibility records
"grammar/specification/"| normative language meaning| implementation-specific target selection
"grammar/spec/compatibility.md"| general compatibility principles| target-specific version records
"grammar/compatibility/versions.md"| cross-layer version model| target realization
"grammar/compatibility/compatibility-matrix.md"| repository-wide compatibility relationships| target capability probing
"grammar/compatibility/migrations.md"| migration procedures| target discovery
"grammar/compatibility/deprecated.md"| deprecation lifecycle| target execution
"grammar/compatibility/feature-gates.md"| feature availability/lifecycle| physical target allocation
"grammar/compatibility/target-compatibility-version.md"| target compatibility-contract versioning| target physical identity
"grammar/compatibility/target-compatibility.md"| target compatibility semantics| target version numbering
"grammar/hardware/targets.g4"| source-level target intent| compatibility resolution
"grammar/hardware/capabilities.g4"| capability declarations/references| runtime capability discovery
"grammar/resources/"| resource requirements and constraints| physical allocation
"grammar/hardware/"| hardware intent and target abstractions| physical device discovery
"grammar/dialects/"| dialect syntax/registration| target compatibility policy
"src/frontend/ast/"| frontend structural representation| physical realization
semantic analysis| semantic meaning and validation| hardware allocation
Classical IR| canonical classical representation| physical target identity
"quantum::ir"| canonical quantum representation| physical QPU identity
routing| logical-to-physical mapping| source semantics
scheduling| execution scheduling| language meaning
QEC| quantum error correction| dialect versioning
ZQN| fault/noise execution semantics| target compatibility versioning
HAL| hardware/runtime abstraction| language definition
runtime| execution| source grammar

---

1.3 What this document owns

This document owns:

- target compatibility-contract version identity;
- target compatibility-contract version syntax;
- compatibility-contract version evolution;
- compatibility-contract version comparison;
- target compatibility version negotiation;
- compatibility classification;
- target-contract migration;
- compatibility provenance;
- fail-closed unknown-version behavior;
- deterministic target compatibility resolution;
- compatibility requirements between artifact and target contract.

---

1.4 What this document does not own

This document does not own:

- target identity;
- target hardware inventory;
- machine discovery;
- physical device enumeration;
- device selection algorithms;
- routing;
- scheduling;
- resource allocation;
- calibration;
- QEC algorithms;
- runtime execution;
- compiler optimization;
- ABI semantics;
- language semantics;
- AST structure;
- canonical IR structure;
- dialect syntax;
- application-specific semantics.

---

2. Definition of Target Compatibility Version

A Target Compatibility Version identifies the version of the contract used to determine whether a Zamani artifact can be legally and semantically realized against a target environment.

It describes the interpretation of target-facing compatibility information.

Conceptually:

Target Compatibility Contract
        │
        └── Compatibility Contract Version

It is therefore a contract version, not a hardware-generation number.

---

2.1 Why target compatibility needs its own version

A target can change in many ways:

physical hardware
firmware
driver
runtime
compiler
ABI
capability inventory
resource inventory
target contract

These changes have different compatibility consequences.

For example:

Target version:
    8.0

Target compatibility contract:
    3.2

is valid.

Likewise:

Target version:
    9.0

Target compatibility contract:
    3.2

may remain compatible if the new target preserves the same contract.

Conversely:

Target version:
    8.1

Target compatibility contract:
    4.0

may represent a breaking compatibility-contract change.

---

2.2 Target compatibility version is not target capability version

A target capability describes what the target can provide.

Examples:

quantum.measurement
gpu.compute
tensor.compute
hdl.synthesis
distributed.messaging
native.ffi

The compatibility version describes how the compiler interprets the target compatibility contract.

These MUST remain separate.

---

2.3 Target compatibility version is not resource capacity

A target compatibility version MUST NOT encode physical capacity.

The following are invalid architectural uses:

target-compatibility-version = number of CPUs
target-compatibility-version = number of qubits
target-compatibility-version = memory size
target-compatibility-version = GPU count
target-compatibility-version = node count

Capacity belongs to resource negotiation.

---

3. Version Identity

A target compatibility identity SHOULD contain:

target compatibility contract ID
target compatibility version
contract status
supported artifact contracts
supported semantic contracts
supported IR contracts
capability schema compatibility
resource schema compatibility
policy compatibility
migration information
provenance

A conceptual identity is:

target-compatibility:<contract-id>@<version>

The concrete serialized representation belongs to the repository's version/metadata machinery.

---

3.1 Stable identity

A stable target compatibility contract identity MUST NOT be silently reused for unrelated semantics.

Once published:

- its identity remains stable;
- breaking changes require explicit version evolution;
- incompatible interpretation MUST NOT be hidden;
- migrations MUST be explicit;
- compatibility MUST be testable.

---

3.2 Open-world target identities

Target identities MUST remain open.

The grammar MUST NOT require a closed enumeration such as:

CPU
GPU
FPGA
ASIC
QPU

as the complete universe.

Those are examples of target classes.

The architecture must permit:

future::processor
future::accelerator
photonic::compute
neuromorphic::compute
molecular::compute
hybrid::compute
custom::domain::target

without changing the target compatibility-version mechanism.

---

4. Version Format

Stable target compatibility contracts SHOULD use:

MAJOR.MINOR.PATCH

with optional pre-release and build metadata according to the repository's general versioning authority.

This document does not redefine general version parsing.

The common version rules belong to the repository's versioning specifications.

---

4.1 No language-level version ceilings

The target compatibility model MUST NOT define:

MAX_TARGET_VERSION
MAX_TARGET_COMPATIBILITY_VERSION
MAX_VERSION_COMPONENT
MAX_TARGETS
MAX_CAPABILITIES
MAX_RESOURCES
MAX_FEATURES

as language semantics.

Version components MUST be treated as logical version values rather than fixed-capacity hardware quantities.

An implementation may choose an internal representation appropriate to Rust and its dependencies, but an implementation representation limit MUST NOT become a Zamani language restriction.

---

4.2 Major version

A major target compatibility-contract version is required when the interpretation of target compatibility information becomes incompatible.

Examples include:

- changing the meaning of a required capability;
- changing the meaning of a resource requirement;
- changing the compatibility interpretation of a stable IR contract;
- removing a mandatory compatibility field;
- changing target-contract semantics incompatibly;
- changing migration semantics incompatibly;
- changing compatibility status interpretation;
- changing deterministic resolution semantics incompatibly.

---

4.3 Minor version

A minor version MAY add backward-compatible functionality.

Examples:

- new optional compatibility metadata;
- new optional capability descriptors;
- new optional resource descriptors;
- new diagnostic categories;
- new target classes;
- new compatibility evidence;
- new non-breaking target policy metadata.

Existing compatible artifacts MUST retain their specified meaning.

---

4.4 Patch version

A patch version MAY correct:

- documentation;
- diagnostics;
- metadata errors;
- conformance errors;
- non-semantic implementation bugs;
- deterministic resolution defects;
- migration metadata errors.

A patch version MUST NOT silently change target compatibility meaning.

---

5. Compatibility Dimensions

Target compatibility MUST be multidimensional.

A single boolean such as:

targetCompatible = true

is insufficient for production use.

At minimum, compatibility MUST distinguish:

1. language compatibility;
2. grammar compatibility;
3. AST compatibility;
4. semantic-contract compatibility;
5. type compatibility;
6. effect compatibility;
7. capability compatibility;
8. resource feasibility;
9. policy compatibility;
10. provenance compatibility;
11. Classical IR compatibility;
12. "quantum::ir" compatibility;
13. ABI compatibility;
14. runtime compatibility;
15. target compatibility;
16. execution compatibility;
17. determinism/reproducibility compatibility.

---

5.1 Compatibility is conjunctive where required

A target realization is valid only when every mandatory compatibility dimension is satisfied.

Conceptually:

Language
AND
Semantic
AND
IR
AND
Target Contract
AND
Capability
AND
Resource
AND
Policy
AND
ABI/Runtime

where applicable.

---

5.2 Compatibility is not feasibility

The following distinctions are mandatory.

Language incompatibility

The program requires a language semantic contract the compiler does not support.

Dialect incompatibility

A required dialect contract cannot be interpreted.

IR incompatibility

The artifact uses an IR contract unsupported by the target-facing toolchain.

Target-contract incompatibility

The target does not satisfy the declared target compatibility contract.

Capability unavailability

The target lacks a required capability.

Resource infeasibility

The target has insufficient resources for the particular workload.

Policy rejection

The target could execute the artifact but policy prohibits execution.

Runtime incompatibility

The runtime cannot execute the compatible artifact.

These MUST produce distinguishable diagnostics.

---

6. Compatibility Result Model

A production resolver SHOULD expose explicit result classes.

Recommended conceptual statuses:

COMPATIBLE
SOURCE_COMPATIBLE
SEMANTIC_COMPATIBLE
IR_COMPATIBLE
TARGET_COMPATIBLE

CAPABILITY_UNAVAILABLE
RESOURCE_INFEASIBLE
POLICY_REJECTED
RUNTIME_INCOMPATIBLE
ABI_INCOMPATIBLE

MIGRATION_REQUIRED
UNSUPPORTED_VERSION
UNKNOWN_VERSION
INCOMPATIBLE
CONFLICT
AMBIGUOUS
INVALID

---

6.1 Unknown is not compatible

Unknown compatibility information MUST fail closed.

For example:

target compatibility contract = unknown

MUST NOT be interpreted as:

compatible

unless an explicit compatibility contract says that unknown information is safe.

For security-sensitive or semantic compatibility decisions, unknown MUST be rejected.

---

6.2 Capability failure is not version failure

If a target supports:

target compatibility version = compatible

but lacks:

capability("quantum.measurement")

the result MUST NOT be:

TARGET_COMPATIBILITY_VERSION_INCOMPATIBLE

It SHOULD instead be:

CAPABILITY_UNAVAILABLE

This distinction is essential for POCO-REAF.

---

6.3 Resource failure is not semantic failure

For example:

requires memory >= required_memory

may fail on a particular target.

That does not make the source program semantically invalid.

The correct classification is:

RESOURCE_INFEASIBLE

rather than:

LANGUAGE_INCOMPATIBLE

or:

SEMANTIC_INCOMPATIBLE

---

7. Target Contract Inputs

Target compatibility analysis consumes target-facing information from several independent sources.

Conceptually:

Target Identity
       │
Target Version
       │
Target Compatibility Version
       │
Capabilities
       │
Resources
       │
Topology
       │
Policies
       │
Runtime / ABI
       │
       ▼
Target Compatibility Resolver

---

7.1 Target identity

Target identity answers:

«What abstract target contract is being considered?»

Examples:

classical::cpu
classical::gpu
hardware::fpga
hardware::asic
quantum::qpu
quantum::simulator
distributed::cluster
hpc::compute
hybrid::compute

These are symbolic identities.

They do not identify physical devices.

---

7.2 Target version

Target version describes the target's own version.

It is separate from compatibility-contract version.

For example:

target = quantum::qpu
target-version = 7.4
target-compatibility-version = 3.1

---

7.3 Capability contract

Capabilities describe available semantic abilities.

Examples:

capability("quantum.measurement")
capability("quantum.dynamic-circuit")
capability("gpu.compute")
capability("tensor.compute")
capability("hdl.synthesis")
capability("distributed.messaging")

The capability namespace MUST remain extensible.

---

7.4 Resource contract

Resources describe quantities or classes available to a realization.

Examples:

qubits
memory
compute
storage
bandwidth
latency
energy
thermal_budget
parallelism

The resource model MUST remain symbolic and scalable.

No fixed physical ceiling belongs here.

---

7.5 Topology contract

Targets may expose topology.

Examples:

linear
mesh
torus
tree
fully_connected
hierarchical
custom

Topology descriptions are target metadata.

They MUST NOT become permanent language limitations.

---

8. Relationship to "grammar/hardware/targets.g4"

"grammar/hardware/targets.g4" owns source-level target intent.

This document owns target compatibility-contract versioning.

The relationship is:

targets.g4
     │
     ▼
Target Intent
     │
     ▼
AST
     │
     ▼
Semantic Target Model
     │
     ▼
Target Compatibility Resolver
     │
     ├── target version
     ├── target compatibility version
     ├── capabilities
     ├── resources
     ├── policies
     └── constraints

"targets.g4" MUST NOT implement compatibility negotiation.

This document MUST NOT duplicate "hardwareTargetDeclaration".

---

9. Target Compatibility Contract

A target compatibility contract SHOULD identify:

target identity
target version
target compatibility version
supported language contracts
supported semantic contracts
supported AST contracts where applicable
supported IR contracts
supported dialect contracts where applicable
capability schema
resource schema
policy schema
ABI compatibility
runtime compatibility
migration support
deprecation state
provenance

---

9.1 Contract example

An illustrative abstract contract may contain:

target:
    quantum::qpu

target_version:
    7.4

target_compatibility:
    3.1

language:
    compatible_with >= 1.0 < 2.0

semantic:
    compatible_with >= 2.0 < 3.0

ir:
    compatible_with classical::ir >= 1.0
    compatible_with quantum::ir >= 2.0

capabilities:
    quantum.measurement
    quantum.dynamic-circuit

resources:
    qubits
    memory
    execution_time

This is metadata, not a physical allocation.

---

10. Classical IR Integration

A target compatibility contract MAY declare supported Classical IR contracts.

The relationship is:

Zamani semantic model
        ↓
Classical IR
        ↓
target compatibility
        ↓
lowering
        ↓
HAL

A target MUST NOT reinterpret an unsupported Classical IR operation as a different semantic operation merely to obtain execution.

If translation exists, it MUST be explicitly declared and validated.

---

11. "quantum::ir" Integration

"quantum::ir" remains the canonical quantum IR boundary.

The required path is:

Quantum source
     ↓
Quantum semantic model
     ↓
quantum::ir
     ↓
quantum::ir compatibility validation
     ↓
target compatibility
     ↓
routing
     ↓
scheduling
     ↓
QEC / resilience where applicable
     ↓
ZQN
     ↓
HAL
     ↓
QPU / simulator / future quantum target

A target compatibility contract MUST NOT create a competing quantum IR.

The following are prohibited as replacement authorities:

TargetQuantumIR
VendorQuantumIR
QpuIR
DeviceQuantumIR

as universal Zamani semantic boundaries.

A target-specific representation may exist downstream as an implementation artifact, but it MUST NOT replace "quantum::ir".

---

12. HDL Integration

HDL target compatibility follows:

HDL intent
    ↓
HDL semantic model
    ↓
HDL canonical representation
    ↓
target compatibility
    ↓
synthesis / simulation / verification
    ↓
FPGA / ASIC / hardware realization

Target compatibility MUST distinguish:

HDL language compatibility

from:

synthesis capability

and:

physical resource availability

A target lacking a synthesis capability is not automatically a language incompatibility.

---

13. Quantum Target Compatibility

Quantum compatibility MUST distinguish at least:

logical quantum requirements
physical target capabilities
resource feasibility
topology
native realization
measurement
dynamic control
noise/resilience
QEC requirements

For example:

requires capability("quantum.measurement")

is a capability requirement.

It is not a declaration that the target has a fixed number of measurement units.

Likewise:

requires qubits >= n

is a resource requirement.

It does not define a universal maximum.

---

13.1 Quantum operation compatibility

Quantum operations SHOULD remain open and data-driven.

Target compatibility MUST NOT require a universal enumeration such as:

H
X
Y
Z
CNOT

as the complete operation universe.

Instead, compatibility should evaluate:

operation identity
parameters
targets
results
effects
capabilities
resource requirements
lowering availability

This permits future and vendor-defined operations through dialect/metadata mechanisms.

---

13.2 Quantum target compatibility failure

If:

quantum::ir

is valid but a target cannot realize a required operation, the resolver MUST distinguish:

IR_COMPATIBLE

from:

LOWERING_UNAVAILABLE

or:

CAPABILITY_UNAVAILABLE

rather than declaring the source program semantically invalid.

---

14. Resource Compatibility

Target compatibility MUST consume the universal resource model.

The target does not impose universal limits on programs.

Instead:

program requirement
        +
target resource availability
        =
feasibility result

Examples:

requires memory >= required_memory
requires qubits >= required_qubits
requires compute >= required_compute
requires bandwidth >= required_bandwidth

The values MUST be evaluated from program semantics, workload information, or target metadata.

---

14.1 No fixed hardware capacities

The compatibility specification MUST NOT introduce:

64GB
24GB
32-bit
8 threads
100 nodes
1000 qubits

as universal language assumptions.

Such values may exist in tests as explicit test data, but they MUST NOT become language rules.

Production compatibility MUST support targets whose resources are smaller, larger, or structurally different.

---

14.2 Symbolic resources

Where a requirement cannot be known statically, the compatibility system MAY preserve it symbolically.

Example:

required_memory
required_qubits
required_bandwidth
required_parallelism

The final target resolver may evaluate the requirement against the available target resources.

---

15. Capability Compatibility

Capabilities MUST be namespaced and open.

Examples:

quantum.measurement
quantum.dynamic-circuit
gpu.compute
tensor.compute
fpga.synthesis
asic.synthesis
distributed.messaging
network.secure-channel
native.ffi

The target compatibility contract determines whether the target-facing capability schema can represent the requirement.

The actual capability availability is a separate result.

---

15.1 Capability versioning

Capability definitions MAY evolve.

A capability compatibility contract MUST distinguish:

capability exists

from:

capability has compatible semantics

A capability with the same spelling but incompatible meaning MUST NOT be treated as compatible solely because its identifier matches.

---

15.2 Capability aliases

Aliases MAY be used where explicitly declared.

For example:

old capability
    ↓
migration alias
    ↓
new capability

An alias MUST define:

- source capability;
- target capability;
- semantic equivalence;
- version range;
- migration status;
- deprecation state.

Silent aliases are prohibited.

---

16. Policy Compatibility

Target compatibility MUST incorporate applicable policies.

Policies may constrain:

security
resources
deployment
execution
adaptation
networking
native calls
foreign calls
reflection
simulation
quantum execution
distributed execution

A policy rejection MUST remain distinct from a semantic incompatibility.

Example:

target supports native.ffi

but:

policy forbids native.ffi

The result is:

POLICY_REJECTED

not:

CAPABILITY_UNAVAILABLE

---

17. Effect Compatibility

Target compatibility MUST respect effects.

Relevant effects may include:

io
network
mutation
randomness
native
foreign
distributed
measurement
quantum
learning
adaptation
reflection
code_generation
simulation

An effect requirement MUST be checked against:

target capabilities
runtime capabilities
security policies
execution policies

A target MUST NOT silently remove an effect.

---

18. Contract Compatibility

Target compatibility MUST preserve:

requires
ensures
invariant
assume
guarantee
property

Target-specific optimization MUST NOT invalidate a program's contracts.

If a target cannot satisfy a required contract, compilation or realization MUST fail with a contract-related diagnostic.

---

19. Deterministic Compatibility Resolution

Target compatibility resolution MUST be deterministic.

For the same:

source
language contract
semantic contract
IR contract
dialect set
target contract
capability snapshot
resource snapshot
policy set
compatibility policy

the resolver MUST produce the same compatibility classification under the same resolver version.

---

19.1 No order-dependent resolution

Resolution MUST NOT depend on:

- filesystem enumeration order;
- hash-map iteration order;
- thread scheduling;
- nondeterministic discovery order;
- plugin loading order;
- network response order;
- target enumeration order.

Stable identifiers and explicit ordering MUST be used.

---

19.2 Ambiguous target compatibility

If two target compatibility interpretations are possible and neither is explicitly preferred, the resolver MUST return:

AMBIGUOUS

It MUST NOT silently select one.

---

20. Target Compatibility Negotiation

The negotiation sequence SHOULD be:

1. Identify target
2. Identify target version
3. Identify target compatibility contract
4. Validate compatibility-version syntax
5. Validate language compatibility
6. Validate semantic compatibility
7. Validate AST compatibility where applicable
8. Validate Classical IR compatibility
9. Validate quantum::ir compatibility
10. Resolve dialect compatibility
11. Resolve capability requirements
12. Resolve resource requirements
13. Resolve effects
14. Resolve policies
15. Resolve ABI/runtime compatibility
16. Determine migration requirements
17. Produce compatibility result
18. Record provenance
19. Permit downstream lowering only when valid

---

20.1 Fail-closed negotiation

If any mandatory compatibility contract is unknown:

UNKNOWN

MUST be returned.

The compiler MUST NOT guess.

---

20.2 No nearest-version guessing

If the requested target compatibility version is:

5.7

and the target provides:

5.6
5.8

the resolver MUST NOT automatically choose one merely because it is numerically close.

It may select one only when the declared compatibility range explicitly permits it.

---

21. Forward Compatibility

A newer target compatibility contract MAY support an older artifact if the newer contract explicitly preserves the required semantics.

Numeric ordering alone does not prove semantic compatibility.

Therefore:

target compatibility 4.x

MUST NOT automatically imply:

artifact compatible

for every:

target compatibility 3.x

unless the compatibility contract says so.

---

22. Backward Compatibility

An older target compatibility contract MAY support an artifact built against a newer contract only when an explicit migration or compatibility adapter proves semantic equivalence.

Otherwise:

MIGRATION_REQUIRED

or:

INCOMPATIBLE

MUST be returned.

---

23. Migration

Target compatibility migrations belong with the repository's migration framework.

This document defines the requirements for target-compatibility migration.

A migration MUST identify:

source compatibility version
target compatibility version
migration identity
migration rules
semantic preservation
loss conditions
diagnostics
provenance

---

23.1 Migration must preserve semantics

A migration MUST NOT silently change:

- program meaning;
- type meaning;
- effect meaning;
- capability meaning;
- resource semantics;
- contract meaning;
- quantum semantics;
- HDL semantics.

If semantic preservation cannot be proven, automatic migration MUST NOT occur.

---

23.2 Lossy migration

A migration MAY be lossy only when the loss is explicitly represented and the applicable language/compatibility policy permits it.

Examples:

feature unsupported
precision reduced
optional metadata discarded
diagnostic metadata unavailable

Mandatory semantic information MUST never be silently discarded.

---

24. Deprecation

A target compatibility contract or field MAY become deprecated.

Deprecation MUST include:

introduced
deprecated
migration path
replacement
removal conditions
compatibility impact

A deprecated compatibility contract MAY remain usable for a defined compatibility window.

Removal MUST follow the repository's deprecation authority.

---

25. Provenance

Every resolved target compatibility decision SHOULD be traceable.

The provenance record SHOULD contain:

artifact identity
language version
grammar version where applicable
AST contract version where applicable
semantic contract version
Classical IR version where applicable
quantum::ir version where applicable
dialect identities and versions
target identity
target version
target compatibility version
capability snapshot
resource snapshot
policy snapshot
migration path
resolver version
compatibility result
diagnostic evidence
timestamp where operationally appropriate

Provenance MUST distinguish source facts from runtime observations.

---

25.1 Reproducibility

For reproducible compilation, a compatibility decision SHOULD be reproducible from a captured compatibility snapshot.

The snapshot SHOULD contain enough information to reproduce:

target compatibility result
capability decisions
resource decisions
policy decisions
migration decisions

A live target may change after compilation.

Therefore reproducible builds SHOULD use an explicit compatibility snapshot or equivalent immutable record.

---

26. POCO-REAF

Target compatibility is one of the central mechanisms enabling POCO-REAF.

The source program describes:

intent
requirements
constraints
capabilities
preferences
policies
contracts

The program does not need to encode:

physical CPU
physical GPU
physical QPU
physical FPGA
physical memory bank
physical network path
physical qubit mapping

The architecture is:

One source program
       │
       ▼
One semantic meaning
       │
       ▼
Portable IR
       │
       ▼
Target compatibility
       │
       ▼
Capability/resource negotiation
       │
       ▼
Target-specific lowering
       │
       ▼
Execution

---

26.1 Target changes

Changing:

CPU → GPU
GPU → FPGA
FPGA → ASIC
ASIC → accelerator
accelerator → QPU
QPU → simulator
single machine → cluster
cluster → HPC
HPC → distributed system

MUST NOT inherently require changing the source program.

The target may require a different lowering or realization strategy.

---

26.2 Target failure

If a target cannot satisfy the program:

target compatibility
        +
capability/resource analysis
        ↓
failure

the compiler MUST report why.

It MUST NOT rewrite the program's semantics automatically.

---

27. Scaling from Tiny to Arbitrarily Large Systems

Target compatibility MUST be open-ended.

The language has no universal physical maximum.

The same compatibility mechanism MUST support:

tiny embedded target
single-core target
multicore target
manycore target
GPU target
FPGA target
ASIC target
accelerator target
QPU target
simulator
HPC system
cluster
distributed system
cloud system
heterogeneous system
future computational substrate

subject only to actual resource and capability availability.

---

27.1 No target-size semantics

The compatibility contract MUST NOT define program semantics in terms of:

machine size
number of CPUs
number of GPUs
number of QPUs
number of nodes
memory size
thread count
register width
network size

Those are realization facts.

---

27.2 Arbitrarily large logical workloads

A logical workload MAY express:

n
problem_size
required_memory
required_qubits
required_parallelism
required_bandwidth

where those values are determined by the program or execution environment.

The grammar and compatibility model MUST NOT establish a universal upper bound.

---

28. Heterogeneous Targets

A target compatibility contract MAY describe heterogeneous environments.

For example:

CPU
+
GPU
+
FPGA
+
QPU
+
network

The compatibility resolver MUST treat this as a capability/resource environment rather than requiring a new language.

---

28.1 Heterogeneous compatibility

The resolver MUST be capable of determining:

which semantic requirement
        ↓
requires which capability
        ↓
using which resource class
        ↓
under which policy

Physical assignment remains downstream.

---

29. Distributed Targets

Distributed compatibility MUST distinguish:

distributed capability

from:

number of nodes

The source may require:

capability("distributed.messaging")
requires topology(required_topology)

without specifying a universal node limit.

Target feasibility determines whether the environment can satisfy the requirement.

---

30. Simulation Targets

Simulation is a legitimate target class.

The same target compatibility framework MUST support:

classical simulation
quantum simulation
HDL simulation
hardware simulation
distributed simulation
AI simulation
fault simulation

Simulation MUST NOT become a second language.

It is a target realization strategy.

---

31. Future Computational Domains

The target compatibility architecture MUST support future target classes without modifying the universal compatibility model.

Potential future targets include:

neuromorphic
photonic
molecular
biological
optical
memristive
analog
hybrid
distributed quantum
new accelerator architectures

These are examples, not a closed enumeration.

The compatibility architecture is based on:

identity
version
capabilities
resources
contracts
policies
IR compatibility
lowering

rather than a fixed list of machines.

---

32. Dialect Integration

Target compatibility MUST integrate with dialect compatibility.

The relationship is:

Dialect
   ↓
Dialect Version
   ↓
Dialect Compatibility
   ↓
Semantic Model
   ↓
IR
   ↓
Target Compatibility

A target MUST NOT become responsible for defining dialect semantics.

Likewise, a dialect MUST NOT silently assume a target-specific physical implementation.

---

32.1 Dialect target requirements

A dialect MAY declare:

required capability
required resource
required target class
required target compatibility contract

These become semantic requirements.

They MUST be validated independently.

---

32.2 Application-specific domains

Application-specific functionality MUST NOT force new target compatibility keywords.

For example, an application may require:

capability("vision.inference")
capability("robotics.control")
capability("distributed.ledger")

without creating permanent target grammar branches.

---

33. Compiler Integration

The compiler MUST perform target compatibility validation after semantic information is available.

The compiler flow is:

source
 ↓
parse
 ↓
AST
 ↓
semantic validation
 ↓
resource/effect/capability analysis
 ↓
canonical IR
 ↓
target compatibility
 ↓
lowering
 ↓
routing
 ↓
scheduling
 ↓
resilience
 ↓
HAL

Target compatibility MUST NOT be used to alter source parsing.

---

34. Runtime Integration

Runtime compatibility MUST remain separate.

A target can be:

target-compatible

while the selected runtime is:

runtime-incompatible

The diagnostic must identify the runtime problem.

The runtime MUST NOT silently reinterpret the source program to compensate for an incompatibility.

---

35. ABI Integration

ABI compatibility is a separate compatibility dimension.

For example:

semantic compatibility = true
IR compatibility = true
target compatibility = true
ABI compatibility = false

MUST be representable.

The correct result is:

ABI_INCOMPATIBLE

not:

TARGET_INCOMPATIBLE

unless the target compatibility contract explicitly includes the ABI dependency.

---

36. Security Integration

Target compatibility MUST be evaluated under security policies.

The resolver MUST account for:

sandbox
authorization
capability restrictions
native execution
foreign calls
network access
filesystem access
reflection
code generation
adaptation

Security rejection MUST remain distinguishable from technical target incompatibility.

---

37. Adaptive Execution

Adaptive execution MAY use target compatibility information.

For example:

detect
evaluate
select
retry
recover
fallback

may use target capability information.

However, adaptive execution MUST NOT silently alter program semantics.

Any adaptation that changes semantic behavior MUST be explicitly declared, authorized, and recorded according to the effect/policy/provenance system.

---

38. Target Compatibility and Resilience

Target compatibility may feed resilience planning.

The system may distinguish:

Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

from compatibility status.

These are runtime/target-state classifications, not version numbers.

For example:

target contract compatible
+
target state unavailable

is not a version incompatibility.

---

39. Target Compatibility and QEC

QEC remains downstream.

Target compatibility MAY determine whether a target supports required QEC-related capabilities.

For example:

capability("quantum.error-correction")

may be required.

But target compatibility MUST NOT implement QEC.

The flow remains:

quantum::ir
      ↓
target compatibility
      ↓
QEC planning
      ↓
physical realization

---

40. Target Compatibility and ZQN

ZQN remains downstream.

Target compatibility may establish whether the target can satisfy required noise/fault/resilience contracts.

It MUST NOT redefine source quantum semantics.

---

41. Diagnostics

Production implementations MUST use structured diagnostics.

Recommended categories include:

TARGET_COMPATIBILITY_UNKNOWN
TARGET_COMPATIBILITY_INVALID
TARGET_COMPATIBILITY_UNSUPPORTED
TARGET_COMPATIBILITY_INCOMPATIBLE

TARGET_VERSION_UNSUPPORTED

TARGET_CAPABILITY_UNAVAILABLE
TARGET_CAPABILITY_INCOMPATIBLE

TARGET_RESOURCE_INFEASIBLE
TARGET_RESOURCE_UNKNOWN

TARGET_POLICY_REJECTED

TARGET_IR_INCOMPATIBLE
TARGET_QUANTUM_IR_INCOMPATIBLE

TARGET_ABI_INCOMPATIBLE
TARGET_RUNTIME_INCOMPATIBLE

TARGET_MIGRATION_REQUIRED
TARGET_MIGRATION_UNAVAILABLE

TARGET_COMPATIBILITY_CONFLICT
TARGET_COMPATIBILITY_AMBIGUOUS

The final diagnostic taxonomy belongs to the repository diagnostics subsystem.

This document defines the semantic categories.

---

41.1 Diagnostic requirements

A target compatibility diagnostic SHOULD identify:

artifact
target
target version
target compatibility version
required contract
available contract
failed dimension
reason
evidence
migration suggestion where applicable
source location where applicable
provenance reference

Diagnostics MUST NOT merely say:

target incompatible

when the actual failure is known.

---

42. Safe Rust Implementation Contract

The reference implementation MUST use:

Rust 1.97 or later
Rust 2021
safe Rust

Rust "unsafe" MUST NOT be required.

---

42.1 Error handling

Compatibility resolution MUST be fallible.

Production code SHOULD use:

Result<T, E>

or the repository's established error model.

The resolver MUST NOT use panic-based control flow for malformed or untrusted compatibility metadata.

---

42.2 Checked version processing

Version comparison MUST avoid unchecked arithmetic.

Implementations MUST use:

- checked parsing;
- explicit validation;
- deterministic comparison;
- structured errors.

Version values MUST NOT overflow silently.

---

42.3 Arbitrary scale

The implementation MUST NOT encode language-level limits through fixed-size counters merely for convenience.

Where practical, large version components, dependency sets, capability sets, and resource expressions SHOULD be represented using scalable structures.

Any implementation limit that exists because of available memory, CPU time, parser infrastructure, or operating-system constraints MUST remain an implementation/resource limit rather than a language semantic limit.

---

42.4 Deterministic collections

Where ordering affects compatibility resolution, the implementation MUST use deterministic ordering.

Examples include:

stable sorting
ordered maps
ordered sets
canonical identifiers

Hash iteration order MUST NOT determine compatibility results.

---

42.5 No recursive dependency assumptions

Dependency resolution MUST NOT rely on a fixed maximum recursion depth.

For potentially large dependency graphs, iterative traversal or an equivalent scalable strategy SHOULD be used.

---

43. Target Compatibility Manifest

A target compatibility manifest SHOULD conceptually contain:

identity
target_version
target_compatibility_version

language_compatibility
grammar_compatibility
ast_compatibility
semantic_compatibility

classical_ir_compatibility
quantum_ir_compatibility

dialect_compatibility

capabilities
resources
effects
policies

abi_compatibility
runtime_compatibility

migration_support
deprecation_state

provenance
integrity

The exact serialization format belongs to the repository's machine-contract layer.

---

43.1 Manifest integrity

Where manifests are externally supplied, the implementation SHOULD support provenance/integrity validation.

A target compatibility decision MUST NOT trust unvalidated metadata when security policy requires authenticated metadata.

---

44. Compatibility Resolution Algorithm

A production resolver SHOULD follow this deterministic sequence.

Step 1 — Identify artifact

Determine:

language contract
semantic contract
AST contract where applicable
IR contracts
dialects
requirements
capabilities
policies

Step 2 — Identify target

Determine:

target identity
target version
target compatibility version

Step 3 — Validate target compatibility version

Reject:

malformed
unknown
unsupported
ambiguous

versions according to the repository's compatibility policy.

Step 4 — Validate language compatibility

Consume the language-version contract.

Step 5 — Validate semantic compatibility

Consume the semantic-version contract.

Step 6 — Validate AST compatibility

Where the artifact depends on an AST contract, validate it.

Step 7 — Validate IR compatibility

Validate:

Classical IR
quantum::ir
other domain IR contracts

where applicable.

Step 8 — Validate dialect compatibility

Resolve required dialects and versions.

Step 9 — Validate capabilities

Evaluate required capabilities.

Step 10 — Validate resources

Evaluate resource requirements.

Step 11 — Validate effects

Ensure required effects are permitted and supported.

Step 12 — Validate policies

Apply security, execution, deployment, and resource policies.

Step 13 — Validate ABI/runtime

Evaluate applicable ABI and runtime contracts.

Step 14 — Determine migration

If migration is required, determine whether a semantics-preserving migration exists.

Step 15 — Produce result

Return the most specific compatibility classification.

Step 16 — Record provenance

Record the evidence used for the decision.

Step 17 — Permit downstream realization

Only after compatibility succeeds or an explicitly permitted degraded mode is selected may lowering, routing, scheduling, and target realization proceed.

---

45. Compatibility Result Precedence

When multiple failures exist, diagnostics SHOULD preserve all relevant failures while providing a deterministic primary classification.

A recommended conceptual priority is:

INVALID_METADATA
        ↓
UNKNOWN_COMPATIBILITY
        ↓
INCOMPATIBLE_CONTRACT
        ↓
MIGRATION_REQUIRED
        ↓
POLICY_REJECTED
        ↓
CAPABILITY_UNAVAILABLE
        ↓
RESOURCE_INFEASIBLE
        ↓
ABI/RUNTIME_INCOMPATIBLE
        ↓
COMPATIBLE

This ordering is diagnostic guidance, not a replacement for reporting all applicable failures.

---

46. Compatibility with Partial Targets

A target MAY be partially compatible.

For example:

Classical IR        compatible
Quantum IR          compatible
Measurement         available
Dynamic circuits    unavailable

The result MUST distinguish:

partially compatible

from:

fully compatible

The compiler may continue only when the missing capability is not required or an explicitly authorized fallback exists.

---

47. Fallbacks

Fallbacks MUST be explicit.

Examples:

GPU unavailable
    ↓
CPU fallback

or:

QPU unavailable
    ↓
quantum simulator

A fallback MUST NOT silently alter semantic meaning.

If the fallback changes a guarantee, precision, timing property, security property, or other declared contract, the compiler MUST diagnose the change.

---

48. Preferences

Target preferences are not requirements.

For example:

prefer capability("gpu.compute")

does not mean:

require capability("gpu.compute")

The resolver MAY select another compatible realization.

Preferences MUST NOT be promoted to requirements merely because a preferred target is unavailable.

---

49. Constraints

Constraints are mandatory.

For example:

constrain topology(required_topology)

cannot be discarded merely because another topology is available.

A target failing a mandatory constraint MUST be rejected for that realization.

---

50. Target Compatibility and Optimization

Optimization MAY select different implementations when semantic equivalence is preserved.

For example:

CPU implementation
GPU implementation
FPGA implementation
QPU implementation
simulator implementation

may differ internally.

Target compatibility does not require identical machine instructions.

It requires semantic compatibility with the program's contract.

---

51. Target Compatibility and Performance

Performance is not automatically semantic compatibility.

A target may be:

semantically compatible

but:

performance requirement infeasible

if performance is explicitly declared as a mandatory contract.

Performance preferences remain preferences.

---

52. Target Compatibility and Determinism

If a program requires deterministic/reproducible execution, target compatibility MUST include the corresponding determinism requirements.

A target with nondeterministic behavior MUST NOT be treated as fully compatible with a strict deterministic contract unless an explicit mechanism restores the required guarantee.

---

53. Target Compatibility and Randomness

Randomness MUST remain an explicit effect where applicable.

Target compatibility MUST distinguish:

randomness permitted

from:

cryptographically secure randomness required

from:

deterministic replay required

These are different contracts.

---

54. Target Compatibility and Adaptation

Controlled adaptation may select among compatible realizations.

For example:

target A unavailable
        ↓
target B compatible
        ↓
adapt execution plan

The adaptation itself MUST be governed by:

policy
capability
effects
authorization
resource constraints
provenance

Adaptation MUST NOT become unrestricted self-modification.

---

55. Target Compatibility and Explainability

Compatibility decisions SHOULD be explainable.

A compiler SHOULD be able to report:

why target was selected
why another target was rejected
which capability was missing
which resource was insufficient
which policy rejected execution
which migration was applied
which IR contract was required

This integrates with the repository's evidence/provenance/decision-record model.

---

56. Target Compatibility and Evidence

Where compatibility depends on evidence, the resolver MAY preserve:

claim
evidence
source
verification
confidence
provenance

For example:

claim:
    target supports capability X

evidence:
    signed target manifest

verified_by:
    target capability validator

The compatibility decision SHOULD remain traceable.

---

57. Compatibility Testing

Every target compatibility implementation MUST have tests covering:

positive
negative
boundary
migration
version negotiation
unknown versions
dependency conflicts
capability failures
resource failures
policy failures
IR compatibility
quantum::ir compatibility
ABI compatibility
runtime compatibility
determinism
reproducibility
scalability
cross-domain integration

---

57.1 Positive tests

At minimum:

compatible target
compatible target contract
compatible Classical IR
compatible quantum::ir
compatible capabilities
sufficient resources
compatible policy

---

57.2 Negative tests

Must include:

unknown target compatibility version
unsupported target compatibility version
incompatible target contract
missing capability
insufficient resource
policy rejection
IR mismatch
quantum::ir mismatch
ABI mismatch
runtime mismatch
ambiguous target
invalid manifest
invalid migration

---

57.3 Boundary tests

Boundary tests MUST include:

minimum supported compatibility contract
maximum representable implementation value
large version components
large dependency graph
large capability set
large resource expression
many target alternatives
many dialect dependencies
deeply composed target contracts

The language MUST NOT define artificial semantic ceilings for these cases.

---

57.4 Scalability tests

Scalability tests MUST verify that the architecture does not introduce limits based on:

number of targets
number of capabilities
number of resources
number of dialects
number of target alternatives
number of nodes
number of CPUs
number of GPUs
number of QPUs
number of qubits
memory capacity
topology size

Tests SHOULD use generated or parameterized workloads.

---

57.5 Determinism tests

For identical compatibility inputs:

resolve(A) == resolve(A)

MUST hold.

The resolver MUST NOT produce different target compatibility results because of:

thread scheduling
hash ordering
filesystem ordering
network ordering
plugin discovery ordering

---

58. Cross-Domain Tests

The repository MUST test combinations including:

Classical + target
Quantum + target
HDL + target
AI + target
Data + target
Distributed + target
Networking + target
Hybrid + target
Classical + Quantum + target
Quantum + HDL + target
Quantum + Distributed + target
AI + Quantum + target
AI + Hardware + target
Classical + Quantum + HDL + Hardware + target

---

59. Mandatory POCO-REAF Integration Test

The repository SHOULD maintain a canonical integration test in which one source program expresses:

classical computation
+
tensor computation
+
quantum operation
+
measurement
+
AI/reasoning operation
+
parallelism
+
resource requirements
+
capability requirements
+
effects
+
contracts
+
policy
+
provenance
+
simulation/fallback

The test MUST verify:

Source
  ↓
AST
  ↓
Semantic Model
  ↓
Capability Analysis
  ↓
Resource Analysis
  ↓
Policy Analysis
  ↓
Classical IR
  ↓
quantum::ir
  ↓
Target Compatibility
  ↓
Execution Plan

The source MUST remain target-independent.

---

60. File Independence Contract

This file is complete independently when its relationships to every relevant subsystem are known.

The compatibility contract is:

DEPENDS_ON:
    grammar/DESIGN.md
    grammar/spec/versioning.md
    grammar/spec/compatibility.md
    grammar/compatibility/versions.md
    grammar/compatibility/compatibility-matrix.md
    grammar/compatibility/target-compatibility.md
    grammar/compatibility/migrations.md
    grammar/compatibility/deprecated.md
    grammar/compatibility/feature-gates.md
    grammar/hardware/targets.g4
    grammar/resources/
    grammar/hardware/
    canonical IR contracts
    quantum::ir contract

EXPORTS:
    target compatibility-version semantics
    target compatibility result model
    target compatibility negotiation rules
    migration/version rules
    provenance requirements
    deterministic resolution requirements

CONSUMED_BY:
    compatibility resolver
    compiler
    target planner
    lowering
    runtime integration
    HAL integration
    compatibility tests
    tooling

AST_OWNER:
    frontend AST subsystem

SEMANTIC_OWNER:
    target compatibility semantic layer

IR_OWNER:
    Classical IR / quantum::ir / domain IR owners

TEST_OWNER:
    grammar/tests/compatibility/
    target compatibility test suite

SPEC_OWNER:
    grammar/specification/
    grammar/spec/

---

61. Integration with "grammar/compatibility/dialect-version.md"

Dialect version and target compatibility version MUST remain independent.

The relationship is:

Dialect Version
      ↓
Dialect Compatibility
      ↓
Semantic Model
      ↓
IR
      ↓
Target Compatibility

A dialect version MUST NOT encode a target version.

A target compatibility version MUST NOT encode a dialect version.

---

62. Integration with "grammar/compatibility/semantic-version.md"

Semantic compatibility MUST be resolved before target compatibility.

Required relationship:

Semantic Contract
      ↓
Target Compatibility

A target MUST NOT claim compatibility with an artifact whose semantic contract it cannot preserve.

---

63. Integration with "grammar/compatibility/ir-version.md"

IR compatibility is a prerequisite for target lowering.

Required relationship:

Semantic Model
      ↓
IR Contract
      ↓
Target Compatibility
      ↓
Lowering

The target compatibility document does not redefine IR versions.

It consumes them.

---

64. Integration with "grammar/compatibility/AST-version.md"

AST compatibility is relevant where serialized or externally persisted AST artifacts are used.

The target compatibility system MUST NOT make physical target information part of the portable AST merely to simplify compatibility.

---

65. Integration with "grammar/compatibility/grammar-version.md"

Grammar compatibility is a frontend concern.

Target compatibility begins after the program's syntax has been interpreted into semantic information.

A target MUST NOT influence source parsing.

---

66. Integration with "grammar/compatibility/compatibility-matrix.md"

The compatibility matrix SHOULD record relationships such as:

Language 1.x
Semantic 2.x
Classical IR 1.x
quantum::ir 2.x
Dialect X 3.x
Target Compatibility 4.x

This document defines what the target compatibility version means.

The matrix records how it relates to other versions.

---

67. Integration with "grammar/specification/"

Normative target semantics belong in the specification hierarchy.

This document defines compatibility versioning.

It MUST NOT become a substitute for the normative target semantics.

---

68. Integration with "grammar/spec/"

Machine-readable contracts SHOULD be able to represent:

target identity
target version
target compatibility version
capabilities
resources
IR compatibility
dialect compatibility
policies
migration
provenance

The machine contract MUST remain consistent with this document.

---

69. Integration with "grammar/hardware/targets.g4"

The target grammar remains responsible for declarations such as target intent.

It MUST NOT be expanded merely to implement compatibility-version resolution.

The compatibility layer consumes the semantic result.

---

70. Integration with "grammar/resources/"

Resource requirements flow into target compatibility as:

resource requirement
        ↓
target resource availability
        ↓
feasibility

Resource scarcity is not language incompatibility.

---

71. Integration with "grammar/hardware/capabilities.g4"

Capability declarations and references remain owned by the hardware/capability subsystem.

Target compatibility evaluates them.

---

72. Integration with "grammar/effects/"

Effects remain semantically explicit.

Target compatibility determines whether the target environment can legally support required effects under the applicable policies.

---

73. Integration with "grammar/policies/"

Policies may:

allow
forbid
prefer
constrain
require
fallback

Target compatibility MUST consume these decisions.

---

74. Integration with "grammar/security/"

Security constraints MUST be enforced before target realization.

Examples include:

sandbox
authorization
native execution
FFI
network
filesystem
reflection
code generation
adaptation

---

75. Integration with "grammar/execution/"

Execution policies may use target compatibility to select:

execute
simulate
retry
recover
fallback
adapt

Target compatibility does not own those execution semantics.

---

76. Integration with routing and scheduling

Target compatibility answers:

«Can this target contract support the required semantics and capabilities?»

Routing answers:

«How can logical work be mapped onto the target topology?»

Scheduling answers:

«When and in what execution order should operations run?»

These MUST remain separate.

---

77. Integration with resilience

Target compatibility may identify compatible resilience capabilities.

Actual resilience remains downstream.

The resolver MUST NOT confuse:

target incompatible

with:

target temporarily unavailable

or:

target degraded

---

78. Integration with HAL

HAL is the physical realization boundary.

The flow is:

Target Compatibility
       ↓
Realization Plan
       ↓
HAL
       ↓
Physical/Runtime Target

HAL MUST NOT silently change compatibility semantics.

---

79. Compatibility of Target Discovery

Target discovery MAY produce:

target identity
target version
target compatibility version
capabilities
resources
topology
policy metadata

Discovery MUST NOT modify source semantics.

Target discovery MUST be treated as an input to compatibility resolution.

---

80. Target Snapshot

For reproducible compilation, a target snapshot SHOULD contain:

target identity
target version
target compatibility version
capabilities
resources
topology
policies
runtime contract
ABI contract

The snapshot SHOULD be immutable for the compatibility decision.

---

81. Target Compatibility Locking

A build system MAY lock:

target compatibility contract
capability snapshot
resource assumptions
dialect versions
IR versions
migration choices

This allows deterministic builds without requiring the source to name a physical machine.

---

82. Physical Target Independence

A compatibility lock SHOULD NOT require:

serial number
PCI address
MAC address
physical qubit ID
physical CPU ID
GPU UUID
machine hostname

unless explicitly required by a deployment policy.

Such values belong to deployment/operations rather than portable language semantics.

---

83. Target Compatibility and Deployment

Deployment may bind a portable artifact to a physical environment.

That binding is downstream:

portable artifact
      ↓
target compatibility
      ↓
deployment policy
      ↓
physical environment

The deployment environment MUST NOT redefine the artifact's language semantics.

---

84. Compatibility and Recompilation

A portable program may be recompiled for a different target.

The target compatibility system MUST determine whether:

same source semantics

can be realized on the new target.

Recompilation MUST NOT require source changes merely because the target changed.

---

85. Compatibility and Optimization Profiles

Optimization profiles MAY affect:

performance
latency
energy
memory
parallelism

They MUST NOT silently change target compatibility semantics.

A profile that changes a mandatory semantic guarantee is not merely an optimization profile.

---

86. Compatibility and Precision

Targets may support different numeric or computational precision.

The compatibility system MUST distinguish:

exact semantics
required precision
permitted approximation
preference for precision

If a program requires a precision guarantee that a target cannot satisfy, the result is a compatibility/feasibility failure.

Silent precision reduction is prohibited when precision is part of the semantic contract.

---

87. Compatibility and Numerical Stability

Where numerical stability is explicitly specified, target compatibility MAY evaluate the target against that requirement.

The compiler MUST NOT assume that a faster target is automatically compatible.

---

88. Compatibility and Energy

Energy requirements MAY be expressed as target constraints or preferences.

The system MUST distinguish:

energy required

from:

energy preferred

and:

energy observed

Observed runtime energy is not a language version.

---

89. Compatibility and Thermal Constraints

Thermal constraints are target/resource information.

They MUST NOT become fixed grammar limits.

A target may reject an execution plan because of thermal policy while remaining language-compatible.

---

90. Compatibility and Network Constraints

Networking may participate in target compatibility through:

network capability
bandwidth
latency
topology
security
protocol compatibility

The target compatibility system MUST distinguish network capability failure from language failure.

---

91. Compatibility and Data Interoperability

Data formats such as:

JSON
XML
SQL

may participate in interoperability contracts.

Their compatibility MUST remain separate from target compatibility unless the target contract explicitly requires a particular data interface.

---

92. Compatibility and FFI

FFI requires:

effect("foreign")
capability("native.ffi")
ABI compatibility
calling convention
data layout
runtime support
policy authorization

Target compatibility MUST consume these contracts without redefining the FFI language model.

---

93. Compatibility and Reflection

Reflection and metaprogramming MAY depend on target capabilities.

For example:

capability("reflection.runtime")

may be required.

The absence of that capability is not a language incompatibility.

---

94. Compatibility and Learning

Learning may require:

learning effect
data resources
compute capability
randomness
adaptation
provenance
policy

Target compatibility evaluates those requirements.

It MUST NOT hard-code a particular learning implementation.

---

95. Compatibility and Adaptation

Adaptation MUST be explicitly controlled.

The target compatibility system may supply information to an adaptation planner, but the adaptation planner remains responsible for:

authorization
policy
effect checking
provenance
semantic validation

---

96. Compatibility and Provenance

Every compatibility transformation SHOULD be attributable to:

resolver
contract
version
target
evidence
migration
policy

This allows developers to answer:

«Why was this target accepted?»

and:

«Why was this target rejected?»

---

97. Compatibility and Explainability

The compatibility subsystem SHOULD provide a structured explanation graph:

Program Requirement
        ↓
Required Capability
        ↓
Target Capability
        ↓
Resource Requirement
        ↓
Target Resource
        ↓
Policy
        ↓
Compatibility Result

This explanation is especially important for complex heterogeneous systems.

---

98. Compatibility Security

Untrusted target metadata MUST NOT be allowed to:

- execute code;
- modify source semantics;
- bypass policies;
- override capability checks;
- override resource checks;
- select privileged execution;
- bypass sandbox restrictions.

Target metadata is data.

It is not executable Zamani source.

---

99. Compatibility Metadata Validation

External compatibility metadata MUST be validated before use.

Validation SHOULD include:

syntax
identity
version
schema
references
capabilities
resources
policies
migration
integrity
provenance

Malformed metadata MUST fail closed.

---

100. No Hidden Target Semantics

The compiler MUST NOT infer semantic meaning from:

vendor name
device name
machine size
hardware generation
filesystem path
environment variable
hostname

unless that information is explicitly represented through the target contract.

---

101. No Vendor Lock-In

Vendor-specific target contracts MAY exist.

They MUST be represented as explicit target/dialect/capability contracts.

They MUST NOT become hidden core-language assumptions.

The source language remains target-neutral.

---

102. Compatibility with Future Hardware

When a future target is introduced, the preferred integration path is:

new target identity
        ↓
target contract
        ↓
target compatibility version
        ↓
capability declarations
        ↓
resource declarations
        ↓
lowering
        ↓
HAL integration

The core language SHOULD NOT require a new keyword merely because a new machine class appears.

---

103. Repository Completion Requirements

This file is considered complete only when:

- [ ] target compatibility version is clearly separated from target version;
- [ ] target compatibility version is separated from language version;
- [ ] target compatibility version is separated from grammar version;
- [ ] target compatibility version is separated from AST version;
- [ ] target compatibility version is separated from semantic version;
- [ ] target compatibility version is separated from IR version;
- [ ] target compatibility version is separated from dialect version;
- [ ] target compatibility version is separated from ABI version;
- [ ] target compatibility version is separated from runtime version;
- [ ] target compatibility version is separated from compiler version;
- [ ] target compatibility version is separated from physical hardware generation;
- [ ] unknown versions fail closed;
- [ ] compatibility is multidimensional;
- [ ] target compatibility is distinct from capability availability;
- [ ] target compatibility is distinct from resource feasibility;
- [ ] target compatibility is distinct from policy rejection;
- [ ] target compatibility is distinct from runtime failure;
- [ ] target compatibility is distinct from ABI failure;
- [ ] target compatibility consumes resource requirements;
- [ ] target compatibility consumes capability requirements;
- [ ] target compatibility consumes policies;
- [ ] target compatibility consumes effects;
- [ ] target compatibility integrates with Classical IR;
- [ ] target compatibility integrates with "quantum::ir";
- [ ] target compatibility integrates with HDL;
- [ ] target compatibility integrates with hardware targets;
- [ ] target compatibility integrates with dialects;
- [ ] target compatibility integrates with routing;
- [ ] target compatibility integrates with scheduling;
- [ ] target compatibility integrates with resilience;
- [ ] target compatibility integrates with QEC;
- [ ] target compatibility integrates with ZQN;
- [ ] target compatibility integrates with HAL;
- [ ] migration is explicit;
- [ ] deprecation is explicit;
- [ ] provenance is defined;
- [ ] deterministic resolution is defined;
- [ ] reproducibility is defined;
- [ ] scalability is defined;
- [ ] no universal hardware capacity is hard-coded;
- [ ] no fixed target universe is hard-coded;
- [ ] no physical device identity is required for portable semantics;
- [ ] safe Rust is sufficient;
- [ ] Rust 1.97+ is supported;
- [ ] "unsafe" is not required;
- [ ] positive tests are defined;
- [ ] negative tests are defined;
- [ ] boundary tests are defined;
- [ ] scalability tests are defined;
- [ ] determinism tests are defined;
- [ ] cross-domain tests are defined;
- [ ] POCO-REAF tests are defined.

---

104. Production Readiness Checklist

Before declaring the target compatibility subsystem production-ready:

Authority

- [ ] "grammar/DESIGN.md" agrees.
- [ ] "grammar/specification/" agrees.
- [ ] "grammar/spec/" agrees.
- [ ] no competing authority exists.

Versioning

- [ ] target compatibility version has an independent identity;
- [ ] version comparison is deterministic;
- [ ] version ranges are explicit;
- [ ] unknown versions fail closed;
- [ ] migrations are explicit;
- [ ] deprecations are explicit.

Frontend

- [ ] target intent comes from "grammar/hardware/targets.g4";
- [ ] target compatibility does not modify parsing;
- [ ] AST remains portable.

Semantics

- [ ] semantic compatibility is checked before realization;
- [ ] effects are checked;
- [ ] capabilities are checked;
- [ ] resources are checked;
- [ ] contracts are preserved;
- [ ] policies are checked.

IR

- [ ] Classical IR compatibility is validated;
- [ ] "quantum::ir" compatibility is validated;
- [ ] no competing quantum IR exists.

Target

- [ ] target identity is symbolic;
- [ ] target version is independent;
- [ ] target compatibility version is independent;
- [ ] capabilities are open-ended;
- [ ] resources are open-ended;
- [ ] topology is target metadata;
- [ ] physical device identity is downstream.

Runtime

- [ ] ABI compatibility is separate;
- [ ] runtime compatibility is separate;
- [ ] HAL remains the realization boundary.

Scalability

- [ ] no universal machine-size constants exist;
- [ ] no universal quantum-size constants exist;
- [ ] no universal node limits exist;
- [ ] no universal memory limits exist;
- [ ] no universal GPU/FPGA/QPU limits exist;
- [ ] no target enumeration is closed.

Safety

- [ ] malformed metadata produces structured errors;
- [ ] resolver does not panic on untrusted input;
- [ ] version arithmetic is checked;
- [ ] resolution is deterministic;
- [ ] safe Rust only;
- [ ] no "unsafe".

Verification

- [ ] positive tests pass;
- [ ] negative tests pass;
- [ ] boundary tests pass;
- [ ] scalability tests pass;
- [ ] deterministic tests pass;
- [ ] reproducibility tests pass;
- [ ] cross-domain tests pass;
- [ ] quantum tests pass;
- [ ] HDL tests pass;
- [ ] distributed tests pass;
- [ ] heterogeneous-target tests pass;
- [ ] POCO-REAF integration tests pass.

---

105. Permanent Architecture Invariants

The following invariants are mandatory.

Invariant 1 — One language

Zamani remains one language.

Invariant 2 — Target compatibility is downstream

Target compatibility MUST NOT define source-language semantics.

Invariant 3 — Target version is independent

Target version MUST NOT be reused as target compatibility version.

Invariant 4 — Capability is not resource

A capability and a resource are different semantic concepts.

Invariant 5 — Resource failure is not language failure

Insufficient target resources MUST NOT invalidate the source language.

Invariant 6 — Capability failure is not version failure

Missing capability MUST NOT be misreported as compatibility-version failure.

Invariant 7 — Unknown compatibility fails closed

The compiler MUST NOT guess unknown compatibility semantics.

Invariant 8 — No physical limits in language semantics

Hardware capacity MUST NOT become a permanent grammar limit.

Invariant 9 — Open target universe

Future target classes MUST be introducible without redesigning the compatibility model.

Invariant 10 — Canonical quantum boundary

"quantum::ir" remains the canonical quantum IR boundary.

Invariant 11 — Routing remains downstream

Target compatibility does not perform physical routing.

Invariant 12 — Scheduling remains downstream

Target compatibility does not define the physical execution schedule.

Invariant 13 — QEC remains downstream

Target compatibility may check QEC capabilities but does not implement QEC.

Invariant 14 — ZQN remains downstream

Target compatibility does not replace ZQN.

Invariant 15 — HAL remains the realization boundary

Physical target realization remains a HAL/backend concern.

Invariant 16 — Deterministic resolution

Equivalent inputs MUST produce equivalent compatibility decisions.

Invariant 17 — Provenance

Compatibility decisions SHOULD remain explainable and traceable.

Invariant 18 — Safe Rust

The implementation MUST use Rust 1.97+ and MUST NOT require "unsafe".

---

106. Final POCO-REAF Target Compatibility Model

The complete production model is:

                         Zamani Source
                               │
                               ▼
                       Language Version
                               │
                               ▼
                    Canonical Lexer / Parser
                               │
                               ▼
                    Domain-Neutral AST
                               │
                               ▼
                       Semantic Analysis
                               │
          ┌────────────────────┼────────────────────┐
          │                    │                    │
          ▼                    ▼                    ▼
        Types              Effects              Contracts
          │                    │                    │
          └────────────────────┼────────────────────┘
                               │
                               ▼
                      Capabilities / Resources
                               │
                               ▼
                       Portable Semantic Model
                               │
                 ┌─────────────┴─────────────┐
                 │                           │
                 ▼                           ▼
            Classical IR                 quantum::ir
                 │                           │
                 └─────────────┬─────────────┘
                               │
                               ▼
                    Target Compatibility
                               │
          ┌────────────────────┼────────────────────┐
          │                    │                    │
          ▼                    ▼                    ▼
       Target              Capabilities          Resources
       Contract                                  / Topology
          │                    │                    │
          └────────────────────┼────────────────────┘
                               │
                               ▼
                            Policies
                               │
                               ▼
                      Compatibility Result
                               │
                  ┌────────────┴────────────┐
                  │                         │
                  ▼                         ▼
             Compatible                Incompatible
                  │
                  ▼
                 Lowering
                  │
          ┌───────┼────────┐
          ▼       ▼        ▼
       Routing Scheduling Resilience
                          │
                    ┌─────┴─────┐
                    ▼           ▼
                   QEC         ZQN
                    │           │
                    └─────┬─────┘
                          ▼
                         HAL
                          │
                          ▼
                   Target Realization
                          │
       ┌──────────┬───────┼────────┬───────────┐
       ▼          ▼       ▼        ▼           ▼
      CPU        GPU     FPGA     ASIC        QPU
       │          │       │        │           │
       └──────────┴───────┴────────┴───────────┘
                          │
                          ▼
                 Simulator / HPC /
                 Cluster / Distributed /
                 Cloud / Future Target

The essential rule is:

SOURCE
  ↓
MEANING
  ↓
REQUIREMENTS
  +
CAPABILITIES
  +
RESOURCES
  +
CONSTRAINTS
  +
POLICIES
  ↓
CANONICAL IR
  ↓
TARGET COMPATIBILITY
  ↓
REALIZATION

Not:

SOURCE
  ↓
CURRENT MACHINE
  ↓
LANGUAGE MEANING

---

107. Final Production Principle

The target compatibility system exists to preserve one semantic program across changing execution environments.

A Zamani program may be considered for:

tiny embedded hardware
single CPU
multicore CPU
manycore system
GPU
FPGA
ASIC
specialized accelerator
QPU
quantum simulator
HPC
cluster
distributed environment
cloud
heterogeneous system
future computational substrate

without making physical capacity part of the language definition.

The target compatibility contract answers:

«Can this target environment legally and semantically realize this Zamani artifact under the declared contracts?»

It does not answer:

«What does the Zamani program mean?»

That meaning belongs to the language and semantic layers.

It does not answer:

«How are physical resources mapped?»

That belongs to lowering, routing, scheduling, resilience, QEC, ZQN, and HAL.

It does not answer:

«Which exact physical device should execute it?»

That belongs to deployment and target realization.

Therefore the permanent POCO-REAF boundary is:

Program Once
      ↓
Preserve Semantic Meaning
      ↓
Compile Once to Portable Representation
      ↓
Resolve Compatibility
      ↓
Negotiate Capabilities and Resources
      ↓
Lower for the Available Target
      ↓
Route / Schedule / Recover
      ↓
Realize Through HAL
      ↓
Run Everywhere
      ↓
Anywhere
      ↓
Forever

The target may be tiny.

The target may be enormous.

The target may be classical.

The target may be quantum.

The target may be HDL-oriented.

The target may be heterogeneous.

The target may be distributed.

The target may not exist yet when the language is designed.

None of those facts should require changing the fundamental meaning of the Zamani program.

That is the production target-compatibility contract required for POCO-REAF.