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
    pub first: Value,
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
    EParens(Box<Expression>),
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
    EFuncInt(Box<Expression>),
    EFuncCeil(Box<Expression>),
    EFuncFloor(Box<Expression>),
    EFuncAbs(Box<Expression>),
    EFuncLog{     base:Option<Box<Expression>>, val:Box<Expression>},
    EFuncRound{modulus:Option<Box<Expression>>, val:Box<Expression>},
    EFuncMin{first:Box<Expression>, rest:Box<[Expression]>},
    EFuncMax{first:Box<Expression>, rest:Box<[Expression]>},
    //
    EFuncE,
    EFuncPi,
    //
    EFuncSin(Box<Expression>),
    EFuncCos(Box<Expression>),
    EFuncTan(Box<Expression>),
    EFuncASin(Box<Expression>),
    EFuncACos(Box<Expression>),
    EFuncATan(Box<Expression>),
    EFuncSinH(Box<Expression>),
    EFuncCosH(Box<Expression>),
    EFuncTanH(Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub struct PrintFunc(pub Box<[ExpressionOrString]>);

#[derive(Debug, PartialEq)]
pub enum ExpressionOrString {
    EExpr(Box<Expression>),
    EStr(String),
}

#[derive(Debug, PartialEq)]
pub struct EvalFunc {
    pub expr:   Box<Expression>,
    pub kwargs: Box<[KWArg]>,
}

#[derive(Debug, PartialEq)]
pub struct KWArg {
    pub name: Variable,
    pub expr: Box<Expression>,
}

