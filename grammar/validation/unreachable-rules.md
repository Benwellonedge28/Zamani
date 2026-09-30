Zamani Unreachable Grammar Rules Validation

Path: "grammar/validation/unreachable-rules.md"
Status: Normative production validation specification
Language: Zamani
Grammar technology: ANTLR4-compatible modular grammar architecture
Rust baseline: Rust 1.97 / Rust 1.97.1, Edition 2021
Safety: Production Rust implementation MUST use safe Rust; "unsafe" is prohibited
Primary objective: Deterministic detection and classification of unreachable grammar rules and alternatives across the complete Zamani grammar composition graph
Scalability objective: From the smallest valid grammar/program to arbitrarily large grammars/programs subject only to actual implementation resources and explicitly configured validation budgets
Portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)"
Canonical root grammar: "grammar/Zamani.g4"
Canonical parser composition: "grammar/antlr/ZamaniParser.g4"
Canonical lexer composition: "grammar/antlr/ZamaniLexer.g4"
Executable lexer: "src/lexer.rs"
Executable parser: "src/parser.rs"
Source AST: "src/ast/"
Semantic analysis: "src/semantic.rs"
Canonical IR lowering: "src/ir_gen.rs"
IR verification: "src/ir_verify.rs"
Canonical quantum semantic/IR boundary: "src/quantum/ir/"

---

1. Purpose

This document defines the complete production contract for detecting, classifying, reporting, and resolving unreachable grammar rules and unreachable grammar alternatives in Zamani.

The validator MUST answer all of the following questions:

1. Can the rule be reached from a canonical parser entry point?
2. Can the rule be reached through the actual imported grammar composition graph?
3. Can an alternative inside an otherwise reachable rule ever be selected?
4. Is the rule intentionally standalone?
5. Is the rule test-only?
6. Is the rule generated/supporting infrastructure?
7. Is the rule deprecated?
8. Is the rule experimental or not yet implemented?
9. Is the rule accidentally orphaned?
10. Is the apparent orphan actually caused by a broken or missing grammar import?
11. Is the rule reachable from source syntax but semantically unsupported downstream?
12. Does the rule have AST, semantic, or IR integration?
13. Does the rule accidentally create a second language surface?
14. Does the rule introduce a fixed scalability limit?
15. Does the rule violate the canonical grammar ownership model?

Unreachable-rule validation is therefore not merely a parser warning.

It is a repository-wide conformance analysis.

The validator MUST distinguish:

unreachable grammar rule

from:

unresolved grammar dependency

and:

intentionally standalone grammar rule

and:

deprecated grammar rule

and:

test-only grammar rule

and:

generated/support rule

These categories have different consequences.

---

2. Architectural Authority

Unreachable-rule validation MUST follow the repository's existing authority model.

Artifact| Responsibility
"grammar/DESIGN.md"| Normative architecture
"grammar/README.md"| Grammar navigation and authority model
"grammar/specification/"| Normative human-readable language specification
"grammar/spec/"| Formal language contracts
"grammar/Zamani.g4"| Canonical complete-program ANTLR composition root
"grammar/antlr/ZamaniParser.g4"| Canonical parser composition
"grammar/antlr/ZamaniLexer.g4"| Canonical lexer composition
"grammar/*/*.g4"| Modular grammar ownership
"src/lexer.rs"| Executable lexical implementation
"src/parser.rs"| Executable reference parser
"src/ast/"| Executable source AST
"src/semantic.rs"| Semantic analysis
"src/ir_gen.rs"| AST/semantic lowering to IR
"src/ir_verify.rs"| IR structural verification
"src/quantum/ir/"| Canonical quantum semantic/IR boundary
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Historical/proposed/extended design material
"grammar/validation/"| Validation policy
"grammar/tests/"| Grammar conformance evidence

No rule appearing only in "Zamani-Grammar.md" becomes reachable merely because it is described there.

No rule appearing only in "grammar/specification/" becomes implemented merely because it is specified there.

No rule appearing only in a leaf grammar becomes part of the canonical source language unless the canonical composition graph reaches it.

---

3. Scope

This contract covers:

- parser grammar rules;
- parser grammar alternatives;
- imported parser grammars;
- imported lexer grammars where parser reachability depends on lexical composition;
- modular grammar dispatchers;
- dialect grammar composition;
- interoperability grammars;
- generated/supporting grammar rules;
- test-only grammar fragments;
- deprecated grammar fragments;
- grammar rules that are intentionally standalone;
- parser entry points;
- cross-domain dispatch;
- source-level reachability;
- grammar dependency reachability;
- semantic/AST/IR integration classification.

This contract does NOT make unreachable-rule analysis responsible for:

- type checking;
- resource allocation;
- hardware discovery;
- routing;
- scheduling;
- QEC;
- ZQN;
- calibration;
- HAL;
- runtime execution.

Those downstream responsibilities are checked through the corresponding validation contracts.

---

4. Canonical Entry Point

The canonical complete-source entry point is:

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

The current "grammar/Zamani.g4" explicitly defines:

program
    : sourceUnit EOF
    ;

Therefore "program" is the canonical root for complete Zamani source parsing.

A production validator MUST begin reachability from this canonical root.

It MUST NOT begin from:

- an arbitrary leaf rule;
- a documentation example;
- a historical grammar;
- a generated parser method;
- a rule discovered by filename;
- a test fixture;
- a domain-specific entry point.

---

5. Complete Grammar Composition Graph

Reachability MUST be calculated over the complete grammar composition graph.

The expected architecture is:

Zamani.g4
    |
    +--> ZamaniParser
    |
    +--> ZamaniLexer
            |
            v
      canonical lexical hierarchy

The parser composition is then conceptually:

ZamaniParser
    |
    +--> Core
    +--> Types
    +--> Expressions
    +--> Declarations
    +--> Statements
    +--> Functions
    +--> Modules
    +--> Effects
    +--> Memory
    +--> Concurrency
    +--> Classical
    +--> Quantum
    +--> Hybrid
    +--> HDL
    +--> Hardware
    +--> Distributed
    +--> AI
    +--> Data
    +--> Networking
    +--> Security
    +--> Resources
    +--> Compile
    +--> Execution
    +--> Interoperability
    +--> Dialects
    +--> Macros
    +--> Metaprogramming

The validator MUST verify that every imported grammar name resolves to exactly one canonical grammar composition unit.

---

6. Current Repository Composition Requirement

The current repository contains:

grammar/Zamani.g4
grammar/antlr/ZamaniParser.g4
grammar/antlr/ZamaniLexer.g4

"ZamaniParser.g4" imports:

Core
Types
Expressions
Declarations
Statements
Functions
Modules
Effects
Memory
Concurrency
Classical
Quantum
Hybrid
HDL
Hardware
Distributed
AI
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

The validator MUST resolve these imports by grammar identity, not by assuming that a directory name automatically represents a grammar.

This distinction is critical because the repository currently contains many leaf ".g4" files under directories such as:

grammar/core/
grammar/types/
grammar/expressions/
grammar/quantum/
grammar/hdl/
...

but the existence of leaf files does not itself prove that a canonical parser grammar named "Core", "Types", "Quantum", "HDL", etc. exists.

Therefore the validator MUST distinguish:

MISSING_COMPOSITION_GRAMMAR

from:

UNREACHABLE_RULE

A missing composition grammar means the reachability graph cannot yet be completely constructed.

It MUST NOT be reported as though every rule in the missing grammar were proven unreachable.

---

7. Reachability Is a Graph Property

For every parser grammar rule:

Rule A

the validator constructs directed edges for references to other parser rules.

For example:

sourceUnit
    : sourceElement*
    ;

produces:

sourceUnit → sourceElement

and:

sourceElement
    : declarationElement
    | statementElement
    | domainElement
    ;

produces:

sourceElement → declarationElement
sourceElement → statementElement
sourceElement → domainElement

Reachability is then the transitive closure beginning at the canonical root:

program

A rule is canonically reachable if there exists a valid path:

program → ... → rule

through resolved parser grammar dependencies.

---

8. Reachability Must Be Computed Across Imports

A validator MUST NOT limit analysis to one ".g4" file.

For example:

program
  ↓
sourceUnit
  ↓
sourceElement
  ↓
domainElement
  ↓
quantumElement
  ↓
quantumDeclaration
  ↓
quantumOperation

may cross multiple grammar files.

The validator MUST therefore construct a repository-level grammar graph:

Grammar
  |
  +--> imported Grammar
          |
          +--> imported Grammar
                  |
                  +--> rule

Every edge must carry provenance:

source grammar
source rule
reference
target grammar
target rule

This permits precise diagnostics.

---

9. Rule Classification

Every discovered parser rule MUST receive exactly one primary reachability classification.

The supported classifications are:

REACHABLE
STANDALONE
TEST_ONLY
GENERATED_SUPPORT
DEPRECATED
EXPERIMENTAL
UNIMPLEMENTED
ORPHANED
UNRESOLVED_DEPENDENCY

A rule MAY additionally carry secondary metadata such as:

DOMAIN_CORE
DOMAIN_QUANTUM
DOMAIN_CLASSICAL
DOMAIN_HDL
DOMAIN_HARDWARE
DOMAIN_AI
DOMAIN_DISTRIBUTED
DOMAIN_INTEROPERABILITY
...

but domain classification MUST NOT replace reachability classification.

---

10. REACHABLE

A rule is "REACHABLE" when:

1. its grammar is resolved;
2. its rule definition is resolved;
3. the rule has a path from a canonical parser entry point;
4. all dependencies required for that path are resolvable;
5. the path is not dependent on disabled or invalid grammar composition;
6. the rule belongs to the canonical Zamani source grammar.

A reachable rule is eligible for stable source syntax only after the other conformance contracts are satisfied.

Reachability alone does not imply:

implemented
semantically valid
AST complete
IR complete
runtime supported
hardware supported

Those require downstream evidence.

---

11. STANDALONE

A rule is "STANDALONE" when it is intentionally exposed as an independent grammar fragment but is not reachable from the complete-program root.

Examples include:

- parser tooling entry points;
- syntax fragment testing;
- editor/language-server parsing entry points;
- interoperability fragment entry points;
- grammar conformance fixtures;
- reusable fragments deliberately exposed to grammar tooling.

A standalone rule MUST have explicit documentation.

The validator MUST NOT infer standalone status merely because:

- its name begins with "universal";
- it is in a README;
- it is referenced by a test;
- it appears useful.

Standalone status requires an explicit repository contract.

---

12. TEST_ONLY

A rule is "TEST_ONLY" when it exists exclusively to test grammar behavior.

Test-only rules MUST:

- reside in an explicitly test-scoped grammar;
- not be imported by the canonical grammar;
- not be described as stable source syntax;
- not be used to infer language capabilities;
- not affect "grammar/grammar.md" as implemented source syntax.

A test-only rule accidentally imported by canonical grammar composition MUST be reported.

---

13. GENERATED_SUPPORT

Generated/support rules may exist to support:

- parser generation;
- lexer generation;
- grammar transformation;
- parser diagnostics;
- tooling;
- code generation.

Such rules are not necessarily source-language rules.

The validator MUST prevent them from being mistaken for public language syntax.

Generated rules MUST carry provenance identifying:

generator
source grammar
generation stage
generated artifact

Generated output MUST NOT become a second authority.

---

14. DEPRECATED

A deprecated rule may remain reachable for compatibility.

That does not make it a preferred current-language construct.

A deprecated rule MUST specify:

deprecated since
replacement
migration path
removal policy
compatibility status
tests

A deprecated rule that is unreachable is not automatically an error.

It is valid only if the compatibility contract intentionally preserves its historical record.

---

15. EXPERIMENTAL

Experimental rules may be intentionally unreachable from the stable grammar.

This is preferable to silently exposing experimental syntax.

Experimental rules MUST specify:

feature identifier
owner
status
syntax
AST mapping
semantic mapping
IR mapping
feature gate
tests
promotion criteria

An experimental rule MUST NOT be reported as a stable unreachable-rule failure.

---

16. UNIMPLEMENTED

A rule may be specified but not yet implemented.

For example:

specification
    ↓
proposed grammar
    ↓
AST not implemented

Such a rule MUST NOT be silently included in the stable language.

The validator should report:

UNIMPLEMENTED_RULE

rather than:

REACHABLE

if it is intentionally held outside the canonical source composition.

---

17. ORPHANED

A rule is "ORPHANED" when:

- it is not reachable;
- it is not intentionally standalone;
- it is not test-only;
- it is not generated/support;
- it is not deprecated;
- it is not experimental;
- it is not intentionally unimplemented;
- and no documented reason explains its isolation.

An orphaned rule is a production error.

Diagnostic category:

ZGRM-RCH-001
ORPHANED_GRAMMAR_RULE

The diagnostic MUST identify:

grammar
rule
source span
classification evidence
nearest known grammar owner
suggested integration point

---

18. UNRESOLVED_DEPENDENCY

This is a distinct failure.

Example:

import Quantum;

when no grammar named "Quantum" can be resolved.

Diagnostic:

ZGRM-RCH-002
UNRESOLVED_GRAMMAR_DEPENDENCY

The validator MUST NOT transform this into:

Quantum rules are unreachable

because that would be logically incorrect.

The correct conclusion is:

reachability analysis is incomplete because a required composition dependency is unresolved

---

19. Current "ZamaniParser.g4" Universal Rules

The current canonical parser composition contains the following universal integration rules:

universalDeclaration
universalStatement
universalExpression
universalType
universalBlock
universalFunction
universalModule
universalEffect
universalResource
universalHardware
universalQuantum
universalHDL
universalDistributed
universalAI
universalData
universalNetworking
universalSecurity
universalCompilation
universalExecution
universalInteroperability
universalDialect
universalMacro
universalMetaprogramming

These rules are not directly reachable from the current:

program
    ↓
sourceUnit
    ↓
sourceElement

source path.

They therefore MUST NOT be silently classified as ordinary reachable source rules.

Their correct architectural interpretation is:

parser-level integration contracts

unless and until they are deliberately wired into the canonical source grammar.

The validator MUST classify them as:

STANDALONE

only when an explicit standalone contract exists.

Otherwise they MUST be reported as:

ORPHANED

until the repository explicitly establishes their intended role.

The validator MUST NOT automatically connect them to "program" merely to make the warning disappear.

That would change the language.

---

20. Why Universal Rules Must Not Be Automatically Connected

A rule such as:

universalQuantum
    : quantumDeclaration
    ;

does not mean that every quantum declaration should be independently legal at every source position.

Likewise:

universalType
    : typeExpression
    ;

does not mean that a type expression should become a top-level program element.

Reachability is therefore a semantic architecture question as well as a graph question.

The validator MUST never solve an orphan warning by adding arbitrary edges.

---

21. Rule Ownership

Every grammar rule MUST have exactly one owner.

For example:

program
    → grammar/Zamani.g4

sourceUnit
    → grammar/antlr/ZamaniParser.g4

quantum operation syntax
    → grammar/quantum/

HDL signal syntax
    → grammar/hdl/

type syntax
    → grammar/types/

expression precedence
    → grammar/expressions/

A rule duplicated in two authoritative grammar locations MUST produce:

DUPLICATE_GRAMMAR_AUTHORITY

This is different from unreachable-rule detection but MUST be reported by the same validation pipeline.

---

22. Rule Ownership and Reachability

A rule cannot be considered production-complete merely because it is reachable.

The production chain remains:

rule
 ↓
lexer/token contract
 ↓
AST mapping
 ↓
semantic mapping
 ↓
canonical IR mapping
 ↓
implementation
 ↓
tests

For quantum syntax:

quantum grammar
 ↓
domain-neutral AST
 ↓
semantic quantum representation
 ↓
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

Unreachable-rule validation MUST therefore expose the rule's downstream integration metadata where available.

---

23. No Grammar-to-Hardware Reachability

The reachability graph MUST stop at grammar ownership.

It MUST NOT follow:

grammar
 → hardware backend
 → physical device

because hardware realization is not grammar reachability.

The grammar describes portable source meaning.

Hardware capabilities are downstream.

---

24. POCO-REAF Invariant

Unreachable-rule analysis MUST preserve POCO-REAF.

The validator MUST NOT make a rule unreachable merely because a target cannot currently support it.

For example:

requires capability("quantum.measurement")

does not make quantum syntax unreachable.

Likewise:

requires memory >= required_memory

does not make a memory rule unreachable.

The parser describes intent.

Target capability is evaluated later.

---

25. No Artificial Scalability Limits

Reachability algorithms MUST NOT use language-level constants such as:

MAX_RULES
MAX_GRAMMAR_DEPTH
MAX_REACHABILITY_DEPTH
MAX_ALTERNATIVES
MAX_IMPORTS
MAX_DOMAIN_COUNT
MAX_PROGRAM_SIZE
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

These are prohibited as universal language limits.

The graph traversal MUST continue until the reachable closure has been computed or an explicitly configured implementation resource budget is exhausted.

---

26. Resource Budgets

A validator implementation MAY have externally configured budgets for:

- wall-clock time;
- memory;
- diagnostic count;
- graph nodes processed;
- graph edges processed;
- parser/grammar expansion work;
- filesystem traversal;
- incremental-validation work.

These are implementation budgets.

They MUST NOT become language semantics.

A budget exhaustion diagnostic MUST be distinct:

ZGRM-RCH-900
VALIDATION_RESOURCE_EXHAUSTED

It MUST NOT be reported as:

UNREACHABLE_RULE

---

27. Deterministic Reachability

Given identical:

repository snapshot
grammar files
grammar configuration
feature gates
dialect configuration
language version
validator version
resource policy

reachability results MUST be identical.

Diagnostic ordering MUST be deterministic.

Recommended deterministic ordering:

1. canonical grammar path;
2. grammar name;
3. rule name;
4. source position;
5. diagnostic code.

Hash-map iteration order MUST NOT determine diagnostic order.

---

28. Grammar Dependency Graph

The validator MUST model at least three graph layers.

Layer A — Grammar import graph

Grammar → Grammar

Layer B — Rule reference graph

Rule → Rule

Layer C — Composition ownership graph

Domain → Dispatcher → Leaf grammar

The validator MUST keep these graphs distinct.

This prevents a missing import from being misreported as an unreachable rule.

---

29. Grammar Import Resolution

Every import MUST resolve by grammar identity.

The validator MUST verify:

imported grammar exists
grammar kind is correct
grammar name is correct
grammar appears exactly once
grammar source is unambiguous
grammar version is compatible
grammar is enabled

Duplicate grammar identities MUST fail validation.

---

30. Path Does Not Define Grammar Identity

The validator MUST NOT assume:

grammar/quantum/

automatically means:

Quantum

A grammar's declared name is authoritative for ANTLR composition.

Therefore:

filesystem path
+
grammar declaration
+
import graph

must agree.

A mismatch is:

ZGRM-RCH-003
GRAMMAR_IDENTITY_MISMATCH

---

31. Duplicate Grammar Identity

Two files declaring the same grammar identity create a competing authority.

For example:

grammar/quantum/Quantum.g4
grammar/legacy/Quantum.g4

both declaring:

parser grammar Quantum;

must fail unless one is explicitly classified as:

historical
deprecated
test-only
foreign
generated

and excluded from canonical composition.

---

32. Rule Definition Uniqueness

Within one canonical grammar namespace, each parser rule name MUST be defined exactly once.

Duplicate definitions produce:

ZGRM-RCH-004
DUPLICATE_RULE_DEFINITION

The validator MUST identify all definitions.

It MUST NOT arbitrarily select one.

---

33. Rule Reference Resolution

For every parser-rule reference:

source grammar
source rule
target rule

the validator MUST determine whether the target is:

local
imported
qualified
generated
foreign
missing
ambiguous

A missing rule is not an unreachable rule.

It is:

UNRESOLVED_RULE_REFERENCE

---

34. Token Versus Rule Resolution

ANTLR grammars distinguish parser rules and lexer tokens.

The validator MUST not confuse:

identifier

with:

IDENTIFIER

or any equivalent naming convention.

Case and grammar declaration rules MUST be respected.

A token referenced as a parser rule, or a parser rule referenced as a token, is a grammar-construction error.

---

35. Reachability From Multiple Public Entry Points

The canonical complete-source root is:

program

but tooling may intentionally expose other entry points.

Examples:

expression
typeExpression
pattern
quantumDeclaration
hdlModule

These must not automatically become public language roots.

Each additional entry point MUST be explicitly classified.

The validator therefore computes:

canonical reachability

and separately:

tooling reachability

A rule reachable only through tooling MUST NOT be reported as stable source syntax.

---

36. Standalone Entry-Point Contract

A standalone grammar entry point MUST declare:

entry-point-id
purpose
owner
consumer
source-language status
AST output
semantic output
test suite
canonical status

Example:

entry-point-id: expression-fragment
purpose: editor expression parsing
consumer: language server
source-language status: fragment-only

---

37. Alternative Reachability

Rule reachability is not enough.

Every alternative MUST also be analyzed.

Consider:

rule
    : identifier
    | identifier "." identifier
    ;

Depending on the surrounding grammar, the first alternative may consume only a prefix and prevent the second from being selected.

The validator MUST analyze:

alternative reachability

independently of:

rule reachability

---

38. Dead Alternatives

A dead alternative is an alternative that cannot be selected for any valid input under the grammar's actual parser semantics.

Diagnostic:

ZGRM-RCH-010
DEAD_GRAMMAR_ALTERNATIVE

The validator MUST provide:

rule
alternative index
source span
reason
subsuming alternative
witness or proof basis

---

39. Prefix Subsumption

The validator MUST detect cases where an earlier alternative consumes the same viable prefix and prevents a later alternative from being selected under the relevant grammar context.

It MUST use FIRST/FOLLOW and lookahead information where necessary.

It MUST NOT rely solely on textual comparison.

---

40. Empty Alternative Analysis

Empty alternatives require special handling.

Example:

modifier
    :
    | PUBLIC
    ;

An empty alternative may make another rule nullable.

The validator MUST propagate nullability through:

?
*
+
optional fragments
empty alternatives
delegated nullable rules

This information is required for both reachability and recursion analysis.

---

41. Nullable Prefixes

A rule can be reachable but still have an unreachable branch due to nullable prefixes.

Example:

A
    : B C
    | C
    ;

B
    :
    | X
    ;

The validator MUST determine whether the first alternative contributes any language not already accepted by the second.

---

42. Recursive Reachability

Recursive rules are reachable when their entry point is reachable.

For example:

expression
  ↓
postfixExpression
  ↓
callSuffix
  ↓
expression

does not imply that the recursive occurrence is unreachable.

The validator MUST distinguish:

reachable recursion

from:

unreachable recursion

and from:

left recursion

Left-recursion policy remains governed by:

grammar/validation/left-recursion.md

---

43. Cycles

A grammar cycle is not automatically an unreachable rule.

Example:

expression
 → block
 → statement
 → expression

may be completely reachable.

The validator MUST therefore report cycles separately:

RECURSIVE_CYCLE

and only report unreachable rules when there is no valid path from an enabled entry point.

---

44. Nullable Cycles

A cycle consisting entirely of nullable transitions is a production error.

Example:

A → B
B → C
C → A

where all three rules can consume zero tokens.

This must be reported through left-recursion/progress validation, not mislabeled merely as unreachable.

The two findings may coexist.

---

45. Progress Requirement

Every canonical parser path must either:

1. consume a token before recursively re-entering the same nullable cycle; or
2. terminate.

This prevents:

infinite parser recursion

and:

non-progress loops

The rule is especially important for scalable programs because resource-scalable syntax must not become resource-exhausting solely due to grammar structure.

---

46. Source-Level Reachability Versus Semantic Reachability

A parser rule may be reachable while its resulting AST construct is semantically rejected.

That is not an unreachable grammar rule.

Example:

quantumOperation

may be syntactically valid while semantic analysis later rejects an invalid resource requirement.

The validator MUST NOT remove grammar rules because semantic validation rejects some instances.

---

47. AST Integration

For every stable reachable rule, the validator SHOULD identify its AST integration.

The expected relationship is:

grammar rule
    ↓
AST node

The AST in "src/ast/mod.rs" currently includes constructs such as:

Statement::QuantumCircuit
Statement::NoiseModel
Statement::FidelityCheck
Statement::SurfaceCode
Statement::NanoAgent
Statement::SankofaMemory
Statement::EffectDeclaration
...

and:

Expression::QuantumOp
Expression::Entangle
Expression::NanoOp
Expression::Recall
Expression::Remember
Expression::Learn
...

Reachability validation MUST NOT infer that every AST variant corresponds to stable grammar syntax.

It must instead verify an explicit mapping.

---

48. AST Orphans

A stable AST node with no grammar production is a separate conformance issue.

Diagnostic:

ZGRM-RCH-020
AST_WITHOUT_STABLE_GRAMMAR

This is not the same as an unreachable grammar rule.

