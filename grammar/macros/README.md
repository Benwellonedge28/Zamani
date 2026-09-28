Zamani Macro Grammar

Path: "grammar/macros/README.md"
Language: Zamani
Subsystem: Source-language macros and syntax-level metaprogramming
Grammar technology: ANTLR-compatible grammar components
Implementation baseline: Rust 2021, Rust 1.97 / Rust 1.97.1
Rust safety requirement: Safe Rust only; "unsafe" is prohibited
Status: Production architecture, ownership, integration, and conformance contract
Scalability target: From the smallest supported program to arbitrarily large programs, subject only to available resources and explicitly configured implementation safeguards
Portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)"

---

1. Purpose

"grammar/macros/" defines the source-language syntax contract for Zamani macros.

This directory exists to make macro syntax independently understandable, independently testable, and independently integrable into the canonical Zamani language without creating a second language, second AST, second IR, or target-specific macro system.

The macro subsystem must support programs ranging from tiny embedded programs to very large classical, quantum, hybrid, HDL, accelerator, distributed, AI, HPC, and future computing systems.

Macro syntax must therefore describe source intent and source structure, not current machine capacity.

The macro subsystem must remain valid regardless of whether the resulting program ultimately executes on:

- a tiny embedded processor;
- one CPU;
- many CPUs;
- a GPU;
- many GPUs;
- an FPGA;
- an ASIC;
- an accelerator;
- a QPU;
- a quantum simulator;
- an HPC system;
- a cluster;
- a distributed system;
- a cloud deployment;
- a heterogeneous system;
- a future computing architecture.

The macro grammar must not require modification merely because a new target architecture is introduced.

---

2. Fundamental Architectural Rule

The macro grammar answers:

«What macro source syntax is valid Zamani?»

It does not answer:

«How is the macro resolved, expanded, executed, optimized, scheduled, routed, resource-checked, lowered, or deployed?»

The production pipeline is:

Zamani source
      │
      ▼
canonical lexer
      │
      ▼
canonical parser
      │
      ├───────────────┐
      ▼               ▼
macro declarations   macro invocations
      │               │
      └───────┬───────┘
              ▼
       canonical frontend AST
              │
              ▼
       name/module resolution
              │
              ▼
        macro resolution
              │
              ▼
         macro expansion
              │
              ▼
    hygiene + provenance handling
              │
              ▼
       semantic validation
              │
              ▼
      canonical semantic model
              │
              ▼
        canonical IR boundary
              │
       ┌──────┼──────────┐
       ▼      ▼          ▼
  classical quantum::ir HDL/hardware
       │      │          │
       └──────┼──────────┘
              ▼
        optimization
              │
              ▼
 routing / scheduling / resilience
              │
              ▼
             ZQN
              │
              ▼
             HAL
              │
              ▼
       target realization

The macro grammar participates only in the source-language portion of this pipeline.

---

3. Repository Authority

The macro subsystem is subordinate to the repository-wide grammar authority model.

The authority order is:

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
canonical lexer/parser implementation
        │
        ▼
src/frontend/ast/
        │
        ▼
semantic analysis
        │
        ▼
canonical IR

The roles of the major existing files remain distinct.

File| Authority
"grammar/DESIGN.md"| Overall grammar architecture and boundaries
"grammar/README.md"| Navigation and repository-wide ownership
"grammar/specification/"| Normative human-readable language specification
"grammar/spec/"| Formal feature contracts
"grammar/Zamani.g4"| Canonical ANTLR composition/root grammar
"grammar/grammar.md"| Current implementation conformance
"grammar/Zamani-Grammar.md"| Historical, extended, proposed, experimental, and aspirational design material
"grammar/macros/README.md"| Macro subsystem architecture and completion contract
"grammar/macros/*.g4"| Modular macro syntax components
"src/frontend/ast/"| Actual source AST representation
compiler macro infrastructure| Resolution, expansion, hygiene, provenance, and expansion policy
canonical IR| Post-semantic representation

No macro document may override "grammar/DESIGN.md".

No macro document may silently introduce syntax that is absent from the authoritative specification and canonical grammar.

"grammar/Zamani-Grammar.md" remains useful historical/design material, but material becomes language syntax only through the normal promotion process:

proposal
   ↓
semantic design
   ↓
AST contract
   ↓
formal specification
   ↓
canonical grammar
   ↓
implementation
   ↓
IR integration
   ↓
tests
   ↓
stable feature

---

4. Existing Macro Files

The existing macro directory already contains the following components:

grammar/macros/
├── README.md
├── macros.g4
├── declarations.g4
├── invocations.g4
├── parameters.g4
├── expansion.g4
├── hygiene.g4
├── token-stream.g4
├── syntax-tree.g4
├── diagnostics.g4
└── safety.g4

These files must remain separate where their ownership is meaningful.

They must not become independent languages.

Their responsibilities are:

File| Sole primary responsibility
"macros.g4"| Macro grammar composition and central macro syntax
"declarations.g4"| Macro declaration syntax
"invocations.g4"| Macro invocation syntax
"parameters.g4"| Macro parameter syntax
"token-stream.g4"| Token-tree/token-stream structural syntax
"syntax-tree.g4"| Explicit syntax-tree-oriented macro syntax
"expansion.g4"| Syntax associated with explicit expansion controls, where specified
"hygiene.g4"| Syntax for explicitly exposed hygiene controls, if any
"diagnostics.g4"| Macro-specific diagnostic syntax/metadata where language syntax requires it
"safety.g4"| Macro safety-related source declarations/constraints where such syntax is specified
"README.md"| Complete subsystem contract

The grammar files do not implement the behavior named by their filenames.

For example:

"hygiene.g4" does not implement hygiene.

"expansion.g4" does not expand macros.

"diagnostics.g4" does not generate compiler diagnostics.

"token-stream.g4" does not execute token transformations.

---

5. Ownership

5.1 This directory owns

"grammar/macros/" owns source syntax for:

- macro declarations;
- macro visibility;
- macro names;
- macro parameters;
- macro parameter patterns;
- optional parameter types where specified;
- parameter defaults;
- macro bodies;
- macro invocation syntax;
- macro invocation paths;
- macro invocation delimiters;
- invocation arguments;
- token-tree syntax where explicitly supported;
- syntax-tree quotation syntax where explicitly supported;
- syntax-level macro composition;
- macro-specific source attributes where formally specified;
- macro-specific syntactic extension points.

---

5.2 This directory does not own

This directory does not own:

- lexer implementation;
- token enumeration;
- token allocation;
- source-file loading;
- AST storage;
- AST node allocation;
- "NodeId" allocation;
- source-span implementation;
- symbol tables;
- name resolution;
- module resolution;
- import resolution;
- visibility checking implementation;
- type checking;
- effect checking;
- capability checking;
- resource analysis;
- macro lookup;
- macro selection;
- macro expansion;
- expansion execution;
- hygiene implementation;
- provenance implementation;
- compile-time execution;
- arbitrary host-code execution;
- filesystem access;
- network access;
- package installation;
- process spawning;
- hardware discovery;
- device selection;
- CPU selection;
- GPU selection;
- FPGA selection;
- ASIC selection;
- QPU selection;
- simulator selection;
- physical qubit assignment;
- routing;
- scheduling;
- calibration;
- QEC;
- ZQN;
- resilience;
- optimization;
- target lowering;
- runtime execution;
- canonical IR construction.

This separation is mandatory.

---

6. Domain Neutrality

Macros are a general source-language mechanism.

The macro grammar must not need separate implementations for:

- classical computing;
- quantum computing;
- hybrid computing;
- HDL;
- hardware/software co-design;
- AI;
- tensor computing;
- numerical computing;
- distributed computing;
- networking;
- security;
- embedded computing;
- accelerators;
- HPC;
- future domains.

A macro may generate syntax belonging to any of those domains.

The macro grammar remains domain-neutral.

For example:

prepare!(register)

is syntactically a macro invocation.

The macro grammar does not determine whether "register" eventually represents:

- classical storage;
- a tensor;
- a quantum register;
- hardware resources;
- distributed state;
- an accelerator buffer.

That is determined downstream.

---

7. Canonical Shared Syntax

Macro grammar components must reuse repository-wide syntax.

They must use the canonical definitions of:

- identifiers;
- names;
- paths;
- attributes;
- modifiers;
- visibility;
- blocks;
- expressions;
- types;
- literals;
- punctuation;
- source locations.

The macro subsystem must not create alternative definitions for these concepts.

For example, if "core/paths.g4" owns qualified paths, "invocations.g4" must consume that path abstraction rather than creating a competing "macroQualifiedPath".

Likewise, macro arguments must use canonical expression syntax rather than creating a second expression language.

---

8. Macro Declaration Syntax

The conceptual declaration form is:

visibility? macro name genericParameters? (parameters?) body

For example:

macro build(value) {
    ...
}

or:

macro build<T>(value: T) {
    ...
}

or:

macro build<T>(value: T = default_value) {
    ...
}

The exact accepted syntax is determined by the canonical grammar and specification.

A declaration must preserve:

- declaration identity;
- source name;
- visibility;
- generic parameter structure;
- parameter ordering;
- parameter names;
- optional parameter types;
- defaults;
- body;
- source spans;
- source provenance.

The parser does not resolve any of these semantically.

---

9. Macro Names

Macro names use the canonical Zamani identifier model.

The macro grammar must not invent a second identifier syntax.

A macro may be referenced through a qualified canonical path.

Conceptually:

build!(x)

math::build!(x)

domain::subdomain::build!(x)

There is no grammar-defined namespace-depth limit.

The implementation must not convert namespace depth into a language maximum.

---

10. Macro Invocation

The canonical conceptual invocation is:

macroPath ! ( argumentList? )

Examples:

build!()

build!(value)

build!(a, b, c)

module::build!(value)

The "!" is a syntax marker.

It does not itself mean:

- execute immediately;
- execute during lexing;
- execute during parsing;
- execute host code;
- execute Rust;
- access the filesystem;
- access the network;
- spawn processes;
- choose hardware;
- execute quantum operations;
- bypass security;
- bypass semantic analysis.

