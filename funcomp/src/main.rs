use funcomp::parser::{Rule, SrcParser};
use pest::Parser;

fn main() {
    let mut pairs = SrcParser::parse(Rule::stmt, "rot is 1 + Sin(1);").unwrap();
    let expr = SrcParser.stmt(pairs.next().unwrap().into_inner());
    println!("{:?}", expr)
}
