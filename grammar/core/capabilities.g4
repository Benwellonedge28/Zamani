/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/capabilities.g4
 *
 * Purpose:
 *     Canonical parser grammar for source-level capability declarations and
 *     capability references.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions and requires no `unsafe`.
 *     The Zamani compiler implementation MUST use safe Rust only.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This file owns ONLY the SOURCE SYNTAX of capabilities.
 *
 * A capability expresses a named computational property, ability, facility,
 * semantic feature, execution feature, language feature, or extension
 * contract.
 *
 * Examples:
 *
 *     capability zamani::quantum::dynamic_control;
 *     capability zamani::quantum::dynamic_control version 1.2.0;
 *     capability zamani::compute::parallel;
 *     capability future::example;
 *
 * A capability is SOURCE INTENT.
 *
 * It is NOT:
 *
 *     - a physical device;
 *     - a hardware identifier;
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a physical qubit;
 *     - a machine topology;
 *     - a resource allocation;
 *     - a scheduler decision;
 *     - a routing decision;
 *     - a calibration record;
 *     - a backend selection;
 *     - a runtime authorization token.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - capability declaration syntax;
 *     - capability reference syntax;
 *     - capability identity structure;
 *     - optional capability version requirement;
 *     - capability lists;
 *     - optional capability aliases where explicitly supported;
 *     - capability annotations at the capability syntax boundary;
 *     - syntactic composition of capability predicates;
 *     - syntactic capability attributes.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical identifiers;
 *     - keywords;
 *     - Unicode identifier rules;
 *     - identifier normalization;
 *     - version semantics;
 *     - semantic version compatibility;
 *     - capability registry;
 *     - capability discovery;
 *     - target capability discovery;
 *     - resource allocation;
 *     - hardware discovery;
 *     - backend selection;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime execution;
 *     - security authorization;
 *     - quantum::ir;
 *     - classical IR;
 *     - HDL IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Capability syntax MUST remain independent of machine scale.
 *
 * It MUST NOT contain:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTERS
 *     MAX_TENSOR_DIMENSIONS
 *     MAX_CAPABILITIES
 *
 * A capability may describe a property that is available on:
 *
 *     one machine
 *     many machines
 *     one accelerator
 *     many accelerators
 *     one quantum processor
 *     many quantum processors
 *     a simulator
 *     a distributed system
 *     an embedded system
 *     a future architecture
 *
 * without changing this grammar.
 *
 * ============================================================================
 * FUNDAMENTAL SEPARATION
 * ============================================================================
 *
 * Capability:
 *
 *     "What can an execution environment do?"
 *
 * Requirement:
 *
 *     "What capability does this program require?"
 *
 * Resource:
 *
 *     "What resource is available or requested?"
 *
 * Constraint:
 *
 *     "What conditions must a realization satisfy?"
 *
 * Preference:
 *
 *     "Which valid realization is preferred?"
 *
 * Target:
 *
 *     "What execution target/context is being described?"
 *
 * These concepts MUST NOT be collapsed.
 *
 * In particular:
 *
 *     capability quantum
 *
 * MUST NOT imply:
 *
 *     use device X
 *
 *     use N qubits
 *
 *     use topology Y
 *
 *     use backend Z
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     ZamaniLexer
 *          |
 *          v
 *     capabilities.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic capability resolution
 *          |
 *          +--> capability registry
 *          +--> version compatibility
 *          +--> requirement analysis
 *          +--> resource analysis
 *          +--> effect analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          |
 *          v
 *     compilation / optimization / routing / scheduling
 *          |
 *          v
 *     hardware / runtime / deployment
 *
 * This dependency direction MUST NOT be reversed.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum capabilities are source-level names.
 *
 * For example:
 *
 *     zamani::quantum::dynamic_control
 *     zamani::quantum::mid_circuit_measurement
 *     zamani::quantum::logical_qubits
 *
 * are capability identities.
 *
 * This grammar does NOT import:
 *
 *     quantum::ir
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
 *     calibration
 *
 * Semantic analysis may later resolve a capability to the appropriate
 * quantum semantic representation.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Capability names MUST be open-ended.
 *
 * This grammar therefore MUST NOT define:
 *
 *     quantumCapability
 *     gpuCapability
 *     cpuCapability
 *     fpgaCapability
 *     vendorCapability
 *
 * as closed enumerations.
 *
 * New computational domains and capabilities are represented using ordinary
 * qualified names.
 *
 * This permits future capabilities without changing this file.
 *
 * ============================================================================
 * VERSION BOUNDARY
 * ============================================================================
 *
 * Capability version syntax is source syntax.
 *
 * Version compatibility is semantic.
 *
 * This grammar MAY parse:
 *
 *     1
 *     1.2
 *     1.2.3
 *     1.2.3-alpha
 *     1.2.3+build
 *
 * and comparison/range operators.
 *
 * It MUST NOT decide whether a version is semantically compatible.
 *
 * For example:
 *
 *     capability foo version >= 1.2;
 *
 * is syntactically valid.
 *
 * Whether implementation version 2.0 satisfies that requirement belongs to
 * semantic compatibility analysis.
 *
 * ============================================================================
 * VERSION SCALABILITY
 * ============================================================================
 *
 * Numeric components are consumed from the canonical numeric token.
 *
 * No grammar rule imposes a fixed number of capabilities or declarations.
 *
 * Repetition uses:
 *
 *     *
 *     +
 *
 * rather than finite bounds.
 *
 * ============================================================================
 * IDENTIFIER BOUNDARY
 * ============================================================================
 *
 * The canonical `Names` grammar owns:
 *
 *     identifier
 *     qualifiedName
 *
 * This file MUST NOT recreate identifier syntax.
 *
 * Therefore:
 *
 *     identifier
 *     qualifiedName
 *
 * are imported/shared through the canonical parser grammar composition
 * mechanism.
 *
 * ============================================================================
 * ANNOTATION BOUNDARY
 * ============================================================================
 *
 * Capability annotations belong to the general annotation system.
 *
 * This file MUST NOT redefine:
 *
 *     @identifier(...)
 *
 * syntax.
 *
 * Where annotations are accepted, this grammar consumes the canonical
 * `attributes` / `attribute` rule supplied by the core grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser should produce a source-level capability representation
 * containing, conceptually:
 *
 *     identity
 *     version constraint
 *     annotations
 *     source span
 *
 * Identity is preserved exactly enough for source diagnostics and later
 * semantic resolution.
 *
 * The AST MUST NOT contain:
 *
 *     device identity
 *     hardware state
 *     resource allocation
 *     runtime capability token
 *     scheduler state
 *     target selection
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether the capability exists;
 *     - whether it is declared by an extension;
 *     - what namespace it belongs to;
 *     - what its semantic kind is;
 *     - what version means;
 *     - whether the version constraint is satisfiable;
 *     - whether it is applicable to a computation;
 *     - whether it conflicts with another capability;
 *     - whether it is provided by an execution context.
 *
 * None of those decisions belong in this grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar:
 *
 *     - contains no semantic predicates;
 *     - contains no actions;
 *     - performs no I/O;
 *     - performs no network access;
 *     - performs no hardware discovery;
 *     - performs no runtime calls;
 *     - contains no random behavior.
 *
 * Parsing is therefore deterministic for a deterministic token stream.
 *
 * ============================================================================
 */

