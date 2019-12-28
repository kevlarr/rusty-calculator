use crate::parser::syntax::Node;

pub struct Stack(Vec<Node>);

impl Stack {
    pub fn new() -> Self {
        Stack(Vec::new())
    }

    pub fn push(&mut self, n: Node) {
        self.0.push(n);
    }
}

