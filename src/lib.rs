use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rayon::prelude::*;
use reqwest;
use scraper::{Html, Selector};

#[pyfunction]
pub fn fetch_title(url: &str) -> PyResult<String> {
    let res = reqwest::blocking::get(url)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;

    let body = res
        .text()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
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
pub fn calculate_moving_average_rs(prices: Vec<f64>, window_size: usize) -> PyResult<Vec<f64>> {
    Ok(calculate_moving_average(&prices, window_size))
}

pub fn calculate_moving_average(prices: &[f64], window_size: usize) -> Vec<f64> {
    let window_length = f64::from(window_size as u32);
    prices
        .par_windows(window_size)
        .map(|window| window.iter().sum::<f64>() / window_length)
        .collect()
}

#[pymodule]
fn rshare(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fetch_title, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_moving_average_rs, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*; // 导入要测试的函数和模块

    #[test]
    fn test_fetch_title() {
        let url = "https://example.com";
        let result = fetch_title(url);
        // 首先确保结果是 Ok
        assert!(result.is_ok());
        // 然后比较 Ok 中的值
        assert_eq!(result.unwrap(), "Example Domain".to_string());
    }

    #[test]
    fn test_calculate_moving_average() {
        // 编写针对 calculate_moving_average 函数的测试
        let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
        let window_size = 5;
        let result = calculate_moving_average(&prices, window_size);

        // 使用 assert_eq! 宏检查结果是否与预期相符
        assert_eq!(result, vec![10.0, 10.5, 11.0, 11.5, 12.0, 12.5, 13.0]);
    }
}
