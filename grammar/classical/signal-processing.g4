/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/classical/signal-processing.g4
* 
* Status:
* PRODUCTION CLASSICAL SIGNAL-PROCESSING DOMAIN GRAMMAR
* 
* Domain:
* Classical computation / signal processing
* 
* Grammar technology:
* ANTLR4 parser grammar
* 
* Implementation baseline:
* Rust 1.97 / Rust 1.97.1
* Rust 2021
* 
* Safety:
* No embedded Rust actions.
* No semantic predicates.
* No unsafe Rust requirement.
* No target-specific parser behavior.
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file owns the SOURCE-SYNTAX boundary for classical signal processing.
* 
* It does NOT implement signal-processing algorithms.
* 
* It does NOT define:
* 
* FFT implementation
* DFT implementation
* FIR implementation
* IIR implementation
* convolution implementation
* correlation implementation
* resampling implementation
* interpolation implementation
* filter coefficients
* numerical kernels
* SIMD width
* vector width
* CPU count
* GPU count
* FPGA count
* accelerator count
* memory capacity
* device identifiers
* physical addresses
* scheduling
* placement
* routing
* hardware discovery
* 
* Those belong downstream.
* 
* ============================================================================
* ARCHITECTURAL PIPELINE
* ============================================================================
* 
* Zamani source
*      |
*      v
* ZamaniLexer
*      |
*      v
* ZamaniParser
*      |
*      v
* SignalProcessing
*      |
*      v
* domain-neutral frontend AST
*      |
*      v
* semantic analysis
*      |
*      +--> signal type/shape/rate analysis
*      +--> unit analysis
*      +--> effect analysis
*      +--> capability analysis
*      +--> resource analysis
*      |
*      v
* canonical semantic model
*      |
*      v
* classical / numerical / data IR
*      |
*      v
* optimization
*      |
*      v
* scheduling / placement / lowering
*      |
*      v
* target realization
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Signal-processing syntax describes WHAT signal computation means.
* 
* It does not prescribe WHERE or HOW that computation executes.
* 
* Therefore the same source semantics may be realized on:
* 
* scalar CPU
* multicore CPU
* vector processor
* GPU
* FPGA
* ASIC
* DSP
* accelerator
* embedded processor
* cluster
* distributed system
* simulator
* future architecture
* 
* without changing this grammar.
* 
* No language-level resource limit is introduced here.
* 
* ============================================================================
* HARD-CODING PROHIBITION
* ============================================================================
* 
* This grammar deliberately contains no:
* 
* MAX_SIGNAL_LENGTH
* MAX_CHANNELS
* MAX_SAMPLES
* MAX_RATE
* MAX_FREQUENCY
* MAX_TAPS
* MAX_FILTER_ORDER
* MAX_FFT_SIZE
* MAX_WINDOW_SIZE
* MAX_BUFFER_SIZE
* MAX_DIMENSIONS
* MAX_VECTOR_WIDTH
* MAX_THREADS
* MAX_CORES
* MAX_GPUS
* MAX_FPGAS
* MAX_ACCELERATORS
* MAX_MEMORY
* 
* Any finite quantity written by the programmer is PROGRAM SEMANTICS.
* 
* Example:
* 
* samples = signal::sample(input, rate)
* 
* does not establish a universal maximum sample count.
* 
* Likewise:
* 
* fft(input, size)
* 
* does not make "size" a compiler-defined maximum.
* 
* Resource exhaustion is a downstream implementation/resource issue, not
* grammar semantics.
* 
* ============================================================================
* OPEN-WORLD OPERATION MODEL
* ============================================================================
* 
* Signal-processing operations are intentionally OPEN-WORLD.
* 
* The grammar does not enumerate:
* 
* FFT
* DFT
* FIR
* IIR
* STFT
* DCT
* wavelet
* Hilbert
* Kalman
* LMS
* RLS
* convolution
* correlation
* resampling
* demodulation
* modulation
* etc.
* 
* Those are operation names and semantic capabilities.
* 
* This permits:
* 
* fft(signal)
* signal::fft(signal)
* dsp::fft(signal)
* vendor::custom_transform(signal)
* future::transform(signal)
* 
* without changing the core grammar for every new algorithm.
* 
* The semantic layer determines whether an operation name is:
* 
* standard
* intrinsic
* library-provided
* dialect-provided
* user-defined
* vendor-provided
* unavailable
* 
* The parser only recognizes its structure.
* 
* ============================================================================
* TOKEN POLICY
* ============================================================================
* 
* Existing canonical lexer tokens are reused.
* 
* No signal-specific lexer token is required.
* 
* In particular, this grammar uses existing:
* 
* IDENTIFIER
* LPAREN
* RPAREN
* LBRACKET
* RBRACKET
* COMMA
* COLON
* DOT
* DOUBLE_COLON
* LESS
* GREATER
* ASSIGN
* PIPE
* THIN_ARROW
* ELLIPSIS
* 
* together with the existing expression/type/name rules.
* 
* Signal-processing concepts such as:
* 
* sample
* signal
* spectrum
* filter
* window
* frequency
* phase
* amplitude
* convolution
* fft
* 
* remain identifiers unless they are already language-wide keywords.
* 
* This prevents lexical keyword explosion.
* 
* ============================================================================
* COMPOSITION CONTRACT
* ============================================================================
* 
* This grammar is a leaf/domain grammar.
* 
* It is intended to be imported by:
* 
* grammar/classical/classical.g4
* 
* and then composed upward through:
* 
* grammar/antlr/ZamaniParser.g4
* 
* The final composition chain is:
* 
* Zamani.g4
*      |
* ZamaniParser.g4
*      |
* Classical
*      |
* SignalProcessing
* 
* The grammar intentionally reuses canonical rules such as:
* 
* expression
* typeExpression
* identifier
* qualifiedName
* 
* supplied by the shared parser composition hierarchy.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* This grammar MUST NOT require a signal-specific backend AST.
* 
* The frontend should preserve the generic operation structure already used
* by Zamani:
* 
* operation name
* namespace / qualification
* operands
* parameters
* results
* attributes
* modifiers
* effects
* capabilities
* source span
* 
* Signal-specific meaning is attached during semantic analysis.
* 
* Conceptually:
* 
* signalOperationInvocation
*         |
*         v
* generic frontend operation
*         |
*         v
* signal-processing semantic operation
*         |
*         v
* canonical classical/data/numerical IR
* 
* This file MUST NOT introduce:
* 
* SignalOperation enum
* FilterKind enum containing every possible filter
* TransformKind enum containing every possible transform
* 
* because those would turn the grammar into a closed algorithm registry.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis is responsible for determining:
* 
* signal element type
* signal shape
* channel structure
* sample domain
* sampling rate
* time domain
* frequency domain
* units
* dimensional consistency
* axis meaning
* transform legality
* filter legality
* coefficient compatibility
* convolution compatibility
* correlation compatibility
* resampling legality
* window compatibility
* overlap compatibility
* boundary conditions
* numerical precision
* determinism
* effects
* resource requirements
* capability requirements
* portability
* 
* These are NOT parser decisions.
* 
* ============================================================================
* RESOURCE / CAPABILITY CONTRACT
* ============================================================================
* 
* A signal program may express resource or capability intent through the
* repository's canonical resource grammar.
* 
* Examples:
* 
* requires capability("signal.transform")
* requires capability("signal.streaming")
* requires capability("signal.acceleration")
* 
* The exact capability registry is owned outside this file.
* 
* This grammar must not define whether a target actually provides a
* capability.
* 
* Likewise, this grammar does not choose:
* 
* CPU
* GPU
* FPGA
* DSP
* ASIC
* accelerator
* 
* The compiler and resource/capability layers make those decisions.
* 
* ============================================================================
* SIGNAL MODEL
* ============================================================================
* 
* A signal is treated as semantic data.
* 
* A signal may represent:
* 
* scalar samples
* vector samples
* matrix-valued samples
* tensor-valued samples
* symbolic samples
* complex-valued samples
* multichannel samples
* time-domain data
* frequency-domain data
* event streams
* continuous-domain abstractions
* discrete-domain abstractions
* 
* The grammar does not impose a fixed representation.
* 
* ============================================================================
* SHAPE MODEL
* ============================================================================
* 
* Signal shape is structural and unbounded.
* 
* Examples:
* 
* <N>
* <Channels, Samples>
* <Batch, Channels, Samples>
* <Batch, Time, Frequency, Channel>
* 
* Dimensions are expressions.
* 
* No fixed number of dimensions is imposed.
* 
* ============================================================================
* DOMAIN MODEL
* ============================================================================
* 
* The grammar supports explicit semantic signal-domain constructs:
* 
* signal
* stream
* sample
* window
* spectrum
* filter
* transform
* 
* These are semantic categories rather than implementation algorithms.
* 
* ============================================================================
  */

