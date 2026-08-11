// Zamani.g4 — Comprehensive ANTLR4 combined grammar for the Zamani language.
grammar Zamani;

program : declaration* EOF ;

declaration
    : docComment? attributeDecl? ( moduleDecl
    | importDecl
    | exportDecl
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
    | quantumCircuitDecl
    | nanoAgentDecl
    | languageDecl
    | effectDecl
    | mtsDecl
    | sankofaDecl
    | agentDecl
    | cognitiveBlock
    | metaBlock
    | hdlModuleDecl
    | cloudDecl
    | distributedDecl
    | onDeviceAgentDecl
    | selfEvolveDecl
    | optPassDecl
    | targetPlatform
    | runtimeDecl
    | actorDecl
    | aiSystemDecl
    | agiSystemDecl
    | asiSystemDecl
    | aesiSystemDecl
    | asesiSystemDecl
    | adminInterfaceDecl
    | paymentGatewayDecl
    | userFeedbackDecl
    | copyrightNoticeDecl
    | tailorMadeFeatureDecl
    | programOnceDecl
    | maliciousIdeaDetection
    | userBlockingDecl
    | legalActionDecl
    | sandboxDecl
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
    | chatArchitectDecl
    | greenComputingAttr
    | thermalOptDecl
    | resourceConserveDecl
    | selfDiscoverDecl
    | developerAnalyticsDecl
    | licenseTrackingDecl
    | deploymentDecl
    | versionReleaseDecl
    | lspServerDecl
    | typeClassDecl
    | typeClassInstance
    | higherKindedTypeDecl
    | selfAdjustDecl
    | selfVersioningDecl
    | extensionMethodDecl
    | extensionPropertyDecl
    | extensionIndexerDecl
    | extensionOperatorDecl
    | macroDecl
    | domainSpecificLanguageDecl
    | aspectDecl
    | typeProviderDecl
    | dataParallelismDecl
    | concurrentDataStructureDecl
    | messageHandlerDecl
    | musicDecl
    | roboticsDecl
    | deepLearningDecl
    | graphicsDecl
    | videoDecl
    | tensorDecl
    | matrixDecl
    | vectorDecl
    | mlModelDecl
    | quantumMlBlock
    | explainableRlBlock
    | explainableDeepLearningBlock
    | knowledgeGraphBlock
    | probabilisticGraphicalModelBlock
    | transferLearningBlock
    | multiAgentBlock
    | autonomousSystemBlock
    | graphModelingBlock
    | advancedNlpBlock
    | cognitiveArchitectureBlock
    | aiForBusinessBlock
    | vrArInteractionBlock
    | imageVideoAnalysisBlock
    | fileScopedType
    | hybridDef
    | interfaceDef
    | statement
    ) ;

moduleDecl : 'module' ident ('::' ident)* (blockExpr | ';') ;
importDecl : 'import' modulePath ('as' ident)? ';'? ;
exportDecl : 'export' ident ('to' ident)? ';'? ;
modulePath : ident ('::' ident | '.' ident)* ;
useStmt : 'use' usePath ';'? ;
usePath : segment ('::' segment)* ('::' '*')? | segment ('::' segment)* '::' '{' ident (',' ident)* '}' ;
segment : ident ;
globalUsing : 'global' 'using' ident ';' ;

functionDecl : modifiers? 'fn' ident typeParams? '(' params? ')' ('->' typeExpr)? ('with' effectList)? blockExpr ;
params : param (',' param)* ;
param : 'mut'? ident (':' typeExpr)? ('=' expression)? | '...' typeExpr ident ;
modifiers : modifier+ ;
modifier : 'pub' | 'private' | 'protected' | 'static' | 'const' | 'async' | 'unsafe' | 'inline' | 'override' | 'final' | 'abstract' | 'virtual' | 'sealed' | 'partial' | 'file' | 'required' | 'init' ;

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
pattern : ident | literal | '_' | '(' pattern (',' pattern)* ')' | '[' pattern (',' pattern)* ']' | '[' pattern (',' pattern)* '...' pattern ']' | pattern '|' pattern | ident ':' typeExpr ;
unsafeBlock : 'unsafe' (ident? blockExpr | '!' '(' 'evas' ':' expression ')' blockExpr) ;
throwStmt : 'throw' expression ';' ;
tryCatchStmt : 'try' blockExpr catchClause* finallyClause? ;
catchClause : 'catch' ('(' param ')')? blockExpr ;
finallyClause : 'finally' blockExpr ;
blockExpr : '{' statement* '}' ;

