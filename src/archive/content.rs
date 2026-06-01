use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Index,
    path::PathBuf,
    sync::Arc,
};

pub struct Content(Vec<Arc<str>>);

impl Content {
    pub fn from_file(path: PathBuf) -> io::Result<Self> {
        let file: File = File::open(path)?;
        let reader: BufReader<File> = BufReader::new(file);
        let lines: Vec<Arc<str>> = reader.lines().map(Result::unwrap).map(Arc::from).collect();

        Ok(Content(lines))
    }

    pub fn get_lines(&self) -> &[Arc<str>] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for Content {
    type Output = Arc<str>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

// impl Len
