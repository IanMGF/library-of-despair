use std::{fs::File, ops::Index, path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};

type Timestamp = i64;

#[derive(Deserialize, Serialize, Clone)]
pub struct AssignmentUnit {
    pub time: Timestamp,
    pub assignments: Arc<[Arc<str>]>,
}

#[derive(Deserialize)]
pub struct AssignmentSet(Vec<AssignmentUnit>);

impl AssignmentSet {
    pub fn from_file(path: PathBuf) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let mut assignment_set: AssignmentSet = AssignmentSet(vec![]);
        let mut assignment_rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(file);

        for result in assignment_rdr.deserialize() {
            let record_vec: Vec<Arc<str>> = result.unwrap();
            let time: i64 = record_vec[0].parse().unwrap();
            let assignments: Arc<[Arc<str>]> = record_vec.into_iter().skip(1).collect();
            assignment_set.0.push(AssignmentUnit { time, assignments });
        }

        Ok(assignment_set)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Index<usize> for AssignmentSet {
    type Output = AssignmentUnit;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
