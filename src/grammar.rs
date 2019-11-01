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


use crate::stackvec::{StackVec8, StackVec16};


// #[derive(Debug, PartialEq)]
// pub struct Expression(pub Box<[ExpressionTok]>);  // This data structure allows invalid states to exist, but it's so convenient!
//
// #[derive(Debug, PartialEq)]
// pub enum ExpressionTok {
//     EValue(Value),
//     EBinaryOp(BinaryOp),
// }


#[derive(Debug, PartialEq)]
pub struct ExpressionI(pub usize);
#[derive(Debug, PartialEq)]
pub struct ValueI(pub usize);


#[derive(Debug, PartialEq)]  // More awkward, but unable to represent invalid states, and also more efficient.
pub struct Expression {
    pub first: Value,
    pub pairs: StackVec16<ExprPair>,
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
pub struct Variable(pub StackString32);

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
    EFuncLog{     base:Option<ExpressionI>, val:ExpressionI},
    EFuncRound{modulus:Option<ExpressionI>, val:ExpressionI},
    EFuncMin{first:ExpressionI, rest:StackVec8<ExpressionI>},
    EFuncMax{first:ExpressionI, rest:StackVec8<ExpressionI>},
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
pub struct PrintFunc(pub StackVec16<ExpressionOrString>);

#[derive(Debug, PartialEq)]
pub enum ExpressionOrString {
    EExpr(ExpressionI),
    EStr(StackString256),
}

#[derive(Debug, PartialEq)]
pub struct EvalFunc {
    pub expr:   ExpressionI,
    pub kwargs: StackVec16<KWArg>,
}

#[derive(Debug, PartialEq)]
pub struct KWArg {
    pub name: Variable,
    pub expr: ExpressionI,
}

