use funcomp::ast::{BinOp, Expr, Ident, Lit, UnOp};
use funcomp::p;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "expr.pest"]
struct ExprParser;

impl<'ast> ExprParser {
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

    pub fn expr(&'ast self, mut pairs: Pairs<'ast, Rule>) -> Expr {
        let mut expr = self.factor(pairs.next().unwrap().into_inner());
        while let Some(_) = pairs.peek() {
            let op = self.binop(pairs.next().unwrap());
            let rhs = self.factor(pairs.next().unwrap().into_inner());
            expr = Expr::Binary(p!(expr), op, p!(rhs))
        }
        expr
    }

    pub fn factor(&'ast self, mut pairs: Pairs<'ast, Rule>) -> Expr {
        let mut expr = self.unary(pairs.next().unwrap().into_inner());
        while let Some(_) = pairs.peek() {
            let op = self.binop(pairs.next().unwrap());
            let rhs = self.unary(pairs.next().unwrap().into_inner());
            expr = Expr::Binary(p!(expr), op, p!(rhs));
        }
        expr
    }

    pub fn unary(&'ast self, mut pairs: Pairs<'ast, Rule>) -> Expr {
        if let Some(leftest) = pairs.peek() {
            match leftest.as_rule() {
                Rule::plus | Rule::minus => {
                    let op = self.unop(pairs.next().unwrap());
                    let rhs = self.unary(pairs.next().unwrap().into_inner());
                    Expr::unary(op, Box::new(rhs))
                }
                Rule::call => {
                    let mut call = pairs.next().unwrap().into_inner();
                    let callee = self.primary(call.next().unwrap().into_inner());
                    let args = self.arguments(call.next().unwrap().into_inner());
                    Expr::call(Box::new(callee), args)
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
                    Expr::Lit(lit)
                }
                Rule::grouping => self.grouping(pairs.next().unwrap().into_inner()),
                Rule::ident => {
                    let ident = pairs.next().unwrap();
                    let ident = Ident {
                        name: ident.as_str(),
                    };
                    Expr::Ident(ident)
                }
                _ => panic!("Invalid primary type."),
            }
        } else {
            panic!("Invalid primary.")
        }
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

fn main() {
    let mut pairs = ExprParser::parse(Rule::expr, "1 + Sin(a, s, d, f)").unwrap();
    let expr = ExprParser.expr(pairs.next().unwrap().into_inner());
    println!("{:?}", expr)
}
