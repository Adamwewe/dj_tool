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

    use crate::FolderParser;
    use crate::Crawler;
    use crate::encoder::generate_waveform;

    #[pyo3::pyfunction]
    pub fn hello() -> PyResult<String> {
        println!("Hello py");
        Ok(String::from("Ok"))
    }

    #[pyo3::pyfunction]
    pub fn folder_parser_wrapper() -> PyResult<Vec<Crawler>> {
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
    ) -> PyResult<Bound<'py, PyArray1<f32>>> {
        //TODO: parallelize between folder crawling and getting waveforms, for now we do lazy loop
        let waves = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            let mut result = Vec::new();
            for x in &crawler_obj {
                if let Ok(wave) = generate_waveform(&x, target_width).await {
                result.extend(wave.peaks);
                }
            }
            result
        });
        Ok(PyArray1::from_vec(py, waves)) 
    }

















}
