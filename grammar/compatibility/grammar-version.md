Zamani Grammar Version Contract

Path: "grammar/compatibility/grammar-version.md"
Status: Normative
Scope: Concrete grammar representation, grammar evolution, grammar-version identity, grammar conformance, parser compatibility, lexer compatibility, AST compatibility, semantic compatibility, generated parser compatibility, grammar migration, dialect grammar compatibility, and repository-wide grammar integration
Language: Zamani
Grammar technology: ANTLR4
Implementation baseline: Rust 1.97 or later
Rust edition: 2021
Rust safety requirement: Safe Rust only; production implementation MUST NOT use Rust "unsafe"
Primary portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the normative compatibility contract for the concrete Zamani grammar representation.

It answers:

- What is a grammar version?
- What does a grammar version identify?
- How does grammar evolution remain compatible with the language specification?
- How does grammar evolution interact with the canonical lexer?
- How does grammar evolution interact with the parser?
- How does grammar evolution interact with the domain-neutral AST?
- How does grammar evolution interact with semantic analysis?
- How does grammar evolution interact with canonical IR?
- How are grammar changes classified?
- When is a grammar change breaking?
- How are contextual keywords introduced safely?
- How are dialect grammars versioned?
- How are generated parser artifacts associated with a grammar version?
- How are migrations handled?
- How are deprecated grammar forms handled?
- How are historical grammar descriptions prevented from becoming accidental language authority?
- How is POCO-REAF preserved?
- How is grammar scalability preserved without artificial machine or resource limits?
- How are grammar changes tested before being considered complete?

This document is specifically concerned with the grammar layer.

It does not replace:

grammar/specification/language-version.md

which owns the meaning of a Zamani language version.

It does not replace:

grammar/spec/versioning.md

which owns cross-layer versioning contracts.

It does not replace:

grammar/spec/compatibility.md

which owns general compatibility relationships.

It does not replace:

grammar/compatibility/versions.md

which owns release-level version policy.

It does not replace:

grammar/compatibility/migrations.md

which owns migration procedures.

It does not replace:

grammar/compatibility/deprecated.md

which owns the deprecation lifecycle.

It does not replace:

grammar/compatibility/reserved.md

which owns reserved syntax and identifiers.

It does not replace:

grammar/compatibility/compatibility-matrix.md

which owns repository-wide compatibility relationships.

---

2. Core Principle

A grammar version describes how a particular Zamani language contract is represented in concrete syntax.

It does not define a new language.

The governing relationship is:

language specification
        |
        v
grammar contract
        |
        v
canonical lexer
        |
        v
canonical ANTLR grammar
        |
        v
parser
        |
        v
domain-neutral AST
        |
        v
semantic model
        |
        v
canonical IR

The grammar is therefore an implementation of a language contract.

The grammar MUST NOT silently become the owner of semantics that belong to the language specification or semantic system.

---

3. Normative Terminology

The following terms are normative:

- MUST
- MUST NOT
- REQUIRED
- SHALL
- SHALL NOT
- SHOULD
- SHOULD NOT
- MAY
- OPTIONAL

A compatibility claim is valid only when supported by the relevant specification and conformance tests.

---

4. Ownership

4.1 This file owns

This file owns the compatibility contract concerning:

- grammar-version identity;
- grammar-version compatibility;
- grammar evolution;
- concrete grammar conformance;
- parser-grammar compatibility;
- grammar/lexer compatibility;
- grammar/AST compatibility obligations;
- grammar/semantic compatibility obligations;
- generated parser compatibility;
- grammar migration requirements;
- grammar deprecation compatibility;
- grammar compatibility metadata;
- grammar conformance classification;
- grammar-version provenance;
- grammar-version verification;
- grammar-version transition rules.

---

4.2 This file does not own

This file does NOT own:

- the meaning of Zamani language versions;
- individual language keywords;
- individual token definitions;
- general language semantics;
- type semantics;
- effect semantics;
- capability semantics;
- resource semantics;
- quantum semantics;
- classical semantics;
- HDL semantics;
- AI semantics;
- runtime behavior;
- hardware behavior;
- scheduling;
- routing;
- calibration;
- QEC;
- ZQN;
- HAL;
- backend algorithms;
- compiler optimization algorithms;
- physical hardware limits.

Those responsibilities remain with their respective authorities.

---

5. Authority Model

The repository MUST maintain the following ownership hierarchy:

grammar/DESIGN.md
        |
        v
grammar/specification/
        |
        +--> language contract
        +--> language version
        +--> normative syntax
        +--> normative semantics
        |
        v
grammar/spec/
        |
        +--> cross-layer contracts
        +--> type contracts
        +--> effect contracts
        +--> resource contracts
        +--> compatibility contracts
        |
        v
grammar/compatibility/
        |
        +--> grammar compatibility
        +--> release compatibility
        +--> migration
        +--> deprecation
        +--> reserved syntax
        +--> compatibility matrix
        |
        v
grammar/lexer/
        |
        v
grammar/antlr/ZamaniLexer.g4
        |
        v
grammar/Zamani.g4
        |
        v
ANTLR parser
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
        +-------------------+
        |                   |
        v                   v
 Classical IR         quantum::ir
        |                   |
        +---------+---------+
                  |
                  v
        optimization/lowering
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
              targets

No lower layer may redefine the contract of a higher layer.

---

6. Relationship to Language Version

A language version and a grammar version are distinct.

For example:

Language version:
1.4.0

Grammar version:
1.4.2

may be valid.

The language version describes the language contract.

The grammar version describes the concrete grammar implementation that conforms to that contract.

A grammar patch may therefore fix:

- an ANTLR ambiguity;
- parser generation;
- token handling;
- grammar factoring;
- unreachable rules;
- parser diagnostics;
- generated-parser compatibility;

without changing the language semantics.

Conversely, a semantic language change may require a new language version even if the concrete grammar change is small.

---

7. Required Version Layers

The repository MUST distinguish at least:

Language Version
Grammar Version
Lexer Contract Version
AST Contract Version
Semantic Contract Version
Classical IR Version
Quantum IR Version
Dialect Version
Compiler Version
Artifact Version
ABI Version
Runtime Version
Target Contract Version

These versions MUST NOT be conflated.

In particular:

Rust version
!=
Compiler version
!=
Grammar version
!=
Language version

The supported implementation baseline is:

Rust 1.97+
Rust edition 2021
safe Rust

Rust version changes MUST NOT automatically produce a Zamani grammar-version change.

---

8. Grammar Version Identity

A grammar version identifies a specific compatible concrete grammar contract.

The recommended representation is:

MAJOR.MINOR.PATCH

For example:

1.0.0
1.1.0
1.1.1
2.0.0

Grammar-version numbering follows grammar compatibility rules, not merely repository commit counts.

