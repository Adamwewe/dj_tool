mod core;
use core::folder_crawler;
use core::parse_path;
use core::encoder;
use crate::encoder::generate_waveform;



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
/*

steps:
    - figure out if flac or wav
    
    if flac:
        - Read flac Header
            - Parse metadata blocks
            - Decode audio frames
            - subframe decoding
            - Apply residual encoding
            - 
 */


    // let bytes : Option<Vec<u8>> = parseToBytes(items);


//     let encoded  : Vec<u8> = pcmEncoder(bytes.unwrap());

//     println!("{:?}", encoded);
