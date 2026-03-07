pub mod decoder;
pub mod encoder;

use std::path::Path;

pub struct AudioStream {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
}

impl AudioStream {
    pub async fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let (samples, sample_rate, channels) = decoder::decode_audio(path).await?;
        Ok(Self {
            samples,
            sample_rate,
            channels,
        })
    }
    
    //for spectrograms
    pub fn to_mono(&mut self) {
        if self.channels <= 1 {
            return;
        }
        let ch = self.channels as usize;
        self.samples = self
            .samples
            .chunks(ch)
            .map(|chunk| chunk.iter().sum::<f32>() / ch as f32)
            .collect();
        self.channels = 1;
    }

    pub fn duration_secs(&self) -> f32 {
        let num_frames = self.samples.len() / self.channels as usize;
        num_frames as f32 / self.sample_rate as f32
    }

    pub fn write_to(
        &self,
        output_path: &str,
        bitrate_kbps: Option<u32>,
        bits_per_sample: Option<u16>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ext = Path::new(output_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        match ext.as_str() {
            "mp3" => encoder::write_mp3(
                &self.samples,
                self.sample_rate,
                self.channels,
                bitrate_kbps.unwrap_or(320),
                output_path,
            ),
            "aiff" | "aif" => encoder::write_aiff(
                &self.samples,
                self.sample_rate,
                self.channels,
                bits_per_sample.unwrap_or(16),
                output_path,
            ),
            other => Err(format!(
                "Unsupported output format: .{}\nSupported: .mp3, .aiff, .aif",
                other
            ).into()),
        }
    }
}
