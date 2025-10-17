use pyo3::prelude::*;
use pyo3::BoundObject;
use numpy::{PyReadonlyArray1, PyArray1, IntoPyArray};

#[pyfunction]
pub fn add_arrays<'py>(
    py: Python<'py>,
    a: PyReadonlyArray1<'py, f64>,
    b: PyReadonlyArray1<'py, f64>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let a = a.as_slice()?;
    let b = b.as_slice()?;
    let out: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
    Ok(out.into_pyarray(py).into_bound())
}

#[pymodule]
pub fn add_arrays_module(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_arrays, &m)?)?;
    Ok(())
}
