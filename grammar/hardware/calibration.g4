/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/calibration.g4
 *
 * Grammar:
 *     ZamaniHardwareCalibrationParser
 *
 * Status:
 *     CANONICAL HARDWARE-CALIBRATION CONTRACT GRAMMAR
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     Action-free ANTLR grammar.
 *     No embedded Rust.
 *     No semantic predicates.
 *     No I/O.
 *     No hardware discovery.
 *     No runtime execution.
 *     No unsafe implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines SOURCE-LEVEL CALIBRATION INTENT for hardware contracts.
 *
 * It describes what calibration evidence, freshness, validity, provenance,
 * characterization, or calibration capability a realization may require,
 * prefer, constrain, expose, or reference.
 *
 * It does NOT contain calibration measurements.
 *
 * It does NOT perform calibration.
 *
 * It does NOT query a device.
 *
 * It does NOT select a calibration snapshot.
 *
 * It does NOT contain provider APIs.
 *
 * It does NOT contain physical device state.
 *
 * It does NOT create quantum::ir.
 *
 * It does NOT create a second calibration IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - hardware calibration contracts;
 *   - calibration requirements;
 *   - calibration constraints;
 *   - calibration preferences;
 *   - calibration hints;
 *   - calibration capability declarations;
 *   - calibration references;
 *   - calibration selectors;
 *   - calibration freshness intent;
 *   - calibration validity intent;
 *   - calibration provenance requirements;
 *   - calibration characterization requirements;
 *   - calibration confidence/uncertainty requirements;
 *   - calibration resource scope intent;
 *   - calibration property expressions;
 *   - calibration applicability;
 *   - calibration compatibility intent;
 *   - calibration observability intent;
 *   - calibration policy metadata;
 *   - extensible calibration properties.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - calibration measurements;
 *   - calibration snapshot storage;
 *   - calibration acquisition;
 *   - calibration experiments;
 *   - provider SDKs;
 *   - device APIs;
 *   - authentication;
 *   - secrets;
 *   - hardware discovery;
 *   - physical device IDs;
 *   - physical qubit IDs;
 *   - physical topology;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - QEC implementation;
 *   - ZQN implementation;
 *   - benchmarking implementation;
 *   - runtime telemetry;
 *   - HAL implementation;
 *   - canonical quantum::ir;
 *   - a second calibration IR.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * ZamaniTokens
 *   |
 *   v
 * hardware/calibration.g4
 *   |
 *   v
 * domain-neutral frontend AST
 *   |
 *   v
 * semantic calibration requirement/contract
 *   |
 *   +--------------------+
 *   |                    |
 *   v                    v
 * resource analysis   target compatibility
 *   |                    |
 *   +----------+---------+
 *              |
 *              v
 *      calibration resolver
 *              |
 *              v
 *      CalibrationSnapshot
 *              |
 *       +------+------+--------+
 *       |             |        |
 *       v             v        v
 *    routing      scheduling   ZQN
 *       |             |        |
 *       +-------------+--------+
 *                     |
 *                     v
 *                    HAL
 *                     |
 *                     v
 *                  runtime
 *
 * `CalibrationSnapshot` remains owned by the existing Rust calibration
 * subsystem. This grammar does not redefine it.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Calibration syntax MUST describe portable intent.
 *
 * Valid source-level concepts include:
 *
 *     requires calibration;
 *     requires calibration freshness <= allowed_age;
 *     requires calibration confidence >= required_confidence;
 *     requires calibration provenance::trusted;
 *     prefer calibration method::measured;
 *     constraint calibration validity >= required_interval;
 *
 * The grammar MUST NOT require:
 *
 *     a particular device;
 *     a particular provider;
 *     a particular physical qubit;
 *     a particular calibration database;
 *     a particular calibration timestamp;
 *     a particular calibration record;
 *     a particular vendor API;
 *     a particular backend.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level calibration capacity limits.
 *
 * Forbidden universal limits include:
 *
 *     MAX_CALIBRATIONS
 *     MAX_SNAPSHOTS
 *     MAX_QUBITS
 *     MAX_GATE_CALIBRATIONS
 *     MAX_COUPLINGS
 *     MAX_PARAMETERS
 *     MAX_METRICS
 *     MAX_RESOURCES
 *     MAX_DEVICES
 *
 * A finite number appearing in source is program semantics.
 *
 * For example:
 *
 *     validity <= 24h;
 *
 * is a source requirement.
 *
 * It does NOT establish a Zamani-wide 24-hour calibration limit.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * REQUIREMENT
 *
 *     Mandatory semantic intent.
 *
 * CONSTRAINT
 *
 *     Mandatory restriction on a realization.
 *
 * PREFERENCE
 *
 *     Non-binding optimization guidance.
 *
 * HINT
 *
 *     Advisory information.
 *
 * The grammar preserves these distinctions.
 *
 * ============================================================================
 * CALIBRATION VS CALIBRATION STATE
 * ============================================================================
 *
 * This grammar describes CALIBRATION INTENT.
 *
 * Existing Rust code describes CALIBRATION STATE/EVIDENCE:
 *
 *     src/quantum/hardware/calibration.rs
 *
 * Existing ZQN code describes calibration semantics:
 *
 *     src/quantum/zqn/calibration/
 *
 * The grammar MUST NOT duplicate either representation.
 *
 * ============================================================================
 * CALIBRATION SNAPSHOT BOUNDARY
 * ============================================================================
 *
 * A source program may express:
 *
 *     requires calibration snapshot;
 *
 * or:
 *
 *     requires calibration freshness <= allowed_age;
 *
 * but the parser does not construct a CalibrationSnapshot.
 *
 * The downstream semantic layer resolves:
 *
 *     source intent
 *         ->
 *     calibration requirement
 *         ->
 *     available calibration evidence
 *         ->
 *     validated CalibrationSnapshot
 *
 * ============================================================================
 * FRESHNESS
 * ============================================================================
 *
 * Freshness is expressed symbolically or dimensionally.
 *
 * Examples:
 *
 *     freshness <= allowed_age;
 *     age <= calibration_age_budget;
 *     validity_remaining >= required_validity;
 *
 * The grammar does not impose a universal freshness period.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Provenance is declarative.
 *
 * Examples:
 *
 *     provenance = measured;
 *     provenance = provider_verified;
 *     provenance = imported;
 *
 * The grammar does not authenticate provenance.
 *
 * Authentication and trust belong to security and calibration validation.
 *
 * ============================================================================
 * QUALITY / CONFIDENCE
 * ============================================================================
 *
 * Calibration quality may be expressed through:
 *
 *     confidence;
 *     uncertainty;
 *     sample_count;
 *     fidelity;
 *     error;
 *     validity;
 *     evidence;
 *
 * These are expressions, not parser-level numerical limits.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * The following are intentionally NOT part of this grammar:
 *
 *     physical qubit 17;
 *     backend ibm_x;
 *     device arn;
 *     PCI address;
 *     provider API;
 *     pulse duration hard-coded for one device;
 *     vendor calibration schema.
 *
 * Vendor-specific information belongs in explicit dialects or downstream
 * target descriptions.
 *
 * ============================================================================
 * KEYWORD POLICY
 * ============================================================================
 *
 * `CALIBRATION` is a canonical language keyword once added to the shared
 * lexical vocabulary.
 *
 * Calibration property names remain extensible.
 *
 * Examples:
 *
 *     freshness
 *     age
 *     validity
 *     provenance
 *     confidence
 *     uncertainty
 *     sample_count
 *     method
 *     source
 *     scope
 *     revision
 *     schema
 *     characterization
 *
 * These should NOT become an ever-growing global keyword list.
 *
 * ============================================================================
 * SHARED PARSER CONTRACT
 * ============================================================================
 *
 * This file consumes:
 *
 *     hardwareExpression
 *     hardwareQualifiedName
 *     hardwarePropertyStatement
 *     hardwareRelationOperator
 *     hardwareLogicalOperator
 *
 * from the hardware parser composition layer.
 *
 * It does not create a competing expression grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar maps conceptually to:
 *
 *     HardwareCalibrationDeclaration
 *         {
 *             name
 *             target
 *             items
 *             attributes
 *             source_span
 *         }
 *
 * Calibration items map conceptually to:
 *
 *     CalibrationRequirement
 *     CalibrationConstraint
 *     CalibrationPreference
 *     CalibrationHint
 *     CalibrationCapability
 *     CalibrationReference
 *     CalibrationSelector
 *     CalibrationProperty
 *
 * The exact Rust AST types remain owned by the frontend AST subsystem.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *   - resolve calibration references;
 *   - validate calibration property names;
 *   - validate units;
 *   - validate dimensions;
 *   - validate confidence ranges;
 *   - validate uncertainty semantics;
 *   - validate freshness semantics;
 *   - validate validity intervals;
 *   - distinguish requirement/constraint/preference/hint;
 *   - preserve unknown future properties;
 *   - resolve calibration applicability;
 *   - detect contradictory requirements;
 *   - evaluate target compatibility;
 *   - preserve source provenance.
 *
 * The parser itself performs none of these tasks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file MUST NOT create a calibration-specific IR.
 *
 * The canonical lowering path is:
 *
 *     calibration syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic calibration contract
 *          |
 *          +--> resource analysis
 *          +--> target compatibility
 *          +--> calibration resolver
 *          |
 *          v
 *     canonical compiler representation
 *
 * If calibration affects quantum execution:
 *
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *
 * remains the canonical quantum boundary.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may use calibration requirements to:
 *
 *   - reject an incompatible target;
 *   - select an appropriate calibration-aware compilation path;
 *   - preserve calibration dependencies;
 *   - request calibration evidence;
 *   - constrain routing;
 *   - constrain scheduling;
 *   - trigger target compatibility validation.
 *
 * It must not turn calibration syntax into direct device commands.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime may:
 *
 *   - acquire calibration state;
 *   - resolve a calibration snapshot;
 *   - validate freshness;
 *   - validate provenance;
 *   - reject stale evidence;
 *   - report unavailable calibration;
 *   - select a valid runtime realization.
 *
 * These are runtime operations, not parser operations.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Calibration syntax MUST NOT carry:
 *
 *   - API keys;
 *   - access tokens;
 *   - passwords;
 *   - private keys;
 *   - authentication headers;
 *   - secrets.
 *
 * Calibration provenance does not equal authorization.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * It MUST NOT depend on:
 *
 *   - current time;
 *   - random values;
 *   - hardware state;
 *   - provider state;
 *   - filesystem state;
 *   - network state;
 *   - environment variables.
 *
 * Freshness evaluation occurs downstream.
 *
 * ============================================================================
 */


