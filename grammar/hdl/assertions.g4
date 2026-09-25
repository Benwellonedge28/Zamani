/*

* ============================================================================
* Zamani Universal Computing Language
* ============================================================================
* 
* FILE
* ---
* grammar/hdl/assertions.g4
* 
* STATUS
* ---
* CANONICAL PRODUCTION HDL ASSERTION / PROPERTY GRAMMAR
* 
* OWNER
* ---
* HDL verification assertion and property SOURCE SYNTAX.
* 
* GRAMMAR TECHNOLOGY
* ---
* ANTLR4 parser grammar.
* 
* IMPLEMENTATION BASELINE
* ---
* Rust 1.97 / Rust 1.97.1
* Rust 2021
* Safe Rust only.
* 
* ============================================================================
* 1. AUTHORITY
* ============================================================================
* 
* This file is the SINGLE AUTHORITATIVE SYNTAX OWNER for HDL assertions and
* HDL verification-property declarations.
* 
* It is subordinate to:
* 
* grammar/DESIGN.md
* grammar/spec/hdl.md
* 
* and is composed by:
* 
* grammar/hdl/hdl.g4
* 
* The dependency direction is:
* 
* specification
*      |
*      v
* HDL assertion/property syntax
*      |
*      v
* domain-neutral frontend AST
*      |
*      v
* semantic verification model
*      |
*      v
* canonical HDL/hardware semantic representation
*      |
*      v
* verification / synthesis / simulation / timing / target lowering
* 
* This file MUST NOT become an independent language.
* 
* ============================================================================
* 2. PURPOSE
* ============================================================================
* 
* This grammar provides portable source syntax for expressing properties of
* HDL/hardware behavior.
* 
* It supports the distinction between:
* 
* assertion
*     A property that the implementation is required to satisfy.
* 
* assumption
*     A property describing an environmental/precondition assumption.
* 
* coverage
*     A property/event whose occurrence is to be observed for coverage.
* 
* property declaration
*     A named reusable verification property.
* 
* The syntax records verification intent.
* 
* It does NOT perform verification.
* 
* ============================================================================
* 3. IMPORTANT CURRENT-REPOSITORY CORRECTION
* ============================================================================
* 
* The repository currently contains several assertion implementations:
* 
* grammar/statements/assertions.g4
* grammar/hdl/hdl.g4
* grammar/hdl/sequential.g4
* grammar/hdl/combinational.g4
* 
* These MUST NOT remain independent owners of the same HDL assertion syntax.
* 
* The intended ownership is:
* 
* grammar/hdl/assertions.g4
*     |
*     +--> hdlAssertion
*     +--> hdlAssumption
*     +--> hdlCoverage
*     +--> hdlPropertyDeclaration
*     +--> reusable HDL verification-property syntax
* 
* The surrounding HDL grammars only COMPOSE these rules.
* 
* In particular:
* 
* sequential.g4
* combinational.g4
* hardware-modules.g4
* hdl.g4
* 
* MUST NOT independently redefine HDL assertion syntax.
* 
* ============================================================================
* 4. WHAT THIS FILE OWNS
* ============================================================================
* 
* THIS FILE OWNS:
* 
* hdlAssertion
* hdlAssertionCore
* hdlAssumption
* hdlCoverage
* hdlPropertyDeclaration
* hdlPropertyExpression
* hdlVerificationClock
* hdlVerificationDisable
* hdlVerificationQualifier
* hdlVerificationAction
* hdlVerificationArgumentList
* hdlVerificationNamedArgument
* 
* and only the syntax needed to compose those constructs.
* 
* ============================================================================
* 5. WHAT THIS FILE DOES NOT OWN
* ============================================================================
* 
* This file does NOT own:
* 
* lexer rules
* keyword definitions
* punctuation
* identifiers
* numeric literals
* strings
* general expressions
* general types
* modules
* ports
* signals
* nets
* registers
* memories
* clocks
* resets
* timing declarations
* processes
* combinational behavior
* sequential behavior
* state machines
* pipelines
* generation
* synthesis
* simulation
* formal verification algorithms
* theorem proving
* SAT/SMT solving
* model checking
* temporal-property evaluation
* timing closure
* clock-tree construction
* CDC implementation
* physical placement
* physical routing
* vendor primitives
* FPGA resources
* ASIC cells
* CPU selection
* GPU selection
* QPU selection
* quantum::ir
* QEC
* ZQN
* HAL
* runtime execution
* 
* ============================================================================
* 6. CANONICAL LEXER
* ============================================================================
* 
* This parser consumes:
* 
* tokenVocab = ZamaniLexer;
* 
* There is exactly ONE canonical Zamani lexer.
* 
* This file MUST NOT define:
* 
* lexer grammar
* tokens {}
* token aliases
* parser-local lexical rules
* a hardware-specific lexer
* 
* The current canonical keyword vocabulary already provides:
* 
* ASSERT
* PROPERTY
* 
* Therefore this grammar uses those canonical token names.
* 
* ============================================================================
* 7. ASSUME / COVER TOKEN COMPATIBILITY
* ============================================================================
* 
* The current grammar/lexer/keywords.g4 does not currently define canonical
* ASSUME and COVER tokens.
* 
* The previous HDL grammar nevertheless attempted to consume:
* 
* K_ASSUME
* K_COVER
* 
* Those K_* names are not an acceptable second token vocabulary.
* 
* This file therefore does NOT invent K_ASSUME or K_COVER.
* 
* To make ASSUME and COVER first-class reserved HDL verification constructs,
* the canonical lexer MUST eventually add:
* 
* ASSUME : 'assume' ;
* COVER  : 'cover' ;
* 
* to:
* 
* grammar/lexer/keywords.g4
* 
* After that change this grammar's:
* 
* hdlAssumption
* hdlCoverage
* 
* productions can consume those canonical tokens.
* 
* Until that lexer change is made, the existing production syntax supported
* by this file remains "assert" and named "property" syntax.
* 
* The integration requirement is deliberately explicit rather than hiding a
* second lexical universe inside this grammar.
* 
* ============================================================================
* 8. ASSERTION MODEL
* ============================================================================
* 
* The grammar distinguishes:
* 
* immediate assertion
* 
* from:
* 
* property assertion
* 
* Immediate:
* 
* assert(condition);
* 
* Property:
* 
* assert property(condition);
* 
* A property may additionally contain:
* 
* clocking information
* disable conditions
* named properties
* verification arguments
* action/diagnostic metadata
* 
* The grammar records structure.
* 
* Semantic analysis determines what each construct means.
* 
* ============================================================================
* 9. BASIC ASSERTION
* ============================================================================
* 
* Canonical immediate assertion:
* 
* assert(condition);
* 
* Optional diagnostic expression:
* 
* assert(condition, "message");
* 
* Optional structured verification arguments:
* 
* assert(condition, message: "message", severity: level);
* 
* The semantic layer determines which named arguments are valid.
* 
* This grammar does not create a fixed universal list of verification
* backends or diagnostic systems.
* 
* ============================================================================
* 10. PROPERTY ASSERTION
* ============================================================================
* 
* Property assertions use the existing reserved:
* 
* PROPERTY
* 
* token.
* 
* Canonical shape:
* 
* assert property(condition);
* 
* Optional verification metadata:
* 
* assert property(condition, ...);
* 
* The condition itself remains an ordinary Zamani expression.
* 
* This avoids creating a second expression grammar solely for verification.
* 
* ============================================================================
* 11. NAMED PROPERTIES
* ============================================================================
* 
* A reusable property may be declared:
* 
* property name = expression;
* 
* The property body remains an expression.
* 
* A property may then be referenced by:
* 
* assert property(name);
* 
* Semantic analysis determines whether the referenced expression is a valid
* HDL verification property.
* 
* The grammar does not prove that it is.
* 
* ============================================================================
* 12. PROPERTY PARAMETERS
* ============================================================================
* 
* Parameterized properties are permitted:
* 
* property stable(x, y) = x == y;
* 
* The argument list is syntactic.
* 
* Semantic analysis determines:
* 
* parameter binding
* arity
* type compatibility
* ownership
* scope
* elaboration
* specialization
* 
* There is no parser-defined maximum number of property parameters.
* 
* ============================================================================
* 13. CLOCKING
* ============================================================================
* 
* Verification properties may be associated with logical clock/event intent.
* 
* Example:
* 
* assert property (
*     condition
* ) clock(clk);
* 
* The clock is represented as an expression/reference.
* 
* This grammar does NOT:
* 
* select a physical clock;
* select a PLL;
* select a clock tree;
* select a clock pin;
* impose a fixed frequency;
* impose a fixed clock count.
* 
* Ordinary clock declarations remain owned by:
* 
* grammar/hdl/clocks.g4
* 
* Clocking relationships remain owned by:
* 
* grammar/hdl/clocking.g4
* 
* Timing constraints remain owned by:
* 
* grammar/hdl/timing.g4
* 
* This file only consumes verification-specific clock association syntax.
* 
* ============================================================================
* 14. DISABLE CONDITIONS
* ============================================================================
* 
* A property may express a semantic disable condition:
* 
* assert property(condition) disable(condition2);
* 
* The grammar records both expressions.
* 
* Semantic analysis determines:
* 
* whether the disable condition is valid;
* when it takes effect;
* whether it is synchronous;
* whether it is compatible with the clocking model.
* 
* The grammar does not implement reset semantics.
* 
* ============================================================================
* 15. ASSUMPTIONS
* ============================================================================
* 
* Once the canonical lexer exposes:
* 
* ASSUME
* 
* the production form is:
* 
* assume property(condition);
* 
* and optionally:
* 
* assume property(condition) clock(clk);
* 
* and:
* 
* assume property(condition) disable(reset_condition);
* 
* The grammar deliberately keeps assumption semantics separate from
* assertions.
* 
* An assumption does not mean that the hardware is proven to satisfy the
* condition.
* 
* It constrains the verification environment/model.
* 
* ============================================================================
* 16. COVERAGE
* ============================================================================
* 
* Once the canonical lexer exposes:
* 
* COVER
* 
* the production form is:
* 
* cover property(condition);
* 
* Coverage asks whether a specified behavior/property can occur or is
* exercised according to the verification semantics.
* 
* It is not an assertion failure condition.
* 
* ============================================================================
* 17. NO FIXED TEMPORAL OPERATOR ENUMERATION
* ============================================================================
* 
* This grammar does NOT hard-code a finite list of temporal operators such as:
* 
* always
* eventually
* until
* next
* eventually_always
* throughout
* implication
* repetition
* 
* merely because one verification technology uses those names.
* 
* Such concepts may be represented by ordinary expressions/functions or by
* future language extensions.
* 
* If a future temporal operator becomes genuine Zamani language syntax, it
* must first receive:
* 
* specification
* lexical contract if necessary
* AST contract
* semantic contract
* IR/verification contract
* compatibility decision
* tests
* 
* This keeps the core grammar open to future verification technologies.
* 
* ============================================================================
* 18. PROPERTY QUALIFIERS
* ============================================================================
* 
* Verification metadata is represented structurally rather than by a
* hard-coded backend-specific property language.
* 
* A property assertion may contain:
* 
* clock(...)
* disable(...)
* metadata(...)
* 
* Future semantic qualifiers may be added through the same extensible
* mechanism without changing the meaning of existing assertion expressions.
* 
* ============================================================================
* 19. ACTION / DIAGNOSTIC DATA
* ============================================================================
* 
* Assertion diagnostics may be expressed as ordinary expressions.
* 
* Examples:
* 
* assert(condition, "invalid state");
* 
* assert(condition, diagnostic);
* 
* assert(condition, make_diagnostic(context));
* 
* This file does not define a fixed diagnostic object.
* 
* Semantic analysis determines the accepted value/type.
* 
* ============================================================================
* 20. NAMED VERIFICATION ARGUMENTS
* ============================================================================
* 
* A verification construct may carry named metadata:
* 
* assert(
*     condition,
*     severity: level,
*     message: "state mismatch"
* );
* 
* Generic named arguments are intentionally used instead of hard-coding:
* 
* severity;
* message;
* action;
* backend;
* solver;
* waveform;
* report;
* 
* as universal parser constructs.
* 
* This keeps the syntax extensible.
* 
* ============================================================================
* 21. GRAMMAR
* ============================================================================
* 
* The parser grammar below is deliberately compact.
* 
* General expressions remain external.
* 
* ============================================================================
  */

