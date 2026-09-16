Zamani Compatibility Specification

Path: "grammar/spec/compatibility.md"
Language: Zamani
Status: Normative
Specification role: Compatibility, conformance, migration, interoperability, and semantic-preservation contract
Implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Rust safety requirement: Production compiler implementation MUST NOT use Rust "unsafe"
Primary portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: From the smallest supported computation to arbitrarily large computations, bounded by actual program semantics, representation limits, explicitly declared resource policies, physical constraints, and resources available to the implementation/target — never by arbitrary language-level ceilings.

---

0. Document Contract

0.1 Purpose

This document defines how Zamani preserves compatibility while evolving across:

- source programs;
- language versions;
- lexical contracts;
- grammar representations;
- parsers;
- frontend ASTs;
- semantic models;
- type systems;
- effect systems;
- capability systems;
- resource requirements;
- canonical IR;
- "quantum::ir";
- classical IR;
- HDL/hardware representations;
- dialects;
- compiled artifacts;
- ABIs;
- runtimes;
- targets;
- interoperability formats;
- development tools;
- generated documentation;
- future computational substrates.

Compatibility is defined primarily in terms of preservation of specified meaning.

Implementation details MAY change.

Representations MAY change.

Compiler algorithms MAY change.

Hardware MAY change.

Target realization MAY change.

The meaning of a compatible Zamani program MUST NOT silently change.

---

0.2 Fundamental rule

The fundamental compatibility invariant is:

same source
+
same applicable language contract
+
same applicable dialect contracts
=
same specified program meaning

Subject to explicitly documented:

- implementation-defined behavior;
- target-defined behavior;
- dialect-defined behavior;
- unspecified behavior;
- resource availability;
- physical constraints;
- explicitly requested approximation or alternative execution semantics.

A compiler MUST NOT silently change source semantics merely because a target has fewer resources or different hardware.

---

0.3 Compatibility is not identical to implementation equality

The following are deliberately different:

source compatibility
syntax compatibility
AST compatibility
semantic compatibility
IR compatibility
artifact compatibility
ABI compatibility
runtime compatibility
target compatibility
execution compatibility

Two implementations MAY use different:

- parser implementations;
- AST layouts;
- optimization algorithms;
- IR encodings;
- instruction selections;
- scheduling strategies;
- routing algorithms;
- memory layouts;
- hardware mappings;

while remaining semantically compatible.

Conversely, two implementations MAY accept syntactically similar programs while being semantically incompatible.

Therefore:

same representation != necessarily same semantics

different representation != necessarily incompatible

---

1. Authority and Ownership

1.1 Compatibility does not create another language authority

This file owns compatibility policy.

It does not become a second source-language specification.

The authority chain is:

grammar/DESIGN.md
        │
        ▼
language specification
        │
        ├── grammar/spec/lexical.md
        ├── grammar/spec/syntax.md
        ├── grammar/spec/semantics.md
        ├── grammar/spec/type-system.md
        ├── grammar/spec/resources.md
        ├── grammar/spec/quantum.md
        └── other normative specification contracts
        │
        ▼
compatibility contract
        │
        ▼
canonical grammar representation
        │
        ▼
lexer / parser
        │
        ▼
frontend AST
        │
        ▼
semantic model
        │
        ▼
canonical IR
        │
        ├── classical semantics
        ├── quantum::ir
        └── HDL/hardware semantics
        │
        ▼
optimization / lowering
        │
        ├── routing
        ├── scheduling
        ├── resilience
        ├── QEC
        └── ZQN
        │
        ▼
HAL / target realization
        │
        ▼
runtime / execution

No lower layer may silently redefine the meaning established by a higher layer.

---

1.2 Existing file ownership

The following ownership MUST remain explicit.

File / subsystem| Owns| Does not own
"grammar/DESIGN.md"| grammar architecture and boundaries| individual implementation behavior
"grammar/spec/lexical.md"| lexical contract| semantic interpretation
"grammar/spec/syntax.md"| source syntax| target realization
"grammar/spec/semantics.md"| language meaning| parser implementation
"grammar/spec/type-system.md"| type semantics| hardware mapping
"grammar/spec/resources.md"| resource/capability semantics| physical scheduling
"grammar/spec/quantum.md"| quantum language semantics| physical QPU implementation
"grammar/spec/compatibility.md"| compatibility policy| grammar production definitions
"grammar/specification/language-version.md"| language-version model| detailed compatibility decisions
"grammar/Zamani.g4"| ANTLR grammar representation| independent semantic authority
"grammar/Zamani-Grammar.md"| broader design/history/proposals| automatic implementation claims
"grammar/grammar.md"| implementation-conformance reference| future language authority
"src/lexer.rs"| executable lexical implementation| semantic analysis
"src/parser.rs"| executable parsing| target realization
"src/frontend/ast/"| domain-neutral source AST| physical resource mapping
semantic layer| semantic validation/resolution| lexical tokenization
canonical IR| computational representation| source-language grammar
"quantum::ir"| canonical quantum semantic boundary| source grammar
routing| physical realization| language meaning
scheduling| timing/resource ordering| source syntax
QEC| quantum error correction| parser/grammar
ZQN| quantum fault/noise semantics| duplicate quantum IR
HAL| target capabilities/state| source-language semantics
runtime| execution| source syntax

The repository's design already establishes the separation between source syntax, AST, semantic analysis, IR, routing/scheduling/resilience, ZQN, HAL, and target realization. This compatibility contract enforces that separation rather than introducing another architecture.

---

2. Compatibility Dimensions

Zamani MUST NOT reduce compatibility to a single boolean.

At minimum, the following dimensions MUST be distinguishable:

1. source compatibility;
2. lexical compatibility;
3. syntax compatibility;
4. parser compatibility;
5. AST compatibility;
6. type compatibility;
7. effect compatibility;
8. semantic compatibility;
9. capability compatibility;
10. resource compatibility;
11. module compatibility;
12. package compatibility;
13. dialect compatibility;
14. canonical-IR compatibility;
15. "quantum::ir" compatibility;
16. artifact compatibility;
17. serialization compatibility;
18. ABI compatibility;
19. runtime compatibility;
20. target compatibility;
21. execution compatibility;
22. tooling compatibility;
23. diagnostic compatibility;
24. interoperability compatibility;
25. security compatibility;
26. provenance compatibility.

A compatibility failure MUST identify the relevant dimension whenever determinable.

Prefer:

ZMN-COMP-LANGUAGE-VERSION
ZMN-COMP-SOURCE-SYNTAX
ZMN-COMP-SEMANTIC
ZMN-COMP-AST
ZMN-COMP-IR
ZMN-COMP-QUANTUM-IR
ZMN-COMP-DIALECT
ZMN-COMP-ABI
ZMN-COMP-RUNTIME
ZMN-COMP-TARGET-CAPABILITY
ZMN-COMP-RESOURCE

over an unqualified:

incompatible

---

3. Compatibility Levels

Zamani implementations SHOULD expose compatibility as a multidimensional result.

A convenient conformance scale is:

Level| Meaning
C0| no compatibility established
C1| lexical compatibility
C2| source/syntax compatibility
C3| AST compatibility
C4| semantic compatibility
C5| canonical IR compatibility
C6| artifact compatibility
C7| runtime compatibility
C8| target/execution compatibility
C9| complete compatibility under the declared contract

A higher level MUST NOT be inferred merely because a lower-level representation happens to match.

For example:

C4 semantic compatibility

does not imply:

C8 target compatibility

because a target may lack required capabilities or resources.

---

4. Language Version and Compatibility Version Are Different

"grammar/specification/language-version.md" owns the language-version model.

This file owns how versions interact.

Therefore:

language-version.md
    =
how Zamani versions its language contract

compatibility.md
    =
how versions interoperate

Neither file may silently redefine the other's responsibility.

---

5. Version Dimensions

Zamani MUST independently distinguish:

Language Version
Lexical Contract Version
Grammar Representation Version
AST Contract Version
Semantic Model Version
Type-System Version
Effect-System Version
Resource/Capability Contract Version
Classical IR Version
Quantum IR Version
Hardware IR Version
Dialect Version
Artifact Format Version
Serialization Version
ABI Version
Compiler Version
Runtime Version
Target Descriptor Version
Backend Version
Toolchain Version

Changing one does not automatically change all others.

For example:

new compiler
    !=
new language version

and:

new GPU backend
    !=
new language semantics

and:

new quantum::ir representation
    !=
new source syntax

unless the corresponding public contract actually changed.

---

6. Compatibility Classes

Every language feature MUST have an explicit status and compatibility classification.

Allowed classifications:

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

6.1 STABLE

A stable feature:

- is normatively specified;
- has a defined semantic meaning;
- has an AST representation where applicable;
- has a semantic representation;
- has a defined IR path where applicable;
- has diagnostics;
- has conformance tests;
- has compatibility rules;
- is suitable for production use.

---

6.2 COMPATIBLE_EXTENSION

A compatible extension adds functionality without changing the meaning of existing valid programs.

Examples:

- new unambiguous syntax;
- new capability;
- new backend;
- new target;
- new optimization;
- new interoperable format;
- new domain facility.

The extension MUST undergo ambiguity and semantic-compatibility testing.

---

6.3 EXPERIMENTAL

Experimental features:

- MAY change;
- MAY be removed;
- MUST be explicitly marked;
- MUST be versioned or feature-gated;
- MUST NOT silently become stable;
- MUST NOT silently alter stable syntax.

---

6.4 DEPRECATED

A deprecated feature remains available under its compatibility contract but has a documented replacement.

Deprecation MUST identify:

- first deprecated version;
- reason;
- replacement;
- semantic differences;
- migration procedure;
- warning/diagnostic;
- earliest permitted removal version.

