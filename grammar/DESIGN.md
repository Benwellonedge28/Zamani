Zamani Grammar Design

Path: "grammar/DESIGN.md"
Language: Zamani
Compiler: ZUTC / Zamani compiler
Repository: "Benwellonedge28/Zamani"
Default branch: "main"
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust safety policy: Safe Rust only; production compiler code MUST NOT use Rust "unsafe"
Primary portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: From the smallest supported computation to arbitrarily large computations, bounded only by the actual semantics of the program, representation limits, declared resource policies, and resources available to the compiler/runtime/target.

---

1. Status and Authority

This document is the normative architectural design for the "grammar/" subsystem.

It defines:

- what the grammar owns;
- what it does not own;
- the authority relationship between existing grammar/specification files;
- how syntax integrates with the Rust lexer and parser;
- how syntax maps to the frontend AST;
- how AST constructs map to semantic analysis;
- how semantic constructs map to canonical IR;
- how quantum syntax reaches "quantum::ir";
- how classical, quantum, HDL, hybrid, AI, distributed, networking, security, data, and other domains coexist;
- how POCO-REAF is protected;
- how hardware/resource scalability is protected;
- how compatibility is maintained;
- how grammar features become independently completable;
- how production conformance is tested.

This document does not claim that every feature described by the architecture is already implemented.

A feature is implemented only when its complete pipeline exists:

Specification
    ↓
Lexical contract
    ↓
Grammar
    ↓
Lexer
    ↓
Parser
    ↓
AST
    ↓
Structural validation
    ↓
Name/module resolution
    ↓
Type/effect/resource/capability semantics
    ↓
Canonical semantic representation
    ↓
IR
    ↓
Optimization/lowering
    ↓
Backend/runtime integration
    ↓
Tests

A syntax-only addition is therefore not a production feature.

---

2. Fundamental Language Objective

Zamani is intended to be a universal programming language capable of expressing:

- classical computation;
- systems programming;
- scientific computing;
- numerical computing;
- symbolic computing;
- high-performance computing;
- parallel computation;
- distributed computation;
- quantum computation;
- hybrid quantum-classical computation;
- hardware description;
- hardware/software co-design;
- embedded computation;
- accelerators;
- AI/ML;
- tensor/data computation;
- networking;
- cryptography;
- security;
- edge/cloud computation;
- nano-oriented computation;
- temporal computation;
- metaprogramming;
- domain-specific extensions;
- future computational paradigms.

The grammar must therefore be universal without becoming target-specific.

The fundamental distinction is:

Zamani source describes:
    WHAT the program means
    WHAT guarantees it requires
    WHAT capabilities it requires
    WHAT resources it requires
    WHAT constraints it permits
    WHAT preferences it has
    WHAT computations must occur

Later compilation/runtime infrastructure determines:
    WHERE
    WHEN
    HOW
    ON WHICH TARGET
    USING WHICH PHYSICAL RESOURCE
    USING WHICH NATIVE INSTRUCTION SET

This separation is mandatory.

---

3. POCO-REAF

The central architectural objective is:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Anywhere
      ↓
Forever

POCO-REAF means that source-level semantics are not coupled to a particular machine.

It does not mean that every program can execute on every target regardless of resources.

The following are distinct states:

Lexically valid
    ≠
Syntactically valid
    ≠
Semantically valid
    ≠
Compilable
    ≠
Target compatible
    ≠
Resource feasible
    ≠
Runtime available

For example, a program requiring a symbolic number of quantum resources is still valid when a particular QPU does not currently possess enough physical qubits.

The compiler may:

- map it to a larger target;
- distribute it;
- use logical qubits;
- decompose operations;
- route operations;
- schedule operations;
- use error correction;
- simulate it;
- compile it for another backend;
- wait for resources;
- reject the target with a precise resource diagnostic.

The compiler must not silently redefine the program merely because the target is smaller.

---

4. Scalability Principle

Zamani must scale conceptually from:

one value
one variable
one operation
one qubit
one classical processor
one accelerator
one device

to:

arbitrarily large collections
arbitrarily large programs
arbitrarily large datasets
arbitrarily large tensors
arbitrarily large quantum computations
arbitrarily large distributed systems
arbitrarily large heterogeneous systems

subject to available resources and explicitly defined semantic constraints.

The grammar MUST NOT contain universal machine ceilings such as:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_TIMELINES
MAX_AGENTS
MAX_DEVICES
MAX_NETWORK_LINKS
MAX_ACCELERATORS

Likewise, the grammar MUST NOT encode finite enumerations of physical resources such as:

Qubit0
Qubit1
...
Qubit1023

as the universal language model.

A program may contain an explicit value such as:

1024

when that number is program semantics.

The prohibition applies to compiler-defined artificial limits, not ordinary program values.

---

5. What "Nothing Must Be Hard Coded" Means

The rule is semantic, not a prohibition on constants.

Valid:

let n = 1024;
allocate n elements;

because "1024" is program data.

Invalid architecture:

const MAX_QUBITS = 1024;

when that constant defines the maximum quantum computation representable by the language.

Likewise:

matrix<1024, 1024>

may be valid.

But:

the grammar only supports matrices <= 1024 × 1024

is prohibited.

The same distinction applies to:

- CPUs;
- cores;
- threads;
- GPUs;
- FPGAs;
- QPUs;
- memory;
- storage;
- nodes;
- network endpoints;
- tensor dimensions;
- vector widths;
- registers;
- timelines;
- processes;
- agents;
- accelerator counts.

---

6. Requirement, Capability, Preference, Hint, and Realization

Zamani MUST distinguish the following concepts.

6.1 Semantic requirement

Example:

requires qubits >= n

The program requires a semantic resource.

6.2 Capability requirement

Example:

requires capability("quantum.mid_circuit_measurement")

The program requires a capability.

6.3 Constraint

Example:

requires latency <= budget

A property must remain within a defined constraint.

6.4 Preference

Example:

prefer accelerator("quantum")

This does not make the target mandatory.

6.5 Hint

Example:

hint locality

This gives compilation information without changing program meaning.

6.6 Implementation realization

Example:

map logical_resource -> physical_resource

Physical realization belongs downstream.

The compiler may determine:

logical qubit
    ↓
physical qubit

without requiring the programmer to hard-code a physical device.

---

7. Repository Authority Model

The repository currently contains multiple grammar-related artifacts.

They MUST have different responsibilities.

7.1 "grammar/DESIGN.md"

This file.

Owns:

- grammar architecture;
- authority;
- integration;
- invariants;
- production criteria.

It does not define every individual grammar production.

---

7.2 "grammar/Zamani.g4"

This file MUST remain.

It is the canonical ANTLR grammar representation of the accepted Zamani syntax.

It MUST NOT become an independent language.

It MUST converge with:

grammar/specification/
grammar/spec/
src/lexer.rs
src/parser.rs
src/ast/

It is the ANTLR syntax representation, not the semantic authority.

The existing "Zamani.g4" currently contains a very broad monolithic surface, including mathematical operations, quantum constructs, nano constructs, OOP, effects, modules, and other domains. It must therefore be progressively reconciled rather than simply expanded indefinitely.

---

7.3 "grammar/grammar.md"

This file MUST remain.

Its role is:

«implementation-conformance reference.»

It describes what the reference compiler currently accepts.

It must distinguish:

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

It must not silently become a second normative language specification.

---

7.4 "grammar/Zamani-Grammar.md"

This file MUST remain.

It may contain:

- historical designs;
- proposed constructs;
- advanced language ideas;
- future paradigms;
- NIMBUS/Universal-Trinity material;
- Sankofa concepts;
- MTS concepts;
- nano concepts;
- AI concepts;
- omniversal concepts;
- experimental syntax.

But a construct contained in this file is not automatically valid Zamani syntax.

Promotion requires:

Design
 ↓
Specification
 ↓
Grammar
 ↓
Lexer
 ↓
Parser
 ↓
AST
 ↓
Semantics
 ↓
IR
 ↓
Compiler/runtime
 ↓
Tests
 ↓