A grammar version MUST be traceable to:

grammar version
language version
lexer contract
AST contract
semantic contract
grammar source
generated parser artifact
conformance tests

---

9. Grammar Version Does Not Identify Hardware

A grammar version MUST NOT encode:

- CPU count;
- CPU architecture count;
- GPU count;
- FPGA count;
- ASIC count;
- QPU count;
- qubit capacity;
- memory capacity;
- thread count;
- register width;
- tensor rank limit;
- network size;
- cluster size;
- device count;
- topology size;
- accelerator count.

For example, grammar versioning MUST NOT contain constructs equivalent to:

grammar 1.0 for 8 CPUs
grammar 1.0 for 24 GB GPU memory
grammar 1.0 for 64 GB RAM
grammar 1.0 for 32-bit registers
grammar 1.0 for 32 qubits

Such limitations do not belong to grammar compatibility.

---

10. Grammar Version and POCO-REAF

Grammar versioning MUST preserve:

Program Once
Compile Once
Run Everywhere
Anywhere
Forever

The grammar MUST describe portable source meaning.

A grammar version MUST NOT make a source program dependent on a particular:

- processor;
- machine size;
- memory size;
- quantum processor;
- FPGA;
- accelerator;
- cluster size;
- physical topology.

Target feasibility remains a separate concern.

Therefore:

same source
+
compatible language contract
+
compatible grammar interpretation

MUST preserve the same intended source meaning independently of whether the eventual realization uses:

embedded
CPU
multicore
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

---

11. Grammar Compatibility Invariant

The fundamental invariant is:

«A grammar-compatible implementation MUST NOT silently change the specified interpretation of valid source code.»

A grammar change that changes source interpretation MUST be classified.

It MUST NOT be hidden behind:

- parser implementation details;
- generated parser changes;
- lexer implementation changes;
- grammar refactoring;
- grammar file movement;
- ANTLR regeneration.

---

12. Grammar Compatibility Classes

Every grammar change MUST be assigned a compatibility class.

The classes are:

Stable
Compatible Extension
Grammar Refactoring
Bug Fix
Contextual Extension
Experimental
Deprecated
Removed
Breaking
Dialect-Defined
Implementation-Defined
Target-Defined
Reserved

---

12.1 Stable

A stable grammar construct is part of the supported grammar contract.

Its interpretation is protected by compatibility rules.

---

12.2 Compatible Extension

A compatible extension adds syntax without changing the interpretation of existing valid programs.

Examples:

- a new non-conflicting construct;
- a new optional clause;
- an extensible attribute;
- a new generic operation form;
- a new dialect registration mechanism.

---

12.3 Grammar Refactoring

A grammar may be internally reorganized without changing its external language.

For example:

old:
large-rule
    -> internal-rule-a
    -> internal-rule-b

may become:

new:
large-rule
    -> shared-rule
    -> domain-rule

provided that:

- accepted valid source remains equivalent;
- rejected invalid source remains intentionally equivalent;
- tokenization remains compatible;
- AST meaning remains compatible;
- semantic interpretation remains compatible.

A pure grammar refactor does not require a language-version increment.

---

12.4 Bug Fix

A grammar bug may be corrected when the existing behavior violated the specified language contract.

The correction MUST document:

- the previous implementation behavior;
- the specified behavior;
- affected source;
- diagnostics;
- compatibility impact.

If valid programs previously relied on the erroneous behavior, the change MUST be classified appropriately rather than silently called a patch.

---

12.5 Contextual Extension

A new keyword SHOULD initially be contextual when global reservation could unnecessarily break existing identifiers.

This is especially important for an extensible language.

A contextual keyword MUST NOT become globally reserved merely because a new feature needs it.

---

12.6 Experimental

Experimental syntax MUST be explicitly marked.

Experimental syntax MUST NOT automatically become stable.

It MUST define:

- feature identity;
- grammar version;
- language-version relationship;
- owning specification;
- AST representation;
- semantic status;
- migration strategy;
- removal policy;
- compatibility guarantees.

---

12.7 Deprecated

Deprecated grammar remains recognized for the documented compatibility period unless explicitly removed.

Deprecation MUST provide:

- replacement;
- diagnostic guidance;
- version information;
- migration guidance;
- compatibility status.

---

12.8 Removed

A removed construct MUST NOT remain accidentally accepted by a grammar simply because a parser rule was not deleted.

Removal requires:

specification
+
grammar
+
lexer
+
AST
+
semantic
+
tests
+
documentation

to agree.

---

12.9 Breaking

A grammar change is breaking when it changes the accepted or interpreted language in a way that violates the compatibility contract.

Examples:

- changing precedence;
- changing associativity;
- changing an existing construct's parse;
- turning an identifier into an unconditional keyword;
- changing declaration structure;
- changing literal interpretation;
- changing statement boundaries;
- changing parser interpretation of existing valid syntax.

Breaking changes require explicit classification and migration handling.

---

13. Grammar Versioning Rules

13.1 MAJOR

A grammar MAJOR version MAY change:

- incompatible syntax;
- incompatible tokenization;
- incompatible parse structure;
- incompatible grammar-level interpretation;
- removal of stable syntax;
- incompatible precedence;
- incompatible associativity.

A grammar major version MUST NOT be incremented merely because:

- a backend was added;
- a new processor is supported;
- a new quantum target exists;
- an optimizer changed;
- the compiler was refactored;
- a new hardware generation exists.

---

13.2 MINOR

A grammar MINOR version SHOULD contain compatible grammar extensions.

Examples:

- new non-conflicting syntax;
- new optional clauses;
- new generic syntax;
- new extensibility mechanisms;
- new contextual keywords;
- new domain-neutral constructs.

Existing valid source MUST retain its meaning.

---

13.3 PATCH

A grammar PATCH version SHOULD contain:

- parser-generation corrections;
- non-semantic grammar fixes;
- documentation metadata corrections;
- deterministic grammar-generation corrections;
- diagnostics corrections;
- internal grammar refactoring that preserves the grammar contract.

A patch MUST NOT silently change the meaning of valid source.

---

14. Canonical Grammar Authority

The repository's canonical grammar composition root is:

grammar/Zamani.g4

It MUST remain the canonical composition root.

It MUST NOT become a second semantic specification.

Its responsibility is primarily:

program
item
expression dispatch
type dispatch
declaration dispatch
statement dispatch
domain grammar composition
EOF

Feature grammars MUST be composed through the established grammar architecture.

A feature grammar MUST NOT bypass the canonical composition architecture merely to become independently usable.

---

15. Lexer Integration

The canonical lexical authority is:

grammar/lexer/

with the generated/public ANTLR lexer represented through:

grammar/antlr/ZamaniLexer.g4

Grammar-version compatibility MUST therefore include lexer compatibility.

The relationship is:

lexer token contract
        |
        v
ZamaniLexer
        |
        v
parser grammar

A parser grammar MUST NOT define a second token vocabulary.

---

16. Token Evolution

Every grammar change that introduces, removes, renames, or changes token usage MUST be checked against:

grammar/lexer/tokens.g4
grammar/lexer/keywords.g4
grammar/antlr/ZamaniLexer.g4

where those files exist in the active architecture.

Token evolution MUST consider:

- lexical ambiguity;
- keyword collisions;
- identifiers;
- contextual keywords;
- operators;
- punctuation;
- literals;
- parser precedence;
- macro expansion;
- dialects;
- syntax highlighting;
- tooling;
- source compatibility.

The following classes of overlap require centralized review:

Question / QuestionMark
Ampersand / BitAnd
Pipe / BitOr
Arrow / ThinArrow

The exact canonical names are determined by the lexical authority.

This document does not establish duplicate token identities.

---

17. Keyword Evolution

A grammar feature MUST NOT automatically receive a globally reserved keyword.

The preferred evolution order is:

ordinary identifier
        |
        v
contextual keyword
        |
        v
dialect-scoped keyword
        |
        v
reserved keyword

A globally reserved keyword requires compatibility analysis.

Application-specific concepts SHOULD normally remain identifiers, libraries, dialect constructs, capabilities, or policies.

The core grammar MUST NOT become a catalogue of application domains.

---

18. AST Compatibility

Grammar compatibility is not complete when the parser succeeds.

The pipeline MUST preserve:

source
  |
  v
tokens
  |
  v
parse tree
  |
  v
domain-neutral AST

A grammar change MUST identify whether the resulting AST:

- remains structurally equivalent;
- requires a new AST node;
- changes an existing node;
- requires semantic normalization;
- requires AST-version migration.

A parser context MUST NOT become an accidental semantic model.

The grammar owns syntax.

The AST owns source structure.

Semantic analysis owns meaning.

---

19. AST Information Preservation

A grammar change MUST NOT discard information required downstream.

Where applicable, the resulting AST MUST preserve:

- source span;
- source ordering;
- names;
- qualified names;
- attributes;
- modifiers;
- parameters;
- operands;
- results;
- expressions;
- declarations;
- effects;
- requirements;
- capabilities;
- resource intent;
- contracts;
- policies;
- provenance;
- dialect identity;
- version information.

This is particularly important for:

- quantum programs;
- HDL;
- AI reasoning;
- contracts;
- policies;
- provenance;
- resource negotiation;
- hybrid computation.

---

20. Semantic Compatibility

Grammar compatibility MUST be evaluated through semantic meaning.

A grammar change is semantically breaking if it changes the meaning of existing valid source.

Examples include changing:

- evaluation order;
- ownership;
- type interpretation;
- effect interpretation;
- capability requirements;
- resource requirements;
- concurrency semantics;
- quantum measurement semantics;
- hybrid execution semantics;
- HDL semantics;
- distributed semantics;
- policy semantics.

A grammar file MUST NOT encode semantic checks that belong downstream merely to simplify parser implementation.

---

21. Canonical IR Boundary

Grammar versioning MUST preserve the canonical semantic pipeline:

source
  |
  v
grammar
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
  +------------------+
  |                  |
  v                  v
Classical IR       quantum::ir

Quantum syntax MUST eventually lower into the canonical:

quantum::ir

boundary.

Grammar versioning MUST NOT create:

quantum grammar IR
alternate quantum IR
temporary quantum semantic IR
vendor quantum IR

as competing canonical representations.

Vendor-specific or dialect-specific representations MUST be normalized into the appropriate semantic/IR boundary.

---

22. Quantum Grammar Compatibility

Quantum grammar evolution MUST remain extensible.

The grammar MUST NOT define a permanent finite universe of quantum operations.

It MUST support a generic operation model containing concepts such as:

operation specifier
targets
parameters
results
attributes
modifiers
effects
capabilities
resource requirements

The grammar MUST NOT depend on a permanently enumerated list equivalent to:

X
Y
Z
H
CNOT
...

as the only possible operations.

New quantum operations SHOULD be representable through the established operation/dialect/metadata architecture without changing the universal grammar.

This is essential for long-term grammar compatibility.

---

23. Classical Grammar Compatibility

Classical grammar evolution MUST preserve compatibility with:

- scalar values;
- aggregates;
- functions;
- memory;
- concurrency;
- generic types;
- data processing;
- numerical computation;
- tensor computation;
- effects;
- resource requirements;
- capabilities;
- contracts;
- provenance.

Classical grammar MUST NOT introduce fixed machine capacities.

---

24. HDL Grammar Compatibility

HDL grammar evolution MUST distinguish:

hardware intent

from:

physical realization

Grammar compatibility MUST NOT hard-code:

- fixed bus widths;
- fixed register widths;
- fixed device counts;
- fixed memory capacities;
- fixed FPGA resources;
- fixed ASIC resources.

Where a hardware property is semantically required, it MUST be expressed through the established:

type
resource
capability
constraint
policy
target

architecture.

---

25. Resource and Capability Compatibility

Grammar versioning MUST distinguish:

resource requirement

from:

capability

and from:

target

For example:

requires qubits >= n;

is a resource requirement.

requires capability("quantum.measurement");

is a capability requirement.

Neither expression identifies a specific physical device.

Grammar evolution MUST preserve this distinction.

---

26. Effects Compatibility

Grammar constructs that represent operations with effects MUST remain compatible with the effect architecture.

Potential effect categories include:

io
network
mutation
randomness
native
foreign
distributed
measurement
quantum
learning
adaptation
reflection
code_generation
simulation

The grammar identifies syntax.

The semantic effect system determines whether an operation is permitted and what effect it has.

A grammar change MUST NOT silently bypass effect analysis.

---

27. Contracts Compatibility

Contract syntax is integrated through the existing contract architecture.

Relevant constructs include:

requires
ensures
invariant
assume
guarantee
property

Grammar compatibility MUST preserve the distinction between:

contract syntax

and:

contract semantic validation

The grammar MUST NOT independently implement contract truth evaluation.

The existing contract validation architecture remains authoritative.

---

28. Policy Compatibility

Policy syntax belongs to the policy architecture.

Relevant policy concerns include:

requirements
constraints
capabilities
resources
permissions
prohibitions
preferences
fallbacks
selection
negotiation
execution
adaptation
simulation
sandboxing
determinism
reproducibility
provenance

Grammar compatibility MUST ensure that policy syntax remains connected to:

grammar/policies/
grammar/security/
grammar/resources/
grammar/execution/
grammar/effects/

without creating duplicate policy languages.

---

29. Provenance Compatibility

Grammar changes MUST preserve provenance where the downstream AST/semantic model requires it.

