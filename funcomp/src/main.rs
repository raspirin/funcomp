use pest::Parser;
use funcomp::parser::{SrcParser, Rule};

fn main() {
    let mut pairs = SrcParser::parse(Rule::stmt, "rot is Sin(1);").unwrap();
    let expr = SrcParser.stmt(pairs.next().unwrap().into_inner());
    println!("{:?}", expr)
}
