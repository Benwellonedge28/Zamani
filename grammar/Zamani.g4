// Zamani.g4 — Comprehensive ANTLR4 combined grammar for the Zamani omniversal language compiler.
// Expanded with full OOP feature set and additional features inspired by many languages (C++, Java, C#, Rust, Python, Kotlin, Swift, TypeScript).
// New additions: friend declarations, extern/ffi, templates (C++-style), metaclasses, access-specifier sections (public:/protected:/private:),
// constexpr/consteval, native/extern methods, method references (::), RAII/unique_ptr/move semantics markers, copy/move constructors,
// explicit override/virtual resolution, sealed/final behavior, module-level visibility, annotation targets, checked/un-checked exceptions markers,
// named argument spread, splat operator, trailing return types, and more syntactic sugar.

grammar Zamani;

program : declaration* EOF ;

declaration
    : docComment? attributeDecl? ( moduleDecl
    | importDecl
    | exportDecl
    | packageDecl
    | functionDecl
    | templateDecl
    | structDecl
    | enumDecl
    | traitDecl
    | implDecl
    | typeAliasDecl
    | constStmt
    | classDecl
    | interfaceDecl
    | recordDecl
    // === Quantum Computing ===
    | quantumCircuitDecl
    | quantumGateDecl
    | quantumMeasurementDecl
    | quantumVariationalDecl
    | quantumTranspilerDecl
    // === Nano Runtime ===
    | nanoAgentDecl
    | nanoExecutorDecl
    | nanoContextDecl
    // === Core Language ===
    | languageDecl
    | effectDecl
    | mtsDecl
    | sankofaDecl
    | agentDecl
    | cognitiveBlock
    | metaBlock
    // === Hardware/HDL ===
    | hdlModuleDecl
    | verilogModuleDecl
    | spirvShaderDecl
    // === Distributed Systems ===
    | cloudDecl
    | distributedDecl
    | onDeviceAgentDecl
    | nimbusNodeDecl
    | nimbusClusterDecl
    | evasFilterDecl
    // === Self-Evolution ===
    | selfEvolveDecl
    | optPassDecl
    | selfDiscoverDecl
    | selfAdjustDecl
    | selfVersioningDecl
    // === Target/Platform ===
    | targetPlatform
    | runtimeDecl
    // === Concurrency/Actors ===
    | actorDecl
    | messageHandlerDecl
    | concurrentDataStructureDecl
    // === AI Systems ===
    | aiSystemDecl
    | agiSystemDecl
    | asiSystemDecl
    | aesiSystemDecl
    | asesiSystemDecl
    | cognitiveEngineDecl
    | cognitiveArchitectureBlock
    | knowledgeGraphBlock
    // === Administrative/Security ===
    | adminInterfaceDecl
    | paymentGatewayDecl
    | userFeedbackDecl
    | userBlockingDecl
    | copyrightNoticeDecl
    | tailorMadeFeatureDecl
    | programOnceDecl
    | maliciousIdeaDetection
    | legalActionDecl
    | sandboxDecl
    // === Omniversal Features ===
    | omniversalSimulationDecl
    | omniversalCodeSynthDecl
    | omniversalDeployDecl
    | omniversalAlignmentDecl
    | omniversalContainmentDecl
    | omniversalTrustDecl
    | omniversalKnowledgeDecl
    | omniversalGenerativeDecl
    | omniversalSovereigntyDecl
    | omniversalGoalDecl
    | omniversalBioNanoDecl
    | omniversalRealityDecl
    | omniversalNlpDecl
    // === Chat/Architecture ===
    | chatArchitectDecl
    // === Resource Management ===
    | greenComputingAttr
    | thermalOptDecl
    | resourceConserveDecl
    | resourceOrchestratorDecl
    // === Analytics/Tracking ===
    | developerAnalyticsDecl
    | licenseTrackingDecl
    | dataProvenanceDecl
    // === Deployment ===
    | deploymentDecl
    | versionReleaseDecl
    // === LSP/Language Server ===
    | lspServerDecl
    // === Advanced Type System ===
    | typeClassDecl
    | typeClassInstance
    | higherKindedTypeDecl
    | typeProviderDecl
    | fileScopedType
    // === Extensions ===
    | extensionDecl
    | extensionMethodDecl
    | extensionPropertyDecl
    | extensionIndexerDecl
    | extensionOperatorDecl
    | macroDecl
    | domainSpecificLanguageDecl
    | aspectDecl
    // === Parallelism ===
    | dataParallelismDecl
    | messagePassingDecl
    // === AI/ML Features ===
    | deepLearningDecl
    | mlModelDecl
    | quantumMlBlock
    | transferLearningBlock
    | multiAgentBlock
    | autonomousSystemBlock
    | explainableRlBlock
    | explainableDeepLearningBlock
    | probabilisticGraphicalModelBlock
    | advancedNlpBlock
    | aiForBusinessBlock
    // === Graphics/Multimedia ===
    | graphicsDecl
    | videoDecl
    | musicDecl
    // === Specialized Domains ===
    | roboticsDecl
    | tensorDecl
    | matrixDecl
    | vectorDecl
    | graphModelingBlock
    | vrArInteractionBlock
    | imageVideoAnalysisBlock
    // === Cryptography ===
    | cryptoDecl
    | quantumIdentityDecl
    | provenanceManagerDecl
    // === Meta-Programming ===
    | metaProgrammingDecl
    | languageDialectDecl
    | reflectionDecl
    // === Math Foundations ===
    | mathematicalDiscoveryDecl
    | conjectureProofDecl
    | algebraicGeometryDecl
    | differentialGeometryDecl
    | categoryTheoryDecl
    | numberTheoryDecl
    // === Hybrid/Interface ===
    | hybridDef
    | interfaceDef
    // === Statements ===
    | statement
    ) ;