A grammar-derived artifact SHOULD be traceable through:

source
  |
  v
grammar version
  |
  v
parser
  |
  v
AST
  |
  v
semantic transformation
  |
  v
IR

Where provenance is required, the resulting artifact SHOULD identify:

- source location;
- language version;
- grammar version;
- dialect versions;
- transformation identity;
- compiler provenance;
- semantic transformation provenance.

---

30. Dialect Grammar Compatibility

Dialects are independently versionable extensions.

A dialect MUST identify:

dialect identity
dialect version
language compatibility
grammar compatibility
semantic contract
AST contract
IR contract
capabilities
resource requirements
effects
migration policy

Dialect grammar MUST NOT silently redefine standard Zamani syntax.

Dialect syntax SHOULD be namespaced or otherwise explicitly scoped.

---

31. Vendor Grammar Extensions

Vendor-specific grammar MUST NOT automatically enter the universal grammar.

Vendor extensions SHOULD use an extension mechanism equivalent to:

vendor::<namespace>::<feature>

or the canonical namespace mechanism established elsewhere in the repository.

Vendor grammar MUST identify:

- owner;
- namespace;
- version;
- semantic contract;
- capabilities;
- resource requirements;
- compatibility;
- migration;
- deprecation status.

A vendor extension MUST NOT change the meaning of standard syntax.

---

32. Experimental Grammar

Experimental grammar MUST be explicitly identifiable.

It MUST NOT be promoted to stable merely because:

- it exists in "Zamani-Grammar.md";
- a parser rule exists;
- an implementation accepts it;
- an example uses it.

Promotion requires:

proposal
  |
  v
specification
  |
  v
grammar
  |
  v
AST
  |
  v
semantic model
  |
  v
IR
  |
  v
tests
  |
  v
stable

---

33. Historical Grammar

"grammar/Zamani-Grammar.md" may contain:

stable
proposed
experimental
deprecated
historical
not implemented

Historical material MUST NOT automatically affect grammar-version compatibility.

A construct appearing in historical documentation is not sufficient evidence that it belongs to a stable grammar version.

---

34. Grammar Generation

Generated parser/lexer artifacts MUST be traceable to:

grammar source
grammar version
lexer contract version
ANTLR toolchain version
generation configuration
source revision

Generation MUST be deterministic to the extent supported by the toolchain.

A generated artifact MUST NOT be considered authoritative over the grammar source.

The source grammar remains authoritative.

---

35. ANTLR Integration

The grammar system uses ANTLR4.

Grammar compatibility MUST therefore account for:

ANTLR grammar source
ANTLR lexer
ANTLR parser
ANTLR imports
token vocabulary
generated parser
generated lexer
parser configuration
toolchain version

An ANTLR toolchain update does not automatically constitute a language-version change.

If generated behavior changes in a way that changes valid-source interpretation, the change MUST be compatibility-reviewed.

---

36. Rust Integration

The production implementation baseline is:

Rust 1.97 or later
Rust edition 2021
safe Rust only

The compiler MUST NOT use Rust "unsafe" merely to implement:

- grammar versioning;
- grammar parsing;
- compatibility checks;
- version resolution;
- migration;
- diagnostics;
- AST construction;
- semantic validation.

Grammar compatibility MUST remain independent of the Rust implementation version.

---

37. Version Resolution

The effective grammar version MUST be determinable.

It SHOULD be obtained from:

1. grammar metadata;
2. language-version contract;
3. compiler-supported grammar mapping;
4. generated-artifact metadata;
5. documented compatibility defaults where legacy support requires them.

Resolution MUST be deterministic.

An implementation MUST NOT randomly or implicitly select a grammar version.

An unsupported grammar version MUST produce a structured diagnostic.

---

38. Language Version and Grammar Version Mapping

A language version MAY map to one or more compatible grammar versions.

Conceptually:

Language 1.0
    |
    +--> Grammar 1.0.0
    +--> Grammar 1.0.1
    +--> Grammar 1.0.2

Language 1.1
    |
    +--> Grammar 1.1.0
    +--> Grammar 1.1.1

A grammar version MUST declare which language contract it implements.

A grammar version MUST NOT silently implement a different semantic language contract.

---

39. Multiple Grammar Implementations

Multiple grammar representations MAY exist internally when required by:

- tooling;
- incremental parsing;
- dialect parsing;
- editor support;
- compatibility parsing;
- migration;
- generated parser architecture.

However, only the canonical grammar architecture defines standard Zamani syntax.

Alternative parser implementations MUST conform to the same language contract.

They MUST NOT become independent language authorities.

---

40. Legacy Grammar

A legacy grammar MAY be retained for migration.

Its responsibilities are limited to:

recognize old source
        |
        v
normalize/migrate
        |
        v
current semantic representation

A legacy grammar MUST NOT silently reinterpret old source.

Its version MUST be explicit.

---

41. Grammar Migration

Grammar migration is governed by:

grammar/compatibility/migrations.md

This file establishes the requirement that a migration be:

- deterministic;
- source-preserving where possible;
- semantics-preserving where promised;
- diagnosable;
- testable;
- provenance-aware.

A migration MUST NOT silently alter program intent.

---

42. Deprecation

Deprecation is governed by:

grammar/compatibility/deprecated.md

A deprecated grammar construct MUST identify:

introduced version
deprecated version
replacement
removal eligibility
migration strategy
diagnostic

Deprecated syntax MUST NOT be removed merely because a newer grammar rule exists.

---

43. Reserved Syntax

Reserved identifiers and syntax are governed by:

grammar/compatibility/reserved.md

The reservation mechanism MUST reserve language evolution space, not hardware capacity.

The grammar MUST NOT reserve identifiers representing arbitrary future machines or resource counts.

---

44. Grammar Compatibility and Resources

Grammar evolution MUST remain independent of resource availability.

For example:

requires memory >= required_memory;

does not imply that the grammar has a maximum memory value.

Likewise:

requires qubits >= n;

does not establish a maximum qubit count.

The grammar must remain capable of representing arbitrarily large valid resource expressions subject only to implementation/storage constraints.

---

45. No Artificial Grammar Limits

No grammar rule may introduce artificial fixed limits on:

program size
declaration count
expression count
statement count
module count
function count
type count
generic parameter count
pattern count
quantum operation count
qubit count
resource requirements
capability requirements
policy rules
contract clauses
agents
messages
nodes
devices
tensor dimensions
network participants

For example, grammar MUST NOT contain architecture rules equivalent to:

exactly 32 operations
maximum 64 qubits
maximum 8 devices
maximum 16 policy rules
maximum 1024 declarations

Any implementation resource limit must remain an implementation/resource concern rather than a language grammar ceiling.

---

46. Unbounded Grammar Composition

Where a construct naturally represents a sequence, the grammar SHOULD use extensible forms such as:

item*

or:

item+

rather than artificial bounded alternatives.

The implementation MUST still provide resource-exhaustion protection and diagnostics without converting those protections into language-level semantic limits.

---

47. Error Handling

Grammar-version errors MUST be structured.

At minimum, diagnostics SHOULD distinguish:

UnknownGrammarVersion
UnsupportedGrammarVersion
IncompatibleGrammarVersion
MissingGrammarVersion
MalformedGrammarVersion
GrammarLanguageMismatch
GrammarLexerMismatch
GrammarAstMismatch
GrammarSemanticMismatch
DialectGrammarMismatch
GeneratedArtifactMismatch
MigrationRequired
DeprecatedGrammar
RemovedGrammarConstruct

Diagnostics SHOULD contain:

- source span;
- effective language version;
- effective grammar version;
- expected compatibility;
- actual compatibility;
- remediation guidance.

---

48. Determinism

Grammar-version resolution MUST be deterministic.

Given identical:

source
language contract
grammar metadata
dialect metadata
compiler compatibility information

the grammar-version decision MUST be identical.

No grammar compatibility decision may depend on:

- random selection;
- unspecified ordering;
- hardware identity;
- machine size;
- current resource availability;

unless such information is explicitly part of a downstream target-selection contract.

---

49. Grammar and Capability Negotiation

Grammar compatibility MUST NOT be confused with capability negotiation.

The order is:

grammar compatibility
        |
        v
source parsing
        |
        v
semantic analysis
        |
        v
capability/resource analysis
        |
        v
target negotiation

A program may be grammar-compatible and semantically valid while being infeasible for a particular target.

That is not a grammar-version failure.

---

50. Grammar and Target Compatibility

A target may reject a valid program because it lacks:

capability
resource
effect permission
required topology
required execution mode
required quantum capability
required hardware feature

This MUST NOT cause the source grammar to change.

The distinction is:

grammar compatibility
        !=
target feasibility

---

51. Grammar and Quantum Targets

A grammar version MUST NOT identify:

- a specific QPU;
- a fixed number of qubits;
- a fixed connectivity topology;
- a fixed gate set;
- a fixed calibration state.

Those belong downstream to:

capabilities
resources
routing
scheduling
QEC
ZQN
HAL
target realization

---

52. Grammar and HDL Targets

Likewise, grammar versions MUST NOT encode:

- FPGA family;
- ASIC process;
- physical cell library;
- fixed logic-resource capacity;
- fixed clock technology;
- fixed physical interconnect.

Those are target-level concerns.

---

53. Grammar and Adaptive Execution

Grammar features supporting:

adapt
retry
recover
fallback
simulation
policy
learning
reasoning

MUST preserve the same compatibility architecture.

The grammar describes intent.

The semantic model determines legality.

The execution system determines realization.

Adaptation MUST remain governed by policy, capability, effects, resources, authorization, and provenance where required by the relevant semantic contracts.

---

54. Grammar and Deterministic/Reproducible Execution

Grammar constructs related to deterministic or reproducible execution MUST preserve their source-level meaning across compatible grammar versions.

Grammar evolution MUST NOT silently convert deterministic source into nondeterministic behavior.

Where reproducibility metadata is required, it belongs in the appropriate:

execution
compatibility
provenance
artifact

contracts.

---

55. Grammar and Metaprogramming

Grammar changes affecting:

- macros;
- reflection;
- syntax trees;
- compile-time execution;
- code generation;
- type-level computation;

MUST be checked for interactions with:

grammar/macros/
grammar/metaprogramming/
grammar/compile/

Generated source MUST NOT bypass language-version compatibility.

A generated program remains subject to the applicable language and grammar contracts.

---

56. Grammar and Interoperability

Grammar compatibility MUST account for interoperability constructs involving:

FFI
ABI
foreign functions
foreign types
calling conventions
data layouts
external APIs
dialects
SQL
JSON
XML

Interoperability formats MUST NOT silently become part of the universal Zamani grammar merely because they are supported.

External formats SHOULD be handled through the established dialect/interoperability architecture.

---

57. Grammar and Application Domains

The universal grammar MUST remain domain-neutral.

Application-specific concepts such as:

computer vision
sentiment analysis
robotics
blockchain
payments
administration
legal workflows
VR
AR
specific AI products

MUST NOT become global grammar keywords merely because a library or application needs them.

They SHOULD be represented through:

libraries
dialects
capabilities
policies
services
application semantics

This protects grammar compatibility and long-term extensibility.

---

58. Compatibility Matrix Integration

Every grammar-version change MUST be reflected in:

grammar/compatibility/compatibility-matrix.md

where it affects cross-layer compatibility.

The matrix MUST be able to answer:

grammar version
        |
        +--> language version
        +--> lexer version
        +--> AST version
        +--> semantic version
        +--> IR version
        +--> dialect versions
        +--> compiler support
        +--> migration status

---

59. Version Policy Integration

Release-level compatibility remains owned by:

grammar/compatibility/versions.md

This file supplies the grammar-specific rules that "versions.md" consumes.

Neither file may contradict the other.

If a conflict is discovered:

grammar/specification/

remains authoritative for language meaning, while the relevant higher-level compatibility specification determines the interpretation of the version relationship.

---

60. Cross-Layer Versioning Integration

Cross-layer implementation rules are owned by:

grammar/spec/versioning.md

Grammar-version information MUST be propagated through the implementation layers where relevant:

source
  |
  v
grammar version
  |
  v
AST
  |
  v
semantic model
  |
  v
IR
  |
  v
artifact

The grammar version MUST NOT be used as a substitute for AST, semantic, IR, ABI, or runtime versioning.

---

61. Language Specification Integration

The normative language version is owned by:

grammar/specification/language-version.md

This grammar-version document MUST conform to it.

If a grammar change appears to require a semantic language-version change, the language specification MUST be updated first or concurrently according to the repository's change process.

The grammar MUST NOT establish a semantic change independently.

---

62. Grammar Documentation Integration

"grammar/grammar.md" is an implementation/conformance reference.

It MUST report grammar-version status consistently with this document.

It MUST NOT override:

grammar/specification/
grammar/spec/
grammar/compatibility/

---

63. Historical Documentation Integration

"grammar/Zamani-Grammar.md" may contain broad or historical material.

A construct documented there MUST NOT automatically be considered part of a grammar version.

Promotion requires explicit status and conformance.

---

64. Required Grammar Metadata

Every production grammar component SHOULD have machine-readable or structurally documented metadata identifying:

feature identity
grammar owner
grammar version
language-version relationship
status
dependencies
exports
lexer dependencies
AST contract
semantic contract
IR destination
tests
compatibility class
dialect status
deprecation status

The metadata MUST be consistent with repository-wide conformance metadata.

---

65. Required Feature Contract

