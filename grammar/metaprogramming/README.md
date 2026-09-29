Zamani Metaprogramming Grammar

Path: "grammar/metaprogramming/"
Status: Production architecture and integration contract
Language: Zamani
Grammar technology: ANTLR parser-grammar composition
Rust baseline: Rust 1.97.1 / Rust 2021
Safety: Safe Rust only; "unsafe" is not permitted or required
Authority: This document defines the metaprogramming grammar subsystem contract; the normative language specification remains under "grammar/specification/".

---

1. Purpose

"grammar/metaprogramming/" defines the source-language syntax boundary for Zamani metaprogramming.

Metaprogramming allows Zamani programs to describe compile-time transformations and compile-time inspection while remaining part of the same language and the same semantic pipeline as ordinary Zamani code.

The subsystem may support:

- compile-time computation;
- compile-time value production;
- compile-time reflection;
- quotation;
- unquotation/splicing;
- source generation;
- specialization;
- compile-time structural transformation;
- interaction with the macro subsystem;
- compile-time inspection of declarations;
- compile-time inspection of types;
- compile-time inspection of attributes;
- compile-time inspection of effects;
- compile-time inspection of capabilities;
- compile-time inspection of resource requirements;
- generation of classical programs;
- generation of quantum programs;
- generation of hybrid programs;
- generation of HDL/hardware intent;
- generation of distributed/data/AI/networking/security constructs.

The subsystem does not execute these facilities merely because they are parsed.

The architectural sequence is:

source
  ↓
canonical lexer
  ↓
canonical parser
  ↓
domain-neutral frontend AST
  ↓
ordinary semantic analysis
  ↓
metaprogramming semantic analysis
  ↓
authorized transformation/evaluation
  ↓
validated Zamani AST / semantic model
  ↓
canonical semantic representation
  ↓
canonical IR
  ↓
optimization
  ↓
routing / scheduling / resilience / QEC / ZQN
  ↓
HAL / target realization
  ↓
runtime

The fundamental invariant is:

«Metaprogramming may transform Zamani programs, but it must never replace Zamani's canonical semantic pipeline.»

---

2. Authority Model

Metaprogramming is subordinate to the repository-wide language authority model.

The authority hierarchy is:

grammar/specification/
        ↓
normative language rules
        ↓
grammar/spec/
        ↓
formal subsystem contracts
        ↓
grammar/Zamani.g4
        ↓
canonical ANTLR composition root
        ↓
modular grammar components
        ↓
Rust lexer/parser
        ↓
frontend AST
        ↓
semantic analysis
        ↓
canonical semantic model
        ↓
canonical IR
        ↓
compiler/runtime

The following files must therefore not compete with one another:

Artifact| Authority
"grammar/specification/"| Normative language specification
"grammar/spec/"| Formal contracts and conformance rules
"grammar/Zamani.g4"| Canonical ANTLR root/composition
"grammar/lexer/"| Lexical contracts
"grammar/core/"| Shared language structure
"grammar/macros/"| Macro syntax
"grammar/metaprogramming/"| Metaprogramming syntax
"src/lexer.rs"| Rust lexer implementation
"src/parser.rs"| Rust parser implementation
"src/frontend/ast/"| Domain-neutral frontend AST
semantic subsystem| Semantic meaning
"quantum::ir"| Canonical quantum IR boundary
compiler| Lowering/optimization/target realization
runtime/HAL| Execution and target interaction
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Historical/extended design reference

"Zamani-Grammar.md" does not make syntax legal merely by describing it.

"grammar.md" does not create new language rules merely because an implementation happens to support something.

---

3. Scope

This directory owns the syntax and composition contracts for metaprogramming.

It does not own the implementation of metaprogramming.

3.1 This directory owns

- metaprogramming syntax;
- metaprogramming grammar composition;
- compile-time execution boundaries;
- reflection syntax;
- quotation syntax;
- unquotation/splicing syntax;
- source-generation syntax;
- specialization syntax;
- metaprogramming-specific grammar integration;
- source-level transformation boundaries;
- syntax-level metaprogramming attributes when such attributes are explicitly part of the language contract.

3.2 This directory does not own

It does not own:

- lexer implementation;
- token definitions;
- keyword definitions;
- ordinary expressions;
- ordinary statements;
- ordinary declarations;
- ordinary types;
- ordinary patterns;
- generic type semantics;
- macro expansion implementation;
- hygiene implementation;
- name resolution;
- type checking;
- effect checking;
- capability checking;
- resource allocation;
- compiler host access;
- filesystem access;
- network access;
- process execution;
- hardware discovery;
- target selection;
- optimization;
- scheduling;
- routing;
- calibration;
- QEC;
- ZQN;
- resilience;
- HAL;
- runtime execution;
- classical IR;
- "quantum::ir";
- HDL/hardware IR.

No metaprogramming grammar rule may silently acquire ownership over one of these domains.

---

4. Current Directory Contract

The production subsystem consists of the existing metaprogramming grammar components.

grammar/metaprogramming/
├── README.md
├── metaprogramming.g4
├── compile-time-execution.g4
├── reflection.g4
├── generation.g4
└── specialization.g4

The existing composition grammar also references quotation/splicing integration concepts.

Those concepts must not remain phantom dependencies.

If quotation/unquotation syntax is implemented as a distinct grammar component, it must have a single authoritative owner and be added to the directory contract through the repository's normal feature-completion process.

If quotation is fully owned by "generation.g4", then "metaprogramming.g4" must delegate to the generation owner rather than referring to an independently nonexistent grammar authority.

There must never be two independent implementations of quotation/splicing syntax.

This README therefore treats quotation and unquotation as language-level metaprogramming facilities while requiring their concrete ownership to be resolved before the composed grammar is considered production-complete.

---

5. File-by-File Responsibilities

5.1 "metaprogramming.g4"

Purpose

Composition boundary for all metaprogramming facilities.

Owns

- metaprogramming dispatch;
- metaprogramming declaration entry;
- metaprogramming expression entry;
- metaprogramming statement entry;
- integration of compile-time execution;
- integration of reflection;
- integration of generation;
- integration of specialization;
- integration of quotation/splicing where the repository's final quotation owner requires it.

Does not own

It must not redefine:

- macro syntax;
- compile-time implementation;
- reflection implementation;
- generation implementation;
- specialization algorithms;
- quotation implementation;
- ordinary expressions;
- ordinary statements;
- ordinary declarations;
- types;
- identifiers;
- names;
- paths.

Required public categories

The composition layer may expose:

metaprogrammingDeclaration
metaprogrammingExpression
metaprogrammingStatement

These must delegate to the actual owning grammar components.

Required delegated concepts

The final composed parser must provide exactly one authoritative implementation for:

macro declaration
macro invocation
compile-time declaration
compile-time expression
compile-time statement
generation declaration
generation expression
generation statement
reflection declaration
reflection expression
reflection statement
specialization declaration
specialization expression
specialization statement
quotation
unquotation/splicing

The names may change if the canonical grammar architecture requires different names, but ownership must remain singular and explicit.

Completion condition

