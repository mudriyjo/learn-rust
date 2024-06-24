// 1. Load file
// 2. Load file and capitalize -?
// 3. Load file and desirialize 2 type of error ways of handle this:
// - anyhow
// - thiserror + Enum

use std::{
    io::{self, Error, Read},
    path::Path,
};

use serde::Deserialize;

fn read_string_from_file() -> Result<String, std::io::Error> {
    let path = Path::new("my_file.txt");
    let content = std::fs::read_to_string(path);
    content
}

fn read_file_and_uppercase() -> Result<String, std::io::Error> {
    let content = read_string_from_file()?;
    Ok(content.to_uppercase())
}

#[derive(Debug, Deserialize)]
struct User {
    name: String,
    age: u32,
}

fn load_file_deserialize() -> Result<Vec<User>, std::io::Error> {
    let raw = read_string_from_file()?;
    let users: Vec<User> = serde_json::from_str(&raw)?;
    Ok(users)
}

fn load_file_deserialize_anyhow() -> anyhow::Result<Vec<User>> {
    let raw = read_string_from_file()?;
    let users: Vec<User> = serde_json::from_str(&raw)?;
    Ok(users)
}

use thiserror::Error;

#[derive(Error, Debug)]
enum LoadError {
    #[error("Can't load file")]
    FileContentLoadError(String),
    #[error("Can't deseriallize data")]
    DataDesirealizeError(String),
}
fn load_file_deserialize_thiserror() -> Result<Vec<User>, LoadError> {
    let raw =
        read_string_from_file()
        .map_err(|e| LoadError::FileContentLoadError(e.to_string()))?;
    let users: Vec<User> =
        serde_json::from_str(&raw)
        .map_err(|e| LoadError::DataDesirealizeError(e.to_string()))?;
    Ok(users)
}

fn main() {
    let res = read_string_from_file();
    match res {
        Ok(content) => println!("File content: {content}"),
        Err(e) => println!("{e:#?}"),
    };

    let users = load_file_deserialize();
    println!("{users:#?}");

    let users = load_file_deserialize_thiserror();
    println!("{users:#?}");
}