// ============================================================================
// Additional top-level constructs
// ============================================================================

templateDecl : 'template' '<' templateParam (',' templateParam)* '>' declaration ;
templateParam : ('typename' | 'class' | 'int' | 'size_t')? IDENTIFIER ( '=' typeExpr )? ;

metaclassDecl : 'metaclass' ident '{' metaclassMember* '}' ;
metaclassMember : methodDecl | propertyDecl | attributeDecl ;

externDecl : 'extern' (STRING | ident) '{' externMember* '}' ';' ;
externMember : functionDecl | variableImport ;
variableImport : 'var' ident ':' typeExpr ';' ;

friendDecl : 'friend' (classDecl | functionDecl | ident) ';' ;

// ============================================================================
// MODULE SYSTEM
// ============================================================================

moduleDecl : 'module' ident ('::' ident)* (blockExpr | ';') ;
importDecl : 'import' modulePath ('as' ident)? ';'? ;
exportDecl : 'export' ident ('to' ident)? ';'? ;
modulePath : ident ('::' ident | '.' ident)* ;
useStmt : 'use' usePath ';'? ;
usePath : segment ('::' segment)* ('::' '*')? | segment ('::' segment)* '::' '{' ident (',' ident)* '}' ;
segment : ident ;
globalUsing : 'global' 'using' ident ';' ;

// Package / manifest
packageDecl : 'package' ident '{' packageField* '}' ;
packageField : 'version' ':' STRING ';'
             | 'depends' ':' '[' (depSpec (',' depSpec)*)? ']' ';'
             | 'repository' ':' STRING ';'
             | 'license' ':' STRING ';'
             ;

depSpec : STRING ('@' (STRING | 'semver'))? ;

// ============================================================================
// FUNCTIONS & DECLARATIONS
// ============================================================================

functionDecl : modifiers? ('constexpr' | 'consteval')? 'fn' ident typeParams? '(' params? ')' ('->' typeExpr)? ('with' effectList)? contractDecl? ( 'const' )? ( 'noexcept' | 'throws' typeExprList )? blockExpr ;
params : param (',' param)* (',' '...' )? ;
param : 'mut'? ident (':' typeExpr)? ('=' expression)? | '...' typeExpr ident ;
modifiers : modifier+ ;
modifier : 'pub' | 'private' | 'protected' | 'static' | 'const' | 'async' | 'unsafe' | 'inline' 
         | 'override' | 'final' | 'abstract' | 'virtual' | 'sealed' | 'partial' | 'file' 
         | 'required' | 'internal' | 'visible' | 'experimental' | 'deprecated' | 'noInline'
         | 'synchronized' | 'transient' | 'volatile' | 'readonly' | 'lazy' | 'atomic' | 'native' | 'extern' ;

// Contract/spec
contractDecl : 'contract' ident '{' contractClause* '}' ;
contractClause : 'requires' '(' expression ')' ';'
               | 'ensures' '(' expression ')' ';'
               | 'invariant' '(' expression ')' ';'
               ;

// ============================================================================
// STATEMENTS & CONTROL FLOW
// ============================================================================

statement 
    : returnStmt
    | breakStmt
    | continueStmt
    | whileStmt
    | forStmt
    | loopExpr
    | ifExpr
    | matchStmt
    | unsafeBlock
    | throwStmt
    | tryCatchStmt
    | letStmt
    | constStmt
    | blockExpr
    | yieldStmt
    | expression ';'? ;

