use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use num_bigint::BigUint;
use std::str::FromStr;
use pyo3::types::{PyDict, PyTuple};

#[pyclass]
struct _DoubleDecorator {
    func: PyObject,
}

struct PyBigUint(BigUint);

impl<'a> FromPyObject<'a> for PyBigUint {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let s = ob.str().unwrap().to_str().unwrap();
        PyResult::Ok(PyBigUint(BigUint::from_str(s).unwrap()))
    }
}

impl IntoPy<PyObject> for PyBigUint {
    fn into_py(self, py: Python) -> PyObject {
        let result = py.eval(
            format!("int({})", self.0.to_string()).as_str(),
            None, None
        ).unwrap();
        result.extract::<PyObject>().unwrap()
    }
}

#[pymethods]
impl _DoubleDecorator {
    #[call]
    #[args(args = "*", kwargs = "**")]
    fn __call__(&self, args: &PyTuple, kwargs: Option<&PyDict>) -> PyBigUint {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let res = self.func.call(py, args, kwargs).unwrap();
        let res = res.extract::<PyBigUint>(py).unwrap().0 * BigUint::from(2u32);
        PyBigUint(res)
    }
}

#[pyfunction]
fn double(func: PyObject) -> _DoubleDecorator {
    _DoubleDecorator { func }
}

#[pymodule]
fn decorators(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(double, m)?)?;

    Ok(())
}
