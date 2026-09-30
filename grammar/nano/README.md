Zamani Nano Domain Grammar

Path: "grammar/nano/README.md"
Language: Zamani
Domain: Nano-oriented computation, atom/molecule/material/interaction/protocol/agent domains
Status: Normative domain architecture and integration contract
Compiler baseline: Rust 2021, Rust 1.97 / Rust 1.97.1
Safety: Safe Rust only; production Rust MUST NOT use "unsafe"
Primary portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)"
Scalability objective: From the smallest supported nano computation to arbitrarily large computations, bounded only by program semantics, representation limits, declared policies, implementation capacity, target capabilities, and resources actually available.

---

1. Purpose

This file defines the production architecture, ownership boundaries, integration contracts, scalability rules, implementation requirements, validation requirements, and completion criteria for the Zamani "nano/" grammar domain.

The nano domain is part of one Zamani language.

It is not a separate nano programming language.

Nano-oriented programs MUST therefore use the same:

- lexer;
- identifiers;
- names;
- paths;
- expressions;
- types;
- declarations;
- statements;
- functions;
- modules;
- effects;
- memory model;
- concurrency model;
- resources;
- capabilities;
- security model;
- interoperability model;
- diagnostics;
- source locations;
- semantic analysis;
- canonical IR;
- compilation model;
- execution model;
- compatibility model

used by the rest of Zamani.

This directory owns only the nano-domain syntactic boundary and its domain-specific grammar contracts.

It MUST NOT become a second language, a second compiler, a physical-science database, a simulator, a hardware allocator, or a second IR.

---

2. Architectural Position

The complete architecture is:

Zamani source
     |
     v
canonical lexer
     |
     v
canonical parser
     |
     +-----------------------------+
     |                             |
     v                             v
universal grammar            nano domain grammar
                                  |
                 +----------------+----------------+
                 |                |                |
                 v                v                v
               atoms         molecules        materials
                 |                |                |
                 +----------------+----------------+
                                  |
                         interactions/protocols
                                  |
                         agents/capabilities
                                  |
                                  v
                         domain-neutral AST
                                  |
                                  v
                   structural/name/type analysis
                                  |
                                  v
              semantic/resource/capability/effect analysis
                                  |
                                  v
                     canonical semantic model
                                  |
                                  v
                           canonical IR
                                  |
              +-------------------+-------------------+
              |                   |                   |
              v                   v                   v
       classical semantics    quantum::ir      HDL/hardware IR
              |                   |                   |
              +-------------------+-------------------+
                                  |
                                  v
                    optimization / lowering
                                  |
             +--------------------+--------------------+
             |                    |                    |
             v                    v                    v
          simulation          scheduling           routing
             |                    |                    |
             +--------------------+--------------------+
                                  |
                         resilience / QEC / ZQN
                                  |
                                  v
                                  HAL
                                  |
                                  v
                         target realization
                                  |
       +-------------+------------+------------+-------------+
       |             |            |            |             |
      CPU           GPU          FPGA         QPU       future target
       |             |            |            |             |
       +-------------+------------+------------+-------------+
                                  |
                                  v
                              runtime

Nano syntax MUST stop at the appropriate language/semantic boundary.

It MUST NOT attempt to perform any downstream operation during parsing.

---

3. Repository Authority

The nano domain follows the repository-wide authority model.

3.1 Normative architecture

"grammar/DESIGN.md"

Owns:

- repository-wide grammar architecture;
- boundaries;
- frontend principles;
- canonical IR policy;
- POCO-REAF;
- scalability;
- safe-Rust requirements;
- domain integration.

Nano MUST conform to it.

---

3.2 Repository navigation and authority

"grammar/README.md"

Owns:

- grammar navigation;
- repository-level authority explanation;
- contribution workflow;
- high-level integration.

This file does not replace it.

---

3.3 Canonical grammar composition root

"grammar/Zamani.g4"

Owns:

- root parser composition;
- universal program structure;
- declaration dispatch;
- statement dispatch;
- expression/type entry points;
- EOF;
- canonical grammar composition.

Nano grammar components MUST be reachable through this composition root.

No nano grammar file may become an alternative root grammar.

---

3.4 Implementation-conformance reference

"grammar/grammar.md"

Owns the description of what the implementation actually accepts.

Nano features MUST NOT be marked implemented merely because a ".g4" file exists.

A nano feature is implemented only when its complete frontend/backend contract exists.

---

3.5 Extended design reference

"grammar/Zamani-Grammar.md"

May contain:

- historical nano concepts;
- proposed nano concepts;
- extended physical-computing concepts;
- Sankofa-related concepts;
- future designs.

It does not independently authorize syntax.

A feature from that document becomes production syntax only through the normal promotion process:

proposal
  ->
semantic design
  ->
AST contract
  ->
grammar
  ->
implementation
  ->
canonical IR
  ->
tests
  ->
compatibility
  ->
stable

---

3.6 Normative specification

Nano behavior MUST align with:

grammar/specification/
grammar/spec/

Relevant specifications include, where applicable:

grammar/specification/language.md
grammar/specification/lexical.md
grammar/specification/syntax.md
grammar/specification/semantics.md
grammar/specification/poco-reaf.md
grammar/specification/grammar-authority.md

grammar/spec/type-system.md
grammar/spec/compatibility.md
grammar/spec/lexical.md
grammar/spec/syntax.md

A nano grammar file MUST NOT contradict those contracts.

---

4. Existing Nano Grammar Components

The current repository already contains these nano grammar components:

grammar/nano/
├── agents.g4
├── atoms.g4
├── molecules.g4
├── materials.g4
├── interactions.g4
├── protocols.g4
└── capabilities.g4

The repository currently does not expose "grammar/nano/deployment.g4".

Therefore:

deployment.g4

is a planned integration component, not an existing file.

It MUST NOT be described as implemented until it actually exists and is integrated.

---

5. Nano Domain File Ownership

Each file has one primary responsibility.

File| Owns
"agents.g4"| Nano-agent source constructs
"atoms.g4"| Atomic-domain source constructs
"molecules.g4"| Molecular-domain source constructs
"materials.g4"| Material-domain source constructs
"interactions.g4"| Nano-domain interactions
"protocols.g4"| Nano-domain protocols
"capabilities.g4"| Nano-domain capability contracts
"deployment.g4"| Nano deployment intent, when created
"README.md"| Domain architecture and integration contract

No file may silently assume ownership of another file's domain.

---

6. What Nano Grammar Owns

The nano domain may express source-level intent concerning:

- atoms;
- molecules;
- materials;
- nano-agents;
- interactions;
- protocols;
- capabilities;
- requirements;
- constraints;
- preferences;
- observations;
- transformations;
- composition;
- relationships;
- domain metadata;
- nano-oriented computation;
- nano-oriented communication;
- nano-oriented data;
- nano-oriented processes;
- nano-oriented hardware/software relationships;
- domain-specific annotations;
- future nano-domain extensions.

The grammar describes structure and intent.

It does not determine physical truth.

---

7. What Nano Grammar Does Not Own

