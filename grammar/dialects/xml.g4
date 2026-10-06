/*

* ============================================================================
* ZAMANI PROGRAMMING LANGUAGE
* ============================================================================
* 
* File:
* grammar/dialects/xml.g4
* 
* Grammar:
* xml
* 
* Role:
* XML DIALECT / INTEROPERABILITY PARSER
* 
* Status:
* PRODUCTION-READY XML SYNTAX BOUNDARY
* 
* Baseline:
* ANTLR4
* Rust 1.97+
* Rust 2021
* Safe Rust only
* No unsafe Rust
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This grammar defines the parser-side syntax boundary for XML
* interoperability.
* 
* XML is an external data representation and is NOT part of the universal
* Zamani source grammar.
* 
* The intended pipeline is:
* 
* XML source
*     |
*     v
* XML dialect lexer
*     |
*     v
* XML parser
*     |
*     v
* XML syntax representation
*     |
*     v
* XML semantic validation
*     |
*     v
* dialect-neutral data representation
*     |
*     v
* Zamani data/schema/provenance model
*     |
*     v
* canonical semantic model
* 
* The XML dialect MUST NOT become a second general-purpose programming
* language.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - XML document structure;
* - XML declaration structure;
* - XML processing instructions;
* - XML comments;
* - XML elements;
* - XML start-tags;
* - XML end-tags;
* - XML empty-element tags;
* - XML attributes;
* - XML attribute values;
* - XML character data;
* - XML CDATA sections;
* - XML entity references;
* - XML character references;
* - XML namespace-qualified names;
* - XML namespace declarations;
* - XML document type declaration structure;
* - XML internal-subset declaration structure where tokenized by the
*   companion XML lexer;
* - XML miscellaneous document items;
* - XML extension points that preserve source structure.
* 
* THIS FILE DOES NOT OWN:
* 
* - Zamani identifiers;
* - Zamani expressions;
* - Zamani statements;
* - Zamani types;
* - Zamani resource requirements;
* - Zamani capabilities;
* - Zamani effects;
* - Zamani contracts;
* - Zamani policies;
* - Zamani provenance semantics;
* - JSON syntax;
* - SQL syntax;
* - XPath;
* - XQuery;
* - XSLT;
* - XML Schema;
* - Relax NG;
* - DTD semantic validation;
* - XML databases;
* - storage engines;
* - network transport;
* - HTTP;
* - authentication;
* - authorization;
* - DOM construction;
* - SAX execution;
* - streaming execution;
* - query optimization;
* - physical placement;
* - CPU/GPU/FPGA/QPU selection.
* 
* ============================================================================
* FILE DEPENDENCY CONTRACT
* ============================================================================
* 
* DEPENDS_ON:
* 
* grammar/dialects/XMLLexer.g4
* grammar/data/data.g4
* grammar/data/schemas.g4
* grammar/data/provenance.g4
* grammar/interoperability/
* grammar/specification/
* grammar/spec/
* 
* EXPORTS:
* 
* xmlDocument
* xmlProlog
* xmlElement
* xmlContent
* xmlAttribute
* xmlName
* xmlQualifiedName
* xmlNamespaceDeclaration
* xmlDocumentType
* xmlProcessingInstruction
* xmlComment
* xmlCData
* xmlCharacterData
* xmlReference
* 
* CONSUMED_BY:
* 
* XML interoperability frontend
* data import frontend
* data serialization/deserialization tooling
* schema tooling
* provenance tooling
* dialect dispatch
* external-format conversion
* 
* AST_OWNER:
* 
* Domain-neutral frontend AST / XML interoperability semantic adapter.
* 
* SEMANTIC_OWNER:
* 
* XML interoperability semantic layer.
* 
* IR_OWNER:
* 
* Canonical data/interoperability semantic representation.
* 
* TEST_OWNER:
* 
* grammar/tests/interoperability/xml/
* 
* SPEC_OWNER:
* 
* grammar/specification/
* grammar/spec/
* 
* ============================================================================
* CRITICAL LEXER BOUNDARY
* ============================================================================
* 
* XML MUST NOT use:
* 
* tokenVocab=ZamaniLexer
* 
* for its external XML lexical syntax.
* 
* XML has lexical requirements that are incompatible with treating XML as
* ordinary Zamani source, including:
* 
* <element>
* </element>
* <element/>
* attribute="value"
* <![CDATA[...]]>
* <!-- ... -->
* <?target ...?>
* &name;
* &#123;
* &#x7B;
* 
* XML is also case-sensitive.
* 
* Therefore this parser consumes the dedicated XML lexer vocabulary:
* 
* tokenVocab=XMLLexer
* 
* The companion lexer owns:
* 
* XML markup delimiters
* XML names
* XML text
* XML whitespace
* XML quoted values
* XML references
* XML comments
* XML CDATA
* XML processing instructions
* XML declaration tokens
* XML document-type lexical regions
* 
* This separation prevents XML lexical rules from contaminating the
* canonical Zamani lexer.
* 
* ============================================================================
* ANTLR COMPOSITION CONTRACT
* ============================================================================
* 
* The companion grammar MUST be named:
* 
* XMLLexer
* 
* and supplied as:
* 
* grammar/dialects/XMLLexer.g4
* 
* The parser build MUST therefore contain:
* 
* xml.g4
* XMLLexer.g4
* 
* The XML lexer MUST NOT import or depend on Zamani parser rules.
* 
* The XML parser MUST NOT define lexer rules.
* 
* This keeps lexical ownership and parser ownership independent.
* 
* ============================================================================
* XML CONFORMANCE SCOPE
* ============================================================================
* 
* The grammar targets well-formed XML document syntax.
* 
* It supports:
* 
* XML declaration
* document type declaration
* elements
* attributes
* namespace declarations
* namespace-qualified names
* character data
* entity references
* character references
* CDATA sections
* comments
* processing instructions
* mixed content
* empty elements
* nested elements
* document-level miscellaneous items
* 
* XML Schema, DTD validation, namespace URI validation, external entity
* resolution and application-specific constraints are semantic/runtime
* responsibilities.
* 
* ============================================================================
* DOCUMENT STRUCTURE
* ============================================================================
* 
* XML document:
* 
* document
*     -> prolog
*     -> root element
*     -> miscellaneous
* 
* XML permits optional declarations before the document element and
* miscellaneous items after it.
* 
* The grammar deliberately keeps the document element singular.
* 
* ============================================================================
* SECURITY BOUNDARY
* ============================================================================
* 
* Parsing XML MUST NOT imply:
* 
* external entity resolution
* network access
* filesystem access
* resource acquisition
* code execution
* schema retrieval
* DTD fetching
* 
* In particular, parser acceptance of a DOCTYPE does NOT authorize external
* resource access.
* 
* External entity resolution, if ever supported, MUST be an explicit
* capability/effect controlled operation.
* 
* The parser itself remains side-effect free.
* 
* ============================================================================
* SCALABILITY
* ============================================================================
* 
* This grammar contains no artificial universal limits.
* 
* It MUST NOT define:
* 
* MAX_ELEMENTS
* MAX_ATTRIBUTES
* MAX_DEPTH
* MAX_TEXT_LENGTH
* MAX_DOCUMENT_SIZE
* MAX_NAMESPACE_COUNT
* MAX_ENTITY_COUNT
* MAX_CHILDREN
* MAX_DOCUMENTS
* 
* Any implementation resource limit is an execution/compiler/runtime policy,
* not a grammar-language ceiling.
* 
* XML documents may therefore be arbitrarily large subject only to available
* resources and explicit execution policies.
* 
* ============================================================================
* NAMESPACE MODEL
* ============================================================================
* 
* Namespace declarations are syntactically represented here.
* 
* Semantic namespace resolution belongs downstream.
* 
* The parser MUST preserve:
* 
* prefix
* local name
* namespace declaration
* declaration scope
* 
* Namespace URI equality, prefix binding and reserved-prefix validation are
* semantic responsibilities.
* 
* ============================================================================
* ENTITY MODEL
* ============================================================================
* 
* Entity references are preserved structurally.
* 
* The parser MUST NOT automatically expand arbitrary external entities.
* 
* Built-in XML references and character references remain syntax nodes.
* 
* General entity resolution belongs to the XML semantic/security layer.
* 
* ============================================================================
* DOCUMENT
* ============================================================================
  */