---

11. Macro Arguments

Macro arguments reuse canonical Zamani expression syntax wherever the language permits.

Conceptually:

argumentList
    : expression (COMMA expression)*
    ;

The actual implementation must consume the canonical expression rule rather than duplicate it.

Arguments can therefore naturally contain future Zamani constructs.

Examples may include:

build!(x + y)

build!(tensor)

build!(quantum_register)

build!(requires capability("gpu.compute"))

where those expressions are valid under the relevant canonical language rules.

The macro grammar does not determine whether an argument is computationally meaningful.

---

12. No Fixed Argument Count

The grammar must use repetition rather than enumerating argument positions.

Forbidden architectural pattern:

argument1
argument2
argument3
...
argumentN

Required conceptual pattern:

argument (separator argument)*

There is therefore no grammar-defined universal maximum argument count.

An implementation may enforce configurable compilation/resource safeguards.

Such safeguards are not language semantics.

---

13. No Fixed Parameter Count

The same principle applies to macro parameters.

Do not define a fixed number of parameter positions.

Do not introduce a language constant representing a maximum parameter count.

Parameter lists must use grammar repetition.

The actual compiler may protect itself through configurable resource policies, but those policies must remain outside the language grammar.

---

14. No Fixed Macro Count

The language does not define a universal maximum number of macro declarations.

A source program can contain as many macro declarations as its available resources permit.

Compiler-side protection may include configurable:

- memory budgets;
- compilation budgets;
- expansion budgets;
- time budgets;
- generated-structure budgets;
- diagnostic budgets.

These are implementation/resource policies.

They must not become syntax restrictions.

---

15. No Artificial Expansion Ceiling

The grammar must not encode an expansion ceiling.

Do not encode grammar-level constructs equivalent to:

MAX_EXPANSION_DEPTH
MAX_EXPANSION_SIZE
MAX_MACROS
MAX_MACRO_PARAMETERS

as universal language limits.

The compiler may have configurable safeguards for hostile or pathological input.

Those safeguards must be:

1. configurable;
2. documented;
3. observable;
4. diagnosable;
5. independent of language meaning;
6. separable from target hardware limits;
7. testable;
8. compatible with larger available resources.

A resource safeguard is not a grammar capacity.

---

16. Important Clarification About "MAX_*" References

The names of prohibited capacity constants may appear in documentation describing what must not be implemented, validation rules, negative tests, or audits.

That does not make those names language limits.

The prohibition concerns actual language/compiler architecture such as:

MAX_QUBITS = ...
MAX_THREADS = ...
MAX_DEVICES = ...

being used to define universal language capacity.

Documentation may explicitly mention such names to detect and prevent their accidental introduction.

The "hardcoding-audit" validation must therefore distinguish:

documentation describing prohibited hard-coding

from:

actual executable or grammar capacity enforcement

A naïve text search that reports every mention of "MAX_*" as a violation is insufficient.

---

17. Recursion

Macro recursion is a compiler-expansion concern, not a grammar capacity.

The parser may parse nested macro invocations.

For example:

outer!(inner!(value))

The parser does not determine whether expansion terminates.

The expansion system must independently manage:

- recursion;
- expansion ordering;
- cycle detection;
- expansion budgets;
- generated-node budgets;
- time budgets;
- memory budgets.

The grammar remains independent of those policies.

---

18. Expansion

Macro expansion is downstream from parsing.

The source pipeline is:

macro invocation
        ↓
AST macro invocation
        ↓
name/module resolution
        ↓
macro resolution
        ↓
argument binding
        ↓
expansion
        ↓
hygiene
        ↓
provenance
        ↓
semantic analysis

The grammar recognizes the invocation.

It does not perform expansion.

The macro expansion implementation must not require a grammar edit every time a new Zamani domain is introduced.

That property is essential for POCO-REAF.

---

19. Macro Expansion Must Not Bypass Semantic Analysis

Generated code is still Zamani code.

After expansion, generated constructs must pass the appropriate compiler phases.

Expansion must not automatically grant permission to:

- use undeclared names;
- violate types;
- violate ownership;
- violate effects;
- bypass capabilities;
- exceed resource requirements;
- access unauthorized resources;
- violate security policy;
- select unavailable hardware;
- bypass quantum validation;
- bypass HDL validation.

The architectural rule is:

generated syntax
      ↓
ordinary Zamani validation

not:

generated syntax
      ↓
trusted executable result

---

20. Hygiene

Macro hygiene is a compiler/semantic property.

It cannot be implemented merely by grammar productions.

The intended pipeline is:

parse
  ↓
AST
  ↓
resolve
  ↓
expand
  ↓
hygiene
  ↓
provenance
  ↓
semantic analysis

The grammar must preserve sufficient source structure for hygiene implementation.

The implementation must prevent unintended identifier capture unless the language specification explicitly defines capture behavior.

Generated bindings must not accidentally:

- capture caller bindings;
- shadow caller bindings unexpectedly;
- become captured by caller bindings;
- corrupt lexical scope;
- bypass visibility rules.

---

21. Explicit Hygiene Syntax

"hygiene.g4" may define syntax only if the language specification explicitly exposes source-level hygiene controls.

Such syntax must have a complete contract containing:

- lexical representation;
- grammar rule;
- AST mapping;
- semantic meaning;
- hygiene behavior;
- provenance behavior;
- diagnostics;
- compatibility;
- positive tests;
- negative tests;
- boundary tests;
- scalability tests.

If no user-visible hygiene syntax is specified, "hygiene.g4" must remain a documented integration boundary rather than inventing syntax merely to make the file non-empty.

---

22. Token Trees

The existing "token-stream.g4" and "syntax-tree.g4" components provide a structured path for macros that require syntax-level token/tree manipulation.

Token trees are source structure.

They are not arbitrary executable host programs.

A token tree may contain constructs from any Zamani domain.

For example, a macro token tree may ultimately contain:

quantum operation

or:

hdl module

or:

parallel computation

without requiring the macro grammar to understand the complete semantics of those domains.

---

23. Balanced Delimiters

Token-tree syntax must preserve balanced source delimiters.

Supported delimiter forms must use the canonical lexical tokens.

The grammar must not create separate delimiter tokens for macros.

The structural representation must support arbitrary nesting subject to available resources and compiler safeguards.

No grammar-level fixed nesting depth is permitted.

---

24. Token-Tree Ownership

"token-stream.g4" owns token-tree structure.

It must not own:

- macro resolution;
- expansion;
- hygiene;
- semantic analysis;
- AST construction policy;
- target selection.

"syntax-tree.g4" may consume token-tree structure when syntax-tree operations are part of the language specification.

Neither file may become a second parser for the whole language.

---

25. Macro Body

Macro bodies should reuse canonical Zamani block syntax wherever possible.

Do not create a parallel body language merely for macros.

The conceptual relationship is:

macroBody
    ↓
canonical block/source structure

This allows macro bodies to evolve with Zamani.

A macro body may contain:

- classical statements;
- quantum statements;
- hybrid statements;
- HDL declarations;
- hardware intent;
- resource requirements;
- distributed constructs;
- AI constructs;
- future domain constructs.

The macro grammar should not need to enumerate them all.

---

26. Generic Macro Parameters

Generic parameters must reuse the canonical generic/type-parameter model where possible.

Conceptually:

macro transform<T>(value: T) {
    ...
}

The grammar records generic syntax.

It does not determine:

- type inference;
- specialization;
- monomorphization;
- runtime representation;
- hardware mapping.

Those belong downstream.

---

27. Parameter Defaults

Defaults must use canonical expression syntax.

Conceptually:

macro build(size: Size = default_size) {
    ...
}

The parser records the expression.

The parser does not evaluate it.

Default evaluation/substitution is a compiler semantic operation.

A default expression must not silently execute arbitrary host code.

---

28. Macro Parameter Types

A macro parameter type is source-level macro information unless the language specification explicitly defines another interpretation.

A parameter type does not automatically imply:

- runtime allocation;
- memory allocation;
- CPU allocation;
- GPU allocation;
- qubit allocation;
- physical register allocation;
- FPGA resource allocation;
- network capacity.

For example:

macro build<T>(value: T)

does not mean that "T" has a physical hardware representation at macro-expansion time.

---

29. Macro Resolution

Macro resolution is downstream from parsing.

The conceptual process is:

macro invocation
       ↓
canonical path resolution
       ↓
scope lookup
       ↓
module/package lookup
       ↓
visibility checking
       ↓
candidate discovery
       ↓
parameter matching
       ↓
generic constraint checking
       ↓
macro selection

The grammar must not contain a global macro registry.

The parser must not perform symbol-table lookup.

The grammar must not mutate compiler-global state.

---

30. Modules and Imports

Macro paths must integrate with the existing module system.

The macro subsystem must not invent another import mechanism.

Macro availability must respect:

- module boundaries;
- visibility;
- package boundaries;
- aliases;
- namespaces;
- versions;
- compatibility rules.

If a macro is imported through the normal module system, the macro invocation must use the same canonical name/path semantics.

---

31. AST Contract

The macro grammar must lower into the existing canonical frontend AST.

The existing repository contains macro-expression AST infrastructure.

The macro grammar must therefore not introduce a competing:

MacroAst
MacroInvocationAst
MacroArgumentAst

hierarchy unless the canonical AST specification explicitly changes to require it.

The preferred architecture is:

grammar
   ↓
canonical parser
   ↓
canonical AST
   ↓
macro expression/declaration representation

---

32. Macro Expression AST

A macro invocation represented as an expression must retain enough information to identify:

- the macro name/path;
- argument node references;
- source span;
- ordinary AST metadata;
- expansion/provenance metadata where supported by the canonical AST architecture.

The macro expression represents the source invocation, not an already-expanded computation.

This distinction must remain explicit.

---

33. AST Responsibilities

The AST owns:

- source structure;
- node identity;
- source spans;
- declaration structure;
- expression structure;
- argument references;
- source metadata.