returnStmt : 'return' expression? ';' ;
breakStmt : 'break' ';' ;
continueStmt : 'continue' ';' ;
whileStmt : 'while' expression blockExpr ;
forStmt : 'for' ident 'in' expression blockExpr ;
loopExpr : 'loop' blockExpr ;
ifExpr : 'if' expression blockExpr ('else' (ifExpr | blockExpr))? ;
matchStmt : matchExpr ;
matchExpr : 'match' expression '{' matchCase* '}' ;
matchCase : ('case' pattern | pattern) ('when' expression)? '=>' expression ','? ;
pattern : ident | literal | '_' | '(' pattern (',' pattern)* ')' | '[' pattern (',' pattern)* ']' 
        | '[' pattern (',' pattern)* '...' pattern ']' | pattern '|' pattern | ident ':' typeExpr ;
unsafeBlock : 'unsafe' (ident? blockExpr | '!' '(' 'evas' ':' expression ')' blockExpr) ;
throwStmt : 'throw' expression ('checked' | 'unchecked')? ';' ;
tryCatchStmt : 'try' blockExpr catchClause* finallyClause? ;
catchClause : 'catch' ('(' param ')')? blockExpr ;
finallyClause : 'finally' blockExpr ;
blockExpr : '{' statement* '}' ;

letStmt : ('let' | 'var') 'mut'? ident (':' typeExpr)? '=' expression ';' ;
constStmt : 'const' ident (':' typeExpr)? '=' expression ';' ;

// Yield for generators / coroutines
yieldStmt : 'yield' expression ';' ;

// ============================================================================
// EXPRESSIONS
// ============================================================================

expression : assignmentExpr ;
assignmentExpr : rangeExpr (assignOp assignmentExpr)? ;
assignOp : '=' | '+=' | '-=' | '*=' | '/=' | '%=' | '&=' | '|=' | '^=' | '<<=' | '>>=' ;
rangeExpr : logicalOrExpr (('..' | '..=') logicalOrExpr)? ;
logicalOrExpr : logicalAndExpr (('||' | 'or') logicalAndExpr)* ;
logicalAndExpr : bitOrExpr (('&&' | 'and') bitAndExpr)* ;
bitOrExpr : bitXorExpr ('|' bitXorExpr)* ;
bitXorExpr : bitAndExpr ('^' bitAndExpr)* ;
bitAndExpr : equalityExpr ('&' equalityExpr)* ;
equalityExpr : comparisonExpr (('==' | '!=' | '===' | '!==') comparisonExpr)* ;
comparisonExpr : shiftExpr (('<' | '<=' | '>' | '>=' | 'instanceof' | 'is' | 'has') shiftExpr)* ;
shiftExpr : sumExpr (('<<' | '>>' | '>>>') sumExpr)* ;
sumExpr : productExpr (('+' | '-') productExpr)* ;
productExpr : castExpr (('*' | '/') castExpr)* ;
castExpr : prefixExpr (('as' | ':') typeExpr)* ;
prefixExpr : ('-' | '!' | '~' | '&' 'mut'? | '*' | '++' | '--' | '@') prefixExpr | postfixExpr ;
postfixExpr : primaryExpr postfixOp* ;

postfixOp 
    : '(' args? ')' # callOp
    | '[' expression ']' # indexOp
    | '.' ident ('(' args? ')')? # memberOp
    | '?' # tryPropagateOp
    | '++' # postIncOp
    | '--' # postDecOp
    | 'with' '[' effectList ']' # withEffectOp
    | 'with' '{' (ident ':' expression ';')* '}' # withBlockOp
    | 'await'                                      # awaitOp
    | '::' ident                                   # methodRefOp
    ;

args : expression (',' expression)* | namedArgument (',' namedArgument)* ;
namedArgument : ident '=' expression ;

primaryExpr
    : ident structLiteralTail? # identExpr
    | literal # literalExpr
    | '(' expression ')' # parenExpr
    | '(' expression (',' expression)+ ')' # tupleExpr
    | '[' (expression (',' expression)*)? ']' # arrayExpr
    | 'map' '{' (expression '=>' expression (',' expression '=>' expression)*)? '}' # mapExpr
    | blockExpr # blockValExpr
    | '|' params? '|' (blockExpr | expression) # lambdaExpr
    | 'fn' '(' params? ')' ('->' typeExpr)? blockExpr # anonFnExpr
    | ifExpr # ifValExpr
    | matchExpr # matchValExpr
    | loopExpr # loopValExpr
    | 'quantum' '{' quantumGatePath* '}' # quantumExpr
    | 'nano' '{' nanoInstr* '}' # nanoExpr
    | 'spawn' (expression | blockExpr) # spawnExpr
    | 'channel' '(' typeExpr (',' INT)? ')' # channelExpr
    | 'new' typeExpr ('(' args? ')')? # newExpr
    | 'this' ('(' args? ')')? # thisExpr
    | 'super' ('(' args? ')')? # superExpr
    | anonymousClassExpr
    | objectExpr
    | methodRefExpr
    ;

