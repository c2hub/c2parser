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


#[test]
fn parse_pexp()
{
	// ident
	C2Parser::parse_str(Rule::pexp, "potato").unwrap();

	// number - int
	C2Parser::parse_str(Rule::pexp, "55546").unwrap();

	// number - float
	C2Parser::parse_str(Rule::pexp, "5.546").unwrap();

	// stringlit
	C2Parser::parse_str(Rule::pexp, "\"potato\"").unwrap();

	// charlit
	C2Parser::parse_str(Rule::pexp, "'p'").unwrap();

	// exp in parens
	C2Parser::parse_str(Rule::pexp, "(potato)").unwrap();
}

#[test]
fn parse_pfexp()
{
	// pfexp - params
	C2Parser::parse_str(Rule::pfexp, "potato()").unwrap();

	// pfexp - square
	C2Parser::parse_str(Rule::pfexp, "potato[]").unwrap();

	// pfexp - bitf
	C2Parser::parse_str(Rule::pfexp, "potato[1:2]").unwrap();

	// pfexp - dot op
	C2Parser::parse_str(Rule::pfexp, "potato.other_symbol").unwrap();

	// pfexp - incr
	C2Parser::parse_str(Rule::pfexp, "potato++").unwrap();

	// pfexp - decr
	C2Parser::parse_str(Rule::pfexp, "potato--").unwrap();
}

#[test]
fn parse_uexp()
{
	// uexp - prefix
	C2Parser::parse_str(Rule::uexp, "++potato").unwrap();

	// uexp - postfix
	C2Parser::parse_str(Rule::uexp, "--potato").unwrap();

	// uexp - unop - b_com
	C2Parser::parse_str(Rule::uexp, "~potato").unwrap();

	// uexp - unop - addr_op
	C2Parser::parse_str(Rule::uexp, "&potato").unwrap();

	// uexp - unop - deref_op
	C2Parser::parse_str(Rule::uexp, "*potato").unwrap();

	// uexp - unop - un_plus_op
	C2Parser::parse_str(Rule::uexp, "+potato").unwrap();

	// uexp - unop - un_minus_op
	C2Parser::parse_str(Rule::uexp, "-potato").unwrap();

	// uexp - unop - b_negation_op
	C2Parser::parse_str(Rule::uexp, "!potato").unwrap();

	// uexp - sizeof
	C2Parser::parse_str(Rule::uexp, "sizeof(potato)").unwrap();
	C2Parser::parse_str(Rule::uexp, "sizeof potato").unwrap();

	// uexp - elemsof
	C2Parser::parse_str(Rule::uexp, "elemsof(potato)").unwrap();
	C2Parser::parse_str(Rule::uexp, "elemsof potato").unwrap();
}

#[test]
fn parse_cast()
{
	// test propagation
	C2Parser::parse_str(Rule::cast, "potato").unwrap();

	// w/ spaces
	C2Parser::parse_str(Rule::cast, "( -> potato ) lol").unwrap();

	// w/out spaces
	C2Parser::parse_str(Rule::cast, "(->potato)lol").unwrap();
}

#[test]
fn parse_mexp()
{
	// multiply
	C2Parser::parse_str(Rule::mexp, "5 * 5").unwrap();

	// divide
	C2Parser::parse_str(Rule::mexp, "4 / 4").unwrap();

	// modulo
	C2Parser::parse_str(Rule::mexp, "5 % 3").unwrap();
}

#[test]
fn parse_aexp()
{
	// add
	C2Parser::parse_str(Rule::aexp, "5 + 6").unwrap();

	// subtract
	C2Parser::parse_str(Rule::aexp, "6 - 5").unwrap();
}


#[test]
fn parse_sexp()
{
	// left shift
	C2Parser::parse_str(Rule::sexp, "5 << 1").unwrap();

	// right shft
	C2Parser::parse_str(Rule::sexp, "5 >> 1").unwrap();
}


#[test]
fn parse_rexp()
{
	// lt_eq_op
	C2Parser::parse_str(Rule::rexp, "5 <= 6").unwrap();

	// gt_eq_op
	C2Parser::parse_str(Rule::rexp, "5 >= 6").unwrap();

	// lt_op
	C2Parser::parse_str(Rule::rexp, "5 < 6").unwrap();

	// gt_op
	C2Parser::parse_str(Rule::rexp, "5 > 6").unwrap();
}

#[test]
fn parse_eexp()
{
	// eq_op
	C2Parser::parse_str(Rule::eexp, "potato == potato").unwrap();

	// ne_op
	C2Parser::parse_str(Rule::eexp, "potato != potato").unwrap();
}

#[test]
fn parse_bexp()
{
	// b_or
	C2Parser::parse_str(Rule::bexp, "potato | potato").unwrap();

	// b_xor
	C2Parser::parse_str(Rule::bexp, "potato ^ potato").unwrap();

	// b_and
	C2Parser::parse_str(Rule::bexp, "potato & potato").unwrap();
}

#[test]
fn parse_lexp()
{
	// and
	C2Parser::parse_str(Rule::lexp, "potato && potato").unwrap();

	// or
	C2Parser::parse_str(Rule::lexp, "potato || potato").unwrap();
}

#[test]
fn parse_elexp()
{
	C2Parser::parse_str(Rule::elexp, "potato ? potato : potato").unwrap();
}

#[test]
fn parse_asexp()
{
	// plus_assign_op
	C2Parser::parse_str(Rule::asexp, "potato += 5").unwrap();

	// minus_assign_op
	C2Parser::parse_str(Rule::asexp, "potato -= 5").unwrap();

	// multi_assign_op
	C2Parser::parse_str(Rule::asexp, "potato *= 5").unwrap();

	// divid_assign_op
	C2Parser::parse_str(Rule::asexp, "potato /= 5").unwrap();

	// mod_assign_op
	C2Parser::parse_str(Rule::asexp, "potato %= 5").unwrap();

	// lshft_assign_op
	C2Parser::parse_str(Rule::asexp, "potato <<= 5").unwrap();

	// rshft_assign_op
	C2Parser::parse_str(Rule::asexp, "potato >>= 5").unwrap();

	// b_or_assign_op
	C2Parser::parse_str(Rule::asexp, "potato |= 5").unwrap();

	// b_and_assign_op
	C2Parser::parse_str(Rule::asexp, "potato &= 5").unwrap();

	// b_xor_assign_op
	C2Parser::parse_str(Rule::asexp, "potato ^= 5").unwrap();

	// assign_op
	C2Parser::parse_str(Rule::asexp, "potato = potato").unwrap();
}

#[test]
fn parse_exp() // comma operator
{
	C2Parser::parse_str(Rule::asexp, "potato, potato, potato").unwrap();
}

#[test]
fn parse_params()
{
	// one
	C2Parser::parse_str(Rule::exp, "(potato)").unwrap();

	// many
	C2Parser::parse_str(Rule::exp, "(potato, potato, potato)").unwrap();
}

#[test]
fn parse_complex()
{
	C2Parser::parse_str(Rule::exp,"
			(
				1 + 2, 
				3 * 4,
				a = b,
				c == d,
				(5 < 4 && 4 > 5) && (4 <= 5 || 5 >= 4 )
			)
		").unwrap();
}
