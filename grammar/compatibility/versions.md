Zamani Language Version and Compatibility Policy

Path: "grammar/compatibility/versions.md"
Status: Normative
Scope: Zamani source-language versions, grammar compatibility, semantic compatibility, compiler compatibility, IR compatibility, target compatibility, dialect compatibility, migration, deprecation, and POCO-REAF portability
Language: Zamani
Compiler baseline: Rust 1.97.0 minimum; Rust 1.97.1 production validation
Safety: Safe Rust only; Rust "unsafe" is prohibited
Primary objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the normative versioning and compatibility contract for Zamani.

It establishes how Zamani evolves while preserving:

- source compatibility where promised;
- semantic stability;
- deterministic parsing;
- AST compatibility;
- compiler compatibility;
- canonical IR boundaries;
- quantum semantic compatibility;
- hardware independence;
- target portability;
- dialect isolation;
- reproducible language evolution;
- long-term POCO-REAF portability.

This document applies to all language syntax represented by:

- "grammar/Zamani.g4";
- the canonical language specification;
- "grammar/grammar.md";
- "grammar/Zamani-Grammar.md";
- modular grammar files under "grammar/";
- lexer implementation;
- parser implementation;
- AST implementation;
- semantic analysis;
- IR generation;
- canonical quantum IR;
- compiler tooling;
- examples;
- conformance tests;
- compatibility tests.

This file does not define runtime behavior, target-specific hardware behavior, scheduling policy, or backend implementation.

---

2. Compatibility Is Layered

Zamani compatibility is not one property.

The following layers MUST be considered independently:

Source Compatibility
        ↓
Lexical Compatibility
        ↓
Syntactic Compatibility
        ↓
AST Compatibility
        ↓
Name/Module Compatibility
        ↓
Type Compatibility
        ↓
Effect Compatibility
        ↓
Capability/Resource Compatibility
        ↓
Semantic Compatibility
        ↓
IR Compatibility
        ↓
Target Compatibility
        ↓
Runtime Compatibility

A change MUST identify which layer it affects.

A program can therefore be:

- syntactically compatible but semantically incompatible;
- semantically compatible but unavailable on a particular target;
- source-compatible but not ABI-compatible;
- parser-compatible but not runtime-compatible;
- language-compatible but resource-infeasible.

These distinctions MUST NOT be collapsed.

---

3. Core Compatibility Principle

The fundamental rule is:

«A Zamani language-version change MUST NOT silently change the meaning of valid existing source code.»

When a breaking semantic change is unavoidable, it MUST be:

1. explicitly versioned;
2. documented;
3. detectable;
4. diagnosable;
5. migratable where practical;
6. covered by compatibility tests.

A compiler MUST NOT silently reinterpret old source according to new semantics.

---

4. Language Version Is Independent of Rust Version

Zamani language versioning MUST NOT be coupled to the Rust compiler version.

Rust is an implementation dependency.

Zamani is the language.

Therefore:

Rust 1.97.0
Rust 1.97.1

do not constitute different Zamani language versions.

The production baseline is:

Minimum supported Rust: 1.97.0
Production validation:  Rust 1.97.1

The Cargo manifest MUST express a valid Rust version constraint.

It MUST NOT contain prose such as:

rust-version = "1.97" or "1.97.1"

The exact manifest representation MUST use valid Cargo syntax.

For the current production baseline, the intended minimum is:

rust-version = "1.97"

or another valid Cargo-compatible representation that establishes Rust 1.97.x as the minimum supported series.

CI MUST explicitly validate the supported minimum and production patch release.

---

5. Zamani Language Version Format

Zamani language versions use Semantic Versioning principles:

MAJOR.MINOR.PATCH

For example:

1.0.0
1.1.0
1.1.1
2.0.0

The meaning is:

MAJOR

Increment MAJOR when an incompatible language change is intentionally introduced.

Examples:

- changing the meaning of existing valid syntax;
- removing stable syntax;
- changing stable type semantics incompatibly;
- changing stable ownership/resource semantics incompatibly;
- changing canonical semantic interpretation incompatibly;
- changing module resolution incompatibly;
- changing stable quantum operation semantics incompatibly.

MINOR

Increment MINOR when functionality is added in a backward-compatible manner.

Examples:

- new syntax that cannot reinterpret existing valid programs;
- new optional language capabilities;
- new standard attributes;
- new extensible operation forms;
- new domain constructs that do not conflict with existing syntax;
- new target-independent resource expressions.

PATCH

Increment PATCH for compatible corrections.

Examples:

- diagnostic improvements;
- documentation corrections;
- grammar clarifications that do not alter accepted semantics;
- parser bug fixes;
- conformance corrections;
- implementation fixes preserving specified behavior.

A patch release MUST NOT introduce a new meaning for existing valid source.

---

6. Language Version Must Be Explicit

Every compilable Zamani program MUST have a determinable language version.

Version resolution MUST be deterministic.

The compiler MAY obtain the version from:

1. an explicit source declaration;
2. a package/project manifest;
3. a workspace declaration;
4. a compiler-selected default for legacy source.

The resolution precedence MUST be specified by the language implementation.

An explicit source version MUST override an implicit default when the language defines source-level version declarations.

The compiler MUST report the effective language version in diagnostics and machine-readable compilation metadata where tooling supports it.

---

7. Version Declaration

The final concrete syntax for language-version declarations MUST be defined by the canonical grammar.

Conceptually, Zamani supports a declaration equivalent to:

language "1.0";

or:

version "1.0";

The exact spelling MUST have exactly one canonical definition.

The following MUST NOT become competing syntax:

language "1.0";
version "1.0";
@version("1.0")
pragma version 1.0

unless the language specification explicitly defines each as an intentional compatibility form.

If multiple historical forms exist, they MUST have a documented normalization rule.

---

8. Version Resolution Contract

The compiler MUST resolve the language version before performing version-sensitive parsing or semantic interpretation.

The conceptual sequence is:

Source
  ↓
Source identity
  ↓
Version discovery
  ↓
Version validation
  ↓
Version-specific lexical policy
  ↓
Version-specific parsing
  ↓
AST
  ↓
Semantic analysis
  ↓
IR

A version MUST NOT be inferred from arbitrary syntax after semantic interpretation has already occurred.

