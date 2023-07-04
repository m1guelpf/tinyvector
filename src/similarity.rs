use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
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
		Distance::DotProduct => 0.0,
		// Precompute the sum of squares of the vector
		Distance::Euclidean => vec.iter().map(|x| x.powi(2)).sum(),
		// Precompute the magnitude of the vector
		Distance::Cosine => vec.iter().map(|&x| x.powi(2)).sum::<f32>().sqrt(),
	}
}

pub fn get_distance_fn(metric: Distance) -> impl Fn(&[f32], &[f32], f32) -> f32 {
	match metric {
		Distance::DotProduct => dot_product,
		Distance::Cosine => cosine_similarity,
		Distance::Euclidean => euclidian_distance,
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

fn cosine_similarity(a: &[f32], b: &[f32], mag_a: f32) -> f32 {
	let mut dot_product = 0.0;
	let mut mag_b = 0.0;

	for (i, j) in a.iter().zip(b) {
		dot_product += i * j;
		mag_b += j.powi(2);
	}

	let mag_b = mag_b.sqrt();
	if mag_a == 0.0 || mag_b == 0.0 {
		0.0
	} else {
		dot_product / (mag_a * mag_b)
	}
}

fn dot_product(a: &[f32], b: &[f32], _: f32) -> f32 {
	a.iter().zip(b).fold(0.0, |acc, (x, y)| acc + x * y)
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
