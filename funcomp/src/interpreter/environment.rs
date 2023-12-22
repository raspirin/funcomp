use std::collections::HashMap;

pub enum IdentTy {
    Var,
    Func,
    Const,
}

pub struct Environment {
    pub lookup: HashMap<String, IdentTy>,
}

macro_rules! lookup_list {
    ($lookup: expr, $ty: expr, $list: expr) => {{
        for elem in $list {
            $lookup.insert(elem.into(), $ty);
        }
    }};
}

impl Default for Environment {
    fn default() -> Self {
        let mut lookup = HashMap::new();
        lookup.insert("T".into(), IdentTy::Var);
        lookup_list!(
            lookup,
            IdentTy::Func,
            ["Sin", "Cos", "Exp", "Sqrt", "Ln"]
        );
        lookup.insert("PI".into(), IdentTy::Const);
        Self { lookup }
    }
}
