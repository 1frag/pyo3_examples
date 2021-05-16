use pyo3::{
    prelude::*,
    wrap_pyfunction,
};
use pyo3_asyncio;
use tokio;
use std::time::Duration;

async fn rust_sleep(secs: u64) {
    tokio::time::sleep(Duration::from_secs(secs)).await;
}

#[pyfunction]
fn fetch_db(py: Python, secs: &PyAny) -> PyResult<PyObject> {
    let secs = secs.extract()?;

    pyo3_asyncio::tokio::into_coroutine(py, async move {
        rust_sleep(secs).await;
        Python::with_gil(|py| Ok(py.None()))
    })
}

#[pyfunction]
fn init(py: Python) -> PyResult<()> {
    pyo3_asyncio::try_init(py)?;
    pyo3_asyncio::tokio::init_multi_thread();
    Ok(())
}

#[pymodule]
fn async_sleep(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_db, m)?)?;
    Ok(())
}
