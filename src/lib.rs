use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use reqwest;
use scraper::{Html, Selector};
use rayon::prelude::*;

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




#[pyfunction]
fn calculate_moving_average_rs(prices: Vec<f64>, window_size: usize) -> PyResult<Vec<f64>> {
    Ok(calculate_moving_average(&prices, window_size))
}

fn calculate_moving_average(prices: &[f64], window_size: usize) -> Vec<f64> {
    prices
        .par_windows(window_size)
        .map(|window| {
            let sum: f64 = window.iter().sum();
            sum / window.len() as f64
        })
        .collect()
}

#[pymodule]
fn rshare(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fetch_title, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_moving_average_rs, m)?)?;
    Ok(())
}

// Optional: Main function for testing
// fn main() {
//     let stock_prices = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
//     let averages = calculate_moving_average(&stock_prices, 5);
//     println!("{:?}", averages);
// }