Each grammar feature SHOULD document:

Purpose
Owns
Does Not Own
Dependencies
Lexer Dependencies
Imported Grammar Rules
Exported Grammar Rules
AST Contract
Semantic Contract
Type Contract
Effect Contract
Capability Contract
Resource Contract
Contract Integration
Policy Integration
Provenance Integration
IR Contract
Quantum Boundary
HDL Boundary
Backend Boundary
Diagnostics
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility Tests
Determinism Tests
Migration
Deprecation
Completion Criteria

This ensures that a feature can be completed independently without reopening it merely because another subsystem is implemented later.

---

66. Definition of Grammar Feature Completion

A grammar feature is NOT complete merely because:

ANTLR accepts the syntax

It is complete only when:

Specification
      |
      v
Lexer
      |
      v
Grammar
      |
      v
AST
      |
      v
Semantic model
      |
      v
Type/effect/resource/capability validation
      |
      v
Canonical IR
      |
      v
Tests

have been integrated as required by the feature.

For a quantum feature:

...
      |
      v
quantum::ir
      |
      v
routing/scheduling/QEC/ZQN/HAL

must be reachable where applicable.

---

67. Mandatory Grammar Tests

Every grammar-version change MUST be tested at the relevant layers.

At minimum:

lexical tests
parser tests
AST tests
semantic tests
negative tests
boundary tests
compatibility tests
determinism tests

Where applicable:

resource tests
capability tests
effect tests
contract tests
policy tests
provenance tests
dialect tests
quantum tests
HDL tests
hybrid tests
distributed tests
AI tests
interoperability tests
metaprogramming tests
simulation tests
sandbox tests

---

68. Positive Tests

Positive tests MUST establish that intended source remains accepted.

They SHOULD include:

minimal.zm
classical.zm
quantum.zm
hybrid.zm
hdl.zm
poco-reaf.zm

and relevant feature-specific examples.

---

69. Negative Tests

Negative tests MUST establish that invalid syntax remains rejected where required.

They MUST cover:

- malformed versions;
- incompatible versions;
- unsupported grammar versions;
- invalid token sequences;
- ambiguous constructs;
- illegal dialect syntax;
- removed syntax;
- malformed declarations.

---

70. Boundary Tests

Boundary tests MUST cover interactions between grammar domains.

Examples:

classical + quantum
classical + HDL
quantum + AI
AI + contracts
AI + policies
quantum + resources
HDL + resources
distributed + policy
networking + effects
FFI + effects
metaprogramming + provenance
simulation + determinism

---

71. Scalability Tests

Grammar conformance MUST be tested with structurally large programs.

The purpose is not to establish a maximum.

The purpose is to verify that the grammar architecture does not contain accidental finite ceilings.

Tests SHOULD include generated or parameterized programs containing increasing:

declarations
functions
types
operations
modules
policies
contracts
resource requirements
capabilities
agents
messages
quantum operations
HDL structures
data operations

No test should establish a language-level maximum merely because a test fixture has a finite size.

---

72. Determinism Tests

Given identical source and version metadata:

source
+
language version
+
grammar version
+
dialect versions

the parser MUST produce deterministic structural output.

Where parser generation or normalization is involved, tests SHOULD verify stable:

tokenization
parse structure
AST structure
source spans
diagnostics
version resolution

---

73. Compatibility Regression Tests

Every grammar-version release SHOULD include tests against the previous compatible grammar contract.

The test suite SHOULD verify:

old valid source
        |
        v
new compatible grammar
        |
        v
equivalent AST
        |
        v
equivalent semantics

where compatibility promises equivalence.

---

74. No Silent Reinterpretation

The following is prohibited:

old source
   |
   v
new grammar
   |
   v
different meaning

without an explicit compatibility classification.

If reinterpretation is intentional, it MUST be:

- documented;
- versioned;
- diagnosed where necessary;
- covered by tests;
- migrated where practical.

---

75. Source Compatibility Versus Parse Compatibility

The following distinctions MUST be maintained:

source compatible

means the same source remains valid and meaningful.

parse compatible

means the source can still be structurally parsed.

A program may parse successfully while being semantically incompatible.

Therefore:

parse success
!=
language compatibility

---

76. AST Compatibility Versus Grammar Compatibility

Likewise:

grammar compatibility
!=
AST compatibility

A grammar change may preserve source syntax while changing the AST representation.

When that occurs, the AST compatibility contract MUST be reviewed.

---

77. Semantic Compatibility Versus Grammar Compatibility

Likewise:

grammar compatibility
!=
semantic compatibility

A grammar can accept exactly the same source while downstream semantic interpretation changes.

Such a change MUST be governed by the semantic and language-version authorities.

---

78. IR Compatibility

Grammar evolution MUST preserve the intended semantic information required for canonical IR lowering.

A grammar change MUST NOT cause silent loss of information required by:

Classical IR
quantum::ir
HDL/hardware representation
distributed representation
other canonical domain representations

If the IR contract changes, that change belongs to the appropriate IR-version authority.

---

79. Generated Artifact Compatibility

Generated parser artifacts SHOULD record:

grammar version
language version
lexer contract version
ANTLR generation version
generation metadata

A generated artifact MUST NOT be accepted as authoritative merely because it has a newer timestamp.

Compatibility MUST be determined from explicit metadata and conformance.

---

80. Reproducible Grammar Generation

Grammar generation SHOULD be reproducible.

The same:

grammar source
lexer source
ANTLR toolchain
generation configuration
version metadata

SHOULD produce equivalent generated parser behavior.

The repository SHOULD detect accidental generated-artifact drift.

---

81. Grammar Dependency Direction

The dependency direction MUST remain:

specification
    ↓
grammar
    ↓
AST
    ↓
semantics
    ↓
IR
    ↓
lowering
    ↓
target realization

A grammar MUST NOT import backend implementation details merely to decide whether source syntax is valid.

A grammar MUST NOT depend on:

hardware discovery
runtime state
QPU calibration
scheduler state
physical topology
device availability

for ordinary syntax validity.

---

82. Grammar and Runtime State

The grammar parser MUST NOT require runtime state to determine ordinary source syntax.

For example, whether:

GPU
QPU
FPGA
CPU

is currently available MUST NOT determine whether a standard grammar construct can be parsed.

Runtime availability is evaluated downstream.

---

83. Grammar and Resource Availability

Similarly:

requires memory >= required_memory;

must parse independently of whether the current machine has sufficient memory.

The resource system evaluates feasibility later.

This is essential to POCO-REAF.

---

84. Grammar and Future Computing

The grammar version contract MUST support future computational domains without requiring permanent redesign of the universal grammar.

Potential future domains may include:

photonic
neuromorphic
optical
analog
reversible
biological
molecular
probabilistic
new accelerator architectures
new quantum architectures
future hybrid substrates