parser grammar SignalProcessing;

options {
tokenVocab = ZamaniLexer;
}

/* ============================================================================

* 1. PUBLIC SIGNAL-PROCESSING CONSTRUCT
* ============================================================================
* 
* This is the stable entry point imported by Classical.
* 
* It deliberately covers declarations, expressions and computation regions
* without becoming a second program grammar.
* 
* ============================================================================
  */

signalProcessingConstruct
: signalProcessingDeclaration
| signalProcessingStatement
| signalProcessingExpression
| signalProcessingPipeline
;

/* ============================================================================

* 2. DOMAIN DECLARATIONS
* ============================================================================
* 
* Signal declarations remain domain-specific only where the declaration
* carries signal semantics that cannot be expressed by a normal declaration.
* 
* ============================================================================
  */

signalProcessingDeclaration
: signalDeclaration
| streamDeclaration
| sampleDeclaration
| windowDeclaration
| filterDeclaration
| transformDeclaration
| spectrumDeclaration
| signalProfileDeclaration
;

/* ============================================================================

* 3. SIGNAL DECLARATION
* ============================================================================
* 
* Example conceptual forms:
* 
* signal audio: Signal<f32>
* signal audio: Signal<f32, <Channels, Samples>>
* signal audio: signalType
* 
* The exact type semantics belong to the type system.
* 
* ============================================================================
  */

signalDeclaration
: identifier
COLON
signalTypeSpecification
| identifier
ASSIGN
signalExpression
| identifier
COLON
signalTypeSpecification
ASSIGN
signalExpression
;

/* ============================================================================

* 4. STREAM DECLARATION
* ============================================================================
  */

streamDeclaration
: identifier
COLON
streamTypeSpecification
| identifier
ASSIGN
signalStreamExpression
| identifier
COLON
streamTypeSpecification
ASSIGN
signalStreamExpression
;

/* ============================================================================

* 5. SAMPLE DECLARATION
* ============================================================================
  */

sampleDeclaration
: identifier
COLON
sampleTypeSpecification
| identifier
ASSIGN
signalSampleExpression
| identifier
COLON
sampleTypeSpecification
ASSIGN
signalSampleExpression
;

/* ============================================================================

* 6. WINDOW DECLARATION
* ============================================================================
  */

windowDeclaration
: identifier
COLON
windowTypeSpecification
| identifier
ASSIGN
signalWindowExpression
| identifier
COLON
windowTypeSpecification
ASSIGN
signalWindowExpression
;

/* ============================================================================

* 7. FILTER DECLARATION
* ============================================================================
* 
* Filter declarations are intentionally structural.
* 
* A filter may be user-defined, library-defined, symbolic, parameterized,
* adaptive, or supplied by a dialect.
* 
* ============================================================================
  */

filterDeclaration
: identifier
COLON
filterTypeSpecification
| identifier
ASSIGN
signalFilterExpression
| identifier
COLON
filterTypeSpecification
ASSIGN
signalFilterExpression
;

/* ============================================================================

* 8. TRANSFORM DECLARATION
* ============================================================================
  */

transformDeclaration
: identifier
COLON
transformTypeSpecification
| identifier
ASSIGN
signalTransformExpression
| identifier
COLON
transformTypeSpecification
ASSIGN
signalTransformExpression
;

/* ============================================================================

* 9. SPECTRUM DECLARATION
* ============================================================================
  */

spectrumDeclaration
: identifier
COLON
spectrumTypeSpecification
| identifier
ASSIGN
signalSpectrumExpression
| identifier
COLON
spectrumTypeSpecification
ASSIGN
signalSpectrumExpression
;

/* ============================================================================

* 10. SIGNAL PROFILE DECLARATION
* ============================================================================
* 
* A profile describes semantic metadata such as:
* 
* sampling rate
* units
* domain
* channels
* precision
* provenance
* 
* The profile itself does not allocate storage or select hardware.
* 
* ============================================================================
  */

signalProfileDeclaration
: identifier
COLON
signalProfileType
| identifier
ASSIGN
signalProfileExpression
;

/* ============================================================================

* 11. SIGNAL TYPE SPECIFICATIONS
* ============================================================================
* 
* These are domain wrappers around the canonical type system.
* 
* They do not create a second type language.
* 
* ============================================================================
  */

signalTypeSpecification
: typeExpression
;

streamTypeSpecification
: typeExpression
;

sampleTypeSpecification
: typeExpression
;

windowTypeSpecification
: typeExpression
;

filterTypeSpecification
: typeExpression
;

transformTypeSpecification
: typeExpression
;

spectrumTypeSpecification
: typeExpression
;

signalProfileType
: typeExpression
;

/* ============================================================================

* 12. STATEMENTS
* ============================================================================
* 
* Statement ownership remains in statements/.
* 
* This domain wrapper exists only where the canonical parser needs to
* classify a signal-processing statement.
* 
* ============================================================================
  */

signalProcessingStatement
: signalOperationStatement
| signalPipelineStatement
| signalWindowStatement
| signalSamplingStatement
| signalTransformStatement
| signalFilterStatement
| signalAnalysisStatement
;

/* ============================================================================

* 13. SIGNAL OPERATION STATEMENT
* ============================================================================
  */

signalOperationStatement
: signalOperationInvocation
;

/* ============================================================================

* 14. SIGNAL PIPELINE STATEMENT
* ============================================================================
  */

signalPipelineStatement
: signalProcessingPipeline
;

/* ============================================================================

* 15. WINDOW STATEMENT
* ============================================================================
  */

signalWindowStatement
: signalWindowExpression
;

