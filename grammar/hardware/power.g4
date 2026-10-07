/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/hardware/power.g4
 *
 * GRAMMAR
 * -------
 * ZamaniHardwarePowerParser
 *
 * STATUS
 * ------
 * Canonical hardware power-intent grammar.
 *
 * RUNTIME / COMPILER
 * ------------------
 * Rust 1.97+
 * Rust 2021+
 *
 * SAFETY
 * ------
 * Action-free ANTLR4 parser grammar.
 *
 * This grammar:
 *
 *   - contains no embedded Rust;
 *   - contains no target-language actions;
 *   - contains no semantic predicates;
 *   - performs no hardware discovery;
 *   - performs no runtime execution;
 *   - performs no resource allocation;
 *   - introduces no unsafe Rust requirement.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 * Express portable, target-independent hardware power intent.
 *
 * OWNED
 * -----
 * This file owns:
 *
 *   - hardware power declarations;
 *   - power requirements;
 *   - power constraints;
 *   - power preferences;
 *   - power hints;
 *   - power budgets;
 *   - power profiles;
 *   - logical power domains;
 *   - logical power states;
 *   - logical power transitions;
 *   - power measurements as contracts;
 *   - extensible power properties;
 *   - symbolic power relationships.
 *
 * DOES NOT OWN
 * ------------
 * This file does not own:
 *
 *   - lexical token definitions;
 *   - identifier syntax;
 *   - qualified-name syntax;
 *   - general expressions;
 *   - general types;
 *   - generic resources;
 *   - generic capabilities;
 *   - generic constraints;
 *   - thermal semantics;
 *   - timing semantics;
 *   - energy-resource semantics;
 *   - physical power measurement;
 *   - voltage regulation;
 *   - clock/power management;
 *   - DVFS implementation;
 *   - device discovery;
 *   - target selection;
 *   - placement;
 *   - routing;
 *   - scheduling;
 *   - synthesis;
 *   - calibration;
 *   - runtime power management;
 *   - hardware drivers;
 *   - quantum operations;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - HAL implementation.
 *
 * DEPENDS_ON
 * ----------
 *   - ZamaniLexer
 *   - ZamaniExpressions
 *   - ZamaniNames
 *
 * EXPORTS
 * -------
 *   - hardwarePowerDeclaration
 *   - hardwarePowerItem
 *   - hardwarePowerRequirement
 *   - hardwarePowerConstraint
 *   - hardwarePowerPreference
 *   - hardwarePowerHint
 *   - hardwarePowerBudget
 *   - hardwarePowerProfile
 *   - hardwarePowerDomain
 *   - hardwarePowerState
 *   - hardwarePowerTransition
 *   - hardwarePowerMeasurementContract
 *   - hardwarePowerProperty
 *
 * CONSUMED_BY
 * -----------
 *   - grammar/hardware/hardware.g4
 *
 * AST_OWNER
 * ---------
 * Domain-neutral frontend AST.
 *
 * The grammar must map into existing generic declaration/property/
 * requirement/constraint structures where available.
 *
 * SEMANTIC_OWNER
 * --------------
 * Hardware/resource semantic analysis.
 *
 * TYPE_OWNER
 * ----------
 * Canonical type/expression semantic system.
 *
 * EFFECT_OWNER
 * ------------
 * Effects are not introduced by this grammar.
 * Any measurement/runtime effect belongs to the semantic/runtime layer.
 *
 * CAPABILITY_OWNER
 * ----------------
 * Canonical capability/resource subsystem.
 *
 * RESOURCE_OWNER
 * --------------
 * Generic resource semantics remain owned by grammar/resources/.
 *
 * CONTRACT_OWNER
 * --------------
 * Generic contracts remain owned by grammar/validation/.
 *
 * POLICY_OWNER
 * ------------
 * Generic policy semantics remain owned by grammar/policies/ and
 * grammar/security/.
 *
 * PROVENANCE_OWNER
 * ----------------
 * Canonical provenance subsystem.
 *
 * IR_OWNER
 * --------
 * No hardware-specific IR is created here.
 *
 * Lowering proceeds through the canonical semantic/IR architecture.
 *
 * QUANTUM BOUNDARY
 * ----------------
 * Power intent may constrain a quantum realization, but this grammar:
 *
 *   - does not define quantum operations;
 *   - does not define qubits;
 *   - does not define physical qubits;
 *   - does not define routing;
 *   - does not define QEC;
 *   - does not define calibration;
 *   - does not construct quantum::ir.
 *
 * Quantum semantic lowering remains:
 *
 *   source
 *     -> frontend AST
 *     -> semantic model
 *     -> quantum::ir
 *
 * HDL BOUNDARY
 * ------------
 * Power intent may constrain HDL/hardware realization.
 *
 * HDL syntax remains owned by grammar/hdl/.
 *
 * BACKEND BOUNDARY
 * ----------------
 * Backend stages consume semantic power requirements and constraints for:
 *
 *   - target feasibility;
 *   - optimization;
 *   - placement;
 *   - scheduling;
 *   - synthesis;
 *   - runtime/deployment planning.
 *
 * No backend decision is made here.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Power syntax describes PORTABLE INTENT.
 *
 * It must remain valid regardless of whether the eventual realization is:
 *
 *   - a tiny embedded target;
 *   - a CPU;
 *   - a multicore system;
 *   - a GPU;
 *   - an FPGA;
 *   - an ASIC;
 *   - an accelerator;
 *   - a QPU;
 *   - a simulator;
 *   - an HPC system;
 *   - a cluster;
 *   - a distributed system;
 *   - a cloud system;
 *   - a future architecture.
 *
 * The source does not select the physical realization.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT define:
 *
 *   MAX_POWER
 *   MAX_ENERGY
 *   MAX_POWER_DOMAINS
 *   MAX_POWER_STATES
 *   MAX_POWER_PROFILES
 *   MAX_POWER_TRANSITIONS
 *   MAX_DEVICES
 *   MAX_CPUS
 *   MAX_GPUS
 *   MAX_FPGAS
 *   MAX_QPUS
 *   MAX_NODES
 *   MAX_THREADS
 *   MAX_MEMORY
 *
 * or equivalent bounded alternatives.
 *
 * All quantities are expressions.
 *
 * All collections use unbounded grammar repetition.
 *
 * Physical limits belong to:
 *
 *   - target capabilities;
 *   - resource availability;
 *   - semantic validation;
 *   - compilation policy;
 *   - scheduling;
 *   - runtime;
 *   - deployment.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical parser-facing lexer is:
 *
 *   grammar/antlr/ZamaniLexer.g4
 *
 * The grammar therefore uses:
 *
 *   tokenVocab = ZamaniLexer;
 *
 * This file MUST NOT use ZamaniTokens directly.
 *
 * It MUST NOT define lexical tokens.
 *
 * ============================================================================
 * NAME / EXPRESSION AUTHORITY
 * ============================================================================
 *
 * Identifier and qualified-name syntax is owned by ZamaniNames.
 *
 * Expression syntax is owned by ZamaniExpressions.
 *
 * This grammar consumes those rules and never recreates them.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwarePowerParser;

