use crate::parser::AstNode;

pub fn eval(ast: AstNode) -> f64 {
    // This evaluates the AST recursively.

    match ast {
        AstNode::Add(left, right) => eval(*left) + eval(*right),
        AstNode::Sub(left, right) => eval(*left) - eval(*right),
        AstNode::Mul(left, right) => eval(*left) * eval(*right),
        AstNode::Div(left, right) => eval(*left) / eval(*right),
        AstNode::Num(n) => n,
    }
}