Stable

---

8. Single Language Rule

There MUST be exactly one canonical Zamani language.

The repository may contain:

- ANTLR grammar;
- Rust lexer;
- Rust parser;
- language reference;
- generated documentation;
- compatibility documentation;
- IDE grammar;
- syntax-highlighting grammar;

but these are representations of one language.

They MUST NOT silently create different languages.

---

9. Canonical Compiler Pipeline

The production architecture is:

                         Zamani Source
                              │
                              ▼
                     Source Map / File ID
                              │
                              ▼
                           Lexer
                              │
                              ▼
                        Token Stream
                              │
                              ▼
                           Parser
                              │
                              ▼
                       Frontend AST
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
       Name Resolution   Type Analysis   Effect Analysis
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                 Resource/Capability Analysis
                              │
                              ▼
                    Semantic Validation
                              │
                              ▼
                 Canonical Semantic Model
                              │
             ┌────────────────┼─────────────────┐
             ▼                ▼                 ▼
        Classical         Quantum             HDL/
        Semantics        Semantics         Hardware Intent
             │                │                 │
             └────────────────┼─────────────────┘
                              ▼
                       Canonical IR
                              │
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
                       Target Lowering
                              │
          ┌───────────┬──────┼──────┬────────────┐
          ▼           ▼      ▼      ▼            ▼
         CPU         GPU    FPGA    QPU       Future Target

The grammar MUST remain above target-specific realization.

---

10. Integration With Existing Rust Frontend

The repository currently has:

src/lexer.rs
src/parser.rs
src/ast/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs

The grammar must integrate with those files rather than create another frontend architecture.

Ownership:

Component| Owns| Must not own
"grammar/"| language syntax specification| runtime
"Zamani.g4"| ANTLR syntax| semantic lowering
"grammar.md"| implementation reference| future promises
"Zamani-Grammar.md"| design/proposal material| implementation claims
"src/lexer.rs"| tokenization| type checking
"src/parser.rs"| syntax recognition| hardware mapping
"src/ast/"| source structure| physical realization
"src/semantic.rs"| semantic validation| source tokenization
"src/ir_gen.rs"| AST → IR lowering| hardware scheduling
"src/ir_verify.rs"| IR validation| source parsing
"src/quantum/ir/"| canonical quantum semantics| source grammar
optimization| semantics-preserving transformation| language syntax
routing| physical realization| language definition
scheduling| time/resource scheduling| grammar
QEC| quantum error correction| parser
ZQN| fault/noise semantics| duplicate quantum IR
HAL| target capability/state| language grammar
runtime| execution| source syntax

---

11. Rust Baseline and Safety

The repository explicitly targets Rust 1.97 / 1.97.1. The toolchain and package metadata currently encode that baseline.

Production grammar/frontend architecture MUST therefore be compatible with:

Rust 1.97
Rust 1.97.1
Edition 2021

The compiler implementation MUST use safe Rust.

Prohibited in production compiler code:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

This requirement applies to the implementation.

The presence of the source-language word "unsafe" is a separate language-design issue.

---

12. Source-Language "unsafe"

The existing grammar and lexer currently contain an "unsafe" language concept. The lexer has "KeywordUnsafe", and the parser has an "unsafe" parsing path.

Therefore:

«"No unsafe" means no Rust "unsafe" in the Zamani compiler implementation, unless a future repository-wide safety policy explicitly changes this.»

It does not automatically mean that the Zamani source language cannot discuss unsafe operations.

However, any source-level unsafe feature MUST have an independently defined:

- syntax contract;
- semantic contract;
- capability contract;
- security model;
- compiler implementation;
- diagnostics;
- tests.

If Zamani ultimately adopts a safe-by-default language with no source-level unsafe execution model, "unsafe" should become deprecated/reserved rather than silently retaining incomplete semantics.

No grammar change may claim that merely parsing "unsafe" provides safety guarantees.

---

13. Lexical Architecture

"src/lexer.rs" is the executable lexical implementation.

The lexical contract is defined by:

grammar/spec/lexical.md

with the ANTLR representation conforming to it.

The lexer MUST provide:

- deterministic tokenization;
- UTF-8 handling;
- source spans;
- stable token identity;
- longest-valid operator matching;
- literal recognition;
- structured diagnostics;
- malformed-input recovery;
- no target-dependent behavior;
- no hidden global state;
- no machine-size assumptions.

The existing lexer contains a large token inventory spanning core, OOP, quantum, nano, Sankofa, temporal, mathematical and advanced-system concepts.

That inventory MUST be audited before further keyword expansion.

---

14. Canonical Token Identity

The current lexer contains overlapping token concepts including:

BitAnd / Ampersand
BitOr  / Pipe
Question / QuestionMark

These must not represent duplicate lexical meanings.

One source spelling should normally have one canonical token identity.

Example:

&
 ↓
BitAndToken
 ↓
parser context
 ↓
AST meaning
 ↓
semantic meaning

The lexer should not create several competing tokens merely because the same spelling can have different semantic uses.

The same rule applies to:

|
?
*
-
+

where context can determine meaning.

---

15. Keyword Policy

Zamani has a very broad keyword inventory.

A new concept MUST NOT become a reserved keyword merely because it exists as a library feature.

Prefer:

identifier
+
compositional syntax
+
semantic resolution

over:

one keyword for every operation

This is particularly important for:

- quantum gates;
- mathematical algorithms;
- AI algorithms;
- accelerator names;
- hardware devices;
- vendor operations;
- networking protocols;
- cryptographic algorithms.

---

16. Literal Scalability

The lexer must identify literals without imposing host-machine representation limits.

For example:

999999999999999999999999999999999999999999999999

must not be lexically rejected solely because it exceeds "u64".

Lexical processing should preserve the literal.

Semantic/type checking determines whether it is valid for the selected type.

Likewise floating-point literals must not be silently rounded during lexical recognition.

---

17. Parser Architecture

"src/parser.rs" is currently a hand-written recursive-descent / Pratt parser with explicit precedence handling. Its precedence structure covers assignment, ranges, logical/bitwise operations, equality, comparison, shifts, arithmetic, prefix operations, calls, indexing, and member access.

The production parser MUST:

- be deterministic;
- preserve source spans;
- produce canonical AST structures;
- report structured errors;
- guarantee progress during recovery;
- avoid infinite loops;
- avoid silently accepting unsupported constructs;
- avoid target-specific behavior;
- avoid artificial input-size limits;
- use safe Rust;
- support deep programs through appropriate iterative mechanisms where necessary.

---

18. Parser Recovery

On invalid input:

unexpected token
       ↓
diagnostic
       ↓
consume/recover
       ↓
synchronization point
       ↓
continue where safe

Recovery MUST make progress.

The parser must never repeatedly process the same invalid token forever.

Recovery nodes must not silently become executable program semantics.

---

19. Deep-Structure Scalability

A universal language cannot assume shallow programs.

Implementations must avoid unnecessary recursion where deeply nested input can exhaust the call stack.

Where appropriate, use:

- explicit stacks;
- explicit worklists;
- iterative traversals;
- streaming;
- incremental processing;
- lazy structures;
- configurable resource budgets.

The compiler must distinguish:

language limit

from:

resource budget

A compilation budget may legitimately reject a compilation because of resource exhaustion.

That does not establish a language-level maximum.

---

20. Canonical Grammar Composition

"grammar/Zamani.g4" remains the top-level ANTLR entry point.

Conceptually:

Zamani.g4
   │
   ├── core
   ├── types
   ├── expressions
   ├── statements
   ├── declarations
   ├── functions
   ├── modules
   ├── effects
   ├── memory
   ├── concurrency
   ├── classical
   ├── quantum
   ├── hybrid
   ├── hdl
   ├── hardware
   ├── distributed
   ├── ai
   ├── data
   ├── networking
   ├── security
   ├── resources
   ├── compile
   ├── execution
   ├── interoperability
   ├── dialects
   ├── macros
   └── metaprogramming

These are conceptual ownership boundaries.

