// to track grammar changes
const _GRAMMAR: &'static str = include_str!("c2.pest");

#[derive(Parser)]
#[grammar = "c2.pest"]
pub struct C2Parser;