---

9. Version Ranges

Where packages, modules, dependencies, or tools express compatibility ranges, ranges MUST describe supported language versions rather than hardware versions.

For example:

>=1.0,<2.0

means that the consumer accepts the corresponding language-version family.

It does not mean:

- CPU generation;
- GPU generation;
- QPU generation;
- operating-system version;
- hardware topology;
- compiler optimization level.

---

10. Language Version and Compiler Version

A Zamani compiler has at least two independent versions:

Language Version
Compiler Version

Example:

Language: 1.2.0
Compiler: 0.8.3

A compiler MAY support multiple Zamani language versions.

A compiler MUST report both independently.

Compiler upgrades MUST NOT automatically imply a language-version migration.

---

11. Compiler Compatibility

A compiler MUST explicitly declare which Zamani language versions it supports.

Conceptually:

Compiler
 ├── supports Zamani 1.0
 ├── supports Zamani 1.1
 └── supports Zamani 1.2

Unsupported versions MUST produce a structured diagnostic.

The compiler MUST NOT silently compile an unsupported language version as if it were another version.

---

12. Forward Compatibility

A compiler MUST NOT claim to support future language versions merely because their syntax happens to parse.

For example, a compiler supporting:

Zamani 1.x

MUST NOT silently accept:

Zamani 2.x

and interpret unknown constructs using 1.x semantics.

Unknown future constructs MUST produce an explicit diagnostic unless they are covered by a deliberately defined extension mechanism.

---

13. Backward Compatibility

Within a stable major version, valid source SHOULD remain valid.

A compatible compiler upgrade MUST preserve the meaning of stable source.

For example:

Zamani 1.0 source

should remain usable by a compiler supporting:

Zamani 1.1

unless the feature is explicitly classified as:

- experimental;
- implementation-defined;
- deprecated;
- unstable;
- opt-in;
- target-dependent.

Such exceptions MUST be documented.

---

14. Experimental Features

Experimental language features MUST NOT silently become stable semantics.

An experimental feature MUST have:

- a unique identity;
- an owning specification;
- a version/status;
- a capability or feature marker where necessary;
- conformance tests;
- compatibility status;
- migration policy.

Possible statuses include:

PROPOSED
DESIGNED
EXPERIMENTAL
IMPLEMENTED
STABLE
DEPRECATED
REMOVED

The status MUST NOT be inferred merely from the existence of grammar rules.

A grammar rule does not prove semantic implementation.

---

15. Grammar Presence Does Not Equal Feature Availability

The following chain defines implementation maturity:

Specified
    ↓
Lexically represented
    ↓
Parsed
    ↓
AST represented
    ↓
Semantically validated
    ↓
Lowered
    ↓
IR represented
    ↓
Backend supported
    ↓
Runtime supported
    ↓
Tested
    ↓
Stable

A construct MUST NOT be documented as stable merely because it exists in "Zamani.g4".

This is especially important for:

- quantum features;
- hardware features;
- AI features;
- HDL features;
- distributed features;
- advanced effects;
- future computing constructs.

---

16. Canonical Grammar Authority

The canonical language specification MUST be the ultimate normative authority for language meaning.

"grammar/Zamani.g4" is the canonical ANTLR grammar representation.

It MUST conform to the canonical specification.

"grammar/grammar.md" is implementation-conformance documentation.

"grammar/Zamani-Grammar.md" is broader language design/specification material and may contain future or proposed capabilities.

The three files MUST NOT become competing definitions.

The authority relationship is:

Canonical Language Specification
             ↓
      Canonical Grammar
             ↓
      Lexer / Parser
             ↓
            AST
             ↓
    Semantic Validation
             ↓
      Canonical IR

---

17. Required Integration With Grammar Files

This file establishes compatibility contracts for:

grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/specification/language-version.md
grammar/specification/compatibility.md
grammar/compatibility/versions.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md
grammar/compatibility/reserved.md
grammar/compatibility/compatibility-matrix.md

No other grammar file may define an incompatible versioning system.

Version syntax MUST be owned by the canonical specification and grammar.

Migration policy belongs in:

grammar/compatibility/migrations.md

Deprecation policy belongs in:

grammar/compatibility/deprecated.md

Reserved syntax belongs in:

grammar/compatibility/reserved.md

Compatibility matrices belong in:

grammar/compatibility/compatibility-matrix.md

---

18. Lexer Compatibility

The lexer MUST treat language version as part of lexical policy.

Version changes MAY alter:

- reserved keywords;
- contextual keywords;
- literal syntax;
- operator availability;
- annotation syntax;
- comment syntax;
- escape sequences;
- Unicode lexical behavior.

Such changes MUST be explicitly versioned.

A previously valid identifier MUST NOT silently become a keyword in a compatible language version unless an explicit contextual-keyword mechanism preserves source compatibility.

---

19. Keyword Compatibility

Adding a new globally reserved keyword can break existing source.

Therefore new keywords SHOULD preferably be introduced as:

1. contextual keywords;
2. namespace-qualified constructs;
3. explicit feature-gated constructs;
4. versioned syntax;
5. otherwise only in a major language-version transition.

The language evolution process MUST evaluate identifier collision before reserving a new keyword.

---

20. Operator Compatibility

Operators are language syntax and therefore compatibility-sensitive.

Changing:

a + b

to have a different meaning is a breaking semantic change.

Changing operator precedence is also potentially breaking.

Changing associativity is potentially breaking.

Adding an operator that changes parsing of previously valid source is potentially breaking.

Therefore operator changes MUST include:

- precedence analysis;
- associativity analysis;
- lexer ambiguity analysis;
- parser ambiguity analysis;
- compatibility tests;
- migration guidance.

---

21. AST Compatibility

The AST is an internal compiler contract but must preserve all source semantics required by downstream stages.

A grammar change is incomplete if the AST cannot represent it without loss.

For every new language construct:

Grammar
   ↓
Parser
   ↓
AST

must be updated as one contract.

The AST MUST NOT require target-specific structures merely to represent portable source semantics.

In particular, quantum AST structures MUST NOT require:

- physical qubit IDs;
- device IDs;
- native gate sets;
- hardware topology;
- calibration records.

Those belong to later semantic/target layers.

---

22. Semantic Compatibility

