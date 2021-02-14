use pyo3::prelude::*;
use pyo3::{create_exception, wrap_pyfunction};
use pyo3::exceptions::{PyException, PyKeyError, PyValueError};

create_exception!(exceptions, MyFirstException, PyException);
create_exception!(exceptions, MyExceptionBasedOnKeyError, PyKeyError);

#[pyfunction]
fn func_that_raise_value_error_42() -> PyResult<()> {
    let err = PyValueError::new_err(42);
    Err(err)
}

#[pyfunction]
fn ignore_value_error(func: PyObject) -> PyResult<PyObject> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    match func.call0(py) {
        Ok(t) => Ok(t),
        Err(er) => {
            if er.is_instance::<PyValueError>(py) {
                Ok(py.None())
            } else {
                Err(er)
            }
        }
    }
}

#[pymodule]
fn exceptions(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("MyFirstException",
          py.get_type::<MyFirstException>())?;
    m.add("MyExceptionBasedOnKeyError",
          py.get_type::<MyExceptionBasedOnKeyError>())?;
    m.add_function(wrap_pyfunction!(func_that_raise_value_error_42, m)?)?;
    m.add_function(wrap_pyfunction!(ignore_value_error, m)?)?;
    Ok(())
}
