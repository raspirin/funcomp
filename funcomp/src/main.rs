use pest::Parser;
use funcomp::parser::{SrcParser, Rule};

fn main() {
    let mut pairs = SrcParser::parse(Rule::expr, "Sin()").unwrap();
    let expr = SrcParser.expr(pairs.next().unwrap().into_inner());
    println!("{:?}", expr)
}
