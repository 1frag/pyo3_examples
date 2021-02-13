use pyo3::prelude::*;
use pyo3::class::iter::{IterNextOutput, PyIterProtocol};
use pyo3::class::pyasync::PyAsyncProtocol;
use pyo3::wrap_pyfunction;

#[pyclass]
struct _Sleep {
    n: i32,
    state: State,
    future: Option<PyObject>,
}

#[derive(PartialEq, Eq, Clone)]
enum State {
    Initial,
    Pending,
    Done,
}

#[pymethods]
impl _Sleep {
    #[new]
    fn new(n: i32) -> Self {
        _Sleep { n, state: State::Initial, future: None }
    }

    fn _work_for_initial(&mut self) {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let asyncio = PyModule::import(py, "asyncio").unwrap();
        let loop_obj: PyObject = asyncio.call0("get_event_loop")
            .unwrap().extract().unwrap();
        let future = loop_obj
            .call_method0(py, "create_future")
            .unwrap();
        loop_obj.call_method1(py, "call_later", (
            self.n,
            future.getattr(py, "set_result").unwrap(),
            py.None(),
        )).unwrap();
        self.future = Some(future);
        self.state = State::Pending;
    }

    fn _work_for_pending(&mut self) -> Option<PyObject> {
        if self.future.is_none() { panic!() }
        let future = self.future.as_ref().unwrap();
        let gil = Python::acquire_gil();
        let py = gil.python();

        if !future
            .call_method0(py, "done").unwrap()
            .extract::<PyObject>(py).unwrap()
            .is_true(py).unwrap() {
            let res: PyObject = future
                .call_method0(py, "__iter__").unwrap()
                .extract::<PyObject>(py).unwrap()
                .call_method0(py, "__next__").unwrap()
                .extract::<PyObject>(py).unwrap();
            return Some(res)
        }
        self.state = State::Done;
        None
    }
}

#[pyproto]
impl PyIterProtocol for _Sleep {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<_Sleep>> {
        Ok(slf.into())
    }

    fn __next__(mut slf: PyRefMut<Self>) -> IterNextOutput<Option<PyObject>, ()> {
        if slf.state == State::Initial {
            slf._work_for_initial();
        }
        if slf.state == State::Pending {
            let res = slf._work_for_pending();
            if res.is_some() {
                return IterNextOutput::Yield(res);
            }
        }
        if slf.state == State::Done {
            return IterNextOutput::Return(())
        }
        unreachable!()
    }
}

#[pyproto]
impl PyAsyncProtocol for _Sleep {
    fn __await__(slf: PyRef<Self>) -> _Sleep {
        _Sleep::new(slf.n)
    }
}

#[pyfunction]
fn sleep(n: i32) -> _Sleep {
    _Sleep::new(n)
}

#[pymodule]
fn async_funcs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sleep, m)?)?;
    Ok(())
}
