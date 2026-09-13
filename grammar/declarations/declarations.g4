/*
 * ZamaniDeclarations.g4
 *
 * Production declaration grammar for Zamani.
 *
 * OWNERSHIP
 * ---------
 * This grammar owns the syntactic structure of declarations.
 *
 * It does NOT own:
 *   - lexical/token definitions
 *   - semantic/type checking
 *   - symbol-table construction
 *   - canonical IR
 *   - quantum::ir
 *   - optimization
 *   - routing
 *   - scheduling
 *   - hardware discovery
 *   - resource discovery
 *   - runtime execution
 *   - backend selection
 *
 * ARCHITECTURE
 * ------------
 *
 * Source
 *   -> lexer
 *   -> parser
 *   -> declarations / expressions / statements / types
 *   -> AST
 *   -> semantic analysis
 *   -> canonical IR
 *   -> domain lowering
 *   -> optimization
 *   -> routing / scheduling
 *   -> target lowering
 *   -> runtime
 *
 * The declaration layer expresses PROGRAM SEMANTICS and INTENT.
 * Physical resource counts, topology, device identifiers, timing,
 * qubit counts, core counts, memory capacities, accelerator counts,
 * and other target-specific properties MUST NOT be encoded here.
 *
 * POCO-REAF
 * ---------
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A declaration can express:
 *
 *   - what an entity is
 *   - what it requires
 *   - what it provides
 *   - what it implements
 *   - what capabilities it may use
 *   - what effects it has
 *   - what constraints it declares
 *   - what generic relationships it has
 *
 * It must not prescribe an arbitrary machine configuration.
 *
 * RUST
 * ----
 *
 * This grammar generates parser infrastructure consumed by the
 * Rust implementation. Nothing here requires Rust `unsafe`.
 *
 * Rust compatibility target:
 *
 *   Rust 1.97 / 1.97.1
 *
 * ANTLR
 * -----
 *
 * This is a parser grammar intended to be imported by the future
 * authoritative Zamani parser grammar after lexical/parser separation.
 *
 * Expected integration:
 *
 *   ZamaniLexer.g4
 *          |
 *          v
 *   ZamaniParser.g4
 *          |
 *          +--> ZamaniDeclarations.g4
 *          +--> ZamaniExpressions.g4
 *          +--> ZamaniStatements.g4
 *          +--> ZamaniTypes.g4
 *          +--> ZamaniFunctions.g4
 *          +--> ...
 *
 * Do not create another independent `grammar Zamani;` here.
 */

parser grammar ZamaniDeclarations;