anonymousClassExpr : 'new' typeExpr '{' classMember* '}' ;
objectExpr : 'object' ('<' typeArgs '>')? '{' classMember* '}' ;
methodRefExpr : primaryExpr '::' ident ;

quantumGatePath : ident ('(' expression (',' expression)* ')')? ';' ;
nanoInstr : ident ('(' expression (',' expression)* ')')? ';' ;

// ============================================================================
// TYPE SYSTEM (with variance and additional generics features)
// ============================================================================

typeExpr
    : 'i8' | 'i16' | 'i32' | 'i64' | 'u8' | 'u16' | 'u32' | 'u64' | 'f32' | 'f64'
    | 'bool' | 'str' | 'char' | 'void'
    | ident typeArgs?
    | typeExpr '[' ']'
    | '(' typeExpr (',' typeExpr)* ')'
    | typeExpr '|' typeExpr
    | typeExpr '&' typeExpr
    | typeExpr '->' typeExpr
    | typeExpr '?'
    | 'quantum' typeExpr
    | 'nano' typeExpr
    ;

typeParams : '<' typeParam (',' typeParam)* '>' ;
// variance: in/out on type params
typeParam : ('in' | 'out')? ident ('extends' typeExpr)? ('=' typeExpr)? ;
// wildcards like Java: ?, ? extends T, ? super T
wildcardType : '?' ( 'extends' typeExpr | 'super' typeExpr )? ;

typeArgs : '<' (typeExpr | wildcardType) (',' (typeExpr | wildcardType))* '>' ;

// where/generic constraints
genericWhereClause : 'where' genericConstraint (',' genericConstraint)* ;
genericConstraint : ident ':' typeExpr ;

// ============================================================================
// STRUCTURES & TYPES (ADVANCED OOP)
// ============================================================================

structDecl : 'struct' ident typeParams? '{' fieldDecl* '}' ;
fieldDecl : modifiers? ident ':' typeExpr ('=' expression)? ';' ;

enumDecl : 'enum' ident typeParams? '{' enumVariant (',' enumVariant)* ','? '}' ;
enumVariant : ident ('(' typeExpr (',' typeExpr)* ')' | '{' fieldDecl* '}' )? ( 'when' blockExpr )? ;

// Classes: support data, sealed, abstract, final, mixin, companion, delegation
classDecl : classModifiers? classKind? 'class' ident typeParams? primaryConstructor? ('extends' baseList)? ('implements' typeExpr (',' typeExpr)*)? classBody? ;
classKind : 'data' | 'abstract' | 'final' | 'sealed' | 'mixin' ;
classModifiers : modifier+ ;

baseList : typeExpr (',' typeExpr)* ;

// primary constructor inline with class header
primaryConstructor : '(' constructorParamList? ')' ('where' genericWhereClause)? ;
constructorParamList : constructorParam (',' constructorParam)* ;
constructorParam : modifiers? ('val' | 'var')? ident (':' typeExpr)? ('=' expression)? ('move' | 'copy')? ;

// optional class body — support C++ style access specifiers sections
classBody : '{' classBodyElement* '}' ;
classBodyElement : accessSection | classMember ;
accessSection : ('public' | 'protected' | 'private' | 'friend') ':' classMember* ;

// secondary constructors
secondaryConstructor : 'constructor' '(' params? ')' (':' constructorInitList)? blockExpr ;
constructorInitList : ident '(' args? ')' (',' ident '(' args? ')')* ;

// class members: fields, properties, methods, constructors, nested types, companion, delegates, operators, events, indexers
classMember : fieldDecl
            | propertyDecl
            | methodDecl
            | secondaryConstructor
            | destructorDecl
            | nestedTypeDecl
            | companionDecl
            | delegationDecl
            | operatorDecl
            | eventDecl
            | delegateDecl
            | indexerDecl
            | staticBlock
            | initBlock
            | friendDecl
            | externDecl
            | ';'
            ;

// nested/inner class
nestedTypeDecl : classDecl | interfaceDecl | enumDecl | traitDecl | recordDecl ;

// companion object
companionDecl : 'companion' ('object' | 'class')? (ident)? '{' classMember* '}' ;

// delegation: 'by' clause to delegate implementation of an interface to a field/expression
delegationDecl : 'delegate' 'to' expression ';' ;

// property with backing field, delegates, observers, accessors
propertyDecl : modifiers? ('val' | 'var') ident (':' typeExpr)? ( '=' expression )? ( '{' propertyAccessor* '}' | ';' ) ;
// property accessor supports accessor-level modifiers and visibility
propertyAccessor : accessorModifiers? ( 'get' '(' ')' ( 'async' )? | 'set' '(' param? ')' | 'init' '(' ')' ) blockExpr ;
accessorModifiers : modifier+ ;