"metaprogramming.g4" is complete only when:

- every delegated rule resolves;
- every delegated rule has exactly one owner;
- no duplicate parser rules exist;
- no duplicate lexer vocabulary exists;
- canonical expression/type/name rules are reused;
- macro syntax is not duplicated;
- quotation syntax is not duplicated;
- generated code re-enters the canonical frontend;
- no target-language action executes during parsing.

---

6. "compile-time-execution.g4"

Purpose

Defines the syntax boundary for explicitly requested compile-time computation.

Owns

- compile-time execution contexts;
- compile-time blocks;
- compile-time execution expressions;
- compile-time execution statements;
- compile-time result boundaries;
- explicit compile-time invocation syntax;
- compile-time transformation entry points.

Does not own

It must not implement:

- constant folding;
- partial evaluation;
- specialization algorithms;
- interpreter execution;
- evaluator implementation;
- VM implementation;
- filesystem access;
- network access;
- process execution;
- environment inspection;
- hardware discovery;
- QPU execution;
- GPU execution;
- FPGA execution;
- runtime execution;
- optimization;
- routing;
- scheduling;
- QEC;
- ZQN;
- IR construction.

Mandatory semantic sequence

A parsed compile-time construct must pass through:

parse
 ↓
AST construction
 ↓
name resolution
 ↓
type validation
 ↓
effect validation
 ↓
capability validation
 ↓
resource validation
 ↓
security policy
 ↓
compile-time eligibility
 ↓
authorized evaluation
 ↓
result validation
 ↓
canonical semantic model

Parsing itself must never execute compile-time code.

---

7. "reflection.g4"

Purpose

Defines source-level reflection syntax.

Reflection is a view over existing Zamani language information.

It is not a second type system, AST, IR, hardware model, or runtime model.

Owns

- reflection expressions;
- reflection subjects;
- reflection queries;
- declaration inspection syntax;
- type inspection syntax;
- signature inspection syntax;
- generic-parameter inspection syntax;
- attribute inspection syntax;
- effect inspection syntax;
- capability inspection syntax;
- resource-intent inspection syntax where explicitly specified.

Does not own

Reflection must not define:

- new types;
- declarations;
- AST structures;
- IR structures;
- hardware discovery;
- target selection;
- physical-device state;
- arbitrary host introspection;
- implicit filesystem inspection;
- implicit environment inspection.

Determinism

Reflection over language-defined compile-time information should be deterministic.

For example:

declaration identity
type identity
function signature
generic parameters
attributes
effects
declared capabilities
declared resource requirements

should not depend on the compiling machine.

Reflection over external state must be explicitly represented as an effect/capability dependency.

---

8. "generation.g4"

Purpose

Defines source-generation syntax.

Generated Zamani source must remain ordinary Zamani source.

Owns

- source quotation boundaries where generation owns them;
- generated-source boundaries;
- source construction syntax;
- generated declarations;
- generated expressions;
- generated statements;
- source fragments;
- splicing/unquotation integration;
- generation-specific syntax.

Does not own

It does not own:

- AST implementation;
- AST mutation;
- pretty-printing implementation;
- compiler backend;
- target code generation;
- quantum IR;
- HDL IR;
- machine code;
- target-specific lowering.

Mandatory generated-code pipeline

Generated source must follow:

generated representation
        ↓
canonical frontend
        ↓
lexer
        ↓
parser
        ↓
AST
        ↓
semantic analysis
        ↓
canonical semantic model
        ↓
canonical IR

A generator must never be allowed to bypass:

- type checking;
- effect checking;
- capability checking;
- resource checking;
- ownership checking;
- security checking;
- domain validation;
- provenance validation.

---

9. Quotation and Unquotation

Quotation and unquotation are metaprogramming facilities, but their ownership must remain singular.

9.1 Quotation

Quotation represents Zamani syntax/data as a compile-time value.

The exact surface syntax is determined by the canonical specification.

Quotation must preserve:

- syntax category;
- source spans;
- source provenance;
- lexical identity;
- semantic category;
- nesting;
- hygiene context where applicable.

9.2 Unquotation / Splicing

Unquotation/splicing inserts a compile-time value into a quotation or generated structure.

It must preserve:

- syntax-category correctness;
- source provenance;
- hygiene;
- semantic validation;
- diagnostics.

9.3 No second syntax language

Quotation must not introduce a second incompatible grammar.

A quoted:

expression

must represent the same Zamani expression category used by ordinary source syntax.

Likewise:

type
statement
declaration
pattern

must use the canonical corresponding language categories.

9.4 Typed quotation

Where the language supports typed quotation, quotation categories should distinguish at least:

quoted expression
quoted type
quoted pattern
quoted statement
quoted declaration
quoted source/item

The exact representation belongs to the canonical AST/semantic contract.

9.5 Unquotation safety

Unquotation must not permit malformed syntax to bypass parsing or semantic validation.

---

10. "specialization.g4"

Purpose

Defines syntax for requesting compile-time specialization.

Specialization permits implementation adaptation to known semantic information while preserving program meaning.

Owns

- specialization requests;
- specialization parameters;
- specialization conditions;
- specialization declarations;
- specialization expressions;
- specialization-specific source attributes where specified.

Does not own

It does not own:

- optimizer algorithms;
- target selection;
- scheduling;
- routing;
- backend selection;
- hardware discovery;
- physical mapping;
- runtime dispatch implementation.

Semantic invariant

Specialization may change:

implementation strategy
representation
algorithmic realization
resource strategy
target lowering

but must not silently change:

program meaning
observable semantics
type meaning
effect meaning
resource requirements
capability requirements
security guarantees

---

11. Macro Integration

Macros remain owned by:

grammar/macros/

Metaprogramming must integrate with macros without duplicating macro syntax.

The intended architecture is:

macro syntax
      ↓
macro AST
      ↓
macro semantic validation
      ↓
hygienic expansion
      ↓
generated/expanded Zamani AST
      ↓
ordinary semantic validation
      ↓
canonical semantic model

Metaprogramming must not redefine:

- macro declarations;
- macro parameters;
- macro invocation syntax;
- macro paths;
- macro hygiene;
- macro expansion implementation.

Macro invariants

Macro expansion must preserve:

- hygiene;
- source spans;
- definition provenance;
- invocation provenance;
- expansion provenance;
- deterministic behavior;
- semantic identity;
- diagnostics.

Generated quantum, HDL, hardware, distributed, AI, classical, or other domain constructs must return to their normal semantic pipelines.

---

12. AST Contract

Metaprogramming grammar produces parser structure.

It does not define a second AST.

The canonical frontend AST remains the repository's domain-neutral AST under the frontend architecture.

Every metaprogramming construct must have an explicit AST mapping before the feature can be marked complete.

At minimum the mapping must preserve:

- source span;
- source ordering;
- nesting;
- construct identity;
- names;
- paths;
- attributes;
- arguments;
- generic parameters;
- quotation category;
- splicing category;
- transformation origin;
- macro provenance;
- generated-source provenance.

Generated nodes must remain traceable to:

original source
    ↓
metaprogram invocation
    ↓
