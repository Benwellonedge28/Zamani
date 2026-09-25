/**

* ============================================================================
* Zamani Universal Computing Language
* ============================================================================
* 
* FILE
* ---
* grammar/hardware/power.g4
* 
* GRAMMAR
* ---
* ZamaniHardwarePowerParser
* 
* STATUS
* ---
* Production-ready hardware power-intent grammar
* 
* RUNTIME / COMPILER BASELINE
* ---
* Rust 1.97 / Rust 1.97.1
* Rust 2021
* 
* SAFETY
* ---
* Action-free ANTLR grammar.
* No embedded Rust.
* No embedded target-language actions.
* No semantic predicates.
* No unsafe implementation requirement.
* 
* ============================================================================
* 1. PURPOSE
* ============================================================================
* 
* This grammar defines the source-language syntax for expressing
* target-independent hardware power intent.
* 
* It allows Zamani programs and hardware contracts to describe:
* 
* - power requirements;
* - power constraints;
* - power preferences;
* - power hints;
* - power budgets;
* - power limits as program/contract constraints;
* - power relationships;
* - power profiles;
* - power domains;
* - power states;
* - power transitions;
* - power properties;
* - power measurements/contracts;
* - dynamic and static power intent;
* - energy relationships relevant to power contracts;
* - scaling relationships;
* - target-independent power characteristics;
* - extensible vendor/technology properties.
* 
* This file describes SOURCE-LEVEL POWER INTENT.
* 
* It does NOT implement:
* 
* - physical power measurement;
* - power estimation algorithms;
* - thermal simulation;
* - device discovery;
* - voltage regulation;
* - clock/power management;
* - runtime power enforcement;
* - scheduler implementation;
* - placement;
* - routing;
* - synthesis;
* - target selection;
* - hardware drivers;
* - calibration;
* - physical electrical models.
* 
* ============================================================================
* 2. ARCHITECTURAL POSITION
* ============================================================================
* 
* The intended pipeline is:
* 
* Zamani source
*      |
*      v
* Zamani lexer
*      |
*      v
* Zamani parser
*      |
*      v
* domain-neutral AST
*      |
*      v
* semantic analysis
*      |
*      +--> type analysis
*      +--> capability analysis
*      +--> resource analysis
*      +--> power-contract validation
*      +--> thermal-contract validation
*      |
*      v
* canonical semantic representation
*      |
*      v
* compiler IR
*      |
*      +--> optimization
*      +--> scheduling
*      +--> placement
*      +--> routing
*      +--> synthesis
*      +--> resilience
*      |
*      v
* HAL / target realization
*      |
*      v
* runtime / deployment
* 
* Power grammar therefore remains ABOVE physical realization.
* 
* ============================================================================
* 3. OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - hardware power declarations;
* - hardware power contracts;
* - power-specific hardware intent;
* - power requirements;
* - power constraints;
* - power preferences;
* - power hints;
* - power budgets;
* - power profiles;
* - power domains;
* - power states;
* - power transitions;
* - power relationships;
* - power properties;
* - extensible power metadata.
* 
* THIS FILE DOES NOT OWN:
* 
* - the POWER lexer token;
* - numeric literal syntax;
* - unit lexical syntax;
* - general expressions;
* - general identifiers;
* - general types;
* - universal resources;
* - universal capabilities;
* - clocks;
* - timing;
* - thermal semantics;
* - energy resource declarations;
* - hardware targets;
* - physical devices;
* - CPU/GPU/FPGA/QPU declarations;
* - HDL;
* - quantum operations;
* - quantum::ir;
* - QEC;
* - ZQN;
* - routing;
* - scheduling;
* - optimization;
* - HAL;
* - runtime execution.
* 
* ============================================================================
* 4. CRITICAL RESOURCE BOUNDARY
* ============================================================================
* 
* grammar/hardware/resources.g4 already owns the universal resource clause:
* 
* power = expression;
* 
* That rule MUST remain there.
* 
* This file does NOT redefine:
* 
* hardwareResourcePowerClause
* 
* and does NOT create a second universal resource grammar.
* 
* Instead:
* 
* hardware/resources.g4
*         |
*         +--> generic resource power
* 
* hardware/power.g4
*         |
*         +--> hardware-specific power contract
* 
* These are complementary ownership domains.
* 
* Example:
* 
* resource accelerator {
*     power = power_budget;
* };
* 
* belongs to the resource grammar.
* 
* Whereas:
* 
* power contract accelerator_power {
*     budget <= available_power;
* }
* 
* belongs to this grammar.
* 
* Semantic analysis is responsible for determining whether the two contracts
* are compatible.
* 
* ============================================================================
* 5. POWER IS NOT A HARDWARE LIMIT
* ============================================================================
* 
* Power values are program/contract data.
* 
* A value such as:
* 
* 100
* 
* does NOT establish:
* 
* MAX_POWER = 100
* 
* Likewise:
* 
* power <= 100 W
* 
* is a PROGRAM CONSTRAINT.
* 
* It is not a universal Zamani compiler limit.
* 
* The target may provide:
* 
* 1 W
* 
* 10 W
* 
* 100 W
* 
* 1 kW
* 
* 1 MW
* 
* ...
* 
* or another available capability.
* 
* Whether the target satisfies the source requirement is determined
* downstream.
* 
* ============================================================================
* 6. ABSOLUTE SCALABILITY RULE
* ============================================================================
* 
* This grammar MUST NOT encode:
* 
* MAX_POWER
* MAX_ENERGY
* MAX_POWER_DOMAINS
* MAX_POWER_STATES
* MAX_POWER_PROFILES
* MAX_POWER_TRANSITIONS
* MAX_DEVICES
* MAX_ACCELERATORS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_QPUS
* MAX_NODES
* 
* Nor may it encode equivalent limits through bounded grammar alternatives.
* 
* Repetition is therefore intentionally unbounded:
* 
* *
* +
* 
* where the language semantics require arbitrary collections.
* 
* Actual limits belong to:
* 
* target capabilities
* semantic validation
* resource availability
* compiler policy
* runtime availability
* deployment policy
* physical hardware
* 
* ============================================================================
* 7. POCO-REAF
* ============================================================================
* 
* Power contracts MUST preserve:
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* The same source-level power intent may be realized differently on:
* 
* embedded systems
* CPUs
* multicore CPUs
* GPUs
* FPGAs
* ASICs
* accelerators
* QPUs
* simulators
* HPC systems
* clusters
* distributed systems
* cloud systems
* edge systems
* future hardware
* 
* The grammar describes WHAT POWER PROPERTY is required or preferred.
* 
* It does not prescribe WHICH physical device supplies it.
* 
* ============================================================================
* 8. REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
* ============================================================================
* 
* These concepts MUST remain distinct.
* 
* Requirement
* ---
* A property required for valid realization.
* 
* Example:
* 
* require <= power_budget;
* 
* Constraint
* ---
* A condition that must hold for the selected realization.
* 
* Example:
* 
* constraint peak <= peak_limit;
* 
* Preference
* ---
* A desirable property that may be violated if necessary.
* 
* Example:
* 
* prefer lower <= power;
* 
* Hint
* ---
* An optimization suggestion without semantic necessity.
* 
* Example:
* 
* hint minimize dynamic_power;
* 
* The semantic layer determines how these categories interact.
* 
* ============================================================================
* 9. POWER DECLARATION
* ============================================================================
* 
* A named power contract is the principal public entry point.
* 
* Example:
* 
* power contract compute_power {
*     budget <= available_power;
* }
* 
* The identifier is symbolic.
* 
* It is NOT a physical device identifier.
* 
* ============================================================================
  */

