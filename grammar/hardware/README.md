Zamani Hardware Grammar

Path: "grammar/hardware/"
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Canonical language root: "grammar/Zamani.g4"
Hardware composition root: "grammar/hardware/hardware.g4"
Grammar technology: ANTLR4
Compiler implementation: Rust 2021
Rust baseline: Rust 1.97 / Rust 1.97.1
Safety requirement: Safe Rust only; production implementation MUST NOT use "unsafe"
Primary portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)
Scalability objective: From the smallest meaningful computation to arbitrarily large computations, limited only by program semantics, representational constraints, declared requirements/policies, compiler/runtime resources, and target availability
Status: Normative hardware-subsystem architecture and implementation-completion contract

---

1. Purpose

"grammar/hardware/" defines the Zamani source-language boundary for expressing hardware intent.

The subsystem provides syntax for describing hardware-related semantics without making the Zamani language dependent on a particular physical machine.

It covers the language-facing representation of:

- hardware declarations;
- abstract hardware;
- hardware resources;
- hardware capabilities;
- hardware requirements;
- hardware constraints;
- hardware preferences;
- hardware hints;
- target classes;
- logical devices;
- accelerators;
- CPU-class computation;
- GPU-class computation;
- FPGA-class computation;
- ASIC-class computation;
- QPU-class computation;
- quantum-device capabilities;
- memory;
- interconnects;
- topology;
- placement;
- timing;
- power;
- thermal properties;
- reliability;
- resilience;
- calibration intent;
- negotiation;
- deployment intent;
- heterogeneous hardware;
- hardware/software co-design;
- future hardware classes.

The fundamental rule is:

«The hardware grammar describes portable hardware intent. It does not perform hardware realization.»

The subsystem therefore sits between the general Zamani language and downstream semantic/compiler infrastructure.

The intended model is:

Zamani Source
     |
     v
Canonical Lexer
     |
     v
Canonical Parser
     |
     v
Frontend AST
     |
     v
Semantic Analysis
     |
     v
Hardware Intent
     |
     v
Resource / Capability / Constraint Analysis
     |
     v
Canonical Semantic Model / IR
     |
     +-------------------+-------------------+
     |                   |                   |
     v                   v                   v
 Classical          quantum::ir       HDL/Hardware
     |                   |                   |
     +-------------------+-------------------+
                         |
                         v
                Optimization / Lowering
                         |
              +----------+----------+
              |          |          |
              v          v          v
           Routing   Scheduling  Resilience
                                    |
                                    v
                                   ZQN
                                    |
                                    v
                                   HAL
                                    |
                                    v
                           Target Realization

---

2. Production Status

This document defines the production target for "grammar/hardware/".

It must not falsely imply that every existing hardware grammar implementation is already complete.

The current repository contains substantial hardware work, including:

hardware.g4
resources.g4
capabilities.g4
constraints.g4
targets.g4
devices.g4
topology.g4
placement.g4
accelerators.g4
cpu.g4
gpu.g4
fpga.g4
asic.g4
qpu.g4
quantum-device.g4
memory.g4
interconnect.g4
timing.g4
power.g4
thermal.g4
reliability.g4
calibration.g4
negotiation.g4
deployment.g4

There are also overlapping or historical concepts such as:

hardware-constraints.g4

The repository currently contains integration inconsistencies that this README explicitly addresses.

In particular, production completion requires convergence of:

- lexer vocabulary;
- parser vocabulary;
- grammar imports;
- grammar ownership;
- AST contracts;
- semantic contracts;
- IR contracts;
- hardware/resource/capability boundaries;
- quantum integration;
- HDL integration;
- compiler integration;
- HAL integration;
- test coverage;
- compatibility;
- scalability;
- hard-coding prevention.

Therefore:

«A grammar file is not production-ready merely because ANTLR accepts it.»

A hardware feature is production-ready only after its complete language-to-runtime contract has been established.

---

3. Repository Architecture Observed

The repository establishes a broader architecture in which:

grammar/DESIGN.md

is the normative grammar architecture.

grammar/Zamani.g4

is intended to be the canonical ANTLR composition root.

grammar/grammar.md

is the implementation-conformance reference.

grammar/Zamani-Grammar.md

is the extended/historical/design reference and must not silently introduce accepted syntax.

The Rust implementation currently includes:

src/lexer.rs
src/parser.rs

and the repository architecture also establishes a domain-neutral frontend AST direction under:

src/frontend/ast/

with canonical quantum semantics eventually reaching:

quantum::ir

The hardware grammar must integrate with that architecture rather than creating a parallel architecture.

---

4. Single-Language Rule

Zamani is one language.

Hardware is not a separate programming language.

Quantum is not a separate programming language.

HDL is not a separate programming language.

Classical computing is not a separate programming language.

AI, distributed computing, networking, and other domains are not separate languages.

They are domains of the same language.

The common foundation includes:

lexical model
identifiers
names
paths
literals
types
expressions
statements
declarations
functions
modules
effects
resources
capabilities
constraints
diagnostics
source locations
versioning
compatibility

Hardware-specific grammar extends this foundation.

It must not replace it.

---

5. Authority Model

The hardware subsystem follows the repository-wide authority model.

The intended hierarchy is:

grammar/DESIGN.md
        |
        v
grammar/specification/
        |
        v
grammar/spec/
        |
        v
feature contracts
        |
        v
canonical lexer contract
        |
        v
grammar/Zamani.g4
        |
        v
Rust lexer/parser
        |
        v
domain-neutral AST
        |
        v
semantic analysis
        |
        v
canonical IR
        |
        v
compiler/backend/runtime
        |
        v
grammar/grammar.md

"grammar/Zamani-Grammar.md" remains an extended/historical/design reference.

It must not silently become a second authority.

Likewise, "grammar/grammar.md" documents implementation conformance and must not become a competing language specification.

---

6. Relationship to "grammar/DESIGN.md"

"grammar/DESIGN.md" is the repository-wide architectural authority.

This README specializes that architecture for hardware.

Therefore:

- "DESIGN.md" defines global architectural rules;
- this README defines hardware-specific ownership;
- individual ".g4" files define individual syntax contracts;
- Rust implementation defines executable frontend behavior;
- semantic analysis defines hardware meaning;
- IR defines compiler representation;
- backends/HAL/runtime define realization.

If this document conflicts with "grammar/DESIGN.md", "DESIGN.md" takes precedence and this document must be corrected.

---

7. Relationship to "grammar/Zamani.g4"

"grammar/Zamani.g4" remains the single complete-language ANTLR composition root.

"grammar/hardware/hardware.g4" is only the hardware-domain composition root.

The relationship is:

grammar/Zamani.g4
        |
        +--> core
        +--> types
        +--> expressions
        +--> declarations
        +--> statements
        +--> functions
        +--> modules
        +--> effects
        +--> memory
        +--> concurrency
        |
        +--> classical
        +--> quantum
        +--> hybrid
        +--> HDL
        +--> hardware
        +--> distributed
        +--> AI
        +--> data
        +--> networking
        +--> security
        +--> interoperability
        +--> dialects
        +--> macros
        +--> metaprogramming

The hardware grammar MUST NOT become another complete Zamani grammar.

---

8. Current Lexer Integration Requirement

The actual "src/lexer.rs" currently defines its own "TokenType" enumeration and contains tokens for general language concepts as well as Zamani-specific concepts.

Examples include:

Identifier
String
Integer
Float
Char
Boolean
QuantumLiteral
NanoAnnotation
MTSLiteral

and many keywords/operators.

It also currently contains overlapping token concepts such as:

BitAnd
Ampersand

Question

and the parser uses both "BitAnd" and "Ampersand" in precedence handling.

Hardware grammar files must not introduce another hardware-specific token vocabulary.

In particular, the existing hardware grammar material that uses "ZamaniTokens" / "K_*" style vocabulary must converge with the canonical lexer architecture.

The production rule is:

«Hardware grammar consumes the canonical Zamani token vocabulary. It does not invent a second lexical namespace.»

Any token that is genuinely required must first be established through the canonical lexical contract.

---

9. Canonical Names and Expressions

Hardware grammars must reuse the repository's canonical:

identifier
qualified name
path
expression
literal
type
attribute
modifier

rules.

A hardware grammar must not redefine:

identifier
qualifiedName
expression
type
literal

merely for convenience.

This is required to prevent grammar divergence.

For example:

hardware.g4
resources.g4
targets.g4
devices.g4
topology.g4
placement.g4

must all consume the same conceptual name/expression/type system.

---

10. Ownership Model

"grammar/hardware/" owns source syntax for hardware intent.

It owns:

- hardware declarations;
- hardware contracts;
- hardware resource references;
- hardware capability references;
- hardware requirements;
- hardware constraints;
- hardware preferences;
- hardware hints;
- abstract target references;
- logical device declarations;
- accelerator declarations;
- hardware topology intent;
- placement intent;
- memory intent;
- interconnect intent;
- timing intent;
- power intent;
- thermal intent;
- reliability intent;
- calibration intent;
- deployment intent;
- hardware negotiation;
- hardware-domain properties;
- hardware-domain extension points.

It does not own:

- lexical definitions;
- general identifiers;
- general expression precedence;
- general types;
- compiler resource allocation;
- hardware discovery;
- physical device enumeration;
- device drivers;
- physical addressing;
- routing algorithms;
- scheduling algorithms;
- optimization algorithms;
- synthesis implementation;
- calibration algorithms;
- QEC implementation;
- ZQN implementation;
- quantum IR;
- runtime execution.

---

11. Hardware Intent Versus Hardware Realization

This distinction is the central design rule.

The source program describes:

WHAT

The compiler and runtime determine:

WHERE
WHEN
HOW
ON WHICH TARGET
USING WHICH PHYSICAL RESOURCES

For example:

requires capability("gpu.compute")

is hardware intent.

It does not mean:

use GPU #0

Likewise:

requires qubits >= required_qubits

is a resource requirement.

It does not mean:

use physical qubits 0..N

Likewise:

requires topology(...)

is topology intent.

It does not perform routing.

---

12. POCO-REAF

The hardware subsystem must protect:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

POCO-REAF means that a programmer can express stable computation semantics without rewriting the fundamental program solely because the realization changes.

