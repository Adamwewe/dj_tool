use std::collections::HashMap;
use std::fs;
// use std::ops::Deref;
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct Crawler{
    pub path : String,
    pub format : String,
}

impl Crawler {
    pub fn new(path : String) -> Self {
        Self{path: path,
        format: String::new()}
    }

    pub fn crawl(&mut self) -> Vec<Crawler>{

        let items = WalkDir::new(String::from("./")
        + &self.path.clone());
        
        let mut collect : Vec<Crawler> = Vec::new();

        for file in items.into_iter().filter_map(|file| file.ok()) {
            if file.metadata()
                .unwrap()
                .is_file() {  //what the fuck is this
                   let parsed_path = file.path().display().to_string();

                    let crawler_instance = Self{
                        path : parsed_path.clone(),
                        format : self.get_extension(&parsed_path)
                        };
                    collect.push(crawler_instance)
            } 
    }
        collect 
    }
    fn get_extension(&mut self, path: &String) ->  String {
        let split_path = path.split(".").collect::<Vec<_>>();
        let extension_index = split_path.len() - 1;
        let extension = &split_path[extension_index];
        String::from(*extension)
    }

}

