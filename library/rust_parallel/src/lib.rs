use pyo3::prelude::*;
use rayon::prelude::*;

#[pyfunction]
fn square_list(input: Vec<u64>) -> PyResult<Vec<u64>> {
    let result: Vec<u64> = input.par_iter().map(|&x| x * x).collect();
    Ok(result)
}
#[pymodule]
fn rust_parallel(_py: Python, m: &PyModule) -> PyResult<()>{
    m.add_function(wrap_pyfunction!(square_list,m)?)?;
    Ok(())
}