letStmt : ('let' | 'var') 'mut'? ident (':' typeExpr)? '=' expression ';' ;
constStmt : 'const' ident (':' typeExpr)? '=' expression ';' ;

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
productExpr : castExpr (('*' | '/' | '%') castExpr)* ;
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
    | 'async' expression # asyncExpr
    | 'await' expression # awaitExpr
    | 'spawn' expression # spawnValExpr
    | 'new' ident typeArgs? '(' args? ')' # newExpr
    | 'try' expression catchClause* # tryCatchExpr
    | 'yield' expression? # yieldExpr
    | recallExpr # recallValExpr
    | learnExpr # learnValExpr
    | performExpr # performValExpr
    | zamaniExpr # zamaniValExpr
    | sasaExpr # sasaValExpr
    | quantumOpExpr # quantumOpValExpr
    | nanoExpr # nanoValExpr
    | mtsExpr # mtsValExpr
    | consensusExpr # consensusValExpr
    | ancestorCall # ancestorValExpr
    | mopExpr # mopValExpr
    | macroCall # macroCallExpr
    | 'super' ('.' ident | '(' args? ')')? # superExpr
    | 'this' # thisExpr
    | 'self' # selfExpr
    | interpolatedString # interpStringExpr
    ;

structLiteralTail : '{' (ident ':' expression ','?)* '}' ;
recallExpr : 'recall' ('(' expression ')' | expression) ;
learnExpr : ('learn' | 'infer') 'from'? expression ('with' 'weight' expression)? ;
performExpr : 'perform' expression ;
zamaniExpr : 'zamani' (blockExpr | expression) ;
sasaExpr : 'sasa' (blockExpr | expression) ;
quantumOpExpr 
    : 'quantum' ident ('(' args? ')')? 
    | 'superpose' '(' expression (',' expression)* ')' 
    | 'entangle' '(' ident ',' ident ')' 
    | 'measure' '(' args? ')'
    | 'reset' '(' args? ')'
    | 'barrier' '(' args? ')'
    ;
nanoExpr : nanoLit | 'assemble' '(' expression ')' | 'deploy' '(' expression ')' ;
mtsExpr : mtsLit | 'parallel' '(' blockExpr ')' | 'speculative' '(' blockExpr ')' | 'counterfactual' '(' expression ',' blockExpr ')' ;
consensusExpr : 'consensus' '[' exprList ']' 'vote' expression ;
ancestorCall : 'ancestral' ident '(' args? ')' ';'? ;
mopExpr : 'reflect' '(' expression ')' | 'introspect' '(' ident ')' | 'meta_eval' '(' expression ')' | 'quote' '{' statement* '}' | 'unquote' '(' expression ')' | 'splice' '(' expression ')' ;
macroCall : ident '!' '(' args? ')' ;
exprList : expression (',' expression)* ;
expressionStmt : expression ';' ;

