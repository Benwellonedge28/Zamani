Zamani HDL Grammar

Path: "grammar/hdl/"
Domain: Hardware Description, Hardware Construction, Hardware/Software Co-Design
Language: Zamani
Canonical grammar root: "grammar/Zamani.g4"
Implementation baseline: Rust 2021, Rust 1.97 / Rust 1.97.1
Safety: Zamani-owned Rust implementation MUST use safe Rust; "unsafe" is prohibited
Primary portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

"grammar/hdl/" contains the Zamani source-language grammar components required to express portable hardware intent and hardware/software co-design.

HDL is a first-class domain of the Zamani programming language.

It is not a separate programming language, parser, lexer, AST, semantic universe, or IR.

The HDL subsystem must allow Zamani programs to express, where supported by the language specification:

- hardware modules;
- generic hardware components;
- parameters;
- ports;
- interfaces;
- signals;
- nets;
- registers;
- memories;
- arrays;
- clocks;
- clock relationships;
- resets;
- combinational behavior;
- sequential behavior;
- processes;
- state machines;
- pipelines;
- structural generation;
- instances;
- protocols;
- timing intent;
- assertions;
- verification intent;
- simulation intent;
- synthesis intent;
- physical intent;
- hardware/software boundaries;
- accelerator intent;
- hardware resource requirements;
- hardware capability requirements;
- hardware preferences;
- hardware constraints;
- hardware/quantum interfaces;
- hardware/classical interfaces;
- hardware/AI/data interfaces.

The grammar expresses source syntax.

It does not perform:

- synthesis;
- placement;
- routing;
- timing closure;
- physical design;
- FPGA programming;
- ASIC fabrication;
- target selection;
- hardware discovery;
- calibration;
- runtime scheduling;
- QEC;
- ZQN;
- physical qubit allocation.

Those responsibilities belong downstream.

---

2. Architectural Authority

The HDL subsystem participates in exactly one Zamani language architecture.

The authority chain is:

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ▼
grammar/spec/
        │
        ▼
grammar/Zamani.g4
        │
        ▼
