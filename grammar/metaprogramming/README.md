Zamani Metaprogramming Grammar

Path: "grammar/metaprogramming/"
Language: Zamani Universal Programming Language
Subsystem: Metaprogramming, compile-time computation, reflection, quotation, generation, specialization, type-level computation, schema metaprogramming, and related source-transformation facilities
Grammar technology: ANTLR4 parser-grammar composition
Compiler baseline: Rust 1.97 / Rust 1.97.1, Rust 2021
Safety: Safe Rust only; production compiler implementation MUST NOT require or use "unsafe"
Portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)"
Authority: "grammar/specification/" and "grammar/spec/" are normative; "grammar/Zamani.g4" is the canonical combined ANTLR root; this README is the metaprogramming subsystem contract.

---

1. Purpose

"grammar/metaprogramming/" defines the source-language syntax and integration contracts for Zamani metaprogramming.

Metaprogramming allows a Zamani program to reason about, construct, transform, specialize, inspect, or generate Zamani program structure while remaining inside the same language architecture.

Metaprogramming may support:

- compile-time computation;
- compile-time values;
- compile-time functions;
- compile-time bindings;
- quotation;
- unquotation/splicing;
- source generation;
- reflection;
- introspection;
- specialization;
- type-level computation;
- schema metaprogramming;
- source transformation;
- syntax-tree manipulation;
- compile-time validation;
- compile-time derivation;
- compile-time structural construction;
- compile-time domain adaptation;
- generation of classical source;
- generation of quantum source;
- generation of hybrid source;
- generation of HDL source;
- generation of hardware intent;
- generation of distributed/data/AI/networking/security source.

Metaprogramming is not a separate programming language.

It is a phase-aware extension of Zamani's existing language.

The fundamental pipeline is:

Zamani source
    |
    v
canonical lexer
    |
    v
canonical parser
    |
    v
domain-neutral frontend AST
    |
    v
structural analysis
    |
    v
name / type / effect / capability / resource analysis
    |
    v
metaprogramming validation
    |
    v
authorized compile-time evaluation/transformation
    |
    v
validated Zamani source structure / semantic model
    |
    v
canonical semantic representation
    |
    v
canonical IR
    |
    +-------------------+-------------------+
    |                   |                   |
    v                   v                   v
 classical            quantum::ir       HDL/hardware
    |                   |                   |
    +-------------------+-------------------+
                        |
                        v
                 optimization/lowering
                        |
             +----------+----------+
             |          |          |
             v          v          v
          routing   scheduling  resilience
                                   |
                                  QEC
                                   |
                                  ZQN
                                   |
                                  HAL
                                   |
                                  v
                           target realization

The invariant is:

«Metaprogramming may transform Zamani source meaning, but it MUST NOT replace, bypass, or create a competing semantic pipeline.»

---

2. Production Status

This README is the production architecture and completion contract for the subsystem.

It does not falsely declare every existing grammar component to be fully implemented by the Rust compiler.

A metaprogramming feature is "STABLE" only when all required stages have been completed:

specification
    ↓
lexical contract
    ↓
grammar contract
    ↓
canonical parser integration
    ↓
frontend AST mapping
    ↓
semantic validation
    ↓
compile-time execution/transformation where applicable
    ↓
canonical semantic representation
    ↓
canonical IR mapping
    ↓
compiler integration
    ↓
runtime/target integration where applicable
    ↓
positive tests
    ↓
negative tests
    ↓
boundary tests
    ↓
scalability tests
    ↓
determinism tests
    ↓
compatibility tests
    ↓
security review

A ".g4" file existing in the repository does not by itself establish that its feature is implemented.

---

3. Authority Model

Zamani has one language.

Metaprogramming is subordinate to the repository-wide authority model.

The authority order is:

grammar/specification/
        ↓
normative language specification
        ↓
grammar/spec/
        ↓
formal subsystem contracts
        ↓
grammar/Zamani.g4
        ↓
grammar/antlr/ZamaniParser.g4
        ↓
modular grammar components
        ↓
src/lexer.rs / canonical lexer
        ↓
src/parser.rs / parser implementation
        ↓
frontend AST
        ↓
semantic analysis
        ↓
canonical semantic model
        ↓
canonical IR
        ↓
compiler
        ↓
runtime / HAL / target realization

The following files have distinct roles:

Artifact| Authority
"grammar/specification/"| Normative language specification
"grammar/spec/"| Formal contracts/conformance rules
"grammar/DESIGN.md"| Normative grammar architecture
"grammar/Zamani.g4"| Canonical ANTLR root
"grammar/antlr/ZamaniParser.g4"| Canonical parser composition
"grammar/lexer/"| Lexical contracts
"grammar/macros/"| Macro syntax and macro-specific contracts
"grammar/metaprogramming/"| Metaprogramming syntax contracts
"src/lexer.rs"| Rust lexer implementation
"src/parser.rs"| Rust parser implementation
"src/frontend/ast/"| Domain-neutral frontend AST where established by the repository
semantic subsystem| Meaning and validity
"quantum::ir"| Canonical quantum IR boundary
compiler| Optimization/lowering/target realization
runtime/HAL| Execution and target interaction
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Historical/extended design reference

"Zamani-Grammar.md" MUST NOT make a feature legal merely because it describes it.

"grammar.md" MUST NOT make a feature normative merely because an implementation happens to accept it.

This README MUST NOT override either normative specification.

---

4. Core Architectural Principle

Metaprogramming operates above the canonical semantic model.

It may produce or transform source structure.

It may compute compile-time values.

It may inspect permitted language metadata.

It may request specialization.

It may generate source.

It may construct source structures.

It may derive types or schemas.

It may participate in compile-time validation.

It MUST NOT directly become:

- machine code;
- a backend;
- a scheduler;
- a router;
- a QEC engine;
- a ZQN engine;
- a HAL;
- a hardware discovery system;
- a physical device allocator;
- a physical qubit allocator;
- a runtime execution engine.

The metaprogramming boundary is:

source structure
       ↓
metaprogramming operation
       ↓
validated source structure
       ↓
ordinary semantic pipeline

not:

source
  ↓
metaprogram
  ↓
machine instructions

---

5. Scope

5.1 This directory owns

"grammar/metaprogramming/" owns:

- metaprogramming grammar composition;
- compile-time syntax;
- quotation syntax;
- unquotation/splicing syntax;
- reflection syntax;
- source-generation syntax;
- specialization syntax;
- type-level metaprogramming syntax;
- schema metaprogramming syntax;
- introspection syntax where distinct from ordinary reflection;
- metaprogramming-specific source boundaries;
- metaprogramming-specific syntactic metadata;
- phase-boundary syntax;
- integration contracts among those facilities.

5.2 This directory does not own

It does not own:

- lexer implementation;
- token implementation;
- ordinary identifier syntax;
- ordinary name syntax;
- ordinary paths;
- ordinary expressions;
- ordinary statements;
- ordinary declarations;
- ordinary type syntax;
- generic declarations;
- ordinary schemas;
- macro implementation;
- macro expansion;
- macro hygiene implementation;
- name resolution;
- type checking;
- effect checking;
- capability authorization;
- resource allocation;
- hardware discovery;
- target selection;
- device selection;
- physical qubit selection;
- routing;
- scheduling;
- optimization;
- QEC;
- ZQN;
- resilience implementation;
- HAL;
- runtime execution;
- classical IR;
- "quantum::ir";
- HDL/hardware IR.

---

6. Existing Files and Their Permanent Ownership

The current repository already contains:

grammar/metaprogramming/
├── README.md
├── capabilities.g4
├── code-generation.g4
├── compile-time-execution.g4
├── compile-time.g4
├── generation.g4
├── introspection.g4
├── metaprogramming.g4
├── quotation.g4
├── reflection.g4
├── schemas.g4
├── specialization.g4
├── type-level.g4
└── unquotation.g4

These files MUST NOT be unnecessarily renamed.

Their ownership is fixed below.

---

7. "metaprogramming.g4"

Purpose

Composition root for the metaprogramming subsystem.

Owns

- metaprogramming declaration dispatch;
- metaprogramming expression dispatch;
- metaprogramming statement dispatch;
- integration of compile-time facilities;
- integration of generation;
- integration of reflection;
- integration of specialization;
- integration of quotation/unquotation;
- integration of type-level metaprogramming;
- integration of schema metaprogramming;
- integration of introspection;
- integration of metaprogramming capabilities.

Does not own

It MUST NOT redefine the syntax owned by:

- "macros/";
- "compile-time-execution.g4";
- "compile-time.g4";
- "generation.g4";
- "code-generation.g4";
- "quotation.g4";
- "unquotation.g4";
- "reflection.g4";
- "introspection.g4";
- "specialization.g4";
- "type-level.g4";
- "schemas.g4";
- "capabilities.g4".

Required integration model

The composition layer should expose stable wrappers such as:

metaprogrammingDeclaration
metaprogrammingExpression
metaprogrammingStatement

and delegate to exactly one owning component.

No duplicate public grammar hierarchy is permitted.

Completion condition

This file is complete when:

- every referenced rule exists;
- every referenced rule has one owner;
- no phantom rule is referenced;
- no duplicate public rule exists;
- quotation has one owner;
- unquotation has one owner;
- generation has one owner;
- reflection has one owner;
- introspection has one owner;
- specialization has one owner;
- type-level syntax has one owner;
- ordinary expression/type/name rules are reused;
- canonical lexer vocabulary is used;
- no parser action executes code.

---

8. "compile-time-execution.g4"

Purpose

Owns the source-level boundary for explicitly requested compile-time execution.

Owns

- compile-time execution contexts;
- compile-time blocks;
- compile-time evaluation boundaries;
- compile-time sequencing;
- compile-time value production;
- compile-time generated-source boundaries;
- explicit compile-time execution requests.

Does not own

It does not own:

- evaluator implementation;
- interpreter implementation;
- constant folding;
- optimization;
- specialization algorithms;
- macro expansion;
- reflection semantics;
- source generation semantics;
- runtime execution;
- hardware discovery.

Semantic requirement

A parsed compile-time construct MUST first undergo:

parse
 ↓
AST
 ↓
name resolution
 ↓
type checking
 ↓
