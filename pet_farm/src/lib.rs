use pyo3::prelude::*;

#[pyclass]
struct Cat {}

#[pyclass]
struct Dog {}

trait Say {
    fn say(&self) -> String;
}

impl Say for Cat {
    fn say(&self) -> String {
        "Meow".to_string()
    }
}

impl Say for Dog {
    fn say(&self) -> String {
        "Woof".to_string()
    }
}

#[pymethods]
impl Cat {
    #[new]
    fn new() -> Self {
        Cat {}
    }

    fn say(&self) -> String {
        Say::say(self)
    }
}

#[pymethods]
impl Dog {
    #[new]
    fn new() -> Self {
        Dog {}
    }

    fn say(&self) -> String {
        Say::say(self)
    }
}

#[pymodule]
fn pet_farm(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Cat>()?;
    m.add_class::<Dog>()?;

    Ok(())
}
