use funcomp_derive::ItemKind;

#[macro_export]
macro_rules! P {
    ($ty: ty) => {::std::boxed::Box<$ty>};
}

#[macro_export]
macro_rules! p {
    ($e: expr) => {
        ::std::boxed::Box::new($e)
    };
}

#[derive(Debug)]
pub enum BinOp {
    Plus,
    Minus,
    Asterisk,
    Slash,
}

#[derive(Debug)]
pub enum UnOp {
    Neg,
    Pos,
}

#[derive(Debug)]
pub enum Lit {
    Number(f32),
}

#[derive(Debug)]
pub struct Ident<'ast> {
    pub name: &'ast str,
}

#[derive(ItemKind, Debug)]
pub enum Expr<'ast> {
    Binary(Box<Expr<'ast>>, BinOp, Box<Expr<'ast>>),
    Unary(UnOp, Box<Expr<'ast>>),
    Call(Box<Expr<'ast>>, Vec<Expr<'ast>>),
    Grouping(Box<Expr<'ast>>),
    Lit(Lit),
    Ident(Ident<'ast>),
}

#[derive(ItemKind, Debug)]
pub enum Stmt<'ast> {
    Draw(P!(Expr<'ast>), P!(Expr<'ast>), P!(Expr<'ast>), P!(Expr<'ast>), P!(Expr<'ast>), P!(Expr<'ast>)),
    Rot(P!(Expr<'ast>)),
    Scale(P!(Expr<'ast>)),
    Origin(P!(Expr<'ast>)),
}
