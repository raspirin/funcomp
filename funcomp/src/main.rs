use funcomp::interpreter::static_checker::StaticChecker;
use funcomp::interpreter::Interpreter;
use funcomp::parser::{Rule, SrcParser};
use pest::Parser;

fn main() {
    let source = r#"
for T from 1 to 200 step 2 draw (T, T);

"#;
    let mut static_checker = StaticChecker::default();
    let interpreter = Interpreter::default();
    let pairs = SrcParser::parse(Rule::source, source).unwrap();
    let source = SrcParser.source(pairs);
    static_checker.check(&source);
    interpreter.accept(&source).interpret();
}
