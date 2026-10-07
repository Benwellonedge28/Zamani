Zamani HDL Grammar

Path: "grammar/hdl/"
Role: HDL subsystem architecture, orchestration, ownership, integration, scalability, and completion contract
Language: Zamani
Grammar technology: ANTLR4
Rust baseline: Rust 1.97 or later
Rust edition: Rust 2021
Implementation safety: Safe Rust only; no "unsafe" code
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Repository: "Benwellonedge28/Zamani"

---

1. Purpose

"grammar/hdl/" is Zamani's complete Hardware Description Language grammar subsystem.

This directory provides the source-language grammar required to express:

- hardware structure;
- hardware behavior;
- hardware/software co-design;
- parameterized hardware;
- generic hardware;
- interfaces;
- ports;
- signals;
- nets;
- wires;
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
- parallel hardware;
- generated hardware;
- module instances;
- protocols;
- timing intent;
- verification intent;
- simulation intent;
- synthesis intent;
- physical implementation intent;
- hardware dialects;
- accelerator intent;
- resource requirements;
- capability requirements;
- contracts;
- policies;
- provenance;
- classical/hardware integration;
- quantum/hardware integration;
- future hardware classes.

The directory is part of the single Zamani language.

It is not:

- a separate programming language;
- a separate lexer;
- a separate AST;
- a separate semantic universe;
- a separate compiler;
- a target-selection engine;
- a placement engine;
- a routing engine;
- a synthesis implementation;
- a physical-design system;
- a HAL;
- a quantum compiler;
- a QEC implementation.

The directory describes portable hardware and computational intent.

Downstream compiler layers determine how that intent is validated, optimized, elaborated, synthesized, scheduled, placed, routed, lowered, and realized.

---

2. The HDL Directory Is Orchestrated by This File

"grammar/hdl/README.md" is the directory-level orchestration contract.

It answers:

1. What every HDL file is responsible for.
2. Which file owns each HDL concept.
3. Which files may depend on which other files.
4. Which concepts belong outside "grammar/hdl/".
5. How HDL integrates with the repository-wide lexer.
6. How HDL integrates with "grammar/Zamani.g4".
7. How HDL integrates with the domain-neutral AST.
8. How HDL integrates with semantic analysis.
9. How resources and capabilities are represented.
10. How effects, contracts, policies, and provenance participate.
11. How classical computation integrates.
12. How quantum computation integrates.
13. How hardware/software co-design integrates.
14. How simulation, verification, and synthesis are separated.
15. How target independence is preserved.
16. How arbitrary parameterized scale is supported.
17. How generated hardware avoids language-level limits.
18. How every individual HDL file becomes independently completable.
19. What constitutes production readiness.
20. What must never be introduced into the HDL grammar.

This README is therefore the orchestrator of the directory, but it does not replace "hdl.g4" as the executable ANTLR composition root.

The two roles are deliberately different:

grammar/hdl/README.md
    │
    │ architecture / ownership / integration contract
    ▼
grammar/hdl/hdl.g4
    │
    │ executable ANTLR composition
    ▼
HDL grammar delegates

---

3. Authority Hierarchy

The HDL subsystem participates in the repository-wide authority model.

The authority chain is:

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ▼
grammar/spec/
        │
        ├── grammar/spec/hdl.md
        │
        ▼
grammar/lexer/
        │
        ▼
grammar/antlr/ZamaniLexer.g4
        │
        ▼
grammar/hdl/
        │
        ▼
grammar/hdl/hdl.g4
        │
        ▼
grammar/Zamani.g4
        │
        ▼
frontend AST
        │
        ▼
semantic analysis
        │
        ▼
canonical semantic model
        │
        ├── Classical semantics
        ├── Quantum semantics
        ├── HDL/hardware semantics
        ├── AI/data semantics
        ├── distributed semantics
        └── hybrid semantics
        │
        ▼
canonical compiler IR boundaries
        │
        ├── Classical IR
        └── quantum::ir
        │
        ▼
optimization
        │
        ▼
lowering / synthesis
        │
        ▼
routing / scheduling / placement
        │
        ▼
resilience / QEC / ZQN where applicable
        │
        ▼
HAL
        │
        ▼
target realization

No file under "grammar/hdl/" may establish a competing authority.

---

4. Authority of Each Repository Layer

4.1 "grammar/DESIGN.md"

Owns:

- repository-wide architecture;
- language boundaries;
- AST architecture;
- semantic architecture;
- IR architecture;
- resource abstraction;
- capability abstraction;
- effects;
- contracts;
- policies;
- provenance;
- portability;
- scalability;
- compatibility.

"grammar/hdl/README.md" must conform to it.

---

4.2 "grammar/specification/"

Owns the human-readable normative language specification.

HDL-specific semantic definitions belong here when the specification structure requires them.

---

4.3 "grammar/spec/hdl.md"

Owns the normative HDL semantic contract.

It defines what HDL constructs mean.

This README defines how the HDL grammar directory is organized and integrated.

The distinction is:

spec/hdl.md
    = what HDL means

hdl/README.md
    = how the HDL grammar subsystem is organized and integrated

hdl/hdl.g4
    = how HDL grammar components are composed

