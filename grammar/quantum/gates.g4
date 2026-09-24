/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/gates.g4
 *
 * Grammar:
 *     QuantumGates
 *
 * Status:
 *     CANONICAL / PRODUCTION QUANTUM GATE-DEFINITION GRAMMAR
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL GATE DEFINITION AND GATE-NAME SYNTAX.
 *
 * It does NOT own quantum operation invocation.
 *
 * Operation invocation is owned by:
 *
 *     grammar/quantum/operations.g4
 *
 * Examples of invocation:
 *
 *     apply H(q);
 *     apply X(q);
 *     apply CNOT(control, target);
 *     apply RX(theta)(q);
 *     apply custom_gate(q);
 *     apply vendor::operation(parameter)(q);
 *
 * This file instead owns declarations such as:
 *
 *     gate bell(a: qubit, b: qubit) {
 *         apply H(a);
 *         apply CNOT(a, b);
 *     }
 *
 *     gate rotation(theta: Angle, q: qubit) {
 *         apply RX(theta)(q);
 *     }
 *
 * Gate names remain OPEN-ENDED identifiers.
 *
 * This grammar deliberately does NOT enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     S
 *     T
 *     CNOT
 *     CX
 *     CZ
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *     U
 *     vendor gates
 *     simulator gates
 *     future gates
 *
 * Standard-gate identity is resolved semantically.
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - gate declarations;
 *     - gate names;
 *     - gate declaration signatures;
 *     - gate formal parameters;
 *     - gate formal operands;
 *     - gate declaration bodies;
 *     - gate-level attributes;
 *     - reusable gate declaration syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified-name syntax;
 *     - general expressions;
 *     - general types;
 *     - qubit declarations;
 *     - quantum-register declarations;
 *     - qubit references;
 *     - operation invocation;
 *     - operation parameter invocation;
 *     - operation targets;
 *     - measurements;
 *     - reset;
 *     - barriers;
 *     - observables;
 *     - dynamic circuits;
 *     - physical qubits;
 *     - topology;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - calibration;
 *     - hardware selection;
 *     - resource allocation;
 *     - capability discovery;
 *     - canonical quantum::ir;
 *     - backend execution.
 *
 * ============================================================================
 * CRITICAL OWNERSHIP RULE
 * ============================================================================
 *
 * There MUST be one owner for each syntactic concept.
 *
 * Therefore:
 *
 *     gates.g4
 *         -> gate declaration syntax
 *
 *     operations.g4
 *         -> operation invocation syntax
 *
 *     parameterized-operations.g4
 *         -> parameter declaration/binding extensions where applicable
 *
 *     controlled-operations.g4
 *         -> controlled-operation extensions
 *
 *     qubits.g4
 *         -> qubit declaration/reference syntax
 *
 *     quantum-registers.g4
 *         -> quantum-register syntax
 *
 *     measurement.g4
 *         -> measurement syntax
 *
 *     reset.g4
 *         -> reset syntax
 *
 *     circuits.g4
 *         -> circuit syntax
 *
 *     quantum::ir
 *         -> canonical quantum semantic representation
 *
 * No rule in this file may duplicate one of those owners.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Gate definitions describe reusable logical computation.
 *
 * They MUST NOT bind a gate definition to:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     physical qubit
 *     physical register
 *     device identifier
 *     vendor device
 *     coupling map
 *     topology
 *     pulse
 *     calibration
 *     clock period
 *     physical channel
 *     scheduler
 *     router
 *
 * The same gate definition must remain representable on any compatible
 * implementation.
 *
 * Example:
 *
 *     gate bell(a: qubit, b: qubit) {
 *         apply H(a);
 *         apply CNOT(a, b);
 *     }
 *
 * describes logical computation.
 *
 * It does not state:
 *
 *     use QPU 0
 *     use physical qubit 17
 *     use coupling edge 17 -> 18
 *     use a particular pulse
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level limits for:
 *
 *     gate count;
 *     gate declaration count;
 *     formal parameter count;
 *     formal operand count;
 *     gate-body statement count;
 *     source program size;
 *     qubit count;
 *     register count;
 *     circuit depth;
 *     operation count;
 *     target count;
 *     device count.
 *
 * This file therefore contains none of:
 *
 *     MAX_GATES
 *     MAX_GATE_COUNT
 *     MAX_PARAMETERS
 *     MAX_OPERANDS
 *     MAX_QUBITS
 *     MAX_REGISTERS
 *     MAX_CIRCUIT_DEPTH
 *     MAX_DEVICES
 *     MAX_QPUS
 *
 * or equivalent artificial language limits.
 *
 * ANTLR repetition operators such as `*` and `+` express grammatical
 * cardinality without establishing a machine capacity.
 *
 * Actual resource limits belong to explicit resource/compiler/runtime
 * policies and target capabilities.
 *
 * ============================================================================
 * STANDARD GATES VS EXTENSIBLE GATES
 * ============================================================================
 *
 * A standard gate may be known by the semantic system:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     RX
 *     ...
 *
 * But the parser does NOT need a closed list of these names.
 *
 * The following are all syntactically valid gate names:
 *
 *     H
 *     X
 *     CNOT
 *     RX
 *     custom_gate
 *     library::gate
 *     vendor::operation
 *     future::gate
 *
 * Their meaning is determined downstream.
 *
 * This is required for:
 *
 *     future quantum architectures;
 *     user-defined gates;
 *     vendor extensions;
 *     simulator operations;
 *     research operations;
 *     dialects;
 *     imported operation libraries;
 *     new standards.
 *
 * ============================================================================
 * CANONICAL DOWNSTREAM PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     QuantumGates
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type validation
 *          +--> operand validation
 *          +--> effect validation
 *          +--> capability validation
 *          +--> resource validation
 *          |
 *          v
 *     canonical quantum semantic representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
 *          +--> decomposition
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> QEC
 *          +--> ZQN
 *          +--> HAL
 *          |
 *          v
 *     target lowering
 *          |
 *          v
 *     runtime
 *
 * This file must never bypass the AST or semantic layer.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The canonical production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Operation/gate names are NOT dedicated lexer tokens.
 *
 * The lexer must therefore allow arbitrary valid identifiers to represent:
 *
 *     standard gate names;
 *     user-defined gates;
 *     vendor names;
 *     library names;
 *     future operation names.
 *
 * This prevents the lexer from becoming a registry of current quantum
 * technology.
 *
 * ============================================================================
 * QUALIFIED NAMES
 * ============================================================================
 *
 * Qualified gate names use the canonical name grammar.
 *
 * Examples:
 *
 *     bell
 *     library::bell
 *     vendor::native_gate
 *     research::new_gate
 *
 * This file does not define another qualified-name implementation.
 *
 * ============================================================================
 * FORMAL SIGNATURE MODEL
 * ============================================================================
 *
 * Gate declarations use one canonical formal parameter list.
 *
 * Example:
 *
 *     gate bell(a: qubit, b: qubit) {
 *         ...
 *     }
 *
 *     gate rotation(theta: Angle, q: qubit) {
 *         ...
 *     }
 *
 * This avoids the ambiguity present in the previous grammar where two
 * adjacent parenthesized clauses could be interpreted as:
 *
 *     parameters
 *
 * versus:
 *
 *     operands
 *
 * Formal arguments are ordinary typed declarations.
 *
 * Semantic analysis determines whether a formal is:
 *
 *     quantum operand;
 *     classical parameter;
 *     symbolic parameter;
 *     type parameter;
 *     resource parameter;
 *     another supported formal category.
 *
 * The grammar does not hard-code a finite list of quantum parameter types.
 *
 * ============================================================================
 * GATE BODY
 * ============================================================================
 *
 * Gate bodies contain quantum operation statements.
 *
 * Operation invocation remains owned by:
 *
 *     grammar/quantum/operations.g4
 *
 * Therefore this file does not redefine:
 *
 *     apply H(q);
 *     apply RX(theta)(q);
 *     apply vendor::operation(q);
 *
 * Gate bodies consume the canonical operation grammar.
 *
 * ============================================================================
 * GATE RECURSION
 * ============================================================================
 *
 * A gate may invoke another previously declared or imported operation/gate.
 *
 * Semantic analysis is responsible for detecting:
 *
 *     direct recursion;
 *     indirect recursion;
 *     illegal cyclic dependencies;
 *     recursive definitions requiring special treatment.
 *
 * The parser does not impose a finite gate-call nesting depth.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Formal parameter types are delegated to the canonical type grammar.
 *
 * The grammar does not enumerate:
 *
 *     qubit;
 *     logical_qubit;
 *     Angle;
 *     Float;
 *     Tensor;
 *     custom types;
 *
 * as an exhaustive type vocabulary.
 *
 * A quantum operand type can therefore evolve without requiring this grammar
 * to be rewritten for every future semantic type.
 *
 * ============================================================================
 * ATTRIBUTE CONTRACT
 * ============================================================================
 *
 * Gate declarations may carry attributes through the canonical attribute
 * grammar.
 *
 * Attributes may describe source-level intent such as:
 *
 *     logical;
 *     pure;
 *     adjointable;
 *     controlled;
 *     differentiable;
 *     approximate;
 *     reusable.
 *
 * Attribute semantics are NOT defined here.
 *
 * In particular, an attribute must not silently select:
 *
 *     a hardware device;
 *     a physical qubit;
 *     a topology;
 *     a calibration;
 *     a backend;
 *     a scheduler.
 *
 * ============================================================================
 * SEMANTIC VALIDATION CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis must validate:
 *
 *     1. gate name resolution;
 *     2. duplicate declarations;
 *     3. declaration visibility;
 *     4. formal parameter validity;
 *     5. formal parameter type validity;
 *     6. quantum operand validity;
 *     7. parameter/operand classification;
 *     8. duplicate formal names;
 *     9. body validity;
 *    10. operation resolution;
 *    11. operation argument compatibility;
 *    12. recursive gate dependencies;
 *    13. modifier capabilities;
 *    14. required effects;
 *    15. required capabilities;
 *    16. required resources;
 *    17. target compatibility.
 *
 * None of these checks belong in the parser grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough structure for the frontend AST to represent:
 *
 *     gate name;
 *     qualified name;
 *     formal declaration order;
 *     formal names;
 *     formal types;
 *     attributes;
 *     gate body;
 *     source spans.
 *
 * A representative semantic shape is:
 *
 *     GateDeclaration {
 *         name,
 *         formals,
 *         attributes,
 *         body,
 *         source_span
 *     }
 *
 * The exact Rust AST representation remains owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar must not define another AST model.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * A gate declaration does not create a second quantum IR.
 *
 * After semantic analysis:
 *
 *     GateDeclaration
 *          |
 *          v
 *     semantic operation definition
 *          |
 *          v
 *     quantum::ir
 *
 * Any reusable operation representation must converge on the canonical
 * quantum semantic/IR architecture already established by the repository.
 *
 * This grammar must not introduce:
 *
 *     QuantumGate AST;
 *     PhysicalGate AST;
 *     VendorGate IR;
 *     GateIR;
 *     HardwareGate IR;
 *
 * as competing canonical representations.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Gate syntax itself does not allocate resources.
 *
 * A gate definition can be semantically analyzed for resource requirements,
 * but resource availability belongs to:
 *
 *     grammar/resources/
 *     semantic analysis
 *     compiler resource analysis
 *     hardware capability analysis
 *     HAL
 *     runtime
 *
 * Example:
 *
 *     gate algorithm_step(q: qubit) {
 *         ...
 *     }
 *
 * does not establish:
 *
 *     MAX_QUBITS = 1
 *
 * or:
 *
 *     physical_qubit = 0
 *
 * ============================================================================
 * HARDWARE-INDEPENDENCE CONTRACT
 * ============================================================================
 *
 * This grammar must never encode:
 *
 *     physical_qubit_0
 *     physical_qubit_1
 *     qpu_0
 *     gpu_0
 *     device_0
 *     coupling_map
 *     topology
 *     pulse_id
 *     calibration_id
 *     clock_frequency
 *     fixed_gate_duration
 *     fixed_native_gate_set
 *
 * Hardware realization is downstream.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given the same:
 *
 *     source;
 *     language version;
 *     imported grammar versions;
 *
 * parsing must produce the same structural result.
 *
 * The grammar does not:
 *
 *     discover hardware;
 *     inspect files;
 *     inspect environment variables;
 *     access the network;
 *     invoke a backend;
 *     invoke a scheduler;
 *     invoke a router;
 *     use randomness.
 *
 * Formal declaration order must be preserved.
 *
 * Gate body statement order must be preserved.
 *
 * Attribute order must be preserved where the AST contract requires source
 * fidelity.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no executable actions.
 *
 * It performs no:
 *
 *     filesystem access;
 *     network access;
 *     command execution;
 *     device access;
 *     hardware discovery;
 *     credential access;
 *     runtime execution.
 *
 * Parsing untrusted source therefore remains a source-processing operation.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It uses:
 *
 *     tokenVocab = ZamaniLexer
 *
 * It imports reusable parser grammars rather than declaring lexer rules.
 *
 * The grammar name and filename intentionally match:
 *
 *     QuantumGates
 *     gates.g4
 *
 * ============================================================================
 */