parser grammar Capabilities;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * CAPABILITY DECLARATION
 * ============================================================================
 *
 * Canonical declaration:
 *
 *     capability foo;
 *
 *     capability foo::bar;
 *
 *     capability foo::bar version 1.2.3;
 *
 *     capability foo::bar version >= 1.2;
 *
 *     capability foo::bar version [1.2, 2.0];
 *
 * The keyword `capability` MUST be supplied by the canonical lexer.
 *
 * It is deliberately not accepted as an arbitrary identifier because a
 * capability declaration is a language-level declaration.
 */

capabilityDeclaration
    : CAPABILITY
      capabilityName
      capabilityVersionClause?
      capabilityAttributeList?
      SEMI
    ;


/* ============================================================================
 * CAPABILITY REFERENCE
 * ============================================================================
 *
 * A reference is used by requirements, preferences, effects, constraints,
 * targets, execution declarations, extensions, and other semantic consumers.
 *
 * A reference does not declare the capability.
 */

capabilityReference
    : capabilityName
      capabilityVersionClause?
    ;


/* ============================================================================
 * CAPABILITY NAME
 * ============================================================================
 *
 * Capability names use the canonical qualified-name model.
 *
 * Examples:
 *
 *     quantum
 *     quantum::dynamic_control
 *     zamani::quantum::dynamic_control
 *     future::compute::new_architecture
 *
 * The grammar does not assign meaning to any namespace.
 */

