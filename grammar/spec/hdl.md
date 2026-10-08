Zamani HDL Specification

File: "grammar/spec/hdl.md"
Language: Zamani
Domain: Hardware Description, Hardware/Software Co-Design, Reconfigurable Computing, Accelerators, Digital Systems, Physical Implementation Intent
Status: Normative Production Specification
Specification Role: Canonical HDL semantic, ownership, integration, portability, scalability, and conformance contract
Implementation Baseline: Rust 1.97 or later, Rust 2021 edition
Implementation Safety: Zamani-owned Rust implementation MUST NOT use "unsafe"
Canonical Language Composition Root: "grammar/Zamani.g4"
Canonical HDL Grammar Root: "grammar/hdl/hdl.g4"
Canonical Lexical Authority: "grammar/antlr/ZamaniLexer.g4"
Canonical Token Registry: "grammar/lexer/tokens.g4"
Canonical AST: Existing domain-neutral Zamani frontend AST
Canonical Semantic Boundary: Domain-neutral semantic model followed by hardware/domain lowering
Quantum Boundary: Quantum semantics MUST converge on the existing "quantum::ir"; this specification MUST NOT define a second quantum IR.

---

1. Purpose

This document is the normative specification for Hardware Description Language capabilities in Zamani.

Zamani HDL is a first-class computational domain within Zamani.

It MUST support expressing hardware and hardware/software intent for systems ranging from extremely small implementations to arbitrarily large realizations permitted by the available resources.

The language MUST allow one semantic program to be considered for different realizations without requiring source-level rewriting solely because the implementation target changes.

The governing architectural objective is:

Program Once → Compile Once → Run Everywhere, Anywhere, Forever (POCO-REAF).

For HDL this means:

Zamani HDL source
        │
        ▼
portable hardware intent
        │
        ▼
domain-neutral AST
        │
        ▼
semantic validation
        │
        ├── type analysis
        ├── effect analysis
        ├── capability analysis
        ├── resource analysis
        ├── contract analysis
        ├── policy analysis
        └── provenance
        │
        ▼
hardware semantic model
        │
        ▼
canonical IR / domain lowering
        │
        ▼
optimization
        │
        ▼
scheduling
        │
        ▼
resource allocation
        │
        ▼
placement
        │
        ▼
routing
        │
        ▼
technology / target lowering
        │
        ├── FPGA
        ├── ASIC
        ├── accelerator
        ├── CPU-integrated hardware
        ├── GPU-adjacent hardware
        ├── reconfigurable hardware
        ├── simulator
        └── future hardware

The source program describes meaning and intent.

The implementation determines realization.

---

2. Core Principle

The fundamental HDL rule is:

«Describe what hardware computation means and what properties it requires; do not accidentally encode the physical organization of one particular implementation.»

Therefore source-level HDL MUST distinguish:

semantic intent

from:

physical realization

For example, a source program MAY express:

requires capability("parallel.compute");
requires capability("memory");
requires capability("high_speed.interconnect");
requires memory >= required_memory;
requires throughput >= required_throughput;
requires latency <= required_latency;

The source MUST NOT silently imply:

use FPGA device 0
use LUT 17
use BRAM 3
use DSP 8
use routing channel 7
use physical pin 42
use vendor primitive X

unless the program explicitly enters a target-specific or physical-intent dialect.

Portable HDL MUST remain independent of such implementation identity.

---

3. Normative Language

The following terms are normative.

- MUST — mandatory.
- MUST NOT — prohibited.
- SHOULD — recommended unless technically justified otherwise.
- SHOULD NOT — discouraged unless technically justified.
- MAY — permitted.
- INTENT — semantic declaration of desired behavior or implementation property.
- REQUIREMENT — condition required for a valid realization.
- CAPABILITY — ability offered by a realization.
- RESOURCE — available computational or physical capacity.
- CONSTRAINT — condition a valid realization must satisfy.
- PREFERENCE — non-semantic optimization preference.
- HINT — optional implementation guidance.
- POLICY — governing rule controlling permissible realization or execution.
- CONTRACT — semantic condition that must hold.
- PROVENANCE — traceable origin and transformation history.
- REALIZATION — mapping semantic intent to an actual implementation.
- TARGET — a concrete implementation environment.
- DIALECT — explicitly scoped language extension.
- PHYSICAL INTENT — target-aware implementation information that is intentionally more specific than portable hardware intent.

---

4. Authority and Ownership

The HDL architecture follows the repository-wide authority model.

The intended dependency direction is:

grammar/specification/
        │
        ▼
grammar/spec/
        │
        ▼
grammar/lexer/
        │
        ▼
grammar/antlr/
        │
        ▼
grammar/hdl/
        │
        ▼
domain-neutral AST
        │
        ▼
semantic analysis
        │
        ▼
canonical IR
        │
        ▼
hardware lowering
        │
        ▼
target realization

The responsibilities are:

"grammar/specification/"

Owns language-wide normative architecture.

"grammar/spec/hdl.md"

Owns HDL semantic requirements and the HDL integration contract.

"grammar/lexer/"

Owns the lexical vocabulary and lexical metadata.

"grammar/antlr/ZamaniLexer.g4"

Owns actual ANTLR lexical recognition.

"grammar/hdl/*.g4"

Owns HDL source syntax.

"grammar/Zamani.g4"

Owns top-level language composition.

It MUST NOT redefine HDL semantics.

AST implementation

Owns the representation of parsed HDL syntax in the domain-neutral AST.

Semantic analysis

Owns:

- type correctness;
- width correctness;
- connectivity legality;
- clock-domain correctness;
- reset correctness;
- behavioral legality;
- resource requirements;
- capability requirements;
- effect checking;
- contract checking;
- policy checking;
- provenance.

Hardware realization

Owns:

- synthesis;
- technology mapping;
- placement;
- routing;
- physical resource allocation;
- timing closure;
- vendor mapping;
- device selection;
- deployment.

No downstream implementation layer may silently redefine the source-language meaning.

---

5. Repository Integration Contract

This specification integrates with the existing HDL subsystem.

The canonical HDL composition root is:

grammar/hdl/hdl.g4

Existing specialized HDL grammars remain independently owned.

The HDL subsystem includes, where present:

grammar/hdl/
├── hdl.g4
├── clocks.g4
├── clocking.g4
├── combinational.g4
├── sequential.g4
├── processes.g4
├── registers.g4
├── memories.g4
├── signals.g4
├── nets.g4
├── wires.g4
├── ports.g4
├── interfaces.g4
├── hardware-interfaces.g4
├── hardware-modules.g4
├── hardware-generics.g4
├── hardware-parameters.g4
├── pipelines.g4
├── state_machines.g4
├── parallelism.g4
├── timing.g4
├── reset.g4
├── protocols.g4
├── assertions.g4
├── verification.g4
├── simulation.g4
├── synthesis.g4
├── generate.g4
├── arrays.g4
├── physical-intent.g4
├── co-design.g4
├── hardware-software-integration.g4
└── hardware-dialects.g4

The exact directory may grow.

New HDL capabilities MUST be added as independently owned grammar components when doing so improves maintainability.

The composition root MUST remain thin.

---

6. Composition Root Rule

"grammar/hdl/hdl.g4" is a composition root.

It MUST NOT become a second monolithic HDL grammar.

It SHOULD primarily:

1. import the specialized HDL grammars;
2. expose the canonical HDL composition rules;
3. expose an embedded HDL entry point;
4. optionally expose a standalone HDL entry point.

The critical distinction is:

hdlDesign

versus:

hdlDesignBody

A standalone entry point MAY consume EOF.

An embedded entry point MUST NOT consume the universal parser's EOF.

Conceptually:

hdlDesign
    : hdlDesignBody EOF
    ;

hdlDesignBody
    : hdlItem+
    ;

The universal Zamani parser MUST consume the final EOF.

This prevents HDL from becoming incompatible with the universal grammar.

---

7. Embedded and Standalone HDL

HDL MUST support two parsing contexts.

7.1 Embedded HDL

Embedded HDL occurs inside a normal Zamani source unit.

Conceptually:

Zamani program
    ├── classical item
    ├── quantum item
    ├── HDL item
    ├── AI item
    └── hybrid item

Embedded HDL MUST NOT own:

EOF

The universal parser owns EOF.

7.2 Standalone HDL

Standalone HDL MAY be used for:

- HDL-only tools;
- HDL validation;
- HDL conformance tests;
- HDL import/export;
- HDL editor tooling;
- HDL-specific analysis.

Standalone parsing MAY expose a complete document rule terminating in EOF.

The semantic model MUST be identical in both modes.

---

8. HDL Is Not a Separate Language

HDL is a Zamani domain.

HDL MUST reuse the universal Zamani language infrastructure for:

- identifiers;
- qualified names;
- expressions;
- values;
- types;
- generics;
- attributes;
- annotations;
- modules;
- declarations;
- imports;
- exports;
- functions;
- effects;
- capabilities;
- resources;
- contracts;
- policies;
- provenance;
- diagnostics;
- source locations;
- compatibility.

HDL-specific syntax MUST only exist where hardware semantics require it.

---

9. HDL Feature Ownership

The HDL subsystem owns syntax and semantics for:

- hardware modules;
- hardware interfaces;
- ports;
- signals;
- nets;
- wires;
- registers;
- memories;
- clocks;
- clock domains;
- resets;
- combinational logic;
- sequential logic;
- processes;
- state machines;
- pipelines;
- structural instances;
- generation;
- parameterization;
- hardware generics;
- timing intent;
- protocols;
- verification intent;
- simulation intent;
- synthesis intent;
- physical intent;
- hardware/software co-design;
- accelerator intent;
- hardware parallelism.

HDL MUST NOT own:

- compiler optimization algorithms;
- scheduler implementation;
- target discovery implementation;
- vendor databases;
- physical placement algorithms;
- routing algorithms;
- synthesis algorithms;
- fabrication;
- deployment infrastructure;
- runtime hardware discovery;
- HAL implementation;
- quantum error-correction implementation;
- ZQN implementation.

Those are downstream systems.

---

10. Universal Scalability Rule

Zamani HDL MUST NOT impose artificial finite limits on the language.

The following MUST NOT exist as universal language limits:

MAX_MODULES
MAX_PORTS
MAX_SIGNALS
MAX_NETS
MAX_REGISTERS
MAX_MEMORIES
MAX_STATES
MAX_TRANSITIONS
MAX_PIPELINE_STAGES
MAX_INSTANCES
MAX_GENERATED_INSTANCES
MAX_WIDTH
MAX_MEMORY_DEPTH
MAX_MEMORY_WIDTH
MAX_CLOCKS
MAX_CLOCK_DOMAINS
MAX_DEVICES
MAX_CHANNELS
MAX_PARALLEL_UNITS
MAX_LUTS
MAX_BRAMS
MAX_DSPS
MAX_FPGAS
MAX_ASICS
MAX_ACCELERATORS

The same rule applies to any future hardware quantity.

A compiler implementation MAY have implementation limits.

Such a limit MUST be reported as an implementation/resource limitation.

It MUST NOT become a language semantic restriction.

---

11. Meaning of "Infinity"

For this specification, "infinity" means:

«The language imposes no artificial finite bound on a semantically parameterizable quantity.»

It does NOT mean physical hardware is infinite.

For example:

pipeline_stages = N

may be valid for arbitrary semantically valid "N".

A particular target may lack the resources.

The compiler MUST report:

required resource unavailable

rather than silently rewriting:

N

into another value.

---

12. Program Meaning Versus Target Feasibility

A target may be:

feasible

or:

infeasible

for a program.

The compiler MUST distinguish:

semantic invalidity

from:

target infeasibility

For example:

memory depth must be positive

is a semantic error.

Whereas:

target cannot provide requested memory capacity

is a realization/resource error.

The latter MUST NOT cause the source semantics to be silently altered.

---

13. Parameterization

HDL MUST be parameterizable.

Parameters MAY describe:

- widths;
- depths;
- dimensions;
- element types;
- channel counts;
- pipeline stages;
- latency;
- throughput;
- protocol properties;
- memory properties;
- timing properties;
- resource requirements;
- capability requirements;
- structural multiplicity.

Parameters MAY be:

- constants;
- expressions;
- generic values;
- generic types;
- symbolic values;
- compile-time values;
- configuration values;
- derived values.

No universal maximum parameter value may be imposed.

---

14. Hardware Generics

"grammar/hdl/hardware-generics.g4" MUST reuse the universal Zamani generic/type/expression model.

It MUST NOT introduce a separate hardware type system.

Generic elaboration follows:

generic declaration
        ↓
generic binding
        ↓
constraint checking
        ↓
specialization
        ↓
semantic elaboration
        ↓
hardware realization

Generic constraints MUST be semantic constraints.

They MUST NOT be disguised physical machine limits.

---

15. Generic Specialization

A specialization MUST preserve source semantics.

For example:

GenericModule<WIDTH = W>

means:

instantiate the generic semantic definition with WIDTH = W

It does not mean:

select a particular physical implementation

unless the source explicitly requests a target-specific realization.

---

16. Hardware Modules

A hardware module is a composable semantic unit.

A module MAY contain:

- parameters;
- generics;
- ports;
- interfaces;
- local types;
- signals;
- nets;
- wires;
- registers;
- memories;
- processes;
- combinational behavior;
- sequential behavior;
- state machines;
- pipelines;
- instances;
- generated structures;
- assertions;
- contracts;
- timing intent;
- resource requirements;
- capability requirements;
- policies;
- provenance metadata.

There is no fixed number of members.

---

17. Module Identity

A module identifier identifies a semantic module.

It MUST NOT implicitly identify:

- a physical chip;
- a physical FPGA;
- an ASIC;
- a board;
- a package;
- a physical location;
- a particular routing resource.

Physical identity belongs to realization or explicitly target-specific dialects.

---

18. Module Instances

An instance represents a semantic occurrence of a module.

Instance identity MUST remain distinct from physical resource identity.

Therefore:

instance accelerator

does not mean:

accelerator #3

on a particular physical device.

Placement is downstream.

---

19. Structural Composition

Modules MUST be composable.

The language MUST permit arbitrary semantic structural composition subject to:

- syntax validity;
- type validity;
- connectivity validity;
- contract validity;
- resource feasibility;
- capability feasibility;
- policy validity.

Instance counts MUST NOT have language-level finite limits.

---

20. Ports

Ports define externally visible hardware interfaces.

A port MAY specify:

- direction;
- type;
- dimensions;
- width;
- protocol;
- timing intent;
- attributes;
- capability requirements;
- resource requirements.

The canonical port syntax is owned by:

grammar/hdl/ports.g4

Interface-level composition is owned by:

grammar/hdl/interfaces.g4
grammar/hdl/hardware-interfaces.g4

No competing port grammar may be introduced elsewhere.

---

21. Port Types

Port types MUST use the canonical Zamani type system.

HDL MUST NOT create a separate type universe merely to represent hardware.

A hardware-specific type MAY carry hardware semantic metadata such as:

- bit representation;
- signaling semantics;
- timing semantics;
- protocol semantics;
- physical intent.

The underlying type model remains part of Zamani's universal type system.

---

22. Signals

Signals represent semantic communication or value relationships.

A signal MAY have:

- type;
- initial value;
- driver semantics;
- timing metadata;
- attributes;
- contracts;
- provenance.

A signal does not inherently identify a physical wire.

---

23. Nets

Nets represent connectivity semantics where required.

The semantic model MUST distinguish:

logical connectivity

from:

physical routing

A net declaration MUST NOT implicitly identify:

- a routing track;
- a switch matrix element;
- a metal layer;
- a package connection;
- a vendor routing primitive.

---

24. Multiple Drivers

Where multiple drivers are permitted, semantic analysis MUST determine:

- whether multiple drivers are legal;
- whether resolution is required;
- whether a resolution function exists;
- whether contention is possible;
- whether high impedance is meaningful;
- whether conflicting drivers are an error.

The grammar recognizes syntax.

The semantic layer determines legality.

---

25. Registers

Registers represent persistent hardware state.

Register semantics MUST define:

- state type;
- update behavior;
- clock association;
- reset behavior;
- enable behavior;
- initialization where supported;
- edge semantics.

Register width MUST be parameterizable.

No universal register width may be imposed.

---

26. Memories

"grammar/hdl/memories.g4" owns memory syntax.

Memory semantics MAY express:

- element type;
- width;
- depth;
- dimensionality;
- read behavior;
- write behavior;
- latency;
- throughput;
- initialization;
- persistence;
- access protocol;
- resource requirements.

Memory dimensions MUST remain semantic values.

A source memory MUST NOT implicitly mean:

BRAM
SRAM
DRAM
HBM
register file
cache
vendor memory primitive

The implementation chooses a realization capable of satisfying the requirements.

---

27. Memory Scaling

Memory capacity MUST be expressed symbolically or semantically.

Examples include:

requires memory >= required_memory;
requires bandwidth >= required_bandwidth;
requires capability("memory.random_access");

No fixed universal capacity may be assumed.

---

28. Clocks

"grammar/hdl/clocks.g4" owns clock declaration syntax.

"grammar/hdl/clocking.g4" owns clocking relationships and clock-domain semantics at the grammar level.

A clock MAY describe:

- identity;
- frequency;
- period;
- duty cycle;
- phase;
- source relationship;
- domain;
- edge semantics;
- constraints;
- attributes.

Clock declarations MUST remain target-neutral unless explicitly target-specific.

---

29. Clock Frequency

Frequency is a semantic quantity.

For example:

frequency = desired_frequency

defines intent.

If a target cannot satisfy that requirement, the implementation MUST report infeasibility.

It MUST NOT silently substitute a different observable frequency while claiming the same semantic result.

---

30. Clock Domains

A clock domain is a semantic timing region.

A design MAY contain arbitrarily many clock domains.

No grammar-level maximum exists.

Semantic analysis MUST reason about relationships between clock domains.

---

31. Clock-Domain Crossing

Cross-domain communication MUST be explicitly analyzable.

The semantic model MUST support concepts such as:

- synchronization;
- handshake;
- asynchronous FIFO;
- pulse synchronization;
- domain bridge;
- sampled transfer;
- CDC assumptions.

The exact implementation may vary by target.

The source semantics describe required correctness, not a vendor-specific primitive.

---

32. Resets

Reset syntax is owned by:

grammar/hdl/reset.g4

Reset semantics MUST distinguish, where applicable:

- synchronous;
- asynchronous;
- active-high;
- active-low;
- reset sequencing;
- reset dependencies;
- reset domains;
- reset release behavior.

Reset declarations MUST NOT imply a particular vendor primitive.

---

33. Combinational Logic

"grammar/hdl/combinational.g4" owns combinational syntax.

Combinational semantics MUST ensure that state is not introduced implicitly.

Semantic analysis MUST detect applicable conditions including:

- incomplete assignments;
- unintended latches;
- illegal feedback;
- multiple incompatible drivers;
- invalid dependencies;
- combinational cycles where prohibited.

---

34. Sequential Logic

"grammar/hdl/sequential.g4" owns sequential intent.

Sequential semantics MUST define:

state
+
event
+
transition

The semantic layer determines:

- state dependencies;
- update ordering;
- clock relationship;
- reset behavior;
- enable semantics;
- timing assumptions.

---

35. Processes