---

6.5 REMOVED

A removed feature is not valid in the language version where removal applies.

Removed syntax MUST NOT be silently reinterpreted as another construct.

It MUST produce a deterministic diagnostic when encountered in a context where the old construct would otherwise have been recognized.

---

6.6 RESERVED

Reserved syntax is intentionally unavailable for ordinary use.

It exists to preserve future evolution space.

Reserved syntax MUST NOT accidentally become valid because a parser rule happens to match it.

---

6.7 IMPLEMENTATION_DEFINED

The specification permits more than one implementation behavior and requires the implementation to document its selected behavior.

Implementation-defined behavior MUST NOT be confused with unspecified behavior.

---

6.8 TARGET_DEFINED

A behavior is target-defined when it depends on an explicitly selected target contract.

Target-defined behavior MUST NOT silently alter portable Zamani semantics.

---

6.9 DIALECT_DEFINED

A behavior belongs to an explicitly selected dialect.

A dialect MUST identify:

- name;
- version;
- namespace;
- owner;
- syntax extensions;
- semantic extensions;
- AST mapping;
- IR mapping;
- compatibility contract.

---

7. Stable-Feature Completion Gate

A feature MUST NOT be classified "STABLE" merely because its grammar rule exists.

The production completion path is:

Specification
    ↓
Lexical Contract
    ↓
Grammar
    ↓
Lexer
    ↓
Parser
    ↓
AST
    ↓
Structural Validation
    ↓
Name Resolution
    ↓
Type Validation
    ↓
Effect Validation
    ↓
Capability/Resource Validation
    ↓
Semantic Validation
    ↓
Canonical Semantic Model
    ↓
IR Lowering
    ↓
IR Verification
    ↓
Compiler Integration
    ↓
Runtime/Target Integration
    ↓
Positive Tests
    ↓
Negative Tests
    ↓
Boundary Tests
    ↓
Scalability Tests
    ↓
Determinism Tests
    ↓
Compatibility Tests
    ↓
Documentation
    ↓
STABLE

Failure at any required stage means the feature MUST NOT be advertised as fully stable.

---

8. No Phantom Features

Documentation MUST NOT imply that an unsupported feature is implemented.

Every documented feature MUST be classifiable as:

STABLE
IMPLEMENTED
SPECIFIED_NOT_IMPLEMENTED
EXPERIMENTAL
DEPRECATED
REMOVED
RESERVED
DIALECT_DEFINED
TARGET_DEFINED

There MUST NOT be a hidden state equivalent to:

documented but silently unsupported

"grammar/Zamani-Grammar.md" is especially subject to this rule because it contains broader design and aspirational material.

---

9. No Phantom Syntax

A parser MUST NOT accept syntax that cannot be represented correctly downstream.

Mandatory invariant:

accepted source
    ↓
AST

Every accepted construct MUST have a structural representation.

Then:

AST
    ↓
semantic analysis

Every semantically valid construct MUST have a defined interpretation.

Then:

semantic model
    ↓
IR

Every construct crossing an IR boundary MUST:

1. lower correctly; or
2. produce a structured unsupported-feature diagnostic before code generation.

It MUST NOT disappear.

---

10. No Silent Semantic Loss

The following is prohibited:

source
  ↓
parser accepts
  ↓
AST partially stores information
  ↓
IR generation drops information
  ↓
program meaning changes

Examples include:

- ignored quantum controls;
- ignored adjoints;
- ignored measurement modifiers;
- dropped effect information;
- dropped capability requirements;
- dropped resource constraints;
- dropped type qualifiers;
- dropped ownership information;
- dropped source spans required by diagnostics;
- silently discarded attributes;
- unknown quantum operations emitted as comments;
- unsupported operations converted to no-ops;
- hardware requirements silently discarded;
- unsupported dialect constructs silently erased.

If information contributes to semantics, compatibility, diagnostics, security, provenance, or reproducibility, it MUST survive until the subsystem responsible for interpreting it.

---

11. Source Compatibility

Source compatibility means that a source program remains valid and semantically equivalent under the applicable compatible language contract.

11.1 Backward source compatibility

A newer compiler accepts older source according to its declared compatibility policy.

old source
    ↓
new compiler

---

11.2 Forward source compatibility

An older compiler MAY understand newer source only where the newer language contract explicitly guarantees such compatibility.

Zamani MUST NOT assume universal forward compatibility.

---

11.3 Cross-implementation compatibility

Independent implementations MUST be capable of accepting the same source and producing equivalent semantics when they claim conformance to the same language contract.

---

12. Source Compatibility Invariant

For a compatible change:

Program P
+
Language Contract V

and:

Program P
+
Compatible Contract V'

MUST preserve the specified semantics of "P".

If that cannot be guaranteed, the change MUST be classified as breaking or version-gated.

---

13. Breaking Changes

A change is breaking if it can cause an existing stable program to:

- fail lexical analysis;
- fail parsing;
- change parse structure in a semantically relevant way;
- fail type checking;
- change type meaning;
- change ownership;
- change effects;
- change resource semantics;
- change capability requirements;
- change execution order;
- change observable behavior;
- change concurrency semantics;
- change synchronization guarantees;
- change quantum state evolution;
- change measurement semantics;
- change classical feed-forward;
- change hardware intent;
- change distributed consistency;
- change security guarantees;
- change deterministic behavior where determinism is specified;
- produce a materially different result.

Breaking changes MUST NOT be hidden inside a compatible patch release.

---

14. Non-Breaking Changes

A change MAY be compatible when it:

- adds unambiguous syntax;
- adds a backend;
- adds a target;
- adds a simulator;
- adds an optimization;
- improves performance without changing semantics;
- improves diagnostics;
- fixes an implementation bug without contradicting the normative specification;
- adds an interoperable format;
- adds a capability;
- adds a resource expression without changing existing semantics;
- improves scalability;
- improves compilation performance;
- adds target lowering;
- adds serialization while preserving semantic meaning;
- refactors grammar organization without changing accepted semantics.

Every such change MUST still pass compatibility validation.

---

15. Grammar Compatibility

"grammar/Zamani.g4" is the canonical ANTLR grammar representation.

It MUST:

- conform to the normative specification;
- implement the declared language version;
- avoid undocumented syntax;
- avoid target-dependent parsing;
- avoid artificial machine limits;
- preserve semantic information;
- remain deterministic;
- preserve required source spans;
- reject removed syntax appropriately;
- remain compatible with the Rust frontend;
- pass grammar conformance tests.

A grammar refactoring that does not change the accepted language or its semantics MAY be implementation-compatible.

Examples:

split one grammar into imported components
rename internal ANTLR rule
factor duplicated production
improve error recovery
reorganize grammar files

are not automatically language-version changes.

---

16. "grammar/Zamani.g4" and Modular Grammar Files

Modular grammar files under:

grammar/core/
grammar/types/
grammar/expressions/
grammar/statements/
grammar/declarations/
grammar/functions/
grammar/modules/
grammar/effects/
grammar/memory/
grammar/concurrency/
grammar/classical/
grammar/quantum/
grammar/hybrid/
grammar/hdl/
grammar/hardware/
grammar/distributed/
grammar/ai/
grammar/data/
grammar/networking/
grammar/security/
grammar/resources/
grammar/compile/
grammar/execution/
grammar/interoperability/
grammar/dialects/
grammar/macros/
grammar/metaprogramming/

MUST be representations of one language contract.

A new subdirectory MUST NOT become an independent authority.

Every modular grammar feature MUST identify:

Specification
Lexer Contract
Grammar Rule
AST Contract
Semantic Contract
IR Contract
Compiler Consumer
Runtime Consumer
Tests
Compatibility

---

17. "grammar/grammar.md"

"grammar/grammar.md" MUST remain an implementation-conformance reference.

It MUST distinguish at least:

SPECIFIED
LEXER_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
VERIFIER_IMPLEMENTED
BACKEND_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL
DEPRECATED
REMOVED

It MUST NOT turn implementation limitations into language rules.

For example:

implementation currently supports N resources

MUST NOT become:

Zamani supports at most N resources

unless the limitation is genuinely semantic and intentionally specified.

---

18. "grammar/Zamani-Grammar.md"

"grammar/Zamani-Grammar.md" MUST remain useful as broad design/reference material.

Every construct described there MUST have an explicit status.

Recommended statuses:

NORMATIVE
STABLE
EXPERIMENTAL
PROPOSED
ASPIRATIONAL
HISTORICAL
DEPRECATED
REMOVED
IMPLEMENTATION_DEFINED
DIALECT_DEFINED
TARGET_DEFINED

Appearance in this file MUST NOT automatically make syntax valid.

Promotion requires:

Design
 ↓
Specification
 ↓
Compatibility analysis
 ↓
Lexical contract
 ↓
Grammar
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

19. Lexical Compatibility

"grammar/spec/lexical.md" owns lexical semantics.

Compatibility requires agreement between:

canonical lexical specification
        │
        ├── src/lexer.rs
        └── ANTLR lexer representation

The following MUST remain compatible:

- token boundaries;
- identifiers;
- keywords;
- contextual keywords;
- literals;
- operators;
- punctuation;
- comments;
- source encoding;
- source spans;
- invalid-input handling.

The lexer MUST NOT depend on:

- CPU architecture;
- CPU count;
- GPU availability;
- QPU availability;
- FPGA availability;
- OS locale;
- wall-clock time;
- random state;
- network state;
- target topology.

---

20. Token Evolution

Token changes MUST be checked for:

- lexical ambiguity;
- parser ambiguity;
- precedence changes;
- identifier compatibility;
- keyword collisions;
- formatter behavior;
- syntax highlighting;
- IDE behavior;
- macro behavior;
- dialect behavior;
- source compatibility.

