use crate::ast::{BinOp, Expr, Lit, Stmt, UnOp};
use crate::interpreter::environment::{Environment, IdentTy};
use crate::interpreter::visit::{walk_expr, walk_stmt, Visitor};
use crate::p;
use crate::parser::Rule::source;
use std::f32::consts::PI;
use std::ops::Neg;

pub mod environment;
pub mod typeck;
pub mod visit;

#[derive(Clone, Default)]
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

#[derive(Default)]
pub struct Interpreter<'ast> {
    pub environment: Environment,
    pub state: State,
    pub cal_stack: Vec<Expr<'ast>>,
    pub statements: Vec<Stmt<'ast>>,
}

impl<'ast> Interpreter<'ast> {
    pub fn resolve(&'ast mut self, src: &[Stmt<'ast>]) {
        for stmt in src.iter() {
            self.visit_stmt(stmt);
        }
        for i in self.statements.iter() {
            println!("{:?}", i)
        }
    }
}

impl<'ast> Visitor<'ast> for Interpreter<'ast> {
    fn visit_expr(&mut self, expr: &Expr<'ast>) {
        walk_expr(self, expr);
        match expr {
            Expr::Binary(_, op, _) => {
                let rhs = self.cal_stack.pop().unwrap();
                let lhs = self.cal_stack.pop().unwrap();
                match (lhs, rhs) {
                    (Expr::Lit(Lit::Number(lhs)), Expr::Lit(Lit::Number(rhs))) => {
                        let operation = match op {
                            BinOp::Plus => |l, r| l + r,
                            BinOp::Minus => |l, r| l - r,
                            BinOp::Asterisk => |l, r| l * r,
                            BinOp::Slash => |l, r| l / r,
                        };
                        let result = operation(lhs, rhs);
                        self.cal_stack.push(Expr::Lit(Lit::Number(result)))
                    }
                    (lhs, rhs) => {
                        self.cal_stack
                            .push(Expr::binary(p!(lhs), op.clone(), p!(rhs)));
                    }
                }
            }
            Expr::Unary(op, _) => {
                let operand = self.cal_stack.pop().unwrap();
                match operand {
                    Expr::Lit(Lit::Number(lit)) => match op {
                        UnOp::Neg => self.cal_stack.push(Expr::lit(Lit::Number(lit.neg()))),
                        UnOp::Pos => self.cal_stack.push(Expr::lit(Lit::Number(lit))),
                    },
                    _ => self.cal_stack.push(Expr::unary(op.clone(), p!(operand))),
                }
            }
            Expr::Call(_, _) => {
                let arg = self.cal_stack.pop().unwrap();
                let callee = self.cal_stack.pop().unwrap();
                match arg {
                    Expr::Lit(lit) => {
                        let Lit::Number(lit) = lit;
                        if let Expr::Ident(callee) = callee {
                            let name = callee.name;
                            let result = match name {
                                "Sin" => f32::sin(lit),
                                "Cos" => f32::cos(lit),
                                "Tan" => f32::tan(lit),
                                "Sqrt" => f32::sqrt(lit),
                                "Exp" => f32::exp2(lit),
                                "Ln" => f32::ln(lit),
                                _ => panic!("Invalid internal func."),
                            };
                            self.cal_stack.push(Expr::lit(Lit::Number(result)))
                        } else {
                            panic!("Can't reach")
                        }
                    }
                    _ => self.cal_stack.push(Expr::call(p!(callee), vec![arg])),
                }
            }
            Expr::Grouping(_) => {
                let inner = self.cal_stack.pop().unwrap();
                self.cal_stack.push(inner);
            }
            Expr::Lit(lit) => self.cal_stack.push(Expr::lit(lit.clone())),
            Expr::Ident(ident) => {
                let name = ident.name;
                let ty = self.environment.lookup.get(name).unwrap();
                match ty {
                    IdentTy::Const => self.cal_stack.push(Expr::lit(Lit::Number(PI))),
                    _ => self.cal_stack.push(Expr::ident(ident.clone())),
                }
            }
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt<'ast>) {
        walk_stmt(self, stmt);
        match stmt {
            Stmt::EOI => {
                self.statements.push(Stmt::eoi());
            }
            Stmt::Rot(_) => {
                let inner = self.cal_stack.pop().unwrap();
                self.statements.push(Stmt::rot(p!(inner)));
            }
            Stmt::Scale(_) => {
                let inner = self.cal_stack.pop().unwrap();
                self.statements.push(Stmt::scale(p!(inner)));
            }
            Stmt::Origin(_) => {
                let inner = self.cal_stack.pop().unwrap();
                self.statements.push(Stmt::origin(p!(inner)));
            }
            Stmt::Draw(_, _, _, _, _, _) => {
                let y = self.cal_stack.pop().unwrap();
                let x = self.cal_stack.pop().unwrap();
                let step = self.cal_stack.pop().unwrap();
                let to = self.cal_stack.pop().unwrap();
                let from = self.cal_stack.pop().unwrap();
                let ident = self.cal_stack.pop().unwrap();
                self.statements.push(Stmt::draw(
                    p!(ident),
                    p!(from),
                    p!(to),
                    p!(step),
                    p!(x),
                    p!(y),
                ));
            }
        }
    }
}
