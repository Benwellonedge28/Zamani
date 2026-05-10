pub mod ast {
    use crate::tokens::Span;
    use crate::lexer::TokenType;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Program {
        pub statements: Vec<Statement>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Statement {
        Let(Span, String, Option<TypeExpr>, Expression),
        Return(Span, Expression),
        Expression(Expression),
        Function(Span, String, Vec<Parameter>, Option<TypeExpr>, Box<Expression>),
        QuantumCircuit(Span, String, Box<Expression>),
        NanoAgent(Span, String, Box<Expression>),
        SankofaMemory(Span, String, Expression),
        TypeDeclaration(Span, String, TypeExpr),
        EffectDeclaration(Span, String),
        LanguageDeclaration(Span, String, Expression),
        While(Span, Box<Expression>, Box<Expression>),
        For(Span, Identifier, Box<Expression>, Box<Expression>),
        Break(Span),
        Continue(Span),
        Match(Span, Box<Expression>, Vec<MatchCase>),
        Unsafe(Span, Option<Identifier>, Box<Expression>), // New: unsafe!(evas:proof_id) { ... }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Expression {
        Identifier(Identifier),
        Literal(Literal),
        Prefix(Span, TokenType, Box<Expression>),
        Infix(Span, Box<Expression>, TokenType, Box<Expression>),
        If(Span, Box<Expression>, Box<Expression>, Option<Box<Expression>>),
        Block(Span, Vec<Statement>),
        Call(Span, Box<Expression>, Vec<Expression>),
        Index(Span, Box<Expression>, Box<Expression>),
        MemberAccess(Span, Box<Expression>, Identifier),
        // Add more expression types as needed for Zenith's grammar
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Literal {
        Integer(String, Span),
        Float(String, Span),
        String(String, Span),
        Boolean(bool, Span),
        Char(char, Span),
        Quantum(String, Span),
        Nano(String, Span),
        MTS(String, Span),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Identifier(pub String, pub Span);

    // New: Represents a type expression in the AST
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TypeExpr {
        Base(Identifier),
        Generic(Box<TypeExpr>, Vec<TypeExpr>),
        Array(Box<TypeExpr>, Option<String>),
        FunctionType(Vec<TypeExpr>, Box<TypeExpr>),
        DependentPi(Identifier, Box<TypeExpr>, Box<TypeExpr>),
        DependentSigma(Identifier, Box<TypeExpr>, Box<TypeExpr>),
        Linear(Box<TypeExpr>),
        Affine(Box<TypeExpr>),
        Effectful(Box<TypeExpr>, Vec<Identifier>),
        Universe(usize),
        SankofaHistory(Box<TypeExpr>, Box<Expression>),
        SankofaConsensus(Box<TypeExpr>),
        SankofaInterMemory(Identifier, Box<TypeExpr>),
        // ... more specific types from Zenith's complex type system
    }

    // New: Represents a function parameter
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Parameter {
        pub name: Identifier,
        pub typ: Option<TypeExpr>,
    }

    // New: Represents a single case in a match statement
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct MatchCase {
        pub span: Span,
        pub pattern: Expression,
        pub body: Box<Expression>,
    }

    // Helper to get the span of any expression
    impl Expression {
        pub fn span(&self) -> &Span {
            match self {
                Expression::Identifier(id) => &id.1,
                Expression::Literal(lit) => match lit {
                    Literal::Integer(_, span) => span,
                    Literal::Float(_, span) => span,
                    Literal::String(_, span) => span,
                    Literal::Boolean(_, span) => span,
                    Literal::Char(_, span) => span,
                    Literal::Quantum(_, span) => span,
                    Literal::Nano(_, span) => span,
                    Literal::MTS(_, span) => span,
                },
                Expression::Prefix(span, _, _) => span,
                Expression::Infix(span, _, _, _) => span,
                Expression::If(span, _, _, _) => span,
                Expression::Block(span, _) => span,
                Expression::Call(span, _, _) => span,
                Expression::Index(span, _, _) => span,
                Expression::MemberAccess(span, _, _) => span,
            }
        }
    }

    // Helper to get the span of any TypeExpr
    impl TypeExpr {
        pub fn span(&self) -> &Span {
            match self {
                TypeExpr::Base(id) => &id.1,
                TypeExpr::Generic(base, _) => base.span(),
                TypeExpr::Array(base, _) => base.span(),
                TypeExpr::FunctionType(_, ret) => ret.span(), // Simplified, should cover all
                TypeExpr::DependentPi(_, _, ret) => ret.span(),
                TypeExpr::DependentSigma(_, _, ret) => ret.span(),
                TypeExpr::Linear(base) => base.span(),
                TypeExpr::Affine(base) => base.span(),
                TypeExpr::Effectful(base, _) => base.span(),
                TypeExpr::Universe(_) => &Span::new(0,0,0), // Placeholder
                TypeExpr::SankofaHistory(base, _) => base.span(),
                TypeExpr::SankofaConsensus(base) => base.span(),
                TypeExpr::SankofaInterMemory(_, base) => base.span(),
            }
        }
    }
}
