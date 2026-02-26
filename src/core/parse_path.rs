use std::io;

//TODO: think about how this would work with typer
pub struct FolderParser{
    pub path : String
}

impl FolderParser {
    pub fn parser() -> Self {
        
    println!("Please enter folder name: ");

    let mut path = String::new();

    io::stdin()
        .read_line(&mut path)
        .expect("failed to read from stdin");

    let path = path.trim()
        .parse::<String>()
        .expect("invalid input");
        
        Self {
            path
        }
    }
}
