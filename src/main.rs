use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() {
    let data_parsed = Grammar::parse(Rule::field, "-292.33");
    println!("{:?}", data_parsed); 
}