parser grammar QuantumGates;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * Names:
 *     qualifiedName
 *     identifier
 *
 * Expressions:
 *     expression
 *
 * Types:
 *     typeExpression
 *
 * Operations:
 *     quantumOperationStatement
 *
 * Attributes:
 *     attributes
 *
 * The detailed implementations remain owned by their canonical grammars.
 */
import
    Names,
    Expressions,
    Types,
    QuantumOperations
;


/*
 * ============================================================================
 * 1. GATE DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     gate bell(a: qubit, b: qubit) {
 *         apply H(a);
 *         apply CNOT(a, b);
 *     }
 *
 *     gate rotation(theta: Angle, q: qubit) {
 *         apply RX(theta)(q);
 *     }
 *
 * No hardware information is encoded.
 */
quantumGateDeclaration
    : gateAttributePrefix*
      K_GATE
      gateName
      gateFormalParameterClause
      gateBody
    ;


/*
 * ============================================================================
 * 2. GATE NAME
 * ============================================================================
 *
 * Gate names are ordinary qualified names.
 *
 * No fixed gate inventory is encoded here.
 *
 * Therefore:
 *
 *     H
 *     X
 *     CNOT
 *     custom_gate
 *     vendor::operation
 *     future::gate
 *
 * are all represented by the same syntactic name mechanism.
 */
