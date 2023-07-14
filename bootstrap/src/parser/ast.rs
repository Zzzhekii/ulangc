#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ast {
    nodes: Vec<Node>
}

impl Ast {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Node;
