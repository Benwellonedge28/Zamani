// Zamani.g4 — Comprehensive high-performance mathematical programming language
// COMPLETE SPECIFICATION with full mathematics support:
// - Linear algebra, matrix operations, tensor computing
// - Symbolic mathematics, calculus, differential equations
// - Statistical distributions, probability, stochastic processes
// - Complex numbers, polynomial arithmetic, number theory
// - Numeric methods, FFT, signal processing, optimization
// - Quantum-enhanced mathematics, nano-computational primitives

grammar Zamani;

program : docComment? declaration* EOF ;

// ==========================================================================
// DECLARATIONS
// ==========================================================================

declaration
    : moduleDecl
    | importDecl
    | exportDecl
    | functionDecl
    | structDecl
    | enumDecl
    | traitDecl
    | implDecl
    | classDecl
    | interfaceDecl
    | recordDecl
    | typeAlias
    | constDecl
    | quantumCircuitDecl
    | nanoAgentDecl
    | languageDecl
    | effectDecl
    | globalUsing
    | attributeDecl
    | hdlModuleDecl
    | pluginDecl
    | macroDecl
    | packageDecl
    | mathVectorDecl
    | mathMatrixDecl
    | mathTensorDecl
    | mathSymbolicDecl
    | statement
    ;

// Package Manifest
packageDecl: 'package' IDENTIFIER '{' packageField* '}' ;
packageField
    : 'version' ':' STRING ';'
    | 'depends' ':' '[' STRING (',' STRING)* ']' ';'
    | 'repository' ':' STRING ';'
    | 'license' ':' STRING ';'
    ;

// ==========================================================================
// DOCUMENTATION & ANNOTATIONS
// ==========================================================================

docComment: (DOC_COMMENT)+ ;

attributeDecl: annotation+ declaration ;

annotation: '@' IDENTIFIER ('(' annotationValue? ')')? ;

annotationValue
    : literal
    | IDENTIFIER
    | annotation
    | annotationValue ',' annotationValue
    ;

// ==========================================================================
// MODULE SYSTEM
// ==========================================================================

moduleDecl: 'module' IDENTIFIER ('::' IDENTIFIER)* ('{' declaration* '}')? ';' ;

importDecl
    : 'import' IDENTIFIER ';'
    | 'import' IDENTIFIER 'as' IDENTIFIER ';'
    | 'import' '{' importList '}' 'from' STRING ';'
    | 'import' '*' 'as' IDENTIFIER 'from' STRING ';'
    ;

importList: IDENTIFIER (',' IDENTIFIER)* ;

exportDecl
    : 'export' IDENTIFIER ('to' IDENTIFIER)? ';'
    | 'export' '{' exportList '}' ';'
    | 'export' '*' 'from' STRING ';'
    ;

exportList: IDENTIFIER ('as' IDENTIFIER)? (',' IDENTIFIER ('as' IDENTIFIER)?)* ;

globalUsing: 'global' 'using' IDENTIFIER ';' ;

usingDirective: 'using' IDENTIFIER ';' ;

// ==========================================================================
// VISIBILITY & MODIFIERS
// ==========================================================================

visibilityModifier: 'public' | 'private' | 'protected' | 'internal' ;

modifier
    : 'pub' | 'private' | 'protected' | 'static' | 'const' | 'async'
    | 'unsafe' | 'inline' | 'override' | 'final' | 'abstract'
    | 'mut' | 'sealed' | 'partial' | 'extern' | 'volatile'
    | 'simd' | 'vectorized' | 'gpu' | 'parallel' | 'pure' | 'immutable'
    ;

modifiers: modifier+ ;

// ==========================================================================
// FUNCTION DECLARATIONS
// ==========================================================================

functionDecl
    : visibilityModifier? modifier* 'fn' IDENTIFIER genericParameters? '(' parameterList? ')'
      ('->' typeExpr)? effectClause? contractClause? block
    ;

contractClause: 'contract' IDENTIFIER ('{' contractBody '}')?;

contractBody
    : ('requires' '(' expression ')' ';')*
    ('ensures' '(' expression ')' ';')*
    ('invariant' '(' expression ')' ';')*
    ;

effectClause: 'with' 'effects' '{' effectName (',' effectName)* '}' ;

effectName: IDENTIFIER ;

lambdaExpression: '|' parameterList? '|' ('->' typeExpr)? block ;

asyncFunctionDecl: 'async' 'fn' IDENTIFIER genericParameters? '(' parameterList? ')'
                   ('->' typeExpr)? block ;

// ==========================================================================
// VARIABLE DECLARATIONS
// ==========================================================================

variableDeclaration
    : varKeyword IDENTIFIER (':' typeExpr)? '=' expression ';'
    | varKeyword IDENTIFIER ':' typeExpr ';'
    ;

varKeyword: 'let' | 'const' | 'var' ;

constDecl: 'const' IDENTIFIER ':' typeExpr '=' expression ';' ;

// ==========================================================================
// CONTROL FLOW STATEMENTS
// ==========================================================================

statement
    : variableDeclaration
    | conditionalStatement
    | loopStatement
    | tryCatchFinally
    | matchStatement
    | throwStatement
    | returnStatement
    | breakStatement
    | continueStatement
    | blockStatement
    | unsafeBlock
    | effectHandleStmt
    | quantumStmt
    | nanoStmt
    | mtsStmt
    | sankofaStmt
    | invokeStmt
    | foreignFunctionCall
    | dataStmt
    | databaseOp
    | webService
    | mathStmt
    | expression ';'
    | ';'
    ;

blockStatement: block ;

conditionalStatement
    : 'if' expression block ('else' 'if' expression block)* ('else' block)?
    ;

loopStatement
    : 'while' expression block
    | 'do' block 'while' expression ';'
    | 'for' IDENTIFIER 'in' expression block
    | 'for' '(' varKeyword IDENTIFIER ':' typeExpr '=' expression ';' expression ';' expression ')' block
    | 'forall' IDENTIFIER 'in' expression 'when' expression block  // Quantified loop
    | 'foreach' IDENTIFIER 'in' expression 'parallel' block         // Parallel loop
    | 'reduce' IDENTIFIER 'in' expression 'with' expression block   // Reduction loop
    ;