/* ============================================================================

* 16. SAMPLING STATEMENT
* ============================================================================
  */

signalSamplingStatement
: signalSamplingExpression
;

/* ============================================================================

* 17. TRANSFORM STATEMENT
* ============================================================================
  */

signalTransformStatement
: signalTransformExpression
;

/* ============================================================================

* 18. FILTER STATEMENT
* ============================================================================
  */

signalFilterStatement
: signalFilterExpression
;

/* ============================================================================

* 19. ANALYSIS STATEMENT
* ============================================================================
  */

signalAnalysisStatement
: signalAnalysisExpression
;

/* ============================================================================

* 20. SIGNAL EXPRESSIONS
* ============================================================================
* 
* Signal expressions reuse the universal expression grammar.
* 
* No arithmetic, indexing, slicing, call, comparison, or logical expression
* syntax is duplicated here.
* 
* ============================================================================
  */

signalProcessingExpression
: signalExpression
;

signalExpression
: signalOperationInvocation
| signalReference
| signalConstruction
| signalSamplingExpression
| signalWindowExpression
| signalTransformExpression
| signalFilterExpression
| signalSpectrumExpression
| signalAnalysisExpression
| signalPipelineExpression
| expression
;

/* ============================================================================

* 21. SIGNAL REFERENCE
* ============================================================================
  */

signalReference
: identifier
| qualifiedName
;

/* ============================================================================

* 22. SIGNAL CONSTRUCTION
* ============================================================================
  */

signalConstruction
: signalConstructorInvocation
| signalLiteral
| signalStreamConstruction
;

/* ============================================================================

* 23. SIGNAL CONSTRUCTOR
* ============================================================================
* 
* The constructor name is open-world.
* 
* Examples:
* 
* signal(...)
* samples(...)
* stream(...)
* channel(...)
* buffer(...)
* 
* The semantic layer decides which constructors exist.
* 
* ============================================================================
  */

signalConstructorInvocation
: signalOperationName
LPAREN
signalArgumentList?
RPAREN
;

/* ============================================================================

* 24. SIGNAL LITERAL
* ============================================================================
* 
* Literal structure remains recursive and unbounded.
* 
* It is intentionally compatible with ordinary expression syntax.
* 
* ============================================================================
  */

signalLiteral
: LBRACKET
signalLiteralElementList?
RBRACKET
;

signalLiteralElementList
: signalLiteralElement
(COMMA signalLiteralElement)*
COMMA?
;

signalLiteralElement
: expression
| signalLiteral
;

/* ============================================================================

* 25. STREAM CONSTRUCTION
* ============================================================================
  */

signalStreamConstruction
: signalOperationName
LPAREN
signalArgumentList?
RPAREN
;

/* ============================================================================

* 26. SIGNAL SAMPLING
* ============================================================================
* 
* Sampling is an operation category, not a fixed algorithm.
* 
* ============================================================================
  */

signalSamplingExpression
: signalOperationInvocation
;

/* ============================================================================

* 27. WINDOW EXPRESSION
* ============================================================================
  */

signalWindowExpression
: signalOperationInvocation
;

/* ============================================================================

* 28. TRANSFORM EXPRESSION
* ============================================================================
* 
* Supports open-world transforms such as:
* 
* fft(...)
* dft(...)
* dct(...)
* wavelet(...)
* transform(...)
* 
* without enumerating them.
* 
* ============================================================================
  */

signalTransformExpression
: signalOperationInvocation
;

/* ============================================================================

* 29. FILTER EXPRESSION
* ============================================================================
* 
* Supports:
* 
* fir(...)
* iir(...)
* filter(...)
* adaptive(...)
* notch(...)
* lowpass(...)
* highpass(...)
* 
* and future operations without grammar modification.
* 
* ============================================================================
  */

signalFilterExpression
: signalOperationInvocation
;

/* ============================================================================

* 30. SPECTRUM EXPRESSION
* ============================================================================
  */

signalSpectrumExpression
: signalOperationInvocation
;

/* ============================================================================

* 31. ANALYSIS EXPRESSION
* ============================================================================
* 
* Analysis includes semantic categories such as:
* 
* spectral analysis
* statistical analysis
* frequency estimation
* phase estimation
* amplitude estimation
* noise analysis
* feature extraction
* 
* Algorithms remain open-world operation names.
* 
* ============================================================================
  */

signalAnalysisExpression
: signalOperationInvocation
;

/* ============================================================================

* 32. STREAM EXPRESSION
* ============================================================================
  */

signalStreamExpression
: signalOperationInvocation
| signalReference
| expression
;

/* ============================================================================

* 33. SAMPLE EXPRESSION
* ============================================================================
  */

signalSampleExpression
: signalOperationInvocation
| signalReference
| expression
;

/* ============================================================================

* 34. SIGNAL WINDOW EXPRESSION
* ============================================================================
  */

signalWindowExpression
: signalOperationInvocation
| signalReference
| expression
;

/* ============================================================================

* 35. FILTER EXPRESSION
* ============================================================================
  */

signalFilterExpression
: signalOperationInvocation
| signalReference
| expression
;

/* ============================================================================

* 36. TRANSFORM EXPRESSION
* ============================================================================
  */

signalTransformExpression
: signalOperationInvocation
| signalReference
| expression
;

/* ============================================================================

* 37. SPECTRUM EXPRESSION
* ============================================================================
  */

signalSpectrumExpression
: signalOperationInvocation
| signalReference
| expression
;

/* ============================================================================

* 38. PROFILE EXPRESSION
* ============================================================================
  */

signalProfileExpression
: signalOperationInvocation
| signalReference
| expression
;

/* ============================================================================

* 39. PIPELINE EXPRESSION
* ============================================================================
* 
* A pipeline is structural composition.
* 
* Example:
* 
* source
*   |> filter(...)
*   |> transform(...)
*   |> analyze(...)
* 
* The PIPE token is already part of the canonical Zamani operator vocabulary.
* 
* No pipeline length limit exists.
* 
* ============================================================================
  */

signalPipelineExpression
: signalPipelineStage
(PIPE signalPipelineStage)*
;

signalPipelineStage
: signalOperationInvocation
| signalReference
| expression
;

/* ============================================================================

* 40. PIPELINE
* ============================================================================
  */

signalProcessingPipeline
: signalPipelineExpression
;

/* ============================================================================

* 41. GENERIC SIGNAL OPERATION
* ============================================================================
* 
* This is the central extensibility mechanism.
* 
* Operation names remain source-level names.
* 
* ============================================================================
  */

signalOperationInvocation
: signalOperationName
LPAREN
signalArgumentList?
RPAREN
;

/* ============================================================================

* 42. OPERATION NAME
* ============================================================================
* 
* Qualified names permit:
* 
* fft
* signal::fft
* dsp::fft
* scientific::transform
* vendor::operation
* future::operation
* 
* The grammar does not decide whether the operation is valid.
* 
* ============================================================================
  */

signalOperationName
: identifier
| qualifiedName
;

/* ============================================================================

* 43. ARGUMENT LIST
* ============================================================================
  */

signalArgumentList
: signalArgument
(COMMA signalArgument)*
COMMA?
;

