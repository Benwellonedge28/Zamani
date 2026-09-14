Zamani HDL Grammar

Path: "grammar/hdl/README.md"
Domain: Hardware Description Language / Hardware-Software Co-Design
Language: Zamani
Grammar baseline: ANTLR-compatible grammar integrated with the canonical Zamani language
Compiler baseline: Rust 1.97 / Rust 1.97.1
Safety: Safe Rust only; "unsafe" is prohibited
Primary architectural objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: From the smallest supported hardware design to arbitrarily large designs, subject only to explicitly represented semantics, compiler/resource policy, target capabilities, and available resources.

---

1. Purpose

The "grammar/hdl/" directory defines the Zamani source-language syntax for hardware description and hardware/software co-design.

It allows Zamani programs to express hardware intent including:

- hardware modules;
- interfaces;
- ports;
- signals;
- nets;
- registers;
- memories;
- clocks;
- resets;
- timing relationships;
- combinational logic;
- sequential logic;
- processes;
- state machines;
- pipelines;
- instances;
- generated hardware;
- hardware parameters;
- hardware generics;
- hardware constraints;
- hardware assertions;
- target-independent hardware capabilities;
- hardware/software boundaries.

The HDL grammar is part of the Zamani language.

It is not a separate programming language.

It must therefore share the canonical Zamani:

- lexical model;
- identifiers;
- literals;
- expressions;
- types;
- declarations;
- attributes;
- modules;
- diagnostics;
- versioning;
- capability model;
- resource model;
- semantic model.

The HDL grammar describes hardware meaning and intent.

It must not encode accidental limitations of today's:

- FPGA;
- ASIC;
- CPU;
- GPU;
- accelerator;
- fabrication process;
- package;
- board;
- clock generator;
- routing fabric;
- memory technology;
- vendor;
- synthesis tool;
- deployment environment.

---

2. Architectural Position

The HDL frontend participates in the following pipeline:

Zamani source
     |
     v
Canonical lexer
     |
     v
Canonical parser
     |
     v
HDL syntax
     |
     v
Frontend AST
     |
     v
Name resolution
     |
     v
Type analysis
     |
     v
Hardware semantic analysis
     |
     +----------------------+
     |                      |
     v                      v
Capability analysis     Resource analysis
     |                      |
     +----------+-----------+
                |
                v
Canonical hardware semantic representation
                |
                +--> optimization
                |
                +--> synthesis
                |
                +--> scheduling
                |
                +--> placement
                |
                +--> routing
                |
                +--> target lowering
                |
                v
        FPGA / ASIC / CPU / GPU /
        accelerator / simulator /
        emulator / future target

The grammar is therefore upstream of implementation.

It must not depend on downstream implementation details.

---

3. Ownership

3.1 This directory owns

"grammar/hdl/" owns the syntax of Zamani HDL constructs.

This includes:

- HDL module declarations;
- HDL interface declarations;
- port syntax;
- signal syntax;
- net syntax;
- register declarations;
- memory declarations;
- clock declarations;
- reset declarations;
- process syntax;
- sensitivity syntax;
- combinational blocks;
- sequential blocks;
- state-machine syntax;
- pipeline syntax;
- instance syntax;
- generate syntax;
- hardware parameters;
- hardware generics;
- hardware interfaces;
- HDL timing syntax;
- HDL assertions;
- HDL-specific attributes;
- HDL-specific modifiers;
- HDL composition syntax;
- hardware-domain syntactic constraints.

3.2 This directory does not own

The HDL grammar does not own:

- tokenization;
- lexical token definitions;
- general identifier syntax;
- general expression semantics;
- general type semantics;
- name resolution;
- type checking;
- constant evaluation;
- ownership;
- borrowing;
- lifetime analysis;
- resource discovery;
- hardware discovery;
- device discovery;
- target selection;
- synthesis;
- placement;
- routing;
- timing closure;
- physical design;
- FPGA bitstream generation;
- ASIC layout;
- fabrication;
- runtime execution;
- hardware calibration;
- quantum IR;
- quantum error correction;
- quantum noise semantics;
- ZQN;
- resilience;
- scheduling algorithms;
- optimization algorithms;
- backend-specific instruction encodings.

Those responsibilities belong to other repository layers.

---

4. Non-Negotiable Architectural Boundary

The HDL grammar must preserve:

syntax != semantics != implementation

More precisely:

HDL syntax
    !=
