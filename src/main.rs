use rshare::fetch_title; // 假设您的库名为 rshare

fn main() {
    let url = "https://example.com".to_string();
    let title = fetch_title(url).unwrap();
    println!("Fetched title: {}", title);
}
