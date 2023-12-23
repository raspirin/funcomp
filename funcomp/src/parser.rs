use crate::ast::{BinOp, Expr, Ident, Lit, Stmt, UnOp};
use crate::p;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

macro_rules! single_expr_stmt {
    ($expr: ident) => {
        pub fn $expr(&'ast self, mut pairs: Pairs<'ast, Rule>) -> Stmt {
            let ident = self.expr(pairs.next().unwrap().into_inner());
            Stmt::$expr(p!(ident))
        }
    };
}

macro_rules! dual_expr_stmt {
    ($expr: ident) => {
        pub fn $expr(&'ast self, mut pairs: Pairs<'ast, Rule>) -> Stmt {
            let lhs = self.expr(pairs.next().unwrap().into_inner());
            let rhs = self.expr(pairs.next().unwrap().into_inner());
            Stmt::$expr(p!(lhs), p!(rhs))
        }
    };
}

macro_rules! dual_operand_expr {
    ($expr: ident, $lit: literal, $upstream: ident) => {
        pub fn $expr(&'ast self, mut pairs: Pairs<'ast, Rule>) -> Expr {
            let lit = ::std::string::String::from($lit);
            if let Some(expr) = pairs.next() {
                match expr.as_rule() {
                    Rule::$upstream => {
                        let mut expr = self.$upstream(expr.into_inner());
                        while let Some(_) = pairs.peek() {
                            let op = self.binop(pairs.next().unwrap());
                            let rhs = self.$upstream(pairs.next().unwrap().into_inner());
                            expr = Expr::binary(p!(expr), op, p!(rhs));
                        }
                        expr
                    }
                    _ => panic!("Invalid {} type.", lit),
                }
            } else {
                panic!("Invalid {}.", lit)
            }
        }
    };
}

#[derive(Parser)]
#[grammar = "expr.pest"]
pub struct SrcParser;

impl<'ast> SrcParser {
    pub fn binop(&self, op: Pair<Rule>) -> BinOp {
        match op.as_rule() {
            Rule::plus => BinOp::Plus,
            Rule::minus => BinOp::Minus,
            Rule::asterisk => BinOp::Asterisk,
            Rule::slash => BinOp::Slash,
            _ => panic!("Invalid bin op."),
        }
    }

    pub fn unop(&self, op: Pair<Rule>) -> UnOp {
        match op.as_rule() {
            Rule::plus => UnOp::Pos,
            Rule::minus => UnOp::Neg,
            _ => panic!("Invalid un op."),
        }
    }

    pub fn source(&'ast self, pairs: Pairs<'ast, Rule>) -> Vec<Stmt> {
        let mut ret = vec![];
        for statement in pairs {
            match statement.as_rule() {
                Rule::stmt => {
                    ret.push(self.stmt(statement.into_inner()));
                }
                Rule::EOI => {
                    ret.push(Stmt::eoi());
                }
                _ => panic!("Invalid statement type.")
            }
        }
        ret
    }

    pub fn stmt(&'ast self, mut pairs: Pairs<'ast, Rule>) -> Stmt {
        if let Some(statement) = pairs.next() {
            match statement.as_rule() {
                Rule::draw => self.draw(statement.into_inner()),
                Rule::rot => self.rot(statement.into_inner()),
                Rule::scale => self.scale(statement.into_inner()),
                Rule::origin => self.origin(statement.into_inner()),
                _ => panic!("Invalid statement type."),
            }
        } else {
            panic!("Invalid statement.")
        }
    }

    single_expr_stmt! {rot}
    dual_expr_stmt! {scale}
    dual_expr_stmt! {origin}

    pub fn draw(&'ast self, mut pairs: Pairs<'ast, Rule>) -> Stmt {
        let ident = self.ident(pairs.next().unwrap());
        let from = self.expr(pairs.next().unwrap().into_inner());
        let to = self.expr(pairs.next().unwrap().into_inner());
        let step = self.expr(pairs.next().unwrap().into_inner());
        let x = self.expr(pairs.next().unwrap().into_inner());
        let y = self.expr(pairs.next().unwrap().into_inner());
        Stmt::Draw(p!(ident), p!(from), p!(to), p!(step), p!(x), p!(y))
    }

    dual_operand_expr! {expr, "expr", factor}
    dual_operand_expr! {factor, "factor", unary}

    pub fn unary(&'ast self, mut pairs: Pairs<'ast, Rule>) -> Expr {
        if let Some(leftest) = pairs.peek() {
            match leftest.as_rule() {
                Rule::plus | Rule::minus => {
                    let op = self.unop(pairs.next().unwrap());
                    let rhs = self.unary(pairs.next().unwrap().into_inner());
                    Expr::unary(op, p!(rhs))
                }
                Rule::call => {
                    let mut call = pairs.next().unwrap().into_inner();
                    let callee = self.primary(call.next().unwrap().into_inner());
                    let args = if let Some(args) = call.next() {
                        self.arguments(args.into_inner())
                    } else {
                        vec![]
                    };
                    Expr::call(p!(callee), args)
                }
                Rule::primary => self.primary(pairs.next().unwrap().into_inner()),
                _ => panic!("Invalid unary type: {:?}.", leftest),
            }
        } else {
            panic!("Invalid unary.")
        }
    }

    pub fn primary(&'ast self, mut pairs: Pairs<'ast, Rule>) -> Expr {
        if let Some(primary) = pairs.peek() {
            match primary.as_rule() {
                Rule::number => {
                    let lit = Lit::Number(pairs.next().unwrap().as_str().parse().unwrap());
                    Expr::lit(lit)
                }
                Rule::grouping => self.grouping(pairs.next().unwrap().into_inner()),
                Rule::ident => self.ident(pairs.next().unwrap()),
                _ => panic!("Invalid primary type."),
            }
        } else {
            panic!("Invalid primary.")
        }
    }

    pub fn ident(&'ast self, pair: Pair<'ast, Rule>) -> Expr {
        let ident = Ident {
            name: pair.as_str(),
        };
        Expr::ident(ident)
    }

    pub fn arguments(&'ast self, pairs: Pairs<'ast, Rule>) -> Vec<Expr> {
        pairs.map(|pair| self.expr(pair.into_inner())).collect()
    }

    pub fn grouping(&'ast self, mut pairs: Pairs<'ast, Rule>) -> Expr {
        if let Some(inner) = pairs.next() {
            self.expr(inner.into_inner())
        } else {
            panic!("Invalid grouping.")
        }
    }
}
