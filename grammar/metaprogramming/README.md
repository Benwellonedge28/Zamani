Zamani Metaprogramming Grammar

Path: "grammar/metaprogramming/"
Language: Zamani Universal Programming Language
Subsystem: Metaprogramming, compile-time computation, quotation, unquotation, syntax-tree construction, reflection, introspection, generation, specialization, type-level computation, schema transformation, and controlled compile-time program transformation
Grammar technology: ANTLR4 parser-grammar composition
Compiler baseline: Rust 1.97 or later, Rust 2021 edition
Safety requirement: Safe Rust only; production implementation MUST NOT require or use "unsafe" Rust
Portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)
Status: Production architecture and subsystem orchestration contract

---

1. Purpose

"grammar/metaprogramming/" is Zamani's metaprogramming subsystem.

This directory provides the language-level syntax necessary for programs and compile-time tooling to:

- compute values during compilation;
- inspect permitted language structure;
- construct language structure;
- quote Zamani syntax;
- unquote or splice computed syntax;
- transform syntax trees;
- generate Zamani source structures;
- request specialization;
- perform type-level computation;
- transform schemas;
- perform compile-time validation;
- derive declarations and other language structures;
- inspect semantic metadata;
- express controlled compile-time transformations.

This directory is not a second programming language.

It is not a second compiler.

It is not a second AST.

It is not a second IR.

It is not a backend.

It is not a runtime.

It is not a hardware description database.

It is not a device-selection mechanism.

It is not a macro-expansion implementation.

The subsystem exists to extend the existing Zamani language while preserving the repository-wide architecture.

---

2. The Orchestrator Principle

This README is the orchestrator for every file under "grammar/metaprogramming/".

It establishes:

1. permanent ownership;
2. dependency direction;
3. composition order;
4. public integration boundaries;
5. AST expectations;
6. semantic expectations;
7. effect/capability/resource expectations;
8. provenance requirements;
9. IR boundaries;
10. compiler integration;
11. macro integration;
12. dialect integration;
13. security requirements;
14. determinism requirements;
15. scalability requirements;
16. compatibility requirements;
17. testing requirements;
18. definition-of-done criteria.

Individual ".g4" files own their detailed syntax.

This README owns the relationship between those files.

The governing rule is:

«A metaprogramming grammar file must be independently complete according to its contract and must expose stable interfaces so that changes in another subsystem do not require rewriting its internal syntax merely to restore undocumented integration.»

---

3. Canonical Architecture

The complete Zamani pipeline is:

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
structural validation
    |
    +--> name resolution
    +--> type analysis
    +--> effect analysis
    +--> capability analysis
    +--> resource analysis
    +--> contract analysis
    +--> policy analysis
    +--> provenance
    |
    v
metaprogramming analysis
    |
    +--> compile-time evaluation
    +--> reflection
    +--> introspection
    +--> quotation
    +--> unquotation
    +--> syntax-tree transformation
    +--> generation
    +--> specialization
    +--> type-level computation
    +--> schema transformation
    |
    v
validated transformed Zamani structure
    |
    v
canonical semantic model
    |
    +--> classical computation
    +--> quantum computation
    +--> hybrid computation
    +--> HDL/hardware
    +--> AI/ML
    +--> data
    +--> distributed computation
    +--> networking
    +--> accelerators
    +--> future domains
    |
    v
canonical IR
    |
    +--> classical representation
    +--> quantum::ir
    +--> HDL/hardware representation
    +--> other domain IRs where explicitly defined
    |
    v
optimization
    |
    v
lowering
    |
    v
routing / scheduling / resilience
    |
    v
ZQN / HAL / target realization
    |
    v
runtime

Metaprogramming MUST NOT bypass this architecture.

The critical rule is:

metaprogramming
      |
      v
validated Zamani structure
      |
      v
normal semantic pipeline

not:

metaprogramming
      |
      v
backend

---

4. Authority Model

The subsystem follows the repository-wide authority hierarchy.

grammar/specification/
        |
        v
normative language specification
        |
        v
grammar/spec/
        |
        v
formal subsystem contracts
        |
        v
grammar/DESIGN.md
        |
        v
grammar/Zamani.g4
        |
        v
grammar/antlr/ZamaniParser.g4
        |
        v
modular grammar components
        |
        v
Rust lexer/parser/frontend
        |
        v
domain-neutral AST
        |
        v
semantic analysis
        |
        v
canonical semantic representation
        |
        v
canonical IR
        |
        v
compiler
        |
        v
runtime / HAL / targets

The responsibilities are distinct.

Artifact| Responsibility
"grammar/specification/"| Normative language specification
"grammar/spec/"| Formal subsystem contracts and conformance rules
"grammar/DESIGN.md"| Repository grammar architecture
"grammar/Zamani.g4"| Complete ANTLR root
"grammar/antlr/ZamaniLexer.g4"| Canonical lexer composition
"grammar/antlr/ZamaniParser.g4"| Canonical parser composition
"grammar/lexer/"| Lexical contracts
"grammar/macros/"| Macro syntax and macro-specific grammar
"grammar/metaprogramming/"| Metaprogramming grammar subsystem
"grammar/expressions/metaprogramming.g4"| Expression-level metaprogramming integration
Rust frontend| Executable parser/frontend implementation
AST| Domain-neutral source representation
semantic subsystem| Meaning and validity
canonical IR| Target-independent semantic representation
"quantum::ir"| Canonical quantum IR boundary
compiler| Optimization, lowering and realization
runtime/HAL| Execution and target interaction

No file in this directory may silently override a higher-level normative specification.

---

5. What This Directory Owns

"grammar/metaprogramming/" owns the syntax and grammar composition for:

- compile-time execution requests;
- compile-time contexts;
- compile-time expressions;
- compile-time declarations;
- compile-time statements;
- quotation;
- unquotation;
- syntax-tree manipulation;
- reflection;
- introspection;
- source generation;
- code-generation intent where specifically assigned to this subsystem;
- specialization requests;
- type-level metaprogramming;
- schema metaprogramming;
- metaprogramming capabilities;
- metaprogramming-specific integration boundaries.

It owns the composition of those facilities.

It does not own their runtime/compiler implementations.

---

6. What This Directory Does Not Own

This directory does not own:

- lexical token spelling;
- identifier syntax;
- qualified names;
- ordinary expressions;
- ordinary statements;
- ordinary declarations;
- ordinary types;
- generic syntax outside metaprogramming-specific boundaries;
- macro expansion;
- macro hygiene;
- name resolution;
- type checking;
- effect implementation;
- capability authorization;
- resource allocation;
- target discovery;
- hardware discovery;
- device selection;
- physical qubit allocation;
- routing;
- scheduling;
- optimization implementation;
- QEC;
- ZQN;
- HAL;
- runtime execution;
- backend implementation;
- compiler implementation;
- AST implementation;
- canonical IR implementation.

The grammar only declares the language surface.

---

7. Permanent File Inventory

The current subsystem contains:

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
├── syntax-tree.g4
├── type-level.g4
└── unquotation.g4

Existing filenames are retained.

No rename is required merely to achieve production readiness.

The files have the following permanent responsibilities.

---

8. "metaprogramming.g4"

Role

"metaprogramming.g4" is the ANTLR composition/orchestration root for this directory.

It is the most important grammar file in this directory after this README.

Owns

It owns only:

- metaprogramming subsystem composition;
- stable metaprogramming declaration boundary;
- stable metaprogramming statement boundary;
- stable metaprogramming facility boundary;
- composition of the leaf facilities;
- integration wrappers needed by the canonical parser.

