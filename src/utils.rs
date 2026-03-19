use std::{
    collections::HashSet,
    fs,
    io::{BufRead, BufReader, Error},
};

pub fn parse_file(file_name: &str) -> Result<Vec<String>, Error> {
    let file = fs::File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut names = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        names.extend(line.split(',').map(|s| s.trim().to_lowercase().to_string()));
    }

    let names: Vec<String> = names.into_iter().collect();

    Ok(names)
}