/* ============================================================================
 * GRAMMAR
 * ========================================================================== */

parser grammar ZamaniHardwareCalibrationParser;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Canonical hardware calibration declaration.
 *
 * Examples:
 *
 *     calibration qpu_requirements {
 *         requires calibration::available;
 *         requires freshness <= allowed_age;
 *     }
 *
 *     calibration execution_policy {
 *         prefer method = measured;
 *         constraint confidence >= required_confidence;
 *     }
 *
 * `CALIBRATION` is a shared lexical token and must be added to the canonical
 * lexer vocabulary before this grammar is composed.
 */
hardwareCalibrationDeclaration
    : K_CALIBRATION
      hardwareCalibrationName?
      hardwareCalibrationTarget?
      hardwareCalibrationBody
      SEMICOLON?
    ;


/* ============================================================================
 * 2. NAME
 * ========================================================================== */

hardwareCalibrationName
    : IDENTIFIER
    ;


/* ============================================================================
 * 3. TARGET
 * ========================================================================== */

/**
 * Target is an abstract semantic subject.
 *
 * It is NOT a physical device identifier.
 */
hardwareCalibrationTarget
    : K_FOR
      hardwareQualifiedName
    ;


/* ============================================================================
 * 4. BODY
 * ========================================================================== */

hardwareCalibrationBody
    : LBRACE
      hardwareCalibrationItem*
      RBRACE
    ;