options {
    tokenVocab = ZamaniLexer;
}

import
    ZamaniExpressions,
    ZamaniNames
;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * hardware.g4 exposes this rule through hardwareDeclaration.
 *
 * The declaration has deliberately small fixed syntax.
 *
 * The extensible part is inside the body.
 *
 * ============================================================================
 */

hardwarePowerDeclaration
    : hardwarePowerAttribute*
      POWER
      hardwarePowerDeclarationKind?
      identifier
      hardwarePowerParameterList?
      hardwarePowerTargetClause?
      hardwarePowerBody
    ;


/* ============================================================================
 * 2. DECLARATION KIND
 * ============================================================================
 *
 * Only vocabulary already represented by the canonical lexer is used.
 *
 * "state" and "transition" are intentionally NOT lexer keywords.
 * They remain body-level semantic constructs represented by identifiers
 * where appropriate.
 *
 * ============================================================================
 */

hardwarePowerDeclarationKind
    : CONTRACT
    | PROFILE
    | DOMAIN
    ;


/* ============================================================================
 * 3. ATTRIBUTES
 * ============================================================================
 *
 * Attributes are syntactic metadata.
 *
 * Their meaning is resolved semantically.
 *
 * ============================================================================
 */

hardwarePowerAttribute
    : AT
      qualifiedName
      (
          LPAREN
          expressionList?
          RPAREN
      )?
    ;


/* ============================================================================
 * 4. GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic parameters remain symbolic.
 *
 * They do not encode machine capacity.
 *
 * ============================================================================
 */

hardwarePowerParameterList
    : LT
      hardwarePowerParameter
      (
          COMMA
          hardwarePowerParameter
      )*
      GT
    ;

hardwarePowerParameter
    : identifier
      (
          COLON
          qualifiedName
      )?
      (
          ASSIGN
          expression
      )?
    ;


/* ============================================================================
 * 5. ABSTRACT TARGET ASSOCIATION
 * ============================================================================
 *
 * TARGET refers to an abstract compilation/execution context.
 *
 * It does not require a physical device identity.
 *
 * ============================================================================
 */

