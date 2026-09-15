Zamani Metaprogramming Grammar

Path: "grammar/metaprogramming/"
Status: Production architecture and integration contract
Language: Zamani
Parser technology: ANTLR grammar composition
Compiler implementation: Rust 1.97 / Rust 1.97.1
Safety: Safe Rust only; no "unsafe" required or permitted by this subsystem

---

1. Purpose

The "grammar/metaprogramming/" subsystem defines the source-language syntax and architectural boundaries for Zamani metaprogramming.

Metaprogramming allows Zamani programs and compiler tooling to:

- execute explicitly authorized computations during compilation;
- inspect language-defined program structure;
- generate Zamani source constructs;
- request specialization;
- define and invoke macros;
- manipulate compile-time values;
- inspect types, declarations, metadata, capabilities, and effects;
- construct portable programs without embedding machine-specific assumptions.

Metaprogramming is a language facility, not a compiler implementation layer.

The grammar recognizes syntax.

The frontend constructs the canonical AST.

Semantic analysis determines legality.

The metaprogramming engine performs authorized transformation.

Generated or transformed programs return to the ordinary Zamani semantic pipeline.

The subsystem must never become a second AST, IR, quantum IR, hardware model, runtime, or compiler.

---

2. Architectural position

The authoritative pipeline is:

Zamani source
     |
     v
canonical lexer
     |
     v
canonical parser
     |
     v
canonical AST
     |
     +-----------------------------+
     |                             |
     v                             v
ordinary semantic analysis     metaprogramming analysis
                                   |
                 +-----------------+----------------+
                 |                 |                |
                 v                 v                v
              macros         compile-time      reflection
                 |            evaluation            |
                 +-----------------+----------------+
                                   |
                                   v
                              generation
                                   |
                                   v
                            specialization
                                   |
                                   v
                         validated Zamani AST
                                   |
                                   v
                     canonical semantic model
                                   |
              +--------------------+---------------------+
              |                    |                     |
              v                    v                     v
        classical IR          quantum::ir          HDL/hardware IR
              |                    |                     |
              +--------------------+---------------------+
                                   |
                                   v
                             optimization
                                   |
                                   v
                        routing / scheduling
                                   |
                                   v
                         hardware abstraction
                                   |
                                   v
                               runtime

The critical rule is:

«Metaprogramming transforms or describes Zamani semantics; it does not replace the canonical semantic pipeline.»

---

3. Ownership model

3.1 This directory owns

"grammar/metaprogramming/" owns:

- metaprogramming syntax composition;
- compile-time execution syntax;
- reflection syntax;
- source-generation syntax;
- specialization syntax;
- integration boundaries between those facilities;
- metaprogramming-specific syntactic categories;
- syntax-level quotation/splicing boundaries;
- metaprogramming source structure;
- syntax required to express compile-time transformations;
- explicit metaprogramming attributes where applicable.

---

3.2 This directory does not own

It does not own:

- lexer tokens;
- identifiers;
- keywords;
- literals;
- operators;
- expressions;
- statements;
- declarations;
- types;
- generic parameters;
- patterns;
- ordinary functions;
- modules;
- effects;
- capabilities;
- resources;
- hardware discovery;
- target selection;
- scheduling;
- routing;
- optimization;
- runtime execution;
- classical IR;
- quantum IR;
- HDL IR;
- QEC;
- ZQN;
- resilience;
- simulation;
- backend selection;
- physical qubit assignment;
- physical addresses;
- device discovery;
- compiler execution policy.

Those concepts remain owned by their canonical repository subsystems.

---

4. Canonical ownership boundaries

The following ownership table is normative.

Concept| Owner
Tokens| "grammar/lexer/"
Keywords| "grammar/lexer/keywords.g4"
Identifiers| "grammar/core/names.g4" / canonical lexer/parser
Paths| "grammar/core/paths.g4"
Attributes| "grammar/core/attributes.g4"
Expressions| "grammar/expressions/"
Statements| "grammar/statements/"
Types| "grammar/types/"
Patterns| canonical statement/declaration grammar
Functions| "grammar/functions/"
Modules| "grammar/modules/"
Effects| "grammar/effects/"
Resources| "grammar/resources/"
Hardware requirements| "grammar/hardware/" + "grammar/resources/"
Quantum syntax| "grammar/quantum/"
Quantum semantic representation| repository quantum IR
Quantum canonical IR| "quantum::ir"
Macro syntax| "grammar/macros/"
Macro expansion| compiler semantic/metaprogramming implementation
Compile-time execution syntax| "grammar/metaprogramming/compile-time-execution.g4"
Reflection syntax| "grammar/metaprogramming/reflection.g4"
Generation syntax| "grammar/metaprogramming/generation.g4"
Specialization syntax| "grammar/metaprogramming/specialization.g4"
Metaprogramming composition| "grammar/metaprogramming/metaprogramming.g4"
Runtime execution| runtime subsystem
Hardware discovery| hardware HAL
Scheduling| "grammar/execution/" + scheduling subsystem
Optimization| optimization subsystem
Fault description| ZQN
Error correction| QEC
Resilience decisions| resilience subsystem