Semantic compatibility is stronger than syntactic compatibility.

Two source programs are semantically compatible only if their intended language meaning remains equivalent under the applicable language version.

Changes affecting:

- type inference;
- ownership;
- borrowing;
- resource semantics;
- effects;
- concurrency;
- evaluation order;
- numerical semantics;
- quantum measurement semantics;
- quantum control;
- module resolution;
- capability checking;

MUST be treated as semantic changes even if the grammar itself remains unchanged.

---

23. Canonical IR Compatibility

The compiler MUST distinguish:

Source Compatibility

from:

IR Compatibility

The canonical IR may evolve independently of source syntax when a stable semantic translation remains possible.

However, an IR change MUST NOT silently alter the meaning of valid source programs.

The canonical IR is the target-independent semantic boundary.

For quantum computation:

Zamani Source
      ↓
AST
      ↓
Semantic Quantum Model
      ↓
quantum::ir
      ↓
Optimization
      ↓
Routing
      ↓
Scheduling
      ↓
ZQN / Resilience
      ↓
Hardware Lowering

"quantum::ir" remains the canonical quantum semantic boundary.

No compatibility mechanism may introduce a competing frontend quantum IR merely to accommodate a grammar version.

---

24. Quantum Compatibility

Quantum language evolution MUST remain independent of current hardware.

A new quantum processor MUST NOT require a new Zamani language version merely because it introduces:

- a new gate;
- a new topology;
- more qubits;
- different connectivity;
- different native instructions;
- different calibration;
- different pulse semantics.

These are target capabilities.

Likewise, a language version MUST NOT encode:

MAX_QUBITS
MAX_PHYSICAL_QUBITS
MAX_GATES
MAX_REGISTER_SIZE

or equivalent permanent limits.

---

25. Quantum Operation Compatibility

Quantum operations SHOULD use an extensible semantic operation model.

The grammar MUST NOT make today's gate vocabulary the permanent universe of Zamani quantum operations.

For example, the language MUST be capable of representing an operation semantically without requiring every operation to become a new keyword.

Conceptually:

operation
    name
    namespace
    parameters
    controls
    targets
    modifiers
    attributes

Semantic validation determines whether an operation exists.

Target lowering determines how it can be implemented.

---

26. Hardware Compatibility

Hardware compatibility MUST be determined from capabilities, resources, constraints, and target descriptions.

Source syntax MUST NOT require a particular:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- memory size;
- number of devices;
- topology;
- instruction set;
- register width.

A target may be unable to execute a program.

That does not make the source language incompatible.

The compiler MUST distinguish:

Language incompatibility

from:

Target incompatibility

and:

Resource insufficiency

---

27. POCO-REAF Compatibility

POCO-REAF requires preservation of program meaning across target changes.

The formal compatibility objective is:

One Source Program
       ↓
One Defined Semantic Meaning
       ↓
Target-Independent Representation
       ↓
Target-Specific Realization

The following are separate questions:

Can the program be parsed?
Can the program be semantically validated?
Can it be lowered?
Can the target realize it?
Are sufficient resources available?
Can it meet requested performance constraints?

A failure at a later layer MUST NOT be misreported as source-language incompatibility.

---

28. "Compile Once" Clarification

POCO-REAF does not promise that one binary can execute unchanged on every possible architecture.

Instead, it requires that the program's portable semantic representation survive target changes.

The implementation MAY use:

- target-independent IR;
- cached compilation artifacts;
- portable packages;
- deferred lowering;
- runtime dispatch;
- target-specific code generation;
- JIT compilation;
- AOT compilation;
- heterogeneous compilation;
- remote compilation;
- distributed lowering.

The source program MUST NOT require semantic rewriting solely because the target changes.

---

29. Resource Compatibility

Resource requirements are not language-version requirements.

For example:

requires quantum capability

does not mean:

requires a specific QPU

Similarly:

requires memory capacity >= expression

does not establish a language-level maximum.

Resource requirements MUST remain expressible independently of physical target identity.

---

30. No Hard-Coded Compatibility Limits

Compatibility files MUST NOT introduce artificial scalability limits.

The following are prohibited unless they are genuine language semantics:

MAX_SUPPORTED_QUBITS
MAX_SUPPORTED_CORES
MAX_SUPPORTED_THREADS
MAX_SUPPORTED_NODES
MAX_SUPPORTED_DEVICES
MAX_SUPPORTED_TENSOR_DIMENSION
MAX_SUPPORTED_PROGRAM_SIZE
MAX_SUPPORTED_MODULES
MAX_SUPPORTED_GATES

A compatibility matrix MUST NOT use a finite target-size table as a hidden language limitation.

For example, this is invalid as a language rule:

Zamani supports up to 1024 qubits.

A valid formulation is:

The language has no grammar-defined maximum number of qubits.
Actual execution is bounded by the resources and capabilities available to the selected compilation/execution environment.

---

31. Versioning Does Not Freeze Scalability

A stable Zamani language version MUST remain capable of expressing computations larger than any particular implementation tested at the time of language release, subject to representational and resource constraints.

A test environment that can process:

N

resources MUST NOT imply a language limit of:

N

The distinction is:

Test capacity
≠
Language capacity

and:

Current backend capacity
≠
Language capacity

---

32. Arbitrarily Large Programs

The language specification MUST NOT define an arbitrary semantic maximum for:

- source size;
- number of declarations;
- number of functions;
- number of modules;
- expression size;
- quantum operations;
- qubits;
- classical values;
- tensor dimensions;
- distributed nodes;
- hardware resources.

Implementations MAY have practical resource budgets.

Such budgets MUST be:

- implementation-level;
- configurable where practical;
- diagnosable;
- distinct from language semantics;
- documented separately.

---

33. Parser Depth Compatibility

A parser implementation MUST NOT use an arbitrary language-version limit such as:

MAX_NESTING = 1024

to define language compatibility.

Where deep source structures could exhaust the Rust call stack, implementation techniques such as:

- explicit stacks;
- iterative parsing where practical;
- worklists;
- incremental processing;

SHOULD be preferred.

Any implementation safety budget MUST be distinguished from language syntax.

Rust "unsafe" MUST NOT be used to bypass these constraints.

---

34. Version-Specific Grammar

A language version MAY define a different grammar.