Likewise:

grammar rule with no AST mapping

should be reported as:

ZGRM-RCH-021
GRAMMAR_WITHOUT_AST_MAPPING

unless the rule is intentionally syntax-only infrastructure.

---

49. Semantic Integration

Every stable reachable grammar construct that produces AST should identify its semantic consumer.

Conceptually:

grammar
 ↓
AST
 ↓
semantic analysis

The semantic implementation currently operates over "crate::ast::*".

Therefore the validator MUST check that stable grammar constructs do not require semantic concepts that the AST cannot represent.

---

50. IR Integration

For constructs that lower into IR, the expected path is:

grammar
 ↓
AST
 ↓
semantic analysis
 ↓
IR

The current repository contains:

src/ir_gen.rs
src/ir_verify.rs

The validator MUST ensure that grammar reachability does not falsely imply IR implementation.

A reachable grammar rule without required IR support is:

GRAMMAR_REACHABLE_BUT_IR_UNIMPLEMENTED

not:

UNREACHABLE_RULE

---

51. Quantum Integration

Quantum grammar reachability MUST terminate at the canonical semantic boundary:

quantum::ir

The validator MUST NOT require grammar rules to reference:

- physical qubit IDs;
- coupling maps;
- native gate sets;
- calibration data;
- pulse schedules;
- physical topology;
- backend-specific instruction encodings.

A generic quantum operation remains reachable if its source syntax is part of the canonical grammar.

The grammar MUST remain compatible with extensible operations such as:

apply H
apply custom_gate
apply vendor.operation
apply operation(parameter)

without making a finite gate catalogue a reachability requirement.

---

52. Classical Integration

Classical grammar rules may describe:

- scalar computation;
- vector computation;
- matrix computation;
- tensor computation;
- numerical computation;
- symbolic computation;
- signal processing;
- optimization;
- concurrency;
- parallel computation.

A rule does not become unreachable because a particular target lacks an accelerator.

Target capability is downstream.

---

53. HDL Integration

HDL grammar rules must remain reachable based on language composition, not based on whether a particular FPGA, ASIC, simulator, or synthesis backend exists.

The validator MUST NOT classify:

register
signal
pipeline
clock
memory
interface
state machine

as unreachable because a target does not support a particular implementation.

---

54. Hardware Intent

Hardware intent syntax remains source-level semantic intent.

For example:

requires capability("gpu.compute")
requires capability("quantum.measurement")
requires memory(...)
requires topology(...)

must be treated as grammar constructs independent of target realization.

---

55. Distributed Computing

Rules for:

nodes
processes
services
actors
messages
channels
replication
partitioning
consistency
collectives

must not require a fixed number of machines.

A distributed rule remains reachable regardless of current cluster size.

---

56. AI and Data Domains

AI/data rules must be validated through the same graph.

Framework-specific implementations such as:

CUDA
PyTorch
TensorFlow
vendor-specific APIs

must not become implicit reachability roots.

Framework interoperability is downstream.

---

57. Dialect Reachability

Dialect rules require special treatment.

A dialect may extend the canonical language without making every dialect rule part of the default language.

The validator MUST compute:

default dialect reachability

and:

enabled dialect reachability

separately.

An inactive dialect rule MUST NOT be reported as an orphan if its dialect manifest explicitly identifies it as gated.

---

58. Dialect Cycle Protection

A dialect MUST NOT introduce:

core → dialect → core

without explicit composition and guaranteed termination.

A dialect MUST NOT silently create a second program root.

A dialect cannot bypass reachability validation merely by being dynamically selected.

---

59. Interoperability Grammar Reachability

Foreign grammar fragments such as:

OpenQASM
QIR
HDL formats
C/C++
Rust
Python
WASM
serialization formats

must be treated as interoperability artifacts unless explicitly composed into Zamani source syntax.

An interoperability grammar that is not imported into the canonical parser is not an unreachable stable Zamani rule.

It is a separate grammar artifact.

---

60. Foreign Grammar Classification

The validator MUST support:

CANONICAL_ZAMANI
DIALECT
INTEROPERABILITY
HISTORICAL
TEST_ONLY
GENERATED

grammar scopes.

Only "CANONICAL_ZAMANI" contributes to the default source-language reachability closure.

---

61. Historical Grammar

"grammar/Zamani-Grammar.md" is not itself a parser grammar.

Its feature descriptions MUST NOT be converted into reachability edges.

Historical material must remain:

documentation

until promoted through the established process:

proposal
 ↓
semantic design
 ↓
AST
 ↓
canonical grammar
 ↓
implementation
 ↓
IR
 ↓
tests
 ↓
stable

---

62. "grammar/grammar.md"

"grammar/grammar.md" is an implementation-conformance reference.

The unreachable-rule validator SHOULD feed its findings into the implementation status model:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

An unreachable stable rule MUST NOT be represented as:

IMPLEMENTED

merely because its grammar file exists.

---

63. Diagnostics

Every unreachable-rule diagnostic MUST contain:

code
severity
grammar
rule
source location
classification
reason
canonical root
nearest reachable ancestor, if any
dependency path
suggested action

Example:

error[ZGRM-RCH-001]:
grammar rule `quantumOperation` is not reachable from canonical
entry point `program`.

grammar: grammar/quantum/operations.g4
classification: ORPHANED
root: program
nearest reachable ancestor: quantumElement
suggestion: integrate the owning dispatcher or explicitly classify
the rule as standalone/test-only/experimental.

---

64. Missing Dependency Diagnostic

Example:

error[ZGRM-RCH-002]:
canonical parser composition imports grammar `Quantum`, but no unique
canonical parser grammar named `Quantum` was resolved.

This prevents complete reachability analysis of quantum grammar rules.

This is an unresolved grammar dependency, not proof that the quantum
rules are unreachable.

This distinction is mandatory.

---

65. Standalone Diagnostic

If a standalone rule lacks classification metadata:

error[ZGRM-RCH-005]:
rule `universalQuantum` is unreachable from `program` and has no
explicit standalone/test/generated/deprecated/experimental classification.

This prevents accidental grammar islands.

---

66. Reachability Metadata

Where grammar metadata is available, it SHOULD support:

reachability:
  status: reachable | standalone | test-only | generated | deprecated |
          experimental | unimplemented
  entry_points:
    - program
  owner: grammar/quantum/
  public: true | false

The exact metadata representation is governed by the repository's feature/specification system.

The validator MUST NOT require YAML/TOML/JSON specifically unless another repository contract establishes that format.

The essential requirement is explicit machine-readable classification.

---

67. Comments Are Not Sufficient Evidence

A prose comment saying:

this rule is standalone

is weaker than a machine-readable classification.

The validator MAY use documented comments as human diagnostics, but production status MUST ultimately be represented by an authoritative classification mechanism.

Otherwise a typo or stale comment can silently change validation behavior.

---

68. No Name-Based Exceptions

The validator MUST NOT say:

if rule name starts with "universal" then ignore unreachable

That would make the validator dependent on naming accidents.

The same applies to:

quantum*
hdl*
ai*
test*
experimental*
legacy*

Names can assist diagnostics.

They MUST NOT define correctness.

---

69. No Directory-Based Exceptions

Likewise:

grammar/quantum/

does not automatically make every rule reachable.

And:

grammar/tests/

does not automatically make every rule test-only if the grammar is imported into canonical composition.

Composition and explicit classification determine status.

---

70. Generated Grammar Exceptions

Generated grammars may contain rules that are not directly reachable from the canonical root.

They must be classified using provenance.

A generated artifact cannot become authoritative simply because it contains more rules than the source grammar.

---

71. Unreachable Alternatives in Generated Grammars

Generated parser transformations can introduce helper rules.

These must be marked:

GENERATED_SUPPORT

and traced to their source grammar.

The validator MUST NOT report generated implementation artifacts as user-language defects unless their generated structure indicates an actual source grammar problem.

---

72. Parser Generator Independence

The semantic meaning of reachability MUST NOT depend on an ANTLR implementation accident.

ANTLR-specific generated helper rules are implementation details.

The canonical source-language reachability graph is built from declared grammar rules before generated parser output.

---

73. Rust Parser Integration

The repository has an independent executable parser in:

src/parser.rs

This parser uses its own parsing functions and a precedence-driven expression implementation.

Therefore the validator MUST distinguish:

ANTLR grammar reachability

from:

Rust parser implementation reachability

The two must eventually agree for stable syntax.

---

