Zamani HDL Specification

File: "grammar/spec/hdl.md"
Language: Zamani
Domain: Hardware Description, Hardware/Software Co-Design, Reconfigurable Computing, ASIC/FPGA/Accelerator Intent
Status: Normative production specification
Specification role: Canonical HDL semantic and syntactic contract
Implementation baseline: Rust 1.97 / Rust 1.97.1, Rust 2021 edition
Implementation safety: Zamani-owned Rust implementation MUST NOT use "unsafe"
Canonical grammar composition root: "grammar/Zamani.g4"
HDL grammar implementation: "grammar/hdl/"
Canonical semantic boundary: Domain-neutral semantic model followed by the appropriate hardware/domain IR
Quantum boundary: Quantum constructs MUST ultimately integrate through the existing "quantum::ir"; this document MUST NOT introduce a second quantum IR.

---

1. Purpose

This document defines the normative contract for Hardware Description Language capabilities in Zamani.

Zamani HDL is not intended to be merely another syntax for describing an FPGA or ASIC.

It is a first-class part of the Zamani Universal Computing Language and MUST support:

- digital hardware description;
- hardware structure;
- hardware behavior;
- combinational logic;
- sequential logic;
- clocks;
- resets;
- signals;
- nets;
- registers;
- memories;
- interfaces;
- protocols;
- state machines;
- pipelines;
- parameterized hardware;
- generated hardware;
- module composition;
- hardware/software co-design;
- accelerator description;
- CPU-related hardware;
- GPU-related hardware;
- FPGA-related hardware;
- ASIC-related hardware;
- reconfigurable hardware;
- heterogeneous accelerators;
- quantum control hardware where appropriate;
- embedded systems;
- timing intent;
- verification intent;
- synthesis intent;
- simulation intent;
- physical implementation intent;
- resource requirements;
- capability requirements;
- deployment constraints.

The central design principle is:

«Describe what the hardware computation means, not an accidental description of one particular physical machine.»

The HDL system MUST therefore support:

Program Once → Compile Once → Run Everywhere, Anywhere, Forever (POCO-REAF).

POCO-REAF means that portable HDL source can describe computational and hardware intent without requiring the source program to be rewritten merely because the available implementation changes in:

- device capacity;
- FPGA family;
- ASIC implementation;
- fabrication technology;
- accelerator size;
- memory capacity;
- interconnect;
- clock capability;
- available parallelism;
- available synthesis resources;
- deployment scale.

A program requiring capabilities that a target cannot provide MAY be rejected.

The compiler MUST NOT silently change the program's semantics merely to fit an incapable target.

---

2. Normative Language

The following terms are normative:

- MUST — mandatory.
- MUST NOT — prohibited.
- SHOULD — recommended unless a documented technical reason exists otherwise.
- SHOULD NOT — discouraged unless justified.
- MAY — permitted.
- SEMANTIC — part of the meaning of the program.
- INTENT — a portable declaration of desired behavior, capability, resource, timing, or implementation property.
- TARGET REALIZATION — the mapping of semantic hardware intent to an actual target.
- IMPLEMENTATION LIMIT — a practical limitation of a particular compiler, synthesizer, simulator, device, runtime, or deployment environment.
- LANGUAGE LIMIT — a restriction imposed by the Zamani language itself.
- RESOURCE — a quantity or kind of computational/physical capacity.
- CAPABILITY — an ability provided by a realization.
- CONSTRAINT — a condition that a valid realization MUST satisfy.
- PREFERENCE — an optimization preference that does not alter semantic validity.
- HINT — optional information intended to help implementation without changing semantics.
- PHYSICAL REALIZATION — a concrete mapping onto actual hardware resources.

---

3. Authority

The HDL specification participates in the repository's overall authority hierarchy.

The intended relationship is:

grammar/specification/language.md
            │
            ▼
grammar/specification/syntax.md
            │
            ▼
grammar/spec/hdl.md
            │
            ├── lexical contracts
            ├── type contracts
            ├── semantic contracts
            ├── resource contracts
            └── hardware contracts
            │
            ▼
