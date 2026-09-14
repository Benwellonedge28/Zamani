/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/security/security-constraints.g4
 *
 * Purpose:
 *     Security-specific constraint syntax.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe code.
 *     - No filesystem access.
 *     - No network access.
 *     - No runtime calls.
 *     - No hardware discovery.
 *     - No policy evaluation.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical Zamani lexer
 *          |
 *          v
 *     parser composition
 *          |
 *          +--> Core constraints
 *          |
 *          +--> THIS FILE
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic security analysis
 *          |
 *          +--> identity analysis
 *          +--> permission analysis
 *          +--> capability analysis
 *          +--> trust analysis
 *          +--> privacy analysis
 *          +--> cryptographic analysis
 *          +--> resource analysis
 *          +--> target analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed representation
 *          |
 *          v
 *     optimization / scheduling / routing / resilience
 *          |
 *          v
 *     target lowering
 *          |
 *          v
 *     runtime / deployment
 *
 * ============================================================================
 * CORE ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file specializes the GENERIC constraint model for security semantics.
 *
 * It MUST NOT create a second generic constraint language.
 *
 * Generic constraint semantics remain owned by:
 *
 *     grammar/core/constraints.g4
 *
 * Security constraints may refer to:
 *
 *     identity
 *     principal
 *     permission
 *     capability
 *     trust
 *     privacy
 *     cryptographic intent
 *     authentication intent
 *     authorization intent
 *     audit requirements
 *     secure execution
 *     isolation
 *     integrity
 *     confidentiality
 *     availability
 *     provenance
 *
 * using open-ended qualified names and canonical expressions.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - security-specific constraint declarations;
 *     - security constraint clauses;
 *     - security constraint expressions;
 *     - security constraint targets;
 *     - security-specific constraint modifiers;
 *     - security constraint composition;
 *     - security constraint references;
 *     - security constraint assertions;
 *     - security constraint metadata;
 *     - security-specific constraint syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - generic constraints;
 *     - identifiers;
 *     - qualified names;
 *     - lexical tokens;
 *     - general expressions;
 *     - types;
 *     - requirements;
 *     - capabilities;
 *     - identities;
 *     - principals;
 *     - permissions;
 *     - authentication implementation;
 *     - authorization enforcement;
 *     - trust evaluation;
 *     - cryptographic implementation;
 *     - key generation;
 *     - key storage;
 *     - privacy implementation;
 *     - audit storage;
 *     - resource discovery;
 *     - hardware discovery;
 *     - device selection;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - resilience;
 *     - runtime execution;
 *     - deployment.
 *
 * ============================================================================
 * SECURITY CONSTRAINT PRINCIPLE
 * ============================================================================
 *
 * A security constraint states a condition that a valid realization MUST
 * satisfy.
 *
 * It is not itself:
 *
 *     - a permission grant;
 *     - a denial;
 *     - an authentication operation;
 *     - an encryption operation;
 *     - a trust decision;
 *     - a runtime enforcement mechanism;
 *     - a hardware selection;
 *     - a resource allocation.
 *
 * For example:
 *
 *     security constraint secure_execution == true;
 *
 * means that a valid realization must satisfy the semantic condition.
 *
 * It does NOT mean:
 *
 *     use enclave X
 *     use CPU Y
 *     use provider Z
 *     use device N
 *
 * ============================================================================
 * UNIVERSAL COMPUTING / POCO-REAF
 * ============================================================================
 *
 * Security constraints must remain valid across:
 *
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     quantum processors
 *     quantum simulators
 *     accelerators
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational substrates
 *
 * No constraint in this grammar may require a particular physical machine
 * unless that physical identity is explicitly part of the source program's
 * intended semantics and is represented by an appropriate downstream target
 * or deployment mechanism.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no finite language-level limit on:
 *
 *     - number of constraints;
 *     - number of security domains;
 *     - number of principals;
 *     - number of permissions;
 *     - number of capabilities;
 *     - number of trust relationships;
 *     - number of policies;
 *     - number of security properties;
 *     - number of expressions;
 *     - number of nested groups;
 *     - qualified-name depth;
 *     - program size;
 *     - machine size;
 *     - node count;
 *     - processor count;
 *     - accelerator count;
 *     - qubit count;
 *     - memory capacity;
 *     - storage capacity.
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * Practical limits belong to:
 *
 *     parser implementation
 *     available memory
 *     compiler policy
 *     operating system
 *     runtime environment
 *     deployment environment
 *
 * and MUST NOT become grammar-level constants.
 *
 * ============================================================================
 * OPEN-WORLD SECURITY
 * ============================================================================
 *
 * Security properties are open-ended.
 *
 * This grammar MUST NOT enumerate a permanent closed list of:
 *
 *     algorithms
 *     providers
 *     vendors
 *     enclaves
 *     identity systems
 *     trust systems
 *     regulatory systems
 *     hardware security modules
 *     processors
 *     devices
 *
 * Examples such as:
 *
 *     future::security::property
 *     organization::security::requirement
 *     domain::confidentiality
 *
 * must remain syntactically representable without grammar modification.
 *
 * ============================================================================
 * DOMAIN OWNERSHIP
 * ============================================================================
 *
 * Identity:
 *
 *     grammar/security/identities.g4
 *
 * Permissions:
 *
 *     grammar/security/permissions.g4
 *
 * Capabilities:
 *
 *     grammar/security/capabilities.g4
 *
 * Cryptography:
 *
 *     grammar/security/cryptography.g4
 *
 * Privacy:
 *
 *     grammar/security/privacy.g4
 *
 * Trust:
 *
 *     grammar/security/trust.g4
 *
 * Generic constraints:
 *
 *     grammar/core/constraints.g4
 *
 * Generic requirements:
 *
 *     grammar/core/requirements.g4
 *
 * This file references those concepts semantically but does not redefine
 * their ownership.
 *
 * ============================================================================
 * AUTHORIZATION BOUNDARY
 * ============================================================================
 *
 * Security constraints MUST NOT replace permissions.
 *
 * A constraint such as:
 *
 *     security::authorization_required == true
 *
 * expresses a required condition.
 *
 * It does not grant authorization.
 *
 * Permission semantics remain owned by:
 *
 *     grammar/security/permissions.g4
 *
 * ============================================================================
 * TRUST BOUNDARY
 * ============================================================================
 *
 * Trust relationships remain owned by:
 *
 *     grammar/security/trust.g4
 *
 * A security constraint may reference trust properties:
 *
 *     trust::execution_environment == trusted
 *
 * but this grammar does not determine whether the environment is trusted.
 *
 * ============================================================================
 * CRYPTOGRAPHIC BOUNDARY
 * ============================================================================
 *
 * Cryptographic mechanisms remain owned by:
 *
 *     grammar/security/cryptography.g4
 *
 * This grammar may constrain a cryptographic property:
 *
 *     cryptography::confidentiality == required
 *
 * but MUST NOT define:
 *
 *     key material
 *     private keys
 *     passwords
 *     secrets
 *     certificates
 *     cryptographic implementation
 *     algorithm internals
 *
 * ============================================================================
 * PRIVACY BOUNDARY
 * ============================================================================
 *
 * Privacy semantics remain owned by:
 *
 *     grammar/security/privacy.g4
 *
 * Security constraints may refer to privacy properties but must not redefine:
 *
 *     privacy policy
 *     consent
 *     retention
 *     minimization
 *     jurisdiction
 *     disclosure policy
 *
 * ============================================================================
 * IDENTITY BOUNDARY
 * ============================================================================
 *
 * Identity declarations remain owned by:
 *
 *     grammar/security/identities.g4
 *
 * This grammar only references identity concepts by canonical names.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Security constraints can apply to quantum computation.
 *
 * Examples of semantically valid references include:
 *
 *     quantum::execution_integrity
 *     quantum::confidential_execution
 *     quantum::measurement_provenance
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum topology
 *     calibration
 *     QEC codes
 *     ZQN fault models
 *     backend selection
 *
 * If a security constraint affects quantum compilation, semantic analysis may
 * propagate that information toward quantum::ir.
 *
 * The grammar itself MUST NOT import or construct quantum::ir.
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * Security constraints may describe abstract hardware/security properties:
 *
 *     hardware::secure_execution == true
 *     hardware::memory_isolation == required
 *
 * They MUST NOT encode:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     fpga0
 *     device addresses
 *     fixed topology
 *     fixed machine identifiers
 *     fixed memory capacity
 *
 * Hardware realization belongs downstream.
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * A security constraint may refer to resource properties:
 *
 *     resource::protected_memory >= required
 *
 * but this file does not define:
 *
 *     resource allocation;
 *     capacity discovery;
 *     scheduling;
 *     placement;
 *     memory management.
 *
 * Those belong to the resource and execution subsystems.
 *
 * ============================================================================
 * CORE CONSTRAINT INTEGRATION
 * ============================================================================
 *
 * Generic constraint semantics are owned by:
 *
 *     grammar/core/constraints.g4
 *
 * This file therefore uses the canonical generic constraint expression where
 * possible rather than creating an incompatible expression hierarchy.
 *
 * Conceptual dependency:
 *
 *     Core constraints
 *          |
 *          v
 *     security constraint target
 *          |
 *          v
 *     security constraint expression
 *          |
 *          v
 *     semantic security constraint
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar has:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no runtime state;
 *     - no filesystem access;
 *     - no network access;
 *     - no randomness;
 *     - no hardware discovery.
 *
 * Parsing is therefore determined solely by the supplied token stream.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The frontend AST must preserve:
 *
 *     - source order;
 *     - target structure;
 *     - constraint expression structure;
 *     - operator kind;
 *     - grouping;
 *     - negation;
 *     - qualified-name structure;
 *     - literal values;
 *     - source spans;
 *     - attached metadata.
 *
 * Semantic normalization belongs downstream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Conceptual AST nodes:
 *
 *     SecurityConstraintDeclaration
 *     SecurityConstraintTarget
 *     SecurityConstraintExpression
 *     SecurityConstraintReference
 *     SecurityConstraintMetadata
 *
 * SecurityConstraintExpression may represent:
 *
 *     Predicate
 *     Comparison
 *     Equality
 *     Inequality
 *     Negation
 *     Conjunction
 *     Disjunction
 *     Group
 *     Reference
 *
 * Exact Rust AST structures belong to the frontend AST subsystem.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether referenced names resolve;
 *     - whether security properties are known;
 *     - whether values have compatible types;
 *     - whether a constraint is meaningful;
 *     - whether multiple constraints conflict;
 *     - whether the constraint is satisfiable;
 *     - whether the result is known or unknown;
 *     - whether the constraint applies to a selected semantic object;
 *     - whether the constraint remains valid for a target.
 *
 * The grammar does none of these operations.
 *
 * ============================================================================
 * COMPILATION CONTRACT
 * ============================================================================
 *
 * Compilation may consume security constraints to:
 *
 *     - reject invalid realizations;
 *     - preserve mandatory security conditions;
 *     - select compatible implementation strategies;
 *     - participate in target capability negotiation;
 *     - propagate security metadata;
 *     - preserve constraints through lowering.
 *
 * Compilation MUST NOT reinterpret a constraint as an instruction to select
 * an arbitrary physical device.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime may evaluate security constraints against actual execution state.
 *
 * Runtime evaluation is distinct from parsing.
 *
 * Example:
 *
 *     source
 *       |
 *       v
 *     security constraint AST
 *       |
 *       v
 *     semantic security constraint
 *       |
 *       v
 *     compiled security metadata
 *       |
 *       v
 *     runtime environment
 *       |
 *       v
 *     constraint evaluation
 *
 * Runtime failure is therefore not a parser failure.
 *
 * ============================================================================
 * RESILIENCE CONTRACT
 * ============================================================================
 *
 * Security constraints may be consumed by resilience analysis when recovery
 * decisions must preserve security guarantees.
 *
 * This file MUST NOT implement:
 *
 *     retry
 *     rollback
 *     recovery
 *     backend switching
 *     quarantine
 *     rerouting
 *     recompilation
 *
 * Resilience remains downstream.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes the canonical Zamani lexer vocabulary.
 *
 * The canonical lexer owns:
 *
 *     keyword spelling
 *     punctuation
 *     operators
 *     identifiers
 *     literals
 *
 * This grammar MUST NOT define lexer tokens.
 *
 * ============================================================================
 * TOKEN COMPATIBILITY
 * ============================================================================
 *
 * The repository's current lexical foundation uses token names such as:
 *
 *     K_WHEN
 *     K_WITH
 *     K_FROM
 *     K_TO
 *     K_IF
 *     K_AND
 *     K_OR
 *     K_NOT
 *     K_TRUST
 *
 * and operator tokens such as:
 *
 *     EQUAL_EQUAL
 *     NOT_EQUAL
 *     LESS
 *     LESS_EQUAL
 *     GREATER
 *     GREATER_EQUAL
 *
 * Token names MUST be synchronized with the actual canonical lexer before
 * parser generation.
 *
 * This file deliberately does NOT invent speculative tokens such as:
 *
 *     SECURITY_CONSTRAINT
 *     SECURE
 *     REQUIRE
 *     CONSTRAIN
 *     SECURITY_PROPERTY
 *
 * unless those are explicitly introduced into the canonical lexer as part of
 * a separately versioned language change.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * Public integration points:
 *
 *     securityConstraintDeclaration
 *     securityConstraintClause
 *     securityConstraintExpression
 *     securityConstraintReference
 *     securityConstraintTarget
 *     securityConstraintList
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PARSER
 * ============================================================================
 */