Physical ".g4" decomposition must occur only where it improves maintainability and can be integrated without creating independent grammars.

---

21. "grammar/antlr/"

The existing "grammar/antlr/" directory must not become a second grammar authority.

Before retaining any generated or legacy ANTLR grammar there, repository references must establish whether it is actively consumed.

The desired invariant is:

grammar/Zamani.g4
        ↓
canonical ANTLR entry

and not:

grammar/Zamani.g4
        +
grammar/antlr/Zamani.g4
        +
another parser grammar

If files under "grammar/antlr/" are generated artifacts, their provenance must be documented.

If they are obsolete and unused, they should eventually be removed rather than maintained indefinitely.

No rename of "Zamani.g4" is required.

---

22. Universal Grammar Layers

The grammar is divided conceptually into:

Universal syntax
    ↓
Domain syntax
    ↓
Semantic capabilities
    ↓
Canonical semantic representation

Universal syntax includes:

- identifiers;
- names;
- paths;
- attributes;
- declarations;
- types;
- expressions;
- statements;
- patterns;
- functions;
- modules;
- effects;
- memory;
- concurrency.

Domain syntax includes:

- classical;
- quantum;
- HDL;
- hybrid;
- AI;
- distributed;
- data;
- networking;
- security;
- hardware;
- resource;
- execution.

Domains MUST share the universal syntax rather than becoming separate languages.

---

23. Core Grammar

"grammar/core/" owns language-wide syntax.

It must cover:

- source units;
- identifiers;
- qualified names;
- paths;
- attributes;
- modifiers;
- visibility;
- blocks;
- documentation;
- source-level metadata.

Core MUST NOT own:

- quantum semantics;
- hardware topology;
- QEC;
- routing;
- scheduling;
- backend instructions.

---

24. Types

"grammar/types/" owns type syntax.

The architecture supports, when individually implemented:

- primitive types;
- named types;
- generic types;
- tuples;
- arrays;
- slices;
- functions;
- references;
- optional types;
- result types;
- never;
- linear types;
- affine types;
- dependent types;
- effectful types;
- temporal types;
- resource types;
- capability types;
- quantum types;
- hardware-intent types.

The existing repository type specification includes primitive, composite, dependent, linear/affine, and quantum types.

The existing type specification also states fixed "Int"/"Float" representations. Those are semantic type definitions and must not be confused with grammar-wide hardware limits.

A future arbitrary-precision or target-independent numeric model must be introduced as an explicit type-system change, not silently through the lexer.

---

25. Expressions

"grammar/expressions/" owns expression syntax.

It must cover:

- literals;
- identifiers;
- unary expressions;
- arithmetic;
- comparison;
- logical;
- bitwise;
- shifts;
- ranges;
- assignment;
- calls;
- indexing;
- member access;
- tuples;
- arrays;
- lambdas;
- closures;
- conditional expressions;
- match expressions;
- async/await;
- spawn;
- domain expressions;
- quantum expressions;
- effect expressions;
- macro expressions;
- metaprogramming expressions.

Expression grammar must remain compositional.

---

26. Mathematical Computing

The current "Zamani.g4" contains a very large direct mathematical vocabulary, including vector, matrix, tensor, symbolic, calculus, statistical, numerical, signal-processing and optimization constructs.

These capabilities remain part of Zamani's intended computational surface.

However:

«Every mathematical algorithm does not need to become a grammar keyword.»

The preferred architecture is:

generic syntax
      ↓
typed value
      ↓
mathematical operation
      ↓
intrinsic / standard library
      ↓
optimization
      ↓
backend

A mathematical construct should become dedicated language syntax only when it has language-level semantic significance.

This prevents grammar explosion while preserving mathematical expressiveness.

---

27. Functions

"grammar/functions/" owns:

- function declarations;
- parameters;
- generic parameters;
- return types;
- contracts;
- effects;
- async functions;
- generators;
- closures;
- lambdas;
- calling-convention intent.

Calling-convention syntax must express semantic/ABI requirements rather than encode a particular physical processor.

---

28. Declarations

"grammar/declarations/" owns:

- variables;
- constants;
- structures;
- records;
- enums;
- traits;
- interfaces;
- implementations;
- classes;
- type aliases;
- resource declarations;
- capability declarations;
- domain declarations.

The universal declaration dispatcher belongs at the composition root.

---

29. Statements and Control Flow

"grammar/statements/" owns:

- bindings;
- conditionals;
- loops;
- match;
- return;
- break;
- continue;
- exceptions;
- effects;
- resource operations;
- domain statements;
- expression statements.

Parallelism MUST remain semantic rather than assuming a fixed number of threads.

For example:

parallel

does not mean:

exactly eight threads

unless "8" is explicitly part of program semantics.

---

30. Modules

"grammar/modules/" owns:

- modules;
- imports;
- exports;
- namespaces;
- packages;
- aliases;
- dependencies;
- visibility;
- versioning.

The module system must support arbitrarily large module graphs subject to actual compiler/resource constraints.

---

31. Effects

"grammar/effects/" owns the syntax for:

- effect declarations;
- effect operations;
- effect handlers;
- effect lists;
- effectful types;
- effect composition.

The grammar does not execute effects.

Semantic analysis validates them.

Runtime infrastructure realizes them.

---

32. Memory

"grammar/memory/" owns portable memory semantics:

- ownership;
- borrowing;
- references;
- allocation intent;
- regions;
- shared memory;
- distributed memory;
- persistent memory;
- accelerator memory;
- quantum memory;
- memory capabilities.

The grammar MUST NOT define universal physical memory capacity.

It may express:

requires memory >= expression

but not establish:

Zamani supports at most 64 GB

---

33. Concurrency

"grammar/concurrency/" owns:

- asynchronous computation;
- tasks;
- spawn;
- await;
- actors;
- channels;
- synchronization;
- parallelism;
- data parallelism;
- task parallelism;
- pipelines;
- reductions;
- deterministic parallelism;
- distributed concurrency.

The number of workers is a runtime/resource concern unless explicitly part of program semantics.

---

34. Classical Computing

"grammar/classical/" owns classical computational intent:

- scalars;
- integers;
- floating-point values;
- vectors;
- matrices;
- tensors;
- numerical computation;
- symbolic computation;
- signal processing;
- scientific computing;
- optimization;
- control;
- statistics.

Classical constructs must lower into the repository's canonical classical/control/data IR architecture rather than bypassing semantic analysis.

---

35. Quantum Computing

Quantum computing is a first-class domain.

"grammar/quantum/" owns source syntax for:

- quantum types;
- registers;
- states;
- operations;
- parameters;
- controls;
- adjoints;
- measurement;
- reset;
- barriers;
- observables;
- channels;
- dynamic control;
- classical feed-forward;
- logical operations;
- error-correction intent;
- noise/fault intent;
- resource requirements;
- pulse intent where explicitly supported.

The grammar does not own:

- physical qubit allocation;
- calibration;
- coupling maps;
- native gate implementation;
- physical topology;
- device identifiers;
- scheduling;
- QEC execution;
- ZQN execution;
- HAL state.

---

36. Quantum Operation Model

The current "Zamani.g4" includes explicit fixed quantum operation/gate alternatives. That architecture must not become the universal quantum model.

Do not make the complete grammar equivalent to:

gate
    : H
    | X
    | Y
    | Z
    | CNOT
    | T
    | S
    | SWAP
    | ...

Instead:

QuantumOperation
    :=
        operation specification
        +
        parameters
        +
        modifiers
        +
        controls
        +
        targets

The operation may be:

- standard;
- user-defined;
- library-defined;
- dialect-defined;
- backend-provided;
- future-defined.

Semantic resolution determines whether it exists and whether it is valid.

---

37. Quantum Gate Portability

A backend may possess:

native gate set

but that is not the Zamani language definition.

The compilation flow is:

Zamani quantum operation
        ↓
canonical quantum semantic representation
        ↓
quantum::ir
        ↓
optimization
        ↓
decomposition
        ↓
routing
        ↓
scheduling
        ↓
calibration/noise-aware lowering
        ↓
