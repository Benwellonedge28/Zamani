# Zamani Language Grammar

This document is the formal grammar reference for Zamani, derived directly
from the reference implementation (`src/lexer.rs`, `src/parser.rs`,
`src/ast/mod.rs`). It describes exactly what the compiler (`ZUTC`) accepts
today, so it stays in sync with the code rather than aspirational syntax.

Notation is EBNF-like:

```
| alternation        [x]     optional
{x}  zero or more      (x)     grouping
"x"  literal token      IDENT   lexical class
```

---

## 1. Lexical Grammar

### 1.1 Whitespace & Comments
- Whitespace (space, tab, CR, LF) is insignificant and skipped between tokens.
- `// ...` line comments run to end of line.
- `/* ... */` block comments (non-nesting) run until the closing `*/`.

### 1.2 Identifiers
```
IDENT = (ALPHA | "_") { ALPHA | DIGIT | "_" } ;
ALPHA = "a".."z" | "A".."Z" ;
DIGIT = "0".."9" ;
```

### 1.3 Literals
```
INTEGER  = DIGIT { DIGIT } ;
FLOAT    = DIGIT { DIGIT } "." DIGIT { DIGIT } ;
STRING   = '"' { any-char-except-'"' | escape } '"' ;
CHAR     = "'" (any-char | escape) "'" ;
BOOLEAN  = "true" | "false" ;
NIL      = "nil" | "null" ;
```

### 1.4 Zamani-native literals
```
QUANTUM_LITERAL = "|" ("0" | "1" | "+" | "-") "⟩" ;        e.g. |0⟩  |+⟩
NANO_ANNOTATION = "@" IDENT [ "(" ... ")" ] ;               e.g. @atom  @molecule(x)
MTS_LITERAL     = "mts" "[" ... "]" ;                       e.g. mts[timestamp]
```
> **Status:** `QUANTUM_LITERAL` and `NANO_ANNOTATION` are fully lexed today.
> `MTS_LITERAL` is declared as a token kind but not yet produced by the
> lexer — `mts` currently lexes as a plain identifier. Treat it as a
> reserved/planned literal form until wired up.

### 1.5 Punctuation & Operators
```
( ) { } [ ]  , . ; : -> => :: ~ # @
+  -  *  /  %  =
== != < > <= >=
&& || & | ^
<< >>
+= -= *= /=
.. ..=
? !
```

### 1.6 Keywords (reserved, cannot be used as identifiers)

Core / control flow:
`let  var  const  mut  fn  return  if  else  while  for  in  loop  break
continue  match  with  true  false  nil  null`

Types & OOP:
`struct  enum  trait  impl  class  interface  extends  implements  public
private  protected  static  virtual  override  abstract  new  this  super
type  self  Self  void  int  float  bool  str  String  char`

Modules:
`module  import  use  from  as  where`

Effects & safety:
`effect  perform  handle  unsafe  try  catch  throw  yield`

Concurrency:
`async  await  spawn`

Zamani-native (quantum / nano / sankofa / dependent types):
`quantum  circuit  nano  agent  remember  recall  learn  infer  wisdom
zamani  sasa  ancestor  linear  affine  language  is  and  or  sizeof
len  print  println  assert  panic  Pi (Π)  Sigma (Σ)  switch  case  then`

---

## 2. Grammar of a Program

```
Program = { Statement } ;
```

Top-level statement dispatch (`parse_statement`):
```
Statement =
    LetStmt
  | ConstStmt
  | FunctionDecl
  | ReturnStmt
  | BreakStmt
  | ContinueStmt
  | WhileStmt
  | ForStmt
  | MatchStmt
  | StructDecl
  | EnumDecl
  | TraitDecl
  | ImplBlock
  | ClassDecl
  | InterfaceDecl
  | ModuleDecl
  | ImportStmt
  | UseStmt
  | QuantumCircuitDecl
  | NanoAgentDecl
  | SankofaRememberStmt
  | EffectDecl
  | HandleStmt
  | TypeAliasDecl
  | UnsafeBlock
  | WisdomStmt
  | LanguageDecl
  | AttributeStmt
  | ExpressionStmt ;
```

Any statement may be preceded by an attribute: `AttributeStmt = "#" "[" ... "]" Statement ;`
(attributes are currently parsed and discarded — reserved for future use,
e.g. derive macros, lint suppression).

