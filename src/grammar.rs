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


use stacked::{SVec8, SVec16, SString32, SString256};


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ExpressionI(pub usize);
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ValueI(pub usize);


#[derive(Debug, PartialEq)]
pub struct Expression {
    pub first: Value,
    pub pairs: SVec16<ExprPair>,
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
pub struct Variable(pub SString32);

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    EPos(ValueI),
    ENeg(ValueI),
    ENot(ValueI),
    EParens(ExpressionI),
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
    EFuncInt(ExpressionI),
    EFuncCeil(ExpressionI),
    EFuncFloor(ExpressionI),
    EFuncAbs(ExpressionI),
    EFuncLog{     base:Option<ExpressionI>, expr:ExpressionI},
    EFuncRound{modulus:Option<ExpressionI>, expr:ExpressionI},
    EFuncMin{first:ExpressionI, rest:SVec8<ExpressionI>},
    EFuncMax{first:ExpressionI, rest:SVec8<ExpressionI>},
    //
    EFuncE,
    EFuncPi,
    //
    EFuncSin(ExpressionI),
    EFuncCos(ExpressionI),
    EFuncTan(ExpressionI),
    EFuncASin(ExpressionI),
    EFuncACos(ExpressionI),
    EFuncATan(ExpressionI),
    EFuncSinH(ExpressionI),
    EFuncCosH(ExpressionI),
    EFuncTanH(ExpressionI),
}

#[derive(Debug, PartialEq)]
pub struct PrintFunc(pub SVec16<ExpressionOrString>);

#[derive(Debug, PartialEq)]
pub enum ExpressionOrString {
    EExpr(ExpressionI),
    EStr(SString256),
}

#[derive(Debug, PartialEq)]
pub struct EvalFunc {
    pub expr:   ExpressionI,
    pub kwargs: SVec16<KWArg>,
}

#[derive(Debug, PartialEq)]
pub struct KWArg {
    pub name: Variable,
    pub expr: ExpressionI,
}

