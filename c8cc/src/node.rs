#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramNode {
    pub func: FunctionNode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionNode {
    pub id: String,
    pub statement: StatementNode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprNode {
    pub value: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatementNode {
    pub expr: ExprNode,
}