typeExpr
    : baseType ('<' typeExpr (',' typeExpr)* '>')?
    | '(' ')'
    | '(' typeExpr ')'
    | '(' typeExpr (',' typeExpr)+ ')' ('->' typeExpr)?
    | 'fn' '(' (typeExpr (',' typeExpr)*)? ')' ('->' typeExpr)?
    | '&' 'mut'? '[' typeExpr ']'
    | '&' 'mut'? typeExpr
    | '*' 'mut'? typeExpr
    | '[' typeExpr (';' expression)? ']'
    | 'Self' | 'self'
    | typeExpr '?'
    | 'Box' '<' typeExpr '>'
    | 'linear' typeExpr
    | 'affine' typeExpr
    | 'session' '{' sessionOp* '}'
    | piType
    | sigmaType
    | identityType
    | 'Type_0' | 'Type_1' | 'Type_2' | 'Type_N'
    | 'Kind' | 'Sort' | 'Prop'
    | quantumType
    | nanoType
    | mtsType
    | sankofaType
    | cognitiveType
    | typeExpr 'with' 'effects' '{' effectName (',' effectName)* '}'
    | 'hkt' '<' typeParam '.' typeExpr '>'
    | 'exists' typeParam '.' typeExpr
    | 'singleton' typeExpr
    | typeExpr '.' ident
    ;

baseType : 'void' | 'int' | 'float' | 'bool' | 'string' | 'char' | 'bytes' | 'i8' | 'i16' | 'i32' | 'i64' | 'i128' | 'u8' | 'u16' | 'u32' | 'u64' | 'u128' | 'f32' | 'f64' | 'usize' | 'isize' | ident ;
typeParams : '<' typeParam (',' typeParam)* '>' ;
typeParam : ident (':' typeExpr)? ;
typeArgs : '<' typeExpr (',' typeExpr)* '>' ;

structDecl : modifiers? 'struct' ident typeParams? '{' structField* '}' ;
structField : modifiers? ident ':' typeExpr (',' | ';')? ;
enumDecl : modifiers? 'enum' ident typeParams? '{' enumVariant* '}' ;
enumVariant : ident ('(' typeExpr (',' typeExpr)* ')')? (',' | ';')? ;
traitDecl : modifiers? 'trait' ident typeParams? ('where' whereClause)? '{' traitItem* '}' ;
traitItem : functionDecl | typeAliasDecl | constDecl ;
implDecl : 'impl' typeParams? (ident 'for')? typeExpr ('where' whereClause)? '{' implItem* '}' ;
implItem : functionDecl | typeAliasDecl | constDecl ;
typeAliasDecl : modifiers? 'type' ident typeParams? '=' typeExpr ';' ;
constDecl : modifiers? 'const' ident ':' typeExpr '=' expression ';' ;
classDecl : modifiers? 'class' ident typeParams? ('extends' typeExpr)? ('implements' typeExpr (',' typeExpr)*)? '{' classItem* '}' ;
classItem : modifiers? (functionDecl | structField | constructorDecl | destructorDecl) ;
constructorDecl : 'init' '(' params? ')' blockExpr ;
destructorDecl : 'deinit' '(' ')' blockExpr ;
interfaceDecl : modifiers? 'interface' ident typeParams? '{' traitItem* '}' ;
recordDecl : modifiers? 'record' ident typeParams? '(' params? ')' ('->' typeExpr)? (blockExpr | ';') ;