loopControl: 'break' ';' | 'continue' ';' ;

breakStatement: 'break' ';' ;

continueStatement: 'continue' ';' ;

returnStatement: 'return' expression? ';' ;

throwStatement: 'throw' expression ';' ;

matchStatement: 'match' expression '{' matchCase+ '}' ;

matchCase: 'case' pattern ('when' expression)? '=>' (expression | block) ;

pattern
    : literal
    | IDENTIFIER
    | '_'
    | tuplePattern
    | arrayPattern
    | listPattern
    | orPattern
    | typePattern
    ;

tuplePattern: '(' pattern (',' pattern)+ ')' ;

arrayPattern: '[' pattern (',' pattern)* ']' ;

listPattern: '[' pattern (',' pattern)* ']' ('...' pattern)? ;

orPattern: pattern '|' pattern ;

typePattern: IDENTIFIER ':' typeExpr ;

unsafeBlock: 'unsafe' ('!' | block | '(' 'evas' ':' expression ')' block) ;

tryCatchFinally
    : 'try' block catchClause* finallyClause?
    ;

catchClause: 'catch' '(' IDENTIFIER (':' typeExpr)? ')' block ;

finallyClause: 'finally' block ;

block: '{' statement* '}' ;

// ==========================================================================
// MATHEMATICAL STATEMENTS & OPERATIONS
// ==========================================================================

mathStmt
    : vectorOp
    | matrixOp
    | tensorOp
    | symbolicOp
    | calcOp
    | statisticalOp
    | numericOp
    | signalOp
    | optimizationStmt
    ;

// Vector Operations
vectorOp
    : 'vectorize' IDENTIFIER '(' argumentList? ')' ';'
    | 'map' lambdaExpression 'over' IDENTIFIER ';'
    | 'zip' IDENTIFIER 'with' IDENTIFIER 'using' lambdaExpression ';'
    | 'reduce' IDENTIFIER 'with' lambdaExpression ';'
    | 'scan' IDENTIFIER 'with' lambdaExpression ';'
    | 'filter' IDENTIFIER 'by' lambdaExpression ';'
    ;

// Matrix Operations
matrixOp
    : 'transpose' IDENTIFIER ';'
    | 'inverse' IDENTIFIER ';'
    | 'determinant' IDENTIFIER ';'
    | 'rank' IDENTIFIER ';'
    | 'eigenvalues' IDENTIFIER ';'
    | 'eigenvectors' IDENTIFIER ';'
    | 'svd' IDENTIFIER ';'
    | 'qr' IDENTIFIER ';'
    | 'lu' IDENTIFIER ';'
    | 'cholesky' IDENTIFIER ';'
    | 'norm' IDENTIFIER ('(' STRING ')')? ';'
    | 'trace' IDENTIFIER ';'
    | 'lu_solve' IDENTIFIER ',' IDENTIFIER ';'
    | 'qr_solve' IDENTIFIER ',' IDENTIFIER ';'
    ;

// Tensor Operations
tensorOp
    : 'contract' IDENTIFIER (',' IDENTIFIER)+ ('over' IDENTIFIER (',' IDENTIFIER)*)? ';'
    | 'einsum' STRING ('with' IDENTIFIER (',' IDENTIFIER)*)? ';'
    | 'reshape' IDENTIFIER 'to' INTEGER (',' INTEGER)* ';'
    | 'permute' IDENTIFIER 'axes' INTEGER (',' INTEGER)* ';'
    | 'outer_product' IDENTIFIER ',' IDENTIFIER ';'
    | 'inner_product' IDENTIFIER ',' IDENTIFIER ';'
    | 'kronecker' IDENTIFIER ',' IDENTIFIER ';'
    | 'hadamard' IDENTIFIER (',' IDENTIFIER)+ ';'  // Element-wise product
    ;

// Symbolic Operations
symbolicOp
    : 'expand' expression ';'
    | 'simplify' expression ';'
    | 'factor' expression ';'
    | 'collect' expression 'by' IDENTIFIER ';'
    | 'substitute' IDENTIFIER 'with' expression 'in' expression ';'
    | 'solve' expression ('for' IDENTIFIER)? ';'
    | 'dsolve' expression ('for' IDENTIFIER)? ';'  // Differential equations
    | 'series' expression 'around' expression 'order' INTEGER ';'  // Taylor series
    | 'roots' expression ';'
    | 'resultant' expression ',' expression ('wrt' IDENTIFIER)? ';'
    | 'gcd' expression ',' expression ';'
    | 'lcm' expression ',' expression ';'
    ;

// Calculus Operations
calcOp
    : 'diff' expression ('wrt' IDENTIFIER)? ';'
    | 'integral' expression ('from' expression 'to' expression)? ';'
    | 'limit' expression 'as' IDENTIFIER '->' expression ';'
    | 'gradient' IDENTIFIER ';'
    | 'jacobian' IDENTIFIER ';'
    | 'hessian' IDENTIFIER ';'
    | 'laplacian' IDENTIFIER ';'
    | 'divergence' IDENTIFIER ';'
    | 'curl' IDENTIFIER ';'
    | 'directional_derivative' IDENTIFIER 'in' IDENTIFIER ';'
    ;

// Statistical Operations
statisticalOp
    : 'mean' IDENTIFIER ';'
    | 'median' IDENTIFIER ';'
    | 'mode' IDENTIFIER ';'
    | 'variance' IDENTIFIER ';'
    | 'std_dev' IDENTIFIER ';'
    | 'covariance' IDENTIFIER ',' IDENTIFIER ';'
    | 'correlation' IDENTIFIER ',' IDENTIFIER ';'
    | 'quantile' IDENTIFIER ',' DECIMAL ';'
    | 'histogram' IDENTIFIER 'bins' INTEGER ';'
    | 'pdf' IDENTIFIER 'dist' IDENTIFIER ';'
    | 'cdf' IDENTIFIER 'dist' IDENTIFIER ';'
    | 'sample' 'from' IDENTIFIER 'size' INTEGER ';'
    | 'hypothesis_test' IDENTIFIER ',' IDENTIFIER 'test' STRING ';'
    | 'anova' IDENTIFIER (',' IDENTIFIER)+ ';'
    | 'regression' 'y' IDENTIFIER 'x' IDENTIFIER (',' IDENTIFIER)* ';'
    | 'pca' IDENTIFIER 'components' INTEGER ';'
    ;

