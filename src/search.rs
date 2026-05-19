use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct SearchParams {
    pub query: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct SearchResult(Vec<SearchResultItem>);

#[derive(Deserialize, Serialize)]
pub(crate) struct SearchResultItem {
    text: String,
    character: String,
    portrait_url: String,
}

pub(crate) async fn search(_params: Query<SearchParams>) -> Json<SearchResult> {
    // TODO: Implement search algorithms
    Json(SearchResult(vec![]))
}