// property delegation: e.g., val x by delegateExpr
propertyDelegate : 'by' expression ;
// property observers
propertyObserver : 'willSet' '(' ident? ')' blockExpr | 'didSet' '(' ident? ')' blockExpr ;

methodDecl : modifiers? 'fn' ident typeParams? '(' params? ')' ('->' typeExpr)? ('throws' typeExprList)? ( 'const' )? ( 'noexcept' )? blockExpr ;
// destructor
destructorDecl : 'destructor' '(' ')' ( 'noexcept' )? blockExpr ;

// static initializer and instance init blocks
staticBlock : 'static' blockExpr ;
initBlock : 'init' blockExpr ;

// operator overloading
operatorDecl : modifiers? 'operator' operatorToken '(' params? ')' ('->' typeExpr)? blockExpr ;
operatorToken : '+' | '-' | '*' | '/' | '%' | '==' | '!=' | '<' | '<=' | '>' | '>=' | '<<' | '>>' | '[]' | '()' | '[]=' | '->' | '|' | '^' | '&' | '::' ;

// events and delegates
eventDecl : modifiers? 'event' ident (':' typeExpr)? ( '{' eventAccessor* '}' | ';' ) ;
eventAccessor : 'add' '(' param? ')' blockExpr | 'remove' '(' param? ')' blockExpr | 'invoke' '(' params? ')' blockExpr ;

delegateDecl : modifiers? 'delegate' ident '(' params? ')' ('->' typeExpr)? ';' ;

// indexer declaration
indexerDecl : modifiers? 'indexer' '(' params? ')' ':' typeExpr ( '{' propertyAccessor* '}' | ';' ) ;

interfaceDecl : 'interface' ident typeParams? '{' interfaceMember* '}' ;
interfaceMember : methodSignature | propertySignature | typeMember ;
methodSignature : ident '(' params? ')' ('->' typeExpr)? ('default' blockExpr)? ';' ;
propertySignature : 'get'? 'set'? ident ':' typeExpr ('default' '{' propertyAccessor* '}' )? ';' ;

typeMember : 'type' ident ('=' typeExpr)? ';' ;

recordDecl : 'record' ident '(' recordField (',' recordField)* ')' ('extends' typeExpr)? ( '{' classMember* '}' )? ;
recordField : modifiers? ident ':' typeExpr ('=' expression)? ;

typeAliasDecl : 'type' ident '=' typeExpr ';' ;
traitDecl : 'trait' ident typeParams? '{' traitMember* '}' ;
traitMember : methodSignature | propertySignature | typeMember ;

implDecl : 'impl' typeParams? typeExpr ('for' typeExpr)? '{' implMember* '}' ;
implMember : methodDecl | associatedTypeImpl | propertyDecl ;
associatedTypeImpl : 'type' ident '=' typeExpr ';' ;

// ============================================================================
// QUANTUM COMPUTING
// ============================================================================

quantumCircuitDecl : 'quantum_circuit' ident '{' quantumGatePath* '}' ;
quantumGateDecl : 'quantum_gate' ident '(' params? ')' blockExpr ;
quantumMeasurementDecl : 'quantum_measure' ident '{' ident (',' ident)* '}' ;
quantumVariationalDecl : 'quantum_variational' ident '(' params? ')' blockExpr ;
quantumTranspilerDecl : 'quantum_transpile' ident 'to' ident blockExpr ;

// ============================================================================
// (rest unchanged) ...
// ============================================================================

// NANO RUNTIME
nanoAgentDecl : 'nano_agent' ident '{' nanoAgentMember* '}' ;
nanoAgentMember : ident ':' typeExpr ';' | methodDecl ;
nanoExecutorDecl : 'nano_executor' ident '{' executorConfig* '}' ;
executorConfig : 'config' ident '=' expression ';' ;
nanoContextDecl : 'nano_context' ident '{' ident ':' typeExpr ';'* '}' ;

// CORE LANGUAGE FEATURES
languageDecl : 'language' ident '{' languageFeature* '}' ;
languageFeature : 'feature' ident ';' ;

effectDecl : 'effect' ident '{' effectMember* '}' ;
effectMember : ident (':' typeExpr)? ';' ;
effectList : effect (',' effect)* ;
effect : ident ('(' expression (',' expression)* ')')? ;

mtsDecl : 'mts' ident '{' mtsState* '}' ;
mtsState : ident ':' typeExpr ';' ;

