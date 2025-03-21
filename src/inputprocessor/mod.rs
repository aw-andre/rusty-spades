use super::config::Config;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use bio::io::fastq::{ Reader, Records };

impl Config {
    fn to_paths(&self) -> (&Path, &Path) {
        let forward = Path::new(self.forward.as_deref().unwrap());
        let reverse = Path::new(self.reverse.as_deref().unwrap());
        (forward, reverse)
    }

    fn to_files(&self) -> (Reader<BufReader<File>>, Reader<BufReader<File>>) {
        let (forward, reverse) = self.to_paths();
        let forward = Reader::from_file(forward).expect("Forward file is invalid!");
        let reverse = Reader::from_file(reverse).expect("Reverse file is invalid!");
        (forward, reverse)
    }

    pub fn to_iterators(&self) -> (Records<BufReader<File>>, Records<BufReader<File>>) {
        let (forward, reverse) = self.to_files();
        let forward = forward.records();
        let reverse = reverse.records();
        (forward, reverse)
    }
}