transformation
    ↓
generated node

The implementation may represent this provenance however the frontend architecture requires; the grammar must not invent a competing AST representation.

---

13. Semantic Contract

Grammar acceptance does not establish semantic validity.

The semantic subsystem must validate:

- name resolution;
- binding;
- type correctness;
- generic correctness;
- effect legality;
- capability authorization;
- resource requirements;
- compile-time availability;
- compile-time purity where required;
- deterministic behavior where required;
- recursion policy;
- expansion policy;
- specialization validity;
- quotation validity;
- unquotation validity;
- generated-source validity;
- provenance;
- hygiene;
- security policy.

Unknown semantic annotations must not silently acquire meaning.

Unknown generated constructs must not silently disappear.

---

14. Canonical IR Contract

The metaprogramming grammar produces zero IR.

It must never directly construct:

QuantumGate
Qubit
PhysicalQubit
ClassicalInstruction
HardwareInstruction
ScheduleOperation
QEC operation
ZQN fault
Resilience action

The required architecture is:

metaprogramming syntax
        ↓
frontend AST
        ↓
semantic analysis
        ↓
canonical semantic model
        ↓
canonical IR

For quantum:

generated Zamani quantum source
        ↓
canonical frontend AST
        ↓
quantum semantic analysis
        ↓
quantum::ir

"quantum::ir" remains the canonical quantum IR boundary.

No metaprogramming feature may create a parallel frontend quantum IR.

---

15. Quantum Integration

Metaprogramming may generate quantum programs.

It may also inspect statically available quantum-language information.

It must not define a new quantum semantic representation.

Generated quantum source must follow:

generation
   ↓
ordinary quantum syntax
   ↓
AST
   ↓
quantum semantic analysis
   ↓
quantum::ir
   ↓
optimization
   ↓
routing
   ↓
scheduling
   ↓
QEC / resilience
   ↓
ZQN
   ↓
HAL
   ↓
target realization

Quantum operation extensibility

Metaprogramming must not require a permanent parser enumeration such as:

H
X
Y
Z
CNOT
...

A data-driven quantum operation model remains preferred:

operationSpecifier
quantumTargetList
parameters
results
attributes
modifiers
effects
capabilities

This permits generated programs to express:

apply H ...
apply custom_gate ...
apply vendor.operation ...
apply operation(parameter) ...

without changing the language merely because a new operation exists.

---

16. Quantum Scalability

No metaprogramming grammar rule may establish universal limits such as:

MAX_QUBITS
MAX_QUBIT_REGISTER
MAX_GATES
MAX_CIRCUIT_DEPTH
MAX_DEVICES
MAX_QUANTUM_OPERATIONS

Likewise it must not hard-code:

physical qubit 0..N

as a universal language mechanism.

Valid semantic intent may include:

requires qubits >= n
requires capability("quantum.measurement")
requires capability("quantum.mid_circuit_measurement")
requires topology(...)

The compiler, scheduler, router, QEC system, ZQN layer, HAL, and target backend determine how that requirement is realized.

---

17. Classical Computing Integration

Metaprogramming may operate over canonical Zamani constructs representing:

- scalar values;
- integers;
- floating-point values;
- vectors;
- matrices;
- tensors;
- records;
- structures;
- functions;
- generic types;
- symbolic expressions;
- data structures.

It must reuse canonical Zamani types and expressions.

It must not introduce a metaprogramming-only mathematical type system.

A generated classical program must be indistinguishable semantically from an equivalent directly written program after successful semantic validation.

---

18. HDL Integration

Metaprogramming may generate parameterized hardware intent such as:

- modules;
- ports;
- signals;
- registers;
- state machines;
- pipelines;
- interfaces;
- protocols;
- memories;
- timing intent;
- verification properties.

The generated hardware remains subject to the normal HDL and hardware semantic pipeline.

The metaprogramming layer must not embed accidental physical assumptions.

Prefer:

generate parameterized hardware for width

over a universal language assumption such as:

all registers are 32 bits

The width may be a program parameter.

The language must not impose a universal width ceiling.

---

19. Hardware and Target Integration

Metaprogramming must distinguish:

Concept| Meaning
Requirement| What the program needs
Capability| What an environment can provide
Constraint| What must not be violated
Preference| Desired implementation characteristic
Hint| Guidance to implementation
Target| Explicit realization context
Physical mapping| Downstream realization decision

Metaprogramming must not silently convert compile-time inspection into physical hardware selection.

For example:

requires capability("gpu.compute")

is fundamentally different from:

select GPU 0

Likewise:

requires qubits >= n

is different from:

map logical q0 to physical qubit 17

The first belongs to portable program intent.

The second belongs to target realization.

---

20. POCO-REAF

Metaprogramming is subject to:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever»

This does not mean that one immutable machine-code binary can literally execute natively on every architecture without an appropriate runtime, translation layer, virtual machine, universal IR, or target realization mechanism.

It means that the program's semantic intent must remain portable.

The architecture is:

Zamani source
      ↓
portable semantic intent
      ↓
compile-time transformation
      ↓
canonical semantic model
      ↓
canonical IR
      ↓
target adaptation
      ↓
execution

Compile-time computation must not accidentally bind source semantics to:

- compiling CPU;
- GPU model;
- FPGA model;
- QPU model;
- ASIC;
- device identifier;
- physical qubit;
- physical address;
- memory bank;
- node identifier;
- network topology;
- vendor-specific implementation;
- host filesystem;
- host environment.

If a program intentionally requires target-specific information, that dependency must be explicit and represented through the appropriate target/resource/capability/effect contract.

---

21. Scalability Contract

The language must scale from:

tiny
↓
embedded
↓
single-core
↓
multicore
↓
many-core
↓
GPU
↓
FPGA
↓
ASIC
↓
accelerator
↓
QPU
↓
HPC
↓
cluster
↓
distributed
↓
cloud
↓
future execution models

The grammar itself must not impose finite universal capacity.

This means there must be no grammar-level limits for:

- number of declarations;
- number of functions;
- number of modules;
- number of generic parameters;
- number of generated nodes;
- number of macro expansions;
- number of quotation levels;
- number of specialization variants;
- number of quantum operations;
- number of qubits;
- tensor rank;
- tensor dimensions;
- number of hardware resources;
- number of nodes;
- number of devices;
- number of timelines;
- source size.

Actual compiler resource exhaustion is an implementation concern.

When a compiler reaches a configured resource budget, it must produce a structured diagnostic rather than silently truncating or changing program semantics.

---

22. Resource-Budget Distinction

The absence of language limits does not prohibit implementation safeguards.

A compiler may have configurable budgets for:

- compilation time;
- memory consumption;
- metaprogram execution;
- expansion work;
- generated source size;
- recursion detection;
- specialization work;
- diagnostic size;
- caching;
- evaluator steps.

Those are:

implementation policy

not:

language semantics

They must be:

- explicit;
- configurable where appropriate;
- deterministic within a declared compilation context;
- diagnostically observable;
- non-semantic;
- non-silent.

The compiler must never convert:

implementation budget

into:

universal language maximum

---

23. Security Contract

