Zamani Language-Version Compatibility Contract

Path: "grammar/compatibility/language-version.md"
Status: Normative compatibility contract
Language: Zamani
Primary scope: Language-version compatibility, version resolution, version identity, source evolution, migration, reproducibility, cross-layer compatibility, and long-term semantic preservation
Grammar technology: ANTLR4-compatible grammar composition
Rust implementation baseline: Rust 1.97 or later
Rust edition: 2021
Rust safety requirement: Production Zamani Rust implementation MUST use safe Rust; Rust "unsafe" MUST NOT be required or used
Primary portability objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Scalability objective: No artificial language-level capacity ceiling; actual execution is constrained only by semantics, representation requirements, implementation capabilities, declared requirements, target capabilities, policies, physical feasibility, and available resources

---

1. Purpose

This document defines the compatibility contract specifically concerned with the Zamani language version.

It connects the normative language-version model with:

- lexical compatibility;
- grammar compatibility;
- parser compatibility;
- AST compatibility;
- name and module compatibility;
- type compatibility;
- effect compatibility;
- resource compatibility;
- capability compatibility;
- semantic compatibility;
- classical computation;
- quantum computation;
- "quantum::ir";
- HDL and hardware intent;
- hybrid computation;
- AI and data computation;
- distributed computation;
- networking;
- security;
- interoperability;
- dialects;
- metaprogramming;
- compilation;
- execution;
- artifacts;
- ABI boundaries;
- runtime compatibility;
- target compatibility;
- migration;
- deprecation;
- diagnostics;
- reproducibility;
- provenance;
- tooling.

This file does not define the complete Zamani language.

Its responsibility is to answer:

«Given a Zamani language contract identified by a language version, what compatibility guarantees exist, how is that version resolved, how is evolution classified, and how does the version identity propagate through the compiler and execution architecture?»

The normative language semantics remain owned by:

grammar/specification/

The cross-layer implementation/version contract remains owned by:

grammar/spec/

The release and compatibility policy remains coordinated with:

grammar/compatibility/

The canonical grammar remains:

grammar/Zamani.g4

The executable Rust frontend remains owned by the corresponding "src/" frontend modules.

This file connects those contracts.

---

2. Core Principle

The fundamental compatibility invariant is:

«A compatible Zamani implementation MUST NOT silently change the specified meaning of valid existing Zamani source.»

Compatibility therefore means semantic preservation, not textual identity.

The following are intentionally different:

same source
same AST
same IR
same artifact
same binary
same runtime
same target
same execution result

They are related, but none of them automatically implies the others.

Two implementations MAY use different:

- lexer implementations;
- parser implementations;
- AST layouts;
- optimization algorithms;
- canonical representations;
- IR encodings;
- instruction-selection strategies;
- scheduling algorithms;
- routing algorithms;
- memory layouts;
- runtime implementations;
- hardware mappings;

and still implement the same Zamani language semantics.

Conversely, two implementations MAY accept identical source text while being incompatible if they assign different semantic meaning to that source.

Therefore:

representation equality
        !=
semantic compatibility

and:

representation difference
        !=
semantic incompatibility

---

3. Repository Authority

Language-version compatibility MUST respect the repository's authority model.

The intended relationship is:

grammar/DESIGN.md
        |
        v
grammar/specification/
        |
        |-- language meaning
        |-- language-version semantics
        |-- type semantics
        |-- effect semantics
        |-- resource semantics
        |-- capability semantics
        |-- domain semantics
        |
        v
grammar/spec/
        |
        |-- machine-checkable contracts
        |-- cross-layer versioning
        |-- compatibility interpretation
        |
        v
grammar/compatibility/
        |
        |-- compatibility policy
        |-- migration policy
        |-- deprecation policy
        |-- compatibility matrix
        |-- conformance contracts
        |
        v
grammar/Zamani.g4
        |
        v
ANTLR lexer/parser representation
        |
        v
Rust lexer/parser
        |
        v
domain-neutral AST
        |
        v
structural validation
        |
        v
semantic analysis
        |
        +-----------------------+
        |                       |
        v                       v
 classical semantics      quantum semantics
        |                       |
        v                       v
 classical IR             quantum::ir
        |                       |
        +-----------+-----------+
                    |
                    v
             optimization
                    |
                    v
             lowering/routing
                    |
                    v
              scheduling
                    |
                    v
        resilience / recovery / QEC
                    |
                    v
                   ZQN
                    |
                    v
                   HAL
                    |
                    v
             target realization

No lower layer may silently redefine the language-version meaning established by a higher layer.

---

4. Ownership of This File

This file owns:

1. language-version compatibility interpretation;
2. version compatibility guarantees;
3. compatibility classes;
4. version identity propagation;
5. compatibility consequences of language evolution;
6. version-resolution requirements;
7. compatibility metadata requirements;
8. version-aware migration requirements;
9. version-aware artifact requirements;
10. version-aware conformance requirements;
11. language/compiler/IR/runtime/target version separation;
12. POCO-REAF versioning requirements.

This file does not own:

- concrete language syntax;
- parser grammar production definitions;
- lexer token definitions;
- AST node implementation;
- semantic implementation;
- optimizer implementation;
- routing algorithms;
- scheduling algorithms;
- QEC implementation;
- ZQN implementation;
- hardware calibration;
- backend implementation;
- target discovery implementation;
- physical resource limits;
- runtime implementation details.

Those remain with their respective owners.

---

5. Required Companion Files

This contract is intentionally integrated with the following repository files.

File| Responsibility
"grammar/DESIGN.md"| Overall grammar architecture and invariants
"grammar/README.md"| Navigation
"grammar/grammar.md"| Implementation/conformance status
"grammar/Zamani-Grammar.md"| Historical, extended, proposed and explanatory grammar material
"grammar/specification/language-version.md"| Normative language-version model
"grammar/spec/versioning.md"| Cross-layer versioning contract
"grammar/spec/compatibility.md"| Compatibility dimensions and interpretation
"grammar/core/versioning.g4"| Version-related source grammar
"grammar/lexer/"| Lexical representation and token compatibility
"grammar/compatibility/versions.md"| General version/release compatibility policy
"grammar/compatibility/migrations.md"| Migration procedures
"grammar/compatibility/deprecated.md"| Deprecation lifecycle
"grammar/compatibility/compatibility-matrix.md"| Compatibility relationships
"grammar/compatibility/ast-conformance.md"| AST conformance
"grammar/compatibility/frontend-conformance.md"| Frontend conformance
"grammar/compatibility/ir-conformance.md"| IR conformance
"grammar/compatibility/dialect-compatibility.md"| Dialect compatibility
"grammar/compatibility/feature-gates.md"| Feature-state gating
"grammar/compatibility/reserved.md"| Reserved syntax/identifiers
"grammar/Zamani.g4"| Canonical ANTLR composition root
"src/lexer.rs"| Executable lexer
"src/parser.rs"| Executable parser
"src/ast/" and/or "src/frontend/ast/"| Domain-neutral frontend AST
semantic subsystem| Semantic validation
canonical IR subsystem| Canonical intermediate representation
"quantum::ir"| Canonical quantum semantic IR boundary
compiler/backend subsystem| Target realization
runtime subsystem| Runtime execution
test suites| Conformance evidence
"Cargo.toml"| Rust implementation baseline

No file listed above may redefine this document's compatibility semantics without an explicit authority decision.

---

6. Version Identity

A Zamani language version identifies a language contract.

It does not identify:

- a compiler binary;
- a Rust version;
- an operating system;
- a CPU generation;
- a GPU generation;
- an FPGA generation;
- an ASIC generation;
- a QPU;
- a physical device;
- a cluster;
- a cloud provider;
- a runtime process;
- a deployment location;
- a memory capacity;
- a device count.

Conceptually:

LanguageVersion
    =
    source lexical contract
    +
    source grammar contract
    +
    source semantic contract
    +
    type contract
    +
    effect contract
    +
    resource/capability contract
    +
    compatibility contract

Target information is separate.

Compiler information is separate.

IR information is separate.

Runtime information is separate.

---

7. Version Number Model

Zamani language versions SHOULD use:

MAJOR.MINOR.PATCH

with optional pre-release and build metadata where defined by the normative version specification.

Examples:

1.0.0
1.1.0
1.1.1
2.0.0
2.0.0-alpha
2.0.0-beta
2.0.0-rc.1
2.0.0+build.7

The language-version grammar MUST NOT impose arbitrary upper bounds on numeric components.

For example, the grammar MUST NOT define:

0..255
0..65535
MAX_VERSION
MAX_MAJOR
MAX_MINOR
MAX_PATCH

A version component is limited only by the representation and validation contract actually required by the implementation.

A compiler implementation MAY reject a representation that cannot be represented safely, but that implementation limitation MUST NOT silently become a universal language limit.

---

8. Version Components

The canonical semantic components are:

major
minor
patch
pre_release
build_metadata

An implementation MAY internally support additional metadata, but additional implementation fields MUST NOT silently alter the language-version identity.

If the grammar accepts extended version syntax for compatibility, semantic validation MUST determine whether that syntax belongs to:

- canonical language version syntax;
- legacy syntax;
- dialect syntax;
- implementation-defined syntax;
- rejected syntax.

Parser acceptance alone does not establish language-version validity.

---

9. Major Version Compatibility

A major language-version transition MAY introduce intentionally breaking changes.

Examples include:

- changing evaluation semantics;
- changing ownership semantics;
- changing type meaning;
- changing effect meaning;
- changing resource semantics;
- changing capability semantics;
- changing module resolution;
- changing stable operator semantics;
- removing stable syntax;
- changing stable quantum semantics;
- changing stable measurement semantics;
- changing stable concurrency guarantees;
- changing stable hardware-intent semantics.

A major version MUST NOT be required merely because:

- a new compiler implementation exists;
- a backend is rewritten;
- a new CPU target is added;
- a new GPU target is added;
- a new FPGA target is added;
- a new ASIC target is added;
- a new QPU target is added;
- a new accelerator is added;
- routing improves;
- scheduling changes;
- QEC improves;
- ZQN changes internally;
- runtime performance improves;
- compilation becomes faster.

Those are not language-version changes unless the specified language semantics change.

---

10. Minor Version Compatibility

A minor language-version transition SHOULD add backward-compatible functionality.

Examples:

- new unambiguous syntax;
- new portable abstractions;
- new capabilities;
- new resource expressions;
- new contract forms;
- new effect categories;
- new interoperable representations;
- new domain constructs;
- new extensible operation forms;
- new dialect facilities.

A minor release MUST NOT silently change the meaning of existing stable source.

If a proposed minor change can reinterpret existing valid source, it MUST undergo breaking-change analysis.

---

11. Patch Version Compatibility

A patch version is reserved for compatible corrections.

Examples:

- diagnostic corrections;
- documentation corrections;
- conformance corrections;
- parser bug fixes that restore specified behavior;
- lexer bug fixes that restore specified behavior;
- compatibility metadata corrections;
- migration documentation corrections;
- implementation corrections preserving specified semantics;
- deterministic behavior corrections where the specification already requires determinism.

A patch release MUST NOT intentionally change the meaning of valid stable source.

If a bug fix changes specified semantics rather than correcting an implementation defect, it MUST be classified according to the resulting compatibility impact.

---

12. Compiler Version Independence

The Zamani language version MUST remain independent from the compiler version.

For example:

Language:
1.2.0

Compiler:
0.8.x

is valid.

A compiler MAY support multiple language versions:

compiler
  |
  +-- Zamani 1.0
  +-- Zamani 1.1
  +-- Zamani 1.2

A compiler MUST NOT require an exact compiler implementation version merely because a source program declares a language version.

Compiler implementation compatibility belongs to compiler/toolchain contracts.

---

13. Rust Version Independence

The Rust implementation version is not a Zamani language version.

The repository currently declares a Rust implementation baseline compatible with:

Rust 1.97 or later
Rust edition 2021

The repository's actual Cargo manifest remains authoritative for the implementation baseline.

This compatibility document MUST NOT introduce Cargo syntax.

The Rust implementation MUST use safe Rust.

Production Zamani Rust code MUST NOT require or introduce:

unsafe

for language-versioning, grammar, parsing, semantic validation, compatibility checking, migration, diagnostics, or related functionality.

A newer Rust compiler MAY build the implementation without creating a new Zamani language version, provided language behavior remains compatible.

---

14. Rust Implementation Integration

The production compatibility path is:

language-version specification
        |
        v
version model
        |
        v
version parser/validator
        |
        v
frontend compilation context
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
semantic analysis

The effective language version MUST be available to every stage that has version-sensitive behavior.

This includes, where applicable:

- lexer configuration;
- parser configuration;
- keyword classification;
- grammar feature gating;
- AST construction;
- semantic analysis;
- type checking;
- effect checking;
- resource checking;
- capability checking;
- diagnostics;
- compatibility validation;
- migration;
- artifact generation;
- provenance;
- tooling.

Version-sensitive behavior MUST NOT be obtained implicitly from unrelated compiler state.

---

15. Canonical Version Resolution

Every compilable source unit MUST have a deterministic effective language version.

The effective version MAY originate from:

1. an explicit source declaration;
2. a project manifest;
3. a workspace configuration;
4. a documented legacy default.

The precedence MUST be defined by the normative language-version specification and MUST be implemented consistently.

The compiler MUST NOT resolve the language version differently depending on:

- operating system;
- CPU;
- GPU;
- QPU;
- filesystem ordering;
- network state;
- current time;
- locale;
- random state;
- backend selection.

Given identical source and identical version-resolution inputs:

effective_language_version

MUST be deterministic.

---

16. Explicit Source Version

Zamani MAY allow a source-level language-version declaration.

The existing versioning grammar provides machinery for declarations conceptually represented by forms such as:

language Zamani "1.0";

This compatibility document does not redefine that syntax.

The canonical syntax remains owned by:

grammar/specification/language-version.md
grammar/core/versioning.g4

The important invariant is:

source version declaration
        =
language contract selection

It MUST NOT mean:

select CPU
select GPU
select QPU
select memory size
select device
select node count

---

17. Version Declaration Semantics

A language-version declaration MUST identify the language contract.

It MUST NOT directly select:

- hardware;
- compiler executable;
- runtime executable;
- operating system;
- device identifier;
- physical qubit;
- memory bank;
- network endpoint.

Those concerns belong to other contracts.

For example:

language Zamani "1.0.0";

means, conceptually:

«Interpret this source according to the specified Zamani 1.0.0 language contract.»

It does not mean:

«Use compiler version 1.0.0.»

---

18. Version Ranges

Version ranges MAY be used for:

- package compatibility;
- dialect compatibility;
- compiler compatibility;
- runtime compatibility;
- API compatibility;
- ABI compatibility;
- grammar compatibility;
- language compatibility.

Version ranges MUST remain semantically distinct from resource requirements.

For example:

>= 1.2.0
< 2.0.0

describes a version relationship.

It does not describe:

>= 2 GPUs
>= 64 GB memory
>= 100 qubits

Resource requirements belong to the resource/capability system.

---

19. Version Range Resolution

A version range MUST be resolved against a named version domain.

For example:

language
compiler
runtime
dialect
package
api
abi
grammar

The implementation MUST NOT infer the version domain from a numeric value.

These are invalid semantic assumptions:

1.0.0 == language version
1.0.0 == compiler version
1.0.0 == runtime version

without an explicit subject.

Version requirements SHOULD therefore retain their subject identity through AST and semantic analysis.

---

20. Version Domains

Zamani MUST distinguish at least:

Language Version
Grammar Contract Version
Lexer Contract Version
AST Contract Version
Semantic Model Version
Type-System Contract Version
Effect-System Contract Version
Resource/Capability Contract Version
Classical IR Version
Quantum IR Version
HDL/Hardware IR Version
Dialect Version
Package Version
API Version
ABI Version
Artifact Format Version
Serialization Version
Compiler Version
Runtime Version
Target Descriptor Version
Backend Version
Toolchain Version

Changing one version does not automatically change every other version.

For example:

new compiler
    !=
new language version

and:

new GPU backend
    !=
new language version

and:

new quantum::ir encoding
    !=
new source syntax

unless the corresponding public semantic contract changes.

---

21. Version Identity Versus Compatibility Identity

A version number identifies a contract.

A compatibility identity describes whether two contracts can interoperate.

Therefore:

Version A
Version B

does not by itself answer:

Are A and B compatible?

Compatibility requires evaluation of:

- syntax;
- semantics;
- types;
- effects;
- resources;
- capabilities;
- AST;
- IR;
- artifacts;
- ABI;
- runtime;
- dialects;
- target requirements.

The compatibility result MUST identify the relevant dimension.

---

22. Compatibility Dimensions

Zamani compatibility MUST NOT be represented only as:

compatible = true

unless the scope is explicitly defined.

The minimum compatibility dimensions are:

1. source;
2. lexical;
3. syntax;
4. parser;
5. AST;
6. names/modules;
7. types;
8. effects;
9. resources;
10. capabilities;
11. semantics;
12. determinism;
13. dialect;
14. classical IR;
15. "quantum::ir";
16. HDL/hardware representation;
17. artifact;
18. serialization;
19. ABI;
20. runtime;
21. target;
22. execution;
23. tooling;
24. diagnostics;
25. interoperability;
26. security;
27. provenance.

A diagnostic SHOULD identify the failing dimension.

Recommended diagnostic families include:

ZMN-COMP-LANGUAGE-VERSION
ZMN-COMP-SOURCE
ZMN-COMP-LEXICAL
ZMN-COMP-SYNTAX
ZMN-COMP-AST
ZMN-COMP-TYPE
ZMN-COMP-EFFECT
ZMN-COMP-RESOURCE
ZMN-COMP-CAPABILITY
ZMN-COMP-SEMANTIC
ZMN-COMP-DIALECT
ZMN-COMP-IR
ZMN-COMP-QUANTUM-IR
ZMN-COMP-HDL
ZMN-COMP-ARTIFACT
ZMN-COMP-ABI
ZMN-COMP-RUNTIME
ZMN-COMP-TARGET
ZMN-COMP-EXECUTION
ZMN-COMP-TOOLING
ZMN-COMP-DIAGNOSTIC
ZMN-COMP-PROVENANCE

---

23. Compatibility Classes

Every stable or versioned feature MUST have an explicit compatibility classification.

The supported classes are:

STABLE
COMPATIBLE_EXTENSION
EXPERIMENTAL
DEPRECATED
REMOVED
RESERVED
IMPLEMENTATION_DEFINED
TARGET_DEFINED
DIALECT_DEFINED
SPECIFIED_NOT_IMPLEMENTED

A feature MUST NOT exist in an undocumented compatibility state.

---

24. STABLE

A stable feature:

- is normatively specified;
- has defined lexical behavior where applicable;
- has defined syntax;
- has defined AST behavior;
- has defined semantics;
- has type behavior;
- has effect behavior where applicable;
- has resource/capability behavior where applicable;
- has IR behavior where applicable;
- has diagnostics;
- has tests;
- has compatibility rules;
- is suitable for production use.

Grammar presence alone is insufficient.

---

25. COMPATIBLE_EXTENSION

A compatible extension adds capability without changing the meaning of existing valid stable programs.

Examples:

- new target support;
- new backend;
- new simulator;
- new accelerator;
- new capability;
- new resource expression;
- new interoperable format;
- new dialect;
- new unambiguous language feature;
- new optimization;
- new diagnostics.

Every compatible extension MUST undergo compatibility analysis.

---

26. EXPERIMENTAL

An experimental feature MAY change or be removed.

It MUST have:

- stable feature identity;
- owning specification;
- status;
- syntax contract;
- AST contract;
- semantic contract;
- compatibility classification;
- implementation status;
- tests;
- migration considerations.

Experimental features MUST NOT silently become stable.

Experimental syntax MUST NOT silently reinterpret existing stable syntax.

---

27. DEPRECATED

A deprecated feature remains available under its documented compatibility contract.

Deprecation MUST identify:

- feature identity;
- first deprecated version;
- reason;
- replacement;
- semantic differences;
- migration path;
- warning policy;
- earliest permitted removal version;
- affected tests;
- affected tooling.

Deprecation MUST NOT be used to hide incomplete architectural ownership.

---

28. REMOVED

A removed feature is no longer valid for the language version where removal applies.

Removed syntax MUST NOT be silently reinterpreted as another construct.

The compiler SHOULD provide a structured diagnostic identifying:

- the removed feature;
- the version where it was removed;
- the last supported version;
- the replacement;
- migration guidance.

---

29. RESERVED

Reserved syntax and identifiers are intentionally unavailable for ordinary source use.

Reserved elements exist to preserve future evolution space.

A reserved construct MUST NOT accidentally become valid because a parser rule happens to match it.

Reserved status is owned by:

grammar/compatibility/reserved.md

This file defines only its compatibility implications.

---

30. IMPLEMENTATION_DEFINED

Implementation-defined behavior is behavior for which the language specification explicitly permits a set of implementation choices.

The implementation MUST document its selected behavior.

Implementation-defined behavior MUST NOT be silently presented as universal Zamani semantics.

---

31. TARGET_DEFINED

Target-defined behavior depends on a selected target contract.

Target-defined behavior MUST remain distinguishable from portable language semantics.

For example:

portable semantic intent
        |
        v
target capability
        |
        v
target realization

A target-defined decision MUST NOT silently rewrite portable source semantics.

---

32. DIALECT_DEFINED

Dialect-defined behavior belongs to an explicitly selected dialect.

A dialect MUST identify:

name
namespace
version
owner
syntax extensions
semantic extensions
AST mapping
IR mapping
compatibility policy

Dialect versioning MUST NOT silently become core language versioning.

---

33. SPECIFIED_NOT_IMPLEMENTED

A feature MAY be normatively specified before its implementation is complete.

In that state:

specified
    !=
implemented

The compiler MUST NOT advertise unsupported implementation as production-ready merely because the specification exists.

The status MUST be visible through the repository's conformance machinery.

---

34. Feature Lifecycle

The standard lifecycle is:

PROPOSED
    |
    v
DESIGNED
    |
    v
SPECIFIED
    |
    v
EXPERIMENTAL
    |
    v
IMPLEMENTED
    |
    v
TESTED
    |
    v
STABLE
    |
    v
DEPRECATED
    |
    v
REMOVED

A feature MAY remain:

SPECIFIED_NOT_IMPLEMENTED

for an arbitrary period.

A feature MUST NOT be marked stable merely because grammar rules exist.

---

35. Production Feature Completion

The required path is:

Specification
    |
    v
Lexical contract
    |
    v
Grammar
    |
    v
Lexer
    |
    v
Parser
    |
    v
AST
    |
    v
Structural validation
    |
    v
Name/module validation
    |
    v
Type validation
    |
    v
Effect validation
    |
    v
Capability/resource validation
    |
    v
Semantic validation
    |
    v
Canonical semantic representation
    |
    v
IR
    |
    v
IR verification
    |
    v
Compiler integration
    |
    v
Runtime/target integration where applicable
    |
    v
Positive tests
    |
    v
Negative tests
    |
    v
Boundary tests
    |
    v
Scalability tests
    |
    v
Determinism tests
    |
    v
Compatibility tests
    |
    v
Documentation
    |
    v
STABLE

A feature MUST NOT be advertised as stable if a required stage is absent.

---

36. No Phantom Features

Documentation MUST NOT imply implementation where implementation evidence is absent.

Every documented feature MUST have a status such as:

STABLE
IMPLEMENTED
EXPERIMENTAL
SPECIFIED_NOT_IMPLEMENTED
DEPRECATED
REMOVED
RESERVED
DIALECT_DEFINED
TARGET_DEFINED
IMPLEMENTATION_DEFINED

This rule applies especially to:

grammar/Zamani-Grammar.md

because that document may contain historical and aspirational material.

---

37. No Phantom Syntax

Parser acceptance MUST NOT be confused with feature implementation.

The required invariant is:

accepted source
      |
      v
AST

Every accepted construct MUST have a structural representation.

Then:

AST
      |
      v
semantic analysis

Every semantically valid construct MUST have a defined semantic interpretation.

Then:

semantic representation
      |
      v
IR

Every construct crossing an IR boundary MUST either:

1. lower correctly; or
2. produce a structured unsupported-feature diagnostic before code generation.

It MUST NOT disappear.

---

38. No Silent Semantic Loss

The following is prohibited:

source
  |
  v
parser accepts
  |
  v
AST drops information
  |
  v
IR drops information
  |
  v
different computation

Examples include silently dropping:

- quantum controls;
- adjoints;
- measurement semantics;
- effect information;
- capability requirements;
- resource constraints;
- type qualifiers;
- ownership information;
- contract conditions;
- policy constraints;
- provenance;
- important attributes;
- dialect metadata;
- hardware intent;
- deterministic-execution requirements.

Semantically relevant information MUST survive until the subsystem responsible for interpreting it.

---

39. Lexical Compatibility

Language-version evolution may affect:

- keywords;
- contextual keywords;
- identifiers;
- operators;
- delimiters;
- literals;
- numeric forms;
- Unicode;
- comments;
- interpolation;
- quantum literals.

The compatibility path is:

language-version specification
        |
        v
grammar/spec/lexical.md
        |
        +------------------+
        |                  |
        v                  v