### 2.1 Bindings
```
LetStmt   = "let" ["mut"] IDENT [":" TypeExpr] "=" Expression [";"] ;
VarStmt   = "var" IDENT [":" TypeExpr] "=" Expression [";"] ;   -- alias of LetStmt
ConstStmt = "const" IDENT [":" TypeExpr] "=" Expression [";"] ;
```
`var` is lexed as a distinct keyword but dispatches through the same
`parse_let` path as `let`.

### 2.2 Control Flow
```
ReturnStmt   = "return" [Expression] [";"] ;
BreakStmt    = "break" [";"] ;
ContinueStmt = "continue" [";"] ;
WhileStmt    = "while" Expression BlockExpr ;
ForStmt      = "for" IDENT "in" Expression BlockExpr ;
MatchStmt    = "match" Expression MatchBody ;
MatchBody    = "{" { MatchCase } "}" ;
MatchCase    = Pattern "=>" Expression [","] ;
```
`match` is also usable as an expression (see §3).

### 2.3 Functions
```
FunctionDecl = "fn" IDENT ["<" TypeParams ">"] "(" [Params] ")"
               ["->" TypeExpr] BlockExpr ;
Params       = Param { "," Param } ;
Param        = ["mut"] IDENT [":" TypeExpr] ["=" Expression] ;
```
A parameter without a type annotation is legal (dynamically typed
parameter); a trailing `= expr` marks a default value.

### 2.4 Structs, Enums, Traits, Impls
```
StructDecl = "struct" IDENT ["<" TypeParams ">"] "{" { StructField } "}" ;
StructField = ["public"] IDENT [":" TypeExpr] ["," ] ;
```
Struct fields may omit the `: Type` annotation (defaults to `Unit`/inferred).

```
EnumDecl     = "enum" IDENT ["<" TypeParams ">"] "{" { EnumVariant } "}" ;
EnumVariant  = IDENT [ EnumVariantKind ] [","] ;
EnumVariantKind = "(" TypeExpr { "," TypeExpr } ")"        -- tuple variant
                | "{" { StructField } "}"                  -- struct variant
                | (* empty *) ;                            -- unit variant

TraitDecl  = "trait" IDENT ["<" TypeParams ">"] [":" ...] "{" { TraitItem } "}" ;
TraitItem  = TraitMethod | TraitAssocType ;
TraitMethod = "fn" IDENT ["<" ... ">"] "(" [Params] ")" ["->" TypeExpr]
              (BlockExpr | ";") ;
TraitAssocType = "type" IDENT [":" TypeExpr] ";" ;

ImplBlock  = "impl" ["<" TypeParams ">"] TypeExpr ["for" TypeExpr]
             "{" { ImplItem } "}" ;
```
`impl Type { ... }` is an inherent impl; `impl Trait for Type { ... }`
implements a trait.

### 2.5 Object-Oriented Constructs
```
ClassDecl = "class" IDENT [("extends"|"implements") IDENT {"," IDENT}]
            "{" { ClassMember } "}" ;
ClassMember = ["public"] ["static"]
              ( "fn" IDENT ["<" ... ">"] "(" [Params] ")" ["->" TypeExpr] BlockExpr
              | "new" "(" [Params] ")" BlockExpr                 -- constructor
              | IDENT [":" TypeExpr] ["=" Expression] ("," | ";") ) ;

InterfaceDecl = "interface" IDENT [":" IDENT {"," IDENT}]
                "{" { InterfaceMember } "}" ;
```

### 2.6 Modules & Imports
```
ModuleDecl = "module" IDENT ( BlockExpr | ";" ) ;
ImportStmt = "import" IDENT { ("." | "::") IDENT } [";"] ;
UseStmt    = "use" UsePath [";"] ;
UsePath    = Segment { "::" Segment }
           | Segment { "::" Segment } "::" "*"                   -- glob
           | Segment { "::" Segment } "::" "{" IDENT {"," IDENT} "}" ; -- named
```

### 2.7 Type Aliases
```
TypeAliasDecl = "type" IDENT ["<" TypeParams ">"] "=" TypeExpr [";"] ;
```

### 2.8 Safety
```
UnsafeBlock = "unsafe" [IDENT] BlockExpr ;
```

---

## 3. Zamani-Native Constructs

Zamani extends a conventional imperative/OO core with first-class support
for quantum computing, nano-scale agents, and "Sankofa" ancestral-memory
semantics (learn from the past to inform the present).

