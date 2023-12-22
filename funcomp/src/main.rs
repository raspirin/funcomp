use funcomp::parser::{Rule, SrcParser};
use pest::Parser;

fn main() {
    let pairs = SrcParser::parse(Rule::source, "rot is 1 + Sin(1);").unwrap();
    let source = SrcParser.ast(pairs);
    println!("{:?}", source)
}
