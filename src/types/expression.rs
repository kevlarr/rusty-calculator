use ::std::cmp::PartialEq;
use ::std::fmt;

use super::{Operation, ParseResult};

pub enum Expression {
    Val(i64),
    Node(Box<Expression>, Operation, Box<Expression>),
}

impl Expression {
    /// Parses string input into an Expression tree.
    /// Sub-expressions require parentheses, eg. "(104 + 1) - 15".
    pub fn from_str(s: &str) -> Expression {
        use super::super::{add};

        Expression::Node(
            Box::new(Expression::Val(5)),
            add,
            Box::new(Expression::Val(12)),
        )
    }

    /// Evaluates self expression, which in turn will evaluate all
    /// child expressions if self is a Node
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

impl PartialEq for Expression {
    fn eq(&self, other: &Expression) -> bool {
        use self::Expression::*;

        match (self, other) {
            (Val(x), Val(y)) => x == y,
            (Node(lhs1, op1, rhs1), Node(lhs2, op2, rhs2)) => {
                lhs1 == lhs2 && op1 == op2 && rhs1 == rhs2
            },
            (_, _) => false,
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expression::*;

        match self {
            Val(x) =>
                write!(f, "{}", x)?,
            Node(lhs, op, rhs) =>
                write!(f, "({:?} op {:?})", lhs, rhs)?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_eq() {
    }

    #[test]
    fn test_from_str() {
        use super::{
            super::super::{add, multiply, subtract},
            Expression::{self, Node, Val},
        };

        let str1 = "5 + 12";
        let str2 = "(15-3)*12";

        assert_eq!(
            Expression::from_str(str1),
            Node(Box::new(Val(5)), add, Box::new(Val(12))),
        );
        assert_eq!(
            Expression::from_str(str2),
            Node(
                Box::new(Node(
                    Box::new(Val(15)),
                    subtract,
                    Box::new(Val(3)),
                )),
                multiply,
                Box::new(Val(12)),
            )
        );
    }

    #[test]
    fn test_val() {
        use super::Expression::Val;

        let exp = Val(5);
        assert_eq!(exp.eval(), Ok(5));
    }

    #[test]
    fn test_node() {
        use super::{Expression::*, super::super::{add, subtract}};

        let exp = Node(
            Box::new(Val(5)),
            add,
            Box::new(Node(
                Box::new(Val(15)),
                subtract,
                Box::new(Val(5)),
            )),
        );
        assert_eq!(exp.eval(), Ok(15));
    }
}