No subsystem may silently duplicate another subsystem's ownership.

---

5. Files in this directory

The production directory is:

grammar/metaprogramming/
├── README.md
├── metaprogramming.g4
├── compile-time-execution.g4
├── reflection.g4
├── generation.g4
└── specialization.g4

The existing macro subsystem remains separate:

grammar/macros/
├── README.md
├── macros.g4
├── declarations.g4
├── invocations.g4
├── hygiene.g4
└── expansion.g4

This separation is intentional.

Macro expansion is a dedicated language mechanism.

Metaprogramming is the broader compile-time programming architecture.

They integrate without duplicating macro syntax.

---

6. "metaprogramming.g4"

Purpose

"metaprogramming.g4" is the composition grammar.

It connects:

- macros;
- compile-time execution;
- generation;
- reflection;
- specialization.

It must remain deliberately thin.

Owns

- metaprogramming dispatch;
- composition entry points;
- shared meta-language boundaries;
- common integration categories.

Does not own

It must not redefine:

- macro syntax;
- reflection syntax;
- generation syntax;
- specialization syntax;
- ordinary expressions;
- ordinary statements;
- types;
- declarations;
- identifiers.

Integration

The canonical parser may expose:

metaprogrammingDeclaration
metaprogrammingExpression
metaprogrammingStatement

where appropriate.

The composition layer delegates to the owning grammar.

Conceptually:

metaprogrammingDeclaration
    |
    +--> macroDeclaration
    +--> compileTimeDeclaration
    +--> generationDeclaration
    +--> reflectionDeclaration
    +--> specializationDeclaration

The same model applies to expressions and statements.

Critical requirement

The current composition approach uses names such as:

macroDeclarationCore
compileTimeDeclarationCore
generationDeclarationCore
reflectionDeclarationCore
specializationDeclarationCore

These are integration contracts, not permission to leave undefined parser rules indefinitely.

Before this grammar is considered production-complete, each referenced core rule must be provided by exactly one authoritative grammar component.

There must be no unresolved rule references in the final composed parser.

---

7. "compile-time-execution.g4"

Purpose

Defines source syntax for explicitly requested compile-time computation.

Owns

- compile-time execution expressions;
- compile-time execution statements;
- compile-time declarations;
- compile-time invocation syntax;
- explicit compile-time blocks;
- compile-time evaluation boundaries.

Does not own

It does not own:

- the evaluator;
- interpreter;
- VM;
- optimizer;
- runtime;
- filesystem;
- network;
- process execution;
- hardware access;
- target discovery.

Semantic requirement

Parsing compile-time execution must never execute the program.

The compiler must first:

1. parse;
2. build AST;
3. resolve names;
4. validate types;
5. validate effects;
6. validate capabilities;
7. establish provenance;
8. apply sandbox/security policy;
9. execute only authorized compile-time computation.

---

8. "reflection.g4"

Purpose

Defines source-level reflection.

The existing reflection grammar already establishes the correct architectural principle: reflection is a view over existing language information rather than a second type system, AST, IR, runtime model, or hardware model.

Owns

- reflection expressions;
- reflection subjects;
- reflection queries;
- member projections;
- declaration projections;
- type projections;
- metadata projections;
- capability projections;
- effect projections;
- language-entity inspection syntax.

Does not own

Reflection must not own:

- type definitions;
- declaration definitions;
- hardware discovery;
- quantum IR;
- resource discovery;
- runtime inspection;
- arbitrary host inspection.

Determinism

Reflection over language-defined information should be deterministic.

Examples include:

- declaration names;
- type information;
- signatures;
- generic parameters;
- attributes;
- effects;
- statically known capabilities.

Reflection over external or dynamic state must be explicitly represented and governed by semantic effects/capabilities.

Hardware rule

Reflection must never silently turn:

current machine state

into:

permanent program semantics

unless the programmer explicitly requested such a dependency and the semantic model records it.

---

9. "generation.g4"

Purpose

Defines source-generation syntax.

Generation allows compile-time facilities to produce ordinary Zamani source constructs.

Owns

- quotation boundaries;
- source construction;
- source-generation expressions;
- generated declarations;
- generated statements;
- generated expressions;
- generated source fragments;
- splicing syntax.

