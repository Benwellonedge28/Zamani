/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/physical-qubits.g4
 *
 * Purpose:
 *     Canonical parser fragment for explicit physical-qubit intent and
 *     physical-qubit references.
 *
 * Grammar technology:
 *     ANTLR4 parser fragment
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * A physical qubit is part of the hardware vocabulary.
 *
 * This file therefore provides syntax for programs that explicitly need to
 * express physical-resource intent or target-bound physical references.
 *
 * It DOES NOT perform physical allocation.
 *
 * It DOES NOT determine:
 *
 *     - which QPU is selected;
 *     - which device is selected;
 *     - which physical topology exists;
 *     - which physical qubit is available;
 *     - whether a physical identifier exists;
 *     - how logical qubits are mapped;
 *     - how routing is performed;
 *     - how operations are scheduled;
 *     - how calibration is performed;
 *     - how pulses are generated;
 *     - how noise is modeled;
 *     - how QEC is implemented;
 *     - how hardware resources are discovered.
 *
 * Those responsibilities belong downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Normal Zamani quantum source SHOULD remain physical-hardware independent.
 *
 * For example:
 *
 *     qubit q;
 *
 * does NOT mean:
 *
 *     physical qubit 0
 *
 * and:
 *
 *     q[0]
 *
 * means the first element selected from the source-level quantum value `q`;
 * it MUST NOT acquire the semantic meaning "hardware qubit 0".
 *
 * Explicit physical syntax exists only for programs whose semantics genuinely
 * require physical-resource identity or physical-resource constraints.
 *
 * Such programs are target-bound by their semantics and therefore cannot
 * automatically claim full hardware portability.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This file contains NO fixed hardware limits.
 *
 * It does NOT define:
 *
 *     MAX_QUBITS
 *     MAX_PHYSICAL_QUBITS
 *     MAX_DEVICES
 *     MAX_TOPOLOGY_SIZE
 *     MAX_QPU_SIZE
 *     MAX_REGISTER_SIZE
 *     MAX_PHYSICAL_INDEX
 *
 * Physical identifiers are symbolic expressions/references.
 *
 * Numeric identifiers, when required by a particular backend, are interpreted
 * by the target/hardware layer rather than restricted by this grammar.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - explicit physical-qubit declarations;
 *     - physical-qubit references;
 *     - physical-qubit collections;
 *     - physical-qubit indexing;
 *     - physical-qubit ranges;
 *     - physical-qubit groups;
 *     - physical-qubit target expressions;
 *     - physical-resource identity intent;
 *     - explicit physical-resource binding syntax;
 *     - target-bound physical-resource annotations.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - general expressions;
 *     - general types;
 *     - logical-qubit semantics;
 *     - logical-to-physical mapping;
 *     - routing;
 *     - topology;
 *     - scheduling;
 *     - calibration;
 *     - hardware discovery;
 *     - backend selection;
 *     - QEC;
 *     - ZQN;
 *     - optimization;
 *     - canonical quantum IR;
 *     - runtime allocation.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     canonical lexer
 *            |
 *            v
 *     core names / expressions
 *            |
 *            v
 *     physical-qubits.g4
 *            |
 *            v
 *     quantum parser
 *            |
 *            v
 *     frontend AST
 *            |
 *            v
 *     semantic validation
 *            |
 *            +--------------------+
 *            |                    |
 *            v                    v
 *     quantum::ir          hardware/resource model
 *            |                    |
 *            +---------+----------+
 *                      |
 *                      v
 *                 routing / mapping
 *                      |
 *                      v
 *                 scheduling
 *                      |
 *                      v
 *                 hardware HAL
 *                      |
 *                      v
 *                    runtime
 *
 * The dependency direction MUST NOT be reversed.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION RULE
 * ============================================================================
 *
 * This file is a parser fragment.
 *
 * It MUST NOT declare:
 *
 *     grammar PhysicalQubits;
 *     parser grammar PhysicalQubits;
 *     lexer grammar PhysicalQubits;
 *
 * It MUST NOT define lexer tokens.
 *
 * Keyword ownership belongs to the canonical lexer.
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. PHYSICAL QUBIT DECLARATION
 * ========================================================================== */

