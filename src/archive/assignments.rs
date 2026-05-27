use std::{collections::HashSet, fs::File, path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};

type Timestamp = i64;

#[derive(Deserialize, Serialize, Clone)]
pub struct AssignmentUnit {
    pub time: Timestamp,
    pub assignments: HashSet<String>,
}

pub struct AssignmentSet(pub Arc<[AssignmentUnit]>);

#[derive(Serialize, Deserialize)]
pub struct OwnedAssignmentSet(pub Vec<AssignmentUnit>);

impl From<OwnedAssignmentSet> for AssignmentSet {
    fn from(OwnedAssignmentSet(value): OwnedAssignmentSet) -> Self {
        AssignmentSet(value.into())
    }
}

impl From<AssignmentSet> for OwnedAssignmentSet {
    fn from(AssignmentSet(value): AssignmentSet) -> Self {
        let v: Vec<AssignmentUnit> = value.into_iter().cloned().collect();
        OwnedAssignmentSet(v)
    }
}

impl OwnedAssignmentSet {
    pub fn from_file(path: PathBuf) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let mut assignment_set: OwnedAssignmentSet = OwnedAssignmentSet(vec![]);
        let mut assignment_rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(file);

        for result in assignment_rdr.deserialize() {
            let record_vec: Vec<String> = result.unwrap();
            let time: i64 = record_vec[0].parse().unwrap();
            let assignments: HashSet<String> = record_vec.into_iter().skip(1).collect();
            assignment_set.0.push(AssignmentUnit { time, assignments });
        }

        Ok(assignment_set)
    }
}
