#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramNode {
    pub func: FunctionNode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionNode {
    pub id: String,
    pub statement: StatementNode,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnaryOp {
    BitwiseNot,
    LogicalNot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Constant(usize),
    Unary(UnaryOp, Box<ExprNode>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprNode {
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatementNode {
    pub expr: ExprNode,
}