"grammar/hdl/processes.g4" owns process declarations.

A process MUST have an explicitly analyzable execution model.

A process MAY represent:

- combinational behavior;
- sequential behavior;
- clocked behavior;
- event-driven behavior;
- protocol behavior.

Processes MUST NOT create a hidden runtime semantics that conflicts with Zamani concurrency.

---

36. Hardware Concurrency

Hardware parallelism is semantic.

"grammar/hdl/parallelism.g4" MUST integrate with the repository's universal concurrency and execution architecture.

Hardware parallelism MUST NOT create an independent concurrency universe.

The semantic relationship is:

hardware parallel intent
        ↓
semantic dependency graph
        ↓
resource requirements
        ↓
scheduling
        ↓
target realization

---

37. Pipelines

"grammar/hdl/pipelines.g4" owns pipeline syntax.

A pipeline MAY specify:

- stages;
- dependencies;
- throughput;
- latency;
- buffering;
- stage constraints;
- timing intent;
- resource preferences.

Pipeline depth MUST NOT have a language-level maximum.

The compiler MAY reject a realization if the target cannot satisfy the requested pipeline.

---

38. State Machines

"grammar/hdl/state_machines.g4" owns state-machine syntax.

A state machine MUST have semantic representations for:

- states;
- transitions;
- guards;
- actions;
- initial state;
- terminal behavior where applicable.

The number of states and transitions MUST NOT be artificially bounded.

State-machine correctness MUST be independently analyzable.

---

39. Generate and Structural Replication

"grammar/hdl/generate.g4" owns structural generation syntax.

Generation MUST support parameterized replication without imposing a universal finite count.

Generation MUST be evaluated through the semantic/elaboration layer.

A generator MUST NOT bypass:

- type checking;
- resource analysis;
- capability analysis;
- contracts;
- policies;
- provenance.

Generated structures MUST remain traceable to their source generator.

---

40. Hardware Arrays

"grammar/hdl/arrays.g4" owns HDL-specific array syntax where required.

Array dimensions MUST be semantic values.

No universal maximum rank or dimension size may be imposed.

The semantic layer MUST distinguish:

logical array

from:

physical storage realization

---

41. Protocols

"grammar/hdl/protocols.g4" owns target-independent protocol declarations.

Protocols MAY describe:

- message structure;
- handshake;
- ordering;
- valid/ready semantics;
- request/response;
- streaming;
- flow control;
- consistency;
- timing assumptions.

Protocol syntax MUST remain independent of a particular bus implementation unless explicitly scoped to a dialect.

---

42. Interfaces

Interfaces MUST describe logical contracts between hardware components.

An interface MAY contain:

- ports;
- signals;
- protocols;
- timing;
- capabilities;
- requirements;
- contracts;
- metadata.

Interfaces MUST remain composable.

---

43. Timing Intent

"grammar/hdl/timing.g4" owns source-level timing intent.

Timing semantics MAY include:

- latency;
- throughput;
- period;
- frequency;
- setup intent;
- hold intent;
- timing relationships;
- pipeline constraints;
- clock relationships;
- synchronization requirements.

Timing intent is not the same as timing closure.

Timing closure is a downstream realization problem.

---

44. Timing Correctness

A timing requirement is part of semantic validity when the program explicitly makes it observable or contractual.

The implementation MUST distinguish:

required timing

from:

preferred timing

For example:

requires latency <= L

is a requirement.

Whereas:

prefer latency <= L

is a preference.

The latter MAY be violated if semantic correctness remains intact.

---

45. Verification

"grammar/hdl/verification.g4" owns verification intent.

Verification MAY express:

- assertions;
- properties;
- assumptions;
- guarantees;
- invariants;
- coverage intent;
- formal properties;
- simulation checks;
- equivalence requirements.

Verification constructs MUST integrate with:

grammar/validation/

rather than creating an isolated contract system.

---

46. Assertions

"grammar/hdl/assertions.g4" owns HDL-specific assertion syntax.

Assertions MUST integrate with the universal contract model.

Conceptually:

assertion
    ├── condition
    ├── scope
    ├── assumptions
    ├── evidence
    ├── severity
    └── provenance

---

47. Contracts

HDL MUST consume the universal contract system.

Relevant constructs include:

requires
ensures
invariant
assume
guarantee
property

A hardware contract MAY constrain:

- signal relationships;
- timing;
- state;
- interfaces;
- resources;
- capabilities;
- protocols;
- safety properties.

Contracts MUST NOT be silently discarded during lowering.

---

48. Resource Requirements

HDL MUST integrate with:

grammar/resources/

A hardware description MAY express:

requires memory >= required_memory;
requires capability("parallel.compute");
requires capability("hardware.pipeline");
requires topology(required_topology);

Resource requirements MUST be symbolic and scalable.

They MUST NOT hard-code the capacity of a particular machine.

---

49. Resource Requirements Versus Preferences

The semantic model MUST distinguish:

requires

from:

prefer

and:

constrain
allow
forbid

A requirement affects feasibility.

A preference affects optimization.

A prohibition restricts realization.

An allowance expands acceptable realization.

These categories MUST remain semantically distinct.

---

50. Capabilities

HDL capabilities describe what an implementation must be able to provide.

Examples:

capability("parallel.compute")
capability("memory.random_access")
capability("streaming")
capability("reconfigurable")
capability("hardware.pipeline")
capability("high_speed.interconnect")

Capability names MUST remain open-ended.

The grammar MUST NOT enumerate every future hardware capability.

---

51. Effects

HDL MUST participate in the universal effect system.

Relevant effects may include:

io
network
native
foreign
distributed
measurement
quantum
simulation
mutation
code_generation

Hardware constructs that interact with software, external devices, I/O, networking, quantum measurement, or simulation MUST carry appropriate effect information.

HDL MUST NOT create an independent effect system.

---

52. Policies

HDL MUST integrate with:

grammar/policies/
grammar/security/
grammar/execution/policies.g4

Policies MAY constrain:

- target selection;
- resource usage;
- physical realization;
- deployment;
- security;
- simulation;
- synthesis;
- reconfiguration;
- adaptation;
- external interfaces.

A policy MUST NOT silently change the meaning of a hardware operation.

---

53. Provenance

Every hardware construct that undergoes elaboration, generation, specialization, optimization, or lowering SHOULD remain traceable.

Provenance MUST be compatible with the universal provenance system.

At minimum, hardware transformations SHOULD be capable of recording:

source
derived_from
generated_by
specialized_from
transformed_by
optimized_by
lowered_by
verified_by
realized_by

Generated hardware MUST remain traceable to the source construct that generated it.

---

54. Determinism

HDL compilation and semantic analysis SHOULD be deterministic.

Given:

same source
+
same language version
+
same dialect versions
+
same compiler configuration
+
same target capability description
+
same relevant policies

the semantic result MUST be reproducible.

Where nondeterministic optimization is permitted, the compiler MUST preserve semantic equivalence.

---

55. Simulation

"grammar/hdl/simulation.g4" owns HDL simulation intent.

Simulation is an execution strategy.

It is not a second language.

Simulation MAY model:

- combinational behavior;
- sequential behavior;
- timing;
- clocks;
- resets;
- protocols;
- faults;
- resource behavior;
- quantum-control interfaces;
- distributed hardware;
- heterogeneous systems.

Simulation MUST consume the same semantic model as actual realization.

---

56. Simulation Versus Implementation

Simulation MUST NOT establish independent source semantics.

The relationship is:

Zamani source
     ↓
semantic model
     ├───────────────┐
     ↓               ↓
simulation        realization

The same semantic intent is used by both.

---

57. Synthesis

"grammar/hdl/synthesis.g4" owns source-level synthesis intent.

Synthesis semantics MAY specify:

- whether synthesis is required;
- allowed implementation classes;
- optimization objectives;
- resource preferences;
- timing preferences;
- area preferences;
- power preferences;
- reliability requirements.

The synthesis algorithm itself is outside the grammar.

---

58. Optimization

HDL optimization MUST preserve semantic meaning.

The compiler MAY:

- pipeline;
- replicate;
- share;
- reorder independent operations;
- retime where legal;
- eliminate dead hardware;
- fuse compatible operations;
- select equivalent implementations.

Such transformations MUST respect:

- contracts;
- effects;
- resource requirements;
- capabilities;
- policies;
- observable timing semantics;
- provenance.

---

59. Physical Intent

"grammar/hdl/physical-intent.g4" owns explicit physical-intent syntax.

Physical intent is deliberately more specific than portable HDL.

Examples of information that MAY be represented there include:

- placement preference;
- region preference;
- pin constraint;
- physical clock resource;
- implementation technology;
- target family;
- routing preference.

Physical intent MUST be clearly marked as target-dependent.

It MUST NOT silently become a universal HDL requirement.

---

60. Portable Versus Target-Specific HDL

There are two categories:

Portable HDL

Describes:

- behavior;
- structure;
- interfaces;
- resources;
- capabilities;
- contracts;
- timing requirements;
- implementation preferences.

Target-specific HDL

May additionally describe:

- vendor primitives;
- physical resources;
- device families;
- pin assignments;
- implementation regions;
- technology-specific structures.

Target-specific HDL MUST be represented through explicit dialect or target mechanisms.

It MUST NOT contaminate the universal grammar.

---

61. Hardware Dialects

"grammar/hdl/hardware-dialects.g4" provides the open-world extension mechanism for HDL/hardware dialects.

A dialect MUST declare:

- identity;
- version;
- owner;
- compatibility;
- syntax extensions;
- semantic extensions;
- capability requirements;
- target scope;
- provenance requirements.

A dialect MUST NOT redefine the meaning of universal Zamani constructs.

---

62. Vendor Extensions

Vendor-specific constructs MAY exist.

