use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

trait Compare {
	fn compare(&self, metadata: &HashMap<String, String>) -> bool;
}

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
	pub metadata_field: String,
	pub op: EqualityCompOp,
	pub comp_value: String,
}

impl Compare for Comparator {
	fn compare(&self, metadata: &HashMap<String, String>) -> bool {
		let metadata_value = metadata.get(&self.metadata_field).unwrap_or(&"".to_string());
		let op = get_equality_comp_op_fn(self.op);
		op(*metadata_value, self.comp_value)
	}
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
	pub lhs: Box<Filter>,
	pub op: LogicalCompOp,
	pub rhs: Box<Filter>,
}

impl Compare for Logic {
	fn compare(&self, metadata: &HashMap<String, String>) -> bool {
		let lhs = self.lhs.compare(metadata);
		let rhs = self.rhs.compare(metadata);
		let op = get_logical_comp_op_fn(self.op);
		op(lhs, rhs)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub enum Filter {
	Comparator(Comparator),
	Logic(Logic),
}

impl Compare for Filter {
	fn compare(&self, metadata: &HashMap<String, String>) -> bool {
		match self {
			Filter::Comparator(c) => c.compare(metadata),
			Filter::Logic(l) => l.compare(metadata),
		}
	}
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

fn parse_logic_helper(input: HashMap<String, String>, key: &str) -> Result<Filter, Error> {
	match input.get(key) {
		Some(s) => { 
			match parse(s) {
				Ok(f) => { Ok(f) },
				Err(_) => { Err(Error::InvalidFilter) }
			}
		},
		None => { Err(Error::InvalidFilter) },
	}
}



fn parse_logic(input: HashMap<String, String>, op: LogicalCompOp) -> Result<Filter, Error> {
	let lhs = parse_logic_helper(input, "lhs");
	let rhs = parse_logic_helper(input, "rhs");
	Ok(Filter::Logic(Logic { lhs, op, rhs }))
}

fn parse_comparator(input: HashMap<String, String>, op: EqualityCompOp) -> Result<Filter, Error> {
	fn parse_field(input: HashMap<String, String>, key: &str) -> Result<String, Error> {
		match input.get(key) {
			Some(s) => Ok(s.to_string()),
			None => return Err(Error::InvalidFilter),
		}
	}

	let metadata_field = match input.keys {
		Some(s) => Ok(s.to_string()),
		None => return Err(Error::InvalidFilter),
	};
	Ok(Filter::Comparator(Comparator { metadata_field, op: EqualityCompOp::Eq, comp_value }))
}

pub fn parse(input: HashMap<String, String>) -> Result<Filter, Error> {
	if input.keys().len() != 1 {
		return Err(Error::InvalidFilter);
	}

	match input.keys().next().unwrap().as_str() {
		"$and" => parse_logic(input.get("$and").unwrap().to_string(), LogicalCompOp::And),
		"$or" => parse_logic(input.get("$or").unwrap().to_string(), LogicalCompOp::Or),
		"$eq" => parse_comparator(input.get("$eq").unwrap().to_string(), EqualityCompOp::Eq),
		"$ne" => parse_comparator(input.get("$ne").unwrap().to_string(), EqualityCompOp::Ne),
		"$gt" => parse_comparator(input.get("$gt").unwrap().to_string(), EqualityCompOp::Gt),
		"$gte" => parse_comparator(input.get("$gte").unwrap().to_string(), EqualityCompOp::Gte),
		"$lt" => parse_comparator(input.get("$lt").unwrap().to_string(), EqualityCompOp::Lt),
		"$lte" => parse_comparator(input.get("$lte").unwrap().to_string(), EqualityCompOp::Lte),
		_ => Err(Error::InvalidFilter),
	}
}