grammar/hdl/*.g4
        │
        ▼
canonical lexer
        │
        ▼
canonical parser
        │
        ▼
domain-neutral frontend AST
        │
        ▼
semantic analysis
        │
        ├── type analysis
        ├── resource analysis
        ├── capability analysis
        ├── timing analysis
        ├── ownership/lifetime analysis where applicable
        ├── verification analysis
        └── portability analysis
        │
        ▼
canonical semantic model
        │
        ├── hardware semantic representation
        ├── classical semantics
        ├── quantum semantics
        └── hybrid semantics
        │
        ▼
canonical IR boundaries
        │
        ├── hardware/HDL IR
        ├── classical IR
        └── quantum::ir
        │
        ▼
optimization / synthesis / decomposition
        │
        ▼
scheduling / placement / routing
        │
        ▼
resilience / QEC / ZQN where applicable
        │
        ▼
HAL / backend
        │
        ▼
target realization

No file under "grammar/hdl/" may establish a competing authority.

---

3. Relationship to "grammar/Zamani.g4"

"grammar/Zamani.g4" remains the canonical ANTLR composition root.

The HDL directory does not replace it.

The root grammar owns language-wide composition and dispatch.

HDL owns the detailed syntax of HDL constructs.

Conceptually:

Zamani.g4
    │
    ├── classical
    ├── quantum
    ├── hybrid
    ├── HDL
    ├── data
    ├── AI
    ├── distributed
    ├── networking
    ├── security
    └── other Zamani domains

The root grammar must expose HDL through a single authoritative integration path.

There must not be:

Zamani.g4
ZamaniHDL.g4
AnotherHDL.g4
VendorHDL.g4

all claiming to be canonical.

---

4. This README's Role

This file is the HDL subsystem navigation and integration contract.

It defines:

- ownership;
- responsibilities;
- file relationships;
- integration boundaries;
- dependency direction;
- production-readiness requirements;
- scalability rules;
- portability rules;
- testing requirements;
- hard-coding prohibitions;
- AST/semantic/IR expectations;
- compiler/runtime integration.

It does not become another normative grammar specification.

Normative semantic definitions belong in:

grammar/spec/hdl.md

Normative architecture belongs in:

grammar/DESIGN.md

The canonical grammar composition belongs in:

grammar/Zamani.g4

The implementation-conformance reference belongs in:

grammar/grammar.md

---

5. Non-Negotiable Principles

The HDL subsystem MUST obey all of the following.

5.1 One Language

HDL is Zamani syntax.

5.2 One Canonical Root

"grammar/Zamani.g4" is the canonical grammar composition root.

5.3 One Lexical Authority

HDL uses the canonical Zamani lexical/token system.

HDL MUST NOT create a second lexer.

5.4 Domain-Neutral Frontend AST

HDL syntax MUST integrate into the existing domain-neutral frontend AST architecture.

The frontend AST MUST NOT become a physical netlist or vendor-specific representation.

5.5 Semantic Separation

The following are different concepts:

syntax
semantic meaning
requirement
constraint
capability
preference
hint
implementation
physical realization

They MUST remain distinguishable.

5.6 No Artificial Hardware Limits

The language MUST NOT impose today's finite hardware capacity as universal language limits.

5.7 Parameterization

Hardware quantities MUST be parameterizable wherever semantically appropriate.

5.8 Target Independence

Portable HDL MUST NOT accidentally select a physical target.

5.9 Explicit Target Dependence

Target-specific behavior MUST be explicit and isolated.

5.10 Safe Rust

Zamani-owned Rust implementation MUST target Rust 1.97 / 1.97.1 and MUST NOT use "unsafe".

5.11 Source Provenance

Every semantic hardware construct must remain traceable to source locations.

5.12 Determinism

Parsing and deterministic semantic operations must be reproducible for identical inputs and configuration.

---

6. POCO-REAF Contract

The HDL architecture exists in support of:

Program_Once
    ↓
Compile_Once
    ↓
Run_Everywhere
    ↓
Run_Anywhere
    ↓
Run_Forever

A Zamani hardware program should describe:

WHAT the computation is
WHAT behavior is required
WHAT resources are required
WHAT capabilities are required
WHAT constraints must be satisfied
WHAT properties must hold
WHAT optimizations are preferred

rather than unnecessarily describing:

WHICH FPGA
WHICH ASIC
WHICH physical LUT
WHICH physical BRAM
WHICH physical DSP
WHICH physical register
WHICH routing track
WHICH package pin
WHICH vendor primitive

The compiler and target environment determine physical realization.

---

7. Meaning of "Infinity"

"Infinity" means:

«The language architecture does not impose an arbitrary finite maximum on semantically parameterizable quantities.»

It does not mean physical resources are infinite.

For example:

module Pipeline<N> { ... }

may allow arbitrary "N" subject to:

- source semantics;
- compiler resources;
- available memory;
- available compilation time;
- target capabilities;
- target resources;
- explicit program constraints.

A target that cannot realize a requested design must report a capability/resource failure.

It MUST NOT redefine the language as having a smaller universal maximum.

---

8. Explicitly Prohibited Universal Limits

The HDL grammar MUST NOT introduce language-level constants such as:

MAX_MODULES
MAX_PORTS
MAX_SIGNALS
MAX_NETS
MAX_REGISTERS
MAX_MEMORIES
MAX_WIDTH
MAX_MEMORY_DEPTH
MAX_MEMORY_WIDTH
MAX_PIPELINE_STAGES
MAX_PIPELINE_DEPTH
MAX_STATES
MAX_TRANSITIONS
MAX_INSTANCES
MAX_GENERATED_INSTANCES
MAX_CLOCKS
MAX_CLOCK_DOMAINS
MAX_CHANNELS
MAX_INTERFACES
MAX_LUTS
MAX_BRAMS
MAX_DSPS
MAX_FPGAS
MAX_ASICS
MAX_ACCELERATORS
MAX_DEVICES

Nor equivalent hidden limits.

The same prohibition applies to:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_THREADS
MAX_NODES
MAX_NETWORK_SIZE
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
MAX_MEMORY
MAX_DEVICE_COUNT

where those are being used as universal language restrictions.

---

9. Program Quantities Are Not Language Limits

This is valid:

const WIDTH = 1024;
memory data : logic[WIDTH];

because "1024" is program semantics.

This is not acceptable:

// Universal language rule:
WIDTH <= 1024

merely because one implementation currently has a 1024-bit limitation.

The same distinction applies to:

- memory;
- ports;
- pipeline stages;
- instances;
- states;
- channels;
- nodes;
- clock domains;
- hardware units;
- accelerators.

---

10. Current HDL Directory

The existing repository already contains a substantial HDL subsystem.

The current architecture must retain existing filenames rather than creating competing replacements.

The directory includes components such as:

grammar/hdl/
├── README.md
├── arrays.g4
├── assertions.g4
├── clocking.g4
├── clocks.g4
├── co-design.g4
├── combinational.g4
├── generate.g4
├── hardware-dialects.g4
├── hardware-generics.g4
├── hardware-interfaces.g4
├── hardware-modules.g4
├── hdl.g4
├── interfaces.g4
├── memories.g4
├── nets.g4
├── parameters.g4
├── physical-intent.g4
├── pipelines.g4
├── ports.g4
├── processes.g4
├── protocols.g4
├── registers.g4
├── reset.g4
├── sequential.g4
├── synthesis.g4
├── simulation.g4
├── verification.g4
└── additional existing HDL grammar components

The actual repository tree is authoritative for the exact current filename set.

This README MUST NOT claim that a file exists when it does not.

Likewise, it MUST NOT prescribe a renamed replacement for an existing file merely because an older design used a different name.

---

11. File Ownership Model

Every HDL grammar file MUST have one primary responsibility.

A file MUST explicitly state:

Purpose
Owns
Does Not Own
Consumes
Produces
AST Contract
Semantic Contract
IR Contract
Integration
Tests
Hard-Coding Audit
Completion Criteria

No file may silently redefine another file's concepts.

---

12. "hdl.g4"

Purpose

"grammar/hdl/hdl.g4" is the HDL grammar composition/aggregation component.

Owns

- HDL entry/composition rules;
- HDL member dispatch;
- integration of subordinate HDL grammar components;
- common HDL structural composition.

Does not own

Detailed definitions for:

- ports;
- clocks;
- memories;
- pipelines;
- interfaces;
- protocols;
- state machines;
- generation;
- verification;
- synthesis;
- simulation;
- physical intent.

Those belong to their existing dedicated files.

Integration

"hdl.g4" integrates with:

grammar/Zamani.g4
canonical lexer/token vocabulary
core/
declarations/
expressions/
types/
modules/
hardware/
resources/
compile/
execution/
hybrid/
quantum/
validation/
tests/

"hdl.g4" MUST NOT become a second monolithic "Zamani.g4".

---

13. Lexer Integration

HDL MUST use the canonical Zamani lexer.

The HDL grammar MUST NOT contain an independent lexer.

HDL lexical concepts must be registered through the shared lexical authority.

Examples include:

module
interface
input
output
inout
signal
net
wire
register
memory
clock
reset
process
always
combinational
sequential
state
transition
pipeline
stage
instance
generate
parameter
generic
assert
assume
cover

The exact token spelling and token ownership are determined by the canonical lexical layer.

If a required HDL token is absent from the canonical lexer, the correct action is to resolve the lexical conformance gap centrally.

Do not create an HDL-specific token universe.

---

14. Token-Vocabulary Integration

The existing HDL implementation has used "tokenVocab = ZamaniTokens".

The repository must have exactly one authoritative token vocabulary.

If "ZamaniTokens" is a generated vocabulary, its generation and ownership must be explicitly documented.

If the canonical lexer instead owns the vocabulary through another generated artifact, all parser grammars must converge on that artifact.

The important invariant is:

ONE canonical lexical vocabulary

not:

HDL token vocabulary
Quantum token vocabulary
Classical token vocabulary
Vendor token vocabulary

---

15. Shared Syntax

HDL MUST reuse common Zamani syntax for:

- identifiers;
- qualified names;
- paths;
- literals;
- expressions;
- operators;
- ranges;
- types;
- generic arguments;
- attributes;
- modifiers;
- annotations;
- constraints.

HDL MUST NOT create unnecessary duplicates such as:

generalIdentifier
hdlIdentifier
quantumIdentifier
hardwareIdentifier

unless the semantic specification proves that separate lexical categories are necessary.

---

16. AST Contract

HDL grammar rules MUST map into the existing domain-neutral frontend AST architecture.

The pipeline is:

HDL source
   ↓
HDL grammar
   ↓
domain-neutral AST
   ↓
hardware semantic analysis
   ↓
hardware semantic representation
   ↓
hardware IR / downstream canonical IR

The parser MUST NOT directly create:

- FPGA placement objects;
- ASIC layout objects;
- vendor primitives;
- routing graphs;
- physical netlists;
- timing-closure state;
- bitstreams;
- calibration records.

Those belong downstream.

---

17. Source Spans

Every HDL declaration and significant construct must preserve source provenance.

Diagnostics must be able to identify, where applicable:

module
parameter
port
interface
signal
net
register
memory
clock
reset
process
state
transition
pipeline
instance
generate construct
assertion
constraint
requirement
capability

Source spans are part of the frontend contract.

---

18. Hardware Semantic Boundary

After parsing, HDL constructs become hardware semantic information.

The semantic layer owns:

- name resolution;
- type validation;
- width validation;
- signedness;
- connectivity legality;
- driver analysis;
- clock analysis;
- reset analysis;
- timing analysis;
- resource requirements;
- capability requirements;
- synthesis legality;
- verification contracts.

The grammar only establishes syntactic structure.

---

19. Hardware IR Boundary

The HDL grammar MUST NOT define a second frontend language-specific IR.

The canonical flow is:

Zamani AST
    ↓
semantic hardware model
    ↓
canonical hardware/HDL IR
    ↓
optimization
    ↓
synthesis
    ↓
scheduling
    ↓
placement
    ↓
routing
    ↓
target lowering

The exact IR implementation is owned by the compiler architecture.

The grammar only defines the source contract required to reach that representation.

---

20. Quantum Boundary

HDL may participate in quantum/hybrid hardware systems.

However:

HDL != quantum language
HDL != quantum IR
HDL != QEC
HDL != ZQN

Quantum semantics remain owned by the quantum subsystem.

Quantum constructs ultimately integrate through the existing:

quantum::ir

There must be no second competing quantum IR under "grammar/hdl/".

HDL may express:

- control hardware;
- timing;
- interfaces;
- classical feed-forward support;
- accelerator structures;
- hardware around quantum devices.

It must not redefine:

- qubits;
- quantum operations;
- quantum states;
- measurement;
- QEC;
- noise semantics.

---

21. Classical Integration

HDL and classical Zamani computation must be composable.

The architecture must support:

classical computation
        ↓
hardware accelerator
        ↓
hardware execution
        ↓
classical result

A programmer should not need to rewrite the algorithm merely because its implementation moves between:

- CPU;
- GPU;
- FPGA;
- ASIC;
- accelerator;
- other supported targets.

---

22. Hybrid Integration

HDL must integrate with hybrid programs.

Valid conceptual flows include:

classical
   ↓
hardware
   ↓
quantum
   ↓
classical

and:

software
   ↓
accelerator
   ↓
hardware
   ↓
software

The common semantic model must preserve the meaning of the entire computation.

---

23. Hardware Modules

"hardware-modules.g4" owns hardware module declaration syntax.

A module may contain:

- parameters;
- generics;
- types;
- interfaces;
- ports;
- signals;
- nets;
- registers;
- memories;
- clocks;
- resets;
- assignments;
- processes;
- state machines;
- pipelines;
- instances;
- generated structures;
- assertions;
- timing intent.

A module does not implicitly identify:

- a chip;
- an FPGA;
- an ASIC;
- a board;
- a package;
- a vendor.

---

24. Module Instances

An instance is a semantic occurrence of a module.

Instance identity is not physical resource identity.

For example:

instance compute of ComputeUnit(...)

does not mean:

physical accelerator 3

Physical placement is downstream.

Instance cardinality is not grammar-limited.

---

25. Ports

"ports.g4" owns port syntax.

Ports may describe:

- direction;
- type;
- width;
- dimensions;
- attributes;
- protocol;
- timing;
- capabilities.

Common directions include:

input
output
inout

No fixed number of ports is permitted.

---

26. Interfaces

"interfaces.g4" and "hardware-interfaces.g4" must remain coordinated.

An interface may describe:

- ports;
- signals;
- parameters;
- types;
- protocol;
- ordering;
- timing;
- capability requirements.

An interface is a semantic contract.

It does not automatically identify a physical bus.

---

27. Signals

"signals.g4" owns signal syntax.

Signals represent semantic communication or state relationships.

The semantic layer determines:

- driver legality;
- type compatibility;
- width compatibility;
- resolution;
- timing legality;
- clock-domain legality.

A signal does not automatically mean a particular physical wire.

---

28. Nets

"nets.g4" owns net syntax.

A net is a logical connectivity abstraction.

It does not identify:

- FPGA routing tracks;
- switch-box resources;
- ASIC metal;
- physical wires;
- package connections.

Multiple-driver semantics must be explicitly defined by the semantic layer.

---

29. Registers

"registers.g4" owns register syntax.

Registers may express:

- type;
- width;
- initialization;
- clock;
- edge;
- reset;
- enable;
- attributes.

The backend decides whether realization uses:

- flip-flops;
- memory;
- distributed storage;
- custom storage;
- another equivalent structure.

There is no universal register width.

---

30. Memories

"memories.g4" owns memory syntax.

Memory declarations may express:

- element type;
- width;
- depth;
- dimensions;
- read behavior;
- write behavior;
- latency;
- throughput;
- initialization;
- persistence;
- attributes.

Memory size is semantic.

Physical realization may be:

- SRAM;
- DRAM;
- BRAM;
- distributed RAM;
- register storage;
- HBM;
- external memory;
- another suitable resource.

The grammar must not decide this implicitly.

---

31. Arrays

"arrays.g4" owns hardware-array syntax.

Arrays may represent:

- repeated components;
- repeated signals;
- repeated storage;
- vector structures;
- parameterized hardware collections.

Array cardinality must be parameterizable.

Physical replication is downstream.

---

32. Clocks

"clocks.g4" owns clock declaration syntax.

"clocking.g4" owns clock relationship/clocking constructs.

Clock syntax may express:

- identity;
- frequency;
- period;
- phase;
- duty cycle;
- relationships;
- constraints;
- attributes.

It must not silently identify:

- physical oscillator;
- clock pin;
- PLL;
- clock tree;
- vendor clock primitive.

---

33. Clock Domains

A clock domain is semantic.

The language must permit arbitrarily many domains subject to available resources.

Semantic analysis must be capable of detecting relevant:

- clock-domain crossings;
- missing synchronization;
- incompatible assumptions;
- timing inconsistencies.

The grammar does not perform CDC analysis.

---

34. Resets

"reset.g4" owns reset syntax.

Reset semantics may include:

- synchronous;
- asynchronous;
- active-high;
- active-low;
- reset relationships;
- reset ordering;
- reset domains.

Reset syntax must not encode vendor-specific reset primitives.

---

35. Combinational Logic

"combinational.g4" owns combinational HDL syntax.

Semantic analysis determines:

- completeness;
- state absence;
- multiple drivers;
- combinational cycles;
- type correctness;
- width correctness.

The grammar must not encode a particular gate implementation.

---

36. Sequential Logic

"sequential.g4" owns sequential syntax.

Sequential constructs must define their timing/state relationships through the semantic contract.

The compiler may realize sequential behavior through different target technologies.

---

37. Processes

"processes.g4" owns HDL process syntax.

Processes may contain:

- declarations;
- assignments;
- conditionals;
- loops where permitted;
- function calls where permitted;
- assertions;
- state updates.

A process is a semantic hardware behavior description.

It is not automatically equivalent to a software thread.

---

38. Assignment Semantics

If Zamani distinguishes:

- continuous assignment;
- procedural assignment;
- sequential/state assignment;

those distinctions must be explicitly specified.

The semantic model must determine their observable behavior.

Backend implementation is separate.

---

39. State Machines

"state_machines.g4" owns state-machine syntax.

A state machine may have arbitrary:

- states;
- transitions;
- guards;
- actions;
- entry behavior;
- exit behavior;
- reset behavior.

There is no universal maximum state count.

State names do not identify physical registers.

Semantic validation should identify relevant:

- undefined states;
- impossible transitions;
- unreachable states;
- conflicting transitions;
- missing reset behavior.

---

40. Pipelines

"pipelines.g4" owns pipeline syntax.

Pipeline constructs may describe:

- stages;
- stage behavior;
- dependencies;
- latency;
- throughput;
- buffering;
- valid/ready semantics;
- timing requirements.

There is no universal pipeline-stage limit.

---

41. Pipeline Transformation

The compiler may transform pipelines through:

- retiming;
- stage balancing;
- replication;
- resource sharing;
- stage insertion;
- stage removal.

Such transformations must preserve all semantic contracts.

If latency or throughput is a hard requirement, it must not be silently violated.

---

42. Generate

"generate.g4" owns structural generation syntax.

Generation may produce:

- instances;
- signals;
- memories;
- logic;
- interfaces;
- pipelines;
- other permitted hardware structures.

Generated cardinality can depend on symbolic parameters.

There is no language-level maximum.

Generate expansion must be deterministic for identical source, inputs, and configuration unless explicit nondeterminism is part of the language semantics.

---

43. Generate Safety

Generate constructs MUST NOT become an unrestricted compiler escape hatch.

They must not silently permit:

- arbitrary filesystem modification;
- arbitrary process execution;
- arbitrary network access;
- uncontrolled recursion;
- infinite expansion;
- hidden target-specific behavior.

Compile-time computation remains subject to Zamani's metaprogramming and compile-time execution policies.

---

44. Hardware Generics

"hardware-generics.g4" owns generic hardware syntax.

Generics may parameterize:

- widths;
- depths;
- dimensions;
- types;
- topology;
- interfaces;
- pipeline structure;
- memory organization;
- algorithmic structure.

Generic constraints must be semantic.

Example:

requires width > 0
requires depth > 0
requires capability("memory")

---

45. Hardware Parameters

"parameters.g4" owns hardware parameter syntax.

Parameters may represent:

- counts;
- dimensions;
- widths;
- depths;
- latency;
- throughput;
- timing;
- topology;
- protocol choices;
- resource requirements.

Parameters are not compiler-wide maximums.

---

46. Timing

"timing.g4" owns timing syntax.

Timing may express:

- latency;
- throughput;
- period;
- frequency;
- ordering;
- phase;
- timing relationships.

Timing quantities must use Zamani's canonical unit/duration model.

No fixed timing universe is allowed.

---

47. Timing Is Not Physical Routing

A timing requirement such as:

requires latency <= L

does not tell the compiler:

use physical path X

Timing closure is a downstream responsibility.

---

48. Protocols

"protocols.g4" owns hardware protocol syntax.

Protocols may describe:

- request/response;
- handshake;
- streaming;
- valid/ready;
- packet transfer;
- message transfer;
- memory-mapped behavior;
- custom semantic protocols.

Protocols are contracts.

Physical implementation is downstream.

---

49. Assertions

"assertions.g4" owns HDL assertion syntax.

Assertions must integrate with the common validation/verification architecture.

They must preserve source provenance.

The semantic model must distinguish:

- safety;
- liveness where supported;
- assumptions;
- guarantees;
- coverage.

An assertion must not silently become a synthesis directive unless explicitly defined as such.

---

50. Verification

"verification.g4" owns HDL verification-intent syntax.

Verification constructs must be classified as appropriate:

synthesizable
simulation-only
verification-only
target-dependent

A simulation-only construct must not accidentally enter synthesis semantics.

---

51. Simulation

"simulation.g4" owns simulation intent.

Simulation may express:

- stimulus;
- timing;
- testbench behavior;
- monitors;
- waveforms;
- verification scenarios.

Simulation semantics must remain distinct from hardware synthesis semantics.

---

52. Synthesis

"synthesis.g4" owns general synthesis-intent syntax.

It may express semantic intent such as:

- pipeline preference;
- resource sharing;
- parallelism;
- latency preference;
- throughput preference;
- area preference;
- energy preference.

A preference is not a hard requirement unless the language explicitly marks it as such.

---

53. Physical Intent

"physical-intent.g4" owns explicit physical implementation intent.

Physical intent is the boundary between portable hardware semantics and deliberately target-dependent implementation.

Physical intent MUST be:

- explicit;
- namespaced;
- attributable;
- versionable;
- target-aware;
- compatibility-aware.

It must not leak into portable HDL by default.

For example:

portable:
    requires capability("high_bandwidth_memory")

target-specific:
    bind memory to target_resource(...)

The second form must be clearly distinguishable from the first.

---

54. Hardware Dialects

"hardware-dialects.g4" owns controlled hardware dialect syntax.

A dialect must identify:

- dialect name;
- version;
- namespace;
- semantic owner;
- syntax extension;
- AST mapping;
- semantic mapping;
- IR mapping;
- compatibility requirements;
- target scope.

Vendor-specific syntax must not silently become core Zamani syntax.

---

55. Co-Design

"co-design.g4" owns hardware/software co-design syntax.

It must permit explicit boundaries between:

software
hardware
accelerator
data movement
communication
timing
resource requirements

A co-design declaration must preserve the common Zamani semantic model.

It must not create a separate programming language.

---

56. Resource Integration

HDL integrates with:

grammar/resources/
grammar/hardware/
grammar/compile/
grammar/execution/

The following concepts MUST remain separate:

requirement
constraint
capability
preference
hint
target
placement

For example:

requires capability("parallel.compute")

is not equivalent to:

use accelerator 3

The first is portable capability intent.

The second is implementation-specific placement.

---

57. Hardware Capability Model

Capabilities must describe what a target can provide.

Examples:

capability("parallel.compute")
capability("high_bandwidth_memory")
capability("reconfigurable.logic")
capability("hardware.multiply")
capability("streaming")
capability("quantum.control")

Capability identifiers are semantic names.

The grammar does not contain the target's actual capability database.

---

58. Resource Requirements

Resource requirements may express quantities or properties.

Examples:

requires memory >= required_memory
requires bandwidth >= required_bandwidth
requires qubits >= required_qubits
requires capability("hardware.multiply")

Such requirements are not universal hardware limits.

They are program contracts.

---

59. Resource Negotiation

If a program permits alternative implementations, it may explicitly express preferences or negotiation.

For example:

prefer throughput
prefer low_latency
prefer energy_efficiency

A preference may be unsatisfied without invalidating the program.

A requirement cannot be silently downgraded.

---

60. No Silent Semantic Downgrade

The compiler MUST NOT silently change a required:

- width;
- latency;
- throughput;
- precision;
- memory capacity;
- timing property;
- protocol property;
- reliability guarantee;
- correctness condition.

If approximation or negotiation is permitted, it must be explicitly represented by source semantics.

---

61. Type Integration

HDL uses the canonical Zamani type system.

It must support, where specified:

- scalar types;
- integer types;
- signed/unsigned types;
- bit/logic types;
- arrays;
- generic types;
- structured types;
- parameterized types;
- domain-specific types.

HDL MUST NOT create a second general-purpose type system.

---

62. Width Semantics

Width is semantic where observable.

Widths may be:

- literal;
- symbolic;
- generic;
- computed;
- configuration-derived;
- target-selected only where explicitly permitted.

The grammar MUST NOT impose a universal maximum width.

Semantic analysis must validate:

- invalid widths;
- incompatible widths;
- invalid slices;
- invalid concatenations;
- incompatible assignments.

---

63. Signedness

Signedness must be explicit or deterministically derived according to the canonical type system.

It affects:

- arithmetic;
- comparison;
- extension;
- truncation;
- conversion.

Target representation must preserve observable semantics.

---

64. Bit-Level Semantics

HDL should reuse Zamani's common operators where possible:

&
|
^
~
<<
>>

and other canonical operators.

HDL-specific duplicate operator vocabularies must not be introduced without specification justification.

---

65. Simulation Values

If Zamani supports four-state or unknown values such as:

0
1
X
Z

their semantics must be explicit.

Simulation unknowns must not automatically be interpreted as physical hardware states.

Simulation semantics and synthesis semantics must remain distinguishable.

---

66. Memory and Physical Storage

A source-level memory is semantic.

It does not imply:

BRAM
SRAM
DRAM
HBM
LUTRAM
register file
cache

The compiler may choose an implementation satisfying the semantic contract.

---

67. Hardware Topology

Logical topology may be described using:

- graphs;
- meshes;
- trees;
- rings;
- networks;
- parameterized connectivity.

Topology cardinality must not be grammar-limited.

Physical topology is determined downstream.

---

68. Distributed Hardware

HDL may integrate with "grammar/distributed/".

Hardware nodes and communication structures remain semantic abstractions.

The language must not impose:

MAX_NODES
MAX_LINKS
MAX_DEVICES

as universal limits.

---

69. Networking Integration

Hardware networking constructs integrate with:

grammar/networking/

Networking owns:

- endpoints;
- addresses;
- protocols;
- communication semantics.

HDL owns the hardware representation of the interface.

---

70. AI/Data Integration

Hardware accelerators for AI/data workloads must integrate with:

grammar/ai/
grammar/data/

AI semantics remain in AI.

Data semantics remain in data.

HDL describes hardware structure and implementation intent.

The grammar must not hard-code:

- CUDA;
- ROCm;
- TensorRT;
- a particular neural-network framework;
- a particular accelerator vendor.

---

71. Security Integration

Hardware security constructs integrate with:

grammar/security/

Security semantics remain explicit.

Examples include:

- isolation;
- secure interfaces;
- cryptographic accelerators;
- trusted boundaries;
- secure computation.

The HDL grammar must not duplicate the security type/policy system.

---

72. Interoperability

HDL must integrate with:

grammar/interoperability/

for formats and external ecosystems where supported.

Examples may include:

- Verilog/SystemVerilog interoperability;
- VHDL interoperability;
- HDL netlist exchange;
- QIR-related systems;
- OpenQASM-related systems;
- LLVM/MLIR-related toolchains.

External formats are interoperability boundaries.

They do not become the canonical Zamani semantic model.

---

73. Rust Requirements

All Zamani-owned Rust implementation associated with HDL MUST:

- target Rust 1.97 / Rust 1.97.1;
- use Rust 2021;
- compile without "unsafe";
- use structured errors;
- preserve source spans;
- avoid hard-coded target capacities;
- remain deterministic where required;
- use bounded/resource-aware processing where necessary;
- avoid target-specific assumptions in portable semantic structures.

---

74. "unsafe" Prohibition

No Zamani-owned HDL frontend, parser integration, semantic implementation, validation code, test infrastructure, or hardware-domain compiler code may contain:

unsafe

The architecture must use safe abstractions.

External dependencies may have their own internal implementation details, but Zamani-owned code must not require "unsafe" blocks.

---

75. Runtime and Compiler Resource Limits

Implementation resource safeguards are permitted.

For example, a compiler may stop because it has exhausted:

- memory;
- compilation time;
- recursion budget;
- expansion budget;
- operating-system resources.

Such failures must be reported as implementation/resource failures.

They must not be documented as language limitations.

Correct distinction:

resource exhausted while compiling

not:

Zamani supports only N modules

---

76. Generate/Elaboration Limits

Elaboration may require operational safeguards.

Such safeguards must be:

- configurable;
- diagnosable;
- separate from language semantics;
- documented as implementation policy.

They must not become hidden language maxima.

---

77. Diagnostics

HDL diagnostics must distinguish:

lexical error
syntax error
name-resolution error
type error
width error
semantic hardware error
resource requirement failure
capability failure
timing failure
verification failure
synthesis failure
placement failure
routing failure
target incompatibility
compiler resource exhaustion

These categories must not be collapsed into generic parser errors.

---

78. Error Recovery

Parser error recovery must not produce silently valid hardware semantics from malformed input.

The parser may recover sufficiently to report multiple diagnostics.

Compilation must not proceed to hardware realization on invalid semantic state.

---

79. Determinism

For identical:

- source;
- compiler version;
- language version;
- dialect versions;
- configuration;
- target description;

deterministic parsing and semantic analysis must produce deterministic results.

Where optimization is nondeterministic internally, externally observable semantics must remain deterministic unless explicitly specified otherwise.

---

80. Reproducibility

HDL compilation should preserve:

- source version;
- grammar version;
- dialect versions;
- compiler version;
- target description;
- relevant configuration;
- generated-artifact provenance.

This supports POCO-REAF and long-term reproducibility.

---

81. Versioning

HDL features must be versioned through the common Zamani compatibility architecture.

Do not introduce:

HDL v2 language
HDL v3 language

as competing languages.

Instead use:

Zamani language version
HDL feature version
dialect version

where appropriate.

---

82. Compatibility

A change to HDL syntax must be evaluated against:

lexer
parser
AST
semantic analysis
IR
compiler
runtime
tests
documentation
interoperability

Backward-compatible additions should not change existing valid program meaning.

Breaking changes require explicit versioning/migration policy.

---

83. Deprecation

Deprecated HDL constructs must:

- be documented;
- have a replacement path;
- generate diagnostics where appropriate;
- remain traceable to a language version;
- not silently acquire a different meaning.

---

84. AST/Grammar Completion Contract

An HDL grammar file is not complete merely because ANTLR accepts it.

For every grammar feature, the following must already be known:

grammar rule
    ↓
token dependencies
    ↓
AST representation
    ↓
semantic representation
    ↓
IR mapping
    ↓
compiler consumers
    ↓
target consumers
    ↓
diagnostics
    ↓
tests

No "we will decide the AST later" implementation is production-complete.

---

85. Independent-File Completion Contract

Every HDL file must be independently completable.

Its documentation must state:

Purpose

What the file defines.

Owns

The exact syntax responsibility.

Does Not Own

Everything intentionally delegated elsewhere.

Inputs

Tokens, common grammar rules, shared types, expressions, etc.

Outputs

Grammar rules exposed to the HDL composition root.

AST Contract

The expected domain-neutral representation.

Semantic Contract

The required semantic interpretation.

IR Contract

How the semantic construct reaches the canonical downstream representation.

Compiler Integration

Which compiler stages consume it.

Runtime/Target Integration

Which downstream systems may realize it.

Cross-Domain Integration

Classical, quantum, hybrid, AI, data, networking, distributed, security, etc.

Tests

Positive, negative, boundary, scalability, compatibility, determinism.

Hard-Coding Audit

Evidence that no artificial hardware capacity has been encoded.

Completion Criteria

A concrete checklist.

This prevents later files from forcing unnecessary redesign.

---

86. Production Test Structure

The HDL test suite must cover at least:

tests/lexical/
tests/syntax/
tests/types/
tests/hardware/
tests/hdl/
tests/quantum/
tests/hybrid/
tests/classical/
tests/resources/
tests/compile/
tests/execution/
tests/interoperability/
tests/validation/

Within HDL tests:

positive/
negative/
boundary/
scalability/
determinism/
compatibility/
diagnostics/

---

87. Positive Tests

Positive tests must cover:

- minimal modules;
- parameterized modules;
- generic modules;
- ports;
- interfaces;
- signals;
- nets;
- registers;
- memories;
- clocks;
- resets;
- combinational logic;
- sequential logic;
- processes;
- state machines;
- pipelines;
- generate;
- protocols;
- assertions;
- simulation;
- synthesis intent;
- co-design;
- resource requirements;
- capability requirements;
- hybrid integration.

---

88. Negative Tests

Negative tests must cover:

- invalid declarations;
- undefined identifiers;
- invalid widths;
- incompatible widths;
- illegal drivers;
- invalid clock usage;
- invalid reset usage;
- invalid state transitions;
- malformed protocols;
- impossible constraints;
- invalid generic parameters;
- illegal generate expansion;
- unsupported target requirements.

---

89. Scalability Tests

Scalability tests must intentionally cover increasing quantities of:

- modules;
- ports;
- signals;
- nets;
- registers;
- memories;
- states;
- transitions;
- pipeline stages;
- instances;
- generated instances;
- interfaces;
- clock domains;
- communication channels.

The tests must verify that the language architecture does not impose artificial finite maxima.

---

90. Boundary Tests

Boundary tests must include:

- smallest valid width;
- symbolic width;
- large width;
- parameterized dimensions;
- zero/empty cases where legal;
- invalid zero cases where illegal;
- very large generated structures;
- deeply nested structures;
- large state machines;
- large pipelines;
- large memories.

The test harness must distinguish language invalidity from implementation resource exhaustion.

---

91. Hard-Coding Audit

The HDL grammar must be audited for:

MAX_*
fixed hardware counts
fixed widths
fixed depths
fixed stage counts
fixed port counts
fixed device counts
fixed topology sizes
vendor-specific assumptions
physical-resource assumptions

The audit must cover:

*.g4
*.md
test fixtures
generated grammar
semantic contracts
validation rules

The audit must distinguish legitimate program constants from forbidden universal compiler limits.

---

92. Examples of Correct Portability

Parameterized width

module Compute<WIDTH> {
    input  data : logic[WIDTH];
}

Parameterized memory

memory data : logic[WORD_WIDTH][DEPTH];

Parameterized pipeline

pipeline<STAGES> {
    ...
}

Capability requirement

requires capability("parallel.compute")

Resource requirement

requires memory >= required_memory

Timing requirement

requires latency <= required_latency

These express program intent.

---

93. Examples of Incorrect Universal Hard-Coding

These must not be universal language restrictions:

MAX_WIDTH = 32
MAX_MEMORY_DEPTH = 65536
MAX_PIPELINE_STAGES = 32
MAX_PORTS = 128
MAX_MODULES = 1024
MAX_CLOCKS = 16
MAX_DEVICES = 8
MAX_NODES = 64

Likewise, the grammar must not implicitly assume:

32-bit registers
64-bit addresses
fixed FPGA LUT counts
fixed BRAM counts
fixed DSP counts
fixed clock counts
fixed bus widths

---

94. Portable vs Physical Intent

The architectural distinction is:

Portable semantic intent
        ↓
resource/capability analysis
        ↓
implementation strategy
        ↓
physical realization

Portable:

requires capability("high_bandwidth_memory")

Target-specific:

bind memory to target_resource(...)

Both may exist in Zamani, but they must never be confused.

---

95. Optimization Boundary

HDL syntax describes semantics.

Optimization may transform the representation through:

- constant propagation;
- dead logic elimination;
- common-subexpression elimination;
- resource sharing;
- replication;
- retiming;
- pipelining;
- memory transformation;
- parallelization.

Optimization must preserve declared semantics.

---

96. Synthesis Boundary

Synthesis converts semantic hardware intent into an implementation representation.

It may choose:

- gates;
- LUTs;
- registers;
- memories;
- DSP resources;
- custom structures;
- target-specific primitives.

Those choices are not the responsibility of the source grammar.

---

97. Placement Boundary

Placement maps logical implementation structures to target resources.

The HDL grammar does not perform placement.

---

98. Routing Boundary

Routing maps logical connections to target interconnect.

The HDL grammar does not perform routing.

Logical connectivity belongs in HDL.

Physical connectivity belongs downstream.

---

99. Scheduling Boundary

Timing and ordering intent may be expressed by HDL.

Scheduling determines a realizable execution/implementation schedule.

HDL MUST NOT duplicate scheduler implementation.

---

100. Resilience Boundary

HDL may express reliability requirements.

Resilience analysis and implementation remain downstream.

The existing Zamani resilience model must remain authoritative.

---

101. Quantum/QEC/ZQN Boundary

For hybrid quantum hardware:

HDL
 ↓
hardware semantics
 ↓
hybrid semantics
 ↓
quantum::ir
 ↓
optimization
 ↓
routing
 ↓
scheduling
 ↓
QEC/resilience/ZQN
 ↓
HAL
 ↓
target

HDL MUST NOT implement QEC.

HDL MUST NOT implement ZQN.

HDL MUST NOT create another quantum IR.

---

102. Security Boundary

HDL security requirements integrate with "grammar/security/".

The source may express semantic requirements.

The security subsystem determines:

- policy;
- authorization;
- trust;
- cryptographic semantics;
- isolation.

The HDL grammar must not duplicate the security policy engine.

---

103. Distributed Boundary

HDL may describe hardware participating in distributed systems.

Distributed semantics remain owned by "grammar/distributed/".

Hardware topology and distributed placement must remain separable.

---

104. Data Boundary

HDL data representation must preserve the semantics of Zamani data types.

A tensor, stream, matrix, or record must not lose semantic type information merely because it enters hardware.

---

105. AI Boundary

AI semantics remain owned by "grammar/ai/".

HDL describes accelerator realization.

The grammar must not turn every AI framework primitive into a hardware keyword.

---

106. Dialect Boundary

Dialect extensions must never silently modify core HDL semantics.

Each dialect must be:

identified
versioned
namespaced
validated
mapped to AST
mapped to semantics
mapped to IR
tested

---

107. Interoperability Boundary

External HDL formats may be imported/exported.

The translation path is:

external format
    ↓
interoperability adapter
    ↓
Zamani semantic model
    ↓
canonical Zamani representation

Not:

external format
    ↓
new competing Zamani IR

---

108. Performance

The implementation should support scalable compilation through:

- modular parsing;
- incremental compilation where available;
- caching;
- deterministic elaboration;
- reusable semantic results;
- efficient graph representations;
- resource-aware expansion.

Performance optimization must not change language semantics.

---

109. Memory Safety

Zamani-owned implementation must use safe Rust.

Large hardware descriptions must use scalable structures rather than unnecessary recursive runtime structures.

Resource-aware processing may reject pathological compilation workloads, but that must be reported as resource exhaustion rather than a language limit.

---

110. Security of Compilation

The HDL compiler must treat generated/elaborated hardware descriptions as potentially hostile or resource-intensive input.

Controls may be applied to:

- generate expansion;
- compile-time evaluation;
- parser workload;
- semantic analysis;
- graph construction;
- verification workload.

These are implementation security policies.

They are not language-level hardware maxima.

---

111. No Hidden Vendor Lock-In

The core HDL grammar must not silently depend on:

- Xilinx-only constructs;
- AMD-only constructs;
- Intel FPGA-only constructs;
- NVIDIA-only constructs;
- ASIC foundry-specific primitives;
- one board;
- one CPU;
- one GPU;
- one QPU;
- one accelerator.

Vendor functionality belongs in explicit dialects/interoperability/target layers.

---

112. Hardware Abstraction Layer

The HAL is downstream.

The relationship is:

Zamani HDL
    ↓
semantic hardware model
    ↓
compiler/IR
    ↓
HAL
    ↓
target

HDL MUST NOT directly instantiate HAL objects.

---

113. Target Selection

Portable HDL should normally avoid choosing a physical target.

Target selection belongs to:

grammar/compile/
grammar/hardware/
deployment configuration
compiler policy
target descriptions

Explicit target-specific source constructs must remain clearly marked as such.

---

114. Target Capability Failure

If a target cannot satisfy:

required capability
required resource
required timing
required width
required precision
required protocol
required reliability

the compiler must report a clear failure.

It must not silently rewrite the source semantics.

---

115. Approximation

Approximation is permitted only where explicitly represented.

For example:

prefer error <= bound

may permit optimization within the declared approximation contract.

Exact semantics must remain exact unless the source explicitly permits approximation.

---

116. Physical Constants

Physical quantities may be program semantics where meaningful:

frequency
period
latency
energy
power
temperature
area
bandwidth

But a current target's physical capacity must not become a universal grammar restriction.

---

117. Generic Hardware Reuse

The preferred architecture is:

generic hardware definition
        ↓
constraints
        ↓
specialization
        ↓
semantic validation
        ↓
implementation

not:

copy source
edit for every FPGA
edit again for every ASIC
edit again for every accelerator

This is central to POCO-REAF.

---

118. Hardware/Software Co-Design Contract

A single Zamani program may contain:

software algorithm
hardware accelerator
data movement
memory behavior
communication
timing intent
verification properties
deployment constraints

The language must preserve one semantic program model.

---

119. Production Completion Criteria

The HDL subsystem is production-ready only when:

Architecture

- [ ] "grammar/Zamani.g4" remains the canonical composition root.
- [ ] No competing HDL root grammar exists.
- [ ] No competing lexer exists.
- [ ] Existing HDL filenames remain authoritative.
- [ ] Responsibilities are non-overlapping.

Lexical

- [ ] Every HDL token comes from the canonical lexical system.
- [ ] No duplicate token universe exists.
- [ ] Keyword spelling is centralized.
- [ ] Operators are shared where appropriate.

Syntax

- [ ] All HDL constructs have defined grammar ownership.
- [ ] Ambiguities are resolved.
- [ ] Precedence interactions are defined.
- [ ] Error recovery is defined.

AST

- [ ] Every syntax construct has an AST contract.
- [ ] AST remains domain-neutral.
- [ ] Source spans are preserved.
- [ ] No physical implementation objects leak into the frontend AST.

Semantics

- [ ] Module semantics defined.
- [ ] Port semantics defined.
- [ ] Signal semantics defined.
- [ ] Net semantics defined.
- [ ] Register semantics defined.
- [ ] Memory semantics defined.
- [ ] Clock semantics defined.
- [ ] Reset semantics defined.
- [ ] Timing semantics defined.
- [ ] Process semantics defined.
- [ ] State-machine semantics defined.
- [ ] Pipeline semantics defined.
- [ ] Generate semantics defined.
- [ ] Protocol semantics defined.
- [ ] Assertion semantics defined.
- [ ] Simulation semantics defined.
- [ ] Synthesis-intent semantics defined.
- [ ] Physical-intent semantics defined.
- [ ] Co-design semantics defined.

Resources

- [ ] Requirements are distinct from constraints.
- [ ] Constraints are distinct from preferences.
- [ ] Preferences are distinct from hints.
- [ ] Capabilities are distinct from physical resources.
- [ ] No artificial capacity limits exist.

Portability

- [ ] Parameterization is supported.
- [ ] Generic hardware is supported.
- [ ] Target independence is preserved.
- [ ] Target-specific behavior is explicit.
- [ ] Resource failures are distinguishable from syntax errors.

Quantum

- [ ] HDL integrates with hybrid computing.
- [ ] Quantum semantics remain in the quantum subsystem.
- [ ] "quantum::ir" remains the canonical quantum boundary.
- [ ] No second quantum IR exists.

Compiler

- [ ] Semantic hardware representation is defined.
- [ ] IR integration is defined.
- [ ] Optimization integration is defined.
- [ ] Synthesis integration is defined.
- [ ] Scheduling integration is defined.
- [ ] Placement integration is defined.
- [ ] Routing integration is defined.
- [ ] HAL integration is defined.

Rust

- [ ] Rust 1.97 / 1.97.1 is supported.
- [ ] Rust 2021 is used.
- [ ] Zamani-owned Rust contains no "unsafe".
- [ ] Structured diagnostics are used.
- [ ] Source provenance is preserved.

Validation

- [ ] Positive tests exist.
- [ ] Negative tests exist.
- [ ] Boundary tests exist.
- [ ] Scalability tests exist.
- [ ] Determinism tests exist.
- [ ] Compatibility tests exist.
- [ ] Hard-coding audit passes.
- [ ] Repository-wide grammar conformance passes.

---

120. Definition of "Done" for an HDL File

A file under "grammar/hdl/" is considered complete only when all applicable items below are resolved:

1. Purpose
2. Ownership
3. Non-ownership
4. Token dependencies
5. Grammar dependencies
6. Composition dependency
7. AST mapping
8. Source-span mapping
9. Semantic mapping
10. IR mapping
11. Compiler consumers
12. Runtime/target consumers
13. Resource integration
14. Capability integration
15. Cross-domain integration
16. Diagnostics
17. Versioning
18. Compatibility
19. Positive tests
20. Negative tests
21. Boundary tests
22. Scalability tests
23. Determinism tests
24. Hard-coding audit
25. Security/resource-exhaustion analysis
26. Completion criteria

Once these are satisfied, another HDL file being implemented later must not require the completed file to be redesigned merely to discover its integration contract.

---

121. Required Integration Direction

The dependency direction must remain:

shared lexer/tokens
        ↓
shared core grammar
        ↓
shared expressions/types/declarations
        ↓
HDL grammar components
        ↓
grammar/hdl/hdl.g4
        ↓
grammar/Zamani.g4
        ↓
frontend parser
        ↓
domain-neutral AST
        ↓
semantic analysis
        ↓
hardware semantic model
        ↓
canonical IR
        ↓
compiler
        ↓
optimization/synthesis
        ↓
scheduling
        ↓
placement
        ↓
routing
        ↓
HAL
        ↓
target

No downstream layer should become a hidden dependency of the grammar.

---

122. What Must Never Happen

Do not:

- create a second "Zamani.g4";
- create a second HDL root grammar;
- create a second lexer;
- create a second AST universe;
- create a second quantum IR;
- enumerate every FPGA primitive as core syntax;
- enumerate every CPU instruction as HDL syntax;
- hard-code today's hardware limits;
- hard-code fixed register widths;
- hard-code fixed memory sizes;
- hard-code fixed pipeline depth;
- hard-code fixed node counts;
- hard-code fixed topology;
- make vendor APIs core language syntax;
- make QEC part of HDL semantics;
- make ZQN part of HDL semantics;
- make HAL objects parser constructs;
- let "Zamani-Grammar.md" silently introduce syntax;
- let "grammar/grammar.md" become a second authority.

---

123. Final HDL Architecture

The complete production architecture is:

                       Zamani Source
                             │
                             ▼
                     grammar/Zamani.g4
                             │
                             ▼
                    Canonical Lexer
                             │
                             ▼
                    Canonical Parser
                             │
                             ▼
                 Domain-Neutral Frontend AST
                             │
                             ▼
                   Structural Validation
                             │
                             ▼
                    Semantic Analysis
                             │
          ┌──────────────────┼──────────────────┐
          │                  │                  │
          ▼                  ▼                  ▼
        Types            Resources         Capabilities
          │                  │                  │
          └──────────────────┼──────────────────┘
                             │
                             ▼
                  Hardware Semantic Model
                             │
             ┌───────────────┼────────────────┐
             │               │                │
             ▼               ▼                ▼
         Verification    Optimization      Analysis
             │               │                │
             └───────────────┼────────────────┘
                             │
                             ▼
                    Canonical Hardware IR
                             │
                             ▼
                         Synthesis
                             │
                             ▼
                        Scheduling
                             │
                             ▼
                         Placement
                             │
                             ▼
                          Routing
                             │
                             ▼
                     Target Lowering
                             │
          ┌──────────────────┼──────────────────┐
          │                  │                  │
          ▼                  ▼                  ▼
         CPU                GPU                FPGA
          │                  │                  │
          ├──────────────────┼──────────────────┤
          │                  │                  │
          ▼                  ▼                  ▼
         ASIC           Accelerator       Future Target

For hybrid quantum systems:

                  Zamani HDL / Hybrid
                          │
                          ▼
                 Hardware Semantics
                          │
                          ▼
                    quantum::ir
                          │
                          ▼
                     Optimization
                          │
              ┌───────────┼───────────┐
              ▼           ▼           ▼
           Routing     Scheduling    QEC
                                      │
                                      ▼
                                     ZQN
                                      │
                                      ▼
                                     HAL
                                      │
                                      ▼
                               Quantum Target

---

124. Final Rule

The fundamental Zamani HDL rule is:

«HDL describes portable computational and hardware intent. The compiler and target system determine how that intent becomes an actual physical implementation.»

Therefore:

WHAT
    → Zamani source

WHY
    → semantic contracts

WHICH CAPABILITY
    → capability analysis

WHICH RESOURCE
    → resource analysis

HOW
    → optimization / synthesis

WHEN
    → timing / scheduling

WHERE
    → placement / routing / deployment

WHICH DEVICE
    → target realization

WHICH PHYSICAL RESOURCE
    → backend/HAL

WHICH QUBIT
    → quantum routing/HAL

WHICH QEC STRATEGY
    → QEC/resilience

WHICH NOISE/FAULT MODEL
    → ZQN

The HDL subsystem is production-ready only when that separation remains intact from:

source
→ lexer
→ parser
→ AST
→ semantic analysis
→ canonical IR
→ optimization
→ synthesis
→ scheduling
→ placement
→ routing
→ resilience/QEC/ZQN where applicable
→ HAL
→ target

while preserving:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

without turning the capabilities or resource limits of today's hardware into permanent limits of the Zamani language.