However, the implementation MUST retain a clear mapping:

Language Version
       ↓
Grammar Rules
       ↓
AST Contract
       ↓
Semantic Contract

The parser MUST NOT select grammar behavior based on undocumented heuristics.

---

35. Grammar Evolution

A grammar change MUST first be classified as:

Non-breaking
Breaking
Experimental
Deprecated
Removal
Clarification
Bug fix

The classification MUST be recorded before implementation.

Every breaking change MUST identify:

- affected constructs;
- old behavior;
- new behavior;
- affected versions;
- migration path;
- diagnostic behavior;
- compatibility tests.

---

36. Syntax Additions

A syntax addition is compatible only if it cannot reinterpret existing valid source in an incompatible way.

Before adding syntax, analyze:

- lexical conflicts;
- keyword conflicts;
- identifier conflicts;
- operator conflicts;
- precedence changes;
- ambiguity;
- parser recovery;
- AST representation;
- semantic interpretation.

If the addition conflicts with existing valid source, it MUST use an explicit compatibility mechanism.

---

37. Syntax Removal

Stable syntax MUST NOT be removed in a compatible MINOR or PATCH release.

Removal requires:

Deprecated
    ↓
Migration period
    ↓
Major-version removal

unless the construct was explicitly experimental or unstable.

---

38. Deprecation

Deprecation MUST be explicit.

A deprecated construct MUST have:

- deprecation version;
- reason;
- replacement;
- migration guidance;
- removal status;
- tests for expected diagnostics.

Deprecation MUST NOT silently change semantics.

---

39. Reserved Syntax

Future language space SHOULD be reserved only where doing so has clear architectural value.

Reserved names MUST NOT accidentally become unusable forever merely because a speculative future feature was imagined.

Reserved syntax MUST be:

- documented;
- versioned;
- scoped;
- reviewable;
- removable if unused.

The reserved namespace belongs to the language architecture, not to individual hardware vendors.

---

40. Dialect Compatibility

Zamani MAY support dialects or domain extensions.

A dialect MUST NOT silently redefine stable core Zamani syntax.

A dialect MUST have:

- identity;
- namespace;
- version;
- owner;
- capability requirements;
- compatibility declaration;
- semantic contract;
- lowering contract.

A dialect MUST NOT create a second incompatible language under the Zamani name.

---

41. Vendor Extensions

Vendor extensions MUST remain isolated from the portable language core.

A vendor may define:

vendor-specific capability
vendor-specific operation
vendor-specific target attribute
vendor-specific optimization hint

but MUST NOT redefine stable Zamani semantics.

Vendor extensions MUST NOT impose their hardware limitations on portable Zamani source.

---

42. Hardware-Specific Syntax

Hardware-specific syntax MUST be clearly distinguished from portable semantics.

For example:

portable computation

and:

target realization constraint

are different concepts.

A target constraint may be attached through:

- target declarations;
- capability requirements;
- resource requirements;
- constraints;
- preferences;
- hints;
- deployment configuration.

It MUST NOT silently change the meaning of the portable computation.

---

43. Module Compatibility

Module paths and names are compatibility-sensitive.

A compatible release MUST NOT unexpectedly redirect an existing module name to different semantics.

Module changes MUST consider:

- imports;
- exports;
- visibility;
- aliases;
- package versions;
- dependency resolution;
- namespace collisions.

---

44. Package Compatibility

Package compatibility MUST distinguish:

Package Version
Language Version
Compiler Version
ABI Version
IR Version
Runtime Version

These versions MUST NOT be conflated.

A package MAY support multiple language versions.

A compiler MAY consume packages built with different compiler versions if their declared interfaces and IR/ABI contracts remain compatible.

---

45. ABI Compatibility

Language source compatibility does not imply ABI compatibility.

ABI compatibility belongs to the interoperability/compiler/runtime layers.

Changes to:

- calling convention;
- layout;
- alignment;
- symbol naming;
- binary representation;
- FFI contracts;

MUST be versioned independently where necessary.

The grammar MUST NOT encode ABI implementation details into ordinary portable source syntax.

---

46. Runtime Compatibility

Runtime compatibility is distinct from language compatibility.

A stable source program may require a newer runtime because it uses a newer runtime capability.

The compiler/runtime MUST report:

language version
compiler version
runtime requirement
target capability requirement

separately.

---

47. Execution Compatibility

Execution compatibility MUST account for:

- target capabilities;
- resources;
- runtime features;
- security policies;
- scheduling constraints;
- deployment constraints;
- availability.

Execution failure MUST NOT be represented as a parser failure.

---

48. Semantic Preservation Across Lowering

Every compatibility-preserving compilation MUST satisfy:

Source Semantics
      =
Canonical IR Semantics
      =
Target Realization Semantics

subject to explicitly defined implementation approximations, numerical models, probabilistic semantics, or hardware execution models.

For quantum programs, measurement, probabilistic behavior, entanglement, control flow, and error semantics MUST remain faithful to the defined quantum model.

---

49. Quantum Backend Evolution

A backend may evolve from:

native gate set A

to:

native gate set B

without requiring a source-language version change.

The compiler may perform:

decomposition
routing
optimization
scheduling
resilience transformation

provided semantic meaning is preserved.

The grammar MUST NOT encode native gate sets as permanent language requirements.

---

50. QEC and ZQN Compatibility

Quantum error correction and ZQN-related semantics MUST remain downstream of source parsing unless a construct is explicitly part of Zamani source semantics.

The compatibility architecture is:

Quantum Source
      ↓
AST
      ↓
Semantic Quantum Model
      ↓
quantum::ir
      ↓
QEC / ZQN / Optimization / Routing / Scheduling

A new hardware error model MUST NOT require a new source-language syntax unless the error model itself is intentionally exposed as a language semantic.

---

51. Scheduling Compatibility

Scheduling policies are not language versions.

A change from:

ASAP

to:

ALAP

or another scheduling strategy MUST NOT change source compatibility when both preserve program semantics.

Scheduling belongs to the execution/compiler layer.

Source-level timing constraints, when they are semantically meaningful, must be distinguished from implementation scheduling policy.

---

52. Optimization Compatibility

Optimization passes MUST preserve language semantics.

Changing an optimization strategy MUST NOT require source migration if the language semantics remain unchanged.

