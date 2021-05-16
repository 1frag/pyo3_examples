use pyo3::{
    prelude::*,
    class::iter::{IterNextOutput, PyIterProtocol},
    wrap_pyfunction,
    types::{PyBytes, PyTuple, PyList, PyInt, PyString, PyFloat, PySequence},
    exceptions::PyAssertionError,
};
use num_bigint::BigInt;
use num_traits::{Zero, One};
use std::mem::replace;


#[pyclass]
struct Fib {
    a: BigInt,
    b: BigInt,
    direct: bool,
}

#[pymethods]
impl Fib {
    #[new]
    fn new() -> Self {
        Fib { a: Zero::zero(), b: One::one(), direct: true }
    }
}

#[pyproto]
impl PyIterProtocol for Fib {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<Fib>> {
        Ok(slf.into())
    }

    fn __next__(mut slf: PyRefMut<Self>) -> IterNextOutput<BigInt, ()> {
        slf.direct ^= true;
        match slf.direct {
            false => {
                slf.a = &slf.a + &slf.b;
                IterNextOutput::Yield(slf.a.clone())
            }
            true => {
                slf.b = &slf.a + &slf.b;
                IterNextOutput::Yield(slf.b.clone())
            }
        }
    }
}

#[pyfunction]
fn nth_fib(n: usize) -> String {
    let mut f0: BigInt = Zero::zero();
    let mut f1: BigInt = One::one();
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

fn is_integer_instance(obj: &PyAny) -> bool {
    obj.is_instance::<PyInt>().unwrap()
}

fn is_string_instance(obj: &PyAny) -> bool {
    obj.is_instance::<PyString>().unwrap()
}

fn is_float_instance(obj: &PyAny) -> bool {
    obj.is_instance::<PyFloat>().unwrap()
}

fn obj_to_tuple(obj: &PyAny) -> PyResult<&PyTuple> {
    let seq = <PySequence as PyTryFrom>::try_from(obj)?;
    seq.tuple()
}

#[pyfunction(args = "*", first = true, select = false)]
fn check_key(
    args: &PyTuple,
    first: bool,
    select: bool,
) -> PyResult<Vec<&PyAny>> {
    /* Bind for function at:
    https://github.com/tarantool/tarantool-python/blob/
        30b377595872eb03f743ff4f15317bfbca5d4fbb/tarantool/utils.py#L32
    */

    if select && args.len() == 0 {
        let v: Vec<&PyAny> = vec![];
        return Ok(v);
    }
    if args.len() == 1 {
        let args0 = args.get_item(0);
        if first && (args0.is_instance::<PyList>().unwrap() ||
            args0.is_instance::<PyTuple>().unwrap()) {
            let new_args = obj_to_tuple(args0).unwrap();
            return check_key(new_args, false, select);
        } else if args0.is_none() && select {
            let v: Vec<&PyAny> = vec![];
            return Ok(v);
        }
    }
    if args.iter().filter(|obj| {
        !is_integer_instance(*obj)
            && !is_string_instance(*obj)
            && !is_float_instance(*obj)
    }).next().is_some() {
        return Err(PyAssertionError::new_err("arguments should be int, str or float"));
    }

    Ok(args.iter().collect::<Vec<&PyAny>>())
}

#[pymodule]
fn utils(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Fib>()?;
    m.add_function(wrap_pyfunction!(nth_fib, m)?)?;
    m.add_function(wrap_pyfunction!(str_xor, m)?)?;
    m.add_function(wrap_pyfunction!(check_key, m)?)?;
    Ok(())
}