Nano grammar MUST NOT own:

- lexical token definitions;
- general identifiers;
- general qualified names;
- general expression precedence;
- general types;
- universal declarations;
- universal statements;
- generic modules;
- general functions;
- memory allocation;
- hardware discovery;
- resource allocation;
- physical placement;
- scheduling;
- routing;
- calibration;
- quantum error correction;
- ZQN;
- HAL;
- compiler optimization;
- simulation implementation;
- physical chemistry;
- molecular dynamics algorithms;
- material databases;
- periodic-table databases;
- vendor hardware databases;
- runtime execution;
- physical device control;
- target-specific machine code;
- canonical IR construction.

Those responsibilities belong elsewhere in the repository.

---

8. Open-World Principle

Nano must be an open-world domain.

The grammar MUST NOT enumerate every possible:

- atom;
- isotope;
- molecule;
- material;
- interaction;
- protocol;
- agent;
- sensor;
- actuator;
- device;
- physical phenomenon;
- material property;
- simulation method;
- vendor implementation;
- future nano technology.

For example, the grammar MUST NOT become:

atom
    : hydrogen
    | helium
    | lithium
    | ...

Likewise, it MUST NOT become:

interaction
    : van_der_waals
    | covalent
    | ...

unless a concept has genuine language-level semantics that require dedicated syntax.

The default model is:

generic Zamani syntax
        +
typed semantic entities
        +
capabilities
        +
requirements
        +
domain metadata

This allows future nano concepts to be introduced without rewriting the core lexical model.

---

9. Annotations Are Extensible

The existing nano grammar components use open annotation forms.

Examples include:

@agent
@atom
@molecule
@material
@interaction
@protocol
@capability
@requires
@observe
@act
@coordinate

These annotation names MUST remain semantic identifiers rather than becoming a giant closed keyword list.

The grammar should recognize the structural form:

@ identifier

where appropriate.

Semantic analysis determines:

- whether the annotation is registered;
- which version defines it;
- what arguments it requires;
- what type it applies to;
- what capabilities it implies;
- what resources it requires;
- what constraints it introduces;
- whether it is deprecated;
- whether it is compatible with the active language edition.

This is critical for long-term scalability.

---

10. Lexer Integration

Nano MUST reuse the canonical lexer.

The nano grammar MUST NOT introduce duplicate lexer rules.

The authoritative lexical implementation remains:

src/lexer.rs

The lexical specification remains under:

grammar/lexer/
grammar/specification/lexical.md
grammar/spec/lexical.md

Nano-specific syntax MUST use existing canonical tokens such as applicable:

AT
IDENTIFIER
LPAREN
RPAREN
LBRACE
RBRACE
LBRACKET
RBRACKET
COLON
COMMA
DOT
SEMICOLON
ASSIGN

Exact token names MUST follow the actual canonical lexer.

A nano grammar file MUST NOT invent an alternate spelling for an existing lexical concept.

---

11. Identifier and Naming Integration

Nano MUST reuse the universal Zamani naming model.

It MUST NOT define a nano-specific identifier grammar.

This allows:

atom
molecule
material
agent
interaction
protocol
capability

to participate in the same:

- namespace system;
- module system;
- imports;
- aliases;
- qualified paths;
- generic parameters;
- symbol resolution;
- visibility;
- versioning.

A nano entity is therefore an ordinary Zamani semantic entity with nano-domain meaning.

---

12. Expression Integration

Nano grammar components MUST reuse:

expression
argumentList

from the canonical expression grammar.

Nano must not define a second expression language.

Therefore values such as:

mass
energy
position
temperature
dimension
composition
probability
state
resource requirement
capability parameter

remain ordinary Zamani expressions.

Examples:

@atom substrate {
    mass = mass_value;
}

or equivalent syntax defined by the individual grammar component.

The exact semantics belong to semantic analysis.

---

13. Type Integration

Nano MUST reuse the universal type system.

Relevant types may include:

Atom
Molecule
Material
Agent
Interaction
Protocol
Capability
Resource
Tensor<T, shape>
Memory<T, size>
Qubit
Qubit[n]

where those types are actually defined by the language/type system.

Nano MUST NOT create an isolated nano type system.

Domain-specific types may be introduced through semantic/type-system extensions, but their integration MUST remain compatible with:

grammar/types/
grammar/spec/type-system.md

---

14. AST Contract

Nano grammar constructs MUST map to the existing domain-neutral frontend AST.

The grammar MUST NOT require a parallel:

NanoAST
NanoAtomAST
NanoMoleculeAST
NanoMaterialAST
NanoAgentAST

unless a repository-wide AST design explicitly establishes such domain-neutral nodes.

The preferred architecture is:

nano syntax
    |
    v
generic declaration / annotation / expression / block nodes
    |
    v
semantic analysis
    |
    v
nano semantic model

This prevents the frontend from becoming a collection of vendor- or domain-specific AST hierarchies.

Every nano grammar construct must have a predetermined AST mapping before the grammar rule is considered complete.

---

15. Semantic Contract

Parsing MUST remain structural.

Semantic analysis owns:

- symbol resolution;
- type checking;
- annotation resolution;
- domain registration;
- physical-model validation;
- resource requirements;
- capability requirements;
- effects;
- ownership;
- lifetime;
- security;
- provenance;
- determinism;
- compatibility;
- cross-domain validation.

The grammar must never try to decide whether a nano construct is physically possible.

For example:

@atom X

may be syntactically valid.

Whether "X" represents a valid atomic entity is a semantic/domain-model question.

Likewise:

@molecule M

does not cause the parser to perform chemistry validation.

---

16. Resource Semantics

Nano programs may express resource requirements.

Examples conceptually include:

requires memory >= required_memory
requires capability("nano.interaction")
requires capability("simulation.atomistic")
requires capability("quantum.state")
requires capability("material.model")
requires topology(...)

The exact syntax must follow the canonical resource/capability grammar.

Nano MUST distinguish:

Requirement

A condition that must be satisfied.

Capability

Something a target or execution environment can provide.

Constraint

A condition restricting valid realizations.

Preference

A desired realization that is not necessarily required.

Hint

An optimization suggestion that does not change semantics.

Realization

A downstream implementation decision.

These concepts MUST NOT be conflated.

---

17. No Artificial Resource Limits

Nano grammar MUST NOT define universal limits such as:

MAX_AGENTS
MAX_ATOMS
MAX_MOLECULES
MAX_MATERIALS
MAX_INTERACTIONS
MAX_PROTOCOLS
MAX_NANO_DEVICES
MAX_NANO_RESOURCES
MAX_NANO_MEMORY
MAX_NANO_NODES
MAX_NANO_DIMENSION
MAX_NANO_DEPTH

Likewise it MUST NOT contain fixed parser-level limits such as:

atomCount <= 1024
moleculeCount <= 4096
agentCount <= 256

unless such a value is explicitly part of a program's own semantics rather than an implementation ceiling.

The following distinction is mandatory:

program requirement
        !=
compiler maximum

---

18. POCO-REAF