HDL semantic model
    !=
hardware resource model
    !=
target capability model
    !=
physical implementation

For example:

module accelerator {
    ...
}

does not mean:

use FPGA X

and:

memory data ...

does not mean:

allocate exactly N physical BRAM blocks

and:

clock ...

does not mean:

use physical clock pin X

unless such information is explicitly part of the program's semantic contract.

---

5. POCO-REAF

The HDL grammar must support:

Program Once
        |
Compile Once
        |
Run Everywhere
        |
Run Anywhere
        |
Run Forever

POCO-REAF means that a Zamani HDL design describes its intended computation and hardware behavior without requiring source rewriting merely because the implementation target changes.

The same source-level design should be capable of being considered for:

- simulation;
- emulation;
- FPGA synthesis;
- ASIC synthesis;
- hardware acceleration;
- heterogeneous systems;
- embedded systems;
- reconfigurable hardware;
- future hardware technologies.

This does not mean every target can realize every design.

Target limitations are expressed by:

- capability analysis;
- resource analysis;
- constraints;
- compilation policy;
- target descriptions;
- synthesis feasibility;
- runtime/deployment policy.

They must not become arbitrary grammar-level restrictions.

---

6. Scalability Contract

The grammar must support arbitrary cardinality wherever the language semantics permit it.

There must be no grammar-defined maximum for:

- modules;
- interfaces;
- ports;
- signals;
- nets;
- registers;
- memories;
- dimensions;
- widths;
- states;
- transitions;
- processes;
- clocks;
- resets;
- pipeline stages;
- instances;
- generated instances;
- hierarchy depth;
- hardware blocks;
- channels;
- connections;
- parameters;
- generic parameters.

The grammar must never introduce concepts such as:

MAX_PORTS
MAX_SIGNALS
MAX_REGISTERS
MAX_MEMORY
MAX_STAGES
MAX_MODULES
MAX_WIDTH
MAX_STATES

or equivalent hidden restrictions.

ANTLR cardinality operators such as:

*
+
?

represent grammar cardinality.

They are not machine-resource limits.

---

7. Resource Independence

A hardware design may express resource requirements.

However, resource requirements must remain separate from syntax.

For example:

requires memory capacity >= expression

is conceptually different from:

use physical memory block 3

The first expresses a requirement.

The second expresses a target-specific implementation decision.

The HDL grammar must preserve this distinction.

Resource semantics belong to the shared resource/capability infrastructure under "grammar/resources/" and the corresponding compiler/runtime systems.

---

8. Hardware Independence

The HDL grammar must not assume:

- a fixed FPGA family;
- a fixed ASIC technology;
- a fixed transistor process;
- a fixed LUT architecture;
- a fixed DSP architecture;
- a fixed BRAM architecture;
- a fixed register-file architecture;
- a fixed clock tree;
- a fixed routing topology;
- a fixed bus width;
- a fixed address width;
- a fixed physical pin count;
- a fixed package;
- a fixed board;
- a fixed vendor.

Such information belongs to target descriptions and hardware capabilities.

---

9. Current HDL Directory Contract

The HDL directory is decomposed by responsibility.

The intended ownership is:

grammar/hdl/
├── README.md
├── hdl.g4
├── hardware-modules.g4
├── ports.g4
├── signals.g4
├── wires.g4
├── registers.g4
├── clocks.g4
├── timing.g4
├── combinational.g4
├── sequential.g4
├── processes.g4
├── state-machines.g4
├── memories.g4
├── pipelines.g4
├── hardware-generics.g4
├── hardware-parameters.g4
├── hardware-interfaces.g4
└── hardware-dialects.g4

Each file must own one coherent HDL responsibility.

No file should silently redefine concepts owned by another HDL grammar component.

---

10. "hdl.g4"

Purpose

"hdl.g4" is the HDL grammar aggregation and entry-point grammar.

Owns

- HDL grammar entry point;
- composition of HDL declarations;
- HDL member dispatch;
- shared HDL syntactic composition;
- integration of HDL subgrammars.

Does not own

Detailed definitions that belong to:

- ports;
- clocks;
- memories;
- pipelines;
- state machines;
- interfaces;
- parameters;
- dialects.

Those belong to their dedicated files.

Integration

"hdl.g4" integrates with:

- canonical Zamani lexer;
- canonical parser;
- common core grammar;
- common expressions;
- common types;
- declarations;
- modules;
- hardware grammar;
- resources;
- capabilities;
- execution;
- compiler frontend.