effect checking
 ↓
capability checking
 ↓
resource checking
 ↓
security validation
 ↓
compile-time eligibility
 ↓
authorized evaluation
 ↓
result validation

Parsing never executes compile-time code.

---

9. "compile-time.g4"

Purpose

Provides the compile-time integration boundary used by other grammar domains.

Owns

- compile-time integration wrappers;
- compile-time declaration/expression/statement bridges;
- compile-time semantic-context wrappers;
- compile-time transformation boundaries.

Does not own

It MUST NOT become a second implementation of:

- compile-time execution;
- source generation;
- reflection;
- specialization;
- ordinary compile-time functions.

Where another file already owns the concrete construct, this file MUST delegate.

---

10. "generation.g4"

Purpose

Owns source generation.

The existing repository uses "SYNTHESIZE" as the established source-generation vocabulary. This component must therefore use that canonical lexical contract rather than independently inventing a competing "GENERATE" keyword.

Owns

- source-generation requests;
- generated source structures;
- generated declarations;
- generated statements;
- generated expressions;
- generated types where explicitly supported;
- generated items;
- generation result categories;
- generation metadata;
- generation provenance;
- generation context;
- source-structure construction.

Does not own

It does not generate:

- machine instructions;
- object files;
- executables;
- GPU instructions;
- FPGA bitstreams;
- ASIC layouts;
- QPU instructions;
- physical schedules.

Those belong downstream.

Mandatory pipeline

synthesize
    ↓
generated Zamani source structure
    ↓
canonical parser/AST
    ↓
semantic analysis
    ↓
canonical semantic model
    ↓
canonical IR

Generated source MUST re-enter the normal language pipeline.

---

11. "code-generation.g4"

Purpose

Separates metaprogram source generation from compiler/backend code generation.

This distinction is mandatory.

"generation.g4"

Means:

«Construct or request Zamani source structure.»

"compile/code-generation.g4"

Means:

«Request compiler-level artifact/code-generation behavior.»

"code-generation.g4" MUST NOT become a second source-generation grammar.

It MUST delegate source construction to "generation.g4" where appropriate.

It MUST NOT emit machine code from the parser.

---

12. "quotation.g4"

Purpose

Owns canonical quotation syntax.

Quotation represents Zamani source structure as a compile-time metaprogramming value.

The existing architecture establishes:

quote { ... }

as the canonical quotation concept.

Quotation MUST reuse canonical Zamani syntax inside the quotation.

A quoted expression is not allowed to become a second expression language.

Owns

- quotation boundaries;
- quote expressions;
- quotation bodies;
- quotation category;
- quotation integration boundary;
- quotation provenance boundary;
- quotation nesting.

Does not own

It does not own:

- macro expansion;
- macro hygiene implementation;
- reflection;
- source-generation implementation;
- specialization;
- compile-time evaluation implementation;
- AST implementation;
- IR.

---

13. "unquotation.g4"

Purpose

Provides the independently named unquotation/splicing integration surface while respecting the existing canonical quotation contract.

The current repository deliberately uses "quotation.g4" as the canonical quotation/unquotation core and this file as an integration wrapper.

That ownership model MUST remain explicit.

Owns

- unquotation integration wrappers;
- unquotation/splice source boundaries;
- unquotation semantic-category boundaries;
- independent composition names where required.

Does not own

It MUST NOT create a second independent definition of:

unquoteExpressionCore

when that core remains owned by "quotation.g4".

The canonical relationship is:

quotation.g4
    ├── quoteExpressionCore
    ├── unquoteExpressionCore
    └── metaQuoteCore / metaSpliceCore

unquotation.g4
    └── integration wrappers

If ownership is ever moved, the move MUST be made atomically across specification, grammar, parser composition, lexer, AST, semantic analysis, and tests.

---

14. Quotation/Unquotation Phase Rules

Quotation establishes a source-structure phase.

Unquotation crosses from a compile-time semantic value into the enclosing quoted structure.

Therefore:

quote {
    ...
    unquote expression
    ...
}

is semantically different from ordinary runtime evaluation.

"unquote" outside a valid quotation context MUST be diagnosed semantically.

The parser may recognize its syntax, but the parser MUST NOT attempt to determine whether the enclosing semantic phase is valid.

---

15. No Quasiquotation by Accident

The subsystem MUST NOT silently introduce a third quotation language.

The canonical concepts are:

quote
unquote

A future "quasiquote", template literal, splice operator, or alternative quotation mechanism requires its own complete language-feature contract.

It MUST NOT be smuggled into an existing quotation rule merely because the implementation can parse it.

---

16. "reflection.g4"

Purpose

Owns ordinary language-defined reflection.

Reflection is a view over the canonical semantic model.

It may inspect permitted source-level information such as:

- declarations;
- functions;
- types;
- generic parameters;
- attributes;
- effects;
- capabilities;
- resource requirements;
- modules;
- operations;
- schema information;
- domain metadata.

Owns

- reflection expressions;
- reflection subjects;
- reflection projections;
- reflection selectors;
- reflection query chains;
- explicit type reflection;
- source-level reflection syntax.

Does not own

Reflection MUST NOT become:

- hardware discovery;
- device discovery;
- physical qubit inspection;
- runtime memory inspection;
- arbitrary host introspection;
- filesystem inspection;
- environment inspection;
- network inspection;
- second type system;
- second AST;
- second IR.

Subject rule

Reflection MUST NOT recursively consume the complete "expression" grammar as its subject in a way that creates:

expression
 → reflection
 → expression

Use canonical names, paths, and type expressions where the specification defines reflection subjects.

If arbitrary expression reflection is ever needed, it MUST be introduced through explicit quotation or another phase-safe semantic mechanism.

---

17. "introspection.g4"

Purpose

"introspection.g4" handles introspection facilities that are semantically distinct from ordinary language reflection.

The distinction is:

Reflection

Inspection of canonical language-defined semantic structure.

Introspection

Inspection of explicitly authorized broader semantic/environmental metadata.

Introspection MUST NOT silently imply hardware discovery.

For example:

reflect(hardware::capability)

can mean inspection of a source-declared capability or canonical semantic capability description.

It MUST NOT automatically mean:

query physical GPU
query QPU
query CPU
query FPGA

Those are downstream capability/resource concerns.

Ownership rule

"reflection.g4" remains authoritative for ordinary reflection.

"introspection.g4" MUST NOT redefine "reflectionExpressionCore".

---

18. "specialization.g4"

Purpose

Owns source-level specialization requests.

Specialization expresses intent to adapt a generic or parameterized program under known semantic information.

Example conceptual form:

specialize compute<T>

or:

specialize datapath<WordType> with (...)

Owns

- specialization requests;
- specialization targets;
- specialization arguments;
- type specialization arguments;
- value specialization arguments;
- named specialization arguments;
- specialization policies;
- specialization metadata.

Does not own

It does not perform:

- monomorphization;
- optimization;
- constant folding;
- target selection;
- hardware selection;
- physical mapping;
- routing;
- scheduling;
- QEC;
- ZQN;
- runtime dispatch.

Lexical prerequisite

The current specialization grammar refers to "K_SPECIALIZE".

The checked Rust lexer currently does not define that token.

Therefore production integration requires a single canonical lexical decision:

specification
    ↓
canonical lexer vocabulary
    ↓
ANTLR lexer
    ↓
Rust lexer
    ↓
parser

Do not add "K_SPECIALIZE" independently to multiple lexical systems.

If the final lexical name is changed, all consumers must change together.

---

19. "type-level.g4"

Purpose

Owns source-level type metaprogramming.

Type-level metaprogramming may describe:

- type-level values;
- type-level bindings;
- type-level transformations;
- type construction;
- type application;
- type predicates;
- normalization requests;
- type-level conditionals;
- type-level iteration;
- type-level specialization;
- type-level extensions.

Critical rule

Ordinary types remain owned by:

grammar/types/

Type-level metaprogramming MUST consume canonical type expressions rather than creating a second type language.

It MUST NOT define a competing:

TypeLevelType

that becomes an alternative universal type system.

Internal compiler representations may exist downstream, but they are not grammar authorities.

---

20. Type-Level Scalability

There must be no grammar-level limits such as:

MAX_TYPE_LEVEL_DEPTH
MAX_TYPE_LEVEL_OPERATIONS
MAX_TYPE_LEVEL_ARGUMENTS
MAX_TYPE_GENERATIONS
MAX_TYPE_SPECIALIZATIONS

Type-level computation may become resource-intensive.

That is an implementation/resource-management problem.

Compiler policy may impose:

- execution budgets;
- memory budgets;
- cancellation;
- recursion protection;
- evaluation-step limits;
- compilation timeouts.

Those are not language semantics.

---

21. "schemas.g4"

Purpose

Owns metaprogramming over logical schemas.

It is distinct from:

grammar/data/schemas.g4

which owns ordinary data-schema syntax.

The relationship is:

data/schemas.g4
    ↓
ordinary logical schema

metaprogramming/schemas.g4
    ↓
compile-time schema inspection/transformation/generation

"metaprogramming/schemas.g4" MUST NOT redefine ordinary schema declarations.

It may describe:

- schema references;
- schema projections;
- schema transformations;
- schema composition;
- schema derivation;
- schema validation requests;
- schema generation;
- schema compatibility;
- schema evolution;
- schema provenance.

---

22. "capabilities.g4"

Purpose

Provides metaprogramming-specific source-level capability integration where required.

Capabilities describe authorization or semantic availability.

They are not physical device identifiers.

The grammar MUST preserve the distinction:

requirement
capability
preference
hint
implementation decision

For example:

requires capability("quantum.measurement")

does not mean:

use QPU 0

Likewise:

requires capability("gpu.compute")

does not mean:

use GPU 0

---

23. Capability Security

Metaprogramming MUST NOT gain implicit capabilities merely because it runs during compilation.

In particular, parsing or evaluating a metaprogram does not automatically authorize:

- filesystem access;
- network access;
- process spawning;
- environment inspection;
- credentials;
- secrets;
- hardware access;
- device discovery;
- QPU access;
- GPU access;
- FPGA access;
- arbitrary host memory;
- compiler-global mutation.

Such operations, if supported by the language, require explicit semantic authorization and appropriate effect/capability/security contracts.