Metaprogramming is potentially executable during compilation.

Therefore it is a privileged compiler facility.

Parsing must never execute it.

Compile-time execution must be authorized by semantic analysis and compiler policy.

No implicit access is permitted to:

filesystem
network
environment
credentials
secrets
processes
subprocesses
hardware
devices
QPU
GPU
FPGA
clock
randomness
host memory

If such access becomes a supported language facility, it must have explicit:

capability
effect
resource
security
provenance

contracts.

A metaprogram must not gain authority merely because it was generated by another metaprogram.

Capabilities must not be escalated through quotation, unquotation, macro expansion, reflection, or specialization.

---

24. Determinism

Parsing must be deterministic.

For identical:

source
grammar version
lexer configuration
parser configuration
explicit language configuration

the syntactic result must be stable.

Metaprogram execution must be deterministic wherever the language contract requires deterministic compilation.

Reflection over language-defined information must be deterministic.

If nondeterminism is explicitly supported, it must be represented in the semantic/effect model rather than introduced implicitly by the compiler host.

The following must not silently affect language meaning:

- wall-clock time;
- random host state;
- CPU identity;
- GPU availability;
- filesystem ordering;
- network state;
- environment-variable ordering;
- device enumeration order.

---

25. Provenance Contract

Every transformation must be traceable.

The compiler must be able to associate generated constructs with their origins.

Conceptually:

source location
      ↓
metaprogram definition
      ↓
invocation
      ↓
transformation
      ↓
generated construct

Provenance must support:

- diagnostics;
- error locations;
- macro debugging;
- generated-source tracing;
- reproducibility;
- security auditing;
- semantic debugging;
- compatibility analysis.

Generated code must not erase its origin.

---

26. Hygiene Contract

Where metaprogramming interacts with macros or generated identifiers, hygiene must be preserved.

The grammar describes syntax.

The semantic implementation owns binding identity.

Generated identifiers must not accidentally capture unrelated identifiers merely because the generated text happens to contain the same spelling.

The implementation must preserve the distinction between:

source spelling

and:

semantic binding identity

No metaprogramming grammar rule may implement hygiene by textual substitution alone.

---

27. Reflection Safety

Reflection must distinguish:

language-defined information

from:

external machine state

The following may be statically inspectable where specified:

- declarations;
- types;
- signatures;
- attributes;
- generic parameters;
- effects;
- capabilities;
- resource requirements;
- dialect metadata;
- source-level provenance.

The following must not become implicit semantic dependencies:

- current CPU;
- current GPU;
- installed devices;
- physical qubits;
- current filesystem;
- current network;
- host username;
- host environment;
- secret material.

If such data is deliberately requested, it must pass through explicit capability/effect semantics.

---

28. Generated Program Validation

Generated source is not trusted merely because a trusted compiler generated it.

Every generated construct must undergo the same relevant validation as authored source.

At minimum:

lexical validity
      ↓
syntactic validity
      ↓
AST construction
      ↓
name resolution
      ↓
type validation
      ↓
effect validation
      ↓
capability validation
      ↓
resource validation
      ↓
domain validation
      ↓
security validation
      ↓
canonical semantic model

No generator may use a privileged path to skip these stages.

---

29. No Silent Semantic Loss

The following is prohibited:

source
  ↓
parse
  ↓
partial AST
  ↓
missing metaprogramming information
  ↓
IR

Information must not silently disappear.

This applies to:

- quotation category;
- unquotation;
- type information;
- effects;
- capabilities;
- resource requirements;
- attributes;
- modifiers;
- quantum controls;
- quantum measurement intent;
- HDL parameters;
- hardware constraints;
- provenance;
- security metadata.

If a construct cannot be represented by the next stage, the compiler must report a structured diagnostic before semantic completion.

It must not turn unsupported semantics into:

no-op
comment
discarded attribute
default behavior
target-specific guess

---

30. No Phantom Features

A feature must not be called production-ready merely because a grammar file or documentation page exists.

Every feature must have an explicit status:

SPECIFIED
IMPLEMENTED
PARTIALLY_IMPLEMENTED
EXPERIMENTAL
STABLE
DEPRECATED
REMOVED
RESERVED
DIALECT_DEFINED
TARGET_DEFINED

The appearance of a feature in "Zamani-Grammar.md" does not make it legal syntax.

A feature becomes stable only after the complete pipeline exists.

---

31. Feature Completion Contract

Every metaprogramming feature must answer all of the following before completion:

Feature identity
Purpose
Status
Specification owner
Lexical contract
Syntax owner
AST representation
Semantic representation
Effect contract
Capability contract
Resource contract
Security contract
Provenance contract
IR mapping
Compiler consumer
Runtime consumer
Diagnostics
Compatibility
Positive tests
Negative tests
Boundary tests
Scalability tests
Determinism tests
Portability tests
Hard-coding audit

A feature is incomplete if any required contract is missing.

---

32. File Independence Contract

Each metaprogramming grammar file must be designed so that its contract is complete before another file is modified.

For each file:

Purpose
Owns
Does not own
Inputs
Tokens consumed
Delegated grammar rules
AST mapping
Semantic mapping
Effects
Capabilities
Resources
Security
Provenance
IR integration
Compiler integration
Runtime integration
Tests
Compatibility
Scalability
Completion criteria

must be specified in advance.

This prevents the following failure mode:

finish file A
      ↓
edit file B
      ↓
discover A was incomplete
      ↓
rewrite A

The dependency contract must instead be known before implementation.

---

33. Integration Matrix

Facility| Grammar owner| AST owner| Semantic owner| IR owner
Macro declaration| "grammar/macros/"| frontend AST| macro semantics| normal program IR
Macro invocation| "grammar/macros/"| frontend AST| macro semantics| normal program IR
Compile-time execution| "compile-time-execution.g4"| frontend AST| compile-time evaluator| generated semantic program
Reflection| "reflection.g4"| frontend AST| reflection semantics| no separate IR
Generation| "generation.g4"| frontend AST| generation semantics| generated semantic program
Quotation| canonical quotation owner| frontend AST| quotation semantics| no separate IR
Unquotation| canonical quotation/generation owner| frontend AST| splice semantics| generated semantic program
Specialization| "specialization.g4"| frontend AST| specialization semantics| specialized normal IR
Quantum output| "grammar/quantum/"| frontend AST| quantum semantics| "quantum::ir"
HDL output| "grammar/hdl/"| frontend AST| HDL semantics| HDL/hardware IR
Hardware requirements| "grammar/hardware/" / "grammar/resources/"| frontend AST| resource/capability semantics| target-dependent lowering

No row may introduce a second authority.

---

34. Cross-Domain Integration

Metaprogramming must remain domain-neutral.

It may generate:

classical
quantum
hybrid
HDL
hardware
AI
data
distributed
networking
security
interoperability

constructs.

But the metaprogramming subsystem does not own their semantics.

The generated construct is returned to the domain's normal grammar and semantic pipeline.

For example:

metaprogram
   ↓
generated quantum source
   ↓
quantum grammar
   ↓
frontend AST
   ↓