### 3.1 Quantum Circuits
```
QuantumCircuitDecl = ("quantum" "circuit" | "circuit" | "quantum")
                      IDENT (BlockExpr | Expression) ;
QuantumOpExpr       = "quantum" IDENT ["(" [Args] ")"] ;   -- e.g. quantum hadamard(q)
```
Both `quantum circuit Bell { ... }` and the shorthand `circuit Bell { ... }`
are accepted.

### 3.2 Nano Agents
```
NanoAgentDecl = ("nano" "agent" | "agent" | "nano") IDENT
                (BlockExpr | Expression) ;
```
Both `nano agent Healer { ... }` and the shorthand `agent Healer { ... }`
are accepted.

### 3.3 Sankofa Memory (remember / recall / learn / zamani / sasa)
```
SankofaRememberStmt = "remember" IDENT [":" TypeExpr] "=" Expression [";"] ;
RecallExpr = "recall" ( "(" Expression ")" | Expression ) ;
LearnExpr  = ("learn" | "infer") ["from"] Expression ;
ZamaniExpr = "zamani" (BlockExpr | Expression) ;   -- "the past" temporal scope
SasaExpr   = "sasa"   (BlockExpr | Expression) ;   -- "the present" temporal scope
WisdomStmt = "wisdom" IDENT ["=" Expression] [";"] ;
```
Example: `remember wisdom_of_elders = 42;` then later `recall(wisdom_of_elders)`.
`learn from data` and `infer from data` are equivalent forms that consume
an expression describing the data/domain to learn from.

### 3.4 Algebraic Effects
```
EffectDecl = "effect" IDENT (BlockExpr | ";") ;
PerformExpr = "perform" Expression ;                 -- e.g. perform Effect(args)
HandleStmt  = "handle" IDENT BlockExpr ["with" BlockExpr] ;
```

### 3.5 Language Pragma
```
LanguageDecl = "language" IDENT [STRING] [";"] ;      -- e.g. language Zamani "1.0";
```

---

## 4. Expression Grammar

### 4.1 Precedence (lowest to highest binding)
```
1. Lowest
2. Assign            = += -= *= /=
3. Range              .. ..=
4. LogicalOr          || or
5. LogicalAnd         && and
6. BitOr              |
7. BitXor             ^
8. BitAnd             & (bitwise-and)
9. Equality           == !=
10. Comparison        < <= > >=
11. Shift             << >>
12. Sum               + -
13. Product           * / %
14. Prefix            unary - ! ~ & &mut *  (right-assoc)
15. Call              f(...)
16. Index             a[...]
17. Member            a.b
```
Parsing uses standard Pratt/precedence-climbing: `parse_expression(min_prec)`.

### 4.2 Primary / Prefix Expressions
```
Expression =
    IDENT
  | Literal
  | "(" Expression ")"
  | "(" Expression {"," Expression} ")"        -- tuple, 2+ elements
  | "[" [Expression {"," Expression}] "]"      -- array literal
  | "{" { Statement } "}"                      -- block expression (value = last stmt)
  | "|" [Params] "|" (BlockExpr | Expression)  -- lambda/closure
  | "fn" "(" [Params] ")" ["->" TypeExpr] BlockExpr  -- anonymous fn
  | "-" Expression | "!" Expression | "~" Expression
  | "&" ["mut"] Expression | "*" Expression
  | IfExpr | MatchExpr | LoopExpr
  | "async" Expression | "await" Expression | "spawn" Expression
  | "new" IDENT ["(" [Args] ")"]
  | "try" Expression { "catch" ["(" [IDENT ":"] TypeExpr ")"] BlockExpr }
  | RecallExpr | LearnExpr | PerformExpr | ZamaniExpr | SasaExpr | QuantumOpExpr
  | IDENT "{" { IDENT ":" Expression [","] } "}"      -- struct literal
  ;

IfExpr   = "if" Expression BlockExpr ["else" (IfExpr | BlockExpr)] ;
MatchExpr = "match" Expression MatchBody ;
LoopExpr = "loop" BlockExpr ;
```

