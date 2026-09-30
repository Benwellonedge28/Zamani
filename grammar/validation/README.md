Zamani Grammar Validation

Path: "grammar/validation/README.md"
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Primary branch: "main"
Grammar technology: ANTLR4-compatible grammar composition
Rust baseline: Rust 1.97 / Rust 1.97.1, Edition 2021
Safety requirement: production Rust validation code MUST use safe Rust; "unsafe" is prohibited
Primary objective: repository-wide production validation of the Zamani language grammar and its integration contracts
Scalability objective: from the smallest valid grammar/program to arbitrarily large grammars/programs subject only to actual representation, execution, configured validation budgets, and resources actually available
Portability objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

"grammar/validation/" is the automated correctness and conformance layer for the Zamani grammar subsystem.

It does not define a second language.

It verifies that the language represented by:

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
src/lexer.rs
        |
        v
src/parser.rs
        |
        v
src/ast/
        |
        v
semantic analysis
        |
        v
canonical IR
        |
        +--------------------+
        |                    |
        v                    v
classical semantics     quantum::ir
        |                    |
        +---------+----------+
                  |
                  v
          compiler pipeline
                  |
        +---------+----------+
        |                    |
        v                    v
routing/scheduling     resilience/QEC/ZQN
        |                    |
        +---------+----------+
                  |
                  v
                 HAL
                  |
                  v
          target realization

is internally consistent.

Validation MUST therefore cover more than whether ANTLR can generate a parser.

A grammar feature is production-ready only when its entire contract can be traced and validated through the relevant downstream layers.

---

2. Validation Is Not the Language Authority

The authority hierarchy remains:

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
src/lexer.rs / src/parser.rs
        |
        v
src/ast/
        |
        v
semantic analysis
        |
        v
canonical IR

Validation observes and checks this hierarchy.

It MUST NOT silently promote a proposed feature into accepted syntax.

In particular:

- "grammar/Zamani-Grammar.md" does not automatically define legal syntax.
- A README does not define legal syntax.
- A test fixture does not define legal syntax.
- A leaf ".g4" file does not become canonical merely because it exists.
- An unreachable rule does not become reachable merely because a domain directory exists.
- A semantic capability does not automatically become a keyword.
- A target capability does not automatically become a grammar construct.

---

3. Existing Repository Contracts

The validation subsystem integrates with the existing repository rather than replacing it.

Artifact| Validation relationship
"grammar/DESIGN.md"| Normative architecture and ownership
"grammar/README.md"| Grammar subsystem navigation and authority
"grammar/specification/"| Human-readable normative specification
"grammar/spec/"| Formal language contracts
"grammar/Zamani.g4"| Canonical ANTLR root/composition contract
"grammar/antlr/ZamaniParser.g4"| Canonical parser composition
"grammar/antlr/ZamaniLexer.g4"| Canonical lexer composition
"grammar/*/*.g4"| Domain/modular grammar implementation
"src/lexer.rs"| Executable lexical implementation
"src/parser.rs"| Executable parser implementation
"src/ast/"| Source AST
"src/frontend/"| Frontend integration
"src/semantic.rs"| Semantic validation
"src/ir_gen.rs"| AST/semantic → IR lowering
"src/ir_verify.rs"| IR verification
"src/quantum/ir/"| Canonical quantum semantic/IR boundary
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Extended/historical/proposed design material
"grammar/tests/"| Grammar conformance evidence
"grammar/validation/"| Validation contracts and validation policy

Validation MUST preserve these ownership boundaries.

---

4. Primary Validation Pipeline

The production validation pipeline is:

Repository
    |
    v
File inventory
    |
    v
Grammar composition discovery
    |
    v
Import/dependency resolution
    |
    v
Lexical validation
    |
    v
Parser-rule validation
    |
    +--> undefined rules
    +--> duplicate rules
    +--> duplicate tokens
    +--> keyword collisions
    +--> unreachable rules
    +--> unreachable alternatives
    +--> left recursion
    +--> ambiguity
    +--> precedence
    +--> nullable-cycle analysis
    +--> EOF/entry-point analysis
    |
    v
Source-span validation
    |
    v
AST coverage
    |
    v
Semantic coverage
    |
    v
IR coverage
    |
    v
Domain integration
    |
    +--> classical
    +--> quantum
    +--> hybrid
    +--> HDL
    +--> hardware
    +--> AI
    +--> distributed
    +--> data
    +--> networking
    +--> security
    +--> resources
    +--> execution
    +--> interoperability
    +--> dialects
    |
    v
Portability validation
    |
    v
Hard-coding/scalability audit
    |
    v
Diagnostics validation
    |
    v
Determinism validation
    |
    v
Compatibility validation
    |
    v
Positive/negative/boundary/scalability tests
    |
    v
Production acceptance

No individual parser warning is sufficient to declare the grammar production-ready.

---

5. Validation Categories

The validation subsystem MUST cover at least:

1. grammar composition;
2. import resolution;
3. lexical correctness;
4. duplicate tokens;
5. duplicate keywords;
6. keyword collisions;
7. duplicate parser rules;
8. undefined parser references;
9. unreachable rules;
10. unreachable alternatives;
11. intentionally standalone rules;
12. test-only rules;
13. generated/support rules;
14. deprecated rules;
15. experimental rules;
16. left recursion;
17. indirect left recursion;
18. nullable recursion;
19. grammar ambiguity;
20. precedence;
21. associativity;
22. parser entry points;
23. EOF reachability;
24. source spans;
25. error recovery;
26. AST coverage;
27. semantic coverage;
28. canonical IR coverage;
29. quantum "quantum::ir" integration;
30. resource/capability separation;
31. portability;
32. hard-coded artificial limits;
33. scalability;
34. determinism;
35. diagnostics;
36. compatibility;
37. domain integration;
38. dialect isolation;
39. interoperability;
40. generated-file consistency.