quantum semantic analysis
   ↓
quantum::ir

Likewise:

metaprogram
   ↓
generated HDL source
   ↓
HDL grammar
   ↓
frontend AST
   ↓
HDL semantic analysis
   ↓
HDL/hardware IR

---

35. Hardware Independence

Metaprogramming must not make the compiler host the default target.

This is invalid as implicit language semantics:

if compiler_has_gpu:
    generate_gpu_program()
else:
    generate_cpu_program()

unless the program explicitly declares a capability-dependent specialization contract and the compiler records that dependency.

A better semantic model is:

if capability("gpu.compute"):
    specialize(...)

where the capability query itself is explicit and subject to the resource/capability/effect model.

The compiler must not silently convert host availability into source meaning.

---

36. Compile-Time Hardware Inspection

If future Zamani supports compile-time target inspection, the following distinction is mandatory:

capability discovery

versus:

semantic program dependency

A metaprogram may inspect an explicitly supplied target description.

That does not mean the generated program becomes permanently tied to that target.

Target-specific specialization must be represented as a target-specific realization or explicitly declared dependency.

---

37. Resource Scaling

Metaprogramming may generate structures proportional to a program parameter.

For example:

generate N operations

where "N" is a program-level value or semantic requirement.

The grammar must not impose a universal maximum.

The compiler may reject a particular compilation because an implementation budget is exhausted, but that rejection must be a resource diagnostic rather than a claim that Zamani semantics forbid the program.

---

38. Infinite / Unbounded Semantics

“Scale to infinity” means the language must not define artificial finite ceilings.

It does not mean that a physical implementation can allocate infinite memory or perform infinite work.

Therefore:

language domain

must be unbounded in its semantic model subject to mathematical validity, while:

implementation

may have finite available resources.

The compiler must preserve this distinction.

---

39. Rust 1.97.1 Contract

The compiler implementation consuming these grammar definitions must target:

Rust 1.97.1
Rust 2021 edition

Production implementation must use safe Rust.

"unsafe" must not be required for:

- parsing;
- AST construction;
- metaprogram evaluation;
- reflection;
- quotation;
- unquotation;
- generation;
- specialization;
- provenance;
- diagnostics;
- validation.

If an external dependency internally uses "unsafe", that does not authorize Zamani source or compiler subsystem code to introduce unnecessary unsafe code. The Zamani metaprogramming implementation itself must maintain a safe-Rust boundary.

---

40. Parser Safety

ANTLR grammar files must contain no executable target-language actions that:

- execute metaprograms;
- inspect the host;
- access files;
- access networks;
- access devices;
- mutate compiler state;
- invoke external processes;
- access secrets.

The grammar is declarative.

Semantic execution belongs downstream.

---

41. Diagnostics

Metaprogramming diagnostics must identify:

- source span;
- construct;
- feature;
- transformation stage;
- originating source where applicable;
- generated-source location where applicable;
- expansion provenance;
- reason for failure;
- expected syntax where appropriate;
- capability/effect/resource cause where applicable.

Diagnostics must distinguish:

syntax error
semantic error
capability error
resource error
security error
compile-time evaluation error
generation error
specialization error
unsupported target error
implementation resource exhaustion

These must not be collapsed into generic parser failures.

---

42. Error Recovery

The grammar should support useful parser recovery without accepting invalid metaprograms as valid programs.

Recovery must not:

- execute code;
- invent missing semantic constructs;
- silently discard quotation;
- silently discard unquotation;
- silently discard macro metadata;
- silently discard generated constructs.

A recovered parse tree that is not semantically valid must remain diagnostically invalid.

---

43. Compatibility

Metaprogramming syntax is part of the public language contract.

Changes must be evaluated for:

- lexical compatibility;
- parser compatibility;
- AST compatibility;
- semantic compatibility;
- macro compatibility;
- quotation compatibility;
- generated-source compatibility;
- provenance compatibility;
- dialect compatibility;
- source compatibility.

Adding a new keyword merely for metaprogramming should be avoided when an existing contextual or attribute-based mechanism can provide the same semantics without breaking ordinary identifiers.

The core language must not become a finite dictionary of metaprogramming operations.

---

44. Dialect Integration

Metaprogramming must not silently create dialects.

If a dialect extends metaprogramming, it must explicitly identify:

dialect name
dialect version
owner
syntax extensions
semantic extensions
AST mapping
IR mapping
capabilities
compatibility policy
feature status

Dialect syntax must remain opt-in or otherwise explicitly selected according to the canonical dialect system.

A dialect must not alter the meaning of stable core syntax silently.

---

45. Interoperability

Metaprogramming may generate interoperability constructs for:

C
C++
Rust
Python
WASM
OpenQASM
QIR
HDL
other supported formats

but these remain interoperability features.

The canonical Zamani semantic model remains authoritative.

External formats must not become a second Zamani semantic representation.

---

46. No Vendor Lock-In

Metaprogramming must not require language syntax for every vendor operation.

Vendor-specific constructs should preferably be represented through:

capability
operation identity
namespace
dialect
interoperability layer
target-specific lowering

rather than permanent core keywords.

For example, a quantum operation may semantically identify:

vendor.operation

without requiring a new root grammar rule every time a vendor introduces a new operation.

The same principle applies to:

- GPU operations;
- accelerator operations;
- FPGA primitives;
- AI operations;
- networking protocols;
- cryptographic primitives;
- future computing models.

---

47. Performance

Metaprogramming must be scalable in compiler implementation without creating language-level limits.

The implementation should support where appropriate:

- incremental evaluation;
- memoization;
- caching;
- dependency tracking;
- parallel compilation;
- deterministic cache keys;
- invalidation;
- lazy reflection;
- lazy generation;
- specialization caching.

These are implementation concerns.

They must not alter source semantics.

---

48. Reproducibility

A compilation should be reproducible when its declared inputs are identical.

Metaprogramming inputs must therefore be explicit.

The reproducibility model should account for:

source
language version
dialect versions
compiler version
metaprogram inputs
explicit capabilities
explicit target description
explicit resource policy
dependencies
compile-time configuration

Implicit host state must not become hidden program input.

---

49. Caching

Generated results may be cached when the semantic inputs are identical.

A cache key must not omit information that can affect semantics.

At minimum, relevant identity may include:

source identity
metaprogram identity
language version
dialect configuration
dependency identity
compile-time input values
capability context
relevant effect context
specialization context

Caching must never cause one target's generated result to be incorrectly reused for an incompatible target.

---

50. Recursion and Resource Exhaustion

Metaprogramming may be recursively defined where the language permits it.

The grammar must not impose an arbitrary universal recursion ceiling.

The implementation may have configurable evaluation/expansion budgets.

When exhausted, it must report:

metaprogram resource exhaustion

rather than silently truncating expansion.

Examples include:

- recursive macro expansion;
- recursive generation;
- recursive specialization;
- cyclic compile-time evaluation;
- excessive generated syntax.

The diagnostic must identify the relevant provenance chain where possible.

---

51. Hard-Coding Audit

Every metaprogramming grammar file must be checked for accidental universal capacity constants.