Optimization failures MUST NOT be reported as grammar incompatibilities unless the optimization depends on a genuinely unsupported language feature.

---

53. Determinism

For identical:

source
language version
configuration

the lexical and syntactic interpretation MUST be deterministic.

Compatibility tests MUST verify:

same source
+
same language version
=
same AST structure

subject to documented metadata such as source-location identity.

---

54. Diagnostics Compatibility

Diagnostics SHOULD remain structurally stable enough for tooling to consume them.

Machine-readable diagnostics SHOULD include:

- diagnostic code;
- severity;
- language version;
- source location;
- affected construct;
- compatibility category;
- suggested migration where applicable.

Human-readable wording MAY improve between compatible releases.

Diagnostic codes SHOULD remain stable unless their semantic category changes.

---

55. Compatibility Diagnostic Categories

The compiler SHOULD distinguish at least:

UnsupportedLanguageVersion
UnsupportedFeature
DeprecatedFeature
RemovedFeature
InvalidVersion
IncompatibleVersion
SyntaxConflict
SemanticIncompatibility
TargetIncompatibility
ResourceInsufficiency
CapabilityUnavailable
RuntimeIncompatibility
AbiIncompatibility
DialectIncompatibility

Exact error names belong to the compiler error contract.

---

56. Feature Capability Versus Version

A feature may be unavailable because of:

language version
compiler implementation
target capability
runtime capability
enabled dialect
build configuration

These causes MUST remain distinguishable.

For example:

quantum operation unavailable

does not automatically mean:

language version unsupported

It may instead mean:

target capability unavailable

---

57. Compatibility Matrix

The repository MUST maintain a compatibility matrix under:

grammar/compatibility/compatibility-matrix.md

The matrix SHOULD cover:

Layer| Version| Status| Compatible With| Notes
Zamani language| X.Y.Z| stable| declared ranges| source semantics
Grammar| corresponding| stable| language version| syntax
Lexer| corresponding| stable| language version| tokens
Parser| corresponding| stable| language version| AST construction
AST| corresponding| stable| semantic model| structural contract
Semantic model| corresponding| stable| IR| meaning
"quantum::ir"| declared| stable/experimental| quantum compiler| canonical quantum meaning
Compiler| declared| stable| language range| implementation
Runtime| declared| stable| execution contract| runtime behavior
ABI| declared| stable| FFI targets| binary interface

The matrix MUST NOT become a hidden finite limit on target scale.

---

58. Compatibility Test Requirements

Every language-version release MUST have tests covering:

Positive tests

Existing valid programs remain valid where compatibility is promised.

Negative tests

Unsupported versions and invalid migrations fail deterministically.

Boundary tests

Test:

- first supported version;
- latest supported version;
- unsupported older versions;
- unsupported future versions;
- major-version boundaries.

Cross-domain tests

Test combinations including:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

Scalability tests

Verify that compatibility infrastructure does not impose artificial limits on:

- qubits;
- cores;
- threads;
- nodes;
- devices;
- memory;
- tensor dimensions;
- operations;
- modules;
- source size.

---

59. Golden Compatibility Tests

The repository SHOULD maintain versioned golden programs.

Conceptually:

tests/compatibility/
    v1/
    v2/
    migrations/
    deprecated/
    removed/
    cross-version/

Each stable language version MUST have representative source programs.

Golden tests MUST verify:

source
→ lexer
→ parser
→ AST
→ semantic validation
→ IR

where the corresponding implementation exists.

---

60. AST Compatibility Tests

For source constructs whose AST representation is part of a tooling contract, compatibility tests SHOULD verify that equivalent source constructs produce equivalent canonical AST structures.

The test MUST NOT depend on:

- pointer addresses;
- hash-map iteration order;
- hardware identifiers;
- nondeterministic metadata.

---

61. Semantic Compatibility Tests

A compatibility test is incomplete if it only verifies parsing.

For stable constructs, tests SHOULD verify:

Source A
   ↓
AST A
   ↓
Semantic A
   ↓
IR A

and compare the relevant semantic properties across compiler versions.

---

62. Round-Trip Compatibility

Where a formatter/printer exists:

Source
 ↓
Parser
 ↓
AST
 ↓
Printer
 ↓
Source'
 ↓
Parser

must preserve semantics.

The output does not need to be textually identical unless the formatter contract explicitly requires it.

---

63. Serialization Compatibility

If AST, IR, package, or metadata serialization is exposed as a persistent interface, its format MUST have its own version.

For example:

AST format version
IR format version
Package metadata version

must not be assumed identical to:

Zamani language version

Readers MUST reject unsupported serialized versions safely.

---

64. Migration Policy

Migration MUST be explicit and deterministic.

A migration MUST identify:

from version
to version
affected feature
old syntax
new syntax
semantic impact
automatic migration availability
manual migration requirements

Migration tooling MUST NOT silently change program semantics.

Migration transforms MUST preserve source intent wherever possible.

---

65. Automatic Migration

Automatic migration MAY be provided for mechanical changes such as:

- renamed syntax;
- deprecated aliases;
- explicit version declarations;
- syntax normalization;
- namespace changes.

Automatic migration MUST NOT guess about semantic intent when multiple interpretations are possible.

Ambiguous migrations MUST require developer intervention.

---

66. Major-Version Migration

A major version migration MUST be explicit.

For example:

Zamani 1.x
    ↓
Zamani 2.x

must not happen silently.

The compiler SHOULD provide a diagnostic identifying:

- current source version;
- required version;
- affected construct;
- migration documentation.

---

67. Minor-Version Migration

Minor-version upgrades SHOULD normally be source-compatible.

If a minor release contains a known incompatibility due to a specification correction, that correction MUST be explicitly classified and documented.

The language project MUST NOT use MINOR versions as an excuse for undocumented breaking changes.

---

68. Patch-Version Compatibility

PATCH releases MUST preserve:

- syntax;
- semantics;
- stable AST meaning;
- stable package interfaces;
- stable compatibility guarantees.

Patch releases may correct implementation bugs.

If correcting a bug changes behavior of previously accepted invalid source, that is not necessarily a breaking language change.

However, the change MUST be documented where users could reasonably have depended on the erroneous behavior.

---

69. Specification Corrections

A specification error MAY be corrected without a major language release when:

1. the previous behavior was explicitly invalid;
2. the correction does not redefine valid source;
3. compatibility impact is analyzed;
4. conformance tests are updated.

If users could reasonably rely on the previous behavior as valid language semantics, the change MUST follow normal compatibility policy.

---

70. Documentation Compatibility

Documentation MUST distinguish:

Stable
Experimental
Proposed
Implemented
Unsupported
Deprecated
Removed

"grammar/Zamani-Grammar.md" MUST NOT present proposed syntax as stable merely because it is described there.

"grammar/grammar.md" MUST remain aligned with the implementation.

"grammar/Zamani.g4" MUST remain aligned with canonical syntax.

---

71. Compatibility Ownership

Ownership is divided as follows:

File/Subsystem| Owns| Does Not Own
"versions.md"| version policy| syntax implementation
"language-version.md"| language-version declaration semantics| migration mechanics
"compatibility.md"| compatibility principles| individual historical entries
"migrations.md"| migration procedures| core version semantics
"deprecated.md"| deprecation records| parser implementation
"reserved.md"| reserved syntax| target hardware
"compatibility-matrix.md"| compatibility relationships| language semantics
"Zamani.g4"| ANTLR syntax| semantic compatibility
lexer| tokenization| semantic version interpretation beyond lexical policy
parser| syntax recognition| target lowering
AST| source structure| hardware realization
semantic layer| meaning and validity| textual parsing
"quantum::ir"| canonical quantum semantics| source grammar
optimization| semantics-preserving transformation| language definition
scheduling| timing/resource scheduling| syntax
hardware| target capability/realization| portable language meaning
runtime| execution| grammar definition

---

72. Upstream Contracts

This file depends on the following upstream contracts:

Canonical language specification
Canonical language-version definition
Canonical grammar
Lexer token contract
Parser contract
AST contract
Semantic model
Module/package model
Canonical IR contract
quantum::ir contract

These contracts MUST define enough information to determine whether a change is compatible.

---

73. Downstream Consumers

This file is consumed by:

- compiler version handling;
- parser/frontend version selection;
- semantic validation;
- package manager;
- build system;
- formatter;
- LSP;
- diagnostics;
- migration tooling;
- compatibility tests;
- documentation tooling;
- release tooling;
- IR versioning;
- interoperability tooling.

No downstream consumer may invent an alternative compatibility policy.

---

74. Integration With "grammar/Zamani.g4"

"Zamani.g4" MUST:

- recognize only syntax defined for the applicable language version;
- avoid silent acceptance of future-version constructs;
- preserve compatibility where required;
- avoid fixed hardware limits;
- remain independent of target hardware;
- avoid duplicate quantum semantic definitions.

Version-sensitive grammar behavior MUST be traceable to this compatibility policy and the canonical specification.

---

75. Integration With "src/lexer.rs"

The lexer MUST:

- implement the lexical contract for the selected language version;
- recognize only valid tokens for that version;
- preserve deterministic tokenization;
- produce source spans;
- provide structured errors;
- avoid hidden version-dependent global state.

New keywords MUST undergo compatibility analysis before being reserved.

---

76. Integration With "src/parser.rs"

The parser MUST:

- select the correct language-version grammar;
- reject unsupported versions;
- produce canonical AST nodes;
- remain deterministic;
- preserve source spans;
- recover safely;
- always make progress after errors;
- avoid target-specific assumptions;
- use safe Rust only.

---

77. Integration With "src/ast/"

Every versioned syntax feature MUST have a corresponding AST representation where required.

AST structures MUST be:

- lossless with respect to required source semantics;
- domain-neutral where appropriate;
- independent of hardware realization;
- independent of backend implementation.

Quantum AST structures MUST eventually lower toward canonical quantum semantics rather than becoming a competing quantum IR.

---

78. Integration With Semantic Analysis

Semantic analysis MUST evaluate version-specific rules after parsing.

It MUST distinguish:

version invalid

from:

syntax invalid

from:

semantic invalid

from:

capability unavailable

from:

resource unavailable

---

79. Integration With "quantum::ir"

The compatibility system MUST NOT create version-specific duplicate quantum IRs merely because source syntax evolves.

Instead:

Zamani v1 syntax
      ↓
AST
      ↓
canonical quantum semantics
      ↓
quantum::ir

Zamani v2 syntax
      ↓
AST
      ↓
canonical quantum semantics
      ↓
quantum::ir

where semantic compatibility exists.

If "quantum::ir" itself changes incompatibly, its versioning MUST be handled by the quantum IR contract independently of source grammar versioning.

---

80. Integration With QEC, ZQN, Routing and Scheduling

Compatibility changes to source quantum constructs MUST be tested through the complete semantic path when the corresponding implementation exists:

Source
 ↓
AST
 ↓
Semantic quantum model
 ↓
quantum::ir
 ↓
QEC
 ↓
ZQN
 ↓
Optimization
 ↓
Routing
 ↓
Scheduling
 ↓
Hardware lowering

No downstream subsystem may reinterpret a stable source construct merely because its internal implementation changed.

---

81. Integration With Hardware

Hardware capabilities MUST be discovered or supplied through target infrastructure.

The compatibility system MUST NOT encode:

QPU X
CPU Y
GPU Z

as permanent language requirements.

A target description may state:

supports operation X
supports capability Y
provides resource Z

without changing language version compatibility.

---

82. Integration With Resources

Resource expressions belong to the resource model.

Versioning MUST preserve the distinction:

language requirement

versus:

resource requirement

For example:

requires qubits(expression)

must not imply a compiler-defined maximum.

---

83. Integration With Compilation

The compiler MUST retain enough metadata to identify:

- language version;
- compiler version;
- grammar/specification revision where necessary;
- IR version;
- target requirements;
- runtime requirements;
- dialect versions.

This metadata MUST be machine-readable where compilation artifacts are persisted.

---

84. Integration With Runtime

Runtime compatibility MUST be checked against runtime requirements independently from source language version.

A runtime MUST NOT execute an artifact whose required semantic/runtime contract it cannot satisfy.

Failure MUST produce a structured compatibility error.

---

85. Integration With Tooling

Tooling such as:

- LSP;
- formatter;
- syntax highlighter;
- language server;
- documentation generator;
- static analyzer;
- IDE integrations;