---

6. Validation Files

The existing validation directory is the correct place for focused validation contracts.

The intended structure is:

grammar/validation/
├── README.md
├── grammar-validator.md
├── ambiguity.md
├── left-recursion.md
├── unreachable-rules.md
├── duplicate-tokens.md
├── keyword-collisions.md
├── precedence.md
├── source-spans.md
├── ast-coverage.md
├── semantic-coverage.md
├── ir-coverage.md
├── portability.md
├── hard-coding.md
├── scalability.md
└── determinism.md

Existing files MUST be retained.

New files are added only where the validation concern is not already represented.

"README.md" owns the cross-validation architecture.

The individual validation contracts own their specialized rules.

They MUST NOT duplicate one another's authority.

---

7. Standard Validation Contract

Every validation contract under this directory MUST define:

Purpose
Status
Scope
Owns
Does Not Own
Inputs
Outputs
Dependencies
Repository Files
Authority
Algorithm
Diagnostics
Severity
Positive Cases
Negative Cases
Boundary Cases
Scalability Cases
Determinism Requirements
Compatibility Requirements
AST Integration
Semantic Integration
IR Integration
Compiler Integration
Runtime/Backend Integration
Security Considerations
Resource Considerations
Completion Criteria

This is mandatory because a validation file must be independently complete.

A later file changing another subsystem MUST NOT require the validation contract to be rewritten merely to explain its fundamental ownership.

Cross-file changes should be detected through validation results and conformance matrices rather than hidden assumptions.

---

8. "grammar-validator.md"

Owns the general validation engine contract.

It MUST define:

- validation phases;
- validation inputs;
- repository discovery;
- grammar loading;
- grammar identity;
- dependency graph construction;
- rule graph construction;
- diagnostic model;
- validation severity;
- deterministic ordering;
- validation result aggregation;
- failure policy;
- machine-readable output;
- human-readable output;
- CI integration;
- configured resource budgets.

It MUST NOT own the detailed semantics of each individual validation category.

Those belong to their dedicated contracts.

The validator MUST be deterministic.

Given:

same repository snapshot
same grammar configuration
same validation configuration
same tool version

it MUST produce equivalent validation results.

---

9. Grammar Identity

Validation MUST distinguish:

file path
grammar name
grammar type
grammar role
composition role
domain
status
entry-point status

A directory named:

grammar/quantum/

does not itself define a grammar named "Quantum".

A file:

grammar/quantum/operations.g4

does not automatically become part of the canonical parser.

The validator MUST use actual ANTLR grammar declarations and imports.

It MUST NOT infer grammar composition from directory names alone.

---

10. Canonical Entry Points

The complete Zamani source entry point is:

grammar/Zamani.g4
        |
        v
program
        |
        v
sourceUnit
        |
        v
sourceElement*
        |
        v
EOF

Validation MUST begin complete-program reachability from the canonical entry point.

It MUST NOT arbitrarily select a domain rule as the canonical source entry point.

Examples of invalid root assumptions include:

quantumOperation
hdlModule
tensorExpression
agent
gpuKernel

These may be valid standalone tooling entry points, but they are not automatically the Zamani source root.

---

11. Multiple Entry Points

A grammar may intentionally expose additional entry points for:

- editor tooling;
- syntax fragments;
- interoperability formats;
- test grammars;
- language-server features;
- migration tools;
- domain-specific tooling.

These MUST be explicitly classified.

A production validator MUST therefore distinguish:

CANONICAL_ENTRY
STANDALONE_ENTRY
TEST_ENTRY
TOOLING_ENTRY
GENERATED_ENTRY
DEPRECATED_ENTRY

An additional entry point MUST NOT silently alter complete-program semantics.

---

12. Import Resolution

Every grammar import MUST be resolved.

For each import:

source grammar
source location
imported grammar name
resolved grammar file
grammar identity

MUST be recorded.

The validator MUST distinguish:

RESOLVED
UNRESOLVED
AMBIGUOUS
DUPLICATE
INCOMPATIBLE
DEPRECATED

An unresolved import MUST NOT be reported as an unreachable-rule finding.

This distinction is important.

If:

ZamaniParser
    |
    +--> Quantum

cannot resolve "Quantum", the validator cannot truthfully conclude that every rule under a presumed Quantum grammar is unreachable.

The correct result is:

UNRESOLVED_DEPENDENCY

---

13. Unreachable Rules

The existing "grammar/validation/unreachable-rules.md" remains the detailed contract for unreachable-rule analysis.

This README establishes the higher-level integration only.

The validator MUST distinguish:

REACHABLE
STANDALONE
TEST_ONLY
GENERATED_SUPPORT
DEPRECATED
EXPERIMENTAL
UNIMPLEMENTED
ORPHANED
UNRESOLVED_DEPENDENCY

A rule is "ORPHANED" only when the grammar composition is sufficiently resolved to establish that it is not reachable and it has no explicit legitimate independent role.

A rule whose grammar dependency is missing is not automatically orphaned.

---

14. Unreachable Alternatives

A reachable rule can contain an unreachable alternative.

For example:

operation
    : identifier
    | identifier
    ;

The second alternative does not introduce a distinct language path.

Likewise, an alternative can become unreachable because an earlier alternative consumes every possible input.

Validation MUST therefore operate at both:

rule level

and:

alternative level

The validator MUST report the precise grammar location.

---

15. Duplicate Rules

Within a canonical grammar composition, duplicate rule identities MUST be rejected unless the composition mechanism explicitly defines a legal override/extension model.

A validator MUST distinguish:

duplicate rule

from:

same rule name in intentionally isolated grammar

and:

same semantic concept represented by differently named rules

The latter is an architecture/conformance concern rather than necessarily an ANTLR duplicate-rule error.