Does not own

It must not own detailed syntax for:

- compile-time execution;
- quotation;
- unquotation;
- reflection;
- introspection;
- generation;
- code generation;
- specialization;
- type-level computation;
- schemas;
- syntax trees;
- capabilities.

Those belong to their respective files.

Required public boundaries

The composition root should expose stable subsystem-level categories such as:

metaprogrammingDeclaration
metaprogrammingStatement
metaprogrammingFacilityExpression
metaprogrammingElement

The exact rule names must remain synchronized with the actual leaf grammar exports.

No phantom rules may remain.

No rule may be referenced solely because an earlier design document expected it to exist.

Critical requirement

Every imported public rule must have exactly one owner.

The orchestrator must not duplicate leaf rules.

---

9. "compile-time-execution.g4"

Role

Owns the explicit source syntax for requesting compile-time execution.

Owns

- compile-time execution boundary;
- compile-time execution blocks;
- compile-time sequencing;
- compile-time evaluation requests;
- compile-time result boundaries;
- compile-time execution context syntax.

Does not own

- evaluator implementation;
- constant-folding implementation;
- optimizer implementation;
- macro expansion;
- runtime execution;
- hardware access;
- reflection implementation;
- generation implementation.

Semantic boundary

A compile-time execution construct follows:

parse
  |
  v
AST
  |
  v
name resolution
  |
  v
type validation
  |
  v
effect validation
  |
  v
capability validation
  |
  v
resource validation
  |
  v
security/policy validation
  |
  v
compile-time eligibility
  |
  v
authorized evaluation
  |
  v
result validation

Parsing never executes the construct.

---

10. "compile-time.g4"

Role

Provides the general compile-time integration boundary.

It connects compile-time constructs to other grammar facilities without becoming another implementation of them.

Owns

- compile-time context wrappers;
- compile-time declaration integration;
- compile-time expression integration;
- compile-time statement integration;
- phase-boundary syntax where required.

Does not own

- ordinary expressions;
- compile-time evaluator implementation;
- reflection;
- generation;
- specialization;
- macro expansion.

Integration

compile-time.g4
       |
       +--> compile-time-execution.g4
       +--> generation.g4
       +--> reflection.g4
       +--> specialization.g4
       +--> quotation.g4

The concrete facility remains owned by its leaf grammar.

---

11. "quotation.g4"

Role

Owns the canonical quotation syntax.

Quotation represents Zamani source structure as a compile-time semantic value.

Owns

- quotation boundaries;
- quote expressions;
- quoted structure;
- quotation categories;
- quotation nesting;
- quotation-specific syntax.

Canonical concept

The repository's quotation model uses:

quote { ... }

where specified.

Quoted contents remain Zamani syntax.

Quotation must not create a second expression language.

Does not own

- macro expansion;
- macro hygiene;
- AST implementation;
- reflection;
- generation implementation;
- specialization;
- compile-time evaluator;
- IR.

---

12. "unquotation.g4"

Role

Provides the explicit unquotation/splicing integration surface.

Ownership rule

The quotation subsystem remains the canonical owner of quotation/unquotation core semantics where the existing grammar architecture establishes those rules.

"unquotation.g4" must not silently create competing core rules.

Owns

- unquotation integration wrappers;
- splice integration boundaries;
- composition surfaces needed by the canonical parser.

Does not own

- quotation implementation;
- compile-time evaluation;
- macro expansion;
- AST construction.

Semantic requirement

An unquotation construct is valid only within an appropriate quotation/splicing context.

The parser recognizes structure.

Semantic analysis determines phase legality.

---

13. "syntax-tree.g4"

Role

Owns grammar-level integration for syntax-tree representations.

It provides the language surface required to manipulate or represent canonical Zamani syntax structures.

Owns

- syntax-tree references;
- syntax-tree construction boundaries;
- syntax-tree node access where specified;
- syntax-tree transformation boundaries;
- syntax-tree metadata surfaces.

Does not own

- macro token-tree implementation;
- AST implementation;
- parser implementation;
- source-map implementation;
- compiler IR.

Critical invariant

A syntax tree represented by metaprogramming remains compatible with the canonical frontend representation.

It must not become an unrelated second AST hierarchy.

---

14. "generation.g4"

Role

Owns source-level Zamani generation.

The established source-generation vocabulary must be reused rather than creating an unnecessary competing keyword.

Where the repository's canonical lexical contract uses "SYNTHESIZE", generation syntax must consume that canonical token.

Owns

- source-generation requests;
- generated declarations;
- generated statements;
- generated expressions;
- generated types where supported;
- generated items;
- generation metadata;
- generation context;
- generation provenance boundaries.

Does not own

- machine-code generation;
- object-file generation;
- executable generation;
- GPU instruction generation;
- FPGA bitstream generation;
- ASIC implementation;
- QPU scheduling;
- physical routing.

Mandatory pipeline

generation request
       |
       v
generated Zamani structure
       |
       v
canonical AST
       |
       v
semantic validation
       |
       v
canonical semantic model
       |
       v
canonical IR

Generated code must re-enter normal language validation.

---

15. "code-generation.g4"

Role

Defines metaprogramming-side code-generation intent/context where required by the language architecture.

It must remain distinct from source generation.

Mandatory distinction

generation.g4
    =
source-level Zamani structure generation

while:

code-generation.g4
    =
compiler/code-generation intent or integration

depending on the normative specification.

Does not own

- machine-code implementation;
- backend implementation;
- linker implementation;
- target instruction selection;
- source-generation implementation.

If a construct creates Zamani source structure, it belongs to "generation.g4".

If it expresses compiler artifact generation intent, it belongs to the compilation subsystem boundary.

No third category may be invented.

---

16. "reflection.g4"

Role

Owns language-defined reflection.

Reflection operates on information represented by Zamani's canonical semantic model.

May inspect

Where permitted by the specification:

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
- schemas;
- semantic metadata.

Does not own

Reflection must not automatically mean:

- hardware discovery;
- filesystem inspection;
- network discovery;
- credential inspection;
- environment inspection;
- physical qubit inspection;
- arbitrary host introspection;
- device enumeration;
- runtime memory inspection.

Those operations belong to explicitly authorized downstream mechanisms.

No second type system

Reflection must expose canonical language metadata.

It must not invent another type representation with different semantics.

---

17. "introspection.g4"

Role

Owns introspection facilities that are explicitly broader or semantically distinct from ordinary language reflection.

The distinction is:

reflection
    =
language-defined semantic structure

while:

introspection
    =
explicitly authorized broader metadata

Critical rule

Introspection never automatically grants environmental or hardware access.

A source-level capability description is not equivalent to querying an actual device.

For example:

capability metadata

is distinct from:

physical device discovery

The latter belongs downstream.

---

18. "specialization.g4"

Role

Owns source-level specialization intent.

Specialization allows a generic or parameterized computation to be specialized under known semantic information.

Owns

- specialization requests;
- specialization parameters;
- specialization constraints;
- specialization metadata;
- specialization context;
- specialization declarations/expressions where specified.

Does not own

- optimizer implementation;
- machine-specific instruction selection;
- backend implementation;
- target device selection;
- physical resource allocation.

POCO-REAF requirement

Specialization must preserve program meaning.

It may adapt an implementation to:

- available capabilities;
- available resources;
- known compile-time information;
- domain-specific semantics;
- target-independent optimization opportunities.

It must not force source programs to identify a particular machine.

---

