use rshare::fetch_title;
use rusty_talib;

use polars_core::prelude::*;

fn main() {
    let url = "https://example.com";
    let title = fetch_title(url).unwrap();
    println!("Fetched title: {}", title);

    let random_data: [i32; 7] = [23, 25, 12, 28, 33, 31, 35];
    let close = Series::new("data", random_data);
    let ma = rusty_talib::moving_average(&close,Some(3)).unwrap();
    println!("Moving average: {}", ma);
}