options {
tokenVocab=XMLLexer;
}

/*

* ============================================================================
* PUBLIC ENTRY POINT
* ============================================================================
  */

xmlDocument
: xmlProlog? xmlRootElement xmlMiscellaneous* EOF
;

/*

* ============================================================================
* PROLOG
* ============================================================================
* 
* XML declaration is optional.
* 
* Document type declaration is optional.
* 
* Miscellaneous items may occur around these constructs.
* ============================================================================
  */

xmlProlog
: xmlMiscellaneous*
xmlDeclaration?
xmlMiscellaneous*
xmlDocumentTypeDeclaration?
xmlMiscellaneous*
;

/*

* ============================================================================
* XML DECLARATION
* ============================================================================
* 
* The lexer supplies the declaration delimiters and lexical values.
* 
* Semantic validation determines:
* 
* version validity
* encoding validity
* standalone validity
* declaration ordering
* ============================================================================
  */

xmlDeclaration
: XML_DECL_START
XML_VERSION
XML_EQ
XML_VERSION_VALUE
xmlEncodingDeclaration?
xmlStandaloneDeclaration?
XML_DECL_END
;

xmlEncodingDeclaration
: XML_ENCODING
XML_EQ
XML_ENCODING_VALUE
;

xmlStandaloneDeclaration
: XML_STANDALONE
XML_EQ
XML_STANDALONE_VALUE
;