19. "type-level.g4"

Role

Owns type-level metaprogramming syntax.

Owns

- type-level expressions;
- type-level declarations;
- compile-time type computation;
- type-level transformations;
- type-level derivation;
- type-level constraints where specified.

Does not own

- the ordinary type grammar;
- type checking implementation;
- ordinary generics;
- runtime values;
- backend representation.

Integration

type-level syntax
       |
       v
canonical type representation
       |
       v
type analysis
       |
       v
semantic model

It must not create a second type system.

---

20. "schemas.g4"

Role

Owns metaprogramming operations over schemas.

This is distinct from ordinary data-schema syntax.

Owns

- schema transformation;
- schema derivation;
- schema generation;
- schema validation requests;
- schema compatibility requests;
- schema metaprogramming expressions/declarations/statements.

Does not own

- ordinary JSON syntax;
- ordinary XML syntax;
- SQL grammar;
- external data formats;
- runtime database engines;
- ordinary data structures.

Those remain under their existing data/interoperability owners.

---

21. "capabilities.g4"

Role

Defines metaprogramming-specific capability requirements and restrictions.

Owns

- metaprogramming capability declarations;
- metaprogramming capability requirements;
- capability restrictions;
- capability-related metaprogramming expressions.

Does not own

- global capability semantics;
- authorization implementation;
- security policy implementation;
- hardware discovery.

Global capability semantics remain under the repository's capability/resource architecture.

Critical distinction

Parsing:

requires capability(...)

does not grant the capability.

The semantic/security system decides whether it is permitted.

---

22. Canonical Expression Integration

The repository already has:

grammar/expressions/metaprogramming.g4

This file owns the expression-level integration boundary.

That distinction is mandatory.

grammar/metaprogramming/metaprogramming.g4
    =
metaprogramming subsystem orchestration

grammar/expressions/metaprogramming.g4
    =
ordinary-expression integration

Therefore "grammar/metaprogramming/metaprogramming.g4" must not attempt to become the owner of the canonical "metaprogrammingExpression" rule if that rule is owned by "grammar/expressions/metaprogramming.g4".

The expression layer should connect metaprogramming expressions into the ordinary expression hierarchy.

The metaprogramming subsystem remains the owner of the underlying facilities.

---

23. Canonical Parser Integration

The canonical parser is:

grammar/antlr/ZamaniParser.g4

It is the composition point through which the metaprogramming subsystem becomes reachable by the complete language.

The integration direction is:

grammar/Zamani.g4
        |
        v
grammar/antlr/ZamaniParser.g4
        |
        v
metaprogramming composition
        |
        +--> compile-time
        +--> quotation
        +--> unquotation
        +--> generation
        +--> reflection
        +--> introspection
        +--> specialization
        +--> type-level
        +--> schemas
        +--> syntax-tree
        +--> capabilities

The parser must not contain duplicate implementations of these facilities.

A grammar file existing on disk does not make its rules reachable.

The canonical parser build must explicitly import or otherwise compose the required grammar.

---

24. Lexer Integration

Metaprogramming grammar files contain no lexer rules.

The lexical authority remains the canonical Zamani lexer/token system.

Relevant authorities include:

grammar/lexer/
grammar/antlr/ZamaniLexer.g4
grammar/lexer/tokens.g4

A metaprogramming grammar may consume an existing token only when that token is part of the canonical lexical contract.

No metaprogramming file may invent duplicate token spellings.

Before introducing a new reserved word:

1. determine whether an existing identifier or token is sufficient;
2. determine whether the feature actually requires reserved syntax;
3. update the normative specification;
4. update the canonical token registry;
5. update the canonical lexer;
6. update parser composition;
7. update Rust lexer conformance;
8. update tests.

The grammar directory is not permitted to create isolated lexical vocabulary.

---

25. AST Contract

ANTLR parser contexts are not the Zamani AST.

The frontend AST remains domain-neutral.

A metaprogramming construct must be represented using the repository's canonical AST model or the repository's established metaprogramming AST nodes where those nodes are formally part of the domain-neutral frontend architecture.

At minimum, semantic preservation must cover:

- source span;
- source ordering;
- nesting;
- construct kind;
- names;
- paths;
- arguments;
- child structures;
- attributes;
- phase information where semantically required;
- provenance;
- generated-source relationships where applicable.

The AST must not:

- execute metaprograms;
- perform capability authorization;
- allocate hardware;
- choose targets;
- create physical quantum mappings;
- construct backend instructions.

---

26. Semantic Contract

Grammar acceptance means:

«syntactically valid metaprogramming structure.»

It does not mean:

- semantically valid;
- type-correct;
- effect-safe;
- capability-authorized;
- resource-feasible;
- policy-compliant;
- deterministic;
- reproducible;
- executable.

Semantic processing must therefore validate:

syntax
  |
  v
AST
  |
  v
name resolution
  |
  v
type validation
  |
  v
effect validation
  |
  v
capability validation
  |
  v
resource validation
  |
  v
contract validation
  |
  v
policy/security validation
  |
  v
provenance
  |
  v
authorized metaprogram execution/transformation

---

27. Effect Integration

Metaprogramming operations may have effects.

Examples include:

- compile-time computation;
- source generation;
- reflection;
- introspection;
- file access, if explicitly supported;
- network access, if explicitly supported;
- native calls, if explicitly supported;
- foreign calls;
- code generation;
- code transformation;
- randomness;
- environment access.

An effect is not automatically permitted merely because a grammar construct exists.

The effect system remains authoritative.

Metaprogramming must therefore integrate with:

grammar/effects/

without duplicating effect semantics.

---

28. Capability Integration

Capability requirements are separate from effects.

For example:

effect = network

is not the same semantic concept as:

capability = network.access

Metaprogramming constructs may require both.

The semantic pipeline must therefore preserve:

operation
    |
    +--> effects
    |
    +--> capabilities
    |
    +--> resources
    |
    +--> policies

---

29. Resource Integration

Metaprogramming must not define fixed universal resource limits.

No grammar-level limits may be introduced for:

- generated nodes;
- quotation depth;
- specialization count;
- compile-time execution;
- generated source;
- reflection requests;
- schema size;
- type-level complexity;
- macro expansion;
- memory;
- processors;
- threads;
- accelerators;
- quantum resources;
- devices.

Compiler implementations may have configurable resource budgets.

Those are operational controls.

They are not Zamani language ceilings.

Resource exhaustion must produce a defined diagnostic or controlled compilation outcome.

---

30. Policy Integration

Metaprogramming can be powerful enough to affect compilation itself.

Therefore sensitive operations must be subject to policy.

Policies may govern:

- compile-time execution;
- reflection;
- introspection;
- source generation;
- file access;
- network access;
- native calls;
- foreign calls;
- adaptation;
- code generation;
- resource consumption;
- reproducibility;
- trust.

Policy semantics belong to the repository's policy/security architecture.

This directory only expresses the relevant source boundary.

---

31. Provenance

Metaprogramming must preserve provenance.

Generated or transformed structures must be traceable to their origin.

At minimum, provenance should be capable of relating:

original source
    |
    v
metaprogram
    |
    v
transformation
    |
    v
generated structure
    |
    v
validated semantic structure
    |
    v
IR

Provenance supports:

- diagnostics;
- source maps;
- debugging;
- IDE tooling;
- reproducibility;
- deterministic compilation;
- security auditing;
- generated-code inspection;
- compiler explanation;
- verification.

Metaprogramming grammar does not implement provenance storage.