At minimum, the audit must reject or review constructs resembling:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ACCELERATORS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_DEVICE_COUNT
MAX_TIMELINES

and equivalent semantic hard-codings.

The presence of an ordinary program literal is not itself a violation.

For example:

let n = 1024;

is program data.

A rule such as:

generated_count <= 1024

as a universal grammar restriction is not.

---

52. Test Architecture

Metaprogramming must have all of the following test classes:

tests/
├── metaprogramming/
│   ├── syntax/
│   ├── compile-time/
│   ├── reflection/
│   ├── quotation/
│   ├── unquotation/
│   ├── generation/
│   ├── specialization/
│   ├── macros/
│   ├── provenance/
│   ├── hygiene/
│   ├── diagnostics/
│   ├── security/
│   ├── determinism/
│   ├── compatibility/
│   ├── portability/
│   ├── scalability/
│   ├── negative/
│   └── boundary/

No directory should be created merely for appearance; it must contain real conformance tests when the repository adopts that structure.

---

53. Positive Tests

Positive tests must cover at least:

- valid compile-time computation;
- valid compile-time blocks;
- valid reflection;
- valid declaration reflection;
- valid type reflection;
- valid quotation;
- valid unquotation;
- valid source generation;
- valid specialization;
- valid macro/metaprogram interaction;
- generated classical program;
- generated quantum program;
- generated hybrid program;
- generated HDL;
- generated hardware intent;
- generated distributed/data constructs.

---

54. Negative Tests

Negative tests must cover:

- malformed metaprogram syntax;
- invalid quotation;
- invalid unquotation;
- category-mismatched splice;
- invalid reflection subject;
- invalid compile-time effect;
- unauthorized capability;
- unauthorized filesystem access;
- unauthorized network access;
- unauthorized process access;
- invalid generated syntax;
- invalid generated type;
- invalid generated resource requirement;
- invalid specialization;
- semantic-loss attempt;
- invalid macro integration;
- provenance failure;
- hygiene violation;
- unsupported target dependency.

---

55. Boundary Tests

Boundary tests must cover:

- empty quotations;
- empty generated blocks where legal;
- nested quotation;
- nested unquotation;
- nested metaprograms;
- nested macro expansion;
- nested specialization;
- large valid source structures;
- deeply nested but valid syntax;
- large generated structures;
- large reflection sets;
- large generic structures;
- complex type reflection;
- mixed classical/quantum generation;
- mixed software/HDL generation.

---

56. Scalability Tests

Scalability tests must increase workload without changing the language contract.

Examples:

small generated program
large generated program
larger generated program

and:

small macro set
large macro set
large specialization set
large quotation structures
large reflection structures
large quantum generation
large HDL generation
large distributed generation

The tests must verify that no artificial language ceiling appears.

Compiler resource exhaustion must be reported explicitly if a configured implementation budget is reached.

---

57. Determinism Tests

Repeat identical compilation inputs and verify stable:

- parse tree;
- AST structure;
- provenance;
- generated source;
- diagnostics;
- semantic representation;
- specialization decisions where deterministic;
- cache identity where applicable.

The test must be repeated under different hardware environments where feasible to ensure host state is not silently entering language semantics.

---

58. Portability Tests

Generated programs must be checked through the same semantic path for:

classical
quantum
hybrid
HDL
hardware
AI
distributed
data
networking
security

The tests must verify that metaprogramming does not silently bind source semantics to one target.

---

59. Integration Tests

The complete repository pipeline must be exercised:

grammar specification
        ↓
lexer grammar
        ↓
parser grammar
        ↓
Rust lexer
        ↓
Rust parser
        ↓
frontend AST
        ↓
semantic analysis
        ↓
metaprogramming analysis
        ↓
canonical semantic model
        ↓
canonical IR
        ↓
compiler
        ↓
runtime / target

For quantum:

generated source
        ↓
frontend AST
        ↓
quantum semantics
        ↓
quantum::ir
        ↓
optimization
        ↓
routing
        ↓
scheduling
        ↓
QEC / resilience
        ↓
ZQN
        ↓
HAL

No metaprogramming-only shortcut is permitted.

---

60. Integration With "grammar/Zamani.g4"

"grammar/Zamani.g4" remains the canonical ANTLR composition root.

Metaprogramming must be reached through the canonical parser composition hierarchy.

"Zamani.g4" must not duplicate metaprogramming rules.

The final composition must guarantee:

Zamani.g4
    ↓
canonical parser composition
    ↓
metaprogramming grammar

and not:

Zamani.g4
    ├── metaprogramming copy A
    └── metaprogramming copy B

The root grammar must remain a composition boundary rather than a second implementation of metaprogramming.

---

61. Integration With "grammar/grammar.md"

"grammar.md" is an implementation-conformance reference.

Metaprogramming status must be represented explicitly there.

For each feature distinguish:

SPECIFIED
LEXICALLY_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
COMPILER_IMPLEMENTED
RUNTIME_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL
DEPRECATED

A grammar rule alone must never cause a feature to appear as fully implemented.

---

62. Integration With "grammar/Zamani-Grammar.md"

"Zamani-Grammar.md" remains a broad design/reference source.

Metaprogramming features described there must be classified.

Possible classifications include:

STABLE
PROPOSED
EXPERIMENTAL
ASPIRATIONAL
HISTORICAL
DEPRECATED
REMOVED
NOT_IMPLEMENTED

Promotion must follow:

design
 ↓
normative specification
 ↓
lexical contract
 ↓
AST contract
 ↓
grammar
 ↓
semantic implementation
 ↓
IR integration
 ↓
compiler integration
 ↓
runtime integration
 ↓
tests
 ↓
stable

Description alone never makes syntax legal.

---

63. Integration With "grammar/macros/"

Macro ownership remains separate.

The two systems have this relationship:

                 metaprogramming
                  /     |      \
                 /      |       \
             macros  reflection  generation
                |
             hygiene
                |
             expansion

Macros are one metaprogramming facility.

Metaprogramming is not a replacement for the macro subsystem.

The macro subsystem must not become a second metaprogramming grammar either.

---

64. Integration With "grammar/types/"

Metaprogramming may inspect and manipulate type-level information.

It must reuse canonical type syntax.

It must not create:

MetaType
MetaGenericType
MetaQuantumType

as an alternative type language merely because the implementation needs an internal representation.

Internal compiler representations are not grammar authorities.

---

65. Integration With "grammar/expressions/"

Metaprogramming must reuse canonical expressions.

A quoted expression is a representation of an ordinary Zamani expression, not a new expression language.

Reflection of expressions must not redefine expression precedence.

Generated expressions must be reparsed/validated according to the canonical expression grammar.

---

66. Integration With "grammar/declarations/"

Generated declarations must use canonical declaration categories.

Metaprogramming must not define a separate declaration syntax merely to construct declarations.

The generated result must be validated exactly as an authored declaration.

---

67. Integration With "grammar/modules/"

Generated modules, imports, exports, namespaces, and package declarations must respect the module system.

Metaprogramming must not bypass:

- visibility;
- dependency resolution;
- package boundaries;
- module versioning;
- import/export validation.

---

68. Integration With "grammar/effects/"

Compile-time operations must carry appropriate effect information.

An operation that accesses an external resource cannot become pure merely because it runs during compilation.

Effects must remain explicit.

Metaprogramming must not erase effects from generated code.

---

69. Integration With "grammar/resources/"

Generated programs may introduce resource requirements.

Those requirements must remain explicit.

For example:

requires qubits >= n
requires memory >= required_memory
requires capability("tensor.compute")
requires capability("gpu.compute")

must remain semantic requirements rather than being converted into physical allocations during grammar processing.

---

70. Integration With "grammar/hardware/"

Hardware-aware specialization must remain downstream of portable semantics.

The metaprogramming layer may generate hardware intent but must not become the hardware allocator.

Target realization remains the responsibility of the hardware/compiler pipeline.

---

71. Integration With "grammar/compile/"

Compile-time computation and compilation directives must remain distinct.

Metaprogramming describes transformations.

The compile subsystem owns:

- target selection;
- compilation profiles;
- reproducibility;
- optimization profiles;
- artifact production;
- deployment intent;
- cross-compilation.

Metaprogramming must not silently replace these facilities.

---

72. Integration With "grammar/execution/"

Metaprogramming may generate execution policies, but execution semantics remain owned by the execution subsystem.

The metaprogramming layer must not decide:

- final scheduling;
- final placement;
- runtime lifecycle;
- checkpointing;
- recovery;
- runtime tracing;
- runtime profiling.

---

73. Integration With "grammar/interoperability/"

Generated interoperability constructs must pass through the normal FFI/interoperability semantics.

Metaprogramming must not bypass ABI, calling-convention, safety, or type checks.

---

74. Integration With "grammar/dialects/"

A metaprogram may generate dialect syntax only where that dialect is explicitly available and permitted.

The generated syntax must be validated against:

dialect identity
dialect version
dialect capabilities
dialect compatibility

A metaprogram must not silently introduce a dialect dependency.

---

75. No Second Compiler

The metaprogramming subsystem must not become a compiler inside the compiler.

It may contain compile-time computation, but the result must ultimately return to the ordinary Zamani semantic pipeline.

The architecture is:

Zamani
  ↓
metaprogram
  ↓
Zamani representation
  ↓
Zamani compiler

not:

Zamani
  ↓
metaprogram
  ↓
independent compiler
  ↓
independent IR
  ↓
target

---

76. No Second Language

Metaprogramming is not a second language embedded inside Zamani.

It is an extension of Zamani's compile-time semantic capabilities.

It must reuse:

- names;
- paths;
- expressions;
- types;
- declarations;
- statements;
- patterns;
- attributes;
- effects;
- capabilities;
- resources.

The fewer duplicated language concepts exist, the smaller the compatibility surface becomes.

---

77. No Target-Specific Semantics in Core Metaprogramming

The core grammar must not introduce syntax whose sole purpose is:

CPU model X
GPU model Y
QPU model Z
FPGA family A
physical qubit N
memory bank N
node N
core N
thread N

unless the language specification explicitly defines those as target/deployment semantics.

Portable source must remain target-independent.

---

78. Semantic Requirement vs Implementation Decision

Metaprogramming must preserve this distinction:

Semantic requirement
    ↓
what must be true

Capability
    ↓
what target can provide

Preference
    ↓
what is desirable

Implementation decision
    ↓
how the compiler realizes it

For example:

requires capability("quantum.measurement")

is not:

use qpu 0

and:

requires memory >= required_memory

is not:

allocate physical address X

The compiler remains responsible for realization.

---

79. Future-Proofing

The metaprogramming architecture must permit future facilities without changing existing core semantics unnecessarily.

Potential future facilities include:

- proof generation;
- symbolic execution;
- compile-time verification;
- staged computation;
- automatic differentiation;
- hardware-aware specialization;
- quantum-circuit generation;
- HDL generation;
- accelerator generation;
- distributed program synthesis;
- schema-driven generation;
- formal contract generation.

New facilities must integrate through the same:

syntax
→ AST
→ semantic contract
→ canonical semantic model
→ IR

pipeline.

---

80. Production Readiness Gate

The metaprogramming subsystem is production-ready only when all of the following are true.

Grammar

- [ ] Every grammar file compiles.
- [ ] Every delegated rule resolves.
- [ ] Every delegated rule has exactly one owner.
- [ ] No duplicate rule names exist.
- [ ] No duplicate token definitions exist.
- [ ] Canonical lexer vocabulary is used.
- [ ] Canonical expression/type/name rules are reused.
- [ ] Quotation ownership is singular.
- [ ] Unquotation ownership is singular.
- [ ] Macro syntax is not duplicated.

AST

- [ ] Every feature has an AST mapping.
- [ ] Source spans are preserved.
- [ ] Provenance is preserved.
- [ ] Hygiene information is preserved.
- [ ] No second metaprogramming AST is introduced unnecessarily.
- [ ] No second quantum AST is introduced unnecessarily.

Semantics

- [ ] Name resolution is defined.
- [ ] Type semantics are defined.
- [ ] Effects are defined.
- [ ] Capabilities are defined.
- [ ] Resources are defined.
- [ ] Security is defined.
- [ ] Determinism is defined.
- [ ] Generated code is revalidated.
- [ ] Specialization preserves semantics.
- [ ] Reflection has explicit boundaries.

IR

- [ ] No metaprogramming IR exists.
- [ ] Generated classical code reaches canonical IR.
- [ ] Generated quantum code reaches "quantum::ir".
- [ ] Generated HDL reaches canonical HDL/hardware representation.
- [ ] No IR information is silently lost.

Scalability

- [ ] No universal hardware limits exist.
- [ ] No fixed quantum limit exists.
- [ ] No fixed macro count exists.
- [ ] No fixed generation size exists.
- [ ] No fixed specialization count exists.
- [ ] No fixed quotation depth exists.
- [ ] Compiler resource budgets are implementation policies.
- [ ] Resource exhaustion produces diagnostics.

Security

- [ ] Parsing never executes metaprograms.
- [ ] Compile-time execution is authorized.
- [ ] No implicit filesystem access exists.
- [ ] No implicit network access exists.
- [ ] No implicit process execution exists.
- [ ] No implicit secret access exists.
- [ ] No capability escalation exists.
- [ ] Generated code cannot bypass semantic validation.

Portability

- [ ] POCO-REAF invariants are preserved.
- [ ] Host hardware does not silently affect source meaning.
- [ ] Target specialization is explicit.
- [ ] Resource requirements remain separate from physical allocation.
- [ ] Quantum source remains portable.
- [ ] HDL source remains parameterizable.
- [ ] Future targets can consume canonical semantic representations.

Rust

- [ ] Rust 2021.
- [ ] Rust 1.97.1 compatibility.
- [ ] No "unsafe" required.
- [ ] No target-language actions in grammar.
- [ ] Compiler implementation remains safe Rust.

Tests

