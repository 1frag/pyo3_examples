use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
fn world() -> String {
    "hello world".to_string()
}

#[pyfunction]
fn somebody(name: &str) -> String {
    format!("hello {}", name)
}

#[pymodule]
fn hello(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(world, m)?)?;
    m.add_function(wrap_pyfunction!(somebody, m)?)?;

    Ok(())
}