sankofaDecl : 'sankofa' ident '{' sankofaMember* '}' ;
sankofaMember : ident ':' typeExpr ';' | methodDecl ;

agentDecl : 'agent' ident '{' agentProperty* agentBehavior* '}' ;
agentProperty : 'property' ident ':' typeExpr ';' ;
agentBehavior : 'behavior' ident blockExpr ;

cognitiveBlock : 'cognitive' '{' cognitiveStmt* '}' ;
cognitiveStmt : 'reason' expression ';' | 'learn' expression ';' | 'adapt' expression ';' ;

metaBlock : 'meta' '{' metaOperation* '}' ;
metaOperation : 'transform' expression ';' | 'reflect' ident ';' ;

// HARDWARE/HDL
hdlModuleDecl : 'hdl_module' ident '{' hdlPort* hdlStmt* '}' ;
hdlPort : ('input' | 'output') ident ':' typeExpr ';' ;
hdlStmt : 'assign' ident '=' expression ';' | 'always' '@' '(' hdlEvent ')' blockExpr ;
hdlEvent : 'posedge' ident | 'negedge' ident ;

verilogModuleDecl : 'verilog_module' IDENTIFIER '{' verilogContent '}' ;
verilogContent : (~('{' | '}'))* ;

spirvShaderDecl : 'spirv_shader' ident '{' spirvInstr* '}' ;
spirvInstr : ident expression ';' ;

// DISTRIBUTED SYSTEMS
cloudDecl : 'cloud' ident '{' cloudConfig* '}' ;
cloudConfig : ident '=' expression ';' ;

distributedDecl : 'distributed' ident '{' distributeTarget* '}' ;
distributeTarget : 'target' ident ';' ;

onDeviceAgentDecl : 'on_device_agent' ident '{' deviceAgentConfig* '}' ;
deviceAgentConfig : ident ':' typeExpr ';' ;

nimbusNodeDecl : 'nimbus_node' ident '{' nimbusNodeProperty* '}' ;
nimbusNodeProperty : ident ':' typeExpr ';' | 'role' ':' ident ';' ;

nimbusClusterDecl : 'nimbus_cluster' ident '{' nimbusClusterMember* '}' ;
nimbusClusterMember : 'node' ident ';' | 'consensus_threshold' ':' FLOAT ';' ;

evasFilterDecl : 'evas_filter' ident '{' evasPolicy* '}' ;
evasPolicy : 'policy' ident blockExpr ;

// SELF-EVOLUTION & ADAPTATION
selfEvolveDecl : 'self_evolve' ident '{' evolutionRule* '}' ;
evolutionRule : 'rule' ident blockExpr ;

optPassDecl : 'opt_pass' ident blockExpr ;

selfDiscoverDecl : 'self_discover' ident '{' discoveryTarget* '}' ;
discoveryTarget : 'discover' ident ';' ;

selfAdjustDecl : 'self_adjust' ident '{' adjustmentStrategy* '}' ;
adjustmentStrategy : 'strategy' ident blockExpr ;

selfVersioningDecl : 'self_version' ident '{' versionInfo* '}' ;
versionInfo : 'version' STRING ';' | 'depends_on' ident ';' ;

// TARGET & RUNTIME
targetPlatform : 'target' ident blockExpr ;
runtimeDecl : 'runtime' ident '{' runtimeConfig* '}' ;
runtimeConfig : ident '=' expression ';' ;

// CONCURRENCY & ACTORS
actorDecl : 'actor' ident '{' actorMember* '}' ;
actorMember : methodDecl | 'mailbox' ':' typeExpr ';' | 'state' ':' typeExpr ';' ;

messageHandlerDecl : 'message_handler' ident blockExpr ;

concurrentDataStructureDecl : 'concurrent' typeExpr '{' concurrencyMember* '}' ;
concurrencyMember : methodDecl | 'lock' ':' typeExpr ';' ;

// AI/ML SYSTEMS
aiSystemDecl : 'ai_system' ident '{' aiSystemMember* '}' ;
aiSystemMember : methodDecl | 'model' ':' typeExpr ';' ;

agiSystemDecl : 'agi_system' ident '{' agiMember* '}' ;
agiMember : 'capability' ':' typeExpr ';' | methodDecl ;

asiSystemDecl : 'asi_system' ident '{' asiMember* '}' ;
asiMember : 'specialization' ':' typeExpr ';' | methodDecl ;

aesiSystemDecl : 'aesi_system' ident '{' aesiMember* '}' ;
aesiMember : 'enterprise_feature' ':' typeExpr ';' | methodDecl ;

asesiSystemDecl : 'asesi_system' ident '{' asesiMember* '}' ;
asesiMember : 'sovereign_feature' ':' typeExpr ';' | methodDecl ;