parser grammar SecurityConstraints;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions;


/*
 * ============================================================================
 * 1. STANDALONE ENTRY POINT
 * ============================================================================
 *
 * Used by grammar tests and grammar tooling.
 *
 * The complete Zamani parser may consume securityConstraintDeclaration
 * directly instead of using this EOF-consuming rule.
 */

securityConstraintsFile
    : securityConstraintDeclaration*
      EOF
    ;


/*
 * ============================================================================
 * 2. SECURITY CONSTRAINT DECLARATION
 * ============================================================================
 *
 * Canonical semantic shape:
 *
 *     security constraint <target> <expression>;
 *
 * The concrete security vocabulary should be introduced by the canonical
 * lexer only when the language specification has standardized it.
 *
 * To remain compatible with the present lexical foundation, the declaration
 * uses an identifier-based security namespace rather than inventing new
 * lexer keywords.
 *
 * Example conceptual forms:
 *
 *     security::constraint::confidentiality == required;
 *
 *     security::constraint::integrity == required;
 *
 *     security::constraint::trusted_execution == true;
 *
 * The exact semantic interpretation belongs downstream.
 */

securityConstraintDeclaration
    : attributes*
      visibility?
      securityConstraintHead
      securityConstraintBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 3. SECURITY CONSTRAINT HEAD
 * ============================================================================
 *
 * A qualified name is used as the open-world security constraint identity.
 *
 * This avoids hard-coding a finite list of security domains.
 */