/* ============================================================================
 * 5. ITEM DISPATCH
 * ========================================================================== */

hardwareCalibrationItem
    : hardwareCalibrationRequirement
    | hardwareCalibrationConstraint
    | hardwareCalibrationPreference
    | hardwareCalibrationHint
    | hardwareCalibrationCapability
    | hardwareCalibrationReference
    | hardwareCalibrationSelector
    | hardwareCalibrationProperty
    | hardwareCalibrationProfile
    | hardwareCalibrationContract
    | hardwareCalibrationAssertion
    | hardwarePropertyStatement
    ;


/* ============================================================================
 * 6. REQUIREMENTS
 * ========================================================================== */

hardwareCalibrationRequirement
    : K_REQUIRES
      hardwareCalibrationCondition
      SEMICOLON
    ;


/* ============================================================================
 * 7. CONSTRAINTS
 * ========================================================================== */

hardwareCalibrationConstraint
    : K_CONSTRAINT
      hardwareCalibrationCondition
      SEMICOLON
    ;


/* ============================================================================
 * 8. PREFERENCES
 * ========================================================================== */

hardwareCalibrationPreference
    : K_PREFER
      hardwareCalibrationCondition
      SEMICOLON
    ;


/* ============================================================================
 * 9. HINTS
 * ========================================================================== */

