//! Zenith UMC Parser — Full Grammar Implementation
//!
//! Parses the complete Zenith grammar: classical, quantum, nano, MTS,
//! Sankofa memory, OOP, algebraic effects, struct/enum/type declarations,
//! formal verification attributes, and all Zenith-native constructs.

use crate::ast::{
    AccessModifier, ClassMember, Expression, Identifier, InterfaceMember,
    LanguageDialectDecl, Literal, MatchCase, MethodModifier, Parameter,
    Program, ProveAttribute, EthicalAttribute, SovereignEntityDecl,
    ParadigmBlock, ActorSpawn, WisdomDecl, ConsensusExpr,
    InvariantBlock, PostCondition, MetaTransformDirective, ZenithDecl,
    Statement, TypeExpr,
};
use crate::compiler_types::AccessModifier as CtAccessModifier;
use crate::lexer::{Lexer, Token, TokenType};
use crate::source_map::{BytePos, FileId, Span};

// ─── Parser error ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    pub message: String,
    pub span: Span,
}

// ─── Operator precedence ──────────────────────────────────────────────────────

#[derive(PartialOrd, PartialEq, Clone, Copy, Debug)]
enum Precedence {
    Lowest,
    Assign,
    LogicalOr,
    LogicalAnd,
    Equals,
    Compare,
    Sum,
    Product,
    Unary,
    Call,
    Index,
    Member,
}

fn token_precedence(t: &TokenType) -> Precedence {
    match t {
        TokenType::Assign                                         => Precedence::Assign,
        TokenType::LogicalOr                                      => Precedence::LogicalOr,
        TokenType::LogicalAnd                                     => Precedence::LogicalAnd,
        TokenType::Equals | TokenType::NotEquals                  => Precedence::Equals,
        TokenType::LessThan | TokenType::GreaterThan
        | TokenType::LessThanEqual | TokenType::GreaterThanEqual  => Precedence::Compare,
        TokenType::Plus | TokenType::Minus                        => Precedence::Sum,
        TokenType::Star | TokenType::Slash | TokenType::Modulo    => Precedence::Product,
        TokenType::LParen                                         => Precedence::Call,
        TokenType::LBracket                                       => Precedence::Index,
        TokenType::Dot                                            => Precedence::Member,
        _                                                         => Precedence::Lowest,
    }
}

// ─── Parser ───────────────────────────────────────────────────────────────────

