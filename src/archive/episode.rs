use std::{fs::File, io, path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{archive::assignments::AssignmentSet, archive::cast::Cast, archive::content::Content};

#[derive(Serialize, Deserialize)]
pub struct EpisodeInfo {
    pub name: Arc<str>,
    pub id: Arc<str>,
    pub season_name: Arc<str>,
    pub season_id: Arc<str>,
    pub video_id: Arc<str>,
    pub number: u8,
    pub wiki_url: Option<Arc<str>>,
}

impl EpisodeInfo {
    pub fn get_episodes_list() -> std::io::Result<Vec<EpisodeInfo>> {
        let file: File = File::open("documents/episodes.yaml")?;
        let episodes: Vec<EpisodeInfo> = yaml_serde::from_reader(file).unwrap();
        Ok(episodes)
    }

    pub fn load_content(&self) -> std::io::Result<Content> {
        let id: &str = &self.id;
        let path: PathBuf = PathBuf::from(format!("documents/{id}/content.txt"));
        Content::from_file(path)
    }

    pub fn load_cast(&self) -> std::io::Result<Cast> {
        let id: &str = &self.id;
        let file: File = File::open(format!("documents/{id}/cast.yaml"))?;
        let cast: Cast = yaml_serde::from_reader(file).unwrap();
        Ok(cast)
    }

    pub fn load_assignment(&self) -> std::io::Result<AssignmentSet> {
        let id: &str = &self.id;
        let path: PathBuf = PathBuf::from(format!("documents/{id}/assignment.csv"));
        AssignmentSet::from_file(path)
    }

    pub fn load_episode(self) -> io::Result<Episode> {
        let cast = self.load_cast()?;
        let content = self.load_content()?;
        let assignment = self.load_assignment()?;
        Ok(Episode {
            cast,
            info: self,
            content,
            assignment,
        })
    }
}

pub struct Episode {
    cast: Cast,
    info: EpisodeInfo,
    content: Content,
    assignment: AssignmentSet,
}

impl Episode {
    pub fn get_info(&self) -> &EpisodeInfo {
        &self.info
    }

    pub fn get_content(&self) -> &Content {
        &self.content
    }

    pub fn get_cast(&self) -> &Cast {
        &self.cast
    }

    pub fn get_assignment(&self) -> &AssignmentSet {
        &self.assignment
    }

    pub fn line_count(&self) -> usize {
        debug_assert!(self.content.len() == self.assignment.len());
        self.content.len()
    }
}