/* ============================================================================

* 44. ARGUMENT
* ============================================================================
* 
* Both positional and named arguments are supported.
* 
* Named argument semantics are resolved by semantic analysis.
* 
* ============================================================================
  */

signalArgument
: signalNamedArgument
| expression
;

/* ============================================================================

* 45. NAMED ARGUMENT
* ============================================================================
  */

signalNamedArgument
: identifier
COLON
expression
;

/* ============================================================================

* 46. OPERATION TYPE PARAMETERS
* ============================================================================
* 
* Generic operation/type parameters remain compatible with the canonical
* type system.
* 
* No finite number of parameters is imposed.
* 
* ============================================================================
  */

signalTypeArgumentList
: LESS
signalTypeArgumentSequence?
GREATER
;

signalTypeArgumentSequence
: typeExpression
(COMMA typeExpression)*
COMMA?
;

/* ============================================================================

* 47. SIGNAL SHAPE
* ============================================================================
* 
* Shape dimensions are arbitrary expressions.
* 
* Examples:
* 
* <N>
* <Channels, Samples>
* <Batch, Channels, Time>
* 
* ============================================================================
  */

signalShapeSpecification
: LESS
signalDimensionList?
GREATER
;

signalDimensionList
: expression
(COMMA expression)*
COMMA?
;

/* ============================================================================

* 48. SIGNAL AXES
* ============================================================================
  */

signalAxisList
: signalAxis
(COMMA signalAxis)*
COMMA?
;

signalAxis
: expression
;

/* ============================================================================

* 49. CHANNEL SPECIFICATION
* ============================================================================
* 
* Channel count is semantic data.
* 
* No fixed channel limit is imposed.
* 
* ============================================================================
  */

signalChannelSpecification
: expression
;

/* ============================================================================

* 50. SAMPLE-RATE SPECIFICATION
* ============================================================================
* 
* The rate is an expression so that it can be:
* 
* literal
* constant
* symbolic
* generic
* computed
* runtime-derived
* 
* Units are semantic.
* 
* ============================================================================
  */

signalRateSpecification
: expression
;

/* ============================================================================

* 51. FREQUENCY SPECIFICATION
* ============================================================================
  */

signalFrequencySpecification
: expression
;

/* ============================================================================

* 52. PHASE SPECIFICATION
* ============================================================================
  */

signalPhaseSpecification
: expression
;

/* ============================================================================

* 53. AMPLITUDE SPECIFICATION
* ============================================================================
  */

signalAmplitudeSpecification
: expression
;

/* ============================================================================

* 54. TIME SPECIFICATION
* ============================================================================
  */

signalTimeSpecification
: expression
;

/* ============================================================================

* 55. WINDOW SIZE
* ============================================================================
  */

signalWindowSize
: expression
;

/* ============================================================================

* 56. WINDOW HOP
* ============================================================================
  */

signalWindowHop
: expression
;

/* ============================================================================

* 57. OVERLAP
* ============================================================================
  */

signalOverlap
: expression
;

/* ============================================================================

* 58. FILTER ORDER
* ============================================================================
  */

signalFilterOrder
: expression
;

/* ============================================================================

* 59. FILTER COEFFICIENTS
* ============================================================================
* 
* Coefficients are expressions and therefore may be:
* 
* literals
* arrays
* vectors
* tensors
* symbolic values
* generated values
* runtime values
* 
* ============================================================================
  */

signalFilterCoefficients
: expression
;

/* ============================================================================

* 60. TRANSFORM SIZE
* ============================================================================
  */

signalTransformSize
: expression
;

/* ============================================================================

* 61. TRANSFORM AXIS
* ============================================================================
  */

signalTransformAxis
: expression
;

/* ============================================================================

* 62. PADDING / BOUNDARY MODE
* ============================================================================
  */

signalBoundaryMode
: expression
;

/* ============================================================================

* 63. PRECISION
* ============================================================================
* 
* Precision is semantic.
* 
* It must not be interpreted as a hardware register width.
* 
* ============================================================================
  */

signalPrecision
: typeExpression
| expression
;

/* ============================================================================

* 64. DOMAIN
* ============================================================================
  */

signalDomainSpecification
: identifier
| qualifiedName
| expression
;

/* ============================================================================

* 65. SIGNAL METADATA
* ============================================================================
* 
* Metadata remains structural.
* 
* ============================================================================
  */

signalMetadata
: LBRACKET
signalMetadataEntryList?
RBRACKET
;

signalMetadataEntryList
: signalMetadataEntry
(COMMA signalMetadataEntry)*
COMMA?
;

signalMetadataEntry
: identifier
COLON
expression
;

/* ============================================================================

* 66. SIGNAL PROFILE EXPRESSION
* ============================================================================
  */

signalProfileExpression
: signalOperationInvocation
| signalReference
| signalMetadata
| expression
;

/* ============================================================================

* 67. SIGNAL OPERATION ATTRIBUTE
* ============================================================================
* 
* Attributes are expressions rather than fixed parser-level algorithm
* properties.
* 
* ============================================================================
  */

signalOperationAttribute
: identifier
COLON
expression
;

signalOperationAttributeList
: LBRACKET
signalOperationAttributeEntryList?
RBRACKET
;

signalOperationAttributeEntryList
: signalOperationAttribute
(COMMA signalOperationAttribute)*
COMMA?
;

/* ============================================================================

* 68. SIGNAL OPERATION WITH ATTRIBUTES
* ============================================================================
  */

signalAttributedOperation
: signalOperationName
signalTypeArgumentList?
signalOperationAttributeList?
LPAREN
signalArgumentList?
RPAREN
;

/* ============================================================================

* 69. SIGNAL OPERATION INVOCATION WITH ATTRIBUTES
* ============================================================================
* 
* Kept separate from the basic invocation so consumers can distinguish
* syntactic attributes without changing the generic operation model.
* 
* ============================================================================
  */

signalOperationWithAttributes
: signalAttributedOperation
;

/* ============================================================================

* 70. SIGNAL CONNECTION
* ============================================================================
* 
* A connection describes semantic flow between signal values.
* 
* It does not imply a physical wire or device connection.
* 
* ============================================================================
  */

signalConnection
: signalExpression
THIN_ARROW
signalExpression
;

/* ============================================================================

* 71. SIGNAL COMPOSITION
* ============================================================================
  */

signalComposition
: signalExpression
PIPE
signalExpression
;

/* ============================================================================

* 72. SIGNAL GRAPH
* ============================================================================
* 
* The graph is structural.
* 
* No node/edge count is hard-coded.
* 
* ============================================================================
  */

signalGraph
: signalGraphNode*
;

signalGraphNode
: signalConnection
| signalOperationInvocation
| signalComposition
;

/* ============================================================================

* 73. SIGNAL PARAMETER
* ============================================================================
  */

signalParameter
: identifier
COLON
expression
| expression
;

/* ============================================================================

* 74. SIGNAL PARAMETER LIST
* ============================================================================
  */

signalParameterList
: signalParameter
(COMMA signalParameter)*
COMMA?
;

/* ============================================================================

* 75. SIGNAL OPERATION SPECIFICATION
* ============================================================================
* 
* This rule is deliberately generic.
* 
* It can represent:
* 
* fft
* filter
* convolution
* correlation
* resample
* demodulate
* feature_extract
* custom::operation
* 
* without changing the parser.
* 
* ============================================================================
  */

