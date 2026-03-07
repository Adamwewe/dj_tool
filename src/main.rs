mod core;
use core::parse_path::FolderParser;
use core::folder_crawler::Crawler;
use core::audio::decoder::decode_audio;
use core::spectrogram::fft;


//This is used as an entry point for dev to avoid recompilation via maturin

#[tokio::main]
async fn main() {
    //TODO: parallelize
    let parsed = FolderParser::parser();
    let items = Crawler::new(parsed.path)
        .crawl();

    let waves = items
        .iter()
        .map(|x| decode_audio(&x.path))
        .collect::<Vec<_>>();

     for item in waves {
        match item.await {
            Ok(sample) => {
                let (samples, sample_rate, channels) = sample;
                let spec = fft::compute_spectrogram(&samples, 2048, 1024); 
                println!("spectrogram: {:?}", spec); 
            },
            Err(e) => eprintln!("Error found: {}", e),
        }
    }
}