Important current issue

The current HDL grammar declares:

tokenVocab = ZamaniTokens;

while the repository's ANTLR directory currently contains "ZamaniLexer.g4" and "ZamaniParser.g4".

This must be resolved as an ANTLR architecture issue, not silently ignored.

The final architecture must have exactly one authoritative lexer token vocabulary.

If "ZamaniTokens" is a generated/derived token vocabulary, its generation and ownership must be documented.

If "ZamaniLexer" is authoritative, HDL parser grammars must be migrated consistently to that vocabulary.

No HDL grammar file may independently invent a second token universe.

---

11. Lexer Integration

HDL grammar files are parser grammars.

They must not contain a second lexer.

HDL keywords must be recognized by the canonical Zamani lexical layer.

Examples of HDL lexical concepts include:

module
interface
input
output
inout
signal
wire
net
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
localparam
signed
unsigned
posedge
negedge
synchronous
asynchronous
assert
assume
cover

The exact canonical spelling must be defined by the shared lexical authority.

The HDL parser must consume those tokens rather than redefine them.

---

12. Shared Syntax Dependencies

HDL grammar must reuse shared Zamani constructs for:

- identifiers;
- qualified names;
- literals;
- expressions;
- ranges;
- type expressions;
- generic arguments;
- attributes;
- annotations;
- constraints;
- requirements;
- capabilities.

HDL must not create incompatible duplicate forms.

For example, there must not be:

general identifier
HDL identifier
quantum identifier
hardware identifier

unless semantic restrictions explicitly require distinct categories.

---

13. HDL Modules

Hardware modules are semantic units of hardware composition.

A module may contain:

- parameters;
- generic parameters;
- ports;
- interfaces;
- signals;
- nets;
- registers;
- memories;
- clocks;
- resets;
- processes;
- state machines;
- pipelines;
- instances;
- generate constructs;
- assertions;
- timing declarations.

A module does not imply a particular physical implementation.

---

14. Ports

Ports define module boundaries.

The grammar must support:

- input;
- output;
- bidirectional/inout;
- typed ports;
- parameterized widths;
- structured types;
- interface associations;
- attributes;
- constraints.

A port width must be capable of being expressed using symbolic or generic expressions where the language permits.

Do not impose:

width <= 32

or:

width <= 64

at grammar level.

---

15. Signals and Nets

Signals represent hardware communication and state/data flow.

The grammar must distinguish syntactic categories where the hardware semantic model requires them, including:

- logical signals;
- wires;
- nets;
- resolved nets;
- tri-state forms where supported;
- driven signals;
- connected signals.

The semantic layer determines:

- driver legality;
- multiple-driver legality;
- resolution;
- direction compatibility;
- width compatibility;
- clock-domain correctness.

The grammar does not perform those checks.

---

16. Registers

Registers represent sequential state.

The grammar must support:

- typed registers;
- initialization where semantically allowed;
- clock association;
- reset association;
- enable conditions;
- attributes;
- parameterized widths;
- generic state types.

The grammar must not decide whether the target realizes a register using:

- flip-flops;
- block RAM;
- distributed RAM;
- latches;
- custom storage;
- another technology.

That is backend responsibility.

---

17. Memories

Memory declarations must support symbolic and parameterized dimensions.

The grammar must not define fixed:

MAX_MEMORY_DEPTH
MAX_MEMORY_WIDTH

or equivalent limitations.

Memory semantics must be separated from physical implementation.

For example:

memory data : word_t [depth];

describes semantic memory structure.

It does not inherently mean:

BRAM
SRAM
DRAM
HBM
LUTRAM

The backend determines realization.

---

18. Clocks

The grammar may describe semantic clock relationships.

It must distinguish:

clock identity
clock relationship
clock frequency requirement
clock phase relationship
clock duty requirement
clock-domain relationship

from:

physical oscillator
physical pin
PLL
clock tree
vendor primitive

The latter belong to target-specific implementation.

---

19. Resets

The grammar may represent:

- synchronous reset;
- asynchronous reset;
- active-high reset;
- active-low reset;
- reset relationships.

Semantic analysis must determine legality.

Physical reset distribution remains outside grammar ownership.

---

20. Timing

Timing syntax must represent semantic timing requirements.

Examples include:

