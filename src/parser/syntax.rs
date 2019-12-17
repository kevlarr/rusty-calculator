#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Mod,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Val(i64),
    Expr(Box<Node>, Operation, Box<Node>),
    NoOp,
}