Nano MUST satisfy the same portability model as the rest of Zamani.

The programmer describes:

WHAT

rather than unnecessarily describing:

WHICH PHYSICAL MACHINE

The same nano program may be lowered to:

- symbolic execution;
- classical simulation;
- numerical simulation;
- molecular simulation;
- atomistic simulation;
- quantum simulation;
- QPU execution;
- CPU execution;
- GPU execution;
- FPGA execution;
- ASIC execution;
- embedded execution;
- distributed execution;
- HPC;
- cloud execution;
- laboratory/device integration;
- future execution substrates.

The target determines how the semantics are realized.

---

19. Scaling Model

Nano programs MUST scale from tiny computations to arbitrarily large computations subject to available resources.

The grammar MUST therefore allow unbounded semantic cardinality for:

- atoms;
- molecules;
- components;
- properties;
- agents;
- interactions;
- protocols;
- observations;
- transformations;
- processes;
- references;
- declarations;
- dimensions;
- data;
- resources;
- communication relationships.

The grammar MUST NOT encode today's physical or compiler capacities as language limits.

Scaling may be limited at execution time by:

- actual memory;
- compute capacity;
- available devices;
- communication bandwidth;
- latency;
- storage;
- target capabilities;
- physical feasibility;
- scheduling constraints;
- security policy;
- declared resource budgets;
- compiler implementation capacity.

Those are runtime/compiler concerns, not universal grammar ceilings.

---

20. Numeric Values

Numeric values appearing in source remain valid program semantics.

For example:

1024

may represent:

- a dimension;
- an amount;
- a count;
- a threshold;
- a property;
- a physical parameter;
- an algorithmic value.

The existence of "1024" in source MUST NOT cause the grammar to treat "1024" as a maximum.

This distinction is fundamental to POCO-REAF.

---

21. Atoms

"grammar/nano/atoms.g4" owns source-level atomic constructs.

It MUST remain responsible for syntax such as:

- atom declarations;
- atom parameters;
- atom properties;
- atom composition;
- atom references;
- atom-local requirements;
- atom-local constraints;
- atom-local capabilities;
- atom-local metadata.

It MUST NOT own:

- periodic-table implementation;
- isotope database;
- electron configuration algorithms;
- physical chemistry;
- quantum simulation;
- physical resource allocation.

Atomic semantic models belong downstream.

---

22. Molecules

"grammar/nano/molecules.g4" owns source-level molecular constructs.

It may express:

- molecular declarations;
- components;
- molecular parameters;
- component bindings;
- nested constructs;
- properties;
- requirements;
- constraints;
- capabilities;
- symbolic dimensions;
- initialization.

It MUST NOT implement:

- chemical validity;
- molecular dynamics;
- bond-energy calculations;
- physical synthesis;
- molecular simulation.

Those belong to semantic/domain engines.

---

23. Materials

"grammar/nano/materials.g4" owns source-level material constructs.

It may express:

- material declarations;
- material instances;
- material properties;
- composition;
- models;
- transformations;
- observations;
- requirements;
- constraints;
- capabilities;
- metadata.

It MUST NOT contain:

- a fixed material catalogue;
- vendor material databases;
- a fixed periodic table;
- hard-coded physical constants;
- simulation algorithms.

Material semantics belong downstream.

---

24. Interactions

"grammar/nano/interactions.g4" owns structural interaction syntax.

An interaction can represent a semantic relationship such as:

- communication;
- transfer;
- coupling;
- transformation;
- observation;
- actuation;
- dependency;
- synchronization;
- association.

The grammar MUST NOT enumerate every physical interaction.

For example, the grammar should support an open semantic operation model rather than requiring an ever-growing list such as:

interactionA
interactionB
interactionC
...

The semantic layer determines what an interaction means.

---

25. Protocols

"grammar/nano/protocols.g4" owns source-level protocol structures.

Protocols may describe:

- participants;
- messages;
- stages;
- operations;
- transitions;
- requirements;
- capabilities;
- constraints;
- security properties;
- timing intent;
- coordination.

Protocol syntax MUST reuse the universal:

- expressions;
- statements;
- types;
- modules;
- effects;
- resource model;
- security model.

A protocol MUST NOT directly perform networking or hardware operations during parsing.

Actual realization belongs to networking/runtime/backend layers.

---

26. Nano Agents

"grammar/nano/agents.g4" owns nano-domain agent structure.

It may express:

- agent declarations;
- identity;
- composition;
- interactions;
- goals;
- policies;
- observations;
- actions;
- capabilities;
- requirements;
- constraints;
- coordination;
- lifecycle.

It MUST NOT become a second general AI-agent language.

General AI agent semantics remain owned by:

grammar/ai/

Where an entity is both an AI agent and a nano agent, semantic composition must relate the two models rather than duplicating either.

---

27. Capabilities

"grammar/nano/capabilities.g4" owns nano-specific capability contract syntax.

Capabilities should express what is required/provided without embedding a physical implementation.

Examples conceptually include:

capability("nano.interaction")
capability("nano.material.model")
capability("nano.atomistic.simulation")
capability("nano.quantum")

Capability identity, versioning, provider resolution, and compatibility remain semantic/resource concerns.

Nano capabilities MUST integrate with:

grammar/resources/
grammar/hardware/
grammar/security/
grammar/compile/
grammar/execution/

They MUST NOT become a second capability system.

---

28. Deployment

"grammar/nano/deployment.g4" is currently a required future integration file but is not presently present in the repository.

When created, it MUST own only nano-specific deployment intent.

It MUST NOT duplicate generic deployment.

Generic deployment remains owned by:

grammar/execution/deployment.g4
grammar/hardware/deployment.g4

The nano deployment grammar should therefore act as a domain adapter.

Conceptually:

nano deployment intent
        |
        v
generic execution/deployment model
        |
        v
hardware/resource placement
        |
        v
target realization

It MUST NOT directly encode physical device selection.

---

29. Nano and Quantum Integration

Nano programs may participate in quantum computation.

Nano grammar MUST NOT define quantum syntax.

Quantum syntax remains owned by:

grammar/quantum/

Nano constructs that contain quantum computation must use the canonical quantum path:

nano source
   |
   v
domain-neutral AST
   |
   v
semantic analysis
   |
   v
quantum semantics
   |
   v
quantum::ir
   |
   v
optimization
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
   |
   v
QPU or simulator

Nano MUST NOT create:

NanoQuantumIR
NanoQubitIR
NanoCircuitIR

as competing quantum representations.

---

30. Nano and Classical Integration

Nano computations may lower to classical computation.

The semantic path is:

nano source
   |
   v
domain-neutral AST
   |
   v
semantic nano model
   |
   v
classical semantic representation
   |
   v
canonical classical IR
   |
   v
optimization
   |
   v
CPU/GPU/FPGA/ASIC/etc.

The nano grammar does not select the final processor.

---

31. Nano and HDL Integration

Nano-oriented hardware intent may participate in HDL/co-design.

Nano grammar MUST NOT duplicate HDL grammar.

HDL remains owned by:

grammar/hdl/

