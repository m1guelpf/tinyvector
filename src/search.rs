use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub enum EqualityCompOp {
	#[serde(rename = "eq")]
	Eq,
	#[serde(rename = "ne")]
	Ne,
	#[serde(rename = "gt")]
	Gt,
	#[serde(rename = "gte")]
	Gte,
	#[serde(rename = "lt")]
	Lt,
	#[serde(rename = "lte")]
	Lte,
}

fn eq(lhs: String, rhs: String) -> bool { lhs == rhs }
fn ne(lhs: String, rhs: String) -> bool { lhs != rhs }
fn gt(lhs: String, rhs: String) -> bool { lhs > rhs }
fn gte(lhs: String, rhs: String) -> bool { lhs >= rhs }
fn lt(lhs: String, rhs: String) -> bool { lhs < rhs }
fn lte(lhs: String, rhs: String) -> bool { lhs <= rhs }

fn get_equality_comp_op_fn(op: EqualityCompOp) -> impl Fn(String, String) -> bool {
	match op {
		EqualityCompOp::Eq => eq,
		EqualityCompOp::Ne => ne,
		EqualityCompOp::Gt => gt,
		EqualityCompOp::Gte => gte,
		EqualityCompOp::Lt => lt,
		EqualityCompOp::Lte => lte,
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]

pub struct Comparator {
	pub op: EqualityCompOp,
	pub val: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]

pub enum LogicalCompOp {
	#[serde(rename = "and")]
	And,
	#[serde(rename = "or")]
	Or,
}

fn and(lhs: bool, rhs: bool) -> bool { lhs && rhs }
fn or(lhs: bool, rhs: bool) -> bool { lhs || rhs }

fn get_logical_comp_op_fn(op: LogicalCompOp) -> impl Fn(bool, bool) -> bool {
	match op {
		LogicalCompOp::And => and,
		LogicalCompOp::Or => or,
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Logic {
	pub lhs: Comparator,
	pub op: LogicalCompOp,
	pub rhs: Comparator,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub enum FilterOp {
	Comparator,
	Logic,
}

pub struct Filter {
	pub val: FilterType,
}

/***

{
	"filter": {
		"$and": [
			...opt1,
			...opt2,
		],
		"$or": [
			...opt1,
			...opt2,
		],
		"$eq": {
			"field": "value"
		},
		"$ne": {
			"field": "value"
		},
		"$gt": {
			"field": "value"
		},
		"$gte": {
			"field": "value"
		},
		"$lt": {
			"field": "value"
		},
		"$lte": {
			"field": "value"
		},
	}
}

 ***/

 #[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Filter incorrectly formatted")]
	InvalidFilter,
}

pub fn parse(input: HashMap<String, String>) -> Result<Filter, Error> {
	if input.keys().len() != 1 {
		return Err(Error::InvalidFilter);
	}

	match input.keys().next().unwrap().as_str() {
		"$and" => {
			let lhs = parse(input.get("lhs"));
			let rhs = parse(input.get("rhs"));
			Ok() // Logic { lhs, op: LogicalCompOp::And, rhs }
		},
		"$or" => {
			let lhs = parse(input.get("lhs"));
			let rhs = parse(input.get("rhs"));
			let op = FilterOp::Logic(Logic { lhs, op: LogicalCompOp::Or, rhs });
			Filter { op }
		},
		"$eq" => {
			let op = get_equality_comp_op_fn(EqualityCompOp::Eq);
			let val = input.get("val").unwrap().to_string();
			Filter::Comparator(Comparator { op: EqualityCompOp::Eq, val })
		},
		"$ne" => {
			let op = get_equality_comp_op_fn(EqualityCompOp::Ne);
			let val = input.get("val").unwrap().to_string();
			Filter::Comparator(Comparator { op: EqualityCompOp::Ne, val })
		},
		"$gt" => {
			let op = get_equality_comp_op_fn(EqualityCompOp::Gt);
			let val = input.get("val").unwrap().to_string();
			Filter::Comparator(Comparator { op: EqualityCompOp::Gt, val })
		},
		"$gte" => {
			let op = get_equality_comp_op_fn(EqualityCompOp::Gte);
			let val = input.get("val").unwrap().to_string();
			Filter::Comparator(Comparator { op: EqualityCompOp::Gte, val })
		},
		"$lt" => {
			let op = get_equality_comp_op_fn(EqualityCompOp::Lt);
			let val = input.get("val").unwrap().to_string();
			Filter::Comparator(Comparator { op: EqualityCompOp::Lt, val })
		},
		"$lte" => {
			let op = get_equality_comp_op_fn(EqualityCompOp::Lte);
			let val = input.get("val").unwrap().to_string();
			Filter::Comparator(Comparator { op: EqualityCompOp::Lte, val })
		},
		_ => Err(Error::InvalidFilter),
	}
}