74. ANTLR/Rust Conformance

For every stable source construct:

ANTLR grammar accepts
        ↕
Rust parser accepts

and:

ANTLR grammar rejects
        ↕
Rust parser rejects

for the defined conformance corpus, except where an intentional compatibility deviation is documented.

An ANTLR rule that is unreachable but has a corresponding Rust parser method is evidence of frontend drift.

It MUST be reported.

---

75. Rust Parser-Only Syntax

If "src/parser.rs" accepts a construct that has no canonical grammar rule, report:

ZGRM-RCH-030
RUST_PARSER_SYNTAX_WITHOUT_CANONICAL_GRAMMAR

This is not fixed by marking the grammar rule unreachable.

The source language must have one authoritative specification path.

---

76. Grammar-Only Syntax

If a grammar rule is reachable but "src/parser.rs" has no corresponding parsing implementation, report:

ZGRM-RCH-031
GRAMMAR_SYNTAX_WITHOUT_RUST_PARSER_SUPPORT

unless the repository explicitly declares the ANTLR parser as the executable frontend for that feature.

---

77. AST-Only Syntax

An AST variant without a canonical grammar mapping MUST be independently reported.

For example, "src/ast/mod.rs" contains numerous Zamani-native and advanced constructs.

The validator must not assume that every AST enum variant is currently stable source syntax.

---

78. IR-Only Constructs

IR constructs may exist for backend or optimization purposes without direct source syntax.

They must not be used to create grammar reachability.

For example, an IR instruction such as:

Unreachable

does not require a source-level grammar rule named "unreachable".

IR reachability and grammar reachability are separate graphs.

---

79. Canonical Quantum IR

The validator MUST preserve:

source
 ↓
AST
 ↓
semantic analysis
 ↓
quantum::ir

No unreachable-rule exception may be justified by introducing a separate frontend quantum IR.

---

80. Source Spans

Every reported unreachable rule MUST have a stable source location whenever the source grammar is available.

At minimum:

file
line
column
rule name

For alternatives:

alternative span

must be reported.

Generated rules should instead report generated provenance where source spans are unavailable.

---

81. Diagnostics Must Be Scalable

The validator MUST NOT allocate an unbounded diagnostic structure merely because the grammar is large.

Implementation may use:

- streaming diagnostics;
- deterministic sorting buffers;
- configurable diagnostic budgets;
- incremental results;
- compact graph representations.

If diagnostics are truncated because of a resource budget, the validator MUST say so.

---

82. No Silent Diagnostic Truncation

A validator MUST NOT silently emit:

first N errors

and claim that no other unreachable rules exist.

If diagnostics are capped, report:

VALIDATION_RESOURCE_EXHAUSTED

or:

DIAGNOSTIC_LIMIT_REACHED

with the number of suppressed findings.

---

83. Algorithmic Requirements

A production implementation SHOULD use:

repository inventory
        ↓
grammar identity resolution
        ↓
import graph
        ↓
rule definition index
        ↓
rule reference index
        ↓
nullability
        ↓
FIRST/FOLLOW as needed
        ↓
canonical entry-point closure
        ↓
classification
        ↓
alternative reachability
        ↓
cross-layer conformance

For ordinary graph reachability, the implementation SHOULD use an iterative traversal rather than recursive Rust function calls.

This avoids unnecessary call-stack dependence for very large grammar graphs.

---

84. No Artificial Traversal Depth

The implementation MUST NOT use a semantic rule such as:

if depth > 1024 then unreachable

or:

if depth > 4096 then stop

Such a threshold changes correctness.

If a traversal budget is needed, it must be external:

ValidationBudget {
    max_work: ...
}

and exhaustion must produce a resource diagnostic.

---

85. Iterative Graph Traversal

Production Rust validation SHOULD conceptually use:

worklist = [program]

while worklist is not empty:
    rule = pop(worklist)

    if rule already visited:
        continue

    mark rule reachable

    enqueue all resolved rule references

This permits arbitrary graph depth subject to available memory and configured validation resources.

---

86. Strongly Connected Components

For recursion-aware analysis, the validator SHOULD use a graph algorithm such as Tarjan or Kosaraju implemented in safe Rust.

No unsafe memory manipulation is permitted.

SCC computation MUST remain deterministic.

---

87. Nullability Fixed Point

Nullability must be calculated to a fixed point.

Conceptually:

nullable = empty

repeat
    changed = false

    for each rule:
        if an alternative can derive ε:
            if rule not nullable:
                add rule
                changed = true

until changed == false

The implementation MUST NOT terminate based on an arbitrary iteration count.

Termination is guaranteed by the finite set of discovered grammar rules for the current repository snapshot.

---

88. FIRST Analysis

FIRST analysis must similarly converge to a fixed point.

For scalable grammar size, the implementation SHOULD avoid repeated whole-repository rescans where dependency tracking can be used.

Results MUST be deterministic.

---

89. Alternative Reachability

Each alternative should be assigned:

REACHABLE
DEAD
UNKNOWN_RESOURCE_LIMIT
GENERATED
TEST_ONLY
DEPRECATED

"UNKNOWN_RESOURCE_LIMIT" must never be treated as proof of deadness.

---

90. Witness Generation

Where practical, the validator SHOULD produce a witness for reachability.

For example:

program
→ sourceUnit
→ sourceElement
→ declarationElement
→ declaration
→ functionDeclaration

For dead alternatives, it SHOULD provide the reason no valid token sequence can select the alternative.

A proof without a witness may still be accepted where exact witness generation is computationally expensive, provided the diagnostic clearly identifies the analysis basis.

---

91. False Positives Are Production Defects

The validator must not report intentionally standalone rules as orphaned.

Therefore the classification model is mandatory.

A validator that simply computes:

all rules - reachable rules

and labels everything as an error is incorrect for a modular grammar repository.

---

92. False Negatives Are Also Production Defects

Conversely, silently ignoring all unreachable rules because the grammar is modular is incorrect.

An accidentally orphaned quantum, HDL, classical, or resource rule could otherwise remain invisible.

Every non-reachable rule must receive an explicit classification.

---

93. Public Rule Inventory

The validator SHOULD produce an inventory:

Rule| Grammar| Reachability| Entry point| Owner| AST| Semantic| IR
"program"| "Zamani"| reachable| "program"| root| Program| yes| yes
"sourceUnit"| "ZamaniParser"| reachable| "program"| parser| Program| yes| yes
"sourceElement"| "ZamaniParser"| reachable| "program"| parser| Program| yes| yes
"universalQuantum"| "ZamaniParser"| standalone/orphan pending explicit classification| tooling| parser| declared contract| downstream| "quantum::ir"

The exact inventory may be generated.

---

94. Current "ZamaniParser.g4" Reachability Finding

Based on the current parser composition, the following 23 rules are defined as universal integration contracts:

universalDeclaration
universalStatement
universalExpression
universalType
universalBlock
universalFunction
universalModule
universalEffect
universalResource
universalHardware
universalQuantum
universalHDL
universalDistributed
universalAI
universalData
universalNetworking
universalSecurity
universalCompilation
universalExecution
universalInteroperability
universalDialect
universalMacro
universalMetaprogramming

They are not currently reached by "sourceElement".

This is intentional-looking architecture, but intent must be made machine-verifiable.

Therefore the production validator MUST require one of:

explicit standalone classification

or:

explicit canonical integration

for each rule.

It MUST NOT silently ignore them.

---

95. What Must Not Be Done to Fix Those Rules

Do NOT solve their orphan status by blindly changing:

sourceElement

to:

sourceElement
    : ...
    | universalDeclaration
    | universalStatement
    | universalExpression
    | ...

That would create redundant and potentially ambiguous language entry paths.

Likewise, do not delete them merely to silence the validator.

Their intended role must first be established.

---

96. Correct Integration Pattern

The preferred architecture is:

canonical source syntax
        |
        v
domain dispatcher
        |
        v
domain rule
        |
        v
AST

Universal integration rules should exist only when they provide a real consumer-facing contract.

For example:

universalQuantum

may be useful to tooling that needs to parse a quantum declaration fragment.

If so:

STANDALONE

is correct.

If it is intended to be source syntax, it must be reachable through the canonical source grammar.

---

97. No Duplicate Entry Paths

The validator MUST detect:

sourceElement → quantumElement → quantumDeclaration

and:

sourceElement → universalQuantum → quantumDeclaration

if both expose the same syntax and produce ambiguous duplicate ownership.