It must preserve the information required by the semantic/compiler layer.

---

32. Determinism

Parsing is deterministic.

Parsing MUST NOT depend on:

- wall-clock time;
- randomness;
- hardware;
- filesystem state;
- network state;
- environment variables;
- device availability;
- runtime state.

Compile-time execution may have nondeterministic inputs only where explicitly specified and authorized.

If nondeterminism can affect generated program meaning, the language must provide an explicit semantic contract for it.

Reproducible builds must remain possible.

---

33. Security Boundary

Parsing a metaprogram does not execute it.

Recognition of:

reflection
generation
compile-time execution
introspection
quotation
specialization

must never itself grant permission to:

- read arbitrary files;
- access credentials;
- access secrets;
- inspect arbitrary environment variables;
- execute subprocesses;
- contact arbitrary networks;
- access devices;
- invoke hardware;
- invoke a QPU;
- invoke a GPU;
- invoke an FPGA;
- mutate compiler configuration;
- bypass security policy.

Any such capability must be explicitly authorized downstream.

---

34. Macro Integration

Metaprogramming and macros are related but are not the same subsystem.

The macro subsystem remains under:

grammar/macros/

The boundary is:

grammar/macros/
    |
    +--> macro declarations
    +--> macro invocations
    +--> macro expansion
    +--> hygiene
    |
    v
metaprogramming infrastructure

Metaprogramming must not duplicate:

- macro declaration syntax;
- macro invocation syntax;
- macro hygiene;
- macro expansion semantics.

Macro expansion must preserve:

- source spans;
- provenance;
- binding relationships;
- hygiene;
- type information;
- effects;
- capabilities;
- resource requirements.

After controlled expansion, resulting Zamani structure returns to the normal frontend/semantic pipeline.

---

35. Quotation and Macro Boundary

Quotation is a language metaprogramming mechanism.

Macros are a language transformation mechanism with their own expansion/hygiene architecture.

They may exchange syntax representations, but they must not silently become interchangeable.

The boundary is:

quotation
    =
explicit source-structure value

while:

macro expansion
    =
controlled language transformation

Neither is permitted to bypass semantic validation.

---

36. Generated Classical Programs

Generated classical constructs must return to the ordinary Zamani language pipeline.

generated source
    |
    v
canonical AST
    |
    v
classical semantic analysis
    |
    v
canonical representation
    |
    v
classical IR

No metaprogramming-specific classical IR is permitted.

---

37. Generated Quantum Programs

Metaprogramming may construct or transform quantum source structures.

It must not enumerate or hard-code a universal quantum gate set.

It must not introduce:

- fixed physical qubits;
- fixed device IDs;
- vendor hardware layouts;
- routing decisions;
- calibration data;
- scheduling decisions.

The path is:

metaprogram
    |
    v
generated quantum source
    |
    v
canonical AST
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
decomposition
    |
    v
routing
    |
    v
scheduling
    |
    v
QEC / resilience
    |
    v
ZQN
    |
    v
HAL
    |
    v
target

There is no metaprogramming-specific quantum IR.

---

38. Generated HDL and Hardware Structures

Metaprogramming may generate HDL or hardware intent.

Generated HDL must enter the canonical HDL/hardware semantic pipeline.

It must not encode universal fixed:

- bus widths;
- register counts;
- device counts;
- FPGA capacities;
- ASIC resources;
- clock frequencies;
- memory capacities;
- topology sizes.

Concrete values may exist when they are actual program semantics.

They must not be mistaken for universal language limits.

---

39. AI, Data, Distributed and Other Domains

Metaprogramming is domain-neutral.

It may generate or transform structures belonging to:

- AI;
- machine learning;
- tensors;
- data processing;
- graphs;
- distributed computing;
- networking;
- cryptography;
- scientific computing;
- accelerators;
- embedded systems;
- classical computing;
- quantum computing;
- HDL;
- hardware/software co-design;
- future domains.

The metaprogramming subsystem does not need a new grammar branch for every application domain.

The domain-specific semantic subsystem owns the generated construct after normal frontend validation.

---

40. POCO-REAF Contract

Metaprogramming must preserve:

Program_Once
      |
      v
Compile_Once
      |
      v
Run_Everywhere
      |
      v
Run_Anywhere
      |
      v
Forever

This does not mean that every physical machine can execute every program.

It means that changing machine scale or target realization must not require rewriting source merely because the machine changed.

Metaprogramming therefore must not hard-code:

- CPU identity;
- GPU identity;
- FPGA identity;
- ASIC identity;
- QPU identity;
- physical qubit identity;
- node counts;
- fixed memory capacities;
- fixed register widths;
- vendor topology;
- vendor-specific execution layout.

The program describes computation and requirements.

Downstream compilation determines realization.

---

41. Scalability Contract

The grammar has no artificial universal capacity ceiling.

The language must remain structurally capable of representing computations whose size is determined by available resources.

This applies to:

- syntax trees;
- generated source;
- declarations;
- expressions;
- types;
- specializations;
- schemas;
- compile-time values;
- quotation;
- unquotation;
- reflection;
- generated quantum operations;
- generated HDL;
- generated distributed structures;
- generated AI/data structures.

Operational limits may exist in implementations.

They must be:

- configurable;
- documented;
- diagnosable;
- independent of source-language semantics;
- independent of physical hardware enumeration.

---

42. No Hard-Coded Universe

The following concepts must never become universal grammar constants:

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

The same rule applies to metaprogramming-specific ceilings.

Do not replace one hard-coded ceiling with another such as:

MAX_META_DEPTH
MAX_GENERATED_NODES
MAX_SPECIALIZATIONS
MAX_QUOTE_DEPTH

if it is intended as a language restriction.

Implementation resource budgets are different and are permitted.

---

43. No Application Keyword Explosion

Metaprogramming must remain universal.

A new application area must not automatically require a new keyword.

For example, the language does not need dedicated metaprogramming keywords for every:

- computer-vision algorithm;
- robotics system;
- blockchain operation;
- payment system;
- sentiment model;
- scientific algorithm;
- AI architecture;
- networking protocol;
- hardware vendor;
- quantum gate;
- accelerator.

Such functionality belongs in:

- libraries;
- semantic capabilities;
- dialects;
- schemas;
- metadata;
- interoperability;
- policies;
- external services.

The grammar describes stable computational concepts.

---

44. Open-World Extension Model

New capabilities should normally be introduced through:

semantic metadata
capabilities
resources
schemas
dialects
libraries
versioned specifications

rather than by modifying universal grammar rules.

The desired model is:

new capability
    |
    v
capability/schema/semantic registration
    |
    v
existing generic syntax
    |
    v
semantic validation
    |
    v
domain implementation

not:

new capability
    |
    v
new keyword
    |
    v
new grammar branch

unless the feature genuinely requires new language syntax.

---

45. Reflection Safety

Reflection must operate only over explicitly permitted semantic information.

Reflection must not be interpreted as unrestricted host introspection.

The following are distinct:

reflect(type)

reflect(declaration)

reflect(capability)

and:

inspect physical device

inspect host memory

inspect credentials

The latter require separate downstream authorization.

---

46. Compile-Time Execution Safety

Compile-time execution is powerful but must remain bounded by the language security architecture.

A compile-time program is still a program.

Therefore its execution must participate in:

- type checking;
- effect checking;
- capability checking;
- resource checking;
- policy checking;
- provenance;
- reproducibility;
- diagnostics.

Compile-time execution must not become an implicit privileged escape hatch.

---

