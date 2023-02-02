use std::fs::File;
use std::io::Read;
use std::path::Path;


pub trait WordCount {
    type Error;
    fn words(&mut self) -> Result<usize, Self::Error>;
}

impl WordCount for File {
    type Error = std::io::Error;

    fn words(&mut self) -> Result<usize, Self::Error> {
        let mut lines: String = String::new();
        self.read_to_string(&mut lines)?;

        let removed_new_lines = lines.trim().replace('\n', " ");
        let words: Vec<&str> = removed_new_lines.split(' ').collect();
        dbg!(words.clone());

        Ok(words.len())
    }
}


fn main() -> std::io::Result<()> {
    let path = Path::new("test.txt");
    let mut file = File::open(path)?;

    println!("{} contains {} words", path.file_name().unwrap_or_default().to_str().unwrap(), file.words()?);

    Ok(())
}
