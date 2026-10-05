Zamani Grammar Validation

Path: "grammar/validation/README.md"
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Grammar technology: ANTLR4-compatible parser grammars
Rust baseline: Rust 1.97 / Rust 1.97.1, Edition 2021
Rust safety: safe Rust only
Primary objective: repository-wide grammar conformance, structural validation, integration validation, and production-readiness validation
Portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)

---

1. Purpose

"grammar/validation/" is the validation and conformance subsystem of the Zamani grammar architecture.

It verifies that the grammar repository remains:

- syntactically coherent;
- lexically coherent;
- structurally coherent;
- semantically traceable;
- AST-compatible;
- IR-compatible;
- domain-neutral where required;
- quantum-compatible;
- HDL-compatible;
- resource-aware without imposing artificial resource ceilings;
- capability-aware;
- effect-aware;
- contract-aware;
- policy-aware;
- provenance-aware;
- deterministic;
- portable;
- scalable;
- backwards-compatible according to the repository compatibility policy.

The validation directory does not define a second Zamani language.

Its job is to validate the language defined elsewhere.

The fundamental architecture is:

Specification
    |
    v
Canonical grammar
    |
    v
Lexer
    |
    v
Parser
    |
    v
Domain-neutral AST
    |
    v
Structural validation
    |
    +--> type validation
    +--> effect validation
    +--> capability validation
    +--> resource validation
    +--> contract validation
    +--> policy validation
    +--> provenance validation
    |
    v
Semantic model
    |
    +--> classical
    +--> quantum
    +--> hybrid
    +--> HDL
    +--> hardware
    +--> AI/reasoning
    +--> data
    +--> distributed
    +--> networking
    +--> interoperability
    |
    v
Canonical IR
    |
    +--> classical IR
    |
    +--> quantum::ir
    |
    v
Optimization
    |
    v
Lowering
    |
    v
Routing
    |
    v
Scheduling
    |
    v
Resilience / recovery / QEC
    |
    v
ZQN
    |
    v
HAL
    |
    v
Target realization

Validation observes this pipeline and verifies its contracts.

It does not replace any layer.

---

2. Authority Model

Validation follows the repository's authority hierarchy.

The intended hierarchy is:

grammar/DESIGN.md
        |
        v
grammar/specification/
        |
        v
grammar/spec/
        |
        v
grammar/Zamani.g4
        |
        v
grammar/antlr/
        |
        v