The grammar MUST reserve extension mechanisms rather than attempting to enumerate every future technology.

---

85. Extension Principle

When a future feature is needed, the preferred process is:

Can existing syntax represent it?
        |
       yes
        |
        v
reuse existing syntax
        |
       no
        |
        v
can a dialect represent it?
        |
       yes
        |
        v
add dialect
        |
       no
        |
        v
design universal syntax extension
        |
        v
compatibility review
        |
        v
specification
        |
        v
grammar

This prevents grammar keyword and rule explosion.

---

86. Grammar Evolution and Generic Quantum Operations

Quantum operations MUST remain data-driven.

The grammar should represent:

operationSpecifier
targets
parameters
results
attributes
modifiers

rather than permanently enumerating all known operations.

This allows new quantum operations to be introduced through compatible metadata/dialect mechanisms.

---

87. Grammar Evolution and AI/Reasoning

Generic constructs such as:

infer
deduce
reason
assert
retract
query
learn
adapt

MUST be evaluated against existing Zamani syntax before new lexical reservations are introduced.

Their semantic integration MUST remain connected to:

types
effects
capabilities
resources
contracts
policies
provenance
canonical semantic representation

Grammar compatibility MUST NOT create a separate language architecture for these features.

---

88. Grammar Evolution and Agents

Agent syntax MUST integrate with the existing concurrency architecture.

Where agent semantics require actors:

AI agent
    |
    v
actor
    |
    v
message
    |
    v
concurrency runtime

The grammar MUST NOT introduce a second independent actor language merely to support agents.

---

89. Grammar Evolution and Sandbox

Sandbox syntax MUST remain integrated with:

security
effects
capabilities
resources
policies
execution

The grammar MUST NOT directly inspect the host operating system or hardware to decide syntax validity.

---

90. Grammar Evolution and Simulation

Simulation is an execution strategy.

Grammar support for simulation MUST remain compatible with:

classical simulation
quantum simulation
HDL simulation
hardware simulation
distributed simulation
AI simulation
fault simulation
performance simulation

Simulation MUST NOT become a second programming language.

---

91. Grammar Version and Reproducibility

A grammar-versioned build SHOULD preserve enough metadata to establish:

source language version
grammar version
dialect versions
compiler version
IR version
artifact version

This allows future tooling to determine how an artifact was produced and whether migration is required.

---

92. Forward Compatibility

A newer grammar implementation MAY understand older compatible grammar versions.

It MUST NOT silently claim compatibility with an unknown future grammar contract.

For unknown versions, the implementation SHOULD produce a structured diagnostic.

Forward compatibility MUST NOT mean:

accept everything

without semantic validation.

---

93. Backward Compatibility

A newer compatible grammar SHOULD accept source written for an older compatible grammar version where the compatibility matrix says it should.

Where source migration is required, the implementation SHOULD identify:

migration required

rather than silently changing semantics.

---

94. Compatibility Negotiation

When multiple grammar versions are available, selection MUST be deterministic.

The implementation MAY consider:

requested grammar version
language version
compiler-supported versions
dialect versions
compatibility matrix
migration availability

It MUST NOT select a grammar solely because it is the newest available version.

---

95. Exact Version Versus Range

Exact grammar versions and grammar-version ranges MUST remain distinct.

For example:

grammar 1.2.0

identifies an exact contract.

A range such as:

>=1.2.0 <2.0.0

identifies a compatibility requirement.

The exact range syntax is owned by the versioning grammar and semantic version resolver.

This document defines the compatibility meaning, not an alternative parser syntax.

---

96. Version Metadata Must Be Machine-Checkable

Grammar compatibility metadata SHOULD be machine-readable.

At minimum, tooling should be able to determine:

grammar version
language version
status
compatibility class
supported previous versions
supported next-compatible versions
lexer contract
AST contract
semantic contract
dialect requirements
migration status
deprecation status

Human documentation alone is insufficient for production compatibility enforcement.

---

97. Compatibility Verification

A production implementation SHOULD perform compatibility checks before accepting a grammar as canonical.

Verification SHOULD include:

grammar metadata validation
lexer compatibility
parser generation
parser conformance
AST conformance
semantic conformance
IR preservation
negative tests
regression tests
determinism tests

---

98. Safe Rust Verification

The Rust implementation SHOULD enforce the safety requirement through repository tooling.

Production code MUST remain safe Rust.

Compatibility tooling MUST NOT require:

unsafe

for normal grammar/version processing.

CI SHOULD detect accidental use of unsafe code according to repository policy.

---

99. Grammar Version Provenance

Every released grammar version SHOULD be traceable to:

language specification revision
grammar source revision
lexer revision
AST contract revision
semantic contract revision
test-suite revision
ANTLR generation configuration
compiler compatibility metadata

This provides reproducibility and long-term maintenance.

---

100. Completion Criteria

"grammar/compatibility/grammar-version.md" and the grammar-version subsystem are production-ready only when all of the following are true:

Authority

- Language-version authority is unambiguous.
- Grammar-version authority is unambiguous.
- Cross-layer version authority is unambiguous.
- Release compatibility authority is unambiguous.
- No duplicate version authority exists.

Grammar

- "grammar/Zamani.g4" remains the canonical grammar composition root.
- Feature grammars have explicit ownership.
- Lexer authority is centralized.
- Token duplication is controlled.
- Contextual keyword evolution is supported.
- Dialect extensions are versionable.
- Experimental grammar is explicitly classified.

AST

- Grammar-to-AST mapping is defined.
- Source information is preserved.
- Grammar changes are checked for AST compatibility.

Semantics

- Grammar does not become semantic authority.
- Type/effect/resource/capability semantics remain downstream.
- Contracts remain semantically validated downstream.
- Policies remain semantically evaluated downstream.
- Provenance remains available where required.

IR

- Classical semantics retain their canonical IR path.
- Quantum semantics retain "quantum::ir" as the canonical quantum boundary.
- Grammar does not introduce competing IRs.
- Information required for lowering is preserved.

POCO-REAF

- Grammar contains no artificial machine-size limits.
- Grammar contains no fixed hardware-capacity limits.
- Resource requirements remain symbolic/generalized.
- Capability requirements remain target-independent.
- Source syntax remains independent of physical target selection.
- Future computational substrates can be added through extension mechanisms.

Compatibility

- MAJOR/MINOR/PATCH rules are defined.
- Breaking changes are explicitly classified.
- Contextual keyword evolution is defined.
- Deprecation is integrated.
- Migration is integrated.
- Reserved syntax is integrated.
- Forward compatibility behavior is defined.
- Unknown versions are diagnosed.
- Silent semantic reinterpretation is prohibited.

Testing