MUST use the same language-version rules.

Tooling MUST NOT silently parse a different language than the compiler.

---

86. Integration With Examples

Every stable language-version feature SHOULD have at least one example where practical.

Examples MUST declare or inherit a deterministic language version.

Examples MUST NOT claim support for features whose implementation status is merely:

PROPOSED
DESIGNED

---

87. Version Detection and Source Discovery

Version discovery MUST NOT depend on:

- hardware;
- runtime environment;
- machine topology;
- number of processors;
- available GPUs;
- number of qubits.

Language interpretation MUST remain deterministic across machines.

---

88. Reproducibility

A build intended to be reproducible SHOULD record:

language version
compiler version
dependency versions
IR version
dialect versions
target specification
relevant compilation configuration

Target-specific details MUST remain metadata rather than altering the language semantics.

---

89. Cross-Platform Compatibility

The same source program SHOULD remain semantically identical across:

- Linux;
- Windows;
- macOS;
- embedded environments;
- cloud systems;
- clusters;
- HPC systems;
- CPU targets;
- GPU targets;
- FPGA targets;
- ASIC toolchains;
- quantum simulators;
- QPUs;
- future targets.

Where execution differs due to floating-point models, probabilistic behavior, hardware noise, or explicitly implementation-defined behavior, the relevant semantic contract MUST state that distinction.

---

90. Numerical Compatibility

Numerical behavior MUST be specified separately from syntax versioning.

A compiler MUST NOT silently change numerical semantics merely because:

- a different CPU is selected;
- a GPU is selected;
- vectorization changes;
- an accelerator is selected.

Where exact numerical equivalence is impossible, the language/runtime contract MUST specify the allowed behavior.

---

91. Distributed Compatibility

Changing the number of nodes MUST NOT require a language version change.

A program should be able to express distributed computation without encoding:

node_count = permanent_language_limit

Deployment infrastructure determines actual placement.

---

92. AI/Data Compatibility

Tensor, model, dataset, training, and inference constructs MUST remain independent of fixed hardware dimensions where those dimensions are not semantic requirements.

For example:

tensor dimension

may be program data.

It MUST NOT become a grammar-level machine limit.

---

93. HDL Compatibility

HDL language evolution MUST distinguish:

hardware semantics

from:

physical implementation

Changing FPGA family, ASIC process, clock capability, routing resources, or hardware implementation strategy MUST NOT automatically require a language-version change.

---

94. Security Compatibility

Security semantics are compatibility-sensitive.

Changes to:

- permissions;
- capability requirements;
- identity semantics;
- cryptographic declarations;
- trust boundaries;

MUST be explicitly versioned when they alter program meaning.

A security relaxation MUST NOT be introduced silently.

---

95. Safe Rust Requirement

All compatibility infrastructure implemented in Rust MUST use safe Rust.

The following are prohibited:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

Compatibility code MUST NOT rely on undefined behavior or unsafe memory manipulation.

The Rust compiler baseline is:

Rust 1.97.x

with Rust 1.97.1 used for production validation.

---

96. No Compiler-Version Leakage

The language specification MUST NOT expose Rust implementation details as Zamani language semantics.

For example, the language MUST NOT say:

this syntax exists because Rust 1.97 supports X

Such information belongs in implementation documentation.

---

97. Compatibility and Feature Flags

Compiler feature flags MAY enable implementation capabilities.

They MUST NOT silently redefine stable language semantics.

A feature flag MUST be classified as one of:

implementation capability
experimental language feature
target capability
tooling capability
build configuration

The classification MUST be explicit.

---

98. No Hidden Compatibility Modes

The compiler MUST NOT have undocumented modes that alter language semantics.

Compatibility behavior MUST be discoverable through:

- source version;
- package metadata;
- documented compiler configuration;
- explicit dialect declaration.

Environment variables MUST NOT silently change language meaning.

---

99. Legacy Compatibility

Legacy source MAY be supported through a legacy language mode.

Legacy mode MUST:

- identify the source version;
- have documented semantics;
- be tested;
- provide migration diagnostics;
- avoid silently changing old program meaning.

Legacy support MUST NOT prevent the canonical language from evolving indefinitely.

---

100. Removal of Legacy Support

Legacy versions MAY eventually become unsupported.

Removal MUST include:

- release notice;
- compatibility matrix update;
- migration documentation;
- diagnostic;
- removal from supported-version tests.

The source program itself remains historically identifiable even after compiler support ends.

---

101. Infinite/Future Scalability Principle

"Forever" in POCO-REAF means semantic extensibility, not a promise that a finite compiler implementation has infinite memory or execution time.

The language MUST therefore avoid architecture that assumes today's:

- processor widths;
- memory capacities;
- quantum technologies;
- accelerator counts;
- communication models;
- storage systems;
- hardware topologies.

Future systems must be able to implement existing Zamani semantics without requiring the language to be rewritten around every new machine.

---

102. Future-Version Compatibility

Future language versions SHOULD preserve stable semantic concepts even when their syntax evolves.

For example:

portable computation
resource requirement
capability
constraint
effect
quantum operation
hardware intent

should remain composable semantic categories.

Future hardware MUST be able to introduce new implementations behind these abstractions.

---

103. Extension Mechanism

The language MUST prefer extensible semantic mechanisms over repeated breaking grammar additions.

Examples include:

namespaces
attributes
capabilities
operations
generic types
traits/interfaces
dialects
effect declarations
resource expressions
target descriptions

A new domain SHOULD NOT automatically require a new core keyword.

---

104. Compatibility Review Gate

Every language change MUST pass a compatibility review before merging.

The review MUST answer:

1. Does existing valid source still parse?
2. Does its AST meaning remain stable?
3. Does its semantic meaning remain stable?
4. Does its IR meaning remain stable?
5. Does quantum meaning remain stable where applicable?
6. Does the change introduce keyword conflicts?
7. Does it introduce operator conflicts?
8. Does it introduce hardware assumptions?
9. Does it introduce resource limits?
10. Does it require a version increment?
11. Does it require migration documentation?
12. Does it require deprecation?
13. Are compatibility tests present?
14. Are cross-domain tests present?
15. Does it preserve safe-Rust requirements?

---

105. Hard-Coding Audit