hardwareCalibrationHint
    : K_HINT
      hardwareCalibrationCondition
      SEMICOLON
    ;


/* ============================================================================
 * 10. CONDITION
 * ============================================================================
 *
 * Boolean structure is retained for semantic analysis.
 *
 * Examples:
 *
 *     confidence >= required_confidence
 *     freshness <= allowed_age
 *     calibration::available
 *     confidence >= required_confidence
 *         and freshness <= allowed_age
 *
 * The parser does not evaluate the condition.
 */

hardwareCalibrationCondition
    : hardwareCalibrationPredicate
      (
          hardwareLogicalOperator
          hardwareCalibrationPredicate
      )*
    ;


hardwareCalibrationPredicate
    : hardwareCalibrationExpression
      (
          hardwareRelationOperator
          hardwareCalibrationExpression
      )?
    ;


/* ============================================================================
 * 11. EXPRESSION BRIDGE
 * ============================================================================
 *
 * Calibration values remain ordinary Zamani expressions.
 *
 * This file deliberately does not define a second expression language.
 */

hardwareCalibrationExpression
    : hardwareExpression
    | hardwareQualifiedName
    ;


/* ============================================================================
 * 12. CAPABILITY
 * ========================================================================== */

/**
 * Calibration capability expresses what a realization can provide.
 *
 * Examples:
 *
 *     capability calibration::snapshots;
 *     capability calibration::provenance;
 *     capability calibration::versioning;
 *     capability calibration::freshness;
 */
hardwareCalibrationCapability
    : K_CAPABILITY
      hardwareCalibrationReferenceExpression
      SEMICOLON
    ;


hardwareCalibrationReferenceExpression
    : hardwareQualifiedName
    ;


/* ============================================================================
 * 13. CALIBRATION REFERENCE
 * ========================================================================== */

/**
 * A reference identifies an abstract calibration class/resource.
 *
 * It does NOT resolve a concrete snapshot.
 */
hardwareCalibrationReference
    : K_REFERENCE
      hardwareQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 14. CALIBRATION SELECTOR
 * ============================================================================
 *
 * Selectors describe semantic selection criteria.
 *
 * They do not select physical hardware during parsing.
 */

hardwareCalibrationSelector
    : K_SELECT
      hardwareCalibrationSelectorBody
    ;


hardwareCalibrationSelectorBody
    : LBRACE
      hardwareCalibrationSelectorItem*
      RBRACE
    ;


hardwareCalibrationSelectorItem
    : hardwareCalibrationProperty
    | hardwareCalibrationCondition
      SEMICOLON
    ;


/* ============================================================================
 * 15. PROPERTY
 * ============================================================================
 *
 * Generic properties deliberately remain extensible.
 */

hardwareCalibrationProperty
    : hardwareCalibrationPropertyName
      hardwareCalibrationPropertyOperator
      hardwareCalibrationExpression
      SEMICOLON
    ;


