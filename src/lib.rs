use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use reqwest;
use scraper::{Html, Selector};

#[pyfunction]
fn fetch_title(url: String) -> PyResult<String> {
    let res = reqwest::blocking::get(&url).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;

    let body = res.text().map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("title").unwrap();

    let title_text = document
        .select(&selector)
        .next()
        .map(|e| e.text().collect::<Vec<_>>().join(""))
        .unwrap_or_default();

    Ok(title_text)
}

#[pymodule]
fn rshare(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fetch_title, m)?)?;
    Ok(())
}