The AST does not own:

- macro lookup;
- macro expansion;
- hygiene algorithms;
- resource allocation;
- target selection;
- hardware state;
- quantum routing;
- scheduling;
- QEC;
- runtime execution.

---

34. Source Spans

Every macro declaration and invocation must remain traceable to source locations.

Diagnostics must be able to identify at least:

macro declaration location
macro invocation location
argument location
generated construct location

where the underlying compiler representation supports those distinctions.

The macro grammar does not implement source spans.

The lexer/parser/AST infrastructure owns source location representation.

---

35. Expansion Provenance

Expansion must preserve ancestry.

Conceptually:

original source
     │
     ▼
macro invocation
     │
     ▼
macro declaration
     │
     ▼
generated syntax
     │
     ▼
nested expansion

Diagnostics must be capable of reporting the relevant expansion chain.

A compiler error originating in generated code should not lose the location at which the programmer invoked the macro.

---

36. Canonical IR Boundary

Macros must not directly create target-specific IR.

The required architecture is:

macro syntax
      ↓
frontend AST
      ↓
macro expansion
      ↓
semantic analysis
      ↓
canonical semantic representation
      ↓
canonical IR

For quantum programs:

macro
  ↓
AST
  ↓
expansion
  ↓
semantic quantum operation
  ↓
quantum::ir
  ↓
optimization
  ↓
routing
  ↓
scheduling
  ↓
QEC / resilience / ZQN where applicable
  ↓
HAL
  ↓
target

There must not be a separate macro-specific quantum IR.

---

37. Quantum Independence

Macro grammar must not contain assumptions about:

- number of qubits;
- physical qubit IDs;
- logical qubit IDs;
- gate sets;
- coupling maps;
- QPU topology;
- QPU vendor;
- calibration data;
- pulse representation;
- QEC code;
- ZQN implementation;
- quantum backend.

A macro such as:

prepare!(q)

does not establish any hardware limit.

Likewise:

entangle!(register)

does not imply a fixed number of qubits.

The generated program is evaluated against actual resource/capability information downstream.

---

38. Generic Quantum Operations

The macro system must remain compatible with the repository's data-driven quantum operation model.

It must not require macros to enumerate gates such as:

H
X
Y
Z
CNOT
...

The canonical quantum operation model should remain capable of representing:

apply H to q
apply custom_gate to q
apply vendor.operation to q
apply operation(parameter) to q

A macro may generate any valid quantum operation syntax without modifying the macro grammar for every new operation.

---

39. Classical Independence

The macro grammar must not hard-code:

- CPU count;
- core count;
- thread count;
- register width;
- cache size;
- memory size;
- SIMD width.

A macro can generate portable classical computation.

Resource realization happens downstream.

For example:

parallel!(work)

does not imply:

8 threads

or:

32 cores

unless those are explicit program requirements rather than grammar assumptions.

---

40. HDL Independence

Macros may generate HDL syntax.

The macro grammar itself must not encode:

- fixed bus widths;
- fixed register widths;
- fixed FPGA capacities;
- fixed LUT counts;
- fixed BRAM counts;
- fixed ASIC dimensions;
- fixed clock frequencies.

For example, a macro generating a parameterized hardware structure remains source-level syntax.

The actual implementation is resolved later.

---

41. Hardware Independence

Macro expansion must never be a hidden hardware-selection mechanism.

A macro must not silently choose:

CPU 0
GPU 1
FPGA device 2
QPU device 3
physical qubit 4

unless the language explicitly contains target-specific syntax and the construct is correctly classified as a target/deployment decision rather than portable source semantics.

The macro grammar must preserve the distinction between:

portable requirement

and:

target realization

---

42. Resource Requirements

Macros may generate resource requirements if those constructs are part of the canonical Zamani language.

Examples:

requires qubits >= n

requires memory >= required_memory

requires capability("tensor.compute")

requires capability("gpu.compute")

requires capability("quantum.measurement")

These are semantic requirements.

They are not grammar-defined hardware limits.

---

43. Requirement vs Implementation Decision

The compiler must distinguish:

Requirement

requires qubits >= n

Capability

requires capability("quantum.measurement")

Preference

prefer quantum accelerator

Implementation decision

map logical resource to physical resource

A macro must not collapse these concepts.

---

44. POCO-REAF

The macro subsystem contributes to:

Program_Once
Compile_Once
Run_Everywhere
Anywhere
Forever

by keeping macros independent of target realization.

The same source-level macro invocation should be able to participate in compilation to:

atom/embedded
    ↓
CPU
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
QPU
    ↓
simulator
    ↓
HPC
    ↓
cluster
    ↓
distributed/cloud
    ↓
future targets

provided that the resulting program's requirements can be satisfied.

The macro grammar must not encode the realization.

---

45. Scalability

Scalability means:

«The language does not establish artificial finite capacity merely because a particular implementation has finite resources.»

The grammar must use unbounded structural constructs where appropriate:

*
+
recursive structures
lists
maps
generic structures
nested blocks
qualified paths
parameterized syntax

Actual implementation limits are governed by available resources and configurable policies.

---

46. Tiny-to-Large Principle

A valid macro system must work for:

macro x() {}

as well as very large macro-enabled programs.

The same language model must not need a different grammar for:

- embedded;
- workstation;
- server;
- accelerator;
- QPU;
- HPC;
- cluster;
- distributed;
- cloud.

Scale is a property of compilation and execution resources, not macro grammar variants.

---

47. Resource Safeguards

A production compiler must be able to defend itself against pathological macro expansion.

Examples include configurable budgets for:

- expansion work;
- generated nodes;
- generated bytes;
- memory;
- compilation time;
- diagnostics;
- recursion/cycle tracking;
- dependency traversal.

These are implementation safeguards.

They must not be represented as universal source-language ceilings.

A resource-exhaustion diagnostic should communicate:

the current compilation policy cannot admit this expansion

rather than falsely claiming:

Zamani does not support this program

when the language itself imposes no such semantic limitation.

---

48. Determinism

Macro expansion must be deterministic where the language specification promises deterministic compilation.

Given the same:

source
+
compiler version
+
language version
+
macro definitions
+
dependencies
+
explicit compilation configuration

the compiler must produce the same semantic result, except where nondeterminism is explicitly part of the language model.

The grammar itself must be deterministic.

Ambiguous macro syntax must not be resolved through accidental parser behavior.

---

49. No Hidden Side Effects During Parsing

Parsing a macro must not:

- execute it;
- access the filesystem;
- access the network;
- invoke external processes;
- query hardware;
- mutate global compiler state;
- perform quantum execution;
- perform GPU execution;
- access secrets;
- perform cryptographic operations.

Parsing is structural.

Expansion is a controlled compiler phase.

Execution is a later phase.

---

50. Compile-Time Execution Boundary

If Zamani later exposes compile-time computation, it must be explicitly specified.

Compile-time execution must have:

- an explicit semantic boundary;
- capability policy;
- deterministic behavior where promised;
- resource policy;
- security policy;
- provenance;
- diagnostics;
- cancellation;
- failure semantics;
- compatibility rules.

A macro must never acquire unrestricted host privileges merely because it is called during compilation.

---

51. No Arbitrary Rust Escape Hatch

The macro system must not require embedded Rust actions inside ANTLR grammar.

Do not introduce grammar actions equivalent to:

{ arbitrary Rust }

for macro expansion.

ANTLR grammar must describe syntax.

Compiler behavior belongs in Rust implementation modules.

The Rust implementation must remain compatible with:

Rust 2021
Rust 1.97 / 1.97.1

and must not require "unsafe".

---

52. Safe Rust Requirement

All Zamani-owned Rust implementation associated with the macro pipeline must use safe Rust.

No "unsafe" is permitted for:

- macro parsing;
- macro resolution;
- macro expansion;
- token-tree manipulation;
- hygiene;
- provenance;
- diagnostics;
- resource accounting.

If a future external backend internally requires unsafe implementation details, that implementation detail must remain outside the Zamani macro grammar and must not leak into its source-language contract.

---

53. Security Boundary

Macros are compiler inputs and therefore untrusted input.

Macro infrastructure must not implicitly grant:

- filesystem permissions;
- network permissions;
- process permissions;
- secret access;
- credential access;
- arbitrary code execution;
- hardware-control privileges.

Security-sensitive operations must pass through the repository's established capability/security model.

Macro syntax alone is never authorization.

---

54. Macro Safety

"safety.g4" must describe only source-level safety constructs that are explicitly part of the language.

It must not become a second semantic analyzer.

The safety contract must integrate with:

grammar/security/
grammar/resources/
grammar/effects/
grammar/spec/
semantic analysis

A macro cannot bypass safety because the generated syntax originated inside a macro.

---

55. Diagnostics

Macro diagnostics must distinguish:

1. declaration errors;
2. invocation errors;
3. path resolution errors;
4. argument errors;
5. parameter mismatch;
6. generic constraint errors;
7. expansion errors;
8. recursion/cycle errors;
9. hygiene errors;
10. provenance errors;
11. generated-code semantic errors;
12. resource-policy failures;
13. security-policy failures.

The existing macro diagnostic vocabulary, including codes such as:

ZMN-MACRO-EXPANSION
ZMN-MACRO-HYGIENE
ZMN-MACRO-RECURSION

must remain subordinate to the repository-wide diagnostic specification.

Diagnostic codes must not be duplicated across unrelated files.

---

56. Diagnostic Provenance

A diagnostic originating from generated code should provide enough information to understand both:

where generated code failed

and:

which macro invocation produced it

where the compiler has that provenance.

A useful diagnostic chain is conceptually:

error in generated expression
    ↓
generated by macro `foo`
    ↓
invoked at source location X
    ↓
macro declared at source location Y

The exact presentation belongs to the diagnostic subsystem.

---

57. Expansion Errors

Expansion errors must be distinguishable from syntax errors.

For example:

invalid macro invocation

is different from:

macro resolved but expansion failed

which is different from:

expanded code is syntactically invalid

which is different from:

expanded code is syntactically valid but semantically invalid

This separation is required for production diagnostics.

---

58. Token-Tree Errors

Malformed token-tree structure must produce structural diagnostics such as:

- unmatched delimiter;
- unexpected delimiter;
- incomplete token tree;
- invalid token-tree element;
- malformed syntax quotation.

The parser must recover where the canonical error-recovery strategy allows it.

---

59. Macro Expansion and Other Domains

The macro subsystem must integrate with:

grammar/classical/
grammar/quantum/
grammar/hybrid/
grammar/hdl/
grammar/hardware/
grammar/resources/
grammar/distributed/
grammar/ai/
grammar/data/
grammar/networking/
grammar/security/
grammar/compile/
grammar/execution/

The integration rule is:

macro
   ↓
canonical syntax
   ↓
domain syntax
   ↓
domain semantic validation

The macro subsystem must not duplicate those domains.

---

60. Classical Example

A macro may generate classical syntax:

vectorize!(operation)

The macro subsystem recognizes the invocation.

The classical subsystem owns the meaning of generated vector operations.

The macro subsystem does not decide:

- SIMD width;
- CPU architecture;
- vector register width;
- GPU execution;
- accelerator selection.

---

61. Quantum Example

A macro may generate:

prepare_state!(q)

The macro subsystem handles only macro syntax.

Quantum semantic infrastructure determines what the generated operation means.

Quantum compilation determines:

- decomposition;
- routing;
- scheduling;
- resource realization;
- QEC;
- resilience;
- backend compatibility.

The macro subsystem remains unchanged.

---

62. HDL Example

A macro may generate parameterized HDL:

pipeline!(stage_count, operation)

The macro grammar does not establish:

stage_count <= fixed number

nor does it establish a fixed FPGA or ASIC capacity.

Hardware synthesis and resource analysis determine feasibility.

---

63. Distributed Example

A macro may generate distributed computation.

The macro grammar does not establish:

N nodes

as a universal language limit.

Node count is a program/resource/deployment property.

---

64. AI Example

A macro may generate tensor/model/dataflow syntax.

The macro subsystem does not hard-code:

- tensor rank;
- accelerator count;
- GPU count;
- memory capacity;
- model size.

Those are determined by the semantic/resource/target pipeline.

---

65. Interoperability

Macros may generate or construct source representations associated with interoperability features.

Examples can include:

- C;
- C++;
- Rust;
- Python;
- WebAssembly;
- OpenQASM;
- QIR;
- HDL;
- other formally supported representations.

However, those formats are interoperability targets.

They are not the canonical Zamani semantic model.

The macro subsystem must not make a foreign format the semantic authority.

---

66. Dialects

Macros must integrate with "grammar/dialects/".

A dialect must not silently become a macro-defined language.

A dialect that extends syntax must have:

- name;
- version;
- ownership;
- syntax contract;
- semantic contract;
- AST mapping;
- IR mapping;
- compatibility;
- feature status.

Macros cannot silently register new universal keywords.

---

67. Macro-Generated Dialect Syntax

If a macro generates dialect syntax, the dialect must still be known and validated by the compiler.

The macro does not grant automatic permission to introduce unknown syntax.

The correct model is:

dialect declaration
       ↓
dialect availability
       ↓
macro expansion
       ↓
dialect-aware parsing/semantic processing

or the equivalent architecture defined by the canonical parser.

---

68. Metaprogramming Integration

"grammar/metaprogramming/" is the broader compile-time/metaprogramming subsystem.

The macro subsystem must not duplicate its responsibilities.

The relationship is:

grammar/macros/
    │
    ├── macro declarations
    ├── macro invocations
    ├── token/syntax structures
    └── macro-specific syntax
             │
             ▼
grammar/metaprogramming/
             │
             ├── reflection
             ├── quotation
             ├── code generation
             ├── compile-time facilities
             └── type-level facilities

Where both subsystems address the same syntax, the authority must be explicitly assigned rather than duplicated.

---

69. Existing Compiler Integration

The repository already contains compiler-side macro infrastructure, including:

src/compiler/macro_engine.rs

and broader metaprogramming infrastructure including:

src/toolchain/meta_programming.rs

These implementations must not be treated as interchangeable semantic authorities.

The production architecture must define one clear responsibility for each.

The grammar layer specifies source syntax.

The compiler macro engine owns actual expansion policy and behavior.

The broader metaprogramming layer may provide compiler/toolchain-level facilities that are not themselves equivalent to source-language macro syntax.

---

70. Important Existing Implementation Boundary

The existing compiler macro implementation has configuration concepts for controlling expansion work/size/depth.

Those are useful as implementation safeguards, but they must not be confused with language limits.

The production contract is therefore:

language:
    no artificial macro capacity ceiling

implementation:
    configurable protection against resource exhaustion

Changing an implementation budget must not require changing the language grammar.

---

71. Macro Implementation Must Not Pretend Expansion Is Complete

If an existing compiler API currently returns source text or performs limited substitution, the grammar README must not claim that full production token-tree/hygienic expansion is already implemented.

Conformance must distinguish:

syntax specified

from:

syntax parsed

from:

expansion implemented

from:

hygiene implemented

from:

semantic validation implemented

from:

IR integration implemented

This information belongs in implementation conformance reporting.

---

72. Feature Status

Macro features must use the repository's canonical statuses:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

A macro feature must not be called "stable" merely because a ".g4" file exists.

Production status requires end-to-end evidence.

---

73. Feature Completion Contract

Every independently maintained macro grammar component must have a completion contract containing:

Purpose
Status
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Syntax
AST Contract
Semantic Contract
IR Contract
Compiler Integration
Runtime Integration
Diagnostics
Security
Resource Policy
Scalability
Determinism
Compatibility
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Hard-Coding Audit
Completion Criteria

This is mandatory.

---

74. "macros.g4" Completion Contract

"macros.g4" is complete only when:

- composition ownership is defined;
- canonical lexer vocabulary is used;
- declaration integration is defined;
- invocation integration is defined;
- parameter integration is defined;
- token-tree integration is defined;
- expression integration is defined;
- block integration is defined;
- AST mapping is defined;
- diagnostics are defined;
- no duplicate root grammar exists;
- no fixed capacity exists;
- ANTLR generation succeeds;
- parser integration is validated.

---

75. "declarations.g4" Completion Contract

Complete only when:

- macro declaration syntax is fully specified;
- visibility is defined;
- name ownership is defined;
- generic integration is defined;
- parameter integration is defined;
- body integration is defined;
- canonical declaration syntax is reused;
- AST mapping is specified;
- semantic mapping is specified;
- diagnostics are specified;
- positive/negative/boundary tests exist.

---

76. "parameters.g4" Completion Contract

Complete only when:

- parameter syntax is defined;
- names use canonical identifiers;
- optional types use canonical type syntax;
- defaults use canonical expressions;
- parameter ordering is preserved;
- no fixed parameter count exists;
- AST mapping exists;
- semantic validation exists;
- diagnostics exist;
- tests exist.

---

77. "invocations.g4" Completion Contract

Complete only when:

- invocation syntax is defined;
- macro path integration is defined;
- canonical "!" token is used;
- argument integration is defined;
- expression integration is defined;
- nested invocations are supported;
- qualified paths are supported;
- no namespace depth ceiling exists;
- no fixed argument count exists;
- AST mapping exists;
- diagnostics exist;
- tests exist.

---

78. "token-stream.g4" Completion Contract

Complete only when:

- token-tree structure is specified;
- balanced delimiters are handled;
- canonical tokens are reused;
- arbitrary structural nesting is supported subject to resources;
- no fixed nesting constant is encoded;
- token-tree AST/representation ownership is documented;
- expansion integration is documented;
- hygiene integration is documented;
- diagnostics exist;
- tests exist.

---

79. "syntax-tree.g4" Completion Contract

Complete only when:

- syntax-tree operations are explicitly specified;
- token-tree ownership is respected;
- AST mapping is explicit;
- no second AST hierarchy is introduced;
- expansion integration is explicit;
- hygiene/provenance integration is explicit;
- security boundaries are explicit;
- tests exist.

---

80. "expansion.g4" Completion Contract

Complete only when:

- every expansion-related syntax form is specified;
- no expansion implementation is embedded in grammar;
- resource-policy integration is defined;
- provenance integration is defined;
- hygiene integration is defined;
- diagnostics are defined;
- AST mapping is defined;
- semantic mapping is defined;
- compatibility is defined;
- tests exist.

If there is no user-visible expansion syntax, this file must remain limited to formally justified extension points rather than inventing commands such as "expand_now".

---

81. "hygiene.g4" Completion Contract

Complete only when:

- explicit hygiene syntax is actually specified;
- syntax is unambiguous;
- AST mapping exists;
- semantic meaning exists;
- compiler hygiene behavior exists;
- provenance behavior exists;
- diagnostics exist;
- compatibility exists;
- tests exist.

If no explicit hygiene syntax exists, hygiene remains a compiler concern and this grammar component must not fabricate syntax.

---

82. "diagnostics.g4" Completion Contract

Complete only when:

- macro-specific source syntax requiring diagnostics metadata is defined;
- diagnostic identifiers are governed centrally;
- no duplicate diagnostic authority exists;
- source spans are preserved;
- expansion provenance is preserved;
- diagnostics can distinguish syntax, resolution, expansion, hygiene, and semantic failures.

---

83. "safety.g4" Completion Contract

Complete only when:

- source-level safety syntax is explicitly specified;
- safety declarations map into the canonical semantic safety/capability model;
- grammar does not implement authorization;
- no unsafe Rust is required;
- no hardware privilege is granted implicitly;
- diagnostics are defined;
- tests are defined.

---

84. Expression Integration

The existing repository contains a dedicated macro-expression grammar integration under:

grammar/expressions/macros.g4

This must remain consistent with:

grammar/macros/invocations.g4

There must be one semantic macro invocation concept.

The expression layer should consume the macro invocation rule rather than redefine it.

Conceptually:

canonical expression
       │
       ├── ordinary expression forms
       │
       └── macro expression
                  │
                  ▼
          macro invocation

No duplicate macro-expression syntax is permitted.

---

85. Statement Integration

If macros are valid as statements, the statement grammar must consume the canonical macro construct.

Do not create separate semantically equivalent forms such as:

macroStatement
macroInvocationStatement
macroExecutionStatement

unless their semantics genuinely differ.

A syntactic form must have one owner.

---

86. Declaration Integration

Macro declarations must integrate with the canonical declaration dispatcher.

The root grammar should eventually compose:

declaration
    ├── ordinary declarations
    ├── function declarations
    ├── type declarations
    ├── module declarations
    ├── domain declarations
    └── macro declarations

The root composition remains owned by:

grammar/Zamani.g4

---

87. Lexer Integration

The macro subsystem must consume the canonical lexer vocabulary.

It must not create an independent lexer.

The lexer owns:

- "!";
- identifiers;
- delimiters;
- commas;
- generic punctuation;
- keywords;
- literals;
- whitespace;
- comments.

Macro grammar components reference those tokens.

They do not redefine them.

---

88. Keyword Integration

The word:

macro

must be governed by the canonical keyword registry.

If "macro" is a keyword, it must be registered centrally.

If the language later changes the spelling, all macro grammar components consume the canonical token rather than independently declaring a replacement.

---

89. "!" Token Integration

The macro invocation marker must use the canonical token representation.

It must not conflict with:

- logical operators;
- factorial-like syntax;
- other punctuation;
- future language constructs.

The ambiguity policy belongs to the lexer/operator/grammar validation system.

---

90. "QuestionMark" / "Question" and "Ampersand" / "BitAnd"

The repository already identifies token naming/collision issues in the broader lexer architecture.

Macro grammar must not reintroduce those ambiguities.

The macro subsystem must consume canonical token names after the lexer authority resolves them.

No macro grammar file may define aliases merely to work around unresolved lexer duplication.

---

91. Source Compatibility

Existing accepted macro syntax must not be changed gratuitously.

Before modifying syntax:

1. identify current accepted forms;
2. identify canonical specification;
3. identify parser behavior;
4. identify AST representation;
5. identify tests;
6. determine whether change is breaking;
7. define migration if required.

Do not rename existing macro grammar files without necessity.

---

92. Compatibility With Existing "grammar/antlr/"

The repository currently contains ANTLR-related parser material under:

grammar/antlr/

and macro-related parser integration.

This does not automatically make "grammar/antlr/" a second language authority.

The production architecture must establish one canonical composition path.

The intended rule is:

grammar/Zamani.g4
        ↓
canonical ANTLR composition
        ↓
macro grammar components

Any generated or legacy parser grammar under "grammar/antlr/" must be explicitly classified as:

- generated;
- implementation support;
- compatibility;
- legacy;
- deprecated;

rather than silently competing with "grammar/Zamani.g4".

---

93. No Second Root Grammar

The repository must not maintain two independently authoritative root grammars.

In particular, avoid a situation where:

grammar/Zamani.g4

and:

grammar/antlr/ZamaniParser.g4

both independently define the complete Zamani language.

If generated/parser-support grammar files are retained, their relationship to the canonical root must be explicit.

---

94. Integration With "grammar/DESIGN.md"

This subsystem follows these repository-wide principles:

- one language;
- deterministic parsing;
- canonical lexical vocabulary;
- no artificial hardware limits;
- target-independent source semantics;
- canonical frontend AST;
- semantic/resource/capability separation;
- canonical "quantum::ir";
- no duplicate IR;
- safe Rust;
- explicit compatibility;
- traceable source provenance.

Any future macro proposal that violates these principles must not be accepted merely because it is convenient to implement.

---

95. Integration With "grammar/specification/"

The human-readable normative specification must define:

- macro existence;
- declaration syntax;
- invocation syntax;
- parameter syntax;
- body syntax;
- token-tree syntax if standardized;
- hygiene semantics;
- expansion semantics;
- visibility;
- diagnostics;
- compatibility.

This README describes subsystem architecture.

It must not silently replace normative specification text.

---

96. Integration With "grammar/spec/"

A formal macro specification should exist under the formal specification hierarchy when the repository's specification organization provides the appropriate location.

That contract should cover:

syntax
AST
semantics
expansion
hygiene
provenance
diagnostics
security
resource policy
compatibility
tests

The exact filename must follow the existing repository organization rather than creating a duplicate specification authority.

---

97. Integration With "grammar/validation/"

The macro subsystem must be validated by:

grammar/validation/

Relevant validation includes:

- ambiguity;
- duplicate tokens;
- unreachable rules;
- left recursion where applicable;
- parser conflicts;
- source-span coverage;
- AST coverage;
- semantic coverage;
- IR coverage;
- hard-coding;
- scalability;
- determinism;
- compatibility.

The macro README must not duplicate those validation algorithms.

---

98. Hard-Coding Audit

The macro subsystem must pass a hard-coding audit.

The audit must detect actual capacity restrictions such as:

fixed macro count
fixed parameter count
fixed argument count
fixed token-tree depth
fixed expansion capacity
fixed source size
fixed generated-node capacity
fixed hardware capacity

The audit must distinguish these from ordinary:

- examples;
- documentation;
- negative tests;
- explanatory references;
- diagnostic examples.

The goal is to prevent actual architectural ceilings, not to ban words from documentation.

---

99. Hardware Hard-Coding

The macro subsystem must contain no language-level assumptions equivalent to:

maximum CPUs
maximum GPUs
maximum FPGAs
maximum QPUs
maximum qubits
maximum nodes
maximum memory
maximum threads
maximum tensor rank
maximum register width
maximum network size
maximum device count

A macro can generate a program that requests resources.

The compiler determines whether the available target can satisfy them.

---

100. Compiler Safeguards vs Language Semantics

This distinction is mandatory.

Language semantics

The language permits arbitrary structural macro programs.

Compiler safeguard

This compilation invocation has a configured resource budget.

A safeguard failure must not redefine the language.

For example:

configured compilation budget exhausted

is not equivalent to:

Zamani macro syntax has a universal maximum.

---

101. Memory Scaling

Macro expansion can consume substantial memory.

The compiler must therefore use resource-aware data structures and configurable admission controls.

However, the grammar itself must not establish a memory ceiling.

The architecture must permit expansion sizes appropriate to available resources.

---

102. Streaming and Incremental Expansion

Where practical, compiler implementation may support:

- incremental parsing;
- incremental expansion;
- streaming token processing;
- lazy expansion;
- demand-driven expansion;
- cached expansion;
- parallel expansion where deterministic ordering can be preserved.

These are implementation strategies.

They must not change macro language syntax.

---

103. Parallel Macro Expansion

Independent macro expansions may be processed concurrently by the compiler when semantic dependencies allow.

The language result must remain deterministic where deterministic compilation is promised.

Parallel implementation must not change source meaning.

No source-language macro syntax should specify an implementation-specific compiler thread count.

---

104. Caching

Expansion results may be cached if the relevant inputs are stable.

A valid cache key may need to account for:

- macro identity;
- macro definition;
- arguments;
- language version;
- macro environment;
- relevant dependencies;
- compilation configuration;
- feature/dialect configuration.

Caching must not change semantics.

---

105. Reproducibility

Macro expansion must support reproducible builds.

The compiler should be able to identify the inputs influencing expansion.

Generated output must be traceable to:

- source;
- macro definition;
- macro invocation;
- dependencies;
- compiler/language version;
- relevant configuration.

---

106. Dependency Changes

A macro definition changing must invalidate affected expansion results.

The grammar itself does not implement cache invalidation.

The compiler/build system owns dependency tracking.

The macro contract must nevertheless make the semantic inputs identifiable enough for such tracking.

---

107. Error Recovery

Macro grammar must integrate with canonical parser error recovery.

Malformed macro syntax must not corrupt unrelated source parsing more than necessary.

Recovery should preserve enough structure for subsequent diagnostics.

Error recovery must not silently reinterpret malformed macro syntax as unrelated program constructs when that would hide the actual error.

---

108. Ambiguity

Macro syntax must have deterministic disambiguation.

Potential conflicts include:

- "!";
- generic delimiters;
- paths;
- ordinary function calls;
- attributes;
- blocks;
- token trees;
- expression syntax;
- statement syntax.

These must be resolved centrally through the grammar validation policy.

---

109. Ordinary Function Call vs Macro Invocation

The language must preserve a clear syntactic distinction.

Conceptually:

function(value)

is a function call.

macro!(value)

is a macro invocation.

The grammar must not make the two forms semantically interchangeable.

---

110. Macro Invocation Inside Expressions

Where macro expressions are supported:

let value = make_value!(input);

must map through the canonical expression system.

The expression grammar must not create an alternate AST hierarchy.

---

111. Nested Macro Invocations

Nested invocations must be supported where syntactically valid:

outer!(inner!(value))

No fixed nesting depth belongs in the grammar.

The compiler may protect itself through configurable resource policy.

---

112. Macro Expansion Order

Expansion order is a compiler semantic concern.

The grammar must not encode an arbitrary execution ordering.

The compiler must specify:

- dependency order;
- nested expansion order;
- cycle behavior;
- visibility timing;
- generated-definition availability.

The resulting behavior must be deterministic where promised.

---

113. Recursive Macros

Recursive macro definitions may be syntactically representable.

Whether a recursive expansion terminates is a compiler concern.

The compiler must detect or control pathological recursion without converting the detection mechanism into a language capacity.

---

114. Cycles

Macro dependency cycles must be handled by semantic/compiler infrastructure.

A cycle must produce a meaningful diagnostic rather than:

- parser failure;
- stack overflow;
- uncontrolled resource exhaustion.

The grammar itself does not resolve cycles.

---

115. Hygiene and Generated Names

Generated names must have a well-defined hygiene model.

The implementation must not rely on accidental string concatenation to ensure uniqueness.

The canonical compiler representation should distinguish generated identifiers by provenance/context as required by the language's hygiene model.

