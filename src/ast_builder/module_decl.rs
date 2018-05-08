use parser::Rule;
use ast_builder::{
	BuilderError,
	Pair,
	BuildResult
};

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ModuleDecl {
	pub name: String
}

pub fn parse_module_decl(input: Pair) -> BuildResult<ModuleDecl> {
	if let Rule::module = input.as_rule() { /*Ok*/ } else {
		return Err(BuilderError::UnexpectedRule(input.as_rule(), Rule::module));
	}

	let name = match input.into_inner()
		.next()
	{
		Some(p) => if let Rule::ident = p.as_rule() {
			p.clone()
				.into_span()
				.as_str()
				.to_string()
		} else { return Err(BuilderError::UnexpectedRule(p.as_rule(), Rule::module)) },
		None => return Err(BuilderError::ExpectedPair)
	};

	Ok(ModuleDecl { name: name })
}