They MUST be isolated.

A vendor extension MUST NOT require modifications to the universal HDL grammar merely to introduce a new vendor feature.

The preferred architecture is:

universal HDL
      +
dialect metadata
      +
target capability

rather than:

universal HDL
      +
per-vendor hard-coded grammar branches

---

63. Hardware/Software Co-Design

"grammar/hdl/co-design.g4" and:

grammar/hdl/hardware-software-integration.g4

define the source-level boundary between software and hardware.

The semantic model MUST support:

software
    ↓
hardware interface
    ↓
accelerator
    ↓
hardware implementation

and:

hardware
    ↓
software-visible interface
    ↓
runtime

The interface MUST carry:

- types;
- ownership;
- effects;
- capabilities;
- resource requirements;
- contracts;
- provenance.

---

64. Accelerators

Accelerators MUST be represented as semantic capabilities rather than hard-coded device classes.

A program MAY request:

requires capability("tensor.compute");

rather than:

requires accelerator = "device-X";

The compiler may realize the request through:

- FPGA;
- ASIC;
- CPU;
- GPU;
- dedicated accelerator;
- reconfigurable hardware;
- future hardware.

---

65. Heterogeneous Computing

HDL MUST compose with classical, quantum, AI, data, distributed, and networking domains.

A valid program MAY conceptually contain:

classical computation
        ↓
hardware accelerator
        ↓
quantum control
        ↓
measurement
        ↓
classical decision
        ↓
distributed execution

No domain may create an isolated semantic universe.

---

66. Quantum/HDL Boundary

HDL MAY describe quantum-control hardware and interfaces.

It MUST NOT define a second quantum semantic model.

Quantum computation MUST converge on:

quantum::ir

The boundary is:

HDL control intent
       ↓
hybrid semantic model
       ↓
quantum operation
       ↓
quantum::ir

HDL MAY describe:

- control signals;
- timing;
- synchronization;
- measurement interfaces;
- cryogenic/control interfaces;
- classical feedback;
- hardware scheduling.

The quantum semantic meaning remains owned by the quantum subsystem.

---

67. Quantum-Classical Feedback

The language MUST support hardware patterns where:

quantum operation
        ↓
measurement
        ↓
classical result
        ↓
classical decision
        ↓
hardware control

The effect and provenance systems MUST preserve this boundary.

Measurement MUST be represented as an explicit semantic effect where applicable.

---

68. Reconfigurable Hardware

HDL MAY express reconfiguration intent.

Reconfiguration MUST participate in:

- effects;
- capabilities;
- resources;
- policies;
- authorization;
- provenance.

Reconfiguration MUST NOT mean unrestricted self-modification.

The semantic architecture is:

requested adaptation
        ↓
policy
        ↓
authorization
        ↓
capability check
        ↓
resource check
        ↓
validated new realization
        ↓
provenance

---

69. Adaptive Hardware

Adaptive hardware MAY respond to:

- workload;
- environment;
- resource state;
- fault state;
- performance;
- policy;
- measured conditions.

Adaptation MUST preserve declared contracts.

An implementation MUST NOT change hardware behavior outside the declared adaptation policy.

---

70. Reliability and Fault Handling

HDL MAY participate in the universal resilience model.

Relevant states include:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

Relevant outcomes include:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

HDL MUST consume these semantics rather than defining incompatible hardware-specific resilience states.

---

71. Safety-Critical Hardware

Safety-critical designs SHOULD use:

- contracts;
- invariants;
- assertions;
- provenance;
- deterministic compilation;
- explicit policies;
- capability restrictions;
- verification properties.

A target implementation MUST NOT silently weaken safety requirements.

---

72. Resource Negotiation

Hardware realization follows:

source requirement
        ↓
target capability discovery
        ↓
resource negotiation
        ↓
constraint solving
        ↓
realization planning

The grammar expresses the requirement.

The target system provides the capability information.

The compiler decides feasibility.

The HAL realizes the decision.

---

73. No Target Discovery in Grammar

The HDL grammar MUST NOT contain logic that discovers:

- available FPGA devices;
- CPU count;
- GPU count;
- memory size;
- accelerator count;
- physical routing resources;
- physical pins;
- fabrication technology.

Those belong to target discovery and realization infrastructure.

---

74. No Physical Resource Constants

The universal grammar MUST NOT encode constants representing physical implementation capacity.

Examples prohibited as universal assumptions:

32-bit hardware
64-bit hardware
8 pipeline stages
1024 memory entries
24 accelerator units
32 clock domains
64 devices

Such values MAY occur as explicit program parameters or target-specific dialect declarations.

They MUST NOT become universal language assumptions.

---

75. Width Semantics

Hardware widths MUST be semantic.

A width MAY be:

- literal;
- symbolic;
- generic;
- derived;
- type-level;
- configuration-dependent.

The semantic layer MUST determine whether a width is valid.

The target layer determines whether that width is realizable.

---

76. Arbitrary Precision and Large Values

Where the Zamani type system permits arbitrary or parameterized integer values, HDL MUST NOT introduce smaller implementation assumptions.

A compiler implementation MAY use efficient internal representations.

It MUST NOT make a representation width a language-level semantic limit.

---

77. Resource-Driven Scaling

The intended scaling model is:

same semantic program
        ↓
different target capability set
        ↓
different valid realization

For example:

tiny embedded realization

and:

large distributed accelerator realization

may derive from the same semantic source if both satisfy the program's requirements.

---

78. Scaling Failure

When a target cannot satisfy a required property, the compiler MUST produce a structured diagnostic.

The diagnostic SHOULD identify:

requirement
available capability/resource
missing capability/resource
source location
affected construct
possible realization alternatives

The compiler MUST NOT silently reduce the requirement.

---

79. Resource Preferences

The language MAY express preferences such as:

prefer accelerator;
prefer lower_power;
prefer lower_latency;
prefer higher_throughput;
prefer reconfigurable;

Preferences MUST NOT change correctness.

If no realization satisfies a preference but one satisfies all requirements, compilation MAY proceed.

---

80. Constraints

Constraints may restrict realization.

Examples:

constrain topology(...);
constrain latency <= bound;
constrain power <= budget;

Constraints MUST be evaluated by the semantic/resource realization system.

They MUST NOT be interpreted as hidden grammar limits.

---

81. Forbid and Allow

Policies MAY express:

allow ...
forbid ...

Examples include:

forbid capability("vendor.specific.feature");
forbid effect("native");
allow capability("reconfigurable");

These integrate with the universal policy model.

---

82. Effects and Hardware I/O

Hardware interfaces that communicate with the external world SHOULD carry explicit effects.

Examples:

effect(io)
effect(network)
effect(native)
effect(foreign)

This allows hardware/software co-design to participate in the same effect analysis as the rest of Zamani.

---

83. Foreign Interfaces

Hardware MAY interact with foreign APIs and ABI boundaries.

The boundary MUST integrate with:

grammar/interoperability/

and the universal effect/capability model.

Foreign hardware interfaces MUST NOT bypass:

- type checking;
- capability checking;
- policy checking;
- provenance.

---

84. ABI

Hardware/software ABI information belongs to the interoperability layer.

HDL may describe ABI-relevant interface intent, but ABI realization belongs downstream.

ABI metadata MAY include:

- calling convention;
- data layout;
- alignment;
- ownership;
- register mapping;
- memory mapping;
- synchronization;
- interface protocol.

No ABI property may silently override source semantics.

---

85. Deterministic Reproducibility

Production HDL compilation SHOULD record:

language version
grammar version
AST version
semantic version
IR version
dialect versions
target capability description
policy versions
compiler version
relevant configuration

This information belongs in provenance/compatibility infrastructure.

---

86. Version Compatibility

HDL MUST participate in:

grammar/compatibility/

Compatibility MUST distinguish:

- source compatibility;
- grammar compatibility;
- AST compatibility;
- semantic compatibility;
- IR compatibility;
- dialect compatibility;
- target compatibility.

A target-specific limitation MUST NOT be represented as a source-language incompatibility.

---

87. Deprecation

HDL constructs MAY be deprecated.

Deprecation MUST provide:

- construct identity;
- version;
- replacement;
- migration information;
- compatibility behavior;
- diagnostic severity.

Deprecated constructs MUST NOT silently acquire new semantics.

---

88. Metaprogramming

HDL MAY participate in Zamani metaprogramming.

Generated HDL MUST pass through normal:

parse
→ AST
→ semantic validation
→ type checking
→ effect checking
→ capability checking
→ resource checking
→ contract checking
→ policy checking
→ provenance

Generated code MUST NOT bypass validation.

---

89. Reflection

Reflection MAY inspect HDL semantic metadata where explicitly authorized.

Reflection MUST respect:

- capability requirements;
- security policies;
- provenance;
- effects;
- target boundaries.

Reflection MUST NOT automatically grant physical access.

---

90. Evidence and Explainability

Hardware compilation SHOULD support explanations for:

- resource decisions;
- placement decisions;
- optimization;
- synthesis choices;
- scheduling;
- target selection;
- rejection;
- fallback;
- simulation decisions.

The explanation model SHOULD integrate with the universal evidence/provenance system.

---

91. Semantic Model

Every HDL construct MUST ultimately map to a domain-neutral semantic representation.

The semantic representation MUST be capable of representing:

identity
type
operands
parameters
results
attributes
modifiers
effects
capabilities
resources
contracts
policies
provenance
source location

Where applicable, it MUST additionally represent:

timing
clock domain
reset domain
connectivity
state
protocol
hardware intent
realization constraints

---

92. AST Contract

The AST MUST represent source structure without embedding target-specific implementation decisions.

HDL AST nodes SHOULD preserve:

- source locations;
- declaration identity;
- generic parameters;
- type expressions;
- expressions;
- structural relationships;
- attributes;
- modifiers;
- contracts;
- resource declarations;
- capability declarations;
- policy references.

The AST MUST NOT store resolved FPGA LUT numbers, physical routing tracks, or other realization artifacts as universal semantic meaning.

---

93. Semantic Ownership

The semantic layer owns:

type correctness
connectivity correctness
width correctness
clock correctness
reset correctness
protocol correctness
contract correctness
effect correctness
capability requirements
resource requirements
policy compliance

The grammar only establishes syntactic structure.

---

94. Canonical IR Boundary

HDL MUST NOT introduce an incompatible universal IR.

The canonical architecture remains:

domain-neutral semantic model
        │
        ├── Classical IR
        │
        └── quantum::ir

Hardware-specific lowering MAY produce intermediate implementation representations downstream.

Such representations are compiler/backend implementation artifacts, not additional source-language authorities.

---

95. Hardware Lowering

Hardware lowering MAY transform:

hardware semantic intent

into:

implementation-oriented representation

including:

- technology-independent hardware IR;
- FPGA-oriented IR;
- ASIC-oriented IR;
- accelerator IR;
- scheduling representation;
- placement representation;
- routing representation.

These MUST remain downstream from the canonical semantic model.

---

96. Lowering Correctness

Every lowering transformation MUST preserve semantic meaning.

Where exact preservation is impossible, the compiler MUST report the incompatibility.

The compiler MUST NOT silently substitute an approximation when the source contract requires exact semantics.

---

97. Optimization Correctness

Optimization MUST preserve:

- observable behavior;
- declared contracts;
- required timing properties;
- required resource semantics;
- required effects;
- required capabilities;
- policy restrictions.

Optimizations MAY improve:

- latency;
- throughput;
- area;
- energy;
- resource usage;
- reliability.

---

98. Scheduling

Scheduling belongs downstream.

HDL source MAY express scheduling constraints or preferences.

The scheduler determines an implementation schedule consistent with:

- dependencies;
- resources;
- timing;
- contracts;
- capabilities;
- policies.

No universal scheduler algorithm is mandated by the HDL grammar.

---

99. Placement

Placement is a realization concern.

Portable HDL MUST NOT require a particular physical placement.

Physical-intent dialects MAY constrain placement explicitly.

Such constraints MUST be identifiable as target-dependent.

---

100. Routing

Routing is a realization concern.

Portable HDL MUST describe connectivity.

The routing system determines the physical realization.

Source-level net identity MUST NOT be confused with physical routing-resource identity.

---

101. Technology Mapping

Technology mapping MAY select:

- LUT structures;
- gates;
- standard cells;
- DSP blocks;
- memory primitives;
- vendor accelerators;
- custom ASIC structures;
- future implementation resources.

The selected mapping MUST satisfy the semantic model.

---

102. FPGA Realization

FPGA realization MAY map semantic HDL into:

- programmable logic;
- memories;
- arithmetic resources;
- clock resources;
- I/O;
- routing.

The universal HDL grammar MUST NOT assume any particular FPGA architecture.

---

103. ASIC Realization

ASIC realization MAY map semantic HDL into:

- standard cells;
- memories;
- clock trees;
- custom macros;
- physical regions;
- technology-specific resources.

The universal HDL grammar MUST remain technology-independent.

---

104. Accelerator Realization

Accelerator realization MAY map semantic hardware into:

- matrix engines;
- tensor engines;
- DSP arrays;
- vector engines;
- domain-specific accelerators;
- reconfigurable structures.

The language MUST describe requirements and capabilities rather than hard-code a fixed accelerator taxonomy.

---

105. CPU/GPU Integration

Hardware descriptions MAY participate in CPU/GPU heterogeneous systems.

The source may describe:

host
device
accelerator
memory
communication
synchronization

The implementation determines actual target mapping.

---

106. Distributed Hardware

HDL MAY be deployed across multiple devices or nodes.

The source MUST NOT impose a universal maximum node/device count.

Distributed placement belongs to:

grammar/distributed/
grammar/networking/
grammar/resources/

and downstream realization.

---

107. Networking

Hardware networking MUST integrate with:

grammar/networking/

Network hardware MAY express:

- endpoints;
- channels;
- streams;
- protocols;
- topology;
- bandwidth;
- latency;
- ordering;
- reliability.

Network realization remains target-dependent.

---

108. Data Movement

Data movement is a semantic concern when it affects correctness or declared performance requirements.

The semantic model SHOULD represent:

- producer;
- consumer;
- transfer;
- ordering;
- ownership;
- synchronization;
- bandwidth requirements.

Physical interconnect selection remains downstream.

---

109. Power and Energy

HDL MAY express power or energy requirements and preferences.

For example:

requires power <= power_budget;
prefer lower_energy;

Power constraints MUST remain symbolic.

No fixed universal power value is implied.

---

110. Thermal Constraints

Where supported, HDL MAY express thermal constraints.

Thermal realization belongs to hardware/resource analysis.

A thermal constraint MUST NOT cause silent semantic changes.

---

111. Reliability

HDL MAY express reliability requirements such as:

- redundancy;
- fault tolerance;
- recovery;
- replication;
- error detection;
- error correction.

The implementation decides the realization subject to declared requirements.

---

112. Formal Verification

HDL MUST be compatible with formal verification.

Formal properties MAY be attached to:

- modules;
- interfaces;
- signals;
- state;
- transitions;
- protocols;
- timing;
- generated structures.

Formal verification results SHOULD become provenance/evidence artifacts.

---

113. Verification Evidence

Verification SHOULD produce structured evidence:

claim
evidence
method
tool
version
input
result
provenance

This integrates with the universal evidence/provenance architecture.

---

114. Simulation Evidence

Simulation results SHOULD preserve:

- source identity;
- simulation configuration;
- semantic version;
- target model;
- input;
- output;
- assertions;
- failures;
- provenance.

Simulation MUST NOT silently redefine the source semantics.

---

115. Fault Simulation

HDL simulation MAY model:

- transient faults;
- persistent faults;
- timing faults;
- communication faults;
- memory faults;
- hardware degradation.

Fault models MUST be explicit.

---

116. Security

HDL security MUST integrate with:

grammar/security/

Relevant properties include:

- capability restrictions;
- trust boundaries;
- authorization;
- sandboxing;
- audit;
- provenance;
- secure interfaces.

Hardware access MUST NOT automatically imply unrestricted software access.

---

117. Sandbox

Hardware simulation, generated code, foreign execution, reflection, and target-specific execution MAY be sandboxed.

Sandboxing MUST integrate with:

effects
capabilities
resources
policies
security

---

118. Adaptation and Reconfiguration Security

Any runtime or compile-time adaptation affecting hardware MUST be governed by explicit policy.

The minimum conceptual chain is:

adaptation request
      ↓
authorization
      ↓
policy
      ↓
capability check
      ↓
resource check
      ↓
contract check
      ↓
realization
      ↓
provenance

---

119. No Unrestricted Self-Modifying Hardware

A hardware construct MUST NOT implicitly authorize unrestricted modification of its own semantics.

Adaptation may alter an authorized realization while preserving the declared semantic contract.

---

120. Diagnostics

HDL diagnostics MUST distinguish categories.

At minimum:

syntax error
type error
generic error
connectivity error
width error
clock error
reset error
protocol error
contract violation
effect violation
capability failure
resource failure
policy violation
dialect incompatibility
target incompatibility
realization failure
verification failure

Diagnostics SHOULD identify:

- source location;
- construct;
- requirement;
- actual state;
- expected state;
- relevant policy;
- relevant capability;
- relevant resource.

---

121. Error Recovery

Parser error recovery MUST remain parser-specific.

Semantic error recovery MUST NOT invent hardware semantics.

A parser MUST NOT recover from invalid syntax by silently selecting a different hardware operation.

---

122. Source Locations

Every HDL AST construct MUST preserve source location information sufficient for diagnostics and provenance.

At minimum:

file
start position
end position

Additional source mapping MAY include:

- module;
- declaration;
- generated source;
- macro expansion;
- specialization origin.

---

123. Generated Source

Generated HDL MUST preserve provenance to its generator.

The compiler SHOULD support:

generated_by
derived_from
specialized_from

relationships.

Generated source MUST remain diagnosable.

---

124. Generic Elaboration

Generic elaboration MUST be deterministic given identical inputs.

The elaborator MUST:

1. resolve generic declarations;
2. bind arguments;
3. evaluate permitted compile-time expressions;
4. validate constraints;
5. instantiate semantic structure;
6. preserve provenance;
7. emit diagnostics where invalid.

Elaboration MUST NOT perform target placement.

---

125. Compile-Time Evaluation

Compile-time HDL expressions MUST obey the universal metaprogramming and effect model.

A compile-time expression MUST NOT gain unrestricted access to:

- filesystem;
- network;
- native execution;
- physical devices;

unless explicitly authorized by effects, capabilities, and policy.

---

126. Resource Analysis

Resource analysis MUST be symbolic where possible.

It SHOULD determine:

required resources
available resources
resource deltas
resource bottlenecks

without introducing language-level finite capacities.

---

127. Capability Negotiation

Capability negotiation MUST be open-ended.

A target may advertise:

capability("hardware.pipeline")
capability("tensor.compute")
capability("reconfigurable")
capability("high_speed.interconnect")

The source requests capabilities.

The compiler resolves the intersection.

---

128. Target Discovery

Target discovery is outside the grammar.

The target environment MAY provide:

resource inventory
capability inventory
topology
timing capabilities
memory capabilities
reconfiguration capabilities

The compiler consumes that information.

---

129. Resource Substitution

A target MAY substitute equivalent physical resources if semantic requirements remain satisfied.

For example:

semantic arithmetic operation

may be realized using different physical resources.

The substitution MUST preserve semantics.

---

130. Semantic Equivalence

Two realizations are equivalent when they satisfy the same observable semantic contract.

Equivalent implementation may differ in:

- physical resources;
- topology;
- device family;
- scheduling;
- placement;
- routing;
- microarchitecture.

The language therefore describes semantic identity rather than physical identity.

---

131. Observable Properties

The compiler MUST treat explicitly declared observable properties as semantic.

Potentially observable properties include:

- result;
- ordering;
- externally visible timing;
- protocol behavior;
- synchronization;
- side effects;
- resource contracts;
- power constraints where explicitly contractual.

An optimization MUST NOT change an observable property without authorization.

---

132. Performance Properties

Performance MAY be:

required
preferred
informational

These categories MUST remain distinct.

For example:

requires latency <= L

is fundamentally different from:

prefer latency <= L

---

133. Portability Classes

HDL constructs SHOULD be classifiable as:

portable
conditionally portable
target-specific
dialect-specific
implementation-specific

This classification SHOULD be available to tooling and diagnostics.

---

134. Portable Core

The portable HDL core SHOULD contain:

modules
ports
interfaces
signals
nets
registers
memories
processes
combinational behavior
sequential behavior
state machines
pipelines
parameters
generics
protocols
timing intent
contracts
resources
capabilities
policies
verification
simulation intent

It MUST NOT require a specific vendor or physical device.

---

135. Dialect Boundary

A dialect MAY add syntax and semantics.

Every dialect MUST declare:

dialect identity
version
owner
scope
compatibility
capabilities
target requirements
semantic extensions

A dialect MUST NOT silently alter the semantics of portable constructs.

---

136. Interoperability

HDL interoperability MUST use:

grammar/interoperability/

Existing HDL interoperability grammar MUST remain an integration boundary rather than becoming a competing HDL language.

External HDL formats may be imported through:

external parser
      ↓
normalized semantic representation
      ↓
Zamani semantic model

---

137. External HDL Formats

External HDL formats may include vendor or established hardware languages.

They MUST NOT be inserted wholesale into the universal Zamani parser.

The preferred architecture is:

external HDL
    ↓
external frontend
    ↓
normalized semantic model
    ↓
Zamani

---

138. Canonical Syntax Rule

No specialized HDL grammar may define its own lexical token if an existing canonical token already represents the same lexical concept.

All lexical ownership belongs to:

grammar/lexer/tokens.g4
grammar/antlr/ZamaniLexer.g4

---

139. Keyword Rule

HDL-specific keywords MUST be added only when they provide genuine syntactic value.

Application-specific names MUST remain identifiers unless there is a language-level reason to reserve them.

The HDL grammar MUST NOT create keyword explosion.

---

140. Expression Reuse

HDL expressions MUST reuse the universal expression grammar whenever the semantics permit.

HDL-specific expression rules are justified only where ordinary Zamani expressions cannot accurately represent hardware semantics.

---

141. Type Reuse

HDL MUST reuse the universal type system.

Hardware-specific types MUST be integrated into the canonical type architecture.

No separate HDL-only type checker may become authoritative.

---

142. Generic Reuse

HDL generics MUST integrate with the universal generic/type constraint model.

Hardware generics MUST NOT fork the generic semantics.

---

143. Contract Reuse

HDL contracts MUST use the universal validation architecture.

No independent HDL contract semantics may be created.

---

144. Effect Reuse

HDL effects MUST use the universal effects architecture.

No independent hardware effect taxonomy may become authoritative.

---

145. Resource Reuse

HDL resources MUST use the universal resource model.

No hardware grammar may define fixed global hardware capacities.

---

146. Capability Reuse

HDL capabilities MUST use the universal capability model.

New capabilities MUST be metadata-driven where possible.

---

147. Policy Reuse

HDL policies MUST use the universal policy architecture.

Security, resource, execution, adaptation, and deployment policies MUST be composable.

---

148. Provenance Reuse

HDL provenance MUST use the universal provenance architecture.

The same provenance model must be usable for:

classical
quantum
HDL
AI
data
distributed
networking

---

149. Explainability

The compiler SHOULD be able to explain:

why this target was selected
why another target was rejected
why a resource was required
why an optimization was applied
why a transformation was rejected
why a timing constraint failed
why a capability was missing

These explanations SHOULD be represented through the universal provenance/evidence system.

---

150. POCO-REAF Requirement

HDL source SHOULD remain unchanged when moving between targets provided that:

1. the source semantics remain the same;
2. the target provides required capabilities;
3. resource requirements can be satisfied;
4. policies permit the realization;
5. contracts remain satisfiable.

Target adaptation belongs downstream.

---

151. POCO-REAF Does Not Mean Universal Feasibility

POCO-REAF does NOT mean:

«every program must run on every possible machine.»

It means:

«the source program should not need to be rewritten merely because the implementation target changes, provided the target can satisfy the program's declared semantics and requirements.»

If a target cannot satisfy the requirements, the compiler MUST report infeasibility.

---

152. Source Stability

The source program MUST remain semantically stable across realizations.

The compiler MAY generate different:

- schedules;
- placements;
- routes;
- technology mappings;
- implementation structures.

These differences MUST NOT change source semantics.

---

153. Reproducibility

Production HDL builds SHOULD record:

source hash
language version
grammar version
AST version
semantic version
IR version
compiler version
dialect versions
target description
capability set
resource description
policy versions

This belongs to the compatibility/provenance system.

---

154. Rust Implementation Contract

The implementation of the HDL compiler, parser integration, semantic analysis, and supporting infrastructure MUST target:

Rust 1.97 or later
Rust 2021 edition

Zamani-owned Rust code MUST NOT use:

unsafe

The implementation SHOULD prefer:

- ownership;
- borrowing;
- safe concurrency;
- enums;
- pattern matching;
- typed errors;
- "Result";
- "Option";
- deterministic collections where ordering matters;
- explicit validation;
- immutable data where practical.

This implementation requirement MUST NOT leak into Zamani source semantics.

---

155. Memory Safety

The HDL frontend and semantic system MUST remain memory-safe without "unsafe".

Large designs MUST be handled through scalable data structures and streaming/incremental techniques where appropriate.

The absence of "unsafe" MUST NOT become an excuse to introduce artificial design-size limits.

---

156. Incremental Compilation

Production tooling SHOULD support incremental analysis where practical.

Changes to one HDL construct SHOULD NOT require rebuilding unrelated semantic regions when dependency analysis proves they are unaffected.

Incremental compilation MUST preserve semantic equivalence with full compilation.

---

157. Large-Design Handling

The implementation SHOULD support large designs through:

- lazy structures where appropriate;
- incremental parsing;
- incremental semantic analysis;
- structural sharing;
- bounded diagnostic storage;
- streaming artifacts;
- parallel analysis where deterministic;
- scalable graph representations.

These are implementation strategies, not language limits.

---

158. Concurrency of Compilation

Compiler analysis MAY execute in parallel.

Parallel compiler execution MUST remain deterministic with respect to semantic results.

Shared mutable state SHOULD be minimized.

---

159. Diagnostics at Scale

A compiler handling very large designs SHOULD support configurable diagnostic collection without changing semantic results.

The number of reported diagnostics MAY be operationally bounded.

Such a diagnostic-output bound MUST NOT be confused with a language limit on the number of HDL errors or constructs.

---

160. Semantic Graphs

HDL semantic analysis SHOULD model relationships as graphs where appropriate.

Potential graph entities include:

modules
instances
signals
nets
ports
processes
states
transitions
clocks
domains
memories
pipelines
resources
capabilities
contracts
policies

The graph representation MUST remain scalable.

---

161. Dependency Analysis

Dependency analysis MUST support:

- combinational dependencies;
- sequential dependencies;
- module dependencies;
- generic dependencies;
- clock dependencies;
- reset dependencies;
- protocol dependencies;
- resource dependencies.

No artificial graph-size maximum may be imposed by the language.

---

162. Cycles

Cycles MUST be classified semantically.

Examples:

legal sequential feedback
illegal combinational cycle
intentional protocol cycle
clock-domain dependency

The compiler MUST not classify every graph cycle as invalid.

---

163. Hardware State

Hardware state MUST be explicit in the semantic model.

State may arise from:

- registers;
- memories;
- state machines;
- feedback structures;
- persistent storage.

A construct MUST NOT accidentally create hidden state.

---

164. Initialization

Initialization semantics MUST be explicit.

The language MUST distinguish:

source initialization intent

from:

target power-up behavior

where those are not equivalent.

---

165. Reset and Initialization

Reset and initialization MUST remain separate semantic concepts unless the specification explicitly defines them as equivalent in a particular context.

A target-specific power-up property MUST NOT silently become a portable reset guarantee.

---

166. Timing and Functional Semantics

Timing MAY be:

non-observable implementation detail

or:

explicitly observable semantic property

The distinction MUST be determined by the program's declarations and contracts.

An optimizer may change non-observable implementation timing.

It MUST preserve observable timing contracts.

---

167. Pipeline Retiming

Retiming MAY be performed where legal.

Retiming MUST preserve:

- functional behavior;
- required latency;
- required protocol behavior;
- explicit state semantics;
- contracts.

If the source explicitly exposes stage identity, retiming may require additional semantic permission.

---

168. Resource Sharing

Resource sharing MAY be performed when semantics permit it.

The compiler MUST consider:

- concurrency;
- timing;
- resource requirements;
- state;
- effects;
- policies.

---

169. Replication

Replication MAY be used to satisfy throughput requirements.

The compiler MUST NOT replicate stateful logic in a way that changes semantic behavior unless the transformation is proven valid.

---

170. Hardware/Software Memory Model