---

16. Duplicate Tokens

"duplicate-tokens.md" owns token duplication validation.

The validator MUST identify collisions such as:

Question
QuestionMark

or:

Ampersand
BitAnd

when the apparent duplication represents one lexical concept.

The validator MUST not blindly reject two distinct tokens merely because their spellings overlap.

It must classify:

same spelling + same meaning
same spelling + context-dependent meaning
different spelling + same semantic meaning
intentionally distinct lexical tokens

The canonical lexical contract remains under:

grammar/lexer/

---

17. Keyword Collisions

Validation MUST detect collisions among:

- keywords;
- identifiers;
- contextual keywords;
- operators;
- domain keywords;
- dialect keywords;
- interoperability keywords.

The language MUST avoid allowing one domain to silently steal a general identifier from another domain.

Contextual keywords are permitted only when their context and compatibility behavior are explicitly specified.

---

18. Left Recursion

"left-recursion.md" owns left-recursion validation.

Validation MUST detect:

direct left recursion

and:

indirect left recursion

including cycles such as:

A → B
B → C
C → A

when the cycle permits left-recursive derivation.

Nullable rules MUST be considered.

The validator MUST NOT treat every recursive grammar as invalid.

Valid recursion is required for constructs such as:

- nested expressions;
- nested blocks;
- recursive types;
- recursive patterns;
- recursive declarations.

The goal is to identify parser-problematic recursion, not eliminate recursion from the language.

---

19. Nullable Cycles

Validation MUST detect cycles where every transition can derive an empty sequence.

For example:

A → B
B → C?
C → A?

can create a zero-consumption cycle.

Such a cycle may cause:

- nontermination;
- exponential behavior;
- parser instability;
- invalid adaptive prediction.

A production grammar MUST not contain an uncontrolled nullable cycle.

---

20. Ambiguity

"ambiguity.md" owns ambiguity analysis.

Validation MUST identify:

- exact duplicate alternatives;
- prefix collisions;
- overlapping lexical forms;
- ambiguous expression forms;
- ambiguous declaration/statement boundaries;
- contextual keyword ambiguity;
- dialect collisions;
- interoperability collisions.

Not every syntactic ambiguity is necessarily an error.

The validator MUST classify ambiguity as:

RESOLVED_BY_PRECEDENCE
RESOLVED_BY_CONTEXT
RESOLVED_BY_TOKENIZATION
RESOLVED_BY_PARSER_PREDICTION
INTENTIONAL
UNRESOLVED

"UNRESOLVED" ambiguity is a production failure.

---

21. Precedence and Associativity

"precedence.md" owns expression precedence.

Every operator with multiple possible parse interpretations MUST have an explicit precedence/associativity contract.

Validation MUST cover:

- arithmetic;
- logical;
- comparison;
- bitwise;
- shifts;
- assignment;
- ranges;
- unary operators;
- postfix operators;
- calls;
- indexing;
- member access;
- conditional expressions;
- domain-specific operators.

Adding a new operator without updating the precedence contract MUST fail validation.

---

22. Lexer/Parser Agreement

Validation MUST compare the lexical specification with the executable lexer and parser.

At minimum it must identify:

specified token but lexer missing
lexer token but specification missing
grammar token but lexer missing
lexer token never consumed
keyword accepted by grammar but unavailable lexically
literal specified but not lexed
operator specified but not tokenized

This is particularly important for literal forms such as:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

and for arbitrary-magnitude numeric literals.

A lexical feature is not complete merely because its syntax appears in a ".md" document.

---

23. Source Span Validation

"source-spans.md" owns source-location conformance.

Every source-level AST node that can produce a diagnostic MUST have sufficient source provenance.

Validation MUST ensure that relevant constructs can preserve:

source file
start offset
end offset
start line
start column
end line
end column

or an equivalent canonical source-span representation.

The exact representation belongs to the Rust frontend.

The validation layer checks its presence and consistency.

Source spans MUST survive:

lexer
→ parser
→ AST
→ semantic analysis
→ IR provenance where required
→ diagnostics

---

24. AST Coverage

"ast-coverage.md" owns grammar-to-AST traceability.

For every accepted stable syntax construct, validation MUST establish:

grammar production
        |
        v
AST representation
        |
        v
semantic interpretation

A grammar rule without a defined AST interpretation MUST NOT be marked fully implemented.

The validator must identify:

GRAMMAR_ONLY
AST_MISSING
AST_PARTIAL
AST_COMPLETE

A domain-neutral AST remains the preferred architecture.

Quantum syntax MUST NOT force the frontend into a vendor-specific or topology-specific AST.

---

25. Semantic Coverage

"semantic-coverage.md" owns grammar-to-semantic traceability.

Every stable source construct MUST identify:

semantic owner
semantic inputs
semantic outputs
validation rules
diagnostics
resource implications
capability implications
effect implications
ownership implications

The parser must not perform semantic work that belongs to semantic analysis.

For example, parsing:

requires capability("quantum.measurement")

does not mean the parser decides whether a target possesses that capability.

---

26. IR Coverage

"ir-coverage.md" owns grammar/AST/semantic-to-IR traceability.

A stable feature MUST identify its canonical lowering.

The required chain is:

Source
  |
  v
Grammar
  |
  v
AST
  |
  v
Semantic Model
  |
  v
Canonical IR

A feature MUST NOT introduce an unnecessary second semantic IR.

---

27. Quantum IR Invariant

Quantum syntax MUST ultimately use the repository's canonical:

src/quantum/ir/

boundary.

Validation MUST reject architectural drift toward:

grammar
    ↓
frontend-specific quantum IR
    ↓
second quantum IR
    ↓
canonical quantum IR

The preferred chain remains:

Zamani source
    |
    v
domain-neutral AST
    |
    v