Hardware capability and target intent remain owned by:

grammar/hardware/
grammar/resources/

The relationship is:

nano intent
     |
     v
semantic model
     |
     +------------------+
     |                  |
     v                  v
software realization  hardware realization
                          |
                          v
                       HDL IR
                          |
                          v
                    synthesis/backend

This permits the same algorithmic intent to scale between software and hardware implementations.

---

32. Nano and AI Integration

General AI/ML syntax remains under:

grammar/ai/

Nano MUST not duplicate:

- model declarations;
- training;
- inference;
- neural operations;
- agent semantics;
- differentiable programming.

Nano may provide domain-specific structures that semantic analysis can connect to AI constructs.

The resulting representation must remain one semantic program.

---

33. Nano and Data/Tensor Integration

Nano computation may use:

grammar/data/
grammar/classical/

and tensor/data semantics.

Nano MUST reuse the universal tensor/data model rather than creating:

NanoTensor
NanoMatrix
NanoVector

without an explicit language-wide semantic reason.

Shapes and dimensions are program semantics.

They are not compiler maximums.

---

34. Nano and Memory Integration

Memory ownership remains under:

grammar/memory/

Nano-specific memory behavior may express semantic requirements, but must not define machine-specific memory ceilings.

No nano grammar may encode:

MAX_NANO_MEMORY
MAX_ATOMIC_MEMORY
MAX_MOLECULAR_MEMORY

Memory realization belongs to the memory/resource/compiler/runtime layers.

---

35. Nano and Concurrency

Nano interactions may require:

- parallelism;
- asynchronous execution;
- synchronization;
- coordination;
- message passing;
- distributed execution.

Generic concurrency syntax remains under:

grammar/concurrency/

Nano MUST consume the universal concurrency model rather than creating a separate threading/task system.

No fixed:

MAX_NANO_THREADS
MAX_NANO_PROCESSES

may be imposed.

---

36. Nano and Distributed Computing

Nano computation may scale from:

single process

to:

many processes
many devices
many nodes
clusters
HPC
cloud
distributed systems

Distributed semantics remain owned by:

grammar/distributed/

Nano only provides the domain-specific semantic information necessary to integrate with that system.

No fixed node/device count belongs in nano grammar.

---

37. Nano and Networking

Protocols and interactions may eventually map to networking constructs.

Networking remains owned by:

grammar/networking/

Nano MUST NOT directly encode:

- socket implementations;
- IP-specific assumptions;
- fixed network topologies;
- physical links;
- device addresses.

Such details belong downstream unless explicitly represented as portable semantic requirements.

---

38. Nano and Security

Nano constructs may carry security requirements.

Security remains owned by:

grammar/security/

Nano MUST reuse:

- identity;
- authorization;
- capability;
- policy;
- provenance;
- cryptography;
- trust;
- secure-computation semantics.

A nano grammar file MUST NOT invent a second security model.

---

39. Nano and Resource/Hardware Resolution

The nano domain can express intent.

Resource and hardware layers resolve realization.

The architecture is:

nano requirement
       |
       v
capability analysis
       |
       v
resource analysis
       |
       v
target discovery
       |
       v
allocation
       |
       v
placement
       |
       v
routing
       |
       v
scheduling
       |
       v
execution

The grammar must stop before target-specific realization.

---

40. Physical Reality Boundary

The grammar does not prove that a physical system exists.

For example:

@material hypothetical_material

may be syntactically valid.

Whether the material:

- exists;
- can be synthesized;
- is stable;
- is physically feasible;
- has the declared properties;
- can be simulated;
- can be manufactured

is outside parsing.

This separation is essential.

---

41. Simulation Boundary

Nano syntax may describe a computation intended for simulation.

The grammar MUST NOT contain simulator-specific implementation logic.

A simulation backend may choose:

- classical simulation;
- quantum simulation;
- molecular dynamics;
- Monte Carlo;
- symbolic methods;
- numerical methods;
- tensor methods;
- hybrid methods.

That selection belongs downstream.

---

42. Vendor Neutrality

Nano grammar MUST NOT embed vendor-specific syntax into the core language.

Vendor integration must use:

grammar/interoperability/
grammar/dialects/
grammar/hardware/
grammar/resources/

where appropriate.

Vendor extensions MUST declare:

- identity;
- version;
- capabilities;
- syntax extensions;
- semantic mappings;
- compatibility;
- lowering behavior.

They MUST NOT silently alter core nano semantics.

---

43. Dialect Integration

Nano dialects are permitted only through the canonical dialect architecture.

A dialect MUST NOT redefine:

- identifiers;
- general expressions;
- general types;
- universal statements;
- resource semantics;
- capability semantics;
- quantum IR.

A dialect may extend the nano domain when its extension contract is explicit.

Dialect lifecycle:

proposed
    ->
designed
    ->
experimental
    ->
implemented
    ->
tested
    ->
compatible
    ->
stable

---

44. Determinism

Nano parsing MUST be deterministic.

For a fixed:

source
+
lexer version
+
grammar version
+
language edition

the parser must produce the same structural result.

Nano grammar files MUST NOT contain:

- embedded Rust actions;
- nondeterministic semantic predicates;
- network access;
- filesystem access;
- hardware discovery;
- runtime calls;
- random behavior.

Semantic analysis may depend on external resource/capability state where explicitly required, but that must occur after parsing.

---

45. Safe Rust Requirement

The nano grammar and all production compiler integration MUST be compatible with:

Rust 1.97 / Rust 1.97.1
Rust 2021

Production compiler code MUST use safe Rust.

"unsafe" MUST NOT be introduced to implement nano support.

This includes:

- lexer integration;
- parser integration;
- AST conversion;
- semantic analysis;
- IR generation;
- IR validation;
- tests;
- resource analysis;
- diagnostics;
- tooling.

Performance optimizations must remain within safe Rust.

---

46. IR Integration

The nano grammar itself does not construct IR.

The compiler pipeline is responsible for:

AST
 ->
semantic model
 ->
canonical IR

The existing repository already has IR-generation and IR-verification responsibilities, including nano-related IR support.

Nano integration MUST use those existing canonical mechanisms.

No new nano IR is permitted merely because the nano domain is specialized.

Where the operation is classical, it must use the canonical classical/semantic IR.

Where it is quantum, it must use:

quantum::ir

Where it is hardware-oriented, it must use the repository's canonical hardware/HDL lowering path.

---

47. NanoOp Boundary

The existing compiler/IR architecture already contains a nano-oriented "NanoOp" concept in the IR verification path.

This does not authorize the grammar to invent a separate Nano IR.

The correct relationship is:

nano syntax
      |
      v
domain-neutral AST
      |
      v
nano semantic analysis
      |
      v
canonical semantic operation
      |
      v
existing IR representation

If "NanoOp" remains the appropriate canonical IR instruction, its semantic contract must be documented and verified independently.

If a future IR redesign replaces it, nano grammar MUST remain insulated from that implementation detail.

---

48. AST/IR Traceability

Every production nano construct must have an explicit traceability chain:

source syntax
    ->