capabilityName
    : qualifiedName
    ;


/* ============================================================================
 * CAPABILITY NAME LIST
 * ============================================================================
 */

capabilityNameList
    : capabilityName
      (COMMA capabilityName)*
    ;


/* ============================================================================
 * CAPABILITY REFERENCE LIST
 * ============================================================================
 */

capabilityReferenceList
    : capabilityReference
      (COMMA capabilityReference)*
    ;


/* ============================================================================
 * OPTIONAL CAPABILITY REFERENCE LIST
 * ============================================================================
 */

optionalCapabilityReferenceList
    : capabilityReferenceList?
    ;


/* ============================================================================
 * CAPABILITY DECLARATION LIST
 * ============================================================================
 *
 * There is deliberately no fixed maximum.
 */

capabilityDeclarationList
    : capabilityDeclaration+
    ;


/* ============================================================================
 * CAPABILITY VERSION CLAUSE
 * ============================================================================
 *
 * Version syntax is deliberately separated from capability identity.
 *
 * Examples:
 *
 *     version 1
 *     version 1.2
 *     version 1.2.3
 *     version >= 1.2
 *     version <= 2.0
 *     version > 1.0
 *     version < 3.0
 *     version 1.2 .. 2.0
 *
 * The semantic interpretation is downstream.
 */

capabilityVersionClause
    : VERSION capabilityVersionExpression
    ;


/* ============================================================================
 * CAPABILITY VERSION EXPRESSION
 * ============================================================================
 *
 * The grammar supports composable constraints without imposing a semantic
 * policy such as "major version compatibility".
 */

capabilityVersionExpression
    : capabilityVersionConstraint
    | capabilityVersionRange
    | capabilityVersionSet
    | capabilityVersionReference
    ;


/* ============================================================================
 * SINGLE VERSION CONSTRAINT
 * ============================================================================
 */

capabilityVersionConstraint
    : capabilityVersionComparator?
      capabilityVersion
    ;


/* ============================================================================
 * VERSION COMPARATORS
 * ============================================================================
 *
 * Operators are interpreted semantically.
 */

capabilityVersionComparator
    : LT
    | LE
    | GT
    | GE
    | EQ
    | NE
    ;


/* ============================================================================
 * VERSION RANGE
 * ============================================================================
 *
 * Inclusive range:
 *
 *     [1.0, 2.0]
 *
 * Exclusive range:
 *
 *     (1.0, 2.0)
 *
 * Mixed:
 *
 *     [1.0, 2.0)
 *
 * The endpoints are syntactic values.
 *
 * Range satisfiability is semantic analysis.
 */

capabilityVersionRange
    : LEFT_BRACKET
      capabilityVersion
      COMMA
      capabilityVersion
      RIGHT_BRACKET

    | LEFT_PAREN
      capabilityVersion
      COMMA
      capabilityVersion
      RIGHT_PAREN

    | LEFT_BRACKET
      capabilityVersion
      COMMA
      capabilityVersion
      RIGHT_PAREN

    | LEFT_PAREN
      capabilityVersion
      COMMA
      capabilityVersion
      RIGHT_BRACKET
    ;


