// Zamani.g4 — Comprehensive ANTLR4 combined grammar for the Zamani omniversal language compiler.
// Includes OOP enhancements: classes (abstract/final/sealed), properties with accessors, constructors, destructors,
// static/init blocks, inner/nested types, operator overloading, 'new', 'this', 'super', synchronized, transient, volatile, readonly, lazy.

grammar Zamani;

program : declaration* EOF ;

declaration
    : docComment? attributeDecl? ( moduleDecl
    | importDecl
    | exportDecl
    | packageDecl
    | functionDecl
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

functionDecl : modifiers? 'fn' ident typeParams? '(' params? ')' ('->' typeExpr)? ('with' effectList)? contractDecl? blockExpr ;
params : param (',' param)* ;
param : 'mut'? ident (':' typeExpr)? ('=' expression)? | '...' typeExpr ident ;
modifiers : modifier+ ;
modifier : 'pub' | 'private' | 'protected' | 'static' | 'const' | 'async' | 'unsafe' | 'inline' 
         | 'override' | 'final' | 'abstract' | 'virtual' | 'sealed' | 'partial' | 'file' 
         | 'required' | 'internal' | 'visible' | 'experimental' | 'deprecated' | 'noInline'
         | 'synchronized' | 'transient' | 'volatile' | 'readonly' | 'lazy' ;

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
throwStmt : 'throw' expression ';' ;
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
prefixExpr : ('-' | '!' | '~' | '&' 'mut'? | '*' | '++' | '--') prefixExpr | postfixExpr ;
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
    | 'new' typeExpr ('(' args? ')')? # newExpr
    | 'this' # thisExpr
    | 'super' # superExpr
    ;

quantumGatePath : ident ('(' expression (',' expression)* ')')? ';' ;
nanoInstr : ident ('(' expression (',' expression)* ')')? ';' ;

// ============================================================================
// TYPE SYSTEM
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
typeParam : ident ('extends' typeExpr)? ;
typeArgs : '<' typeExpr (',' typeExpr)* '>' ;

// ============================================================================
// STRUCTURES & TYPES (OOP ENHANCEMENTS)
// ============================================================================

structDecl : 'struct' ident typeParams? '{' fieldDecl* '}' ;
fieldDecl : modifiers? ident ':' typeExpr ('=' expression)? ';' ;

enumDecl : 'enum' ident typeParams? '{' enumVariant (',' enumVariant)* ','? '}' ;
enumVariant : ident ('(' typeExpr (',' typeExpr)* ')' | '{' fieldDecl* '}')? ;

classDecl : classModifiers? 'class' ident typeParams? ('extends' typeExpr)? ('implements' typeExpr (',' typeExpr)*)? '{' classMember* '}' ;
classModifiers : modifier+ ;
classMember : fieldDecl | propertyDecl | methodDecl | constructorDecl | destructorDecl | classDecl | interfaceDecl | enumDecl | staticBlock | initBlock | operatorDecl | constructorDecl ;

// property with optional accessors or initializer
propertyDecl : modifiers? 'property' ident ':' typeExpr ( '{' propertyAccessor* '}' | '=' expression ';' | ';' ) ;
propertyAccessor : 'get' '(' ')' blockExpr | 'set' '(' param ')' blockExpr ;

methodDecl : modifiers? 'fn' ident '(' params? ')' ('->' typeExpr)? blockExpr ;
constructorDecl : 'constructor' '(' params? ')' blockExpr ;
destructorDecl : 'destructor' '(' ')' blockExpr ;

// static initializer and instance init blocks
staticBlock : 'static' blockExpr ;
initBlock : 'init' blockExpr ;

// operator overloading
operatorDecl : 'operator' ( '+' | '-' | '*' | '/' | '%' | '==' | '!=' | '<' | '<=' | '>' | '>=' | '<<' | '>>' | '[]' | '()' ) '(' params? ')' ('->' typeExpr)? blockExpr ;

interfaceDecl : 'interface' ident typeParams? '{' interfaceMember* '}' ;
interfaceMember : methodSignature | propertySignature ;
methodSignature : ident '(' params? ')' '->' typeExpr ';' ;
propertySignature : 'get'? 'set'? ident ':' typeExpr ';' ;

recordDecl : 'record' ident '(' recordField (',' recordField)* ')' ;
recordField : ident ':' typeExpr ;

typeAliasDecl : 'type' ident '=' typeExpr ';' ;
traitDecl : 'trait' ident typeParams? '{' traitMember* '}' ;
traitMember : methodSignature | associatedType ;
associatedType : 'type' ident ';' ;

implDecl : 'impl' typeParams? typeExpr '{' implMember* '}' ;
implMember : methodDecl | associatedTypeImpl ;
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
// NANO RUNTIME
// ============================================================================

nanoAgentDecl : 'nano_agent' ident '{' nanoAgentMember* '}' ;
nanoAgentMember : ident ':' typeExpr ';' | methodDecl ;
nanoExecutorDecl : 'nano_executor' ident '{' executorConfig* '}' ;
executorConfig : 'config' ident '=' expression ';' ;
nanoContextDecl : 'nano_context' ident '{' ident ':' typeExpr ';'* '}' ;

// ============================================================================
// CORE LANGUAGE FEATURES
// ============================================================================

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

// ============================================================================
// HARDWARE/HDL
// ============================================================================

hdlModuleDecl : 'hdl_module' ident '{' hdlPort* hdlStmt* '}' ;
hdlPort : ('input' | 'output') ident ':' typeExpr ';' ;
hdlStmt : 'assign' ident '=' expression ';' | 'always' '@' '(' hdlEvent ')' blockExpr ;
hdlEvent : 'posedge' ident | 'negedge' ident ;

verilogModuleDecl : 'verilog_module' IDENTIFIER '{' verilogContent '}' ;
verilogContent : (~('{' | '}'))* ;

spirvShaderDecl : 'spirv_shader' ident '{' spirvInstr* '}' ;
spirvInstr : ident expression ';' ;

// ============================================================================
// DISTRIBUTED SYSTEMS
// ============================================================================

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

// ============================================================================
// SELF-EVOLUTION & ADAPTATION
// ============================================================================

selfEvolveDecl : 'self_evolve' ident '{' evolutionRule* '}' ;
evolutionRule : 'rule' ident blockExpr ;

optPassDecl : 'opt_pass' ident blockExpr ;

selfDiscoverDecl : 'self_discover' ident '{' discoveryTarget* '}' ;
discoveryTarget : 'discover' ident ';' ;

selfAdjustDecl : 'self_adjust' ident '{' adjustmentStrategy* '}' ;
adjustmentStrategy : 'strategy' ident blockExpr ;

selfVersioningDecl : 'self_version' ident '{' versionInfo* '}' ;
versionInfo : 'version' STRING ';' | 'depends_on' ident ';' ;

// ============================================================================
// TARGET & RUNTIME
// ============================================================================

targetPlatform : 'target' ident blockExpr ;
runtimeDecl : 'runtime' ident '{' runtimeConfig* '}' ;
runtimeConfig : ident '=' expression ';' ;

// ============================================================================
// CONCURRENCY & ACTORS
// ============================================================================

actorDecl : 'actor' ident '{' actorMember* '}' ;
actorMember : methodDecl | 'mailbox' ':' typeExpr ';' | 'state' ':' typeExpr ';' ;

messageHandlerDecl : 'message_handler' ident blockExpr ;

concurrentDataStructureDecl : 'concurrent' typeExpr '{' concurrencyMember* '}' ;
concurrencyMember : methodDecl | 'lock' ':' typeExpr ';' ;

// ============================================================================
// AI/ML SYSTEMS
// ============================================================================

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

// ============================================================================
// SECURITY & ADMINISTRATION
// ============================================================================

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

// ============================================================================
// OMNIVERSAL FEATURES
// ============================================================================

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

// ============================================================================
// CHAT & ARCHITECTURE
// ============================================================================

chatArchitectDecl : 'chat_architect' ident '{' chatConfig* '}' ;
chatConfig : ident '=' expression ';' ;

// ============================================================================
// RESOURCE MANAGEMENT
// ============================================================================

greenComputingAttr : '@green_computing' ;

thermalOptDecl : 'thermal_optimization' '{' thermalPolicy* '}' ;
thermalPolicy : 'policy' ident blockExpr ;

resourceConserveDecl : 'resource_conservation' ident '{' conservePolicy* '}' ;
conservePolicy : 'policy' ident blockExpr ;

resourceOrchestratorDecl : 'resource_orchestrator' ident '{' orchestratorConfig* '}' ;
orchestratorConfig : ident '=' expression ';' ;

// ============================================================================
// ANALYTICS & TRACKING
// ============================================================================

developerAnalyticsDecl : 'dev_analytics' ident '{' analyticsConfig* '}' ;
analyticsConfig : ident '=' expression ';' ;

licenseTrackingDecl : 'license_tracking' ident blockExpr ;

dataProvenanceDecl : 'data_provenance' ident '{' provenanceConfig* '}' ;
provenanceConfig : ident ':' typeExpr ';' ;

// ============================================================================
// DEPLOYMENT
// ============================================================================

deploymentDecl : 'deployment' ident '{' deploymentTarget* '}' ;
deploymentTarget : 'target' ident ';' ;

versionReleaseDecl : 'version_release' STRING '{' releaseNote* '}' ;
releaseNote : 'note' STRING ';' ;

// ============================================================================
// LSP & LANGUAGE SERVER
// ============================================================================

lspServerDecl : 'lsp_server' ident '{' lspConfig* '}' ;
lspConfig : ident '=' expression ';' ;

// ============================================================================
// ADVANCED TYPE SYSTEM
// ============================================================================

typeClassDecl : 'typeclass' ident typeParams? '{' typeClassMember* '}' ;
typeClassMember : methodSignature ;

typeClassInstance : 'instance' typeExpr 'for' typeExpr '{' implMember* '}' ;

higherKindedTypeDecl : 'hkt' ident typeParams? blockExpr ;

typeProviderDecl : 'type_provider' ident blockExpr ;

fileScopedType : 'file_scope' typeExpr ';' ;

// ============================================================================
// EXTENSIONS
// ============================================================================

extensionMethodDecl : 'extension' typeExpr '{' methodDecl* '}' ;
extensionPropertyDecl : 'extension_property' ident ':' typeExpr blockExpr ;
extensionIndexerDecl : 'extension_indexer' typeExpr blockExpr ;
extensionOperatorDecl : 'extension_operator' ident blockExpr ;

macroDecl : 'macro' ident '(' params? ')' blockExpr ;

domainSpecificLanguageDecl : 'dsl' ident '{' dslRule* '}' ;
dslRule : ident '=>' expression ';' ;

aspectDecl : 'aspect' ident '{' aspectAdvice* '}' ;
aspectAdvice : 'before' | 'after' | 'around' ;

// ============================================================================
// PARALLELISM & MESSAGE PASSING
// ============================================================================

dataParallelismDecl : 'data_parallel' ident '{' parallelConfig* '}' ;
parallelConfig : ident '=' expression ';' ;

// Enhanced msgChannel with buffering
msgChannel : 'channel' ident ':' typeExpr ('buffered' '(' INT ')')? ';' ;

messagePassingDecl : 'message_passing' ident '{' msgChannel* '}' ;

// select/csp style primitives
selectExpr : 'select' '{' selectCase* '}' ;

selectCase : 'case' ( channelReceive | channelSend | 'default' ) '=>' blockExpr ','? ;

channelReceive : ident '<-' expression    // e.g., x <- ch
               | '<-' expression          // receive into discard
               ;
channelSend : expression '->' expression  // e.g., msg -> ch
            ;

// ============================================================================
// AI/ML FEATURES
// ============================================================================

deepLearningDecl : 'deep_learning' ident '{' dlLayer* '}' ;
dlLayer : 'layer' ident ':' typeExpr ';' ;

mlModelDecl : 'ml_model' ident '{' modelConfig* '}' ;
modelConfig : ident '=' expression ';' ;

quantumMlBlock : 'quantum_ml' '{' qmlComponent* '}' ;
qmlComponent : ident ':' typeExpr ';' ;

transferLearningBlock : 'transfer_learning' ident blockExpr ;
multiAgentBlock : 'multi_agent' ident '{' agentConfig* '}' ;
agentConfig : 'agent' ident ';' ;

autonomousSystemBlock : 'autonomous_system' ident blockExpr ;

explainableRlBlock : 'explainable_rl' ident blockExpr ;
explainableDeepLearningBlock : 'explainable_dl' ident blockExpr ;

probabilisticGraphicalModelBlock : 'pgm' ident '{' pgmNode* '}' ;
pgmNode : 'node' ident ':' typeExpr ';' ;

advancedNlpBlock : 'advanced_nlp' ident '{' nlpFeature* '}' ;
nlpFeature : ident ':' typeExpr ';' ;

aiForBusinessBlock : 'ai_business' ident blockExpr ;

// ============================================================================
// GRAPHICS & MULTIMEDIA
// ============================================================================

graphicsDecl : 'graphics' ident '{' graphicsConfig* '}' ;
graphicsConfig : ident '=' expression ';' ;

videoDecl : 'video' ident '{' videoConfig* '}' ;
videoConfig : ident '=' expression ';' ;

musicDecl : 'music' ident '{' musicConfig* '}' ;
musicConfig : ident '=' expression ';' ;

// ============================================================================
// SPECIALIZED DOMAINS
// ============================================================================

roboticsDecl : 'robotics' ident '{' roboticsComponent* '}' ;
roboticsComponent : ident ':' typeExpr ';' | methodDecl ;

tensorDecl : 'tensor' ident '<' typeExpr '>' '{' tensorConfig* '}' ;
tensorConfig : ident '=' expression ';' ;

matrixDecl : 'matrix' ident '<' typeExpr '>' '{' matrixConfig* '}' ;
matrixConfig : ident '=' expression ';' ;

vectorDecl : 'vector' ident '<' typeExpr '>' '{' vectorConfig* '}' ;
vectorConfig : ident '=' expression ';' ;

graphModelingBlock : 'graph_model' ident '{' graphConfig* '}' ;
graphConfig : ident ':' typeExpr ';' ;

vrArInteractionBlock : 'vr_ar' ident '{' vrArConfig* '}' ;
vrArConfig : ident '=' expression ';' ;

imageVideoAnalysisBlock : 'image_video_analysis' ident blockExpr ;

// ============================================================================
// CRYPTOGRAPHY
// ============================================================================

cryptoDecl : 'crypto' ident '{' cryptoAlgorithm* '}' ;
cryptoAlgorithm : 'algorithm' ident ';' ;

quantumIdentityDecl : 'quantum_identity' ident '{' identityConfig* '}' ;
identityConfig : ident ':' typeExpr ';' ;

provenanceManagerDecl : 'provenance_manager' ident '{' provenanceMethod* '}' ;
provenanceMethod : methodDecl ;

// ============================================================================
// META-PROGRAMMING
// ============================================================================

metaProgrammingDecl : 'meta_programming' '{' metaTransform* '}' ;
metaTransform : 'transform' STRING '=>' STRING ';' ;

languageDialectDecl : 'language_dialect' ident '{' dialectFeature* '}' ;
dialectFeature : 'keyword' STRING ';' ;

reflectionDecl : 'reflection' ident blockExpr ;

// ============================================================================
// MATHEMATICAL FOUNDATIONS
// ============================================================================

mathematicalDiscoveryDecl : 'math_discovery' ident '{' discoveryConfig* '}' ;
discoveryConfig : ident '=' expression ';' ;

conjectureProofDecl : 'conjecture' ident '{' proofConfig* '}' ;
proofConfig : 'statement' ':' typeExpr ';' ;

algebraicGeometryDecl : 'algebraic_geometry' ident blockExpr ;
differentialGeometryDecl : 'differential_geometry' ident blockExpr ;
categoryTheoryDecl : 'category_theory' ident blockExpr ;
numberTheoryDecl : 'number_theory' ident blockExpr ;

// ============================================================================
// HYBRID & INTERFACES
// ============================================================================

hybridDef : 'hybrid' ident '{' hybridMember* '}' ;
hybridMember : methodDecl | fieldDecl ;

interfaceDef : 'interface' ident '{' interfaceMember* '}' ;

// ============================================================================
// LITERALS & TOKENS
// ============================================================================

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
attributeDecl : '@' ident ('(' args? ')')? ;

// ============================================================================
// LEXER TOKENS
// ============================================================================

fragment LETTER : [a-zA-Z_] ;
fragment DIGIT : [0-9] ;
fragment HEX_DIGIT : [0-9a-fA-F] ;

// Keywords that should be recognized before IDENTIFIER
THIS : 'this' ;
SUPER : 'super' ;
NEW : 'new' ;

// Raw / multiline / byte array strings — place before generic STRING
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

