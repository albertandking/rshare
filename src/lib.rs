use numpy::PyReadonlyArray1;
use numpy::{IntoPyArray, PyArray1};
use pyo3::prelude::*; // 用于在 Rust 中编写 Python 函数和模块
use pyo3::wrap_pyfunction;
use rayon::prelude::*; // 用于并行数据处理
use reqwest; // 用于执行 HTTP 请求
use scraper::{Html, Selector}; // 用于解析 HTML 文档

// Python 函数，用于获取网页标题
#[pyfunction]
pub fn fetch_title(url: &str) -> PyResult<String> {
    // 执行 HTTP GET 请求
    let res = reqwest::blocking::get(url)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;

    // 读取响应体
    let body = res
        .text()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;

    // 解析响应体为 HTML
    let document = Html::parse_document(&body);
    let selector = Selector::parse("title").unwrap();

    // 提取并返回网页标题
    let title_text = document
        .select(&selector)
        .next()
        .map(|e| e.text().collect::<Vec<_>>().join(""))
        .unwrap_or_default();

    Ok(title_text)
}

#[pyfunction]
pub fn calculate_moving_average_rs<'py>(
    py: Python<'py>,
    data: PyReadonlyArray1<f64>,
    window_size: usize,
) -> PyResult<&'py PyArray1<f64>> {
    // 读取 NumPy 数组数据
    let data_slice = data.as_slice()?;
    // 根据窗口大小选择计算方法
    if window_size > 80 || data_slice.len() < 500000 {
        let result = calculate_moving_average_big(data_slice, window_size);
        Ok(result.into_pyarray(py))
    } else {
        let result = calculate_moving_average_small(data_slice, window_size);
        Ok(result.into_pyarray(py))
    }
}

// 小窗口大小时计算移动平均的函数
pub fn calculate_moving_average_small(data: &[f64], window_size: usize) -> Vec<f64> {
    let window_length = f64::from(window_size as u32);
    // 初始化结果向量
    let mut averages: Vec<f64> = vec![std::f64::NAN; window_size - 1];

    // 使用并行迭代器计算移动平均
    averages.par_extend(
        data.par_windows(window_size)
            .map(|window| window.iter().sum::<f64>() / window_length),
    );

    averages
}

// 大窗口大小时计算移动平均的函数
pub fn calculate_moving_average_big(data: &[f64], window_size: usize) -> Vec<f64> {
    let mut result = Vec::with_capacity(data.len());
    let mut sum = 0.0;
    let window_length = window_size as f64;

    // 使用前缀和计算移动平均
    for i in 0..data.len() {
        sum += data[i];
        if i >= window_size {
            sum -= data[i - window_size];
        }
        // 填充结果向量
        if i >= window_size - 1 {
            result.push(sum / window_length);
        } else {
            // 窗口未满时开始部分填充 NaN
            result.push(std::f64::NAN);
        }
    }

    result
}

// 将 Python 函数注册到模块
#[pymodule]
fn rshare(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fetch_title, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_moving_average_rs, m)?)?;
    Ok(())
}

// 单元测试模块
#[cfg(test)]
mod tests {
    use super::*; // 导入上面定义的函数和模块

    // 测试获取网页标题功能
    #[test]
    fn test_fetch_title() {
        // 这里应提供一个真实可访问的 URL 地址
        let url = "https://example.com";
        let result = fetch_title(url);
        assert!(result.is_ok()); // 确保结果是 Ok
                                 // 对比实际标题和预期标题
        assert_eq!(result.unwrap(), "Example Domain".to_string());
    }

    // 测试计算移动平均的功能
    #[test]
    fn test_calculate_moving_average() {
        // 测试数据集
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        // 窗口大小
        let window_size = 3;
        // 使用小窗口函数计算移动平均
        let averages_small = calculate_moving_average_small(&data, window_size);
        // 使用大窗口函数计算移动平均
        let averages_big = calculate_moving_average_big(&data, window_size);
        // 预期的移动平均结果
        let expected_averages = vec![std::f64::NAN, std::f64::NAN, 2.0, 3.0, 4.0];

        // 确保两种方法产生相同的结果
        assert_eq!(averages_small, expected_averages);
        assert_eq!(averages_big, expected_averages);
    }
}