hdl/*.g4
    = individual syntax ownership

---

4.4 "grammar/lexer/"

Owns lexical registration.

HDL does not create a second lexer.

---

4.5 "grammar/antlr/ZamaniLexer.g4"

Owns executable lexical recognition for the language.

HDL consumes the canonical lexical system.

---

4.6 "grammar/Zamani.g4"

Owns complete-language composition.

It remains the canonical Zamani grammar root.

HDL does not replace it.

---

4.7 "grammar/hdl/hdl.g4"

Owns the HDL grammar composition root.

It is the only executable HDL composition root.

It must not become a second monolithic "Zamani.g4".

---

5. Canonical Architecture

The complete architecture is:

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
                     HDL Composition Root
                         hdl/hdl.g4
                              │
             ┌────────────────┼────────────────┐
             │                │                │
             ▼                ▼                ▼
         HDL modules       HDL behavior     HDL intent
             │                │                │
             └────────────────┼────────────────┘
                              │
                              ▼
                    Domain-Neutral AST
                              │
                              ▼
                  Structural Validation
                              │
       ┌──────────────┬───────┼───────┬──────────────┐
       │              │       │       │              │
       ▼              ▼       ▼       ▼              ▼
     Types         Effects Resources Contracts     Policies
       │              │       │       │              │
       └──────────────┴───────┼───────┴──────────────┘
                              │
                              ▼
                         Provenance
                              │
                              ▼
                    HDL Semantic Model
                              │
             ┌────────────────┼────────────────┐
             │                │                │
             ▼                ▼                ▼
       Verification      Simulation       Synthesis
             │                │                │
             └────────────────┼────────────────┘
                              │
                              ▼
                       Compiler IR Boundary
                              │
               ┌──────────────┴──────────────┐
               │                             │
               ▼                             ▼
         Classical IR                    quantum::ir
               │                             │
               └──────────────┬──────────────┘
                              │
                              ▼
                         Optimization
                              │
                              ▼
                       Lowering / Elaboration
                              │
                              ▼
                   Scheduling / Placement
                              │
                              ▼
                            Routing
                              │
                              ▼
                    Resilience / Recovery
                              │
                              ▼
                           ZQN
                              │
                              ▼
                            HAL
                              │
                              ▼
                       Target realization

The HDL grammar owns only the source-language portion.

---

6. POCO-REAF Contract

HDL is subject to:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

A portable HDL program should describe:

- computational intent;
- structural intent;
- behavioral intent;
- timing intent;
- verification properties;
- resource requirements;
- capability requirements;
- constraints;
- preferences;
- policies;
- contracts;
- interoperability requirements;
- provenance requirements.

It should not unnecessarily describe:

- one FPGA;
- one ASIC;
- one CPU;
- one GPU;
- one accelerator;
- one board;
- one package;
- one physical pin;
- one physical routing track;
- one memory block;
- one vendor primitive;
- one fixed hardware inventory.

The compiler and target environment determine realization.

---

7. Meaning of "Scale to Infinity"

For this architecture:

«"Infinity" means that the language does not impose an arbitrary finite maximum on semantically parameterizable quantities.»

It does not mean that physical resources are infinite.

For example:

module Pipeline<STAGES> {
    ...
}

may represent any valid "STAGES" value permitted by:

- the source program;
- type semantics;
- compiler resources;
- compilation policy;
- available memory;
- available compilation time;
- target capabilities;
- target resources;
- explicit program constraints.

A target that cannot satisfy a request must report a resource or capability failure.

It must not redefine the language as having a smaller universal limit.

---

8. No Universal Hardware Ceilings

No HDL file may introduce language-level ceilings such as:

MAX_MODULES
MAX_PORTS
MAX_SIGNALS
MAX_NETS
MAX_WIRES
MAX_REGISTERS
MAX_MEMORIES
MAX_WIDTH
MAX_DEPTH
MAX_PIPELINE_STAGES
MAX_STATES
MAX_TRANSITIONS
MAX_INSTANCES
MAX_GENERATED_INSTANCES
MAX_CLOCKS
MAX_CLOCK_DOMAINS
MAX_CHANNELS
MAX_INTERFACES
MAX_ARRAY_ELEMENTS
MAX_DEVICES
MAX_ACCELERATORS
MAX_FPGAS
MAX_ASICS
MAX_QPUS
MAX_QUBITS
MAX_NODES
MAX_MEMORY
MAX_NETWORK_SIZE

Nor may equivalent hidden limits be introduced through grammar cardinalities.

For example, these are forbidden as universal grammar restrictions:

width <= 32
ports <= 128
states <= 1024
pipeline_depth <= 64
instances <= 1000

A number appearing in a Zamani source program is program semantics.

It is not automatically a language capacity limit.

---

9. Program Constants Versus Language Limits

This is valid:

const WIDTH = 1024;
memory data : logic[WIDTH];

because "1024" is part of the program's meaning.

This is not valid as a universal grammar rule:

WIDTH <= 1024

merely because an implementation currently has such a limitation.

The same distinction applies to:

- widths;
- depths;
- ports;
- modules;
- instances;
- pipeline stages;
- states;
- transitions;
- memories;
- clocks;
- channels;
- interfaces;
- generated structures;
- hardware resources.

---

10. Current Canonical HDL File Set

The current "grammar/hdl/" directory contains:

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
├── hardware-modules.g4
├── hardware-software-integration.g4
├── hdl.g4
├── interfaces.g4
├── memories.g4
├── nets.g4
├── parallelism.g4
├── parameters.g4
├── physical-intent.g4
├── pipelines.g4
├── ports.g4
├── processes.g4
├── protocols.g4
├── registers.g4
├── reset.g4
├── sequential.g4
├── signals.g4
├── simulation.g4
├── state_machines.g4
├── synthesis.g4
├── timing.g4
├── verification.g4
└── wires.g4

These filenames are preserved.

A new file must not be created merely because another name appears more attractive.

A new file is justified only when:

1. a new semantic responsibility exists;
2. an existing owner cannot reasonably own it;
3. the dependency boundary is explicit;
4. the AST/semantic/IR integration is defined;
5. tests can be assigned;
6. the specification has an owner.

---

11. File Ownership Matrix

File| Primary responsibility
"hdl.g4"| HDL composition and dispatch
"hardware-modules.g4"| Hardware module declarations
"hardware-generics.g4"| Generic hardware declarations and constraints
"parameters.g4"| HDL parameter declarations and references
"interfaces.g4"| HDL interface declarations
"ports.g4"| Port declarations
"signals.g4"| Logical signal declarations
"nets.g4"| Canonical logical connectivity/net declarations
"wires.g4"| Wire-specific compatibility/surface syntax integrated with nets
"registers.g4"| Register declarations
"memories.g4"| Memory declarations
"arrays.g4"| Hardware array structure
"clocks.g4"| Clock declarations and clock-domain identity
"clocking.g4"| Clock relationship and clocking constructs
"reset.g4"| Reset declarations and reset semantics at source boundary
"processes.g4"| HDL processes
"combinational.g4"| Combinational behavior
"sequential.g4"| Sequential behavior
"state_machines.g4"| State-machine syntax
"pipelines.g4"| Pipeline declarations and structure
"parallelism.g4"| HDL-specific parallel structure
"generate.g4"| Parameterized/generated hardware structure
"protocols.g4"| Hardware protocol intent
"timing.g4"| Timing intent
"assertions.g4"| HDL assertion syntax
"verification.g4"| Verification intent and verification composition
"simulation.g4"| Simulation intent
"synthesis.g4"| Synthesis intent
"physical-intent.g4"| Explicit physical implementation intent
"co-design.g4"| Hardware/software co-design
"hardware-software-integration.g4"| Hardware/software boundary integration
"hardware-dialects.g4"| Explicit HDL dialect extension points
"README.md"| Directory-wide orchestration and integration contract

No file owns another file's primary semantic concept.

---

12. Standard Contract Required by Every HDL File

Every ".g4" file under this directory must document the following before it is considered complete.

12.1 Identity

FILE:
DOMAIN:
STATUS:
OWNER:

---

12.2 Purpose

State exactly why the file exists.

---

12.3 Owns

List every syntax concept whose primary ownership belongs to the file.

---

12.4 Does Not Own

Explicitly identify adjacent concepts that belong elsewhere.

This is mandatory for overlapping concepts.

---

12.5 Dependencies

Declare:

DEPENDS_ON:

including:

- lexer dependencies;
- core grammar dependencies;
- expression dependencies;
- type dependencies;
- declaration dependencies;
- sibling HDL dependencies;
- repository-wide semantic dependencies.

---

12.6 Exports

Declare:

EXPORTS:

containing only public grammar rules intended for consumption by another grammar.

---

12.7 Consumers

Declare:

CONSUMED_BY:

including:

- "hdl.g4";
- "Zamani.g4";
- other HDL delegates;
- standalone parser tests;
- tooling where applicable.

---

12.8 AST Contract

State:

- AST category;
- source-span requirements;
- attributes;
- identifiers;
- generic information;
- source provenance.

The grammar must not create a physical implementation AST.

---

12.9 Semantic Contract

State exactly what semantic analysis must derive.

Examples:

- width;
- shape;
- direction;
- connectivity;
- clock domain;
- reset behavior;
- timing;
- resource requirements;
- capability requirements;
- contracts;
- policies;
- provenance.

---

12.10 Type Contract

State:

- accepted types;
- produced types;
- generic constraints;
- width constraints;
- shape constraints;
- compatibility rules.

---

12.11 Effect Contract

State whether the construct participates in:

- IO;
- mutation;
- hardware interaction;
- simulation;
- foreign interaction;
- network behavior;
- distributed behavior;
- measurement;
- randomness;
- code generation;
- other repository-wide effects.

The file must reuse the canonical effect system.

---

12.12 Capability Contract

State which capabilities can be:

- required;
- preferred;
- constrained;
- prohibited.

The file must not discover physical hardware.

---

12.13 Resource Contract

State which resource concepts may be expressed.

Examples:

requires memory >= required_memory;
requires capability("hardware.pipeline");
requires capability("streaming");

The exact universal resource grammar remains owned by "grammar/resources/".

---

12.14 Contract and Policy Contract

State how the construct interacts with:

- "requires";
- "ensures";
- "invariant";
- "assume";
- "guarantee";
- "property";
- policies;
- permissions;
- prohibitions;
- fallback behavior.

---

12.15 Provenance Contract

Every significant source construct must remain traceable through:

source
    ↓
AST
    ↓
semantic representation
    ↓
transformation
    ↓
implementation

Source locations must not be discarded at the grammar boundary.

---

12.16 IR Contract

State:

- what semantic representation the grammar feeds;
- which compiler-owned IR consumes it;
- whether it can participate in Classical IR;
- whether it can participate in "quantum::ir";
- whether it remains hardware semantic information until later lowering.

The HDL grammar must never invent a competing universal IR.

---

12.17 Diagnostics Contract

Define the minimum diagnostics for:

- malformed syntax;
- missing identifiers;
- invalid references;
- incompatible types;
- invalid dimensions;
- illegal connectivity;
- invalid clock relationships;
- invalid reset relationships;
- invalid generated structures;
- invalid protocol usage.

Semantic diagnostics remain semantic-layer responsibilities.

---

12.18 Tests

Every file must have:

positive tests
negative tests
boundary tests
scalability tests
determinism tests
compatibility tests
cross-domain tests

where applicable.

---

13. Dependency Direction

The dependency direction is:

canonical lexer
       ↓
core grammar
       ↓
types / expressions / declarations
       ↓
HDL leaf grammars
       ↓
hdl.g4
       ↓
Zamani.g4
       ↓
frontend AST
       ↓
semantic analysis
       ↓
compiler

No HDL leaf grammar may depend on:

- backend target implementation;
- HAL implementation;
- physical device inventory;
- runtime scheduler implementation;
- target discovery;
- vendor SDK;
- QPU provider API.

Those are downstream concerns.

---

14. "hdl.g4" — Composition Root

"grammar/hdl/hdl.g4" is the executable HDL composition root.

It owns:

- HDL grammar identity;
- imports;
- HDL source entry point;
- declaration dispatch;
- statement dispatch;
- expression integration;
- type integration;
- attribute integration;
- resource integration;
- capability integration;
- contract integration;
- policy integration;
- provenance integration;
- effect integration;
- cross-domain façades;
- standalone HDL parsing boundary.

It does not own the detailed syntax of individual HDL features.

The rule is:

hdl.g4
    =
composition

not:

hdl.g4
    =
implementation of every feature

---

15. "hdl.g4" Import Contract

The existing HDL composition root imports the HDL delegates.

The imported grammar identities must remain synchronized with the actual filenames and grammar declarations.

Every imported grammar must provide the public rules consumed by "hdl.g4".

The repository must validate:

1. every imported grammar exists;
2. every grammar declaration name is correct;
3. every imported public rule exists;
4. every referenced rule resolves;
5. no duplicate canonical production exists;
6. no import cycle exists;
7. no feature is silently unreachable;
8. no feature is silently duplicated.

A filename and an ANTLR grammar name are not assumed to be identical.

Both must be validated.

---

16. "hdl.g4" Public Entry Points

The HDL composition root provides:

hdlDesign
hdlSourceElement
hdlDeclaration
hdlStatement
hdlExpression
hdlTypeExpression
hdlAttribute
hdlAttributeList
hdlBlock
hdlName
hdlQualifiedName
hdlRequirement
hdlConstraint
hdlCapability
hdlPolicy
hdlContract
hdlProvenance
hdlEffect
hdlDomainElement
hdlHybridElement
hdlVerificationElement
hdlSimulationElement
hdlSynthesisElement
hdlResourceElement

These are integration façades.

They must not duplicate the semantic ownership of their underlying systems.

---

17. Standalone HDL Parsing

The standalone HDL entry point:

hdlDesign
    : hdlSourceElement* EOF
    ;

is allowed to consume an arbitrary number of source elements.

There is no language-level source-count ceiling.

Standalone parsing exists for:

- grammar validation;
- parser tests;
- HDL tooling;
- IDE support;
- conformance testing;
- isolated HDL development.

The complete Zamani language remains rooted at "grammar/Zamani.g4".

---

18. Canonical Lexer Integration

All HDL grammars use:

grammar/antlr/ZamaniLexer.g4

through the canonical token vocabulary.

No HDL file may define lexer rules.

No HDL file may create:

HDLLexer
HardwareLexer
VendorHDLLexer
QuantumHDLLexer

or another lexical universe.

---

19. Keyword Policy

An HDL term becomes a language keyword only when it has stable language-level grammatical meaning.

Hardware-specific names should normally remain identifiers.

For example:

adder
cache
tensor_unit
compute_engine
accelerator
pipeline
memory_controller
vendor_component
future_device

should not become reserved words merely because they are hardware concepts.

Vendor names and future hardware names must remain extensible.

---

20. Shared Language Constructs

HDL must reuse repository-wide rules for:

- identifiers;
- qualified names;
- namespaces;
- paths;
- literals;
- expressions;
- operators;
- types;
- generics;
- attributes;
- annotations;
- modifiers;
- declarations;
- blocks;
- contracts;
- requirements;
- constraints;
- capabilities;
- policies;
- provenance;
- effects.

HDL must not create duplicate forms such as:

hdlIdentifier
hardwareIdentifier
hdlExpression
hardwareExpression
hdlType
hardwareType

unless a normative semantic distinction requires them.

---

21. "hardware-modules.g4"

Owns

- hardware module declaration;
- module identity;
- module body;
- module-level structural composition;
- module-level parameters/generics integration;
- module instances.

Does not own

- individual ports;
- memory implementation;
- clock implementation;
- process semantics;
- synthesis;
- physical placement.

Integration

hardware-modules.g4
    ↓
hdl.g4
    ↓
domain-neutral AST
    ↓
hardware semantic model

Completion

A module grammar is complete only when module declarations can be independently parsed, represented, validated, tested, and consumed without requiring another module grammar to redefine them.

---

22. "hardware-generics.g4"

Owns

- generic hardware parameters;
- generic constraints;
- generic specialization syntax;
- symbolic hardware structure.

Does not own

- ordinary program generics;
- type-system semantics;
- target hardware limits.

Generic semantics must reuse the repository-wide type and constraint systems.

Generic hardware must support arbitrary symbolic scale.

---

23. "parameters.g4"

Owns

- HDL parameter declarations;
- local parameter declarations where applicable;
- parameter references;
- parameter expressions.

Parameters may represent:

- widths;
- depths;
- dimensions;
- latency;
- pipeline structure;
- replication;
- protocol configuration;
- implementation preferences.

A parameter value is program semantics.

It is not a language maximum.

---

24. "interfaces.g4"

Owns

Hardware interface declarations.

Interfaces may express:

- ports;
- signals;
- types;
- parameters;
- protocols;
- timing;
- ordering;
- capabilities;
- requirements.

An interface is a logical contract.

It does not automatically identify:

- a physical bus;
- a package;
- a board connector;
- a vendor interface.

---

25. "ports.g4"

Owns

Port syntax.

Ports may express:

- direction;
- type;
- dimensions;
- width;
- protocol;
- timing;
- attributes;
- capability requirements.

Common directions may include:

input
output
inout

Port count is never a universal language limit.

---

26. "signals.g4"

Owns

Logical signal declarations.

The semantic layer determines:

- type compatibility;
- width compatibility;
- driver legality;
- resolution;
- timing;
- clock-domain compatibility.

A signal is not automatically a physical wire.

---

27. "nets.g4"

"nets.g4" owns the canonical logical connectivity model.

A net represents logical connectivity.

It does not represent:

- routing tracks;
- switch boxes;
- metal layers;
- package traces;
- physical wires.

Physical routing remains downstream.

---

28. "wires.g4"

"wires.g4" must not become a second connectivity model.

Its role is limited to wire-specific syntax or compatibility surfaces that normalize into the canonical net/connectivity semantic model.

The ownership invariant is:

logical connectivity
        ↓
nets.g4

not:

nets.g4
    +
wires.g4
    =
two independent connectivity systems

---

29. "registers.g4"

Owns

Register declarations and register-level source intent.

May express:

- type;
- width;
- initialization;
- clock;
- edge;
- reset;
- enable;
- attributes.

It does not decide physical realization.

A logical register may become:

- flip-flops;
- distributed storage;
- memory;
- specialized storage;
- another semantically equivalent realization.

There is no universal register width.

---

30. "memories.g4"

Owns

Memory declarations.

May express:

- element type;
- width;
- depth;
- dimensions;
- read semantics;
- write semantics;
- latency;
- throughput;
- initialization;
- persistence;
- attributes.

Memory dimensions are program semantics.

Physical realization may use:

- registers;
- SRAM;
- distributed memory;
- block memory;
- external memory;
- accelerator memory;
- another target resource.

No universal memory size is encoded.

---

31. "arrays.g4"

Owns

Hardware array structure.

Arrays may represent:

- storage;
- replicated logic;
- ports;
- signals;
- memories;
- generated structures.

Array extents remain symbolic or program-defined.

No universal array length is permitted.

---

32. "clocks.g4"

Owns

Clock declarations and clock identity.

May express:

- clock source identity;
- frequency;
- period;
- phase;
- relationship;
- attributes;
- requirements.

The grammar does not construct:

- clock trees;
- PLL configurations;
- physical clock routing;
- target-specific clock resources.

Those are downstream.

---

33. "clocking.g4"

Owns

Clock relationships and clocking structures.

It must remain distinct from:

clock declaration
clock-domain analysis
clock-tree implementation

The semantic layer determines legality.

The backend determines realization.

---

34. "reset.g4"

Owns

Reset source syntax and reset intent.

It may express:

- reset identity;
- polarity;
- synchronization intent;
- relation to clocks;
- initialization intent.

It does not implement:

- physical reset networks;
- device startup circuitry;
- target-specific reset primitives.

---

35. "processes.g4"

Owns

Hardware process syntax.

Processes describe behavioral intent.

Semantic analysis determines whether a process is:

- combinational;
- sequential;
- clocked;
- event-driven;
- stateful;
- otherwise supported by the language specification.

A process must not be treated as an imperative software thread merely because its syntax contains statements.

---

36. "combinational.g4"

Owns

Combinational hardware behavior.

The semantic layer must validate:

- complete assignments;
- absence of unintended state;
- type compatibility;
- dependency relationships;
- combinational legality.

Synthesis decides implementation.

---

37. "sequential.g4"

Owns

Sequential hardware behavior.

The semantic layer validates:

- clock relationship;
- state updates;
- reset behavior;
- enable semantics;
- sequential completeness;
- ordering.

The backend chooses physical realization.

---

38. "state_machines.g4"

Owns

State-machine syntax.

A state machine may contain:

- states;
- transitions;
- guards;
- actions;
- initial state;
- terminal states;
- properties.

The number of states and transitions is program semantics.

There is no universal maximum.

---

39. "pipelines.g4"

Owns

Pipeline structure and pipeline intent.

It may express:

- stages;
- stage relationships;
- dependencies;
- latency intent;
- throughput intent;
- balancing constraints;
- resource-sharing preferences.

The actual number of physical stages is determined downstream.

A source program may request a parameterized stage count.

---

40. "parallelism.g4"

Owns

HDL-specific parallel structure.

It must integrate with:

grammar/concurrency/
grammar/distributed/
grammar/execution/
grammar/resources/
grammar/hardware/

It must not create a second concurrency model.

Parallelism describes semantic opportunity or required structure.

Physical replication is a downstream realization choice unless explicitly constrained.

---

41. "generate.g4"

Owns

Parameterized and generated hardware structure.

Generation may depend on:

- parameters;
- generic arguments;
- compile-time conditions;
- symbolic dimensions;
- program structure.

Generation must remain deterministic under identical:

- source;
- language version;
- dialect configuration;
- explicit compilation configuration.

Generation must not inspect:

- network state;
- physical device inventory;
- wall-clock time;
- ambient environment;
- uncontrolled randomness.

---

42. "protocols.g4"

Owns

Hardware protocol intent.

Protocols may describe:

- message structure;
- ordering;
- handshake;
- sequencing;
- timing;
- flow control;
- capability requirements.

Protocol syntax must remain independent of a particular vendor bus unless represented through an explicit dialect.

---

43. "timing.g4"

Owns

Timing intent.

Timing may express:

- latency;
- period;
- frequency;
- setup intent;
- hold intent;
- ordering;
- synchronization;
- throughput;
- timing relationships.

Timing values are semantic requirements or constraints.

They are not universal hardware capacities.

Timing closure remains downstream.

---

44. "assertions.g4"

Owns

HDL assertion syntax.

Assertions must integrate with:

grammar/validation/
grammar/spec/
grammar/contracts/

where applicable.

Assertions describe properties.

They do not implement the verification engine.

---

45. "verification.g4"

Owns

Verification intent.

It may represent:

- properties;
- assertions;
- assumptions;
- guarantees;
- coverage intent;
- verification configurations;
- verification scopes.

Verification semantics must remain separate from simulation and synthesis.

---

46. "simulation.g4"

Owns

Simulation intent.

Simulation may support:

- behavioral simulation;
- cycle-oriented simulation;
- timing-oriented simulation;
- fault simulation;
- hardware-model simulation;
- hybrid simulation.

Simulation is an execution strategy.

It is not a second language.

It is not a replacement for target execution.

---

47. "synthesis.g4"

Owns

Synthesis intent.

It may express:

- synthesis constraints;
- optimization preferences;
- implementation properties;
- preservation requirements;
- approximation permissions where explicitly supported.

It does not perform synthesis.

Synthesis implementation belongs downstream.

---

48. "physical-intent.g4"

Owns

Explicit physical implementation intent when the language permits it.

Examples may include:

- physical placement preferences;
- implementation regions;
- physical relationships;
- packaging intent;
- target-specific constraints.

Physical intent must remain explicitly distinguishable from portable intent.

Portable HDL must not silently become target-specific.

---

49. "co-design.g4"

Owns

Hardware/software co-design syntax.

It provides a source-level boundary between:

software intent
      ↕
hardware intent

A single Zamani program may therefore express:

- algorithm;
- accelerator;
- data movement;
- memory behavior;
- hardware execution;
- software control;
- timing;
- verification;
- resource requirements.

The semantic system maintains one overall program model.

---

50. "hardware-software-integration.g4"

Owns

Explicit hardware/software integration constructs.

The architecture is:

Zamani software
       │
       ▼
integration boundary
       │
       ▼
hardware semantic model
       │
       ▼
compiler
       │
       ▼
target realization

This file must integrate with:

- "grammar/classical/";
- "grammar/effects/";
- "grammar/resources/";
- "grammar/capabilities/";
- "grammar/interoperability/";
- "grammar/execution/";
- "grammar/hardware/".

It must not duplicate FFI/ABI ownership.

---

51. "hardware-dialects.g4"

Owns

Explicit HDL dialect extension points.

A dialect must be:

- identified;
- namespaced;
- versioned;
- explicitly selected;
- validated;
- mapped to the canonical AST;
- mapped to semantic constructs;
- mapped to compiler IR;
- tested.

A dialect must not silently change core HDL meaning.

Vendor-specific constructs belong here or in the appropriate interoperability/target layer.

They do not become universal Zamani grammar.

---

52. Cross-Domain Integration

HDL participates in the complete Zamani architecture.

It may interact with:

classical
quantum
AI
data
tensor
concurrency
distributed
networking
security
memory
execution
interoperability
metaprogramming
resources
effects
validation
policies
provenance
hardware

The HDL grammar does not copy those grammars.

It consumes their canonical integration points.

---

53. Classical Integration

Classical computation remains owned by the classical subsystem.

HDL may represent the realization of a classical computation in hardware.

The conceptual path is:

classical intent
       │
       ▼
semantic model
       │
       ▼
hardware realization intent
       │
       ▼
HDL semantics
       │
       ▼
compiler

The algorithm's meaning must not change merely because its implementation moves between:

- CPU;
- multicore;
- GPU;
- FPGA;
- ASIC;
- accelerator;
- future target.

---

54. Quantum Integration

HDL can participate in hybrid quantum systems.

The ownership boundary is:

HDL
 │
 ├── control
 ├── timing
 ├── interfaces
 ├── memory
 ├── classical control
 └── hardware around quantum computation
 │
 ▼
hybrid semantic model
 │
 ▼
quantum semantics
 │
 ▼
quantum::ir

HDL must not define:

- quantum operations;
- quantum states;
- qubit allocation;
- quantum routing;
- QEC implementation;
- physical QPU topology;
- quantum calibration.

Those belong to the quantum/compiler/runtime architecture.

---

55. Canonical Quantum IR Boundary

The canonical quantum IR remains:

quantum::ir

There must not be:

HDLQuantumIR
HardwareQuantumIR
QuantumHardwareIR
HDLQPUIR

as competing universal quantum representations.

HDL provides hardware intent.

Quantum semantics remain owned by the quantum subsystem.

---

56. Classical IR Boundary

The repository-wide Classical IR remains the canonical representation for applicable classical computation.

HDL must not invent a competing classical IR.

Hardware-specific semantic information remains in the hardware semantic layer until compiler-owned lowering determines the appropriate representation.

---

57. No Artificial "Hardware IR" Authority

This README deliberately does not declare a new universal "HardwareIR" as another language-wide authority.

The flow is:

HDL source
    ↓
domain-neutral AST
    ↓
hardware semantic model
    ↓
compiler-owned lowering
    ├── Classical IR where appropriate
    ├── quantum::ir where quantum semantics apply
    └── target/compiler representations where explicitly owned

If a dedicated hardware IR is later introduced, it must be specified and owned by the compiler/IR architecture, not silently created by "grammar/hdl/".

---

58. Resource Integration

HDL consumes the repository-wide resource model.

The semantic distinction is:

requirement
constraint
capability
preference
hint
resource
target
placement
realization

These are not interchangeable.

For example:

requires capability("hardware.pipeline");

means the program requires a capability.

It does not mean:

use FPGA 0

Likewise:

prefer capability("parallel.compute");

is a preference rather than a physical allocation.

---

59. Capability Integration

Capabilities describe what an environment can provide.

Examples may include:

hardware.pipeline
hardware.streaming
hardware.reconfigurable
accelerator.compute
tensor.compute
memory.coherent
quantum.control
quantum.measurement

The names remain open-world.

The grammar must not enumerate every future hardware capability.

Capabilities are resolved downstream.

---

60. Effect Integration

HDL participates in the repository-wide effect system.

Potential effects include:

- IO;
- mutation;
- hardware interaction;
- native;
- foreign;
- distributed;
- network;
- simulation;
- measurement;
- quantum;
- code generation;
- reflection where applicable.

The HDL grammar does not implement effect checking.

It provides source constructs that semantic analysis annotates.

---

61. Contract Integration

HDL constructs may participate in:

requires
ensures
invariant
assume
guarantee
property

Examples:

requires capability("hardware.pipeline");

ensures output_valid;

invariant no_conflicting_drivers;

The universal contract system remains outside "grammar/hdl/".

HDL consumes it.

---

62. Policy Integration

HDL must integrate with repository-wide policies.

Policies may constrain:

- target selection;
- resource use;
- physical intent;
- optimization;
- approximation;
- simulation;
- synthesis;
- security;
- deployment;
- adaptation.

The HDL grammar must not create a second policy engine.

---

63. Provenance Integration

Hardware compilation can involve substantial transformations.

The compiler must therefore preserve:

source
   ↓
parsed construct
   ↓
semantic construct
   ↓
optimization
   ↓
synthesis
   ↓
lowering
   ↓
physical realization

Provenance can be used to explain:

- why a module was specialized;
- why a pipeline was transformed;
- why resources were replicated;
- why a target was rejected;
- why an implementation was selected;
- which source construct produced a physical structure.

The grammar records source structure.

The semantic/compiler layers construct provenance.

---

64. Determinism

Parsing and deterministic semantic processing must depend only on explicitly defined inputs.

They must not depend on:

- machine CPU count;
- GPU availability;
- FPGA inventory;
- QPU inventory;
- network state;
- filesystem state;
- wall-clock time;
- uncontrolled randomness;
- scheduler state;
- physical target discovery.

Identical source plus identical declared configuration must produce equivalent parser and semantic results.

---

65. Resource Exhaustion Versus Language Limits

Compiler resource exhaustion is allowed.

Language-level artificial limits are not.

For example, a compiler may be unable to elaborate a generated design because the compilation environment has insufficient memory.

The diagnostic must describe resource exhaustion.

It must not claim:

Zamani supports only N generated instances.

unless that is an explicit program or policy constraint rather than a universal language restriction.

---

66. Security and Resource-Bounded Compilation

Compile-time generation can potentially produce extremely large structures.

Implementations may therefore enforce operational controls over:

- compilation time;
- memory consumption;
- generated-node expansion;
- recursion depth;
- parser workload;
- verification workload;
- elaboration workload.

These are implementation or policy controls.

They are not universal HDL semantic maxima.

Diagnostics must distinguish:

compiler resource exhausted

from:

program violates language semantics

---

67. Rust Integration

The implementation baseline is:

Rust 1.97 or later
Rust 2021
safe Rust

Zamani-owned Rust implementation must contain no "unsafe" code.

The grammar itself must contain:

- no embedded Rust actions;
- no target-specific code;
- no filesystem operations;
- no network operations;
- no hardware discovery;
- no runtime execution.

The ANTLR grammar remains declarative.

Rust owns implementation behavior downstream.

---

68. Rust Ownership and Scalability

Compiler implementations must use scalable safe-Rust data structures.

They must avoid unnecessary:

- recursive runtime structures;
- uncontrolled cloning;
- global mutable state;
- target-dependent parser behavior;
- hidden static capacity assumptions.

Large HDL designs should be representable through:

- symbolic structures;
- indexed collections;
- arenas or equivalent ownership-safe structures where appropriate;
- references/IDs with explicit ownership;
- incremental processing;
- deterministic traversal;
- resource-aware elaboration.

The implementation must remain safe while supporting very large designs.

---

69. No Vendor Lock-In

Core HDL syntax must not silently depend on one vendor.

The universal grammar must not require:

- one FPGA family;
- one ASIC flow;
- one CPU;
- one GPU;
- one accelerator;
- one board;
- one package;
- one QPU;
- one synthesis vendor.

Vendor-specific constructs require explicit dialect or interoperability ownership.

---

70. Target Independence

Portable HDL should normally avoid identifying a physical target.

The distinction is:

portable intent
      ↓
requirements
      ↓
capabilities
      ↓
constraints
      ↓
preferences
      ↓
compiler realization

Explicit target-specific intent may exist, but it must be recognizable as target-specific.

---

71. Target Capability Failure

If a target cannot satisfy:

- required capability;
- required resource;
- required timing;
- required width;
- required precision;
- required protocol;
- required reliability;
- required memory;
- required topology;

the compiler must produce a structured failure.

It must not silently change program semantics.

Possible outcomes include:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

where those outcomes are defined by the repository-wide execution/resilience architecture.

---

72. Simulation Boundary

Simulation is an execution mode.

The conceptual path is:

HDL source
    ↓
HDL semantic model
    ↓
simulation planning
    ↓
simulation backend

Simulation must be able to participate in:

- hardware validation;
- classical/hardware co-design;
- quantum/hardware systems;
- fault analysis;
- timing analysis;
- verification.

Simulation does not become a separate source language.

---

73. Verification Boundary

Verification is a semantic consumer of HDL intent.

The flow is:

HDL properties
     ↓
domain-neutral AST
     ↓
verification semantics
     ↓
verification engine

The grammar does not execute assertions.

---

74. Synthesis Boundary

Synthesis consumes hardware semantic intent.

The flow is:

HDL source
    ↓
semantic hardware model
    ↓
synthesis analysis
    ↓
optimization
    ↓
technology-independent representation
    ↓
target lowering

Synthesis may choose:

- gates;
- registers;
- memories;
- arithmetic resources;
- DSP-like structures;
- vendor primitives;
- custom structures.

Those choices are not core grammar concepts.

---

75. Scheduling Boundary

HDL can express timing and ordering intent.

Scheduling decides a realizable implementation schedule.

The grammar must not implement the scheduler.

---

76. Placement Boundary

Placement maps logical implementation structures to physical resources.

The grammar does not perform placement.

---

77. Routing Boundary

Routing maps logical connectivity to physical interconnect.

The grammar does not perform routing.

---

78. HAL Boundary

The Hardware Abstraction Layer is downstream.

The relationship is:

Zamani HDL
    ↓
HDL semantic model
    ↓
compiler
    ↓
lowering
    ↓
HAL
    ↓
target

The grammar must never directly construct HAL objects.

---

79. Interoperability

External HDL formats must integrate through:

external format
       ↓
interoperability adapter
       ↓
Zamani semantic model
       ↓
canonical Zamani representation

They must not create:

external format
       ↓
competing Zamani AST
       ↓
competing Zamani IR

Relevant interoperability ownership remains under:

grammar/interoperability/

---

80. Dialect Contract

Every HDL dialect must define:

dialect identity
namespace
version
compatibility
lexer requirements
grammar extension
AST mapping
semantic mapping
resource mapping
capability mapping
effect mapping
policy mapping
provenance mapping
IR mapping
tests

A dialect cannot silently alter the meaning of portable HDL.

---

81. File Completion Contract

A file under "grammar/hdl/" is DONE only when all applicable integration information is known in advance.

The file must have:

Purpose
Owner
Non-ownership
Dependencies
Public rules
Consumers
AST contract
Source-span contract
Type contract
Semantic contract
Effect contract
Capability contract
Resource contract
Contract contract
Policy contract
Provenance contract
IR contract
Diagnostics
Compatibility
Positive tests
Negative tests
Boundary tests
Scalability tests
Determinism tests
Cross-domain tests
Hard-coding audit
Completion criteria

A later file must not require reopening a completed file merely to discover what its integration should have been.

A completed file may be changed only when:

1. its own contract changes;
2. the language specification changes;
3. a deliberate compatibility change occurs;
4. a deliberate ownership boundary changes;
5. a new public integration point is introduced.

---

82. Independent-First Implementation Order

The HDL subsystem should be implemented in dependency order.

Stage 1 — Directory contract

Complete:

grammar/hdl/README.md

This file establishes the contracts used by every other HDL file.

---

Stage 2 — Shared foundations

Ensure integration with:

grammar/lexer/
grammar/core/
grammar/types/
grammar/expressions/
grammar/declarations/
grammar/statements/
grammar/resources/
grammar/effects/
grammar/validation/
grammar/policies/

No HDL leaf grammar should duplicate these systems.

---

Stage 3 — Parameterization

Complete:

parameters.g4
hardware-generics.g4

These provide scalable symbolic foundations.

---

Stage 4 — Structural foundations

Complete:

hardware-modules.g4
interfaces.g4
ports.g4
signals.g4
nets.g4
wires.g4
arrays.g4

---

Stage 5 — State and storage

Complete:

registers.g4
memories.g4
clocks.g4
clocking.g4
reset.g4

---

Stage 6 — Behavior

Complete:

processes.g4
combinational.g4
sequential.g4
state_machines.g4
pipelines.g4
parallelism.g4
generate.g4

---

Stage 7 — Communication and timing

Complete:

protocols.g4
timing.g4

---

Stage 8 — Correctness and execution

Complete:

assertions.g4
verification.g4
simulation.g4
synthesis.g4

---

Stage 9 — Co-design and physical intent

Complete:

co-design.g4
hardware-software-integration.g4
physical-intent.g4
hardware-dialects.g4

---

Stage 10 — Composition

Complete:

hdl.g4

Only after every imported public rule and ownership boundary has been validated.

---

Stage 11 — Repository integration

Validate:

grammar/Zamani.g4
frontend AST
semantic analysis
resources
effects
validation
policies
provenance
compiler
Classical IR
quantum::ir
execution
hardware
interoperability
HAL

---

83. Required Integration Contract for Each File

"arrays.g4"

OWNER:
Hardware array syntax

DEPENDS_ON:
types
expressions
parameters
hardware-generics

CONSUMED_BY:
hardware-modules
memories
signals
hdl.g4

SEMANTIC:
array shape, extent, indexing, dimensional validity

IR:
compiler-owned lowering

SCALABILITY:
symbolic and program-defined dimensions

DONE:
No universal array cardinality.

---

"assertions.g4"

OWNER:
HDL assertions

DEPENDS_ON:
expressions
validation
contracts

CONSUMED_BY:
verification
hdl.g4

SEMANTIC:
property evaluation and source provenance

IR:
verification/compiler representation

DONE:
No execution engine embedded in grammar.

---

"clocking.g4"

OWNER:
Clocking relationships

DEPENDS_ON:
clocks
expressions
timing

CONSUMED_BY:
sequential
processes
timing
hdl.g4

SEMANTIC:
clock relationships and synchronization intent

DONE:
No physical clock-tree implementation.

---

"clocks.g4"

OWNER:
Clock declarations

DEPENDS_ON:
types
expressions
parameters

CONSUMED_BY:
clocking
sequential
processes
timing
reset

SEMANTIC:
clock identity and timing intent

DONE:
No target-specific clock resources.

---

"co-design.g4"

OWNER:
Hardware/software co-design syntax

DEPENDS_ON:
modules
interfaces
ports
expressions
resources
effects

CONSUMED_BY:
hdl.g4
hardware-software-integration.g4

SEMANTIC:
software/hardware relationship

IR:
compiler-owned hybrid lowering

DONE:
No second programming language.

---

"combinational.g4"

OWNER:
Combinational behavior

DEPENDS_ON:
processes
expressions
types

CONSUMED_BY:
hdl.g4

SEMANTIC:
combinational completeness and dependencies

DONE:
No physical gate selection.

---

"generate.g4"

OWNER:
Parameterized/generated hardware structure

DEPENDS_ON:
parameters
hardware-generics
modules
expressions

CONSUMED_BY:
hardware-modules
hdl.g4

SEMANTIC:
deterministic elaboration

SCALABILITY:
symbolic generation with implementation resource controls

DONE:
No fixed generated-instance maximum.

---

"hardware-dialects.g4"

OWNER:
HDL dialect extensions

DEPENDS_ON:
core names
attributes
modules
interoperability

CONSUMED_BY:
hdl.g4

SEMANTIC:
versioned/namespaced dialect resolution

DONE:
No dialect may silently redefine core HDL semantics.

---

"hardware-generics.g4"

OWNER:
Generic hardware structure

DEPENDS_ON:
types
parameters
constraints

CONSUMED_BY:
hardware-modules
generate
hdl.g4

SEMANTIC:
generic binding and specialization

DONE:
No target capacity encoded as generic-system limits.

---

"hardware-modules.g4"

OWNER:
Hardware module declarations and instances

DEPENDS_ON:
generics
parameters
interfaces
ports
signals
nets
registers
memories
processes
pipelines
generate

CONSUMED_BY:
hdl.g4
co-design

SEMANTIC:
module identity, scope, composition

DONE:
Module count is unrestricted by language-level constants.

---

"hardware-software-integration.g4"

OWNER:
Hardware/software boundaries

DEPENDS_ON:
co-design
interfaces
effects
resources
interoperability

CONSUMED_BY:
hdl.g4

SEMANTIC:
data/control boundary and integration intent

DONE:
No duplication of FFI/ABI semantics.

---

"interfaces.g4"

OWNER:
Hardware interface contracts

DEPENDS_ON:
ports
signals
protocols
types

CONSUMED_BY:
modules
co-design
hdl.g4

SEMANTIC:
interface compatibility

DONE:
No physical bus assumed by default.

---

"memories.g4"

OWNER:
Memory declarations

DEPENDS_ON:
types
arrays
parameters
clocking

CONSUMED_BY:
modules
co-design
hdl.g4

SEMANTIC:
memory shape, access behavior, latency/throughput intent

DONE:
No fixed memory capacity.

---

"nets.g4"

OWNER:
Canonical logical connectivity

DEPENDS_ON:
types
names
expressions

CONSUMED_BY:
modules
wires
signals
hdl.g4

SEMANTIC:
drivers, connectivity, resolution

DONE:
One canonical logical connectivity model.

---

"parallelism.g4"

OWNER:
HDL parallel structure

DEPENDS_ON:
modules
generate
resources
concurrency

CONSUMED_BY:
hdl.g4

SEMANTIC:
parallel execution/structure intent

DONE:
No fixed lane/unit count.

---

"parameters.g4"

OWNER:
HDL parameter declarations and references

DEPENDS_ON:
types
expressions
generics

CONSUMED_BY:
modules
generate
memories
pipelines
arrays
hdl.g4

SEMANTIC:
symbolic/program-level configuration

DONE:
Parameter values are never converted into universal language limits.

---

"physical-intent.g4"

OWNER:
Explicit physical implementation intent

DEPENDS_ON:
hardware
resources
capabilities
policies

CONSUMED_BY:
hdl.g4

SEMANTIC:
explicit physical constraints/preferences

DONE:
Physical intent remains distinguishable from portable intent.

---

"pipelines.g4"

OWNER:
Pipeline intent

DEPENDS_ON:
modules
parameters
timing
parallelism

CONSUMED_BY:
hdl.g4

SEMANTIC:
stage structure, latency, throughput

DONE:
No fixed pipeline-depth ceiling.

---

"ports.g4"

OWNER:
Port declarations

DEPENDS_ON:
types
expressions
interfaces

CONSUMED_BY:
modules
interfaces
hdl.g4

SEMANTIC:
direction, shape, type, protocol

DONE:
No universal port count.

---

"processes.g4"

OWNER:
Hardware processes

DEPENDS_ON:
expressions
blocks
clocks
reset

CONSUMED_BY:
combinational
sequential
modules
hdl.g4

SEMANTIC:
process classification and behavior

DONE:
No software-thread semantics accidentally introduced.

---

"protocols.g4"

OWNER:
Hardware protocol intent

DEPENDS_ON:
interfaces
ports
signals
timing

CONSUMED_BY:
interfaces
hdl.g4

SEMANTIC:
protocol state, ordering, handshake, timing

DONE:
Vendor protocols remain explicit dialects/interoperability.

---

"registers.g4"

OWNER:
Register declarations

DEPENDS_ON:
types
clocking
reset
parameters

CONSUMED_BY:
modules
sequential
hdl.g4

SEMANTIC:
logical state

DONE:
No fixed register width.

---

"reset.g4"

OWNER:
Reset declarations and source intent

DEPENDS_ON:
clocks
clocking
expressions

CONSUMED_BY:
registers
sequential
processes
hdl.g4

SEMANTIC:
reset relationship and legality

DONE:
No physical reset network implementation.

---

"sequential.g4"

OWNER:
Sequential behavior

DEPENDS_ON:
processes
registers
clocks
reset
expressions

CONSUMED_BY:
hdl.g4

SEMANTIC:
state transition behavior

DONE:
No physical flip-flop selection.

---

"signals.g4"

OWNER:
Logical signals

DEPENDS_ON:
types
expressions
nets

CONSUMED_BY:
modules
processes
interfaces
hdl.g4

SEMANTIC:
signal type and driver relationships

DONE:
No physical wire assumption.

---

"simulation.g4"

OWNER:
Simulation intent

DEPENDS_ON:
verification
processes
timing
expressions

CONSUMED_BY:
hdl.g4

SEMANTIC:
simulation configuration and intent

DONE:
No simulator-specific core language dependency.

---

"state_machines.g4"

OWNER:
State-machine syntax

DEPENDS_ON:
types
expressions
processes
clocking

CONSUMED_BY:
hdl.g4

SEMANTIC:
states, transitions, guards, actions

DONE:
No state-count ceiling.

---

"synthesis.g4"

OWNER:
Synthesis intent

DEPENDS_ON:
modules
timing
resources
constraints
policies

CONSUMED_BY:
hdl.g4

SEMANTIC:
synthesis requirements and preferences

DONE:
No synthesis implementation embedded in grammar.

---

"timing.g4"

OWNER:
Timing intent

DEPENDS_ON:
clocks
clocking
expressions
constraints

CONSUMED_BY:
processes
pipelines
protocols
synthesis
hdl.g4

SEMANTIC:
timing requirements and relationships

DONE:
No physical timing closure in grammar.

---

"verification.g4"

OWNER:
Verification intent

DEPENDS_ON:
assertions
contracts
properties
expressions

CONSUMED_BY:
hdl.g4

SEMANTIC:
verification properties and scope

DONE:
No verification engine embedded in grammar.

---

"wires.g4"

OWNER:
Wire-specific surface syntax

DEPENDS_ON:
nets

CONSUMED_BY:
modules
hdl.g4

SEMANTIC:
normalize into canonical connectivity

DONE:
No competing net model.

---

84. AST Contract

Every HDL construct must ultimately become part of the domain-neutral frontend AST.

The frontend AST must preserve:

- source span;
- identifiers;
- qualified names;
- generic arguments;
- parameter expressions;
- types;
- attributes;
- modifiers;
- declarations;
- relationships;
- source provenance.

The frontend AST must not contain backend-specific objects such as:

- FPGA LUT;
- physical BRAM;
- physical DSP;
- routing track;
- ASIC metal layer;
- package pin;
- QPU physical qubit;
- vendor primitive instance;
- bitstream;
- calibration record.

Those belong downstream.

---

85. Semantic Contract

After parsing, semantic analysis owns:

- name resolution;
- scope resolution;
- generic binding;
- parameter evaluation;
- type checking;
- width checking;
- shape checking;
- driver analysis;
- connectivity analysis;
- clock analysis;
- reset analysis;
- timing analysis;
- process classification;
- state-machine validation;
- pipeline validation;
- protocol validation;
- resource analysis;
- capability analysis;
- effect analysis;
- contract validation;
- policy validation;
- provenance construction;
- synthesis eligibility;
- simulation eligibility;
- target-independent optimization eligibility.

---

86. Hardware Semantic Model

The hardware semantic model represents what the program means as hardware.

It must remain independent of:

- current hardware inventory;
- vendor implementation;
- physical placement;
- routing;
- calibration;
- fabrication technology.

It is the bridge between source syntax and compiler realization.

---

87. Physical Realization Separation

The separation is mandatory:

WHAT
    ↓
Zamani source

WHY
    ↓
contracts / requirements

WHAT IS ALLOWED
    ↓
policies / capabilities

WHAT IS AVAILABLE
    ↓
resource environment

HOW
    ↓
optimization / synthesis

WHEN
    ↓
scheduling / timing

WHERE
    ↓
placement / deployment

HOW CONNECTED
    ↓
routing

WHICH DEVICE
    ↓
target realization

No grammar file may collapse these layers.

---

88. Hardware/AI Integration

AI semantics remain owned by "grammar/ai/".

HDL can describe hardware realization of:

- tensor computation;
- inference;
- learning accelerators;
- neural-symbolic computation;
- dataflow;
- accelerator pipelines.

The HDL grammar must not turn every AI operation into a keyword.

AI operations remain semantic operations and libraries.

---

89. Hardware/Data Integration

HDL may represent data movement and hardware computation over:

- arrays;
- tensors;
- matrices;
- streams;
- records;
- buffers;
- memories.

The data subsystem remains the owner of universal data semantics.

HDL describes how those semantics participate in hardware realization.

---

90. Hardware/Distributed Integration

Hardware may participate in distributed computation.

The architecture remains:

distributed semantics
       +
hardware realization

not:

HDL creates another distributed language

Node counts, device counts, and topology sizes remain open-ended.

---

91. Hardware/Networking Integration

Networking semantics remain owned by "grammar/networking/".

HDL may describe:

- network interfaces;
- packet-processing hardware;
- streaming hardware;
- network accelerators;
- protocol implementation.

Network topology and physical deployment remain downstream.

---

92. Hardware/Concurrency Integration

HDL parallelism must integrate with the existing concurrency architecture.

There must not be two incompatible meanings of:

- actor;
- task;
- channel;
- synchronization;
- parallelism.

Hardware-specific constructs may map to the common semantic model.

---

93. Hardware/Metaprogramming Integration

Compile-time generation may be used to create hardware structures.

It must integrate with:

grammar/metaprogramming/
grammar/macros/
grammar/compile/

Generated hardware must preserve:

- provenance;
- deterministic behavior;
- type information;
- contracts;
- policies;
- resource requirements.

---

94. Compile-Time Generation Safety

Compile-time generation must not silently:

- inspect arbitrary external state;
- access the network;
- query hardware;
- depend on wall-clock time;
- depend on uncontrolled randomness.

If such behavior is explicitly supported elsewhere, its effects and capabilities must be declared and checked.

---

95. Compatibility

Compatibility is owned by:

grammar/compatibility/

HDL files must not accumulate undocumented historical syntax indefinitely.

Legacy HDL syntax must be:

- identified;
- versioned;
- classified;
- mapped to canonical semantics;
- tested;
- deprecated where appropriate.

---

96. Error Classification

Errors must be classified correctly.

Lexical error

Invalid tokenization.

Syntax error

Invalid grammar structure.

Semantic error

Valid syntax with invalid meaning.

Type error

Invalid type relationship.

Resource error

Required resource unavailable.

Capability error

Required capability unavailable.

Contract error

Required property not satisfied.

Policy error

Construct prohibited by active policy.

Compatibility error

Unsupported language/version transition.

Backend error

Valid source cannot be realized by the selected target.

These must not be collapsed into generic parser errors.

---

97. Deterministic Diagnostics

Diagnostics should preserve:

- source span;
- construct identity;
- semantic category;
- relevant requirement;
- actual observed condition;
- suggested resolution where available;
- provenance.

The parser must not access external target state to generate ordinary syntax diagnostics.

---

98. Scalability Model

The HDL architecture must scale in two independent dimensions.

98.1 Semantic scale

The language supports arbitrary program-defined:

- widths;
- depths;
- dimensions;
- modules;
- instances;
- states;
- transitions;
- pipeline stages;
- memories;
- clocks;
- channels;
- generated structures.

98.2 Physical scale

The same semantic program can be considered for:

tiny embedded target
        ↓
single CPU
        ↓
multicore
        ↓
GPU
        ↓
FPGA
        ↓
ASIC
        ↓
accelerator
        ↓
heterogeneous system
        ↓
HPC
        ↓
cluster
        ↓
distributed system
        ↓
future target

Physical feasibility is evaluated downstream.

---

99. Scalability Test Contract

Every major HDL construct must be tested at multiple semantic scales.

The tests should include:

minimal
small
parameterized
large
symbolically large
cross-domain
resource constrained
capability constrained

The purpose is to verify that no hidden grammar capacity has appeared.

---

100. Hard-Coding Audit

The complete HDL directory must be audited for:

MAX_*
fixed widths
fixed depths
fixed stage counts
fixed module counts
fixed instance counts
fixed port counts
fixed state counts
fixed memory sizes
fixed device counts
fixed topology sizes
fixed vendor assumptions
fixed hardware inventory

The audit must inspect:

*.g4
*.md
test fixtures
generated grammar
semantic contracts
validation rules

Legitimate program constants are allowed.

Universal implementation limits disguised as grammar rules are not.

---

101. Correct Portable Intent

Examples of portable intent include:

requires capability("hardware.pipeline");

requires capability("streaming");

requires memory >= required_memory;

prefer capability("parallel.compute");

constrain latency <= required_latency;

These express semantic requirements.

They do not identify a physical device.

---

102. Incorrect Portable Architecture

The core grammar must not require source such as:

use FPGA0;
use GPU0;
use QPU1;
use BRAM3;
use DSP7;
use LUT42;

unless an explicit target-specific dialect is being used.

Portable programs should not need physical inventory knowledge.

---

103. Target-Specific Architecture

Explicit target-specific source is allowed only through an explicit boundary.

The semantic path is:

portable HDL
     +
explicit target intent
     ↓
target-aware semantic validation
     ↓
target lowering

Target-specific intent must never silently contaminate the portable core.

---

104. Reproducibility

A production HDL compilation should be reproducible from:

- source;
- language version;
- grammar version;
- dialect versions;
- declared compiler configuration;
- declared policies;
- declared target information;
- declared resource information.

Uncontrolled external state must not silently change parsing semantics.

---

105. Optimization Contract

HDL semantics may be optimized through:

- constant propagation;
- dead-logic elimination;
- common-subexpression elimination;
- resource sharing;
- replication;
- retiming;
- pipelining;
- memory transformations;
- parallelization;
- specialization.

Optimization must preserve:

- type semantics;
- timing contracts where exact;
- functional behavior;
- declared properties;
- required effects;
- resource constraints;
- policies;
- provenance.

---

106. Approximation Contract

Approximation is permitted only when explicitly represented by source semantics, contracts, or policy.

For example:

prefer error <= allowed_error;

may permit an approximation-aware implementation if the language specification defines the construct.

Exact source semantics must not be silently weakened.

---

107. Hardware Resource Abstraction

Physical resources are discovered downstream.

Examples include:

logic
storage
memory
compute
bandwidth
clock resources
interconnect
thermal budget
power budget
accelerator capacity
quantum control resources

The grammar expresses requirements and constraints.

The compiler resolves realization.

---

108. Hardware Capability Abstraction

Capabilities describe semantic abilities rather than inventories.

For example:

capability("hardware.pipeline")

is fundamentally different from:

physical_pipeline_unit_7

The former supports POCO-REAF.

The latter is target-specific realization information.

---

109. Future Hardware

The core grammar must remain capable of representing hardware not yet known when the grammar was written.

This requires:

- open identifiers;
- generic modules;
- parameterization;
- dialects;
- capabilities;
- resources;
- policies;
- target-independent semantics.

The language must not require a new core grammar rule for every future device class.

---

110. No Enumeration of the Hardware Universe

The grammar must not attempt to enumerate every:

- CPU;
- GPU;
- FPGA;
- ASIC;
- accelerator;
- DSP;
- memory technology;
- interconnect;
- board;
- package;
- quantum processor;
- future device.

The language describes capabilities and intent.

Target descriptions provide realization knowledge.

---

111. Production Test Matrix

The HDL subsystem must have tests covering:

lexical
parser
AST
semantic
type
effects
resources
capabilities
contracts
policies
provenance
simulation
verification
synthesis intent
co-design
classical integration
quantum integration
parallelism
distributed integration
interoperability
dialects
compatibility
scalability
determinism
negative cases
boundary cases

---

112. Required HDL Integration Programs

At minimum, repository-level examples should cover:

minimal.zm
classical.zm
hdl.zm
hybrid.zm
poco-reaf.zm

and progressively include:

generic hardware
parameterized memory
parameterized pipeline
generated hardware
state machine
multi-clock design
hardware/software co-design
hardware accelerator
simulation
verification
synthesis intent
resource requirements
capability requirements
policy-constrained hardware
quantum-control hardware
classical/quantum hybrid hardware
dialect usage
large symbolic design

---

113. Cross-Domain Integration Test

At least one production conformance test must combine:

classical computation
+
parameterized hardware
+
memory
+
pipeline
+
parallelism
+
resource requirements
+
capability requirements
+
contracts
+
policies
+
provenance
+
simulation
+
verification
+
quantum interaction
+
hardware/software co-design

The expected pipeline is:

source
 ↓
lexer
 ↓
parser
 ↓
domain-neutral AST
 ↓
structural validation
 ↓
type analysis
 ↓
effect analysis
 ↓
resource analysis
 ↓
capability analysis
 ↓
contract analysis
 ↓
policy analysis
 ↓
provenance
 ↓
semantic hardware model
 ↓
Classical IR / quantum::ir where applicable
 ↓
optimization
 ↓
lowering
 ↓
scheduling
 ↓
routing
 ↓
resilience
 ↓
ZQN where applicable
 ↓
HAL
 ↓
target

---

114. HDL and POCO-REAF Acceptance Test

A production POCO-REAF test should compile the same semantic program against multiple target descriptions without changing its source-level meaning.

The target environments may differ in:

- memory;
- compute capacity;
- parallelism;
- accelerator availability;
- hardware capabilities;
- topology;
- timing;
- power;
- thermal limits;
- quantum resources.

The source program remains stable.

The compiler may choose different implementations.

---

115. Target Failure Test

A target that cannot satisfy:

requirement
capability
constraint
contract
policy
timing
resource

must produce a structured diagnostic.

The compiler must not silently:

- reduce a width;
- remove an operation;
- remove a pipeline stage;
- change a protocol;
- change timing semantics;
- replace exact computation with approximation;
- select an unrelated device.

---

116. Determinism Test

For identical:

source
grammar version
language version
dialect configuration
compiler configuration
declared target information
declared resource information
declared policy

the grammar and deterministic frontend must produce equivalent results.

No hidden environmental dependency is permitted.

---

117. Repository-Wide Integration

The HDL subsystem must integrate with:

grammar/Zamani.g4
grammar/DESIGN.md
grammar/specification/
grammar/spec/
grammar/lexer/
grammar/antlr/
grammar/core/
grammar/types/
grammar/expressions/
grammar/statements/
grammar/declarations/
grammar/functions/
grammar/modules/
grammar/resources/
grammar/effects/
grammar/validation/
grammar/policies/
grammar/security/
grammar/execution/
grammar/classical/
grammar/quantum/
grammar/hybrid/
grammar/ai/
grammar/data/
grammar/concurrency/
grammar/distributed/
grammar/networking/
grammar/hardware/
grammar/interoperability/
grammar/dialects/
grammar/metaprogramming/
grammar/compile/
grammar/compatibility/

The HDL directory must consume those systems rather than reproduce them.

---

118. AST Integration Contract

The HDL subsystem must not require a separate:

HDLAST
HardwareAST
VendorHDLAST
QuantumHardwareAST

as competing frontend ASTs.

The frontend AST remains domain-neutral.

Semantic specialization occurs after parsing.

---

119. Semantic Ownership Contract

The semantic ownership is:

grammar/hdl/
       ↓
HDL syntax
       ↓
frontend AST
       ↓
hardware semantic analysis
       ↓
shared semantic systems
       ├── types
       ├── effects
       ├── resources
       ├── capabilities
       ├── contracts
       ├── policies
       └── provenance

The HDL grammar must never attempt to perform these analyses itself.

---

120. IR Ownership Contract

The HDL grammar does not create IR.

The compiler owns IR lowering.

Applicable representations include:

Classical IR
quantum::ir
compiler-owned hardware/target representations

where those are explicitly defined by the compiler architecture.

The README must not create a new universal IR authority merely to make HDL appear self-contained.

---

121. Runtime Ownership

Runtime behavior belongs outside the grammar.

The grammar does not:

- execute processes;
- execute simulation;
- schedule hardware;
- allocate devices;
- allocate memory;
- allocate qubits;
- discover devices;
- route networks;
- control clocks;
- perform QEC.

It expresses the source intent required by those downstream systems.

---

122. Backend Ownership

Backends own:

- target selection;
- lowering;
- physical mapping;
- placement;
- routing;
- scheduling;
- synthesis;
- target-specific optimization;
- code generation;
- bitstream generation;
- deployment.

The HDL grammar remains target-independent.

---

123. HAL Ownership

HAL owns target realization.

The HAL may know:

- physical device topology;
- physical resource inventory;
- target capabilities;
- device identifiers;
- implementation details.

The portable HDL grammar must not require those details.

---

124. Definition of Production Readiness

"grammar/hdl/" is production-ready only when all of the following are true.

Architecture

- [ ] "grammar/Zamani.g4" remains the canonical language composition root.
- [ ] "grammar/hdl/hdl.g4" is the only HDL composition root.
- [ ] "grammar/hdl/README.md" is the directory orchestration contract.
- [ ] Every HDL feature has exactly one canonical grammar owner.
- [ ] No duplicate lexer exists.
- [ ] No duplicate AST exists.
- [ ] No competing universal IR exists.
- [ ] No import cycle exists.
- [ ] No orphan HDL grammar exists.

Lexer

- [ ] All tokens come from the canonical lexer.
- [ ] HDL does not define lexer rules.
- [ ] Operation names remain extensible identifiers unless language-level reservation is justified.
- [ ] Vendor names do not become accidental core keywords.

Syntax

- [ ] Every HDL construct has an owner.
- [ ] Public grammar rules are explicitly documented.
- [ ] Cross-file dependencies are documented.
- [ ] Ambiguous ownership is resolved.
- [ ] Parser entry points are documented.
- [ ] EOF ownership is unambiguous.

AST

- [ ] Every construct has an AST mapping.
- [ ] Source spans are preserved.
- [ ] AST remains domain-neutral.
- [ ] No physical implementation objects leak into the frontend AST.

Semantics

- [ ] Module semantics are defined.
- [ ] Generic semantics are defined.
- [ ] Parameter semantics are defined.
- [ ] Port semantics are defined.
- [ ] Interface semantics are defined.
- [ ] Signal semantics are defined.
- [ ] Net semantics are defined.
- [ ] Wire semantics are normalized.
- [ ] Register semantics are defined.
- [ ] Memory semantics are defined.
- [ ] Clock semantics are defined.
- [ ] Reset semantics are defined.
- [ ] Process semantics are defined.
- [ ] Combinational semantics are defined.
- [ ] Sequential semantics are defined.
- [ ] State-machine semantics are defined.
- [ ] Pipeline semantics are defined.
- [ ] Parallelism semantics are defined.
- [ ] Generation semantics are defined.
- [ ] Protocol semantics are defined.
- [ ] Timing semantics are defined.
- [ ] Verification semantics are defined.
- [ ] Simulation semantics are defined.
- [ ] Synthesis intent is defined.
- [ ] Physical intent is defined.
- [ ] Co-design semantics are defined.

Universal semantic systems

- [ ] Types integrate with the canonical type system.
- [ ] Effects integrate with the canonical effect system.
- [ ] Resources integrate with the canonical resource system.
- [ ] Capabilities integrate with the canonical capability system.
- [ ] Contracts integrate with the canonical contract system.
- [ ] Policies integrate with the canonical policy system.
- [ ] Provenance integrates with the canonical provenance system.

Portability

- [ ] Source remains target-independent by default.
- [ ] Generic hardware is supported.
- [ ] Parameterized hardware is supported.
- [ ] Future hardware can be represented without core grammar redesign.
- [ ] Resource requirements remain symbolic where appropriate.
- [ ] Capability requirements remain open-world.
- [ ] Physical realization remains downstream.

Scalability

- [ ] No universal hardware capacity is hard-coded.
- [ ] No hidden cardinality ceiling exists.
- [ ] Large symbolic designs are representable.
- [ ] Generated designs are resource-bounded by implementation policy rather than language semantics.
- [ ] Safe Rust implementations support scalable representations.
- [ ] Deterministic processing is preserved.

Quantum

- [ ] HDL integrates with hybrid quantum systems.
- [ ] Quantum syntax remains owned by "grammar/quantum/".
- [ ] "quantum::ir" remains the canonical quantum IR.
- [ ] HDL does not implement QEC.
- [ ] HDL does not implement quantum routing.
- [ ] HDL does not enumerate quantum operations.

Compiler

- [ ] AST integration is defined.
- [ ] Semantic integration is defined.
- [ ] Resource analysis integration is defined.
- [ ] Capability analysis integration is defined.
- [ ] Effect analysis integration is defined.
- [ ] Contract integration is defined.
- [ ] Policy integration is defined.
- [ ] Provenance integration is defined.
- [ ] Classical IR integration is defined.
- [ ] "quantum::ir" integration is defined.
- [ ] Optimization integration is defined.
- [ ] Lowering integration is defined.
- [ ] Scheduling integration is defined.
- [ ] Placement integration is defined.
- [ ] Routing integration is defined.
- [ ] Synthesis integration is defined.
- [ ] HAL integration is defined.

Rust

- [ ] Rust 1.97 or later is supported.
- [ ] Rust 2021 is used.
- [ ] Zamani-owned Rust contains no "unsafe".
- [ ] Grammar files contain no embedded Rust actions.
- [ ] Diagnostics preserve source provenance.
- [ ] Deterministic behavior is tested.

Testing

- [ ] Positive tests exist.
- [ ] Negative tests exist.
- [ ] Boundary tests exist.
- [ ] Scalability tests exist.
- [ ] Determinism tests exist.
- [ ] Compatibility tests exist.
- [ ] Cross-domain tests exist.
- [ ] Hard-coding audit passes.
- [ ] Repository-wide grammar conformance passes.

---

125. What Must Never Happen

Never:

1. Create a second "Zamani.g4".
2. Create a second HDL composition root.
3. Create an HDL-specific lexer.
4. Create a competing frontend AST.
5. Create a competing quantum IR.
6. Enumerate every FPGA primitive in core HDL.
7. Enumerate every ASIC primitive in core HDL.
8. Enumerate every CPU instruction in HDL.
9. Enumerate every GPU instruction in HDL.
10. Enumerate every quantum operation in HDL.
11. Hard-code today's hardware capacity.
12. Hard-code register widths.
13. Hard-code memory sizes.
14. Hard-code pipeline depth.
15. Hard-code module counts.
16. Hard-code instance counts.
17. Hard-code port counts.
18. Hard-code device counts.
19. Hard-code node counts.
20. Hard-code topology sizes.
21. Make a vendor API core language syntax.
22. Make a HAL object a parser construct.
23. Perform placement in the grammar.
24. Perform routing in the grammar.
25. Perform scheduling in the grammar.
26. Perform synthesis in the grammar.
27. Perform QEC in the grammar.
28. Perform target discovery in the grammar.
29. Perform hardware discovery in the grammar.
30. Perform runtime execution in the grammar.
31. Make simulation a second language.
32. Make verification a second language.
33. Make synthesis a second language.
34. Make hardware/software co-design a second language.
35. Duplicate the resource system.
36. Duplicate the capability system.
37. Duplicate the effect system.
38. Duplicate the policy system.
39. Duplicate the provenance system.
40. Let historical documentation silently introduce new syntax.

---

126. The One-Owner Rule

For every semantic concept:

ONE CONCEPT
     ↓
ONE CANONICAL OWNER
     ↓
ZERO COMPETING DEFINITIONS

Composition files may re-export or dispatch.

They must not redefine.

Examples:

net connectivity
    → nets.g4

wire surface compatibility
    → wires.g4

module declaration
    → hardware-modules.g4

pipeline
    → pipelines.g4

timing
    → timing.g4

assertion
    → assertions.g4

verification
    → verification.g4

---

127. The No-Rework Rule

Before a file is declared complete, its integration contract must already specify:

who consumes it
what it consumes
what it exports
what AST it maps to
what semantic model consumes it
what types it requires
what effects it produces
what capabilities it requires
what resources it requires
what contracts affect it
what policies affect it
what provenance it produces
what compiler representation consumes it
what diagnostics it produces
what tests prove it

Therefore:

«Implementing another HDL file later must not require reopening a completed file merely to discover its missing integration contract.»

This is the required independent-file-first development model.

---

128. Final HDL Architecture

The final architecture is:

                         ZAMANI SOURCE
                              │
                              ▼
                     grammar/Zamani.g4
                              │
                              ▼
                   grammar/antlr/ZamaniLexer.g4
                              │
                              ▼
                         HDL dispatch
                              │
                              ▼
                      grammar/hdl/hdl.g4
                              │
             ┌────────────────┼────────────────┐
             │                │                │
             ▼                ▼                ▼
         structure         behavior          intent
             │                │                │
             └────────────────┼────────────────┘
                              │
                              ▼
                   Domain-Neutral AST
                              │
                              ▼
                  Structural Validation
                              │
       ┌──────────────┬───────┼───────┬──────────────┐
       │              │       │       │              │
       ▼              ▼       ▼       ▼              ▼
     Types         Effects Resources Contracts     Policies
       │              │       │       │              │
       └──────────────┴───────┼───────┴──────────────┘
                              │
                              ▼
                         Provenance
                              │
                              ▼
                  Hardware Semantic Model
                              │
          ┌───────────────────┼───────────────────┐
          │                   │                   │
          ▼                   ▼                   ▼
      Classical            Quantum             Hardware
      semantics            semantics           semantics
          │                   │                   │
          │                   ▼                   │
          │              quantum::ir             │
          │                   │                   │
          └───────────────────┼───────────────────┘
                              │
                              ▼
                         Optimization
                              │
                              ▼
                           Lowering
                              │
                              ▼
                    Synthesis / Elaboration
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
                    Resilience / Recovery
                              │
                              ▼
                             ZQN
                              │
                              ▼
                             HAL
                              │
          ┌───────────────────┼────────────────────┐
          │                   │                    │
          ▼                   ▼                    ▼
         CPU                 GPU                  FPGA
          │                   │                    │
          ├───────────────────┼────────────────────┤
          │                   │                    │
          ▼                   ▼                    ▼
         ASIC            Accelerator         Future Target
                              │
                              ▼
                    HPC / Cluster / Distributed

---

129. Final Definition

The fundamental HDL rule is:

«Zamani HDL describes portable computational, structural, behavioral, verification, timing, and hardware realization intent. The compiler and target system determine how that intent becomes an actual implementation.»

Therefore:

WHAT
→ Zamani source

WHY
→ contracts / requirements

WHAT IS ALLOWED
→ policies

WHAT IS AVAILABLE
→ capabilities / resources

HOW
→ optimization / synthesis / lowering

WHEN
→ timing / scheduling

WHERE
→ placement / deployment

HOW CONNECTED
→ routing

WHICH DEVICE
→ target realization

WHICH PHYSICAL RESOURCE
→ backend / HAL

WHICH QUANTUM REPRESENTATION
→ quantum::ir

WHICH QEC STRATEGY
→ resilience / QEC

WHICH FAULT / NOISE MODEL
→ ZQN

The source program remains the stable semantic contract.

The implementation may change.

The target may change.

The machine size may change.

The number of processors may change.

The amount of memory may change.

The available accelerator may change.

The FPGA may change.

The ASIC may change.

The quantum processor may change.

The distributed topology may change.

The future hardware may be completely different.

The source-level HDL meaning must remain stable unless the programmer explicitly changes the program.

---

130. Final Production Gate

"grammar/hdl/" may be declared production-ready only when this complete chain is valid:

Specification
      ↓
Canonical Lexer
      ↓
ANTLR Grammar
      ↓
HDL Composition
      ↓
Domain-Neutral AST
      ↓
Structural Validation
      ↓
Type Analysis
      ↓
Effect Analysis
      ↓
Capability Analysis
      ↓
Resource Analysis
      ↓
Contract Analysis
      ↓
Policy Analysis
      ↓
Provenance
      ↓
Hardware Semantic Model
      ↓
Classical IR / quantum::ir where applicable
      ↓
Optimization
      ↓
Elaboration / Lowering
      ↓
Synthesis
      ↓
Scheduling
      ↓
Placement
      ↓
Routing
      ↓
Resilience / Recovery / QEC where applicable
      ↓
ZQN where applicable
      ↓
HAL
      ↓
Target

And every HDL file must independently satisfy:

OWNERSHIP
DEPENDENCIES
PUBLIC RULES
AST CONTRACT
SEMANTIC CONTRACT
TYPE CONTRACT
EFFECT CONTRACT
CAPABILITY CONTRACT
RESOURCE CONTRACT
CONTRACT/POLICY CONTRACT
PROVENANCE CONTRACT
IR CONTRACT
DIAGNOSTICS
TESTS
SCALABILITY
DETERMINISM
COMPATIBILITY
HARD-CODING AUDIT
COMPLETION CRITERIA

The resulting HDL subsystem is therefore not a collection of unrelated grammar files.

It is one orchestrated, target-independent, open-world hardware language subsystem whose individual files have explicit ownership and integration boundaries and whose complete meaning participates in the single Zamani architecture:

Program Once
Compile Once
Run Everywhere
Run Anywhere
Run Forever

with no artificial universal hardware ceiling and with physical feasibility determined by the resources, capabilities, constraints, policies, and realization environment available at compilation and execution time.