47. Generated Source Must Be Revalidated

Every generated source structure must pass through normal language validation.

Required path:

generate
   |
   v
generated structure
   |
   v
canonical parser/AST representation
   |
   v
name resolution
   |
   v
type validation
   |
   v
effect validation
   |
   v
capability validation
   |
   v
resource validation
   |
   v
contract validation
   |
   v
policy/security validation
   |
   v
canonical semantic model

Generated code is not trusted merely because the compiler generated it.

---

48. No Parser Actions

ANTLR grammar files must not execute arbitrary Rust or host-language code during parsing.

The metaprogramming grammar must remain declarative.

No grammar action may:

- execute a metaprogram;
- access the filesystem;
- access a network;
- inspect hardware;
- invoke a compiler;
- invoke a subprocess;
- modify global state;
- access secrets;
- perform target selection.

---

49. Rust Integration

The production implementation baseline is:

Rust 1.97 or later
Rust 2021 edition
safe Rust only
no unsafe Rust

The grammar itself is language-neutral, but its implementation contracts must remain compatible with safe Rust.

Production metaprogramming infrastructure must not require:

unsafe

or equivalent unsafe FFI assumptions.

Where foreign interoperability is necessary, the repository's safe abstraction boundary must preserve the no-unsafe requirement of the production metaprogramming implementation.

---

50. Canonical Rust Frontend

The grammar must remain compatible with the repository's actual Rust frontend.

The frontend responsibilities remain separate:

lexer
parser
AST
semantic analysis
compiler
runtime

The metaprogramming grammar does not replace those implementations.

Where ANTLR and Rust frontend implementations coexist, they must represent the same normative language.

Neither implementation may silently define a second language.

---

51. IR Contract

Metaprogramming grammar creates no IR.

It must not create:

MetaprogrammingIR
MetaIR
QuantumMetaIR
HardwareMetaIR

unless such an IR is explicitly introduced by a future normative architecture outside the grammar layer.

Generated or transformed semantic structures proceed to the existing canonical representation.

Quantum constructs proceed to:

quantum::ir

Classical constructs proceed through the canonical classical representation.

HDL/hardware constructs proceed through the canonical HDL/hardware representation.

---

52. Compiler Integration

The compiler consumes validated semantic structures.

The metaprogramming subsystem may provide:

- compile-time values;
- generated structures;
- specialization requests;
- type-level results;
- reflection results;
- schema transformations;
- provenance;
- transformation metadata.

The compiler decides:

- optimization;
- lowering;
- specialization implementation;
- target adaptation;
- scheduling;
- routing;
- artifact generation;
- backend realization.

The grammar does not make those decisions.

---

53. Target Independence

Metaprogramming syntax must remain independent of:

- processor vendor;
- GPU vendor;
- FPGA vendor;
- ASIC family;
- QPU vendor;
- device identifier;
- physical topology;
- physical qubit layout;
- cluster size;
- cloud provider.

A metaprogram may express target-independent requirements.

Target realization remains downstream.

---

54. Dialect Integration

Dialect-specific syntax must remain outside the universal metaprogramming core unless explicitly promoted into the normative language.

A dialect may provide:

- additional metadata;
- domain-specific syntax;
- external format integration;
- vendor-specific facilities;
- experimental facilities.

The universal metaprogramming core must remain stable.

Dialect-specific generated structures must still enter the canonical semantic pipeline.

---

55. Compatibility

Metaprogramming must support long-lived source compatibility.

Compatibility must account for:

- language version;
- grammar version;
- AST contract;
- semantic contract;
- metaprogramming feature version;
- macro compatibility;
- dialect version;
- IR compatibility;
- generated-source compatibility;
- provenance compatibility.

Breaking changes must be explicit and versioned.

A historical or experimental grammar description does not automatically become legal syntax.

---

56. Diagnostics

Diagnostics must distinguish at least:

Syntax errors

Examples:

- malformed quotation;
- malformed unquotation;
- malformed specialization;
- malformed compile-time construct;
- malformed generation request.

Semantic errors

Examples:

- unquotation outside quotation;
- invalid reflection subject;
- invalid specialization;
- invalid type-level operation;
- invalid generated structure.

Security errors

Examples:

- unauthorized compile-time effect;
- unauthorized introspection;
- unauthorized file access;
- unauthorized native call.

Capability errors

Examples:

- missing required capability;
- forbidden capability.

Resource errors

Examples:

- insufficient compilation resource;
- compile-time budget exhaustion;
- generated-structure resource exhaustion.

Compatibility errors

Examples:

- unsupported feature version;
- incompatible generated structure;
- incompatible dialect.

Diagnostics must retain source and generated-source provenance.

---

57. Testing Architecture

Tests must exist at multiple levels.

grammar/metaprogramming/
        |
        +--> lexical conformance
        +--> parser conformance
        +--> AST conformance
        +--> semantic conformance
        +--> compile-time execution
        +--> quotation
        +--> unquotation
        +--> syntax-tree
        +--> generation
        +--> reflection
        +--> introspection
        +--> specialization
        +--> type-level
        +--> schemas
        +--> capabilities
        +--> macro integration
        +--> quantum integration
        +--> HDL integration
        +--> classical integration
        +--> distributed integration
        +--> security
        +--> determinism
        +--> scalability
        +--> compatibility

---

58. Positive Tests

Every facility must have valid examples.

At minimum:

- compile-time execution;
- compile-time expressions;
- quotation;
- nested quotation;
- unquotation;
- syntax-tree construction;
- source generation;
- reflection;
- introspection;
- specialization;
- type-level computation;
- schema transformation;
- capability requirements;
- generated classical computation;
- generated quantum computation;
- generated HDL;
- cross-domain generated structures.

---

59. Negative Tests

Every facility must have invalid examples.

At minimum:

- malformed syntax;
- invalid phase;
- unquotation outside quotation;
- invalid generated syntax;
- invalid reflection;
- unauthorized introspection;
- invalid specialization;
- invalid type-level operation;
- missing capability;
- forbidden effect;
- invalid resource requirement;
- invalid policy;
- invalid provenance;
- generated structure rejected by semantic analysis.

---

60. Boundary Tests

The subsystem must test boundaries between:

metaprogramming ↔ expressions
metaprogramming ↔ statements
metaprogramming ↔ declarations
metaprogramming ↔ types
metaprogramming ↔ macros
metaprogramming ↔ effects
metaprogramming ↔ capabilities
metaprogramming ↔ resources
metaprogramming ↔ policies
metaprogramming ↔ provenance
metaprogramming ↔ classical
metaprogramming ↔ quantum
metaprogramming ↔ HDL
metaprogramming ↔ hardware
metaprogramming ↔ AI
metaprogramming ↔ data
metaprogramming ↔ distributed
metaprogramming ↔ interoperability

---

61. Quantum Integration Tests

At least one test must demonstrate:

metaprogram
    |
    v
generated quantum structure
    |
    v
AST
    |
    v
semantic quantum representation
    |
    v
quantum::ir

The test must demonstrate that metaprogramming does not create a competing quantum representation.

No test should require a fixed universal quantum gate list.

---

62. HDL Integration Tests

At least one test must demonstrate:

metaprogram
    |
    v
generated HDL/hardware intent
    |
    v
canonical AST
    |
    v
HDL semantic model
    |
    v
HDL/hardware compilation

The test must not depend on a universal fixed bus/register/device capacity.

---

63. POCO-REAF Integration Test

At least one conformance test must demonstrate that the same source-level metaprogram can produce semantically equivalent output for different target profiles.

