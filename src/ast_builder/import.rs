use parser::Rule;
use ast_builder::{
	BuilderError,
	Pair,
	BuildResult
};

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Import {
	pub name: String,
	pub alias: Option<String>,
	local: bool,
}

pub fn parse_import(input: Pair) -> BuildResult<Import>
{
	if let Rule::import = input.as_rule() { /*Ok*/ } else
	{
		return Err(BuilderError::UnexpectedRule(input.as_rule(), Rule::module));
	}

	let mut import = Import::default();

	let mut got_name: bool = false;
	for pair in input.into_inner()
	{
		match pair.as_rule()
		{
			Rule::ident => if !got_name
			{
				import.name = pair.clone().into_span().as_str().to_string();
				got_name = true
			} else { import.alias = Some(pair.clone().into_span().as_str().to_string()); },
			Rule::local => { import.local = true; },
			_ => return Err(BuilderError::UnexpectedPair)
		}
	};

	if import == Import::default()
	{
		return Err(BuilderError::InvalidPair);
	}
	Ok(import)
}
