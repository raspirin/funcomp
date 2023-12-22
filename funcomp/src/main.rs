use funcomp::parser::{Rule, SrcParser};
use pest::Parser;
use funcomp::interpreter::typeck::StaticChecker;
use funcomp::interpreter::visit::Visitor;

fn main() {
    let source = r#"rot is 2;
// comment
origin is PI;
for T from 1 to Sin(1) step 1 + Sin(1) draw (Sin(1), T);
"#;
    let mut typeck = StaticChecker::default();
    let pairs = SrcParser::parse(Rule::source, source).unwrap();
    let source = SrcParser.source(pairs);
    for stmt in source {
        typeck.visit_stmt(&stmt);
    }
}