The test must vary target realization rather than source semantics.

For example:

same source
    |
    +--> small target
    |
    +--> larger target
    |
    +--> accelerator target
    |
    +--> quantum-capable target
    |
    +--> heterogeneous target

The grammar must remain unchanged.

---

64. Scalability Tests

Scalability tests must validate that the implementation does not depend on fixed universal capacities.

Test progressively larger:

- generated syntax trees;
- compile-time computations;
- quotation structures;
- specialization sets;
- schema transformations;
- reflection results;
- generated declarations;
- generated expressions;
- generated quantum structures;
- generated HDL structures.

The purpose is not to promise physically infinite computation.

The purpose is to verify that the language has no artificial fixed ceiling where the implementation can scale according to available resources.

---

65. Determinism Tests

Repeated compilation of identical inputs under identical declared configuration must produce deterministic:

- parse structure;
- semantic results;
- generated structures;
- diagnostics;
- provenance relationships;
- specialization decisions where required by the specification.

Randomness, time, environment, hardware and external state must not silently influence language meaning.

---

66. Security Tests

Security tests must verify that parsing cannot:

- execute code;
- access files;
- access networks;
- access secrets;
- access hardware;
- invoke devices;
- grant capabilities;
- bypass policy;
- bypass resource checks.

Separate tests must verify that explicitly authorized compile-time capabilities are correctly enforced.

---

67. File Completion Contract

Every ".g4" file under this directory is considered complete only when its contract is known.

Each file must have, either directly in its documentation/header or through an authoritative specification, all of the following:

Purpose
Owns
Does Not Own
Inputs
Outputs
Dependencies
Dependency Direction
Lexer Contract
Grammar Contract
AST Contract
Semantic Contract
Type Contract
Effect Contract
Capability Contract
Resource Contract
Contract/Validation Contract
Policy Contract
Provenance Contract
IR Contract
Compiler Integration
Runtime Integration
Quantum Integration
HDL Integration
Dialect Integration
Compatibility
Diagnostics
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Determinism Tests
Security Tests
Hard-Coding Audit
Completion Criteria

This allows an individual file to be completed independently.

---

68. Dependency Contract

Every public grammar component must be able to answer:

DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
IR_OWNER:
SPEC_OWNER:
TEST_OWNER:

For example:

File:
grammar/metaprogramming/quotation.g4

DEPENDS_ON:
    canonical lexer vocabulary
    canonical expression/type/name structures
    compile-time semantic contract

EXPORTS:
    quotation grammar boundary

CONSUMED_BY:
    metaprogramming.g4
    expressions/metaprogramming.g4
    canonical parser

AST_OWNER:
    domain-neutral frontend AST

SEMANTIC_OWNER:
    semantic metaprogramming subsystem

IR_OWNER:
    canonical semantic/IR pipeline

SPEC_OWNER:
    normative metaprogramming specification

TEST_OWNER:
    metaprogramming quotation conformance tests

The exact names must follow the actual repository implementation.

---

69. Dependency Direction

Dependencies must flow toward canonical infrastructure.

Preferred direction:

leaf metaprogramming grammar
        |
        v
metaprogramming composition
        |
        v
canonical parser
        |
        v
AST
        |
        v
semantic analysis

Do not create circular dependencies such as:

canonical expression
    |
    v
metaprogramming
    |
    v
canonical expression

Where circularity would occur, use a narrow integration boundary.

---

70. No Phantom Rules

A grammar file is not complete if it references a rule that is merely expected to exist.

Before declaring the subsystem complete:

- every imported grammar must exist;
- every imported grammar name must match its actual grammar name;
- every referenced public rule must exist;
- every public rule must have one owner;
- every cross-file dependency must be reachable through canonical composition;
- no stale rule names may remain;
- no compatibility alias may silently become a second language construct.

This is especially important for the metaprogramming composition root.

---

71. No Duplicate Public Rule Ownership

The same public rule must not be independently defined by:

grammar/metaprogramming/
grammar/expressions/
grammar/macros/
grammar/compile/

when the repository has assigned it to one owner.

For example, the expression integration layer may own:

metaprogrammingExpression

while the metaprogramming subsystem owns the underlying facility rules.

This prevents recursive or circular grammar composition.

---

72. Compile-Time and Constant Evaluation

Compile-time execution must remain distinguishable from ordinary compiler optimization.

For example:

compile-time execution

is an explicit language semantic mechanism.

Whereas:

constant folding

is a compiler optimization.

The two must not be conflated.

An optimizer may perform constant evaluation without turning optimization into a source-language metaprogram.

---

73. Specialization and Optimization

Specialization may be requested explicitly.

Optimization may occur independently.

The architecture is:

source intent
      |
      v
specialization request
      |
      v
semantic/compiler analysis
      |
      v
specialized semantic representation
      |
      v
optimization

The grammar does not prescribe the optimizer algorithm.

---

74. Reflection and Introspection

The two facilities must remain distinct.

Reflection
    =
canonical language semantic information

Introspection
    =
explicitly authorized broader information

Neither facility may become an unrestricted system-information mechanism.

---

75. Generation and Code Generation

The subsystem must preserve the following distinction permanently:

SOURCE GENERATION
    |
    v
Zamani source structures

versus:

COMPILER CODE GENERATION
    |
    v
target artifacts

Source generation must return through the frontend.

Compiler code generation remains downstream.

---

76. Type-Level Computation and Runtime Computation

Type-level computation is compile-time semantic computation.

It must not silently become runtime computation.

Runtime values cannot be assumed to exist during type-level evaluation unless the language specification explicitly provides such semantics.

This boundary is necessary for:

- determinism;
- type soundness;
- reproducibility;
- compiler safety;
- portability.

---

77. Schema Metaprogramming

Schema metaprogramming must remain generic.

It must be possible to operate on schema descriptions without making the core grammar dependent on:

- one database;
- one serialization format;
- one cloud provider;
- one data engine;
- one external framework.

SQL, JSON, XML and other formats remain interoperability concerns.

---

78. Interoperability

Metaprogramming may generate interoperability declarations.

Those declarations must proceed through:

generated source
    |
    v
canonical AST
    |
    v
interoperability semantic validation
    |
    v
ABI/FFI boundary
    |
    v
compiler/backend

Metaprogramming must not directly invoke foreign functions from parser actions.

---

79. Provenance of Generated Code

Generated code must retain enough provenance to answer:

- what generated it;
- where the generating construct came from;
- what source structure was transformed;
- what specialization was applied;
- what quotation produced it;
- what macro or metaprogram contributed it;
- what semantic transformation occurred.

This is essential for production diagnostics and reproducibility.

---

80. Error Recovery

Parser error recovery must not execute metaprogramming operations.

Malformed constructs must remain inert syntax.

A parser must be able to report an error without:

- executing code;
- generating files;
- contacting a device;
- changing compiler state;
- invoking external systems.

---

81. Resource Exhaustion

Resource exhaustion must be explicit.

Examples include:

- compile-time evaluation budget exhausted;
- generated structure too large for current compilation resources;
- compiler memory unavailable;
- compilation time budget exceeded;
- expansion resource exhausted.

Such failures are implementation/resource diagnostics.

They must not be transformed into language-level universal ceilings.

---

82. Forward Compatibility

A future target, accelerator, language extension or computational model must not require rewriting this subsystem merely because the new target exists.

The extension should normally enter through:

semantic capability
resource description
dialect
schema
library
compiler backend
target profile

The universal metaprogramming grammar remains stable.