Potential realizations include:

atom-scale / nano-oriented target
embedded processor
single CPU
multicore CPU
GPU
FPGA
ASIC
accelerator
QPU
quantum simulator
heterogeneous system
HPC system
cluster
distributed system
cloud
future target

The source remains target-independent wherever the computation's semantics permit.

---

13. POCO-REAF Does Not Remove Resource Requirements

POCO-REAF does not mean:

«every program can execute on every machine regardless of resources.»

These states are different:

lexically valid
    !=
syntactically valid
    !=
semantically valid
    !=
target compatible
    !=
resource feasible
    !=
runtime available

For example:

requires capability("quantum.mid_circuit_measurement")

may be valid Zamani syntax and valid program semantics even if a particular target lacks that capability.

The compiler may:

- choose another target;
- choose another execution mode;
- transform the program if semantics permit;
- use a simulator;
- distribute the computation;
- use logical resources;
- decompose operations;
- route operations;
- schedule operations;
- apply resilience mechanisms;
- reject the target with a precise diagnostic.

It must not silently change the program's meaning.

---

14. Absolute Scalability Rule

No hardware grammar may impose universal hardware capacities.

The following concepts MUST NOT become language-level constants:

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

Equivalent disguised restrictions are also prohibited.

For example, do not encode:

qubit0
qubit1
qubit2
...

as the only representable hardware model.

Do not encode:

gpu0
gpu1
gpu2

as the universal GPU model.

Do not encode:

wire [31:0]

as a universal hardware width.

Do not encode:

RAM = 64GB
VRAM = 24GB

as language assumptions.

---

15. Program Constants Versus Language Limits

Normal program constants remain valid.

For example:

let n = 1024;

is program data.

Likewise:

Tensor<T, 1024, 1024>

may be valid program semantics.

What is prohibited is turning that value into a universal implementation ceiling:

MAX_TENSOR_DIMENSION = 1024

or:

Zamani tensors can never exceed 1024 × 1024

The rule applies to:

- qubits;
- CPUs;
- cores;
- threads;
- GPUs;
- FPGAs;
- ASIC resources;
- memory;
- tensor dimensions;
- tensor rank;
- register widths;
- nodes;
- devices;
- links;
- timelines;
- ports;
- channels.

---

16. Requirement / Capability / Constraint / Preference / Hint

The hardware grammar MUST keep these concepts distinct.

16.1 Requirement

A condition required for a valid realization.

Conceptually:

requires qubits >= required_qubits

16.2 Capability

A capability provided by a target.

Conceptually:

requires capability("quantum.measurement")

16.3 Constraint

A condition that must hold.

Conceptually:

requires latency <= latency_budget

16.4 Preference

A desirable realization property that is not necessarily mandatory.

Conceptually:

prefer accelerator("tensor.compute")

16.5 Hint

Information supplied to optimization without changing program meaning.

Conceptually:

hint locality

16.6 Realization

A downstream decision mapping abstract resources to concrete resources.

For example:

logical resource
      |
      v
target resource
      |
      v
physical resource

This mapping belongs downstream.

---

17. Resource Semantics

Hardware resources are semantic quantities, not fixed machine assumptions.

Possible resource classes include:

compute
memory
storage
bandwidth
latency
throughput
energy
power
thermal capacity
reliability
quantum resources
programmable logic
communication
accelerator capacity

Resource quantities must be expressible through the canonical expression system.

Examples:

requires resource.compute >= required_compute
requires resource.memory >= required_memory
requires resource.qubits >= required_qubits
requires resource.bandwidth >= required_bandwidth

The actual target value is determined during semantic/resource analysis and target realization.

---

18. Capability Semantics

Capabilities describe what an implementation can provide.

Examples:

quantum.compute
quantum.measurement
quantum.mid_circuit_measurement
quantum.dynamic_control
tensor.compute
gpu.compute
fpga.reconfiguration
programmable_logic
distributed.communication
high_bandwidth_memory
cryptographic.compute

Capabilities must be extensible.

Vendor-specific capabilities must not pollute the core language with permanent keywords.

Qualified names are preferred:

vendor.domain.capability

or another canonical extension mechanism defined by the core capability model.

---

19. Target Semantics

A target represents an abstract realization class or target contract.

Examples:

cpu
gpu
fpga
asic
accelerator
quantum
simulator
embedded
distributed
cloud
heterogeneous
custom.domain.target

A target is not automatically a physical device.

The grammar must not assume:

target gpu0

means a particular physical GPU unless a downstream deployment contract explicitly defines that meaning.

---

20. Device Semantics

"devices.g4" owns logical device declarations and device intent.

It must not perform physical device discovery.

The grammar must not depend on:

- PCI addresses;
- serial numbers;
- IP addresses;
- physical qubit IDs;
- operating-system device paths;
- driver handles;
- machine-specific IDs.

A source-level device name is semantic unless a separate deployment/realization contract gives it physical meaning.

---

21. Topology Semantics

"topology.g4" owns abstract topology intent.

It may represent:

- logical nodes;
- logical endpoints;
- connectivity;
- relationships;
- direction;
- locality;
- distance;
- grouping;
- hierarchy;
- topology properties;
- topology requirements;
- topology constraints;
- topology preferences.

It does not perform:

- graph discovery;
- route generation;
- shortest-path computation;
- physical placement;
- scheduling.

The downstream flow is:

topology intent
       |
       v
semantic topology model
       |
       v
target topology
       |
       v
routing

---

22. Placement Semantics

"placement.g4" owns placement intent.

It may express:

- affinity;
- anti-affinity;
- co-location;
- separation;
- locality;
- grouping;
- migration intent;
- elasticity;
- placement constraints;
- placement preferences;
- placement hints.

It does not perform actual placement.

The flow is:

placement intent
       |
       v
semantic placement model
       |
       v
resource/target analysis
       |
       v
physical placement

---

23. Memory Integration

"hardware/memory.g4" is a hardware-facing memory contract.

The generic memory model remains owned by the memory subsystem.

The relationship is:

grammar/memory/
        |
        v
generic memory semantics
        |
        v
grammar/hardware/memory.g4
        |
        v
hardware memory intent

Hardware memory syntax may describe:

- capacity requirements;
- bandwidth;
- latency;
- address-space properties;
- memory classes;
- locality;
- persistence;
- coherence requirements;
- capability requirements;
- resource requirements;
- power/energy properties.

It must not assume:

64GB RAM
24GB VRAM
32-bit registers

as universal limits.

---

24. Interconnect Integration

"interconnect.g4" owns source-level interconnect intent.

It may represent:

- logical links;
- endpoint groups;
- channels;
- fabrics;
- direction;
- bandwidth;
- latency;
- distance;
- reliability;
- ordering;
- flow control;
- QoS;
- protocol intent;
- connectivity;
- abstract routing domains.

It does not perform routing.

The flow is:

interconnect intent
       |
       v
semantic interconnect model
       |
       v
topology analysis
       |
       v
routing
       |
       v
scheduling
       |
       v
HAL

---

25. Timing Integration

Timing syntax describes intent.

It may represent:

- timing constraints;
- latency;
- throughput;
- ordering;
- synchronization;
- deadlines;
- timing relationships;
- clock intent;
- temporal constraints.

Timing syntax must not assume a universal hardware clock frequency.

For example, do not establish:

clock = 1GHz

as a language-wide assumption.

Actual clocking is target realization.

---

26. Power and Thermal Integration

Power and thermal syntax describes constraints and requirements.

It may express:

power budget
energy budget
thermal budget
thermal constraints
power preferences
energy requirements

The grammar does not perform:

- thermal simulation;
- power estimation;
- DVFS;
- physical cooling;
- chip-level thermal placement.

Those belong downstream.

---

27. Reliability and Resilience

"reliability.g4" owns hardware-facing reliability intent.

The grammar may express:

- reliability requirements;
- availability requirements;
- fault tolerance;
- resilience requirements;
- degradation policies;
- recovery intent;
- redundancy intent.

The existing resilience vocabulary must remain semantically compatible with:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

and outcomes:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

These are semantic states/outcomes.

The grammar does not implement recovery.

---

28. Calibration Boundary

"calibration.g4" may describe calibration-related intent or requirements.

It must not implement calibration.

Calibration algorithms, measurements, device characterization, and calibration data belong downstream.

For quantum systems this is particularly important.

The architecture remains:

source
  |
  v
hardware/quantum intent
  |
  v
semantic analysis
  |
  v
quantum::ir
  |
  v
device analysis
  |
  v
calibration

---

29. Negotiation Boundary

"negotiation.g4" represents requirements and capability negotiation intent.

It must not itself query hardware.

For example:

requires capability("tensor.compute")

is source-level intent.

Actual capability discovery occurs through target/HAL/runtime infrastructure.

The grammar is deterministic and must behave identically regardless of the physical machine available during parsing.

---

30. Deployment Boundary

"deployment.g4" describes deployment intent.

It may represent:

- target classes;
- environment requirements;
- resource requirements;
- deployment constraints;
- placement preferences;
- resilience policies;
- execution modes.

It must not directly perform deployment.

Actual deployment belongs downstream.

---

31. CPU / GPU / FPGA / ASIC / Accelerator Integration

The specialized files:

cpu.g4
gpu.g4
fpga.g4
asic.g4
accelerators.g4

must describe classes of hardware capability and intent, not hard-coded machines.

They may describe:

compute characteristics
capability classes
memory relationships
parallelism properties
programmability
reconfiguration
specialized acceleration
performance properties
constraints
requirements

They must not impose:

exact core count
exact GPU count
exact register count
exact SIMD width
exact FPGA capacity
exact ASIC dimensions

as universal language limits.

---

32. Quantum Device Integration

"quantum-device.g4" is hardware-domain syntax.

It may describe:

- quantum-device class;
- quantum capabilities;
- measurement;
- reset;
- dynamic circuits;
- mid-circuit measurement;
- classical feed-forward;
- logical/physical resource relationships;
- abstract connectivity;
- timing;
- quality requirements;
- reliability;
- resource requirements.

It must not create a second quantum IR.

The semantic flow is:

quantum source
      |
      v
frontend AST
      |
      v
quantum semantic analysis
      |
      v