hardwareCalibrationPropertyOperator
    : ASSIGN
    | hardwareRelationOperator
    ;


hardwareCalibrationPropertyName
    : IDENTIFIER
    | K_AVAILABILITY
    | K_CAPACITY
    | K_CONFIDENCE
    | K_ERROR
    | K_FIDELITY
    | K_FRESHNESS
    | K_METHOD
    | K_PROVENANCE
    | K_REVISION
    | K_SCHEMA
    | K_SCOPE
    | K_SOURCE
    | K_STATUS
    | K_UNCERTAINTY
    | K_VALIDITY
    ;


/* ============================================================================
 * 16. PROFILE
 * ============================================================================
 *
 * Profiles are reusable semantic calibration policies.
 */

hardwareCalibrationProfile
    : K_PROFILE
      IDENTIFIER
      hardwareCalibrationBody
    ;


/* ============================================================================
 * 17. CONTRACT
 * ============================================================================
 *
 * Contracts group calibration obligations.
 */

hardwareCalibrationContract
    : K_CONTRACT
      IDENTIFIER
      hardwareCalibrationBody
    ;


/* ============================================================================
 * 18. ASSERTION
 * ============================================================================
 */

hardwareCalibrationAssertion
    : K_ASSERT
      hardwareCalibrationCondition
      SEMICOLON
    ;


/* ============================================================================
 * 19. STANDARD CALIBRATION PROPERTY NAMES
 * ============================================================================
 *
 * These names are semantically standardized but remain properties rather than
 * independent grammar constructs.
 *
 * Canonical downstream meanings include:
 *
 *     available
 *     freshness
 *     age
 *     validity
 *     valid_from
 *     valid_until
 *     confidence
 *     uncertainty
 *     sample_count
 *     method
 *     provenance
 *     source
 *     revision
 *     schema
 *     scope
 *     characterization
 *     evidence
 *     measured
 *     derived
 *     estimated
 *     imported
 *
 * Future properties remain expressible through IDENTIFIER.
 */


/* ============================================================================
 * 20. COLLECTIONS
 * ============================================================================
 *
 * Collections are structurally unbounded by the grammar.
 */

hardwareCalibrationReferenceList
    : hardwareQualifiedName
      (
          COMMA
          hardwareQualifiedName
      )*
    ;


hardwareCalibrationPropertyList
    : hardwareCalibrationProperty+
    ;


/* ============================================================================
 * 21. CALIBRATION EVIDENCE INTENT
 * ============================================================================
 *
 * This describes the class of evidence required.
 *
 * It does NOT embed evidence itself.
 */

hardwareCalibrationEvidence
    : K_EVIDENCE
      hardwareCalibrationEvidenceBody
    ;


hardwareCalibrationEvidenceBody
    : LBRACE
      hardwareCalibrationEvidenceItem*
      RBRACE
    ;


hardwareCalibrationEvidenceItem
    : hardwareCalibrationProperty
    | hardwareCalibrationCondition
      SEMICOLON
    ;


/* ============================================================================
 * 22. FRESHNESS CONTRACT
 * ============================================================================
 *
 * Explicit convenience syntax for freshness.
 *
 * Examples:
 *
 *     freshness <= allowed_age;
 *     freshness >= minimum_validity;
 */

hardwareCalibrationFreshness
    : K_FRESHNESS
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON
    ;


/* ============================================================================
 * 23. VALIDITY CONTRACT
 * ============================================================================
 */

hardwareCalibrationValidity
    : K_VALIDITY
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON
    ;


/* ============================================================================
 * 24. CONFIDENCE CONTRACT
 * ============================================================================
 */

hardwareCalibrationConfidence
    : K_CONFIDENCE
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON
    ;


/* ============================================================================
 * 25. UNCERTAINTY CONTRACT
 * ============================================================================
 */

hardwareCalibrationUncertainty
    : K_UNCERTAINTY
      hardwareRelationOperator
      hardwareExpression
      SEMICOLON
    ;


/* ============================================================================
 * 26. METHOD CONTRACT
 * ============================================================================
 */