native target representation

A target lacking a required capability must produce a structured diagnostic.

It must not silently substitute a different computation.

---

38. Canonical "quantum::ir"

The repository contains a substantial "src/quantum/ir" architecture, including canonical quantum identities, controls, validation, resources, classical integration, analysis and other IR components.

Therefore:

«"quantum::ir" remains the canonical quantum semantic boundary.»

The grammar MUST NOT create another competing quantum IR.

The intended flow is:

Zamani source
    ↓
Frontend AST
    ↓
Semantic quantum model
    ↓
quantum::ir
    ↓
optimization
    ↓
routing
    ↓
scheduling
    ↓
resilience/QEC/ZQN
    ↓
HAL/backend

Frontend-specific gate enums or hardware-specific operation structures MUST NOT become an alternative canonical representation.

---

39. Quantum Identity Ownership

Quantum identifiers such as:

- logical qubit IDs;
- physical qubit IDs;
- classical bit IDs;
- operation identities;

must remain owned by their canonical quantum IR/domain modules.

The repository explicitly contains canonical quantum ID modules under "src/quantum/ir", including qubit and physical-qubit identity structures.

The grammar must never invent another physical-qubit identity system.

The source language may express logical resources.

Physical IDs are downstream realization information.

---

40. Quantum Resource Scaling

The grammar must support:

one qubit

through:

symbolic register
parameterized register
dynamically sized quantum resource
large logical computation
distributed quantum computation

without embedding an artificial maximum.

The semantic system may reject a target because:

available physical qubits < required resources

but that is a target/resource diagnostic, not a syntax error.

---

41. Quantum Error Correction

The grammar may express intent such as:

- fault-tolerant requirement;
- error-correction requirement;
- logical operation intent;
- noise tolerance;
- reliability constraints.

It must not implement QEC.

Ownership remains:

Grammar
    → syntax

Semantic analysis
    → validity

QEC
    → error-correction strategy

ZQN
    → quantum fault/noise semantics

Routing
    → physical realization

Scheduling
    → timing/order/resource scheduling

HAL
    → target capability/state

The repository's quantum resilience architecture already integrates with "quantum::ir"; the grammar must remain upstream of it.

---

42. ZQN

ZQN is a downstream quantum fault/noise semantics layer.

Grammar constructs may declare intent such as:

requires fault_tolerance(...)
requires noise_budget(...)
requires reliability(...)

but the grammar must not duplicate ZQN's fault model.

ZQN consumes canonical semantic/IR information.

---

43. Hybrid Computing

"grammar/hybrid/" owns the syntax required to compose:

classical
    ↓
quantum
    ↓
measurement
    ↓
classical decision
    ↓
quantum

and other heterogeneous execution models.

Hybrid computation must use shared:

- values;
- types;
- control flow;
- effects;
- resources;
- capabilities;
- source spans;
- diagnostics.

Quantum and classical are domains of one language, not two independent languages.

---

44. HDL

"grammar/hdl/" owns hardware-description and hardware/software co-design syntax.

It may express:

- modules;
- ports;
- signals;
- nets;
- registers;
- combinational logic;
- sequential logic;
- clocks;
- resets;
- timing;
- state machines;
- pipelines;
- memories;
- interfaces;
- protocols;
- assertions;
- generate constructs;
- synthesis intent;
- simulation intent;
- verification intent;
- physical intent.

HDL must describe hardware intent.

It must not turn today's FPGA/ASIC architecture into permanent language limits.

---

45. Hardware

"grammar/hardware/" owns target-independent hardware intent.

It may express:

- target requirements;
- compute capabilities;
- memory capabilities;
- accelerator requirements;
- quantum-device capabilities;
- interconnect requirements;
- topology constraints;
- timing requirements;
- power constraints;
- thermal constraints;
- reliability requirements;
- calibration requirements;
- deployment intent.

The compiler/runtime determines actual target realization.

---

46. Resource System

"grammar/resources/" owns source-level resource intent.

It must distinguish:

requirement
constraint
capability
preference
hint
budget
placement intent
negotiation

This is essential for POCO-REAF.

A program should be able to say:

requires capability("tensor.compute")

without saying:

use GPU 0

The latter is target realization.

---

47. Distributed Computing

"grammar/distributed/" owns:

- nodes;
- processes;
- services;
- actors;
- messages;
- channels;
- replication;
- partitioning;
- consistency;
- transactions;
- fault tolerance;
- collective operations;
- topology intent;
- deployment intent.

The grammar must never impose a universal node count.

---

48. AI and Machine Learning

"grammar/ai/" owns syntax for AI/ML semantic intent such as:

- models;
- tensors;
- datasets;
- training;
- inference;
- differentiable computation;
- probabilistic computation;
- neural computation;
- symbolic computation;
- agents;
- pipelines;
- distributed training;
- deployment.

Framework-specific implementation belongs outside the grammar.

For example, the grammar should not become a list of:

pytorch
tensorflow
jax
cuda
rocm
...

keywords.

Those are ecosystem/backend concerns.

---

49. Data

"grammar/data/" owns:

- collections;
- records;
- schemas;
- tables;
- streams;
- tensors;
- datasets;
- transformations;
- queries;
- pipelines;
- persistence;
- serialization;
- provenance.

Data sizes remain resource-dependent.

---

50. Networking

"grammar/networking/" owns portable networking intent:

- endpoints;
- abstract addresses;
- protocols;
- channels;
- requests;
- responses;
- streaming;
- routing intent;
- service discovery;
- distributed computation.

Physical network addresses and topology belong to deployment/runtime infrastructure unless explicitly represented as program semantics.

---

51. Security

"grammar/security/" owns language-level security intent:

- identity;
- authorization;
- capabilities;
- policies;
- secrets;
- cryptographic intent;
- signatures;
- hashes;
- secure computation;
- provenance;
- trust;
- zero-knowledge constructs where implemented.

The grammar should not become an algorithm catalog.

Cryptographic implementation remains a library/backend responsibility.

---

52. Interoperability

"grammar/interoperability/" owns source-level interfaces to external representations.

Potential formats include:

- C;
- C++;
- Rust;
- Python;
- WebAssembly;
- OpenQASM;
- QIR;
- HDL;
- serialization formats.

These are interoperability boundaries.

They are not competing canonical semantic models.

In particular:

OpenQASM
QIR
LLVM
MLIR
vendor IR

must not replace Zamani's canonical semantic model.

---

53. Dialects

"grammar/dialects/" provides controlled extension.

A dialect must declare:

name
version
owner
syntax additions
semantic additions
AST mapping
IR mapping
capabilities
compatibility
feature gates
diagnostic behavior

A dialect must not silently redefine existing Zamani semantics.

Dialect extensions must remain versioned and discoverable.

---

54. Macros

"grammar/macros/" owns syntax for:

- macro declarations;
- invocations;
- parameters;
- expansion;
- hygiene;
- token/syntax-tree operations;
- macro diagnostics.

Macro expansion must not bypass:

lexical validation
syntax validation
type checking
semantic validation
security validation

Generated code becomes ordinary Zamani semantic input after expansion.

---

55. Metaprogramming

"grammar/metaprogramming/" may support:

- reflection;
- introspection;
- quotation;
- unquotation;
- code generation;
- compile-time computation;
- type-level computation;
- schema generation.

It must not create an unrestricted escape from compiler validation.

---

56. Sankofa

The broad design material contains Sankofa concepts such as:

- remember;
- recall;
- learn;
- infer;
- wisdom;
- zamani;
- sasa;
- history;
- temporal learning.

These may remain first-class language concepts where their semantics are actually defined.

The grammar only parses the constructs.

It must not maintain memory or hidden state during parsing.

The architecture is:

Sankofa syntax
    ↓
AST
    ↓
semantic model
    ↓
canonical representation
    ↓
runtime/service

---

57. Multi-Timeline System

MTS constructs may express:

- timelines;
- branching;
- speculative execution;
- observation;
- fork/merge;
- temporal state.

The grammar must not define a fixed number of timelines.

There must be no universal:

MAX_TIMELINES

or equivalent.

Timeline capacity is a semantic/runtime/resource concern.

---

58. Nano Computing

Nano-related syntax may express:

- nano agents;
- atomic-scale entities;
- molecular entities;
- materials;
- interactions;
- nano protocols;
- nano capabilities.

The grammar must not hard-code a finite physical universe.

The semantic model determines whether a referenced entity or operation exists.

---

59. Contracts and Formal Verification

Zamani already has contract-oriented syntax in its grammar, including:

- "requires";
- "ensures";
- "invariant".

The architecture permits these to become formal semantic constraints.

A contract is not merely documentation.

When implemented as a language guarantee, it must have:

grammar
 ↓
AST
 ↓
semantic representation
 ↓
verification
 ↓
diagnostics

A source construct must not claim "formally proven" merely because it parsed.

---

60. Attributes and Annotations

Attributes and annotations may carry:

- compilation metadata;
- diagnostics metadata;
- optimization intent;
- capability declarations;
- verification requirements;
- domain metadata;
- interoperability metadata.

Their semantics must be registered.

Unknown attributes must have a defined policy:

error
warning
ignored
dialect-owned

They must not silently change program semantics.

---

61. AST Contract

Every grammar construct must have a predetermined AST contract before the grammar feature is considered complete.

The contract is:

Grammar rule
    ↓
AST node
    ↓
Semantic representation
    ↓
Canonical IR representation

No feature may be designed as:

grammar now
AST later

because that causes cross-file rework.

---

62. Domain-Neutral AST

The frontend AST must remain a source representation.

It must not become:

- LLVM IR;
- QIR;
- MLIR;
- physical quantum hardware model;
- routing graph;
- calibration database;
- scheduling DAG;
- QEC implementation;
- vendor backend IR.

For quantum operations, the preferred structure is generic operation semantics rather than a closed enum containing every gate.

Conceptually:

Operation {
    name
    namespace
    operands
    parameters
    results
    attributes
    modifiers
    effects
    capabilities
    source
}

The semantic quantum layer then maps that representation into "quantum::ir".

---

63. IR Integration

The repository's current top-level IR specification describes a typed SSA-like IR with arithmetic, calls, returns, phi nodes and basic values.

That IR specification must be reconciled with the richer domain-specific architecture.

The grammar design therefore imposes this rule:

«No grammar feature is complete until its semantic meaning has a defined destination in the canonical IR architecture.»

For classical constructs:

AST
 ↓
semantic model
 ↓
classical/control/data IR

For quantum constructs:

AST
 ↓
semantic quantum model
 ↓
quantum::ir

For HDL/hardware constructs:

AST
 ↓
hardware semantic model
 ↓
HDL/hardware IR

For cross-domain constructs:

AST
 ↓
semantic model
 ↓
coordinated canonical IR representation

The grammar must not create an IR merely to avoid integrating with existing IR.

---

64. No Duplicate Quantum IR

Prohibited:

src/quantum/frontend/QuantumIR

competing with:

src/quantum/ir

The frontend may have temporary semantic structures, but they must lower into the canonical "quantum::ir".

This rule is mandatory.

---

65. Routing

Routing owns:

logical resource
        ↓
physical realization

The grammar does not own physical coupling maps.

The grammar may express requirements such as:

requires connectivity(...)

but routing determines the actual realization.

---

66. Scheduling

Scheduling owns:

- ordering;
- timing;
- resource occupancy;
- dependencies;
- delays;
- dynamic scheduling;
- distributed scheduling;
- hardware timing.

The grammar can express scheduling intent/constraints.

It must not encode a particular schedule as universal language semantics.

---

67. Optimization

Optimization must preserve program semantics.

The grammar must not contain backend-specific optimization instructions merely to make a particular compiler fast.

Optimization belongs after semantic lowering.

---

68. Resilience

Resilience owns execution recovery/orchestration.

The grammar may express policies such as:

retry
recover
degrade
escalate
reject

where those are part of the language's formally defined execution semantics.

The implementation belongs to runtime/resilience infrastructure.

---

69. HAL

The Hardware Abstraction Layer owns:

- target capability;
- target state;
- physical resources;
- device interfaces;
- calibration;
- hardware-specific execution.

The grammar must remain independent of HAL implementation details.

---

70. Compilation and Target Selection

"grammar/compile/" owns portable compilation intent:

- target requirements;
- target selection policy;
- optimization profiles;
- specialization;
- cross compilation;
- reproducibility;
- deterministic builds;
- artifacts;
- deployment intent;
- provenance.

The programmer expresses intent.

The compiler discovers the actual target realization.

---

71. Execution

"grammar/execution/" owns portable execution intent:

- entry points;
- runtime environments;
- execution policy;
- placement intent;
- resilience;
- recovery;
- checkpointing;
- observability;
- tracing;
- profiling;
- lifecycle.

Execution syntax must not hard-code a particular machine topology.

---

72. Determinism

For identical:

source
language version
compiler version/configuration
explicit compilation configuration

the compiler must produce deterministic language-level interpretation.

Sources of nondeterminism must be explicitly controlled, including:

- hash iteration;
- random selection;
- parallel traversal order;
- unspecified backend selection;
- time;
- network state.

If nondeterminism is intentionally part of the language, it must be explicit and semantically represented.

---

73. Source Spans

Every syntactic construct that can produce a diagnostic MUST retain source-location information.

At minimum:

file identity
start position
end position

must be available.

This applies to:

- lexer diagnostics;
- parser diagnostics;
- type errors;
- semantic errors;
- resource errors;
- quantum errors;
- hardware compatibility errors;
- macro errors;
- interoperability errors.

---

74. Diagnostics

Diagnostics must be structured.

Conceptually:

Diagnostic {
    severity
    code
    message
    primary_span
    secondary_spans
    notes
    help
}

Errors must distinguish:

lexical
syntax
name resolution
type
effect
ownership
resource
capability
domain
IR
target
runtime

A target-resource failure must not be reported as a grammar failure.

---

75. Compatibility

"grammar/compatibility/" owns language evolution.

Compatibility must cover:

- language versions;
- grammar changes;
- token changes;
- AST changes;
- semantic changes;
- IR changes;
- dialect changes;
- deprecations;
- migrations.

Breaking changes require explicit versioning.

---

76. Feature Lifecycle

Every feature must have a lifecycle:

PROPOSED
    ↓
DESIGNED
    ↓
SPECIFIED
    ↓
GRAMMAR_IMPLEMENTED
    ↓
LEXER_IMPLEMENTED
    ↓
PARSER_IMPLEMENTED
    ↓
AST_IMPLEMENTED
    ↓
SEMANTIC_IMPLEMENTED
    ↓
IR_IMPLEMENTED
    ↓
COMPILER_INTEGRATED
    ↓
RUNTIME_INTEGRATED
    ↓
TESTED
    ↓
STABLE

Possible terminal/side states:

EXPERIMENTAL
DEPRECATED
REMOVED
REJECTED

No proposal document may imply implementation.

---

77. Feature Completeness Contract

Every feature MUST have all of the following predetermined:

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
Diagnostics
Security
Performance
Hard-Coding Audit
Completion Criteria

This is mandatory for independently completable work.

---

78. Independent-File Completion Rule

A file is complete only when its downstream contracts are already known.

For example:

quantum/operations.g4

cannot be declared complete until the project already knows:

- its AST representation;
- semantic representation;
- "quantum::ir" destination;
- parameter rules;
- control rules;
- target rules;
- diagnostics;
- compatibility;
- tests.

The file must not later require redesign simply because another subsystem was implemented.

If another subsystem discovers a genuine semantic contradiction, that is a specification change, not normal integration rework.

---

79. Domain Integration Contract

Every domain follows:

Domain Syntax
     ↓
Universal AST
     ↓
Domain Semantic Model
     ↓
Canonical IR
     ↓
Optimization
     ↓
Domain/Target Lowering

Domains may share semantic infrastructure but may not silently redefine:

- identifiers;
- source spans;
- expressions;
- types;
- diagnostics;
- resource semantics;
- capabilities.

