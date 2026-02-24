mod core;
use core::parse_path::FolderParser;
use core::folder_crawler::Crawler;
use core::encoder::generate_waveform;



#[tokio::main]
async fn main() {
    let parsed = FolderParser::parser();
    let items = Crawler::new(parsed.path)
        .crawl();
    let waves = items
        .iter()
        .map(async move |x| generate_waveform(&x, 1000).await)
        .collect::<Vec<_>>();
 
    for item in waves {
        println!("items: {:?}", item.await);
    }
}