grammar/lexer/        ANTLR lexer
        |                  |
        +--------+---------+
                 |
                 v
             src/lexer.rs

The lexical implementation MUST remain deterministic.

Given identical:

source
language version
feature configuration
dialect configuration
compatibility mode

the resulting tokenization MUST be deterministic.

---

40. Keyword Evolution

Introducing a globally reserved keyword may invalidate an existing identifier.

Therefore a new keyword MUST undergo:

- identifier collision analysis;
- lexical ambiguity analysis;
- parser ambiguity analysis;
- dialect collision analysis;
- macro collision analysis;
- tooling analysis;
- migration analysis.

Where semantics permit, new vocabulary SHOULD initially be introduced as:

- contextual keywords;
- dialect-scoped keywords;
- namespace-qualified constructs;
- attribute-based constructs;
- otherwise compatibility-preserving syntax.

This is particularly important as Zamani grows across classical, quantum, HDL, AI, distributed, networking, security, data, and future domains.

Application-specific concepts SHOULD generally remain identifiers, library names, dialect names, capabilities, or policies rather than consuming globally reserved keywords.

---

41. Token Identity

Token identity is a compatibility-sensitive lexical contract.

The canonical token relationship is:

grammar/lexer/
grammar/antlr/
grammar/Zamani.g4
        |
        v
src/lexer.rs
        |
        v
src/parser.rs

Duplicate token concepts MUST be explicitly reconciled.

Examples of concepts requiring centralized review include:

Question / QuestionMark
Ampersand / BitAnd
BitOr / Pipe
Arrow / ThinArrow

This document does not decide their final token identities.

The lexical authority does.

However, changing their interpretation can constitute a language-version compatibility change and MUST be assessed accordingly.

---

42. Operator Compatibility

Operator compatibility includes:

- spelling;
- token identity;
- precedence;
- associativity;
- arity;
- parse structure;
- semantic meaning;
- type behavior;
- effect behavior.

Changing precedence can be breaking even when source text remains lexically valid.

Changing semantic meaning is breaking even when the parse tree remains unchanged.

Therefore operator changes MUST be evaluated at both:

syntax compatibility

and:

semantic compatibility

levels.

---

43. AST Compatibility

The frontend AST is a structural representation of source semantics.

Language-version changes MUST identify AST consequences.

An AST change MUST document, where applicable:

- added nodes;
- removed nodes;
- changed node kinds;
- added fields;
- removed fields;
- changed field meaning;
- changed optionality;
- changed discriminants;
- source-span behavior;
- semantic consequences.

An internal AST refactoring MAY be source-compatible if the public AST contract is not exposed and semantic meaning is preserved.

If the AST is externally serialized or consumed by tools, its schema MUST be separately versioned.

---

44. AST and Target Independence

The source AST MUST NOT encode target-specific realization decisions merely because the target changes.

The AST MUST NOT require:

- physical CPU identifiers;
- physical GPU identifiers;
- physical FPGA identifiers;
- physical ASIC identifiers;
- physical QPU identifiers;
- physical qubit assignments;
- memory-bank assignments;
- routing decisions;
- schedule decisions;
- calibration records.

Those belong downstream.

For example:

logical quantum resource

is source/semantic information.

A mapping such as:

logical resource -> physical device resource

is a realization decision.

Changing the latter MUST NOT automatically change the language version.

---

45. Semantic Compatibility

Semantic compatibility is stronger than syntactic compatibility.

A language-version change is breaking if it changes observable specified behavior involving:

- evaluation;
- name resolution;
- binding;
- scope;
- types;
- ownership;
- lifetimes;
- effects;
- resource semantics;
- capability semantics;
- contracts;
- policies;
- provenance;
- concurrency;
- synchronization;
- numerical semantics;
- quantum state evolution;
- quantum measurement;
- classical feed-forward;
- HDL intent;
- distributed semantics;
- security semantics.

A syntax-preserving semantic change can therefore require a major language version.

---

46. Type Compatibility

Type-system evolution MUST preserve compatibility according to the specified type contract.

Compatibility analysis applies to:

- primitive types;
- generic types;
- bounds;
- associated types;
- function types;
- references;
- ownership;
- borrowing;
- linear types;
- affine types;
- arrays;
- tensors;
- records;
- sums;
- options;
- results;
- quantum types;
- hardware types;
- capability-bearing types;
- resource-bearing types;
- probabilistic/uncertain types.

An implementation MUST distinguish:

semantic type capacity

from:

target hardware capacity

For example:

Tensor<T, shape>

describes algorithmic semantics.

It MUST NOT silently encode the maximum tensor size of a particular accelerator into the language version.

---

47. Effect Compatibility

Effect semantics are part of language compatibility.

Version changes affecting:

- I/O;
- network;
- mutation;
- randomness;
- native execution;
- foreign calls;
- distributed execution;
- measurement;
- quantum effects;
- learning;
- adaptation;
- reflection;
- code generation;
- simulation;

MUST undergo semantic compatibility analysis.

A compiler MUST NOT silently remove an effect merely because a target cannot represent it.

---

48. Resource Compatibility

Resource requirements are separate from language versions.

Examples include:

requires qubits >= n;
requires memory >= required_memory;
requires capability("quantum.measurement");
requires capability("gpu.compute");
requires capability("tensor.compute");
requires topology(required_topology);

These describe requirements.

They do not establish language-version identity.

A target that cannot satisfy a requirement is not automatically a source-language compatibility failure.

The implementation SHOULD distinguish:

language incompatibility
resource infeasibility
capability unavailability
target incompatibility

---

49. No Artificial Compatibility Ceilings

This document MUST NOT establish universal limits such as:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_PROGRAM_SIZE
MAX_MODULE_COUNT
MAX_FUNCTION_COUNT
MAX_VERSION_COMPONENT
MAX_VERSION_RANGE_COUNT

These MUST NOT be hidden in compatibility tables, examples, implementation guidance, or version policies.

A version number MUST NOT encode machine capacity.

---

50. Scalability Principle

Zamani scalability means:

«The language-version architecture MUST NOT impose arbitrary domain-specific capacity ceilings.»

Actual execution can be limited by:

- mathematical requirements;
- representation limits;
- compiler resources;
- memory availability;
- target capabilities;
- resource availability;
- physical feasibility;
- execution policies;
- security policy;
- runtime constraints;
- implementation limits.

These are different from language expressiveness.

The required distinction is:

language expressiveness
        !=
representation capacity
        !=
compiler capacity
        !=
target capacity
        !=
currently tested capacity

---

51. Representation Limits

The language MUST NOT falsely promise that every finite implementation can represent every mathematically expressible value.

If a representation limit exists, it MUST be:

1. technically necessary;
2. explicitly documented;
3. associated with the responsible layer;
4. distinguishable from a language-level semantic restriction;
5. reported through a structured diagnostic where applicable;
6. avoided in portable source semantics wherever practical.

A representation limit MUST NOT be disguised as a language version.

---

52. Quantum Compatibility

Quantum language versions MUST preserve logical quantum semantics.

Compatibility includes, where applicable:

- qubit identity;
- logical resource identity;
- register identity;
- operation ordering;
- operation parameters;
- controls;
- adjoints;
- measurement;
- reset;
- observables;
- classical feed-forward;
- dynamic control;
- probability semantics;
- state evolution;
- entanglement;
- channels;
- noise intent;
- error-correction requirements.

The language version MUST NOT encode a fixed hardware gate catalog.

The current physical gate set is a target property.

---

53. Generic Quantum Operations

Quantum syntax is intentionally extensible.

The compatibility architecture MUST permit the canonical quantum operation model to represent, where specified:

operationSpecifier
targets
parameters
results
attributes
modifiers
effects
capabilities
resources
provenance

The language MUST NOT require a new language version merely because a new compatible quantum operation becomes available on a target.

A new operation MAY be:

- built-in;
- custom;
- vendor-defined;
- dialect-defined;
- parameterized;

provided the applicable semantic contract exists.

---

54. "quantum::ir" Compatibility

The repository's canonical quantum semantic boundary is:

quantum::ir

The compatibility path is:

Zamani source
      |
      v
frontend AST
      |
      v
semantic quantum model
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
resilience / QEC / ZQN
      |
      v
HAL
      |
      v
target

Quantum IR versioning is separate from language versioning.

A change to "quantum::ir" MUST NOT automatically require a language-version change if source semantics remain unchanged.

Conversely, a change to quantum source semantics MUST be reflected in language compatibility even if "quantum::ir" remains structurally unchanged.

---

55. Quantum Target Compatibility

Quantum targets may differ in:

- available logical resources;
- physical resources;
- connectivity;
- native operations;
- measurement capabilities;
- timing;
- calibration;
- noise;
- fidelity;
- error-correction capabilities;
- dynamic-circuit support.

These are target properties.

The source language MUST express requirements rather than embed a universal physical configuration.

A target failure MUST identify the missing:

capability
resource
topology
policy

where determinable.

---

56. QEC and ZQN Compatibility

Changes in:

- QEC algorithms;
- QEC scheduling;
- syndrome-processing implementation;
- physical error models;
- resilience mechanisms;
- ZQN implementation;

do not automatically constitute language-version changes.

They become language compatibility concerns when the specified source semantics change.

The versioning system MUST therefore preserve the boundary:

language semantics
        |
        v
quantum::ir
        |
        v
QEC / resilience / ZQN
        |
        v
physical realization

---

57. Classical Compatibility

Classical computation remains a first-class Zamani domain.

Language-version compatibility applies to:

- scalar operations;
- integer semantics;
- floating-point semantics;
- vectors;
- matrices;
- tensors;
- symbolic computation;
- numerical computation;
- statistics;
- optimization;
- scientific computation;
- control computation;
- parallel computation.

Adding a new CPU or accelerator backend is not a language-version change unless source semantics change.

---

58. HDL and Hardware Compatibility

Hardware language semantics MUST distinguish:

hardware intent

from:

hardware realization

Language compatibility therefore does not depend on a specific:

- FPGA family;
- ASIC generation;
- register width;
- memory technology;
- clock frequency;
- interconnect;
- device count;
- accelerator generation.

A target may fail to realize an intent.

That is target feasibility.

It is not automatically language incompatibility.

---

59. Hybrid Compatibility

Hybrid classical/quantum/accelerator programs MUST preserve source-level semantic relationships across domains.

Examples include:

classical -> quantum
quantum -> classical
measurement -> classical control
classical -> accelerator
accelerator -> classical
AI -> quantum
quantum -> AI
HDL -> software
software -> hardware

A language-version change affecting the boundary between these domains is compatibility-sensitive.

The version contract MUST NOT create independent incompatible language-version systems for each domain.

---

60. AI, Learning, Reasoning and Adaptation Compatibility

Language-version compatibility applies to generic computational constructs such as:

- inference;
- reasoning;
- deduction;
- knowledge;
- learning;
- adaptation;
- uncertainty;
- probability;
- evidence;
- provenance;
- explanation;
- decision records;
- agents;
- neural-symbolic composition.

Their semantic contracts MUST remain independent of application-specific naming.

A version change MUST distinguish:

core language semantics

from:

library semantics
dialect semantics
capability semantics
policy semantics
application semantics

Learning and adaptation MUST retain their effect, policy, capability, resource and provenance contracts across compatible versions.

Adaptation MUST NOT silently become unrestricted source-code mutation merely because a compiler version changes.

---

61. Contract Compatibility

Version changes affecting:

requires
ensures
invariant
assume
guarantee
property

MUST preserve their specified logical meaning.

A compiler MUST NOT silently weaken or strengthen a stable contract without compatibility classification.

Contract information MUST survive relevant frontend and semantic transformations.

---

62. Policy Compatibility

Policy semantics may affect:

- security;
- resources;
- execution;
- adaptation;
- deployment;
- simulation;
- target selection;
- interoperability.

A language-version change affecting policy semantics is compatibility-sensitive.

A target policy MUST NOT silently become a source-language rule.

---

63. Provenance Compatibility

Where provenance is specified, version evolution MUST preserve sufficient provenance to distinguish:

source version
compiler version
semantic transformation
IR version
dialect version
target realization
artifact version

A compatible transformation MUST NOT silently discard provenance needed for:

- reproducibility;
- audit;
- diagnostics;
- semantic explanation;
- migration;
- compatibility validation;
- security analysis.

---

64. Determinism

Where Zamani specifies deterministic behavior, language-version compatibility MUST preserve it.

Determinism may apply to:

- lexical analysis;
- parsing;
- name resolution;
- type checking;
- effect checking;
- version resolution;
- semantic normalization;
- reproducible compilation.

Some domains may intentionally introduce nondeterminism:

- randomness;
- distributed scheduling;
- concurrency;
- probabilistic computation;
- learning;
- physical quantum execution.

Such nondeterminism MUST be explicit in the semantic contract.

A compiler MUST NOT falsely claim deterministic execution merely because the source parses deterministically.

---

65. Reproducibility

Reproducibility MUST distinguish:

same source
same language semantics
same compiler inputs
same artifact
same target realization
same execution result

These are not equivalent guarantees.

A reproducible build SHOULD record:

- language version;
- dependency versions;
- dialect versions;
- compiler version;
- relevant IR versions;
- feature configuration;
- compatibility mode;
- target descriptor;
- relevant policy configuration;
- provenance.

Reproducibility MUST NOT require fixed physical hardware capacities in source semantics.

---

66. Artifact Compatibility

Compiled or serialized artifacts SHOULD contain enough metadata to establish their language contract.

Where applicable:

language_version
grammar_contract_version
semantic_contract_version
ast_contract_version
ir_version
quantum_ir_version
dialect_versions
package_versions
artifact_format_version
serialization_version
capability_requirements
resource_requirements
compiler_provenance

Artifacts MUST NOT depend solely on the version of the compiler executable that produced them.

---

67. Artifact Validation

Before executing or transforming an artifact, the consumer SHOULD determine:

1. language version;
2. artifact format version;
3. relevant IR versions;
4. dialect versions;
5. required capabilities;
6. required resources;
7. required policies;
8. compatibility status;
9. migration requirements.

An unsupported artifact MUST result in a structured diagnostic.

It MUST NOT be silently treated as an older or newer artifact format.

---

68. Forward Compatibility

A compiler MUST NOT claim support for an unknown future language version merely because the source happens to parse.

For example:

compiler supports 1.x
source declares 2.x

MUST NOT silently mean:

interpret 2.x as 1.x

unless the language specification explicitly defines a forward-compatible subset or extension mechanism.

Unknown semantics MUST produce an explicit diagnostic when they cannot be safely interpreted.

---

69. Backward Compatibility

A compiler supporting a newer compatible language version SHOULD accept older stable source according to the compatibility contract.

For example:

old stable source
       |
       v
new compiler
       |
       v
old language semantics preserved

Compatibility exceptions MUST be explicit for:

- experimental features;
- deprecated features;
- removed features;
- dialect-specific constructs;
- implementation-defined behavior;
- target-defined behavior.

---

70. Compatibility Does Not Mean Universal Acceptance

A compiler does not need to accept every source version forever.

Instead, it MUST clearly declare its supported language-version set.

For example:

supported:
1.0.x
1.1.x
1.2.x

unsupported:
0.x
2.x

An unsupported version MUST generate a diagnostic that identifies:

requested version
supported versions/ranges
reason
migration option where available

---

71. Version Selection Must Not Depend on Target

The effective language version MUST be resolved before target realization.

The following architecture is required:

source
  |
  v
language-version resolution
  |
  v
lexical/parser configuration
  |
  v
AST
  |
  v
semantic analysis
  |
  v
target-independent semantic representation
  |
  v
target selection

The compiler MUST NOT select a different language version merely because a target is:

- smaller;
- larger;
- quantum;
- classical;
- embedded;
- distributed;
- remote;
- accelerated;
- simulated.

Target constraints belong downstream.

---

72. POCO-REAF

POCO-REAF depends on semantic stability.

The intended architecture is:

Program Once
      |
      v
Stable source semantics
      |
      v
Canonical semantic representation
      |
      v
Compile Once
      |
      v
Portable artifact / canonical representation
      |
      v
Target capability negotiation
      |
      v
Specialization
      |
      v
Lowering
      |
      v
Routing / scheduling
      |
      v
Resilience / recovery
      |
      v
Target realization
      |
      v
Run Everywhere / Anywhere

POCO-REAF MUST NOT be interpreted as the impossible requirement that one historical native machine-code binary must execute unchanged on every future physical architecture.

Instead, it requires long-term preservation of source-level and semantic intent.

---

73. Forever Compatibility

"Forever" means:

semantic continuity where compatibility is promised

It does not mean:

no language evolution ever

Long-lived Zamani programs and artifacts SHOULD remain recoverable through:

- explicit version metadata;
- compatibility metadata;
- migration information;
- provenance;
- semantic normalization;
- version-aware tooling.

A future implementation SHOULD be able to determine:

what language contract was intended
what dialects were required
what semantics were required
what artifact representation was used
what capabilities were required
what transformations occurred

before attempting migration or execution.

---

74. Version Compatibility Across Dialects

A dialect is independently versioned.

The relationship is:

Zamani language version
        +
dialect identity
        +
dialect version
        =
effective source contract

A dialect MUST NOT silently override stable core semantics.

If a dialect changes a stable core construct, it MUST either:

1. use a separately scoped syntax;
2. use explicit versioning;
3. use an explicitly defined compatibility mode;
4. or be rejected as incompatible.

Dialect version changes MUST be evaluated independently from core language version changes.

---

75. Version Compatibility Across Packages

Packages have their own versions.

A package version MUST NOT silently redefine the language version.

The dependency relationship is conceptually:

package version
        |
        +--> required language range
        +--> required dialect ranges
        +--> required API ranges
        +--> required ABI ranges

The package manager/tooling layer owns dependency resolution.

The language compatibility layer owns interpretation of the language-version contract.

---

76. Version Compatibility Across APIs and ABIs

API and ABI versions are distinct from language versions.

For example:

language version
    !=
API version
    !=
ABI version

A language-compatible source program may depend on an incompatible external ABI.

That is an interoperability failure, not necessarily a language-version failure.

FFI/ABI compatibility MUST therefore be represented at the interoperability boundary.

---

77. Runtime Compatibility

Runtime compatibility is separate from source compatibility.

A runtime MAY change implementation while preserving language semantics.

However, runtime changes affecting observable language behavior MUST undergo semantic compatibility analysis.

Examples include changes to:

- memory ordering;
- concurrency guarantees;
- deterministic execution;
- exception/error behavior;
- effect handling;
- resource accounting;
- security guarantees;
- quantum execution semantics.

---

78. Target Compatibility

Target compatibility answers:

«Can this target realize the already-defined program semantics?»

It is not the same question as:

«Is this source valid Zamani?»

A target may fail because it lacks:

capability
resource
topology
runtime facility
security policy
ABI

Such failure MUST NOT be reported as a syntax error.

---

79. Target-Independent Source

Portable Zamani source SHOULD express:

intent
constraints
requirements
capabilities
preferences
policies
contracts
effects
provenance

rather than physical realization details.

For example:

requires capability("gpu.compute");

is portable intent.

A source-level requirement such as:

use physical GPU device 7

is target-specific and should be explicitly classified as such.

The compatibility system MUST preserve this distinction.

---

80. Resource Availability and Versioning

Resource availability MUST NOT determine the language version.

The following is prohibited:

Zamani-8Q
Zamani-64Q
Zamani-1024Core
Zamani-4GPU

as language-version identities.

Instead:

language version
+
semantic requirements
+
target capabilities
+
resource availability

determine feasibility.

---

81. Compiler Configuration

Compiler configuration MAY select:

- optimization strategy;
- diagnostics;
- debug information;
- target;
- execution mode;
- simulation mode;
- compatibility mode;
- migration mode.

Compiler configuration MUST NOT silently change stable source semantics.

A configuration that changes semantics MUST be explicitly represented and documented.

---

82. Feature Gates

Feature gates may be used for:

- experimental features;
- staged features;
- compatibility modes;
- migration modes;
- dialect activation.

A feature gate MUST identify:

feature identity
language version relationship
status
semantic contract
compatibility classification

A feature gate MUST NOT be used to disguise a target limitation as a language feature.

---

83. Compatibility Modes

A compiler MAY support compatibility modes such as:

strict
legacy
migration
diagnostic
experimental

if specified.

Compatibility modes MUST be explicit.

They MUST NOT silently activate based on:

- target hardware;
- compiler executable path;
- current date;
- environment variables without documented semantics;
- random state;
- filesystem state.

A compatibility mode affecting language meaning MUST be visible to tooling and diagnostics.

---

84. Legacy Defaults

If historical Zamani source did not contain an explicit version declaration, the compiler MAY use a legacy default.

The default MUST be:

- documented;
- deterministic;
- versioned;
- removable only through an explicit compatibility policy.

A legacy default MUST NOT continue indefinitely without a defined compatibility strategy if doing so creates ambiguity.

The compiler SHOULD warn when a source file relies on a legacy default where migration is appropriate.

---

85. Migration Contract

Every intentional breaking language change SHOULD provide migration guidance.

Migration metadata SHOULD identify:

old_version
new_version
feature
old_syntax
new_syntax
semantic_difference
automatic_migration
manual_migration
affected_ast
affected_ir
affected_dialects
affected_tools
affected_backends

Migration tooling SHOULD operate on structured syntax or AST information rather than fragile text substitution wherever possible.

---

86. Migration Safety

A migration MUST NOT claim success merely because the transformed source parses.

A valid migration must preserve, where promised:

- semantics;
- types;
- effects;
- resources;
- capabilities;
- contracts;
- policies;
- provenance;
- quantum semantics;
- hardware intent;
- deterministic guarantees.

The migration pipeline is:

old source
    |
    v
old language contract
    |
    v
parse
    |
    v
old AST
    |
    v
semantic interpretation
    |
    v
migration transformation
    |
    v
new AST
    |
    v
new semantic validation
    |
    v
new source / representation

---

87. Deprecation and Removal

Removal MUST be preceded by the repository's deprecation policy where practical.

The expected path is:

STABLE
   |
   v
DEPRECATED
   |
   v
migration available
   |
   v
warning period
   |
   v
removal eligibility
   |
   v
REMOVED

The exact minimum periods are owned by the release policy rather than this file.

---

88. Compatibility Matrix Integration

Every language-version transition MUST be represented in:

grammar/compatibility/compatibility-matrix.md

The matrix SHOULD distinguish:

source
lexical
syntax
AST
semantic
type
effect
resource
capability
IR
quantum::ir
artifact
ABI
runtime
target
dialect
tooling

A single cell marked:

compatible

is insufficient when different dimensions have different results.

---

89. Conformance Integration

Version compatibility MUST be tested by the repository's conformance suites.

At minimum:

grammar/compatibility/frontend-conformance.md
grammar/compatibility/ast-conformance.md
grammar/compatibility/ir-conformance.md
grammar/compatibility/compatibility-matrix.md

must be integrated with implementation evidence.

The compatibility system MUST detect:

- version declaration disagreement;
- lexer disagreement;
- parser disagreement;
- AST disagreement;
- semantic disagreement;
- IR disagreement;
- unsupported version acceptance;
- accidental future-version acceptance;
- incorrect migration;
- incorrect deprecation behavior;
- incorrect diagnostics.

---

90. Positive Compatibility Tests

For every supported compatible transition, tests SHOULD establish:

old source
+
new compiler
=
same specified semantics

Tests should include:

- minimal programs;
- generic programs;
- classical programs;
- quantum programs;
- hybrid programs;
- HDL programs;
- distributed programs;
- AI/data programs;
- networking programs;
- interoperability programs;
- metaprogramming programs.

---

91. Negative Compatibility Tests

Negative tests MUST verify that:

- unsupported versions are rejected;
- removed features are rejected;
- malformed versions are rejected;
- incompatible ranges are rejected;
- incompatible dialects are rejected;
- incompatible artifacts are rejected;
- unsupported IR versions are rejected;
- unknown future semantics are not silently accepted.

---

92. Boundary Compatibility Tests

Boundary tests MUST cover:

- major/minor transitions;
- minor/patch transitions;
- pre-release versions;
- build metadata;
- version ranges;
- empty or malformed declarations;
- legacy syntax;
- contextual keywords;
- dialect boundaries;
- AST schema changes;
- IR changes;
- artifact changes;
- target feasibility failures.

---

93. Scalability Tests

Versioning itself MUST be scalable.

Tests MUST verify that the implementation does not impose arbitrary fixed ceilings on:

- version component size;
- version metadata size;
- number of compatibility requirements;
- number of dialect requirements;
- number of package requirements;
- number of version predicates;
- number of features.

Tests MUST distinguish:

implementation capacity

from:

language semantic capacity

No test may accidentally establish a language ceiling merely because the test machine has finite resources.

---

94. Version Predicate Scalability

The version grammar and semantic implementation MUST permit an arbitrary number of logically valid constraints subject only to available implementation resources.

For example:

>= 1.0.0
and < 2.0.0
and != 1.4.0
and != 1.5.0
...

MUST NOT have a language-defined fixed maximum count.

The parser and semantic layer MAY reject a pathological input because an implementation resource budget is exhausted.