grammar/hdl/*.g4
            │
            ▼
grammar/Zamani.g4
            │
            ▼
src/lexer.rs
            │
            ▼
src/parser.rs
            │
            ▼
src/frontend/ast/
            │
            ▼
structural validation
            │
            ▼
semantic analysis
            │
            ▼
hardware semantic representation
            │
            ├── verification
            ├── optimization
            ├── synthesis
            ├── scheduling
            ├── placement
            ├── routing
            └── target lowering
            │
            ▼
actual hardware / simulation / deployment

This document defines HDL meaning.

"grammar/hdl/*.g4" defines HDL syntax.

"grammar/Zamani.g4" composes the language.

"src/lexer.rs" owns actual lexical recognition.

"src/parser.rs" owns parsing into the frontend AST.

"src/frontend/ast/" owns the domain-neutral AST representation.

Semantic analysis owns meaning, type correctness, width correctness, clock-domain correctness, resource requirements, capability requirements, and hardware legality.

Synthesis, placement, routing, scheduling, hardware discovery, HAL, runtime, and deployment MUST NOT redefine source-language semantics.

---

4. Existing HDL Files

The existing HDL directory MUST be extended rather than unnecessarily renamed or replaced.

The current repository already contains HDL grammar components including:

grammar/hdl/README.md
grammar/hdl/hdl.g4
grammar/hdl/clocks.g4
grammar/hdl/combinational.g4
grammar/hdl/hardware-dialects.g4
grammar/hdl/hardware-generics.g4
grammar/hdl/hardware-interfaces.g4
grammar/hdl/hardware-modules.g4
grammar/hdl/hardware-parameters.g4
grammar/hdl/memories.g4
grammar/hdl/pipelines.g4
grammar/hdl/ports.g4
grammar/hdl/processes.g4
grammar/hdl/registers.g4

These files MUST remain part of the architecture unless an explicit repository audit proves that a particular file is obsolete.

The repository already establishes "grammar/hdl/hdl.g4" as a parser grammar using "ZamaniTokens". That is the correct general direction because HDL grammar MUST consume the canonical lexical vocabulary rather than creating an independent HDL lexer.

The HDL specification therefore defines how these files integrate.

---

5. Critical Correction: Standalone Versus Embedded HDL Parsing

The existing HDL parser grammar currently exposes:

hdlDesign
    : hdlModuleDeclaration+
    EOF
    ;

This is acceptable only for a standalone HDL parsing entry point.

It MUST NOT be used as the embedded HDL entry point of the universal Zamani parser because the universal parser owns the final "EOF".

The architecture MUST distinguish:

hdlDesign
    : hdlModuleDeclaration+
    EOF
    ;

from the compositional form:

hdlDesignBody
    : hdlModuleDeclaration+
    ;

The canonical universal grammar MUST use the non-EOF domain entry.

Therefore:

Zamani.g4
    └── program
          └── item
                └── hdlItem
                      └── hdlModuleDeclaration

The standalone HDL entry MAY remain for:

- HDL-only tooling;
- HDL conformance tests;
- HDL parser tests;
- HDL interoperability;
- external HDL import/export.

This distinction prevents the HDL grammar from becoming incompatible with the universal Zamani grammar.

---

6. HDL Is One Zamani Domain, Not a Separate Language

HDL MUST remain part of Zamani.

A Zamani program MAY contain:

classical computation
        ↓
hardware module
        ↓
accelerator invocation
        ↓
quantum control
        ↓
measurement
        ↓
classical processing

without switching languages.

HDL MUST therefore reuse:

- identifiers;
- qualified names;
- expressions;
- types;
- generics;
- attributes;
- modules;
- functions;
- effects;
- resources;
- capabilities;
- concurrency;
- diagnostics;
- source locations;
- imports;
- exports;
- contracts.

HDL-specific constructs MUST only be introduced when hardware semantics genuinely require them.

---

7. HDL Ownership

HDL grammar owns syntax for:

- hardware modules;
- ports;
- interfaces;
- signals;
- nets;
- registers;
- memories;
- clocks;
- resets;
- processes;
- combinational behavior;
- sequential behavior;
- state machines;
- pipelines;
- module instances;
- generated hardware;
- hardware parameters;
- hardware generics;
- hardware assertions;
- timing declarations;
- hardware connectivity;
- hardware-local declarations;
- hardware intent;
- hardware verification intent.

HDL grammar MUST NOT own:

- physical FPGA selection;
- ASIC selection;
- fabrication;
- placement;
- routing;
- timing closure;
- synthesis algorithms;
- resource allocation algorithms;
- physical pin assignment;
- vendor device databases;
- QEC implementation;
- ZQN implementation;
- quantum routing;
- runtime hardware discovery;
- HAL implementation;
- compiler optimization algorithms;
- scheduler implementation;
- simulator implementation.

---

8. Hardware Intent Versus Hardware Realization

This distinction is fundamental.

Source code MAY say:

requires capability("memory")
requires capability("parallel.compute")
requires capability("high_speed.interconnect")

It MAY say:

requires throughput >= desired_throughput

It MAY say:

requires latency <= desired_latency

It MAY say:

prefer accelerator("matrix")

But the source MUST NOT silently mean:

use FPGA device 0
use LUT 17
use BRAM 3
use DSP 8
use physical pin 42
use routing channel 7

unless such physical identity is explicitly part of a target-specific program.

Portable HDL MUST remain abstract.

---

9. No Artificial Hardware Limits

Zamani HDL MUST NOT impose universal constants such as:

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
MAX_CLOCKS
MAX_DOMAINS
MAX_DEVICES
MAX_LUTS
MAX_BRAMS
MAX_DSPS
MAX_FPGAS
MAX_ASICS
MAX_ACCELERATORS

These MUST NOT become grammar-level limitations.

The following are also prohibited as universal language limits:

MAX_BITS
MAX_BUS_WIDTH
MAX_MEMORY_DEPTH
MAX_MEMORY_WIDTH
MAX_PIPELINE_DEPTH
MAX_PARALLEL_UNITS
MAX_CLOCK_DOMAINS
MAX_STATE_COUNT
MAX_CHANNEL_COUNT

A program MAY explicitly request a quantity.

For example:

width = 1024

is program semantics.

But:

width <= 1024

MUST NOT be imposed by the language merely because an existing implementation happens to use 1024-bit hardware.

---

10. Meaning of "Infinity"

In this specification, "infinity" means:

«No artificial finite limit is imposed by the language architecture where the quantity is semantically parameterizable.»

It does NOT mean that physical hardware is infinite.

For example:

pipeline stages

are unbounded by the grammar.

A particular FPGA may have insufficient resources for a requested pipeline.

The compiler MUST then report a resource/capability failure.

It MUST NOT change:

128 stages

into:

32 stages

without an explicit semantic transformation approved by the program's constraints.

---

11. Parameterization

Hardware descriptions MUST be parameterizable.

Existing repository components such as:

hardware-generics.g4
hardware-parameters.g4

MUST provide reusable parameterization mechanisms.

Parameters MAY represent:

- widths;
- depths;
- dimensions;
- latency;
- throughput;
- pipeline stages;
- number of channels;
- number of instances;
- memory dimensions;
- protocol properties;
- interface properties;
- timing values;
- resource requirements;
- capability constraints.

Parameters MAY be:

- constants;
- expressions;
- type parameters;
- generic parameters;
- symbolic values;
- compile-time values;
- configuration values;
- externally supplied values where explicitly permitted.

No fixed parameter count is permitted.

---

12. Generic Hardware

Conceptually:

module MatrixUnit<ROWS, COLS, ELEMENT> {
    ...
}

is valid architectural syntax.

The existing "hardware-modules.g4" already uses parameterized module concepts of this kind.

Generic hardware MUST behave like generic software:

generic definition
       ↓
constraint checking
       ↓
specialization
       ↓
semantic hardware model
       ↓
target realization

Specialization MUST NOT alter the generic program's semantics.

---

13. Hardware Modules

A hardware module defines a composable unit of hardware behavior and/or structure.

Conceptually:

module Name<parameters>(ports) {
    members
}

A module MAY contain:

- parameters;
- local parameters;
- type declarations;
- interface declarations;
- port declarations;
- signal declarations;
- net declarations;
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
- timing intent;
- nested hardware declarations where permitted.

There MUST be no fixed number of members.

---

14. Module Identity

A module name is a semantic identifier.

The module MUST NOT implicitly identify:

- a vendor;
- a physical FPGA;
- an ASIC;
- a chip;
- a package;
- a board;
- a memory bank;
- a physical location.

If target identity is required, it MUST be represented through explicit target/deployment mechanisms outside portable HDL semantics.

---

15. Module Composition

Modules MUST be composable.

A module MAY instantiate another module.

Conceptually:

module system {
    instance compute of ComputeUnit(...);
    instance memory of MemoryUnit(...);
}

Instance count MUST NOT be bounded by the language.

Generate constructs MUST allow scalable structural generation.

---

16. Module Instances

An instance represents a semantic occurrence of a module.

Instance identity is distinct from physical resource identity.

Therefore:

instance compute

does not mean:

physical FPGA #0

or:

physical accelerator #3

Placement is downstream.

---

17. Ports

Ports define externally visible module interfaces.

The existing "ports.g4" and "hardware-interfaces.g4" MUST remain integrated into the canonical HDL grammar.

Ports MAY have:

- direction;
- type;
- width;
- dimensions;
- attributes;
- protocol semantics;
- timing constraints;
- capability requirements.

Directions MAY include:

input
output
inout

Additional directions MAY be introduced only through a specified semantic extension.

There MUST be no fixed number of ports.

---

18. Port Types

A port type describes semantic data or signaling behavior.

Examples include:

logic
bit
bool
integer
unsigned
signed
custom_type

Parameterized forms MAY describe widths or dimensions.

For example:

data : logic[width]

where "width" is a semantic parameter.

The language MUST NOT silently convert the parameter into a compiler-global maximum.

---

19. Signals

Signals represent named semantic communication/state relationships within hardware.

Conceptually:

signal value : logic;

Signals MAY have:

- type;
- initial value where semantically supported;
- attributes;
- timing properties;
- driving rules;
- resolution semantics.

A signal MUST NOT implicitly represent a particular physical wire.

---

20. Nets

Nets represent hardware connectivity where multiple-driver or resolved connectivity semantics are required.

The language MUST distinguish a semantic net from a physical routing resource.

For example:

net bus;

does not identify:

- routing track;
- FPGA switch;
- metal layer;
- physical wire;
- package connection.

Those belong to target realization.

---

21. Resolution Semantics

Where multiple drivers are permitted, the semantic model MUST define:

- whether multiple drivers are legal;
- how conflicts are represented;
- whether a resolution function exists;
- whether unresolved contention is an error;
- whether high impedance is meaningful;
- whether pull-up/pull-down behavior is modeled.

The parser only recognizes the syntax.

Semantic analysis determines whether the construction is legal.

---

22. Registers

Registers represent stateful storage updated according to defined clock/reset semantics.

Conceptually:

register state : logic;

A register MAY specify:

- type;
- initialization;
- clock;
- reset;
- enable;
- edge;
- attributes.

Register width MUST be parameterizable.

The language MUST NOT impose a universal register width.

---

23. Memories

The existing:

grammar/hdl/memories.g4

MUST be retained and integrated into the complete HDL specification.

Memory declarations MAY express:

- element type;
- dimensions;
- depth;
- width;
- number of dimensions;
- read behavior;
- write behavior;
- synchronous/asynchronous behavior;
- latency intent;
- throughput intent;
- initialization;
- persistence intent;
- attributes.

Memory dimensions MUST be semantic.

A declaration such as:

memory data : logic[width][depth];

MAY use symbolic "width" and "depth".

No universal maximum depth or width may be imposed by the grammar.

---

24. Memory Realization

Source-level memory does not imply:

- SRAM;
- DRAM;
- BRAM;
- URAM;
- register file;
- cache;
- external memory;
- HBM;
- distributed RAM.

The compiler MAY select a suitable realization.

The selected realization MUST satisfy semantic requirements.

---

25. Clocks

The existing "clocks.g4" MUST remain the HDL clock syntax component.

A clock is a semantic timing source.

A clock declaration MAY describe:

- identity;
- frequency;
- period;
- duty cycle;
- phase;
- relationship to another clock;
- attributes;
- constraints.

The source MAY specify a frequency as a semantic requirement.

It MUST NOT implicitly mean:

use clock pin X

or:

use PLL 3

unless explicit target-dependent syntax is used.

---

26. Clock Frequency

Clock frequency MUST be represented as a semantic quantity.

For example:

frequency = 1GHz

is valid as a program requirement.

It does not mean that every target can satisfy it.

A target unable to provide the requested timing MUST result in a diagnostic.

The implementation MUST NOT silently change:

1GHz

to:

500MHz

while claiming semantic equivalence if the frequency is observable or contractually required.

---

27. Clock Domains

A clock domain is a semantic region associated with a timing source.

A design MAY contain arbitrarily many clock domains.

There MUST be no grammar-level maximum.

Cross-domain communication MUST be explicitly analyzable.

Semantic analysis MUST detect, where applicable:

- unsafe crossings;
- missing synchronization;
- incompatible timing assumptions;
- metastability-sensitive boundaries;
- illegal clock-domain assumptions.

---

28. Clock-Domain Crossing

The language MAY provide explicit constructs or attributes for CDC intent.

Examples of semantic categories include:

- synchronizer;
- handshake;
- asynchronous FIFO;
- pulse synchronization;
- domain bridge.

A CDC declaration MUST describe semantic intent.

It MUST NOT hard-code a particular FPGA primitive unless explicitly target-specific.

---

29. Resets

Reset semantics MUST distinguish:

- synchronous reset;
- asynchronous reset;
- active-high;
- active-low;
- reset sequencing;
- reset dependencies;
- reset domains.

The existing reset facilities in "hdl.g4" MUST remain compatible with this semantic model.

Reset syntax MUST NOT imply a particular vendor primitive.

---

30. Combinational Logic

The existing "combinational.g4" MUST define syntax for combinational hardware behavior.

A combinational process MUST have no implicit persistent state unless explicitly introduced.

The semantic analyzer MUST detect:

- unintended incomplete assignment;
- illegal state retention;
- combinational cycles where prohibited;
- invalid multiple drivers;
- type/width mismatch.

The compiler MAY implement combinational behavior using:

- gates;
- LUTs;
- ASIC logic;
- programmable fabric;
- custom hardware;
- other equivalent realizations.

---

31. Sequential Logic

Sequential behavior MUST explicitly or inferably establish its state and timing relationship.

A sequential process MAY depend on:

- clock;
- edge;
- reset;
- enable;
- state;
- inputs.

The semantic model MUST distinguish sequential state from ordinary software variables.

---

32. Procedural Hardware

The existing:

processes.g4

MUST support procedural descriptions of hardware behavior.

Procedural syntax MUST NOT automatically imply software execution.

For example, a procedural block describing combinational behavior represents hardware semantics, not necessarily a runtime loop.

The compiler determines the realization.

---

33. "always", "process", Combinational, Sequential

The existing HDL grammar supports constructs including:

process
always
combinational
sequential

These MUST have clearly distinct semantic contracts.

The specification MUST NOT allow two keywords to have accidentally identical or contradictory meanings.

Each construct MUST define:

- sensitivity semantics;
- execution semantics;
- state semantics;
- assignment rules;
- timing semantics;
- legality conditions;
- AST representation;
- semantic representation.

---

34. Blocking and Non-Blocking Assignment

If Zamani exposes multiple hardware assignment modes, their semantic differences MUST be explicitly specified.

The language MUST distinguish:

- immediate procedural update;
- clocked/state update;
- continuous assignment.

Syntax alone MUST NOT determine a backend-specific implementation.

---

35. State Machines

HDL MUST support explicit state-machine semantics.

A state machine MAY contain:

- arbitrary states;
- transitions;
- guards;
- actions;
- entry behavior;
- exit behavior;
- reset state;
- default behavior.

There MUST be no fixed maximum number of states or transitions.

State identifiers are semantic names.

They MUST NOT correspond to physical flip-flop positions.

---

36. State-Machine Correctness

Semantic validation MUST detect where applicable:

- undefined states;
- invalid transitions;
- unreachable states;
- impossible guards;
- conflicting transitions;
- missing reset behavior;
- incomplete behavior;
- nondeterministic transitions where determinism is required.

Verification MAY additionally identify:

- dead states;
- unreachable states;
- dead transitions;
- invariant violations.

---

37. Pipelines

The existing "pipelines.g4" MUST remain the pipeline syntax component.

A pipeline expresses staged computation.

A pipeline MAY contain:

- arbitrary stages;
- stage-local state;
- input/output relationships;
- latency;
- throughput;
- dependencies;
- buffering;
- valid/ready semantics;
- clock relationships.

The number of stages MUST NOT be language-limited.

---

38. Pipeline Semantics

Pipeline latency and throughput MUST be distinguished.

For example:

latency = L
throughput = T

are different semantic properties.

A compiler MAY transform a pipeline if the requested observable behavior remains valid.

If latency is explicitly semantic, it MUST be preserved or an explicit contract violation MUST be reported.

---

39. Pipeline Optimization

The compiler MAY:

- add pipeline stages;
- remove redundant stages;
- rebalance stages;
- retime;
- replicate computation;
- share resources;
- transform memory structures.

However, the compiler MUST respect:

- latency contracts;
- throughput contracts;
- timing contracts;
- resource constraints;
- observable ordering;
- verification properties.

---

40. Hardware Interfaces

Interfaces represent reusable communication contracts.

The existing:

hardware-interfaces.g4

MUST remain integrated.

An interface MAY contain:

- ports;
- signals;
- parameters;
- types;
- protocol semantics;
- timing requirements;
- ordering requirements;
- capability requirements.

An interface MUST NOT inherently identify a physical bus.

---

41. Protocols

Protocol syntax MAY describe:

- request/response;
- valid/ready;
- handshake;
- streaming;
- packetized transfer;
- memory-mapped intent;
- message-oriented communication;
- custom protocol contracts.

Protocol definitions MUST describe semantic behavior.

Vendor-specific protocols SHOULD be implemented as dialects or interoperability contracts rather than contaminating universal HDL syntax.

---

42. Connectivity

Hardware connectivity MUST be represented independently of physical routing.

For example:

connect producer.output -> consumer.input

describes semantic connectivity.

It MUST NOT automatically mean:

route through switch 17

or:

use physical wire 4

Physical routing belongs downstream.

---

43. Generate Constructs

HDL MUST support parameterized structural generation.

A generate construct MAY create:

- module instances;
- signals;
- memories;
- logic;
- pipelines;
- state-machine components;
- interfaces.

Generated cardinality MAY depend on parameters.

There MUST be no universal fixed maximum.

Generate expansion MUST be deterministic for identical inputs and semantic configuration unless explicit nondeterminism is declared.

---

44. Generate Safety

Generate constructs MUST NOT permit:

- uncontrolled recursive expansion;
- infinite compile-time expansion;
- hidden compiler escape hatches;
- arbitrary filesystem access;
- arbitrary network access;
- arbitrary process execution.

Compile-time computation MUST be governed by the language's metaprogramming and compile-time evaluation rules.

---

45. Hardware Generics

Hardware generics MUST support parameterized hardware definitions.

Generic constraints MAY include:

requires width > 0
requires depth > 0
requires capability("memory")

A constraint describes validity.

It does not identify the implementation.

Generic specialization MUST occur after parsing and semantic validation.

---

46. Hardware Parameters

Parameters MAY define:

- dimensions;
- timing;
- widths;
- counts;
- topology;
- capacities;
- protocol options;
- algorithmic choices.

Parameters MUST be typed where the type system requires it.

The parameter system MUST integrate with:

- generic types;
- compile-time evaluation;
- constant evaluation;
- resource analysis;
- capability analysis;
- hardware synthesis.

---

47. Type and Width Semantics

Hardware width is a semantic property when it affects observable behavior.

For example:

logic[width]

MUST preserve the specified width semantics.

The compiler MAY use:

- packed bits;
- machine words;
- vectors;
- registers;
- memories;
- custom hardware.

But representation MUST preserve semantics.

Width arithmetic MUST be validated semantically.

Examples of invalid constructs include:

- negative width;
- zero width where prohibited;
- incompatible assignment width;
- impossible concatenation;
- invalid slice;
- out-of-range static index where statically provable.

---

48. Signedness

Signed and unsigned types MUST have distinct semantic behavior.

Signedness MUST affect:

- comparison;
- arithmetic;
- extension;
- truncation;
- conversion.

The compiler MUST NOT infer signedness from the target's preferred representation.

---

49. Bit-Level Operations

HDL MUST integrate with Zamani's common expression system for:

- bitwise AND;
- OR;
- XOR;
- NOT;
- shifts;
- concatenation where defined;
- slicing;
- replication where defined.

The HDL grammar SHOULD reuse universal operators rather than create duplicate operator vocabularies.

---

50. Hardware Literals

Where HDL requires domain-specific literals, they MUST be explicitly specified.

Potential categories include:

- bit vectors;
- logic vectors;
- high impedance;
- unknown/X values where simulation semantics require them;
- timing quantities;
- frequency;
- phase;
- physical units;
- protocol values.

Simulation-only values MUST NOT silently become synthesis semantics.

---

51. Four-State Logic

If Zamani supports four-state HDL simulation values:

0
1
X
Z

their meanings MUST be explicitly defined.

"X" and "Z" MUST NOT automatically imply physical hardware states in every target.

The specification MUST distinguish:

- simulation unknown;
- simulation contention;
- high impedance;
- synthesizable behavior.

---

52. Simulation Semantics

Simulation syntax MAY describe:

- stimulus;
- timing;
- assertions;
- waveforms;
- verification;
- testbench behavior;
- monitors.

Simulation constructs MUST be explicitly identified as:

- synthesizable;
- simulation-only;
- verification-only;
- target-dependent.

A simulation-only construct MUST NOT accidentally enter a synthesis path as hardware.

---

53. Synthesis Intent

HDL MAY express synthesis intent.

Examples include semantic categories such as:

- pipeline;
- resource sharing;
- parallelism;
- latency;
- throughput;
- memory preference;
- implementation preference.

Synthesis intent is not itself synthesis.

The synthesis engine owns actual transformation.

---

54. Optimization Intent

Optimization hints MAY suggest:

prefer latency
prefer throughput
prefer area
prefer energy
prefer resource sharing
prefer parallelism

These MUST be preferences unless explicitly declared as hard constraints.

A preference MUST NOT make an otherwise semantically valid program invalid merely because a compiler cannot satisfy the preference.

---

55. Resource Requirements

HDL MAY declare genuine resource requirements.

Examples:

requires memory >= required_capacity
requires throughput >= required_rate
requires capability("parallel.compute")

Resource requirements MUST remain abstract.

The source MUST NOT encode today's FPGA resource count as a language maximum.

---

56. Capability Requirements

Hardware programs MAY require capabilities.

Examples:

requires capability("reconfigurable.logic")
requires capability("high_bandwidth_memory")
requires capability("parallel.compute")
requires capability("hardware.multiply")

Capability names are semantic identifiers.

Capability databases belong outside the grammar.

---

57. Requirements, Constraints, Preferences, Hints

These concepts MUST remain distinct.

Requirement

Must be satisfied.

Constraint

Restricts valid realization.

Preference

Optimizes a valid realization.

Hint

Provides optional guidance.

Implementation decision

Is selected by the compiler/toolchain.

For example:

requires throughput >= T
prefer memory("high_bandwidth")

does not mean:

use specific memory device 3

---

58. Physical Constraints

Physical constraints MAY exist in target-specific contexts.

Examples include:

- pin assignments;
- package constraints;
- voltage constraints;
- placement regions;
- clock regions;
- physical interfaces.

However, these MUST be explicitly identified as target-dependent.

Portable HDL MUST NOT accidentally acquire physical dependencies through ordinary declarations.

---

59. Target Profiles

Target profiles belong to the compilation/hardware system.

A target profile MAY describe:

- capabilities;
- resource availability;
- timing capabilities;
- memory;
- interconnect;
- device features;
- supported protocols;
- supported synthesis constructs.

Target profiles MUST NOT redefine the source-language syntax.

---

60. Target Realization

The conceptual flow is:

portable HDL intent
        │
        ▼
semantic hardware model
        │
        ▼
resource/capability analysis
        │
        ▼
optimization
        │
        ▼
scheduling
        │
        ▼
synthesis
        │
        ▼
placement
        │
        ▼
routing
        │
        ▼
target lowering
        │
        ▼
actual hardware

The ordering MAY be optimized internally, but the semantic ownership boundaries MUST remain.

---

61. Hardware/Software Co-Design

HDL MUST integrate with ordinary Zamani software.

A hardware module MAY be associated with:

- software functions;
- accelerator calls;
- shared data;
- memory;
- communication;
- asynchronous execution;
- synchronization;
- capabilities;
- resource requirements.

A software program MUST NOT need to be rewritten solely because an accelerator is:

- absent;
- small;
- large;
- FPGA-based;
- ASIC-based;
- CPU-based;
- GPU-based;
- another supported realization.

Where semantics permit multiple realizations, the compiler may select among them.

---

62. Accelerator Semantics

An accelerator is a realization of a semantic computation.

The source MAY express:

accelerator function

but MUST NOT assume that the accelerator is physically implemented by:

- FPGA;
- GPU;
- ASIC;
- DSP;
- tensor unit;
- dedicated chip.

The implementation selects the realization.

---

63. CPU/GPU/FPGA/QPU Integration

HDL MUST be composable with:

- CPU computation;
- GPU computation;
- FPGA computation;
- QPU computation;
- other accelerators.

A heterogeneous system MUST remain one semantic program.

For example:

classical algorithm
        ↓
hardware accelerator
        ↓
quantum operation
        ↓
measurement
        ↓
classical decision

MUST be expressible without creating separate incompatible semantic universes.

---

64. Quantum Hardware Integration

HDL MAY describe quantum-control hardware where the repository provides such semantics.

However:

HDL MUST NOT create a second quantum semantic IR.

Quantum semantics MUST eventually integrate with the existing:

quantum::ir

boundary.

The conceptual flow remains:

Zamani source
      ↓
domain-neutral AST
      ↓
semantic analysis
      ↓
quantum::ir
      ↓
optimization
      ↓
routing
      ↓
scheduling
      ↓
QEC / resilience / ZQN
      ↓
HAL
      ↓
quantum hardware realization

HDL may describe supporting classical/control hardware without taking ownership of QEC, ZQN, routing, or HAL semantics.

---

65. Clock and Quantum Integration

When quantum control hardware interacts with classical control hardware, the language MUST distinguish:

- logical quantum timing;
- physical pulse timing;
- classical clock timing;
- hardware synchronization.

Physical pulse realization belongs downstream.

A source-level quantum or hybrid construct MUST NOT require a particular physical qubit or pulse generator unless explicitly target-specific.

---

66. Distributed Hardware

HDL MAY participate in distributed hardware descriptions.

Examples include:

- multiple processing elements;
- network-connected accelerators;
- distributed memory;
- chiplets;
- multi-device systems;
- reconfigurable systems.

The number of components MUST NOT be language-limited.

Topology may be semantic intent.

Physical topology realization remains downstream.

---

67. Chiplet and Multi-Die Systems

HDL MAY describe semantic modules corresponding to:

- chiplets;
- dies;
- interposers;
- package-level interfaces.

These MUST remain abstract unless physical implementation is explicitly requested.

A module named "chiplet" MUST NOT automatically identify a physical die.

---

68. Reconfigurable Hardware

HDL MAY express reconfiguration intent.

A reconfigurable region MAY have:

- interface contracts;
- supported configurations;
- resource requirements;
- timing constraints;
- lifecycle semantics.

The language MUST NOT require a particular FPGA vendor's reconfiguration mechanism.

---

69. Dynamic Hardware

Where supported, dynamic hardware configuration MAY be represented semantically.

The distinction MUST be maintained between:

program-defined configuration

and:

implementation-defined reconfiguration mechanism

The compiler/runtime owns the latter.

---

70. Hardware Timing

Timing is semantic only when explicitly specified or when required by the construct's defined hardware semantics.

Timing constructs MAY describe:

- latency;
- period;
- frequency;
- setup;
- hold;
- throughput;
- phase;
- ordering;
- synchronization.

The language MUST distinguish hard timing requirements from optimization preferences.

---

71. Timing Failure

If a target cannot satisfy a hard timing requirement, the implementation MUST report an explicit failure.

It MUST NOT silently:

- reduce frequency;
- increase latency;
- lower throughput;
- change clock relationships;
- alter protocol behavior.

unless the program explicitly permits such adaptation.

---

72. Timing Closure

Timing closure is NOT a grammar responsibility.

It belongs to the implementation pipeline.

The grammar can express:

requires timing <= T

but does not implement timing analysis.

---

73. Verification Properties

HDL MUST support semantic verification intent.

Possible constructs include:

assert
assume
cover

The existing HDL grammar already includes assertion constructs and these MUST remain integrated.

Verification properties MUST be associated with:

- source locations;
- semantic scope;
- clocking context where relevant;
- assumptions;
- guarantees;
- observable properties.

---

74. Assertions

An assertion expresses a condition that must hold according to its declared semantic context.

The compiler MAY use assertions for:

- simulation;
- formal verification;
- synthesis-time checking;
- runtime checking where meaningful.

The assertion's semantic role MUST be explicit.

---

75. Assumptions

An assumption describes an environmental property relied upon by verification.

An assumption MUST NOT silently become a hardware requirement unless its specification explicitly states that behavior.

---

76. Coverage

Coverage constructs MAY describe properties or behaviors that should be observed during verification.

Coverage MUST NOT alter functional hardware semantics.

---

77. Formal Verification

The language MAY expose formal verification intent.

The compiler MAY lower verification properties to:

- SAT;
- SMT;
- model checking;
- symbolic simulation;
- equivalence checking;
- other verification systems.

The grammar MUST NOT depend on a particular verification engine.

---

78. Hardware Contracts

Hardware modules SHOULD support contracts describing:

- legal inputs;
- legal outputs;
- invariants;
- timing properties;
- resource assumptions;
- protocol guarantees.

Contracts MUST integrate with semantic analysis and verification.

---

79. Hardware Effects

HDL constructs have domain-specific effects.

Examples include:

- state mutation;
- signal drive;
- memory mutation;
- clock dependence;
- external I/O;
- communication;
- hardware interaction.

Effects MUST integrate with Zamani's universal effect system where applicable.

---

80. Hardware Ownership and Resource Semantics

Where hardware resources are represented as language-level values or objects, ownership/resource semantics MUST remain explicit.

A hardware resource MUST NOT be duplicated in a way that violates its semantic ownership contract.

The compiler MAY realize ownership through:

- static analysis;
- synthesis;
- resource sharing;
- replication;
- arbitration.

---

81. Concurrency

Hardware is inherently concurrent in many contexts.

HDL MUST integrate with Zamani concurrency semantics.

Independent hardware processes MAY execute concurrently.

Concurrency MUST NOT be translated into an arbitrary sequential software interpretation unless the semantic model permits it.

The compiler MAY lower concurrency to:

- parallel logic;
- pipelines;
- time-multiplexing;
- scheduling;
- software emulation.

The selected realization MUST preserve semantics.

---

82. Determinism

Hardware semantics MUST distinguish:

- deterministic combinational behavior;
- deterministic sequential behavior;
- explicitly nondeterministic behavior;
- simulation unknowns;
- externally dependent behavior.

Identical deterministic source programs under identical semantic conditions MUST produce equivalent observable behavior.

---

83. Hardware Cycles

The semantic analyzer MUST distinguish:

- valid sequential feedback;
- valid combinational feedback where explicitly supported;
- invalid combinational cycles;
- clocked cycles.

The parser MUST NOT attempt to determine physical legality.

---

84. Recursion

Recursive hardware generation MAY be supported only when compile-time expansion is well-defined and terminates.

Recursive runtime software calls embedded in a hardware description MUST not automatically mean recursive physical hardware.

The semantic system MUST distinguish:

recursive computation

from:

recursive structural generation

---

85. Hardware Functions

Functions MAY be used to abstract hardware behavior where the semantic model permits.

A hardware function MUST clearly distinguish:

- combinational function;
- sequential function;
- compile-time function;
- simulation function;
- software function;
- hardware/software boundary function.

A function MUST NOT accidentally change execution domain.

---

86. Domain Boundary

The AST MUST remain domain-neutral.

The preferred operation representation remains conceptually:

Operation {
    name,
    namespace,
    operands,
    parameters,
    results,
    attributes,
    modifiers,
    effects,
    capabilities,
    source
}

HDL-specific semantics MUST be attached through semantic analysis rather than forcing the universal AST to become a giant HDL enum.

---

87. AST Integration

Every HDL grammar rule MUST have a predetermined AST mapping.

The mapping MUST be established before the grammar rule is considered complete.

Conceptually:

hdlModuleDeclaration
        ↓
domain-neutral Module/Declaration node
        ↓
semantic HardwareModule
        ↓
hardware semantic IR

Similarly:

hdlPortDeclaration
        ↓
declaration/port AST
        ↓
semantic HardwarePort
        ↓
hardware IR port

The grammar MUST NOT invent frontend-only semantic models that bypass the common AST.

---

88. Semantic Integration

Every HDL construct MUST have a semantic contract covering:

- name resolution;
- type checking;
- width checking;
- signedness;
- ownership;
- effect checking;
- clock-domain checking;
- timing;
- connectivity;
- driver legality;
- resource requirements;
- capability requirements;
- verification properties;
- target independence.

---

89. IR Integration

HDL syntax MUST lower into the repository's canonical semantic hardware/domain representation.

The exact IR implementation MUST be determined by the existing compiler architecture.

This document MUST NOT create a competing IR solely for grammar convenience.

The mapping MUST preserve:

- module identity;
- port identity;
- types;
- widths;
- connectivity;
- state;
- timing;
- protocols;
- resource intent;
- capability requirements;
- verification intent.

---

90. Synthesis Boundary

Synthesis begins after semantic validation.

The grammar MUST NOT contain synthesis algorithms.

The synthesis subsystem owns transformations such as:

- Boolean optimization;
- resource sharing;
- logic minimization;
- retiming;
- technology mapping;
- memory inference;
- operator inference.

---

91. Placement Boundary

Placement is target realization.

The source MUST NOT require a physical location unless using explicit target-specific constructs.

Portable HDL SHOULD remain location-independent.

---

92. Routing Boundary

Routing is target realization.

The source-level:

connect A -> B

describes logical connectivity.

The routing system determines:

physical path

subject to target constraints.

---

93. Scheduling Boundary

Hardware scheduling MAY be needed for:

- resource sharing;
- time multiplexing;
- dynamic execution;
- accelerator orchestration;
- hybrid systems.

Scheduling is not owned by HDL syntax.

HDL MAY specify timing requirements.

Scheduling determines a legal realization.

---

94. Hardware Resource Allocation

Resource allocation belongs to compiler/synthesis/target systems.

The language MAY express:

requires capability(...)
requires resource(...)

but MUST NOT contain universal assumptions about available:

- LUTs;
- DSPs;
- BRAM;
- URAM;
- registers;
- routing tracks;
- clock buffers;
- I/O pins.

---

95. Vendor Independence

Core Zamani HDL MUST NOT require vendor-specific syntax for ordinary hardware concepts.

Vendor-specific functionality MAY be exposed through:

dialects/
interoperability/
target-specific profiles

A vendor dialect MUST explicitly declare:

- vendor;
- dialect name;
- version;
- syntax additions;
- semantic additions;
- target requirements;
- compatibility;
- lowering rules.

---

96. Existing "hardware-dialects.g4"

The existing:

grammar/hdl/hardware-dialects.g4

MUST remain an extension mechanism rather than becoming a second universal HDL language.

Dialect syntax MUST:

- use canonical tokens;
- reuse universal expressions/types;
- declare explicit ownership;
- identify semantic extensions;
- remain isolated from core semantics;
- have compatibility metadata.

---

97. Interoperability With Existing HDLs

Zamani SHOULD support interoperability with established HDL ecosystems where required.

Potential formats include:

- Verilog;
- SystemVerilog;
- VHDL;
- vendor HDL;
- netlists;
- intermediate hardware representations.

These are interoperability formats.

They MUST NOT become the canonical Zamani semantic model.

---

98. Importing External HDL

Imported HDL MUST pass through a controlled interoperability boundary.

Conceptually:

external HDL
     ↓
import frontend
     ↓
domain-neutral representation
     ↓
semantic validation
     ↓
Zamani hardware semantics

The imported construct MUST NOT bypass:

- type checking;
- source provenance;
- semantic validation;
- capability analysis;
- security rules.

---

99. Exporting HDL

Zamani MAY export to established HDL formats.

Export MUST be a lowering operation.

Export MUST NOT redefine Zamani source semantics.

If the target format cannot represent a required semantic property, the compiler MUST report an explicit diagnostic rather than silently dropping the property.

---

100. Hardware Dialect Safety

A dialect MUST NOT:

- introduce hidden global state;
- bypass semantic validation;
- access arbitrary files;
- access arbitrary networks;
- execute arbitrary processes;
- mutate compiler state without declared semantics;
- bypass security boundaries;
- introduce unsafe Rust into the Zamani implementation.

The Rust implementation MUST remain compatible with Rust 1.97 / 1.97.1 and MUST NOT use "unsafe".

---

101. Security

Hardware descriptions can affect physical systems.

The implementation MUST therefore distinguish:

- source-level intent;
- compile-time actions;
- target access;
- physical deployment.

A grammar file MUST NOT grant physical access.

Security-sensitive operations MUST be controlled by:

- capability systems;
- compiler policies;
- deployment policies;
- runtime/HAL controls.

---

102. Source Provenance

Every HDL construct MUST preserve source provenance sufficient for:

- diagnostics;
- error reporting;
- IDE tooling;
- formatting;
- refactoring;
- verification;
- synthesis diagnostics;
- generated HDL traceability;
- debugging.

Generated hardware SHOULD be traceable back to originating source constructs.

---

103. Diagnostics

HDL diagnostics MUST be structured and source-aware.

Diagnostics SHOULD identify:

- severity;
- error code;
- source file;
- source span;
- construct;
- cause;
- relevant constraint;
- expected value/type;
- actual value/type;
- remediation;
- downstream impact where useful.

Examples include:

HDL001 invalid port direction
HDL002 incompatible signal type
HDL003 width mismatch
HDL004 invalid multiple driver
HDL005 unresolved clock domain crossing
HDL006 impossible timing requirement
HDL007 unsatisfied hardware capability
HDL008 unsatisfied resource requirement
HDL009 invalid state transition
HDL010 combinational cycle
HDL011 invalid memory dimension
HDL012 synthesis-only construct in simulation context
HDL013 simulation-only construct in synthesis context
HDL014 unsupported target realization

The exact numerical registry belongs to the repository's diagnostic specification.

---

104. Error Recovery

The parser SHOULD recover from local syntax errors sufficiently to report multiple diagnostics in tooling contexts.

Recovery MUST NOT create misleading semantic nodes that appear valid.

Malformed HDL MUST NOT silently become different valid hardware.

---

105. Static Validation

Before synthesis, semantic validation SHOULD check:

- syntax completeness;
- name resolution;
- type correctness;
- width correctness;
- signedness;
- connectivity;
- driver legality;
- clock domains;
- reset behavior;
- memory dimensions;
- state-machine validity;
- pipeline consistency;
- timing requirements;
- resource requirements;
- capability requirements;
- verification contracts.

---

106. Scalability

HDL must scale across:

single operation
    ↓
small module
    ↓
large accelerator
    ↓
multi-accelerator system
    ↓
heterogeneous system
    ↓
distributed hardware
    ↓
future computational architectures

The grammar MUST NOT require separate syntax merely because the hardware grows.

A parameterized module SHOULD remain the same semantic construct at different scales.

---

107. Structural Scaling

The language MUST support arbitrary semantic cardinality for:

- modules;
- instances;
- ports;
- signals;
- nets;
- registers;
- memories;
- states;
- transitions;
- pipeline stages;
- processes;
- interfaces;
- generated structures;
- clock domains.

The implementation MAY have practical limits.

Those limits MUST be explicit implementation diagnostics.

---

108. Compilation Scaling

Compilation SHOULD support incremental and compositional analysis where practical.

Large HDL programs SHOULD be decomposable by:

- module;
- package;
- interface;
- generic specialization;
- compilation unit.

A compiler MUST NOT require all physical target decisions to be encoded in every source file.

---

109. Resource Scaling

A source program MAY request more resources than a target possesses.

This is not automatically a syntax error.

The pipeline SHOULD distinguish:

syntactically invalid
semantically invalid
resource-unsatisfiable
capability-unsatisfiable
target-incompatible

This distinction is necessary for POCO-REAF.

---

110. Resource Failure

When resources are insufficient, the compiler MUST produce an explicit diagnostic.

It MUST NOT silently:

- truncate a memory;
- reduce a width;
- remove pipeline stages;
- reduce parallelism;
- drop modules;
- eliminate verification properties;
- change clock requirements;
- change protocol behavior.

Any semantic transformation that changes requirements MUST require explicit permission from the language's transformation/optimization semantics.

---

111. Optimization Freedom

The compiler MAY choose different realizations when semantic equivalence is proven.

Examples:

parallel logic
    ↔
time-multiplexed logic

or:

distributed computation
    ↔
single accelerator

where permitted.

Optimization MUST preserve all observable semantics and declared hard constraints.

---

112. Semantic Equivalence

Two hardware realizations are equivalent when they preserve the defined observable behavior.

Observables MAY include:

- outputs;
- state transitions;
- protocol behavior;
- timing contracts;
- resource contracts when explicitly semantic;
- error behavior;
- verification properties;
- external communication;
- explicitly exposed power/thermal constraints.

Internal implementation structure is not automatically observable.

---

113. Power and Thermal Intent

Where supported by the resource/hardware system, HDL MAY express:

- power budgets;
- thermal constraints;
- energy preferences;
- cooling requirements.

These MUST be represented as resource/constraint metadata rather than as fixed physical assumptions.

For example:

requires power <= P
prefer energy <= E

does not identify a particular cooling system.

---

114. Reliability

HDL MAY express reliability requirements.

Examples include:

- reliability target;
- fault tolerance;
- redundancy;
- availability;
- recovery capability.

The hardware compiler MAY realize these through:

- redundancy;
- replication;
- error detection;
- correction;
- checkpointing.

The source requirement remains semantic.

---

115. Fault Tolerance

Fault-tolerance intent MUST remain distinct from its implementation.

The HDL layer may express:

requires capability("fault.tolerant")

but MUST NOT take ownership of:

- ZQN fault semantics;
- QEC algorithms;
- resilience orchestration.

Those remain owned by their corresponding subsystems.

---

116. Mixed Hardware and Software State

When hardware and software share state, the semantic model MUST define:

- ownership;
- visibility;
- synchronization;
- consistency;
- ordering;
- memory semantics.

The compiler MUST NOT assume that CPU memory and hardware memory are automatically equivalent.

---

117. DMA and Data Movement

Data movement MAY be expressed semantically.

For example:

transfer data from source to destination

describes a computation/dataflow requirement.

It does not require:

- DMA engine 0;
- PCIe lane 3;
- physical memory bank 2.

Those are target realizations.

---

118. Streaming

HDL SHOULD support streaming semantics.

A stream MAY have:

- element type;
- rate;
- ordering;
- backpressure;
- buffering;
- timing;
- termination semantics.

The compiler determines whether the target uses:

- FIFO;
- register pipeline;
- network link;
- DMA;
- software queue;
- other realization.

---

119. Backpressure

Where streaming protocols expose backpressure, its semantics MUST be explicit.

The compiler MUST preserve:

- whether producers may stall;
- whether consumers may stall;
- ordering;
- losslessness;
- bounded/unbounded buffering semantics where declared.

---

120. Memory Consistency

If multiple hardware agents access shared state, the semantic model MUST define the required consistency/ordering behavior.

Possible semantics include:

- sequential consistency;
- relaxed ordering;
- explicit synchronization;
- atomic operations.

The hardware target may implement these differently.

---

121. Atomics

Hardware atomicity MUST be a semantic property.

The source MUST NOT assume that a target provides a particular atomic primitive unless expressed as a capability requirement.

---

122. Hardware Atomic Operations

Hardware atomic operations MAY include:

- compare-and-swap;
- atomic add;
- atomic exchange;
- locks;
- barriers.

These MUST be integrated with Zamani's common concurrency semantics where applicable.

---

123. Hardware/Network Boundaries

Network-connected hardware MAY expose interfaces through the networking domain.

The HDL layer owns the hardware-side interface semantics.

The networking subsystem owns network protocol semantics.

Neither subsystem should duplicate the other's IR.

---

124. Hardware/Data Integration

HDL MAY operate on:

- arrays;
- tensors;
- streams;
- records;
- structured data.

The data subsystem owns data semantics.

HDL owns how those semantic values participate in hardware computation.

---

125. AI/ML Accelerator Integration

HDL MAY describe hardware realization of AI/ML computation.

The AI subsystem owns:

- model semantics;
- training semantics;
- inference semantics;
- differentiability;
- model/data meaning.

HDL owns:

- accelerator structure;
- interfaces;
- pipelines;
- memory;
- timing;
- hardware realization intent.

The two domains MUST integrate through the common semantic model.

---

126. Scientific Computing Integration

Scientific computations MAY be mapped onto hardware accelerators.

HDL MUST NOT hard-code:

- floating-point width;
- vector width;
- tensor dimensions;
- accelerator count.

Those are parameterizable semantic or target properties.

---

127. Future Hardware

The HDL architecture MUST permit new hardware paradigms without requiring a redesign of the entire language.

Potential future realizations include:

- new accelerators;
- optical computing;
- neuromorphic hardware;
- analog computing;
- reversible computing;
- molecular/nano hardware;
- quantum control systems;
- photonic systems;
- emerging memory;
- future heterogeneous architectures.

New paradigms SHOULD be introduced through semantic capabilities and domain extensions rather than hard-coded machine assumptions.

---

128. Dialect Extension

A future hardware paradigm MAY be represented as a dialect when it requires syntax not appropriate for the core HDL.

A dialect MUST declare:

name
version
syntax extensions
semantic extensions
AST mapping
IR mapping
capabilities
target requirements
compatibility
diagnostics
tests

A dialect MUST NOT silently modify core HDL meaning.

---

129. Metaprogramming

Hardware metaprogramming MAY generate structures.

It MUST preserve:

- deterministic expansion;
- source provenance;
- semantic validation;
- resource safety;
- termination;
- security.

Generated HDL MUST be validated as if written directly where applicable.

---

130. Macros

Macros MAY abstract repetitive hardware syntax.

Macros MUST NOT:

- bypass semantic analysis;
- bypass type checking;
- introduce hidden physical assumptions;
- access arbitrary external resources;
- generate unbounded structures without explicit bounded semantics.

Macro expansion MUST preserve source provenance.

---

131. Compile-Time Hardware Computation

Compile-time functions MAY calculate:

- widths;
- dimensions;
- addresses where target-specific;
- pipeline counts;
- generated topology;
- parameter values.

Compile-time computation MUST remain distinct from runtime hardware execution.

---

132. Deterministic Generation

For identical:

- source;
- parameters;
- semantic configuration;
- dialect versions;
- dependency versions;

hardware generation SHOULD be deterministic.

Nondeterministic optimization MAY choose among equivalent realizations, but reproducibility metadata SHOULD record relevant decisions.

---

133. Reproducibility

Hardware builds SHOULD record:

- source version;
- compiler version;
- grammar version;
- dialect versions;
- parameter values;
- target profile;
- capability environment;
- optimization configuration;
- generated artifact provenance.

The language itself MUST NOT encode mutable machine-specific assumptions into portable source.

---

134. Versioning

HDL syntax MUST be versioned through the repository's compatibility system.

The following files remain relevant:

grammar/compatibility/versions.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md
grammar/spec/compatibility.md

An HDL syntax change MUST identify:

- affected grammar rules;
- affected tokens;
- AST impact;
- semantic impact;
- IR impact;
- compatibility impact;
- migration path;
- tests.

---

135. Backward Compatibility

Existing valid HDL syntax SHOULD remain valid unless there is a documented language-breaking change.

Deprecated syntax MUST have:

- deprecation status;
- replacement;
- version information;
- diagnostics;
- migration guidance.

---

136. Existing Grammar Integration

The current HDL grammar components MUST be treated as implementations of this contract.

At minimum, integration must cover:

hdl.g4
clocks.g4
combinational.g4
hardware-dialects.g4
hardware-generics.g4
hardware-interfaces.g4
hardware-modules.g4
hardware-parameters.g4
memories.g4
pipelines.g4
ports.g4
processes.g4
registers.g4

The exact decomposition may evolve, but the semantic ownership described in this specification MUST remain stable.

---

137. Lexer Integration

HDL MUST use the canonical Zamani lexical system.

There MUST NOT be an independent HDL lexer that creates incompatible tokens.

HDL keywords MUST be registered in the canonical keyword/token registry.

The implementation already uses "tokenVocab = ZamaniTokens" in "grammar/hdl/hdl.g4"; this pattern MUST remain the integration direction.

---

138. Keyword Discipline

The HDL domain MUST NOT create a keyword for every possible hardware operation.

For example, arbitrary operations should generally be represented through:

identifier
qualified name
generic operation
library/intrinsic
attribute
dialect

rather than expanding the global keyword vocabulary indefinitely.

Keywords SHOULD be reserved only for constructs with genuine language-level syntactic semantics.

---

139. Expression Reuse

HDL expressions MUST reuse the common Zamani expression system wherever possible.

Hardware-specific expressions MAY add:

- ranges;
- widths;
- bit selections;
- concatenation;
- replication;
- timing expressions.

They MUST integrate with the universal type and expression model.

---

140. Type Reuse

HDL types MUST integrate with "grammar/types/".

The HDL grammar MUST NOT independently redefine:

- identifiers;
- generics;
- function types;
- tuples;
- records;
- arrays;
- symbolic values.

Hardware-specific type semantics MAY extend the common type system.

---

141. Resource Integration

HDL resource requirements MUST integrate with:

grammar/resources/
grammar/hardware/
grammar/compile/
grammar/execution/

A hardware resource requirement is not itself a physical allocation.

For example:

requires memory >= M

means the realization must provide sufficient semantic memory capacity.

It does not mean:

allocate BRAM #4

---

142. Compile Integration

HDL compilation MUST integrate with:

grammar/compile/

Compilation intent MAY include:

- target independence;
- optimization;
- synthesis;
- simulation;
- verification;
- reproducibility;
- specialization;
- cross-compilation.

Compilation syntax MUST NOT become a replacement for hardware semantics.

---

143. Execution Integration

Runtime/execution semantics belong to:

grammar/execution/

HDL may define hardware behavior that executes continuously or in response to events.

Execution constructs MUST distinguish:

- elaboration;
- simulation;
- synthesis;
- deployment;
- runtime interaction.

---

144. Hardware Integration

Hardware capability and target descriptions belong to:

grammar/hardware/

HDL describes the hardware being requested/described.

Hardware profiles describe what a target provides.

These MUST NOT be conflated.

---

145. Hybrid Integration

HDL MUST integrate with:

grammar/hybrid/

for software/hardware and classical/quantum/hardware interactions.

Examples include:

software
    ↓
accelerator
    ↓
hardware
    ↓
measurement
    ↓
software

The boundary MUST be explicit in semantics.

---

146. Distributed Integration

HDL MAY participate in distributed computation.

The distributed subsystem owns:

- distributed semantics;
- node/process abstractions;
- communication semantics;
- consistency;
- replication.

HDL owns hardware endpoints and hardware interfaces.

---

147. Networking Integration

Networking owns:

- network protocols;
- endpoints;
- addressing;
- routing;
- network semantics.

HDL owns hardware-side network interface structure.

A physical MAC/PHY selection is target realization unless explicitly target-specific.

---

148. Security Integration

Security constraints MAY be expressed through:

- capabilities;
- authorization;
- isolation;
- trusted interfaces;
- secure computation;
- provenance.

Hardware security implementation belongs downstream.

---

149. Verification Integration

The grammar MUST support a traceable verification pipeline:

HDL syntax
   ↓
AST
   ↓
semantic property
   ↓
verification representation
   ↓
formal/simulation tool

Verification tools MUST be able to identify the originating source span.

---

150. Testing Requirements

Every HDL feature MUST have:

- positive tests;
- negative tests;
- boundary tests;
- scalability tests;
- compatibility tests;
- determinism tests;
- diagnostics tests;
- integration tests.

No feature is complete solely because ANTLR accepts its syntax.

---

151. Positive Tests

Positive tests MUST cover:

- smallest valid module;
- parameterized module;
- module composition;
- ports;
- signals;
- nets;
- registers;
- memories;
- clocks;
- resets;
- processes;
- combinational logic;
- sequential logic;
- state machines;
- pipelines;
- instances;
- generate;
- assertions;
- timing;
- interfaces.

---

152. Negative Tests

Negative tests MUST cover:

- invalid syntax;
- invalid names;
- invalid types;
- width mismatch;
- invalid dimensions;
- multiple illegal drivers;
- invalid clock reference;
- invalid reset;
- illegal CDC;
- invalid state transition;
- combinational cycle;
- impossible timing requirement;
- unsatisfied capability;
- unsatisfied resource requirement;
- illegal dialect construct;
- simulation-only construct in synthesis context;
- synthesis-only construct in simulation context.

---

153. Boundary Tests

Boundary tests MUST cover:

- smallest legal widths;
- large symbolic widths;
- empty/optional constructs where legal;
- large parameter values;
- large memory dimensions;
- many pipeline stages;
- many states;
- many instances;
- many ports;
- many modules;
- many clock domains.

The tests MUST NOT encode an artificial universal maximum.

---

154. Scalability Tests

Scalability tests MUST verify that the architecture does not accidentally introduce finite grammar limits.

Tests SHOULD progressively exercise:

1 module
→ many modules
→ parameterized modules
→ generated modules
→ large module graph

and:

1 port
→ many ports

1 stage
→ many stages

1 state
→ many states

1 instance
→ many instances

The test harness MAY impose practical test sizes, but those sizes MUST NOT become language semantics.

---

155. Determinism Tests

Given identical source and semantic configuration:

- parsing MUST be deterministic;
- AST construction MUST be deterministic;
- semantic diagnostics MUST be stable;
- generated semantic representation SHOULD be deterministic;
- source spans MUST be stable.

Compiler optimization MAY choose different equivalent implementations only where the language permits implementation freedom.

---

156. Compatibility Tests

Compatibility tests MUST compare:

grammar/spec/hdl.md
        ↕
grammar/hdl/*.g4
        ↕
grammar/Zamani.g4
        ↕
src/lexer.rs
        ↕
src/parser.rs
        ↕
src/frontend/ast/
        ↕
semantic implementation
        ↕
hardware IR
        ↕
compiler

A feature MUST NOT be marked stable if one of these contracts silently disagrees.

---

157. Hard-Coding Audit

Every HDL grammar file MUST pass a hard-coding audit.

The audit MUST reject universal architectural constants such as:

MAX_PORTS
MAX_WIDTH
MAX_MODULES
MAX_REGISTERS
MAX_MEMORIES
MAX_STAGES
MAX_STATES
MAX_INSTANCES
MAX_CLOCKS
MAX_DEVICES
MAX_LUTS
MAX_BRAMS
MAX_DSPS

The audit SHOULD also detect suspicious physical identities such as:

FPGA0
GPU0
QPU0
LUT0
BRAM0
DSP0
PIN0
CLOCK0

when used as universal source-language semantics.

A target-specific dialect MAY legitimately contain physical identifiers if the dialect explicitly declares itself target-dependent.

---

158. Distinguishing Program Numbers From Language Limits

This distinction is mandatory.

Valid:

const width = 1024;

Valid:

memory data : logic[width][depth];

Valid:

pipeline stages = n;

Invalid as universal language design:

MAX_WIDTH = 1024;
MAX_DEPTH = 65536;
MAX_STAGES = 32;

The first examples are program semantics.

The second examples impose artificial implementation limits.

---

159. No Fixed Physical Mapping

Portable HDL MUST NOT assume:

register 0
memory bank 0
LUT 0
DSP 0
BRAM 0
pin 0
clock region 0
routing channel 0

as universal identities.

Logical names MUST remain distinct from physical resource identities.

---

160. Hardware Topology

Topology MAY be expressed semantically when it matters.

For example:

requires topology(...)

or equivalent resource intent MAY constrain realization.

However, portable source SHOULD prefer semantic connectivity over physical coordinates.

The target system maps logical topology to physical topology.

---

161. Physical Addressing

Physical addresses MUST NOT be part of ordinary portable HDL semantics.

If memory-mapped I/O requires an address, it MUST be explicitly represented as target/deployment-specific intent.

The semantic distinction is:

resource identity

versus:

physical address

---

162. Physical Pinning

Physical pins belong to target realization.

Portable HDL SHOULD describe:

input clock

rather than:

pin 42

Target-specific deployment configuration MAY map the logical port to a physical pin.

---

163. Fabrication Technology

The core HDL language MUST NOT require a particular fabrication node.

Examples such as:

3nm
5nm
7nm
28nm

may be target constraints/preferences.

They are not universal HDL semantics.

---

164. Analog and Mixed-Signal Extension

If Zamani later supports analog or mixed-signal HDL, it MUST extend the existing semantic architecture.

It MUST NOT assume digital-only semantics for:

- continuous values;
- voltage;
- current;
- impedance;
- frequency;
- analog timing.

A future analog dialect/domain SHOULD reuse common module/interface/resource foundations.

---

165. Physical Units

Where timing, frequency, power, voltage, current, temperature, or other physical quantities are supported, unit semantics MUST be explicit.

The unit system MUST prevent accidental dimensionally invalid operations where the type system can detect them.

---

166. Simulation Versus Physical Semantics

The implementation MUST distinguish:

simulation approximation

from:

physical hardware behavior

A simulation feature MUST NOT automatically imply synthesizable hardware.

Likewise, synthesis semantics MUST NOT be assumed to reproduce every simulator-only behavior.

---

167. Testbench Integration

Testbenches MAY be represented through the verification/simulation system.

A testbench MUST NOT accidentally become part of synthesized hardware.

The semantic category MUST be explicit.

---

168. Assertions in Synthesis

Assertions MAY be:

- synthesis-visible;
- synthesis-checked;
- simulation-only;
- formal-only.

The specification of each assertion category MUST identify its intended phase.

---

169. Hardware Lifecycle

The hardware lifecycle MAY include:

parse
→ validate
→ elaborate
→ specialize
→ optimize
→ synthesize
→ verify
→ place
→ route
→ package
→ deploy
→ execute
→ observe

The grammar describes source intent.

The toolchain owns the lifecycle.

---

170. Elaboration

Elaboration resolves semantic structure such as:

- generic parameters;
- generated instances;
- type parameters;
- constant expressions.

Elaboration MUST occur before physical realization.

Elaboration MUST remain deterministic where the language requires deterministic behavior.

---

171. Specialization

Specialization MAY select concrete parameter values.

Specialization MUST preserve generic semantics.

Failure to specialize because the target cannot satisfy a requirement MUST produce an explicit diagnostic.

---

172. Hardware Reflection

If hardware reflection is supported, it MUST expose semantic information rather than arbitrary backend internals unless explicitly target-dependent.

Reflection MAY expose:

- module metadata;
- port metadata;
- type information;
- capabilities;
- resource requirements.

---

173. Hardware Introspection

Runtime introspection MAY query target capabilities where the execution model permits it.

This does not change the meaning of statically portable source.

Runtime adaptation MUST respect the program's semantic constraints.

---

174. Adaptive Realization

A program MAY permit adaptive implementation.

For example, the compiler/runtime MAY select among:

CPU
GPU
FPGA
ASIC
QPU
future accelerator

when all selected realizations satisfy the same semantic contract.

This is one of the mechanisms enabling POCO-REAF.

---

175. Hardware Availability

Hardware availability is an environment property.

The grammar MUST NOT encode the current availability of hardware.

The compiler/runtime/HAL discovers availability.

---

176. Capability Negotiation

Capability negotiation MAY occur between:

program requirements
        ↕
available target capabilities

Negotiation MUST preserve semantic requirements.

A preference may be negotiated.

A hard requirement may not be ignored.

---

177. Fallback

A program MAY specify explicit fallback semantics where meaningful.

For example:

preferred hardware realization
fallback software realization

Fallback MUST preserve defined semantics.

The compiler MUST NOT invent a fallback that changes behavior.

---

178. Graceful Failure

When no legal realization exists, the implementation MUST fail explicitly.

The error SHOULD identify:

- required capability;
- unavailable capability;
- required resource;
- available resource;
- target;
- affected source construct;
- possible alternative if known.

---

179. Hardware Error Semantics

Hardware errors MAY include:

- unavailable device;
- resource exhaustion;
- timing failure;
- communication failure;
- verification failure;
- deployment failure.

These MUST remain distinct from ordinary source syntax errors.

---

180. Runtime Errors

Runtime hardware errors belong to the runtime/execution model.

The HDL grammar MAY declare how a module participates in defined error handling.

It MUST NOT embed a runtime implementation.

---

181. Observability

HDL MAY expose semantic observability through:

- signals;
- events;
- traces;
- counters;
- performance metrics;
- verification hooks.

Profiling metadata MUST NOT alter functional semantics.

---

182. Performance Contracts

Performance constraints MAY be expressed.

Examples:

requires throughput >= T
requires latency <= L
prefer energy <= E

Hard performance requirements are constraints.

Performance preferences are optimization hints.

---

183. No Silent Performance Degradation

If a performance property is declared as a hard semantic requirement, a realization that fails it MUST NOT be presented as compliant.

---

184. Resource Sharing

The compiler MAY share hardware resources when semantics permit.

For example, two operations MAY use the same multiplier at different times.

Resource sharing MUST respect:

- concurrency;
- latency;
- throughput;
- state;
- timing;
- ordering.

---

185. Resource Replication

The compiler MAY replicate hardware to satisfy:

- throughput;
- latency;
- concurrency.

The source need not identify the exact number of physical units unless that count is semantically required.

---

186. Parallelism

HDL MAY express parallel semantic operations.

The implementation MAY realize them using:

- replicated logic;
- SIMD;
- pipeline;
- time multiplexing;
- distributed execution.

The language MUST NOT hard-code one realization.

---

187. Structural Versus Behavioral HDL

Zamani MAY support both:

Structural description

Describes composition/connectivity.

Behavioral description

Describes required behavior.

These forms MUST lower to the same semantic hardware model where their meanings are equivalent.

---

188. Behavioral Equivalence

Two descriptions MAY be equivalent even when one is:

behavioral

and the other is:

structural

The compiler/verification system MAY prove equivalence.

The source language MUST NOT require developers to manually rewrite between forms solely for target changes.

---

189. Hardware Abstraction Level

Zamani HDL MUST support multiple abstraction levels.

Conceptually:

algorithmic
    ↓
behavioral
    ↓
RTL-like
    ↓
structural
    ↓
target realization

The language MUST not require all programs to start at physical implementation level.

---

190. RTL Compatibility

Where RTL semantics are provided, they MUST be explicitly specified.

RTL constructs MUST retain:

- clock semantics;
- state semantics;
- combinational semantics;
- timing intent;
- interface semantics.

---

191. High-Level Synthesis

Zamani MAY support high-level hardware descriptions that compile into RTL or another target representation.

The programmer SHOULD be able to describe computation rather than manually instantiate every primitive.

---

192. Low-Level Escape Hatch

Target-specific low-level constructs MAY exist through explicit dialects/interoperability.

They MUST be marked as target-dependent.

They MUST NOT contaminate portable semantics.

---

193. Primitive Cells

Primitive hardware cells MAY be represented as target dialect entities.

Core Zamani HDL MUST NOT assume a fixed universal primitive library.

---

194. Vendor Libraries

Vendor libraries MAY be imported through interoperability mechanisms.

The import MUST declare:

- vendor;
- version;
- ABI/interface contract;
- target dependence;
- semantic assumptions.

---

195. No Vendor Lock-In in Core Syntax

The core language MUST remain meaningful without any particular:

- FPGA vendor;
- ASIC vendor;
- EDA tool;
- simulator;
- synthesis engine;
- board;
- device.

---

196. EDA Integration

Zamani MAY integrate with external EDA systems.

Such integration belongs to tooling/interoperability.

The grammar MUST remain independent of tool invocation syntax unless that syntax is explicitly a language-level build feature.

---

197. Generated Artifacts

Generated Verilog/SystemVerilog/VHDL/netlists/bitstreams/etc. are artifacts.

They are not source-language authority.

The source remains the semantic origin.

---

198. Artifact Provenance

Generated artifacts SHOULD retain:

- source hash;
- source version;
- compiler version;
- grammar version;
- target profile;
- parameters;
- dialects;
- optimization information.

---

199. Documentation Authority

"grammar/spec/hdl.md" is normative.

"grammar/hdl/README.md" is navigational and implementation-oriented.

"grammar/hdl/*.g4" are grammar implementation components.

"grammar/Zamani.g4" is the canonical ANTLR composition root.

"grammar/grammar.md" describes implementation conformance.

"grammar/Zamani-Grammar.md" may retain broader historical/aspirational material.

None of the latter documents may silently override this specification.

---

200. Feature Lifecycle

Every new HDL feature MUST progress through:

proposal
   ↓
syntax contract
   ↓
AST contract
   ↓
semantic contract
   ↓
IR contract
   ↓
compiler integration
   ↓
runtime/tooling integration
   ↓
positive tests
   ↓
negative tests
   ↓
boundary tests
   ↓
scalability tests
   ↓
compatibility tests
   ↓
documentation
   ↓
stable

A feature is not production-ready merely because its ".g4" rule parses.

---

201. Feature Manifest

Where feature manifests are used under:

grammar/specification/features/

every HDL feature SHOULD declare:

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
tooling_consumers
domain
capabilities
resource_requirements
positive_tests
negative_tests
boundary_tests
scalability_tests
compatibility
hard_coding_policy
diagnostics

This gives each feature a closed integration contract.

---

202. Per-File Completion Contract

Every HDL grammar/specification file MUST be independently completable.

Each file SHOULD explicitly establish:

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
Hard-Coding Audit
Diagnostics
Security
Performance
Completion Criteria

This is mandatory for production development discipline.

---

203. Completion Criteria for an HDL Grammar File

A file MUST NOT be marked complete until:

- its syntax is defined;
- its lexical dependencies are known;
- ambiguities are resolved;
- its AST mapping is defined;
- its semantic mapping is defined;
- its IR mapping is defined;
- downstream consumers are identified;
- diagnostics are defined;
- negative behavior is defined;
- boundary behavior is defined;
- scalability behavior is defined;
- compatibility behavior is defined;
- hard-coding audit passes;
- determinism requirements are defined;
- cross-domain integration is defined;
- tests exist.

This is specifically intended to prevent later re-editing merely because another subsystem is implemented.

---

204. Integration Matrix

HDL Concern| Grammar| AST| Semantics| IR| Compiler| Runtime/Target
Module| "hdl"| declaration/module| module identity| hardware module| elaboration| realization
Port| "ports"| port node| interface/type| IR port| lowering| physical interface
Signal| "hdl"| declaration| signal semantics| signal| synthesis| hardware
Net| "hdl"| declaration| connectivity| net| routing| physical route
Register| "registers"| state node| sequential state| state element| synthesis| storage
Memory| "memories"| memory node| memory semantics| memory| inference| memory resource
Clock| "clocks"| clock node| timing| timing model| timing analysis| clock resource
Reset| "clocks"/HDL| reset node| reset semantics| reset model| lowering| reset realization
Process| "processes"| process node| behavior| process/logic| synthesis| execution
Pipeline| "pipelines"| pipeline node| latency/throughput| pipeline| optimization| hardware
Instance| "hardware-modules"| instance| composition| module instance| elaboration| physical realization
Generate| hardware generics| generated structure| elaboration| expanded IR| specialization| hardware
Assertion| HDL| assertion node| verification property| verification IR| formal/sim| verifier
Timing| HDL| attribute/constraint| timing semantics| timing metadata| timing analysis| target
Capability| hardware/resources| metadata| capability requirement| requirement| target analysis| HAL/target
Resource| resources| metadata| resource requirement| resource metadata| allocation| target
Dialect| hardware-dialects| extension node| dialect semantics| dialect mapping| lowering| target

---

205. Integration With "grammar/specification/syntax.md"

The universal syntax specification already establishes HDL as a first-class domain and maps:

HDL
→ grammar/hdl/
→ grammar/spec/hdl.md
→ hardware nodes
→ HDL/hardware IR

This document provides the detailed HDL contract for that relationship. The universal syntax specification remains responsible for overall language composition.

---

206. Integration With "grammar/specification/domains.md"

The repository's domain specification defines HDL as describing hardware structure and behavior as semantic hardware intent. This document refines that principle into concrete HDL contracts.

The HDL domain MUST therefore remain consistent with the universal domain model.

---

207. Integration With "grammar/spec/semantics.md"

The repository's semantic specification establishes:

program
→ semantic analysis
→ canonical semantic model
→ classical/quantum/HDL semantics
→ appropriate IR
→ verification
→ optimization
→ routing/scheduling/resilience/QEC/ZQN
→ HAL
→ runtime

HDL MUST participate in that pipeline rather than establishing an independent compiler architecture.

---

208. Integration With "grammar/DESIGN.md"

The HDL specification depends on the repository-level principles of:

- one language;
- deterministic parsing;
- domain-neutral AST;
- target independence;
- resource/capability separation;
- no artificial hardware limits;
- canonical IR boundaries;
- explicit compatibility;
- source provenance.

The existing repository design explicitly treats HDL as hardware intent rather than physical implementation.

---

209. Integration With "grammar/Zamani.g4"

"grammar/Zamani.g4" MUST remain the canonical composition root.

It MUST compose HDL syntax.

It MUST NOT duplicate every HDL rule.

The intended architecture is:

Zamani.g4
    │
    ├── universal syntax
    │
    └── HDL dispatch
            │
            └── grammar/hdl/*

The exact ANTLR import/composition mechanism MUST follow the grammar implementation architecture adopted by the repository.

---

210. Integration With "src/lexer.rs"

The canonical lexer owns:

- identifiers;
- literals;
- operators;
- punctuation;
- HDL keywords;
- comments;
- source locations.

HDL grammar MUST consume the canonical tokens.

If a token is required by an HDL construct but does not exist in the lexer, that is a lexical conformance gap.

It MUST NOT be solved by creating an HDL-specific lexer.

---

211. Integration With "src/parser.rs"

The parser MUST construct domain-neutral AST nodes.

HDL parsing MUST NOT directly construct:

- synthesis objects;
- physical FPGA objects;
- netlists;
- vendor primitives;
- routing graphs;
- placement objects.

Those belong downstream.

---

212. Integration With "src/frontend/ast/"

The AST MUST remain domain-neutral.

Hardware-specific semantic information SHOULD be represented through:

- generic declarations;
- operations;
- attributes;
- types;
- source spans;
- structured metadata.

A hardware semantic model may be produced after AST validation.

---

213. Integration With Classical Computing

HDL and classical computing MUST be composable.

For example:

classical function
        ↓
hardware accelerator
        ↓
result

The source MUST NOT require duplicated algorithms merely because execution moves between classical and hardware domains.

---

214. Integration With Quantum Computing

HDL and quantum computing MUST be composable.

Quantum semantics remain owned by the quantum subsystem and ultimately "quantum::ir".

HDL MAY provide supporting:

- control logic;
- timing;
- interfaces;
- memory;
- pulse/control hardware intent.

It MUST NOT duplicate quantum semantic ownership.

---

215. Integration With Hybrid Computing

HDL MUST integrate with hybrid execution.

A hybrid program MAY contain:

classical
→ hardware
→ quantum
→ classical

or:

software
→ accelerator
→ hardware
→ software

The common semantic model must represent the boundaries.

---

216. Integration With AI

AI accelerator hardware MUST integrate with AI semantic constructs.

For example:

model
→ computation
→ accelerator realization

AI semantics remain in the AI domain.

HDL describes the hardware realization.

---

217. Integration With Data

Hardware data structures MUST integrate with the data domain.

A tensor or stream used by hardware MUST preserve its semantic type.

Hardware representation is downstream.

---

218. Integration With Networking

Hardware networking interfaces MUST integrate with networking semantics.

A hardware port may represent a network interface.

The network subsystem defines:

- endpoint semantics;
- protocol;
- addressing;
- communication.

HDL defines the hardware-side structure.

---

219. Integration With Security

Secure hardware constructs MUST integrate with the security domain.

Examples:

secure module
trusted interface
isolated resource
cryptographic accelerator

Security semantics MUST remain explicit.

---

220. Integration With Distributed Computing

Distributed hardware systems MUST integrate with the distributed subsystem.

HDL can describe:

processing element
interconnect
interface
memory
accelerator

Distributed semantics determine:

- placement;
- communication;
- consistency;
- replication.

---

221. Integration With Resource Management

Hardware resources MUST integrate with the repository's resource-management architecture.

The grammar describes requirements.

The compiler/runtime determines availability.

The target system determines realization.

---

222. Integration With HAL

The Hardware Abstraction Layer is downstream.

HDL MUST NOT directly instantiate HAL objects.

The semantic pipeline is:

HDL intent
   ↓
hardware semantic model
   ↓
compiler
   ↓
HAL
   ↓
target

---

223. Integration With Scheduling

HDL MAY express timing and concurrency constraints.

Scheduling determines an implementation schedule.

HDL MUST NOT duplicate the scheduler.

---

224. Integration With Routing

HDL expresses logical connectivity.

Routing determines physical connectivity.

The two must remain separate.

---

225. Integration With Optimization

HDL expresses semantics and constraints.

Optimization may transform the design.

Optimization MUST preserve semantics.

---

226. Integration With Verification

Every hardware semantic construct MUST remain traceable into verification where applicable.

Verification MUST be able to report failures back to source locations.

---

227. Rust Implementation Requirements

All Zamani-owned Rust implementation associated with this HDL specification MUST:

- compile under Rust 1.97 / Rust 1.97.1;
- use Rust 2021;
- avoid "unsafe";
- avoid unsafe FFI assumptions;
- avoid hard-coded machine limits;
- use typed representations;
- provide structured errors;
- preserve source spans;
- remain deterministic where required.

External tools may have their own implementation languages, but Zamani's Rust implementation MUST retain these requirements.

---

228. No "unsafe"

The HDL frontend, parser integration, semantic model, validation tooling, and related Zamani Rust code MUST NOT use:

unsafe

If an external library uses unsafe internally, Zamani-owned code MUST still expose a safe abstraction and MUST NOT require unsafe blocks in Zamani's implementation.

---

229. Performance

The grammar and semantic architecture SHOULD support:

- incremental parsing;
- modular compilation;
- reusable ASTs;
- cached semantic analysis;
- incremental elaboration;
- incremental synthesis where possible.

Performance optimizations MUST NOT alter semantics.

---

230. Memory Safety

All HDL frontend structures MUST use safe ownership and borrowing semantics.

Large generated hardware descriptions SHOULD use scalable data structures rather than recursively constructing unnecessarily deep call stacks.

---

231. Recursion Depth

The language MUST NOT impose an artificial semantic maximum on:

- module nesting;
- generate nesting;
- state-machine complexity;
- pipeline depth.

An implementation MAY protect itself against pathological input through resource policies.

Such protection MUST produce explicit diagnostics.

---

232. Parser Resource Limits

Parser resource limits MAY exist as operational safeguards.

For example, an implementation may stop parsing after exhausting available memory.

Such a limit MUST NOT be documented as:

Zamani HDL supports at most N modules.

Instead it is an implementation resource failure.

---

233. Security Resource Limits

Resource limits may be necessary to prevent denial-of-service attacks during:

- parsing;
- elaboration;
- macro expansion;
- generate expansion;
- compile-time evaluation.

These limits MUST be implementation/security policies, not language semantics.

---

234. Semantic Resource Exhaustion

If semantic analysis exhausts resources, the implementation MUST report:

resource exhaustion during semantic analysis

rather than pretending that the source language rejects the construct.

---

235. Compiler Resource Exhaustion

Likewise, synthesis/resource exhaustion MUST remain distinct from syntax failure.

This distinction is required for portable source.

---

236. Target Compatibility

A target is compatible when it can provide a realization satisfying all required semantics and constraints.

A target is incompatible when it cannot.

A target MAY be incompatible because of:

- insufficient resources;
- missing capabilities;
- timing;
- unsupported semantics;
- unsupported data types;
- unsupported protocols;
- unsupported verification requirements.

---

237. No Semantic Downgrade

The compiler MUST NOT silently downgrade:

required width
required latency
required throughput
required memory
required timing
required reliability
required protocol
required precision

unless the source explicitly allows approximation or negotiation.

---

238. Approximation

Approximate hardware may be supported through explicit semantics.

For example:

prefer approximation <= error_bound

must distinguish approximation from exact computation.

The compiler MUST NOT silently introduce approximation into exact semantics.

---

239. Floating-Point Semantics

If hardware descriptions use floating-point types, the type system MUST define:

- precision;
- rounding;
- overflow;
- underflow;
- NaN;
- infinity;
- exceptional behavior.

Hardware implementation may differ internally only where semantics remain equivalent.

---

240. Fixed-Point Semantics

Fixed-point types MAY specify:

- integer width;
- fractional width;
- signedness;
- rounding;
- saturation.

These are semantic properties when observable.

The grammar MUST NOT impose a universal fixed-point width.

---

241. Arbitrary Width

The HDL architecture MUST allow widths to be:

- literal;
- symbolic;
- generic;
- computed;
- target-selected where explicitly permitted.

This is essential for POCO-REAF.

---

242. Arbitrary Dimensions

The same principle applies to:

- memory dimensions;
- tensor dimensions;
- arrays;
- matrices;
- vectors;
- pipeline arrays.

The language MUST not hard-code fixed dimensions.

---

243. Hardware Arrays

Hardware arrays MAY represent repeated semantic structures.

Examples include:

array<Module, n>

or equivalent grammar forms.

Array cardinality is semantic.

Physical replication is downstream.

---

244. Hardware Graphs

Large hardware systems may be naturally represented as graphs.

The semantic model MAY represent:

- nodes;
- edges;
- ports;
- resources;
- dependencies.

The graph size MUST NOT be grammar-limited.

---

245. Topology Generation

Topology may be generated from parameters.

Examples:

mesh(size)
tree(branching)
ring(count)
custom topology

These describe logical topology.

Physical realization remains target-dependent.

---

246. Communication Semantics

Communication constructs MUST define:

- source;
- destination;
- data;
- ordering;
- reliability;
- buffering;
- synchronization.

Physical interconnect selection is downstream.

---

247. Protocol Correctness

The compiler/verification system SHOULD be capable of checking:

- valid/ready behavior;
- handshake completion;
- deadlock conditions;
- protocol violations;
- ordering violations.

---

248. Deadlock

Where concurrent HDL semantics can deadlock, semantic analysis SHOULD identify statically provable deadlocks.

The grammar itself does not perform deadlock analysis.

---

249. Liveness

Verification MAY express liveness properties.

Liveness properties MUST be distinct from safety assertions.

---

250. Safety

Safety properties MAY express conditions that must never occur.

They MUST integrate with formal verification semantics.

---

251. Hardware Contracts and POCO-REAF

A portable hardware module should describe:

WHAT

rather than:

WHERE

For example:

module MatrixMultiply<rows, cols> {
    ...
}

is portable.

A physical mapping such as:

place module on FPGA region X

is target-specific.

This separation is central to POCO-REAF.

---

252. Complete HDL Pipeline

The final architecture is:

                     Zamani Source
                           │
                           ▼
                   grammar/Zamani.g4
                           │
                           ▼
                         Lexer
                           │
                           ▼
                         Parser
                           │
                           ▼
                  Domain-Neutral AST
                           │
                           ▼
                  Structural Validation
                           │
                           ▼
                 Semantic Analysis
                           │
          ┌────────────────┼────────────────┐
          │                │                │
          ▼                ▼                ▼
        Types          Resources       Capabilities
          │                │                │
          └────────────────┼────────────────┘
                           │
                           ▼
                  Hardware Semantics
                           │
                           ▼
                    Hardware IR
                           │
          ┌────────────────┼────────────────┐
          │                │                │
          ▼                ▼                ▼
      Verification     Optimization      Analysis
          │                │                │
          └────────────────┼────────────────┘
                           │
                           ▼
                    Scheduling
                           │
                           ▼
                      Synthesis
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
                           ▼
                FPGA / ASIC / CPU / GPU /
              Accelerator / Future Target

For quantum-connected systems:

HDL / hybrid semantics
          │
          ▼
    quantum::ir
          │
          ▼
 optimization
          │
 routing / scheduling
          │
 QEC / resilience / ZQN
          │
 HAL
          │
 quantum hardware

HDL MUST NOT create a competing quantum IR.

---

253. Production Invariants

The following invariants are mandatory.

Invariant 1 — One Language

HDL is part of Zamani.

Invariant 2 — One Canonical Root

"grammar/Zamani.g4" remains the canonical ANTLR composition root.

Invariant 3 — One Lexer

HDL uses the canonical Zamani lexer/token vocabulary.

Invariant 4 — Domain-Neutral AST

HDL syntax does not force a hardware-only universal AST.

Invariant 5 — Semantic Separation

Syntax, semantics, resources, capabilities, and physical realization remain separate.

Invariant 6 — No Artificial Limits

No universal hardware-size constants exist in the grammar.

Invariant 7 — Parameterization

Hardware quantities can remain symbolic.

Invariant 8 — Target Independence

Portable HDL does not select physical hardware implicitly.

Invariant 9 — Explicit Constraints

Hard requirements are distinguished from preferences and hints.

Invariant 10 — Explicit Failure

Unsatisfied resources/capabilities produce diagnostics.

Invariant 11 — Semantic Preservation

Optimization and lowering preserve declared semantics.

Invariant 12 — Quantum Boundary

Quantum semantics integrate through existing "quantum::ir".

Invariant 13 — Safe Rust

Zamani-owned Rust implementation uses Rust 1.97/1.97.1 and no "unsafe".

Invariant 14 — Provenance

HDL constructs remain traceable to source.

Invariant 15 — Determinism

Parsing and semantic analysis are deterministic where the language requires determinism.

---

254. Definition of Production Ready

"grammar/spec/hdl.md" and the HDL grammar subsystem are production-ready only when all of the following are true:

✓ HDL syntax is normatively defined
✓ HDL semantic meaning is defined
✓ lexer ownership is defined
✓ parser ownership is defined
✓ AST mappings are defined
✓ semantic mappings are defined
✓ IR mappings are defined
✓ compiler integration is defined
✓ runtime/target integration is defined
✓ resource semantics are defined
✓ capability semantics are defined
✓ timing semantics are defined
✓ clock semantics are defined
✓ reset semantics are defined
✓ memory semantics are defined
✓ pipeline semantics are defined
✓ state-machine semantics are defined
✓ interface semantics are defined
✓ protocol semantics are defined
✓ verification semantics are defined
✓ simulation/synthesis boundaries are defined
✓ physical realization boundaries are defined
✓ dialect boundaries are defined
✓ interoperability boundaries are defined
✓ quantum integration is defined
✓ hybrid integration is defined
✓ classical integration is defined
✓ distributed integration is defined
✓ AI/data integration is defined
✓ networking integration is defined
✓ security integration is defined
✓ scalability policy is defined
✓ hard-coding policy is defined
✓ compatibility policy is defined
✓ diagnostics are defined
✓ provenance is defined
✓ deterministic behavior is defined
✓ positive tests exist
✓ negative tests exist
✓ boundary tests exist
✓ scalability tests exist
✓ compatibility tests exist
✓ determinism tests exist
✓ hard-coding audit passes
✓ Rust 1.97/1.97.1 compatibility is verified
✓ Zamani-owned Rust contains no unsafe

---

255. Final Architectural Rule

The most important rule of Zamani HDL is:

«HDL describes portable computational and hardware intent; the compiler and target system determine how that intent becomes physical hardware.»

Therefore:

WHAT

belongs in Zamani source.

WHICH RESOURCE

belongs in capability/resource analysis.

HOW

belongs in optimization/synthesis.

WHERE

belongs in placement/routing/deployment.

WHEN

belongs in timing/scheduling when timing is semantically relevant.

WHICH DEVICE

belongs in target realization.

WHICH PHYSICAL QUBIT

belongs downstream in quantum routing/HAL.

WHICH QEC STRATEGY

belongs to QEC/resilience.

WHICH FAULT/NOISE MODEL

belongs to ZQN.

This separation is what permits one Zamani source program to scale from a tiny hardware description to a very large heterogeneous system without turning today's hardware limits into tomorrow's language limits.

---

256. Final Integration Contract

This file is complete as the normative HDL specification when the implementation conforms to the following contract:

grammar/spec/hdl.md
        │
        ├── grammar/hdl/README.md
        │
        ├── grammar/hdl/hdl.g4
        ├── grammar/hdl/clocks.g4
        ├── grammar/hdl/combinational.g4
        ├── grammar/hdl/hardware-dialects.g4
        ├── grammar/hdl/hardware-generics.g4
        ├── grammar/hdl/hardware-interfaces.g4
        ├── grammar/hdl/hardware-modules.g4
        ├── grammar/hdl/hardware-parameters.g4
        ├── grammar/hdl/memories.g4
        ├── grammar/hdl/pipelines.g4
        ├── grammar/hdl/ports.g4
        ├── grammar/hdl/processes.g4
        └── grammar/hdl/registers.g4
                │
                ▼
        grammar/Zamani.g4
                │
                ▼
          canonical lexer
                │
                ▼
          canonical parser
                │
                ▼
        domain-neutral AST
                │
                ▼
       semantic hardware model
                │
                ├── resource/capability analysis
                ├── timing/clock analysis
                ├── verification
                └── type/width analysis
                │
                ▼
          hardware/domain IR
                │
                ▼
      optimization / synthesis
                │
                ▼
       scheduling / placement
                │
                ▼
             routing
                │
                ▼
        target realization

No downstream file may require this specification to be retroactively rewritten merely to determine where a feature belongs. Every HDL feature must instead enter through a complete syntax → AST → semantics → IR → compiler → target → test contract.

This is the contract that makes the existing "grammar/hdl/" directory a production HDL subsystem rather than a collection of independent grammar fragments.