Known overlapping token concepts MUST be resolved through the canonical lexical authority rather than independently duplicated.

Examples requiring centralized treatment include:

Question / QuestionMark
BitAnd / Ampersand
BitOr / Pipe
Arrow / ThinArrow

The exact canonical token identity belongs to the lexical contract.

---

21. Keyword Compatibility

A new globally reserved keyword can break existing identifiers.

Therefore new vocabulary SHOULD preferentially be:

- contextual;
- namespace-qualified;
- attribute-based;
- identifier-based;
- dialect-scoped;

when language semantics permit.

This is especially important for:

- quantum gates;
- mathematical functions;
- AI algorithms;
- accelerator operations;
- hardware names;
- vendor operations;
- cryptographic algorithms;
- networking protocols.

The language MUST NOT become a finite dictionary of every operation available in computing.

---

22. Literal Compatibility

Literal syntax MUST remain independent of host representation.

For numeric literals:

source spelling
    ↓
lexical representation
    ↓
semantic literal
    ↓
typed value

The lexer MUST NOT prematurely constrain literals to:

u32
u64
i32
i64
usize

merely because those are host representations.

A sufficiently large integer literal MUST remain lexically representable when its spelling conforms to the language.

Overflow and representability are later semantic/type decisions.

---

23. Unicode Compatibility

The canonical source encoding is UTF-8.

Compatibility requires:

- deterministic UTF-8 validation;
- stable identifier rules;
- stable keyword comparison;
- source-spelling preservation;
- explicit Unicode-version policy;
- no silent normalization that changes identity.

If identifier normalization is ever introduced, it MUST be explicitly versioned.

---

24. Parser Compatibility

Parser compatibility requires that the same valid source be assigned the same syntactic structure under compatible language versions.

A parser MUST NOT silently change:

- precedence;
- associativity;
- binding;
- block structure;
- pattern interpretation;
- declaration interpretation;
- expression interpretation;
- quantum operation structure;
- HDL structure.

Parser refactoring is allowed when semantic interpretation remains unchanged.

---

25. Precedence Compatibility

Operator precedence is public language behavior.

A compatible release MUST NOT silently alter:

- precedence;
- associativity;
- unary/postfix binding;
- call binding;
- indexing;
- member access.

The canonical precedence contract belongs to "grammar/spec/syntax.md".

Any precedence change MUST be treated as a compatibility event and tested against old source.

---

26. AST Compatibility

The frontend AST is a structural bridge, not the final semantic IR.

AST migrations MUST preserve all semantically meaningful information.

At minimum where applicable:

- source spans;
- names;
- paths;
- declarations;
- types;
- generic parameters;
- constraints;
- effects;
- capabilities;
- resource requirements;
- attributes;
- modifiers;
- control flow;
- quantum intent;
- HDL intent;
- interoperability metadata;
- version metadata.

AST implementation details MAY change if the canonical semantic information remains representable.

---

27. AST Migration Rule

When an existing AST is decomposed or replaced:

legacy AST
    ↓
lossless compatibility adapter
    ↓
canonical frontend AST
    ↓
semantic analysis

A migration MUST NOT create two competing meanings.

The compatibility adapter MUST be lossless for all supported semantics.

If lossless conversion is impossible, the migration is a semantic compatibility change and MUST be versioned accordingly.

---

28. Semantic Compatibility

Semantic compatibility has priority over representation compatibility.

A compatible implementation MUST preserve:

- evaluation meaning;
- type meaning;
- ownership meaning;
- effect meaning;
- resource meaning;
- capability meaning;
- concurrency meaning;
- synchronization meaning;
- error behavior;
- quantum meaning;
- hardware intent;
- distributed meaning;
- security guarantees.

Optimization is permitted only when semantics are preserved.

---

29. Classical Compatibility

Classical computation MUST preserve its specified:

- numerical semantics;
- type semantics;
- evaluation semantics;
- memory semantics;
- ownership semantics;
- concurrency semantics;
- synchronization semantics;
- exception/error behavior;
- determinism guarantees;
- observable side effects.

Changing from:

CPU

to:

GPU

or:

accelerator

MUST NOT silently redefine the program.

---

30. Quantum Compatibility

Quantum compatibility MUST preserve, where applicable:

- logical qubit identity;
- register identity;
- operation identity;
- operation ordering;
- parameter meaning;
- control semantics;
- adjoint/inverse semantics;
- measurement semantics;
- reset semantics;
- observable semantics;
- state evolution;
- entanglement semantics;
- probability semantics;
- classical feed-forward;
- dynamic-circuit semantics;
- resource requirements;
- error-model intent.

Quantum source syntax MUST NOT create a second quantum semantic IR.

The canonical quantum semantic boundary remains:

Zamani source
    ↓
frontend AST
    ↓
semantic quantum model
    ↓
quantum::ir

Then:

quantum::ir
    ↓
optimization
    ↓
routing
    ↓
scheduling
    ↓
QEC/resilience
    ↓
ZQN
    ↓
HAL
    ↓
target realization

A backend MAY reject a program because a target lacks a required quantum capability.

It MUST NOT silently change the requested quantum computation to fit the target.

---

31. Quantum Gate and Operation Compatibility

The language MUST NOT establish a universal finite list of quantum operations as the semantic limit.

The following should remain possible where semantically supported:

H
X
CNOT
custom_operation
vendor.operation
parameterized_operation
logical_operation
future_operation

Operation identity MUST be represented generically enough to support extension without changing the canonical semantic model.

Adding a new physical gate MUST NOT require changing the core language merely because the hardware vendor introduced it.

---

32. Quantum Resource Compatibility

The compatibility model MUST NOT encode limits such as:

maximum qubits
maximum logical qubits
maximum physical qubits
maximum controls
maximum circuit depth
maximum quantum registers
maximum measurement count

as universal language ceilings.

These are distinct from:

program requirements
target capabilities
compiler budgets
runtime resources
physical constraints

For example:

requires qubits >= n

is semantic intent.

It is not equivalent to:

Zamani supports at most N qubits

---

33. QEC, ZQN, Routing, Scheduling, and HAL Compatibility

Compatibility MUST preserve subsystem ownership.

Subsystem| Compatibility responsibility
Quantum grammar| express quantum intent
"quantum::ir"| canonical quantum semantics
QEC| preserve specified error-correction intent
ZQN| preserve fault/noise semantics
Routing| preserve logical computation while selecting physical realization
Scheduling| preserve ordering/dependency semantics
HAL| expose target capability/state
Optimization| preserve semantics while transforming representation

A change in routing or scheduling MUST NOT be classified as a language semantic change merely because physical execution changes.

A change that alters program meaning MUST be classified as semantic and versioned accordingly.

---

34. HDL Compatibility

HDL and hardware/software co-design syntax MUST preserve:

- module meaning;
- port meaning;
- signal meaning;
- register meaning;
- combinational semantics;
- sequential semantics;
- clock semantics;
- reset semantics;
- timing constraints;
- interface semantics;
- state-machine behavior;
- verification properties;
- parameterization.

HDL compatibility MUST NOT establish universal fixed widths or counts unless the width/count is itself part of the source semantics.

For example:

parameterized width

is compatible with scalability.

A universal grammar limit such as:

all registers <= 32 bits

is not.

---

35. Hardware Compatibility

Hardware descriptions MUST distinguish:

semantic requirement
capability requirement
constraint
preference
hint
physical realization

For example:

requires capability("tensor.compute")

is not the same as:

use GPU 0

and:

requires qubits >= n

is not the same as:

map q0 -> physical_qubit(17)

The latter belongs to downstream realization.

---

36. Resource Compatibility

Resource compatibility MUST distinguish:

required
available
preferred
optional
budgeted
discovered
allocated
realized

A resource shortage MUST NOT be reported as a syntax incompatibility.

For example:

valid program
+
target lacks required resources
=
resource/target compatibility failure

not:

invalid Zamani

---

37. POCO-REAF Compatibility

POCO-REAF requires source-level semantic portability.

The model is:

Program Once
      ↓
Compile Once
      ↓
Portable semantic/compiled representation
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

This does not mean that one historical native machine-code binary is guaranteed to execute natively on every future architecture.

Instead, long-lived artifacts MUST preserve enough information for:

- validation;
- compatibility checking;
- migration;
- re-lowering;
- target realization;
- provenance;
- semantic preservation.

---

38. "Forever" Compatibility

"Forever" MUST mean semantic continuity and recoverability, not immutable implementation details.

A long-lived artifact SHOULD identify:

language version
grammar compatibility
semantic model version
IR version
dialect versions
capability requirements
resource requirements
artifact format
compiler provenance
target-independent intent

A future implementation SHOULD be able to determine what the artifact means before attempting execution.

---

39. Scalability Compatibility

Compatibility MUST NOT introduce artificial upper bounds on:

- source size;
- declaration count;
- statement count;
- function count;
- module count;
- generic parameters;
- array dimensions;
- tensor rank;
- tensor dimensions;
- classical values;
- logical qubits;
- physical qubits;
- quantum operations;
- resources;
- nodes;
- processes;
- tasks;
- channels;
- agents;
- timelines;
- ports;
- HDL instances;
- accelerators;
- devices.

An implementation MAY have resource budgets.

Those budgets MUST be represented as implementation/resource policy, not language semantics.

For example:

compiler memory budget exhausted

MUST NOT become:

source language maximum reached

unless the language specification actually defines such a limit.

---

40. "Nothing Must Be Hard Coded"

This rule means:

«Do not turn today's implementation limits into tomorrow's language semantics.»