The grammar only provides the source structures from which that representation is derived.

---

116. User-Visible Escaping

If the language provides an explicit mechanism to opt into intentional capture or unhygienic behavior, that mechanism must be formally specified.

It must never be an undocumented compiler trick.

Such a feature requires:

- syntax;
- semantics;
- security implications;
- diagnostics;
- compatibility;
- AST mapping;
- provenance;
- tests.

---

117. Security and Hygiene

Hygiene must not be used as a security substitute.

A hygienic macro can still generate code that:

- requests excessive resources;
- performs unauthorized operations;
- violates semantic constraints;
- uses prohibited capabilities.

Therefore:

hygiene

and:

security/capability validation

remain separate concerns.

---

118. Macro Expansion Into Quantum Code

A macro may generate quantum code.

After expansion:

generated quantum syntax
        ↓
quantum semantic analysis
        ↓
quantum::ir

The macro subsystem does not perform:

- qubit allocation;
- gate decomposition;
- routing;
- scheduling;
- QEC;
- calibration;
- physical mapping.

---

119. Macro Expansion Into HDL

A macro may generate HDL.

After expansion:

generated HDL
    ↓
HDL semantic validation
    ↓
hardware/HDL representation
    ↓
synthesis/lowering

The macro subsystem does not decide:

- FPGA resource allocation;
- ASIC layout;
- clock implementation;
- physical placement.

---

120. Macro Expansion Into Distributed Programs

A macro may generate distributed constructs.

The distributed subsystem determines:

- process/service semantics;
- communication semantics;
- consistency;
- replication;
- placement;
- fault tolerance.

The macro grammar remains unchanged.

---

121. Macro Expansion Into AI/Data Programs

A macro may generate:

- tensors;
- models;
- training pipelines;
- inference;
- data transformations;
- agents.

The AI/data subsystem owns those semantics.

The macro subsystem only performs its source-level role.

---

122. No Framework-Specific Macro Grammar

The macro grammar must not contain framework-specific syntax merely to support:

- CUDA;
- ROCm;
- vendor QPU APIs;
- a specific FPGA vendor;
- a specific AI framework;
- a specific cloud provider;
- a specific cluster scheduler.

Framework integration belongs to interoperability/backend/toolchain layers.

---

123. Future-Proofing

A new computing domain should be able to consume macro-generated syntax without changing the macro subsystem.

The intended model is:

new domain
    ↓
domain grammar
    ↓
domain AST/semantic contract
    ↓
canonical semantic model/IR

Macros remain generic.

This is one of the principal scalability properties of the design.

---

124. Versioning

Macro syntax must participate in the canonical Zamani versioning model.

Breaking changes require:

- version identification;
- compatibility classification;
- migration guidance;
- diagnostics;
- tests.

A macro extension must not silently alter the meaning of existing source programs.

---

125. Deprecation

Deprecated macro syntax must remain documented through the repository's compatibility system.

Deprecation must identify:

- feature;
- introduced version;
- deprecated version;
- replacement;
- removal policy;
- migration behavior.

Do not delete old syntax merely because a newer syntax is preferred unless the compatibility policy permits it.

---

126. Experimental Macro Features

Experimental syntax must be explicitly marked.

It must not be presented as stable language syntax.

Experimental macro features require:

- feature identifier;
- status;
- grammar;
- AST plan;
- semantic plan;
- implementation status;
- compatibility expectations;
- tests.

---

127. Historical Material

Historical macro proposals may remain in:

grammar/Zamani-Grammar.md

or appropriate historical documentation.

Historical material must not silently become parser input.

The canonical macro grammar recognizes only formally accepted syntax.

---

128. Testing Architecture

Macro tests must be divided into:

tests/
├── syntax/
├── macros/
├── negative/
├── boundary/
├── scalability/
├── determinism/
├── compatibility/
└── diagnostics/

The existing repository test organization should be extended rather than duplicated.

---

129. Positive Tests

Positive tests must cover:

- empty macro body where legal;
- declaration;
- visibility;
- parameters;
- generic parameters;
- defaults;
- invocation;
- zero arguments;
- one argument;
- multiple arguments;
- qualified paths;
- nested invocations;
- expression context;
- statement context;
- token trees;
- syntax trees;
- generated classical syntax;
- generated quantum syntax;
- generated HDL syntax;
- generated hybrid syntax;
- generated future-domain syntax through generic structures.

---

130. Negative Tests

Negative tests must cover:

- missing macro name;
- invalid identifier;
- malformed parameter;
- malformed default;
- missing delimiter;
- unmatched delimiter;
- invalid invocation marker;
- invalid path;
- invalid argument separator;
- invalid generic syntax;
- malformed token tree;
- expansion cycle;
- unresolved macro;
- inaccessible macro;
- argument mismatch;
- invalid generated syntax;
- invalid generated semantics;
- hygiene violations;
- resource-policy exhaustion;
- security-policy rejection.

---

131. Boundary Tests

Boundary tests must test structural extremes without declaring them universal limits.

Examples:

- zero parameters;
- many parameters;
- zero arguments;
- many arguments;
- deeply nested source;
- deeply nested token trees;
- large macro bodies;
- large generated structures;
- long paths;
- many nested macro invocations;
- large numbers of independent macro definitions.

Tests must not encode an arbitrary implementation value as the language maximum.

---

132. Scalability Tests

Scalability tests must vary program size according to available resources.

They should establish that increasing:

- macro count;
- parameter count;
- argument count;
- nesting;
- generated structure;
- token-tree size;

does not cause an artificial language ceiling.

If a configured compiler budget is reached, the test must identify that as an implementation/resource-policy result.

---

133. Determinism Tests

Repeated compilation of identical inputs must verify deterministic behavior where promised.

Test:

same source
same macro definitions
same compiler
same configuration

and compare:

- AST;
- expansion;
- generated structure;
- diagnostics;
- semantic result;
- IR.

---

134. Compatibility Tests

Compatibility tests must verify:

- old macro syntax;
- current macro syntax;
- versioned syntax;
- deprecated syntax;
- dialect interactions;
- generated code compatibility.

Breaking changes must be intentional and documented.

---

135. AST Conformance Tests

Every accepted macro grammar production must have an AST mapping.

The test should establish:

grammar rule
      ↓
parser node
      ↓
canonical AST node

No accepted production may terminate in an undefined AST representation.

---

136. Semantic Conformance Tests

Every macro AST representation must have a semantic consumer or an explicitly recorded status.

A grammar production that parses successfully but has no semantic interpretation is not production complete.

---

137. IR Conformance Tests

Macro-generated constructs must ultimately map to canonical semantic/IR representations.

The macro subsystem itself does not own IR generation.

Tests must verify that expansion does not create an unreachable semantic island.

---

138. Source-to-IR Traceability

Production macro conformance should be traceable as:

source macro
   ↓
grammar rule
   ↓
AST node
   ↓
macro resolution
   ↓
expansion
   ↓
generated AST
   ↓
semantic node
   ↓
canonical IR

This makes individual feature completion auditable.

---

139. No Re-Editing Dependency Principle

A macro grammar file is not complete merely because its own text compiles.

Before declaring it complete, its contract must already specify its relationships to:

- lexer;
- core;
- expressions;
- statements;
- declarations;
- modules;
- types;
- AST;
- semantic analysis;
- compiler macro engine;
- diagnostics;
- validation;
- compatibility;
- IR.

This allows downstream implementation to proceed without redesigning the file every time another subsystem is completed.

---

140. Dependency Direction

The preferred dependency direction is:

lexer
  ↓
core syntax
  ↓
macro syntax
  ↓
AST
  ↓
semantic/compiler infrastructure

The macro grammar must not depend on downstream hardware implementations.

Likewise, hardware implementations must not force macro syntax changes unless the language itself intentionally introduces a new source-level feature.

---

141. Integration Matrix

Component| Macro subsystem relationship
"grammar/lexer/"| Supplies canonical tokens
"grammar/core/"| Supplies names, paths, attributes, blocks
"grammar/expressions/"| Supplies canonical argument/expression syntax
"grammar/statements/"| Consumes macro statements where valid
"grammar/declarations/"| Integrates macro declarations
"grammar/types/"| Supplies parameter type syntax
"grammar/modules/"| Resolves macro paths/imports
"grammar/effects/"| Validates generated effectful constructs
"grammar/resources/"| Validates generated resource requirements
"grammar/security/"| Validates generated security/capability requirements
"grammar/quantum/"| Owns generated quantum semantics
"grammar/hdl/"| Owns generated HDL semantics
"grammar/hardware/"| Owns hardware intent
"grammar/classical/"| Owns classical semantics
"grammar/hybrid/"| Owns cross-domain semantics
"grammar/dialects/"| Owns dialect extension contracts
"grammar/metaprogramming/"| Owns broader metaprogramming contracts
"grammar/validation/"| Validates grammar properties
"grammar/compatibility/"| Owns version/migration policy
"src/frontend/ast/"| Owns canonical AST
macro compiler infrastructure| Owns resolution/expansion/hygiene
canonical IR| Owns post-semantic representation

---

142. Completion of the Whole Macro Subsystem

The macro subsystem is production-ready only when all of the following are true:

- [ ] authoritative specification exists;
- [ ] macro syntax has one authority;
- [ ] "Zamani.g4" is the canonical composition root;
- [ ] lexer tokens are canonical;
- [ ] no duplicate macro grammar exists;
- [ ] declaration syntax is complete;
- [ ] parameter syntax is complete;
- [ ] invocation syntax is complete;
- [ ] argument syntax is complete;
- [ ] token-tree syntax is complete where supported;
- [ ] syntax-tree syntax is complete where supported;
- [ ] expansion boundary is defined;
- [ ] hygiene boundary is defined;
- [ ] provenance is defined;
- [ ] diagnostics are defined;
- [ ] AST mapping exists;
- [ ] semantic mapping exists;
- [ ] IR integration exists;
- [ ] compiler integration exists;
- [ ] security boundary exists;
- [ ] resource policy exists;
- [ ] deterministic behavior is specified;
- [ ] compatibility is specified;
- [ ] positive tests exist;
- [ ] negative tests exist;
- [ ] boundary tests exist;
- [ ] scalability tests exist;
- [ ] determinism tests exist;
- [ ] compatibility tests exist;
- [ ] hard-coding audit passes;
- [ ] ANTLR generation succeeds;
- [ ] Rust frontend conformance succeeds;
- [ ] safe-Rust requirement passes;
- [ ] no "unsafe" is required;
- [ ] no target-specific language limits exist.

