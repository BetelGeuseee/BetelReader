use serde::{ Serialize,Deserialize};
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use std::process::exit;
const FILE_NAME: &str = "Config.toml";
#[derive(Debug, Serialize,Deserialize)]
struct Config{
    config: ConfigurationData,
}
#[derive(Debug, Serialize,Deserialize)]
struct ConfigurationData{
    cache:String,
    url:String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Manga{
  url: String,
  chapter: usize,
}

impl Manga{
    fn new(url: String,chapter: usize) ->Self {
        Manga {
            url,
            chapter
        }
    }
}
fn config() -> Result<Config, std::io::Error> {
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

//creates json file to store cache - history of manga stored in specified path.
fn cache_file(path: &str) -> Result<(), std::io::Error> {
    if let Some(parent_dir) = Path::new(path).parent() {
        match fs::create_dir_all(parent_dir) {
            Ok(_) => {
                OpenOptions::new()
                    .write(true)     // Open for writing
                    .read(true)      // Allow reading the file
                    .create(true)    // Create the file if it doesn't exist
                    .open(path)?;
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(())
}
fn add_new_manga() -> Result<(), std::io::Error>{

}
fn main() {
    let config = config().unwrap();
    match cache_file(&config.config.cache){
        Ok(_) => {
            println!("Cache file created successfully!");
        },
        Err(e) => {
            eprintln!("Error creating cache file: {}",e);
            exit(1);
        }
    }
    /**

    **/
}


#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_read_config(){
        assert_eq!(config().unwrap().config.cache,"/home/.cache/manga-his.json");
    }
}
