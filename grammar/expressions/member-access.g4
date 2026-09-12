parser grammar member_access;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Zamani member-access grammar
 *
 * Responsibility:
 *   Defines the syntactic suffix used to access a member of a value,
 *   type, namespace, module, resource, capability, hardware object,
 *   quantum object, HDL object, or other semantic entity.
 *
 * This grammar deliberately does NOT define:
 *   - the complete expression grammar
 *   - identifier semantics
 *   - type/member lookup
 *   - field existence
 *   - method existence
 *   - visibility/access control
 *   - inheritance/trait/interface resolution
 *   - hardware discovery
 *   - quantum-device discovery
 *   - resource discovery
 *   - IR lowering
 *   - runtime dispatch
 *
 * Those concerns belong to semantic analysis and later compiler/runtime
 * layers.
 *
 * Integration model:
 *
 *   primary/postfix expression
 *          |
 *          +--> member_access_suffix
 *          |
 *          +--> call suffix
 *          |
 *          +--> indexing suffix
 *          |
 *          +--> other postfix constructs
 *
 * The root expressions grammar owns the ordering/repetition of postfix
 * operations. This grammar owns only the member-access suffix itself.
 *
 * No machine size, hardware count, topology, address, device identifier,
 * register count, qubit count, or other implementation limit is encoded.
 */

/*
 * Member-access suffix
 *
 * Examples:
 *
 *   value.member
 *   object.field
 *   module.item
 *   quantum_register.qubit
 *   hardware_resource.capability
 *   tensor.shape
 *   namespace.symbol
 *
 * Chained access is intentionally NOT implemented recursively here.
 * The enclosing expressions grammar can repeat this suffix:
 *
 *   a.b.c.d
 *
 * without imposing any fixed depth.
 */
memberAccessSuffix
    : DOT memberName
    ;

/*
 * Member name
 *
 * A member is syntactically a name. Semantic analysis determines whether
 * that name denotes:
 *
 *   - a field
 *   - property
 *   - method
 *   - associated item
 *   - namespace member
 *   - module member
 *   - type member
 *   - capability
 *   - resource
 *   - hardware component
 *   - quantum component
 *   - HDL component
 *   - implementation-defined extension
 *
 * The identifier grammar remains authoritative for identifier syntax.
 */
memberName
    : IDENTIFIER
    ;