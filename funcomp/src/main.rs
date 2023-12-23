use funcomp::interpreter::static_checker::StaticChecker;
use funcomp::interpreter::Interpreter;
use funcomp::parser::{Rule, SrcParser};
use pest::Parser;

fn main() {
    let source = r#"rot is 2 + 1 * Sin(PI);
origin is (Sin(Cos(PI)), 1);
scale is (Sin(1), Cos(1));
for T from 1 to 2 step 0.3 draw (Sin(T), Cos(T));
"#;
    let mut static_checker = StaticChecker::default();
    let interpreter = Interpreter::default();
    let pairs = SrcParser::parse(Rule::source, source).unwrap();
    let source = SrcParser.source(pairs);
    static_checker.check(&source);
    interpreter.accept(&source).interpret();
}