---

143. Production Acceptance Gate

A macro grammar change is accepted only if all applicable stages pass:

Specification
      ↓
Grammar
      ↓
ANTLR validation
      ↓
Lexer conformance
      ↓
Parser conformance
      ↓
AST conformance
      ↓
Macro resolution
      ↓
Expansion
      ↓
Hygiene
      ↓
Provenance
      ↓
Semantic analysis
      ↓
Canonical IR
      ↓
Diagnostics
      ↓
Security/resource validation
      ↓
Positive tests
      ↓
Negative tests
      ↓
Boundary tests
      ↓
Scalability tests
      ↓
Determinism tests
      ↓
Compatibility tests

A grammar-only test is insufficient for a production macro feature.

---

144. Macro Feature Promotion

A macro feature moves through:

HISTORICAL
    ↓
PROPOSED
    ↓
EXPERIMENTAL
    ↓
SPECIFIED
    ↓
IMPLEMENTED
    ↓
CONFORMANCE TESTED
    ↓
STABLE

A feature must not skip semantic and test validation.

---

145. No Silent Keyword Creation

Macros must not silently create new global Zamani keywords.

A macro may introduce generated identifiers through expansion, but generated syntax must still obey the canonical language grammar.

If a macro system eventually supports syntax extension, that extension must be formally governed by "grammar/dialects/" and the language extensibility specification.

---

146. No Semantic Bypass

Macros must not become a mechanism for bypassing:

- type safety;
- ownership;
- effect checking;
- capability checking;
- resource checking;
- security;
- quantum correctness;
- HDL correctness;
- hardware requirements;
- distributed-system constraints.

Generated code is still subject to normal semantic rules.

---

147. No Target Leakage

The following must never become implicit macro semantics:

CPU identity
GPU identity
FPGA identity
ASIC identity
QPU identity
physical qubit identity
memory-bank identity
network-node identity
accelerator identity

If target-specific information is intentionally exposed, it must travel through the canonical target/resource/capability model.

---

148. Future Hardware

A new hardware family must not require changing:

grammar/macros/

unless the new hardware introduces a genuinely new source-language semantic concept.

For example, introducing a new QPU vendor must not require:

vendor_x_macro.g4

in the core macro grammar.

Vendor integration belongs downstream.

---

149. Future Computing Models

The same rule applies to future computation models.

If Zamani later gains a new computational domain, macro syntax remains generic.

The new domain defines its own:

syntax
AST
semantics
IR
compiler lowering
runtime

and consumes macro-expanded canonical syntax.

---

150. Relationship to Sankofa

Broader Sankofa/meta-programming concepts appearing in "Zamani-Grammar.md" must not automatically become macro syntax.

Concepts such as:

- memory;
- recall;
- learning;
- inference;
- temporal reasoning;
- provenance;
- consensus;
- knowledge;

must be promoted individually through the normal specification/AST/semantic/IR process.

A historical concept is not automatically a parser feature.

---

151. Relationship to Multi-Timeline Concepts

If macro-related multi-timeline functionality is eventually standardized, it must integrate with the canonical execution/timeline model.

The macro grammar must not independently define a timeline runtime.

No fixed number of timelines or branches belongs in the grammar.

---

152. Relationship to Nano Computing

If macro expansion generates nano-domain syntax, the macro subsystem remains generic.

The nano domain owns its semantics.

The macro grammar does not need to know:

- atom count;
- molecule count;
- material count;
- interaction count;
- physical device capacity.

---

153. Performance

The macro grammar should remain structurally simple enough to support efficient parsing.

Avoid unnecessary duplication of canonical expression/type/path rules.

Avoid grammar alternatives that introduce excessive ambiguity.

Avoid semantic predicates that make parsing dependent on compiler-global state.

Performance optimization must preserve deterministic language semantics.

---

154. Memory Safety

All Rust implementation associated with this subsystem must rely on safe Rust abstractions.

Prefer:

- ownership;
- borrowing;
- slices;
- immutable references;
- arenas where safely implemented;
- vectors;
- maps;
- explicit resource accounting.

Do not use "unsafe" merely for optimization.

---

155. Integer and Size Representation

Implementation metadata such as:

- source offsets;
- node counts;
- expansion accounting;
- resource accounting;

must use representations appropriate to the actual implementation requirements.

Do not select a small integer type merely because ordinary programs are small.

Likewise, do not introduce an arbitrary language capacity merely because a particular Rust integer type is convenient.

---

156. Resource Availability

The phrase:

«scalable to infinity»

must be interpreted architecturally as:

«no artificial language-defined ceiling; scale is bounded only by representational reality and resources available to the particular compilation/execution environment.»

No finite physical machine can provide literally infinite memory or computation.

POCO-REAF therefore means that source semantics do not need to be rewritten merely because the target scale changes.

---

157. Build Reproducibility

Macro grammar generation must be reproducible.

CI must validate:

- grammar source;
- generated parser artifacts where applicable;
- grammar imports;
- token vocabulary;
- versioned dependencies;
- Rust 1.97 / 1.97.1 compatibility.

Generated artifacts must not silently become a second source of truth.

---

158. Generated Files

If ANTLR-generated files are committed, their generated status must be explicit.

If they are generated during the build, CI must regenerate and validate them.

Hand-editing generated parser artifacts must not be required as part of ordinary macro grammar maintenance.

---

159. CI Requirements

CI for the macro subsystem should perform at least:

ANTLR grammar validation
ANTLR generation
duplicate-token validation
ambiguity validation
hard-coding audit
scalability audit
Rust formatting
Rust compilation
Rust tests
macro parser tests
AST tests
semantic tests
diagnostic tests
compatibility tests

The exact CI commands belong to the repository build/tooling configuration.

---

160. Rust Version

The macro implementation must support:

Rust 1.97
Rust 1.97.1
Rust 2021 edition

No implementation should require a newer Rust feature without an explicit repository-wide version change.

No "unsafe" code is permitted.

---

161. Documentation Integrity

This README must not claim a feature is implemented merely because:

- a grammar file exists;
- an AST type exists;
- a compiler struct exists;
- a design document describes it.

Implementation status must be derived from actual repository conformance.

"grammar/grammar.md" remains responsible for reporting current implementation status.

---

162. Avoiding Documentation Drift

The following documents must remain consistent:

