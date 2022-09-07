use std::borrow::Cow;
use std::fs::File;

use gif::{Encoder, Frame, Repeat};
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn create_gif(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gol_gif, m)?)?;
    Ok(())
}

/// cell size: 50 x 50
#[pyfunction]
fn gol_gif(w: u16, h: u16, status: Vec<Vec<u8>>, path: String) {
    let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
    let (width, height) = (w, h);
    let mut image = File::create(path).unwrap();
    let mut encoder = Encoder::new(&mut image, width, height, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for state in status {
        let mut frame = Frame::default();
        frame.width = w;
        frame.height = h;
        frame.buffer = Cow::Borrowed(&*state);
        encoder.write_frame(&frame).unwrap();
    }
}