quantumCircuitDecl : 'quantum' 'circuit' ident '(' params? ')' blockExpr ;
nanoAgentDecl : 'nano' 'agent' ident '(' params? ')' blockExpr ;
languageDecl : 'language' ident '{' statement* '}' ;
effectDecl : 'effect' ident '{' effectOp* '}' ;
effectOp : 'op' ident '(' params? ')' '->' typeExpr ';' ;
effectList : effectName (',' effectName)* ;
effectName : ident ;
mtsDecl : 'mts' ident '{' statement* '}' ;
sankofaDecl : 'sankofa' ident '{' statement* '}' ;
agentDecl : 'agent' ident '{' (agentCapability | agentBehavior)* '}' ;
cognitiveBlock : 'cognitive' ident '{' statement* '}' ;
metaBlock : 'meta' ident '{' statement* '}' ;
hdlModuleDecl : 'hdl' 'module' ident '(' params? ')' '{' statement* '}' ;
cloudDecl : 'cloud' ident '{' statement* '}' ;
distributedDecl : 'distributed' ident '{' statement* '}' ;
onDeviceAgentDecl : 'on_device' 'agent' ident '{' statement* '}' ;
selfEvolveDecl : 'self_evolve' ident '{' statement* '}' ;
optPassDecl : 'opt_pass' ident '{' statement* '}' ;
targetPlatform : 'target_platform' ident '{' statement* '}' ;
runtimeDecl : 'runtime' ident '{' statement* '}' ;
actorDecl : 'actor' ident '{' statement* '}' ;
aiSystemDecl : 'ai' 'system' ident '{' statement* '}' ;
agiSystemDecl : 'agi' 'system' ident '{' statement* '}' ;
asiSystemDecl : 'asi' 'system' ident '{' statement* '}' ;
aesiSystemDecl : 'aesi' 'system' ident '{' statement* '}' ;
asesiSystemDecl : 'asesi' 'system' ident '{' statement* '}' ;
adminInterfaceDecl : 'admin' 'interface' ident '{' statement* '}' ;
paymentGatewayDecl : 'payment' 'gateway' ident '{' statement* '}' ;
userFeedbackDecl : 'user' 'feedback' ident '{' statement* '}' ;
copyrightNoticeDecl : 'copyright' 'notice' ident '{' statement* '}' ;
tailorMadeFeatureDecl : 'feature' ident '{' statement* '}' ;
programOnceDecl : 'program_once' ident '{' statement* '}' ;
maliciousIdeaDetection : 'malicious' 'idea' 'detection' '{' statement* '}' ;
userBlockingDecl : 'block' 'user' ident '{' statement* '}' ;
legalActionDecl : 'legal' 'action' ident '{' statement* '}' ;
sandboxDecl : 'sandbox' ident '{' statement* '}' ;
omniversalSimulationDecl : 'omniversal' 'simulate' ident '{' statement* '}' ;
omniversalCodeSynthDecl : 'omniversal' 'synthesize' ident '{' statement* '}' ;
omniversalDeployDecl : 'omniversal' 'deploy' ident '{' statement* '}' ;
omniversalAlignmentDecl : 'omniversal' 'alignment' ident '{' statement* '}' ;
omniversalContainmentDecl : 'omniversal' 'containment' ident '{' statement* '}' ;
omniversalTrustDecl : 'omniversal' 'trust' ident '{' statement* '}' ;
omniversalKnowledgeDecl : 'omniversal' 'knowledge' ident '{' statement* '}' ;
omniversalGenerativeDecl : 'omniversal' 'generate' ident '{' statement* '}' ;
omniversalSovereigntyDecl : 'omniversal' 'sovereignty' ident '{' statement* '}' ;
omniversalGoalDecl : 'omniversal' 'goal' ident '{' statement* '}' ;
omniversalBioNanoDecl : 'omniversal' 'bionano' ident '{' statement* '}' ;
omniversalRealityDecl : 'omniversal' 'reality' ident '{' statement* '}' ;
omniversalNlpDecl : 'omniversal' 'nlp' ident '{' statement* '}' ;
chatArchitectDecl : 'chat' 'architect' ident '{' statement* '}' ;
greenComputingAttr : 'green' ident ;
thermalOptDecl : 'thermal' 'optimize' ident '{' statement* '}' ;
resourceConserveDecl : 'resource' 'conserve' ident '{' statement* '}' ;
selfDiscoverDecl : 'self' 'discover' ident '{' statement* '}' ;
developerAnalyticsDecl : 'developer' 'analytics' ident '{' statement* '}' ;
licenseTrackingDecl : 'license' 'tracking' ident '{' statement* '}' ;
deploymentDecl : 'deployment' ident '{' statement* '}' ;
versionReleaseDecl : 'version' 'release' ident '{' statement* '}' ;
lspServerDecl : 'lsp' 'server' ident '{' statement* '}' ;
typeClassDecl : 'typeclass' ident '{' statement* '}' ;
typeClassInstance : 'instance' ident 'for' typeExpr '{' statement* '}' ;
higherKindedTypeDecl : 'hkt' ident '{' statement* '}' ;
selfAdjustDecl : 'self_adjust' ident '{' statement* '}' ;
selfVersioningDecl : 'self_versioning' ident '{' statement* '}' ;
extensionMethodDecl : 'extension' 'method' ident '{' statement* '}' ;
extensionPropertyDecl : 'extension' 'property' ident '{' statement* '}' ;
extensionIndexerDecl : 'extension' 'indexer' ident '{' statement* '}' ;
extensionOperatorDecl : 'extension' 'operator' ident '{' statement* '}' ;
macroDecl : 'macro' ident '{' statement* '}' ;
domainSpecificLanguageDecl : 'dsl' ident '{' statement* '}' ;
aspectDecl : 'aspect' ident '{' statement* '}' ;
typeProviderDecl : 'type_provider' ident '{' statement* '}' ;
dataParallelismDecl : 'data' 'parallelism' ident '{' statement* '}' ;
concurrentDataStructureDecl : 'concurrent' 'data' 'structure' ident '{' statement* '}' ;
messageHandlerDecl : 'message' 'handler' ident '{' statement* '}' ;
musicDecl : 'music' ident '{' statement* '}' ;
roboticsDecl : 'robotics' ident '{' statement* '}' ;
deepLearningDecl : 'deep_learning' ident '{' statement* '}' ;
graphicsDecl : 'graphics' ident '{' statement* '}' ;
videoDecl : 'video' ident '{' statement* '}' ;
tensorDecl : 'tensor' ident '{' statement* '}' ;
matrixDecl : 'matrix' ident '{' statement* '}' ;
vectorDecl : 'vector' ident '{' statement* '}' ;
mlModelDecl : 'ml' 'model' ident '{' statement* '}' ;
quantumMlBlock : 'quantum_ml' ident '{' statement* '}' ;
explainableRlBlock : 'explainable_rl' ident '{' statement* '}' ;
explainableDeepLearningBlock : 'explainable_deep_learning' ident '{' statement* '}' ;
knowledgeGraphBlock : 'knowledge_graph' ident '{' statement* '}' ;
probabilisticGraphicalModelBlock : 'probabilistic_graphical_model' ident '{' statement* '}' ;
transferLearningBlock : 'transfer_learning' ident '{' statement* '}' ;
multiAgentBlock : 'multi_agent' ident '{' statement* '}' ;
autonomousSystemBlock : 'autonomous_system' ident '{' statement* '}' ;
graphModelingBlock : 'graph_modeling' ident '{' statement* '}' ;
advancedNlpBlock : 'advanced_nlp' ident '{' statement* '}' ;
cognitiveArchitectureBlock : 'cognitive_architecture' ident '{' statement* '}' ;
aiForBusinessBlock : 'ai_for_business' ident '{' statement* '}' ;
vrArInteractionBlock : 'vr_ar_interaction' ident '{' statement* '}' ;
imageVideoAnalysisBlock : 'image_video_analysis' ident '{' statement* '}' ;
fileScopedType : 'file' 'scoped' 'type' ident '{' statement* '}' ;
hybridDef : 'hybrid' ident '{' statement* '}' ;
interfaceDef : 'interface' ident '{' statement* '}' ;
agentCapability : 'capability' ident '{' statement* '}' ;
agentBehavior : 'behavior' ident '{' statement* '}' ;
explainStmt : 'explain' expression ';' ;
transparentStmt : 'transparent' expression ';' ;
asiCapabilityDef : 'asi' 'capability' ident '{' statement* '}' ;
aesiCapabilityDef : 'aesi' 'capability' ident '{' statement* '}' ;
asesiCapabilityDef : 'asesi' 'capability' ident '{' statement* '}' ;