Such a failure is an implementation/resource diagnostic, not a language semantic limit.

---

95. Diagnostics

Version diagnostics MUST be structured.

At minimum, applicable diagnostics SHOULD contain:

code
severity
message
source span
requested version
effective version
supported versions/ranges
compatibility dimension
related feature
migration guidance

Examples:

ZMN-COMP-LANGUAGE-VERSION
ZMN-COMP-UNSUPPORTED-VERSION
ZMN-COMP-FUTURE-VERSION
ZMN-COMP-REMOVED-FEATURE
ZMN-COMP-DEPRECATED-FEATURE
ZMN-COMP-INCOMPATIBLE-DIALECT
ZMN-COMP-INCOMPATIBLE-ARTIFACT

A version error MUST NOT be reported as an unrelated syntax error when the source syntax is valid but the requested language contract is unsupported.

---

96. Source Spans

Version declarations and compatibility diagnostics MUST preserve source locations.

The versioning frontend MUST integrate with the repository source-map/span architecture.

A diagnostic SHOULD identify:

file
line
column
span

where applicable.

This is necessary for:

- compiler diagnostics;
- migration;
- IDE integration;
- language-server integration;
- tooling;
- automated migration.

---

97. Tooling Compatibility

Version-aware tooling includes:

- formatter;
- linter;
- language server;
- syntax highlighter;
- compiler;
- package manager;
- migration tools;
- documentation tools;
- AST tools;
- IR tools.

Tooling MUST NOT silently assume the newest language version.

Tools SHOULD obtain the effective language version through the same resolution mechanism as the compiler.

---

98. Formatter Compatibility

A formatter MUST preserve semantics across supported versions.

It MUST NOT automatically rewrite source from one language version to another unless explicitly requested.

For example:

format

and:

migrate

are different operations.

A formatter SHOULD preserve:

- version declaration;
- dialect declaration;
- compatibility mode;
- source semantics.

---

99. Language Server Compatibility

The language server MUST use the effective language version when providing:

- syntax highlighting;
- diagnostics;
- completion;
- code actions;
- migration suggestions;
- semantic information.

An editor MUST NOT highlight a feature as valid merely because it exists in the newest grammar if the source declares an older incompatible language version.

---

100. Macro and Metaprogramming Compatibility

Macros, compile-time computation, reflection, and syntax generation are version-sensitive.

Generated source or syntax trees MUST carry sufficient language-version context.

A macro MUST NOT silently generate syntax requiring a newer language version than the consuming source contract permits.

Where version translation is supported, the transformation MUST be explicit and validated.

---

101. Generated Source Compatibility

Generated Zamani source MUST identify or inherit its language contract deterministically.

Generated source MUST NOT silently depend on:

- generator version alone;
- host machine;
- target machine;
- compiler installation state.

The generation pipeline SHOULD preserve:

source provenance
generator provenance
language version
dialect versions
feature configuration

---

102. Interoperability Compatibility

External formats such as:

- SQL;
- JSON;
- XML;
- OpenQASM;
- foreign source;
- foreign binary formats;

MUST have their own compatibility contracts.

Their versions MUST NOT automatically become Zamani language versions.

The integration is:

external format
      |
      v
format-specific parser
      |
      v
Zamani semantic model
      |
      v
canonical representation

The language version remains the Zamani source semantic contract.

---

103. FFI and ABI Compatibility

FFI boundaries MUST explicitly distinguish:

Zamani language version
ABI version
foreign API version
foreign library version
target ABI

A change in a foreign ABI MUST NOT automatically change Zamani language semantics.

However, source declarations that depend on ABI semantics MUST undergo interoperability compatibility analysis.

---

104. Security Compatibility

Security semantics are compatibility-critical.

Changes to:

- authorization;
- authentication;
- capability enforcement;
- sandboxing;
- confidentiality;
- integrity;
- cryptographic semantics;
- provenance;
- policy enforcement;

MUST be treated as semantic compatibility changes when observable.

A security weakening MUST NOT silently appear as a patch-level language correction.

---

105. Simulation Compatibility

Simulation is an execution strategy.

Versioning MUST distinguish:

simulation semantics

from:

physical execution

A new simulator backend does not automatically require a language-version change.

A change in the specified semantics of "simulate" may.

Simulation may target:

- classical systems;
- quantum systems;
- HDL;
- distributed systems;
- AI systems;
- hardware behavior;
- fault behavior.

The versioning model MUST remain domain-neutral.

---

106. Adaptive Execution Compatibility

Adaptive execution may involve:

detect
evaluate
select
fallback
retry
recover
adapt

Changes to adaptive execution MUST preserve stable source semantics.

Implementation changes in:

- scheduling;
- recovery;
- target selection;
- resilience;

do not automatically constitute language-version changes.

If the source-level meaning of adaptation changes, compatibility analysis is mandatory.

---

107. Concurrency Compatibility

Version changes affecting:

- actor semantics;
- message ordering;
- synchronization;
- memory visibility;
- task semantics;
- async semantics;
- cancellation;
- failure behavior;

MUST undergo semantic compatibility analysis.

A backend may schedule differently while preserving semantics.

A scheduler implementation change alone is not necessarily a language-version change.

---

108. Distributed Compatibility

Distributed source semantics MUST remain independent of:

- number of nodes;
- cluster size;
- network topology;
- physical location;
- service placement.

A source program MUST NOT require a language-version change merely because it is deployed on a larger or smaller distributed system.

Changes to distributed consistency semantics or message guarantees are language compatibility concerns.

---

109. Networking Compatibility

Networking semantics MUST distinguish:

protocol
endpoint
transport
capability
security policy
deployment

Changing an endpoint location does not constitute a language-version change.

Changing the semantics of a networking operation does.

---

110. Hardware Capability Evolution

New hardware capabilities SHOULD normally be represented as:

capabilities
resources
target descriptors
dialects
backend support

rather than new language versions.

For example:

capability("quantum.mid_circuit_measurement")

can evolve independently of:

Zamani language version

provided the source semantics of the capability contract remain compatible.

---

111. Future Targets

The language-version system MUST NOT require enumeration of every future target.

A new target can be integrated through:

target descriptor
capabilities
resources
lowering
backend
HAL
runtime

without modifying the fundamental language-version model.

This is essential for long-term POCO-REAF.

---

112. Future Domains

The same principle applies to future computational domains.

A future domain SHOULD integrate through:

core language
      |
      v
domain semantics
      |
      v
domain IR
      |
      v
canonical execution architecture

rather than creating a competing language-version system.

The language-version contract is domain-neutral.

---

113. Version Metadata Schema

Where machine-readable version metadata is used, it SHOULD conceptually contain:

LanguageVersionMetadata {
    language
    version
    compatibility_policy
    feature_set
    dialects
    compiler_requirements
    runtime_requirements
    ir_requirements
    capability_requirements
    resource_requirements
    artifact_format
    provenance
}

The exact serialization format belongs to the machine-readable specification.

The schema MUST itself be versioned.

---

114. Version Metadata Immutability

Once a compilation context has resolved:

effective language version

that value MUST be immutable for the compilation of the source unit.

A later target decision MUST NOT mutate it.

Similarly:

resolved dialect version
resolved compatibility mode
resolved feature set

MUST be treated as explicit compilation-context state.

---

115. Compilation Cache Compatibility

Compilation caches MUST include every language-semantic input that can affect the generated result.

Where applicable, cache identity SHOULD include:

source semantic identity
language version
feature configuration
dialect versions
relevant dependency versions
compiler semantic version
IR version
target-independent semantic configuration

A target-specific cache MUST additionally include relevant target identity.

A cache MUST NOT reuse a result from a semantically incompatible language version.

---

116. Version and Provenance

Compiler provenance SHOULD record:

language_version
compiler_version
grammar_contract
semantic_contract
dialect_versions
ir_versions
source_identity
dependency_identity
configuration_identity

This allows future tools to determine why an artifact was produced.

---

117. Compatibility and Optimization

Optimization is not normally a language-version operation.

An optimizer MAY transform:

source semantics

into a different implementation representation while preserving semantic equivalence.

Examples:

- instruction fusion;
- loop transformation;
- vectorization;
- quantum gate optimization;
- tensor optimization;
- scheduling;
- routing;
- hardware-specific lowering.

An optimization MUST NOT change source semantics unless explicitly requested by a defined semantic mode.

---

118. Compatibility and Lowering

Lowering may depend on:

- target capabilities;
- resource availability;
- topology;
- runtime facilities;
- ABI;
- hardware.

Lowering decisions MUST NOT silently change the language-version contract.

If a lowering cannot preserve required semantics, compilation MUST fail or use an explicitly specified alternative semantic mode.

---

119. Compatibility and Resilience

Resilience states such as:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

belong to execution/runtime semantics.

Changes in the implementation of resilience do not automatically constitute language-version changes.

Changes to specified source-level failure semantics do.

---

120. Compatibility and Outcomes

Execution outcomes such as:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

must remain versioned according to their owning execution/resilience contract.

A change in their source-visible meaning is compatibility-sensitive.

A private runtime implementation change is not automatically a language-version change.

---

121. Compatibility and Contracts

Version transitions MUST preserve the distinction between:

requires
ensures
invariant
assume
guarantee
property

Changing the interpretation of a stable contract construct is a semantic compatibility change.

Changing only the internal proof engine is not automatically a language-version change.

---

122. Compatibility and Evidence

Where evidence is part of the language semantic model, versioning MUST preserve:

claim
evidence
source
confidence
derivation
verification
provenance

A compiler transformation MUST NOT silently discard evidence that is required by the specified semantics.

---

123. Compatibility and Adaptation

Adaptation is especially compatibility-sensitive.

The architecture remains:

adaptation
    |
    v
policy
    |
    v
authorization
    |
    v
capability
    |
    v
effects
    |
    v
resources
    |
    v
provenance
    |
    v
validated state/model/strategy change

A version change MUST NOT turn controlled adaptation into unrestricted self-modifying behavior without an explicit semantic change.

---

124. Version Security

Language-version declarations SHOULD be validated before:

- loading external modules;
- resolving dialects;
- loading plugins;
- executing macros;
- invoking foreign code;
- generating artifacts;
- executing artifacts.

Unknown or malformed versions MUST NOT be used to bypass compatibility or security checks.

---

125. Version Parsing Security

Version parsing MUST be safe against:

- excessive input;
- pathological ranges;
- deeply nested predicates;
- repeated metadata;
- malformed Unicode;
- integer overflow;
- allocation exhaustion;
- parser ambiguity;
- denial-of-service patterns.

The Rust implementation MUST use safe Rust.

Where resource limits are required to protect the compiler, they MUST be implementation/resource policies rather than language-level semantic ceilings.

---

126. Integer Handling

Version components MUST be parsed without unchecked arithmetic assumptions.

The implementation MUST detect overflow or representation failure.

It MUST NOT:

wrap
truncate

a version component silently.

For example, if an implementation cannot represent an enormous version component, it MUST produce a structured diagnostic.

It MUST NOT reinterpret:

very-large-version

as another smaller version.

---

127. Comparison Semantics

Version comparison MUST be deterministic.

The comparison model MUST distinguish:

major
minor
patch
pre-release
build metadata

according to the normative version specification.

Build metadata MUST NOT silently change language semantic precedence if the specification defines it as non-semantic metadata.

Pre-release versions MUST NOT be treated as stable versions without explicit compatibility rules.

---

128. Pre-release Compatibility

Pre-release versions such as:

1.0.0-alpha
1.0.0-beta
1.0.0-rc.1

MUST be explicitly classified.

A stable compiler MUST NOT automatically claim that every pre-release version is compatible with the corresponding stable release.

Pre-release semantics may change before stabilization.

---

129. Build Metadata

Build metadata MAY identify:

- build provenance;
- packaging information;
- CI information;
- implementation identity.

Build metadata MUST NOT silently change language semantics if the normative version model treats it as non-semantic.

If build metadata affects semantics, it must not be represented merely as non-semantic build metadata.

---

130. Version Channels

Named channels such as:

stable
beta
alpha
rc
dev

MAY be supported.

Channels are aliases or policy references, not permanent language versions.

A channel MUST resolve deterministically within the selected toolchain/repository context.

A channel MUST NOT be treated as a physical target.

---

131. Exact Versus Compatible Requirements

The compatibility system MUST distinguish:

exact
minimum
maximum
compatible range

For example:

exactly 1.2.0
minimum 1.2.0
compatible with 1.x
range 1.2.0 .. 2.0.0

These have different semantics.

The implementation MUST NOT silently convert one into another.

---

132. Version Requirements and Resources

The following are different:

requires language >= 1.2.0

and:

requires memory >= required_memory

and:

requires capability("gpu.compute")

