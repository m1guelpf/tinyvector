use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]

pub enum Operation {
	#[serde(rename = "and")]
	And,
	#[serde(rename = "or")]
	Or,
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

enum OpSide {
	String(String),
	Comparator(&Comparator),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct Comparator {
	pub lhs: OpSide,
	pub op: Operation,
	pub rhs: OpSide,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Filter incorrectly formatted")]
	InvalidFilter,
}

pub fn parse(input: Option<HashMap<String, String>>) -> Option<Comparator> {
	match input {
		Some(input) => match input {
			"$and" => {},
			"$or" => {},
			"$eq" => {},
			"$ne" => {},
			"$gt" => {},
			"$gte" => {},
			"$lt" => {},
			"$lte" => {},
			_ => {},
		},
		None => None,
	}
}

fn get_operation_fn(op: Operation) -> impl Fn(String, String) -> bool {
	match op {
		Operation::And => and,
		Operation::Or => or,
		Operation::Eq => eq,
		Operation::Ne => ne,
		Operation::Gt => gt,
		Operation::Gte => gte,
		Operation::Lt => lt,
		Operation::Lte => lte,
	}
}

fn and(lhs: bool, rhs: bool) -> bool { lhs && rhs }
fn or(lhs: bool, rhs: bool) -> bool { lhs || rhs }
fn eq(lhs: String, rhs: String) -> bool { lhs == rhs }
fn ne(lhs: String, rhs: String) -> bool { lhs != rhs }
fn gt(lhs: String, rhs: String) -> bool { lhs > rhs }
fn gte(lhs: String, rhs: String) -> bool { lhs >= rhs }
fn lt(lhs: String, rhs: String) -> bool { lhs < rhs }
fn lte(lhs: String, rhs: String) -> bool { lhs <= rhs }