hardwareCalibrationMethod
    : K_METHOD
      ASSIGN
      hardwareQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 27. PROVENANCE CONTRACT
 * ============================================================================
 */

hardwareCalibrationProvenance
    : K_PROVENANCE
      ASSIGN
      hardwareQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 28. SCOPE CONTRACT
 * ============================================================================
 */

hardwareCalibrationScope
    : K_SCOPE
      ASSIGN
      hardwareQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 29. REVISION CONTRACT
 * ============================================================================
 */

hardwareCalibrationRevision
    : K_REVISION
      ASSIGN
      hardwareExpression
      SEMICOLON
    ;


/* ============================================================================
 * 30. SCHEMA CONTRACT
 * ============================================================================
 */

hardwareCalibrationSchema
    : K_SCHEMA
      ASSIGN
      hardwareQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 31. SOURCE CONTRACT
 * ============================================================================
 */

hardwareCalibrationSource
    : K_SOURCE
      ASSIGN
      hardwareQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 32. STATUS CONTRACT
 * ============================================================================
 */

hardwareCalibrationStatus
    : K_STATUS
      ASSIGN
      hardwareQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 33. CALIBRATION RESOURCE SCOPE
 * ============================================================================
 *
 * A calibration contract may state that calibration applies to an abstract
 * resource class.
 *
 * It must never force a physical resource identifier.
 */

hardwareCalibrationResource
    : K_RESOURCE
      hardwareQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 34. TARGET COMPATIBILITY
 * ============================================================================
 *
 * Compatibility is semantic intent.
 *
 * Actual compatibility checking remains in:
 *
 *     src/quantum/hardware/compatibility.rs
 *
 * and the corresponding target/ZQN compatibility layers.
 */

hardwareCalibrationCompatibility
    : K_COMPATIBILITY
      hardwareCalibrationCondition
      SEMICOLON
    ;


/* ============================================================================
 * 35. CHARACTERIZATION
 * ============================================================================
 *
 * Characterization is deliberately represented as a requirement/property
 * boundary. Experimental protocols remain downstream.
 */

hardwareCalibrationCharacterization
    : K_CHARACTERIZATION
      hardwareCalibrationCharacterizationBody
    ;


hardwareCalibrationCharacterizationBody
    : LBRACE
      hardwareCalibrationProperty*
      RBRACE
    ;


/* ============================================================================
 * 36. EXTENSION POINT
 * ============================================================================
 *
 * Future calibration concepts should normally use qualified properties rather
 * than requiring permanent core syntax.
 */

hardwareCalibrationExtensionProperty
    : hardwareQualifiedName
      ASSIGN
      hardwareExpression
      SEMICOLON
    ;


/* ============================================================================
 * 37. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It owns hardware calibration intent.
 * [x] It does not own calibration state.
 * [x] It does not own calibration acquisition.
 * [x] It does not own device discovery.
 * [x] It does not own provider APIs.
 * [x] It does not own QEC.
 * [x] It does not own ZQN.
 * [x] It does not create a second IR.
 * [x] It does not impose capacity limits.
 * [x] It supports symbolic quantities.
 * [x] It supports requirements.
 * [x] It supports constraints.
 * [x] It supports preferences.
 * [x] It supports hints.
 * [x] It supports capability intent.
 * [x] It supports provenance intent.
 * [x] It supports freshness intent.
 * [x] It supports validity intent.
 * [x] It supports confidence/uncertainty intent.
 * [x] It supports future extensibility.
 * [x] It is deterministic.
 * [x] It contains no embedded Rust.
 * [x] It requires no unsafe Rust.
 *
 * Integration completion additionally requires:
 *
 * [ ] canonical CALIBRATION lexer token;
 * [ ] composition into ZamaniHardwareParser;
 * [ ] hardwareItem dispatch;
 * [ ] AST mapping;
 * [ ] semantic mapping;
 * [ ] conformance tests;
 * [ ] Rust 1.97.1 frontend validation.
 *
 * These integration items are deliberately outside this leaf grammar so that
 * this file remains independently stable.
 *
 * ============================================================================
 */