Does not own

It does not own:

- generated AST implementation;
- source printer implementation;
- compiler backend;
- code generator;
- target-specific lowering;
- quantum IR generation;
- hardware code generation.

Generated-source rule

Generated Zamani code must re-enter the ordinary language pipeline:

generated source
      |
      v
lexer
      |
      v
parser
      |
      v
AST
      |
      v
semantic validation
      |
      v
canonical semantic representation

Generation must never provide a bypass around semantic validation.

---

10. "specialization.g4"

Purpose

Defines syntax for requesting compile-time specialization.

Specialization may adapt an implementation to known semantic information while preserving program meaning.

Owns

- specialization requests;
- specialization parameters;
- specialization conditions;
- specialization attributes;
- specialization declarations;
- specialization expressions.

Does not own

It does not own:

- optimization algorithms;
- target selection;
- scheduling;
- routing;
- backend selection;
- hardware discovery;
- runtime dispatch.

Semantic rule

Specialization is permitted to change implementation strategy.

It must not silently change program semantics.

For example:

portable algorithm
        |
        +--> CPU specialization
        +--> GPU specialization
        +--> FPGA specialization
        +--> quantum specialization

is valid.

But:

specialization
    -> silently changes mathematical meaning

is invalid.

---

11. Macro integration

Macros are owned by:

grammar/macros/

Metaprogramming must integrate with them but must not duplicate their grammar.

The dependency is:

macro syntax
    |
    v
macro AST
    |
    v
macro expansion
    |
    v
validated Zamani AST

Macro expansion must preserve:

- hygiene;
- source provenance;
- source spans;
- deterministic expansion;
- semantic identity;
- diagnostics.

Generated code must pass through normal semantic validation.

---

12. Canonical AST contract

The grammar creates an ANTLR parse tree.

It does not create the canonical Zamani AST.

The frontend must translate parser output into the repository's canonical AST.

Every metaprogramming AST node must preserve enough information for:

- source location;
- source ordering;
- nesting;
- names;
- paths;
- attributes;
- arguments;
- type syntax;
- expression syntax;
- transformation origin;
- expansion provenance.

Generated constructs must retain provenance linking them back to their generator and originating source.

---

13. Semantic contract

The semantic layer is responsible for determining whether a syntactically valid metaprogram is legal.

Semantic analysis must handle:

- name resolution;
- type checking;
- generic resolution;
- effect checking;
- capability checking;
- resource requirements;
- compile-time availability;
- purity where required;
- deterministic behavior where required;
- recursion/expansion policy;
- specialization validity;
- generated-source validation;
- provenance;
- security policy.

Grammar acceptance does not imply semantic validity.

---

14. No grammar-to-IR coupling

The metaprogramming grammar creates zero IR.

It must never directly create:

QuantumGate
Qubit
PhysicalQubit
ClassicalInstruction
HardwareInstruction
ScheduleOperation
QEC operation
ZQN fault
Resilience action

The correct path is:

metaprogramming syntax
        |
        v
AST
        |
        v
semantic analysis
        |
        v
canonical semantic representation
        |
        +--> classical IR
        +--> quantum::ir
        +--> HDL/hardware representation
        +--> other future domain IR

This keeps the grammar independent from future hardware and compiler implementation changes.

---

15. Quantum integration

Metaprogramming may generate or inspect quantum programs.

It must not create a second quantum representation.

For example:

compile-time generation
        |
        v
Zamani quantum source
        |
        v
quantum AST
        |
        v
semantic quantum representation
        |
        v
quantum::ir

The canonical quantum semantic boundary remains:

quantum::ir

Metaprogramming must never bypass that boundary.

Generated quantum programs must receive exactly the same semantic validation as handwritten quantum programs.

---

16. Quantum scalability

No metaprogramming grammar rule may encode:

MAX_QUBITS
MAX_QUBIT_REGISTER
MAX_GATES
MAX_CIRCUITS
MAX_DEVICES

or equivalent limits.

The following must remain legal abstractions:

for each q in quantum_register:
    ...

generate circuit for required_size

specialize for available capability

The actual available resources are determined later.

A program may express a semantic requirement such as:

requires quantum capability

without requiring:

device = X
qubits = N
topology = Y

unless those are genuinely part of the program's semantics.

---

17. Classical integration

Metaprogramming may manipulate:

- scalar values;
- structures;
- arrays;
- vectors;
- matrices;
- tensors;
- functions;
- types;
- declarations;
- generic parameters;
- symbolic expressions.

It must use canonical classical syntax and semantic types.

It must not introduce a second mathematical or classical type system.

---

18. HDL and hardware integration

Metaprogramming may generate:

