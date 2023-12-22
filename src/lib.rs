use polars_core::prelude::*;
use pyo3::prelude::*; // 引入 PyO3 库，用于在 Rust 中编写 Python 函数
use pyo3::wrap_pyfunction;
use rayon::prelude::*; // 引入 Rayon 库，用于数据的并行处理
use reqwest; // 引入 reqwest 库，用于进行 HTTP 请求
use scraper::{Html, Selector}; // 引入 scraper 库，用于解析 HTML 文档 // 引入 Polars 库，用于计算移动平均值

// 定义一个 Python 函数，用于获取网页的标题
#[pyfunction]
pub fn fetch_title(url: &str) -> PyResult<String> {
    // 发起 HTTP GET 请求
    let res = reqwest::blocking::get(url)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;

    // 获取响应正文
    let body = res
        .text()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;

    // 解析 HTML 文档
    let document = Html::parse_document(&body);
    let selector = Selector::parse("title").unwrap();

    // 提取标题文本
    let title_text = document
        .select(&selector)
        .next()
        .map(|e| e.text().collect::<Vec<_>>().join(""))
        .unwrap_or_default();

    Ok(title_text)
}

// 定义一个 Python 函数，用于计算移动平均值
#[pyfunction]
pub fn calculate_moving_average_rs(data: Vec<f64>, window_size: usize) -> PyResult<Vec<f64>> {
    // 调用内部 Rust 函数进行计算
    Ok(calculate_moving_average(&data, window_size))
}

// 内部 Rust 函数，用于计算移动平均值
pub fn calculate_moving_average(data: &[f64], window_size: usize) -> Vec<f64> {
    let window_length = f64::from(window_size as u32);
    // 使用 Rayon 的并行窗口函数处理数据
    data.par_windows(window_size)
        .map(|window| window.iter().sum::<f64>() / window_length)
        .collect()
}

#[pyfunction]
pub fn calculate_moving_average_talib_rs(data: Vec<f64>, window_size: u32) -> PyResult<Vec<f64>> {
    let close = Series::new("data", &data);
    let ma = rusty_talib::moving_average(&close, Some(window_size)).unwrap();
    let vec: Vec<Option<f64>> = ma.f64().unwrap().into_iter().collect();
    let d = vec.into_iter().map(|x| x.unwrap()).collect();
    Ok(d)
}

// 将函数包装为 Python 模块
#[pymodule]
fn rshare(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fetch_title, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_moving_average_rs, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_moving_average_talib_rs, m)?)?;
    Ok(())
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*; // 导入要测试的函数和模块

    // 测试获取网页标题的函数
    #[test]
    fn test_fetch_title() {
        let url = "https://example.com";
        let result = fetch_title(url);
        // 首先确保结果是 Ok
        assert!(result.is_ok());
        // 然后比较 Ok 中的值
        assert_eq!(result.unwrap(), "Example Domain".to_string());
    }

    // 测试计算移动平均值的函数
    #[test]
    fn test_calculate_moving_average() {
        // 编写针对 calculate_moving_average 函数的测试
        let data = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
        let window_size = 5;
        let result = calculate_moving_average(&data, window_size);

        // 使用 assert_eq! 宏检查结果是否与预期相符
        assert_eq!(result, vec![10.0, 10.5, 11.0, 11.5, 12.0, 12.5, 13.0]);
    }

    #[test]
    fn test_calculate_moving_average_talib_rs() {
        // 编写针对 calculate_moving_average 函数的测试
        let data = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
        let window_size = 5;
        let result = calculate_moving_average_talib_rs(data, window_size);

        // 使用 assert_eq! 宏检查结果是否与预期相符
        assert_eq!(
            result.unwrap(),
            vec![10.0, 10.5, 11.0, 11.5, 12.0, 12.5, 13.0]
        );
    }
}