// Numeric Operations
numericOp
    : 'fft' IDENTIFIER ';'
    | 'ifft' IDENTIFIER ';'
    | 'rfft' IDENTIFIER ';'
    | 'irfft' IDENTIFIER ';'
    | 'convolve' IDENTIFIER ',' IDENTIFIER ';'
    | 'correlate' IDENTIFIER ',' IDENTIFIER ';'
    | 'interpolate' IDENTIFIER 'kind' STRING ';'
    | 'differentiate' IDENTIFIER ('order' INTEGER)? ';'
    | 'integrate' IDENTIFIER 'from' expression 'to' expression ';'
    | 'quad' IDENTIFIER 'from' expression 'to' expression ';'
    | 'quad_log' IDENTIFIER 'from' expression 'to' expression ';'
    | 'quad_oscillatory' IDENTIFIER 'from' expression 'to' expression ';'
    ;

// Signal Processing
signalOp
    : 'butter' 'order' INTEGER 'wn' DECIMAL ('btype' STRING)? ';'
    | 'cheby1' 'order' INTEGER 'rp' DECIMAL 'wn' DECIMAL ';'
    | 'cheby2' 'order' INTEGER 'rs' DECIMAL 'wn' DECIMAL ';'
    | 'bessel' 'order' INTEGER 'wn' DECIMAL ';'
    | 'ellip' 'order' INTEGER 'rp' DECIMAL 'rs' DECIMAL 'wn' DECIMAL ';'
    | 'filter' IDENTIFIER 'signal' IDENTIFIER ';'
    | 'welch' IDENTIFIER 'nperseg' INTEGER ';'
    | 'spectrogram' IDENTIFIER ';'
    ;

// Optimization Operations
optimizationStmt
    : 'minimize' expression ('subject' 'to' constraintList)? ('method' STRING)? ';'
    | 'maximize' expression ('subject' 'to' constraintList)? ('method' STRING)? ';'
    | 'gradient_descent' IDENTIFIER 'rate' DECIMAL 'iterations' INTEGER ';'
    | 'newton_method' IDENTIFIER 'tolerance' DECIMAL ';'
    | 'bisect' IDENTIFIER 'from' expression 'to' expression ';'
    | 'secant' IDENTIFIER 'x0' expression 'x1' expression ';'
    | 'brent' IDENTIFIER 'from' expression 'to' expression ';'
    | 'linear_solve' IDENTIFIER ',' IDENTIFIER ';'
    | 'least_squares' IDENTIFIER ',' IDENTIFIER ';'
    | 'nonlinear_solve' expression ('initial' IDENTIFIER)? ';'
    ;

constraintList: constraintExpr (',' constraintExpr)* ;

constraintExpr
    : expression ('<=' | '>=' | '==' | '<' | '>') expression
    ;

// ==========================================================================
// MATHEMATICAL TYPE DECLARATIONS
// ==========================================================================

mathVectorDecl: 'vector' IDENTIFIER '<' typeExpr (',' INTEGER)? '>' ('=' vectorInit)? ';' ;

vectorInit
    : '[' expression (',' expression)* ']'
    | 'zeros' '(' INTEGER ')'
    | 'ones' '(' INTEGER ')'
    | 'range' '(' expression ',' expression (',' expression)? ')'
    | 'linspace' '(' expression ',' expression ',' INTEGER ')'
    | 'logspace' '(' expression ',' expression ',' INTEGER ')'
    | 'random' '(' INTEGER ')'
    | 'normal' '(' INTEGER ',' expression ',' expression ')'
    ;

mathMatrixDecl: 'matrix' IDENTIFIER '<' typeExpr (',' INTEGER ',' INTEGER)? '>' ('=' matrixInit)? ';' ;

matrixInit
    : '[[' (expression (',' expression)*)? (',' '[' (expression (',' expression)*)? ']')* ']]'
    | 'zeros' '(' INTEGER ',' INTEGER ')'
    | 'ones' '(' INTEGER ',' INTEGER ')'
    | 'eye' '(' INTEGER (',' INTEGER)? ')'
    | 'diag' '(' expression (',' INTEGER)? ')'
    | 'random' '(' INTEGER ',' INTEGER ')'
    | 'normal' '(' INTEGER ',' INTEGER ',' expression ',' expression ')'
    | 'uniform' '(' INTEGER ',' INTEGER ',' expression ',' expression ')'
    ;

mathTensorDecl: 'tensor' IDENTIFIER '<' typeExpr (',' INTEGER)+ '>' ('=' tensorInit)? ';' ;

tensorInit
    : 'zeros' '(' INTEGER (',' INTEGER)* ')'
    | 'ones' '(' INTEGER (',' INTEGER)* ')'
    | 'random' '(' INTEGER (',' INTEGER)* ')'
    | 'normal' '(' INTEGER (',' INTEGER)* ',' expression ',' expression ')'
    ;

mathSymbolicDecl: 'sym' IDENTIFIER ('=' expression)? ';' ;

// ==========================================================================
// CLASS & OOP FEATURES
// ==========================================================================

classDecl
    : modifier* 'class' IDENTIFIER genericParameters? extendsClause? implementsClause?
      permitsClause? '{' classMember* '}'
    ;

extendsClause: 'extends' IDENTIFIER (',' IDENTIFIER)* ;

implementsClause: 'implements' IDENTIFIER (',' IDENTIFIER)* ;

permitsClause: 'permits' IDENTIFIER (',' IDENTIFIER)* ;

classMember
    : propertyDef
    | methodDef
    | constructorDef
    | destructorDef
    | staticPropertyDef
    | staticMethodDef
    | innerClassDef
    | eventDef
    | indexerDef
    | operatorOverload
    | delegateDef
    | initBlock
    | staticBlock
    ;

propertyDef
    : visibilityModifier? typeExpr IDENTIFIER ('{' propertyAccessor* '}' | '=' expression)? ';'
    ;