- hardware modules;
- ports;
- signals;
- registers;
- state machines;
- pipelines;
- hardware interfaces;
- parameterized hardware structures.

However, it must not encode physical assumptions into otherwise portable source.

Valid:

generate hardware parameterized by width

Potentially non-portable:

generate exactly device 7

unless the device identity is intentionally part of a target-specific deployment contract.

The distinction is:

semantic hardware design

versus:

physical implementation

must remain explicit.

---

19. Resource and capability integration

Metaprogramming may inspect or express semantic information involving:

- requirements;
- capabilities;
- constraints;
- preferences;
- hints;
- resources;
- targets.

These concepts remain distinct.

For example:

requirement

means what the program needs.

capability

means what an execution environment can provide.

preference

means what implementation is preferred.

constraint

means what must not be violated.

hint

guides implementation without becoming a semantic requirement.

Metaprogramming must not collapse these categories.

---

20. POCO-REAF contract

Metaprogramming must preserve:

«Program Once → Compile Once → Run Everywhere → Run Anywhere → Run Forever»

The intended architecture is:

source intent
    |
    v
portable semantics
    |
    v
compile-time transformation
    |
    v
canonical semantic representation
    |
    v
target adaptation
    |
    v
execution

Compile-time computation must not accidentally bind a program to:

- the compiling machine;
- CPU model;
- GPU model;
- FPGA;
- QPU;
- qubit count;
- memory capacity;
- physical topology;
- device identifier;
- vendor;
- physical address;
- local filesystem;
- network environment.

If such information is deliberately requested, it must become an explicit dependency through the language's capability/resource/effect system.

---

21. Compile-once requirement

POCO-REAF requires an important distinction.

A compiled artifact may contain target-dependent information when targeting a specific deployment.

Therefore:

Program Once

does not mean:

one immutable machine-code binary can physically execute on every architecture

without an appropriate execution/translation layer.

Instead, Zamani must preserve a stable semantic representation from which target-specific execution artifacts can be derived.

The metaprogramming system must never confuse:

program semantics

with:

one particular machine implementation.

---

22. Security boundary

Metaprogramming is potentially executable during compilation and therefore must be treated as a privileged compiler facility.

Parsing must never execute metaprograms.

Compile-time execution must require explicit authorization.

The metaprogramming system must not implicitly access:

- filesystem;
- network;
- environment variables;
- credentials;
- host memory;
- arbitrary processes;
- subprocesses;
- hardware;
- QPU;
- GPU;
- FPGA;
- secret material.

Such operations, if supported by the language, must require explicit capabilities and effects.

---

23. Determinism

Parsing must be deterministic.

For identical source and identical lexer/parser configuration:

source
    |
    v
tokens
    |
    v
parse tree

must produce the same syntactic result.

Compile-time execution may be nondeterministic only where explicitly permitted by the language and compiler policy.

Deterministic language-defined reflection should remain deterministic.

Compiler implementations should record sufficient provenance to reproduce transformations.

---

24. Provenance

Every generated construct must be traceable.

The compiler should be able to answer:

Where did this generated declaration come from?

Which metaprogram produced it?

Which source invocation triggered it?

Which specialization produced this version?

Which compile-time inputs influenced it?

This is essential for:

- diagnostics;
- debugging;
- reproducible builds;
- security auditing;
- incremental compilation;
- IDE tooling;
- provenance verification.

Provenance belongs to the AST/semantic/compiler infrastructure, not to an independent metaprogramming IR.

---

25. Hygiene

Macro-generated identifiers must respect the macro subsystem's hygiene rules.

Metaprogramming must not introduce an alternative hygiene system.

The architecture must prevent generated identifiers from accidentally capturing unrelated identifiers.

The owning subsystem is:

grammar/macros/hygiene.g4

and its compiler-side semantic implementation.

---

26. Expansion and recursion limits

The grammar must not impose artificial limits such as:

MAX_MACROS
MAX_EXPANSION_DEPTH
MAX_SPECIALIZATIONS
MAX_GENERATED_ITEMS
MAX_REFLECTION_DEPTH

Compiler implementations may enforce configurable resource budgets for:

- memory;
- execution time;
- expansion;
- recursion;
- generated source;
- compilation work;
- cancellation.

These are compiler policy/resource limits, not language grammar semantics.

They must therefore remain configurable and target/environment dependent.

---

27. Infinite scalability principle

"Infinity" in the Zamani requirement means:

«The grammar must not establish an arbitrary finite ceiling on scale.»

Actual execution is bounded by available:

- memory;
- compute;
- storage;
- communication;
- compilation resources;
- hardware;
- time;
- runtime policy.