/* ============================================================================
 * VERSION SET
 * ============================================================================
 *
 * A set permits multiple constraints.
 *
 * Examples:
 *
 *     version >= 1.0 and < 3.0
 *
 *     version >= 1.0 and < 3.0 or == 5.0
 *
 * The parser captures structure.
 *
 * Semantic precedence and satisfiability belong to semantic analysis.
 */

capabilityVersionSet
    : capabilityVersionDisjunction
    ;


capabilityVersionDisjunction
    : capabilityVersionConjunction
      (OR capabilityVersionConjunction)*
    ;


capabilityVersionConjunction
    : capabilityVersionPrimary
      (AND capabilityVersionPrimary)*
    ;


capabilityVersionPrimary
    : capabilityVersionConstraint
    | capabilityVersionRange
    | LEFT_PAREN capabilityVersionDisjunction RIGHT_PAREN
    ;


/* ============================================================================
 * VERSION REFERENCE
 * ============================================================================
 *
 * Allows a version requirement to refer to a named version contract.
 *
 * Example:
 *
 *     version standard::stable
 *
 * The referenced object is resolved semantically.
 */

capabilityVersionReference
    : qualifiedName
    ;


/* ============================================================================
 * CAPABILITY VERSION
 * ============================================================================
 *
 * The version grammar is intentionally unbounded in semantic scale.
 *
 * Numeric components are represented by the canonical INTEGER token.
 *
 * Supported structural forms:
 *
 *     1
 *     1.2
 *     1.2.3
 *     1.2.3-alpha
 *     1.2.3+build
 *     1.2.3-alpha+build
 *
 * The exact semantic-versioning policy belongs to the versioning subsystem.
 */

capabilityVersion
    : versionCore versionSuffix?
    ;


versionCore
    : INTEGER
    | INTEGER DOT INTEGER
    | INTEGER DOT INTEGER DOT INTEGER
    ;


versionSuffix
    : versionPreRelease versionBuildMetadata?
    | versionBuildMetadata
    ;


versionPreRelease
    : MINUS versionIdentifierList
    ;


versionBuildMetadata
    : PLUS versionIdentifierList
    ;


versionIdentifierList
    : versionIdentifier
      (DOT versionIdentifier)*
    ;


versionIdentifier
    : IDENTIFIER
    | INTEGER
    ;


/* ============================================================================
 * CAPABILITY ATTRIBUTES
 * ============================================================================
 *
 * Attribute syntax is delegated to the canonical attribute grammar.
 *
 * The rule is an integration boundary rather than a second attribute system.
 */

capabilityAttributeList
    : attributes
    ;


/* ============================================================================
 * CAPABILITY REFERENCE WITH ALIAS
 * ============================================================================
 *
 * Alias semantics belong to the consuming declaration.
 *
 * This rule only represents:
 *
 *     capability.name as local_name
 */

capabilityAlias
    : capabilityReference AS identifier
    ;


/* ============================================================================
 * CAPABILITY ALIAS LIST
 * ============================================================================
 */

capabilityAliasList
    : capabilityAlias
      (COMMA capabilityAlias)*
    ;


/* ============================================================================
 * CAPABILITY EXPRESSION
 * ============================================================================
 *
 * This is the generic syntactic expression used where a source construct
 * accepts capability predicates.
 *
 * It deliberately does not evaluate them.
 *
 * Examples:
 *
 *     quantum
 *     quantum::dynamic_control
 *     quantum::dynamic_control and classical
 *     quantum or simulator
 *     not legacy::feature
 */

capabilityExpression
    : capabilityOrExpression
    ;


capabilityOrExpression
    : capabilityAndExpression
      (OR capabilityAndExpression)*
    ;


capabilityAndExpression
    : capabilityNotExpression
      (AND capabilityNotExpression)*
    ;