It does NOT prohibit ordinary program constants.

Valid:

let n = 1024;
allocate n;

Invalid as a universal language limitation:

MAX_ELEMENTS = 1024

when it defines the maximum representable computation.

The rule applies to:

- qubits;
- CPUs;
- cores;
- threads;
- GPUs;
- FPGAs;
- QPUs;
- memory;
- storage;
- nodes;
- devices;
- tensor dimensions;
- vector widths;
- registers;
- timelines;
- agents;
- processes;
- channels;
- network participants;
- accelerator counts.

---

41. Resource Availability Is Not Source Compatibility

A program can be valid while a particular target is incapable of executing it.

Therefore:

language validity
    !=
target compatibility
    !=
resource feasibility

Example:

requires qubits >= n

may be valid on every target as a source-level statement.

A target with insufficient resources produces:

ZMN-COMP-RESOURCE

or:

ZMN-COMP-TARGET-CAPABILITY

rather than changing the source's validity.

---

42. Capability Compatibility

Capabilities MUST be represented independently from physical identity.

Prefer:

requires capability("quantum.mid_circuit_measurement")

over:

requires qpu("Vendor-X-Device-17")

when the program requires a capability rather than a particular device.

Target realization MAY discover a physical implementation satisfying the capability.

---

43. Requirement, Constraint, Preference, Hint, Realization

Compatibility MUST preserve the distinction:

Requirement
Constraint
Preference
Hint
Realization

A compiler MUST NOT reinterpret a preference as a requirement.

A compiler MUST NOT drop a requirement.

A compiler MAY ignore a non-semantic hint when documented.

A physical realization MUST NOT leak backward into portable source semantics.

---

44. Distributed Compatibility

Distributed programs MUST preserve:

- message semantics;
- ordering guarantees;
- synchronization semantics;
- consistency guarantees;
- transaction semantics;
- replication semantics;
- failure semantics;
- communication requirements.

Changing:

1 node

to:

many nodes

MUST NOT require source rewriting merely because the deployment scales.

Node count MUST NOT be a universal language limit.

---

45. Concurrency Compatibility

Compatibility MUST preserve specified:

- happens-before relationships;
- synchronization;
- task semantics;
- channel semantics;
- actor semantics;
- atomicity;
- ordering;
- determinism guarantees.

Changing the number of available workers MUST NOT silently change a program's specified semantics.

---

46. AI/Data Compatibility

AI and data constructs MUST preserve semantic meaning independently of framework or accelerator.

Changing:

CPU

to:

GPU

or:

TPU/accelerator

MUST NOT change the language meaning unless the program explicitly selects target-dependent semantics.

Framework-specific representations belong downstream or in interoperability contracts.

---

47. Dialect Compatibility

Every dialect MUST declare:

dialect name
dialect version
language compatibility range
syntax extensions
semantic extensions
AST mapping
IR mapping
capabilities
resource requirements
compatibility guarantees
migration policy

A dialect MUST NOT silently modify core Zamani semantics.

Two dialects using the same spelling for incompatible semantics MUST NOT be simultaneously active without explicit disambiguation.

---

48. Dialect Isolation

A dialect MUST NOT:

- redefine a stable core keyword silently;
- redefine stable operators silently;
- redefine type semantics silently;
- redefine quantum semantics silently;
- redefine resource semantics silently;
- bypass semantic validation;
- bypass IR verification;
- bypass security validation.

Dialect extensions MUST remain traceable to their owning specification.

---

49. Module and Package Compatibility

Modules and packages MUST declare compatible language/dialect requirements where necessary.

A dependency MAY specify conceptually:

requires Zamani >= 1.2 < 2.0

but exact dependency syntax belongs to the module/package specification.

Dependency resolution MUST NOT silently select semantically incompatible versions.

A package's source compatibility MUST be evaluated independently from the compiler binary version.

---

50. Interoperability Compatibility

Interoperability formats such as:

- OpenQASM;
- QIR;
- HDL formats;
- C;
- C++;
- Rust;
- Python;
- WebAssembly;
- other external representations;

are interoperability boundaries.

They MUST NOT silently become the canonical Zamani semantic model.

The pipeline remains:

external representation
    ↓
interop importer/exporter
    ↓
Zamani semantic representation

or:

Zamani semantic representation
    ↓
interop lowering
    ↓
external representation

Information that cannot be represented losslessly MUST produce an explicit diagnostic or be represented through an explicitly defined approximation contract.

---

51. Serialization Compatibility

Serialized artifacts MUST identify their format version.

Serialization compatibility MUST distinguish:

format compatibility
semantic compatibility

A newer serializer MAY change its binary/text representation while preserving semantics if the format contract allows it.

An incompatible serialized artifact MUST fail deterministically with a structured diagnostic.

---

52. Artifact Compatibility

A production artifact SHOULD carry sufficient metadata to establish:

language version
semantic contract version
IR version
dialect versions
artifact format version
capability requirements
resource requirements
provenance
compiler/toolchain information
compatibility profile

Artifact metadata MUST NOT depend solely on the compiler executable version.

---

53. ABI Compatibility

ABI compatibility is separate from language compatibility.

An ABI change MUST NOT automatically imply a source-language change.

ABI compatibility includes, where applicable:

- calling convention;
- data layout;
- alignment;
- symbol conventions;
- exception/error ABI;
- foreign-function boundary;
- object representation.

ABI details belong to the relevant target/interop contract.

---

54. Runtime Compatibility

Runtime compatibility requires preservation of the runtime semantics promised by the language contract.

A runtime MAY evolve:

- scheduling;
- memory allocation;
- caching;
- device selection;
- transport;
- optimization;
- observability;

provided specified program semantics remain intact.

A runtime MUST report unavailable required capabilities rather than silently weakening program semantics.

---

55. Target Compatibility

Target compatibility MUST answer:

Can this target realize the required semantics?

It MUST NOT answer merely:

Can this target parse the source?

Target compatibility SHOULD evaluate:

- required capabilities;
- resource requirements;
- ABI;
- runtime contract;
- supported IR;
- supported dialects;
- numerical guarantees;
- quantum capabilities;
- hardware constraints;
- security requirements;
- deployment requirements.

---

56. Target Failure

A target failure MUST NOT be disguised as a source-language error.

Examples:

target lacks capability
target lacks resources
target lacks ABI support
target lacks runtime support
target lacks required quantum operation
target lacks required HDL realization

must remain distinct from:

invalid syntax
invalid type
invalid identifier

---

57. Semantic Preservation Across Optimization

An optimization is compatible only if it preserves the applicable semantic contract.

The optimizer MAY change:

- instruction selection;
- operation ordering where permitted;
- scheduling;
- routing;
- representation;
- memory layout;
- decomposition;
- parallelization.

It MUST NOT change:

- specified observable behavior;
- required numerical semantics;
- quantum measurement semantics;
- required ordering;
- ownership guarantees;
- security guarantees;
- resource semantics.

---

58. Quantum Optimization Compatibility

Quantum optimization MUST preserve the canonical quantum semantics.

Examples of permissible transformations include, when proven valid:

gate cancellation
gate fusion
commutation
decomposition
routing
scheduling
pulse lowering

provided they preserve the specified computation.

Physical optimization MUST NOT silently change:

measurement meaning
classical feed-forward
logical state semantics
required error guarantees

---

59. Hardware Mapping Compatibility

A compiler MAY transform:

logical resource
    ↓
physical resource

without changing source semantics.

For quantum:

logical qubit
    ↓
physical qubit

For classical:

logical task
    ↓
CPU/GPU/accelerator resource

For distributed:

logical service
    ↓
physical node

The mapping is a realization decision unless the source explicitly specifies physical behavior.

---

60. Source-Level Physical Identifiers

Physical identifiers MUST NOT become universal semantic identifiers.

A source MAY explicitly use target-specific functionality when a target-specific dialect or execution contract permits it.

However, target-specific constructs MUST be visibly target-dependent and MUST NOT masquerade as portable source semantics.

---

61. Diagnostics Compatibility

Diagnostics are part of the tooling contract but are not generally equivalent to language semantics.

Diagnostics MUST be:

- deterministic;
- structured;
- source-located;
- reproducible;
- stable enough for tooling;
- independent of hash iteration order;
- independent of machine topology;
- associated with the relevant language/feature version where necessary.

Diagnostic wording MAY improve without being a language-breaking change, provided diagnostic identity and machine-readable classification remain sufficiently stable for tooling.

---

62. Diagnostic Identity

Where tooling compatibility matters, diagnostics SHOULD have stable identifiers.

Examples:

ZMN-COMP-LANGUAGE-VERSION
ZMN-COMP-SYNTAX-REMOVED
ZMN-COMP-SEMANTIC
ZMN-COMP-AST
ZMN-COMP-IR
ZMN-COMP-QUANTUM-IR
ZMN-COMP-DIALECT
ZMN-COMP-RESOURCE
ZMN-COMP-TARGET-CAPABILITY
ZMN-COMP-ABI
ZMN-COMP-RUNTIME
ZMN-COMP-SERIALIZATION

Text may change.

The semantic diagnostic classification SHOULD remain stable.

---

63. Error Recovery Compatibility

Error recovery MUST NOT manufacture a valid semantic program from invalid source.

Production compilation MUST distinguish:

invalid source

from:

IDE recovery representation

An IDE MAY construct an error-tolerant tree.

That tree MUST NOT be treated as a valid compilation AST.

---

64. Determinism

For identical:

source
language version
dialect configuration
compiler policy
relevant target-independent inputs

the compatibility decision MUST be deterministic.

Compatibility results MUST NOT depend on:

- hash iteration order;
- random seeds unless explicitly specified;
- wall-clock time;
- network state unless network state is an explicit input;
- physical machine topology;
- number of available CPUs;
- number of GPUs;
- number of QPUs;
- incidental device enumeration order.

---

65. Provenance

Compatibility-relevant artifacts SHOULD preserve provenance sufficient to determine:

language version
dialect versions
IR version
artifact version
compiler version
relevant specification revision
target descriptor
resource/capability requirements

Provenance MUST NOT be used to change source semantics.

---

66. Reproducibility

A reproducible build MUST identify all compatibility-relevant inputs.

At minimum where applicable:

- language version;
- dependency versions;
- dialect versions;
- compiler configuration;
- feature gates;
- source inputs;
- relevant target-independent compilation options;
- artifact format version.

Target-dependent realization MUST be distinguished from source-level reproducibility.

---

67. Compatibility and Resource Budgets

A compiler MAY define external budgets for:

- memory;
- compile time;
- diagnostics;
- parser work;
- optimization work;
- IR size;
- runtime memory;
- execution time.

These budgets MUST NOT be presented as language semantics.

For example:

compiler budget exceeded

is not:

program invalid

unless the budget is an explicit program constraint.

---

68. Compatibility and Infinite/Unbounded Constructs

Zamani may represent semantically unbounded constructs.

Compatibility MUST NOT impose artificial finite limits on:

- loops;
- streams;
- generators;
- channels;
- timelines;
- distributed nodes;
- agents;
- quantum operations;
- datasets;
- tensor dimensions.

Actual implementations MAY have finite resource budgets.

Those are implementation/runtime constraints.

---

69. Compatibility and Mathematical Domains

Adding a mathematical operation MUST NOT require changing core language compatibility merely because a library gains a new algorithm.

Prefer:

generic operation
+
typed semantic capability
+
library/intrinsic resolution

over:

one permanent keyword for every mathematical function

Existing mathematical syntax MUST retain its specified meaning.

---

70. Compatibility and AI/Data Domains

AI/data features MUST remain compatible with the universal semantic model.

Framework changes MUST NOT automatically constitute language changes.

For example:

model
tensor
dataset
training
inference
agent

may have stable language semantics while their implementation targets change from:

CPU
GPU
accelerator
distributed cluster
future hardware

---

71. Compatibility and Security

Security guarantees are semantic compatibility concerns.

A change that weakens a stable security guarantee MUST be treated as a compatibility event.

Security-sensitive compatibility MUST preserve:

- authorization semantics;
- identity semantics;
- capability semantics;
- secret-handling guarantees;
- cryptographic contract;
- isolation guarantees;
- provenance guarantees.

The compiler MUST NOT silently weaken security to make a target executable.

---

72. Safe Rust Requirement

The Rust implementation baseline is:

Rust 1.97
Rust 1.97.1
Edition 2021

Production compiler code MUST NOT use:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

The compatibility contract applies to language behavior, while the Rust safety rule applies to implementation.

The source-language spelling "unsafe" is a separate language-design concern and MUST NOT be confused with Rust implementation safety.

---

73. No Unsafe Rust Compatibility Escape Hatch

A compiler implementation MUST NOT introduce an "unsafe" implementation dependency merely to support a language feature.

If a feature cannot be implemented safely within the current architecture, the implementation MUST:

1. redesign the implementation;
2. use safe Rust abstractions;
3. isolate the unsupported capability;
4. or classify the feature as unsupported/experimental.

It MUST NOT silently violate the repository's safety requirement.

---

74. Compatibility of "unsafe" Source Syntax

If the Zamani source language retains an "unsafe" spelling for compatibility reasons, its meaning MUST be explicitly specified.

Parsing the spelling MUST NOT imply that unrestricted unsafe execution is implemented.

If the final stable language disallows unrestricted unsafe execution, the spelling SHOULD become:

deprecated
reserved
or explicitly rejected

according to the applicable language version.

This MUST be coordinated with:

grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/specification/language-version.md
grammar/compatibility/deprecated.md
grammar/compatibility/reserved.md
src/lexer.rs
src/parser.rs

---

75. Deprecation

Deprecation MUST be coordinated across the repository.

When a feature is deprecated, update as applicable:

grammar/spec/compatibility.md
grammar/compatibility/deprecated.md
grammar/compatibility/versions.md
grammar/compatibility/migrations.md
grammar/compatibility/compatibility-matrix.md
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/specification/language-version.md
grammar/tests/
src/lexer.rs
src/parser.rs
src/frontend/ast/
semantic implementation
IR lowering
tooling
documentation

The feature MUST NOT be removed from one layer while remaining silently accepted by another.

---

76. Removal

Removal MUST be a repository-wide compatibility event.

Before removal:

1. identify the feature;
2. identify all consumers;
3. identify source compatibility impact;
4. provide migration;
5. update tests;
6. update diagnostics;
7. update specification;
8. update grammar;
9. update lexer/parser;
10. update AST;
11. update semantics;
12. update IR;
13. update tooling;
14. update documentation;
15. define the removal language version.

---

77. Migration

Every breaking change SHOULD have an explicit migration path.

Migration documentation MUST identify:

old construct
new construct
semantic differences
source transformation
manual migration requirements
compatibility window
first affected version

Automated migration MAY be provided.

Automated migration MUST NOT silently change semantics.

---

78. Compatibility Matrix

The repository's "grammar/compatibility/compatibility-matrix.md" remains the detailed feature matrix.

This file defines what that matrix MUST track.

Each feature SHOULD have:

Dimension| Required status
Specification| required
Lexical| required where applicable
Grammar| required
Parser| required
AST| required
Semantic| required
Type| required where applicable
Effect| required where applicable
Resource| required where applicable
Capability| required where applicable
IR| required where applicable
IR verification| required where applicable
Compiler| required
Runtime| required where applicable
Backend| required where applicable
Tests| required
Version| required
Compatibility| required

A feature MUST NOT be marked "STABLE" while required dimensions remain unresolved.

---

79. Feature Manifest Integration

Where feature manifests exist or are introduced under the specification system, each compatibility-sensitive feature SHOULD identify:

feature_id
name
status
introduced_version
deprecated_version
removed_version
grammar_rules
lexer_tokens
ast_nodes
semantic_rules
ir_mapping
compiler_consumers
runtime_consumers
capabilities
resources
positive_tests
negative_tests
boundary_tests
scalability_tests
determinism_tests
compatibility_tests
migration

This makes each feature independently completable.

A developer working on one feature MUST be able to determine its compatibility obligations without waiting for an unrelated feature to be designed.

---

80. Independent File Completion Contract

Every compatibility-sensitive file MUST identify:

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
Determinism Tests
Compatibility Tests
Migration
Diagnostics
Security
Hard-Coding Audit
Completion Criteria

No file should be considered complete merely because its local syntax/documentation is finished.

---

81. No Re-Edit Dependency Principle

A feature's compatibility contract MUST be established before dependent implementation work begins.

The intended workflow is:

independent contract
    ↓
independent implementation
    ↓
independent tests
    ↓
integration contract
    ↓
integration validation

rather than:

file A
    ↓
file B discovers missing information
    ↓
edit A
    ↓
file C discovers another missing contract
    ↓
edit A again

Integration details MUST therefore be declared in advance.

---

82. Cross-Domain Compatibility

All domains share the same language foundation.

Conceptually:

                    Zamani
                       │
       ┌───────────────┼────────────────┐
       │               │                │
   Classical        Quantum            HDL
       │               │                │
       └───────────────┼────────────────┘
                       │
                     Hybrid
                       │
       ┌───────────────┼────────────────┐
       │               │                │
      AI             Data          Distributed
       │               │                │
       └───────────────┼────────────────┘
                       │
                 Common Semantics
                       │
              Capability / Resource
                       │
                Canonical IR
                       │
        ┌──────────────┼──────────────┐
        │              │              │
    Classical      quantum::ir     Hardware
        │              │              │
        └──────────────┼──────────────┘
                       │
              Optimization / Lowering
                       │
             Routing / Scheduling
                       │
                  Resilience/QEC
                       │
                      ZQN
                       │
                      HAL
                       │
                 Target realization

A domain extension MUST NOT create a parallel language.

---

83. Canonical Quantum Boundary

"quantum::ir" remains the canonical quantum semantic boundary.

The compatibility contract therefore prohibits:

grammar quantum IR
+
frontend quantum IR
+
quantum::ir

when they represent the same semantic layer.

The preferred flow is:

Zamani syntax
    ↓
generic/domain-neutral AST
    ↓
semantic quantum representation
    ↓
quantum::ir

Compatibility adapters MUST be lossless.

---

84. IR Compatibility

IR compatibility MUST distinguish:

source semantics

from:

IR representation

An IR may change representation while preserving source semantics.

However, if an IR change causes information loss, the change is not merely representational.

IR versions MUST therefore document:

- schema;
- semantics;
- required invariants;
- supported source constructs;
- serialization;
- verification;
- migration;
- compatibility range.

---

85. "quantum::ir" Compatibility

Changes to "quantum::ir" MUST preserve the semantic contract unless explicitly versioned.

Compatibility MUST cover:

- operation identity;
- operands;
- parameters;
- results;
- controls;
- adjoints;
- measurement;
- reset;
- ordering;
- classical dependencies;
- resource requirements;
- error semantics;
- source provenance where required.

A new quantum IR representation MUST NOT require source-language changes unless source semantics themselves changed.

---

86. IR Verification

An IR consumer MUST reject malformed or semantically impossible IR deterministically.

No IR transformation may silently:

- drop operations;
- drop controls;
- drop dependencies;
- drop resource requirements;
- drop error semantics;
- drop source provenance required by the contract.

IR verification is part of compatibility assurance.