---

24. Macro Integration

Macros remain owned by:

grammar/macros/

Metaprogramming MUST NOT redefine macro declarations or invocations.

The integration is:

macro syntax
    ↓
macro AST
    ↓
macro resolution
    ↓
macro expansion
    ↓
hygiene/provenance
    ↓
generated Zamani source structure
    ↓
ordinary semantic validation

A macro may consume quotation values.

A macro may generate quotation values.

A macro may participate in metaprogramming.

But:

«Quotation is not a hidden macro implementation.»

And:

«Metaprogramming is not a mechanism for bypassing macro hygiene.»

---

25. Hygiene

Generated identifiers MUST preserve the language's hygiene/provenance rules.

Metaprogramming MUST NOT provide an implicit unhygienic escape hatch.

Where the macro subsystem owns hygiene, metaprogramming consumes that contract.

Generated source must preserve enough information to distinguish:

original source
macro definition
macro invocation
quotation
unquotation
generated source
specialization

The precise data structure belongs to the frontend/compiler implementation.

The grammar only establishes the syntactic boundaries needed to preserve that information.

---

26. Provenance

Every metaprogramming construct that transforms source structure must remain traceable.

At minimum, downstream representations must be capable of preserving:

- original source span;
- metaprogramming construct span;
- quotation origin;
- unquotation origin;
- macro origin where applicable;
- generation origin;
- specialization origin;
- transformation identity/version where applicable.

Provenance is necessary for:

- diagnostics;
- debugging;
- source maps;
- reproducibility;
- verification;
- incremental compilation;
- generated-code inspection;
- security auditing.

---

27. AST Contract

Metaprogramming MUST use the canonical frontend AST architecture.

It MUST NOT introduce:

MetaAst
MetaprogrammingAst
QuantumMetaAst
HardwareMetaAst
TypeLevelAst
ReflectionAst
GenerationAst

as competing universal ASTs.

Internal implementation structures may exist where justified, but they are downstream implementation details.

The parser must map syntax into the canonical frontend representation.

Every stable metaprogramming feature must specify:

grammar rule
    ↓
AST node/category
    ↓
semantic meaning

before being considered complete.

---

28. AST Information That Must Survive

Metaprogramming transformations must preserve, as applicable:

- source span;
- source ordering;
- construct category;
- identifier/path;
- arguments;
- generic parameters;
- quotation category;
- unquotation category;
- attributes;
- phase information;
- provenance;
- hygiene information;
- transformation origin.

A generated node must not become indistinguishable from unrelated source when that distinction is required for diagnostics or semantic correctness.

---

29. Semantic Contract

Parsing establishes syntax only.

Semantic analysis establishes:

- whether a construct is legal;
- whether the construct is in the correct phase;
- whether names resolve;
- whether types are valid;
- whether generic arguments are valid;
- whether effects are permitted;
- whether capabilities are authorized;
- whether resource requirements are satisfiable;
- whether compile-time execution is permitted;
- whether reflection is permitted;
- whether generated structure is valid;
- whether unquotation is valid;
- whether specialization is meaningful;
- whether type-level computation is legal;
- whether schema transformations are valid;
- whether generated semantics preserve program meaning.

---

30. Metaprogramming Must Not Bypass Validation

Generated or transformed source MUST NOT bypass:

- lexical validation;
- parsing;
- name resolution;
- type checking;
- effect checking;
- capability checking;
- resource checking;
- ownership checking;
- security validation;
- domain validation;
- provenance validation;
- compatibility validation.

The required rule is:

generated structure
       ↓
canonical language validation
       ↓
semantic acceptance

not:

generated structure
       ↓
trusted compiler internals

---

31. Canonical IR Contract

The metaprogramming grammar produces zero IR.

It MUST NOT construct:

- classical IR;
- quantum IR;
- HDL IR;
- hardware IR;
- schedule IR;
- routing plans;
- QEC plans;
- ZQN state;
- HAL commands.

The canonical path is:

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

generated quantum source
       ↓
canonical frontend AST
       ↓
quantum semantic analysis
       ↓
quantum::ir

"quantum::ir" remains the canonical quantum IR boundary.

---

32. Quantum Integration

Metaprogramming may generate or inspect quantum source.

It MUST NOT create a metaprogramming-specific quantum IR.

A generated quantum construct must eventually follow:

generated source
    ↓
quantum syntax
    ↓
frontend AST
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

The metaprogramming layer must remain independent of physical realization.

---

33. Quantum Operation Extensibility

Metaprogramming MUST NOT depend on a fixed parser enumeration of quantum gates.

The architecture must support data-driven operation descriptions such as:

apply H ...
apply custom_gate ...
apply vendor.operation ...
apply operation(parameter) ...

The semantic operation model may carry:

- name;
- namespace;
- operands;
- parameters;
- results;
- attributes;
- modifiers;
- effects;
- capabilities;
- source metadata.

A new quantum operation MUST NOT require modifying metaprogramming grammar merely because the operation did not exist previously.

---

34. Quantum Resource Separation

Metaprogramming may construct source that expresses:

requires qubits >= n
requires capability("quantum.measurement")
requires capability("quantum.mid_circuit_measurement")
requires topology(...)

It MUST NOT encode universal physical limits such as:

MAX_QUBITS
MAX_QUBIT_REGISTER

It MUST NOT select physical qubits.

It MUST NOT allocate QPU resources.

It MUST NOT perform routing.

It MUST NOT perform QEC.

Those remain downstream responsibilities.

---

35. Classical Computing Integration

Metaprogramming may generate or transform canonical:

- integers;
- floating-point values;
- scalars;
- vectors;
- matrices;
- tensors;
- records;
- structures;
- functions;
- generic types;
- symbolic expressions;
- collections;
- data transformations.

Generated classical source MUST pass through ordinary semantic validation.

No metaprogramming-only classical type system is permitted.

---

36. HDL and Hardware Integration

Metaprogramming may generate:

- HDL modules;
- ports;
- signals;
- registers;
- state machines;
- pipelines;
- interfaces;
- protocols;
- memory declarations;
- hardware parameters;
- verification properties;
- hardware/software co-design structures.

It MUST generate hardware intent, not silently bind to physical hardware.

It MUST NOT hard-code:

wire [31:0]
RAM = 64GB
VRAM = 24GB
register = 32-bit

as universal language assumptions.

Parameterized source is permitted.

Physical realization remains downstream.

---

37. Distributed Computing Integration

Metaprogramming may generate:

- distributed declarations;
- tasks;
- actors;
- channels;
- services;
- messages;
- partitioning intent;
- replication intent;
- consistency policies;
- deployment intent.

It MUST NOT establish:

MAX_NODES
MAX_PROCESSES
MAX_CHANNELS

as language limits.

The generated source must express intent and allow downstream placement and scheduling to adapt to available resources.

---

38. AI/Data Integration

Metaprogramming may generate:

- model declarations;
- tensor operations;
- training structures;
- inference structures;
- data schemas;
- data pipelines;
- agents;
- symbolic computations;
- probabilistic structures.

It MUST NOT hard-code a specific AI framework into the core metaprogramming grammar.

Framework-specific behavior belongs to interoperability, dialect, library, or backend layers.

---

39. Networking and Security Integration

Metaprogramming may generate source involving:

- endpoints;
- protocols;
- services;
- policies;
- identities;
- authorization;
- cryptographic abstractions;
- secure computation.

Parsing such source MUST NOT perform network or security operations.

For example:

network.send(...)

is syntax.

It is not an instruction to send a packet during parsing.

Likewise:

file.read(...)

is syntax.

It does not grant filesystem access.

---

40. Compile-Time Effects

Compile-time computation is still computation.

A compile-time function MUST NOT automatically inherit arbitrary authority.

Compile-time effects must be explicitly classified.

Potential categories include:

pure
deterministic
nondeterministic
filesystem-dependent
network-dependent
environment-dependent
time-dependent
randomness-dependent
resource-dependent
target-dependent

The exact effect vocabulary belongs to the canonical effects specification.

The metaprogramming grammar must not invent a competing effect system.

---

41. Determinism

Parsing MUST be deterministic.

Parsing MUST depend only on:

- source text;
- selected language version;
- grammar;
- lexical configuration;
- explicitly selected dialect configuration.

Parsing MUST NOT depend on:

- CPU count;
- GPU availability;
- QPU availability;
- filesystem state;
- network state;
- wall-clock time;
- environment variables;
- random state;
- runtime state.

Compile-time evaluation may be nondeterministic only when the language explicitly permits it and semantic effect analysis records that dependency.

---

42. Reproducibility

Pure deterministic metaprograms SHOULD produce equivalent semantic results from equivalent inputs.

If a metaprogram depends on:

- time;
- randomness;
- environment;
- filesystem;
- network;
- target capabilities;
- resource availability;

that dependency MUST be explicit in the semantic/effect/capability model.

The compiler MUST NOT silently turn the build host into permanent program semantics.

---

43. POCO-REAF

Metaprogramming must preserve:

Program_Once
      ↓
Compile_Once
      ↓
Run_Everywhere
      ↓
Anywhere
      ↓
Forever

POCO-REAF does not mean that every target can execute every program regardless of resources.

Instead:

portable source meaning
        ↓
resource/capability requirements
        ↓
target analysis
        ↓
target realization

A program may require resources unavailable on a particular target.

The correct response is target/resource analysis, not changing the source-language meaning.

Possible downstream strategies include:

- compile for a larger target;
- distribute computation;
- use an accelerator;
- use logical quantum resources;
- decompose operations;
- route operations;
- schedule operations;
- use QEC;
- simulate where semantically permitted;
- defer execution;
- reject an infeasible target with a precise diagnostic.

Metaprogramming must not encode those target decisions into universal syntax.

---

44. No Hard-Coded Scalability Limits

The metaprogramming grammar MUST NOT define universal limits for:

macros
quotations
unquotations
generations
specializations
reflection queries
introspection queries
type-level operations
schema fields
generated declarations
generated expressions
generated statements
generated source
nested transformations
quantum operations
qubits
CPUs
cores
threads
GPUs
FPGAs
ASIC resources
QPUs
accelerators
nodes
processes
memory
storage
tensor rank
tensor dimensions
register width
network size
devices
timelines

