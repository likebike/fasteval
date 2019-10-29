// === Algebra Grammar ===
//
// Expression: Value (BinaryOp Value)*
//
// Value: Constant || UnaryOp || Callable || Variable
// #^^^ Variable must be last to avoid masking.
//
// Constant: (\.[0-9])+(k || K || M || G || T)?
//
// Variable: [a-zA-Z_][a-zA-Z_0-9]*
//
// UnaryOp: +Value || -Value || (Expression) || !Value
//
// BinaryOp: + || - || * || / || % || ^ || < || <= || == || != || >= || > || or || and
//
// Callable: PrintFunc || EvalFunc || Function
// #^^^ Function must be last to avoid masking.
//
// Function: Variable(Expression(,Expression)*)
//
// PrintFunc: print(ExpressionOrString,*)
//
// ExpressionOrString: Expression || String
//
// String: ".*"
//
// EvalFunc: eval(Expression(,Variable=Expression)*)

// #[derive(Debug, PartialEq)]
// pub struct Expression(pub Box<[ExpressionTok]>);  // This data structure allows invalid states to exist, but it's so convenient!
//
// #[derive(Debug, PartialEq)]
// pub enum ExpressionTok {
//     EValue(Value),
//     EBinaryOp(BinaryOp),
// }

#[derive(Debug, PartialEq)]  // More awkward, but unable to represent invalid states.
pub struct Expression {
    pub first: Box<Value>,  // Box here so I don't need to have dozens of boxes below.
    pub pairs: Box<[ExprPair]>,
}

#[derive(Debug, PartialEq)]
pub struct ExprPair(pub BinaryOp, pub Value);

#[derive(Debug, PartialEq)]
pub enum Value {
    EConstant(Constant),
    EVariable(Variable),
    EUnaryOp(UnaryOp),
    ECallable(Callable),
}

#[derive(Debug, PartialEq)]
pub struct Constant(pub f64);

#[derive(Debug, PartialEq)]
pub struct Variable(pub String);

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    EPos(Box<Value>),
    ENeg(Box<Value>),
    ENot(Box<Value>),
    EParens(Expression),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BinaryOp {
    EPlus,
    EMinus,
    EMul,
    EDiv,
    EMod,
    EExp,
    ELT,
    ELTE,
    EEQ,
    ENE,
    EGTE,
    EGT,
    EOR,
    EAND,
}

#[derive(Debug, PartialEq)]
pub enum Callable {
    EFunc(Func),
    EPrintFunc(PrintFunc),
    EEvalFunc(EvalFunc),
}

#[derive(Debug, PartialEq)]
pub enum Func {
    EFuncInt(Expression),
    EFuncCeil(Expression),
    EFuncFloor(Expression),
    EFuncAbs(Expression),
    EFuncLog{     base:Option<Expression>, val:Expression},
    EFuncRound{modulus:Option<Expression>, val:Expression},
    EFuncMin{first:Expression, rest:Box<[Expression]>},
    EFuncMax{first:Expression, rest:Box<[Expression]>},
    //
    EFuncE,
    EFuncPi,
    //
    EFuncSin(Expression),
    EFuncCos(Expression),
    EFuncTan(Expression),
    EFuncASin(Expression),
    EFuncACos(Expression),
    EFuncATan(Expression),
    EFuncSinH(Expression),
    EFuncCosH(Expression),
    EFuncTanH(Expression),
}

#[derive(Debug, PartialEq)]
pub struct PrintFunc(pub Box<[ExpressionOrString]>);

#[derive(Debug, PartialEq)]
pub enum ExpressionOrString {
    EExpr(Expression),
    EStr(String),
}

#[derive(Debug, PartialEq)]
pub struct EvalFunc {
    pub expr:   Expression,
    pub kwargs: Box<[KWArg]>,
}

#[derive(Debug, PartialEq)]
pub struct KWArg {
    pub name: Variable,
    pub expr: Expression,
}

