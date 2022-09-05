use pyo3::{pyfunction, pymodule, types::PyModule, wrap_pyfunction, PyResult, Python};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn area(width: usize, height: usize) -> PyResult<usize> {
    let square = Square::new(width, height);
    let area = square.area();
    Ok(area)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_intro(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(area, m)?)?;
    Ok(())
}

struct Square {
    width: usize,
    height: usize,
}

impl Square {
    fn new(width: usize, height: usize) -> Square {
        Square { width, height }
    }

    fn area(&self) -> usize {
        self.width * self.height
    }
}