parser grammar HdlAssertions;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* 22. PUBLIC ASSERTION ENTRY
* ============================================================================
* 
* This is the canonical HDL assertion entry point.
* 
* Surrounding HDL grammars MUST reference:
* 
* hdlAssertion
* 
* and MUST NOT reproduce its alternatives.
* ============================================================================
  */

hdlAssertion
: hdlVerificationLabel?
ASSERT
hdlAssertionBody
SEMICOLON
;

/*

* ============================================================================
* 23. ASSERTION BODY
* ============================================================================
  */

hdlAssertionBody
: hdlImmediateAssertion
| hdlPropertyAssertion
;

/*

* ============================================================================
* 24. IMMEDIATE ASSERTION
* ============================================================================
* 
* assert(condition);
* 
* assert(condition, explanation);
* 
* assert(condition, named_argument: value);
* 
* assert(condition, explanation, named_argument: value, ...);
* 
* The semantic layer determines which optional values are valid.
* ============================================================================
  */

hdlImmediateAssertion
: LPAREN
hdlExpression
(
COMMA
hdlVerificationArgumentList
)?
RPAREN
;

/*

* ============================================================================
* 25. PROPERTY ASSERTION
* ============================================================================
* 
* assert property(condition);
* 
* assert property(condition) clock(clk);
* 
* assert property(condition) disable(reset);
* 
* assert property(condition)
*     clock(clk)
*     disable(reset);
* ============================================================================
  */