- [ ] Positive tests.
- [ ] Negative tests.
- [ ] Boundary tests.
- [ ] Scalability tests.
- [ ] Determinism tests.
- [ ] Compatibility tests.
- [ ] Security tests.
- [ ] Provenance tests.
- [ ] Hygiene tests.
- [ ] Classical integration tests.
- [ ] Quantum integration tests.
- [ ] Hybrid integration tests.
- [ ] HDL integration tests.
- [ ] Hardware/resource integration tests.
- [ ] Distributed/data/AI interoperability tests where implemented.

---

81. Definition of Done for Each File

A metaprogramming file is DONE only when:

Purpose
✓

Owns
✓

Does not own
✓

Token dependencies
✓

Grammar dependencies
✓

AST mapping
✓

Semantic mapping
✓

Effect contract
✓

Capability contract
✓

Resource contract
✓

Security contract
✓

Provenance contract
✓

IR contract
✓

Compiler consumers
✓

Runtime consumers
✓

Cross-domain integration
✓

Positive tests
✓

Negative tests
✓

Boundary tests
✓

Scalability tests
✓

Determinism tests
✓

Compatibility tests
✓

Hard-coding audit
✓

Completion criteria
✓

A later modification to another subsystem must not reveal an unspecified fundamental responsibility in a file that had already been marked complete.

If such a dependency is discovered, the feature was not actually complete and must be treated as incomplete architecture rather than patched opportunistically.

---

82. Final Architecture

The production metaprogramming architecture is:

                       Zamani Source
                            │
                            ▼
                    Canonical Lexer
                            │
                            ▼
                    Canonical Parser
                            │
                            ▼
                   Domain-Neutral AST
                            │
             ┌──────────────┴──────────────┐
             │                             │
             ▼                             ▼
      Normal Semantics             Metaprogramming Semantics
                                           │
                 ┌─────────────────────────┼────────────────────────┐
                 │                         │                        │
                 ▼                         ▼                        ▼
              Macros                  Reflection              Generation
                 │                         │                        │
                 └─────────────────────────┼────────────────────────┘
                                           │
                                  Quotation / Splicing
                                           │
                                           ▼
                                     Specialization
                                           │
                                           ▼
                                Validated Zamani AST
                                           │
                                           ▼
                              Canonical Semantic Model
                                           │
              ┌────────────────────────────┼────────────────────────────┐
              │                            │                            │
              ▼                            ▼                            ▼
        Classical IR                  quantum::ir                 HDL/Hardware
              │                            │                            │
              └────────────────────────────┼────────────────────────────┘
                                           │
                                           ▼
                                      Optimization
                                           │
                         ┌─────────────────┼─────────────────┐
                         │                 │                 │
                         ▼                 ▼                 ▼
                      Routing          Scheduling        Resilience
                         │                 │                 │
                         └─────────────────┼─────────────────┘
                                           │
                                           ▼
                                          ZQN
                                           │
                                           ▼
                                          HAL
                                           │
                                           ▼
                                   Target Realization
                                           │
                ┌──────────────┬───────────┼───────────┬──────────────┐
                ▼              ▼           ▼           ▼              ▼
               CPU            GPU         FPGA        QPU           Future
                │              │           │           │            targets
                └──────────────┴───────────┴───────────┴──────────────┘
                                           │
                                           ▼
                                        Runtime

The metaprogramming subsystem therefore remains a language-level transformation facility, not a second compiler, second AST, second IR, second quantum frontend, hardware allocator, runtime, or vendor language.

---

83. Governing Principles

The following rules are non-negotiable.

1. "grammar/metaprogramming/" defines syntax and integration contracts, not execution.
2. "grammar/Zamani.g4" remains the canonical grammar composition root.
3. The normative specification remains under "grammar/specification/".
4. "grammar/grammar.md" remains an implementation-conformance reference.
5. "grammar/Zamani-Grammar.md" remains an extended/historical design source.
6. Macro syntax remains owned by "grammar/macros/".
7. Quotation and unquotation must have one authoritative owner.
8. Metaprogramming must not create a second AST.
9. Metaprogramming must not create a second IR.
10. Metaprogramming must not create a second quantum IR.
11. "quantum::ir" remains the canonical quantum IR boundary.
12. Generated quantum programs use the ordinary quantum pipeline.
13. Generated HDL uses the ordinary HDL pipeline.
14. Generated hardware intent uses the ordinary hardware/resource pipeline.
15. Generated programs cannot bypass semantic validation.
16. Parsing never executes metaprograms.
17. Compile-time execution requires explicit semantic authorization.
18. Reflection must distinguish language state from external machine state.
19. Quotation must reuse canonical Zamani syntax categories.
20. Unquotation must preserve syntax-category correctness.
21. Macro expansion must preserve hygiene and provenance.
22. Specialization must preserve semantics.
23. Resource requirements must remain distinct from physical allocation.
24. Capabilities must remain distinct from preferences.
25. Implementation budgets must not become language limits.
26. No fixed qubit limit may be encoded.
27. No fixed CPU/core/thread/GPU/FPGA/node limit may be encoded.
28. No fixed tensor-rank or register-width limit may be encoded.
29. No physical hardware identifier may become an implicit source dependency.
30. No vendor operation should require permanent core-language enumeration when a generic semantic operation model is sufficient.
31. No AI framework should become part of the core metaprogramming grammar.
32. No mathematical library function should become a keyword merely for metaprogramming convenience.
33. Dialects must remain explicit.
34. External interoperability formats must not become Zamani's semantic authority.
35. Rust implementation must remain Rust 1.97.1 / Rust 2021.
36. No "unsafe" Rust is required or permitted for this subsystem.
37. Every feature requires AST, semantic, IR, compiler, runtime, and test contracts before being declared complete.
38. Every completed feature requires positive, negative, boundary, scalability, determinism, compatibility, and security validation where applicable.
39. Every completed feature must pass the hard-coding audit.
40. The language boundary remains target-independent.
41. The compiler remains target-aware.
42. The runtime remains responsible for execution.
43. Hardware realization remains downstream of portable semantic intent.
44. The ultimate objective is:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).»

---

84. Final Completion Statement

"grammar/metaprogramming/README.md" is complete as an architectural contract when a developer can use it, together with the referenced authoritative contracts, to implement or verify any metaprogramming feature without having to redesign the ownership model after another grammar/domain file changes.

The production rule is therefore:

ONE LANGUAGE
    ↓
ONE LEXICAL AUTHORITY
    ↓
ONE PARSER COMPOSITION
    ↓
ONE DOMAIN-NEUTRAL AST
    ↓
ONE SEMANTIC MODEL
    ↓
CANONICAL DOMAIN IRs
    ↓
quantum::ir for quantum semantics
    ↓
OPTIMIZATION / ROUTING / SCHEDULING / RESILIENCE / ZQN
    ↓
HAL
    ↓
TARGET REALIZATION
    ↓
RUNTIME

Metaprogramming sits inside that architecture rather than beside it.

Its job is to make Zamani programs programmable at compile time without sacrificing portability, safety, determinism, provenance, semantic correctness, hardware independence, or scalability.

That is the required contract for production-grade Zamani metaprogramming.