parser grammar AcceleratorInteroperability;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Zamani Hybrid — Accelerator Interoperability
 *
 * PURPOSE
 * -------
 * Defines syntax for expressing interoperability between a computation
 * and an accelerator domain without making the source program dependent
 * on a particular accelerator, device, vendor, topology, capacity, or
 * deployment.
 *
 * ARCHITECTURAL POSITION
 * ----------------------
 *
 *   Zamani source
 *        |
 *        v
 *   canonical lexer
 *        |
 *        v
 *   Zamani parser
 *        |
 *        +--> classical syntax
 *        +--> quantum syntax
 *        +--> hybrid syntax
 *        +--> accelerator interoperability  <--- THIS GRAMMAR
 *        |
 *        v
 *   AST / semantic analysis
 *        |
 *        +--> capability checking
 *        +--> resource checking
 *        +--> target selection
 *        +--> lowering
 *        |
 *        +--> classical IR
 *        +--> quantum::ir
 *        +--> hardware / accelerator lowering
 *        |
 *        v
 *   optimization / routing / scheduling / compilation / runtime
 *
 * THIS FILE DOES NOT:
 *   - define a lexer
 *   - define machine sizes
 *   - define accelerator IDs
 *   - define device addresses
 *   - define topology
 *   - define physical placement
 *   - define scheduling
 *   - define runtime dispatch
 *   - define quantum IR
 *   - define classical IR
 *   - define hardware capabilities
 *   - define resource limits
 *   - define vendor-specific device semantics
 *
 * The semantic layer interprets the syntax and resolves it against
 * capabilities/resources/targets available at compilation or execution.
 *
 * IMPORTANT
 * ---------
 * This grammar intentionally contains no target-specific lexer actions,
 * embedded Rust code, semantic predicates, unsafe code, fixed resource
 * limits, or generated-machine assumptions.
 */


/* -------------------------------------------------------------------------
 * Top-level accelerator interoperability declarations
 * ------------------------------------------------------------------------- */

accelerator_interoperability
    : accelerator_interoperability_item*
    ;

accelerator_interoperability_item
    : accelerator_interface_declaration
    | accelerator_binding_declaration
    | accelerator_operation_declaration
    | accelerator_data_exchange_declaration
    | accelerator_execution_declaration
    ;


/* -------------------------------------------------------------------------
 * Accelerator interfaces
 *
 * Describes a semantic interface through which computation may interact
 * with an accelerator. It does not identify a physical device.
 * ------------------------------------------------------------------------- */

accelerator_interface_declaration
    : ACCELERATOR INTERFACE identifier
      accelerator_interface_body
    ;

accelerator_interface_body
    : LBRACE
      accelerator_interface_member*
      RBRACE
    ;

accelerator_interface_member
    : accelerator_interface_input
    | accelerator_interface_output
    | accelerator_interface_operation
    | accelerator_interface_contract
    | accelerator_interface_attribute
    ;

accelerator_interface_input
    : INPUT parameter_declaration
    ;

accelerator_interface_output
    : OUTPUT parameter_declaration
    ;

accelerator_interface_operation
    : OPERATION identifier
      LPAREN parameter_list? RPAREN
      return_type?
      SEMICOLON
    ;

accelerator_interface_contract
    : REQUIRES expression SEMICOLON
    ;

accelerator_interface_attribute
    : attribute
    ;


/* -------------------------------------------------------------------------
 * Accelerator bindings
 *
 * A binding connects a program-level accelerator role/interface to an
 * implementation selected elsewhere.
 *
 * The binding is intentionally symbolic. A name is not interpreted here
 * as a physical device identifier.
 * ------------------------------------------------------------------------- */

accelerator_binding_declaration
    : BIND identifier
      TO accelerator_binding_target
      accelerator_binding_clause*
      SEMICOLON
    ;

accelerator_binding_target
    : identifier
    | qualified_name
    ;

accelerator_binding_clause
    : capability_clause
    | requirement_clause
    | constraint_clause
    | preference_clause
    | hint_clause
    ;


/* -------------------------------------------------------------------------
 * Accelerator operations
 *
 * Describes invocation of a semantic accelerator operation.
 *
 * Operation names and arguments remain program-level symbols. Mapping to
 * CPU instructions, GPU kernels, FPGA logic, tensor engines, DSPs, ASICs,
 * quantum accelerators, or future accelerator classes is downstream.
 * ------------------------------------------------------------------------- */