Therefore the grammar should accept arbitrarily large syntactic structures subject only to implementation/resource limits.

No grammar constant may be used to simulate machine capacity.

---

28. Error handling

The grammar must produce precise syntax diagnostics.

The frontend/compiler must distinguish:

lexical error
syntax error
name-resolution error
type error
effect error
capability error
resource error
metaprogramming authorization error
compile-time execution error
generation error
specialization error
semantic error

A parser must not encode semantic failures as comments or silently accept malformed constructs.

---

29. Generated-code validation

Generated source must be treated as real Zamani code.

The following is mandatory:

generate
   |
   v
parse
   |
   v
AST
   |
   v
resolve
   |
   v
type-check
   |
   v
effect-check
   |
   v
capability-check
   |
   v
semantic validation
   |
   v
canonical IR

Generated code must not bypass:

- type checking;
- capability checking;
- resource checking;
- security validation;
- quantum semantic validation;
- hardware validation.

---

30. Runtime boundary

Compile-time metaprogramming must not automatically become runtime metaprogramming.

The source must make the phase explicit.

Conceptually:

compile_time { ... }

is distinct from:

runtime { ... }

and from:

reflection over runtime state

Runtime reflection, if eventually supported, must be separately specified.

The compile-time grammar must not silently grant runtime introspection.

---

31. Interoperability

Metaprogramming may generate interoperable source for:

- C;
- C++;
- Python;
- OpenQASM;
- Verilog;
- other supported dialects.

However, generated interoperability constructs must enter the appropriate interoperability subsystem.

For example:

metaprogram
    |
    v
generate OpenQASM-compatible Zamani representation
    |
    v
interoperability validation
    |
    v
OpenQASM lowering/export

Metaprogramming does not own the foreign-language grammar.

---

32. Dialect integration

Future Zamani dialects may provide metaprogramming extensions.

The architecture must support:

core metaprogramming
        |
        +--> standard facilities
        |
        +--> dialect facilities
        |
        +--> experimental facilities
        |
        +--> vendor extensions

Extensions must be namespaced and versioned.

A dialect must not silently redefine the meaning of core metaprogramming constructs.

---

33. Versioning

Metaprogramming syntax is part of the Zamani language contract.

Changes must follow the language compatibility policy.

Every syntax change must be classified as:

- additive;
- compatible;
- deprecated;
- breaking;
- experimental.

Generated source must record sufficient language-version information to support reproducible interpretation.

---

34. Reserved namespace policy

Do not reserve large numbers of identifiers unnecessarily.

Prefer:

core syntax

for fundamental language concepts.

Prefer semantic identifiers for extensible reflection/generation APIs where possible.

This avoids turning every future metaprogramming API name into a permanently reserved keyword.

Keywords should be introduced only when grammar ambiguity, readability, security, or semantic necessity requires them.

---

35. Lexer integration

Metaprogramming grammar files must consume the canonical lexer.

They must not create private lexical vocabularies.

The architecture is:

canonical Zamani lexer
        |
        v
metaprogramming parser

Any new keyword must be registered centrally in:

grammar/lexer/keywords.g4

and integrated into the canonical lexer.

A sibling grammar must never independently redefine a keyword token.

---

36. Parser integration

The canonical parser must compose metaprogramming entry points.

There must be exactly one authoritative syntactic meaning for each production.

The parser architecture must avoid:

Zamani.g4
    |
    +--> local metaprogramming grammar

metaprogramming.g4
    |
    +--> different metaprogramming grammar

Instead:

canonical parser
       |
       +--> metaprogramming composition
                    |
                    +--> macro grammar
                    +--> compile-time grammar
                    +--> generation grammar
                    +--> reflection grammar
                    +--> specialization grammar

---

37. AST integration

The canonical frontend must map metaprogramming constructs into canonical AST nodes.

There must not be:

MetaprogrammingAST

as an independent semantic universe.

A metaprogramming AST node may contain a metaprogramming-specific payload, but its embedded:

- expressions;
- types;
- statements;
- declarations;
- patterns;
- names;

must use canonical AST structures.

---

38. Classical IR integration

Compile-time-generated classical computation must eventually lower through the same classical semantic and IR infrastructure as handwritten classical code.

No special metaprogramming classical IR is permitted.

---

39. Quantum IR integration

Generated quantum source must eventually lower through:

quantum::ir

and must not directly instantiate or manipulate quantum IR from grammar code.

The pipeline is:

metaprogramming
       |
       v
generated quantum syntax
       |
       v
canonical quantum AST
       |
       v
quantum semantic analysis
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
hardware HAL

---

40. QEC integration

Metaprogramming may generate source that requests or describes error-correction constructs.

