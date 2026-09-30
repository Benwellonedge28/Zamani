Zamani Grammar Validator

Path: "grammar/validation/grammar-validator.md"
Status: Normative production validation specification
Language: Zamani
Grammar technology: ANTLR4-compatible grammar architecture
Implementation baseline: Rust 1.97 / Rust 1.97.1, Rust 2021
Safety: Rust compiler/tooling implementation MUST use safe Rust; "unsafe" MUST NOT be used
Primary objective: Production-grade grammar validation from the smallest supported computation to arbitrarily large computation subject to actual semantic/resource availability
Portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)"
Canonical grammar: "grammar/Zamani.g4"
Canonical executable frontend: "src/lexer.rs", "src/parser.rs", "src/frontend/ast/", "src/semantic.rs"
Canonical IR lowering: "src/ir_gen.rs", "src/ir_verify.rs"
Canonical quantum semantic/IR boundary: "src/quantum/ir/"

---

1. Purpose

This document defines the complete production validation contract for the Zamani grammar subsystem.

It is the orchestration layer connecting the existing specialized validation contracts:

grammar/validation/ambiguity-rules.md
grammar/validation/hardcoding-audit.md
grammar/validation/scalability-rules.md
grammar/validation/semantic-boundaries.md
grammar/validation/compatibility-rules.md
grammar/validation/naming-rules.md

with:

grammar/specification/
grammar/spec/
grammar/lexer/
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
grammar/tests/
grammar/compatibility/

and the executable compiler implementation:

src/lexer.rs
src/parser.rs
src/ast/
src/frontend/ast/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs
src/quantum/

The validator MUST determine whether the grammar is internally correct, repository-consistent, semantically traceable, scalable, deterministic, compatible, and integrated with the compiler architecture.

A grammar MUST NOT be considered production-ready merely because ANTLR can generate a parser from it.

The complete production condition is:

Specification
    ↓
Lexical contract
    ↓
ANTLR grammar
    ↓
Rust lexer
    ↓
Parser
    ↓
AST
    ↓
Structural validation
    ↓
Name/module resolution
    ↓
Type/effect/resource/capability analysis
    ↓
Canonical semantic model
    ↓
IR
    ↓
IR verification
    ↓
Optimization/lowering
    ↓
Routing/scheduling/resilience/QEC/ZQN
    ↓
HAL/backend/runtime
    ↓
Tests

Every stage relevant to a feature MUST be traceable.

---

2. Core Production Invariant

For every stable Zamani feature:

source syntax
    ↕
lexical contract
    ↕
grammar
    ↕
AST
    ↕
semantic contract
    ↕
canonical IR
    ↕
implementation
    ↕
tests

must describe one language.

No layer may silently define a different language.

The validator therefore checks both:

local correctness

and:

cross-layer conformance

---

3. Authority Model

The validator MUST enforce the repository's existing authority model.

Artifact| Authority
"grammar/DESIGN.md"| Normative architecture
"grammar/specification/"| Normative human-readable language specification
"grammar/spec/"| Formal contracts and conformance requirements
"grammar/Zamani.g4"| Canonical ANTLR syntax composition
"src/lexer.rs"| Executable lexical implementation
"src/parser.rs"| Executable reference parser
"src/frontend/ast/"| Domain-neutral source AST
"src/semantic.rs"| Executable semantic analysis
"src/ir_gen.rs"| AST/semantic → IR lowering
"src/ir_verify.rs"| IR structural verification
"src/quantum/ir/"| Canonical quantum semantic/IR boundary
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Historical/proposed/extended design material
"grammar/validation/"| Validation policy
"grammar/tests/"| Grammar conformance tests

The validator MUST NOT promote a feature merely because it appears in "Zamani-Grammar.md".

Likewise, a feature MUST NOT be considered implemented merely because it appears in "grammar/specification/".

---

4. Canonical Validation Pipeline

The production validator MUST conceptually execute the following stages:

                    Repository Snapshot
                           │
                           ▼
                    Authority Discovery
                           │
                           ▼
                    File Inventory
                           │
                           ▼
                  Specification Validation
                           │
                           ▼
                    Grammar Discovery
                           │
                           ▼
                    Grammar Structure
                           │
             ┌─────────────┼──────────────┐
             ▼             ▼              ▼
         Lexical       Syntactic       Naming
         Analysis      Analysis        Analysis
             │             │              │
             └─────────────┼──────────────┘
                           ▼
                     Ambiguity Analysis
                           │
                           ▼
                    Reachability Analysis
                           │
                           ▼
                  Precedence/Associativity
                           │
                           ▼
                     Source-Span Audit
                           │
                           ▼
                    AST Coverage Audit
                           │
                           ▼
                 Semantic Coverage Audit
                           │
                           ▼
                       IR Coverage
                           │
                           ▼
                Cross-Layer Conformance
                           │
             ┌─────────────┼──────────────┐
             ▼             ▼              ▼
       Hard-Coding     Scalability    Portability
          Audit           Audit          Audit
             │             │              │
             └─────────────┼──────────────┘
                           ▼
                    Compatibility
                           │
                           ▼
                     Determinism
                           │
                           ▼
                  Security/Safety Audit
                           │
                           ▼
                     Test Coverage
                           │
                           ▼
                  Production Readiness

A failure in an earlier mandatory stage MUST prevent the repository from being reported as production-ready.

---

5. Validator Responsibilities

The validator owns orchestration of:

1. grammar discovery;
2. grammar composition;
3. grammar syntax validity;
4. lexer/grammar consistency;
5. duplicate token detection;
6. keyword collision detection;
7. grammar-rule reachability;
8. unreachable alternative detection;
9. ambiguity detection;
10. left-recursion policy;
11. precedence validation;
12. associativity validation;
13. parser-progress validation;
14. source-span coverage;
15. AST coverage;
16. semantic coverage;
17. IR coverage;
18. feature-manifest coverage;
19. compatibility coverage;
20. hard-coding audit;
21. scalability audit;
22. portability audit;
23. determinism audit;
24. diagnostics coverage;
25. test coverage;
26. cross-domain integration;
27. production readiness.

It does not own:

- semantic implementation;
- target selection;
- routing;
- scheduling;
- QEC implementation;
- ZQN implementation;
- HAL implementation;
- runtime implementation;
- hardware discovery.

Those are downstream consumers whose integration contracts are verified.

---

6. Non-Responsibilities

The validator MUST NOT:

- modify program semantics;
- select a hardware target;
- select a physical qubit;
- choose a CPU;
- choose a GPU;
- choose an FPGA;
- choose a QPU;
- perform routing;
- perform scheduling;
- perform error correction;
- perform calibration;
- execute arbitrary source code;
- silently rewrite invalid grammar;
- silently rename grammar rules;
- silently introduce compatibility behavior;
- invent AST nodes;
- invent IR nodes;
- treat target capability as parser state.

Validation is analysis, not compilation.

---

7. Validator Safety Requirements

Any Rust implementation of the validator MUST target:

Rust 1.97
Rust 1.97.1
Edition 2021

Production validator code MUST be safe Rust.

The implementation MUST NOT contain:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

The repository SHOULD enforce:

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

where applicable to the validator/compiler crates or modules.

The validator MUST NOT require unsafe Rust merely to process large grammars or source programs.

---

8. Resource-Scalable Validation

The validator MUST be capable of processing:

small grammar
small program
small AST
small IR

through:

very large grammar
very large program
very large AST
very large IR
very large repository

subject to actual available resources.

The validator MUST NOT establish artificial language limits such as:

MAX_RULES
MAX_TOKENS
MAX_ALTERNATIVES
MAX_AST_NODES
MAX_EXPRESSION_DEPTH
MAX_PROGRAM_SIZE
MAX_QUANTUM_OPERATIONS
MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

These names MUST be rejected when they represent universal language limits.

Implementation resource budgets MAY exist.

They MUST be classified separately as:

validator resource limit

rather than:

Zamani language limit

A resource-exhaustion diagnostic MUST be distinguishable from a language-invalid diagnostic.

---

9. Resource Budget Contract

Validation MAY accept explicit implementation budgets such as:

maximum validation work
maximum wall-clock time
maximum resident memory
maximum diagnostic count
maximum expansion work
maximum recursion/worklist depth

but these MUST:

1. be external to language semantics;
2. be configurable;
3. have deterministic failure behavior;
4. never change the interpretation of a valid program;
5. produce an implementation/resource diagnostic;
6. never be encoded as grammar constraints;
7. never be presented as universal Zamani limits.

The validator MUST prefer graceful resource exhaustion over stack overflow, runaway recursion, or uncontrolled memory retention.

---

10. Incremental and Deterministic Validation

The validator SHOULD support incremental validation where practical.

If only:

grammar/quantum/operations.g4

changes, unrelated validation work SHOULD be reusable where dependency information permits.

Incremental validation MUST NOT alter results.

For identical:

repository state
toolchain
validator configuration
language version
dialect configuration
feature configuration

the validator MUST produce identical semantic findings.

Ordering of diagnostics MUST be deterministic.

---

11. Repository Inventory

The validator MUST first construct a repository inventory.

At minimum it MUST discover:

grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/DESIGN.md
grammar/README.md