docComment : DOC_COMMENT+ ;
attributeDecl : '#[' ident (',' ident)* ']' ;
ident : IDENT | THIS | SELF_LOWER | SELF_UPPER | INT_KW | FLOAT_KW | BOOL_KW | STR_KW | STRING_KW | CHAR_KW | VOID | QUANTUM | NANO | AGENT | CIRCUIT | EFFECT | HANDLE | REMEMBER | RECALL | LEARN | INFER | WISDOM | ZAMANI | SASA | ANCESTOR | LINEAR | AFFINE | LANGUAGE | MTS_KW | LEN | PRINT | PRINTLN | ASSERT | PANIC ;
statement : letStmt | constStmt | returnStmt | breakStmt | continueStmt | throwStmt | expressionStmt | ifExpr | matchStmt | whileStmt | forStmt | loopExpr | unsafeBlock | tryCatchStmt | blockExpr ;
identList : ident (',' ident)* ;
ORBITAL : 's' | 'p' | 'd' | 'f' ;
FORMULA : STRING ;
OPERATOR : '+' | '-' | '*' | '/' | '%' | '==' | '!=' | '<' | '<=' | '>' | '>=' | '&&' | '||' | '!' | '&' | '|' | '^' | '<<' | '>>' | '>>>' | '~' ;

