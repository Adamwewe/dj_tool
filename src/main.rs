mod core;
use core::parse_path::FolderParser;
use core::folder_crawler::Crawler;
use core::audio::decoder::decode_audio;



#[tokio::main]
async fn main() {
    let parsed = FolderParser::parser();
    let items = Crawler::new(parsed.path)
        .crawl();

    let waves = items
        .iter()
        .map(|x| decode_audio(&x.path))
        .collect::<Vec<_>>();

     for item in waves {
         println!("items: {:?}", item);
     }
}
