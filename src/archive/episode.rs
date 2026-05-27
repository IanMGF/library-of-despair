use std::{fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    archive::assignments::OwnedAssignmentSet, archive::cast::Cast, archive::content::Content,
};

#[derive(Serialize, Deserialize)]
pub struct Episode {
    pub name: String,
    pub id: String,
    pub season_name: String,
    pub season_id: String,
    pub youtube_url: String,
    pub number: u8,
    pub wiki_url: Option<String>,
}

impl Episode {
    pub fn get_episodes_list() -> std::io::Result<Vec<Episode>> {
        let file: File = File::open("documents/episodes.yaml")?;
        let episodes: Vec<Episode> = yaml_serde::from_reader(file).unwrap();
        Ok(episodes)
    }

    pub fn get_content(&self) -> std::io::Result<Content> {
        let id: &str = self.id.as_str();
        let path: PathBuf = PathBuf::from(format!("documents/{id}/content.txt"));
        Content::from_file(path)
    }

    pub fn get_cast(&self) -> std::io::Result<Cast> {
        let id: &str = self.id.as_str();
        let file: File = File::open(format!("documents/{id}/content.txt"))?;
        let cast: Cast = yaml_serde::from_reader(file).unwrap();
        Ok(cast)
    }

    pub fn get_assignments(&self) -> std::io::Result<OwnedAssignmentSet> {
        let id: &str = self.id.as_str();
        let path: PathBuf = PathBuf::from(format!("documents/{id}/assignment.csv"));
        OwnedAssignmentSet::from_file(path)
    }
}