The following names MUST NOT be used as language-level resource ceilings:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

Compiler resource policies are allowed.

Language-level hardware ceilings are not.

---

45. Resource Availability

The architecture is:

program semantics
        ↓
resource requirements
        ↓
capability requirements
        ↓
compiler admission
        ↓
available resources
        ↓
target realization

This means the same source can scale from tiny to very large execution environments.

A source program may express:

requires memory >= required_memory
requires qubits >= n
requires capability("tensor.compute")
requires capability("gpu.compute")
requires capability("quantum.measurement")
requires topology(...)

Those are semantic requirements.

They are not hardware allocations.

---

46. Requirement vs Capability vs Preference

The following must remain distinct.

Requirement

Must be satisfied.

requires capability("quantum.measurement")

Constraint

Restricts valid realizations.

requires topology(...)

Preference

Requests a preferred implementation.

prefer accelerator(...)

Hint

Provides optimization information without changing semantic validity.

hint ...

Implementation decision

Belongs downstream.

map ...
place ...
route ...
schedule ...

Metaprogramming MUST NOT collapse these categories.

---

47. Target Independence

Metaprogramming grammar MUST NOT contain universal alternatives such as:

cudaMetaExpression
rocmMetaExpression
nvidiaMetaExpression
amdMetaExpression
intelMetaExpression
qpuVendorMetaExpression
fpgaVendorMetaExpression

Vendor-specific constructs belong under:

- interoperability;
- dialects;
- libraries;
- backend contracts.

The universal metaprogramming grammar remains target-independent.

---

48. Security Boundary

Parsing metaprogramming syntax performs no execution.

The parser MUST NOT:

- execute metaprograms;
- execute generated code;
- execute host code;
- read arbitrary files;
- write files;
- access networks;
- access credentials;
- inspect arbitrary memory;
- inspect devices;
- contact QPUs;
- contact GPUs;
- contact FPGAs;
- spawn processes;
- mutate global compiler state.

The semantic/compiler subsystem may authorize explicitly defined compile-time capabilities.

Those capabilities must be visible and auditable.

---

49. No Embedded Rust Actions

All ANTLR metaprogramming grammar files MUST remain declarative parser grammars.

They MUST NOT contain embedded Rust execution actions.

The grammar must not depend on implementation-specific Rust behavior.

Compiler implementation targets:

Rust 2021
Rust 1.97
Rust 1.97.1

Production compiler code MUST remain safe Rust.

No "unsafe" is required.

No "unsafe" is permitted for metaprogramming implementation.

---

50. Lexer Integration

All metaprogramming-specific keywords/tokens must be defined exactly once in the canonical lexical system.

The checked repository currently has "SYNTHESIZE" in the Rust lexer.

The metaprogramming grammar also contains contracts referring to concepts such as:

QUOTE
UNQUOTE
REFLECT
K_SPECIALIZE
TYPELEVEL

These MUST NOT be assumed to exist merely because a ".g4" file mentions them.

The production process is:

language specification
        ↓
canonical lexical vocabulary
        ↓
ANTLR lexer
        ↓
Rust lexer
        ↓
parser grammar
        ↓
tests

A token is complete only when all required layers agree.

There must be no situation where:

metaprogramming/reflection.g4

defines a token that the canonical lexer does not know about while another lexer independently defines it.

---

51. Token Naming

Token names must follow the canonical repository vocabulary.

Do not create:

MetaQuoteToken
MetaUnquoteToken
MetaReflectToken
MetaSpecializeToken

merely to avoid resolving the existing lexical authority.

Likewise, do not define both:

QUOTE
KeywordQuote

for the same lexical meaning.

One spelling/lexical meaning must have one canonical token identity.

---

52. Ordinary Grammar Reuse

Metaprogramming MUST reuse canonical:

- identifiers;
- qualified names;
- paths;
- expressions;
- statements;
- declarations;
- types;
- patterns;
- attributes;
- generic parameters;
- argument lists;
- blocks.

For example, a quotation of an expression must use the canonical expression hierarchy.

A generated type must use the canonical type hierarchy.

A specialization argument must use the canonical argument/type/value rules.

No second language is permitted inside metaprogramming.

---

53. Grammar Dependency Direction

The dependency direction is:

Zamani.g4
    ↓
ZamaniParser.g4
    ↓
Metaprogramming
    ↓
metaprogramming leaf components
    ↓
shared core/type/expression/declaration rules

Metaprogramming grammar MUST NOT depend on:

- compiler backend implementation;
- runtime;
- HAL;
- physical hardware;
- QEC implementation;
- ZQN implementation.

---

54. No Circular Grammar Dependencies

Avoid cycles such as:

expression
 → metaprogramming
 → expression

unless the canonical expression architecture explicitly establishes the cycle and it has been proven deterministic.

For quotation, prefer canonical integration boundaries such as:

quoteExpressionCore
unquoteExpressionCore

rather than recursively embedding the entire metaprogramming grammar into itself.

---

55. Phase Model

Metaprogramming has explicit phases.

At minimum the implementation must distinguish:

runtime phase
compile-time phase
quotation phase
unquotation phase
generation phase
semantic-analysis phase

The exact formal phase model belongs to "grammar/specification/".

The grammar establishes syntactic boundaries.

Semantic analysis establishes whether a construct is legal in the current phase.

---

56. Nested Metaprogramming

Nested metaprogramming is permitted where the language specification allows it.

Examples include:

quote {
    let inner = quote {
        ...
    }
}

or generated structures containing further metaprogramming constructs.

There must be no artificial nesting limit in the grammar.

Compiler resource controls may prevent pathological compilation behavior.

Such controls are implementation policy, not language semantics.

---

57. Generated Source Must Be Canonical

When generation produces:

- a function;
- a type;
- an expression;
- a statement;
- a quantum operation;
- a quantum circuit fragment;
- an HDL module;
- a hardware interface;
- a distributed declaration;
- an AI structure;
- a data structure;

the result must remain ordinary Zamani source structure.

The compiler MUST NOT create a privileged "generated source" path that skips semantic validation.

---

58. Generated Quantum Source

Generated quantum source follows:

metaprogram
    ↓
quantum source
    ↓
canonical AST
    ↓
quantum semantic validation
    ↓
quantum::ir

The generator does not know:

- physical qubit numbering;
- device topology;
- calibration;
- physical gate set;
- routing;
- scheduling;
- QEC;
- ZQN;
- HAL implementation.

---

59. Generated HDL Source

Generated HDL source follows:

metaprogram
    ↓
HDL source
    ↓
canonical AST
    ↓
HDL/hardware semantic validation
    ↓
canonical hardware representation
    ↓
synthesis/lowering

Generation may parameterize widths, resources, timing intent, interfaces, and structures.

It must not impose universal machine limits.

---

60. Specialization and POCO-REAF

Specialization may adapt implementation to semantic information.

It MUST preserve the meaning of the program.

It must not silently turn:

specialize algorithm<...>

into:

use CPU X
use GPU Y
use QPU Z

unless that target binding is explicitly a downstream compilation decision.

Specialization may be:

- eager;
- deferred;
- partial;
- cached;
- target-aware downstream;
- resource-aware downstream.

Those decisions belong to the semantic/compiler system.

---

61. Type-Level Integration

Type-level computation may consume:

- canonical types;
- compile-time values;
- reflection results;
- generic information;
- type-level bindings.

It may produce:

- types;
- type-level values;
- constraints;
- metadata;
- specialization inputs;
- generation inputs.

It MUST return generated source through the ordinary metaprogramming pipeline.

---

62. Reflection and Type-Level Integration

Reflection remains owned by "reflection.g4".

Type-level metaprogramming may consume reflection results.

The relationship is:

reflection
    ↓
canonical semantic metadata
    ↓
type-level computation

not:

type-level grammar
    ↓
new reflection grammar

---

63. Generation and Type-Level Integration

The relationship is:

type-level computation
    ↓
compile-time result
    ↓
generation input
    ↓
generated Zamani source
    ↓
ordinary validation

Type-level computation does not directly construct backend IR.

---

64. Schema Integration

The relationship is:

data schema
    ↓
schema semantic model
    ↓
metaprogramming schema query/transformation
    ↓
derived/generated schema or source
    ↓
ordinary schema/source validation

The metaprogramming schema subsystem must not become a second database/query language.

---

65. Compile-Time Functions

Compile-time functions remain compatible with the existing function subsystem.

Where the repository uses "const fn" as the compile-time-function marker, metaprogramming must consume that canonical function model.

It must not redefine function declaration syntax.

The function subsystem owns:

- function declarations;
- parameters;
- generic declarations;
- return types;
- calling syntax.

Metaprogramming owns the compile-time transformation/evaluation boundary.

---

66. Resource and Capability Integration

Compile-time execution may have resource requirements.

Examples:

requires memory >= required_memory
requires capability("compile.time.evaluation")

Those declarations are interpreted semantically.

The grammar must not decide whether a machine satisfies them.

A compiler may reject a build because its actual compilation resources are insufficient.

That is not a language-level maximum.

---

67. Compiler Resource Limits

Compiler implementations may protect themselves against pathological metaprograms through:

- memory budgets;
- execution budgets;
- recursion detection;
- cancellation;
- timeouts;
- generated-source budgets;
- cache policies;
- incremental compilation;
- resource admission.

Such limits MUST be:

1. implementation policy;
2. diagnostically visible;
3. distinguishable from language rules;
4. configurable where appropriate;
5. absent from the grammar as universal semantic ceilings.

---

68. "Infinity" and Scalability

"Infinity" in POCO-REAF means that the language does not impose arbitrary finite machine-scale ceilings.

It does not claim that physical hardware, finite storage, compiler memory, or mathematical representations are literally infinite.

The intended invariant is:

language capacity
    is not artificially capped by today's machine capacity

Therefore:

tiny target
   ↓
same source semantics
   ↓
larger target
   ↓
same source semantics
   ↓
heterogeneous target
   ↓
same source semantics

subject to program semantics, compatibility, and actual resources.

---

69. Diagnostics

Metaprogramming diagnostics must distinguish:

lexical error
syntax error
phase error
name error
type error
effect error
capability error
resource error
quotation error
unquotation error
reflection error
generation error
specialization error
type-level error
schema-metaprogramming error
security error
compatibility error
compiler-resource exhaustion

A compiler-resource exhaustion error must not be reported as:

«Zamani language maximum exceeded»

unless the language specification genuinely defines such a semantic limit.

---

70. Source Spans

Every metaprogramming construct must preserve source locations.

At minimum:

- keyword;
- construct;
- target/name;
- arguments;
- quotation;
- unquotation;
- generation body;
- specialization target;
- specialization arguments;
- reflection selector;
- type-level construct;
- schema transformation.

Generated diagnostics should be able to identify both the generated location and its originating metaprogramming source.

---

71. Error Recovery

Grammar components must permit useful recovery without inventing semantic meaning.

Malformed constructs should produce diagnostics rather than:

- silently disappearing;
- silently becoming ordinary expressions;
- silently becoming runtime constructs;
- silently selecting another domain;
- silently executing partial metaprograms.

---

72. Compatibility

Every stable metaprogramming syntax change must be tracked through:

language version
feature status
old syntax
new syntax
migration
deprecation
AST impact
semantic impact
IR impact
tests

The compatibility subsystem owns version policy.

Metaprogramming must not invent a private versioning system.

---

73. Feature Lifecycle

Every metaprogramming feature follows:

proposal
   ↓
semantic design
   ↓
AST contract
   ↓
lexical contract
   ↓
grammar contract
   ↓
parser integration
   ↓
semantic implementation
   ↓
IR integration
   ↓
tests
   ↓
compatibility
   ↓
stable

Statuses include:

PROPOSED
EXPERIMENTAL
PLANNED
IMPLEMENTED
PARTIAL
STABLE
DEPRECATED
HISTORICAL
REJECTED

No feature becomes "STABLE" merely because its grammar parses.

---

74. Feature Manifest Integration

Where the repository adopts machine-readable feature manifests, every substantial metaprogramming feature should have one.

The manifest should identify:

id
name
status
language_version
grammar_files
lexer_tokens
AST_mapping
semantic_rules
IR_mapping
compiler_consumers
runtime_consumers
capabilities
resource_requirements
positive_tests
negative_tests
boundary_tests
scalability_tests
determinism_tests
compatibility
security
provenance
hard_coding_policy

This is the preferred mechanism for ensuring that one feature can be completed independently.

---

75. Independent Completion Principle

Every metaprogramming file must be completable without waiting for an unrelated later file to invent its missing contracts.

Therefore each file's contract must already identify:

Purpose
Owns
Does not own
Inputs
Outputs
Dependencies
Lexer dependencies
AST contract
Semantic contract
IR contract
Compiler consumers
Runtime consumers
Cross-domain consumers
Security
Diagnostics
Scalability
Determinism
Compatibility
Tests
Completion criteria

This prevents the repeated cycle:

implement file A
 ↓
implement file B
 ↓
discover file A must change
 ↓
re-edit A
 ↓
discover file C changes B
 ↓
re-edit A/B

The intended workflow is:

contract A
 ↓
complete A
 ↓
contract B already knows A's boundary
 ↓
complete B

---

76. File Completion Contract

A metaprogramming grammar file is considered complete only when all applicable items are known and resolved.

Required

- purpose;
- ownership;
- non-ownership;
- dependencies;
- lexer contract;
- grammar contract;
- AST contract;
- semantic contract;
- IR contract;
- downstream consumers;
- source-span behavior;
- diagnostics;
- security;
- determinism;
- scalability;
- compatibility;
- positive tests;
- negative tests;
- boundary tests;
- completion criteria.

Where applicable

- resource contract;
- capability contract;
- provenance contract;
- hygiene contract;
- phase contract;
- quantum integration;
- HDL integration;
- distributed integration;
- AI/data integration.

---

77. Testing Architecture

The subsystem must have tests covering:

tests/
├── metaprogramming/
├── quotation/
├── unquotation/
├── compile-time/
├── reflection/
├── introspection/
├── generation/
├── specialization/
├── type-level/
├── schemas/
├── capabilities/
├── macros/
├── negative/
├── boundary/
├── scalability/
├── determinism/
├── compatibility/
├── provenance/
├── security/
└── integration/

Existing test organization must be reused where already established rather than unnecessarily renamed.

---

78. Positive Tests

Positive tests must cover:

- minimal compile-time construct;
- compile-time value;
- compile-time function;
- quotation;
- unquotation;
- nested quotation;
- source generation;
- reflection;
- introspection;
- specialization;
- type-level computation;
- schema transformation;
- capability-aware metaprogramming;
- macro/metaprogramming interaction;
- generated classical source;
- generated quantum source;
- generated hybrid source;
- generated HDL source;
- generated hardware intent;
- generated distributed source;
- generated data/AI source.

---

79. Negative Tests

Negative tests must include:

- malformed quotation;
- malformed unquotation;
- unquote outside quotation;
- invalid quotation category;
- invalid generated source;
- generated source bypass attempt;
- invalid reflection subject;
- unauthorized introspection;
- invalid specialization;
- invalid type-level operation;
- invalid schema transformation;
- unauthorized compile-time effect;
- unauthorized capability;
- phase violation;
- duplicate ownership;
- invalid token;
- unsupported syntax;
- malformed nested constructs.

---

80. Boundary Tests

Boundary tests must cover:

- empty bodies where legal;
- single-element bodies;
- deeply nested quotation;
- nested unquotation;
- nested generation;
- empty argument lists;
- large argument lists;
- long identifiers;
- qualified names;
- generic arguments;
- nested type expressions;
- nested generated structures;
- Unicode identifiers where supported;
- large numeric values;
- mixed domain structures;
- compile-time/runtime boundaries.

Tests must distinguish language semantics from implementation resource exhaustion.

---

81. Scalability Tests

The test suite must progressively exercise:

- many declarations;
- many generated declarations;
- many expressions;
- many quotation elements;
- many unquotations;
- deeply nested transformations;
- many reflection projections;
- many specialization arguments;
- large type-level structures;
- large schemas;
- large generated programs;
- quantum source structures;
- HDL source structures;
- distributed source structures.

No test may establish an artificial maximum merely because the test runner has finite resources.

---

82. Determinism Tests

Equivalent source under equivalent language configuration should produce equivalent parser structures.

Tests must verify that parsing does not change because of:

- CPU count;
- GPU presence;
- QPU presence;
- FPGA presence;
- memory capacity;
- network availability;
- filesystem state;
- system clock;
- random state;
- environment variables.

Compile-time semantic determinism must be tested separately from parser determinism.

---

83. Reproducibility Tests

Pure compile-time computations should be reproducible.

External-state-dependent metaprograms must be explicitly classified.

The test suite should be able to distinguish:

deterministic compile-time computation

from:

environment-dependent compile-time computation

without silently treating the latter as deterministic.

---

84. Security Tests

Security tests must verify that parsing cannot:

- execute code;
- access files;
- access networks;
- access environment variables;
- access credentials;
- access hardware;
- execute QPU operations;
- execute GPU operations;
- spawn processes.

Tests must also verify that explicit compile-time capabilities cannot be silently escalated into unrelated capabilities.

---

85. Provenance Tests

Generated constructs must retain enough provenance to identify:

source
 ↓
metaprogram
 ↓
transformation
 ↓
generated construct

Tests must cover:

- diagnostics;
- nested generation;
- nested quotation;
- macro + quotation;
- macro + generation;
- specialization + generation;
- type-level + generation.

---

86. Quantum Metaprogramming Tests

Tests must cover:

generate quantum operation
generate quantum circuit
generate parameterized quantum source
generate custom quantum operation
reflect quantum declaration
reflect quantum type
specialize quantum abstraction
type-level quantum information

The tests must not enumerate a permanently closed set of quantum gates.

They must not establish a maximum qubit count.

They must verify that generated quantum source reaches:

quantum::ir

through ordinary semantic analysis.

---

87. HDL Metaprogramming Tests

Tests must cover:

- generated modules;
- generated ports;
- generated signals;
- generated registers;
- generated state machines;
- generated pipelines;
- generated interfaces;
- generated parameterized widths;
- generated timing intent;
- generated verification properties.

No universal bus-width or register-width ceiling may be encoded.

---

88. Distributed Metaprogramming Tests

Tests must cover generated:

- tasks;
- actors;
- services;
- channels;
- messages;
- partitions;
- replication;
- deployment intent;
- topology constraints.

No universal node count is permitted.

---

89. Macro + Metaprogramming Tests

Test:

macro → quotation
macro → generation
quotation → macro
generation → macro
macro → specialization
specialization → generation

The tests must verify:

- hygiene;
- provenance;
- deterministic expansion;
- semantic revalidation;
- no duplicate macro syntax.

---

90. AST Conformance

For every stable metaprogramming feature, the implementation must establish:

grammar rule
    ↓
parser context
    ↓
AST representation
    ↓
semantic representation

No grammar feature is complete if its AST destination is ambiguous.

---

91. IR Conformance

For every stable feature that contributes executable or compilable semantics:

AST
 ↓
semantic model
 ↓
canonical IR

must be documented.

Metaprogramming itself does not require a metaprogramming IR.

Where a compiler internally needs a transformation representation, it is an implementation detail and MUST NOT become a competing public language IR.

---

92. Quantum IR Conformance

Quantum-generated source must reach:

quantum::ir

There must not be:

metaprogramming::quantum::ir
quantum::meta::ir
quotation::quantum::ir
generation::quantum::ir

as competing quantum representations.

---

93. Compiler Integration

The compiler must be able to identify whether a metaprogramming construct is:

- compile-time only;
- source-transforming;
- value-producing;
- type-producing;
- metadata-producing;
- specialization-producing;
- generation-producing;
- runtime-visible.

The grammar does not decide the final lowering strategy.

---

94. Runtime Integration

Most metaprogramming constructs should disappear or become ordinary source semantics before runtime.

Runtime-visible metadata is permitted only where the language specification explicitly defines it.

Metaprogramming MUST NOT silently install a runtime metaprogramming engine.

---

95. Incremental Compilation

Metaprogramming should integrate with incremental compilation through semantic dependency tracking.

A change in:

metaprogram source

should invalidate dependent generated structures according to semantic dependency information.

The grammar itself does not implement incremental compilation.

---

96. Caching

Compile-time evaluation may be cacheable where semantic rules permit.

Cache identity must not silently omit relevant inputs.

Potential cache dependencies include:

- source;
- language version;
- compiler configuration;
- semantic inputs;
- explicit capabilities;
- explicit external inputs;
- dependency versions;
- target-independent compilation profile.

The grammar does not implement caching.

---

97. Target-Aware Metaprogramming

Target-aware metaprogramming must be explicitly classified.

The language may eventually permit a metaprogram to inspect abstract target capability information.

However:

target capability

is not equivalent to:

physical device identity

and:

capability query

is not automatically:

hardware allocation

This distinction is essential to POCO-REAF.

---

98. Hardware Adaptation

The preferred architecture is:

metaprogram
    ↓
portable source variation
    ↓
semantic validation
    ↓
resource/capability analysis
    ↓
compiler target adaptation

rather than:

metaprogram
    ↓
discover physical hardware
    ↓
rewrite source to today's device

The latter couples source semantics to deployment state and undermines long-term portability.

---

99. No Physical Resource Enumeration

Metaprogramming syntax MUST NOT require universal constructs such as:

physical_qubit(0)
gpu(0)
cpu_core(0)
fpga_region(0)
node(0)
memory_bank(0)
device(0)

unless such constructs are explicitly defined as target-specific interoperability/deployment facilities.

Those facilities must remain outside the portable metaprogramming core.

---

100. Dialect Integration

Dialect-specific metaprogramming belongs under:

grammar/dialects/

A dialect may extend metaprogramming only through an explicit dialect contract containing:

- name;
- version;
- owner;
- lexical additions;
- grammar additions;
- semantic additions;
- AST mapping;
- IR mapping;
- capability requirements;
- compatibility;
- feature status.

A dialect MUST NOT silently modify universal metaprogramming syntax.

---

101. Interoperability Integration

Foreign source generation may target:

- OpenQASM;
- QIR;
- HDL formats;
- WebAssembly;
- foreign-language interfaces;
- other explicitly supported formats.

However:

foreign format

is an interoperability target, not the canonical Zamani semantic model.

The preferred flow is:

Zamani semantics
    ↓
canonical IR
    ↓
interoperability lowering
    ↓
foreign representation

For generated Zamani source, the flow is:

generated Zamani source
    ↓
canonical parser/AST
    ↓
semantic analysis

---

102. No Vendor Lock-In

Metaprogramming must not require a specific:

- CPU vendor;
- GPU vendor;
- FPGA vendor;
- ASIC vendor;
- QPU vendor;
- AI framework;
- cloud provider;
- operating system;
- runtime implementation.

Vendor support belongs downstream through capabilities, interoperability, dialects, libraries, or backends.

---

103. Mathematical Metaprogramming

Metaprogramming may construct mathematical structures such as:

- expressions;
- matrices;
- tensors;
- symbolic terms;
- constraints;
- transformations.

It should reuse canonical mathematical expression/type semantics.

Do not turn every mathematical operation into a new metaprogramming keyword.

Prefer:

generic operation
+
canonical expression
+
typed library/intrinsic
+
semantic capability

where language-level syntax is unnecessary.

---

104. No Framework-Specific Meta Languages

Do not introduce:

CUDA metaprogramming
TensorFlow metaprogramming
PyTorch metaprogramming
Qiskit metaprogramming
vendor-specific HDL metaprogramming

as universal grammar branches.

Framework-specific source is an interoperability/dialect/backend concern.

---

105. Future-Proofing

The metaprogramming subsystem must support future computational domains without requiring a new metaprogramming language for each domain.

A future domain should normally reuse:

quotation
unquotation
reflection
generation
specialization
type-level computation
compile-time computation
capability queries
resource requirements
canonical AST
canonical semantic model
canonical IR

A new syntax form must justify why existing universal mechanisms are insufficient.

---

106. File-by-File Integration Matrix

File| Primary owner| Primary consumers
"metaprogramming.g4"| composition| "ZamaniParser.g4", domain parser
"compile-time-execution.g4"| compile-time execution syntax| semantic compile-time subsystem
"compile-time.g4"| compile-time integration| expressions/functions/compile/metaprogramming
"generation.g4"| Zamani source generation| compiler/metaprogramming
"code-generation.g4"| compiler code-generation intent| "grammar/compile/"
"quotation.g4"| quotation/unquotation core| expressions/metaprogramming/macros
"unquotation.g4"| unquotation integration| quotation/metaprogramming
"reflection.g4"| language reflection| semantic reflection
"introspection.g4"| broader authorized introspection| semantic introspection
"specialization.g4"| specialization requests| generic resolution/compiler
"type-level.g4"| type-level computation| type system/metaprogramming
"schemas.g4"| schema metaprogramming| data/schema semantic subsystem
"capabilities.g4"| metaprogramming capability syntax| effects/security/resources/compiler

No row may acquire ownership of another row's semantic implementation.

---

107. Cross-Repository Integration Matrix

Layer| Responsibility
"grammar/specification/"| Normative language meaning
"grammar/spec/"| Formal contracts
"grammar/DESIGN.md"| Architecture
"grammar/Zamani.g4"| Combined grammar root
"grammar/antlr/ZamaniParser.g4"| Parser composition
"grammar/lexer/"| Lexical authority
"grammar/metaprogramming/"| Metaprogramming syntax
"grammar/macros/"| Macro syntax
"grammar/types/"| Ordinary types
"grammar/expressions/"| Ordinary expressions
"grammar/functions/"| Function syntax
"grammar/data/"| Ordinary data schemas
"grammar/compile/"| Compilation intent
"grammar/hardware/"| Hardware semantics
"grammar/resources/"| Resource semantics
"grammar/quantum/"| Quantum source semantics
"src/lexer.rs"| Rust lexical implementation
"src/parser.rs"| Rust parser implementation
frontend AST| Canonical source structure
semantic analysis| Meaning and validation
"quantum::ir"| Quantum semantic IR
compiler| Lowering/optimization
routing| Physical/logical routing
scheduling| Resource/time scheduling
QEC| Quantum error correction
ZQN| Quantum noise/fault semantics
HAL| Hardware abstraction
runtime| Execution

---

108. Existing Repository Inconsistencies That This Contract Prevents

The production subsystem must specifically prevent the following classes of inconsistency.

108.1 Lexer/grammar token mismatch

A ".g4" file cannot declare that a token exists merely because its comments reference it.

Canonical lexer vocabulary must be synchronized.

108.2 Duplicate quotation ownership

"quotation.g4" and "unquotation.g4" cannot both independently define the same core rule.

108.3 Duplicate generation ownership

"generation.g4" and "code-generation.g4" must remain distinct.

108.4 Duplicate reflection ownership

"reflection.g4" and "introspection.g4" must remain distinct.

108.5 Duplicate compile-time ownership

"compile-time-execution.g4" and "compile-time.g4" must remain distinct.

108.6 Second type system

"type-level.g4" cannot redefine ordinary "typeExpression".

108.7 Second AST

Metaprogramming cannot introduce a competing frontend AST.

108.8 Second IR

Metaprogramming cannot introduce a competing universal or quantum IR.

108.9 Target leakage

Metaprogramming cannot turn source transformation into hardware allocation.

108.10 Resource hard-coding

Metaprogramming cannot turn implementation capacity into language semantics.

---

109. No Unnecessary Renaming

Existing filenames are retained.

Do not rename:

README.md
capabilities.g4
code-generation.g4
compile-time-execution.g4
compile-time.g4
generation.g4
introspection.g4
metaprogramming.g4
quotation.g4
reflection.g4
schemas.g4
specialization.g4
type-level.g4
unquotation.g4

A rename is justified only if:

1. the file is demonstrably obsolete;
2. the file is irreconcilably duplicated;
3. repository-wide references are migrated atomically;
4. specification, grammar, tooling, and tests are updated together.

Naming preference alone is insufficient.

---

110. No Parallel Hierarchy

Do not create a second metaprogramming tree merely to solve an ownership problem.

The existing:

grammar/metaprogramming/

directory is the subsystem.

New subdirectories should be introduced only if the number of files or ownership boundaries makes them necessary for maintainability.

Any new directory must have:

- clear ownership;
- README/contract;
- composition rule;
- dependency direction;
- AST contract;
- semantic contract;
- IR contract;
- tests;
- compatibility.

---

111. Production ANTLR Requirements

All metaprogramming parser grammars must:

- be valid ANTLR4 parser grammars;
- use canonical token vocabularies;
- avoid embedded implementation actions;
- avoid target-specific predicates;
- avoid filesystem/network behavior;
- avoid semantic execution;
- avoid machine-dependent parsing;
- avoid fixed resource ceilings;
- expose stable composition rules;
- avoid duplicate public rules.

---

112. Parser/ANTLR Compatibility

The ANTLR grammar is an architectural contract.

The existing Rust parser is also part of the implementation reality.

Therefore the repository must maintain explicit conformance between:

ANTLR grammar
    ↕
Rust lexer
    ↕
Rust parser
    ↕
frontend AST

A grammar-only change is incomplete when the production Rust frontend is expected to support the feature.

---

113. Rust Safety Contract

The metaprogramming implementation must compile under:

Rust 1.97
Rust 1.97.1
Rust 2021

Production code must use safe Rust.

Forbidden for metaprogramming implementation:

unsafe { ... }

and unnecessary "unsafe" APIs.

ANTLR grammar files themselves must contain no embedded Rust actions.

---

114. No Runtime Escape Hatch

Metaprogramming MUST NOT become a hidden mechanism for executing arbitrary compiler-host code.

The following must not be implicitly possible:

compile-time {
    execute arbitrary host program
}

without an explicit, specified, authorized capability/effect model.

A metaprogram is still subject to the security architecture.

---

115. No Compiler-State Mutation

A metaprogram must not silently mutate:

- compiler global state;
- language configuration;
- target configuration;
- environment;
- dependency graph;
- cache;
- generated artifacts.

If the language eventually permits controlled mutation, it must be explicitly specified as a capability/effect.

---

116. Dependency Purity