The presence of a universal integration rule must not create a second canonical route to the same construct unless explicitly designed.

---

98. Domain Dispatcher Requirement

Every canonical domain grammar must have a resolvable dispatcher if the parser imports it.

Conceptually:

Quantum
   |
   +--> quantumDeclaration
   +--> quantumStatement
   +--> quantumExpression

The exact file name is not prescribed by this document.

The grammar identity and composition contract are what matter.

---

99. Leaf Grammar Requirement

A leaf grammar is not canonical merely because it exists.

For example:

grammar/hdl/memories.g4

may contain valid HDL syntax.

It becomes part of the canonical source language only through the HDL composition chain.

This protects modularity.

---

100. Empty Directories

An empty conceptual directory is not itself an unreachable-rule failure.

However, if a canonical parser import expects a dispatcher grammar that does not exist, that is:

UNRESOLVED_GRAMMAR_DEPENDENCY

The validator must report the actual dependency problem.

---

101. File Naming

Existing filenames MUST NOT be renamed merely to satisfy unreachable-rule validation.

In particular:

grammar/Zamani.g4
grammar/Zamani-Grammar.md
grammar/grammar.md
grammar/DESIGN.md
grammar/README.md
grammar/antlr/ZamaniParser.g4
grammar/antlr/ZamaniLexer.g4

remain valid architectural filenames.

The validator must adapt to existing repository ownership.

---

102. The Existing "grammar/antlr/" Directory

The current repository contains:

grammar/antlr/ZamaniParser.g4
grammar/antlr/ZamaniLexer.g4

This is not inherently a second root grammar.

The architecture explicitly defines:

grammar/Zamani.g4
    ↓
ZamaniParser
ZamaniLexer

The validator MUST therefore treat "grammar/Zamani.g4" as the canonical complete-program root and "grammar/antlr/" as the canonical imported composition layer.

It must not invent a competing root.

---

103. Root Grammar Reachability

"grammar/Zamani.g4" contains only:

program
    : sourceUnit EOF
    ;

This is correct for a composition root.

The validator MUST NOT complain that the root contains few rules.

The production criterion is:

does the root reach the complete parser composition?

not:

does the root file contain every rule?

---

104. EOF Reachability

The canonical program rule MUST reach:

EOF

A grammar that reaches a source rule but does not require complete input consumption is incomplete for full-program parsing.

The validator MUST detect:

root accepts prefix but does not consume complete source

as:

ZGRM-RCH-040
INCOMPLETE_ROOT_CONSUMPTION

---

105. Entry-Point Reachability

The validator MUST verify:

program
 → sourceUnit
 → sourceElement*
 → EOF

or the exact equivalent specified by the canonical grammar.

It MUST NOT assume rule names if the normative specification changes them.

The invariant is complete source consumption, not the spelling of the rule name.

---

106. Grammar Rule Status and "grammar.md"

Unreachable status MUST be reflected in conformance reporting.

Suggested status:

REACHABLE + fully implemented
    → IMPLEMENTED

REACHABLE + partial downstream support
    → PARTIALLY IMPLEMENTED

STANDALONE
    → IMPLEMENTATION FRAGMENT / TOOLING

EXPERIMENTAL
    → PLANNED / EXPERIMENTAL

DEPRECATED
    → DEPRECATED

ORPHANED
    → INVALID / CONFORMANCE FAILURE

UNRESOLVED_DEPENDENCY
    → INVALID / CONFORMANCE FAILURE

The exact final status vocabulary remains owned by "grammar/grammar.md".

---

107. Compatibility

A previously reachable rule may become unreachable only through an explicit language-version change.

Such a change MUST specify:

old status
new status
version
migration
compatibility behavior
diagnostics

A rule cannot accidentally disappear from the grammar because a dispatcher stopped importing it.

That would be a compatibility regression.

---

108. Versioned Reachability

Reachability is version-sensitive.

The validator MUST support:

language version
grammar version
dialect version
feature gates

A rule may be:

reachable in vN
deprecated in vN+1
removed in vN+2

This is valid only when documented.

---

109. Incremental Validation

The validator SHOULD maintain dependency information so that changes to:

grammar/quantum/operations.g4

do not require unrelated grammar analysis when dependency closure proves those areas unaffected.

However, incremental results MUST equal clean full-validation results.

Incremental mode MUST NOT introduce a different definition of reachability.

---

110. Change Invalidation

A grammar change MUST invalidate at least:

the changed grammar
its import dependents
its rule-reference dependents
its reachability dependents
its AST/semantic/IR conformance dependents where applicable

A rule's cached reachable status must never survive an invalidated dependency.

---

111. Repository Snapshot

Validation must operate against one coherent repository snapshot.

It MUST NOT combine:

Zamani.g4 from commit A
ZamaniParser.g4 from commit B
src/parser.rs from commit C

unless explicitly performing historical comparison.

This prevents false reachability conclusions.

---

112. Deterministic Snapshot Identity

A validation report SHOULD record:

repository revision
grammar version
language version
validator version
configuration
enabled dialects/features

This makes production validation reproducible.

---

113. No Network-Dependent Reachability

Grammar reachability MUST NOT depend on live:

- network services;
- hardware;
- cloud services;
- package registries;
- QPU availability;
- GPU availability;
- filesystem state outside the repository snapshot.

All grammar dependencies must be locally resolvable from the declared source set.

---

114. No Runtime-Dependent Reachability

A rule is not reachable or unreachable based on:

runtime state
environment variable
machine type
CPU count
GPU count
QPU availability
memory capacity

Those belong to later compilation/deployment capability analysis.

---

115. Security

The validator MUST treat grammar files as data.

It MUST NOT execute grammar actions.

It MUST NOT execute embedded source code.

It MUST NOT load arbitrary dynamic libraries.

It MUST NOT invoke unsafe parser actions.

The validator must be safe Rust.

---

116. Rust Safety

Validator implementation requirements:

Rust 1.97
Rust 1.97.1
Edition 2021

Production validator code MUST contain no:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

Where appropriate, the crate/module SHOULD enforce:

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

The validator must not require unsafe Rust for scalability.

---

117. Memory Scalability

For large grammars, the validator SHOULD:

- intern rule and grammar identities where useful;
- avoid duplicating complete source text in graph nodes;
- store compact IDs instead of repeated strings;
- use iterative traversal;
- release intermediate analysis structures when no longer needed;
- support incremental invalidation;
- stream diagnostics when appropriate.

These are implementation techniques.

They do not impose language limits.

---

118. Large Grammar Correctness

The validator must work for:

tiny grammar
small grammar
medium grammar
large grammar
very large grammar
future arbitrarily large grammar

subject only to actual available resources.

A larger number of rules must not change the meaning of reachability.

---

119. Program Size Is Not Grammar Reachability

A source program with:

one statement

and a source program with:

arbitrarily many statements

must use the same grammar reachability model.

"sourceUnit*" is not converted into a finite language limit.

---

120. Quantum Scale

Quantum grammar reachability must not depend on:

number of qubits
number of gates
number of registers
number of measurements
number of devices

A rule is reachable because it is grammatically composed, not because the current machine has enough qubits.

---

121. Tensor Scale

Tensor syntax must not become unreachable because:

rank is large
dimension is large
shape is large

Resource feasibility belongs downstream.

---

122. HDL Scale

HDL rules must not become unreachable because:

bus width is large
register count is large
pipeline depth is large
memory depth is large
module count is large

The grammar remains parameterized.

---

123. Distributed Scale

Distributed syntax must not use:

MAX_NODES
MAX_PROCESSES
MAX_TASKS
MAX_CHANNELS

as grammar limits.

Reachability is independent of deployment size.

---

124. Diagnostic Severity

Recommended severity:

Condition| Severity
canonical root unreachable| ERROR
stable orphaned rule| ERROR
unresolved canonical import| ERROR
dead stable alternative| ERROR
duplicate rule| ERROR
duplicate grammar identity| ERROR
unclassified standalone rule| ERROR
reachable deprecated rule| WARNING
experimental unreachable rule| INFO
test-only rule| INFO
generated support rule| INFO
foreign interoperability rule| INFO

Severity does not change semantic classification.

---

125. Production Gate

The unreachable-rule validator passes only when:

all canonical imports resolve
AND
canonical root resolves
AND
canonical root consumes complete source
AND
every canonical rule is reachable or explicitly classified
AND
no stable orphan exists
AND
no dead stable alternative exists
AND
no duplicate canonical rule exists
AND
no unresolved canonical rule reference exists
AND
no hidden grammar composition exists
AND
no unexplained AST grammar mismatch exists
AND
no unexplained Rust parser grammar mismatch exists
AND
no validation resource exhaustion occurred

---

126. Production Failure Conditions

The validator MUST fail production validation for:

ORPHANED_GRAMMAR_RULE
UNRESOLVED_GRAMMAR_DEPENDENCY
UNRESOLVED_RULE_REFERENCE
DUPLICATE_RULE_DEFINITION
DUPLICATE_GRAMMAR_IDENTITY
DEAD_GRAMMAR_ALTERNATIVE
INCOMPLETE_ROOT_CONSUMPTION
UNCLASSIFIED_STANDALONE_RULE
CANONICAL_GRAMMAR_CYCLE_WITHOUT_PROGRESS
GRAMMAR_WITHOUT_REQUIRED_AST_MAPPING
STABLE_GRAMMAR_WITHOUT_REQUIRED_SEMANTIC_MAPPING

where the relevant downstream contract applies.

---

127. Production Non-Failures

The validator MUST NOT fail merely because a rule is:

STANDALONE
TEST_ONLY
GENERATED_SUPPORT
EXPERIMENTAL
DEPRECATED
UNIMPLEMENTED

provided the classification is explicit and valid.

---

128. Stable Feature Completion

A grammar rule is not a completed feature simply because it is reachable.

A stable feature requires:

Specification
    ↓
Lexical contract
    ↓
Grammar
    ↓
Reachability
    ↓
AST
    ↓
Semantic analysis
    ↓
Canonical IR
    ↓
Compiler integration
    ↓
Runtime/backend integration where applicable
    ↓
Tests

For quantum:

quantum syntax
    ↓
domain-neutral AST
    ↓
semantic quantum representation
    ↓
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

---

129. Test Requirements

The validation test suite MUST include:

reachable rule
unreachable rule
standalone rule
test-only rule
generated rule
deprecated rule
experimental rule
missing import
missing rule
duplicate rule
duplicate grammar
dead alternative
nullable rule
nullable cycle
recursive cycle
cross-grammar reachability
dialect reachability
foreign grammar isolation
AST mismatch
Rust parser mismatch

---

130. Boundary Tests

Boundary tests MUST include:

empty grammar
single rule
single terminal
deep rule chain
wide rule graph
large import graph
large SCC
large nullable closure
large alternative set
large diagnostic set
large source unit

No artificial threshold may be encoded as a language rule.

---

131. Scalability Tests

Scalability tests MUST verify that increasing:

number of rules
number of grammars
number of imports
number of alternatives
number of nested constructs
number of domains
number of dialects

does not alter the correctness of reachability.

Where validation eventually exhausts resources, the result must be:

VALIDATION_RESOURCE_EXHAUSTED

not a false reachability result.

---

132. Determinism Tests

Run the same repository snapshot multiple times.

Expected:

same reachable set
same unreachable set
same classifications
same diagnostics
same diagnostic order

No randomness is permitted.

---

133. Mutation Tests

The validator SHOULD use mutation tests such as:

remove one import
remove one rule reference
rename one rule
duplicate one rule
delete one dispatcher
disable one dialect
make one alternative subsume another

Each mutation must produce the expected diagnostic.

---

134. Regression Tests for Current Architecture

The repository MUST preserve tests proving:

program → sourceUnit → sourceElement

is reachable.

It MUST also test the current universal rules:

universalDeclaration
...
universalMetaprogramming

as explicitly classified integration contracts rather than silently ignoring their current lack of canonical source reachability.

---

135. Regression Test for Missing Composition Grammar

If a required grammar dispatcher such as:

Quantum

cannot be resolved, the test must expect:

UNRESOLVED_GRAMMAR_DEPENDENCY

not:

UNREACHABLE_QUANTUM_RULE

This distinction is essential.

---

136. Regression Test for Leaf Grammar

A leaf file such as:

grammar/hdl/memories.g4

must not automatically become a canonical source rule merely because it exists.

Only the actual composition graph determines canonical reachability.

---

137. Regression Test for Standalone Universal Rule

For example:

universalQuantum

must pass validation as an unreachable standalone rule only when its explicit standalone classification exists.

Without classification:

ORPHANED_GRAMMAR_RULE

must be produced.

---

138. Regression Test for Program Prefix Acceptance

A grammar such as:

program
    : sourceUnit
    ;

without complete input consumption must fail the root-consumption test if the language requires complete source parsing.

The existing:

program
    : sourceUnit EOF
    ;

satisfies this specific invariant.

---

139. Regression Test for Right Recursion

A valid right-recursive construct such as:

assignmentExpression

must not be falsely reported as left recursion or unreachable.

---

140. Regression Test for Prefix Recursion

A valid prefix structure such as:

prefixOperator prefixExpression

must remain reachable and valid when "prefixOperator" is guaranteed to consume input.

If that operator becomes nullable, the validator must detect the resulting progress problem.

---

141. Regression Test for Postfix Composition

Postfix constructs must support arbitrary suffix chains without requiring left recursion.

For example:

a()
a()[i]
a()[i].field
a()[i].field(x)

The number of suffixes is not bounded by the validator.

---

142. Regression Test for Quantum Extensibility

Quantum reachability tests must permit:

H
X
custom_gate
vendor.operation
operation(parameter)

without requiring a fixed list of operations.

The validator is checking grammar connectivity, not hardware gate support.

---

143. Regression Test for Hardware Independence

The validator must produce the same grammar reachability result regardless of:

CPU count
GPU count
FPGA availability
QPU availability
memory capacity
cluster size

---

144. Regression Test for Resource Requirements

A grammar construct such as:

requires qubits >= n

must remain reachable even if no current target satisfies the requirement.

The target capability decision belongs downstream.

---

145. Regression Test for No Hard-Coded Limits

The validator MUST cooperate with:

grammar/validation/hardcoding-audit.md
grammar/validation/scalability-rules.md

and must not classify a grammar rule based on forbidden universal limits such as:

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

---

146. Cross-Validation With Other Validation Contracts

This document integrates with:

grammar/validation/grammar-validator.md
grammar/validation/ambiguity-rules.md
grammar/validation/ambiguity.md
grammar/validation/left-recursion.md
grammar/validation/hardcoding-audit.md
grammar/validation/scalability-rules.md
grammar/validation/semantic-boundaries.md
grammar/validation/compatibility-rules.md
grammar/validation/naming-rules.md

Ownership MUST remain separated.

This document owns:

reachability
orphan classification
rule/alternative reachability
grammar dependency resolution

It does not duplicate the complete policies of those documents.

---

147. Interaction With Ambiguity Validation

An alternative may be:

reachable

but still:

ambiguous

Therefore:

reachable ≠ unambiguous

The validator must run ambiguity analysis separately.

---

148. Interaction With Left-Recursion Validation

A recursive rule may be:

reachable

and:

valid right recursion

or:

reachable

and:

invalid left recursion

These are distinct findings.

---

149. Interaction With Hard-Coding Validation

A reachable rule may still violate scalability.

For example:

registerWidth
    : '32'
    ;

could be reachable but architecturally invalid if it establishes a universal hardware limit.

That is a hard-coding failure, not an unreachable-rule failure.

---

150. Interaction With Semantic Boundaries

A rule may be reachable but incorrectly own:

routing
scheduling
calibration
QEC
hardware selection
runtime execution

That is a semantic-boundary violation.

The reachability validator should link to the relevant finding rather than duplicating it.

---

151. Interaction With Compatibility

A rule may be intentionally unreachable in the current language version because it is deprecated or removed.

That is valid only with compatibility metadata.

---

152. Interaction With Naming

Naming validation ensures that:

rule identity
grammar identity
feature identity
AST identity
semantic identity
IR identity

remain stable.

This document relies on those identities but does not redefine naming rules.

---

153. Feature Manifest Integration

Where the repository introduces feature manifests, each feature SHOULD contain:

grammar_rules
entry_points
reachability_status
owner
ast_nodes
semantic_nodes
ir_nodes
tests
compatibility

The validator can then compare declared reachability with observed reachability.

Mismatch is a conformance failure.

---

154. No Hidden Grammar Features

A feature is not stable merely because:

a `.g4` file exists

or:

a grammar rule exists

or:

a parser method exists

or:

an AST enum variant exists

The canonical pipeline must agree.

