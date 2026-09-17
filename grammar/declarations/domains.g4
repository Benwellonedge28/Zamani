/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/domains.g4
 *
 * Grammar:
 *     ZamaniDomains
 *
 * Role:
 *     AUTHORITATIVE SOURCE-LEVEL DOMAIN DECLARATION GRAMMAR
 *
 * Status:
 *     Production-target modular parser grammar
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     No embedded Rust.
 *     No semantic predicates.
 *     No actions.
 *     No filesystem access.
 *     No network access.
 *     No runtime access.
 *     No hardware discovery.
 *     No target selection.
 *     No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the source syntax of a Zamani `domain` declaration.
 *
 * A domain is a source-level namespace/semantic boundary used to group
 * computational intent belonging to a coherent computational domain.
 *
 * Examples of domains include:
 *
 *     classical
 *     quantum
 *     hybrid
 *     hdl
 *     hardware
 *     distributed
 *     ai
 *     data
 *     networking
 *     security
 *     scientific
 *     embedded
 *     accelerator
 *     future computational domains
 *
 * These are NOT separate languages.
 *
 * A domain declaration therefore does not introduce:
 *
 *     - a second lexer;
 *     - a second parser;
 *     - a second AST;
 *     - a second type system;
 *     - a second semantic model;
 *     - a second quantum IR;
 *     - a hardware backend;
 *     - a runtime;
 *     - a resource allocator.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     domainDeclaration
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     name/type/effect/resource/capability analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *     +----+----------+----------+----------+
 *     |               |          |          |
 *     v               v          v          v
 * classical       quantum      HDL      future domains
 * semantics       semantics   semantics
 *                    |
 *                    v
 *                quantum::ir
 *                    |
 *                    v
 *          optimization / lowering
 *                    |
 *          routing / scheduling
 *                    |
 *          resilience / QEC / ZQN
 *                    |
 *                   HAL
 *                    |
 *             target realization
 *
 * IMPORTANT:
 *
 *     quantum::ir
 *
 * remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NOT construct or define a quantum IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - domainDeclaration
 *     - domain identity syntax
 *     - domain attributes
 *     - domain visibility/modifier composition
 *     - domain generic parameters
 *     - domain inheritance syntax
 *     - domain version syntax
 *     - domain capability declarations
 *     - domain resource declarations
 *     - domain type-alias members
 *     - domain requirement clauses
 *     - domain provision clauses
 *     - domain usage clauses
 *     - domain constraint clauses
 *     - domain preference clauses
 *     - domain hint clauses
 *     - domain properties
 *     - nested domain declarations
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers
 *     - qualified names
 *     - attributes
 *     - visibility
 *     - modifiers
 *     - expressions
 *     - type expressions
 *     - capability identity
 *     - capability version semantics
 *     - resource identity
 *     - resource feasibility
 *     - functions
 *     - classes
 *     - structs
 *     - traits
 *     - interfaces
 *     - modules
 *     - quantum operations
 *     - HDL operations
 *     - hardware realization
 *     - routing
 *     - scheduling
 *     - optimization
 *     - calibration
 *     - QEC
 *     - ZQN
 *     - HAL
 *     - runtime execution
 *
 * Those remain owned by their existing grammar/semantic components.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Domain declarations describe PORTABLE COMPUTATIONAL INTENT.
 *
 * They MUST NOT encode universal implementation limits.
 *
 * This grammar contains NO:
 *
 *     MAX_DOMAINS
 *     MAX_DOMAIN_DEPTH
 *     MAX_DOMAIN_MEMBERS
 *     MAX_CAPABILITIES
 *     MAX_RESOURCES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_NETWORK_LINKS
 *     MAX_ACCELERATORS
 *
 * Domain nesting is therefore represented recursively and has no
 * language-level cardinality limit.
 *
 * Practical implementation limits remain compiler/runtime policy and MUST NOT
 * become grammar restrictions.
 *
 * ============================================================================
 * OPEN-WORLD DOMAIN MODEL
 * ============================================================================
 *
 * The grammar intentionally does NOT enumerate:
 *
 *     classical
 *     quantum
 *     hdl
 *     ai
 *     gpu
 *     fpga
 *     qpu
 *     neuromorphic
 *     optical
 *     biological
 *     nano
 *     future
 *
 * as a closed domain enumeration.
 *
 * Domain identity is a qualified source name.
 *
 * Therefore new computational paradigms can be introduced without modifying
 * this grammar.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Canonical shared parser components:
 *
 *     Attributes
 *     Capabilities
 *     Names
 *     Types
 *     Expressions
 *     Visibility
 *     Modifiers
 *
 * Existing declaration/resource components:
 *
 *     Resources
 *     ZamaniDeclarationAliases
 *
 * The canonical lexer vocabulary remains the repository's single lexer.
 *
 * ============================================================================
 */

