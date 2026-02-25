//entry point for pyo3
pub mod folder_crawler;
pub mod parse_path;
pub mod encoder;

use parse_path::FolderParser;
use folder_crawler::Crawler;

use std::fmt;

#[pyo3::pymodule] 
mod decoding_backend {
    
    use pyo3::prelude::*;
    use pyo3::exceptions::PyTypeError;
    use numpy::{IntoPyArray, PyArray1};
    use pyo3::types::PyList;

    use crate::FolderParser;
    use crate::Crawler;
    use crate::encoder::generate_waveform;

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
        target_width : usize,
    ) -> PyResult<Bound<'py, PyList>> {
        //TODO: parallelize between folder crawling and getting waveforms, for now we do lazy loop
        let array = PyList::empty(py);

        tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            for i in crawler_obj {
                match generate_waveform(&i, target_width).await {
                        Ok(wave) => {
                            let waves = PyArray1::from_vec(py, wave.peaks);
                            array.append(waves).unwrap();
                        },
                        Err(e) => eprintln!("Error: {}", e),
                }
                // if let Ok(wave) = generate_waveform(&x, target_width).await {
                // println!("{:?}", wave.peaks.len());
                // result.extend(wave.peaks);
            }
        });
        Ok(array)
    }

















}
