use common_types::archive::{assignments::AssignmentUnit, episode::Episode};
use common_types::result::Timestamp;

use crate::search::{CharacterId, DialogueLine, Moment, SearchResultItem};

pub(crate) struct ScoredLine<'a> {
    episode: &'a Episode,
    line_number: usize,
    score: f64,
}

impl<'a> ScoredLine<'a> {
    pub fn new(episode: &'a Episode, line_number: usize, score: f64) -> Self {
        ScoredLine {
            episode,
            line_number,
            score,
        }
    }
}

/// A wrapper type meant to implement
/// ordering operators (<, ==, >) using [SearchResultItem.score]
/// as a base, as implementing them on the type itself would be
/// semantically incoherent
impl<'a> PartialEq for ScoredLine<'a> {
    fn eq(&self, other: &Self) -> bool {
        // self.0.score.total_cmp(&other.0.score) == std::cmp::Ordering::Equal
        self.score.total_cmp(&other.score) == std::cmp::Ordering::Equal
    }
}

impl<'a> Eq for ScoredLine<'a> {}

impl<'a> PartialOrd for ScoredLine<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for ScoredLine<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.total_cmp(&other.score)
    }
}

impl<'a> From<ScoredLine<'a>> for SearchResultItem {
    fn from(sc: ScoredLine<'a>) -> Self {
        let ScoredLine {
            episode,
            line_number,
            score,
        } = sc;

        let line_before = line_from_episode_and_line_number(episode, line_number - 1);
        let line = line_from_episode_and_line_number(episode, line_number).unwrap();
        let line_after = line_from_episode_and_line_number(episode, line_number + 1);

        // If 'line_before' exists, 'timestamp' should be it's time. Otherwise, it should be the time of 'line'
        let timestamp = line_before.as_ref().unwrap_or(&line).time;

        let ep_info = episode.get_info();
        let moment: Moment = Moment {
            timestamp,
            episode_name: ep_info.name.clone(),
            episode_number: ep_info.number,
            season_name: ep_info.season_name.clone(),
            video_id: ep_info.video_id.clone(),
            episode_id: ep_info.id.clone(),
        };

        SearchResultItem {
            line_before,
            line,
            line_after,
            moment,
            score,
        }
    }
}

fn line_from_episode_and_line_number(
    episode: &Episode,
    line_number: usize,
) -> Option<DialogueLine> {
    if episode.line_count() <= line_number {
        return None;
    }

    let assignment_set = &episode.get_assignment();
    let content = &episode.get_content();

    let AssignmentUnit { time, assignments } = &assignment_set[line_number];

    let speakers = assignments
        .iter()
        .flat_map(|id| {
            let member_by_id = episode.get_cast().get_member_by_id(id);
            member_by_id.map(|m| CharacterId {
                id: m.id.clone(),
                name: m.name.clone(),
            })
        })
        .collect();
    let text = content[line_number].clone();
    let time = Timestamp(*time);
    let number = line_number as u16;

    Some(DialogueLine {
        speakers,
        text,
        time,
        number,
    })
}