capabilityNotExpression
    : NOT capabilityNotExpression
    | capabilityPrimaryExpression
    ;


capabilityPrimaryExpression
    : capabilityReference
    | LEFT_PAREN capabilityExpression RIGHT_PAREN
    ;


/* ============================================================================
 * CAPABILITY EXPRESSION LIST
 * ============================================================================
 */

capabilityExpressionList
    : capabilityExpression
      (COMMA capabilityExpression)*
    ;


/* ============================================================================
 * CAPABILITY REQUIREMENT REFERENCE
 * ============================================================================
 *
 * This rule intentionally provides syntax that requirements.g4 can consume.
 *
 * It does not turn a capability into a requirement by itself.
 */

capabilityRequirementReference
    : capabilityReference
    ;


/* ============================================================================
 * CAPABILITY PROVISION REFERENCE
 * ============================================================================
 *
 * A provider declaration may reuse the same capability identity.
 */

capabilityProvisionReference
    : capabilityReference
    ;


/* ============================================================================
 * CAPABILITY PREFERENCE REFERENCE
 * ============================================================================
 *
 * A preference may refer to a capability without making it mandatory.
 */

capabilityPreferenceReference
    : capabilityReference
    ;


/* ============================================================================
 * CAPABILITY CONSTRAINT REFERENCE
 * ============================================================================
 *
 * Constraints may mention capabilities, but constraint semantics remain
 * outside this file.
 */

capabilityConstraintReference
    : capabilityReference
    ;


/* ============================================================================
 * CAPABILITY EFFECT REFERENCE
 * ============================================================================
 *
 * Effects may identify capabilities associated with an operation.
 *
 * This is only syntax.
 */

capabilityEffectReference
    : capabilityReference
    ;


/* ============================================================================
 * CAPABILITY TARGET REFERENCE
 * ============================================================================
 *
 * A target may expose capability names.
 *
 * This rule does NOT bind a capability to a particular physical target.
 */

capabilityTargetReference
    : capabilityReference
    ;


/* ============================================================================
 * CAPABILITY FEATURE REFERENCE
 * ============================================================================
 *
 * Generic extension point for future domains.
 */

capabilityFeatureReference
    : capabilityReference
    ;


/* ============================================================================
 * CAPABILITY LIST WRAPPERS
 * ============================================================================
 *
 * These wrappers exist so higher-level grammars do not reproduce comma-list
 * syntax and accidentally create incompatible capability representations.
 */

capabilityRequirementList
    : capabilityRequirementReference
      (COMMA capabilityRequirementReference)*
    ;


capabilityProvisionList
    : capabilityProvisionReference
      (COMMA capabilityProvisionReference)*
    ;


capabilityPreferenceList
    : capabilityPreferenceReference
      (COMMA capabilityPreferenceReference)*
    ;


capabilityConstraintList
    : capabilityConstraintReference
      (COMMA capabilityConstraintReference)*
    ;


capabilityEffectList
    : capabilityEffectReference
      (COMMA capabilityEffectReference)*
    ;


capabilityTargetList
    : capabilityTargetReference
      (COMMA capabilityTargetReference)*
    ;


/* ============================================================================
 * CAPABILITY DECLARATION BODY
 * ============================================================================
 *
 * This optional body provides an extensibility boundary without requiring
 * capability syntax to know every future property.
 *
 * Example:
 *
 *     capability quantum::dynamic_control {
 *         ...
 *     }
 *
 * The contents are deliberately delegated to capability properties.
 *
 * IMPORTANT:
 *
 * The language specification must define the body property grammar before
 * enabling this production in the canonical root grammar. It is therefore
 * intentionally NOT included in capabilityDeclaration above.
 *
 * This rule is reserved as an integration extension point.
 */


/* ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * CORE:
 *
 *     names.g4
 *         |
 *         +--> identifier
 *         +--> qualifiedName
 *
 *     attributes.g4
 *         |
 *         +--> attributes
 *
 *     versioning.g4
 *         |
 *         +--> shared version policy
 *
 * CAPABILITIES:
 *
 *     capabilities.g4
 *         |
 *         +--> capabilityDeclaration
 *         +--> capabilityReference
 *         +--> capabilityExpression
 *
 * REQUIREMENTS:
 *
 *     requirements.g4
 *         |
 *         +--> consumes capabilityReference
 *
 * CONSTRAINTS:
 *
 *     constraints.g4
 *         |
 *         +--> may consume capabilityReference
 *
 * PREFERENCES:
 *
 *     preferences / hints
 *         |
 *         +--> may consume capabilityReference
 *
 * EFFECTS:
 *
 *     effects.g4
 *         |
 *         +--> capabilityEffectReference
 *
 * HARDWARE:
 *
 *     hardware/capabilities.g4
 *         |
 *         +--> may wrap capabilityReference
 *
 * QUANTUM:
 *
 *     quantum/quantum-capabilities.g4
 *         |
 *         +--> may wrap capabilityReference
 *
 * DISTRIBUTED:
 *
 *     distributed/*
 *         |
 *         +--> may consume capabilityReference
 *
 * AI:
 *
 *     ai/*
 *         |
 *         +--> may consume capabilityReference
 *
 * HDL:
 *
 *     hdl/*
 *         |
 *         +--> may consume capabilityReference
 *
 * ============================================================================
 * NON-DEPENDENCIES
 * ============================================================================
 *
 * This file MUST NOT depend on:
 *
 *     src/quantum/ir
 *     src/quantum/qec
 *     src/quantum/zqn
 *     src/quantum/scheduling
 *     src/quantum/routing
 *     src/quantum/optimization
 *     src/quantum/hardware
 *     runtime
 *     calibration
 *     backend SDKs
 *     vendor APIs
 *
 * Those systems consume semantic capability information after parsing.
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * The parser must lower capability syntax into the existing native AST
 * capability representation rather than creating a second capability model.
 *
 * The repository already contains:
 *
 *     src/frontend/ast/node/capabilities/capability.rs
 *
 * which represents a source-level capability identity and version
 * requirement. That AST explicitly keeps capability meaning separate from
 * hardware/backend realization.
 *
 * The AST should therefore receive:
 *
 *     capability namespace/name
 *     capability version constraint
 *     annotations
 *     source span
 *
 * without receiving:
 *
 *     physical resource
 *     device ID
 *     backend ID
 *     topology
 *     calibration
 *     runtime token
 *
 * ============================================================================
 * QUANTUM IR INTEGRATION
 * ============================================================================
 *
 * This file MUST NOT import or reference `quantum::ir` directly.
 *
 * Correct pipeline:
 *
 *     capability syntax
 *          |
 *          v
 *     native AST Capability
 *          |
 *          v
 *     semantic capability resolution
 *          |
 *          v
 *     semantic capability representation
 *          |
 *          v
 *     quantum::ir where applicable
 *
 * This prevents the grammar from becoming a second quantum IR.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Capability syntax describes ability.
 *
 * Resource syntax describes availability/consumption.
 *
 * Example conceptual separation:
 *
 *     requires quantum::dynamic_control
 *
 * versus:
 *
 *     resource quantum_execution
 *
 * The first is a capability requirement.
 *
 * The second concerns resource semantics.
 *
 * Neither one should be silently transformed into the other.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware capability information is discovered by hardware/runtime layers.
 *
 * The grammar only represents portable source-level capability contracts.
 *
 * Therefore:
 *
 *     capability quantum::dynamic_control
 *
 * MUST NOT encode:
 *
 *     device = qpu42
 *     qubits = 127
 *     topology = ...
 *     calibration = ...
 *
 * Such information belongs to target/resource/capability resolution.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime capability tokens and authorization credentials are NOT source
 * capability identities.
 *
 * A runtime MAY resolve:
 *
 *     zamani::quantum::dynamic_control
 *
 * into a runtime capability token.
 *
 * That conversion is outside the grammar.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Capability names are untrusted source input.
 *
 * Parsing MUST:
 *
 *     - perform no filesystem access;
 *     - perform no network access;
 *     - execute no capability;
 *     - load no backend;
 *     - contact no provider;
 *     - resolve no credentials;
 *     - authorize no operation.
 *
 * Capability names are data.
 *
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * Syntax errors MUST be reported by the parser.
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 * SYNTAX ERROR:
 *
 *     capability foo::;
 *
 * SEMANTIC ERROR:
 *
 *     capability foo::unknown_feature;
 *
 * CAPABILITY AVAILABILITY ERROR:
 *
 *     requires foo::feature
 *
 * when the selected execution environment does not provide it.
 *
 * These error classes MUST remain distinguishable.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Capability identity syntax is intentionally stable:
 *
 *     qualifiedName
 *
 * Capability version requirements are independently extensible.
 *
 * A future capability MUST NOT require changing this file merely because it
 * belongs to a new computational domain.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains no finite limits on:
 *
 *     capability declarations
 *     capability references
 *     qualified-name depth
 *     capability-list length
 *     version identifier count
 *     source program size
 *
 * All repeated structures are represented using unbounded grammar repetition.
 *
 * Practical parser/compiler limits remain implementation/resource concerns.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     capability quantum::dynamic_control;
 *     capability quantum::dynamic_control version 1;
 *     capability quantum::dynamic_control version 1.2;
 *     capability quantum::dynamic_control version 1.2.3;
 *     capability quantum::dynamic_control version >= 1.2;
 *     capability quantum::dynamic_control version <= 2.0;
 *     capability quantum::dynamic_control version [1.0, 2.0];
 *     capability future::domain::feature;
 *
 * Negative:
 *
 *     capability;
 *     capability foo::;
 *     capability ::foo;
 *     capability foo version;
 *     capability foo version 1..;
 *     capability foo version [2.0, 1.0];
 *
 * Boundary:
 *
 *     one capability
 *     many capabilities
 *     deeply qualified capability names
 *     large version components
 *     long version metadata
 *
 * Cross-domain:
 *
 *     classical capability
 *     quantum capability
 *     HDL capability
 *     hardware capability
 *     distributed capability
 *     AI capability
 *     networking capability
 *     security capability
 *     hybrid capability
 *
 * POCO-REAF:
 *
 *     capability names remain unchanged when target resources change.
 *
 * Determinism:
 *
 *     identical token streams produce identical parse trees.
 *
 * Round trip:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> printer
 *       -> parser
 *
 * preserves capability identity and version structure.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when:
 *
 * [ ] canonical lexer exposes the required capability declaration token;
 * [ ] Names-qualifiedName is the sole name syntax authority;
 * [ ] attribute syntax comes from the canonical attribute grammar;
 * [ ] version semantics remain outside this grammar;
 * [ ] capability AST lowering targets the existing native Capability model;
 * [ ] no second capability IR exists;
 * [ ] no hardware capability is hard-coded;
 * [ ] no quantum resource count is hard-coded;
 * [ ] no backend is selected by this grammar;
 * [ ] no physical resource is selected by this grammar;
 * [ ] no runtime capability token is represented here;
 * [ ] requirements.g4 consumes capabilityReference;
 * [ ] constraints.g4 can consume capabilityReference;
 * [ ] effects.g4 can consume capabilityReference;
 * [ ] resource semantics remain separate;
 * [ ] target semantics remain separate;
 * [ ] parser tests pass;
 * [ ] negative tests pass;
 * [ ] boundary tests pass;
 * [ ] cross-domain tests pass;
 * [ ] deterministic parsing tests pass;
 * [ ] round-trip tests pass;
 * [ ] generated ANTLR parser compiles;
 * [ ] Rust 1.97 / 1.97.1 compilation succeeds;
 * [ ] `unsafe` is not required anywhere in the integration;
 * [ ] hard-coding audit passes.
 *
 * ============================================================================
 */