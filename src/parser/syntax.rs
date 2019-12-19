/// The supported binary operations for building a syntax tree.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Mod,
}

/// The possible syntax tree elements.
///
/// While a single literal value (eg. "5") is valid syntax, more
/// interesting (read: useful) things happen when the original
/// expression is a little more complex.
///
/// Additionally, while "5 * 3 + 2" is valid syntax even without
/// parentheses, the expression will be converted to a tree equivalent
/// to "(5 * 3) + 2" such that any expression can be described in terms
/// of either a single literal value or a binary operation whose operands
/// are potentially nested expressions.
#[derive(Debug, PartialEq)]
pub enum Syntax {
    Empty,
    Expression(Box<Syntax>, BinaryOp, Box<Syntax>),
    Literal(i64),
}

/// Blergh.
#[derive(Debug, PartialEq)]
pub struct AST(Syntax);

impl AST {
    pub fn new() -> Self {
        AST(Syntax::Empty)
    }

    pub fn with_syntax(s: Syntax) -> Self {
        AST(s)
    }
}
