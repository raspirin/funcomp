use crate::ast::{Expr, Stmt};

macro_rules! walk_list {
    ($visitor: expr, $method: ident, $list: expr) => {{
        for elem in $list {
            $visitor.$method(elem)
        }
    }};
}

pub trait Visitor<'ast>: Sized {
    fn visit_expr(&mut self, expr: &Expr<'ast>) {
        walk_expr(self, expr);
    }

    fn visit_stmt(&mut self, stmt: &Stmt<'ast>) {
        walk_stmt(self, stmt);
    }
}

pub fn walk_expr<'ast, V: Visitor<'ast>>(visitor: &mut V, expr: &Expr<'ast>) {
    match expr {
        Expr::Binary(lhs, _, rhs) => {
            visitor.visit_expr(lhs);
            visitor.visit_expr(rhs);
        }
        Expr::Unary(_, operand) => {
            visitor.visit_expr(operand);
        }
        Expr::Call(callee, args) => {
            visitor.visit_expr(callee);
            walk_list!(visitor, visit_expr, args);
        }
        Expr::Grouping(expr) => {
            visitor.visit_expr(expr);
        }
        Expr::Lit(_) => {}
        Expr::Ident(_) => {}
    }
}

pub fn walk_stmt<'ast, V: Visitor<'ast>>(visitor: &mut V, stmt: &Stmt<'ast>) {
    match stmt {
        Stmt::Draw(a0, a1, a2, a3, a4, a5) => {
            walk_list!(visitor, visit_expr, vec![a0, a1, a2, a3, a4, a5]);
        }
        Stmt::Rot(a0) => {
            visitor.visit_expr(a0)
        }
        Stmt::Scale(a0) => {
            visitor.visit_expr(a0)
        }
        Stmt::Origin(a0) => {
            visitor.visit_expr(a0)
        }
        Stmt::EOI => {}
    }
}
