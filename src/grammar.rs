// === Algebra Grammar ===
//
// Expression: Value (BinaryOp Value)*
//
// Value: Constant || Variable || UnaryOp || Callable
//
// Constant: (\.[0-9])+(k || K || M || G || T)?
//
// Variable: [a-zA-Z_][a-zA-Z_0-9]*
//
// UnaryOp: +Value || -Value || (Expression) || !Value
//
// BinaryOp: + || - || * || / || % || ^ || < || <= || == || != || >= || > || or || and
//
// Callable: Function || PrintFunc || EvalFunc
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

#[derive(Debug, PartialEq)]
pub struct Expression(pub Box<[ExpressionTok]>);

#[derive(Debug, PartialEq)]
pub enum ExpressionTok {
    EValue(Value),
    EBinaryOp(BinaryOp),
}

#[derive(Debug, PartialEq)]
pub enum Value {
    EConstant(Constant),
    EVariable(Variable),
//  EUnaryOp(UnaryOp),
//  ECallable,
}

#[derive(Debug, PartialEq)]
pub struct Constant(pub f64);

#[derive(Debug, PartialEq)]
pub struct Variable(pub String);

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    EPos(Box<Value>),
    ENeg(Box<Value>),
    EParens(Box<Expression>),
    ENot(Box<Value>),
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