options {
    /*
     * The lexer grammar is the single lexical authority.
     *
     * The repository currently has a monolithic combined Zamani.g4.
     * The intended production integration is to extract its lexer
     * vocabulary into ZamaniLexer.g4 and make the parser modules
     * consume that vocabulary.
     *
     * This avoids every declaration file creating its own token set.
     */
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * DECLARATION ROOT
 * ============================================================================
 *
 * `declaration` is deliberately semantic-domain neutral.
 *
 * Domain-specific declarations remain separate rules and can be extended
 * without changing the universal declaration contract.
 *
 * No declaration rule contains fixed machine sizes.
 */
declaration
    : attributedDeclaration
    | visibilityDeclaration
    | moduleDeclaration
    | importDeclaration
    | exportDeclaration
    | packageDeclaration
    | namespaceDeclaration
    | usingDeclaration
    | functionDeclaration
    | asyncFunctionDeclaration
    | constantDeclaration
    | variableDeclaration
    | typeDeclaration
    | typeAliasDeclaration
    | structDeclaration
    | enumDeclaration
    | unionDeclaration
    | recordDeclaration
    | classDeclaration
    | interfaceDeclaration
    | traitDeclaration
    | implementationDeclaration
    | effectDeclaration
    | capabilityDeclaration
    | resourceDeclaration
    | requirementDeclaration
    | constraintDeclaration
    | contractDeclaration
    | quantumDeclaration
    | hardwareDeclaration
    | hdlDeclaration
    | acceleratorDeclaration
    | distributedDeclaration
    | dataDeclaration
    | macroDeclaration
    | languageDeclaration
    | dialectDeclaration
    | pluginDeclaration
    ;

/*
 * ============================================================================
 * ATTRIBUTES
 * ============================================================================
 *
 * Attributes decorate declarations but do not change ownership of the
 * declaration itself.
 *
 * Semantic interpretation belongs to semantic analysis.
 */
attributedDeclaration
    : attribute+ declaration
    ;

attribute
    : '@' qualifiedName
    | '@' qualifiedName '(' attributeArguments? ')'
    ;

attributeArguments
    : attributeArgument (',' attributeArgument)*
    ;

attributeArgument
    : attributeName '=' attributeValue
    | attributeValue
    ;

attributeName
    : identifier
    ;

attributeValue
    : literal
    | qualifiedName
    | attribute
    | arrayAttributeValue
    | mapAttributeValue
    ;

arrayAttributeValue
    : '[' attributeValueList? ']'
    ;

attributeValueList
    : attributeValue (',' attributeValue)*
    ;

mapAttributeValue
    : '{' mapAttributeEntryList? '}'
    ;

mapAttributeEntryList
    : mapAttributeEntry (',' mapAttributeEntry)*
    ;

mapAttributeEntry
    : attributeName ':' attributeValue
    ;

/*
 * ============================================================================
 * VISIBILITY
 * ============================================================================
 *
 * Visibility is deliberately separate from modifiers.
 *
 * `unsafe` is NOT a declaration modifier.
 *
 * Unsafe semantics, if the language eventually supports them, belong to
 * the dedicated unsafe syntax/semantic subsystem and must never silently
 * turn an ordinary declaration into unsafe code.
 */
visibilityDeclaration
    : visibilityModifier declarationWithoutVisibility
    ;

visibilityModifier
    : PUBLIC
    | PRIVATE
    | PROTECTED
    | INTERNAL
    | PACKAGE
    ;

declarationWithoutVisibility
    : moduleDeclaration
    | importDeclaration
    | exportDeclaration
    | packageDeclaration
    | namespaceDeclaration
    | usingDeclaration
    | functionDeclaration
    | asyncFunctionDeclaration
    | constantDeclaration
    | variableDeclaration
    | typeDeclaration
    | typeAliasDeclaration
    | structDeclaration
    | enumDeclaration
    | unionDeclaration
    | recordDeclaration
    | classDeclaration
    | interfaceDeclaration
    | traitDeclaration
    | implementationDeclaration
    | effectDeclaration
    | capabilityDeclaration
    | resourceDeclaration
    | requirementDeclaration
    | constraintDeclaration
    | contractDeclaration
    | quantumDeclaration
    | hardwareDeclaration
    | hdlDeclaration
    | acceleratorDeclaration
    | distributedDeclaration
    | dataDeclaration
    | macroDeclaration
    | languageDeclaration
    | dialectDeclaration
    | pluginDeclaration
    ;

/*
 * ============================================================================
 * DECLARATION MODIFIERS
 * ============================================================================
 *
 * Modifiers are intentionally conservative.
 *
 * Domain-specific modifiers should not be added here simply because a
 * backend happens to understand them.
 *
 * Backend information belongs in capabilities, requirements, constraints,
 * target descriptions, or backend-specific dialects.
 */
declarationModifiers
    : declarationModifier*
    ;

declarationModifier
    : STATIC
    | FINAL
    | ABSTRACT
    | SEALED
    | PARTIAL
    | EXTERN
    | OVERRIDE
    | INLINE
    | PURE
    | ASYNC
    | CONST
    | MUT
    ;

/*
 * ============================================================================
 * MODULE / PACKAGE / NAMESPACE
 * ============================================================================
 */

moduleDeclaration
    : MODULE qualifiedName moduleBody?
    ;

moduleBody
    : '{' declaration* '}'
    ;

packageDeclaration
    : PACKAGE qualifiedName packageBody?
    ;

packageBody
    : '{' packageMember* '}'
    ;

packageMember
    : packageField
    | declaration
    ;

packageField
    : VERSION ':' literal ';'
    | REPOSITORY ':' literal ';'
    | LICENSE ':' literal ';'
    | DEPENDS ':' '[' literalList? ']' ';'
    ;

namespaceDeclaration
    : NAMESPACE qualifiedName '{' declaration* '}'
    ;

usingDeclaration
    : USING qualifiedName usingAlias? ';'
    ;

usingAlias
    : AS identifier
    ;

/*
 * ============================================================================
 * IMPORT / EXPORT
 * ============================================================================
 */

importDeclaration
    : IMPORT importTarget importAlias? ';'
    | IMPORT '{' importSpecifierList '}' FROM importSource ';'
    | IMPORT STAR AS identifier FROM importSource ';'
    ;

importTarget
    : qualifiedName
    ;

importAlias
    : AS identifier
    ;

importSpecifierList
    : importSpecifier (',' importSpecifier)*
    ;

importSpecifier
    : identifier importRename?
    ;

importRename
    : AS identifier
    ;

importSource
    : stringLiteral
    | qualifiedName
    ;

exportDeclaration
    : EXPORT exportTarget ';'
    | EXPORT '{' exportSpecifierList '}' ';'
    | EXPORT STAR FROM importSource ';'
    ;

exportTarget
    : qualifiedName
    ;

exportSpecifierList
    : exportSpecifier (',' exportSpecifier)*
    ;

exportSpecifier
    : identifier exportRename?
    ;

exportRename
    : AS identifier
    ;

/*
 * ============================================================================
 * CONSTANTS / VARIABLES
 * ============================================================================
 *
 * Declaration syntax does not impose a maximum number of declarations,
 * dimensions, elements, resources, or instances.
 *
 * Such limits belong to semantic/resource validation.
 */

constantDeclaration
    : declarationModifiers CONST identifier ':' typeReference '=' expression ';'
    ;

variableDeclaration
    : declarationModifiers LET identifier variableTypeAnnotation?
      variableInitializer? ';'
    ;

variableTypeAnnotation
    : ':' typeReference
    ;

variableInitializer
    : '=' expression
    ;

/*
 * ============================================================================
 * TYPE DECLARATIONS
 * ============================================================================
 */

typeDeclaration
    : declarationModifiers TYPE identifier genericParameterList?
      typeDeclarationBody
    ;

typeDeclarationBody
    : typeConstraintClause? ';'
    | '=' typeReference ';'
    ;

typeAliasDeclaration
    : declarationModifiers TYPE identifier genericParameterList?
      '=' typeReference ';'
    ;

/*
 * ============================================================================
 * STRUCTURES
 * ============================================================================
 */

structDeclaration
    : declarationModifiers STRUCT identifier genericParameterList?
      typeInheritanceClause?
      structBody
    ;

structBody
    : '{' structMember* '}'
    ;

structMember
    : attributedDeclaration
    | visibilityModifier? declarationModifiers? fieldDeclaration
    | visibilityModifier? declarationModifiers? functionDeclaration
    | visibilityModifier? declarationModifiers? constantDeclaration
    ;

fieldDeclaration
    : identifier ':' typeReference fieldInitializer? ';'
    ;

fieldInitializer
    : '=' expression
    ;

/*
 * ============================================================================
 * ENUMERATIONS
 * ============================================================================
 *
 * Enum cardinality is not bounded by grammar.
 *
 * Discriminants are semantic values, not physical resource limits.
 */

enumDeclaration
    : declarationModifiers ENUM identifier genericParameterList?
      enumUnderlyingType?
      enumBody
    ;

enumUnderlyingType
    : ':' typeReference
    ;

enumBody
    : '{' enumMemberList? enumTrailingComma? '}'
    ;

enumMemberList
    : enumMember (',' enumMember)*
    ;

enumTrailingComma
    : ','
    ;

enumMember
    : attributedEnumMember
    ;

attributedEnumMember
    : attribute* identifier enumMemberPayload?
    ;

enumMemberPayload
    : '(' parameterList? ')'
    | '{' enumMemberFieldList? '}'
    ;

enumMemberFieldList
    : fieldDeclaration (',' fieldDeclaration)*
    ;

/*
 * ============================================================================
 * UNIONS
 * ============================================================================
 */

unionDeclaration
    : declarationModifiers UNION identifier genericParameterList?
      '=' unionVariantList ';'
    ;

unionVariantList
    : unionVariant ('|' unionVariant)*
    ;

unionVariant
    : attribute* identifier unionVariantPayload?
    ;

unionVariantPayload
    : '(' parameterList? ')'
    | '{' structMember* '}'
    ;

/*
 * ============================================================================
 * RECORDS
 * ============================================================================
 */

recordDeclaration
    : declarationModifiers RECORD identifier genericParameterList?
      typeInheritanceClause?
      recordBody
    ;

recordBody
    : '{' recordMember* '}'
    ;

recordMember
    : attributedDeclaration
    | visibilityModifier? declarationModifiers? fieldDeclaration
    | visibilityModifier? declarationModifiers? functionDeclaration
    | visibilityModifier? declarationModifiers? constantDeclaration
    ;

/*
 * ============================================================================
 * CLASSES
 * ============================================================================
 */

classDeclaration
    : declarationModifiers CLASS identifier genericParameterList?
      typeInheritanceClause?
      classBody
    ;

classBody
    : '{' classMember* '}'
    ;

classMember
    : attributedDeclaration
    | visibilityModifier? declarationModifiers? fieldDeclaration
    | visibilityModifier? declarationModifiers? constantDeclaration
    | visibilityModifier? declarationModifiers? functionDeclaration
    | visibilityModifier? declarationModifiers? constructorDeclaration
    | visibilityModifier? declarationModifiers? destructorDeclaration
    | visibilityModifier? declarationModifiers? propertyDeclaration
    | visibilityModifier? declarationModifiers? operatorDeclaration
    ;

constructorDeclaration
    : identifier '(' parameterList? ')' block
    ;

destructorDeclaration
    : TILDE identifier '(' ')' block
    ;

propertyDeclaration
    : identifier ':' typeReference
      '{'
      propertyAccessor*
      '}'
    ;

propertyAccessor
    : GET block
    | SET parameterList? block
    ;

operatorDeclaration
    : OPERATOR operatorSymbol '(' parameterList? ')' functionReturnClause? block
    ;

/*
 * ============================================================================
 * INTERFACES
 * ============================================================================
 */

interfaceDeclaration
    : declarationModifiers INTERFACE identifier genericParameterList?
      interfaceInheritanceClause?
      interfaceBody
    ;

interfaceBody
    : '{' interfaceMember* '}'
    ;

interfaceMember
    : attributedInterfaceMember
    ;

attributedInterfaceMember
    : attribute*
      interfaceMemberCore
    ;

interfaceMemberCore
    : interfaceMethodDeclaration
    | interfacePropertyDeclaration
    | interfaceAssociatedType
    | interfaceConstantDeclaration
    ;

interfaceMethodDeclaration
    : declarationModifiers? FN identifier genericParameterList?
      '(' parameterList? ')'
      functionReturnClause?
      effectClause?
      contractClause?
      interfaceMethodBody?
    ;

interfaceMethodBody
    : block
    ;

interfacePropertyDeclaration
    : declarationModifiers? identifier ':' typeReference
    ;

interfaceAssociatedType
    : TYPE identifier typeConstraintClause? ';'
    ;

interfaceConstantDeclaration
    : CONST identifier ':' typeReference ';'
    ;

/*
 * ============================================================================
 * TRAITS
 * ============================================================================
 */

traitDeclaration
    : declarationModifiers TRAIT identifier genericParameterList?
      traitInheritanceClause?
      traitBody
    ;

traitBody
    : '{' traitMember* '}'
    ;

traitMember
    : attributedDeclaration
    | traitMethodDeclaration
    | traitAssociatedType
    | traitConstantDeclaration
    ;

traitMethodDeclaration
    : declarationModifiers? FN identifier genericParameterList?
      '(' parameterList? ')'
      functionReturnClause?
      effectClause?
      contractClause?
      traitMethodBody?
    ;

traitMethodBody
    : block
    ;

traitAssociatedType
    : TYPE identifier typeConstraintClause? ';'
    ;

traitConstantDeclaration
    : CONST identifier ':' typeReference '=' expression ';'
    ;

/*
 * ============================================================================
 * IMPLEMENTATIONS
 * ============================================================================
 *
 * Implementations are semantic relationships.
 *
 * They do not identify a concrete CPU/GPU/QPU/device.
 */
implementationDeclaration
    : declarationModifiers IMPL implementationGenerics?
      implementationTarget implementationForClause?
      implementationWhereClause?
      implementationBody
    ;

implementationGenerics
    : genericParameterList
    ;

implementationTarget
    : qualifiedName
    | typeReference
    ;

implementationForClause
    : FOR typeReference
    ;

implementationWhereClause
    : WHERE typeConstraintList
    ;

implementationBody
    : '{' implementationMember* '}'
    ;

implementationMember
    : attributedDeclaration
    | visibilityModifier? declarationModifiers? functionDeclaration
    | visibilityModifier? declarationModifiers? constantDeclaration
    | visibilityModifier? declarationModifiers? typeDeclaration
    ;

/*
 * ============================================================================
 * GENERICS
 * ============================================================================
 *
 * No fixed number of generic parameters.
 *
 * The compiler/resource system determines practical limits.
 */

genericParameterList
    : '<' genericParameter (',' genericParameter)* '>'
    ;

genericParameter
    : attribute*
      genericParameterKind
      identifier
      genericParameterBound?
      genericParameterDefault?
    ;

genericParameterKind
    : TYPE_PARAMETER
    | CONST_PARAMETER
    | RESOURCE_PARAMETER
    | CAPABILITY_PARAMETER
    ;

genericParameterBound
    : ':' typeConstraint
    ;

genericParameterDefault
    : '=' genericParameterDefaultValue
    ;

genericParameterDefaultValue
    : typeReference
    | expression
    ;

typeConstraint
    : typeReference ('+' typeReference)*
    ;

typeConstraintList
    : typeConstraint (',' typeConstraint)*
    ;

/*
 * ============================================================================
 * INHERITANCE / RELATIONSHIPS
 * ============================================================================
 */

typeInheritanceClause
    : EXTENDS typeReferenceList
    ;

interfaceInheritanceClause
    : EXTENDS typeReferenceList
    ;

traitInheritanceClause
    : EXTENDS typeReferenceList
    ;

typeInheritanceAndImplementationClause
    : EXTENDS typeReferenceList
      IMPLEMENTS typeReferenceList
    ;

implementsClause
    : IMPLEMENTS typeReferenceList
    ;

typeReferenceList
    : typeReference (',' typeReference)*
    ;

/*
 * ============================================================================
 * FUNCTIONS
 * ============================================================================
 *
 * Function declarations intentionally remain in the declaration layer only
 * as a contract. The detailed function grammar may be delegated to
 * grammar/functions/functions.g4.
 *
 * This rule is kept here so declaration membership is explicit.
 */

functionDeclaration
    : declarationModifiers? FN identifier genericParameterList?
      '(' parameterList? ')'
      functionReturnClause?
      effectClause?
      contractClause?
      block
    ;

asyncFunctionDeclaration
    : declarationModifiers? ASYNC FN identifier genericParameterList?
      '(' parameterList? ')'
      functionReturnClause?
      effectClause?
      contractClause?
      block
    ;

functionReturnClause
    : ARROW typeReference
    ;

/*
 * ============================================================================
 * PARAMETERS
 * ============================================================================
 */

parameterList
    : parameter (',' parameter)*
    ;

parameter
    : attribute*
      parameterMode?
      identifier
      ':' typeReference
      parameterDefault?
    ;

parameterMode
    : REF
    | MUT
    | OUT
    | IN
    | MOVE
    ;

parameterDefault
    : '=' expression
    ;

/*
 * ============================================================================
 * EFFECTS
 * ============================================================================
 *
 * Effects describe semantic behavior.
 *
 * They do not select a machine.
 */

effectDeclaration
    : declarationModifiers? EFFECT identifier genericParameterList?
      effectParameterClause?
      effectReturnClause?
      effectBody
    ;

effectParameterClause
    : '(' parameterList? ')'
    ;

effectReturnClause
    : ARROW typeReference
    ;

effectBody
    : ';'
    | '{' effectMember* '}'
    ;

effectMember
    : effectOperationDeclaration
    | effectAssociatedType
    ;

effectOperationDeclaration
    : FN identifier '(' parameterList? ')'
      functionReturnClause?
      ';'
    ;

effectAssociatedType
    : TYPE identifier typeConstraintClause? ';'
    ;

effectClause
    : WITH EFFECTS '{' effectNameList? '}'
    ;

effectNameList
    : effectName (',' effectName)*
    ;

effectName
    : qualifiedName
    ;

/*
 * ============================================================================
 * CAPABILITIES
 * ============================================================================
 *
 * A capability is a semantic requirement/permission.
 *
 * It is deliberately not a hardware identifier.
 *
 * Examples:
 *
 *   capability quantum;
 *   capability floating_point;
 *   capability distributed;
 *   capability measurement;
 *
 * The meaning is resolved by semantic analysis and capability negotiation.
 */

capabilityDeclaration
    : declarationModifiers? CAPABILITY identifier
      capabilityParameterClause?
      capabilityBody
    ;

capabilityParameterClause
    : '(' argumentList? ')'
    ;

capabilityBody
    : ';'
    | '{' capabilityMember* '}'
    ;

capabilityMember
    : capabilityRequirement
    | capabilityConstraint
    | capabilityMetadata
    ;

capabilityRequirement
    : REQUIRES expression ';'
    ;

capabilityConstraint
    : CONSTRAINT expression ';'
    ;

capabilityMetadata
    : identifier ':' expression ';'
    ;

/*
 * ============================================================================
 * RESOURCES
 * ============================================================================
 *
 * Resource declarations describe abstract resources.
 *
 * They do NOT encode:
 *
 *   max_qubits = 64
 *   cores = 16
 *   gpu_count = 4
 *
 * A resource quantity is an expression and therefore may be:
 *
 *   - symbolic
 *   - generic
 *   - compile-time known
 *   - runtime supplied
 *   - target negotiated
 *
 * This is essential for POCO-REAF.
 */

resourceDeclaration
    : declarationModifiers? RESOURCE identifier
      genericParameterList?
      resourceBody
    ;

resourceBody
    : ';'
    | '{' resourceMember* '}'
    ;

resourceMember
    : resourceRequirement
    | resourceConstraint
    | resourceProperty
    | resourceCapability
    ;

resourceRequirement
    : REQUIRES expression ';'
    ;

resourceConstraint
    : CONSTRAINT expression ';'
    ;

resourceProperty
    : identifier ':' typeReference resourceInitializer? ';'
    ;

resourceInitializer
    : '=' expression
    ;

resourceCapability
    : CAPABILITY qualifiedName ';'
    ;

/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Requirement != resource != constraint != preference.
 *
 * Keeping these syntactically separate prevents accidental semantic collapse.
 */

requirementDeclaration
    : declarationModifiers? REQUIRES identifier
      requirementBody
    ;

requirementBody
    : ';'
    | '{' requirementMember* '}'
    ;

requirementMember
    : requirementExpression
    | requirementCapability
    | requirementResource
    | requirementProperty
    ;

requirementExpression
    : REQUIREMENT expression ';'
    ;

requirementCapability
    : CAPABILITY qualifiedName ';'
    ;

requirementResource
    : RESOURCE qualifiedName ';'
    ;

requirementProperty
    : identifier ':' expression ';'
    ;

/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict valid implementations.
 *
 * They do not identify a particular machine unless the program explicitly
 * expresses such a semantic requirement.
 */

constraintDeclaration
    : declarationModifiers? CONSTRAINT identifier
      constraintBody
    ;

constraintBody
    : ';'
    | '=' expression ';'
    | '{' constraintMember* '}'
    ;

constraintMember
    : REQUIREMENT expression ';'
    | CONSTRAINT expression ';'
    | ASSERT expression ';'
    ;

/*
 * ============================================================================
 * CONTRACTS
 * ============================================================================
 */

contractDeclaration
    : declarationModifiers? CONTRACT identifier
      contractBody
    ;

contractBody
    : ';'
    | '{' contractClauseMember* '}'
    ;

contractClauseMember
    : REQUIRES expression ';'
    | ENSURES expression ';'
    | INVARIANT expression ';'
    | ASSERT expression ';'
    ;

/*
 * ============================================================================
 * QUANTUM DECLARATIONS
 * ============================================================================
 *
 * The declaration layer allows quantum semantics without encoding a fixed
 * quantum processor.
 *
 * Important:
 *
 *   quantum declaration != quantum hardware
 *
 * Physical qubit mapping belongs to routing/hardware layers.
 * Scheduling belongs to scheduling.
 * Error correction belongs to QEC.
 * Noise/fault semantics belong to ZQN.
 * Canonical quantum semantics belong to quantum::ir.
 */

quantumDeclaration
    : quantumCircuitDeclaration
    | quantumOperationDeclaration
    | quantumResourceDeclaration
    | quantumCapabilityDeclaration
    ;

quantumCircuitDeclaration
    : declarationModifiers? QUANTUM CIRCUIT identifier
      genericParameterList?
      '(' parameterList? ')'
      functionReturnClause?
      effectClause?
      contractClause?
      block
    ;

quantumOperationDeclaration
    : declarationModifiers? QUANTUM OPERATION identifier
      genericParameterList?
      '(' parameterList? ')'
      functionReturnClause?
      quantumOperationBody
    ;

quantumOperationBody
    : ';'
    | block
    ;

quantumResourceDeclaration
    : declarationModifiers? QUANTUM RESOURCE identifier
      resourceBody
    ;

quantumCapabilityDeclaration
    : declarationModifiers? QUANTUM CAPABILITY identifier
      capabilityBody
    ;

/*
 * ============================================================================
 * HARDWARE DECLARATIONS
 * ============================================================================
 *
 * Hardware declarations describe hardware semantics and interfaces.
 *
 * They do not require a fixed implementation size.
 */

hardwareDeclaration
    : hardwareModuleDeclaration
    | hardwareInterfaceDeclaration
    | hardwareCapabilityDeclaration
    | hardwareResourceDeclaration
    ;

hardwareModuleDeclaration
    : declarationModifiers? HARDWARE MODULE identifier
      genericParameterList?
      hardwareBody
    ;

hardwareInterfaceDeclaration
    : declarationModifiers? HARDWARE INTERFACE identifier
      genericParameterList?
      hardwareInterfaceBody
    ;

hardwareCapabilityDeclaration
    : declarationModifiers? HARDWARE CAPABILITY identifier
      capabilityBody
    ;

hardwareResourceDeclaration
    : declarationModifiers? HARDWARE RESOURCE identifier
      resourceBody
    ;

hardwareBody
    : '{' declaration* '}'
    ;

hardwareInterfaceBody
    : '{' hardwareInterfaceMember* '}'
    ;

hardwareInterfaceMember
    : portDeclaration
    | signalDeclaration
    | clockDeclaration
    | hardwareRequirement
    ;

hardwareRequirement
    : REQUIRES expression ';'
    ;

/*
 * ============================================================================
 * HDL DECLARATIONS
 * ============================================================================
 */

hdlDeclaration
    : hdlModuleDeclaration
    | hdlInterfaceDeclaration
    | hdlTypeDeclaration
    ;

hdlModuleDeclaration
    : declarationModifiers? HDL MODULE identifier
      genericParameterList?
      hdlModuleBody
    ;

hdlModuleBody
    : '{' hdlMember* '}'
    ;

hdlMember
    : portDeclaration
    | signalDeclaration
    | clockDeclaration
    | registerDeclaration
    | processDeclaration
    | stateDeclaration
    | hardwareParameterDeclaration
    | functionDeclaration
    | declaration
    ;

hdlInterfaceDeclaration
    : declarationModifiers? HDL INTERFACE identifier
      genericParameterList?
      hdlInterfaceBody
    ;

hdlInterfaceBody
    : '{' hdlInterfaceMember* '}'
    ;

hdlInterfaceMember
    : portDeclaration
    | signalDeclaration
    | clockDeclaration
    ;

hdlTypeDeclaration
    : declarationModifiers? HDL TYPE identifier
      genericParameterList?
      '=' typeReference ';'
    ;

portDeclaration
    : PORT identifier ':' typeReference portDirection? ';'
    ;

portDirection
    : IN
    | OUT
    | INOUT
    ;

signalDeclaration
    : SIGNAL identifier ':' typeReference ';'
    ;

clockDeclaration
    : CLOCK identifier clockSpecification? ';'
    ;

clockSpecification
    : '(' argumentList? ')'
    ;

registerDeclaration
    : REGISTER identifier ':' typeReference
      registerInitializer?
      ';'
    ;

registerInitializer
    : '=' expression
    ;

processDeclaration
    : PROCESS identifier? processSensitivity? block
    ;

processSensitivity
    : '(' sensitivityList? ')'
    ;

sensitivityList
    : expression (',' expression)*
    ;

stateDeclaration
    : STATE identifier statePayload?
    ;

statePayload
    : ':' typeReference
    | '{' declaration* '}'
    ;

hardwareParameterDeclaration
    : PARAMETER identifier ':' typeReference
      ('=' expression)?
      ';'
    ;

/*
 * ============================================================================
 * ACCELERATOR DECLARATIONS
 * ============================================================================
 */

acceleratorDeclaration
    : declarationModifiers? ACCELERATOR identifier
      genericParameterList?
      acceleratorBody
    ;

acceleratorBody
    : ';'
    | '{' acceleratorMember* '}'
    ;

acceleratorMember
    : capabilityDeclaration
    | requirementDeclaration
    | resourceDeclaration
    | functionDeclaration
    | acceleratorInterface
    ;

acceleratorInterface
    : INTERFACE identifier '{' declaration* '}'
    ;

/*
 * ============================================================================
 * DISTRIBUTED DECLARATIONS
 * ============================================================================
 */

distributedDeclaration
    : distributedServiceDeclaration
    | distributedNodeDeclaration
    | distributedResourceDeclaration
    ;

distributedServiceDeclaration
    : declarationModifiers? DISTRIBUTED SERVICE identifier
      genericParameterList?
      distributedBody
    ;

distributedNodeDeclaration
    : declarationModifiers? DISTRIBUTED NODE identifier
      genericParameterList?
      distributedBody
    ;

distributedResourceDeclaration
    : declarationModifiers? DISTRIBUTED RESOURCE identifier
      resourceBody
    ;

distributedBody
    : '{' declaration* '}'
    ;

/*
 * ============================================================================
 * DATA DECLARATIONS
 * ============================================================================
 */

dataDeclaration
    : dataSchemaDeclaration
    | dataStreamDeclaration
    | dataRecordDeclaration
    ;

dataSchemaDeclaration
    : declarationModifiers? DATA SCHEMA identifier
      genericParameterList?
      dataSchemaBody
    ;

dataSchemaBody
    : '{' fieldDeclaration* '}'
    ;

dataStreamDeclaration
    : declarationModifiers? DATA STREAM identifier
      genericParameterList?
      ':' typeReference
      ';'
    ;

dataRecordDeclaration
    : declarationModifiers? DATA RECORD identifier
      genericParameterList?
      recordBody
    ;

/*
 * ============================================================================
 * MACROS
 * ============================================================================
 *
 * Macro syntax is declaration syntax only.
 * Macro expansion semantics belong to metaprogramming.
 */

macroDeclaration
    : declarationModifiers? MACRO identifier
      genericParameterList?
      macroParameterClause?
      macroBody
    ;

macroParameterClause
    : '(' macroParameterList? ')'
    ;

macroParameterList
    : macroParameter (',' macroParameter)*
    ;

macroParameter
    : identifier ':' macroParameterKind
    ;

macroParameterKind
    : IDENTIFIER
    ;

macroBody
    : block
    ;

/*
 * ============================================================================
 * LANGUAGE DECLARATIONS
 * ============================================================================
 *
 * Language/dialect extension is declarative metadata.
 *
 * It must not dynamically mutate the base grammar during ordinary parsing.
 * Actual grammar extension is validated and registered by the compiler.
 */

languageDeclaration
    : declarationModifiers? LANGUAGE identifier
      languageVersion?
      languageBody
    ;

languageVersion
    : VERSION literal
    ;

languageBody
    : ';'
    | '{' languageMember* '}'
    ;

languageMember
    : EXTENDS typeReferenceList ';'
    | CAPABILITY qualifiedName ';'
    | DIALECT qualifiedName ';'
    | identifier ':' expression ';'
    ;

/*
 * ============================================================================
 * DIALECT DECLARATIONS
 * ============================================================================
 */

dialectDeclaration
    : declarationModifiers? DIALECT identifier
      dialectVersion?
      dialectBody
    ;

dialectVersion
    : VERSION literal
    ;

dialectBody
    : ';'
    | '{' dialectMember* '}'
    ;

dialectMember
    : EXTENDS typeReferenceList ';'
    | CAPABILITY qualifiedName ';'
    | RESERVES identifierList ';'
    | identifier ':' expression ';'
    ;

/*
 * ============================================================================
 * PLUGIN DECLARATIONS
 * ============================================================================
 *
 * Plugin declarations describe integration contracts.
 *
 * Actual loading, verification, sandboxing, and execution belong to the
 * compiler/plugin subsystem.
 */

pluginDeclaration
    : declarationModifiers? PLUGIN identifier
      pluginBody
    ;

pluginBody
    : ';'
    | '{' pluginMember* '}'
    ;

pluginMember
    : CAPABILITY qualifiedName ';'
    | LANGUAGE identifier ';'
    | DIALECT identifier ';'
    | TRANSPILER identifier ';'
    | ENTRY_POINT stringLiteral ';'
    | identifier ':' expression ';'
    ;

/*
 * ============================================================================
 * CONTRACT HELPERS
 * ============================================================================
 */

contractClause
    : CONTRACT identifier? contractClauseBody?
    ;

contractClauseBody
    : '{' contractClauseMember* '}'
    ;

/*
 * ============================================================================
 * NAMES
 * ============================================================================
 *
 * Names remain unconstrained by machine characteristics.
 */

qualifiedName
    : identifier
    | qualifiedName DOUBLE_COLON identifier
    ;

identifier
    : IDENTIFIER
    ;

identifierList
    : identifier (',' identifier)*
    ;

/*
 * ============================================================================
 * TYPE REFERENCES
 * ============================================================================
 *
 * Detailed type construction belongs to the type grammar.
 *
 * This rule intentionally acts as an integration boundary.
 */

typeReference
    : identifier
    | qualifiedName
    | genericTypeReference
    | functionTypeReference
    | arrayTypeReference
    | tupleTypeReference
    ;

genericTypeReference
    : qualifiedName genericArgumentList
    ;

genericArgumentList
    : '<' genericArgument (',' genericArgument)* '>'
    ;

genericArgument
    : typeReference
    | expression
    ;

functionTypeReference
    : '(' typeReferenceList? ')' ARROW typeReference
    ;

arrayTypeReference
    : typeReference '[' expression? ']'
    ;

tupleTypeReference
    : '(' typeReference ',' typeReferenceList? ')'
    ;

/*
 * ============================================================================
 * LITERALS / EXPRESSIONS
 * ============================================================================
 *
 * These are intentionally integration boundaries.
 *
 * The authoritative expression/type grammar must be imported by the root
 * parser rather than duplicated here.
 */

literal
    : integerLiteral
    | decimalLiteral
    | stringLiteral
    | characterLiteral
    | booleanLiteral
    | NULL
    ;

integerLiteral
    : INTEGER
    ;

decimalLiteral
    : DECIMAL
    ;

stringLiteral
    : STRING
    ;

characterLiteral
    : CHAR
    ;

booleanLiteral
    : TRUE
    | FALSE
    ;

argumentList
    : expressionArgument (',' expressionArgument)*
    ;

expressionArgument
    : expression
    | identifier ':' expression
    ;

expression
    : primaryExpression
    ;

/*
 * `primaryExpression` is an integration placeholder at this grammar layer.
 *
 * The production parser should replace/delegate this rule to the
 * authoritative expressions grammar.
 */
primaryExpression
    : identifier
    | literal
    ;

/*
 * ============================================================================
 * OPERATOR SYMBOLS
 * ============================================================================
 *
 * Operators are syntactic symbols only.
 * Their meaning belongs to semantic/operator resolution.
 */

operatorSymbol
    : PLUS
    | MINUS
    | STAR
    | SLASH
    | PERCENT
    | EQUAL_EQUAL
    | NOT_EQUAL
    | LESS
    | GREATER
    | LESS_EQUAL
    | GREATER_EQUAL
    | AMPERSAND
    | PIPE
    | CARET
    | SHIFT_LEFT
    | SHIFT_RIGHT
    | MATMUL
    ;

/*
 * ============================================================================
 * COMPLETION / INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * 1. It contains no machine-size constants.
 * 2. It contains no physical device identifiers.
 * 3. It contains no qubit-count limits.
 * 4. It contains no CPU/GPU/FPGA/QPU-count limits.
 * 5. It does not define a second AST.
 * 6. It does not define canonical quantum IR.
 * 7. It does not perform semantic validation.
 * 8. It does not select a backend.
 * 9. It does not perform routing.
 * 10. It does not perform scheduling.
 * 11. It does not perform optimization.
 * 12. It does not perform hardware discovery.
 * 13. It does not execute code.
 * 14. It is deterministic for identical token streams.
 * 15. All declaration forms have explicit ownership.
 * 16. Domain-specific declarations remain extensible.
 * 17. Quantum declarations remain backend-independent.
 * 18. Hardware declarations remain resource-independent.
 * 19. Resource requirements remain distinct from constraints.
 * 20. Capabilities remain distinct from resources.
 * 21. Syntax remains compatible with the canonical AST layer.
 * 22. Rust integration requires no `unsafe`.
 *
 * Downstream semantic ownership:
 *
 *   declarations
 *       |
 *       +--> symbol resolution
 *       +--> type checking
 *       +--> capability checking
 *       +--> effect checking
 *       +--> resource/constraint validation
 *       |
 *       v
 *   semantic AST
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL IR
 *       +--> hardware abstraction
 *       |
 *       v
 *   optimization / routing / scheduling / lowering / runtime
 *
 * Quantum boundary:
 *
 *   quantum declaration
 *          |
 *          v
 *   semantic quantum representation
 *          |
 *          v
 *   quantum::ir
 *
 * QEC, ZQN, scheduling, routing, hardware HAL, and resilience MUST NOT
 * become dependencies of this grammar file merely because a declaration
 * mentions those concepts.
 *
 * They consume the semantic representation later in the pipeline.
 */