securityConstraintHead
    : securityConstraintReference
      securityConstraintTarget?
      securityConstraintExpression?
    ;


/*
 * ============================================================================
 * 4. SECURITY CONSTRAINT TARGET
 * ============================================================================
 *
 * A constraint may optionally identify the semantic subject to which it
 * applies.
 *
 * The subject is a source-level reference, not a physical device.
 *
 * Examples:
 *
 *     security::confidentiality for data::record;
 *     security::integrity for quantum::program;
 *     security::isolation for hardware::module;
 *
 * The target's actual semantic type is resolved downstream.
 */

securityConstraintTarget
    : K_FOR
      securityConstraintReference
    ;


/*
 * ============================================================================
 * 5. SECURITY CONSTRAINT BODY
 * ============================================================================
 */

securityConstraintBody
    : LBRACE
      securityConstraintMember*
      RBRACE
    ;

securityConstraintMember
    : securityConstraintClause
    | securityConstraintProperty
    | securityConstraintMetadata
    ;


/*
 * ============================================================================
 * 6. SECURITY CONSTRAINT CLAUSE
 * ============================================================================
 *
 * A clause represents a condition that must hold.
 */

securityConstraintClause
    : securityConstraintReference
      securityConstraintPredicate?
      securityConstraintExpression?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. SECURITY CONSTRAINT PREDICATES
 * ============================================================================
 *
 * These predicates are intentionally structural.
 *
 * They do not perform evaluation.
 */