grammar rule
    ->
AST representation
    ->
semantic entity
    ->
canonical IR representation
    ->
compiler consumer
    ->
runtime/backend consumer

No grammar rule is complete until this mapping exists.

This prevents:

grammar implemented today
AST invented later
IR invented later
compiler patched later

which causes cascading rework.

---

49. Cross-Domain Integration Matrix

Nano concern| Primary owner| Nano responsibility
Lexing| "src/lexer.rs", "grammar/lexer/"| Reuse
Parsing| "grammar/Zamani.g4" + nano grammars| Provide domain rules
Names| "grammar/core/"| Reuse
Expressions| "grammar/expressions/"| Reuse
Types| "grammar/types/"| Reuse
Statements| "grammar/statements/"| Reuse
Modules| "grammar/modules/"| Reuse
Effects| "grammar/effects/"| Reuse
Memory| "grammar/memory/"| Reuse
Concurrency| "grammar/concurrency/"| Reuse
Resources| "grammar/resources/"| Consume
Capabilities| "grammar/resources/", nano capabilities| Specialize
Classical| "grammar/classical/"| Interoperate
Quantum| "grammar/quantum/"| Interoperate
Hybrid| "grammar/hybrid/"| Interoperate
HDL| "grammar/hdl/"| Interoperate
Hardware| "grammar/hardware/"| Consume
Distributed| "grammar/distributed/"| Interoperate
AI| "grammar/ai/"| Interoperate
Data| "grammar/data/"| Interoperate
Networking| "grammar/networking/"| Interoperate
Security| "grammar/security/"| Consume
Compile| "grammar/compile/"| Consume
Execution| "grammar/execution/"| Consume
Interoperability| "grammar/interoperability/"| Consume
Dialects| "grammar/dialects/"| Consume
AST| "src/frontend/ast/"| Map to existing neutral model
Semantic analysis| semantic subsystem| Define nano meaning
IR| "src/ir_gen.rs" and canonical IR| Lower
IR validation| "src/ir_verify.rs"| Verify
Runtime| runtime/backend| Execute

---

50. Required Feature Contract

Every nano feature must be independently complete.

A feature is NOT complete merely because its ".g4" file parses.

The completion contract is:

Feature
Purpose
Status
Owns
Does not own
Syntax
Lexer dependencies
Parser dependencies
AST mapping
Semantic mapping
Type mapping
Effect mapping
Resource mapping
Capability mapping
IR mapping
Compiler consumers
Runtime consumers
Diagnostics
Security implications
Determinism
Positive tests
Negative tests
Boundary tests
Scalability tests
Compatibility tests
Hard-coding audit
Documentation
Completion criteria

A file may be considered complete only after all applicable sections have been resolved.

---

51. Required Header for Every Nano ".g4"

Every nano grammar component should document at least:

FILE
GRAMMAR
STATUS
LANGUAGE
COMPILER BASELINE
SAFETY
PURPOSE
ARCHITECTURAL POSITION
OWNERSHIP
NON-OWNERSHIP
LEXER DEPENDENCIES
PARSER DEPENDENCIES
OPEN-WORLD RULE
POCO-REAF
SCALABILITY
AST CONTRACT
SEMANTIC CONTRACT
RESOURCE CONTRACT
CAPABILITY CONTRACT
IR CONTRACT
INTEGRATION
DIAGNOSTICS
SECURITY
TESTING
COMPATIBILITY
COMPLETION CRITERIA

The existing nano grammar files already follow much of this documentation model. Future edits should complete missing contracts rather than introducing a different documentation style.

---

52. Positive Tests

Every nano feature needs positive tests covering:

- minimal valid form;
- named form;
- parameterized form;
- typed form;
- nested form where supported;
- expressions;
- references;
- annotations;
- capabilities;
- requirements;
- constraints;
- preferences;
- cross-domain use;
- modules/imports;
- large symbolic values;
- Unicode where permitted;
- source locations.

Tests must verify both acceptance and the expected AST/semantic interpretation where test infrastructure supports it.

---

53. Negative Tests

Every nano feature requires explicit rejection tests for:

- malformed declarations;
- malformed annotations;
- missing required components;
- invalid delimiters;
- invalid type forms;
- invalid expression forms;
- duplicate constructs where prohibited;
- invalid capability declarations;
- invalid resource contracts;
- invalid cross-domain references;
- invalid module visibility;
- invalid lifecycle transitions;
- unsupported syntax.

Negative tests must test the actual intended boundary rather than accidental parser failures.

---

54. Boundary Tests

Boundary tests must cover:

- empty bodies;
- single-item constructs;
- nested constructs;
- deeply nested structures;
- large identifiers;
- large numeric literals;
- large symbolic dimensions;
- many members;
- many parameters;
- many interactions;
- many participants;
- long qualified names;
- Unicode identifiers where supported;
- very large source units.

No boundary test may establish a fake universal maximum.

---

55. Scalability Tests

Nano scalability testing must test growth as data and resources increase.

Examples:

1 atom
many atoms

1 molecule
many molecules

1 interaction
many interactions

1 agent
many agents

1 protocol
many participants

small material model
large material model

small nano computation
large nano computation

Tests should use generated or parameterized workloads where possible rather than manually encoding arbitrary upper bounds.

The objective is to prove that implementation limits are resource/representation limits rather than language limits.

---

56. Resource-Availability Tests

The implementation must distinguish:

valid program

from:

valid program but insufficient target resources

For example:

source program
    |
    v
valid semantic program
    |
    v
target lacks capability
    |
    v
structured capability/resource diagnostic

It MUST NOT become:

target lacks resources
    |
    v
silently change program meaning

---

57. Determinism Tests

Tests must verify that the same source and grammar version produce stable syntax/AST results.

At minimum:

same source
same language version
same grammar version
same lexer version
        =>
same parse structure

Where semantic resolution depends on external capability/resource state, that dependency must be explicit and separately tested.

---

58. Compatibility Tests

Nano syntax must be tested against:

- language versions;
- grammar versions;
- lexer versions;
- parser versions;
- AST versions;
- semantic versions;
- IR versions;
- dialect versions.

Deprecated nano constructs must have explicit migration behavior.

Compatibility must not be inferred from filename similarity.

---

59. Diagnostics

Nano diagnostics must provide enough information to identify:

- source location;
- construct;
- domain;
- violated rule;
- expected form;
- actual form;
- relevant capability;
- relevant resource;
- language version;
- feature version;
- suggested migration where applicable.

Diagnostics should distinguish:

syntax error
type error
semantic error
capability error
resource error
compatibility error
security error
unsupported target

rather than collapsing all failures into generic parse errors.

---

60. Error Ownership

The grammar owns syntactic errors.

Semantic analysis owns semantic errors.

Resource analysis owns resource errors.

Capability analysis owns capability errors.

Target lowering owns target realization errors.

Runtime owns execution failures.

Nano grammar MUST NOT attempt to produce downstream errors during parsing.

---

61. Security Boundary

Nano grammar must not:

- execute external code;
- read files;
- access networks;
- discover hardware;
- invoke devices;
- query laboratories;
- access databases;
- execute simulations;
- allocate physical resources.

These operations belong to controlled downstream systems.

Parsing must remain safe and deterministic.

---

62. Provenance

Nano semantic entities should retain source provenance where supported.

At minimum, downstream representations should be able to associate semantic constructs with:

source file
source span
grammar construct
language version
feature version
dialect
module

This is required for:

- diagnostics;
- reproducibility;
- debugging;
- security;
- compilation provenance;
- scientific traceability;
- interoperability.

---

63. Reproducibility

Nano compilation should preserve enough information to reproduce semantic decisions.

Where target-specific realization occurs, the compiler should be able to explain:

source intent
        ->
semantic requirement
        ->
selected capability
        ->
selected resource
        ->
lowering decision
        ->
target artifact

The nano grammar itself does not make these decisions.

It supplies the source-level intent required to make them reproducibly.

---

64. No Physical Assumptions in Syntax

Nano syntax MUST remain independent of assumptions such as:

- atom size;
- molecular size;
- number of atoms in a material;
- number of particles;
- device size;
- sensor count;
- actuator count;
- memory capacity;
- processor count;
- GPU count;
- QPU count;
- FPGA capacity;
- network size;
- physical distance;
- clock rate.

Those may appear as program data or requirements, but they MUST NOT become universal language restrictions.

---

65. No Fixed Periodic Table in Grammar

A periodic table, isotope database, material catalogue, or physical-property database must never become part of the core grammar.

Instead:

source identifier
       |
       v
semantic registry/model
       |
       v
validated entity

This permits:

- future elements;
- synthetic materials;
- user-defined models;
- scientific databases;
- vendor-neutral models;
- research extensions.

---

66. No Fixed Molecular Model

The grammar must not assume that all molecular computation uses one:

- bonding model;
- force field;
- geometry representation;
- simulation engine;
- physical theory;
- coordinate system.

Those are semantic/backend choices.

---

67. No Fixed Material Model

The grammar must not force every material to use one physical representation.

A material may be represented by:

- symbolic model;
- analytical model;
- numerical model;
- tensor model;
- molecular model;
- quantum model;
- experimental data;
- hybrid model;
- future model.

The type/semantic system decides validity.

---

68. Nano-to-Quantum Boundary

Nano and quantum may intersect naturally.

Examples include:

- quantum materials;
- nanoscale quantum devices;
- quantum simulation of molecular systems;
- quantum chemistry;
- quantum sensing;
- quantum control.

The architecture must remain:

nano syntax
      |
      v
semantic nano model
      |
      +---- quantum semantics
      |          |
      |          v
      |      quantum::ir
      |
      +---- classical semantics
      |
      +---- hardware semantics

No duplicated quantum frontend is permitted.

---

69. Nano-to-Hardware Boundary

Nano programs may eventually describe or constrain:

- sensors;
- actuators;
- embedded devices;
- nano-electronic components;
- accelerators;
- specialized hardware.

The source language should express intent.

The hardware layer determines realization.

Therefore:

nano intent
     !=
physical device selection

and:

requires capability(...)

is fundamentally different from:

use device #7

The latter is target-specific realization and belongs downstream unless explicitly required by a target-bound program.

---

70. Nano-to-Distributed Boundary

A nano computation may scale from:

one computation element

to:

many computation elements

and eventually:

distributed systems
clusters
HPC
cloud
heterogeneous systems

The nano grammar must not contain a fixed distributed architecture.

Distribution belongs to:

grammar/distributed/
grammar/concurrency/
grammar/networking/

---

71. Nano-to-Future-Domain Boundary

Nano must remain extensible for future computational domains.

A future domain should be able to consume nano semantics without requiring changes to:

- identifier syntax;
- expression syntax;
- universal type syntax;
- resource model;
- capability model;
- canonical IR architecture.

This is one of the principal reasons the nano grammar uses open-world semantic identifiers.

---

72. Integration with "grammar/Zamani.g4"

The canonical composition should expose nano through one domain dispatcher.

Conceptually:

universalConstruct
    : ...
    | nanoConstruct
    | ...
    ;

or the equivalent structure established by the actual canonical grammar.

The exact rule name must follow "Zamani.g4".

The important invariant is:

Zamani.g4
    |
    v
Nano dispatcher
    |
    +--> NanoAgents
    +--> NanoAtoms
    +--> NanoMolecules
    +--> NanoMaterials
    +--> NanoInteractions
    +--> NanoProtocols
    +--> NanoCapabilities
    +--> NanoDeployment

The nano components must not independently parse complete Zamani programs.

---

73. Integration with "src/lexer.rs"

No nano-specific lexer implementation should be necessary merely because a new annotation name is introduced.

For example:

@future_nano_construct

should remain structurally representable through the universal annotation model where appropriate.

If a genuinely new lexical category is required, it must first be specified in:

grammar/lexer/
grammar/specification/lexical.md

and then implemented in the canonical lexer.

The nano grammar must not silently introduce lexical syntax.

---

74. Integration with "src/parser.rs"

The Rust parser is an implementation boundary.

If the repository's reference parser is hand-written, nano integration must maintain parity with the canonical grammar.

The parser must:

- recognize the same accepted syntax;
- reject the same invalid syntax;
- preserve source spans;
- produce the intended AST;
- remain deterministic.

ANTLR grammar acceptance alone is insufficient.

---

75. Integration with "src/frontend/ast/"

Nano must map to the domain-neutral AST architecture.

Before a nano grammar construct becomes stable, its AST mapping must be explicitly identified.

No stable nano grammar rule should depend on an AST node that does not exist.

Conversely, adding an AST node without corresponding grammar and semantic meaning should not be treated as a complete language feature.

---

76. Integration with Semantic Analysis

The semantic layer must interpret nano constructs.

The semantic contract must cover:

names
types
annotations
properties
relationships
capabilities
requirements
constraints
effects
resources
ownership
provenance
security
cross-domain relationships

Physical validity must be delegated to the appropriate scientific/domain implementation.

---

77. Integration with "src/ir_gen.rs"

Nano semantics must lower through the canonical IR-generation path.

The IR generator must not need to understand every physical nano concept directly.

Preferred model:

Nano semantic operation
        |
        v
canonical operation representation
        |
        v
IR lowering

If a specialized IR instruction such as "NanoOp" is retained, its semantics must be explicitly defined and verified.

---

78. Integration with "src/ir_verify.rs"

Every nano-related IR instruction must be verifiable.

The verifier must ensure:

- operands are defined;
- operand types are valid;
- result types are valid;
- control-flow invariants hold;
- resource/capability metadata is structurally valid where represented;
- invalid IR is rejected deterministically.

IR verification must not rely on Rust "unsafe".

---

79. Canonical Quantum Boundary

Where nano semantics become quantum semantics:

nano
 ->
semantic analysis
 ->
quantum::ir

must be the canonical path.

No:

nano_quantum_ir
nano_circuit_ir
nano_qpu_ir

should be introduced as a competing frontend representation.

---

