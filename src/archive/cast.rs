use std::{collections::HashSet, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct CastMember {
    pub r#id: Arc<str>,
    pub name: Arc<str>,
    pub img_url: Option<Arc<str>>,
    pub wiki_page: Option<Arc<str>>,
    pub aliases: Vec<Arc<str>>,
}

#[derive(Serialize, Deserialize)]
pub struct Cast(pub HashSet<CastMember>);

impl Cast {
    pub fn get_member_by_id(&self, id: &str) -> Option<&CastMember> {
        self.0
            .iter()
            .find(|member| member.id.as_ref() == id)
    }
}
