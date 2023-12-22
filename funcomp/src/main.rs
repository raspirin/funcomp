use funcomp::parser::{Rule, SrcParser};
use pest::Parser;
use funcomp::interpreter::Interpreter;
use funcomp::interpreter::typeck::StaticChecker;

fn main() {
    let source = r#"rot is 2 + 1 * Sin(PI);
"#;
    let mut static_checker = StaticChecker::default();
    let mut interpreter = Interpreter::default();
    let pairs = SrcParser::parse(Rule::source, source).unwrap();
    let source = SrcParser.source(pairs);
    static_checker.check(&source);
    interpreter.resolve(&source)
}
