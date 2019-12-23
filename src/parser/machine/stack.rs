use crate::parser::syntax::Fragment;

pub struct Stack(Vec<Fragment>);

impl Stack {
    pub fn new() -> Self {
        Stack(Vec::new())
    }

    pub fn push(&mut self, f: Fragment) {
        self.0.push(f);
    }
}