The first is version compatibility.

The second is resource compatibility.

The third is capability compatibility.

They MUST remain distinct through AST, semantic analysis, diagnostics, and provenance.

---

133. Version Requirements and Policies

Policies may constrain acceptable versions.

For example, a deployment policy may require:

language version within an approved range

This does not change the language itself.

Policy resolution MUST occur through the policy subsystem.

---

134. Version Requirements and Contracts

A contract MAY assert that a required language/API/dialect version exists.

Such a contract MUST preserve its distinction from ordinary language semantics.

Version requirements MUST NOT be confused with:

preconditions
postconditions
invariants

unless explicitly specified.

---

135. Version Requirements and Capabilities

Capabilities are not versions.

For example:

capability("quantum.measurement")

does not imply:

language version 1.2

A target may implement a capability introduced by a newer backend without requiring a newer language contract.

---

136. Compatibility With Existing "grammar/core/versioning.g4"

The versioning grammar already provides machinery for:

- language declarations;
- version expressions;
- exact versions;
- ranges;
- constraints;
- version references;
- channels;
- attributes;
- compatibility declarations;
- dialect versions;
- package versions;
- API versions;
- ABI versions;
- compiler/runtime requirements;
- grammar requirements.

This file MUST integrate with that grammar rather than reproduce it.

The ownership boundary is:

grammar/core/versioning.g4
    =
what version-related source syntax can be parsed

grammar/specification/language-version.md
    =
what that syntax means

grammar/compatibility/language-version.md
    =
how that meaning participates in compatibility

semantic implementation
    =
whether a requested contract is valid/supported

---

137. No Semantic Actions in Grammar

Version compatibility MUST NOT be implemented through grammar-side semantic actions.

The grammar should remain declarative.

The parser MUST produce structural information.

Semantic resolution belongs downstream.

This preserves:

- deterministic parsing;
- ANTLR/Rust frontend consistency;
- testability;
- portability;
- safe Rust;
- separation of concerns.

---

138. Rust/ANTLR Agreement

The ANTLR grammar and Rust frontend MUST agree on version syntax and semantics.

The conformance suite MUST test both where both implementations exist.

Given the same:

source
language-version context
feature configuration
dialect configuration

both frontend paths MUST produce equivalent version information.

A disagreement is a frontend conformance failure.

---

139. Version Context in AST

Where language version affects interpretation, the compiler's semantic context MUST retain the effective language version.

Conceptually:

CompilationContext {
    language_version
    feature_set
    dialects
    compatibility_mode
    module_environment
    symbol_environment
    type_environment
    effect_environment
    resource_environment
    capability_environment
    policy_environment
    provenance_context
}

The exact Rust structure belongs to the compiler implementation.

The important requirement is that version context be explicit rather than inferred from unrelated state.

---

140. Version Context in Semantic Analysis

Semantic analysis MUST use the effective language version to determine:

- whether a feature exists;
- whether a feature is stable;
- whether a feature is experimental;
- whether syntax is deprecated;
- whether syntax is removed;
- whether a dialect is compatible;
- whether semantics are compatible;
- whether migration is required.

Parsing alone MUST NOT make these decisions.

---

141. Version Context in IR

Canonical IR generation MUST retain sufficient language-version provenance where required.

The IR does not need to reproduce the entire source version declaration.

However, where semantic or artifact compatibility depends on it, the compiler MUST retain a normalized version identity.

This is particularly important for:

- serialized IR;
- cached IR;
- quantum IR;
- cross-tool pipelines;
- long-lived artifacts.

---

142. Version Context in "quantum::ir"

"quantum::ir" MAY carry source-language provenance.

Where required, it SHOULD distinguish:

language_version
quantum_ir_version
dialect_versions
compiler_provenance
semantic_provenance

The quantum IR version MUST remain independently versioned.

---

143. Version Context in HDL

HDL/hardware semantic artifacts SHOULD distinguish:

language_version
hdl_contract_version
hardware_intent_version
target_descriptor_version

A hardware backend update MUST NOT automatically invalidate the source language contract.

---

144. Version Context in Artifacts

An artifact intended for long-term reuse SHOULD carry normalized version metadata.

At minimum, where applicable:

language identity
language version
artifact format
IR versions
dialect versions
compiler provenance
compatibility requirements

The artifact format itself MUST have an independent version.

---

145. Version Context in Provenance

Every compatibility-sensitive transformation SHOULD preserve:

input language version
output language version
transformation identity
reason
tool identity
tool version
timestamp where required
source identity

A provenance timestamp MUST NOT be used as a semantic version.

---

146. Version Context in Diagnostics

Diagnostics SHOULD expose the relevant version context.

For example:

requested language version: 2.0.0
supported language versions: 1.x
feature: <feature>
status: unsupported
migration: available

The diagnostic must distinguish:

unsupported language version

from:

unsupported target

and:

unsupported resource

---

147. Compatibility With "grammar/grammar.md"

"grammar/grammar.md" remains an implementation/conformance reference.

It MUST report version-sensitive implementation state.

For example:

language_version
syntax
grammar_rules
lexer_tokens
AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL
DEPRECATED

However:

grammar.md

MUST NOT decide language semantics merely because a grammar rule exists.

---

148. Compatibility With "grammar/Zamani-Grammar.md"

"grammar/Zamani-Grammar.md" may contain:

- historical syntax;
- proposed syntax;
- experimental syntax;
- aspirational concepts;
- extended design;
- deprecated concepts.

Its contents MUST NOT automatically become compatible language syntax.

Every relevant feature MUST have an explicit status.

---

149. Compatibility With "grammar/compatibility/versions.md"

This file complements:

grammar/compatibility/versions.md

The relationship is:

compatibility/language-version.md
    |
    |-- language-version compatibility contract
    |
    +--> compatibility/versions.md
            |
            +-- broader release/version policy

If the two documents appear to conflict:

1. "grammar/DESIGN.md" controls architecture;
2. "grammar/specification/" controls normative language semantics;
3. "grammar/spec/" controls cross-layer formal contracts;
4. compatibility documents implement those contracts.

Conflicts MUST be resolved explicitly.

No document may silently override another.

---

150. Compatibility With "grammar/spec/versioning.md"

The relationship is:

specification/language-version.md
        |
        v
spec/versioning.md
        |
        v
compatibility/language-version.md

The first defines the language-level model.

The second defines cross-layer implementation/version relationships.

This file applies those rules to compatibility decisions.

---

151. Compatibility With "grammar/spec/compatibility.md"

"grammar/spec/compatibility.md" owns compatibility dimensions and their interpretation.

This file applies those dimensions specifically to language-version evolution.

It MUST NOT duplicate the complete semantic compatibility specification.

---

152. Compatibility With "grammar/core/versioning.g4"

The grammar owns source syntax.

This file owns compatibility semantics.

Therefore this file MUST NOT introduce a new version syntax.

If the syntax changes, the change belongs to:

grammar/specification/language-version.md
grammar/core/versioning.g4
grammar/Zamani.g4
grammar/lexer/

followed by corresponding compatibility analysis.

---

153. Compatibility With Lexer

Lexer changes MUST identify:

old token behavior
new token behavior
affected versions
affected identifiers
migration impact

A new keyword that changes old identifiers from identifiers into keywords may require a breaking language-version transition.

---

154. Compatibility With Parser

Parser changes MUST identify:

- accepted language changes;
- rejected language changes;
- precedence changes;
- associativity changes;
- ambiguity changes;
- source-span changes;
- AST changes.

Parser refactoring without semantic change is not automatically a language-version change.

---

155. Compatibility With AST

AST changes MUST be classified independently.

Possible outcomes include:

source-compatible
AST-incompatible
semantic-compatible

This is legitimate when the AST is internal.

If the AST is a public serialized contract, AST compatibility must be maintained or explicitly versioned.

---

156. Compatibility With Semantic Analysis

Semantic analysis is the decisive stage for determining whether a versioned source construct is valid under the requested language contract.

Semantic analysis MUST validate:

- version support;
- feature status;
- type behavior;
- effect behavior;
- capability behavior;
- resource behavior;
- policy behavior;
- contract behavior;
- dialect compatibility.

---

157. Compatibility With Compilation

Compilation MUST preserve language-version semantics while allowing implementation-specific realization.

A compiler MAY:

- specialize;
- optimize;
- vectorize;
- distribute;
- parallelize;
- route;
- schedule;
- lower;
- simulate;
- target accelerators;
- target quantum systems.

These transformations MUST preserve specified semantics.

---

158. Compatibility With Runtime

Runtime compatibility MUST distinguish:

language semantics

from:

runtime implementation

A runtime update may be compatible even when its implementation changes completely.

If runtime behavior visible to the language changes, compatibility analysis is required.

---

159. Compatibility With Target Realization

Target realization MUST occur after language-version resolution and semantic validation.

The target cannot choose the language semantics.

The target can determine:

can this semantic program be realized?

It cannot redefine:

what this source program means

---

160. Compatibility and Hardware Scaling

The same language contract MUST be usable across:

tiny embedded targets
CPU
multicore CPU
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC
cluster
distributed system
cloud
future target

provided the target can satisfy the required semantics.

No language version may be created merely to distinguish target size.

---

161. Compatibility and Infinite Scalability

Zamani MUST NOT claim that a finite implementation literally possesses infinite memory, infinite compute, or infinite physical resources.

Instead:

«The language-version architecture MUST remain open-ended and MUST NOT impose arbitrary source-level ceilings on computation scale.»

Thus a program may scale from very small to arbitrarily large instances as supported by:

- representation;
- compiler;
- runtime;
- target;
- resources;
- physical feasibility.

This is the technically correct interpretation of "atom to everywhere."

---

162. Version Evolution Must Preserve Scale Independence

A language-version change MUST NOT convert a target-specific limitation into a language-wide limitation.

For example, if a compiler currently cannot compile a very large tensor, that does not justify changing the language contract to impose a smaller tensor maximum.

Similarly:

target has N qubits

does not imply:

language supports only N qubits

---

163. Compatibility and Resource Negotiation

The architecture is:

source intent
      |
      v
requirements
      |
      v
capabilities
      |
      v
resource negotiation
      |
      v
target realization

Versioning applies to the meaning of each contract.

Resource negotiation itself MUST remain independent from language-version numbering.

---

164. Compatibility and Capability Negotiation

Capabilities may evolve independently.

A new capability can generally be introduced without changing the language version if:

- its name is compatible;
- its semantic contract is stable;
- it does not reinterpret existing capabilities;
- its absence remains diagnosable.

Changing the meaning of an existing stable capability is compatibility-sensitive.

---

165. Compatibility and Policies

Policies may reject a version even when the language itself supports it.

For example:

language version supported
policy rejects version

is a policy failure, not a language semantic failure.

Diagnostics MUST preserve that distinction.

---

166. Compatibility and Sandbox

Sandbox behavior may constrain:

- FFI;
- native calls;
- filesystem;
- network;
- reflection;
- adaptation;
- code generation;
- resource use.

Changing sandbox implementation does not automatically change the language version.

Changing specified sandbox semantics does.

---

167. Compatibility and Reflection

Reflection and metaprogramming MUST be version-aware.

A reflective operation MUST NOT assume that every future language-version construct exists in the current compiler.

Unknown versioned constructs MUST be represented safely or rejected explicitly.

---

168. Compatibility and Generated Code

Generated code MUST NOT silently target a different language contract from the consuming program.

If a generator produces a newer language construct, it MUST either:

- declare the newer required version;
- generate compatible older syntax;
- or report an incompatibility.

---

169. Compatibility and Package Manifests

Project/package configuration may declare:

package version
language version
dependency version ranges
dialect versions

These values MUST remain distinct.

The project configuration subsystem MUST NOT interpret package version as language version.

---

170. Compatibility and "Zamani.toml"

The repository's package configuration currently exposes an "edition" field.

That field MUST NOT silently become a second language-version authority.

The project configuration and language-version systems MUST define a deterministic relationship.

If "edition" is intended to represent a Zamani language edition, its mapping MUST be specified explicitly.

Otherwise:

package edition

MUST remain distinct from:

language version

This relationship should be resolved in the package/configuration specification rather than inferred from implementation code.

---

171. Compatibility and CLI

The CLI MAY expose commands such as:

zamani --version

and version-related validation/migration commands.

CLI output MUST distinguish:

compiler version
language versions supported
default language version
runtime version

A compiler's own version output MUST NOT be presented as the language version.

---

172. Compatibility and CI

Production CI SHOULD validate at least:

cargo check --all-targets
cargo test --all-targets
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings

subject to the repository's actual supported feature configuration.

CI MUST validate the declared Rust baseline.

The repository's Cargo manifest remains the source of truth for the Rust implementation version.

