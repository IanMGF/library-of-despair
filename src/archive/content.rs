use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

pub struct Content(pub Vec<String>);

impl Content {
    pub fn from_file(path: PathBuf) -> io::Result<Self> {
        let file: File = File::open(path)?;
        let reader: BufReader<File> = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

        Ok(Content(lines))
    }
}
