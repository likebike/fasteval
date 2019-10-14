// === Algebra Grammar ===
//
// Expression: Value (BinaryOp Value)*
//
// Value: Constant || UnaryOp || Callable || Variable
//
// BinaryOp: + || - || * || / || % || ^ || < || <= || == || != || >= || > || or || and
//
// Constant: (\.[0-9])+(k || K || M || G || T)?
//
// UnaryOp: +Value || -Value || (Expression) || !Value
//
// Callable: Function || PrintFunc || EvalFunc
//
// Function: Variable(Expression(,Expression)*)
//
// Variable: [a-zA-Z_][a-zA-Z_0-9]*
//
// PrintFunc: print(ExpressionOrString,*)
//
// ExpressionOrString: Expression || String
//
// String: ".*"
//
// EvalFunc: eval(Expression(,Variable=Expression)*)

struct Expression([ExpressionTok]);

enum ExpressionTok {
    EValue(Value),
    EBinaryOp(BinaryOp),
}

enum Value {
    EConstant(),
//  EUnaryOp,
//  ECallable,
//  EVariable,
}

enum BinaryOp {
    EPlus,
//  EMinus,
//  EMul,
//  EDiv,
//  EMod,
//  EExp,
//  ELT,
//  ELTE,
//  EEQ,
//  ENE,
//  EGTE,
//  EGT,
//  EOR,
//  EAND,
}

struct Constant(String);