---

80. Grammar-to-AST Traceability

Every public grammar rule must be traceable to an AST construct or an explicitly syntax-only construct.

No accepted syntax may disappear during parsing.

No AST node may exist without a defined source syntax or explicit compiler-generated origin.

Traceability must be testable.

---

81. AST-to-Semantic Traceability

Every AST construct must have:

semantic owner
validation rules
error model
resource model
capability model

If a construct has no semantic owner, it is incomplete.

---

82. Semantic-to-IR Traceability

Every semantically meaningful executable construct must have a defined IR destination.

The destination may be:

- existing generic IR;
- canonical quantum IR;
- classical/data IR;
- hardware/HDL IR;
- effect/resource metadata;
- another explicitly documented canonical representation.

There must be no "orphan semantics".

---

83. Grammar and Hardware Separation

The grammar MUST NOT depend on:

- CPU vendor;
- CPU ISA;
- GPU model;
- FPGA family;
- QPU model;
- physical qubit number;
- coupling map;
- pulse shape;
- calibration table;
- memory-bank count;
- physical network topology;
- accelerator count.

These are target facts.

The grammar may express requirements that later systems resolve.

---

84. Resource Feasibility

A resource failure must look conceptually like:

program requires:
    capability X
    resource Y

selected target provides:
    capability X
    resource Y'

Y' < Y

result:
    target/resource incompatibility

It must not become:

syntax error: resource too large

---

85. Infinity and Physical Reality

"Infinity" in POCO-REAF means:

«The language architecture has no arbitrary artificial finite machine ceiling.»

It does not mean:

«Actual hardware has infinite resources.»

All actual executions remain bounded by:

- memory;
- compute;
- storage;
- addressability;
- compiler resources;
- runtime resources;
- target capabilities;
- energy;
- time;
- external system limits.

The language must not confuse physical limits with language design limits.

---

86. No Fixed Quantum Gate Universe

The language must support future quantum operations without requiring a new grammar release for every new gate.

Prefer:

operation namespace/name
parameters
controls
targets
modifiers

over:

new gate → new keyword → new lexer token → new parser alternative

A new operation should normally be expressible through semantic registration.

---

87. No Fixed Mathematical Algorithm Universe

Likewise, a new algorithm must not require:

new keyword
new lexer token
new parser production

unless it genuinely changes language semantics.

This is especially important for:

- linear algebra;
- numerical methods;
- optimization;
- statistics;
- signal processing;
- AI;
- scientific computing.

---

88. No Framework Lock-In

Zamani syntax must not become a syntax wrapper for:

- CUDA;
- ROCm;
- TensorFlow;
- PyTorch;
- JAX;
- vendor QPU APIs;
- vendor FPGA tools;
- vendor CPU intrinsics.

Frameworks are implementation targets.

Zamani describes portable semantics.

---

89. Embedded and Systems Computing

The same architecture must support embedded targets.

Source may express:

- timing requirements;
- memory requirements;
- peripheral capabilities;
- real-time constraints;
- interrupt intent;
- communication;
- device capabilities.

But the language must not assume:

8-bit MCU
32-bit MCU
64 KB RAM
4 timers
2 UARTs

as universal properties.

---

90. HPC

HPC constructs must support:

- vectorization;
- parallel loops;
- distributed execution;
- reductions;
- communication;
- collective operations;
- accelerator usage;
- resource constraints.

The number of cores/nodes remains a target/resource property.

---

91. Cloud and Edge

The language may express:

- deployment requirements;
- locality;
- availability;
- replication;
- latency;
- bandwidth;
- service requirements;
- data locality.

Cloud provider names must not become language semantics.

---

92. Security and Provenance

All compilation-sensitive constructs should remain traceable.

Where applicable:

source
 ↓
AST
 ↓
semantic identity
 ↓
IR
 ↓
artifact

must preserve provenance.

Generated artifacts should be attributable to:

- source version;
- language version;
- compiler version;
- configuration;
- relevant target information.

---

93. Generated Artifacts

Generated grammar/parser files must never silently become source authority.

Generated artifacts must identify:

generated from
generator version
language version
source specification

Manual edits to generated files must either be prohibited or explicitly treated as source changes.

---

94. Testing Architecture

"grammar/tests/" must test the entire grammar boundary.

Required categories:

lexical/
syntax/
expressions/
types/
declarations/
functions/
modules/
effects/
memory/
concurrency/
classical/
quantum/
hybrid/
hdl/
hardware/
distributed/
ai/
data/
networking/
security/
interoperability/
dialects/
macros/
metaprogramming/
diagnostics/
negative/
boundary/
scalability/
determinism/
compatibility/

---

95. Positive Tests

Every feature requires valid examples.

Tests must cover:

- minimum valid form;
- ordinary form;
- composed form;
- nested form;
- cross-domain form.

---

96. Negative Tests

Every feature requires invalid examples.

Tests must verify:

- malformed syntax;
- invalid types;
- invalid resource requirements;
- invalid capabilities;
- invalid combinations;
- unsupported target requirements;
- duplicate declarations;
- ambiguity.

---

97. Boundary Tests

Boundary tests must verify:

- empty constructs where permitted;
- single-element constructs;
- very large representable values;
- deeply nested structures;
- large lists;
- large expression trees;
- large module graphs;
- large resource descriptions.

Boundary tests must never establish arbitrary language ceilings.

---

98. Scalability Tests

Scalability tests must verify that the language remains structurally valid as sizes grow.

Examples:

1 qubit
2 qubits
many qubits
symbolic qubit count

1 tensor dimension
many dimensions
large symbolic shapes

1 node
many nodes
symbolic deployment size

1 task
many tasks
nested parallelism

The test suite must distinguish:

language acceptance

from:

test-machine resource availability

---

99. Quantum Test Requirements

Quantum conformance must include:

- one-qubit programs;
- multi-qubit programs;
- parameterized operations;
- custom operations;
- multi-target operations;
- controlled operations;
- negative controls;
- adjoints;
- measurement;
- mid-circuit measurement;
- reset;
- classical feed-forward;
- dynamic control;
- logical qubits;
- physical mapping metadata;
- error-correction intent;
- noise requirements;
- resource requirements;
- unknown operations;
- unsupported target capabilities.

No test may establish a permanent maximum qubit count.

---

100. Cross-Domain Tests

Production readiness requires tests such as:

classical → quantum
quantum → classical
classical → HDL
HDL → accelerator
AI → tensor → accelerator
distributed → quantum
networking → distributed
security → distributed
resource → quantum
resource → AI
hardware → compilation
compile → execution

The objective is not merely that each domain parses independently.

The objective is that domains compose.

---

101. Conformance Matrix

A production feature must be traceable through:

Layer| Required
Specification| yes
Lexer| yes where lexically required
ANTLR| yes
Rust parser| yes
AST| yes
Semantic analysis| yes
Canonical IR| yes if executable
Compiler| yes
Runtime| yes where runtime semantics exist
Backend| yes for claimed backend support
Positive tests| yes
Negative tests| yes
Boundary tests| yes
Scalability tests| yes
Compatibility tests| yes
Documentation| yes

A feature with only grammar support is not production complete.

---

102. Hard-Coding Audit

Every grammar change MUST pass a hard-coding audit.

Search for concepts such as:

MAX_
LIMIT_
FIXED_
DEFAULT_QUBITS
DEFAULT_CORES
DEFAULT_THREADS
DEFAULT_GPUS
DEFAULT_NODES
QUBIT_0
QUBIT_1
CPU_0
GPU_0
NODE_0

These are not automatically prohibited.

They must be classified as one of:

program semantics
test fixture
backend-specific implementation
resource policy
language limit

Only the last category is forbidden when it artificially limits the universal language.

---

103. Resource Budgets

Resource budgets are permitted.

Examples:

compile budget
memory budget
time budget
optimization budget
execution budget
simulation budget
diagnostic budget

These are operational constraints.

They must not be confused with grammar limits.

---

104. Error Handling and Resource Exhaustion

