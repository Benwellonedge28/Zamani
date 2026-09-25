Worked for 32s

I inspected the current main branch and the actual grammar/hardware/ contents, including hardware.g4, resources.g4, capabilities.g4, constraints.g4, targets.g4, devices.g4, topology.g4, placement.g4, accelerators.g4, cpu.g4, gpu.g4, fpga.g4, asic.g4, qpu.g4, quantum-device.g4, memory.g4, interconnect.g4, timing.g4, power.g4, thermal.g4, reliability.g4, calibration.g4, negotiation.g4, deployment.g4, and the repository-level DESIGN.md, Zamani.g4, and lexer architecture.

There is an important correction to make to the existing README: it currently describes the subsystem as essentially production-ready, while the actual repository still has integration blockers—most notably the current hardware.g4 using ZamaniTokens/K_* vocabulary rather than cleanly aligning with the canonical lexer architecture, and the presence of overlapping/legacy ownership such as hardware-constraints.g4. The README below therefore defines production architecture and completion criteria, without falsely claiming that every current implementation file already satisfies them.

Here is the complete replacement for grammar/hardware/README.md:

# Zamani Hardware Grammar

**Path:** `grammar/hardware/`  
**Primary composition grammar:** `grammar/hardware/hardware.g4`  
**Language:** Zamani Universal Computing Language  
**Grammar technology:** ANTLR  
**Compiler/frontend baseline:** Rust 1.97 / Rust 1.97.1  
**Rust edition:** 2021  
**Rust safety policy:** production compiler implementation uses safe Rust; no `unsafe`  
**Primary portability objective:** `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever` (POCO-REAF)  
**Scalability objective:** from the smallest supported computation to arbitrarily large computations, subject only to program semantics, representational limits, declared requirements, compiler/runtime resources, and target availability  
**Status:** Normative hardware-subsystem architecture and completion contract

---

# 1. Purpose

`grammar/hardware/` defines the Zamani source-language syntax required to describe **hardware intent**.

It provides the language boundary for expressing:

- hardware declarations;
- abstract hardware composition;
- hardware interfaces;
- ports and connections;
- resources;
- capabilities;
- requirements;
- constraints;
- preferences;
- hints;
- targets;
- topology;
- placement intent;
- mappings;
- device classes;
- accelerators;
- CPU-class targets;
- GPU-class targets;
- FPGA-class targets;
- ASIC-class targets;
- QPU-class targets;
- quantum-device capabilities;
- memory contracts;
- compute contracts;
- interconnect contracts;
- timing intent;
- power intent;
- thermal constraints;
- reliability requirements;
- calibration metadata;
- deployment intent;
- hardware negotiation;
- future hardware extension points.

The subsystem exists to allow a Zamani program to express **what hardware relationship a computation requires** without permanently binding the program to a particular physical machine.

The fundamental rule is:

> Hardware syntax describes portable semantic intent.  
> Hardware realization is performed downstream.

This subsystem is therefore one of the foundations required for:

> **Program Once → Compile Once → Run Everywhere → Anywhere → Forever**

and the broader Zamani objective:

> **From Atom to Everywhere.**

---

# 2. Production Status Clarification

This README is the **normative architecture and completion contract** for `grammar/hardware/`.

It does **not** claim that every existing `.g4` file in this directory is already production-complete.

The current repository contains substantial hardware grammar work, but production readiness requires repository-wide convergence.

In particular, the current implementation contains integration issues that must be resolved before the subsystem can truthfully be considered complete.

Known categories include:

- grammar/token-vocabulary convergence;
- duplicate ownership;
- legacy grammar names;
- composition-root cleanup;
- specialized grammar import normalization;
- expression/type reuse;
- AST traceability;
- semantic-model traceability;
- IR traceability;
- repository-wide conformance;
- negative testing;
- scalability testing;
- hard-coding audits;
- generated-parser verification.

Therefore:

> **This document defines what "done" means.**

It must not be used to mark an individual grammar implementation complete merely because ANTLR accepts its syntax.

---

# 3. Architectural Position

The hardware grammar is located at the syntax layer.

The complete architecture is:

```text
                    Zamani Source
                         |
                         v
                Canonical Zamani Lexer
                         |
                         v
                     Token Stream
                         |
                         v
                 Canonical Zamani Parser
                         |
                         v
                    Frontend AST
                         |
             +-----------+-----------+
             |           |           |
             v           v           v
          Names       Types       Expressions
             |           |           |
             +-----------+-----------+
                         |
                         v
                 Semantic Analysis
                         |
          +--------------+---------------+
          |              |               |
          v              v               v
       Resources     Capabilities     Requirements
          |              |               |
          +--------------+---------------+
                         |
                         v
                 Semantic Hardware Model
                         |
                         v
                  Canonical Compiler IR
                         |
             +-----------+------------+
             |           |            |
             v           v            v
         Classical   quantum::ir    HDL/Hardware
             |           |            |
             +-----------+------------+
                         |
                         v
                    Optimization
                         |
             +-----------+------------+
             |           |            |
             v           v            v
          Routing    Scheduling    Resilience
                         |
                         v
                        ZQN
                         |
                         v
                        HAL
                         |
                         v
                 Target Lowering
                         |
        +--------+-------+--------+--------+
        |        |       |        |        |
        v        v       v        v        v
       CPU      GPU     FPGA     QPU     Future
                                             targets

The hardware grammar is upstream of:

resource resolution;

target discovery;

physical placement;

routing;

scheduling;

synthesis;

calibration;

resilience;

runtime dispatch;

device drivers.


It must not absorb those responsibilities.


---

4. Core Architectural Invariant

The following distinction is permanent:

Grammar
    !=
AST
    !=
Semantic Model
    !=
IR
    !=
Compiler Backend
    !=
HAL
    !=
Runtime
    !=
Physical Hardware

The grammar recognizes source syntax.

The AST preserves source structure.

Semantic analysis determines meaning and validity.

IR represents compiler-level semantics.

Backends perform target-specific lowering.

The HAL exposes target capabilities and runtime state.

The runtime executes the resulting program.

Physical hardware is the final realization.

No layer may silently become another layer.


---

5. Ownership

5.1 grammar/hardware/ owns

The subsystem owns source syntax for:

hardware contracts;

hardware declarations;

abstract hardware composition;

resources;

capabilities;

requirements;

constraints;

preferences;

hints;

targets;

topology requirements;

placement intent;

mappings;

device contracts;

accelerator contracts;

compute contracts;

memory contracts;

interconnect contracts;

timing contracts;

power contracts;

thermal contracts;

reliability contracts;

calibration intent;

deployment intent;

hardware extension points.



---

5.2 grammar/hardware/ does not own

It does not own:

lexical tokenization;

identifier definitions;

general expression syntax;

general type syntax;

general statement syntax;

canonical AST implementation;

type checking;

capability discovery;

resource allocation;

target discovery;

physical device enumeration;

physical addresses;

device drivers;

calibration algorithms;

synthesis algorithms;

place-and-route algorithms;

routing algorithms;

scheduling algorithms;

optimization algorithms;

QEC algorithms;

ZQN implementation;

quantum circuit semantics;

quantum::ir;

runtime execution;

deployment orchestration.



---

6. Single Authority Rule

There must be exactly one canonical Zamani language.

The repository may contain:

ANTLR grammar;

Rust lexer;

Rust parser;

AST;

semantic implementation;

generated documentation;

compatibility specifications;

IDE grammars;

syntax highlighting;

historical design material;


but these must all describe the same language.

They must not become competing language definitions.

The authority hierarchy is:

grammar/DESIGN.md
        |
        v
grammar/specification/
        |
        v
grammar/spec/
        |
        v
canonical lexer contract
        |
        v
canonical Zamani grammar
        |
        v
Rust lexer/parser
        |
        v
AST contract
        |
        v
semantic contract
        |
        v
canonical IR contracts
        |
        v
implementation conformance

grammar/Zamani-Grammar.md is historical/extended design material and is not an independent authority.

grammar/grammar.md is an implementation-conformance reference and is not an independent authority.


---

7. Relationship to grammar/DESIGN.md

grammar/DESIGN.md is the repository-wide normative architecture.

This README specializes that architecture for hardware.

If this README conflicts with grammar/DESIGN.md, the repository-wide design contract takes precedence and this document must be corrected.

Hardware-specific decisions must remain consistent with:

canonical lexical architecture;

canonical AST architecture;

canonical semantic architecture;

canonical IR architecture;

quantum::ir;

resource/capability separation;

POCO-REAF;

no artificial hardware limits.



---

8. Relationship to grammar/Zamani.g4

grammar/Zamani.g4 is the canonical language composition root.

It must ultimately dispatch into the hardware grammar.

The intended relationship is:

Zamani.g4
    |
    +--> classical
    |
    +--> quantum
    |
    +--> hybrid
    |
    +--> HDL
    |
    +--> hardware
    |
    +--> distributed
    |
    +--> AI/data
    |
    +--> networking
    |
    +--> security
    |
    +--> other domains

hardware.g4 must not become a second Zamani root grammar.

The hardware subsystem must expose a stable hardware entry point that the canonical root can consume.


---

9. Relationship to grammar/grammar.md

grammar/grammar.md describes what the current implementation accepts.

It should ultimately be generated or validated from:

Specification
+
Canonical grammar
+
Lexer implementation
+
Parser implementation
+
AST implementation

It must distinguish at minimum:

SPECIFIED
LEXER_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
BACKEND_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL
DEPRECATED

Hardware syntax must not be considered implemented merely because it exists in hardware.g4.


---

10. Relationship to grammar/Zamani-Grammar.md

Zamani-Grammar.md may contain:

future hardware concepts;

experimental hardware ideas;

universal-computing concepts;

historical grammar designs;

future accelerators;

future computational substrates;

Sankofa concepts;

MTS concepts;

nano concepts;

AI concepts;

proposed hardware paradigms.


It is not automatically accepted language syntax.

The promotion path is:

Zamani-Grammar.md
        |
        v
Feature proposal
        |
        v
Semantic design
        |
        v
AST contract
        |
        v
Canonical grammar
        |
        v
Lexer implementation
        |
        v
Parser implementation
        |
        v
Semantic implementation
        |
        v
IR contract
        |
        v
Compiler/backend integration
        |
        v
Tests
        |
        v
Stable


---

11. POCO-REAF

The hardware subsystem exists to protect:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

POCO-REAF means that the program describes semantic intent independently of the accidental characteristics of the machine on which it eventually executes.

A Zamani program should be able to express:

what computation is required
what capabilities are required
what resources are required
what constraints must hold
what preferences are desirable
what relationships between resources matter

without requiring the source program to permanently encode:

which CPU
which GPU
which QPU
which FPGA
which ASIC
which physical qubit
which memory bank
which physical core
which device ID
which physical address


---

12. POCO-REAF Does Not Mean "Every Program Runs Everywhere"

Portability does not mean that every target has sufficient resources.

These are distinct:

source validity
    !=
semantic validity
    !=
target compatibility
    !=
resource feasibility
    !=
runtime availability

For example, a program may semantically require a quantity of quantum resources that a particular QPU cannot currently provide.

The compiler may then:

select another target;

distribute the computation;

use logical resources;

transform the computation;

decompose operations;

route operations;

schedule operations;

apply resilience mechanisms;

use a simulator;

defer execution;

reject the target with a precise diagnostic.


The grammar must not silently change the program's semantics merely because one target is smaller.


---

13. Absolute Scalability Rule

There are no grammar-level hardware capacity limits.

The subsystem must never impose universal limits such as:

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

Nor may equivalent restrictions be hidden through:

bounded grammar repetitions;

finite device enumerations;

fixed physical identifiers;

fixed topology sizes;

fixed resource arrays;

fixed machine-specific alternatives;

fixed register counts;

fixed memory capacities;

fixed accelerator counts.


The grammar may parse arbitrary finite source structures supported by the parser implementation.

Actual implementation limits are implementation/resource concerns, not language semantics.


---

14. Program Constants Are Not Hardware Limits

The rule does not prohibit normal program values.

This is valid:

let n = 1024;
allocate n resources;

because 1024 is program data.

This is not acceptable as a universal language limitation:

MAX_QUBITS = 1024

when it means that Zamani can never represent a larger computation.

Similarly:

Tensor<T, 1024, 1024>

may be valid program semantics.

But:

the grammar only supports tensors up to 1024 × 1024

is prohibited.


---

15. Requirement / Capability / Constraint / Preference / Hint / Realization

These concepts must remain separate.

15.1 Requirement

A condition necessary for valid execution.

Conceptually:

requires qubits >= n


---

15.2 Capability

Something the target can provide.

Conceptually:

requires capability("quantum.measurement")


---

15.3 Constraint

A condition that must remain satisfied.

Conceptually:

requires latency <= budget


---

15.4 Preference

A desirable implementation property that is not semantic necessity.

Conceptually:

prefer accelerator("tensor.compute")


---

15.5 Hint

Information supplied to optimization or realization without changing program meaning.

Conceptually:

hint locality


---

15.6 Realization

A downstream implementation choice.

Conceptually:

logical resource
    ->
physical resource

Physical realization belongs to compilation, routing, scheduling, HAL, deployment, and runtime systems.


---

16. Hardware Resource Model

Hardware resources are semantic quantities or relationships.

Examples include:

compute capacity;

memory;

storage;

bandwidth;

communication capacity;

accelerator capacity;

quantum resources;

programmable logic resources;

timing budget;

power budget;

thermal budget;

reliability budget.


Resource declarations must use expressions and semantic types.

They must not embed physical limits.

The following conceptual forms are valid:

requires memory >= required_memory
requires compute >= required_compute
requires qubits >= required_qubits
requires bandwidth >= required_bandwidth

The actual resource value is resolved downstream.


---

17. Capability Model

Capabilities describe what a target can do.

Examples:

quantum.compute
quantum.measurement
quantum.mid_circuit_measurement
tensor.compute
gpu.compute
fpga.reconfiguration
programmable_logic
cryptographic.compute
high_bandwidth_memory
distributed.communication

Capability names should be extensible.

Vendor-specific capabilities should not require permanent core-language keywords.

Qualified capability names and extension mechanisms should be preferred.


---

18. Target Model

Targets represent target classes or target contracts.

A target is not necessarily a physical device.

For example, a target can conceptually describe:

CPU-class execution
GPU-class execution
FPGA-class execution
ASIC-class execution
QPU-class execution
heterogeneous execution
distributed execution
embedded execution
cloud execution
simulator execution
future execution model

The grammar must not assume that a target name identifies one physical machine.


---

19. Device Model

A source-level device identifier is a semantic name unless a separate target/deployment contract explicitly defines it as a realization identifier.

The core hardware grammar must not require physical enumeration.

It must not assume:

gpu0
gpu1
gpu2
...

or:

qpu0
qpu1
...

as the universal hardware model.

Physical discovery belongs downstream.


---

20. Topology Model

Topology syntax expresses relationships and requirements.

Examples include:

connectivity;

proximity;

degree;

distance;

locality;

communication relationships;

hierarchy;

affinity;

partitioning.


Topology syntax must not perform graph discovery.

The topology grammar describes requirements.

Routing and physical topology resolution happen later.


---

21. Placement Model

Placement describes intent.

Examples include:

affinity;

locality;

grouping;

proximity;

anti-affinity;

colocation;

separation;

hierarchy.


Placement syntax is an input to downstream mapping and scheduling.

It is not the physical placement algorithm.


---

22. Hardware / HDL Separation

grammar/hardware/ and grammar/hdl/ have different responsibilities.

grammar/hardware/
    hardware contracts and realization intent

grammar/hdl/
    HDL behavioral and structural syntax

Hardware syntax may state that programmable logic or a particular capability is required.

HDL syntax may define:

modules;

signals;

processes;

combinational logic;

sequential logic;

state machines;

pipelines;

interfaces;

memories.


hardware.g4 must not become a duplicate HDL grammar.

There must be no circular dependency:

hardware.g4 <-> hdl.g4

The semantic layers may later integrate.


---

23. Hardware / Quantum Separation

The hardware subsystem describes quantum devices and capabilities.

The quantum subsystem describes quantum computation.

Therefore:

grammar/quantum/
    quantum program semantics

grammar/hardware/
    quantum device contracts

The architecture is:

quantum source
      |
      v
quantum AST
      |
      v
quantum semantic model
      |
      v
quantum::ir
      |
      v
routing
      |
      v
scheduling
      |
      v
resilience/QEC/ZQN
      |
      v
hardware realization

hardware.g4 must never create a second quantum IR.


---

24. Hardware / QEC Separation

The hardware grammar may express requirements related to error correction or reliability.

It must not implement QEC.

Examples of semantic information that may be represented:

requires capability("quantum.error_correction")
requires reliability >= required_reliability
requires noise <= allowed_noise

The actual QEC implementation belongs downstream.

QEC owns:

code selection;

syndrome processing;

correction;

logical error analysis;

fault-tolerance transformation.



---

25. Hardware / ZQN Separation

ZQN is a downstream semantic/system component.

The hardware grammar may expose source-level requirements or properties relevant to noise and reliability.

It must not duplicate ZQN.

The intended direction is:

hardware syntax
      |
      v
semantic hardware model
      |
      v
ZQN / resilience analysis
      |
      v
target realization


---

26. Hardware / Routing Separation

Topology and placement information are inputs to routing.

The grammar must not perform routing.

Routing owns:

path selection;

resource mapping;

connectivity resolution;

movement/decomposition;

physical path construction.


The grammar merely represents the source-level intent.


---

27. Hardware / Scheduling Separation

Timing and resource information can be consumed by the scheduler.

The grammar does not schedule.

Scheduling owns:

ordering;

temporal placement;

resource reservation;

synchronization;

concurrency;

dependency resolution;

execution timing.


The hardware grammar only preserves the relevant source-level contract.


---

28. Hardware / Optimization Separation

Preferences and hints may guide optimization.

The grammar does not optimize.

Optimization owns semantics-preserving transformations.

A parser must never choose a GPU, FPGA, QPU, CPU, physical qubit, or execution schedule merely because the grammar contains a preference.


---

29. Hardware / HAL Separation

The HAL is responsible for target-specific capabilities and runtime state.

The hardware grammar must not query the HAL.

There must be no:

grammar -> device discovery
grammar -> runtime
grammar -> driver
grammar -> physical hardware

relationship.

Instead:

source
  |
  v
semantic requirements
  |
  v
compiler
  |
  v
HAL
  |
  v
available target


---

30. Hardware / Runtime Separation

The parser must never execute hardware operations.

No grammar action may:

open a device;

allocate memory;

communicate with a GPU;

access a QPU;

invoke a driver;

query a physical machine;

perform scheduling;

execute code.


The grammar must be action-free.


---

31. Existing Hardware Directory

The current repository contains the following important hardware files:

grammar/hardware/
├── README.md
├── hardware.g4
├── devices.g4
├── resources.g4
├── capabilities.g4
├── constraints.g4
├── hardware-constraints.g4
├── targets.g4
├── topology.g4
├── placement.g4
├── accelerators.g4
├── compute.g4
├── memory.g4
├── interconnect.g4
├── timing.g4
├── power.g4
├── thermal.g4
├── reliability.g4
├── calibration.g4
├── negotiation.g4
├── deployment.g4
├── cpu.g4
├── gpu.g4
├── fpga.g4
├── asic.g4
├── qpu.g4
└── quantum-device.g4

Existing files should be expanded and normalized rather than unnecessarily renamed.


---

32. hardware.g4

Purpose

hardware.g4 is the hardware composition grammar.

It is not the implementation grammar for every hardware technology.

Owns

hardware declaration entry;

common hardware composition;

common hardware attributes;

common generic parameters;

common hardware contracts;

common hardware body;

integration of specialized hardware declarations.


Does not own

It must not duplicate:

resources;

capabilities;

constraints;

targets;

devices;

topology;

placement;

accelerators;

CPU;

GPU;

FPGA;

ASIC;

QPU;

timing;

power;

thermal;

reliability;

calibration;

deployment;

interconnect;

memory.


Required correction

The current hardware.g4 must be normalized to the canonical lexer vocabulary and parser architecture.

It must not rely on a second token vocabulary such as a parallel ZamaniTokens architecture if the canonical repository lexer is ZamaniLexer.

It must not use undefined K_* tokens merely because an older hardware grammar used them.

It must reuse:

canonical identifiers;

canonical expressions;

canonical types;

canonical punctuation;

canonical operators.


The composition grammar must delegate rather than duplicate.


---

33. resources.g4

Owns

resource declarations;

resource quantities;

resource relationships;

resource references;

resource properties.


Does not own

allocation;

runtime accounting;

scheduling;

target discovery.


Integration

Consumes:

core names
types
expressions

Produces:

resource syntax

Downstream:

semantic resource model


---

34. capabilities.g4

Owns

capability declaration;

capability reference;

capability properties;

capability relationships.


Does not own

capability discovery;

device probing;

runtime feature detection.


Integration

Capabilities must map to a semantic capability model.

They must remain extensible.


---

35. constraints.g4

Owns

hardware constraints;

comparison relationships;

constraint expressions;

constraint properties.


Does not own

optimization;

scheduling;

target discovery.


It represents conditions that downstream analysis must satisfy.


---

36. hardware-constraints.g4

This file is currently an overlapping/duplicate constraint authority.

It must not remain a second permanent constraints grammar.

Before deletion:

1. identify all references;


2. compare rules against constraints.g4;


3. migrate unique functionality to the canonical owner;


4. update imports;


5. update tests;


6. update documentation;


7. verify no references remain;


8. remove the redundant file only after conformance passes.



Do not maintain two grammars defining the same concept.


---

37. targets.g4

Owns

target declarations;

target properties;

target contracts;

target classes;

target-independent target metadata.


Does not own

physical target discovery;

backend implementation;

runtime allocation.


Target syntax must describe classes of execution environments.


---

38. devices.g4

Owns

abstract device declarations;

device classes;

device relationships;

semantic device properties.


Does not own

physical enumeration;

driver handles;

operating-system device IDs;

physical addresses.



---

39. topology.g4

Owns

topology declarations;

topology requirements;

connectivity;

degree;

distance;

locality;

hierarchy.


Does not own

routing;

graph optimization;

physical topology discovery.



---

40. placement.g4

Owns

placement intent;

affinity;

grouping;

locality;

proximity;

separation;

placement properties.


Does not own

physical placement;

scheduling;

resource allocation.



---

41. accelerators.g4

Owns

accelerator contracts;

accelerator classes;

accelerator capabilities;

accelerator resources;

accelerator properties.


Does not own

CUDA;

ROCm;

vendor runtime APIs;

physical accelerator allocation;

scheduling.


Framework/vendor implementation belongs outside the language core.


---

42. compute.g4

This grammar should describe generic compute capability and hardware compute intent.

It must not encode fixed:

core counts;

execution units;

vector widths;

instruction widths;

processor counts.


Hardware-specific characteristics belong in semantic properties or capability models.


---

43. memory.g4

Owns

memory contracts;

memory classes;

memory relationships;

memory capabilities;

memory properties;

memory access intent.


It must not hard-code:

64 GB RAM
24 GB VRAM
32-bit registers
fixed cache sizes
fixed bank counts

Actual values belong to target discovery and semantic resource resolution.


---

44. interconnect.g4

Owns

abstract interconnect declarations;

communication capabilities;

bandwidth requirements;

latency requirements;

connectivity relationships;

interconnect properties.


Does not own

physical network discovery;

packet routing;

physical switch configuration;

driver implementation.



---

45. timing.g4

Timing is a semantic contract.

It may describe:

latency;

duration;

deadlines;

periods;

synchronization;

temporal relationships;

timing constraints.


It must not assume a particular clock frequency.

For example, the language may express:

requires latency <= budget

without assuming:

clock = 3.2GHz

unless that value is explicitly program/target semantics.


---

46. power.g4

Power syntax may express:

power requirements;

power constraints;

energy budgets;

energy properties;

power preferences.


It must not impose universal machine limits.


---

47. thermal.g4

Thermal syntax may express:

thermal constraints;

temperature relationships;

cooling requirements;

thermal budgets;

thermal properties.


It must not encode a universal physical operating range as a language limitation.


---

48. reliability.g4

Reliability syntax may express:

reliability requirements;

availability;

fault tolerance;

recovery expectations;

degradation policies;

reliability properties.


The resilience subsystem remains responsible for runtime and compiler decisions.

Required resilience vocabulary may include:

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

These are semantic vocabulary, not runtime implementations.


---

49. calibration.g4

Calibration syntax may describe calibration requirements or metadata.

It must not perform calibration.

Actual calibration remains a target/runtime concern.


---

50. negotiation.g4

Negotiation syntax may express:

acceptable capabilities;

resource alternatives;

preferences;

fallback policies;

target selection criteria.


It must not perform runtime negotiation during parsing.


---

51. deployment.g4

Deployment syntax may express deployment intent.

It must not become a second runtime or orchestration language.

Physical deployment decisions belong downstream.


---

52. cpu.g4

CPU syntax must remain CPU-class and architecture-independent.

It must not assume:

N cores
N registers
fixed ISA
fixed register width
fixed cache size

A target may expose such facts later through capabilities and resources.


---

53. gpu.g4

GPU syntax must remain extensible across GPU architectures.

It must not hard-code:

GPU count;

warp size;

wavefront size;

physical GPU IDs;

vendor runtime behavior;

memory capacity;

architecture-specific instruction limits.


Vendor-specific information should use extension/property mechanisms.

The existing GPU grammar requires normalization into the same parser-grammar architecture as the other hardware grammars before it can become a clean composition dependency.


---

54. fpga.g4

FPGA syntax must express:

programmable logic;

FPGA capability;

FPGA resources;

parameters;

configuration intent.


It must not hard-code:

LUT counts;

DSP counts;

BRAM counts;

physical pins;

board identities;

synthesis tool behavior.



---

55. asic.g4

ASIC syntax must express:

ASIC contracts;

implementation requirements;

technology-independent properties;

physical constraints that are genuinely semantic.


It must not own:

foundry selection;

transistor-level synthesis;

place-and-route;

timing closure;

physical design implementation.



---

56. qpu.g4

QPU syntax describes quantum-processing hardware.

It may express:

QPU capabilities;

quantum resources;

measurement capability;

connectivity;

topology;

device-level properties;

error characteristics;

execution capabilities.


It must not enumerate quantum gates.

Quantum computation belongs to grammar/quantum/.

The canonical quantum semantic boundary remains:

quantum::ir


---

57. quantum-device.g4

This grammar describes quantum-device contracts.

It must remain separate from quantum program syntax.

The intended division is:

quantum/
    computation

hardware/quantum-device.g4
    quantum hardware

No second quantum IR may be introduced.


---

58. Generic Parameters

Hardware constructs should support semantic generic parameters where useful.

For example, conceptually:

hardware Accelerator<Capacity> {
    requires resource.compute >= Capacity;
}

The value of Capacity is a semantic parameter.

It is not a parser-defined maximum.

Generic parameters allow the same source structure to scale across:

tiny systems;

embedded systems;

large accelerators;

heterogeneous systems;

clusters;

future systems.



---

59. Qualified Names and Extensibility

The hardware grammar must support extensibility through names and properties rather than an ever-growing list of keywords.

Conceptually:

capability("vendor.feature")

or:

vendor.namespace.property

This permits new hardware technologies to be represented without modifying the core grammar every time a vendor introduces a feature.

A new keyword should be introduced only when the concept genuinely has language-level semantic meaning.


---

60. Hardware Extensions

New hardware technologies should first attempt to use:

existing resources;

existing capabilities;

requirements;

constraints;

preferences;

hints;

properties;

generic parameters;

target classes;

dialect extensions.


A new grammar file is justified only when the technology introduces syntax that cannot be represented cleanly by the existing abstractions.


---

61. No Vendor Lock-In

The core hardware grammar must not require:

CUDA;

ROCm;

vendor-specific QPU APIs;

FPGA vendor languages;

ASIC vendor flows;

proprietary driver APIs;

proprietary deployment formats.


Such systems belong to interoperability, dialect, backend, HAL, or tooling layers.


---

62. Hardware and Classical Computing

Hardware syntax must compose with classical computation.

Example semantic relationship:

classical computation
        |
        v
requires compute
        |
        v
target capability
        |
        v
CPU/GPU/accelerator realization

The classical program should remain semantically independent from the physical machine.


---

63. Hardware and Quantum Computing

The same hardware contract can support quantum execution:

quantum computation
        |
        v
quantum::ir
        |
        v
resource/capability requirements
        |
        v
QPU target
        |
        v
routing/scheduling/QEC/ZQN
        |
        v
physical execution

The quantum program does not need to be rewritten merely because the target changes.


---

64. Hardware and HDL

Hardware/software co-design must be possible:

software algorithm
       +
hardware intent
       +
HDL implementation
       +
resource requirements
       +
timing requirements
       +
verification properties

The semantic model must preserve the relationship between the domains without collapsing their grammar ownership.


---

65. Hardware and AI/Data

AI and data workloads may express requirements such as:

requires capability("tensor.compute")
requires memory >= required_memory
prefer accelerator("tensor")

The hardware grammar must not encode a particular AI framework.

The AI grammar remains responsible for AI semantics.

The data grammar remains responsible for data semantics.

Hardware remains responsible for hardware intent.


---

66. Hardware and Distributed Computing

Distributed programs may express:

compute resources;

communication requirements;

topology;

placement;

replication;

locality;

bandwidth;

latency;

fault tolerance.


There must be no universal:

MAX_NODES

or equivalent hidden node limit.


---

67. Hardware and Networking

Networking capabilities may be represented as semantic capabilities and resource requirements.

Examples:

requires capability("network.streaming")
requires bandwidth >= required_bandwidth
requires latency <= budget

Physical addresses and routes remain downstream.


---

68. Hardware and Security

Hardware security capabilities may include:

secure execution;

cryptographic acceleration;

trusted execution;

secure memory;

hardware-backed identity;

isolation.


Security semantics remain owned by grammar/security/ and the security semantic subsystem.

Hardware merely represents the hardware relationship.


---

69. Hardware and Effects

Hardware operations may have effects.

For example:

device access;

resource acquisition;

external communication;

timing;

measurement;

persistent state.


The hardware grammar must not reimplement the effect system.

Effects remain integrated through the canonical effect grammar and semantic system.


---

70. Hardware and Memory

Memory must be represented semantically.

The language must support the distinction between:

memory requirement
memory capability
memory preference
memory placement
memory realization

For example:

requires memory >= workload_memory

does not mean:

allocate physical RAM now


---

71. Hardware and Concurrency

Hardware resources may participate in:

parallelism;

task parallelism;

data parallelism;

pipelines;

asynchronous execution;

distributed execution.


The hardware grammar does not own concurrency semantics.

grammar/concurrency/ owns the concurrency language.

Hardware only provides relevant capabilities and resource contracts.


---

72. Canonical Names

Hardware grammars must reuse the canonical name and identifier infrastructure.

They must not create independent versions of:

identifier
qualified name
path
member access
namespace

If a shared name rule is needed, it belongs in the common grammar infrastructure.


---

73. Canonical Expressions

Hardware quantities must use the canonical expression grammar.

Do not create local copies of arithmetic expressions inside hardware grammars.

This is especially important for:

resource quantities;

constraints;

timing;

power;

thermal values;

capacities;

dimensions;

topology parameters.


The direction must be:

hardware grammar
      |
      v
canonical expression grammar

not:

hardware grammar
      |
      +--> private expression language


---

74. Canonical Types

Hardware grammars must reuse the canonical type system.

Hardware-specific semantic types may be defined under grammar/types/ when necessary.

The hardware grammar should reference them rather than redefining:

generic types;

arrays;

references;

numeric types;

resource types;

capability types.



---

75. AST Contract

Every hardware grammar construct must have a predetermined AST representation.

The AST must preserve enough information to represent:

source span;

name;

attributes;

modifiers;

generic parameters;

properties;

resources;

capabilities;

requirements;

constraints;

preferences;

hints;

target information;

topology;

placement;

mappings;

device class;

specialization metadata.


The AST must not contain physical runtime state.


---

76. Semantic Contract

Semantic analysis interprets the AST.

For example:

requires memory >= required_memory

must eventually be checked for:

valid resource name;

valid expression;

valid units/types;

valid comparison;

valid resource semantics.


The parser does not perform this analysis.


---

77. IR Contract

Hardware grammar must lower indirectly:

source
  |
  v
grammar
  |
  v
AST
  |
  v
semantic hardware model
  |
  v
canonical IR / semantic representations

Never:

source
  |
  v
hardware grammar
  |
  v
physical hardware

And never:

hardware grammar
  |
  v
second quantum IR


---

78. quantum::ir Boundary

Quantum computation must eventually reach the canonical:

quantum::ir

There must be exactly one canonical quantum IR boundary.

The hardware grammar can describe QPU requirements and device capabilities.

It cannot replace the quantum IR.


---

79. Compiler Integration

Hardware semantic information may be consumed by:

target selection;

capability analysis;

resource analysis;

optimization;

lowering;

routing;

scheduling;

synthesis;

resilience;

deployment.


The grammar must expose source information without embedding these algorithms.


---

80. Runtime Integration

Runtime systems may use compiled hardware metadata to:

discover available targets;

query capabilities;

allocate resources;

dispatch execution;

adapt to runtime conditions;

recover from failures.


Runtime must not need to reinterpret source syntax to discover basic hardware semantics.


---

81. Diagnostics

Hardware syntax must support deterministic diagnostics.

Diagnostics should identify:

source file;

source span;

offending token;

expected syntax;

relevant construct;

stable diagnostic category.


Semantic diagnostics should distinguish:

syntax error
type error
unknown capability
unknown resource
invalid requirement
unsatisfied requirement
invalid constraint
invalid target
invalid topology
invalid placement

The parser must not attempt to solve semantic errors by silently changing the source meaning.


---

82. Determinism

The hardware grammar must parse deterministically.

It must avoid:

unnecessary ambiguous alternatives;

competing declaration forms;

duplicate ownership;

hidden precedence rules;

context-sensitive hacks where avoidable;

embedded executable predicates.


The same source and same grammar version must produce the same parse structure.


---

83. Action-Free Grammar

All hardware grammars must remain action-free.

No:

{ Rust code }

actions.

No:

@parser::members

containing runtime logic.

No embedded device calls.

No unsafe Rust.

The parser is a syntax recognizer, not a hardware controller.


---

84. Rust 1.97 / 1.97.1

The repository implementation must remain compatible with:

Rust 1.97
Rust 1.97.1
Edition 2021

The grammar itself must not depend on unstable or unsafe Rust behavior.

Generated parser code must be tested against the repository's supported toolchain.


---

85. No unsafe

The compiler implementation must not introduce Rust:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

The grammar must remain action-free so that it cannot introduce hidden implementation behavior.

This safety rule applies to the compiler implementation.

It is distinct from whether the Zamani source language itself eventually has an unsafe source-level construct.


---

86. Existing gpu.g4 Integration

The existing gpu.g4 must be treated carefully.

It currently does not fit the desired clean parser-grammar architecture as cleanly as the other specialized parser grammars.

Before it becomes a direct imported dependency of the composition root, it must be normalized to:

parser grammar ...
options { tokenVocab = canonical lexer; }

and must:

reuse canonical tokens;

reuse canonical names;

reuse canonical expressions;

reuse canonical types;

expose one stable GPU declaration entry point;

contain no embedded implementation code;

contain no duplicate hardware composition;

contain no physical resource limits.


Until that normalization is complete, hardware.g4 must not create a duplicate GPU implementation merely to compensate.


---

87. Legacy Token Vocabulary

The current hardware grammar contains legacy-style constructs such as:

ZamaniTokens
K_HARDWARE
K_RESOURCE
K_CAPABILITY
...

These must be reconciled with the canonical lexical architecture.

The solution is not to create another lexer.

The solution is:

one canonical lexer
        |
        v
one canonical token vocabulary
        |
        v
all parser grammars

Any compatibility bridge must be explicit and temporary.


---

88. Keyword Policy

Hardware technology names should not automatically become keywords.

For example, the existence of a new accelerator vendor should not require a new permanent Zamani keyword.

Prefer:

qualified names
properties
capabilities
dialects
extension points

over:

one new keyword for every technology

This is essential for POCO-REAF.


---

89. Hardware Technology Evolution

A new technology should be representable without rewriting old programs.

For example, future hardware could introduce:

new accelerators;

new memory technologies;

new interconnects;

new compute paradigms;

new quantum architectures;

new programmable substrates;

new heterogeneous systems.


Existing programs should remain valid where their semantics remain valid.

The compiler can discover the new realization.


---

90. Resource Negotiation

Hardware requirements may have alternatives.

The language should eventually support concepts such as:

requires capability("A")
or capability("B")

or equivalent canonical requirement semantics.

Negotiation belongs to semantic/compiler layers.

The grammar must preserve enough structure for those layers.


---

91. Resource Availability

The language may express resource requirements.

Availability is target-dependent.

Therefore:

requires memory >= required_memory

does not mean that the grammar itself knows how much memory exists.

The semantic/compiler pipeline evaluates the requirement against a target.


---

92. Tiny-to-Large Scaling

The same semantic hardware model must work for:

single operation
single resource
single device
embedded system
microcontroller
CPU
multicore CPU
GPU
FPGA
ASIC
QPU
accelerator
heterogeneous system
cluster
HPC system
distributed system
cloud
future computational substrate

There must be no grammar-defined transition point where a larger machine requires a different language.


---

93. Atom-to-Everywhere Principle

The hardware subsystem must support the broader Zamani vision:

atom
  |
  v
molecule
  |
  v
device
  |
  v
embedded system
  |
  v
processor
  |
  v
accelerator
  |
  v
heterogeneous system
  |
  v
cluster
  |
  v
distributed system
  |
  v
cloud/HPC
  |
  v
future systems

The source-level semantic abstractions remain stable.

Only the realization changes.


---

94. Hardware Generics

Generic hardware parameters may represent:

dimensions;

capacities;

widths;

counts;

timing parameters;

resource quantities;

feature selections.


These are program/semantic parameters.

They must not become global compiler maximums.


---

95. Dimension Semantics

Hardware dimensions must be represented symbolically or generically where possible.

Examples:

width
lanes
capacity
depth
degree
distance
bandwidth
latency

The grammar must not assume a specific machine width.


---

96. Physical Identifiers

Physical identifiers belong to target realization.

Core hardware syntax should not require:

physical_cpu_7
gpu_3
qpu_1
qubit_17
memory_bank_4

unless a separate explicitly target-specific/deployment-level construct defines such a realization.

Portable source should remain portable.


---

97. Physical Addresses

Physical addresses are not core hardware-language semantics.

Do not encode:

0x80000000

as a universal hardware placement mechanism.

If low-level address semantics are required for a specific target, they must belong to an explicitly defined target/ABI/interop layer.


---

98. Vendor Extensions

Vendor extensions must be isolated.

Preferred structure:

vendor.namespace.feature

or an explicit dialect.

Vendor syntax must not silently become universal Zamani syntax.


---

99. Dialect Integration

Hardware dialects must declare:

name;

version;

owner;

syntax extensions;

semantic extensions;

AST mapping;

IR mapping;

compatibility;

feature gates.


A dialect must not create a competing language root.


---

100. Security Boundary

Hardware grammar parsing must not:

access hardware;

execute vendor code;

load arbitrary plugins;

perform network requests;

access filesystem state;

inspect local machine resources.


This protects deterministic builds and reproducible parsing.


---

101. Reproducibility

Hardware source compilation must be reproducible.

Target-specific discovery must occur in a controlled downstream phase.

The parser itself must not depend on:

current CPU
current GPU
current QPU
current memory
current network
current clock
current device inventory

for syntax validity.


---

102. Cross-Compilation

A hardware-aware Zamani program should be parseable and semantically analyzable without having the final physical target installed.

Target-specific validation may occur later.

This enables:

cross compilation;

remote compilation;

CI builds;

reproducible builds;

offline analysis;

simulation;

deployment planning.



---

103. Simulation

The hardware contract model must support simulation targets.

A simulator is a target realization, not a separate language.

The same semantic hardware requirements should be representable for:

real hardware
simulator
emulator
virtual target

where semantically appropriate.


---

104. Future Hardware

Future hardware must be representable without requiring the grammar to predict every future device.

This is achieved through:

capabilities;

resources;

generic parameters;

properties;

qualified names;

dialects;

target classes;

extensible semantic models.


This is preferable to maintaining an ever-growing list of physical device types.


---

105. File Completion Contract

Every hardware .g4 file is complete only when its contract is known in advance.

Every file must document:

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
Public Rules
AST Contract
Semantic Contract
IR Contract
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
Hard-Coding Audit
Diagnostics
Security
Performance
Completion Criteria

This is mandatory.


---

106. Independent-First Completion Rule

The requirement is:

> A file should be able to be completed without needing to redesign it after another file changes.



To achieve that, every file must define its integration contracts before implementation.

For example, before completing topology.g4, define:

what topology owns
what it consumes
what it exports
how its AST is represented
how semantic topology is represented
how routing consumes it
how placement consumes it
what tests prove it complete

Then other files integrate against the contract.

They must not silently redefine topology.


---

107. Stable Public Rules

Public parser rules must be treated as compatibility surfaces.

Changing a public rule name requires:

compatibility analysis;

parser updates;

AST impact analysis;

tests;

documentation updates;

migration information.


Avoid unnecessary renames.

Existing major filenames should remain unless there is a demonstrated architectural reason to remove or consolidate them.


---

108. Duplicate Ownership Rule

No two files may permanently own the same concept.

Examples of prohibited duplication:

constraints.g4
hardware-constraints.g4

both defining hardware constraints.

Likewise:

hardware.g4
gpu.g4

both defining GPU declarations.

Likewise:

hardware.g4
qpu.g4

both defining QPU declarations.

The composition root delegates.

The specialized grammar owns.


---

109. Import Direction

The desired dependency direction is:

canonical lexer
      |
      v
core names / paths
      |
      v
canonical expressions / types
      |
      v
hardware specialized grammars
      |
      v
hardware composition grammar
      |
      v
Zamani root composition

Specialized hardware grammars must not depend on:

runtime;

compiler backend;

HAL implementation;

physical device manager.



---

110. No Circular Grammar Dependencies

The following are prohibited:

hardware.g4 <-> hdl.g4
hardware.g4 <-> quantum.g4
hardware.g4 <-> runtime grammar
hardware.g4 <-> compiler implementation

Shared syntax should be moved upward into common grammar infrastructure.


---

111. Resource / Capability Separation

A resource is not a capability.

For example:

resource memory

describes a resource.

capability quantum.measurement

describes an ability.

A target can have:

resource capacity
+
capability

without these being the same semantic concept.


---

112. Constraint / Preference Separation

A constraint must affect validity.

A preference must guide realization without changing program correctness.

Do not represent both as one generic "requirement" category internally.

The distinction must survive into semantic analysis.


---

113. Requirement / Realization Separation

A source program may require:

qubits >= n

without specifying:

physical_qubit(0)
physical_qubit(1)
...

Physical mapping is downstream.

This is one of the central requirements for POCO-REAF.


---

114. Hardware Contracts and Semantic Types

If hardware concepts require new semantic types, those types belong under:

grammar/types/

rather than being secretly implemented inside hardware grammar.

Examples may include:

Resource<T>
Capability
HardwareTarget
Topology
Placement

where such types are genuinely needed by the language design.


---

115. Hardware Contracts and Effects

If hardware access has effects, those effects must map into the canonical effect system.

The hardware grammar must not invent a second effect language.


---

116. Hardware Contracts and Ownership

If a hardware resource has ownership or borrowing semantics, those concepts must integrate with:

grammar/memory/
grammar/types/
grammar/effects/

rather than creating another ownership model under hardware.


---

117. Hardware Contracts and Concurrency

If hardware resources are shared concurrently, concurrency semantics remain under:

grammar/concurrency/

Hardware provides resource/capability metadata.

Concurrency determines how the program expresses parallel access.


---

118. Hardware Contracts and Distributed Execution

Distributed hardware relationships must integrate with:

grammar/distributed/
grammar/networking/

The hardware subsystem must not create a second distributed-programming model.


---

119. Hardware Contracts and AI

AI acceleration requirements integrate with:

grammar/ai/
grammar/data/
grammar/classical/

Hardware remains responsible for hardware intent.


---

120. Hardware Contracts and Interoperability

External hardware descriptions may be imported through:

grammar/interoperability/

Possible external formats include:

HDL formats;

QASM;

QIR;

LLVM-related representations;

MLIR-related representations;

vendor formats.


These are interoperability boundaries, not competing Zamani semantic models.


---

121. Testing Architecture

Hardware testing must occur at several levels:

lexical
syntax
AST
semantic
IR
compiler
target
runtime

A syntax-only test is insufficient for production readiness.


---

122. Positive Tests

Every public hardware construct needs positive tests.

Required categories include:

minimal hardware
resources
capabilities
requirements
constraints
preferences
targets
devices
topology
placement
mapping
connections
memory
interconnect
timing
power
thermal
reliability
accelerators
CPU
GPU
FPGA
ASIC
QPU
quantum device
deployment
negotiation


---

123. Negative Tests

Negative tests must verify rejection of:

malformed hardware declarations;

missing identifiers;

invalid generic syntax;

invalid resource expressions;

malformed requirements;

invalid constraints;

malformed topology;

malformed placement;

malformed mappings;

invalid properties;

invalid target declarations;

invalid specialized declarations.



---

124. Boundary Tests

Boundary tests must include:

empty collections where legal;

one-element collections;

nested hardware declarations;

large property lists;

large generic lists;

deeply nested expressions;

large topology descriptions;

large resource sets;

large device compositions.


Tests must not accidentally define artificial language limits.


---

125. Scalability Tests

Scalability tests must vary:

number of hardware declarations
number of resources
number of capabilities
number of ports
number of connections
number of instances
number of topology relationships
number of properties
number of generic parameters

The tests must establish that the grammar has no arbitrary fixed hardware capacity.


---

126. Tiny-to-Infinite Test Principle

"Infinity" here means no language-defined artificial upper bound.

No finite test can literally instantiate an infinite machine.

Therefore production tests establish:

no grammar-defined maximum
+
successful scaling across increasingly large inputs
+
graceful failure only from implementation/resource exhaustion

This distinction must be documented.


---

127. POCO-REAF Test

A representative test should conceptually compile the same semantic program against:

tiny target
CPU
GPU
FPGA
ASIC
QPU
heterogeneous target
distributed target
simulator
future/extension target

The source semantics remain unchanged.

Only target realization changes.


---

128. Cross-Domain Tests

Mandatory combinations include:

classical + hardware
quantum + hardware
HDL + hardware
classical + quantum + hardware
quantum + HDL + hardware
AI + hardware
distributed + hardware
networking + hardware
security + hardware
AI + quantum + hardware
classical + quantum + HDL + hardware

These tests must verify grammar composition and semantic compatibility.


---

129. Determinism Tests

The same source must produce the same:

token sequence;

parse structure;

diagnostics;

AST structure;


for the same compiler/grammar version.

Hardware discovery must not affect parsing.


---

130. Hard-Coding Audit

Every hardware grammar change must be scanned for:

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

and equivalent forms.

Also inspect for accidental fixed resources such as:

gpu0
gpu1
qpu0
qubit0
qubit1
core0
core1
memory64gb

when these are being used as universal language structures.


---

131. Hard-Coding Classification

Every fixed number encountered must be classified as:

1. program semantics
2. explicit semantic requirement
3. target-specific requirement
4. resource constraint
5. implementation limitation
6. test fixture
7. documentation example
8. accidental language limitation

Only legitimate categories may remain.

An implementation limitation must not silently become a language limitation.


---

132. Performance

The grammar must remain scalable in parser complexity.

Avoid:

unnecessary ambiguous alternatives;

repeated expensive lookahead;

duplicated expression grammars;

recursive structures that can be represented iteratively;

redundant dispatch layers.


Large hardware descriptions should be parseable without quadratic behavior introduced by avoidable grammar ambiguity.

Performance benchmarking belongs in validation.


---

133. Error Recovery

Error recovery must preserve useful diagnostics.

Malformed hardware syntax should not cause unrelated parts of a program to become unintelligible when recovery is possible.

The parser must remain deterministic.


---

134. Source Spans

Every hardware AST construct must be traceable to source spans.

At minimum:

start position
end position
source file identity

The semantic and diagnostic layers must be able to report errors against these spans.


---

135. Tooling

The grammar must support:

syntax highlighting;

IDE completion;

language-server parsing;

navigation;

formatting;

diagnostics;

AST inspection;

documentation generation;

refactoring tools.


Stable syntax and public rule contracts are therefore tooling compatibility surfaces.


---

136. Documentation Generation

Hardware documentation should ultimately be generated from authoritative contracts where practical.

Generated documentation must not become another independent language specification.


---

137. Compatibility

Hardware syntax changes must be tracked in:

grammar/compatibility/
grammar/spec/compatibility.md

Compatibility analysis must consider:

lexer
parser
AST
semantic model
IR
compiler
backend
runtime

A grammar change is not complete until downstream impact is understood.


---

138. Versioning

Hardware extensions must be version-aware.

A feature may be:

experimental
proposed
stable
deprecated
removed

Versioning must not require rewriting unrelated hardware programs.


---

139. Deprecation

Deprecated syntax must have:

documented replacement;

compatibility period;

diagnostics;

migration path;

tests.


Do not silently remove existing syntax without compatibility analysis.


---

140. Repository-Wide Integration Matrix

Component	Hardware relationship

grammar/DESIGN.md	architectural authority
grammar/Zamani.g4	canonical root composition
grammar/lexer/	lexical contract
grammar/core/	names, paths, common syntax
grammar/types/	shared type system
grammar/expressions/	shared expressions
grammar/resources/	universal resource model
grammar/classical/	classical computation
grammar/quantum/	quantum computation
grammar/hybrid/	quantum/classical composition
grammar/hdl/	HDL syntax
grammar/hardware/	hardware intent
grammar/distributed/	distributed semantics
grammar/ai/	AI semantics
grammar/data/	data semantics
grammar/networking/	networking semantics
grammar/security/	security semantics
grammar/compile/	compilation intent
grammar/execution/	execution intent
grammar/interoperability/	external formats
grammar/dialects/	controlled extensions
src/lexer.rs	executable lexical implementation
src/parser.rs	executable parser
src/frontend/ast/	syntax AST
semantic analysis	hardware meaning
quantum::ir	canonical quantum semantic boundary
QEC	quantum error correction
ZQN	fault/noise semantics
optimization	transformation
routing	physical mapping
scheduling	temporal/resource scheduling
HAL	target capabilities/state
runtime	execution



---

141. Ownership Matrix

Concept	Owner

Identifiers	grammar/core/
Expressions	grammar/expressions/
General types	grammar/types/
Hardware composition	hardware/hardware.g4
Resources	hardware/resources.g4
Capabilities	hardware/capabilities.g4
Constraints	hardware/constraints.g4
Targets	hardware/targets.g4
Devices	hardware/devices.g4
Topology	hardware/topology.g4
Placement	hardware/placement.g4
Accelerators	hardware/accelerators.g4
Compute	hardware/compute.g4
Memory	hardware/memory.g4
Interconnect	hardware/interconnect.g4
Timing	hardware/timing.g4
Power	hardware/power.g4
Thermal	hardware/thermal.g4
Reliability	hardware/reliability.g4
Calibration	hardware/calibration.g4
Negotiation	hardware/negotiation.g4
Deployment	hardware/deployment.g4
CPU	hardware/cpu.g4
GPU	hardware/gpu.g4
FPGA	hardware/fpga.g4
ASIC	hardware/asic.g4
QPU	hardware/qpu.g4
Quantum devices	hardware/quantum-device.g4
Quantum computation	grammar/quantum/
Quantum canonical IR	quantum::ir
HDL	grammar/hdl/
Routing	compiler routing subsystem
Scheduling	compiler scheduling subsystem
QEC	quantum resilience subsystem
ZQN	fault/noise subsystem
Physical discovery	HAL/target subsystem
Runtime execution	runtime



---

142. Recommended Independent-First Completion Order

The hardware subsystem should be completed in this order:

1. README.md
       |
       v
2. canonical lexer/token contract
       |
       v
3. core names/paths
       |
       v
4. canonical expressions
       |
       v
5. canonical types
       |
       v
6. resources.g4
       |
       v
7. capabilities.g4
       |
       v
8. constraints.g4
       |
       v
9. devices.g4
       |
       v
10. targets.g4
       |
       v
11. topology.g4
       |
       v
12. placement.g4
       |
       v
13. compute.g4
       |
       v
14. memory.g4
       |
       v
15. interconnect.g4
       |
       v
16. timing.g4
       |
       v
17. power.g4
       |
       v
18. thermal.g4
       |
       v
19. reliability.g4
       |
       v
20. calibration.g4
       |
       v
21. negotiation.g4
       |
       v
22. deployment.g4
       |
       v
23. accelerators.g4
       |
       v
24. cpu.g4
       |
       v
25. gpu.g4
       |
       v
26. fpga.g4
       |
       v
27. asic.g4
       |
       v
28. qpu.g4
       |
       v
29. quantum-device.g4
       |
       v
30. hardware.g4 composition
       |
       v
31. AST integration
       |
       v
32. semantic integration
       |
       v
33. IR integration
       |
       v
34. compiler integration
       |
       v
35. HAL integration
       |
       v
36. runtime integration
       |
       v
37. repository-wide tests

The order is intentional.

It prevents the composition root from compensating for incomplete leaf grammars.


---

143. Completion Criteria for resources.g4

Complete only when:

resource syntax is defined;

expressions are canonical;

types are canonical;

resource semantics are documented;

AST mapping is defined;

semantic mapping is defined;

IR mapping is defined;

hardware integration is defined;

compiler consumers are defined;

runtime consumers are defined;

positive tests exist;

negative tests exist;

scalability tests exist;

hard-coding audit passes.



---

144. Completion Criteria for capabilities.g4

Complete only when:

capability syntax is defined;

capability names are extensible;

qualified names work;

AST mapping exists;

semantic capability model exists;

target capability checking is defined;

vendor extension behavior is defined;

tests exist;

no physical discovery is embedded;

no fixed capability list becomes mandatory.



---

145. Completion Criteria for targets.g4

Complete only when:

target classes are representable;

target properties are representable;

target requirements are representable;

target constraints are representable;

target realization is downstream;

no physical device enumeration is required;

AST/semantic/IR contracts exist;

tests exist.



---

146. Completion Criteria for topology.g4

Complete only when:

abstract topology is expressible;

relationships are expressible;

requirements are expressible;

properties are expressible;

topology does not perform routing;

topology does not discover hardware;

routing can consume its semantic representation;

tests exist.



---

147. Completion Criteria for placement.g4

Complete only when:

placement intent is expressible;

affinity is expressible;

locality is expressible;

grouping is expressible;

constraints are expressible;

physical placement is downstream;

AST/semantic/IR contracts exist;

tests exist.



---

148. Completion Criteria for CPU/GPU/FPGA/ASIC/QPU

Every specialized compute grammar is complete only when:

class declaration
+
capabilities
+
resources
+
parameters
+
properties
+
requirements
+
constraints
+
AST
+
semantics
+
IR
+
compiler consumers
+
tests

are defined.

None may encode a universal physical capacity.


---

149. Completion Criteria for hardware.g4

hardware.g4 is complete only when:

it is a composition grammar;

it has one stable hardware entry point;

it uses the canonical lexer vocabulary;

it uses canonical names;

it uses canonical expressions;

it uses canonical types;

it delegates specialized ownership;

it does not duplicate leaf grammars;

it does not contain physical limits;

it does not contain device discovery;

it does not contain runtime behavior;

it does not create a quantum IR;

it integrates with the canonical Zamani root;

its AST contract is complete;

its semantic contract is complete;

its IR contract is complete;

all public rules have tests;

integration tests pass.



---

150. Completion Criteria for the Entire Directory

grammar/hardware/ is production-ready only when:

[ ] Authority is unambiguous
[ ] Hardware ownership is unambiguous
[ ] Duplicate ownership is removed
[ ] Canonical lexer is used
[ ] Canonical names are used
[ ] Canonical expressions are used
[ ] Canonical types are used
[ ] Hardware composition is modular
[ ] Resource model is integrated
[ ] Capability model is integrated
[ ] Requirement model is integrated
[ ] Constraint model is integrated
[ ] Preference model is integrated
[ ] Target model is integrated
[ ] Device model is integrated
[ ] Topology model is integrated
[ ] Placement model is integrated
[ ] Accelerator model is integrated
[ ] CPU model is integrated
[ ] GPU model is integrated
[ ] FPGA model is integrated
[ ] ASIC model is integrated
[ ] QPU model is integrated
[ ] Quantum-device model is integrated
[ ] Memory model is integrated
[ ] Interconnect model is integrated
[ ] Timing model is integrated
[ ] Power model is integrated
[ ] Thermal model is integrated
[ ] Reliability model is integrated
[ ] Calibration boundary is defined
[ ] Negotiation boundary is defined
[ ] Deployment boundary is defined
[ ] HDL separation is enforced
[ ] Quantum separation is enforced
[ ] quantum::ir remains canonical
[ ] QEC remains downstream
[ ] ZQN remains downstream
[ ] Routing remains downstream
[ ] Scheduling remains downstream
[ ] Optimization remains downstream
[ ] HAL remains downstream
[ ] Runtime remains downstream
[ ] No unsafe Rust is introduced
[ ] Grammar is action-free
[ ] No hardware discovery occurs during parsing
[ ] No physical device enumeration occurs in core grammar
[ ] No artificial capacity limits exist
[ ] No fixed qubit limit exists
[ ] No fixed CPU limit exists
[ ] No fixed GPU limit exists
[ ] No fixed FPGA limit exists
[ ] No fixed node limit exists
[ ] No fixed memory limit exists
[ ] No fixed thread limit exists
[ ] No fixed tensor-rank limit exists
[ ] No fixed register-width limit exists
[ ] No fixed network-size limit exists
[ ] No fixed device-count limit exists
[ ] Generic parameters work
[ ] Qualified names work
[ ] Vendor extensions work without core-language pollution
[ ] Dialect integration is defined
[ ] Source spans are preserved
[ ] Diagnostics are deterministic
[ ] Positive tests pass
[ ] Negative tests pass
[ ] Boundary tests pass
[ ] Scalability tests pass
[ ] Determinism tests pass
[ ] Cross-domain tests pass
[ ] Compatibility tests pass
[ ] AST coverage passes
[ ] Semantic coverage passes
[ ] IR coverage passes
[ ] Compiler integration passes
[ ] HAL integration passes
[ ] Runtime integration passes
[ ] Rust 1.97/1.97.1 build passes


---

151. Production Validation Pipeline

The final validation pipeline is:

README / architecture
        |
        v
Specification
        |
        v
Lexical contract
        |
        v
Canonical tokens
        |
        v
ANTLR grammar validation
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
Semantic coverage
        |
        v
IR coverage
        |
        v
Compiler coverage
        |
        v
HAL/target coverage
        |
        v
Runtime coverage
        |
        v
Cross-domain tests
        |
        v
Scalability tests
        |
        v
Compatibility tests
        |
        v
Production release

A grammar file cannot be declared production-ready solely because ANTLR generates a parser.


---

152. What Must Never Be Added

Do not add:

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

Do not add:

physical_qubit_0
physical_qubit_1
...

as a universal hardware model.

Do not add:

gpu0
gpu1
gpu2

as a universal GPU model.

Do not add:

wire [31:0]

as a universal hardware width.

Do not add:

RAM = 64GB
VRAM = 24GB

as language-wide assumptions.

Do not add fixed topology assumptions.

Do not add fixed clock frequencies.

Do not add fixed vendor assumptions.

Do not add a second quantum IR.

Do not add hardware execution actions.

Do not add hardware discovery to the parser.

Do not add runtime behavior to the grammar.


---

153. What Should Be Added

Prefer:

requires resource >= expression
requires capability("qualified.capability")
requires topology(...)
requires placement(...)
prefer ...
hint ...
target ...
property ...
generic parameter ...
qualified extension ...

This keeps hardware intent portable.


---

154. Example Portable Hardware Intent

A conceptual Zamani program may eventually express:

hardware WorkloadTarget<RequiredCompute> {
    requires resource.compute >= RequiredCompute;
    requires capability("tensor.compute");
    prefer accelerator("compute");
}

The same semantic contract may then be realized on:

embedded processor
CPU
multicore CPU
GPU
FPGA
ASIC
accelerator
cluster
cloud
future target

without the grammar defining a physical machine.


---

155. Example Quantum Hardware Intent

Conceptually:

hardware QuantumTarget<RequiredQubits> {
    requires capability("quantum.compute");
    requires capability("quantum.measurement");
    requires resource.qubits >= RequiredQubits;
}

This does not mean that the grammar knows how many physical qubits exist.

The compiler determines feasibility against the selected target.


---

156. Example Topology Intent

Conceptually:

requires topology(
    connectivity = required_connectivity,
    distance = required_distance
);

The grammar preserves the requirement.

Routing determines the physical realization.


---

157. Example Memory Intent

Conceptually:

requires memory >= workload_memory;

The compiler determines whether the selected target provides sufficient memory and how the program should be mapped.


---

158. Example Capability Intent

Conceptually:

requires capability("quantum.mid_circuit_measurement");

The source does not name a vendor or physical device.

The target capability system determines whether the requirement can be satisfied.


---

159. Example Preference

Conceptually:

prefer accelerator("tensor.compute");

This is not a mandatory physical mapping.

If the preferred accelerator is unavailable, downstream policy determines what happens.


---

160. Example Physical Realization

Physical realization should occur later:

logical resource
       |
       v
target resource
       |
       v
physical resource

The realization may change between targets without changing source semantics.


---

161. Repository Maintenance Rule

When adding a new hardware technology:

1. Search existing grammar ownership.


2. Search existing capabilities.


3. Search existing resources.


4. Search existing target classes.


5. Search existing dialect mechanisms.


6. Determine whether existing syntax is sufficient.


7. Add new syntax only if necessary.


8. Define AST contract before grammar implementation.


9. Define semantic contract before implementation.


10. Define IR contract before implementation.


11. Define compiler consumers.


12. Define runtime consumers.


13. Add positive tests.


14. Add negative tests.


15. Add scalability tests.


16. Perform hard-coding audit.


17. Verify no duplicate owner exists.


18. Verify no dependency cycle exists.




---

162. Required Change Review

Every hardware grammar change must answer:

1. What syntax changed?
2. Which file owns it?
3. Why does that file own it?
4. Does another file already own the concept?
5. Does the change require a new keyword?
6. Could a qualified name represent it?
7. Could a property represent it?
8. Could a capability represent it?
9. Could a resource represent it?
10. Does it introduce a machine assumption?
11. Does it introduce a physical identifier?
12. Does it introduce a fixed capacity?
13. Does it alter the AST?
14. Does it alter semantics?
15. Does it alter IR?
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

163. Definition of a Complete Hardware Feature

A hardware feature is not complete when:

grammar parses

It is complete only when:

Specification
    +
Lexer
    +
Parser
    +
AST
    +
Semantic analysis
    +
IR
    +
Compiler
    +
Target/HAL
    +
Runtime
    +
Tests

are all accounted for.

If a downstream implementation does not exist yet, the feature must be marked:

PLANNED

or:

PARTIALLY IMPLEMENTED

rather than falsely marked production-ready.


---

164. Definition of POCO-REAF Success

The hardware subsystem succeeds when:

one source program
       |
       v
one semantic meaning
       |
       +----> tiny target
       |
       +----> CPU
       |
       +----> GPU
       |
       +----> FPGA
       |
       +----> ASIC
       |
       +----> QPU
       |
       +----> accelerator
       |
       +----> heterogeneous system
       |
       +----> distributed system
       |
       +----> simulator
       |
       +----> future target

and the target-specific implementation changes without requiring the programmer to rewrite the fundamental computation merely because hardware changes.


---

165. Final Hardware Architecture

The intended final model is:

ZAMANI PROGRAM
                               |
                               v
                        Canonical Lexer
                               |
                               v
                         Zamani Parser
                               |
                               v
                          Frontend AST
                               |
             +-----------------+-----------------+
             |                 |                 |
             v                 v                 v
          Classical         Quantum            HDL
             |                 |                 |
             +-----------------+-----------------+
                               |
                               v
                    Hardware Intent Model
                               |
             +-----------------+-----------------+
             |                 |                 |
             v                 v                 v
         Resources        Capabilities       Requirements
             |                 |                 |
             +-----------------+-----------------+
                               |
             +-----------------+-----------------+
             |                 |                 |
             v                 v                 v
        Constraints       Preferences          Hints
             |                 |                 |
             +-----------------+-----------------+
                               |
                               v
                    Canonical Semantic Model
                               |
                               v
                       Canonical Compiler IR
                               |
             +-----------------+-----------------+
             |                 |                 |
             v                 v                 v
        Classical IR      quantum::ir      HDL/Hardware
             |                 |                 |
             +-----------------+-----------------+
                               |
                               v
                         Optimization
                               |
             +-----------------+-----------------+
             |                 |                 |
             v                 v                 v
           Routing        Scheduling        Resilience
                               |
                               v
                              ZQN
                               |
                               v
                              HAL
                               |
                               v
                       Target realization
                               |
          +----------+---------+---------+----------+
          |          |                   |          |
          v          v                   v          v
         CPU        GPU                 FPGA       QPU
          |          |                   |          |
          +----------+-------------------+----------+
                               |
                               v
                    Future target classes


---

166. Non-Negotiable Invariants

The following are permanent hardware-subsystem invariants:

NO artificial hardware capacity limits
NO fixed qubit limits
NO fixed CPU limits
NO fixed GPU limits
NO fixed FPGA limits
NO fixed node limits
NO fixed memory limits
NO fixed thread limits
NO fixed tensor-rank limits
NO fixed register-width limits
NO fixed network-size limits
NO fixed device-count limits

NO physical device discovery during parsing
NO physical device enumeration in portable hardware grammar
NO physical addresses in portable hardware grammar
NO vendor lock-in
NO runtime execution in grammar
NO embedded unsafe Rust
NO embedded executable grammar actions

NO duplicate constraints grammar
NO duplicate resource grammar
NO duplicate capability grammar
NO duplicate target grammar
NO duplicate device grammar
NO duplicate quantum IR
NO duplicate QEC model
NO duplicate ZQN model
NO duplicate HDL behavior

NO circular grammar dependencies

YES canonical lexer
YES canonical names
YES canonical expressions
YES canonical types
YES resource semantics
YES capability semantics
YES requirement semantics
YES constraint semantics
YES preference semantics
YES hint semantics
YES generic parameters
YES qualified names
YES target independence
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
YES cross-domain composition
YES scalability
YES POCO-REAF


---

167. Final Rule

The hardware grammar must describe:

WHAT the computation requires
WHAT capabilities are needed
WHAT resources are required
WHAT constraints apply
WHAT relationships matter
WHAT implementation preferences exist
WHAT hardware intent is being expressed

It must not permanently describe:

WHICH physical machine
WHICH physical CPU
WHICH physical GPU
WHICH physical FPGA
WHICH physical ASIC
WHICH physical QPU
WHICH physical qubit
WHICH physical core
WHICH physical memory bank
WHICH physical address

Those decisions belong to downstream compilation and runtime realization.

Therefore the invariant is:

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
Target-independent Optimization
     |
     v
Routing / Scheduling / Resilience / ZQN
     |
     v
HAL
     |
     v
Target Realization

This is the architecture required for:

> Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever



and:

> Zamani — From Atom to Everywhere.




---

168. Final Status of This README

This README is complete when it serves as the stable contract against which every file in grammar/hardware/ can be independently implemented and verified.

It intentionally does not claim that the existing .g4 implementations are already compliant.

The next implementation work must bring each existing file into conformance with this contract, beginning with the independent leaf grammars and ending with hardware.g4 as the composition root.

No unnecessary filename renaming is required.

No parallel hardware grammar hierarchy should be created.

Existing useful functionality should be expanded and normalized.

Redundant implementations should be consolidated only after references, AST dependencies, semantic dependencies, and tests have been migrated.

The final hardware subsystem must remain:

machine-independent
resource-aware
capability-aware
target-independent
extensible
deterministic
safe
scalable
cross-domain
compiler-integrated
runtime-integrated
future-proof

while preserving the single-language Zamani architecture.

This version deliberately corrects the biggest problem in the current README: **documentation status must not outrun implementation status**. The existing repository already has the right broad hardware components, but `hardware.g4` and some specialized grammars still need normalization against the canonical lexer/parser architecture before the subsystem can legitimately be called production-ready.