---

83. Future Computational Domains

The subsystem must be capable of generating or transforming structures for future domains without requiring a new metaprogramming language.

The principle is:

new domain
    |
    v
domain semantic specification
    |
    v
domain grammar
    |
    v
canonical AST
    |
    v
metaprogramming can construct it
    |
    v
domain semantic pipeline

This is how Zamani scales beyond today's computing models.

---

84. Production Hard-Coding Audit

The following are prohibited in metaprogramming grammar:

fixed CPU counts
fixed GPU counts
fixed FPGA counts
fixed QPU counts
fixed qubit counts
fixed node counts
fixed memory capacities
fixed register widths
fixed tensor ranks
fixed device counts
fixed topology sizes
fixed generated-node limits
fixed specialization limits
fixed quotation-depth limits
fixed compile-time limits
fixed reflection limits
fixed schema limits
fixed source-size language ceilings

A literal number appearing in a program is not automatically a hard-coded language limit.

The audit concerns implementation assumptions that turn finite examples into universal language ceilings.

---

85. Repository Integration Matrix

Component| Metaprogramming relationship
"grammar/Zamani.g4"| Complete-language root
"grammar/antlr/ZamaniParser.g4"| Canonical parser composition
"grammar/antlr/ZamaniLexer.g4"| Canonical lexical composition
"grammar/lexer/"| Token/lexical authority
"grammar/core/"| Names, metadata, annotations, universal structures
"grammar/types/"| Ordinary type system
"grammar/expressions/"| Ordinary expression system and metaprogramming expression integration
"grammar/statements/"| Statement integration
"grammar/declarations/"| Declaration integration
"grammar/functions/"| Function/type-level integration
"grammar/macros/"| Macro syntax, expansion and hygiene
"grammar/effects/"| Effect semantics
"grammar/resources/"| Resource semantics
"grammar/security/"| Security and authorization
"grammar/validation/"| Contracts and semantic validation
"grammar/policies/"| Policy semantics
"grammar/compile/"| Compiler intent and compilation pipeline
"grammar/classical/"| Classical semantic realization
"grammar/quantum/"| Quantum semantic realization
"grammar/hybrid/"| Hybrid computation
"grammar/hdl/"| HDL semantics
"grammar/hardware/"| Hardware intent/resources
"grammar/ai/"| AI/ML semantics
"grammar/data/"| Data/schema/query semantics
"grammar/distributed/"| Distributed semantics
"grammar/networking/"| Networking semantics
"grammar/interoperability/"| FFI/ABI/external formats
"grammar/dialects/"| Extensible domain-specific syntax
"grammar/compatibility/"| Versioning and migration
"grammar/tests/"| Conformance
frontend AST| Canonical structural representation
semantic layer| Meaning and validation
canonical IR| Target-independent representation
"quantum::ir"| Canonical quantum representation
compiler| Transformation and lowering
runtime/HAL| Execution

---

86. Independent-File-First Implementation Order

The subsystem must be implemented in dependency order.

Step 1 — Authority

Complete:

README.md
normative metaprogramming specification
formal metaprogramming contracts

before changing leaf grammar semantics.

Step 2 — Lexical prerequisites

Verify:

canonical token registry
canonical lexer
Rust lexer
ANTLR lexer

before introducing or depending on new reserved words.

Step 3 — Leaf grammars

Complete independently:

quotation.g4
unquotation.g4
syntax-tree.g4
compile-time-execution.g4
compile-time.g4
generation.g4
code-generation.g4
reflection.g4
introspection.g4
specialization.g4
type-level.g4
schemas.g4
capabilities.g4

Step 4 — Composition

Complete:

metaprogramming.g4

only after the leaf public interfaces are known.

Step 5 — Expression integration

Integrate with:

grammar/expressions/metaprogramming.g4

without creating duplicate public ownership.

Step 6 — Canonical parser

Integrate through:

grammar/antlr/ZamaniParser.g4

Step 7 — AST

Verify all facilities map into the canonical domain-neutral AST.

Step 8 — Semantics

Integrate:

types
effects
capabilities
resources
contracts
policies
provenance
security

Step 9 — Compiler

Integrate:

compile-time evaluator
macro engine
generation
specialization
reflection
type-level evaluation

through the repository's actual compiler infrastructure.

Step 10 — IR

Verify all generated/transformed domain structures enter the normal canonical IR pipeline.

Quantum structures must enter "quantum::ir".

Step 11 — Cross-domain validation

Test:

classical
quantum
hybrid
HDL
hardware
AI
data
distributed
networking
interoperability

Step 12 — Production gates

Run:

ANTLR validation
Rust parser validation
AST validation
semantic validation
security validation
determinism validation
scalability validation
compatibility validation
hard-coding audit

---

87. Definition of Done for Every File

A file is complete only when:

[ ] Purpose is explicit
[ ] Ownership is explicit
[ ] Non-ownership is explicit
[ ] Dependencies are explicit
[ ] Dependency direction is explicit
[ ] Lexer contract is explicit
[ ] Public rules are explicit
[ ] AST contract is explicit
[ ] Semantic contract is explicit
[ ] Type contract is explicit
[ ] Effect contract is explicit
[ ] Capability contract is explicit
[ ] Resource contract is explicit
[ ] Policy contract is explicit
[ ] Provenance contract is explicit
[ ] IR destination is explicit
[ ] Compiler consumer is explicit
[ ] Runtime boundary is explicit
[ ] Security boundary is explicit
[ ] Diagnostics are defined
[ ] Compatibility is defined
[ ] Positive tests exist
[ ] Negative tests exist
[ ] Boundary tests exist
[ ] Scalability tests exist
[ ] Determinism tests exist where applicable
[ ] Cross-domain tests exist where applicable
[ ] Hard-coding audit passes
[ ] Rust 1.97+ compatibility is verified
[ ] No unsafe Rust is required
[ ] Canonical parser reaches the rule
[ ] No duplicate public owner exists
[ ] No phantom rule exists

---

88. Definition of Done for the Directory

"grammar/metaprogramming/" is production-ready only when:

[ ] README orchestrates every file
[ ] Every file has one permanent responsibility
[ ] Every public rule has one owner
[ ] No phantom imports remain
[ ] No duplicate public rules remain
[ ] Canonical lexer integration is verified
[ ] Canonical parser integration is verified
[ ] Expression integration is verified
[ ] Macro boundary is verified
[ ] AST mapping is verified
[ ] Semantic mapping is verified
[ ] Effects are integrated
[ ] Capabilities are integrated
[ ] Resources are integrated
[ ] Policies are integrated
[ ] Provenance is integrated
[ ] Security is integrated
[ ] Compile-time execution is controlled
[ ] Generated source is revalidated
[ ] Specialization is controlled
[ ] Reflection is controlled
[ ] Introspection is controlled
[ ] Type-level computation is controlled
[ ] Schema transformation is controlled
[ ] Classical integration is verified
[ ] Quantum integration is verified
[ ] quantum::ir remains canonical
[ ] HDL integration is verified
[ ] Hardware realization remains downstream
[ ] No universal resource ceilings exist
[ ] No vendor-specific universal syntax exists
[ ] No second IR exists
[ ] No second language exists
[ ] ANTLR generation succeeds
[ ] Rust frontend conformance succeeds
[ ] Semantic conformance succeeds
[ ] Positive tests pass
[ ] Negative tests pass
[ ] Boundary tests pass
[ ] Scalability tests pass
[ ] Determinism tests pass
[ ] Security tests pass
[ ] Compatibility tests pass
[ ] Safe-Rust requirement passes