parser grammar ZamaniHardwarePowerParser;

options {
tokenVocab = ZamaniTokens;
}

/* ============================================================================

* 10. PUBLIC ENTRY POINT
* ============================================================================
* 
* hardware.g4 MUST delegate power declarations to:
* 
* hardwarePowerDeclaration
* 
* This rule is intentionally independent of the implementation of the
* hardware composition grammar.
  */

hardwarePowerDeclaration
: hardwarePowerAttributes*
hardwarePowerVisibility?
hardwarePowerModifier*
POWER
hardwarePowerDeclarationKind?
identifier
hardwarePowerGenericParameters?
hardwarePowerTargetClause?
hardwarePowerContractBody
;

/* ============================================================================

* 11. DECLARATION KIND
* ============================================================================
* 
* The declaration kind remains intentionally open-ended.
* 
* The standard "contract" spelling is provided as a stable language form.
* 
* Additional kinds may be introduced by dialect/semantic extensions without
* requiring the POWER token itself to change.
  */

hardwarePowerDeclarationKind
: CONTRACT
| PROFILE
| DOMAIN
| STATE
;

/* ============================================================================

* 12. VISIBILITY
* ============================================================================
  */

hardwarePowerVisibility
: PUBLIC
| PRIVATE
| PROTECTED
| INTERNAL
;

/* ============================================================================

* 13. MODIFIERS
* ============================================================================
* 
* Modifiers describe source-level declaration properties.
* 
* They do not describe physical implementation.
  */

hardwarePowerModifier
: STATIC
| CONST
| EXTERN
| FINAL
| ABSTRACT
| SEALED
| PARTIAL
;

/* ============================================================================

* 14. ATTRIBUTES
* ============================================================================
* 
* Attributes remain qualified names rather than a fixed vendor/technology
* vocabulary.
* 
* This is necessary for future hardware evolution.
* 
* Example:
* 
* @vendor.example(power_mode)
* power contract ...
* 
* Semantic validation determines whether an attribute is known and valid.
  */

hardwarePowerAttributes
: AT
qualifiedName
(
LPAREN
hardwarePowerAttributeArguments?
RPAREN
)?
;

hardwarePowerAttributeArguments
: expressionList
;

/* ============================================================================

* 15. GENERIC PARAMETERS
* ============================================================================
* 
* Power contracts may depend on program-level symbolic values.
* 
* Example:
* 
* power contract budget<limit, workload> {
*     budget <= limit;
* }
* 
* No finite hardware scale is encoded.
  */

hardwarePowerGenericParameters
: LT
hardwarePowerGenericParameter
(
COMMA
hardwarePowerGenericParameter
)*
COMMA?
GT
;

hardwarePowerGenericParameter
: identifier
(
COLON
hardwarePowerGenericBound
)?
(
ASSIGN
expression
)?
;

hardwarePowerGenericBound
: qualifiedName
| hardwarePowerCapabilityReference
;

/* ============================================================================

* 16. TARGET CLAUSE
* ============================================================================
* 
* A power contract MAY identify an abstract target class.
* 
* It MUST NOT require a physical device.
* 
* Example:
* 
* target accelerator;
* 
* is an abstract target relationship.
* 
* Physical target selection remains downstream.
  */