whereClause : typeExpr (':' typeExpr)? (',' typeExpr (':' typeExpr)?)* ;
literal : INTEGER | FLOAT | STRING | CHAR | BOOLEAN | NIL | quantumLit | nanoLit | mtsLit | rawStringLit | utf8StringLit ;
quantumLit : '|' ('0' | '1' | '+' | '-' | ident) '\u27E9' ;
nanoLit : '@atom' '(' ident ':' ORBITAL ')' | '@molecule' '(' FORMULA ')' ;
mtsLit : 'mts' '[' STRING ']' ;
rawStringLit : 'r' STRING ;
utf8StringLit : 'u8' STRING ;
interpolatedString : 'f' STRING ;
piType : 'Pi' '(' ident ':' typeExpr ')' '.' typeExpr ;
sigmaType : 'Sigma' '(' ident ':' typeExpr ')' '.' typeExpr ;
identityType : 'Id' '(' typeExpr ',' expression ',' expression ')' ;
sessionOp : 'send' typeExpr | 'recv' typeExpr | 'offer' '{' sessionBranch* '}' | 'choice' '{' sessionBranch* '}' | 'close' ;
sessionBranch : ident '->' typeExpr ;
quantumType : 'Qubit' | 'QReg' '[' expression ']' | 'Superposition' '<' typeExpr '>' | 'Entangled' '<' typeExpr ',' typeExpr '>' ;
nanoType : 'Atom' | 'Molecule' | 'NanoAgent' ;
mtsType : 'Mts' | 'Parallel' | 'Speculative' ;
sankofaType : 'Past' | 'Present' | 'Future' ;
cognitiveType : 'CognitiveState' | 'Consciousness' | 'Neural' ;