grammar/specification/**
grammar/spec/**
grammar/lexer/**
grammar/core/**
grammar/types/**
grammar/expressions/**
grammar/statements/**
grammar/declarations/**
grammar/functions/**
grammar/modules/**
grammar/effects/**
grammar/memory/**
grammar/concurrency/**
grammar/classical/**
grammar/quantum/**
grammar/hybrid/**
grammar/hdl/**
grammar/hardware/**
grammar/distributed/**
grammar/ai/**
grammar/data/**
grammar/networking/**
grammar/security/**
grammar/resources/**
grammar/compile/**
grammar/execution/**
grammar/interoperability/**
grammar/dialects/**
grammar/macros/**
grammar/metaprogramming/**
grammar/validation/**
grammar/compatibility/**
grammar/tests/**

and relevant compiler files:

src/lexer.rs
src/parser.rs
src/ast/**
src/frontend/ast/**
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs
src/quantum/**

The validator MUST NOT assume that every conceptual directory exists.

Missing directories are reported only when they are required by an accepted feature contract.

Empty directories MUST NOT be required solely for visual completeness.

---

12. Existing Filename Preservation

The validator MUST recognize the existing authoritative filenames:

grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/DESIGN.md
grammar/README.md

They MUST NOT be renamed merely to satisfy validator organization.

The validator MUST also recognize the repository's existing validation documents rather than requiring them to be renamed.

In particular:

grammar/validation/grammar-validation.md

may coexist with this document if already present in the repository history, but the production architecture MUST avoid two independent orchestration authorities.

This document is the canonical orchestration contract for the validator.

Specialized files remain specialized contracts.

---

13. Specialized Validation Contracts

The validator MUST integrate, not duplicate, the specialized policies.

13.1 Ambiguity

Authority:

grammar/validation/ambiguity-rules.md

Checks:

- lexical ambiguity;
- token-boundary ambiguity;
- syntactic ambiguity;
- precedence;
- associativity;
- declaration/expression ambiguity;
- type/expression ambiguity;
- generic closing ambiguity;
- keyword ambiguity;
- qualified-name ambiguity;
- macro ambiguity;
- dialect ambiguity;
- pattern ambiguity;
- block/literal ambiguity.

---

13.2 Hard-Coding

Authority:

grammar/validation/hardcoding-audit.md

Checks:

- artificial hardware limits;
- fixed resource counts;
- fixed topology;
- fixed qubit counts;
- fixed tensor rank;
- fixed register width;
- fixed node counts;
- fixed thread counts;
- fixed device counts;
- vendor-specific universal syntax;
- backend-specific grammar rules.

The validator MUST distinguish program constants from artificial compiler limits.

---

13.3 Scalability

Authority:

grammar/validation/scalability-rules.md

Checks:

- arbitrary collection sizes;
- arbitrary program size;
- arbitrary nesting within available resources;
- arbitrary resource quantities;
- dynamic resource requirements;
- distributed scale;
- quantum scale;
- tensor scale;
- HDL parameterization;
- macro/metaprogramming scale;
- diagnostics scale;
- parser recovery scale.

---

13.4 Semantic Boundaries

Authority:

grammar/validation/semantic-boundaries.md

Checks that syntax does not absorb responsibilities belonging to:

semantic analysis
resource analysis
capability analysis
IR generation
routing
scheduling
QEC
ZQN
HAL
runtime

---

13.5 Compatibility

Authority:

grammar/validation/compatibility-rules.md

Checks:

- language versions;
- grammar versions;
- token evolution;
- keyword evolution;
- AST evolution;
- IR compatibility;
- dialect compatibility;
- feature gates;
- migration requirements;
- deprecated syntax.

---

13.6 Naming

Authority:

grammar/validation/naming-rules.md

Checks:

- grammar-rule naming;
- token naming;
- AST mapping names;
- feature IDs;
- semantic IDs;
- IR IDs;
- namespace collisions;
- generated artifact names.

---

14. Grammar Composition Validation

"grammar/Zamani.g4" is the canonical ANTLR composition root.

The validator MUST verify that it:

1. exists;
2. declares the expected grammar identity;
3. has one canonical root;
4. reaches the program/source-unit entry point;
5. reaches EOF;
6. imports/delegates to the intended parser/lexer composition;
7. does not create a competing language;
8. does not silently bypass modular grammar contracts;
9. does not contain target-specific semantic logic;
10. does not introduce duplicate domain definitions.

The validator MUST detect multiple competing canonical roots.

For example, the repository MUST NOT silently maintain:

grammar/Zamani.g4
grammar/antlr/Zamani.g4
grammar/another/Zamani.g4

as independent canonical grammars.

---

15. Root Entry-Point Validation

The canonical grammar MUST provide a deterministic complete-source entry point conceptually equivalent to:

program
    → sourceUnit EOF

The exact rule names MAY differ if the normative specification defines them differently.

The validator MUST verify:

root
 ↓
source unit
 ↓
top-level item/declaration/statement dispatch
 ↓
complete consumption
 ↓
EOF

A grammar that accepts a valid prefix while leaving trailing tokens unvalidated MUST fail production validation.

---

16. Grammar Reachability

Every public grammar rule MUST be classified as:

reachable
intentionally standalone
test-only
generated/supporting
deprecated
unimplemented

A rule that is neither reachable nor intentionally standalone MUST be reported.

Unreachable rules MUST NOT silently remain part of the stable language.

The validator MUST distinguish:

unreachable rule

from:

public standalone grammar fragment

because the repository intentionally contains independently testable grammar components.

---

17. Dead Alternatives

The validator MUST detect alternatives that can never be selected because an earlier alternative subsumes them.

Example:

rule
    : identifier
    | identifier "."
    ;

if the surrounding grammar makes the second alternative unreachable.

Dead alternatives MUST be removed or explicitly documented as generated/test-only.

---

18. Left Recursion

ANTLR-compatible grammar recursion MUST be validated.

The validator MUST classify recursion as:

direct left recursion
indirect left recursion
right recursion
mutual recursion
precedence recursion

Left recursion MAY be accepted only where the selected ANTLR strategy and generated parser semantics explicitly support it.

The validator MUST NOT merely suppress warnings.

A recursion pattern that causes:

- nontermination;
- stack exhaustion;
- nondeterministic parsing;
- incorrect precedence

MUST fail validation.

---

19. Expression Precedence

Expression precedence MUST be defined centrally.

The validator MUST verify consistency among:

grammar/expressions/
grammar/spec/syntax.md
src/parser.rs
grammar/validation/ambiguity-rules.md

At minimum, precedence-sensitive constructs MUST cover:

assignment
range
logical OR
logical AND
bitwise OR
bitwise XOR
bitwise AND
equality
comparison
shift
addition/subtraction
multiplication/division/remainder
prefix operators
postfix operators
calls
indexing
member access

The exact precedence order MUST be taken from the normative syntax specification and executable parser.

The validator MUST detect discrepancies rather than inventing precedence.

---

20. Rust Parser Conformance

"src/parser.rs" is the executable reference parser.

The validator MUST compare stable syntax behavior between:

Zamani.g4

and:

src/parser.rs

For every stable construct:

ANTLR acceptance

and:

reference parser acceptance

MUST agree unless a documented compatibility exception exists.

An intentional difference MUST identify:

feature
reason
affected syntax
version
expected behavior
tests
migration

---

21. Lexer Conformance

"src/lexer.rs" is the executable lexical implementation.

The validator MUST compare it with:

grammar/lexer/
grammar/spec/lexical.md
Zamani.g4

Checks MUST include:

- token spelling;
- token identity;
- keyword status;
- identifier behavior;
- operator behavior;
- delimiter behavior;
- literal behavior;
- Unicode behavior;
- comments;
- whitespace;
- source spans;
- malformed token behavior.

---

22. Duplicate Token Identity

The validator MUST detect duplicate lexical concepts.

Known classes requiring audit include:

Question / QuestionMark
Ampersand / BitAnd
Pipe / BitOr

The validator MUST distinguish:

one spelling
one token identity
multiple syntactic meanings

from:

one spelling
multiple competing token identities

The former is generally valid.

The latter requires explicit justification.

---

23. Longest-Match Validation

For overlapping operators, the validator MUST verify deterministic longest-valid matching.

Examples include combinations such as:

=
==
=>
>=
>>
>>=
>>>
::
:
..
..=
...

The actual accepted operator set MUST be derived from the repository's canonical lexical contract.

The validator MUST detect cases where token ordering causes one operator to become unreachable.

---

24. Keyword Collision Validation

Every keyword MUST be checked against:

identifier grammar
existing source identifiers
domain vocabulary
dialect vocabulary
compatibility policy

A new global keyword MUST undergo compatibility analysis.

The validator MUST reject accidental collisions.

Contextual keywords MUST be validated according to their declared syntactic scope.

---

25. Literal Validation

The validator MUST ensure that lexical recognition does not impose artificial machine-size limits.

For example, a decimal literal exceeding "u64" representation MUST NOT be rejected solely because the host implementation uses "u64" internally.

Lexing MUST preserve enough information for later semantic interpretation.

The validator MUST audit:

integer literals
floating-point literals
character literals
string literals
byte literals
boolean literals
array literals
tensor literals
quantum state literals
domain-specific literals

where each is part of the accepted language.

---

26. Quantum Literal Validation

If the accepted language supports quantum state literals such as:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

the validator MUST verify:

lexer
 ↓
parser
 ↓
AST
 ↓
semantic representation
 ↓
quantum::ir

without imposing a fixed state/register size.

The validator MUST NOT require a finite gate enumeration to validate a generic quantum operation.

---

27. Quantum Operation Validation

Quantum operations MUST remain open-ended semantic operations.

The validator MUST reject a grammar architecture that makes the stable language equivalent to:

H
X
Y
Z
CNOT
...

as the complete universe of operations.

The accepted architecture MUST permit semantic operation identifiers such as:

H
custom_gate
vendor.operation
namespace.operation
operation(parameter)

where the normative language specification supports them.

The validator MUST verify:

operation name
namespace
operands
parameters
results
attributes
modifiers
effects
capabilities
source span

can reach the appropriate semantic representation.

Quantum syntax MUST lower toward:

src/quantum/ir/

and MUST NOT create a competing frontend quantum IR.

---

28. Quantum Hardware Independence

The validator MUST reject grammar rules that require a fixed physical quantum device.

Invalid universal grammar concepts include:

Qubit0
Qubit1
Qubit1023

when they define the language's universal quantum model.

Valid program semantics may contain explicit resource quantities or physical mapping metadata where the language specification explicitly permits target-bound code.

The validator MUST distinguish:

logical resource requirement

from:

physical realization

---

29. HDL Validation

HDL grammar MUST be validated for parameterization.

The validator MUST reject universal language restrictions such as:

wire [31:0]

when the width is being imposed as a universal language maximum.

The validator MUST permit parameterized intent where specified:

width
depth
lanes
ports
channels
pipeline stages
memory dimensions

without requiring fixed hardware capacity.

HDL syntax MUST remain target-independent unless explicitly placed in a target-bound context.

---

30. Hardware Validation

Hardware grammar MUST express intent rather than today's device inventory.

The validator MUST distinguish:

requires capability("gpu.compute")
requires capability("quantum.measurement")
requires memory >= required_memory
requires topology(...)

from:

use_gpu_0
use_qpu_7
use_core_31
use_node_1024

The first category is portable semantic intent.

The second category is realization-specific.

Target realization belongs downstream.

---

31. Resource Validation

The validator MUST recognize the distinction among:

requirement
constraint
capability
preference
hint
realization

These MUST NOT collapse into one grammar category.

Example:

requires qubits >= n

is a resource requirement.

requires capability("quantum.measurement")

is a capability requirement.

prefer accelerator("quantum")

is a preference.

map logical_qubit -> physical_qubit

is realization.

The validator MUST ensure that realization does not become mandatory source semantics merely because a particular backend exists.

---

32. AST Coverage

Every accepted stable grammar construct MUST have an AST representation.

The validator MUST build or consume a traceability map:

grammar rule
    ↓
AST node

No stable syntax may terminate at:

parse tree only

unless the feature is explicitly syntax-only and non-semantic.

The AST MUST remain domain-neutral at the frontend boundary.

Hardware topology, calibration, routing decisions, QEC schedules, and physical target selection MUST NOT be embedded into the generic source AST merely because the syntax mentions a requirement.

---

33. AST Source Span Coverage

Every source-originating AST construct MUST preserve source provenance according to the repository's source-map/span architecture.

The validator MUST verify that relevant nodes preserve:

source identity
start position
end position

according to the canonical source-span representation.

Byte offsets are the authoritative machine-readable coordinates where the repository's source-location architecture defines them as such.

Line and column information MUST remain diagnostic metadata rather than the sole source identity.

Generated nodes MAY have no direct source span where explicitly permitted.

---

34. Semantic Coverage

For every stable AST construct, the validator MUST identify its semantic owner.

The mapping is conceptually:

AST
 ↓
semantic rule

Examples:

type syntax
    → type analysis

effect syntax
    → effect analysis

resource syntax
    → resource analysis

capability syntax
    → capability analysis

quantum operation syntax
    → quantum semantic analysis

HDL syntax
    → hardware-intent semantics

The grammar validator MUST NOT attempt to implement these semantic rules itself.

---

35. IR Coverage

Every executable semantic construct MUST have an identified IR destination.

The validator MUST establish:

AST construct
    ↓
semantic construct
    ↓
IR construct

If no IR destination exists, the feature MUST NOT be marked production-implemented.

A syntax feature with:

grammar ✓
lexer ✓
parser ✓
AST ✓
semantic ✓
IR ✗

is:

PARTIALLY IMPLEMENTED

not:

IMPLEMENTED

---

36. Canonical Quantum IR

Quantum syntax MUST integrate with:

src/quantum/ir/

as the canonical quantum semantic/IR boundary.

The validator MUST detect duplicate representations that unnecessarily introduce:

frontend quantum IR

alongside:

quantum::ir

when both attempt to own the same semantic responsibility.

Routing, scheduling, QEC, resilience, ZQN, and HAL MUST consume or transform canonical semantic representations downstream.

They MUST NOT redefine source grammar semantics.

---

37. IR Verification Integration

The validator MUST integrate with the invariants enforced by:

src/ir_verify.rs

A grammar feature cannot be considered fully integrated if its lowering produces IR that violates the existing verifier.

The validation chain is:

source
 ↓
parse
 ↓
AST
 ↓
semantic analysis
 ↓
IR generation
 ↓
IR verification

The validator MUST distinguish:

grammar invalid

from:

valid grammar but invalid lowering

and:

valid lowering but invalid IR

---

38. Canonical IR Scalability

The validator MUST inspect IR integration for artificial capacity limits.

IR types such as:

qubit identifiers
tensor dimensions
register widths
resource quantities
instruction counts
operation identifiers

MUST NOT acquire grammar-level artificial limits.

A downstream IR resource limit may exist as an implementation/resource constraint, but it MUST NOT silently redefine the Zamani language.

---

39. Semantic Boundary Validation

The validator MUST enforce:

Grammar
  ↓
AST
  ↓
Semantics
  ↓
IR

and prevent:

Grammar
  ↓
Routing

or:

Grammar
  ↓
QEC

or:

Grammar
  ↓
HAL

or:

Grammar
  ↓
Calibration

or:

Grammar
  ↓
Physical device

as universal language dependencies.

---

40. Feature Manifest Validation

Where machine-readable feature manifests exist, the validator MUST use them as traceability contracts.

A feature manifest SHOULD identify:

id
name
status
version
specification
tokens
grammar_rules
ast_nodes
semantic_rules
ir_mapping
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
hard_coding_policy

A manifest is not a second language authority.

The validator MUST compare manifest claims against actual repository state.

For example:

manifest says AST node exists

but:

AST node does not exist

MUST produce a validation error.

Likewise:

manifest says stable

while:

IR mapping missing

MUST prevent stable status.

---

41. Feature Status

The validator MUST support explicit status values:

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
PARTIALLY_IMPLEMENTED

A feature MUST NOT be promoted automatically.

The validator MUST calculate the highest justified implementation status from evidence.

Example:

SPECIFIED
✓

LEXER_IMPLEMENTED
✓

PARSER_IMPLEMENTED
✓

AST_IMPLEMENTED
✓

SEMANTIC_IMPLEMENTED
✗

IR_IMPLEMENTED
✗

Result:

PARTIALLY_IMPLEMENTED

not:

STABLE

---

42. "grammar.md" Conformance

The validator MUST compare implementation findings with:

grammar/grammar.md

"grammar.md" is the implementation-conformance reference.

It MUST NOT claim:

IMPLEMENTED

when the executable pipeline is incomplete.

The validator MUST detect:

grammar.md says implemented
actual parser says unsupported

and:

grammar.md says unsupported
actual stable parser accepts it

Both are conformance failures.

---

43. "Zamani-Grammar.md" Validation

"Zamani-Grammar.md" is historical/design/extended material.

The validator MUST NOT treat its contents as automatically accepted syntax.

Features found there MUST be classified as:

stable
proposed
experimental
deprecated
historical
not implemented

where appropriate.

A feature may be promoted only through:

design
 ↓
normative specification
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
semantic implementation
 ↓
IR
 ↓
compiler/runtime
 ↓
tests
 ↓
stable

---

44. Grammar-to-Specification Conformance

For every stable grammar production, the validator MUST identify its specification authority.

A grammar rule without a specification owner MUST be reported.

A specification rule without grammar implementation MUST be classified:

SPECIFIED / NOT IMPLEMENTED

rather than silently considered implemented.

---

45. Grammar-to-Test Conformance

Every stable syntax feature MUST have tests appropriate to its semantics.

Minimum required categories:

positive
negative
boundary
scalability
determinism
compatibility

Domain-specific features SHOULD additionally have:

cross-domain
resource
capability
source-span
diagnostic
IR-lowering

tests as appropriate.

---

46. Positive Tests

Positive tests MUST establish that valid source is accepted.

Examples should cover:

minimal form
normal form
nested form
generic form
qualified form
parameterized form
cross-domain form

Positive tests MUST NOT accidentally encode machine-size limits.

---

47. Negative Tests

Negative tests MUST establish rejection of invalid source.

They MUST distinguish:

lexical error
syntax error
semantic error
resource error
capability error
compatibility error
unsupported-feature error

A source program that is syntactically valid but cannot execute on a target due to insufficient resources MUST NOT be reported as a grammar error.

---

48. Boundary Tests

Boundary tests MUST examine:

empty constructs
single-item constructs
deep nesting
large literals
large identifiers
large argument lists
large collections
large operation sequences
large generic structures
Unicode boundaries
source-file boundaries
module boundaries
dialect boundaries

The purpose is to discover implementation failures without turning those boundaries into language ceilings.

---

49. Scalability Tests

Scalability tests MUST validate that the same semantic construct can grow with available resources.

Examples include:

one element
many elements
parameterized element count
very large element count

and:

one qubit
many qubits
symbolic qubit requirement

and:

one node
many nodes
distributed topology

and:

one tensor dimension
large tensor dimensions
symbolic shape

The test suite MUST NOT define a final maximum as the language limit.

---

50. Determinism Tests

For fixed:

source
language version
dialects
feature gates
macro environment
validator configuration
repository state

validation MUST be deterministic.

Repeated validation MUST produce the same:

accept/reject result
diagnostic identity
diagnostic severity
source span
feature status
dependency classification

and stable ordering.

Hash-map iteration order MUST NOT determine diagnostics or semantic decisions.

---

51. Parser Recovery Validation

Invalid source MUST NOT cause parser recovery to loop indefinitely.

The validator MUST test that recovery:

1. detects an error;
2. emits a diagnostic;
3. advances or otherwise makes provable progress;
4. reaches a synchronization point;
5. eventually terminates.

The validator MUST detect recovery loops.

A parser that repeatedly processes the same token without progress MUST fail.

---

52. Deep Input Validation

Deeply nested source MUST NOT rely unnecessarily on native call-stack depth.

Where practical, validation infrastructure SHOULD use:

explicit stacks
worklists
iterative traversal
bounded diagnostic queues
streaming processing
incremental analysis

Implementation limits MAY exist but MUST be resource diagnostics rather than language semantics.

---

53. Diagnostic Validation

Every validator diagnostic MUST have:

stable diagnostic code
severity
message
primary source span when applicable
related spans when applicable
feature/rule identity when applicable
category

Suggested categories:

LEXICAL
GRAMMAR
AMBIGUITY
REACHABILITY
PRECEDENCE
NAMING
AST
SEMANTIC
IR
PORTABILITY
SCALABILITY
HARDCODING
COMPATIBILITY
DETERMINISM
SECURITY
RESOURCE
IMPLEMENTATION
INTEGRATION

Diagnostics MUST distinguish errors from warnings and informational findings.

---

54. Diagnostic Stability

Diagnostic wording MAY evolve.

Diagnostic identity SHOULD remain stable through a compatibility period.

Therefore a diagnostic SHOULD have a stable machine-readable code such as:

ZGV001

or another repository-approved namespace.

The exact code allocation MUST be centralized.

Individual grammar files MUST NOT independently invent conflicting diagnostic codes.

---

55. Hard-Coding Audit

The validator MUST scan grammar and relevant implementation metadata for artificial limits.

The audit MUST include patterns corresponding to:

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

and equivalent naming patterns.

The audit MUST also inspect semantic forms such as:

if qubit_count > 1024

when "1024" is being used as a universal language ceiling.

It MUST NOT flag ordinary program semantics merely because they contain numeric constants.

---

56. Downstream Resource Limits

The repository currently contains downstream concepts such as resource limits and maximum-resource APIs.

The validator MUST NOT mechanically classify every occurrence of:

max_qubits
max_memory
max_nodes

as an architectural violation.

Instead, it MUST determine ownership.

Valid:

device capability
runtime resource budget
benchmark configuration
backend feasibility limit
deployment policy
allocator policy

Invalid:

language grammar ceiling
universal parser ceiling
universal AST ceiling
universal quantum language ceiling

The validator MUST therefore report:

HARDCODING_REVIEW_REQUIRED

when ownership cannot be established automatically, rather than blindly declaring the repository invalid.

---

57. POCO-REAF Validation

The validator MUST explicitly test the portability principle.

For a portable source program:

parse(source)

MUST NOT depend on:

CPU availability
GPU availability
FPGA availability
QPU availability
number of qubits
number of cores
number of nodes
memory capacity
network topology
runtime queue state
calibration state
current device

unless the source explicitly enters a target-bound construct.

The same source MUST retain the same source-level semantics when compiled for:

tiny target
CPU
multicore CPU
GPU
FPGA
ASIC
QPU
quantum simulator
accelerator
HPC
cluster
distributed system
cloud
future target

provided the target can satisfy the program's declared semantic/resource requirements.

---

58. Resource Feasibility Versus Language Validity

The validator MUST preserve the distinction:

lexically valid
≠
syntactically valid
≠
semantically valid
≠
resource feasible
≠
target compatible
≠
runtime available

For example:

requires qubits >= n

may be valid source semantics even when a particular QPU has fewer physical qubits.

That situation is NOT a grammar error.

It is a downstream resource/target feasibility issue.

---

59. Capability Validation

Capability expressions MUST be validated as semantic requirements.

Examples:

requires capability("tensor.compute")
requires capability("gpu.compute")
requires capability("quantum.measurement")
requires capability("quantum.mid_circuit_measurement")

The validator MUST ensure that capability identifiers remain data-driven and extensible.

A new hardware vendor MUST NOT require a grammar fork merely to introduce a capability.

---

60. Classical Domain Validation

Classical constructs MUST be checked for:

lexical coverage
syntax coverage
type coverage
AST coverage
semantic coverage
IR coverage
scalability

Mathematical algorithms SHOULD NOT be required to become individual parser keywords merely because they exist as library/intrinsic operations.

The validator MUST permit extensible operation models where specified.

---

61. AI/Data Validation

AI and data grammar MUST NOT become framework-specific language syntax.

The validator MUST reject unnecessary coupling such as requiring:

framework-specific model name
vendor-specific accelerator
framework-specific tensor primitive

as core language syntax.

AI/data constructs MUST integrate through:

types
expressions
operations
capabilities
resources
semantic models
IR

according to their respective specifications.

---

62. Distributed Validation

Distributed syntax MUST NOT hard-code:

MAX_NODES
node_0
node_1
...

as the universal model.

The validator MUST verify that topology and placement are expressed as requirements, constraints, capabilities, preferences, or downstream realization as appropriate.

---

63. Networking Validation

Networking syntax MUST remain independent of a specific deployment.

The validator MUST distinguish:

endpoint
protocol
capability
routing intent
service identity

from:

specific machine address
specific physical link
specific network topology

where the latter belongs to deployment or target-bound configuration.

---

64. Security Validation

Security syntax MUST be validated for:

deterministic parsing
capability boundaries
identity semantics
authorization semantics
secret handling
cryptographic operation representation
provenance

The grammar MUST NOT claim that parsing a security keyword itself provides a security guarantee.

Security guarantees belong to semantic/compiler/runtime enforcement.

---

65. Macro Validation

Macro validation MUST verify:

deterministic expansion
hygiene
source provenance
expansion ordering
recursive expansion behavior
diagnostic preservation
semantic validation after expansion

Macro implementation limits MUST NOT become language-level semantic limits.

Macro expansion MUST NOT bypass AST/semantic validation.

---

66. Metaprogramming Validation

Metaprogramming MUST NOT become an unrestricted hidden compiler escape hatch.

The validator MUST identify:

compile-time execution boundary
generated syntax
generated AST
generated semantic entities
generated source spans/provenance
capability requirements
resource requirements

Generated constructs MUST pass the same appropriate validation stages as ordinary constructs.

---

67. Dialect Validation

Every dialect MUST identify:

name
version
owner
syntax extensions
semantic extensions
AST mapping
IR mapping
compatibility policy
feature gates

A dialect MUST NOT silently alter core Zamani syntax.

Dialect selection MUST be deterministic.

The same source plus different explicitly selected dialect configuration MAY produce different accepted syntax; hidden environment-dependent dialect selection MUST NOT be permitted.

---

68. Version Validation

The validator MUST validate:

language version
grammar version
dialect version
feature version
AST compatibility
IR compatibility

Breaking grammar changes MUST have compatibility metadata.

Adding a keyword MUST trigger identifier-collision validation.

Removing syntax MUST trigger migration/deprecation validation.

Changing precedence MUST trigger compatibility analysis.

---

69. Compatibility With Existing Programs

A grammar change MUST identify whether it is:

backward compatible
source compatible
AST compatible
semantic compatible
IR compatible
dialect compatible
breaking

These categories MUST NOT be collapsed into one generic "compatible" status.

---

70. Naming Validation

Grammar rule names MUST be:

unique within their grammar namespace
stable
descriptive
consistent

Token names MUST NOT collide.

AST names MUST NOT be generated from ambiguous grammar names without explicit mapping.

Domain names MUST NOT redefine universal names.

Qualified names MUST be canonical.

---

71. Cross-Domain Validation

The validator MUST test combinations of domains where the language specification permits them.

Minimum integration classes include:

classical + quantum
classical + HDL
classical + distributed
quantum + classical
quantum + hardware
quantum + resources
HDL + hardware
AI + tensor/data
AI + accelerator
distributed + networking
security + networking
compile + hardware
execution + resources

Cross-domain syntax MUST remain deterministic.

---

72. Hybrid Quantum-Classical Validation

Hybrid constructs MUST preserve:

classical semantics
quantum semantics
measurement semantics
feed-forward semantics
resource requirements
capabilities

The grammar validator MUST ensure that a hybrid construct does not require a target-specific QPU realization at parse time.

---

73. HDL/Software Co-Design Validation

A co-design construct MAY describe:

computation
hardware intent
memory behavior
communication
timing constraints
verification properties

but the validator MUST ensure that:

source intent

is distinct from:

physical implementation

The same source-level computation SHOULD remain expressible across different hardware implementations.

---

74. Error-Correction Validation

Quantum error-correction syntax MUST be validated only as language-level intent.

The validator MUST NOT require grammar knowledge of:

physical calibration
pulse implementation
routing algorithm
specific QEC decoder
device topology

unless explicitly defined as target-bound syntax.

The semantic/QEC implementation remains downstream.

---

75. Resilience-State Validation

Where the language models resilience state, the validator MUST preserve the defined semantic states:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

These are semantic values, not parser-side hardware assumptions.

The validator MUST ensure they do not accidentally become fixed hardware enumerations elsewhere.

---

76. Outcome Validation

Where execution/resilience semantics define outcomes such as:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

the validator MUST verify:

syntax
AST
semantic representation
IR/runtime integration

and MUST ensure that the grammar does not itself implement recovery behavior.

---

77. Source-Span Validation Across Layers

The validator MUST verify source provenance through:

lexer
 ↓
parser
 ↓
AST
 ↓
semantic diagnostics
 ↓
IR metadata where supported

A diagnostic generated downstream SHOULD be able to identify the source construct that caused it.

Generated transformations MAY introduce derived provenance.

Loss of source identity MUST be reported where it prevents required diagnostics or conformance.

---

78. Repository Consistency Matrix

The validator MUST conceptually maintain a matrix:

Layer| Required evidence
Specification| normative definition
Lexer| token implementation
Grammar| parser production
Parser| executable parsing
AST| representation
Semantic| meaning
IR| lowering
IR verification| invariants
Backend/runtime| execution consumer
Tests| conformance
Compatibility| version policy
Documentation| user-visible contract

A stable feature with a missing mandatory cell MUST NOT be reported as fully implemented.

---

79. Feature Closure Rule

A feature is independently complete only when its complete contract is known before dependent files are modified.

For each feature, validation MUST establish:

Purpose
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Syntax
Tokens
AST
Semantics
IR
Diagnostics
Compatibility
Security
Performance
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Determinism Tests
Hard-Coding Audit
Completion Criteria

This is the repository's feature closure rule.

It exists specifically to prevent:

file A completed
file B later changes A
file A must be reopened

from becoming the normal development process.

---

80. Independent-First Validation

Validation MUST permit foundational files to be completed independently.

The dependency order is:

authority
    ↓
lexical contracts
    ↓
core syntax
    ↓
types
    ↓
expressions
    ↓
declarations
    ↓
statements
    ↓
functions/modules
    ↓
effects/memory/concurrency
    ↓
resources/capabilities
    ↓
domains
    ↓
advanced facilities
    ↓
root composition
    ↓
implementation conformance

A dependent file MUST NOT be declared complete when its upstream contract is intentionally unresolved.

---

81. Circular Dependency Detection

The validator MUST construct a dependency graph.

It MUST detect cycles such as:

A depends on B
B depends on C
C depends on A

where the cycle prevents independent completion.

Some semantic recursion is valid.

The validator MUST distinguish:

legitimate language recursion

from:

development-contract dependency cycle

---

82. Grammar Fragment Standalone Validation

Grammar fragments MAY be independently compiled where they are intentionally designed as standalone/testable components.

A standalone fragment MUST declare:

entry rule
required tokens
required imports
adapter rules
intended consumers

It MUST NOT falsely claim to be a complete Zamani language.

This permits independent development without creating competing root grammars.

---

83. Generated Grammar Validation

Generated grammar artifacts MUST identify:

source authority
generator
generation version
generation inputs
generation timestamp policy

Generated files MUST NOT become manually edited authorities unless explicitly specified.

If generated output is committed, the validator MUST be able to reproduce it or explain why reproducibility is intentionally unavailable.

---

84. ANTLR Validation

The validator MUST verify:

ANTLR grammar syntax
imports
tokens
channels
modes
parser rules
lexer rules
rule references
generated parser construction
generated lexer construction

ANTLR warnings MUST be classified.

Warnings affecting correctness MUST fail production validation.

Informational warnings MAY be accepted only when documented.

---

85. Ambiguity Severity

Ambiguity findings MUST be classified as:

ERROR
WARNING
INFO

An ambiguity affecting stable syntax MUST be an error.

A deliberately deferred semantic overload MAY be allowed if:

1. syntax is deterministic;
2. semantic ownership is explicit;
3. overload resolution is deterministic;
4. diagnostics exist for unresolved ambiguity.

---

86. Semantic Overload Versus Grammar Ambiguity

The validator MUST permit semantic overload where syntax is deterministic.

Example:

compute(x)

may resolve to multiple semantic candidates.

That is not necessarily grammar ambiguity.

However:

compute(x)

MUST produce one parse tree.

If semantic resolution leaves multiple valid candidates, semantic analysis MUST report an ambiguity diagnostic.

---

87. Environment Independence

Validation MUST NOT change because of:

current CPU
current GPU
current QPU
current operating system
current network
current filesystem contents
current wall-clock time
randomness
device calibration
runtime queue
provider availability

unless explicitly supplied as declared validation configuration.

This is required for reproducibility and POCO-REAF.

---

88. Filesystem Independence

Parsing MUST NOT depend on whether a referenced module physically exists.

The parser determines syntax.

Module resolution determines whether the referenced module exists.

Therefore:

module path syntax

MUST be validated independently from:

filesystem availability

This distinction is required for reproducible and cross-platform builds.

---

89. Network Independence

The validator MUST NOT require live network access to determine basic syntax validity.

External dependencies MAY be checked by separate dependency-resolution tooling.

Grammar validation itself MUST remain deterministic and reproducible.

---

90. Hardware Independence

The validator MUST NOT query actual hardware to determine whether grammar is valid.

For example:

requires capability("gpu.compute")

is syntactically and semantically valid independent of whether a GPU is currently installed.

Hardware feasibility is downstream.

---

91. Security Boundary

The validator MUST treat grammar files, feature manifests, source files, and generated parser artifacts as untrusted input.

It MUST avoid:

arbitrary command execution
unsafe deserialization
unbounded recursive expansion
unbounded memory retention
implicit network execution
implicit filesystem mutation

Validation MUST be analysis-only.

---

92. Macro and Metaprogramming Security

If validation requires macro expansion or compile-time evaluation, the execution model MUST be explicitly sandboxed and capability-controlled.

The validator MUST NOT execute arbitrary user code merely to determine grammar validity unless the language specification explicitly requires such execution and a secure execution architecture exists.

---

93. Diagnostic Limits

The validator MAY cap the number of emitted diagnostics for resource protection.

However:

diagnostic_count_limit

MUST be an implementation configuration, not a language limit.

When the cap is reached, the validator MUST emit one deterministic summary indicating that additional diagnostics were suppressed.

---

94. Error Precedence

When multiple validation errors exist, the validator SHOULD report foundational errors before dependent errors.

Preferred ordering:

repository/authority
lexical
grammar syntax
reachability
ambiguity
precedence
AST
semantic
IR
integration
compatibility
scalability
hard-coding
tests

The exact ordering MUST be deterministic.

A downstream error that is merely a consequence of an earlier fatal error MAY be suppressed and marked as dependent.

---

95. Validation Finding Identity

Every finding SHOULD identify:

validator rule
file
line/range when available
grammar rule/token/feature
severity
diagnostic code

Example conceptual identity:

ZGV-HARDCODE-001
grammar/quantum/...
universal resource ceiling detected

The exact code registry MUST be centralized.

---

96. Suppressions

Suppression MAY exist only through explicit, reviewable mechanisms.

A suppression MUST identify:

finding
reason
owner
scope
expiry/review policy

Global suppression of an entire validation class MUST NOT be permitted as a normal development mechanism.

For example, disabling all hard-coding validation merely because one downstream resource module has a legitimate implementation budget is prohibited.

---

97. Validation Baselines

Existing known findings MAY be recorded in a baseline.

A baseline MUST contain:

finding identity
file
reason
status
owner
expected resolution

New findings MUST still fail when they are outside the accepted baseline.

Baselines MUST NOT turn known architectural violations into permanent accepted language behavior.

---

98. Production Gate

The validator MUST expose an overall production status equivalent to:

PRODUCTION_READY

only when all mandatory gates pass.

Possible non-ready statuses:

INVALID
INCOMPLETE
PARTIALLY_IMPLEMENTED
EXPERIMENTAL
BLOCKED
RESOURCE_LIMITED

"RESOURCE_LIMITED" MUST NOT mean the language itself is invalid.

---

99. Production Gate — Authority

Required:

[ ] One normative language authority
[ ] One canonical ANTLR composition root
[ ] No competing root grammar
[ ] Zamani.g4 identified as canonical grammar
[ ] grammar.md identified as implementation reference
[ ] Zamani-Grammar.md identified as historical/design material
[ ] Specification/grammar relationships documented

---

100. Production Gate — Lexical Layer

Required:

[ ] Canonical token inventory
[ ] No unexplained duplicate token identity
[ ] Keyword registry consistent
[ ] Identifier rules consistent
[ ] Operator rules consistent
[ ] Longest-match behavior verified
[ ] Literal behavior verified
[ ] Unicode behavior verified
[ ] Quantum literals verified
[ ] Source spans preserved
[ ] Malformed tokens produce deterministic diagnostics

---

101. Production Gate — Grammar

Required:

[ ] Root grammar parses
[ ] Root reaches EOF
[ ] Imports resolve
[ ] No unreachable stable rules
[ ] No dead alternatives
[ ] No unexplained ambiguity
[ ] Recursion validated
[ ] Precedence validated
[ ] Associativity validated
[ ] Parser recovery progresses
[ ] Standalone fragments classified

---

102. Production Gate — AST

Required:

[ ] Every stable construct maps to AST
[ ] AST is domain-neutral where required
[ ] Source spans preserved
[ ] No physical target realization embedded in generic AST
[ ] Quantum syntax maps toward canonical quantum semantics
[ ] No duplicate frontend quantum IR

---

103. Production Gate — Semantics

Required:

[ ] Type rules identified
[ ] Effect rules identified
[ ] Resource rules identified
[ ] Capability rules identified
[ ] Ownership/linearity rules identified
[ ] Domain semantics identified
[ ] Semantic ambiguity handled
[ ] Target feasibility separated from source validity

---

104. Production Gate — IR

Required:

[ ] Every executable feature has an IR destination
[ ] AST → semantic → IR mapping exists
[ ] IR verifier accepts valid lowering
[ ] Invalid lowering is rejected
[ ] Quantum lowering reaches quantum::ir
[ ] No duplicate quantum IR authority
[ ] No artificial grammar-level IR capacity limits

---

105. Production Gate — Portability

Required:

[ ] No universal hardware ceilings
[ ] No fixed resource counts
[ ] No fixed topology
[ ] Requirements separated from realization
[ ] Capabilities separated from realization
[ ] Preferences separated from requirements
[ ] Target selection downstream
[ ] Hardware state not required for parsing

---

106. Production Gate — Safety

Required:

[ ] Rust 1.97 compatibility
[ ] Rust 1.97.1 compatibility
[ ] Rust 2021
[ ] No unsafe Rust
[ ] No hidden unsafe dependency in validator logic
[ ] Deterministic validation
[ ] Bounded resource behavior
[ ] No arbitrary code execution during ordinary grammar validation

---

107. Production Gate — Testing

Required:

[ ] Positive tests
[ ] Negative tests
[ ] Boundary tests
[ ] Scalability tests
[ ] Determinism tests
[ ] Compatibility tests
[ ] Cross-domain tests
[ ] Source-span tests
[ ] Diagnostic tests
[ ] AST coverage tests
[ ] Semantic coverage tests
[ ] IR coverage tests

---

108. Production Gate — Documentation

Required:

[ ] Specification linked
[ ] Grammar linked
[ ] Lexer linked
[ ] Parser linked
[ ] AST linked
[ ] Semantic owner linked
[ ] IR owner linked
[ ] Tests linked
[ ] Compatibility policy linked
[ ] Downstream consumers identified

---

109. Production Gate — Feature Closure

For every stable feature:

[ ] Purpose
[ ] Owns
[ ] Does not own
[ ] Syntax
[ ] Tokens
[ ] AST
[ ] Semantics
[ ] IR
[ ] Diagnostics
[ ] Resource semantics
[ ] Capability semantics
[ ] Tests
[ ] Compatibility
[ ] Scalability
[ ] Determinism
[ ] Hard-coding audit
[ ] Security review
[ ] Performance review

must be complete.

---

110. Validator Output Contract

The validator SHOULD produce machine-readable output containing at least:

repository
revision
validator_version
language_version
grammar_version
status
findings
features
coverage
dependencies
resource_budget
duration

Each finding SHOULD contain:

code
severity
category
path
source_location
rule
feature
message
related_files
related_rules

Human-readable output MAY additionally provide summaries.

---

111. Machine-Readable Result Example

Conceptually:

validation:
    status: PRODUCTION_READY

    authority:
        passed: true

    lexical:
        passed: true

    grammar:
        passed: true

    ambiguity:
        passed: true

    ast:
        passed: true

    semantic:
        passed: true

    ir:
        passed: true

    portability:
        passed: true

    scalability:
        passed: true

    hard_coding:
        passed: true

    compatibility:
        passed: true

    determinism:
        passed: true

    tests:
        passed: true

This is an output model, not an additional language specification.

---

112. Validation of Existing Downstream Quantum Limits

The repository contains downstream quantum/resource-limit concepts.

The validator MUST inspect their ownership.

A downstream object such as:

device limits
memory limits
benchmark limits
runtime budgets
backend feasibility limits

MAY legitimately contain a maximum.

Such a limit MUST be classified as:

target/runtime/resource policy

and MUST NOT be promoted into:

grammar limit
language limit
AST limit
universal quantum limit

This distinction is mandatory for POCO-REAF.

---

113. Validation of Benchmark Limits

Benchmarking infrastructure MAY deliberately constrain workloads.

For example, a benchmark may choose to test a finite number of qubits.

That does NOT establish:

maximum Zamani qubits

The validator MUST therefore classify benchmark constants separately from language constants.

---

114. Validation of Runtime Budgets

Runtime pools MAY be bounded.

A runtime pool may intentionally enforce:

allocation budget
queue budget
memory budget
execution budget

The validator MUST not mistake such runtime policy for grammar semantics.

The distinction MUST be documented in the affected downstream module.

---

115. Validation of Compiler Budgets

Compilers MAY impose practical implementation budgets.

Examples:

maximum optimizer work
maximum compilation time
maximum diagnostic memory
maximum temporary storage

These MUST be:

implementation constraints

not:

language constraints

The validator MUST verify that source semantics do not change because such a budget is reached.

---

116. Validation of Deep Recursion

The validator SHOULD identify recursive implementations that can fail on deeply nested valid programs.

Where a grammar construct is theoretically unbounded, the implementation SHOULD use iterative techniques where practical.

A stack-exhaustion issue MUST be reported as an implementation scalability defect, not converted into a grammar maximum.

---

117. Validation of Large Identifiers

The validator MUST test identifiers larger than typical machine-word assumptions.

The grammar MUST NOT require identifier length to fit a fixed integer type as a semantic rule.

Implementation limits MAY exist as resource policies and must be diagnosed separately.

---

118. Validation of Large Numeric Literals

The validator MUST test numeric literals whose textual representation exceeds common primitive integer widths.

Lexing MUST preserve literal information.

Semantic conversion MUST determine representability.

The lexer MUST NOT silently truncate.

The validator MUST detect truncation.

---

119. Validation of Generic Nesting

Nested generic types MUST be tested beyond shallow examples.

Examples:

Vector<T>
Vector<Vector<T>>
Map<String, Vector<T>>
Map<String, Map<String, Vector<T>>>

The validator MUST ensure that nesting behavior is not accidentally limited by grammar construction.

---

120. Validation of Expression Nesting

Expression nesting MUST be tested.

Examples:

f(x)
f(g(x))
f(g(h(x)))

and more deeply nested equivalent forms.

Implementation recursion limits MUST NOT silently become language syntax limits.

---

121. Validation of Collection Size

Collection syntax MUST be validated independently of collection implementation capacity.

Examples:

[]
[x]
[x, y]
large collections

The grammar MUST not establish a maximum number of elements.

---

122. Validation of Tensor Shape

Tensor grammar MUST permit parameterized and semantically defined shapes.

The validator MUST reject grammar-level assumptions such as:

rank <= N
dimension <= N

unless such a bound is explicitly part of the language semantics rather than an implementation artifact.

---

123. Validation of Register Width

HDL/classical register syntax MUST not universally establish:

32-bit
64-bit
128-bit

as a language ceiling.

Widths may be explicit program semantics.

The validator MUST distinguish:

program-declared width

from:

compiler-supported maximum width

---

124. Validation of Distributed Scale

Distributed grammar MUST not impose:

node count maximum
endpoint maximum
process maximum
actor maximum
channel maximum

as universal language semantics.

Resource feasibility belongs downstream.

---

125. Validation of Timeline Scale

If MTS/timeline features are implemented, the validator MUST not impose a universal maximum number of:

timelines
branches
forks
merges
observations

Any runtime budget MUST remain an implementation/resource policy.

---

126. Validation of Nano Scale

Nano-oriented grammar MUST remain semantic and compositional.

The validator MUST not turn a finite physical implementation inventory into universal syntax.

Future physical domains MUST remain extensible through semantic models rather than requiring parser redesign for every new device or material.

---

127. Validation of Future Extensibility

The validator MUST ask:

Can a new operation be introduced without changing the core parser?
Can a new hardware capability be introduced without changing the core grammar?
Can a new accelerator be introduced without adding a keyword?
Can a new quantum operation be introduced without adding a grammar alternative?
Can a new vendor backend be introduced without changing source semantics?
Can a new dialect be added through the dialect system?

A "no" answer is not automatically a failure, but it MUST be justified by the language architecture.

---

128. Open-World Operation Validation

Operation syntax SHOULD support data-driven operation names where the specification permits.

This applies especially to:

quantum operations
mathematical operations
AI operations
accelerator operations
vendor extensions
future computational operations

The validator MUST detect unnecessary closed enumerations.

---

129. Vendor Independence

The validator MUST flag core grammar rules that directly encode vendor-specific:

instruction
device
API
accelerator
QPU
GPU
FPGA primitive

unless the construct belongs to an explicitly target-bound interoperability/dialect area.

Vendor-specific integration belongs under:

interoperability/
dialects/
hardware/
backend

as appropriate.

---

130. Interoperability Validation

External formats such as:

OpenQASM
QIR
LLVM-related representations
MLIR-related representations
HDL formats
WASM
foreign-function interfaces

MUST be treated as interoperability boundaries.

They MUST NOT silently become the canonical Zamani source semantic model.

Quantum source syntax MUST still map to canonical:

quantum::ir

before backend-specific interoperability where required.

---

131. Test Naming

Tests SHOULD use stable descriptive names.

Examples:

minimal.zm
classical.zm
generic.zm
quantum.zm
hybrid.zm
hdl.zm
poco-reaf.zm

as already established by the repository's test strategy.

Additional tests SHOULD encode the semantic property being tested rather than an arbitrary implementation detail.

---

132. Test Isolation

A grammar test MUST identify whether it tests:

lexer
parser
AST
semantic
IR
integration

A test MUST NOT pass merely because a downstream implementation accidentally accepts malformed syntax.

---

133. Differential Parser Testing

Where both ANTLR and Rust parser paths exist, the validator SHOULD perform differential testing.

For each test source:

source
 ↓
ANTLR parser

and:

source
 ↓
Rust parser

should produce compatible acceptance/rejection outcomes for stable syntax.

For accepted source, normalized syntax/AST equivalence SHOULD be checked where the repository provides the required adapters.

---

134. Differential Lexer Testing

Likewise:

source
 ↓
ANTLR lexer

and:

source
 ↓
src/lexer.rs

should produce compatible tokenization for stable syntax.

Token identity mappings MUST be explicit where ANTLR and Rust use different internal representations.

---

135. Normalization

The validator MAY normalize:

ANTLR parse trees
Rust AST
source spans
token identities

for comparison.

Normalization MUST NOT erase meaningful distinctions.

In particular it MUST preserve:

operator identity
source structure
literal value
qualified name
type arguments
resource expressions
quantum operation parameters
attributes
modifiers

---

136. AST Equivalence

Equivalent syntax representations MAY differ internally between ANTLR and the Rust parser.

The validator SHOULD compare semantic structure rather than parser-generator implementation details.

The required invariant is:

same accepted source
→ same language meaning

not:

identical internal parser object

---

137. Semantic Equivalence

Where semantic normalization exists, the validator SHOULD verify:

ANTLR parse
→ normalized AST
→ semantic model

and:

Rust parse
→ normalized AST
→ semantic model

converge.

Any intentional difference MUST be documented.

---

138. IR Equivalence

For features that reach canonical IR, the validator SHOULD compare lowering results after permitted normalization.

It MUST ensure that parser implementation differences do not produce different source semantics.

For quantum features this includes the canonical "quantum::ir" representation.

---

139. Optimization Independence

Grammar validation MUST run before optimization-dependent decisions.

A program MUST not become syntactically valid or invalid because optimization is enabled.

Optimization MAY transform IR.

It MUST NOT alter source parsing meaning.

---

140. Routing Independence

Routing MUST remain downstream of language semantics.

The validator MUST reject any architecture where the parser requires a physical topology to determine whether basic quantum syntax is valid.

---

141. Scheduling Independence

Scheduling MUST remain downstream.

Syntax such as:

operation

must not become invalid merely because a particular scheduler cannot currently place it.

That is a resource/backend issue.

---

142. QEC Independence

QEC implementation MUST remain downstream.

The validator MUST ensure that:

requires error_correction(...)

is validated as language/resource intent, while actual code construction remains downstream.

---

143. ZQN Independence

ZQN/fault/noise semantics MUST remain downstream.

The grammar MAY express intent and requirements defined by the specification, but MUST NOT encode a particular noise model as universal syntax unless explicitly specified.

---

144. HAL Independence

HAL owns target capability/state.

The grammar validator MUST not call HAL merely to decide whether a source program is syntactically valid.

---

145. Runtime Independence

Runtime availability MUST NOT affect grammar acceptance.

A valid program can be:

syntactically valid
semantically valid
resource-infeasible on the current target

without being a grammar failure.

---

146. Compatibility With Rust 1.97.1

The validator implementation MUST avoid requiring language/library features introduced after the declared Rust baseline.

Repository CI SHOULD compile the validator using:

rustc 1.97.1

and the declared Rust 2021 edition.

If the repository also supports exactly Rust 1.97, compatibility with that baseline MUST be tested according to the project's toolchain policy.

---

147. No Unsafe Escape Hatch

The validator MUST NOT introduce:

unsafe

merely for:

performance
parser integration
ANTLR integration
large-file handling
source mapping
memory management

Safe abstractions MUST be preferred.

If an external dependency uses unsafe internally, the repository's policy for dependencies MUST determine whether it is acceptable; validator source itself MUST remain safe.

---

148. Validator Performance

Performance optimization MUST preserve correctness.

The validator SHOULD prefer:

indexed lookup
interning where appropriate
incremental validation
dependency-aware invalidation
iterative traversal
streaming
bounded diagnostics
memoization

where these improve scalability without changing semantics.

The validator MUST NOT sacrifice deterministic results for parallelism.

Parallel validation MAY be used if result ordering and findings remain deterministic.

---

149. Parallel Validation

Validation stages MAY execute concurrently when independent.

For example:

lexer validation
type grammar validation
quantum grammar validation
HDL grammar validation

may be analyzed concurrently.

The final result MUST be normalized into deterministic order.

Race-dependent findings are prohibited.

---

150. Memory Management

The validator SHOULD avoid retaining unnecessary copies of:

source
tokens
parse trees
AST
IR
diagnostics

where streaming or indexed representations suffice.

Large repositories MUST NOT require the validator to load every artifact into memory simultaneously if dependency-aware processing can avoid it.

---

151. Repository-Wide Dependency Graph

The validator SHOULD construct a graph:

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
semantic
    ↓
IR
    ↓
backend/runtime

plus domain edges.

The graph MUST support detection of:

missing dependency
circular dependency
unused feature
unreachable feature
orphaned AST node
orphaned grammar rule
orphaned semantic rule
orphaned IR construct

---

152. Orphan Detection

An orphaned artifact is one that claims participation in the language but has no valid integration path.

Examples:

grammar rule with no root reachability
AST node with no parser source
semantic rule with no AST source
IR node with no lowering source
feature manifest with no implementation
test with no corresponding feature

The validator MUST report these.

---

153. Missing Consumer Detection

If a grammar feature declares downstream consumers:

src/semantic.rs
src/ir_gen.rs
src/quantum/ir/

but those consumers do not implement the required contract, the feature MUST remain incomplete.

---

154. Missing Producer Detection

If an AST/semantic/IR node exists but no accepted syntax or semantic source can produce it, the validator SHOULD report it as:

ORPHANED_IMPLEMENTATION

unless it is intentionally internal/generated.

---

155. Test-to-Feature Traceability

Tests SHOULD identify their feature.

For example:

quantum operation test
→ quantum.operations

The validator SHOULD identify stable features lacking required tests.

A test that exists but exercises an obsolete grammar path SHOULD be reported.

---

156. Compatibility Test Traceability

Every breaking change MUST have compatibility tests.

Examples:

keyword addition
operator change
precedence change
grammar removal
dialect change
AST shape change
IR mapping change

---

157. Negative-Space Validation

The validator MUST verify not only what the grammar accepts but also what it must reject.

Examples:

unknown reserved syntax
invalid operator combinations
malformed quantum operation
invalid generic closure
unclosed delimiters
invalid attribute placement
invalid target-bound syntax in portable context

Negative-space validation is essential to prevent accidental language expansion.

---

158. Acceptance-Surface Audit

The validator SHOULD produce an inventory of:

accepted tokens
accepted rules
accepted keywords
accepted operators
accepted literals
accepted declarations
accepted expressions
accepted statements
accepted domains

This inventory SHOULD be compared with:

normative specification
grammar.md

to detect accidental acceptance.

---

159. Rejection-Surface Audit

Likewise, tests SHOULD record constructs that must remain rejected.

This is especially important for:

deprecated syntax
experimental syntax
historical syntax
target-specific syntax
unsupported dialect syntax
malformed syntax

---

160. Experimental Feature Validation

Experimental features MUST be isolated behind explicit status/feature metadata.

The validator MUST ensure that experimental syntax cannot silently become stable syntax merely because the parser happens to recognize it.

---

161. Deprecated Feature Validation

Deprecated syntax MUST:

remain deterministic
produce appropriate diagnostics
have compatibility documentation
have migration guidance where required
not silently change meaning

The validator SHOULD ensure deprecated features have removal/version policy.

---

162. Historical Feature Validation

Historical material in "Zamani-Grammar.md" MUST NOT be treated as active syntax.

The validator SHOULD detect accidental references from canonical grammar to historical-only constructs.

---

163. Documentation Conformance

The validator MUST detect contradictions among:

DESIGN.md
README.md
grammar.md
Zamani-Grammar.md
specification/**
spec/**
validation/**

Not every prose difference is an error.

The validator MUST focus on normative contradictions affecting:

syntax
authority
implementation status
compatibility
scalability
semantic ownership

---

164. README Authority

"grammar/README.md" is navigation/authority documentation.

It MUST NOT introduce grammar productions that contradict "Zamani.g4" or normative specification.

The validator SHOULD flag normative-looking syntax examples in README when they are not linked to an accepted specification/grammar feature.

---

165. "DESIGN.md" Conformance

This validator MUST conform to the architectural principles already defined by "grammar/DESIGN.md".

In particular:

one language
deterministic parsing
no artificial hardware limits
semantic/resource/capability separation
canonical quantum::ir
complete feature pipeline
safe Rust
POCO-REAF

are mandatory architectural constraints.

---

166. Feature Promotion Gate

A feature may progress:

PROPOSED
    ↓
SPECIFIED
    ↓
LEXICALLY_DEFINED
    ↓
GRAMMATICALLY_DEFINED
    ↓
PARSER_IMPLEMENTED
    ↓
AST_IMPLEMENTED
    ↓
SEMANTIC_IMPLEMENTED
    ↓
IR_IMPLEMENTED
    ↓
BACKEND/RUNTIME_IMPLEMENTED
    ↓
TESTED
    ↓
STABLE

The validator MUST NOT permit promotion when a mandatory intermediate stage is absent.

---

167. Feature Demotion

If a previously stable feature loses an essential integration contract, the validator MUST report regression.

Examples:

AST removed
IR lowering removed
parser no longer accepts stable syntax
tests removed
compatibility contract broken

A previously stable feature MUST NOT remain marked stable solely because documentation still says stable.

---

168. Regression Detection

The validator SHOULD compare the current repository state against the previous accepted revision where available.

It SHOULD detect:

new grammar rule
removed grammar rule
changed token
changed keyword
changed precedence
changed AST mapping
changed semantic mapping
changed IR mapping
changed diagnostics

and route each change through compatibility policy.

---

169. Validator Versioning

The validator itself SHOULD have a version.

A validator version change MUST be distinguishable from:

language version
grammar version
feature version
dialect version

A validator update MUST NOT silently redefine language semantics.

---

170. Validator Configuration

Configuration SHOULD contain only validation policy.

It MUST NOT contain hidden source-language semantics.

Allowed examples:

repository root
language version
dialect selection
feature selection
resource budget
diagnostic verbosity
output format
baseline path

Disallowed:

max_qubits = 1024
max_threads = 8
max_tensor_rank = 4

when used as universal language rules.

---

171. Configuration Determinism

Given identical:

configuration
repository revision
toolchain

the validator MUST produce identical findings.

Environment variables SHOULD NOT silently change language validation.

---

172. Continuous Integration Gate

CI SHOULD execute at least:

grammar structure validation
ANTLR generation
Rust compilation
lexer conformance
parser conformance
AST conformance
semantic conformance
IR verification
hard-coding audit
scalability tests
determinism tests
compatibility tests
full grammar tests

A stable grammar merge MUST NOT bypass these gates.

---

173. Local Developer Gate

Developers SHOULD be able to run validation at progressively larger scopes:

single file
single directory
single feature
single domain
grammar subsystem
whole repository

The result MUST remain semantically consistent across scopes.

---

174. Single-File Completion

When a grammar file declares itself complete, validation MUST be able to determine:

what it owns
what it depends on
what consumes it
what tests cover it
what downstream contracts exist

A later unrelated file MUST NOT be expected to redefine the completed file's contract.

This is a direct implementation of the independent-files-first requirement.

---

175. Integration Manifest

Every nontrivial grammar component SHOULD expose an integration manifest or equivalent metadata containing:

owner
dependencies
entry_rules
exported_rules
tokens_used
tokens_defined
ast_nodes
semantic_consumers
ir_consumers
tests
compatibility
status

The validator SHOULD compare declarations against actual references.

---

176. Missing Integration Metadata

If a feature is too small to require a manifest, the file MAY use its documented completion header instead.

For substantial cross-domain features, missing integration metadata MUST prevent full production status.

---

177. Completion Header

Each grammar/validation-related file SHOULD identify:

Path
Status
Purpose
Owns
Does Not Own
Dependencies
Consumers
Normative References
Implementation References
Tests
Completion Criteria

This prevents ambiguity about file responsibility.

---

178. No Unnecessary File Renaming

The validator MUST not require renaming existing repository files simply to satisfy this architecture.

New files/directories MAY be introduced where a real ownership boundary exists.

Existing authoritative files MUST be integrated first.

---

179. Validation of New Directories

A new directory MUST justify:

ownership boundary
independent lifecycle
integration boundary
test boundary
specification boundary

A directory created only to make the tree appear complete MUST NOT be required.

---

180. Validation of Grammar Fragment Names

Names SHOULD reflect semantic ownership.

For example:

quantum/operations.g4
hardware/capabilities.g4
resources/requirements.g4
expressions/arithmetic.g4

are preferable to vague names.

The validator MUST detect collisions but SHOULD NOT enforce arbitrary naming aesthetics beyond the canonical naming policy.

---

181. Cross-File Contract Stability

A completed file's public contract MUST be stable.

If another file needs to modify the completed file merely to consume it, the dependency design is incomplete.

The validator SHOULD identify such cases through integration manifests and dependency analysis.

---

182. Validation of Public Grammar Surface

Only explicitly public grammar rules SHOULD be exposed as stable extension points.

Internal adapter rules MUST be marked accordingly.

This prevents downstream components from depending accidentally on implementation details.

---

183. Grammar API Stability

For reusable grammar fragments, the validator SHOULD treat:

public rule names
token names
entry points
imports

as an API.

Breaking changes MUST pass compatibility validation.

---

184. Source Compatibility

The validator MUST distinguish:

same source still parses

from:

same source parses with different meaning

The second is a semantic compatibility failure even if parsing succeeds.

---

185. Semantic Preservation

Grammar refactoring is allowed only when source semantics are preserved.

For example:

old grammar

and:

refactored modular grammar

may have different parse-tree shapes but MUST produce equivalent language semantics for stable programs.

---

186. Grammar Refactoring Validation

Refactoring MUST test:

positive corpus
negative corpus
AST corpus
semantic corpus
IR corpus

before being considered complete.

---

187. Canonical Source Corpus

The repository SHOULD maintain a canonical conformance corpus under:

grammar/tests/

covering all stable domains.

The validator SHOULD use it for differential testing.

---

188. Minimal Corpus

The minimal corpus MUST include:

minimal.zm
classical.zm
generic.zm
quantum.zm
hybrid.zm
hdl.zm
poco-reaf.zm

or their current repository-equivalent paths.

---

189. POCO-REAF Corpus

"poco-reaf.zm" SHOULD demonstrate:

portable computation
resource requirements
capabilities
preferences
target-independent semantics

without embedding today's machine capacities.

It SHOULD be used to validate that target information remains downstream.

---

190. Quantum Corpus

Quantum conformance SHOULD include:

qubit declaration
parameterized register
state literal
generic operation
custom operation
qualified operation
operation parameters
multiple targets
measurement
reset
control
adjoint
dynamic control
classical feed-forward
resource requirements
capability requirements

No finite gate list may be assumed by the validator.

---

191. HDL Corpus

HDL conformance SHOULD include:

module
ports
signals
nets
registers
combinational logic
sequential logic
clock
reset
parameterized widths
memory
pipeline
state machine
assertion
verification
co-design

without imposing fixed universal widths or capacities.

---

192. Classical Corpus

Classical conformance SHOULD include:

scalars
integers
floating point
vectors
matrices
tensors
functions
closures
control flow
parallelism
numeric operations
symbolic operations

and test that numeric magnitude is not accidentally bounded by lexer representation.

---

193. Distributed Corpus

Distributed conformance SHOULD include:

process
service
message
channel
actor
placement intent
replication
partitioning
consistency
collective
topology requirement

without fixed node counts.

---

194. AI/Data Corpus

AI/data conformance SHOULD include:

tensor
dataset
model
training
inference
pipeline
agent
distributed execution
accelerator capability

without requiring a particular framework or vendor.

---

195. Interoperability Corpus

Interoperability tests SHOULD verify that external formats are adapters.

Examples:

OpenQASM input/output
QIR integration
foreign functions
HDL interoperability
WASM interoperability
FFI

must not silently replace canonical Zamani semantics.

---

196. Validation of Source-Level "unsafe"

The repository contains a source-language concept named "unsafe".

The validator MUST distinguish:

Rust implementation unsafe

from:

Zamani source-language token `unsafe`

The Rust implementation MUST remain safe.

A Zamani source-level unsafe construct, if retained, requires its own:

syntax
semantic
capability
security
compiler
diagnostics
tests

contract.

Parsing the word "unsafe" MUST NOT be interpreted as proof of implementation safety.

---

197. No Hidden Unsafe Dependencies in Validator Logic

The validator MUST NOT rely on unsafe memory manipulation to process:

large grammar
large source
large token stream
large AST
large IR

Safe collections and ownership-aware processing MUST be used.

---

198. Validator Correctness Over Convenience

The validator MUST prefer a false-positive review finding over silently accepting a potentially architecture-breaking feature when automated ownership cannot be determined.

However, findings MUST identify the exact reason for review.

Example:

HARDCODING_REVIEW_REQUIRED:
max_qubits appears in downstream resource policy.
Determine whether it is a backend limit or language limit.

This is preferable to either:

everything with max_qubits is invalid

or:

everything with max_qubits is accepted

---

199. Validation Severity Policy

ERROR

Use for:

ambiguous stable grammar
unreachable required rule
duplicate token identity without justification
missing AST mapping
missing semantic mapping
missing IR mapping
canonical grammar inconsistency
hard-coded universal limit
non-deterministic behavior
Rust unsafe in validator implementation
broken stable compatibility

WARNING

Use for:

experimental feature
deprecated syntax
review-required downstream limit
optional metadata missing
noncritical documentation drift

INFO

Use for:

optimization opportunity
optional coverage
future scalability enhancement

---

200. Final Production-Ready Definition

"grammar/validation/grammar-validator.md" and the validator implementation are production-ready only when the validator can establish all of the following:

[✓] One canonical language
[✓] One canonical ANTLR composition root
[✓] Deterministic lexical behavior
[✓] Deterministic syntactic behavior
[✓] Deterministic semantic ownership
[✓] No unexplained ambiguity
[✓] No unreachable stable grammar
[✓] No accidental duplicate tokens
[✓] Explicit precedence
[✓] Explicit associativity
[✓] Parser recovery makes progress
[✓] Source spans preserved
[✓] Every stable construct maps to AST
[✓] Every executable construct maps to semantics
[✓] Every executable construct maps to IR
[✓] Quantum syntax reaches quantum::ir
[✓] No duplicate quantum IR
[✓] No grammar-level physical hardware limits
[✓] No artificial resource ceilings
[✓] Requirement/capability/preference/realization separation
[✓] POCO-REAF preserved
[✓] Classical/quantum/HDL/hybrid integration
[✓] Distributed scalability
[✓] AI/data extensibility
[✓] Interoperability isolation
[✓] Dialect isolation
[✓] Macro hygiene
[✓] Metaprogramming boundaries
[✓] Compatibility policy
[✓] Feature status traceability
[✓] Feature closure
[✓] Positive tests
[✓] Negative tests
[✓] Boundary tests
[✓] Scalability tests
[✓] Determinism tests
[✓] Compatibility tests
[✓] Cross-domain tests
[✓] Safe Rust
[✓] Rust 1.97 / 1.97.1 compatibility
[✓] No hidden environment dependence
[✓] No hidden hardware dependence
[✓] No hidden filesystem dependence
[✓] No hidden network dependence

Only after these conditions pass may the grammar subsystem report:

PRODUCTION_READY

---

201. Definitive Integration Model

The validator MUST enforce this architecture:

                         ZAMANI SOURCE
                              │
                              ▼
                    grammar/Zamani.g4
                              │
                              ▼
                           LEXER
                              │
                              ▼
                        TOKEN STREAM
                              │
                              ▼
                           PARSER
                              │
                              ▼
                     DOMAIN-NEUTRAL AST
                              │
                              ▼
              ┌───────────────────────────────┐
              │ Structural Validation         │
              │ Name Resolution              │
              │ Type Analysis                │
              │ Effect Analysis              │
              │ Ownership/Linearity          │
              │ Resource Analysis            │
              │ Capability Analysis          │
              │ Portability Validation       │
              └───────────────────────────────┘
                              │
                              ▼
                   CANONICAL SEMANTIC MODEL
                              │
             ┌────────────────┼────────────────┐
             │                │                │
             ▼                ▼                ▼
        CLASSICAL          QUANTUM            HDL
        SEMANTICS          SEMANTICS        HARDWARE
             │                │                │
             │                ▼                │
             │          quantum::ir            │
             │                │                │
             └────────────────┼────────────────┘
                              │
                              ▼
                       CANONICAL IR
                              │
                              ▼
                        IR VERIFICATION
                              │
                              ▼
                         OPTIMIZATION
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
          ROUTING         SCHEDULING       RESILIENCE
             │                │                │
             └────────────────┼────────────────┘
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
        ┌───────────┬─────────┼─────────┬────────────┐
        ▼           ▼         ▼         ▼            ▼
       CPU         GPU       FPGA      QPU       FUTURE
        │           │         │         │        TARGETS
        └───────────┴─────────┴─────────┴────────────┘

The validator exists to prove that the grammar remains on the left side of the semantic/realization boundary.

---

202. Final POCO-REAF Contract

The validator MUST protect the following invariant:

                         SAME PROGRAM
                              │
            ┌─────────────────┼─────────────────┐
            ▼                 ▼                 ▼
        tiny target       medium target     enormous target
            │                 │                 │
            ▼                 ▼                 ▼
           CPU             GPU/FPGA        QPU/HPC/cluster
            │                 │                 │
            └─────────────────┼─────────────────┘
                              │
                              ▼
                     SAME SOURCE SEMANTICS

Different targets MAY require:

different optimization
different routing
different scheduling
different resource placement
different QEC strategy
different backend lowering
different runtime decisions

but they MUST NOT require the language grammar to be rewritten merely because the available machine resources changed.

---

203. Final Validator Rule

The ultimate rule is:

Zamani grammar validation MUST validate portable program meaning,
not today's machine.

Therefore:

Grammar
    defines syntax.

Lexer
    defines tokenization.

Parser
    defines syntactic structure.

AST
    preserves source structure.

Semantic analysis
    defines program meaning and validity.

Resource/capability analysis
    defines what the program requires.

Canonical IR
    defines executable computation.

quantum::ir
    remains the canonical quantum semantic/IR boundary.

Optimization
    transforms implementation while preserving meaning.

Routing
    realizes physical/resource placement.

Scheduling
    realizes execution order and resource timing.

QEC
    realizes quantum error-correction strategy.

ZQN
    models quantum fault/noise semantics.

HAL
    exposes target capabilities and state.

Runtime/backend
    executes the realized program.

Grammar validation
    verifies that these boundaries remain correct.

No hardware limitation may become a language limitation merely because that hardware exists today.

No backend capability may become parser state merely because that backend exists.

No feature may be called production-ready merely because its syntax parses.

No quantum frontend representation may replace "quantum::ir".

No Rust "unsafe" may be introduced into the validator/compiler implementation.

No fixed resource maximum may be used as a universal language ceiling.

No file may be considered complete while its ownership, AST, semantic, IR, integration, compatibility, scalability, diagnostic, and test contracts remain undefined.

The production objective is therefore:

                    ONE LANGUAGE
                         │
                         ▼
                  ONE SOURCE MODEL
                         │
                         ▼
                 DETERMINISTIC GRAMMAR
                         │
                         ▼
                  DOMAIN-NEUTRAL AST
                         │
                         ▼
                  CANONICAL SEMANTICS
                         │
                         ▼
                    CANONICAL IR
                         │
             ┌───────────┼───────────┐
             ▼           ▼           ▼
         Classical     Quantum      HDL
             │           │           │
             └───────────┼───────────┘
                         ▼
                 OPTIMIZE / LOWER
                         │
             ┌───────────┼───────────┐
             ▼           ▼           ▼
          Routing     Scheduling    Resilience
             │           │           │
             └───────────┼───────────┘
                         ▼
                        ZQN
                         │
                         ▼
                        HAL
                         │
                         ▼
                 ANY CAPABLE TARGET

with:

Program Once
    →
Compile Once
    →
Run Everywhere
    →
Anywhere
    →
Forever

subject only to the actual semantic requirements of the program, the capabilities and resources of the selected target, implementation/resource budgets, and explicitly declared constraints—not arbitrary limits accidentally embedded in the grammar.