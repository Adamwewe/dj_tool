//entry point for pyo3

use pyo3::prelude::*;

#[pyo3::pyfunction]
pub fn translate() -> PyResult<String> {
    println!("Hello py");
    Ok(String::from("Ok"))
}