grammar/*/*.g4
        |
        v
src/lexer.rs
        |
        v
src/parser.rs
        |
        v
src/ast/
        |
        v
semantic implementation
        |
        v
canonical IR

Validation sits across this hierarchy:

                         VALIDATION
                             |
        +--------------------+--------------------+
        |                    |                    |
        v                    v                    v
 specification          grammar             implementation
        |                    |                    |
        +--------------------+--------------------+
                             |
                             v
                        conformance

Validation must not silently promote material into normative syntax.

Therefore:

- a README does not define syntax;
- an example does not define syntax;
- a test fixture does not define syntax;
- a historical document does not define syntax;
- an isolated ".g4" file does not automatically become reachable;
- a semantic capability does not automatically require a keyword;
- a target capability does not automatically become source syntax;
- a domain directory does not automatically become part of the canonical parser.

---

3. Ownership

Validation owns

"grammar/validation/" owns:

- validation contracts;
- conformance rules;
- validation entry points;
- validation-specific grammar facades;
- validation metadata;
- grammar-integrity checks;
- cross-layer conformance checks;
- production-readiness criteria;
- validation diagnostics contracts;
- validation test organization.

Validation does not own

Validation does not own:

- canonical source syntax;
- canonical tokens;
- expression precedence;
- AST definitions;
- semantic meaning;
- type inference;
- resource discovery;
- hardware discovery;
- target selection;
- quantum routing;
- quantum scheduling;
- QEC;
- ZQN;
- HAL;
- runtime execution;
- backend implementation;
- physical device configuration.

Those remain owned by their respective subsystems.

---

4. Canonical Source Ownership

The most important validation rule is:

«A validation facade must consume canonical syntax rather than redefine it.»

For example:

grammar/statements/contract.g4
        |
        +--> requiresStatement
        +--> ensuresStatement
        +--> invariantStatement
        +--> assumeStatement
        +--> guaranteeStatement
        +--> propertyStatement
        |
        v
grammar/validation/
        |
        +--> requires.g4
        +--> ensures.g4
        +--> invariants.g4
        +--> assumptions.g4
        +--> guarantees.g4
        +--> properties.g4
        +--> preconditions.g4
        +--> postconditions.g4
        +--> refinement.g4
        +--> proofs.g4
        +--> evidence.g4

The validation files must not redefine:

requiresStatement
ensuresStatement
invariantStatement
assumeStatement
guaranteeStatement
propertyStatement
expression
types
identifiers
punctuation
resource syntax
capability syntax
policy syntax

unless the repository deliberately establishes a new canonical owner.

---

5. Existing Validation Files

The validation directory currently contains the following feature boundaries:

grammar/validation/
├── README.md
├── assertions.g4
├── assumptions.g4
├── contracts.g4
├── ensures.g4
├── evidence.g4
├── guarantees.g4
├── invariants.g4
├── postconditions.g4
├── preconditions.g4
├── properties.g4
├── proofs.g4
├── refinement.g4
└── requires.g4

Each file has a specific role.

File| Primary responsibility
"assertions.g4"| isolated validation of canonical assertion syntax
"assumptions.g4"| isolated validation of canonical assumption syntax
"contracts.g4"| contract validation composition
"ensures.g4"| isolated postcondition validation
"evidence.g4"| evidence/verification validation boundary
"guarantees.g4"| isolated guarantee validation
"invariants.g4"| isolated invariant validation
"postconditions.g4"| postcondition facade
"preconditions.g4"| precondition facade
"properties.g4"| property validation facade
"proofs.g4"| proof-obligation validation boundary
"refinement.g4"| refinement validation boundary
"requires.g4"| isolated requirement/precondition validation

These files must remain independently meaningful.

---

6. Validation Facade Rule

A validation grammar should normally have this shape:

parser grammar FeatureValidation;

options {
    tokenVocab = ZamaniLexer;
}

import CanonicalOwner;

featureValidationUnit
    : canonicalFeatureRule EOF
    ;

The validation facade should:

- reuse the canonical lexer;
- reuse canonical parser rules;
- expose a complete-input entry point;
- preserve source syntax ownership;
- support isolated parser testing;
- avoid semantic duplication.

It should not:

- create a second syntax;
- create a second AST;
- create a second semantic model;
- create a second IR;
- perform hardware discovery;
- execute code;
- impose physical limits.

---

7. Validation Composition Root

A repository-level validation composition grammar should provide a single complete-input validation boundary.

The intended future composition file is:

grammar/validation/validation.g4

Its responsibility is orchestration.

It should own:

- the validation root;
- validation-category dispatch;
- complete-input boundary;
- composition of canonical validation rules;
- validation entry-point classification.

It should not redefine any feature grammar.

Conceptually:

validationUnit
    : validationItem EOF
    ;

validationItem
    : canonicalValidationRule
    ;

The exact alternatives must be derived from actual canonical grammar ownership.

The composition root must not import a facade merely because the filename exists.

Reachability must be established from actual grammar declarations.

---

8. Complete-Input Rule

Every isolated validation entry point intended for direct testing must have an unambiguous complete-input boundary.

Preferred form:

featureValidationUnit
    : canonicalRule EOF
    ;

The repository must avoid accidental structures such as:

validationUnit
    : featureValidationUnit
    | anotherValidationUnit
    ;

when those imported units already contain "EOF".

This can produce nested complete-input boundaries and ambiguous composition.

Therefore:

«"EOF" belongs at the appropriate complete-input boundary, not indiscriminately at every imported layer.»

---

9. Contract Validation

The canonical standalone contract source owner is:

grammar/statements/contract.g4

The canonical contract family includes:

requires
ensures
invariant
assume
guarantee
property

The validation architecture therefore follows:

contract.g4
    |
    +--> requiresStatement
    +--> ensuresStatement
    +--> invariantStatement
    +--> assumeStatement
    +--> guaranteeStatement
    +--> propertyStatement

Validation consumes those rules.

It must not create another contract language.

---

10. Contract Semantic Model

Validation must recognize that a contract is more than syntax.

The semantic model should be capable of representing:

Contract
├── condition
├── scope
├── assumptions
├── requirements
├── guarantees
├── evidence
├── provenance
├── policy
└── source span

Grammar validation verifies only the source structure.

Semantic analysis determines:

- whether the condition is meaningful;
- whether names resolve;
- whether types are valid;
- whether capabilities exist;
- whether resources satisfy requirements;
- whether policies permit the operation;
- whether evidence is sufficient;
- whether a property can be verified.

---

11. Assertions

"assertions.g4" is a validation facade for canonical assertion syntax.

Assertions are distinct from contracts.

An assertion may represent a runtime or executable condition.

Validation must preserve the distinction between:

assertion

and:

contract

and:

formal verification obligation

and:

property

Their semantic relationship belongs downstream.

---

12. Preconditions

"preconditions.g4" validates the semantic concept of preconditions using canonical syntax.

The primary standalone source construct is:

requires

Function-specific contracts may have their own canonical owner.

Therefore validation must support both concepts without creating duplicate syntax.

Architecture:

requiresStatement
        |
        +--> standalone precondition
        |
        +--> semantic precondition

Function contracts remain owned by the function-contract subsystem.

---

13. Postconditions

"postconditions.g4" validates postconditions through:

ensuresStatement

The presence of a lexer token representing another possible postcondition spelling does not automatically make that spelling canonical.

Canonical syntax is determined by the parser grammar.

Validation must therefore distinguish:

lexically available

from:

canonically parseable

and:

specified

from:

implemented

---

14. Assumptions

"assumptions.g4" is a facade over the canonical assumption syntax.

Assumptions may influence:

- semantic verification;
- optimization legality;
- resource reasoning;
- simulation;
- hardware reasoning;
- proof obligations;
- adaptive execution.

They do not themselves select a target.

---

15. Guarantees

"guarantees.g4" validates canonical guarantee syntax.

Guarantees may later be consumed by:

- semantic verification;
- optimization;
- execution planning;
- resilience;
- deployment;
- hardware realization.

A guarantee is not automatically a hardware guarantee.

Its interpretation depends on its semantic scope.

---

16. Properties

"properties.g4" validates canonical property syntax.

Properties are intentionally general.

They may describe:

- mathematical relationships;
- invariants;
- numerical properties;
- AI model properties;
- quantum properties;
- HDL properties;
- distributed-system properties;
- security properties;
- resource properties;
- transformation properties.

The grammar must remain domain-neutral.

---

17. Refinement

"refinement.g4" must not invent a second "refinement" source statement unless the canonical language later specifies one.

Refinement is primarily a semantic relationship.

It may represent:

specification
    |
    v
implementation

or:

general type
    |
    v
refined type

or:

general property
    |
    v
stronger property

or:

abstract operation
    |
    v
concrete operation

The semantic layer determines which refinement model applies.

---

18. Proofs

"proofs.g4" is a validation boundary for proof obligations.

It must not invent unsupported core syntax such as:

prove(...)
theorem(...)
witness(...)
solver(...)
tactic(...)
axiom(...)

unless those constructs are first established through the normal language lifecycle:

specification
    |
    v
AST design
    |
    v
semantic design
    |
    v
canonical grammar
    |
    v
implementation
    |
    v
tests
    |
    v
validation facade

A property may become a formal proof obligation without requiring a new proof keyword.

---

19. Evidence

Evidence is primarily a semantic/provenance concept.

The validation layer must therefore avoid inventing universal source syntax merely to represent evidence.

The intended model is:

claim
   |
   v
obligation
   |
   v
evidence
   |
   v
verification
   |
   v
decision
   |
   v
provenance

Evidence can originate from:

- static analysis;
- formal verification;
- tests;
- simulation;
- execution;
- measurement;
- quantum execution;
- hardware verification;
- compiler transformations;
- trusted external artifacts.

The source grammar should not need a different keyword for every evidence source.

---

20. Validation Categories

The production validator must cover at least:

1. grammar discovery;
2. grammar identity;
3. import resolution;
4. lexer/parser agreement;
5. duplicate rules;
6. duplicate tokens;
7. duplicate keywords;
8. keyword collisions;
9. undefined references;
10. unreachable rules;
11. unreachable alternatives;
12. intentional standalone rules;
13. test-only rules;
14. generated support rules;
15. deprecated rules;
16. experimental rules;
17. direct left recursion;
18. indirect left recursion;
19. nullable recursion;
20. ambiguity;
21. precedence;
22. associativity;
23. parser entry points;
24. EOF reachability;
25. source spans;
26. AST coverage;
27. semantic coverage;
28. effect coverage;
29. capability coverage;
30. resource coverage;
31. contract coverage;
32. policy coverage;
33. provenance coverage;
34. canonical IR coverage;
35. quantum IR integration;
36. HDL integration;
37. domain integration;
38. interoperability;
39. dialect isolation;
40. scalability;
41. portability;
42. determinism;
43. diagnostics;
44. compatibility;
45. generated-file consistency;
46. hard-coded-limit detection.

---

21. Import Resolution

Every grammar import must be resolvable.

For every import the validator should record:

source grammar
imported grammar name
resolved path
grammar type
grammar role
status

Statuses should include:

RESOLVED
UNRESOLVED
AMBIGUOUS
DUPLICATE
INCOMPATIBLE
DEPRECATED

An unresolved import must not automatically be reported as an unreachable rule.

The validator must first establish whether the dependency graph is complete.

---

22. Grammar Identity

Validation must distinguish:

file path
grammar name
grammar type
grammar role
domain
status
entry-point role

Directory names must not be treated as grammar declarations.

For example:

grammar/quantum/

does not automatically imply that every file inside it is part of the canonical source parser.

Actual ANTLR declarations and imports determine composition.

---

23. Entry-Point Classification

Every exposed parser entry point should be classified as one of:

CANONICAL_ENTRY
STANDALONE_ENTRY
VALIDATION_ENTRY
TOOLING_ENTRY
TEST_ENTRY
GENERATED_ENTRY
DEPRECATED_ENTRY

This prevents isolated validation grammars from accidentally becoming source-language roots.

---

24. Unreachable Rules

Validation must distinguish:

REACHABLE
STANDALONE
TEST_ONLY
TOOLING_ONLY
GENERATED_SUPPORT
DEPRECATED
EXPERIMENTAL
UNIMPLEMENTED
ORPHANED
UNRESOLVED_DEPENDENCY

A rule is "ORPHANED" only when dependency resolution is sufficiently complete to establish that it has no legitimate role.

An unresolved import is not proof of orphaning.

---

25. Unreachable Alternatives

Validation must inspect alternatives inside reachable rules.

For example:

operation
    : identifier
    | identifier
    ;

contains a redundant alternative.

The validator should also identify alternatives hidden by an earlier broader alternative.

Reports must identify:

grammar
rule
alternative
source location
reason
severity

---

26. Duplicate Rules

Duplicate rule identities within the same canonical grammar composition must be rejected unless a documented extension mechanism explicitly permits them.

The validator must distinguish:

duplicate grammar rule

from:

same concept, different grammar rule

The second case is an architecture/conformance problem rather than necessarily an ANTLR naming error.

---

27. Duplicate Tokens

The canonical lexical authority remains under:

grammar/antlr/
grammar/lexer/

Validation must identify accidental duplication such as:

Question
QuestionMark

or:

Ampersand
BitAnd

when both represent the same lexical role.

It must not reject intentionally distinct tokens without examining:

- spelling;
- tokenization context;
- semantic meaning;
- parser usage;
- compatibility requirements.

---

28. Keyword Collisions

Validation must check:

- reserved keywords;
- contextual keywords;
- identifiers;
- operators;
- domain keywords;
- dialect keywords;
- interoperability keywords.

A domain grammar must not silently make a previously valid identifier unusable.

New keywords require:

lexical specification
+
compatibility analysis
+
parser integration
+
tests

---

29. Lexer/Parser Agreement

Validation must detect mismatches between:

specification
lexer
parser
AST

Examples:

specified token missing from lexer
lexer token missing from specification
parser token missing from lexer
lexer token never consumed
grammar construct impossible to tokenize
literal documented but not emitted
operator documented but not tokenized
keyword documented but not accepted

This is particularly important for:

- Unicode;
- numeric literals;
- interpolation;
- quantum literals;
- operators;
- delimiters;
- contextual keywords.

A construct is not production-ready merely because it appears in documentation.

---

30. Expression Validation

Expression validation must ensure that validation grammars consume the canonical expression hierarchy.

Validation grammars must not create alternate versions of:

expression
primary
unary
binary
call
index
member access
match
lambda
query
reasoning

The canonical expression subsystem owns expression precedence and associativity.

---

31. Precedence and Associativity

The validation system must detect changes affecting:

- arithmetic;
- comparison;
- logical operations;
- bitwise operations;
- shifts;
- assignment;
- range expressions;
- unary operators;
- postfix operations;
- function calls;
- indexing;
- member access;
- conditional expressions;
- domain-specific operators.

Every newly introduced operator must have an explicit precedence and associativity contract.

---

32. Left Recursion

Validation must detect:

direct left recursion

and:

indirect left recursion

including cycles through nullable productions.

It must not reject legitimate recursive constructs merely because recursion exists.

Valid recursion can be required for:

- nested expressions;
- recursive types;
- recursive declarations;
- patterns;
- nested blocks;
- recursive data structures.

The objective is parser safety, not elimination of recursion.

---

33. Nullable Cycles

The validator must detect cycles capable of consuming zero tokens indefinitely.

Examples include structures equivalent to:

A -> B
B -> C?
C -> A?

when the complete cycle can derive empty input.

Uncontrolled nullable cycles are production failures because they may cause:

- parser nontermination;
- unstable prediction;
- excessive memory usage;
- pathological parse behavior.

---

34. Ambiguity

Ambiguity analysis must classify ambiguity as:

RESOLVED_BY_PRECEDENCE
RESOLVED_BY_CONTEXT
RESOLVED_BY_TOKENIZATION
RESOLVED_BY_PARSER_PREDICTION
INTENTIONAL
UNRESOLVED

"UNRESOLVED" ambiguity is a production failure.

Validation must examine:

- declaration/statement boundaries;
- expressions;
- contextual keywords;
- dialect constructs;
- interoperability constructs;
- quantum operations;
- HDL constructs;
- resource expressions;
- policy expressions;
- AI/reasoning expressions.

---

35. AST Coverage

Every stable accepted construct must have an AST contract.

The expected chain is:

grammar rule
    |
    v
AST node or canonical AST representation
    |
    v
semantic interpretation

Validation must identify:

GRAMMAR_ONLY
AST_MISSING
AST_PARTIAL
AST_COMPLETE

A stable grammar feature cannot be considered complete merely because ANTLR accepts it.

---

36. Domain-Neutral AST

The AST must remain independent of physical realization.

Validation must prevent source grammar from forcing AST structures containing:

vendor-specific quantum topology
physical qubit mapping
QEC routing decisions
calibration data
GPU-specific launch dimensions
FPGA-specific placement
ASIC-specific wiring
physical memory addresses
device-specific scheduling

Such information belongs downstream.

---

37. Semantic Coverage

Every stable grammar feature must identify:

semantic owner
inputs
outputs
type implications
effect implications
capability implications
resource implications
contract implications
policy implications
provenance implications
diagnostics

Parsing establishes structure.

Semantic analysis establishes meaning.

---

38. Effect Coverage

Features such as:

learn
adapt
reason
measure
simulate
reflect
generate
network
foreign call
native call

may carry semantic effects.

Validation must ensure that the grammar-to-semantic contract identifies those effects.

The grammar itself must not execute the effect.

---

39. Capability Coverage

Capabilities describe what an execution environment can provide.

Examples include:

capability("tensor.compute")
capability("quantum.measurement")
capability("gpu.compute")
capability("network")
capability("native.execute")

Validation verifies structural correctness.

Capability availability is resolved later.

---

40. Resource Coverage

Resource requirements are symbolic.

Examples:

requires memory >= required_memory;
requires qubits >= required_qubits;
requires capability("tensor.compute");
requires topology(required_topology);

The grammar must not turn these into fixed universal capacities.

Validation must explicitly reject architectural hard-coding such as:

maximum machine size
maximum processor count
maximum accelerator count
maximum memory capacity
maximum qubit count
maximum node count

when those values are being used as language-level ceilings.

Program-level numeric values are still ordinary values.

For example:

requires memory >= 4096;

can be valid program data.

It does not establish a language-wide memory ceiling.

---

41. POCO-REAF Validation

Validation must distinguish:

source portability

from:

physical feasibility

A program can remain unchanged while a target reports:

required capability unavailable

or:

resource requirement cannot be satisfied

or:

policy prevents realization

The grammar must not rewrite the source merely because a particular target is smaller, larger, newer, older, quantum, classical, distributed, or heterogeneous.

---

42. Scalability

Validation must ensure that grammar files contain no artificial language-level ceilings on:

- program size;
- source-unit count;
- declaration count;
- expression count;
- expression depth;
- type complexity;
- tensor rank;
- quantum operations;
- qubits;
- processors;
- threads;
- GPUs;
- FPGAs;
- nodes;
- devices;
- memory;
- storage;
- accelerators;
- network participants.

The correct interpretation of "infinity" is:

«no artificial finite capacity is encoded by the language grammar.»

Actual implementation exhaustion is governed by:

- available memory;
- parser configuration;
- compiler resources;
- runtime resources;
- target resources;
- explicit validation budgets;
- operating-system constraints;
- execution policies.

These are not language grammar ceilings.

---

43. Quantum Validation

Quantum validation must preserve the architecture:

Zamani source
    |
    v
domain-neutral AST
    |
    v
quantum semantic model
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
hardware

Validation must prevent grammar-level encoding of:

- physical qubit maps;
- device calibration;
- routing;
- scheduling;
- QEC implementation;
- hardware topology;
- vendor-specific physical constraints.

---

44. Quantum Operation Scalability

Validation must not require a fixed list of operations.

The preferred architecture is generic operation representation.

Conceptually:

operationSpecifier
quantumTargetList
parameters
results
attributes
modifiers

This allows:

built-in operation
custom operation
parameterized operation
vendor operation
future operation
dialect operation

without requiring the universal grammar to be rewritten for every new operation.

---

45. Classical Validation

Classical computation must use the same universal infrastructure for:

types
operations
effects
capabilities
resources
contracts
policies
provenance

Validation must not create a separate semantic universe for classical computation.

---

46. HDL Validation

HDL validation must distinguish:

hardware intent

from:

physical realization

Validation can verify structural HDL syntax and semantic traceability.

It must not encode universal physical limits.

Hardware realization belongs downstream:

HDL intent
    |
    v
semantic hardware model
    |
    v
simulation / verification
    |
    v
synthesis
    |
    v
target realization

---

47. AI and Reasoning Feature Integration

The expanded reasoning/knowledge/learning feature set must integrate with validation through the same common language architecture.

Relevant capabilities include:

infer
deduce
reason
assert
retract
query
learn
adapt
pattern matching
guards
uncertainty
probability
evidence
provenance
explanations
decisions
policies
agents
simulation
neural-symbolic composition

Validation must verify that each feature has:

lexer
grammar
AST
semantic owner
type behavior
effect behavior
capability behavior
resource behavior
policy behavior
provenance behavior
IR destination
tests

The validation subsystem must not create application-specific syntax for every possible use.

---

48. Knowledge Validation

Knowledge operations should remain generic.

Examples of semantic operations include:

assert
retract
query

They may be used for:

- reasoning;
- graph data;
- scientific information;
- configuration;
- compiler facts;
- hardware capabilities;
- security facts;
- provenance.

Validation must not assume that knowledge means only one application domain.

---

49. Learning Validation

Learning operations may have:

input
target
model
data
objective
algorithm
resources
capabilities
effects
policy
provenance

The grammar should not enumerate every learning algorithm.

Validation must ensure algorithm selection remains a semantic/library/dialect concern unless a particular algorithm becomes normative language syntax.

---

50. Adaptation Validation

Adaptation must be controlled.

The semantic chain should be:

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
resource validation
    |
    v
provenance
    |
    v
validated state/model/strategy change

Validation must ensure adaptation cannot silently bypass:

- contracts;
- effects;
- capabilities;
- resource requirements;
- security policies;
- provenance.

---

51. Uncertainty Validation

Uncertainty should integrate with the type and semantic systems.

Possible semantic concepts include:

uncertain
probability
distribution
confidence
belief

Validation must not impose a specific probability implementation.

The grammar expresses portable meaning.

The semantic/runtime layers determine realization.

---

52. Evidence and Provenance

Evidence must integrate with a common provenance model.

The model should be able to represent:

source
derived_from
generated_by
transformed_by
verified_by
reason
evidence
decision
version
timestamp

Validation must verify that features which claim provenance support can preserve the required provenance contract.

---

53. Policy Validation

Policies should be validated independently from their eventual consumers.

A policy may affect:

- resources;
- capabilities;
- effects;
- security;
- adaptation;
- execution;
- simulation;
- deployment;
- quantum execution;
- distributed execution;
- hardware realization.

The grammar must not create one policy language per subsystem.

---

54. Security Validation

Validation must verify that security-sensitive constructs have explicit semantic boundaries.

Examples:

sandbox
authorization
capability
trust
policy
provenance
audit

Parsing must never:

- execute commands;
- access the filesystem;
- access secrets;
- inspect hardware;
- connect to the network;
- invoke foreign code.

Security behavior belongs to later phases.

---

55. Simulation Validation

Simulation is an execution mode, not a separate language.

Validation must support the semantic possibility of:

classical simulation
quantum simulation
HDL simulation
hardware simulation
distributed simulation
AI/model simulation
fault simulation
performance simulation

The source meaning should remain portable.

---

56. Adaptive Execution Validation

Adaptive execution may consume:

detect
evaluate
select
fallback
retry
recover
adapt

The validation subsystem should verify structural integration with the execution/resilience contracts.

Where the repository already defines resilience states such as:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

and outcomes such as:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

validation must verify consistency with those canonical definitions rather than reproducing them inside grammar files.

---

57. Concurrency and Agents

Multi-agent semantics must reuse the existing concurrency architecture.

Preferred integration:

AI agent
    |
    v
actor
    |
    v
message
    |
    v
channel/task
    |
    v
scheduler

Validation must reject architectural duplication such as:

AI actor system

being implemented separately from the canonical actor system.

---

58. Interoperability Validation

FFI and ABI features must integrate with:

effects
capabilities
types
data layout
calling convention
linkage
foreign declarations

Validation must ensure that a foreign call can be traced through:

source
    |
    v
AST
    |
    v
semantic foreign-call model
    |
    v
effect/capability validation
    |
    v
canonical IR
    |
    v
ABI/backend

---

59. Dialect Validation

Dialects must be isolated.

A dialect must not silently alter core language meaning.

Every dialect should identify:

dialect name
version
owner
imports
extensions
reserved syntax
AST extensions
semantic extensions
compatibility
tests

Validation must verify that dialect syntax does not accidentally collide with canonical Zamani syntax.

---

60. Interchange Formats

Formats such as:

SQL
JSON
XML

should remain interoperability/data concerns rather than forcing their complete external grammars into the universal core grammar.

Validation must ensure:

external format
    |
    v
format parser
    |
    v
Zamani semantic representation
    |
    v
canonical IR/data model

when such integration is provided.

---

61. Generated Grammar Validation

If grammar files are generated or composed automatically, validation must distinguish:

source grammar
generated grammar
generated support grammar

Generated artifacts must not become accidental authorities.

The validator should be capable of detecting:

generated file differs from source specification
generated file is stale
generated file is missing
generated file contains unexpected rules
generated token differs from canonical token registry

---

62. Source Span Validation

All source constructs capable of producing diagnostics should preserve source provenance.

At minimum the frontend should be able to preserve an equivalent of:

source file
start position
end position
line
column

Validation verifies that source spans survive:

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
    |
    v
IR/provenance where required
    |
    v
diagnostics

The exact Rust representation belongs to the frontend.

---

63. Diagnostic Validation

Validation diagnostics should contain enough information to identify:

severity
category
grammar
rule
alternative
source location
diagnostic code
description
cause
recommended remediation

Severity should be machine-readable.

Recommended categories include:

INFO
WARNING
ERROR
FATAL

Validation tooling should support deterministic output.

---

64. Determinism

Given:

same repository snapshot
same grammar configuration
same validation configuration
same validator version

validation must produce equivalent results.

Validation must not depend on:

- system time;
- random ordering;
- network state;
- hardware availability;
- CPU count;
- GPU count;
- QPU availability;
- filesystem traversal order;
- nondeterministic hash iteration.

When filesystem enumeration is required, results must be explicitly ordered.

---

65. Resource Budgets

Validation itself may require resource controls.

These are implementation controls, not language limits.

Examples include:

validation time budget
memory budget
diagnostic budget
recursion protection
parser recovery budget
file traversal policy

Such configuration must not become grammar-level capacity constants.

A validation budget exhaustion should report:

VALIDATION_RESOURCE_EXHAUSTED

rather than incorrectly claiming:

LANGUAGE_LIMIT_REACHED

---

66. Scalability Testing

Scalability tests must vary:

source size
grammar size
number of files
number of rules
number of alternatives
expression depth
type complexity
contract count
resource constraints
capability declarations
domain combinations

The grammar itself must remain unchanged.

Tests should establish behavior across increasing scales.

The objective is to detect:

- accidental quadratic behavior;
- exponential parser behavior;
- memory leaks;
- nullable-cycle problems;
- ambiguity explosions;
- excessive diagnostics;
- nondeterministic ordering.

---

67. Boundary Testing

Boundary tests must cover transitions between:

lexer ↔ parser
parser ↔ AST
AST ↔ semantic model
semantic model ↔ resources
semantic model ↔ capabilities
semantic model ↔ effects
semantic model ↔ contracts
semantic model ↔ policies
semantic model ↔ provenance
semantic model ↔ IR
classical ↔ quantum
classical ↔ HDL
host ↔ accelerator
AI ↔ reasoning
AI ↔ quantum
source ↔ dialect
Zamani ↔ foreign interface

---

68. Cross-Domain Validation

At minimum, the validation suite must exercise combinations of:

classical
quantum
hybrid
HDL
hardware
AI/reasoning
data
distributed
networking
security
interoperability
simulation

The purpose is not to make every grammar rule understand every domain.

The purpose is to prove that independently owned subsystems compose without violating ownership boundaries.

---

69. Mandatory End-to-End Validation Program

The repository should maintain an integration program combining:

reasoning
learning
adaptation
knowledge
uncertainty
contracts
provenance
policies
classical computation
tensor/data operations
quantum operations
measurement
hybrid control
agents
parallelism
resource requirements
capabilities
effects
simulation
hardware intent

The expected pipeline is:

source
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
  +--> types
  +--> effects
  +--> capabilities
  +--> resources
  +--> contracts
  +--> policies
  +--> provenance
  |
  v
semantic model
  |
  +--> classical
  +--> quantum
  +--> hybrid
  +--> HDL
  +--> AI
  |
  v
canonical IR
  |
  +--> classical IR
  +--> quantum::ir

No validation file should need to know the complete implementation details of every downstream domain.

---

70. Required Test Classes

Every validation feature must have:

positive tests
negative tests
boundary tests
scalability tests
determinism tests
portability tests
compatibility tests
cross-domain tests
diagnostic tests

Where applicable it should additionally have:

round-trip tests
generated-grammar tests
AST coverage tests
semantic coverage tests
IR coverage tests
resource tests
capability tests
effect tests
policy tests
provenance tests

---

71. Positive Tests

Positive tests verify that valid syntax remains accepted.

They must cover:

- minimal forms;
- nested forms;
- expressions;
- identifiers;
- qualified names;
- Unicode where supported;
- domain-neutral values;
- quantum-derived expressions;
- HDL-derived expressions;
- resource expressions;
- capability expressions;
- contracts;
- policies.

---

72. Negative Tests

Negative tests must verify rejection of structurally invalid syntax.

Examples include:

missing operands
missing delimiters
missing terminators
unexpected tokens
extra expressions
invalid nesting
malformed contract
malformed property
malformed validation entry

Negative grammar tests must not duplicate semantic diagnostics.

For example:

unknown_variable

may be syntactically valid and therefore belongs to semantic testing.

---

73. Compatibility

Validation must track compatibility status for every changed construct.

Statuses may include:

STABLE
EXPERIMENTAL
PROPOSED
DEPRECATED
HISTORICAL
REMOVED
NOT_IMPLEMENTED

A grammar change that breaks previously valid source must have an explicit compatibility decision.

---

74. Versioning

Validation should distinguish:

language version
grammar version
lexer version
AST version
semantic version
IR version
dialect version

A grammar version change must not silently imply an AST or IR compatibility change.

Those transitions must be explicitly recorded.

---

75. Validation Metadata

Every validation feature file should declare, in documentation or machine-readable metadata:

DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
IR_OWNER:
TEST_OWNER:
SPEC_OWNER:

Example:

DEPENDS_ON:
    grammar/statements/contract.g4
    grammar/expressions/
    grammar/antlr/ZamaniLexer.g4

EXPORTS:
    requiresValidationUnit

CONSUMED_BY:
    grammar/validation/validation.g4
    validation test tooling

AST_OWNER:
    src/ast/

SEMANTIC_OWNER:
    semantic contract subsystem

IR_OWNER:
    canonical semantic/IR lowering

TEST_OWNER:
    grammar/tests/validation/

SPEC_OWNER:
    grammar/spec/
    grammar/specification/

This makes each file independently understandable.

---

76. Definition of DONE for a Validation File

A validation file is not complete merely because it compiles.

It is complete when:

[ ] Purpose documented
[ ] Ownership documented
[ ] Non-ownership documented
[ ] Dependencies documented
[ ] Public entry points documented
[ ] Lexer dependencies documented
[ ] Canonical source owner identified
[ ] AST owner identified
[ ] Semantic owner identified
[ ] IR owner identified
[ ] Effect contract identified
[ ] Capability contract identified
[ ] Resource contract identified
[ ] Contract relationship identified
[ ] Policy relationship identified
[ ] Provenance relationship identified
[ ] Quantum boundary identified where relevant
[ ] HDL boundary identified where relevant
[ ] Backend boundary identified
[ ] Diagnostics identified
[ ] Positive tests exist
[ ] Negative tests exist
[ ] Boundary tests exist
[ ] Scalability tests exist
[ ] Determinism tests exist
[ ] Portability tests exist
[ ] Compatibility tests exist
[ ] No duplicate syntax exists
[ ] No artificial capacity limit exists
[ ] No target-specific source syntax exists
[ ] No runtime behavior is embedded in grammar
[ ] No embedded Rust actions exist
[ ] Safe Rust implementation remains possible
[ ] Rust 1.97 / 1.97.1 compatibility is preserved
[ ] Canonical AST path is verified
[ ] Canonical IR path is verified

---

77. Hard-Coding Audit

Validation must explicitly detect accidental universal capacity constants.

The grammar architecture must not define universal ceilings for:

qubits
CPUs
cores
threads
GPUs
FPGAs
ASIC resources
nodes
devices
memory
storage
tensor rank
register width
network size
accelerator count

Program data may contain numeric values.

The distinction is:

program value

versus:

language capacity

For example:

requires memory >= required_memory;

is a portable requirement.

A universal implementation constant that limits all Zamani programs to a particular memory quantity is not.

---

78. Hardware Independence

Validation must ensure that grammar files do not encode:

physical device identifiers
physical addresses
physical qubit identifiers
fixed accelerator counts
fixed processor counts
fixed memory banks
fixed topology
fixed vendor scheduling
fixed routing

Those belong to target descriptions and realization layers.

---

79. Resource Negotiation

Validation must recognize the separation:

program intent
    |
    v
requirements
    |
    v
capabilities
    |
    v
constraints
    |
    v
preferences
    |
    v
negotiation
    |
    v
execution plan

This is central to POCO-REAF.

A source program expresses what it needs.

The compiler/runtime determines how the available environment can satisfy it.

---

80. Portability Rule

A source construct must not become target-dependent merely because its eventual implementation differs across targets.

The same source may be lowered to:

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
future hardware

Validation checks source-level portability.

Target feasibility is a later concern.

---

81. Rust Boundary

ANTLR grammar files are language-grammar artifacts.

Rust version requirements apply to the implementation that consumes or validates the grammars.

The repository implementation baseline is:

Rust 1.97
or
Rust 1.97.1

Edition 2021
safe Rust only

Grammar files must not contain Rust implementation code.

If validation tooling is implemented in Rust, it must preserve the repository's safety requirements.

---

82. No Embedded Execution

Validation grammars must not contain actions that:

- execute programs;
- access files;
- access networks;
- inspect hardware;
- call foreign functions;
- mutate compiler state;
- allocate target resources;
- access secrets;
- invoke external tools during parsing.

Validation tooling may perform those operations outside the parser under explicit tool contracts.

---

83. Repository Integration

Validation integrates with:

grammar/DESIGN.md
grammar/README.md
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/Zamani.g4
grammar/antlr/
grammar/lexer/
grammar/core/
grammar/types/
grammar/expressions/
grammar/statements/
grammar/resources/
grammar/effects/
grammar/security/
grammar/execution/
grammar/quantum/
grammar/hybrid/
grammar/hdl/
grammar/hardware/
grammar/classical/
grammar/data/
grammar/distributed/
grammar/networking/
grammar/interoperability/
grammar/dialects/
grammar/metaprogramming/
grammar/tests/
src/lexer.rs
src/parser.rs
src/ast/
src/frontend/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs
src/quantum/ir/

The validation directory consumes contracts from these locations.

It does not replace them.

---

84. Integration with "grammar/grammar.md"

"grammar/grammar.md" should provide implementation-conformance status.

Validation should consume or verify statuses such as:

SPECIFIED
IMPLEMENTED
PARTIALLY_IMPLEMENTED
PLANNED
DEPRECATED

Additional machine-checkable statuses may include:

AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL

Validation must not mark a feature implemented merely because its parser rule exists.

---

85. Integration with "grammar/Zamani-Grammar.md"

"grammar/Zamani-Grammar.md" remains extended/historical/proposed documentation.

Validation must ensure that material labelled:

proposed
experimental
deprecated
historical
not implemented

is not accidentally treated as stable source syntax.

---

86. Integration with "grammar/antlr/ZamaniLexer.g4"

Validation verifies:

token exists
token spelling is correct
token category is correct
token is consumed where intended
token does not collide
token status is correct

The lexer remains the lexical authority.

Validation does not define tokens.

---

87. Integration with "grammar/lexer/"

The lexer subsystem should remain responsible for the canonical token registry.

Validation must compare:

token registry
        |
        v
lexer
        |
        v
parser consumption

Any mismatch must be reported.

---

88. Integration with Resources

Validation must verify that resource syntax is structurally separate from physical realization.

The intended relationship is:

resource requirement
        |
        v
semantic requirement
        |
        v
capability negotiation
        |
        v
execution planning

Validation must not resolve hardware resources.

---

89. Integration with Effects

Validation must verify that source constructs with effects identify their semantic effect contract.

Examples include:

network
foreign
native
measurement
randomness
learning
adaptation
reflection
simulation
distributed

Effect checking belongs downstream.

---

90. Integration with Policies

Validation verifies structural policy syntax.

Semantic policy analysis determines whether a policy:

- permits;
- forbids;
- constrains;
- prefers;
- authorizes;
- requires;
- limits an operation.

Policy resolution remains outside the grammar.

---

91. Integration with Provenance

Validation must preserve the possibility of tracing:

source
    |
    v
parsed construct
    |
    v
semantic transformation
    |
    v
derived artifact
    |
    v
verification
    |
    v
decision

Where provenance is required by the semantic contract, source locations must remain available.

---

92. Integration with Classical IR

Classical constructs validated here eventually lower through the repository's canonical classical semantic/IR pipeline.

Validation must not create an alternative classical IR.

---

93. Integration with Quantum IR

Quantum constructs validated here must eventually converge on:

src/quantum/ir/

The validation layer must not introduce:

validation quantum IR
frontend quantum IR
vendor quantum IR
temporary quantum IR

as another canonical semantic layer.

---

94. Integration with HDL

HDL validation should eventually trace:

source
    |
    v
AST
    |
    v
hardware semantic model
    |
    v
verification/simulation
    |
    v
synthesis
    |
    v
target realization

Validation does not perform synthesis.

---

95. Integration with AI/Reasoning

Reasoning, knowledge, learning, adaptation, uncertainty, evidence, explanations, policies, agents, and neural-symbolic composition must all retain:

grammar
AST
semantic
type
effect
capability
resource
policy
provenance
IR

contracts.

Validation checks the chain.

It does not become the owner of AI semantics.

---

96. Integration with Concurrency

Agent semantics must reuse the canonical concurrency architecture.

Validation must verify composition rather than duplication.

Preferred model:

agent
    |
    v
actor
    |
    v
message
    |
    v
scheduler

---

97. Integration with Interoperability

Foreign declarations must remain traceable through:

type system
effects
capabilities
ABI
calling convention
data layout
linkage
backend

Validation checks that those contracts exist.

---

98. Integration with Metaprogramming

Reflection and code generation must remain controlled.

Validation must ensure the grammar-to-semantic contract identifies:

reflection effect
generation effect
capability requirements
policy requirements
provenance requirements

The validation grammar itself must never execute generated code.

---

99. Required Validation Test Organization

The validation test suite should ultimately support:

grammar/tests/
├── lexical/
├── parser/
├── validation/
│   ├── imports/
│   ├── rules/
│   ├── alternatives/
│   ├── tokens/
│   ├── keywords/
│   ├── ambiguity/
│   ├── recursion/
│   ├── precedence/
│   ├── source-spans/
│   ├── ast/
│   ├── semantic/
│   ├── effects/
│   ├── capabilities/
│   ├── resources/
│   ├── contracts/
│   ├── policies/
│   ├── provenance/
│   ├── ir/
│   ├── quantum/
│   ├── hdl/
│   ├── interoperability/
│   ├── scalability/
│   ├── portability/
│   ├── determinism/
│   └── compatibility/
├── classical/
├── quantum/
├── hybrid/
├── hdl/
├── ai/
├── distributed/
├── networking/
└── negative/

Existing test organization should be retained where already established; this structure describes the target coverage rather than requiring unnecessary renames.

---

100. Required Integration Programs

The validation suite should include, as applicable:

minimal.zm
classical.zm
generic.zm
quantum.zm
hybrid.zm
hdl.zm
poco-reaf.zm

and feature-oriented programs covering:

reasoning
knowledge
learning
adaptation
contracts
provenance
policies
uncertainty
agents
neural-symbolic composition
sandboxing
simulation
FFI
patterns
advanced types
quantum learning
hybrid AI/quantum execution

The filenames should remain descriptive and should not turn application concepts into core language keywords.

---

101. The Full Production Validation Trace

A production-ready grammar feature must be traceable through:

SPECIFICATION
    |
    v
LEXER
    |
    v
GRAMMAR
    |
    v
AST
    |
    v
STRUCTURAL VALIDATION
    |
    v
TYPE CHECKING
    |
    v
EFFECT CHECKING
    |
    v
CAPABILITY CHECKING
    |
    v
RESOURCE CHECKING
    |
    v
CONTRACT CHECKING
    |
    v
POLICY CHECKING
    |
    v
PROVENANCE
    |
    v
CANONICAL IR
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
RESILIENCE
    |
    v
ZQN
    |
    v
HAL
    |
    v
TARGET

Not every feature uses every phase.

However, every feature must explicitly identify where it enters and where it exits the pipeline.

---

102. Feature Contract Requirement

Every validation ".g4" file must document:

Purpose

Why the file exists.

Owns

Exact validation entry points owned by the file.

Does Not Own

Canonical syntax and semantic responsibilities owned elsewhere.

Public Rules

Rules intended for composition.

Private Rules

Internal helper rules.

Lexer Dependencies

Tokens consumed.

Grammar Dependencies

Imported grammars.

AST Contract

Expected AST representation.

Semantic Contract

Semantic interpretation.

Type Contract

Type-system relationship.

Effect Contract

Effects produced or required.

Capability Contract

Capabilities required.

Resource Contract

Resource implications.

Contract Contract

Interaction with "requires", "ensures", "invariant", "assume", "guarantee", and "property".

Policy Contract

Policy interaction.

Provenance Contract

Source/decision/evidence traceability.

IR Contract

Canonical IR destination.

Quantum Boundary

Required quantum integration, if applicable.

HDL Boundary

Required hardware integration, if applicable.

Backend Boundary

How later compilation consumes the result.

Diagnostics

Required diagnostic conditions.

Tests

Positive, negative, boundary, scalability, deterministic, compatibility, and portability tests.

Integration

Exact upstream and downstream dependencies.

Completion Criteria

Objective definition of DONE.

---

103. Independent-File-First Rule

A validation file is designed first as an independent unit.

The workflow is:

1. define purpose
2. define ownership
3. define dependencies
4. define public interface
5. define canonical owner
6. define AST contract
7. define semantic contract
8. define IR contract
9. define tests
10. validate independently
11. integrate
12. validate integration

A later file should not force a completed file to be rewritten merely because the later file exists.

If a genuine interface change is necessary, the dependency contract must identify it explicitly.

---

104. No Hidden Transitive Dependencies

A validation file must not depend accidentally on a rule only because another imported grammar happens to import it.

If a validation file directly consumes:

expression

it should explicitly import the grammar owning "expression".

If it consumes:

statementTerminator

it should explicitly import the grammar owning that rule.

This makes the file independently understandable and prevents fragile import chains.

---

105. Complete Validation Entry Point

A standalone validation grammar should normally expose:

featureValidationUnit
    : canonicalRule EOF
    ;

The root validation composition grammar should own the final:

validationUnit
    : validationItem EOF
    ;

This prevents nested "EOF" problems.

---

106. Validation Severity

Validation findings should be categorized consistently.

Recommended severity:

INFO
WARNING
ERROR
FATAL

Examples:

unresolved import             ERROR
duplicate canonical rule      ERROR
unresolved ambiguity          ERROR
missing AST contract          ERROR
deprecated construct          WARNING
experimental construct        INFO/WARNING
unused test-only rule         INFO
validation resource exhausted ERROR

Severity must be deterministic and machine-readable.

---

107. Machine-Readable Results

Validation tooling should be able to emit structured results containing:

validator_version
repository_revision
grammar_revision
feature
category
severity
code
message
path
line
column
related_paths
status

The exact serialization format belongs to validation tooling rather than grammar syntax.

---

108. CI Integration

Production validation should run in CI after:

lexer generation
grammar generation
parser generation
frontend tests
AST tests
semantic tests
IR tests

Where practical, lightweight grammar validation should also run before expensive compilation stages.

The CI pipeline should distinguish:

grammar failure
AST failure
semantic failure
IR failure
test failure
toolchain failure

instead of collapsing everything into one generic error.

---

109. Compatibility with Rust 1.97 / 1.97.1

Grammar files themselves are ANTLR artifacts.

The repository implementation and validation tooling must remain compatible with:

Rust 1.97
Rust 1.97.1
Edition 2021

No grammar design should require language features unavailable under that baseline.

---

110. Safety Boundary

The validation architecture must require safe Rust implementation.

Grammar files contain no Rust actions.

Validation tooling must not depend on privileged execution.

Any operation involving:

- filesystem;
- network;
- process execution;
- foreign interfaces;
- hardware;
- external services

must occur through explicitly designed tooling boundaries with their own security and capability contracts.

---

111. What Validation Must Never Do

Validation must never:

select a CPU
select a GPU
select a QPU
select a physical qubit
route a circuit
schedule hardware
perform QEC
choose calibration
allocate hardware
execute a program
contact a device
rewrite program meaning
invent syntax
invent AST nodes
invent a semantic IR
invent a backend

It verifies contracts.

---

112. Production Readiness Matrix

The validation subsystem is production-ready only when all applicable dimensions are satisfied:

Dimension| Required
Lexer| validated
Parser| validated
Grammar composition| validated
Imports| resolved
Rules| unique
Alternatives| reachable/intentional
Recursion| safe
Ambiguity| resolved/classified
Precedence| specified
AST| covered
Semantics| covered
Types| covered
Effects| covered
Capabilities| covered
Resources| covered
Contracts| covered
Policies| covered
Provenance| covered
Classical IR| covered
Quantum IR| covered where applicable
HDL| covered where applicable
Interoperability| covered where applicable
Diagnostics| tested
Determinism| tested
Compatibility| tested
Scalability| tested
Portability| tested
Security| tested
Cross-domain integration| tested

---

113. Completion Criteria for "grammar/validation/"

The validation subsystem is production-ready when:

[ ] Every validation grammar has an explicit owner.
[ ] Every validation grammar delegates to canonical syntax.
[ ] No validation grammar silently defines new core syntax.
[ ] All imports resolve.
[ ] Canonical entry points are known.
[ ] Standalone entry points are classified.
[ ] Duplicate rules are detected.
[ ] Duplicate tokens are detected.
[ ] Keyword collisions are detected.
[ ] Undefined references are detected.
[ ] Unreachable rules are classified.
[ ] Unreachable alternatives are detected.
[ ] Left recursion is checked.
[ ] Nullable cycles are checked.
[ ] Ambiguity is classified.
[ ] Precedence is validated.
[ ] Lexer/parser agreement is validated.
[ ] Source spans are validated.
[ ] AST coverage is validated.
[ ] Semantic coverage is validated.
[ ] Effect coverage is validated.
[ ] Capability coverage is validated.
[ ] Resource coverage is validated.
[ ] Contract coverage is validated.
[ ] Policy coverage is validated.
[ ] Provenance coverage is validated.
[ ] Canonical IR coverage is validated.
[ ] Quantum constructs converge on quantum::ir.
[ ] HDL constructs converge on the hardware semantic pipeline.
[ ] Interoperability has explicit boundaries.
[ ] Dialects cannot silently redefine core syntax.
[ ] No artificial hardware ceilings are encoded.
[ ] No artificial language-size ceilings are encoded.
[ ] Deterministic validation is guaranteed.
[ ] Scalability tests exist.
[ ] Portability tests exist.
[ ] Compatibility tests exist.
[ ] Cross-domain tests exist.
[ ] Negative tests exist.
[ ] Boundary tests exist.
[ ] Diagnostic tests exist.
[ ] CI validation is integrated.
[ ] Rust 1.97 / 1.97.1 compatibility is preserved.
[ ] Safe Rust is sufficient for all validation tooling.

---

114. Final Architecture

The intended final relationship is:

                         ZAMANI
                           |
             +-------------+-------------+
             |                           |
       LANGUAGE AUTHORITY          VALIDATION
             |                           |
             v                           v
      specification                conformance
             |                           |
             v                           |
       canonical lexer                  |
             |                           |
             v                           |
      canonical grammar <---------------+
             |
             v
            AST
             |
             v
       semantic model
             |
    +--------+--------+----------------+
    |        |        |                |
 classical quantum   HDL              AI
    |        |        |                |
    +--------+--------+----------------+
             |
             v
      resources/effects/
      capabilities/contracts/
      policies/provenance
             |
             v
       canonical IR
             |
       +-----+------+
       |            |
classical IR    quantum::ir
       |            |
       +-----+------+
             |
       optimization
             |
       lowering
             |
       routing
             |
       scheduling
             |
       resilience/QEC
             |
            ZQN
             |
            HAL
             |
       target realization

The validation subsystem therefore becomes a cross-cutting conformance layer, not another language layer.

---

115. Final Principle

The central rule for "grammar/validation/" is:

«Validate every layer, own only validation, and never duplicate the authority of another layer.»

That gives Zamani the required architecture for:

tiny systems
        |
        v
embedded
        |
        v
CPU
        |
        v
multicore
        |
        v
GPU
        |
        v
FPGA
        |
        v
ASIC
        |
        v
accelerator
        |
        v
QPU
        |
        v
simulator
        |
        v
HPC
        |
        v
cluster
        |
        v
distributed systems
        |
        v
cloud
        |
        v
future computational systems

without turning today's hardware characteristics into tomorrow's language limits.

The source program expresses portable computational intent.

The grammar represents that intent.

The AST preserves that intent.

The semantic system determines its meaning.

Resources and capabilities determine feasibility.

Policies and contracts constrain realization.

Canonical IR represents executable meaning.

The backend determines realization.

Validation verifies that every one of those boundaries remains intact.

That is the correct production role of "grammar/validation/" in a POCO-REAF Zamani architecture.