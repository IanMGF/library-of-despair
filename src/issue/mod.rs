use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueData {
    pub episode_id: Arc<str>,
    pub line_number: u32,
    pub suggestion: Arc<str>,
}

#[axum::debug_handler]
pub async fn create_issue(
    State(pool): State<Arc<PgPool>>,
    Json(issue_data): Json<IssueData>,
) -> StatusCode {
    let suggestion = issue_data.suggestion.as_ref();
    let line_number = issue_data.line_number as i32;
    let ep_id = issue_data.episode_id.as_ref();

    let create_query = sqlx::query!(
        r#"
            INSERT INTO transcription_issues (episode_id, line_number, issue)
            VALUES ($1, $2, $3)
        "#,
        ep_id,
        line_number,
        suggestion
    );

    let result = create_query.execute(pool.as_ref()).await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            eprintln!("{err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueFilter {
    pub episode_id: Option<Arc<str>>,
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "status")]
#[sqlx(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub enum LoggedIssueStatus {
    Pending,
    Ignored,
    Corrected,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggedIssue {
    pub episode_id: Arc<str>,
    pub line_number: i32,
    pub issue: Arc<str>,
    pub status: LoggedIssueStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueList(Vec<LoggedIssue>);

#[axum::debug_handler]
pub async fn get_issues(
    Query(_filter): Query<IssueFilter>,
    State(pool): State<Arc<PgPool>>,
) -> Json<IssueList> {
    let retrieve_query = sqlx::query_as!(
        LoggedIssue,
        r#"SELECT episode_id, line_number, issue, status as "status: LoggedIssueStatus" FROM transcription_issues"#
    );

    let result = retrieve_query.fetch_all(pool.as_ref()).await;

    match result {
        Ok(v) => Json(IssueList(v)),
        Err(err) => {
            eprintln!("{err}");
            Json(IssueList(Vec::new()))
        }
    }
}