It must not implement QEC algorithms.

QEC remains responsible for:

- code definitions;
- syndrome processing;
- correction;
- logical error handling;
- QEC resource semantics.

The metaprogramming subsystem merely provides syntax transformation.

---

41. ZQN integration

Metaprogramming may generate programs whose execution is affected by ZQN-described faults.

It does not own:

- noise models;
- fault classification;
- correlated faults;
- leakage;
- loss;
- erasure;
- fault injection.

Those remain ZQN responsibilities.

---

42. Optimization integration

Specialization may expose optimization opportunities.

However:

specialization syntax

does not mean:

optimization algorithm

Optimization remains downstream.

The metaprogramming subsystem must not encode:

- peephole optimization;
- gate cancellation;
- scheduling;
- routing;
- target-specific instruction selection.

---

43. Scheduling integration

Metaprogramming must not encode physical timing assumptions.

Generated programs may express semantic timing requirements when timing is genuinely part of the language domain.

Actual:

- scheduling;
- resource allocation;
- ordering;
- alignment;
- delay insertion;
- dynamical decoupling;

remain downstream scheduling responsibilities.

---

44. Hardware integration

Metaprogramming may generate parameterized hardware designs.

It must not assume:

CPU count
GPU count
FPGA count
QPU count
core count
memory capacity
device address
topology
vendor

unless explicitly represented as target/resource constraints.

---

45. Distributed integration

Generated distributed programs must use the normal distributed language model.

Metaprogramming must not assume:

node 0
node 1
node N

as permanent machine topology.

Topology belongs to deployment/resource/hardware systems.

---

46. AI and data integration

Metaprogramming may generate:

- tensor operations;
- model structures;
- data pipelines;
- kernels;
- accelerator specializations.

It must use canonical types and expressions.

It must not introduce a second tensor type system.

---

47. Security requirements

The implementation must be safe Rust.

Required properties:

- Rust 1.97 or 1.97.1;
- no "unsafe";
- no Rust actions embedded in grammar;
- no parser-time execution;
- no implicit filesystem access;
- no implicit network access;
- no implicit process execution;
- no implicit hardware access;
- explicit capability boundaries;
- provenance;
- deterministic parsing.

Compiler implementation must avoid using metaprogramming as an escape hatch around security validation.

---

48. Resource accounting

Compiler-side metaprogramming execution may consume resources.

The compiler should therefore support configurable resource policies for:

- memory;
- CPU time;
- expansion work;
- recursion;
- generated source size;
- compilation work;
- cancellation.

These policies belong outside grammar semantics.

A policy may reject a compilation because resources are insufficient.

That must not mean the language grammar has a maximum program size.

---

49. Cancellation

Compile-time metaprogramming must support compiler-level cancellation where the compiler infrastructure provides it.

Cancellation must not change source semantics.

A cancelled compilation is an incomplete compilation, not a successful program transformation.

---

50. Incremental compilation

Metaprogramming should support incremental compilation through explicit dependency tracking.

The compiler should know which compile-time inputs affect generated output.

Changes to unrelated source must not require unnecessary regeneration.

Generated artifacts should be invalidated when their semantic dependencies change.

---

51. Reproducibility

A reproducible metaprogramming build should record:

- source version;
- language version;
- metaprogram inputs;
- compile-time dependencies;
- transformation identity;
- relevant capability declarations;
- deterministic configuration;
- provenance.

Host-specific information must not silently affect output.

If nondeterministic compilation is explicitly permitted, it must be represented and documented.

---

52. Testing contract

Every metaprogramming grammar component requires:

Positive tests

Valid:

- compile-time expressions;
- compile-time blocks;
- reflection;
- generation;
- specialization;
- macro integration;
- nested metaprograms;
- generic metaprograms.

Negative tests

Invalid:

- malformed quotation;
- malformed splice;
- invalid phase boundaries;
- malformed reflection;
- malformed specialization;
- malformed generated constructs;
- illegal nesting;
- ambiguous constructs.

Boundary tests

Test:

- minimal source;
- very large source;
- deeply nested constructs;
- many generated constructs;
- large generic structures;
- large reflection queries.

No test may imply a language-level fixed maximum.

---

53. Cross-domain tests

At minimum test:

classical + metaprogramming
quantum + metaprogramming
HDL + metaprogramming
hardware + metaprogramming
classical + quantum + metaprogramming
quantum + HDL + metaprogramming
quantum + hardware + metaprogramming
AI + metaprogramming
AI + quantum + metaprogramming
distributed + metaprogramming
classical + quantum + HDL + hardware + metaprogramming

The final combination must still flow through the canonical semantic pipeline.

---

54. Quantum scalability tests

