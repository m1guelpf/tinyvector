use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub enum Distance {
	#[serde(rename = "euclidean")]
	Euclidean,
	#[serde(rename = "cosine")]
	Cosine,
	#[serde(rename = "dot")]
	DotProduct,
}

pub fn get_cache_attr(metric: Distance, vec: &[f32]) -> f32 {
	match metric {
		// Dot product doesn't allow any caching
		Distance::DotProduct | Distance::Euclidean => 0.0,
		// Precompute the magnitude of the vector
		Distance::Cosine => vec.iter().map(|&x| x.powi(2)).sum::<f32>().sqrt(),
	}
}

pub fn get_distance_fn(metric: Distance) -> impl Fn(&[f32], &[f32], f32) -> f32 {
	match metric {
		Distance::Euclidean => euclidian_distance,
		// We use dot product for cosine because we've normalized the vectors on insertion
		Distance::Cosine | Distance::DotProduct => dot_product,
	}
}

fn euclidian_distance(a: &[f32], b: &[f32], a_sum_squares: f32) -> f32 {
	let mut cross_terms = 0.0;
	let mut b_sum_squares = 0.0;

	for (i, j) in a.iter().zip(b) {
		cross_terms += i * j;
		b_sum_squares += j.powi(2);
	}

	2.0f32
		.mul_add(-cross_terms, a_sum_squares + b_sum_squares)
		.max(0.0)
		.sqrt()
}

fn dot_product(a: &[f32], b: &[f32], _: f32) -> f32 {
	a.iter().zip(b).fold(0.0, |acc, (x, y)| acc + x * y)
}

pub fn normalize(vec: &[f32]) -> Vec<f32> {
	let magnitude = (vec.iter().fold(0.0, |acc, &val| val.mul_add(val, acc))).sqrt();

	if magnitude > std::f32::EPSILON {
		vec.iter().map(|&val| val / magnitude).collect()
	} else {
		vec.to_vec()
	}
}

pub struct ScoreIndex {
	pub score: f32,
	pub index: usize,
}

impl PartialEq for ScoreIndex {
	fn eq(&self, other: &Self) -> bool {
		self.score.eq(&other.score)
	}
}

impl Eq for ScoreIndex {}

impl PartialOrd for ScoreIndex {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		// The comparison is intentionally reversed here to make the heap a min-heap
		other.score.partial_cmp(&self.score)
	}
}

impl Ord for ScoreIndex {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap_or(Ordering::Equal)
	}
}
