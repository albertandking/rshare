use numpy::PyReadonlyArray1;
use numpy::{IntoPyArray, PyArray1};
use pyo3::prelude::*; // 用于在 Rust 中编写 Python 函数和模块
use pyo3::wrap_pyfunction;
use rayon::prelude::*; // 用于并行数据处理
use std::collections::HashMap; // 用于存储键值对
use pyo3::types::IntoPyDict;
extern crate rshare as rshare_rs;
use rshare_rs::fetch_title;

#[pyfunction]
pub fn get_ak_version(name: &str) -> PyResult<String> {
    Python::with_gil(|py| {
        let builtins = PyModule::import(py, "akshare")?;
        let version: String = builtins
            .getattr("__version__")?
            .extract()?;
        Ok(format!("{} 调用了我内部的 AKShare {} 版本", name, version))
    })
}

#[pyfunction]
pub fn mycode() -> PyResult<String> {
    let key1 = "key1";
    let val1 = 1;
    let key2 = "key2";
    let val2 = 2;

    Python::with_gil(|py| {
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            "def example(*args, **kwargs):
                if args != ():
                    print('called with args', args)
                if kwargs != {}:
                    print('called with kwargs', kwargs)
                if args == () and kwargs == {}:
                    print('called with no arguments')",
            "",
            "",
        )?
        .getattr("example")?
        .into();

        // call object with PyDict
        let kwargs = [(key1, val1)].into_py_dict(py);
        fun.call(py, (), Some(kwargs))?;

        // pass arguments as Vec
        let kwargs = vec![(key1, val1), (key2, val2)];
        fun.call(py, (), Some(kwargs.into_py_dict(py)))?;

        // pass arguments as HashMap
        let mut kwargs = HashMap::<&str, i32>::new();
        kwargs.insert(key1, 1);
        fun.call(py, (), Some(kwargs.into_py_dict(py)))?;
        Ok("ssssss".to_string())
    })
}

// Python 函数，用于获取网页标题
#[pyfunction]
pub fn get_title(url: &str) -> PyResult<String> {
    // 提取并返回网页标题
    let title_text = fetch_title(url)?;
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

#[pyfunction]
pub fn calculate_moving_average_in_rs<'py>(
    py: Python<'py>,
    data: PyReadonlyArray1<f64>,
    window_size: usize,
) -> PyResult<&'py PyArray1<f64>> {
    let data_slice = data.as_slice()?;
    if window_size == 0 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "window_size must be greater than 0",
        ));
    }
    if data_slice.len() < window_size {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "data length must be at least as large as window_size",
        ));
    }

    let result = unsafe { PyArray1::new(py, [data_slice.len()], false) };
    let result_slice = unsafe { result.as_slice_mut()? };
    let window_length = window_size as f64;

    let mut sum = 0.0;
    for i in 0..window_size {
        sum += data_slice[i];
        result_slice[i] = f64::NAN; // Fill the start with NaN.
    }

    for i in window_size..data_slice.len() {
        sum += data_slice[i] - data_slice[i - window_size];
        result_slice[i] = sum / window_length;
    }

    Ok(result)
}

// 将 Python 函数注册到模块
#[pymodule]
fn rshare(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_ak_version, m)?)?;
    m.add_function(wrap_pyfunction!(mycode, m)?)?;
    m.add_function(wrap_pyfunction!(get_title, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_moving_average_rs, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_moving_average_in_rs, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
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
        assert_eq!(result.unwrap(), "我来了".to_string());
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