Tests must demonstrate that the grammar does not impose limits based on:

- qubit count;
- gate count;
- register size;
- circuit depth;
- device count.

Tests should use parameterized source rather than testing only fixed examples.

Bad test philosophy:

the language supports exactly 32 qubits

Correct test philosophy:

the grammar permits a program whose qubit/resource size is determined
by semantic parameters and available resources.

---

55. Hard-coding audit

The metaprogramming directory must be audited for:

MAX_*
32
64
128
1024
fixed qubit counts
fixed device IDs
fixed CPU counts
fixed GPU counts
fixed FPGA counts
fixed topology
fixed memory
fixed register count
fixed generated-item count
fixed reflection depth
fixed macro count

Every discovered limit must be classified as:

1. language semantic requirement;
2. compiler resource policy;
3. implementation detail;
4. test limitation;
5. documentation example;
6. accidental hard-coding.

Accidental hard-coding must be removed.

---

56. No accidental semantic specialization

A metaprogram must not accidentally turn:

available capability

into:

required capability

For example, discovering that a compiler has a GPU must not automatically cause generated source to require a GPU.

Likewise:

available QPU has N qubits

must not automatically become:

program requires N qubits

unless the source explicitly requests that dependency.

---

57. Future-proofing

The grammar must be capable of supporting future metaprogramming mechanisms without requiring redesign of the entire language.

Potential future facilities include:

- compile-time queries;
- staged programming;
- dependent compile-time values;
- type-level computation;
- proof generation;
- formal verification generation;
- hardware specialization;
- quantum circuit generation;
- symbolic program transformation;
- automatic differentiation generation;
- accelerator kernel generation;
- deployment generation;
- distributed topology specialization.

New facilities must integrate through the existing architecture rather than create another semantic pipeline.

---

58. No circular dependencies

The dependency graph must remain acyclic.

Valid:

lexer
  ↓
parser grammar
  ↓
AST
  ↓
semantic analysis
  ↓
metaprogramming engine
  ↓
canonical semantic representation
  ↓
IR
  ↓
compiler
  ↓
runtime

Invalid:

grammar
  ↔
IR

Invalid:

grammar
  ↔
runtime

Invalid:

metaprogramming
  ↔
hardware discovery

Invalid:

quantum grammar
  ↔
quantum IR

---

59. Dependency graph for this directory

The intended dependency order is:

grammar/lexer/
        |
        v
grammar/core/
        |
        +--------------------+
        |                    |
        v                    v
grammar/types/       grammar/expressions/
        |                    |
        +---------+----------+
                  |
                  v
          grammar/statements/
                  |
                  v
          grammar/declarations/
                  |
                  v
            grammar/functions/
                  |
                  v
             grammar/modules/
                  |
                  v
             grammar/effects/
                  |
                  v
       grammar/resources/
                  |
                  v
          grammar/macros/
                  |
                  v
 grammar/metaprogramming/
        |
        +--> compile-time-execution.g4
        |
        +--> reflection.g4
        |
        +--> generation.g4
        |
        +--> specialization.g4
        |
        v
 metaprogramming.g4
        |
        v
 canonical parser
        |
        v
 canonical AST

The individual metaprogramming files should be independently completable before composition is finalized.

---

60. Recommended implementation order

Implement in this order:

Phase 1 — Contracts

Complete and freeze:

grammar/metaprogramming/README.md

This file establishes ownership and integration contracts for every sibling.

Phase 2 — Independent facilities

Implement and validate:

compile-time-execution.g4
reflection.g4
generation.g4
specialization.g4

Each must compile against the canonical lexer/parser contracts.

Phase 3 — Macro integration

Validate:

grammar/macros/

against the metaprogramming boundary.

No macro grammar duplication is allowed.

Phase 4 — Composition

Complete:

metaprogramming.g4

Only after all referenced sibling contracts exist.

Phase 5 — Canonical parser integration

Integrate metaprogramming entry points into the authoritative Zamani parser.

Phase 6 — AST integration

Implement canonical AST conversion.

Phase 7 — Semantic integration

Add:

- name resolution;
- type checking;
- effect checking;
- capability checking;
- resource checking;
- provenance;
- phase checking.

Phase 8 — Compiler integration

Integrate:

- macro expansion;
- compile-time execution;
- generation;
- specialization.

Phase 9 — Domain integration

Validate:

- classical;
- quantum;
- HDL;
- hardware;
- distributed;
- AI;
- data;
- networking.

Phase 10 — Production validation

Run:

- parser tests;
- negative tests;
- boundary tests;
- scalability tests;
- determinism tests;
- provenance tests;
- security tests;
- cross-domain tests;
- round-trip tests.

---

