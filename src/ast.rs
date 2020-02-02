#[derive(Debug)]
pub enum Node<'a> {
    Atom(&'a str),
    Seq(Vec<Node<'a>>),
}

impl<'a> Node<'a> {
    pub fn try_name(&self) -> Option<&'a str> {
        match self {
            Node::Atom(name) => Some(name),
            _ => None,
        }
    }

    pub fn try_seq(&self) -> Option<&[Node<'a>]> {
        match self {
            Node::Seq(seq) => Some(seq),
            _ => None,
        }
    }
}