semantic quantum model
    |
    v
src/quantum/ir/
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

---

28. Quantum Gate Scalability

Validation MUST specifically prevent the grammar from becoming a fixed list of today's quantum gates.

This architecture is preferred:

quantumOperation
    : operationSpecifier quantumTargetList
    ;

rather than a universal enumeration such as:

H
X
Y
Z
CNOT
...

Validation MUST permit:

H
X
custom_gate
vendor.operation
operation(parameter)
future.operation

provided the semantic contract defines how unknown/custom operations are represented.

The grammar MUST NOT impose a fixed universal gate inventory.

---

29. Quantum Resource Scalability

Validation MUST reject grammar-level artificial limits such as:

MAX_QUBITS
MAX_LOGICAL_QUBITS
MAX_PHYSICAL_QUBITS
MAX_QUANTUM_GATES
MAX_QPU_COUNT

The grammar may represent:

Qubit
Qubit[n]

where "n" is program semantics.

Resource feasibility belongs downstream.

---

30. HDL and Hardware Validation

Validation MUST ensure that HDL and hardware grammar describe intent rather than today's machine dimensions.

The validator MUST flag universal constructs equivalent to:

wire [31:0]

when "32" is being used as a language-imposed universal hardware width.

It MUST distinguish that from a programmer explicitly declaring:

width = 32

as program semantics.

The same distinction applies to:

- bus widths;
- registers;
- memory;
- pipeline depth;
- device count;
- accelerator count;
- topology;
- clock domains;
- physical pins.

---

31. Resource and Capability Validation

Validation MUST preserve the distinction between:

requirement
capability
constraint
preference
hint
realization

For example:

requires qubits >= n

is not equivalent to:

use physical qubit 17

Likewise:

requires capability("gpu.compute")

does not select a particular GPU.

Validation MUST ensure that target realization does not leak backward into universal source syntax.

---

32. Portability Validation

"portability.md" owns POCO-REAF validation.

A source construct is portable when its meaning does not depend unnecessarily on a particular:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- accelerator;
- node;
- operating system;
- vendor runtime;
- physical topology;
- register width;
- memory bank;
- device identifier.

Portability validation MUST not require every target to support every feature.

Instead it verifies that target-specific feasibility is represented as:

capability
requirement
constraint
preference
profile
dialect
interoperability boundary

rather than as accidental language-wide limitations.

---

33. Hard-Coding Validation

"hard-coding.md" owns artificial-limit detection.

The validator MUST inspect grammar, specification, tests, validation rules, and relevant implementation metadata for accidental universal limits.

The audit MUST include at least:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ACCELERATORS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_TIMELINES
MAX_AGENTS
MAX_GATE_COUNT

This list is an audit vocabulary, not a language feature.

The validator MUST also detect equivalent hard-coded forms even when different names are used.

Examples include:

qubit_count <= 1024
threads < 256
tensor_rank == 8
register_width == 32
nodes <= 1024

when these are presented as universal language constraints.

---

34. Numeric Literals Are Not Automatically Hard-Coded Limits

The hard-coding validator MUST NOT falsely reject ordinary program constants.

Valid:

let n = 1024;
let width = 32;
matrix<1024, 1024>;
allocate n;

These are program semantics.

Invalid architecture:

Zamani supports at most 1024 qubits.

or:

MAX_QUBITS = 1024

when this defines a universal compiler/language ceiling.

The validator must understand the difference between:

program data

and:

implementation limit

---

35. Scalability Validation

"scalability.md" owns scalability conformance.

The language must scale conceptually from:

one value
one operation
one function
one qubit
one process
one device

to:

large programs
large datasets
large tensors
large quantum computations
large classical computations
large distributed systems
large heterogeneous systems
future computational systems

subject to actual resources and explicit semantic constraints.

Validation MUST reject artificial grammar ceilings.

It MUST NOT require the validator itself to allocate an unbounded amount of memory.

---

36. Validation Budgets

"Unlimited scalability" does not mean an implementation may be forced into nontermination.

Validation tools MAY have explicit operational budgets for:

- wall-clock time;
- memory;
- recursion depth;
- graph size;
- diagnostics;
- generated output;
- parser work;
- test workload.

These are tool execution policies, not language limits.

A budget MUST be:

explicit
configurable
documented
diagnosable
independent of source-language semantics

A validation budget exhaustion MUST be reported distinctly from a grammar error.

For example:

VALIDATION_BUDGET_EXCEEDED

must not be reported as:

INVALID_GRAMMAR

---

37. No Fixed Validation Depth

Validation algorithms MUST avoid architecture such as:

MAX_RULE_DEPTH = 1024

as an implicit correctness boundary.

Where a traversal requires protection against pathological input, use explicit validation resources and iterative algorithms where practical.

The validator must be able to process progressively larger grammar graphs without source-language changes.

---

38. Determinism

"determinism.md" owns deterministic validation behavior.

The same repository snapshot and configuration MUST produce stable:

- rule ordering;
- diagnostics;
- file ordering;
- dependency ordering;
- graph traversal;
- classification;
- generated reports.

Filesystem enumeration order MUST NOT determine semantic validation results.

Hash-map iteration order MUST NOT determine diagnostic ordering.

Parallel validation, if introduced, MUST preserve deterministic output ordering.

---

39. Diagnostics

Every validation diagnostic MUST contain enough information to act on it.

At minimum:

code
severity
message
file
source location where available
rule/token/grammar identity where applicable
classification
related location(s)
suggested remediation where deterministic

Severity SHOULD distinguish:

ERROR
WARNING
INFO
NOTE

Validation MUST define which severities fail production acceptance.

Unknown or malformed validation states MUST NOT silently pass.

---

40. Diagnostic Stability

Diagnostic codes should be stable across compatible tool releases.

Examples:

GRAMMAR_UNDEFINED_RULE
GRAMMAR_DUPLICATE_RULE
GRAMMAR_DUPLICATE_TOKEN
GRAMMAR_KEYWORD_COLLISION
GRAMMAR_UNREACHABLE_RULE
GRAMMAR_UNREACHABLE_ALTERNATIVE
GRAMMAR_UNRESOLVED_IMPORT
GRAMMAR_AMBIGUITY
GRAMMAR_LEFT_RECURSION
GRAMMAR_NULLABLE_CYCLE
GRAMMAR_PRECEDENCE_CONFLICT
GRAMMAR_AST_COVERAGE_MISSING
GRAMMAR_SEMANTIC_COVERAGE_MISSING
GRAMMAR_IR_COVERAGE_MISSING
GRAMMAR_HARD_CODED_LIMIT
GRAMMAR_PORTABILITY_VIOLATION
GRAMMAR_NONDETERMINISTIC_VALIDATION
GRAMMAR_VALIDATION_BUDGET_EXCEEDED

The final registry belongs to the validation tooling implementation.

---

41. Positive Tests

Positive tests prove that valid syntax remains accepted.

They MUST cover:

- core language;
- expressions;
- types;
- declarations;
- statements;
- functions;
- modules;
- effects;
- memory;
- concurrency;
- classical;
- quantum;
- hybrid;
- HDL;
- hardware;
- resources;
- distributed;
- AI;
- data;
- networking;
- security;
- compile;
- execution;
- interoperability;
- dialects;
- macros;
- metaprogramming.

A positive parse is not sufficient for a stable feature.

The relevant AST, semantic, IR, and downstream contracts must also be exercised.

---

42. Negative Tests

Negative tests prove that invalid constructs are rejected with the correct diagnostic.

They MUST cover:

- malformed syntax;
- undefined grammar constructs;
- invalid token sequences;
- ambiguous constructs where ambiguity is forbidden;
- illegal resource forms;
- invalid capability forms;
- unsupported target requirements;
- invalid type combinations;
- invalid quantum operation forms;
- invalid HDL declarations;
- invalid module structure;
- invalid dialect usage;
- compatibility violations.

The validator MUST verify not only rejection but diagnostic classification.

---

43. Boundary Tests

Boundary tests must test the smallest and largest practical representative cases without defining artificial language ceilings.

Examples:

one declaration
many declarations

one function
deeply nested functions

one qubit
many qubits

one tensor dimension
large tensor dimensions

one process
large process graph

one hardware resource
large resource description

The test suite MUST NOT turn the largest test fixture into a language maximum.

---

44. Scalability Tests

Scalability tests MUST use generated or parameterized cases where appropriate.

They should progressively exercise:

small
medium
large
very large
resource-constrained
resource-abundant

The expected invariant is:

larger valid input
        ≠
automatically invalid input

Failure due to an explicitly exhausted validation budget must be distinguishable from language rejection.

---

45. Grammar → AST → Semantic → IR Coverage

Production validation MUST maintain a traceability chain:

Feature ID
    |
    +--> specification
    |
    +--> lexical contract
    |
    +--> grammar rule
    |
    +--> lexer token
    |
    +--> parser rule
    |
    +--> AST node
    |
    +--> semantic rule
    |
    +--> IR representation
    |
    +--> compiler consumer
    |
    +--> runtime/backend consumer
    |
    +--> tests

A missing link is a conformance finding.

This is the mechanism that prevents:

«"The parser accepts it, therefore it is implemented."»

from becoming a production mistake.

---

46. Domain Integration

Validation MUST ensure that all major domains participate in the same language architecture.

At minimum:

Classical
Quantum
Hybrid
HDL
Hardware
AI
Distributed
Data
Networking
Security
Resources
Compile
Execution
Interoperability
Dialects
Macros
Metaprogramming

must integrate with the common:

lexical system
identifiers
source spans
types
expressions
declarations
statements
modules
effects
diagnostics
capabilities
resources
compatibility

A domain-specific grammar MUST NOT silently become a separate language.

---

47. Quantum Domain Validation

Quantum validation MUST cover:

qubit types
registers
states
operations
parameters
controls
adjoints
measurement
reset
barriers
classical feed-forward
dynamic control
observables
channels
noise
error correction
logical operations
pulse intent
circuits
kernels
resource requirements
interoperability

The validator MUST ensure that these constructs remain target-independent at the grammar level.

Physical topology, routing, calibration, pulse realization, QEC implementation, and device selection belong downstream.

---

48. HDL Domain Validation

HDL validation MUST cover:

modules
ports
signals
nets
registers
combinational logic
sequential logic
clocking
reset
timing
assertions
interfaces
protocols
state machines
pipelines
memories
arrays
parameters
generate constructs
synthesis intent
simulation intent
verification intent
physical intent
co-design

The validator MUST ensure that parameterized intent is not replaced by universal fixed machine dimensions.

---

49. Hardware Domain Validation

Hardware validation MUST cover:

target
capability
resource
compute
memory
accelerator
quantum device
interconnect
topology
timing
power
thermal
reliability
calibration
constraint
negotiation
deployment

The grammar must describe requirements and intent.

The validator must reject architecture that turns discovered hardware into universal source syntax.

---

50. Distributed Validation

Distributed constructs MUST remain scalable.

The validator MUST NOT impose universal limits on:

- nodes;
- processes;
- actors;
- channels;
- services;
- replicas;
- partitions;
- messages;
- topology size.

The source language can describe explicit numbers where they are program semantics.

The implementation must not silently convert them into compiler-wide maxima.

---

51. AI and Data Validation

AI/data validation MUST ensure that framework-specific implementation does not become universal syntax.

Validation may cover:

model
tensor
dataset
training
inference
optimization
differentiation
probabilistic computation
agents
pipelines
distributed training
deployment

