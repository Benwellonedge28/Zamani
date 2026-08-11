//! Zamani Parser — Full recursive-descent / Pratt implementation
//! Covers: let/const/fn/struct/enum/trait/impl/class/interface/
//!         module/import/use/while/for/match/if/block/closure/
//!         async/await/spawn/try/try-catch/quantum/nano/sankofa/
//!         wisdom/effect/handle/unsafe/macros/type-aliases

use crate::ast::*;
use crate::lexer::{Lexer, Token, TokenType};
use crate::source_map::Span;

// ─── Precedence ───────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest,
    Assign,
    Range,
    LogicalOr,
    LogicalAnd,
    BitOr,
    BitXor,
    BitAnd,
    Equality,
    Comparison,
    Shift,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
    Member,
}
impl Precedence {
    fn of(tok: &TokenType) -> Self {
        use TokenType::*;
        match tok {
            Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign => Precedence::Assign,
            DotDot | DotDotEq => Precedence::Range,
            LogicalOr | KeywordOr => Precedence::LogicalOr,
            LogicalAnd | KeywordAnd => Precedence::LogicalAnd,
            Pipe => Precedence::BitOr,
            Caret => Precedence::BitXor,
            BitAnd | Ampersand => Precedence::BitAnd,
            Equals | NotEquals => Precedence::Equality,
            LessThan | LessThanEqual | GreaterThan | GreaterThanEqual => Precedence::Comparison,
            LeftShift | RightShift => Precedence::Shift,
            Plus | Minus => Precedence::Sum,
            Star | Slash | Modulo => Precedence::Product,
            LParen => Precedence::Call,
            LBracket => Precedence::Index,
            Dot => Precedence::Member,
            _ => Precedence::Lowest,
        }
    }
}

