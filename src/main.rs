use rshare::fetch_title;

fn main() {
    let url = "https://example.com";
    let title = fetch_title(url).unwrap();
    println!("Fetched title: {}", title);
}