but a framework-specific backend belongs downstream.

The grammar MUST NOT become a list of framework APIs.

---

52. Interoperability Validation

Interoperability grammars MUST be clearly classified.

Examples include:

OpenQASM
QIR
LLVM-related representations
MLIR-related representations
HDL formats
C
C++
Python
Rust
Wasm

These formats MUST NOT silently become the canonical Zamani semantic model.

Validation MUST identify:

source language
foreign format
import boundary
export boundary
semantic mapping
lossless/lossy status
version
compatibility

---

53. Dialect Validation

Every dialect MUST explicitly identify:

name
version
owner
status
syntax extensions
semantic extensions
AST mapping
IR mapping
capabilities
feature gates
compatibility

A dialect MUST NOT silently change core Zamani semantics.

Validation MUST ensure that a dialect can be disabled without corrupting canonical core-language parsing.

---

54. Compatibility Validation

Compatibility validation MUST compare relevant versions of:

specification
Zamani.g4
lexer
parser
AST
semantic analysis
IR
compiler
runtime
dialects
interoperability formats
tests

Changes MUST be classified as:

source-compatible
source-incompatible
semantic-compatible
semantic-incompatible
diagnostic-compatible
diagnostic-incompatible
IR-compatible
IR-incompatible
deprecated
experimental

Compatibility metadata MUST not be inferred solely from filenames.

---

55. Generated Artifacts

Validation MUST distinguish:

authoritative source
generated artifact
derived documentation
cache
build output
test output

Generated files MUST NOT become accidental authorities.

For example:

grammar/grammar.md

may be generated/derived, but it must not silently override:

grammar/specification/
grammar/spec/
grammar/Zamani.g4

when discrepancies occur.

---

56. Repository Drift Detection

Validation MUST detect drift between:

specification
grammar
lexer
parser
AST
semantic analysis
IR
tests

Examples:

specification says feature exists
grammar does not parse it

grammar parses feature
AST has no representation

AST represents feature
semantic analyzer ignores it

semantic analyzer supports feature
IR lowering drops it

IR supports feature
tests never exercise it

Each must receive a precise conformance result.

---

57. Feature Status

Validation SHOULD support a common feature status model:

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
PLANNED
NOT_IMPLEMENTED

A feature may have multiple implementation dimensions.

For example:

PARSER_IMPLEMENTED
AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_NOT_IMPLEMENTED

must not be summarized simply as:

IMPLEMENTED

---

58. Validation and "grammar.md"

"grammar/grammar.md" remains the implementation-conformance reference.

Validation should provide the evidence required to classify grammar features accurately.

The intended relationship is:

validation results
        |
        v
implementation conformance
        |
        v
grammar/grammar.md

"grammar.md" should therefore not claim a feature is implemented solely because a grammar rule exists.

---

59. Validation and "Zamani-Grammar.md"

"Zamani-Grammar.md" remains an extended design source.

Validation MUST NOT automatically parse its contents as executable grammar.

Features from that file must follow:

proposal
    |
    v
semantic design
    |
    v
specification
    |
    v
AST contract
    |
    v
canonical grammar
    |
    v
implementation
    |
    v
IR
    |
    v
tests
    |
    v
stable

This prevents the historical/experimental document from becoming a competing language authority.

---

60. Security

Validation tooling MUST use safe Rust.

Production validation code MUST contain no:

unsafe

The validator MUST treat grammar input as untrusted input.

It MUST defend against:

- pathological recursion;
- enormous token streams;
- pathological grammar graphs;
- excessive diagnostics;
- resource exhaustion;
- malformed Unicode;
- malformed files;
- cyclic imports;
- dependency explosions;
- adversarially large identifiers/literals.

Resource protection must be implemented through explicit validation budgets and safe algorithms.

Those budgets are implementation controls, not Zamani language limits.

---

61. No Unsafe Rust

The validation implementation MUST target:

Rust 2021
Rust 1.97 / Rust 1.97.1

and MUST use safe Rust only.

This means:

no unsafe blocks
no unsafe functions
no unsafe traits
no unsafe implementations

unless the repository's project-wide policy is explicitly changed later.

The validation architecture should therefore prefer:

- owned values;
- borrowing;
- safe collections;
- explicit graph structures;
- iterative traversal;
- bounded work queues;
- typed diagnostics;
- deterministic sorting.

---

62. Memory and Time Safety

The validator MUST avoid algorithms whose cost can explode unnecessarily for ordinary grammar growth.

Preferred approaches include:

indexed graph representations
memoized reachability
strongly connected component analysis
work-list algorithms
iterative traversal
deterministic sorting
incremental validation where safe

Validation correctness MUST NOT depend on recursion depth that grows directly with arbitrary source size.

---

63. Strongly Connected Components

Graph-based validation SHOULD use strongly connected components where appropriate for:

- import cycles;
- rule cycles;
- nullable cycles;
- dependency cycles;
- dialect dependency cycles.

A cycle is not automatically invalid.

The validator must determine whether the cycle is semantically permitted.

For example:

recursive type

may be valid.

Whereas:

nullable parser cycle

may be invalid.

---

64. Validation Result Model

The validation subsystem SHOULD conceptually produce a structured result:

ValidationReport
    |
    +-- repository
    +-- revision
    +-- validator version
    +-- configuration
    +-- grammar graph
    +-- lexical findings
    +-- parser findings
    +-- reachability findings
    +-- ambiguity findings
    +-- precedence findings
    +-- source-span findings
    +-- AST findings
    +-- semantic findings
    +-- IR findings
    +-- portability findings
    +-- scalability findings
    +-- hard-coding findings
    +-- compatibility findings
    +-- diagnostics
    +-- production status

The concrete Rust types belong to the validation implementation, not this README.

---

65. Production Acceptance

