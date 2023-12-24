use std::{env, io};
use std::fs::File;
use funcomp::interpreter::static_checker::StaticChecker;
use funcomp::interpreter::Interpreter;
use funcomp::parser::{Rule, SrcParser};
use pest::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let file = File::open(path).unwrap();
    let source = io::read_to_string(file).unwrap();
    let mut static_checker = StaticChecker::default();
    let interpreter = Interpreter::default();
    let pairs = SrcParser::parse(Rule::source, &source).unwrap();
    let source = SrcParser.source(pairs);
    static_checker.check(&source);
    interpreter.accept(&source).interpret();
}