---

87. Compiler Compatibility

Compiler versions MAY change implementation algorithms.

Examples:

new optimizer
new scheduler
new parser implementation
new register allocator
new routing algorithm
new QEC strategy
new backend

do not automatically imply a language-version change.

They become language-breaking only if they change specified program semantics.

---

88. Runtime Compatibility

Runtime changes MUST preserve the language's runtime contract.

Runtime MAY change:

- scheduling;
- allocation;
- transport;
- caching;
- device selection;
- observability;
- execution strategy.

Runtime MUST NOT silently weaken explicit program guarantees.

---

89. Backend Compatibility

A new backend is normally a compatible extension if:

- it preserves language semantics;
- it reports unsupported capabilities correctly;
- it does not reinterpret portable constructs;
- it respects resource requirements;
- it respects safety/security constraints.

Backend-specific behavior MUST be explicitly identified as target-defined where applicable.

---

90. Physical Realization

Physical realization belongs after semantic compilation.

The following may vary:

physical CPU
physical GPU
physical FPGA
physical QPU
physical memory bank
physical network node
physical qubit
physical accelerator

without changing the portable program.

The compiler/runtime MAY select different physical resources on different executions.

---

91. Capability Negotiation

Capability negotiation MUST be separate from language compatibility.

Conceptually:

program requirements
       ↓
target capability discovery
       ↓
capability matching
       ↓
lowering/realization

A target may satisfy a requirement through:

- native support;
- decomposition;
- emulation;
- simulation;
- distribution;
- virtualization;
- another compatible implementation.

If the semantic contract permits such realization.

---

92. Approximation and Alternative Semantics

A compiler MUST NOT silently approximate a program to make it executable.

Approximation MUST be:

- explicitly requested;
- explicitly represented;
- semantically documented;
- compatibility-classified;
- diagnostically visible.

The same applies to:

- reduced precision;
- approximate quantum simulation;
- lossy numerical transformations;
- relaxed consistency;
- approximate optimization;
- reduced fidelity.

---

93. Compatibility of Numerical Semantics

Numerical changes MUST distinguish:

mathematical semantics
representation semantics
precision policy
target realization

Changing hardware floating-point support MUST NOT silently change a program's specified numerical guarantees.

Where implementation-defined numerical behavior exists, the applicable specification MUST define the compatibility contract.

---

94. Compatibility of Memory Semantics

Changes to:

- allocation;
- ownership;
- borrowing;
- address spaces;
- persistence;
- shared memory;
- distributed memory;
- accelerator memory;

MUST preserve the specified language semantics.

Physical memory size is not a language compatibility dimension unless explicitly expressed as a program requirement.

---

95. Compatibility of Concurrency

A compatible implementation MUST preserve specified:

- ordering;
- synchronization;
- atomicity;
- race-freedom guarantees;
- happens-before relationships;
- channel semantics;
- task semantics;
- actor semantics;
- deterministic execution guarantees.

Increasing or decreasing worker resources MUST NOT silently alter specified semantics.

---

96. Compatibility of Distributed Execution

Distributed realization MAY change:

node count
placement
routing
replication
transport
scheduling

while preserving program semantics.

A change to consistency, ordering, failure semantics, or security guarantees is a semantic compatibility event.

---

97. Compatibility of HDL/Hardware Co-Design

Changing the synthesis target:

FPGA
ASIC
simulation
emulation
accelerator
future hardware

MUST NOT change the source hardware intent unless the source explicitly requests target-specific behavior.

A synthesis limitation MUST be reported as a target/resource limitation.

---

98. Compatibility of Networking

Network compatibility MUST distinguish:

protocol semantics
endpoint capability
address representation
transport implementation
topology
deployment

Changing a physical address or network topology MUST NOT change the portable program's meaning unless the program explicitly depends on that topology.

---

99. Compatibility of Security and Cryptography

Cryptographic and security semantics MUST be preserved across compatible implementations.

Replacing a backend implementation MUST NOT silently downgrade:

- algorithm guarantees;
- key requirements;
- authentication semantics;
- authorization;
- isolation;
- provenance;
- confidentiality;
- integrity.

A target lacking a required security capability MUST fail compatibility validation.

---

100. Compatibility of Macros and Metaprogramming

Macro expansion MUST remain compatible with the language contract.

A macro MUST NOT bypass:

- syntax validation;
- type validation;
- effect validation;
- capability validation;
- resource validation;
- security validation;
- IR verification.

Changing macro expansion semantics is a compatibility event if existing programs can change meaning.

---

101. Compatibility of Reflection

Reflection/introspection MUST NOT expose implementation details as stable language semantics unless explicitly specified.

The language MUST distinguish:

stable semantic reflection

from:

implementation-specific inspection

The latter MUST be marked accordingly.

---

102. Compatibility of Tooling

Tooling includes:

- formatter;
- syntax highlighter;
- IDE;
- LSP;
- parser generators;
- documentation generators;
- static analyzers;
- linters;
- conformance tools.

Tooling MUST follow the canonical language contract.

Tooling MUST NOT independently introduce new source syntax.

---

103. Compatibility of Generated Artifacts

Generated artifacts MUST identify their source and contract where required.

Generated files MUST NOT silently become authoritative source specifications.

The relationship should be:

normative specification
        ↓
canonical implementation
        ↓
generated artifact

not:

generated artifact
        ↓
accidental authority

---

104. Compatibility of Examples and Fixtures

Examples MUST be version-aware when syntax is version-sensitive.

A stale example MUST NOT be used as evidence that a feature is currently stable.

Conformance fixtures MUST identify:

- language version;
- dialect;
- expected result;
- expected diagnostics where relevant;
- compatibility classification.

---

105. Compatibility Testing

Every compatibility-sensitive feature MUST have, where applicable:

positive tests
negative tests
boundary tests
scalability tests
determinism tests
migration tests
backward-compatibility tests
cross-version tests
cross-implementation tests
IR tests
diagnostic tests

---

106. Positive Compatibility Tests

Positive tests verify:

valid source
→ valid tokens
→ valid parse
→ valid AST
→ valid semantics
→ valid IR

where the feature's pipeline is implemented.

---

107. Negative Compatibility Tests

Negative tests verify that invalid or unsupported constructs:

- are rejected;
- produce deterministic diagnostics;
- are not silently reinterpreted;
- do not produce misleading semantic output.

---

108. Boundary Compatibility Tests

Boundary tests MUST exercise transitions such as:

small → large
single → many
classical → hybrid
logical → physical
local → distributed
CPU → accelerator
classical → quantum
quantum → classical feed-forward
software → hardware intent

without relying on arbitrary fixed limits.

---

109. Scalability Compatibility Tests

Scalability tests MUST establish that language semantics do not change when resource scale changes.

Examples:

1
many
larger
very large
resource-constrained
resource-rich

The exact test size is an implementation concern.

The semantic principle is that no artificial language ceiling is introduced.

---

110. Determinism Tests

For identical compatibility inputs:

source
language version
dialect configuration
relevant policy

the result MUST be deterministic.

Tests MUST detect dependence on:

- hash order;
- random state;
- machine topology;
- CPU count;
- device enumeration;
- wall-clock time.

---

111. Cross-Version Testing

For every compatible language transition:

old source
    ↓
old compiler

old source
    ↓
new compiler

must be compared for semantic equivalence where compatibility is promised.

Tests MUST distinguish:

expected migration failure

from:

unexpected incompatibility

---

112. Cross-Implementation Testing

Independent implementations claiming Zamani conformance SHOULD consume common conformance fixtures.

The comparison MUST be semantic rather than dependent on identical:

- AST structure;
- parser implementation;
- IR layout;
- optimization strategy.

---

113. Compatibility Test Oracle

The preferred test oracle is:

semantic result

not:

exact parser tree

unless parser structure itself is part of the public contract.

Similarly:

IR byte-for-byte equality

MUST NOT be required when multiple semantically equivalent IR representations are valid.

---

114. Compatibility and Source Spans

Source spans are part of frontend tooling compatibility.

Where diagnostics, IDE behavior, provenance, macros, or source transformations require spans, compatible implementations MUST preserve sufficient source location information.

A grammar refactor MUST NOT silently destroy required source locations.

---

115. Compatibility and Attributes

Attributes MUST follow:

lexer
 ↓
parser
 ↓
AST
 ↓
attribute validation
 ↓
semantic interpretation

A semantically meaningful attribute MUST NOT be parsed and silently discarded.

An intentionally non-semantic attribute MUST be documented as such.

---

116. Compatibility and Unknown Attributes

Unknown attributes MAY be:

- rejected;
- warned;
- preserved;
- accepted under an extension mechanism;

according to the applicable language/version/dialect contract.

The behavior MUST be deterministic.

A compiler MUST NOT silently reinterpret an unknown semantic attribute.

---

117. Compatibility and Feature Gates

Experimental features SHOULD use explicit feature gates or equivalent version/dialect mechanisms.

Feature gates MUST:

- have stable identifiers;
- be version-aware;
- be documented;
- produce deterministic diagnostics;
- not silently alter stable syntax.

A feature gate MUST NOT be used to bypass semantic validation.

---

118. Compatibility and Reserved Space

Reserved vocabulary exists to preserve future evolution.

Reserved syntax MUST NOT accidentally become available as an ordinary identifier when compatibility policy requires reservation.

Conversely, ordinary identifiers MUST NOT be unnecessarily reserved merely to anticipate hypothetical future features.

---

119. Compatibility and Future Computing

Future computing models MUST be introduced through the same compatibility framework.

A future domain MAY introduce:

- new syntax;
- new capabilities;
- new resources;
- new semantic constructs;
- new IR mappings;
- new target realization.