// ─── Error ────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
    pub message: String,
    pub span: Span,
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
        let peek = lexer.next_token();
        Parser {
            lexer,
            current,
            peek,
            errors: vec![],
        }
    }
    pub fn get_errors(&self) -> &Vec<ParserError> {
        &self.errors
    }
    fn advance(&mut self) -> Token {
        let prev = self.current.clone();
        self.current = self.peek.clone();
        self.peek = self.lexer.next_token();
        prev
    }
    fn current_is(&self, t: TokenType) -> bool {
        self.current.token_type == t
    }
    fn peek_is(&self, t: TokenType) -> bool {
        self.peek.token_type == t
    }
    fn expect(&mut self, t: TokenType) -> Option<Token> {
        if self.current_is(t.clone()) {
            Some(self.advance())
        } else {
            let msg = format!(
                "Expected {:?}, found {:?} ('{}')",
                t, self.current.token_type, self.current.literal
            );
            self.errors.push(ParserError {
                message: msg,
                span: self.current.span.clone(),
            });
            None
        }
    }
    fn skip_semis(&mut self) {
        while self.current_is(TokenType::Semicolon) {
            self.advance();
        }
    }

    // ── Program ───────────────────────────────────────────────────────────────
    pub fn parse_program(&mut self) -> Program {
        let start = self.current.span.clone();
        let mut stmts = vec![];
        while !self.current_is(TokenType::EOF) {
            self.skip_semis();
            if self.current_is(TokenType::EOF) {
                break;
            }
            if let Some(s) = self.parse_statement() {
                stmts.push(s);
            }
            self.skip_semis();
        }
        Program::new(stmts, start)
    }

    // ── Statements ────────────────────────────────────────────────────────────
    fn parse_statement(&mut self) -> Option<Statement> {
        use TokenType::*;
        match self.current.token_type.clone() {
            KeywordLet | KeywordVar => self.parse_let(),
            KeywordConst => self.parse_const(),
            KeywordFn => self.parse_function(),
            KeywordReturn => self.parse_return(),
            KeywordBreak => {
                let s = self.advance().span;
                if self.current_is(Semicolon) {
                    self.advance();
                }
                Some(Statement::Break(s))
            }
            KeywordContinue => {
                let s = self.advance().span;
                if self.current_is(Semicolon) {
                    self.advance();
                }
                Some(Statement::Continue(s))
            }
            KeywordWhile => self.parse_while(),
            KeywordFor => self.parse_for(),
            KeywordMatch => self.parse_match_stmt(),
            KeywordStruct => self.parse_struct(),
            KeywordEnum => self.parse_enum(),
            KeywordTrait => self.parse_trait(),
            KeywordImpl => self.parse_impl(),
            KeywordClass => self.parse_class(),
            KeywordInterface => self.parse_interface(),
            KeywordModule => self.parse_module(),
            KeywordImport => self.parse_import(),
            KeywordUse => self.parse_use(),
            KeywordQuantum | KeywordCircuit => self.parse_quantum_circuit(),
            KeywordNano | KeywordAgent => self.parse_nano_agent(),
            KeywordRemember => self.parse_sankofa_remember(),
            KeywordEffect => self.parse_effect_decl(),
            KeywordHandle => self.parse_handle(),
            KeywordType => self.parse_type_alias(),
            KeywordUnsafe => self.parse_unsafe(),
            KeywordWisdom => self.parse_wisdom(),
            KeywordLanguage => self.parse_language_decl(),
            KeywordOmniversal => {
                self.advance(); // consume 'omniversal'
                match self.current.token_type {
                    TokenType::KeywordSimulate => self.parse_omniversal_block(Statement::OmniversalSimulation),
                    TokenType::KeywordSynthesize => self.parse_omniversal_block(Statement::OmniversalCodeSynth),
                    TokenType::KeywordDeploy => self.parse_omniversal_block(Statement::OmniversalDeploy),
                    TokenType::KeywordAlignment => self.parse_omniversal_block(Statement::OmniversalAlignment),
                    TokenType::KeywordContainment => self.parse_omniversal_block(Statement::OmniversalContainment),
                    TokenType::KeywordTrust => self.parse_omniversal_block(Statement::OmniversalTrust),
                    TokenType::KeywordKnowledge => self.parse_omniversal_block(Statement::OmniversalKnowledge),
                    TokenType::KeywordGenerative => self.parse_omniversal_block(Statement::OmniversalGenerative),
                    TokenType::KeywordSovereignty => self.parse_omniversal_block(Statement::OmniversalSovereignty),
                    TokenType::KeywordGoal => self.parse_omniversal_block(Statement::OmniversalGoal),
                    TokenType::KeywordBionano => self.parse_omniversal_block(Statement::OmniversalBioNano),
                    TokenType::KeywordReality => self.parse_omniversal_block(Statement::OmniversalReality),
                    TokenType::KeywordNlp => self.parse_omniversal_block(Statement::OmniversalNlp),
                    _ => self.parse_expr_stmt(),
                }
            }
            Hash => self.parse_attribute_stmt(),
            _ => self.parse_expr_stmt(),
        }
    }
    fn parse_expr_stmt(&mut self) -> Option<Statement> {
        let e = self.parse_expression(Precedence::Lowest)?;
        if self.current_is(TokenType::Semicolon) {
            self.advance();
        }
        Some(Statement::Expression(e))
    }

    // ── Bindings ──────────────────────────────────────────────────────────────
    fn parse_let(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let _mut_flag = if self.current_is(TokenType::KeywordMut) {
            self.advance();
            true
        } else {
            false
        };
        let name = self.current.literal.clone();
        self.advance();
        let typ = if self.current_is(TokenType::Colon) {
            self.advance();
            self.parse_type_expr()
        } else {
            None
        };
        self.expect(TokenType::Assign)?;
        let val = self.parse_expression(Precedence::Lowest)?;
        if self.current_is(TokenType::Semicolon) {
            self.advance();
        }
        Some(Statement::Let(span, name, typ, val))
    }
    fn parse_const(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let name = self.current.literal.clone();
        self.advance();
        let typ = if self.current_is(TokenType::Colon) {
            self.advance();
            self.parse_type_expr()
        } else {
            None
        };
        self.expect(TokenType::Assign)?;
        let val = self.parse_expression(Precedence::Lowest)?;
        if self.current_is(TokenType::Semicolon) {
            self.advance();
        }
        Some(Statement::Const(span, name, typ, val))
    }

    // ── Function ──────────────────────────────────────────────────────────────
    fn parse_function(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        if self.current_is(TokenType::KeywordAsync) {
            self.advance();
        }
        let name = self.current.literal.clone();
        self.advance();
        if self.current_is(TokenType::LessThan) {
            self.skip_generic_params();
        }
        let params = self.parse_params()?;
        let ret = if self.current_is(TokenType::ThinArrow) {
            self.advance();
            self.parse_type_expr()
        } else {
            None
        };
        if self.current_is(TokenType::KeywordWhere) {
            while !self.current_is(TokenType::LBrace) && !self.current_is(TokenType::EOF) {
                self.advance();
            }
        }
        let body = self.parse_block_expr()?;
        Some(Statement::Function(span, name, params, ret, body))
    }
    fn parse_return(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        if self.current_is(TokenType::Semicolon) || self.current_is(TokenType::RBrace) {
            if self.current_is(TokenType::Semicolon) {
                self.advance();
            }
            return Some(Statement::Return(
                span.clone(),
                Expression::Literal(Literal::Unit(span)),
            ));
        }
        let val = self.parse_expression(Precedence::Lowest)?;
        if self.current_is(TokenType::Semicolon) {
            self.advance();
        }
        Some(Statement::Return(span, val))
    }

    // ── Control flow ──────────────────────────────────────────────────────────
    fn parse_while(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let cond = self.parse_expression(Precedence::Lowest)?;
        let body = self.parse_block_expr()?;
        Some(Statement::While(span, cond, body))
    }
    fn parse_for(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let vs = self.current.span.clone();
        let vn = self.current.literal.clone();
        self.advance();
        self.expect(TokenType::KeywordIn)?;
        let iter = self.parse_expression(Precedence::Lowest)?;
        let body = self.parse_block_expr()?;
        Some(Statement::For(
            span,
            crate::ast::Identifier::new(vn, vs),
            iter,
            body,
        ))
    }
    fn parse_match_stmt(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let expr = self.parse_expression(Precedence::Lowest)?;
        let cases = self.parse_match_body()?;
        Some(Statement::Match(span, expr, cases))
    }
    fn parse_match_body(&mut self) -> Option<Vec<MatchCase>> {
        self.expect(TokenType::LBrace)?;
        let mut cases = vec![];
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            self.skip_semis();
            if self.current_is(TokenType::RBrace) {
                break;
            }
            let cs = self.current.span.clone();
            let pat = self.parse_pattern()?;
            let guard = if self.current_is(TokenType::KeywordIf) {
                self.advance();
                Some(self.parse_expression(Precedence::Lowest)?)
            } else {
                None
            };
            self.expect(TokenType::FatArrow)?;
            let body = if self.current_is(TokenType::LBrace) {
                self.parse_block_expr()?
            } else {
                self.parse_expression(Precedence::Lowest)?
            };
            if self.current_is(TokenType::Comma) {
                self.advance();
            }
            cases.push(MatchCase {
                pattern: pat,
                guard,
                body,
                span: cs,
            });
        }
        self.expect(TokenType::RBrace)?;
        Some(cases)
    }

    // ── Struct ────────────────────────────────────────────────────────────────
    fn parse_struct(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let ns = self.current.span.clone();
        let n = self.current.literal.clone();
        self.advance();
        let tp = self.parse_type_params();
        self.expect(TokenType::LBrace)?;
        let mut fields = vec![];
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            self.skip_semis();
            if self.current_is(TokenType::RBrace) {
                break;
            }
            let vis = if self.current_is(TokenType::KeywordPublic) {
                self.advance();
                Visibility::Public
            } else {
                Visibility::Private
            };
            let fs = self.current.span.clone();
            let fn_ = self.current.literal.clone();
            self.advance();
            let ft = if self.current_is(TokenType::Colon) {
                self.advance();
                self.parse_type_expr().unwrap_or(TypeExpr::Unit)
            } else {
                TypeExpr::Unit
            };
            if self.current_is(TokenType::Comma) {
                self.advance();
            }
            fields.push(StructField {
                name: crate::ast::Identifier::new(fn_, fs.clone()),
                typ: ft,
                visibility: vis,
                span: fs,
            });
        }
        self.expect(TokenType::RBrace)?;
        Some(Statement::Struct(
            span,
            crate::ast::Identifier::new(n, ns),
            tp,
            fields,
        ))
    }

    // ── Enum ──────────────────────────────────────────────────────────────────
    fn parse_enum(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let ns = self.current.span.clone();
        let n = self.current.literal.clone();
        self.advance();
        let tp = self.parse_type_params();
        self.expect(TokenType::LBrace)?;
        let mut variants = vec![];
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            self.skip_semis();
            if self.current_is(TokenType::RBrace) {
                break;
            }
            let vs = self.current.span.clone();
            let vn = self.current.literal.clone();
            self.advance();
            let kind = if self.current_is(TokenType::LParen) {
                self.advance();
                let mut ts = vec![];
                while !self.current_is(TokenType::RParen) && !self.current_is(TokenType::EOF) {
                    if let Some(t) = self.parse_type_expr() {
                        ts.push(t);
                    }
                    if self.current_is(TokenType::Comma) {
                        self.advance();
                    }
                }
                self.expect(TokenType::RParen)?;
                EnumVariantKind::Tuple(ts)
            } else {
                EnumVariantKind::Unit
            };
            if self.current_is(TokenType::Comma) {
                self.advance();
            }
            variants.push(EnumVariant {
                name: crate::ast::Identifier::new(vn, vs.clone()),
                fields: kind,
                span: vs,
            });
        }
        self.expect(TokenType::RBrace)?;
        Some(Statement::Enum(
            span,
            crate::ast::Identifier::new(n, ns),
            tp,
            variants,
        ))
    }

    // ── Trait ─────────────────────────────────────────────────────────────────
    fn parse_trait(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let ns = self.current.span.clone();
        let n = self.current.literal.clone();
        self.advance();
        let tp = self.parse_type_params();
        if self.current_is(TokenType::Colon) {
            while !self.current_is(TokenType::LBrace) && !self.current_is(TokenType::EOF) {
                self.advance();
            }
        }
        self.expect(TokenType::LBrace)?;
        let mut items = vec![];
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            self.skip_semis();
            if self.current_is(TokenType::RBrace) {
                break;
            }
            let is = self.current.span.clone();
            if self.current_is(TokenType::KeywordFn) {
                self.advance();
                let ms = self.current.span.clone();
                let mn = self.current.literal.clone();
                self.advance();
                if self.current_is(TokenType::LessThan) {
                    self.skip_generic_params();
                }
                let params = self.parse_params().unwrap_or_default();
                let ret = if self.current_is(TokenType::ThinArrow) {
                    self.advance();
                    self.parse_type_expr()
                } else {
                    None
                };
                let default_body = if self.current_is(TokenType::LBrace) {
                    Some(self.parse_block_expr()?)
                } else {
                    if self.current_is(TokenType::Semicolon) {
                        self.advance();
                    }
                    None
                };
                items.push(TraitItem {
                    name: crate::ast::Identifier::new(mn, ms),
                    kind: TraitItemKind::Method {
                        params,
                        ret,
                        default_body,
                    },
                    span: is,
                });
            } else if self.current_is(TokenType::KeywordType) {
                self.advance();
                let ts = self.current.span.clone();
                let tn = self.current.literal.clone();
                self.advance();
                let bound = if self.current_is(TokenType::Colon) {
                    self.advance();
                    self.parse_type_expr()
                } else {
                    None
                };
                if self.current_is(TokenType::Semicolon) {
                    self.advance();
                }
                items.push(TraitItem {
                    name: crate::ast::Identifier::new(tn, ts.clone()),
                    kind: TraitItemKind::AssociatedType(bound),
                    span: ts,
                });
            } else {
                self.advance();
            }
        }
        self.expect(TokenType::RBrace)?;
        Some(Statement::Trait(
            span,
            crate::ast::Identifier::new(n, ns),
            tp,
            items,
        ))
    }

    // ── Impl ──────────────────────────────────────────────────────────────────
    fn parse_impl(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let _tp = self.parse_type_params();
        let first = self.parse_type_expr()?;
        let (trait_name, impl_ty) = if self.current_is(TokenType::KeywordFor) {
            self.advance();
            let target = self.parse_type_expr()?;
            let tid = match &first {
                TypeExpr::Identifier(id) => Some(id.clone()),
                _ => None,
            };
            (tid, target)
        } else {
            (None, first)
        };
        if self.current_is(TokenType::KeywordWhere) {
            while !self.current_is(TokenType::LBrace) && !self.current_is(TokenType::EOF) {
                self.advance();
            }
        }
        self.expect(TokenType::LBrace)?;
        let mut items = vec![];
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            self.skip_semis();
            if self.current_is(TokenType::RBrace) {
                break;
            }
            let is = self.current.span.clone();
            let vis = if self.current_is(TokenType::KeywordPublic) {
                self.advance();
                Visibility::Public
            } else {
                Visibility::Private
            };
            if self.current_is(TokenType::KeywordFn) {
                self.advance();
                let ms = self.current.span.clone();
                let mn = self.current.literal.clone();
                self.advance();
                if self.current_is(TokenType::LessThan) {
                    self.skip_generic_params();
                }
                let params = self.parse_params().unwrap_or_default();
                let ret = if self.current_is(TokenType::ThinArrow) {
                    self.advance();
                    self.parse_type_expr()
                } else {
                    None
                };
                let body = if self.current_is(TokenType::LBrace) {
                    self.parse_block_expr()?
                } else {
                    if self.current_is(TokenType::Semicolon) {
                        self.advance();
                    }
                    Expression::Block(is.clone(), vec![])
                };
                items.push(ImplItem {
                    name: crate::ast::Identifier::new(mn, ms),
                    kind: ImplItemKind::Method { params, ret, body },
                    visibility: vis,
                    span: is,
                });
            } else if self.current_is(TokenType::KeywordType) {
                self.advance();
                let ts = self.current.span.clone();
                let tn = self.current.literal.clone();
                self.advance();
                self.expect(TokenType::Assign)?;
                let ty = self.parse_type_expr().unwrap_or(TypeExpr::Unit);
                if self.current_is(TokenType::Semicolon) {
                    self.advance();
                }
                items.push(ImplItem {
                    name: crate::ast::Identifier::new(tn, ts.clone()),
                    kind: ImplItemKind::AssociatedType(ty),
                    visibility: vis,
                    span: ts,
                });
            } else {
                self.advance();
            }
        }
        self.expect(TokenType::RBrace)?;
        Some(Statement::Impl(span, trait_name, impl_ty, items))
    }

    // ── Class ─────────────────────────────────────────────────────────────────
    fn parse_class(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let ns = self.current.span.clone();
        let n = self.current.literal.clone();
        self.advance();
        let mut bases = vec![];
        if self.current_is(TokenType::KeywordExtends)
            || self.current_is(TokenType::KeywordImplements)
        {
            self.advance();
            loop {
                let bs = self.current.span.clone();
                let bn = self.current.literal.clone();
                self.advance();
                bases.push(crate::ast::Identifier::new(bn, bs));
                if self.current_is(TokenType::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(TokenType::LBrace)?;
        let mut members = vec![];
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            self.skip_semis();
            if self.current_is(TokenType::RBrace) {
                break;
            }
            let vis = if self.current_is(TokenType::KeywordPublic) {
                self.advance();
                Visibility::Public
            } else {
                Visibility::Private
            };
            let is_static = if self.current_is(TokenType::KeywordStatic) {
                self.advance();
                true
            } else {
                false
            };
            if self.current_is(TokenType::KeywordFn) {
                self.advance();
                let ms = self.current.span.clone();
                let mn = self.current.literal.clone();
                self.advance();
                if self.current_is(TokenType::LessThan) {
                    self.skip_generic_params();
                }
                let params = self.parse_params().unwrap_or_default();
                let ret = if self.current_is(TokenType::ThinArrow) {
                    self.advance();
                    self.parse_type_expr()
                } else {
                    None
                };
                let body = self.parse_block_expr()?;
                members.push(ClassMember::Method {
                    name: crate::ast::Identifier::new(mn, ms),
                    params,
                    ret,
                    body,
                    visibility: vis,
                    is_static,
                    is_virtual: false,
                });
            } else if self.current_is(TokenType::KeywordNew) {
                self.advance();
                let params = self.parse_params().unwrap_or_default();
                let body = self.parse_block_expr()?;
                members.push(ClassMember::Constructor { params, body });
            } else {
                let fs = self.current.span.clone();
                let fn_ = self.current.literal.clone();
                self.advance();
                let ft = if self.current_is(TokenType::Colon) {
                    self.advance();
                    self.parse_type_expr().unwrap_or(TypeExpr::Unit)
                } else {
                    TypeExpr::Unit
                };
                let default = if self.current_is(TokenType::Assign) {
                    self.advance();
                    self.parse_expression(Precedence::Lowest)
                } else {
                    None
                };
                if self.current_is(TokenType::Semicolon) || self.current_is(TokenType::Comma) {
                    self.advance();
                }
                members.push(ClassMember::Field {
                    name: crate::ast::Identifier::new(fn_, fs.clone()),
                    typ: ft,
                    visibility: vis,
                    default,
                });
                let _ = is_static;
            }
        }
        self.expect(TokenType::RBrace)?;
        Some(Statement::Class(
            span,
            crate::ast::Identifier::new(n, ns),
            bases,
            members,
        ))
    }

    // ── Interface ─────────────────────────────────────────────────────────────
    fn parse_interface(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let ns = self.current.span.clone();
        let n = self.current.literal.clone();
        self.advance();
        let mut bases = vec![];
        if self.current_is(TokenType::Colon) {
            self.advance();
            loop {
                let bs = self.current.span.clone();
                let bn = self.current.literal.clone();
                self.advance();
                bases.push(crate::ast::Identifier::new(bn, bs));
                if self.current_is(TokenType::Plus) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(TokenType::LBrace)?;
        let mut members = vec![];
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            self.skip_semis();
            if self.current_is(TokenType::RBrace) {
                break;
            }
            if self.current_is(TokenType::KeywordFn) {
                self.advance();
                let ms = self.current.span.clone();
                let mn = self.current.literal.clone();
                self.advance();
                if self.current_is(TokenType::LessThan) {
                    self.skip_generic_params();
                }
                let params = self.parse_params().unwrap_or_default();
                let ret = if self.current_is(TokenType::ThinArrow) {
                    self.advance();
                    self.parse_type_expr()
                } else {
                    None
                };
                let default_body = if self.current_is(TokenType::LBrace) {
                    Some(self.parse_block_expr()?)
                } else {
                    if self.current_is(TokenType::Semicolon) {
                        self.advance();
                    }
                    None
                };
                members.push(InterfaceMember::Method {
                    name: crate::ast::Identifier::new(mn, ms),
                    params,
                    ret,
                    default_body,
                });
            } else {
                let ps = self.current.span.clone();
                let pn = self.current.literal.clone();
                self.advance();
                let pt = if self.current_is(TokenType::Colon) {
                    self.advance();
                    self.parse_type_expr().unwrap_or(TypeExpr::Unit)
                } else {
                    TypeExpr::Unit
                };
                if self.current_is(TokenType::Semicolon) {
                    self.advance();
                }
                members.push(InterfaceMember::Property {
                    name: crate::ast::Identifier::new(pn, ps),
                    typ: pt,
                });
            }
        }
        self.expect(TokenType::RBrace)?;
        Some(Statement::Interface(
            span,
            crate::ast::Identifier::new(n, ns),
            bases,
            members,
        ))
    }

    // ── Module / Import / Use ─────────────────────────────────────────────────
    fn parse_module(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let name = self.current.literal.clone();
        self.advance();
        if self.current_is(TokenType::LBrace) {
            self.advance();
            let mut stmts = vec![];
            while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
                self.skip_semis();
                if let Some(s) = self.parse_statement() {
                    stmts.push(s);
                }
            }
            self.expect(TokenType::RBrace)?;
            Some(Statement::Module(span, name, stmts))
        } else {
            if self.current_is(TokenType::Semicolon) {
                self.advance();
            }
            Some(Statement::Module(span, name, vec![]))
        }
    }
    fn parse_import(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let mut path = vec![];
        loop {
            path.push(self.current.literal.clone());
            self.advance();
            if self.current_is(TokenType::Dot) || self.current_is(TokenType::DoubleColon) {
                self.advance();
            } else {
                break;
            }
        }
        if self.current_is(TokenType::Semicolon) {
            self.advance();
        }
        Some(Statement::Import(span, path))
    }
    fn parse_use(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let mut segs = vec![];
        loop {
            if self.current_is(TokenType::Star) {
                self.advance();
                if self.current_is(TokenType::Semicolon) {
                    self.advance();
                }
                return Some(Statement::Use(
                    span,
                    UsePath {
                        segments: segs,
                        kind: UseKind::Glob,
                    },
                ));
            }
            segs.push(self.current.literal.clone());
            self.advance();
            if self.current_is(TokenType::DoubleColon) {
                self.advance();
                if self.current_is(TokenType::LBrace) {
                    self.advance();
                    let mut names = vec![];
                    while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
                        names.push(self.current.literal.clone());
                        self.advance();
                        if self.current_is(TokenType::Comma) {
                            self.advance();
                        }
                    }
                    self.expect(TokenType::RBrace)?;
                    if self.current_is(TokenType::Semicolon) {
                        self.advance();
                    }
                    return Some(Statement::Use(
                        span,
                        UsePath {
                            segments: segs,
                            kind: UseKind::Named(names),
                        },
                    ));
                }
            } else {
                break;
            }
        }
        if self.current_is(TokenType::Semicolon) {
            self.advance();
        }
        Some(Statement::Use(
            span,
            UsePath {
                segments: segs,
                kind: UseKind::Single,
            },
        ))
    }

    // ── Zamani-specific statements ────────────────────────────────────────────
    fn parse_quantum_circuit(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        // Support both `quantum circuit Name { ... }` and `circuit Name { ... }`
        if self.current_is(TokenType::KeywordCircuit) || self.current_is(TokenType::KeywordQuantum)
        {
            self.advance();
        }
        let name = self.current.literal.clone();
        self.advance();
        let body = if self.current_is(TokenType::LBrace) {
            self.parse_block_expr()?
        } else {
            self.parse_expression(Precedence::Lowest)?
        };
        Some(Statement::QuantumCircuit(span, name, body))
    }
    fn parse_nano_agent(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        // Support both `nano agent Name { ... }` and `agent Name { ... }`
        if self.current_is(TokenType::KeywordAgent) || self.current_is(TokenType::KeywordNano) {
            self.advance();
        }
        let name = self.current.literal.clone();
        self.advance();
        let body = if self.current_is(TokenType::LBrace) {
            self.parse_block_expr()?
        } else {
            self.parse_expression(Precedence::Lowest)?
        };
        Some(Statement::NanoAgent(span, name, body))
    }
    fn parse_sankofa_remember(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let name = self.current.literal.clone();
        self.advance();
        if self.current_is(TokenType::Colon) {
            self.advance();
            self.parse_type_expr();
        }
        self.expect(TokenType::Assign)?;
        let val = self.parse_expression(Precedence::Lowest)?;
        if self.current_is(TokenType::Semicolon) {
            self.advance();
        }
        Some(Statement::SankofaMemory(span, name, val))
    }
    fn parse_effect_decl(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let ns = self.current.span.clone();
        let n = self.current.literal.clone();
        self.advance();
        if self.current_is(TokenType::LBrace) {
            while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
                self.advance();
            }
            self.advance();
        } else if self.current_is(TokenType::Semicolon) {
            self.advance();
        }
        Some(Statement::EffectDeclaration(
            span,
            crate::ast::Identifier::new(n, ns),
        ))
    }
    fn parse_handle(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let en = self.current.literal.clone();
        let es = self.current.span.clone();
        self.advance();
        let body = self.parse_block_expr()?;
        let handler = if self.current_is(TokenType::KeywordWith) {
            self.advance();
            self.parse_block_expr()?
        } else {
            Expression::Block(span.clone(), vec![])
        };
        Some(Statement::Handle(
            span,
            crate::ast::Identifier::new(en, es),
            body,
            handler,
        ))
    }
    fn parse_type_alias(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let ns = self.current.span.clone();
        let n = self.current.literal.clone();
        self.advance();
        let tp = self.parse_type_params();
        self.expect(TokenType::Assign)?;
        let ty = self.parse_type_expr()?;
        if self.current_is(TokenType::Semicolon) {
            self.advance();
        }
        Some(Statement::TypeAlias(
            span,
            crate::ast::Identifier::new(n, ns),
            tp,
            ty,
        ))
    }
    fn parse_unsafe(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let label = if self.current_is(TokenType::Identifier) {
            let ls = self.current.span.clone();
            let l = self.current.literal.clone();
            self.advance();
            Some(crate::ast::Identifier::new(l, ls))
        } else {
            None
        };
        let body = self.parse_block_expr()?;
        Some(Statement::Unsafe(span, label, body))
    }
    fn parse_wisdom(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let name = self.current.literal.clone();
        self.advance();
        let val = if self.current_is(TokenType::Assign) {
            self.advance();
            self.parse_expression(Precedence::Lowest)?
        } else {
            Expression::Literal(Literal::Null(span.clone()))
        };
        if self.current_is(TokenType::Semicolon) {
            self.advance();
        }
        Some(Statement::Wisdom(span, name, val))
    }
    fn parse_language_decl(&mut self) -> Option<Statement> {
        let span = self.advance().span;
        let lang = self.current.literal.clone();
        self.advance();
        let ver = if self.current_is(TokenType::String) {
            let v = self.current.literal.clone();
            self.advance();
            v
        } else {
            "1.0".into()
        };
        if self.current_is(TokenType::Semicolon) {
            self.advance();
        }
        Some(Statement::LanguageDeclaration(span, lang, ver))
    }
    fn parse_attribute_stmt(&mut self) -> Option<Statement> {
        if self.current_is(TokenType::Hash) {
            self.advance();
            if self.current_is(TokenType::LBracket) {
                self.advance();
                let mut d = 1i32;
                while d > 0 && !self.current_is(TokenType::EOF) {
                    if self.current_is(TokenType::LBracket) {
                        d += 1;
                    }
                    if self.current_is(TokenType::RBracket) {
                        d -= 1;
                    }
                    self.advance();
                }
            }
        }
        self.parse_statement()
    }

    // ── Expressions ───────────────────────────────────────────────────────────
    fn parse_expression(&mut self, prec: Precedence) -> Option<Expression> {
        let mut left = self.parse_prefix()?;
        while Precedence::of(&self.current.token_type) > prec {
            left = self.parse_infix(left)?;
        }
        Some(left)
    }

    fn parse_prefix(&mut self) -> Option<Expression> {
        use TokenType::*;
        match self.current.token_type.clone() {
            Integer => {
                let s = self.current.span.clone();
                let n: i64 = self.current.literal.parse().unwrap_or(0);
                self.advance();
                Some(Expression::Literal(Literal::Integer(n, s)))
            }
            Float => {
                let s = self.current.span.clone();
                let f: f64 = self.current.literal.parse().unwrap_or(0.0);
                self.advance();
                Some(Expression::Literal(Literal::Float(f, s)))
            }
            String => {
                let s = self.current.span.clone();
                let v = self.current.literal.clone();
                self.advance();
                Some(Expression::Literal(Literal::String(v, s)))
            }
            Char => {
                let s = self.current.span.clone();
                let c = self.current.literal.chars().next().unwrap_or('\0');
                self.advance();
                Some(Expression::Literal(Literal::Char(c, s)))
            }
            KeywordTrue => {
                let s = self.advance().span;
                Some(Expression::Literal(Literal::Boolean(true, s)))
            }
            KeywordFalse => {
                let s = self.advance().span;
                Some(Expression::Literal(Literal::Boolean(false, s)))
            }
            KeywordNil => {
                let s = self.advance().span;
                Some(Expression::Literal(Literal::Null(s)))
            }
            QuantumLiteral => {
                let s = self.current.span.clone();
                let q = self.current.literal.clone();
                self.advance();
                Some(Expression::Literal(Literal::Quantum(q, s)))
            }
            Identifier => {
                let s = self.current.span.clone();
                let name = self.current.literal.clone();
                self.advance();
                if self.current_is(LBrace)
                    && matches!(self.peek.token_type, Identifier | String)
                    && self.peek_is(Colon)
                {
                    self.advance();
                    let mut fields = vec![];
                    while !self.current_is(RBrace) && !self.current_is(EOF) {
                        let fn_ = self.current.literal.clone();
                        self.advance();
                        self.expect(Colon)?;
                        let fv = self.parse_expression(Precedence::Lowest)?;
                        if self.current_is(Comma) {
                            self.advance();
                        }
                        fields.push((fn_, fv));
                    }
                    self.expect(RBrace)?;
                    return Some(Expression::Struct(
                        s.clone(),
                        crate::ast::Identifier::new(name, s),
                        fields,
                    ));
                }
                Some(Expression::Identifier(crate::ast::Identifier::new(name, s)))
            }
            Minus => {
                let s = self.advance().span;
                let i = self.parse_expression(Precedence::Prefix)?;
                Some(Expression::Prefix(s, Minus, Box::new(i)))
            }
            Not => {
                let s = self.advance().span;
                let i = self.parse_expression(Precedence::Prefix)?;
                Some(Expression::Prefix(s, Not, Box::new(i)))
            }
            Tilde => {
                let s = self.advance().span;
                let i = self.parse_expression(Precedence::Prefix)?;
                Some(Expression::Prefix(s, Tilde, Box::new(i)))
            }
            Ampersand | BitAnd => {
                let s = self.advance().span;
                if self.current_is(KeywordMut) {
                    self.advance();
                }
                let i = self.parse_expression(Precedence::Prefix)?;
                Some(Expression::Prefix(s, Ampersand, Box::new(i)))
            }
            Star => {
                let s = self.advance().span;
                let i = self.parse_expression(Precedence::Prefix)?;
                Some(Expression::Prefix(s, Star, Box::new(i)))
            }
            LBrace => Some(self.parse_block_expr()?),
            LParen => {
                let s = self.advance().span;
                if self.current_is(RParen) {
                    self.advance();
                    return Some(Expression::Literal(Literal::Unit(s)));
                }
                let first = self.parse_expression(Precedence::Lowest)?;
                if self.current_is(Comma) {
                    self.advance();
                    let mut items = vec![first];
                    while !self.current_is(RParen) && !self.current_is(EOF) {
                        items.push(self.parse_expression(Precedence::Lowest)?);
                        if self.current_is(Comma) {
                            self.advance();
                        }
                    }
                    self.expect(RParen)?;
                    return Some(Expression::Tuple(s, items));
                }
                self.expect(RParen)?;
                Some(first)
            }
            LBracket => {
                let s = self.advance().span;
                let mut items = vec![];
                while !self.current_is(RBracket) && !self.current_is(EOF) {
                    items.push(self.parse_expression(Precedence::Lowest)?);
                    if self.current_is(Comma) {
                        self.advance();
                    }
                }
                self.expect(RBracket)?;
                Some(Expression::Array(s, items))
            }
            Pipe => {
                let s = self.advance().span;
                let mut params = vec![];
                while !self.current_is(Pipe) && !self.current_is(EOF) {
                    let pn = self.current.literal.clone();
                    let ps = self.current.span.clone();
                    self.advance();
                    let pty = if self.current_is(Colon) {
                        self.advance();
                        self.parse_type_expr()
                    } else {
                        None
                    };
                    params.push(Parameter {
                        name: crate::ast::Identifier::new(pn, ps),
                        typ: pty,
                        default: None,
                        is_self: false,
                        is_mutable: false,
                    });
                    if self.current_is(Comma) {
                        self.advance();
                    }
                }
                self.expect(Pipe)?;
                let body = if self.current_is(LBrace) {
                    self.parse_block_expr()?
                } else {
                    self.parse_expression(Precedence::Lowest)?
                };
                Some(Expression::Lambda(s, params, Box::new(body)))
            }
            KeywordFn => {
                let s = self.advance().span;
                if self.current_is(LessThan) {
                    self.skip_generic_params();
                }
                let params = self.parse_params().unwrap_or_default();
                let _ret = if self.current_is(ThinArrow) {
                    self.advance();
                    self.parse_type_expr()
                } else {
                    None
                };
                let body = self.parse_block_expr()?;
                Some(Expression::Lambda(s, params, Box::new(body)))
            }
            KeywordIf => Some(self.parse_if_expr()?),
            KeywordMatch => {
                let s = self.advance().span;
                let sc = self.parse_expression(Precedence::Lowest)?;
                let cases = self.parse_match_body()?;
                Some(Expression::Match(s, Box::new(sc), cases))
            }
            KeywordLoop => {
                let s = self.advance().span;
                let b = self.parse_block_expr()?;
                Some(Expression::Loop(s, Box::new(b)))
            }
            KeywordAsync => {
                let s = self.advance().span;
                let i = self.parse_expression(Precedence::Lowest)?;
                Some(Expression::Async(s, Box::new(i)))
            }
            KeywordAwait => {
                let s = self.advance().span;
                let i = self.parse_expression(Precedence::Lowest)?;
                Some(Expression::Await(s, Box::new(i)))
            }
            KeywordSpawn => {
                let s = self.advance().span;
                let i = self.parse_expression(Precedence::Lowest)?;
                Some(Expression::Spawn(s, Box::new(i)))
            }
            KeywordNew => {
                let s = self.advance().span;
                let ts = self.current.span.clone();
                let tn = self.current.literal.clone();
                self.advance();
                let args = if self.current_is(LParen) {
                    self.parse_call_args()?
                } else {
                    vec![]
                };
                Some(Expression::New(
                    s,
                    crate::ast::Identifier::new(tn, ts),
                    args,
                ))
            }
            KeywordTry => {
                let s = self.advance().span;
                let body = self.parse_expression(Precedence::Lowest)?;
                if self.current_is(KeywordCatch) {
                    let mut arms = vec![];
                    while self.current_is(KeywordCatch) {
                        let cs = self.advance().span;
                        let (ety, binding) = if self.current_is(LParen) {
                            self.advance();
                            let bn = if self.current_is(Identifier) && self.peek_is(Colon) {
                                let n = self.current.literal.clone();
                                let ns = self.current.span.clone();
                                self.advance();
                                self.advance();
                                Some(crate::ast::Identifier::new(n, ns))
                            } else {
                                None
                            };
                            let t = self.parse_type_expr();
                            self.expect(RParen)?;
                            (t, bn)
                        } else {
                            (None, None)
                        };
                        let cb = self.parse_block_expr()?;
                        arms.push(CatchArm {
                            error_type: ety,
                            binding,
                            body: cb,
                            span: cs,
                        });
                    }
                    Some(Expression::TryCatch(s, Box::new(body), arms))
                } else {
                    Some(body)
                }
            }
            KeywordRecall => {
                let s = self.advance().span;
                let d = if self.current_is(LParen) {
                    self.advance();
                    let d = self.parse_expression(Precedence::Lowest)?;
                    self.expect(RParen)?;
                    d
                } else {
                    self.parse_expression(Precedence::Lowest)?
                };
                Some(Expression::Recall(s, Box::new(d)))
            }
            KeywordLearn => {
                let s = self.advance().span;
                if self.current_is(KeywordFrom) {
                    self.advance();
                }
                let d = self.parse_expression(Precedence::Lowest)?;
                Some(Expression::Learn(s, Box::new(d)))
            }
            KeywordInfer => {
                let s = self.advance().span;
                if self.current_is(KeywordFrom) {
                    self.advance();
                }
                let d = self.parse_expression(Precedence::Lowest)?;
                Some(Expression::Learn(s, Box::new(d)))
            }
            KeywordPerform => {
                let s = self.advance().span;
                let d = self.parse_expression(Precedence::Lowest)?;
                Some(Expression::Perform(s, Box::new(d)))
            }
            KeywordZamani => {
                let s = self.advance().span;
                let d = if self.current_is(LBrace) {
                    self.parse_block_expr()?
                } else {
                    self.parse_expression(Precedence::Lowest)?
                };
                Some(Expression::Zamani(s, Box::new(d)))
            }
            KeywordSasa => {
                let s = self.advance().span;
                let d = if self.current_is(LBrace) {
                    self.parse_block_expr()?
                } else {
                    self.parse_expression(Precedence::Lowest)?
                };
                Some(Expression::Sasa(s, Box::new(d)))
            }
            KeywordQuantum => {
                let s = self.advance().span;
                let gate = self.current.literal.clone();
                self.advance();
                let args = if self.current_is(LParen) {
                    self.parse_call_args()?
                } else {
                    vec![]
                };
                Some(Expression::QuantumOp(s, gate, args))
            }
            _ => {
                let s = self.current.span.clone();
                let name = self.current.literal.clone();
                if !name.is_empty() {
                    self.advance();
                    if self.current_is(LParen) {
                        let args = self.parse_call_args()?;
                        return Some(Expression::Call(
                            s.clone(),
                            Box::new(Expression::Identifier(crate::ast::Identifier::new(name, s))),
                            args,
                        ));
                    }
                    return Some(Expression::Identifier(crate::ast::Identifier::new(name, s)));
                }
                let msg = format!(
                    "Unexpected token: {:?} ('{}')",
                    self.current.token_type, self.current.literal
                );
                self.errors.push(ParserError {
                    message: msg,
                    span: self.current.span.clone(),
                });
                self.advance();
                None
            }
        }
    }

    fn parse_infix(&mut self, left: Expression) -> Option<Expression> {
        use TokenType::*;
        let op = self.current.token_type.clone();
        let span = self.current.span.clone();
        match op {
            LParen => {
                let args = self.parse_call_args()?;
                Some(Expression::Call(span, Box::new(left), args))
            }
            LBracket => {
                self.advance();
                let i = self.parse_expression(Precedence::Lowest)?;
                self.expect(RBracket)?;
                Some(Expression::Index(span, Box::new(left), Box::new(i)))
            }
            Dot => {
                self.advance();
                let fn_ = self.current.literal.clone();
                let fs = self.current.span.clone();
                self.advance();
                if self.current_is(LParen) {
                    let args = self.parse_call_args()?;
                    Some(Expression::MethodCall(
                        span,
                        Box::new(left),
                        crate::ast::Identifier::new(fn_, fs),
                        args,
                    ))
                } else {
                    Some(Expression::MemberAccess(
                        span,
                        Box::new(left),
                        crate::ast::Identifier::new(fn_, fs),
                    ))
                }
            }
            DotDot => {
                self.advance();
                let r = self.parse_expression(Precedence::Range)?;
                Some(Expression::Range(span, Box::new(left), Box::new(r), false))
            }
            DotDotEq => {
                self.advance();
                let r = self.parse_expression(Precedence::Range)?;
                Some(Expression::Range(span, Box::new(left), Box::new(r), true))
            }
            Assign => {
                self.advance();
                let r = self.parse_expression(Precedence::Assign)?;
                Some(Expression::Assign(span, Box::new(left), Box::new(r)))
            }
            PlusAssign => {
                self.advance();
                let r = self.parse_expression(Precedence::Assign)?;
                Some(Expression::CompoundAssign(
                    span,
                    Box::new(left),
                    Plus,
                    Box::new(r),
                ))
            }
            MinusAssign => {
                self.advance();
                let r = self.parse_expression(Precedence::Assign)?;
                Some(Expression::CompoundAssign(
                    span,
                    Box::new(left),
                    Minus,
                    Box::new(r),
                ))
            }
            StarAssign => {
                self.advance();
                let r = self.parse_expression(Precedence::Assign)?;
                Some(Expression::CompoundAssign(
                    span,
                    Box::new(left),
                    Star,
                    Box::new(r),
                ))
            }
            SlashAssign => {
                self.advance();
                let r = self.parse_expression(Precedence::Assign)?;
                Some(Expression::CompoundAssign(
                    span,
                    Box::new(left),
                    Slash,
                    Box::new(r),
                ))
            }
            KeywordAs => {
                self.advance();
                let ty = self.parse_type_expr()?;
                Some(Expression::Cast(span, Box::new(left), ty))
            }
            QuestionMark => {
                self.advance();
                Some(Expression::Try(span, Box::new(left)))
            }
            _ => {
                let prec = Precedence::of(&op);
                self.advance();
                let r = self.parse_expression(prec)?;
                Some(Expression::Infix(span, Box::new(left), op, Box::new(r)))
            }
        }
    }

    fn parse_if_expr(&mut self) -> Option<Expression> {
        let s = self.advance().span;
        let cond = self.parse_expression(Precedence::Lowest)?;
        let then = self.parse_block_expr()?;
        let else_ = if self.current_is(TokenType::KeywordElse) {
            self.advance();
            if self.current_is(TokenType::KeywordIf) {
                Some(Box::new(self.parse_if_expr()?))
            } else {
                Some(Box::new(self.parse_block_expr()?))
            }
        } else {
            None
        };
        Some(Expression::If(s, Box::new(cond), Box::new(then), else_))
    }

    fn parse_block_expr(&mut self) -> Option<Expression> {
        let s = self.expect(TokenType::LBrace)?.span;
        let mut stmts = vec![];
        while !self.current_is(TokenType::RBrace) && !self.current_is(TokenType::EOF) {
            self.skip_semis();
            if self.current_is(TokenType::RBrace) {
                break;
            }
            if let Some(s2) = self.parse_statement() {
                stmts.push(s2);
            }
        }
        self.expect(TokenType::RBrace)?;
        Some(Expression::Block(s, stmts))
    }

    // ── Parameters ────────────────────────────────────────────────────────────
    fn parse_params(&mut self) -> Option<Vec<Parameter>> {
        self.expect(TokenType::LParen)?;
        let mut params = vec![];
        while !self.current_is(TokenType::RParen) && !self.current_is(TokenType::EOF) {
            if self.current_is(TokenType::KeywordSelf) {
                let ss = self.current.span.clone();
                self.advance();
                params.push(Parameter {
                    name: crate::ast::Identifier::new("self", ss),
                    typ: None,
                    default: None,
                    is_self: true,
                    is_mutable: false,
                });
                if self.current_is(TokenType::Comma) {
                    self.advance();
                }
                continue;
            }
            let is_mut = if self.current_is(TokenType::KeywordMut) {
                self.advance();
                true
            } else {
                false
            };
            let ns = self.current.span.clone();
            let n = self.current.literal.clone();
            self.advance();
            let typ = if self.current_is(TokenType::Colon) {
                self.advance();
                self.parse_type_expr()
            } else {
                None
            };
            let default = if self.current_is(TokenType::Assign) {
                self.advance();
                self.parse_expression(Precedence::Lowest)
            } else {
                None
            };
            params.push(Parameter {
                name: crate::ast::Identifier::new(n, ns),
                typ,
                default,
                is_self: false,
                is_mutable: is_mut,
            });
            if self.current_is(TokenType::Comma) {
                self.advance();
            }
        }
        self.expect(TokenType::RParen)?;
        Some(params)
    }

    fn parse_call_args(&mut self) -> Option<Vec<Expression>> {
        self.expect(TokenType::LParen)?;
        let mut args = vec![];
        while !self.current_is(TokenType::RParen) && !self.current_is(TokenType::EOF) {
            args.push(self.parse_expression(Precedence::Lowest)?);
            if self.current_is(TokenType::Comma) {
                self.advance();
            }
        }
        self.expect(TokenType::RParen)?;
        Some(args)
    }

    // ── Type expressions ──────────────────────────────────────────────────────
    pub fn parse_type_expr(&mut self) -> Option<TypeExpr> {
        use TokenType::*;
        let result = match self.current.token_type.clone() {
            Identifier | KeywordInt | KeywordFloat | KeywordBool | KeywordStr
            | KeywordStringType | KeywordCharType | KeywordVoid | KeywordSelf | KeywordQuantum => {
                let name = self.current.literal.clone();
                let s = self.current.span.clone();
                self.advance();
                if self.current_is(LessThan) {
                    self.advance();
                    let mut args = vec![];
                    while !self.current_is(GreaterThan) && !self.current_is(EOF) {
                        if let Some(t) = self.parse_type_expr() {
                            args.push(t);
                        }
                        if self.current_is(Comma) {
                            self.advance();
                        }
                    }
                    self.expect(GreaterThan)?;
                    return Some(TypeExpr::Generic(
                        Box::new(TypeExpr::Identifier(crate::ast::Identifier::new(name, s))),
                        args,
                    ));
                }
                if name == "Self" {
                    return Some(TypeExpr::SelfType);
                }
                Some(TypeExpr::Identifier(crate::ast::Identifier::new(name, s)))
            }
            Ampersand | BitAnd => {
                self.advance();
                let is_mut = if self.current_is(KeywordMut) {
                    self.advance();
                    true
                } else {
                    false
                };
                if self.current_is(LBracket) {
                    self.advance();
                    let i = self.parse_type_expr()?;
                    self.expect(RBracket)?;
                    return Some(TypeExpr::Slice(Box::new(i)));
                }
                let i = self.parse_type_expr()?;
                Some(TypeExpr::Reference(is_mut, Box::new(i)))
            }
            Star => {
                self.advance();
                let is_mut = if self.current_is(KeywordMut) {
                    self.advance();
                    true
                } else {
                    false
                };
                let i = self.parse_type_expr()?;
                Some(TypeExpr::Pointer(is_mut, Box::new(i)))
            }
            LBracket => {
                self.advance();
                let i = self.parse_type_expr()?;
                if self.current_is(Semicolon) {
                    while !self.current_is(RBracket) && !self.current_is(EOF) {
                        self.advance();
                    }
                }
                self.expect(RBracket)?;
                Some(TypeExpr::Array(Box::new(i)))
            }
            LParen => {
                self.advance();
                if self.current_is(RParen) {
                    self.advance();
                    return Some(TypeExpr::Unit);
                }
                let first = self.parse_type_expr()?;
                if self.current_is(Comma) {
                    self.advance();
                    let mut ts = vec![first];
                    while !self.current_is(RParen) && !self.current_is(EOF) {
                        if let Some(t) = self.parse_type_expr() {
                            ts.push(t);
                        }
                        if self.current_is(Comma) {
                            self.advance();
                        }
                    }
                    self.expect(RParen)?;
                    if self.current_is(ThinArrow) {
                        self.advance();
                        let ret = self.parse_type_expr()?;
                        return Some(TypeExpr::Function(ts, Box::new(ret)));
                    }
                    return Some(TypeExpr::Tuple(ts));
                }
                self.expect(RParen)?;
                if self.current_is(ThinArrow) {
                    self.advance();
                    let ret = self.parse_type_expr()?;
                    return Some(TypeExpr::Function(vec![first], Box::new(ret)));
                }
                Some(first)
            }
            KeywordFn => {
                self.advance();
                self.expect(LParen)?;
                let mut ps = vec![];
                while !self.current_is(RParen) && !self.current_is(EOF) {
                    if let Some(t) = self.parse_type_expr() {
                        ps.push(t);
                    }
                    if self.current_is(Comma) {
                        self.advance();
                    }
                }
                self.expect(RParen)?;
                let ret = if self.current_is(ThinArrow) {
                    self.advance();
                    self.parse_type_expr().unwrap_or(TypeExpr::Unit)
                } else {
                    TypeExpr::Unit
                };
                Some(TypeExpr::Function(ps, Box::new(ret)))
            }
            _ => {
                let n = self.current.literal.clone();
                let s = self.current.span.clone();
                if !n.is_empty()
                    && n.chars()
                        .next()
                        .map(|c| c.is_uppercase() || c == '_')
                        .unwrap_or(false)
                {
                    self.advance();
                    return Some(TypeExpr::Identifier(crate::ast::Identifier::new(n, s)));
                }
                None
            }
        };
        if let Some(ty) = result {
            if self.current_is(QuestionMark) {
                self.advance();
                Some(TypeExpr::Optional(Box::new(ty)))
            } else {
                Some(ty)
            }
        } else {
            None
        }
    }

    fn parse_type_params(&mut self) -> Vec<TypeParameter> {
        if !self.current_is(TokenType::LessThan) {
            return vec![];
        }
        self.advance();
        let mut params = vec![];
        while !self.current_is(TokenType::GreaterThan) && !self.current_is(TokenType::EOF) {
            let s = self.current.span.clone();
            let n = self.current.literal.clone();
            self.advance();
            let mut bounds = vec![];
            if self.current_is(TokenType::Colon) {
                self.advance();
                loop {
                    let bs = self.current.span.clone();
                    let bn = self.current.literal.clone();
                    self.advance();
                    bounds.push(TypeBound::Trait(crate::ast::Identifier::new(bn, bs)));
                    if self.current_is(TokenType::Plus) {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
            params.push(TypeParameter {
                name: crate::ast::Identifier::new(n, s),
                bounds,
            });
            if self.current_is(TokenType::Comma) {
                self.advance();
            }
        }
        self.expect(TokenType::GreaterThan);
        params
    }
    fn skip_generic_params(&mut self) {
        let mut d = 0i32;
        loop {
            if self.current_is(TokenType::LessThan) {
                d += 1;
            }
            if self.current_is(TokenType::GreaterThan) {
                d -= 1;
                self.advance();
                if d <= 0 {
                    break;
                }
                continue;
            }
            if self.current_is(TokenType::EOF) {
                break;
            }
            self.advance();
        }
    }

    // ── Patterns ──────────────────────────────────────────────────────────────
    fn parse_pattern(&mut self) -> Option<Pattern> {
        use TokenType::*;
        match self.current.token_type.clone() {
            Identifier if self.current.literal == "_" => {
                let s = self.advance().span;
                Some(Pattern::Wildcard(s))
            }
            Star => {
                let s = self.advance().span;
                Some(Pattern::Wildcard(s))
            }
            Identifier => {
                let s = self.current.span.clone();
                let n = self.current.literal.clone();
                self.advance();
                if self.current_is(LParen) {
                    self.advance();
                    let mut ps = vec![];
                    while !self.current_is(RParen) && !self.current_is(EOF) {
                        if let Some(p) = self.parse_pattern() {
                            ps.push(p);
                        }
                        if self.current_is(Comma) {
                            self.advance();
                        }
                    }
                    self.expect(RParen)?;
                    return Some(Pattern::Enum(
                        s.clone(),
                        crate::ast::Identifier::new(n, s),
                        ps,
                    ));
                }
                if self.current_is(LBrace) {
                    self.advance();
                    let mut fs = vec![];
                    while !self.current_is(RBrace) && !self.current_is(EOF) {
                        let fn_ = self.current.literal.clone();
                        self.advance();
                        let fp = if self.current_is(Colon) {
                            self.advance();
                            self.parse_pattern()?
                        } else {
                            Pattern::Identifier(crate::ast::Identifier::new(fn_.clone(), s.clone()))
                        };
                        if self.current_is(Comma) {
                            self.advance();
                        }
                        fs.push((fn_, fp));
                    }
                    self.expect(RBrace)?;
                    return Some(Pattern::Struct(
                        s.clone(),
                        crate::ast::Identifier::new(n, s),
                        fs,
                    ));
                }
                Some(Pattern::Identifier(crate::ast::Identifier::new(n, s)))
            }
            Integer => {
                let s = self.current.span.clone();
                let n: i64 = self.current.literal.parse().unwrap_or(0);
                self.advance();
                Some(Pattern::Literal(Literal::Integer(n, s)))
            }
            Float => {
                let s = self.current.span.clone();
                let f: f64 = self.current.literal.parse().unwrap_or(0.0);
                self.advance();
                Some(Pattern::Literal(Literal::Float(f, s)))
            }
            String => {
                let s = self.current.span.clone();
                let v = self.current.literal.clone();
                self.advance();
                Some(Pattern::Literal(Literal::String(v, s)))
            }
            KeywordTrue => {
                let s = self.advance().span;
                Some(Pattern::Literal(Literal::Boolean(true, s)))
            }
            KeywordFalse => {
                let s = self.advance().span;
                Some(Pattern::Literal(Literal::Boolean(false, s)))
            }
            LParen => {
                let s = self.advance().span;
                if self.current_is(RParen) {
                    self.advance();
                    return Some(Pattern::Literal(Literal::Unit(s)));
                }
                let mut ps = vec![];
                while !self.current_is(RParen) && !self.current_is(EOF) {
                    if let Some(p) = self.parse_pattern() {
                        ps.push(p);
                    }
                    if self.current_is(Comma) {
                        self.advance();
                    }
                }
                self.expect(RParen)?;
                Some(Pattern::Tuple(s, ps))
            }
            _ => {
                let s = self.current.span.clone();
                self.advance();
                Some(Pattern::Wildcard(s))
            }
        }
    }

    // ── Omniversal Declarations ───────────────────────────────────────────────
    fn parse_omniversal_block(
        &mut self,
        ctor: fn(Span, String, Vec<Statement>) -> Statement,
    ) -> Option<Statement> {
        let span = self.advance().span;
        let name = self.current.literal.clone();
        self.advance();
        let body = self.parse_block_expr()?;
        match body {
            Expression::Block(_, stmts) => Some(ctor(span, name, stmts)),
            _ => Some(ctor(span, name, vec![])),
        }
    }
}