hardwarePowerTargetClause
    : TARGET
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 6. POWER BODY
 * ============================================================================
 */

hardwarePowerBody
    : LBRACE
      hardwarePowerItem*
      RBRACE
    ;


/* ============================================================================
 * 7. POWER ITEM DISPATCH
 * ============================================================================
 *
 * Every power-specific construct enters through this single dispatch rule.
 *
 * No duplicate hidden power grammar should be added to hardware.g4.
 *
 * ============================================================================
 */

hardwarePowerItem
    : hardwarePowerAttribute*
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
        | hardwarePowerMeasurementContract
        | hardwarePowerProperty
      )
    ;


/* ============================================================================
 * 8. REQUIREMENT
 * ============================================================================
 *
 * REQUIRES is the canonical repository token.
 *
 * The expression determines the actual requirement.
 *
 * ============================================================================
 */

hardwarePowerRequirement
    : REQUIRES
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 9. CONSTRAINT
 * ============================================================================
 */

hardwarePowerConstraint
    : CONSTRAINT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 10. PREFERENCE
 * ============================================================================
 */

hardwarePowerPreference
    : PREFER
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 11. HINT
 * ============================================================================
 */

hardwarePowerHint
    : HINT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 12. POWER BUDGET
 * ============================================================================
 *
 * A budget is source-level semantic information.
 *
 * It is never a compiler-wide maximum.
 *
 * ============================================================================
 */

hardwarePowerBudget
    : BUDGET
      hardwarePowerRelationalOperator
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 13. POWER PROFILE
 * ============================================================================
 *
 * Profiles are named logical descriptions.
 *
 * Property names remain open through qualifiedName.
 *
 * ============================================================================
 */

hardwarePowerProfile
    : PROFILE
      identifier
      hardwarePowerBlock
    ;


/* ============================================================================
 * 14. POWER DOMAIN
 * ============================================================================
 *
 * A power domain is a logical semantic grouping.
 *
 * It does not imply a physical voltage rail, package, die, or device.
 *
 * ============================================================================
 */

hardwarePowerDomain
    : DOMAIN
      identifier
      hardwarePowerBlock
    ;


/* ============================================================================
 * 15. POWER STATE
 * ============================================================================
 *
 * STATE is deliberately not a reserved lexer token.
 *
 * The construct is introduced by the qualified name "state".
 *
 * This keeps the lexical vocabulary extensible and avoids another global
 * keyword solely for hardware power management.
 *
 * ============================================================================
 */

hardwarePowerState
    : hardwarePowerStateKeyword
      identifier
      hardwarePowerBlock
    ;

hardwarePowerStateKeyword
    : identifier
    ;


/* ============================================================================
 * 16. POWER TRANSITION
 * ============================================================================
 *
 * TRANSITION is likewise represented through a symbolic name rather than
 * introducing a universal lexer keyword.
 *
 * Semantic validation recognizes the declaration form.
 *
 * ============================================================================
 */

hardwarePowerTransition
    : hardwarePowerTransitionKeyword
      hardwarePowerStateReference
      TO
      hardwarePowerStateReference
      hardwarePowerTransitionBlock?
      SEMICOLON
    ;

hardwarePowerTransitionKeyword
    : identifier
    ;

hardwarePowerStateReference
    : qualifiedName
    ;

hardwarePowerTransitionBlock
    : hardwarePowerBlock
    ;


/* ============================================================================
 * 17. MEASUREMENT CONTRACT
 * ============================================================================
 *
 * This is a CONTRACT concerning measurement.
 *
 * It does not perform a measurement.
 *
 * ============================================================================
 */

hardwarePowerMeasurementContract
    : MEASUREMENT
      identifier
      hardwarePowerBlock
    ;


/* ============================================================================
 * 18. GENERIC POWER PROPERTY
 * ============================================================================
 *
 * The property name is open-ended.
 *
 * This is critical for future hardware technologies.
 *
 * The language therefore does not require a new grammar release for every
 * future power-related property.
 *
 * Examples of semantic property names include:
 *
 *   static
 *   dynamic
 *   peak
 *   average
 *   sustained
 *   transient
 *   leakage
 *   switching
 *   idle
 *   profile
 *   efficiency
 *
 * These are NOT hard-coded here.
 *
 * ============================================================================
 */

hardwarePowerProperty
    : qualifiedName
      hardwarePowerRelationalOperator
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 19. COMMON BLOCK
 * ============================================================================
 */

hardwarePowerBlock
    : LBRACE
      hardwarePowerItem*
      RBRACE
    ;


