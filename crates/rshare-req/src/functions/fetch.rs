use reqwest; // 用于执行 HTTP 请求
use scraper::{Html, Selector}; // 用于解析 HTML 文档
use std::io::Result; // 用于返回 Result 类型

// Python 函数，用于获取网页标题
pub fn fetch_title(url: &str) -> Result<String> {
    // 执行 HTTP GET 请求
    let res = reqwest::blocking::get(url).unwrap();

    // 读取响应体
    let body = res.text().unwrap();

    // 解析响应体为 HTML
    let document = Html::parse_document(&body);
    let selector = Selector::parse("title").unwrap();

    // 提取并返回网页标题
    let title_text = document
        .select(&selector)
        .next()
        .map(|e| e.text().collect::<Vec<_>>().join(""))
        .unwrap_or_default();
    let title_text = "Rust 采集：".to_string() + title_text.as_str();
    Ok(title_text)
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
    }
    }