quantum::ir
      |
      v
hardware capability/resource analysis
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

"quantum::ir" remains canonical.

---

33. Relationship to the Quantum Grammar

The hardware subsystem must not duplicate quantum operation syntax.

For example, the quantum subsystem owns semantic quantum operations such as:

apply H ...
apply custom_gate ...
apply vendor.operation ...
apply operation(parameter) ...

Hardware owns the capability of the target to provide those operations.

This means the distinction is:

quantum:
    what operation the program performs

hardware:
    what capability the target provides

compiler:
    how the operation is realized

routing:
    where resources are placed

scheduling:
    when operations execute

QEC/ZQN:
    how resilience/noise is handled

HAL:
    how the target exposes execution

---

34. No Fixed Quantum Hardware Model

The hardware subsystem must never assume:

MAX_QUBITS

or any equivalent hidden maximum.

It must support semantic quantities such as:

requires resource.qubits >= n

where "n" is a program expression.

A target may have:

small capacity
large capacity
distributed capacity
logical capacity
simulated capacity
future capacity

without requiring changes to the grammar.

---

35. HDL Boundary

Hardware grammar and HDL grammar are related but not identical.

Hardware grammar describes:

hardware intent
resource relationships
target requirements
capabilities
deployment
placement
topology
constraints

HDL describes:

hardware structure
signals
nets
ports
modules
state
combinational behavior
sequential behavior
clocking
timing
verification
synthesis intent

The hardware subsystem must not absorb HDL behavioral semantics.

The HDL subsystem must not become a hidden hardware deployment language.

The integration is:

Zamani program
      |
      +--> software/classical semantics
      |
      +--> quantum semantics
      |
      +--> HDL semantics
      |
      +--> hardware intent
              |
              v
       hardware/software
          co-design
              |
              v
       compiler/lowering

---

36. Hardware/Software Co-Design

Zamani should allow one semantic program to express computation plus hardware intent where appropriate.

For example:

computation
    +
memory intent
    +
parallelism intent
    +
accelerator intent
    +
communication intent
    +
timing constraints
    +
hardware implementation intent

The same semantic computation may subsequently be realized as:

CPU execution
GPU execution
FPGA accelerator
ASIC implementation
quantum/hybrid execution
heterogeneous execution
distributed execution

The grammar must preserve the distinction between algorithmic semantics and implementation intent.

---

37. Generic Parameters

Hardware syntax must support symbolic and generic quantities.

Conceptually:

hardware Target<N> {
    requires resource.compute >= N;
}

or:

hardware QuantumTarget<Q> {
    requires resource.qubits >= Q;
}

The exact surface syntax must follow the canonical Zamani type/expression/generic systems.

The hardware grammar must not invent a second generic mechanism.

---

38. Qualified Names

Hardware extensions should be representable through qualified names.

Examples:

vendor.compute.capability
vendor.quantum.operation
domain.accelerator.feature

This allows future hardware technologies without requiring a new Zamani keyword for every vendor or technology.

The language should evolve through semantic capability registries and dialect mechanisms rather than permanent keyword accumulation.

---

39. Vendor Independence

The core hardware grammar must not contain permanent vendor-specific syntax merely because a current device exists.

Vendor-specific behavior should be represented through:

- qualified names;
- capabilities;
- properties;
- dialects;
- interoperability contracts;
- backend metadata.

For example, the grammar should not need:

nvidia_gpu
amd_gpu
intel_gpu
vendor_x_qpu
vendor_y_fpga

as universal keywords.

---

40. No Physical Hardware Discovery During Parsing

Parsing must be deterministic.

The parser must not ask:

How many GPUs are installed?
How many qubits exist?
Which FPGA is available?
How much RAM exists?
What is the PCI address?
What driver is installed?

The parser must operate on source text only.

Target/resource discovery occurs later.

Therefore:

same source
+
same language version
+
same lexical/parser rules
=
same syntax result

regardless of the target machine.

---

41. No Runtime Execution in Grammar

ANTLR grammar files must remain action-free.

Hardware grammar MUST NOT:

- execute hardware operations;
- access files;
- access networks;
- invoke drivers;
- query operating-system devices;
- allocate physical resources;
- perform routing;
- perform scheduling;
- perform calibration;
- execute quantum operations;
- perform synthesis.

No embedded Rust implementation belongs in the grammar.

---

42. Rust Safety

The Rust implementation baseline is:

Rust 1.97 / Rust 1.97.1
Rust 2021

Production compiler/runtime implementation must use safe Rust.

No "unsafe" implementation is required for the hardware grammar architecture.

The grammar itself must remain action-free.

Generated ANTLR/Rust integration must be reviewed so that no project-owned unsafe implementation is introduced.

If a generated dependency internally contains unsafe implementation details, that dependency must be evaluated separately; the Zamani implementation itself must not add unsafe blocks or unsafe APIs merely to support hardware grammar functionality.

---

43. Current Rust Lexer/Parser Integration

The current "src/parser.rs" is a recursive-descent/Pratt parser and has its own precedence model.

It currently handles concepts such as:

Assign
Range
LogicalOr
LogicalAnd
BitOr
BitXor
BitAnd
Equality
Comparison
Shift
Sum
Product
Prefix
Call
Index
Member

Hardware grammar must not redefine these precedence relationships.

Hardware expressions must reuse the canonical expression semantics.

If hardware syntax requires an expression, the hardware feature contract must identify:

canonical expression rule
AST representation
semantic interpretation
IR interpretation

before the hardware grammar is considered complete.

---

44. AST Contract

Every hardware grammar construct requires a predetermined AST mapping.

The required chain is:

grammar rule
      |
      v
AST node
      |
      v
semantic hardware model
      |
      v
canonical IR

A grammar rule is incomplete if its AST mapping is undefined.

For example:

hardwareRequirement

must identify:

AST representation
semantic requirement representation
resource/capability interaction
diagnostics
IR representation or lowering boundary
compiler consumers

before it is declared complete.

---

45. Domain-Neutral AST Requirement

Hardware-specific parser rules must not force vendor-specific or physical-device-specific structures into:

src/frontend/ast/

The frontend AST should preserve language structure.

Hardware-specific meaning belongs in semantic analysis.

For example, an AST should represent a generic hardware requirement rather than directly storing a physical GPU handle.

---

46. Semantic Contract

Semantic analysis owns:

- capability validation;
- resource validation;
- requirement validation;
- constraint validation;
- target compatibility;
- type compatibility;
- topology compatibility;
- placement compatibility;
- hardware property validation.

The grammar only establishes syntax.

---

47. IR Contract

The hardware subsystem must map to the repository's canonical IR architecture.

It must not create a second competing hardware IR merely because hardware syntax exists.

Where a hardware-specific IR is necessary, it must have an explicitly defined boundary and relationship to the canonical compiler IR.

Quantum semantics MUST continue to reach:

quantum::ir

and must not be diverted into:

hardware::quantum_ir

or another competing representation.

---

48. Resource and Capability Integration

The hardware subsystem intersects strongly with:

grammar/resources/
grammar/core/
grammar/compile/
grammar/execution/
grammar/quantum/
grammar/hdl/

The ownership rule is:

core:
    universal resource/capability concepts

resources:
    general resource semantics

hardware:
    hardware-specific resource/capability intent

quantum:
    quantum computation semantics

HDL:
    hardware-description semantics

compile:
    compilation and target selection

execution:
    runtime/execution policies

HAL:
    actual target capability/resource realization

No subsystem should duplicate another subsystem's authority.

---

49. Target Selection Boundary

Hardware grammar can express target requirements and preferences.

It must not perform target selection.

The distinction is:

target requirement
        |
        v
target analysis
        |
        v
candidate targets
        |
        v
selection policy
        |
        v
chosen realization

The grammar stops before selection.

---

50. Routing Boundary

Topology and placement syntax feed routing.

They do not implement routing.

The intended pipeline is:

logical topology
      |
      v
hardware semantic model
      |
      v
target topology
      |
      v
routing
      |
      v
physical realization

Quantum routing remains compatible with "quantum::ir".

---

51. Scheduling Boundary

Hardware timing and resource requirements feed scheduling.

The grammar does not schedule.

Scheduling must remain downstream so that the same program can be scheduled differently on different targets.

---

52. Optimization Boundary

Hardware preferences and hints may guide optimization.

They must not force optimizer implementation into grammar.

The grammar expresses:

intent

The optimizer determines:

implementation

provided program semantics remain unchanged.

---

53. Resilience / QEC / ZQN Boundary

Hardware reliability and resilience requirements may feed:

resilience
QEC
ZQN

but the hardware grammar must not duplicate their implementations.

For quantum computing:

hardware capability
       |
       v
quantum semantic model
       |
       v
quantum::ir
       |
       v
QEC / resilience / ZQN
       |
       v
target realization

The grammar does not implement QEC.

The grammar does not implement noise simulation.

The grammar does not implement ZQN.

---

54. HAL Boundary

The Hardware Abstraction Layer is the boundary through which concrete target capabilities and operations become available to the compiler/runtime.

The grammar does not call the HAL.

The flow is:

source
  |
  v
grammar
  |
  v
AST
  |
  v
semantic model
  |
  v
compiler
  |
  v
HAL
  |
  v
target

This separation is essential for POCO-REAF.

---

55. Hardware File Ownership

The existing hardware directory should be organized according to the following ownership model.

"hardware.g4"

Owns:

- hardware composition;
- hardware entry point;
- hardware declaration dispatch;
- hardware-domain statement dispatch;
- hardware-domain integration.

Does not own specialized leaf syntax.

Completion requires:

canonical imports
canonical lexer
canonical names
canonical expressions
canonical types
all leaf grammars integrated
no duplicate rules
AST contract
semantic contract
IR contract
tests

---

"resources.g4"

Owns:

- hardware resource declarations;
- resource references;
- quantities;
- resource requirements;
- resource constraints;
- resource preferences;
- resource profiles;
- hardware resource properties.

Does not own generic resource semantics or physical allocation.

---

"capabilities.g4"

Owns:

- hardware capability integration;
- hardware capability requirements;
- capability preferences;
- capability constraints;
- hardware capability properties;
- hardware capability extension points.

It must consume the canonical capability model.

---

