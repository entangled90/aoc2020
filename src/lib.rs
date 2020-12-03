pub mod aoc {
    use std::fs::File;
    use std::io::BufReader;

    pub fn load_data(
        filename: &str,
    ) -> std::result::Result<BufReader<File>, Box<dyn std::error::Error>> {
        let file = File::open(filename)?;
        Ok(BufReader::new(file))
    }
}