- latency;
- throughput;
- frequency;
- period;
- phase;
- ordering;
- setup/hold-related semantic constraints where applicable;
- pipeline relationships.

Timing values must use the canonical duration/unit syntax.

No fixed timing grid may be assumed.

For example, the grammar must not assume:

1 ns
10 ns
100 ns

as a universal timing universe.

A target may support one timing regime while another target supports another.

---

21. Combinational Logic

Combinational constructs describe logic whose outputs are determined by current inputs and referenced state-free values.

The grammar must support compositional expressions and procedural forms where defined.

Semantic analysis must detect:

- incomplete assignments;
- unintended storage;
- invalid dependencies;
- combinational cycles;
- unsupported constructs.

The grammar must not encode those semantic judgments.

---

22. Sequential Logic

Sequential constructs represent state-changing behavior.

The grammar must permit:

- clocked processes;
- reset behavior;
- enable behavior;
- state updates;
- sequential assignments;
- parameterized state.

Semantic analysis determines:

- clock correctness;
- reset consistency;
- conflicting assignments;
- illegal sensitivity;
- data hazards;
- unsupported behavior.

---

23. Processes

Processes provide behavioral HDL descriptions.

The grammar must support:

- process declarations;
- sensitivity lists;
- blocks;
- statements;
- assignments;
- conditionals;
- loops where legal;
- local declarations;
- assertions.

The grammar must not assume that one process equals one physical hardware block.

Synthesis and semantic lowering determine implementation.

---

24. State Machines

State-machine syntax must support arbitrary:

- states;
- transitions;
- transition conditions;
- outputs;
- state variables;
- reset states;
- parameterized state representations.

No fixed number of states may be imposed.

The grammar must not assume:

STATE0
STATE1
STATE2

as the only valid state model.

The semantic representation must remain extensible.

---

25. Pipelines

Pipeline syntax must support arbitrary stage counts.

The grammar must not impose:

MAX_STAGES

or a fixed pipeline depth.

Pipeline descriptions should express:

- stage identity;
- stage behavior;
- stage dependencies;
- latency;
- throughput;
- data flow;
- valid/ready relationships;
- optional timing constraints.

Physical pipeline insertion remains an optimization/synthesis decision unless explicitly specified by source semantics.

---

26. Hardware Generics

Generics provide reusable hardware descriptions.

They should permit parameterization of:

- widths;
- dimensions;
- types;
- interfaces;
- architecture choices;
- algorithmic structure;
- memory organization;
- pipeline structure.

Generics must not be limited to today's hardware.

A generic hardware component should be able to specialize to:

- small embedded implementations;
- large FPGA implementations;
- ASIC implementations;
- accelerator implementations;
- future targets.

---

27. Hardware Parameters

Parameters represent compile-time or elaboration-time values.

They must be distinguishable from:

- runtime values;
- resource requirements;
- target properties;
- capabilities;
- physical constants.

A parameter such as:

WIDTH

does not imply a fixed maximum width.

It is a symbolic value whose actual specialization is determined later.

---

28. Hardware Interfaces

Interfaces define reusable hardware communication contracts.

They must support composition without requiring a particular:

- bus vendor;
- FPGA vendor;
- physical pinout;
- physical protocol implementation.

Where protocol semantics are standardized or intentionally domain-specific, they should be represented through dialects or interface definitions rather than hard-coded into the core HDL grammar.

---

29. Hardware Dialects

"hardware-dialects.g4" provides controlled extensibility.

Dialects must be:

- explicitly named;
- versionable;
- namespaced;
- capability-aware;
- semantically registered;
- compatibility-aware.

A vendor-specific feature must not silently become a universal Zamani language construct.

Vendor extensions must remain distinguishable from core Zamani semantics.

---

30. Hardware/Software Co-Design

HDL syntax must integrate with ordinary Zamani computation.

A Zamani program may conceptually contain:

classical computation
        |
        v
hardware invocation
        |
        v
accelerated hardware
        |
        v
classical result

The HDL grammar must therefore remain compatible with:

- functions;
- modules;
- types;
- effects;
- memory;
- concurrency;
- resources;
- execution;
- interoperability.

The hardware boundary must be represented semantically rather than by creating a second programming language.

---

31. Quantum Integration

HDL may participate in hybrid quantum-classical-hardware systems.

However:

HDL grammar
    !=
quantum grammar

and:

HDL grammar
    !=
quantum::ir

Quantum syntax is owned by the quantum grammar.