---

89. What Must Never Happen

The metaprogramming subsystem must never become:

a second programming language
a second AST
a second type system
a second IR
a second quantum IR
a compiler backend
a runtime
a hardware database
a device registry
a physical resource allocator
a scheduler
a router
a QEC engine
a ZQN engine
a HAL
a vendor API
an unrestricted host-language escape hatch
an implicit security bypass
an application-specific DSL

---

90. What the Subsystem Must Become

It must become:

a stable source-language metaprogramming layer

with:

one language
one lexical authority
one parser authority
one AST model
one semantic model
one effect model
one capability model
one resource model
one policy model
one provenance model
one canonical IR architecture
one quantum IR boundary
one compiler pipeline
one target realization boundary
one compatibility model
one conformance model

and extensibility through:

generic syntax
semantic metadata
capabilities
resources
schemas
dialects
libraries
versioned contracts

---

91. Final Orchestration Graph

The permanent directory architecture is:

                         README.md
                            |
                            v
                 metaprogramming.g4
                            |
          +-----------------+------------------+
          |        |        |        |         |
          v        v        v        v         v
     compile   quotation  syntax   reflection generation
       time                tree
          |        |        |        |         |
          +--------+--------+--------+---------+
                            |
          +-----------------+------------------+
          |                 |                  |
          v                 v                  v
   introspection     specialization      type-level
          |                 |                  |
          +-----------------+------------------+
                            |
                  +---------+---------+
                  |                   |
                  v                   v
              schemas           capabilities
                  |                   |
                  +---------+---------+
                            |
                            v
                 canonical parser
                            |
                            v
                  domain-neutral AST
                            |
                            v
              semantic validation
                            |
       +--------------------+--------------------+
       |         |          |          |         |
       v         v          v          v         v
     types    effects   capabilities resources policies
       |         |          |          |         |
       +---------+----------+----------+---------+
                            |
                            v
                       provenance
                            |
                            v
                   canonical semantics
                            |
              +-------------+-------------+
              |             |             |
              v             v             v
          classical     quantum::ir    HDL/hardware
              |             |             |
              +-------------+-------------+
                            |
                            v
                      optimization
                            |
                            v
                         lowering
                            |
                    routing/scheduling
                            |
                    resilience / QEC
                            |
                           ZQN
                            |
                           HAL
                            |
                      target realization

---

92. Permanent Integration Rule

The entire subsystem is governed by:

«Metaprogramming expresses compile-time computation and source/semantic transformation; the canonical frontend defines structure; semantic analysis defines meaning; effects, capabilities, resources, contracts and policies define admissibility; provenance defines traceability; canonical IR defines representation; compiler infrastructure defines transformation; target infrastructure defines realization; runtime infrastructure defines execution.»

No layer may silently assume ownership of another layer.

---

93. Final POCO-REAF Principle

The metaprogramming subsystem contributes to POCO-REAF by allowing source-level computation to remain independent of machine realization.

The desired architecture is:

                    Zamani Program
                          |
                          v
                  Metaprogramming
                          |
                          v
                Stable source meaning
                          |
                          v
                  Canonical AST
                          |
                          v
                 Semantic validation
                          |
          +---------------+---------------+
          |       |       |       |       |
          v       v       v       v       v
        Types  Effects  Resources  Capabilities  Policies
                          |
                          v
                     Provenance
                          |
                          v
                  Canonical semantic model
                          |
          +---------------+---------------+
          |               |               |
          v               v               v
      Classical       quantum::ir     HDL/Hardware
          |               |               |
          +---------------+---------------+
                          |
                          v
                    Optimization
                          |
                          v
                      Lowering
                          |
                    Routing/Scheduling
                          |
                 Resilience / Recovery
                          |
                       ZQN / HAL
                          |
                          v
                 Target realization
                          |
          +---------------+---------------+
          |       |       |       |       |
         CPU     GPU     FPGA    ASIC     QPU
          |       |       |       |       |
          +-------+-------+-------+-------+
                          |
                 Embedded / HPC /
              Distributed / Cloud /
                  Future targets

The same source-level meaning must survive this process.

---

94. Final Production Rule

The single governing rule for "grammar/metaprogramming/" is:

«Metaprogramming may compute, inspect, construct, transform, derive, specialize and generate Zamani structures, but every resulting structure must remain subject to the canonical language, semantic, security, resource, capability, policy, provenance and IR pipelines.»

Therefore:

DO NOT hard-code hardware into metaprogramming.

DO NOT hard-code compiler capacity into language semantics.

DO NOT enumerate every future target.

DO NOT enumerate every quantum operation.

DO NOT create a second quantum IR.

DO NOT create a second AST.

DO NOT create a second type system.

DO NOT execute code while parsing.

DO NOT grant capabilities through syntax.

DO NOT treat reflection as unrestricted system access.

DO NOT treat generation as backend execution.

DO NOT treat compile-time execution as an implicit privilege.

DO NOT allow generated code to bypass validation.

DO NOT create duplicate public grammar owners.

DO NOT leave phantom grammar references.

DO NOT claim implementation merely because a .g4 file exists.

Instead:

DEFINE THE SEMANTICS
        |
        v
DEFINE THE CONTRACT
        |
        v
DEFINE THE LEXICAL REQUIREMENTS
        |
        v
DEFINE THE GRAMMAR
        |
        v
DEFINE THE AST MAPPING
        |
        v
DEFINE THE SEMANTIC MAPPING
        |
        v
DEFINE EFFECTS
        |
        v
DEFINE CAPABILITIES
        |
        v
DEFINE RESOURCES
        |
        v
DEFINE POLICIES
        |
        v
DEFINE PROVENANCE
        |
        v
DEFINE THE IR BOUNDARY
        |
        v
DEFINE THE COMPILER CONSUMER
        |
        v
DEFINE THE TARGET BOUNDARY
        |
        v
DEFINE THE TESTS
        |
        v
AUDIT FOR HARD-CODING
        |
        v
VERIFY SAFE RUST
        |
        v
VERIFY ANTLR/RUST CONFORMANCE
        |
        v
MARK THE FEATURE STABLE

---

95. Final Statement

"grammar/metaprogramming/" is therefore a phase-aware, target-independent, open-world metaprogramming subsystem.

It provides the mechanisms necessary for Zamani to express:

- compile-time computation;
- quotation;
- unquotation;
- syntax-tree transformation;
- reflection;
- introspection;
- generation;
- specialization;
- type-level computation;
- schema transformation;
- controlled compile-time capabilities;

while integrating cleanly with:

- macros;
- expressions;
- declarations;
- types;
- effects;
- resources;
- capabilities;
- validation;
- policies;
- provenance;
- security;
- compilation;
- classical computation;
- quantum computation;
- "quantum::ir";
- hybrid computation;
- HDL;
- hardware;
- AI;
- data;
- distributed computing;
- networking;
- interoperability;
- dialects;
- future computational domains.

Its purpose is not to make the grammar infinitely large.

Its purpose is to make the semantic architecture extensible without requiring the universal grammar to know the future.

The implementation must remain compatible with Rust 1.97 or later and must use safe Rust only.

The subsystem must remain free of artificial language-level capacity ceilings.

The source program must describe stable computation and contractual requirements rather than today's physical machine.

That is the metaprogramming architecture required for Zamani to preserve:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

from the smallest supported computation to the largest computation that the program's semantics, implementation, target capabilities, and available resources can validly realize.