80. Scheduling, Routing, QEC and ZQN

Nano grammar does not own:

- scheduling;
- routing;
- QEC;
- ZQN;
- calibration;
- resilience;
- physical mapping.

These occur after semantic lowering.

The general flow remains:

source
 ->
AST
 ->
semantic analysis
 ->
canonical IR
 ->
optimization
 ->
routing
 ->
scheduling
 ->
resilience/QEC/ZQN
 ->
HAL
 ->
target

Nano source may express requirements consumed by these systems, but must not duplicate their algorithms.

---

81. Deployment Ownership

Generic deployment remains under the execution/hardware domains.

Nano deployment, once implemented, should therefore be a thin domain-specific adapter.

It MUST NOT become another complete deployment language.

The desired structure is:

NanoDeployment
      |
      v
generic deployment semantic model
      |
      v
resource/capability analysis
      |
      v
hardware/execution deployment
      |
      v
target

---

82. Feature Status Model

Every nano feature MUST have one status:

SPECIFIED
PROPOSED
EXPERIMENTAL
PARTIALLY_IMPLEMENTED
IMPLEMENTED
STABLE
DEPRECATED
HISTORICAL

The status must reflect actual repository state.

A ".g4" file existing is not sufficient evidence for "IMPLEMENTED".

A semantic implementation without parser support is not complete.

A parser implementation without tests is not production complete.

---

83. Production Definition

Nano grammar is production-ready only when all of the following are true:

[ ] Normative specification exists
[ ] Ownership is explicit
[ ] Non-ownership is explicit
[ ] Lexer contract is complete
[ ] Parser contract is complete
[ ] Zamani.g4 composition is complete
[ ] Rust parser conformance exists
[ ] AST mapping exists
[ ] Semantic mapping exists
[ ] Type mapping exists
[ ] Effect mapping exists
[ ] Resource mapping exists
[ ] Capability mapping exists
[ ] IR mapping exists
[ ] IR verification exists
[ ] Compiler integration exists
[ ] Runtime integration exists where applicable
[ ] Diagnostics exist
[ ] Positive tests exist
[ ] Negative tests exist
[ ] Boundary tests exist
[ ] Scalability tests exist
[ ] Determinism tests exist
[ ] Compatibility tests exist
[ ] Hard-coding audit passes
[ ] No unsafe Rust is required
[ ] No competing IR exists
[ ] No competing root grammar exists
[ ] POCO-REAF contract passes

---

84. Hard-Coding Audit

The following must be rejected when used as universal implementation ceilings:

MAX_AGENTS
MAX_ATOMS
MAX_MOLECULES
MAX_MATERIALS
MAX_INTERACTIONS
MAX_PROTOCOLS
MAX_NANO_DEVICES
MAX_NANO_RESOURCES
MAX_NANO_MEMORY
MAX_NANO_NODES
MAX_NANO_THREADS
MAX_NANO_PROCESSES
MAX_NANO_DIMENSION
MAX_NANO_DEPTH

The audit must also search for semantic equivalents, not merely these exact strings.

Examples of suspicious constructs include:

if atom_count > 1024
if molecule_count >= 4096
array[256] as universal nano capacity

A numeric value is allowed when it is program data or a declared requirement.

It is prohibited when it becomes an undocumented compiler ceiling.

---

85. Scalability Is Not Infinite Memory

"Scale to infinity" in Zamani means:

«The language does not impose an artificial finite scalability ceiling.»

It does not mean:

«Every physical machine has infinite resources.»

A program may exceed available resources.

That must result in a correct resource/capability failure rather than a grammar restriction or silent semantic change.

Therefore:

language scalability
        !=
physical infinity

Instead:

language scalability
        =
no artificial language ceiling
+
resource-aware execution
+
explicit capability/resource semantics

---

86. Requirements Versus Realization

Nano must preserve this distinction:

requirement
constraint
capability
preference
hint
realization

For example:

requires capability("nano.simulation")

does not mean:

use simulator X

Likewise:

requires memory >= required_memory

does not mean:

use memory bank 3

And:

prefer accelerator("quantum")

does not mean:

execute on physical QPU 7

This separation is fundamental to POCO-REAF.

---

87. Source Compatibility

Nano syntax must remain compatible with the repository-wide language versioning model.

When syntax changes:

old source
    |
    v
compatibility analysis
    |
    +--> unchanged
    |
    +--> migration
    |
    +--> deprecated
    |
    +--> rejected with diagnostic

A syntax change must not silently change the meaning of an existing valid program.

---

88. Documentation Requirements

Each nano grammar file must document:

1. Purpose
2. Ownership
3. Non-ownership
4. Dependencies
5. Syntax
6. AST mapping
7. Semantic mapping
8. Resource integration
9. Capability integration
10. IR integration
11. Compiler integration
12. Runtime integration
13. Diagnostics
14. Security
15. Determinism
16. Scalability
17. Compatibility
18. Tests
19. Hard-coding audit
20. Completion criteria

This ensures that a file can be completed independently without needing a later redesign merely because another file was added.

---

89. Independent-First Implementation Order

The nano domain should be completed in dependency order.

Stage 1 — Domain contract

This README.

Stage 2 — Atomic foundation

atoms.g4

Stage 3 — Molecular foundation

molecules.g4

Stage 4 — Material foundation

materials.g4

Stage 5 — Interaction model

interactions.g4

Stage 6 — Protocol model

protocols.g4

Stage 7 — Capability model

capabilities.g4

Stage 8 — Agent model

agents.g4

Stage 9 — Deployment adapter

deployment.g4

Stage 10 — Canonical composition

grammar/Zamani.g4

Stage 11 — Reference implementation

src/lexer.rs
src/parser.rs
src/frontend/ast/

Stage 12 — Semantic integration

Semantic/type/effect/resource/capability analysis.

Stage 13 — IR

src/ir_gen.rs
src/ir_verify.rs

Stage 14 — End-to-end validation

Positive, negative, boundary, scalability, determinism and compatibility tests.

This order minimizes cross-file rework.

---

90. Existing Files Must Not Be Unnecessarily Renamed

The following existing names remain authoritative:

grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/DESIGN.md
grammar/README.md

grammar/nano/agents.g4
grammar/nano/atoms.g4
grammar/nano/molecules.g4
grammar/nano/materials.g4
grammar/nano/interactions.g4
grammar/nano/protocols.g4
grammar/nano/capabilities.g4

No rename is required to implement this architecture.

New files should be created only where an actual missing responsibility exists.

At present, "deployment.g4" is the clear nano-domain component identified by the architecture but absent from the inspected "grammar/nano/" directory.

---

91. Do Not Create Parallel Hierarchies

Do not create:

grammar/nano2/
grammar/physical/
grammar/nanoscale/
grammar/nano_language/

to duplicate the existing domain.

The existing "grammar/nano/" directory is the canonical home for nano-domain grammar.

Populate it.

Do not create a competing structure merely to avoid integrating existing work.

---

92. Do Not Create a Second Nano Grammar Root

There must not be:

grammar/nano/Nano.g4

acting as an independent language root.