The validation subsystem MUST define an explicit production gate.

A grammar snapshot is production-acceptable only when:

[ ] canonical root resolves
[ ] canonical parser resolves
[ ] canonical lexer resolves
[ ] all required imports resolve
[ ] no unresolved required grammar dependency exists
[ ] no illegal duplicate rule exists
[ ] no illegal duplicate token exists
[ ] no unresolved keyword collision exists
[ ] no invalid left recursion exists
[ ] no invalid nullable cycle exists
[ ] no unresolved ambiguity exists
[ ] precedence is validated
[ ] source spans are validated
[ ] stable syntax has AST coverage
[ ] stable syntax has semantic coverage
[ ] stable syntax has IR coverage
[ ] quantum features map through quantum::ir
[ ] portability constraints pass
[ ] artificial hard-coded limits pass
[ ] scalability audit passes
[ ] deterministic validation passes
[ ] diagnostics are valid
[ ] compatibility checks pass
[ ] positive tests pass
[ ] negative tests pass
[ ] boundary tests pass
[ ] scalability tests pass
[ ] relevant domain integration tests pass

A failure in any mandatory gate prevents a production-ready status.

---

66. What Validation MUST NOT Do

The validation subsystem MUST NOT:

- define new Zamani syntax;
- choose hardware targets;
- allocate physical resources;
- perform routing;
- perform scheduling;
- perform QEC;
- perform calibration;
- implement ZQN;
- implement HAL;
- choose a QPU;
- choose a GPU;
- choose an FPGA;
- choose a CPU;
- impose hardware-derived grammar limits;
- replace semantic analysis;
- replace IR verification;
- create a second quantum IR;
- turn proposed syntax into stable syntax;
- silently modify source files;
- silently rename existing files;
- silently delete existing grammar files.

Validation is an observer and correctness gate.

---

67. Integration With Compiler Stages

The complete integration remains:

Source
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
Name/module resolution
  |
  v
Type/effect/ownership analysis
  |
  v
Resource/capability analysis
  |
  v
Semantic validation
  |
  v
Canonical semantic model
  |
  +----------------+----------------+
  |                |                |
  v                v                v
Classical       quantum::ir      HDL/hardware
  |                |                |
  +----------------+----------------+
                   |
                   v
                 IR
                   |
                   v
              Optimization
                   |
        +----------+----------+
        |          |          |
        v          v          v
     Routing   Scheduling  Resilience
        |          |          |
        +----------+----------+
                   |
                   v
                  ZQN
                   |
                   v
                  HAL
                   |
                   v
            Target realization

Validation belongs primarily before and around the frontend/semantic boundary, while validating traceability into later stages.

---

68. Validation Must Respect Ownership

The following ownership rules are mandatory:

Concern| Owner
Lexical spelling| lexer/lexical contracts
Syntax| grammar
Source structure| AST
Meaning| semantic analysis
Resource feasibility| resource/capability analysis
Classical canonical representation| canonical IR
Quantum canonical representation| "src/quantum/ir/"
Physical routing| routing
Execution ordering| scheduling
Error correction| QEC
Noise/fault semantics| ZQN
Device abstraction| HAL
Target realization| backend
Grammar correctness| validation

Validation checks each boundary; it does not take ownership away from it.

---

69. Future-Proofing

Validation MUST allow future domains to be added without changing the fundamental validation architecture.

A future domain should be able to provide:

domain specification
domain grammar
lexer additions if required
AST mapping
semantic contract
IR mapping
capabilities
resource requirements
diagnostics
compatibility rules
tests

without creating:

second language
second root grammar
second universal type system
second AST
second canonical IR

unless the architecture explicitly evolves through a documented language-version decision.

---

70. Future Computing Models

The validation model MUST remain neutral toward future computational models.

It must not assume that today's categories:

CPU
GPU
FPGA
ASIC
QPU

are the final set of possible computational targets.

The source-language contract should therefore be expressed in terms of:

semantics
capabilities
resources
constraints
effects
data
computation
communication
correctness

rather than a finite list of machines.

---

71. "Infinity" and Practical Implementation

Zamani's scalability objective is:

tiny → arbitrarily large

subject to actual resources.

"Infinity" is therefore a language-design objective meaning:

«No artificial finite compiler-language ceiling is introduced merely because current hardware or implementation resources are finite.»

It does not require:

- infinite memory;
- infinite execution time;
- infinite parser output;
- infinite physical qubits;
- infinite network nodes.

A finite target may reject or defer a program whose actual requirements exceed available resources.

That is a resource-feasibility result, not a grammar limitation.

---

72. Example: Portable Quantum Validation

This is valid source-level intent:

requires qubits >= n
requires capability("quantum.measurement")

apply H to q
apply custom_gate to q
measure q

Validation should establish:

lexically valid
        ↓
syntactically valid
        ↓
AST represented
        ↓
quantum semantic model represented
        ↓
quantum::ir mapping exists
        ↓
resource requirement represented
        ↓
capability requirement represented

It must not require the grammar to know:

QPU model
physical qubit IDs
coupling graph
calibration
routing
pulse implementation

---

73. Example: Portable Hardware Validation

A program may express parameterized intent:

requires capability("accelerator.compute")
requires memory >= required_memory

The validation layer verifies that these constructs are represented correctly.

It does not determine:

which accelerator
which memory bank
which bus
which physical register
which FPGA slice
which ASIC unit

Those are downstream realization decisions.

---

74. Example: Classical Scaling

A program may express:

let n = input_size();
parallel for i in range(0, n) {
    compute(i);
}

Validation must not require:

n <= 1024
threads <= 64
cores <= 16

unless those numbers are explicitly part of the program's own semantics.

The compiler/runtime may later determine an appropriate execution strategy.

---

75. Example: HDL Scaling

A hardware design may express parameterized dimensions.