---

173. Safe Rust CI Enforcement

Production CI SHOULD enforce the no-"unsafe" requirement through appropriate repository checks.

The requirement applies to Zamani-owned Rust implementation code.

Dependencies are governed by dependency policy and audit requirements.

The language specification MUST NOT require Rust "unsafe".

---

174. Compatibility Regression Tests

Every language-version release MUST run regression tests against previously supported compatible versions.

At minimum:

old stable source
new compiler

must be tested.

Where multiple implementations exist:

ANTLR frontend
Rust frontend

must be compared.

---

175. Cross-Domain Version Tests

The version conformance suite MUST include representative programs combining:

classical + quantum
classical + HDL
AI + classical
AI + quantum
AI + reasoning
learning + adaptation
contracts + policies
effects + capabilities
resources + quantum
distributed + concurrency
networking + security
FFI + ABI
simulation + execution

The purpose is to ensure that versioning remains a single language-wide contract.

---

176. POCO-REAF Integration Test

At least one end-to-end test SHOULD verify:

Zamani source
      |
      v
language-version resolution
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
structural validation
      |
      v
type/effect validation
      |
      v
resource/capability validation
      |
      v
contract/policy validation
      |
      v
provenance
      |
      v
canonical semantic model
      |
      +------------------+
      |                  |
      v                  v
classical IR        quantum::ir
      |                  |
      +--------+---------+
               |
               v
optimization
               |
               v
lowering
               |
               v
routing/scheduling
               |
               v
resilience/QEC/ZQN
               |
               v
HAL
               |
               v
target

The same source semantics MUST remain intact when the target changes.

---

177. Compatibility Test Matrix

The production compatibility matrix SHOULD contain at least:

Transition| Expected classification
Patch → patch| compatible
Minor → newer minor| compatible when no breaking semantics
Major → major| compatibility must be explicitly established
Stable → experimental| not automatically compatible
Stable → deprecated| source may remain compatible
Stable → removed| breaking
Language → compiler| independent
Language → runtime| independent unless observable semantics change
Language → target| independent
Language → quantum IR| independently versioned
Language → dialect| independently versioned
Language → ABI| independently versioned
Language → package| independently versioned

---

178. Version Compatibility Levels

Implementations MAY expose a compatibility level.

A useful scale is:

C0  no compatibility established
C1  lexical compatibility
C2  source/syntax compatibility
C3  AST compatibility
C4  semantic compatibility
C5  canonical IR compatibility
C6  artifact compatibility
C7  runtime compatibility
C8  target/execution compatibility
C9  complete declared compatibility

A higher level MUST NOT be inferred merely from lower-level representation equality.

For example:

C4 semantic compatibility

does not imply:

C8 target compatibility

because the target may lack resources or capabilities.

---

179. Semantic Compatibility Is the Primary Compatibility Goal

For POCO-REAF, semantic compatibility is more important than implementation identity.

The key equation is:

same source
+
same language semantic contract
=
same intended computation

Target realization may differ.

The source meaning must not.

---

180. Compatibility and Optimization Equivalence

Two compiled programs may differ substantially while being compatible if:

Semantics(A) == Semantics(B)

under the applicable language contract.

This applies to:

- scalar optimization;
- tensor optimization;
- parallelization;
- quantum optimization;
- gate fusion;
- routing;
- scheduling;
- hardware specialization.

---

181. Compatibility and Approximation

Approximation MUST NOT silently replace exact semantics.

If a compiler or runtime provides an approximation mode, that mode MUST be explicitly specified.

It SHOULD carry provenance describing:

requested semantics
approximation mode
error/quality contract
target reason

A target limitation alone MUST NOT silently authorize semantic approximation.

---

182. Compatibility and Numerical Semantics

Changes to:

- overflow;
- underflow;
- rounding;
- floating-point behavior;
- integer semantics;
- precision;
- NaN handling;
- numerical determinism;

may be semantic compatibility changes.

Backend-specific numerical behavior MUST remain within the language's declared numerical contract.

---

183. Compatibility and Quantum Measurement

Quantum measurement semantics are compatibility-critical.

A version change MUST explicitly classify any change involving:

- measurement probability;
- collapse/state update semantics;
- result representation;
- classical feed-forward;
- repeated measurement;
- dynamic control;
- measurement ordering.

Backend-specific implementation must preserve specified semantics.

---

184. Compatibility and Hardware Timing

Hardware timing may be target-specific.

A change in:

clock frequency
latency
gate duration
pipeline depth

does not automatically change the language version.

If timing guarantees are part of source semantics, changes to those guarantees become compatibility-sensitive.

---

185. Compatibility and Memory

Memory capacity is a resource property.

Language-version compatibility MUST distinguish:

semantic memory model

from:

available memory

A target may fail because it lacks sufficient memory.

The language MUST NOT acquire a smaller semantic memory model merely because one target has limited memory.

---

186. Compatibility and Network Topology

Network topology is a target/deployment property unless explicitly part of source semantics.

A program's language version MUST NOT depend on:

- node count;
- IP addresses;
- physical topology;
- network interface count.

Explicit topology requirements belong to the resource/capability/deployment model.

---

187. Compatibility and Device Count

The language-version system MUST NOT encode a fixed number of:

- devices;
- accelerators;
- QPUs;
- CPUs;
- GPUs;
- FPGAs;
- nodes.

Device count is a resource/target concern.

---

188. Compatibility and Tensor Scaling

Tensor shape that is algorithmically meaningful is semantic.

Hardware tensor capacity is target-specific.

Therefore:

tensor shape

MAY be part of language semantics.

But:

maximum accelerator tensor size

MUST NOT become a language-version limit.

---

189. Compatibility and Program Size

The language-version system MUST NOT impose arbitrary limits on:

- program size;
- function count;
- module count;
- declaration count;
- expression count;
- version predicate count.

The implementation MAY exhaust resources.

Such exhaustion MUST be reported as an implementation/resource failure.

---

190. Compatibility and Parser Depth

Parser implementations may have finite stack or recursion resources.

Those are implementation constraints.

They MUST NOT be represented as semantic language-version restrictions unless technically unavoidable and explicitly specified.

Implementations SHOULD prefer safe techniques that avoid accidental stack exhaustion.

---

191. Compatibility and Memory Safety

The Rust implementation MUST preserve memory safety through safe Rust.

Language-version parsing MUST NOT depend on undefined behavior.

Malformed version input MUST produce controlled diagnostics rather than memory unsafety.

---

192. Compatibility and Error Recovery

Error recovery may differ between parser implementations.

Such differences are compatible only when they do not change accepted valid source or semantic interpretation.

Diagnostics may improve across patch versions without changing the language version.

---

193. Compatibility and Diagnostics

Diagnostic wording may change without being a language-version change.

However, diagnostic classification must remain stable enough for tooling where it is a public contract.

Machine-readable diagnostics SHOULD use stable error codes rather than relying on exact prose matching.

---

194. Compatibility and Documentation

Documentation changes are not automatically language-version changes.

However, documentation MUST accurately state:

language version
feature status
implementation status
compatibility status

Documentation MUST NOT silently promote:

proposed

to:

stable

---

195. Compatibility and Historical Material

Historical grammar material MUST remain explicitly marked.

Historical material may describe:

- old syntax;
- abandoned proposals;
- previous version semantics;
- obsolete implementation approaches.

Historical documentation MUST NOT be interpreted as current syntax.

---

196. Compatibility and Experimental Domains

Experimental domains may evolve independently while using the common language-version contract.

Examples include:

- new accelerator models;
- new quantum features;
- new HDL capabilities;
- new AI computation;
- new data abstractions;
- new distributed mechanisms.

Their experimental status MUST remain explicit.

---

197. Version Compatibility and Application Libraries

Application-specific functionality SHOULD generally remain outside core language-version semantics.

For example, application functionality involving:

- vision;
- robotics;
- payments;
- legal workflows;
- administration;
- domain-specific analysis;

should normally be implemented through:

libraries
dialects
capabilities
policies
services
applications

Adding such a library MUST NOT create a new core language version.

---

198. Version Compatibility and Standard Libraries

A standard-library release MAY have its own version.

The language version and standard-library version MUST remain distinguishable.

A library MAY require:

language >= X

without making:

library version == language version

---

199. Version Compatibility and Package Resolution

Dependency resolution MUST evaluate language compatibility explicitly.

A dependency requiring:

language >= 1.2.0 < 2.0.0

MUST NOT be satisfied by a compiler using incompatible semantics merely because its compiler version is numerically newer.

---

200. Version Compatibility and Workspace Builds

A workspace containing multiple packages MAY contain different compatible language-version requirements.

The package/tooling layer MUST determine whether those requirements have a valid common resolution.

If no valid resolution exists, the build MUST fail explicitly.

---

201. Version Compatibility and Modules

Module declarations MAY carry version requirements.

Module compatibility MUST remain distinct from language syntax.

The module system owns module dependency semantics.

Language-version compatibility determines whether the source language contract required by the module is supported.

---

202. Version Compatibility and External Dependencies

External dependencies MAY require:

- language versions;
- API versions;
- ABI versions;
- runtime versions;
- dialect versions.

These requirements MUST remain separately identifiable.

---

203. Version Compatibility and Dialect Registration

Dialect registration MUST provide:

dialect name
dialect version
required language range
provided syntax
provided semantics
AST mapping
IR mapping
compatibility rules

A dialect MUST NOT silently modify the core language version.

---

204. Version Compatibility and Vendor Extensions

Vendor extensions MAY be supported through dialects or capability contracts.

Vendor extensions MUST NOT become mandatory core language semantics unless formally adopted into the language specification.

This preserves portability.

---

205. Version Compatibility and Future Hardware

Future hardware MUST be able to implement existing language semantics without requiring old programs to be rewritten merely because hardware changes.

The compiler may specialize the program according to:

target capabilities
resource availability
topology
performance
power
thermal limits
reliability

while preserving source semantics.

---

206. Version Compatibility and Future Language Evolution

Future versions SHOULD preserve source semantics wherever practical.

When preservation is impossible:

breaking change
        |
        v
major version
        |
        v
diagnostic
        |
        v
migration
        |
        v
new semantics

must be explicit.

---

207. Compatibility Review Checklist

Every proposed language-version change MUST answer:

Authority

- Which specification owns the change?
- Which file owns the grammar?
- Which file owns the AST?
- Which subsystem owns semantics?

Syntax

- Does tokenization change?
- Does parsing change?
- Does precedence change?
- Does an identifier become a keyword?
- Does ambiguity change?

Semantics

- Does meaning change?
- Does type behavior change?
- Does effect behavior change?
- Does resource behavior change?
- Does capability behavior change?
- Does policy behavior change?

Domains

- Classical?
- Quantum?
- "quantum::ir"?
- HDL?
- Hardware?
- Hybrid?
- AI/data?
- Distributed?
- Networking?
- Security?
- Interoperability?

Compatibility

- Is the change breaking?
- What version contains it?
- What old versions remain supported?
- Is migration possible?
- Is deprecation required?

Tooling

- Lexer?
- Parser?
- AST?
- Formatter?
- LSP?
- Compiler?
- Package tooling?
- Migration tooling?

Testing

- Positive?
- Negative?
- Boundary?
- Scalability?
- Determinism?
- Cross-domain?
- Compatibility?
- Diagnostics?

Safety

- Does Rust remain safe?
- Does any implementation path introduce "unsafe"?
- Are malformed versions handled safely?

---

208. File Completion Contract

This file is complete only when:

1. its ownership is unambiguous;
2. its relationship to "grammar/specification/language-version.md" is explicit;
3. its relationship to "grammar/spec/versioning.md" is explicit;
4. its relationship to "grammar/spec/compatibility.md" is explicit;
5. its relationship to "grammar/compatibility/versions.md" is explicit;
6. its relationship to "grammar/core/versioning.g4" is explicit;
7. its relationship to lexer/parser/AST is explicit;
8. its relationship to semantic analysis is explicit;
9. its relationship to canonical IR is explicit;
10. its relationship to "quantum::ir" is explicit;
11. its relationship to dialects is explicit;
12. its relationship to packages is explicit;
13. its relationship to artifacts is explicit;
14. its relationship to runtime/targets is explicit;
15. compatibility classes are defined;
16. version-resolution requirements are defined;
17. migration requirements are defined;
18. deprecation requirements are defined;
19. scalability requirements are defined;
20. no artificial capacity ceiling is introduced;
21. Rust safety requirements are defined;
22. diagnostic requirements are defined;
23. conformance requirements are defined;
24. POCO-REAF requirements are defined;
25. future target/domain integration is defined.

---

209. Dependency Contract

This file has the following logical dependency contract.