Quantum semantic representation is owned by the canonical "quantum::ir".

HDL may describe hardware interfaces or accelerators used by quantum systems, but it must not redefine:

- qubits;
- quantum gates;
- quantum circuits;
- quantum state;
- QEC;
- ZQN.

Integration occurs through shared semantic and hardware interfaces.

---

32. Resource Integration

HDL resource requirements integrate with:

grammar/resources/

and downstream resource-management infrastructure.

The following concepts remain distinct:

resource
requirement
constraint
capability
preference
hint
target
placement

For example:

requires high-throughput memory

is not equivalent to:

use physical memory block X

---

33. Scheduling Integration

The HDL grammar does not schedule hardware.

Scheduling belongs to the repository scheduling subsystem.

HDL may provide semantic information such as:

- timing requirements;
- ordering constraints;
- pipeline relationships;
- latency requirements;
- synchronization semantics.

The scheduler decides how to realize those requirements.

The grammar must not embed scheduler algorithms.

---

34. Optimization Integration

Optimization consumes semantic HDL representation.

The grammar does not own:

- logic minimization;
- retiming;
- resource sharing;
- dead logic elimination;
- pipeline optimization;
- technology mapping;
- gate minimization.

These are compiler/backend responsibilities.

Any optimization must preserve source semantics.

---

35. Hardware Integration

Hardware infrastructure owns:

- hardware discovery;
- device capabilities;
- topology;
- target descriptions;
- calibration;
- physical resources;
- deployment.

The HDL grammar must not depend on discovered hardware.

This is essential to POCO-REAF.

---

36. Runtime Integration

Runtime infrastructure may consume compiled hardware artifacts and execution metadata.

The HDL grammar itself must never:

- execute hardware;
- discover devices;
- access files;
- access networks;
- invoke processes;
- inspect the host machine;
- perform calibration;
- select devices.

Grammar parsing must remain deterministic and side-effect free.

---

37. AST Contract

Every HDL construct must have a stable structural representation in the frontend AST.

The AST should preserve, as applicable:

- source spans;
- names;
- qualified names;
- generic parameters;
- parameters;
- types;
- expressions;
- attributes;
- declarations;
- hierarchy;
- timing information;
- process structure;
- state-machine structure;
- pipeline structure;
- interface structure.

The AST must preserve source meaning without prematurely converting the design into physical hardware.

---

38. Semantic Contract

Semantic analysis must validate:

- declaration legality;
- name resolution;
- type compatibility;
- width compatibility;
- direction compatibility;
- driver rules;
- assignment legality;
- process semantics;
- clock relationships;
- reset semantics;
- state-machine consistency;
- pipeline consistency;
- parameter validity;
- generic specialization;
- capability requirements;
- resource requirements.

The grammar must not duplicate these semantic checks.

---

39. Canonical IR Contract

The HDL frontend must eventually lower to a canonical hardware semantic representation owned outside the grammar directory.

The grammar must never become an IR.

The representation must be capable of preserving:

- hardware structure;
- data flow;
- control flow;
- state;
- timing semantics;
- interface semantics;
- resource requirements;
- capability requirements;
- hierarchy;
- provenance.

Downstream passes may then transform this representation.

---

40. Provenance

Every HDL semantic object produced by frontend processing should be traceable to source locations.

Diagnostics and compiler transformations should be able to answer:

Which source construct created this hardware object?

Source spans must therefore survive:

source
 -> AST
 -> semantic model
 -> IR
 -> lowered representation

where practical.

---

41. Diagnostics

HDL diagnostics must be:

- deterministic;
- structured;
- source-located;
- actionable;
- stable enough for tooling;
- independent of vendor implementation;
- independent of machine size.

Examples include:

undefined signal
duplicate declaration
invalid port direction
incompatible widths
invalid clock relationship
invalid reset relationship
multiple conflicting drivers
invalid state transition
invalid pipeline dependency
invalid parameter
invalid generic argument
unsupported semantic feature
missing capability
insufficient target resources

The last two must not be confused.

A syntactically valid design may be rejected later because a target cannot realize it.

---

42. Error Boundary

The grammar must distinguish:

syntax error

from:

semantic error

from:

capability error

from:

resource error

from:

target implementation error

from:

runtime error

This distinction is essential for POCO-REAF.

---

43. Determinism

Parsing the same source with the same grammar version must produce the same parse structure.

The HDL grammar must contain no:

- randomness;
- environment-dependent parsing;
- hardware-dependent parsing;
- network-dependent parsing;
- filesystem-dependent parsing;
- clock-dependent parsing;
- mutable global parser state.

---

44. Safe-Rust Requirement

The HDL grammar is specification/parser input, but all reference compiler infrastructure integrating it must remain compatible with:

Rust 1.97
Rust 1.97.1

and must use safe Rust.

The compiler integration must not require:

unsafe
unsafe fn
unsafe impl
unsafe {}

No HDL feature may require unsafe Rust merely to parse or represent it.

---

45. No Target Discovery During Parsing

The parser must not inspect:

- CPU count;
- GPU count;
- FPGA count;
- QPU count;
- memory size;
- device topology;
- operating system;
- installed tools;
- network availability.

Parsing answers:

«Is this valid Zamani HDL syntax?»

It does not answer:

«Can this particular machine implement it?»

---

46. No Physical Identifiers in Core HDL

Core HDL must not require physical identifiers such as:

fpga0
gpu0
qpu0
pin17
lut3
bram8
dsp12
core4

as intrinsic language constructs.

Such identifiers may exist in explicit target/deployment descriptions when physical binding is genuinely part of a deployment artifact.

They must not become universal source semantics.

---

47. Target Binding

Target binding occurs downstream:

portable HDL
     |
     v
semantic hardware representation
     |
     v
target capability discovery
     |
     v
constraint evaluation
     |
     v
placement/routing/synthesis
     |
     v
target artifact

This allows the same source to be considered for different targets.

---

48. Tiny-to-Large Scaling

A valid HDL design must not become syntactically invalid merely because its scale changes.

The same language model must support:

one signal

through:

millions/billions of generated structures

subject to available resources.

Scaling mechanisms should include:

- generics;
- parameters;
- reusable modules;
- hierarchical composition;
- generate constructs;
- iteration;
- symbolic dimensions;
- templates;
- metaprogramming where supported.

---

49. Infinite-Scale Meaning

"Infinity" is not a literal runtime resource guarantee.

The correct architectural interpretation is:

«Zamani HDL imposes no arbitrary finite machine-size ceiling where the semantics do not require one.»

Actual execution remains bounded by:

- available memory;
- compiler resources;
- synthesis resources;
- target resources;
- execution resources;
- physical laws;
- numerical representation;
- explicit user constraints.

The language must not manufacture additional artificial limits.

---

50. HDL and Classical Computing

HDL must integrate with classical computation without forcing developers to maintain separate semantic universes.

A hardware accelerator can therefore be represented as part of a larger Zamani program:

classical algorithm
      |
      v
hardware implementation
      |
      v
hardware result
      |
      v
classical continuation

The boundary must remain explicit in the AST and semantic model.

---

51. HDL and Distributed Computing

HDL may participate in systems where hardware is distributed across:

- devices;
- nodes;
- accelerators;
- clusters;
- edge systems;
- cloud infrastructure.

The HDL grammar does not own distributed placement.

Distributed placement belongs to:

grammar/distributed/

and downstream deployment infrastructure.

---

52. HDL and AI

AI/ML accelerators may be described using HDL.

The HDL grammar must not hard-code:

- tensor dimensions;
- accelerator counts;
- fixed matrix sizes;
- fixed memory sizes.

Those belong to parameters, types, semantic constraints, resources, or target descriptions as appropriate.

---

53. HDL and Data

Hardware data structures may use:

- arrays;
- vectors;
- records;
- structured types;
- parameterized widths;
- streams.

The grammar should reuse the common type/data model wherever possible.

---

54. HDL and Security

Security-sensitive hardware constructs must remain compositional.

Security semantics may include:

- isolation;
- access constraints;
- secure interfaces;
- cryptographic accelerators;
- trusted boundaries.

The HDL grammar should integrate with the shared security/capability system instead of creating a separate security language.

---

55. Dialect Policy

A dialect may extend HDL only when:

1. the extension is explicitly registered;
2. its namespace is unambiguous;
3. its version is defined;
4. its compatibility rules are defined;
5. its semantic owner is identified;
6. its lowering path is defined;
7. its diagnostics are defined;
8. its tests exist.

Vendor-specific syntax must not silently alter core Zamani semantics.

---

56. Versioning

HDL syntax must be version-aware.

Every incompatible syntax change must have:

- language version impact;
- migration documentation;
- compatibility status;
- deprecation policy;
- test coverage.

The HDL grammar must not use undocumented syntax changes.

---

57. Compatibility

Backward compatibility must distinguish:

source compatibility
grammar compatibility
AST compatibility
semantic compatibility
IR compatibility
backend compatibility
target compatibility

A target becoming incapable of realizing a design does not automatically make the source language incompatible.

---

58. Integration Matrix

Component| HDL relationship
Lexer| Supplies canonical HDL tokens
Parser| Invokes HDL grammar
AST| Represents HDL source structure
Semantic analysis| Validates HDL meaning
Type system| Validates hardware types
Effects| Represents hardware-related effects
Resources| Represents resource requirements
Capabilities| Represents target capabilities
Classical| Enables software/hardware co-design
Quantum| Enables hybrid quantum hardware systems
Hardware| Owns target realization
Optimization| Optimizes semantic hardware representation
Scheduling| Determines timing/order realization
Routing| Determines physical connectivity
ZQN| Remains separate; only integrates where quantum noise semantics are relevant
QEC| Remains separate; HDL may interface with QEC implementations
"quantum::ir"| Remains canonical quantum semantic boundary
Runtime| Executes/deploys compiled artifacts
Interoperability| Integrates external HDL/foreign ecosystems
Diagnostics| Reports source-located errors
Tests| Verifies syntax and integration
Documentation| Defines language contracts

---

59. Forbidden Dependencies

HDL grammar must not directly depend on:

specific FPGA
specific ASIC
specific CPU
specific GPU
specific QPU
specific vendor
specific calibration
specific routing topology
specific scheduler implementation
specific optimizer implementation
specific runtime
specific operating system
specific device address
specific machine size

It may consume shared grammar abstractions representing:

capability
requirement
constraint
resource
target
preference
hint

without knowing their physical realization.

---

60. Testing Contract

Every HDL grammar component must have tests.

Required categories:

Positive

Valid:

- module;
- interface;
- ports;
- signals;
- wires;
- registers;
- memories;
- clocks;
- resets;
- processes;
- combinational blocks;
- sequential blocks;
- state machines;
- pipelines;
- parameters;
- generics;
- instances;
- generate constructs;
- timing;
- assertions.

Negative

Invalid:

- declarations;
- duplicate names where semantic analysis rejects them;
- malformed parameter lists;
- malformed ports;
- malformed timing;
- invalid syntax;
- invalid state-machine syntax;
- invalid process syntax;
- malformed hierarchy.

Boundary

Test:

- zero optional members;
- one member;
- many members;
- deeply nested modules;
- deeply nested expressions;
- large symbolic dimensions;
- large generated structures.

The grammar itself must not impose a machine-sized maximum.

Cross-domain

Test:

classical + HDL
quantum + HDL
HDL + hardware
HDL + distributed
AI + HDL
quantum + classical + HDL
classical + quantum + HDL + hardware

Determinism

Repeated parsing must produce equivalent parse results.

Round-trip

Where a canonical printer exists:

source
 -> parse
 -> AST
 -> print
 -> parse

must preserve intended syntax/semantics.

---

61. Hard-Coding Audit

Every HDL grammar change must be audited for:

- fixed widths;
- fixed dimensions;
- fixed stage counts;
- fixed module counts;
- fixed state counts;
- fixed port counts;
- fixed register counts;
- fixed memory sizes;
- fixed clock counts;
- fixed device IDs;
- fixed FPGA resources;
- fixed ASIC resources;
- fixed topology;
- fixed addresses;
- fixed vendors;
- fixed deployment assumptions.

Every finding must be classified as:

1. language semantic requirement;
2. target-specific requirement;
3. resource constraint;
4. implementation limitation;
5. accidental hard-coding;
6. test-only limitation;
7. documentation-only limitation.

Accidental hard-coding must be removed.

---

62. Security Requirements

Grammar parsing must be safe against malformed input.

The frontend must avoid:

- unbounded accidental recursion where practical;
- infinite parser loops;
- hidden filesystem access;
- hidden network access;
- command execution;
- environment-dependent behavior;
- unsafe Rust;
- implicit code execution.

Compiler resource limits may exist as configurable safety policies.

They must not be confused with language semantic limits.

---

63. Parser Resource Policies

Production implementations may enforce configurable limits for protection against denial-of-service or pathological input.

For example:

maximum parser work
maximum diagnostic count
maximum nesting budget
maximum source bytes

These are implementation/resource policies, not language semantics.

They must therefore be:

- configurable;
- documented;
- diagnosable;
- distinguishable from syntax validity.

---

64. Diagnostics and Recovery

Malformed HDL must never cause parser recovery to invent executable hardware semantics.

Recovery must:

1. identify the unexpected token;
2. record a diagnostic;
3. consume input;
4. guarantee progress;
5. synchronize at an appropriate boundary;
6. resume only when structurally safe.

Error-recovery nodes must remain marked as invalid or incomplete.

---

65. Completion Contract for This README

This README is complete when it defines:

- HDL ownership;
- non-ownership;
- grammar architecture;
- file responsibilities;
- parser integration;
- lexer integration;
- AST integration;
- semantic integration;
- IR integration;
- resource integration;
- hardware integration;
- scheduling integration;
- optimization integration;
- runtime integration;
- quantum integration;
- POCO-REAF requirements;
- scalability requirements;
- hard-coding policy;
- dialect policy;
- compatibility policy;
- testing policy;
- safety policy.

No later HDL grammar file should require changing the architectural ownership established here.

If a later file discovers a genuinely missing language-wide abstraction, the abstraction belongs in the appropriate shared grammar/specification layer rather than silently changing HDL ownership.

---

66. Completion Criteria for "grammar/hdl/README.md"

This file is considered complete only when all of the following are true:

- [x] HDL purpose is defined.
- [x] HDL ownership is defined.
- [x] Non-ownership is defined.
- [x] POCO-REAF is defined.
- [x] hardware independence is defined.
- [x] scalability policy is defined.
- [x] no arbitrary hardware maximums are permitted.
- [x] resource/capability separation is defined.
- [x] lexer boundary is defined.
- [x] parser boundary is defined.
- [x] AST contract is defined.
- [x] semantic contract is defined.
- [x] IR boundary is defined.
- [x] quantum boundary is defined.
- [x] "quantum::ir" remains outside HDL ownership.
- [x] scheduling remains outside HDL ownership.
- [x] optimization remains outside HDL ownership.
- [x] hardware realization remains outside HDL ownership.
- [x] runtime remains outside grammar ownership.
- [x] dialect ownership is defined.
- [x] versioning is defined.
- [x] compatibility is defined.
- [x] diagnostics are defined.
- [x] safe-Rust requirement is defined.
- [x] deterministic parsing is defined.
- [x] testing requirements are defined.
- [x] hard-coding audit is defined.
- [x] cross-domain integration is defined.
- [x] independently-completable-file principle is defined.

---

67. Implementation Order After This README

"README.md" is intentionally an architectural foundation.

The HDL implementation should then proceed in dependency order:

README.md
    |
    v
hardware-generics.g4
    |
    v
hardware-parameters.g4
    |
    v
ports.g4
    |
    v
hardware-interfaces.g4
    |
    v
signals.g4
    |
    v
wires.g4
    |
    v
registers.g4
    |
    v
clocks.g4
    |
    v
timing.g4
    |
    v
memories.g4
    |
    v
combinational.g4
    |
    v
sequential.g4
    |
    v
processes.g4
    |
    v
state-machines.g4
    |
    v
pipelines.g4
    |
    v
hardware-modules.g4
    |
    v
hardware-dialects.g4
    |
    v
hdl.g4
    |
    v
ANTLR integration
    |
    v
AST integration
    |
    v
semantic integration
    |
    v
hardware IR integration
    |
    v
compiler integration
    |
    v
cross-domain integration

The exact order may be adjusted when a dependency audit proves that two files are genuinely independent, but a later HDL file must never redefine an earlier file's ownership.

---

68. Final Principle

The fundamental HDL rule is:

«Zamani HDL describes hardware intent, structure, behavior, interfaces, timing semantics, capabilities, requirements, and constraints — not arbitrary limitations of the machine currently available.»

Therefore:

One HDL program
      |
      v
One semantic meaning
      |
      +------------------+
      |                  |
      v                  v
Small implementation   Large implementation
      |                  |
      +--------+---------+
               |
               v
        Many architectures
               |
               v
        Many technologies
               |
               v
        Many deployment targets
               |
               v
        Future hardware

Subject only to the semantics of the program, declared constraints, target capabilities, and available resources.

That is the HDL expression of:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

and:

Zamani
From Atom to Everywhere