"constraints.g4"

Owns:

- hardware-specific constraint integration;
- hardware constraint classification;
- hardware resource constraints;
- hardware capability constraints;
- hardware target constraints;
- hardware topology constraints;
- hardware timing constraints;
- hardware reliability constraints;
- hardware implementation-intent constraints.

It must not duplicate generic constraint expression semantics.

---

"hardware-constraints.g4"

This file is currently overlapping with "constraints.g4".

It must not remain a second production implementation.

The final repository should choose one authoritative implementation.

The preferred transition is:

hardware-constraints.g4
        |
        v
compatibility/deprecation facade
        |
        v
constraints.g4

after repository-wide references are migrated.

It may eventually be retired only after:

- references are migrated;
- tests are migrated;
- imports are migrated;
- compatibility requirements are satisfied.

No unnecessary rename should be performed merely for cosmetic reasons.

---

"targets.g4"

Owns:

- target declarations;
- target classes;
- target parameters;
- target requirements;
- target constraints;
- target preferences;
- target properties;
- target extension.

Does not own target selection or physical target discovery.

---

"devices.g4"

Owns:

- logical device declarations;
- device classes;
- device capabilities;
- device resources;
- device relationships;
- device requirements;
- device constraints;
- device preferences.

Does not enumerate physical devices.

---

"topology.g4"

Owns:

- abstract topology;
- logical nodes;
- logical endpoints;
- relationships;
- connectivity;
- topology properties;
- topology requirements;
- topology constraints;
- topology preferences.

Does not own routing.

---

"placement.g4"

Owns:

- placement declarations;
- affinity;
- anti-affinity;
- co-location;
- separation;
- locality;
- grouping;
- migration;
- elasticity;
- placement requirements;
- placement constraints;
- placement preferences.

Does not perform placement.

---

"accelerators.g4"

Owns generic accelerator intent.

It must not encode individual accelerator vendors as permanent language features.

---

"cpu.g4"

Owns CPU-class hardware intent.

It must not impose fixed core/thread/register limits.

---

"gpu.g4"

Owns GPU-class hardware intent.

It must not impose fixed GPU counts, memory sizes, or vendor-specific universal assumptions.

---

"fpga.g4"

Owns FPGA-class intent:

- programmable logic;
- reconfiguration;
- accelerator intent;
- resource relationships;
- interfaces;
- timing intent.

It must not encode one FPGA family as the language model.

---

"asic.g4"

Owns ASIC-class intent.

It must not encode fixed silicon characteristics as universal language limits.

---

"qpu.g4"

Owns QPU-class target intent.

It must not duplicate quantum computation semantics.

---

"quantum-device.g4"

Owns quantum-device hardware capability intent.

It must integrate with:

grammar/quantum/
src/quantum/
quantum::ir
QEC
ZQN
HAL

without creating another quantum language or IR.

---

"memory.g4"

Owns hardware-facing memory intent.

Generic memory remains in:

grammar/memory/

---

"interconnect.g4"

Owns hardware-facing communication/interconnect intent.

Routing remains downstream.

---

"timing.g4"

Owns hardware timing intent.

Actual scheduling remains downstream.

---

"power.g4"

Owns power intent.

Power estimation and physical power management remain downstream.

---

"thermal.g4"

Owns thermal intent.

Thermal analysis remains downstream.

---

"reliability.g4"

Owns reliability requirements and intent.

Reliability implementation remains downstream.

---

"calibration.g4"

Owns calibration-related source intent.

Calibration algorithms remain downstream.

---

"negotiation.g4"

Owns source-level capability/resource negotiation intent.

Actual negotiation/discovery remains downstream.

---

"deployment.g4"

Owns source-level deployment intent.

Actual deployment remains downstream.

---

56. Independent File Completion Contract

Every file under "grammar/hardware/" MUST be independently completable.

Before implementing a file, its contract must answer:

File
Purpose
Status
Owns
Does Not Own
Dependencies
Upstream Contracts
Downstream Consumers
Public Rules
AST Mapping
Semantic Mapping
IR Mapping
Compiler Consumers
Runtime Consumers
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Determinism Tests
Compatibility
Diagnostics
Security
Performance
Hard-Coding Audit
Completion Criteria

This ensures that finishing one file does not require reopening it merely because another file was subsequently implemented.

---

57. Dependency Direction

Dependencies must flow toward shared foundations.

Preferred direction:

lexer
   |
   v
core names/types/expressions
   |
   v
hardware leaf grammars
   |
   v
hardware composition
   |
   v
Zamani composition root

Avoid:

hardware A -> hardware B
hardware B -> hardware C
hardware C -> hardware A

Circular grammar dependencies must not be introduced.

Where multiple hardware domains need a common concept, the concept belongs in the appropriate shared foundation.

---

58. Hardware Composition Contract

The intended hardware composition is:

hardware.g4
    |
    +--> resources.g4
    +--> capabilities.g4
    +--> constraints.g4
    +--> targets.g4
    +--> devices.g4
    +--> topology.g4
    +--> placement.g4
    +--> accelerators.g4
    +--> memory.g4
    +--> interconnect.g4
    +--> timing.g4
    +--> power.g4
    +--> thermal.g4
    +--> reliability.g4
    +--> calibration.g4
    +--> negotiation.g4
    +--> deployment.g4
    +--> cpu.g4
    +--> gpu.g4
    +--> fpga.g4
    +--> asic.g4
    +--> qpu.g4
    +--> quantum-device.g4

This is a composition relationship, not a duplication relationship.

---

59. Grammar Modularity

The hardware subsystem should use leaf grammars for concepts that genuinely have independent ownership.

However, excessive fragmentation is also prohibited.

A new file is justified only when it provides:

- a coherent ownership boundary;
- reusable syntax;
- independent tests;
- a clear integration point;
- a meaningful semantic boundary.

Do not create a file merely to move five grammar lines somewhere else.

---

60. No Competing Hardware Grammar

There must be exactly one canonical hardware-domain grammar composition.

Do not create:

hardware.g4
hardware2.g4
universal-hardware.g4
hardware-language.g4

as competing roots.

Likewise, do not create another hardware directory beside:

grammar/hardware/

unless a future architectural decision explicitly establishes a different responsibility.

Existing functionality must be expanded and normalized.

---

61. Feature Contracts

Hardware features should eventually have machine-readable feature contracts under the repository's feature-contract system.

A hardware feature contract should identify:

feature ID
name
status
version
syntax owner
lexer dependencies
AST nodes
semantic rules
IR mapping
compiler consumers
runtime consumers
capabilities
resources
constraints
tests
compatibility
hard-coding policy

This allows a feature to be independently completed.

---

62. Feature Promotion

A hardware feature follows:

Zamani-Grammar.md
        |
        v
proposal
        |
        v
semantic design
        |
        v
feature contract
        |
        v
AST contract
        |
        v
grammar
        |
        v
lexer/parser
        |
        v
semantic implementation
        |
        v
IR
        |
        v
compiler/backend
        |
        v
tests
        |
        v
stable

Presence in "Zamani-Grammar.md" alone never makes a feature legal Zamani syntax.

---

63. Positive Tests

Every hardware feature needs positive tests.

Examples include:

minimal hardware contract
resource requirement
capability requirement
target declaration
logical device
abstract topology
placement intent
memory requirement
interconnect requirement
accelerator preference
timing requirement
reliability requirement
deployment intent
heterogeneous target
parameterized hardware contract
qualified capability

---

64. Negative Tests

Every hardware feature must also have rejection tests.

Examples:

invalid hardware declaration
invalid capability syntax
invalid resource expression
invalid topology
invalid placement
duplicate invalid declarations
malformed qualified name
invalid constraint
invalid target contract
invalid hardware-specific type usage

Negative tests are necessary to establish the actual language boundary.

---

65. Boundary Tests

Boundary tests must include:

empty hardware contract
single resource
large resource list
deep topology
large logical device set
parameterized resource
zero where semantically invalid
negative values where semantically invalid
very large representable quantities
nested hardware declarations
nested constraints
combined capability/resource constraints

No boundary test may accidentally establish an artificial maximum.

---

66. Scalability Tests

Scalability tests must verify that grammar design does not introduce artificial hardware limits.

Test dimensions include:

number of resources
number of capabilities
number of devices
number of topology relationships
number of endpoints
number of ports
number of constraints
number of target properties
number of accelerator declarations
number of logical qubits
number of hardware requirements
number of modules

The test suite may choose finite test values.

Those values are test parameters, not language limits.

---

67. Determinism Tests

Parsing the same source must not depend on:

- installed hardware;
- CPU count;
- GPU count;
- available RAM;
- device IDs;
- network topology;
- QPU availability;
- driver state.

The same source and language version must produce the same syntactic result.

---

68. Portability Tests

Hardware intent must be tested against abstract targets.

For example, one semantic source contract may be analyzed against:

embedded
CPU
GPU
FPGA
ASIC
QPU
simulator
heterogeneous
distributed

The grammar should remain unchanged.

Only downstream target feasibility changes.

---

69. Hard-Coding Audit

Every hardware grammar change requires a hard-coding audit.

Search for:

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

Also search for equivalent hidden restrictions.

Examples:

fixed repetition counts
fixed device enumerations
fixed resource arrays
fixed physical IDs
fixed topology sizes
fixed register widths
fixed memory capacities
fixed accelerator counts

Any discovered value must be classified as:

program semantics
test data
implementation limit
target capability
language limitation

Only the first four may be acceptable in the appropriate layer.

A universal language limitation is prohibited unless it is an unavoidable representation rule explicitly documented by the language specification.

---

70. Hardware Identifiers

The following must not become universal hardware semantics:

physical_qubit_0
gpu0
cpu3
fpga1
node7
memory_bank2
pci:...
/dev/...

Such identifiers may exist in explicit deployment/target-specific contracts where required.

They must never become the foundational hardware model.

---

71. Hardware Quantities

Quantities should be represented through the canonical expression system.

Examples:

n
required_memory
required_compute
latency_budget
bandwidth_requirement
qubit_count

The compiler evaluates or reasons about these values according to semantic rules.

The grammar does not need to know the final physical quantity.

---

72. Resource Availability

Resource availability is a runtime/target property.

Conceptually:

program requirement
       |
       v
target capability/resource inventory
       |
       v
feasibility analysis

The source language should remain valid independently of today's inventory.

---

73. Dynamic and Elastic Hardware

Where supported by semantics, hardware intent should allow:

- elastic resources;
- dynamic allocation;
- dynamic device availability;
- scaling;
- resource pools;
- distributed resources;
- changing target availability.

The grammar should express intent rather than fixed inventory.

---

74. Heterogeneous Computing

Hardware grammar must support heterogeneous intent.

A computation may require multiple capability classes:

CPU-class control
GPU-class tensor compute
FPGA-class streaming acceleration
quantum-class computation
distributed communication

The source should be able to describe relationships among these without hard-coding a specific physical machine.

---

75. Distributed Hardware

The hardware subsystem integrates with:

grammar/distributed/
grammar/networking/

Hardware topology may describe logical relationships.

Distributed semantics describe computation/distribution.

Networking describes communication.

The hardware grammar must not duplicate distributed execution semantics.

---

76. AI / Tensor Hardware

Hardware acceleration for AI must integrate with:

grammar/ai/
grammar/data/
grammar/classical/
grammar/hardware/

The hardware grammar may express capabilities such as:

tensor.compute
matrix.compute
accelerator.compute
high_bandwidth_memory

It must not encode a particular AI framework.

---

77. Embedded Hardware

Embedded execution is a target class, not a fixed machine.

The hardware grammar must not assume:

fixed flash
fixed RAM
fixed CPU
fixed register width

as universal properties.

The target capability/resource model determines actual feasibility.

---

78. Future Hardware

Future hardware must be representable without requiring a redesign of the language core.

This is achieved through:

qualified names
capabilities
resources
properties
generic contracts
dialects
extension points

rather than adding a keyword for every new device class.

---

79. Dialect Integration

Hardware-specific dialects must integrate through the repository dialect mechanism.

A dialect should identify:

name
version
syntax extensions
semantic extensions
AST mapping
IR mapping
compatibility
feature gates

A dialect must not silently redefine core Zamani semantics.

---

80. Interoperability

Hardware grammar may interoperate with:

HDL
QASM
QIR
LLVM-family representations
MLIR-family representations
WASM
foreign functions
vendor toolchains

but these are interoperability/lowering boundaries.

They are not competing Zamani semantic authorities.

---

81. Error Diagnostics

Hardware grammar and semantic integration must preserve source spans.

Diagnostics should identify:

source file
line
column
span
error category
primary message
related context
suggestion where appropriate

Diagnostics must be deterministic.

They should distinguish:

syntax error
name error
type error
resource error
capability error
constraint error
target incompatibility
resource infeasibility
deployment error

---

82. Error Layering

A syntax error belongs to parsing.

A missing capability belongs to semantic/target analysis.

Insufficient resources belong to resource feasibility analysis.

Unavailable runtime hardware belongs to runtime/deployment.

Do not report a runtime resource problem as a parser error.

---

83. Security Boundary

Hardware grammar must not become a privileged hardware-control escape hatch.

Parsing source must not:

- execute commands;
- access devices;
- access arbitrary files;
- invoke drivers;
- modify hardware;
- query privileged state.

Hardware execution belongs downstream behind explicit security boundaries.

---

84. Deterministic Grammar Requirement

The grammar must avoid unnecessary ambiguity.

Production validation must include:

- ambiguous alternatives;
- unreachable rules;
- duplicate rules;
- duplicate token concepts;
- left recursion where unsupported;
- precedence conflicts;
- keyword collisions;
- import conflicts.

---

85. Existing Token Vocabulary Integration

The current Rust lexer already contains broad Zamani vocabulary.

Hardware grammar must not assume a token exists merely because a hardware ".g4" file names it.

The implementation sequence is:

specification
      |
      v
lexical contract
      |
      v
canonical lexer token
      |
      v
grammar
      |
      v
parser

not:

hardware grammar invents token
      |
      v
hope lexer eventually supports it

This rule prevents later re-editing.

---

86. Existing "unsafe" Keyword

The current Rust lexer includes a "KeywordUnsafe" token.

That does not change the hardware implementation safety rule.

Hardware grammar work must not introduce or require unsafe Rust.

If the broader Zamani language ultimately retains an "unsafe" source-language construct, that is a separate language-semantics decision and must not be confused with Rust implementation safety.

For the hardware subsystem:

«No hardware grammar implementation requires unsafe Rust.»

---

87. Generated Parser Policy

Generated parser sources should be treated as build artifacts unless repository policy explicitly makes them tracked sources.

The source-of-truth remains:

grammar/*.g4

and the canonical specification.

Generated code must not be manually modified to repair grammar behavior.

---

88. Build Compatibility

The hardware grammar must remain compatible with:

Rust 1.97
Rust 1.97.1
Rust 2021

The implementation must not require newer Rust-only language features unless the repository baseline is intentionally raised through an explicit compatibility change.

---

89. Performance

Grammar design should avoid unnecessary pathological ambiguity and excessive backtracking.

Performance must be evaluated against representative source sizes.

However:

«Performance optimization must never introduce a semantic hardware capacity limit.»

For example, changing a parser algorithm to improve performance is acceptable.

Changing the language to allow only 1024 devices is not.

---

90. Memory Safety

No unsafe memory manipulation is required for hardware grammar.

Compiler data structures should use safe Rust abstractions.

Potentially large hardware descriptions must be represented through scalable data structures rather than fixed arrays.

---

91. Resource Representation

Prefer dynamic structures such as semantic collections rather than fixed-size structures.

Conceptually:

resources: collection
capabilities: collection
devices: collection
connections: collection
constraints: collection

The actual Rust representation belongs to implementation layers, but the grammar must not impose fixed cardinality.

---

92. Physical Limits Versus Representational Limits

There is an important distinction.

The language cannot promise mathematically infinite physical hardware.

It can provide an unbounded language model subject to implementation representation.

Therefore:

infinity

in POCO-REAF means:

«no artificial language-level finite hardware ceiling.»

Actual execution remains bounded by:

- available resources;
- representation;
- target capabilities;
- runtime environment;
- physical reality.

This distinction must remain explicit.

---

93. "From Atom to Everywhere"

The hardware architecture must support the conceptual spectrum:

atomic / nano-scale computation
        |
        v
embedded
        |
        v
single processor
        |
        v
multicore
        |
        v
GPU
        |
        v
FPGA
        |
        v
ASIC
        |
        v
accelerator
        |
        v
QPU
        |
        v
heterogeneous system
        |
        v
HPC
        |
        v
cluster
        |
        v
distributed/cloud
        |
        v
future computational substrates

The grammar must not need a separate language for each level.

---

94. Hardware Intent Example

Conceptually, a portable contract can look like:

hardware ComputeTarget<RequiredCompute> {
    requires resource.compute >= RequiredCompute;
    requires capability("tensor.compute");
    prefer accelerator("compute");
}

The same semantic intent could be considered against:

CPU
GPU
FPGA
ASIC
accelerator
heterogeneous system
distributed system
future target

The grammar does not choose the physical machine.

---

95. Quantum Hardware Example

Conceptually:

hardware QuantumTarget<RequiredQubits> {
    requires capability("quantum.compute");
    requires capability("quantum.measurement");
    requires resource.qubits >= RequiredQubits;
}

The compiler determines whether a target can satisfy the requirement.

The grammar does not establish a maximum number of qubits.

---

96. Topology Example

Conceptually:

requires topology(
    connectivity = required_connectivity,
    distance = required_distance
);

The grammar preserves the requirement.

The routing subsystem determines physical realization.

---

97. Memory Example

Conceptually:

requires memory >= required_memory;

The compiler determines whether memory requirements can be satisfied.

It may choose:

- local memory;
- accelerator memory;
- distributed memory;
- virtual memory;
- another target;
- another execution strategy.

The grammar does not encode a fixed memory capacity.

---

98. Capability Example

Conceptually:

requires capability("quantum.mid_circuit_measurement");

The source remains independent of a particular QPU.

Capability resolution occurs later.

---

99. Preference Example

Conceptually:

prefer accelerator("tensor.compute");

This does not force a physical accelerator.

If no such accelerator exists, downstream policy determines whether another implementation can satisfy the program.

---

100. Physical Realization Example

The downstream architecture may eventually perform:

logical resource
       |
       v
target resource
       |
       v
physical resource

For example:

logical qubit
       |
       v
logical QPU resource
       |
       v
physical qubit

or:

logical accelerator
       |
       v
GPU-class accelerator
       |
       v
physical GPU

This is not grammar responsibility.

---

101. Compile-Once Requirement

POCO-REAF requires the compiler architecture to preserve semantic meaning across targets.

The hardware grammar therefore MUST NOT encode backend-specific implementation decisions into the portable program.

The compiler may perform:

target selection
specialization
optimization
decomposition
routing
scheduling
resource allocation
lowering

without requiring source rewriting where semantics permit.

---

102. Compile-Time Versus Runtime Hardware

Some information is available during compilation.

Other information is only available at runtime.

The grammar must not force all hardware information to be known statically.

The architecture may support:

compile-time requirement
runtime capability
dynamic availability
late binding
deployment-time selection

as long as their semantic contracts are explicit.

---

103. Dynamic Resource Availability

A program may be compiled for an abstract target and later encounter different available resources.

The system should distinguish:

program requirement
target capability
current availability

This is necessary for:

- clusters;
- cloud systems;
- heterogeneous machines;
- QPU queues;
- elastic accelerators;
- dynamic distributed resources.

---

104. Hardware Contracts Must Be Declarative

Hardware grammar should be declarative.

Prefer:

requires ...
provides ...
supports ...
constrains ...
prefers ...
hints ...

over executable parser-level commands.

---

105. No Hidden Compiler Policy in Grammar

The grammar must not encode assumptions such as:

GPU is always preferred
CPU is always fallback
QPU always requires physical qubits
FPGA always means synthesis

Those are compiler policy decisions.

The language should expose explicit semantics.

---

106. No Vendor-Specific Grammar Explosion

A future technology must not require:

keyword vendorA
keyword vendorB
keyword vendorC

unless the language semantics genuinely require a dedicated keyword.

Most extension should use:

qualified names
capabilities
properties
dialects

---

107. Hardware and Classical Computing

Hardware intent must integrate naturally with classical computation.

A classical algorithm can request:

compute
memory
vectorization
tensor.compute
parallel execution
accelerator

without becoming a different language.

---

108. Hardware and Quantum Computing

Quantum programs can request:

quantum.compute
quantum.measurement
quantum.reset
quantum.dynamic_control
quantum.mid_circuit_measurement
quantum.connectivity
quantum reliability

without hard-coding a physical QPU.

---

109. Hardware and HDL

HDL constructs may ultimately lower into hardware realizations described through the hardware subsystem.

The boundary must remain:

HDL:
    structure/behavior

Hardware:
    realization intent/resource/capability

Compiler:
    lowering/synthesis planning

Backend:
    physical realization

---

110. Hardware and Distributed Computing

Hardware topology can express:

locality
connectivity
communication
placement

Distributed semantics express:

processes
actors
services
messages
replication
consistency

These must not be conflated.

---

111. Hardware and Networking

Networking owns network programming semantics.

Hardware owns hardware-level interconnect intent.

The two can interoperate without duplicating:

endpoint
protocol
routing
communication

ownership.

---

112. Hardware and Security

Hardware grammar can express security capabilities and requirements, but security semantics remain integrated with:

grammar/security/

Examples:

secure execution
trusted capability
confidential computation
cryptographic acceleration

must be represented through shared semantic models.

---

113. Hardware and AI

AI computation may request:

tensor.compute
matrix.compute
accelerator
high_bandwidth_memory
parallel execution

The hardware grammar provides target intent.

AI semantics remain in:

grammar/ai/

---

114. Hardware and Data

Data movement requirements may interact with:

memory
interconnect
bandwidth
latency
storage

but data semantics remain in:

grammar/data/

---

115. Compatibility

Hardware grammar evolution must preserve existing valid programs where possible.

Changes must classify:

compatible
source-compatible
AST-compatible
semantic-compatible
IR-compatible
breaking
deprecated
experimental

The repository's compatibility subsystem remains authoritative for version policy.

---

116. Deprecated Hardware Grammar

When a hardware file becomes redundant, do not immediately delete it.

Preferred sequence:

old grammar
    |
    v
mark deprecated
    |
    v
migrate references
    |
    v
migrate tests
    |
    v
validate imports
    |
    v
remove duplicate authority

This is particularly relevant to:

hardware-constraints.g4

because "constraints.g4" already claims canonical ownership.

---

117. Grammar-to-AST Traceability

Every public grammar rule must have a traceability record.

Conceptually:

hardwareResourceRequirement
        |
        +--> AST node
        |
        +--> semantic model
        |
        +--> resource analysis
        |
        +--> IR/lowering
        |
        +--> tests

No orphan grammar rules are permitted.

---

118. AST-to-Semantics Traceability

Every hardware AST node must identify:

semantic owner
validation rules
resource effects
capability effects
diagnostics
IR/lowering destination

---

119. Semantic-to-IR Traceability

Every semantic hardware concept must identify whether it:

maps directly to IR
maps to metadata
maps to target requirements
maps to compiler constraints
is consumed only during analysis
is lowered before IR

This prevents semantic concepts from disappearing silently.

---

120. Compiler Consumer Contract

Every hardware feature must state which compiler phase consumes it.

Possible consumers include:

name resolution
type checking
effect analysis
resource analysis
capability analysis
target analysis
optimization
placement
routing
scheduling
resilience
QEC
ZQN
lowering
deployment

---

121. Runtime Consumer Contract

If a hardware feature has runtime significance, it must identify the runtime consumer.

Examples:

dynamic capability
resource availability
device state
runtime selection
recovery
monitoring

If it has no runtime consumer, the feature must state that explicitly.

---

122. No Orphan Features

A feature is incomplete if it exists only in:

hardware/*.g4

without:

AST
semantic ownership
IR/lowering
tests

The feature must be marked:

PARTIAL

or:

PLANNED

rather than incorrectly marked stable.

---

123. Production Readiness Checklist: Architecture

The directory passes architecture review only when:

[ ] One hardware composition root exists
[ ] One complete-language composition root exists
[ ] No competing hardware language exists
[ ] Ownership is explicit
[ ] Dependencies are explicit
[ ] Dependency direction is acyclic
[ ] Generic syntax is reused
[ ] Canonical lexer is reused
[ ] Canonical expressions are reused
[ ] Canonical types are reused
[ ] AST mapping exists
[ ] Semantic mapping exists
[ ] IR mapping exists

---

124. Production Readiness Checklist: Scalability

[ ] No MAX_QUBITS
[ ] No MAX_CPUS
[ ] No MAX_GPUS
[ ] No MAX_FPGAS
[ ] No MAX_NODES
[ ] No MAX_MEMORY
[ ] No MAX_THREADS
[ ] No MAX_TENSOR_RANK
[ ] No MAX_REGISTER_WIDTH
[ ] No MAX_NETWORK_SIZE
[ ] No MAX_DEVICE_COUNT
[ ] No fixed device enumeration
[ ] No fixed topology size
[ ] No fixed memory capacity
[ ] No fixed register width
[ ] No fixed accelerator count
[ ] No fixed timeline count
[ ] No fixed port count
[ ] No fixed channel count

---

125. Production Readiness Checklist: Safety

[ ] Grammar is action-free
[ ] No embedded Rust actions
[ ] No embedded hardware execution
[ ] No hardware discovery during parsing
[ ] No filesystem access
[ ] No network access
[ ] No driver access
[ ] No runtime execution
[ ] No project-owned unsafe Rust
[ ] Rust 1.97 compatible
[ ] Rust 1.97.1 compatible
[ ] Rust 2021 compatible

---

126. Production Readiness Checklist: Quantum

[ ] Quantum hardware capability is supported
[ ] Quantum resource requirements are supported
[ ] Measurement capability is supported
[ ] Dynamic-circuit capability is supported
[ ] Mid-circuit measurement capability is supported
[ ] Classical feed-forward capability is supported
[ ] Abstract connectivity is supported
[ ] No fixed qubit limit exists
[ ] No fixed physical-qubit model exists
[ ] quantum::ir remains canonical
[ ] No duplicate quantum IR exists
[ ] QEC remains downstream
[ ] ZQN remains downstream

---

127. Production Readiness Checklist: HDL

[ ] HDL boundary is explicit
[ ] Hardware intent is separate from HDL behavior
[ ] Parameterized hardware is supported
[ ] Timing intent is supported
[ ] Memory intent is supported
[ ] Interface intent is supported
[ ] Synthesis intent has a downstream boundary
[ ] No fixed register width is imposed
[ ] No fixed FPGA capacity is imposed
[ ] No physical chip assumption is embedded

---

128. Production Readiness Checklist: Compiler

[ ] Resource analysis consumes hardware requirements
[ ] Capability analysis consumes hardware capabilities
[ ] Target analysis consumes target intent
[ ] Placement consumes placement intent
[ ] Routing consumes topology intent
[ ] Scheduling consumes timing intent
[ ] Resilience consumes reliability intent
[ ] QEC consumes quantum resilience requirements
[ ] ZQN consumes noise/resilience semantics
[ ] HAL consumes target realization
[ ] Runtime consumes runtime-relevant hardware metadata

---

129. Production Readiness Checklist: Tests

[ ] Positive tests
[ ] Negative tests
[ ] Boundary tests
[ ] Scalability tests
[ ] Determinism tests
[ ] Portability tests
[ ] Compatibility tests
[ ] AST tests
[ ] Semantic tests
[ ] IR tests
[ ] Compiler integration tests
[ ] HAL integration tests
[ ] Runtime integration tests
[ ] Cross-domain tests
[ ] Hard-coding tests

---

130. Cross-Domain Integration Matrix

Hardware must integrate with:

Domain| Hardware relationship
"core/"| names, attributes, common declarations
"types/"| resource/capability types
"expressions/"| quantities and predicates
"declarations/"| hardware declarations
"statements/"| hardware intent statements
"functions/"| hardware-aware functions
"modules/"| hardware module boundaries
"effects/"| hardware-related effects
"memory/"| generic memory semantics
"concurrency/"| parallel resource intent
"classical/"| CPU/general computation
"quantum/"| quantum computation
"hybrid/"| quantum-classical execution
"hdl/"| hardware structure/behavior
"resources/"| generic resource semantics
"compile/"| target selection/lowering
"execution/"| runtime policies
"distributed/"| distributed hardware
"ai/"| accelerator/tensor requirements
"data/"| data movement/storage
"networking/"| interconnect/network relationships
"security/"| trusted/secure hardware capabilities
"interoperability/"| external hardware formats
"dialects/"| future/vendor extensions
"validation/"| conformance
"compatibility/"| versioning and migration
"tests/"| production verification

---

131. Independent Implementation Order

To minimize re-editing, implement hardware files in dependency order.

Stage 1 — shared contracts

Complete first:

grammar/DESIGN.md
grammar/specification/
grammar/spec/
grammar/lexer/
grammar/core/
grammar/expressions/
grammar/types/

These define the foundations.

Stage 2 — hardware leaf contracts

Then complete:

resources.g4
capabilities.g4
targets.g4
devices.g4
constraints.g4

These establish the core hardware intent vocabulary.

Stage 3 — relationship contracts

Then:

topology.g4
placement.g4
memory.g4
interconnect.g4
timing.g4

Stage 4 — target classes

Then:

accelerators.g4
cpu.g4
gpu.g4
fpga.g4
asic.g4
qpu.g4
quantum-device.g4

Stage 5 — physical-quality intent

Then:

power.g4
thermal.g4
reliability.g4
calibration.g4

Stage 6 — deployment

Then:

negotiation.g4
deployment.g4

Stage 7 — hardware composition

Only after leaf contracts are stable:

hardware.g4

Stage 8 — complete-language composition

Then integrate through:

grammar/Zamani.g4

Stage 9 — implementation conformance

Then update/validate:

grammar/grammar.md

Stage 10 — full validation

Then run:

lexer
parser
AST
semantic
IR
compiler
HAL
runtime
cross-domain
scalability
compatibility

tests.

---

132. Why "hardware.g4" Comes Last

"hardware.g4" is the composition point.

If it is implemented before its leaf contracts are settled, it becomes a source of repeated edits.

The independent-first strategy is therefore:

leaf contract
      |
      v
leaf implementation
      |
      v
leaf tests
      |
      v
hardware composition
      |
      v
Zamani composition

This satisfies the requirement that an individual file can be considered complete without repeatedly reopening it after unrelated files change.

---

133. Definition of "Done" for an Individual File

A file is complete only when:

Purpose
        defined

Ownership
        defined

Dependencies
        defined

Public syntax
        defined

AST contract
        defined

Semantic contract
        defined

IR contract
        defined

Compiler consumers
        defined

Runtime consumers
        defined

Diagnostics
        defined

Compatibility
        defined

Positive tests
        present

Negative tests
        present

Boundary tests
        present

Scalability tests
        present

Determinism tests
        present

Hard-coding audit
        passed

Only then is the file marked production-ready.

---

134. Definition of "Done" for a Hardware Feature

A feature is complete only when:

Specification
    +
Lexer
    +
Parser
    +
AST
    +
Semantic Analysis
    +
IR
    +
Compiler
    +
Target/HAL
    +
Runtime where applicable
    +
Tests

are all accounted for.

If one layer is missing:

PARTIAL

or:

PLANNED

must be used.

---

135. Definition of Hardware Grammar Production Readiness

"grammar/hardware/" is production-ready only when:

one language
one lexer
one parser architecture
one AST architecture
one semantic architecture
one canonical IR architecture
one hardware composition root
no duplicate hardware authority
no fixed hardware capacity
no physical-device dependency
no vendor lock-in
no parser-time hardware discovery
no unsafe implementation
complete AST traceability
complete semantic traceability
complete IR traceability
complete compiler integration
complete target integration
complete HAL integration
complete test coverage

all hold.

---

136. Repository-Wide Conformance

Hardware grammar production readiness must be verified against:

grammar/DESIGN.md
grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/spec/
grammar/specification/
grammar/core/
grammar/types/
grammar/expressions/
grammar/resources/
grammar/quantum/
grammar/hdl/
grammar/compile/
grammar/execution/
grammar/validation/
grammar/compatibility/
src/lexer.rs
src/parser.rs
src/frontend/ast/
src/quantum/

The hardware subsystem cannot declare itself complete while contradicting a higher-level contract.

---

137. Current Integration Issues That Must Be Corrected

The existing repository inspection identifies several classes of issues that must be resolved.

137.1 Token vocabulary divergence

Some existing hardware grammar material uses vocabulary that does not directly align with the current canonical Rust lexer.

Resolution:

canonical lexer vocabulary
        |
        v
ANTLR grammar vocabulary
        |
        v
hardware grammar

No second token vocabulary.

---

137.2 Duplicate constraint ownership

Both:

constraints.g4
hardware-constraints.g4

currently claim hardware constraint responsibility.

Resolution:

constraints.g4
    |
    v
canonical implementation

hardware-constraints.g4
    |
    v
compatibility/deprecation

until safe retirement is possible.

---

137.3 Composition authority

Some hardware files document relationships to parser structures that must be reconciled with the actual canonical "grammar/Zamani.g4" architecture.

Resolution:

«"grammar/Zamani.g4" remains the only complete-language root.»

---

137.4 Quantum authority

Some hardware documentation references quantum-device behavior while the repository architecture establishes:

quantum::ir

as the canonical quantum IR.

Resolution:

«hardware syntax may describe quantum hardware capabilities, but quantum program semantics continue through "quantum::ir".»

---

137.5 Documentation status

Documentation must not call a feature production-ready merely because ".g4" syntax exists.

The status must reflect:

SPECIFIED
IMPLEMENTED
PARTIAL
PLANNED
DEPRECATED
EXPERIMENTAL
STABLE

according to actual repository integration.

---

138. No Unnecessary Renames

Existing major filenames should be retained.

Do not rename:

hardware.g4
resources.g4
capabilities.g4
constraints.g4
targets.g4
devices.g4
topology.g4
placement.g4
memory.g4
interconnect.g4
...

merely for aesthetics.

If duplicate ownership must be removed, prefer:

migration
deprecation
reference redirection
test migration

before deletion.

---

139. Expansion Rule

The objective is to expand existing repository functionality, not replace working features with an unrelated architecture.

For every existing hardware feature:

inspect
        |
        v
classify ownership
        |
        v
preserve useful behavior
        |
        v
normalize vocabulary
        |
        v
connect AST
        |
        v
connect semantics
        |
        v
connect IR
        |
        v
test

Only redundant or contradictory implementations should be consolidated.

---

140. What Must Never Be Added

Never add universal language assumptions such as:

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

Never make these the universal language model:

physical qubit IDs
GPU IDs
CPU IDs
FPGA coordinates
memory-bank IDs
PCI addresses
device handles
driver handles

Never add:

hardware discovery
runtime execution
routing implementation
scheduling implementation
calibration implementation
QEC implementation
ZQN implementation

to parser grammar.

Never create another quantum IR.

Never create another complete Zamani grammar.

Never turn every vendor feature into a core keyword.

---

141. What Should Be Preferred

Prefer:

requires ...
provides ...
supports ...
prefer ...
hint ...
property ...
target ...
capability(...)
resource(...)
topology(...)
placement(...)

Prefer:

qualified names
generic parameters
symbolic quantities
semantic constraints
capability contracts
resource contracts
target-independent intent

Prefer downstream resolution for:

device selection
resource allocation
physical mapping
routing
scheduling
calibration
deployment

---

142. Hardware Contract Example

Conceptually:

hardware ComputeTarget<N> {
    requires resource.compute >= N;
    requires capability("tensor.compute");
    prefer accelerator("compute");
}

This describes requirements.

It does not say:

use GPU #3

or:

use 16 CPU cores

---

143. Hardware + Quantum Example

Conceptually:

hardware QuantumTarget<N> {
    requires capability("quantum.compute");
    requires capability("quantum.measurement");
    requires resource.qubits >= N;
}

The actual target may be:

QPU
simulator
distributed quantum system
future quantum target

subject to semantic feasibility.

---

144. Hardware + HDL Example

Conceptually:

hardware AcceleratorTarget<W> {
    requires capability("programmable.logic");
    requires resource.compute >= required_compute;
    requires resource.memory >= required_memory;
}

HDL can describe the implementation structure separately.

---

145. Hardware + Distributed Example

Conceptually:

hardware DistributedTarget {
    requires capability("distributed.communication");
    requires topology(...);
    requires resource.compute >= required_compute;
}

The number of nodes is not hard-coded.

---

146. Hardware + AI Example

Conceptually:

hardware TensorTarget {
    requires capability("tensor.compute");
    requires resource.memory >= required_memory;
    prefer accelerator("matrix.compute");
}

This remains framework-independent.

---

147. Hardware + Security Example

Conceptually:

hardware SecureTarget {
    requires capability("secure.compute");
    requires capability("trusted.execution");
}

Security semantics remain integrated with the security subsystem.

---

148. Hardware + Portability

A hardware contract should be portable across target classes.

For example:

same source
       |
       +--> embedded
       +--> CPU
       +--> GPU
       +--> FPGA
       +--> ASIC
       +--> QPU
       +--> simulator
       +--> cluster
       +--> cloud
       +--> future target

Target feasibility may differ.

Source semantics should not.

---

149. Hardware + Resource Negotiation

The intended model is:

program
   |
   v
requirements
   |
   v
target capabilities
   |
   v
resource feasibility
   |
   v
realization strategy

The parser remains independent of the result.

---

150. Hardware + Runtime Availability

A target may satisfy the static contract but become unavailable at runtime.

Therefore:

semantic validity
        !=
runtime availability

The runtime may return a documented outcome such as:

RETRY
RECOVER
ESCALATE
REJECT

according to the runtime/resilience architecture.

---

151. Future-Proofing

Future hardware must be introduced through extension rather than grammar replacement.

A new hardware class should first answer:

Does existing capability syntax express it?
Does existing resource syntax express it?
Does existing target syntax express it?
Does existing property syntax express it?
Does an existing dialect mechanism express it?
Does the semantic model already support it?

Only if the answer is no should new grammar syntax be introduced.

---

152. New Hardware Feature Review

Before adding a new hardware feature:

1. Search existing grammar ownership.
2. Search "grammar/spec/".
3. Search "grammar/specification/".
4. Search "grammar/core/".
5. Search "grammar/resources/".
6. Search "grammar/quantum/".
7. Search "grammar/hdl/".
8. Search "grammar/compile/".
9. Search "grammar/execution/".
10. Search existing capabilities.
11. Search existing resources.
12. Search existing target classes.
13. Search existing dialect mechanisms.
14. Determine whether existing syntax is sufficient.
15. Define semantic ownership.
16. Define AST representation.
17. Define IR/lowering.
18. Define compiler consumers.
19. Define runtime consumers.
20. Define tests.
21. Perform hard-coding audit.
22. Check compatibility.
23. Check cross-domain integration.
24. Only then add syntax if necessary.

---

153. Hardware Change Review Questions

Every hardware grammar change must answer:

1. What syntax changed?
2. Which file owns it?
3. Why does that file own it?
4. Does another file already own it?
5. Does it require a new keyword?
6. Could a qualified name represent it?
7. Could a capability represent it?
8. Could a resource represent it?
9. Could a property represent it?
10. Does it introduce a physical-machine assumption?
11. Does it introduce a fixed capacity?
12. Does it introduce physical identifiers?
13. Does it change the AST?
14. Does it change semantic analysis?
15. Does it change IR?
16. Does it affect quantum::ir?
17. Does it affect QEC?
18. Does it affect ZQN?
19. Does it affect routing?
20. Does it affect scheduling?
21. Does it affect optimization?
22. Does it affect HAL?
23. Does it affect runtime?
24. Does it affect compatibility?
25. Does it preserve POCO-REAF?
26. Does it preserve scalability?
27. Does it preserve deterministic parsing?
28. Are positive tests present?
29. Are negative tests present?
30. Are boundary tests present?
31. Are scalability tests present?
32. Has the hard-coding audit passed?

---

154. Validation Pipeline

The complete validation pipeline is:

hardware specification
        |
        v
feature contract
        |
        v
lexical contract
        |
        v
ANTLR grammar
        |
        v
ANTLR validation
        |
        v
Rust lexer conformance
        |
        v
Rust parser conformance
        |
        v
AST coverage
        |
        v
semantic coverage
        |
        v
IR coverage
        |
        v
compiler coverage
        |
        v
HAL coverage
        |
        v
runtime coverage
        |
        v
cross-domain tests
        |
        v
scalability tests
        |
        v
compatibility tests
        |
        v
production

---

155. Production Gate

The hardware subsystem must not be marked production-ready until all required gates pass.

Architecture

[ ] Ownership is unambiguous
[ ] Composition is unambiguous
[ ] Dependencies are acyclic
[ ] No duplicate authority exists

Syntax

[ ] Grammar is valid
[ ] Grammar is deterministic
[ ] No unnecessary ambiguity
[ ] Canonical lexer vocabulary used
[ ] Canonical names used
[ ] Canonical expressions used
[ ] Canonical types used

Semantics

[ ] AST contracts complete
[ ] Semantic contracts complete
[ ] Resource contracts complete
[ ] Capability contracts complete
[ ] Target contracts complete
[ ] Constraint contracts complete

IR

[ ] IR mapping complete
[ ] No duplicate quantum IR
[ ] Hardware metadata has defined ownership
[ ] Lowering boundary defined

Compiler

[ ] Resource analysis integrated
[ ] Capability analysis integrated
[ ] Target analysis integrated
[ ] Placement integrated
[ ] Routing integrated
[ ] Scheduling integrated
[ ] Resilience integrated
[ ] QEC integration defined
[ ] ZQN integration defined
[ ] HAL integration defined

Safety

[ ] No unsafe Rust introduced
[ ] Grammar is action-free
[ ] No runtime execution in grammar
[ ] No hardware discovery in parser

Scalability

[ ] No artificial hardware capacity limits
[ ] No fixed qubit ceiling
[ ] No fixed CPU ceiling
[ ] No fixed GPU ceiling
[ ] No fixed FPGA ceiling
[ ] No fixed node ceiling
[ ] No fixed memory ceiling
[ ] No fixed thread ceiling
[ ] No fixed tensor-rank ceiling
[ ] No fixed register-width ceiling
[ ] No fixed network-size ceiling
[ ] No fixed device-count ceiling

Tests

[ ] Positive
[ ] Negative
[ ] Boundary
[ ] Scalability
[ ] Determinism
[ ] Portability
[ ] Compatibility
[ ] AST
[ ] Semantic
[ ] IR
[ ] Compiler
[ ] HAL
[ ] Runtime
[ ] Cross-domain

---

156. Final Hardware Architecture

The target architecture is:

                         ZAMANI SOURCE
                               |
                               v
                       Canonical Lexer
                               |
                               v
                      Canonical Parser
                               |
                               v
                         Frontend AST
                               |
             +-----------------+-----------------+
             |                 |                 |
             v                 v                 v
          Classical         Quantum             HDL
             |                 |                 |
             +-----------------+-----------------+
                               |
                               v
                     Hardware Intent Model
                               |
        +----------------------+----------------------+
        |                      |                      |
        v                      v                      v
    Resources             Capabilities          Requirements
        |                      |                      |
        +----------------------+----------------------+
                               |
        +----------------------+----------------------+
        |                      |                      |
        v                      v                      v
   Constraints            Preferences              Hints
        |                      |                      |
        +----------------------+----------------------+
                               |
                               v
                   Canonical Semantic Model
                               |
                               v
                     Canonical Compiler IR
                               |
          +--------------------+--------------------+
          |                    |                    |
          v                    v                    v
      Classical           quantum::ir         HDL/Hardware
          |                    |                    |
          +--------------------+--------------------+
                               |
                               v
                         Optimization
                               |
          +--------------------+--------------------+
          |                    |                    |
          v                    v                    v
       Placement            Routing             Scheduling
                               |
                               v
                           Resilience
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
          +---------+----------+----------+----------+
          |         |                     |          |
          v         v                     v          v
         CPU       GPU                  FPGA        QPU
          |         |                     |          |
          +---------+----------+----------+----------+
                               |
                               v
                       Future Targets

---

157. Fundamental Invariants

The following are non-negotiable:

ONE Zamani language
ONE canonical lexical architecture
ONE canonical language composition root
ONE hardware composition root
ONE domain-neutral AST architecture
ONE canonical quantum IR
NO artificial hardware limits
NO physical-device dependency in portable syntax
NO vendor lock-in
NO parser-time hardware discovery
NO runtime execution in grammar
NO unsafe Rust
NO duplicate hardware grammar authority
NO duplicate resource authority
NO duplicate capability authority
NO duplicate constraint authority
NO duplicate quantum IR
NO hidden physical topology
NO fixed device enumeration
NO fixed memory assumptions
NO fixed register assumptions
NO fixed qubit assumptions
NO fixed CPU/GPU/FPGA assumptions

And:

YES declarative hardware intent
YES resource requirements
YES capability requirements
YES constraints
YES preferences
YES hints
YES generic parameters
YES symbolic quantities
YES qualified names
YES extensibility
YES deterministic parsing
YES source spans
YES diagnostics
YES AST traceability
YES semantic traceability
YES IR traceability
YES compiler integration
YES HAL integration
YES runtime integration
YES quantum integration
YES HDL integration
YES distributed integration
YES heterogeneous computing
YES scalability
YES POCO-REAF

---

158. Final Definition of the Hardware Grammar

The Zamani hardware grammar must describe:

WHAT hardware relationship is required
WHAT resources are required
WHAT capabilities are required
WHAT constraints apply
WHAT properties matter
WHAT topology matters
WHAT placement intent exists
WHAT target classes are compatible
WHAT implementation preferences exist

It must not permanently describe:

WHICH physical CPU
WHICH physical GPU
WHICH physical FPGA
WHICH physical ASIC
WHICH physical QPU
WHICH physical qubit
WHICH physical core
WHICH physical memory bank
WHICH physical address
WHICH physical node
WHICH physical device

Those decisions belong downstream.

---

159. Final POCO-REAF Principle

The complete hardware architecture exists to enable:

                 PROGRAM ONCE
                       |
                       v
                STABLE SEMANTICS
                       |
                       v
               COMPILE / ANALYZE
                       |
                       v
             DISCOVER CAPABILITIES
                       |
                       v
             RESOLVE RESOURCES
                       |
                       v
                  OPTIMIZE
                       |
          +------------+------------+
          |            |            |
          v            v            v
       ROUTING     SCHEDULING    RESILIENCE
                                      |
                                      v
                                     ZQN
                                      |
                                      v
                                     HAL
                                      |
          +------------+---------------+-------------+
          |            |               |             |
          v            v               v             v
         CPU          GPU             FPGA          QPU
          |            |               |             |
          +------------+---------------+-------------+
                                      |
                                      v
                               FUTURE TARGETS

The programmer writes the semantic computation once.

The hardware subsystem expresses what that computation requires from its realization.

The compiler and runtime determine how those requirements are satisfied.

This is the mechanism by which Zamani can pursue:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever»

while remaining honest about actual resource availability.

---

160. Final Implementation Rule

The repository must not attempt to make "grammar/hardware/" production-ready by continually expanding "hardware.g4".

Instead:

complete contracts independently
        |
        v
complete leaf grammars independently
        |
        v
complete AST mappings
        |
        v
complete semantic mappings
        |
        v
complete IR mappings
        |
        v
complete compiler consumers
        |
        v
complete tests
        |
        v
compose hardware.g4
        |
        v
compose grammar/Zamani.g4
        |
        v
validate against lexer/parser
        |
        v
validate entire compiler pipeline

This is the required implementation strategy.

---

161. Final Repository Rule

Existing repository functionality must be:

inspected
preserved where valid
expanded where incomplete
normalized where inconsistent
deprecated where duplicated
tested where untested
integrated where isolated

It must not be discarded merely because the architecture is being strengthened.

Existing filenames should remain unless there is a concrete technical reason to change them.

In particular:

grammar/hardware/hardware.g4
grammar/hardware/resources.g4
grammar/hardware/capabilities.g4
grammar/hardware/constraints.g4
grammar/hardware/targets.g4
grammar/hardware/devices.g4
grammar/hardware/topology.g4
grammar/hardware/placement.g4

and the other existing hardware files should be treated as the implementation surface to converge, not as disposable prototypes.

---

162. Completion Statement

"grammar/hardware/README.md" is complete when it serves as the stable contract against which every hardware grammar file can be independently implemented, integrated, tested, and declared complete.

The implementation is complete only when the actual repository satisfies this contract.

In particular, documentation must never outrun implementation.

The production hardware subsystem must ultimately be:

target-independent
resource-aware
capability-aware
constraint-aware
scalable
deterministic
safe
extensible
vendor-neutral
quantum-compatible
HDL-compatible
compiler-integrated
HAL-integrated
runtime-integrated
future-compatible

while preserving one Zamani language and one coherent semantic architecture.

The final invariant is:

Zamani Source
      |
      v
Portable Hardware Intent
      |
      v
Semantic Analysis
      |
      v
Canonical IR
      |
      v
Optimization
      |
      v
Placement / Routing / Scheduling / Resilience
      |
      v
ZQN
      |
      v
HAL
      |
      v
Target Realization

Therefore:

«Hardware grammar defines intent; the compiler defines realization; the target defines availability; the runtime defines execution.»

And:

«No artificial hardware ceiling belongs in the Zamani language.»

And:

«"quantum::ir" remains the canonical quantum IR.»

And:

«"grammar/Zamani.g4" remains the canonical complete-language composition root.»

And:

«"grammar/hardware/hardware.g4" remains the canonical hardware-domain composition root.»

And:

«Rust 1.97 / 1.97.1 and Rust 2021 remain the implementation baseline, with no project-owned "unsafe" Rust.»

And ultimately:

«Zamani — Program Once, Compile Once, Run Everywhere, Anywhere, Forever — from the smallest meaningful computation to arbitrarily large computation, subject only to actual semantics, resources, capabilities, and available targets.»