propertyAccessor
    : 'get' ('(' ')' ('{' block '}' | '=>' expression))?
    | 'set' ('(' IDENTIFIER ')' ('{' block '}' | '=>' expression))?
    ;

methodDef
    : visibilityModifier? modifier* 'fn' IDENTIFIER genericParameters? '(' parameterList? ')'
      ('->' typeExpr)? effectClause? block
    ;

constructorDef: 'fn' IDENTIFIER '(' parameterList? ')' block ;

destructorDef: 'destructor' '(' ')' block ;

staticPropertyDef: 'static' visibilityModifier? typeExpr IDENTIFIER ('=' expression)? ';' ;

staticMethodDef: 'static' visibilityModifier? 'fn' IDENTIFIER '(' parameterList? ')'
                 ('->' typeExpr)? block ;

innerClassDecl: 'class' IDENTIFIER '{' classMember* '}' ;

eventDef: 'event' typeExpr IDENTIFIER ';' ;

indexerDef: 'this' '[' parameterList? ']' '{' propertyAccessor+ '}' ;

operatorOverload: 'operator' OPERATOR '(' parameterList? ')' block ;

delegateDef: 'delegate' typeExpr? IDENTIFIER '(' parameterList? ')' ';' ;

initBlock: 'init' '{' statement* '}' ;

staticBlock: 'static' '{' statement* '}' ;

// ==========================================================================
// INTERFACE DECLARATIONS
// ==========================================================================

interfaceDecl
    : modifier* 'interface' IDENTIFIER genericParameters? extendsClause? '{' interfaceMember* '}'
    ;

interfaceMember
    : methodDef
    | 'default' 'fn' IDENTIFIER '(' parameterList? ')' ('->' typeExpr)? block
    | 'static' 'fn' IDENTIFIER '(' parameterList? ')' ('->' typeExpr)? block
    | 'private' 'fn' IDENTIFIER '(' parameterList? ')' ('->' typeExpr)? block
    | 'async' 'fn' IDENTIFIER '(' parameterList? ')' ('->' typeExpr)? block
    ;

// ==========================================================================
// STRUCT & ENUM DECLARATIONS
// ==========================================================================

structDecl: 'struct' IDENTIFIER genericParameters? '{' structBody '}' ;

structBody: (visibilityModifier? typeExpr IDENTIFIER ';')* ;

enumDecl: 'enum' IDENTIFIER genericParameters? (':' typeExpr)? '{' enumBody '}' ;

enumBody: enumMember (',' enumMember)* ;

enumMember: IDENTIFIER ('(' argumentList? ')')? ;

recordDecl: 'record' IDENTIFIER genericParameters? '(' parameterList? ')'
            extendsClause? implementsClause? '{' recordBody? '}' ;

recordBody: classMember* ;

// ==========================================================================
// TRAIT DECLARATIONS
// ==========================================================================

traitDecl: 'trait' IDENTIFIER genericParameters? '{' traitBody '}' ;

traitBody: (methodDef | associatedType | constantDef)* ;

associatedType: 'type' IDENTIFIER (':' typeConstraint)? ';' ;

constantDef: 'const' IDENTIFIER ':' typeExpr '=' expression ';' ;

implDecl: 'impl' genericParameters? IDENTIFIER ('for' typeExpr)? '{' implBody* '}' ;

implBody: methodDef | constantDef | associatedType ;

// ==========================================================================
// TYPE ALIASES & ADVANCED TYPES
// ==========================================================================

typeAlias: 'type' IDENTIFIER genericParameters? '=' typeExpr ';' ;

genericParameters: '<' genericParameter (',' genericParameter)* '>' ;

genericParameter: IDENTIFIER ('extends' typeConstraint)? ;

typeConstraint: typeExpr ('+' typeExpr)* ;

// ==========================================================================
// QUANTUM COMPUTING
// ==========================================================================

quantumCircuitDecl: 'quantum' 'circuit' IDENTIFIER '(' parameterList? ')' ('->' typeExpr)? block ;

quantumStmt
    : 'measure' IDENTIFIER ('->' IDENTIFIER)?
    | 'reset' IDENTIFIER
    | 'barrier' (IDENTIFIER (',' IDENTIFIER)*)?
    | quantumGate
    ;

quantumGate
    : ('Hadamard' | 'CNOT' | 'PauliX' | 'PauliY' | 'PauliZ' | 'T' | 'S' | 'Swap' | 'RX' | 'RY' | 'RZ' | 'Toffoli' | 'Fredkin')
      '(' IDENTIFIER (',' IDENTIFIER)* (',' DECIMAL)? ')'
    ;

// ==========================================================================
// NANO-AGENTS
// ==========================================================================

nanoAgentDecl: 'nano' 'agent' IDENTIFIER '{' nanoAgentBody '}' ;

nanoAgentBody: nanoCapability* nanoBehavior* nanoProtocol* ;

nanoCapability: 'capability' IDENTIFIER '(' parameterList? ')' block ;

nanoBehavior: 'behavior' IDENTIFIER '(' parameterList? ')' block ;

nanoProtocol: 'protocol' IDENTIFIER '{' protocolRule* '}' ;

protocolRule: 'on' IDENTIFIER '=>' block ;

nanoStmt: 'assemble' '(' expression ')' | 'deploy' '(' expression ')' ;

// ==========================================================================
// MULTI-TIMELINE SYSTEM (MTS)
// ==========================================================================

mtsDecl: 'mts' 'timeline' IDENTIFIER '{' mtsBody '}' ;

mtsBody: mtsSlice* mtsOperation* ;

mtsSlice: 'slice' IDENTIFIER '[' INTEGER ']' block ;

mtsOperation
    : 'fork' '(' IDENTIFIER ')'
    | 'merge' '(' IDENTIFIER ')'
    | 'observe' '(' IDENTIFIER ')'
    | 'rewind' '(' INTEGER ')'
    ;

mtsStmt: 'parallel' '(' block ')' | 'speculative' '(' block ')'
       | 'counterfactual' '(' expression ',' block ')' ;

// ==========================================================================
// SANKOFA MEMORY SYSTEM
// ==========================================================================