hdlPropertyAssertion
: PROPERTY
hdlPropertySpecification
hdlVerificationQualifier*
;

/*

* ============================================================================
* 26. PROPERTY SPECIFICATION
* ============================================================================
  */

hdlPropertySpecification
: LPAREN
hdlPropertyExpression
(
COMMA
hdlVerificationArgumentList
)?
RPAREN
| hdlPropertyReference
(
LPAREN
hdlVerificationArgumentList?
RPAREN
)?
;

/*

* ============================================================================
* 27. PROPERTY EXPRESSION
* ============================================================================
* 
* The property expression intentionally crosses into the canonical HDL
* expression boundary.
* 
* Temporal semantics, if any, are represented downstream.
* ============================================================================
  */

hdlPropertyExpression
: hdlExpression
;

/*

* ============================================================================
* 28. PROPERTY REFERENCE
* ============================================================================
* 
* Named property references use logical names.
* 
* They do not identify:
* 
* physical verification engines;
* vendor assertion libraries;
* solver IDs;
* hardware devices.
* ============================================================================
  */

hdlPropertyReference
: hdlQualifiedName
;

/*

* ============================================================================
* 29. PROPERTY DECLARATION
* ============================================================================
* 
* property stable(a, b) = a == b;
* 
* property ready_when_valid(v, r) = v && r;
* 
* No finite property count or parameter count is imposed.
* ============================================================================
  */

hdlPropertyDeclaration
: PROPERTY
identifier
hdlPropertyParameterList?
ASSIGN
hdlPropertyExpression
SEMICOLON
;

/*

* ============================================================================
* 30. PROPERTY PARAMETERS
* ============================================================================
  */

hdlPropertyParameterList
: LPAREN
hdlPropertyParameter*
RPAREN
;

hdlPropertyParameter
: identifier
(
COLON
hdlTypeExpression
)?
;

/*

* ============================================================================
* 31. VERIFICATION QUALIFIERS
* ============================================================================
* 
* These qualifiers are intentionally small and semantic.
* 
* clock(...)
* disable(...)
* 
* Additional qualifiers should only be introduced after specification and
* compatibility review.
* ============================================================================
  */

hdlVerificationQualifier
: hdlVerificationClock
| hdlVerificationDisable
;

