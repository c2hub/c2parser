use std::error::Error;
use std::fmt;

use pest::iterators;
use pest::inputs::StrInput;

use parser::Rule;

// modules and reexports
pub mod module_decl;
pub use self::module_decl::{
	parse_module_decl,
	ModuleDecl,
};

pub mod import;
pub use self::import::{
	parse_import,
	Import,
};

pub mod head;
pub use self::head::{
	parse_head,
	Head,
};

// BuilderError
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuilderError {
	UnexpectedRule(Rule, Rule),
	ExpectedPair,
	InvalidPair,
	UnexpectedPair,
}

impl Error for BuilderError {
	fn description(&self) -> &str {
		match *self {
			BuilderError::UnexpectedRule(..) => "pair is a different rule than expected",
			BuilderError::ExpectedPair => "pair was expected but none received",
			BuilderError::InvalidPair => "pair contains invalid data",
			BuilderError::UnexpectedPair => "unexpected pair type"
		}
	}
}

impl fmt::Display for BuilderError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			BuilderError::UnexpectedRule(got, expected) => {
				write!(f, "Pair has a different rule than expected. Got: {:?}, Expected: {:?}", got, expected)
			},
			BuilderError::ExpectedPair |
			BuilderError::UnexpectedPair |
			BuilderError::InvalidPair => write!(f, "{}", self.description()),
		}
	}
}

// Useful aliases
pub type Pair<'a> = iterators::Pair<Rule, StrInput<'a>>;
pub type BuildResult<T> = Result<T, BuilderError>;
