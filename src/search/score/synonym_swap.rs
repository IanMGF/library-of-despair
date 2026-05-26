use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SynonymSwappingScore(HashMap<String, HashSet<String>>);
