use serde::{ Serialize,Deserialize};
use std::fs;
use std::path::Path;
use std::process::exit;

const URL: &str = "https://myonepiecemanga.com/manga/one-piece-chapter-1122/";
const FILE_NAME: &str = "Config.toml";
#[derive(Debug, Serialize,Deserialize)]
struct Config{
    config: ConfigurationData,
}
#[derive(Debug, Serialize,Deserialize)]
struct ConfigurationData{
    cache:String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Manga{
  url: String,
  chapter: usize,
}

fn read_config() -> Result<Config, std::io::Error> {
    let content = match fs::read_to_string(FILE_NAME) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("An error has occured while reading Config.toml: {}",e);
            return Err(e);
        }
    };
    let config: Config = match toml::from_str(&content) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: occured while mapping toml file {}",e);
            exit(1);
        }
    };
    Ok(config)
}

fn create_cache_file(path: &str) -> Result<(), std::io::Error> {
    if let Some(parent_dir) = Path::new(path).parent() {
        fs::create_dir_all(parent_dir)?;
    }
    Ok(())
}
fn main() {
    let config = read_config().unwrap();
    match create_cache_file(&config.config.cache){
        Ok(_) => {
            print!("Cache file created successfully!");
        },
        Err(e) => {
            eprintln!("Error creating cache file: {}",e);
            exit(1);
        }
    }
}
#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_read_config(){
        assert_eq!(read_config().unwrap().config.cache,"/home/.cache/manga-his.json");
    }
}
