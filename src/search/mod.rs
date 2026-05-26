pub mod score;

use std::path::PathBuf;

use axum::{Json, extract::Query};
use backend::{
    archive::content::Content,
    assignments::{AssignmentSet, AssignmentUnit, OwnedAssignmentSet},
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct SearchParams {
    pub query: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct SearchResult(Vec<SearchResultItem>);

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SearchResultItem {
    line_before: Option<Line>,
    line: Line,
    line_after: Option<Line>,
    moment: Moment,
    score: f64,
}

type Timestamp = i64;
#[derive(Deserialize, Serialize)]
pub(crate) struct Line {
    time: Timestamp,
    speakers: Vec<String>,
    text: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Moment {
    timestamp: String,
    episode_name: String,
    episode_number: u8,
    season_name: String,
    thumbnail_url: String,
    youtube_url: String,
}

pub(crate) async fn search(
    Query(SearchParams { query }): Query<SearchParams>,
) -> Json<SearchResult> {
    let Content(lines) = Content::from_file(PathBuf::from("documents/content.txt")).unwrap();
    let assignment_set =
        OwnedAssignmentSet::from_file(PathBuf::from("documents/assignment.csv")).unwrap();

    let borrowed_assignment_set = assignment_set.into();
    let map_entry_closure = |entry: (usize, &str, f64)| {
        map_entry_to_item(
            (entry.0, entry.2),
            &borrowed_assignment_set,
            lines.as_slice(),
        )
    };
    let mut results: Vec<SearchResultItem> = lines
        .iter()
        .enumerate()
        .par_bridge()
        .map(|(_i, line)| {
            (
                _i,
                line.as_str(),
                score::attribute_score(&query, line.as_str()),
            )
        })
        .map(map_entry_closure)
        .collect();

    results.sort_by(|a, b| b.score.total_cmp(&a.score));
    results.truncate(413);
    Json(SearchResult(results))
}

fn map_entry_to_item(
    (i, score): (usize, f64),
    &AssignmentSet(ref assignments_vec): &AssignmentSet,
    lines: &[String],
) -> SearchResultItem {
    let line_before = if i > 0 {
        Some(line_from_assignment_and_text(
            &assignments_vec[i - 1],
            lines[i - 1].clone(),
        ))
    } else {
        None
    };

    let line: Line = line_from_assignment_and_text(&assignments_vec[i], lines[i].clone());

    let line_after = if i < lines.len() - 1 {
        Some(line_from_assignment_and_text(
            &assignments_vec[i + 1],
            lines[i + 1].clone(),
        ))
    } else {
        None
    };

    let timestamp = line_before.as_ref().map(|l| l.time).unwrap_or(line.time);
    let timestamp = chrono::TimeDelta::milliseconds(timestamp);
    let timestamp_str = format!(
        "{}:{:0>2}:{:0>2}",
        timestamp.num_hours(),
        timestamp.num_minutes() % 60,
        timestamp.num_seconds() % 60,
    );

    // Temporarily mocked values
    // TODO: Add episode-tracking file
    let moment: Moment = Moment {
        timestamp: timestamp_str,
        episode_name: String::from("Episódio 1"),
        episode_number: 1,
        season_name: String::from("Ordem Paranormal RPG"),
        thumbnail_url: String::new(),
        youtube_url: format!(
            "https://www.youtube.com/watch?v=uplnCYc0fDg&t={}s",
            timestamp.num_seconds()
        ),
    };

    SearchResultItem {
        line_before,
        line,
        line_after,
        moment,
        score,
    }
}

fn line_from_assignment_and_text(assignment: &AssignmentUnit, text: String) -> Line {
    let AssignmentUnit { time, assignments } = assignment;
    let speakers: Vec<_> = assignments.iter().map(ToString::to_string).collect();
    let time = *time;
    Line {
        speakers,
        text,
        time,
    }
}