Where HDL interacts with software-visible memory, the memory model MUST be explicitly defined.

The model MUST cover, where applicable:

- ordering;
- visibility;
- synchronization;
- ownership;
- atomicity;
- coherence;
- DMA;
- buffering.

The source MUST NOT rely on accidental target behavior.

---

171. Interfaces to Runtime

Hardware runtime interaction MUST use explicit interfaces.

The runtime MUST consume:

capabilities
resources
effects
contracts
policies
provenance

where applicable.

---

172. Runtime Discovery

Runtime hardware discovery belongs outside the grammar.

The runtime may discover:

available devices
resources
capabilities
health
topology

The semantic program remains unchanged.

---

173. Health-Aware Realization

Where adaptive execution is supported, target health MAY influence realization.

Health changes MUST be governed by policy.

A degraded target MUST NOT cause arbitrary semantic changes.

---

174. Fallbacks

Hardware fallback behavior MAY be declared.

Fallbacks MUST preserve declared semantics.

A fallback MAY change implementation strategy but not required program meaning.

---

175. Recovery

Recovery MAY include:

- retry;
- reconfiguration;
- relocation;
- redundancy;
- alternate implementation;
- simulation fallback.

Recovery behavior MUST be represented in the execution/resilience architecture.

---

176. Simulation Fallback

A hardware construct MAY be executed in a simulator when physical realization is unavailable and the program permits simulation.

Simulation MUST be clearly distinguished from physical execution.

---

177. Verification Before Realization

Production hardware flows SHOULD permit:

semantic validation
        ↓
formal/static verification
        ↓
simulation
        ↓
resource analysis
        ↓
synthesis

The exact ordering MAY vary provided semantic dependencies are respected.

---

178. Cross-Domain Integration

HDL MUST be able to participate in mixed-domain programs.

A complete program MAY combine:

classical computation
+
AI model
+
reasoning
+
learning
+
hardware accelerator
+
quantum operation
+
measurement
+
distributed execution
+
networking

The HDL layer must not require a separate language mode.

---

179. AI/Hardware Integration

AI-related hardware MAY consume universal AI semantic constructs.

For example:

learn
reason
infer
adapt

may result in hardware realization.

The HDL subsystem MUST NOT duplicate AI semantics.

---

180. Neural/Hardware Composition

Neural computation MAY be represented as:

AI semantic operation
        ↓
hardware capability requirements
        ↓
hardware realization

rather than requiring application-specific hardware keywords.

---

181. Data/Hardware Integration

Hardware data paths MAY consume the universal data type and data interoperability systems.

Data schemas MUST remain compatible with:

grammar/data/
grammar/types/
grammar/interoperability/

---

182. Networking/Hardware Integration

Network hardware MAY integrate with:

grammar/networking/
grammar/distributed/
grammar/security/

Network semantics MUST remain distinct from physical implementation.

---

183. HDL and Contracts

The complete HDL contract chain is:

requires
    ↓
type validation
    ↓
effect validation
    ↓
capability validation
    ↓
resource validation
    ↓
contract validation
    ↓
policy validation
    ↓
provenance
    ↓
realization

No stage may silently bypass an earlier semantic requirement.

---

184. HDL and Evidence

Hardware claims SHOULD be supported by evidence.

Examples:

timing claim
    ↓
timing analysis evidence

functional claim
    ↓
formal verification evidence

resource claim
    ↓
resource analysis evidence

simulation claim
    ↓
simulation evidence

---

185. HDL and Explainability

Compilation tooling SHOULD explain hardware realization decisions.

A developer SHOULD be able to determine:

why a resource was selected
why a resource was rejected
why a module was replicated
why a pipeline was inserted
why timing failed
why a target was rejected

---

186. Conformance Requirements

An HDL implementation is conformant only if:

1. it recognizes all required normative syntax;
2. it produces the required AST representation;
3. it implements required semantic validation;
4. it integrates universal types;
5. it integrates effects;
6. it integrates capabilities;
7. it integrates resources;
8. it integrates contracts;
9. it integrates policies;
10. it preserves provenance;
11. it respects target independence;
12. it does not introduce prohibited universal limits;
13. it integrates with the canonical IR architecture;
14. it produces required diagnostics;
15. it passes the required conformance tests.

---

187. Required Test Categories

HDL MUST have tests covering:

lexical
parser
AST
semantic
types
generics
connectivity
clocking
reset
memory
timing
protocol
contracts
resources
capabilities
effects
policies
provenance
verification
simulation
synthesis intent
physical intent
co-design
quantum boundary
distributed integration
interoperability
compatibility
scalability
determinism
negative cases
boundary cases

---

188. Positive Tests

Every HDL construct MUST have at least one valid test.

Tests MUST include representative:

- minimal;
- generic;
- parameterized;
- nested;
- cross-domain;
- large symbolic cases.

---

189. Negative Tests

Every semantic restriction MUST have negative tests.

Examples:

invalid type
invalid width
invalid connection
invalid driver
invalid clock crossing
invalid reset
invalid protocol
unsatisfied capability
unsatisfied resource
contract violation
policy violation
dialect mismatch

---

190. Boundary Tests

Boundary tests MUST combine multiple HDL systems.

Examples:

generic module + pipeline
clock + CDC + reset
memory + interface + protocol
hardware + software ABI
hardware + quantum control
hardware + simulation
hardware + resource requirement
hardware + policy

---

191. Scalability Tests

Scalability tests MUST verify that no artificial finite limit is accidentally introduced.

Tests SHOULD use:

symbolic widths
symbolic depths
large parameter values
many modules
many ports
many instances
many clock domains
large generated structures
large state machines
large pipelines
large dependency graphs

The exact numerical test values MUST NOT become normative language limits.

---

192. Tiny-Target Tests

At the opposite extreme, tests MUST verify that the same semantic architecture can describe very small hardware.

Examples:

single signal
single combinational operation
single register
single-state machine
small memory
minimal interface

The language MUST not require large-system infrastructure for tiny programs.

---

193. Cross-Scale Tests

The same generic HDL source SHOULD be instantiated at substantially different semantic scales.

The test MUST verify:

same source
different parameters
different resources
different targets

while preserving semantic equivalence.

---

194. Target Portability Tests

A production implementation SHOULD test the same source against multiple target descriptions.

For example:

small FPGA
large FPGA
ASIC model
accelerator model
simulator
heterogeneous target

The source MUST remain unchanged unless target-specific syntax is deliberately used.

---

195. Resource Failure Tests

The compiler MUST test target infeasibility.

A resource failure MUST:

- identify the requirement;
- identify the unavailable capability/resource;
- preserve source semantics;
- avoid silent degradation;
- provide an actionable diagnostic where possible.

---

196. Capability Failure Tests

Capability failures MUST be distinguished from resource failures where possible.

For example:

capability missing

is different from:

capability exists but capacity is insufficient

---

197. Determinism Tests

The compiler MUST verify that repeated compilation under identical inputs produces equivalent semantic output.

Where serialized artifacts are required to be byte-for-byte reproducible, deterministic ordering MUST be used.

---

198. Provenance Tests

Generated and transformed hardware MUST retain provenance.

Tests MUST verify:

source
→ generic
→ specialization
→ generation
→ optimization
→ lowering

can be traced where provenance is required.

---

199. Compatibility Tests

The HDL subsystem MUST test:

- language-version compatibility;
- grammar-version compatibility;
- dialect compatibility;
- AST compatibility;
- semantic compatibility;
- IR compatibility.

---

200. Integration Test

The mandatory end-to-end HDL test MUST exercise:

Zamani source
    ↓
lexer
    ↓
ANTLR parser
    ↓
HDL AST
    ↓
semantic analysis
    ↓
type checking
    ↓
effect checking
    ↓
capability checking
    ↓
resource checking
    ↓
contract checking
    ↓
policy checking
    ↓
provenance
    ↓
canonical semantic/IR boundary
    ↓
hardware lowering
    ↓
target realization plan

---

201. Universal Cross-Domain Test

At least one integration test SHOULD combine:

generic hardware
+
classical computation
+
AI inference
+
learning
+
resource requirements
+
capabilities
+
effects
+
contracts
+
policy
+
provenance
+
quantum control
+
measurement
+
parallel execution
+
simulation
+
hardware realization

This is the principal cross-domain POCO-REAF validation.

---

202. Required Example Programs

The HDL conformance suite SHOULD contain:

minimal.zm
classical.zm
hdl.zm
generic-hardware.zm
parameterized-hardware.zm
memory.zm
pipeline.zm
clocking.zm
cdc.zm
reset.zm
protocol.zm
verification.zm
simulation.zm
synthesis.zm
physical-intent.zm
co-design.zm
accelerator.zm
quantum-hardware.zm
hybrid-hardware.zm
poco-reaf.zm

---

203. File-Level Completion Contract

Every HDL grammar file MUST document:

Purpose
Owns
Does Not Own
Public Rules
Private Rules
Lexer Dependencies
Grammar Dependencies
AST Contract
Semantic Contract
Type Contract
Effect Contract
Capability Contract
Resource Contract
Contract Contract
Policy Contract
Provenance Contract
IR Contract
Quantum Boundary
HDL Boundary
Backend Boundary
Diagnostics
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility
Integration
Completion Criteria

This contract is mandatory because a file MUST be independently completable.

---

204. Dependency Declaration

Each HDL grammar file SHOULD explicitly declare:

DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
IR_OWNER:
TEST_OWNER:
SPEC_OWNER:

For example:

SPEC_OWNER:
grammar/spec/hdl.md

AST_OWNER:
domain-neutral frontend AST

SEMANTIC_OWNER:
HDL semantic analysis

IR_OWNER:
canonical semantic/IR lowering

TEST_OWNER:
grammar/tests/hdl/