securityConstraintPredicate
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


/*
 * ============================================================================
 * 8. SECURITY CONSTRAINT EXPRESSIONS
 * ============================================================================
 *
 * General expressions remain owned by grammar/expressions.
 *
 * Security constraints therefore do not create a second expression language.
 */

securityConstraintExpression
    : securityConstraintNot
    | securityConstraintLogical
    | securityConstraintComparison
    | securityConstraintPrimary
    ;


securityConstraintNot
    : K_NOT
      securityConstraintExpression
    ;


securityConstraintLogical
    : securityConstraintExpression
      securityConstraintLogicalOperator
      securityConstraintExpression
    ;


securityConstraintLogicalOperator
    : K_AND
    | K_OR
    ;


securityConstraintComparison
    : securityConstraintPrimary
      securityConstraintPredicate
      securityConstraintPrimary
    ;


securityConstraintPrimary
    : securityConstraintReference
    | literal
    | LPAREN
      securityConstraintExpression
      RPAREN
    | expression
    ;


/*
 * ============================================================================
 * 9. REFERENCES
 * ============================================================================
 *
 * References remain open-world.
 *
 * Examples:
 *
 *     identity::subject
 *     principal::operator
 *     permission::execute
 *     capability::secure_execution
 *     trust::execution_environment
 *     privacy::confidential
 *     cryptography::protected_data
 *     quantum::execution_integrity
 *     hardware::isolation
 *     future::security::property
 *
 * No enumeration is performed here.
 */

securityConstraintReference
    : qualifiedName
    ;


securityConstraintReferenceList
    : securityConstraintReference
      (COMMA securityConstraintReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * 10. PROPERTY
 * ============================================================================
 */

securityConstraintProperty
    : identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. METADATA
 * ============================================================================
 *
 * Metadata remains syntactic.
 *
 * Provenance, classification, source location, version information and
 * annotations are interpreted by the frontend/semantic layers.
 */

securityConstraintMetadata
    : identifier
      COLON
      expression
      SEMICOLON
    ;