DEPENDS_ON:
    grammar/DESIGN.md
    grammar/specification/language-version.md
    grammar/spec/versioning.md
    grammar/spec/compatibility.md
    grammar/compatibility/versions.md
    grammar/compatibility/migrations.md
    grammar/compatibility/deprecated.md
    grammar/core/versioning.g4
    grammar/lexer/
    grammar/Zamani.g4
    frontend AST contract
    semantic contract
    canonical IR contracts

EXPORTS:
    language-version compatibility rules
    version compatibility classification
    version propagation rules
    compatibility review requirements
    migration requirements
    artifact version requirements
    POCO-REAF versioning requirements

AST_OWNER:
    frontend AST subsystem

SEMANTIC_OWNER:
    grammar/specification/ and semantic implementation

TYPE_OWNER:
    type-system specification and implementation

EFFECT_OWNER:
    effects specification and implementation

RESOURCE_OWNER:
    resources specification and implementation

CAPABILITY_OWNER:
    capability/resource subsystem

POLICY_OWNER:
    policies/security/execution subsystem

PROVENANCE_OWNER:
    provenance subsystem

IR_OWNER:
    canonical IR owners
    quantum::ir for quantum semantics

SPEC_OWNER:
    grammar/specification/language-version.md

COMPATIBILITY_OWNER:
    grammar/compatibility/

TEST_OWNER:
    grammar/tests/
    compatibility conformance suites

INTEGRATION:
    source
      -> version resolution
      -> lexer/parser
      -> AST
      -> semantic analysis
      -> canonical representation
      -> IR
      -> compiler
      -> runtime
      -> target

COMPLETION_CRITERIA:
    All normative version transitions have explicit compatibility
    classification, tests, diagnostics, migration policy where required,
    and cross-layer integration evidence.

---

210. Required Invariants

The following invariants are mandatory.

Invariant 1

A language version identifies language semantics, not hardware.

Invariant 2

A compiler version is not a language version.

Invariant 3

A Rust version is not a language version.

Invariant 4

A target version is not a language version.

Invariant 5

An IR version is not automatically a language version.

Invariant 6

A dialect version is not automatically a language version.

Invariant 7

A package version is not automatically a language version.

Invariant 8

Parser acceptance does not establish semantic compatibility.

Invariant 9

Grammar presence does not establish implementation completeness.

Invariant 10

Target infeasibility does not automatically establish source incompatibility.

Invariant 11

Resource limits must not become language limits.

Invariant 12

Capability absence must not be reported as syntax failure.

Invariant 13

Unknown future language semantics must not be silently interpreted as older semantics.

Invariant 14

Removed syntax must not be silently reinterpreted.

Invariant 15

Stable semantic changes require explicit compatibility classification.

Invariant 16

Migration must preserve semantics where migration is claimed to be safe.

Invariant 17

"quantum::ir" remains the canonical quantum semantic boundary.

Invariant 18

Quantum hardware mapping remains downstream from language semantics.

Invariant 19

HDL intent remains distinct from physical hardware realization.

Invariant 20

Application libraries and dialects must not silently redefine core language-version semantics.

Invariant 21

Version resolution must be deterministic.

Invariant 22

Effective language version must be explicit compilation-context state.

Invariant 23

Version information required for compatibility must survive into artifacts/provenance.

Invariant 24

Production Rust implementation must use safe Rust.

Invariant 25

No artificial universal hardware or computation ceilings may be introduced by versioning.

Invariant 26

A compatibility claim requires implementation/conformance evidence.

Invariant 27

A compatible compiler must preserve specified source meaning.

Invariant 28

A target may realize the same source semantics differently.

Invariant 29

A future compiler must not silently reinterpret unsupported future language semantics.

Invariant 30

POCO-REAF is achieved through semantic stability and target-independent realization, not through fixed hardware assumptions.

---

211. Production Readiness Gates

This file and its implementation are production-ready only when all applicable gates pass.

Gate A — Authority

- Language-version semantics have one normative owner.
- Compatibility semantics have one compatibility owner.
- Grammar has one canonical composition root.
- No competing version grammar exists.

Gate B — Resolution

- Every compilation has a deterministic effective language version.
- Explicit and inherited version sources are resolved consistently.
- Unsupported versions are rejected.
- Future unknown versions are not silently accepted.

Gate C — Frontend

- Lexer behavior is version-aware where required.
- Parser behavior is version-aware where required.
- AST preserves version-relevant semantics.
- ANTLR and Rust frontend behavior agrees.

Gate D — Semantics

- Versioned feature status is checked semantically.
- Type compatibility is checked.
- Effect compatibility is checked.
- Resource/capability compatibility is checked.
- Contract/policy compatibility is checked.
- Domain semantics remain version-aware.

Gate E — IR

- Canonical IR boundaries are explicit.
- "quantum::ir" remains canonical for quantum semantics.
- IR version is independently tracked.
- No semantic information is silently lost.

Gate F — Artifacts

- Artifacts carry required version metadata.
- Artifact compatibility is checked.
- Migration is available where required.

Gate G — Testing

- Positive tests pass.
- Negative tests pass.
- Boundary tests pass.
- Scalability tests pass.
- Determinism tests pass.
- Cross-domain tests pass.
- Compatibility tests pass.
- Diagnostics are verified.

Gate H — Safety

- Rust 1.97+ baseline is verified.
- Rust 2021 edition is verified.
- Production Rust contains no "unsafe".
- Version parsing is overflow-safe.
- Malformed input is safely rejected.
- Resource exhaustion is handled predictably.

Gate I — POCO-REAF

- Source semantics remain target-independent.
- Target selection occurs after semantic resolution.
- Hardware capacity does not define language compatibility.
- Target-specific failures remain target/resource/capability diagnostics.
- Existing compatible source survives compiler evolution.

---

212. Recommended Implementation Order

Implementation should proceed in dependency order.

Phase 1 — Normative version model

Complete and reconcile:

grammar/specification/language-version.md
grammar/spec/versioning.md
grammar/spec/compatibility.md

Resolve all conflicts before modifying dependent implementation.

Phase 2 — Version grammar

Complete:

grammar/core/versioning.g4

and integrate it with:

grammar/Zamani.g4

Phase 3 — Lexer

Complete version-related tokens and contextual keyword behavior under:

grammar/lexer/
grammar/antlr/
src/lexer.rs

Phase 4 — AST

Define normalized version AST representation.

Phase 5 — Semantic resolution

Implement:

version parsing
version normalization
version comparison
version range evaluation
feature gating
compatibility classification

using safe Rust.

Phase 6 — Compiler context

Propagate:

effective_language_version
feature_set
dialect_versions
compatibility_mode

through the semantic pipeline.

Phase 7 — Compatibility

Integrate:

grammar/compatibility/versions.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md
grammar/compatibility/compatibility-matrix.md

Phase 8 — IR/artifacts

Propagate version identity where required into:

canonical IR
quantum::ir
artifacts
provenance
caches

Phase 9 — Tooling

Integrate:

CLI
formatter
LSP
migration tooling
package tooling
documentation generation

Phase 10 — Conformance

Run the complete version compatibility matrix across:

lexer
parser
AST
semantic analysis
IR
quantum::ir
artifacts
runtime
targets
dialects

---

213. What This File Must Never Do

This file MUST NOT:

- define a competing grammar;
- define target hardware;
- enumerate every future device;
- impose fixed machine capacities;
- define quantum gate lists;
- define QEC algorithms;
- define scheduling algorithms;
- define routing algorithms;
- define runtime implementation;
- define backend instruction selection;
- define physical calibration;
- define application-specific language keywords;
- define library versions as language versions;
- define compiler versions as language versions;
- use Rust implementation details as source semantics;
- use target limitations as language limitations.

---

214. Final Compatibility Model

The complete version architecture is:

                    ZAMANI LANGUAGE VERSION
                              |
          +-------------------+-------------------+
          |                   |                   |
       Lexical             Syntax             Semantics
          |                   |                   |
          +-------------------+-------------------+
                              |
                              v
                             AST
                              |
             +----------------+----------------+
             |                |                |
            Types           Effects       Requirements
             |                |                |
             +----------------+----------------+
                              |
                  +-----------+-----------+
                  |                       |
             Capabilities             Resources
                  |                       |
                  +-----------+-----------+
                              |
                           Policies
                              |
                          Contracts
                              |
                         Provenance
                              |
                              v
                    Canonical Semantics
                              |
              +---------------+---------------+
              |               |               |
         Classical        Quantum           HDL
             IR          quantum::ir       intent
              |               |               |
              +---------------+---------------+
                              |
                        Optimization
                              |
                         Specialization
                              |
                          Lowering
                              |
                    Routing / Scheduling
                              |
                   Resilience / Recovery
                              |
                         QEC / ZQN
                              |
                             HAL
                              |
                    Target Realization
                              |
          +-----------+-------+-------+-----------+
          |           |       |       |           |
         CPU         GPU    FPGA    ASIC        QPU
          |           |       |       |           |
          +-----------+-------+-------+-----------+
                              |
                   Distributed / HPC / Cloud
                              |
                       Future Targets

The language version remains at the top.

Target realization remains at the bottom.

No target may redefine the language contract.

---

215. Final POCO-REAF Invariant

The definitive invariant is:

Same Zamani source
        +
Same language-version contract
        +
Same semantic inputs
        =
Same specified program meaning

while:

target
backend
hardware
topology
resource availability
optimization
routing
scheduling
runtime

may change the realization.

Therefore:

PROGRAM
   |
   v
LANGUAGE CONTRACT
   |
   v
SEMANTIC IDENTITY
   |
   v
CANONICAL REPRESENTATION
   |
   v
TARGET-INDEPENDENT COMPILATION
   |
   v
TARGET CAPABILITY NEGOTIATION
   |
   v
SPECIALIZATION
   |
   v
LOWERING
   |
   v
REALIZATION

This is the compatibility foundation required for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.

The language can evolve.

The compiler can evolve.

The IR can evolve.

The runtime can evolve.

The hardware can evolve.

Quantum processors can evolve.

Classical processors can evolve.

HDL targets can evolve.

Accelerators can evolve.

Distributed infrastructure can evolve.

New computational domains can evolve.

But whenever compatibility is promised, the specified meaning of existing Zamani source MUST remain stable.

That is the fundamental requirement that allows Zamani to scale from the smallest supported computation to arbitrarily large computations without turning today's finite implementation or hardware characteristics into tomorrow's permanent language restrictions.

---

216. Final Definition of Done

"grammar/compatibility/language-version.md" is DONE when:

[✓] language-version ownership is explicit
[✓] compatibility ownership is explicit
[✓] version syntax is delegated to the canonical grammar
[✓] version semantics are delegated to the normative specification
[✓] compiler versions are separated from language versions
[✓] Rust versions are separated from language versions
[✓] dialect versions are separated from language versions
[✓] package versions are separated from language versions
[✓] API/ABI versions are separated from language versions
[✓] IR versions are separated from language versions
[✓] quantum::ir is explicitly integrated
[✓] classical IR is explicitly integrated
[✓] HDL/hardware semantics are explicitly integrated
[✓] AI/data semantics are explicitly integrated
[✓] resource compatibility is separated from version compatibility
[✓] capability compatibility is separated from version compatibility
[✓] target compatibility is separated from source compatibility
[✓] deterministic version resolution is required
[✓] forward compatibility is explicitly controlled
[✓] backward compatibility is explicitly controlled
[✓] experimental features are controlled
[✓] deprecation is controlled
[✓] removal is controlled
[✓] migration is controlled
[✓] artifact compatibility is controlled
[✓] provenance is integrated
[✓] diagnostics are integrated
[✓] tooling is integrated
[✓] scalability is explicitly protected
[✓] artificial capacity ceilings are prohibited
[✓] quantum compatibility is protected
[✓] HDL/hardware portability is protected
[✓] POCO-REAF is explicitly defined
[✓] Rust 1.97+ is addressed
[✓] Rust 2021 is addressed
[✓] production Rust safety is addressed
[✓] no competing grammar authority is created
[✓] independent file completion contract is defined
[✓] dependency/integration contract is defined
[✓] production-readiness gates are defined

The resulting rule is simple:

«Version the language contract, not the machine.»

«Preserve semantics, not implementation details.»

«Negotiate resources and capabilities instead of hard-coding capacity.»

«Version every independent public boundary independently.»

«Never silently reinterpret existing source.»

«Never let finite hardware become a permanent language ceiling.»

«Never allow parser acceptance to masquerade as semantic implementation.»

«Never allow implementation convenience to redefine the language.»

«Preserve enough provenance and compatibility metadata that long-lived programs and artifacts remain understandable and migratable as Zamani, its compiler, its IRs, runtimes, and hardware continue to evolve.»