cognitiveEngineDecl : 'cognitive_engine' ident '{' cognitiveEngineMember* '}' ;
cognitiveEngineMember : 'transform' ':' typeExpr ';' | 'dialect' ':' typeExpr ';' | methodDecl ;

cognitiveArchitectureBlock : 'cognitive_architecture' '{' architectureComponent* '}' ;
architectureComponent : ident ':' typeExpr ';' ;

knowledgeGraphBlock : 'knowledge_graph' '{' graphNode* '}' ;
graphNode : 'node' ident '{' nodeProperty* '}' ;
nodeProperty : ident ':' typeExpr ';' ;

// SECURITY & ADMINISTRATION
adminInterfaceDecl : 'admin_interface' ident '{' adminMethod* '}' ;
adminMethod : methodDecl ;

paymentGatewayDecl : 'payment_gateway' ident '{' paymentMethod* '}' ;
paymentMethod : 'method' ident blockExpr ;

userFeedbackDecl : 'user_feedback' ident '{' feedbackConfig* '}' ;
feedbackConfig : ident '=' expression ';' ;

userBlockingDecl : 'user_blocking' '{' blockingRule* '}' ;
blockingRule : 'rule' ident blockExpr ;

copyrightNoticeDecl : 'copyright' STRING ';' ;

tailorMadeFeatureDecl : 'tailormade' ident '{' featureConfig* '}' ;
featureConfig : ident '=' expression ';' ;

programOnceDecl : 'program_once' ident blockExpr ;

maliciousIdeaDetection : 'detect_malicious' '{' detectionRule* '}' ;
detectionRule : 'rule' ident blockExpr ;

legalActionDecl : 'legal_action' ident blockExpr ;

sandboxDecl : 'sandbox' ident '{' sandboxPolicy* '}' ;
sandboxPolicy : 'policy' ident blockExpr ;

// OMNIVERSAL FEATURES
omniversalSimulationDecl : 'omniversal_simulation' ident '{' simConfig* '}' ;
simConfig : ident '=' expression ';' ;

omniversalCodeSynthDecl : 'omniversal_code_synth' ident blockExpr ;

omniversalDeployDecl : 'omniversal_deploy' ident '{' deployConfig* '}' ;
deployConfig : 'target' '=' ident ';' ;

omniversalAlignmentDecl : 'omniversal_alignment' '{' alignmentRule* '}' ;
alignmentRule : 'rule' ident blockExpr ;

omniversalContainmentDecl : 'omniversal_containment' ident blockExpr ;

omniversalTrustDecl : 'omniversal_trust' '{' trustPolicy* '}' ;
trustPolicy : 'policy' ident blockExpr ;

omniversalKnowledgeDecl : 'omniversal_knowledge' ident '{' knowledgeItem* '}' ;
knowledgeItem : 'fact' STRING ';' ;

omniversalGenerativeDecl : 'omniversal_generative' ident blockExpr ;

omniversalSovereigntyDecl : 'omniversal_sovereignty' ident blockExpr ;

omniversalGoalDecl : 'omniversal_goal' ident blockExpr ;

omniversalBioNanoDecl : 'omniversal_bionano' ident '{' bioConfig* '}' ;
bioConfig : ident '=' expression ';' ;

omniversalRealityDecl : 'omniversal_reality' ident '{' realityConfig* '}' ;
realityConfig : ident '=' expression ';' ;

omniversalNlpDecl : 'omniversal_nlp' ident '{' nlpComponent* '}' ;
nlpComponent : 'module' ident ';' ;

// CHAT & ARCHITECTURE
chatArchitectDecl : 'chat_architect' ident '{' chatConfig* '}' ;
chatConfig : ident '=' expression ';' ;

// RESOURCE MANAGEMENT
greenComputingAttr : '@green_computing' ;

thermalOptDecl : 'thermal_optimization' '{' thermalPolicy* '}' ;
thermalPolicy : 'policy' ident blockExpr ;

resourceConserveDecl : 'resource_conservation' ident '{' conservePolicy* '}' ;
conservePolicy : 'policy' ident blockExpr ;

resourceOrchestratorDecl : 'resource_orchestrator' ident '{' orchestratorConfig* '}' ;
orchestratorConfig : ident '=' expression ';' ;

// ANALYTICS & TRACKING
developerAnalyticsDecl : 'dev_analytics' ident '{' analyticsConfig* '}' ;
analyticsConfig : ident '=' expression ';' ;

licenseTrackingDecl : 'license_tracking' ident blockExpr ;

dataProvenanceDecl : 'data_provenance' ident '{' provenanceConfig* '}' ;
provenanceConfig : ident ':' typeExpr ';' ;

