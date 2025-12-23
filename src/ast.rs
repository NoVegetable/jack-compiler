pub type Identifier<'source> = &'source str;
pub type ClassName<'source> = Identifier<'source>;
pub type SubroutineName<'source> = Identifier<'source>;
pub type VarName<'source> = Identifier<'source>;

#[derive(Debug, Clone, PartialEq)]
pub struct Class<'source> {
    pub name: ClassName<'source>,
    pub variables: Vec<ClassVarDec<'source>>,
    pub subroutines: Vec<SubroutineDec<'source>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassVarDec<'source> {
    pub kind: VarKind,
    pub ty: Ty<'source>,
    pub names: Vec<VarName<'source>>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VarKind {
    Static,
    Field,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ty<'source> {
    Int,
    Char,
    Boolean,
    Class(ClassName<'source>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubroutineDec<'source> {
    pub kind: SubroutineKind,
    pub return_ty: SubroutineReturnTy<'source>,
    pub name: SubroutineName<'source>,
    pub params: ParameterList<'source>,
    pub body: SubroutineBody<'source>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SubroutineKind {
    Constructor,
    Function,
    Method,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SubroutineReturnTy<'source> {
    Void,
    Type(Ty<'source>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterList<'source>(pub Vec<Parameter<'source>>);

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter<'source> {
    pub ty: Ty<'source>,
    pub name: VarName<'source>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubroutineBody<'source> {
    pub variables: Vec<VarDec<'source>>,
    pub stmts: Stmts<'source>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDec<'source> {
    pub ty: Ty<'source>,
    pub names: Vec<VarName<'source>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stmts<'source>(pub Vec<Stmt<'source>>);

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<'source> {
    Let(LetStmt<'source>),
    If(IfStmt<'source>),
    While(WhileStmt<'source>),
    Do(DoStmt<'source>),
    Return(ReturnStmt<'source>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStmt<'source> {
    pub var_name: VarName<'source>,
    pub idx_expr: Option<Expression<'source>>,
    pub assign_expr: Expression<'source>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt<'source> {
    pub condition: Expression<'source>,
    pub stmts: Stmts<'source>,
    pub else_stmts: Option<Stmts<'source>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStmt<'source> {
    pub condition: Expression<'source>,
    pub stmts: Stmts<'source>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoStmt<'source> {
    pub call: SubroutineCall<'source>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt<'source> {
    pub return_val: Option<Expression<'source>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression<'source> {
    pub leading_term: Box<Term<'source>>,
    pub following_terms: Vec<(Op, Box<Term<'source>>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term<'source> {
    IntegerConst(u16),
    StringConst(&'source str),
    KeywordConst(KeywordConst),
    VarRef(VarName<'source>),
    VarRefWithIdx(VarName<'source>, Expression<'source>),
    SubroutineCall(SubroutineCall<'source>),
    Expr(Expression<'source>),
    UnaryOperation(UnaryOp, Box<Term<'source>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubroutineCall<'source> {
    pub prefix: Option<Identifier<'source>>,
    pub name: SubroutineName<'source>,
    pub args: ExpressionList<'source>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionList<'source>(pub Vec<Expression<'source>>);

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