/*

* ============================================================================
* 32. CLOCK QUALIFIER
* ============================================================================
  */

hdlVerificationClock
: identifier
LPAREN
hdlExpression
RPAREN
;

/*

* ============================================================================
* 33. DISABLE QUALIFIER
* ============================================================================
  */

hdlVerificationDisable
: identifier
LPAREN
hdlExpression
RPAREN
;

/*

* ============================================================================
* 34. WHY QUALIFIERS USE IDENTIFIERS
* ============================================================================
* 
* The current canonical lexer does not reserve every future verification
* qualifier.
* 
* This allows:
* 
* clock(...)
* disable(...)
* 
* to be recognized as syntactic qualifier shapes without introducing a
* second keyword vocabulary.
* 
* Semantic analysis MUST validate that the identifier is a permitted
* verification qualifier in that position.
* 
* If Zamani later makes a qualifier a reserved language keyword, that change
* belongs in grammar/lexer/keywords.g4 and the parser contract can migrate
* without changing the semantic model.
* 
* ============================================================================
* 35. VERIFICATION ARGUMENTS
* ============================================================================
  */

hdlVerificationArgumentList
: hdlVerificationArgument
(
COMMA
hdlVerificationArgument
)*
COMMA?
;

/*

* ============================================================================
* 36. VERIFICATION ARGUMENT
* ============================================================================
* 
* Both positional and named arguments are allowed.
* 
* Examples:
* 
* "message"
* 
* diagnostic
* 
* message: "state mismatch"
* 
* severity: level
* ============================================================================
  */

hdlVerificationArgument
: hdlExpression
| identifier
COLON
hdlExpression
;

/*

* ============================================================================
* 37. VERIFICATION LABEL
* ============================================================================
* 
* Optional source-level labels permit diagnostics and downstream verification
* tooling to identify a property without requiring target-specific IDs.
* 
* Example:
* 
* label: assert(condition);
* 
* The label remains source metadata.
* ============================================================================
  */

hdlVerificationLabel
: identifier
COLON
;

/*

* ============================================================================
* 38. ASSUMPTION CONTRACT
* ============================================================================
* 
* ASSUME is intentionally documented here as a future canonical lexical
* extension because the current Zamani lexer does not expose ASSUME.
* 
* When grammar/lexer/keywords.g4 adds:
* 
* ASSUME : 'assume' ;
* 
* this grammar MUST add:
* 
* hdlAssumption
* 
* to the HDL verification dispatcher.
* 
* Canonical target syntax:
* 
* assume property(condition);
* 
* ============================================================================
  */

/*

* hdlAssumption
* : hdlVerificationLabel?
*   ASSUME
*   PROPERTY
*   hdlPropertySpecification
*   hdlVerificationQualifier*
*   SEMICOLON
* ;

*/

/*

* ============================================================================
* 39. COVER CONTRACT
* ============================================================================
* 
* COVER is handled identically.
* 
* When grammar/lexer/keywords.g4 adds:
* 
* COVER : 'cover' ;
* 
* this grammar MUST add:
* 
* hdlCoverage
* 
* to the HDL verification dispatcher.
* 
* Canonical target syntax:
* 
* cover property(condition);
* 
* ============================================================================
  */

/*

* hdlCoverage
* : hdlVerificationLabel?
*   COVER
*   PROPERTY
*   hdlPropertySpecification
*   hdlVerificationQualifier*
*   SEMICOLON
* ;

*/

