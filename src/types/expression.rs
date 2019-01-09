use super::{Operation, ParseResult};

pub enum Expression {
    Val(i64),
    Node(Box<Expression>, Operation, Box<Expression>),
}

impl Expression {
    pub fn eval(&self) -> ParseResult {
        match self {
            Expression::Val(val) => Ok(*val),
            Expression::Node(lhs, op, rhs) => match (lhs.eval(), rhs.eval()) {
                (Ok(x), Ok(y)) => op(x, y),
                (Err(e), _) | (_, Err(e)) => Err(e),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_val() {
        let exp = super::Expression::Val(5);
        assert_eq!(exp.eval(), Ok(5));
    }

    #[test]
    fn test_node() {
        use super::{Expression, super::super::{add, subtract}};
        let exp = Expression::Node(
            Box::new(Expression::Val(5)),
            add,
            Box::new(Expression::Node(
                Box::new(Expression::Val(15)),
                subtract,
                Box::new(Expression::Val(5)),
            )),
        );
        assert_eq!(exp.eval(), Ok(15));
    }
}