// DEPLOYMENT
deploymentDecl : 'deployment' ident '{' deploymentTarget* '}' ;
deploymentTarget : 'target' ident ';' ;

versionReleaseDecl : 'version_release' STRING '{' releaseNote* '}' ;
releaseNote : 'note' STRING ';' ;

// LSP & LANGUAGE SERVER
lspServerDecl : 'lsp_server' ident '{' lspConfig* '}' ;
lspConfig : ident '=' expression ';' ;

// ADVANCED TYPE SYSTEM
typeClassDecl : 'typeclass' ident typeParams? '{' typeClassMember* '}' ;
typeClassMember : methodSignature ;

typeClassInstance : 'instance' typeExpr 'for' typeExpr '{' implMember* '}' ;

higherKindedTypeDecl : 'hkt' ident typeParams? blockExpr ;

typeProviderDecl : 'type_provider' ident blockExpr ;

fileScopedType : 'file_scope' typeExpr ';' ;

// EXTENSIONS
extensionDecl : 'extension' typeExpr '{' extensionMember* '}' ;
extensionMember : methodDecl | propertyDecl | indexerDecl | operatorDecl ;

extensionMethodDecl : 'extension' typeExpr '{' methodDecl* '}' ;
extensionPropertyDecl : 'extension_property' ident ':' typeExpr blockExpr ;
extensionIndexerDecl : 'extension_indexer' typeExpr blockExpr ;
extensionOperatorDecl : 'extension_operator' ident blockExpr ;

macroDecl : 'macro' ident '(' params? ')' blockExpr ;

domainSpecificLanguageDecl : 'dsl' ident '{' dslRule* '}' ;
dslRule : ident '=>' expression ';' ;

aspectDecl : 'aspect' ident '{' aspectAdvice* '}' ;
aspectAdvice : 'before' | 'after' | 'around' ;

// PARALLELISM & MESSAGE PASSING
// (select/channel previously defined)

// (AI/ML, GRAPHICS, SPECIALIZED DOMAINS, CRYPTOGRAPHY, META-PROGRAMMING,
// MATHEMATICAL FOUNDATIONS, HYBRID & INTERFACES etc.) — keep as previously defined

// LITERALS & TOKENS (expanded)
literal
    : INT
    | BIGINT
    | DECIMAL
    | FLOAT
    | STRING
    | MULTILINE_STRING
    | RAW_STRING
    | BYTEARRAY
    | CHAR
    | 'true'
    | 'false'
    | 'null'
    | 'undefined'
    ;

structLiteralTail : '{' (ident ':' expression (',' ident ':' expression)*)? '}' ;

ident : IDENTIFIER ;
docComment : DOC_COMMENT ;
attributeDecl : ('@' ident ('(' args? ')')?)+ ;

// LEXER TOKENS
fragment LETTER : [a-zA-Z_] ;
fragment DIGIT : [0-9] ;
fragment HEX_DIGIT : [0-9a-fA-F] ;

// Some identifiers are emitted as explicit tokens to avoid ambiguity
THIS : 'this' ;
SUPER : 'super' ;
NEW : 'new' ;
OBJECT : 'object' ;
COMPANION : 'companion' ;
DATA : 'data' ;
SEALED : 'sealed' ;
MIXIN : 'mixin' ;
FRIEND : 'friend' ;
EXTERN : 'extern' ;
TEMPLATE : 'template' ;
METACLASS : 'metaclass' ;
CONSTEXPR : 'constexpr' ;
CONSTEVAL : 'consteval' ;
NATIVE : 'native' ;

// Raw / multiline / byte array strings
RAW_STRING : 'r#"' (~[\n\r])* '"#' ;
MULTILINE_STRING : '"""' (~["]| '"' ~["] | '""' ~["] )* '"""' ;
BYTEARRAY : 'b"' (~["\\\r\n] | '\\' .)* '"' ;

// Numeric literal extensions
BIGINT : DIGIT+ 'n' ;            // e.g., 123n
DECIMAL : DIGIT+ '.' DIGIT+ 'd' ; // e.g., 1.23d

IDENTIFIER : LETTER (LETTER | DIGIT)* ;
INT : DIGIT+ ('_' DIGIT+)* | '0x' HEX_DIGIT+ | '0b' [01]+ ;
FLOAT : DIGIT+ '.' DIGIT+ ([eE] [+-]? DIGIT+)? ;
STRING : '"' (~["\\\n\r] | '\\' . )* '"' ;
CHAR : '\'' (~['\\\n\r] | '\\' .) '\'' ;
DOC_COMMENT : '///' (~[\n\r])* ;
COMMENT : '//' (~[\n\r])* -> skip ;
BLOCK_COMMENT : '/*' .*? '*/' -> skip ;
WS : [ \t\r\n]+ -> skip ;
