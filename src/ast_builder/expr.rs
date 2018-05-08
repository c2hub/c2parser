use parser::Rule;
use ast_builder::{
	BuilderError,
	Pair,
	BuildResult
};

pub trait Expression {
	fn get_any(&self) -> Box<Any>;
	fn eat_any(self) -> Box<Any>;
	fn get_type(&self) -> ExpressionType;
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum ExpressionType {
	// compatibility layer, lol
	Identifier,
	Number,
	StringLit,
	CharLit,

	// general expression
	PrimaryExpression,
	PrefixExpression,
	FunctionCall,
	Cast,
	UnaryExpression,
	MultiplicativeExpression, // also modulo and divide
	AdditionExpression, // also subtraction
	ShiftExpression,
	RelationExpression,
	EquivalenceExpression,
	BitWiseExpression,
	BooleanExpression,
	ConditionalExpression,
	AssignmentExpression,
	ConstantExpression,
	Expression, // top-level
}

// primary expression
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum PrimaryExpression {
	pub name: String,
	pub alias: Option<String>,
	local: bool,
}



pub fn parse_import(input: Pair) -> BuildResult<Expression>
{
}