sankofaDecl: memoryDecl | wisdomDecl | historyDecl | consensusDecl | interMemoryDecl ;

memoryDecl: 'remember' IDENTIFIER ':' typeExpr '=' expression ';' ;

wisdomDecl: 'wisdom' IDENTIFIER '{' wisdomBody '}' ;

wisdomBody: (premiseDecl | inferenceRule | wisdomStmt)* ;

premiseDecl: 'premise' IDENTIFIER ':' typeExpr '=' expression ';' ;

inferenceRule: 'rule' IDENTIFIER '(' parameterList? ')' '=>' block ;

wisdomStmt: 'conclude' expression ';' ;

sankofaStmt
    : 'learn' 'from' expression ('with' 'weight' expression)? ';'
    | 'recall' '(' expression ',' expression ')' ';'
    | 'ancestral' IDENTIFIER '(' argumentList? ')' ';'
    | 'consensus' '[' expressionList ']' 'vote' expression ';'
    | 'zamani' '{' statement* '}'
    | 'sasa' '{' statement* '}'
    ;

expressionList: expression (',' expression)* ;

interMemoryDecl: 'intermemory' IDENTIFIER '<' IDENTIFIER ',' typeExpr '>' ';' ;

// ==========================================================================
// ALGEBRAIC EFFECTS
// ==========================================================================

effectDecl: 'effect' IDENTIFIER genericParameters? '(' parameterList? ')' ('->' typeExpr)? ';' ;

effectHandleStmt: 'handle' expression '{' effectHandler* '}' ('with' '{' effectRecovery* '}')? ;

effectHandler: 'case' effectName '(' parameterList? ')' '=>' block ;

effectRecovery: '|' IDENTIFIER ':' typeExpr '|' '=>' block ;

// ==========================================================================
// META-PROGRAMMING & LANGUAGE FEATURES
// ==========================================================================

languageDecl: 'language' IDENTIFIER '=' STRING ('{' langBody '}')? ';' ;

langBody: grammarRule* ;

grammarRule: IDENTIFIER ':' STRING ';' ;

invokeStmt: 'invoke' modulePath '(' argumentList? ')' ';' ;

modulePath: IDENTIFIER ('::' IDENTIFIER)* ;

foreignFunctionCall
    : 'extern' STRING '{' externDecl* '}'
    | 'foreign' IDENTIFIER '::' IDENTIFIER '(' argumentList? ')' ';'
    ;

externDecl
    : visibilityModifier? 'fn' IDENTIFIER '(' parameterList? ')' ('->' typeExpr)? ';'
    | visibilityModifier? 'type' IDENTIFIER '=' typeExpr ';'
    ;

macroDecl: 'macro' IDENTIFIER '(' parameterList? ')' block ;

macroCall: IDENTIFIER '!' '(' argumentList? ')' ;

mopExpr
    : 'reflect' '(' expression ')'
    | 'introspect' '(' IDENTIFIER ')'
    | 'meta_eval' '(' expression ')'
    | 'quote' '{' statement* '}'
    | 'unquote' '(' expression ')'
    | 'splice' '(' expression ')'
    ;

// ==========================================================================
// PLUGINS
// ==========================================================================

pluginDecl: 'plugin' IDENTIFIER '{' pluginDefinition* '}' ;

pluginDefinition
    : 'language' IDENTIFIER ';'
    | 'transpiler' IDENTIFIER ';'
    | 'hook' STRING ';'
    | 'entry_point' STRING ';'
    ;

// ==========================================================================
// DATA & SERIALIZATION
// ==========================================================================

dataStmt
    : 'serialize' expression 'to' dataFormat ';'
    | 'deserialize' expression 'from' dataFormat ';'
    ;

dataFormat: 'json' | 'xml' | 'messagepack' | 'protobuf' | 'avro' | 'netcdf' | 'hdf5' ;

// ==========================================================================
// DATABASE & WEB SERVICES
// ==========================================================================

databaseOp: 'db' '::' IDENTIFIER '(' argumentList? ')' ';' ;

webService: 'http' '::' IDENTIFIER '(' argumentList? ')' ';' ;

hdlModuleDecl: 'hdl' 'module' IDENTIFIER '{' hdlPort* hdlStatement* '}' ;

hdlPort: 'port' IDENTIFIER ':' typeExpr ';' ;

hdlStatement: statement ;

// ==========================================================================
// EXPRESSIONS
// ==========================================================================

expression
    : primary
    | expression '.' IDENTIFIER                                    // Property access
    | expression '?.' IDENTIFIER                                   // Optional property access
    | expression '[' expression ']'                                // Indexing
    | expression '[' expression '..' expression ']'               // Range
    | expression '(' argumentList? ')'                             // Function call
    | expression '::' IDENTIFIER '(' argumentList? ')'            // Module invocation
    | '...' expression                                              // Spread operator
    | '++' expression | expression '++'                            // Increment
    | '--' expression | expression '--'                            // Decrement
    | expression '^' expression                                    // Power
    | expression MATMUL expression                                 // Matrix multiplication
    | expression '*' expression | expression '/' expression | expression '%' expression   // Mul
    | expression '+' expression | expression '-' expression        // Add
    | expression '<<' expression | expression '>>' expression | expression '>>>' expression // Shift
    | expression '<' expression | expression '>' expression | expression '<=' expression | expression '>=' expression // Relational
    | expression '==' expression | expression '!=' expression | expression '===' expression | expression '!==' expression // Equality
    | expression '&' expression                                    // Bitwise AND
    | expression '^' expression                                    // Bitwise XOR
    | expression '|' expression                                    // Bitwise OR
    | expression '&&' expression                                   // Logical AND
    | expression '||' expression                                   // Logical OR
    | expression '?' expression ':' expression                     // Ternary
    | 'await' expression                                            // Await
    | 'yield' expression                                            // Yield
    | 'perform' effectName '(' expression? ')'                     // Effect
    | lambdaExpression                                             // Lambda
    | '(' expression ')'                                           // Grouping
    | 'new' IDENTIFIER genericTypes? '(' argumentList? ')'        // New
    | 'this'                                                       // This
    | 'super' ('.' IDENTIFIER | '(' argumentList? ')')?           // Super
    | '(' typeExpr ')' expression                                  // Cast
    | expression 'instanceof' typeExpr                             // Instance check
    | expression 'with' '{' (IDENTIFIER ':' expression ';')* '}'  // With expression
    | mopExpr                                                      // Meta-object protocol
    | macroCall                                                    // Macro call
    | mathExpression                                               // Math expressions
    ;