signalOperationSpecification
: signalOperationName
signalTypeArgumentList?
signalOperationAttributeList?
LPAREN
signalParameterList?
RPAREN
;

/* ============================================================================

* 76. SIGNAL COMPUTATION REGION
* ============================================================================
* 
* A region may contain an unbounded sequence of signal-domain constructs.
* 
* The enclosing block syntax remains owned by the universal language grammar.
* 
* ============================================================================
  */

signalComputationRegion
: signalProcessingConstruct*
;

/* ============================================================================

* 77. SIGNAL COMPUTATION BODY
* ============================================================================
  */

signalComputationBody
: signalProcessingConstruct*
;

/* ============================================================================

* 78. SIGNAL SOURCE
* ============================================================================
  */

signalSource
: signalReference
| signalConstruction
| signalOperationInvocation
| expression
;

/* ============================================================================

* 79. SIGNAL SINK
* ============================================================================
  */

signalSink
: signalReference
| signalOperationInvocation
| expression
;

/* ============================================================================

* 80. SIGNAL TRANSFER
* ============================================================================
  */

signalTransfer
: signalSource
THIN_ARROW
signalSink
;

/* ============================================================================

* 81. SIGNAL PROCESSING NETWORK
* ============================================================================
  */

signalProcessingNetwork
: signalNetworkElement*
;

signalNetworkElement
: signalTransfer
| signalOperationInvocation
| signalComposition
;

/* ============================================================================

* 82. SIGNAL CHANNEL
* ============================================================================
  */

signalChannel
: signalReference
| signalOperationInvocation
| expression
;

/* ============================================================================

* 83. MULTICHANNEL SIGNAL
* ============================================================================
* 
* The list is intentionally unbounded.
* 
* ============================================================================
  */

signalChannelList
: signalChannel
(COMMA signalChannel)*
COMMA?
;

/* ============================================================================

* 84. SIGNAL COLLECTION
* ============================================================================
  */

signalCollection
: LBRACKET
signalChannelList?
RBRACKET
;

/* ============================================================================

* 85. SIGNAL TRANSFORM CHAIN
* ============================================================================
  */

signalTransformChain
: signalTransformStage
(PIPE signalTransformStage)*
;

signalTransformStage
: signalOperationInvocation
| signalReference
| expression
;

/* ============================================================================

* 86. FILTER CHAIN
* ============================================================================
  */

signalFilterChain
: signalFilterStage
(PIPE signalFilterStage)*
;

signalFilterStage
: signalOperationInvocation
| signalReference
| expression
;

/* ============================================================================

* 87. ANALYSIS CHAIN
* ============================================================================
  */

signalAnalysisChain
: signalAnalysisStage
(PIPE signalAnalysisStage)*
;

signalAnalysisStage
: signalOperationInvocation
| signalReference
| expression
;

/* ============================================================================

* 88. STREAMING CHAIN
* ============================================================================
  */

signalStreamingChain
: signalStreamingStage
(PIPE signalStreamingStage)*
;

signalStreamingStage
: signalOperationInvocation
| signalReference
| expression
;

/* ============================================================================

* 89. SAMPLE DOMAIN
* ============================================================================
  */

signalSampleDomain
: identifier
| qualifiedName
| expression
;

/* ============================================================================

* 90. TIME DOMAIN
* ============================================================================
  */

signalTimeDomain
: identifier
| qualifiedName
| expression
;

/* ============================================================================

* 91. FREQUENCY DOMAIN
* ============================================================================
  */

signalFrequencyDomain
: identifier
| qualifiedName
| expression
;

/* ============================================================================

* 92. SIGNAL REPRESENTATION
* ============================================================================
  */

signalRepresentation
: identifier
| qualifiedName
| typeExpression
| expression
;

/* ============================================================================

* 93. SIGNAL STORAGE INTENT
* ============================================================================
* 
* This is semantic intent only.
* 
* It does not select memory hardware.
* 
* ============================================================================
  */

signalStorageIntent
: signalOperationInvocation
| expression
;

/* ============================================================================

* 94. SIGNAL STREAMING INTENT
* ============================================================================
  */

signalStreamingIntent
: signalOperationInvocation
| expression
;

/* ============================================================================

* 95. SIGNAL PARALLELISM INTENT
* ============================================================================
* 
* Parallelism is semantic intent.
* 
* No thread/core count is embedded.
* 
* ============================================================================
  */

signalParallelismIntent
: signalOperationInvocation
| expression
;

/* ============================================================================

* 96. SIGNAL RESOURCE INTENT
* ============================================================================
* 
* Resource requirements are owned by resources/.
* 
* This rule provides only a domain integration hook.
* 
* ============================================================================
  */

signalResourceIntent
: signalOperationInvocation
| expression
;

/* ============================================================================

* 97. SIGNAL CAPABILITY INTENT
* ============================================================================
  */

signalCapabilityIntent
: signalOperationInvocation
| expression
;

/* ============================================================================

* 98. SIGNAL QUALITY INTENT
* ============================================================================
* 
* Quality concepts may include:
* 
* SNR
* SINAD
* THD
* error tolerance
* accuracy
* stability
* 
* These remain semantic expressions.
* 
* ============================================================================
  */

signalQualityIntent
: signalOperationInvocation
| expression
;

/* ============================================================================

* 99. SIGNAL CORRECTNESS CONTRACT
* ============================================================================
* 
* Correctness conditions are expressions.
* 
* The semantic/contract system determines their interpretation.
* 
* ============================================================================
  */

signalCorrectnessContract
: expression
;

/* ============================================================================

* 100. SIGNAL EFFECT
* ============================================================================
* 
* Effects remain owned by effects/.
* 
* ============================================================================
  */

signalEffect
: expression
;

/* ============================================================================

* 101. SIGNAL RESULT
* ============================================================================
  */

signalResult
: expression
;

/* ============================================================================

* 102. SIGNAL OPERATION RESULT LIST
* ============================================================================
  */

signalResultList
: signalResult
(COMMA signalResult)*
COMMA?
;

/* ============================================================================

* 103. SIGNAL OPERATION RESULT SPECIFICATION
* ============================================================================
  */

signalResultSpecification
: signalResultList
;

/* ============================================================================

* 104. SIGNAL OPERATION CONTRACT
* ============================================================================
  */

signalOperationContract
: signalCorrectnessContract
| signalQualityIntent
| signalCapabilityIntent
| signalResourceIntent
;

/* ============================================================================

* 105. SIGNAL OPERATION DEFINITION
* ============================================================================
* 
* A definition names a signal operation while keeping implementation details
* outside the grammar.
* 
* ============================================================================
  */

signalOperationDefinition
: identifier
signalTypeArgumentList?
signalOperationAttributeList?
LPAREN
signalParameterList?
RPAREN
signalOperationBody?
;

signalOperationBody
: signalOperationContract*
;

/* ============================================================================

* 106. SIGNAL FUNCTION-LIKE DEFINITION
* ============================================================================
* 
* This is intentionally a domain hook rather than a replacement for the
* universal function grammar.
* 
* ============================================================================
  */

