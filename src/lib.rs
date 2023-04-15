use pyo3::exceptions::PyException;
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
        let mut key: String = args.to_string();
        if let Option::Some(kwds) = kwargs {
            key.push_str(&kwds.to_string());
        }
        let cache = self.cache.get_mut().as_ref(py);
        match cache.get_item(key.clone()) {
            Some(value) => Ok(value.into()),
            None => {
                let result = self.wraps.call(py, args, kwargs)?;
                cache.set_item(key, result.clone())?;
                Ok(result)
            }
        }
    }
}

#[pymodule]
pub fn cachers(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_class::<PyMemcache>()?;
    Ok(())
}