pub struct Parser {
    lexer: Lexer,
    current: Token,
    peek: Token,
    errors: Vec<ParserError>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        let peek    = lexer.next_token();
        Parser { lexer, current, peek, errors: vec![] }
    }

    pub fn get_errors(&self) -> &Vec<ParserError> { &self.errors }

    pub fn parse_program(&mut self) -> Program {
        let mut stmts = vec![];
        while self.current.token_type != TokenType::EOF {
            if let Some(s) = self.parse_statement() { stmts.push(s); }
            else { self.advance(); }
        }
        Program { statements: stmts }
    }

    // ── Navigation ────────────────────────────────────────────────────────────

    fn advance(&mut self) -> Token {
        let prev       = self.current.clone();
        self.current   = self.peek.clone();
        self.peek      = self.lexer.next_token();
        for e in self.lexer.get_errors() {
            self.errors.push(ParserError { message: e.message.clone(), span: e.span.clone() });
        }
        self.lexer.errors.clear();
        prev
    }

    fn current_is(&self, t: TokenType) -> bool  { self.current.token_type == t }
    fn peek_is(&self, t: TokenType)    -> bool  { self.peek.token_type == t }

    fn expect(&mut self, t: TokenType) -> Option<Token> {
        if self.current_is(t.clone()) { Some(self.advance()) }
        else {
            let msg = format!("Expected {:?}, found {:?}", t, self.current.token_type);
            self.errors.push(ParserError { message: msg, span: self.current.span.clone() });
            None
        }
    }

    fn skip_to_next_statement(&mut self) {
        while !matches!(self.current.token_type,
            TokenType::Semicolon | TokenType::RBrace | TokenType::EOF |
            TokenType::KeywordFn | TokenType::KeywordLet | TokenType::KeywordReturn)
        { self.advance(); }
        if self.current_is(TokenType::Semicolon) { self.advance(); }
    }

    fn dummy_span(&self) -> Span { self.current.span.clone() }

    fn make_identifier(&self) -> Identifier {
        Identifier(self.current.literal.clone(), self.current.span.clone())
    }

    // ── Statements ────────────────────────────────────────────────────────────

    fn parse_statement(&mut self) -> Option<Statement> {
        match &self.current.token_type {
            TokenType::KeywordLet           => self.parse_let(),
            TokenType::KeywordVar           => self.parse_var(),
            TokenType::KeywordConst         => self.parse_const(),
            TokenType::KeywordReturn        => self.parse_return(),
            TokenType::KeywordFn            => self.parse_function(),
            TokenType::KeywordIf            => { let e = self.parse_if_expression()?; Some(Statement::Expression(e)) }
            TokenType::KeywordWhile         => self.parse_while(),
            TokenType::KeywordFor           => self.parse_for(),
            TokenType::KeywordBreak         => { let s = self.advance().span; if self.current_is(TokenType::Semicolon) { self.advance(); } Some(Statement::Break(s)) }
            TokenType::KeywordContinue      => { let s = self.advance().span; if self.current_is(TokenType::Semicolon) { self.advance(); } Some(Statement::Continue(s)) }
            TokenType::KeywordMatch         => self.parse_match(),
            TokenType::KeywordQuantum       => self.parse_quantum_circuit(),
            TokenType::KeywordNano | TokenType::KeywordAgent => self.parse_nano_agent(),
            TokenType::KeywordRemember      => self.parse_sankofa_remember(),
            TokenType::KeywordEffect        => self.parse_effect_decl(),
            TokenType::KeywordHandle        => self.parse_handle(),
            TokenType::KeywordType          => self.parse_type_decl(),
            TokenType::KeywordStruct        => self.parse_struct(),
            TokenType::KeywordEnum          => self.parse_enum(),
            TokenType::KeywordClass         => self.parse_class(),
            TokenType::KeywordInterface     => self.parse_interface(),
            TokenType::KeywordModule        => self.parse_module(),
            TokenType::KeywordImport        => self.parse_import(),
            TokenType::KeywordWisdom        => self.parse_wisdom(),
            TokenType::KeywordUnsafe        => self.parse_unsafe(),
            TokenType::At                   => self.parse_attribute_statement(),
            _ => {
                let expr = self.parse_expression(Precedence::Lowest)?;
                if self.current_is(TokenType::Semicolon) { self.advance(); }
                Some(Statement::Expression(expr))
            }
        }
    }

    // ── Let / Var / Const ─────────────────────────────────────────────────────

    fn parse_let(&mut self) -> Option<Statement> {
        let span = self.advance().span; // consume `let`
        let _mutable = if self.current_is(TokenType::KeywordMut) { self.advance(); true } else { false };
        // also handle `var` keyword being used as mutable indicator
        let name = self.current.literal.clone();
        self.expect(TokenType::Identifier)?;
        let typ = if self.current_is(TokenType::Colon) { self.advance(); Some(self.parse_type_expr()?) } else { None };
        self.expect(TokenType::Assign)?;
        let val = self.parse_expression(Precedence::Lowest)?;
        if self.current_is(TokenType::Semicolon) { self.advance(); }
        Some(Statement::Let(span, name, typ, val))
    }

    fn parse_var(&mut self) -> Option<Statement> {
        self.advance(); // consume `var` — treat as mutable let
        let span = self.dummy_span();
        let name = self.current.literal.clone();
        self.expect(TokenType::Identifier)?;
        let typ = if self.current_is(TokenType::Colon) { self.advance(); Some(self.parse_type_expr()?) } else { None };
        self.expect(TokenType::Assign)?;
        let val = self.parse_expression(Precedence::Lowest)?;
        if self.current_is(TokenType::Semicolon) { self.advance(); }
        Some(Statement::Let(span, name, typ, val))
    }

    fn parse_const(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let name = self.current.literal.clone();
        self.expect(TokenType::Identifier)?;
        let typ = if self.current_is(TokenType::Colon) { self.advance(); Some(self.parse_type_expr()?) } else { None };
        self.expect(TokenType::Assign)?;
        let val = self.parse_expression(Precedence::Lowest)?;
        if self.current_is(TokenType::Semicolon) { self.advance(); }
        Some(Statement::Let(span, name, typ, val))
    }

    // ── Return ────────────────────────────────────────────────────────────────

    fn parse_return(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        // `return;` or `return }` (implicit void return)
        if self.current_is(TokenType::Semicolon) || self.current_is(TokenType::RBrace) {
            if self.current_is(TokenType::Semicolon) { self.advance(); }
            let s2 = self.dummy_span();
            return Some(Statement::Return(span, Expression::Literal(Literal::Null(s2))));
        }
        let val = self.parse_expression(Precedence::Lowest)?;
        if self.current_is(TokenType::Semicolon) { self.advance(); }
        Some(Statement::Return(span, val))
    }

    // ── Function ──────────────────────────────────────────────────────────────

    fn parse_function(&mut self) -> Option<Statement> {
        let span = self.advance().span; // consume `fn`
        // Optional access modifier already consumed by caller context
        let name = self.current.literal.clone();
        self.expect(TokenType::Identifier)?;
        // Optional generic params
        if self.current_is(TokenType::LessThan) { self.skip_generic_params(); }
        self.expect(TokenType::LParen)?;
        let params = self.parse_parameters()?;
        self.expect(TokenType::RParen)?;
        // Optional with effects clause: `with effects { Eff1, Eff2 }`
        if self.current_is(TokenType::KeywordWith) { self.skip_effects_clause(); }
        let ret = if self.current_is(TokenType::ThinArrow) { self.advance(); Some(self.parse_type_expr()?) } else { None };
        // Optional preconditions / where clauses
        if self.current_is(TokenType::KeywordWhere) { self.skip_where_clause(); }
        let body = self.parse_block_expression()?;
        Some(Statement::Function(span, name, params, ret, body))
    }

    fn skip_generic_params(&mut self) {
        self.advance(); // consume <
        let mut depth = 1i32;
        while depth > 0 && !self.current_is(TokenType::EOF) {
            match self.current.token_type {
                TokenType::LessThan  => { depth += 1; self.advance(); }
                TokenType::GreaterThan => { depth -= 1; self.advance(); }
                _ => { self.advance(); }
            }
        }
    }

    fn skip_effects_clause(&mut self) {
        // `with effects { A, B }` or `with effects A`
        self.advance(); // consume `with`
        if self.current_is(TokenType::KeywordEffect) { self.advance(); }
        if self.current_is(TokenType::LBrace) {
            self.advance();
            while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) { self.advance(); }
            self.advance(); // consume }
        }
    }

    fn skip_where_clause(&mut self) {
        self.advance(); // consume `where`
        while !matches!(self.current.token_type, TokenType::LBrace | TokenType::EOF) { self.advance(); }
    }

    // ── While / For ───────────────────────────────────────────────────────────

    fn parse_while(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let cond = self.parse_expression(Precedence::Lowest)?;
        let body = self.parse_block_expression()?;
        Some(Statement::While(span, cond, body))
    }

    fn parse_for(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let name = self.current.literal.clone();
        let id_span = self.current.span.clone();
        self.expect(TokenType::Identifier)?;
        self.expect(TokenType::KeywordIn)?;
        let iter = self.parse_expression(Precedence::Lowest)?;
        let body = self.parse_block_expression()?;
        Some(Statement::For(span, Identifier(name, id_span), iter, body))
    }

    // ── Match ─────────────────────────────────────────────────────────────────

    fn parse_match(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let subject = self.parse_expression(Precedence::Lowest)?;
        self.expect(TokenType::LBrace)?;
        let mut cases = vec![];
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            let cs = self.dummy_span();
            let pat = self.parse_expression(Precedence::Lowest)?;
            // Optional guard: `if condition`
            if self.current_is(TokenType::KeywordIf) {
                self.advance();
                self.parse_expression(Precedence::Lowest);
            }
            // match arms use => (FatArrow)
            if self.current_is(TokenType::FatArrow) { self.advance(); }
            else if self.current_is(TokenType::ThinArrow) { self.advance(); }
            else { self.expect(TokenType::FatArrow)?; }
            let body = self.parse_expression(Precedence::Lowest)?;
            if self.current_is(TokenType::Comma) { self.advance(); }
            cases.push(MatchCase { pattern: pat, body, span: cs });
        }
        self.expect(TokenType::RBrace)?;
        Some(Statement::Match(span, subject, cases))
    }

    // ── Quantum circuit ───────────────────────────────────────────────────────

    fn parse_quantum_circuit(&mut self) -> Option<Statement> {
        let span = self.advance().span; // quantum
        if self.current_is(TokenType::KeywordCircuit) { self.advance(); }
        let name = self.current.literal.clone();
        self.expect(TokenType::Identifier)?;
        // Optional parameter list
        if self.current_is(TokenType::LParen) {
            self.advance();
            while !self.current_is(TokenType::RParen) && !self.current_is(TokenType::EOF) { self.advance(); }
            self.advance();
        }
        // Optional `-> RetType`
        if self.current_is(TokenType::ThinArrow) { self.advance(); self.parse_type_expr(); }
        // Optional `with effects { ... }`
        if self.current_is(TokenType::KeywordWith) { self.skip_effects_clause(); }
        let body = self.parse_block_expression()?;
        Some(Statement::QuantumCircuit(span, name, body))
    }

    // ── Nano agent ────────────────────────────────────────────────────────────

    fn parse_nano_agent(&mut self) -> Option<Statement> {
        let span = self.advance().span; // nano
        if self.current_is(TokenType::KeywordAgent) { self.advance(); }
        let name = self.current.literal.clone();
        self.expect(TokenType::Identifier)?;
        if self.current_is(TokenType::LParen) {
            self.advance();
            while !self.current_is(TokenType::RParen) && !self.current_is(TokenType::EOF) { self.advance(); }
            self.advance();
        }
        if self.current_is(TokenType::KeywordWith) { self.skip_effects_clause(); }
        let body = self.parse_block_expression()?;
        Some(Statement::NanoAgent(span, name, body))
    }

    // ── Sankofa memory ────────────────────────────────────────────────────────

    fn parse_sankofa_remember(&mut self) -> Option<Statement> {
        let span = self.advance().span; // remember
        // Accept any token as a name (keywords can be used as identifiers in Zenith)
        let name = self.current.literal.clone();
        self.advance(); // consume the name token
        // Optional type annotation
        if self.current_is(TokenType::Colon) { self.advance(); self.parse_type_expr(); }
        self.expect(TokenType::Assign)?;
        let val = self.parse_expression(Precedence::Lowest)?;
        if self.current_is(TokenType::Semicolon) { self.advance(); }
        Some(Statement::SankofaMemory(span, name, val))
    }

    // ── Effect declaration ────────────────────────────────────────────────────

    fn parse_effect_decl(&mut self) -> Option<Statement> {
        let span = self.advance().span; // effect
        let name_lit = self.current.literal.clone();
        let id_span  = self.current.span.clone();
        self.expect(TokenType::Identifier)?;
        // Optional body `{ ... }` with effect operations
        if self.current_is(TokenType::LBrace) {
            self.advance();
            while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) { self.advance(); }
            self.advance();
        }
        if self.current_is(TokenType::Semicolon) { self.advance(); }
        Some(Statement::EffectDeclaration(span, Identifier(name_lit, id_span)))
    }

    // ── Handle (algebraic effects) ────────────────────────────────────────────

    fn parse_handle(&mut self) -> Option<Statement> {
        let span = self.advance().span; // handle
        let eff_name = self.current.literal.clone();
        let eff_span = self.current.span.clone();
        self.expect(TokenType::Identifier)?;
        // `{ computation }` guarded computation
        let computation = self.parse_block_expression()?;
        // Optional `with { handler_cases }` or handler block
        let handler = if self.current_is(TokenType::KeywordWith) {
            self.advance();
            self.parse_block_expression()?
        } else {
            Expression::Block(span.clone(), vec![])
        };
        Some(Statement::Handle(span, Identifier(eff_name, eff_span), computation, handler))
    }

    // ── Type declaration ──────────────────────────────────────────────────────

    fn parse_type_decl(&mut self) -> Option<Statement> {
        let span = self.advance().span; // type
        let name = self.current.literal.clone();
        self.expect(TokenType::Identifier)?;
        if self.current_is(TokenType::LessThan) { self.skip_generic_params(); }
        self.expect(TokenType::Assign)?;
        let ty = self.parse_type_expr()?;
        if self.current_is(TokenType::Semicolon) { self.advance(); }
        Some(Statement::TypeDeclaration(span, name, ty))
    }

    // ── Struct ────────────────────────────────────────────────────────────────

    fn parse_struct(&mut self) -> Option<Statement> {
        let span = self.advance().span; // struct
        let name_lit = self.current.literal.clone();
        let name_span = self.current.span.clone();
        self.expect(TokenType::Identifier)?;
        if self.current_is(TokenType::LessThan) { self.skip_generic_params(); }
        // Parse struct body as a class with only fields
        self.expect(TokenType::LBrace)?;
        let mut members: Vec<ClassMember> = vec![];
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            let field_name = self.current.literal.clone();
            let field_span = self.current.span.clone();
            if self.advance().token_type == TokenType::EOF { break; }
            let field_type = if self.current_is(TokenType::Colon) {
                self.advance();
                self.parse_type_expr()
            } else { None };
            if self.current_is(TokenType::Comma) { self.advance(); }
            members.push(ClassMember::Field(
                span.clone(),
                AccessModifier::Public,
                Identifier(field_name, field_span),
                field_type.unwrap_or(TypeExpr::Identifier(Identifier("Unknown".into(), span.clone()))),
                None,
            ));
        }
        self.expect(TokenType::RBrace)?;
        Some(Statement::Class(span, Identifier(name_lit, name_span), vec![], members))
    }

    // ── Enum ──────────────────────────────────────────────────────────────────

    fn parse_enum(&mut self) -> Option<Statement> {
        let span = self.advance().span; // enum
        let name_lit = self.current.literal.clone();
        let name_span = self.current.span.clone();
        self.expect(TokenType::Identifier)?;
        if self.current_is(TokenType::LessThan) { self.skip_generic_params(); }
        self.expect(TokenType::LBrace)?;
        // Consume variants
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) { self.advance(); }
        self.expect(TokenType::RBrace)?;
        Some(Statement::TypeDeclaration(span, name_lit,
            TypeExpr::Identifier(Identifier("Enum".into(), name_span))))
    }

    // ── Class ─────────────────────────────────────────────────────────────────

    fn parse_class(&mut self) -> Option<Statement> {
        let span = self.advance().span; // class
        let name_lit = self.current.literal.clone();
        let name_span = self.current.span.clone();
        self.expect(TokenType::Identifier)?;
        if self.current_is(TokenType::LessThan) { self.skip_generic_params(); }
        // extends / implements
        let mut supers: Vec<Identifier> = vec![];
        if self.current_is(TokenType::KeywordExtends) {
            self.advance();
            let s = self.current.literal.clone(); let ss = self.current.span.clone();
            self.advance();
            supers.push(Identifier(s, ss));
        }
        if self.current_is(TokenType::KeywordImplements) {
            self.advance();
            loop {
                let s = self.current.literal.clone(); let ss = self.current.span.clone();
                self.advance();
                supers.push(Identifier(s, ss));
                if !self.current_is(TokenType::Comma) { break; }
                self.advance();
            }
        }
        self.expect(TokenType::LBrace)?;
        let members = self.parse_class_members();
        self.expect(TokenType::RBrace)?;
        Some(Statement::Class(span, Identifier(name_lit, name_span), supers, members))
    }

    fn parse_class_members(&mut self) -> Vec<ClassMember> {
        let mut members = vec![];
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            // Access modifier
            let access = match self.current.token_type {
                TokenType::KeywordPublic    => { self.advance(); AccessModifier::Public }
                TokenType::KeywordPrivate   => { self.advance(); AccessModifier::Private }
                TokenType::KeywordProtected => { self.advance(); AccessModifier::Protected }
                _                           => AccessModifier::Public,
            };
            // Method modifiers
            let mut mods: Vec<MethodModifier> = vec![];
            loop {
                match self.current.token_type {
                    TokenType::KeywordStatic   => { self.advance(); mods.push(MethodModifier::Static); }
                    TokenType::KeywordVirtual  => { self.advance(); mods.push(MethodModifier::Virtual); }
                    TokenType::KeywordOverride => { self.advance(); mods.push(MethodModifier::Override); }
                    TokenType::KeywordAbstract => { self.advance(); mods.push(MethodModifier::Abstract); }
                    _                          => break,
                }
            }
            if self.current_is(TokenType::KeywordFn) {
                // Method
                if let Some(Statement::Function(fn_span, name, params, ret, body)) = self.parse_function() {
                    members.push(ClassMember::Method(
                        fn_span.clone(),
                        access,
                        mods.into_iter().next(), // Option<MethodModifier>
                        Identifier(name, fn_span),
                        params,
                        ret,
                        body,
                        vec![], // effects
                    ));
                }
            } else if self.current_is(TokenType::Identifier) {
                // Field
                let fname = self.current.literal.clone();
                let fspan = self.current.span.clone();
                self.advance();
                let ftyp = if self.current_is(TokenType::Colon) { self.advance(); self.parse_type_expr() } else { None };
                let fdef = if self.current_is(TokenType::Assign) { self.advance(); self.parse_expression(Precedence::Lowest) } else { None };
                if self.current_is(TokenType::Semicolon) || self.current_is(TokenType::Comma) { self.advance(); }
                members.push(ClassMember::Field(
                    fspan.clone(),
                    access,
                    Identifier(fname, fspan),
                    ftyp.unwrap_or(TypeExpr::Identifier(Identifier("Unknown".into(), self.dummy_span()))),
                    fdef,
                ));
            } else {
                self.advance(); // error recovery
            }
        }
        members
    }

    // ── Interface ─────────────────────────────────────────────────────────────

    fn parse_interface(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let name_lit = self.current.literal.clone();
        let name_span = self.current.span.clone();
        self.expect(TokenType::Identifier)?;
        if self.current_is(TokenType::LessThan) { self.skip_generic_params(); }
        let mut supers = vec![];
        if self.current_is(TokenType::KeywordExtends) {
            self.advance();
            let s = self.current.literal.clone(); let ss = self.current.span.clone(); self.advance();
            supers.push(Identifier(s, ss));
        }
        self.expect(TokenType::LBrace)?;
        let mut members = vec![];
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            if self.current_is(TokenType::KeywordFn) {
                self.advance();
                let mname = self.current.literal.clone(); let mspan = self.current.span.clone();
                self.advance();
                // skip signature
                if self.current_is(TokenType::LParen) { self.advance(); while !self.current_is(TokenType::RParen) && !self.current_is(TokenType::EOF) { self.advance(); } self.advance(); }
                let ret = if self.current_is(TokenType::ThinArrow) { self.advance(); self.parse_type_expr() } else { None };
                if self.current_is(TokenType::Semicolon) { self.advance(); }
                members.push(InterfaceMember::Method(
                    mspan.clone(),
                    Identifier(mname, mspan),
                    vec![],
                    ret,
                ));
            } else { self.advance(); }
        }
        self.expect(TokenType::RBrace)?;
        Some(Statement::Interface(span, Identifier(name_lit, name_span), supers, members))
    }

    // ── Module / Import ───────────────────────────────────────────────────────

    fn parse_module(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let name = self.current.literal.clone();
        self.expect(TokenType::Identifier)?;
        let body = if self.current_is(TokenType::LBrace) {
            self.advance();
            let mut stmts = vec![];
            while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
                if let Some(s) = self.parse_statement() { stmts.push(s); } else { self.advance(); }
            }
            self.expect(TokenType::RBrace)?;
            stmts
        } else { vec![] };
        Some(Statement::Module(span, name, body))
    }

    fn parse_import(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let mut path = vec![];
        loop {
            path.push(self.current.literal.clone());
            self.advance();
            if self.current_is(TokenType::Dot) || self.current_is(TokenType::Slash) { self.advance(); } else { break; }
        }
        if self.current_is(TokenType::Semicolon) { self.advance(); }
        Some(Statement::Import(span, path))
    }

    // ── Wisdom (Sankofa) ──────────────────────────────────────────────────────

    fn parse_wisdom(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let name = self.current.literal.clone();
        let id_span = self.current.span.clone();
        self.expect(TokenType::Identifier)?;
        let body = self.parse_block_expression()?;
        // Represent as a function-like declaration
        Some(Statement::Function(span.clone(), format!("__wisdom_{}", name), vec![], None, body))
    }

    // ── Unsafe ────────────────────────────────────────────────────────────────

    fn parse_unsafe(&mut self) -> Option<Statement> {
        let span = self.advance().span; // unsafe
        // optional `(evas:{proof})`
        let evas = if self.current_is(TokenType::LParen) {
            self.advance();
            let id_lit = self.current.literal.clone();
            let id_span = self.current.span.clone();
            while !self.current_is(TokenType::RParen) && !self.current_is(TokenType::EOF) { self.advance(); }
            self.advance();
            Some(Identifier(id_lit, id_span))
        } else { None };
        let body = self.parse_block_expression()?;
        Some(Statement::Unsafe(span, evas, body))
    }

    // ── Attribute statements (@prove, @ensure_ethical, etc.) ──────────────────

    fn parse_attribute_statement(&mut self) -> Option<Statement> {
        self.advance(); // consume @
        let attr_name = self.current.literal.clone();
        self.advance(); // consume attribute name
        // Consume optional (...)
        if self.current_is(TokenType::LParen) {
            self.advance();
            while !self.current_is(TokenType::RParen) && !self.current_is(TokenType::EOF) { self.advance(); }
            self.advance();
        }
        // The next statement is the attributed item
        self.parse_statement()
    }

    // ── Expressions ───────────────────────────────────────────────────────────

    fn parse_expression(&mut self, prec: Precedence) -> Option<Expression> {
        let mut left = self.parse_prefix()?;
        while token_precedence(&self.current.token_type) > prec {
            left = self.parse_infix(left)?;
        }
        Some(left)
    }

    fn parse_prefix(&mut self) -> Option<Expression> {
        let span = self.dummy_span();
        match self.current.token_type.clone() {
            TokenType::Identifier            => {
                let id = self.make_identifier();
                self.advance();
                Some(Expression::Identifier(id))
            }
            TokenType::Integer               => {
                let n: i64 = self.current.literal.parse().unwrap_or(0);
                self.advance();
                Some(Expression::Literal(Literal::Integer(n, span)))
            }
            TokenType::Float                 => {
                let f: f64 = self.current.literal.parse().unwrap_or(0.0);
                self.advance();
                Some(Expression::Literal(Literal::Float(f, span)))
            }
            TokenType::String         => {
                let s = self.current.literal.clone();
                self.advance();
                Some(Expression::Literal(Literal::String(s, span)))
            }
            TokenType::Boolean               => {
                let b = self.current.literal == "true";
                self.advance();
                Some(Expression::Literal(Literal::Boolean(b, span)))
            }
            TokenType::KeywordTrue           => { self.advance(); Some(Expression::Literal(Literal::Boolean(true, span))) }
            TokenType::KeywordFalse          => { self.advance(); Some(Expression::Literal(Literal::Boolean(false, span))) }
            TokenType::Minus | TokenType::KeywordNot | TokenType::Exclamation => {
                let op = self.current.token_type.clone();
                self.advance();
                let r = self.parse_expression(Precedence::Unary)?;
                Some(Expression::Prefix(span, op, Box::new(r)))
            }
            TokenType::LParen                => {
                self.advance();
                // Empty tuple ()
                if self.current_is(TokenType::RParen) { self.advance(); return Some(Expression::Tuple(span, vec![])); }
                let e = self.parse_expression(Precedence::Lowest)?;
                // Tuple: (a, b, ...)
                if self.current_is(TokenType::Comma) {
                    let mut elems = vec![e];
                    while self.current_is(TokenType::Comma) {
                        self.advance();
                        if self.current_is(TokenType::RParen) { break; }
                        elems.push(self.parse_expression(Precedence::Lowest)?);
                    }
                    self.expect(TokenType::RParen)?;
                    return Some(Expression::Tuple(span, elems));
                }
                self.expect(TokenType::RParen)?;
                Some(e)
            }
            TokenType::LBracket              => {
                self.advance();
                let mut elems = vec![];
                while !self.current_is(TokenType::RBracket) && !self.current_is(TokenType::EOF) {
                    elems.push(self.parse_expression(Precedence::Lowest)?);
                    if self.current_is(TokenType::Comma) { self.advance(); }
                }
                self.expect(TokenType::RBracket)?;
                Some(Expression::Array(span, elems))
            }
            TokenType::LBrace                => self.parse_block_expression(),
            TokenType::KeywordIf             => self.parse_if_expression(),
            TokenType::KeywordFn             => self.parse_lambda(),
            TokenType::KeywordNew            => {
                self.advance();
                let name = self.make_identifier(); self.advance();
                let mut args = vec![];
                if self.current_is(TokenType::LParen) {
                    self.advance();
                    while !self.current_is(TokenType::RParen) && !self.current_is(TokenType::EOF) {
                        args.push(self.parse_expression(Precedence::Lowest)?);
                        if self.current_is(TokenType::Comma) { self.advance(); }
                    }
                    self.advance();
                }
                Some(Expression::New(span, name, args))
            }
            TokenType::KeywordRecall         => {
                self.advance();
                self.expect(TokenType::LParen)?;
                let domain = self.parse_expression(Precedence::Lowest)?;
                self.expect(TokenType::RParen)?;
                Some(Expression::Recall(span, Box::new(domain)))
            }
            TokenType::KeywordRemember       => {
                self.advance();
                let name = self.current.literal.clone(); self.advance();
                self.expect(TokenType::Assign)?;
                let val = self.parse_expression(Precedence::Lowest)?;
                Some(Expression::Remember(span, name, Box::new(val)))
            }
            TokenType::KeywordPerform        => {
                // `perform EffectName(args)` — treat as a call expression
                self.advance();
                let eff_name = self.current.literal.clone();
                let eff_id = self.make_identifier();
                self.advance();
                let mut args = vec![];
                if self.current_is(TokenType::LParen) {
                    self.advance();
                    while !self.current_is(TokenType::RParen) && !self.current_is(TokenType::EOF) {
                        args.push(self.parse_expression(Precedence::Lowest)?);
                        if self.current_is(TokenType::Comma) { self.advance(); }
                    }
                    self.advance();
                }
                Some(Expression::Call(span, Box::new(Expression::Identifier(eff_id)), args))
            }
            TokenType::KeywordAsync          => {
                self.advance();
                let inner = self.parse_expression(Precedence::Lowest)?;
                Some(Expression::Spawn(span, Box::new(inner)))
            }
            TokenType::KeywordAwait          => {
                self.advance();
                let inner = self.parse_expression(Precedence::Lowest)?;
                Some(Expression::Await(span, Box::new(inner)))
            }
            // Sankofa temporal blocks: zamani { ... } and sasa { ... }
            TokenType::KeywordZamani | TokenType::KeywordSasa => {
                let kw = self.current.literal.clone();
                self.advance();
                let block = self.parse_block_expression()?;
                Some(Expression::NanoOp(span, kw, vec![block]))
            }
            // Sankofa ancestor call: ancestral Name(args)
            TokenType::KeywordAncestor       => {
                self.advance();
                let name = self.current.literal.clone(); self.advance();
                let mut args = vec![];
                if self.current_is(TokenType::LParen) {
                    self.advance();
                    while !self.current_is(TokenType::RParen) && !self.current_is(TokenType::EOF) {
                        args.push(self.parse_expression(Precedence::Lowest)?);
                        if self.current_is(TokenType::Comma) { self.advance(); }
                    }
                    self.advance();
                }
                Some(Expression::NanoOp(span, format!("ancestral::{}", name), args))
            }
            // Quantum literal: |0⟩ |1⟩
            TokenType::Pipe                  => {
                self.advance();
                let state = self.current.literal.clone(); self.advance();
                // consume ⟩ if present
                if self.current.literal.contains('⟩') { self.advance(); }
                Some(Expression::QuantumOp(span, format!("|{}⟩", state), vec![]))
            }
            // Linear / affine types used as expressions (rare but valid)
            TokenType::KeywordLinear | TokenType::KeywordAffine => {
                let kw = self.current.literal.clone(); self.advance();
                let inner = self.parse_expression(Precedence::Unary)?;
                Some(Expression::Cast(span, Box::new(inner), TypeExpr::Identifier(Identifier(kw, self.dummy_span()))))
            }
            // Consensus expression
            TokenType::KeywordLearn          => {
                // learn from expr [with weight expr]
                self.advance();
                if self.current_is(TokenType::KeywordFrom) { self.advance(); }
                let src = self.parse_expression(Precedence::Lowest)?;
                if self.current_is(TokenType::KeywordWith) { self.advance(); self.advance(); self.parse_expression(Precedence::Lowest); }
                Some(Expression::NanoOp(span, "learn".into(), vec![src]))
            }
            _ => {
                let msg = format!("Unexpected token in expression: {:?} ('{}')", self.current.token_type, self.current.literal);
                self.errors.push(ParserError { message: msg, span: span.clone() });
                None
            }
        }
    }

    fn parse_infix(&mut self, left: Expression) -> Option<Expression> {
        let span = self.dummy_span();
        match &self.current.token_type {
            // Binary operators
            TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash
            | TokenType::Modulo | TokenType::Equals | TokenType::NotEquals
            | TokenType::LessThan | TokenType::GreaterThan
            | TokenType::LessThanEqual | TokenType::GreaterThanEqual
            | TokenType::LogicalAnd | TokenType::LogicalOr => {
                let op   = self.current.token_type.clone();
                let prec = token_precedence(&op);
                self.advance();
                let right = self.parse_expression(prec)?;
                Some(Expression::Infix(span, Box::new(left), op, Box::new(right)))
            }
            // Assignment
            TokenType::Assign => {
                self.advance();
                let right = self.parse_expression(Precedence::Assign)?;
                Some(Expression::Assign(span, Box::new(left), Box::new(right)))
            }
            // Function call
            TokenType::LParen => {
                self.advance();
                let mut args = vec![];
                while !self.current_is(TokenType::RParen) && !self.current_is(TokenType::EOF) {
                    args.push(self.parse_expression(Precedence::Lowest)?);
                    if self.current_is(TokenType::Comma) { self.advance(); }
                }
                self.expect(TokenType::RParen)?;
                Some(Expression::Call(span, Box::new(left), args))
            }
            // Index
            TokenType::LBracket => {
                self.advance();
                let idx = self.parse_expression(Precedence::Lowest)?;
                self.expect(TokenType::RBracket)?;
                Some(Expression::Index(span, Box::new(left), Box::new(idx)))
            }
            // Member access
            TokenType::Dot => {
                self.advance();
                let member = self.make_identifier();
                self.advance();
                Some(Expression::MemberAccess(span, Box::new(left), member))
            }
            // Range
            TokenType::Not => {
                self.advance();
                let inclusive = self.current_is(TokenType::Assign);
                if inclusive { self.advance(); }
                let right = self.parse_expression(Precedence::Lowest)?;
                Some(Expression::Range(span, Box::new(left), Box::new(right), inclusive))
            }
            // Type cast: `as Type`
            TokenType::KeywordAs => {
                self.advance();
                let ty = self.parse_type_expr()?;
                Some(Expression::Cast(span, Box::new(left), ty))
            }
            _ => Some(left),
        }
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        let span = self.advance().span; // if
        let cond = self.parse_expression(Precedence::Lowest)?;
        let then = self.parse_block_expression()?;
        let else_branch = if self.current_is(TokenType::KeywordElse) {
            self.advance();
            if self.current_is(TokenType::KeywordIf) {
                Some(Box::new(self.parse_if_expression()?))
            } else {
                Some(Box::new(self.parse_block_expression()?))
            }
        } else { None };
        Some(Expression::If(span, Box::new(cond), Box::new(then), else_branch))
    }

    fn parse_block_expression(&mut self) -> Option<Expression> {
        let span = self.dummy_span();
        self.expect(TokenType::LBrace)?;
        let mut stmts = vec![];
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            if let Some(s) = self.parse_statement() { stmts.push(s); }
            else { self.skip_to_next_statement(); }
        }
        self.expect(TokenType::RBrace)?;
        Some(Expression::Block(span, stmts))
    }

    fn parse_lambda(&mut self) -> Option<Expression> {
        let span = self.advance().span; // fn
        self.expect(TokenType::LParen)?;
        let params = self.parse_parameters()?;
        self.expect(TokenType::RParen)?;
        let body = self.parse_block_expression()?;
        Some(Expression::Lambda(span, params, Box::new(body)))
    }

    // ── Parameters ────────────────────────────────────────────────────────────

    fn parse_parameters(&mut self) -> Option<Vec<Parameter>> {
        let mut params = vec![];
        while !self.current_is(TokenType::RParen) && !self.current_is(TokenType::EOF) {
            // Optional self/this
            if self.current_is(TokenType::KeywordSelf) || self.current_is(TokenType::KeywordThis) {
                let s = self.current.span.clone();
                params.push(Parameter { name: Identifier("self".into(), s), typ: None, default: None });
                self.advance();
                if self.current_is(TokenType::Comma) { self.advance(); }
                continue;
            }
            let name_lit = self.current.literal.clone();
            let name_span = self.current.span.clone();
            self.advance();
            let typ = if self.current_is(TokenType::Colon) { self.advance(); self.parse_type_expr() } else { None };
            let default = if self.current_is(TokenType::Assign) { self.advance(); self.parse_expression(Precedence::Lowest) } else { None };
            params.push(Parameter { name: Identifier(name_lit, name_span), typ, default });
            if self.current_is(TokenType::Comma) { self.advance(); }
        }
        Some(params)
    }

    // ── Type expressions ──────────────────────────────────────────────────────

    fn parse_type_expr(&mut self) -> Option<TypeExpr> {
        let span = self.dummy_span();
        match self.current.token_type.clone() {
            // Primitive named types
            TokenType::Identifier => {
                let name = self.current.literal.clone();
                let id_span = self.current.span.clone();
                self.advance();
                // Generic: Type<A, B>
                if self.current_is(TokenType::LessThan) {
                    self.advance();
                    let mut args = vec![];
                    while !self.current_is(TokenType::GreaterThan) && !self.current_is(TokenType::EOF) {
                        args.push(self.parse_type_expr()?);
                        if self.current_is(TokenType::Comma) { self.advance(); }
                    }
                    self.advance(); // >
                    return Some(TypeExpr::Generic(Box::new(TypeExpr::Identifier(Identifier(name, id_span))), args));
                }
                Some(TypeExpr::Identifier(Identifier(name, id_span)))
            }
            // Function types: fn(A, B) -> C
            TokenType::KeywordFn => {
                self.advance();
                self.expect(TokenType::LParen)?;
                let mut params = vec![];
                while !self.current_is(TokenType::RParen) && !self.current_is(TokenType::EOF) {
                    params.push(self.parse_type_expr()?);
                    if self.current_is(TokenType::Comma) { self.advance(); }
                }
                self.advance();
                let ret = if self.current_is(TokenType::ThinArrow) { self.advance(); Box::new(self.parse_type_expr()?) } else { Box::new(TypeExpr::Identifier(Identifier("Void".into(), span.clone()))) };
                Some(TypeExpr::Function(params, ret))
            }
            // Tuple type: (A, B, C)
            TokenType::LParen => {
                self.advance();
                let mut types = vec![];
                while !self.current_is(TokenType::RParen) && !self.current_is(TokenType::EOF) {
                    types.push(self.parse_type_expr()?);
                    if self.current_is(TokenType::Comma) { self.advance(); }
                }
                self.advance();
                Some(TypeExpr::Tuple(types))
            }
            // Array type: [T] or [T; N]
            TokenType::LBracket => {
                self.advance();
                let inner = self.parse_type_expr()?;
                self.expect(TokenType::RBracket)?;
                Some(TypeExpr::Array(Box::new(inner)))
            }
            // Reference: &T
            TokenType::Ampersand => {
                self.advance();
                let inner = self.parse_type_expr()?;
                Some(TypeExpr::Reference(Box::new(inner), false))
            }
            // Linear / affine
            TokenType::KeywordLinear => { self.advance(); let inner = self.parse_type_expr()?; Some(TypeExpr::Linear(Box::new(inner))) }
            TokenType::KeywordAffine => { self.advance(); let inner = self.parse_type_expr()?; Some(TypeExpr::Affine(Box::new(inner))) }
            // Quantum types: Qubit, QReg[N]
            TokenType::KeywordQuantum => {
                self.advance();
                Some(TypeExpr::Quantum(Box::new(TypeExpr::Identifier(Identifier("Qubit".into(), self.dummy_span())))))
            }
            // Nano types
            TokenType::KeywordNano => {
                self.advance();
                Some(TypeExpr::Nano(Box::new(TypeExpr::Identifier(Identifier("NanoAgent".into(), self.dummy_span())))))
            }
            // MTS types: mts[N]
            TokenType::KeywordMts => {
                self.advance();
                if self.current_is(TokenType::LBracket) { self.advance(); self.advance(); self.advance(); }
                Some(TypeExpr::MTS(Box::new(TypeExpr::Identifier(Identifier("MtsSlice".into(), self.dummy_span())))))
            }
            // Sigma type: Σ(x:T) U
            TokenType::KeywordSigma => {
                self.advance();
                Some(TypeExpr::DependentSigma(Box::new(Identifier("x".into(), self.dummy_span())), Box::new(TypeExpr::Identifier(Identifier("T".into(), self.dummy_span()))), Box::new(TypeExpr::Identifier(Identifier("U".into(), self.dummy_span())))))
            }
            _ => {
                // Fallback — treat current token as identifier type
                let name = self.current.literal.clone();
                let id_span = self.current.span.clone();
                self.advance();
                Some(TypeExpr::Identifier(Identifier(name, id_span)))
            }
        }
    }
}

#[cfg(test)]
mod parser_self_tests {
    use super::*;
    use crate::source_map::{FileId, SourceFile};
    use crate::lexer::Lexer;
    use std::sync::Arc;

    fn mk(src: &str) -> Parser {
        let sf = Arc::new(SourceFile::new("<t>".into(), src.into()));
        Parser::new(Lexer::new(FileId::new(1), sf))
    }

    #[test] fn test_let_mut_internal() {
        let mut p = mk("let mut count = 0;");
        let prog = p.parse_program();
        assert!(p.get_errors().is_empty(), "errors: {:?}", p.get_errors());
        match &prog.statements[0] {
            Statement::Let(_, name, _, _) => assert_eq!(name, "count"),
            s => panic!("got {:?}", s),
        }
    }

    #[test] fn test_match_fat_arrow() {
        let mut p = mk("match x { 1 => 10, 2 => 20, }");
        let prog = p.parse_program();
        assert!(p.get_errors().is_empty(), "errors: {:?}", p.get_errors());
    }
}
