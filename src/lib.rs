use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use std::cell::Cell;

#[pyclass(name = "Memcache")]
pub struct PyMemcache {
    wraps: Py<PyAny>,
    cache: Cell<Py<PyDict>>,
}

#[pymethods]
impl PyMemcache {

    #[new]
    fn __new__(py: Python<'_>, wraps: Py<PyAny>) -> Self {
        PyMemcache {
            wraps,
            cache: Cell::new(PyDict::new(py).into()),
        }
    }

    // #[getter]
    // fn cache(&self, py: Python) -> &PyDict {
    //     self.cache.into_inner().into_ref(py)
    // }

    #[pyo3(signature = (*args, **kwargs))]
    fn __call__(
        &mut self,
        py: Python<'_>,
        args: &PyTuple,
        kwargs: Option<&PyDict>,
    ) -> PyResult<Py<PyAny>> {
        let mut k: String = args.to_string();
        if !kwargs.is_none() {
            k.push_str(&kwargs.unwrap().to_string());
        }
        let cache: &PyDict = self.cache.get_mut().as_ref(py);
        if cache.contains(k.to_owned()).unwrap() {
            return Ok(cache.get_item(k).unwrap().into())
        }
        let value = self.wraps.call(py, args, kwargs).expect("failed to execute wrapped function");
        cache.set_item(k, &value);
        Ok(value)
    }
}

#[pymodule]
pub fn decorator(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_class::<PyMemcache>()?;
    Ok(())
}