grammar/DESIGN.md
grammar/README.md
grammar/specification/
grammar/spec/
grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/macros/README.md
grammar/macros/*.g4

However, they do not all have equal authority.

The macro README describes the macro subsystem.

It does not become a competing root specification.

---

163. What Must Never Be Added to Macro Grammar

Do not add universal syntax equivalent to:

use_gpu_0
use_cpu_7
use_qpu_2
use_qubit_31
use_32_threads
use_64gb_memory
use_24gb_vram
use_32bit_register

as a general macro-language requirement.

Do not encode current hardware inventories into source grammar.

---

164. What Is Allowed

Portable requirements may express:

requires qubits >= n

requires memory >= required_memory

requires capability("gpu.compute")

requires capability("quantum.measurement")

requires topology(...)

These express program requirements.

Actual realization is downstream.

---

165. Macro Expansion and Resource Requirements

A macro may generate resource requirements.

For example:

macro quantum_algorithm(register) {
    requires capability("quantum.measurement");
    ...
}

The macro system does not decide whether the current machine satisfies the requirement.

Resource analysis performs that check after expansion.

---

166. Macro Expansion and Effects

A macro may generate effectful operations.

The generated program must be checked by the canonical effect system.

Macro expansion does not grant effect permissions.

---

167. Macro Expansion and Capabilities

A macro may generate capability requirements.

The compiler must verify those requirements through the canonical capability system.

Macro invocation itself is not authorization.

---

168. Macro Expansion and Ownership

Generated code must obey the same ownership/borrowing/reference semantics as handwritten Zamani code.

The macro system must not create a hidden ownership model.

---

169. Macro Expansion and Types

Generated expressions must pass the normal type system.

Macro expansion must not use textual substitution to bypass type correctness.

Where token-tree substitution is used, semantic validation remains mandatory.

---

170. Macro Expansion and Effects

Generated effects must pass normal effect validation.

The macro engine must not silently suppress effect checking.

---

171. Macro Expansion and Concurrency

Generated concurrency constructs must pass the canonical concurrency model.

The macro subsystem must not encode a fixed number of workers.

---

172. Macro Expansion and Distributed Computing

Generated distributed constructs must pass normal distributed semantic/resource validation.

Node count, topology, placement, and communication realization remain downstream.

---

173. Macro Expansion and Security

Generated security-sensitive constructs must pass the security/capability system.

Macro expansion must never be treated as an implicit trust boundary.

---

174. Macro Expansion and Interoperability

Generated foreign-language/interoperability constructs must pass the canonical interoperability boundary.

A macro must not silently create an unsupported ABI or calling convention.

---

175. Macro Expansion and Optimization

Optimization occurs after semantic validity.

The macro system must not assume that textual expansion is the final optimized representation.

For example:

macro
 ↓
expanded source
 ↓
semantic representation
 ↓
optimization

rather than:

macro
 ↓
hand-optimized target code

---

176. Macro Expansion and Scheduling

Scheduling is downstream.

A macro does not directly assign execution times unless a formally specified source-level scheduling construct exists.

Even then, the scheduler determines whether the request can be realized.

---

177. Macro Expansion and Routing

Routing is downstream.

Quantum macros do not directly select physical paths or coupling-map routes.

Hardware macros do not directly perform physical placement.

---

178. Macro Expansion and QEC

Quantum macro syntax must not implement QEC.

QEC remains downstream.

A macro may generate an explicit fault-tolerance requirement if such syntax is part of the canonical language.

---

179. Macro Expansion and ZQN

ZQN remains responsible for its canonical fault/noise/resilience semantics.

Macro grammar must not duplicate ZQN.

A macro can generate valid constructs consumed by ZQN after semantic lowering.

---

180. Macro Expansion and HAL

HAL remains responsible for actual hardware abstraction.

Macro syntax must not become a hardware-control API.

---

181. Testing a New Macro Feature

When adding a new macro feature, the developer must complete this sequence before modifying unrelated files:

1. Identify the language requirement.
2. Identify the owning grammar file.
3. Identify the canonical tokens.
4. Define syntax.
5. Define AST mapping.
6. Define semantic mapping.
7. Define expansion behavior.
8. Define hygiene/provenance.
9. Define diagnostics.
10. Define security/resource behavior.
11. Define IR integration.
12. Define compatibility.
13. Add positive tests.
14. Add negative tests.
15. Add boundary tests.
16. Add scalability tests.
17. Add determinism tests.
18. Run validation.

This is the independent-first development rule.

---

182. No Unnecessary File Renaming

Existing filenames are part of the repository's established structure.

Do not rename:

grammar/macros/README.md
grammar/macros/macros.g4
grammar/macros/declarations.g4
grammar/macros/invocations.g4
grammar/macros/parameters.g4
grammar/macros/expansion.g4
grammar/macros/hygiene.g4
grammar/macros/token-stream.g4
grammar/macros/syntax-tree.g4
grammar/macros/diagnostics.g4
grammar/macros/safety.g4

unless an actual technical conflict makes a rename necessary.

Prefer completing and integrating existing files.

---

183. No Parallel Hierarchy

Do not create another macro subsystem elsewhere merely because an existing file is inconvenient.

The canonical macro grammar remains:

grammar/macros/

Other directories may consume it, but must not duplicate it.

---

184. Single Ownership Rule

Every macro syntax concept must have exactly one primary owner.

For example:

macro declaration → declarations.g4
macro invocation → invocations.g4
parameter → parameters.g4
token tree → token-stream.g4
syntax-tree operation → syntax-tree.g4

Other files may reference these rules.

They must not redefine equivalent rules.

---

185. Integration With "grammar/expressions/macros.g4"

The existing expression-level macro integration must consume the canonical invocation rule.

There must not be:

grammar/macros/invocations.g4

defining one macro invocation model and:

grammar/expressions/macros.g4

defining another.

The expression grammar is an integration consumer.

---

186. Integration With AST

The parser must construct the canonical AST representation already established by the repository.

If a required AST representation is missing, the feature is not complete merely because grammar syntax has been added.

The AST change must be designed before marking the grammar complete.

---

187. Integration With Compiler Macro Engine

The macro compiler engine must receive a representation that preserves:

- identity;
- arguments;
- source locations;
- expansion context;
- relevant provenance.

The grammar does not dictate the internal compiler data structures beyond the observable contract.

---

188. Existing String-Based Expansion

If existing compiler infrastructure currently represents generated macro output as source text, this must be treated as an implementation stage rather than the final architectural requirement.

A production system must ensure that textual expansion cannot bypass:

- parsing;
- hygiene;
- provenance;
- semantic analysis.

The eventual implementation may use token trees/ASTs where appropriate without changing the source-language contract.

---

189. Macro Engine Resource Configuration

Compiler resource configuration must remain separate from grammar.

Configuration can provide policies such as:

maximum expansion work permitted for this compilation

but must not alter the accepted language grammar.

Different machines can therefore use different compilation budgets without requiring different language definitions.

---

190. Portability Test

A macro feature should be tested conceptually against multiple target classes:

tiny classical
large classical
GPU
FPGA
ASIC
QPU
simulator
HPC
distributed
future/unknown capability target

The test question is:

«Does macro source syntax remain unchanged while target realization changes?»

If yes, the macro architecture preserves POCO-REAF.

---

191. Unknown Future Targets

A macro should not require the compiler to know every future target today.

Target-independent syntax should remain valid until semantic requirements are evaluated.

An unknown future target can implement the appropriate downstream backend without requiring a rewrite of macro syntax.

---

192. Macro System Invariants

The following are non-negotiable invariants:

1. One canonical macro language.
2. One canonical root grammar.
3. One canonical lexer.
4. One canonical frontend AST.
5. One canonical semantic model.
6. No duplicate quantum IR.
7. No target-specific macro grammar.
8. No artificial hardware limits.
9. No arbitrary host execution during parsing.
10. No implicit privilege escalation.
11. No unsafe Rust.
12. No silent keyword injection.
13. No semantic bypass.
14. Source provenance preserved.
15. Hygiene preserved where promised.
16. Determinism preserved where promised.
17. Resource safeguards remain configurable implementation policy.
18. New domains must integrate without rewriting the macro architecture.

---

193. Definition of Production Ready

"grammar/macros/" is production-ready when a developer can answer all of these questions without guessing:

Syntax

- What is a macro declaration?
- What is a macro invocation?
- What is a macro path?
- What is a macro parameter?
- What is an argument?
- What is a token tree?

AST

- Which AST node represents it?
- Where are source spans stored?
- How are arguments represented?

Semantics

- How is the macro resolved?
- How are parameters matched?
- How are defaults handled?
- How is expansion validated?

Hygiene

- How are generated identifiers protected?
- How is provenance retained?

Compiler

- Where does expansion happen?
- Where are expansion budgets enforced?
- Where are diagnostics generated?

IR

- How does generated code reach canonical IR?
- How does generated quantum code reach "quantum::ir"?

Scalability

- Is there an artificial macro limit?
- Is there an artificial argument/parameter limit?
- Is there an artificial expansion ceiling?
- Is there target-specific syntax?

Safety

- Does any macro implementation require "unsafe"?
- Can a macro bypass security?
- Can a macro perform hidden I/O?

Compatibility

- Which language version defines the syntax?
- What is deprecated?
- How are breaking changes handled?

If any answer requires guessing, the subsystem is not yet production complete.

---

194. Final Architecture

The complete macro architecture is:

                         Zamani Source
                              │
                              ▼
                       Canonical Lexer
                              │
                              ▼
                      Canonical Parser
                              │
             ┌────────────────┴────────────────┐
             │                                 │
             ▼                                 ▼
      Macro Declaration                 Macro Invocation
             │                                 │
             └────────────────┬────────────────┘
                              ▼
                    Canonical Frontend AST
                              │
                              ▼
                    Module / Name Resolution
                              │
                              ▼
                       Macro Resolution
                              │
                              ▼
                       Argument Binding
                              │
                              ▼
                         Expansion
                              │
                     ┌────────┴────────┐
                     ▼                 ▼
                  Hygiene          Provenance
                     │                 │
                     └────────┬────────┘
                              ▼
                     Semantic Analysis
                              │
            ┌─────────────────┼─────────────────┐
            │                 │                 │
            ▼                 ▼                 ▼
        Classical        quantum::ir       HDL/Hardware
            │                 │                 │
            └─────────────────┼─────────────────┘
                              ▼
                         Optimization
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
          Routing         Scheduling       Resilience
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                             ZQN
                              │
                              ▼
                             HAL
                              │
                              ▼
                      Target Realization
                              │
          ┌───────────┬───────┼────────┬───────────┐
          ▼           ▼       ▼        ▼           ▼
         CPU         GPU     FPGA     QPU        Future
          │           │       │        │          targets
          └───────────┴───────┼────────┴───────────┘
                              ▼
                          Execution

The macro subsystem is therefore a portable source transformation mechanism, not a target compiler.

---

195. Final POCO-REAF Guarantee

The architectural promise is:

PROGRAM ONCE
     │
     ▼
CANONICAL ZAMANI SOURCE
     │
     ▼
MACROS EXPAND INTO CANONICAL ZAMANI
     │
     ▼
SEMANTIC VALIDATION
     │
     ▼
CANONICAL IR
     │
     ▼
TARGET-SPECIFIC LOWERING
     │
     ├── tiny machine
     ├── CPU
     ├── multicore
     ├── GPU
     ├── FPGA
     ├── ASIC
     ├── accelerator
     ├── QPU
     ├── simulator
     ├── HPC
     ├── cluster
     ├── distributed
     ├── cloud
     └── future architecture

The programmer writes the semantic program once.

Macro expansion must not force the programmer to rewrite the program for a larger or different machine.

The compiler determines the appropriate realization from:

program semantics
+
requirements
+
capabilities
+
resources
+
target characteristics
+
compiler policies

rather than from grammar-defined machine limits.

---

196. Completion Statement

This file is complete when:

- its ownership is unambiguous;
- every existing macro grammar component has a defined role;
- every cross-directory dependency is documented;
- lexer ownership is documented;
- AST ownership is documented;
- semantic ownership is documented;
- expansion ownership is documented;
- hygiene ownership is documented;
- provenance ownership is documented;
- diagnostics ownership is documented;
- resource-policy boundaries are documented;
- security boundaries are documented;
- canonical IR integration is documented;
- quantum integration is documented;
- classical integration is documented;
- HDL/hardware integration is documented;
- distributed/AI/data integration is documented;
- POCO-REAF requirements are documented;
- Rust 1.97/1.97.1 compatibility is documented;
- safe-Rust-only requirements are documented;
- no artificial language capacity is specified;
- no unnecessary existing files are renamed;
- no competing macro grammar is introduced;
- all macro grammar files can be validated independently against this contract;
- the complete subsystem can subsequently be integrated through "grammar/Zamani.g4" without redesigning this contract.

This README is the architectural contract.

The ".g4" files implement its syntax portions.

The canonical AST implements its structural representation.

The compiler implements resolution, expansion, hygiene, provenance, and resource policy.

Semantic analysis validates generated programs.

Canonical IR represents their meaning.

Backends realize that meaning on available targets.

That separation is the foundation for scalable Zamani macros and for "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)".