A pure metaprogram should depend only on explicitly supplied semantic inputs.

Implicit dependencies on:

- current time;
- machine architecture;
- environment variables;
- filesystem contents;
- network;
- installed software;
- hardware;

must be prohibited unless explicitly represented.

This protects reproducibility and POCO-REAF.

---

117. Generated Code Safety

Generated code is not trusted merely because a metaprogram generated it.

It must undergo the same semantic/security validation as handwritten code.

This is particularly important for generated:

- security policies;
- networking;
- hardware;
- quantum operations;
- resource requests;
- distributed execution;
- foreign interfaces.

---

118. Capability Escalation Prevention

The following is forbidden:

metaprogram has capability A
        ↓
generated source automatically receives capability B

unless the language specification explicitly defines that derivation.

Capabilities must be propagated according to semantic rules.

---

119. Resource Escalation Prevention

Likewise:

metaprogram requires small resource set
        ↓
generated source silently requires arbitrary resources

must not go undetected.

Generated source resource requirements must be analyzed normally.

---

120. Semantic Preservation

For transformations that claim to preserve semantics, the compiler must establish the appropriate semantic equivalence or preservation rule.

The grammar itself does not prove semantic equivalence.

This separation is essential:

grammar
    = syntax

semantic analysis
    = meaning

transformation engine
    = transformation

verification
    = preservation evidence where required

---

121. Error Handling

Every metaprogramming feature must have defined diagnostics for:

- invalid syntax;
- invalid phase;
- invalid type;
- invalid capability;
- invalid effect;
- invalid resource requirement;
- invalid quotation;
- invalid unquotation;
- invalid reflection;
- invalid introspection;
- invalid generation;
- invalid specialization;
- invalid type-level computation;
- invalid schema transformation;
- unauthorized operation;
- compiler-resource exhaustion.

Diagnostics must identify the originating source construct whenever possible.

---

122. Performance

Performance optimizations are permitted only if they preserve semantics.

Possible implementation strategies include:

- memoization;
- incremental evaluation;
- lazy evaluation;
- structural sharing;
- persistent representations;
- parallel compile-time evaluation where deterministic;
- caching;
- dependency-based invalidation.

These are implementation choices.

They must not leak into grammar semantics.

---

123. Parallel Compile-Time Evaluation

Compile-time computations may eventually be evaluated in parallel.

The language semantics must distinguish:

parallelizable pure computation

from:

ordered/effectful computation

The grammar does not decide whether the compiler uses one thread or many.

No:

MAX_THREADS

belongs in this subsystem.

---

124. Distributed Compilation

The same metaprogram may eventually be evaluated across distributed compilation resources.

Its source semantics must remain independent of:

- compiler node count;
- compiler cluster topology;
- physical machine addresses.

The compiler may distribute compilation work.

The grammar does not describe compiler cluster topology.

---

125. Incremental Generated Source

Generated source should be attributable to the semantic inputs that produced it.

This permits future incremental systems to determine:

what changed
    ↓
what generated output changed
    ↓
what downstream semantic/IR artifacts must be rebuilt

The grammar provides provenance boundaries; the compiler implements dependency tracking.

---

126. Build Reproducibility

Metaprogramming must support reproducible builds for deterministic inputs.

A build must not silently depend on:

current CPU
current GPU
current QPU
current filesystem
current environment
current clock
current random state

unless such dependencies are explicitly part of the build contract.

---

127. Security and Reproducibility Relationship

Security-sensitive external dependencies should also be provenance-visible.

If a metaprogram consumes an external resource, the compiler should be able to identify:

- what resource was accessed;
- under which capability;
- under which effect;
- during which phase;
- with what provenance;
- under which compilation context.

The grammar does not implement this auditing system.

---

128. Metaprogramming and Future Hardware

A future accelerator must not require a new universal metaprogramming language.

The expected path is:

existing metaprogramming
        ↓
existing source-generation mechanisms
        ↓
future domain grammar where necessary
        ↓
semantic model
        ↓
canonical IR
        ↓
future backend

This is a primary POCO-REAF requirement.

---

129. Metaprogramming and Future Quantum Technologies

Likewise, new quantum technologies must not require a permanently growing gate enumeration.

The metaprogramming layer should generate semantic quantum operations using the extensible quantum operation model.

New:

- gate families;
- logical operations;
- measurement mechanisms;
- control mechanisms;
- error-correction techniques;
- device capabilities;

must be handled downstream unless they genuinely require new source syntax.

---

130. Metaprogramming and Nano/Future Domains

Metaprogramming may generate future computational-domain constructs.

The grammar must not encode fixed:

- atomic counts;
- molecular counts;
- agent counts;
- material dimensions;
- device counts.

Those are semantic/resource properties.

---

131. Testing the "Program Once" Property

The repository should include conformance programs that describe the same semantic computation without embedding target-specific assumptions.

The same source should be analyzed against different capability/resource contexts.

Expected behavior:

same source
    ↓
same semantic meaning
    ↓
different target realization

where resource/capability conditions permit execution.

---

132. Testing Target Independence

A metaprogramming parser test must produce equivalent syntax structures regardless of whether the test host has:

- CPU only;
- CPU + GPU;
- CPU + FPGA;
- CPU + QPU;
- accelerator;
- distributed resources.

The parser must not inspect those resources.

---

133. Testing Resource Adaptation

Tests should distinguish:

program requires N resources

from:

compiler supports only N resources

The first can be language semantics.

The second is an implementation condition.

They must never be confused.

---

134. Completion Checklist: "metaprogramming.g4"

Complete only when:

- [ ] one metaprogramming composition root;
- [ ] all component rules resolve;
- [ ] no duplicate ownership;
- [ ] canonical lexer used;
- [ ] canonical expressions/types/names reused;
- [ ] AST contract complete;
- [ ] semantic contract complete;
- [ ] IR contract complete;
- [ ] quotation ownership resolved;
- [ ] unquotation ownership resolved;
- [ ] generation ownership resolved;
- [ ] reflection/introspection boundary resolved;
- [ ] compile-time boundary resolved;
- [ ] specialization boundary resolved;
- [ ] type-level boundary resolved;
- [ ] schema boundary resolved;
- [ ] capability boundary resolved;
- [ ] tests complete;
- [ ] no hard-coded resource limits;
- [ ] safe Rust integration complete.

---

135. Completion Checklist: "quotation.g4" / "unquotation.g4"

Complete only when:

- [ ] canonical quote syntax defined;
- [ ] canonical unquote syntax defined;
- [ ] exactly one unquotation core;
- [ ] canonical block/expression rules reused;
- [ ] phase semantics documented;
- [ ] hygiene boundary documented;
- [ ] provenance documented;
- [ ] AST mapping defined;
- [ ] semantic mapping defined;
- [ ] generated-source integration defined;
- [ ] macro integration defined;
- [ ] negative tests exist;
- [ ] nested tests exist;
- [ ] scalability tests exist;
- [ ] no artificial quotation limits exist.

---

136. Completion Checklist: "reflection.g4"

Complete only when:

- [ ] reflection subject is non-recursive;
- [ ] canonical names/paths/types reused;
- [ ] reflection projections defined;
- [ ] reflection selectors defined;
- [ ] semantic visibility rules defined;
- [ ] capability rules defined;
- [ ] external-state boundary defined;
- [ ] AST mapping defined;
- [ ] semantic mapping defined;
- [ ] diagnostics defined;
- [ ] security tests defined;
- [ ] determinism tests defined;
- [ ] no hardware-discovery semantics are hidden in syntax.

---

137. Completion Checklist: "generation.g4"

Complete only when:

- [ ] source-generation syntax defined;
- [ ] canonical "SYNTHESIZE" lexical integration resolved;
- [ ] generated source categories defined;
- [ ] quotation integration defined;
- [ ] unquotation integration defined;
- [ ] provenance defined;
- [ ] hygiene defined;
- [ ] AST mapping defined;
- [ ] semantic validation defined;
- [ ] quantum integration defined;
- [ ] HDL integration defined;
- [ ] classical integration defined;
- [ ] distributed integration defined;
- [ ] security defined;
- [ ] no backend code generation is embedded;
- [ ] no physical hardware binding is embedded.

---

138. Completion Checklist: "specialization.g4"

Complete only when:

- [ ] canonical specialization token exists;
- [ ] specialization target defined;
- [ ] type arguments defined;
- [ ] value arguments defined;
- [ ] named arguments defined;
- [ ] policies defined;
- [ ] AST mapping defined;
- [ ] semantic mapping defined;
- [ ] generic integration defined;
- [ ] type integration defined;
- [ ] quantum integration defined;
- [ ] HDL integration defined;
- [ ] resource separation defined;
- [ ] target independence defined;
- [ ] no fixed specialization limits exist.

---

139. Completion Checklist: "type-level.g4"

Complete only when:

- [ ] ordinary type syntax is reused;
- [ ] type-level computation boundary is explicit;
- [ ] bindings defined;
- [ ] transformations defined;
- [ ] predicates defined;
- [ ] normalization defined;
- [ ] iteration defined;
- [ ] specialization integration defined;
- [ ] reflection integration defined;
- [ ] generation integration defined;
- [ ] AST mapping defined;
- [ ] semantic mapping defined;
- [ ] no second type system exists;
- [ ] no fixed type-level limits exist.

---

140. Completion Checklist: "schemas.g4"

Complete only when:

- [ ] ordinary schemas remain owned by "data/schemas.g4";
- [ ] meta-schema declarations defined;
- [ ] projections defined;
- [ ] transformations defined;
- [ ] composition defined;
- [ ] derivation defined;
- [ ] validation defined;
- [ ] compatibility defined;
- [ ] provenance defined;
- [ ] generation integration defined;
- [ ] AST mapping defined;
- [ ] semantic mapping defined;
- [ ] no database implementation is embedded.

---

141. Completion Checklist: "introspection.g4"

Complete only when:

- [ ] reflection remains separately owned;
- [ ] introspection subjects defined;
- [ ] authorized metadata defined;
- [ ] external-state access is explicit;
- [ ] capability boundaries exist;
- [ ] security rules exist;
- [ ] deterministic/static introspection is distinguished from dynamic introspection;
- [ ] AST mapping exists;
- [ ] semantic mapping exists;
- [ ] no hardware discovery is implicit.

