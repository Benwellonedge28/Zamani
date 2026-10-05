Zamani Resource Grammar Subsystem

Path: "grammar/resources/"
Language: Zamani
Grammar technology: ANTLR4
Compiler baseline: Rust 1.97 / Rust 1.97.1
Rust edition: Rust 2021
Safety: Safe Rust only; "unsafe" is prohibited
Primary architecture: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Status: Normative directory-level architecture and integration contract

---

1. Purpose

The "grammar/resources/" directory defines the source-language resource-intent grammar subsystem of Zamani.

Its purpose is to allow a Zamani program to express:

- resource requirements;
- capabilities;
- constraints;
- budgets;
- preferences;
- hints;
- negotiation intent;
- scalability intent;
- resource references;
- resource properties;
- resource quantities;
- resource relationships;
- target intent;
- resource lifecycle intent;
- resource profiles;
- resource groups;
- resource contracts;
- resource metadata;
- resource provenance;
- resource policies where appropriate.

The subsystem is deliberately target-independent.

A Zamani program describes what it requires, permits, prefers, or expects.

It does not directly prescribe:

- a particular CPU;
- a particular GPU;
- a particular FPGA;
- a particular ASIC;
- a particular QPU;
- a particular memory bank;
- a particular node;
- a particular cloud instance;
- a particular physical qubit;
- a particular network device;
- a particular hardware topology;
- a particular vendor implementation.

Those decisions belong to semantic analysis, capability resolution, resource planning, compilation, lowering, routing, scheduling, deployment, runtime, and HAL layers.

The central rule is:

«Resource intent is not resource realization.»

---

2. Architectural Position

The resource subsystem participates in the complete Zamani pipeline:

Zamani source
    |
    v
Canonical lexer
    |
    v
ANTLR parser
    |
    v
Domain-neutral AST
    |
    v
Structural validation
    |
    v
Name resolution
    |
    v
Type analysis
    |
    v
Effect analysis
    |
    v
Capability analysis
    |
    v
Resource analysis
    |
    v
Contract analysis
    |
    v
Policy analysis
    |
    v
Provenance
    |
    v
Canonical semantic model
    |
    +------------------+------------------+
    |                  |                  |
    v                  v                  v
Classical         quantum::ir        HDL/hardware
    |                  |                  |
    +------------------+------------------+
                       |
                       v
                  Optimization
                       |
                       v
                    Lowering
                       |
              +--------+--------+
              |        |        |
              v        v        v
           Routing  Scheduling Resilience
              |        |        |
              +--------+--------+
                       |
                       v
                      ZQN
                       |
                       v
                      HAL
                       |
                       v
                Target realization

The resource subsystem is therefore an intent and constraint boundary, not an execution engine.

---

3. Directory Contract

The directory is organized around these semantic layers:

grammar/resources/
│
├── README.md
│
├── resources.g4
├── resource-expressions.g4
│
├── requirements.g4
├── capabilities.g4
├── constraints.g4
├── budgets.g4
├── preferences.g4
├── hints.g4
├── negotiation.g4
└── scalability.g4

Additional files or subdirectories may be introduced when a resource concept becomes sufficiently independent to justify its own ownership.

A new file must not be created merely to duplicate an existing resource abstraction.

The ownership rule is:

resources.g4
    |
    +-- composition/orchestration
    |
    +-- declarations
    |
    +-- resource body dispatch
    |
    +-- lifecycle dispatch
    |
    +-- open-world composition
    |
    +-- integration boundary

while:

resource-expressions.g4
    |
    +-- resource expressions
    +-- resource references
    +-- resource selectors
    +-- resource values
    +-- resource ranges
    +-- resource quantities
    +-- generic resource expression helpers

and:

requirements.g4   -> mandatory intent
capabilities.g4   -> capability intent
constraints.g4    -> mandatory realization constraints
budgets.g4        -> bounded resource/cost intent
preferences.g4    -> advisory optimization intent
hints.g4          -> weak advisory information
negotiation.g4    -> resolution/selection negotiation
scalability.g4    -> scaling intent

---

4. Authority Hierarchy

The following authority hierarchy is mandatory.

4.1 Global architecture

grammar/DESIGN.md

Owns:

- overall language architecture;
- AST/semantic/IR separation;
- POCO-REAF;
- target independence;
- domain boundaries;
- extensibility;
- scalability principles.

---

4.2 Normative specification

grammar/specification/

Owns normative language semantics.

---

4.3 Resource specification

grammar/spec/resources.md

Owns the normative resource semantic model.

---

4.4 Resource grammar composition

grammar/resources/resources.g4

Owns resource grammar composition.

It is the resource grammar orchestrator.

It must not become another monolithic implementation of every resource concept.

---

4.5 Resource components