// Lexer Rules
MODULE: 'module'; IMPORT: 'import'; EXPORT: 'export'; AS: 'as'; USE: 'use'; GLOBAL: 'global'; USING: 'using';
FN: 'fn'; PUB: 'pub'; PRIVATE: 'private'; PROTECTED: 'protected'; STATIC: 'static'; CONST: 'const'; ASYNC: 'async';
UNSAFE: 'unsafe'; INLINE: 'inline'; OVERRIDE: 'override'; FINAL: 'final'; ABSTRACT: 'abstract'; VIRTUAL: 'virtual';
SEALED: 'sealed'; PARTIAL: 'partial'; FILE: 'file'; REQUIRED: 'required'; INIT: 'init';
RETURN: 'return'; BREAK: 'break'; CONTINUE: 'continue'; WHILE: 'while'; FOR: 'for'; IN: 'in'; LOOP: 'loop';
IF: 'if'; ELSE: 'else'; MATCH: 'match'; CASE: 'case'; WHEN: 'when'; THROW: 'throw'; TRY: 'try'; CATCH: 'catch'; FINALLY: 'finally';
LET: 'let'; VAR: 'var'; TYPE: 'type'; STRUCT: 'struct'; ENUM: 'enum'; TRAIT: 'trait'; IMPL: 'impl'; CLASS: 'class';
INTERFACE: 'interface'; RECORD: 'record'; QUANTUM: 'quantum'; CIRCUIT: 'circuit'; MEASURE: 'measure'; RESET: 'reset'; BARRIER: 'barrier';
THIS: 'this'; SELF_LOWER: 'self'; SELF_UPPER: 'Self'; INT_KW: 'int'; FLOAT_KW: 'float'; BOOL_KW: 'bool'; STR_KW: 'string';
STRING_KW: 'String'; CHAR_KW: 'char'; VOID: 'void'; NANO: 'nano'; AGENT: 'agent'; EFFECT: 'effect'; HANDLE: 'handle';
REMEMBER: 'remember'; RECALL: 'recall'; LEARN: 'learn'; INFER: 'infer'; WISDOM: 'wisdom'; ZAMANI: 'zamani';
SASA: 'sasa'; ANCESTOR: 'ancestral'; LINEAR: 'linear'; AFFINE: 'affine'; LANGUAGE: 'language'; MTS_KW: 'mts';
LEN: 'len'; PRINT: 'print'; PRINTLN: 'println'; ASSERT: 'assert'; PANIC: 'panic';
TRUE: 'true'; FALSE: 'false'; NIL_KW: 'nil'; NULL_KW: 'null';

BOOLEAN: TRUE | FALSE;
NIL: NIL_KW | NULL_KW;
INTEGER: DIGIT+ | '0x' HEX_DIGIT+ | '0b' BIN_DIGIT+ | '0o' OCT_DIGIT+;
FLOAT: DIGIT+ '.' DIGIT+ ('e' ('+' | '-')? DIGIT+)?;
STRING: '"' (ESC | ~["\\])* '"';
CHAR: '\'' (ESC | ~['\\]) '\'';

QUANTUM_LITERAL: '|' ('0' | '1' | '+' | '-') '\u27E9';
NANO_ANNOTATION: '@' IDENT ('(' ~[)]* ')')?;
MTS_LITERAL: 'mts' '[' ~[\]]* ']';

IDENT: ALPHA (ALPHA | DIGIT)*;

LPAREN: '('; RPAREN: ')'; LBRACE: '{'; RBRACE: '}'; LBRACK: '['; RBRACK: ']';
COMMA: ','; DOT: '.'; SEMI: ';'; COLON: ':'; ARROW: '->'; FATARROW: '=>';
COLONCOLON: '::'; TILDE: '~'; HASH: '#'; AT: '@'; BANG: '!';
PLUS: '+'; MINUS: '-'; STAR: '*'; SLASH: '/'; PERCENT: '%'; ASSIGN: '=';
EQ: '=='; NEQ: '!='; LT: '<'; GT: '>'; LE: '<='; GE: '>=';
ANDAND: '&&'; OROR: '||'; AMP: '&'; PIPE: '|'; CARET: '^';
SHL: '<<'; SHR: '>>'; SHRU: '>>>';
PLUSEQ: '+='; MINUSEQ: '-='; STAREQ: '*='; SLASHEQ: '/=';
DOTDOT: '..'; DOTDOTEQ: '..=';
QUESTION: '?'; DOLLAR: '$';

LINE_COMMENT: '//' ~[\r\n]* -> skip;
BLOCK_COMMENT: '/*' .*? '*/' -> skip;
DOC_COMMENT: '///' ~[\r\n]*;
WS: [ \t\r\n]+ -> skip;

fragment ALPHA: [a-zA-Z_];
fragment DIGIT: [0-9];
fragment HEX_DIGIT: [0-9a-fA-F];
fragment BIN_DIGIT: [01];
fragment OCT_DIGIT: [0-7];
fragment ESC: '\\' [nrt0"'\\];