signalFunctionLikeDefinition
: identifier
LPAREN
signalParameterList?
RPAREN
THIN_ARROW
typeExpression
;

/* ============================================================================

* 107. SIGNAL METADATA VALUE
* ============================================================================
  */

signalMetadataValue
: expression
;

/* ============================================================================

* 108. SIGNAL UNIT
* ============================================================================
* 
* Unit names are semantic identifiers.
* 
* Examples:
* 
* Hz
* kHz
* MHz
* s
* ms
* us
* ns
* 
* The grammar does not create a fixed unit registry.
* 
* ============================================================================
  */

signalUnit
: identifier
| qualifiedName
;

/* ============================================================================

* 109. SIGNAL QUANTITY
* ============================================================================
* 
* A quantity is structurally represented as a value plus optional unit.
* 
* ============================================================================
  */

signalQuantity
: expression
signalUnit?
;

/* ============================================================================

* 110. SIGNAL AXIS SPECIFICATION
* ============================================================================
  */

signalAxisSpecification
: signalAxis
| signalAxisList
;

/* ============================================================================

* 111. SIGNAL INDEX SPECIFICATION
* ============================================================================
  */

signalIndexSpecification
: expression
;

/* ============================================================================

* 112. SIGNAL SLICE SPECIFICATION
* ============================================================================
* 
* The universal range syntax remains authoritative.
* 
* ============================================================================
  */

signalSliceSpecification
: expression
;

/* ============================================================================

* 113. SIGNAL SELECTION
* ============================================================================
  */

signalSelection
: signalReference
| signalOperationInvocation
| expression
;

/* ============================================================================

* 114. SIGNAL REDUCTION
* ============================================================================
  */

signalReduction
: signalOperationInvocation
| expression
;

/* ============================================================================

* 115. SIGNAL AGGREGATION
* ============================================================================
  */

signalAggregation
: signalOperationInvocation
| expression
;

/* ============================================================================

* 116. SIGNAL NORMALIZATION
* ============================================================================
  */

signalNormalization
: signalOperationInvocation
| expression
;

/* ============================================================================

* 117. SIGNAL FILTER DESIGN
* ============================================================================
  */

signalFilterDesign
: signalOperationInvocation
| expression
;

/* ============================================================================

* 118. SIGNAL TRANSFORM DESIGN
* ============================================================================
  */

signalTransformDesign
: signalOperationInvocation
| expression
;

/* ============================================================================

* 119. SIGNAL RESAMPLING
* ============================================================================
  */

signalResampling
: signalOperationInvocation
| expression
;

/* ============================================================================

* 120. SIGNAL INTERPOLATION
* ============================================================================
  */

signalInterpolation
: signalOperationInvocation
| expression
;

/* ============================================================================

* 121. SIGNAL EXTRAPOLATION
* ============================================================================
  */

signalExtrapolation
: signalOperationInvocation
| expression
;

/* ============================================================================

* 122. SIGNAL CONVOLUTION
* ============================================================================
  */

signalConvolution
: signalOperationInvocation
| expression
;

/* ============================================================================

* 123. SIGNAL CORRELATION
* ============================================================================
  */

signalCorrelation
: signalOperationInvocation
| expression
;

/* ============================================================================

* 124. SIGNAL MODULATION
* ============================================================================
  */

signalModulation
: signalOperationInvocation
| expression
;

/* ============================================================================

* 125. SIGNAL DEMODULATION
* ============================================================================
  */

signalDemodulation
: signalOperationInvocation
| expression
;

/* ============================================================================

* 126. SIGNAL DETECTION
* ============================================================================
  */

signalDetection
: signalOperationInvocation
| expression
;

/* ============================================================================

* 127. SIGNAL ESTIMATION
* ============================================================================
  */

signalEstimation
: signalOperationInvocation
| expression
;

/* ============================================================================

* 128. SIGNAL FEATURE EXTRACTION
* ============================================================================
  */

signalFeatureExtraction
: signalOperationInvocation
| expression
;

/* ============================================================================

* 129. SIGNAL NOISE MODEL
* ============================================================================
  */

signalNoiseModel
: signalOperationInvocation
| expression
;

/* ============================================================================

* 130. SIGNAL NOISE ANALYSIS
* ============================================================================
  */

signalNoiseAnalysis
: signalOperationInvocation
| expression
;

/* ============================================================================

* 131. SIGNAL SPECTRAL ANALYSIS
* ============================================================================
  */

signalSpectralAnalysis
: signalOperationInvocation
| expression
;

/* ============================================================================

* 132. SIGNAL TIME-FREQUENCY ANALYSIS
* ============================================================================
  */

signalTimeFrequencyAnalysis
: signalOperationInvocation
| expression
;

/* ============================================================================

* 133. SIGNAL STATISTICAL ANALYSIS
* ============================================================================
  */

signalStatisticalAnalysis
: signalOperationInvocation
| expression
;

/* ============================================================================

* 134. SIGNAL RECONSTRUCTION
* ============================================================================
  */

signalReconstruction
: signalOperationInvocation
| expression
;

/* ============================================================================

* 135. SIGNAL SYNTHESIS
* ============================================================================
  */

signalSynthesis
: signalOperationInvocation
| expression
;

/* ============================================================================

* 136. SIGNAL CONTROL
* ============================================================================
  */

signalControl
: signalOperationInvocation
| expression
;

/* ============================================================================

* 137. SIGNAL GENERATION
* ============================================================================
  */

signalGeneration
: signalOperationInvocation
| expression
;

/* ============================================================================

* 138. SIGNAL ACQUISITION
* ============================================================================
  */

signalAcquisition
: signalOperationInvocation
| expression
;

/* ============================================================================

* 139. SIGNAL OUTPUT
* ============================================================================
  */

signalOutput
: signalOperationInvocation
| expression
;

/* ============================================================================

* 140. SIGNAL INPUT
* ============================================================================
  */

signalInput
: signalOperationInvocation
| expression
;

/* ============================================================================

* 141. SIGNAL DOMAIN CONVERSION
* ============================================================================
  */

signalDomainConversion
: signalOperationInvocation
| expression
;

/* ============================================================================

* 142. SIGNAL VALIDATION
* ============================================================================
  */

signalValidation
: signalOperationInvocation
| expression
;

/* ============================================================================

* 143. SIGNAL COMPARISON
* ============================================================================
  */

signalComparison
: signalOperationInvocation
| expression
;

/* ============================================================================

* 144. SIGNAL PROPERTY
* ============================================================================
  */

signalProperty
: identifier
| qualifiedName
| expression
;

/* ============================================================================

* 145. SIGNAL PROPERTY ASSIGNMENT
* ============================================================================
  */

signalPropertyAssignment
: signalProperty
ASSIGN
expression
;

/* ============================================================================

* 146. SIGNAL PROPERTY LIST
* ============================================================================
  */

signalPropertyList
: signalPropertyAssignment
(COMMA signalPropertyAssignment)*
COMMA?
;

/* ============================================================================

* 147. SIGNAL CONFIGURATION
* ============================================================================
  */

signalConfiguration
: LBRACKET
signalPropertyList?
RBRACKET
;

