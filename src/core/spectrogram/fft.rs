use rustfft::{num_complex::Complex, FftPlanner};
use std::f32::consts::PI;

pub fn hann_window(size: usize) -> Vec<f32> {
    (0..size)
        .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f32 / size as f32).cos()))
        .collect()
}

pub fn compute_spectrogram(
    samples: &[f32],
    fft_size: usize,
    hop_size: usize,
) -> Vec<Vec<f32>> {
    let window = hann_window(fft_size);
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(fft_size);
    let num_bins = fft_size / 2 + 1;

    let num_frames = if samples.len() >= fft_size {
        (samples.len() - fft_size) / hop_size + 1
    } else {
        0
    };

    let mut spectrogram = Vec::with_capacity(num_frames);

    for frame_idx in 0..num_frames {
        let start = frame_idx * hop_size;

        let mut buffer: Vec<Complex<f32>> = samples[start..start + fft_size]
            .iter()
            .zip(window.iter())
            .map(|(&s, &w)| Complex { re: s * w, im: 0.0 })
            .collect();

        fft.process(&mut buffer);

        let magnitudes: Vec<f32> = buffer[..num_bins]
            .iter()
            .map(|c| c.norm())
            .collect();

        spectrogram.push(magnitudes);
    }

    spectrogram
}

//DBs are scaled to log 
pub fn power_to_db(spectrogram: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let ref_max = spectrogram
        .iter()
        .flat_map(|frame| frame.iter())
        .cloned()
        .fold(f32::MIN, f32::max)
        .max(f32::EPSILON);

    spectrogram
        .iter()
        .map(|frame| {
            frame
                .iter()
                .map(|&mag| {
                    let power = (mag * mag).max(f32::EPSILON);
                    10.0 * (power / (ref_max * ref_max)).log10()
                })
                .collect()
        })
        .collect()
}