The compiler must distinguish:

invalid program

from:

valid program but insufficient compiler resources

and:

valid program but insufficient target resources

and:

valid program but unsupported target capability

Each receives a distinct diagnostic category.

---

105. Grammar Performance

Grammar design must avoid pathological ambiguity.

Production validation must check:

- ambiguous alternatives;
- unreachable productions;
- redundant alternatives;
- precedence conflicts;
- left recursion;
- excessive backtracking;
- token conflicts;
- keyword collisions.

ANTLR grammar validation and Rust parser validation must agree on accepted language behavior.

---

106. ANTLR and Rust Parser Equivalence

The ANTLR grammar and Rust parser do not need identical internal implementation techniques.

They MUST have equivalent accepted-language semantics.

For conformance:

source
 ↓
ANTLR parser

and:

source
 ↓
reference Rust parser

must agree for stable syntax.

Any intentional difference requires a compatibility specification and test.

---

107. Reference Parser Authority

For the current repository:

src/lexer.rs
src/parser.rs
src/ast/

constitute the executable frontend implementation.

Therefore "grammar/grammar.md" must accurately report current implementation behavior.

"Zamani.g4" is the canonical external/ANTLR representation.

The specification defines what the implementation is intended to become.

---

108. Specification Versus Implementation

The architecture intentionally distinguishes:

Normative

from:

Implemented

A feature can be normative but not yet implemented.

A feature can be implemented experimentally but not stable.

A feature can be present in "Zamani-Grammar.md" without being normative.

This distinction is mandatory.

---

109. Existing "ZAMANI_TYPE_SYSTEM.md"

The root type-system document remains relevant.

"grammar/spec/type-system.md" must cross-reference it.

If contradictions exist, they must be explicitly resolved.

There must not be two different definitions of:

Int
Float
Qubit
QReg
Option
Result
linear
affine

without a versioned compatibility rule.

---

110. Existing "ZAMANI_IR_SPEC.md"

The root IR specification remains relevant.

"grammar/spec/" must describe grammar-to-IR contracts without redefining the entire IR.

If the simple root IR specification and richer domain IR architecture differ, the discrepancy must be resolved through the IR architecture rather than hidden in grammar rules.

The grammar must not compensate for IR incompleteness by introducing backend-specific syntax.

---

111. Repository-Wide Integration

A grammar feature is not complete until repository references have been audited.

At minimum:

grammar/
src/lexer.rs
src/parser.rs
src/ast/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs
src/quantum/
tests/
examples/
docs/

must be considered where relevant.

This prevents:

grammar says yes
lexer says no
parser says no
AST cannot represent it
IR cannot represent it

from being considered production-ready.

---

112. No Unnecessary Renames

The following existing filenames remain authoritative entry points:

grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/DESIGN.md
grammar/README.md

They must not be renamed merely for organizational aesthetics.

Directories should be added only where they establish a genuine ownership boundary.

---

113. Directory Expansion Policy

The desired conceptual grammar structure is:

grammar/
├── DESIGN.md
├── README.md
├── Zamani.g4
├── Zamani-Grammar.md
├── grammar.md
│
├── specification/
├── spec/
├── lexer/
├── core/
├── types/
├── expressions/
├── statements/
├── declarations/
├── functions/
├── modules/
├── effects/
├── memory/
├── concurrency/
├── classical/
├── quantum/
├── hybrid/
├── hdl/
├── hardware/
├── distributed/
├── ai/
├── data/
├── networking/
├── security/
├── resources/
├── compile/
├── execution/
├── interoperability/
├── dialects/
├── macros/
├── metaprogramming/
├── validation/
├── compatibility/
├── reference/
└── tests/

Not every directory requires immediate creation of every conceivable file.

No empty placeholder files should be created merely to make the tree look complete.

A directory becomes justified when it owns a real contract or implementation surface.

---

114. Specification Structure

"grammar/specification/" is the human-readable normative language specification.

It should ultimately contain, as justified:

README.md
language.md
lexical.md
syntax.md
semantics.md
types.md
effects.md
resources.md
portability.md
domains.md
diagnostics.md
versioning.md

These files own specification prose.

They do not replace "Zamani.g4".

---

115. Machine-Oriented "spec/"

"grammar/spec/" owns formal contracts and conformance material.

It should cover:

lexical.md
syntax.md
semantics.md
type-system.md
effects.md
resources.md
portability.md
quantum.md
classical.md
hdl.md
hybrid.md
concurrency.md
distributed.md
ai.md
data.md
networking.md
security.md
interoperability.md
diagnostics.md
source-spans.md
determinism.md
versioning.md
compatibility.md

No document should duplicate another authority unnecessarily.

---

116. Feature Manifests

For complex or cross-domain features, the architecture SHOULD support machine-readable feature manifests.

Conceptually:

specification/features/
├── quantum-operations.yaml
├── quantum-measurement.yaml
├── quantum-control.yaml
├── hdl-module.yaml
├── tensor.yaml
├── resources.yaml
├── distributed.yaml
└── ...

Each manifest may contain:

id:
name:
status:
version:
syntax:
tokens:
grammar_rules:
ast_nodes:
semantic_rules:
ir_mapping:
compiler_consumers:
runtime_consumers:
capabilities:
resource_requirements:
positive_tests:
negative_tests:
boundary_tests:
scalability_tests:
compatibility:
hard_coding_policy:

This is the preferred mechanism for satisfying the requirement that a feature can be completed independently without reopening unrelated files later.

---

117. Feature Manifest Rule

A feature manifest is not another grammar authority.

It is a traceability contract.

The source of truth remains:

normative specification
+
canonical grammar
+
reference implementation

The manifest connects them.

---

118. Completion Criteria for a Grammar File

A grammar file is complete only if:

[ ] Purpose defined
[ ] Ownership defined
[ ] Non-ownership defined
[ ] Inputs defined
[ ] Outputs defined
[ ] Dependencies defined
[ ] Upstream contracts defined
[ ] Downstream consumers defined
[ ] Grammar contract complete
[ ] AST mapping complete
[ ] Semantic mapping complete
[ ] IR destination defined
[ ] Diagnostics defined
[ ] Compatibility defined
[ ] Positive tests exist
[ ] Negative tests exist
[ ] Boundary tests exist
[ ] Scalability tests exist
[ ] Determinism tested
[ ] Hard-coding audit passed
[ ] Security reviewed
[ ] Performance implications reviewed
[ ] Documentation linked
[ ] No unresolved TODO affecting correctness

---

119. Completion Criteria for a Domain

A domain is production-ready only when:

syntax
+
lexer
+
parser
+
AST
+
semantics
+
IR
+
compiler integration
+
runtime integration
+
tests
+
documentation
+
compatibility

are all aligned.

Therefore:

«"quantum/" is not production-ready merely because quantum syntax parses.»

The same applies to:

- HDL;
- AI;
- distributed;
- networking;
- security;
- classical;
- hybrid.

---

120. Production Readiness Gate

The grammar subsystem is production-ready only when all of the following are true:

Authority

[ ] One normative language authority
[ ] Zamani.g4 reconciled
[ ] grammar.md reflects implementation
[ ] Zamani-Grammar.md status-marked

Lexical

[ ] Token registry canonical
[ ] Duplicate token meanings resolved
[ ] Literals deterministic
[ ] Unicode policy defined
[ ] Source spans preserved
[ ] Errors structured

Parsing

[ ] Rust parser conforms
[ ] ANTLR parser conforms
[ ] Recovery makes progress
[ ] Precedence deterministic
[ ] Deep structures supported

AST

[ ] Every accepted construct has representation
[ ] Domain-neutral
[ ] Source spans preserved
[ ] No hardware realization

Semantics

[ ] Type rules defined
[ ] Effect rules defined
[ ] Resource rules defined
[ ] Capability rules defined
[ ] Ownership rules defined
[ ] Domain rules defined

IR

[ ] Every executable construct has an IR destination
[ ] quantum::ir remains canonical quantum boundary
[ ] No duplicate quantum IR

Portability

