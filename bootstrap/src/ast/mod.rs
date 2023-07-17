#![allow(dead_code)]

pub mod token;

pub type NodeId = usize;

pub const CRATE_NODE: NodeId = 0;

pub struct Ast {
    nodes: Vec<Node>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            nodes: vec![
            ],
        }
    }
}

pub struct Node {
    data:       NodeKind,
    parent:     Option<NodeId>,
    children:   Option<NodeId>,
    statics:    Vec<NodeId>,
}

enum NodeKind {
    Crate(Crate),
    Module(Module),
    
}

struct Crate;
struct Module;


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct SourcePos {
    pub line:   usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct SourceRange {
    pub start:  SourcePos,
    pub end:    SourcePos,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct SourceView<'v> {
    pub range: SourceRange,
    pub view:  &'v str,
}
