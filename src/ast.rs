pub type Identifier = String;
pub type ClassName = Identifier;
pub type SubroutineName = Identifier;
pub type VarName = Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: ClassName,
    pub variables: Vec<ClassVarDec>,
    pub subroutines: Vec<SubroutineDec>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassVarDec {
    pub kind: VarKind,
    pub ty: Ty,
    pub names: Vec<VarName>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VarKind {
    Static,
    Field,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ty {
    Int,
    Char,
    Boolean,
    Class(ClassName),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubroutineDec {
    pub kind: SubroutineKind,
    pub return_ty: SubroutineReturnTy,
    pub name: SubroutineName,
    pub params: ParameterList,
    pub body: SubroutineBody,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SubroutineKind {
    Constructor,
    Function,
    Method,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SubroutineReturnTy {
    Void,
    Type(Ty),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterList(pub Vec<Parameter>);

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub ty: Ty,
    pub name: VarName,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubroutineBody {
    pub variables: Vec<VarDec>,
    pub stmts: Stmts,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDec {
    pub ty: Ty,
    pub names: Vec<VarName>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stmts(pub Vec<Stmt>);

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let(LetStmt),
    If(IfStmt),
    While(WhileStmt),
    Do(DoStmt),
    Return(ReturnStmt),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStmt {
    pub var_name: VarName,
    pub idx_expr: Option<Expression>,
    pub assign_expr: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub condition: Expression,
    pub stmts: Stmts,
    pub else_stmts: Option<Stmts>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStmt {
    pub condition: Expression,
    pub stmts: Stmts,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoStmt {
    pub call: SubroutineCall,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub return_val: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub leading_term: Box<Term>,
    pub following_terms: Vec<(Op, Box<Term>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    IntegerConst(u16),
    StringConst(String),
    KeywordConst(KeywordConst),
    VarRef(VarName),
    VarRefWithIdx(VarName, Expression),
    SubroutineCall(SubroutineCall),
    Expr(Expression),
    UnaryOperation(UnaryOp, Box<Term>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubroutineCall {
    pub prefix: Option<Identifier>,
    pub name: SubroutineName,
    pub args: ExpressionList,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionList(pub Vec<Expression>);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Lt,
    Gt,
    Eq,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UnaryOp {
    Negative,
    Neg,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeywordConst {
    True,
    False,
    Null,
    This,
}