/* ============================================================================
 * 20. RELATIONAL OPERATOR
 * ============================================================================
 *
 * These are canonical lexer tokens.
 *
 * No duplicate operator definitions are permitted.
 *
 * ============================================================================
 */

hardwarePowerRelationalOperator
    : ASSIGN
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    | EQUAL_EQUAL
    | NOT_EQUAL
    ;


/* ============================================================================
 * 21. INTEGRATION CONTRACT
 * ============================================================================
 *
 * hardware.g4 MUST:
 *
 *   1. import ZamaniHardwarePowerParser;
 *   2. expose hardwarePowerDeclaration exactly once from hardwareDeclaration.
 *
 * hardware.g4 MUST NOT:
 *
 *   - duplicate any rule in this file;
 *   - define another power declaration;
 *   - redefine POWER;
 *   - redefine power properties;
 *   - redefine power requirements;
 *   - redefine power constraints.
 *
 * ============================================================================
 * 22. RESOURCE BOUNDARY
 * ============================================================================
 *
 * Generic resource declarations remain owned by:
 *
 *   grammar/hardware/resources.g4
 *   grammar/resources/
 *
 * For example, a generic resource relationship may express a power value.
 *
 * This grammar instead describes a named hardware power contract.
 *
 * The semantic layer correlates the two.
 *
 * ============================================================================
 * 23. THERMAL BOUNDARY
 * ============================================================================
 *
 * Thermal syntax remains owned by:
 *
 *   grammar/hardware/thermal.g4
 *
 * Power may participate in thermal analysis downstream.
 *
 * This grammar does not define:
 *
 *   temperature;
 *   cooling;
 *   heat transfer;
 *   thermal simulation;
 *   physical thermal limits.
 *
 * ============================================================================
 * 24. ENERGY BOUNDARY
 * ============================================================================
 *
 * POWER and ENERGY are related but distinct semantic quantities.
 *
 * This grammar does not redefine generic energy-resource syntax.
 *
 * Energy semantics remain available through the generic resource and
 * semantic systems.
 *
 * ============================================================================
 * 25. QUANTUM BOUNDARY
 * ============================================================================
 *
 * A power contract can constrain a quantum realization.
 *
 * It does not define:
 *
 *   gates;
 *   qubits;
 *   circuits;
 *   physical qubit identifiers;
 *   topology;
 *   calibration;
 *   QEC;
 *   routing;
 *   scheduling.
 *
 * Those remain downstream.
 *
 * ============================================================================
 * 26. HDL BOUNDARY
 * ============================================================================
 *
 * Power properties can constrain an HDL realization.
 *
 * HDL declarations and behavior remain owned by grammar/hdl/.
 *
 * ============================================================================
 * 27. AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve:
 *
 *   - declaration kind;
 *   - declaration name;
 *   - generic parameters;
 *   - target association;
 *   - property/requirement/constraint classification;
 *   - expressions;
 *   - nesting;
 *   - source locations.
 *
 * Recommended semantic AST mapping:
 *
 *   hardwarePowerDeclaration
 *       -> PowerDeclaration
 *
 *   hardwarePowerRequirement
 *       -> Requirement
 *
 *   hardwarePowerConstraint
 *       -> Constraint
 *
 *   hardwarePowerPreference
 *       -> Preference
 *
 *   hardwarePowerHint
 *       -> Hint
 *
 *   hardwarePowerBudget
 *       -> Resource/Contract Budget
 *
 *   hardwarePowerProperty
 *       -> Property
 *
 *   hardwarePowerProfile
 *       -> Named Property Group
 *
 *   hardwarePowerDomain
 *       -> Logical Domain
 *
 *   hardwarePowerState
 *       -> Logical State
 *
 *   hardwarePowerTransition
 *       -> Logical Transition
 *
 *   hardwarePowerMeasurementContract
 *       -> Measurement Contract
 *
 * The exact existing Rust AST types should be reused when equivalent types
 * already exist.
 *
 * ============================================================================
 * 28. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *   - name resolution;
 *   - generic parameter validation;
 *   - expression typing;
 *   - dimensional/unit validation;
 *   - power/energy distinction;
 *   - requirement classification;
 *   - constraint validation;
 *   - preference classification;
 *   - capability matching;
 *   - resource matching;
 *   - target feasibility;
 *   - policy validation;
 *   - provenance;
 *   - satisfiability analysis.
 *
 * The parser MUST NOT decide whether a target satisfies a power requirement.
 *
 * ============================================================================
 * 29. SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar has no artificial finite capacity.
 *
 * These are intentionally unbounded:
 *
 *   hardwarePowerItem*
 *   hardwarePowerAttribute*
 *
 * and expression size is delegated to the canonical expression grammar.
 *
 * Therefore the grammar itself does not impose a ceiling on:
 *
 *   - power properties;
 *   - power domains;
 *   - logical states;
 *   - transitions;
 *   - contracts;
 *   - profiles;
 *   - resources;
 *   - devices;
 *   - targets.
 *
 * Actual limits are implementation/resource limits, never language constants.
 *
 * ============================================================================
 * 30. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *   - no actions;
 *   - no semantic predicates;
 *   - no randomness;
 *   - no environment access;
 *   - no hardware access;
 *   - no filesystem access;
 *   - no network access.
 *
 * Parsing is therefore determined by the source token stream and selected
 * grammar version.
 *
 * ============================================================================
 * 31. DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics should identify:
 *
 *   - missing declaration name;
 *   - malformed generic parameter;
 *   - missing body;
 *   - malformed requirement;
 *   - malformed constraint;
 *   - malformed budget;
 *   - malformed property;
 *   - malformed state;
 *   - malformed transition;
 *   - malformed measurement contract;
 *   - missing statement terminator.
 *
 * Semantic diagnostics, not parser diagnostics, should identify:
 *
 *   - unknown power property;
 *   - invalid unit;
 *   - incompatible dimensions;
 *   - impossible requirement;
 *   - unavailable capability;
 *   - unsatisfied target constraint;
 *   - conflicting power contracts.
 *
 * ============================================================================
 * 32. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The grammar must accept structurally valid forms such as:
 *
 *   power contract workload {
 *       requires power <= available_power;
 *       constraint peak_power <= power_budget;
 *       prefer power_efficiency >= desired_efficiency;
 *       hint dynamic_power;
 *       budget <= workload_budget;
 *       peak <= peak_limit;
 *       profile compute {
 *           dynamic <= dynamic_budget;
 *           static <= static_budget;
 *       }
 *   }
 *
 *   power domain compute {
 *       requires capability("power.management");
 *   }
 *
 *   power profile low_power {
 *       target hardware::power;
 *       dynamic <= dynamic_budget;
 *       leakage <= leakage_budget;
 *   }
 *
 * Exact semantic validity is checked downstream.
 *
 * ============================================================================
 * 33. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Reject syntactically malformed forms such as:
 *
 *   power contract {
 *       ...
 *   }
 *
 *   power contract workload {
 *       requires ;
 *   }
 *
 *   power contract workload {
 *       <= budget;
 *   }
 *
 *   power contract workload {
 *       budget;
 *   }
 *
 *   power contract workload {
 *       peak <= ;
 *   }
 *
 * ============================================================================
 * 34. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must generate:
 *
 *   - arbitrarily many properties;
 *   - arbitrarily many profiles;
 *   - arbitrarily many nested logical domains;
 *   - arbitrarily many symbolic constraints;
 *   - arbitrarily large expressions;
 *
 * subject only to available test resources.
 *
 * No test may assert a language-level maximum.
 *
 * ============================================================================
 * 35. COMPATIBILITY
 * ============================================================================
 *
 * Existing source compatibility must be handled by:
 *
 *   grammar/compatibility/
 *
 * This grammar must not create duplicate lexical aliases.
 *
 * In particular:
 *
 *   REQUIRE
 *
 * is NOT introduced merely for compatibility.
 *
 * The canonical spelling is:
 *
 *   REQUIRES
 *
 * ============================================================================
 * 36. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] tokenVocab is ZamaniLexer;
 * [ ] all imported rules resolve;
 * [ ] no local lexical tokens exist;
 * [ ] no K_* token aliases exist;
 * [ ] no undefined parser rules remain;
 * [ ] no undefined lexer tokens remain;
 * [ ] hardware.g4 imports this parser exactly once;
 * [ ] hardware.g4 dispatches hardwarePowerDeclaration exactly once;
 * [ ] generic resources remain owned elsewhere;
 * [ ] thermal semantics remain owned elsewhere;
 * [ ] energy semantics remain distinct;
 * [ ] no hardware capacity constants exist;
 * [ ] no physical device identity is required;
 * [ ] AST mapping is documented;
 * [ ] semantic ownership is documented;
 * [ ] IR ownership is documented;
 * [ ] quantum::ir remains untouched by the grammar;
 * [ ] positive tests pass;
 * [ ] negative tests pass;
 * [ ] boundary tests pass;
 * [ ] scalability tests pass;
 * [ ] compatibility tests pass;
 * [ ] generated Rust compiles under Rust 1.97+;
 * [ ] the generated implementation contains no unsafe Rust.
 *
 * ============================================================================
 */