The canonical root remains:

grammar/Zamani.g4

Nano is composed into Zamani.

---

93. Do Not Create a Nano Lexer

There must not be a:

NanoLexer.g4

that competes with "ZamaniLexer".

All nano syntax must use the canonical Zamani lexical model.

---

94. Do Not Create a Nano Parser Language

The nano grammar files are parser components.

They are not standalone languages.

They must consume canonical:

identifier
typeExpression
expression
argumentList
statement

and other shared grammar contracts.

---

95. Future Extensions

New nano features should normally be added through:

new semantic concept
        |
        v
feature specification
        |
        v
existing generic syntax if sufficient
        |
        +---- yes ---> no grammar change
        |
        +---- no ----> new domain grammar rule
                         |
                         v
                       AST
                         |
                         v
                     semantics
                         |
                         v
                       IR
                         |
                         v
                       tests

The default should be semantic extension before syntactic extension.

Do not add syntax merely because a semantic feature has a new name.

---

96. Generic Operation Principle

When nano computation introduces an operation whose semantics are data-driven, the grammar should prefer generic operation structures.

Conceptually:

operationSpecifier
operationTargetList
operationParameters
operationAttributes

rather than an ever-growing enumeration.

This is consistent with the repository's broader requirement that quantum and other computing domains remain open-world.

---

97. Scientific Extensibility

Nano must support future scientific models without changing the language core.

Possible semantic models may include:

- atomistic models;
- molecular models;
- material models;
- quantum models;
- statistical models;
- continuum models;
- experimental models;
- symbolic models;
- numerical models;
- learned models;
- hybrid models.

These are semantic/backend concerns.

The grammar supplies the structural language boundary.

---

98. Runtime Independence

The parser and grammar must not assume a particular runtime.

Nano programs may eventually execute under:

- native runtime;
- embedded runtime;
- simulator;
- distributed runtime;
- quantum runtime;
- accelerator runtime;
- laboratory integration runtime;
- future runtime.

Runtime selection occurs downstream.

---

99. Interoperability

Nano may interoperate with external systems through:

grammar/interoperability/

Potential external representations may include scientific, simulation, hardware, data, or quantum formats.

External formats must remain interoperability boundaries.

They must not replace Zamani's canonical semantic representation.

---

100. Testing Architecture

Nano tests should eventually occupy:

grammar/tests/nano/

with at least:

nano/
├── lexical/
├── syntax/
├── atoms/
├── molecules/
├── materials/
├── interactions/
├── protocols/
├── agents/
├── capabilities/
├── deployment/
├── quantum/
├── classical/
├── hdl/
├── distributed/
├── security/
├── positive/
├── negative/
├── boundary/
├── scalability/
├── determinism/
└── compatibility/

The exact existing test hierarchy should be reused where one already exists rather than creating duplicate test roots.

---

101. Minimal End-to-End Acceptance Chain

A nano feature is not production-ready until it can demonstrate:

source
  |
  v
canonical lexer
  |
  v
canonical parser
  |
  v
nano grammar
  |
  v
domain-neutral AST
  |
  v
semantic analysis
  |
  v
resource/capability analysis
  |
  v
canonical IR
  |
  v
IR verification
  |
  v
optimization/lowering
  |
  v
target realization

and the reverse diagnostic path:

target/resource failure
        |
        v
structured diagnostic
        |
        v
source location
        |
        v
original nano construct

must remain available.

---

102. Definition of Nano Grammar Completion

"grammar/nano/README.md" itself is complete when this architecture is the stable contract for the directory.

Each ".g4" file is complete only when:

[ ] Owns exactly one defined responsibility
[ ] Has explicit non-ownership
[ ] Reuses canonical lexer
[ ] Reuses canonical expressions
[ ] Reuses canonical types
[ ] Reuses canonical statements
[ ] Is reachable from Zamani.g4
[ ] Has AST mapping
[ ] Has semantic mapping
[ ] Has resource mapping
[ ] Has capability mapping
[ ] Has IR mapping
[ ] Has compiler consumer
[ ] Has runtime consumer where required
[ ] Has diagnostics
[ ] Has positive tests
[ ] Has negative tests
[ ] Has boundary tests
[ ] Has scalability tests
[ ] Has deterministic parsing tests
[ ] Has compatibility tests
[ ] Has hard-coding audit
[ ] Uses no unsafe Rust in implementation
[ ] Creates no competing IR
[ ] Creates no competing language

---

103. Final Nano Architecture

The production nano architecture is therefore:

                         ZAMANI
                           |
                    grammar/Zamani.g4
                           |
                    canonical lexer
                           |
                    canonical parser
                           |
                  universal AST boundary
                           |
                 +---------+---------+
                 |         |         |
              atoms    molecules   materials
                 |         |         |
                 +---------+---------+
                           |
                 interactions/protocols
                           |
                       agents
                           |
                    capabilities
                           |
                    semantic model
                           |
              +------------+------------+
              |            |            |
          classical      quantum       HDL
              |            |            |
              |       quantum::ir       |
              +------------+------------+
                           |
                     canonical IR
                           |
                 optimization/lowering
                           |
          +----------------+----------------+
          |                |                |
       simulation       routing         scheduling
          |                |                |
          +----------------+----------------+
                           |
                    resilience/QEC/ZQN
                           |
                          HAL
                           |
                    target realization
                           |
       +---------+---------+---------+---------+
       |         |         |         |         |
      CPU       GPU       FPGA      QPU      future
       |         |         |         |         |
       +---------+---------+---------+---------+
                           |
                        runtime

The invariant is:

«Nano is a domain of Zamani, not a separate language.»

The second invariant is:

«The nano grammar expresses computational and scientific intent; semantic analysis establishes meaning; resource/capability systems establish feasibility; canonical IR establishes implementation-independent computation; downstream systems establish physical realization.»

The third invariant is:

«No artificial nano-scale ceiling may be encoded into the language.»

The fourth invariant is:

«Nano programs must be able to scale from the smallest meaningful computation to arbitrarily large computations as resources permit, without rewriting the source merely because the realization changes.»

The fifth invariant is:

«Where nano computation crosses into quantum computation, the canonical quantum boundary remains "quantum::ir"; no competing nano quantum IR is permitted.»

The sixth invariant is:

«Where nano computation crosses into classical, HDL, hardware, distributed, AI, networking, security, memory, or execution domains, it must use those domains' existing canonical contracts rather than creating parallel systems.»

The seventh invariant is:

«Rust 1.97 / 1.97.1, Rust 2021, safe Rust only; no "unsafe" implementation is required or permitted.»

The final completion criterion is therefore:

Nano source
    ->
one canonical Zamani frontend
    ->
one domain-neutral semantic architecture
    ->
canonical IR
    ->
target-independent optimization/lowering
    ->
capability/resource-aware realization
    ->
CPU / GPU / FPGA / ASIC / QPU / simulator /
embedded / HPC / cluster / distributed / cloud /
future computational substrate

without changing the meaning of the original program merely because the target changed.

That is the nano-domain implementation of:

"Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)".