Validation must ensure that:

width
depth
count
lanes
channels

are treated as semantic parameters where appropriate rather than hidden universal implementation limits.

A target may later determine whether the design is realizable.

---

76. Example: Distributed Scaling

A distributed computation may describe:

replication
partitioning
communication
consistency
placement
fault tolerance

without the grammar establishing:

MAX_NODES = N

The available cluster determines feasibility.

---

77. Test Matrix

The validation test matrix MUST eventually cover:

Area| Positive| Negative| Boundary| Scale| Determinism
Lexer| ✓| ✓| ✓| ✓| ✓
Parser| ✓| ✓| ✓| ✓| ✓
Expressions| ✓| ✓| ✓| ✓| ✓
Types| ✓| ✓| ✓| ✓| ✓
Declarations| ✓| ✓| ✓| ✓| ✓
Statements| ✓| ✓| ✓| ✓| ✓
Modules| ✓| ✓| ✓| ✓| ✓
Effects| ✓| ✓| ✓| ✓| ✓
Memory| ✓| ✓| ✓| ✓| ✓
Concurrency| ✓| ✓| ✓| ✓| ✓
Classical| ✓| ✓| ✓| ✓| ✓
Quantum| ✓| ✓| ✓| ✓| ✓
Hybrid| ✓| ✓| ✓| ✓| ✓
HDL| ✓| ✓| ✓| ✓| ✓
Hardware| ✓| ✓| ✓| ✓| ✓
Distributed| ✓| ✓| ✓| ✓| ✓
AI| ✓| ✓| ✓| ✓| ✓
Data| ✓| ✓| ✓| ✓| ✓
Networking| ✓| ✓| ✓| ✓| ✓
Security| ✓| ✓| ✓| ✓| ✓
Resources| ✓| ✓| ✓| ✓| ✓
Execution| ✓| ✓| ✓| ✓| ✓
Interoperability| ✓| ✓| ✓| ✓| ✓
Dialects| ✓| ✓| ✓| ✓| ✓
Macros| ✓| ✓| ✓| ✓| ✓
Metaprogramming| ✓| ✓| ✓| ✓| ✓
Compatibility| ✓| ✓| ✓| ✓| ✓

Not every feature needs every column when a category is genuinely inapplicable, but such exclusions MUST be explicit.

---

78. Completion Criteria for This File

"grammar/validation/README.md" is complete when it establishes:

✓ validation purpose
✓ validation authority
✓ validation scope
✓ repository integration
✓ canonical entry-point policy
✓ grammar composition policy
✓ lexical validation
✓ parser validation
✓ reachability validation
✓ ambiguity validation
✓ recursion validation
✓ precedence validation
✓ source-span validation
✓ AST validation
✓ semantic validation
✓ IR validation
✓ quantum::ir validation
✓ HDL validation
✓ hardware validation
✓ resource/capability validation
✓ portability validation
✓ hard-coding validation
✓ scalability validation
✓ deterministic validation
✓ diagnostic validation
✓ compatibility validation
✓ security/safe-Rust policy
✓ positive/negative/boundary/scalability testing
✓ production acceptance gate
✓ downstream ownership boundaries
✓ future-domain extensibility

It must not need to be rewritten merely because an individual domain adds another valid grammar rule.

---

79. Integration Contract With Existing Validation Files

The existing and future validation files MUST relate as follows:

README.md
    |
    +--> grammar-validator.md
    |
    +--> ambiguity.md
    |
    +--> left-recursion.md
    |
    +--> unreachable-rules.md
    |
    +--> duplicate-tokens.md
    |
    +--> keyword-collisions.md
    |
    +--> precedence.md
    |
    +--> source-spans.md
    |
    +--> ast-coverage.md
    |
    +--> semantic-coverage.md
    |
    +--> ir-coverage.md
    |
    +--> portability.md
    |
    +--> hard-coding.md
    |
    +--> scalability.md
    |
    +--> determinism.md

"README.md" defines how they work together.

Each specialized document defines its own detailed contract.

No specialized document should redefine the repository-wide authority hierarchy.

---

80. Final Production Invariant

The ultimate validation invariant is:

One language
      |
      v
One canonical source grammar
      |
      v
One lexical contract
      |
      v
One parser architecture
      |
      v
One domain-neutral AST
      |
      v
One semantic architecture
      |
      +--------------------+
      |                    |
      v                    v
Classical             Quantum semantics
                           |
                           v
                       quantum::ir
      |                    |
      +---------+----------+
                |
                v
          Canonical IR
                |
                v
        Target-independent
           optimization
                |
        +-------+-------+
        |       |       |
        v       v       v
      routing scheduling resilience
        |       |       |
        +-------+-------+
                |
                v
               ZQN
                |
                v
               HAL
                |
                v
       Target realization

The grammar validation subsystem must protect this architecture against drift.

---

81. Core Production Principle

The central rule for "grammar/validation/" is:

«Validate the contract, not merely the parser.»

A production-ready Zamani feature is not:

grammar accepts syntax

It is:

specification
    ↓
lexical contract
    ↓
grammar
    ↓
lexer
    ↓
parser
    ↓
AST
    ↓
semantic validation
    ↓
resource/capability validation
    ↓
canonical IR
    ↓
compiler
    ↓
runtime/backend where applicable
    ↓
tests
    ↓
compatibility
    ↓
deterministic validation
    ↓
scalability audit
    ↓
production acceptance

And the scalability invariant is:

«No artificial machine-derived ceiling may be introduced into the language merely because a current implementation or target has finite resources.»

Zamani therefore remains capable of expressing computation from the smallest useful unit to arbitrarily large realizations, while actual feasibility is determined by semantics, declared requirements, capabilities, compiler/runtime budgets, and resources actually available.

This preserves the intended:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

architecture without turning validation into another source-language authority.