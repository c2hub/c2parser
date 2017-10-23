extern crate pest;
#[macro_use] extern crate pest_derive;

pub mod parser;
pub mod ast_builder;

#[cfg(test)]
mod tests;

pub use parser::C2Parser;