// Mathematical expressions
mathExpression
    : mathFunctionCall
    | mathConstant
    | complexLiteral
    | polynomialExpr
    ;

mathFunctionCall
    : MATH_FUNC '(' argumentList? ')'
    | MATH_FUNC_ADVANCED '(' argumentList? ')'
    | STATS_FUNC '(' argumentList? ')'
    ;

MATH_FUNC: 'sin' | 'cos' | 'tan' | 'asin' | 'acos' | 'atan' | 'atan2'
         | 'sinh' | 'cosh' | 'tanh' | 'asinh' | 'acosh' | 'atanh'
         | 'exp' | 'log' | 'log10' | 'log2' | 'sqrt' | 'cbrt'
         | 'abs' | 'ceil' | 'floor' | 'round' | 'trunc'
         | 'gamma' | 'lgamma' | 'erf' | 'erfc'
         | 'real' | 'imag' | 'conj' | 'arg' | 'abs'
         ;

MATH_FUNC_ADVANCED: 'besselj' | 'bessely' | 'besseli' | 'besselk'
                  | 'legendre' | 'hermite' | 'laguerre'
                  | 'chebyshev_t' | 'chebyshev_u'
                  | 'jacobi' | 'gegenbauer'
                  | 'zeta' | 'polylog' | 'dilog'
                  | 'elliptic_k' | 'elliptic_e' | 'elliptic_pi'
                  ;

STATS_FUNC: 'norm_pdf' | 'norm_cdf' | 'norm_ppf' | 'norm_logpdf'
          | 'uniform_pdf' | 'uniform_cdf'
          | 'beta_pdf' | 'beta_cdf'
          | 'gamma_pdf' | 'gamma_cdf'
          | 'chi2_pdf' | 'chi2_cdf'
          | 'student_t_pdf' | 'student_t_cdf'
          | 'f_pdf' | 'f_cdf'
          | 'poisson_pmf' | 'poisson_cdf'
          | 'binomial_pmf' | 'binomial_cdf'
          | 'multinomial_pmf'
          | 'dirichlet_pdf'
          ;

mathConstant: 'π' | 'pi' | 'e' | 'φ' | 'phi' | 'γ' | 'gamma_euler' | 'i' ;

complexLiteral: INTEGER_OR_FLOAT ('i' | 'j') | '(' expression ('+' | '-') expression ('i' | 'j') ')' ;

polynomialExpr: 'poly' '(' argumentList? ')' | 'Poly' '[' expression (',' expression)* ']' ;

primary
    : IDENTIFIER
    | literal
    | templateString
    | arrayLiteral
    | mapLiteral
    | tupleLiteral
    | structLiteral
    | quantumLiteral
    | nanoLiteral
    | mtsLiteral
    | vectorLiteral
    | matrixLiteral
    ;

templateString: '`' (templatePart | templateExpression)* '`' ;

