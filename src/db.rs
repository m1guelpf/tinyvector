use anyhow::Context;
use axum::Extension;
use lazy_static::lazy_static;
use schemars::JsonSchema;
use std::{cmp::Ordering, collections::HashMap, fs, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::similarity::{get_distance_fn, Distance};

lazy_static! {
	pub static ref STORE_PATH: PathBuf = PathBuf::from("./storage/db");
}

#[allow(clippy::module_name_repetitions)]
pub type DbExtension = Extension<Arc<RwLock<Db>>>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Collection already exists")]
	UniqueViolation,

	#[error("Collection doesn't exist")]
	NotFound,

	#[error("The dimension of the vector doesn't match the dimension of the collection")]
	DimensionMismatch,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Db {
	pub collections: HashMap<String, Collection>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct SimilarityResult {
	score: f32,
	embedding: Embedding,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Collection {
	/// Dimension of the vectors in the collection
	pub dimension: usize,
	/// Distance metric used for querying
	pub distance: Distance,
	/// Embeddings in the collection
	#[serde(default)]
	pub embeddings: Vec<Embedding>,
}

impl Collection {
	pub fn get_similarity(&self, query: &[f32], k: usize) -> Vec<SimilarityResult> {
		let mut scores = Vec::with_capacity(self.embeddings.len());

		let distance_fn = get_distance_fn(&self.distance);
		for embedding in &self.embeddings {
			scores.push(distance_fn(&embedding.vector, query));
		}

		let mut partitioned_indices: Vec<usize> = (0..scores.len()).collect();
		partitioned_indices.sort_unstable_by(|&a, &b| {
			scores[b].partial_cmp(&scores[a]).unwrap_or(Ordering::Equal)
		});

		partitioned_indices
			.iter()
			.take(k)
			.map(|&i| SimilarityResult {
				score: scores[i],
				embedding: self.embeddings[i].clone(),
			})
			.collect()
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Embedding {
	pub id: String,
	pub vector: Vec<f32>,
	pub metadata: Option<HashMap<String, String>>,
}

impl Db {
	pub fn new() -> Self {
		Self {
			collections: HashMap::new(),
		}
	}

	pub fn extension(self) -> DbExtension {
		Extension(Arc::new(RwLock::new(self)))
	}

	pub fn create_collection(
		&mut self,
		name: String,
		dimension: usize,
		distance: Distance,
	) -> Result<Collection, Error> {
		if self.collections.contains_key(&name) {
			return Err(Error::UniqueViolation);
		}

		let collection = Collection {
			dimension,
			distance,
			embeddings: Vec::new(),
		};

		self.collections.insert(name, collection.clone());

		Ok(collection)
	}

	pub fn delete_collection(&mut self, name: &str) -> Result<(), Error> {
		if !self.collections.contains_key(name) {
			return Err(Error::NotFound);
		}

		self.collections.remove(name);

		Ok(())
	}

	pub fn insert_into_collection(
		&mut self,
		collection_name: &str,
		embedding: Embedding,
	) -> Result<(), Error> {
		let collection = self
			.collections
			.get_mut(collection_name)
			.ok_or(Error::NotFound)?;

		if collection.embeddings.iter().any(|e| e.id == embedding.id) {
			return Err(Error::UniqueViolation);
		}

		if embedding.vector.len() != collection.dimension {
			return Err(Error::DimensionMismatch);
		}

		collection.embeddings.push(embedding);

		Ok(())
	}

	pub fn get_collection(&self, name: &str) -> Option<&Collection> {
		self.collections.get(name)
	}

	fn load_from_store() -> anyhow::Result<Self> {
		if !STORE_PATH.exists() {
			tracing::debug!("Creating database store");
			fs::create_dir_all(STORE_PATH.parent().context("Invalid store path")?)?;

			return Ok(Self::new());
		}

		tracing::debug!("Loading database from store");
		let db = fs::read(STORE_PATH.as_path())?;
		Ok(bincode::deserialize(&db[..])?)
	}

	fn save_to_store(&self) -> anyhow::Result<()> {
		let db = bincode::serialize(self)?;

		fs::write(STORE_PATH.as_path(), db)?;

		Ok(())
	}
}

impl Drop for Db {
	fn drop(&mut self) {
		tracing::debug!("Saving database to store");
		self.save_to_store().ok();
	}
}

pub fn from_store() -> anyhow::Result<Db> {
	Db::load_from_store()
}