hardwarePowerTargetClause
: TARGET
qualifiedName
SEMICOLON
;

/* ============================================================================

* 17. POWER CONTRACT BODY
* ============================================================================
  */

hardwarePowerContractBody
: LBRACE
hardwarePowerItem*
RBRACE
;

/* ============================================================================

* 18. POWER BODY DISPATCH
* ============================================================================
* 
* All power-specific constructs enter through this single dispatch rule.
* 
* This prevents a second hidden power grammar from being created inside
* hardware.g4.
  */

hardwarePowerItem
: hardwarePowerAttributes*
(
hardwarePowerRequirement
| hardwarePowerConstraint
| hardwarePowerPreference
| hardwarePowerHint
| hardwarePowerBudget
| hardwarePowerProfile
| hardwarePowerDomain
| hardwarePowerState
| hardwarePowerTransition
| hardwarePowerRelation
| hardwarePowerProperty
| hardwarePowerMeasurementContract
| hardwarePowerAssertion
| hardwarePowerCapabilityRequirement
| hardwarePowerResourceRequirement
| hardwarePowerScalingContract
| hardwarePowerNestedContract
)
;

/* ============================================================================

* 19. REQUIREMENT
* ============================================================================
* 
* Example:
* 
* require peak <= power_budget;
* 
* The grammar does not evaluate the expression.
  */

hardwarePowerRequirement
: REQUIRE
hardwarePowerRequirementExpression
SEMICOLON
;

/* ============================================================================

* 20. CONSTRAINT
* ============================================================================
  */

hardwarePowerConstraint
: CONSTRAINT
hardwarePowerConstraintExpression
SEMICOLON
;

/* ============================================================================

* 21. PREFERENCE
* ============================================================================
  */

hardwarePowerPreference
: PREFER
hardwarePowerPreferenceExpression
SEMICOLON
;

/* ============================================================================

* 22. HINT
* ============================================================================
  */

hardwarePowerHint
: HINT
hardwarePowerHintExpression
SEMICOLON
;

/* ============================================================================

* 23. POWER BUDGET
* ============================================================================
* 
* A budget is a semantic bound.
* 
* It is NOT a compiler-wide maximum.
* 
* Example:
* 
* budget <= workload_power;
* 
* or:
* 
* budget = power_budget;
* 
* Both are source-level expressions.
  */

hardwarePowerBudget
: BUDGET
hardwarePowerBudgetOperator
expression
SEMICOLON
;

hardwarePowerBudgetOperator
: ASSIGN
| LE
| LT
| GE
| GT
;

/* ============================================================================

* 24. POWER PROFILE
* ============================================================================
* 
* Profiles allow source programs to describe power behavior without
* prescribing implementation.
* 
* Example:
* 
* profile compute {
*     dynamic = dynamic_power;
*     static = leakage_power;
* }
* 
* Property names remain open.
  */

hardwarePowerProfile
: PROFILE
identifier
hardwarePowerProfileBody
;

hardwarePowerProfileBody
: LBRACE
hardwarePowerProfileItem*
RBRACE
;

hardwarePowerProfileItem
: hardwarePowerProperty
| hardwarePowerRequirement
| hardwarePowerConstraint
| hardwarePowerPreference
| hardwarePowerHint
;

/* ============================================================================

* 25. POWER DOMAIN
* ============================================================================
* 
* A power domain is a logical semantic grouping.
* 
* It does not identify a physical voltage rail unless a downstream target
* contract explicitly maps it to one.
  */

hardwarePowerDomain
: DOMAIN
identifier
hardwarePowerDomainBody
;

hardwarePowerDomainBody
: LBRACE
hardwarePowerDomainItem*
RBRACE
;

hardwarePowerDomainItem
: hardwarePowerProperty
| hardwarePowerRequirement
| hardwarePowerConstraint
| hardwarePowerPreference
| hardwarePowerHint
| hardwarePowerReference
;

/* ============================================================================

* 26. POWER STATE
* ============================================================================
* 
* Power states are symbolic semantic states.
* 
* The grammar does not enumerate:
* 
* ON
* OFF
* SLEEP
* DEEP_SLEEP
* TURBO
* 
* as universal states.
* 
* State names remain identifiers so future targets can introduce states
* through semantic contracts/dialects.
  */

hardwarePowerState
: STATE
identifier
hardwarePowerStateBody
;

hardwarePowerStateBody
: LBRACE
hardwarePowerStateItem*
RBRACE
;

hardwarePowerStateItem
: hardwarePowerProperty
| hardwarePowerRequirement
| hardwarePowerConstraint
| hardwarePowerPreference
| hardwarePowerHint
| hardwarePowerReference
;

/* ============================================================================

* 27. POWER TRANSITION
* ============================================================================
* 
* Describes a logical relationship between power states.
* 
* It does not implement a power-management controller.
  */

hardwarePowerTransition
: TRANSITION
hardwarePowerStateReference
TO
hardwarePowerStateReference
hardwarePowerTransitionBody?
SEMICOLON
;

hardwarePowerTransitionBody
: LBRACE
hardwarePowerTransitionItem*
RBRACE
;

hardwarePowerTransitionItem
: hardwarePowerProperty
| hardwarePowerRequirement
| hardwarePowerConstraint
| hardwarePowerPreference
| hardwarePowerHint
;

/* ============================================================================

* 28. POWER RELATION
* ============================================================================
* 
* Relations allow arbitrary source-level relationships without enumerating
* every possible physical power-management concept.
* 
* Example:
* 
* relation accelerator_power {
*     source = accelerator;
*     property = dynamic;
*     expression = workload_power;
* }
* 
* ============================================================================
  */