parser grammar ZamaniDomains;

options {
    tokenVocab = ZamaniLexer;
}

import
    Attributes,
    Capabilities,
    Expressions,
    Names,
    Types,
    Visibility,
    Modifiers,
    Resources,
    ZamaniDeclarationAliases
;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the rule imported by the canonical declaration dispatcher.
 */

domainDeclaration
    : attribute*
      visibilityModifier?
      modifierList?
      K_DOMAIN
      qualifiedName
      domainGenericParameters?
      domainExtendsClause?
      domainVersionClause?
      domainBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. GENERIC PARAMETERS
 * ============================================================================
 *
 * Domain parameters remain source-level parameters.
 *
 * They may represent:
 *
 *     types
 *     values
 *     shapes
 *     resource abstractions
 *     capability abstractions
 *
 * Semantic interpretation belongs downstream.
 *
 * There is deliberately no fixed parameter count.
 */

domainGenericParameters
    : LESS_THAN
      domainGenericParameter
      (COMMA domainGenericParameter)*
      COMMA?
      GREATER_THAN
    ;

domainGenericParameter
    : identifier
      domainGenericParameterType?
      domainGenericParameterDefault?
    ;

domainGenericParameterType
    : COLON
      typeExpression
    ;

domainGenericParameterDefault
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 3. DOMAIN INHERITANCE
 * ============================================================================
 *
 * A domain may refine one or more existing domain contracts.
 *
 * This is source-level inheritance/extension.
 *
 * It does NOT mean:
 *
 *     hardware inheritance
 *     device inheritance
 *     physical topology inheritance
 *     backend inheritance
 */

domainExtendsClause
    : K_EXTENDS
      domainParentList
    ;

domainParentList
    : qualifiedName
      (COMMA qualifiedName)*
      COMMA?
    ;


/*
 * ============================================================================
 * 4. VERSION
 * ============================================================================
 *
 * Version syntax is intentionally represented as a source literal.
 *
 * Compatibility semantics belong to compatibility/version analysis.
 */

domainVersionClause
    : K_VERSION
      literal
    ;


/*
 * ============================================================================
 * 5. DOMAIN BODY
 * ============================================================================
 */

domainBody
    : LBRACE
      domainMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 6. DOMAIN MEMBER
 * ============================================================================
 *
 * The domain body intentionally owns only constructs that are genuinely
 * domain-level.
 *
 * Ordinary functions, classes, structs, statements, etc. remain owned by
 * their respective declaration/statement grammars and are composed at the
 * appropriate enclosing language boundary.
 *
 * This prevents `domain` from becoming a second universal declaration grammar.
 */

domainMember
    : domainCapabilityDeclaration
    | domainResourceDeclaration
    | domainTypeAliasDeclaration
    | domainRequirementClause
    | domainProvisionClause
    | domainUseClause
    | domainConstraintClause
    | domainPreferenceClause
    | domainHintClause
    | domainProperty
    | domainNestedDeclaration
    ;


/*
 * ============================================================================
 * 7. CAPABILITY DECLARATION
 * ============================================================================
 *
 * Capability identity and capability declaration syntax remain owned by
 * grammar/core/capabilities.g4.
 *
 * This rule is an integration adapter only.
 */

