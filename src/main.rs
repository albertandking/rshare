use rshare::fetch_title; // 假设您的库名为 rshare

fn main() {
    let url = "https://example.com";
    let title = fetch_title(url).unwrap();
    println!("Fetched title: {}", title);
}