hardwarePowerRelation
: RELATION
identifier
hardwarePowerRelationBody
;

hardwarePowerRelationBody
: LBRACE
hardwarePowerRelationItem*
RBRACE
;

hardwarePowerRelationItem
: hardwarePowerProperty
| hardwarePowerRequirement
| hardwarePowerConstraint
| hardwarePowerPreference
| hardwarePowerHint
| hardwarePowerReference
;

/* ============================================================================

* 29. POWER PROPERTY
* ============================================================================
* 
* Property names are deliberately open.
* 
* Standard semantic names may include:
* 
* static
* dynamic
* idle
* peak
* average
* sustained
* transient
* leakage
* switching
* 
* They remain identifiers rather than permanent universal keywords.
* 
* This avoids requiring a grammar release for every future power technology.
  */

hardwarePowerProperty
: hardwarePowerPropertyName
hardwarePowerPropertyOperator
expression
SEMICOLON
;

hardwarePowerPropertyName
: identifier
;

hardwarePowerPropertyOperator
: ASSIGN
| LE
| LT
| GE
| GT
;

/* ============================================================================

* 30. MEASUREMENT CONTRACT
* ============================================================================
* 
* This does not perform measurement.
* 
* It describes a contract concerning an observable power quantity.
* 
* Example:
* 
* measurement peak {
*     require <= peak_limit;
* }
* 
* The runtime/measurement subsystem is responsible for obtaining actual
* measurements.
  */

hardwarePowerMeasurementContract
: MEASUREMENT
identifier
hardwarePowerMeasurementBody
;

hardwarePowerMeasurementBody
: LBRACE
hardwarePowerMeasurementItem*
RBRACE
;

hardwarePowerMeasurementItem
: hardwarePowerProperty
| hardwarePowerRequirement
| hardwarePowerConstraint
| hardwarePowerPreference
| hardwarePowerHint
;

/* ============================================================================

* 31. ASSERTION
* ============================================================================
* 
* Assertions are source-level contracts.
* 
* They are not runtime implementation.
  */

hardwarePowerAssertion
: ASSERT
hardwarePowerBooleanExpression
SEMICOLON
;

/* ============================================================================

* 32. CAPABILITY REQUIREMENT
* ============================================================================
* 
* Example:
* 
* capability power_management;
* 
* This describes an abstract capability.
* 
* It does not discover whether a physical target implements it.
  */

hardwarePowerCapabilityRequirement
: CAPABILITY
hardwarePowerCapabilityReference
hardwarePowerCapabilityValue?
SEMICOLON
;

hardwarePowerCapabilityReference
: qualifiedName
;

hardwarePowerCapabilityValue
: ASSIGN
expression
;

/* ============================================================================

* 33. RESOURCE REQUIREMENT
* ============================================================================
* 
* This is deliberately a reference to the universal resource model.
* 
* The power grammar does not redefine resource declarations.
* 
* Example:
* 
* resource power >= required_power;
* 
* Semantic analysis determines whether the referenced resource exists and
* whether the requirement is satisfiable.
  */

hardwarePowerResourceRequirement
: RESOURCE
qualifiedName
hardwarePowerResourceRelation?
SEMICOLON
;

hardwarePowerResourceRelation
: hardwarePowerComparisonOperator
expression
;

/* ============================================================================

* 34. SCALING CONTRACT
* ============================================================================
* 
* Power behavior can depend on workload/resource scale.
* 
* The grammar accepts arbitrary expressions.
* 
* It does not impose a finite number of scaling points.
* 
* Example:
* 
* scaling workload {
*     power = base_power + coefficient * workload;
* }

*/

hardwarePowerScalingContract
: SCALING
identifier
hardwarePowerScalingBody
;

hardwarePowerScalingBody
: LBRACE
hardwarePowerScalingItem*
RBRACE
;

hardwarePowerScalingItem
: hardwarePowerProperty
| hardwarePowerRequirement
| hardwarePowerConstraint
| hardwarePowerPreference
| hardwarePowerHint
;

/* ============================================================================

* 35. NESTED CONTRACT
* ============================================================================
* 
* Allows compositional power contracts without introducing another grammar
* namespace.
  */

hardwarePowerNestedContract
: CONTRACT
identifier
hardwarePowerContractBody
;

/* ============================================================================

* 36. REFERENCES
* ============================================================================
* 
* Power references are symbolic.
* 
* They may identify:
* 
* a logical power domain;
* a power profile;
* a resource;
* a capability;
* a target;
* a semantic property.
* 
* They MUST NOT inherently identify a physical device.
  */

hardwarePowerReference
: REFERENCE
qualifiedName
SEMICOLON
;

hardwarePowerStateReference
: qualifiedName
;

/* ============================================================================

* 37. REQUIREMENT EXPRESSIONS
* ============================================================================
* 
* Requirement expressions intentionally use the canonical expression model.
* 
* The power grammar does not create a second expression language.
  */

hardwarePowerRequirementExpression
: hardwarePowerBooleanExpression
;

hardwarePowerConstraintExpression
: hardwarePowerBooleanExpression
;

hardwarePowerPreferenceExpression
: hardwarePowerBooleanExpression
;

hardwarePowerHintExpression
: expression
;

hardwarePowerBooleanExpression
: expression
;