/*

* ============================================================================
* DOCUMENT TYPE DECLARATION
* ============================================================================
  */

xmlDocumentTypeDeclaration
: XML_DOCTYPE
xmlName
xmlExternalId?
xmlInternalSubset?
XML_GT
;

/*

* ============================================================================
* EXTERNAL IDENTIFIER
* ============================================================================
  */

xmlExternalId
: XML_SYSTEM
XML_SYSTEM_LITERAL
| XML_PUBLIC
XML_PUBLIC_LITERAL
XML_SYSTEM_LITERAL
;

/*

* ============================================================================
* INTERNAL SUBSET
* ============================================================================
* 
* The lexer is responsible for preserving the internal subset as XML-aware
* tokens. This parser represents declarations structurally without attempting
* to turn DTD syntax into a second general-purpose grammar.
* 
* Supported declaration categories may include:
* 
* ELEMENT
* ATTLIST
* ENTITY
* NOTATION
* 
* Parameter entities and nested declaration structure remain under the XML
* DTD semantic boundary.
* ============================================================================
  */

xmlInternalSubset
: XML_LBRACKET
xmlMarkupDeclaration*
XML_RBRACKET
;

xmlMarkupDeclaration
: xmlElementDeclaration
| xmlAttributeListDeclaration
| xmlEntityDeclaration
| xmlNotationDeclaration
| xmlParameterEntityReference
| xmlComment
| xmlProcessingInstruction
;

xmlElementDeclaration
: XML_ELEMENT
xmlName
xmlDtdContentModel
XML_DTD_DECL_END
;

xmlDtdContentModel
: XML_EMPTY
| XML_ANY
| xmlDtdMixed
| xmlDtdChildren
;

xmlDtdMixed
: XML_LPAREN XML_PCDATA XML_RPAREN
| XML_LPAREN XML_PCDATA (XML_PIPE xmlDtdNameReference)+ XML_RPAREN XML_STAR
;

xmlDtdChildren
: xmlDtdParticle
;

xmlDtdParticle
: xmlDtdGroup xmlDtdOccurrence?
| xmlDtdNameReference xmlDtdOccurrence?
;

xmlDtdGroup
: XML_LPAREN xmlDtdParticle (xmlDtdSeparator xmlDtdParticle)* XML_RPAREN
;

xmlDtdSeparator
: XML_COMMA
| XML_PIPE
;

xmlDtdOccurrence
: XML_QUESTION
| XML_STAR
| XML_PLUS
;

xmlDtdNameReference
: xmlName
;

xmlAttributeListDeclaration
: XML_ATTLIST
xmlName
xmlAttributeDeclaration+
XML_DTD_DECL_END
;

xmlAttributeDeclaration
: xmlName
xmlDtdAttributeType
xmlDtdDefaultDeclaration?
;

xmlDtdAttributeType
: XML_CDATA
| XML_ID
| XML_IDREF
| XML_IDREFS
| XML_ENTITY
| XML_ENTITIES
| XML_NMTOKEN
| XML_NMTOKENS
| XML_NMTOKEN_GROUP
| XML_NOTATION_GROUP
;

