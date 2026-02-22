use std::io;

pub struct FolderParser{
    pub path : String
}

impl FolderParser {
    pub fn parser() -> Self 
    // TODO: add regex robustness in other method?? Some exception hadnling structure would be nice as well
    {
        
    println!("Please enter folder name: ");

    let mut path = String::new();

    io::stdin()
        .read_line(&mut path)
        .expect("failed to read from stdin");

    let path = path.trim().parse::<String>().expect("invalid input");
        
        Self {
            path : path
        }
    }
}