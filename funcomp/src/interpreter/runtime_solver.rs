use crate::ast::{BinOp, Expr, Lit, UnOp};
use crate::interpreter::environment::{Environment, IdentTy};
use crate::interpreter::visit::{walk_expr, Visitor};

pub struct RuntimeSolver {
    pub t: Vec<f32>,
    pub stack: Vec<f32>,
    pub environment: Environment,
    pub cur_t: f32,
}

impl RuntimeSolver {
    pub fn new(t: Vec<f32>) -> Self {
        Self {
            t,
            stack: vec![],
            environment: Environment::default(),
            cur_t: 0.,
        }
    }

    pub fn solve_all(&mut self, expr: &Expr) -> Vec<f32> {
        self.t
            .clone()
            .iter()
            .map(|t| {
                self.cur_t = *t;
                self.visit_expr(expr);
                self.stack.pop().unwrap()
            })
            .collect()
    }
}

impl<'ast> Visitor<'ast> for RuntimeSolver {
    fn visit_expr(&mut self, expr: &Expr<'ast>) {
        walk_expr(self, expr);
        match expr {
            Expr::Binary(_, op, _) => {
                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                let result = match op {
                    BinOp::Plus => lhs + rhs,
                    BinOp::Minus => lhs - rhs,
                    BinOp::Asterisk => lhs * rhs,
                    BinOp::Slash => lhs / rhs,
                };
                self.stack.push(result);
            }
            Expr::Unary(op, _) => {
                let operand = self.stack.pop().unwrap();
                match op {
                    UnOp::Neg => self.stack.push(-operand),
                    UnOp::Pos => self.stack.push(operand),
                }
            }
            Expr::Call(callee, _) => {
                let arg = self.stack.pop().unwrap();
                if let Expr::Ident(callee) = &**callee {
                    let name = callee.name;
                    if let Some(lookup) = self.environment.lookup.get(name) {
                        match lookup {
                            IdentTy::Func => match name {
                                "Sin" => self.stack.push(f32::sin(arg)),
                                "Cos" => self.stack.push(f32::cos(arg)),
                                "Tan" => self.stack.push(f32::tan(arg)),
                                "Exp" => self.stack.push(f32::exp2(arg)),
                                "Sqrt" => self.stack.push(f32::sqrt(arg)),
                                "Ln" => self.stack.push(f32::ln(arg)),
                                _ => panic!("Invalid func name during runtime."),
                            },
                            IdentTy::Var => {}
                            IdentTy::Const => {}
                        }
                    }
                }
            }
            Expr::Lit(lit) => {
                let Lit::Number(lit) = *lit;
                self.stack.push(lit);
            }
            Expr::Ident(ident) => {
                let name = ident.name;
                if let Some(lookup) = self.environment.lookup.get(name) {
                    match lookup {
                        IdentTy::Var => self.stack.push(self.cur_t),
                        IdentTy::Func => {}
                        IdentTy::Const => {}
                    }
                }
            }
            Expr::Grouping(_) => {}
        }
    }
}