domainCapabilityDeclaration
    : capabilityDeclaration
    ;


/*
 * ============================================================================
 * 8. RESOURCE DECLARATION
 * ============================================================================
 *
 * Resource declaration syntax remains owned by grammar/resources/resources.g4.
 *
 * A domain can declare abstract resource intent without selecting a physical
 * machine resource.
 */

domainResourceDeclaration
    : resourceDeclaration
    ;


/*
 * ============================================================================
 * 9. TYPE ALIAS
 * ============================================================================
 *
 * Type alias syntax remains owned by the declaration alias grammar.
 */

domainTypeAliasDeclaration
    : typeAliasDeclaration
    ;


/*
 * ============================================================================
 * 10. REQUIREMENTS
 * ============================================================================
 *
 * A requirement is mandatory semantic intent.
 *
 * Examples:
 *
 *     requires capability zamani::quantum::dynamic_control;
 *
 *     requires resource zamani::compute::parallel;
 *
 *     requires expression;
 *
 * None of these select a physical implementation.
 */

domainRequirementClause
    : K_REQUIRES
      domainRequirementBody
      SEMICOLON
    ;

domainRequirementBody
    : K_CAPABILITY
      capabilityReference
    | K_RESOURCE
      qualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 11. PROVISIONS
 * ============================================================================
 *
 * A domain may declare capabilities/resources it provides semantically.
 *
 * This describes a contract.
 *
 * It does NOT advertise a particular physical device.
 */

domainProvisionClause
    : K_PROVIDES
      domainProvisionBody
      SEMICOLON
    ;

domainProvisionBody
    : K_CAPABILITY
      capabilityReferenceList
    | K_RESOURCE
      domainResourceReferenceList
    ;

capabilityReferenceList
    : capabilityReference
      (COMMA capabilityReference)*
      COMMA?
    ;

domainResourceReferenceList
    : qualifiedName
      (COMMA qualifiedName)*
      COMMA?
    ;


/*
 * ============================================================================
 * 12. USES
 * ============================================================================
 *
 * `uses` describes a semantic dependency/consumption relationship.
 *
 * It does not perform runtime allocation.
 */

domainUseClause
    : K_USES
      domainUseBody
      SEMICOLON
    ;

domainUseBody
    : K_RESOURCE
      qualifiedName
    | K_CAPABILITY
      capabilityReference
    | qualifiedName
    ;


/*
 * ============================================================================
 * 13. CONSTRAINTS
 * ============================================================================
 *
 * Constraints are mandatory semantic conditions.
 *
 * They are deliberately represented as expressions.
 *
 * The parser does not attempt to decide whether a constraint is satisfiable.
 */

domainConstraintClause
    : K_CONSTRAINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. PREFERENCES
 * ============================================================================
 *
 * Preferences are advisory optimization intent.
 *
 * A preference MUST NOT turn into a hard language requirement merely because
 * a backend cannot satisfy it.
 */

domainPreferenceClause
    : K_PREFERENCE
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. HINTS
 * ============================================================================
 *
 * Hints are advisory.
 */

domainHintClause
    : K_HINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. DOMAIN PROPERTIES
 * ============================================================================
 *
 * Properties provide extensible domain metadata without creating a new
 * keyword for every future computational paradigm.
 *
 * Examples:
 *
 *     semantics: "quantum";
 *     model: some_expression;
 *     profile: future::profile;
 *
 * The parser preserves the expression.
 *
 * Semantic analysis decides whether the property is meaningful.
 */

domainProperty
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. NESTED DOMAIN
 * ============================================================================
 *
 * Domains may be nested.
 *
 * There is deliberately no grammar-level nesting limit.
 */

domainNestedDeclaration
    : domainDeclaration
    ;


