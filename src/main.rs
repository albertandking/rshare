// 引入 rand 库的 Rng trait，以便使用随机数生成器
use rand::Rng;
// 引入 rshare 库，以便使用大数据集和小数据集的移动平均值计算函数，以及抓取网页标题的函数
use rshare::{calculate_moving_average_big, calculate_moving_average_small, fetch_title};
// 引入标准库中的 Instant 类型，用于测量时间段
use std::time::Instant;

fn main() {
    // 开始抓取网页标题的时间点
    let fetch_start = Instant::now();
    // 定义要抓取标题的网页 URL
    let url = "https://example.com";
    // 调用 fetch_title 函数从 URL 中获取标题，如果出现错误则触发 panic
    let title = fetch_title(url).unwrap();
    // 计算抓取标题所花费的时间
    let fetch_duration = fetch_start.elapsed();
    // 打印出抓取到的标题
    println!("Fetched title: {}", title);
    // 打印出抓取标题所花费的时间
    println!("Fetching title took: {:?}", fetch_duration);

    // 开始创建随机数据向量的时间点
    let vec_start = Instant::now();
    // 定义随机数据的大小，即数据向量中元素的数量
    let data_size = 10000000;

    // 生成一个随机浮点数的向量
    let data: Vec<f64> = (0..data_size).map(|_| rand::thread_rng().gen()).collect();

    // 利用固定数值生成一个向量
    // let initial_vec = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    // let data = initial_vec.repeat(data_size);

    // 计算创建向量所花费的时间
    let vec_duration = vec_start.elapsed();
    // 打印出创建向量所花费的时间
    println!("Vector creation took: {:?}", vec_duration);

    // 开始计算小数据集移动平均值的时间点
    let ma_start = Instant::now();
    // 设置移动平均值计算的窗口大小
    let window_size = 5;
    // 调用 calculate_moving_average_small 函数计算小数据集的移动平均值
    calculate_moving_average_small(&data, window_size);
    // 计算移动平均值计算所花费的时间
    let ma_duration = ma_start.elapsed();
    // 打印出计算移动平均值所花费的时间
    println!("Small dataset moving average calculation took: {:?}", ma_duration);

    // 开始计算大数据集移动平均值的时间点
    let ma_start = Instant::now();
    // 设置移动平均值计算的窗口大小为更大的值
    let window_size = 500;
    // 调用 calculate_moving_average_big 函数计算大数据集的移动平均值
    calculate_moving_average_big(&data, window_size);
    // 计算移动平均值计算所花费的时间
    let ma_duration = ma_start.elapsed();
    // 打印出计算移动平均值所花费的时间
    println!("Big dataset moving average calculation took: {:?}", ma_duration);
}