/*
 * Explicit physical-resource declaration.
 *
 * Examples:
 *
 *     physical qubit p;
 *
 *     physical qubit p : PhysicalQubit;
 *
 * The declaration establishes that `p` is intended to denote a physical
 * quantum resource.
 *
 * It does NOT allocate that resource.
 *
 * The semantic layer must determine whether this declaration is legal in the
 * current compilation profile.
 */
physicalQubitDeclaration
    : K_PHYSICAL
      K_QUBIT
      identifier
      physicalQubitTypeAnnotation?
      SEMICOLON
    ;


/* ============================================================================
 * 2. PHYSICAL QUBIT TYPE ANNOTATION
 * ========================================================================== */

/*
 * PhysicalQubit is a semantic type.
 *
 * The actual type implementation belongs to the language type system.
 *
 * This grammar only recognizes the syntactic annotation.
 */
physicalQubitTypeAnnotation
    : COLON physicalQubitType
    ;


physicalQubitType
    : K_PHYSICAL K_QUBIT
    | qualifiedName
    ;


/* ============================================================================
 * 3. PHYSICAL QUBIT COLLECTION
 * ========================================================================== */

/*
 * A physical collection expresses a collection of physical resources.
 *
 * The extent is an expression.
 *
 * There is deliberately no fixed upper bound.
 *
 * Example:
 *
 *     physical qubit p[count];
 *
 * IMPORTANT:
 *
 * This does NOT imply that the resources are physically contiguous.
 *
 * Physical placement and topology are downstream concerns.
 */
physicalQubitCollectionDeclaration
    : K_PHYSICAL
      K_QUBIT
      identifier
      LBRACKET
      expression
      RBRACKET
      physicalQubitTypeAnnotation?
      SEMICOLON
    ;


/* ============================================================================
 * 4. PHYSICAL QUBIT REFERENCE
 * ========================================================================== */

/*
 * A physical reference is symbolic.
 *
 * The actual identity is resolved against a target/hardware vocabulary.
 *
 * It must never be interpreted by the grammar as a particular hardware
 * address or device.
 */
physicalQubitReference
    : identifier
    ;


/* ============================================================================
 * 5. PHYSICAL QUBIT INDEX
 * ========================================================================== */

/*
 * Example:
 *
 *     p[index]
 *
 * `index` is an expression.
 *
 * The grammar does not restrict it to a fixed-width integer.
 */
physicalQubitIndex
    : physicalQubitReference
      LBRACKET
      expression
      RBRACKET
    ;


/* ============================================================================
 * 6. PHYSICAL QUBIT RANGE
 * ========================================================================== */

/*
 * Examples:
 *
 *     p[start .. end]
 *
 *     p[start ..= end]
 *
 * This is a source-level selection expression.
 *
 * It does NOT imply physical adjacency.
 *
 * A backend may map the selected resources to arbitrary physical locations.
 */
physicalQubitRange
    : physicalQubitReference
      LBRACKET
      expression
      physicalQubitRangeOperator
      expression
      RBRACKET
    ;


physicalQubitRangeOperator
    : DOT_DOT
    | DOT_DOT_EQ
    ;


/* ============================================================================
 * 7. PHYSICAL QUBIT GROUP
 * ========================================================================== */

/*
 * Example:
 *
 *     (p, q, r)
 *
 * The number of group members is not fixed by the grammar.
 */
physicalQubitGroup
    : LPAREN
      physicalQubitReferenceList
      RPAREN
    ;


physicalQubitReferenceList
    : physicalQubitReference
      (COMMA physicalQubitReference)*
    ;


