mod parser;

use serde::{Serialize, Deserialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command};
use clap::Parser;
use reqwest::blocking::{Client, Response};
use select::document::Document;
use select::node::Node;
use select::predicate::Name;
use crate::parser::Args;

// const FILE_NAME: &str = "Config.toml";
const CONFIG: &str = include_str!("../Config.toml");
#[derive(Debug, Serialize,Deserialize)]
struct Config{
    config: ConfigurationData,
}
#[derive(Debug, Serialize,Deserialize)]
struct ConfigurationData{
    cache:String,
    url:String,
    chapter_list: String,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
struct Manga{
  name: String,
  url: String,
  chapter: u32,
}

impl Manga{
    fn new(name: String,url: String,chapter: u32) ->Self {
        Manga {
            name,
            url,
            chapter
        }
    }
}
fn config() -> Result<Config, std::io::Error> {
    let config: Config = match toml::from_str(&CONFIG) {
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

fn attribute_finder(image: &Node) -> Option<String> {
    let img_types = [".png", ".jpg", ".jpeg"];
    for (attr, value) in image.attrs() {
        if img_types.iter().any(|typ| value.contains(typ)) {
            return Some(attr.to_string());
        }
    }
    None
}

fn get_chapter_list(config: &Config) -> Vec<String> {
    let client = Client::new();
    let response = client
        .get(&config.config.chapter_list)
        .send()
        .expect("Failed to retrieve HTML document");
    let mut chapter_list = Vec::new();

    if response.status().is_success() {
        let body = response.text().expect("Failed to read response body");
        let document = Document::from(body.as_str());

        // Find all `<li>` tags with an `<a>` child
        for li in document.find(Name("li")) {
            if let Some(a_tag) = li.find(Name("a")).next() {
                // Get the text content of the `<a>` tag
                let chapter_name = a_tag.text().trim().to_string();
                if !chapter_name.is_empty() {
                    chapter_list.push(chapter_name);
                }
            }
        }
    }
    for chapters in &chapter_list {
        println!("{}", chapters);
    }
    chapter_list
}
fn get_chapter(formatted_url: &str) -> Vec<String>{
    let client = Client::new();
    let response: Response = client
        .get(formatted_url).send().expect("Failed to retrieve");
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

    }
     chapter_list
}

fn format_url(url: &str,num: &u32) -> String {
      url.replace("{{}}", &num.to_string())
}

fn render_with_feh(images: Vec<String>){
    let status = Command::new("feh")
        .arg("-Z") // Zoom images to fit the screen
        .arg("--geometry")
        .arg("960x540") // Fullscreen mode
        .args(images) // Pass the image URLs directly
        .status()
        .expect("Failed to execute feh");

    if !status.success() {
        eprintln!("feh exited with a non-zero status");
    }
}
fn render_chapter(num: &u32,config: &Config){
    let pretty_url = format_url(&config.config.url,&num);
    let images : Vec<String> = get_chapter(&pretty_url);
    render_with_feh(images);
}

fn main() {
    let config = config().unwrap();
    let args = Args::parse();
    if args.list{
        get_chapter_list(&config);
    }
    if let Some(num) = args.num {
        render_chapter(&num,&config);
    }

    //TODO: cache for maintaining state
    // match cache_file(&config.config.cache){
    //     Ok(_) => {
    //         println!("Cache file created successfully!");
    //     },
    //     Err(e) => {
    //         eprintln!("Error creating cache file: {}",e);
    //         exit(1);
    //     }
    // }

}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_read_config(){
        assert_eq!(config().unwrap().config.cache,"/home/.cache/manga-his.json");
    }
}