Every compatibility change MUST be audited for accidental hard-coding.

Search for:

MAX_
LIMIT_
COUNT_
SIZE_
WIDTH_
DEPTH_
QUBIT_
CORE_
THREAD_
DEVICE_
NODE_
GPU_
FPGA_
MEMORY_

The presence of such identifiers is not automatically forbidden.

Each occurrence MUST be classified as:

language semantic
resource policy
implementation budget
test fixture
diagnostic limit
target capability
accidental hard-code

Accidental language-level hard-coding MUST be removed.

---

106. Compatibility Completion Criteria

"grammar/compatibility/versions.md" is complete only when:

- language-version semantics are defined;
- compiler-version semantics are separated;
- Rust baseline is defined;
- source compatibility is defined;
- semantic compatibility is defined;
- AST compatibility is addressed;
- IR compatibility is addressed;
- quantum compatibility is addressed;
- hardware compatibility is addressed;
- resource compatibility is addressed;
- dialect compatibility is addressed;
- migration ownership is established;
- deprecation ownership is established;
- reserved-space ownership is established;
- compatibility matrices are established;
- tests are specified;
- POCO-REAF implications are defined;
- hard-coding rules are defined;
- downstream integration is documented.

---

107. Required Companion Files

This file establishes the contracts for the following compatibility subsystem:

grammar/compatibility/
├── README.md
├── versions.md
├── migrations.md
├── deprecated.md
├── reserved.md
└── compatibility-matrix.md

Their responsibilities are:

"README.md"

Owns navigation and architecture of the compatibility subsystem.

Does not own individual version decisions.

"versions.md"

Owns the normative version model and compatibility principles.

Does not own individual historical migrations.

"migrations.md"

Owns concrete migration procedures.

Does not redefine the version model.

"deprecated.md"

Owns individual deprecation records.

Does not define general language-version semantics.

"reserved.md"

Owns reserved syntax and namespace policy.

Does not own implementation-specific keywords.

"compatibility-matrix.md"

Owns the concrete compatibility matrix between versions and implementation layers.

Does not redefine the underlying semantics.

---

108. Dependency Order

The compatibility subsystem MUST be implemented in this order:

1. specification/language-version.md
        ↓
2. compatibility/versions.md
        ↓
3. compatibility/deprecated.md
        ↓
4. compatibility/reserved.md
        ↓
5. compatibility/migrations.md
        ↓
6. compatibility/compatibility-matrix.md
        ↓
7. grammar/Zamani.g4
        ↓
8. lexer
        ↓
9. parser
        ↓
10. AST
        ↓
11. semantic analysis
        ↓
12. canonical IR
        ↓
13. quantum::ir integration
        ↓
14. compiler
        ↓
15. runtime/tooling
        ↓
16. compatibility test suite

No downstream file should need to invent version semantics after "versions.md" is accepted.

---

109. File Integration Contract

This file is intentionally designed to be independently completable.

Once this file is accepted as normative:

- "language-version.md" consumes its version terminology;
- "Zamani.g4" consumes its grammar-version rules;
- lexer consumes its lexical compatibility rules;
- parser consumes its parser-version rules;
- AST consumes its structural compatibility requirements;
- semantic analysis consumes its semantic compatibility rules;
- "quantum::ir" consumes its quantum compatibility boundary;
- compiler consumes its compiler/language distinction;
- runtime consumes its runtime compatibility distinction;
- migration tooling consumes its migration contract;
- compatibility tests consume its version categories.

Those files MUST NOT redefine the underlying version model.

If a downstream implementation discovers a contradiction, the contradiction MUST be resolved by an explicit architecture/specification change rather than an undocumented local exception.

---

110. Production Invariants

The following invariants are mandatory:

I1. One canonical Zamani language.

I2. Language version is independent of Rust version.

I3. Rust implementation uses no unsafe code.

I4. Stable source semantics are preserved across compatible releases.

I5. Syntax compatibility is distinct from semantic compatibility.

I6. Semantic compatibility is distinct from target compatibility.

I7. Resource insufficiency is distinct from language incompatibility.

I8. Hardware capabilities do not define language limits.

I9. Quantum syntax does not define physical hardware limits.

I10. quantum::ir remains the canonical quantum semantic boundary.

I11. No duplicate quantum IR is created for versioning.

I12. Grammar files do not silently invent semantics.

I13. Future versions are not silently accepted as older versions.

I14. Deprecated features have explicit status.

I15. Removed features have explicit migration history.

I16. Compatibility changes are tested.

I17. Version resolution is deterministic.

I18. Parser behavior is deterministic for identical source/version inputs.

I19. Compatibility metadata is machine-readable where artifacts are persisted.

I20. No artificial finite machine-size limits are introduced.

I21. Target-specific details remain outside portable source semantics.

I22. Language evolution must remain extensible to future computational paradigms.

I23. Existing valid features are preserved unless explicitly migrated or removed.

I24. Compatibility infrastructure itself must not become a scalability bottleneck.

I25. Every compatibility contract has an explicit owner.

---

111. Final Compatibility Principle

Zamani compatibility is ultimately defined by preservation of meaning.

The fundamental invariant is:

                SAME SOURCE
                     │
                     ▼
              SAME LANGUAGE VERSION
                     │
                     ▼
              SAME LANGUAGE MEANING
                     │
          ┌──────────┴──────────┐
          ▼                     ▼
     MACHINE A              MACHINE B
          │                     │
          ▼                     ▼
   Target realization     Target realization
          │                     │
          └──────────┬──────────┘
                     ▼
            SAME PROGRAM INTENT

Machines may differ.

Architectures may differ.

Resources may differ.

Quantum processors may differ.

Hardware topologies may differ.

Schedulers may differ.

Optimizers may differ.

Backends may differ.

Runtime environments may differ.

The stable Zamani source meaning MUST NOT depend on those differences unless the programmer explicitly requested target-specific behavior.

Therefore:

«Version Zamani semantics, not machines.»

«Version language meaning, not hardware capacity.»

«Version source contracts, not temporary implementation limitations.»

«Preserve one semantic program across many realizations.»

This is the compatibility foundation required for:

Program Once → Compile Once → Run Everywhere → Anywhere → Forever

and for Zamani's architectural objective:

From Atom to Everywhere.