/* ============================================================================
 * 8. PHYSICAL QUBIT TARGET
 * ========================================================================== */

/*
 * Unified integration point for physical operations.
 *
 * This rule intentionally permits:
 *
 *     p
 *     p[index]
 *     p[start .. end]
 *     (p, q, r)
 *
 * without defining a second expression language.
 */
physicalQubitTarget
    : physicalQubitReference
    | physicalQubitIndex
    | physicalQubitRange
    | physicalQubitGroup
    ;


/* ============================================================================
 * 9. PHYSICAL QUBIT TARGET LIST
 * ========================================================================== */

physicalQubitTargetList
    : physicalQubitTarget
      (COMMA physicalQubitTarget)*
    ;


/* ============================================================================
 * 10. PHYSICAL RESOURCE BINDING
 * ========================================================================== */

/*
 * Explicit physical binding is a target-bound semantic operation.
 *
 * Example:
 *
 *     bind physical p;
 *
 * The actual mapping mechanism belongs to routing/hardware compilation.
 *
 * The grammar merely records the user's explicit request.
 */
physicalQubitBinding
    : K_BIND
      K_PHYSICAL
      physicalQubitReference
      SEMICOLON
    ;


/* ============================================================================
 * 11. PHYSICAL RESOURCE GROUP BINDING
 * ========================================================================== */

/*
 * Example:
 *
 *     bind physical (p, q, r);
 *
 * No topology assumption is made.
 */
physicalQubitGroupBinding
    : K_BIND
      K_PHYSICAL
      physicalQubitGroup
      SEMICOLON
    ;


/* ============================================================================
 * 12. PHYSICAL RESOURCE REQUIREMENT
 * ========================================================================== */

/*
 * A physical-resource requirement is distinct from a physical identifier.
 *
 * Example:
 *
 *     requires physical qubit;
 *
 * means that physical quantum resources are required.
 *
 * It does NOT mean:
 *
 *     use physical qubit 0.
 */
physicalQubitRequirement
    : K_REQUIRES
      K_PHYSICAL
      K_QUBIT
      SEMICOLON
    ;


/* ============================================================================
 * 13. PHYSICAL RESOURCE COUNT REQUIREMENT
 * ========================================================================== */

/*
 * A source program may semantically require a number of physical resources.
 *
 * Example:
 *
 *     requires physical qubits(count);
 *
 * `count` is an expression.
 *
 * There is no grammar-level maximum.
 *
 * This requirement is a resource constraint, not a hardware selection.
 */
physicalQubitCountRequirement
    : K_REQUIRES
      K_PHYSICAL
      K_QUBIT
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 14. PHYSICAL RESOURCE CAPABILITY REQUIREMENT
 * ========================================================================== */

/*
 * A program may require a capability of physical quantum resources without
 * naming a particular machine.
 *
 * Example:
 *
 *     requires physical capability;
 *
 * The capability expression itself is interpreted by the resource/capability
 * subsystem.
 */
physicalQubitCapabilityRequirement
    : K_REQUIRES
      K_PHYSICAL
      K_CAPABILITY
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 15. PHYSICAL RESOURCE TARGET REQUIREMENT
 * ========================================================================== */

/*
 * A target requirement is intentionally symbolic.
 *
 * Example:
 *
 *     requires physical target;
 *
 * Target identity belongs to the compilation/deployment layer.
 */
physicalQubitTargetRequirement
    : K_REQUIRES
      K_PHYSICAL
      K_TARGET
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 16. PHYSICAL RESOURCE CONSTRAINT
 * ========================================================================== */

/*
 * A physical constraint expresses a requirement that must be respected by
 * physical realization.
 *
 * It does not implement the constraint.
 *
 * Example:
 *
 *     constrain physical expression;
 *
 * The resource/constraint subsystem performs interpretation.
 */