- Positive grammar tests exist.
- Negative grammar tests exist.
- Lexer tests exist.
- AST tests exist.
- Semantic compatibility tests exist.
- Cross-domain tests exist.
- Scalability tests exist.
- Determinism tests exist.
- Regression tests exist.
- Dialect compatibility tests exist.
- Generated-parser tests exist.

Implementation

- Rust 1.97+ is supported.
- Rust edition 2021 remains supported.
- Production implementation uses safe Rust.
- Grammar generation is reproducible.
- Grammar metadata is machine-checkable.
- Generated artifacts are traceable to their grammar version.

---

101. Required Repository Integration

The following integration is normative:

grammar/compatibility/grammar-version.md
        |
        +--> grammar/specification/language-version.md
        |
        +--> grammar/spec/versioning.md
        |
        +--> grammar/spec/compatibility.md
        |
        +--> grammar/compatibility/versions.md
        |
        +--> grammar/compatibility/migrations.md
        |
        +--> grammar/compatibility/deprecated.md
        |
        +--> grammar/compatibility/reserved.md
        |
        +--> grammar/compatibility/compatibility-matrix.md
        |
        +--> grammar/grammar.md
        |
        +--> grammar/Zamani-Grammar.md
        |
        +--> grammar/DESIGN.md
        |
        +--> grammar/lexer/
        |
        +--> grammar/antlr/ZamaniLexer.g4
        |
        +--> grammar/Zamani.g4
        |
        +--> grammar/core/versioning.g4
        |
        +--> domain grammar directories
        |
        +--> frontend AST
        |
        +--> semantic analysis
        |
        +--> canonical IR
        |
        +--> quantum::ir
        |
        +--> tests

No one of these files is permitted to silently establish a conflicting grammar-version interpretation.

---

102. Integration Ownership Table

Artifact| Ownership
"grammar/DESIGN.md"| Overall architecture and boundaries
"grammar/specification/language-version.md"| Meaning of language versions
"grammar/spec/versioning.md"| Cross-layer versioning
"grammar/spec/compatibility.md"| General compatibility relationships
"grammar/compatibility/grammar-version.md"| Concrete grammar-version contract
"grammar/compatibility/versions.md"| Release/version policy
"grammar/compatibility/migrations.md"| Migration procedures
"grammar/compatibility/deprecated.md"| Deprecation lifecycle
"grammar/compatibility/reserved.md"| Reserved syntax and identifiers
"grammar/compatibility/compatibility-matrix.md"| Repository-wide compatibility matrix
"grammar/grammar.md"| Conformance/implementation reference
"grammar/Zamani-Grammar.md"| Historical/extended grammar material
"grammar/lexer/"| Lexical authority
"grammar/antlr/ZamaniLexer.g4"| Canonical ANTLR lexer
"grammar/Zamani.g4"| Canonical grammar composition root
"grammar/core/versioning.g4"| Versioning syntax
"src/ast/" / frontend AST| Structural source representation
semantic layer| Meaning and semantic validation
Classical IR| Canonical classical representation
"quantum::ir"| Canonical quantum representation
routing| Physical/logical mapping
scheduling| Execution scheduling
QEC| Quantum error correction
ZQN| Quantum/noise/resilience downstream representation
HAL| Target realization
runtime| Execution

---

103. Final Compatibility Invariant

The entire grammar-version architecture is governed by:

Grammar versions describe syntax contracts.

Language versions describe language contracts.

AST versions describe structural representation contracts.

Semantic versions describe meaning contracts.

IR versions describe intermediate representation contracts.

Dialect versions describe explicitly scoped extensions.

Compiler versions describe implementations.

Target versions describe realization environments.

None of these may silently substitute for another.

Therefore:

same source
+
same language contract
+
compatible grammar interpretation

MUST preserve the intended meaning of the program.

And:

different machine
different hardware
different resource capacity
different accelerator
different QPU
different topology
different scale

MUST NOT, by themselves, require a grammar-version change.

The language remains portable.

The target realization adapts downstream.

---

104. Production Architecture

The final production relationship is:

                    LANGUAGE CONTRACT
                           |
                           v
                  LANGUAGE VERSION
                           |
                           v
                    GRAMMAR VERSION
                           |
                           v
                     LEXER CONTRACT
                           |
                           v
                    ANTLR GRAMMAR
                           |
                           v
                       PARSER
                           |
                           v
                  DOMAIN-NEUTRAL AST
                           |
                           v
                 STRUCTURAL VALIDATION
                           |
          +----------------+----------------+
          |                |                |
          v                v                v
        TYPES           EFFECTS        CAPABILITIES
          |                |                |
          +----------------+----------------+
                           |
                           v
                      RESOURCES
                           |
                           v
                       CONTRACTS
                           |
                           v
                       POLICIES
                           |
                           v
                      PROVENANCE
                           |
                           v
                 SEMANTIC RESOLUTION
                           |
            +--------------+--------------+
            |                             |
            v                             v
      CLASSICAL IR                   quantum::ir
            |                             |
            +--------------+--------------+
                           |
                           v
                    OPTIMIZATION
                           |
                           v
                       LOWERING
                           |
                           v
                      ROUTING
                           |
                           v
                    SCHEDULING
                           |
                           v
                 RESILIENCE / QEC / ZQN
                           |
                           v
                          HAL
                           |
          +----------------+----------------+
          |                |                |
         CPU              GPU             FPGA
          |                |                |
         ASIC          ACCELERATOR          QPU
          |                |                |
       EMBEDDED           HPC            SIMULATOR
          |                |                |
       CLUSTER        DISTRIBUTED          CLOUD
          |                |                |
          +----------------+----------------+
                           |
                           v
                    FUTURE TARGETS

The grammar version is therefore a compatibility coordinate, not a machine capability coordinate.

That distinction is mandatory for maintaining a grammar capable of supporting computation from the smallest practical realization through arbitrarily larger realizations permitted by available resources, while preserving the source-level POCO-REAF contract.

---

105. Definition of Done for This File

This file itself is complete when:

grammar/specification/language-version.md
        |
        v
grammar/spec/versioning.md
        |
        v
grammar/spec/compatibility.md
        |
        v
grammar/compatibility/grammar-version.md

forms a non-conflicting hierarchy, and when:

grammar/compatibility/grammar-version.md
        |
        +--> grammar/compatibility/versions.md
        +--> migrations.md
        +--> deprecated.md
        +--> reserved.md
        +--> compatibility-matrix.md
        +--> grammar/Zamani.g4
        +--> lexer
        +--> AST
        +--> semantics
        +--> canonical IR
        +--> quantum::ir
        +--> tests

can be validated without requiring this file to be rewritten merely because another subsystem is subsequently implemented.

No grammar-version rule in this document may establish:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

or any equivalent fixed-capacity limitation.

The grammar version describes the contract of the grammar, not the size of the universe in which a Zamani program may execute.