templatePart: ~[`$\\]+ | '\\' . ;

templateExpression: '${' expression '}' ;

arrayLiteral: '[' (expression (',' expression)*)? ']' ;

mapLiteral: '{' (mapEntry (',' mapEntry)*)? '}' ;

mapEntry: expression ':' expression ;

tupleLiteral: '(' expression (',' expression)+ ')' ;

structLiteral: IDENTIFIER '{' (IDENTIFIER ':' expression (',' IDENTIFIER ':' expression)*)? '}' ;

quantumLiteral: '|' QUBIT_STATE 'rangle' ;

QUBIT_STATE: ('0' | '1' | '+' | '-' | IDENTIFIER) ;

nanoLiteral
    : '@atom' '(' ELEMENT ':' ORBITAL ')'
    | '@molecule' '(' FORMULA ')'
    ;

ELEMENT: IDENTIFIER ;

ORBITAL: ('1s' | '2s' | '2p' | '3s' | '3p' | '3d' | '4s' | '4p' | '4d' | '4f') ;

FORMULA: IDENTIFIER (DIGIT* IDENTIFIER)* ;

mtsLiteral: 'mts' '[' INTEGER ']' ;

vectorLiteral: '[' expression (',' expression)* ']' ;

matrixLiteral: '[[' (expression (',' expression)*)? (',' '[' (expression (',' expression)*)? ']')* ']]' ;

// ==========================================================================
// TYPES
// ==========================================================================

typeExpr
    : baseType
    | genericType
    | arrayType
    | tupleType
    | functionType
    | dependentType
    | linearType
    | affineType
    | effectfulType
    | universeType
    | quantumType
    | nanoType
    | mtsType
    | sankofaType
    | cognitiveType
    | nullableType
    | refType
    | boxedType
    | unionType
    | mathType
    ;

baseType
    : 'void' | 'int' | 'float' | 'bool' | 'string' | 'char' | 'bytes'
    | 'i8' | 'i16' | 'i32' | 'i64' | 'i128'
    | 'u8' | 'u16' | 'u32' | 'u64' | 'u128'
    | 'f32' | 'f64' | 'usize' | 'isize'
    | 'never' | 'null'
    | IDENTIFIER
    ;

genericType: baseType '<' typeExpr (',' typeExpr)* '>' ;

genericTypes: '<' typeExpr (',' typeExpr)* '>' ;

arrayType: typeExpr '[' expression? ']' ;

tupleType: '(' typeExpr (',' typeExpr)+ ')' ;

functionType: '(' typeExpr (',' typeExpr)* ')' '->' typeExpr ;

nullableType: typeExpr '?' ;

refType: ('ref' | '&') ('mut')? typeExpr ;

boxedType: 'Box' '<' typeExpr '>' ;

unionType: typeExpr '|' typeExpr ;

// Dependent Types
dependentType: piType | sigmaType | identityType ;

piType: ('Pi' | '\u03A0') '(' IDENTIFIER ':' typeExpr ')' typeExpr ;

sigmaType: ('Sigma' | '\u03A3') '(' IDENTIFIER ':' typeExpr ')' typeExpr ;

identityType: 'Id' '(' typeExpr ',' expression ',' expression ')' ;

// Universe Hierarchy
universeType: 'Type_0' | 'Type_1' | 'Type_2' | 'Type_N' | 'Kind' | 'Sort' | 'Prop' ;

// Linear / Affine Types
linearType: 'linear' typeExpr ;

affineType: 'affine' typeExpr ;

// Effectful Types
effectfulType: typeExpr 'with' 'effects' '{' effectName (',' effectName)* '}' ;

// Quantum Types
quantumType
    : 'Qubit'
    | 'QReg' '[' expression ']'
    | 'Superposition' '<' typeExpr '>'
    | 'Entangled' '<' typeExpr ',' typeExpr '>'
    | 'QMeasured' '<' typeExpr '>'
    | 'QArray' '<' typeExpr ',' expression '>'
    ;

// Nano Types
nanoType
    : 'Atom' '<' typeExpr '>'
    | 'Molecule' '<' typeExpr '>'
    | 'NanoAgent' '<' typeExpr '>'
    | 'Archaeve' '<' typeExpr '>'
    ;

// MTS Types
mtsType: 'MtsSlice' '<' typeExpr '>' ;

// Sankofa Types
sankofaType
    : 'History' '<' typeExpr ',' expression '>'
    | 'ConsensusTrue' '<' typeExpr '>'
    | 'InterMemory' '<' STRING ',' typeExpr '>'
    ;

// Cognitive Types
cognitiveType
    : 'CognitiveState' '<' typeExpr '>'
    | 'Consciousness' '<' typeExpr '>'
    | 'Biological' '<' typeExpr '>'
    | 'Neural' '<' typeExpr '>'
    | 'MemoryBank' '<' typeExpr '>'
    | 'AgentType'
    | 'NarrowAI' | 'AGI' | 'ASI' | 'AESI' | 'ASESI'
    ;

// Mathematical Types
mathType
    : 'Complex' '<' typeExpr '>'
    | 'Vector' '<' typeExpr (',' INTEGER)? '>'
    | 'Matrix' '<' typeExpr (',' INTEGER ',' INTEGER)? '>'
    | 'Tensor' '<' typeExpr (',' INTEGER)+ '>'
    | 'Polynomial' '<' typeExpr '>'
    | 'Rational' '<' typeExpr '>'
    | 'Distribution' '<' typeExpr '>'
    | 'Symbolic' '<' typeExpr '>'
    | 'Expr'  // Symbolic expression
    | 'Derivative' '<' typeExpr '>'
    | 'Integral' '<' typeExpr '>'
    ;

// ==========================================================================
// ARGUMENTS & PARAMETERS
// ==========================================================================

argumentList
    : argument (',' argument)*
    | argument (',' argument)* ',' '...' IDENTIFIER
    ;

argument
    : expression
    | IDENTIFIER ':' expression
    | '...' expression
    ;

parameterList: parameter (',' parameter)* ;

parameter
    : visibilityModifier? IDENTIFIER ':' typeExpr ('=' literal)?
    | '...' IDENTIFIER ':' typeExpr
    ;

// ==========================================================================
// LITERALS
// ==========================================================================

literal
    : INTEGER
    | DECIMAL
    | STRING
    | CHAR
    | BOOLEAN
    | 'null'
    | 'undefined'
    ;

// ==========================================================================
// LEXER
// ==========================================================================

// Keywords
IMPORT: 'import' ;
INVOKE: 'invoke' ;
TRANSCODE: 'transcode' ;
OVERRIDE: 'override' ;
LANG: 'lang' ;
MODULE: 'module' ;
EXPORT: 'export' ;
TO: 'to' ;
FROM: 'from' ;
PLUGIN: 'plugin' ;
LANGUAGE: 'language' ;
TRANSPILER: 'transpiler' ;
SELF: 'self' ;
FOREIGN: 'foreign' ;
INTEROP: 'interop' ;
CDECL: 'cdecl' ;
STDCALL: 'stdcall' ;
JAVA: 'java' ;
GC: 'gc' ;
CAST: 'cast' ;
CONVERT: 'convert' ;
DATA: 'data' ;
SERIALIZE: 'serialize' ;
DESERIALIZE: 'deserialize' ;
STREAM: 'stream' ;
DATABASE: 'db' ;
HTTP: 'http' ;
INTERFACE: 'interface' ;
NAMESPACE: 'namespace' ;
FUNCTION: 'function' ;
IF: 'if' ;
ELSE: 'else' ;
WHILE: 'while' ;
DO: 'do' ;
FOR: 'for' ;
FORALL: 'forall' ;
FOREACH: 'foreach' ;
REDUCE: 'reduce' ;
BREAK: 'break' ;
CONTINUE: 'continue' ;
MATCH: 'match' ;
CASE: 'case' ;
TRY: 'try' ;
CATCH: 'catch' ;
FINALLY: 'finally' ;
ASYNC: 'async' ;
AWAIT: 'await' ;
YIELD: 'yield' ;
THROW: 'throw' ;
QUANTUM: 'quantum' ;
CIRCUIT: 'circuit' ;
NANO: 'nano' ;
AGENT: 'agent' ;
MTS: 'mts' ;
SANKOFA: 'sankofa' ;
REMEMBER: 'remember' ;
LEARN: 'learn' ;
RECALL: 'recall' ;
CONSENSUS: 'consensus' ;
EFFECT: 'effect' ;
HANDLE: 'handle' ;
PERFORM: 'perform' ;
EXTERN: 'extern' ;
UNSAFE: 'unsafe' ;
CLASS: 'class' ;
STRUCT: 'struct' ;
ENUM: 'enum' ;
TRAIT: 'trait' ;
IMPL: 'impl' ;
RECORD: 'record' ;
EXTENDS: 'extends' ;
IMPLEMENTS: 'implements' ;
MACRO: 'macro' ;
PACKAGE: 'package' ;
TYPE: 'type' ;
CONTRACT: 'contract' ;
REQUIRES: 'requires' ;
ENSURES: 'ensures' ;
INVARIANT: 'invariant' ;
VECTOR: 'vector' ;
MATRIX: 'matrix' ;
TENSOR: 'tensor' ;
VECTORIZE: 'vectorize' ;
MAP: 'map' ;
ZIP: 'zip' ;
ZIP2: 'zip' ;
SCAN: 'scan' ;
FILTER: 'filter' ;
TRANSPOSE: 'transpose' ;
INVERSE: 'inverse' ;
DETERMINANT: 'determinant' ;
RANK: 'rank' ;
EIGENVALUES: 'eigenvalues' ;
EIGENVECTORS: 'eigenvectors' ;
SVD: 'svd' ;
QR: 'qr' ;
LU: 'lu' ;
CHOLESKY: 'cholesky' ;
NORM: 'norm' ;
TRACE: 'trace' ;
CONTRACT: 'contract' ;
EINSUM: 'einsum' ;
RESHAPE: 'reshape' ;
PERMUTE: 'permute' ;
EXPAND: 'expand' ;
SIMPLIFY: 'simplify' ;
FACTOR: 'factor' ;
SOLVE: 'solve' ;
DSOLVE: 'dsolve' ;
SERIES: 'series' ;
ROOTS: 'roots' ;
RESULTANT: 'resultant' ;
DIFF: 'diff' ;
INTEGRAL: 'integral' ;
LIMIT: 'limit' ;
GRADIENT: 'gradient' ;
JACOBIAN: 'jacobian' ;
HESSIAN: 'hessian' ;
LAPLACIAN: 'laplacian' ;
DIVERGENCE: 'divergence' ;
CURL: 'curl' ;
MEAN: 'mean' ;
MEDIAN: 'median' ;
MODE: 'mode' ;
VARIANCE: 'variance' ;
STD_DEV: 'std_dev' ;
COVARIANCE: 'covariance' ;
CORRELATION: 'correlation' ;
QUANTILE: 'quantile' ;
HISTOGRAM: 'histogram' ;
PDF: 'pdf' ;
CDF: 'cdf' ;
SAMPLE: 'sample' ;
HYPOTHESIS_TEST: 'hypothesis_test' ;
ANOVA: 'anova' ;
REGRESSION: 'regression' ;
PCA: 'pca' ;
FFT: 'fft' ;
IFFT: 'ifft' ;
RFFT: 'rfft' ;
IRFFT: 'irfft' ;
CONVOLVE: 'convolve' ;
CORRELATE: 'correlate' ;
INTERPOLATE: 'interpolate' ;
MINIMIZE: 'minimize' ;
MAXIMIZE: 'maximize' ;
SUBJECT: 'subject' ;
METHOD: 'method' ;
OPERATOR: '+' | '-' | '*' | '/' | '%' | '==' | '!=' | '<' | '>' | '<=' | '>=' | '&&' | '||' | '&' | '|' | '^' | '~' | '<<' | '>>' | '[' | ']' | '(' | ')' ;

// Mathematical operators
MATMUL: '@@' | '⊗' ;  // Matrix multiplication

// Common tokens
IDENTIFIER: [a-zA-Z_][a-zA-Z_0-9]* ;
INTEGER: [0-9]+ | '0x' [0-9a-fA-F]+ | '0b' [0-1]+ | '0o' [0-7]+ ;
DECIMAL: [0-9]+ '.' [0-9]+ ([eE] [+-]? [0-9]+)? ;
INTEGER_OR_FLOAT: INTEGER | DECIMAL ;
BOOLEAN: 'true' | 'false' ;
CHAR: '\'' (ESC | ~['\\]) '\'' ;

// STRINGS
STRING: '"' (ESC | ~["\\])* '"' ;
TEMPLATE_STRING: '`' (ESC | ~[`\\])* '`' ;
RAW_STRING: 'r' '#'* '"' ~["]* '"' '#'* ;
UTF8_STRING: 'u8' '"' (ESC | ~["\\])* '"' ;

// Escape sequences
fragment ESC: '\\' (["\\/bfnrt] | 'u' [0-9a-fA-F]{4}) ;

// Comments
DOC_COMMENT: '///' ~[\r\n]* | '/**' .*? '*/' ;
LINE_COMMENT: '//' ~[\r\n]* -> skip ;
COMMENT: '/*' .*? '*/' -> skip ;

// Operators and punctuation
LPAREN: '(' ;
RPAREN: ')' ;
LBRACE: '{' ;
RBRACE: '}' ;
LBRACKET: '[' ;
RBRACKET: ']' ;
LT: '<' ;
GT: '>' ;
COMMA: ',' ;
SEMI: ';' ;
COLONCOLON: '::' ;
COLON: ':' ;
DOT: '.' ;
QUESTION: '?' ;
ARROW: '=>' ;
FATARROW: '->' ;
ELLIPSIS: '...' ;
DOUBLESTAR: '**' ;
CARET: '^' ;
TILDE: '~' ;
AMPERSAND: '&' ;
PIPE: '|' ;
AT: '@' ;
DOLLAR: '$' ;
BANG: '!' ;
EQ: '=' ;
PLUSEQ: '+=' ;
MINUSEQ: '-=' ;
STAREQ: '*=' ;
SLASHEQ: '/=' ;
PERCENTEQ: '%=' ;
AMPEQ: '&=' ;
PIPEEQ: '|=' ;
CARETEQ: '^=' ;
LSHIFT: '<<' ;
RSHIFT: '>>' ;
LSHIFTEQ: '<<=' ;
RSHIFTEQ: '>>=' ;
EQEQ: '==' ;
BANGEQ: '!=' ;
EQEQEQ: '===' ;
BANGEQEQ: '!==' ;
LTEQ: '<=' ;
GTEQ: '>=' ;
DOUBLEAND: '&&' ;
DOUBLEOR: '||' ;
PLUSPLUS: '++' ;
MINUSMINUS: '--' ;
DOTQUESTION: '?.' ;
RANGLE: '>' ;

// Whitespace
WS: [ \t\r\n]+ -> skip ;

// Fragment for digits
fragment DIGIT: [0-9] ;