physicalQubitConstraint
    : K_CONSTRAIN
      K_PHYSICAL
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 17. PHYSICAL RESOURCE PREFERENCE
 * ========================================================================== */

/*
 * Preferences are not requirements.
 *
 * This distinction is essential for POCO-REAF.
 *
 * A preference may be ignored or replaced by the compiler when necessary.
 */
physicalQubitPreference
    : K_PREFER
      K_PHYSICAL
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 18. PHYSICAL RESOURCE HINT
 * ========================================================================== */

/*
 * A hint is advisory.
 *
 * It must never silently become a semantic requirement.
 */
physicalQubitHint
    : K_HINT
      K_PHYSICAL
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 19. PHYSICAL QUBIT IDENTITY EXPRESSION
 * ========================================================================== */

/*
 * Physical identity is target-specific.
 *
 * Therefore the grammar accepts an expression rather than embedding a
 * particular physical-ID representation.
 *
 * Examples:
 *
 *     hardware_qubit
 *
 *     target.qubit
 *
 *     device_resource
 *
 *     symbolic_id
 *
 * Numeric identifiers may be represented by the general expression/literal
 * system when a target-specific program genuinely needs them.
 *
 * This grammar imposes no range.
 */
physicalQubitIdentity
    : expression
    ;


/* ============================================================================
 * 20. EXPLICIT PHYSICAL REFERENCE
 * ========================================================================== */

/*
 * Explicit qualification makes physical intent visible.
 *
 * Example:
 *
 *     physical(p)
 *
 * This syntax is deliberately separate from ordinary:
 *
 *     p
 *
 * so that a normal identifier never becomes physical merely by context.
 */
physicalQubitExplicitReference
    : K_PHYSICAL
      LPAREN
      physicalQubitIdentity
      RPAREN
    ;


/* ============================================================================
 * 21. PHYSICAL TARGET EXPRESSION
 * ========================================================================== */

/*
 * Unified integration point for target-bound quantum operations.
 */
physicalQubitTargetExpression
    : physicalQubitReference
    | physicalQubitIndex
    | physicalQubitRange
    | physicalQubitGroup
    | physicalQubitExplicitReference
    ;


/* ============================================================================
 * 22. PHYSICAL OPERATION TARGETS
 * ========================================================================== */

/*
 * Physical operation syntax is intentionally generic.
 *
 * Gate semantics remain owned by gates.g4 / operations.g4.
 *
 * This rule merely supplies physical targets to those constructs.
 */
physicalOperationTarget
    : physicalQubitTargetExpression
    ;


physicalOperationTargetList
    : physicalOperationTarget
      (COMMA physicalOperationTarget)*
    ;


/* ============================================================================
 * 23. PHYSICAL RESOURCE VIEW
 * ========================================================================== */

/*
 * A physical view is a source-level derived reference.
 *
 * Example:
 *
 *     physical view = p[start .. end];
 *
 * The ownership/borrowing/copy semantics belong to the type/effect system.
 */
physicalQubitViewDeclaration
    : K_PHYSICAL
      K_VIEW
      identifier
      ASSIGN
      physicalQubitTargetExpression
      SEMICOLON
    ;


/* ============================================================================
 * 24. PHYSICAL RESOURCE ALIAS
 * ========================================================================== */

/*
 * Alias creation is syntactic only.
 *
 * Whether aliases are legal for a given physical resource is determined by
 * semantic analysis.
 */
physicalQubitAliasDeclaration
    : K_PHYSICAL
      K_ALIAS
      identifier
      ASSIGN
      physicalQubitTargetExpression
      SEMICOLON
    ;


/* ============================================================================
 * 25. PHYSICAL RESOURCE DECLARATION UNION
 * ========================================================================== */

/*
 * Integration point for quantum.g4 and hardware/quantum-device.g4.
 */
physicalQubitDeclarationStatement
    : physicalQubitDeclaration
    | physicalQubitCollectionDeclaration
    | physicalQubitViewDeclaration
    | physicalQubitAliasDeclaration
    ;