gateName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 3. GATE FORMAL PARAMETER CLAUSE
 * ============================================================================
 *
 * A single canonical formal list avoids the previous ambiguity between:
 *
 *     parameter list
 *
 * and:
 *
 *     operand list.
 *
 * The semantic layer classifies formals according to their types and
 * declaration context.
 */
gateFormalParameterClause
    : LPAREN
      gateFormalParameterList?
      RPAREN
    ;


/*
 * ============================================================================
 * 4. FORMAL PARAMETER LIST
 * ============================================================================
 *
 * No finite number of parameters is imposed.
 *
 * Examples:
 *
 *     theta: Angle
 *
 *     q: qubit
 *
 *     control: qubit
 *
 *     target: qubit
 *
 *     data: Tensor<Scalar, shape>
 *
 * Type meaning belongs to the canonical type system.
 */
gateFormalParameterList
    : gateFormalParameter
      (
          COMMA
          gateFormalParameter
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 5. FORMAL PARAMETER
 * ============================================================================
 *
 * Formal declarations preserve:
 *
 *     name;
 *     type;
 *     optional default;
 *
 * Semantic analysis determines whether the formal is:
 *
 *     classical;
 *     quantum;
 *     symbolic;
 *     generic;
 *     resource-related;
 *     another valid Zamani semantic category.
 */
gateFormalParameter
    : identifier
      COLON
      typeExpression
      gateFormalDefault?
    ;


/*
 * ============================================================================
 * 6. FORMAL DEFAULT
 * ============================================================================
 *
 * Default values are ordinary Zamani expressions.
 *
 * Example:
 *
 *     theta: Angle = pi / 2
 *
 * The grammar does not evaluate the expression.
 */
gateFormalDefault
    : EQUAL
      expression
    ;


/*
 * ============================================================================
 * 7. GATE BODY
 * ============================================================================
 *
 * Operation invocation remains owned by QuantumOperations.
 *
 * Therefore this grammar consumes:
 *
 *     quantumOperationStatement
 *
 * rather than redefining operation syntax.
 */
gateBody
    : LBRACE
      quantumGateBodyItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 8. GATE BODY ITEM
 * ============================================================================
 *
 * The gate body is deliberately based on the canonical operation grammar.
 *
 * Additional quantum constructs may be admitted by the higher-level quantum
 * composition grammar without changing the gate-definition ownership here.
 */
quantumGateBodyItem
    : quantumOperationStatement
    ;


/*
 * ============================================================================
 * 9. GATE DECLARATION ATTRIBUTES
 * ============================================================================
 *
 * Attributes are intentionally represented through a small syntactic bridge.
 *
 * The actual attribute semantics remain owned by the canonical attributes
 * system.
 *
 * Examples:
 *
 *     @logical
 *     @adjointable
 *     @differentiable
 *
 * An attribute never automatically means:
 *
 *     physical hardware;
 *     target device;
 *     calibration;
 *     topology.
 */
gateAttributePrefix
    : AT
      qualifiedName
      gateAttributeArguments?
    ;


/*
 * ============================================================================
 * 10. GATE ATTRIBUTE ARGUMENTS
 * ============================================================================
 *
 * Attribute arguments are ordinary expressions.
 *
 * No hardware-specific value vocabulary is introduced.
 */
gateAttributeArguments
    : LPAREN
      gateAttributeArgumentList?
      RPAREN
    ;

gateAttributeArgumentList
    : gateAttributeArgument
      (
          COMMA
          gateAttributeArgument
      )*
      COMMA?
    ;

gateAttributeArgument
    : expression
    ;


/*
 * ============================================================================
 * 11. GATE REFERENCE
 * ============================================================================
 *
 * A gate reference is intentionally just a canonical qualified name.
 *
 * This rule exists as a named integration point for semantic consumers.
 */
gateReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 12. GATE DEFINITION FAMILY
 * ============================================================================
 *
 * This rule is the reusable family entry point for grammar composition.
 */
quantumGateDefinition
    : quantumGateDeclaration
    ;


/*
 * ============================================================================
 * 13. GATE DECLARATION LIST
 * ============================================================================
 *
 * No gate-count limit exists.
 */
quantumGateDefinitions
    : quantumGateDefinition*
    ;


/*
 * ============================================================================
 * 14. GATE SIGNATURE
 * ============================================================================
 *
 * Named integration rule for consumers that need to inspect a gate's
 * declaration signature independently of its body.
 */
gateSignature
    : gateFormalParameterClause
    ;


/*
 * ============================================================================
 * 15. GATE FORMAL
 * ============================================================================
 *
 * Named wrapper for AST/semantic integration.
 */
gateFormal
    : gateFormalParameter
    ;


/*
 * ============================================================================
 * 16. GATE BODY OPERATION
 * ============================================================================
 *
 * Named integration point for semantic tooling.
 *
 * The actual syntax remains owned by QuantumOperations.
 */
gateBodyOperation
    : quantumOperationStatement
    ;


/*
 * ============================================================================
 * 17. SEMANTIC CLASSIFICATION CONTRACT
 * ============================================================================
 *
 * The grammar intentionally does NOT contain:
 *
 *     quantumFormalParameter
 *     classicalFormalParameter
 *     physicalQubitParameter
 *     hardwareParameter
 *
 * as mutually exclusive closed parser categories.
 *
 * Instead:
 *
 *     identifier : typeExpression
 *
 * is the common syntactic representation.
 *
 * Semantic analysis determines the meaning of the type.
 *
 * This is important for future types and dialects.
 */


/*
 * ============================================================================
 * 18. QUANTUM OPERAND SEMANTICS
 * ============================================================================
 *
 * A formal such as:
 *
 *     q: qubit
 *
 * can be recognized semantically as a quantum operand.
 *
 * A formal such as:
 *
 *     q: logical_qubit
 *
 * can be recognized semantically according to the type system.
 *
 * A formal such as:
 *
 *     q: Qubit[n]
 *
 * can express a scalable semantic type/contract.
 *
 * This grammar does not impose a maximum value for n.
 */


/*
 * ============================================================================
 * 19. PARAMETER SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no artificial maximum for:
 *
 *     formal parameters;
 *     parameter expression size;
 *     gate body statements;
 *     gate definitions.
 *
 * External parser/compiler resource policies may impose operational limits,
 * but those are not language semantics.
 */


/*
 * ============================================================================
 * 20. GATE COMPOSITION
 * ============================================================================
 *
 * Gate definitions compose through ordinary operation invocation.
 *
 * Example:
 *
 *     gate bell(a: qubit, b: qubit) {
 *         apply H(a);
 *         apply CNOT(a, b);
 *     }
 *
 *     gate larger(c: qubit, d: qubit, e: qubit) {
 *         apply bell(c, d);
 *         apply bell(d, e);
 *     }
 *
 * The parser does not decide whether the called name is:
 *
 *     a primitive gate;
 *     a user-defined gate;
 *     an imported gate;
 *     a dialect operation;
 *     a library operation.
 *
 * Name resolution decides this downstream.
 */


/*
 * ============================================================================
 * 21. RECURSION
 * ============================================================================
 *
 * Recursive gate definitions remain syntactically representable.
 *
 * Semantic analysis MUST reject or otherwise handle illegal recursion.
 *
 * The parser must not encode a finite recursion depth.
 */


/*
 * ============================================================================
 * 22. MODIFIERS
 * ============================================================================
 *
 * Controlled, adjoint, inverse and other operation modifiers belong to:
 *
 *     grammar/quantum/operations.g4
 *     grammar/quantum/controlled-operations.g4
 *     related operation grammars.
 *
 * This file does not duplicate them.
 */


/*
 * ============================================================================
 * 23. PARAMETERIZED OPERATIONS
 * ============================================================================
 *
 * Gate formal parameters are declarations.
 *
 * Operation invocation parameters remain owned by:
 *
 *     grammar/quantum/operations.g4
 *
 * This distinction prevents two competing parameter grammars.
 */


/*
 * ============================================================================
 * 24. MEASUREMENT
 * ============================================================================
 *
 * Measurement syntax is not owned here.
 *
 * Gate bodies may only consume measurement syntax if the higher-level
 * language explicitly permits it through the canonical quantum statement
 * composition.
 *
 * This prevents gates.g4 from becoming a second quantum-statement grammar.
 */


/*
 * ============================================================================
 * 25. RESET
 * ============================================================================
 *
 * Reset syntax is not owned here.
 *
 * Its owner remains:
 *
 *     grammar/quantum/reset.g4
 */


/*
 * ============================================================================
 * 26. BARRIER
 * ============================================================================
 *
 * Barrier syntax is not owned here.
 *
 * Its owner remains the canonical barrier grammar.
 *
 * A gate definition therefore does not silently acquire a hardware-specific
 * synchronization primitive.
 */


/*
 * ============================================================================
 * 27. PHYSICAL REALIZATION
 * ============================================================================
 *
 * A gate declaration never specifies:
 *
 *     physical qubit;
 *     coupling edge;
 *     pulse;
 *     calibration;
 *     native instruction;
 *     device;
 *     target;
 *     routing path.
 *
 * These are downstream realization decisions.
 */


/*
 * ============================================================================
 * 28. RESOURCE REQUIREMENTS
 * ============================================================================
 *
 * A gate declaration may eventually participate in semantic resource
 * requirements.
 *
 * Example semantic requirement:
 *
 *     requires capability("quantum.mid_circuit_measurement")
 *
 * But resource requirement syntax is owned by:
 *
 *     grammar/resources/
 *
 * This grammar must not duplicate that syntax.
 */


/*
 * ============================================================================
 * 29. CAPABILITIES
 * ============================================================================
 *
 * Capability discovery is downstream.
 *
 * A gate may require a capability through semantic metadata, but the grammar
 * does not query or enumerate hardware capabilities.
 */


/*
 * ============================================================================
 * 30. DIALECTS
 * ============================================================================
 *
 * Dialect-specific gate names remain identifiers/qualified names.
 *
 * Example:
 *
 *     ion::custom_gate
 *
 *     photonic::operation
 *
 *     superconducting::operation
 *
 *     future::operation
 *
 * The parser does not need to know the vendor or technology.
 *
 * Dialect validation belongs to the dialect subsystem.
 */


/*
 * ============================================================================
 * 31. STANDARD-GATE COMPATIBILITY
 * ============================================================================
 *
 * Existing standard names remain source-compatible because they are parsed as
 * ordinary identifiers.
 *
 * Therefore a source program can continue to express:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *
 * without placing those names into the grammar as a closed enumeration.
 *
 * This is the critical difference between:
 *
 *     language syntax
 *
 * and:
 *
 *     current operation registry.
 */


/*
 * ============================================================================
 * 32. OPEN-ENDED FUTURE COMPATIBILITY
 * ============================================================================
 *
 * Future operation names do not require grammar changes.
 *
 * For example:
 *
 *     next_generation::gate
 *
 * remains syntactically valid if it satisfies the canonical name grammar.
 *
 * Semantic analysis determines whether it exists.
 *
 * This allows Zamani to evolve without continuously editing gates.g4 merely
 * because quantum technology evolves.
 */


/*
 * ============================================================================
 * 33. AST LOWERING
 * ============================================================================
 *
 * The AST layer must preserve:
 *
 *     gate name;
 *     formal order;
 *     formal names;
 *     formal types;
 *     defaults;
 *     attributes;
 *     body;
 *     source spans.
 *
 * The grammar must not lower directly into quantum::ir.
 */


/*
 * ============================================================================
 * 34. QUANTUM IR
 * ============================================================================
 *
 * The canonical quantum IR remains:
 *
 *     quantum::ir
 *
 * Gate syntax is lowered into that existing architecture.
 *
 * No:
 *
 *     QuantumGateIR
 *     GateDefinitionIR
 *     VendorGateIR
 *
 * may become a competing canonical quantum IR.
 */


/*
 * ============================================================================
 * 35. OPTIMIZATION
 * ============================================================================
 *
 * Optimization may later:
 *
 *     inline;
 *     specialize;
 *     decompose;
 *     simplify;
 *     fuse;
 *     eliminate;
 *     reorder where semantically legal.
 *
 * None of those transformations belong in this grammar.
 */


/*
 * ============================================================================
 * 36. ROUTING
 * ============================================================================
 *
 * Routing determines physical realization downstream.
 *
 * gates.g4 must not encode:
 *
 *     coupling maps;
 *     physical adjacency;
 *     topology;
 *     qubit placement.
 */


/*
 * ============================================================================
 * 37. SCHEDULING
 * ============================================================================
 *
 * Scheduling determines ordering/timing at the target level.
 *
 * Gate syntax does not encode fixed durations or clock periods.
 */


/*
 * ============================================================================
 * 38. QEC
 * ============================================================================
 *
 * QEC may transform or protect operations after semantic lowering.
 *
 * gates.g4 does not implement:
 *
 *     stabilizers;
 *     syndromes;
 *     decoders;
 *     code distance;
 *     recovery;
 *     logical-to-physical encoding.
 */


/*
 * ============================================================================
 * 39. ZQN
 * ============================================================================
 *
 * ZQN/fault/noise semantics remain downstream.
 *
 * Gate declarations do not define:
 *
 *     physical error probability;
 *     calibration;
 *     noise channels;
 *     leakage;
 *     transport error.
 */


/*
 * ============================================================================
 * 40. HAL
 * ============================================================================
 *
 * HAL determines actual target capabilities.
 *
 * gates.g4 remains target independent.
 */


/*
 * ============================================================================
 * 41. ERROR MODEL
 * ============================================================================
 *
 * Syntax errors include:
 *
 *     missing gate name;
 *     malformed formal list;
 *     malformed type;
 *     malformed default expression;
 *     malformed body;
 *     malformed attribute syntax.
 *
 * Semantic errors include:
 *
 *     duplicate gate name;
 *     duplicate formal;
 *     unresolved type;
 *     invalid quantum operand type;
 *     invalid operation invocation;
 *     recursive definition;
 *     unavailable capability;
 *     unavailable resource.
 *
 * Resource errors must NOT be reported as syntax errors.
 */


/*
 * ============================================================================
 * 42. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The grammar must reject structurally malformed examples such as:
 *
 *     gate {
 *     gate bell {
 *     gate bell(a: ) {
 *     gate bell(a qubit) {
 *     gate bell(a: qubit {
 *     gate bell(a: qubit)) {
 *
 * Semantic validation must separately reject examples such as:
 *
 *     duplicate formal names;
 *     unknown types;
 *     unresolved operation names;
 *     illegal recursive definitions;
 *     incompatible quantum operands.
 */


/*
 * ============================================================================
 * 43. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * Required examples include:
 *
 *     gate bell(a: qubit, b: qubit) {
 *         apply H(a);
 *         apply CNOT(a, b);
 *     }
 *
 *     gate rotation(theta: Angle, q: qubit) {
 *         apply RX(theta)(q);
 *     }
 *
 *     gate custom(q: Qubit[n]) {
 *         apply library::operation(q);
 *     }
 *
 *     gate namespaced::operation(q: qubit) {
 *         apply H(q);
 *     }
 *
 *     gate generic(data: Tensor<Scalar, shape>, q: qubit) {
 *         apply custom::operation(data, q);
 *     }
 *
 * Exact availability of example types is validated by the canonical type
 * system, not by a closed gate grammar.
 */


/*
 * ============================================================================
 * 44. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Tests must include:
 *
 *     zero formal parameters;
 *     one formal parameter;
 *     many formal parameters;
 *     many gate-body operations;
 *     nested gate references;
 *     deeply composed gate definitions;
 *     large qualified names;
 *     large symbolic expressions;
 *     arbitrary valid qubit counts;
 *     parameterized qubit types;
 *     user-defined operation names;
 *     namespaced operation names.
 *
 * Tests must not encode an architectural maximum such as:
 *
 *     8 parameters;
 *     16 operands;
 *     32 qubits;
 *     1024 gates.
 *
 * External parser/compiler resource-policy tests may test operational limits,
 * but those are not language-level grammar limits.
 */


/*
 * ============================================================================
 * 45. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The same grammar must accept structurally equivalent gate definitions
 * ranging from very small:
 *
 *     gate id(q: qubit) {
 *         apply I(q);
 *     }
 *
 * to arbitrarily large source representations, subject only to external
 * compiler/parser resource policies.
 *
 * "Infinity" here means:
 *
 *     no artificial language-level finite machine ceiling.
 *
 * Actual execution remains bounded by resources available to the compiler,
 * runtime, target and deployment environment.
 */


/*
 * ============================================================================
 * 46. DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Identical source and identical language-version configuration must produce
 * identical parse structure.
 *
 * Parsing must not depend on:
 *
 *     hardware;
 *     device availability;
 *     network state;
 *     filesystem state;
 *     wall-clock time;
 *     randomness;
 *     environment variables.
 */


/*
 * ============================================================================
 * 47. ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where formatter support exists:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     formatter
 *       |
 *       v
 *     parser
 *
 * must preserve gate semantics, including:
 *
 *     gate name;
 *     qualified name;
 *     formal ordering;
 *     formal types;
 *     defaults;
 *     attributes;
 *     body operation ordering.
 */


/*
 * ============================================================================
 * 48. COMPATIBILITY WITH OPERATIONS.G4
 * ============================================================================
 *
 * operations.g4 is the canonical owner of:
 *
 *     quantumOperationStatement
 *     quantumOperationApplication
 *     quantumOperationInvocation
 *     quantumOperationDesignator
 *     quantumOperationParameterClause
 *     quantumOperationTargetClause
 *     quantumOperationModifier
 *
 * gates.g4 MUST consume those rules.
 *
 * It MUST NOT redefine them.
 *
 * This removes the previous duplication where gates.g4 independently
 * implemented:
 *
 *     gateApplication
 *     quantumOperationApplication
 *     quantumOperationName
 *     gateParameterList
 *     gateOperandList
 *     gateModifierList
 *
 * while operations.g4 already owned the same conceptual operation language.
 */


/*
 * ============================================================================
 * 49. COMPATIBILITY WITH PARAMETERIZED-OPERATIONS.G4
 * ============================================================================
 *
 * Gate declaration formals are declaration syntax.
 *
 * Parameterized operation invocation remains owned by the parameterized
 * operation/operation grammars.
 *
 * No second parameter invocation grammar is introduced here.
 */


/*
 * ============================================================================
 * 50. COMPATIBILITY WITH CONTROLLED-OPERATIONS.G4
 * ============================================================================
 *
 * Controlled operation syntax remains downstream of the operation grammar.
 *
 * A gate declaration may contain a controlled operation invocation because the
 * body consumes quantumOperationStatement.
 *
 * gates.g4 does not define:
 *
 *     control(...)
 *
 * again.
 */


/*
 * ============================================================================
 * 51. COMPATIBILITY WITH QUBITS.G4
 * ============================================================================
 *
 * Formal quantum operands use ordinary type syntax.
 *
 * gates.g4 does not define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     qubitReference
 *     registerReference
 *
 * Those remain owned by the appropriate quantum declaration/reference
 * grammars.
 */


/*
 * ============================================================================
 * 52. COMPATIBILITY WITH QUANTUM.G4
 * ============================================================================
 *
 * quantum.g4 remains the quantum composition boundary.
 *
 * It should expose:
 *
 *     quantumGateDeclaration
 *
 * through its declaration-element composition.
 *
 * It should NOT duplicate the gate declaration itself.
 */


/*
 * ============================================================================
 * 53. COMPATIBILITY WITH ROOT ZAMANI.G4
 * ============================================================================
 *
 * The root grammar:
 *
 *     grammar/Zamani.g4
 *
 * remains the complete-program composition root.
 *
 * It should not enumerate gate names.
 *
 * It should not import this leaf grammar directly if the canonical parser
 * composition already imports the quantum composition layer.
 *
 * The dependency should remain:
 *
 *     Zamani
 *        |
 *        v
 *     ZamaniParser
 *        |
 *        v
 *     Quantum composition
 *        |
 *        v
 *     QuantumGates
 */


/*
 * ============================================================================
 * 54. COMPATIBILITY WITH GRAMMAR.MD
 * ============================================================================
 *
 * grammar/grammar.md must report gate definitions according to actual
 * implementation status.
 *
 * The presence of this grammar alone does NOT make gate declarations
 * implemented in the Rust frontend.
 *
 * Status progression:
 *
 *     SPECIFIED
 *       ->
 *     GRAMMAR_IMPLEMENTED
 *       ->
 *     LEXER_IMPLEMENTED
 *       ->
 *     PARSER_IMPLEMENTED
 *       ->
 *     AST_IMPLEMENTED
 *       ->
 *     SEMANTIC_IMPLEMENTED
 *       ->
 *     IR_IMPLEMENTED
 *       ->
 *     TESTED
 *       ->
 *     STABLE
 */


/*
 * ============================================================================
 * 55. COMPATIBILITY WITH ZAMANI-GRAMMAR.MD
 * ============================================================================
 *
 * Zamani-Grammar.md may describe:
 *
 *     historical gate syntax;
 *     proposed gate syntax;
 *     experimental gates;
 *     future operation systems.
 *
 * Such material does not automatically change this grammar.
 *
 * Promotion follows:
 *
 *     design
 *       ->
 *     specification
 *       ->
 *     AST contract
 *       ->
 *     grammar
 *       ->
 *     semantic implementation
 *       ->
 *     IR contract
 *       ->
 *     tests
 *       ->
 *     stable
 */


/*
 * ============================================================================
 * 56. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file must pass the following audit:
 *
 * [x] no fixed gate-name enumeration;
 * [x] no MAX_GATES;
 * [x] no MAX_QUBITS;
 * [x] no MAX_PARAMETERS;
 * [x] no MAX_OPERANDS;
 * [x] no MAX_CIRCUIT_DEPTH;
 * [x] no fixed physical qubit IDs;
 * [x] no fixed device IDs;
 * [x] no fixed topology;
 * [x] no fixed native gate set;
 * [x] no fixed pulse set;
 * [x] no calibration data;
 * [x] no routing decisions;
 * [x] no scheduling decisions;
 * [x] no QEC implementation;
 * [x] no ZQN implementation;
 * [x] no HAL implementation;
 * [x] no backend selection.
 */


/*
 * ============================================================================
 * 57. SAFETY AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no Rust actions;
 *     no embedded unsafe code;
 *     no filesystem access;
 *     no network access;
 *     no device access;
 *     no executable semantic actions.
 *
 * The consuming Rust implementation remains subject to:
 *
 *     Rust 1.97 / 1.97.1;
 *     Rust 2021;
 *     safe Rust only.
 */


/*
 * ============================================================================
 * 58. COMPLETION CRITERIA
 * ============================================================================
 *
 * gates.g4 is complete when:
 *
 * [x] Gate declarations have one canonical owner.
 *
 * [x] Gate names are open-ended.
 *
 * [x] Standard gates are not enumerated by the parser.
 *
 * [x] User-defined gates are supported.
 *
 * [x] Namespaced gates are supported.
 *
 * [x] Formal parameters are scalable.
 *
 * [x] Formal types delegate to the canonical type grammar.
 *
 * [x] Default expressions delegate to the canonical expression grammar.
 *
 * [x] Gate bodies consume the canonical operation grammar.
 *
 * [x] Operation invocation is not duplicated.
 *
 * [x] Measurement is not duplicated.
 *
 * [x] Reset is not duplicated.
 *
 * [x] Barrier is not duplicated.
 *
 * [x] Controlled-operation syntax is not duplicated.
 *
 * [x] Parameterized-operation invocation is not duplicated.
 *
 * [x] Qubit-reference syntax is not duplicated.
 *
 * [x] No hardware realization is encoded.
 *
 * [x] No resource ceiling is encoded.
 *
 * [x] No artificial machine-size limit is encoded.
 *
 * [x] No competing quantum IR is introduced.
 *
 * [x] AST integration is defined.
 *
 * [x] semantic integration is defined.
 *
 * [x] quantum::ir integration is defined.
 *
 * [x] compiler integration is defined.
 *
 * [x] runtime integration is defined.
 *
 * [x] positive tests are defined.
 *
 * [x] negative tests are defined.
 *
 * [x] boundary tests are defined.
 *
 * [x] scalability tests are defined.
 *
 * [x] determinism requirements are defined.
 *
 * [x] compatibility requirements are defined.
 *
 * [x] hard-coding audit is defined.
 *
 * [x] Rust 1.97 / 1.97.1 compatibility is documented.
 *
 * [x] no unsafe Rust is required.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * gates.g4 answers one question:
 *
 *     "How is a reusable logical quantum gate declared?"
 *
 * It does NOT answer:
 *
 *     "How is the gate invoked?"
 *     "Which qubit is physical?"
 *     "Which QPU executes it?"
 *     "How is it routed?"
 *     "How is it scheduled?"
 *     "How is it error corrected?"
 *     "What noise model applies?"
 *     "Which backend executes it?"
 *
 * Those responsibilities remain downstream.
 *
 * ============================================================================
 */