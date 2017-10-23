use parser::{C2Parser, Rule};
use pest::Parser;

// parsing tests?
#[test]
fn parse_module()
{
	C2Parser::parse_str(Rule::c2,
		"
		module c2_test;

		import stdio;
		import stdlib local;
		import assert as ass;
		import sergei as potato local;
		"
	).unwrap();
}

// builder tests
#[test]
fn module_decl()
{
	use ast_builder::{parse_module_decl, ModuleDecl};

	assert_eq!(
		parse_module_decl(
			C2Parser::parse_str(Rule::module, "module test;")
				.unwrap()
				.next()
				.unwrap()
		).unwrap(),
		ModuleDecl { name: "test".to_string() }
	)
}

#[test]
fn import()
{
	use ast_builder::parse_import;

	parse_import(
		C2Parser::parse_str(Rule::import, "import stdio;")
			.unwrap()
			.next()
			.unwrap()
	).unwrap();

	parse_import(
		C2Parser::parse_str(Rule::import, "import stdlib local;")
			.unwrap()
			.next()
			.unwrap()
	).unwrap();

	parse_import(
		C2Parser::parse_str(Rule::import, "import assert as ass;")
			.unwrap()
			.next()
			.unwrap()
	).unwrap();

	parse_import(
		C2Parser::parse_str(Rule::import, "import sergei as potato local;")
			.unwrap()
			.next()
			.unwrap()
	).unwrap();
}

#[test]
fn head()
{
	use ast_builder::parse_head;

	parse_head(
		C2Parser::parse_str(Rule::c2,
			"
			module c2_test;

			import stdio;
			import stdlib local;
			import assert as ass;
			import sergei as potato local;
			"
		).unwrap().next().unwrap()
	).unwrap();
}
