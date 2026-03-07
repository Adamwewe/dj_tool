//entry point for pyo3
pub mod folder_crawler;
pub mod parse_path;

pub mod audio;
pub mod spectrogram;

use parse_path::FolderParser;
use folder_crawler::Crawler;
use pyo3::prelude::*;
use numpy::{IntoPyArray, PyArray2};

use audio::AudioStream;

use std::fmt;

fn vec2d_to_numpy<'py>(
    py: Python<'py>,
    data: Vec<Vec<f32>>
) -> PyResult<Bound<'py, PyArray2<f32>>> {
    let rows = data.len();
    let cols = if rows > 0 {data[0].len()} else{0};
    let flat = data.into_iter().flatten().collect();

    Ok(
        numpy::ndarray::Array2::from_shape_vec((rows, cols), flat)
        .map_err(|e|
            pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?
            .into_pyarray(py)
    )
}


fn block_on<F, T>()(f: F) -> PyResult<T>
    where
        F: std::future::Future<Output = Result<T, Box<dyn std::error::Error + send + Sync>>,
    {
        
    }


#[pyfunction]
#[pyo3(signature = (path, fft_size=2048, hop_size=512, to_db=true))]
async fn get_spectrogram<'py>(
    py: Python<'py>,
    path: &str,
    fft_size: usize,
    hop_size: usize,
    to_db: bool,
) -> PyResult<Bound<'py, PyArray2<f32>>> {
    
    let mut audio : Result<AudioStream, Box<>> = AudioStream::from_file(path).await?
        .map_err(|e|
            pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            audio.to_mono();

}









#[pyo3::pymodule] 
mod decoding_backend {
    
    use pyo3::prelude::*;
    use pyo3::exceptions::PyTypeError;
    use numpy::{IntoPyArray, PyArray1};
    use pyo3::types::PyList;

    use crate::FolderParser;
    use crate::Crawler;
    use crate::audio::decoder::decode_audio;

    #[pyo3::pyfunction]
    pub fn hello() -> PyResult<String> {
        println!("Hello py");
        Ok(String::from("Ok"))
    }

    #[pyo3::pyfunction]
    pub fn get_tracks() -> PyResult<Vec<Crawler>> {
    let parsed = FolderParser::parser(); 
    let items = Crawler::new(parsed.path)
        .crawl();
    
    if !items.is_empty(){
        return Ok(items)
    }
    let response = PyErr::new::<PyTypeError, &str>("No items found");
    Err(response)
    }

    #[pyo3::pyfunction]
    pub fn get_streams<'py>(
        py: Python<'py>, 
        crawler_obj: Vec<Crawler>,
    ) -> PyResult<Bound<'py, PyList>> {
        //TODO: parallelize between folder crawling and getting waveforms, for now we do lazy loop
        let array = PyList::empty(py);
        //TODO:try non async, see if that fixes bug
        tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            for i in crawler_obj {
                match decode_audio(&i.path).await {
                        Ok(wave) => {
                            let (samples, sample_rate, channels) = wave;
                            let waves = PyArray1::from_vec(py, samples);
                            array.append(waves).unwrap();
                        },
                        Err(e) => eprintln!("Error: {}", e),
                }
            }
        });
        Ok(array)
    }




// #[pyo3::pyfunction]
//     pub fn get_streams<'py>(
//         py: Python<'py>, 
//         crawler_obj: Vec<Crawler>,
//         target_width : usize,
//     ) -> PyResult<Bound<'py, PyList>> {
//         //TODO: parallelize between folder crawling and getting waveforms, for now we do lazy loop
//         let array = PyList::empty(py);
//         //TODO:try non async, see if that fixes bug
//             for i in crawler_obj {
//                 match generate_waveform(&i, target_width){
//                     Ok(wave) => {
//                             let waves = PyArray1::from_vec(py, wave.peaks);
//                             array.append("ee").unwrap();
//                         },
//                         Err(e) => eprintln!("Error: {}", e),
//                 }
//                 // if let Ok(wave) = generate_waveform(&x, target_width).await {
//                 // println!("{:?}", wave.peaks.len());
//                 // result.extend(wave.peaks);
//             }
//         Ok(array)
//     }













}
