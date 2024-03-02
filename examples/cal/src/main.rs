extern crate rshare;

fn main() {
    let url = "https://www.baidu.com";
    let title = rshare::fetch_title(url).unwrap();
    println!("Title: {}", title);
}