### 4.3 Postfix / Infix Expressions
```
PostfixExpr =
    Expression "(" [Args] ")"                 -- call
  | Expression "[" Expression "]"             -- index
  | Expression "." IDENT                      -- member access
  | Expression "." IDENT "(" [Args] ")"       -- method call
  | Expression "?"                            -- try-propagate (expr?)
  | Expression "as" TypeExpr                  -- cast
  | Expression ":" TypeExpr                   -- type ascription
  | Expression "=" Expression                 -- assignment
  | Expression ("+="|"-="|"*="|"/=") Expression
  | Expression BinOp Expression               -- see precedence table
  | Expression (".." | "..=") Expression      -- range (exclusive/inclusive)
  ;

Args = Expression { "," Expression } ;
```

### 4.4 Block Expressions
```
BlockExpr = "{" { Statement } "}" ;
```
The value of a block is the value of its trailing expression statement
(if the last statement is an `Expression` with no terminating semicolon
semantics enforced at this grammar level — value production is handled
by the IR generator).

---

## 5. Type Expression Grammar

```
TypeExpr =
    IDENT                                  -- named type: Int, Bool, Foo
  | IDENT "<" TypeExpr {"," TypeExpr} ">"  -- generic: List<Int>, HashMap<K,V>
  | "(" ")"                                -- unit type
  | "(" TypeExpr ")"                       -- parenthesized
  | "(" TypeExpr {"," TypeExpr} ")"        -- tuple: (Int, Float)
  | "(" TypeExpr {"," TypeExpr} ")" "->" TypeExpr   -- fn type via tuple form
  | "fn" "(" [TypeExpr {"," TypeExpr}] ")" ["->" TypeExpr]  -- fn(Int) -> Bool
  | "&" ["mut"] TypeExpr                   -- reference
  | "&" ["mut"] "[" TypeExpr "]"           -- slice: &[Int]
  | "*" ["mut"] TypeExpr                   -- raw pointer
  | "[" TypeExpr [";" ...] "]"             -- array (with optional const-size expr)
  | "Self" | "self"                        -- Self type
  ;
```

Zamani-specific type qualifiers exist in the AST (`TypeExpr::Quantum`,
`Linear`, `Affine`, `Temporal`, `Optional`, `Result`, `Never`) as semantic
targets; surface syntax for some of these (e.g. `T?` optional sugar) is
planned but not yet exposed through `parse_type_expr` — use the explicit
generic forms (`Optional<T>`, `Result<T, E>`) until postfix `?` sugar for
types lands.

---

## 6. Patterns (Match Arms)

Current `match` arms in the parser accept any `Expression` on the left-hand
side of `=>` (literal patterns, identifiers as bindings, etc.). Structural
destructuring patterns (tuple/struct/enum patterns) are a planned extension
tracked in the roadmap — see `ZAMANI_LANGUAGE_SPEC` docs.

---

## 7. Complete Example

```zamani
language Zamani "1.0";

import stdlib.math;
use quantum::gates::*;

type PatientId = String;

struct Patient {
    id: PatientId,
    age: Int,
}

trait Greet {
    fn hello(name: String) -> String;
}

class Robot extends Machine {
    public name: String,

    new(name: String) {
        this.name = name;
    }

    fn speak() -> String {
        return "beep";
    }
}

effect QuantumDecoherence;

quantum circuit Bell {
    let q = quantum hadamard(q0);
}

nano agent Healer {
    let dose = 10;
}

remember wisdom_of_elders = 42;

fn plan(patient: Patient) -> Int {
    let past = zamani { recall(wisdom_of_elders) };
    let now = sasa { patient.age };
    let insight = learn from now;

    handle QuantumDecoherence {
        let r = perform QuantumDecoherence(reason: "noise");
    } with {
        println("recovered");
    }

    match patient.age {
        0 => 0,
        _ => now,
    }
}
```

---

## 8. Grammar ↔ Compiler Cross-Reference

| Grammar section        | Source of truth              |
|-------------------------|-------------------------------|
| Lexical grammar         | `src/lexer.rs` (`TokenType`, keyword map, `next_token`) |
| Statement grammar        | `src/parser.rs::parse_statement` and its callees |
| Expression grammar       | `src/parser.rs::parse_prefix`, `parse_infix`, `Precedence::of` |
| Type expression grammar  | `src/parser.rs::parse_type_expr` |
| AST node shapes          | `src/ast/mod.rs` (`Statement`, `Expression`, `TypeExpr`) |
| Semantic rules           | `src/semantic.rs` |
| Lowering to IR           | `src/ir_gen.rs` |

This file should be updated whenever the parser or lexer's accepted syntax
changes, so it always reflects what actually compiles — not just what is
aspirationally documented elsewhere in the `ZAMANI_*.md` specs.
