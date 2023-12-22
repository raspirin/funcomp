use crate::ast::{Expr, Stmt};
use crate::interpreter::environment::{Environment, IdentTy};
use crate::interpreter::visit::{Visitor, walk_expr, walk_stmt};

#[derive(Eq, PartialEq)]
pub enum ValueType {
    Callable,
    Const,
    Var,
}

#[derive(Default)]
pub struct StaticChecker {
    pub stack: Vec<ValueType>,
    pub environment: Environment,
}

impl StaticChecker {
    pub fn check(&mut self, src: &[Stmt]) {
        for stmt in src.iter() {
            self.visit_stmt(stmt)
        }
    }
}

impl<'ast> Visitor<'ast> for StaticChecker {
    fn visit_expr(&mut self, expr: &Expr<'ast>) {
        walk_expr(self, expr);
        match expr {
            Expr::Binary(_, _, _) => {
                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                match (lhs, rhs) {
                    (ValueType::Const, ValueType::Const) => self.stack.push(ValueType::Const),
                    (ValueType::Var, ValueType::Var) => self.stack.push(ValueType::Var),
                    (ValueType::Const, ValueType::Var) | (ValueType::Var, ValueType::Const) => self.stack.push(ValueType::Var),
                    (ValueType::Callable, _) | (_, ValueType::Callable) => self.stack.push(ValueType::Callable),
                }
            }
            Expr::Unary(_, _) => {
                let operand = self.stack.pop().unwrap();
                self.stack.push(operand);
            }
            Expr::Call(_, args) => {
                let mut args_ty = vec![];
                for _ in args {
                    let ty = self.stack.pop().unwrap();
                    args_ty.push(ty);
                }
                if args_ty.len() != 1 {
                    panic!("Too much arguments.")
                }

                let callee = self.stack.pop().unwrap();
                if callee != ValueType::Callable {
                    panic!("Expect a valid func in call-expr.")
                }
                if args_ty.iter().all(|ty| ValueType::Const == *ty) {
                    self.stack.push(ValueType::Const);
                } else {
                    self.stack.push(ValueType::Callable);
                }
            }
            Expr::Grouping(_) => {
                let inner = self.stack.pop().unwrap();
                self.stack.push(inner);
            }
            Expr::Lit(_) => {
                self.stack.push(ValueType::Const)
            }
            Expr::Ident(ident) => {
                let name = ident.name;
                let lookup = self.environment.lookup.get(name);
                if let Some(ty) = lookup {
                    match ty {
                        IdentTy::Var => self.stack.push(ValueType::Var),
                        IdentTy::Func => self.stack.push(ValueType::Callable),
                        IdentTy::Const => self.stack.push(ValueType::Const),
                    }
                } else {
                    panic!("Expect a valid ident.")
                }
            }
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt<'ast>) {
        walk_stmt(self, stmt);
        match stmt {
            Stmt::Draw(_, _, _, _, _, _) => {
                let _y = self.stack.pop().unwrap();
                let _x = self.stack.pop().unwrap();
                let step = self.stack.pop().unwrap();
                let to = self.stack.pop().unwrap();
                let from = self.stack.pop().unwrap();
                let ident = self.stack.pop().unwrap();

                if ident != ValueType::Var {
                    panic!("Expect an Ident after For.")
                }
                if from != ValueType::Const || to != ValueType::Const || step != ValueType::Const {
                    panic!("Expect a Const in <from>/<to>/<step>")
                }
            }
            Stmt::Rot(_) | Stmt::Scale(_) | Stmt::Origin(_) => {
                let expr = self.stack.pop().unwrap();
                if expr != ValueType::Const {
                    panic!("Expect a Const in Rot/Scale/Origin")
                }
            }
            Stmt::EOI => {}
        }
    }
}