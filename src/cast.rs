use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct CastMember {
    pub r#id: String,
    pub name: String,
    pub img_url: String,
    pub wiki_page: String,
    pub aliases: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Cast(pub HashSet<CastMember>);
