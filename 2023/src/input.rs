use std::{
    fs::{create_dir, read_to_string, write},
    path::{Path, PathBuf},
};

use anyhow::{Result,anyhow};
use once_cell::sync::Lazy;
use reqwest::blocking;

static INPUT_DIR_PATH: Lazy<&Path> = Lazy::new(|| {Path::new("input")});

#[derive(Debug)]
pub struct Input {
    pub raw: String,
}

fn get_cache_file_path(day: usize) -> PathBuf {
    INPUT_DIR_PATH.join(format!("{day}.txt"))
}

pub fn get_input(day: usize) -> Result<Input> {
    let cached_input = get_input_from_cache(day);

    if cached_input.is_ok() {
        return cached_input;
    }

    let response = blocking::Client::new()
        .get(format!("https://adventofcode.com/2023/day/{day}/input"))
        .header("Cookie", "session=53616c7465645f5ff1a56de37498e4ebe200286024ee4812450a695b1355aef3e55306f1cc236f884a8991b0c70c0152125f83393f7bb1bbe1b72d17b3d1440d")
        .send()?;

    if !response.status().is_success() {
        return Err(anyhow!(format!("{:#?}", response.text())))
    }
    
    let raw = response
        .text()?
        .trim_end()
        .to_owned();

    cache_input(day, &raw)?;

    Ok(Input { raw })
}


fn get_input_from_cache(day: usize) -> Result<Input> {
    if !INPUT_DIR_PATH.exists() {
        return Err(anyhow!("Cache directory not found: {}/", INPUT_DIR_PATH.display()));
    }
    
    let input_file_path = get_cache_file_path(day);
    let input = read_to_string(&input_file_path);

    match input {
        Ok(raw) => Ok(Input { raw }),
        _ => Err(anyhow!("Cache file not found: {}", input_file_path.display()))
    }
}

fn cache_input(day: usize, input: &str) -> Result<()> {
    if !INPUT_DIR_PATH.exists() {
        create_dir(*INPUT_DIR_PATH)?;
    }
    write(get_cache_file_path(day), input)?;
    Ok(())
}
