use std::f32::consts::PI;

pub fn compute_mfcc(
    mel_spectrogram: &[Vec<f32>],
    num_mfcc: usize,
) -> Vec<Vec<f32>> {
    let num_mels = mel_spectrogram[0].len();

    mel_spectrogram
        .iter()
        .map(|frame| {
            let log_mel: Vec<f32> = frame
                .iter()
                .map(|&e| (e.max(f32::EPSILON)).ln())
                .collect();

            (0..num_mfcc)
                .map(|k| {
                    log_mel
                        .iter()
                        .enumerate()
                        .map(|(n, &val)| {
                            val * (PI * k as f32 * (n as f32 + 0.5) / num_mels as f32).cos()
                        })
                        .sum()
                })
                .collect()
        })
        .collect()
}