---

142. Completion Checklist: "compile-time-execution.g4"

Complete only when:

- [ ] compile-time boundary is explicit;
- [ ] execution is not performed by parsing;
- [ ] compile-time inputs are defined;
- [ ] compile-time outputs are defined;
- [ ] effects are checked;
- [ ] capabilities are checked;
- [ ] resources are checked;
- [ ] security is checked;
- [ ] determinism is classified;
- [ ] generated output is revalidated;
- [ ] no runtime escape hatch exists.

---

143. Completion Checklist: "capabilities.g4"

Complete only when:

- [ ] capability syntax is defined;
- [ ] canonical capability vocabulary is reused;
- [ ] capabilities are distinct from requirements;
- [ ] capabilities are distinct from hardware identities;
- [ ] security semantics are defined downstream;
- [ ] capability escalation is prevented;
- [ ] no physical-device limits are encoded.

---

144. Repository-Level Acceptance

The metaprogramming subsystem is production-ready only when:

specification
       ↓
lexical vocabulary
       ↓
ANTLR grammar
       ↓
canonical parser
       ↓
Rust lexer/parser
       ↓
frontend AST
       ↓
semantic analysis
       ↓
metaprogram evaluation/transformation
       ↓
canonical semantic model
       ↓
canonical IR
       ↓
compiler
       ↓
runtime/backend

is traceable for every "STABLE" feature.

---

145. Mandatory Repository Audits

Before declaring the subsystem stable, run audits for:

Lexical audit

Find:

- duplicate tokens;
- missing tokens;
- inconsistent keyword names;
- token aliases with conflicting meanings.

Grammar audit

Find:

- duplicate rules;
- unreachable rules;
- undefined rules;
- import cycles;
- precedence cycles;
- hidden ambiguity.

AST audit

Find:

- syntax without AST representation;
- duplicate AST representations;
- domain-specific metaprogramming AST leakage.

Semantic audit

Find:

- syntax without semantic validation;
- phase violations;
- effect bypass;
- capability bypass;
- resource bypass.

IR audit

Find:

- syntax without IR mapping where required;
- duplicate IRs;
- direct backend construction.

Scalability audit

Search for forbidden universal limits.

Security audit

Search for:

- host execution;
- filesystem access;
- network access;
- environment access;
- hardware access;
- unsafe Rust.

Compatibility audit

Find syntax that lacks language-version status.

---

146. Forbidden Universal Constructs

The following categories are forbidden as universal metaprogramming grammar semantics:

MAX_*
fixed CPU counts
fixed GPU counts
fixed FPGA counts
fixed QPU counts
fixed qubit counts
fixed node counts
fixed memory capacities
fixed tensor dimensions
fixed tensor ranks
fixed register widths
fixed device counts
fixed timeline counts
fixed compiler-thread counts
physical device identifiers
physical qubit identifiers
vendor-specific universal grammar branches
implicit host execution
implicit filesystem access
implicit network access
implicit hardware discovery
implicit runtime execution

---

147. Allowed Semantic Scaling

The following concepts are explicitly compatible with POCO-REAF:

requires qubits >= n
requires memory >= required_memory
requires capability("tensor.compute")
requires capability("gpu.compute")
requires capability("quantum.measurement")
requires topology(...)
requires capability(...)
prefer accelerator(...)
hint ...

The compiler may then determine how to realize the intent.

---

148. Meaning of a Numeric Literal

A number appearing in metaprogramming is not automatically a compiler limit.

For example:

1024

may be:

- a program value;
- a type-level value;
- an array dimension;
- a specialization parameter;
- a resource requirement.

It MUST NOT become a hidden universal compiler ceiling.

---

149. No Artificial Machine Model

Metaprogramming syntax must not assume:

register = 32-bit
address = 64-bit
RAM = fixed size
GPU = fixed count
CPU = fixed count
QPU = fixed count
FPGA = fixed capacity
node = fixed capacity

The machine model is discovered and interpreted downstream.

---

150. Semantic Portability Rule

The strongest invariant is:

«A metaprogram must describe a transformation of program meaning, not a transformation into one particular machine.»

Therefore:

metaprogram
    ↓
portable source semantics

is valid.

Whereas:

metaprogram
    ↓
today's physical device

is not a universal language mechanism.

---

151. Long-Term Architecture

The permanent architecture is:

                       Zamani Source
                            |
                            v
                    Canonical Lexer
                            |
                            v
                   Canonical Parser
                            |
                            v
                   Domain-Neutral AST
                            |
              +-------------+-------------+
              |                           |
              v                           v
       Ordinary semantics          Metaprogramming
                                      |
                   +------------------+------------------+
                   |                  |                  |
                   v                  v                  v
                quote              reflect           generate
                   |                  |                  |
                   +------------------+------------------+
                                      |
                                   transform
                                      |
                                      v
                            Validated Zamani Structure
                                      |
                                      v
                              Semantic Analysis
                                      |
                                      v
                         Canonical Semantic Model
                                      |
                     +----------------+----------------+
                     |                |                |
                     v                v                v
                 Classical       quantum::ir       HDL/Hardware
                     |                |                |
                     +----------------+----------------+
                                      |
                                      v
                                  Optimization
                                      |
                          +-----------+-----------+
                          |           |           |
                          v           v           v
                       Routing   Scheduling   Resilience
                                                  |
                                                 QEC
                                                  |
                                                 ZQN
                                                  |
                                                 HAL
                                                  |
                                                  v
                                           Target Realization

---

152. Final Non-Negotiable Invariants

1. Zamani remains one language.

2. Metaprogramming is not a second language.

3. "grammar/specification/" remains normative.

4. "grammar/DESIGN.md" remains the architectural authority.

5. "grammar/Zamani.g4" remains the canonical combined grammar root.

6. "grammar/antlr/ZamaniParser.g4" remains the canonical parser composition root.

7. The metaprogramming directory remains subordinate to those roots.

8. Every metaprogramming facility has exactly one syntax owner.

9. Ordinary expressions are not redefined.

10. Ordinary types are not redefined.

11. Ordinary names and paths are not redefined.

12. Macros remain owned by "grammar/macros/".

13. Quotation has one canonical core.

14. Unquotation has one canonical core.

15. "unquotation.g4" must not create a competing unquotation language.

16. Generation and compiler code generation remain separate.

17. Reflection and introspection remain separate.

18. Type-level metaprogramming does not create a second type system.

19. Schema metaprogramming does not create a second schema language.

20. Metaprogramming grammar produces no IR.

21. Metaprogramming grammar produces no machine code.

22. Generated source re-enters canonical semantic validation.

23. Quantum source ultimately reaches "quantum::ir".

24. No metaprogramming-specific quantum IR exists.

25. No physical qubit selection occurs in the grammar.

26. No hardware target is selected by universal metaprogramming syntax.

27. No fixed resource ceiling is encoded.

28. No "MAX_QUBITS" or equivalent universal limit exists.

29. No fixed CPU/GPU/FPGA/QPU/node/device limit exists.

30. Resource requirements are distinct from capabilities.

31. Requirements are distinct from preferences.

32. Preferences are distinct from implementation decisions.

33. Parsing never executes metaprograms.

34. Parsing never accesses host resources.

35. Compile-time effects require explicit semantic authorization.

36. Generated source cannot bypass semantic validation.

37. Generated source cannot bypass security validation.

38. Provenance is preserved.

39. Hygiene is preserved where applicable.

40. Parsing is deterministic.

41. Pure compile-time computation is reproducible.

42. Rust 1.97/1.97.1 is supported.

43. Rust 2021 is supported.

44. Production implementation uses safe Rust only.

45. No "unsafe" is required or permitted.

46. Every stable feature has a complete AST contract.

47. Every stable executable feature has an IR contract.

48. Every stable feature has diagnostics.

49. Every stable feature has appropriate positive tests.

50. Every stable feature has appropriate negative tests.

51. Every stable feature has boundary tests.

52. Every stable feature has scalability tests.

53. Every stable feature has determinism tests.

54. Every stable feature has compatibility status.

55. Every substantial feature has a complete integration contract.

56. Existing filenames are not unnecessarily renamed.

57. New files are added only when they establish a genuine missing ownership boundary.

58. New directories are added only when maintainability requires them.

59. No parallel metaprogramming hierarchy is created.

60. A new computational domain must reuse the universal metaprogramming model wherever possible.

---

153. Definition of "Done"

A metaprogramming file is DONE only when a developer can answer all of these questions without waiting for another unrelated file to be designed:

What syntax does this file own?
What syntax does it not own?
Which tokens does it consume?
Where are those tokens defined?
Which canonical rules does it consume?
Which rules consume this file?
What AST structure receives its output?
What semantic rules validate it?
What phase does it operate in?
What effects may it require?
What capabilities may it require?
What resources may it require?
How is provenance preserved?
How is hygiene preserved?
How is it validated?
How does generated source re-enter the language?
How does it reach canonical IR?
How does quantum output reach quantum::ir?
Which compiler components consume it?
Which runtime components consume it?
What are its security boundaries?
What are its diagnostics?
What are its positive tests?
What are its negative tests?
What are its boundary tests?
What are its scalability tests?
What are its determinism tests?
What is its compatibility policy?
What forbidden hard-coded limits were audited?
Does it use unsafe Rust?

If any required answer is unknown, the file is not independently complete.

---

154. Final Production Rule

The metaprogramming subsystem exists to let Zamani programs reason about and transform Zamani programs, not to make the compiler's current machine architecture part of the language.

The permanent boundary is therefore:

Zamani source
     ↓
metaprogramming
     ↓
validated Zamani source structure
     ↓
canonical AST
     ↓
semantic analysis
     ↓
canonical semantic model
     ↓
canonical IR
     ↓
optimization / lowering
     ↓
routing / scheduling / resilience / QEC / ZQN
     ↓
HAL
     ↓
actual target

This is the metaprogramming foundation required for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

from the smallest supported computation to arbitrarily large heterogeneous systems, constrained by the actual semantics of the program, declared requirements, implementation resources, target capabilities, and physical resources — never by arbitrary machine-size constants embedded in the language grammar.