---

205. Completion Criteria

An HDL grammar file is not complete merely because ANTLR accepts it.

It is complete only when:

specification
    ↓
lexer
    ↓
grammar
    ↓
AST
    ↓
semantic model
    ↓
type/effect/capability/resource validation
    ↓
contracts/policies
    ↓
provenance
    ↓
canonical IR boundary
    ↓
tests

are all accounted for.

---

206. Ownership Rule

No HDL feature may have two semantic owners.

If two files appear to own the same semantic construct:

1. designate one canonical owner;
2. convert the other into a composition/delegation layer;
3. remove duplicated semantic authority;
4. update integration metadata;
5. add a regression test.

---

207. Duplication Rule

The repository MUST NOT contain duplicate definitions of:

- HDL keywords;
- HDL identifiers;
- hardware generic semantics;
- hardware type semantics;
- contract semantics;
- resource semantics;
- capability semantics;
- effect semantics;
- provenance semantics.

Delegation is preferred over duplication.

---

208. Composition Root Rule

"grammar/hdl/hdl.g4" MUST remain small enough to understand as an architecture map.

A developer reading it SHOULD immediately see:

HDL root
   ├── modules
   ├── interfaces
   ├── behavior
   ├── state
   ├── timing
   ├── connectivity
   ├── generation
   ├── verification
   ├── simulation
   ├── synthesis
   ├── physical intent
   └── co-design

---

209. Specification-to-Grammar Rule

"grammar/spec/hdl.md" MUST describe meaning.

"grammar/hdl/*.g4" MUST describe syntax.

The specification MUST NOT become a second parser implementation.

The grammar MUST NOT become the semantic specification.

---

210. Specification-to-Implementation Rule

The implementation MUST NOT invent semantics absent from this specification or another authoritative specification.

Experimental behavior MUST be explicitly marked experimental.

---

211. Experimental Features

An experimental HDL feature MUST identify:

status
owner
proposal
semantic model
grammar
AST
implementation
tests
compatibility policy

Experimental constructs MUST NOT silently become stable language law.

---

212. Future Hardware

The architecture MUST remain open to hardware that does not yet exist.

Future hardware MUST be representable through:

capabilities
resources
dialects
target descriptions
semantic operations

rather than requiring a redesign of the universal grammar.

---

213. Future Computational Models

HDL MUST remain compatible with future computational technologies.

A future accelerator, memory technology, interconnect, or computational substrate SHOULD be representable through existing abstractions whenever its semantics fit them.

Only genuinely new semantic concepts should require language evolution.

---

214. Open-World Hardware Model

The hardware model is intentionally open-world.

The grammar MUST NOT assume that today's:

CPU
GPU
FPGA
ASIC
accelerator
memory
interconnect

are the complete set of possible realization classes.

---

215. Domain-Neutral Hardware Intent

The strongest portable abstraction is:

intent
+
requirements
+
capabilities
+
constraints
+
contracts
+
policies

rather than:

device
+
fixed resource count
+
fixed topology

---

216. Final Semantic Model

The HDL semantic model MUST fit the universal Zamani model:

VALUE
  │
TYPE
  │
OPERATION
  │
├──────────────┬──────────────┐
│              │              │
EFFECT     CAPABILITY      RESOURCE
│              │              │
└──────────────┼──────────────┘
               │
         REQUIREMENT
               │
         CONSTRAINT
               │
            POLICY
               │
           CONTRACT
               │
           EVIDENCE
               │
          PROVENANCE
               │
        SEMANTIC MODEL
               │
        CANONICAL IR
               │
        HARDWARE LOWERING
               │
      TARGET REALIZATION

---

217. Final HDL Architecture

The production architecture is:

                    ZAMANI
                       │
        ┌──────────────┼──────────────┐
        │              │              │
    Classical       Quantum          HDL
        │              │              │
        │        quantum::ir         │
        │              │              │
        └──────────────┼──────────────┘
                       │
              Domain-Neutral
              Semantic Model
                       │
       ┌───────────────┼────────────────┐
       │               │                │
     Types          Effects        Capabilities
       │               │                │
       └───────────────┼────────────────┘
                       │
                 Resources
                       │
                  Contracts
                       │
                   Policies
                       │
                 Provenance
                       │
                Canonical IR
                       │
              Target-independent
                 optimization
                       │
       ┌───────────────┼────────────────┐
       │               │                │
   Scheduling       Lowering       Resilience
       │               │                │
       └───────────────┼────────────────┘
                       │
                  Realization
                       │
       ┌───────┬───────┼───────┬────────┐
       │       │       │       │        │
      CPU     GPU     FPGA    ASIC   Accelerator
       │       │       │       │        │
       └───────┴───────┼───────┴────────┘
                       │
                 Distributed /
                 HPC / Future

---

218. Production-Ready Definition

The HDL subsystem MUST NOT be declared production-ready merely because the parser accepts HDL syntax.

It is production-ready only when every supported construct has a complete path:

SPECIFICATION
      ↓
LEXER
      ↓
GRAMMAR
      ↓
AST
      ↓
SEMANTICS
      ↓
TYPE CHECKING
      ↓
EFFECT CHECKING
      ↓
CAPABILITY CHECKING
      ↓
RESOURCE CHECKING
      ↓
CONTRACT CHECKING
      ↓
POLICY CHECKING
      ↓
PROVENANCE
      ↓
CANONICAL IR
      ↓
OPTIMIZATION
      ↓
LOWERING
      ↓
SCHEDULING
      ↓
PLACEMENT
      ↓
ROUTING
      ↓
VERIFICATION
      ↓
RESILIENCE
      ↓
TARGET REALIZATION

And every construct MUST have:

positive test
negative test
boundary test
scalability test
cross-domain test
determinism test
compatibility test

---

219. Non-Negotiable Production Rules

The following rules are absolute for Zamani HDL.

Rule 1 — No artificial universal capacity limits

The language MUST NOT impose finite machine-capacity constants.

Rule 2 — No physical realization leakage

Portable HDL MUST NOT implicitly encode physical hardware identity.

Rule 3 — No duplicate semantic systems

HDL MUST reuse universal types, effects, resources, capabilities, contracts, policies, and provenance.

Rule 4 — No second quantum IR

Quantum semantics MUST converge on "quantum::ir".

Rule 5 — No second concurrency model

Hardware parallelism MUST integrate with Zamani concurrency semantics.

Rule 6 — No silent semantic degradation

A target that cannot satisfy requirements MUST produce a diagnostic rather than silently changing program meaning.

Rule 7 — No unrestricted adaptation

Reconfiguration/adaptation MUST be governed by capability, policy, effects, contracts, resources, and provenance.

Rule 8 — No vendor pollution

Vendor-specific behavior MUST use explicit dialect/target mechanisms.

Rule 9 — No lexical duplication

The canonical lexer remains the only lexical authority.

Rule 10 — No grammar-level hardware inventory

Hardware discovery belongs outside the grammar.

Rule 11 — No unsafe Rust

Zamani-owned Rust implementation MUST use safe Rust only.

Rule 12 — Source portability

The same source SHOULD remain valid across targets whenever target capabilities satisfy its requirements.

Rule 13 — Semantic portability

Different physical implementations MUST preserve the same declared semantic contract.

Rule 14 — Provenance

Transformations and generated structures MUST remain traceable where provenance is required.

Rule 15 — Independent-file completion

Every HDL grammar file MUST have an explicit ownership and integration contract before being considered complete.

---

220. Definition of Done for "grammar/spec/hdl.md"

This specification is complete when:

- "grammar/hdl/hdl.g4" uses it as its semantic authority;
- all HDL delegates reference it;
- the standalone/embedded parser distinction is implemented;
- HDL uses the canonical lexer;
- HDL uses the universal type system;
- HDL uses universal effects;
- HDL uses universal capabilities;
- HDL uses universal resources;
- HDL uses universal contracts;
- HDL uses universal policies;
- HDL uses universal provenance;
- HDL integrates with the domain-neutral AST;
- HDL integrates with the canonical semantic model;
- HDL does not introduce a competing universal IR;
- quantum interaction terminates at "quantum::ir";
- target realization remains downstream;
- physical intent is explicitly scoped;
- no artificial hardware capacities are imposed;
- generic hardware is scalable;
- generated hardware is provenance-aware;
- timing/clock/reset semantics are explicit;
- verification and simulation share the semantic model;
- hardware/software co-design uses the interoperability architecture;
- all supported features have conformance tests;
- large and tiny designs are both covered;
- target infeasibility is reported rather than silently rewritten;
- the implementation is compatible with Rust 1.97+;
- Zamani-owned Rust code uses no "unsafe";
- deterministic compilation is tested;
- compatibility and migration are specified.

---

221. Final Architectural Statement

Zamani HDL is therefore not a language for describing one class of hardware.

It is a portable semantic hardware description system embedded inside the universal Zamani programming language.

Its source describes:

what must happen
what may happen
what is required
what is forbidden
what capabilities are needed
what resources are needed
what contracts must hold
what policies govern realization
what evidence/provenance must be preserved

Its implementation determines:

how
where
when
with which physical resources
on which target
using which technology

This separation is the foundation of scalable hardware compilation.

The resulting architecture permits the same semantic HDL program to be considered for:

tiny embedded hardware
single-core systems
multicore systems
CPU-integrated accelerators
GPU-adjacent accelerators
FPGA
ASIC
reconfigurable hardware
specialized accelerators
quantum-control systems
simulation
HPC
cluster
distributed hardware
future computational substrates

without turning any particular machine configuration into a universal language limitation.

That is the required HDL foundation for POCO-REAF from the smallest realizable system to arbitrarily large realizations permitted by available resources, while preserving one language, one semantic model, one source meaning, and one scalable architecture.