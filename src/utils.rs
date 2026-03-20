use std::{
    collections::HashSet,
    fs::{self, File},
    io::{BufRead, BufReader, Error, copy},
    path::PathBuf,
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

pub fn download_precomputed_dataset() -> Result<(), Error> {
    let url = "https://github.com/syioa/ungai/raw/main/assets/dataset/names.zst";
    let cache_dir = dirs::data_local_dir().unwrap().join("ungai");
    let file_path = cache_dir.join("names.zst");

    if !file_path.exists() {
        fs::create_dir_all(&cache_dir)?;
        download_file(url, &file_path)?;
    }

    Ok(())
}

fn download_file(url: &str, path: &PathBuf) -> Result<(), Error> {
    let mut response = reqwest::blocking::get(url).expect("Request Failed");
    let mut file = File::create(path)?;
    copy(&mut response, &mut file)?;

    Ok(())
}

pub fn get_default_dataset_path() -> PathBuf {
    let mut path = dirs::data_local_dir().expect("Couldn't find data directory");

    path.push("ungai");
    path.push("names.zst");

    path
}