grammar/resources/*.g4

Each component owns one well-defined resource concept.

---

4.6 Global grammar composition

grammar/Zamani.g4
grammar/antlr/ZamaniParser.g4

These integrate the resource subsystem into the complete language.

---

4.7 Conformance

grammar/grammar.md

Records actual implementation/conformance status.

It is not an alternative grammar authority.

---

4.8 Historical and extended material

grammar/Zamani-Grammar.md

May describe proposed, experimental, historical, deprecated, or extended concepts.

It cannot silently make syntax normative.

---

5. Ownership Contract

5.1 This directory owns

The resource subsystem owns source-level resource intent.

This includes:

Resource identity

- symbolic resource names;
- resource references;
- resource kinds;
- qualified resource kinds;
- resource selectors;
- resource groups.

Resource requirements

- required capabilities;
- required quantities;
- mandatory conditions;
- mandatory resource properties.

Resource constraints

- realization constraints;
- topology constraints;
- performance constraints;
- timing constraints;
- energy constraints;
- reliability constraints;
- placement constraints.

Capabilities

- capability requirements;
- capability assertions;
- capability expressions;
- capability predicates;
- capability metadata.

Preferences

- preferred capabilities;
- preferred targets;
- preferred properties;
- optimization objectives;
- ordering preferences.

Hints

- locality hints;
- optimization hints;
- placement hints;
- implementation hints;
- advisory metadata.

Budgets

- resource budgets;
- time budgets;
- energy budgets;
- cost budgets;
- capacity budgets;
- execution budgets.

Negotiation

- alternative realization strategies;
- fallback intent;
- negotiation strategies;
- negotiation policies;
- negotiation phases.

Scalability

- symbolic scale;
- workload-dependent scale;
- elasticity intent;
- scaling dimensions;
- scaling policies;
- symbolic bounds.

Resource lifecycle intent

- reserve;
- acquire;
- allocate;
- bind;
- attach;
- release;
- deallocate;
- detach;
- migrate;
- recover;
- reconfigure.

The exact ownership of individual lifecycle rules remains defined by "resources.g4".

---

6. Does Not Own

The resource subsystem does not own:

- lexer definitions;
- token spelling;
- Unicode classification;
- identifiers;
- numeric literal implementation;
- general expression precedence;
- general arithmetic;
- general logical operators;
- general type semantics;
- general effects;
- module resolution;
- symbol resolution;
- physical resource discovery;
- physical allocation;
- hardware discovery;
- hardware inventory;
- scheduling algorithms;
- routing algorithms;
- optimization algorithms;
- QEC algorithms;
- calibration;
- physical qubit mapping;
- classical IR;
- "quantum::ir";
- ZQN;
- HAL;
- vendor instruction selection;
- cloud provider implementation;
- runtime allocation;
- operating-system scheduling;
- physical topology discovery.

If another subsystem owns a concept, the resource subsystem references it instead of recreating it.

---

7. Required Resource Distinctions

The grammar and semantic model must preserve these distinctions:

resource declaration
        !=
resource reference
        !=
requirement
        !=
capability
        !=
constraint
        !=
budget
        !=
preference
        !=
hint
        !=
negotiation
        !=
scalability
        !=
physical allocation

In particular:

requirement != capability
capability != preference
preference != hint
requirement != implementation choice
logical resource != physical resource
target class != target instance
resource intent != allocation

These are semantic invariants.

---

8. Open-World Resource Model

Zamani must support resource categories that do not exist when the language grammar is written.

Therefore resource kinds are open-ended.

The grammar must permit symbolic names such as:

compute
memory
storage
network
accelerator
tensor.compute
quantum.logical_qubit
quantum.physical_qubit
hardware.reconfigurable
future.resource
future.architecture.resource
vendor.extension

The parser does not need to know whether a resource kind is:

- currently supported;
- vendor-specific;
- future;
- experimental;
- imported from a dialect;
- available on the current target.

That determination belongs downstream.

---

9. No Universal Hardware Ceiling

The grammar must never encode artificial hardware limits.

The following concepts must never become language-level maxima:

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
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK

Equivalent indirect restrictions are also prohibited.

For example, the grammar must never encode:

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

The number of resources represented by a program is determined by program data and semantic structures, not by grammar capacity.

---

10. Meaning of "Infinity"

POCO-REAF scalability means:

«Zamani imposes no artificial finite resource ceiling at the language grammar level.»

It does not mean that every physical target can execute every program.

For example:

requires qubits >= logical_qubits;

may be valid for an arbitrary symbolic "logical_qubits".

If the selected target cannot satisfy it, semantic resource analysis reports an unsatisfied requirement.

The compiler must not silently:

- reduce the requested number;
- change the algorithm;
- remove operations;
- replace a requirement with a preference;
- select a weaker capability;
- alter program meaning.

Therefore:

source portability
        !=
physical feasibility

---

11. Symbolic Resource Quantities

Resource quantities must support symbolic expressions.

Examples:

requires memory >= required_memory;
requires qubits >= logical_qubits;
requires workers >= workload.parallelism;
requires storage >= dataset.size;
requires bandwidth >= workload.bandwidth;

The grammar parses the expressions.

Semantic analysis determines:

- type;
- units;
- dependencies;
- computability;
- feasibility;
- target availability.

The grammar must not assume that quantities are fixed constants.

---

12. Requirements

A requirement is mandatory.

Conceptual form:

requires qubits >= logical_qubits;

means:

«A valid realization must satisfy the stated condition.»

An unsatisfied requirement is a semantic/resource feasibility failure.

It must not silently become:

- a hint;
- a preference;
- an optimization suggestion;
- an ignored annotation.

---

13. Capabilities

A capability describes what a realization may provide.

Examples:

quantum.measurement
quantum.dynamic_circuit
tensor.compute
accelerator.compute
distributed.execution
network.high_bandwidth
hardware.reconfigurable

Capabilities are open-world.

The parser must not contain an exhaustive list of capabilities.

For example:

requires capability("future.accelerator.operation");

must remain structurally representable even when the current compiler does not know how to realize it.

Capability interpretation belongs to semantic analysis and target capability registries.

---

14. Constraints

Constraints express conditions a valid realization must respect.

Examples:

constraint latency <= latency_budget;
constraint energy <= energy_budget;
constraint topology == required_topology;

The grammar records the condition.

It does not determine feasibility.

Feasibility belongs to:

semantic analysis
        |
resource analysis
        |
capability resolution
        |
target planning

---

15. Preferences

Preferences are advisory.

Examples:

prefer low_latency;
prefer capability("tensor.compute");

A preference may influence:

- target selection;
- optimization;
- scheduling;
- placement;
- execution planning.

A preference must never silently become a mandatory requirement.

---

16. Hints

Hints are weaker than preferences.

They may provide implementation guidance without becoming semantic obligations.

Examples:

hint locality;
hint vectorization;
hint accelerator;

An implementation may ignore a hint provided doing so does not violate a stronger semantic requirement.

---

17. Budgets

Budgets represent bounded resources or cost dimensions.

Examples include:

time
energy
memory
storage
network usage
execution cost

Budgets are symbolic and target-independent.

A budget such as:

budget energy <= allowed_energy;

does not select a physical power source.

The target realization determines how the budget is measured.

---

18. Negotiation

Negotiation expresses how a program may select among valid realizations.

Negotiation must distinguish:

mandatory requirements
        |
constraints
        |
preferences
        |
hints
        |
fallback strategies

A fallback may only be used when its semantic conditions permit it.

Negotiation must never silently violate a mandatory requirement.

---

19. Scalability

"scalability.g4" owns source-level scalability intent.

The resource grammar must support symbolic scaling across:

- workload;
- memory;
- compute;
- parallelism;
- data volume;
- tensor dimensions;
- quantum resources;
- network resources;
- distributed resources;
- accelerator resources;
- execution duration;
- other future resource dimensions.

There must be no grammar-level finite scaling ceiling.

---

20. Target Independence

The resource grammar may express target classes:

cpu
gpu
fpga
asic
accelerator
quantum
embedded
distributed
heterogeneous

but must not assume that these are the complete universe of targets.

The distinction is:

target class
    |
    !=
target instance
    |
    !=
physical allocation

For example:

target quantum;

does not mean:

use QPU 0;

---

21. Quantum Boundary

Resource grammar may express quantum resource intent:

requires qubits >= logical_qubits;
requires capability("quantum.measurement");
requires capability("quantum.dynamic_circuit");

It must not own:

- quantum operation semantics;
- gate decomposition;
- routing;
- coupling maps;
- calibration;
- physical qubit assignment;
- error-correction algorithms;
- QEC implementation;
- QPU instruction selection.

Those belong to the quantum semantic subsystem and eventually:

quantum::ir
    |
optimization
    |
decomposition
    |
routing
    |
scheduling
    |
resilience/QEC
    |
ZQN
    |
HAL

---

22. HDL Boundary

Resource intent may describe requirements for:

- timing;
- capacity;
- throughput;
- power;
- thermal characteristics;
- reconfigurability;
- synthesis capabilities;
- hardware capabilities.

The resource grammar must not own:

- signal semantics;
- HDL expression semantics;
- RTL semantics;
- synthesis;
- placement;
- routing;
- physical design.

Those belong to "grammar/hdl/" and "grammar/hardware/".

---

23. Classical Boundary

Classical programs may consume resource semantics for:

- memory;
- compute;
- parallelism;
- vectorization;
- accelerator capabilities;
- storage;
- networking;
- timing.

Resource grammar must not duplicate classical computation syntax.

---

24. AI and Reasoning Boundary

AI, learning, reasoning, knowledge, adaptation, uncertainty, agents, and related facilities may express resource requirements.

For example:

requires capability("tensor.compute");
requires capability("learning.execute");
requires memory >= model.memory;

The resource subsystem does not own:

- model syntax;
- reasoning syntax;
- learning syntax;
- knowledge syntax;
- agent syntax.

Those belong to "grammar/ai/" and related semantic systems.

---

25. Concurrency and Distributed Boundary

Resource semantics may express:

requires workers >= required_workers;
requires capability("distributed.execution");
requires capability("collective.communication");

But the resource grammar does not own:

- actor lifecycle;
- task scheduling;
- channels;
- message passing;
- synchronization;
- distributed algorithms.

Those belong to:

grammar/concurrency/
grammar/distributed/

---

26. Effects Boundary

Resource intent may interact with effects.

Examples:

network
native
foreign
distributed
quantum
measurement
simulation

The resource grammar does not define effect semantics.

Effects belong to:

grammar/effects/

and the semantic effect system.

---

27. Capability Boundary

Capability syntax is owned by:

grammar/resources/capabilities.g4

Capability discovery and resolution are downstream.

The grammar must never query hardware.

The parser cannot know:

does this machine have a GPU?
does this QPU have enough logical qubits?
does this FPGA support this feature?

Those are semantic/resource/backend questions.

---

28. Contract Boundary

Resource requirements may participate in:

requires
ensures
invariant
assume
guarantee
property

but contract semantics are owned by the validation/contract subsystem.

A resource requirement can therefore be represented as part of a larger semantic contract without moving contract ownership into this directory.

---

29. Policy Boundary

Resource selection may be affected by policies such as:

- security;
- authorization;
- deployment;
- cost;
- locality;
- data sovereignty;
- energy;
- reliability;
- trust.

The resource grammar may consume policy-related syntax where explicitly integrated, but policy semantics belong to the policy/security subsystems.

---

30. Provenance Boundary

Resource decisions should be traceable.

The semantic model should be capable of recording:

requirement
    |
capability evaluation
    |
candidate realization
    |
decision
    |
reason
    |
evidence
    |
selected realization

The grammar does not generate provenance records itself.

It preserves the source information required for semantic provenance.

---

31. "resources.g4" Ownership

"resources.g4" is the orchestrator.

It owns:

- public resource entry points;
- resource dispatch;
- resource declarations;
- resource bodies;
- composition of child grammars;
- lifecycle composition;
- generic resource properties;
- integration wrappers;
- resource-domain boundaries.

It must not duplicate rules owned by:

capabilities.g4
requirements.g4
constraints.g4
budgets.g4
preferences.g4
hints.g4
negotiation.g4
scalability.g4
resource-expressions.g4

The rule is:

«Compose; do not duplicate.»

---

32. "resource-expressions.g4" Ownership

This file owns reusable resource expression infrastructure.

It may own:

- resource expressions;
- selectors;
- references;
- literals;
- ranges;
- properties;
- quantities;
- function-like resource expressions;
- generic resource predicates.

It must not redefine specialized payload rules owned by other files.

If a helper conflicts with a specialized public rule, the helper must receive a unique name rather than relying on ANTLR import-order shadowing.

---

33. "requirements.g4" Ownership

Owns:

- "resourceRequirements";
- "resourceRequirement";
- requirement clauses;
- requirement expressions;
- requirement predicates;
- requirement groups;
- requirement comparisons;
- requirement assignments.

Does not own:

- capability discovery;
- target allocation;
- scheduling;
- hardware realization.

---

34. "capabilities.g4" Ownership

Owns:

- capability declarations;
- capability expressions;
- capability requirements;
- capability assertions;
- capability intent;
- capability metadata.

Does not own:

- physical capability discovery;
- target selection;
- hardware inventory.

---

35. "constraints.g4" Ownership

Owns:

- resource constraints;
- constraint expressions;
- constraint atoms;
- comparisons;
- constraint lists.

It must reuse common expression infrastructure rather than creating competing comparison grammars.

---

36. "budgets.g4" Ownership

Owns:

- resource budget declarations;
- budget kinds;
- budget expressions;
- budget constraints;
- budget properties.

Budget values remain symbolic.

---

37. "preferences.g4" Ownership

Owns:

- preferences;
- objectives;
- ordering;
- conditions;
- preference properties;
- preference groups;
- preference values.

Preferences remain advisory.

---

38. "hints.g4" Ownership

Owns:

- hints;
- hint expressions;
- hint specifications;
- hint assignments;
- hint properties;
- hint groups.

Hints remain non-mandatory.

---

39. "negotiation.g4" Ownership

Owns:

- negotiation declarations;
- negotiation requirements;
- negotiation constraints;
- negotiation preferences;
- negotiation hints;
- fallback strategies;
- negotiation phases;
- negotiation policies.

It does not own the actual target-selection algorithm.

---

40. "scalability.g4" Ownership

Owns:

- scalability declarations;
- scalability specifications;
- scalability dimensions;
- scalability policies;
- symbolic bounds;
- scalability assignments.

It must not define machine-size limits.

---

41. ANTLR Import Contract

All resource component grammars must have one clear owner for each public rule.

ANTLR imported grammars are composed into the importing grammar. Therefore duplicate rule names must not be used as an architectural mechanism.

The production requirement is:

one public rule
        |
one owner
        |
many consumers

not:

one rule name
        |
multiple competing definitions
        |
import-order-dependent behavior

This is especially important for rules currently overlapping between resource expression and specialized grammars.

The following classes of duplicate definitions must be eliminated during resource grammar productionization:

resourceCapabilityExpression
resourceCapabilityValue
resourceCapabilityConstraint
resourcePreferenceValue
resourceHintValue
resourceComparisonOperator
optionalResourceExpressionList
resourceTargetExpression

The exact renamed helper must remain internal to its owning grammar where possible.

---

42. Dependency Contract

Every resource grammar file must document:

DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
TYPE_OWNER:
EFFECT_OWNER:
CAPABILITY_OWNER:
RESOURCE_OWNER:
CONTRACT_OWNER:
POLICY_OWNER:
PROVENANCE_OWNER:
IR_OWNER:
TEST_OWNER:
SPEC_OWNER:

The directory-level contract is:

DEPENDS_ON:
    grammar/lexer/*
    grammar/core/*
    grammar/expressions/*
    grammar/types/*

EXPORTS:
    resource grammar entry points

CONSUMED_BY:
    grammar/antlr/ZamaniParser.g4
    grammar/Zamani.g4
    semantic resource analysis
    resource validation

AST_OWNER:
    canonical Zamani AST subsystem

SEMANTIC_OWNER:
    semantic resource/capability analysis

TYPE_OWNER:
    canonical type system

EFFECT_OWNER:
    grammar/effects + semantic effect system

CAPABILITY_OWNER:
    resource capability semantic subsystem

RESOURCE_OWNER:
    resource semantic planner

CONTRACT_OWNER:
    validation/contract subsystem

POLICY_OWNER:
    policy/security semantic subsystem

PROVENANCE_OWNER:
    provenance subsystem

IR_OWNER:
    canonical semantic IR and domain IR boundaries

TEST_OWNER:
    grammar/tests/resources/

SPEC_OWNER:
    grammar/spec/resources.md

---

43. AST Contract

The resource grammar must produce information that can be represented in a domain-neutral AST.

The AST may contain concepts such as:

ResourceDeclaration
ResourceReference
ResourceRequirement
ResourceCapability
ResourceConstraint
ResourceBudget
ResourcePreference
ResourceHint
ResourceNegotiation
ResourceScalability
ResourceProperty
ResourceExpression
ResourceLifecycleIntent

The AST must not contain:

GPU0
QPU0
physical_qubit_17
CUDA_block
vendor_specific_device_layout
hardware_calibration
QEC_schedule
physical_routing

Those are downstream realization concepts.

---

44. Semantic Contract

The semantic resource model must transform source intent into a structured representation suitable for:

requirement analysis
capability resolution
constraint checking
budget analysis
preference ranking
hint processing
negotiation
scalability analysis
execution planning
target realization

The semantic model must preserve the difference between:

mandatory
advisory
optional
conditional
fallback

---

45. Type Contract

Resource expressions participate in the canonical type system.

The resource subsystem must not invent an incompatible type universe.

Values such as:

memory
duration
energy
bandwidth
latency
throughput
count
ratio
probability
capacity

must use canonical types/units where the repository provides them.

Unit checking belongs to semantic/type analysis.

---

46. Effect Contract

Resource declarations themselves do not automatically perform runtime effects.

However, resource-related operations may interact with effects such as:

allocation
network
native
foreign
distributed
simulation
quantum
measurement

The grammar preserves enough information for the effect system to analyze those interactions.

---

47. Resource Contract

The complete semantic resource pipeline is:

source intent
    |
    v
parse
    |
    v
AST
    |
    v
structural validation
    |
    v
semantic resource model
    |
    v
requirement analysis
    |
    v
capability resolution
    |
    v
constraint validation
    |
    v
budget analysis
    |
    v
preference evaluation
    |
    v
negotiation
    |
    v
execution planning
    |
    v
target realization

No grammar rule may skip directly from source syntax to physical allocation.

---

48. Contract Contract

Resource constructs may participate in:

requires
ensures
invariant
assume
guarantee
property

but the resource grammar does not own the semantics of those constructs.

The resource subsystem supplies resource facts and requirements to the contract system.

---

49. Policy Contract

Resource realization may be constrained by policies.

Examples:

security policy
cost policy
energy policy
locality policy
trust policy
deployment policy
data policy
hardware policy

Policies must be evaluated downstream.

The grammar does not silently convert a policy into a resource requirement.

---

50. Provenance Contract

Resource decisions should retain provenance sufficient to answer:

What was required?
Why was it required?
Which capability satisfied it?
Which constraints were evaluated?
Which alternatives were considered?
Why was a realization selected?
Which fallback was used?
Which policy affected the decision?

This is especially important for:

- AI execution;
- scientific computing;
- quantum execution;
- distributed execution;
- hardware selection;
- security;
- reproducibility.

---

51. IR Contract

The resource grammar does not own a target-specific IR.

Resource information must flow through the canonical semantic model into appropriate IRs.

For example:

resource intent
        |
semantic resource model
        |
execution plan
        |
+-------+--------+--------+
|       |        |        |
CPU   GPU/FPGA quantum   distributed
                |
                v
             quantum::ir

The resource grammar must never directly emit:

- LLVM IR;
- vendor GPU IR;
- physical QPU instructions;
- HDL netlists;
- FPGA placement;
- ASIC layout.

---

52. Quantum Integration Contract

Resource syntax must remain independent of physical quantum topology.

Valid examples include the conceptual forms:

requires qubits >= logical_qubits;
requires capability("quantum.measurement");
requires capability("quantum.dynamic_circuit");

The following do not belong in resource grammar semantics:

physical_qubit(17)
coupling_map(...)
calibration(...)
route_to(...)

Those belong downstream.

---

53. HDL Integration Contract

Resource requirements can influence HDL/hardware realization:

requires capability("hardware.reconfigurable");
requires timing <= required_timing;
requires throughput >= required_throughput;

But synthesis and physical design remain outside this directory.

---

54. Backend Integration Contract

The backend consumes the semantic resource model.

It may use resource information for:

- target selection;
- specialization;
- lowering;
- scheduling;
- placement;
- routing;
- execution planning;
- deployment.

The backend must not require the source grammar to know its implementation details.

---

55. Diagnostics Contract

Resource syntax errors must be distinguishable from resource feasibility failures.

Syntax failure

The source cannot be parsed.

Structural failure

The parsed resource construct violates grammar-level structure.

Semantic failure

The construct is syntactically valid but semantically invalid.

Capability failure

A required capability cannot be satisfied.

Resource failure

A mandatory resource condition cannot be satisfied.

Constraint failure

A mandatory realization constraint cannot be satisfied.

Policy failure

A permitted realization is prohibited by policy.

Backend failure

A valid semantic program cannot be realized by the selected implementation.

These must not be collapsed into one generic error.

---

56. Error Preservation

The compiler must never silently recover from an unsatisfied mandatory resource requirement by changing its meaning.

For example:

requires qubits >= n;

must not silently become:

prefer qubits >= n;

Similarly:

requires capability("tensor.compute");

must not silently become:

prefer capability("tensor.compute");

If no realization exists, the compiler reports the failure.

---

57. Determinism

Parsing must be deterministic.

Resource grammar behavior must not depend on:

- hardware discovery;
- network state;
- runtime allocation;
- cloud state;
- vendor state;
- current machine capacity.

Those are downstream semantic/runtime concerns.

Given the same source and grammar version, parsing must produce the same structural result.

---

58. Reproducibility

Resource semantics should preserve enough information for reproducible compilation.

Relevant information includes:

- source resource intent;
- grammar version;
- language version;
- semantic version;
- capability declarations;
- selected policies;
- provenance;
- specialization decisions.

Actual physical realization may vary when the source explicitly permits target adaptation.

---

59. Extensibility

New resource kinds must be addable without rewriting the universal resource grammar.

New capabilities must be addable without modifying the universal resource grammar.

New vendors must not require universal grammar changes merely to introduce vendor-specific capability names.

New hardware architectures must not require a new resource grammar root.

Future resource categories should normally enter through:

qualified names
dialects
capability registries
semantic metadata
resource profiles

rather than universal keyword proliferation.

---

60. Domain Independence

The same resource subsystem serves:

classical computing
scientific computing
AI/ML
data processing
quantum computing
hybrid computing
HDL
hardware/software co-design
embedded systems
accelerators
parallel computing
HPC
distributed computing
network computing
simulation
future computational domains

There must not be separate resource languages for each domain.

---

61. No Application-Specific Resource Keyword Explosion

Application concepts should normally remain identifiers or library/dialect concepts.

For example, the resource grammar should not need dedicated universal keywords for:

robot
vision
sentiment
blockchain
payment
legal
VR
AR
administration

A program can express the relevant capability through the open-world resource model:

requires capability("robotics.control");
requires capability("vision.inference");
requires capability("distributed.ledger");

The resource grammar remains unchanged.

---

62. Resource Profiles

A resource profile may group related resource intent.

Conceptually:

profile scientific_execution {
    ...
}

Profiles must remain semantic abstractions.

They must not become hard-coded hardware profiles.

For example, a profile must not permanently mean:

24 GB GPU
32 CPU cores
64 GB RAM

unless those values are explicit program data in a particular application.

---

63. Resource Groups

Resource groups represent logical collections.

The grammar must permit arbitrarily many members.

It must not impose a fixed number of members.

Conceptually:

group compute_resources {
    ...
}

The actual number of realized resources is downstream.

---

64. Resource Lifecycle

Lifecycle syntax represents intent only.

Potential lifecycle concepts include:

reserve
acquire
allocate
bind
attach
release
deallocate
unbind
detach
migrate
reconfigure
recover

The grammar does not perform these actions.

Runtime/execution/resource management owns actual lifecycle operations.

---

65. Resource Allocation Boundary

The parser must never allocate resources.

The compiler must never assume that parsing a declaration allocates a physical resource.

The distinction is:

resource declaration
        |
        v
semantic resource object
        |
        v
planning
        |
        v
allocation decision
        |
        v
runtime/backend
        |
        v
physical resource

---

66. Security Boundary

Resource capabilities may interact with security capabilities.

Examples:

capability("network.access");
capability("native.execute");
capability("device.control");

Security determines whether those capabilities may be granted.

Resource grammar must not bypass authorization.

---

67. Sandbox Boundary

A sandbox may restrict resource capabilities.

Conceptually:

sandbox {
    ...
}

can constrain:

- network resources;
- native resources;
- filesystem resources;
- hardware resources;
- foreign calls;
- reflection;
- adaptive execution.

Sandbox semantics belong to the security/execution systems.

---

68. Testing Contract

The resource subsystem must contain at least these test classes:

lexical
parser
AST
semantic
type
effect
capability
resource
contract
policy
provenance
compatibility
determinism
scalability
cross-domain
negative
boundary

Recommended location:

grammar/tests/resources/

or the repository's canonical resource test structure.

---

69. Positive Tests

Positive tests must cover at minimum:

resource declarations
resource references
requirements
capabilities
constraints
budgets
preferences
hints
negotiation
scalability
symbolic quantities
resource expressions
resource properties
resource groups
resource profiles
resource lifecycle intent
qualified resource names
future resource names
cross-domain resources

---

70. Negative Tests

Negative tests must cover:

malformed resource declarations
invalid resource clauses
missing required expressions
invalid resource comparisons
invalid capability syntax
invalid requirement syntax
invalid budget syntax
invalid preference syntax
invalid negotiation syntax
invalid scalability syntax
duplicate incompatible declarations
invalid lifecycle forms
invalid nesting

---

71. Semantic Negative Tests

Separate semantic tests must verify:

unsatisfied requirement
unknown required capability
invalid resource type
incompatible units
violated constraint
exceeded budget
forbidden policy
invalid negotiation fallback
invalid scalability relation

These are not parser errors.

---

72. Boundary Tests

Boundary tests must combine resource semantics with:

types
effects
contracts
policies
AI
classical
quantum
hybrid
HDL
hardware
concurrency
distributed
networking
interoperability
metaprogramming
simulation

---

73. Scalability Tests

The resource grammar must be tested with:

one resource
many resources
symbolically sized resources
large resource sets
large requirement sets
large capability sets
large property sets
nested resource groups
nested expressions
large symbolic quantities
large distributed descriptions
large quantum resource requirements
large tensor/resource dimensions

The test suite must not treat an arbitrary fixed maximum as the definition of correctness.

---

74. Cross-Domain Test

At least one conformance program must combine:

classical computation
+
AI/learning
+
reasoning
+
resource requirements
+
capabilities
+
contracts
+
policies
+
provenance
+
parallelism
+
quantum resources
+
hybrid execution
+
HDL/hardware intent
+
simulation
+
distributed execution

The required semantic pipeline is:

source
  |
  v
AST
  |
  v
semantic model
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
execution plan
  |
  +--> classical IR
  +--> quantum::ir
  +--> HDL/hardware semantic model

---

75. Compatibility

Resource syntax must support controlled evolution.

Every incompatible change must define:

old syntax
new syntax
migration rule
deprecation status
compatibility window
diagnostic

The resource grammar must not silently reinterpret old syntax to mean something semantically different.

---

76. Versioning

Resource semantics participate in:

language version
grammar version
AST version
semantic version
IR version
dialect version
capability schema version
resource schema version

The versioning system belongs to compatibility/provenance infrastructure.

---

77. Independent-File Completion Rule

A resource grammar file is not considered complete merely because ANTLR accepts it.

A file is complete when:

ownership defined
        +
dependencies defined
        +
exports defined
        +
AST contract defined
        +
semantic contract defined
        +
type contract defined
        +
effect contract defined
        +
capability contract defined
        +
resource contract defined
        +
contract integration defined
        +
policy integration defined
        +
provenance integration defined
        +
IR boundary defined
        +
diagnostics defined
        +
tests defined
        +
compatibility defined
        +
scalability defined
        +
integration defined

Once those contracts are stable, changing an unrelated resource file must not require reopening the completed file merely to discover how it integrates.

---

78. File Completion Matrix

File| Owns| Primary Consumers
"resources.g4"| orchestration/composition| parser
"resource-expressions.g4"| resource expression infrastructure| all resource grammars
"requirements.g4"| mandatory requirements| semantic resource analysis
"capabilities.g4"| capability intent| capability analysis
"constraints.g4"| mandatory constraints| resource validation
"budgets.g4"| budgets| planning
"preferences.g4"| advisory preferences| optimization/planning
"hints.g4"| advisory hints| planning
"negotiation.g4"| realization negotiation| resource planner
"scalability.g4"| scaling intent| planner/compiler

---

79. Required Integration With the Parser

The canonical parser must consume the resource subsystem through one stable boundary.

Current integration target:

grammar/antlr/ZamaniParser.g4
        |
        v
Resources
        |
        v
resourceElement

The root parser must not duplicate resource productions.

The resource subsystem must expose stable public entry rules.

---

80. Required Integration With the Lexer

Resource grammars consume the canonical lexer vocabulary.

They must not define lexer tokens.

The dependency is:

grammar/antlr/ZamaniLexer.g4
        |
        v
grammar/lexer/*
        |
        v
token vocabulary
        |
        v
resource parser grammars

New resource syntax requiring a reserved word must first establish lexical ownership in the canonical lexer hierarchy.

---

81. Required Integration With Expressions

Resource expressions must consume the canonical expression infrastructure where appropriate.

The resource subsystem must not create a second arithmetic or logical expression language.

The direction is:

canonical expression model
        |
        v
resource expressions

not:

resource expressions
        |
        v
second general expression language

---

82. Required Integration With Types

Resource values must use canonical types.

For example:

quantity
duration
size
energy
bandwidth
latency
count
ratio
probability

must eventually map to the repository's canonical type/units system.

---

83. Required Integration With Effects

Resource-related runtime actions must participate in the canonical effect system.

Examples:

allocate
release
network
native
foreign
device
quantum
distributed
simulation

The resource grammar itself remains declarative.

---

84. Required Integration With Capabilities

The capability grammar defines source intent.

The semantic capability subsystem resolves:

required capability
        |
        v
available capability
        |
        v
candidate realization

This resolution is never performed by ANTLR.

---

85. Required Integration With Execution

Execution planning consumes the semantic resource model.

The planner may determine:

target
specialization
parallelization
placement
scheduling
routing
fallback
retry
migration

The source resource grammar does not make those physical decisions.

---

86. Required Integration With Quantum

Quantum resource intent feeds:

quantum semantic model
        |
        v
quantum::ir

Resource requirements may influence:

- logical resource selection;
- execution strategy;
- decomposition;
- routing;
- scheduling;
- resilience.

They must not encode physical topology.

---

87. Required Integration With Hardware

Hardware realization consumes:

requirements
capabilities
constraints
budgets
preferences
hints

and combines them with actual target information.

The hardware subsystem determines physical feasibility.

---

88. Required Integration With Distributed Computing

Distributed realization may consume:

node requirements
network capabilities
collective communication
latency
bandwidth
topology
reliability
fault tolerance

The grammar remains independent of the number of nodes.

---

89. Required Integration With Future Hardware

Future hardware must be representable without changing the fundamental resource grammar.

For example:

requires capability("future.compute.model");
requires capability("future.acceleration");

should remain structurally valid.

The semantic/backend layers determine whether such capabilities are currently realizable.

---

90. Safe-Rust Contract

The grammar itself contains no Rust implementation code.

Generated parser integration must remain compatible with:

Rust 1.97
Rust 1.97.1
Rust 2021

and the repository's ANTLR Rust runtime.

No grammar feature may require:

unsafe

or unsafe generated extensions.

Semantic/resource implementation must remain in safe Rust.

---

91. No Runtime Coupling

ANTLR parsing must not:

- inspect hardware;
- access files;
- access networks;
- allocate resources;
- query a QPU;
- query a GPU;
- invoke a compiler backend;
- run an AI model;
- perform scheduling;
- perform routing.

Parsing produces structure.

Semantic analysis gives that structure meaning.

---

92. No Hidden Resource Limits

A resource subsystem review must search for both explicit and implicit limits.

Forbidden:

MAX_*
fixed device arrays
fixed resource enum universes
fixed resource counts
fixed topology sizes
fixed capability counts
fixed group sizes
fixed nesting limits
fixed property counts
fixed target counts

unless a limit is explicitly an implementation constraint outside the language semantics and is never exposed as a language ceiling.

---

93. Hard-Coding Audit

Every resource grammar change must pass the following audit:

[ ] No universal hardware maximum
[ ] No fixed resource count
[ ] No fixed device count
[ ] No fixed node count
[ ] No fixed qubit count
[ ] No fixed memory size
[ ] No fixed tensor rank
[ ] No fixed register width
[ ] No vendor-specific physical selection
[ ] No physical topology encoded in grammar
[ ] No runtime allocation in parser
[ ] No backend-specific syntax in universal resource rules
[ ] No duplicated public rule ownership
[ ] No application-specific keyword explosion

---

94. Dependency Direction

The required dependency direction is:

lexer
  |
  v
core / expressions / types
  |
  v
resource expressions
  |
  +------------------------------+
  |              |               |
  v              v               v
requirements capabilities constraints
  |              |               |
  +--------------+---------------+
                 |
        +--------+--------+
        |        |        |
        v        v        v
     budgets preferences hints
        |        |        |
        +--------+--------+
                 |
                 v
            negotiation
                 |
                 v
            scalability
                 |
                 v
          resources.g4
                 |
                 v
             parser
                 |
                 v
                AST
                 |
                 v
             semantics

The actual ANTLR import graph may differ where required by ANTLR grammar composition, but semantic ownership must follow this conceptual direction.

---

95. Anti-Circularity Rule

Resource grammars must not create circular semantic ownership.

For example:

capability -> resource -> capability -> resource

must not become an uncontrolled semantic cycle.

Instead:

source syntax
     |
     v
AST
     |
     v
semantic model
     |
     +--> capability requirement
     |
     +--> resource requirement
     |
     +--> constraint
     |
     v
resolution

The semantic model is the convergence point.

---

96. Resource Expressions Must Remain General

The resource expression subsystem must support symbolic expressions without becoming a second general-purpose programming language.

It may express:

required_memory
logical_qubits
workload.parallelism
dataset.size
topology
capability(...)
property(...)

but general computation belongs to the canonical expression/type system.

---

97. Resource Capability Names Must Remain Open

The following are examples, not a closed enumeration:

quantum.measurement
quantum.dynamic_circuit
tensor.compute
tensor.accelerated
distributed.execution
network.high_bandwidth
hardware.reconfigurable
simulation.quantum
simulation.hardware
learning.execute
reasoning.execute
native.execute
foreign.call

New capabilities should not require a universal grammar rewrite when represented as qualified names or capability metadata.

---

98. Resource Semantics and Program Meaning

A resource declaration must never change the mathematical or computational meaning of the program merely because the target differs.

Target adaptation may change:

- representation;
- scheduling;
- placement;
- decomposition;
- execution strategy;
- specialization.

It must not silently change:

- required semantics;
- observable behavior;
- mandatory contracts;
- required capabilities.

---

99. POCO-REAF Requirement

The resource subsystem is one of the mechanisms that enables:

Program Once
        |
Compile Once
        |
Run Everywhere
        |
Run Anywhere
        |
Run Forever

The source program expresses:

intent
requirements
constraints
capabilities
preferences
policies
contracts

The toolchain determines the realization.

Therefore:

same source
     |
     +--> embedded
     +--> CPU
     +--> multicore
     +--> GPU
     +--> FPGA
     +--> ASIC
     +--> accelerator
     +--> QPU
     +--> simulator
     +--> HPC
     +--> cluster
     +--> distributed
     +--> cloud
     +--> future target

provided the target can satisfy the program's semantic requirements.

---

100. Completion Criteria for "grammar/resources/"

The resource subsystem is production-ready only when all of the following are true:

[ ] resources.g4 is the sole resource grammar orchestrator
[ ] every leaf grammar has one clear ownership boundary
[ ] duplicate public ANTLR rules are eliminated
[ ] resource-expressions.g4 owns generic resource expressions
[ ] requirements.g4 owns requirements
[ ] capabilities.g4 owns capabilities
[ ] constraints.g4 owns constraints
[ ] budgets.g4 owns budgets
[ ] preferences.g4 owns preferences
[ ] hints.g4 owns hints
[ ] negotiation.g4 owns negotiation
[ ] scalability.g4 owns scalability
[ ] root parser integrates through one resource boundary
[ ] no resource grammar defines lexer tokens
[ ] no grammar duplicates the general expression language
[ ] AST remains domain-neutral
[ ] semantic resource model exists
[ ] capability analysis is downstream
[ ] resource feasibility is downstream
[ ] target selection is downstream
[ ] physical allocation is downstream
[ ] quantum::ir remains the quantum IR boundary
[ ] HDL/hardware remain downstream domain boundaries
[ ] no universal hardware ceilings exist
[ ] no fixed resource universe exists
[ ] symbolic quantities are supported
[ ] open-world capability names are supported
[ ] open-world resource kinds are supported
[ ] requirements cannot silently become preferences
[ ] constraints cannot silently disappear
[ ] parser behavior is deterministic
[ ] semantic diagnostics distinguish syntax from feasibility
[ ] positive tests exist
[ ] negative tests exist
[ ] boundary tests exist
[ ] scalability tests exist
[ ] cross-domain tests exist
[ ] compatibility tests exist
[ ] provenance integration exists
[ ] policy integration exists
[ ] contract integration exists
[ ] effect integration exists
[ ] type integration exists
[ ] Rust 1.97/1.97.1 compatibility is verified
[ ] Rust 2021 compatibility is verified
[ ] no unsafe Rust is required
[ ] ANTLR generation succeeds without duplicate-rule ambiguity
[ ] repository tests pass

---

101. Definition of DONE for This README

This README is complete when it serves as the stable contract between every file under "grammar/resources/" and the rest of Zamani.

A developer should be able to open any resource grammar file and answer, without reopening this README or guessing:

What does this file own?
What does it not own?
What rules does it export?
What does it depend on?
Who consumes it?
What AST does it produce?
What semantic model consumes it?
How does typing affect it?
How do effects affect it?
How do capabilities affect it?
How do resources affect it?
How do contracts affect it?
How do policies affect it?
What provenance must survive?
What IR receives its meaning?
What is its quantum boundary?
What is its HDL boundary?
What is its backend boundary?
What errors must it produce?
What tests prove it?
What scalability guarantees apply?
What compatibility guarantees apply?
When is it considered complete?

That is the required standard for every resource grammar file.

---

102. Final Resource Architecture

The production architecture is therefore:

                         RESOURCES
                             |
                    resources.g4
                    /     |      \
                   /      |       \
                  v       v        v
        resource expressions   declarations
                  |
        +---------+---------+
        |         |         |
        v         v         v
 requirements capabilities constraints
        |         |         |
        +---------+---------+
                  |
        +---------+---------+---------+
        |         |         |         |
        v         v         v         v
     budgets preferences hints negotiation
                                      |
                                      v
                                 scalability
                                      |
                                      v
                             domain-neutral AST
                                      |
                                      v
                             semantic resource model
                                      |
             +------------------------+----------------------+
             |                        |                      |
             v                        v                      v
       classical                 quantum::ir           HDL/hardware
             |                        |                      |
             +------------------------+----------------------+
                                      |
                                  optimization
                                      |
                                   lowering
                                      |
                            routing / scheduling
                                      |
                                resilience / QEC
                                      |
                                     ZQN
                                      |
                                     HAL
                                      |
                              target realization

The essential invariant is:

«The grammar describes resource intent; the compiler discovers and plans realization; the runtime/backend performs realization.»

That separation is what allows the resource subsystem to scale from extremely small systems through heterogeneous, quantum, hardware, HPC, distributed, and future computational environments without introducing a language-level ceiling.