use pyo3::prelude::*;
use pyo3::class::iter::{IterNextOutput, PyIterProtocol};
use pyo3::wrap_pyfunction;
use pyo3::types::PyBytes;
use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;
use std::convert::TryInto;
use std::borrow::Borrow;

struct PyBigUint(BigUint);

impl Clone for PyBigUint {
    fn clone(&self) -> Self {
        PyBigUint(self.0.clone())
    }
}

#[pyclass]
struct Fib {
    a: PyBigUint,
    b: PyBigUint,
    direct: bool,
}

#[pymethods]
impl Fib {
    #[new]
    fn new() -> Self {
        Fib { a: PyBigUint(Zero::zero()), b: PyBigUint(One::one()), direct: true }
    }
}

impl IntoPy<pyo3::Py<PyAny>> for PyBigUint {
    fn into_py(self, py: Python) -> Py<PyAny> {
        format!("{}", self.0).into_py(py)
    }
}

#[pyproto]
impl PyIterProtocol for Fib {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<Fib>> {
        Ok(slf.into())
    }

    fn __next__(mut slf: PyRefMut<Self>) -> IterNextOutput<PyBigUint, ()> {
        slf.direct ^= true;
        match slf.direct {
            false => {
                slf.a.0 = &slf.a.0 + &slf.b.0;
                IterNextOutput::Yield(slf.a.clone())
            }
            true => {
                slf.b.0 = &slf.a.0 + &slf.b.0;
                IterNextOutput::Yield(slf.b.clone())
            }
        }
    }
}

#[pyfunction]
fn nth_fib(n: usize) -> String {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2);
    }
    format!("{}", f0)
}

#[pyfunction]
fn str_xor<'a>(
    py: Python<'a>,
    rhs: &'a [u8],
    lhs: &'a [u8],
) -> &'a PyBytes {
    let data = lhs.iter().zip(rhs.iter()).map(|(l, r)| {
        l ^ r
    }).collect::<Vec<u8>>();
    PyBytes::new(py, data.as_slice())
}

#[pymodule]
fn utils(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Fib>()?;
    m.add_function(wrap_pyfunction!(nth_fib, m)?)?;
    m.add_function(wrap_pyfunction!(str_xor, m)?)?;
    Ok(())
}