/* ============================================================================

* 148. SIGNAL PROCESSING DIRECTIVE
* ============================================================================
* 
* Directives are semantic declarations, not compiler commands embedded into
* the parser.
* 
* ============================================================================
  */

signalProcessingDirective
: signalOperationInvocation
;

/* ============================================================================

* 149. SIGNAL PIPELINE CONFIGURATION
* ============================================================================
  */

signalPipelineConfiguration
: signalConfiguration?
;

/* ============================================================================

* 150. SIGNAL PIPELINE STAGE
* ============================================================================
  */

signalPipelineOperation
: signalOperationName
signalTypeArgumentList?
signalPipelineConfiguration?
LPAREN
signalArgumentList?
RPAREN
;

/* ============================================================================

* 151. SIGNAL PIPELINE WITH EXPLICIT STAGES
* ============================================================================
  */

signalExplicitPipeline
: signalPipelineOperation
(PIPE signalPipelineOperation)*
;

/* ============================================================================

* 152. SIGNAL PROCESSING GRAPH
* ============================================================================
  */

signalProcessingGraph
: signalProcessingGraphElement*
;

signalProcessingGraphElement
: signalExplicitPipeline
| signalTransfer
| signalOperationInvocation
;

/* ============================================================================

* 153. SIGNAL STREAM PROCESSING
* ============================================================================
  */

signalStreamProcessing
: signalStreamingChain
;

/* ============================================================================

* 154. SIGNAL BATCH PROCESSING
* ============================================================================
  */

signalBatchProcessing
: signalOperationInvocation
| signalPipelineExpression
;

/* ============================================================================

* 155. SIGNAL ONLINE PROCESSING
* ============================================================================
  */

signalOnlineProcessing
: signalStreamingChain
;

/* ============================================================================

* 156. SIGNAL OFFLINE PROCESSING
* ============================================================================
  */

signalOfflineProcessing
: signalBatchProcessing
;

/* ============================================================================

* 157. SIGNAL WINDOWING
* ============================================================================
  */

signalWindowing
: signalWindowExpression
;

/* ============================================================================

* 158. SIGNAL FILTERING
* ============================================================================
  */

signalFiltering
: signalFilterExpression
;

/* ============================================================================

* 159. SIGNAL TRANSFORMATION
* ============================================================================
  */

signalTransformation
: signalTransformExpression
;

/* ============================================================================

* 160. SIGNAL SPECTRUM PROCESSING
* ============================================================================
  */

signalSpectrumProcessing
: signalSpectrumExpression
;

/* ============================================================================

* 161. SIGNAL ANALYTICS
* ============================================================================
  */

signalAnalytics
: signalAnalysisExpression
;

/* ============================================================================

* 162. SIGNAL PROCESSING OPERATION CATEGORY
* ============================================================================
* 
* These categories are parser-level integration hooks.
* 
* They intentionally map to the same generic operation syntax.
* 
* ============================================================================
  */

signalOperationCategory
: signalAcquisition
| signalGeneration
| signalSamplingExpression
| signalWindowing
| signalFiltering
| signalTransformation
| signalConvolution
| signalCorrelation
| signalResampling
| signalInterpolation
| signalExtrapolation
| signalModulation
| signalDemodulation
| signalDetection
| signalEstimation
| signalFeatureExtraction
| signalSpectralAnalysis
| signalTimeFrequencyAnalysis
| signalStatisticalAnalysis
| signalNoiseAnalysis
| signalReconstruction
| signalSynthesis
| signalDomainConversion
| signalValidation
| signalComparison
;

/* ============================================================================

* 163. SIGNAL OPERATION DISPATCH
* ============================================================================
  */

signalOperation
: signalOperationSpecification
| signalOperationInvocation
| signalAttributedOperation
;

/* ============================================================================

* 164. SIGNAL DOMAIN VALUE
* ============================================================================
  */

signalDomainValue
: signalExpression
;

/* ============================================================================

* 165. SIGNAL DOMAIN BODY
* ============================================================================
  */

signalDomainBody
: signalProcessingConstruct*
;

/* ============================================================================

* 166. SIGNAL DOMAIN CONSTRUCT LIST
* ============================================================================
  */

signalConstructList
: signalProcessingConstruct*
;

/* ============================================================================

* 167. SIGNAL DOMAIN ENTRY
* ============================================================================
  */

signalProcessingEntry
: signalProcessingConstruct
;

/* ============================================================================

* 168. SIGNAL DOMAIN VALUE LIST
* ============================================================================
  */

signalValueList
: signalExpression
(COMMA signalExpression)*
COMMA?
;

/* ============================================================================

* 169. SIGNAL DOMAIN ARGUMENT LIST
* ============================================================================
  */

signalExpressionList
: signalExpression
(COMMA signalExpression)*
COMMA?
;

/* ============================================================================

* 170. SIGNAL PROCESSING INVOCATION
* ============================================================================
  */

signalProcessingInvocation
: signalOperationInvocation
;

/* ============================================================================

* 171. SIGNAL PROCESSING CALL
* ============================================================================
  */

signalProcessingCall
: signalOperationInvocation
;

/* ============================================================================

* 172. SIGNAL PROCESSING REFERENCE
* ============================================================================
  */

signalProcessingReference
: signalReference
;

/* ============================================================================

* 173. SIGNAL PROCESSING VALUE
* ============================================================================
  */

signalProcessingValue
: signalExpression
;

/* ============================================================================

* 174. SIGNAL PROCESSING TYPE
* ============================================================================
  */

signalProcessingType
: typeExpression
;

/* ============================================================================

* 175. SIGNAL PROCESSING SHAPE
* ============================================================================
  */

signalProcessingShape
: signalShapeSpecification
;

/* ============================================================================

* 176. SIGNAL PROCESSING RATE
* ============================================================================
  */

signalProcessingRate
: signalRateSpecification
;

/* ============================================================================

* 177. SIGNAL PROCESSING FREQUENCY
* ============================================================================
  */

signalProcessingFrequency
: signalFrequencySpecification
;

/* ============================================================================

* 178. SIGNAL PROCESSING PHASE
* ============================================================================
  */

signalProcessingPhase
: signalPhaseSpecification
;

/* ============================================================================

* 179. SIGNAL PROCESSING AMPLITUDE
* ============================================================================
  */

signalProcessingAmplitude
: signalAmplitudeSpecification
;

/* ============================================================================

* 180. SIGNAL PROCESSING AXIS
* ============================================================================
  */

signalProcessingAxis
: signalAxisSpecification
;

/* ============================================================================

* 181. SIGNAL PROCESSING WINDOW SIZE
* ============================================================================
  */

signalProcessingWindowSize
: signalWindowSize
;

/* ============================================================================

* 182. SIGNAL PROCESSING FILTER ORDER
* ============================================================================
  */

signalProcessingFilterOrder
: signalFilterOrder
;

/* ============================================================================

* 183. SIGNAL PROCESSING TRANSFORM SIZE
* ============================================================================
  */

signalProcessingTransformSize
: signalTransformSize
;

/* ============================================================================

* 184. SIGNAL PROCESSING PRECISION
* ============================================================================
  */

signalProcessingPrecision
: signalPrecision
;

/* ============================================================================

* 185. SIGNAL PROCESSING UNIT
* ============================================================================
  */