61. Completion criteria for "grammar/metaprogramming/"

The directory is production-ready only when all of the following are true:

- [ ] Every grammar file has exactly one owner.
- [ ] No grammar production is duplicated accidentally.
- [ ] No unresolved parser rules remain.
- [ ] Canonical lexer integration works.
- [ ] Canonical parser integration works.
- [ ] Canonical AST conversion works.
- [ ] Semantic validation works.
- [ ] Macro integration works.
- [ ] Compile-time execution is explicitly authorized.
- [ ] Reflection is deterministic where required.
- [ ] Generation preserves provenance.
- [ ] Specialization preserves semantics.
- [ ] Generated source undergoes ordinary validation.
- [ ] No metaprogramming grammar creates IR.
- [ ] Quantum output reaches "quantum::ir".
- [ ] Classical output reaches classical IR.
- [ ] HDL output reaches HDL/hardware representation.
- [ ] Hardware-specific information remains outside portable semantics.
- [ ] No machine-size limits are encoded.
- [ ] No fixed qubit limits are encoded.
- [ ] No fixed device limits are encoded.
- [ ] No parser-time execution exists.
- [ ] No implicit filesystem/network access exists.
- [ ] No implicit hardware access exists.
- [ ] Rust implementation uses no "unsafe".
- [ ] Rust 1.97 / 1.97.1 compatibility is verified.
- [ ] Deterministic parsing is verified.
- [ ] Security boundaries are verified.
- [ ] Provenance is verified.
- [ ] Cross-domain tests pass.
- [ ] Large-scale parsing tests pass.
- [ ] Smallest valid programs pass.
- [ ] Round-trip tests pass where applicable.
- [ ] Compatibility policy is documented.
- [ ] Future extension points are documented.

---

62. Definition of done for each file

A file in this directory is not considered complete merely because ANTLR accepts it.

A file is complete only when:

syntax
  +
ownership
  +
dependencies
  +
AST contract
  +
semantic contract
  +
integration contract
  +
security contract
  +
scalability contract
  +
compatibility contract
  +
tests
  +
hard-coding audit

have all been satisfied.

No later sibling file should require changing the fundamental ownership model of an already-completed file.

If a later integration exposes a missing contract, the correct action is to update the architecture deliberately rather than silently patching an earlier grammar file.

---

63. Production invariants

The following invariants are mandatory.

Invariant 1

Grammar describes syntax, not execution.

Invariant 2

Metaprogramming does not create IR.

Invariant 3

Generated code is ordinary Zamani code and receives ordinary semantic validation.

Invariant 4

Quantum semantics ultimately use "quantum::ir".

Invariant 5

Hardware characteristics are not silently converted into source semantics.

Invariant 6

Compiler resource limits are not language grammar limits.

Invariant 7

Reflection does not become arbitrary host introspection.

Invariant 8

Compile-time execution requires explicit authority.

Invariant 9

Specialization cannot silently alter program meaning.

Invariant 10

No unsafe Rust is required.

Invariant 11

The dependency graph remains acyclic.

Invariant 12

The metaprogramming subsystem remains extensible without becoming a second programming language.

---

64. POCO-REAF final guarantee

The metaprogramming subsystem exists to make programs more expressive without destroying portability.

The intended model is:

                ONE PROGRAM
                    |
                    v
             stable semantics
                    |
                    v
             metaprogramming
                    |
                    v
          canonical semantic model
                    |
        +-----------+-----------+
        |           |           |
        v           v           v
      CPU/GPU     Quantum      HDL
        |           |           |
        +-----------+-----------+
                    |
                    v
           hardware adaptation
                    |
                    v
                execution

The source program describes what the computation means.

Compilation and execution determine how that meaning is realized on the available machine.

Therefore:

«Zamani metaprogramming must describe transformations of computation, not accidental limitations of the machine performing those transformations.»

That is the architectural requirement that allows Zamani to scale from the smallest supported computation to arbitrarily large computations limited only by available resources, while preserving the goal:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).

---

65. Final ownership statement

"grammar/metaprogramming/" is a syntax and language-contract subsystem.

It is not:

an IR
a compiler
a runtime
a simulator
a hardware abstraction layer
a scheduler
an optimizer
a quantum compiler
a QEC engine
a ZQN engine
a resilience engine

Its permanent responsibility is:

Recognize
    ↓
Compose
    ↓
Preserve source intent
    ↓
Expose canonical AST boundaries
    ↓
Permit authorized compile-time transformation
    ↓
Return transformed programs to the canonical semantic pipeline

This boundary must remain stable as Zamani evolves from current classical, quantum, HDL, hardware, distributed, AI, and data facilities toward future computational models.