/* ============================================================================
 * 26. PHYSICAL RESOURCE REQUIREMENT UNION
 * ========================================================================== */

physicalQubitRequirementStatement
    : physicalQubitRequirement
    | physicalQubitCountRequirement
    | physicalQubitCapabilityRequirement
    | physicalQubitTargetRequirement
    ;


/* ============================================================================
 * 27. PHYSICAL RESOURCE POLICY UNION
 * ========================================================================== */

physicalQubitPolicyStatement
    : physicalQubitConstraint
    | physicalQubitPreference
    | physicalQubitHint
    ;


/* ============================================================================
 * 28. PHYSICAL RESOURCE STATEMENT
 * ========================================================================== */

/*
 * Single integration point for consumers that need all explicit physical
 * resource syntax.
 */
physicalQubitStatement
    : physicalQubitDeclarationStatement
    | physicalQubitBinding
    | physicalQubitGroupBinding
    | physicalQubitRequirementStatement
    | physicalQubitPolicyStatement
    ;


/* ============================================================================
 * 29. SEMANTIC BOUNDARY
 * ========================================================================== */

/*
 * IMPORTANT:
 *
 * The following meanings MUST be established during semantic analysis.
 *
 * `physicalQubitReference`
 *     -> source-level symbolic physical resource
 *
 * `physicalQubitIdentity`
 *     -> target-specific identity expression
 *
 * `physicalQubitCountRequirement`
 *     -> resource requirement
 *
 * `physicalQubitCapabilityRequirement`
 *     -> capability requirement
 *
 * `physicalQubitConstraint`
 *     -> compilation/resource constraint
 *
 * `physicalQubitPreference`
 *     -> optimization preference
 *
 * `physicalQubitHint`
 *     -> advisory information
 *
 * None of these productions allocates hardware.
 */


/* ============================================================================
 * 30. PORTABILITY BOUNDARY
 * ========================================================================== */

/*
 * Portable source:
 *
 *     qubit q;
 *
 * Target-independent.
 *
 * Explicit physical source:
 *
 *     physical qubit p;
 *
 * Potentially target-bound.
 *
 * The semantic analyzer MUST therefore classify physical syntax as one of:
 *
 *     portable
 *     capability-dependent
 *     target-constrained
 *     target-bound
 *     non-portable
 *
 * according to the actual construct.
 *
 * The grammar itself must not silently classify all physical syntax as
 * portable.
 */


/* ============================================================================
 * 31. FORBIDDEN SEMANTIC INTERPRETATIONS
 * ========================================================================== */

/*
 * The parser/grammar integration MUST NOT interpret:
 *
 *     p[0]
 *
 * as:
 *
 *     physical qubit 0
 *
 * unless `p` is already semantically known to be a physical-resource
 * collection.
 *
 * Likewise:
 *
 *     q[0]
 *
 * remains a source-level selection from `q`.
 *
 * It MUST NOT automatically become:
 *
 *     physical qubit 0
 *
 * during parsing.
 */


/* ============================================================================
 * 32. IR BOUNDARY
 * ========================================================================== */

/*
 * This file creates NO IR type.
 *
 * Parsed physical-qubit syntax must first become frontend semantic data.
 *
 * Semantic lowering then maps it into the canonical quantum representation
 * and/or the hardware/resource representation as appropriate.
 *
 * There must be no:
 *
 *     PhysicalQubitId
 *
 * grammar type.
 *
 * There must be no:
 *
 *     HardwareQubit
 *
 * grammar struct.
 *
 * There must be no:
 *
 *     topology graph
 *
 * grammar object.
 *
 * There must be no:
 *
 *     device allocation table
 *
 * grammar object.
 *
 * Those belong downstream.
 */


/* ============================================================================
 * 33. END OF FILE
 * ========================================================================== */