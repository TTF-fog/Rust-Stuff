use std::string::String;
#[macro_use]
use serde;
use confy;
use confy::ConfyError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ConfyConfig {
    mode:bool,
    current:String,
    recent_files: Vec<String>
}

impl Default for ConfyConfig {
    fn default() -> Self {
        ConfyConfig {
            mode: false,
            current: "".to_string(),
            recent_files: vec!["file1".parse().unwrap(), "file2".parse().unwrap()]
        }
    }
}

pub fn main() -> Result<(), confy::ConfyError> {
    let mut cfg: ConfyConfig = confy::load("Rust Notes", None)?;
    println!("{:?}", cfg.recent_files);
    add_file_to_recent(&mut cfg, "bye!!".to_string())?;
    println!("{:?}",read_recent().expect("error"));
    Ok(())

}
pub fn add_file_to_recent(config:&mut ConfyConfig,file:String) -> Result<(), ConfyError>{
    config.recent_files.push(file);
    confy::store("Rust Notes", None, &config)?;
    Ok(())
}
pub fn read_recent() -> Result<(Vec<String>),ConfyError>{
    let mut cfg: ConfyConfig = confy::load("Rust Notes", None)?;
    Ok((cfg.recent_files))
}