/*
 * ============================================================================
 * 18. DOMAIN MEMBER SEQUENCE
 * ============================================================================
 *
 * Source order is significant and MUST be preserved.
 *
 * The grammar does not sort, deduplicate, normalize, or otherwise reorder
 * members.
 *
 * Duplicate declarations are semantic diagnostics.
 */


/*
 * ============================================================================
 * 19. SOURCE-LEVEL EXAMPLES
 * ============================================================================
 *
 * Minimal:
 *
 *     domain example;
 *
 * Parameterized:
 *
 *     domain compute<T: Numeric>;
 *
 * Inherited:
 *
 *     domain quantum_extension
 *         extends zamani::quantum;
 *
 * Versioned:
 *
 *     domain accelerator
 *         version "1.0.0";
 *
 * Contract:
 *
 *     domain hybrid {
 *         requires capability zamani::quantum::dynamic_control;
 *         requires resource zamani::compute::parallel;
 *         provides capability zamani::hybrid::classical_quantum;
 *         constraint available_resources >= required_resources;
 *         prefer execution::low_latency;
 *         hint deployment::adaptive;
 *     }
 *
 * Nested:
 *
 *     domain computing {
 *         domain classical;
 *         domain quantum;
 *         domain hybrid;
 *     }
 *
 * These are source-level declarations.
 *
 * They do not select:
 *
 *     CPU 0
 *     GPU 3
 *     FPGA 2
 *     QPU 7
 *     physical qubit 14
 *     memory bank 1
 *     network node 5
 *
 * ============================================================================
 * 20. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions
 *     no semantic predicates
 *     no randomness
 *     no external state
 *     no timestamps
 *     no filesystem access
 *     no network access
 *
 * A fixed token stream and grammar version therefore produce deterministic
 * parser structure.
 *
 * ============================================================================
 * 21. SOURCE SPANS
 * ============================================================================
 *
 * Every construct represented by the frontend AST MUST retain its complete
 * source span.
 *
 * This grammar itself does not construct spans.
 *
 * The parser/frontend AST layer owns source-span construction.
 *
 * ============================================================================
 * 22. SEMANTIC SEPARATION
 * ============================================================================
 *
 * The parser MUST NOT decide:
 *
 *     whether a domain is valid for a target;
 *     whether a capability exists;
 *     whether a resource exists;
 *     whether resources are sufficient;
 *     whether two capabilities conflict;
 *     whether a quantum operation is physically realizable;
 *     whether QEC is possible;
 *     whether ZQN requirements are satisfiable;
 *     which device should be selected;
 *     which topology should be used;
 *     how scheduling occurs;
 *     how routing occurs.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * 23. HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS:
 *
 *     no fixed domain count
 *     no fixed nesting depth
 *     no fixed member count
 *     no fixed generic count
 *     no fixed capability count
 *     no fixed resource count
 *     no CPU limits
 *     no GPU limits
 *     no FPGA limits
 *     no QPU limits
 *     no qubit limits
 *     no node limits
 *     no memory limits
 *     no topology limits
 *
 * ============================================================================
 * 24. COMPLETION CRITERIA
 * ============================================================================
 *
 * This grammar component is complete only when:
 *
 *     [ ] canonical lexer tokens exist
 *     [ ] parser grammar compiles
 *     [ ] declaration dispatcher imports this grammar
 *     [ ] duplicate root domain grammar is removed from the composition path
 *     [ ] domain AST node exists
 *     [ ] NodeKind registration exists
 *     [ ] source spans are preserved
 *     [ ] structural validation exists
 *     [ ] semantic domain validation exists
 *     [ ] capability resolution exists
 *     [ ] resource analysis exists
 *     [ ] portability analysis exists
 *     [ ] canonical semantic mapping exists
 *     [ ] compiler consumers exist
 *     [ ] runtime consumers exist where required
 *     [ ] positive tests exist
 *     [ ] negative tests exist
 *     [ ] boundary tests exist
 *     [ ] scalability tests exist
 *     [ ] determinism tests exist
 *     [ ] compatibility tests exist
 *     [ ] hard-coding audit passes
 *
 * ============================================================================
 */