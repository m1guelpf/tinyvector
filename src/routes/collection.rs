use aide::axum::{
	routing::{delete, get, post, put},
	ApiRouter,
};
use axum::{extract::Path, http::StatusCode, Extension};
use axum_jsonschema::Json;
use schemars::JsonSchema;
use std::{
	collections::HashMap,
	time::Instant,
};

use crate::{
	db::{self, Collection, DbExtension, Embedding, Error as DbError, SimilarityResult},
	errors::HTTPError,
	similarity::Distance,
};

pub fn handler() -> ApiRouter {
	ApiRouter::new().nest(
		"/collections",
		ApiRouter::new()
			.api_route("/", get(get_collections))
			.api_route("/:collection_name", put(create_collection))
			.api_route("/:collection_name", post(query_collection))
			.api_route("/:collection_name", get(get_collection_info))
			.api_route("/:collection_name", delete(delete_collection))
			.api_route("/:collection_name/embeddings", get(get_embeddings))
			.api_route("/:collection_name/embeddings", post(query_embeddings))
			.api_route("/:collection_name/embeddings", delete(delete_embeddings))
			.api_route("/:collection_name/embeddings/:embedding_id", put(insert_into_collection))
			.api_route("/:collection_name/embeddings/:embedding_id", get(get_embedding))
			.api_route("/:collection_name/embeddings/:embedding_id", delete(delete_embedding)),
	)
}

/// Get collection names
async fn get_collections(
	Extension(db): DbExtension,
) -> Result<Json<Vec<String>>, HTTPError> {
	tracing::trace!("Getting collection names");

	let db = db.read().await;

	let results = db.list();

	Ok(Json(results))
}

