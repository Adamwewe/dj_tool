use std::fs::File;
use std::io::{Write, BufWriter};
use std::mem::MaybeUninit;

pub fn write_mp3(
    samples: &[f32],
    sample_rate: u32,
    channels: u16,
    bitrate_kbps: u32,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use mp3lame_encoder::{Builder, FlushNoGap, InterleavedPcm};

    let mut builder = Builder::new().ok_or("failed to create LAME encoder")?;
    builder.set_num_channels(channels as u8)
        .map_err(|e| format!("set_num_channels failed: {:?}", e))?;
    builder.set_sample_rate(sample_rate)
        .map_err(|e| format!("set_sample_rate failed: {:?}", e))?;
    builder.set_quality(mp3lame_encoder::Quality::Best)
        .map_err(|e| format!("set_quality failed: {:?}", e))?;
    builder.set_brate(mp3lame_encoder::Birtate::Kbps320)
        .map_err(|e| format!("set_brate failed: {:?}", e))?;

    let mut encoder = builder.build()
        .map_err(|e| format!("encoder build failed: {:?}", e))?;

    // LAME expects i16 PCM
    let pcm_i16: Vec<i16> = samples
        .iter()
        .map(|&s| (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
        .collect();

    let input = InterleavedPcm(&pcm_i16);

    // Allocate MaybeUninit buffer — the API requires this, not Vec<u8>
    let buf_size = mp3lame_encoder::max_required_buffer_size(pcm_i16.len());
    let mut mp3_buf: Vec<MaybeUninit<u8>> = vec![MaybeUninit::uninit(); buf_size];

    // Encode
    let encoded_size = encoder.encode(input, &mut mp3_buf)
        .map_err(|e| format!("encode failed: {:?}", e))?;

    // Flush remaining frames
    let mut flush_buf: Vec<MaybeUninit<u8>> = vec![MaybeUninit::uninit(); 7200];
    let flush_size = encoder.flush::<FlushNoGap>(&mut flush_buf)
        .map_err(|e| format!("flush failed: {:?}", e))?;

    // Convert MaybeUninit<u8> → u8 (safe because encode/flush initialized these bytes)
    let mut output_bytes = Vec::with_capacity(encoded_size + flush_size);
    unsafe {
        output_bytes.extend_from_slice(
            &*(&mp3_buf[..encoded_size] as *const [MaybeUninit<u8>] as *const [u8])
        );
        output_bytes.extend_from_slice(
            &*(&flush_buf[..flush_size] as *const [MaybeUninit<u8>] as *const [u8])
        );
    }

    let mut file = File::create(output_path)?;
    file.write_all(&output_bytes)?;
    Ok(())
}

pub fn write_aiff(
    samples: &[f32],
    sample_rate: u32,
    channels: u16,
    bits_per_sample: u16,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(output_path)?;
    let mut w = BufWriter::new(file);

    let num_sample_frames = samples.len() as u32 / channels as u32;
    let bytes_per_sample = bits_per_sample / 8;
    let sound_data_size = num_sample_frames * channels as u32 * bytes_per_sample as u32;

    let ssnd_chunk_size = 8 + sound_data_size;
    let comm_chunk_size = 18u32;
    let form_size = 4 + (8 + comm_chunk_size) + (8 + ssnd_chunk_size);

    // FORM header
    w.write_all(b"FORM")?;
    w.write_all(&form_size.to_be_bytes())?;
    w.write_all(b"AIFF")?;

    // COMM chunk
    w.write_all(b"COMM")?;
    w.write_all(&comm_chunk_size.to_be_bytes())?;
    w.write_all(&channels.to_be_bytes())?;
    w.write_all(&num_sample_frames.to_be_bytes())?;
    w.write_all(&bits_per_sample.to_be_bytes())?;
    w.write_all(&f64_to_extended(sample_rate as f64))?;

    // SSND chunk
    w.write_all(b"SSND")?;
    w.write_all(&ssnd_chunk_size.to_be_bytes())?;
    w.write_all(&0u32.to_be_bytes())?;
    w.write_all(&0u32.to_be_bytes())?;

    match bits_per_sample {
        16 => {
            for &s in samples {
                let i = (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                w.write_all(&i.to_be_bytes())?;
            }
        }
        24 => {
            for &s in samples {
                let i = (s.clamp(-1.0, 1.0) * 8_388_607.0) as i32;
                let bytes = i.to_be_bytes();
                w.write_all(&bytes[1..4])?;
            }
        }
        32 => {
            for &s in samples {
                let i = (s.clamp(-1.0, 1.0) * i32::MAX as f32) as i32;
                w.write_all(&i.to_be_bytes())?;
            }
        }
        _ => return Err(format!("Unsupported bit depth: {}", bits_per_sample).into()),
    }

    w.flush()?;
    Ok(())
}

fn f64_to_extended(val: f64) -> [u8; 10] {
    let mut result = [0u8; 10];
    if val == 0.0 { return result; }

    let sign: u16 = if val < 0.0 { 1 } else { 0 };
    let val = val.abs();

    let exponent = val.log2().floor() as i32;
    let mantissa = val / (2.0_f64.powi(exponent));

    let biased_exp = (exponent + 16383) as u16;
    let mantissa_bits = (mantissa * (1u64 << 63) as f64) as u64;

    let exp_field = (sign << 15) | biased_exp;
    result[0] = (exp_field >> 8) as u8;
    result[1] = (exp_field & 0xFF) as u8;
    result[2..10].copy_from_slice(&mantissa_bits.to_be_bytes());

    result
}