It MUST NOT require rewriting existing source merely because a new physical computing model exists.

---

120. Compatibility and Atom-to-Everywhere Scaling

The compatibility model applies equally to:

single value
single operation
single device
embedded system
CPU
multicore
GPU
FPGA
ASIC
QPU
hybrid system
cluster
HPC system
cloud
distributed fabric
heterogeneous system
future substrate

The language contract MUST remain independent of the scale at which the program executes.

---

121. Compatibility and Resource Discovery

Resource discovery belongs downstream.

The source program may declare:

requires capability(...)
requires resource(...)
requires constraint(...)

The compiler/runtime may discover:

available CPU
available GPU
available FPGA
available QPU
available memory
available network
available accelerator

Compatibility is determined by matching requirements against capabilities.

Source semantics MUST NOT be rewritten merely because discovery returns a different physical topology.

---

122. Compatibility and Physical Limits

Physical limits are real and MUST be respected.

However:

physical limit

is not automatically:

language limit

Examples:

QPU has insufficient physical qubits

does not imply:

Zamani cannot express larger quantum programs

Similarly:

target memory insufficient

does not imply:

Zamani source cannot describe a larger computation

unless the program's semantics explicitly require that execution resource.

---

123. Compatibility and Compiler Resource Limits

A compiler may fail because it exhausts its own resources.

Such failures MUST be distinguished from source incompatibility.

Examples:

compiler memory exhausted
compiler time budget exceeded
IR budget exceeded
optimization budget exceeded
diagnostic budget exceeded

These are implementation/resource failures.

They MUST NOT be represented as language semantic errors unless the source itself violates an explicit contract.

---

124. Compatibility and Runtime Resource Limits

Likewise:

runtime memory unavailable
target accelerator unavailable
QPU capacity unavailable
network resource unavailable

are runtime/target compatibility failures.

They MUST NOT cause silent semantic degradation.

---

125. Compatibility and Approximate Execution

Approximate execution MUST be explicit.

For example, a system MAY support an explicitly requested:

approximate

execution mode.

Such a mode MUST define:

- approximation class;
- permitted error;
- affected semantics;
- diagnostics;
- reproducibility;
- compatibility implications.

It MUST NOT be silently activated merely because a target is insufficient.

---

126. Compatibility and Deterministic Provenance

Compatibility decisions SHOULD be reproducible from declared inputs.

The system SHOULD record:

language version
dialect versions
feature gates
artifact version
IR version
target descriptor
capability set
resource requirements
compiler configuration

This supports long-lived POCO-REAF artifacts.

---

127. Compatibility Review Checklist

Before accepting a language change, reviewers MUST determine:

Source

- Does existing valid source retain its meaning?
- Does the change introduce ambiguity?
- Does it change keyword behavior?
- Does it change precedence?

Lexer

- Are token boundaries stable?
- Are identifiers affected?
- Are literals affected?

Parser

- Is parsing deterministic?
- Is the AST mapping defined?

AST

- Is every semantic field represented?
- Is source provenance preserved?

Semantics

- Is meaning unchanged?
- Are type/effect/resource rules defined?

IR

- Is canonical IR mapping defined?
- Is "quantum::ir" affected?
- Is migration required?

Compiler

- Are all consumers identified?
- Are optimizations semantics-preserving?

Runtime

- Are runtime requirements identified?

Target

- Is the change target-independent?
- If target-dependent, is that explicit?

Compatibility

- Is the change stable, additive, experimental, deprecated, or breaking?
- Is migration required?

Scalability

- Does it introduce an artificial resource limit?

Safety

- Does implementation remain safe Rust?

Tests

- Positive?
- Negative?
- Boundary?
- Scalability?
- Determinism?
- Compatibility?
- Migration?

---

128. Hard-Coding Audit

Every compatibility-sensitive change MUST pass a hard-coding audit.

Reject universal language limits such as:

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

Also reject universal fixed physical identities such as:

Qubit0
Qubit1
Qubit2
...

when used as the language's fundamental resource model.

Explicit program values remain valid.

---

129. Compatibility Audit of Existing Repository

Before declaring the grammar production-ready, the repository compatibility validator SHOULD verify:

grammar/specification/language-version.md
        ↕
grammar/spec/compatibility.md

grammar/spec/lexical.md
        ↕
src/lexer.rs
        ↕
ANTLR lexer

grammar/spec/syntax.md
        ↕
grammar/Zamani.g4
        ↕
src/parser.rs

src/parser.rs
        ↕
src/frontend/ast/

AST
        ↕
semantic analysis

semantic model
        ↕
canonical IR

quantum semantics
        ↕
quantum::ir

IR
        ↕
ir verification

IR
        ↕
compiler

compiler
        ↕
runtime

runtime
        ↕
HAL / targets

all of the above
        ↕
grammar/tests/

Any divergence MUST be classified rather than silently tolerated.

---

130. Compatibility Drift

Compatibility drift occurs when two repository components claim different contracts.

Examples:

spec says A
lexer accepts B
parser accepts C
AST stores D
semantic layer interprets E
IR produces F

This is a production defect.

The validator MUST report the first divergent boundary.

---

131. First-Divergence Rule

When a feature fails compatibility validation, identify the earliest layer where the contracts diverge.

For example:

Specification: accepts A
Lexer: rejects A

is primarily a lexical compatibility defect.

Where:

Specification: A means X
Lexer: accepts A
Parser: accepts A
AST: stores A
Semantic layer: interprets A as Y

the semantic layer is the first semantic divergence.

This prevents downstream patches from hiding upstream defects.

---

132. No Downstream Compensation for Upstream Divergence

A downstream subsystem MUST NOT compensate for an upstream contract violation merely to make tests pass.

Examples:

- semantic layer should not reinterpret malformed AST;
- IR generator should not reconstruct information discarded by AST;
- backend should not infer missing source semantics;
- runtime should not guess missing resource requirements;
- scheduler should not invent missing quantum dependencies.

The upstream contract MUST be corrected.

---

133. Compatibility of Generated Code

Generated target code is not the source language.

A backend MAY emit different:

- instructions;
- schedules;
- memory layouts;
- device mappings;
- kernels;
- pulse sequences;

provided semantic compatibility remains intact.

Generated code changes are therefore normally implementation/target changes rather than language changes.

---

134. Compatibility of Scheduling

Scheduling may change:

when
where
and in what physical order

operations execute, provided semantic dependencies remain satisfied.

A scheduling change becomes a language compatibility issue only when it changes specified observable semantics.

---

135. Compatibility of Routing

Routing may change physical paths.

For quantum systems:

logical qubits
    ↓
physical qubits

may change across targets.

The source program remains compatible if logical semantics remain unchanged.

---

136. Compatibility of QEC

QEC implementations may evolve independently.

Changing:

- code selection;
- syndrome scheduling;
- decoding;
- correction strategy;

does not automatically change the source language.

It becomes a compatibility issue when the specified logical semantics or error guarantees change.

---

137. Compatibility of ZQN

ZQN remains responsible for fault/noise semantics.

Changes to ZQN representation or implementation MUST preserve the contract promised to higher layers.

ZQN MUST NOT redefine source syntax.

---

138. Compatibility of HAL

HAL compatibility concerns:

- capability discovery;
- target state;
- resource state;
- calibration state;
- supported operations;
- execution interface.

HAL changes MUST NOT redefine source-language semantics.

A HAL may report that a target cannot satisfy a program.

It MUST NOT silently alter the program.

---

139. Compatibility of Calibration and Benchmarking

Calibration data and benchmark results are target/runtime information.

They MUST NOT silently become language semantics.

A compiler MAY use calibration/benchmark information for:

- optimization;
- scheduling;
- routing;
- target selection;

provided semantic behavior remains preserved.

---

140. Compatibility of Resource Management

Resource managers may change allocation strategies.

They MUST preserve:

- explicit requirements;
- explicit constraints;
- ownership;
- lifetime;
- correctness guarantees.

Allocation strategy is not normally source compatibility unless its change affects specified observable semantics.

---

141. Compatibility of Cancellation and Recovery

Cancellation, checkpointing, recovery, and resilience are runtime/semantic boundaries.

If cancellation or recovery semantics are part of the language contract, compatible implementations MUST preserve them.

Otherwise implementation-specific behavior MUST be clearly classified.

---

142. Compatibility of Determinism

If a program requests or requires deterministic behavior, implementation changes MUST preserve that guarantee.

A compiler MUST NOT introduce nondeterminism merely because more resources are available.

Where nondeterminism is intentionally allowed, the language contract MUST identify it.

---

143. Compatibility of Parallelism

Parallelization is compatible when it preserves specified semantics.

The number of workers MUST NOT be a language-level compatibility requirement unless explicitly part of program semantics.

The compiler MAY scale execution from:

one worker

to:

many workers

without source changes when semantics permit.

---

144. Compatibility of Distributed Scaling

The same principle applies to distributed systems.

The program SHOULD be able to express:

parallel
distributed
replicated
partitioned
streamed

without embedding a universal node count.

Physical node count belongs to deployment/resource realization.

---

145. Compatibility of Embedded Execution

Embedded targets may have severe physical constraints.

Those constraints MUST be represented as target/resource compatibility.

They MUST NOT redefine the source language.

A compiler may reject a target because:

required memory unavailable
required capability unavailable
required runtime unavailable

without declaring the source invalid.

---

146. Compatibility of Future Targets

A future target MAY implement existing Zamani semantics without changing source code.

To support this, the language MUST keep:

source semantics

separate from:

target realization

This is a core POCO-REAF invariant.

---

147. Compatibility Contract for New Files

A new compatibility-sensitive file MUST document:

Purpose
Owns
Does Not Own
Inputs
Outputs
Upstream Contracts
Downstream Consumers
Version
Compatibility
Migration
Tests
Scalability
Hard-Coding Audit
Completion Criteria

A new file MUST NOT create a competing authority.

---

148. Compatibility Contract for Existing Files

Existing files SHOULD be changed only when:

1. their current behavior contradicts a normative contract;
2. compatibility requires synchronization;
3. a migration requires them;
4. an implementation defect is found.

File renaming is not required merely to establish compatibility.

Existing names such as:

Zamani.g4
grammar.md
Zamani-Grammar.md

should remain unless there is a concrete technical reason to rename them.

---

149. Required Integration With Existing Compatibility Files

This file MUST coordinate with, rather than duplicate, the existing compatibility hierarchy:

grammar/compatibility/versions.md
    → compatibility/version-specific policy

grammar/compatibility/migrations.md
    → migration procedures

grammar/compatibility/deprecated.md
    → deprecation records

grammar/compatibility/reserved.md
    → reserved vocabulary/constructs

grammar/compatibility/compatibility-matrix.md
    → feature-by-feature matrix

grammar/validation/compatibility-rules.md
    → executable validation rules

grammar/specification/language-version.md
    → language version model

This file defines the normative cross-layer compatibility rules connecting those documents.

---

150. Required Integration With Specification Files

The compatibility contract MUST integrate with:

grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/spec/semantics.md
grammar/spec/type-system.md
grammar/spec/resources.md
grammar/spec/quantum.md
grammar/spec/diagnostics.md

Each specification owns its domain.

This file defines how changes in one domain affect compatibility with the others.

---

151. Required Integration With Frontend

The compatibility chain is:

lexical specification
    ↓
src/lexer.rs
    ↓
src/parser.rs
    ↓
src/frontend/ast/

Every source construct must remain traceable across this boundary.

No construct may be accepted in one layer and silently discarded in another.

---

152. Required Integration With Semantic Analysis

Semantic compatibility requires:

AST
 ↓
name resolution
 ↓
type analysis
 ↓
effect analysis
 ↓
capability/resource analysis
 ↓
semantic validation

The semantic layer MUST preserve the information needed by downstream IR generation.

---

153. Required Integration With IR

Every semantically valid construct that is supported by compilation MUST have a defined path into canonical IR.

If no lowering exists:

structured unsupported diagnostic

MUST occur before successful code generation.

---

154. Required Integration With Quantum IR

Quantum constructs MUST lower through the established canonical "quantum::ir" boundary.

No compatibility document, grammar file, frontend, or dialect may create an undocumented second quantum semantic IR.

---

155. Required Integration With Compiler

The compiler MUST consume semantic/IR contracts rather than infer source meaning from syntax accidentally.

Compiler transformations MUST be semantics-preserving.

---

156. Required Integration With Runtime

The runtime MUST receive explicit:

- capability requirements;
- resource requirements;
- execution policies;
- relevant semantic guarantees.

It MUST NOT infer missing semantics from hardware availability.

---

157. Required Integration With Tooling

Tooling MUST derive compatibility information from authoritative specifications and machine-readable contracts where available.

It MUST NOT independently maintain contradictory feature status.

---

158. Required Integration With Tests

Every compatibility rule introduced here SHOULD have a corresponding validation/test rule under:

grammar/validation/
grammar/tests/

where applicable.

---

159. Release Compatibility Gate

A production language release MUST NOT be considered compatible until:

specification
    ↓
grammar
    ↓
lexer
    ↓
parser
    ↓
AST
    ↓
semantics
    ↓
IR
    ↓
compiler
    ↓
runtime

has been checked for the changed features.

Additionally:

positive tests
negative tests
boundary tests
scalability tests
determinism tests
compatibility tests

MUST pass for the applicable release profile.

---

160. Release Checklist

Before release, verify:

- [ ] language version declared;
- [ ] compatibility classification declared;
- [ ] specification updated;
- [ ] lexical contract checked;
- [ ] grammar checked;
- [ ] parser checked;
- [ ] AST checked;
- [ ] semantic model checked;
- [ ] type system checked;
- [ ] effects checked;
- [ ] resources checked;
- [ ] capabilities checked;
- [ ] canonical IR checked;
- [ ] "quantum::ir" checked where applicable;
- [ ] compiler checked;
- [ ] runtime checked;
- [ ] target compatibility checked;
- [ ] dialect compatibility checked;
- [ ] migration documented where necessary;
- [ ] deprecation documented where necessary;
- [ ] diagnostics checked;
- [ ] positive tests pass;
- [ ] negative tests pass;
- [ ] boundary tests pass;
- [ ] scalability tests pass;
- [ ] determinism tests pass;
- [ ] compatibility tests pass;
- [ ] hard-coding audit passes;
- [ ] no Rust "unsafe";
- [ ] no phantom features;
- [ ] no silent semantic loss;
- [ ] no competing language authority.

---

161. Production-Ready Definition

"grammar/spec/compatibility.md" and the connected compatibility system are production-ready only when:

1. one language authority exists;
2. language version and compiler version are separated;
3. grammar representation and semantic authority are separated;
4. compatibility dimensions are independently identifiable;
5. stable features have complete pipelines;
6. AST migrations are lossless;
7. IR migrations are explicit;
8. "quantum::ir" remains canonical;
9. target/resource failures are distinguished from source errors;
10. hardware limits are not language limits;
11. scalability is resource-driven;
12. POCO-REAF is protected;
13. dialects are explicitly versioned;
14. interoperability is explicit;
15. deprecated features have migration paths;
16. removed features cannot silently reappear;
17. diagnostics are deterministic;
18. compatibility tests cover all changed boundaries;
19. the implementation remains safe Rust;
20. no downstream component silently compensates for upstream contract divergence.

---

162. Final Compatibility Invariant

The complete Zamani compatibility architecture is:

                         ZAMANI LANGUAGE
                               │
                  ┌────────────┴────────────┐
                  │                         │
             Language Version         Dialect Version
                  │                         │
                  └────────────┬────────────┘
                               │
                         Source Contract
                               │
              ┌────────────────┼────────────────┐
              │                │                │
           Lexical           Syntax          Semantics
              │                │                │
              └────────────────┼────────────────┘
                               │
                              AST
                               │
                 ┌─────────────┼─────────────┐
                 │             │             │
               Types        Effects      Resources
                 │             │             │
                 └─────────────┼─────────────┘
                               │
                      Canonical Semantics
                               │
              ┌────────────────┼────────────────┐
              │                │                │
          Classical        quantum::ir       HDL/Hardware
              │                │                │
              └────────────────┼────────────────┘
                               │
                             IR
                               │
                  ┌────────────┼────────────┐
                  │            │            │
             Optimization   Routing    Scheduling
                  │            │            │
                  └────────────┼────────────┘
                               │
                         Resilience / QEC
                               │
                              ZQN
                               │
                              HAL
                               │
                       Target Realization
                               │
        ┌──────────────┬───────┼───────┬──────────────┐
        │              │       │       │              │
       CPU            GPU     FPGA    QPU      Future Target
        │              │       │       │              │
        └──────────────┴───────┼───────┴──────────────┘
                               │
                            Runtime
                               │
                           Execution

The compatibility invariant is:

SOURCE SEMANTICS
        MUST
SURVIVE
        ↓
LEXING
        ↓
PARSING
        ↓
AST
        ↓
SEMANTIC ANALYSIS
        ↓
CANONICAL IR
        ↓
OPTIMIZATION
        ↓
ROUTING
        ↓
SCHEDULING
        ↓
RESILIENCE / QEC / ZQN
        ↓
HAL
        ↓
TARGET REALIZATION
        ↓
EXECUTION

The implementation MAY change.

The representation MAY change.

The hardware MAY change.

The compiler MAY change.

The runtime MAY change.

The physical topology MAY change.

The resource scale MAY change.

The target MAY change.

The source-level meaning of a compatible program MUST NOT silently change.

That is the compatibility foundation required for:

Program Once → Compile Once → Run Everywhere → Anywhere → Forever.

---

163. Completion Criteria for This File

This file is complete when:

- [ ] it is the sole normative cross-layer compatibility contract;
- [ ] "language-version.md" remains the version-model authority;
- [ ] "versions.md" remains the detailed version record;
- [ ] "migrations.md" remains the migration procedure authority;
- [ ] "deprecated.md" remains the deprecation registry;
- [ ] "reserved.md" remains the reserved registry;
- [ ] "compatibility-matrix.md" remains the feature matrix;
- [ ] "validation/compatibility-rules.md" implements executable compatibility checks;
- [ ] "grammar/spec/lexical.md" remains lexical authority;
- [ ] "grammar/spec/syntax.md" remains syntax authority;
- [ ] "grammar/spec/semantics.md" remains semantic authority;
- [ ] "grammar/Zamani.g4" remains the canonical ANTLR representation;
- [ ] "grammar/grammar.md" remains implementation-conformance documentation;
- [ ] "grammar/Zamani-Grammar.md" remains broader design/history/proposal material;
- [ ] frontend AST remains domain-neutral;
- [ ] "quantum::ir" remains the canonical quantum semantic boundary;
- [ ] no duplicate semantic IR is introduced;
- [ ] no artificial hardware/resource limits are introduced;
- [ ] no source semantics are silently discarded;
- [ ] all compatibility-sensitive changes are testable;
- [ ] all stable features have complete downstream integration;
- [ ] Rust 1.97/1.97.1 remains supported;
- [ ] production Rust remains free of "unsafe";
- [ ] compatibility decisions are deterministic;
- [ ] POCO-REAF remains an explicit architectural invariant.

End of normative specification.