use std::collections::HashMap;
use std::fs;
// use std::ops::Deref;
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct Crawler{
    pub folders : HashMap<String, Vec<String>>,
    pub path : String
}

impl Crawler {
    pub fn new(path : String) -> Self {
        Self{folders: HashMap::new(),
            path: path}
    }

    pub fn crawl(&mut self) -> Vec<String>{//-> Option<ReadDir> {
    

        let items = WalkDir::new(String::from("./")
        + &self.path.clone());
        
        let mut collect : Vec<String> = Vec::new();

        for file in items.into_iter().filter_map(|file| file.ok()) {
            if file.metadata().unwrap().is_file(){  //what the fuck is this
            collect.push(file.path()
                        .display()
                        .to_string());
        } 
    }

        
        return collect ;        
        
    // pub fn view(&self) {
        //https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html
    // }
    }
}

