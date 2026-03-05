use std::f32::consts::PI;

pub fn mel_filterbank(
    num_mels: usize,
    num_bins: usize,
    sample_rate: u32,
    fft_size: usize,
) -> Vec<Vec<f32>> {
    let freq_to_mel = |f: f32| -> f32 { 2595.0 * (1.0 + f / 700.0).log10() };
    let mel_to_freq = |m: f32| -> f32 { 700.0 * (10.0_f32.powf(m / 2595.0) - 1.0) };

    let mel_low = 0.0;
    let mel_high = freq_to_mel(sample_rate as f32 / 2.0);

    let mel_points: Vec<f32> = (0..=num_mels + 1)
        .map(|i| mel_low + (mel_high - mel_low) * i as f32 / (num_mels + 1) as f32)
        .collect();

    let bin_indices: Vec<f32> = mel_points
        .iter()
        .map(|&m| mel_to_freq(m) * fft_size as f32 / sample_rate as f32)
        .collect();

    let mut filterbank = vec![vec![0.0_f32; num_bins]; num_mels];

    for m in 0..num_mels {
        let left = bin_indices[m];
        let center = bin_indices[m + 1];
        let right = bin_indices[m + 2];

        for k in 0..num_bins {
            let k_f = k as f32;
            if k_f >= left && k_f <= center {
                filterbank[m][k] = (k_f - left) / (center - left).max(f32::EPSILON);
            } else if k_f > center && k_f <= right {
                filterbank[m][k] = (right - k_f) / (right - center).max(f32::EPSILON);
            }
        }
    }

    filterbank
}

pub fn apply_mel_filterbank(
    spectrogram: &[Vec<f32>],
    filterbank: &[Vec<f32>],
) -> Vec<Vec<f32>> {
    let num_mels = filterbank.len();

    spectrogram
        .iter()
        .map(|frame| {
            (0..num_mels)
                .map(|m| {
                    frame
                        .iter()
                        .zip(filterbank[m].iter())
                        .map(|(&mag, &weight)| mag * mag * weight)
                        .sum()
                })
                .collect()
        })
        .collect()
}
