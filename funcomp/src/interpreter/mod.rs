use crate::ast::Expr;
use crate::interpreter::environment::Environment;
use crate::interpreter::visit::{Visitor, walk_expr};

pub mod environment;
pub mod visit;
pub mod typeck;

#[derive(Clone)]
pub struct State {
    pub rot: f32,
    pub origin: f32,
    pub scale: (f32, f32),
}

impl State {
    pub fn set_scale_x(&mut self, x: f32) {
        self.scale.0 = x;
    }

    pub fn set_scale_y(&mut self, y: f32) {
        self.scale.1 = y;
    }
}

pub struct Interpreter<'ast> {
    pub environment: Environment,
    pub state: State,
    pub cal_stack: Vec<Expr<'ast>>,
}

impl<'ast> Visitor<'ast> for Interpreter<'ast> {
    fn visit_expr(&mut self, expr: &Expr<'ast>) {
        walk_expr(self, expr);
        match expr {
            Expr::Binary(_, _, _) => {}
            Expr::Unary(_, _) => {}
            Expr::Call(_, _) => {}
            Expr::Grouping(_) => {}
            Expr::Lit(_) => {}
            Expr::Ident(ident) => {
            }
        }
    }
}
