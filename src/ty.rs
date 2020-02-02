#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Ty {
    Bool,
    Int,
    Func(Vec<Ty>, Box<Ty>),
}