/// Create a new collection
async fn create_collection(
	Path(collection_name): Path<String>,
	Extension(db): DbExtension,
	Json(req): Json<Collection>,
) -> Result<StatusCode, HTTPError> {
	tracing::trace!(
		"Creating collection {collection_name} with dimension {}",
		req.dimension
	);

	let mut db = db.write().await;

	let create_result = db.create_collection(collection_name, req.dimension, req.distance);
	drop(db);

	match create_result {
		Ok(_) => Ok(StatusCode::CREATED),
		Err(db::Error::UniqueViolation) => {
			Err(HTTPError::new("Collection already exists").with_status(StatusCode::CONFLICT))
		},
		Err(_) => Err(HTTPError::new("Couldn't create collection")),
	}
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
struct QueryCollectionQuery {
	/// Vector to query with
	query: Vec<f32>,
	/// Metadata to filter with
	filter: Option<Vec<HashMap<String, String>>>,
	/// Number of results to return
	k: Option<usize>,
}

/// Query a collection
#[allow(clippy::significant_drop_tightening)]
async fn query_collection(
	Path(collection_name): Path<String>,
	Extension(db): DbExtension,
	Json(req): Json<QueryCollectionQuery>,
) -> Result<Json<Vec<SimilarityResult>>, HTTPError> {
	tracing::trace!("Querying collection {collection_name}");

	let db = db.read().await;
	let collection = db
		.get_collection(&collection_name)
		.ok_or_else(|| HTTPError::new("Collection not found").with_status(StatusCode::NOT_FOUND))?;

	if req.query.len() != collection.dimension {
		return Err(HTTPError::new("Query dimension mismatch").with_status(StatusCode::BAD_REQUEST));
	}

	let instant = Instant::now();
	let results = collection.get_by_metadata_and_similarity(&req.filter.unwrap_or_default(), &req.query, req.k.unwrap_or(1));
	drop(db);

	tracing::trace!("Query to {collection_name} took {:?}", instant.elapsed());
	Ok(Json(results))
}

#[derive(Debug, serde::Serialize, JsonSchema)]
struct CollectionInfo {
	/// Name of the collection
	name: String,
	/// Dimension of the embeddings in the collection
	dimension: usize,
	/// Distance function used for the collection
	distance: Distance,
	/// Number of embeddings in the collection
	embedding_count: usize,
}

/// Get collection info
#[allow(clippy::significant_drop_tightening)]
async fn get_collection_info(
	Path(collection_name): Path<String>,
	Extension(db): DbExtension,
) -> Result<Json<CollectionInfo>, HTTPError> {
	tracing::trace!("Getting collection info for {collection_name}");

	let db = db.read().await;
	let collection = db
		.get_collection(&collection_name)
		.ok_or_else(|| HTTPError::new("Collection not found").with_status(StatusCode::NOT_FOUND))?;

	Ok(Json(CollectionInfo {
		name: collection_name,
		distance: collection.distance,
		dimension: collection.dimension,
		embedding_count: collection.embeddings.len(),
	}))
}

/// Delete a collection
async fn delete_collection(
	Path(collection_name): Path<String>,
	Extension(db): DbExtension,
) -> Result<StatusCode, HTTPError> {
	tracing::trace!("Deleting collection {collection_name}");

	let mut db = db.write().await;

	let delete_result = db.delete_collection(&collection_name);
	drop(db);

	match delete_result {
		Ok(_) => Ok(StatusCode::NO_CONTENT),
		Err(DbError::NotFound) => {
			Err(HTTPError::new("Collection not found").with_status(StatusCode::NOT_FOUND))
		},
		Err(_) => Err(HTTPError::new("Couldn't delete collection")),
	}
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
struct EmbeddingData {
	/// Vector computed from a text chunk
	vector: Vec<f32>,
	/// Metadata about the source text
	metadata: Option<HashMap<String, String>>,
}

/// Insert a vector into a collection
async fn insert_into_collection(
	Path((collection_name, embedding_id)): Path<(String, String)>,
	Extension(db): DbExtension,
	Json(embedding_data): Json<EmbeddingData>,
) -> Result<StatusCode, HTTPError> {
	tracing::trace!("Inserting into collection {collection_name}");

	let mut db = db.write().await;

	let embedding = Embedding {
		id: embedding_id,
		vector: embedding_data.vector,
		metadata: embedding_data.metadata,
	};
	let insert_result = db.insert_into_collection(&collection_name, embedding);
	drop(db);

	match insert_result {
		Ok(_) => Ok(StatusCode::CREATED),
		Err(DbError::NotFound) => {
			Err(HTTPError::new("Collection not found").with_status(StatusCode::NOT_FOUND))
		},
		Err(DbError::UniqueViolation) => {
			Err(HTTPError::new("Vector already exists").with_status(StatusCode::CONFLICT))
		},
		Err(DbError::DimensionMismatch) => Err(HTTPError::new(
			"The provided vector has the wrong dimension",
		)
		.with_status(StatusCode::BAD_REQUEST)),
	}
}

/// Query embeddings in a collection
async fn get_embeddings(
	Path(collection_name): Path<String>,
	Extension(db): DbExtension,
) -> Result<Json<Vec<String>>, HTTPError> {
	tracing::trace!("Querying embeddings from collection {collection_name}");

	let db = db.read().await;
	let collection = db
		.get_collection(&collection_name)
		.ok_or_else(|| HTTPError::new("Collection not found").with_status(StatusCode::NOT_FOUND))?;

	let results = collection.list();
	drop(db);

	Ok(Json(results))
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
struct EmbeddingsQuery {
	/// Metadata to filter with
	filter: Vec<HashMap<String, String>>,
	/// Number of results to return
	k: Option<usize>,
}

/// Query embeddings in a collection
async fn query_embeddings(
	Path(collection_name): Path<String>,
	Extension(db): DbExtension,
	Json(req): Json<EmbeddingsQuery>,
) -> Result<Json<Vec<Embedding>>, HTTPError> {
	tracing::trace!("Querying embeddings from collection {collection_name}");

	let db = db.read().await;
	let collection = db
		.get_collection(&collection_name)
		.ok_or_else(|| HTTPError::new("Collection not found").with_status(StatusCode::NOT_FOUND))?;

	let instant = Instant::now();
	let results = collection.get_by_metadata(&req.filter, req.k.unwrap_or(1));
	drop(db);

	tracing::trace!("Query embeddings from {collection_name} took {:?}", instant.elapsed());
	Ok(Json(results))
}

/// Delete embeddings in a collection
async fn delete_embeddings(
	Path(collection_name): Path<String>,
	Extension(db): DbExtension,
	Json(req): Json<EmbeddingsQuery>,
) -> Result<StatusCode, HTTPError> {
	tracing::trace!("Querying embeddings from collection {collection_name}");

	let mut db = db.write().await;
	let collection = db
		.get_collection_mut(&collection_name)
		.ok_or_else(|| HTTPError::new("Collection not found").with_status(StatusCode::NOT_FOUND))?;

	collection.delete_by_metadata(&req.filter);
	drop(db);

	Ok(StatusCode::NO_CONTENT)
}

/// Get an embedding from a collection
async fn get_embedding(
	Path((collection_name, embedding_id)): Path<(String, String)>,
	Extension(db): DbExtension,
) -> Result<Json<Embedding>, HTTPError> {
	tracing::trace!("Getting {embedding_id} from collection {collection_name}");

	let db = db.read().await;
	let collection = db
		.get_collection(&collection_name)
		.ok_or_else(|| HTTPError::new("Collection not found").with_status(StatusCode::NOT_FOUND))?;

	let embedding = collection
		.get(&embedding_id)
		.ok_or_else(|| HTTPError::new("Embedding not found").with_status(StatusCode::NOT_FOUND))?;

	Ok(Json(embedding.to_owned()))
}

/// Delete an embedding from a collection
async fn delete_embedding(
	Path((collection_name, embedding_id)): Path<(String, String)>,
	Extension(db): DbExtension,
) -> Result<StatusCode, HTTPError> {
	tracing::trace!("Removing embedding {embedding_id} from collection {collection_name}");

	let mut db = db.write().await;
	let collection = db
		.get_collection_mut(&collection_name)
		.ok_or_else(|| HTTPError::new("Collection not found").with_status(StatusCode::NOT_FOUND))?;

	let delete_result = collection.delete(&embedding_id);
	drop(db);

	match delete_result {
		true => Ok(StatusCode::NO_CONTENT),
		false => Err(HTTPError::new("Embedding not found").with_status(StatusCode::NOT_FOUND)),
	}
}