[ ] No artificial machine limits
[ ] Resource requirements separated from realization
[ ] Target selection downstream
[ ] Capability negotiation defined

Safety

[ ] Rust 1.97/1.97.1
[ ] no Rust unsafe
[ ] no hidden unsafe implementation
[ ] deterministic behavior

Testing

[ ] positive
[ ] negative
[ ] boundary
[ ] scalability
[ ] deterministic
[ ] compatibility
[ ] cross-domain

---

121. Implementation Order

The production implementation MUST proceed independently-first.

Phase 1 — authority

1. "grammar/DESIGN.md"
2. "grammar/README.md"
3. "grammar/specification/README.md"
4. "grammar/specification/language.md"
5. "grammar/specification/lexical.md"
6. "grammar/specification/syntax.md"
7. "grammar/specification/semantics.md"
8. "grammar/specification/portability.md"
9. "grammar/spec/type-system.md"
10. "grammar/spec/compatibility.md"

Phase 2 — lexical foundation

11. token contract
12. keyword contract
13. operator contract
14. identifier contract
15. literal contract
16. comment contract
17. Unicode contract
18. quantum-literal contract

Phase 3 — universal syntax

19. core names
20. paths
21. attributes
22. modifiers
23. blocks
24. expression precedence
25. expressions
26. types
27. declarations
28. statements
29. functions
30. modules

Phase 4 — language semantics

31. effects
32. memory
33. concurrency
34. resources
35. capabilities

Phase 5 — domains

36. classical
37. quantum
38. hybrid
39. HDL
40. hardware
41. distributed
42. AI
43. data
44. networking
45. security

Phase 6 — advanced facilities

46. compile
47. execution
48. interoperability
49. dialects
50. macros
51. metaprogramming
52. Sankofa
53. MTS
54. nano

Phase 7 — composition

55. reconcile "Zamani.g4"

Phase 8 — implementation conformance

56. reconcile "grammar.md"

Phase 9 — validation

57. positive tests
58. negative tests
59. boundary tests
60. scalability tests
61. determinism tests
62. compatibility tests
63. cross-domain tests

This order ensures foundational contracts exist before dependent grammar files are finalized.

---

122. What Must Not Happen

The following are architectural violations:

1. Renaming "Zamani.g4" without necessity.
2. Renaming "grammar.md" without necessity.
3. Renaming "Zamani-Grammar.md" without necessity.
4. Creating another competing root grammar.
5. Maintaining two canonical ANTLR grammars.
6. Creating a second quantum IR.
7. Hard-coding a universal qubit maximum.
8. Hard-coding CPU/core/thread limits.
9. Hard-coding GPU/FPGA/QPU counts.
10. Hard-coding network-node limits.
11. Hard-coding tensor rank/dimension limits.
12. Hard-coding timeline counts.
13. Making vendor APIs part of core syntax.
14. Making every mathematical algorithm a keyword.
15. Making every quantum gate a lexer keyword.
16. Making QEC a parser implementation.
17. Making ZQN a parser implementation.
18. Making HAL a grammar implementation.
19. Making routing a grammar implementation.
20. Making scheduling a grammar implementation.
21. Treating "Zamani-Grammar.md" as automatically implemented.
22. Treating "grammar.md" as the normative future specification.
23. Allowing grammar features without AST mappings.
24. Allowing AST nodes without semantic contracts.
25. Allowing executable semantics without IR destinations.
26. Allowing resource failures to appear as syntax failures.
27. Allowing target-specific decisions to alter source semantics silently.
28. Using Rust "unsafe" in the compiler implementation.
29. Adding empty directories/files solely for visual completeness.
30. Introducing a feature whose integration contracts are intentionally deferred.

---

123. Final Architectural Contract

The definitive Zamani architecture is:

                       ZAMANI SOURCE
                             │
                             ▼
                    grammar/Zamani.g4
                             │
                             ▼
                          LEXER
                             │
                             ▼
                          PARSER
                             │
                             ▼
                       FRONTEND AST
                             │
                             ▼
             ┌───────────────────────────────┐
             │ Structural Validation         │
             │ Name Resolution              │
             │ Type Analysis                │
             │ Effect Analysis              │
             │ Ownership/Linearity          │
             │ Capability Analysis          │
             │ Resource Analysis            │
             │ Portability Validation       │
             └───────────────────────────────┘
                             │
                             ▼
                 CANONICAL SEMANTIC MODEL
                             │
          ┌──────────────────┼───────────────────┐
          │                  │                   │
          ▼                  ▼                   ▼
      CLASSICAL          QUANTUM              HDL/
      SEMANTICS          SEMANTICS          HARDWARE
          │                  │                   │
          │                  ▼                   │
          │             quantum::ir             │
          │                  │                   │
          └──────────────────┼───────────────────┘
                             │
                             ▼
                    CANONICAL IR LAYER
                             │
                             ▼
                       OPTIMIZATION
                             │
             ┌───────────────┼─────────────────┐
             │               │                 │
             ▼               ▼                 ▼
          ROUTING         SCHEDULING       RESILIENCE
             │               │                 │
             └───────────────┼─────────────────┘
                             │
                             ▼
                            ZQN
                             │
                             ▼
                            HAL
                             │
                             ▼
                      TARGET LOWERING
                             │
       ┌───────────┬─────────┼──────────┬────────────┐
       │           │         │          │            │
       ▼           ▼         ▼          ▼            ▼
      CPU         GPU       FPGA       QPU       Future Targets

The central rule is:

«Zamani source describes portable computation and intent. It does not describe today's machine limits.»

The grammar must therefore remain independent of:

physical qubit count
CPU count
core count
thread count
GPU count
FPGA count
QPU topology
memory capacity
register width
network size
accelerator count
vendor instruction set
calibration
physical placement

Those properties are resolved by later compilation/runtime infrastructure.

---

124. Final POCO-REAF Guarantee

The architecture is considered successful when the same Zamani source can remain semantically valid while the compiler chooses different realizations:

                 SAME ZAMANI PROGRAM
                         │
             ┌───────────┼────────────┐
             ▼           ▼            ▼
          tiny target  medium      enormous target
             │           │            │
             ▼           ▼            ▼
           CPU        GPU/FPGA       QPU/HPC/
                                    distributed/
                                    future target

without requiring the programmer to rewrite the algorithm merely because:

- the number of processors changed;
- the number of GPUs changed;
- the number of qubits changed;
- the memory capacity changed;
- the network topology changed;
- the accelerator changed;
- the hardware generation changed;
- the deployment scale changed.

The compiler may make different implementation decisions.

The program's semantics must remain the same.

---

125. Final Rule

The entire "grammar/" subsystem exists to enforce one boundary:

                 PORTABLE PROGRAM MEANING
                            │
                 ───────────┼───────────
                            │
                     GRAMMAR / AST
                            │
                 ───────────┼───────────
                            │
                    SEMANTIC MODEL
                            │
                 ───────────┼───────────
                            │
                       CANONICAL IR
                            │
                 ───────────┼───────────
                            │
                TARGET REALIZATION
                            │
          ┌─────────────────┼─────────────────┐
          │                 │                 │
         CPU               GPU               QPU
          │                 │                 │
         FPGA             Cluster          Future

Grammar defines portable meaning.

Semantic analysis defines validity.

Canonical IR defines computation.

Optimization transforms computation without changing meaning.

Routing realizes resources.

Scheduling realizes time and resource ordering.

QEC realizes error-correction strategy.

ZQN models quantum faults/noise.

HAL realizes target capabilities.

Runtime executes the result.

No downstream subsystem may be pulled upward into the grammar merely because it is convenient.

No hardware limitation may be pulled upward into the language merely because it exists today.

No frontend quantum representation may replace "quantum::ir".

No Rust "unsafe" is permitted in the compiler implementation.

No feature is production-ready until its complete specification → lexer → parser → AST → semantic → IR → compiler/runtime → test chain is defined.

This is the architectural basis for Zamani to pursue:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever — from the smallest computation to arbitrarily large computation, constrained by actual resources rather than artificial language limits.»