accelerator_operation_declaration
    : ACCELERATE identifier
      LPAREN argument_list? RPAREN
      accelerator_operation_clause*
      SEMICOLON
    ;

accelerator_operation_clause
    : USING identifier
    | capability_clause
    | requirement_clause
    | constraint_clause
    | preference_clause
    | hint_clause
    | accelerator_data_clause
    ;

accelerator_data_clause
    : INPUTS argument_list
    | OUTPUTS argument_list
    ;


/* -------------------------------------------------------------------------
 * Accelerator execution
 *
 * Represents semantic execution intent without prescribing whether the
 * implementation is synchronous, asynchronous, queued, remote, local,
 * distributed, or otherwise.
 * ------------------------------------------------------------------------- */

accelerator_execution_declaration
    : EXECUTE accelerator_execution_target
      accelerator_execution_clause*
      SEMICOLON
    ;

accelerator_execution_target
    : identifier
    | qualified_name
    | call_expression
    ;

accelerator_execution_clause
    : WITH argument_list
    | ON_COMPLETION statement
    | capability_clause
    | requirement_clause
    | constraint_clause
    | preference_clause
    | hint_clause
    ;

ON_COMPLETION
    : identifier
    ;


/* -------------------------------------------------------------------------
 * Data exchange
 *
 * Expresses movement or sharing of logical values between computation
 * domains. It does not prescribe physical buses, DMA engines, PCIe,
 * NVLink, memory fabrics, quantum/classical channels, or network topology.
 * ------------------------------------------------------------------------- */

accelerator_data_exchange_declaration
    : TRANSFER accelerator_transfer_source
      TO accelerator_transfer_destination
      accelerator_transfer_clause*
      SEMICOLON
    ;

accelerator_transfer_source
    : expression
    ;

accelerator_transfer_destination
    : expression
    ;

accelerator_transfer_clause
    : AS type_reference
    | USING identifier
    | capability_clause
    | requirement_clause
    | constraint_clause
    | preference_clause
    | hint_clause
    ;


/* -------------------------------------------------------------------------
 * Interface-level operation
 * ------------------------------------------------------------------------- */

accelerator_interface_operation
    : OPERATION identifier
      LPAREN parameter_list? RPAREN
      return_type?
      interface_operation_clause*
      SEMICOLON
    ;

interface_operation_clause
    : capability_clause
    | requirement_clause
    | constraint_clause
    | preference_clause
    | hint_clause
    ;


/* -------------------------------------------------------------------------
 * Capability / requirement / constraint / preference / hint delegation
 *
 * These rules intentionally reference the shared semantic vocabulary
 * rather than inventing accelerator-specific copies.
 *
 * The exact interpretation belongs to semantic analysis and the existing
 * core/resources/hardware/capability infrastructure.
 * ------------------------------------------------------------------------- */

capability_clause
    : REQUIRES_CAPABILITY expression
    ;

requirement_clause
    : REQUIRES expression
    ;

constraint_clause
    : CONSTRAINED_BY expression
    ;

preference_clause
    : PREFER expression
    ;

hint_clause
    : HINT expression
    ;


/* -------------------------------------------------------------------------
 * Shared declarations / expressions
 *
 * These names are imported from the repository's canonical parser
 * delegates rather than duplicated in this file.
 * ------------------------------------------------------------------------- */

parameter_declaration
    : identifier COLON type_reference
    ;

parameter_list
    : parameter_declaration
      (COMMA parameter_declaration)*
    ;

argument_list
    : expression
      (COMMA expression)*
    ;

return_type
    : ARROW type_reference
    ;

type_reference
    : qualified_name
    ;

call_expression
    : expression
    ;

identifier
    : IDENTIFIER
    ;

qualified_name
    : identifier
      (DOT identifier)*
    ;

attribute
    : AT identifier
      (LPAREN argument_list? RPAREN)?
    ;

statement
    : expression SEMICOLON
    ;

expression
    : identifier
    | qualified_name
    | literal
    | call_expression
    ;

literal
    : integer_literal
    | floating_literal
    | string_literal
    | boolean_literal
    ;

integer_literal
    : INTEGER_LITERAL
    ;

floating_literal
    : FLOAT_LITERAL
    ;

string_literal
    : STRING_LITERAL
    ;

boolean_literal
    : TRUE
    | FALSE
    ;