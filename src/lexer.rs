//! Zenith Lexical Analyzer (Lexer)
//! Lifetime-correct: borrows source from SourceMap, zero leaks, zero runtime keyword cost.

use crate::source_map::{FileId, BytePos, Span}; // Correctly import Span from source_map
use std::collections::HashMap;
use std::sync::LazyLock;
use unicode_xid::UnicodeXID;

pub struct Lexer<'a> {
    file: FileId,
    source: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    current_char_offset: usize,
    current_line: usize,
    current_column: usize,
    keywords: &'static phf::Map<&'static str, TokenType>,
    errors: Vec<LexerError>,
    eof_emitted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    pub message: String,
    pub span: Span,
}

impl<'a> Lexer<'a> {
    pub fn new(file: FileId, source_code: &'a str) -> Self {
        Lexer {
            file,
            source: source_code,
            chars: source_code.chars().peekable(),
            current_char_offset: 0,
            current_line: 1,
            current_column: 1,
            keywords: &KEYWORDS,
            errors: Vec::new(),
            eof_emitted: false,
        }
    }

    // Perfect hash map for keywords. Zero runtime cost.
    static KEYWORDS: phf::Map<&'static str, TokenType> = phf::phf_map! {
        "quantum" => TokenType::KeywordQuantum,
        "nano" => TokenType::KeywordNano,
        "effect" => TokenType::KeywordEffect,
        "handle" => TokenType::KeywordHandle,
        "language" => TokenType::KeywordLanguage,
        "type" => TokenType::KeywordType,
        "kind" => TokenType::KeywordKind,
        "sort" => TokenType::KeywordSort,
        "prop" => TokenType::KeywordProp,
        "linear" => TokenType::KeywordLinear,
        "affine" => TokenType::KeywordAffine,
        "unsafe" => TokenType::KeywordUnsafe,
        "remember" => TokenType::KeywordRemember,
        "recall" => TokenType::KeywordRecall,
        "learn" => TokenType::KeywordLearn,
        "wisdom" => TokenType::KeywordWisdom,
        "zamani" => TokenType::KeywordZamani,
        "sasa" => TokenType::KeywordSasa,
        "ancestral" => TokenType::KeywordAncestral,
        "consensus" => TokenType::KeywordConsensus,
        "observe" => TokenType::KeywordObserve,
        "living_doc" => TokenType::KeywordLivingDoc,
        "temporal_learn" => TokenType::KeywordTemporalLearn,
        "fn" => TokenType::KeywordFn,
        "let" => TokenType::KeywordLet,
        "if" => TokenType::KeywordIf,
        "else" => TokenType::KeywordElse,
        "return" => TokenType::KeywordReturn,
        "true" => TokenType::KeywordTrue,
        "false" => TokenType::KeywordFalse,
        "mts" => TokenType::KeywordMts,
        // --- Added from feedback ---
        "quantum_circuit" => TokenType::KeywordQuantumCircuit,
        "nano_agent" => TokenType::KeywordNanoAgent,
        "History" => TokenType::KeywordHistory, // Type name
        "ConsensusTrue" => TokenType::KeywordConsensusTrue, // Type name
        "InterMemory" => TokenType::KeywordInterMemory, // Type name
        "Superposition" => TokenType::KeywordSuperposition, // Type name
        "Entangled" => TokenType::KeywordEntangled, // Type name
        "QMeasured" => TokenType::KeywordQMeasured, // Type name
        "Archaeve" => TokenType::KeywordArchaeve, // Type name or concept
        "with" => TokenType::KeywordWith, // For "with effects {E1, E2}"
        "module" => TokenType::KeywordModule,
        "import" => TokenType::KeywordImport,
        "export" => TokenType::KeywordExport,
        "struct" => TokenType::KeywordStruct,
        "enum" => TokenType::KeywordEnum,
        "trait" => TokenType::KeywordTrait,
        "impl" => TokenType::KeywordImpl,
        "match" => TokenType::KeywordMatch,
        "case" => TokenType::KeywordCase,
        "default" => TokenType::KeywordDefault,
        "as" => TokenType::KeywordAs,
        "is" => TokenType::KeywordIs,
        "mut" => TokenType::KeywordMut,
        "ref" => TokenType::KeywordRef,
        "val" => TokenType::KeywordVal,
        "var" => TokenType::KeywordVar,
        "static" => TokenType::KeywordStatic,
        "const" => TokenType::KeywordConst,
        "await" => TokenType::KeywordAwait,
        "async" => TokenType::KeywordAsync,
        "yield" => TokenType::KeywordYield,
        "go" => TokenType::KeywordGo,
        "defer" => TokenType::KeywordDefer,
        "package" => TokenType::KeywordPackage,
        "private" => TokenType::KeywordPrivate,
        "public" => TokenType::KeywordPublic,
        "protected" => TokenType::KeywordProtected,
        "interface" => TokenType::KeywordInterface,
        "extends" => TokenType::KeywordExtends,
        "implements" => TokenType::KeywordImplements,
        "new" => TokenType::KeywordNew,
        "this" => TokenType::KeywordThis,
        "super" => TokenType::KeywordSuper,
        "null" => TokenType::KeywordNull,
        "self" => TokenType::KeywordSelf,
        "void" => TokenType::KeywordVoid,
        "unit" => TokenType::KeywordUnit,
        "any" => TokenType::KeywordAny,
        "never" => TokenType::KeywordNever,
        "sizeof" => TokenType::KeywordSizeof,
        "typeof" => TokenType::KeywordTypeof,
        "alignof" => TokenType::KeywordAlignof,
        "macro" => TokenType::KeywordMacro,
        "alias" => TokenType::KeywordAlias,
        "operator" => TokenType::KeywordOperator,
        "where" => TokenType::KeywordWhere,
        "catch" => TokenType::KeywordCatch,
        "try" => TokenType::KeywordTry,
        "throw" => TokenType::KeywordThrow,
        "finally" => TokenType::KeywordFinally,
        "panic" => TokenType::KeywordPanic,
        "assert" => TokenType::KeywordAssert,
        "debug" => TokenType::KeywordDebug,
        "test" => TokenType::KeywordTest,
        "benchmark" => TokenType::KeywordBenchmark,
        "profile" => TokenType::KeywordProfile,
        "extern" => TokenType::KeywordExtern,
        "inline" => TokenType::KeywordInline,
        "no_mangle" => TokenType::KeywordNoMangle,
        "thread_local" => TokenType::KeywordThreadLocal,
        "volatile" => TokenType::KeywordVolatile,
        "atomic" => TokenType::KeywordAtomic,
        "sync" => TokenType::KeywordSync,
        "send" => TokenType::KeywordSend,
        "recv" => TokenType::KeywordRecv,
        "channel" => TokenType::KeywordChannel,
        "select" => TokenType::KeywordSelect,
        "spawn" => TokenType::KeywordSpawn,
        "join" => TokenType::KeywordJoin,
        "guard" => TokenType::KeywordGuard,
        "resource" => TokenType::KeywordResource,
        "acquire" => TokenType::KeywordAcquire,
        "release" => TokenType::KeywordRelease,
        "handle_error" => TokenType::KeywordHandleError,
        "resume" => TokenType::KeywordResume,
        "suspend" => TokenType::KeywordSuspend,
        "event" => TokenType::KeywordEvent,
        "delegate" => TokenType::KeywordDelegate,
        "signal" => TokenType::KeywordSignal,
        "slot" => TokenType::KeywordSlot,
        "attribute" => TokenType::KeywordAttribute,
        "pragma" => TokenType::KeywordPragma,
        "aspect" => TokenType::KeywordAspect,
        "advice" => TokenType::KeywordAdvice,
        "pointcut" => TokenType::KeywordPointcut,
        "around" => TokenType::KeywordAround,
        "before" => TokenType::KeywordBefore,
        "after" => TokenType::KeywordAfter,
        "abstract" => TokenType::KeywordAbstract,
        "final" => TokenType::KeywordFinal,
        "override" => TokenType::KeywordOverride,
        "virtual" => TokenType::KeywordVirtual,
        "sealed" => TokenType::KeywordSealed,
        "dynamic" => TokenType::KeywordDynamic,
        "static_if" => TokenType::KeywordStaticIf,
        "static_for" => TokenType::KeywordStaticFor,
        "union" => TokenType::KeywordUnion,
        "alias_type" => TokenType::KeywordAliasType,
        "type_family" => TokenType::KeywordTypeFamily,
        "dependent_type" => TokenType::KeywordDependentType,
        "proof" => TokenType::KeywordProof,
        "theorem" => TokenType::KeywordTheorem,
        "axiom" => TokenType::KeywordAxiom,
        "assume" => TokenType::KeywordAssume,
        "guarantee" => TokenType::KeywordGuarantee,
        "invariant" => TokenType::KeywordInvariant,
        "precondition" => TokenType::KeywordPrecondition,
        "postcondition" => TokenType::KeywordPostcondition,
        "forall" => TokenType::KeywordForall,
        "exists" => TokenType::KeywordExists,
        "lambda" => TokenType::KeywordLambda,
        "mu" => TokenType::KeywordMu,
        "sigma_type" => TokenType::KeywordSigmaType,
        "pi_type" => TokenType::KeywordPiType,
        "universe" => TokenType::KeywordUniverse,
        "coercion" => TokenType::KeywordCoercion,
        "subsume" => TokenType::KeywordSubsume,
        "delegate_to" => TokenType::KeywordDelegateTo,
        "transmute" => TokenType::KeywordTransmute,
        "inline_asm" => TokenType::KeywordInlineAsm,
        "raw_ptr" => TokenType::KeywordRawPtr,
        "native" => TokenType::KeywordNative,
        "host" => TokenType::KeywordHost,
        "device" => TokenType::KeywordDevice,
        "parallel" => TokenType::KeywordParallel,
        "distributed" => TokenType::KeywordDistributed,
        "actor" => TokenType::KeywordActor,
        "message" => TokenType::KeywordMessage,
        "spawn_task" => TokenType::KeywordSpawnTask,
        "await_task" => TokenType::KeywordAwaitTask,
        "future" => TokenType::KeywordFuture,
        "promise" => TokenType::KeywordPromise,
        "result" => TokenType::KeywordResult,
        "option" => TokenType::KeywordOption,
        "error_type" => TokenType::KeywordErrorType,
        "exception" => TokenType::KeywordException,
        "raise" => TokenType::KeywordRaise,
        "catch_all" => TokenType::KeywordCatchAll,
        "finally_catch" => TokenType::KeywordFinallyCatch,
        "debugger" => TokenType::KeywordDebugger,
        "instrument" => TokenType::KeywordInstrument,
        "telemetry" => TokenType::KeywordTelemetry,
        "log" => TokenType::KeywordLog,
        "trace" => TokenType::KeywordTrace,
        "collect" => TokenType::KeywordCollect,
        "dispose" => TokenType::KeywordDispose,
        "finalize" => TokenType::KeywordFinalize,
        "drop_resource" => TokenType::KeywordDropResource,
        "free" => TokenType::KeywordFree,
        "alloc_global" => TokenType::KeywordAllocGlobal,
        "alloc_local" => TokenType::KeywordAllocLocal,
        "dealloc" => TokenType::KeywordDealloc,
        "memory_map" => TokenType::KeywordMemoryMap,
        "region" => TokenType::KeywordRegion,
        "slab" => TokenType::KeywordSlab,
        "arena" => TokenType::KeywordArena,
        "garbage_collect" => TokenType::KeywordGarbageCollect,
        "reference_count" => TokenType::KeywordReferenceCount,
        "arc_ptr" => TokenType::KeywordArcPtr,
        "rc_ptr" => TokenType::KeywordRcPtr,
        "weak_ptr" => TokenType::KeywordWeakPtr,
        "move_val" => TokenType::KeywordMoveVal,
        "copy_val" => TokenType::KeywordCopyVal,
        "clone_val" => TokenType::KeywordCloneVal,
        "ptr_add" => TokenType::KeywordPtrAdd,
        "ptr_sub" => TokenType::KeywordPtrSub,
        "atomic_load" => TokenType::KeywordAtomicLoad,
        "atomic_store" => TokenType::KeywordAtomicStore,
        "atomic_cas" => TokenType::KeywordAtomicCas,
        "fence" => TokenType::KeywordFence,
        "acquire_release" => TokenType::KeywordAcquireRelease,
        "relaxed" => TokenType::KeywordRelaxed,
        "seq_cst" => TokenType::KeywordSeqCst,
        "unordered" => TokenType::KeywordUnordered,
        "ordered" => TokenType::KeywordOrdered,
        "read_only" => TokenType::KeywordReadOnly,
        "write_only" => TokenType::KeywordWriteOnly,
        "read_write" => TokenType::KeywordReadWrite,
        "exclusive" => TokenType::KeywordExclusive,
        "shared" => TokenType::KeywordShared,
        "volatile_read" => TokenType::KeywordVolatileRead,
        "volatile_write" => TokenType::KeywordVolatileWrite,
        "barrier" => TokenType::KeywordBarrier,
        "memory_barrier" => TokenType::KeywordMemoryBarrier,
        "full_barrier" => TokenType::KeywordFullBarrier,
        "read_barrier" => TokenType::KeywordReadBarrier,
        "write_barrier" => TokenType::KeywordWriteBarrier,
        "data_barrier" => TokenType::KeywordDataBarrier,
        "isa" => TokenType::KeywordIsa,
        "extension" => TokenType::KeywordExtension,
        "builtin" => TokenType::KeywordBuiltin,
        "intrinsic" => TokenType::KeywordIntrinsic,
        "compiler_fence" => TokenType::KeywordCompilerFence,
        "unreachable_unchecked" => TokenType::KeywordUnreachableUnchecked,
        "likely" => TokenType::KeywordLikely,
        "unlikely" => TokenType::KeywordUnlikely,
    };

    fn make_span(&self, start_offset: usize, start_line: usize, start_column: usize) -> Span {
        Span {
            file: self.file,
            start: BytePos(start_offset as u32),
            end: BytePos(self.current_char_offset as u32),
            line: start_line,
            column: start_column,
        }
    }

    fn read_char_and_advance_pos(&mut self) -> Option<char> {
        let c = self.chars.next();
        if let Some(ch) = c {
            if ch == '\n' {
                self.current_line += 1;
                self.current_column = 1;
            } else {
                self.current_column += 1;
            }
            self.current_char_offset += ch.len_utf8();
        }
        c
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn peek_char_n(&mut self, n: usize) -> Option<char> {
        self.chars.clone().nth(n - 1).copied()
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            let start_offset = self.current_char_offset;
            self.skip_whitespace();
            self.skip_comments();
            self.skip_whitespace();
            if self.current_char_offset == start_offset {
                break;
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() {
                self.read_char_and_advance_pos();
            } else {
                break;
            }
        }
    }

    fn skip_comments(&mut self) {
        if self.peek_char() == Some(&'/') && self.peek_char_n(2) == Some('/') {
            let start_span = self.make_span(self.current_char_offset, self.current_line, self.current_column);
            self.read_char_and_advance_pos(); // /
            self.read_char_and_advance_pos(); // /
            while let Some(&c) = self.peek_char() {
                if c == '\n' {
                    self.read_char_and_advance_pos();
                    return;
                }
                self.read_char_and_advance_pos();
            }
        } else if self.peek_char() == Some(&'/') && self.peek_char_n(2) == Some('*') {
            let start_span = self.make_span(self.current_char_offset, self.current_line, self.current_column);
            self.read_char_and_advance_pos(); // /
            self.read_char_and_advance_pos(); // *
            loop {
                match self.read_char_and_advance_pos() {
                    Some('*') if self.peek_char() == Some(&'/') => {
                        self.read_char_and_advance_pos();
                        return;
                    }
                    Some(_) => {}
                    None => {
                        self.errors.push(LexerError {
                            message: "Unterminated multi-line comment.".to_string(),
                            span: self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
                        });
                        return;
                    }
                }
            }
        }
    }

    fn read_identifier_or_keyword(&mut self, first_char: char) -> String {
        let mut ident = String::from(first_char);
        while let Some(&c) = self.peek_char() {
            if UnicodeXID::is_xid_continue(c) || c == '_' {
                ident.push(self.read_char_and_advance_pos().unwrap());
            } else {
                break;
            }
        }
        ident
    }

    fn read_number(&mut self, first_digit: char) -> String {
        let mut num = String::from(first_digit);
        let mut has_decimal = false;
        while let Some(&c) = self.peek_char() {
            if c.is_digit(10) {
                num.push(self.read_char_and_advance_pos().unwrap());
            } else if c == '.' && !has_decimal && self.peek_char_n(2).map_or(false, |n| n.is_digit(10)) {
                num.push(self.read_char_and_advance_pos().unwrap());
                has_decimal = true;
            } else {
                break;
            }
        }
        num
    }

    fn read_string_literal_content(&mut self, start_span: Span) -> String {
        let mut literal_content = String::new();
        while let Some(&c) = self.peek_char() {
            if c == '"' {
                break;
            }
            if c == '\n' {
                self.errors.push(LexerError {
                    message: "Unterminated string literal.".to_string(),
                    span: start_span,
                });
                break;
            }
            if c == '\\' {
                self.read_char_and_advance_pos();
                match self.read_char_and_advance_pos() {
                    Some('n') => literal_content.push('\n'),
                    Some('t') => literal_content.push('\t'),
                    Some('r') => literal_content.push('\r'),
                    Some('\\') => literal_content.push('\\'),
                    Some('"') => literal_content.push('"'),
                    Some(''') => literal_content.push('''),
                    Some('0') => literal_content.push('\0'),
                    Some('u') => {
                        self.errors.push(LexerError {
                            message: "Unicode escape \\u{XXXX} not implemented yet.".to_string(),
                            span: start_span,
                        });
                    }
                    Some(other) => {
                        self.errors.push(LexerError {
                            message: format!("Invalid escape sequence '\\{}'.", other),
                            span: start_span,
                        });
                    }
                    None => break,
                }
            } else {
                literal_content.push(self.read_char_and_advance_pos().unwrap());
            }
        }
        literal_content
    }

    fn read_char_literal_content(&mut self, start_span: Span) -> String {
        let mut literal_content = String::new();
        if let Some(&c) = self.peek_char() {
            if c == '\\' {
                self.read_char_and_advance_pos();
                if let Some(escaped_char) = self.read_char_and_advance_pos() {
                    match escaped_char {
                        'n' => literal_content.push('\n'),
                        't') => literal_content.push('\t'),
                        'r' => literal_content.push('\r'),
                        '\\') => literal_content.push('\\'),
                        '"') => literal_content.push('"'),
                        ''' => literal_content.push('''),
                        other => {
                            self.errors.push(LexerError {
                                message: format!("Invalid escape sequence '\\{}'.", other),
                                span: start_span,
                            });
                        }
                    }
                }
            } else if c != ''' {
                literal_content.push(self.read_char_and_advance_pos().unwrap());
            }
        }
        if literal_content.len() != 1 {
            self.errors.push(LexerError {
                message: "Character literal must contain exactly one character.".to_string(),
                span: start_span,
            });
        }
        literal_content
    }

    fn handle_quantum_literal(&mut self, start_span: Span) -> Option<Token> {
        self.read_char_and_advance_pos(); // consume space after '|'
        let state_start = self.current_char_offset;
        while let Some(&c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' || c == '+' || c == '-' {
                self.read_char_and_advance_pos();
            } else {
                break;
            }
        }
        let state_str = &self.source[state_start..self.current_char_offset];
        if self.peek_char() == Some(&'⟩') {
            self.read_char_and_advance_pos();
            Some(Token::new(
                TokenType::QuantumLiteral,
                format!("|{}⟩", state_str),
                self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
            ))
        } else {
            self.errors.push(LexerError {
                message: "Malformed quantum Dirac literal: expected '⟩'.".to_string(),
                span: self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
            });
            None
        }
    }

    fn handle_nano_annotation(&mut self, start_span: Span) -> Option<Token> {
        let anno_start = self.current_char_offset;
        while let Some(&c) = self.peek_char() {
            if c.is_alphabetic() {
                self.read_char_and_advance_pos();
            } else {
                break;
            }
        }
        let annotation = &self.source[anno_start..self.current_char_offset];
        if (annotation == "atom" || annotation == "molecule") && self.peek_char() == Some(&'(') {
            self.read_char_and_advance_pos(); // '('
            let mut nesting = 1;
            while nesting > 0 {
                match self.read_char_and_advance_pos() {
                    Some('(') => nesting += 1, // fixed: was ')'
                    Some(')') => nesting -= 1,
                    Some(_) => {}
                    None => {
                        self.errors.push(LexerError {
                            message: format!("Unterminated nano annotation '{}'.", annotation),
                            span: self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
                        });
                        return None;
                    }
                }
            }
            Some(Token::new(
                TokenType::NanoAnnotation,
                self.source[start_span.start.0 as usize..self.current_char_offset].to_string(),
                self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
            ))
        } else {
            self.errors.push(LexerError {
                message: format!("Malformed nano annotation '@{}'.", annotation),
                span: self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
            });
            None
        }
    }

    fn handle_mts_literal(&mut self, start_span: Span) -> Option<Token> {
        self.read_char_and_advance_pos(); // t
        self.read_char_and_advance_pos(); // s
        self.read_char_and_advance_pos(); // [
        let num_start = self.current_char_offset;
        while let Some(&c) = self.peek_char() {
            if c.is_digit(10) {
                self.read_char_and_advance_pos();
            } else {
                break;
            }
        }
        if self.peek_char() == Some(&']') {
            self.read_char_and_advance_pos();
            Some(Token::new(
                TokenType::MTSLiteral,
                self.source[start_span.start.0 as usize..self.current_char_offset].to_string(),
                self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
            ))
        } else {
            self.errors.push(LexerError {
                message: "Malformed MTS literal: expected ']'.".to_string(),
                span: self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
            });
            None
        }
    }

    fn handle_directive(&mut self, start_span: Span) -> Option<Token> {
        let name_start = self.current_char_offset;
        while let Some(&c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                self.read_char_and_advance_pos();
            } else {
                break;
            }
        }
        let name = &self.source[name_start..self.current_char_offset];
        Some(Token::new(
            TokenType::Directive,
            format!("#{}", name),
            self.make_span(start_span.start.0 as usize, start_span.line, start_span.column),
        ))
    }

    pub fn get_errors(&self) -> &[LexerError] {
        &self.errors
    }
}

// --- Token & TokenType Definitions ---
pub mod tokens {
    // Use Span from source_map directly
    use crate::source_map::{FileId, BytePos, Span};

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum TokenType {
        Assign, Plus, Minus, Star, Slash,
        LParen, RParen, LBrace, RBrace, LBracket, RBracket,
        Semicolon, Colon, Comma, Dot, Pipe, Caret,
        Equals, NotEquals, LT, GT, LTE, GTE, Bang,
        BitwiseAnd, LogicalAnd, LogicalOr,
        Identifier, Integer, Float, String, Char,
        QuantumLiteral, NanoAnnotation, MTSLiteral,
        KeywordFn, KeywordLet, KeywordIf, KeywordElse, KeywordReturn,
        KeywordTrue, KeywordFalse,
        KeywordQuantum, KeywordNano, KeywordEffect, KeywordHandle,
        KeywordLanguage, KeywordType, KeywordKind, KeywordSort, KeywordProp,
        KeywordLinear, KeywordAffine, KeywordUnsafe,
        KeywordRemember, KeywordRecall, KeywordLearn, KeywordWisdom,
        KeywordZamani, KeywordSasa, KeywordAncestral, KeywordConsensus,
        KeywordObserve, KeywordLivingDoc, KeywordTemporalLearn,
        KeywordMts,
        // --- Added from feedback ---
        KeywordQuantumCircuit,
        KeywordNanoAgent,
        KeywordHistory,
        KeywordConsensusTrue,
        KeywordInterMemory,
        KeywordSuperposition,
        KeywordEntangled,
        KeywordQMeasured,
        KeywordArchaeve,
        KeywordWith,
        KeywordModule, KeywordImport, KeywordExport,
        KeywordStruct, KeywordEnum, KeywordTrait, KeywordImpl,
        KeywordMatch, KeywordCase, KeywordDefault,
        KeywordAs, KeywordIs, KeywordMut, KeywordRef, KeywordVal, KeywordVar,
        KeywordStatic, KeywordConst, KeywordAwait, KeywordAsync, KeywordYield,
        KeywordGo, KeywordDefer, KeywordPackage,
        KeywordPrivate, KeywordPublic, KeywordProtected,
        KeywordInterface, KeywordExtends, KeywordImplements,
        KeywordNew, KeywordThis, KeywordSuper, KeywordNull, KeywordSelf,
        KeywordVoid, KeywordUnit, KeywordAny, KeywordNever,
        KeywordSizeof, KeywordTypeof, KeywordAlignof, KeywordMacro, KeywordAlias,
        KeywordOperator, KeywordWhere, KeywordCatch, KeywordTry, KeywordThrow,
        KeywordFinally, KeywordPanic, KeywordAssert, KeywordDebug, KeywordTest,
        KeywordBenchmark, KeywordProfile, KeywordExtern, KeywordInline,
        KeywordNoMangle, KeywordThreadLocal, KeywordVolatile, KeywordAtomic,
        KeywordSync, KeywordSend, KeywordRecv, KeywordChannel, KeywordSelect,
        KeywordSpawn, KeywordJoin, KeywordGuard, KeywordResource,
        KeywordAcquire, KeywordRelease, KeywordHandleError,
        KeywordResume, KeywordSuspend, KeywordEvent, KeywordDelegate,
        KeywordSignal, KeywordSlot, KeywordAttribute, KeywordPragma,
        KeywordAspect, KeywordAdvice, KeywordPointcut, KeywordAround,
        KeywordBefore, KeywordAfter, KeywordAbstract, KeywordFinal,
        KeywordOverride, KeywordVirtual, KeywordSealed, KeywordDynamic,
        KeywordStaticIf, KeywordStaticFor, KeywordUnion, KeywordAliasType,
        KeywordTypeFamily, KeywordDependentType, KeywordProof, KeywordTheorem,
        KeywordAxiom, KeywordAssume, KeywordGuarantee, KeywordInvariant,
        KeywordPrecondition, KeywordPostcondition, KeywordForall, KeywordExists,
        KeywordLambda, KeywordMu, KeywordSigmaType, KeywordPiType,
        KeywordUniverse, KeywordCoercion, KeywordSubsume, KeywordDelegateTo,
        KeywordTransmute, KeywordInlineAsm, KeywordRawPtr, KeywordNative,
        KeywordHost, KeywordDevice, KeywordParallel, KeywordDistributed,
        KeywordActor, KeywordMessage, KeywordSpawnTask, KeywordAwaitTask,
        KeywordFuture, KeywordPromise, KeywordResult, KeywordOption,
        KeywordErrorType, KeywordException, KeywordRaise, KeywordCatchAll,
        KeywordFinallyCatch, KeywordDebugger, KeywordInstrument, KeywordTelemetry,
        KeywordLog, KeywordTrace, KeywordCollect, KeywordDispose, KeywordFinalize,
        KeywordDropResource, KeywordFree, KeywordAllocGlobal, KeywordAllocLocal,
        KeywordDealloc, KeywordMemoryMap, KeywordRegion, KeywordSlab, KeywordArena,
        KeywordGarbageCollect, KeywordReferenceCount, KeywordArcPtr, KeywordRcPtr,
        KeywordWeakPtr, KeywordMoveVal, KeywordCopyVal, KeywordCloneVal,
        KeywordPtrAdd, KeywordPtrSub, KeywordAtomicLoad, TokenType::KeywordAtomicStore,
        KeywordAtomicCas, KeywordFence, KeywordAcquireRelease, KeywordRelaxed,
        KeywordSeqCst, KeywordUnordered, KeywordOrdered, KeywordReadOnly,
        KeywordWriteOnly, KeywordReadWrite, KeywordExclusive, KeywordShared,
        KeywordVolatileRead, KeywordVolatileWrite, KeywordBarrier,
        KeywordMemoryBarrier, KeywordFullBarrier, KeywordReadBarrier,
        KeywordWriteBarrier, KeywordDataBarrier, KeywordIsa, KeywordExtension,
        KeywordBuiltin, KeywordIntrinsic, KeywordCompilerFence,
        KeywordUnreachableUnchecked, KeywordLikely, KeywordUnlikely,
        PiSymbol, SigmaSymbol, // Unicode Pi and Sigma
        Directive, Illegal, EOF,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Token {
        pub token_type: TokenType,
        pub literal: String,
        pub span: Span,
    }

    impl Token {
        pub fn new(token_type: TokenType, literal: impl Into<String>, span: Span) -> Self {
            Token {
                token_type,
                literal: literal.into(),
                span,
            }
        }
    }
}
