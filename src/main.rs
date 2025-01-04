use serde::{ Serialize,Deserialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command};
use reqwest::blocking::{Client, Response};
use select::document::Document;
use select::node::Node;
use select::predicate::Name;

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

#[derive(Debug, Serialize, Deserialize,Clone)]
struct Manga{
  name: String,
  url: String,
  chapter: usize,
}

impl Manga{
    fn new(name: String,url: String,chapter: usize) ->Self {
        Manga {
            name,
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
fn add_new_manga(manga: &Manga,config: &Config) {
    //retrieve old manga history list
   let mut manga_list: Vec<Manga> = match fs::read_to_string(&config.config.cache) {
       Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
       Err(_) => Vec::new(),
   };
    manga_list.push(manga.clone());
    let ser_manga = serde_json::to_string_pretty(&manga_list).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&config.config.cache)
        .expect("Failed to open file");
    file.write_all(ser_manga.as_bytes()).expect("Failed to write to file");
}

fn input() -> Manga{
    println!("******* BETEL READER ********");
    println!("Enter name of the manga: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    println!("Enter url for {}",name);
    let mut url = String::new();
    std::io::stdin().read_line(&mut url).unwrap();
    let url = url.trim();

    println!("Enter chapter number for {}",name);
    let mut chapter_input = String::new();
    std::io::stdin().read_line(&mut chapter_input).unwrap();

    let chapter: usize = match chapter_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! Please enter valid chapter number");
            exit(1);
        }
    };
    let manga: Manga = Manga::new(String::from(name),String::from(url),chapter);
    println!("Adding manga: {:?}",manga);
    manga
}
fn attribute_finder(image: &Node) -> Option<String> {
    let img_types = [".png", ".jpg", ".jpeg"];
    for (attr, value) in image.attrs() {
        if img_types.iter().any(|typ| value.contains(typ)) {
            return Some(attr.to_string());
        }
    }
    None
}
fn get_chapter() -> Vec<String>{
    let client = Client::new();
    let response: Response = client
        .get("").send().expect("Falied to retrieve");
    let mut chapter_list = Vec::new();
    if response.status().is_success() {
        let body = response.text().expect("Failed to read response");
        let document = Document::from(body.as_str());
        for image in document.find(Name("img")) {
            if let Some(attr) = attribute_finder(&image) {
                let src = image.attr(&attr).unwrap_or("").trim();
                if !src.is_empty() {
                    if !src.starts_with("https:") {
                        chapter_list.push(format!("https:{}", src));
                    } else {
                        chapter_list.push(src.to_string());
                    }
                }
            }
        }

        // let status = Command::new("feh")
        //     .arg("-Z") // Zoom images to fit the screen
        //     .arg("-F") // Fullscreen mode
        //     .args(chapter_list) // Pass the image URLs directly
        //     .status()
        //     .expect("Failed to execute feh");
        //
        // if !status.success() {
        //     eprintln!("feh exited with a non-zero status");
        // }
    }
     chapter_list
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
    let manga = input();
    add_new_manga(&manga,&config);
    get_chapter();
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_read_config(){
        assert_eq!(config().unwrap().config.cache,"/home/.cache/manga-his.json");
    }
}
