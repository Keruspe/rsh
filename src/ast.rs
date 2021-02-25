use crate::variable::Variable;

pub(crate) enum Node {
    Decl(String),
    Assign(String, Variable),
    Print(String),
}

#[derive(Default)]
pub(crate) struct Block(Vec<Node>);

impl Block {
    pub(crate) fn push(&mut self, node: Node) {
        self.0.push(node);
    }
}

impl From<Vec<Node>> for Block {
    fn from(nodes: Vec<Node>) -> Self {
        Self(nodes)
    }
}

impl IntoIterator for Block {
    type Item = Node;
    type IntoIter = std::vec::IntoIter<Node>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
