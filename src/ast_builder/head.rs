use parser::Rule;
use ast_builder::{
	BuilderError,
	Pair,
	BuildResult,
	Import,

	parse_import,
	parse_module_decl,
};

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Head {
	pub name: String,
	pub imports: Vec<Import>,
}

pub fn parse_head(input: Pair) -> BuildResult<Head>
{
	if let Rule::head = input.as_rule() { /*Ok*/ } else
	{
		return Err(BuilderError::UnexpectedRule(input.as_rule(), Rule::module));
	}

	let mut head = Head::default();

	for pair in input.into_inner()
	{
		match pair.as_rule()
		{
			Rule::module => { head.name = parse_module_decl(pair.clone())?.name },
			Rule::import => head.imports.push(parse_import(pair.clone())?),
			_ => return Err(BuilderError::UnexpectedPair)
		}
	};

	if head == Head::default()
	{
		return Err(BuilderError::InvalidPair);
	}
	Ok(head)
}