signalProcessingUnit
: signalUnit
;

/* ============================================================================

* 186. SIGNAL PROCESSING QUANTITY
* ============================================================================
  */

signalProcessingQuantity
: signalQuantity
;

/* ============================================================================

* 187. SIGNAL PROCESSING METADATA
* ============================================================================
  */

signalProcessingMetadata
: signalMetadata
;

/* ============================================================================

* 188. SIGNAL PROCESSING ATTRIBUTE
* ============================================================================
  */

signalProcessingAttribute
: signalOperationAttribute
;

/* ============================================================================

* 189. SIGNAL PROCESSING CONTRACT
* ============================================================================
  */

signalProcessingContract
: signalOperationContract
;

/* ============================================================================

* 190. SIGNAL PROCESSING RESOURCE
* ============================================================================
  */

signalProcessingResource
: signalResourceIntent
;

/* ============================================================================

* 191. SIGNAL PROCESSING CAPABILITY
* ============================================================================
  */

signalProcessingCapability
: signalCapabilityIntent
;

/* ============================================================================

* 192. SIGNAL PROCESSING QUALITY
* ============================================================================
  */

signalProcessingQuality
: signalQualityIntent
;

/* ============================================================================

* 193. SIGNAL PROCESSING EFFECT
* ============================================================================
  */

signalProcessingEffect
: signalEffect
;

/* ============================================================================

* 194. SIGNAL PROCESSING RESULT
* ============================================================================
  */

signalProcessingResult
: signalResult
;

/* ============================================================================

* 195. SIGNAL PROCESSING CONNECTION
* ============================================================================
  */

signalProcessingConnection
: signalConnection
;

/* ============================================================================

* 196. SIGNAL PROCESSING GRAPH
* ============================================================================
  */

signalProcessingGraphEntry
: signalProcessingGraph
;

/* ============================================================================

* 197. SIGNAL PROCESSING NETWORK
* ============================================================================
  */

signalProcessingNetworkEntry
: signalProcessingNetwork
;

/* ============================================================================

* 198. SIGNAL PROCESSING DOMAIN
* ============================================================================
  */

signalProcessingDomain
: signalProcessingConstruct
;

/* ============================================================================

* 199. SIGNAL PROCESSING ROOT
* ============================================================================
* 
* This is the public domain-level root consumed by Classical.
* 
* ============================================================================
  */

signalProcessing
: signalProcessingConstruct
;
/*

* ============================================================================
* END OF FILE
* ============================================================================
* 
* INTEGRATION CONTRACT
* ============================================================================
* 
* 1. Classical composition
* 
* "grammar/classical/classical.g4" should import:
* 
* SignalProcessing
* 
* and add:
* 
* | signalProcessingConstruct
* 
* to its classical domain dispatcher where appropriate.
* 
* 
* 2. Universal parser composition
* 
* "grammar/antlr/ZamaniParser.g4" already imports "Classical".
* 
* No direct import of SignalProcessing belongs in ZamaniParser.g4.
* 
* 
* 3. Lexer
* 
* No lexer modification is required for this file.
* 
* Existing canonical tokens are sufficient.
* 
* 
* 4. AST
* 
* Signal operations lower into the existing domain-neutral operation model.
* 
* No SignalOperation enum or signal-specific backend AST is required.
* 
* 
* 5. Semantic analysis
* 
* Semantic analysis owns:
* 
* operation resolution
* signal typing
* shape analysis
* rate/unit analysis
* channel analysis
* transform legality
* filter legality
* numerical constraints
* capability requirements
* resource requirements
* 
* 
* 6. IR
* 
* Signal-processing constructs lower into the repository's canonical
* classical/numerical/data semantic representation.
* 
* This grammar does not create a new signal-processing IR.
* 
* 
* 7. Quantum integration
* 
* Signal-processing values may participate in hybrid quantum-classical
* programs.
* 
* If a signal operation eventually interacts with quantum computation,
* semantic lowering determines the appropriate representation and preserves
* the established "quantum::ir" boundary.
* 
* This grammar does not create a hybrid quantum IR.
* 
* 
* 8. HDL integration
* 
* Signal-processing intent may eventually lower to HDL/hardware accelerators,
* but this grammar does not describe physical wires, register widths,
* clock counts, FPGA resources, or implementation topology.
* 
* 
* 9. Resource integration
* 
* Resource and capability requirements remain downstream.
* 
* Example:
* 
* requires capability("signal.acceleration")
* 
* is semantic intent, not hardware selection.
* 
* 
* 10. Compile/runtime integration
* 
* Compilation may choose:
* 
* scalar
* vector
* multicore
* GPU
* FPGA
* ASIC
* accelerator
* distributed
* future
* 
* according to available capabilities and resources.
* 
* 
* 11. Rust integration
* 
* Rust 1.97 / 1.97.1, Rust 2021.
* 
* No unsafe Rust is required by this grammar.
* 
* 
* 12. Tests
* 
* The corresponding tests must cover:
* 
* positive
* negative
* boundary
* scalability
* determinism
* compatibility
* cross-domain
* round-trip/source-preservation
* 
* Examples that should parse after composition include:
* 
* signal::fft(input)
* 
* dsp::fft(input, size)
* 
* dsp::filter(input, coefficients)
* 
* dsp::convolve(a, b)
* 
* dsp::correlate(a, b)
* 
* dsp::resample(input, rate)
* 
* dsp::window(input, size, hop)
* 
* dsp::spectrogram(input, window)
* 
* input
*     |> dsp::filter(coefficients)
*     |> dsp::fft()
*     |> dsp::analyze()
* 
* signal::transform(input, transform)
* 
* signal::operation(input, parameter: value)
* 
* The parser must NOT decide whether an operation such as "fft" exists.
* 
* That is semantic/library/capability resolution.
* 
* ============================================================================
* 
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when:
* 
* [x] Existing lexical vocabulary is reused.
* [x] No signal-specific lexer token is required.
* [x] No hardware limit is encoded.
* [x] No signal length limit is encoded.
* [x] No channel limit is encoded.
* [x] No filter-order limit is encoded.
* [x] No FFT-size limit is encoded.
* [x] No vector-width limit is encoded.
* [x] No machine topology is encoded.
* [x] Signal operations are open-world.
* [x] Qualified operation names are supported.
* [x] Named arguments are supported.
* [x] Type parameters are structurally supported.
* [x] Shape dimensions are expressions.
* [x] Pipelines are unbounded.
* [x] Signal literals are recursively structured.
* [x] Semantic validation remains downstream.
* [x] No signal-processing IR is introduced.
* [x] quantum::ir remains the canonical quantum boundary.
* [x] No Rust actions are present.
* [x] No unsafe Rust is required.
* [x] Cross-domain integration is defined.
* [x] Resource/capability integration is defined.
* 
* Remaining repository-level work is composition/testing:
* 
* - import this grammar from Classical;
* - expose signalProcessingConstruct through Classical;
* - add conformance tests;
* - validate ANTLR generation;
* - validate Rust parser equivalence;
* - add grammar-to-AST semantic tests.
* 
* Those steps are integration work and do not require reopening this file
* unless the canonical shared grammar contracts themselves change.
* 
* ============================================================================
  */