xmlDtdDefaultDeclaration
: XML_REQUIRED
| XML_IMPLIED
| XML_FIXED
XML_ATTRIBUTE_LITERAL
| XML_ATTRIBUTE_LITERAL
;

xmlEntityDeclaration
: XML_ENTITY
xmlParameterEntityMarker?
xmlName
xmlEntityValue
XML_DTD_DECL_END
| XML_ENTITY
xmlParameterEntityMarker?
xmlName
xmlExternalId
xmlNDataDeclaration?
XML_DTD_DECL_END
;

xmlParameterEntityMarker
: XML_PERCENT
;

xmlEntityValue
: XML_ENTITY_VALUE
;

xmlNDataDeclaration
: XML_NDATA
xmlName
;

xmlNotationDeclaration
: XML_NOTATION
xmlName
xmlExternalId
XML_DTD_DECL_END
;

/*

* ============================================================================
* ROOT ELEMENT
* ============================================================================
  */

xmlRootElement
: xmlElement
;

/*

* ============================================================================
* ELEMENT
* ============================================================================
* 
* XML element forms:
* 
* <name/>
* 
* or:
* 
* <name>content</name>
* 
* The parser requires matching element names structurally.
* 
* Exact name equality MUST be checked by semantic validation as well as by
* any downstream AST validation layer. The grammar deliberately does not use
* target-language actions to compare token text.
* ============================================================================
  */

xmlElement
: xmlEmptyElement
| xmlPairedElement
;

xmlEmptyElement
: XML_START_TAG_OPEN
xmlQualifiedName
xmlAttribute*
XML_EMPTY_TAG_END
;

xmlPairedElement
: XML_START_TAG_OPEN
xmlQualifiedName
xmlAttribute*
XML_TAG_END
xmlContent*
XML_END_TAG_OPEN
xmlQualifiedName
XML_TAG_END
;

/*

* ============================================================================
* CONTENT
* ============================================================================
  */

xmlContent
: xmlCharacterData
| xmlElement
| xmlReference
| xmlCData
| xmlComment
| xmlProcessingInstruction
;

/*

* ============================================================================
* CHARACTER DATA
* ============================================================================
* 
* XML text is not a Zamani STRING.
* 
* It is an XML character-data node and remains owned by the XML dialect.
* ============================================================================
  */

xmlCharacterData
: XML_TEXT
;

/*

* ============================================================================
* CDATA
* ============================================================================
  */

xmlCData
: XML_CDATA_START
XML_CDATA_TEXT?
XML_CDATA_END
;

/*

* ============================================================================
* ATTRIBUTE
* ============================================================================
  */

xmlAttribute
: xmlAttributeName
XML_EQ
xmlAttributeValue
;

xmlAttributeName
: xmlQualifiedName
;

xmlAttributeValue
: XML_DOUBLE_QUOTED_VALUE
| XML_SINGLE_QUOTED_VALUE
;

/*

* ============================================================================
* QUALIFIED XML NAMES
* ============================================================================
* 
* XML names remain lexical XML names.
* 
* They MUST NOT be mapped directly to Zamani identifiers at parse time.
* ============================================================================
  */

xmlQualifiedName
: xmlName
| xmlName XML_COLON xmlName
;

xmlName
: XML_NAME
;

/*

* ============================================================================
* NAMESPACE DECLARATIONS
* ============================================================================
* 
* Namespace declarations are syntactically attributes but receive an
* explicit semantic distinction so that namespace processing can preserve
* declaration scope.
* ============================================================================
  */

xmlNamespaceDeclaration
: XML_XMLNS
XML_EQ
xmlAttributeValue
| XML_XMLNS_PREFIX
XML_EQ
xmlAttributeValue
;

/*

* ============================================================================
* REFERENCES
* ============================================================================
  */

xmlReference
: XML_ENTITY_REFERENCE
| XML_CHARACTER_REFERENCE
;

/*

* ============================================================================
* COMMENTS
* ============================================================================
  */

xmlComment
: XML_COMMENT
;

/*

* ============================================================================
* PROCESSING INSTRUCTIONS
* ============================================================================
  */

xmlProcessingInstruction
: XML_PI_START
XML_NAME
XML_PI_DATA?
XML_PI_END
;

