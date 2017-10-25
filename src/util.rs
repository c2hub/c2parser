use ast_builder::Pair;
use pest::inputs::{Span, Input};
use ansi_term::Colour::{Purple, Green, Yellow, Red, Cyan};

pub fn get_linecol_string<T: Input>(span: &Span<T>) -> String
{
	let (line, col) = span.clone().split().0.line_col();
	String::from(
		format!("{}:{}", line, col)
	)
}

pub fn display_ast(p: &Pair, i: i32)
{
	let rule = p.as_rule();
	let span = p.clone().into_span(); 
	let children: Vec<_> = p.clone().into_inner().collect();

	(0..i).for_each(|_| print!("  "));
	
	print!("{}",
		if !children.is_empty()	{ Purple.paint(format!("{:?}", rule)) }
		else 					{ Red.paint(format!("{:?}", rule)) }
	);

	if !children.is_empty()
	{
		print!(" {}", Cyan.paint(">".to_string() + &children.len().to_string()));
	}
	else
	{
		print!(":{} '{}'",
			Green.paint(get_linecol_string(&span)),
			Yellow.paint(span.as_str())
		);
	}
	println!();

	for node in children
	{
		display_ast(&node, i+1);
	}
}