/*

* ============================================================================
* 40. SHARED EXPRESSION CONTRACT
* ============================================================================
* 
* This file does NOT define:
* 
* hdlExpression
* expression
* hdlTypeExpression
* identifier
* hdlQualifiedName
* 
* Those are composition dependencies.
* 
* The canonical HDL composition root must provide these rules.
* 
* In the eventual fully modular composition:
* 
* hdlExpression
*     ->
* canonical expression grammar
* 
* and:
* 
* hdlTypeExpression
*     ->
* canonical type grammar
* 
* The assertion grammar therefore remains domain-neutral at the expression
* boundary.
* 
* ============================================================================
* 41. IMPORTANT EXPRESSION INTEGRATION
* ============================================================================
* 
* Do NOT introduce:
* 
* hdlAssertionExpression
* 
* as a second expression hierarchy.
* 
* Do NOT copy:
* 
* arithmetic
* logical
* comparison
* indexing
* calls
* member access
* quantum expressions
* tensor expressions
* 
* into this file.
* 
* An assertion may contain expressions involving:
* 
* classical values
* vectors
* matrices
* tensors
* quantum measurement results
* hybrid values
* hardware signals
* distributed values
* AI/data values
* 
* Semantic analysis determines their legality.
* 
* ============================================================================
* 42. HDL INTEGRATION
* ============================================================================
* 
* The canonical HDL root:
* 
* grammar/hdl/hdl.g4
* 
* must import:
* 
* HdlAssertions
* 
* and use:
* 
* hdlAssertion
* hdlPropertyDeclaration
* 
* as composition points.
* 
* The HDL root MUST remove its existing duplicate:
* 
* hdlAssertion
* hdlAssertionKind
* 
* implementation.
* 
* The old:
* 
* hdlAssertionKind
*     : K_ASSERT
*     | K_ASSUME
*     | K_COVER
* 
* must not survive as a second authority.
* 
* ============================================================================
* 43. MODULE INTEGRATION
* ============================================================================
* 
* Module-level HDL members may include:
* 
* hdlPropertyDeclaration
* hdlAssertion
* 
* where permitted by the HDL semantic model.
* 
* The module grammar should reference these imported rules rather than
* reproduce their syntax.
* 
* ============================================================================
* 44. SEQUENTIAL INTEGRATION
* ============================================================================
* 
* grammar/hdl/sequential.g4 currently contains:
* 
* hdlSequentialAssertion
* 
* That construct MUST be migrated to the shared HDL assertion boundary.
* 
* The sequential grammar should retain only the sequential statement
* composition boundary.
* 
* It should not own:
* 
* ASSERT
* PROPERTY
* assertion arguments
* property declarations
* verification qualifiers.
* 
* The resulting dependency is:
* 
* sequential.g4
*      |
*      v
* HdlAssertions
*      |
*      v
* hdlAssertion
* 
* This avoids separate sequential assertion semantics.
* 
* ============================================================================
* 45. COMBINATIONAL INTEGRATION
* ============================================================================
* 
* grammar/hdl/combinational.g4 currently contains a local assertion rule.
* 
* That rule MUST be removed as an independent owner.
* 
* A combinational block may consume:
* 
* hdlAssertion
* 
* through the common HDL verification boundary.
* 
* The assertion's expression may reference combinational values.
* 
* Semantic analysis remains responsible for determining whether a verification
* property is legal in a combinational context.
* 
* ============================================================================
* 46. HARDWARE-MODULE INTEGRATION
* ============================================================================
* 
* grammar/hdl/hardware-modules.g4 contains assertion-related module syntax.
* 
* It must delegate to:
* 
* hdlAssertion
* hdlPropertyDeclaration
* 
* rather than define:
* 
* hdlModuleAssert
* 
* independently.
* 
* The existing:
* 
* hdlModuleAssert
*     : K_ASSERT expression SEMICOLON
* 
* should be retired as a duplicate syntax owner.
* 
* ============================================================================
* 47. CLOCKING INTEGRATION
* ============================================================================
* 
* Clock declarations remain owned by:
* 
* grammar/hdl/clocks.g4
* 
* Clocking relationships remain owned by:
* 
* grammar/hdl/clocking.g4
* 
* Assertion clock qualifiers merely reference logical clock/event expressions.
* 
* No physical clock selection occurs here.
* 
* ============================================================================
* 48. TIMING INTEGRATION
* ============================================================================
* 
* Timing declarations remain owned by:
* 
* grammar/hdl/timing.g4
* 
* An assertion may reference timing values through ordinary expressions.
* 
* The assertion grammar does not perform:
* 
* timing analysis
* slack analysis
* timing closure
* clock-tree synthesis
* physical delay calculation
* 
* ============================================================================
* 49. RESET INTEGRATION
* ============================================================================
* 
* Reset declarations remain owned by:
* 
* grammar/hdl/reset.g4
* 
* Assertions may reference reset expressions.
* 
* For example:
* 
* assert property(state_valid) disable(reset);
* 
* The parser records the expression.
* 
* Semantic analysis determines whether "reset" denotes a valid disable
* condition in the enclosing clock domain.
* 
* ============================================================================
* 50. STATE-MACHINE INTEGRATION
* ============================================================================
* 
* State-machine syntax remains owned by:
* 
* grammar/hdl/state_machines.g4
* 
* Assertions may reference:
* 
* states
* transitions
* guards
* outputs
* 
* but do not define state-machine syntax.
* 
* ============================================================================
* 51. PIPELINE INTEGRATION
* ============================================================================
* 
* Pipeline syntax remains owned by:
* 
* grammar/hdl/pipelines.g4
* 
* Assertions may reference:
* 
* stage values
* valid signals
* ready signals
* latency expressions
* throughput expressions
* 
* The parser does not perform pipeline scheduling.
* 
* ============================================================================
* 52. MEMORY INTEGRATION
* ============================================================================
* 
* Assertions may inspect logical memory behavior.
* 
* They must not assume:
* 
* a fixed memory size;
* fixed bank count;
* fixed address width;
* fixed physical RAM technology.
* 
* Dimensions remain expressions.
* 
* ============================================================================
* 53. QUANTUM INTEGRATION
* ============================================================================
* 
* HDL assertions may reference hybrid or quantum-related values where the
* enclosing semantic model permits them.
* 
* Example:
* 
* assert(property_value);
* 
* or:
* 
* assert property(measurement == expected);
* 
* The grammar does not define quantum verification semantics.
* 
* If the referenced computation is quantum:
* 
* source
*   ->
* AST
*   ->
* semantic analysis
*   ->
* quantum::ir
*   ->
* optimization
*   ->
* routing
*   ->
* scheduling
*   ->
* QEC / resilience / ZQN
*   ->
* HAL
* 
* remains the established downstream architecture.
* 
* This file MUST NOT create:
* 
* QuantumAssertionIR
* QuantumPropertyIR
* AssertionQIR
* 
* as a competing quantum IR.
* 
* ============================================================================
* 54. VERIFICATION SEMANTICS
* ============================================================================
* 
* Semantic analysis owns:
* 
* predicate typing;
* name resolution;
* property resolution;
* parameter binding;
* clock validation;
* reset/disable validation;
* temporal semantics;
* sampling semantics;
* combinational/sequential legality;
* environment assumptions;
* coverage semantics;
* proof obligations;
* solver/model-checker selection;
* verification capability requirements;
* diagnostics.
* 
* A syntactically valid assertion can therefore be semantically invalid.
* 
* ============================================================================
* 55. VERIFICATION BACKENDS
* ============================================================================
* 
* This grammar does not enumerate:
* 
* SAT
* SMT
* BDD
* model checker A
* model checker B
* simulator A
* simulator B
* vendor assertion engine
* 
* Backend selection belongs downstream.
* 
* A program may express:
* 
* requires capability("formal.verification");
* 
* without naming a specific implementation.
* 
* ============================================================================
* 56. RESOURCE / CAPABILITY INTEGRATION
* ============================================================================
* 
* Verification may require capabilities such as:
* 
* formal.verification
* temporal.verification
* simulation
* waveform
* coverage
* 
* The assertion grammar does not require any particular target to provide
* them.
* 
* Resource/capability satisfaction occurs downstream.
* 
* ============================================================================
* 57. POCO-REAF
* ============================================================================
* 
* Assertion syntax is target independent.
* 
* The same source-level property may be analyzed against:
* 
* tiny embedded hardware
* CPU
* multicore CPU
* GPU
* FPGA
* ASIC
* QPU
* simulator
* accelerator
* HPC system
* cluster
* distributed deployment
* cloud deployment
* future hardware
* 
* The grammar does not select the realization.
* 
* Therefore verification intent remains portable under:
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* subject to semantic validity and available verification/target capabilities.
* 
* ============================================================================
* 58. SCALABILITY
* ============================================================================
* 
* The grammar imposes NO language-level maximum on:
* 
* number of assertions;
* number of properties;
* number of property parameters;
* expression size;
* expression depth;
* qualifier count;
* verification arguments;
* module count;
* signal count;
* clock count;
* clock-domain count;
* state count;
* pipeline stages;
* hardware modules;
* devices;
* nodes;
* qubits;
* CPUs;
* GPUs;
* FPGAs;
* QPUs;
* tensor dimensions;
* memory capacity.
* 
* Repetition is represented structurally.
* 
* Actual resource exhaustion may be handled by compiler/tooling safeguards,
* but such safeguards MUST NOT become language semantics.
* 
* ============================================================================
* 59. HARD-CODING AUDIT
* ============================================================================
* 
* This grammar contains NO:
* 
* MAX_ASSERTIONS
* MAX_PROPERTIES
* MAX_PROPERTY_PARAMETERS
* MAX_ASSERTION_DEPTH
* MAX_CLOCKS
* MAX_CLOCK_DOMAINS
* MAX_SIGNALS
* MAX_REGISTERS
* MAX_MEMORIES
* MAX_STATES
* MAX_PIPELINE_STAGES
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_QPUS
* MAX_NODES
* MAX_DEVICES
* MAX_MEMORY
* MAX_REGISTER_WIDTH
* MAX_TENSOR_RANK
* 
* It contains no:
* 
* physical device IDs;
* physical pin IDs;
* vendor primitive names;
* fixed bus widths;
* fixed clock counts;
* fixed synchronizer counts;
* fixed memory sizes;
* fixed topology.
* 
* A program-level value such as:
* 
* width = 4096
* 
* remains valid program data.
* 
* The prohibition is against converting that value into a universal language
* limit.
* 
* ============================================================================
* 60. DETERMINISM
* ============================================================================
* 
* Parsing depends only on:
* 
* source;
* canonical lexer;
* grammar version;
* parser configuration.
* 
* Parsing MUST NOT depend on:
* 
* current hardware;
* network state;
* filesystem state;
* wall-clock time;
* randomness;
* environment variables;
* runtime state;
* device discovery;
* available verification backend.
* 
* Identical source and parser configuration must produce equivalent parse
* trees.
* 
* ============================================================================
* 61. SECURITY
* ============================================================================
* 
* This grammar:
* 
* performs no I/O;
* performs no networking;
* performs no command execution;
* performs no hardware discovery;
* accesses no credentials;
* executes no verification engine;
* performs no synthesis;
* performs no simulation;
* contains no embedded Rust;
* contains no semantic predicates.
* 
* ============================================================================
* 62. SAFE-RUST CONTRACT
* ============================================================================
* 
* This grammar contains no Rust implementation.
* 
* The consuming Zamani implementation MUST remain:
* 
* Rust 1.97
* or Rust 1.97.1
* Rust 2021
* safe Rust
* 
* No "unsafe" code is required by this grammar.
* 
* This grammar therefore places no requirement for:
* 
* unsafe blocks;
* raw pointers;
* FFI;
* architecture-specific Rust;
* target-specific parser actions.
* 
* ============================================================================
* 63. AST CONTRACT
* ============================================================================
* 
* The frontend AST must preserve at least:
* 
* AssertionStatement
*     condition
*     arguments?
*     source_span
* 
* PropertyAssertion
*     property
*     arguments?
*     qualifiers
*     source_span
* 
* PropertyDeclaration
*     name
*     parameters
*     expression
*     source_span
* 
* VerificationQualifier
*     kind
*     expression
*     source_span
* 
* VerificationArgument
*     name?
*     expression
*     source_span
* 
* The exact Rust structures belong to:
* 
* src/frontend/ast/
* 
* This grammar MUST NOT define those Rust structures.
* 
* ============================================================================
* 64. SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis must determine:
* 
* assertion predicate type;
* property validity;
* property reference resolution;
* property parameter arity;
* parameter type compatibility;
* scope;
* clock validity;
* disable-condition validity;
* verification-context legality;
* side effects;
* resource requirements;
* capability requirements;
* determinism;
* portability;
* backend compatibility.
* 
* ============================================================================
* 65. IR CONTRACT
* ============================================================================
* 
* This file creates NO IR.
* 
* Assertions lower through the canonical semantic pipeline:
* 
* parser
*   ->
* domain-neutral AST
*   ->
* semantic verification model
*   ->
* canonical HDL/hardware semantic representation
*   ->
* verification / synthesis / simulation / target lowering
* 
* If an assertion references quantum semantics, the quantum portion continues
* through:
* 
* quantum::ir
* 
* No second quantum IR is permitted.
* 
* ============================================================================
* 66. COMPILER CONTRACT
* ============================================================================
* 
* The compiler owns:
* 
* AST conversion;
* semantic checking;
* property elaboration;
* verification planning;
* solver/backend selection;
* simulation integration;
* synthesis preservation/removal policy;
* diagnostic generation;
* capability checking;
* resource checking.
* 
* Assertion grammar does not decide whether assertions:
* 
* remain in synthesis;
* are optimized away;
* become simulation checks;
* become formal proof obligations;
* become hardware monitors.
* 
* Those are semantic/compiler decisions.
* 
* ============================================================================
* 67. RUNTIME CONTRACT
* ============================================================================
* 
* This grammar has no runtime behavior.
* 
* If an assertion is retained in:
* 
* simulation;
* emulation;
* runtime monitoring;
* generated hardware;
* 
* that behavior is established downstream.
* 
* ============================================================================
* 68. DIAGNOSTICS
* ============================================================================
* 
* Parser diagnostics should identify:
* 
* missing ASSERT;
* missing PROPERTY;
* missing opening delimiter;
* missing closing delimiter;
* malformed expression;
* malformed property declaration;
* malformed property parameter;
* malformed qualifier;
* malformed verification argument;
* missing statement terminator.
* 
* Semantic diagnostics should identify:
* 
* unknown property;
* invalid property argument;
* invalid predicate type;
* unknown clock;
* invalid disable condition;
* invalid verification context;
* unsupported capability;
* unsatisfied resource requirement;
* illegal hardware semantics.
* 
* ============================================================================
* 69. TEST CONTRACT
* ============================================================================
* 
* Required positive tests:
* 
* assert(condition);
* 
* assert(condition, "message");
* 
* assert(condition, diagnostic);
* 
* assert property(condition);
* 
* assert property(condition) clock(clk);
* 
* assert property(condition) disable(reset);
* 
* property stable(a, b) = a == b;
* 
* assert property(stable(a, b));
* 
* assert(
*     condition,
*     message: "diagnostic",
*     severity: level
* );
* 
* Required negative syntax tests:
* 
* assert;
* 
* assert();
* 
* assert(condition
* 
* assert(condition,);
* 
* assert property;
* 
* property;
* 
* property p = ;
* 
* property p( = expression;
* 
* Required semantic-negative tests:
* 
* unknown property;
* unknown clock;
* invalid predicate;
* wrong property arity;
* incompatible property argument;
* invalid disable expression;
* invalid verification context.
* 
* ============================================================================
* 70. SCALABILITY TEST CONTRACT
* ============================================================================
* 
* Test increasingly large:
* 
* assertion counts;
* property counts;
* property parameters;
* nested expressions;
* verification arguments;
* modules;
* signals;
* clocks;
* clock domains;
* generated hardware structures.
* 
* Tests must verify that no artificial language maximum has been introduced.
* 
* A stress-test failure caused by available compiler memory/CPU is an
* implementation/resource observation, not a new grammar limit.
* 
* ============================================================================
* 71. CROSS-DOMAIN TEST CONTRACT
* ============================================================================
* 
* Assertions must be testable against:
* 
* classical values;
* vector values;
* tensor values;
* quantum measurement results;
* hybrid values;
* hardware signals;
* memory interfaces;
* pipeline values;
* state-machine state;
* distributed values;
* AI/data values.
* 
* The grammar remains unchanged across these domains.
* 
* ============================================================================
* 72. COMPATIBILITY
* ============================================================================
* 
* This file is an additive ownership correction.
* 
* It MUST NOT rename:
* 
* grammar/hdl/hdl.g4
* grammar/hdl/sequential.g4
* grammar/hdl/combinational.g4
* grammar/hdl/hardware-modules.g4
* grammar/statements/assertions.g4
* 
* Instead, those files must progressively delegate their HDL assertion
* boundaries to this grammar.
* 
* Existing simple:
* 
* assert(condition);
* 
* syntax remains compatible.
* 
* ============================================================================
* 73. REQUIRED INTEGRATION CHANGES
* ============================================================================
* 
* The following changes are required outside this file.
* 
* They are listed explicitly so this file can be completed independently
* without later architectural guesswork.
* 
* ---
* A. grammar/hdl/hdl.g4
* ---
* 
* Import:
* 
* HdlAssertions
* 
* Remove the local:
* 
* hdlAssertion
* hdlAssertionKind
* 
* productions.
* 
* Replace their use with:
* 
* hdlAssertion
* hdlPropertyDeclaration
* 
* in the HDL member dispatcher.
* 
* ---
* B. grammar/hdl/sequential.g4
* ---
* 
* Remove:
* 
* hdlSequentialAssertion
* 
* as an independent assertion syntax owner.
* 
* Delegate to the shared:
* 
* hdlAssertion
* 
* at the sequential statement composition boundary.
* 
* ---
* C. grammar/hdl/combinational.g4
* ---
* 
* Remove:
* 
* hdlCombinationalAssertion
* 
* as an independent assertion syntax owner.
* 
* Delegate to:
* 
* hdlAssertion
* 
* ---
* D. grammar/hdl/hardware-modules.g4
* ---
* 
* Retire:
* 
* hdlModuleAssert
* 
* as a duplicate syntax owner.
* 
* Use:
* 
* hdlAssertion
* hdlPropertyDeclaration
* 
* ---
* E. grammar/lexer/keywords.g4
* ---
* 
* Current canonical tokens:
* 
* ASSERT
* PROPERTY
* 
* are sufficient for the immediately supported assertion/property syntax.
* 
* To promote assumptions and coverage to first-class reserved syntax, add:
* 
* ASSUME : 'assume' ;
* COVER  : 'cover' ;
* 
* No K_* aliases should be introduced.
* 
* ---
* F. grammar/spec/hdl.md
* ---
* 
* Record:
* 
* grammar/hdl/assertions.g4
* 
* as the authoritative HDL assertion/property syntax owner.
* 
* Record:
* 
* hdlAssertion
* hdlPropertyDeclaration
* 
* as the canonical public entry rules.
* 
* ---
* G. grammar/grammar.md
* ---
* 
* Track independently:
* 
* SPECIFIED
* GRAMMAR_IMPLEMENTED
* AST_IMPLEMENTED
* SEMANTICALLY_IMPLEMENTED
* IR_IMPLEMENTED
* TESTED
* STABLE
* 
* for HDL assertions and properties.
* 
* ---
* H. grammar/validation/
* ---
* 
* Validation MUST reject:
* 
* duplicate hdlAssertion owners;
* K_ASSERT;
* K_ASSUME;
* K_COVER;
* parser-local lexer definitions;
* second assertion expression grammars;
* fixed verification capacity constants;
* target-specific assertion syntax.
* 
* ============================================================================
* 74. OWNERSHIP MATRIX
* ============================================================================
* 
* Construct                         Owner
* ---
* assert(...)                       assertions.g4
* assert property(...)             assertions.g4
* property declaration             assertions.g4
* assertion arguments              assertions.g4
* verification qualifiers          assertions.g4
* clock declaration                clocks.g4
* clock-domain relation            clocking.g4
* timing declaration               timing.g4
* reset declaration                reset.g4
* sequential behavior              sequential.g4
* combinational behavior           combinational.g4
* state machines                   state_machines.g4
* pipelines                        pipelines.g4
* module composition               hdl.g4 / hardware-modules.g4
* general expressions              canonical expression grammar
* identifiers                      canonical lexer/parser
* semantic verification            semantic analysis
* proof/model checking             verification subsystem
* synthesis                        compiler/backend
* simulation                       execution/simulation subsystem
* physical realization             hardware backend
* 
* ============================================================================
* 75. FINAL ARCHITECTURAL INVARIANT
* ============================================================================
* 
* grammar/hdl/assertions.g4 means:
* 
* "What verification property syntax does Zamani HDL provide?"
* 
* It does NOT mean:
* 
* "How is the property proven?"
* 
* "Which solver is used?"
* 
* "Which simulator is used?"
* 
* "Which FPGA implements the assertion?"
* 
* "Which ASIC cell implements the assertion?"
* 
* "Which CPU executes verification?"
* 
* "Which GPU executes verification?"
* 
* "Which QPU executes verification?"
* 
* "How much memory does verification require?"
* 
* Those questions belong downstream.
* 
* ============================================================================
* 76. PRODUCTION COMPLETION CHECKLIST
* ============================================================================
* 
* [x] Dedicated HDL assertion/property ownership.
* [x] Canonical ZamaniLexer dependency.
* [x] No embedded Rust.
* [x] No unsafe Rust.
* [x] Rust 1.97 / 1.97.1 compatible implementation contract.
* [x] No parser-local lexer.
* [x] No hardware limits.
* [x] No fixed verification capacity.
* [x] No fixed clock count.
* [x] No fixed signal count.
* [x] No fixed property count.
* [x] No fixed property parameter count.
* [x] No vendor dependency.
* [x] No physical device dependency.
* [x] No physical pin dependency.
* [x] No competing quantum IR.
* [x] Explicit AST contract.
* [x] Explicit semantic contract.
* [x] Explicit IR contract.
* [x] Explicit compiler contract.
* [x] Explicit runtime contract.
* [x] Explicit diagnostics contract.
* [x] Explicit scalability contract.
* [x] Explicit POCO-REAF contract.
* [x] Explicit cross-domain integration.
* [x] Explicit repository integration.
* [x] Existing filenames preserved.
* [x] Existing simple assert syntax preserved.
* 
* Remaining repository integration:
* 
* [ ] Import HdlAssertions from hdl.g4.
* [ ] Remove duplicate hdlAssertion from hdl.g4.
* [ ] Remove duplicate sequential assertion owner.
* [ ] Remove duplicate combinational assertion owner.
* [ ] Remove duplicate hardware-module assertion owner.
* [ ] Add ASSUME/COVER to canonical lexer if those forms are promoted.
* [ ] Add conformance tests.
* 
* ============================================================================
  */