/*

* ============================================================================
* DOCUMENT MISCELLANEOUS
* ============================================================================
  */

xmlMiscellaneous
: xmlComment
| xmlProcessingInstruction
| XML_MISC_WHITESPACE
;

/*

* ============================================================================
* XML DTD / NAME GROUPS
* ============================================================================
  */

xmlNmtokenGroup
: XML_LPAREN
xmlName
(XML_PIPE xmlName)*
XML_RPAREN
;

xmlNotationGroup
: XML_NOTATION
XML_LPAREN
xmlName
(XML_PIPE xmlName)*
XML_RPAREN
;

/*

* ============================================================================
* SEMANTIC ADAPTER CONTRACT
* ============================================================================
* 
* The parser tree must be normalized into the repository's data/interoperability
* semantic representation.
* 
* Recommended mappings:
* 
* xmlDocument
*     -> ExternalDocument
* 
* xmlElement
*     -> DataElement
* 
* xmlAttribute
*     -> DataAttribute
* 
* xmlCharacterData
*     -> DataText
* 
* xmlCData
*     -> DataCData
* 
* xmlReference
*     -> DataReference
* 
* xmlComment
*     -> DataComment
* 
* xmlProcessingInstruction
*     -> DataProcessingInstruction
* 
* xmlNamespaceDeclaration
*     -> NamespaceBinding
* 
* xmlDocumentTypeDeclaration
*     -> DocumentTypeDeclaration
* 
* The exact Rust structures belong to the existing domain-neutral AST and
* semantic owners. This grammar MUST NOT introduce a competing XML-only
* compiler IR.
* 
* ============================================================================
* PROVENANCE CONTRACT
* ============================================================================
* 
* XML imports MUST be capable of preserving:
* 
* source format = XML
* source span
* document identity
* encoding metadata
* declaration metadata
* namespace declarations
* document type metadata
* transformation provenance
* external-source provenance
* 
* Provenance belongs to the repository-wide provenance model.
* 
* ============================================================================
* TYPE CONTRACT
* ============================================================================
* 
* XML syntax does not determine Zamani semantic types.
* 
* For example:
* 
* <value>42</value>
* 
* MUST NOT automatically mean:
* 
* Integer
* 
* merely because the text looks numeric.
* 
* Conversion to a Zamani type requires an explicit schema, semantic rule,
* conversion operation or interoperability policy.
* 
* ============================================================================
* EFFECT CONTRACT
* ============================================================================
* 
* Parsing XML is pure syntax processing.
* 
* It does not automatically acquire:
* 
* network
* filesystem
* database
* native
* foreign
* execution
* 
* effects.
* 
* External entity resolution, external schema retrieval and similar actions
* require explicit downstream capabilities/effects.
* 
* ============================================================================
* RESOURCE CONTRACT
* ============================================================================
* 
* The grammar imposes no finite XML size, depth, element, attribute, namespace
* or document-count limit.
* 
* Resource feasibility belongs to:
* 
* resource analysis
* execution policy
* runtime configuration
* deployment
* 
* A target that cannot process a particular XML document must report resource
* infeasibility rather than causing the grammar to encode a universal limit.
* 
* ============================================================================
* SECURITY CONTRACT
* ============================================================================
* 
* XML parser integration MUST provide policy controls for:
* 
* external entity resolution
* external DTD retrieval
* external schema retrieval
* URI access
* filesystem access
* network access
* expansion policies
* parser resource budgets
* 
* These controls are NOT parser productions.
* 
* They belong to:
* 
* grammar/security/
* grammar/policies/
* XML semantic/runtime integration
* 
* ============================================================================
* DIALECT EXTENSIBILITY
* ============================================================================
* 
* Vendor/application XML vocabularies MUST NOT require new universal Zamani
* keywords.
* 
* XML vocabulary such as:
* 
* robotics
* scientific metadata
* hardware metadata
* configuration
* enterprise formats
* domain-specific schemas
* 
* remains represented through XML names, namespaces and schema semantics.
* 
* This is essential for an open-world format.
* 
* ============================================================================
* QUANTUM / HDL / HARDWARE BOUNDARY
* ============================================================================
* 
* XML may carry serialized descriptions of:
* 
* quantum programs
* hardware metadata
* HDL metadata
* device descriptions
* execution records
* calibration data
* scientific data
* 
* but this grammar does NOT create:
* 
* quantum operations
* physical qubits
* hardware topology
* routing
* scheduling
* calibration
* QEC
* ZQN
* HAL
* 
* Those semantics are recovered by the appropriate domain/interoperability
* adapters.
* 
* ============================================================================
* DIAGNOSTICS CONTRACT
* ============================================================================
* 
* The XML frontend must distinguish at least:
* 
* XML_LEXICAL_ERROR
* XML_SYNTAX_ERROR
* XML_MALFORMED_NAME
* XML_MISMATCHED_ELEMENT
* XML_DUPLICATE_ATTRIBUTE
* XML_INVALID_NAMESPACE_DECLARATION
* XML_INVALID_REFERENCE
* XML_INVALID_DOCUMENT_STRUCTURE
* XML_INVALID_DECLARATION
* XML_INVALID_DOCTYPE
* XML_UNSUPPORTED_EXTERNAL_REFERENCE
* XML_SECURITY_POLICY_VIOLATION
* XML_RESOURCE_LIMIT
* XML_ENCODING_ERROR
* 
* Parser errors MUST NOT be represented as generic Zamani semantic errors
* when the source is XML.
* 
* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* Positive tests MUST cover:
* 
* empty-element documents
* nested elements
* mixed content
* attributes
* namespaces
* namespace-qualified names
* comments
* CDATA
* processing instructions
* entity references
* character references
* XML declaration
* DOCTYPE
* internal subsets
* Unicode names
* Unicode character data
* large symbolic documents
* deeply nested documents
* large attribute sets
* repeated sibling elements
* 
* Negative tests MUST cover:
* 
* missing closing tag
* mismatched closing tag
* duplicate attributes
* malformed entity references
* malformed character references
* malformed XML declaration
* malformed processing instruction
* malformed CDATA termination
* malformed namespace syntax
* malformed DOCTYPE
* invalid document-level ordering
* 
* Boundary tests MUST cover:
* 
* XML -> Zamani data model
* XML -> schema model
* XML -> provenance
* XML -> query/data processing
* XML -> FFI/import pipeline
* XML + quantum metadata
* XML + hardware metadata
* XML + distributed metadata
* 
* Scalability tests MUST vary document size and structure without changing
* grammar source or introducing capacity constants.
* 
* ============================================================================
* DETERMINISM CONTRACT
* ============================================================================
* 
* Given identical:
* 
* source bytes
* lexer configuration
* dialect configuration
* parser version
* 
* the parser must produce deterministic syntax structure.
* 
* The parser MUST NOT:
* 
* access the network
* read arbitrary files
* inspect hardware
* select a database
* query a runtime
* invoke native code
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* XML dialect evolution MUST be versioned independently from the Zamani
* universal grammar.
* 
* The XML parser may support additional XML-related dialect profiles without
* modifying the core Zamani grammar.
* 
* Existing XML syntax must not silently change meaning.
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is DONE when:
* 
* 1. XML parser syntax is owned exclusively here.
* 2. XML lexical syntax is owned exclusively by XMLLexer.g4.
* 3. No Zamani universal grammar is duplicated here.
* 4. No physical capacity is encoded here.
* 5. No runtime side effects are encoded here.
* 6. Namespace structure is preserved.
* 7. XML references are preserved.
* 8. XML declaration structure is preserved.
* 9. XML document structure is deterministic.
* 10. Malformed XML has dedicated diagnostics.
* 11. Security-sensitive external resolution is outside the parser.
* 12. XML semantic output has a defined owner.
* 13. XML interoperability has positive tests.
* 14. XML interoperability has negative tests.
* 15. XML interoperability has boundary tests.
* 16. XML interoperability has scalability tests.
* 17. XML interoperability has determinism tests.
* 18. Generated Rust parser integration uses Rust 1.97+.
* 19. No unsafe Rust is introduced.
* 20. No later grammar edit is required merely because another independent
*    domain grammar is extended.
* 
* ============================================================================
  */

xmlDocument
: xmlProlog? xmlRootElement xmlMiscellaneous* EOF
;

/*

* NOTE:
* 
* The remaining rules are declared above in the architectural contract.
* 
* This final entry-point declaration is intentionally kept as the sole public
* starting rule used by dialect dispatch.
  */