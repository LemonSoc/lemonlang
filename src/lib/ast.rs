use std::sync::Arc;

struct P<T> {
    inner: Arc<T>,
}

pub type NodeId = u64;

pub struct Expr {
    pub id: NodeId,
    pub kind: ExprKind,
    // TODO: add attrs
}

pub struct Type {}

enum UnOp {
    Discard,
    Deref,
    Not,
}

enum BinOp {}

struct IdentExpr;
struct MethodCall;
struct CompoundExpr;
struct Param;
struct IfExpr;
struct WhileExpr;
struct LetExpr;
struct AssignExpr;
pub struct CallExpr;
pub struct LiteralExpr;

pub enum ExprK {
    Call(CallExpr),
    Binary(BinOp, P<Expr>, P<Expr>),
    Unary(UnOp, P<Expr>),
    Let(P<IdentExpr>, P<Expr>),
    If(IfExpr),
    While(WhileExpr),
    MethodCall(P<MethodCall>),
    // TODO: pattern matching
    CompoundExpr(P<Vec<Expr>>, Option<P<Expr>>),
    AssignExpr(P<Expr>, P<Expr>),
    Closure(P<Vec<Param>>, P<CompoundExpr>),
    Return(P<Expr>),
    Literal(LiteralExpr),
    Ident(P<IdentExpr>),
    Unit,
}

// TODO:
// Loops,
// Pattern Matching,
// Structs,
// Enums,
// Traits,
// Generics,
// Functions,
// Methods,
// Indexes and Arrays
// return stmt
//

pub enum ExprKind {
    /// An array (`[a, b, c, d]`)
    Array(Vec<P<Expr>>),
    /// (Self-Name, Args Output)
    Call(P<Expr>, Vec<P<Expr>>),
    /// A method call (e.g. `x.foo::<Bar, Baz>(a, b, c)`).
    MethodCall(Box<MethodCall>),
    /// A binary operation (e.g., `a + b`, `a * b`).
    Binary(BinOp, P<Expr>, P<Expr>),
    /// A unary operation (e.g., `!x`, `*x`).
    Unary(UnOp, P<Expr>),
    /// A literal (e.g., `1`, `"foo"`).
    // Lit(token::Lit),
    /// A type ascription (e.g., `42: usize`).
    // Type(P<Expr>, P<Ty>),
    /// A `let pat = expr` expression that is only semantically allowed in the condition
    /// of `if` / `while` expressions. (e.g., `if let 0 = x { .. }`).
    ///
    /// `Span` represents the whole `let pat = expr` statement.
    Let(P<IdentExpr>, P<Expr>),
    /// An `if` block, with an optional `else` block.
    ///
    /// `if expr { block } else { expr }`
    If(P<Expr>, P<Block>, Option<P<Expr>>),
    /// A while loop, with an optional label.
    ///
    /// `'label: while expr { block }`
    While(P<Expr>, P<CompundExpr>),
    /// A `for` loop, with an optional label.
    ///
    /// `'label: for pat in expr { block }`
    ///
    /// This is desugared to a combination of `loop` and `match` expressions.
    ForLoop(P<Pat>, P<Expr>, P<Block>, Option<Label>),
    /// Conditionless loop (can be exited with `break`, `continue`, or `return`).
    ///
    /// `'label: loop { block }`
    Loop(P<Block>, Option<Label>, Span),
    /// A `match` block.
    Match(P<Expr>, ThinVec<Arm>),
    /// A closure (e.g., `move |a, b, c| a + b + c`).
    Closure(Box<Closure>),
    /// A block (`'label: { ... }`).
    Block(P<Block>, Option<Label>),
    /// An async block (`async move { ... }`).
    ///
    /// The async block used to have a `NodeId`, which was removed in favor of
    /// using the parent `NodeId` of the parent `Expr`.
    Async(CaptureBy, P<Block>),
    /// An await expression (`my_future.await`). Span is of await keyword.
    Await(P<Expr>, Span),

    /// A try block (`try { ... }`).
    TryBlock(P<Block>),

    /// An assignment (`a = foo()`).
    /// The `Span` argument is the span of the `=` token.
    Assign(P<Expr>, P<Expr>, Span),
    /// An assignment with an operator.
    ///
    /// E.g., `a += 1`.
    AssignOp(BinOp, P<Expr>, P<Expr>),
    /// Access of a named (e.g., `obj.foo`) or unnamed (e.g., `obj.0`) struct field.
    Field(P<Expr>, Ident),
    /// An indexing operation (e.g., `foo[2]`).
    /// The span represents the span of the `[2]`, including brackets.
    Index(P<Expr>, P<Expr>, Span),
    /// A range (e.g., `1..2`, `1..`, `..2`, `1..=2`, `..=2`; and `..` in destructuring assignment).
    Range(Option<P<Expr>>, Option<P<Expr>>, RangeLimits),
    /// An underscore, used in destructuring assignment to ignore a value.
    Underscore,

    /// Variable reference, possibly containing `::` and/or type
    /// parameters (e.g., `foo::bar::<baz>`).
    ///
    /// Optionally "qualified" (e.g., `<Vec<T> as SomeTrait>::SomeType`).
    Path(Option<P<QSelf>>, Path),

    /// A referencing operation (`&a`, `&mut a`, `&raw const a` or `&raw mut a`).
    AddrOf(BorrowKind, Mutability, P<Expr>),
    /// A `break`, with an optional label to break, and an optional expression.
    Break(Option<Label>, Option<P<Expr>>),
    /// A `continue`, with an optional label.
    Continue(Option<Label>),
    /// A `return`, with an optional value to be returned.
    Ret(Option<P<Expr>>),

    /// Output of the `asm!()` macro.
    InlineAsm(P<InlineAsm>),

    /// Output of the `offset_of!()` macro.
    OffsetOf(P<Ty>, P<[Ident]>),

    /// A macro invocation; pre-expansion.
    MacCall(P<MacCall>),

    /// A struct literal expression.
    ///
    /// E.g., `Foo {x: 1, y: 2}`, or `Foo {x: 1, .. rest}`.
    Struct(P<StructExpr>),

    /// An array literal constructed from one repeated element.
    ///
    /// E.g., `[1; 5]`. The expression is the element to be
    /// repeated; the constant is the number of times to repeat it.
    Repeat(P<Expr>, AnonConst),

    /// No-op: used solely so we can pretty-print faithfully.
    Paren(P<Expr>),

    /// A try expression (`expr?`).
    Try(P<Expr>),

    /// A `yield`, with an optional value to be yielded.
    Yield(Option<P<Expr>>),

    /// A `do yeet` (aka `throw`/`fail`/`bail`/`raise`/whatever),
    /// with an optional value to be returned.
    Yeet(Option<P<Expr>>),

    /// A tail call return, with the value to be returned.
    ///
    /// While `.0` must be a function call, we check this later, after parsing.
    Become(P<Expr>),

    /// Bytes included via `include_bytes!`
    /// Added for optimization purposes to avoid the need to escape
    /// large binary blobs - should always behave like [`ExprKind::Lit`]
    /// with a `ByteStr` literal.
    IncludedBytes(Lrc<[u8]>),

    /// A `format_args!()` expression.
    FormatArgs(P<FormatArgs>),

    /// Placeholder for an expression that wasn't syntactically well formed in some way.
    Err,
}
