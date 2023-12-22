use funcomp::parser::{Rule, SrcParser};
use pest::Parser;

fn main() {
    let source = r#"rot is 2;
// comment
origin is T + Sin(T,               );
"#;
    let pairs = SrcParser::parse(Rule::source, source).unwrap();
    let source = SrcParser.source(pairs);
    println!("{:?}", source)
}