/* ============================================================================

* 38. COMPARISON OPERATOR
* ============================================================================
* 
* Comparison operators are references to the canonical lexical vocabulary.
* 
* This file does not define duplicate operator tokens.
  */

hardwarePowerComparisonOperator
: EQ
| NE
| LT
| LE
| GT
| GE
;

/* ============================================================================

* 39. INTEGRATION BRIDGE
* ============================================================================
* 
* The bridge rule is deliberately named and isolated.
* 
* It allows the hardware composition grammar to consume power syntax without
* taking ownership of its internals.
* 
* hardware.g4 should contain:
* 
* | hardwarePowerDeclaration
* 
* inside hardwareItem.
* 
* No power rules should be copied into hardware.g4.
  */

hardwarePowerContract
: hardwarePowerDeclaration
;

/* ============================================================================

* 40. AST CONTRACT
* ============================================================================
* 
* This grammar MUST map into the domain-neutral frontend AST.
* 
* Recommended conceptual mapping:
* 
* hardwarePowerDeclaration
*      |
*      v
* Declaration
*      |
*      v
* HardwarePowerContract
* 
* The exact Rust AST type remains owned by:
* 
* src/frontend/ast/
* 
* This grammar MUST NOT define Rust AST structures.
* 
* The AST should preserve at minimum:
* 
* declaration name
* declaration kind
* attributes
* modifiers
* generic parameters
* target reference
* body items
* source spans
* 
* For body items, the AST must preserve their semantic category:
* 
* requirement
* constraint
* preference
* hint
* budget
* profile
* domain
* state
* transition
* relation
* property
* measurement contract
* capability requirement
* resource requirement
* scaling contract
* 
* No information may be silently discarded during parsing.
* 
* ============================================================================
* 41. SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis owns:
* 
* unit interpretation;
* dimensional correctness;
* power/energy consistency;
* target compatibility;
* capability validation;
* resource validation;
* requirement classification;
* constraint validation;
* preference validation;
* hint handling;
* state transition validity;
* domain relationships;
* profile validity;
* measurement semantics;
* scaling semantics.
* 
* This grammar intentionally does not decide whether:
* 
* 10 W
* 
* is compatible with a target.
* 
* It only recognizes the source representation.
* 
* ============================================================================
* 42. UNIT / DIMENSION BOUNDARY
* ============================================================================
* 
* This file intentionally does NOT introduce a second unit lexer.
* 
* Power values should use the canonical Zamani literal/expression system.
* 
* If the repository's canonical quantity/unit system supports:
* 
* W
* mW
* kW
* MW
* GW
* 
* or other representations, this grammar consumes them through:
* 
* expression
* 
* rather than defining duplicate power-specific literal tokens.
* 
* The same applies to derived relationships involving:
* 
* energy
* time
* voltage
* current
* frequency
* duration
* 
* Dimensional validity belongs to semantic analysis.
* 
* ============================================================================
* 43. ENERGY / POWER RELATIONSHIP
* ============================================================================
* 
* Power and energy are related but are not the same semantic quantity.
* 
* This grammar therefore does not redefine ENERGY.
* 
* Existing:
* 
* hardware/resources.g4
* 
* owns the resource-level energy clause.
* 
* A power contract may reference energy expressions through:
* 
* expression
* 
* where the semantic type system determines dimensional correctness.
* 
* Conceptually:
* 
* energy = power * duration
* 
* but this relationship MUST NOT be implemented as parser logic.
* 
* ============================================================================
* 44. TIMING INTEGRATION
* ============================================================================
* 
* Power may depend on timing.
* 
* This file does NOT redefine timing.
* 
* Timing remains owned by:
* 
* grammar/hardware/timing.g4
* 
* and related timing contracts.
* 
* Example semantic relationship:
* 
* power contract workload {
*     peak <= peak_power;
*     ...
* }
* 
* Timing analysis may consume the resulting semantic power contract.
* 
* The power grammar does not parse timing schedules.
* 
* ============================================================================
* 45. THERMAL INTEGRATION
* ============================================================================
* 
* Power and thermal behavior are related but distinct domains.
* 
* Power syntax MUST NOT become thermal syntax.
* 
* Thermal analysis, temperature constraints and thermal models belong to the
* thermal/resource/target semantic layer designated by the repository.
* 
* This grammar may reference thermal capabilities/properties symbolically:
* 
* require thermal_management;
* 
* but it does not define thermal equations or thermal simulation.
* 
* ============================================================================
* 46. RESOURCE INTEGRATION
* ============================================================================
* 
* Existing:
* 
* grammar/hardware/resources.g4
* 
* already owns:
* 
* hardwareResourcePowerClause
* 
* with the canonical form:
* 
* power = expression;
* 
* That rule MUST remain unchanged as the resource-level power representation.
* 
* This file provides a higher-level hardware power contract.
* 
* Therefore:
* 
* resource power
* 
* and:
* 
* hardware power contract
* 
* are semantically composable but grammatically distinct.
* 
* ============================================================================
* 47. CAPABILITY INTEGRATION
* ============================================================================
* 
* Existing:
* 
* grammar/hardware/capabilities.g4
* 
* owns hardware capability declarations.
* 
* This file may reference capabilities using:
* 
* qualifiedName
* 
* and MUST NOT duplicate capability declaration ownership.
* 
* Examples:
* 
* capability power_management;
* 
* capability dynamic_power_scaling;
* 
* remain semantic capability references unless explicitly declared through
* the canonical capability grammar.
* 
* ============================================================================
* 48. TARGET INTEGRATION
* ============================================================================
* 
* Existing:
* 
* grammar/hardware/targets.g4
* 
* owns target declarations.
* 
* This file may reference an abstract target through:
* 
* hardwarePowerTargetClause
* 
* but MUST NOT duplicate target declaration syntax.
* 
* Physical target selection remains downstream.
* 
* ============================================================================
* 49. HARDWARE.G4 INTEGRATION
* ============================================================================
* 
* The existing:
* 
* grammar/hardware/hardware.g4
* 
* currently has hardware-wide composition ownership.
* 
* It should consume:
* 
* hardwarePowerDeclaration
* 
* as a specialized hardware item.
* 
* Integration point:
* 
* hardwareItem
*     |
*     +--> hardwarePowerDeclaration
* 
* The parent grammar MUST NOT copy the contents of this file.
* 
* This creates one ownership boundary:
* 
* hardware.g4
*     |
*     +--> power.g4
* 
* rather than:
* 
* hardware.g4
*     |
*     +--> duplicated power rules
* 
* ============================================================================
* 50. ZAMANI.G4 INTEGRATION
* ============================================================================
* 
* grammar/Zamani.g4 remains the root composition grammar.
* 
* It MUST NOT import this file directly.
* 
* The composition chain remains:
* 
* Zamani.g4
*     |
*     v
* ZamaniParser.g4
*     |
*     v
* hardware composition
*     |
*     v
* hardwarePowerDeclaration
* 
* This preserves the repository's single-root architecture.
* 
* ============================================================================
* 51. LEXER INTEGRATION
* ============================================================================
* 
* This file consumes the existing POWER token.
* 
* The canonical lexical owner already defines:
* 
* POWER : 'power' ;
* 
* This file MUST NOT define:
* 
* POWER
* 
* again.
* 
* The lexer hierarchy remains:
* 
* grammar/lexer/
*     |
*     v
* ZamaniTokens
*     |
*     v
* grammar/antlr/ZamaniLexer.g4
* 
* No power-specific lexer is created.
* 
* ============================================================================
* 52. RUST INTEGRATION
* ============================================================================
* 
* This grammar contains no Rust code.
* 
* The Rust implementation consuming its parse tree must remain compatible
* with:
* 
* Rust 1.97
* Rust 1.97.1
* Rust 2021
* 
* and must not require:
* 
* unsafe
* 
* Rust code.
* 
* This file does not prescribe implementation details for:
* 
* src/lexer.rs
* src/parser.rs
* src/frontend/ast/
* 
* Those files own executable frontend behavior.
* 
* ============================================================================
* 53. CANONICAL IR INTEGRATION
* ============================================================================
* 
* This grammar does NOT define an IR.
* 
* Power information must lower through the repository's canonical semantic
* representation and compiler IR.
* 
* Conceptually:
* 
* HardwarePowerContract
*         |
*         v
* SemanticPowerContract
*         |
*         v
* canonical resource/capability/constraint representation
*         |
*         v
* compiler IR
* 
* No second power IR is introduced merely because this grammar exists.
* 
* ============================================================================
* 54. QUANTUM INTEGRATION
* ============================================================================
* 
* Power requirements for quantum execution may be represented here.
* 
* Example semantic intent:
* 
* power contract qpu_power {
*     require peak <= power_budget;
* }
* 
* The contract may later be consumed alongside:
* 
* quantum::ir
* 
* but this grammar MUST NOT modify or duplicate quantum::ir.
* 
* The quantum pipeline remains:
* 
* quantum source
*      |
*      v
* domain-neutral AST
*      |
*      v
* semantic analysis
*      |
*      v
* quantum::ir
*      |
*      v
* optimization
*      |
*      v
* routing
*      |
*      v
* scheduling
*      |
*      v
* QEC / resilience
*      |
*      v
* ZQN
*      |
*      v
* HAL
* 
* Power information is associated with the relevant semantic/IR contracts;
* it does not become a second quantum IR.
* 
* ============================================================================
* 55. HDL INTEGRATION
* ============================================================================
* 
* HDL owns hardware behavior and structure.
* 
* Power contracts may constrain HDL/hardware designs, but this file does not
* duplicate:
* 
* modules
* signals
* nets
* clocks
* processes
* synthesis
* verification
* 
* Those remain owned by grammar/hdl/.
* 
* ============================================================================
* 56. DISTRIBUTED INTEGRATION
* ============================================================================
* 
* Power contracts may apply to distributed workloads.
* 
* The grammar permits symbolic expressions involving arbitrary workload
* scale.
* 
* It does not encode:
* 
* MAX_NODES
* MAX_POWER_DOMAINS
* MAX_WORKERS
* 
* Distributed placement and allocation remain downstream.
* 
* ============================================================================
* 57. ACCELERATOR INTEGRATION
* ============================================================================
* 
* Power contracts may apply to:
* 
* CPU
* GPU
* FPGA
* ASIC
* QPU
* accelerator
* 
* through symbolic target/capability/resource references.
* 
* No accelerator type is hard-coded into this grammar as a finite universe.
* 
* Future accelerator classes may be introduced through qualified names and
* dialect mechanisms.
* 
* ============================================================================
* 58. VENDOR EXTENSION MODEL
* ============================================================================
* 
* Vendor-specific power properties MUST NOT require permanent universal
* keywords.
* 
* Prefer:
* 
* vendor::property
* 
* or attributes/properties using qualified names.
* 
* Example:
* 
* @vendor.example(power_policy)
* 
* or:
* 
* vendor::dynamic_scaling = expression;
* 
* The semantic layer determines whether the extension is supported.
* 
* ============================================================================
* 59. DETERMINISM
* ============================================================================
* 
* Parsing MUST depend only on:
* 
* source text
* selected grammar version
* canonical lexical vocabulary
* parser configuration
* explicitly selected dialects
* 
* Parsing MUST NOT depend on:
* 
* current power consumption
* hardware availability
* physical temperature
* wall-clock time
* randomness
* environment state
* network state
* target discovery
* 
* Hardware availability is a semantic/resource-resolution concern.
* 
* ============================================================================
* 60. DIAGNOSTICS
* ============================================================================
* 
* Syntax errors MUST identify the source span of the malformed construct.
* 
* Semantic errors belong downstream and should distinguish at least:
* 
* invalid syntax
* invalid expression
* invalid power dimension
* unsatisfied power requirement
* violated power constraint
* unavailable capability
* unavailable resource
* unsupported target
* unsupported dialect extension
* 
* A target that cannot satisfy a power requirement MUST NOT cause the source
* program to be incorrectly reported as syntactically invalid.
* 
* ============================================================================
* 61. SECURITY
* ============================================================================
* 
* This grammar performs no:
* 
* hardware discovery
* filesystem access
* network access
* command execution
* environment inspection
* secret access
* 
* Vendor attributes and properties are inert syntax until validated by the
* semantic/compiler layers.
* 
* ============================================================================
* 62. SCALABILITY TEST CONTRACT
* ============================================================================
* 
* Tests for this grammar MUST include:
* 
* tiny power values
* large power values
* symbolic power values
* computed power values
* arbitrary precision/representable values
* many power contracts
* many domains
* many states
* many transitions
* deeply nested contracts
* large expressions
* distributed power contracts
* accelerator power contracts
* quantum power contracts
* future qualified properties
* 
* Tests MUST NOT establish a language maximum.
* 
* ============================================================================
* 63. HARD-CODING AUDIT
* ============================================================================
* 
* This file MUST fail review if it introduces:
* 
* MAX_POWER
* MAX_ENERGY
* MAX_POWER_DOMAINS
* MAX_POWER_STATES
* MAX_POWER_PROFILES
* MAX_POWER_TRANSITIONS
* MAX_DEVICES
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_QPUS
* MAX_ACCELERATORS
* MAX_NODES
* 
* or equivalent hidden bounds.
* 
* The following are also prohibited as universal language semantics:
* 
* fixed wattage
* fixed voltage
* fixed current
* fixed device power
* fixed number of power domains
* fixed number of power states
* 
* Explicit program values remain valid.
* 
* For example:
* 
* budget <= 100W;
* 
* is valid program intent.
* 
* It does NOT establish a universal 100 W language limit.
* 
* ============================================================================
* 64. NEGATIVE CASES
* ============================================================================
* 
* The following must be rejected syntactically when malformed:
* 
* power contract;
* 
* power contract foo {
* 
* power contract foo {
*     require;
* }
* 
* power contract foo {
*     constraint <=;
* }
* 
* power contract foo {
*     budget;
* }
* 
* power contract foo {
*     transition state;
* }
* 
* The exact diagnostic wording is owned by the parser/frontend diagnostic
* subsystem.
* 
* ============================================================================
* 65. POSITIVE CASES
* ============================================================================
* 
* Examples of intended forms:
* 
* power contract system_power {
*     budget <= power_budget;
* }
* 
* power contract accelerator_power {
*     require peak <= allowed_peak_power;
*     constraint average <= average_power_budget;
*     prefer lower_power <= requested_power;
*     hint minimize dynamic_power;
* }
* 
* power contract scalable_power<limit> {
*     budget <= limit;
* }
* 
* power profile compute {
*     dynamic = dynamic_power;
*     static = leakage_power;
*     peak <= peak_power;
* }
* 
* power domain accelerator {
*     require power_management;
* }
* 
* power state idle {
*     constraint power <= idle_power;
* }
* 
* power state active {
*     constraint power <= active_power;
* }
* 
* power transition idle to active {
*     require transition_power <= transition_budget;
* }
* 
* power scaling workload {
*     power = base_power + coefficient * workload;
* }
* 
* power contract qpu_power {
*     capability quantum::power_management;
*     resource power >= required_power;
*     require peak <= power_budget;
* }
* 
* These examples are semantic demonstrations, not fixed required vocabulary.
* 
* ============================================================================
* 66. BOUNDARY CASES
* ============================================================================
* 
* The parser must permit:
* 
* power values represented by expressions;
* symbolic values;
* generic parameters;
* qualified names;
* arbitrarily large representable numeric literals;
* arbitrarily small representable values supported by the lexical/type
* system;
* nested contracts;
* multiple profiles;
* multiple domains;
* multiple states;
* multiple transitions;
* multiple requirements;
* multiple constraints;
* multiple preferences;
* multiple hints.
* 
* The parser must not turn any of these into fixed machine capacities.
* 
* ============================================================================
* 67. COMPLETION / INTEGRATION CHECKLIST
* ============================================================================
* 
* This file is complete when all of the following are true:
* 
* LEXER
* 
* [ ] POWER is consumed from the canonical lexer vocabulary.
* 
* [ ] No POWER token is declared here.
* 
* [ ] No duplicate unit lexer is created.
* 
* [ ] General expressions use the canonical expression grammar.
* 
* [ ] General identifiers use the canonical identifier grammar.
* 
* HARDWARE
* 
* [ ] hardware.g4 delegates hardwarePowerDeclaration here.
* 
* [ ] hardware.g4 does not duplicate power rules.
* 
* [ ] No second hardware power grammar exists.
* 
* RESOURCES
* 
* [ ] hardware/resources.g4 remains owner of hardwareResourcePowerClause.
* 
* [ ] "power = expression;" remains valid there.
* 
* [ ] power.g4 does not redefine hardwareResourcePowerClause.
* 
* CAPABILITIES
* 
* [ ] hardware/capabilities.g4 remains capability declaration authority.
* 
* [ ] power.g4 only references capabilities.
* 
* TARGETS
* 
* [ ] hardware/targets.g4 remains target declaration authority.
* 
* [ ] power.g4 only references abstract targets.
* 
* TIMING
* 
* [ ] hardware/timing.g4 remains timing authority.
* 
* [ ] power.g4 does not redefine timing.
* 
* THERMAL
* 
* [ ] power.g4 does not become a thermal grammar.
* 
* [ ] Thermal semantics remain downstream.
* 
* HDL
* 
* [ ] HDL grammar remains independent.
* 
* [ ] No HDL behavioral syntax is duplicated here.
* 
* QUANTUM
* 
* [ ] No quantum operation syntax is introduced.
* 
* [ ] No quantum gate enumeration is introduced.
* 
* [ ] No second quantum IR is introduced.
* 
* [ ] quantum::ir remains canonical.
* 
* AST
* 
* [ ] Every public rule has a predetermined AST mapping.
* 
* [ ] Source spans are preserved.
* 
* [ ] No semantic information is silently discarded.
* 
* SEMANTICS
* 
* [ ] Power dimensional validation is downstream.
* 
* [ ] Resource satisfaction is downstream.
* 
* [ ] Capability satisfaction is downstream.
* 
* [ ] Target selection is downstream.
* 
* [ ] Physical enforcement is downstream.
* 
* IR
* 
* [ ] No parser-level power IR is created.
* 
* [ ] Power contracts lower through the canonical semantic/IR boundary.
* 
* COMPILER
* 
* [ ] Optimization may consume power contracts.
* 
* [ ] Scheduling may consume power contracts.
* 
* [ ] Placement may consume power contracts.
* 
* [ ] Target lowering may consume power contracts.
* 
* RUNTIME
* 
* [ ] Runtime enforcement is not performed by the parser.
* 
* [ ] Runtime measurement is not performed by the parser.
* 
* SCALABILITY
* 
* [ ] No power capacity constant exists.
* 
* [ ] No bounded power collection exists.
* 
* [ ] No fixed hardware scale is encoded.
* 
* [ ] Program values remain independent of implementation limits.
* 
* POCO-REAF
* 
* [ ] Same source-level power contract can be mapped to different targets.
* 
* [ ] Hardware realization can vary without changing source semantics.
* 
* [ ] Target failure is distinguishable from source syntax failure.
* 
* SAFETY
* 
* [ ] Grammar contains no actions.
* 
* [ ] No unsafe Rust is required.
* 
* VERSIONING
* 
* [ ] Feature status is recorded in the appropriate specification/compatibility
* registry.
* 
* [ ] Future changes are additive where possible.
* 
* [ ] Breaking changes require an explicit migration.
* 
* ============================================================================
* 68. FINAL OWNERSHIP INVARIANT
* ============================================================================
* 
* The final ownership chain is:
* 
* grammar/lexer/
*         |
*         v
* canonical tokens
*         |
*         v
* grammar/antlr/ZamaniLexer.g4
*         |
*         v
* grammar/antlr/ZamaniParser.g4
*         |
*         v
* grammar/hardware/hardware.g4
*         |
*         +--> hardware/resources.g4
*         +--> hardware/capabilities.g4
*         +--> hardware/targets.g4
*         +--> hardware/timing.g4
*         +--> hardware/power.g4
*         |
*         v
* domain-neutral AST
*         |
*         v
* semantic power/resource/capability model
*         |
*         v
* canonical compiler IR
*         |
*         +--> optimization
*         +--> scheduling
*         +--> placement
*         +--> routing
*         +--> synthesis
*         +--> resilience
*         |
*         v
* HAL
*         |
*         v
* target realization
* 
* The invariant is:
* 
* POWER SYNTAX
*      !=
* POWER ANALYSIS
*      !=
* POWER MEASUREMENT
*      !=
* POWER SCHEDULING
*      !=
* POWER MANAGEMENT
*      !=
* PHYSICAL POWER CONTROL
* 
* Each layer owns its own responsibility.
* 
* ============================================================================
* 69. POCO-REAF FINAL INVARIANT
* ============================================================================
* 
* The language expresses:
* 
* what power behavior is required,
* what power behavior is constrained,
* what power behavior is preferred,
* what power behavior is hinted,
* what capabilities are required,
* what resources are required.
* 
* The compiler/runtime decides:
* 
* how that intent is realized.
* 
* Therefore:
* 
* SAME PROGRAM
*      |
*      +--> tiny target
*      |
*      +--> medium target
*      |
*      +--> large target
*      |
*      +--> heterogeneous target
*      |
*      +--> distributed target
*      |
*      +--> quantum target
*      |
*      +--> future target
* 
* without introducing a language-level power ceiling.
* 
* ============================================================================
* END OF FILE
* ============================================================================
  */