---

155. Rule Lifecycle

A grammar rule SHOULD progress through:

PROPOSED
    ↓
EXPERIMENTAL
    ↓
INTEGRATED
    ↓
REACHABLE
    ↓
SEMANTICALLY IMPLEMENTED
    ↓
IR-INTEGRATED
    ↓
TESTED
    ↓
STABLE

A rule can remain experimental and unreachable from stable source syntax during development.

That is preferable to exposing incomplete syntax.

---

156. Production Completion Criteria for This File

"grammar/validation/unreachable-rules.md" is complete when it defines:

- canonical entry point;
- grammar import resolution;
- rule graph construction;
- rule reference resolution;
- reachability closure;
- alternative reachability;
- classification;
- standalone rules;
- test-only rules;
- generated rules;
- deprecated rules;
- experimental rules;
- unimplemented rules;
- orphan rules;
- unresolved dependencies;
- nullable rules;
- recursive cycles;
- cross-grammar traversal;
- dialect handling;
- interoperability handling;
- AST integration;
- semantic integration;
- IR integration;
- quantum integration;
- Rust parser integration;
- ANTLR integration;
- deterministic diagnostics;
- resource budgets;
- scalability;
- POCO-REAF;
- hard-coding separation;
- compatibility;
- testing;
- production gates;
- Rust 1.97/1.97.1 safe-Rust requirements.

---

157. Required Validator Output Model

The implementation SHOULD expose findings equivalent to:

GrammarReachabilityFinding {
    code,
    severity,
    grammar,
    rule,
    alternative,
    classification,
    source_span,
    root,
    dependency_path,
    owner,
    ast_status,
    semantic_status,
    ir_status,
    message,
    remediation
}

The exact Rust type may differ.

The information must remain available.

---

158. Required Result Categories

The validator MUST distinguish at least:

Reachable
Standalone
TestOnly
GeneratedSupport
Deprecated
Experimental
Unimplemented
Orphaned
UnresolvedDependency
DeadAlternative
DuplicateRule
DuplicateGrammar
UnresolvedRuleReference
IncompleteRootConsumption
ValidationResourceExhausted

---

159. Stable Result Semantics

A successful validation result means:

Every canonical stable rule is accounted for.

It does NOT mean:

Every rule in the repository is reachable.

The repository intentionally contains:

- fragments;
- historical material;
- interoperability grammars;
- generated grammars;
- tests;
- experimental features.

Production correctness depends on classification.

---

160. Final Canonical Model

The complete validation model is:

Repository Snapshot
        |
        v
Grammar Inventory
        |
        v
Grammar Identity Resolution
        |
        v
Import Graph
        |
        v
Rule Definition Index
        |
        v
Rule Reference Graph
        |
        +----------------------+
        |                      |
        v                      v
   Nullability             Token/Rule
   Analysis                Resolution
        |                      |
        +----------+-----------+
                   |
                   v
          Canonical Entry Points
                   |
                   v
          Reachability Closure
                   |
        +----------+-----------+
        |          |            |
        v          v            v
    Reachable  Standalone    Orphaned
        |          |            |
        |          |            v
        |          |       Production Error
        |          |
        |          +--> Explicitly Classified
        |
        v
 Alternative Reachability
        |
        v
 AST Conformance
        |
        v
 Semantic Conformance
        |
        v
 IR Conformance
        |
        v
 Cross-Layer Conformance
        |
        v
 Determinism / Scalability / Safety
        |
        v
 Production Readiness

---

161. Fundamental Invariants

The following invariants are mandatory.

Invariant 1 — One canonical root

grammar/Zamani.g4

is the complete-program root.

Invariant 2 — Complete input

A successful complete-program parse consumes:

EOF

Invariant 3 — No silent orphan

Every non-reachable rule has an explicit classification.

Invariant 4 — Missing dependency is not unreachable

An unresolved grammar import must be reported as an unresolved dependency.

Invariant 5 — No automatic wiring

The validator must never modify grammar composition to make a rule reachable.

Invariant 6 — No artificial limits

Reachability is not bounded by machine-size constants.

Invariant 7 — Resource exhaustion is distinct

Validator resource exhaustion must never be reported as language invalidity.

Invariant 8 — Stable syntax has a complete pipeline

grammar
→ lexer
→ parser
→ AST
→ semantic analysis
→ IR
→ implementation
→ tests

Invariant 9 — Quantum has one canonical IR boundary

quantum::ir

remains the canonical quantum semantic/IR boundary.

Invariant 10 — Target realization is downstream

CPU/GPU/FPGA/QPU/HPC/distributed/cloud/future hardware availability does not determine grammar reachability.

Invariant 11 — Deterministic results

Identical validation inputs produce identical findings.

Invariant 12 — Safe Rust

The validator requires no "unsafe" Rust.

---

162. Production Acceptance Checklist

Before accepting the validator as production-ready:

[ ] grammar/Zamani.g4 resolves
[ ] grammar/antlr/ZamaniParser.g4 resolves
[ ] grammar/antlr/ZamaniLexer.g4 resolves
[ ] all canonical imports resolve
[ ] all canonical grammar identities are unique
[ ] program is reachable
[ ] sourceUnit is reachable
[ ] sourceElement is reachable
[ ] EOF is enforced
[ ] every canonical rule has a classification
[ ] every standalone rule is explicit
[ ] every test-only rule is isolated
[ ] every generated rule has provenance
[ ] every deprecated rule has compatibility metadata
[ ] every experimental rule has feature metadata
[ ] no stable orphaned rule exists
[ ] no unresolved canonical rule reference exists
[ ] no dead stable alternative exists
[ ] no duplicate canonical rule exists
[ ] no nullable non-progress cycle exists
[ ] left recursion is validated separately
[ ] ambiguity is validated separately
[ ] AST mappings are checked
[ ] semantic mappings are checked
[ ] IR mappings are checked
[ ] quantum mappings terminate at quantum::ir
[ ] Rust parser conformance is checked
[ ] ANTLR/Rust divergence is reported
[ ] dialect reachability is deterministic
[ ] interoperability grammars are isolated
[ ] no hardware availability affects reachability
[ ] no artificial scale limit exists
[ ] resource budgets are external
[ ] resource exhaustion is distinguishable
[ ] diagnostic ordering is deterministic
[ ] validator is safe Rust
[ ] Rust 1.97/1.97.1 is supported
[ ] no unsafe implementation is required
[ ] positive tests exist
[ ] negative tests exist
[ ] boundary tests exist
[ ] scalability tests exist
[ ] determinism tests exist
[ ] compatibility tests exist
[ ] mutation tests exist

---

163. Final Rule

The production definition of an unreachable grammar rule is:

«A parser grammar rule is unreachable only when it has no valid path from any explicitly enabled canonical or intentionally declared standalone entry point through the resolved grammar composition graph, after grammar identity, imports, rule references, dialect state, and generated/support classifications have been resolved.»

Everything else must be classified accurately.

In particular:

missing import
≠ unreachable rule

unimplemented feature
≠ unreachable rule

experimental feature
≠ unreachable rule

standalone tooling rule
≠ unreachable rule

foreign interoperability grammar
≠ unreachable Zamani rule

deprecated grammar
≠ automatically invalid grammar

recursive grammar
≠ unreachable grammar

semantic rejection
≠ unreachable grammar

hardware incompatibility
≠ unreachable grammar

The validator's job is to expose the actual composition state without changing the language.

The resulting architecture remains:

Zamani source
      |
      v
grammar/Zamani.g4
      |
      v
ZamaniParser / ZamaniLexer
      |
      v
modular grammar composition
      |
      v
lexer / parser
      |
      v
domain-neutral AST
      |
      v
semantic analysis
      |
      v
canonical semantic model
      |
      +-------------------+
      |                   |
      v                   v
 classical             quantum::ir
      |                   |
      +---------+---------+
                |
                v
              IR
                |
                v
        optimization/lowering
                |
        +-------+--------+
        |       |        |
      routing scheduling resilience
                        |
                        v
                       ZQN
                        |
                        v
                       HAL
                        |
                        v
                target realization

This preserves the repository's intended separation of syntax, semantics, resources, capabilities, compilation, scheduling, routing, resilience, QEC, ZQN, HAL, and runtime while allowing the grammar to scale from the smallest computation to arbitrarily large computations subject to actual available resources.

"unreachable-rules.md" therefore becomes a validation contract, not another grammar authority and not another implementation layer.