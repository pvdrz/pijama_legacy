use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use pijama_common::{generator::Generator, location::Location, BinOp, Literal, Primitive, UnOp};

use pijama_ty::Ty;

pub use lower::{LowerError, LowerResult, lower_block};

mod lower;

#[derive(Debug, Copy, Clone)]
pub enum BindKind {
    NonRec,
    Rec,
}

#[derive(Debug)]
pub struct Term {
    pub id: TermId,
    pub kind: TermKind,
}

impl Term {
    pub(crate) fn new(id: TermId, kind: TermKind) -> Self {
        Self { id, kind }
    }
}

#[derive(Debug)]
pub enum TermKind {
    Lit(Literal),
    PrimFn(Primitive),
    Var(LocalId),
    Abs(LocalId, Box<Term>),
    App(Box<Term>, Box<Term>),
    UnaryOp(UnOp, Box<Term>),
    BinaryOp(BinOp, Box<Term>, Box<Term>),
    Cond(Box<Term>, Box<Term>, Box<Term>),
    Let(BindKind, LocalId, Box<Term>, Box<Term>),
}
//
// impl<'a> Display for Term<'a> {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         match self {
//             Term::Var(var) => write!(f, "{}", var),
//             Term::Abs(name, ty, term) => write!(f, "(λ{}:{}. {})", name, ty, term),
//             Term::UnaryOp(op, term) => write!(f, "({}{})", op, term),
//             Term::BinaryOp(op, t1, t2) => write!(f, "({} {} {})", t1, op, t2),
//             Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
//             Term::Lit(literal) => write!(f, "{}", literal),
//             Term::Cond(t1, t2, t3) => write!(f, "(if {} then {} else {})", t1, t2, t3),
//             Term::Let(LetKind::Rec(ty), name, t1, t2) => {
//                 write!(f, "(let rec {} : {} = {} in {})", name, ty.content, t1, t2)
//             }
//             Term::Let(LetKind::NonRec(Some(ty)), name, t1, t2) => {
//                 write!(f, "(let {} : {} = {} in {})", name, ty.content, t1, t2)
//             }
//             Term::Let(LetKind::NonRec(None), name, t1, t2) => {
//                 write!(f, "(let {} = {} in {})", name, t1, t2)
//             }
//             Term::PrimFn(prim) => write!(f, "{}", prim),
//         }
//     }
// }
//
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct LocalId(usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TermId(usize);

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub ty: Ty,
    pub loc: Location,
}

pub struct Store<Id> {
    gen_id: Generator<Id>,
    locations: HashMap<Id, Location>,
    type_info: HashMap<Id, TypeInfo>,
}

impl<Id: Debug + Hash + Eq + Copy> Store<Id> {
    pub(crate) fn new_id(&mut self) -> Id {
        self.gen_id.gen()
    }

    pub(crate) fn insert_location(&mut self, id: Id, loc: Location) {
        assert!(
            self.locations.insert(id, loc).is_none(),
            "Overwrote location for {:?}",
            id
        );
    }
    pub fn insert_type_info(&mut self, id: Id, type_info: TypeInfo) {
        assert!(
            self.type_info.insert(id, type_info).is_none(),
            "Overwrote type information for {:?}",
            id
        );
    }

    pub fn get_location(&self, id: Id) -> Option<Location> {
        self.locations.get(&id).cloned()
    }

    pub fn get_type_info(&self, id: Id) -> Option<&TypeInfo> {
        self.type_info.get(&id)
    }

    pub fn types_mut<'ty>(&'ty mut self) -> impl Iterator<Item=&'ty mut Ty> {
        self.type_info.values_mut().map(|info| &mut info.ty)
    }
}

pub struct Context {
    local: Store<LocalId>,
    term: Store<TermId>,
    ty_gen: Generator<Ty>
}

impl Context {
    pub fn new(ty_gen: Generator<Ty>) -> Self {
        Self {
            local: Store {
                gen_id: Generator::new(LocalId),
                locations: HashMap::default(),
                type_info: HashMap::default(),
            },
            term: Store {
                gen_id: Generator::new(TermId),
                locations: HashMap::default(),
                type_info: HashMap::default(),
            },
            ty_gen
        }
    }

    pub fn new_ty(&mut self) -> Ty {
        self.ty_gen.gen()
    }

    pub fn local(&mut self) -> &mut Store<LocalId> {
        &mut self.local
    }

    pub fn term(&mut self) -> &mut Store<TermId> {
        &mut self.term
    }
}
