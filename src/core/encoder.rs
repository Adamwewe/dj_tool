use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{Decoder, DecoderOptions};
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use std::fs::File;
use pyo3;


use crate::folder_crawler::Crawler;

#[derive(Debug)]
pub struct WaveformData {
    pub peaks: Vec<f32>,
    pub sample_rate: u32,
    pub duration_secs: f64,
    pub samples_per_peak: usize,
}

pub async fn generate_waveform(
    item_obj: &Crawler,
    target_width: usize, // kept at 500 for initial benchmark
) -> Result<WaveformData, Box<dyn std::error::Error>> {
    let (path, format) = (&item_obj.path, &item_obj.format);

    let file = File::open(path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    
    //TODO: Extension needs to be refactored
    let mut hint = Hint::new();
    hint.with_extension(format);
    let format_opts = FormatOptions::default();
    let metadata_opts = MetadataOptions::default();
   
    //TODO: Default probe needs
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &metadata_opts)?;
    
    let mut format = probed.format;
    
    let track = format.default_track()
        .ok_or("no default track")?;
    
    let sample_rate = track.codec_params.sample_rate
        .ok_or("unknown sample rate")?;
    
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())?;
    
    let mut all_samples = Vec::new();
    
    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(_) => break, // End stream if error encountered
        };

        // Decode the packet
        match decoder.decode(&packet) {
            Ok(decoded) => {
                // Extract samples and convert to f32
                let samples = extract_samples(&decoded);
                all_samples.extend(samples);
            }
            Err(e) => eprintln!("Decode error: {}", e),
        }
    }
    
    let samples_per_peak = all_samples.len() / target_width;
    
    let peaks: Vec<f32> = all_samples
        .chunks(samples_per_peak.max(1))
        .map(|chunk| {
            chunk.iter()
                .map(|&s| s.abs())
                .fold(0.0f32, f32::max)
        })
        .collect();
    
    let duration_secs = all_samples.len() as f64 / sample_rate as f64;
    
    let wave = Ok(WaveformData {
        peaks,
        sample_rate,
        duration_secs,
        samples_per_peak,
    });

    println!("{:?}", wave);
    wave
}

//from Claude
fn extract_samples(decoded: &AudioBufferRef) -> Vec<f32> {
    match decoded {
        AudioBufferRef::F32(buf) => {
            // Average all channels into mono
            let num_channels = buf.spec().channels.count();
            let num_frames = buf.frames();
            
            (0..num_frames)
                .map(|frame| {
                    let sum: f32 = (0..num_channels)
                        .map(|ch| buf.chan(ch)[frame])
                        .sum();
                    sum / num_channels as f32
                })
                .collect()
        }
        AudioBufferRef::U8(buf) => {
            convert_to_f32(buf.chan(0), |s| (s as f32 - 128.0) / 128.0)
        }
        AudioBufferRef::U16(buf) => {
            convert_to_f32(buf.chan(0), |s| (s as f32 - 32768.0) / 32768.0)
        }
        AudioBufferRef::U24(buf) => {
            convert_to_f32(buf.chan(0), |s| s.inner() as f32 / 8388608.0)
        }
        AudioBufferRef::U32(buf) => {
            convert_to_f32(buf.chan(0), |s| (s as f32 - 2147483648.0) / 2147483648.0)
        }
        AudioBufferRef::S8(buf) => {
            convert_to_f32(buf.chan(0), |s| s as f32 / 128.0)
        }
        AudioBufferRef::S16(buf) => {
            convert_to_f32(buf.chan(0), |s| s as f32 / 32768.0)
        }
        AudioBufferRef::S24(buf) => {
            convert_to_f32(buf.chan(0), |s| s.inner() as f32 / 8388608.0)
        }
        AudioBufferRef::S32(buf) => {
            convert_to_f32(buf.chan(0), |s| s as f32 / 2147483648.0)
        }
        AudioBufferRef::F64(buf) => {
            buf.chan(0).iter().map(|&s| s as f32).collect()
        }
    }
}

fn convert_to_f32<T, F>(samples: &[T], convert: F) -> Vec<f32>
where
    F: Fn(T) -> f32,
    T: Copy,
{
    samples.iter().map(|&s| convert(s)).collect()
}
